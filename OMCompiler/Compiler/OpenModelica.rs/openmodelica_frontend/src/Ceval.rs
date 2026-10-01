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
use crate::FGraph;
use crate::InstBinding;
use crate::InstUtil;
use crate::Lookup;
use crate::Static;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlSetCR;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_dump::ValuesMake;
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
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

// protected imports
pub fn ceval(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = cevalWork1(
        inCache,
        inEnv,
        inExp,
        inBoolean,
        inMsg,
        numIter,
        numIter > Global::recursionDepthLimit.clone(),
    )?;
    Ok((outCache, outValue))
}

fn cevalWork1(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
    mut iterReached: bool,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (match (inMsg.clone(), iterReached) {
        (_, false) => {
            (outCache, outValue) = cevalWork2(inCache, inEnv, inExp, inBoolean, inMsg, numIter)?;
            (outCache, outValue)
        }
        (Absyn::Msg::MSG { info: mut info }, true) => {
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            str1 = intString(Global::recursionDepthLimit.clone());
            str2 = ExpressionBasics::printExpStr(inExp)?;
            Error::addSourceMessage(
                &(Error::RECURSION_DEPTH_WARNING.clone()),
                list![str1, str2, FGraph::printGraphPathStr(&inEnv)],
                metamodelica::AsArg::as_arg(&info),
            )?;
            return Err("fail");
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outCache, outValue))
}

fn cevalWork2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    pub type ReductionOperator = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Values::Value>,
                metamodelica::Ref<Values::Value>,
            ) -> Result<metamodelica::Ref<Values::Value>>
            + 'static,
    >;

    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), inBoolean, inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::ICONST { integer: i }, _, _) => {
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::INTEGER { integer: i.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::RCONST { real: r }, _, _) => {
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::REAL { real: r.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::SCONST { string: s }, _, _) => {
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::STRING { string: s.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::BCONST { bool: b }, _, _) => {
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::BOOL { boolean: b.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::ENUM_LITERAL { name, index: i }, _, _) => {
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ENUM_LITERAL { name: name.clone(), index: i.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CODE { code: Deref @ Absyn::CodeNode::C_EXPRESSION { exp }, .. }, r#impl, msg) => {
                    let mut exp_1: metamodelica::Ref<Absyn::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, exp_1) = cevalAstExp(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), &(Absyn::dummyInfo.clone()))?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_EXPRESSION { exp: exp_1.clone() }) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CODE { code: Deref @ Absyn::CodeNode::C_ELEMENT { element: elt }, .. }, r#impl, msg) => {
                    let mut elt_1: metamodelica::Ref<Absyn::Element>;
                    let mut cache = (*cache).clone();
                    (cache, elt_1) = cevalAstElt(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&elt), r#impl.clone(), msg.clone())?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_ELEMENT { element: elt_1.clone() }) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::CODE { code: c, .. }, _, _) => {
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::CODE { A: c.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, Deref @ DAE::Exp::ARRAY { array: es, ty: Deref @ DAE::Type::T_ARRAY { dims: arrayDims, .. }, .. }, r#impl, msg) => {
                            let mut es_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                            let mut v: metamodelica::Ref<Values::Value> = metamodelica::Ref::new(Values::Value::META_FAIL);
                            let mut dims: metamodelica::List<i32> = metamodelica::nil();
                            let mut cache = (*cache).clone();
                            (cache, es_1) = cevalList(cache.clone(), env.clone(), es.clone(), r#impl.clone(), msg.clone(), numIter)?;
                            v = 'mc: {
                let __mc_input = ();
                if let Ok(__v) = (|| -> Result<_> {
                            let () = __mc_input.clone() else { return Err("nomatch") };
                            let mut dims: metamodelica::List<i32>;
                            let mut v: metamodelica::Ref<Values::Value>;
                            dims = List::map(arrayDims.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| Expression::dimensionSize(&__a0))?;
                            v = metamodelica::Ref::new(Values::Value::ARRAY { valueLst: es_1.clone(), dimLst: dims.clone() });
                            Ok(v.clone())
                })() { break 'mc __v; }
                if let Ok(__v) = (|| -> Result<_> {
                            let _ = __mc_input.clone() else { return Err("nomatch") };
                            let mut v: metamodelica::Ref<Values::Value>;
                            v = ValuesMake::makeArray(es_1.clone());
                            Ok(v.clone())
                })() { break 'mc __v; }
                return Err("matchcontinue: no arm matched")
            };
                            Ok((cache.clone(), v.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, Deref @ DAE::Exp::ARRAY { array: es, ty: Deref @ DAE::Type::T_UNKNOWN { .. }, .. }, r#impl, msg) => {
                            if !((Config::getGraphicsExpMode()? && Config::getEvaluateParametersInAnnotations()?)) { return Err("guard") }
                            let mut es_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                            let mut v: metamodelica::Ref<Values::Value> = metamodelica::Ref::new(Values::Value::META_FAIL);
                            let mut dims: metamodelica::List<i32> = metamodelica::nil();
                            let mut cache = (*cache).clone();
                            (cache, es_1) = cevalList(cache.clone(), env.clone(), es.clone(), r#impl.clone(), msg.clone(), numIter)?;
                            v = 'mc: {
                let __mc_input = ();
                if let Ok(__v) = (|| -> Result<_> {
                            let () = __mc_input.clone() else { return Err("nomatch") };
                            let mut dims: metamodelica::List<i32>;
                            let mut v: metamodelica::Ref<Values::Value>;
                            dims = list![1];
                            v = metamodelica::Ref::new(Values::Value::ARRAY { valueLst: es_1.clone(), dimLst: dims.clone() });
                            Ok(v.clone())
                })() { break 'mc __v; }
                if let Ok(__v) = (|| -> Result<_> {
                            let _ = __mc_input.clone() else { return Err("nomatch") };
                            let mut v: metamodelica::Ref<Values::Value>;
                            v = ValuesMake::makeArray(es_1.clone());
                            Ok(v.clone())
                })() { break 'mc __v; }
                return Err("matchcontinue: no arm matched")
            };
                            Ok((cache.clone(), v.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::MATRIX { matrix: expll, ty: Deref @ DAE::Type::T_ARRAY { dims: arrayDims, .. }, .. }, r#impl, msg) => {
                    let mut elts: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    dims = List::map(arrayDims.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| Expression::dimensionSize(&__a0))?;
                    (cache, elts) = cevalMatrixElt(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&expll), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: elts.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::LIST { valList: expl }, r#impl, msg) => {
                    let mut es_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut cache = (*cache).clone();
                    (cache, es_1) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::LIST { valueLst: es_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BOX { exp: e1 }, r#impl, msg) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, v) = ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::UNBOX { exp: e1, .. }, r#impl, msg) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::META_BOX { value: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    v = metamodelica::Own::own(__pa1);
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CONS { car: e1, cdr: e2 }, r#impl, msg) => {
                    let mut vallst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, v) = ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    vallst = metamodelica::Own::own(__pa1);
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::LIST { valueLst: metamodelica::cons(v.clone(), vallst.clone()) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { .. }, .. }, _, _) => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::METARECORDCALL { path: funcpath, args: expl, fieldNames, index, .. }, r#impl, msg) => {
                    let mut vallst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut cache = (*cache).clone();
                    (cache, vallst) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::RECORD { record_: funcpath.clone(), orderd: vallst.clone(), comp: fieldNames.clone(), index: index.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::META_OPTION { exp: None }, _, _) => {
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::OPTION { some: None })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::META_OPTION { exp: Some(expExp) }, r#impl, msg) => {
                    let mut value: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, value) = ceval(cache.clone(), env.clone(), expExp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::OPTION { some: Some(value.clone()) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::META_TUPLE { listExp: expl }, r#impl, msg) => {
                    let mut vallst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut cache = (*cache).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    (cache, vallst) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::META_TUPLE { valueLst: vallst.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::TUPLE { PR: expl }, r#impl, msg) => {
                    let mut vallst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut cache = (*cache).clone();
                    (cache, vallst) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::TUPLE { valueLst: vallst.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, false, msg) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, v) = cevalCref(cache.clone(), env.clone(), cr.clone(), false, msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, r#impl, msg) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, v) = cevalCref(cache.clone(), env.clone(), cr.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, expExp, r#impl, msg) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, v) = cevalBuiltin(cache.clone(), env.clone(), expExp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { path: funcpath, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: 0 }, tail: Deref @ metamodelica::ListNode::Cons { head: expExp, tail: Deref @ metamodelica::ListNode::Nil } }, attr: Deref @ DAE::CallAttributes { isImpure: false, .. } }, r#impl, msg) => {
                    let mut value: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    ::match_deref::match_deref! { match &(AbsynUtil::makeNotFullyQualified(funcpath.clone())) {
                        Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    (cache, value) = ceval(cache.clone(), env.clone(), expExp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), value.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e @ Deref @ DAE::Exp::CALL { path: funcpath, expLst: expl, attr: Deref @ DAE::CallAttributes { isImpure: false, .. } }, r#impl, msg) => {
                    let mut vallst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut newval: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let false = (AbsynUtil::pathEqual(&(metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: literal!("Connection"), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("isRoot") }) })), metamodelica::AsArg::as_arg(&funcpath))) else { return Err("pattern mismatch") };
                    (cache, vallst) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    (cache, newval) = BackendCevalInterface::cevalCallFunction(cache.clone(), env.clone(), e.clone(), vallst.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), newval.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CAST { ty, exp: e }, r#impl, msg) => {
                    let mut value: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let true = (Types::isRecord(metamodelica::AsArg::as_arg(&ty))) else { return Err("pattern mismatch") };
                    (cache, value) = ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), value.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e @ Deref @ DAE::Exp::CALL { .. }, true, msg) => {
                    let mut value: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, value) = BackendCevalInterface::cevalInteractiveFunctions(cache.clone(), env.clone(), e.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), value.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, e @ Deref @ DAE::Exp::CALL { .. }, _, _) => {
                    let mut r#str: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- Ceval.ceval DAE.CALL failed: "))?;
                    r#str = ExpressionBasics::printExpStr(e.clone())?;
                    Debug::traceln(r#str.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::RECORD { path: funcpath, exps: expl, comp: fieldNames, .. }, r#impl, msg) => {
                    let mut vallst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut cache = (*cache).clone();
                    (cache, vallst) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::RECORD { record_: funcpath.clone(), orderd: vallst.clone(), comp: fieldNames.clone(), index: -1 })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::ADD { ty: Deref @ DAE::Type::T_STRING { .. } }, exp2: rh }, r#impl, msg) => {
                    let mut r#str: ArcStr;
                    let mut lhvStr: ArcStr;
                    let mut rhvStr: ArcStr;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    lhvStr = metamodelica::Own::own(__pa1);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa3, Deref @ Values::Value::STRING { string: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    rhvStr = metamodelica::Own::own(__pa4);
                    r#str = stringAppend(lhvStr.clone(), rhvStr.clone());
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::ADD { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: rh }, r#impl, msg) => {
                    let mut lhvReal: metamodelica::Real;
                    let mut rhvReal: metamodelica::Real;
                    let mut sum: metamodelica::Real;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    lhvReal = metamodelica::Own::own(__pa1);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa3, Deref @ Values::Value::REAL { real: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    rhvReal = metamodelica::Own::own(__pa4);
                    sum = lhvReal + rhvReal;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::REAL { real: sum })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::ADD_ARR { .. }, exp2: rh }, r#impl, msg) => {
                    let mut vlst1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut vlst2: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    vlst1 = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa4, Deref @ Values::Value::ARRAY { valueLst: __pa5, dimLst: _ }) => (__pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa4);
                    vlst2 = metamodelica::Own::own(__pa5);
                    reslst = ValuesUtil::addElementwiseArrayelt(&vlst1, &vlst2)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::SUB_ARR { .. }, exp2: rh }, r#impl, msg) => {
                    let mut vlst1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut vlst2: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    vlst1 = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa4, Deref @ Values::Value::ARRAY { valueLst: __pa5, dimLst: _ }) => (__pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa4);
                    vlst2 = metamodelica::Own::own(__pa5);
                    reslst = ValuesUtil::subElementwiseArrayelt(&vlst1, &vlst2)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::MUL_ARR { .. }, exp2: rh }, r#impl, msg) => {
                    let mut vlst1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut vlst2: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    vlst1 = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa4, Deref @ Values::Value::ARRAY { valueLst: __pa5, dimLst: _ }) => (__pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa4);
                    vlst2 = metamodelica::Own::own(__pa5);
                    reslst = ValuesUtil::mulElementwiseArrayelt(&vlst1, &vlst2)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::DIV_ARR { .. }, exp2: rh }, r#impl, msg) => {
                    let mut vlst1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut vlst2: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    vlst1 = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa4, Deref @ Values::Value::ARRAY { valueLst: __pa5, dimLst: _ }) => (__pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa4);
                    vlst2 = metamodelica::Own::own(__pa5);
                    reslst = ValuesUtil::divElementwiseArrayelt(&vlst1, &vlst2)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::POW_ARR2 { .. }, exp2: rh }, r#impl, msg) => {
                    let mut vlst1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut vlst2: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    vlst1 = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa4, Deref @ Values::Value::ARRAY { valueLst: __pa5, dimLst: _ }) => (__pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa4);
                    vlst2 = metamodelica::Own::own(__pa5);
                    reslst = ValuesUtil::powElementwiseArrayelt(&vlst1, &vlst2)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::MUL_ARRAY_SCALAR { .. }, exp2: rh }, r#impl, msg) => {
                    let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut aval: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut sval: metamodelica::Ref<Values::Value>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    (cache, sval) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    aval = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    reslst = ValuesUtil::multScalarArrayelt(&sval, aval.clone())?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::ADD_ARRAY_SCALAR { .. }, exp2: rh }, r#impl, msg) => {
                    let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut aval: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut sval: metamodelica::Ref<Values::Value>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    (cache, sval) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    aval = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    reslst = ValuesUtil::addScalarArrayelt(&sval, aval.clone())?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::SUB_SCALAR_ARRAY { .. }, exp2: rh }, r#impl, msg) => {
                    let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut aval: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut sval: metamodelica::Ref<Values::Value>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    (cache, sval) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    aval = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    reslst = ValuesUtil::subScalarArrayelt(&sval, aval.clone())?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::POW_SCALAR_ARRAY { .. }, exp2: rh }, r#impl, msg) => {
                    let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut aval: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut sval: metamodelica::Ref<Values::Value>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    (cache, sval) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    aval = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    reslst = ValuesUtil::powScalarArrayelt(&sval, aval.clone())?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::POW_ARRAY_SCALAR { .. }, exp2: rh }, r#impl, msg) => {
                    let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut aval: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut sval: metamodelica::Ref<Values::Value>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    (cache, sval) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    aval = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    reslst = ValuesUtil::powArrayeltScalar(&sval, aval.clone())?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::DIV_SCALAR_ARRAY { .. }, exp2: rh }, r#impl, msg) => {
                    let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut aval: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut sval: metamodelica::Ref<Values::Value>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    (cache, sval) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    aval = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    reslst = ValuesUtil::divScalarArrayelt(&sval, aval.clone())?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::DIV_ARRAY_SCALAR { .. }, exp2: rh }, r#impl, msg) => {
                    let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut aval: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut sval: metamodelica::Ref<Values::Value>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    (cache, sval) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    aval = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    reslst = ValuesUtil::divArrayeltScalar(&sval, aval.clone())?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::MUL_SCALAR_PRODUCT { .. }, exp2: rh }, r#impl, msg) => {
                    let mut rhvals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut lhvals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut resVal: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    rhvals = metamodelica::Own::own(__pa1);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa3, Deref @ Values::Value::ARRAY { valueLst: __pa4, .. }) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    lhvals = metamodelica::Own::own(__pa4);
                    resVal = ValuesUtil::multScalarProduct(rhvals.clone(), lhvals.clone())?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::MUL_MATRIX_PRODUCT { .. }, exp2: rh }, r#impl, msg) => {
                    let mut rhvals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut lhvals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut elt1: metamodelica::Ref<Values::Value>;
                    let mut elt2: metamodelica::Ref<Values::Value>;
                    let mut resVal: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa2 @ Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ }, .. }) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    elt1 = metamodelica::Own::own(__pa1);
                    lhvals = metamodelica::Own::own(__pa2);
                    let (__pa4, __pa6, __pa5) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa4, Deref @ Values::Value::ARRAY { valueLst: __pa6 @ Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: _ }, .. }) => (__pa4.clone(), __pa6.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa4);
                    elt2 = metamodelica::Own::own(__pa5);
                    rhvals = metamodelica::Own::own(__pa6);
                    let true = (ValuesUtil::isArray(&elt1)) else { return Err("pattern mismatch") };
                    let false = (ValuesUtil::isArray(&elt2)) else { return Err("pattern mismatch") };
                    resVal = ValuesUtil::multScalarProduct(lhvals.clone(), rhvals.clone())?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::MUL_MATRIX_PRODUCT { .. }, exp2: rh }, r#impl, msg) => {
                    let mut rhvals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut lhvals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut elt1: metamodelica::Ref<Values::Value>;
                    let mut elt2: metamodelica::Ref<Values::Value>;
                    let mut resVal: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa2 @ Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ }, .. }) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    elt1 = metamodelica::Own::own(__pa1);
                    rhvals = metamodelica::Own::own(__pa2);
                    let (__pa4, __pa6, __pa5) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa4, Deref @ Values::Value::ARRAY { valueLst: __pa6 @ Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: _ }, .. }) => (__pa4.clone(), __pa6.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa4);
                    elt2 = metamodelica::Own::own(__pa5);
                    lhvals = metamodelica::Own::own(__pa6);
                    let true = (ValuesUtil::isArray(&elt1)) else { return Err("pattern mismatch") };
                    let false = (ValuesUtil::isArray(&elt2)) else { return Err("pattern mismatch") };
                    resVal = ValuesUtil::multScalarProduct(lhvals.clone(), rhvals.clone())?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::MUL_MATRIX_PRODUCT { .. }, exp2: rh }, r#impl, msg) => {
                    let mut vallst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut rhvals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut lhvals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut elt1: metamodelica::Ref<Values::Value>;
                    let mut elt2: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa2 @ Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ }, dimLst: _ }) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    elt1 = metamodelica::Own::own(__pa1);
                    rhvals = metamodelica::Own::own(__pa2);
                    let (__pa4, __pa6, __pa5) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa4, Deref @ Values::Value::ARRAY { valueLst: __pa6 @ Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: _ }, dimLst: _ }) => (__pa4.clone(), __pa6.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa4);
                    elt2 = metamodelica::Own::own(__pa5);
                    lhvals = metamodelica::Own::own(__pa6);
                    let true = (ValuesUtil::isArray(&elt1)) else { return Err("pattern mismatch") };
                    let true = (ValuesUtil::isArray(&elt2)) else { return Err("pattern mismatch") };
                    vallst = ValuesUtil::multMatrix(&lhvals, rhvals.clone())?;
                    Ok((cache.clone(), ValuesMake::makeArray(vallst.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::POW { .. }, exp2: rh }, r#impl, msg) => {
                    let mut resVal: metamodelica::Ref<Values::Value>;
                    let mut lhvVal: metamodelica::Ref<Values::Value>;
                    let mut rhvVal: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, lhvVal) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    (cache, rhvVal) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    resVal = ValuesUtil::safeIntRealOp(&lhvVal, &rhvVal, openmodelica_frontend_types::Values::IntRealOp::POWOP)?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::MUL { .. }, exp2: rh }, r#impl, msg) => {
                    let mut resVal: metamodelica::Ref<Values::Value>;
                    let mut lhvVal: metamodelica::Ref<Values::Value>;
                    let mut rhvVal: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, lhvVal) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    (cache, rhvVal) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    resVal = ValuesUtil::safeIntRealOp(&lhvVal, &rhvVal, openmodelica_frontend_types::Values::IntRealOp::MULOP)?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::DIV { .. }, exp2: rh }, r#impl, msg) => {
                    let mut resVal: metamodelica::Ref<Values::Value>;
                    let mut lhvVal: metamodelica::Ref<Values::Value>;
                    let mut rhvVal: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, lhvVal) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    (cache, rhvVal) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    resVal = ValuesUtil::safeIntRealOp(&lhvVal, &rhvVal, openmodelica_frontend_types::Values::IntRealOp::DIVOP)?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::DIV { .. }, exp2: rh }, r#impl, msg @ Absyn::Msg::MSG { info }) => {
                    let mut lhvStr: ArcStr;
                    let mut rhvStr: ArcStr;
                    let mut lhvVal: metamodelica::Ref<Values::Value>;
                    (_, lhvVal) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    let true = (ValuesUtil::isZero(&lhvVal)) else { return Err("pattern mismatch") };
                    lhvStr = ExpressionBasics::printExpStr(lh.clone())?;
                    rhvStr = ExpressionBasics::printExpStr(rh.clone())?;
                    Error::addSourceMessage(&(Error::DIVISION_BY_ZERO.clone()), list![lhvStr.clone(), rhvStr.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::ADD { .. }, exp2: rh }, r#impl, msg) => {
                    let mut resVal: metamodelica::Ref<Values::Value>;
                    let mut lhvVal: metamodelica::Ref<Values::Value>;
                    let mut rhvVal: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, lhvVal) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    (cache, rhvVal) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    resVal = ValuesUtil::safeIntRealOp(&lhvVal, &rhvVal, openmodelica_frontend_types::Values::IntRealOp::ADDOP)?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp1: lh, operator: DAE::Operator::SUB { .. }, exp2: rh }, r#impl, msg) => {
                    let mut resVal: metamodelica::Ref<Values::Value>;
                    let mut lhvVal: metamodelica::Ref<Values::Value>;
                    let mut rhvVal: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, lhvVal) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    (cache, rhvVal) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    resVal = ValuesUtil::safeIntRealOp(&lhvVal, &rhvVal, openmodelica_frontend_types::Values::IntRealOp::SUBOP)?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: daeExp }, r#impl, msg) => {
                    let mut arr: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut arr_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), daeExp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    arr = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    arr_1 = List::map(arr.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::valueNeg(&__a0))?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: arr_1.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: daeExp }, r#impl, msg) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut v_1: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, v) = ceval(cache.clone(), env.clone(), daeExp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    v_1 = ValuesUtil::valueNeg(&v)?;
                    Ok((cache.clone(), v_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::LBINARY { exp1: lh, operator: DAE::Operator::AND { ty: _ }, exp2: rh }, r#impl, msg) => {
                    let mut lhvBool: bool;
                    let mut rhvBool: bool;
                    let mut resBool: bool;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    lhvBool = metamodelica::Own::own(__pa1);
                    if !(lhvBool) {
                        v = metamodelica::Ref::new(Values::Value::BOOL { boolean: false });
                    } else {
                        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                            (__pa3, Deref @ Values::Value::BOOL { boolean: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        cache = metamodelica::Own::own(__pa3);
                        rhvBool = metamodelica::Own::own(__pa4);
                        resBool = boolAnd(lhvBool, rhvBool);
                        v = metamodelica::Ref::new(Values::Value::BOOL { boolean: resBool });
                    }
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::LBINARY { exp1: lh, operator: DAE::Operator::OR { ty: _ }, exp2: rh }, r#impl, msg) => {
                    let mut lhvBool: bool;
                    let mut rhvBool: bool;
                    let mut resBool: bool;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    lhvBool = metamodelica::Own::own(__pa1);
                    if lhvBool {
                        v = metamodelica::Ref::new(Values::Value::BOOL { boolean: true });
                    } else {
                        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                            (__pa3, Deref @ Values::Value::BOOL { boolean: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        cache = metamodelica::Own::own(__pa3);
                        rhvBool = metamodelica::Own::own(__pa4);
                        resBool = boolOr(lhvBool, rhvBool);
                        v = metamodelica::Ref::new(Values::Value::BOOL { boolean: resBool });
                    }
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::LBINARY { exp1: lh, operator: DAE::Operator::OR { ty: _ }, exp2: rh }, r#impl, msg) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter)?) {
                        (__pa0, __pa1 @ Deref @ Values::Value::BOOL { boolean: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    v = metamodelica::Own::own(__pa1);
                    if '__try3: {
                        unwrap_break_err!(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter), '__try3);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e }, r#impl, msg) => {
                    let mut b: bool;
                    let mut b_1: bool;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    b = metamodelica::Own::own(__pa1);
                    b_1 = boolNot(b);
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::BOOL { boolean: b_1 })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::RELATION { exp1: lhs, operator: relop, exp2: rhs, .. }, r#impl, msg) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut lhs_1: metamodelica::Ref<Values::Value>;
                    let mut rhs_1: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, lhs_1) = ceval(cache.clone(), env.clone(), lhs.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    (cache, rhs_1) = ceval(cache.clone(), env.clone(), rhs.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    v = cevalRelation(&lhs_1, metamodelica::AsArg::as_arg(&relop), &rhs_1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Exp::RANGE { .. }, _, _) => {
                    Ok(cevalRange(inCache.clone(), inEnv.clone(), &inExp, inBoolean, inMsg.clone(), numIter)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_REAL { .. }, exp: e }, r#impl, msg) => {
                    let mut i: i32;
                    let mut r: metamodelica::Real;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    i = metamodelica::Own::own(__pa1);
                    r = intReal(i);
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::REAL { real: r })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_INTEGER { .. }, exp: e }, r#impl, msg) => {
                    let mut i: i32;
                    let mut r: metamodelica::Real;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    r = metamodelica::Own::own(__pa1);
                    i = ((r).0.floor() as i32);
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::INTEGER { integer: i })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_ENUMERATION { path, names: n, .. }, exp: e }, r#impl, msg) => {
                    let mut i: i32;
                    let mut r#str: ArcStr;
                    let mut cache = (*cache).clone();
                    let mut path = (*path).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    i = metamodelica::Own::own(__pa1);
                    r#str = (n).get(i)?;
                    path = AbsynUtil::joinPaths(path.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: r#str.clone() }))?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ENUM_LITERAL { name: path.clone(), index: i })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_REAL { .. }, .. }, exp: e }, r#impl, msg) => {
                    let mut ivals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut rvals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut dims: metamodelica::List<i32>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    ivals = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    rvals = ValuesUtil::typeConvert(DAE::T_INTEGER_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), &ivals)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: rvals.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: e1, expElse: e2 }, r#impl, msg) => {
                    let mut resBool: bool;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, v) = ceval(cache.clone(), env.clone(), cond.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    let __pa0 = ::match_deref::match_deref! { match &(v.clone()) {
                        Deref @ Values::Value::BOOL { boolean: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    resBool = metamodelica::Own::own(__pa0);
                    (cache, v) = ceval(cache.clone(), env.clone(), if (resBool) {e1.clone()} else {e2.clone()}, r#impl.clone(), msg.clone(), numIter)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::ASUB { exp: e, sub: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: indx } }, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, msg) => {
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    vals = metamodelica::Own::own(__pa1);
                    v = (vals).get(indx.clone())?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, Deref @ DAE::Exp::ASUB { exp: e, sub: subs }, r#impl, msg) => {
                            let mut es_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut v: metamodelica::Ref<Values::Value>;
                            let mut dims: metamodelica::List<i32>;
                            let mut cache = (*cache).clone();
                            expl = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (subs.clone()).into_iter().cloned() {
                            let __x = Expression::getSubscriptExp(&(sub.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                                (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            cache = metamodelica::Own::own(__pa0);
                            vals = metamodelica::Own::own(__pa1);
                            dims = metamodelica::Own::own(__pa2);
                            (cache, es_1) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter)?;
                            v = (es_1).head().cloned()?;
                            v = ValuesUtil::nthnthArrayelt(es_1.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: vals.clone(), dimLst: dims.clone() }), v.clone())?;
                            Ok((cache.clone(), v.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::TSUB { exp: e, ix: indx, .. }, r#impl, msg) => {
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::TUPLE { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    vals = metamodelica::Own::own(__pa1);
                    v = (vals).get(indx.clone())?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { iterType, path, foldName, resultName, foldExp, defaultValue: ov, exprType: ty }, expr: daeExp, iterators }, r#impl, msg) => {
                    let mut value: metamodelica::Ref<Values::Value>;
                    let mut dims: metamodelica::List<i32>;
                    let mut names: metamodelica::List<ArcStr>;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut valMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ov = (*ov).clone();
                    env = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, arcstr::literal!(FCore::forScopeName), None)?;
                    (cache, valMatrix, names, dims, tys) = cevalReductionIterators(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&iterators), r#impl.clone(), msg.clone(), numIter + 1)?;
                    valMatrix = makeReductionAllCombinations(valMatrix.clone(), iterType.clone())?;
                    (cache, ov) = cevalReduction(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&path), ov.clone(), metamodelica::AsArg::as_arg(&daeExp), metamodelica::AsArg::as_arg(&ty), metamodelica::AsArg::as_arg(&foldName), metamodelica::AsArg::as_arg(&resultName), foldExp.clone(), &names, valMatrix.clone().reverse(), &tys, r#impl.clone(), metamodelica::AsArg::as_arg(&msg), numIter + 1)?;
                    value = Util::getOptionOrDefault(ov.clone(), openmodelica_frontend_types::Values::Value::interned_META_FAIL());
                    value = backpatchArrayReduction(metamodelica::AsArg::as_arg(&path), iterType.clone(), value.clone(), dims.clone())?;
                    Ok((cache.clone(), value.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Exp::EMPTY { .. }, _, _) => {
                    let mut s: ArcStr;
                    let mut v: metamodelica::Ref<Values::Value>;
                    s = ComponentReferenceBasics::printComponentRefStr(var_field!((*inExp).name, DAE::Exp::EMPTY))?;
                    v = Types::typeToValue(var_field!((*inExp).ty, DAE::Exp::EMPTY))?;
                    Ok((inCache.clone(), metamodelica::Ref::new(Values::Value::EMPTY { scope: var_field!((*inExp).scope, DAE::Exp::EMPTY).clone(), name: s.clone(), ty: v.clone(), tyStr: var_field!((*inExp).tyStr, DAE::Exp::EMPTY).clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    if !((Config::getGraphicsExpMode()?)) { return Err("guard") }
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    ty = Expression::r#typeof(inExp.clone())?;
                    v = Types::typeToValue(&ty)?;
                    Ok((inCache.clone(), metamodelica::Ref::new(Values::Value::EMPTY { scope: literal!("#graphicsExp#"), name: ExpressionBasics::printExpStr(inExp.clone())?, ty: v.clone(), tyStr: TypesDump::unparseType(ty.clone())? })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, e, _, _) => {
                    let true = (Flags::isSet(Flags::CEVAL.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Ceval.ceval failed: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); ArcStr::from(__mm_s) })?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  Scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env))); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

pub fn cevalIfConstant(
    mut cache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut prop: DAE::Properties,
    mut r#impl: bool,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut cache: FCore::Cache = cache;
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut prop: DAE::Properties = prop;
    if Expression::isEvaluatedConst(&exp) {
        return Ok((cache, exp, prop));
    }
    (cache, exp, prop) = 'mc: {
        let __mc_input = prop.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP {
                constFlag: DAE::Const::C_PARAM { .. },
                type_: ref tp,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            if !(!(Flags::getConfigBool(Flags::CEVAL_EQUATION.clone())?)) {
                return Err("guard");
            }
            Ok((
                cache.clone(),
                exp.clone(),
                DAE::Properties::PROP {
                    type_: tp.clone(),
                    constFlag: openmodelica_frontend_types::DAE::Const::C_VAR,
                },
            ))
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let DAE::Properties::PROP {
                constFlag: DAE::Const::C_CONST { .. },
                type_: ref tp,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut v: metamodelica::Ref<Values::Value>;
            let mut cache: FCore::Cache = cache.clone();
            let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
            (cache, v) = ceval(
                cache.clone(),
                inEnv.clone(),
                exp.clone(),
                r#impl,
                Absyn::Msg::MSG { info: inInfo.clone() },
                0,
            )?;
            exp = ValuesUtil::valueExp(v.clone(), Some(exp.clone()))?;
            exp = ValuesUtil::fixZeroSizeArray(exp.clone(), tp.clone())?;
            Ok(((cache.clone(), exp.clone(), prop.clone()), cache.clone(), exp.clone()))
        })() {
            cache = __wb0;
            exp = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let DAE::Properties::PROP_TUPLE { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut v: metamodelica::Ref<Values::Value>;
            let mut cache: FCore::Cache = cache.clone();
            let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
            let DAE::C_CONST { .. } = (Types::propAllConst(prop.clone())?) else {
                return Err("pattern mismatch");
            };
            (cache, v) = ceval(
                cache.clone(),
                inEnv.clone(),
                exp.clone(),
                false,
                Absyn::Msg::MSG { info: inInfo.clone() },
                0,
            )?;
            exp = ValuesUtil::valueExp(v.clone(), Some(exp.clone()))?;
            Ok(((cache.clone(), exp.clone(), prop.clone()), cache.clone(), exp.clone()))
        })() {
            cache = __wb0;
            exp = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP_TUPLE { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            if !(!(Flags::getConfigBool(Flags::CEVAL_EQUATION.clone())?)) {
                return Err("guard");
            }
            let DAE::C_PARAM { .. } = (Types::propAllConst(prop.clone())?) else {
                return Err("pattern mismatch");
            };
            metamodelica::print(literal!(" tuple non constant evaluation not implemented yet\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if !(Expression::isConst(exp.clone())? && !(Config::acceptMetaModelicaGrammar()?)) {
                return Err("guard");
            }
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
            (_, v) = ceval(
                cache.clone(),
                inEnv.clone(),
                exp.clone(),
                r#impl,
                Absyn::Msg::MSG { info: inInfo.clone() },
                0,
            )?;
            exp = ValuesUtil::valueExp(v.clone(), Some(exp.clone()))?;
            exp = ValuesUtil::fixZeroSizeArray(exp.clone(), Types::getPropType(&prop))?;
            Ok(((cache.clone(), exp.clone(), prop.clone()), exp.clone()))
        })() {
            exp = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
            (exp, _) = ExpressionSimplify::simplify1(exp.clone())?;
            Ok(((cache.clone(), exp.clone(), prop.clone()), exp.clone()))
        })() {
            exp = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((cache, exp, prop))
}

fn cevalWholedimRetCall(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inInfo: SourceInfo,
    mut numIter: i32,
) -> Result<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProp: DAE::Properties;
    (outExp, outProp) = (::match_deref::match_deref! { match &(inExp) {
        e @ Deref @ DAE::Exp::CALL { path: p, expLst: el, attr: attr @ Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_ARRAY { dims, .. }, .. } } => {
            let mut v: metamodelica::Ref<Values::Value>;
            let mut cevalType: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut attr = (*attr).clone();
            let true = (Expression::arrayContainWholeDimension(metamodelica::AsArg::as_arg(&dims))) else { return Err("pattern mismatch") };
            (_, v) = ceval(inCache, inEnv, e.clone(), true, Absyn::Msg::MSG { info: inInfo }, numIter + 1)?;
            ty = Types::typeOfValue(v)?;
            cevalType = Types::simplifyType(ty.clone())?;
            assign_field!(attr.ty = cevalType);
            (metamodelica::Ref::new(DAE::Exp::CALL { path: p.clone(), expLst: el.clone(), attr: attr.clone() }), DAE::Properties::PROP { type_: ty, constFlag: openmodelica_frontend_types::DAE::Const::C_PARAM })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outExp, outProp))
}

pub(crate) fn cevalRangeIfConstant(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inProp: DAE::Properties,
    mut r#impl: bool,
    mut inInfo: SourceInfo,
) -> (FCore::Cache, metamodelica::Ref<DAE::Exp>) {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    (outCache, outExp) = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { ty, start: e1, stop: e2, step: e3 } => {
                    let mut cache: FCore::Cache;
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    (cache, e1, _) = cevalIfConstant(inCache.clone(), inEnv.clone(), e1.clone(), inProp.clone(), r#impl, inInfo.clone())?;
                    (_, e2, _) = cevalIfConstant(cache.clone(), inEnv.clone(), e2.clone(), inProp.clone(), r#impl, inInfo.clone())?;
                    Ok((inCache.clone(), metamodelica::Ref::new(DAE::Exp::RANGE { ty: ty.clone(), start: e1.clone(), step: e3.clone(), stop: e2.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inCache.clone(), inExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outCache, outExp)
}

fn cevalBuiltin(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    pub type HandlerFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                FCore::Cache,
                FCore::Graph,
                metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                bool,
                Absyn::Msg,
                i32,
            ) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)>
            + 'static,
    >;

    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache, inEnv, inExp, inBoolean, inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::SIZE { exp, sz: Some(dim) }, r#impl, msg) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, v) = cevalBuiltinSize(cache.clone(), env.clone(), exp.clone(), dim.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::SIZE { exp, sz: None }, r#impl, msg) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, v) = cevalBuiltinSizeMatrix(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { path, expLst: args, attr: Deref @ DAE::CallAttributes { builtin: true, .. } }, r#impl, msg) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut handler: Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>;
                    let mut id: ArcStr;
                    let mut cache = (*cache).clone();
                    id = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
                    handler = cevalBuiltinHandler(id.clone())?;
                    (cache, v) = handler(cache.clone(), env.clone(), args.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e @ Deref @ DAE::Exp::CALL { expLst: expl, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, r#impl, msg) => {
                    let mut newval: metamodelica::Ref<Values::Value>;
                    let mut vallst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut cache = (*cache).clone();
                    (cache, vallst) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    (cache, newval) = BackendCevalInterface::cevalCallFunction(cache.clone(), env.clone(), e.clone(), vallst.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), newval.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

fn cevalBuiltinHandler(
    mut inIdent: ArcStr,
) -> Result<
    Arc<
        dyn ::std::ops::Fn(
                FCore::Cache,
                FCore::Graph,
                metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                bool,
                Absyn::Msg,
                i32,
            ) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)>
            + 'static,
    >,
> {
    pub type HandlerFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                FCore::Cache,
                FCore::Graph,
                metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                bool,
                Absyn::Msg,
                i32,
            ) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)>
            + 'static,
    >;

    let mut handler: Arc<
        dyn ::std::ops::Fn(
                FCore::Cache,
                FCore::Graph,
                metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                bool,
                Absyn::Msg,
                i32,
            ) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)>
            + 'static,
    >;
    handler = (::match_deref::match_deref! { match &(inIdent) {
        Deref @ "floor" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinFloor(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "ceil" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinCeil(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "abs" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinAbs(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "sqrt" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinSqrt(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "nthRoot" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinNthRoot(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "div" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinDiv(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "sin" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinSin(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "cos" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinCos(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "tan" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinTan(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "sinh" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinSinh(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "cosh" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinCosh(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "tanh" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinTanh(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "asin" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinAsin(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "acos" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinAcos(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "atan" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinAtan(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "atan2" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinAtan2(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "log" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinLog(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "log10" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinLog10(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "integer" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinInteger(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "boolean" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinBoolean(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "mod" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinMod(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "max" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinMax(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "min" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinMin(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "rem" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinRem(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "sum" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinSum(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "diagonal" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinDiagonal(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "sign" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinSign(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "exp" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinExp(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "noEvent" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinNoevent(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "cat" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinCat(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "identity" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinIdentity(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "promote" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinPromote(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "String" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinString(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "Integer" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinIntegerEnumeration(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "rooted" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinRooted(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "cross" => {
            (std::sync::Arc::new(cevalBuiltinCross) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "fill" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinFill(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "Modelica.Utilities.Strings.substring" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinSubstring(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "print" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalBuiltinPrint(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "fail" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| -> metamodelica::Result<_> { ::std::result::Result::Ok(cevalBuiltinFail(__a0, &__a1, &__a2, __a3, &__a4, __a5)) }) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "intString" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalIntString(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "realString" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalRealString(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "stringCharInt" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalStringCharInt(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "intStringChar" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalIntStringChar(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "stringLength" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalStringLength(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "stringInt" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalStringInt(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "stringListStringChar" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalStringListStringChar(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "listStringCharString" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalListStringCharString(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "stringAppendList" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalStringAppendList(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "stringDelimitList" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalStringDelimitList(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "listLength" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalListLength(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "listAppend" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalListAppend(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "listReverse" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalListReverse(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "listHead" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalListFirst(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "listRest" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalListRest(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "listMember" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalListMember(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "anyString" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalAnyString(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "listArrayLiteral" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalListArrayLiteral(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "intBitAnd" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalIntBitAnd(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "intBitOr" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalIntBitOr(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "intBitXor" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalIntBitXor(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "intBitLShift" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalIntBitLShift(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "intBitRShift" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalIntBitRShift(__a0, __a1, &__a2, __a3, __a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "numBits" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalNumBits(__a0, &__a1, &__a2, __a3, &__a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        Deref @ "integerMax" => {
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a3: bool, __a4: Absyn::Msg, __a5: i32| cevalIntegerMax(__a0, &__a1, &__a2, __a3, &__a4, __a5)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> + 'static>)
        },
        id => {
            let true = (Flags::isSet(Flags::CEVAL.clone())?) else { return Err("pattern mismatch") };
            Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("No cevalBuiltinHandler found for ")); __mm_s.push_str(&*id); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(handler)
}

pub fn cevalKnownExternalFuncs(
    mut inCache: &FCore::Cache,
    mut env: &FCore::Graph,
    mut funcpath: &metamodelica::Ref<Absyn::Path>,
    mut vals: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut msg: &Absyn::Msg,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut res: metamodelica::Ref<Values::Value>;
    let mut cdef: metamodelica::Ref<SCode::Element>;
    let mut env_1: FCore::Graph;
    let mut fid: ArcStr;
    let mut id: ArcStr;
    let mut oid: Option<ArcStr>;
    let mut extdecl: Option<metamodelica::Ref<SCode::ExternalDecl>>;
    let mut funcRest: SCode::FunctionRestriction;
    (outCache, cdef, env_1) = Lookup::lookupClass(inCache, env, funcpath, None)?;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(cdef) {
        Deref @ SCode::Element::CLASS { name: __pa0, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: __pa1 }, classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: __pa2, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fid = metamodelica::Own::own(__pa0);
    funcRest = metamodelica::Own::own(__pa1);
    extdecl = metamodelica::Own::own(__pa2);
    let SCode::FR_EXTERNAL_FUNCTION { purity: _ } = (funcRest) else {
        return Err("pattern mismatch");
    };
    let __pa4 = ::match_deref::match_deref! { match &(extdecl) {
        Some(Deref @ SCode::ExternalDecl { funcName: __pa4, lang: _, output_: _, args: _, annotation_: _ }) => __pa4.clone(),
        _ => return Err("pattern mismatch"),
    } };
    oid = metamodelica::Own::own(__pa4);
    id = Util::getOptionOrDefault(oid, fid);
    isKnownExternalFunc(&id)?;
    res = cevalKnownExternalFuncs2(&id, vals, msg)?;
    Ok((outCache, res))
}

pub(crate) fn isKnownExternalFunc(mut id: &ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(id.clone()) {
        Deref @ "acos" => (),
        Deref @ "asin" => (),
        Deref @ "atan" => (),
        Deref @ "atan2" => (),
        Deref @ "cos" => (),
        Deref @ "cosh" => (),
        Deref @ "exp" => (),
        Deref @ "log" => (),
        Deref @ "log10" => (),
        Deref @ "sin" => (),
        Deref @ "sinh" => (),
        Deref @ "tan" => (),
        Deref @ "tanh" => (),
        Deref @ "print" => (),
        Deref @ "ModelicaStreams_closeFile" => (),
        Deref @ "ModelicaStrings_substring" => (),
        Deref @ "ModelicaStrings_length" => (),
        Deref @ "ModelicaInternal_print" => (),
        Deref @ "ModelicaInternal_countLines" => (),
        Deref @ "ModelicaInternal_readLine" => (),
        Deref @ "ModelicaInternal_stat" => (),
        Deref @ "ModelicaInternal_fullPathName" => (),
        Deref @ "ModelicaStrings_compare" => (),
        Deref @ "ModelicaStrings_scanReal" => (),
        Deref @ "ModelicaStrings_skipWhiteSpace" => (),
        Deref @ "ModelicaError" => (),
        Deref @ "OpenModelica_regex" => (),
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn cevalKnownExternalFuncs2(
    mut id: &ArcStr,
    mut inValuesValueLst: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inMsg: &Absyn::Msg,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = (::match_deref::match_deref! { match &((id.clone(), &**inValuesValueLst)) {
        (Deref @ "acos", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rv_1: metamodelica::Real;
            let true = (rv.clone() >= metamodelica::OrderedFloat(-1.0_f64) && rv.clone() <= metamodelica::OrderedFloat(1.0_f64)) else { return Err("pattern mismatch") };
            rv_1 = (rv.clone()).acos();
            metamodelica::Ref::new(Values::Value::REAL { real: rv_1 })
        },
        (Deref @ "asin", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rv_1: metamodelica::Real;
            let true = (rv.clone() >= metamodelica::OrderedFloat(-1.0_f64) && rv.clone() <= metamodelica::OrderedFloat(1.0_f64)) else { return Err("pattern mismatch") };
            rv_1 = (rv.clone()).asin();
            metamodelica::Ref::new(Values::Value::REAL { real: rv_1 })
        },
        (Deref @ "atan", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rv_1: metamodelica::Real;
            rv_1 = (rv.clone()).atan();
            metamodelica::Ref::new(Values::Value::REAL { real: rv_1 })
        },
        (Deref @ "atan2", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rv1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rv2 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut rv_1: metamodelica::Real;
            rv_1 = (rv1.clone()).atan2(rv2.clone());
            metamodelica::Ref::new(Values::Value::REAL { real: rv_1 })
        },
        (Deref @ "cos", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rv_1: metamodelica::Real;
            rv_1 = (rv.clone()).cos();
            metamodelica::Ref::new(Values::Value::REAL { real: rv_1 })
        },
        (Deref @ "cosh", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rv_1: metamodelica::Real;
            rv_1 = (rv.clone()).cosh();
            metamodelica::Ref::new(Values::Value::REAL { real: rv_1 })
        },
        (Deref @ "exp", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rv_1: metamodelica::Real;
            rv_1 = (rv.clone()).exp();
            metamodelica::Ref::new(Values::Value::REAL { real: rv_1 })
        },
        (Deref @ "log", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rv_1: metamodelica::Real;
            let true = (rv.clone() > metamodelica::OrderedFloat((0) as f64)) else { return Err("pattern mismatch") };
            rv_1 = (rv.clone()).ln();
            metamodelica::Ref::new(Values::Value::REAL { real: rv_1 })
        },
        (Deref @ "log10", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rv_1: metamodelica::Real;
            let true = (rv.clone() > metamodelica::OrderedFloat((0) as f64)) else { return Err("pattern mismatch") };
            rv_1 = (rv.clone()).log10();
            metamodelica::Ref::new(Values::Value::REAL { real: rv_1 })
        },
        (Deref @ "sin", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rv_1: metamodelica::Real;
            rv_1 = (rv.clone()).sin();
            metamodelica::Ref::new(Values::Value::REAL { real: rv_1 })
        },
        (Deref @ "sinh", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rv_1: metamodelica::Real;
            rv_1 = (rv.clone()).sinh();
            metamodelica::Ref::new(Values::Value::REAL { real: rv_1 })
        },
        (Deref @ "tan", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rv_1: metamodelica::Real;
            rv_1 = (rv.clone()).tan();
            metamodelica::Ref::new(Values::Value::REAL { real: rv_1 })
        },
        (Deref @ "tanh", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rv_1: metamodelica::Real;
            rv_1 = (rv.clone()).tanh();
            metamodelica::Ref::new(Values::Value::REAL { real: rv_1 })
        },
        (Deref @ "ModelicaStrings_substring", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: start }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: stop }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
            let mut r#str = (*r#str).clone();
            r#str = substring(r#str.clone(), start.clone(), stop.clone())?;
            metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() })
        },
        (Deref @ "ModelicaStrings_length", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut i: i32;
            i = ((r#str).len() as i32);
            metamodelica::Ref::new(Values::Value::INTEGER { integer: i })
        },
        (Deref @ "print", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            metamodelica::print(r#str.clone());
            openmodelica_frontend_types::Values::Value::interned_NORETCALL()
        },
        (Deref @ "OpenModelica_regex", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: re }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: extended }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: insensitive }, tail: Deref @ metamodelica::ListNode::Nil } } } } }) => {
            let mut n: i32;
            let mut strs: metamodelica::List<ArcStr>;
            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut v: metamodelica::Ref<Values::Value>;
            (n, strs) = System::regex(r#str.clone(), re.clone(), i.clone(), extended.clone(), insensitive.clone());
            vals = List::map(strs, &fnptr!(ValuesMake::makeString, ArcStr))?;
            v = metamodelica::Ref::new(Values::Value::ARRAY { valueLst: vals, dimLst: list![i.clone()] });
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::INTEGER { integer: n }), v] })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outValue)
}

pub(crate) static EnumCompareLess: std::sync::LazyLock<metamodelica::Ref<Absyn::Path>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: literal!("Modelica"),
            path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                name: literal!("Utilities"),
                path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                    name: literal!("Types"),
                    path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                        name: literal!("Compare"),
                        path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Less") }),
                    }),
                }),
            }),
        })
    });

pub(crate) static EnumCompareEqual: std::sync::LazyLock<metamodelica::Ref<Absyn::Path>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: literal!("Modelica"),
            path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                name: literal!("Utilities"),
                path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                    name: literal!("Types"),
                    path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                        name: literal!("Compare"),
                        path: metamodelica::Ref::new(Absyn::Path::IDENT {
                            name: literal!("Equal"),
                        }),
                    }),
                }),
            }),
        })
    });

pub(crate) static EnumCompareGreater: std::sync::LazyLock<metamodelica::Ref<Absyn::Path>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: literal!("Modelica"),
            path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                name: literal!("Utilities"),
                path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                    name: literal!("Types"),
                    path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                        name: literal!("Compare"),
                        path: metamodelica::Ref::new(Absyn::Path::IDENT {
                            name: literal!("Greater"),
                        }),
                    }),
                }),
            }),
        })
    });

fn cevalMatrixElt(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inMatrix: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<Values::Value>>)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outValues: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
    let mut v: metamodelica::Ref<Values::Value>;
    let mut vl: metamodelica::List<metamodelica::Ref<Values::Value>>;
    for mut expl in &**inMatrix {
        (outCache, vl) = cevalList(outCache, inEnv.clone(), expl.clone(), inBoolean, inMsg.clone(), numIter)?;
        v = ValuesMake::makeArray(vl);
        outValues = metamodelica::cons(v, outValues);
    }
    outValues = metamodelica::Dangerous::listReverseInPlace(outValues);
    Ok((outCache, outValues))
}

fn cevalBuiltinSize(
    mut inCache: FCore::Cache,
    mut inEnv1: FCore::Graph,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inDimExp: metamodelica::Ref<DAE::Exp>,
    mut inBoolean4: bool,
    mut inMsg6: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache, inEnv1, inExp2.clone(), inDimExp, inBoolean4, inMsg6);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::MATRIX { matrix: mat, .. }, Deref @ DAE::Exp::ICONST { integer: 1 }, _, _) => {
                    let mut i: i32;
                    i = ((mat).len() as i32);
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::INTEGER { integer: i })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::MATRIX { matrix: mat, .. }, Deref @ DAE::Exp::ICONST { integer: 2 }, _, _) => {
                    let mut i: i32;
                    i = ((((mat).head().cloned()?)).len() as i32);
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::INTEGER { integer: i })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::MATRIX { matrix: mat, .. }, Deref @ DAE::Exp::ICONST { integer: dim }, r#impl, msg) => {
                    let mut dim_1: i32;
                    let mut i: i32;
                    let mut bl: bool;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    bl = dim.clone() > 2;
                    let true = (bl) else { return Err("pattern mismatch") };
                    dim_1 = dim.clone() - 2;
                    e = (((mat).head().cloned()?)).head().cloned()?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalBuiltinSize(cache.clone(), env.clone(), e.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: dim_1 }), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    i = metamodelica::Own::own(__pa1);
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::INTEGER { integer: i })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, dimExp, r#impl, msg) => {
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut sizelst: metamodelica::List<i32>;
                    let mut dim: i32;
                    let mut i: i32;
                    let mut cache = (*cache).clone();
                    (cache, _, tp, _, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), cr.clone())?;
                    let true = (Types::dimensionsKnown(tp.clone())) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(Types::getDimensionSizes(&tp)?) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    sizelst = metamodelica::Own::own(__pa0);
                    let (__pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), dimExp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa1, Deref @ Values::Value::INTEGER { integer: __pa2 }) => (__pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa1);
                    dim = metamodelica::Own::own(__pa2);
                    i = (sizelst).get(dim)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::INTEGER { integer: i })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, dimExp, r#impl @ false, msg) => {
                    let mut dimv: i32;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut v2: metamodelica::Ref<Values::Value>;
                    let mut ddim: metamodelica::Ref<DAE::Dimension>;
                    let mut cache = (*cache).clone();
                    (cache, dims) = InstUtil::elabComponentArraydimFromEnv(cache.clone(), env.clone(), cr.clone(), Absyn::dummyInfo.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), dimExp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    dimv = metamodelica::Own::own(__pa1);
                    ddim = (dims).get(dimv)?;
                    (cache, v2) = cevalDimension(cache.clone(), env.clone(), &ddim, r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), v2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, dimExp, false, Absyn::Msg::MSG { info }) => {
                            let mut tp: metamodelica::Ref<DAE::Type>;
                            let mut binding: metamodelica::Ref<DAE::Binding>;
                            let mut cr_str: ArcStr;
                            let mut dim_str: ArcStr;
                            let mut size_str: ArcStr;
                            let mut expstr: ArcStr;
                            (_, _, tp, binding, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), cr.clone())?;
                            if !(Types::dimensionsKnown(tp.clone())) {
                                cr_str = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
                                dim_str = ExpressionBasics::printExpStr(dimExp.clone())?;
                                size_str = stringAppendList(list![literal!("size("), cr_str.clone(), literal!(", "), dim_str.clone(), literal!(")")]);
                                Error::addSourceMessage(&(Error::DIMENSION_NOT_KNOWN.clone()), list![size_str.clone()], metamodelica::AsArg::as_arg(&info))?;
                            } else {
                                let _ = (match &*binding {
                DAE::Binding::UNBOUND { .. } => {
                            expstr = ExpressionBasics::printExpStr(inExp2.clone())?;
                            Error::addSourceMessage(&(Error::UNBOUND_VALUE.clone()), list![expstr.clone()], metamodelica::AsArg::as_arg(&info))?;
                            return Err("fail")
                },
                _ => return Err("match: no arm matched"),
            });
                            }
                            Ok(return Err("fail"))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, dimExp, r#impl, msg) => {
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut dimv: i32;
                    let mut v2: metamodelica::Ref<Values::Value>;
                    let mut val: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, _, _, binding, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), cr.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), dimExp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    dimv = metamodelica::Own::own(__pa1);
                    (cache, val) = cevalCrefBinding(cache.clone(), env.clone(), cr.clone(), &binding, r#impl.clone(), msg.clone(), numIter + 1)?;
                    v2 = cevalBuiltinSize2(&val, dimv)?;
                    Ok((cache.clone(), v2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Cons { head: exp, tail: es }, .. }, dimExp, r#impl, msg) => {
                    let mut len: i32;
                    let mut cache = (*cache).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), dimExp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: 1 }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    len = (((metamodelica::cons(exp.clone(), es.clone()))).len() as i32);
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::INTEGER { integer: len })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, exp, dimExp, r#impl, msg) => {
                            let mut adims: metamodelica::List<i32>;
                            let mut dimv: i32;
                            let mut v2: metamodelica::Ref<Values::Value>;
                            let mut val: metamodelica::Ref<Values::Value>;
                            let mut cache = (*cache).clone();
                            (cache, val) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), dimExp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                                (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            cache = metamodelica::Own::own(__pa0);
                            dimv = metamodelica::Own::own(__pa1);
                            v2 = (::match_deref::match_deref! { match &(&*val) {
                Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Nil, dimLst: __esc_adims } => {
                            adims = (*__esc_adims).clone();
                            metamodelica::Ref::new(Values::Value::INTEGER { integer: (adims).get(dimv)? })
                },
                _ => cevalBuiltinSize2(&val, dimv)?,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            Ok((cache.clone(), v2.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, exp, _, _, Absyn::Msg::MSG { .. }) => {
                    let mut expstr: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Print::printErrorBuf(literal!("#-- Ceval.cevalBuiltinSize failed: "))?;
                    expstr = ExpressionBasics::printExpStr(exp.clone())?;
                    Print::printErrorBuf(expstr.clone())?;
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
    Ok((outCache, outValue))
}

fn cevalBuiltinSize2(
    mut inValue: &metamodelica::Ref<Values::Value>,
    mut inInteger: i32,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = (&**inValue, inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: lst, .. }, 1) => {
                    let mut dim: i32;
                    dim = ((lst).len() as i32);
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: dim }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: l, tail: _ }, .. }, ind) => {
                    let mut ind_1: i32;
                    let mut dimVal: metamodelica::Ref<Values::Value>;
                    ind_1 = ind.clone() - 1;
                    dimVal = cevalBuiltinSize2(metamodelica::AsArg::as_arg(&l), ind_1)?;
                    Ok(dimVal.clone())
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
                    Debug::trace(literal!("- Ceval.cevalBuiltinSize2 failed\n"))?;
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

fn cevalBuiltinSize3(
    mut inDims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inIndex: i32,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut v: i32;
    let __pa0 = ::match_deref::match_deref! { match &((inDims).get(inIndex)?) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    v = metamodelica::Own::own(__pa0);
    outValue = metamodelica::Ref::new(Values::Value::INTEGER { integer: v });
    Ok(outValue)
}

fn cevalBuiltinAbs(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inExpExpLst, inBoolean, inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil }, r#impl, msg) => {
                    let mut rv: metamodelica::Real;
                    let mut rv_1: metamodelica::Real;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    rv = metamodelica::Own::own(__pa1);
                    rv_1 = realAbs(rv);
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::REAL { real: rv_1 })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil }, r#impl, msg) => {
                    let mut iv: i32;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    iv = metamodelica::Own::own(__pa1);
                    iv = intAbs(iv);
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::INTEGER { integer: iv })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

fn cevalBuiltinSign(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut b1: bool;
            let mut b2: bool;
            let mut b3: bool;
            let mut iv: i32;
            let mut iv_1: i32;
            let mut v: metamodelica::Ref<Values::Value>;
            (cache, v) = ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?;
            (b1, b2, b3) = (match &*v {
        Values::Value::REAL { real: __esc_rv } => {
            rv = (*__esc_rv).clone();
            (rv.clone() > metamodelica::OrderedFloat(0.0_f64), rv.clone() < metamodelica::OrderedFloat(0.0_f64), rv.clone() == metamodelica::OrderedFloat(0.0_f64))
        },
        Values::Value::INTEGER { integer: __esc_iv } => {
            iv = (*__esc_iv).clone();
            (iv.clone() > 0, iv.clone() < 0, iv.clone() == 0)
        },
        _ => return Err("match: no arm matched"),
    });
            let __pa0 = ::match_deref::match_deref! { match &(List::select(list![(b1, 1), (b2, -1), (b3, 0)], std::sync::Arc::new(fnptr!(Util::tuple21, _)))?) {
                Deref @ metamodelica::ListNode::Cons { head: (_, __pa0), tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            iv_1 = metamodelica::Own::own(__pa0);
            (cache, metamodelica::Ref::new(Values::Value::INTEGER { integer: iv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinExp(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            rv_1 = (rv).exp();
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinNoevent(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut v: metamodelica::Ref<Values::Value>;
            (cache, v) = ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?;
            (cache, v)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinCat(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: dim, tail: matrices } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut dim_int: i32;
            let mut mat_lst: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut v: metamodelica::Ref<Values::Value>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), dim.clone(), r#impl, msg.clone(), numIter + 1)?) {
                (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            dim_int = metamodelica::Own::own(__pa1);
            (cache, mat_lst) = cevalList(cache, env, matrices.clone(), r#impl, msg, numIter)?;
            v = cevalCat(mat_lst, dim_int)?;
            (cache, v)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinIdentity(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut dimension: i32;
            let mut res: metamodelica::Ref<Values::Value>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, dim.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            dimension = metamodelica::Own::own(__pa1);
            res = metamodelica::Ref::new(Values::Value::ARRAY { valueLst: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut j in (1..=dimension).into_iter() {
            let __x = metamodelica::Ref::new(Values::Value::ARRAY { valueLst: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut i in (1..=dimension).into_iter() {
            let __x = if (i.clone() == j.clone()) {metamodelica::Ref::new(Values::Value::INTEGER { integer: 1 })} else {metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 })};
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), dimLst: list![dimension] });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), dimLst: list![dimension, dimension] });
            (cache, res)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinPromote(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: arr, tail: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut arr_val: metamodelica::Ref<Values::Value>;
            let mut res: metamodelica::Ref<Values::Value>;
            let mut dim_val: i32;
            let mut dims: metamodelica::List<i32>;
            let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), arr.clone(), r#impl, msg.clone(), numIter + 1)?) {
                (__pa0, __pa2 @ Deref @ Values::Value::ARRAY { dimLst: __pa1, .. }) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            dims = metamodelica::Own::own(__pa1);
            arr_val = metamodelica::Own::own(__pa2);
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(ceval(cache, env, dim.clone(), r#impl, msg, numIter + 1)?) {
                (__pa4, Deref @ Values::Value::INTEGER { integer: __pa5 }) => (__pa4.clone(), __pa5.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa4);
            dim_val = metamodelica::Own::own(__pa5);
            res = cevalBuiltinPromote2(arr_val, dim_val - ((dims).len() as i32))?;
            (cache, res)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinPromote2(
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inInteger: i32,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = (inValue.clone(), inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, 0) => {
                    Ok(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: list![v.clone()], dimLst: list![1] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: vs, dimLst: Deref @ metamodelica::ListNode::Cons { head: i, tail: _ } }, n) => {
                    let mut n_1: i32;
                    let mut vs_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut il: metamodelica::List<i32>;
                    n_1 = n.clone() - 1;
                    if (vs).is_empty() {
                        vs_1 = vs.clone();
                        il = (var_field!((*inValue).dimLst, Values::Value::ARRAY)).rest()?;
                        il = listAppend(List::fill(0, n.clone() - ((il).len() as i32)), il.clone());
                    } else {
                        let (__pa1, __pa0) = ::match_deref::match_deref! { match &(List::map1(vs.clone(), &cevalBuiltinPromote2, n_1)?) {
                            __pa1 @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { dimLst: __pa0, .. }, tail: _ } => (__pa1.clone(), __pa0.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        il = metamodelica::Own::own(__pa0);
                        vs_1 = metamodelica::Own::own(__pa1);
                    }
                    Ok(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: vs_1.clone(), dimLst: metamodelica::cons(i.clone(), il.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, n) => {
                    let mut n_1: i32;
                    let mut il: metamodelica::List<i32>;
                    let mut v = (*v).clone();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(v.clone()) {
                            Deref @ Values::Value::ARRAY { .. } => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    n_1 = n.clone() - 1;
                    let (__pa2, __pa1) = ::match_deref::match_deref! { match &(cevalBuiltinPromote2(v.clone(), n_1)?) {
                        __pa2 @ Deref @ Values::Value::ARRAY { dimLst: __pa1, .. } => (__pa2.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    il = metamodelica::Own::own(__pa1);
                    v = metamodelica::Own::own(__pa2);
                    Ok(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: list![v.clone()], dimLst: metamodelica::cons(1, il.clone()) }))
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
                    Debug::trace(literal!("- Ceval.cevalBuiltinPromote2 failed\n"))?;
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

fn cevalBuiltinSubstring(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: str_exp, tail: Deref @ metamodelica::ListNode::Cons { head: start_exp, tail: Deref @ metamodelica::ListNode::Cons { head: stop_exp, tail: Deref @ metamodelica::ListNode::Nil } } } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut r#str: ArcStr;
            let mut start: i32;
            let mut stop: i32;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), str_exp.clone(), r#impl, msg.clone(), numIter + 1)?) {
                (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            r#str = metamodelica::Own::own(__pa1);
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), start_exp.clone(), r#impl, msg.clone(), numIter + 1)?) {
                (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa3);
            start = metamodelica::Own::own(__pa4);
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(ceval(cache, env, stop_exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa6, Deref @ Values::Value::INTEGER { integer: __pa7 }) => (__pa6.clone(), __pa7.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa6);
            stop = metamodelica::Own::own(__pa7);
            r#str = substring(r#str, start, stop)?;
            (cache, metamodelica::Ref::new(Values::Value::STRING { string: r#str }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinString(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Cons { head: len_exp, tail: Deref @ metamodelica::ListNode::Cons { head: justified_exp, tail: Deref @ metamodelica::ListNode::Nil } } } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut r#str: ArcStr;
            let mut i: i32;
            let mut b: bool;
            let mut p: metamodelica::Ref<Absyn::Path>;
            let mut v: metamodelica::Ref<Values::Value>;
            (cache, v) = ceval(cache, env.clone(), exp.clone(), r#impl, msg.clone(), numIter + 1)?;
            r#str = (match &*v {
        Values::Value::INTEGER { integer: __esc_i } => {
            i = (*__esc_i).clone();
            intString(i.clone())
        },
        Values::Value::BOOL { boolean: __esc_b } => {
            b = (*__esc_b).clone();
            boolString(b.clone())
        },
        Values::Value::ENUM_LITERAL { name: __esc_p, .. } => {
            p = (*__esc_p).clone();
            AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&p))
        },
        _ => return Err("match: no arm matched"),
    });
            (cache, r#str) = cevalBuiltinStringFormat(cache, env, r#str, len_exp.clone(), justified_exp.clone(), r#impl, msg, numIter + 1)?;
            (cache, metamodelica::Ref::new(Values::Value::STRING { string: r#str }))
        },
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Cons { head: sig_dig, tail: Deref @ metamodelica::ListNode::Cons { head: len_exp, tail: Deref @ metamodelica::ListNode::Cons { head: justified_exp, tail: Deref @ metamodelica::ListNode::Nil } } } } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut r#str: ArcStr;
            let mut format: ArcStr;
            let mut len: i32;
            let mut sig: i32;
            let mut r: metamodelica::Real;
            let mut left_just: bool;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), exp.clone(), r#impl, msg.clone(), numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            r = metamodelica::Own::own(__pa1);
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), len_exp.clone(), r#impl, msg.clone(), numIter + 1)?) {
                (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa3);
            len = metamodelica::Own::own(__pa4);
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), justified_exp.clone(), r#impl, msg.clone(), numIter + 1)?) {
                (__pa6, Deref @ Values::Value::BOOL { boolean: __pa7 }) => (__pa6.clone(), __pa7.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa6);
            left_just = metamodelica::Own::own(__pa7);
            let (__pa9, __pa10) = ::match_deref::match_deref! { match &(ceval(cache, env, sig_dig.clone(), r#impl, msg, numIter + 1)?) {
                (__pa9, Deref @ Values::Value::INTEGER { integer: __pa10 }) => (__pa9.clone(), __pa10.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa9);
            sig = metamodelica::Own::own(__pa10);
            format = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("%")); __mm_s.push_str(&*if (left_just) {literal!("-")} else {literal!("")}); __mm_s.push_str(&*intString(len)); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*intString(sig)); __mm_s.push_str(&*literal!("g")); ArcStr::from(__mm_s) };
            r#str = System::snprintff(format, len + 20, r)?;
            (cache, metamodelica::Ref::new(Values::Value::STRING { string: r#str }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinStringFormat(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inString: ArcStr,
    mut lengthExp: metamodelica::Ref<DAE::Exp>,
    mut justifiedExp: metamodelica::Ref<DAE::Exp>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, ArcStr)> {
    let mut outCache: FCore::Cache;
    let mut outString: ArcStr;
    (outCache, outString) = (match inCache {
        mut cache => {
            let mut min_length: i32;
            let mut left_justified: bool;
            let mut r#str: ArcStr;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, inEnv.clone(), lengthExp, inBoolean, inMsg.clone(), numIter + 1)?) {
                (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            min_length = metamodelica::Own::own(__pa1);
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache, inEnv, justifiedExp, inBoolean, inMsg, numIter + 1)?) {
                (__pa3, Deref @ Values::Value::BOOL { boolean: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa3);
            left_justified = metamodelica::Own::own(__pa4);
            r#str = ExpressionSimplify::cevalBuiltinStringFormat(
                inString.clone(),
                ((inString).len() as i32),
                min_length,
                left_justified,
            );
            (cache, r#str)
        }
    });
    Ok((outCache, outString))
}

fn cevalBuiltinPrint(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut r#str: ArcStr;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            r#str = metamodelica::Own::own(__pa1);
            metamodelica::print(r#str);
            (cache, openmodelica_frontend_types::Values::Value::interned_NORETCALL())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalIntString(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut r#str: ArcStr;
            let mut i: i32;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            i = metamodelica::Own::own(__pa1);
            r#str = intString(i);
            (cache, metamodelica::Ref::new(Values::Value::STRING { string: r#str }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalRealString(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut r#str: ArcStr;
            let mut r: metamodelica::Real;
            let mut v: metamodelica::Ref<Values::Value>;
            (cache, v) = ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?;
            let __pa0 = ::match_deref::match_deref! { match &(v) {
                Deref @ Values::Value::REAL { real: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            r = metamodelica::Own::own(__pa0);
            r#str = realString(r);
            (cache, metamodelica::Ref::new(Values::Value::STRING { string: r#str }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalStringCharInt(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut r#str: ArcStr;
            let mut i: i32;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            r#str = metamodelica::Own::own(__pa1);
            i = stringCharInt(r#str)?;
            (cache, metamodelica::Ref::new(Values::Value::INTEGER { integer: i }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalIntStringChar(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut r#str: ArcStr;
            let mut i: i32;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            i = metamodelica::Own::own(__pa1);
            r#str = intStringChar(i);
            (cache, metamodelica::Ref::new(Values::Value::STRING { string: r#str }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalStringInt(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut r#str: ArcStr;
            let mut i: i32;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            r#str = metamodelica::Own::own(__pa1);
            i = stringInt(r#str)?;
            (cache, metamodelica::Ref::new(Values::Value::INTEGER { integer: i }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalStringLength(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut r#str: ArcStr;
            let mut i: i32;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            r#str = metamodelica::Own::own(__pa1);
            i = ((r#str).len() as i32);
            (cache, metamodelica::Ref::new(Values::Value::INTEGER { integer: i }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalStringListStringChar(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut r#str: ArcStr;
            let mut chList: metamodelica::List<ArcStr>;
            let mut valList: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            r#str = metamodelica::Own::own(__pa1);
            chList = stringListStringChar(r#str);
            valList = List::map(chList, &fnptr!(generateValueString, ArcStr))?;
            (cache, metamodelica::Ref::new(Values::Value::LIST { valueLst: valList }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn generateValueString(mut r#str: ArcStr) -> metamodelica::Ref<Values::Value> {
    let mut val: metamodelica::Ref<Values::Value>;
    val = metamodelica::Ref::new(Values::Value::STRING { string: r#str });
    val
}

fn cevalListStringCharString(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut r#str: ArcStr;
            let mut chList: metamodelica::List<ArcStr>;
            let mut valList: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            valList = metamodelica::Own::own(__pa1);
            chList = List::map(valList, &move |__a0: metamodelica::Ref<Values::Value>| extractValueStringChar(&__a0))?;
            r#str = stringAppendList(chList);
            (cache, metamodelica::Ref::new(Values::Value::STRING { string: r#str }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalStringAppendList(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut r#str: ArcStr;
            let mut chList: metamodelica::List<ArcStr>;
            let mut valList: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            valList = metamodelica::Own::own(__pa1);
            chList = List::map(valList, &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::extractValueString(&__a0))?;
            r#str = stringAppendList(chList);
            (cache, metamodelica::Ref::new(Values::Value::STRING { string: r#str }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalStringDelimitList(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut r#str: ArcStr;
            let mut chList: metamodelica::List<ArcStr>;
            let mut valList: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), exp1.clone(), r#impl, msg.clone(), numIter + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            valList = metamodelica::Own::own(__pa1);
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache, env, exp2.clone(), r#impl, msg, numIter + 1)?) {
                (__pa3, Deref @ Values::Value::STRING { string: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa3);
            r#str = metamodelica::Own::own(__pa4);
            chList = List::map(valList, &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::extractValueString(&__a0))?;
            r#str = stringDelimitList(chList, r#str);
            (cache, metamodelica::Ref::new(Values::Value::STRING { string: r#str }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalListLength(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut i: i32;
            let mut valList: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            valList = metamodelica::Own::own(__pa1);
            i = ((valList).len() as i32);
            (cache, metamodelica::Ref::new(Values::Value::INTEGER { integer: i }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalListAppend(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut valList: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut valList1: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut valList2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), exp1.clone(), r#impl, msg.clone(), numIter + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            valList1 = metamodelica::Own::own(__pa1);
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache, env, exp2.clone(), r#impl, msg, numIter + 1)?) {
                (__pa3, Deref @ Values::Value::LIST { valueLst: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa3);
            valList2 = metamodelica::Own::own(__pa4);
            valList = listAppend(valList1, valList2);
            (cache, metamodelica::Ref::new(Values::Value::LIST { valueLst: valList }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalListReverse(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut valList: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut valList1: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp1.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            valList1 = metamodelica::Own::own(__pa1);
            valList = valList1.reverse();
            (cache, metamodelica::Ref::new(Values::Value::LIST { valueLst: valList }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalListRest(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut valList1: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp1.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa1 } }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            valList1 = metamodelica::Own::own(__pa1);
            (cache, metamodelica::Ref::new(Values::Value::LIST { valueLst: valList1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalListMember(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut val: metamodelica::Ref<Values::Value>;
            let mut b: bool;
            (cache, val) = ceval(cache, env.clone(), exp1.clone(), r#impl, msg.clone(), numIter + 1)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp2.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            vals = metamodelica::Own::own(__pa1);
            b = listMember(val, vals);
            (cache, metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalListArrayLiteral(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            vals = metamodelica::Own::own(__pa1);
            (cache, metamodelica::Ref::new(Values::Value::META_ARRAY { valueLst: vals }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalAnyString(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut s: ArcStr;
            (cache, v) = ceval(cache, env, exp1.clone(), r#impl, msg, numIter + 1)?;
            s = ValuesDump::valString(&v)?;
            (cache, metamodelica::Ref::new(Values::Value::STRING { string: s }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalNumBits(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: &Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Nil => {
            let mut i: i32;
            i = System::numBits();
            (inCache, metamodelica::Ref::new(Values::Value::INTEGER { integer: i }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalIntegerMax(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: &Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Nil => {
            let mut i: i32;
            i = System::intMaxLit();
            (inCache, metamodelica::Ref::new(Values::Value::INTEGER { integer: i }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalIntBitAnd(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut r#impl: bool,
    mut msg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut result: metamodelica::Ref<Values::Value>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut i1: i32;
    let mut i2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    e2 = metamodelica::Own::own(__pa1);
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), e1, r#impl, msg.clone(), numIter + 1)?) {
        (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa3);
    i1 = metamodelica::Own::own(__pa4);
    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(ceval(cache, env, e2, r#impl, msg, numIter + 1)?) {
        (__pa6, Deref @ Values::Value::INTEGER { integer: __pa7 }) => (__pa6.clone(), __pa7.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa6);
    i2 = metamodelica::Own::own(__pa7);
    result = metamodelica::Ref::new(Values::Value::INTEGER {
        integer: intBitAnd(i1, i2),
    });
    Ok((cache, result))
}

fn cevalIntBitOr(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut r#impl: bool,
    mut msg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut result: metamodelica::Ref<Values::Value>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut i1: i32;
    let mut i2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    e2 = metamodelica::Own::own(__pa1);
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), e1, r#impl, msg.clone(), numIter + 1)?) {
        (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa3);
    i1 = metamodelica::Own::own(__pa4);
    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(ceval(cache, env, e2, r#impl, msg, numIter + 1)?) {
        (__pa6, Deref @ Values::Value::INTEGER { integer: __pa7 }) => (__pa6.clone(), __pa7.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa6);
    i2 = metamodelica::Own::own(__pa7);
    result = metamodelica::Ref::new(Values::Value::INTEGER {
        integer: intBitOr(i1, i2),
    });
    Ok((cache, result))
}

fn cevalIntBitXor(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut r#impl: bool,
    mut msg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut result: metamodelica::Ref<Values::Value>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut i1: i32;
    let mut i2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    e2 = metamodelica::Own::own(__pa1);
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), e1, r#impl, msg.clone(), numIter + 1)?) {
        (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa3);
    i1 = metamodelica::Own::own(__pa4);
    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(ceval(cache, env, e2, r#impl, msg, numIter + 1)?) {
        (__pa6, Deref @ Values::Value::INTEGER { integer: __pa7 }) => (__pa6.clone(), __pa7.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa6);
    i2 = metamodelica::Own::own(__pa7);
    result = metamodelica::Ref::new(Values::Value::INTEGER {
        integer: intBitXor(i1, i2),
    });
    Ok((cache, result))
}

fn cevalIntBitLShift(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut r#impl: bool,
    mut msg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut result: metamodelica::Ref<Values::Value>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut i: i32;
    let mut s: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    e2 = metamodelica::Own::own(__pa1);
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), e1, r#impl, msg.clone(), numIter + 1)?) {
        (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa3);
    i = metamodelica::Own::own(__pa4);
    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(ceval(cache, env, e2, r#impl, msg, numIter + 1)?) {
        (__pa6, Deref @ Values::Value::INTEGER { integer: __pa7 }) => (__pa6.clone(), __pa7.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa6);
    s = metamodelica::Own::own(__pa7);
    result = metamodelica::Ref::new(Values::Value::INTEGER {
        integer: intBitLShift(i, s),
    });
    Ok((cache, result))
}

fn cevalIntBitRShift(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut r#impl: bool,
    mut msg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut result: metamodelica::Ref<Values::Value>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut i: i32;
    let mut s: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    e2 = metamodelica::Own::own(__pa1);
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), e1, r#impl, msg.clone(), numIter + 1)?) {
        (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa3);
    i = metamodelica::Own::own(__pa4);
    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(ceval(cache, env, e2, r#impl, msg, numIter + 1)?) {
        (__pa6, Deref @ Values::Value::INTEGER { integer: __pa7 }) => (__pa6.clone(), __pa7.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa6);
    s = metamodelica::Own::own(__pa7);
    result = metamodelica::Ref::new(Values::Value::INTEGER {
        integer: intBitRShift(i, s),
    });
    Ok((cache, result))
}

fn makeLoadLibrariesEntry(
    mut cl: &metamodelica::Ref<SCode::Element>,
    mut acc: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut out: metamodelica::List<metamodelica::Ref<Values::Value>>;
    out = (::match_deref::match_deref! { match cl {
        Deref @ SCode::Element::CLASS { info: SourceInfo { fileName: Deref @ "<interactive>", .. }, .. } => {
            acc
        },
        Deref @ SCode::Element::CLASS { name, info: SourceInfo { fileName, .. }, .. } => {
            let mut dir: ArcStr;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut b: bool;
            let mut fileName = (*fileName).clone();
            dir = System::dirname(fileName.clone());
            fileName = System::basename(fileName.clone());
            v = ValuesMake::makeArray(list![metamodelica::Ref::new(Values::Value::STRING { string: name.clone() }), metamodelica::Ref::new(Values::Value::STRING { string: dir.clone() })]);
            b = stringEq(&fileName, &(literal!("ModelicaBuiltin.mo"))) || stringEq(&fileName, &(literal!("MetaModelicaBuiltin.mo"))) || stringEq(&dir, &(literal!(".")));
            List::consOnTrue(!(b), v, acc)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(out)
}

fn cevalListFirst(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut v: metamodelica::Ref<Values::Value>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp1.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ } }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            v = metamodelica::Own::own(__pa1);
            (cache, ValuesUtil::boxIfUnboxedVal(v))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn extractValueStringChar(mut val: &metamodelica::Ref<Values::Value>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**val {
        Values::Value::STRING { string: __esc_str } => {
            r#str = (*__esc_str).clone();
            let 1 = ((r#str).len() as i32) else {
                return Err("pattern mismatch");
            };
            r#str.clone()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(r#str)
}

fn cevalCat(
    mut v_lst: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut dim: i32,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut v_lst_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
    v_lst_1 = catDimension(v_lst, dim)?;
    outValue = ValuesMake::makeArray(v_lst_1);
    Ok(outValue)
}

fn catDimension(
    mut inValuesValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inInteger: i32,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut outValuesValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
    outValuesValueLst = 'mc: {
        let __mc_input = (inValuesValueLst, inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vlst, 1) => {
                    let mut vlst_lst: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>;
                    let mut v_lst_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    vlst_lst = List::map(vlst.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::arrayValues(&__a0))?;
                    v_lst_1 = List::flatten(vlst_lst.clone())?;
                    Ok(v_lst_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vlst, dim) => {
                    let mut v_lst_lst: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>;
                    let mut v_lst_lst_1: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>;
                    let mut v_lst_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut dim_1: i32;
                    let mut i1: i32;
                    let mut i2: i32;
                    let mut il: metamodelica::List<i32>;
                    v_lst_lst = List::map(vlst.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::arrayValues(&__a0))?;
                    dim_1 = dim.clone() - 1;
                    v_lst_lst_1 = catDimension2(v_lst_lst.clone(), dim_1)?;
                    v_lst_1 = List::map(v_lst_lst_1.clone(), &fnptr!(ValuesMake::makeArray, metamodelica::List<metamodelica::Ref<Values::Value>>))?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(v_lst_1.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { dimLst: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 }, .. }, tail: _ } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    i2 = metamodelica::Own::own(__pa0);
                    il = metamodelica::Own::own(__pa1);
                    i1 = ((v_lst_1).len() as i32);
                    v_lst_1 = cevalBuiltinTranspose2(v_lst_1.clone(), 1, &(metamodelica::cons(i2, metamodelica::cons(i1, il.clone()))));
                    Ok(v_lst_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outValuesValueLst)
}

fn catDimension2(
    mut inValuesValueLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>,
    mut inInteger: i32,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>> {
    let mut outValuesValueLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>;
    outValuesValueLstLst = 'mc: {
        let __mc_input = (inValuesValueLstLst, inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lst, dim) => {
                    let mut l_lst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut first_lst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut first_lst_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut first_lst_2: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>;
                    l_lst = (lst).head().cloned()?;
                    let 1 = (((l_lst).len() as i32)) else { return Err("pattern mismatch") };
                    first_lst = List::map(lst.clone(), &listHead)?;
                    first_lst_1 = catDimension(first_lst.clone(), dim.clone())?;
                    first_lst_2 = List::map(first_lst_1.clone(), &fnptr!(List::create, _))?;
                    Ok(first_lst_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lst, dim) => {
                    let mut first_lst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut first_lst_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut rest: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>;
                    let mut rest_1: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>;
                    let mut res: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>;
                    first_lst = List::map(lst.clone(), &listHead)?;
                    rest = List::map(lst.clone(), &listRest)?;
                    first_lst_1 = catDimension(first_lst.clone(), dim.clone())?;
                    rest_1 = catDimension2(rest.clone(), dim.clone())?;
                    res = List::threadMap(rest_1.clone(), first_lst_1.clone(), &fnptr!(List::consr, _, _))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outValuesValueLstLst)
}

fn cevalBuiltinFloor(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            rv_1 = (rv).floor();
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinCeil(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let mut rvt: metamodelica::Real;
            let mut realRet: metamodelica::Real;
            let mut ri: i32;
            let mut ri_1: i32;
            let mut v: metamodelica::Ref<Values::Value>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            rv_1 = (rv).floor();
            ri = ((rv_1).0.floor() as i32);
            rvt = intReal(ri);
            ri_1 = ri + 1;
            realRet = intReal(ri_1);
            v = if (rvt == rv) {metamodelica::Ref::new(Values::Value::REAL { real: rvt })} else {metamodelica::Ref::new(Values::Value::REAL { real: realRet })};
            (cache, v)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinSqrt(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let mut info: SourceInfo;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg.clone(), numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            if rv < metamodelica::OrderedFloat(0.0_f64) {
                let Absyn::MSG { info: __pa3 } = (msg) else { return Err("pattern mismatch") };
                info = metamodelica::Own::own(__pa3);
                Error::addSourceMessage(&(Error::NEGATIVE_SQRT.clone()), metamodelica::nil(), &info)?;
                return Err("fail");
            } else {
                rv_1 = (rv).sqrt();
            }
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinNthRoot(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut r#impl: bool,
    mut msg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut v_exp: metamodelica::Ref<DAE::Exp>;
    let mut n_exp: metamodelica::Ref<DAE::Exp>;
    let mut v: metamodelica::Real;
    let mut n: i32;
    let mut info: SourceInfo;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    v_exp = metamodelica::Own::own(__pa0);
    n_exp = metamodelica::Own::own(__pa1);
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), v_exp, r#impl, msg.clone(), numIter + 1)?) {
        (__pa3, Deref @ Values::Value::REAL { real: __pa4 }) => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa3);
    v = metamodelica::Own::own(__pa4);
    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(ceval(cache, env, n_exp, r#impl, msg.clone(), numIter + 1)?) {
        (__pa6, Deref @ Values::Value::INTEGER { integer: __pa7 }) => (__pa6.clone(), __pa7.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa6);
    n = metamodelica::Own::own(__pa7);
    if n <= 0 {
        let Absyn::MSG { info: __pa9 } = (msg.clone()) else {
            return Err("pattern mismatch");
        };
        info = metamodelica::Own::own(__pa9);
        Error::addSourceMessage(
            &(Error::NON_POSITIVE_NTH_ROOT.clone()),
            list![
                ArcStr::from(::std::format!("{}", v)),
                ArcStr::from(::std::format!("{}", n))
            ],
            &info,
        )?;
        return Err("fail");
    }
    if intMod(n, 2) == 0 && v < metamodelica::OrderedFloat((0) as f64) {
        let Absyn::MSG { info: __pa10 } = (msg) else {
            return Err("pattern mismatch");
        };
        info = metamodelica::Own::own(__pa10);
        Error::addSourceMessage(
            &(Error::NEGATIVE_NTH_ROOT.clone()),
            list![
                ArcStr::from(::std::format!("{}", v)),
                ArcStr::from(::std::format!("{}", n))
            ],
            &info,
        )?;
        return Err("fail");
    }
    outValue = metamodelica::Ref::new(Values::Value::REAL {
        real: (v).powf(
            (metamodelica::real_div_checked(
                metamodelica::OrderedFloat((1) as f64),
                metamodelica::OrderedFloat((n) as f64),
            )?),
        ),
    });
    Ok((cache, outValue))
}

fn cevalBuiltinSin(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            rv_1 = (rv).sin();
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinSinh(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            rv_1 = (rv).sinh();
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinCos(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            rv_1 = (rv).cos();
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinCosh(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            rv_1 = (rv).cosh();
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinLog(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            let true = (rv > metamodelica::OrderedFloat((0) as f64)) else { return Err("pattern mismatch") };
            rv_1 = (rv).ln();
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinLog10(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            let true = (rv > metamodelica::OrderedFloat((0) as f64)) else { return Err("pattern mismatch") };
            rv_1 = (rv).log10();
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinTan(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut r#impl: bool,
    mut msg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            rv_1 = (rv).tan();
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinTanh(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            rv_1 = (rv).tanh();
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinAsin(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            let true = (rv >= metamodelica::OrderedFloat(-1.0_f64) && rv <= metamodelica::OrderedFloat(1.0_f64)) else { return Err("pattern mismatch") };
            rv_1 = (rv).asin();
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinAcos(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            let true = (rv >= metamodelica::OrderedFloat(-1.0_f64) && rv <= metamodelica::OrderedFloat(1.0_f64)) else { return Err("pattern mismatch") };
            rv_1 = (rv).acos();
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinAtan(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            rv_1 = (rv).atan();
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv_1 }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinAtan2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut rv_1: metamodelica::Real;
            let mut rv_2: metamodelica::Real;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), exp1.clone(), r#impl, msg.clone(), numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv_1 = metamodelica::Own::own(__pa1);
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache, env, exp2.clone(), r#impl, msg, numIter + 1)?) {
                (__pa3, Deref @ Values::Value::REAL { real: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa3);
            rv_2 = metamodelica::Own::own(__pa4);
            rv = (rv_1).atan2(rv_2);
            (cache, metamodelica::Ref::new(Values::Value::REAL { real: rv }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinDiv(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inExpExpLst, inBoolean, inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, msg) => {
                    let mut rv1: metamodelica::Real;
                    let mut rv2: metamodelica::Real;
                    let mut rv_1: metamodelica::Real;
                    let mut rv_2: metamodelica::Real;
                    let mut b: bool;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    rv1 = metamodelica::Own::own(__pa1);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa3, Deref @ Values::Value::REAL { real: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    rv2 = metamodelica::Own::own(__pa4);
                    rv_1 = metamodelica::real_div_checked(rv1, rv2)?;
                    b = rv_1 < metamodelica::OrderedFloat(0.0_f64);
                    rv_2 = if (b) {(rv_1).ceil()} else {(rv_1).floor()};
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::REAL { real: rv_2 })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, msg) => {
                    let mut rv1: metamodelica::Real;
                    let mut rv2: metamodelica::Real;
                    let mut rv_1: metamodelica::Real;
                    let mut rv_2: metamodelica::Real;
                    let mut ri: i32;
                    let mut b: bool;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    ri = metamodelica::Own::own(__pa1);
                    rv1 = intReal(ri);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa3, Deref @ Values::Value::REAL { real: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    rv2 = metamodelica::Own::own(__pa4);
                    Error::addInternalError(literal!("cevalBuiltinDiv got Integer and Real (type error)\n"), metamodelica::sourceInfo!("FrontEnd/Ceval.mo"))?;
                    rv_1 = metamodelica::real_div_checked(rv1, rv2)?;
                    b = rv_1 < metamodelica::OrderedFloat(0.0_f64);
                    rv_2 = if (b) {(rv_1).ceil()} else {(rv_1).floor()};
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::REAL { real: rv_2 })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, msg) => {
                    let mut rv1: metamodelica::Real;
                    let mut rv2: metamodelica::Real;
                    let mut rv_1: metamodelica::Real;
                    let mut rv_2: metamodelica::Real;
                    let mut ri: i32;
                    let mut b: bool;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    rv1 = metamodelica::Own::own(__pa1);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    ri = metamodelica::Own::own(__pa4);
                    Error::addInternalError(literal!("cevalBuiltinDiv got Real and Integer (type error)\n"), metamodelica::sourceInfo!("FrontEnd/Ceval.mo"))?;
                    rv2 = intReal(ri);
                    rv_1 = metamodelica::real_div_checked(rv1, rv2)?;
                    b = rv_1 < metamodelica::OrderedFloat(0.0_f64);
                    rv_2 = if (b) {(rv_1).ceil()} else {(rv_1).floor()};
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::REAL { real: rv_2 })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, msg) => {
                    let mut ri_1: i32;
                    let mut ri1: i32;
                    let mut ri2: i32;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    ri1 = metamodelica::Own::own(__pa1);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    ri2 = metamodelica::Own::own(__pa4);
                    ri_1 = intDiv(ri1, ri2);
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::INTEGER { integer: ri_1 })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, Absyn::Msg::MSG { info }) => {
                    let mut rv2: metamodelica::Real;
                    let mut exp1_str: ArcStr;
                    let mut exp2_str: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), inMsg.clone(), numIter + 1)?) {
                        (_, Deref @ Values::Value::REAL { real: __pa0 }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    rv2 = metamodelica::Own::own(__pa0);
                    let true = (rv2 == metamodelica::OrderedFloat(0.0_f64)) else { return Err("pattern mismatch") };
                    exp1_str = ExpressionBasics::printExpStr(exp1.clone())?;
                    exp2_str = ExpressionBasics::printExpStr(exp2.clone())?;
                    Error::addSourceMessage(&(Error::DIVISION_BY_ZERO.clone()), list![exp1_str.clone(), exp2_str.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, Absyn::Msg::NO_MSG { .. }) => {
                    let mut rv2: metamodelica::Real;
                    let __pa0 = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), openmodelica_ast::Absyn::Msg::NO_MSG, numIter + 1)?) {
                        (_, Deref @ Values::Value::REAL { real: __pa0 }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    rv2 = metamodelica::Own::own(__pa0);
                    let true = (rv2 == metamodelica::OrderedFloat(0.0_f64)) else { return Err("pattern mismatch") };
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, Absyn::Msg::MSG { info }) => {
                    let mut ri2: i32;
                    let mut lh_str: ArcStr;
                    let mut rh_str: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), inMsg.clone(), numIter + 1)?) {
                        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ri2 = metamodelica::Own::own(__pa0);
                    let true = (ri2 == 0) else { return Err("pattern mismatch") };
                    lh_str = ExpressionBasics::printExpStr(exp1.clone())?;
                    rh_str = ExpressionBasics::printExpStr(exp2.clone())?;
                    Error::addSourceMessage(&(Error::DIVISION_BY_ZERO.clone()), list![lh_str.clone(), rh_str.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, Absyn::Msg::NO_MSG { .. }) => {
                    let mut ri2: i32;
                    let __pa0 = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), openmodelica_ast::Absyn::Msg::NO_MSG, numIter + 1)?) {
                        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ri2 = metamodelica::Own::own(__pa0);
                    let true = (ri2 == 0) else { return Err("pattern mismatch") };
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

fn cevalBuiltinMod(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut r#impl: bool,
    mut msg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut cache: FCore::Cache = inCache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut v1: metamodelica::Ref<Values::Value>;
    let mut v2: metamodelica::Ref<Values::Value>;
    let mut exp1: metamodelica::Ref<DAE::Exp>;
    let mut exp2: metamodelica::Ref<DAE::Exp>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inExpExpLst)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp1 = metamodelica::Own::own(__pa0);
    exp2 = metamodelica::Own::own(__pa1);
    (cache, v1) = ceval(cache, inEnv.clone(), exp1.clone(), r#impl, msg.clone(), numIter + 1)?;
    (cache, v2) = ceval(cache, inEnv, exp2.clone(), r#impl, msg.clone(), numIter + 1)?;
    outValue = (::match_deref::match_deref! { match &((v1, v2, msg)) {
        (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::REAL { real: rv2 }, _) => {
            metamodelica::Ref::new(Values::Value::REAL { real: realMod(rv1.clone(), rv2.clone()) })
        },
        (Deref @ Values::Value::INTEGER { integer: ri }, Deref @ Values::Value::REAL { real: rv2 }, _) => {
            metamodelica::Ref::new(Values::Value::REAL { real: realMod(metamodelica::OrderedFloat((ri.clone()) as f64), rv2.clone()) })
        },
        (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::INTEGER { integer: ri }, _) => {
            metamodelica::Ref::new(Values::Value::REAL { real: realMod(rv1.clone(), metamodelica::OrderedFloat((ri.clone()) as f64)) })
        },
        (Deref @ Values::Value::INTEGER { integer: ri1 }, Deref @ Values::Value::INTEGER { integer: ri2 }, _) => {
            metamodelica::Ref::new(Values::Value::INTEGER { integer: intMod(ri1.clone(), ri2.clone()) })
        },
        (_, Deref @ Values::Value::REAL { real: rv2 }, Absyn::Msg::MSG { info }) if (rv2.clone() == metamodelica::OrderedFloat(0.0_f64)) => {
            let mut lhs_str: ArcStr;
            let mut rhs_str: ArcStr;
            lhs_str = ExpressionBasics::printExpStr(exp1)?;
            rhs_str = ExpressionBasics::printExpStr(exp2)?;
            Error::addSourceMessage(&(Error::MODULO_BY_ZERO.clone()), list![lhs_str, rhs_str], metamodelica::AsArg::as_arg(&info))?;
            return Err("fail")
        },
        (_, Deref @ Values::Value::INTEGER { integer: 0 }, Absyn::Msg::MSG { info }) => {
            let mut lhs_str: ArcStr;
            let mut rhs_str: ArcStr;
            lhs_str = ExpressionBasics::printExpStr(exp1)?;
            rhs_str = ExpressionBasics::printExpStr(exp2)?;
            Error::addSourceMessage(&(Error::MODULO_BY_ZERO.clone()), list![lhs_str, rhs_str], metamodelica::AsArg::as_arg(&info))?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((cache, outValue))
}

fn cevalBuiltinSum(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: arr, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, arr.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            vals = metamodelica::Own::own(__pa1);
            if Types::isInteger(&(Expression::r#typeof(Expression::unboxExp(metamodelica::AsArg::as_arg(&arr)))?)) {
                if (vals).is_empty() {
                    v = metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 });
                } else {
                    let __pa3 = ::match_deref::match_deref! { match &(ValuesUtil::sumArrayelt(&vals)?) {
                        __pa3 @ Deref @ Values::Value::INTEGER { .. } => __pa3.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    v = metamodelica::Own::own(__pa3);
                }
            } else {
                if (vals).is_empty() {
                    v = metamodelica::Ref::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) });
                } else {
                    let __pa4 = ::match_deref::match_deref! { match &(ValuesUtil::sumArrayelt(&vals)?) {
                        __pa4 @ Deref @ Values::Value::REAL { .. } => __pa4.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    v = metamodelica::Own::own(__pa4);
                }
            }
            (cache, v)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinMax(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: arr, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut v_1: metamodelica::Ref<Values::Value>;
            (cache, v) = ceval(cache, env, arr.clone(), r#impl, msg, numIter + 1)?;
            v_1 = cevalBuiltinMaxArr(&v)?;
            (cache, v_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: s1, tail: Deref @ metamodelica::ListNode::Cons { head: s2, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut v1: metamodelica::Ref<Values::Value>;
            let mut v2: metamodelica::Ref<Values::Value>;
            (cache, v1) = ceval(cache, env.clone(), s1.clone(), r#impl, msg.clone(), numIter + 1)?;
            (cache, v2) = ceval(cache, env, s2.clone(), r#impl, msg, numIter + 1)?;
            v = cevalBuiltinMax2(v1, v2)?;
            (cache, v)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinMax2(
    mut v1: metamodelica::Ref<Values::Value>,
    mut v2: metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = (::match_deref::match_deref! { match &((v1.clone(), v2.clone())) {
        (Deref @ Values::Value::INTEGER { integer: i1 }, Deref @ Values::Value::INTEGER { integer: i2 }) => {
            metamodelica::Ref::new(Values::Value::INTEGER { integer: std::cmp::max(i1.clone(), i2.clone()) })
        },
        (Deref @ Values::Value::REAL { real: r1 }, Deref @ Values::Value::REAL { real: r2 }) => {
            metamodelica::Ref::new(Values::Value::REAL { real: std::cmp::max(r1.clone(), r2.clone()) })
        },
        (Deref @ Values::Value::BOOL { boolean: b1 }, Deref @ Values::Value::BOOL { boolean: b2 }) => {
            metamodelica::Ref::new(Values::Value::BOOL { boolean: b1.clone() || b2.clone() })
        },
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => {
            if (var_field!((*v1).index, Values::Value::ENUM_LITERAL).clone() > var_field!((*v2).index, Values::Value::ENUM_LITERAL).clone()) {v1} else {v2}
        },
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            s1 = ValuesDump::valString(&v1)?;
            s2 = ValuesDump::valString(&v2)?;
            Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Ceval.cevalBuiltinMin2 failed: min(")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

fn cevalBuiltinMaxArr(mut inValue: &metamodelica::Ref<Values::Value>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let __pa0 = ::match_deref::match_deref! { match &((*inValue)) {
        Deref @ Values::Value::ARRAY { valueLst: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    vals = metamodelica::Own::own(__pa0);
    outValue = ({
        let mut __acc: Option<_> = None;
        for mut v in (vals).into_iter().cloned() {
            let __x = v.clone();
            __acc = Some(match __acc {
                None => __x,
                Some(__cur) => cevalBuiltinMax2(__x, __cur)?,
            });
        }
        __acc.ok_or_else(|| "empty cevalBuiltinMax2 reduction")?
    });
    Ok(outValue)
}

fn cevalBuiltinMin(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: arr, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut v_1: metamodelica::Ref<Values::Value>;
            (cache, v) = ceval(cache, env, arr.clone(), r#impl, msg, numIter + 1)?;
            v_1 = cevalBuiltinMinArr(&v)?;
            (cache, v_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: s1, tail: Deref @ metamodelica::ListNode::Cons { head: s2, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut v1: metamodelica::Ref<Values::Value>;
            let mut v2: metamodelica::Ref<Values::Value>;
            (cache, v1) = ceval(cache, env.clone(), s1.clone(), r#impl, msg.clone(), numIter + 1)?;
            (cache, v2) = ceval(cache, env, s2.clone(), r#impl, msg, numIter + 1)?;
            v = cevalBuiltinMin2(v1, v2)?;
            (cache, v)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinMin2(
    mut v1: metamodelica::Ref<Values::Value>,
    mut v2: metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = (::match_deref::match_deref! { match &((v1.clone(), v2.clone())) {
        (Deref @ Values::Value::INTEGER { integer: i1 }, Deref @ Values::Value::INTEGER { integer: i2 }) => {
            metamodelica::Ref::new(Values::Value::INTEGER { integer: std::cmp::min(i1.clone(), i2.clone()) })
        },
        (Deref @ Values::Value::REAL { real: r1 }, Deref @ Values::Value::REAL { real: r2 }) => {
            metamodelica::Ref::new(Values::Value::REAL { real: std::cmp::min(r1.clone(), r2.clone()) })
        },
        (Deref @ Values::Value::BOOL { boolean: b1 }, Deref @ Values::Value::BOOL { boolean: b2 }) => {
            metamodelica::Ref::new(Values::Value::BOOL { boolean: b1.clone() && b2.clone() })
        },
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => {
            if (var_field!((*v1).index, Values::Value::ENUM_LITERAL).clone() < var_field!((*v2).index, Values::Value::ENUM_LITERAL).clone()) {v1} else {v2}
        },
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            s1 = ValuesDump::valString(&v1)?;
            s2 = ValuesDump::valString(&v2)?;
            Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Ceval.cevalBuiltinMin2 failed: min(")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

fn cevalBuiltinMinArr(mut inValue: &metamodelica::Ref<Values::Value>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let __pa0 = ::match_deref::match_deref! { match &((*inValue)) {
        Deref @ Values::Value::ARRAY { valueLst: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    vals = metamodelica::Own::own(__pa0);
    outValue = ({
        let mut __acc: Option<_> = None;
        for mut v in (vals).into_iter().cloned() {
            let __x = v.clone();
            __acc = Some(match __acc {
                None => __x,
                Some(__cur) => cevalBuiltinMin2(__x, __cur)?,
            });
        }
        __acc.ok_or_else(|| "empty cevalBuiltinMin2 reduction")?
    });
    Ok(outValue)
}

fn cevalBuiltinRem(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inExpExpLst, inBoolean, inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, msg) => {
                    let mut rv1: metamodelica::Real;
                    let mut rv2: metamodelica::Real;
                    let mut rvd: metamodelica::Real;
                    let mut dr: metamodelica::Real;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    rv1 = metamodelica::Own::own(__pa1);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa3, Deref @ Values::Value::REAL { real: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    rv2 = metamodelica::Own::own(__pa4);
                    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(cevalBuiltinDiv(cache.clone(), env.clone(), &(list![exp1.clone(), exp2.clone()]), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa6, Deref @ Values::Value::REAL { real: __pa7 }) => (__pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa6);
                    dr = metamodelica::Own::own(__pa7);
                    rvd = rv1 - rv2 * dr;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::REAL { real: rvd })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, msg) => {
                    let mut rv1: metamodelica::Real;
                    let mut rv2: metamodelica::Real;
                    let mut rvd: metamodelica::Real;
                    let mut dr: metamodelica::Real;
                    let mut ri: i32;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    ri = metamodelica::Own::own(__pa1);
                    rv1 = intReal(ri);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa3, Deref @ Values::Value::REAL { real: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    rv2 = metamodelica::Own::own(__pa4);
                    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(cevalBuiltinDiv(cache.clone(), env.clone(), &(list![exp1.clone(), exp2.clone()]), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa6, Deref @ Values::Value::REAL { real: __pa7 }) => (__pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa6);
                    dr = metamodelica::Own::own(__pa7);
                    rvd = rv1 - rv2 * dr;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::REAL { real: rvd })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, msg) => {
                    let mut rv1: metamodelica::Real;
                    let mut rv2: metamodelica::Real;
                    let mut rvd: metamodelica::Real;
                    let mut dr: metamodelica::Real;
                    let mut ri: i32;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    rv1 = metamodelica::Own::own(__pa1);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    ri = metamodelica::Own::own(__pa4);
                    rv2 = intReal(ri);
                    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(cevalBuiltinDiv(cache.clone(), env.clone(), &(list![exp1.clone(), exp2.clone()]), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa6, Deref @ Values::Value::REAL { real: __pa7 }) => (__pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa6);
                    dr = metamodelica::Own::own(__pa7);
                    rvd = rv1 - rv2 * dr;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::REAL { real: rvd })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, msg) => {
                    let mut ri1: i32;
                    let mut ri2: i32;
                    let mut ri_1: i32;
                    let mut di: i32;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    ri1 = metamodelica::Own::own(__pa1);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    ri2 = metamodelica::Own::own(__pa4);
                    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(cevalBuiltinDiv(cache.clone(), env.clone(), &(list![exp1.clone(), exp2.clone()]), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa6, Deref @ Values::Value::INTEGER { integer: __pa7 }) => (__pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa6);
                    di = metamodelica::Own::own(__pa7);
                    ri_1 = ri1 - ri2 * di;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::INTEGER { integer: ri_1 })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, Absyn::Msg::MSG { info }) => {
                    let mut rv2: metamodelica::Real;
                    let mut exp1_str: ArcStr;
                    let mut exp2_str: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), inMsg.clone(), numIter + 1)?) {
                        (_, Deref @ Values::Value::REAL { real: __pa0 }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    rv2 = metamodelica::Own::own(__pa0);
                    let true = (rv2 == metamodelica::OrderedFloat(0.0_f64)) else { return Err("pattern mismatch") };
                    exp1_str = ExpressionBasics::printExpStr(exp1.clone())?;
                    exp2_str = ExpressionBasics::printExpStr(exp2.clone())?;
                    Error::addSourceMessage(&(Error::REM_ARG_ZERO.clone()), list![exp1_str.clone(), exp2_str.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Cons { head: exp2, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, Absyn::Msg::MSG { info }) => {
                    let mut ri2: i32;
                    let mut exp1_str: ArcStr;
                    let mut exp2_str: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), inMsg.clone(), numIter + 1)?) {
                        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ri2 = metamodelica::Own::own(__pa0);
                    let true = (ri2 == 0) else { return Err("pattern mismatch") };
                    exp1_str = ExpressionBasics::printExpStr(exp1.clone())?;
                    exp2_str = ExpressionBasics::printExpStr(exp2.clone())?;
                    Error::addSourceMessage(&(Error::REM_ARG_ZERO.clone()), list![exp1_str.clone(), exp2_str.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

fn cevalBuiltinInteger(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut ri: i32;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rv = metamodelica::Own::own(__pa1);
            ri = ((rv).0.floor() as i32);
            (cache, metamodelica::Ref::new(Values::Value::INTEGER { integer: ri }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinBoolean(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut rv: metamodelica::Real;
            let mut iv: i32;
            let mut bv: bool;
            let mut b: bool;
            let mut v: metamodelica::Ref<Values::Value>;
            (cache, v) = ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?;
            b = (match &*v {
        Values::Value::REAL { real: __esc_rv } => {
            rv = (*__esc_rv).clone();
            !(realEq(rv.clone(), metamodelica::OrderedFloat(0.0_f64)))
        },
        Values::Value::INTEGER { integer: __esc_iv } => {
            iv = (*__esc_iv).clone();
            !(intEq(iv.clone(), 0))
        },
        Values::Value::BOOL { boolean: __esc_bv } => {
            bv = (*__esc_bv).clone();
            bv.clone()
        },
        _ => return Err("match: no arm matched"),
    });
            (cache, metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinRooted(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            (cache, _) = ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?;
            (cache, metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinIntegerEnumeration(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpExpLst {
        Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut ri: i32;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?) {
                (__pa0, Deref @ Values::Value::ENUM_LITERAL { index: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            ri = metamodelica::Own::own(__pa1);
            (cache, metamodelica::Ref::new(Values::Value::INTEGER { integer: ri }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinDiagonal(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inExpExpLst, inBoolean, inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil }, r#impl, msg) => {
                            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                            let mut dimension: i32;
                            let mut res: metamodelica::Ref<Values::Value>;
                            let mut zero: metamodelica::Ref<Values::Value>;
                            let mut ty: metamodelica::Ref<DAE::Type>;
                            let mut cache = (*cache).clone();
                            let __pa0 = ::match_deref::match_deref! { match &(Expression::r#typeof(exp.clone())?) {
                                Deref @ DAE::Type::T_ARRAY { ty: __pa0, .. } => __pa0.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            ty = metamodelica::Own::own(__pa0);
                            let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                                (__pa1, Deref @ Values::Value::ARRAY { valueLst: __pa2, dimLst: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } }) => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            cache = metamodelica::Own::own(__pa1);
                            vals = metamodelica::Own::own(__pa2);
                            dimension = metamodelica::Own::own(__pa3);
                            zero = ValuesMake::makeZero(&ty)?;
                            res = metamodelica::Ref::new(Values::Value::ARRAY { valueLst: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
                for mut j in (1..=dimension).into_iter() {
                            let __x = metamodelica::Ref::new(Values::Value::ARRAY { valueLst: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
                for mut i in (1..=dimension).into_iter() {
                            let __x = if (i.clone() == j.clone()) {(vals).get(i.clone())?} else {zero.clone()};
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), dimLst: list![dimension] });
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), dimLst: list![dimension, dimension] });
                            Ok((cache.clone(), res.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Absyn::Msg::MSG { info }) => {
                    Error::addSourceMessage(&(Error::COMPILER_ERROR.clone()), list![literal!("Could not evaluate diagonal. Ceval.cevalBuiltinDiagonal failed.")], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

fn cevalBuiltinCross(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache, inEnv, &*inExpExpLst, inBoolean, inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: xe, tail: Deref @ metamodelica::ListNode::Cons { head: ye, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, msg) => {
                    let mut xv: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut yv: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut res: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), xe.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: Deref @ metamodelica::ListNode::Cons { head: 3, tail: Deref @ metamodelica::ListNode::Nil } }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    xv = metamodelica::Own::own(__pa1);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), ye.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa3, Deref @ Values::Value::ARRAY { valueLst: __pa4, dimLst: Deref @ metamodelica::ListNode::Cons { head: 3, tail: Deref @ metamodelica::ListNode::Nil } }) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    yv = metamodelica::Own::own(__pa4);
                    res = ValuesUtil::crossProduct(&xv, &yv)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Absyn::Msg::MSG { info }) => {
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cross")); __mm_s.push_str(&*ExpressionBasics::printExpStr(metamodelica::Ref::new(DAE::Exp::TUPLE { PR: inExpExpLst.clone() }))?); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::FAILED_TO_EVALUATE_EXPRESSION.clone()), list![r#str.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

fn cevalBuiltinTranspose2(
    mut inValuesValueLst1: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inInteger2: i32,
    mut inDims: &metamodelica::List<i32>,
) -> metamodelica::List<metamodelica::Ref<Values::Value>> {
    let mut outValuesValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
    outValuesValueLst = 'mc: {
        let __mc_input = (inValuesValueLst1, inInteger2, &**inDims);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vlst, indx, Deref @ metamodelica::ListNode::Cons { head: dim1, tail: _ }) => {
                    if !((indx.clone() <= dim1.clone())) { return Err("guard") }
                    let mut transposed_row: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut rest: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut indx_1: i32;
                    transposed_row = List::map1(vlst.clone(), &move |__a0: metamodelica::Ref<Values::Value>, __a1: i32| ValuesUtil::nthArrayelt(&__a0, __a1), indx.clone())?;
                    indx_1 = indx.clone() + 1;
                    rest = cevalBuiltinTranspose2(vlst.clone(), indx_1, inDims);
                    Ok(metamodelica::cons(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: transposed_row.clone(), dimLst: inDims.clone() }), rest.clone()))
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
    outValuesValueLst
}

fn cevalBuiltinSizeMatrix(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache, inEnv, inExp, inBoolean, inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _, _) => {
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut sizelst: metamodelica::List<i32>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, _, tp, _, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), cr.clone())?;
                    sizelst = Types::getDimensionSizes(&tp)?;
                    v = ValuesUtil::intlistToValue(&sizelst)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::MATRIX { ty: Deref @ DAE::Type::T_ARRAY { dims, .. }, .. }, _, _) => {
                    let mut sizelst: metamodelica::List<i32>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    sizelst = List::map(dims.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| Expression::dimensionSize(&__a0))?;
                    v = ValuesUtil::intlistToValue(&sizelst)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp, r#impl, msg) => {
                    let mut sizelst: metamodelica::List<i32>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { dimLst: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    sizelst = metamodelica::Own::own(__pa1);
                    v = ValuesUtil::intlistToValue(&sizelst)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

fn cevalBuiltinFail(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inExpl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inImpl: bool,
    mut inMsg: &Absyn::Msg,
    mut numIter: i32,
) -> (FCore::Cache, metamodelica::Ref<Values::Value>) {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    outCache = inCache;
    outValue = openmodelica_frontend_types::Values::Value::interned_META_FAIL();
    (outCache, outValue)
}

fn cevalBuiltinFill(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inImpl: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inExpl {
        Deref @ metamodelica::ListNode::Cons { head: fill_exp, tail: dims } => {
            let mut cache = inCache;
            let mut fill_val: metamodelica::Ref<Values::Value>;
            (cache, fill_val) = ceval(cache, inEnv.clone(), fill_exp.clone(), inImpl, inMsg.clone(), numIter + 1)?;
            (cache, fill_val) = cevalBuiltinFill2(cache, &inEnv, &fill_val, dims, inImpl, &inMsg, numIter)?;
            (cache, fill_val)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinFill2(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inFillValue: &metamodelica::Ref<Values::Value>,
    mut inDims: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inImpl: bool,
    mut inMsg: &Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match inDims {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache;
            (cache, inFillValue.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: dim, tail: rest_dims } => {
            let mut cache = inCache;
            let mut int_dim: i32;
            let mut array_dims: metamodelica::List<i32>;
            let mut fill_value: metamodelica::Ref<Values::Value>;
            let mut fill_vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (cache, fill_value) = cevalBuiltinFill2(cache, inEnv, inFillValue, rest_dims, inImpl, inMsg, numIter)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, inEnv.clone(), dim.clone(), inImpl, inMsg.clone(), numIter + 1)?) {
                (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            int_dim = metamodelica::Own::own(__pa1);
            fill_vals = List::fill(fill_value.clone(), int_dim);
            array_dims = ValuesUtil::valueDimensions(&fill_value);
            array_dims = metamodelica::cons(int_dim, array_dims);
            (cache, metamodelica::Ref::new(Values::Value::ARRAY { valueLst: fill_vals, dimLst: array_dims }))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outValue))
}

fn cevalRelation(
    mut inValue1: &metamodelica::Ref<Values::Value>,
    mut inOperator: &DAE::Operator,
    mut inValue2: &metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut result: bool;
    result = 'mc: {
        let __mc_input = inOperator.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::GREATER { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(cevalRelationLess(inValue2, inValue1)?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::LESS { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(cevalRelationLess(inValue1, inValue2)?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::LESSEQ { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(cevalRelationLessEq(inValue1, inValue2)?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::GREATEREQ { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(cevalRelationGreaterEq(inValue1, inValue2)?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::EQUAL { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(cevalRelationEqual(inValue1, inValue2)?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::NEQUAL { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(cevalRelationNotEqual(inValue1, inValue2)?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("- Ceval.cevalRelation failed on: "));
                __mm_s.push_str(&*ValuesDump::printValStr(inValue1)?);
                __mm_s.push_str(&*ExpressionDump::relopSymbol(inOperator)?);
                __mm_s.push_str(&*ValuesDump::printValStr(inValue2)?);
                ArcStr::from(__mm_s)
            })?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    outValue = metamodelica::Ref::new(Values::Value::BOOL { boolean: result });
    Ok(outValue)
}

fn cevalRelationLess(
    mut inValue1: &metamodelica::Ref<Values::Value>,
    mut inValue2: &metamodelica::Ref<Values::Value>,
) -> Result<bool> {
    let mut result: bool;
    result = (::match_deref::match_deref! { match (inValue1, inValue2) {
        (Deref @ Values::Value::STRING { .. }, Deref @ Values::Value::STRING { .. }) => stringCompare(&var_field!((**inValue1).string, Values::Value::STRING), &var_field!((**inValue2).string, Values::Value::STRING)) < 0,
        (Deref @ Values::Value::BOOL { .. }, Deref @ Values::Value::BOOL { .. }) => var_field!((**inValue1).boolean, Values::Value::BOOL).clone() < var_field!((**inValue2).boolean, Values::Value::BOOL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).integer, Values::Value::INTEGER).clone() < var_field!((**inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::REAL { .. }) => var_field!((**inValue1).real, Values::Value::REAL).clone() < var_field!((**inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::REAL { .. }) => intReal(var_field!((**inValue1).integer, Values::Value::INTEGER).clone()) < var_field!((**inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).real, Values::Value::REAL).clone() < intReal(var_field!((**inValue2).integer, Values::Value::INTEGER).clone()),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((**inValue1).index, Values::Value::ENUM_LITERAL).clone() < var_field!((**inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).index, Values::Value::ENUM_LITERAL).clone() < var_field!((**inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((**inValue1).integer, Values::Value::INTEGER).clone() < var_field!((**inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        _ => return Err("match: no arm matched"),
    } });
    Ok(result)
}

fn cevalRelationLessEq(
    mut inValue1: &metamodelica::Ref<Values::Value>,
    mut inValue2: &metamodelica::Ref<Values::Value>,
) -> Result<bool> {
    let mut result: bool;
    result = (::match_deref::match_deref! { match (inValue1, inValue2) {
        (Deref @ Values::Value::STRING { .. }, Deref @ Values::Value::STRING { .. }) => stringCompare(&var_field!((**inValue1).string, Values::Value::STRING), &var_field!((**inValue2).string, Values::Value::STRING)) <= 0,
        (Deref @ Values::Value::BOOL { .. }, Deref @ Values::Value::BOOL { .. }) => var_field!((**inValue1).boolean, Values::Value::BOOL).clone() <= var_field!((**inValue2).boolean, Values::Value::BOOL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).integer, Values::Value::INTEGER).clone() <= var_field!((**inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::REAL { .. }) => var_field!((**inValue1).real, Values::Value::REAL).clone() <= var_field!((**inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::REAL { .. }) => intReal(var_field!((**inValue1).integer, Values::Value::INTEGER).clone()) <= var_field!((**inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).real, Values::Value::REAL).clone() <= intReal(var_field!((**inValue2).integer, Values::Value::INTEGER).clone()),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((**inValue1).index, Values::Value::ENUM_LITERAL).clone() <= var_field!((**inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).index, Values::Value::ENUM_LITERAL).clone() <= var_field!((**inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((**inValue1).integer, Values::Value::INTEGER).clone() <= var_field!((**inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        _ => return Err("match: no arm matched"),
    } });
    Ok(result)
}

fn cevalRelationGreaterEq(
    mut inValue1: &metamodelica::Ref<Values::Value>,
    mut inValue2: &metamodelica::Ref<Values::Value>,
) -> Result<bool> {
    let mut result: bool;
    result = (::match_deref::match_deref! { match (inValue1, inValue2) {
        (Deref @ Values::Value::STRING { .. }, Deref @ Values::Value::STRING { .. }) => stringCompare(&var_field!((**inValue1).string, Values::Value::STRING), &var_field!((**inValue2).string, Values::Value::STRING)) >= 0,
        (Deref @ Values::Value::BOOL { .. }, Deref @ Values::Value::BOOL { .. }) => var_field!((**inValue1).boolean, Values::Value::BOOL).clone() >= var_field!((**inValue2).boolean, Values::Value::BOOL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).integer, Values::Value::INTEGER).clone() >= var_field!((**inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::REAL { .. }) => var_field!((**inValue1).real, Values::Value::REAL).clone() >= var_field!((**inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::REAL { .. }) => intReal(var_field!((**inValue1).integer, Values::Value::INTEGER).clone()) >= var_field!((**inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).real, Values::Value::REAL).clone() >= intReal(var_field!((**inValue2).integer, Values::Value::INTEGER).clone()),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((**inValue1).index, Values::Value::ENUM_LITERAL).clone() >= var_field!((**inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).index, Values::Value::ENUM_LITERAL).clone() >= var_field!((**inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((**inValue1).integer, Values::Value::INTEGER).clone() >= var_field!((**inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        _ => return Err("match: no arm matched"),
    } });
    Ok(result)
}

fn cevalRelationEqual(
    mut inValue1: &metamodelica::Ref<Values::Value>,
    mut inValue2: &metamodelica::Ref<Values::Value>,
) -> Result<bool> {
    let mut result: bool;
    result = (::match_deref::match_deref! { match (inValue1, inValue2) {
        (Deref @ Values::Value::STRING { .. }, Deref @ Values::Value::STRING { .. }) => stringCompare(&var_field!((**inValue1).string, Values::Value::STRING), &var_field!((**inValue2).string, Values::Value::STRING)) == 0,
        (Deref @ Values::Value::BOOL { .. }, Deref @ Values::Value::BOOL { .. }) => var_field!((**inValue1).boolean, Values::Value::BOOL).clone() == var_field!((**inValue2).boolean, Values::Value::BOOL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).integer, Values::Value::INTEGER).clone() == var_field!((**inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::REAL { .. }) => var_field!((**inValue1).real, Values::Value::REAL).clone() == var_field!((**inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::REAL { .. }) => intReal(var_field!((**inValue1).integer, Values::Value::INTEGER).clone()) == var_field!((**inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).real, Values::Value::REAL).clone() == intReal(var_field!((**inValue2).integer, Values::Value::INTEGER).clone()),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((**inValue1).index, Values::Value::ENUM_LITERAL).clone() == var_field!((**inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).index, Values::Value::ENUM_LITERAL).clone() == var_field!((**inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((**inValue1).integer, Values::Value::INTEGER).clone() == var_field!((**inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        _ => return Err("match: no arm matched"),
    } });
    Ok(result)
}

fn cevalRelationNotEqual(
    mut inValue1: &metamodelica::Ref<Values::Value>,
    mut inValue2: &metamodelica::Ref<Values::Value>,
) -> Result<bool> {
    let mut result: bool;
    result = (::match_deref::match_deref! { match (inValue1, inValue2) {
        (Deref @ Values::Value::STRING { .. }, Deref @ Values::Value::STRING { .. }) => stringCompare(&var_field!((**inValue1).string, Values::Value::STRING), &var_field!((**inValue2).string, Values::Value::STRING)) != 0,
        (Deref @ Values::Value::BOOL { .. }, Deref @ Values::Value::BOOL { .. }) => var_field!((**inValue1).boolean, Values::Value::BOOL).clone() != var_field!((**inValue2).boolean, Values::Value::BOOL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).integer, Values::Value::INTEGER).clone() != var_field!((**inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::REAL { .. }) => var_field!((**inValue1).real, Values::Value::REAL).clone() != var_field!((**inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::REAL { .. }) => intReal(var_field!((**inValue1).integer, Values::Value::INTEGER).clone()) != var_field!((**inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).real, Values::Value::REAL).clone() != intReal(var_field!((**inValue2).integer, Values::Value::INTEGER).clone()),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((**inValue1).index, Values::Value::ENUM_LITERAL).clone() != var_field!((**inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((**inValue1).index, Values::Value::ENUM_LITERAL).clone() != var_field!((**inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((**inValue1).integer, Values::Value::INTEGER).clone() != var_field!((**inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        _ => return Err("match: no arm matched"),
    } });
    Ok(result)
}

fn cevalRange(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut rangeExp: &metamodelica::Ref<DAE::Exp>,
    mut r#impl: bool,
    mut msg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut start: metamodelica::Ref<DAE::Exp>;
    let mut step: Option<metamodelica::Ref<DAE::Exp>>;
    let mut stop: metamodelica::Ref<DAE::Exp>;
    let mut range_ty: metamodelica::Ref<DAE::Type>;
    let mut vstart: metamodelica::Ref<Values::Value>;
    let mut vstop: metamodelica::Ref<Values::Value>;
    let mut istep: i32;
    let mut rstep: metamodelica::Real;
    let mut arr: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*rangeExp)) {
        Deref @ DAE::Exp::RANGE { ty: __pa0, start: __pa1, step: __pa2, stop: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    range_ty = metamodelica::Own::own(__pa0);
    start = metamodelica::Own::own(__pa1);
    step = metamodelica::Own::own(__pa2);
    stop = metamodelica::Own::own(__pa3);
    (outCache, vstart) = ceval(cache, env.clone(), start, r#impl, msg.clone(), numIter + 1)?;
    (outCache, vstop) = ceval(outCache, env.clone(), stop, r#impl, msg.clone(), numIter + 1)?;
    arr = (::match_deref::match_deref! { match &((&*vstart, &*vstop)) {
        (Deref @ Values::Value::BOOL { .. }, Deref @ Values::Value::BOOL { .. }) => ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut b in (ExpressionSimplify::simplifyRangeBool(var_field!((*vstart).boolean, Values::Value::BOOL).clone(), var_field!((*vstop).boolean, Values::Value::BOOL).clone())).into_iter().cloned() {
            let __x = ValuesMake::makeBoolean(b.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => {
            if (step).is_some() {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(outCache, env, Util::getOption(step)?, r#impl, msg, numIter + 1)?) {
                    (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                outCache = metamodelica::Own::own(__pa0);
                istep = metamodelica::Own::own(__pa1);
            } else {
                istep = 1;
            }
            ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut i in (ExpressionSimplify::simplifyRange(var_field!((*vstart).integer, Values::Value::INTEGER).clone(), istep, var_field!((*vstop).integer, Values::Value::INTEGER).clone())?).into_iter().cloned() {
            let __x = ValuesMake::makeInteger(i.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => cevalRangeEnum(var_field!((*vstart).index, Values::Value::ENUM_LITERAL).clone(), var_field!((*vstop).index, Values::Value::ENUM_LITERAL).clone(), &(Types::arrayElementType(&range_ty)))?,
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::REAL { .. }) => {
            if (step).is_some() {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(outCache, env, Util::getOption(step)?, r#impl, msg, numIter + 1)?) {
                    (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                outCache = metamodelica::Own::own(__pa0);
                rstep = metamodelica::Own::own(__pa1);
            } else {
                rstep = metamodelica::OrderedFloat(1.0_f64);
            }
            ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut r in (ExpressionSimplify::simplifyRangeReal(var_field!((*vstart).real, Values::Value::REAL).clone(), rstep, var_field!((*vstop).real, Values::Value::REAL).clone())?).into_iter().cloned() {
            let __x = ValuesMake::makeReal(r.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => return Err("match: no arm matched"),
    } });
    outValue = ValuesMake::makeArray(arr);
    Ok((outCache, outValue))
}

pub(crate) fn cevalRangeEnum(
    mut startIndex: i32,
    mut stopIndex: i32,
    mut enumType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut enumValList: metamodelica::List<metamodelica::Ref<Values::Value>>;
    enumValList = (match &**enumType {
        DAE::Type::T_ENUMERATION {
            path: enum_type,
            names: enum_names,
            ..
        } if (startIndex <= stopIndex) => {
            let mut enum_paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            let mut enum_values: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut enum_names = (*enum_names).clone();
            enum_names = List::sublist(enum_names.clone(), startIndex, stopIndex - startIndex + 1)?;
            enum_paths = List::map(enum_names.clone(), &fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr))?;
            enum_paths = List::map1r(enum_paths, &AbsynUtil::joinPaths, enum_type.clone())?;
            (enum_values, _) = List::mapFold(
                &enum_paths,
                &fnptr!(makeEnumValue, metamodelica::Ref<Absyn::Path>, i32),
                startIndex,
            )?;
            enum_values
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(enumValList)
}

fn makeEnumValue(mut name: metamodelica::Ref<Absyn::Path>, mut index: i32) -> (metamodelica::Ref<Values::Value>, i32) {
    let mut enumValue: metamodelica::Ref<Values::Value>;
    let mut newIndex: i32;
    enumValue = metamodelica::Ref::new(Values::Value::ENUM_LITERAL {
        name: name,
        index: index,
    });
    newIndex = index + 1;
    (enumValue, newIndex)
}

pub fn cevalList(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<Values::Value>>)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outValuesValueLst: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
    let mut expLstNew: metamodelica::List<metamodelica::Ref<DAE::Exp>> = inExpExpLst;
    let mut v: metamodelica::Ref<Values::Value>;
    for mut exp in &*expLstNew {
        (outCache, v) = ceval(
            outCache,
            inEnv.clone(),
            exp.clone(),
            inBoolean,
            inMsg.clone(),
            numIter + 1,
        )?;
        outValuesValueLst = metamodelica::cons(v, outValuesValueLst);
    }
    outValuesValueLst = metamodelica::Dangerous::listReverseInPlace(outValuesValueLst);
    Ok((outCache, outValuesValueLst))
}

pub(crate) fn cevalCref(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache, inEnv, inComponentRef, inBoolean, inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, c, r#impl, msg) => {
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut classEnv: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut name: ArcStr;
                    let mut const_for_range: Option<DAE::Const>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let mut cache = (*cache).clone();
                    (cache, attr, ty, binding, const_for_range, splicedExpData, classEnv, componentEnv, name) = Lookup::lookupVar(cache.clone(), env.clone(), c.clone())?;
                    (cache, v) = cevalCref_dispatch(cache.clone(), env.clone(), c.clone(), &attr, ty.clone(), &binding, const_for_range.clone(), &splicedExpData, &classEnv, &componentEnv, &name, r#impl.clone(), msg.clone(), numIter)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, c, false, Absyn::Msg::MSG { info }) => {
                    let mut scope_str: ArcStr;
                    let mut r#str: ArcStr;
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupVar(cache.clone(), env.clone(), c.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    scope_str = FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env));
                    r#str = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?;
                    Error::addSourceMessage(&(Error::LOOKUP_VARIABLE_ERROR.clone()), list![r#str.clone(), scope_str.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

pub(crate) fn cevalCref_dispatch(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inAttr: &metamodelica::Ref<DAE::Attributes>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inBinding: &metamodelica::Ref<DAE::Binding>,
    mut constForRange: Option<DAE::Const>,
    mut inSplicedExpData: &InstTypes::SplicedExpData,
    mut inClassEnv: &FCore::Graph,
    mut inComponentEnv: &FCore::Graph,
    mut inFQName: &ArcStr,
    mut inImpl: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match &((inAttr.clone(), inBinding.clone(), constForRange, inImpl, inMsg.clone())) {
        (_, Deref @ DAE::Binding::UNBOUND { .. }, Some(_), _, _) => {
            return Err("fail")
        },
        (_, Deref @ DAE::Binding::UNBOUND { .. }, None, false, Absyn::Msg::MSG { .. }) => {
            let mut v: metamodelica::Ref<Values::Value>;
            let mut r#str: ArcStr;
            let mut scope_str: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            r#str = ComponentReferenceBasics::printComponentRefStr(&inCref)?;
            scope_str = FGraph::printGraphPathStr(&inEnv);
            if Flags::isSet(Flags::CEVAL.clone())? {
                Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Ceval.cevalCref on: ")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(" failed with no constant binding in scope: ")); __mm_s.push_str(&*scope_str); ArcStr::from(__mm_s) })?;
            }
            s1 = FGraph::printGraphPathStr(&inEnv);
            s2 = ComponentReferenceBasics::printComponentRefStr(&inCref)?;
            s3 = TypesDump::printTypeStr(inType.clone());
            v = Types::typeToValue(&inType)?;
            v = metamodelica::Ref::new(Values::Value::EMPTY { scope: s1, name: s2, ty: v, tyStr: s3 });
            (inCache, v)
        },
        (Deref @ DAE::Attributes { variability, .. }, _, _, _, _) => {
            let mut cache: FCore::Cache;
            let mut v: metamodelica::Ref<Values::Value>;
            let true = (SCodeUtil::isParameterOrConst(variability.clone()) || inImpl || FGraph::inForLoopScope(&inEnv)) else { return Err("pattern mismatch") };
            let false = (crefEqualValue(&inCref, inBinding)?) else { return Err("pattern mismatch") };
            (cache, v) = cevalCrefBinding(inCache, inEnv, inCref.clone(), inBinding, inImpl, inMsg, numIter)?;
            cache = FCore::addEvaluatedCref(cache, variability.clone(), ComponentReferenceBasics::crefStripLastSubs(&inCref)?);
            (cache, v)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outValue))
}

pub(crate) fn cevalCrefBinding(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inBinding: &metamodelica::Ref<DAE::Binding>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inComponentRef.clone(),
            &**inBinding,
            inBoolean,
            inMsg.clone(),
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr, Deref @ DAE::Binding::VALBOUND { valBound: v, .. }, r#impl, msg) => {
                    let mut subsc: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut res: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    subsc = ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cr))?;
                    (cache, res) = cevalSubscriptValue(cache.clone(), env.clone(), subsc.clone(), v.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: ty, subscriptLst: Deref @ metamodelica::ListNode::Nil }, Deref @ DAE::Binding::UNBOUND { .. }, _, Absyn::Msg::MSG { info }) => {
                    let mut res: metamodelica::Ref<Values::Value>;
                    let mut vl: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut tpath: metamodelica::Ref<Absyn::Path>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::arrayElementType(metamodelica::AsArg::as_arg(&ty))) {
                        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, varLst: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    tpath = metamodelica::Own::own(__pa0);
                    vl = metamodelica::Own::own(__pa1);
                    let true = (Types::allHaveBindings(&vl)?) else { return Err("pattern mismatch") };
                    binding = InstBinding::makeRecordBinding(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), tpath.clone(), metamodelica::AsArg::as_arg(&ty), &vl, metamodelica::nil(), metamodelica::AsArg::as_arg(&info))?;
                    (cache, res) = cevalCrefBinding(cache.clone(), env.clone(), inComponentRef.clone(), &binding, inBoolean, inMsg.clone(), numIter + 1)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Binding::UNBOUND { .. }, false, Absyn::Msg::MSG { info: _ }) => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Binding::UNBOUND { .. }, true, Absyn::Msg::MSG { info: _ }) => {
                    let true = (Flags::isSet(Flags::CEVAL.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("#- Ceval.cevalCrefBinding: Ignoring unbound when implicit\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr, Deref @ DAE::Binding::EQBOUND { exp, constant_: DAE::Const::C_CONST { .. }, .. }, r#impl, msg) => {
                    let mut subsc: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut res: metamodelica::Ref<Values::Value>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    ::match_deref::match_deref! { match &(exp.clone()) {
                        Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: Deref @ Absyn::Path::IDENT { .. }, .. }, iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    (cache, v) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    subsc = ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cr))?;
                    (cache, res) = cevalSubscriptValue(cache.clone(), env.clone(), subsc.clone(), v.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr, Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(e_val), .. }, r#impl, msg) => {
                    let mut subsc: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut res: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    subsc = ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cr))?;
                    (cache, res) = cevalSubscriptValue(cache.clone(), env.clone(), subsc.clone(), e_val.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr, Deref @ DAE::Binding::EQBOUND { exp, constant_: DAE::Const::C_CONST { .. }, .. }, r#impl, msg) => {
                    let mut subsc: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut res: metamodelica::Ref<Values::Value>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, v) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    subsc = ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cr))?;
                    (cache, res) = cevalSubscriptValue(cache.clone(), env.clone(), subsc.clone(), v.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr, Deref @ DAE::Binding::EQBOUND { exp, constant_: DAE::Const::C_PARAM { .. }, .. }, r#impl, msg) => {
                    let mut subsc: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut res: metamodelica::Ref<Values::Value>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let false = (isRecursiveBinding(metamodelica::AsArg::as_arg(&cr), exp.clone())) else { return Err("pattern mismatch") };
                    (cache, v) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    subsc = ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cr))?;
                    (cache, res) = cevalSubscriptValue(cache.clone(), env.clone(), subsc.clone(), v.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Binding::EQBOUND { exp, constant_: DAE::Const::C_VAR { .. }, .. }, _, Absyn::Msg::MSG { info: _ }) => {
                    let mut expstr: ArcStr;
                    let true = (Flags::isSet(Flags::CEVAL.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("#- Ceval.cevalCrefBinding failed (nonconstant EQBOUND("))?;
                    expstr = ExpressionBasics::printExpStr(exp.clone())?;
                    Debug::trace(expstr.clone())?;
                    Debug::traceln(literal!("))"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, e1, _, _, _) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let true = (Flags::isSet(Flags::CEVAL.clone())?) else { return Err("pattern mismatch") };
                    s1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&e1))?;
                    s2 = TypesDump::printBindingStr(inBinding)?;
                    r#str = FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env));
                    r#str = stringAppendList(list![literal!("- Ceval.cevalCrefBinding: "), s1.clone(), literal!(" = ["), s2.clone(), literal!("] in env:"), r#str.clone(), literal!(" failed")]);
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
    Ok((outCache, outValue))
}

fn isRecursiveBinding(mut cr: &metamodelica::Ref<DAE::ComponentRef>, mut exp: metamodelica::Ref<DAE::Exp>) -> bool {
    let mut res: bool = false;
    res = 'mc: {
        let __mc_input = &*exp;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut res: bool = res.clone();
                    res = List::any(&(Expression::extractCrefsFromExp(exp.clone())?), &({ let __pe_b1 = cr.clone(); move |__pe_a0| ComponentReferenceBasics::crefEqual(&__pe_a0, &__pe_b1) }))?;
                    Ok((res, res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
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
    res
}

pub(crate) fn cevalSubscriptValue(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpSubscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (::match_deref::match_deref! { match &((inExpSubscriptLst, inValue)) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp }, tail: subs }, Deref @ Values::Value::ARRAY { valueLst: lst, .. }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut n: i32;
            let mut subval: metamodelica::Ref<Values::Value>;
            let mut res: metamodelica::Ref<Values::Value>;
            let mut v: metamodelica::Ref<Values::Value>;
            (cache, v) = ceval(cache, env.clone(), exp.clone(), r#impl, msg.clone(), numIter + 1)?;
            n = (match &*v {
        Values::Value::INTEGER { integer: __esc_n } => {
            n = (*__esc_n).clone();
            n.clone()
        },
        Values::Value::ENUM_LITERAL { index: __esc_n, .. } => {
            n = (*__esc_n).clone();
            n.clone()
        },
        _ => return Err("match: no arm matched"),
    });
            subval = (lst).get(n)?;
            (cache, res) = cevalSubscriptValue(cache, env, subs.clone(), subval, r#impl, msg, numIter + 1)?;
            (cache, res)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp }, tail: subs }, Deref @ Values::Value::ARRAY { valueLst: lst, .. }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut res: metamodelica::Ref<Values::Value>;
            let mut sliceLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut subvals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut slice: metamodelica::List<i32>;
            let mut lst = (*lst).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, env.clone(), exp.clone(), r#impl, msg.clone(), numIter + 1)?) {
                (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            sliceLst = metamodelica::Own::own(__pa1);
            slice = List::map(sliceLst, &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::valueInteger(&__a0))?;
            subvals = List::map1r(slice, &listGet, lst.clone())?;
            (cache, lst) = cevalSubscriptValueList(cache, env, subs.clone(), &subvals, r#impl, msg, numIter)?;
            res = ValuesMake::makeArray(lst.clone());
            (cache, res)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: subs }, subval @ Deref @ Values::Value::ARRAY { .. }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut res: metamodelica::Ref<Values::Value>;
            let mut lst: metamodelica::List<metamodelica::Ref<Values::Value>>;
            if (subs).is_empty() {
                res = subval.clone();
            } else {
                (cache, lst) = cevalSubscriptValueList(cache, env, subs.clone(), var_field!((**subval).valueLst, Values::Value::ARRAY), r#impl, msg, numIter + 1)?;
                res = ValuesMake::makeArray(lst);
            }
            (cache, res)
        },
        (Deref @ metamodelica::ListNode::Nil, v) => {
            let mut cache = inCache;
            (cache, v.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalSubscriptValueList(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpSubscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inValue: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<Values::Value>>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::List<metamodelica::Ref<Values::Value>>;
    (outCache, outValue) = (::match_deref::match_deref! { match inValue {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache;
            (cache, metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: subval, tail: subvals } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut subs = inExpSubscriptLst;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut res: metamodelica::Ref<Values::Value>;
            let mut lst: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (cache, res) = cevalSubscriptValue(cache, env.clone(), subs.clone(), subval.clone(), r#impl, msg.clone(), numIter + 1)?;
            (cache, lst) = cevalSubscriptValueList(cache, env, subs, subvals, r#impl, msg, numIter)?;
            (cache, metamodelica::cons(res, lst))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outValue))
}

pub(crate) fn cevalSubscripts(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpSubscriptLst: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inIntegerLst: &metamodelica::List<i32>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Subscript>>)> {
    let mut outCache: FCore::Cache;
    let mut outExpSubscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    (outCache, outExpSubscriptLst) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inExpSubscriptLst, &**inIntegerLst, inBoolean, inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ metamodelica::ListNode::Nil, _, _, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: sub, tail: subs }, Deref @ metamodelica::ListNode::Cons { head: dim, tail: dims }, r#impl, msg) => {
                    let mut sub_1: metamodelica::Ref<DAE::Subscript>;
                    let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut cache = (*cache).clone();
                    (cache, sub_1) = cevalSubscript(cache.clone(), env.clone(), sub.clone(), dim.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    (cache, subs_1) = cevalSubscripts(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&subs), metamodelica::AsArg::as_arg(&dims), r#impl.clone(), msg.clone(), numIter)?;
                    Ok((cache.clone(), metamodelica::cons(sub_1.clone(), subs_1.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: sub, tail: subs }, Deref @ metamodelica::ListNode::Cons { head: dim, tail: dims }, r#impl, msg) => {
                    let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut cache = (*cache).clone();
                    if '__try0: {
                        unwrap_break_err!(cevalSubscript(cache.clone(), env.clone(), sub.clone(), dim.clone(), r#impl.clone(), msg.clone(), numIter + 1), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    (cache, subs_1) = cevalSubscripts(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&subs), metamodelica::AsArg::as_arg(&dims), r#impl.clone(), msg.clone(), numIter)?;
                    Ok((cache.clone(), metamodelica::cons(sub.clone(), subs_1.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExpSubscriptLst))
}

pub(crate) fn cevalSubscript(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inSubscript: metamodelica::Ref<DAE::Subscript>,
    mut inInteger: i32,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Subscript>)> {
    let mut outCache: FCore::Cache;
    let mut outSubscript: metamodelica::Ref<DAE::Subscript>;
    (outCache, outSubscript) = 'mc: {
        let __mc_input = (inCache, inEnv, &*inSubscript, inBoolean, inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Subscript::WHOLEDIM { .. }, _, _) => {
                    Ok((cache.clone(), openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ENUM_LITERAL { .. } }, _, _) => {
                    Ok((cache.clone(), inSubscript.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, Deref @ DAE::Subscript::INDEX { exp: e1 }, r#impl, msg) => {
                            let mut v1: metamodelica::Ref<Values::Value>;
                            let mut e1_1: metamodelica::Ref<DAE::Exp>;
                            let mut cache = (*cache).clone();
                            (cache, v1) = ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                            e1_1 = (match &*v1 {
                Values::Value::INTEGER { integer: _ } => ValuesUtil::valueExp(v1.clone(), None)?,
                Values::Value::ENUM_LITERAL { .. } => ValuesUtil::valueExp(v1.clone(), None)?,
                Values::Value::BOOL { boolean: _ } => ValuesUtil::valueExp(v1.clone(), None)?,
                _ => return Err("match: no arm matched"),
            });
                            Ok((cache.clone(), metamodelica::Ref::new(DAE::Subscript::INDEX { exp: e1_1.clone() })))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Subscript::SLICE { exp: e1 }, r#impl, msg) => {
                    let mut v1: metamodelica::Ref<Values::Value>;
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, v1) = ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    e1_1 = ValuesUtil::valueExp(v1.clone(), Some(e1.clone()))?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Subscript::SLICE { exp: e1_1.clone() })))
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

fn crefEqualValue(
    mut c: &metamodelica::Ref<DAE::ComponentRef>,
    mut v: &metamodelica::Ref<DAE::Binding>,
) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match v {
        Deref @ DAE::Binding::EQBOUND { exp: Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, evaluatedExp: None, constant_: _, source: _ } => {
            ComponentReferenceBasics::crefEqual(c, metamodelica::AsArg::as_arg(&cr))?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

fn dimensionSliceInRange(mut arr: &metamodelica::Ref<Values::Value>, mut dimSize: i32) -> bool {
    let mut inRange: bool;
    inRange = 'mc: {
        let __mc_input = &**arr;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Nil, .. } => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: indx }, tail: vlst }, dimLst: Deref @ metamodelica::ListNode::Cons { head: dim, tail: dims } } => {
                    let mut dim = (*dim).clone();
                    let mut dims = (*dims).clone();
                    dim = dim.clone() - 1;
                    dims = metamodelica::cons(dim.clone(), dims.clone());
                    let true = (indx.clone() <= dimSize) else { return Err("pattern mismatch") };
                    let true = (dimensionSliceInRange(&(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: vlst.clone(), dimLst: dims.clone() })), dimSize)) else { return Err("pattern mismatch") };
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
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    inRange
}

fn cevalReduction<'__b>(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut opPath: &'__b metamodelica::Ref<Absyn::Path>,
    mut inCurValue: Option<metamodelica::Ref<Values::Value>>,
    mut exp: &'__b metamodelica::Ref<DAE::Exp>,
    mut exprType: &'__b metamodelica::Ref<DAE::Type>,
    mut foldName: &'__b ArcStr,
    mut resultName: &'__b ArcStr,
    mut foldExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut iteratorNames: &'__b metamodelica::List<ArcStr>,
    mut inValueMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>,
    mut iterTypes: &'__b metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut r#impl: bool,
    mut msg: &'__b Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, Option<metamodelica::Ref<Values::Value>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCache, inEnv, opPath.clone(), inCurValue.clone(), inValueMatrix)) {
            (cache, _, Deref @ Absyn::Path::IDENT { name: Deref @ "list" }, Some(Deref @ Values::Value::LIST { valueLst: vals }), Deref @ metamodelica::ListNode::Nil) => {
                let mut vals = (*vals).clone();
                vals = vals.clone().reverse();
                return Ok((cache.clone(), Some(metamodelica::Ref::new(Values::Value::LIST { valueLst: vals.clone() }))))
            },
            (cache, _, Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, Some(Deref @ Values::Value::LIST { valueLst: _ }), Deref @ metamodelica::ListNode::Nil) => {
                return Ok((cache.clone(), inCurValue))
            },
            (cache, _, Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, Some(Deref @ Values::Value::ARRAY { valueLst: vals, dimLst: dims }), Deref @ metamodelica::ListNode::Nil) => {
                let mut vals = (*vals).clone();
                vals = vals.clone().reverse();
                return Ok((cache.clone(), Some(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: vals.clone(), dimLst: dims.clone() }))))
            },
            (cache, _, _, curValue, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((cache.clone(), curValue.clone()))
            },
            (cache, env, _, curValue, Deref @ metamodelica::ListNode::Cons { head: vals, tail: valueMatrix }) => {
                let mut new_env: FCore::Graph;
                let mut cache = (*cache).clone();
                let mut curValue = (*curValue).clone();
                new_env = extendFrameForIterators(env.clone(), iteratorNames.clone(), vals.clone(), iterTypes.clone())?;
                (cache, curValue) = cevalReductionEvalAndFold(cache.clone(), new_env, opPath, curValue.clone(), exp.clone(), exprType.clone(), foldName.clone(), resultName.clone(), foldExp.clone(), r#impl, msg.clone(), numIter + 1)?;
                { (inCache, inEnv, opPath, inCurValue, exp, exprType, foldName, resultName, foldExp, iteratorNames, inValueMatrix, iterTypes, r#impl, msg, numIter) = (cache.clone(), env.clone(), opPath, curValue.clone(), exp, exprType, foldName, resultName, foldExp, iteratorNames, valueMatrix.clone(), iterTypes, r#impl, msg, numIter); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn cevalReductionEvalAndFold(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut opPath: &metamodelica::Ref<Absyn::Path>,
    mut inCurValue: Option<metamodelica::Ref<Values::Value>>,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut exprType: metamodelica::Ref<DAE::Type>,
    mut foldName: ArcStr,
    mut resultName: ArcStr,
    mut foldExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut r#impl: bool,
    mut msg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, Option<metamodelica::Ref<Values::Value>>)> {
    let mut newCache: FCore::Cache;
    let mut result: Option<metamodelica::Ref<Values::Value>>;
    (newCache, result) = (::match_deref::match_deref! { match &((inCache, inEnv, inCurValue)) {
        (cache, env, curValue) => {
            let mut value: metamodelica::Ref<Values::Value>;
            let mut cache = (*cache).clone();
            (cache, value) = ceval(cache.clone(), env.clone(), exp, r#impl, msg.clone(), numIter + 1)?;
            (cache, result) = cevalReductionFold(cache.clone(), env.clone(), opPath, curValue.clone(), value, foldName, resultName, foldExp, exprType, r#impl, msg, numIter)?;
            (cache.clone(), result)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((newCache, result))
}

fn cevalReductionFold(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut opPath: &metamodelica::Ref<Absyn::Path>,
    mut inCurValue: Option<metamodelica::Ref<Values::Value>>,
    mut inValue: metamodelica::Ref<Values::Value>,
    mut foldName: ArcStr,
    mut resultName: ArcStr,
    mut foldExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut exprType: metamodelica::Ref<DAE::Type>,
    mut r#impl: bool,
    mut msg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, Option<metamodelica::Ref<Values::Value>>)> {
    let mut newCache: FCore::Cache;
    let mut result: Option<metamodelica::Ref<Values::Value>>;
    (newCache, result) = (::match_deref::match_deref! { match &((&**opPath, inCurValue, foldExp)) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, Some(value), _) => {
            let mut cache = inCache;
            let mut value = (*value).clone();
            value = valueArrayCons(ValuesUtil::unboxIfBoxedVal(inValue), value.clone());
            (cache, Some(value.clone()))
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "list" }, Some(value), _) => {
            let mut cache = inCache;
            let mut value = (*value).clone();
            value = valueCons(ValuesUtil::unboxIfBoxedVal(inValue), metamodelica::AsArg::as_arg(&value))?;
            (cache, Some(value.clone()))
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, Some(value), _) => {
            let mut cache = inCache;
            let mut value = (*value).clone();
            value = valueCons(ValuesUtil::unboxIfBoxedVal(inValue), metamodelica::AsArg::as_arg(&value))?;
            (cache, Some(value.clone()))
        },
        (_, None, _) => {
            let mut cache = inCache;
            (cache, Some(inValue))
        },
        (_, Some(value), Some(exp)) => {
            let mut cache = inCache;
            let mut env: FCore::Graph;
            let mut value = (*value).clone();
            env = FGraph::addForIterator(inEnv, foldName, exprType.clone(), metamodelica::Ref::new(DAE::Binding::VALBOUND { valBound: inValue, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_CONST))?;
            env = FGraph::addForIterator(env, resultName, exprType, metamodelica::Ref::new(DAE::Binding::VALBOUND { valBound: value.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_CONST))?;
            (cache, value) = ceval(cache, env, exp.clone(), r#impl, msg, numIter + 1)?;
            (cache, Some(value.clone()))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((newCache, result))
}

fn valueArrayCons(
    mut v1: metamodelica::Ref<Values::Value>,
    mut v2: metamodelica::Ref<Values::Value>,
) -> metamodelica::Ref<Values::Value> {
    let mut res: metamodelica::Ref<Values::Value>;
    res = (::match_deref::match_deref! { match &(v2.clone()) {
        Deref @ Values::Value::ARRAY { valueLst: vals, dimLst: Deref @ metamodelica::ListNode::Cons { head: dim_size, tail: rest_dims } } => {
            let mut dim_size = (*dim_size).clone();
            dim_size = dim_size.clone() + 1;
            metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::cons(v1, vals.clone()), dimLst: metamodelica::cons(dim_size.clone(), rest_dims.clone()) })
        },
        _ => {
            metamodelica::Ref::new(Values::Value::ARRAY { valueLst: list![v1, v2], dimLst: list![2] })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn valueCons(
    mut inV1: metamodelica::Ref<Values::Value>,
    mut inV2: &metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut res: metamodelica::Ref<Values::Value>;
    res = (::match_deref::match_deref! { match &((inV1, &**inV2)) {
        (Deref @ Values::Value::META_BOX { value: v1 }, Deref @ Values::Value::LIST { valueLst: vals }) => {
            metamodelica::Ref::new(Values::Value::LIST { valueLst: metamodelica::cons(v1.clone(), vals.clone()) })
        },
        (v1, Deref @ Values::Value::LIST { valueLst: vals }) => {
            metamodelica::Ref::new(Values::Value::LIST { valueLst: metamodelica::cons(v1.clone(), vals.clone()) })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(res)
}

fn cevalReductionIterators(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIterators: &metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>,
    mut r#impl: bool,
    mut msg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>,
    metamodelica::List<ArcStr>,
    metamodelica::List<i32>,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut vals: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>> = metamodelica::nil();
    let mut names: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut dims: metamodelica::List<i32> = metamodelica::nil();
    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
    let mut val: metamodelica::Ref<Values::Value>;
    let mut iterVals: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut id: ArcStr;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut guardExp: Option<metamodelica::Ref<DAE::Exp>>;
    for mut iter in &**inIterators {
        let __arc4 = iter.clone();
        let DAE::REDUCTIONITER {
            id: __pa0,
            exp: __pa1,
            guardExp: __pa2,
            ty: __pa3,
        } = &*__arc4;
        id = metamodelica::Own::own(__pa0);
        exp = metamodelica::Own::own(__pa1);
        guardExp = metamodelica::Own::own(__pa2);
        ty = metamodelica::Own::own(__pa3);
        (outCache, val) = ceval(outCache, inEnv.clone(), exp, r#impl, msg.clone(), numIter + 1)?;
        iterVals = ValuesUtil::arrayOrListVals(&val, true)?;
        (outCache, iterVals) = filterReductionIterator(
            outCache,
            inEnv.clone(),
            &id,
            &ty,
            iterVals,
            guardExp,
            r#impl,
            &msg,
            numIter,
        )?;
        vals = metamodelica::cons(iterVals.clone(), vals);
        names = metamodelica::cons(id, names);
        dims = metamodelica::cons(((iterVals).len() as i32), dims);
        tys = metamodelica::cons(ty, tys);
    }
    Ok((outCache, vals, names, dims, tys))
}

fn filterReductionIterator(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut id: &ArcStr,
    mut ty: &metamodelica::Ref<DAE::Type>,
    mut inVals: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut guardExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut r#impl: bool,
    mut msg: &Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<Values::Value>>)> {
    let mut outCache: FCore::Cache;
    let mut outVals: metamodelica::List<metamodelica::Ref<Values::Value>>;
    (outCache, outVals) = (::match_deref::match_deref! { match &((inVals, guardExp.clone())) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            let mut cache = inCache;
            (cache, metamodelica::nil())
        },
        (Deref @ metamodelica::ListNode::Cons { head: val, tail: vals }, Some(exp)) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut b: bool;
            let mut new_env: FCore::Graph;
            let mut vals = (*vals).clone();
            new_env = FGraph::addForIterator(env.clone(), id.clone(), ty.clone(), metamodelica::Ref::new(DAE::Binding::VALBOUND { valBound: val.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_CONST))?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache, new_env, exp.clone(), r#impl, msg.clone(), numIter + 1)?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            b = metamodelica::Own::own(__pa1);
            (cache, vals) = filterReductionIterator(cache, env, id, ty, vals.clone(), guardExp, r#impl, msg, numIter)?;
            vals = if (b) {metamodelica::cons(val.clone(), vals.clone())} else {vals.clone()};
            (cache, vals.clone())
        },
        (vals, None) => {
            let mut cache = inCache;
            (cache, vals.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outVals))
}

fn extendFrameForIterators(
    mut inEnv: FCore::Graph,
    mut inNames: metamodelica::List<ArcStr>,
    mut inVals: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inTys: metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<FCore::Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inEnv, inNames, inVals, inTys)) {
            (env, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(env.clone())
            },
            (env, Deref @ metamodelica::ListNode::Cons { head: name, tail: names }, Deref @ metamodelica::ListNode::Cons { head: val, tail: vals }, Deref @ metamodelica::ListNode::Cons { head: ty, tail: tys }) => {
                let mut env = (*env).clone();
                env = FGraph::addForIterator(env.clone(), name.clone(), ty.clone(), metamodelica::Ref::new(DAE::Binding::VALBOUND { valBound: val.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_CONST))?;
                { (inEnv, inNames, inVals, inTys) = (env.clone(), names.clone(), vals.clone(), tys.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn backpatchArrayReduction(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut iterType: Absyn::ReductionIterType,
    mut inValue: metamodelica::Ref<Values::Value>,
    mut dims: metamodelica::List<i32>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = (::match_deref::match_deref! { match &((path.clone(), iterType, inValue.clone(), dims.clone())) {
        (_, _, value, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }) => {
            value.clone()
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, Absyn::ReductionIterType::COMBINE { .. }, Deref @ Values::Value::ARRAY { valueLst: vals, .. }, _) => {
            let mut value: metamodelica::Ref<Values::Value>;
            value = backpatchArrayReduction3(vals.clone(), dims.reverse(), &fnptr!(ValuesMake::makeArray, metamodelica::List<metamodelica::Ref<Values::Value>>))?;
            value
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "list" }, Absyn::ReductionIterType::COMBINE { .. }, Deref @ Values::Value::LIST { valueLst: vals }, _) => {
            let mut value: metamodelica::Ref<Values::Value>;
            value = backpatchArrayReduction3(vals.clone(), dims.reverse(), &fnptr!(ValuesMake::makeList, metamodelica::List<metamodelica::Ref<Values::Value>>))?;
            value
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, Absyn::ReductionIterType::COMBINE { .. }, Deref @ Values::Value::LIST { valueLst: vals }, _) => {
            let mut value: metamodelica::Ref<Values::Value>;
            value = backpatchArrayReduction3(vals.clone(), dims.reverse(), &fnptr!(ValuesMake::makeList, metamodelica::List<metamodelica::Ref<Values::Value>>))?;
            value
        },
        _ => {
            inValue
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

fn backpatchArrayReduction3<'__b>(
    mut inVals: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inDims: metamodelica::List<i32>,
    mut makeSequence: &'__b dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::Ref<Values::Value>>,
    ) -> Result<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::Ref<Values::Value>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<Values::Value>>,
            ) -> Result<metamodelica::Ref<Values::Value>>
            + 'static,
    >;

    '__tco: loop {
        ::match_deref::match_deref! { match &((inVals, inDims)) {
            (vals, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }) => {
                let mut value: metamodelica::Ref<Values::Value>;
                return Ok(makeSequence(vals.clone())?)
            },
            (vals, Deref @ metamodelica::ListNode::Cons { head: dim, tail: dims }) => {
                let mut valMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>;
                let mut value: metamodelica::Ref<Values::Value>;
                let mut vals = (*vals).clone();
                valMatrix = List::partition(vals.clone(), dim.clone())?;
                vals = List::map(valMatrix, makeSequence)?;
                { (inVals, inDims, makeSequence) = (vals.clone(), dims.clone(), makeSequence); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn cevalSimple(mut exp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut val: metamodelica::Ref<Values::Value>;
    (_, val) = ceval(
        FCore::emptyCache(),
        FGraph::empty(),
        exp,
        false,
        Absyn::Msg::MSG {
            info: Absyn::dummyInfo.clone(),
        },
        0,
    )?;
    Ok(val)
}

pub fn cevalSimpleWithFunctionTreeReturnExp(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut functions: metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut oexp: metamodelica::Ref<DAE::Exp>;
    let mut val: metamodelica::Ref<Values::Value>;
    let mut cache: FCore::Cache;
    let mut structuralParameters: (
        metamodelica::Ref<AvlSetCR::Tree>,
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
    );
    let mut functionTree: Mutable::Mutable<metamodelica::Ref<AvlTreePathFunction::Tree>>;
    structuralParameters = (
        openmodelica_frontend_dump::AvlSetCR::Tree::interned_EMPTY(),
        metamodelica::nil(),
    );
    functionTree = Mutable::create(functions);
    cache = FCore::Cache::CACHE {
        initialGraph: None,
        functions: functionTree,
        evaluatedParams: structuralParameters,
        modelName: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
    };
    (_, val) = ceval(
        cache,
        FGraph::empty(),
        exp.clone(),
        false,
        Absyn::Msg::MSG {
            info: Absyn::dummyInfo.clone(),
        },
        0,
    )?;
    oexp = ValuesUtil::valueExp(val, Some(exp))?;
    Ok(oexp)
}

pub(crate) fn cevalAstExp(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<Absyn::Exp>)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    (outCache, outExp) = 'mc: {
        let __mc_input = (inCache, inEnv, inExp, inBoolean, inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, e @ Deref @ Absyn::Exp::INTEGER { .. }, _, _) => {
                    Ok((cache.clone(), e.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, e @ Deref @ Absyn::Exp::REAL { .. }, _, _) => {
                    Ok((cache.clone(), e.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, e @ Deref @ Absyn::Exp::CREF { .. }, _, _) => {
                    Ok((cache.clone(), e.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, e @ Deref @ Absyn::Exp::STRING { .. }, _, _) => {
                    Ok((cache.clone(), e.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, e @ Deref @ Absyn::Exp::BOOL { .. }, _, _) => {
                    Ok((cache.clone(), e.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::BINARY { exp1: e1, op, exp2: e2 }, r#impl, msg) => {
                    let mut e1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut e2_1: metamodelica::Ref<Absyn::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, e1_1) = cevalAstExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), info)?;
                    (cache, e2_1) = cevalAstExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: e1_1.clone(), op: op.clone(), exp2: e2_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::UNARY { op, exp: e }, r#impl, msg) => {
                    let mut e_1: metamodelica::Ref<Absyn::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, e_1) = cevalAstExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Absyn::Exp::UNARY { op: op.clone(), exp: e_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::LBINARY { exp1: e1, op, exp2: e2 }, r#impl, msg) => {
                    let mut e1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut e2_1: metamodelica::Ref<Absyn::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, e1_1) = cevalAstExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), info)?;
                    (cache, e2_1) = cevalAstExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Absyn::Exp::LBINARY { exp1: e1_1.clone(), op: op.clone(), exp2: e2_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::LUNARY { op, exp: e }, r#impl, msg) => {
                    let mut e_1: metamodelica::Ref<Absyn::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, e_1) = cevalAstExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Absyn::Exp::LUNARY { op: op.clone(), exp: e_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::RELATION { exp1: e1, op, exp2: e2 }, r#impl, msg) => {
                    let mut e1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut e2_1: metamodelica::Ref<Absyn::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, e1_1) = cevalAstExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), info)?;
                    (cache, e2_1) = cevalAstExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Absyn::Exp::RELATION { exp1: e1_1.clone(), op: op.clone(), exp2: e2_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::IFEXP { ifExp: cond, trueBranch: then_, elseBranch: else_, elseIfBranch: nest }, r#impl, msg) => {
                    let mut cond_1: metamodelica::Ref<Absyn::Exp>;
                    let mut then_1: metamodelica::Ref<Absyn::Exp>;
                    let mut else_1: metamodelica::Ref<Absyn::Exp>;
                    let mut nest_1: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>;
                    let mut cache = (*cache).clone();
                    (cache, cond_1) = cevalAstExp(cache.clone(), env.clone(), cond.clone(), r#impl.clone(), msg.clone(), info)?;
                    (cache, then_1) = cevalAstExp(cache.clone(), env.clone(), then_.clone(), r#impl.clone(), msg.clone(), info)?;
                    (cache, else_1) = cevalAstExp(cache.clone(), env.clone(), else_.clone(), r#impl.clone(), msg.clone(), info)?;
                    (cache, nest_1) = cevalAstExpexpList(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&nest), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Absyn::Exp::IFEXP { ifExp: cond_1.clone(), trueBranch: then_1.clone(), elseBranch: else_1.clone(), elseIfBranch: nest_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "Eval", subscripts: Deref @ metamodelica::ListNode::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, argNames: Deref @ metamodelica::ListNode::Nil }, .. }, r#impl, msg) => {
                    let mut exp: metamodelica::Ref<Absyn::Exp>;
                    let mut daeExp: metamodelica::Ref<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, daeExp, _) = Static::elabExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), daeExp.clone(), r#impl.clone(), msg.clone(), 0)?) {
                        (__pa0, Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: __pa1 } }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    exp = metamodelica::Own::own(__pa1);
                    Ok((cache.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, e @ Deref @ Absyn::Exp::CALL { .. }, _, _) => {
                    Ok((cache.clone(), e.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::ARRAY { arrayExp: expl }, r#impl, msg) => {
                    let mut expl_1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut cache = (*cache).clone();
                    (cache, expl_1) = cevalAstExpList(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&expl), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: expl_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::MATRIX { matrix: lstExpl }, r#impl, msg) => {
                    let mut lstExpl_1: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>;
                    let mut cache = (*cache).clone();
                    (cache, lstExpl_1) = cevalAstExpListList(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&lstExpl), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Absyn::Exp::MATRIX { matrix: lstExpl_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::RANGE { start: e1, step: Some(e2), stop: e3 }, r#impl, msg) => {
                    let mut e1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut e2_1: metamodelica::Ref<Absyn::Exp>;
                    let mut e3_1: metamodelica::Ref<Absyn::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, e1_1) = cevalAstExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), info)?;
                    (cache, e2_1) = cevalAstExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), info)?;
                    (cache, e3_1) = cevalAstExp(cache.clone(), env.clone(), e3.clone(), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Absyn::Exp::RANGE { start: e1_1.clone(), step: Some(e2_1.clone()), stop: e3_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::RANGE { start: e1, step: None, stop: e3 }, r#impl, msg) => {
                    let mut e1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut e3_1: metamodelica::Ref<Absyn::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, e1_1) = cevalAstExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), info)?;
                    (cache, e3_1) = cevalAstExp(cache.clone(), env.clone(), e3.clone(), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Absyn::Exp::RANGE { start: e1_1.clone(), step: None, stop: e3_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::TUPLE { expressions: expl }, r#impl, msg) => {
                    let mut expl_1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut cache = (*cache).clone();
                    (cache, expl_1) = cevalAstExpList(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&expl), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Absyn::Exp::TUPLE { expressions: expl_1.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Exp::END { .. }, _, _) => {
                    Ok((cache.clone(), openmodelica_ast::Absyn::Exp::interned_END()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, e @ Deref @ Absyn::Exp::CODE { .. }, _, _) => {
                    Ok((cache.clone(), e.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp))
}

pub(crate) fn cevalAstExpList(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynExpLst: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<Absyn::Exp>>)> {
    let mut outCache: FCore::Cache;
    let mut outAbsynExpLst: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    (outCache, outAbsynExpLst) = (::match_deref::match_deref! { match inAbsynExpLst {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache;
            (cache, metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: es } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            (cache, _) = cevalAstExp(cache, env.clone(), e.clone(), r#impl, msg.clone(), info)?;
            (cache, res) = cevalAstExpList(cache, env, es, r#impl, msg, info)?;
            (cache, metamodelica::cons(e.clone(), res))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outAbsynExpLst))
}

fn cevalAstExpListList(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynExpLstLst: &metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut info: &SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outAbsynExpLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>;
    (outCache, outAbsynExpLstLst) = (::match_deref::match_deref! { match inAbsynExpLstLst {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache;
            (cache, metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: es } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut res: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>;
            (cache, _) = cevalAstExpList(cache, env.clone(), metamodelica::AsArg::as_arg(&e), r#impl, msg.clone(), info)?;
            (cache, res) = cevalAstExpListList(cache, env, es, r#impl, msg, info)?;
            (cache, metamodelica::cons(e.clone(), res))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outAbsynExpLstLst))
}

pub(crate) fn cevalAstElt(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inElement: &metamodelica::Ref<Absyn::Element>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
) -> Result<(FCore::Cache, metamodelica::Ref<Absyn::Element>)> {
    let mut outCache: FCore::Cache;
    let mut outElement: metamodelica::Ref<Absyn::Element>;
    (outCache, outElement) = (::match_deref::match_deref! { match inElement {
        Deref @ Absyn::Element::ELEMENT { finalPrefix: f, redeclareKeywords: r, innerOuter: io, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: attr, typeSpec: tp, components: citems }, info: info @ SourceInfo { .. }, constrainClass: c } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut citems_1: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
            (cache, citems_1) = cevalAstCitems(cache, env, metamodelica::AsArg::as_arg(&citems), r#impl, msg, metamodelica::AsArg::as_arg(&info))?;
            (cache, metamodelica::Ref::new(Absyn::Element::ELEMENT { finalPrefix: f.clone(), redeclareKeywords: r.clone(), innerOuter: io.clone(), specification: metamodelica::Ref::new(Absyn::ElementSpec::COMPONENTS { attributes: attr.clone(), typeSpec: tp.clone(), components: citems_1 }), info: info.clone(), constrainClass: c.clone() }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outElement))
}

fn cevalAstCitems(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynComponentItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut info: &SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outAbsynComponentItemLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    (outCache, outAbsynComponentItemLst) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inAbsynComponentItemLst, inBoolean, inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: id, arrayDim: ad, modification: modopt }, condition: cond, comment: cmt }, tail: xs }, r#impl, msg) => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
                    let mut modopt_1: Option<metamodelica::Ref<Absyn::Modification>>;
                    let mut ad_1: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut cache = (*cache).clone();
                    (cache, res) = cevalAstCitems(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&xs), r#impl.clone(), msg.clone(), info)?;
                    (cache, modopt_1) = cevalAstModopt(cache.clone(), env.clone(), modopt.clone(), r#impl.clone(), msg.clone(), info)?;
                    (cache, ad_1) = cevalAstArraydim(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ad), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::cons(metamodelica::Ref::new(Absyn::ComponentItem { component: Absyn::Component { name: id.clone(), arrayDim: ad_1.clone(), modification: modopt_1.clone() }, condition: cond.clone(), comment: cmt.clone() }), res.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: x, tail: xs }, r#impl, msg) => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
                    let mut cache = (*cache).clone();
                    (cache, res) = cevalAstCitems(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&xs), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::cons(x.clone(), res.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outAbsynComponentItemLst))
}

fn cevalAstModopt(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynModificationOption: Option<metamodelica::Ref<Absyn::Modification>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, Option<metamodelica::Ref<Absyn::Modification>>)> {
    let mut outCache: FCore::Cache;
    let mut outAbsynModificationOption: Option<metamodelica::Ref<Absyn::Modification>>;
    (outCache, outAbsynModificationOption) = (::match_deref::match_deref! { match &(inAbsynModificationOption) {
        Some(r#mod) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut res: metamodelica::Ref<Absyn::Modification>;
            (cache, res) = cevalAstModification(cache, env, metamodelica::AsArg::as_arg(&r#mod), r#impl, msg, info)?;
            (cache, Some(res))
        },
        None => {
            let mut cache = inCache;
            (cache, None)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outAbsynModificationOption))
}

fn cevalAstModification(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inModification: &metamodelica::Ref<Absyn::Modification>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<Absyn::Modification>)> {
    let mut outCache: FCore::Cache;
    let mut outModification: metamodelica::Ref<Absyn::Modification>;
    (outCache, outModification) = (::match_deref::match_deref! { match inModification {
        Deref @ Absyn::Modification { elementArgLst: eltargs, eqMod: Deref @ Absyn::EqMod::EQMOD { exp: e, info: info2 } } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut e_1: metamodelica::Ref<Absyn::Exp>;
            let mut eltargs_1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
            (cache, e_1) = cevalAstExp(cache, env.clone(), e.clone(), r#impl, msg.clone(), info)?;
            (cache, eltargs_1) = cevalAstEltargs(cache, env, eltargs, r#impl, msg, info)?;
            (cache, metamodelica::Ref::new(Absyn::Modification { elementArgLst: eltargs_1, eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: e_1, info: info2.clone() }) }))
        },
        Deref @ Absyn::Modification { elementArgLst: eltargs, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut eltargs_1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
            (cache, eltargs_1) = cevalAstEltargs(cache, env, eltargs, r#impl, msg, info)?;
            (cache, metamodelica::Ref::new(Absyn::Modification { elementArgLst: eltargs_1, eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outModification))
}

fn cevalAstEltargs(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynElementArgLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>)> {
    let mut outCache: FCore::Cache;
    let mut outAbsynElementArgLst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    (outCache, outAbsynElementArgLst) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inAbsynElementArgLst, inBoolean, inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: b, eachPrefix: e, path: p, modification: Some(r#mod), comment: stropt, info: mod_info }, tail: args }, r#impl, msg) => {
                    let mut mod_1: metamodelica::Ref<Absyn::Modification>;
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let mut cache = (*cache).clone();
                    (cache, mod_1) = cevalAstModification(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&r#mod), r#impl.clone(), msg.clone(), info)?;
                    (cache, res) = cevalAstEltargs(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&args), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: b.clone(), eachPrefix: e.clone(), path: p.clone(), modification: Some(mod_1.clone()), comment: stropt.clone(), info: mod_info.clone() }), res.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: m, tail: args }, r#impl, msg) => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let mut cache = (*cache).clone();
                    (cache, res) = cevalAstEltargs(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&args), r#impl.clone(), msg.clone(), info)?;
                    Ok((cache.clone(), metamodelica::cons(m.clone(), res.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outAbsynElementArgLst))
}

fn cevalAstArraydim(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inArrayDim: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<Absyn::Subscript>>)> {
    let mut outCache: FCore::Cache;
    let mut outArrayDim: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    (outCache, outArrayDim) = (::match_deref::match_deref! { match inArrayDim {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache;
            (cache, metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::NOSUB { .. }, tail: xs } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            (cache, res) = cevalAstArraydim(cache, env, xs, r#impl, msg, info)?;
            (cache, metamodelica::cons(openmodelica_ast::Absyn::Subscript::interned_NOSUB(), res))
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e }, tail: xs } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            (cache, res) = cevalAstArraydim(cache, env.clone(), xs, r#impl, msg.clone(), info)?;
            (cache, _) = cevalAstExp(cache, env, e.clone(), r#impl, msg, info)?;
            (cache, metamodelica::cons(metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: e.clone() }), res))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outArrayDim))
}

fn cevalAstExpexpList(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpTpls: &metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut info: &SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>,
)> {
    let mut outCache: FCore::Cache;
    let mut outExpTpls: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>;
    (outCache, outExpTpls) = (::match_deref::match_deref! { match inExpTpls {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache;
            (cache, metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: (e1, e2), tail: xs } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut msg = inMsg;
            let mut e1_1: metamodelica::Ref<Absyn::Exp>;
            let mut e2_1: metamodelica::Ref<Absyn::Exp>;
            let mut res: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>;
            (cache, e1_1) = cevalAstExp(cache, env.clone(), e1.clone(), r#impl, msg.clone(), info)?;
            (cache, e2_1) = cevalAstExp(cache, env.clone(), e2.clone(), r#impl, msg.clone(), info)?;
            (cache, res) = cevalAstExpexpList(cache, env, xs, r#impl, msg, info)?;
            (cache, metamodelica::cons((e1_1, e2_1), res))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outExpTpls))
}

pub(crate) fn cevalDimension(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inDimension: &metamodelica::Ref<DAE::Dimension>,
    mut inImpl: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = (match &**inDimension {
        DAE::Dimension::DIM_INTEGER { integer: dim_int } => (
            inCache,
            metamodelica::Ref::new(Values::Value::INTEGER {
                integer: dim_int.clone(),
            }),
        ),
        DAE::Dimension::DIM_ENUM { size: dim_int, .. } => (
            inCache,
            metamodelica::Ref::new(Values::Value::INTEGER {
                integer: dim_int.clone(),
            }),
        ),
        DAE::Dimension::DIM_BOOLEAN { .. } => (inCache, metamodelica::Ref::new(Values::Value::INTEGER { integer: 2 })),
        DAE::Dimension::DIM_EXP { exp } => {
            let mut cache: FCore::Cache;
            let mut res: metamodelica::Ref<Values::Value>;
            (cache, res) = ceval(inCache, inEnv, exp.clone(), inImpl, inMsg, numIter + 1)?;
            (cache, res)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outCache, outValue))
}

fn makeReductionAllCombinations(
    mut inValMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>,
    mut rtype: Absyn::ReductionIterType,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>> {
    let mut valMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>;
    valMatrix = (match rtype {
        Absyn::ReductionIterType::COMBINE { .. } => {
            List::allCombinations(&inValMatrix, Some(100000), &(Absyn::dummyInfo.clone()))?.reverse()
        }
        Absyn::ReductionIterType::THREAD { .. } => List::transposeList(inValMatrix)?.reverse(),
    });
    Ok(valMatrix)
}
