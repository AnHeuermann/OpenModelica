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
use crate::Lookup;
use crate::PrefixUtil;
use crate::Static;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub(crate) fn binary<'__b>(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inOperator1: Absyn::Operator,
    mut inProp1: DAE::Properties,
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inProp2: DAE::Properties,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut AbExp: &'__b metamodelica::Ref<Absyn::Exp>,
    mut AbExp1: &'__b metamodelica::Ref<Absyn::Exp>,
    mut AbExp2: &'__b metamodelica::Ref<Absyn::Exp>,
    mut inImpl: bool,
    mut inPre: &'__b DAE::Prefix,
    mut inInfo: &'__b SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inOperator1, inProp1.clone(), inExp1.clone(), inProp2.clone(), inExp2.clone())) {
            (_, _, _, props1 @ DAE::Properties::PROP_TUPLE { .. }, _, DAE::Properties::PROP { .. }, _) if (!(Config::acceptMetaModelicaGrammar()?)) => {
                let mut cache: FCore::Cache;
                let mut type1: metamodelica::Ref<DAE::Type>;
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut prop: DAE::Properties;
                let ref __pa1 @ DAE::PROP { type_: ref __pa0, constFlag: _ } = (Types::propTupleFirstProp(props1.clone())?) else { return Err("pattern mismatch") };
                type1 = metamodelica::Own::own(__pa0);
                prop = metamodelica::Own::own(__pa1);
                exp = metamodelica::Ref::new(DAE::Exp::TSUB { exp: inExp1, ix: 1, ty: type1 });
                { (inCache, inEnv, inOperator1, inProp1, inExp1, inProp2, inExp2, AbExp, AbExp1, AbExp2, inImpl, inPre, inInfo) = (inCache, inEnv, inOperator1, prop, exp, inProp2, inExp2, AbExp, AbExp1, AbExp2, inImpl, inPre, inInfo); continue '__tco; }
            },
            (_, _, _, DAE::Properties::PROP { .. }, _, props2 @ DAE::Properties::PROP_TUPLE { .. }, _) if (!(Config::acceptMetaModelicaGrammar()?)) => {
                let mut cache: FCore::Cache;
                let mut type2: metamodelica::Ref<DAE::Type>;
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut prop: DAE::Properties;
                let ref __pa1 @ DAE::PROP { type_: ref __pa0, constFlag: _ } = (Types::propTupleFirstProp(props2.clone())?) else { return Err("pattern mismatch") };
                type2 = metamodelica::Own::own(__pa0);
                prop = metamodelica::Own::own(__pa1);
                exp = metamodelica::Ref::new(DAE::Exp::TSUB { exp: inExp2, ix: 1, ty: type2 });
                { (inCache, inEnv, inOperator1, inProp1, inExp1, inProp2, inExp2, AbExp, AbExp1, AbExp2, inImpl, inPre, inInfo) = (inCache, inEnv, inOperator1, inProp1, inExp1, prop, exp, AbExp, AbExp1, AbExp2, inImpl, inPre, inInfo); continue '__tco; }
            },
            (cache, env, aboper, DAE::Properties::PROP { type_: type1, constFlag: const1 }, exp1, DAE::Properties::PROP { type_: type2, constFlag: const2 }, exp2) => {
                let mut opList: metamodelica::List<(DAE::Operator, metamodelica::List<metamodelica::Ref<DAE::Type>>, metamodelica::Ref<DAE::Type>)>;
                let mut otype: metamodelica::Ref<DAE::Type>;
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut r#const: DAE::Const;
                let mut oper: DAE::Operator;
                let mut prop: DAE::Properties;
                let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
                let mut didInline: bool;
                let mut cache = (*cache).clone();
                let mut type1 = (*type1).clone();
                let mut exp1 = (*exp1).clone();
                let mut type2 = (*type2).clone();
                let mut exp2 = (*exp2).clone();
                if Types::isRecord(&(Types::arrayElementType(metamodelica::AsArg::as_arg(&type1)))) || Types::isRecord(&(Types::arrayElementType(metamodelica::AsArg::as_arg(&type2)))) {
                    (cache, exp, _, otype) = binaryUserdef(cache.clone(), env.clone(), aboper.clone(), inExp1, inExp2, type1.clone(), type2.clone(), inImpl, inPre, inInfo.clone())?;
                    functionTree = FCore::getFunctionTree(metamodelica::AsArg::as_arg(&cache));
                    (exp, _) = ExpressionSimplify::simplify1(exp)?;
                    (exp, _, didInline, _) = Inline::inlineExp(exp, (Some(functionTree), list![openmodelica_frontend_types::DAE::InlineType::BUILTIN_EARLY_INLINE, openmodelica_frontend_types::DAE::InlineType::EARLY_INLINE]), DAE::emptyElementSource().clone());
                    (exp, _) = ExpressionSimplify::condsimplify(didInline, exp)?;
                    r#const = Types::constAnd(const1.clone(), const2.clone());
                    prop = DAE::Properties::PROP { type_: otype, constFlag: r#const };
                } else {
                    if Types::isBoxedType(metamodelica::AsArg::as_arg(&type1)) && Types::isBoxedType(metamodelica::AsArg::as_arg(&type2)) {
                        (exp1, type1) = Types::matchType(exp1.clone(), type1.clone(), Types::unboxedType(type1.clone())?, true)?;
                        (exp2, type2) = Types::matchType(exp2.clone(), type2.clone(), Types::unboxedType(type2.clone())?, true)?;
                    }
                    (opList, type1, exp1, type2, exp2, _, _, _, _) = operatorsBinary(aboper.clone(), type1.clone(), exp1.clone(), type2.clone(), exp2.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(deoverload(&opList, list![(exp1.clone(), type1.clone()), (exp2.clone(), type2.clone())], AbExp, inPre.clone(), inInfo)?) {
                        (__pa0, Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } }, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    oper = metamodelica::Own::own(__pa0);
                    exp1 = metamodelica::Own::own(__pa1);
                    exp2 = metamodelica::Own::own(__pa2);
                    otype = metamodelica::Own::own(__pa3);
                    r#const = Types::constAnd(const1.clone(), const2.clone());
                    exp = replaceOperatorWithFcall(AbExp, exp1.clone(), oper.clone(), Some(exp2.clone()), r#const)?;
                    (exp, _) = ExpressionSimplify::simplify(exp)?;
                    prop = DAE::Properties::PROP { type_: otype, constFlag: r#const };
                    warnUnsafeRelations(&inEnv, AbExp, r#const, metamodelica::AsArg::as_arg(&type1), metamodelica::AsArg::as_arg(&type2), exp1.clone(), exp2.clone(), &oper, inPre, inInfo);
                }
                return Ok((cache.clone(), exp, prop))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn unary(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inOperator1: Absyn::Operator,
    mut inProp1: &DAE::Properties,
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut AbExp: &metamodelica::Ref<Absyn::Exp>,
    mut AbExp1: metamodelica::Ref<Absyn::Exp>,
    mut inImpl: bool,
    mut inPre: &DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProp: DAE::Properties;
    (outCache, outExp, outProp) = 'mc: {
        let __mc_input = (
            inCache.clone(),
            inEnv.clone(),
            inOperator1,
            inProp1,
            inExp1,
            AbExp1.clone(),
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, DAE::Properties::PROP_TUPLE { .. }, exp1, _) => {
                    let mut cache: FCore::Cache;
                    let mut type1: metamodelica::Ref<DAE::Type>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let false = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    let ref __pa1 @ DAE::PROP { type_: ref __pa0, constFlag: _ } = (Types::propTupleFirstProp(inProp1.clone())?) else { return Err("pattern mismatch") };
                    type1 = metamodelica::Own::own(__pa0);
                    prop = metamodelica::Own::own(__pa1);
                    exp = metamodelica::Ref::new(DAE::Exp::TSUB { exp: exp1.clone(), ix: 1, ty: type1.clone() });
                    (cache, exp, prop) = unary(inCache.clone(), inEnv.clone(), inOperator1, &prop, exp.clone(), AbExp, AbExp1.clone(), inImpl, inPre, inInfo)?;
                    Ok((cache.clone(), exp.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, aboper, DAE::Properties::PROP { type_: type1, constFlag: r#const }, exp1, _) => {
                    let mut opList: metamodelica::List<(DAE::Operator, metamodelica::List<metamodelica::Ref<DAE::Type>>, metamodelica::Ref<DAE::Type>)>;
                    let mut otype: metamodelica::Ref<DAE::Type>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut oper: DAE::Operator;
                    let mut prop: DAE::Properties;
                    let mut exp1 = (*exp1).clone();
                    let false = (Types::isRecord(&(Types::arrayElementType(metamodelica::AsArg::as_arg(&type1))))) else { return Err("pattern mismatch") };
                    opList = operatorsUnary(aboper.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(deoverload(&opList, list![(exp1.clone(), type1.clone())], AbExp, inPre.clone(), inInfo)?) {
                        (__pa0, Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    oper = metamodelica::Own::own(__pa0);
                    exp1 = metamodelica::Own::own(__pa1);
                    otype = metamodelica::Own::own(__pa2);
                    exp = replaceOperatorWithFcall(AbExp, exp1.clone(), oper.clone(), None, r#const.clone())?;
                    prop = DAE::Properties::PROP { type_: otype.clone(), constFlag: r#const.clone() };
                    Ok((inCache.clone(), exp.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, aboper, DAE::Properties::PROP { type_: type1, constFlag: _ }, _, absexp1) => {
                    let mut str1: ArcStr;
                    let mut operNames: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut operatorEnv: FCore::Graph;
                    let mut recordEnv: FCore::Graph;
                    let mut operatorCl: metamodelica::Ref<SCode::Element>;
                    let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    path = getRecordPath(metamodelica::AsArg::as_arg(&type1))?;
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    (cache, _, recordEnv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &path, None)?;
                    str1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("'")); __mm_s.push_str(&*Dump::opSymbolCompact(aboper.clone())?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) };
                    path = AbsynUtil::joinPaths(path.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: str1.clone() }))?;
                    (cache, operatorCl, operatorEnv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), &recordEnv, &path, None)?;
                    let true = (SCodeUtil::isOperator(&operatorCl)) else { return Err("pattern mismatch") };
                    operNames = AbsynToSCode::getListofQualOperatorFuncsfromOperator(&operatorCl)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsListInEnv(cache.clone(), operatorEnv.clone(), &operNames, inInfo, metamodelica::nil())?) {
                        (__pa0, __pa1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    types = metamodelica::Own::own(__pa1);
                    let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Static::elabCallArgs3(cache.clone(), env.clone(), types.clone(), path.clone(), &(list![absexp1.clone()]), &(metamodelica::nil()), &(metamodelica::nil()), inImpl, inPre.clone(), inInfo.clone())?) {
                        (__pa2, Some((__pa3, __pa4))) => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa2);
                    exp = metamodelica::Own::own(__pa3);
                    prop = metamodelica::Own::own(__pa4);
                    Ok((cache.clone(), exp.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outProp))
}

pub(crate) fn string(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp1: &metamodelica::Ref<Absyn::Exp>,
    mut inImpl: bool,
    mut inDoVect: bool,
    mut inPre: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProp: DAE::Properties;
    (outCache, outExp, outProp) = (::match_deref::match_deref! { match inExp1 {
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "String", subscripts: _ }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: exp1, tail: restargs }, argNames: nargs }, .. } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut str1: ArcStr;
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut operNames: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            let mut recordEnv: FCore::Graph;
            let mut operatorEnv: FCore::Graph;
            let mut operatorCl: metamodelica::Ref<SCode::Element>;
            let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut prop: DAE::Properties;
            let mut type1: metamodelica::Ref<DAE::Type>;
            let mut daeExp: metamodelica::Ref<DAE::Exp>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Static::elabExp(cache, env.clone(), exp1.clone(), inImpl, inDoVect, inPre.clone(), inInfo.clone())?) {
                (__pa0, _, DAE::Properties::PROP { type_: __pa1, constFlag: _ }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            type1 = metamodelica::Own::own(__pa1);
            path = getRecordPath(&type1)?;
            path = AbsynUtil::makeFullyQualified(path);
            (cache, _, recordEnv) = Lookup::lookupClass(&cache, &env, &path, None)?;
            str1 = literal!("'String'");
            path = AbsynUtil::joinPaths(path, metamodelica::Ref::new(Absyn::Path::IDENT { name: str1 }))?;
            (cache, operatorCl, operatorEnv) = Lookup::lookupClass(&cache, &recordEnv, &path, None)?;
            let true = (SCodeUtil::isOperator(&operatorCl)) else { return Err("pattern mismatch") };
            operNames = AbsynToSCode::getListofQualOperatorFuncsfromOperator(&operatorCl)?;
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsListInEnv(cache, operatorEnv, &operNames, &inInfo, metamodelica::nil())?) {
                (__pa2, __pa3 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => (__pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa2);
            types = metamodelica::Own::own(__pa3);
            let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(Static::elabCallArgs3(cache, env, types, path, &(metamodelica::cons(exp1.clone(), restargs.clone())), metamodelica::AsArg::as_arg(&nargs), &(metamodelica::nil()), inImpl, inPre, inInfo)?) {
                (__pa4, Some((__pa5, __pa6))) => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa4);
            daeExp = metamodelica::Own::own(__pa5);
            prop = metamodelica::Own::own(__pa6);
            (cache, daeExp, prop)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProp))
}

pub(crate) fn elabArglist(
    mut inTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inArgs: &metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>)>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
)> {
    let mut outArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (outArgs, outTypes) = (::match_deref::match_deref! { match (inTypes, inArgs) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            (metamodelica::nil(), metamodelica::nil())
        },
        (Deref @ metamodelica::ListNode::Cons { head: pt, tail: pts }, Deref @ metamodelica::ListNode::Cons { head: (arg, atype), tail: args }) => {
            let mut arg_1: metamodelica::Ref<DAE::Exp>;
            let mut atype_1: metamodelica::Ref<DAE::Type>;
            let mut args_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut atypes_1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            (arg_1, atype_1) = Types::matchType(arg.clone(), atype.clone(), pt.clone(), false)?;
            (args_1, atypes_1) = elabArglist(pts, args)?;
            (metamodelica::cons(arg_1, args_1), metamodelica::cons(atype_1, atypes_1))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outArgs, outTypes))
}

pub(crate) fn initCache() -> () {
    {
        let __v = (
            crate::OperatorOverloading::AvlTreePathPathEnv::Tree::interned_EMPTY(),
            crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::interned_EMPTY(),
        );
        crate::Globals::operatorOverloadingCache.with(|__root| *__root.borrow_mut() = __v)
    };
    ()
}

/* We have these as constants instead of function calls as done previously
 * because it takes a long time to generate these types over and over again.
 * The types are a bit hard to read, but they are simply 1 through 9-dimensional
 * arrays of the basic types. */
thread_local! { static __intarrtypes_TLS: metamodelica::List<metamodelica::Ref<DAE::Type>> = list![metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] })]; }
pub(crate) fn intarrtypes() -> metamodelica::List<metamodelica::Ref<DAE::Type>> {
    __intarrtypes_TLS.with(|__t| __t.clone())
}

thread_local! { static __realarrtypes_TLS: metamodelica::List<metamodelica::Ref<DAE::Type>> = list![metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] })]; }
pub(crate) fn realarrtypes() -> metamodelica::List<metamodelica::Ref<DAE::Type>> {
    __realarrtypes_TLS.with(|__t| __t.clone())
}

thread_local! { static __boolarrtypes_TLS: metamodelica::List<metamodelica::Ref<DAE::Type>> = list![metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] })]; }
pub(crate) fn boolarrtypes() -> metamodelica::List<metamodelica::Ref<DAE::Type>> {
    __boolarrtypes_TLS.with(|__t| __t.clone())
}

thread_local! { static __stringarrtypes_TLS: metamodelica::List<metamodelica::Ref<DAE::Type>> = list![metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] })]; }
pub(crate) fn stringarrtypes() -> metamodelica::List<metamodelica::Ref<DAE::Type>> {
    __stringarrtypes_TLS.with(|__t| __t.clone())
}

/* Simply a list of 9 of that basic type; used to match with the array types */
thread_local! { static __inttypes_TLS: metamodelica::List<metamodelica::Ref<DAE::Type>> = list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()]; }
pub(crate) fn inttypes() -> metamodelica::List<metamodelica::Ref<DAE::Type>> {
    __inttypes_TLS.with(|__t| __t.clone())
}

thread_local! { static __realtypes_TLS: metamodelica::List<metamodelica::Ref<DAE::Type>> = list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()]; }
pub(crate) fn realtypes() -> metamodelica::List<metamodelica::Ref<DAE::Type>> {
    __realtypes_TLS.with(|__t| __t.clone())
}

thread_local! { static __stringtypes_TLS: metamodelica::List<metamodelica::Ref<DAE::Type>> = list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()]; }
pub(crate) fn stringtypes() -> metamodelica::List<metamodelica::Ref<DAE::Type>> {
    __stringtypes_TLS.with(|__t| __t.clone())
}

fn deoverloadBinaryUserdefNoConstructor(
    mut inTypeList: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inLhs: &metamodelica::Ref<DAE::Exp>,
    mut inRhs: &metamodelica::Ref<DAE::Exp>,
    mut lhsType: &metamodelica::Ref<DAE::Type>,
    mut rhsType: &metamodelica::Ref<DAE::Type>,
    mut inAcc: metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>,
) -> Result<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>> {
    let mut outExps: metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>;
    outExps = 'mc: {
        let __mc_input = (&**inTypeList, inAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_FUNCTION { path, funcResultType: ty, functionAttributes: attr, funcArg: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { ty: ty1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { ty: ty2, .. }, tail: restArgs } } }, tail: types }, acc) => {
                    let mut daeExp: metamodelica::Ref<DAE::Exp>;
                    let mut lhs: metamodelica::Ref<DAE::Exp>;
                    let mut rhs: metamodelica::Ref<DAE::Exp>;
                    let mut tpl: (metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>);
                    let mut acc = (*acc).clone();
                    (lhs, _) = Types::matchType(inLhs.clone(), lhsType.clone(), ty1.clone(), false)?;
                    (rhs, _) = Types::matchType(inRhs.clone(), rhsType.clone(), ty2.clone(), false)?;
                    daeExp = makeCallFillRestDefaults(path.clone(), list![lhs.clone(), rhs.clone()], restArgs.clone(), Types::makeCallAttr(ty.clone(), metamodelica::AsArg::as_arg(&attr)))?;
                    tpl = (daeExp.clone(), overloadFoldType(ty1.clone(), ty2.clone(), ty.clone()));
                    acc = deoverloadBinaryUserdefNoConstructor(metamodelica::AsArg::as_arg(&types), inLhs, inRhs, lhsType, rhsType, metamodelica::cons(tpl.clone(), acc.clone()))?;
                    Ok(acc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: types }, _) => {
                    let mut acc: metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>;
                    acc = deoverloadBinaryUserdefNoConstructor(metamodelica::AsArg::as_arg(&types), inLhs, inRhs, lhsType, rhsType, inAcc.clone())?;
                    Ok(acc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExps)
}

fn overloadFoldType(
    mut inType1: metamodelica::Ref<DAE::Type>,
    mut inType2: metamodelica::Ref<DAE::Type>,
    mut inType3: metamodelica::Ref<DAE::Type>,
) -> Option<metamodelica::Ref<DAE::Type>> {
    let mut optType: Option<metamodelica::Ref<DAE::Type>>;
    optType = if (Types::equivtypesOrRecordSubtypeOf(inType1.clone(), inType2)
        && Types::equivtypesOrRecordSubtypeOf(inType1.clone(), inType3))
    {
        Some(inType1)
    } else {
        None
    };
    optType
}

fn deoverloadBinaryUserdefNoConstructorListLhs<'__b>(
    mut types: &'__b metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inLhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inRhs: &'__b metamodelica::Ref<DAE::Exp>,
    mut rhsType: &'__b metamodelica::Ref<DAE::Type>,
    mut inAcc: metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>,
) -> Result<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inLhs, inAcc.clone())) {
            (Deref @ metamodelica::ListNode::Cons { head: lhs, tail: rest }, acc) => {
                let mut acc = (*acc).clone();
                acc = deoverloadBinaryUserdefNoConstructor(types, metamodelica::AsArg::as_arg(&lhs), inRhs, &(Expression::r#typeof(lhs.clone())?), rhsType, acc.clone())?;
                { (types, inLhs, inRhs, rhsType, inAcc) = (types, rest.clone(), inRhs, rhsType, acc.clone()); continue '__tco; }
            },
            _ => {
                return Ok(inAcc)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn deoverloadBinaryUserdefNoConstructorListRhs<'__b>(
    mut types: &'__b metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inLhs: &'__b metamodelica::Ref<DAE::Exp>,
    mut inRhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut lhsType: &'__b metamodelica::Ref<DAE::Type>,
    mut inAcc: metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>,
) -> Result<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inRhs, inAcc.clone())) {
            (Deref @ metamodelica::ListNode::Cons { head: rhs, tail: rest }, acc) => {
                let mut acc = (*acc).clone();
                acc = deoverloadBinaryUserdefNoConstructor(types, inLhs, metamodelica::AsArg::as_arg(&rhs), lhsType, &(Expression::r#typeof(rhs.clone())?), acc.clone())?;
                { (types, inLhs, inRhs, lhsType, inAcc) = (types, inLhs, rest.clone(), lhsType, acc.clone()); continue '__tco; }
            },
            _ => {
                return Ok(inAcc)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn deoverloadUnaryUserdefNoConstructor(
    mut inTypeList: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inAcc: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExps = 'mc: {
        let __mc_input = (&**inTypeList, inAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_FUNCTION { path, funcResultType: ty, functionAttributes: attr, funcArg: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { ty: ty1, .. }, tail: restArgs } }, tail: types }, acc) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut daeExp: metamodelica::Ref<DAE::Exp>;
                    let mut acc = (*acc).clone();
                    (exp, _) = Types::matchType(inExp.clone(), inType.clone(), ty1.clone(), false)?;
                    daeExp = makeCallFillRestDefaults(path.clone(), list![exp.clone()], restArgs.clone(), Types::makeCallAttr(ty.clone(), metamodelica::AsArg::as_arg(&attr)))?;
                    acc = deoverloadUnaryUserdefNoConstructor(metamodelica::AsArg::as_arg(&types), inExp, metamodelica::AsArg::as_arg(&ty), metamodelica::cons(daeExp.clone(), acc.clone()))?;
                    Ok(acc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: types }, _) => {
                    let mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    acc = deoverloadUnaryUserdefNoConstructor(metamodelica::AsArg::as_arg(&types), inExp, inType, inAcc.clone())?;
                    Ok(acc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExps)
}

fn binaryUserdef(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inOper: Absyn::Operator,
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inType1: metamodelica::Ref<DAE::Type>,
    mut inType2: metamodelica::Ref<DAE::Type>,
    mut r#impl: bool,
    mut pre: &DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Exp>,
    Option<metamodelica::Ref<DAE::Type>>,
    metamodelica::Ref<DAE::Type>,
)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut foldType: Option<metamodelica::Ref<DAE::Type>>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    (outCache, outExp, foldType, outType) = (::match_deref::match_deref! { match &((inCache, inEnv, inOper, inExp1.clone(), inExp2.clone(), inType1.clone(), inType2.clone())) {
        (cache, env, op, exp1, exp2, type1, type2) => {
            let mut bool1: bool;
            let mut bool2: bool;
            let mut opStr: ArcStr;
            let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut types1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut types2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut daeExp: metamodelica::Ref<DAE::Exp>;
            let mut exps: metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>;
            let mut cache = (*cache).clone();
            bool1 = Types::arrayType(metamodelica::AsArg::as_arg(&type1));
            bool2 = Types::arrayType(metamodelica::AsArg::as_arg(&type2));
            if bool1 && bool2 && AbsynUtil::opIsElementWise(op.clone()) {
                types = metamodelica::nil();
            } else {
                opStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("'")); __mm_s.push_str(&*Dump::opSymbolCompact(op.clone())?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) };
                (cache, types1) = getOperatorFuncsOrEmpty(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &(list![type1.clone()]), &opStr, &info, &(metamodelica::nil()))?;
                (cache, types2) = getOperatorFuncsOrEmpty(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &(list![type2.clone()]), &opStr, &info, &(metamodelica::nil()))?;
                types = List::union(&types1, &types2);
                types = List::select1(types, (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Type>, __a1: SourceInfo| isOperatorBinaryFunctionOrWarn(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>, SourceInfo) -> Result<bool> + 'static>), info.clone())?;
            }
            exps = deoverloadBinaryUserdefNoConstructor(&types, metamodelica::AsArg::as_arg(&exp1), metamodelica::AsArg::as_arg(&exp2), metamodelica::AsArg::as_arg(&type1), metamodelica::AsArg::as_arg(&type2), metamodelica::nil())?;
            (cache, exps) = binaryCastConstructor(cache.clone(), metamodelica::AsArg::as_arg(&env), &inExp1, &inExp2, inType1.clone(), inType2.clone(), exps, types, &info)?;
            (cache, exps) = binaryUserdefArray(cache.clone(), env.clone(), exps, bool1 || bool2, inOper, inExp1, inExp2, inType1, inType2, r#impl, pre, info)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exps) {
                Deref @ metamodelica::ListNode::Cons { head: (__pa0, __pa1), tail: Deref @ metamodelica::ListNode::Nil } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            daeExp = metamodelica::Own::own(__pa0);
            foldType = metamodelica::Own::own(__pa1);
            (cache.clone(), daeExp.clone(), foldType, Expression::r#typeof(daeExp)?)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outExp, foldType, outType))
}

fn binaryUserdefArray(
    mut inCache: FCore::Cache,
    mut env: FCore::Graph,
    mut inExps: metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>,
    mut isArray: bool,
    mut inOper: Absyn::Operator,
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inType1: metamodelica::Ref<DAE::Type>,
    mut inType2: metamodelica::Ref<DAE::Type>,
    mut r#impl: bool,
    mut pre: &DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut cache: FCore::Cache;
    let mut exps: metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>;
    (cache, exps) = (::match_deref::match_deref! { match &((inExps.clone(), isArray)) {
        (Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, _) => {
            (inCache, inExps)
        },
        (Deref @ metamodelica::ListNode::Nil, true) => {
            let mut isRelation: bool;
            let mut isVector1: bool;
            let mut isVector2: bool;
            let mut isScalar1: bool;
            let mut isScalar2: bool;
            let mut isMatrix1: bool;
            let mut isMatrix2: bool;
            isRelation = listMember(inOper, list![openmodelica_ast::Absyn::Operator::LESS, openmodelica_ast::Absyn::Operator::LESSEQ, openmodelica_ast::Absyn::Operator::GREATER, openmodelica_ast::Absyn::Operator::GREATEREQ, openmodelica_ast::Absyn::Operator::EQUAL, openmodelica_ast::Absyn::Operator::NEQUAL]);
            Error::assertionOrAddSourceMessage(!(isRelation), &(Error::COMPILER_ERROR.clone()), list![literal!("Not supporting overloading of relation array operations")], &info)?;
            isScalar1 = !(Types::arrayType(&inType1));
            isScalar2 = !(Types::arrayType(&inType2));
            isVector1 = Types::isArray1D(&inType1);
            isVector2 = Types::isArray1D(&inType2);
            isMatrix1 = Types::isArray2D(&inType1);
            isMatrix2 = Types::isArray2D(&inType2);
            (cache, exps) = binaryUserdefArray2(inCache, env, isScalar1, isVector1, isMatrix1, isScalar2, isVector2, isMatrix2, inOper, inExp1, inExp2, inType1, inType2, r#impl, pre, info)?;
            (cache, exps)
        },
        _ => {
            errorMultipleValid(List::map(inExps, &fnptr!(Util::tuple21, _))?, &info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cache, exps))
}

fn binaryUserdefArray2(
    mut inCache: FCore::Cache,
    mut env: FCore::Graph,
    mut isScalar1: bool,
    mut isVector1: bool,
    mut isMatrix1: bool,
    mut isScalar2: bool,
    mut isVector2: bool,
    mut isMatrix2: bool,
    mut inOper: Absyn::Operator,
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inType1: metamodelica::Ref<DAE::Type>,
    mut inType2: metamodelica::Ref<DAE::Type>,
    mut r#impl: bool,
    mut pre: &DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut cache: FCore::Cache;
    let mut exps: metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>;
    (cache, exps) = (match (
        inCache, isScalar1, isVector1, isMatrix1, isScalar2, isVector2, isMatrix2, inOper,
    ) {
        (mut __esc_cache, false, _, _, true, _, _, _) => {
            cache = __esc_cache.clone();
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut cr: metamodelica::Ref<DAE::Exp>;
            let mut newType1: metamodelica::Ref<DAE::Type>;
            let mut resType: metamodelica::Ref<DAE::Type>;
            let mut dim1: metamodelica::Ref<DAE::Dimension>;
            let mut foldName: ArcStr;
            let mut resultName: ArcStr;
            let mut iterName: ArcStr;
            let mut op: Absyn::Operator;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inType1) {
                Deref @ DAE::Type::T_ARRAY { ty: __pa0, dims: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            newType1 = metamodelica::Own::own(__pa0);
            dim1 = metamodelica::Own::own(__pa1);
            op = Util::assoc(
                inOper,
                list![
                    (
                        openmodelica_ast::Absyn::Operator::ADD_EW,
                        openmodelica_ast::Absyn::Operator::ADD_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::SUB_EW,
                        openmodelica_ast::Absyn::Operator::SUB_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::MUL,
                        openmodelica_ast::Absyn::Operator::MUL_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::MUL_EW,
                        openmodelica_ast::Absyn::Operator::MUL_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::DIV,
                        openmodelica_ast::Absyn::Operator::DIV_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::DIV_EW,
                        openmodelica_ast::Absyn::Operator::DIV_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::POW_EW,
                        openmodelica_ast::Absyn::Operator::POW_EW
                    )
                ],
            )?;
            iterName = Util::getTempVariableIndex();
            foldName = Util::getTempVariableIndex();
            resultName = Util::getTempVariableIndex();
            cr = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: iterName.clone(),
                    identType: newType1.clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: newType1.clone(),
            });
            (cache, exp, _, resType) =
                binaryUserdef(cache, env, op, cr, inExp2, newType1.clone(), inType2, r#impl, pre, info)?;
            resType = Types::liftArray(resType, dim1);
            exp = metamodelica::Ref::new(DAE::Exp::REDUCTION {
                reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("array"),
                    }),
                    iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE,
                    exprType: resType,
                    defaultValue: None,
                    foldName: foldName,
                    resultName: resultName,
                    foldExp: None,
                }),
                expr: exp,
                iterators: metamodelica::cons(
                    metamodelica::Ref::new(DAE::ReductionIterator {
                        id: iterName,
                        exp: inExp1,
                        guardExp: None,
                        ty: newType1,
                    }),
                    metamodelica::nil(),
                ),
            });
            (cache, list![(exp, None)])
        }
        (mut __esc_cache, true, _, _, false, _, _, _) => {
            cache = __esc_cache.clone();
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut cr: metamodelica::Ref<DAE::Exp>;
            let mut newType2: metamodelica::Ref<DAE::Type>;
            let mut resType: metamodelica::Ref<DAE::Type>;
            let mut dim2: metamodelica::Ref<DAE::Dimension>;
            let mut foldName: ArcStr;
            let mut resultName: ArcStr;
            let mut iterName: ArcStr;
            let mut op: Absyn::Operator;
            op = Util::assoc(
                inOper,
                list![
                    (
                        openmodelica_ast::Absyn::Operator::ADD_EW,
                        openmodelica_ast::Absyn::Operator::ADD_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::SUB_EW,
                        openmodelica_ast::Absyn::Operator::SUB_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::MUL,
                        openmodelica_ast::Absyn::Operator::MUL_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::MUL_EW,
                        openmodelica_ast::Absyn::Operator::MUL_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::DIV_EW,
                        openmodelica_ast::Absyn::Operator::DIV_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::POW_EW,
                        openmodelica_ast::Absyn::Operator::POW_EW
                    )
                ],
            )?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inType2) {
                Deref @ DAE::Type::T_ARRAY { ty: __pa0, dims: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            newType2 = metamodelica::Own::own(__pa0);
            dim2 = metamodelica::Own::own(__pa1);
            iterName = Util::getTempVariableIndex();
            foldName = Util::getTempVariableIndex();
            resultName = Util::getTempVariableIndex();
            cr = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: iterName.clone(),
                    identType: newType2.clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: newType2.clone(),
            });
            (cache, exp, _, resType) =
                binaryUserdef(cache, env, op, inExp1, cr, inType1, newType2.clone(), r#impl, pre, info)?;
            resType = metamodelica::Ref::new(DAE::Type::T_ARRAY {
                ty: resType,
                dims: list![dim2],
            });
            exp = metamodelica::Ref::new(DAE::Exp::REDUCTION {
                reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("array"),
                    }),
                    iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE,
                    exprType: resType,
                    defaultValue: None,
                    foldName: foldName,
                    resultName: resultName,
                    foldExp: None,
                }),
                expr: exp,
                iterators: metamodelica::cons(
                    metamodelica::Ref::new(DAE::ReductionIterator {
                        id: iterName,
                        exp: inExp2,
                        guardExp: None,
                        ty: newType2,
                    }),
                    metamodelica::nil(),
                ),
            });
            (cache, list![(exp, None)])
        }
        (_, _, true, _, _, true, _, Absyn::Operator::MUL { .. }) => return Err("fail"),
        (_, _, true, _, _, _, true, Absyn::Operator::MUL { .. }) => return Err("fail"),
        (mut __esc_cache, _, _, true, _, true, _, Absyn::Operator::MUL { .. }) => {
            cache = __esc_cache.clone();
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut cr: metamodelica::Ref<DAE::Exp>;
            let mut cr1: metamodelica::Ref<DAE::Exp>;
            let mut cr2: metamodelica::Ref<DAE::Exp>;
            let mut cr3: metamodelica::Ref<DAE::Exp>;
            let mut cr4: metamodelica::Ref<DAE::Exp>;
            let mut foldExp: metamodelica::Ref<DAE::Exp>;
            let mut newType1: metamodelica::Ref<DAE::Type>;
            let mut newType2: metamodelica::Ref<DAE::Type>;
            let mut resType: metamodelica::Ref<DAE::Type>;
            let mut newType1_1: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut dim2: metamodelica::Ref<DAE::Dimension>;
            let mut dim1_1: metamodelica::Ref<DAE::Dimension>;
            let mut dim1_2: metamodelica::Ref<DAE::Dimension>;
            let mut iter: metamodelica::Ref<DAE::ReductionIterator>;
            let mut iter1: metamodelica::Ref<DAE::ReductionIterator>;
            let mut iter2: metamodelica::Ref<DAE::ReductionIterator>;
            let mut foldName1: ArcStr;
            let mut resultName1: ArcStr;
            let mut foldName2: ArcStr;
            let mut resultName2: ArcStr;
            let mut iterName: ArcStr;
            let mut iterName1: ArcStr;
            let mut iterName2: ArcStr;
            let mut zeroConstructor: Option<metamodelica::Ref<Values::Value>>;
            let mut zeroTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inType1) {
                Deref @ DAE::Type::T_ARRAY { ty: __pa0, dims: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            newType1_1 = metamodelica::Own::own(__pa0);
            dim1_1 = metamodelica::Own::own(__pa1);
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(newType1_1.clone()) {
                Deref @ DAE::Type::T_ARRAY { ty: __pa3, dims: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            newType1 = metamodelica::Own::own(__pa3);
            dim1_2 = metamodelica::Own::own(__pa4);
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(inType2) {
                Deref @ DAE::Type::T_ARRAY { ty: __pa6, dims: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa6.clone(), __pa7.clone()),
                _ => return Err("pattern mismatch"),
            } };
            newType2 = metamodelica::Own::own(__pa6);
            dim2 = metamodelica::Own::own(__pa7);
            let true = (Expression::dimensionsEqual(&dim1_2, &dim2)?) else {
                return Err("pattern mismatch");
            };
            foldName1 = Util::getTempVariableIndex();
            resultName1 = Util::getTempVariableIndex();
            foldName2 = Util::getTempVariableIndex();
            resultName2 = Util::getTempVariableIndex();
            iterName = Util::getTempVariableIndex();
            iterName1 = Util::getTempVariableIndex();
            iterName2 = Util::getTempVariableIndex();
            cr = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: iterName.clone(),
                    identType: newType1_1,
                    subscriptLst: metamodelica::nil(),
                }),
                ty: newType1.clone(),
            });
            cr1 = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: iterName1.clone(),
                    identType: newType1.clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: newType1.clone(),
            });
            cr2 = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: iterName2.clone(),
                    identType: newType2.clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: newType2.clone(),
            });
            cr3 = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: foldName1.clone(),
                    identType: newType1.clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: newType1.clone(),
            });
            cr4 = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: resultName1.clone(),
                    identType: newType2.clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: newType2.clone(),
            });
            let (__pa9, __pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &(binaryUserdef(cache, env.clone(), openmodelica_ast::Absyn::Operator::ADD, cr1, cr2, newType1.clone(), newType2.clone(), r#impl, pre, info.clone())?) {
                (__pa9, __pa10, Some(__pa11), __pa12) => (__pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa9);
            exp = metamodelica::Own::own(__pa10);
            ty = metamodelica::Own::own(__pa11);
            resType = metamodelica::Own::own(__pa12);
            (cache, foldExp, _, _) = binaryUserdef(
                cache,
                env.clone(),
                openmodelica_ast::Absyn::Operator::ADD,
                cr3,
                cr4,
                ty.clone(),
                ty.clone(),
                r#impl,
                pre,
                info.clone(),
            )?;
            (cache, zeroTypes) = getOperatorFuncsOrEmpty(
                &cache,
                &env,
                &(list![ty]),
                &(literal!("'0'")),
                &info,
                &(metamodelica::nil()),
            )?;
            (cache, zeroConstructor) = getZeroConstructor(
                cache,
                env,
                List::filterMap(&zeroTypes, &getZeroConstructorExpression),
                r#impl,
                info,
            )?;
            resType = metamodelica::Ref::new(DAE::Type::T_ARRAY {
                ty: resType,
                dims: list![dim1_1],
            });
            iter = metamodelica::Ref::new(DAE::ReductionIterator {
                id: iterName1,
                exp: cr,
                guardExp: None,
                ty: newType1.clone(),
            });
            iter1 = metamodelica::Ref::new(DAE::ReductionIterator {
                id: iterName,
                exp: inExp1,
                guardExp: None,
                ty: newType1,
            });
            iter2 = metamodelica::Ref::new(DAE::ReductionIterator {
                id: iterName2,
                exp: inExp2,
                guardExp: None,
                ty: newType2,
            });
            exp = metamodelica::Ref::new(DAE::Exp::REDUCTION {
                reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sum") }),
                    iterType: openmodelica_ast::Absyn::ReductionIterType::THREAD,
                    exprType: resType.clone(),
                    defaultValue: zeroConstructor,
                    foldName: foldName1,
                    resultName: resultName1,
                    foldExp: Some(foldExp),
                }),
                expr: exp,
                iterators: metamodelica::cons(iter, metamodelica::cons(iter2, metamodelica::nil())),
            });
            exp = metamodelica::Ref::new(DAE::Exp::REDUCTION {
                reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("array"),
                    }),
                    iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE,
                    exprType: resType,
                    defaultValue: None,
                    foldName: foldName2,
                    resultName: resultName2,
                    foldExp: None,
                }),
                expr: exp,
                iterators: metamodelica::cons(iter1, metamodelica::nil()),
            });
            (cache, list![(exp, None)])
        }
        (mut __esc_cache, _, _, true, _, _, true, Absyn::Operator::MUL { .. }) => {
            cache = __esc_cache.clone();
            let mut mulExp: metamodelica::Ref<DAE::Exp>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut cr1: metamodelica::Ref<DAE::Exp>;
            let mut cr2: metamodelica::Ref<DAE::Exp>;
            let mut cr3: metamodelica::Ref<DAE::Exp>;
            let mut cr4: metamodelica::Ref<DAE::Exp>;
            let mut cr5: metamodelica::Ref<DAE::Exp>;
            let mut cr6: metamodelica::Ref<DAE::Exp>;
            let mut foldExp: metamodelica::Ref<DAE::Exp>;
            let mut transposed: metamodelica::Ref<DAE::Exp>;
            let mut newType1: metamodelica::Ref<DAE::Type>;
            let mut newType2: metamodelica::Ref<DAE::Type>;
            let mut newType1_1: metamodelica::Ref<DAE::Type>;
            let mut newType2_1: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut dim1_1: metamodelica::Ref<DAE::Dimension>;
            let mut dim1_2: metamodelica::Ref<DAE::Dimension>;
            let mut dim2_1: metamodelica::Ref<DAE::Dimension>;
            let mut dim2_2: metamodelica::Ref<DAE::Dimension>;
            let mut iter1: metamodelica::Ref<DAE::ReductionIterator>;
            let mut iter2: metamodelica::Ref<DAE::ReductionIterator>;
            let mut iter3: metamodelica::Ref<DAE::ReductionIterator>;
            let mut iter4: metamodelica::Ref<DAE::ReductionIterator>;
            let mut foldName: ArcStr;
            let mut resultName: ArcStr;
            let mut foldName1: ArcStr;
            let mut resultName1: ArcStr;
            let mut foldName2: ArcStr;
            let mut resultName2: ArcStr;
            let mut iterName1: ArcStr;
            let mut iterName2: ArcStr;
            let mut iterName3: ArcStr;
            let mut iterName4: ArcStr;
            let mut zeroConstructor: Option<metamodelica::Ref<Values::Value>>;
            let mut zeroTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inType1) {
                Deref @ DAE::Type::T_ARRAY { ty: __pa0, dims: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            newType1_1 = metamodelica::Own::own(__pa0);
            dim1_1 = metamodelica::Own::own(__pa1);
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(newType1_1.clone()) {
                Deref @ DAE::Type::T_ARRAY { ty: __pa3, dims: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            newType1 = metamodelica::Own::own(__pa3);
            dim1_2 = metamodelica::Own::own(__pa4);
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(inType2) {
                Deref @ DAE::Type::T_ARRAY { ty: __pa6, dims: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa6.clone(), __pa7.clone()),
                _ => return Err("pattern mismatch"),
            } };
            newType2_1 = metamodelica::Own::own(__pa6);
            dim2_1 = metamodelica::Own::own(__pa7);
            let (__pa9, __pa10) = ::match_deref::match_deref! { match &(newType2_1.clone()) {
                Deref @ DAE::Type::T_ARRAY { ty: __pa9, dims: Deref @ metamodelica::ListNode::Cons { head: __pa10, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa9.clone(), __pa10.clone()),
                _ => return Err("pattern mismatch"),
            } };
            newType2 = metamodelica::Own::own(__pa9);
            dim2_2 = metamodelica::Own::own(__pa10);
            let true = (Expression::dimensionsEqual(&dim1_2, &dim2_1)?) else {
                return Err("pattern mismatch");
            };
            transposed = Expression::makePureBuiltinCall(
                literal!("transpose"),
                list![inExp2],
                Types::liftArray(Types::liftArray(newType2.clone(), dim2_1), dim2_2.clone()),
            );
            iterName1 = Util::getTempVariableIndex();
            iterName2 = Util::getTempVariableIndex();
            iterName3 = Util::getTempVariableIndex();
            iterName4 = Util::getTempVariableIndex();
            foldName1 = Util::getTempVariableIndex();
            resultName1 = Util::getTempVariableIndex();
            foldName2 = Util::getTempVariableIndex();
            resultName2 = Util::getTempVariableIndex();
            foldName = Util::getTempVariableIndex();
            resultName = Util::getTempVariableIndex();
            cr1 = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: iterName1.clone(),
                    identType: newType1_1.clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: newType1_1.clone(),
            });
            cr2 = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: iterName2.clone(),
                    identType: newType2_1.clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: newType2_1.clone(),
            });
            cr3 = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: iterName3.clone(),
                    identType: newType1.clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: newType1.clone(),
            });
            cr4 = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: iterName4.clone(),
                    identType: newType2.clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: newType2.clone(),
            });
            (cache, mulExp, _, ty) = binaryUserdef(
                cache,
                env.clone(),
                openmodelica_ast::Absyn::Operator::MUL,
                cr3,
                cr4,
                newType1,
                newType2,
                r#impl,
                pre,
                info.clone(),
            )?;
            cr5 = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: foldName.clone(),
                    identType: ty.clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: ty.clone(),
            });
            cr6 = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: resultName.clone(),
                    identType: ty.clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: ty.clone(),
            });
            let (__pa12, __pa13, __pa14) = ::match_deref::match_deref! { match &(binaryUserdef(cache, env.clone(), openmodelica_ast::Absyn::Operator::ADD, cr5, cr6, ty.clone(), ty, r#impl, pre, info.clone())?) {
                (__pa12, __pa13, Some(__pa14), _) => (__pa12.clone(), __pa13.clone(), __pa14.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa12);
            foldExp = metamodelica::Own::own(__pa13);
            ty = metamodelica::Own::own(__pa14);
            (cache, zeroTypes) = getOperatorFuncsOrEmpty(
                &cache,
                &env,
                &(list![ty.clone()]),
                &(literal!("'0'")),
                &info,
                &(metamodelica::nil()),
            )?;
            (cache, zeroConstructor) = getZeroConstructor(
                cache,
                env,
                List::filterMap(&zeroTypes, &getZeroConstructorExpression),
                r#impl,
                info,
            )?;
            iter1 = metamodelica::Ref::new(DAE::ReductionIterator {
                id: iterName1,
                exp: inExp1,
                guardExp: None,
                ty: newType1_1.clone(),
            });
            iter2 = metamodelica::Ref::new(DAE::ReductionIterator {
                id: iterName2,
                exp: transposed,
                guardExp: None,
                ty: newType2_1.clone(),
            });
            iter3 = metamodelica::Ref::new(DAE::ReductionIterator {
                id: iterName3,
                exp: cr1,
                guardExp: None,
                ty: newType1_1,
            });
            iter4 = metamodelica::Ref::new(DAE::ReductionIterator {
                id: iterName4,
                exp: cr2,
                guardExp: None,
                ty: newType2_1,
            });
            exp = metamodelica::Ref::new(DAE::Exp::REDUCTION {
                reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sum") }),
                    iterType: openmodelica_ast::Absyn::ReductionIterType::THREAD,
                    exprType: ty.clone(),
                    defaultValue: zeroConstructor,
                    foldName: foldName,
                    resultName: resultName,
                    foldExp: Some(foldExp),
                }),
                expr: mulExp,
                iterators: metamodelica::cons(iter3, metamodelica::cons(iter4, metamodelica::nil())),
            });
            ty = Types::liftArray(ty, dim2_2);
            exp = metamodelica::Ref::new(DAE::Exp::REDUCTION {
                reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("array"),
                    }),
                    iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE,
                    exprType: ty.clone(),
                    defaultValue: None,
                    foldName: foldName2,
                    resultName: resultName2,
                    foldExp: None,
                }),
                expr: exp,
                iterators: metamodelica::cons(iter2, metamodelica::nil()),
            });
            ty = Types::liftArray(ty, dim1_1);
            exp = metamodelica::Ref::new(DAE::Exp::REDUCTION {
                reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("array"),
                    }),
                    iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE,
                    exprType: ty,
                    defaultValue: None,
                    foldName: foldName1,
                    resultName: resultName1,
                    foldExp: None,
                }),
                expr: exp,
                iterators: metamodelica::cons(iter1, metamodelica::nil()),
            });
            (cache, list![(exp, None)])
        }
        (mut __esc_cache, false, _, _, false, _, _, _) => {
            cache = __esc_cache.clone();
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut cr1: metamodelica::Ref<DAE::Exp>;
            let mut cr2: metamodelica::Ref<DAE::Exp>;
            let mut newType1: metamodelica::Ref<DAE::Type>;
            let mut newType2: metamodelica::Ref<DAE::Type>;
            let mut resType: metamodelica::Ref<DAE::Type>;
            let mut dim1: metamodelica::Ref<DAE::Dimension>;
            let mut dim2: metamodelica::Ref<DAE::Dimension>;
            let mut iter1: metamodelica::Ref<DAE::ReductionIterator>;
            let mut iter2: metamodelica::Ref<DAE::ReductionIterator>;
            let mut foldName: ArcStr;
            let mut resultName: ArcStr;
            let mut iterName1: ArcStr;
            let mut iterName2: ArcStr;
            let mut op: Absyn::Operator;
            op = Util::assoc(
                inOper,
                list![
                    (
                        openmodelica_ast::Absyn::Operator::ADD,
                        openmodelica_ast::Absyn::Operator::ADD_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::ADD_EW,
                        openmodelica_ast::Absyn::Operator::ADD_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::SUB,
                        openmodelica_ast::Absyn::Operator::SUB_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::SUB_EW,
                        openmodelica_ast::Absyn::Operator::SUB_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::MUL_EW,
                        openmodelica_ast::Absyn::Operator::MUL_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::DIV_EW,
                        openmodelica_ast::Absyn::Operator::DIV_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::POW_EW,
                        openmodelica_ast::Absyn::Operator::POW_EW
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::AND,
                        openmodelica_ast::Absyn::Operator::AND
                    ),
                    (
                        openmodelica_ast::Absyn::Operator::OR,
                        openmodelica_ast::Absyn::Operator::OR
                    )
                ],
            )?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inType1) {
                Deref @ DAE::Type::T_ARRAY { ty: __pa0, dims: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            newType1 = metamodelica::Own::own(__pa0);
            dim1 = metamodelica::Own::own(__pa1);
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(inType2) {
                Deref @ DAE::Type::T_ARRAY { ty: __pa3, dims: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            newType2 = metamodelica::Own::own(__pa3);
            dim2 = metamodelica::Own::own(__pa4);
            let true = (Expression::dimensionsEqual(&dim1, &dim2)?) else {
                return Err("pattern mismatch");
            };
            foldName = Util::getTempVariableIndex();
            resultName = Util::getTempVariableIndex();
            iterName1 = Util::getTempVariableIndex();
            iterName2 = Util::getTempVariableIndex();
            cr1 = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: iterName1.clone(),
                    identType: newType1.clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: newType1.clone(),
            });
            cr2 = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: iterName2.clone(),
                    identType: newType2.clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: newType2.clone(),
            });
            (cache, exp, _, resType) = binaryUserdef(
                cache,
                env,
                op,
                cr1,
                cr2,
                newType1.clone(),
                newType2.clone(),
                r#impl,
                pre,
                info,
            )?;
            resType = metamodelica::Ref::new(DAE::Type::T_ARRAY {
                ty: resType,
                dims: list![dim2],
            });
            iter1 = metamodelica::Ref::new(DAE::ReductionIterator {
                id: iterName1,
                exp: inExp1,
                guardExp: None,
                ty: newType1,
            });
            iter2 = metamodelica::Ref::new(DAE::ReductionIterator {
                id: iterName2,
                exp: inExp2,
                guardExp: None,
                ty: newType2,
            });
            exp = metamodelica::Ref::new(DAE::Exp::REDUCTION {
                reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("array"),
                    }),
                    iterType: openmodelica_ast::Absyn::ReductionIterType::THREAD,
                    exprType: resType,
                    defaultValue: None,
                    foldName: foldName,
                    resultName: resultName,
                    foldExp: None,
                }),
                expr: exp,
                iterators: metamodelica::cons(iter1, metamodelica::cons(iter2, metamodelica::nil())),
            });
            (cache, list![(exp, None)])
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((cache, exps))
}

fn operatorsBinary(
    mut inOperator: Absyn::Operator,
    mut t1: metamodelica::Ref<DAE::Type>,
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut t2: metamodelica::Ref<DAE::Type>,
    mut e2: metamodelica::Ref<DAE::Exp>,
) -> Result<(
    metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Exp>,
)> {
    let mut ops: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )>;
    let mut t1: metamodelica::Ref<DAE::Type> = t1;
    let mut e1: metamodelica::Ref<DAE::Exp> = e1;
    let mut t2: metamodelica::Ref<DAE::Type> = t2;
    let mut e2: metamodelica::Ref<DAE::Exp> = e2;
    let mut oty1: metamodelica::Ref<DAE::Type> = t1.clone();
    let mut oe1: metamodelica::Ref<DAE::Exp> = e1.clone();
    let mut oty2: metamodelica::Ref<DAE::Type> = t2.clone();
    let mut oe2: metamodelica::Ref<DAE::Exp> = e2.clone();
    let int_mul: DAE::Operator = DAE::Operator::MUL {
        ty: DAE::T_INTEGER_DEFAULT().clone(),
    };
    let real_mul: DAE::Operator = DAE::Operator::MUL {
        ty: DAE::T_REAL_DEFAULT().clone(),
    };
    let real_div: DAE::Operator = DAE::Operator::DIV {
        ty: DAE::T_REAL_DEFAULT().clone(),
    };
    let real_pow: DAE::Operator = DAE::Operator::POW {
        ty: DAE::T_REAL_DEFAULT().clone(),
    };
    let int_mul_sp: DAE::Operator = DAE::Operator::MUL_SCALAR_PRODUCT {
        ty: DAE::T_INTEGER_DEFAULT().clone(),
    };
    let real_mul_sp: DAE::Operator = DAE::Operator::MUL_SCALAR_PRODUCT {
        ty: DAE::T_REAL_DEFAULT().clone(),
    };
    let int_mul_mp: DAE::Operator = DAE::Operator::MUL_MATRIX_PRODUCT {
        ty: DAE::T_INTEGER_DEFAULT().clone(),
    };
    let real_mul_mp: DAE::Operator = DAE::Operator::MUL_MATRIX_PRODUCT {
        ty: DAE::T_REAL_DEFAULT().clone(),
    };
    let int_vector: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: DAE::T_INTEGER_DEFAULT().clone(),
        dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()],
    });
    let int_matrix: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: int_vector.clone(),
        dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()],
    });
    let real_vector: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: DAE::T_REAL_DEFAULT().clone(),
        dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()],
    });
    let real_matrix: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: real_vector.clone(),
        dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()],
    });
    let addIntArrays: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        for mut at in (intarrtypes().clone()).into_iter().cloned() {
            let __x = (
                DAE::Operator::ADD_ARR { ty: int_vector.clone() },
                list![at.clone(), at.clone()],
                at.clone(),
            );
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let addRealArrays: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        for mut at in (realarrtypes().clone()).into_iter().cloned() {
            let __x = (
                DAE::Operator::ADD_ARR {
                    ty: real_vector.clone(),
                },
                list![at.clone(), at.clone()],
                at.clone(),
            );
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let addStringArrays: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        for mut at in (stringarrtypes().clone()).into_iter().cloned() {
            let __x = (
                DAE::Operator::ADD_ARR {
                    ty: metamodelica::Ref::new(DAE::Type::T_ARRAY {
                        ty: DAE::T_STRING_DEFAULT().clone(),
                        dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()],
                    }),
                },
                list![at.clone(), at.clone()],
                at.clone(),
            );
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let addScalars: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = list![
        (
            DAE::Operator::ADD {
                ty: DAE::T_INTEGER_DEFAULT().clone()
            },
            list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()],
            DAE::T_INTEGER_DEFAULT().clone()
        ),
        (
            DAE::Operator::ADD {
                ty: DAE::T_REAL_DEFAULT().clone()
            },
            list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()],
            DAE::T_REAL_DEFAULT().clone()
        ),
        (
            DAE::Operator::ADD {
                ty: DAE::T_STRING_DEFAULT().clone()
            },
            list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()],
            DAE::T_STRING_DEFAULT().clone()
        )
    ];
    let addTypes: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = listAppend(
        addScalars.clone(),
        listAppend(
            addIntArrays.clone(),
            listAppend(addRealArrays.clone(), addStringArrays.clone()),
        ),
    );
    let addIntArrayScalars: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        let __thr_src0 = intarrtypes().clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = inttypes().clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(at), Some(rhs)) => {
                    let __x = (
                        DAE::Operator::ADD_ARRAY_SCALAR { ty: int_vector.clone() },
                        list![at.clone(), rhs.clone()],
                        at.clone(),
                    );
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    let addRealArrayScalars: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        let __thr_src0 = realarrtypes().clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = realtypes().clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(at), Some(rhs)) => {
                    let __x = (
                        DAE::Operator::ADD_ARRAY_SCALAR {
                            ty: real_vector.clone(),
                        },
                        list![at.clone(), rhs.clone()],
                        at.clone(),
                    );
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    let addStringArrayScalars: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = metamodelica::nil();
    let addEwTypes: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = listAppend(
        addIntArrayScalars.clone(),
        listAppend(
            addRealArrayScalars.clone(),
            listAppend(addStringArrayScalars.clone(), addTypes.clone()),
        ),
    );
    let subIntArrays: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        for mut at in (intarrtypes().clone()).into_iter().cloned() {
            let __x = (
                DAE::Operator::SUB_ARR { ty: int_vector.clone() },
                list![at.clone(), at.clone()],
                at.clone(),
            );
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let subRealArrays: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        for mut at in (realarrtypes().clone()).into_iter().cloned() {
            let __x = (
                DAE::Operator::SUB_ARR {
                    ty: real_vector.clone(),
                },
                list![at.clone(), at.clone()],
                at.clone(),
            );
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let subScalars: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = list![
        (
            DAE::Operator::SUB {
                ty: DAE::T_INTEGER_DEFAULT().clone()
            },
            list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()],
            DAE::T_INTEGER_DEFAULT().clone()
        ),
        (
            DAE::Operator::SUB {
                ty: DAE::T_REAL_DEFAULT().clone()
            },
            list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()],
            DAE::T_REAL_DEFAULT().clone()
        )
    ];
    let subTypes: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = listAppend(
        subScalars.clone(),
        listAppend(subIntArrays.clone(), subRealArrays.clone()),
    );
    let subIntArrayScalars: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        let __thr_src0 = intarrtypes().clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = inttypes().clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(at), Some(lhs)) => {
                    let __x = (
                        DAE::Operator::SUB_SCALAR_ARRAY { ty: int_vector.clone() },
                        list![lhs.clone(), at.clone()],
                        at.clone(),
                    );
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    let subRealArrayScalars: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        let __thr_src0 = realarrtypes().clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = realtypes().clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(at), Some(lhs)) => {
                    let __x = (
                        DAE::Operator::SUB_SCALAR_ARRAY {
                            ty: real_vector.clone(),
                        },
                        list![lhs.clone(), at.clone()],
                        at.clone(),
                    );
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    let subEwTypes: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = listAppend(
        subScalars.clone(),
        listAppend(
            subIntArrayScalars.clone(),
            listAppend(
                subRealArrayScalars.clone(),
                listAppend(subIntArrays.clone(), subRealArrays.clone()),
            ),
        ),
    );
    let mulScalars: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = list![
        (
            int_mul.clone(),
            list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()],
            DAE::T_INTEGER_DEFAULT().clone()
        ),
        (
            real_mul.clone(),
            list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()],
            DAE::T_REAL_DEFAULT().clone()
        )
    ];
    let mulScalarProduct: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = list![
        (
            int_mul_sp.clone(),
            list![int_vector.clone(), int_vector.clone()],
            DAE::T_INTEGER_DEFAULT().clone()
        ),
        (
            real_mul_sp.clone(),
            list![real_vector.clone(), real_vector.clone()],
            DAE::T_REAL_DEFAULT().clone()
        )
    ];
    let mulMatrixProduct: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = list![
        (
            int_mul_mp.clone(),
            list![int_vector.clone(), int_matrix.clone()],
            int_vector.clone()
        ),
        (
            int_mul_mp.clone(),
            list![int_matrix.clone(), int_vector.clone()],
            int_vector.clone()
        ),
        (
            int_mul_mp.clone(),
            list![int_matrix.clone(), int_matrix.clone()],
            int_matrix.clone()
        ),
        (
            real_mul_mp.clone(),
            list![real_vector.clone(), real_matrix.clone()],
            real_vector.clone()
        ),
        (
            real_mul_mp.clone(),
            list![real_matrix.clone(), real_vector.clone()],
            real_vector.clone()
        ),
        (
            real_mul_mp.clone(),
            list![real_matrix.clone(), real_matrix.clone()],
            real_matrix.clone()
        )
    ];
    let mulIntArrayScalars: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        let __thr_src0 = intarrtypes().clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = inttypes().clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(at), Some(rhs)) => {
                    let __x = (
                        DAE::Operator::MUL_ARRAY_SCALAR { ty: int_vector.clone() },
                        list![at.clone(), rhs.clone()],
                        at.clone(),
                    );
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    let mulRealArrayScalars: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        let __thr_src0 = realarrtypes().clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = realtypes().clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(at), Some(rhs)) => {
                    let __x = (
                        DAE::Operator::MUL_ARRAY_SCALAR {
                            ty: real_vector.clone(),
                        },
                        list![at.clone(), rhs.clone()],
                        at.clone(),
                    );
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    let mulTypes: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = listAppend(
        mulScalars.clone(),
        listAppend(
            mulIntArrayScalars.clone(),
            listAppend(
                mulRealArrayScalars.clone(),
                listAppend(mulScalarProduct.clone(), mulMatrixProduct.clone()),
            ),
        ),
    );
    let mulIntArray: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        for mut at in (intarrtypes().clone()).into_iter().cloned() {
            let __x = (
                DAE::Operator::MUL_ARR { ty: int_vector.clone() },
                list![at.clone(), at.clone()],
                at.clone(),
            );
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let mulRealArray: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        for mut at in (realarrtypes().clone()).into_iter().cloned() {
            let __x = (
                DAE::Operator::MUL_ARR {
                    ty: real_vector.clone(),
                },
                list![at.clone(), at.clone()],
                at.clone(),
            );
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let mulEwTypes: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = listAppend(
        mulScalars.clone(),
        listAppend(
            mulIntArrayScalars.clone(),
            listAppend(
                mulRealArrayScalars.clone(),
                listAppend(mulIntArray.clone(), mulRealArray.clone()),
            ),
        ),
    );
    let divTypes: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = metamodelica::cons(
        (
            real_div.clone(),
            list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()],
            DAE::T_REAL_DEFAULT().clone(),
        ),
        ({
            let mut __acc: metamodelica::List<(
                DAE::Operator,
                metamodelica::List<metamodelica::Ref<DAE::Type>>,
                metamodelica::Ref<DAE::Type>,
            )> = metamodelica::nil();
            let __thr_src0 = realarrtypes().clone();
            let mut __thr_it0 = (&__thr_src0).into_iter();
            let __thr_src1 = realtypes().clone();
            let mut __thr_it1 = (&__thr_src1).into_iter();
            loop {
                match (__thr_it0.next(), __thr_it1.next()) {
                    (Some(at), Some(rhs)) => {
                        let __x = (
                            DAE::Operator::DIV_ARRAY_SCALAR {
                                ty: real_vector.clone(),
                            },
                            list![at.clone(), rhs.clone()],
                            at.clone(),
                        );
                        __acc = cons(__x, __acc);
                    }
                    (None, None) => break,
                    _ => return Err("threaded for: ranges of unequal length"),
                }
            }
            __acc.reverse()
        }),
    );
    let divRealScalarArray: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        let __thr_src0 = realarrtypes().clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = realtypes().clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(at), Some(lhs)) => {
                    let __x = (
                        DAE::Operator::DIV_SCALAR_ARRAY {
                            ty: real_vector.clone(),
                        },
                        list![lhs.clone(), at.clone()],
                        at.clone(),
                    );
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    let divArrs: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        for mut at in (realarrtypes().clone()).into_iter().cloned() {
            let __x = (
                DAE::Operator::DIV_ARR {
                    ty: real_vector.clone(),
                },
                list![at.clone(), at.clone()],
                at.clone(),
            );
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let divEwTypes: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = listAppend(
        divTypes.clone(),
        listAppend(divRealScalarArray.clone(), divArrs.clone()),
    );
    let powTypes: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = list![
        (
            real_pow.clone(),
            list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()],
            DAE::T_REAL_DEFAULT().clone()
        ),
        (
            DAE::Operator::POW_ARR {
                ty: DAE::T_REAL_DEFAULT().clone()
            },
            list![real_matrix.clone(), DAE::T_INTEGER_DEFAULT().clone()],
            real_matrix.clone()
        )
    ];
    let andTypes: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = metamodelica::cons(
        (
            DAE::Operator::AND {
                ty: DAE::T_BOOL_DEFAULT().clone(),
            },
            list![DAE::T_BOOL_DEFAULT().clone(), DAE::T_BOOL_DEFAULT().clone()],
            DAE::T_BOOL_DEFAULT().clone(),
        ),
        ({
            let mut __acc: metamodelica::List<(
                DAE::Operator,
                metamodelica::List<metamodelica::Ref<DAE::Type>>,
                metamodelica::Ref<DAE::Type>,
            )> = metamodelica::nil();
            for at in (&(boolarrtypes().clone())).into_iter() {
                let __x = (
                    DAE::Operator::AND {
                        ty: DAE::T_BOOL_DEFAULT().clone(),
                    },
                    list![at.clone(), at.clone()],
                    at.clone(),
                );
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    );
    let orTypes: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )> = metamodelica::cons(
        (
            DAE::Operator::OR {
                ty: DAE::T_BOOL_DEFAULT().clone(),
            },
            list![DAE::T_BOOL_DEFAULT().clone(), DAE::T_BOOL_DEFAULT().clone()],
            DAE::T_BOOL_DEFAULT().clone(),
        ),
        ({
            let mut __acc: metamodelica::List<(
                DAE::Operator,
                metamodelica::List<metamodelica::Ref<DAE::Type>>,
                metamodelica::Ref<DAE::Type>,
            )> = metamodelica::nil();
            for at in (&(boolarrtypes().clone())).into_iter() {
                let __x = (
                    DAE::Operator::OR {
                        ty: DAE::T_BOOL_DEFAULT().clone(),
                    },
                    list![at.clone(), at.clone()],
                    at.clone(),
                );
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    );
    let mut op: Absyn::Operator = inOperator;
    let mut ia1: bool = Types::isArray(&t1);
    let mut ia2: bool = Types::isArray(&t2);
    if ia2 && !(ia1) {
        (e1, e2, t1, t2) = (match op {
            Absyn::Operator::ADD_EW { .. } => (e2, e1, t2, t1),
            Absyn::Operator::MUL { .. } => (e2, e1, t2, t1),
            Absyn::Operator::MUL_EW { .. } => (e2, e1, t2, t1),
            _ => (e1, e2, t1, t2),
        });
    } else if ia1 && !(ia2) {
        (op, e2) = (match op {
            Absyn::Operator::SUB_EW { .. } => (openmodelica_ast::Absyn::Operator::ADD_EW, Expression::negate(e2)?),
            _ => (op, e2),
        });
    }
    match '__try0: {
        ops = (match op {
            Absyn::Operator::ADD { .. } => addTypes.clone(),
            Absyn::Operator::ADD_EW { .. } => addEwTypes.clone(),
            Absyn::Operator::SUB { .. } => subTypes.clone(),
            Absyn::Operator::SUB_EW { .. } => subEwTypes.clone(),
            Absyn::Operator::MUL { .. } => mulTypes.clone(),
            Absyn::Operator::MUL_EW { .. } => mulEwTypes.clone(),
            Absyn::Operator::DIV { .. } => divTypes.clone(),
            Absyn::Operator::DIV_EW { .. } => divEwTypes.clone(),
            Absyn::Operator::POW { .. } => powTypes.clone(),
            Absyn::Operator::POW_EW { .. } => {
                let mut realarrs: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                let mut scalars: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                let mut types: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                let mut realscalararrs: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                let mut realarrsscalar: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                realarrs = operatorReturn(
                    DAE::Operator::POW_ARR2 {
                        ty: metamodelica::Ref::new(DAE::Type::T_ARRAY {
                            ty: DAE::T_REAL_DEFAULT().clone(),
                            dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()],
                        }),
                    },
                    realarrtypes().clone(),
                    realarrtypes().clone(),
                    realarrtypes().clone(),
                );
                scalars = list![(
                    DAE::Operator::POW {
                        ty: DAE::T_REAL_DEFAULT().clone()
                    },
                    list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()],
                    DAE::T_REAL_DEFAULT().clone()
                )];
                realscalararrs = operatorReturn(
                    DAE::Operator::POW_SCALAR_ARRAY {
                        ty: metamodelica::Ref::new(DAE::Type::T_ARRAY {
                            ty: DAE::T_REAL_DEFAULT().clone(),
                            dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()],
                        }),
                    },
                    realtypes().clone(),
                    realarrtypes().clone(),
                    realarrtypes().clone(),
                );
                realarrsscalar = operatorReturn(
                    DAE::Operator::POW_ARRAY_SCALAR {
                        ty: metamodelica::Ref::new(DAE::Type::T_ARRAY {
                            ty: DAE::T_REAL_DEFAULT().clone(),
                            dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()],
                        }),
                    },
                    realarrtypes().clone(),
                    realtypes().clone(),
                    realarrtypes().clone(),
                );
                types = unwrap_break_err!(List::flatten(list![scalars.clone(), realscalararrs.clone(), realarrsscalar.clone(), realarrs.clone()]), '__try0);
                types.clone()
            }
            Absyn::Operator::AND { .. } => andTypes.clone(),
            Absyn::Operator::OR { .. } => orTypes.clone(),
            Absyn::Operator::LESS { .. } => {
                let mut scalars: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                let mut types: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                let mut enum_op: (
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                );
                enum_op = makeEnumOperator(
                    DAE::Operator::LESS {
                        ty: DAE::T_ENUMERATION_DEFAULT().clone(),
                    },
                    t1.clone(),
                    t2.clone(),
                );
                scalars = list![
                    (
                        DAE::Operator::LESS {
                            ty: DAE::T_INTEGER_DEFAULT().clone()
                        },
                        list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    ),
                    enum_op.clone(),
                    (
                        DAE::Operator::LESS {
                            ty: DAE::T_REAL_DEFAULT().clone()
                        },
                        list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    ),
                    (
                        DAE::Operator::LESS {
                            ty: DAE::T_BOOL_DEFAULT().clone()
                        },
                        list![DAE::T_BOOL_DEFAULT().clone(), DAE::T_BOOL_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    ),
                    (
                        DAE::Operator::LESS {
                            ty: DAE::T_STRING_DEFAULT().clone()
                        },
                        list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    )
                ];
                types = unwrap_break_err!(List::flatten(list![scalars.clone()]), '__try0);
                types.clone()
            }
            Absyn::Operator::LESSEQ { .. } => {
                let mut scalars: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                let mut types: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                let mut enum_op: (
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                );
                enum_op = makeEnumOperator(
                    DAE::Operator::LESSEQ {
                        ty: DAE::T_ENUMERATION_DEFAULT().clone(),
                    },
                    t1.clone(),
                    t2.clone(),
                );
                scalars = list![
                    (
                        DAE::Operator::LESSEQ {
                            ty: DAE::T_INTEGER_DEFAULT().clone()
                        },
                        list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    ),
                    enum_op.clone(),
                    (
                        DAE::Operator::LESSEQ {
                            ty: DAE::T_REAL_DEFAULT().clone()
                        },
                        list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    ),
                    (
                        DAE::Operator::LESSEQ {
                            ty: DAE::T_BOOL_DEFAULT().clone()
                        },
                        list![DAE::T_BOOL_DEFAULT().clone(), DAE::T_BOOL_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    ),
                    (
                        DAE::Operator::LESSEQ {
                            ty: DAE::T_STRING_DEFAULT().clone()
                        },
                        list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    )
                ];
                types = unwrap_break_err!(List::flatten(list![scalars.clone()]), '__try0);
                types.clone()
            }
            Absyn::Operator::GREATER { .. } => {
                let mut scalars: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                let mut types: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                let mut enum_op: (
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                );
                enum_op = makeEnumOperator(
                    DAE::Operator::GREATER {
                        ty: DAE::T_ENUMERATION_DEFAULT().clone(),
                    },
                    t1.clone(),
                    t2.clone(),
                );
                scalars = list![
                    (
                        DAE::Operator::GREATER {
                            ty: DAE::T_INTEGER_DEFAULT().clone()
                        },
                        list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    ),
                    enum_op.clone(),
                    (
                        DAE::Operator::GREATER {
                            ty: DAE::T_REAL_DEFAULT().clone()
                        },
                        list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    ),
                    (
                        DAE::Operator::GREATER {
                            ty: DAE::T_BOOL_DEFAULT().clone()
                        },
                        list![DAE::T_BOOL_DEFAULT().clone(), DAE::T_BOOL_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    ),
                    (
                        DAE::Operator::GREATER {
                            ty: DAE::T_STRING_DEFAULT().clone()
                        },
                        list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    )
                ];
                types = unwrap_break_err!(List::flatten(list![scalars.clone()]), '__try0);
                types.clone()
            }
            Absyn::Operator::GREATEREQ { .. } => {
                let mut scalars: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                let mut types: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                let mut enum_op: (
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                );
                enum_op = makeEnumOperator(
                    DAE::Operator::GREATEREQ {
                        ty: DAE::T_ENUMERATION_DEFAULT().clone(),
                    },
                    t1.clone(),
                    t2.clone(),
                );
                scalars = list![
                    (
                        DAE::Operator::GREATEREQ {
                            ty: DAE::T_INTEGER_DEFAULT().clone()
                        },
                        list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    ),
                    enum_op.clone(),
                    (
                        DAE::Operator::GREATEREQ {
                            ty: DAE::T_REAL_DEFAULT().clone()
                        },
                        list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    ),
                    (
                        DAE::Operator::GREATEREQ {
                            ty: DAE::T_BOOL_DEFAULT().clone()
                        },
                        list![DAE::T_BOOL_DEFAULT().clone(), DAE::T_BOOL_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    ),
                    (
                        DAE::Operator::GREATEREQ {
                            ty: DAE::T_STRING_DEFAULT().clone()
                        },
                        list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone()
                    )
                ];
                types = unwrap_break_err!(List::flatten(list![scalars.clone()]), '__try0);
                types.clone()
            }
            Absyn::Operator::EQUAL { .. } => {
                let mut types: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                let mut enum_op: (
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                );
                enum_op = makeEnumOperator(
                    DAE::Operator::EQUAL {
                        ty: DAE::T_ENUMERATION_DEFAULT().clone(),
                    },
                    t1.clone(),
                    t2.clone(),
                );
                types = metamodelica::cons(
                    (
                        DAE::Operator::EQUAL {
                            ty: DAE::T_INTEGER_DEFAULT().clone(),
                        },
                        list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone(),
                    ),
                    metamodelica::cons(
                        enum_op.clone(),
                        metamodelica::cons(
                            (
                                DAE::Operator::EQUAL {
                                    ty: DAE::T_REAL_DEFAULT().clone(),
                                },
                                list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()],
                                DAE::T_BOOL_DEFAULT().clone(),
                            ),
                            metamodelica::cons(
                                (
                                    DAE::Operator::EQUAL {
                                        ty: DAE::T_STRING_DEFAULT().clone(),
                                    },
                                    list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()],
                                    DAE::T_BOOL_DEFAULT().clone(),
                                ),
                                metamodelica::cons(
                                    (
                                        DAE::Operator::EQUAL {
                                            ty: DAE::T_BOOL_DEFAULT().clone(),
                                        },
                                        list![DAE::T_BOOL_DEFAULT().clone(), DAE::T_BOOL_DEFAULT().clone()],
                                        DAE::T_BOOL_DEFAULT().clone(),
                                    ),
                                    metamodelica::nil(),
                                ),
                            ),
                        ),
                    ),
                );
                types.clone()
            }
            Absyn::Operator::NEQUAL { .. } => {
                let mut types: metamodelica::List<(
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                )>;
                let mut enum_op: (
                    DAE::Operator,
                    metamodelica::List<metamodelica::Ref<DAE::Type>>,
                    metamodelica::Ref<DAE::Type>,
                );
                enum_op = makeEnumOperator(
                    DAE::Operator::NEQUAL {
                        ty: DAE::T_ENUMERATION_DEFAULT().clone(),
                    },
                    t1.clone(),
                    t2.clone(),
                );
                types = metamodelica::cons(
                    (
                        DAE::Operator::NEQUAL {
                            ty: DAE::T_INTEGER_DEFAULT().clone(),
                        },
                        list![DAE::T_INTEGER_DEFAULT().clone(), DAE::T_INTEGER_DEFAULT().clone()],
                        DAE::T_BOOL_DEFAULT().clone(),
                    ),
                    metamodelica::cons(
                        enum_op.clone(),
                        metamodelica::cons(
                            (
                                DAE::Operator::NEQUAL {
                                    ty: DAE::T_REAL_DEFAULT().clone(),
                                },
                                list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()],
                                DAE::T_BOOL_DEFAULT().clone(),
                            ),
                            metamodelica::cons(
                                (
                                    DAE::Operator::NEQUAL {
                                        ty: DAE::T_STRING_DEFAULT().clone(),
                                    },
                                    list![DAE::T_STRING_DEFAULT().clone(), DAE::T_STRING_DEFAULT().clone()],
                                    DAE::T_BOOL_DEFAULT().clone(),
                                ),
                                metamodelica::cons(
                                    (
                                        DAE::Operator::NEQUAL {
                                            ty: DAE::T_BOOL_DEFAULT().clone(),
                                        },
                                        list![DAE::T_BOOL_DEFAULT().clone(), DAE::T_BOOL_DEFAULT().clone()],
                                        DAE::T_BOOL_DEFAULT().clone(),
                                    ),
                                    metamodelica::nil(),
                                ),
                            ),
                        ),
                    ),
                );
                types.clone()
            }
            _ => break '__try0 Err::<_, _>("match: no arm matched"),
        });
        Ok::<_, &'static str>((ops.clone(),))
    } {
        Ok((__try0_o0,)) => {
            ops = __try0_o0;
        }
        Err(__try0_err) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("OperatorOverloading.operatorsBinary failed, op: "));
                __mm_s.push_str(&*Dump::opSymbol(op)?);
                ArcStr::from(__mm_s)
            })?;
            return Err(__try0_err);
        }
    }
    Ok((ops, t1, e1, t2, e2, oty1, oe1, oty2, oe2))
}

fn operatorsUnary(
    mut op: Absyn::Operator,
) -> Result<
    metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )>,
> {
    let mut ops: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )>;
    ops = (match op {
        Absyn::Operator::UMINUS { .. } => {
            let mut intarrs: metamodelica::List<(
                DAE::Operator,
                metamodelica::List<metamodelica::Ref<DAE::Type>>,
                metamodelica::Ref<DAE::Type>,
            )>;
            let mut realarrs: metamodelica::List<(
                DAE::Operator,
                metamodelica::List<metamodelica::Ref<DAE::Type>>,
                metamodelica::Ref<DAE::Type>,
            )>;
            let mut scalars: metamodelica::List<(
                DAE::Operator,
                metamodelica::List<metamodelica::Ref<DAE::Type>>,
                metamodelica::Ref<DAE::Type>,
            )>;
            let mut types: metamodelica::List<(
                DAE::Operator,
                metamodelica::List<metamodelica::Ref<DAE::Type>>,
                metamodelica::Ref<DAE::Type>,
            )>;
            scalars = list![
                (
                    DAE::Operator::UMINUS {
                        ty: DAE::T_INTEGER_DEFAULT().clone()
                    },
                    list![DAE::T_INTEGER_DEFAULT().clone()],
                    DAE::T_INTEGER_DEFAULT().clone()
                ),
                (
                    DAE::Operator::UMINUS {
                        ty: DAE::T_REAL_DEFAULT().clone()
                    },
                    list![DAE::T_REAL_DEFAULT().clone()],
                    DAE::T_REAL_DEFAULT().clone()
                )
            ];
            intarrs = operatorReturnUnary(
                DAE::Operator::UMINUS_ARR {
                    ty: metamodelica::Ref::new(DAE::Type::T_ARRAY {
                        ty: DAE::T_INTEGER_DEFAULT().clone(),
                        dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()],
                    }),
                },
                &(intarrtypes().clone()),
                &(intarrtypes().clone()),
            )?;
            realarrs = operatorReturnUnary(
                DAE::Operator::UMINUS_ARR {
                    ty: metamodelica::Ref::new(DAE::Type::T_ARRAY {
                        ty: DAE::T_REAL_DEFAULT().clone(),
                        dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()],
                    }),
                },
                &(realarrtypes().clone()),
                &(realarrtypes().clone()),
            )?;
            types = List::flatten(list![scalars, intarrs, realarrs])?;
            types
        }
        Absyn::Operator::NOT { .. } => {
            let mut boolarrs: metamodelica::List<(
                DAE::Operator,
                metamodelica::List<metamodelica::Ref<DAE::Type>>,
                metamodelica::Ref<DAE::Type>,
            )>;
            let mut scalars: metamodelica::List<(
                DAE::Operator,
                metamodelica::List<metamodelica::Ref<DAE::Type>>,
                metamodelica::Ref<DAE::Type>,
            )>;
            let mut types: metamodelica::List<(
                DAE::Operator,
                metamodelica::List<metamodelica::Ref<DAE::Type>>,
                metamodelica::Ref<DAE::Type>,
            )>;
            scalars = list![(
                DAE::Operator::NOT {
                    ty: DAE::T_BOOL_DEFAULT().clone()
                },
                list![DAE::T_BOOL_DEFAULT().clone()],
                DAE::T_BOOL_DEFAULT().clone()
            )];
            boolarrs = operatorReturnUnary(
                DAE::Operator::NOT {
                    ty: DAE::T_BOOL_DEFAULT().clone(),
                },
                &(boolarrtypes().clone()),
                &(boolarrtypes().clone()),
            )?;
            types = List::flatten(list![scalars, boolarrs])?;
            types
        }
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("OperatorOverloading.operatorsUnary failed, op: "));
                __mm_s.push_str(&*Dump::opSymbol(op)?);
                ArcStr::from(__mm_s)
            })?;
            return Err("fail");
        }
    });
    Ok(ops)
}

fn makeEnumOperator(
    mut inOp: DAE::Operator,
    mut inType1: metamodelica::Ref<DAE::Type>,
    mut inType2: metamodelica::Ref<DAE::Type>,
) -> (
    DAE::Operator,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
    metamodelica::Ref<DAE::Type>,
) {
    let mut outOp: (
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    );
    outOp = 'mc: {
        let __mc_input = (&*inType1, &*inType2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ENUMERATION { .. }, Deref @ DAE::Type::T_ENUMERATION { .. }) => {
                    let mut op_ty: metamodelica::Ref<DAE::Type>;
                    let mut op: DAE::Operator;
                    op_ty = Types::simplifyType(inType1.clone())?;
                    op = Expression::setOpType(inOp.clone(), op_ty.clone())?;
                    Ok((op.clone(), list![inType1.clone(), inType2.clone()], DAE::T_BOOL_DEFAULT().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ENUMERATION { .. }, _) => {
                    let mut op_ty: metamodelica::Ref<DAE::Type>;
                    let mut op: DAE::Operator;
                    op_ty = Types::simplifyType(inType1.clone())?;
                    op = Expression::setOpType(inOp.clone(), op_ty.clone())?;
                    Ok((op.clone(), list![inType1.clone(), inType1.clone()], DAE::T_BOOL_DEFAULT().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_ENUMERATION { .. }) => {
                    let mut op_ty: metamodelica::Ref<DAE::Type>;
                    let mut op: DAE::Operator;
                    op_ty = Types::simplifyType(inType2.clone())?;
                    op = Expression::setOpType(inOp.clone(), op_ty.clone())?;
                    Ok((op.clone(), list![inType2.clone(), inType2.clone()], DAE::T_BOOL_DEFAULT().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inOp.clone(), list![DAE::T_ENUMERATION_DEFAULT().clone(), DAE::T_ENUMERATION_DEFAULT().clone()], DAE::T_BOOL_DEFAULT().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outOp
}

fn buildOperatorTypes(
    mut inTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<
    metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )>,
> {
    let mut outOperatorTypes: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )>;
    outOperatorTypes = (::match_deref::match_deref! { match inTypes {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_FUNCTION { funcArg: args, funcResultType: tp, .. }, tail: tps } => {
            let mut funcname = inPath;
            let mut argtypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut rest: metamodelica::List<(DAE::Operator, metamodelica::List<metamodelica::Ref<DAE::Type>>, metamodelica::Ref<DAE::Type>)>;
            argtypes = List::map(args.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::funcArgType(&__a0)) })?;
            rest = buildOperatorTypes(tps, funcname.clone())?;
            metamodelica::cons((DAE::Operator::USERDEFINED { fqName: funcname }, argtypes, tp.clone()), rest)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outOperatorTypes)
}

fn operatorReturn(
    mut inOperator: DAE::Operator,
    mut inLhsTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inRhsTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inReturnTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> metamodelica::List<(
    DAE::Operator,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
    metamodelica::Ref<DAE::Type>,
)> {
    let mut outOperators: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )>;
    outOperators = ({
        let mut __acc: metamodelica::List<(
            DAE::Operator,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<DAE::Type>,
        )> = metamodelica::nil();
        let __thr_src0 = inLhsTypes;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = inRhsTypes;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        let __thr_src2 = inReturnTypes;
        let mut __thr_it2 = (&__thr_src2).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next(), __thr_it2.next()) {
                (Some(l), Some(r), Some(re)) => {
                    let __x = (inOperator.clone(), list![l.clone(), r.clone()], re.clone());
                    __acc = cons(__x, __acc);
                }
                (None, None, None) => break,
                _ => panic!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    outOperators
}

fn operatorReturnUnary(
    mut inOperator: DAE::Operator,
    mut inArgTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inReturnTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<
    metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )>,
> {
    let mut outOperators: metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )>;
    outOperators = (::match_deref::match_deref! { match (inArgTypes, inReturnTypes) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::ListNode::Cons { head: l, tail: lr }, Deref @ metamodelica::ListNode::Cons { head: re, tail: rer }) => {
            let mut op = inOperator;
            let mut rest: metamodelica::List<(DAE::Operator, metamodelica::List<metamodelica::Ref<DAE::Type>>, metamodelica::Ref<DAE::Type>)>;
            let mut t: (DAE::Operator, metamodelica::List<metamodelica::Ref<DAE::Type>>, metamodelica::Ref<DAE::Type>);
            rest = operatorReturnUnary(op.clone(), lr, rer)?;
            t = (op, list![l.clone()], re.clone());
            metamodelica::cons(t, rest)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outOperators)
}

fn getOperatorFuncsOrEmpty(
    mut inCache: &FCore::Cache,
    mut env: &FCore::Graph,
    mut tys: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut opName: &ArcStr,
    mut info: &SourceInfo,
    mut acc: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Type>>)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut funcs: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
    (cache, funcs) = 'mc: {
        let __mc_input = &**tys;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: ty, tail: rest } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut funcs: metamodelica::List<metamodelica::Ref<DAE::Type>> = funcs.clone();
                    (cache, funcs) = getOperatorFuncsOrEmptySingleTy(inCache.clone(), env, metamodelica::AsArg::as_arg(&ty), opName.clone(), info.clone())?;
                    (cache, funcs) = getOperatorFuncsOrEmpty(&cache, env, metamodelica::AsArg::as_arg(&rest), opName, info, &(listAppend(funcs.clone(), acc.clone())))?;
                    Ok(((cache.clone(), funcs.clone()), cache.clone(), funcs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            funcs = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut funcs: metamodelica::List<metamodelica::Ref<DAE::Type>> = funcs.clone();
                    (cache, funcs) = getOperatorFuncsOrEmpty(inCache, env, metamodelica::AsArg::as_arg(&rest), opName, info, acc)?;
                    Ok(((cache.clone(), funcs.clone()), cache.clone(), funcs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            funcs = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut funcs: metamodelica::List<metamodelica::Ref<DAE::Type>> = funcs.clone();
                    let (__pa0, Util::SUCCESS { .. }) = (Static::instantiateDaeFunctionFromTypes(inCache, env, acc.clone(), false, None, true, openmodelica_util::Util::Status::SUCCESS)) else { return Err("pattern mismatch") };
                    cache = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(Types::traverseType(metamodelica::Ref::new(DAE::Type::T_TUPLE { types: acc.clone(), names: None }), -1, &fnptr!(Types::makeExpDimensionsUnknown, metamodelica::Ref<DAE::Type>, i32))?) {
                        (Deref @ DAE::Type::T_TUPLE { types: __pa1, names: _ }, _) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    funcs = metamodelica::Own::own(__pa1);
                    Ok(((cache.clone(), funcs.clone()), cache.clone(), funcs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            funcs = __wb1;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((cache, funcs))
}

pub mod AvlTreePathPathEnv {
    use super::*;
    pub type Key = metamodelica::Ref<Absyn::Path>;

    pub type Value = metamodelica::Ref<Absyn::Path>;

    pub(crate) fn keyStr(mut inKey: Key) -> Result<ArcStr> {
        let mut outString: ArcStr;
        outString = AbsynUtil::pathString(inKey, literal!("."), true, false)?;
        Ok(outString)
    }

    pub(crate) fn valueStr(mut inValue: Value) -> Result<ArcStr> {
        let mut outString: ArcStr;
        outString = AbsynUtil::pathString(inValue, literal!("."), true, false)?;
        Ok(outString)
    }

    pub(crate) fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> Result<i32> {
        let mut outResult: i32;
        outResult = AbsynUtil::pathCompareNoQual(inKey1, inKey2)?;
        Ok(outResult)
    }

    pub use addConflictKeep as addConflictDefault;

    pub type ConflictFunc = std::sync::Arc<dyn ::std::ops::Fn(Value, Value, Key) -> Result<Value> + 'static>;

    /// The binary tree data structure.
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub(crate) enum Tree {
        NODE {
            /// The key of the node.
            key: Key,
            value: Value,
            /// Height of tree, used for balancing
            height: i32,
            /// Left subtree.
            left: metamodelica::Ref<Tree>,
            /// Right subtree.
            right: metamodelica::Ref<Tree>,
        },
        LEAF {
            /// The key of the node.
            key: Key,
            value: Value,
        },
        EMPTY,
    }
    impl metamodelica::gc::MMTrace for Tree {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Tree::NODE {
                    key,
                    value,
                    height,
                    left,
                    right,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(key, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(height, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(left, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(right, __mmv)?;
                    Ok(())
                }
                Tree::LEAF { key, value } => {
                    metamodelica::gc::MMTrace::mm_accept(key, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                    Ok(())
                }
                Tree::EMPTY => Ok(()),
            }
        }
    }
    impl Tree {
        pub fn interned_EMPTY() -> metamodelica::Ref<Tree> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<Tree>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(Tree::EMPTY));
            (*INTERNED).clone()
        }
    }
    pub fn interned_EMPTY() -> metamodelica::Ref<Tree> {
        Tree::interned_EMPTY()
    }
    impl Default for Tree {
        fn default() -> Self {
            Self::EMPTY
        }
    }
    pub(crate) use self::Tree::{EMPTY, LEAF, NODE};

    pub type ValueNode = metamodelica::Ref<Absyn::Path>;

    pub(crate) fn add(
        mut inTree: metamodelica::Ref<Tree>,
        mut inKey: &Key,
        mut inValue: &Value,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::Ref<Absyn::Path>,
            metamodelica::Ref<Absyn::Path>,
        ) -> Result<metamodelica::Ref<Absyn::Path>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = inTree;
        tree = (match &*tree.clone() {
            Tree::EMPTY { .. } => metamodelica::Ref::new(Tree::LEAF {
                key: inKey.clone(),
                value: inValue.clone(),
            }),
            Tree::NODE { key, .. } => {
                let mut value: Value;
                let mut key_comp: i32;
                key_comp = keyCompare(inKey.clone(), key.clone())?;
                if key_comp == -1 {
                    assign_variant_field!(tree => Tree::NODE; left = add(var_field!((*tree).left, Tree::NODE).clone(), inKey, inValue, conflictFunc)?);
                } else if key_comp == 1 {
                    assign_variant_field!(tree => Tree::NODE; right = add(var_field!((*tree).right, Tree::NODE).clone(), inKey, inValue, conflictFunc)?);
                } else {
                    value = conflictFunc(
                        inValue.clone(),
                        var_field!((*tree).value, Tree::NODE).clone(),
                        key.clone(),
                    )?;
                    if !(referenceEq(&*(var_field!((*tree).value, Tree::NODE).clone()), &*(&*value))) {
                        assign_variant_field!(tree => Tree::NODE; value = value);
                    }
                }
                if (key_comp == 0) { tree } else { balance(tree)? }
            }
            Tree::LEAF { key: __tree_key, .. } => {
                let mut value: Value;
                let mut key_comp: i32;
                let mut outTree: metamodelica::Ref<Tree>;
                key_comp = keyCompare(inKey.clone(), __tree_key.clone())?;
                if key_comp == -1 {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: metamodelica::Ref::new(Tree::LEAF {
                            key: inKey.clone(),
                            value: inValue.clone(),
                        }),
                        right: crate::OperatorOverloading::AvlTreePathPathEnv::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::OperatorOverloading::AvlTreePathPathEnv::Tree::interned_EMPTY(),
                        right: metamodelica::Ref::new(Tree::LEAF {
                            key: inKey.clone(),
                            value: inValue.clone(),
                        }),
                    });
                } else {
                    value = conflictFunc(
                        inValue.clone(),
                        var_field!((*tree).value, Tree::LEAF).clone(),
                        var_field!((*tree).key, Tree::LEAF).clone(),
                    )?;
                    if !(referenceEq(&*(var_field!((*tree).value, Tree::LEAF).clone()), &*(&*value))) {
                        assign_variant_field!(tree => Tree::LEAF; value = value);
                    }
                    outTree = tree;
                }
                if (key_comp == 0) { outTree } else { balance(outTree)? }
            }
        });
        Ok(tree)
    }

    pub(crate) fn addConflictFail(mut newValue: &Value, mut oldValue: &Value, mut key: &Key) -> Result<Value> {
        let mut value: Value;
        return Err("fail");
        Ok(value)
    }

    pub fn addConflictKeep(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Value {
        let mut value: Value = oldValue;
        value
    }

    pub(crate) fn addConflictReplace(mut newValue: Value, mut oldValue: &Value, mut key: &Key) -> Value {
        let mut value: Value = newValue;
        value
    }

    pub(crate) fn addList(
        mut tree: metamodelica::Ref<Tree>,
        mut inValues: &metamodelica::List<(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>)>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::Ref<Absyn::Path>,
            metamodelica::Ref<Absyn::Path>,
        ) -> Result<metamodelica::Ref<Absyn::Path>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = tree;
        let mut key: Key;
        let mut value: Value;
        for mut t in &**inValues {
            (key, value) = t.clone();
            tree = add(tree, &key, &value, conflictFunc)?;
        }
        Ok(tree)
    }

    pub(crate) fn addUpdate(
        mut tree: metamodelica::Ref<Tree>,
        mut key: &Key,
        mut r#fn: &dyn ::std::ops::Fn(Option<metamodelica::Ref<Absyn::Path>>) -> Result<metamodelica::Ref<Absyn::Path>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        pub type UpdateFn =
            std::sync::Arc<dyn ::std::ops::Fn(Option<metamodelica::Ref<Absyn::Path>>) -> Result<Value> + 'static>;

        let mut tree: metamodelica::Ref<Tree> = tree;
        let mut key_comp: i32;
        let mut new_tree: metamodelica::Ref<Tree>;
        tree = (match &*tree {
            Tree::EMPTY { .. } => metamodelica::Ref::new(Tree::LEAF {
                key: key.clone(),
                value: r#fn(None)?,
            }),
            Tree::NODE { key: __tree_key, .. } => {
                key_comp = keyCompare(key.clone(), __tree_key.clone())?;
                if key_comp == -1 {
                    assign_variant_field!(tree => Tree::NODE; left = addUpdate(var_field!((*tree).left, Tree::NODE).clone(), key, r#fn)?);
                } else if key_comp == 1 {
                    assign_variant_field!(tree => Tree::NODE; right = addUpdate(var_field!((*tree).right, Tree::NODE).clone(), key, r#fn)?);
                } else {
                    assign_variant_field!(tree => Tree::NODE; value = r#fn(Some(var_field!((*tree).value, Tree::NODE).clone()))?);
                }
                if (key_comp == 0) { tree } else { balance(tree)? }
            }
            Tree::LEAF { key: __tree_key, .. } => {
                key_comp = keyCompare(key.clone(), __tree_key.clone())?;
                if key_comp == -1 {
                    new_tree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: metamodelica::Ref::new(Tree::LEAF {
                            key: key.clone(),
                            value: r#fn(None)?,
                        }),
                        right: crate::OperatorOverloading::AvlTreePathPathEnv::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    new_tree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::OperatorOverloading::AvlTreePathPathEnv::Tree::interned_EMPTY(),
                        right: metamodelica::Ref::new(Tree::LEAF {
                            key: key.clone(),
                            value: r#fn(None)?,
                        }),
                    });
                } else {
                    assign_variant_field!(tree => Tree::LEAF; value = r#fn(Some(var_field!((*tree).value, Tree::LEAF).clone()))?);
                    new_tree = tree;
                }
                if (key_comp == 0) { new_tree } else { balance(new_tree)? }
            }
        });
        Ok(tree)
    }

    fn balance(mut inTree: metamodelica::Ref<Tree>) -> Result<metamodelica::Ref<Tree>> {
        let mut outTree: metamodelica::Ref<Tree> = inTree.clone();
        outTree = (match &*outTree {
            Tree::LEAF { .. } => inTree,
            Tree::NODE {
                left: __outTree_left,
                right: __outTree_right,
                ..
            } => {
                let mut lh: i32;
                let mut rh: i32;
                let mut diff: i32;
                let mut balanced_tree: metamodelica::Ref<Tree>;
                lh = height(metamodelica::AsArg::as_arg(&__outTree_left));
                rh = height(metamodelica::AsArg::as_arg(&__outTree_right));
                diff = lh - rh;
                if diff < -1 {
                    balanced_tree = if (calculateBalance(var_field!((*outTree).right, Tree::NODE)) > 0) {
                        rotateLeft(setTreeLeftRight(
                            outTree.clone(),
                            var_field!((*outTree).left, Tree::NODE).clone(),
                            rotateRight(var_field!((*outTree).right, Tree::NODE).clone())?,
                        )?)?
                    } else {
                        rotateLeft(outTree)?
                    };
                } else if diff > 1 {
                    balanced_tree = if (calculateBalance(var_field!((*outTree).left, Tree::NODE)) < 0) {
                        rotateRight(setTreeLeftRight(
                            outTree.clone(),
                            rotateLeft(var_field!((*outTree).left, Tree::NODE).clone())?,
                            var_field!((*outTree).right, Tree::NODE).clone(),
                        )?)?
                    } else {
                        rotateRight(outTree)?
                    };
                } else if var_field!((*outTree).height, Tree::NODE).clone() != std::cmp::max(lh, rh) + 1 {
                    assign_variant_field!(outTree => Tree::NODE; height = std::cmp::max(lh, rh) + 1);
                    balanced_tree = outTree;
                } else {
                    balanced_tree = outTree;
                }
                balanced_tree
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(outTree)
    }

    fn calculateBalance(mut inNode: &metamodelica::Ref<Tree>) -> i32 {
        let mut outBalance: i32;
        outBalance = (match &**inNode {
            Tree::NODE {
                left: __inNode_left,
                right: __inNode_right,
                ..
            } => {
                height(metamodelica::AsArg::as_arg(&__inNode_left))
                    - height(metamodelica::AsArg::as_arg(&__inNode_right))
            }
            Tree::LEAF { .. } => 0,
            _ => 0,
        });
        outBalance
    }

    pub(crate) fn fold<'__b, FT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut inTree: &'__b metamodelica::Ref<Tree>,
        mut inFunc: &'__b dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>, FT) -> Result<FT>,
        mut inStartValue: FT,
    ) -> Result<FT> {
        pub type FoldFunc<FT: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<FT> + 'static>;

        let mut outResult: FT = inStartValue;
        outResult = (match &**inTree {
            Tree::NODE { key, value, .. } => {
                outResult = fold(var_field!((**inTree).left, Tree::NODE), inFunc, outResult)?;
                outResult = inFunc(key.clone(), value.clone(), outResult)?;
                outResult = fold(var_field!((**inTree).right, Tree::NODE), inFunc, outResult)?;
                outResult
            }
            Tree::LEAF { key, value } => {
                outResult = inFunc(key.clone(), value.clone(), outResult)?;
                outResult
            }
            _ => outResult,
        });
        Ok(outResult)
    }

    pub(crate) fn foldCond<FT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut tree: &metamodelica::Ref<Tree>,
        mut foldFunc: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>, FT) -> Result<(FT, bool)>,
        mut value: FT,
    ) -> Result<FT> {
        pub type FoldFunc<FT: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<(FT, bool)> + 'static>;

        let mut value: FT = value;
        value = (match &**tree {
            Tree::NODE {
                key: __tree_key,
                left: __tree_left,
                right: __tree_right,
                value: __tree_value,
                ..
            } => {
                let mut c: bool;
                (value, c) = foldFunc(__tree_key.clone(), __tree_value.clone(), value)?;
                if c {
                    value = foldCond(metamodelica::AsArg::as_arg(&__tree_left), foldFunc, value)?;
                    value = foldCond(metamodelica::AsArg::as_arg(&__tree_right), foldFunc, value)?;
                }
                value
            }
            Tree::LEAF {
                key: __tree_key,
                value: __tree_value,
            } => {
                let mut c: bool;
                (value, c) = foldFunc(__tree_key.clone(), __tree_value.clone(), value)?;
                value
            }
            _ => value,
        });
        Ok(value)
    }

    pub(crate) fn fold_2<
        FT1: Clone + 'static + metamodelica::gc::MMTrace,
        FT2: Clone + 'static + metamodelica::gc::MMTrace,
    >(
        mut tree: &metamodelica::Ref<Tree>,
        mut foldFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::Ref<Absyn::Path>,
            FT1,
            FT2,
        ) -> Result<(FT1, FT2)>,
        mut foldArg1: FT1,
        mut foldArg2: FT2,
    ) -> Result<(FT1, FT2)> {
        pub type FoldFunc<FT1: Clone + 'static, FT2: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

        let mut foldArg1: FT1 = foldArg1;
        let mut foldArg2: FT2 = foldArg2;
        let () = (match &**tree {
            Tree::NODE {
                key: __tree_key,
                left: __tree_left,
                right: __tree_right,
                value: __tree_value,
                ..
            } => {
                (foldArg1, foldArg2) = fold_2(metamodelica::AsArg::as_arg(&__tree_left), foldFunc, foldArg1, foldArg2)?;
                (foldArg1, foldArg2) = foldFunc(__tree_key.clone(), __tree_value.clone(), foldArg1, foldArg2)?;
                (foldArg1, foldArg2) =
                    fold_2(metamodelica::AsArg::as_arg(&__tree_right), foldFunc, foldArg1, foldArg2)?;
                ()
            }
            Tree::LEAF {
                key: __tree_key,
                value: __tree_value,
            } => {
                (foldArg1, foldArg2) = foldFunc(__tree_key.clone(), __tree_value.clone(), foldArg1, foldArg2)?;
                ()
            }
            _ => (),
        });
        Ok((foldArg1, foldArg2))
    }

    pub(crate) fn forEach(
        mut tree: &metamodelica::Ref<Tree>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>) -> Result<()>,
    ) -> Result<()> {
        pub type EachFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<()> + 'static>;

        let () = (match &**tree {
            Tree::NODE {
                key: __tree_key,
                left: __tree_left,
                right: __tree_right,
                value: __tree_value,
                ..
            } => {
                forEach(metamodelica::AsArg::as_arg(&__tree_left), func)?;
                func(__tree_key.clone(), __tree_value.clone())?;
                forEach(metamodelica::AsArg::as_arg(&__tree_right), func)?;
                ()
            }
            Tree::LEAF {
                key: __tree_key,
                value: __tree_value,
            } => {
                func(__tree_key.clone(), __tree_value.clone())?;
                ()
            }
            Tree::EMPTY { .. } => (),
        });
        Ok(())
    }

    pub(crate) fn fromList(
        mut inValues: &metamodelica::List<(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>)>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::Ref<Absyn::Path>,
            metamodelica::Ref<Absyn::Path>,
        ) -> Result<metamodelica::Ref<Absyn::Path>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = crate::OperatorOverloading::AvlTreePathPathEnv::Tree::interned_EMPTY();
        let mut key: Key;
        let mut value: Value;
        for mut t in &**inValues {
            (key, value) = t.clone();
            tree = add(tree, &key, &value, conflictFunc)?;
        }
        Ok(tree)
    }

    pub(crate) fn get<'__b>(mut tree: &'__b metamodelica::Ref<Tree>, mut key: Key) -> Result<Value> {
        let mut value: Value;
        let mut k: Key;
        k = (match &**tree {
            Tree::NODE { .. } => var_field!((**tree).key, Tree::NODE).clone(),
            Tree::LEAF { .. } => var_field!((**tree).key, Tree::LEAF).clone(),
            _ => return Err("match: no arm matched"),
        });
        value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k)?, tree.clone())) {
            (0, Deref @ Tree::LEAF { .. }) => var_field!((**tree).value, Tree::LEAF).clone(),
            (0, Deref @ Tree::NODE { .. }) => var_field!((**tree).value, Tree::NODE).clone(),
            (1, Deref @ Tree::NODE { .. }) => get(var_field!((**tree).right, Tree::NODE), key)?,
            ((-1), Deref @ Tree::NODE { .. }) => get(var_field!((**tree).left, Tree::NODE), key)?,
            _ => return Err("match: no arm matched"),
        } });
        Ok(value)
    }

    pub(crate) fn getOpt<'__b>(
        mut tree: &'__b metamodelica::Ref<Tree>,
        mut key: Key,
    ) -> Result<Option<metamodelica::Ref<Absyn::Path>>> {
        '__tco: loop {
            let mut k: Key;
            k = (match &**tree {
                Tree::NODE { .. } => var_field!((**tree).key, Tree::NODE).clone(),
                Tree::LEAF { .. } => var_field!((**tree).key, Tree::LEAF).clone(),
                _ => key.clone(),
            });
            ::match_deref::match_deref! { match &((keyCompare(key.clone(), k)?, tree.clone())) {
                (0, Deref @ Tree::LEAF { .. }) => return Ok(Some(var_field!((**tree).value, Tree::LEAF).clone())),
                (0, Deref @ Tree::NODE { .. }) => return Ok(Some(var_field!((**tree).value, Tree::NODE).clone())),
                (1, Deref @ Tree::NODE { .. }) => { (tree, key) = (var_field!((**tree).right, Tree::NODE), key); continue '__tco; },
                ((-1), Deref @ Tree::NODE { .. }) => { (tree, key) = (var_field!((**tree).left, Tree::NODE), key); continue '__tco; },
                _ => return Ok(None),
                _ => return Err("match: no arm matched"),
            } }
        }
    }

    pub(crate) fn hasKey(mut inTree: metamodelica::Ref<Tree>, mut inKey: Key) -> Result<bool> {
        let mut comp: bool = false;
        let mut key: Key;
        let mut key_comp: i32;
        let mut tree: metamodelica::Ref<Tree>;
        key = (match &*inTree {
            Tree::NODE { key: __inTree_key, .. } => __inTree_key.clone(),
            Tree::LEAF { key: __inTree_key, .. } => __inTree_key.clone(),
            Tree::EMPTY { .. } => {
                return Ok(comp);
                return Err("fail");
            }
        });
        key_comp = keyCompare(inKey.clone(), key)?;
        comp = (::match_deref::match_deref! { match &((key_comp, inTree)) {
            (0, _) => true,
            (1, Deref @ Tree::NODE { right: __esc_tree, .. }) => {
                tree = (*__esc_tree).clone();
                hasKey(tree.clone(), inKey)?
            },
            ((-1), Deref @ Tree::NODE { left: __esc_tree, .. }) => {
                tree = (*__esc_tree).clone();
                hasKey(tree.clone(), inKey)?
            },
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(comp)
    }

    fn height(mut inNode: &metamodelica::Ref<Tree>) -> i32 {
        let mut outHeight: i32;
        outHeight = (match &**inNode {
            Tree::NODE {
                height: __inNode_height,
                ..
            } => __inNode_height.clone(),
            Tree::LEAF { .. } => 1,
            _ => 0,
        });
        outHeight
    }

    pub(crate) fn intersection() -> Result<()> {
        return Err("fail");
        Ok(())
    }

    pub(crate) fn isEmpty(mut tree: &metamodelica::Ref<Tree>) -> bool {
        let mut isEmpty: bool;
        isEmpty = (match &**tree {
            Tree::EMPTY { .. } => true,
            _ => false,
        });
        isEmpty
    }

    pub(crate) fn join<'__b>(
        mut tree: metamodelica::Ref<Tree>,
        mut treeToJoin: &'__b metamodelica::Ref<Tree>,
        mut conflictFunc: &'__b dyn ::std::ops::Fn(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::Ref<Absyn::Path>,
            metamodelica::Ref<Absyn::Path>,
        ) -> Result<metamodelica::Ref<Absyn::Path>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        '__tco: loop {
            match &**treeToJoin {
                Tree::EMPTY { .. } => return Ok(tree),
                Tree::NODE { .. } => {
                    tree = add(
                        tree,
                        var_field!((**treeToJoin).key, Tree::NODE),
                        var_field!((**treeToJoin).value, Tree::NODE),
                        conflictFunc,
                    )?;
                    tree = join(tree, var_field!((**treeToJoin).left, Tree::NODE), conflictFunc)?;
                    {
                        (tree, treeToJoin, conflictFunc) =
                            (tree, var_field!((**treeToJoin).right, Tree::NODE), conflictFunc);
                        continue '__tco;
                    }
                }
                Tree::LEAF { .. } => {
                    return Ok(add(
                        tree,
                        var_field!((**treeToJoin).key, Tree::LEAF),
                        var_field!((**treeToJoin).value, Tree::LEAF),
                        conflictFunc,
                    )?);
                }
            }
        }
    }

    pub(crate) fn listKeys<'__b>(
        mut tree: &'__b metamodelica::Ref<Tree>,
        mut lst: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    ) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
        '__tco: loop {
            match &**tree {
                Tree::NODE { key, .. } => {
                    lst = listKeys(var_field!((**tree).right, Tree::NODE), lst);
                    lst = metamodelica::cons(key.clone(), lst);
                    {
                        (tree, lst) = (var_field!((**tree).left, Tree::NODE), lst);
                        continue '__tco;
                    }
                }
                Tree::LEAF { key, .. } => return metamodelica::cons(key.clone(), lst),
                _ => return lst,
            }
        }
    }

    pub(crate) fn listKeysReverse<'__b>(
        mut inTree: &'__b metamodelica::Ref<Tree>,
        mut lst: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    ) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
        '__tco: loop {
            match &**inTree {
                Tree::LEAF { .. } => return metamodelica::cons(var_field!((**inTree).key, Tree::LEAF).clone(), lst),
                Tree::NODE { .. } => {
                    lst = listKeysReverse(var_field!((**inTree).left, Tree::NODE), lst);
                    lst = metamodelica::cons(var_field!((**inTree).key, Tree::NODE).clone(), lst);
                    {
                        (inTree, lst) = (var_field!((**inTree).right, Tree::NODE), lst);
                        continue '__tco;
                    }
                }
                _ => return lst,
            }
        }
    }

    pub(crate) fn listValues<'__b>(
        mut tree: &'__b metamodelica::Ref<Tree>,
        mut lst: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    ) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
        '__tco: loop {
            match &**tree {
                Tree::NODE { value, .. } => {
                    lst = listValues(var_field!((**tree).right, Tree::NODE), lst);
                    lst = metamodelica::cons(value.clone(), lst);
                    {
                        (tree, lst) = (var_field!((**tree).left, Tree::NODE), lst);
                        continue '__tco;
                    }
                }
                Tree::LEAF { value, .. } => return metamodelica::cons(value.clone(), lst),
                _ => return lst,
            }
        }
    }

    pub(crate) fn map(
        mut inTree: metamodelica::Ref<Tree>,
        mut inFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::Ref<Absyn::Path>,
        ) -> Result<metamodelica::Ref<Absyn::Path>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<Value> + 'static>;

        let mut outTree: metamodelica::Ref<Tree> = inTree.clone();
        outTree = (match &*outTree.clone() {
            Tree::NODE {
                key,
                value,
                left: __outTree_left,
                right: __outTree_right,
                ..
            } => {
                let mut new_value: Value;
                let mut new_left: metamodelica::Ref<Tree>;
                let mut new_right: metamodelica::Ref<Tree>;
                new_left = map(__outTree_left.clone(), inFunc)?;
                new_value = inFunc(key.clone(), value.clone())?;
                new_right = map(__outTree_right.clone(), inFunc)?;
                if !(referenceEq(&*(&*new_left), &*(var_field!((*outTree).left, Tree::NODE).clone())))
                    || !(referenceEq(&*(value.clone()), &*(&*new_value)))
                    || !(referenceEq(&*(&*new_right), &*(var_field!((*outTree).right, Tree::NODE).clone())))
                {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: key.clone(),
                        value: new_value,
                        height: var_field!((*outTree).height, Tree::NODE).clone(),
                        left: new_left,
                        right: new_right,
                    });
                }
                outTree
            }
            Tree::LEAF { key, value } => {
                let mut new_value: Value;
                new_value = inFunc(key.clone(), value.clone())?;
                if !(referenceEq(&*(value.clone()), &*(&*new_value))) {
                    assign_variant_field!(outTree => Tree::LEAF; value = new_value);
                }
                outTree
            }
            _ => inTree,
        });
        Ok(outTree)
    }

    pub(crate) fn mapFold<FT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut inTree: metamodelica::Ref<Tree>,
        mut inFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::Ref<Absyn::Path>,
            FT,
        ) -> Result<(metamodelica::Ref<Absyn::Path>, FT)>,
        mut inStartValue: FT,
    ) -> Result<(metamodelica::Ref<Tree>, FT)> {
        pub type MapFunc<FT: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<Value> + 'static>;

        let mut outTree: metamodelica::Ref<Tree> = inTree.clone();
        let mut outResult: FT = inStartValue;
        outTree = (match &*outTree.clone() {
            Tree::NODE {
                key,
                value,
                left: __outTree_left,
                right: __outTree_right,
                ..
            } => {
                let mut new_value: Value;
                let mut new_left: metamodelica::Ref<Tree>;
                let mut new_right: metamodelica::Ref<Tree>;
                (new_left, outResult) = mapFold(__outTree_left.clone(), inFunc, outResult)?;
                (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult)?;
                (new_right, outResult) = mapFold(__outTree_right.clone(), inFunc, outResult)?;
                if !(referenceEq(&*(&*new_left), &*(var_field!((*outTree).left, Tree::NODE).clone())))
                    || !(referenceEq(&*(value.clone()), &*(&*new_value)))
                    || !(referenceEq(&*(&*new_right), &*(var_field!((*outTree).right, Tree::NODE).clone())))
                {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: key.clone(),
                        value: new_value,
                        height: var_field!((*outTree).height, Tree::NODE).clone(),
                        left: new_left,
                        right: new_right,
                    });
                }
                outTree
            }
            Tree::LEAF { key, value } => {
                let mut new_value: Value;
                (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult)?;
                if !(referenceEq(&*(value.clone()), &*(&*new_value))) {
                    assign_variant_field!(outTree => Tree::LEAF; value = new_value);
                }
                outTree
            }
            _ => inTree,
        });
        Ok((outTree, outResult))
    }

    pub(crate) fn new() -> metamodelica::Ref<Tree> {
        let mut outTree: metamodelica::Ref<Tree> =
            crate::OperatorOverloading::AvlTreePathPathEnv::Tree::interned_EMPTY();
        outTree
    }

    pub(crate) fn printNodeStr(mut inNode: &metamodelica::Ref<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr;
        outString = (match &**inNode {
            Tree::NODE {
                key: __inNode_key,
                value: __inNode_value,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*keyStr(__inNode_key.clone())?);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*valueStr(__inNode_value.clone())?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
            Tree::LEAF {
                key: __inNode_key,
                value: __inNode_value,
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*keyStr(__inNode_key.clone())?);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*valueStr(__inNode_value.clone())?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(outString)
    }

    pub(crate) fn printTreeStr(mut inTree: &metamodelica::Ref<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr;
        let mut left: metamodelica::Ref<Tree>;
        let mut right: metamodelica::Ref<Tree>;
        outString = (match &**inTree {
            Tree::EMPTY { .. } => literal!("EMPTY()"),
            Tree::LEAF { .. } => printNodeStr(inTree)?,
            Tree::NODE {
                left: __esc_left,
                right: __esc_right,
                ..
            } => {
                left = (*__esc_left).clone();
                right = (*__esc_right).clone();
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*printTreeStr2(
                        metamodelica::AsArg::as_arg(&left),
                        true,
                        &(literal!("")),
                    )?);
                    __mm_s.push_str(&*printNodeStr(inTree)?);
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*printTreeStr2(
                        metamodelica::AsArg::as_arg(&right),
                        false,
                        &(literal!("")),
                    )?);
                    ArcStr::from(__mm_s)
                }
            }
        });
        Ok(outString)
    }

    fn printTreeStr2(mut inTree: &metamodelica::Ref<Tree>, mut isLeft: bool, mut inIndent: &ArcStr) -> Result<ArcStr> {
        let mut outString: ArcStr;
        let mut left: Option<metamodelica::Ref<Tree>>;
        let mut right: Option<metamodelica::Ref<Tree>>;
        outString = (match &**inTree {
            Tree::NODE {
                left: __inTree_left,
                right: __inTree_right,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*printTreeStr2(
                    metamodelica::AsArg::as_arg(&__inTree_left),
                    true,
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*inIndent);
                        __mm_s.push_str(&*if (isLeft) {
                            literal!("     ")
                        } else {
                            literal!(" │   ")
                        });
                        ArcStr::from(__mm_s)
                    }),
                )?);
                __mm_s.push_str(&*inIndent);
                __mm_s.push_str(&*if (isLeft) { literal!(" ┌") } else { literal!(" └") });
                __mm_s.push_str(&*literal!("────"));
                __mm_s.push_str(&*printNodeStr(inTree)?);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*printTreeStr2(
                    metamodelica::AsArg::as_arg(&__inTree_right),
                    false,
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*inIndent);
                        __mm_s.push_str(&*if (isLeft) {
                            literal!(" │   ")
                        } else {
                            literal!("     ")
                        });
                        ArcStr::from(__mm_s)
                    }),
                )?);
                ArcStr::from(__mm_s)
            }
            Tree::LEAF { .. } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inIndent);
                __mm_s.push_str(&*if (isLeft) { literal!(" ┌") } else { literal!(" └") });
                __mm_s.push_str(&*literal!("────"));
                __mm_s.push_str(&*printNodeStr(inTree)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }
            _ => literal!(""),
        });
        Ok(outString)
    }

    fn referenceEqOrEmpty(mut t1: &metamodelica::Ref<Tree>, mut t2: &metamodelica::Ref<Tree>) -> bool {
        let mut b: bool;
        b = (::match_deref::match_deref! { match (t1, t2) {
            (Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => true,
            _ => referenceEq(&*(&**t1),&*(&**t2)),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        b
    }

    fn rotateLeft(mut inNode: metamodelica::Ref<Tree>) -> Result<metamodelica::Ref<Tree>> {
        let mut outNode: metamodelica::Ref<Tree> = inNode.clone();
        outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
            Deref @ Tree::NODE { right: child @ Deref @ Tree::NODE { .. }, left: __outNode_left, .. } => {
                let mut node: metamodelica::Ref<Tree>;
                node = setTreeLeftRight(outNode, __outNode_left.clone(), var_field!((**child).left, Tree::NODE).clone())?;
                setTreeLeftRight(child.clone(), node, var_field!((**child).right, Tree::NODE).clone())?
            },
            Deref @ Tree::NODE { right: child @ Deref @ Tree::LEAF { .. }, left: __outNode_left, .. } => {
                let mut node: metamodelica::Ref<Tree>;
                node = setTreeLeftRight(outNode, __outNode_left.clone(), crate::OperatorOverloading::AvlTreePathPathEnv::Tree::interned_EMPTY())?;
                setTreeLeftRight(child.clone(), node, crate::OperatorOverloading::AvlTreePathPathEnv::Tree::interned_EMPTY())?
            },
            _ => {
                inNode
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(outNode)
    }

    fn rotateRight(mut inNode: metamodelica::Ref<Tree>) -> Result<metamodelica::Ref<Tree>> {
        let mut outNode: metamodelica::Ref<Tree> = inNode.clone();
        outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
            Deref @ Tree::NODE { left: child @ Deref @ Tree::NODE { .. }, right: __outNode_right, .. } => {
                let mut node: metamodelica::Ref<Tree>;
                node = setTreeLeftRight(outNode, var_field!((**child).right, Tree::NODE).clone(), __outNode_right.clone())?;
                setTreeLeftRight(child.clone(), var_field!((**child).left, Tree::NODE).clone(), node)?
            },
            Deref @ Tree::NODE { left: child @ Deref @ Tree::LEAF { .. }, right: __outNode_right, .. } => {
                let mut node: metamodelica::Ref<Tree>;
                node = setTreeLeftRight(outNode, crate::OperatorOverloading::AvlTreePathPathEnv::Tree::interned_EMPTY(), __outNode_right.clone())?;
                setTreeLeftRight(child.clone(), crate::OperatorOverloading::AvlTreePathPathEnv::Tree::interned_EMPTY(), node)?
            },
            _ => {
                inNode
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(outNode)
    }

    pub(crate) fn setTreeLeftRight(
        mut orig: metamodelica::Ref<Tree>,
        mut left: metamodelica::Ref<Tree>,
        mut right: metamodelica::Ref<Tree>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut res: metamodelica::Ref<Tree>;
        res = (::match_deref::match_deref! { match &((orig.clone(), left.clone(), right.clone())) {
            (Deref @ Tree::NODE { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => metamodelica::Ref::new(Tree::LEAF { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone() }),
            (Deref @ Tree::LEAF { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => orig,
            (Deref @ Tree::NODE { .. }, _, _) => if (referenceEqOrEmpty(var_field!((*orig).left, Tree::NODE), &left) && referenceEqOrEmpty(var_field!((*orig).right, Tree::NODE), &right)) {orig} else {metamodelica::Ref::new(Tree::NODE { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone(), height: std::cmp::max(height(&left), height(&right)) + 1, left: left, right: right })},
            (Deref @ Tree::LEAF { .. }, _, _) => metamodelica::Ref::new(Tree::NODE { key: var_field!((*orig).key, Tree::LEAF).clone(), value: var_field!((*orig).value, Tree::LEAF).clone(), height: std::cmp::max(height(&left), height(&right)) + 1, left: left, right: right }),
            _ => return Err("match: no arm matched"),
        } });
        Ok(res)
    }

    pub(crate) fn smallestKey<'__b>(mut tree: &'__b metamodelica::Ref<Tree>) -> Result<Key> {
        '__tco: loop {
            ::match_deref::match_deref! { match tree {
                Deref @ Tree::NODE { right: Deref @ Tree::EMPTY { .. }, .. } => return Ok(var_field!((**tree).key, Tree::NODE).clone()),
                Deref @ Tree::NODE { .. } => { tree = var_field!((**tree).right, Tree::NODE); continue '__tco; },
                Deref @ Tree::LEAF { .. } => return Ok(var_field!((**tree).key, Tree::LEAF).clone()),
                _ => return Err("match: no arm matched"),
            } }
        }
    }

    pub(crate) fn toList<'__b>(
        mut inTree: &'__b metamodelica::Ref<Tree>,
        mut lst: metamodelica::List<(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>)>,
    ) -> metamodelica::List<(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>)> {
        '__tco: loop {
            match &**inTree {
                Tree::NODE { key, value, .. } => {
                    lst = toList(var_field!((**inTree).right, Tree::NODE), lst);
                    lst = metamodelica::cons((key.clone(), value.clone()), lst);
                    {
                        (inTree, lst) = (var_field!((**inTree).left, Tree::NODE), lst);
                        continue '__tco;
                    }
                }
                Tree::LEAF { key, value } => return metamodelica::cons((key.clone(), value.clone()), lst),
                _ => return lst,
            }
        }
    }

    pub(crate) fn update(
        mut tree: metamodelica::Ref<Tree>,
        mut key: &Key,
        mut value: &Value,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut outTree: metamodelica::Ref<Tree> =
            add(tree.clone(), key, value, &move |__a0: metamodelica::Ref<
                Absyn::Path,
            >,
                                                 __a1: metamodelica::Ref<
                Absyn::Path,
            >,
                                                 __a2: metamodelica::Ref<
                Absyn::Path,
            >|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(addConflictReplace(__a0, &__a1, &__a2))
            })?;
        Ok(outTree)
    }
}

pub mod AvlTreePathOperatorTypes {
    use super::*;
    pub type Key = metamodelica::Ref<Absyn::Path>;

    pub type Value = metamodelica::List<metamodelica::Ref<DAE::Type>>;

    pub(crate) fn keyStr(mut inKey: Key) -> Result<ArcStr> {
        let mut outString: ArcStr;
        outString = AbsynUtil::pathString(inKey, literal!("."), true, false)?;
        Ok(outString)
    }

    pub(crate) fn valueStr(mut inValue: Value) -> Result<ArcStr> {
        let mut outString: ArcStr;
        outString = TypesDump::unparseType(metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: inValue }))?;
        Ok(outString)
    }

    pub(crate) fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> Result<i32> {
        let mut outResult: i32;
        outResult = AbsynUtil::pathCompareNoQual(inKey1, inKey2)?;
        Ok(outResult)
    }

    pub use addConflictKeep as addConflictDefault;

    pub type ConflictFunc = std::sync::Arc<dyn ::std::ops::Fn(Value, Value, Key) -> Result<Value> + 'static>;

    /// The binary tree data structure.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub(crate) enum Tree {
        NODE {
            /// The key of the node.
            key: Key,
            value: Value,
            /// Height of tree, used for balancing
            height: i32,
            /// Left subtree.
            left: metamodelica::Ref<Tree>,
            /// Right subtree.
            right: metamodelica::Ref<Tree>,
        },
        LEAF {
            /// The key of the node.
            key: Key,
            value: Value,
        },
        EMPTY,
    }
    impl metamodelica::gc::MMTrace for Tree {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Tree::NODE {
                    key,
                    value,
                    height,
                    left,
                    right,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(key, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(height, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(left, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(right, __mmv)?;
                    Ok(())
                }
                Tree::LEAF { key, value } => {
                    metamodelica::gc::MMTrace::mm_accept(key, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                    Ok(())
                }
                Tree::EMPTY => Ok(()),
            }
        }
    }
    impl Tree {
        pub fn interned_EMPTY() -> metamodelica::Ref<Tree> {
            thread_local! {
                static INTERNED: metamodelica::Ref<Tree> = metamodelica::Ref::new(Tree::EMPTY);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_EMPTY() -> metamodelica::Ref<Tree> {
        Tree::interned_EMPTY()
    }
    impl Default for Tree {
        fn default() -> Self {
            Self::EMPTY
        }
    }
    pub(crate) use self::Tree::{EMPTY, LEAF, NODE};

    pub type ValueNode = metamodelica::Ref<Absyn::Path>;

    pub(crate) fn add(
        mut inTree: metamodelica::Ref<Tree>,
        mut inKey: &Key,
        mut inValue: &Value,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<Absyn::Path>,
        ) -> Result<metamodelica::List<metamodelica::Ref<DAE::Type>>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = inTree;
        tree = (match &*tree.clone() {
            Tree::EMPTY { .. } => metamodelica::Ref::new(Tree::LEAF {
                key: inKey.clone(),
                value: inValue.clone(),
            }),
            Tree::NODE { key, .. } => {
                let mut value: Value;
                let mut key_comp: i32;
                key_comp = keyCompare(inKey.clone(), key.clone())?;
                if key_comp == -1 {
                    assign_variant_field!(tree => Tree::NODE; left = add(var_field!((*tree).left, Tree::NODE).clone(), inKey, inValue, conflictFunc)?);
                } else if key_comp == 1 {
                    assign_variant_field!(tree => Tree::NODE; right = add(var_field!((*tree).right, Tree::NODE).clone(), inKey, inValue, conflictFunc)?);
                } else {
                    value = conflictFunc(
                        inValue.clone(),
                        var_field!((*tree).value, Tree::NODE).clone(),
                        key.clone(),
                    )?;
                    if !(metamodelica::ReferenceEq::reference_eq(
                        &(var_field!((*tree).value, Tree::NODE).clone()),
                        &(value),
                    )) {
                        assign_variant_field!(tree => Tree::NODE; value = value);
                    }
                }
                if (key_comp == 0) { tree } else { balance(tree)? }
            }
            Tree::LEAF { key: __tree_key, .. } => {
                let mut value: Value;
                let mut key_comp: i32;
                let mut outTree: metamodelica::Ref<Tree>;
                key_comp = keyCompare(inKey.clone(), __tree_key.clone())?;
                if key_comp == -1 {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: metamodelica::Ref::new(Tree::LEAF {
                            key: inKey.clone(),
                            value: inValue.clone(),
                        }),
                        right: crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::interned_EMPTY(),
                        right: metamodelica::Ref::new(Tree::LEAF {
                            key: inKey.clone(),
                            value: inValue.clone(),
                        }),
                    });
                } else {
                    value = conflictFunc(
                        inValue.clone(),
                        var_field!((*tree).value, Tree::LEAF).clone(),
                        var_field!((*tree).key, Tree::LEAF).clone(),
                    )?;
                    if !(metamodelica::ReferenceEq::reference_eq(
                        &(var_field!((*tree).value, Tree::LEAF).clone()),
                        &(value),
                    )) {
                        assign_variant_field!(tree => Tree::LEAF; value = value);
                    }
                    outTree = tree;
                }
                if (key_comp == 0) { outTree } else { balance(outTree)? }
            }
        });
        Ok(tree)
    }

    pub(crate) fn addConflictFail(mut newValue: &Value, mut oldValue: &Value, mut key: &Key) -> Result<Value> {
        let mut value: Value;
        return Err("fail");
        Ok(value)
    }

    pub fn addConflictKeep(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Value {
        let mut value: Value = oldValue;
        value
    }

    pub(crate) fn addConflictReplace(mut newValue: Value, mut oldValue: &Value, mut key: &Key) -> Value {
        let mut value: Value = newValue;
        value
    }

    pub(crate) fn addList(
        mut tree: metamodelica::Ref<Tree>,
        mut inValues: &metamodelica::List<(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
        )>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<Absyn::Path>,
        ) -> Result<metamodelica::List<metamodelica::Ref<DAE::Type>>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = tree;
        let mut key: Key;
        let mut value: Value;
        for mut t in &**inValues {
            (key, value) = t.clone();
            tree = add(tree, &key, &value, conflictFunc)?;
        }
        Ok(tree)
    }

    pub(crate) fn addUpdate(
        mut tree: metamodelica::Ref<Tree>,
        mut key: &Key,
        mut r#fn: &dyn ::std::ops::Fn(
            Option<metamodelica::List<metamodelica::Ref<DAE::Type>>>,
        ) -> Result<metamodelica::List<metamodelica::Ref<DAE::Type>>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        pub type UpdateFn = std::sync::Arc<
            dyn ::std::ops::Fn(Option<metamodelica::List<metamodelica::Ref<DAE::Type>>>) -> Result<Value> + 'static,
        >;

        let mut tree: metamodelica::Ref<Tree> = tree;
        let mut key_comp: i32;
        let mut new_tree: metamodelica::Ref<Tree>;
        tree = (match &*tree {
            Tree::EMPTY { .. } => metamodelica::Ref::new(Tree::LEAF {
                key: key.clone(),
                value: r#fn(None)?,
            }),
            Tree::NODE { key: __tree_key, .. } => {
                key_comp = keyCompare(key.clone(), __tree_key.clone())?;
                if key_comp == -1 {
                    assign_variant_field!(tree => Tree::NODE; left = addUpdate(var_field!((*tree).left, Tree::NODE).clone(), key, r#fn)?);
                } else if key_comp == 1 {
                    assign_variant_field!(tree => Tree::NODE; right = addUpdate(var_field!((*tree).right, Tree::NODE).clone(), key, r#fn)?);
                } else {
                    assign_variant_field!(tree => Tree::NODE; value = r#fn(Some(var_field!((*tree).value, Tree::NODE).clone()))?);
                }
                if (key_comp == 0) { tree } else { balance(tree)? }
            }
            Tree::LEAF { key: __tree_key, .. } => {
                key_comp = keyCompare(key.clone(), __tree_key.clone())?;
                if key_comp == -1 {
                    new_tree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: metamodelica::Ref::new(Tree::LEAF {
                            key: key.clone(),
                            value: r#fn(None)?,
                        }),
                        right: crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    new_tree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::interned_EMPTY(),
                        right: metamodelica::Ref::new(Tree::LEAF {
                            key: key.clone(),
                            value: r#fn(None)?,
                        }),
                    });
                } else {
                    assign_variant_field!(tree => Tree::LEAF; value = r#fn(Some(var_field!((*tree).value, Tree::LEAF).clone()))?);
                    new_tree = tree;
                }
                if (key_comp == 0) { new_tree } else { balance(new_tree)? }
            }
        });
        Ok(tree)
    }

    fn balance(mut inTree: metamodelica::Ref<Tree>) -> Result<metamodelica::Ref<Tree>> {
        let mut outTree: metamodelica::Ref<Tree> = inTree.clone();
        outTree = (match &*outTree {
            Tree::LEAF { .. } => inTree,
            Tree::NODE {
                left: __outTree_left,
                right: __outTree_right,
                ..
            } => {
                let mut lh: i32;
                let mut rh: i32;
                let mut diff: i32;
                let mut balanced_tree: metamodelica::Ref<Tree>;
                lh = height(metamodelica::AsArg::as_arg(&__outTree_left));
                rh = height(metamodelica::AsArg::as_arg(&__outTree_right));
                diff = lh - rh;
                if diff < -1 {
                    balanced_tree = if (calculateBalance(var_field!((*outTree).right, Tree::NODE)) > 0) {
                        rotateLeft(setTreeLeftRight(
                            outTree.clone(),
                            var_field!((*outTree).left, Tree::NODE).clone(),
                            rotateRight(var_field!((*outTree).right, Tree::NODE).clone())?,
                        )?)?
                    } else {
                        rotateLeft(outTree)?
                    };
                } else if diff > 1 {
                    balanced_tree = if (calculateBalance(var_field!((*outTree).left, Tree::NODE)) < 0) {
                        rotateRight(setTreeLeftRight(
                            outTree.clone(),
                            rotateLeft(var_field!((*outTree).left, Tree::NODE).clone())?,
                            var_field!((*outTree).right, Tree::NODE).clone(),
                        )?)?
                    } else {
                        rotateRight(outTree)?
                    };
                } else if var_field!((*outTree).height, Tree::NODE).clone() != std::cmp::max(lh, rh) + 1 {
                    assign_variant_field!(outTree => Tree::NODE; height = std::cmp::max(lh, rh) + 1);
                    balanced_tree = outTree;
                } else {
                    balanced_tree = outTree;
                }
                balanced_tree
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(outTree)
    }

    fn calculateBalance(mut inNode: &metamodelica::Ref<Tree>) -> i32 {
        let mut outBalance: i32;
        outBalance = (match &**inNode {
            Tree::NODE {
                left: __inNode_left,
                right: __inNode_right,
                ..
            } => {
                height(metamodelica::AsArg::as_arg(&__inNode_left))
                    - height(metamodelica::AsArg::as_arg(&__inNode_right))
            }
            Tree::LEAF { .. } => 0,
            _ => 0,
        });
        outBalance
    }

    pub(crate) fn fold<'__b, FT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut inTree: &'__b metamodelica::Ref<Tree>,
        mut inFunc: &'__b dyn ::std::ops::Fn(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            FT,
        ) -> Result<FT>,
        mut inStartValue: FT,
    ) -> Result<FT> {
        pub type FoldFunc<FT: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<FT> + 'static>;

        let mut outResult: FT = inStartValue;
        outResult = (match &**inTree {
            Tree::NODE { key, value, .. } => {
                outResult = fold(var_field!((**inTree).left, Tree::NODE), inFunc, outResult)?;
                outResult = inFunc(key.clone(), value.clone(), outResult)?;
                outResult = fold(var_field!((**inTree).right, Tree::NODE), inFunc, outResult)?;
                outResult
            }
            Tree::LEAF { key, value } => {
                outResult = inFunc(key.clone(), value.clone(), outResult)?;
                outResult
            }
            _ => outResult,
        });
        Ok(outResult)
    }

    pub(crate) fn foldCond<FT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut tree: &metamodelica::Ref<Tree>,
        mut foldFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            FT,
        ) -> Result<(FT, bool)>,
        mut value: FT,
    ) -> Result<FT> {
        pub type FoldFunc<FT: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<(FT, bool)> + 'static>;

        let mut value: FT = value;
        value = (match &**tree {
            Tree::NODE {
                key: __tree_key,
                left: __tree_left,
                right: __tree_right,
                value: __tree_value,
                ..
            } => {
                let mut c: bool;
                (value, c) = foldFunc(__tree_key.clone(), __tree_value.clone(), value)?;
                if c {
                    value = foldCond(metamodelica::AsArg::as_arg(&__tree_left), foldFunc, value)?;
                    value = foldCond(metamodelica::AsArg::as_arg(&__tree_right), foldFunc, value)?;
                }
                value
            }
            Tree::LEAF {
                key: __tree_key,
                value: __tree_value,
            } => {
                let mut c: bool;
                (value, c) = foldFunc(__tree_key.clone(), __tree_value.clone(), value)?;
                value
            }
            _ => value,
        });
        Ok(value)
    }

    pub(crate) fn fold_2<
        FT1: Clone + 'static + metamodelica::gc::MMTrace,
        FT2: Clone + 'static + metamodelica::gc::MMTrace,
    >(
        mut tree: &metamodelica::Ref<Tree>,
        mut foldFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            FT1,
            FT2,
        ) -> Result<(FT1, FT2)>,
        mut foldArg1: FT1,
        mut foldArg2: FT2,
    ) -> Result<(FT1, FT2)> {
        pub type FoldFunc<FT1: Clone + 'static, FT2: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

        let mut foldArg1: FT1 = foldArg1;
        let mut foldArg2: FT2 = foldArg2;
        let () = (match &**tree {
            Tree::NODE {
                key: __tree_key,
                left: __tree_left,
                right: __tree_right,
                value: __tree_value,
                ..
            } => {
                (foldArg1, foldArg2) = fold_2(metamodelica::AsArg::as_arg(&__tree_left), foldFunc, foldArg1, foldArg2)?;
                (foldArg1, foldArg2) = foldFunc(__tree_key.clone(), __tree_value.clone(), foldArg1, foldArg2)?;
                (foldArg1, foldArg2) =
                    fold_2(metamodelica::AsArg::as_arg(&__tree_right), foldFunc, foldArg1, foldArg2)?;
                ()
            }
            Tree::LEAF {
                key: __tree_key,
                value: __tree_value,
            } => {
                (foldArg1, foldArg2) = foldFunc(__tree_key.clone(), __tree_value.clone(), foldArg1, foldArg2)?;
                ()
            }
            _ => (),
        });
        Ok((foldArg1, foldArg2))
    }

    pub(crate) fn forEach(
        mut tree: &metamodelica::Ref<Tree>,
        mut func: &dyn ::std::ops::Fn(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
        ) -> Result<()>,
    ) -> Result<()> {
        pub type EachFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<()> + 'static>;

        let () = (match &**tree {
            Tree::NODE {
                key: __tree_key,
                left: __tree_left,
                right: __tree_right,
                value: __tree_value,
                ..
            } => {
                forEach(metamodelica::AsArg::as_arg(&__tree_left), func)?;
                func(__tree_key.clone(), __tree_value.clone())?;
                forEach(metamodelica::AsArg::as_arg(&__tree_right), func)?;
                ()
            }
            Tree::LEAF {
                key: __tree_key,
                value: __tree_value,
            } => {
                func(__tree_key.clone(), __tree_value.clone())?;
                ()
            }
            Tree::EMPTY { .. } => (),
        });
        Ok(())
    }

    pub(crate) fn fromList(
        mut inValues: &metamodelica::List<(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
        )>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<Absyn::Path>,
        ) -> Result<metamodelica::List<metamodelica::Ref<DAE::Type>>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> =
            crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::interned_EMPTY();
        let mut key: Key;
        let mut value: Value;
        for mut t in &**inValues {
            (key, value) = t.clone();
            tree = add(tree, &key, &value, conflictFunc)?;
        }
        Ok(tree)
    }

    pub(crate) fn get<'__b>(mut tree: &'__b metamodelica::Ref<Tree>, mut key: Key) -> Result<Value> {
        let mut value: Value;
        let mut k: Key;
        k = (match &**tree {
            Tree::NODE { .. } => var_field!((**tree).key, Tree::NODE).clone(),
            Tree::LEAF { .. } => var_field!((**tree).key, Tree::LEAF).clone(),
            _ => return Err("match: no arm matched"),
        });
        value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k)?, tree.clone())) {
            (0, Deref @ Tree::LEAF { .. }) => var_field!((**tree).value, Tree::LEAF).clone(),
            (0, Deref @ Tree::NODE { .. }) => var_field!((**tree).value, Tree::NODE).clone(),
            (1, Deref @ Tree::NODE { .. }) => get(var_field!((**tree).right, Tree::NODE), key)?,
            ((-1), Deref @ Tree::NODE { .. }) => get(var_field!((**tree).left, Tree::NODE), key)?,
            _ => return Err("match: no arm matched"),
        } });
        Ok(value)
    }

    pub(crate) fn getOpt<'__b>(
        mut tree: &'__b metamodelica::Ref<Tree>,
        mut key: Key,
    ) -> Result<Option<metamodelica::List<metamodelica::Ref<DAE::Type>>>> {
        '__tco: loop {
            let mut k: Key;
            k = (match &**tree {
                Tree::NODE { .. } => var_field!((**tree).key, Tree::NODE).clone(),
                Tree::LEAF { .. } => var_field!((**tree).key, Tree::LEAF).clone(),
                _ => key.clone(),
            });
            ::match_deref::match_deref! { match &((keyCompare(key.clone(), k)?, tree.clone())) {
                (0, Deref @ Tree::LEAF { .. }) => return Ok(Some(var_field!((**tree).value, Tree::LEAF).clone())),
                (0, Deref @ Tree::NODE { .. }) => return Ok(Some(var_field!((**tree).value, Tree::NODE).clone())),
                (1, Deref @ Tree::NODE { .. }) => { (tree, key) = (var_field!((**tree).right, Tree::NODE), key); continue '__tco; },
                ((-1), Deref @ Tree::NODE { .. }) => { (tree, key) = (var_field!((**tree).left, Tree::NODE), key); continue '__tco; },
                _ => return Ok(None),
                _ => return Err("match: no arm matched"),
            } }
        }
    }

    pub(crate) fn hasKey(mut inTree: metamodelica::Ref<Tree>, mut inKey: Key) -> Result<bool> {
        let mut comp: bool = false;
        let mut key: Key;
        let mut key_comp: i32;
        let mut tree: metamodelica::Ref<Tree>;
        key = (match &*inTree {
            Tree::NODE { key: __inTree_key, .. } => __inTree_key.clone(),
            Tree::LEAF { key: __inTree_key, .. } => __inTree_key.clone(),
            Tree::EMPTY { .. } => {
                return Ok(comp);
                return Err("fail");
            }
        });
        key_comp = keyCompare(inKey.clone(), key)?;
        comp = (::match_deref::match_deref! { match &((key_comp, inTree)) {
            (0, _) => true,
            (1, Deref @ Tree::NODE { right: __esc_tree, .. }) => {
                tree = (*__esc_tree).clone();
                hasKey(tree.clone(), inKey)?
            },
            ((-1), Deref @ Tree::NODE { left: __esc_tree, .. }) => {
                tree = (*__esc_tree).clone();
                hasKey(tree.clone(), inKey)?
            },
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(comp)
    }

    fn height(mut inNode: &metamodelica::Ref<Tree>) -> i32 {
        let mut outHeight: i32;
        outHeight = (match &**inNode {
            Tree::NODE {
                height: __inNode_height,
                ..
            } => __inNode_height.clone(),
            Tree::LEAF { .. } => 1,
            _ => 0,
        });
        outHeight
    }

    pub(crate) fn intersection() -> Result<()> {
        return Err("fail");
        Ok(())
    }

    pub(crate) fn isEmpty(mut tree: &metamodelica::Ref<Tree>) -> bool {
        let mut isEmpty: bool;
        isEmpty = (match &**tree {
            Tree::EMPTY { .. } => true,
            _ => false,
        });
        isEmpty
    }

    pub(crate) fn join<'__b>(
        mut tree: metamodelica::Ref<Tree>,
        mut treeToJoin: &'__b metamodelica::Ref<Tree>,
        mut conflictFunc: &'__b dyn ::std::ops::Fn(
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            metamodelica::Ref<Absyn::Path>,
        ) -> Result<metamodelica::List<metamodelica::Ref<DAE::Type>>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        '__tco: loop {
            match &**treeToJoin {
                Tree::EMPTY { .. } => return Ok(tree),
                Tree::NODE { .. } => {
                    tree = add(
                        tree,
                        var_field!((**treeToJoin).key, Tree::NODE),
                        var_field!((**treeToJoin).value, Tree::NODE),
                        conflictFunc,
                    )?;
                    tree = join(tree, var_field!((**treeToJoin).left, Tree::NODE), conflictFunc)?;
                    {
                        (tree, treeToJoin, conflictFunc) =
                            (tree, var_field!((**treeToJoin).right, Tree::NODE), conflictFunc);
                        continue '__tco;
                    }
                }
                Tree::LEAF { .. } => {
                    return Ok(add(
                        tree,
                        var_field!((**treeToJoin).key, Tree::LEAF),
                        var_field!((**treeToJoin).value, Tree::LEAF),
                        conflictFunc,
                    )?);
                }
            }
        }
    }

    pub(crate) fn listKeys<'__b>(
        mut tree: &'__b metamodelica::Ref<Tree>,
        mut lst: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    ) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
        '__tco: loop {
            match &**tree {
                Tree::NODE { key, .. } => {
                    lst = listKeys(var_field!((**tree).right, Tree::NODE), lst);
                    lst = metamodelica::cons(key.clone(), lst);
                    {
                        (tree, lst) = (var_field!((**tree).left, Tree::NODE), lst);
                        continue '__tco;
                    }
                }
                Tree::LEAF { key, .. } => return metamodelica::cons(key.clone(), lst),
                _ => return lst,
            }
        }
    }

    pub(crate) fn listKeysReverse<'__b>(
        mut inTree: &'__b metamodelica::Ref<Tree>,
        mut lst: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    ) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
        '__tco: loop {
            match &**inTree {
                Tree::LEAF { .. } => return metamodelica::cons(var_field!((**inTree).key, Tree::LEAF).clone(), lst),
                Tree::NODE { .. } => {
                    lst = listKeysReverse(var_field!((**inTree).left, Tree::NODE), lst);
                    lst = metamodelica::cons(var_field!((**inTree).key, Tree::NODE).clone(), lst);
                    {
                        (inTree, lst) = (var_field!((**inTree).right, Tree::NODE), lst);
                        continue '__tco;
                    }
                }
                _ => return lst,
            }
        }
    }

    pub(crate) fn listValues<'__b>(
        mut tree: &'__b metamodelica::Ref<Tree>,
        mut lst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Type>>>,
    ) -> metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Type>>> {
        '__tco: loop {
            match &**tree {
                Tree::NODE { value, .. } => {
                    lst = listValues(var_field!((**tree).right, Tree::NODE), lst);
                    lst = metamodelica::cons(value.clone(), lst);
                    {
                        (tree, lst) = (var_field!((**tree).left, Tree::NODE), lst);
                        continue '__tco;
                    }
                }
                Tree::LEAF { value, .. } => return metamodelica::cons(value.clone(), lst),
                _ => return lst,
            }
        }
    }

    pub(crate) fn map(
        mut inTree: metamodelica::Ref<Tree>,
        mut inFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
        ) -> Result<metamodelica::List<metamodelica::Ref<DAE::Type>>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<Value> + 'static>;

        let mut outTree: metamodelica::Ref<Tree> = inTree.clone();
        outTree = (match &*outTree.clone() {
            Tree::NODE {
                key,
                value,
                left: __outTree_left,
                right: __outTree_right,
                ..
            } => {
                let mut new_value: Value;
                let mut new_left: metamodelica::Ref<Tree>;
                let mut new_right: metamodelica::Ref<Tree>;
                new_left = map(__outTree_left.clone(), inFunc)?;
                new_value = inFunc(key.clone(), value.clone())?;
                new_right = map(__outTree_right.clone(), inFunc)?;
                if !(referenceEq(&*(&*new_left), &*(var_field!((*outTree).left, Tree::NODE).clone())))
                    || !(metamodelica::ReferenceEq::reference_eq(&(value.clone()), &(new_value)))
                    || !(referenceEq(&*(&*new_right), &*(var_field!((*outTree).right, Tree::NODE).clone())))
                {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: key.clone(),
                        value: new_value,
                        height: var_field!((*outTree).height, Tree::NODE).clone(),
                        left: new_left,
                        right: new_right,
                    });
                }
                outTree
            }
            Tree::LEAF { key, value } => {
                let mut new_value: Value;
                new_value = inFunc(key.clone(), value.clone())?;
                if !(metamodelica::ReferenceEq::reference_eq(&(value.clone()), &(new_value))) {
                    assign_variant_field!(outTree => Tree::LEAF; value = new_value);
                }
                outTree
            }
            _ => inTree,
        });
        Ok(outTree)
    }

    pub(crate) fn mapFold<FT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut inTree: metamodelica::Ref<Tree>,
        mut inFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            FT,
        ) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Type>>, FT)>,
        mut inStartValue: FT,
    ) -> Result<(metamodelica::Ref<Tree>, FT)> {
        pub type MapFunc<FT: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<Value> + 'static>;

        let mut outTree: metamodelica::Ref<Tree> = inTree.clone();
        let mut outResult: FT = inStartValue;
        outTree = (match &*outTree.clone() {
            Tree::NODE {
                key,
                value,
                left: __outTree_left,
                right: __outTree_right,
                ..
            } => {
                let mut new_value: Value;
                let mut new_left: metamodelica::Ref<Tree>;
                let mut new_right: metamodelica::Ref<Tree>;
                (new_left, outResult) = mapFold(__outTree_left.clone(), inFunc, outResult)?;
                (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult)?;
                (new_right, outResult) = mapFold(__outTree_right.clone(), inFunc, outResult)?;
                if !(referenceEq(&*(&*new_left), &*(var_field!((*outTree).left, Tree::NODE).clone())))
                    || !(metamodelica::ReferenceEq::reference_eq(&(value.clone()), &(new_value)))
                    || !(referenceEq(&*(&*new_right), &*(var_field!((*outTree).right, Tree::NODE).clone())))
                {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: key.clone(),
                        value: new_value,
                        height: var_field!((*outTree).height, Tree::NODE).clone(),
                        left: new_left,
                        right: new_right,
                    });
                }
                outTree
            }
            Tree::LEAF { key, value } => {
                let mut new_value: Value;
                (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult)?;
                if !(metamodelica::ReferenceEq::reference_eq(&(value.clone()), &(new_value))) {
                    assign_variant_field!(outTree => Tree::LEAF; value = new_value);
                }
                outTree
            }
            _ => inTree,
        });
        Ok((outTree, outResult))
    }

    pub(crate) fn new() -> metamodelica::Ref<Tree> {
        let mut outTree: metamodelica::Ref<Tree> =
            crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::interned_EMPTY();
        outTree
    }

    pub(crate) fn printNodeStr(mut inNode: &metamodelica::Ref<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr;
        outString = (match &**inNode {
            Tree::NODE {
                key: __inNode_key,
                value: __inNode_value,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*keyStr(__inNode_key.clone())?);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*valueStr(__inNode_value.clone())?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
            Tree::LEAF {
                key: __inNode_key,
                value: __inNode_value,
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*keyStr(__inNode_key.clone())?);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*valueStr(__inNode_value.clone())?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(outString)
    }

    pub(crate) fn printTreeStr(mut inTree: &metamodelica::Ref<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr;
        let mut left: metamodelica::Ref<Tree>;
        let mut right: metamodelica::Ref<Tree>;
        outString = (match &**inTree {
            Tree::EMPTY { .. } => literal!("EMPTY()"),
            Tree::LEAF { .. } => printNodeStr(inTree)?,
            Tree::NODE {
                left: __esc_left,
                right: __esc_right,
                ..
            } => {
                left = (*__esc_left).clone();
                right = (*__esc_right).clone();
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*printTreeStr2(
                        metamodelica::AsArg::as_arg(&left),
                        true,
                        &(literal!("")),
                    )?);
                    __mm_s.push_str(&*printNodeStr(inTree)?);
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*printTreeStr2(
                        metamodelica::AsArg::as_arg(&right),
                        false,
                        &(literal!("")),
                    )?);
                    ArcStr::from(__mm_s)
                }
            }
        });
        Ok(outString)
    }

    fn printTreeStr2(mut inTree: &metamodelica::Ref<Tree>, mut isLeft: bool, mut inIndent: &ArcStr) -> Result<ArcStr> {
        let mut outString: ArcStr;
        let mut left: Option<metamodelica::Ref<Tree>>;
        let mut right: Option<metamodelica::Ref<Tree>>;
        outString = (match &**inTree {
            Tree::NODE {
                left: __inTree_left,
                right: __inTree_right,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*printTreeStr2(
                    metamodelica::AsArg::as_arg(&__inTree_left),
                    true,
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*inIndent);
                        __mm_s.push_str(&*if (isLeft) {
                            literal!("     ")
                        } else {
                            literal!(" │   ")
                        });
                        ArcStr::from(__mm_s)
                    }),
                )?);
                __mm_s.push_str(&*inIndent);
                __mm_s.push_str(&*if (isLeft) { literal!(" ┌") } else { literal!(" └") });
                __mm_s.push_str(&*literal!("────"));
                __mm_s.push_str(&*printNodeStr(inTree)?);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*printTreeStr2(
                    metamodelica::AsArg::as_arg(&__inTree_right),
                    false,
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*inIndent);
                        __mm_s.push_str(&*if (isLeft) {
                            literal!(" │   ")
                        } else {
                            literal!("     ")
                        });
                        ArcStr::from(__mm_s)
                    }),
                )?);
                ArcStr::from(__mm_s)
            }
            Tree::LEAF { .. } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inIndent);
                __mm_s.push_str(&*if (isLeft) { literal!(" ┌") } else { literal!(" └") });
                __mm_s.push_str(&*literal!("────"));
                __mm_s.push_str(&*printNodeStr(inTree)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }
            _ => literal!(""),
        });
        Ok(outString)
    }

    fn referenceEqOrEmpty(mut t1: &metamodelica::Ref<Tree>, mut t2: &metamodelica::Ref<Tree>) -> bool {
        let mut b: bool;
        b = (::match_deref::match_deref! { match (t1, t2) {
            (Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => true,
            _ => referenceEq(&*(&**t1),&*(&**t2)),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        b
    }

    fn rotateLeft(mut inNode: metamodelica::Ref<Tree>) -> Result<metamodelica::Ref<Tree>> {
        let mut outNode: metamodelica::Ref<Tree> = inNode.clone();
        outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
            Deref @ Tree::NODE { right: child @ Deref @ Tree::NODE { .. }, left: __outNode_left, .. } => {
                let mut node: metamodelica::Ref<Tree>;
                node = setTreeLeftRight(outNode, __outNode_left.clone(), var_field!((**child).left, Tree::NODE).clone())?;
                setTreeLeftRight(child.clone(), node, var_field!((**child).right, Tree::NODE).clone())?
            },
            Deref @ Tree::NODE { right: child @ Deref @ Tree::LEAF { .. }, left: __outNode_left, .. } => {
                let mut node: metamodelica::Ref<Tree>;
                node = setTreeLeftRight(outNode, __outNode_left.clone(), crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::interned_EMPTY())?;
                setTreeLeftRight(child.clone(), node, crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::interned_EMPTY())?
            },
            _ => {
                inNode
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(outNode)
    }

    fn rotateRight(mut inNode: metamodelica::Ref<Tree>) -> Result<metamodelica::Ref<Tree>> {
        let mut outNode: metamodelica::Ref<Tree> = inNode.clone();
        outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
            Deref @ Tree::NODE { left: child @ Deref @ Tree::NODE { .. }, right: __outNode_right, .. } => {
                let mut node: metamodelica::Ref<Tree>;
                node = setTreeLeftRight(outNode, var_field!((**child).right, Tree::NODE).clone(), __outNode_right.clone())?;
                setTreeLeftRight(child.clone(), var_field!((**child).left, Tree::NODE).clone(), node)?
            },
            Deref @ Tree::NODE { left: child @ Deref @ Tree::LEAF { .. }, right: __outNode_right, .. } => {
                let mut node: metamodelica::Ref<Tree>;
                node = setTreeLeftRight(outNode, crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::interned_EMPTY(), __outNode_right.clone())?;
                setTreeLeftRight(child.clone(), crate::OperatorOverloading::AvlTreePathOperatorTypes::Tree::interned_EMPTY(), node)?
            },
            _ => {
                inNode
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(outNode)
    }

    pub(crate) fn setTreeLeftRight(
        mut orig: metamodelica::Ref<Tree>,
        mut left: metamodelica::Ref<Tree>,
        mut right: metamodelica::Ref<Tree>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut res: metamodelica::Ref<Tree>;
        res = (::match_deref::match_deref! { match &((orig.clone(), left.clone(), right.clone())) {
            (Deref @ Tree::NODE { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => metamodelica::Ref::new(Tree::LEAF { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone() }),
            (Deref @ Tree::LEAF { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => orig,
            (Deref @ Tree::NODE { .. }, _, _) => if (referenceEqOrEmpty(var_field!((*orig).left, Tree::NODE), &left) && referenceEqOrEmpty(var_field!((*orig).right, Tree::NODE), &right)) {orig} else {metamodelica::Ref::new(Tree::NODE { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone(), height: std::cmp::max(height(&left), height(&right)) + 1, left: left, right: right })},
            (Deref @ Tree::LEAF { .. }, _, _) => metamodelica::Ref::new(Tree::NODE { key: var_field!((*orig).key, Tree::LEAF).clone(), value: var_field!((*orig).value, Tree::LEAF).clone(), height: std::cmp::max(height(&left), height(&right)) + 1, left: left, right: right }),
            _ => return Err("match: no arm matched"),
        } });
        Ok(res)
    }

    pub(crate) fn smallestKey<'__b>(mut tree: &'__b metamodelica::Ref<Tree>) -> Result<Key> {
        '__tco: loop {
            ::match_deref::match_deref! { match tree {
                Deref @ Tree::NODE { right: Deref @ Tree::EMPTY { .. }, .. } => return Ok(var_field!((**tree).key, Tree::NODE).clone()),
                Deref @ Tree::NODE { .. } => { tree = var_field!((**tree).right, Tree::NODE); continue '__tco; },
                Deref @ Tree::LEAF { .. } => return Ok(var_field!((**tree).key, Tree::LEAF).clone()),
                _ => return Err("match: no arm matched"),
            } }
        }
    }

    pub(crate) fn toList<'__b>(
        mut inTree: &'__b metamodelica::Ref<Tree>,
        mut lst: metamodelica::List<(
            metamodelica::Ref<Absyn::Path>,
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
        )>,
    ) -> metamodelica::List<(
        metamodelica::Ref<Absyn::Path>,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
    )> {
        '__tco: loop {
            match &**inTree {
                Tree::NODE { key, value, .. } => {
                    lst = toList(var_field!((**inTree).right, Tree::NODE), lst);
                    lst = metamodelica::cons((key.clone(), value.clone()), lst);
                    {
                        (inTree, lst) = (var_field!((**inTree).left, Tree::NODE), lst);
                        continue '__tco;
                    }
                }
                Tree::LEAF { key, value } => return metamodelica::cons((key.clone(), value.clone()), lst),
                _ => return lst,
            }
        }
    }

    pub(crate) fn update(
        mut tree: metamodelica::Ref<Tree>,
        mut key: &Key,
        mut value: &Value,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut outTree: metamodelica::Ref<Tree> =
            add(tree.clone(), key, value, &move |__a0: metamodelica::List<
                metamodelica::Ref<DAE::Type>,
            >,
                                                 __a1: metamodelica::List<
                metamodelica::Ref<DAE::Type>,
            >,
                                                 __a2: metamodelica::Ref<
                Absyn::Path,
            >|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(addConflictReplace(__a0, &__a1, &__a2))
            })?;
        Ok(outTree)
    }
}

fn getOperatorFuncsOrEmptySingleTy(
    mut cache: FCore::Cache,
    mut env: &FCore::Graph,
    mut ty: &metamodelica::Ref<DAE::Type>,
    mut opName: ArcStr,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Type>>)> {
    let mut cache: FCore::Cache = cache;
    let mut funcs: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut pathIn: metamodelica::Ref<Absyn::Path>;
    let mut opNamePath: metamodelica::Ref<Absyn::Path>;
    let mut operatorCl: metamodelica::Ref<SCode::Element>;
    let mut recordEnv: FCore::Graph;
    let mut operEnv: FCore::Graph;
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut scalarType: metamodelica::Ref<DAE::Type>;
    let mut tree1: metamodelica::Ref<AvlTreePathPathEnv::Tree>;
    let mut tree2: metamodelica::Ref<AvlTreePathOperatorTypes::Tree>;
    let mut trees: (
        metamodelica::Ref<AvlTreePathPathEnv::Tree>,
        metamodelica::Ref<AvlTreePathOperatorTypes::Tree>,
    );
    scalarType = Types::arrayElementType(ty);
    pathIn = AbsynUtil::makeFullyQualified(getRecordPath(&scalarType)?);
    trees = crate::Globals::operatorOverloadingCache.with(|__root| __root.borrow().clone());
    (tree1, tree2) = trees;
    match '__try0: {
        path = unwrap_break_err!(AvlTreePathPathEnv::get(&tree1, pathIn.clone()), '__try0);
        Ok::<_, &'static str>((path.clone(),))
    } {
        Ok((__try0_o0,)) => {
            path = __try0_o0;
        }
        Err(_) => {
            (cache, operatorCl, recordEnv) = Lookup::lookupClass(&cache, env, &pathIn, None)?;
            (cache, path, recordEnv) = lookupOperatorBaseClass(cache.clone(), recordEnv.clone(), operatorCl.clone())?;
            tree1 = AvlTreePathPathEnv::add(
                tree1.clone(),
                &pathIn,
                &path,
                &*(std::sync::Arc::new(fnptr!(AvlTreePathPathEnv::addConflictDefault, _, _, _))
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
            {
                let __v = (tree1.clone(), tree2.clone());
                crate::Globals::operatorOverloadingCache.with(|__root| *__root.borrow_mut() = __v)
            };
        }
    }
    opNamePath = metamodelica::Ref::new(Absyn::Path::IDENT { name: opName.clone() });
    path = AbsynUtil::makeFullyQualified(AbsynUtil::joinPaths(path, opNamePath)?);
    match '__try1: {
        funcs = unwrap_break_err!(AvlTreePathOperatorTypes::get(&tree2, path.clone()), '__try1);
        Ok::<_, &'static str>((funcs.clone(),))
    } {
        Ok((__try1_o0,)) => {
            funcs = __try1_o0;
        }
        Err(_) => {
            (cache, operatorCl, operEnv) = Lookup::lookupClass(&cache, env, &path, None)?;
            let true = (SCodeUtil::isOperator(&operatorCl)) else {
                return Err("pattern mismatch");
            };
            paths = AbsynToSCode::getListofQualOperatorFuncsfromOperator(&operatorCl)?;
            (cache, funcs) =
                Lookup::lookupFunctionsListInEnv(cache.clone(), operEnv.clone(), &paths, &info, metamodelica::nil())?;
            funcs = List::select2(
                funcs.clone(),
                (if (metamodelica::stringEq(&opName, &(literal!("'constructor'")))
                    || metamodelica::stringEq(&opName, &(literal!("'0'"))))
                {
                    ((std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<DAE::Type>,
                              __a1: metamodelica::Ref<DAE::Type>,
                              __a2: SourceInfo|
                              -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(checkOperatorFunctionOutput(&__a0, __a1, &__a2))
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::Type>,
                                    metamodelica::Ref<DAE::Type>,
                                    SourceInfo,
                                ) -> Result<bool>
                                + 'static,
                        >) as _)
                } else {
                    ((std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<DAE::Type>,
                              __a1: metamodelica::Ref<DAE::Type>,
                              __a2: SourceInfo| {
                            checkOperatorFunctionOneOutput(__a0, __a1, &__a2)
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::Type>,
                                    metamodelica::Ref<DAE::Type>,
                                    SourceInfo,
                                ) -> Result<bool>
                                + 'static,
                        >) as _)
                }),
                scalarType.clone(),
                info.clone(),
            )?;
            tree2 = AvlTreePathOperatorTypes::add(
                tree2.clone(),
                &path,
                &funcs,
                &*(std::sync::Arc::new(fnptr!(AvlTreePathOperatorTypes::addConflictDefault, _, _, _))
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
            {
                let __v = (tree1.clone(), tree2.clone());
                crate::Globals::operatorOverloadingCache.with(|__root| *__root.borrow_mut() = __v)
            };
        }
    }
    Ok((cache, funcs))
}

fn lookupOperatorBaseClass(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inClass: metamodelica::Ref<SCode::Element>,
) -> Result<(FCore::Cache, metamodelica::Ref<Absyn::Path>, FCore::Graph)> {
    let mut cache: FCore::Cache;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut env: FCore::Graph;
    (cache, path, env) = (::match_deref::match_deref! { match &((inCache, inEnv, inClass)) {
        (__esc_cache, __esc_env, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __esc_path, arrayDim: None }, .. }, .. }) => {
            cache = (*__esc_cache).clone();
            path = (*__esc_path).clone();
            env = (*__esc_env).clone();
            let mut cl: metamodelica::Ref<SCode::Element>;
            (cache, cl, env) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&path), None)?;
            (cache, path, env) = lookupOperatorBaseClass(cache.clone(), env.clone(), cl)?;
            (cache.clone(), path.clone(), env.clone())
        },
        (__esc_cache, __esc_env, Deref @ SCode::Element::CLASS { name, .. }) => {
            cache = (*__esc_cache).clone();
            env = (*__esc_env).clone();
            path = FGraph::joinScopePath(metamodelica::AsArg::as_arg(&env), metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }))?;
            (cache.clone(), path, env.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((cache, path, env))
}

fn checkOperatorFunctionOneOutput(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut opType: metamodelica::Ref<DAE::Type>,
    mut info: &SourceInfo,
) -> Result<bool> {
    let mut isOK: bool;
    isOK = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_TUPLE { .. }, .. } => {
            false
        },
        Deref @ DAE::Type::T_FUNCTION { funcArg: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { ty: ty1, defaultBinding: None, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { ty: ty2, defaultBinding: None, .. }, tail: _ } }, .. } => {
            let mut b: bool;
            b = Types::equivtypesOrRecordSubtypeOf(Types::arrayElementType(metamodelica::AsArg::as_arg(&ty1)), opType.clone()) || Types::equivtypesOrRecordSubtypeOf(Types::arrayElementType(metamodelica::AsArg::as_arg(&ty2)), opType.clone());
            checkOperatorFunctionOneOutputError(b, opType, ty, info)?;
            b
        },
        Deref @ DAE::Type::T_FUNCTION { funcArg: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { ty: ty1, defaultBinding: None, .. }, tail: _ }, .. } => {
            let mut b: bool;
            b = Types::equivtypesOrRecordSubtypeOf(Types::arrayElementType(metamodelica::AsArg::as_arg(&ty1)), opType.clone());
            checkOperatorFunctionOneOutputError(b, opType, ty, info)?;
            b
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isOK)
}

fn checkOperatorFunctionOneOutputError(
    mut ok: bool,
    mut opType: metamodelica::Ref<DAE::Type>,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (match ok {
        true => (),
        _ => {
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            str1 = TypesDump::unparseType(opType)?;
            str2 = TypesDump::unparseType(ty)?;
            Error::addSourceMessage(
                &(Error::OP_OVERLOAD_OPERATOR_NOT_INPUT.clone()),
                list![str1, str2],
                info,
            )?;
            return Err("fail");
        }
    });
    Ok(())
}

fn checkOperatorFunctionOutput(
    mut ty: &metamodelica::Ref<DAE::Type>,
    mut expected: metamodelica::Ref<DAE::Type>,
    mut info: &SourceInfo,
) -> bool {
    let mut isOK: bool;
    isOK = (match &**ty {
        DAE::Type::T_FUNCTION {
            funcResultType: actual, ..
        } => {
            isOK = Types::equivtypesOrRecordSubtypeOf(actual.clone(), expected);
            isOK
        }
        _ => false,
    });
    isOK
}

fn isOperatorBinaryFunctionOrWarn(mut ty: &metamodelica::Ref<DAE::Type>, mut info: &SourceInfo) -> Result<bool> {
    let mut isBinaryFunc: bool;
    isBinaryFunc = (::match_deref::match_deref! { match ty {
        Deref @ DAE::Type::T_FUNCTION { funcArg: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            false
        },
        Deref @ DAE::Type::T_FUNCTION { funcArg: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { defaultBinding: None, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { defaultBinding: None, .. }, tail: rest } }, .. } => {
            isBinaryFunc = List::mapMapBoolAnd(metamodelica::AsArg::as_arg(&rest), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::funcArgDefaultBinding(&__a0)) }, &fnptr!(isSome, _))?;
            isBinaryFunc
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isBinaryFunc)
}

fn isOperatorUnaryFunction(mut ty: &metamodelica::Ref<DAE::Type>) -> Result<bool> {
    let mut isBinaryFunc: bool;
    isBinaryFunc = (::match_deref::match_deref! { match ty {
        Deref @ DAE::Type::T_FUNCTION { funcArg: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { defaultBinding: None, .. }, tail: rest }, .. } => {
            isBinaryFunc = List::mapMapBoolAnd(metamodelica::AsArg::as_arg(&rest), &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::funcArgDefaultBinding(&__a0)) }, &fnptr!(isSome, _))?;
            isBinaryFunc
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isBinaryFunc)
}

fn getZeroConstructorExpression(mut ty: metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut result: metamodelica::Ref<DAE::Exp>;
    result = (match &*ty.clone() {
        DAE::Type::T_FUNCTION {
            funcArg: args,
            functionAttributes: attr,
            path,
            ..
        } => {
            result = makeCallFillRestDefaults(
                path.clone(),
                metamodelica::nil(),
                args.clone(),
                Types::makeCallAttr(ty, metamodelica::AsArg::as_arg(&attr)),
            )?;
            result
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(result)
}

fn makeCallFillRestDefaults(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut inExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut restArgs: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
    mut attr: metamodelica::Ref<DAE::CallAttributes>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    exps = listAppend(
        inExps,
        List::mapMap(
            restArgs,
            &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Types::funcArgDefaultBinding(&__a0))
            },
            &|o: Option<_>| o.ok_or("pattern mismatch"),
        )?,
    );
    exp = metamodelica::Ref::new(DAE::Exp::CALL {
        path: path,
        expLst: exps,
        attr: attr,
    });
    Ok(exp)
}

fn getRecordPath(mut inType1: &metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(Types::arrayElementType(inType1)) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outPath = metamodelica::Own::own(__pa0);
    Ok(outPath)
}

fn deoverload(
    mut inOperators: &metamodelica::List<(
        DAE::Operator,
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        metamodelica::Ref<DAE::Type>,
    )>,
    mut inArgs: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>)>,
    mut aexp: &metamodelica::Ref<Absyn::Exp>,
    mut inPrefix: DAE::Prefix,
    mut info: &SourceInfo,
) -> Result<(
    DAE::Operator,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<DAE::Type>,
)> {
    let mut outOperator: DAE::Operator;
    let mut outArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    (outOperator, outArgs, outType) = 'mc: {
        let __mc_input = (&**inOperators, inArgs, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (op, params, rtype), tail: _ }, args, pre) => {
                    let mut args_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut types_1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut rtype_1: metamodelica::Ref<DAE::Type>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut op = (*op).clone();
                    (args_1, types_1) = elabArglist(metamodelica::AsArg::as_arg(&params), metamodelica::AsArg::as_arg(&args))?;
                    rtype_1 = computeReturnType(metamodelica::AsArg::as_arg(&op), &types_1, rtype.clone(), pre.clone(), info)?;
                    ty = Types::simplifyType(rtype_1.clone())?;
                    op = Expression::setOpType(op.clone(), ty.clone())?;
                    Ok((op.clone(), args_1.clone(), rtype_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, args, pre) => {
                    let mut args_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut rtype: metamodelica::Ref<DAE::Type>;
                    let mut op: DAE::Operator;
                    (op, args_1, rtype) = deoverload(metamodelica::AsArg::as_arg(&xs), args.clone(), aexp, pre.clone(), info)?;
                    Ok((op.clone(), args_1.clone(), rtype.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, args, pre) => {
                    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tps: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut exps_str: metamodelica::List<ArcStr>;
                    let mut tps_str: metamodelica::List<ArcStr>;
                    let mut pre_str: ArcStr;
                    let mut s: ArcStr;
                    let mut tpsstr: ArcStr;
                    s = Dump::printExpStr(aexp.clone())?;
                    exps = List::map(args.clone(), &fnptr!(Util::tuple21, _))?;
                    tps = List::map(args.clone(), &fnptr!(Util::tuple22, _))?;
                    exps_str = List::map(exps.clone(), &ExpressionBasics::printExpStr)?;
                    stringDelimitList(exps_str.clone(), literal!(", "));
                    tps_str = List::map(tps.clone(), &TypesDump::unparseType)?;
                    tpsstr = stringDelimitList(tps_str.clone(), literal!(", "));
                    pre_str = PrefixUtil::printPrefixStr3(pre.clone())?;
                    Error::addSourceMessage(&(Error::UNRESOLVABLE_TYPE.clone()), list![s.clone(), tpsstr.clone(), pre_str.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outOperator, outArgs, outType))
}

fn computeReturnType(
    mut inOperator: &DAE::Operator,
    mut inTypesTypeLst: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inPrefix: DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (inOperator, &**inTypesTypeLst, inType, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD_ARR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let true = (Types::subtype(typ1.clone(), typ2.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD_ARR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let true = (Types::subtype(typ2.clone(), typ1.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD_ARR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, pre) => {
                    let mut t1_str: ArcStr;
                    let mut t2_str: ArcStr;
                    let mut pre_str: ArcStr;
                    t1_str = TypesDump::unparseType(typ1.clone())?;
                    t2_str = TypesDump::unparseType(typ2.clone())?;
                    pre_str = PrefixUtil::printPrefixStr3(pre.clone())?;
                    Error::addSourceMessage(&(Error::INCOMPATIBLE_TYPES.clone()), list![literal!("vector addition"), pre_str.clone(), t1_str.clone(), t2_str.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB_ARR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let true = (Types::subtype(typ1.clone(), typ2.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB_ARR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let true = (Types::subtype(typ2.clone(), typ1.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB_ARR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, pre) => {
                    let mut t1_str: ArcStr;
                    let mut t2_str: ArcStr;
                    let mut pre_str: ArcStr;
                    t1_str = TypesDump::unparseType(typ1.clone())?;
                    t2_str = TypesDump::unparseType(typ2.clone())?;
                    pre_str = PrefixUtil::printPrefixStr3(pre.clone())?;
                    Error::addSourceMessage(&(Error::INCOMPATIBLE_TYPES.clone()), list![literal!("vector subtraction"), pre_str.clone(), t1_str.clone(), t2_str.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_ARR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let true = (Types::subtype(typ1.clone(), typ2.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_ARR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let true = (Types::subtype(typ2.clone(), typ1.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_ARR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, pre) => {
                    let mut t1_str: ArcStr;
                    let mut t2_str: ArcStr;
                    let mut pre_str: ArcStr;
                    t1_str = TypesDump::unparseType(typ1.clone())?;
                    t2_str = TypesDump::unparseType(typ2.clone())?;
                    pre_str = PrefixUtil::printPrefixStr3(pre.clone())?;
                    Error::addSourceMessage(&(Error::INCOMPATIBLE_TYPES.clone()), list![literal!("vector elementwise multiplication"), pre_str.clone(), t1_str.clone(), t2_str.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV_ARR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let true = (Types::subtype(typ1.clone(), typ2.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV_ARR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let true = (Types::subtype(typ2.clone(), typ1.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV_ARR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, pre) => {
                    let mut t1_str: ArcStr;
                    let mut t2_str: ArcStr;
                    let mut pre_str: ArcStr;
                    t1_str = TypesDump::unparseType(typ1.clone())?;
                    t2_str = TypesDump::unparseType(typ2.clone())?;
                    pre_str = PrefixUtil::printPrefixStr3(pre.clone())?;
                    Error::addSourceMessage(&(Error::INCOMPATIBLE_TYPES.clone()), list![literal!("vector elementwise division"), pre_str.clone(), t1_str.clone(), t2_str.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW_ARR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let mut m: metamodelica::Ref<DAE::Dimension>;
                    let mut n: metamodelica::Ref<DAE::Dimension>;
                    let 2 = (nDims(metamodelica::AsArg::as_arg(&typ1))?) else { return Err("pattern mismatch") };
                    n = Types::getDimensionNth(metamodelica::AsArg::as_arg(&typ1), 1)?;
                    m = Types::getDimensionNth(metamodelica::AsArg::as_arg(&typ1), 2)?;
                    let true = (Expression::dimensionsKnownAndEqual(&n, &m)?) else { return Err("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW_ARR2 { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let true = (Types::subtype(typ1.clone(), typ2.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW_ARR2 { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let true = (Types::subtype(typ2.clone(), typ1.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW_ARR2 { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, pre) => {
                    let mut t1_str: ArcStr;
                    let mut t2_str: ArcStr;
                    let mut pre_str: ArcStr;
                    t1_str = TypesDump::unparseType(typ1.clone())?;
                    t2_str = TypesDump::unparseType(typ2.clone())?;
                    pre_str = PrefixUtil::printPrefixStr3(pre.clone())?;
                    Error::addSourceMessage(&(Error::INCOMPATIBLE_TYPES.clone()), list![literal!("elementwise vector^vector"), pre_str.clone(), t1_str.clone(), t2_str.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_SCALAR_PRODUCT { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, rtype, _) => {
                    let true = (Types::subtype(typ1.clone(), typ2.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(rtype.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_SCALAR_PRODUCT { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, rtype, _) => {
                    let true = (Types::subtype(typ2.clone(), typ1.clone(), true)) else { return Err("pattern mismatch") };
                    Ok(rtype.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_SCALAR_PRODUCT { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, pre) => {
                    let mut t1_str: ArcStr;
                    let mut t2_str: ArcStr;
                    let mut pre_str: ArcStr;
                    t1_str = TypesDump::unparseType(typ1.clone())?;
                    t2_str = TypesDump::unparseType(typ2.clone())?;
                    pre_str = PrefixUtil::printPrefixStr3(pre.clone())?;
                    Error::addSourceMessage(&(Error::INCOMPATIBLE_TYPES.clone()), list![literal!("scalar product"), pre_str.clone(), t1_str.clone(), t2_str.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_MATRIX_PRODUCT { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let mut rtype: metamodelica::Ref<DAE::Type>;
                    let mut etype: metamodelica::Ref<DAE::Type>;
                    let mut n1: metamodelica::Ref<DAE::Dimension>;
                    let mut n2: metamodelica::Ref<DAE::Dimension>;
                    let mut m: metamodelica::Ref<DAE::Dimension>;
                    let 1 = (nDims(metamodelica::AsArg::as_arg(&typ1))?) else { return Err("pattern mismatch") };
                    let 2 = (nDims(metamodelica::AsArg::as_arg(&typ2))?) else { return Err("pattern mismatch") };
                    n1 = Types::getDimensionNth(metamodelica::AsArg::as_arg(&typ1), 1)?;
                    n2 = Types::getDimensionNth(metamodelica::AsArg::as_arg(&typ2), 1)?;
                    m = Types::getDimensionNth(metamodelica::AsArg::as_arg(&typ2), 2)?;
                    let true = (isValidMatrixProductDims(&n1, &n2)?) else { return Err("pattern mismatch") };
                    etype = elementType(typ1.clone())?;
                    rtype = Types::liftArray(etype.clone(), m.clone());
                    Ok(rtype.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_MATRIX_PRODUCT { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let mut rtype: metamodelica::Ref<DAE::Type>;
                    let mut etype: metamodelica::Ref<DAE::Type>;
                    let mut n: metamodelica::Ref<DAE::Dimension>;
                    let mut m1: metamodelica::Ref<DAE::Dimension>;
                    let mut m2: metamodelica::Ref<DAE::Dimension>;
                    let 2 = (nDims(metamodelica::AsArg::as_arg(&typ1))?) else { return Err("pattern mismatch") };
                    let 1 = (nDims(metamodelica::AsArg::as_arg(&typ2))?) else { return Err("pattern mismatch") };
                    n = Types::getDimensionNth(metamodelica::AsArg::as_arg(&typ1), 1)?;
                    m1 = Types::getDimensionNth(metamodelica::AsArg::as_arg(&typ1), 2)?;
                    m2 = Types::getDimensionNth(metamodelica::AsArg::as_arg(&typ2), 1)?;
                    let true = (isValidMatrixProductDims(&m1, &m2)?) else { return Err("pattern mismatch") };
                    etype = elementType(typ2.clone())?;
                    rtype = Types::liftArray(etype.clone(), n.clone());
                    Ok(rtype.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_MATRIX_PRODUCT { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let mut rtype: metamodelica::Ref<DAE::Type>;
                    let mut etype: metamodelica::Ref<DAE::Type>;
                    let mut n: metamodelica::Ref<DAE::Dimension>;
                    let mut m1: metamodelica::Ref<DAE::Dimension>;
                    let mut m2: metamodelica::Ref<DAE::Dimension>;
                    let mut p: metamodelica::Ref<DAE::Dimension>;
                    let 2 = (nDims(metamodelica::AsArg::as_arg(&typ1))?) else { return Err("pattern mismatch") };
                    let 2 = (nDims(metamodelica::AsArg::as_arg(&typ2))?) else { return Err("pattern mismatch") };
                    n = Types::getDimensionNth(metamodelica::AsArg::as_arg(&typ1), 1)?;
                    m1 = Types::getDimensionNth(metamodelica::AsArg::as_arg(&typ1), 2)?;
                    m2 = Types::getDimensionNth(metamodelica::AsArg::as_arg(&typ2), 1)?;
                    p = Types::getDimensionNth(metamodelica::AsArg::as_arg(&typ2), 2)?;
                    let true = (isValidMatrixProductDims(&m1, &m2)?) else { return Err("pattern mismatch") };
                    etype = elementType(typ1.clone())?;
                    rtype = Types::liftArrayListDims(etype.clone(), list![n.clone(), p.clone()]);
                    Ok(rtype.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_MATRIX_PRODUCT { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, pre) => {
                    let mut t1_str: ArcStr;
                    let mut t2_str: ArcStr;
                    let mut pre_str: ArcStr;
                    t1_str = TypesDump::unparseType(typ1.clone())?;
                    t2_str = TypesDump::unparseType(typ2.clone())?;
                    pre_str = PrefixUtil::printPrefixStr3(pre.clone())?;
                    Error::addSourceMessage(&(Error::INCOMPATIBLE_TYPES.clone()), list![literal!("matrix multiplication"), pre_str.clone(), t1_str.clone(), t2_str.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL_ARRAY_SCALAR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD_ARRAY_SCALAR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB_SCALAR_ARRAY { .. }, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    Ok(typ2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV_SCALAR_ARRAY { .. }, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    Ok(typ2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV_ARRAY_SCALAR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW_ARRAY_SCALAR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW_SCALAR_ARRAY { .. }, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    Ok(typ2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { .. }, _, typ, _) => {
                    Ok(typ.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::SUB { .. }, _, typ, _) => {
                    Ok(typ.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, _, typ, _) => {
                    Ok(typ.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { .. }, _, typ, _) => {
                    Ok(typ.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::POW { .. }, _, typ, _) => {
                    Ok(typ.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS { .. }, _, typ, _) => {
                    Ok(typ.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::UMINUS_ARR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: _ }, _, _) => {
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::AND { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let true = (Types::equivtypes(typ1.clone(), typ2.clone())) else { return Err("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::AND { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, pre) => {
                    let mut t1_str: ArcStr;
                    let mut t2_str: ArcStr;
                    let mut pre_str: ArcStr;
                    t1_str = TypesDump::unparseType(typ1.clone())?;
                    t2_str = TypesDump::unparseType(typ2.clone())?;
                    pre_str = PrefixUtil::printPrefixStr3(pre.clone())?;
                    Error::addSourceMessage(&(Error::INCOMPATIBLE_TYPES.clone()), list![literal!("and"), pre_str.clone(), t1_str.clone(), t2_str.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::OR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let true = (Types::equivtypes(typ1.clone(), typ2.clone())) else { return Err("pattern mismatch") };
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::OR { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Cons { head: typ2, tail: Deref @ metamodelica::ListNode::Nil } }, _, pre) => {
                    let mut t1_str: ArcStr;
                    let mut t2_str: ArcStr;
                    let mut pre_str: ArcStr;
                    t1_str = TypesDump::unparseType(typ1.clone())?;
                    t2_str = TypesDump::unparseType(typ2.clone())?;
                    pre_str = PrefixUtil::printPrefixStr3(pre.clone())?;
                    Error::addSourceMessage(&(Error::INCOMPATIBLE_TYPES.clone()), list![literal!("or"), pre_str.clone(), t1_str.clone(), t2_str.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::NOT { .. }, Deref @ metamodelica::ListNode::Cons { head: typ1, tail: Deref @ metamodelica::ListNode::Nil }, _, _) => {
                    Ok(typ1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::LESS { .. }, _, typ, _) => {
                    Ok(typ.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::LESSEQ { .. }, _, typ, _) => {
                    Ok(typ.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::GREATER { .. }, _, typ, _) => {
                    Ok(typ.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::GREATEREQ { .. }, _, typ, _) => {
                    Ok(typ.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::EQUAL { .. }, _, typ, _) => {
                    Ok(typ.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::NEQUAL { .. }, _, typ, _) => {
                    Ok(typ.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::USERDEFINED { .. }, _, typ, _) => {
                    Ok(typ.clone())
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

fn nDims<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> Result<i32> {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_INTEGER { .. } => return Ok(0),
            DAE::Type::T_REAL { .. } => return Ok(0),
            DAE::Type::T_STRING { .. } => return Ok(0),
            DAE::Type::T_BOOL { .. } => return Ok(0),
            DAE::Type::T_ARRAY { ty: t, .. } => {
                let mut ns: i32;
                ns = nDims(t)?;
                return Ok(ns + 1);
            }
            DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
                let mut ns: i32;
                {
                    inType = t;
                    continue '__tco;
                }
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

fn isValidMatrixProductDims(
    mut dim1: &metamodelica::Ref<DAE::Dimension>,
    mut dim2: &metamodelica::Ref<DAE::Dimension>,
) -> Result<bool> {
    let mut res: bool;
    res = Expression::dimensionsKnownAndEqual(dim1, dim2)?
        || !(Expression::dimensionKnown(dim1) || Expression::dimensionKnown(dim2))
        || Flags::getConfigBool(Flags::CHECK_MODEL.clone())? && Expression::dimensionsEqual(dim1, dim2)?;
    Ok(res)
}

fn elementType(mut inType: metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Type>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inType) {
            t @ Deref @ DAE::Type::T_INTEGER { .. } => {
                return Ok(t.clone())
            },
            t @ Deref @ DAE::Type::T_REAL { .. } => {
                return Ok(t.clone())
            },
            t @ Deref @ DAE::Type::T_STRING { .. } => {
                return Ok(t.clone())
            },
            t @ Deref @ DAE::Type::T_BOOL { .. } => {
                return Ok(t.clone())
            },
            Deref @ DAE::Type::T_ARRAY { ty: t, .. } => {
                let mut t_1: metamodelica::Ref<DAE::Type>;
                { inType = t.clone(); continue '__tco; }
            },
            Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
                let mut t_1: metamodelica::Ref<DAE::Type>;
                { inType = t.clone(); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn replaceOperatorWithFcall(
    mut AbExp: &metamodelica::Ref<Absyn::Exp>,
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inOper: DAE::Operator,
    mut inExp2: Option<metamodelica::Ref<DAE::Exp>>,
    mut inConst: DAE::Const,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((AbExp.clone(), inOper.clone(), inExp2)) {
        (Deref @ Absyn::Exp::BINARY { exp1: _, op: _, exp2: _ }, DAE::Operator::USERDEFINED { fqName: funcname }, Some(e2)) => {
            let mut e1 = inExp1;
            metamodelica::Ref::new(DAE::Exp::CALL { path: funcname.clone(), expLst: list![e1, e2.clone()], attr: DAE::callAttrOther().clone() })
        },
        (Deref @ Absyn::Exp::BINARY { exp1: _, op: _, exp2: _ }, _, Some(e2)) => {
            let mut e1 = inExp1;
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1, operator: inOper, exp2: e2.clone() })
        },
        (Deref @ Absyn::Exp::UNARY { op: _, exp: _ }, DAE::Operator::USERDEFINED { fqName: funcname }, None) => {
            let mut e1 = inExp1;
            metamodelica::Ref::new(DAE::Exp::CALL { path: funcname.clone(), expLst: list![e1], attr: DAE::callAttrOther().clone() })
        },
        (Deref @ Absyn::Exp::UNARY { op: _, exp: _ }, _, None) => {
            let mut e1 = inExp1;
            metamodelica::Ref::new(DAE::Exp::UNARY { operator: inOper, exp: e1 })
        },
        (Deref @ Absyn::Exp::LBINARY { exp1: _, op: _, exp2: _ }, DAE::Operator::USERDEFINED { fqName: funcname }, Some(e2)) => {
            let mut e1 = inExp1;
            metamodelica::Ref::new(DAE::Exp::CALL { path: funcname.clone(), expLst: list![e1, e2.clone()], attr: DAE::callAttrOther().clone() })
        },
        (Deref @ Absyn::Exp::LBINARY { exp1: _, op: _, exp2: _ }, _, Some(e2)) => {
            let mut e1 = inExp1;
            metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e1, operator: inOper, exp2: e2.clone() })
        },
        (Deref @ Absyn::Exp::LUNARY { op: _, exp: _ }, DAE::Operator::USERDEFINED { fqName: funcname }, None) => {
            let mut e1 = inExp1;
            metamodelica::Ref::new(DAE::Exp::CALL { path: funcname.clone(), expLst: list![e1], attr: DAE::callAttrOther().clone() })
        },
        (Deref @ Absyn::Exp::LUNARY { op: _, exp: _ }, _, None) => {
            let mut e1 = inExp1;
            metamodelica::Ref::new(DAE::Exp::LUNARY { operator: inOper, exp: e1 })
        },
        (Deref @ Absyn::Exp::RELATION { exp1: _, op: _, exp2: _ }, DAE::Operator::USERDEFINED { fqName: funcname }, Some(e2)) => {
            let mut e1 = inExp1;
            metamodelica::Ref::new(DAE::Exp::CALL { path: funcname.clone(), expLst: list![e1, e2.clone()], attr: DAE::callAttrOther().clone() })
        },
        (Deref @ Absyn::Exp::RELATION { exp1: _, op: _, exp2: _ }, _, Some(e2)) => {
            let mut e1 = inExp1;
            metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1, operator: inOper, exp2: e2.clone(), index: -1, optionExpisASUB: None })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

fn warnUnsafeRelations(
    mut inEnv: &FCore::Graph,
    mut inExp: &metamodelica::Ref<Absyn::Exp>,
    mut variability: DAE::Const,
    mut t1: &metamodelica::Ref<DAE::Type>,
    mut t2: &metamodelica::Ref<DAE::Type>,
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
    mut op: &DAE::Operator,
    mut inPrefix: &DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> () {
    let () = 'mc: {
        let __mc_input = (&**inExp, variability);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (FGraph::inFunctionScope(inEnv)) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::RELATION { exp1: _, op: _, exp2: _ }, DAE::Const::C_VAR { .. }) => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut stmtString: ArcStr;
                    let mut opString: ArcStr;
                    b1 = Types::isReal(t1);
                    b2 = Types::isReal(t1);
                    let true = (boolOr(b1, b2)) else { return Err("pattern mismatch") };
                    verifyOp(op)?;
                    opString = ExpressionDump::relopSymbol(op)?;
                    stmtString = { let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*opString); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::WARNING_RELATION_ON_REAL.clone()), list![stmtString.clone(), opString.clone()], inInfo)?;
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
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn verifyOp(mut op: &DAE::Operator) -> Result<()> {
    let () = (match op.clone() {
        DAE::Operator::EQUAL { ty: _ } => (),
        DAE::Operator::NEQUAL { ty: _ } => (),
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn errorMultipleValid(mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>, mut info: &SourceInfo) -> Result<()> {
    let mut str1: ArcStr;
    let mut str2: ArcStr;
    str1 = intString(((exps).len() as i32));
    str2 = stringDelimitList(List::map(exps, &ExpressionBasics::printExpStr)?, literal!(","));
    Error::addSourceMessage(&(Error::OP_OVERLOAD_MULTIPLE_VALID.clone()), list![str1, str2], info)?;
    Ok(())
}

fn binaryCastConstructor(
    mut inCache: FCore::Cache,
    mut env: &FCore::Graph,
    mut inExp1: &metamodelica::Ref<DAE::Exp>,
    mut inExp2: &metamodelica::Ref<DAE::Exp>,
    mut inType1: metamodelica::Ref<DAE::Type>,
    mut inType2: metamodelica::Ref<DAE::Type>,
    mut exps: metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>,
    mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut info: &SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut cache: FCore::Cache;
    let mut resExps: metamodelica::List<(metamodelica::Ref<DAE::Exp>, Option<metamodelica::Ref<DAE::Type>>)>;
    (cache, resExps) = (::match_deref::match_deref! { match &(exps.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } => {
            (inCache, exps)
        },
        Deref @ metamodelica::ListNode::Nil => {
            let mut args: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::FuncArg>>>;
            let mut tys1: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut tys2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut exps1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut exps2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            args = List::map(types.clone(), &move |__a0: metamodelica::Ref<DAE::Type>| Types::getFuncArg(&__a0))?;
            tys1 = List::mapMap(args.clone(), &listHead, &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::funcArgType(&__a0)) })?;
            args = List::map(args, &listRest)?;
            tys2 = List::mapMap(args, &listHead, &move |__a0: metamodelica::Ref<DAE::FuncArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::funcArgType(&__a0)) })?;
            tys1 = List::setDifference(List::union(&tys1, &(metamodelica::nil())), &(list![inType1.clone()]))?;
            tys2 = List::setDifference(List::union(&tys2, &(metamodelica::nil())), &(list![inType2.clone()]))?;
            (cache, tys1) = getOperatorFuncsOrEmpty(&inCache, env, &tys1, &(literal!("'constructor'")), info, &(metamodelica::nil()))?;
            (cache, tys2) = getOperatorFuncsOrEmpty(&cache, env, &tys2, &(literal!("'constructor'")), info, &(metamodelica::nil()))?;
            tys1 = List::select(tys1, (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Type>| isOperatorUnaryFunction(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>) -> Result<bool> + 'static>))?;
            tys2 = List::select(tys2, (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Type>| isOperatorUnaryFunction(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>) -> Result<bool> + 'static>))?;
            exps1 = deoverloadUnaryUserdefNoConstructor(&tys1, inExp1, &inType1, metamodelica::nil())?;
            exps2 = deoverloadUnaryUserdefNoConstructor(&tys2, inExp2, &inType2, metamodelica::nil())?;
            resExps = deoverloadBinaryUserdefNoConstructorListLhs(&types, exps1, inExp2, &inType2, metamodelica::nil())?;
            resExps = deoverloadBinaryUserdefNoConstructorListRhs(&types, inExp1, exps2, &inType1, resExps)?;
            (cache, resExps)
        },
        _ => {
            errorMultipleValid(List::map(exps, &fnptr!(Util::tuple21, _))?, info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cache, resExps))
}

fn getZeroConstructor(
    mut inCache: FCore::Cache,
    mut env: FCore::Graph,
    mut zexps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut r#impl: bool,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, Option<metamodelica::Ref<Values::Value>>)> {
    let mut cache: FCore::Cache;
    let mut zeroExpression: Option<metamodelica::Ref<Values::Value>>;
    (cache, zeroExpression) = (::match_deref::match_deref! { match &(zexps.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            (inCache, None)
        },
        Deref @ metamodelica::ListNode::Cons { head: zc, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut v: metamodelica::Ref<Values::Value>;
            (cache, v) = Ceval::ceval(inCache, env, zc.clone(), r#impl, Absyn::Msg::MSG { info: info }, 0)?;
            (cache, Some(v))
        },
        _ => {
            errorMultipleValid(zexps, &info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cache, zeroExpression))
}
