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

use crate::Builtin;
use crate::ConnectionGraph;
use crate::FGraph;
use crate::FNode;
use crate::InnerOuter;
use crate::Inst;
use crate::InstExtends;
use crate::InstFunction;
use crate::InstUtil;
use crate::Mod;
use crate::PrefixUtil;
use crate::Static;
use crate::UnitAbsyn;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::HashTableStringToPath;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

/*   - Lookup functions

 These functions look up class and variable names in the environment.
 The names are supplied as a path, and if the path is qualified, a
 variable named as the first part of the path is searched for, and the
 name is looked for in it.

*/
pub fn lookupType(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut msg: Option<SourceInfo>,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Type>, FCore::Graph)> {
    let mut cache: FCore::Cache;
    let mut t: metamodelica::Ref<DAE::Type>;
    let mut env: FCore::Graph;
    (cache, t, env) = (match &*inPath {
        Absyn::Path::IDENT { name: __inPath_name } => {
            (cache, t, env) = lookupTypeIdent(inCache, inEnv, __inPath_name.clone(), msg)?;
            (cache, t, env)
        }
        _ => {
            (cache, t, env) = lookupTypeQual(inCache, inEnv, inPath, msg)?;
            (cache, t, env)
        }
    });
    Ok((cache, t, env))
}

fn lookupTypeQual(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut msg: Option<SourceInfo>,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Type>, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outEnv: FCore::Graph;
    (outCache, outType, outEnv) = 'mc: {
        let __mc_input = (inCache, inEnv, inPath.clone(), msg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "isRoot" } }, _) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    t = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("x"), ty: DAE::T_ANYTYPE_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_BOOL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: inPath.clone() });
                    Ok((cache.clone(), t.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "uniqueRootIndices" } }, _) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    t = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("roots"), ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_ANYTYPE_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), metamodelica::Ref::new(DAE::FuncArg { name: literal!("nodes"), ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_ANYTYPE_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), metamodelica::Ref::new(DAE::FuncArg { name: literal!("message"), ty: DAE::T_STRING_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: inPath.clone() });
                    Ok((cache.clone(), t.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, path, _) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut cache = (*cache).clone();
                    (cache, c, env_1) = lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&path), None)?;
                    (cache, t, env_2) = lookupType2(cache.clone(), env_1.clone(), c.clone())?;
                    Ok((cache.clone(), t.clone(), env_2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, path, Some(info)) => {
                    let mut classname: ArcStr;
                    let mut scope: ArcStr;
                    classname = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
                    classname = stringAppend(classname.clone(), literal!(" (its type) "));
                    scope = FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env));
                    Error::addSourceMessage(&(Error::LOOKUP_ERROR.clone()), list![classname.clone(), scope.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outType, outEnv))
}

pub(crate) fn lookupTypeIdent(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut ident: ArcStr,
    mut msg: Option<SourceInfo>,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Type>, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outEnv: FCore::Graph;
    (outCache, outType, outEnv) = 'mc: {
        let __mc_input = (inCache, inEnv, ident.clone(), msg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ "rooted", _) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    t = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("x"), ty: DAE::T_ANYTYPE_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_BOOL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("rooted") }) });
                    Ok((cache.clone(), t.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut env_1: FCore::Graph;
                    let mut cache = (*cache).clone();
                    (cache, t, env_1) = lookupTypeInEnv(cache.clone(), env.clone(), &ident)?;
                    Ok((cache.clone(), t.clone(), env_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut cache = (*cache).clone();
                    (cache, c, env_1) = lookupClassIdent(cache.clone(), env.clone(), &ident, None)?;
                    (cache, t, env_2) = lookupType2(cache.clone(), env_1.clone(), c.clone())?;
                    Ok((cache.clone(), t.clone(), env_2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, _, Some(info)) => {
                    let mut classname: ArcStr;
                    let mut scope: ArcStr;
                    classname = stringAppend(ident.clone(), literal!(" (its type) "));
                    scope = FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env));
                    Error::addSourceMessage(&(Error::LOOKUP_ERROR.clone()), list![classname.clone(), scope.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outType, outEnv))
}

fn lookupType2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inClass: metamodelica::Ref<SCode::Element>,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Type>, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outEnv: FCore::Graph;
    (outCache, outType, outEnv) = 'mc: {
        let __mc_input = (inCache, inEnv, inClass);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, c @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. }) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let mut env_1 = (*env_1).clone();
                    (cache, env_1, t) = buildRecordType(cache.clone(), env_1.clone(), c.clone())?;
                    Ok((cache.clone(), t.clone(), env_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, c @ Deref @ SCode::Element::CLASS { name: id, encapsulatedPrefix: encflag, restriction: r @ SCode::Restriction::R_ENUMERATION { .. }, .. }) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut env_2: FCore::Graph;
                    let mut env_3: FCore::Graph;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut types: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut names: metamodelica::List<ArcStr>;
                    let mut ci_state: ClassInf::State;
                    let mut r#mod: metamodelica::Ref<DAE::Mod>;
                    let mut cache = (*cache).clone();
                    env_2 = FGraph::openScope(env_1.clone(), encflag.clone(), id.clone(), Some(openmodelica_frontend_dump::FCore::ScopeType::CLASS_SCOPE))?;
                    ci_state = ClassInfUtil::start(metamodelica::AsArg::as_arg(&r), FGraph::getGraphName(&env_2)?)?;
                    r#mod = Mod::getClassModifier(metamodelica::AsArg::as_arg(&env_1), id.clone());
                    (cache, env_3, _, _, _, _, _, types, _, _, _, _) = Inst::instClassIn(cache.clone(), env_2.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), r#mod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), false, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), None)?;
                    (_, names) = SCodeUtil::getClassComponents(metamodelica::AsArg::as_arg(&c))?;
                    Types::checkEnumDuplicateLiterals(names.clone(), var_field!((**c).info, SCode::Element::CLASS))?;
                    path = FGraph::getGraphName(&env_3)?;
                    t = metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: None, path: path.clone(), names: names.clone(), literalVarLst: types.clone(), attributeLst: metamodelica::nil() });
                    env_3 = FGraph::mkTypeNode(env_3.clone(), id.clone(), t.clone())?;
                    Ok((cache.clone(), t.clone(), env_3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_TYPE { .. }, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Real" }, .. }, .. }, .. }) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    t = DAE::T_REAL_DEFAULT().clone();
                    Ok((cache.clone(), t.clone(), env_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_TYPE { .. }, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Integer" }, .. }, .. }, .. }) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    t = DAE::T_INTEGER_DEFAULT().clone();
                    Ok((cache.clone(), t.clone(), env_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_TYPE { .. }, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Boolean" }, .. }, .. }, .. }) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    t = DAE::T_BOOL_DEFAULT().clone();
                    Ok((cache.clone(), t.clone(), env_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_TYPE { .. }, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: Deref @ "String" }, .. }, .. }, .. }) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    t = DAE::T_STRING_DEFAULT().clone();
                    Ok((cache.clone(), t.clone(), env_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, c @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_METARECORD { .. }, .. }) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut env_2: FCore::Graph;
                    let mut cache = (*cache).clone();
                    (cache, env_2, t) = buildMetaRecordType(cache.clone(), env_1.clone(), metamodelica::AsArg::as_arg(&c))?;
                    Ok((cache.clone(), t.clone(), env_2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, c) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut env_2: FCore::Graph;
                    let mut id: ArcStr;
                    let mut cache = (*cache).clone();
                    let mut env_1 = (*env_1).clone();
                    let true = (SCodeUtil::classIsExternalObject(metamodelica::AsArg::as_arg(&c))) else { return Err("pattern mismatch") };
                    (cache, env_1, _, _, _, _, _, _, _, _) = Inst::instClass(cache.clone(), env_1.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, c.clone(), metamodelica::nil(), false, openmodelica_frontend_inst::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
                    let __pa0 = ::match_deref::match_deref! { match &(c.clone()) {
                        Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    id = metamodelica::Own::own(__pa0);
                    (env_1, _) = FGraph::stripLastScopeRef(env_1.clone())?;
                    (cache, t, env_2) = lookupTypeInEnv(cache.clone(), env_1.clone(), &id)?;
                    Ok((cache.clone(), t.clone(), env_2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env_1, c @ Deref @ SCode::Element::CLASS { name: id, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: _ }, .. }) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut env_2: FCore::Graph;
                    let mut env_3: FCore::Graph;
                    let mut cache = (*cache).clone();
                    (cache, env_2, _) = InstFunction::implicitFunctionTypeInstantiation(cache.clone(), env_1.clone(), InnerOuter::emptyInstHierarchy().clone(), c.clone())?;
                    (cache, t, env_3) = lookupTypeInEnv(cache.clone(), env_2.clone(), metamodelica::AsArg::as_arg(&id))?;
                    Ok((cache.clone(), t.clone(), env_3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outType, outEnv))
}

pub fn lookupMetarecordsRecursive(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inUniontypePaths: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Type>>)> {
    let mut outCache: FCore::Cache;
    let mut outMetarecordTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (outCache, _, outMetarecordTypes) = lookupMetarecordsRecursive2(
        inCache,
        inEnv,
        inUniontypePaths,
        HashTableStringToPath::emptyHashTable(),
        metamodelica::nil(),
    )?;
    Ok((outCache, outMetarecordTypes))
}

fn lookupMetarecordsRecursive2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inUniontypePaths: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut inHt: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inAcc: metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<(
    FCore::Cache,
    (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
        ),
    ),
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outHt: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            HashTableStringToPath::FuncHashCref,
            HashTableStringToPath::FuncCrefEqual,
            HashTableStringToPath::FuncCrefStr,
            HashTableStringToPath::FuncExpStr,
        ),
    ) = inHt;
    let mut outMetarecordTypes: metamodelica::List<metamodelica::Ref<DAE::Type>> = inAcc;
    for mut first in &**inUniontypePaths {
        (outCache, outHt, outMetarecordTypes) = lookupMetarecordsRecursive3(
            outCache,
            inEnv.clone(),
            first.clone(),
            AbsynUtil::pathString(first.clone(), literal!("."), true, false)?,
            outHt,
            outMetarecordTypes,
        )?;
    }
    Ok((outCache, outHt, outMetarecordTypes))
}

fn lookupMetarecordsRecursive3(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut r#str: ArcStr,
    mut inHt: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inAcc: metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<(
    FCore::Cache,
    (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
        ),
    ),
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outHt: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            HashTableStringToPath::FuncHashCref,
            HashTableStringToPath::FuncCrefEqual,
            HashTableStringToPath::FuncCrefStr,
            HashTableStringToPath::FuncExpStr,
        ),
    );
    let mut outMetarecordTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (outCache, outHt, outMetarecordTypes) = (match inHt {
        mut ht if (BaseHashTable::hasKey(r#str.clone(), &ht)?) => {
            let mut cache = inCache;
            let mut acc = inAcc;
            (cache, ht.clone(), acc)
        }
        mut ht => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut acc = inAcc;
            let mut uniontypePaths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            let mut uniontypeTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            ht = BaseHashTable::add((r#str.clone(), path.clone()), ht)?;
            (cache, ty, _) = lookupType(cache, env.clone(), path, Some(Absyn::dummyInfo.clone()))?;
            acc = metamodelica::cons(ty.clone(), acc);
            uniontypeTypes = Types::getAllInnerTypesOfType(
                ty,
                &move |__a0: metamodelica::Ref<DAE::Type>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Types::uniontypeFilter(&__a0))
                },
            )?;
            uniontypePaths = List::flatten(List::map(uniontypeTypes, &move |__a0: metamodelica::Ref<
                DAE::Type,
            >| {
                Types::getUniontypePaths(&__a0)
            })?)?;
            (cache, ht, acc) = lookupMetarecordsRecursive2(cache, env, &uniontypePaths, ht, acc)?;
            (cache, ht.clone(), acc)
        }
    });
    Ok((outCache, outHt, outMetarecordTypes))
}

pub fn lookupClass(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inInfo: Option<SourceInfo>,
) -> Result<(FCore::Cache, metamodelica::Ref<SCode::Element>, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outClass: metamodelica::Ref<SCode::Element> =
        <metamodelica::Ref<SCode::Element> as ::std::default::Default>::default();
    let mut outEnv: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    (outCache, outClass, outEnv) = 'mc: {
        let __mc_input = &**inPath;
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::QUALIFIED { name, path: id } => {
                    let mut cenv: FCore::Graph;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outClass: metamodelica::Ref<SCode::Element> = outClass.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    ErrorExt::setCheckpoint(literal!("functionViaComponentRef2"));
                    (outCache, _, _, _, _, _, _, cenv, _) = lookupVarIdent(inCache.clone(), inEnv.clone(), name.clone(), metamodelica::nil())?;
                    (outCache, outClass, outEnv) = lookupClass(&outCache, &cenv, metamodelica::AsArg::as_arg(&id), None)?;
                    ErrorExt::rollBack(literal!("functionViaComponentRef2"));
                    Ok(((outCache.clone(), outClass.clone(), outEnv.clone()), outCache.clone(), outClass.clone(), outEnv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outClass = __wb1;
            outEnv = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::QUALIFIED { name: _, path: _ } => {
                    ErrorExt::rollBack(literal!("functionViaComponentRef2"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outClass: metamodelica::Ref<SCode::Element> = outClass.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    (outCache, outClass, outEnv, _) = lookupClass1(inCache.clone(), inEnv.clone(), inPath.clone(), metamodelica::nil(), Mutable::create(false), inInfo.clone())?;
                    Ok(((outCache.clone(), outClass.clone(), outEnv.clone()), outCache.clone(), outClass.clone(), outEnv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outClass = __wb1;
            outEnv = __wb2;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outClass, outEnv))
}

pub fn lookupClassIdent(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut ident: &ArcStr,
    mut inInfo: Option<SourceInfo>,
) -> Result<(FCore::Cache, metamodelica::Ref<SCode::Element>, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outEnv: FCore::Graph;
    (outCache, outClass, outEnv, _) = lookupClassInEnv(
        inCache,
        inEnv,
        ident,
        metamodelica::nil(),
        Mutable::create(false),
        inInfo,
    )?;
    Ok((outCache, outClass, outEnv))
}

fn lookupClass1(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
    mut inState: Mutable::Mutable<bool>,
    mut inInfo: Option<SourceInfo>,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<SCode::Element>,
    FCore::Graph,
    metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
    let mut errors: i32 = Error::getNumErrorMessages();
    if let Ok((__pa0, __pa1, __pa2, __pa3)) = lookupClass2(
        inCache.clone(),
        inEnv.clone(),
        inPath.clone(),
        inPrevFrames.clone(),
        inState.clone(),
        inInfo.clone(),
    ) {
        outCache = metamodelica::Own::own(__pa0);
        outClass = metamodelica::Own::own(__pa1);
        outEnv = metamodelica::Own::own(__pa2);
        outPrevFrames = metamodelica::Own::own(__pa3);
    } else {
        if (inInfo).is_some() && errors == Error::getNumErrorMessages() {
            Error::addSourceMessage(
                &(Error::LOOKUP_ERROR.clone()),
                list![
                    AbsynUtil::pathString(inPath.clone(), literal!("."), true, false)?,
                    FGraph::printGraphPathStr(&inEnv)
                ],
                &(inInfo.clone().ok_or("pattern mismatch")?),
            )?;
        }
        return Err("fail");
    }
    Ok((outCache, outClass, outEnv, outPrevFrames))
}

fn lookupClass2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
    mut inState: Mutable::Mutable<bool>,
    mut inInfo: Option<SourceInfo>,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<SCode::Element>,
    FCore::Graph,
    metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCache, inEnv, inPath, inPrevFrames)) {
            (cache, env, Deref @ Absyn::Path::FULLYQUALIFIED { path }, Deref @ metamodelica::ListNode::Nil) => {
                let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                let mut c: metamodelica::Ref<SCode::Element>;
                let mut env_1: FCore::Graph;
                let mut prevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                let mut cache = (*cache).clone();
                let mut env = (*env).clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(metamodelica::AsArg::as_arg(&env)).reverse()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                r = metamodelica::Own::own(__pa0);
                prevFrames = metamodelica::Own::own(__pa1);
                Mutable::update(inState.clone(), true);
                env = FGraph::setScope(env.clone(), list![r])?;
                { (inCache, inEnv, inPath, inPrevFrames, inState, inInfo) = (cache.clone(), env.clone(), path.clone(), prevFrames, inState, inInfo); continue '__tco; }
            },
            (cache, env, Deref @ Absyn::Path::QUALIFIED { name: pack, path }, prevFrames) => {
                let mut c: metamodelica::Ref<SCode::Element>;
                let mut env_2: FCore::Graph;
                let mut optFrame: Option<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                let mut cache = (*cache).clone();
                let mut prevFrames = (*prevFrames).clone();
                (optFrame, prevFrames) = lookupPrevFrames(metamodelica::AsArg::as_arg(&pack), metamodelica::AsArg::as_arg(&prevFrames));
                return Ok(lookupClassQualified(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&pack), path.clone(), optFrame, prevFrames.clone(), inState, inInfo)?)
            },
            (cache, env, Deref @ Absyn::Path::IDENT { name: id }, prevFrames) => {
                let mut c: metamodelica::Ref<SCode::Element>;
                let mut env_1: FCore::Graph;
                let mut cache = (*cache).clone();
                let mut prevFrames = (*prevFrames).clone();
                return Ok(lookupClassInEnv(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&id), prevFrames.clone(), inState, inInfo)?)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lookupClassQualified(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut id: &ArcStr,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut inOptFrame: Option<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
    mut inPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
    mut inState: Mutable::Mutable<bool>,
    mut inInfo: Option<SourceInfo>,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<SCode::Element>,
    FCore::Graph,
    metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
    (outCache, outClass, outEnv, outPrevFrames) = (match inOptFrame {
        Some(mut frame) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut prevFrames = inPrevFrames;
            let mut c: metamodelica::Ref<SCode::Element>;
            Mutable::update(inState.clone(), true);
            env = FGraph::pushScopeRef(env, frame)?;
            (cache, c, env, prevFrames) = lookupClass2(cache, env, path, prevFrames, inState, inInfo)?;
            (cache, c, env, prevFrames)
        }
        None => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut c: metamodelica::Ref<SCode::Element>;
            let mut prevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
            let mut optFrame: Option<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
            (cache, c, env, prevFrames) =
                lookupClassInEnv(cache, env, id, metamodelica::nil(), inState.clone(), inInfo.clone())?;
            (optFrame, prevFrames) = lookupPrevFrames(id, &prevFrames);
            (cache, c, env, prevFrames) =
                lookupClassQualified2(cache, env, path, c, optFrame, prevFrames, inState, inInfo)?;
            (cache, c, env, prevFrames)
        }
    });
    Ok((outCache, outClass, outEnv, outPrevFrames))
}

fn lookupClassQualified2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut inC: metamodelica::Ref<SCode::Element>,
    mut optFrame: Option<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
    mut inPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
    mut inState: Mutable::Mutable<bool>,
    mut inInfo: Option<SourceInfo>,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<SCode::Element>,
    FCore::Graph,
    metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
    (outCache, outClass, outEnv, outPrevFrames) = 'mc: {
        let __mc_input = (inCache, inEnv.clone(), &*inC, optFrame, inPrevFrames);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, Some(frame), prevFrames) => {
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut prevFrames = (*prevFrames).clone();
                    env = FGraph::pushScopeRef(env.clone(), frame.clone())?;
                    (cache, c, env, prevFrames) = lookupClass2(cache.clone(), env.clone(), path.clone(), prevFrames.clone(), inState.clone(), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env.clone(), prevFrames.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ SCode::Element::CLASS { name: id, .. }, None, _) => {
                    let mut prevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    r = FNode::child(FGraph::lastScopeRef(metamodelica::AsArg::as_arg(&env))?, id.clone())?;
                    ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                        Deref @ FCore::Data::CL { status: FCore::Status::CLS_INSTANCE { instanceOf: _ }, .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    (cache, env) = Inst::getCachedInstance(cache.clone(), env.clone(), id.clone(), r.clone())?;
                    (cache, c, env, prevFrames) = lookupClass2(cache.clone(), env.clone(), path.clone(), metamodelica::nil(), inState.clone(), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env.clone(), prevFrames.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ SCode::Element::CLASS { name: id, encapsulatedPrefix: encflag, restriction: restr, .. }, None, _) => {
                    let mut prevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                    let mut ci_state: ClassInf::State;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut r#mod: metamodelica::Ref<DAE::Mod>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    env = FGraph::openScope(env.clone(), encflag.clone(), id.clone(), FGraph::restrictionToScopeType(metamodelica::AsArg::as_arg(&restr)))?;
                    ci_state = ClassInfUtil::start(metamodelica::AsArg::as_arg(&restr), FGraph::getGraphName(metamodelica::AsArg::as_arg(&env))?)?;
                    r#mod = Mod::getClassModifier(&inEnv, id.clone());
                    (cache, env, _, _, _) = Inst::partialInstClassIn(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), r#mod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), inC.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0)?;
                    checkPartialScope(env.clone(), &inEnv, metamodelica::AsArg::as_arg(&cache), inInfo.clone())?;
                    (cache, c, env, prevFrames) = lookupClass2(cache.clone(), env.clone(), path.clone(), metamodelica::nil(), inState.clone(), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env.clone(), prevFrames.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outClass, outEnv, outPrevFrames))
}

fn checkPartialScope(
    mut inEnv: FCore::Graph,
    mut inParentEnv: &FCore::Graph,
    mut inCache: &FCore::Cache,
    mut inInfo: Option<SourceInfo>,
) -> Result<()> {
    let mut el: metamodelica::Ref<SCode::Element>;
    let mut pre: DAE::Prefix;
    let mut name: ArcStr;
    let mut pre_str: ArcStr;
    let mut cc_str: ArcStr;
    let mut cls_info: SourceInfo;
    let mut pre_info: SourceInfo;
    let mut info: SourceInfo;
    if (inInfo).is_some()
        && FGraph::isPartialScope(&inEnv)
        && Config::languageStandardAtLeast(Config::LanguageStandard::_3_2.clone())?
    {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FNode::fromRef(FGraph::lastScopeRef(&inEnv)?)) {
            Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e: __pa0, pre: __pa1, .. }, .. } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        el = metamodelica::Own::own(__pa0);
        pre = metamodelica::Own::own(__pa1);
        name = SCodeUtil::elementName(&el)?;
        if FGraph::graphPrefixOf(inParentEnv, &inEnv) && !(PrefixUtil::isNoPrefix(&pre)) {
            pre_str = PrefixUtil::printPrefixStr(&pre)?;
            cls_info = SCodeUtil::elementInfo(&el);
            pre_info = PrefixUtil::getPrefixInfo(&pre);
            cc_str = getConstrainingClass(&el, &((FGraph::stripLastScopeRef(inEnv)?).0), inCache)?;
            Error::addMultiSourceMessage(
                &(Error::USE_OF_PARTIAL_CLASS.clone()),
                &(list![pre_str, name, cc_str]),
                &(list![cls_info, pre_info]),
            )?;
            return Err("fail");
        } else {
            let __pa3 = ::match_deref::match_deref! { match &(inInfo) {
                Some(__pa3) => __pa3.clone(),
                _ => return Err("pattern mismatch"),
            } };
            info = metamodelica::Own::own(__pa3);
            if !(Config::getGraphicsExpMode()?) {
                Error::addSourceMessage(&(Error::LOOKUP_IN_PARTIAL_CLASS.clone()), list![name], &info)?;
            }
        }
    }
    Ok(())
}

fn getConstrainingClass(
    mut inClass: &metamodelica::Ref<SCode::Element>,
    mut inEnv: &FCore::Graph,
    mut inCache: &FCore::Cache,
) -> Result<ArcStr> {
    let mut outPath: ArcStr;
    outPath = 'mc: {
        let __mc_input = &**inClass;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { constrainingClass: cc_path, .. }) }, .. }, .. } => {
                    Ok(AbsynUtil::pathString(cc_path.clone(), literal!("."), true, false)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: ts, .. }, .. } => {
                    let mut el: metamodelica::Ref<SCode::Element>;
                    let mut env: FCore::Graph;
                    (_, el, env) = lookupClass(inCache, inEnv, &(AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&ts))), None)?;
                    Ok(getConstrainingClass(&el, &env, inCache)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*FGraph::printGraphPathStr(inEnv)); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*SCodeUtil::elementName(inClass)?); ArcStr::from(__mm_s) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outPath)
}

fn lookupPrevFrames(
    mut id: &ArcStr,
    mut inPrevFrames: &metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
) -> (
    Option<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
    metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
) {
    let mut outFrame: Option<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
    let mut outPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
    (outFrame, outPrevFrames) = 'mc: {
        let __mc_input = &**inPrevFrames;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: prevFrames } => {
                    let mut sid: ArcStr;
                    let false = (FNode::isRefTop(r#ref.clone())) else { return Err("pattern mismatch") };
                    sid = FNode::refName(r#ref.clone());
                    let true = (metamodelica::stringEq(&id, &sid)) else { return Err("pattern mismatch") };
                    Ok((Some(r#ref.clone()), prevFrames.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((None, metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outFrame, outPrevFrames)
}

fn lookupQualifiedImportedVarInFrame(
    mut inImports: &metamodelica::List<Absyn::Import>,
    mut ident: &ArcStr,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = &**inImports;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::QUAL_IMPORT { path }, tail: _ } => {
                    let mut id: ArcStr;
                    id = AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path));
                    let true = (metamodelica::stringEq(&id, &ident)) else { return Err("pattern mismatch") };
                    Ok(ComponentReference::pathToCref(metamodelica::AsArg::as_arg(&path)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::NAMED_IMPORT { name: id, path }, tail: _ } => {
                    let true = (metamodelica::stringEq(&id, &ident)) else { return Err("pattern mismatch") };
                    Ok(ComponentReference::pathToCref(metamodelica::AsArg::as_arg(&path)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(lookupQualifiedImportedVarInFrame(metamodelica::AsArg::as_arg(&rest), ident)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCref)
}

fn moreLookupUnqualifiedImportedVarInFrame(
    mut inCache: FCore::Cache,
    mut inImports: &metamodelica::List<Absyn::Import>,
    mut inEnv: FCore::Graph,
    mut inIdent: ArcStr,
) -> Result<(FCore::Cache, bool)> {
    let mut outCache: FCore::Cache;
    let mut outBoolean: bool;
    (outCache, outBoolean) = 'mc: {
        let __mc_input = (inCache, &**inImports, inEnv, inIdent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: _ }, env, ident) => {
                    let mut f: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                    let mut prevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(metamodelica::AsArg::as_arg(&env)).reverse()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    f = metamodelica::Own::own(__pa0);
                    prevFrames = metamodelica::Own::own(__pa1);
                    cref = ComponentReference::pathToCref(metamodelica::AsArg::as_arg(&path));
                    cref = ComponentReference::crefPrependIdent(&cref, metamodelica::AsArg::as_arg(&ident), &(metamodelica::nil()), &(DAE::T_UNKNOWN_DEFAULT().clone()))?;
                    env = FGraph::setScope(env.clone(), list![f.clone()])?;
                    (cache, _, _, _, _, _, _, _, _) = lookupVarInPackages(cache.clone(), env.clone(), cref.clone(), prevFrames.clone(), Mutable::create(false))?;
                    Ok((cache.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, env, ident) => {
                    let mut res: bool;
                    let mut cache = (*cache).clone();
                    (cache, res) = moreLookupUnqualifiedImportedVarInFrame(cache.clone(), metamodelica::AsArg::as_arg(&rest), env.clone(), ident.clone())?;
                    Ok((cache.clone(), res))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok((cache.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outBoolean))
}

fn lookupUnqualifiedImportedVarInFrame(
    mut inCache: FCore::Cache,
    mut inImports: &metamodelica::List<Absyn::Import>,
    mut inEnv: FCore::Graph,
    mut inIdent: ArcStr,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::Ref<DAE::Attributes>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Binding>,
    Option<DAE::Const>,
    bool,
    InstTypes::SplicedExpData,
    FCore::Graph,
    ArcStr,
)> {
    let mut outCache: FCore::Cache;
    let mut outClassEnv: FCore::Graph;
    let mut outAttributes: metamodelica::Ref<DAE::Attributes>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    let mut constOfForIteratorRange: Option<DAE::Const>;
    let mut outBoolean: bool;
    let mut splicedExpData: InstTypes::SplicedExpData =
        <InstTypes::SplicedExpData as ::std::default::Default>::default();
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (
        outCache,
        outClassEnv,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        outBoolean,
        splicedExpData,
        outComponentEnv,
        name,
    ) = 'mc: {
        let __mc_input = (inCache, &**inImports, inEnv, inIdent);
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: rest }, env, ident) => {
                    let mut f: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut more: bool;
                    let mut unique: bool;
                    let mut classEnv: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut prevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut bind: metamodelica::Ref<DAE::Binding>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(metamodelica::AsArg::as_arg(&env)).reverse()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    f = metamodelica::Own::own(__pa0);
                    prevFrames = metamodelica::Own::own(__pa1);
                    cref = ComponentReference::pathToCref(metamodelica::AsArg::as_arg(&path));
                    cref = ComponentReference::crefPrependIdent(&cref, metamodelica::AsArg::as_arg(&ident), &(metamodelica::nil()), &(DAE::T_UNKNOWN_DEFAULT().clone()))?;
                    env2 = FGraph::setScope(env.clone(), list![f.clone()])?;
                    (cache, classEnv, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarInPackages(cache.clone(), env2.clone(), cref.clone(), prevFrames.clone(), Mutable::create(false))?;
                    (cache, more) = moreLookupUnqualifiedImportedVarInFrame(cache.clone(), metamodelica::AsArg::as_arg(&rest), env.clone(), ident.clone())?;
                    unique = boolNot(more);
                    Ok(((cache.clone(), classEnv.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), unique, splicedExpData.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, env, ident) => {
                    let mut unique: bool;
                    let mut classEnv: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut bind: metamodelica::Ref<DAE::Binding>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    (cache, classEnv, attr, ty, bind, cnstForRange, unique, splicedExpData, componentEnv, name) = lookupUnqualifiedImportedVarInFrame(cache.clone(), metamodelica::AsArg::as_arg(&rest), env.clone(), ident.clone())?;
                    Ok(((cache.clone(), classEnv.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), unique, splicedExpData.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        outCache,
        outClassEnv,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        outBoolean,
        splicedExpData,
        outComponentEnv,
        name,
    ))
}

fn lookupQualifiedImportedClassInFrame(
    mut inCache: FCore::Cache,
    mut inImport: &metamodelica::List<Absyn::Import>,
    mut inEnv: FCore::Graph,
    mut inIdent: ArcStr,
    mut inState: Mutable::Mutable<bool>,
    mut inInfo: Option<SourceInfo>,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<SCode::Element>,
    FCore::Graph,
    metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
    (outCache, outClass, outEnv, outPrevFrames) = 'mc: {
        let __mc_input = (inCache, &**inImport, inEnv, inIdent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::QUAL_IMPORT { path: Deref @ Absyn::Path::IDENT { name: id } }, tail: _ }, env, ident) => {
                    let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut prevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let true = (metamodelica::stringEq(&id, &ident)) else { return Err("pattern mismatch") };
                    Mutable::update(inState.clone(), true);
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(metamodelica::AsArg::as_arg(&env)).reverse()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    r = metamodelica::Own::own(__pa0);
                    prevFrames = metamodelica::Own::own(__pa1);
                    env = FGraph::setScope(env.clone(), list![r.clone()])?;
                    (cache, c, env_1, prevFrames) = lookupClassInEnv(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&id), prevFrames.clone(), Mutable::create(false), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::QUAL_IMPORT { path }, tail: _ }, env, ident) => {
                    let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut prevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                    let mut id: ArcStr;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    id = AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path));
                    let true = (metamodelica::stringEq(&id, &ident)) else { return Err("pattern mismatch") };
                    Mutable::update(inState.clone(), true);
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(metamodelica::AsArg::as_arg(&env)).reverse()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    r = metamodelica::Own::own(__pa0);
                    prevFrames = metamodelica::Own::own(__pa1);
                    env = FGraph::setScope(env.clone(), list![r.clone()])?;
                    (cache, c, env_1, prevFrames) = lookupClass2(cache.clone(), env.clone(), path.clone(), prevFrames.clone(), Mutable::create(false), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::NAMED_IMPORT { name: id, path }, tail: _ }, env, ident) => {
                    let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut prevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let true = (metamodelica::stringEq(&id, &ident)) else { return Err("pattern mismatch") };
                    Mutable::update(inState.clone(), true);
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(metamodelica::AsArg::as_arg(&env)).reverse()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    r = metamodelica::Own::own(__pa0);
                    prevFrames = metamodelica::Own::own(__pa1);
                    env = FGraph::setScope(env.clone(), list![r.clone()])?;
                    (cache, c, env_1, prevFrames) = lookupClass2(cache.clone(), env.clone(), path.clone(), prevFrames.clone(), Mutable::create(false), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, env, ident) => {
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut prevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                    let mut cache = (*cache).clone();
                    (cache, c, env_1, prevFrames) = lookupQualifiedImportedClassInFrame(cache.clone(), metamodelica::AsArg::as_arg(&rest), env.clone(), ident.clone(), inState.clone(), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outClass, outEnv, outPrevFrames))
}

fn moreLookupUnqualifiedImportedClassInFrame(
    mut inCache: FCore::Cache,
    mut inImports: &metamodelica::List<Absyn::Import>,
    mut inEnv: FCore::Graph,
    mut inIdent: ArcStr,
) -> Result<(FCore::Cache, bool)> {
    let mut outCache: FCore::Cache;
    let mut outBoolean: bool;
    (outCache, outBoolean) = 'mc: {
        let __mc_input = (inCache, &**inImports, inEnv, inIdent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: _ }, env, ident) => {
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut id: ArcStr;
                    let mut encflag: SCode::Encapsulated;
                    let mut restr: SCode::Restriction;
                    let mut env_1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut ci_state: ClassInf::State;
                    let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                    let mut r#mod: metamodelica::Ref<DAE::Mod>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    env = FGraph::topScope(metamodelica::AsArg::as_arg(&env))?;
                    let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5) = ::match_deref::match_deref! { match &(lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&path), None)?) {
                        (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { name: __pa1, encapsulatedPrefix: __pa2, restriction: __pa3, .. }, __pa5) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    id = metamodelica::Own::own(__pa1);
                    encflag = metamodelica::Own::own(__pa2);
                    restr = metamodelica::Own::own(__pa3);
                    c = metamodelica::Own::own(__pa4);
                    env_1 = metamodelica::Own::own(__pa5);
                    env2 = FGraph::openScope(env_1.clone(), encflag, id.clone(), FGraph::restrictionToScopeType(&restr))?;
                    ci_state = ClassInfUtil::start(&restr, FGraph::getGraphName(&env2)?)?;
                    r#mod = Mod::getClassModifier(&env_1, id.clone());
                    (cache, env, _, _, _) = Inst::partialInstClassIn(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), r#mod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0)?;
                    r = FGraph::lastScopeRef(metamodelica::AsArg::as_arg(&env))?;
                    env = FGraph::setScope(env.clone(), list![r.clone()])?;
                    (cache, _, _) = lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &(metamodelica::Ref::new(Absyn::Path::IDENT { name: ident.clone() })), None)?;
                    Ok((cache.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, env, ident) => {
                    let mut res: bool;
                    let mut cache = (*cache).clone();
                    (cache, res) = moreLookupUnqualifiedImportedClassInFrame(cache.clone(), metamodelica::AsArg::as_arg(&rest), env.clone(), ident.clone())?;
                    Ok((cache.clone(), res))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok((cache.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outBoolean))
}

fn lookupUnqualifiedImportedClassInFrame(
    mut inCache: FCore::Cache,
    mut inImports: &metamodelica::List<Absyn::Import>,
    mut inEnv: FCore::Graph,
    mut inIdent: ArcStr,
    mut inInfo: Option<SourceInfo>,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<SCode::Element>,
    FCore::Graph,
    metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
    bool,
)> {
    let mut outCache: FCore::Cache;
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
    let mut outBoolean: bool;
    (outCache, outClass, outEnv, outPrevFrames, outBoolean) = 'mc: {
        let __mc_input = (inCache, &**inImports, inEnv, inIdent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: rest }, env, ident) => {
                    let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut c_1: metamodelica::Ref<SCode::Element>;
                    let mut id: ArcStr;
                    let mut encflag: SCode::Encapsulated;
                    let mut more: bool;
                    let mut unique: bool;
                    let mut restr: SCode::Restriction;
                    let mut env_1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut env3: FCore::Graph;
                    let mut prevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                    let mut ci_state: ClassInf::State;
                    let mut r#mod: metamodelica::Ref<DAE::Mod>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(FGraph::currentScope(metamodelica::AsArg::as_arg(&env)).reverse()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    r = metamodelica::Own::own(__pa0);
                    prevFrames = metamodelica::Own::own(__pa1);
                    env3 = FGraph::setScope(env.clone(), list![r.clone()])?;
                    let (__pa2, __pa6, __pa3, __pa4, __pa5, __pa7, __pa8) = ::match_deref::match_deref! { match &(lookupClass2(cache.clone(), env3.clone(), path.clone(), prevFrames.clone(), Mutable::create(false), inInfo.clone())?) {
                        (__pa2, __pa6 @ Deref @ SCode::Element::CLASS { name: __pa3, encapsulatedPrefix: __pa4, restriction: __pa5, .. }, __pa7, __pa8) => (__pa2.clone(), __pa6.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa7.clone(), __pa8.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa2);
                    id = metamodelica::Own::own(__pa3);
                    encflag = metamodelica::Own::own(__pa4);
                    restr = metamodelica::Own::own(__pa5);
                    c = metamodelica::Own::own(__pa6);
                    env_1 = metamodelica::Own::own(__pa7);
                    prevFrames = metamodelica::Own::own(__pa8);
                    env2 = FGraph::openScope(env_1.clone(), encflag, id.clone(), FGraph::restrictionToScopeType(&restr))?;
                    ci_state = ClassInfUtil::start(&restr, FGraph::getGraphName(&env2)?)?;
                    r#mod = Mod::getClassModifier(&env_1, id.clone());
                    (cache, env2, _, _, _) = Inst::partialInstClassIn(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), r#mod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0)?;
                    (cache, c_1, env2, prevFrames) = lookupClassInEnv(cache.clone(), env2.clone(), metamodelica::AsArg::as_arg(&ident), prevFrames.clone(), Mutable::create(true), inInfo.clone())?;
                    (cache, more) = moreLookupUnqualifiedImportedClassInFrame(cache.clone(), metamodelica::AsArg::as_arg(&rest), env.clone(), ident.clone())?;
                    unique = boolNot(more);
                    Ok((cache.clone(), c_1.clone(), env2.clone(), prevFrames.clone(), unique))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, env, ident) => {
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut unique: bool;
                    let mut env_1: FCore::Graph;
                    let mut prevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                    let mut cache = (*cache).clone();
                    (cache, c, env_1, prevFrames, unique) = lookupUnqualifiedImportedClassInFrame(cache.clone(), metamodelica::AsArg::as_arg(&rest), env.clone(), ident.clone(), inInfo.clone())?;
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone(), unique))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outClass, outEnv, outPrevFrames, outBoolean))
}

pub(crate) fn lookupRecordConstructorClass(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<(FCore::Cache, metamodelica::Ref<SCode::Element>, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outEnv: FCore::Graph;
    (outCache, outClass, outEnv) = (::match_deref::match_deref! { match &((inCache, inEnv, inPath)) {
        (cache, env, path) => {
            let mut c: metamodelica::Ref<SCode::Element>;
            let mut env_1: FCore::Graph;
            let mut cache = (*cache).clone();
            (cache, c, env_1) = lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&path), None)?;
            ::match_deref::match_deref! { match &(c.clone()) {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. } => (),
                _ => return Err("pattern mismatch"),
            } };
            (cache, _, c) = buildRecordConstructorClass(cache.clone(), env_1.clone(), c)?;
            (cache.clone(), c, env_1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outClass, outEnv))
}

pub(crate) fn lookupConnectorVar(
    mut env: &FCore::Graph,
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut firstId: bool,
) -> Result<(
    metamodelica::Ref<DAE::Attributes>,
    metamodelica::Ref<DAE::Type>,
    FCore::Status,
    bool,
)> {
    let mut attr: metamodelica::Ref<DAE::Attributes>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut status: FCore::Status;
    let mut isExpandable: bool = false;
    let mut comp_env: FCore::Graph;
    let mut parent_attr: metamodelica::Ref<DAE::Attributes>;
    (attr, ty, status) = (match &**cr {
        DAE::ComponentRef::CREF_IDENT {
            ident: __cr_ident,
            subscriptLst: __cr_subscriptLst,
            ..
        } => {
            let (__t3, __pa2, _) = lookupConnectorVar2(env.clone(), __cr_ident.clone())?;
            let __arc4 = __t3.clone();
            let DAE::TYPES_VAR {
                attributes: __pa0,
                ty: __pa1,
                ..
            } = &*__arc4;
            attr = metamodelica::Own::own(__pa0);
            ty = metamodelica::Own::own(__pa1);
            status = metamodelica::Own::own(__pa2);
            ty = checkSubscripts(ty, __cr_subscriptLst.clone())?;
            (attr, ty, status)
        }
        DAE::ComponentRef::CREF_QUAL {
            componentRef: __cr_componentRef,
            ident: __cr_ident,
            ..
        } => {
            let (__t4, __pa2, __pa3) = lookupConnectorVar2(env.clone(), __cr_ident.clone())?;
            let __arc5 = __t4.clone();
            let DAE::TYPES_VAR {
                attributes: __pa0,
                ty: __pa1,
                ..
            } = &*__arc5;
            parent_attr = metamodelica::Own::own(__pa0);
            ty = metamodelica::Own::own(__pa1);
            status = metamodelica::Own::own(__pa2);
            comp_env = metamodelica::Own::own(__pa3);
            if FCore::isDeletedComp(&status) {
                attr = parent_attr;
            } else {
                match '__try6: {
                    (attr, ty, status, isExpandable) = unwrap_break_err!(lookupConnectorVar(&comp_env, metamodelica::AsArg::as_arg(&__cr_componentRef), false), '__try6);
                    Ok::<_, &'static str>((attr.clone(), isExpandable.clone()))
                } {
                    Ok((__try6_o0, __try6_o1)) => {
                        attr = __try6_o0;
                        isExpandable = __try6_o1;
                    }
                    Err(_) => {
                        if Types::isExpandableConnector(&ty) {
                            attr = parent_attr.clone();
                            isExpandable = true;
                        } else {
                            return Err("fail");
                        }
                    }
                }
                attr = DAEUtil::setAttrVariability(
                    attr.clone(),
                    SCodeUtil::variabilityOr(
                        DAEUtil::getAttrVariability(&attr),
                        DAEUtil::getAttrVariability(&parent_attr),
                    ),
                );
                if firstId {
                    attr = DAEUtil::setAttrInnerOuter(attr, DAEUtil::getAttrInnerOuter(&parent_attr));
                }
            }
            (attr, ty, status)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((attr, ty, status, isExpandable))
}

fn lookupConnectorVar2(
    mut env: FCore::Graph,
    mut name: ArcStr,
) -> Result<(metamodelica::Ref<DAE::Var>, FCore::Status, FCore::Graph)> {
    let mut var: metamodelica::Ref<DAE::Var>;
    let mut status: FCore::Status;
    let mut compEnv: FCore::Graph;
    let mut scope: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
    let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
    let FCore::G { scope: __pa0, .. } = (env.clone()) else {
        return Err("pattern mismatch");
    };
    scope = metamodelica::Own::own(__pa0);
    for mut r in &*scope {
        ht = FNode::children(&(FNode::fromRef(r.clone())));
        if '__try1: {
            (var, _, _, status, compEnv) = unwrap_break_err!(lookupVar2(&ht, name.clone(), env.clone()), '__try1);
            return Ok((var, status, compEnv));
            Ok::<(), &'static str>(())
        }
        .is_err()
        {
            let true = (FNode::isImplicitRefName(r.clone())) else {
                return Err("pattern mismatch");
            };
        }
    }
    return Err("fail");
    Ok((var, status, compEnv))
}

pub fn lookupVar(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Attributes>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Binding>,
    Option<DAE::Const>,
    InstTypes::SplicedExpData,
    FCore::Graph,
    FCore::Graph,
    ArcStr,
)> {
    let mut outCache: FCore::Cache;
    let mut outAttributes: metamodelica::Ref<DAE::Attributes>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    let mut constOfForIteratorRange: Option<DAE::Const>;
    let mut outSplicedExpData: InstTypes::SplicedExpData;
    let mut outClassEnv: FCore::Graph;
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (
        outCache,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        outSplicedExpData,
        outClassEnv,
        outComponentEnv,
        name,
    ) = 'mc: {
        let __mc_input = (inCache, inEnv, inComponentRef);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref) => {
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut componentEnv: FCore::Graph;
                    let mut classEnv: FCore::Graph;
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    (cache, attr, ty, binding, cnstForRange, splicedExpData, classEnv, componentEnv, name) = lookupVarInternal(cache.clone(), metamodelica::AsArg::as_arg(&env), cref.clone(), openmodelica_frontend_inst::InstTypes::SearchStrategy::SEARCH_ALSO_BUILTIN)?;
                    Ok(((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), classEnv.clone(), componentEnv.clone(), name.clone()), name.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref) => {
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut componentEnv: FCore::Graph;
                    let mut classEnv: FCore::Graph;
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    (cache, classEnv, attr, ty, binding, cnstForRange, splicedExpData, componentEnv, name) = lookupVarInPackages(cache.clone(), env.clone(), cref.clone(), metamodelica::nil(), Mutable::create(false))?;
                    checkPackageVariableConstant(metamodelica::AsArg::as_arg(&env), &classEnv, &componentEnv, &attr, &ty, metamodelica::AsArg::as_arg(&cref))?;
                    Ok(((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), classEnv.clone(), componentEnv.clone(), name.clone()), name.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _) => {
                    if !((Config::getGraphicsExpMode()?)) { return Err("guard") }
                    Ok((cache.clone(), DAE::dummyAttrConst().clone(), DAE::T_UNKNOWN_DEFAULT().clone(), openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), None, InstTypes::SplicedExpData { splicedExp: None, identType: DAE::T_UNKNOWN_DEFAULT().clone() }, env.clone(), env.clone(), literal!("#varNotFound#")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        outCache,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        outSplicedExpData,
        outClassEnv,
        outComponentEnv,
        name,
    ))
}

pub(crate) fn lookupVarIdent(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut ident: ArcStr,
    mut ss: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Attributes>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Binding>,
    Option<DAE::Const>,
    InstTypes::SplicedExpData,
    FCore::Graph,
    FCore::Graph,
    ArcStr,
)> {
    let mut outCache: FCore::Cache;
    let mut outAttributes: metamodelica::Ref<DAE::Attributes>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    let mut constOfForIteratorRange: Option<DAE::Const>;
    let mut outSplicedExpData: InstTypes::SplicedExpData;
    let mut outClassEnv: FCore::Graph;
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (
        outCache,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        outSplicedExpData,
        outClassEnv,
        outComponentEnv,
        name,
    ) = 'mc: {
        let __mc_input = (inCache, inEnv);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (mut cache, mut env) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut attr: metamodelica::Ref<DAE::Attributes>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut binding: metamodelica::Ref<DAE::Binding>;
            let mut componentEnv: FCore::Graph;
            let mut classEnv: FCore::Graph;
            let mut splicedExpData: InstTypes::SplicedExpData;
            let mut cnstForRange: Option<DAE::Const>;
            let mut name: ArcStr = name.clone();
            (
                cache,
                attr,
                ty,
                binding,
                cnstForRange,
                splicedExpData,
                classEnv,
                componentEnv,
                name,
            ) = lookupVarInternalIdent(
                cache.clone(),
                &(env.clone()),
                &ident,
                &ss,
                openmodelica_frontend_inst::InstTypes::SearchStrategy::SEARCH_ALSO_BUILTIN,
            )?;
            Ok((
                (
                    cache.clone(),
                    attr.clone(),
                    ty.clone(),
                    binding.clone(),
                    cnstForRange.clone(),
                    splicedExpData.clone(),
                    classEnv.clone(),
                    componentEnv.clone(),
                    name.clone(),
                ),
                name.clone(),
            ))
        })() {
            name = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (mut cache, mut env) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut attr: metamodelica::Ref<DAE::Attributes>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut binding: metamodelica::Ref<DAE::Binding>;
            let mut componentEnv: FCore::Graph;
            let mut classEnv: FCore::Graph;
            let mut cref: metamodelica::Ref<DAE::ComponentRef>;
            let mut splicedExpData: InstTypes::SplicedExpData;
            let mut cnstForRange: Option<DAE::Const>;
            let mut name: ArcStr = name.clone();
            cref = ComponentReferenceBasics::makeCrefIdent(ident.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), ss.clone());
            (
                cache,
                classEnv,
                attr,
                ty,
                binding,
                cnstForRange,
                splicedExpData,
                componentEnv,
                name,
            ) = lookupVarInPackages(
                cache.clone(),
                env.clone(),
                cref.clone(),
                metamodelica::nil(),
                Mutable::create(false),
            )?;
            checkPackageVariableConstant(&(env.clone()), &classEnv, &componentEnv, &attr, &ty, &cref)?;
            Ok((
                (
                    cache.clone(),
                    attr.clone(),
                    ty.clone(),
                    binding.clone(),
                    cnstForRange.clone(),
                    splicedExpData.clone(),
                    classEnv.clone(),
                    componentEnv.clone(),
                    name.clone(),
                ),
                name.clone(),
            ))
        })() {
            name = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        outCache,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        outSplicedExpData,
        outClassEnv,
        outComponentEnv,
        name,
    ))
}

fn checkPackageVariableConstant(
    mut parentEnv: &FCore::Graph,
    mut classEnv: &FCore::Graph,
    mut componentEnv: &FCore::Graph,
    mut attr: &metamodelica::Ref<DAE::Attributes>,
    mut tp: &metamodelica::Ref<DAE::Type>,
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<()> {
    let () = (match &**attr {
        DAE::Attributes {
            variability: SCode::Variability::CONST { .. },
            ..
        } => (),
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = ComponentReferenceBasics::printComponentRefStr(cref)?;
            s2 = FGraph::printGraphPathStr(classEnv);
            Error::addMessage(
                Error::PACKAGE_VARIABLE_NOT_CONSTANT.clone(),
                list![s1.clone(), s2.clone()],
            )?;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("- Lookup.checkPackageVariableConstant failed: "));
                __mm_s.push_str(&*s1);
                __mm_s.push_str(&*literal!(" in "));
                __mm_s.push_str(&*s2);
                ArcStr::from(__mm_s)
            })?;
            return Err("fail");
        }
    });
    Ok(())
}

pub(crate) fn lookupVarInternal(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut searchStrategy: InstTypes::SearchStrategy,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Attributes>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Binding>,
    Option<DAE::Const>,
    InstTypes::SplicedExpData,
    FCore::Graph,
    FCore::Graph,
    ArcStr,
)> {
    let mut outCache: FCore::Cache;
    let mut outAttributes: metamodelica::Ref<DAE::Attributes>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    let mut constOfForIteratorRange: Option<DAE::Const>;
    let mut splicedExpData: InstTypes::SplicedExpData =
        <InstTypes::SplicedExpData as ::std::default::Default>::default();
    let mut outClassEnv: FCore::Graph;
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (
        outCache,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        splicedExpData,
        outClassEnv,
        outComponentEnv,
        name,
    ) = 'mc: {
        let __mc_input = (inCache, inEnv, inComponentRef, searchStrategy);
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, r#ref, _) => {
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut componentEnv: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    ht = FNode::children(&(FNode::fromRef(r.clone())));
                    (cache, attr, ty, binding, cnstForRange, splicedExpData, componentEnv, name) = lookupVarF(cache.clone(), ht.clone(), metamodelica::AsArg::as_arg(&r#ref), inEnv.clone())?;
                    Ok(((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), inEnv.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, r#ref, _) => {
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    let true = (FNode::isImplicitRefName(r.clone())) else { return Err("pattern mismatch") };
                    (env, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                    (cache, attr, ty, binding, cnstForRange, splicedExpData, env, componentEnv, name) = lookupVarInternal(cache.clone(), &env, r#ref.clone(), searchStrategy)?;
                    Ok(((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), env.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, .. }, r#ref, InstTypes::SearchStrategy::SEARCH_ALSO_BUILTIN { .. }) => {
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    let true = (Builtin::variableIsBuiltin(metamodelica::AsArg::as_arg(&r#ref))?) else { return Err("pattern mismatch") };
                    env = FGraph::topScope(inEnv)?;
                    ht = FNode::children(&(FNode::fromRef(FGraph::lastScopeRef(&env)?)));
                    (cache, attr, ty, binding, cnstForRange, splicedExpData, componentEnv, name) = lookupVarF(cache.clone(), ht.clone(), metamodelica::AsArg::as_arg(&r#ref), env.clone())?;
                    Ok(((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), env.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        outCache,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        splicedExpData,
        outClassEnv,
        outComponentEnv,
        name,
    ))
}

pub(crate) fn lookupVarInternalIdent(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut ident: &ArcStr,
    mut ss: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut searchStrategy: InstTypes::SearchStrategy,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Attributes>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Binding>,
    Option<DAE::Const>,
    InstTypes::SplicedExpData,
    FCore::Graph,
    FCore::Graph,
    ArcStr,
)> {
    let mut outCache: FCore::Cache;
    let mut outAttributes: metamodelica::Ref<DAE::Attributes>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    let mut constOfForIteratorRange: Option<DAE::Const>;
    let mut splicedExpData: InstTypes::SplicedExpData =
        <InstTypes::SplicedExpData as ::std::default::Default>::default();
    let mut outClassEnv: FCore::Graph;
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (
        outCache,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        splicedExpData,
        outClassEnv,
        outComponentEnv,
        name,
    ) = 'mc: {
        let __mc_input = (inCache, inEnv, searchStrategy);
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, _) => {
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut componentEnv: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    ht = FNode::children(&(FNode::fromRef(r.clone())));
                    (cache, attr, ty, binding, cnstForRange, splicedExpData, componentEnv, name) = lookupVarFIdent(cache.clone(), &ht, ident.clone(), ss.clone(), inEnv.clone())?;
                    Ok(((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), inEnv.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, _) => {
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    let true = (FNode::isImplicitRefName(r.clone())) else { return Err("pattern mismatch") };
                    (env, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                    (cache, attr, ty, binding, cnstForRange, splicedExpData, env, componentEnv, name) = lookupVarInternalIdent(cache.clone(), &env, ident, ss, searchStrategy)?;
                    Ok(((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), env.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, .. }, InstTypes::SearchStrategy::SEARCH_ALSO_BUILTIN { .. }) => {
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    let true = (Builtin::variableNameIsBuiltin(ident)?) else { return Err("pattern mismatch") };
                    env = FGraph::topScope(inEnv)?;
                    ht = FNode::children(&(FNode::fromRef(FGraph::lastScopeRef(&env)?)));
                    (cache, attr, ty, binding, cnstForRange, splicedExpData, componentEnv, name) = lookupVarFIdent(cache.clone(), &ht, ident.clone(), ss.clone(), env.clone())?;
                    Ok(((cache.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), splicedExpData.clone(), env.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        outCache,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        splicedExpData,
        outClassEnv,
        outComponentEnv,
        name,
    ))
}

fn frameIsImplAddedScope(mut f: &metamodelica::Ref<FCore::Node>) -> bool {
    let mut b: bool;
    b = (match &**f {
        FCore::Node { name: oname, .. } => FCore::isImplicitScope(oname.clone()),
        _ => false,
    });
    b
}

pub(crate) fn lookupVarInPackages(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
    mut inState: Mutable::Mutable<bool>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::Ref<DAE::Attributes>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Binding>,
    Option<DAE::Const>,
    InstTypes::SplicedExpData,
    FCore::Graph,
    ArcStr,
)> {
    let mut outCache: FCore::Cache;
    let mut outClassEnv: FCore::Graph;
    let mut outAttributes: metamodelica::Ref<DAE::Attributes>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    let mut constOfForIteratorRange: Option<DAE::Const>;
    let mut splicedExpData: InstTypes::SplicedExpData =
        <InstTypes::SplicedExpData as ::std::default::Default>::default();
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (
        outCache,
        outClassEnv,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        splicedExpData,
        outComponentEnv,
        name,
    ) = 'mc: {
        let __mc_input = (inCache, inEnv.clone(), inComponentRef.clone(), inPrevFrames.clone());
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, subscriptLst: Deref @ metamodelica::ListNode::Nil, componentRef: cref, .. }, prevFrames) => {
                            let mut c: metamodelica::Ref<SCode::Element>;
                            let mut n: ArcStr;
                            let mut encflag: SCode::Encapsulated;
                            let mut r: SCode::Restriction;
                            let mut env2: FCore::Graph;
                            let mut env3: FCore::Graph;
                            let mut env5: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
                            let mut p_env: FCore::Graph;
                            let mut componentEnv: FCore::Graph;
                            let mut ci_state: ClassInf::State;
                            let mut attr: metamodelica::Ref<DAE::Attributes>;
                            let mut ty: metamodelica::Ref<DAE::Type>;
                            let mut bind: metamodelica::Ref<DAE::Binding>;
                            let mut f: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                            let mut rr: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                            let mut of: Option<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                            let mut cnstForRange: Option<DAE::Const>;
                            let mut r#mod: metamodelica::Ref<DAE::Mod>;
                            let mut cache = (*cache).clone();
                            let mut prevFrames = (*prevFrames).clone();
                            let mut name: ArcStr = name.clone();
                            let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                            (of, prevFrames) = lookupPrevFrames(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&prevFrames));
                            let () = (match of.clone() {
                Some(mut __esc_f) => {
                            f = __esc_f.clone();
                            Mutable::update(inState.clone(), true);
                            env5 = FGraph::pushScopeRef(env.clone(), f.clone())?;
                            ()
                },
                None => {
                            let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5, __pa6) = ::match_deref::match_deref! { match &(lookupClassInEnv(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&id), prevFrames.clone(), Mutable::create(true), None)?) {
                                (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { name: __pa1, encapsulatedPrefix: __pa2, restriction: __pa3, .. }, __pa5, __pa6) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone(), __pa6.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            cache = metamodelica::Own::own(__pa0);
                            n = metamodelica::Own::own(__pa1);
                            encflag = metamodelica::Own::own(__pa2);
                            r = metamodelica::Own::own(__pa3);
                            c = metamodelica::Own::own(__pa4);
                            env2 = metamodelica::Own::own(__pa5);
                            prevFrames = metamodelica::Own::own(__pa6);
                            Mutable::update(inState.clone(), true);
                            rr = FNode::child(FGraph::lastScopeRef(&env2)?, id.clone())?;
                            if FNode::isRefInstance(rr.clone()) {
                                (cache, env5) = Inst::getCachedInstance(cache.clone(), env2.clone(), id.clone(), rr.clone())?;
                            } else {
                                env3 = FGraph::openScope(env2.clone(), encflag, n.clone(), FGraph::restrictionToScopeType(&r))?;
                                ci_state = ClassInfUtil::start(&r, FGraph::getGraphName(&env3)?)?;
                                r#mod = Mod::getClassModifier(&env2, n.clone());
                                (cache, env5, _, _, _, _, _, _, _, _, _, _) = Inst::instClassIn(cache.clone(), env3.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), r#mod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), false, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), None)?;
                            }
                            ()
                },
            });
                            (cache, p_env, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarInPackages(cache.clone(), env5.clone(), cref.clone(), prevFrames.clone(), inState.clone())?;
                            splicedExpData = prefixSplicedExp(&(ComponentReferenceBasics::crefFirstCref(inComponentRef.clone())?), splicedExpData.clone())?;
                            Ok(((cache.clone(), p_env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr @ Deref @ DAE::ComponentRef::CREF_IDENT { .. }, _) => {
                    let mut componentEnv: FCore::Graph;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut bind: metamodelica::Ref<DAE::Binding>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    (cache, env, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarInPackagesIdent(cache.clone(), env.clone(), var_field!((**cr).ident, DAE::ComponentRef::CREF_IDENT), var_field!((**cr).subscriptLst, DAE::ComponentRef::CREF_IDENT), inPrevFrames.clone(), inState.clone())?;
                    Ok(((cache.clone(), env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr @ Deref @ DAE::ComponentRef::CREF_QUAL { .. }, _) => {
                    let mut componentEnv: FCore::Graph;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut bind: metamodelica::Ref<DAE::Binding>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    ht = FNode::children(&(FNode::fromRef(FGraph::lastScopeRef(metamodelica::AsArg::as_arg(&env))?)));
                    (cache, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarF(cache.clone(), ht.clone(), metamodelica::AsArg::as_arg(&cr), env.clone())?;
                    Ok(((cache.clone(), env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: f, tail: fs }, .. }, cr @ Deref @ DAE::ComponentRef::CREF_QUAL { .. }, prevFrames) => {
                    let mut env: FCore::Graph;
                    let mut p_env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut bind: metamodelica::Ref<DAE::Binding>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    let false = (Mutable::access(inState.clone())) else { return Err("pattern mismatch") };
                    env = FGraph::setScope(inEnv.clone(), fs.clone())?;
                    (cache, p_env, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarInPackages(cache.clone(), env.clone(), cr.clone(), metamodelica::cons(f.clone(), prevFrames.clone()), inState.clone())?;
                    Ok(((cache.clone(), p_env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        outCache,
        outClassEnv,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        splicedExpData,
        outComponentEnv,
        name,
    ))
}

pub(crate) fn lookupVarInPackagesIdent(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut id: &ArcStr,
    mut ss: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
    mut inState: Mutable::Mutable<bool>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::Ref<DAE::Attributes>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Binding>,
    Option<DAE::Const>,
    InstTypes::SplicedExpData,
    FCore::Graph,
    ArcStr,
)> {
    let mut outCache: FCore::Cache;
    let mut outClassEnv: FCore::Graph;
    let mut outAttributes: metamodelica::Ref<DAE::Attributes>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    let mut constOfForIteratorRange: Option<DAE::Const>;
    let mut splicedExpData: InstTypes::SplicedExpData =
        <InstTypes::SplicedExpData as ::std::default::Default>::default();
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr = arcstr::literal!("");
    (
        outCache,
        outClassEnv,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        splicedExpData,
        outComponentEnv,
        name,
    ) = 'mc: {
        let __mc_input = (inCache, inEnv.clone(), inPrevFrames);
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _) => {
                    let mut componentEnv: FCore::Graph;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut bind: metamodelica::Ref<DAE::Binding>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    (cache, attr, ty, bind, cnstForRange, splicedExpData, _, componentEnv, name) = lookupVarInternalIdent(cache.clone(), metamodelica::AsArg::as_arg(&env), id, ss, openmodelica_frontend_inst::InstTypes::SearchStrategy::SEARCH_LOCAL_ONLY)?;
                    Mutable::update(inState.clone(), true);
                    Ok(((cache.clone(), env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _) => {
                    let mut componentEnv: FCore::Graph;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut bind: metamodelica::Ref<DAE::Binding>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    ht = FNode::children(&(FNode::fromRef(FGraph::lastScopeRef(metamodelica::AsArg::as_arg(&env))?)));
                    (cache, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarFIdent(cache.clone(), &ht, id.clone(), ss.clone(), env.clone())?;
                    Ok(((cache.clone(), env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, prevFrames) => {
                    let mut p_env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut node: metamodelica::Ref<FCore::Node>;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut bind: metamodelica::Ref<DAE::Binding>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut f: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut unique: bool;
                    let mut qimports: metamodelica::List<Absyn::Import>;
                    let mut uqimports: metamodelica::List<Absyn::Import>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut prevFrames = (*prevFrames).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    node = FNode::fromRef(FGraph::lastScopeRef(metamodelica::AsArg::as_arg(&env))?);
                    (qimports, uqimports) = FNode::imports(node.clone())?;
                    match '__try0: {
                        let false = ((qimports).is_empty()) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        cr = unwrap_break_err!(lookupQualifiedImportedVarInFrame(&qimports, id), '__try0);
                        Mutable::update(inState.clone(), true);
                        cr = if (metamodelica::stringEq(&(FNode::name(&(FNode::fromRef(unwrap_break_err!(FGraph::lastScopeRef(metamodelica::AsArg::as_arg(&env)), '__try0))))), &(unwrap_break_err!(ComponentReferenceBasics::crefFirstIdent(&cr), '__try0)))) {unwrap_break_err!(ComponentReference::crefStripFirstIdent(&cr), '__try0)} else {cr.clone()};
                        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(FGraph::currentScope(metamodelica::AsArg::as_arg(&env)).reverse()) {
                            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        f = metamodelica::Own::own(__pa1);
                        prevFrames = metamodelica::Own::own(__pa2);
                        env = unwrap_break_err!(FGraph::setScope(env.clone(), list![f.clone()]), '__try0);
                        (cache, p_env, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = unwrap_break_err!(lookupVarInPackages(cache.clone(), env.clone(), cr.clone(), prevFrames.clone(), inState.clone()), '__try0);
                        Ok::<_, &'static str>((attr.clone(), bind.clone(), cache.clone(), cnstForRange.clone(), componentEnv.clone(), name.clone(), p_env.clone(), splicedExpData.clone(), ty.clone()))
                    } {
                        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8)) => {
                            attr = __try0_o0;
                            bind = __try0_o1;
                            cache = __try0_o2;
                            cnstForRange = __try0_o3;
                            componentEnv = __try0_o4;
                            name = __try0_o5;
                            p_env = __try0_o6;
                            splicedExpData = __try0_o7;
                            ty = __try0_o8;
                        }
                        Err(_) => {
                            let false = ((uqimports).is_empty()) else { return Err("pattern mismatch") };
                            (cache, p_env, attr, ty, bind, cnstForRange, unique, splicedExpData, componentEnv, name) = lookupUnqualifiedImportedVarInFrame(cache.clone(), &uqimports, env.clone(), id.clone())?;
                            reportSeveralNamesError(unique, id.clone())?;
                            Mutable::update(inState.clone(), true);
                        }
                    }
                    Ok(((cache.clone(), p_env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: f, tail: fs }, .. }, prevFrames) => {
                    let mut env: FCore::Graph;
                    let mut p_env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut bind: metamodelica::Ref<DAE::Binding>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut cache = (*cache).clone();
                    let mut name: ArcStr = name.clone();
                    let mut splicedExpData: InstTypes::SplicedExpData = splicedExpData.clone();
                    let false = (Mutable::access(inState.clone())) else { return Err("pattern mismatch") };
                    env = FGraph::setScope(inEnv.clone(), fs.clone())?;
                    (cache, p_env, attr, ty, bind, cnstForRange, splicedExpData, componentEnv, name) = lookupVarInPackagesIdent(cache.clone(), env.clone(), id, ss, metamodelica::cons(f.clone(), prevFrames.clone()), inState.clone())?;
                    Ok(((cache.clone(), p_env.clone(), attr.clone(), ty.clone(), bind.clone(), cnstForRange.clone(), splicedExpData.clone(), componentEnv.clone(), name.clone()), name.clone(), splicedExpData.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            splicedExpData = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        outCache,
        outClassEnv,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        splicedExpData,
        outComponentEnv,
        name,
    ))
}

pub(crate) fn lookupVarLocal(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Attributes>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Binding>,
    Option<DAE::Const>,
    InstTypes::SplicedExpData,
    FCore::Graph,
    FCore::Graph,
    ArcStr,
)> {
    let mut outCache: FCore::Cache;
    let mut outAttributes: metamodelica::Ref<DAE::Attributes>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    let mut constOfForIteratorRange: Option<DAE::Const>;
    let mut splicedExpData: InstTypes::SplicedExpData;
    let mut outClassEnv: FCore::Graph;
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr;
    (
        outCache,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        splicedExpData,
        outClassEnv,
        outComponentEnv,
        name,
    ) = lookupVarInternal(
        inCache,
        inEnv,
        inComponentRef,
        openmodelica_frontend_inst::InstTypes::SearchStrategy::SEARCH_LOCAL_ONLY,
    )?;
    Ok((
        outCache,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        splicedExpData,
        outClassEnv,
        outComponentEnv,
        name,
    ))
}

pub(crate) fn lookupIdentLocal(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIdent: ArcStr,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Var>,
    metamodelica::Ref<SCode::Element>,
    metamodelica::Ref<DAE::Mod>,
    FCore::Status,
    FCore::Graph,
)> {
    let mut outCache: FCore::Cache;
    let mut outVar: metamodelica::Ref<DAE::Var>;
    let mut outElement: metamodelica::Ref<SCode::Element>;
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    let mut instStatus: FCore::Status;
    let mut outComponentEnv: FCore::Graph;
    (outCache, outVar, outElement, outMod, instStatus, outComponentEnv) = 'mc: {
        let __mc_input = (inCache, inEnv, inIdent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, id) => {
                    let mut fv: metamodelica::Ref<DAE::Var>;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut m: metamodelica::Ref<DAE::Mod>;
                    let mut i: FCore::Status;
                    let mut componentEnv: FCore::Graph;
                    let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
                    ht = FNode::children(&(FNode::fromRef(r.clone())));
                    (fv, c, m, i, componentEnv) = lookupVar2(&ht, id.clone(), inEnv.clone())?;
                    Ok((cache.clone(), fv.clone(), c.clone(), m.clone(), i.clone(), componentEnv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, id) => {
                    let mut fv: metamodelica::Ref<DAE::Var>;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut m: metamodelica::Ref<DAE::Mod>;
                    let mut i: FCore::Status;
                    let mut env: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let true = (FNode::isImplicitRefName(r.clone())) else { return Err("pattern mismatch") };
                    (env, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                    (cache, fv, c, m, i, componentEnv) = lookupIdentLocal(cache.clone(), &env, id.clone())?;
                    Ok((cache.clone(), fv.clone(), c.clone(), m.clone(), i.clone(), componentEnv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outVar, outElement, outMod, instStatus, outComponentEnv))
}

pub(crate) fn lookupClassLocal(
    mut inEnv: FCore::Graph,
    mut inIdent: ArcStr,
) -> Result<(metamodelica::Ref<SCode::Element>, FCore::Graph)> {
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outEnv: FCore::Graph;
    (outClass, outEnv) = (::match_deref::match_deref! { match &(inEnv) {
        env @ FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. } => {
            let mut id = inIdent;
            let mut cl: metamodelica::Ref<SCode::Element>;
            let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
            let mut r = (*r).clone();
            ht = FNode::children(&(FNode::fromRef(r.clone())));
            r = FCore::RefTree::get(&ht, id)?;
            let __pa0 = ::match_deref::match_deref! { match &(FNode::fromRef(r.clone())) {
                Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e: __pa0, .. }, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cl = metamodelica::Own::own(__pa0);
            (cl, env.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outClass, outEnv))
}

pub(crate) fn lookupIdent(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIdent: ArcStr,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Var>,
    metamodelica::Ref<SCode::Element>,
    metamodelica::Ref<DAE::Mod>,
    FCore::Status,
    FCore::Graph,
)> {
    let mut outCache: FCore::Cache;
    let mut outVar: metamodelica::Ref<DAE::Var>;
    let mut outElement: metamodelica::Ref<SCode::Element>;
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    let mut instStatus: FCore::Status;
    let mut outEnv: FCore::Graph;
    (outCache, outVar, outElement, outMod, instStatus, outEnv) = 'mc: {
        let __mc_input = (inCache, inEnv, inIdent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, id) => {
                    let mut fv: metamodelica::Ref<DAE::Var>;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut m: metamodelica::Ref<DAE::Mod>;
                    let mut i: FCore::Status;
                    let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
                    ht = FNode::children(&(FNode::fromRef(r.clone())));
                    (fv, c, m, i, _) = lookupVar2(&ht, id.clone(), inEnv.clone())?;
                    Ok((cache.clone(), fv.clone(), c.clone(), m.clone(), i.clone(), inEnv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, id) => {
                    let mut fv: metamodelica::Ref<DAE::Var>;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut m: metamodelica::Ref<DAE::Mod>;
                    let mut i: FCore::Status;
                    let mut e: FCore::Graph;
                    let mut cache = (*cache).clone();
                    (e, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                    (cache, fv, c, m, i, e) = lookupIdent(cache.clone(), &e, id.clone())?;
                    Ok((cache.clone(), fv.clone(), c.clone(), m.clone(), i.clone(), e.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outVar, outElement, outMod, instStatus, outEnv))
}

// Function lookup
pub(crate) fn lookupFunctionsInEnv(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inId: metamodelica::Ref<Absyn::Path>,
    mut inInfo: SourceInfo,
) -> (FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Type>>) {
    let mut outCache: FCore::Cache;
    let mut outTypesTypeLst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (outCache, outTypesTypeLst) = 'mc: {
        let __mc_input = (inCache, inEnv, inId, inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Path::QUALIFIED { name, path: id }, info) => {
                    let mut cenv: FCore::Graph;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut cache = (*cache).clone();
                    ErrorExt::setCheckpoint(literal!("functionViaComponentRef"));
                    (cache, _, _, _, _, _, _, cenv, _) = lookupVarIdent(cache.clone(), env.clone(), name.clone(), metamodelica::nil())?;
                    (cache, res) = lookupFunctionsInEnv(cache.clone(), cenv.clone(), id.clone(), info.clone());
                    ErrorExt::rollBack(literal!("functionViaComponentRef"));
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ Absyn::Path::QUALIFIED { name: _, path: _ }, _) => {
                    ErrorExt::rollBack(literal!("functionViaComponentRef"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, id, _) => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut name: ArcStr;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    env = FGraph::selectScope(env.clone(), metamodelica::AsArg::as_arg(&id))?;
                    name = AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&id));
                    (cache, res) = lookupFunctionsInEnv(cache.clone(), env.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), inInfo.clone());
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Path::IDENT { name: r#str }, info) => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut httypes: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    Static::elabBuiltinHandler(metamodelica::AsArg::as_arg(&r#str))?;
                    env = FGraph::topScope(metamodelica::AsArg::as_arg(&env))?;
                    ht = FNode::children(&(FNode::fromRef(FGraph::lastScopeRef(metamodelica::AsArg::as_arg(&env))?)));
                    httypes = getHtTypes(FGraph::lastScopeRef(metamodelica::AsArg::as_arg(&env))?);
                    (cache, res) = lookupFunctionsInFrame(cache.clone(), &ht, &httypes, env.clone(), r#str.clone(), metamodelica::AsArg::as_arg(&info))?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Path::IDENT { name: r#str @ Deref @ "cardinality" }, _) => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut env = (*env).clone();
                    env = FGraph::topScope(metamodelica::AsArg::as_arg(&env))?;
                    res = createGenericBuiltinFunctions(metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&r#str))?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, id, info) => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut cache = (*cache).clone();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(id.clone()) {
                            Deref @ Absyn::Path::FULLYQUALIFIED { path: _ } => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    (cache, res) = lookupFunctionsInEnv2(cache.clone(), metamodelica::AsArg::as_arg(&env), id.clone(), false, metamodelica::AsArg::as_arg(&info))?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Path::FULLYQUALIFIED { path: id }, info) => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    env = FGraph::topScope(metamodelica::AsArg::as_arg(&env))?;
                    (cache, res) = lookupFunctionsInEnv2(cache.clone(), metamodelica::AsArg::as_arg(&env), id.clone(), true, metamodelica::AsArg::as_arg(&info))?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, id, _) => {
                            let mut env_1: FCore::Graph;
                            let mut res: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                            let mut names: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                            let mut info: SourceInfo;
                            let mut cache = (*cache).clone();
                            let mut id = (*id).clone();
                            id = (::match_deref::match_deref! { match &(id.clone()) {
                Deref @ Absyn::Path::IDENT { name: Deref @ "Clock" } => metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: literal!("OpenModelica"), path: metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: literal!("Internal"), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("ClockConstructor") }) }) }),
                _ => id.clone(),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&id), None)?) {
                                (__pa0, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::OVERLOAD { pathLst: __pa1 }, info: __pa2, .. }, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            cache = metamodelica::Own::own(__pa0);
                            names = metamodelica::Own::own(__pa1);
                            info = metamodelica::Own::own(__pa2);
                            env_1 = metamodelica::Own::own(__pa3);
                            (cache, res) = lookupFunctionsListInEnv(cache.clone(), env_1.clone(), &names, &info, metamodelica::nil())?;
                            Ok((cache.clone(), res.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, id, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lookupFunctionsInEnv failed on: ")); __mm_s.push_str(&*AbsynUtil::pathString(id.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outCache, outTypesTypeLst)
}

pub(crate) fn lookupFunctionsListInEnv(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIds: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut info: &SourceInfo,
    mut inAcc: metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Type>>)> {
    let mut outCache: FCore::Cache;
    let mut outTypesTypeLst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (outCache, outTypesTypeLst) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inIds, inAcc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ metamodelica::ListNode::Nil, acc) => {
                    Ok((cache.clone(), acc.clone().reverse()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: id, tail: ids }, acc) => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut cache = (*cache).clone();
                    let mut acc = (*acc).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupFunctionsInEnv(cache.clone(), env.clone(), id.clone(), info.clone())) {
                        (__pa0, __pa1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    res = metamodelica::Own::own(__pa1);
                    (cache, acc) = lookupFunctionsListInEnv(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ids), info, listAppend(res.clone(), acc.clone()))?;
                    Ok((cache.clone(), acc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, Deref @ metamodelica::ListNode::Cons { head: id, tail: _ }, _) => {
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(id.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" not found in scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env))); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![r#str.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outTypesTypeLst))
}

fn lookupFunctionsInEnv2(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut followedQual: bool,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Type>>)> {
    let mut outCache: FCore::Cache;
    let mut outTypesTypeLst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (outCache, outTypesTypeLst) = 'mc: {
        let __mc_input = (inCache, inEnv, inPath, followedQual);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, Deref @ Absyn::Path::IDENT { name: r#str }, _) => {
                    let mut httypes: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut cache = (*cache).clone();
                    ht = FNode::children(&(FNode::fromRef(r.clone())));
                    httypes = getHtTypes(r.clone());
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupFunctionsInFrame(cache.clone(), &ht, &httypes, inEnv.clone(), r#str.clone(), info)?) {
                        (__pa0, __pa1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    res = metamodelica::Own::own(__pa1);
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, id @ Deref @ Absyn::Path::IDENT { .. }, _) => {
                    let mut httypes: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut r#str: ArcStr;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut restr: SCode::Restriction;
                    let mut cache = (*cache).clone();
                    let mut r = (*r).clone();
                    let (__pa0, __pa3, __pa1, __pa2, __pa4) = ::match_deref::match_deref! { match &(lookupClass(metamodelica::AsArg::as_arg(&cache), inEnv, metamodelica::AsArg::as_arg(&id), None)?) {
                        (__pa0, __pa3 @ Deref @ SCode::Element::CLASS { name: __pa1, restriction: __pa2, .. }, __pa4) => (__pa0.clone(), __pa3.clone(), __pa1.clone(), __pa2.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    r#str = metamodelica::Own::own(__pa1);
                    restr = metamodelica::Own::own(__pa2);
                    c = metamodelica::Own::own(__pa3);
                    env_1 = metamodelica::Own::own(__pa4);
                    let true = (SCodeUtil::isFunctionRestriction(&restr)) else { return Err("pattern mismatch") };
                    let (__pa6, __pa8, __pa7) = ::match_deref::match_deref! { match &(InstFunction::implicitFunctionTypeInstantiation(cache.clone(), env_1.clone(), InnerOuter::emptyInstHierarchy().clone(), c.clone())?) {
                        (__pa6, __pa8 @ FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: _ }, .. }, _) => (__pa6.clone(), __pa8.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa6);
                    r = metamodelica::Own::own(__pa7);
                    env_2 = metamodelica::Own::own(__pa8);
                    ht = FNode::children(&(FNode::fromRef(r.clone())));
                    httypes = getHtTypes(r.clone());
                    let (__pa9, __pa10) = ::match_deref::match_deref! { match &(lookupFunctionsInFrame(cache.clone(), &ht, &httypes, env_2.clone(), r#str.clone(), info)?) {
                        (__pa9, __pa10 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => (__pa9.clone(), __pa10.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa9);
                    res = metamodelica::Own::own(__pa10);
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, Deref @ Absyn::Path::QUALIFIED { name: pack, path }, _) => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut env_1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut r#str: ArcStr;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut encflag: SCode::Encapsulated;
                    let mut restr: SCode::Restriction;
                    let mut ci_state: ClassInf::State;
                    let mut r#mod: metamodelica::Ref<DAE::Mod>;
                    let mut cache = (*cache).clone();
                    let mut r = (*r).clone();
                    let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5) = ::match_deref::match_deref! { match &(lookupClass(metamodelica::AsArg::as_arg(&cache), inEnv, &(metamodelica::Ref::new(Absyn::Path::IDENT { name: pack.clone() })), None)?) {
                        (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { name: __pa1, encapsulatedPrefix: __pa2, restriction: __pa3, .. }, __pa5) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    r#str = metamodelica::Own::own(__pa1);
                    encflag = metamodelica::Own::own(__pa2);
                    restr = metamodelica::Own::own(__pa3);
                    c = metamodelica::Own::own(__pa4);
                    env_1 = metamodelica::Own::own(__pa5);
                    r = FNode::child(FGraph::lastScopeRef(&env_1)?, r#str.clone())?;
                    if FNode::isRefInstance(r.clone()) {
                        (cache, env2) = Inst::getCachedInstance(cache.clone(), env_1.clone(), r#str.clone(), r.clone())?;
                    } else {
                        env2 = FGraph::openScope(env_1.clone(), encflag, r#str.clone(), FGraph::restrictionToScopeType(&restr))?;
                        ci_state = ClassInfUtil::start(&restr, FGraph::getGraphName(&env2)?)?;
                        r#mod = Mod::getClassModifier(&env_1, r#str.clone());
                        (cache, env2, _, _, _) = Inst::partialInstClassIn(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), r#mod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0)?;
                    }
                    (cache, res) = lookupFunctionsInEnv2(cache.clone(), &env2, path.clone(), true, info)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, id, false) => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut env: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let false = (FNode::isEncapsulated(&(FNode::fromRef(r.clone())))?) else { return Err("pattern mismatch") };
                    (env, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                    (cache, res) = lookupFunctionsInEnv2(cache.clone(), &env, id.clone(), false, info)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, id @ Deref @ Absyn::Path::IDENT { .. }, false) => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut env: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let true = (FNode::isEncapsulated(&(FNode::fromRef(r.clone())))?) else { return Err("pattern mismatch") };
                    env = FGraph::topScope(inEnv)?;
                    (cache, res) = lookupFunctionsInEnv2(cache.clone(), &env, id.clone(), true, info)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outTypesTypeLst))
}

fn createGenericBuiltinFunctions(
    mut inEnv: &FCore::Graph,
    mut inString: &ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Type>>> {
    let mut outTypesTypeLst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    outTypesTypeLst = (::match_deref::match_deref! { match &(inString.clone()) {
        Deref @ "cardinality" => list![metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("x"), ty: metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("$$") }), isExpandable: false }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_INTEGER_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("cardinality") }) }), metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("x"), ty: metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("$$") }), isExpandable: true }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_INTEGER_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("cardinality") }) })],
        _ => return Err("match: no arm matched"),
    } });
    Ok(outTypesTypeLst)
}

// - Internal functions
//   Type lookup
fn lookupTypeInEnv(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut id: &ArcStr,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Type>, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outEnv: FCore::Graph;
    (outCache, outType, outEnv) = 'mc: {
        let __mc_input = (inCache, inEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env @ FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }) => {
                    let mut c: metamodelica::Ref<DAE::Type>;
                    let mut env_1: FCore::Graph;
                    let mut httypes: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut cache = (*cache).clone();
                    ht = FNode::children(&(FNode::fromRef(r.clone())));
                    httypes = getHtTypes(r.clone());
                    (cache, c, env_1) = lookupTypeInFrame(cache.clone(), &ht, httypes.clone(), env.clone(), id.clone())?;
                    Ok((cache.clone(), c.clone(), env_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env @ FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }) => {
                    let mut c: metamodelica::Ref<DAE::Type>;
                    let mut env_1: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    (env, _) = FGraph::stripLastScopeRef(env.clone())?;
                    (cache, c, env_1) = lookupTypeInEnv(cache.clone(), env.clone(), id)?;
                    env_1 = FGraph::pushScopeRef(env_1.clone(), r.clone())?;
                    Ok((cache.clone(), c.clone(), env_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outType, outEnv))
}

fn getHtTypes(
    mut inParentRef: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
) -> metamodelica::Ref<FCore::RefTree::Tree> {
    let mut ht: metamodelica::Ref<FCore::RefTree::Tree> = metamodelica::Ref::new(FCore::RefTree::Tree::EMPTY);
    ht = 'mc: {
        let __mc_input = inParentRef.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
            let mut ht: metamodelica::Ref<FCore::RefTree::Tree> = ht.clone();
            r = FNode::child(inParentRef.clone(), arcstr::literal!(FNode::tyNodeName))?;
            ht = FNode::children(&(FNode::fromRef(r.clone())));
            Ok((ht.clone(), ht.clone()))
        })() {
            ht = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(FCore::RefTree::new())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ht
}

fn lookupTypeInFrame(
    mut inCache: FCore::Cache,
    mut inBinTree1: &metamodelica::Ref<FCore::RefTree::Tree>,
    mut inBinTree2: metamodelica::Ref<FCore::RefTree::Tree>,
    mut inEnv3: FCore::Graph,
    mut inIdent4: ArcStr,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Type>, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outEnv: FCore::Graph;
    (outCache, outType, outEnv) = (::match_deref::match_deref! { match &((inCache, inBinTree2, inEnv3, inIdent4)) {
        (cache, httypes, env, id) => {
            let mut t: metamodelica::Ref<DAE::Type>;
            let mut item: metamodelica::Ref<FCore::Node>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            item = FNode::fromRef(FCore::RefTree::get(metamodelica::AsArg::as_arg(&httypes), id.clone())?);
            (cache, t, env) = lookupTypeInFrame2(cache.clone(), &item, env.clone(), id.clone())?;
            (cache.clone(), t, env.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outType, outEnv))
}

fn lookupTypeInFrame2(
    mut inCache: FCore::Cache,
    mut item: &metamodelica::Ref<FCore::Node>,
    mut inEnv3: FCore::Graph,
    mut inIdent4: ArcStr,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Type>, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outEnv: FCore::Graph;
    (outCache, outType, outEnv) = (::match_deref::match_deref! { match item {
        Deref @ FCore::Node { data: Deref @ FCore::Data::FT { tys: Deref @ metamodelica::ListNode::Cons { head: t, tail: _ } }, .. } => {
            let mut cache = inCache;
            let mut env = inEnv3;
            (cache, t.clone(), env)
        },
        Deref @ FCore::Node { data: Deref @ FCore::Data::CO { e: comp, .. }, .. } => {
            let mut id = inIdent4;
            let mut info: SourceInfo;
            info = SCodeUtil::elementInfo(metamodelica::AsArg::as_arg(&comp));
            Error::addSourceMessage(&(Error::LOOKUP_TYPE_FOUND_COMP.clone()), list![id], &info)?;
            return Err("fail")
        },
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e: cdef @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. }, .. }, .. } => {
            let mut cache = inCache;
            let mut env = inEnv3;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut env_3: FCore::Graph;
            (cache, env_3, ty) = buildRecordType(cache, env, cdef.clone())?;
            (cache, ty, env_3)
        },
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e: cdef @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_METARECORD { .. }, .. }, .. }, .. } => {
            let mut cache = inCache;
            let mut env = inEnv3;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut env_3: FCore::Graph;
            (cache, env_3, ty) = buildMetaRecordType(cache, env, metamodelica::AsArg::as_arg(&cdef))?;
            (cache, ty, env_3)
        },
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e: cdef @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: _ }, .. }, .. }, .. } => {
            let mut cache = inCache;
            let mut env = inEnv3;
            let mut id = inIdent4;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut cenv: FCore::Graph;
            let mut env_1: FCore::Graph;
            let mut env_3: FCore::Graph;
            cenv = env;
            (cache, env_1, _) = InstFunction::implicitFunctionInstantiation(cache, cenv, InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cdef.clone(), metamodelica::nil())?;
            (cache, ty, env_3) = lookupTypeInEnv(cache, env_1, &id)?;
            (cache, ty, env_3)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outType, outEnv))
}

fn lookupFunctionsInFrame(
    mut inCache: FCore::Cache,
    mut inClasses: &metamodelica::Ref<FCore::RefTree::Tree>,
    mut inFuncTypes: &metamodelica::Ref<FCore::RefTree::Tree>,
    mut inEnv: FCore::Graph,
    mut inFuncName: ArcStr,
    mut inInfo: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Type>>)> {
    let mut outCache: FCore::Cache;
    let mut outFuncTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
    let mut data: metamodelica::Ref<FCore::Data>;
    let mut ty: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_NORETCALL);
    match '__try0: {
        r = unwrap_break_err!(FCore::RefTree::get(inFuncTypes, inFuncName.clone()), '__try0);
        let __pa1 = ::match_deref::match_deref! { match &(FNode::fromRef(r.clone())) {
            Deref @ FCore::Node { data: Deref @ FCore::Data::FT { tys: __pa1 }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        outFuncTypes = metamodelica::Own::own(__pa1);
        outCache = inCache.clone();
        Ok::<_, &'static str>((outCache.clone(), outFuncTypes.clone(), r.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            outCache = __try0_o0;
            outFuncTypes = __try0_o1;
            r = __try0_o2;
        }
        Err(_) => {
            r = FCore::RefTree::get(inClasses, inFuncName.clone())?;
            let __arc4 = FNode::fromRef(r.clone());
            let FCore::N { data: __pa3, .. } = &*__arc4;
            data = metamodelica::Own::own(__pa3);
            (outCache, outFuncTypes) = 'mc: {
                let __mc_input = &*data;
                if let Ok((__v, __wb0)) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                                _ => {
                                    let mut ty: metamodelica::Ref<DAE::Type> = ty.clone();
                                    let __arc1 = FNode::refInstVar(r.clone())?;
                                    let DAE::TYPES_VAR { ty: __pa0, .. } = &*__arc1;
                                    ty = metamodelica::Own::own(__pa0);
                                    ty = (match &*ty {
                        DAE::Type::T_FUNCTION { .. } => {
                                    assign_variant_field!(ty => DAE::Type::T_FUNCTION; path = metamodelica::Ref::new(Absyn::Path::IDENT { name: inFuncName.clone() }));
                                    ty.clone()
                        },
                        _ => return Err("match: no arm matched"),
                    });
                                    Ok(((inCache.clone(), list![ty.clone()]), ty.clone()))
                                }
                                _ => return Err("nomatch"),
                            }}
                })() {
                    ty = __wb0;
                    break 'mc __v;
                }
                if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Deref @ FCore::Data::CO { e: _, .. } => {
                            Error::addSourceMessage(&(Error::LOOKUP_TYPE_FOUND_COMP.clone()), list![inFuncName.clone()], inInfo)?;
                            Ok(return Err("fail"))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    break 'mc __v;
                }
                if let Ok((__v, __wb0)) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Deref @ FCore::Data::CL { e: cl @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. }, .. } => {
                            let mut cache: FCore::Cache;
                            let mut ty: metamodelica::Ref<DAE::Type> = ty.clone();
                            (cache, _, ty) = buildRecordType(inCache.clone(), inEnv.clone(), cl.clone())?;
                            Ok(((cache.clone(), list![ty.clone()]), ty.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    ty = __wb0;
                    break 'mc __v;
                }
                if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Deref @ FCore::Data::CL { e: cl, .. } => {
                            if !((SCodeUtil::isFunction(metamodelica::AsArg::as_arg(&cl)))) { return Err("guard") }
                            let mut tps: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                            let mut cache: FCore::Cache;
                            let mut env: FCore::Graph;
                            (cache, env, _) = InstFunction::implicitFunctionTypeInstantiation(inCache.clone(), inEnv.clone(), InnerOuter::emptyInstHierarchy().clone(), cl.clone())?;
                            (cache, tps) = lookupFunctionsInEnv2(cache.clone(), &env, metamodelica::Ref::new(Absyn::Path::IDENT { name: inFuncName.clone() }), true, inInfo)?;
                            Ok((cache.clone(), tps.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    break 'mc __v;
                }
                if let Ok((__v, __wb0)) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Deref @ FCore::Data::CL { e: cl, .. } => {
                            if !((SCodeUtil::classIsExternalObject(metamodelica::AsArg::as_arg(&cl)))) { return Err("guard") }
                            let mut cache: FCore::Cache;
                            let mut env: FCore::Graph;
                            let mut ty: metamodelica::Ref<DAE::Type> = ty.clone();
                            (cache, env, _, _, _, _, _, _, _, _) = Inst::instClass(inCache.clone(), inEnv.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cl.clone(), metamodelica::nil(), false, openmodelica_frontend_inst::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
                            (cache, ty, _) = lookupTypeInEnv(cache.clone(), env.clone(), &inFuncName)?;
                            Ok(((cache.clone(), list![ty.clone()]), ty.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    ty = __wb0;
                    break 'mc __v;
                }
                return Err("matchcontinue: no arm matched");
            };
        }
    }
    Ok((outCache, outFuncTypes))
}

pub(crate) fn selectUpdatedEnv(mut inNewEnv: FCore::Graph, mut inOldEnv: FCore::Graph) -> FCore::Graph {
    let mut outEnv: FCore::Graph;
    outEnv = if (!(FGraph::isTopScope(&inNewEnv))
        && stringEq(
            &(FGraph::getGraphNameStr(&inNewEnv)),
            &(FGraph::getGraphNameStr(&inOldEnv)),
        )) {
        inNewEnv
    } else {
        inOldEnv
    };
    outEnv
}

fn buildRecordType(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut icdef: metamodelica::Ref<SCode::Element>,
) -> Result<(FCore::Cache, FCore::Graph, metamodelica::Ref<DAE::Type>)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut ftype: metamodelica::Ref<DAE::Type>;
    let mut name: ArcStr;
    let mut cdef: metamodelica::Ref<SCode::Element>;
    (outCache, _, cdef) = buildRecordConstructorClass(cache, env.clone(), icdef)?;
    name = SCodeUtil::className(&cdef)?;
    (outCache, outEnv, _) =
        InstFunction::implicitFunctionTypeInstantiation(outCache, env, InnerOuter::emptyInstHierarchy().clone(), cdef)?;
    (outCache, ftype, _) = lookupTypeInEnv(outCache, outEnv.clone(), &name)?;
    Ok((outCache, outEnv, ftype))
}

fn buildRecordConstructorClass(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inClass: metamodelica::Ref<SCode::Element>,
) -> Result<(FCore::Cache, FCore::Graph, metamodelica::Ref<SCode::Element>)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outClass: metamodelica::Ref<SCode::Element>;
    (outCache, outEnv, outClass) = 'mc: {
        let __mc_input = (inCache, inEnv, inClass);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cl @ Deref @ SCode::Element::CLASS { name: id, info, .. }) => {
                    let mut funcelts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut reselt: metamodelica::Ref<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut cl = (*cl).clone();
                    (cache, env, funcelts, _) = buildRecordConstructorClass2(cache.clone(), env.clone(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD())?;
                    reselt = buildRecordConstructorResultElt(&funcelts, id.clone(), metamodelica::AsArg::as_arg(&env), info.clone());
                    cl = metamodelica::Ref::new(SCode::Element::CLASS { name: id.clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: openmodelica_frontend_types::SCode::FunctionRestriction::FR_RECORD_CONSTRUCTOR }, classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS { elementLst: metamodelica::cons(reselt.clone(), funcelts.clone()), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt: SCode::noComment.clone(), info: info.clone() });
                    Ok((cache.clone(), env.clone(), cl.clone()))
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
                    Debug::trace(literal!("buildRecordConstructorClass failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outClass))
}

fn buildRecordConstructorClass2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut cl: metamodelica::Ref<SCode::Element>,
    mut mods: metamodelica::Ref<DAE::Mod>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut funcelts: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut elts: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    (outCache, outEnv, funcelts, elts) = 'mc: {
        let __mc_input = (inCache, inEnv, &*cl);
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ SCode::Element::CLASS { name, info, .. }) => {
                    let mut cdefelts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut classExtendsElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut extendsElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut compElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut eltsMods: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut fpath: metamodelica::Ref<Absyn::Path>;
                    let mut env1: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut elts: metamodelica::List<metamodelica::Ref<SCode::Element>> = elts.clone();
                    let mut funcelts: metamodelica::List<metamodelica::Ref<SCode::Element>> = funcelts.clone();
                    (cache, env, _, elts, _, _, _, _, _, _) = InstExtends::instDerivedClasses(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, &cl, true, info.clone())?;
                    env = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, name.clone(), Some(openmodelica_frontend_dump::FCore::ScopeType::CLASS_SCOPE))?;
                    fpath = FGraph::getGraphName(metamodelica::AsArg::as_arg(&env))?;
                    (cdefelts, classExtendsElts, extendsElts, compElts) = InstUtil::splitElts(&elts)?;
                    (cache, env, _, _, eltsMods, _, _, _, _, _) = InstExtends::instExtendsAndClassExtendsList(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, extendsElts.clone(), &classExtendsElts, elts.clone(), &(ClassInf::State::RECORD { path: fpath.clone() }), name.clone(), true, false)?;
                    eltsMods = listAppend(eltsMods.clone(), InstUtil::addNomod(compElts.clone()));
                    (cache, env1, _) = InstUtil::addClassdefsToEnv(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, &cdefelts, false, None, false)?;
                    (cache, env1, _) = InstUtil::addComponentsToEnv(cache.clone(), env1.clone(), InnerOuter::emptyInstHierarchy().clone(), mods.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ClassInf::State::RECORD { path: fpath.clone() }, &eltsMods, true)?;
                    (cache, env1, funcelts) = buildRecordConstructorElts(cache.clone(), env1.clone(), &eltsMods, &mods)?;
                    Ok(((cache.clone(), env1.clone(), funcelts.clone(), elts.clone()), elts.clone(), funcelts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            elts = __wb0;
            funcelts = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("buildRecordConstructorClass2 failed, cl:")); __mm_s.push_str(&*SCodeDump::unparseElementStr(cl.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, funcelts, elts))
}

fn selectModifier(
    mut inModID: metamodelica::Ref<DAE::Mod>,
    mut inModNoID: metamodelica::Ref<DAE::Mod>,
) -> metamodelica::Ref<DAE::Mod> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    outMod = (match &*inModID {
        DAE::Mod::NOMOD { .. } => inModNoID,
        _ => inModID,
    });
    outMod
}

fn buildRecordConstructorElts(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inSCodeElementLst: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut mods: &metamodelica::Ref<DAE::Mod>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outSCodeElementLst: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    (outCache, outEnv, outSCodeElementLst) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inSCodeElementLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Nil) => {
                    Ok((cache.clone(), env.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: (Deref @ SCode::Element::COMPONENT { name: id, prefixes: Deref @ SCode::Prefixes { visibility: _, redeclarePrefix: redecl, finalPrefix: f @ SCode::Final::FINAL { .. }, innerOuter: io, replaceablePrefix: repl }, attributes: SCode::Attributes { arrayDims: d, connectorType: ct, parallelism: prl, variability: var, direction: _, isField: isf }, typeSpec: tp, modifications: r#mod, comment, condition: cond, info }, cmod), tail: rest }) => {
                    let mut res: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut vis: SCode::Visibility;
                    let mut dir: Absyn::Direction;
                    let mut umod: metamodelica::Ref<SCode::Mod>;
                    let mut mod_1: metamodelica::Ref<DAE::Mod>;
                    let mut compMod: metamodelica::Ref<DAE::Mod>;
                    let mut fullMod: metamodelica::Ref<DAE::Mod>;
                    let mut selectedMod: metamodelica::Ref<DAE::Mod>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut cmod = (*cmod).clone();
                    (cache, mod_1) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#mod.clone(), true, Mod::ModScope::COMPONENT { name: id.clone() }, info.clone())?;
                    mod_1 = Mod::merge(mods.clone(), mod_1.clone(), literal!(""), true)?;
                    compMod = Mod::lookupCompModification(&mod_1, id.clone())?;
                    fullMod = mod_1.clone();
                    selectedMod = selectModifier(compMod.clone(), fullMod.clone());
                    (cache, cmod) = Mod::updateMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cmod.clone(), true, metamodelica::AsArg::as_arg(&info))?;
                    selectedMod = Mod::merge(cmod.clone(), selectedMod.clone(), literal!(""), true)?;
                    umod = Mod::unelabMod(selectedMod.clone())?;
                    (cache, env, res) = buildRecordConstructorElts(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&rest), mods)?;
                    dir = openmodelica_ast::Absyn::Direction::BIDIR;
                    vis = openmodelica_frontend_types::SCode::Visibility::PROTECTED;
                    Ok((cache.clone(), env.clone(), metamodelica::cons(metamodelica::Ref::new(SCode::Element::COMPONENT { name: id.clone(), prefixes: metamodelica::Ref::new(SCode::Prefixes { visibility: vis, redeclarePrefix: redecl.clone(), finalPrefix: f.clone(), innerOuter: io.clone(), replaceablePrefix: repl.clone() }), attributes: SCode::Attributes { arrayDims: d.clone(), connectorType: ct.clone(), parallelism: prl.clone(), variability: var.clone(), direction: dir, isField: isf.clone() }, typeSpec: tp.clone(), modifications: umod.clone(), comment: comment.clone(), condition: cond.clone(), info: info.clone() }), res.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: (Deref @ SCode::Element::COMPONENT { name: id, prefixes: Deref @ SCode::Prefixes { visibility: vis, redeclarePrefix: redecl, finalPrefix: _, innerOuter: io, replaceablePrefix: repl }, attributes: SCode::Attributes { arrayDims: d, connectorType: ct, parallelism: prl, variability: SCode::Variability::CONST { .. }, direction: _, isField: isf }, typeSpec: tp, modifications: r#mod @ Deref @ SCode::Mod::NOMOD { .. }, comment, condition: cond, info }, cmod), tail: rest }) => {
                    let mut res: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut f: SCode::Final;
                    let mut var: SCode::Variability;
                    let mut dir: Absyn::Direction;
                    let mut umod: metamodelica::Ref<SCode::Mod>;
                    let mut mod_1: metamodelica::Ref<DAE::Mod>;
                    let mut compMod: metamodelica::Ref<DAE::Mod>;
                    let mut fullMod: metamodelica::Ref<DAE::Mod>;
                    let mut selectedMod: metamodelica::Ref<DAE::Mod>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut vis = (*vis).clone();
                    let mut cmod = (*cmod).clone();
                    (cache, mod_1) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#mod.clone(), true, Mod::ModScope::COMPONENT { name: id.clone() }, info.clone())?;
                    mod_1 = Mod::merge(mods.clone(), mod_1.clone(), literal!(""), true)?;
                    compMod = Mod::lookupCompModification(&mod_1, id.clone())?;
                    fullMod = mod_1.clone();
                    selectedMod = selectModifier(compMod.clone(), fullMod.clone());
                    (cache, cmod) = Mod::updateMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cmod.clone(), true, metamodelica::AsArg::as_arg(&info))?;
                    selectedMod = Mod::merge(cmod.clone(), selectedMod.clone(), literal!(""), true)?;
                    umod = Mod::unelabMod(selectedMod.clone())?;
                    (cache, env, res) = buildRecordConstructorElts(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&rest), mods)?;
                    var = openmodelica_frontend_types::SCode::Variability::VAR;
                    dir = openmodelica_ast::Absyn::Direction::INPUT;
                    vis = openmodelica_frontend_types::SCode::Visibility::PUBLIC;
                    f = openmodelica_frontend_types::SCode::Final::NOT_FINAL;
                    Ok((cache.clone(), env.clone(), metamodelica::cons(metamodelica::Ref::new(SCode::Element::COMPONENT { name: id.clone(), prefixes: metamodelica::Ref::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: redecl.clone(), finalPrefix: f, innerOuter: io.clone(), replaceablePrefix: repl.clone() }), attributes: SCode::Attributes { arrayDims: d.clone(), connectorType: ct.clone(), parallelism: prl.clone(), variability: var, direction: dir, isField: isf.clone() }, typeSpec: tp.clone(), modifications: umod.clone(), comment: comment.clone(), condition: cond.clone(), info: info.clone() }), res.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: (Deref @ SCode::Element::COMPONENT { name: id, prefixes: Deref @ SCode::Prefixes { visibility: _, redeclarePrefix: redecl, finalPrefix: f, innerOuter: io, replaceablePrefix: repl }, attributes: SCode::Attributes { arrayDims: d, connectorType: ct, parallelism: prl, variability: var @ SCode::Variability::CONST { .. }, direction: _, isField: isf }, typeSpec: tp, modifications: r#mod, comment, condition: cond, info }, cmod), tail: rest }) => {
                    let mut res: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut vis: SCode::Visibility;
                    let mut dir: Absyn::Direction;
                    let mut umod: metamodelica::Ref<SCode::Mod>;
                    let mut mod_1: metamodelica::Ref<DAE::Mod>;
                    let mut compMod: metamodelica::Ref<DAE::Mod>;
                    let mut fullMod: metamodelica::Ref<DAE::Mod>;
                    let mut selectedMod: metamodelica::Ref<DAE::Mod>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut cmod = (*cmod).clone();
                    (cache, mod_1) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#mod.clone(), true, Mod::ModScope::COMPONENT { name: id.clone() }, info.clone())?;
                    mod_1 = Mod::merge(mods.clone(), mod_1.clone(), literal!(""), true)?;
                    compMod = Mod::lookupCompModification(&mod_1, id.clone())?;
                    fullMod = mod_1.clone();
                    selectedMod = selectModifier(compMod.clone(), fullMod.clone());
                    (cache, cmod) = Mod::updateMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cmod.clone(), true, metamodelica::AsArg::as_arg(&info))?;
                    selectedMod = Mod::merge(cmod.clone(), selectedMod.clone(), literal!(""), true)?;
                    umod = Mod::unelabMod(selectedMod.clone())?;
                    (cache, env, res) = buildRecordConstructorElts(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&rest), mods)?;
                    dir = openmodelica_ast::Absyn::Direction::BIDIR;
                    vis = openmodelica_frontend_types::SCode::Visibility::PROTECTED;
                    Ok((cache.clone(), env.clone(), metamodelica::cons(metamodelica::Ref::new(SCode::Element::COMPONENT { name: id.clone(), prefixes: metamodelica::Ref::new(SCode::Prefixes { visibility: vis, redeclarePrefix: redecl.clone(), finalPrefix: f.clone(), innerOuter: io.clone(), replaceablePrefix: repl.clone() }), attributes: SCode::Attributes { arrayDims: d.clone(), connectorType: ct.clone(), parallelism: prl.clone(), variability: var.clone(), direction: dir, isField: isf.clone() }, typeSpec: tp.clone(), modifications: umod.clone(), comment: comment.clone(), condition: cond.clone(), info: info.clone() }), res.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: (Deref @ SCode::Element::COMPONENT { name: id, prefixes: Deref @ SCode::Prefixes { visibility: _, redeclarePrefix: redecl, finalPrefix: _, innerOuter: io, replaceablePrefix: repl }, attributes: SCode::Attributes { arrayDims: d, connectorType: ct, parallelism: prl, variability: _, direction: _, isField: isf }, typeSpec: tp, modifications: r#mod, comment, condition: cond, info }, cmod), tail: rest }) => {
                    let mut res: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut vis: SCode::Visibility;
                    let mut f: SCode::Final;
                    let mut var: SCode::Variability;
                    let mut dir: Absyn::Direction;
                    let mut umod: metamodelica::Ref<SCode::Mod>;
                    let mut mod_1: metamodelica::Ref<DAE::Mod>;
                    let mut compMod: metamodelica::Ref<DAE::Mod>;
                    let mut fullMod: metamodelica::Ref<DAE::Mod>;
                    let mut selectedMod: metamodelica::Ref<DAE::Mod>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut cmod = (*cmod).clone();
                    (cache, mod_1) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#mod.clone(), true, Mod::ModScope::COMPONENT { name: id.clone() }, info.clone())?;
                    mod_1 = Mod::merge(mods.clone(), mod_1.clone(), literal!(""), true)?;
                    compMod = Mod::lookupCompModification(&mod_1, id.clone())?;
                    fullMod = mod_1.clone();
                    selectedMod = selectModifier(compMod.clone(), fullMod.clone());
                    (cache, cmod) = Mod::updateMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cmod.clone(), true, metamodelica::AsArg::as_arg(&info))?;
                    selectedMod = Mod::merge(cmod.clone(), selectedMod.clone(), literal!(""), true)?;
                    umod = Mod::unelabMod(selectedMod.clone())?;
                    (cache, env, res) = buildRecordConstructorElts(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&rest), mods)?;
                    var = openmodelica_frontend_types::SCode::Variability::VAR;
                    vis = openmodelica_frontend_types::SCode::Visibility::PUBLIC;
                    f = openmodelica_frontend_types::SCode::Final::NOT_FINAL;
                    dir = openmodelica_ast::Absyn::Direction::INPUT;
                    Ok((cache.clone(), env.clone(), metamodelica::cons(metamodelica::Ref::new(SCode::Element::COMPONENT { name: id.clone(), prefixes: metamodelica::Ref::new(SCode::Prefixes { visibility: vis, redeclarePrefix: redecl.clone(), finalPrefix: f, innerOuter: io.clone(), replaceablePrefix: repl.clone() }), attributes: SCode::Attributes { arrayDims: d.clone(), connectorType: ct.clone(), parallelism: prl.clone(), variability: var, direction: dir, isField: isf.clone() }, typeSpec: tp.clone(), modifications: umod.clone(), comment: comment.clone(), condition: cond.clone(), info: info.clone() }), res.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Cons { head: (comp, cmod), tail: _ }) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Lookup.buildRecordConstructorElts failed ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(comp.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!(" with mod: ")); __mm_s.push_str(&*Mod::printModStr(metamodelica::AsArg::as_arg(&cmod))?); __mm_s.push_str(&*literal!(" and: ")); __mm_s.push_str(&*Mod::printModStr(mods)?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outSCodeElementLst))
}

fn buildRecordConstructorResultElt(
    mut elts: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut id: ArcStr,
    mut env: &FCore::Graph,
    mut info: SourceInfo,
) -> metamodelica::Ref<SCode::Element> {
    let mut outElement: metamodelica::Ref<SCode::Element>;
    outElement = metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: literal!("result"),
        prefixes: SCode::defaultPrefixes.clone(),
        attributes: SCode::Attributes {
            arrayDims: metamodelica::nil(),
            connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
            parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
            variability: openmodelica_frontend_types::SCode::Variability::VAR,
            direction: openmodelica_ast::Absyn::Direction::OUTPUT,
            isField: openmodelica_ast::Absyn::IsField::NONFIELD,
        },
        typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT { name: id }),
            arrayDim: None,
        }),
        modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
        comment: SCode::noComment.clone(),
        condition: None,
        info: info,
    });
    outElement
}

fn lookupClassInEnv(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut id: &ArcStr,
    mut inPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
    mut inState: Mutable::Mutable<bool>,
    mut inInfo: Option<SourceInfo>,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<SCode::Element>,
    FCore::Graph,
    metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
    (outCache, outClass, outEnv, outPrevFrames) = 'mc: {
        let __mc_input = (inCache, inEnv, inPrevFrames, &inInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env @ FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, prevFrames, _) => {
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut frame: metamodelica::Ref<FCore::Node>;
                    let mut cache = (*cache).clone();
                    let mut prevFrames = (*prevFrames).clone();
                    frame = FNode::fromRef(r.clone());
                    (cache, c, env_1, prevFrames) = lookupClassInFrame(cache.clone(), frame.clone(), env.clone(), id.clone(), prevFrames.clone(), inState.clone(), inInfo.clone())?;
                    Mutable::update(inState.clone(), true);
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env @ FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, prevFrames, _) => {
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut frame: metamodelica::Ref<FCore::Node>;
                    let mut sid: ArcStr;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut prevFrames = (*prevFrames).clone();
                    let false = (FNode::isRefTop(r.clone())) else { return Err("pattern mismatch") };
                    frame = FNode::fromRef(r.clone());
                    sid = FNode::refName(r.clone());
                    let true = (FNode::isEncapsulated(&frame)?) else { return Err("pattern mismatch") };
                    let true = (stringEq(&id, &sid)) else { return Err("pattern mismatch") };
                    (env, _) = FGraph::stripLastScopeRef(env.clone())?;
                    (cache, c, env, prevFrames) = lookupClassInEnv(cache.clone(), env.clone(), id, metamodelica::cons(r.clone(), prevFrames.clone()), inState.clone(), inInfo.clone())?;
                    Mutable::update(inState.clone(), true);
                    Ok((cache.clone(), c.clone(), env.clone(), prevFrames.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env @ FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, _, Some(info)) => {
                    let mut i_env: FCore::Graph;
                    let mut frame: metamodelica::Ref<FCore::Node>;
                    let mut scope: ArcStr;
                    let false = (FNode::isRefTop(r.clone())) else { return Err("pattern mismatch") };
                    frame = FNode::fromRef(r.clone());
                    let true = (FNode::isEncapsulated(&frame)?) else { return Err("pattern mismatch") };
                    i_env = FGraph::topScope(metamodelica::AsArg::as_arg(&env))?;
                    if '__try0: {
                        unwrap_break_err!(lookupClassInEnv(cache.clone(), i_env.clone(), id, metamodelica::nil(), inState.clone(), None), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    scope = FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env));
                    Error::addSourceMessage(&(Error::LOOKUP_ERROR.clone()), list![id.clone(), scope.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env @ FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, prevFrames, _) => {
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut i_env: FCore::Graph;
                    let mut frame: metamodelica::Ref<FCore::Node>;
                    let mut cache = (*cache).clone();
                    let mut prevFrames = (*prevFrames).clone();
                    frame = FNode::fromRef(r.clone());
                    let true = (FNode::isEncapsulated(&frame)?) else { return Err("pattern mismatch") };
                    i_env = FGraph::topScope(metamodelica::AsArg::as_arg(&env))?;
                    (cache, c, env_1, prevFrames) = lookupClassInEnv(cache.clone(), i_env.clone(), id, metamodelica::nil(), inState.clone(), inInfo.clone())?;
                    Mutable::update(inState.clone(), true);
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env @ FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. }, prevFrames, _) => {
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut frame: metamodelica::Ref<FCore::Node>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut prevFrames = (*prevFrames).clone();
                    let false = (FNode::isRefTop(r.clone())) else { return Err("pattern mismatch") };
                    frame = FNode::fromRef(r.clone());
                    let false = (FNode::isEncapsulated(&frame)?) else { return Err("pattern mismatch") };
                    let false = (Mutable::access(inState.clone())) else { return Err("pattern mismatch") };
                    (env, _) = FGraph::stripLastScopeRef(env.clone())?;
                    (cache, c, env_1, prevFrames) = lookupClassInEnv(cache.clone(), env.clone(), id, metamodelica::cons(r.clone(), prevFrames.clone()), inState.clone(), inInfo.clone())?;
                    Mutable::update(inState.clone(), true);
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outClass, outEnv, outPrevFrames))
}

fn lookupClassInFrame(
    mut inCache: FCore::Cache,
    mut inFrame: metamodelica::Ref<FCore::Node>,
    mut inEnv: FCore::Graph,
    mut inIdent: ArcStr,
    mut inPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
    mut inState: Mutable::Mutable<bool>,
    mut inInfo: Option<SourceInfo>,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<SCode::Element>,
    FCore::Graph,
    metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outEnv: FCore::Graph;
    let mut outPrevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
    (outCache, outClass, outEnv, outPrevFrames) = 'mc: {
        let __mc_input = (inCache, &*inFrame, inEnv, inIdent, inPrevFrames);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ FCore::Node { children: ht, .. }, totenv, name, prevFrames) => {
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                    r = FCore::RefTree::get(metamodelica::AsArg::as_arg(&ht), name.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(FNode::fromRef(r.clone())) {
                        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e: __pa0, .. }, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    c = metamodelica::Own::own(__pa0);
                    Ok((cache.clone(), c.clone(), totenv.clone(), prevFrames.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, totenv, name, _) => {
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut prevFrames: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                    let mut qimports: metamodelica::List<Absyn::Import>;
                    let mut uqimports: metamodelica::List<Absyn::Import>;
                    let mut unique: bool;
                    let mut cache = (*cache).clone();
                    (qimports, uqimports) = FNode::imports(inFrame.clone())?;
                    match '__try0: {
                        let false = ((qimports).is_empty()) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        (cache, c, env_1, prevFrames) = unwrap_break_err!(lookupQualifiedImportedClassInFrame(cache.clone(), &qimports, totenv.clone(), name.clone(), inState.clone(), inInfo.clone()), '__try0);
                        Ok::<_, &'static str>((c.clone(), cache.clone(), env_1.clone(), prevFrames.clone()))
                    } {
                        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
                            c = __try0_o0;
                            cache = __try0_o1;
                            env_1 = __try0_o2;
                            prevFrames = __try0_o3;
                        }
                        Err(_) => {
                            let false = ((uqimports).is_empty()) else { return Err("pattern mismatch") };
                            (cache, c, env_1, prevFrames, unique) = lookupUnqualifiedImportedClassInFrame(cache.clone(), &uqimports, totenv.clone(), name.clone(), inInfo.clone())?;
                            Mutable::update(inState.clone(), true);
                            reportSeveralNamesError(unique, name.clone())?;
                        }
                    }
                    Ok((cache.clone(), c.clone(), env_1.clone(), prevFrames.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outClass, outEnv, outPrevFrames))
}

fn reportSeveralNamesError(mut unique: bool, mut name: ArcStr) -> Result<()> {
    let () = (match unique {
        true => (),
        false => {
            Error::addMessage(Error::IMPORT_SEVERAL_NAMES.clone(), list![name])?;
            ()
        }
    });
    Ok(())
}

fn lookupVar2(
    mut inBinTree: &metamodelica::Ref<FCore::RefTree::Tree>,
    mut inIdent: ArcStr,
    mut inGraph: FCore::Graph,
) -> Result<(
    metamodelica::Ref<DAE::Var>,
    metamodelica::Ref<SCode::Element>,
    metamodelica::Ref<DAE::Mod>,
    FCore::Status,
    FCore::Graph,
)> {
    let mut outVar: metamodelica::Ref<DAE::Var>;
    let mut outElement: metamodelica::Ref<SCode::Element>;
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    let mut instStatus: FCore::Status;
    let mut outEnv: FCore::Graph;
    let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
    let mut s: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
    let mut n: metamodelica::Ref<FCore::Node>;
    let mut name: ArcStr;
    r = FCore::RefTree::get(inBinTree, inIdent.clone())?;
    outVar = FNode::refInstVar(r.clone())?;
    s = FNode::refRefTargetScope(r.clone())?;
    n = FNode::fromRef(r);
    if !(FNode::isComponent(&n)) && Flags::isSet(Flags::LOOKUP.clone())? {
        let false = (Config::acceptMetaModelicaGrammar()?) else {
            return Err("pattern mismatch");
        };
        let __pa0 = ::match_deref::match_deref! { match &(n.clone()) {
            Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e: Deref @ SCode::Element::CLASS { name: __pa0, .. }, .. }, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        name = metamodelica::Own::own(__pa0);
        name = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*inIdent);
            __mm_s.push_str(&*literal!(" = "));
            __mm_s.push_str(&*FGraph::printGraphPathStr(&inGraph));
            __mm_s.push_str(&*literal!("."));
            __mm_s.push_str(&*name);
            ArcStr::from(__mm_s)
        };
        Debug::traceln({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "- Lookup.lookupVar2 failed because we found a class instead of a variable: "
            ));
            __mm_s.push_str(&*name);
            ArcStr::from(__mm_s)
        })?;
        return Err("fail");
    }
    let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(n) {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CO { e: __pa2, r#mod: __pa3, kind: _, status: __pa4 }, .. } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outElement = metamodelica::Own::own(__pa2);
    outMod = metamodelica::Own::own(__pa3);
    instStatus = metamodelica::Own::own(__pa4);
    outEnv = FGraph::setScope(inGraph, s)?;
    Ok((outVar, outElement, outMod, instStatus, outEnv))
}

fn checkSubscripts(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inExpSubscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (inType.clone(), inExpSubscriptLst);
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
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: ys }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![dim.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e @ Deref @ DAE::Exp::RANGE { .. } }, tail: ys }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut dim_int: i32;
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    dim_int = Expression::rangeSize(metamodelica::AsArg::as_arg(&e))?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim_int })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ARRAY { array: se, .. } }, tail: ys }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut dim_int: i32;
                    Expression::dimensionSize(metamodelica::AsArg::as_arg(&dim))?;
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    dim_int = ((se).len() as i32);
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim_int })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e }, tail: ys }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut dim: metamodelica::Ref<DAE::Dimension>;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::r#typeof(e.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    dim = metamodelica::Own::own(__pa0);
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![dim.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: ind } }, tail: ys }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut sz: i32;
                    sz = Expression::dimensionSize(metamodelica::AsArg::as_arg(&dim))?;
                    let true = (ind.clone() > 0) else { return Err("pattern mismatch") };
                    let true = (ind.clone() <= sz) else { return Err("pattern mismatch") };
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    Ok(t_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { .. }, tail: ys }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let true = (Expression::dimensionKnown(metamodelica::AsArg::as_arg(&dim))) else { return Err("pattern mismatch") };
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    Ok(t_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { .. }, tail: ys }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    Ok(t_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_EXP { .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { .. }, tail: ys }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    Ok(t_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: ys }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    t_1 = checkSubscripts(t.clone(), ys.clone())?;
                    Ok(t_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. }, ys) => {
                    Ok(checkSubscripts(t.clone(), ys.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t @ Deref @ DAE::Type::T_UNKNOWN { .. }, _) => {
                    Ok(t.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAARRAY { .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(var_field!((*inType).ty, DAE::Type::T_METAARRAY).clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METAARRAY { .. }, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(inType.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, s) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- Lookup.checkSubscripts failed (tp: "))?;
                    Debug::trace(TypesDump::printTypeStr(t.clone()))?;
                    Debug::trace(literal!(" subs:"))?;
                    Debug::trace(stringDelimitList(List::map(s.clone(), &move |__a0: metamodelica::Ref<DAE::Subscript>| ExpressionBasics::printSubscriptStr(&__a0))?, literal!(",")))?;
                    Debug::trace(literal!(")\n"))?;
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

fn lookupVarF(
    mut inCache: FCore::Cache,
    mut inBinTree: metamodelica::Ref<FCore::RefTree::Tree>,
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut inEnv: FCore::Graph,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Attributes>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Binding>,
    Option<DAE::Const>,
    InstTypes::SplicedExpData,
    FCore::Graph,
    ArcStr,
)> {
    let mut outCache: FCore::Cache;
    let mut outAttributes: metamodelica::Ref<DAE::Attributes>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    let mut constOfForIteratorRange: Option<DAE::Const>;
    let mut splicedExpData: InstTypes::SplicedExpData =
        <InstTypes::SplicedExpData as ::std::default::Default>::default();
    let mut outComponentEnv: FCore::Graph;
    let mut name: ArcStr;
    (
        outCache,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        splicedExpData,
        outComponentEnv,
        name,
    ) = (match &**inComponentRef {
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            subscriptLst: ss,
            ..
        } => {
            (
                outCache,
                outAttributes,
                outType,
                outBinding,
                constOfForIteratorRange,
                splicedExpData,
                outComponentEnv,
                name,
            ) = lookupVarFIdent(inCache, &inBinTree, id.clone(), ss.clone(), inEnv)?;
            (
                outCache,
                outAttributes,
                outType,
                outBinding,
                constOfForIteratorRange,
                splicedExpData,
                outComponentEnv,
                name,
            )
        }
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            subscriptLst: ss,
            componentRef: ids,
            ..
        } => {
            let mut cache = inCache.clone();
            let mut ht = inBinTree.clone();
            let mut ct: metamodelica::Ref<DAE::ConnectorType>;
            let mut prl: SCode::Parallelism;
            let mut vt: SCode::Variability;
            let mut vt2: SCode::Variability;
            let mut di: Absyn::Direction;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut idTp: metamodelica::Ref<DAE::Type>;
            let mut ty2_2: metamodelica::Ref<DAE::Type>;
            let mut tyParent: metamodelica::Ref<DAE::Type>;
            let mut tyChild: metamodelica::Ref<DAE::Type>;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut binding: metamodelica::Ref<DAE::Binding>;
            let mut parentBinding: metamodelica::Ref<DAE::Binding>;
            let mut componentEnv: FCore::Graph;
            let mut io: Absyn::InnerOuter;
            let mut texp: Option<metamodelica::Ref<DAE::Exp>>;
            let mut xCref: metamodelica::Ref<DAE::ComponentRef>;
            let mut tCref: metamodelica::Ref<DAE::ComponentRef>;
            let mut ltCref: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut splicedExp: metamodelica::Ref<DAE::Exp>;
            let mut eType: metamodelica::Ref<DAE::Type>;
            let mut cnstForRange: Option<DAE::Const>;
            let mut vis: SCode::Visibility;
            let mut attr: metamodelica::Ref<DAE::Attributes>;
            let mut oSplicedExp: Option<metamodelica::Ref<DAE::Exp>>;
            let mut ss = (*ss).clone();
            let (__t4, _, _, _, __pa3) = lookupVar2(&ht, id.clone(), inEnv)?;
            let __arc6 = __t4.clone();
            let DAE::TYPES_VAR {
                name: _,
                attributes: __t5,
                ty: __pa1,
                binding: __pa2,
                bind_from_outside: _,
                ..
            } = &*__arc6;
            let __arc7 = __t5.clone();
            let DAE::ATTR { variability: __pa0, .. } = &*__arc7;
            vt2 = metamodelica::Own::own(__pa0);
            tyParent = metamodelica::Own::own(__pa1);
            parentBinding = metamodelica::Own::own(__pa2);
            componentEnv = metamodelica::Own::own(__pa3);
            (attr, ty, binding, cnstForRange, componentEnv, name) = (match &*tyParent {
                DAE::Type::T_METAARRAY { .. } => {
                    let true = (((TypesDump::getDimensions(&tyParent)).len() as i32) == ((ss).len() as i32)) else {
                        return Err("pattern mismatch");
                    };
                    (cache, attr, ty, binding, cnstForRange, name) =
                        lookupVarFMetaModelica(cache, &componentEnv, ids, &(Types::metaArrayElementType(&tyParent)?))?;
                    splicedExpData = InstTypes::SplicedExpData {
                        splicedExp: None,
                        identType: ty.clone(),
                    };
                    (attr, ty, binding, cnstForRange, componentEnv, name)
                }
                _ if (Types::isBoxedType(&tyParent) && !(Types::isUnknownType(&tyParent))) => {
                    ::match_deref::match_deref! { match &(ss.clone()) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    (cache, attr, ty, binding, cnstForRange, name) =
                        lookupVarFMetaModelica(cache, &componentEnv, ids, &tyParent)?;
                    splicedExpData = InstTypes::SplicedExpData {
                        splicedExp: None,
                        identType: ty.clone(),
                    };
                    (attr, ty, binding, cnstForRange, componentEnv, name)
                }
                _ => {
                    let (
                        __pa0,
                        __t14,
                        __pa7,
                        __pa8,
                        __pa9,
                        InstTypes::SPLICEDEXPDATA {
                            splicedExp: __pa10,
                            identType: __pa11,
                        },
                        _,
                        __pa12,
                        __pa13,
                    ) = lookupVar(cache, componentEnv, ids.clone())?;
                    let __arc15 = __t14.clone();
                    let DAE::ATTR {
                        connectorType: __pa1,
                        parallelism: __pa2,
                        variability: __pa3,
                        direction: __pa4,
                        innerOuter: __pa5,
                        visibility: __pa6,
                    } = &*__arc15;
                    cache = metamodelica::Own::own(__pa0);
                    ct = metamodelica::Own::own(__pa1);
                    prl = metamodelica::Own::own(__pa2);
                    vt = metamodelica::Own::own(__pa3);
                    di = metamodelica::Own::own(__pa4);
                    io = metamodelica::Own::own(__pa5);
                    vis = metamodelica::Own::own(__pa6);
                    tyChild = metamodelica::Own::own(__pa7);
                    binding = metamodelica::Own::own(__pa8);
                    cnstForRange = metamodelica::Own::own(__pa9);
                    texp = metamodelica::Own::own(__pa10);
                    idTp = metamodelica::Own::own(__pa11);
                    componentEnv = metamodelica::Own::own(__pa12);
                    name = metamodelica::Own::own(__pa13);
                    ltCref = elabComponentRecursive(texp);
                    ty = tyChild.clone();
                    oSplicedExp = (::match_deref::match_deref! { match &(ltCref) {
                        Deref @ metamodelica::ListNode::Cons { head: __esc_tCref, tail: _ } => {
                            tCref = (*__esc_tCref).clone();
                            ty1 = checkSubscripts(tyParent.clone(), ss.clone())?;
                            ty = sliceDimensionType(ty1, tyChild)?;
                            ty2_2 = Types::simplifyType(tyParent.clone())?;
                            ss = addArrayDimensions(&ty2_2, ss.clone());
                            xCref = ComponentReferenceBasics::makeCrefQual(id.clone(), ty2_2, ss.clone(), tCref.clone());
                            eType = Types::simplifyType(ty.clone())?;
                            splicedExp = Expression::makeCrefExp(xCref, eType)?;
                            Some(splicedExp)
                        },
                        Deref @ metamodelica::ListNode::Nil => None,
                        _ => unreachable!("match_deref! exhaustiveness placeholder"),
                    } });
                    vt = SCodeUtil::variabilityOr(vt, vt2);
                    binding = lookupBinding(inComponentRef, &tyParent, &ty, &parentBinding, binding);
                    splicedExpData = InstTypes::SplicedExpData {
                        splicedExp: oSplicedExp,
                        identType: idTp,
                    };
                    (
                        metamodelica::Ref::new(DAE::Attributes {
                            connectorType: ct,
                            parallelism: prl,
                            variability: vt,
                            direction: di,
                            innerOuter: io,
                            visibility: vis,
                        }),
                        ty,
                        binding,
                        cnstForRange,
                        componentEnv,
                        name,
                    )
                }
            });
            (
                cache,
                attr,
                ty,
                binding,
                cnstForRange,
                splicedExpData,
                componentEnv,
                name,
            )
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((
        outCache,
        outAttributes,
        outType,
        outBinding,
        constOfForIteratorRange,
        splicedExpData,
        outComponentEnv,
        name,
    ))
}

fn lookupVarFIdent(
    mut cache: FCore::Cache,
    mut ht: &metamodelica::Ref<FCore::RefTree::Tree>,
    mut ident: ArcStr,
    mut ss: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inEnv: FCore::Graph,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Attributes>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Binding>,
    Option<DAE::Const>,
    InstTypes::SplicedExpData,
    FCore::Graph,
    ArcStr,
)> {
    let mut cache: FCore::Cache = cache;
    let mut attr: metamodelica::Ref<DAE::Attributes>;
    let mut ty_1: metamodelica::Ref<DAE::Type>;
    let mut bind: metamodelica::Ref<DAE::Binding>;
    let mut cnstForRange: Option<DAE::Const>;
    let mut splicedExpData: InstTypes::SplicedExpData;
    let mut componentEnv: FCore::Graph;
    let mut name: ArcStr;
    let mut tty: metamodelica::Ref<DAE::Type>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut ss_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let (__t6, _, _, _, __pa5) = lookupVar2(ht, ident.clone(), inEnv)?;
    let __arc7 = __t6.clone();
    let DAE::TYPES_VAR {
        name: __pa0,
        attributes: __pa1,
        ty: __pa2,
        binding: __pa3,
        bind_from_outside: _,
        constOfForIteratorRange: __pa4,
    } = &*__arc7;
    name = metamodelica::Own::own(__pa0);
    attr = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    bind = metamodelica::Own::own(__pa3);
    cnstForRange = metamodelica::Own::own(__pa4);
    componentEnv = metamodelica::Own::own(__pa5);
    ty_1 = checkSubscripts(ty.clone(), ss.clone())?;
    tty = Types::simplifyType(ty.clone())?;
    ss_1 = addArrayDimensions(&tty, ss);
    splicedExpData = InstTypes::SplicedExpData {
        splicedExp: Some(Expression::makeCrefExp(
            ComponentReferenceBasics::makeCrefIdent(ident, tty.clone(), ss_1),
            tty,
        )?),
        identType: ty,
    };
    Ok((
        cache,
        attr,
        ty_1,
        bind,
        cnstForRange,
        splicedExpData,
        componentEnv,
        name,
    ))
}

fn lookupVarFMetaModelica(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Attributes>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Binding>,
    Option<DAE::Const>,
    ArcStr,
)> {
    let mut cache: FCore::Cache = inCache;
    let mut attr: metamodelica::Ref<DAE::Attributes>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut binding: metamodelica::Ref<DAE::Binding>;
    let mut cnstForRange: Option<DAE::Const>;
    let mut name: ArcStr;
    (attr, ty, binding, cnstForRange, name) = (match &**cr {
        DAE::ComponentRef::CREF_IDENT {
            ident: __cr_ident,
            subscriptLst: __cr_subscriptLst,
            ..
        } => {
            let mut fields: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            fields = Types::getMetaRecordFields(inType.clone())?;
            let __arc5 = (fields.clone()).get(Types::findVarIndex(__cr_ident.clone(), &fields)? + 1)?;
            let DAE::TYPES_VAR {
                name: __pa0,
                attributes: __pa1,
                ty: __pa2,
                binding: __pa3,
                bind_from_outside: _,
                constOfForIteratorRange: __pa4,
            } = &*__arc5;
            name = metamodelica::Own::own(__pa0);
            attr = metamodelica::Own::own(__pa1);
            ty = metamodelica::Own::own(__pa2);
            binding = metamodelica::Own::own(__pa3);
            cnstForRange = metamodelica::Own::own(__pa4);
            for mut s in &*__cr_subscriptLst.clone() {
                ty = (match &*ty {
                    DAE::Type::T_METAARRAY { ty: __ty_ty } => __ty_ty.clone(),
                    _ => return Err("match: no arm matched"),
                });
            }
            ty = Types::getMetaRecordIfSingleton(ty)?;
            (attr, ty, binding, cnstForRange, name)
        }
        DAE::ComponentRef::CREF_QUAL {
            componentRef: __cr_componentRef,
            ident: __cr_ident,
            subscriptLst: __cr_subscriptLst,
            ..
        } => {
            let mut fields: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            fields = Types::getMetaRecordFields(inType.clone())?;
            let __arc5 = (fields.clone()).get(Types::findVarIndex(__cr_ident.clone(), &fields)? + 1)?;
            let DAE::TYPES_VAR {
                name: __pa0,
                attributes: __pa1,
                ty: __pa2,
                binding: __pa3,
                bind_from_outside: _,
                constOfForIteratorRange: __pa4,
            } = &*__arc5;
            name = metamodelica::Own::own(__pa0);
            attr = metamodelica::Own::own(__pa1);
            ty = metamodelica::Own::own(__pa2);
            binding = metamodelica::Own::own(__pa3);
            cnstForRange = metamodelica::Own::own(__pa4);
            for mut s in &*__cr_subscriptLst.clone() {
                ty = (match &*ty {
                    DAE::Type::T_METAARRAY { ty: __ty_ty } => __ty_ty.clone(),
                    _ => return Err("match: no arm matched"),
                });
            }
            ty = Types::getMetaRecordIfSingleton(ty)?;
            (cache, attr, ty, binding, cnstForRange, name) =
                lookupVarFMetaModelica(cache, inEnv, metamodelica::AsArg::as_arg(&__cr_componentRef), &ty)?;
            (attr, ty, binding, cnstForRange, name)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((cache, attr, ty, binding, cnstForRange, name))
}

fn lookupBinding(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inParentType: &metamodelica::Ref<DAE::Type>,
    mut inChildType: &metamodelica::Ref<DAE::Type>,
    mut inParentBinding: &metamodelica::Ref<DAE::Binding>,
    mut inChildBinding: metamodelica::Ref<DAE::Binding>,
) -> metamodelica::Ref<DAE::Binding> {
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    outBinding = 'mc: {
        let __mc_input = (&**inCref, &**inParentBinding);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, identType: _, subscriptLst: ss, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: cId, identType: _, subscriptLst: Deref @ metamodelica::ListNode::Nil } }, Deref @ DAE::Binding::EQBOUND { exp: e, evaluatedExp: _, constant_: c, source: s }) => {
                    let mut tyElement: metamodelica::Ref<DAE::Type>;
                    let mut b: metamodelica::Ref<DAE::Binding>;
                    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut comp: metamodelica::List<ArcStr>;
                    let mut e = (*e).clone();
                    let true = (Types::isArray(inParentType)) else { return Err("pattern mismatch") };
                    tyElement = Types::arrayElementType(inParentType);
                    let true = (Types::isRecord(&tyElement)) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Expression::applyExpSubscripts(e.clone(), ss.clone())?) {
                        Deref @ DAE::Exp::RECORD { path: _, exps: __pa0, comp: __pa1, ty: _ } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    exps = metamodelica::Own::own(__pa0);
                    comp = metamodelica::Own::own(__pa1);
                    e = (exps).get(List::position(cId.clone(), &comp)?)?;
                    b = metamodelica::Ref::new(DAE::Binding::EQBOUND { exp: e.clone(), evaluatedExp: None, constant_: c.clone(), source: s.clone() });
                    Ok(b.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, identType: _, subscriptLst: ss, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: cId, identType: _, subscriptLst: Deref @ metamodelica::ListNode::Nil } }, Deref @ DAE::Binding::VALBOUND { valBound: v, source: s }) => {
                    let mut tyElement: metamodelica::Ref<DAE::Type>;
                    let mut b: metamodelica::Ref<DAE::Binding>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut comp: metamodelica::List<ArcStr>;
                    let true = (Types::isArray(inParentType)) else { return Err("pattern mismatch") };
                    tyElement = Types::arrayElementType(inParentType);
                    let true = (Types::isRecord(&tyElement)) else { return Err("pattern mismatch") };
                    e = ValuesUtil::valueExp(v.clone(), None)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Expression::applyExpSubscripts(e.clone(), ss.clone())?) {
                        Deref @ DAE::Exp::RECORD { path: _, exps: __pa0, comp: __pa1, ty: _ } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    exps = metamodelica::Own::own(__pa0);
                    comp = metamodelica::Own::own(__pa1);
                    e = (exps).get(List::position(cId.clone(), &comp)?)?;
                    b = metamodelica::Ref::new(DAE::Binding::EQBOUND { exp: e.clone(), evaluatedExp: None, constant_: openmodelica_frontend_types::DAE::Const::C_CONST, source: s.clone() });
                    Ok(b.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inChildBinding.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outBinding
}

fn elabComponentRecursive(
    mut oCref: Option<metamodelica::Ref<DAE::Exp>>,
) -> metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> {
    let mut lref: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    lref = (::match_deref::match_deref! { match &(oCref) {
        Some(Deref @ DAE::Exp::CREF { componentRef: ecpr @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ }, ty: _ }) => {
            metamodelica::cons(ecpr.clone(), metamodelica::nil())
        },
        Some(Deref @ DAE::Exp::CREF { componentRef: ecpr @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, identType: _, subscriptLst: _, componentRef: _ }, ty: _ }) => {
            metamodelica::cons(ecpr.clone(), metamodelica::nil())
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    lref
}

fn addArrayDimensions(
    mut tySub: &metamodelica::Ref<DAE::Type>,
    mut ss: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Subscript>> {
    let mut outType: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    outType = 'mc: {
        let __mc_input = &*ss;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let true = (Types::isArray(tySub)) else { return Err("pattern mismatch") };
                    dims = TypesDump::getDimensions(tySub);
                    subs = List::map(dims.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| makeDimensionSubscript(&__a0))?;
                    subs = expandWholeDimSubScript(&ss, subs.clone())?;
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
                    Ok(ss.clone())
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

fn makeDimensionSubscript(mut inDim: &metamodelica::Ref<DAE::Dimension>) -> Result<metamodelica::Ref<DAE::Subscript>> {
    let mut outSub: metamodelica::Ref<DAE::Subscript>;
    outSub = (match &**inDim {
        DAE::Dimension::DIM_INTEGER { integer: 0 } => metamodelica::Ref::new(DAE::Subscript::SLICE {
            exp: metamodelica::Ref::new(DAE::Exp::ARRAY {
                ty: DAE::T_INTEGER_DEFAULT().clone(),
                scalar: true,
                array: list![metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 })],
            }),
        }),
        DAE::Dimension::DIM_INTEGER {
            integer: __inDim_integer,
        } => metamodelica::Ref::new(DAE::Subscript::SLICE {
            exp: metamodelica::Ref::new(DAE::Exp::RANGE {
                ty: metamodelica::Ref::new(DAE::Type::T_ARRAY {
                    ty: DAE::T_INTEGER_DEFAULT().clone(),
                    dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER {
                        integer: __inDim_integer.clone()
                    })],
                }),
                start: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }),
                step: None,
                stop: metamodelica::Ref::new(DAE::Exp::ICONST {
                    integer: __inDim_integer.clone(),
                }),
            }),
        }),
        DAE::Dimension::DIM_BOOLEAN { .. } => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            expl = list![
                metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
                metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })
            ];
            metamodelica::Ref::new(DAE::Subscript::SLICE {
                exp: metamodelica::Ref::new(DAE::Exp::ARRAY {
                    ty: DAE::T_BOOL_DEFAULT().clone(),
                    scalar: true,
                    array: expl,
                }),
            })
        }
        DAE::Dimension::DIM_ENUM {
            enumTypeName: enum_name,
            literals: l,
            ..
        } => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            expl = makeEnumLiteralIndices(enum_name, l, 1)?;
            metamodelica::Ref::new(DAE::Subscript::SLICE {
                exp: metamodelica::Ref::new(DAE::Exp::ARRAY {
                    ty: metamodelica::Ref::new(DAE::Type::T_ENUMERATION {
                        index: None,
                        path: enum_name.clone(),
                        names: l.clone(),
                        literalVarLst: metamodelica::nil(),
                        attributeLst: metamodelica::nil(),
                    }),
                    scalar: true,
                    array: expl,
                }),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outSub)
}

fn makeEnumLiteralIndices(
    mut enumTypeName: &metamodelica::Ref<Absyn::Path>,
    mut enumLiterals: &metamodelica::List<ArcStr>,
    mut enumIndex: i32,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut enumIndices: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    enumIndices = (::match_deref::match_deref! { match enumLiterals {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: l, tail: ls } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut enum_type_name: metamodelica::Ref<Absyn::Path>;
            enum_type_name = AbsynUtil::joinPaths(enumTypeName.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: l.clone() }))?;
            e = metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL { name: enum_type_name, index: enumIndex });
            expl = makeEnumLiteralIndices(enumTypeName, ls, enumIndex + 1)?;
            metamodelica::cons(e, expl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(enumIndices)
}

fn expandWholeDimSubScript(
    mut inSubs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inSlice: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    let mut outSubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    outSubs = 'mc: {
        let __mc_input = (&**inSubs, inSlice);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: sub1 @ Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::CREF { .. } }, tail: subs1 }, subs2) => {
                    let mut subs2 = (*subs2).clone();
                    subs2 = expandWholeDimSubScript(metamodelica::AsArg::as_arg(&subs1), subs2.clone())?;
                    Ok(metamodelica::cons(sub1.clone(), subs2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
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
                (Deref @ metamodelica::ListNode::Nil, subs2) => {
                    Ok(subs2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: subs1 }, Deref @ metamodelica::ListNode::Cons { head: sub2, tail: subs2 }) => {
                    let mut subs2 = (*subs2).clone();
                    subs2 = expandWholeDimSubScript(metamodelica::AsArg::as_arg(&subs1), subs2.clone())?;
                    Ok(metamodelica::cons(sub2.clone(), subs2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: sub1, tail: subs1 }, Deref @ metamodelica::ListNode::Cons { head: _, tail: subs2 }) => {
                    let mut subs2 = (*subs2).clone();
                    subs2 = expandWholeDimSubScript(metamodelica::AsArg::as_arg(&subs1), subs2.clone())?;
                    Ok(metamodelica::cons(sub1.clone(), subs2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outSubs)
}

fn sliceDimensionType(
    mut inTypeD: metamodelica::Ref<DAE::Type>,
    mut inTypeL: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match &((inTypeD, inTypeL)) {
        (t, tOrg) => {
            let mut dimensions: metamodelica::List<i32>;
            let mut dim2: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut t = (*t).clone();
            dimensions = Types::getDimensionSizes(metamodelica::AsArg::as_arg(&t))?;
            dim2 = List::map(dimensions, &fnptr!(Expression::intDimension, i32))?;
            dim2 = dim2.reverse();
            t = List::foldr(&dim2, &fnptr!(Types::liftArray, metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Dimension>), tOrg.clone())?;
            t.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

pub(crate) fn buildMetaRecordType(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut cdef: &metamodelica::Ref<SCode::Element>,
) -> Result<(FCore::Cache, FCore::Graph, metamodelica::Ref<DAE::Type>)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut ftype: metamodelica::Ref<DAE::Type>;
    let mut id: ArcStr;
    let mut env: FCore::Graph;
    let mut utPath: metamodelica::Ref<Absyn::Path>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut index: i32;
    let mut varlst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut singleton: bool;
    let mut cache: FCore::Cache;
    let mut typeVarsType: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut typeVars: metamodelica::List<ArcStr>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &((*cdef)) {
        Deref @ SCode::Element::CLASS { name: __pa0, restriction: SCode::Restriction::R_METARECORD { name: __pa1, index: __pa2, singleton: __pa3, typeVars: __pa4, .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa5, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    id = metamodelica::Own::own(__pa0);
    utPath = metamodelica::Own::own(__pa1);
    index = metamodelica::Own::own(__pa2);
    singleton = metamodelica::Own::own(__pa3);
    typeVars = metamodelica::Own::own(__pa4);
    els = metamodelica::Own::own(__pa5);
    env = FGraph::openScope(
        inEnv,
        openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        id.clone(),
        Some(openmodelica_frontend_dump::FCore::ScopeType::CLASS_SCOPE),
    )?;
    (cache, utPath) = Inst::makeFullyQualified(inCache, env.clone(), utPath)?;
    path = AbsynUtil::joinPaths(utPath.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: id }))?;
    (outCache, outEnv, _, _, _, _, _, varlst, _, _) = Inst::instElementList(
        cache,
        env,
        InnerOuter::emptyInstHierarchy().clone(),
        UnitAbsyn::noStore().clone(),
        openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
        openmodelica_frontend_types::DAE::Prefix::NOPRE,
        ClassInf::State::META_RECORD {
            path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
        },
        List::map1(
            els,
            &fnptr!(Util::makeTuple, _, _),
            openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
        )?,
        metamodelica::nil(),
        false,
        openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL,
        ConnectionGraph::EMPTY().clone(),
        Connect::emptySet().clone(),
        true,
    )?;
    varlst = Types::boxVarLst(&varlst)?;
    typeVarsType = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
        for mut tv in (typeVars).into_iter().cloned() {
            let __x = metamodelica::Ref::new(DAE::Type::T_METAPOLYMORPHIC { name: tv.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    ftype = metamodelica::Ref::new(DAE::Type::T_METARECORD {
        path: path,
        utPath: utPath,
        typeVars: typeVarsType,
        index: index,
        fields: varlst,
        knownSingleton: singleton,
    });
    Ok((outCache, outEnv, ftype))
}

pub(crate) fn isIterator(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> (Option<bool>, FCore::Cache) {
    let mut outIsIterator: Option<bool>;
    let mut outCache: FCore::Cache;
    (outIsIterator, outCache) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: _ }, .. }) => {
                    let mut id: ArcStr;
                    let mut ht: metamodelica::Ref<FCore::RefTree::Tree>;
                    let mut ic: Option<DAE::Const>;
                    let mut b: bool;
                    ht = FNode::children(&(FNode::fromRef(r#ref.clone())));
                    id = ComponentReferenceBasics::crefFirstIdent(inCref)?;
                    let (__t1, _, _, _, _) = lookupVar2(&ht, id.clone(), inEnv.clone())?;
                    let __arc2 = __t1.clone();
                    let DAE::TYPES_VAR { constOfForIteratorRange: __pa0, .. } = &*__arc2;
                    ic = metamodelica::Own::own(__pa0);
                    b = (ic).is_some();
                    Ok((Some(b), cache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, FCore::Graph::G { scope: Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: _ }, .. }) => {
                    let mut env: FCore::Graph;
                    let mut res: Option<bool>;
                    let mut cache = (*cache).clone();
                    let true = (frameIsImplAddedScope(&(FNode::fromRef(r#ref.clone())))) else { return Err("pattern mismatch") };
                    (env, _) = FGraph::stripLastScopeRef(inEnv.clone())?;
                    (res, cache) = isIterator(cache.clone(), &env, inCref);
                    Ok((res.clone(), cache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((None, inCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outIsIterator, outCache)
}

pub(crate) fn isFunctionCallViaComponent(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPath: &metamodelica::Ref<Absyn::Path>,
) -> bool {
    let mut yes: bool;
    yes = 'mc: {
        let __mc_input = &**inPath;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::QUALIFIED { name, path: _ } => {
                    ErrorExt::setCheckpoint(literal!("functionViaComponentRef10"));
                    lookupVarIdent(inCache.clone(), inEnv.clone(), name.clone(), metamodelica::nil())?;
                    ErrorExt::rollBack(literal!("functionViaComponentRef10"));
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::QUALIFIED { name: _, path: _ } => {
                    ErrorExt::rollBack(literal!("functionViaComponentRef10"));
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
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    yes
}

fn prefixSplicedExp(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inSplicedExp: InstTypes::SplicedExpData,
) -> Result<InstTypes::SplicedExpData> {
    let mut outSplicedExp: InstTypes::SplicedExpData;
    outSplicedExp = (::match_deref::match_deref! { match &(inSplicedExp.clone()) {
        InstTypes::SplicedExpData { splicedExp: Some(Deref @ DAE::Exp::CREF { componentRef: cref, ty: ety }), identType: ty } => {
            let mut cref = (*cref).clone();
            cref = ComponentReference::joinCrefs(inCref, cref.clone())?;
            InstTypes::SplicedExpData { splicedExp: Some(metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ety.clone() })), identType: ty.clone() }
        },
        _ => {
            inSplicedExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSplicedExp)
}

pub(crate) fn isArrayType(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inPath: &metamodelica::Ref<Absyn::Path>,
) -> (FCore::Cache, bool) {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outIsArray: bool;
    let mut el: metamodelica::Ref<SCode::Element>;
    let mut p: metamodelica::Ref<Absyn::Path>;
    let mut env: FCore::Graph;
    match '__try0: {
        (outCache, el, env) = unwrap_break_err!(lookupClass(&inCache, inEnv, inPath, None), '__try0);
        outIsArray = (::match_deref::match_deref! { match &(&*el) {
            Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: Some(_), .. }, .. }, .. } => true,
            Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { attributes: SCode::Attributes { arrayDims: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, .. }, .. } => true,
            Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __esc_p, .. }, .. }, .. } => {
                p = (*__esc_p).clone();
                (outCache, outIsArray) = isArrayType(outCache.clone(), &env, metamodelica::AsArg::as_arg(&p));
                outIsArray
            },
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok::<_, &'static str>((outIsArray.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outIsArray = __try0_o0;
        }
        Err(_) => {
            outIsArray = false;
        }
    }
    (outCache, outIsArray)
}
