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
use crate::ConnectionGraph;
use crate::FGraph;
use crate::InnerOuter;
use crate::Inst;
use crate::InstSection;
use crate::InstUtil;
use crate::Lookup;
use crate::Static;
use crate::UnitAbsyn;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::HashTableStringToPath;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_base::Algorithm;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
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
use openmodelica_util::AvlSetString;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

fn generatePositionalArgs(
    mut fieldNameList: metamodelica::List<ArcStr>,
    mut namedArgList: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut accList: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((fieldNameList, namedArgList.clone(), accList)) {
            (Deref @ metamodelica::ListNode::Nil, _, localAccList) => {
                return Ok((localAccList.clone().reverse(), namedArgList))
            },
            (Deref @ metamodelica::ListNode::Cons { head: firstFieldName, tail: restFieldNames }, localNamedArgList, localAccList) => {
                let mut exp: metamodelica::Ref<Absyn::Exp>;
                let mut localNamedArgList = (*localNamedArgList).clone();
                let mut localAccList = (*localAccList).clone();
                (exp, localNamedArgList) = findFieldExpInList(firstFieldName.clone(), metamodelica::AsArg::as_arg(&localNamedArgList))?;
                { (fieldNameList, namedArgList, accList) = (restFieldNames.clone(), localNamedArgList.clone(), metamodelica::cons(exp, localAccList.clone())); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn findFieldExpInList(
    mut firstFieldName: ArcStr,
    mut namedArgList: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
)> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outNamedArgList: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
    (outExp, outNamedArgList) = (::match_deref::match_deref! { match &((firstFieldName, &**namedArgList)) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            (metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: openmodelica_ast::Absyn::ComponentRef::interned_WILD() }), metamodelica::nil())
        },
        (localFieldName, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: aName, argValue: e }, tail: rest }) if (stringEq(&localFieldName, &aName)) => {
            (e.clone(), rest.clone())
        },
        (localFieldName, Deref @ metamodelica::ListNode::Cons { head: first, tail: rest }) => {
            let mut e: metamodelica::Ref<Absyn::Exp>;
            let mut rest = (*rest).clone();
            (e, rest) = findFieldExpInList(localFieldName.clone(), metamodelica::AsArg::as_arg(&rest))?;
            (e, metamodelica::cons(first.clone(), rest.clone()))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outExp, outNamedArgList))
}

fn checkInvalidPatternNamedArgs(
    mut args: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut fieldNameList: metamodelica::List<ArcStr>,
    mut status: Util::Status,
    mut info: &SourceInfo,
) -> Result<Util::Status> {
    let mut outStatus: Util::Status;
    outStatus = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            status
        },
        _ => {
            let mut argsNames: metamodelica::List<ArcStr>;
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            (argsNames, _) = AbsynUtil::getNamedFuncArgNamesAndValues(args);
            str1 = stringDelimitList(argsNames, literal!(","));
            str2 = stringDelimitList(fieldNameList, literal!(","));
            Error::addSourceMessage(&(Error::META_INVALID_PATTERN_NAMED_FIELD.clone()), list![str1, str2], info)?;
            openmodelica_util::Util::Status::FAILURE
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStatus)
}

pub(crate) fn elabPatternCheckDuplicateBindings(
    mut cache: FCore::Cache,
    mut env: &FCore::Graph,
    mut lhs: metamodelica::Ref<Absyn::Exp>,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Pattern>)> {
    let mut outCache: FCore::Cache;
    let mut pattern: metamodelica::Ref<DAE::Pattern>;
    (outCache, pattern) = elabPattern2(cache, env, lhs, ty, info, Error::getNumErrorMessages())?;
    checkPatternsDuplicateAsBindings(&(metamodelica::cons(pattern.clone(), metamodelica::nil())), info)?;
    Ok((outCache, pattern))
}

fn elabPattern(
    mut cache: FCore::Cache,
    mut env: &FCore::Graph,
    mut lhs: metamodelica::Ref<Absyn::Exp>,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Pattern>)> {
    let mut outCache: FCore::Cache;
    let mut pattern: metamodelica::Ref<DAE::Pattern>;
    (outCache, pattern) = elabPattern2(cache, env, lhs, ty, info, Error::getNumErrorMessages())?;
    Ok((outCache, pattern))
}

fn checkPatternsDuplicateAsBindings(
    mut patterns: &metamodelica::List<metamodelica::Ref<DAE::Pattern>>,
    mut info: &SourceInfo,
) -> Result<()> {
    let mut usedVariables: metamodelica::List<ArcStr>;
    (_, usedVariables) = traversePatternList(
        patterns,
        &fnptr!(
            findBoundVariables,
            metamodelica::Ref<DAE::Pattern>,
            metamodelica::List<ArcStr>
        ),
        metamodelica::nil(),
    )?;
    usedVariables = List::sortedUniqueOnlyDuplicates(
        List::sort(
            usedVariables,
            (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Util::strcmpBool(&__a0, &__a1))
            }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        )?,
        &fnptr!(stringEq, ArcStr, ArcStr),
    )?;
    if !((usedVariables).is_empty()) {
        Error::addSourceMessage(
            &(Error::DUPLICATE_DEFINITION.clone()),
            list![stringDelimitList(usedVariables, literal!(", "))],
            info,
        )?;
        return Err("fail");
    }
    Ok(())
}

fn findBoundVariables(
    mut pat: metamodelica::Ref<DAE::Pattern>,
    mut boundVars: metamodelica::List<ArcStr>,
) -> (metamodelica::Ref<DAE::Pattern>, metamodelica::List<ArcStr>) {
    let mut outPat: metamodelica::Ref<DAE::Pattern> = pat.clone();
    let mut outBoundVars: metamodelica::List<ArcStr>;
    outBoundVars = (match &*pat {
        DAE::Pattern::PAT_AS { id: __pat_id, .. } => metamodelica::cons(__pat_id.clone(), boundVars),
        DAE::Pattern::PAT_AS_FUNC_PTR { id: __pat_id, .. } => metamodelica::cons(__pat_id.clone(), boundVars),
        _ => boundVars,
    });
    (outPat, outBoundVars)
}

fn elabPattern2(
    mut inCache: FCore::Cache,
    mut env: &FCore::Graph,
    mut inLhs: metamodelica::Ref<Absyn::Exp>,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut info: &SourceInfo,
    mut numError: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Pattern>)> {
    let mut outCache: FCore::Cache;
    let mut pattern: metamodelica::Ref<DAE::Pattern> = metamodelica::Ref::new(DAE::Pattern::PAT_WILD);
    (outCache, pattern) = 'mc: {
        let __mc_input = (inCache, inLhs.clone(), ty.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::INTEGER { value: i }, _) => {
                    let mut et: Option<metamodelica::Ref<DAE::Type>>;
                    et = validPatternType(ty.clone(), DAE::T_INTEGER_DEFAULT().clone(), inLhs.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::REAL { value: r#str }, _) => {
                    let mut r: metamodelica::Real;
                    let mut et: Option<metamodelica::Ref<DAE::Type>>;
                    et = validPatternType(ty.clone(), DAE::T_REAL_DEFAULT().clone(), inLhs.clone(), info)?;
                    r = stringReal(r#str.clone())?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: metamodelica::Ref::new(DAE::Exp::RCONST { real: r }) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UMINUS { .. }, exp: Deref @ Absyn::Exp::INTEGER { value: i } }, _) => {
                    let mut et: Option<metamodelica::Ref<DAE::Type>>;
                    let mut i = (*i).clone();
                    et = validPatternType(ty.clone(), DAE::T_INTEGER_DEFAULT().clone(), inLhs.clone(), info)?;
                    i = -(i.clone());
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UMINUS { .. }, exp: Deref @ Absyn::Exp::REAL { value: r#str } }, _) => {
                    let mut r: metamodelica::Real;
                    let mut et: Option<metamodelica::Ref<DAE::Type>>;
                    et = validPatternType(ty.clone(), DAE::T_REAL_DEFAULT().clone(), inLhs.clone(), info)?;
                    r = stringReal(r#str.clone())?;
                    r = -(r);
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: metamodelica::Ref::new(DAE::Exp::RCONST { real: r }) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::STRING { value: s }, _) => {
                    let mut et: Option<metamodelica::Ref<DAE::Type>>;
                    let mut s = (*s).clone();
                    et = validPatternType(ty.clone(), DAE::T_STRING_DEFAULT().clone(), inLhs.clone(), info)?;
                    s = System::unescapedString(s.clone());
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: metamodelica::Ref::new(DAE::Exp::SCONST { string: s.clone() }) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::BOOL { value: b }, _) => {
                    let mut et: Option<metamodelica::Ref<DAE::Type>>;
                    et = validPatternType(ty.clone(), DAE::T_BOOL_DEFAULT().clone(), inLhs.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: metamodelica::Ref::new(DAE::Exp::BCONST { bool: b.clone() }) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::ListNode::Nil }, _) => {
                    let mut et: Option<metamodelica::Ref<DAE::Type>>;
                    et = validPatternType(ty.clone(), DAE::T_METALIST_DEFAULT().clone(), inLhs.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: metamodelica::Ref::new(DAE::Exp::LIST { valList: metamodelica::nil() }) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::ARRAY { arrayExp: exps @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, _) => {
                    let mut lhs: metamodelica::Ref<Absyn::Exp>;
                    let mut cache = (*cache).clone();
                    let mut pattern: metamodelica::Ref<DAE::Pattern> = pattern.clone();
                    lhs = List::fold(&(exps.clone().reverse()), &fnptr!(AbsynUtil::makeCons, metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>), metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: metamodelica::nil() }))?;
                    (cache, pattern) = elabPattern(cache.clone(), env, lhs.clone(), ty.clone(), info)?;
                    Ok(((cache.clone(), pattern.clone()), pattern.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            pattern = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "NONE", subscripts: Deref @ metamodelica::ListNode::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Nil, argNames: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
                    validPatternType(ty.clone(), DAE::T_NONE_DEFAULT().clone(), inLhs.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_CONSTANT { ty: None, exp: metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: None }) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "SOME", subscripts: Deref @ metamodelica::ListNode::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil }, argNames: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Type::T_METAOPTION { ty: ty2 }) => {
                    let mut cache = (*cache).clone();
                    let mut pattern: metamodelica::Ref<DAE::Pattern> = pattern.clone();
                    (cache, pattern) = elabPattern(cache.clone(), env, exp.clone(), ty2.clone(), info)?;
                    Ok(((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_SOME { pat: pattern.clone() })), pattern.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            pattern = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CONS { head, rest: tail }, tyTail @ Deref @ DAE::Type::T_METALIST { ty: tyHead }) => {
                    let mut patternHead: metamodelica::Ref<DAE::Pattern>;
                    let mut patternTail: metamodelica::Ref<DAE::Pattern>;
                    let mut cache = (*cache).clone();
                    let mut tyHead = (*tyHead).clone();
                    tyHead = Types::boxIfUnboxedType(tyHead.clone());
                    (cache, patternHead) = elabPattern(cache.clone(), env, head.clone(), tyHead.clone(), info)?;
                    (cache, patternTail) = elabPattern(cache.clone(), env, tail.clone(), tyTail.clone(), info)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_CONS { head: patternHead.clone(), tail: patternTail.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } }, _) => {
                    let mut cache = (*cache).clone();
                    let mut pattern: metamodelica::Ref<DAE::Pattern> = pattern.clone();
                    (cache, pattern) = elabPattern2(cache.clone(), env, exp.clone(), ty.clone(), info, numError)?;
                    Ok(((cache.clone(), pattern.clone()), pattern.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            pattern = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::TUPLE { expressions: exps }, Deref @ DAE::Type::T_METATUPLE { types: tys }) => {
                    let mut patterns: metamodelica::List<metamodelica::Ref<DAE::Pattern>>;
                    let mut cache = (*cache).clone();
                    let mut tys = (*tys).clone();
                    tys = List::map(tys.clone(), &fnptr!(Types::boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?;
                    (cache, patterns) = elabPatternTuple(cache.clone(), env, metamodelica::AsArg::as_arg(&exps), metamodelica::AsArg::as_arg(&tys), info, &inLhs)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_META_TUPLE { patterns: patterns.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::TUPLE { expressions: exps }, Deref @ DAE::Type::T_TUPLE { types: tys, .. }) => {
                    let mut patterns: metamodelica::List<metamodelica::Ref<DAE::Pattern>>;
                    let mut cache = (*cache).clone();
                    (cache, patterns) = elabPatternTuple(cache.clone(), env, metamodelica::AsArg::as_arg(&exps), metamodelica::AsArg::as_arg(&tys), info, &inLhs)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_CALL_TUPLE { patterns: patterns.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, lhs @ Deref @ Absyn::Exp::CALL { function_: fcr, functionArgs: fargs, .. }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: utPath }, .. }) => {
                    let mut cache = (*cache).clone();
                    let mut pattern: metamodelica::Ref<DAE::Pattern> = pattern.clone();
                    (cache, pattern) = elabPatternCall(cache.clone(), env.clone(), AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&fcr))?, metamodelica::AsArg::as_arg(&fargs), utPath.clone(), info, lhs.clone())?;
                    Ok(((cache.clone(), pattern.clone()), pattern.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            pattern = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, lhs @ Deref @ Absyn::Exp::CALL { function_: fcr, functionArgs: fargs, .. }, Deref @ DAE::Type::T_METAUNIONTYPE { path: utPath, .. }) => {
                    let mut cache = (*cache).clone();
                    let mut pattern: metamodelica::Ref<DAE::Pattern> = pattern.clone();
                    (cache, pattern) = elabPatternCall(cache.clone(), env.clone(), AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&fcr))?, metamodelica::AsArg::as_arg(&fargs), utPath.clone(), info, lhs.clone())?;
                    Ok(((cache.clone(), pattern.clone()), pattern.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            pattern = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, lhs @ Deref @ Absyn::Exp::CALL { function_: fcr, functionArgs: fargs, .. }, Deref @ DAE::Type::T_METARECORD { utPath, .. }) => {
                    let mut cache = (*cache).clone();
                    let mut pattern: metamodelica::Ref<DAE::Pattern> = pattern.clone();
                    (cache, pattern) = elabPatternCall(cache.clone(), env.clone(), AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&fcr))?, metamodelica::AsArg::as_arg(&fargs), utPath.clone(), info, lhs.clone())?;
                    Ok(((cache.clone(), pattern.clone()), pattern.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            pattern = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, Deref @ Absyn::Exp::CREF { .. }, ty1) => {
                            if !((Types::isBoxedType(metamodelica::AsArg::as_arg(&ty1)) || (match &*(Types::unboxedType(ty1.clone())?) {
                DAE::Type::T_ENUMERATION { .. } => true,
                DAE::Type::T_INTEGER { .. } => true,
                DAE::Type::T_REAL { .. } => true,
                DAE::Type::T_STRING { .. } => true,
                DAE::Type::T_BOOL { .. } => true,
                _ => false,
            }))) { return Err("guard") }
                            let mut ty2: metamodelica::Ref<DAE::Type>;
                            let mut et: Option<metamodelica::Ref<DAE::Type>>;
                            let mut elabExp: metamodelica::Ref<DAE::Exp>;
                            let mut r#const: DAE::Const;
                            let mut val: metamodelica::Ref<Values::Value>;
                            let mut cache = (*cache).clone();
                            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Static::elabExp(cache.clone(), env.clone(), inLhs.clone(), false, false, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?) {
                                (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            cache = metamodelica::Own::own(__pa0);
                            elabExp = metamodelica::Own::own(__pa1);
                            ty2 = metamodelica::Own::own(__pa2);
                            r#const = metamodelica::Own::own(__pa3);
                            et = validPatternType(ty1.clone(), ty2.clone(), inLhs.clone(), info)?;
                            let true = (Types::isConstant(r#const)) else { return Err("pattern mismatch") };
                            (cache, val) = Ceval::ceval(cache.clone(), env.clone(), elabExp.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                            elabExp = ValuesUtil::valueExp(val.clone(), None)?;
                            Ok((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_CONSTANT { ty: et.clone(), exp: elabExp.clone() })))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::AS { id, exp }, ty2) => {
                    let mut ty1: metamodelica::Ref<DAE::Type>;
                    let mut et: Option<metamodelica::Ref<DAE::Type>>;
                    let mut lhs: metamodelica::Ref<Absyn::Exp>;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut cache = (*cache).clone();
                    let mut pattern: metamodelica::Ref<DAE::Pattern> = pattern.clone();
                    let (__pa0, __t3, _, _, _, _) = Lookup::lookupIdent(cache.clone(), env, id.clone())?;
                    let __arc4 = __t3.clone();
                    let DAE::TYPES_VAR { ty: __pa1, attributes: __pa2, .. } = &*__arc4;
                    cache = metamodelica::Own::own(__pa0);
                    ty1 = metamodelica::Own::own(__pa1);
                    attr = metamodelica::Own::own(__pa2);
                    lhs = metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: id.clone(), subscripts: metamodelica::nil() }) });
                    Static::checkAssignmentToInput(&lhs, &attr, env, false, info)?;
                    et = validPatternType(ty2.clone(), ty1.clone(), inLhs.clone(), info)?;
                    (cache, pattern) = elabPattern(cache.clone(), env, exp.clone(), ty2.clone(), info)?;
                    pattern = if (Types::isFunctionType(metamodelica::AsArg::as_arg(&ty2))) {metamodelica::Ref::new(DAE::Pattern::PAT_AS_FUNC_PTR { id: id.clone(), pat: pattern.clone() })} else {metamodelica::Ref::new(DAE::Pattern::PAT_AS { id: id.clone(), ty: et.clone(), attr: attr.clone(), pat: pattern.clone() })};
                    Ok(((cache.clone(), pattern.clone()), pattern.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            pattern = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: Deref @ metamodelica::ListNode::Nil } }, ty2) => {
                    let mut ty1: metamodelica::Ref<DAE::Type>;
                    let mut et: Option<metamodelica::Ref<DAE::Type>>;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut variability: SCode::Variability;
                    let mut cache = (*cache).clone();
                    let mut pattern: metamodelica::Ref<DAE::Pattern> = pattern.clone();
                    let (__pa0, __t4, _, _, _, _) = Lookup::lookupIdent(cache.clone(), env, id.clone())?;
                    let __arc6 = __t4.clone();
                    let DAE::TYPES_VAR { ty: __pa1, attributes: __t5, .. } = &*__arc6;
                    let __arc7 = __t5.clone();
                    let __pa3 = (__arc7).clone();
                    let DAE::ATTR { variability: __pa2, .. } = &*__arc7;
                    cache = metamodelica::Own::own(__pa0);
                    ty1 = metamodelica::Own::own(__pa1);
                    variability = metamodelica::Own::own(__pa2);
                    attr = metamodelica::Own::own(__pa3);
                    if SCodeUtil::isParameterOrConst(variability) {
                        Error::addSourceMessage(&(Error::PATTERN_VAR_NOT_VARIABLE.clone()), list![id.clone(), SCodeDump::unparseVariability(variability)], info)?;
                        return Err("fail");
                    }
                    Static::checkAssignmentToInput(&inLhs, &attr, env, false, info)?;
                    et = validPatternType(ty2.clone(), ty1.clone(), inLhs.clone(), info)?;
                    pattern = if (Types::isFunctionType(metamodelica::AsArg::as_arg(&ty2))) {metamodelica::Ref::new(DAE::Pattern::PAT_AS_FUNC_PTR { id: id.clone(), pat: openmodelica_frontend_types::DAE::Pattern::interned_PAT_WILD() })} else {metamodelica::Ref::new(DAE::Pattern::PAT_AS { id: id.clone(), ty: et.clone(), attr: attr.clone(), pat: openmodelica_frontend_types::DAE::Pattern::interned_PAT_WILD() })};
                    Ok(((cache.clone(), pattern.clone()), pattern.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            pattern = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::AS { id, exp: _ }, _) => {
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupIdent(cache.clone(), env, id.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Error::addSourceMessage(&(Error::LOOKUP_VARIABLE_ERROR.clone()), list![id.clone(), literal!("")], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "NONE", subscripts: Deref @ metamodelica::ListNode::Nil } }, _) => {
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupIdent(cache.clone(), env, literal!("NONE")), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Error::addSourceMessage(&(Error::META_NONE_CREF.clone()), metamodelica::nil(), info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: Deref @ metamodelica::ListNode::Nil } }, _) => {
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupIdent(cache.clone(), env, id.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    let false = (metamodelica::stringEq(&(literal!("NONE")), &id)) else { return Err("pattern mismatch") };
                    Error::addSourceMessage(&(Error::LOOKUP_VARIABLE_ERROR.clone()), list![id.clone(), literal!("")], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::WILD { .. } }, _) => {
                    Ok((cache.clone(), openmodelica_frontend_types::DAE::Pattern::interned_PAT_WILD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::EXPRESSIONCOMMENT { .. }, _) => {
                    let mut cache = (*cache).clone();
                    let mut pattern: metamodelica::Ref<DAE::Pattern> = pattern.clone();
                    (cache, pattern) = elabPattern2(cache.clone(), env, var_field!((*inLhs).exp, Absyn::Exp::EXPRESSIONCOMMENT).clone(), ty.clone(), info, numError)?;
                    Ok(((cache.clone(), pattern.clone()), pattern.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            pattern = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, lhs, _) => {
                    let mut r#str: ArcStr;
                    let true = (numError == Error::getNumErrorMessages()) else { return Err("pattern mismatch") };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::printExpStr(lhs.clone())?); __mm_s.push_str(&*literal!(" of type ")); __mm_s.push_str(&*TypesDump::unparseType(ty.clone())?); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::META_INVALID_PATTERN.clone()), list![r#str.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, pattern))
}

fn elabPatternTuple(
    mut inCache: FCore::Cache,
    mut env: &FCore::Graph,
    mut inExps: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inTys: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut info: &SourceInfo,
    mut lhs: &metamodelica::Ref<Absyn::Exp>,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Pattern>>)> {
    let mut outCache: FCore::Cache;
    let mut patterns: metamodelica::List<metamodelica::Ref<DAE::Pattern>>;
    (outCache, patterns) = (::match_deref::match_deref! { match (inExps, inTys) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            (cache, metamodelica::nil())
        },
        (Deref @ metamodelica::ListNode::Cons { head: exp, tail: exps }, Deref @ metamodelica::ListNode::Cons { head: ty, tail: tys }) => {
            let mut cache = inCache;
            let mut pattern: metamodelica::Ref<DAE::Pattern>;
            (cache, pattern) = elabPattern(cache, env, exp.clone(), ty.clone(), info)?;
            (cache, patterns) = elabPatternTuple(cache, env, exps, tys, info, lhs)?;
            (cache, metamodelica::cons(pattern, patterns))
        },
        _ => {
            let mut s: ArcStr;
            s = Dump::printExpStr(lhs.clone())?;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("pattern ")); __mm_s.push_str(&*s); ArcStr::from(__mm_s) };
            Error::addSourceMessage(&(Error::WRONG_NO_OF_ARGS.clone()), list![s], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, patterns))
}

fn elabPatternCall(
    mut inCache: FCore::Cache,
    mut env: FCore::Graph,
    mut callPath: metamodelica::Ref<Absyn::Path>,
    mut fargs: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut utPath: metamodelica::Ref<Absyn::Path>,
    mut info: &SourceInfo,
    mut lhs: metamodelica::Ref<Absyn::Exp>,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Pattern>)> {
    let mut outCache: FCore::Cache;
    let mut pattern: metamodelica::Ref<DAE::Pattern>;
    (outCache, pattern) = 'mc: {
        let __mc_input = (inCache, &**fargs, utPath);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, argNames: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, _) => {
                    Error::addSourceMessage(&(Error::PATTERN_MIXED_POS_NAMED.clone()), list![AbsynUtil::pathString(callPath.clone(), literal!("."), true, false)?], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: funcArgs, argNames: namedArgList }, utPath2) => {
                            let mut utPath1: metamodelica::Ref<Absyn::Path>;
                            let mut fqPath: metamodelica::Ref<Absyn::Path>;
                            let mut index: i32;
                            let mut numPosArgs: i32;
                            let mut invalidArgs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                            let mut funcArgsNamedFixed: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                            let mut funcArgs2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                            let mut fieldNameList: metamodelica::List<ArcStr>;
                            let mut fieldNamesNamed: metamodelica::List<ArcStr>;
                            let mut fieldTypeList: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                            let mut typeVars: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                            let mut fieldVarList: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                            let mut patterns: metamodelica::List<metamodelica::Ref<DAE::Pattern>>;
                            let mut knownSingleton: bool;
                            let mut allWild: bool;
                            let mut cache = (*cache).clone();
                            let mut funcArgs = (*funcArgs).clone();
                            let mut namedArgList = (*namedArgList).clone();
                            (cache, _, _) = Lookup::lookupType(cache.clone(), env.clone(), callPath.clone(), None)?;
                            let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(Lookup::lookupType(cache.clone(), env.clone(), callPath.clone(), None)?) {
                                (__pa0, Deref @ DAE::Type::T_METARECORD { utPath: __pa1, index: __pa2, fields: __pa3, typeVars: __pa4, knownSingleton: __pa5, path: __pa6 }, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            cache = metamodelica::Own::own(__pa0);
                            utPath1 = metamodelica::Own::own(__pa1);
                            index = metamodelica::Own::own(__pa2);
                            fieldVarList = metamodelica::Own::own(__pa3);
                            typeVars = metamodelica::Own::own(__pa4);
                            knownSingleton = metamodelica::Own::own(__pa5);
                            fqPath = metamodelica::Own::own(__pa6);
                            validUniontype(utPath1.clone(), utPath2.clone(), info, lhs.clone())?;
                            fieldTypeList = List::map(fieldVarList.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| Types::getVarType(&__a0))?;
                            fieldNameList = List::map(fieldVarList.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(TypesDump::getVarName(&__a0)) })?;
                            if Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())? {
                                for mut namedArg in &*namedArgList.clone() {
                                    let () = (::match_deref::match_deref! { match &(namedArg.clone()) {
                Deref @ Absyn::NamedArg { argValue: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::WILD { .. } }, .. } => {
                            Error::addSourceMessage(&(Error::META_EMPTY_CALL_PATTERN.clone()), list![namedArg.argName.clone()], info)?;
                            ()
                },
                _ => (),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                                }
                                if (namedArgList).is_empty() && !((funcArgs).is_empty()) {
                                    allWild = true;
                                    for mut arg in &*funcArgs.clone() {
                                                allWild = (::match_deref::match_deref! { match &(arg.clone()) {
                Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::WILD { .. } } => true,
                _ => false,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                                                if !(allWild) {
                                                    break;
                                                }
                                    }
                                    if allWild {
                                                Error::addSourceMessage(&(Error::META_ALL_EMPTY.clone()), list![AbsynUtil::pathString(callPath.clone(), literal!("."), true, false)?], info)?;
                                    }
                                }
                            }
                            (funcArgs, namedArgList) = checkForAllWildCall(funcArgs.clone(), namedArgList.clone(), ((fieldNameList).len() as i32));
                            numPosArgs = ((funcArgs).len() as i32);
                            (_, fieldNamesNamed) = List::split(fieldNameList.clone(), numPosArgs)?;
                            checkMissingArgs(&fqPath, numPosArgs, &fieldNamesNamed, ((namedArgList).len() as i32), info);
                            (funcArgsNamedFixed, invalidArgs) = generatePositionalArgs(fieldNamesNamed.clone(), namedArgList.clone(), metamodelica::nil())?;
                            funcArgs2 = listAppend(funcArgs.clone(), funcArgsNamedFixed.clone());
                            let Util::SUCCESS { .. } = (checkInvalidPatternNamedArgs(invalidArgs.clone(), fieldNameList.clone(), openmodelica_util::Util::Status::SUCCESS, info)?) else { return Err("pattern mismatch") };
                            (cache, patterns) = elabPatternTuple(cache.clone(), &env, &funcArgs2, &fieldTypeList, info, &lhs)?;
                            Ok((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_CALL { name: fqPath.clone(), index: index, patterns: patterns.clone(), fields: fieldVarList.clone(), typeVars: typeVars.clone(), knownSingleton: knownSingleton })))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: funcArgs, argNames: namedArgList }, utPath2) => {
                    let mut fqPath: metamodelica::Ref<Absyn::Path>;
                    let mut numPosArgs: i32;
                    let mut invalidArgs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut funcArgsNamedFixed: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut funcArgs2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut fieldNameList: metamodelica::List<ArcStr>;
                    let mut fieldNamesNamed: metamodelica::List<ArcStr>;
                    let mut fieldTypeList: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut fieldVarList: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut patterns: metamodelica::List<metamodelica::Ref<DAE::Pattern>>;
                    let mut namedPatterns: metamodelica::List<(metamodelica::Ref<DAE::Pattern>, ArcStr, metamodelica::Ref<DAE::Type>)>;
                    let mut cache = (*cache).clone();
                    let mut funcArgs = (*funcArgs).clone();
                    let mut namedArgList = (*namedArgList).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupType(cache.clone(), env.clone(), callPath.clone(), None)?) {
                        (__pa0, Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, varLst: __pa1, .. }, path: __pa2, .. }, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    fieldVarList = metamodelica::Own::own(__pa1);
                    fqPath = metamodelica::Own::own(__pa2);
                    let true = (AbsynUtil::pathEqual(&fqPath, metamodelica::AsArg::as_arg(&utPath2))) else { return Err("pattern mismatch") };
                    fieldTypeList = List::map(fieldVarList.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| Types::getVarType(&__a0))?;
                    fieldNameList = List::map(fieldVarList.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(TypesDump::getVarName(&__a0)) })?;
                    (funcArgs, namedArgList) = checkForAllWildCall(funcArgs.clone(), namedArgList.clone(), ((fieldNameList).len() as i32));
                    numPosArgs = ((funcArgs).len() as i32);
                    (_, fieldNamesNamed) = List::split(fieldNameList.clone(), numPosArgs)?;
                    checkMissingArgs(&fqPath, numPosArgs, &fieldNamesNamed, ((namedArgList).len() as i32), info);
                    (funcArgsNamedFixed, invalidArgs) = generatePositionalArgs(fieldNamesNamed.clone(), namedArgList.clone(), metamodelica::nil())?;
                    funcArgs2 = listAppend(funcArgs.clone(), funcArgsNamedFixed.clone());
                    let Util::SUCCESS { .. } = (checkInvalidPatternNamedArgs(invalidArgs.clone(), fieldNameList.clone(), openmodelica_util::Util::Status::SUCCESS, info)?) else { return Err("pattern mismatch") };
                    (cache, patterns) = elabPatternTuple(cache.clone(), &env, &funcArgs2, &fieldTypeList, info, &lhs)?;
                    namedPatterns = List::zip3(patterns.clone(), fieldNameList.clone(), List::map(fieldTypeList.clone(), &Types::simplifyType)?);
                    namedPatterns = List::filterOnTrue(namedPatterns.clone(), (std::sync::Arc::new(move |__a0: (metamodelica::Ref<DAE::Pattern>, ArcStr, metamodelica::Ref<DAE::Type>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(filterEmptyPattern(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::Pattern>, ArcStr, metamodelica::Ref<DAE::Type>)) -> Result<bool> + 'static>))?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Pattern::PAT_CALL_NAMED { name: fqPath.clone(), patterns: namedPatterns.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _) => {
                    let mut s: ArcStr;
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupType(cache.clone(), env.clone(), callPath.clone(), None), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    s = AbsynUtil::pathString(callPath.clone(), literal!("."), true, false)?;
                    Error::addSourceMessage(&(Error::META_CONSTRUCTOR_NOT_RECORD.clone()), list![s.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, pattern))
}

fn checkMissingArgs(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut numPosArgs: i32,
    mut missingFieldNames: &metamodelica::List<ArcStr>,
    mut numNamedArgs: i32,
    mut info: &SourceInfo,
) -> () {
    let () = (::match_deref::match_deref! { match &((&**missingFieldNames, numNamedArgs)) {
        (Deref @ metamodelica::ListNode::Nil, 0) => (),
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn checkForAllWildCall(
    mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut named: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut numFields: i32,
) -> (
    metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
) {
    let mut outArgs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut outNamed: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
    (outArgs, outNamed) = (::match_deref::match_deref! { match &((args.clone(), named.clone())) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::ALLWILD { .. } }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil) => (metamodelica::nil(), metamodelica::nil()),
        _ => (args, named),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outArgs, outNamed)
}

fn validPatternType(
    mut inTy1: metamodelica::Ref<DAE::Type>,
    mut inTy2: metamodelica::Ref<DAE::Type>,
    mut lhs: metamodelica::Ref<Absyn::Exp>,
    mut info: &SourceInfo,
) -> Result<Option<metamodelica::Ref<DAE::Type>>> {
    let mut ty: Option<metamodelica::Ref<DAE::Type>>;
    ty = 'mc: {
        let __mc_input = (inTy1, inTy2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_METABOXED { ty: ty1 }, ty2) => {
                    let mut et: metamodelica::Ref<DAE::Type>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crefExp: metamodelica::Ref<DAE::Exp>;
                    let mut ty1 = (*ty1).clone();
                    cr = ComponentReferenceBasics::makeCrefIdent(literal!("#DUMMY#"), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    crefExp = Expression::crefExp(cr.clone())?;
                    (_, ty1) = Types::matchType(crefExp.clone(), ty1.clone(), ty2.clone(), true)?;
                    et = Types::simplifyType(ty1.clone())?;
                    Ok(Some(et.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty1, ty2) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crefExp: metamodelica::Ref<DAE::Exp>;
                    cr = ComponentReferenceBasics::makeCrefIdent(literal!("#DUMMY#"), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    crefExp = Expression::crefExp(cr.clone())?;
                    Types::matchType(crefExp.clone(), ty1.clone(), ty2.clone(), true)?;
                    Ok(None)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty1, ty2) => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    s = Dump::printExpStr(lhs.clone())?;
                    s1 = TypesDump::unparseType(ty1.clone())?;
                    s2 = TypesDump::unparseType(ty2.clone())?;
                    Error::addSourceMessage(&(Error::META_TYPE_MISMATCH_PATTERN.clone()), list![s.clone(), s1.clone(), s2.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(ty)
}

fn validUniontype(
    mut path1: metamodelica::Ref<Absyn::Path>,
    mut path2: metamodelica::Ref<Absyn::Path>,
    mut info: &SourceInfo,
    mut lhs: metamodelica::Ref<Absyn::Exp>,
) -> Result<()> {
    if !(AbsynUtil::pathEqual(&path1, &path2)) {
        Error::addSourceMessage(
            &(Error::META_CONSTRUCTOR_NOT_PART_OF_UNIONTYPE.clone()),
            list![
                Dump::printExpStr(lhs)?,
                AbsynUtil::pathString(path1, literal!("."), true, false)?,
                AbsynUtil::pathString(path2, literal!("."), true, false)?
            ],
            info,
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn elabMatchExpression(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut matchExp: metamodelica::Ref<Absyn::Exp>,
    mut r#impl: bool,
    mut performVectorization: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut numError: i32 = Error::getNumErrorMessages();
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache, inEnv, &*matchExp, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::MATCHEXP { matchTy, inputExp: inExp, localDecls: decls, cases, .. }, pre) => {
                    let mut inExps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut matchDecls: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut elabExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut elabCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut prop: DAE::Properties;
                    let mut elabProps: metamodelica::List<DAE::Properties>;
                    let mut resType: metamodelica::Ref<DAE::Type>;
                    let mut et: metamodelica::Ref<DAE::Type>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut ht: (metamodelica::Array<metamodelica::List<(ArcStr, i32)>>, (i32, i32, metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>), i32, (HashTableStringToPath::FuncHashCref, HashTableStringToPath::FuncCrefEqual, HashTableStringToPath::FuncCrefStr, HashTableStringToPath::FuncExpStr));
                    let mut elabMatchTy: DAE::MatchType;
                    let mut hashSize: i32;
                    let mut inputAliases: metamodelica::List<metamodelica::List<ArcStr>>;
                    let mut inputAliasesAndCrefs: metamodelica::List<metamodelica::List<ArcStr>>;
                    let mut declsTree: metamodelica::Ref<AvlSetString::Tree>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut matchTy = (*matchTy).clone();
                    inExps = convertExpToPatterns(inExp.clone());
                    (inExps, inputAliases, inputAliasesAndCrefs) = List::map_3(&inExps, &move |__a0: metamodelica::Ref<Absyn::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getInputAsBinding(&__a0)) })?;
                    (cache, elabExps, elabProps) = Static::elabExpList(cache.clone(), env.clone(), &inExps, r#impl, performVectorization, pre.clone(), info.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(addLocalDecls(cache.clone(), env.clone(), decls.clone(), arcstr::literal!(FCore::matchScopeName), r#impl, &info)?) {
                        (__pa0, Some((__pa1, DAE::DAElist { elementLst: __pa2 }, __pa3))) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    env = metamodelica::Own::own(__pa1);
                    matchDecls = metamodelica::Own::own(__pa2);
                    declsTree = metamodelica::Own::own(__pa3);
                    tys = List::map(elabProps.clone(), &move |__a0: DAE::Properties| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::getPropType(&__a0)) })?;
                    env = addAliasesToEnv(env.clone(), tys.clone(), inputAliases.clone(), &info)?;
                    (cache, elabCases, resType) = elabMatchCases(cache.clone(), env.clone(), cases.clone(), tys.clone(), &inputAliasesAndCrefs, &declsTree, r#impl, performVectorization, metamodelica::AsArg::as_arg(&pre), info.clone())?;
                    prop = DAE::Properties::PROP { type_: resType.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR };
                    et = Types::simplifyType(resType.clone())?;
                    checkMatchSingleInfallibleCase(matchTy.clone(), &elabCases, &info)?;
                    checkInfallibleNoBindingPatterns(&elabCases, matchTy.clone(), &info)?;
                    (elabExps, inputAliases, elabCases) = filterUnusedPatterns(elabExps.clone(), inputAliases.clone(), elabCases.clone(), &info, !(isSingleInfallibleMatch(matchTy.clone(), &elabCases)?));
                    elabCases = caseDeadCodeElimination(matchTy.clone(), &elabCases, &(metamodelica::nil()), metamodelica::nil(), false)?;
                    matchTy = optimizeContinueToMatch(matchTy.clone(), &elabCases, &info)?;
                    elabCases = optimizeContinueJumps(matchTy.clone(), elabCases.clone())?;
                    hashSize = Util::nextPrime(((matchDecls).len() as i32));
                    ht = getUsedLocalCrefs(Flags::isSet(Flags::PATTERNM_SKIP_FILTER_UNUSED_AS_BINDINGS.clone())?, metamodelica::Ref::new(DAE::Exp::MATCHEXPRESSION { matchType: openmodelica_frontend_types::DAE::MatchType::MATCHCONTINUE, inputs: elabExps.clone(), aliases: inputAliases.clone(), localDecls: matchDecls.clone(), cases: elabCases.clone(), et: et.clone() }), hashSize)?;
                    (matchDecls, ht) = filterUnusedDecls(&matchDecls, &ht, metamodelica::nil(), HashTableStringToPath::emptyHashTableSized(hashSize))?;
                    (elabExps, inputAliases, elabCases) = filterUnusedPatterns(elabExps.clone(), inputAliases.clone(), elabCases.clone(), &info, false);
                    (elabMatchTy, elabCases) = optimizeMatchToSwitch(matchTy.clone(), elabCases.clone(), &info);
                    elabMatchTy = unboxSwitchType(elabMatchTy.clone(), &elabExps)?;
                    checkConstantMatchInputs(&elabExps, &info)?;
                    exp = metamodelica::Ref::new(DAE::Exp::MATCHEXPRESSION { matchType: elabMatchTy.clone(), inputs: elabExps.clone(), aliases: inputAliases.clone(), localDecls: matchDecls.clone(), cases: elabCases.clone(), et: et.clone() });
                    Ok((cache.clone(), exp.clone(), prop.clone()))
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
                    let true = (numError == Error::getNumErrorMessages()) else { return Err("pattern mismatch") };
                    r#str = Dump::printExpStr(matchExp.clone())?;
                    Error::addSourceMessage(&(Error::META_MATCH_GENERAL_FAILURE.clone()), list![r#str.clone()], &info)?;
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

fn checkConstantMatchInputs(
    mut inputs: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut info: &SourceInfo,
) -> Result<()> {
    for mut i in &**inputs {
        if Expression::isConstValue(metamodelica::AsArg::as_arg(&i))? {
            Error::addSourceMessage(
                &(Error::META_MATCH_CONSTANT.clone()),
                list![ExpressionBasics::printExpStr(i.clone())?],
                info,
            )?;
        }
    }
    Ok(())
}

fn optimizeMatchToSwitch(
    mut matchTy: Absyn::MatchType,
    mut cases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut info: &SourceInfo,
) -> (DAE::MatchType, metamodelica::List<metamodelica::Ref<DAE::MatchCase>>) {
    let mut outType: DAE::MatchType = DAE::MatchType::MATCHCONTINUE;
    let mut outCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>> = metamodelica::nil();
    (outType, outCases) = 'mc: {
        let __mc_input = matchTy;
        if let Ok(__v) = (|| -> Result<_> {
            let Absyn::MatchType::MATCHCONTINUE { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok((
                openmodelica_frontend_types::DAE::MatchType::MATCHCONTINUE,
                cases.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut tpl: (i32, metamodelica::Ref<DAE::Type>, i32);
            let mut patternMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>;
            let mut optPatternMatrix: metamodelica::List<Option<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>>;
            let mut numNonEmptyColumns: i32;
            let mut r#str: ArcStr;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut outCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>> = outCases.clone();
            let mut outType: DAE::MatchType = outType.clone();
            let true = (((cases).len() as i32) > 2) else {
                return Err("pattern mismatch");
            };
            for mut c in &*cases {
                ::match_deref::match_deref! { match &(c.clone()) {
                    Deref @ DAE::MatchCase { patternGuard: None, .. } => (),
                    _ => return Err("pattern mismatch"),
                } };
            }
            patternMatrix = List::transposeList(List::map(
                cases.clone(),
                &move |__a0: metamodelica::Ref<DAE::MatchCase>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(getCasePatterns(&__a0))
                },
            )?)?;
            (optPatternMatrix, numNonEmptyColumns) =
                removeWildPatternColumnsFromMatrix(patternMatrix.clone(), metamodelica::nil(), 0)?;
            tpl = findPatternToConvertToSwitch(&optPatternMatrix, 1, numNonEmptyColumns, info)?;
            (_, ty, _) = tpl.clone();
            r#str = TypesDump::unparseType(ty.clone())?;
            Error::assertionOrAddSourceMessage(
                !(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?),
                &(Error::MATCH_TO_SWITCH_OPTIMIZATION.clone()),
                list![r#str.clone()],
                info,
            )?;
            outType = DAE::MatchType::MATCH {
                switch: Some(tpl.clone()),
            };
            outCases = optimizeSwitchedMatchCases(&outType, cases.clone());
            Ok(((outType.clone(), outCases.clone()), outCases.clone(), outType.clone()))
        })() {
            outCases = __wb0;
            outType = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((DAE::MatchType::MATCH { switch: None }, cases.clone()))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outType, outCases)
}

fn optimizeSwitchedMatchCases(
    mut inMatchType: &DAE::MatchType,
    mut inCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
) -> metamodelica::List<metamodelica::Ref<DAE::MatchCase>> {
    let mut outCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
    outCases = (::match_deref::match_deref! { match &(inMatchType) {
        DAE::MatchType::MATCH { switch: Some((_, Deref @ DAE::Type::T_METATYPE { .. }, _)) } => {
            let mut pat: metamodelica::Ref<DAE::Pattern>;
            let mut patl: metamodelica::List<metamodelica::Ref<DAE::Pattern>>;
            ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::MatchCase>> = metamodelica::nil();
        for mut c in (inCases).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ DAE::MatchCase { patterns: Deref @ metamodelica::ListNode::Cons { head: __esc_pat @ Deref @ DAE::Pattern::PAT_CALL { patterns: __esc_patl, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            pat = (*__esc_pat).clone();
            patl = (*__esc_patl).clone();
            if allPatternsWild(metamodelica::AsArg::as_arg(&patl)) {
                assign_variant_field!(pat => DAE::Pattern::PAT_CALL; knownSingleton = true);
                assign_field!(c.patterns = list![pat.clone()]);
            }
            c.clone()
        },
        _ => c.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => {
            inCases
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCases
}

fn removeWildPatternColumnsFromMatrix(
    mut inPatternMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>,
    mut inAcc: metamodelica::List<Option<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>>,
    mut inNumAcc: i32,
) -> Result<(
    metamodelica::List<Option<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>>,
    i32,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inPatternMatrix, inAcc, inNumAcc)) {
            (Deref @ metamodelica::ListNode::Nil, acc, numAcc) => {
                return Ok((acc.clone().reverse(), numAcc.clone()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: pats, tail: patternMatrix }, acc, numAcc) => {
                let mut alwaysMatch: bool;
                let mut optPats: Option<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>;
                let mut acc = (*acc).clone();
                let mut numAcc = (*numAcc).clone();
                alwaysMatch = allPatternsAlwaysMatch(List::stripLast(pats.clone())?);
                optPats = if (alwaysMatch) {None} else {Some(pats.clone())};
                numAcc = if (alwaysMatch) {numAcc.clone()} else {numAcc.clone() + 1};
                { (inPatternMatrix, inAcc, inNumAcc) = (patternMatrix.clone(), metamodelica::cons(optPats, acc.clone()), numAcc.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn findPatternToConvertToSwitch(
    mut inPatternMatrix: &metamodelica::List<Option<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>>,
    mut index: i32,
    mut numPatternsInMatrix: i32,
    mut info: &SourceInfo,
) -> Result<(i32, metamodelica::Ref<DAE::Type>, i32)> {
    let mut tpl: (i32, metamodelica::Ref<DAE::Type>, i32);
    tpl = 'mc: {
        let __mc_input = &**inPatternMatrix;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Some(pats), tail: _ } => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut extraarg: i32;
                    (ty, extraarg) = findPatternToConvertToSwitch2(pats.clone(), metamodelica::nil(), DAE::T_UNKNOWN_DEFAULT().clone(), true, numPatternsInMatrix)?;
                    Ok((index, ty.clone(), extraarg))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: patternMatrix } => {
                    Ok(findPatternToConvertToSwitch(metamodelica::AsArg::as_arg(&patternMatrix), index + 1, numPatternsInMatrix, info)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(tpl)
}

fn findPatternToConvertToSwitch2(
    mut ipats: metamodelica::List<metamodelica::Ref<DAE::Pattern>>,
    mut ixs: metamodelica::List<i32>,
    mut ity: metamodelica::Ref<DAE::Type>,
    mut allSubPatternsMatch: bool,
    mut numPatternsInMatrix: i32,
) -> Result<(metamodelica::Ref<DAE::Type>, i32)> {
    let mut outTy: metamodelica::Ref<DAE::Type>;
    let mut extraarg: i32;
    (outTy, extraarg) = (::match_deref::match_deref! { match &((ipats, ity.clone(), allSubPatternsMatch, numPatternsInMatrix)) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Pattern::PAT_CONSTANT { exp: Deref @ DAE::Exp::SCONST { string: r#str }, .. }, tail: pats }, _, _, _) => {
            let mut ix: i32;
            let mut ty: metamodelica::Ref<DAE::Type>;
            ix = stringHashDjb2Mod(&r#str, 65536);
            let false = (listMember(ix, ixs.clone())) else { return Err("pattern mismatch") };
            (ty, extraarg) = findPatternToConvertToSwitch2(pats.clone(), metamodelica::cons(ix, ixs.clone()), DAE::T_STRING_DEFAULT().clone(), allSubPatternsMatch, numPatternsInMatrix)?;
            (ty, extraarg)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Pattern::PAT_CALL { index: ix, patterns: subpats, .. }, tail: pats }, _, _, _) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let false = (listMember(ix.clone(), ixs.clone())) else { return Err("pattern mismatch") };
            (ty, extraarg) = findPatternToConvertToSwitch2(pats.clone(), metamodelica::cons(ix.clone(), ixs.clone()), DAE::T_METATYPE_DEFAULT().clone(), allSubPatternsMatch && allPatternsAlwaysMatch(subpats.clone()), numPatternsInMatrix)?;
            (ty, extraarg)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Pattern::PAT_CONSTANT { exp: Deref @ DAE::Exp::ICONST { integer: ix }, .. }, tail: pats }, _, _, _) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let false = (listMember(ix.clone(), ixs.clone())) else { return Err("pattern mismatch") };
            (ty, extraarg) = findPatternToConvertToSwitch2(pats.clone(), metamodelica::cons(ix.clone(), ixs.clone()), DAE::T_INTEGER_DEFAULT().clone(), allSubPatternsMatch, numPatternsInMatrix)?;
            (ty, extraarg)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Pattern::PAT_CONSTANT { exp: Deref @ DAE::Exp::ENUM_LITERAL { index: ix, .. }, .. }, tail: pats }, _, _, _) if (!(listMember(ix.clone(), ixs.clone()))) => {
            findPatternToConvertToSwitch2(pats.clone(), metamodelica::cons(ix.clone(), ixs.clone()), DAE::T_ENUMERATION_DEFAULT().clone(), allSubPatternsMatch, numPatternsInMatrix)?
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ DAE::Type::T_STRING { .. }, _, _) => {
            let mut ix: i32;
            let true = (((ixs).len() as i32) > 11) else { return Err("pattern mismatch") };
            ix = findMinMod(ixs.clone(), 1)?;
            (DAE::T_STRING_DEFAULT().clone(), ix)
        },
        (Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ DAE::Type::T_STRING { .. }, _, 1) => {
            let mut ix: i32;
            let true = (((ixs).len() as i32) > 11) else { return Err("pattern mismatch") };
            ix = findMinMod(ixs.clone(), 1)?;
            (DAE::T_STRING_DEFAULT().clone(), ix)
        },
        (Deref @ metamodelica::ListNode::Nil, _, _, _) => {
            (ity, 0)
        },
        (Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, _, true, 1) => {
            (ity, 0)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outTy, extraarg))
}

fn findMinMod(mut inIxs: metamodelica::List<i32>, mut inMod: i32) -> Result<i32> {
    let mut outMod: i32;
    outMod = 'mc: {
        let __mc_input = (inIxs.clone(), inMod);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ixs, r#mod) => {
                    let mut ixs = (*ixs).clone();
                    ixs = List::map1(ixs.clone(), &fnptr!(intMod, i32, i32), r#mod.clone())?;
                    ixs = List::sort(ixs.clone(), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    ::match_deref::match_deref! { match &(List::sortedDuplicates(ixs.clone(), &fnptr!(intEq, i32, i32))?) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(r#mod.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (inMod < 65536) else { return Err("pattern mismatch") };
                    Ok(findMinMod(inIxs.clone(), inMod * 2)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMod)
}

fn filterUnusedPatterns(
    mut inputs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inAliases: metamodelica::List<metamodelica::List<ArcStr>>,
    mut inCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut info: &SourceInfo,
    mut emitNotifications: bool,
) -> (
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::List<ArcStr>>,
    metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
) {
    let mut outInputs: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut outAliases: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
    (outInputs, outAliases, outCases) = 'mc: {
        let __mc_input = inCases.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cases => {
                    let mut patternMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>;
                    let mut cases = (*cases).clone();
                    let mut outAliases: metamodelica::List<metamodelica::List<ArcStr>> = outAliases.clone();
                    let mut outInputs: metamodelica::List<metamodelica::Ref<DAE::Exp>> = outInputs.clone();
                    patternMatrix = List::transposeList(List::map(cases.clone(), &move |__a0: metamodelica::Ref<DAE::MatchCase>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getCasePatterns(&__a0)) })?)?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(filterUnusedPatterns2(&inputs, &inAliases, &patternMatrix, false, info, emitNotifications, &(metamodelica::nil()), &(metamodelica::nil()), &(metamodelica::nil()))) {
                        (true, __pa0, __pa1, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    outInputs = metamodelica::Own::own(__pa0);
                    outAliases = metamodelica::Own::own(__pa1);
                    patternMatrix = metamodelica::Own::own(__pa2);
                    patternMatrix = List::transposeList(patternMatrix.clone())?;
                    cases = setCasePatternsCheckZero(cases.clone(), patternMatrix.clone())?;
                    Ok(((outInputs.clone(), outAliases.clone(), cases.clone()), outAliases.clone(), outInputs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outAliases = __wb0;
            outInputs = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inputs.clone(), inAliases.clone(), inCases.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outInputs, outAliases, outCases)
}

fn setCasePatternsCheckZero(
    mut inCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut patternMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::MatchCase>>> {
    let mut outCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
    outCases = (::match_deref::match_deref! { match &((inCases.clone(), patternMatrix.clone())) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => inCases,
        (_, Deref @ metamodelica::ListNode::Nil) => List::map1(inCases, &move |__a0: metamodelica::Ref<DAE::MatchCase>, __a1: metamodelica::List<metamodelica::Ref<DAE::Pattern>>| -> metamodelica::Result<_> { ::std::result::Result::Ok(setCasePatterns(&__a0, __a1)) }, metamodelica::nil())?,
        _ => List::threadMap(inCases, patternMatrix, &move |__a0: metamodelica::Ref<DAE::MatchCase>, __a1: metamodelica::List<metamodelica::Ref<DAE::Pattern>>| -> metamodelica::Result<_> { ::std::result::Result::Ok(setCasePatterns(&__a0, __a1)) })?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCases)
}

fn filterUnusedPatterns2(
    mut inInputs: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inAliases: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut inPatternMatrix: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>,
    mut change: bool,
    mut info: &SourceInfo,
    mut emitNotifications: bool,
    mut inputsAcc: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut aliasesAcc: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut patternMatrixAcc: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>,
) -> (
    bool,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::List<ArcStr>>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>,
) {
    let mut outChange: bool = false;
    let mut outInputs: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut outAliases: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outPatternMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Pattern>>> =
        metamodelica::nil();
    (outChange, outInputs, outAliases, outPatternMatrix) = 'mc: {
        let __mc_input = (&**inInputs, &**inAliases, &**inPatternMatrix, change);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, true) => {
                    Ok((true, inputsAcc.clone().reverse(), aliasesAcc.clone().reverse(), patternMatrixAcc.clone().reverse()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: inputs }, Deref @ metamodelica::ListNode::Cons { head: _, tail: aliases }, Deref @ metamodelica::ListNode::Cons { head: pats, tail: patternMatrix }, _) => {
                    let mut outAliases: metamodelica::List<metamodelica::List<ArcStr>> = outAliases.clone();
                    let mut outChange: bool = outChange.clone();
                    let mut outInputs: metamodelica::List<metamodelica::Ref<DAE::Exp>> = outInputs.clone();
                    let mut outPatternMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Pattern>>> = outPatternMatrix.clone();
                    ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(e.clone(), &fnptr!(Expression::hasNoSideEffects, metamodelica::Ref<DAE::Exp>, bool), true)?) {
                        (_, true) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    let true = (allPatternsWild(metamodelica::AsArg::as_arg(&pats))) else { return Err("pattern mismatch") };
                    if emitNotifications && Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())? {
                        Error::addSourceMessage(&(Error::META_MATCH_UNUSED_INPUT.clone()), list![ExpressionBasics::printExpStr(e.clone())?], info)?;
                    }
                    (outChange, outInputs, outAliases, outPatternMatrix) = filterUnusedPatterns2(metamodelica::AsArg::as_arg(&inputs), metamodelica::AsArg::as_arg(&aliases), metamodelica::AsArg::as_arg(&patternMatrix), true, info, emitNotifications, inputsAcc, aliasesAcc, patternMatrixAcc);
                    Ok(((outChange, outInputs.clone(), outAliases.clone(), outPatternMatrix.clone()), outAliases.clone(), outChange.clone(), outInputs.clone(), outPatternMatrix.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outAliases = __wb0;
            outChange = __wb1;
            outInputs = __wb2;
            outPatternMatrix = __wb3;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: inputs }, Deref @ metamodelica::ListNode::Cons { head: alias, tail: aliases }, Deref @ metamodelica::ListNode::Cons { head: pats, tail: patternMatrix }, _) => {
                    let mut outAliases: metamodelica::List<metamodelica::List<ArcStr>> = outAliases.clone();
                    let mut outChange: bool = outChange.clone();
                    let mut outInputs: metamodelica::List<metamodelica::Ref<DAE::Exp>> = outInputs.clone();
                    let mut outPatternMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Pattern>>> = outPatternMatrix.clone();
                    if emitNotifications && Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())? && Expression::isCref(metamodelica::AsArg::as_arg(&e)) && allPatternsAlwaysMatch(pats.clone()) && !(allPatternsWild(metamodelica::AsArg::as_arg(&pats))) {
                        Error::addSourceMessage(&(Error::META_PATTERN_AS_ONLY.clone()), list![ExpressionBasics::printExpStr(e.clone())?, ExpressionBasics::printExpStr(e.clone())?], info)?;
                    }
                    (outChange, outInputs, outAliases, outPatternMatrix) = filterUnusedPatterns2(metamodelica::AsArg::as_arg(&inputs), metamodelica::AsArg::as_arg(&aliases), metamodelica::AsArg::as_arg(&patternMatrix), change, info, emitNotifications, &(metamodelica::cons(e.clone(), inputsAcc.clone())), &(metamodelica::cons(alias.clone(), aliasesAcc.clone())), &(metamodelica::cons(pats.clone(), patternMatrixAcc.clone())));
                    Ok(((outChange, outInputs.clone(), outAliases.clone(), outPatternMatrix.clone()), outAliases.clone(), outChange.clone(), outInputs.clone(), outPatternMatrix.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outAliases = __wb0;
            outChange = __wb1;
            outInputs = __wb2;
            outPatternMatrix = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((false, metamodelica::nil(), metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChange, outInputs, outAliases, outPatternMatrix)
}

fn getUsedLocalCrefs(
    mut skipFilterUnusedAsBindings: bool,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut hashSize: i32,
) -> Result<(
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
)> {
    let mut ht: (
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
    ht = (::match_deref::match_deref! { match &((skipFilterUnusedAsBindings, exp.clone())) {
        (true, _) => {
            (_, ht) = Expression::traverseExpBottomUp(exp, &addLocalCref, HashTableStringToPath::emptyHashTableSized(hashSize))?;
            ht
        },
        (false, Deref @ DAE::Exp::MATCHEXPRESSION { cases, .. }) => {
            (_, ht) = Expression::traverseCases(metamodelica::AsArg::as_arg(&cases), (std::sync::Arc::new(addLocalCref) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<(ArcStr, i32)>>, (i32, i32, metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<(ArcStr, i32)>>, (i32, i32, metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>)))> + 'static>), HashTableStringToPath::emptyHashTableSized(hashSize))?;
            ht
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(ht)
}

fn filterUnusedAsBindings(
    mut inCases: &metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut ht: &(
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
) -> Result<metamodelica::List<metamodelica::Ref<DAE::MatchCase>>> {
    let mut outCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
    outCases = (::match_deref::match_deref! { match inCases {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns, patternGuard: guardPattern, localDecls, body, result, resultInfo, jump, info }, tail: cases } => {
            let mut patterns = (*patterns).clone();
            let mut cases = (*cases).clone();
            (patterns, _) = traversePatternList(metamodelica::AsArg::as_arg(&patterns), &removePatternAsBinding, (ht.clone(), info.clone()))?;
            cases = filterUnusedAsBindings(metamodelica::AsArg::as_arg(&cases), ht)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: guardPattern.clone(), localDecls: localDecls.clone(), body: body.clone(), result: result.clone(), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info.clone() }), cases.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCases)
}

fn removePatternAsBinding(
    mut inPat: metamodelica::Ref<DAE::Pattern>,
    mut inTpl: (
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
        SourceInfo,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Pattern>,
    (
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
        SourceInfo,
    ),
)> {
    let mut pat: metamodelica::Ref<DAE::Pattern> = inPat.clone();
    let mut outTpl: (
        (
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
        ),
        SourceInfo,
    ) = inTpl.clone();
    pat = 'mc: {
        let __mc_input = (pat.clone(), inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Pattern::PAT_AS { id, pat, .. }, (ht, info)) => {
                    let true = (BaseHashTable::hasKey(id.clone(), &(ht.clone()))?) else { return Err("pattern mismatch") };
                    Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), &(Error::META_UNUSED_AS_BINDING.clone()), list![id.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(pat.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { id, pat }, (ht, _)) => {
                    let true = (BaseHashTable::hasKey(id.clone(), &(ht.clone()))?) else { return Err("pattern mismatch") };
                    Ok(pat.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut pat: metamodelica::Ref<DAE::Pattern> = pat.clone();
                    (pat, _) = simplifyPattern(inPat.clone(), 1)?;
                    Ok((pat.clone(), pat.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            pat = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((pat, outTpl))
}

fn addLocalCref(
    mut inExp: metamodelica::Ref<DAE::Exp>,
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
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
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
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
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
    (outExp, outHt) = (::match_deref::match_deref! { match &(inExp.clone()) {
        exp @ Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut ht = inHt.clone();
            ht = addLocalCrefHelper(cr.clone(), ht)?;
            (exp.clone(), ht)
        },
        exp @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, attr: Deref @ DAE::CallAttributes { builtin: false, .. }, .. } => {
            let mut ht = inHt.clone();
            ht = BaseHashTable::add((name.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") })), ht)?;
            (exp.clone(), ht)
        },
        exp @ Deref @ DAE::Exp::PATTERN { pattern: pat } => {
            let mut ht = inHt.clone();
            (_, ht) = traversePattern(pat.clone(), &fnptr!(addPatternAsBindings, metamodelica::Ref<DAE::Pattern>, (metamodelica::Array<metamodelica::List<(ArcStr, i32)>>, (i32, i32, metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>))), ht)?;
            (exp.clone(), ht)
        },
        exp @ Deref @ DAE::Exp::MATCHEXPRESSION { cases, .. } => {
            let mut ht = inHt.clone();
            ht = addCasesLocalCref(cases.clone(), ht)?;
            (exp.clone(), ht)
        },
        _ => {
            (inExp, inHt)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outHt))
}

fn addLocalCrefHelper(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut iht: (
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
) -> Result<(
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
)> {
    let mut ht: (
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
    ht = (::match_deref::match_deref! { match &((cr, iht.clone())) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, subscriptLst: subs, .. }, __esc_ht) => {
            ht = (*__esc_ht).clone();
            ht = addLocalCrefSubs(subs.clone(), ht.clone())?;
            ht = BaseHashTable::add((name.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") })), ht.clone())?;
            ht.clone()
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, subscriptLst: subs, componentRef: cr2, .. }, __esc_ht) => {
            ht = (*__esc_ht).clone();
            ht = addLocalCrefSubs(subs.clone(), ht.clone())?;
            ht = BaseHashTable::add((name.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") })), ht.clone())?;
            addLocalCrefHelper(cr2.clone(), ht.clone())?
        },
        _ => {
            iht
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ht)
}

fn addLocalCrefSubs(
    mut isubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut iht: (
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
) -> Result<(
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
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((isubs, iht.clone())) {
            (Deref @ metamodelica::ListNode::Nil, ht) => {
                return Ok(ht.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp }, tail: subs }, ht) => {
                let mut ht = (*ht).clone();
                (_, ht) = Expression::traverseExpBottomUp(exp.clone(), &addLocalCref, ht.clone())?;
                { (isubs, iht) = (subs.clone(), ht.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp }, tail: subs }, ht) => {
                let mut ht = (*ht).clone();
                (_, ht) = Expression::traverseExpBottomUp(exp.clone(), &addLocalCref, ht.clone())?;
                { (isubs, iht) = (subs.clone(), ht.clone()); continue '__tco; }
            },
            _ => {
                return Ok(iht)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn checkDefUse(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        metamodelica::Ref<AvlSetString::Tree>,
        metamodelica::Ref<AvlSetString::Tree>,
        SourceInfo,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::Ref<AvlSetString::Tree>,
        metamodelica::Ref<AvlSetString::Tree>,
        SourceInfo,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut outTpl: (
        metamodelica::Ref<AvlSetString::Tree>,
        metamodelica::Ref<AvlSetString::Tree>,
        SourceInfo,
    );
    (outExp, outTpl) = 'mc: {
        let __mc_input = (&*inExp, inTpl.clone());
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty }, extra @ (localsTree, useTree, info)) => {
                    let mut name: ArcStr;
                    let mut outExp: metamodelica::Ref<DAE::Exp> = outExp.clone();
                    name = ComponentReferenceBasics::crefFirstIdent(metamodelica::AsArg::as_arg(&cr))?;
                    if AvlSetString::hasKey(localsTree.clone(), name.clone())? && !(AvlSetString::hasKey(useTree.clone(), name.clone())?) {
                        Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), &(Error::META_UNUSED_ASSIGNMENT.clone()), list![name.clone()], metamodelica::AsArg::as_arg(&info))?;
                        outExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: openmodelica_frontend_types::DAE::ComponentRef::interned_WILD(), ty: ty.clone() });
                    } else {
                        outExp = inExp.clone();
                    }
                    Ok(((outExp.clone(), extra.clone()), outExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outExp = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PATTERN { pattern: pat }, extra) => {
                    let mut pat = (*pat).clone();
                    let mut extra = (*extra).clone();
                    (pat, extra) = traversePattern(pat.clone(), &checkDefUsePattern, extra.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::PATTERN { pattern: pat.clone() }), extra.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTpl)
}

fn checkDefUsePattern(
    mut inPat: metamodelica::Ref<DAE::Pattern>,
    mut inTpl: (
        metamodelica::Ref<AvlSetString::Tree>,
        metamodelica::Ref<AvlSetString::Tree>,
        SourceInfo,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Pattern>,
    (
        metamodelica::Ref<AvlSetString::Tree>,
        metamodelica::Ref<AvlSetString::Tree>,
        SourceInfo,
    ),
)> {
    let mut outPat: metamodelica::Ref<DAE::Pattern>;
    let mut outTpl: (
        metamodelica::Ref<AvlSetString::Tree>,
        metamodelica::Ref<AvlSetString::Tree>,
        SourceInfo,
    ) = inTpl.clone();
    outPat = (::match_deref::match_deref! { match &((inPat.clone(), inTpl)) {
        (Deref @ DAE::Pattern::PAT_AS { id: name, pat, .. }, (localsTree, useTree, info)) => {
            let mut pat = (*pat).clone();
            if AvlSetString::hasKey(localsTree.clone(), name.clone())? && !(AvlSetString::hasKey(useTree.clone(), name.clone())?) {
                Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), &(Error::META_UNUSED_AS_BINDING.clone()), list![name.clone()], metamodelica::AsArg::as_arg(&info))?;
            } else {
                pat = inPat;
            }
            pat.clone()
        },
        (Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { id: name, pat }, (localsTree, useTree, info)) => {
            let mut pat = (*pat).clone();
            if AvlSetString::hasKey(localsTree.clone(), name.clone())? && !(AvlSetString::hasKey(useTree.clone(), name.clone())?) {
                Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), &(Error::META_UNUSED_AS_BINDING.clone()), list![name.clone()], metamodelica::AsArg::as_arg(&info))?;
            } else {
                pat = inPat;
            }
            pat.clone()
        },
        _ => {
            let mut pat: metamodelica::Ref<DAE::Pattern>;
            (pat, _) = simplifyPattern(inPat, 1)?;
            pat
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outPat, outTpl))
}

fn useLocalCref(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<AvlSetString::Tree>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTree: metamodelica::Ref<AvlSetString::Tree>;
    (outExp, outTree) = (::match_deref::match_deref! { match &(inExp.clone()) {
        exp @ Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut tree = inTree.clone();
            tree = useLocalCrefHelper(metamodelica::AsArg::as_arg(&cr), tree)?;
            (exp.clone(), tree)
        },
        exp @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, attr: Deref @ DAE::CallAttributes { builtin: false, .. }, .. } => {
            let mut tree = inTree.clone();
            tree = AvlSetString::add(tree, metamodelica::AsArg::as_arg(&name))?;
            (exp.clone(), tree)
        },
        exp @ Deref @ DAE::Exp::PATTERN { pattern: pat } => {
            let mut tree = inTree.clone();
            (_, tree) = traversePattern(pat.clone(), &fnptr!(usePatternAsBindings, metamodelica::Ref<DAE::Pattern>, metamodelica::Ref<AvlSetString::Tree>), tree)?;
            (exp.clone(), tree)
        },
        exp @ Deref @ DAE::Exp::MATCHEXPRESSION { cases, .. } => {
            let mut tree = inTree.clone();
            tree = useCasesLocalCref(metamodelica::AsArg::as_arg(&cases), tree)?;
            (exp.clone(), tree)
        },
        _ => {
            (inExp, inTree)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTree))
}

fn useLocalCrefHelper<'__b>(
    mut cr: &'__b metamodelica::Ref<DAE::ComponentRef>,
    mut inTree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<AvlSetString::Tree>> {
    let mut tree: metamodelica::Ref<AvlSetString::Tree>;
    tree = (match &**cr {
        DAE::ComponentRef::CREF_IDENT {
            ident: name,
            subscriptLst: subs,
            ..
        } => {
            tree = useLocalCrefSubs(subs, inTree)?;
            AvlSetString::add(tree, name)?
        }
        DAE::ComponentRef::CREF_QUAL {
            ident: name,
            subscriptLst: subs,
            componentRef: cr2,
            ..
        } => {
            tree = useLocalCrefSubs(subs, inTree)?;
            tree = AvlSetString::add(tree, name)?;
            useLocalCrefHelper(cr2, tree)?
        }
        _ => inTree,
    });
    Ok(tree)
}

fn useLocalCrefSubs<'__b>(
    mut isubs: &'__b metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inTree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<AvlSetString::Tree>> {
    let mut tree: metamodelica::Ref<AvlSetString::Tree>;
    tree = (::match_deref::match_deref! { match isubs {
        Deref @ metamodelica::ListNode::Nil => {
            inTree
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp }, tail: subs } => {
            (_, tree) = Expression::traverseExpBottomUp(exp.clone(), &useLocalCref, inTree)?;
            tree = useLocalCrefSubs(subs, tree)?;
            tree
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp }, tail: subs } => {
            (_, tree) = Expression::traverseExpBottomUp(exp.clone(), &useLocalCref, inTree)?;
            tree = useLocalCrefSubs(subs, tree)?;
            tree
        },
        _ => {
            inTree
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tree)
}

fn usePatternAsBindings(
    mut inPat: metamodelica::Ref<DAE::Pattern>,
    mut inTree: metamodelica::Ref<AvlSetString::Tree>,
) -> (metamodelica::Ref<DAE::Pattern>, metamodelica::Ref<AvlSetString::Tree>) {
    let mut outPat: metamodelica::Ref<DAE::Pattern> = inPat.clone();
    let mut outTree: metamodelica::Ref<AvlSetString::Tree>;
    outTree = 'mc: {
        let __mc_input = &*inPat;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_AS { .. } => {
                    Ok(AvlSetString::add(inTree.clone(), var_field!((*inPat).id, DAE::Pattern::PAT_AS))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { .. } => {
                    Ok(AvlSetString::add(inTree.clone(), var_field!((*inPat).id, DAE::Pattern::PAT_AS_FUNC_PTR))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inTree.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outPat, outTree)
}

fn useCasesLocalCref<'__b>(
    mut icases: &'__b metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut inTree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<AvlSetString::Tree>> {
    let mut tree: metamodelica::Ref<AvlSetString::Tree>;
    tree = (::match_deref::match_deref! { match icases {
        Deref @ metamodelica::ListNode::Nil => {
            inTree
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns: pats, .. }, tail: cases } => {
            (_, tree) = traversePatternList(metamodelica::AsArg::as_arg(&pats), &fnptr!(usePatternAsBindings, metamodelica::Ref<DAE::Pattern>, metamodelica::Ref<AvlSetString::Tree>), inTree)?;
            tree = useCasesLocalCref(cases, tree)?;
            tree
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tree)
}

fn addCasesLocalCref(
    mut icases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut iht: (
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
) -> Result<(
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
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((icases, iht)) {
            (Deref @ metamodelica::ListNode::Nil, ht) => {
                return Ok(ht.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns: pats, .. }, tail: cases }, ht) => {
                let mut ht = (*ht).clone();
                (_, ht) = traversePatternList(metamodelica::AsArg::as_arg(&pats), &fnptr!(addPatternAsBindings, metamodelica::Ref<DAE::Pattern>, (metamodelica::Array<metamodelica::List<(ArcStr, i32)>>, (i32, i32, metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>))), ht.clone())?;
                { (icases, iht) = (cases.clone(), ht.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn simplifyPattern<A: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inPat: metamodelica::Ref<DAE::Pattern>,
    mut extra: A,
) -> Result<(metamodelica::Ref<DAE::Pattern>, A)> {
    let mut outPat: metamodelica::Ref<DAE::Pattern>;
    let mut outExtra: A = extra;
    outPat = (match &*inPat {
        DAE::Pattern::PAT_CALL_NAMED {
            name,
            patterns: namedPatterns,
        } => {
            let mut namedPatterns = (*namedPatterns).clone();
            namedPatterns = List::filterOnTrue(namedPatterns.clone(), (std::sync::Arc::new(move |__a0: (metamodelica::Ref<DAE::Pattern>, ArcStr, metamodelica::Ref<DAE::Type>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(filterEmptyPattern(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::Pattern>, ArcStr, metamodelica::Ref<DAE::Type>)) -> Result<bool> + 'static>))?;
            if ((namedPatterns).is_empty()) {
                openmodelica_frontend_types::DAE::Pattern::interned_PAT_WILD()
            } else {
                metamodelica::Ref::new(DAE::Pattern::PAT_CALL_NAMED {
                    name: name.clone(),
                    patterns: namedPatterns.clone(),
                })
            }
        }
        DAE::Pattern::PAT_CALL_TUPLE { patterns } => {
            if (allPatternsWild(metamodelica::AsArg::as_arg(&patterns))) {
                openmodelica_frontend_types::DAE::Pattern::interned_PAT_WILD()
            } else {
                inPat
            }
        }
        DAE::Pattern::PAT_META_TUPLE { patterns } => {
            if (allPatternsWild(metamodelica::AsArg::as_arg(&patterns))) {
                openmodelica_frontend_types::DAE::Pattern::interned_PAT_WILD()
            } else {
                inPat
            }
        }
        _ => inPat,
    });
    Ok((outPat, outExtra))
}

fn addPatternAsBindings(
    mut inPat: metamodelica::Ref<DAE::Pattern>,
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
) -> (
    metamodelica::Ref<DAE::Pattern>,
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
) {
    let mut pat: metamodelica::Ref<DAE::Pattern> = inPat.clone();
    let mut ht: (
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
    ht = 'mc: {
        let __mc_input = inPat;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_AS { id, .. } => {
                    Ok(BaseHashTable::add((id.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") })), ht.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { id, .. } => {
                    Ok(BaseHashTable::add((id.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") })), ht.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (pat, ht)
}

pub(crate) fn traversePatternList<TypeA: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inPatterns: &metamodelica::List<metamodelica::Ref<DAE::Pattern>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Pattern>, TypeA) -> Result<(metamodelica::Ref<DAE::Pattern>, TypeA)>,
    mut inExtra: TypeA,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Pattern>>, TypeA)> {
    pub type Func<TypeA: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Pattern>, TypeA) -> Result<(metamodelica::Ref<DAE::Pattern>, TypeA)>
            + 'static,
    >;

    let mut outPatterns: metamodelica::List<metamodelica::Ref<DAE::Pattern>> = metamodelica::nil();
    let mut extra: TypeA = inExtra;
    let mut p: metamodelica::Ref<DAE::Pattern>;
    for mut pat in &**inPatterns {
        (p, extra) = traversePattern(pat.clone(), func, extra)?;
        outPatterns = metamodelica::cons(p, outPatterns);
    }
    outPatterns = Dangerous::listReverseInPlace(outPatterns);
    Ok((outPatterns, extra))
}

pub(crate) fn traversePattern<TypeA: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inPattern: metamodelica::Ref<DAE::Pattern>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Pattern>, TypeA) -> Result<(metamodelica::Ref<DAE::Pattern>, TypeA)>,
    mut inExtra: TypeA,
) -> Result<(metamodelica::Ref<DAE::Pattern>, TypeA)> {
    pub type Func<TypeA: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Pattern>, TypeA) -> Result<(metamodelica::Ref<DAE::Pattern>, TypeA)>
            + 'static,
    >;

    let mut outPattern: metamodelica::Ref<DAE::Pattern>;
    let mut extra: TypeA = inExtra;
    (outPattern, extra) = (::match_deref::match_deref! { match &(inPattern) {
        Deref @ DAE::Pattern::PAT_AS { id, ty, attr, pat: pat2 } => {
            let mut pat: metamodelica::Ref<DAE::Pattern>;
            let mut pat2 = (*pat2).clone();
            (pat2, extra) = traversePattern(pat2.clone(), func, extra)?;
            pat = metamodelica::Ref::new(DAE::Pattern::PAT_AS { id: id.clone(), ty: ty.clone(), attr: attr.clone(), pat: pat2.clone() });
            (pat, extra) = func(pat, extra)?;
            (pat, extra)
        },
        Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { id, pat: pat2 } => {
            let mut pat: metamodelica::Ref<DAE::Pattern>;
            let mut pat2 = (*pat2).clone();
            (pat2, extra) = traversePattern(pat2.clone(), func, extra)?;
            pat = metamodelica::Ref::new(DAE::Pattern::PAT_AS_FUNC_PTR { id: id.clone(), pat: pat2.clone() });
            (pat, extra) = func(pat, extra)?;
            (pat, extra)
        },
        Deref @ DAE::Pattern::PAT_CALL { name, index, patterns: pats, fields: fieldVars, typeVars, knownSingleton } => {
            let mut pat: metamodelica::Ref<DAE::Pattern>;
            let mut pats = (*pats).clone();
            (pats, extra) = traversePatternList(metamodelica::AsArg::as_arg(&pats), func, extra)?;
            pat = metamodelica::Ref::new(DAE::Pattern::PAT_CALL { name: name.clone(), index: index.clone(), patterns: pats.clone(), fields: fieldVars.clone(), typeVars: typeVars.clone(), knownSingleton: knownSingleton.clone() });
            (pat, extra) = func(pat, extra)?;
            (pat, extra)
        },
        Deref @ DAE::Pattern::PAT_CALL_NAMED { name, patterns: namedpats } => {
            let mut pat: metamodelica::Ref<DAE::Pattern>;
            let mut pats: metamodelica::List<metamodelica::Ref<DAE::Pattern>>;
            let mut fields: metamodelica::List<ArcStr>;
            let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut namedpats = (*namedpats).clone();
            (pats, fields, types) = List::unzip3(namedpats.clone());
            (pats, extra) = traversePatternList(&pats, func, extra)?;
            namedpats = List::zip3(pats, fields, types);
            pat = metamodelica::Ref::new(DAE::Pattern::PAT_CALL_NAMED { name: name.clone(), patterns: namedpats.clone() });
            (pat, extra) = func(pat, extra)?;
            (pat, extra)
        },
        Deref @ DAE::Pattern::PAT_CALL_TUPLE { patterns: pats } => {
            let mut pat: metamodelica::Ref<DAE::Pattern>;
            let mut pats = (*pats).clone();
            (pats, extra) = traversePatternList(metamodelica::AsArg::as_arg(&pats), func, extra)?;
            pat = metamodelica::Ref::new(DAE::Pattern::PAT_CALL_TUPLE { patterns: pats.clone() });
            (pat, extra) = func(pat, extra)?;
            (pat, extra)
        },
        Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: pats } => {
            let mut pat: metamodelica::Ref<DAE::Pattern>;
            let mut pats = (*pats).clone();
            (pats, extra) = traversePatternList(metamodelica::AsArg::as_arg(&pats), func, extra)?;
            pat = metamodelica::Ref::new(DAE::Pattern::PAT_META_TUPLE { patterns: pats.clone() });
            (pat, extra) = func(pat, extra)?;
            (pat, extra)
        },
        Deref @ DAE::Pattern::PAT_CONS { head: pat1, tail: pat2 } => {
            let mut pat: metamodelica::Ref<DAE::Pattern>;
            let mut pat1 = (*pat1).clone();
            let mut pat2 = (*pat2).clone();
            (pat1, extra) = traversePattern(pat1.clone(), func, extra)?;
            (pat2, extra) = traversePattern(pat2.clone(), func, extra)?;
            pat = metamodelica::Ref::new(DAE::Pattern::PAT_CONS { head: pat1.clone(), tail: pat2.clone() });
            (pat, extra) = func(pat, extra)?;
            (pat, extra)
        },
        pat @ Deref @ DAE::Pattern::PAT_CONSTANT { .. } => {
            let mut pat = (*pat).clone();
            (pat, extra) = func(pat.clone(), extra)?;
            (pat.clone(), extra)
        },
        Deref @ DAE::Pattern::PAT_SOME { pat: pat1 } => {
            let mut pat: metamodelica::Ref<DAE::Pattern>;
            let mut pat1 = (*pat1).clone();
            (pat1, extra) = traversePattern(pat1.clone(), func, extra)?;
            pat = metamodelica::Ref::new(DAE::Pattern::PAT_SOME { pat: pat1.clone() });
            (pat, extra) = func(pat, extra)?;
            (pat, extra)
        },
        pat @ Deref @ DAE::Pattern::PAT_WILD { .. } => {
            let mut pat = (*pat).clone();
            (pat, extra) = func(pat.clone(), extra)?;
            (pat.clone(), extra)
        },
        pat => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Patternm.traversePattern failed: ")); __mm_s.push_str(&*ExpressionDump::patternStr(metamodelica::AsArg::as_arg(&pat))?); ArcStr::from(__mm_s) };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outPattern, extra))
}

fn filterUnusedDecls(
    mut matchDecls: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut ht: &(
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
    mut iacc: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut iunusedHt: (
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
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
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
)> {
    let mut outDecls: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut outUnusedHt: (
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
    (outDecls, outUnusedHt) = 'mc: {
        let __mc_input = (&**matchDecls, iacc, iunusedHt);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, acc, unusedHt) => {
                    Ok((acc.clone().reverse(), unusedHt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, source: Deref @ DAE::ElementSource { info, .. }, .. }, tail: rest }, acc, unusedHt) => {
                    let mut acc = (*acc).clone();
                    let mut unusedHt = (*unusedHt).clone();
                    let false = (BaseHashTable::hasKey(name.clone(), ht)?) else { return Err("pattern mismatch") };
                    unusedHt = BaseHashTable::add((name.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") })), unusedHt.clone())?;
                    Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), &(Error::META_UNUSED_DECL.clone()), list![name.clone()], metamodelica::AsArg::as_arg(&info))?;
                    (acc, unusedHt) = filterUnusedDecls(metamodelica::AsArg::as_arg(&rest), ht, acc.clone(), unusedHt.clone())?;
                    Ok((acc.clone(), unusedHt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: el, tail: rest }, acc, unusedHt) => {
                    let mut acc = (*acc).clone();
                    let mut unusedHt = (*unusedHt).clone();
                    (acc, unusedHt) = filterUnusedDecls(metamodelica::AsArg::as_arg(&rest), ht, metamodelica::cons(el.clone(), acc.clone()), unusedHt.clone())?;
                    Ok((acc.clone(), unusedHt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outDecls, outUnusedHt))
}

fn caseDeadCodeElimination(
    mut matchType: Absyn::MatchType,
    mut cases: &metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut prevPatterns: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>,
    mut iacc: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut iter: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::MatchCase>>> {
    let mut outCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
    outCases = 'mc: {
        let __mc_input = (matchType, &**cases, iacc, iter);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, acc, false) => {
                    Ok(acc.clone().reverse())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, acc, true) => {
                    Ok(caseDeadCodeElimination(matchType, &(acc.clone().reverse()), &(metamodelica::nil()), metamodelica::nil(), false)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { body: Deref @ metamodelica::ListNode::Nil, result: None, info, .. }, tail: Deref @ metamodelica::ListNode::Nil }, acc, _) => {
                    Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), &(Error::META_DEAD_CODE.clone()), list![literal!("Last pattern is empty")], metamodelica::AsArg::as_arg(&info))?;
                    Ok(caseDeadCodeElimination(matchType, &(acc.clone().reverse()), &(metamodelica::nil()), metamodelica::nil(), false)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Absyn::MatchType::MATCHCONTINUE { .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns: pats, body: Deref @ metamodelica::ListNode::Nil, result: None, info, .. }, tail: rest }, acc, _) => {
                    let mut acc = (*acc).clone();
                    let true = (Flags::isSet(Flags::PATTERNM_DCE.clone())?) else { return Err("pattern mismatch") };
                    Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), &(Error::META_DEAD_CODE.clone()), list![literal!("Empty matchcontinue case")], metamodelica::AsArg::as_arg(&info))?;
                    acc = caseDeadCodeElimination(matchType.clone(), metamodelica::AsArg::as_arg(&rest), &(metamodelica::cons(pats.clone(), prevPatterns.clone())), acc.clone(), true)?;
                    Ok(acc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: case_ @ Deref @ DAE::MatchCase { patterns: pats, .. }, tail: rest }, acc, _) => {
                    Ok(caseDeadCodeElimination(matchType, metamodelica::AsArg::as_arg(&rest), &(metamodelica::cons(pats.clone(), prevPatterns.clone())), metamodelica::cons(case_.clone(), acc.clone()), iter)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCases)
}

/*
protected function findOverlappingPattern
  input list<DAE.Pattern> patterns;
  input list<DAE.MatchCase> prevCases;
  output SourceInfo info;
algorithm
  info := matchcontinue (patterns,prevCases)
    local
      list<DAE.Pattern> ps1,ps2;
    case (ps1,DAE.CASE(patterns=ps2,info=info)::_)
      algorithm
        true = patternListsDoOverlap(ps1,ps2); ???
      then info;
    case (ps1,_::prevCases) then findOverlappingPattern(ps1,prevCases);
  end matchcontinue;
end findOverlappingPattern;
*/
fn optimizeContinueJumps(
    mut matchType: Absyn::MatchType,
    mut cases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::MatchCase>>> {
    let mut outCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
    outCases = (match matchType {
        Absyn::MatchType::MATCH { .. } => cases,
        _ => optimizeContinueJumps2(&cases)?,
    });
    Ok(outCases)
}

fn optimizeContinueJumps2(
    mut icases: &metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::MatchCase>>> {
    let mut outCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
    outCases = (::match_deref::match_deref! { match icases {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: case_, tail: cases } => {
            let mut case_ = (*case_).clone();
            let mut cases = (*cases).clone();
            case_ = optimizeContinueJump(case_.clone(), metamodelica::AsArg::as_arg(&cases), 0)?;
            cases = optimizeContinueJumps2(metamodelica::AsArg::as_arg(&cases))?;
            metamodelica::cons(case_.clone(), cases.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCases)
}

fn optimizeContinueJump(
    mut case_: metamodelica::Ref<DAE::MatchCase>,
    mut icases: &metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut jump: i32,
) -> Result<metamodelica::Ref<DAE::MatchCase>> {
    let mut outCase: metamodelica::Ref<DAE::MatchCase>;
    outCase = 'mc: {
        let __mc_input = (case_, &**icases);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (case1, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(updateMatchCaseJump(case1.clone(), jump)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (case1 @ Deref @ DAE::MatchCase { patterns: ps1, .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns: ps2, .. }, tail: cases }) => {
                    let true = (patternListsDoNotOverlap(metamodelica::AsArg::as_arg(&ps1), metamodelica::AsArg::as_arg(&ps2))?) else { return Err("pattern mismatch") };
                    Ok(optimizeContinueJump(case1.clone(), metamodelica::AsArg::as_arg(&cases), jump + 1)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (case1, _) => {
                    Ok(updateMatchCaseJump(case1.clone(), jump)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCase)
}

fn updateMatchCaseJump(
    mut case_: metamodelica::Ref<DAE::MatchCase>,
    mut jump: i32,
) -> Result<metamodelica::Ref<DAE::MatchCase>> {
    let mut outCase: metamodelica::Ref<DAE::MatchCase>;
    outCase = (::match_deref::match_deref! { match &((case_.clone(), jump)) {
        (_, 0) => {
            case_
        },
        (Deref @ DAE::MatchCase { patterns, patternGuard: guardPattern, localDecls, body, result, resultInfo, jump: _, info }, _) => {
            metamodelica::Ref::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: guardPattern.clone(), localDecls: localDecls.clone(), body: body.clone(), result: result.clone(), resultInfo: resultInfo.clone(), jump: jump, info: info.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCase)
}

fn optimizeContinueToMatch(
    mut matchType: Absyn::MatchType,
    mut cases: &metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut info: &SourceInfo,
) -> Result<Absyn::MatchType> {
    let mut outMatchType: Absyn::MatchType;
    outMatchType = (match matchType {
        Absyn::MatchType::MATCH { .. } => openmodelica_ast::Absyn::MatchType::MATCH,
        _ => optimizeContinueToMatch2(cases, &(metamodelica::nil()), info),
    });
    if Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())? {
        checkMatchContinueSingleCaseToTry(outMatchType, cases, info)?;
    }
    Ok(outMatchType)
}

fn checkMatchContinueSingleCaseToTry(
    mut matchType: Absyn::MatchType,
    mut cases: &metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((matchType, &**cases)) {
        (Absyn::MatchType::MATCHCONTINUE { .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns: firstPats, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns: elsePats, .. }, tail: Deref @ metamodelica::ListNode::Nil } }) if (!(allPatternsWild(metamodelica::AsArg::as_arg(&firstPats))) && allPatternsWild(metamodelica::AsArg::as_arg(&elsePats))) => {
            Error::addSourceMessage(&(Error::MATCHCONTINUE_TO_TRY_OPTIMIZATION.clone()), metamodelica::nil(), info)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn optimizeContinueToMatch2(
    mut icases: &metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut prevPatterns: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>,
    mut info: &SourceInfo,
) -> Absyn::MatchType {
    let mut outMatchType: Absyn::MatchType;
    outMatchType = 'mc: {
        let __mc_input = &**icases;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Error::assertionOrAddSourceMessage(!(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?), &(Error::MATCHCONTINUE_TO_MATCH_OPTIMIZATION.clone()), metamodelica::nil(), info)?;
                    Ok(openmodelica_ast::Absyn::MatchType::MATCH)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns, .. }, tail: cases } => {
                    assertAllPatternListsDoNotOverlap(prevPatterns, metamodelica::AsArg::as_arg(&patterns))?;
                    Ok(optimizeContinueToMatch2(metamodelica::AsArg::as_arg(&cases), &(metamodelica::cons(patterns.clone(), prevPatterns.clone())), info))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(openmodelica_ast::Absyn::MatchType::MATCHCONTINUE)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outMatchType
}

fn assertAllPatternListsDoNotOverlap(
    mut ipss1: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Pattern>>>,
    mut ps2: &metamodelica::List<metamodelica::Ref<DAE::Pattern>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match ipss1 {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: ps1, tail: pss1 } => {
            let true = (patternListsDoNotOverlap(metamodelica::AsArg::as_arg(&ps1), ps2)?) else { return Err("pattern mismatch") };
            assertAllPatternListsDoNotOverlap(pss1, ps2)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn patternListsDoNotOverlap<'__b>(
    mut ips1: &'__b metamodelica::List<metamodelica::Ref<DAE::Pattern>>,
    mut ips2: &'__b metamodelica::List<metamodelica::Ref<DAE::Pattern>>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match (ips1, ips2) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(false)
            },
            (Deref @ metamodelica::ListNode::Cons { head: p1, tail: ps1 }, Deref @ metamodelica::ListNode::Cons { head: p2, tail: ps2 }) => {
                let mut res: bool;
                res = patternsDoNotOverlap(p1.clone(), p2.clone())?;
                if (!(res)) {{ (ips1, ips2) = (ps1, ps2); continue '__tco; }} else {return Ok(res)}
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn patternsDoNotOverlap(
    mut ip1: metamodelica::Ref<DAE::Pattern>,
    mut ip2: metamodelica::Ref<DAE::Pattern>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((ip1, ip2)) {
            (Deref @ DAE::Pattern::PAT_WILD { .. }, _) => {
                return Ok(false)
            },
            (_, Deref @ DAE::Pattern::PAT_WILD { .. }) => {
                return Ok(false)
            },
            (Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { .. }, _) => {
                return Ok(false)
            },
            (Deref @ DAE::Pattern::PAT_AS { pat: p1, .. }, p2) => {
                { (ip1, ip2) = (p1.clone(), p2.clone()); continue '__tco; }
            },
            (p1, Deref @ DAE::Pattern::PAT_AS { pat: p2, .. }) => {
                { (ip1, ip2) = (p1.clone(), p2.clone()); continue '__tco; }
            },
            (Deref @ DAE::Pattern::PAT_CONS { head: head1, tail: tail1 }, Deref @ DAE::Pattern::PAT_CONS { head: head2, tail: tail2 }) => {
                return Ok(patternsDoNotOverlap(head1.clone(), head2.clone())? || patternsDoNotOverlap(tail1.clone(), tail2.clone())?)
            },
            (Deref @ DAE::Pattern::PAT_SOME { pat: p1 }, Deref @ DAE::Pattern::PAT_SOME { pat: p2 }) => {
                { (ip1, ip2) = (p1.clone(), p2.clone()); continue '__tco; }
            },
            (Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: ps1 }, Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: ps2 }) => {
                return Ok(patternListsDoNotOverlap(metamodelica::AsArg::as_arg(&ps1), metamodelica::AsArg::as_arg(&ps2))?)
            },
            (Deref @ DAE::Pattern::PAT_CALL_TUPLE { patterns: ps1 }, Deref @ DAE::Pattern::PAT_CALL_TUPLE { patterns: ps2 }) => {
                return Ok(patternListsDoNotOverlap(metamodelica::AsArg::as_arg(&ps1), metamodelica::AsArg::as_arg(&ps2))?)
            },
            (Deref @ DAE::Pattern::PAT_CALL { name: name1, index: ix1, patterns: Deref @ metamodelica::ListNode::Nil, fields: _, typeVars: _, .. }, Deref @ DAE::Pattern::PAT_CALL { name: name2, index: ix2, patterns: Deref @ metamodelica::ListNode::Nil, fields: _, typeVars: _, .. }) => {
                let mut res: bool;
                res = ix1.clone() == ix2.clone();
                res = if (res) {AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&name1), metamodelica::AsArg::as_arg(&name2))} else {res};
                return Ok(!(res))
            },
            (Deref @ DAE::Pattern::PAT_CALL { name: name1, index: ix1, patterns: ps1, fields: _, typeVars: _, .. }, Deref @ DAE::Pattern::PAT_CALL { name: name2, index: ix2, patterns: ps2, fields: _, typeVars: _, .. }) => {
                let mut res: bool;
                res = ix1.clone() == ix2.clone();
                res = if (res) {AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&name1), metamodelica::AsArg::as_arg(&name2))} else {res};
                if (res) {return Ok(patternListsDoNotOverlap(metamodelica::AsArg::as_arg(&ps1), metamodelica::AsArg::as_arg(&ps2))?)} else {return Ok(!(res))}
            },
            (Deref @ DAE::Pattern::PAT_CONSTANT { exp: e1, .. }, Deref @ DAE::Pattern::PAT_CONSTANT { exp: e2, .. }) => {
                return Ok(!(ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?))
            },
            (Deref @ DAE::Pattern::PAT_CONSTANT { .. }, _) => {
                return Ok(true)
            },
            (_, Deref @ DAE::Pattern::PAT_CONSTANT { .. }) => {
                return Ok(true)
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn elabMatchCases(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut cases: metamodelica::List<metamodelica::Ref<Absyn::Case>>,
    mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inputAliases: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut matchExpLocalTree: &metamodelica::Ref<AvlSetString::Tree>,
    mut r#impl: bool,
    mut performVectorization: bool,
    mut pre: &DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    metamodelica::Ref<DAE::Type>,
)> {
    let mut outCache: FCore::Cache;
    let mut elabCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
    let mut resType: metamodelica::Ref<DAE::Type>;
    let mut resExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut resTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut tysFixed: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    tysFixed = List::map(tys, &Types::getUniontypeIfMetarecordReplaceAllSubtypes)?;
    (outCache, elabCases, resExps, resTypes) = elabMatchCases2(
        cache,
        env,
        cases,
        &tysFixed,
        inputAliases,
        matchExpLocalTree,
        r#impl,
        performVectorization,
        pre,
        metamodelica::nil(),
        metamodelica::nil(),
        metamodelica::nil(),
    )?;
    (elabCases, resType) = fixCaseReturnTypes(elabCases, resExps, resTypes, info)?;
    Ok((outCache, elabCases, resType))
}

fn elabMatchCases2<'__b>(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut cases: metamodelica::List<metamodelica::Ref<Absyn::Case>>,
    mut tys: &'__b metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inputAliases: &'__b metamodelica::List<metamodelica::List<ArcStr>>,
    mut matchExpLocalTree: &'__b metamodelica::Ref<AvlSetString::Tree>,
    mut r#impl: bool,
    mut performVectorization: bool,
    mut pre: &'__b DAE::Prefix,
    mut inAccCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut inAccExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inAccTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
)> {
    let mut outCache: FCore::Cache;
    let mut elabCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
    let mut resExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut resTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (outCache, elabCases, resExps, resTypes) = (::match_deref::match_deref! { match &(cases) {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache;
            let mut accExps = inAccExps;
            let mut accTypes = inAccTypes;
            (cache, inAccCases.reverse(), accExps.reverse(), accTypes.reverse())
        },
        Deref @ metamodelica::ListNode::Cons { head: case_, tail: rest } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut accExps = inAccExps;
            let mut accTypes = inAccTypes;
            let mut elabCase: metamodelica::Ref<DAE::MatchCase>;
            let mut optType: Option<metamodelica::Ref<DAE::Type>>;
            let mut optExp: Option<metamodelica::Ref<DAE::Exp>>;
            (cache, elabCase, optExp, optType) = elabMatchCase(cache, env.clone(), case_.clone(), tys, inputAliases, matchExpLocalTree, r#impl, performVectorization, pre)?;
            (cache, elabCases, accExps, accTypes) = elabMatchCases2(cache, env, rest.clone(), tys, inputAliases, matchExpLocalTree, r#impl, performVectorization, pre, metamodelica::cons(elabCase, inAccCases), List::consOption(optExp, accExps), List::consOption(optType, accTypes))?;
            (cache, elabCases, accExps, accTypes)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, elabCases, resExps, resTypes))
}

fn elabMatchCase<'__b>(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut acase: metamodelica::Ref<Absyn::Case>,
    mut tys: &'__b metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inputAliases: &'__b metamodelica::List<metamodelica::List<ArcStr>>,
    mut matchExpLocalTree: &'__b metamodelica::Ref<AvlSetString::Tree>,
    mut r#impl: bool,
    mut performVectorization: bool,
    mut pre: &'__b DAE::Prefix,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::MatchCase>,
    Option<metamodelica::Ref<DAE::Exp>>,
    Option<metamodelica::Ref<DAE::Type>>,
)> {
    let mut outCache: FCore::Cache;
    let mut elabCase: metamodelica::Ref<DAE::MatchCase>;
    let mut resExp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut resType: Option<metamodelica::Ref<DAE::Type>>;
    (outCache, elabCase, resExp, resType) = (match &*acase {
        Absyn::Case::CASE {
            pattern,
            patternGuard,
            patternInfo,
            localDecls: decls,
            classPart: cp,
            result,
            resultInfo,
            info,
            ..
        } => {
            let mut cache = inCache;
            let mut env = inEnv.clone();
            let mut patterns: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut elabPatterns: metamodelica::List<metamodelica::Ref<DAE::Pattern>>;
            let mut elabPatterns2: metamodelica::List<metamodelica::Ref<DAE::Pattern>>;
            let mut elabResult: Option<metamodelica::Ref<DAE::Exp>>;
            let mut dPatternGuard: Option<metamodelica::Ref<DAE::Exp>>;
            let mut caseDecls: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut eqAlgs: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            let mut algs: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
            let mut body: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut caseLocalTree: metamodelica::Ref<AvlSetString::Tree>;
            let mut localsTree: metamodelica::Ref<AvlSetString::Tree>;
            let mut useTree: metamodelica::Ref<AvlSetString::Tree>;
            let mut resultInfo = (*resultInfo).clone();
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(addLocalDecls(cache, env, decls.clone(), arcstr::literal!(FCore::caseScopeName), r#impl, metamodelica::AsArg::as_arg(&info))?) {
                (__pa0, Some((__pa1, DAE::DAElist { elementLst: __pa2 }, __pa3))) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            env = metamodelica::Own::own(__pa1);
            caseDecls = metamodelica::Own::own(__pa2);
            caseLocalTree = metamodelica::Own::own(__pa3);
            patterns = convertExpToPatterns(pattern.clone());
            patterns = if (((tys).len() as i32) == 1) {
                list![pattern.clone()]
            } else {
                patterns
            };
            (cache, elabPatterns) = elabPatternTuple(
                cache,
                &env,
                &patterns,
                tys,
                metamodelica::AsArg::as_arg(&patternInfo),
                metamodelica::AsArg::as_arg(&pattern),
            )?;
            checkPatternsDuplicateAsBindings(&elabPatterns, metamodelica::AsArg::as_arg(&patternInfo))?;
            env = FGraph::openNewScope(
                env,
                openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
                Some(arcstr::literal!(FCore::patternTypeScope)),
                None,
            )?;
            (elabPatterns2, cache) = addPatternAliasesList(&elabPatterns, inputAliases.clone(), cache, &inEnv)?;
            (_, env) = traversePatternList(&elabPatterns2, &addEnvKnownAsBindings, env)?;
            eqAlgs = Static::fromEquationsToAlgAssignments(cp.clone())?;
            algs = AbsynToSCode::translateClassdefAlgorithmitems(eqAlgs)?;
            (cache, body) = InstSection::instStatements(
                cache,
                env.clone(),
                InnerOuter::emptyInstHierarchy().clone(),
                pre.clone(),
                &(ClassInf::State::FUNCTION {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("match"),
                    }),
                    isImpure: false,
                }),
                &algs,
                ElementSource::addElementSourceFileInfo(DAE::emptyElementSource().clone(), patternInfo.clone()),
                openmodelica_frontend_types::SCode::Initial::NON_INITIAL,
                true,
                InstTypes::neverUnroll.clone(),
            )?;
            (cache, body, elabResult, resultInfo, resType) = elabResultExp(
                cache,
                env.clone(),
                body,
                result.clone(),
                r#impl,
                performVectorization,
                pre.clone(),
                resultInfo.clone(),
            )?;
            (cache, dPatternGuard) = elabPatternGuard(
                cache,
                env,
                patternGuard.clone(),
                r#impl,
                performVectorization,
                pre.clone(),
                patternInfo.clone(),
            )?;
            localsTree = AvlSetString::join(matchExpLocalTree.clone(), &caseLocalTree)?;
            useTree = AvlSetString::new();
            (_, useTree) = Expression::traverseExpBottomUp(
                metamodelica::Ref::new(DAE::Exp::META_OPTION {
                    exp: elabResult.clone(),
                }),
                &useLocalCref,
                useTree,
            )?;
            (body, useTree) = statementListFindDeadStoreRemoveEmptyStatements(body, localsTree.clone(), useTree)?;
            (_, useTree) = Expression::traverseExpBottomUp(
                metamodelica::Ref::new(DAE::Exp::META_OPTION {
                    exp: dPatternGuard.clone(),
                }),
                &useLocalCref,
                useTree,
            )?;
            (elabPatterns, _) = traversePatternList(
                &elabPatterns,
                &checkDefUsePattern,
                (localsTree.clone(), useTree, patternInfo.clone()),
            )?;
            useTree = AvlSetString::new();
            (_, useTree) = Expression::traverseExpBottomUp(
                metamodelica::Ref::new(DAE::Exp::META_OPTION {
                    exp: elabResult.clone(),
                }),
                &useLocalCref,
                useTree,
            )?;
            (body, useTree) = statementListFindDeadStoreRemoveEmptyStatements(body, localsTree.clone(), useTree)?;
            (_, useTree) = Expression::traverseExpBottomUp(
                metamodelica::Ref::new(DAE::Exp::META_OPTION {
                    exp: dPatternGuard.clone(),
                }),
                &useLocalCref,
                useTree,
            )?;
            (elabPatterns, _) = traversePatternList(
                &elabPatterns,
                &checkDefUsePattern,
                (localsTree, useTree, patternInfo.clone()),
            )?;
            elabCase = metamodelica::Ref::new(DAE::MatchCase {
                patterns: elabPatterns,
                patternGuard: dPatternGuard,
                localDecls: caseDecls,
                body: body,
                result: elabResult.clone(),
                resultInfo: resultInfo.clone(),
                jump: 0,
                info: info.clone(),
            });
            (cache, elabCase, elabResult, resType)
        }
        Absyn::Case::ELSE {
            localDecls: decls,
            classPart: cp,
            result,
            resultInfo,
            info,
            ..
        } => {
            let mut cache = inCache;
            let mut env = inEnv.clone();
            let mut pattern: metamodelica::Ref<Absyn::Exp>;
            let mut patterns: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut elabResult: Option<metamodelica::Ref<DAE::Exp>>;
            let mut len: i32;
            len = ((tys).len() as i32);
            patterns = List::fill(
                metamodelica::Ref::new(Absyn::Exp::CREF {
                    componentRef: openmodelica_ast::Absyn::ComponentRef::interned_WILD(),
                }),
                ((tys).len() as i32),
            );
            pattern = if (len == 1) {
                metamodelica::Ref::new(Absyn::Exp::CREF {
                    componentRef: openmodelica_ast::Absyn::ComponentRef::interned_WILD(),
                })
            } else {
                metamodelica::Ref::new(Absyn::Exp::TUPLE { expressions: patterns })
            };
            (cache, elabCase, elabResult, resType) = elabMatchCase(
                cache,
                env,
                metamodelica::Ref::new(Absyn::Case::CASE {
                    pattern: pattern,
                    patternGuard: None,
                    patternInfo: info.clone(),
                    localDecls: decls.clone(),
                    classPart: cp.clone(),
                    result: result.clone(),
                    resultInfo: resultInfo.clone(),
                    comment: None,
                    info: info.clone(),
                }),
                tys,
                inputAliases,
                matchExpLocalTree,
                r#impl,
                performVectorization,
                pre,
            )?;
            (cache, elabCase, elabResult, resType)
        }
    });
    Ok((outCache, elabCase, resExp, resType))
}

fn elabResultExp(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inBody: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut r#impl: bool,
    mut performVectorization: bool,
    mut pre: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    Option<metamodelica::Ref<DAE::Exp>>,
    SourceInfo,
    Option<metamodelica::Ref<DAE::Type>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outBody: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut resExp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut resultInfo: SourceInfo;
    let mut resType: Option<metamodelica::Ref<DAE::Type>>;
    (outCache, outBody, resExp, resultInfo, resType) = (::match_deref::match_deref! { match &(AbsynUtil::stripCommentExpressions(exp.clone(), false)?) {
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "fail", subscripts: Deref @ metamodelica::ListNode::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Nil, argNames: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut cache = inCache;
            let mut body = inBody;
            (cache, body, None, inInfo, None)
        },
        _ => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut body = inBody;
            let mut elabExp: metamodelica::Ref<DAE::Exp>;
            let mut prop: DAE::Properties;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut info: SourceInfo;
            (cache, elabExp, prop) = Static::elabExp(cache, env, exp, r#impl, performVectorization, pre, inInfo.clone())?;
            ty = Types::getPropType(&prop);
            (elabExp, ty) = makeTupleFromMetaTuple(elabExp, ty)?;
            (body, elabExp, info) = elabResultExp2(!(Flags::isSet(Flags::PATTERNM_MOVE_LAST_EXP.clone())?), body, elabExp, inInfo);
            (cache, body, Some(elabExp), info, Some(ty))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outBody, resExp, resultInfo, resType))
}

fn elabPatternGuard(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut patternGuard: Option<metamodelica::Ref<Absyn::Exp>>,
    mut r#impl: bool,
    mut performVectorization: bool,
    mut pre: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, Option<metamodelica::Ref<DAE::Exp>>)> {
    let mut outCache: FCore::Cache;
    let mut outPatternGuard: Option<metamodelica::Ref<DAE::Exp>>;
    (outCache, outPatternGuard) = 'mc: {
        let __mc_input = (inCache, inEnv, patternGuard, inInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, None, _) => {
                    Ok((cache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Some(exp), info) => {
                    let mut elabExp: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, elabExp, prop) = Static::elabExp(cache.clone(), env.clone(), exp.clone(), r#impl, performVectorization, pre.clone(), info.clone())?;
                    (elabExp, _) = Types::matchType(elabExp.clone(), Types::getPropType(&prop), DAE::T_BOOL_DEFAULT().clone(), true)?;
                    Ok((cache.clone(), Some(elabExp.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Some(exp), info) => {
                    let mut prop: DAE::Properties;
                    let mut r#str: ArcStr;
                    (_, _, prop) = Static::elabExp(cache.clone(), env.clone(), exp.clone(), r#impl, performVectorization, pre.clone(), info.clone())?;
                    r#str = TypesDump::unparseType(Types::getPropType(&prop))?;
                    Error::addSourceMessage(&(Error::GUARD_EXPRESSION_TYPE_MISMATCH.clone()), list![r#str.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outPatternGuard))
}

fn elabResultExp2(
    mut skipPhase: bool,
    mut body: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut elabExp: metamodelica::Ref<DAE::Exp>,
    mut info: SourceInfo,
) -> (
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::Ref<DAE::Exp>,
    SourceInfo,
) {
    let mut outBody: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outInfo: SourceInfo;
    (outBody, outExp, outInfo) = 'mc: {
        let __mc_input = (skipPhase, body.clone(), elabExp.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, b, e, i) => {
                    Ok((b.clone(), e.clone(), i.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, b, elabCr2 @ Deref @ DAE::Exp::CREF { .. }, _) => {
                    let mut elabCr1: metamodelica::Ref<DAE::Exp>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut i: SourceInfo;
                    let mut b = (*b).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(List::splitLast(b.clone())?) {
                        (Deref @ DAE::Statement::STMT_ASSIGN { exp1: __pa0, exp: __pa1, source: Deref @ DAE::ElementSource { info: __pa2, .. }, .. }, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    elabCr1 = metamodelica::Own::own(__pa0);
                    e = metamodelica::Own::own(__pa1);
                    i = metamodelica::Own::own(__pa2);
                    b = metamodelica::Own::own(__pa3);
                    let true = (ExpressionBasics::expEqual(&elabCr1, elabCr2.clone())?) else { return Err("pattern mismatch") };
                    (b, e, i) = elabResultExp2(false, b.clone(), e.clone(), i.clone());
                    Ok((b.clone(), e.clone(), i.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, b, Deref @ DAE::Exp::TUPLE { PR: elabCrs2 }, _) => {
                    let mut elabCrs1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut i: SourceInfo;
                    let mut b = (*b).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(List::splitLast(b.clone())?) {
                        (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: __pa0, exp: __pa1, source: Deref @ DAE::ElementSource { info: __pa2, .. }, .. }, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    elabCrs1 = metamodelica::Own::own(__pa0);
                    e = metamodelica::Own::own(__pa1);
                    i = metamodelica::Own::own(__pa2);
                    b = metamodelica::Own::own(__pa3);
                    let true = (List::isEqualOnTrue(elabCrs1.clone(), elabCrs2.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>| ExpressionBasics::expEqual(&__a0, __a1))?) else { return Err("pattern mismatch") };
                    (b, e, i) = elabResultExp2(false, b.clone(), e.clone(), i.clone());
                    Ok((b.clone(), e.clone(), i.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((body.clone(), elabExp.clone(), info.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outBody, outExp, outInfo)
}

fn fixCaseReturnTypes(
    mut icases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut iexps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut itys: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    metamodelica::Ref<DAE::Type>,
)> {
    let mut outCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
    let mut ty: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_NORETCALL);
    (outCases, ty) = 'mc: {
        let __mc_input = (icases, iexps, itys.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cases, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok((cases.clone(), DAE::T_NORETCALL_DEFAULT().clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cases, exps, tys) => {
                    let mut cases = (*cases).clone();
                    let mut exps = (*exps).clone();
                    let mut ty: metamodelica::Ref<DAE::Type> = ty.clone();
                    ty = List::reduce(&(List::map(tys.clone(), &fnptr!(Types::boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?), &Types::superType)?;
                    ty = Types::superType(ty.clone(), ty.clone())?;
                    ty = Types::unboxedType(ty.clone())?;
                    ty = Types::makeRegularTupleFromMetaTupleOnTrue(Types::allTuple(metamodelica::AsArg::as_arg(&tys)), ty.clone())?;
                    ty = Types::getUniontypeIfMetarecordReplaceAllSubtypes(ty.clone())?;
                    (exps, _) = Types::matchTypes(exps.clone(), tys.clone(), &ty, true)?;
                    cases = Types::fixCaseReturnTypes2(metamodelica::AsArg::as_arg(&cases), exps.clone(), info.clone())?;
                    Ok(((cases.clone(), ty.clone()), ty.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            ty = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cases, exps, tys) => {
                    let mut cases = (*cases).clone();
                    let mut exps = (*exps).clone();
                    let mut ty: metamodelica::Ref<DAE::Type> = ty.clone();
                    ty = List::reduce(metamodelica::AsArg::as_arg(&tys), &Types::superType)?;
                    ty = Types::superType(ty.clone(), ty.clone())?;
                    ty = Types::unboxedType(ty.clone())?;
                    ty = Types::makeRegularTupleFromMetaTupleOnTrue(Types::allTuple(metamodelica::AsArg::as_arg(&tys)), ty.clone())?;
                    ty = Types::getUniontypeIfMetarecordReplaceAllSubtypes(ty.clone())?;
                    (exps, _) = Types::matchTypes(exps.clone(), tys.clone(), &ty, true)?;
                    cases = Types::fixCaseReturnTypes2(metamodelica::AsArg::as_arg(&cases), exps.clone(), info.clone())?;
                    Ok(((cases.clone(), ty.clone()), ty.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            ty = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    tys = List::unionOnTrue(&itys, &(metamodelica::nil()), &fnptr!(Types::equivtypes, metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Type>))?;
                    r#str = stringAppendList(List::map1r(List::map(tys.clone(), &TypesDump::unparseType)?, &fnptr!(stringAppend, ArcStr, ArcStr), literal!("\n  "))?);
                    Error::addSourceMessage(&(Error::META_MATCHEXP_RESULT_TYPES.clone()), list![r#str.clone()], &info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCases, ty))
}

pub fn traverseConstantPatternsHelper<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inT: T,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, T) -> Result<(metamodelica::Ref<DAE::Exp>, T)> + 'static,
    >,
) -> Result<(metamodelica::Ref<DAE::Exp>, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, T) -> Result<(metamodelica::Ref<DAE::Exp>, T)> + 'static,
    >;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outT: T = inT;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        __esc_outExp @ Deref @ DAE::Exp::MATCHEXPRESSION { cases, .. } => {
            outExp = (*__esc_outExp).clone();
            let mut cases2: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
            let mut case_: metamodelica::Ref<DAE::MatchCase>;
            let mut patterns: metamodelica::List<metamodelica::Ref<DAE::Pattern>>;
            cases2 = metamodelica::nil();
            for mut c in &*cases.clone() {
                case_ = c.clone();
                case_ = (match &*case_ {
        DAE::MatchCase { .. } => {
            (patterns, outT) = traversePatternList(&case_.patterns, &({ let __pe_b2: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a1| traverseConstantPatternsHelper2(__pe_a0, __pe_a1, &*__pe_b2) }), outT)?;
            if !(case_.patterns.clone() == patterns.clone()) {
                assign_field!(case_.patterns = patterns);
            }
            case_
        },
    });
                cases2 = metamodelica::cons(case_, cases2);
            }
            cases2 = Dangerous::listReverseInPlace(cases2);
            if !(cases.clone() == cases2.clone()) {
                assign_variant_field!(outExp => DAE::Exp::MATCHEXPRESSION; cases = cases2);
            }
            (outExp, outT) = func(outExp.clone(), outT)?;
            outExp.clone()
        },
        _ => {
            (outExp, outT) = func(inExp, outT)?;
            outExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outT))
}

pub(crate) fn traverseConstantPatternsHelper2<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inPattern: metamodelica::Ref<DAE::Pattern>,
    mut inExtra: T,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, T) -> Result<(metamodelica::Ref<DAE::Exp>, T)>,
) -> Result<(metamodelica::Ref<DAE::Pattern>, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, T) -> Result<(metamodelica::Ref<DAE::Exp>, T)> + 'static,
    >;

    let mut outPattern: metamodelica::Ref<DAE::Pattern>;
    let mut extra: T = inExtra;
    outPattern = (::match_deref::match_deref! { match &(inPattern.clone()) {
        __esc_outPattern @ Deref @ DAE::Pattern::PAT_CONSTANT { .. } => {
            outPattern = (*__esc_outPattern).clone();
            let mut exp: metamodelica::Ref<DAE::Exp>;
            (exp, extra) = func(var_field!((*outPattern).exp, DAE::Pattern::PAT_CONSTANT).clone(), extra)?;
            if !(referenceEq(&*(var_field!((*outPattern).exp, DAE::Pattern::PAT_CONSTANT).clone()),&*(&*exp))) {
                assign_variant_field!(outPattern => DAE::Pattern::PAT_CONSTANT; exp = exp);
            }
            outPattern.clone()
        },
        _ => {
            inPattern
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outPattern, extra))
}

fn filterEmptyPattern(mut tpl: &(metamodelica::Ref<DAE::Pattern>, ArcStr, metamodelica::Ref<DAE::Type>)) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &(tpl) {
        (Deref @ DAE::Pattern::PAT_WILD { .. }, _, _) => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

fn addLocalDecls(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut els: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut scopeName: ArcStr,
    mut r#impl: bool,
    mut info: &SourceInfo,
) -> Result<(
    FCore::Cache,
    Option<(FCore::Graph, DAE::DAElist, metamodelica::Ref<AvlSetString::Tree>)>,
)> {
    let mut outCache: FCore::Cache;
    let mut res: Option<(FCore::Graph, DAE::DAElist, metamodelica::Ref<AvlSetString::Tree>)> = None;
    (outCache, res) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv, els);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Nil) => {
                    let mut declsTree: metamodelica::Ref<AvlSetString::Tree>;
                    declsTree = AvlSetString::new();
                    Ok((cache.clone(), Some((env.clone(), DAE::emptyDae().clone(), declsTree.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ld) => {
                    let mut ld2: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut ld_mod: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut dae1: DAE::DAElist;
                    let mut env2: FCore::Graph;
                    let mut dummyFunc: ClassInf::State;
                    let mut b: bool;
                    let mut declsTree: metamodelica::Ref<AvlSetString::Tree>;
                    let mut names: metamodelica::List<ArcStr>;
                    let mut cache = (*cache).clone();
                    let mut res: Option<(FCore::Graph, DAE::DAElist, metamodelica::Ref<AvlSetString::Tree>)> = res.clone();
                    env2 = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, scopeName.clone(), None)?;
                    ld2 = AbsynToSCode::translateEitemlist(ld.clone(), openmodelica_frontend_types::SCode::Visibility::PROTECTED)?;
                    let true = (List::applyAndFold1(&ld2, &fnptr!(boolAnd, bool, bool), &move |__a0: metamodelica::Ref<SCode::Element>, __a1: Absyn::Direction| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::isComponentWithDirection(&__a0, __a1)) }, openmodelica_ast::Absyn::Direction::BIDIR, true)?) else { return Err("pattern mismatch") };
                    (cache, b) = List::fold1(&ld2, &move |__a0: metamodelica::Ref<SCode::Element>, __a1: FCore::Graph, __a2: (FCore::Cache, bool)| checkLocalShadowing(&__a0, &__a1, __a2), env.clone(), (cache.clone(), false))?;
                    ld2 = if (b) {metamodelica::nil()} else {ld2.clone()};
                    ld_mod = InstUtil::addNomod(ld2.clone());
                    dummyFunc = ClassInf::State::FUNCTION { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("dummieFunc") }), isImpure: false };
                    (cache, env2, _) = InstUtil::addComponentsToEnv(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, dummyFunc.clone(), &ld_mod, r#impl)?;
                    (cache, env2, _, _, dae1, _, _, _, _, _) = Inst::instElementList(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, dummyFunc.clone(), ld_mod.clone(), metamodelica::nil(), r#impl, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), true)?;
                    names = List::map(ld2.clone(), &move |__a0: metamodelica::Ref<SCode::Element>| SCodeUtil::elementName(&__a0))?;
                    declsTree = AvlSetString::addList(AvlSetString::new(), &names)?;
                    res = if (b) {None} else {Some((env2.clone(), dae1.clone(), declsTree.clone()))};
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
                (cache, _, ld) => {
                    let mut ld2: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut r#str: ArcStr;
                    ld2 = AbsynToSCode::translateEitemlist(ld.clone(), openmodelica_frontend_types::SCode::Visibility::PROTECTED)?;
                    let __pa0 = ::match_deref::match_deref! { match &(List::filterOnTrue(ld2.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::isNotComponent(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<bool> + 'static>))?) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ld2 = metamodelica::Own::own(__pa0);
                    r#str = stringDelimitList(List::map1(ld2.clone(), &SCodeDump::unparseElementStr, SCodeDump::defaultOptions.clone())?, literal!(", "));
                    Error::addSourceMessage(&(Error::META_INVALID_LOCAL_ELEMENT.clone()), list![r#str.clone()], info)?;
                    Ok((cache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, ld) => {
                    let mut ld2: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut ld3: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut ld4: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut r#str: ArcStr;
                    ld2 = AbsynToSCode::translateEitemlist(ld.clone(), openmodelica_frontend_types::SCode::Visibility::PROTECTED)?;
                    ld3 = List::select1(ld2.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SCode::Element>, __a1: Absyn::Direction| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::isComponentWithDirection(&__a0, __a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>, Absyn::Direction) -> Result<bool> + 'static>), openmodelica_ast::Absyn::Direction::INPUT)?;
                    ld4 = List::select1(ld2.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SCode::Element>, __a1: Absyn::Direction| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::isComponentWithDirection(&__a0, __a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>, Absyn::Direction) -> Result<bool> + 'static>), openmodelica_ast::Absyn::Direction::OUTPUT)?;
                    let __pa0 = ::match_deref::match_deref! { match &(listAppend(ld3.clone(), ld4.clone())) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ld2 = metamodelica::Own::own(__pa0);
                    r#str = stringDelimitList(List::map1(ld2.clone(), &SCodeDump::unparseElementStr, SCodeDump::defaultOptions.clone())?, literal!(", "));
                    Error::addSourceMessage(&(Error::META_INVALID_LOCAL_ELEMENT.clone()), list![r#str.clone()], info)?;
                    Ok((cache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![literal!("Patternm.addLocalDecls failed")], info)?;
                    Ok((inCache.clone(), None))
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

fn checkLocalShadowing(
    mut elt: &metamodelica::Ref<SCode::Element>,
    mut env: &FCore::Graph,
    mut inTpl: (FCore::Cache, bool),
) -> Result<(FCore::Cache, bool)> {
    let mut outTpl: (FCore::Cache, bool) = inTpl.clone();
    let mut name: ArcStr;
    let mut cache: FCore::Cache;
    let mut b: bool;
    let mut info: SourceInfo;
    let mut var: SCode::Variability;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*elt)) {
        Deref @ SCode::Element::COMPONENT { name: __pa0, info: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    info = metamodelica::Own::own(__pa1);
    (cache, _) = inTpl;
    match '__try2: {
        let (__pa3, __t5, _, _, _, _, _, _, _) = unwrap_break_err!(Lookup::lookupVarInternalIdent(cache.clone(), env, &name, &(metamodelica::nil()), openmodelica_frontend_inst::InstTypes::SearchStrategy::SEARCH_LOCAL_ONLY), '__try2);
        let __arc6 = __t5.clone();
        let DAE::ATTR { variability: __pa4, .. } = &*__arc6;
        cache = metamodelica::Own::own(__pa3);
        var = metamodelica::Own::own(__pa4);
        b = (match var {
            SCode::Variability::CONST { .. } => true,
            _ => false,
        });
        Ok::<_, &'static str>((b.clone(),))
    } {
        Ok((__try2_o0,)) => {
            b = __try2_o0;
        }
        Err(_) => {
            b = true;
        }
    }
    if !(b) {
        Error::addSourceMessage(&(Error::MATCH_SHADOWING.clone()), list![name], &info)?;
        outTpl = (cache, true);
    }
    Ok(outTpl)
}

fn allPatternsWild<'__b>(mut ipats: &'__b metamodelica::List<metamodelica::Ref<DAE::Pattern>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match ipats {
            Deref @ metamodelica::ListNode::Nil => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Pattern::PAT_WILD { .. }, tail: pats } => {
                { ipats = pats; continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn allPatternsAlwaysMatch(mut ipats: metamodelica::List<metamodelica::Ref<DAE::Pattern>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(ipats) {
            Deref @ metamodelica::ListNode::Nil => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Pattern::PAT_WILD { .. }, tail: pats } => {
                { ipats = pats.clone(); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Pattern::PAT_AS { pat, .. }, tail: pats } => {
                { ipats = metamodelica::cons(pat.clone(), pats.clone()); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { pat, .. }, tail: pats } => {
                { ipats = metamodelica::cons(pat.clone(), pats.clone()); continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn isInfallibleNoBinding(mut pat: &metamodelica::Ref<DAE::Pattern>) -> Result<bool> {
    let mut b: bool;
    b = (match &**pat {
        DAE::Pattern::PAT_META_TUPLE { patterns: pats } => {
            List::all(pats, &move |__a0: metamodelica::Ref<DAE::Pattern>| {
                isInfallibleNoBindingOrWild(&__a0)
            })?
        }
        DAE::Pattern::PAT_CALL_TUPLE { patterns: pats } => {
            List::all(pats, &move |__a0: metamodelica::Ref<DAE::Pattern>| {
                isInfallibleNoBindingOrWild(&__a0)
            })?
        }
        DAE::Pattern::PAT_CALL {
            knownSingleton: true,
            patterns: pats,
            ..
        } => List::all(pats, &move |__a0: metamodelica::Ref<DAE::Pattern>| {
            isInfallibleNoBindingOrWild(&__a0)
        })?,
        DAE::Pattern::PAT_CALL_NAMED {
            name: _,
            patterns: namedPats,
        } => (namedPats).is_empty(),
        _ => false,
    });
    Ok(b)
}

fn isInfallibleNoBindingOrWild(mut pat: &metamodelica::Ref<DAE::Pattern>) -> Result<bool> {
    let mut b: bool;
    b = (match &**pat {
        DAE::Pattern::PAT_WILD { .. } => true,
        _ => isInfallibleNoBinding(pat)?,
    });
    Ok(b)
}

fn isInfalliblePattern<'__b>(mut pat: &'__b metamodelica::Ref<DAE::Pattern>) -> Result<bool> {
    '__tco: loop {
        match &**pat {
            DAE::Pattern::PAT_WILD { .. } => return Ok(true),
            DAE::Pattern::PAT_AS { pat: innerPat, .. } => {
                pat = innerPat;
                continue '__tco;
            }
            DAE::Pattern::PAT_AS_FUNC_PTR { pat: innerPat, .. } => {
                pat = innerPat;
                continue '__tco;
            }
            DAE::Pattern::PAT_META_TUPLE { patterns: pats } => {
                return Ok(List::all(pats, &move |__a0: metamodelica::Ref<DAE::Pattern>| {
                    isInfalliblePattern(&__a0)
                })?);
            }
            DAE::Pattern::PAT_CALL_TUPLE { patterns: pats } => {
                return Ok(List::all(pats, &move |__a0: metamodelica::Ref<DAE::Pattern>| {
                    isInfalliblePattern(&__a0)
                })?);
            }
            DAE::Pattern::PAT_CALL {
                knownSingleton: true,
                patterns: pats,
                ..
            } => {
                return Ok(List::all(pats, &move |__a0: metamodelica::Ref<DAE::Pattern>| {
                    isInfalliblePattern(&__a0)
                })?);
            }
            DAE::Pattern::PAT_CALL_NAMED {
                patterns: namedPats, ..
            } => return Ok((namedPats).is_empty()),
            _ => return Ok(false),
        }
    }
}

fn isSingleInfallibleMatch(
    mut matchType: Absyn::MatchType,
    mut cases: &metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((matchType, &**cases)) {
        (Absyn::MatchType::MATCH { .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns: pats, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            List::all(metamodelica::AsArg::as_arg(&pats), &move |__a0: metamodelica::Ref<DAE::Pattern>| isInfalliblePattern(&__a0))?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn checkMatchSingleInfallibleCase(
    mut matchType: Absyn::MatchType,
    mut cases: &metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut info: &SourceInfo,
) -> Result<()> {
    if Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())? && isSingleInfallibleMatch(matchType, cases)? {
        Error::addSourceMessage(
            &(Error::MATCH_SINGLE_INFALLIBLE_CASE.clone()),
            metamodelica::nil(),
            info,
        )?;
    }
    Ok(())
}

fn checkInfallibleNoBindingPatterns(
    mut cases: &metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut matchType: Absyn::MatchType,
    mut info: &SourceInfo,
) -> Result<()> {
    if !(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?) {
        return Ok(());
    }
    if isSingleInfallibleMatch(matchType, cases)? {
        return Ok(());
    }
    for mut c in &**cases {
        let () = (match &*c.clone() {
            DAE::MatchCase {
                patterns: pats,
                info: cinfo,
                ..
            } => {
                for mut p in &*pats.clone() {
                    checkPatternInfallibleNoBinding(
                        metamodelica::AsArg::as_arg(&p),
                        metamodelica::AsArg::as_arg(&cinfo),
                    )?;
                }
                ()
            }
        });
    }
    Ok(())
}

fn checkPatternInfallibleNoBinding(mut pat: &metamodelica::Ref<DAE::Pattern>, mut info: &SourceInfo) -> Result<()> {
    let () = (match &**pat {
        DAE::Pattern::PAT_WILD { .. } => (),
        _ if (isInfallibleNoBinding(pat)?) => {
            Error::addSourceMessage(
                &(Error::META_PATTERN_INFALLIBLE_NO_BINDING.clone()),
                list![ExpressionDump::patternStr(pat)?],
                info,
            )?;
            ()
        }
        DAE::Pattern::PAT_META_TUPLE { patterns: pats } => {
            for mut p in &*pats.clone() {
                checkPatternInfallibleNoBinding(metamodelica::AsArg::as_arg(&p), info)?;
            }
            ()
        }
        DAE::Pattern::PAT_CALL_TUPLE { patterns: pats } => {
            for mut p in &*pats.clone() {
                checkPatternInfallibleNoBinding(metamodelica::AsArg::as_arg(&p), info)?;
            }
            ()
        }
        DAE::Pattern::PAT_CALL { patterns: pats, .. } => {
            for mut p in &*pats.clone() {
                checkPatternInfallibleNoBinding(metamodelica::AsArg::as_arg(&p), info)?;
            }
            ()
        }
        DAE::Pattern::PAT_CALL_NAMED {
            patterns: namedPats, ..
        } => {
            for mut tpl in &*namedPats.clone() {
                checkPatternInfallibleNoBinding(&(Util::tuple31(tpl.clone())), info)?;
            }
            ()
        }
        DAE::Pattern::PAT_CONS {
            head: innerPat,
            tail: __pat_tail,
        } => {
            checkPatternInfallibleNoBinding(innerPat, info)?;
            checkPatternInfallibleNoBinding(metamodelica::AsArg::as_arg(&__pat_tail), info)?;
            ()
        }
        DAE::Pattern::PAT_SOME { pat: innerPat } => {
            checkPatternInfallibleNoBinding(innerPat, info)?;
            ()
        }
        DAE::Pattern::PAT_AS { pat: innerPat, .. } => {
            checkPatternInfallibleNoBinding(innerPat, info)?;
            ()
        }
        DAE::Pattern::PAT_AS_FUNC_PTR { pat: innerPat, .. } => {
            checkPatternInfallibleNoBinding(innerPat, info)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

fn getCasePatterns(
    mut case_: &metamodelica::Ref<DAE::MatchCase>,
) -> metamodelica::List<metamodelica::Ref<DAE::Pattern>> {
    let mut pats: metamodelica::List<metamodelica::Ref<DAE::Pattern>>;
    let __arc1 = &(*case_);
    let DAE::CASE { patterns: __pa0, .. } = &**__arc1;
    pats = metamodelica::Own::own(__pa0);
    pats
}

fn setCasePatterns(
    mut case1: &metamodelica::Ref<DAE::MatchCase>,
    mut pats: metamodelica::List<metamodelica::Ref<DAE::Pattern>>,
) -> metamodelica::Ref<DAE::MatchCase> {
    let mut case2: metamodelica::Ref<DAE::MatchCase>;
    case2 = (match &**case1 {
        DAE::MatchCase {
            patterns: _,
            patternGuard,
            localDecls,
            body,
            result,
            resultInfo,
            jump,
            info,
        } => metamodelica::Ref::new(DAE::MatchCase {
            patterns: pats,
            patternGuard: patternGuard.clone(),
            localDecls: localDecls.clone(),
            body: body.clone(),
            result: result.clone(),
            resultInfo: resultInfo.clone(),
            jump: jump.clone(),
            info: info.clone(),
        }),
    });
    case2
}

pub fn getValueCtor(mut ix: i32) -> i32 {
    let mut ctor: i32;
    ctor = ix + 3;
    ctor
}

pub fn sortPatternsByComplexity(
    mut inPatterns: &metamodelica::List<metamodelica::Ref<DAE::Pattern>>,
) -> Result<metamodelica::List<(metamodelica::Ref<DAE::Pattern>, i32)>> {
    let mut outPatterns: metamodelica::List<(metamodelica::Ref<DAE::Pattern>, i32)>;
    outPatterns = List::toListWithPositions(inPatterns);
    outPatterns = List::sort(
        outPatterns,
        (std::sync::Arc::new(
            move |__a0: (metamodelica::Ref<DAE::Pattern>, i32), __a1: (metamodelica::Ref<DAE::Pattern>, i32)| {
                sortPatternsByComplexityWork(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::Pattern>, i32),
                        (metamodelica::Ref<DAE::Pattern>, i32),
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    Ok(outPatterns)
}

fn sortPatternsByComplexityWork(
    mut tpl1: &(metamodelica::Ref<DAE::Pattern>, i32),
    mut tpl2: &(metamodelica::Ref<DAE::Pattern>, i32),
) -> Result<bool> {
    let mut greater: bool;
    let mut pat1: metamodelica::Ref<DAE::Pattern>;
    let mut pat2: metamodelica::Ref<DAE::Pattern>;
    let mut i1: i32;
    let mut i2: i32;
    let mut c1: i32;
    let mut c2: i32;
    (pat1, i1) = tpl1.clone();
    (pat2, i2) = tpl2.clone();
    (_, c1) = traversePattern(pat1, &patternComplexity, 0)?;
    (_, c2) = traversePattern(pat2, &patternComplexity, 0)?;
    greater = if (c1 == c2) {
        i1 > i2
    } else {
        if (c2 == 0) {
            false
        } else {
            if (c1 == 0) { true } else { c1 > c2 }
        }
    };
    Ok(greater)
}

fn patternComplexity(
    mut inPat: metamodelica::Ref<DAE::Pattern>,
    mut inComplexity: i32,
) -> Result<(metamodelica::Ref<DAE::Pattern>, i32)> {
    let mut outPat: metamodelica::Ref<DAE::Pattern> = inPat.clone();
    let mut i: i32 = inComplexity;
    i = (match &*inPat {
        DAE::Pattern::PAT_CONSTANT { exp, .. } => {
            (_, i) = Expression::traverseExpBottomUp(
                exp.clone(),
                &fnptr!(constantComplexity, metamodelica::Ref<DAE::Exp>, i32),
                i,
            )?;
            i
        }
        DAE::Pattern::PAT_CONS { .. } => i + 5,
        DAE::Pattern::PAT_CALL {
            knownSingleton: false, ..
        } => i + 5,
        DAE::Pattern::PAT_SOME { .. } => i + 5,
        _ => i,
    });
    Ok((outPat, i))
}

fn constantComplexity(mut inExp: metamodelica::Ref<DAE::Exp>, mut ii: i32) -> (metamodelica::Ref<DAE::Exp>, i32) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut oi: i32;
    (outExp, oi) = (::match_deref::match_deref! { match &(inExp) {
        e @ Deref @ DAE::Exp::SCONST { string: r#str } => {
            let mut i = ii;
            (e.clone(), i + 5 + ((r#str).len() as i32))
        },
        e @ Deref @ DAE::Exp::ICONST { integer: _ } => {
            let mut i = ii;
            (e.clone(), i + 1)
        },
        e @ Deref @ DAE::Exp::BCONST { bool: _ } => {
            let mut i = ii;
            (e.clone(), i + 1)
        },
        e @ Deref @ DAE::Exp::RCONST { real: _ } => {
            let mut i = ii;
            (e.clone(), i + 2)
        },
        e => {
            let mut i = ii;
            (e.clone(), i + 5)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, oi)
}

fn addEnvKnownAsBindings(
    mut inPat: metamodelica::Ref<DAE::Pattern>,
    mut inEnv: FCore::Graph,
) -> Result<(metamodelica::Ref<DAE::Pattern>, FCore::Graph)> {
    let mut pat: metamodelica::Ref<DAE::Pattern> = inPat;
    let mut env: FCore::Graph = inEnv;
    env = (match &*pat {
        DAE::Pattern::PAT_AS { pat: __pat_pat, .. } => {
            addEnvKnownAsBindings2(&pat, env, &(findFirstNonAsPattern(__pat_pat.clone())))?
        }
        _ => env,
    });
    Ok((pat, env))
}

fn addEnvKnownAsBindings2(
    mut inPat: &metamodelica::Ref<DAE::Pattern>,
    mut inEnv: FCore::Graph,
    mut firstPattern: &metamodelica::Ref<DAE::Pattern>,
) -> Result<FCore::Graph> {
    let mut env: FCore::Graph = inEnv;
    env = (::match_deref::match_deref! { match (inPat, firstPattern) {
        (Deref @ DAE::Pattern::PAT_AS { id, attr, .. }, Deref @ DAE::Pattern::PAT_CALL { index, typeVars, fields, knownSingleton, name, .. }) => {
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            path = AbsynUtil::stripLast(name)?;
            ty = metamodelica::Ref::new(DAE::Type::T_METARECORD { path: name.clone(), utPath: path, typeVars: typeVars.clone(), index: index.clone(), fields: fields.clone(), knownSingleton: knownSingleton.clone() });
            env = FGraph::mkComponentNode(env, metamodelica::Ref::new(DAE::Var { name: id.clone(), attributes: attr.clone(), ty: ty, binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(SCode::Element::COMPONENT { name: id.clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultVarAttr.clone(), typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: name.clone(), arrayDim: None }), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_dump::FCore::Status::VAR_DAE, FGraph::empty())?;
            env
        },
        _ => {
            env
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(env)
}

fn findFirstNonAsPattern(mut inPattern: metamodelica::Ref<DAE::Pattern>) -> metamodelica::Ref<DAE::Pattern> {
    let mut outPattern: metamodelica::Ref<DAE::Pattern>;
    outPattern = (match &*inPattern {
        DAE::Pattern::PAT_AS {
            pat: __esc_outPattern, ..
        } => {
            outPattern = (*__esc_outPattern).clone();
            findFirstNonAsPattern(outPattern.clone())
        }
        _ => inPattern,
    });
    outPattern
}

fn getInputAsBinding(
    mut inExp: &metamodelica::Ref<Absyn::Exp>,
) -> (
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
) {
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    let mut aliases: metamodelica::List<ArcStr>;
    let mut aliasesAndCrefs: metamodelica::List<ArcStr>;
    (exp, aliases, aliasesAndCrefs) = (::match_deref::match_deref! { match inExp {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: Deref @ metamodelica::ListNode::Nil } } => {
            (inExp.clone(), metamodelica::nil(), list![id.clone()])
        },
        Deref @ Absyn::Exp::AS { id, exp: __esc_exp } => {
            exp = (*__esc_exp).clone();
            (exp, aliases, aliasesAndCrefs) = getInputAsBinding(metamodelica::AsArg::as_arg(&exp));
            (exp.clone(), metamodelica::cons(id.clone(), aliases), metamodelica::cons(id.clone(), aliasesAndCrefs))
        },
        _ => {
            (inExp.clone(), metamodelica::nil(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, aliases, aliasesAndCrefs)
}

fn addPatternAliasesList(
    mut inPatterns: &metamodelica::List<metamodelica::Ref<DAE::Pattern>>,
    mut inAliases: metamodelica::List<metamodelica::List<ArcStr>>,
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Pattern>>, FCore::Cache)> {
    let mut outPatterns: metamodelica::List<metamodelica::Ref<DAE::Pattern>> = metamodelica::nil();
    let mut outCache: FCore::Cache = inCache;
    let mut aliases: metamodelica::List<ArcStr>;
    let mut rest_aliases: metamodelica::List<metamodelica::List<ArcStr>> = inAliases;
    for mut pat in &**inPatterns {
        let mut pat = pat.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_aliases) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        aliases = metamodelica::Own::own(__pa0);
        rest_aliases = metamodelica::Own::own(__pa1);
        (pat, outCache) = addPatternAliases(pat, &aliases, outCache, inEnv)?;
        outPatterns = metamodelica::cons(pat, outPatterns);
    }
    outPatterns = outPatterns.reverse();
    Ok((outPatterns, outCache))
}

fn addPatternAliases(
    mut inPattern: metamodelica::Ref<DAE::Pattern>,
    mut inAliases: &metamodelica::List<ArcStr>,
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
) -> Result<(metamodelica::Ref<DAE::Pattern>, FCore::Cache)> {
    let mut pat: metamodelica::Ref<DAE::Pattern> = inPattern;
    let mut outCache: FCore::Cache = inCache;
    let mut attr: metamodelica::Ref<DAE::Attributes>;
    for mut alias in &**inAliases {
        let (__pa0, __t2, _, _, _, _) = Lookup::lookupIdent(outCache, inEnv, alias.clone())?;
        let __arc3 = __t2.clone();
        let DAE::TYPES_VAR { attributes: __pa1, .. } = &*__arc3;
        outCache = metamodelica::Own::own(__pa0);
        attr = metamodelica::Own::own(__pa1);
        pat = metamodelica::Ref::new(DAE::Pattern::PAT_AS {
            id: alias.clone(),
            ty: None,
            attr: attr,
            pat: pat,
        });
    }
    Ok((pat, outCache))
}

fn addAliasesToEnv<'__b>(
    mut inEnv: FCore::Graph,
    mut inTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inAliases: metamodelica::List<metamodelica::List<ArcStr>>,
    mut info: &'__b SourceInfo,
) -> Result<FCore::Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inEnv.clone(), inTypes.clone(), inAliases)) {
            (_, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(inEnv)
            },
            (_, Deref @ metamodelica::ListNode::Cons { head: _, tail: tys }, Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Nil, tail: aliases }) => {
                { (inEnv, inTypes, inAliases, info) = (inEnv, tys.clone(), aliases.clone(), info); continue '__tco; }
            },
            (env, Deref @ metamodelica::ListNode::Cons { head: ty, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: id, tail: rest }, tail: aliases }) => {
                let mut attr: metamodelica::Ref<DAE::Attributes>;
                let mut env = (*env).clone();
                attr = DAE::dummyAttrInput().clone();
                env = FGraph::mkComponentNode(env.clone(), metamodelica::Ref::new(DAE::Var { name: id.clone(), attributes: attr, ty: ty.clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(SCode::Element::COMPONENT { name: id.clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultVarAttr.clone(), typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("$dummy") }), arrayDim: None }), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: info.clone() }), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_dump::FCore::Status::VAR_DAE, FGraph::empty())?;
                { (inEnv, inTypes, inAliases, info) = (env.clone(), inTypes, metamodelica::cons(rest.clone(), aliases.clone()), info); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn statementListFindDeadStoreRemoveEmptyStatements(
    mut inBody: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut localsTree: metamodelica::Ref<AvlSetString::Tree>,
    mut inUseTree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::Ref<AvlSetString::Tree>,
)> {
    let mut body: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut useTree: metamodelica::Ref<AvlSetString::Tree>;
    (body, useTree) = List::map1Fold(&(inBody.reverse()), &statementFindDeadStore, localsTree, inUseTree)?;
    body = List::select(
        body,
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Statement>| isNotDummyStatement(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Statement>) -> Result<bool> + 'static>),
    )?;
    body = body.reverse();
    Ok((body, useTree))
}

fn statementFindDeadStore(
    mut inStatement: metamodelica::Ref<DAE::Statement>,
    mut localsTree: metamodelica::Ref<AvlSetString::Tree>,
    mut inUseTree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<(metamodelica::Ref<DAE::Statement>, metamodelica::Ref<AvlSetString::Tree>)> {
    let mut outStatement: metamodelica::Ref<DAE::Statement>;
    let mut useTree: metamodelica::Ref<AvlSetString::Tree>;
    (outStatement, useTree) = (::match_deref::match_deref! { match &(inStatement.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { type_: ty, exp1: lhs, exp, source: source @ Deref @ DAE::ElementSource { info, .. } } => {
            let mut lhs = (*lhs).clone();
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), &useLocalCref, inUseTree)?;
            (lhs, _) = Expression::traverseExpBottomUp(lhs.clone(), &fnptr!(checkDefUse, metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<AvlSetString::Tree>, metamodelica::Ref<AvlSetString::Tree>, SourceInfo)), (localsTree, useTree.clone(), info.clone()))?;
            outStatement = Algorithm::makeAssignmentNoTypeCheck(ty.clone(), lhs.clone(), exp.clone(), source.clone());
            (outStatement, useTree)
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: ty, expExpLst: exps, exp, source: source @ Deref @ DAE::ElementSource { info, .. } } => {
            let mut exps = (*exps).clone();
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), &useLocalCref, inUseTree)?;
            let __pa0 = ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(metamodelica::Ref::new(DAE::Exp::TUPLE { PR: exps.clone() }), &fnptr!(checkDefUse, metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<AvlSetString::Tree>, metamodelica::Ref<AvlSetString::Tree>, SourceInfo)), (localsTree, useTree.clone(), info.clone()))?) {
                (Deref @ DAE::Exp::TUPLE { PR: __pa0 }, _) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            exps = metamodelica::Own::own(__pa0);
            outStatement = Algorithm::makeTupleAssignmentNoTypeCheck(ty.clone(), exps.clone(), exp.clone(), source.clone())?;
            (outStatement, useTree)
        },
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_: ty, lhs, exp, source: source @ Deref @ DAE::ElementSource { info, .. } } => {
            let mut lhs = (*lhs).clone();
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), &useLocalCref, inUseTree)?;
            (lhs, _) = Expression::traverseExpBottomUp(lhs.clone(), &fnptr!(checkDefUse, metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<AvlSetString::Tree>, metamodelica::Ref<AvlSetString::Tree>, SourceInfo)), (localsTree, useTree.clone(), info.clone()))?;
            outStatement = Algorithm::makeArrayAssignmentNoTypeCheck(ty.clone(), lhs.clone(), exp.clone(), source.clone());
            (outStatement, useTree)
        },
        Deref @ DAE::Statement::STMT_IF { exp, statementLst: body, else_, source } => {
            let mut elseTree: metamodelica::Ref<AvlSetString::Tree>;
            let mut body = (*body).clone();
            let mut else_ = (*else_).clone();
            (else_, elseTree) = elseFindDeadStore(metamodelica::AsArg::as_arg(&else_), &localsTree, &inUseTree)?;
            (body, useTree) = statementListFindDeadStoreRemoveEmptyStatements(body.clone(), localsTree, inUseTree)?;
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), &useLocalCref, useTree)?;
            useTree = AvlSetString::join(useTree, &elseTree)?;
            (metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: body.clone(), else_: else_.clone(), source: source.clone() }), useTree)
        },
        Deref @ DAE::Statement::STMT_FOR { type_: ty, iterIsArray: b, iter: id, range: exp, statementLst: body, source, sub_iters } => {
            let mut body = (*body).clone();
            ErrorExt::setCheckpoint(literal!("Patternm.statementFindDeadStore"));
            (_, useTree) = List::map1Fold(metamodelica::AsArg::as_arg(&body), &statementFindDeadStore, localsTree.clone(), inUseTree.clone())?;
            ErrorExt::rollBack(literal!("Patternm.statementFindDeadStore"));
            (body, useTree) = statementListFindDeadStoreRemoveEmptyStatements(body.clone(), localsTree, useTree)?;
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), &useLocalCref, useTree)?;
            useTree = AvlSetString::join(useTree, &inUseTree)?;
            (metamodelica::Ref::new(DAE::Statement::STMT_FOR { type_: ty.clone(), iterIsArray: b.clone(), iter: id.clone(), range: exp.clone(), statementLst: body.clone(), source: source.clone(), sub_iters: sub_iters.clone() }), useTree)
        },
        Deref @ DAE::Statement::STMT_WHILE { exp, statementLst: body, source } => {
            let mut body = (*body).clone();
            ErrorExt::setCheckpoint(literal!("Patternm.statementFindDeadStore"));
            (_, useTree) = List::map1Fold(metamodelica::AsArg::as_arg(&body), &statementFindDeadStore, localsTree.clone(), inUseTree.clone())?;
            ErrorExt::rollBack(literal!("Patternm.statementFindDeadStore"));
            (body, useTree) = statementListFindDeadStoreRemoveEmptyStatements(body.clone(), localsTree, useTree)?;
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), &useLocalCref, useTree)?;
            useTree = AvlSetString::join(useTree, &inUseTree)?;
            (metamodelica::Ref::new(DAE::Statement::STMT_WHILE { exp: exp.clone(), statementLst: body.clone(), source: source.clone() }), useTree)
        },
        Deref @ DAE::Statement::STMT_PARFOR { .. } => {
            return Err("fail")
        },
        Deref @ DAE::Statement::STMT_ASSERT { cond, msg, level, .. } => {
            (_, useTree) = Expression::traverseExpBottomUp(cond.clone(), &useLocalCref, inUseTree)?;
            (_, useTree) = Expression::traverseExpBottomUp(msg.clone(), &useLocalCref, useTree)?;
            (_, useTree) = Expression::traverseExpBottomUp(level.clone(), &useLocalCref, useTree)?;
            (inStatement, useTree)
        },
        Deref @ DAE::Statement::STMT_TERMINATE { msg: exp, .. } => {
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), &useLocalCref, AvlSetString::new())?;
            (inStatement, useTree)
        },
        Deref @ DAE::Statement::STMT_WHEN { .. } => {
            return Err("fail")
        },
        Deref @ DAE::Statement::STMT_REINIT { .. } => {
            return Err("fail")
        },
        Deref @ DAE::Statement::STMT_NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fail" }, .. }, .. } => {
            (inStatement, AvlSetString::new())
        },
        Deref @ DAE::Statement::STMT_RETURN { .. } => {
            (inStatement, AvlSetString::new())
        },
        Deref @ DAE::Statement::STMT_NORETCALL { exp, .. } => {
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), &useLocalCref, inUseTree)?;
            (inStatement, useTree)
        },
        Deref @ DAE::Statement::STMT_BREAK { .. } => {
            (inStatement, inUseTree)
        },
        Deref @ DAE::Statement::STMT_CONTINUE { .. } => {
            (inStatement, inUseTree)
        },
        Deref @ DAE::Statement::STMT_ARRAY_INIT { .. } => {
            (inStatement, inUseTree)
        },
        Deref @ DAE::Statement::STMT_FAILURE { body, source } => {
            let mut body = (*body).clone();
            (body, useTree) = statementListFindDeadStoreRemoveEmptyStatements(body.clone(), localsTree, inUseTree)?;
            (metamodelica::Ref::new(DAE::Statement::STMT_FAILURE { body: body.clone(), source: source.clone() }), useTree)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outStatement, useTree))
}

fn elseFindDeadStore(
    mut inElse: &metamodelica::Ref<DAE::Else>,
    mut localsTree: &metamodelica::Ref<AvlSetString::Tree>,
    mut inUseTree: &metamodelica::Ref<AvlSetString::Tree>,
) -> Result<(metamodelica::Ref<DAE::Else>, metamodelica::Ref<AvlSetString::Tree>)> {
    let mut outElse: metamodelica::Ref<DAE::Else>;
    let mut useTree: metamodelica::Ref<AvlSetString::Tree>;
    (outElse, useTree) = (match &**inElse {
        DAE::Else::NOELSE { .. } => (inElse.clone(), inUseTree.clone()),
        DAE::Else::ELSEIF {
            exp,
            statementLst: body,
            else_,
        } => {
            let mut elseTree: metamodelica::Ref<AvlSetString::Tree>;
            let mut body = (*body).clone();
            let mut else_ = (*else_).clone();
            (body, useTree) =
                statementListFindDeadStoreRemoveEmptyStatements(body.clone(), localsTree.clone(), inUseTree.clone())?;
            (_, useTree) = Expression::traverseExpBottomUp(exp.clone(), &useLocalCref, useTree)?;
            (else_, elseTree) = elseFindDeadStore(metamodelica::AsArg::as_arg(&else_), localsTree, inUseTree)?;
            useTree = AvlSetString::join(useTree, &elseTree)?;
            else_ = metamodelica::Ref::new(DAE::Else::ELSEIF {
                exp: exp.clone(),
                statementLst: body.clone(),
                else_: else_.clone(),
            });
            (else_.clone(), useTree)
        }
        DAE::Else::ELSE { statementLst: body } => {
            let mut else_: metamodelica::Ref<DAE::Else>;
            let mut body = (*body).clone();
            (body, useTree) =
                statementListFindDeadStoreRemoveEmptyStatements(body.clone(), localsTree.clone(), inUseTree.clone())?;
            else_ = metamodelica::Ref::new(DAE::Else::ELSE {
                statementLst: body.clone(),
            });
            (else_, useTree)
        }
    });
    Ok((outElse, useTree))
}

fn isNotDummyStatement(mut statement: &metamodelica::Ref<DAE::Statement>) -> Result<bool> {
    let mut b: bool;
    b = Algorithm::isNotDummyStatement(statement)?;
    Error::assertionOrAddSourceMessage(
        b || !(Flags::isSet(Flags::PATTERNM_ALL_INFO.clone())?),
        &(Error::META_DEAD_CODE.clone()),
        list![literal!("Statement optimised away")],
        &(ElementSource::getElementSourceFileInfo(Algorithm::getStatementSource(statement)?)),
    )?;
    Ok(b)
}

fn makeTupleFromMetaTuple(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>)> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    (exp, ty) = (::match_deref::match_deref! { match &((inExp.clone(), inType.clone())) {
        (Deref @ DAE::Exp::META_TUPLE { listExp: exps }, Deref @ DAE::Type::T_METATUPLE { types: tys }) => {
            let mut tys2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut exps = (*exps).clone();
            tys2 = List::map(tys.clone(), &Types::unboxedType)?;
            (exps, tys2) = Types::matchTypeTuple(metamodelica::AsArg::as_arg(&exps), metamodelica::AsArg::as_arg(&tys), &tys2, false)?;
            (metamodelica::Ref::new(DAE::Exp::TUPLE { PR: exps.clone() }), metamodelica::Ref::new(DAE::Type::T_TUPLE { types: tys2, names: None }))
        },
        _ => {
            (inExp, inType)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, ty))
}

fn convertExpToPatterns(mut inExp: metamodelica::Ref<Absyn::Exp>) -> metamodelica::List<metamodelica::Ref<Absyn::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp.clone()) {
            Deref @ Absyn::Exp::EXPRESSIONCOMMENT { exp, .. } => {
                { inExp = exp.clone(); continue '__tco; }
            },
            Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil } } => {
                { inExp = exp.clone(); continue '__tco; }
            },
            Deref @ Absyn::Exp::TUPLE { expressions: __inExp_expressions } => {
                return __inExp_expressions.clone()
            },
            _ => {
                return list![inExp]
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn unboxSwitchType(
    mut elabMatchTy: DAE::MatchType,
    mut elabExps: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<DAE::MatchType> {
    let mut elabMatchTy: DAE::MatchType = elabMatchTy;
    let mut idx: i32;
    let mut hash_mod: i32;
    let mut ty: metamodelica::Ref<DAE::Type>;
    elabMatchTy = (::match_deref::match_deref! { match &(elabMatchTy.clone()) {
        DAE::MatchType::MATCH { switch: Some((__esc_idx, __esc_ty @ Deref @ DAE::Type::T_ENUMERATION { .. }, __esc_hash_mod)) } if (Types::isBoxedType(&(Expression::r#typeof((elabExps).head().cloned()?)?))) => {
            idx = (*__esc_idx).clone();
            ty = (*__esc_ty).clone();
            hash_mod = (*__esc_hash_mod).clone();
            DAE::MatchType::MATCH { switch: Some((idx.clone(), metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: ty.clone() }), hash_mod.clone())) }
        },
        _ => elabMatchTy,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elabMatchTy)
}
