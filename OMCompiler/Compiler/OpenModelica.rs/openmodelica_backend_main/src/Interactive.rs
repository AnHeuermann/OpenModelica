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

use crate::CevalScript;
use crate::InteractiveUtil;
use crate::NFApi;
use crate::Refactor;
use crate::StaticScript;
use openmodelica_ast::Absyn;
use openmodelica_ast::GlobalScript;
use openmodelica_backend::GlobalScriptDump;
use openmodelica_backend::SymbolTable;
use openmodelica_error::ErrorExt;
use openmodelica_frontend::Builtin;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::ConnectionGraph;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::InnerOuter;
use openmodelica_frontend::Inst;
use openmodelica_frontend::InstHashTable;
use openmodelica_frontend::InstUtil;
use openmodelica_frontend::InteractiveTypes;
use openmodelica_frontend::Lookup;
use openmodelica_frontend::Mod;
use openmodelica_frontend::Static;
use openmodelica_frontend::UnitAbsyn;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::MetaUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_loader::Parser;
use openmodelica_program_util::ProgramUtil;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Print;
use openmodelica_util::Settings;
use openmodelica_util::StackOverflow;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

//public imports
// protected imports
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum AnnotationType {
    ICON_ANNOTATION,
    DIAGRAM_ANNOTATION,
}
impl metamodelica::gc::MMTrace for AnnotationType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            AnnotationType::ICON_ANNOTATION => Ok(()),
            AnnotationType::DIAGRAM_ANNOTATION => Ok(()),
        }
    }
}
pub use self::AnnotationType::{DIAGRAM_ANNOTATION, ICON_ANNOTATION};

/// Used by buildEnvForGraphicProgram to avoid excessive work.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum GraphicEnvCache {
    GRAPHIC_ENV_NO_CACHE {
        program: Absyn::Program,
        modelPath: metamodelica::Ref<Absyn::Path>,
    },
    GRAPHIC_ENV_PARTIAL_CACHE {
        program: Absyn::Program,
        modelPath: metamodelica::Ref<Absyn::Path>,
        cache: FCore::Cache,
        env: FCore::Graph,
    },
    GRAPHIC_ENV_FULL_CACHE {
        program: Absyn::Program,
        modelPath: metamodelica::Ref<Absyn::Path>,
        cache: FCore::Cache,
        env: FCore::Graph,
    },
}
impl metamodelica::gc::MMTrace for GraphicEnvCache {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            GraphicEnvCache::GRAPHIC_ENV_NO_CACHE { program, modelPath } => {
                metamodelica::gc::MMTrace::mm_accept(program, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modelPath, __mmv)?;
                Ok(())
            }
            GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE {
                program,
                modelPath,
                cache,
                env,
            } => {
                metamodelica::gc::MMTrace::mm_accept(program, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modelPath, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cache, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(env, __mmv)?;
                Ok(())
            }
            GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE {
                program,
                modelPath,
                cache,
                env,
            } => {
                metamodelica::gc::MMTrace::mm_accept(program, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modelPath, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cache, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(env, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for GraphicEnvCache {
    fn default() -> Self {
        Self::GRAPHIC_ENV_NO_CACHE {
            program: Default::default(),
            modelPath: Default::default(),
        }
    }
}
pub use self::GraphicEnvCache::{GRAPHIC_ENV_FULL_CACHE, GRAPHIC_ENV_NO_CACHE, GRAPHIC_ENV_PARTIAL_CACHE};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum Access {
    hide = 1,
    icon = 2,
    documentation = 3,
    diagram = 4,
    nonPackageText = 5,
    nonPackageDuplicate = 6,
    packageText = 7,
    packageDuplicate = 8,
    all = 9,
}
impl PartialOrd for Access {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Access {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Access {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) fn evaluate(mut inStatements: &GlobalScript::Statements, mut verbose: bool) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut semicolon: bool;
    let mut res: ArcStr;
    let mut resl: metamodelica::List<ArcStr> = metamodelica::nil();
    for mut stmt in &*inStatements.interactiveStmtLst.clone() {
        semicolon = inStatements.semicolon.clone();
        showStatement(stmt.clone(), semicolon, true)?;
        res = evaluate2(metamodelica::AsArg::as_arg(&stmt))?;
        if getEcho() && (verbose || !(semicolon)) {
            res = stringAppend(res, literal!("\n"));
            resl = metamodelica::cons(res, resl);
        }
        showStatement(stmt.clone(), semicolon, false)?;
    }
    outString = stringAppendList(Dangerous::listReverseInPlace(resl));
    Ok(outString)
}

pub(crate) fn evaluateToStdOut(mut statements: &GlobalScript::Statements, mut verbose: bool) -> Result<()> {
    let mut semicolon: bool;
    let mut res: ArcStr;
    semicolon = statements.semicolon.clone();
    for mut stmt in &*statements.interactiveStmtLst.clone() {
        showStatement(stmt.clone(), semicolon, true)?;
        res = evaluate2(metamodelica::AsArg::as_arg(&stmt))?;
        if getEcho() && (verbose || !(semicolon)) {
            metamodelica::print(res);
            metamodelica::print(literal!("\n"));
        }
        showStatement(stmt.clone(), semicolon, false)?;
    }
    Ok(())
}

pub(crate) fn evaluateFork(mut inTpl: &(ArcStr, metamodelica::Ref<SymbolTable::SymbolTable>)) -> bool {
    let mut b: bool;
    b = 'mc: {
        let __mc_input = inTpl;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mosfile, st) => {
                    let mut statements: GlobalScript::Statements;
                    SymbolTable::reset()?;
                    SymbolTable::setAbsyn(st.ast.clone())?;
                    SymbolTable::setSCode(st.explodedAst.clone());
                    { let __v = None; openmodelica_util::Globals::instOnlyForcedFunctions.with(|__root| *__root.borrow_mut() = __v) };
                    statements = Parser::parseexp(mosfile.clone())?;
                    evaluateToStdOut(&statements, true)?;
                    metamodelica::print(Error::printMessagesStr(false));
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
                    metamodelica::print(Error::printMessagesStr(false));
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

fn showStatement(mut s: GlobalScript::Statement, mut semicolon: bool, mut start: bool) -> Result<()> {
    let mut testsuite: bool;
    if !(Flags::isSet(Flags::SHOW_STATEMENT.clone())?) {
        return Ok(());
    }
    testsuite = Testsuite::isRunning()?;
    let () = 'mc: {
        let __mc_input = (start, testsuite);
        if let Ok(__v) = (|| -> Result<_> {
            let (true, true) = __mc_input.clone() else {
                return Err("nomatch");
            };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Evaluating: "));
                __mm_s.push_str(&*printIstmtStr(
                    &(GlobalScript::Statements {
                        interactiveStmtLst: list![s.clone()],
                        semicolon: semicolon,
                    }),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            System::fflush();
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (false, true) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (true, false) = __mc_input.clone() else {
                return Err("nomatch");
            };
            System::realtimeTick(ClockIndexes::RT_CLOCK_SHOW_STATEMENT.clone())?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Evaluating:   > "));
                __mm_s.push_str(&*printIstmtStr(
                    &(GlobalScript::Statements {
                        interactiveStmtLst: list![s.clone()],
                        semicolon: semicolon,
                    }),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            System::fflush();
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (false, false) = __mc_input.clone() else {
                return Err("nomatch");
            };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Evaluated:    < "));
                __mm_s.push_str(&*realString(System::realtimeTock(
                    ClockIndexes::RT_CLOCK_SHOW_STATEMENT.clone(),
                )?));
                __mm_s.push_str(&*literal!(" / "));
                __mm_s.push_str(&*printIstmtStr(
                    &(GlobalScript::Statements {
                        interactiveStmtLst: list![s.clone()],
                        semicolon: semicolon,
                    }),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            System::fflush();
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn getEcho() -> bool {
    let mut outBoolean: bool;
    outBoolean = 0 != Settings::getEcho();
    outBoolean
}

fn evaluate2(mut inStatement: &GlobalScript::Statement) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut r#str: ArcStr;
    let mut str_1: ArcStr;
    let mut algitem: metamodelica::Ref<Absyn::AlgorithmItem> =
        <metamodelica::Ref<Absyn::AlgorithmItem> as ::std::default::Default>::default();
    let mut exp: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let __cp0 = metamodelica::heap_limit::catch(|| -> Result<bool> {
        outString = (::match_deref::match_deref! { match &(inStatement) {
            GlobalScript::Statement::IALG { algItem: __esc_algitem @ Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { .. } } => {
                algitem = (*__esc_algitem).clone();
                InstHashTable::init()?;
                evaluateAlgItem(metamodelica::AsArg::as_arg(&algitem))?
            },
            GlobalScript::Statement::IEXP { exp: __esc_exp, info: __esc_info } => {
                exp = (*__esc_exp).clone();
                info = (*__esc_info).clone();
                InstHashTable::init()?;
                evaluateExprToStr(exp.clone(), info.clone())
            },
            _ => return Err("match: no arm matched"),
        } });
        Ok(false)
    });
    match __cp0 {
        Ok(__returned) => {
            if __returned? {
                return Ok(outString.clone());
            }
        }
        Err(_) => {
            let _rearm = metamodelica::heap_limit::RearmOnDrop;
            r#str = literal!("");
            str_1 = literal!("");
            GCExt::gcollect();
            r#str = StackOverflow::getReadableMessage(literal!("\n"))?;
            if Testsuite::isRunning()? {
                Error::clearCurrentComponent()?;
            }
            Error::addMessage(
                Error::STACK_OVERFLOW_DETAILED.clone(),
                list![GlobalScriptDump::printIstmtStr(inStatement)?, r#str.clone()],
            )?;
            Error::clearCurrentComponent()?;
            outString = literal!("");
        }
    }
    Ok(outString)
}

fn evaluateAlgItem(mut alg: &metamodelica::Ref<Absyn::AlgorithmItem>) -> Result<ArcStr> {
    let mut result: ArcStr;
    result = (match &**alg {
        Absyn::AlgorithmItem::ALGORITHMITEM {
            algorithm_: __alg_algorithm_,
            info: __alg_info,
            ..
        } => evaluateAlgStmt(metamodelica::AsArg::as_arg(&__alg_algorithm_), __alg_info.clone())?,
        _ => literal!(""),
    });
    Ok(result)
}

fn evaluateAlgStmt(mut alg: &metamodelica::Ref<Absyn::Algorithm>, mut info: SourceInfo) -> Result<ArcStr> {
    let mut result: ArcStr;
    let mut env: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut cond: metamodelica::Ref<Absyn::Exp>;
    let mut msg: metamodelica::Ref<Absyn::Exp>;
    let mut exp: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let mut dcond: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut dmsg: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut dexp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut ident: ArcStr;
    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
    let mut value: metamodelica::Ref<Values::Value> = metamodelica::Ref::new(Values::Value::META_FAIL);
    let mut values: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
    let mut ty: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_NORETCALL);
    let mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let mut dsubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
    let mut expl: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut prop: DAE::Properties = <DAE::Properties as ::std::default::Default>::default();
    let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut branches: metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    )> = metamodelica::nil();
    let mut startv: metamodelica::Ref<Values::Value> = metamodelica::Ref::new(Values::Value::META_FAIL);
    let mut stepv: metamodelica::Ref<Values::Value> = metamodelica::Ref::new(Values::Value::META_FAIL);
    let mut stopv: metamodelica::Ref<Values::Value> = metamodelica::Ref::new(Values::Value::META_FAIL);
    let mut starte: metamodelica::Ref<Absyn::Exp>;
    let mut stepe: metamodelica::Ref<Absyn::Exp>;
    let mut stope: metamodelica::Ref<Absyn::Exp>;
    result = 'mc: {
        let __mc_input = &**alg;
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: cond, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. } } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut dcond: metamodelica::Ref<DAE::Exp> = dcond.clone();
                    let mut env: FCore::Graph = env.clone();
                    env = SymbolTable::buildEnv()?;
                    (cache, dcond, _) = StaticScript::elabExp(FCore::emptyCache(), env.clone(), cond.clone(), true, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    ::match_deref::match_deref! { match &(CevalScript::ceval(cache.clone(), env.clone(), dcond.clone(), true, Absyn::Msg::MSG { info: info.clone() }, 0)?) {
                        (_, Deref @ Values::Value::BOOL { boolean: true }) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok((literal!(""), cache.clone(), dcond.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            dcond = __wb1;
            env = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: msg, tail: Deref @ metamodelica::ListNode::Nil } }, .. } } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut dmsg: metamodelica::Ref<DAE::Exp> = dmsg.clone();
                    let mut env: FCore::Graph = env.clone();
                    let mut r#str: ArcStr = r#str.clone();
                    env = SymbolTable::buildEnv()?;
                    (cache, dmsg, _) = StaticScript::elabExp(FCore::emptyCache(), env.clone(), msg.clone(), true, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::ceval(cache.clone(), env.clone(), dmsg.clone(), true, Absyn::Msg::MSG { info: info.clone() }, 0)?) {
                        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    r#str = metamodelica::Own::own(__pa0);
                    Ok((r#str.clone(), cache.clone(), dmsg.clone(), env.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            dmsg = __wb1;
            env = __wb2;
            r#str = __wb3;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_NORETCALL { .. } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut dexp: metamodelica::Ref<DAE::Exp> = dexp.clone();
                    let mut env: FCore::Graph = env.clone();
                    let mut exp: metamodelica::Ref<Absyn::Exp> = exp.clone();
                    env = SymbolTable::buildEnv()?;
                    exp = metamodelica::Ref::new(Absyn::Exp::CALL { function_: var_field!((**alg).functionCall, Absyn::Algorithm::ALG_NORETCALL).clone(), functionArgs: var_field!((**alg).functionArgs, Absyn::Algorithm::ALG_NORETCALL).clone(), typeVars: metamodelica::nil() });
                    (cache, dexp, _) = StaticScript::elabExp(FCore::emptyCache(), env.clone(), exp.clone(), true, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    CevalScript::ceval(cache.clone(), env.clone(), dexp.clone(), true, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    Ok((literal!(""), cache.clone(), dexp.clone(), env.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            dexp = __wb1;
            env = __wb2;
            exp = __wb3;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_ASSIGN { assignComponent: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: ident, subscripts: Deref @ metamodelica::ListNode::Nil } }, value: Deref @ Absyn::Exp::CREF { componentRef: cr } } => {
                    let mut r#str: ArcStr = r#str.clone();
                    let mut ty: metamodelica::Ref<DAE::Type> = ty.clone();
                    let mut value: metamodelica::Ref<Values::Value> = value.clone();
                    value = getVariableValueLst(&(AbsynUtil::pathToStringList(&(AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&cr))?))), &(SymbolTable::getVars()))?;
                    r#str = ValuesDump::valString(&value)?;
                    ty = Types::typeOfValue(value.clone())?;
                    SymbolTable::addVar(&(metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: ident.clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() })), value.clone(), &(FGraph::empty()))?;
                    Ok((r#str.clone(), r#str.clone(), ty.clone(), value.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            ty = __wb1;
            value = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_ASSIGN { assignComponent: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: ident, subscripts: subs } }, .. } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut dexp: metamodelica::Ref<DAE::Exp> = dexp.clone();
                    let mut dsubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = dsubs.clone();
                    let mut env: FCore::Graph = env.clone();
                    let mut r#str: ArcStr = r#str.clone();
                    let mut ty: metamodelica::Ref<DAE::Type> = ty.clone();
                    let mut value: metamodelica::Ref<Values::Value> = value.clone();
                    env = SymbolTable::buildEnv()?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(StaticScript::elabExp(FCore::emptyCache(), env.clone(), var_field!((**alg).value, Absyn::Algorithm::ALG_ASSIGN).clone(), true, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: _, constFlag: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    dexp = metamodelica::Own::own(__pa1);
                    (_, value) = CevalScript::ceval(cache.clone(), env.clone(), dexp.clone(), true, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    (_, dsubs, _) = Static::elabSubscripts(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&subs), true, openmodelica_frontend_types::DAE::Prefix::NOPRE, &info)?;
                    ty = Types::typeOfValue(value.clone())?;
                    r#str = ValuesDump::valString(&value)?;
                    SymbolTable::addVar(&(metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: ident.clone(), identType: ty.clone(), subscriptLst: dsubs.clone() })), value.clone(), &env)?;
                    Ok((r#str.clone(), cache.clone(), dexp.clone(), dsubs.clone(), env.clone(), r#str.clone(), ty.clone(), value.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            dexp = __wb1;
            dsubs = __wb2;
            env = __wb3;
            r#str = __wb4;
            ty = __wb5;
            value = __wb6;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_ASSIGN { assignComponent: Deref @ Absyn::Exp::TUPLE { expressions: expl }, .. } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = crefs.clone();
                    let mut dexp: metamodelica::Ref<DAE::Exp> = dexp.clone();
                    let mut env: FCore::Graph = env.clone();
                    let mut prop: DAE::Properties = prop.clone();
                    let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>> = types.clone();
                    let mut values: metamodelica::List<metamodelica::Ref<Values::Value>> = values.clone();
                    env = SymbolTable::buildEnv()?;
                    (cache, dexp, prop) = StaticScript::elabExp(FCore::emptyCache(), env.clone(), var_field!((**alg).value, Absyn::Algorithm::ALG_ASSIGN).clone(), true, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Types::getPropType(&prop)) {
                        Deref @ DAE::Type::T_TUPLE { types: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    types = metamodelica::Own::own(__pa0);
                    crefs = makeTupleCrefs(expl.clone(), types.clone(), env.clone(), cache.clone(), &info)?;
                    let __pa1 = ::match_deref::match_deref! { match &(CevalScript::ceval(cache.clone(), env.clone(), dexp.clone(), true, Absyn::Msg::MSG { info: info.clone() }, 0)?) {
                        (_, Deref @ Values::Value::TUPLE { valueLst: __pa1 }) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    values = metamodelica::Own::own(__pa1);
                    SymbolTable::addVars(crefs.clone(), values.clone(), &env)?;
                    Ok((literal!(""), cache.clone(), crefs.clone(), dexp.clone(), env.clone(), prop.clone(), types.clone(), values.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            crefs = __wb1;
            dexp = __wb2;
            env = __wb3;
            prop = __wb4;
            types = __wb5;
            values = __wb6;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_IF { .. } => {
                    let mut branches: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>)> = branches.clone();
                    branches = metamodelica::cons((var_field!((**alg).ifExp, Absyn::Algorithm::ALG_IF).clone(), var_field!((**alg).trueBranch, Absyn::Algorithm::ALG_IF).clone()), var_field!((**alg).elseIfAlgorithmBranch, Absyn::Algorithm::ALG_IF).clone());
                    branches = List::appendElt((metamodelica::Ref::new(Absyn::Exp::BOOL { value: true }), var_field!((**alg).elseBranch, Absyn::Algorithm::ALG_IF).clone()), branches.clone());
                    evaluateIfStatementLst(&branches, info.clone())?;
                    Ok((literal!(""), branches.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            branches = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_WHILE { .. } => {
                    let mut value: metamodelica::Ref<Values::Value> = value.clone();
                    value = evaluateExpr(var_field!((**alg).boolExpr, Absyn::Algorithm::ALG_WHILE).clone(), info.clone())?;
                    evaluateWhileStmt(value.clone(), var_field!((**alg).boolExpr, Absyn::Algorithm::ALG_WHILE).clone(), var_field!((**alg).whileBody, Absyn::Algorithm::ALG_WHILE).clone(), &info)?;
                    Ok((literal!(""), value.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            value = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_FOR { iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ForIterator { name: ident, guardExp: None, range: Some(Deref @ Absyn::Exp::RANGE { start: starte, step: None, stop: stope }) }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut startv: metamodelica::Ref<Values::Value> = startv.clone();
                    let mut stopv: metamodelica::Ref<Values::Value> = stopv.clone();
                    startv = evaluateExpr(starte.clone(), info.clone())?;
                    stopv = evaluateExpr(stope.clone(), info.clone())?;
                    evaluateForStmtRangeOpt(ident.clone(), startv.clone(), &(metamodelica::Ref::new(Values::Value::INTEGER { integer: 1 })), &stopv, var_field!((**alg).forBody, Absyn::Algorithm::ALG_FOR));
                    Ok((literal!(""), startv.clone(), stopv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            startv = __wb0;
            stopv = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_FOR { iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ForIterator { name: ident, guardExp: None, range: Some(Deref @ Absyn::Exp::RANGE { start: starte, step: Some(stepe), stop: stope }) }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut startv: metamodelica::Ref<Values::Value> = startv.clone();
                    let mut stepv: metamodelica::Ref<Values::Value> = stepv.clone();
                    let mut stopv: metamodelica::Ref<Values::Value> = stopv.clone();
                    startv = evaluateExpr(starte.clone(), info.clone())?;
                    stepv = evaluateExpr(stepe.clone(), info.clone())?;
                    stopv = evaluateExpr(stope.clone(), info.clone())?;
                    evaluateForStmtRangeOpt(ident.clone(), startv.clone(), &stepv, &stopv, var_field!((**alg).forBody, Absyn::Algorithm::ALG_FOR));
                    Ok((literal!(""), startv.clone(), stepv.clone(), stopv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            startv = __wb0;
            stepv = __wb1;
            stopv = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_FOR { iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ForIterator { name: ident, guardExp: None, range: Some(exp) }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut values: metamodelica::List<metamodelica::Ref<Values::Value>> = values.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(evaluateExpr(exp.clone(), info.clone())?) {
                        Deref @ Values::Value::ARRAY { valueLst: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    values = metamodelica::Own::own(__pa0);
                    evaluateForStmt(ident.clone(), &values, var_field!((**alg).forBody, Absyn::Algorithm::ALG_FOR))?;
                    Ok((literal!(""), values.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            values = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_FOR { iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ForIterator { range: Some(exp), .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = stringRepresOfExpr(exp.clone())?;
                    Error::addSourceMessage(&(Error::NOT_ARRAY_TYPE_IN_FOR_STATEMENT.clone()), list![r#str.clone()], &info)?;
                    Ok((return Err("fail"), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(result)
}

fn evaluateForStmt(
    mut iter: ArcStr,
    mut valList: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut algItemList: &metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
) -> Result<()> {
    for mut val in &**valList {
        SymbolTable::appendVar(iter.clone(), val.clone(), Types::typeOfValue(val.clone())?);
        evaluateAlgStmtLst(algItemList)?;
        SymbolTable::deleteVarFirstEntry(iter.clone())?;
    }
    Ok(())
}

fn evaluateForStmtRangeOpt(
    mut iter: ArcStr,
    mut startVal: metamodelica::Ref<Values::Value>,
    mut stepVal: &metamodelica::Ref<Values::Value>,
    mut stopVal: &metamodelica::Ref<Values::Value>,
    mut algItems: &metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
) -> () {
    let mut val: metamodelica::Ref<Values::Value>;
    val = startVal;
    if '__try0: {
        while unwrap_break_err!(ValuesUtil::safeLessEq(&val, stopVal), '__try0) {
            SymbolTable::appendVar(iter.clone(), val.clone(), unwrap_break_err!(Types::typeOfValue(val.clone()), '__try0));
            unwrap_break_err!(evaluateAlgStmtLst(algItems), '__try0);
            unwrap_break_err!(SymbolTable::deleteVarFirstEntry(iter.clone()), '__try0);
            val = unwrap_break_err!(ValuesUtil::safeIntRealOp(&val, stepVal, openmodelica_frontend_types::Values::IntRealOp::ADDOP), '__try0);
        }
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    ()
}

fn evaluateWhileStmt(
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inAbsynAlgorithmItemLst: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inValue, inExp, inAbsynAlgorithmItemLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: false }, _, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: true }, exp, algitemlst) => {
                    let mut value: metamodelica::Ref<Values::Value>;
                    evaluateAlgStmtLst(metamodelica::AsArg::as_arg(&algitemlst))?;
                    value = evaluateExpr(exp.clone(), info.clone())?;
                    evaluateWhileStmt(value.clone(), exp.clone(), algitemlst.clone(), info)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: _ }, _, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (value, exp, _) => {
                    let mut estr: ArcStr;
                    let mut tstr: ArcStr;
                    let mut vtype: metamodelica::Ref<DAE::Type>;
                    estr = stringRepresOfExpr(exp.clone())?;
                    vtype = Types::typeOfValue(value.clone())?;
                    tstr = TypesDump::unparseTypeNoAttr(&vtype)?;
                    Error::addSourceMessage(&(Error::WHILE_CONDITION_TYPE_ERROR.clone()), list![estr.clone(), tstr.clone()], info)?;
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

fn evaluatePartOfIfStatement(
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inAbsynAlgorithmItemLst: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    mut inTplAbsynExpAbsynAlgorithmItemLstLst: metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    )>,
    mut info: SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (
            inValue,
            inExp,
            inAbsynAlgorithmItemLst,
            inTplAbsynExpAbsynAlgorithmItemLstLst,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: true }, _, algitemlst, _) => {
                    evaluateAlgStmtLst(metamodelica::AsArg::as_arg(&algitemlst))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: false }, _, _, algrest) => {
                    evaluateIfStatementLst(metamodelica::AsArg::as_arg(&algrest), info.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (value, exp, _, _) => {
                    let mut estr: ArcStr;
                    let mut tstr: ArcStr;
                    let mut vtype: metamodelica::Ref<DAE::Type>;
                    estr = stringRepresOfExpr(exp.clone())?;
                    vtype = Types::typeOfValue(value.clone())?;
                    tstr = TypesDump::unparseTypeNoAttr(&vtype)?;
                    Error::addSourceMessage(&(Error::IF_CONDITION_TYPE_ERROR.clone()), list![estr.clone(), tstr.clone()], &info)?;
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

fn evaluateIfStatementLst(
    mut inTplAbsynExpAbsynAlgorithmItemLstLst: &metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    )>,
    mut info: SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inTplAbsynExpAbsynAlgorithmItemLstLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (exp, algitemlst), tail: algrest } => {
            let mut value: metamodelica::Ref<Values::Value>;
            value = evaluateExpr(exp.clone(), info.clone())?;
            evaluatePartOfIfStatement(value, exp.clone(), algitemlst.clone(), algrest.clone(), info)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn evaluateAlgStmtLst(
    mut inAbsynAlgorithmItemLst: &metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
) -> Result<()> {
    for mut algitem in &**inAbsynAlgorithmItemLst {
        evaluateAlgItem(metamodelica::AsArg::as_arg(&algitem))?;
    }
    Ok(())
}

fn evaluateExpr(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CREF { componentRef: cr } => {
                    Ok(getVariableValueLst(&(AbsynUtil::pathToStringList(&(AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&cr))?))), &(SymbolTable::getVars()))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                exp => {
                    let mut env: FCore::Graph;
                    let mut sexp: metamodelica::Ref<DAE::Exp>;
                    let mut value: metamodelica::Ref<Values::Value>;
                    let mut cache: FCore::Cache;
                    env = SymbolTable::buildEnv()?;
                    (cache, sexp, _) = StaticScript::elabExp(FCore::emptyCache(), env.clone(), exp.clone(), true, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    (_, value) = CevalScript::ceval(cache.clone(), env.clone(), sexp.clone(), true, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    Ok(value.clone())
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

fn stringRepresOfExpr(mut exp: metamodelica::Ref<Absyn::Exp>) -> Result<ArcStr> {
    let mut estr: ArcStr;
    let mut env: FCore::Graph;
    let mut sexp: metamodelica::Ref<DAE::Exp>;
    let mut prop: DAE::Properties;
    env = SymbolTable::buildEnv()?;
    (_, sexp, prop) = StaticScript::elabExp(
        FCore::emptyCache(),
        env.clone(),
        exp,
        true,
        true,
        openmodelica_frontend_types::DAE::Prefix::NOPRE,
        Absyn::dummyInfo.clone(),
    )?;
    (_, sexp, prop) = Ceval::cevalIfConstant(FCore::emptyCache(), env, sexp, prop, true, Absyn::dummyInfo.clone())?;
    estr = ExpressionBasics::printExpStr(sexp)?;
    Ok(estr)
}

fn evaluateExprToStr(mut inExp: metamodelica::Ref<Absyn::Exp>, mut info: SourceInfo) -> ArcStr {
    let mut outString: ArcStr;
    match '__try0: {
        outString = unwrap_break_err!(ValuesDump::valString(&(unwrap_break_err!(evaluateExpr(inExp.clone(), info.clone()), '__try0))), '__try0);
        Ok::<_, &'static str>((outString.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outString = __try0_o0;
        }
        Err(_) => {
            outString = literal!("");
        }
    }
    outString
}

pub(crate) fn simulateModel(
    mut className: ArcStr,
    mut stopTime: metamodelica::Real,
    mut numberOfIntervals: i32,
    mut tolerance: metamodelica::Real,
    mut method: ArcStr,
    mut simflags: ArcStr,
) -> ArcStr {
    let mut result: ArcStr;
    let mut nargs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>> = metamodelica::nil();
    let mut callExp: metamodelica::Ref<Absyn::Exp>;
    let mut env: FCore::Graph;
    let mut cache: FCore::Cache;
    let mut sexp: metamodelica::Ref<DAE::Exp>;
    let mut value: metamodelica::Ref<Values::Value>;
    match '__try0: {
        if !(stringEmpty(&simflags)) {
            nargs = metamodelica::cons(
                metamodelica::Ref::new(Absyn::NamedArg {
                    argName: literal!("simflags"),
                    argValue: metamodelica::Ref::new(Absyn::Exp::STRING {
                        value: simflags.clone(),
                    }),
                }),
                nargs.clone(),
            );
        }
        if !(stringEmpty(&method)) {
            nargs = metamodelica::cons(
                metamodelica::Ref::new(Absyn::NamedArg {
                    argName: literal!("method"),
                    argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: method.clone() }),
                }),
                nargs.clone(),
            );
        }
        if tolerance > metamodelica::OrderedFloat(0.0_f64) {
            nargs = metamodelica::cons(
                metamodelica::Ref::new(Absyn::NamedArg {
                    argName: literal!("tolerance"),
                    argValue: metamodelica::Ref::new(Absyn::Exp::REAL {
                        value: realString(tolerance),
                    }),
                }),
                nargs.clone(),
            );
        }
        if numberOfIntervals > 0 {
            nargs = metamodelica::cons(
                metamodelica::Ref::new(Absyn::NamedArg {
                    argName: literal!("numberOfIntervals"),
                    argValue: metamodelica::Ref::new(Absyn::Exp::INTEGER {
                        value: numberOfIntervals,
                    }),
                }),
                nargs.clone(),
            );
        }
        if stopTime > metamodelica::OrderedFloat(0.0_f64) {
            nargs = metamodelica::cons(
                metamodelica::Ref::new(Absyn::NamedArg {
                    argName: literal!("stopTime"),
                    argValue: metamodelica::Ref::new(Absyn::Exp::REAL {
                        value: realString(stopTime),
                    }),
                }),
                nargs.clone(),
            );
        }
        callExp = metamodelica::Ref::new(Absyn::Exp::CALL {
            function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                name: literal!("simulate"),
                subscripts: metamodelica::nil(),
            }),
            functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS {
                args: list![metamodelica::Ref::new(Absyn::Exp::CREF {
                    componentRef: AbsynUtil::pathToCref(
                        &(unwrap_break_err!(Parser::stringPath(className.clone()), '__try0))
                    )
                })],
                argNames: nargs.clone(),
            }),
            typeVars: metamodelica::nil(),
        });
        (_, env) = unwrap_break_err!(Builtin::initialGraph(FCore::emptyCache()), '__try0);
        (cache, sexp, _) = unwrap_break_err!(StaticScript::elabExp(FCore::emptyCache(), env.clone(), callExp.clone(), true, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, Absyn::dummyInfo.clone()), '__try0);
        (_, value) = unwrap_break_err!(CevalScript::ceval(cache.clone(), env.clone(), sexp.clone(), true, Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0), '__try0);
        result = unwrap_break_err!(ValuesDump::valString(&value), '__try0);
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = literal!("");
        }
    }
    result
}

fn makeTupleCrefs(
    mut inCrefs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inEnv: FCore::Graph,
    mut inCache: FCore::Cache,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outCrefs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        let __thr_src0 = inCrefs;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = inTypes;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(c), Some(t)) => {
                    let __x = makeTupleCref(c.clone(), t.clone(), inEnv.clone(), inCache.clone(), inInfo)?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(outCrefs)
}

fn makeTupleCref(
    mut inCref: metamodelica::Ref<Absyn::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inEnv: FCore::Graph,
    mut inCache: FCore::Cache,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: asubs } } => {
            let mut dsubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            (_, dsubs, _) = Static::elabSubscripts(inCache, inEnv, metamodelica::AsArg::as_arg(&asubs), true, openmodelica_frontend_types::DAE::Prefix::NOPRE, inInfo)?;
            metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: id.clone(), identType: inType, subscriptLst: dsubs })
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = Dump::printExpStr(inCref)?;
            Error::addMessage(Error::INVALID_TUPLE_CONTENT.clone(), list![r#str])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCref)
}

pub(crate) fn getTypeOfVariable(
    mut inIdent: ArcStr,
    mut inVariableLst: &metamodelica::List<InteractiveTypes::Variable>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut id: ArcStr;
    let mut tp: metamodelica::Ref<DAE::Type>;
    for mut var in &**inVariableLst {
        let InteractiveTypes::IVAR {
            varIdent: __pa0,
            type_: __pa1,
            ..
        } = &var;
        id = metamodelica::Own::own(__pa0);
        tp = metamodelica::Own::own(__pa1);
        if stringEq(&inIdent, &id) {
            outType = tp;
            return Ok(outType);
        }
    }
    return Err("fail");
    Ok(outType)
}

fn extractAllComponentreplacements(
    mut p: Absyn::Program,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut oldName: metamodelica::Ref<Absyn::ComponentRef>,
    mut newName: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<InteractiveTypes::ComponentReplacementRules> {
    let mut comp_reps: InteractiveTypes::ComponentReplacementRules;
    let mut comps: InteractiveTypes::Components;
    let mut comp_repsrules: InteractiveTypes::ComponentReplacementRules;
    match '__try0: {
        ErrorExt::setCheckpoint(literal!("Interactive.extractAllComponentreplacements"));
        comps = unwrap_break_err!(extractAllComponents(p.clone(), &classPath), '__try0);
        ErrorExt::rollBack(literal!("Interactive.extractAllComponentreplacements"));
        let false = (unwrap_break_err!(isClassReadOnly(&(unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), &p, false, false), '__try0))), '__try0))
        else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        comp_repsrules = InteractiveTypes::ComponentReplacementRules {
            componentReplacementLst: list![InteractiveTypes::ComponentReplacement {
                which1: classPath.clone(),
                the2: oldName.clone(),
                the3: newName.clone()
            }],
            the: 1,
        };
        comp_reps = unwrap_break_err!(getComponentreplacementsrules(comps.clone(), comp_repsrules.clone(), 0), '__try0);
        Ok::<_, &'static str>((comp_reps.clone(), comp_repsrules.clone(), comps.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            comp_reps = __try0_o0;
            comp_repsrules = __try0_o1;
            comps = __try0_o2;
        }
        Err(__try0_err) => {
            ErrorExt::delCheckpoint(literal!("Interactive.extractAllComponentreplacements"));
            return Err(__try0_err);
        }
    }
    Ok(comp_reps)
}

fn isClassReadOnly(mut cl: &metamodelica::Ref<Absyn::Class>) -> Result<bool> {
    let mut readOnly: bool;
    readOnly = (match &**cl {
        Absyn::Class {
            info: SourceInfo {
                isReadOnly: __esc_readOnly,
                ..
            },
            ..
        } => {
            readOnly = (*__esc_readOnly).clone();
            readOnly.clone()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(readOnly)
}

pub(crate) fn renameComponent(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut oldName: metamodelica::Ref<Absyn::ComponentRef>,
    mut newName: metamodelica::Ref<Absyn::ComponentRef>,
    mut program: Absyn::Program,
) -> (Absyn::Program, metamodelica::Ref<Values::Value>) {
    let mut program: Absyn::Program = program;
    let mut result: metamodelica::Ref<Values::Value>;
    let mut comp_reps: InteractiveTypes::ComponentReplacementRules;
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    match '__try0: {
        if unwrap_break_err!(isClassReadOnly(&(unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false), '__try0))), '__try0)
        {
            result = ValuesMake::makeCodeTypeNameStr({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Error: class: "));
                __mm_s.push_str(
                    &*unwrap_break_err!(AbsynUtil::pathString(classPath.clone(), literal!("."), true, false), '__try0),
                );
                __mm_s.push_str(&*literal!(" is in a read only file!"));
                ArcStr::from(__mm_s)
            });
            return (program, result);
        }
        comp_reps = unwrap_break_err!(extractAllComponentreplacements(program.clone(), classPath.clone(), oldName.clone(), newName.clone()), '__try0);
        program = unwrap_break_err!(renameComponentFromComponentreplacements(program.clone(), &comp_reps), '__try0);
        paths = unwrap_break_err!(extractRenamedClassesAsStringList(&comp_reps), '__try0);
        result = ValuesMake::makeCodeTypeNameArray(paths.clone());
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    (program, result)
}

pub(crate) fn renameComponentOnlyInClass(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut oldName: metamodelica::Ref<Absyn::ComponentRef>,
    mut newName: metamodelica::Ref<Absyn::ComponentRef>,
    mut program: Absyn::Program,
) -> (Absyn::Program, metamodelica::Ref<Values::Value>) {
    let mut program: Absyn::Program = program;
    let mut result: metamodelica::Ref<Values::Value>;
    let mut cl: metamodelica::Ref<Absyn::Class>;
    let mut w: Absyn::Within;
    match '__try0: {
        if unwrap_break_err!(isClassReadOnly(&(unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false), '__try0))), '__try0)
        {
            result = ValuesMake::makeCodeTypeNameStr({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Error: class: "));
                __mm_s.push_str(
                    &*unwrap_break_err!(AbsynUtil::pathString(classPath.clone(), literal!("."), true, false), '__try0),
                );
                __mm_s.push_str(&*literal!(" is in a read only file!"));
                ArcStr::from(__mm_s)
            });
            return (program, result);
        }
        cl =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false), '__try0);
        cl = unwrap_break_err!(renameComponentInClass(cl.clone(), oldName.clone(), newName.clone()), '__try0);
        w = unwrap_break_err!(ProgramUtil::buildWithin(AbsynUtil::makeFullyQualified(classPath.clone())), '__try0);
        program = unwrap_break_err!(ProgramUtil::updateProgram(Absyn::Program { classes: list![cl.clone()], within_: w.clone() }, program.clone(), false, false), '__try0);
        result = ValuesMake::makeCodeTypeNameArray(list![classPath.clone()]);
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    (program, result)
}

fn extractRenamedClassesAsStringList(
    mut rules: &InteractiveTypes::ComponentReplacementRules,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut outPaths: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
    outPaths = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
        for mut rule in (rules.componentReplacementLst.clone()).into_iter().cloned() {
            let __x = rule.which1.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outPaths = List::uniqueOnTrue(&outPaths, &move |__a0: metamodelica::Ref<Absyn::Path>,
                                                    __a1: metamodelica::Ref<Absyn::Path>|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
    })?;
    Ok(outPaths)
}

fn renameComponentFromComponentreplacements(
    mut program: Absyn::Program,
    mut rules: &InteractiveTypes::ComponentReplacementRules,
) -> Result<Absyn::Program> {
    let mut program: Absyn::Program = program;
    for mut rule in &*rules.componentReplacementLst.clone() {
        (program, _, _) = AbsynUtil::traverseClasses(
            program,
            None,
            (std::sync::Arc::new(
                move |__a0: (
                    metamodelica::Ref<Absyn::Class>,
                    Option<metamodelica::Ref<Absyn::Path>>,
                    InteractiveTypes::ComponentReplacement,
                )|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(renameComponentVisitor(&__a0))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            (
                                metamodelica::Ref<Absyn::Class>,
                                Option<metamodelica::Ref<Absyn::Path>>,
                                InteractiveTypes::ComponentReplacement,
                            ),
                        ) -> Result<(
                            metamodelica::Ref<Absyn::Class>,
                            Option<metamodelica::Ref<Absyn::Path>>,
                            InteractiveTypes::ComponentReplacement,
                        )> + 'static,
                >),
            rule.clone(),
            true,
        )?;
    }
    Ok(program)
}

fn renameComponentVisitor(
    mut inTplAbsynClassAbsynPathOptionComponentReplacement: &(
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        InteractiveTypes::ComponentReplacement,
    ),
) -> (
    metamodelica::Ref<Absyn::Class>,
    Option<metamodelica::Ref<Absyn::Path>>,
    InteractiveTypes::ComponentReplacement,
) {
    let mut outTplAbsynClassAbsynPathOptionComponentReplacement: (
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        InteractiveTypes::ComponentReplacement,
    );
    outTplAbsynClassAbsynPathOptionComponentReplacement = 'mc: {
        let __mc_input = inTplAbsynClassAbsynPathOptionComponentReplacement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_ @ Deref @ Absyn::Class { name: id, .. }, Some(pa), InteractiveTypes::ComponentReplacement { which1: class_id, the2: old_comp, the3: new_comp }) => {
                    let mut path_1: metamodelica::Ref<Absyn::Path>;
                    let mut class_1: metamodelica::Ref<Absyn::Class>;
                    path_1 = AbsynUtil::joinPaths(pa.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() }))?;
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&class_id), &path_1)) else { return Err("pattern mismatch") };
                    class_1 = renameComponentInClass(class_.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok((class_1.clone(), Some(pa.clone()), InteractiveTypes::ComponentReplacement { which1: class_id.clone(), the2: old_comp.clone(), the3: new_comp.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_ @ Deref @ Absyn::Class { name: id, .. }, None, InteractiveTypes::ComponentReplacement { which1: class_id, the2: old_comp, the3: new_comp }) => {
                    let mut path_1: metamodelica::Ref<Absyn::Path>;
                    let mut class_1: metamodelica::Ref<Absyn::Class>;
                    path_1 = metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() });
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&class_id), &path_1)) else { return Err("pattern mismatch") };
                    class_1 = renameComponentInClass(class_.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok((class_1.clone(), None, InteractiveTypes::ComponentReplacement { which1: class_id.clone(), the2: old_comp.clone(), the3: new_comp.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_, opath, args) => {
                    Ok((class_.clone(), opath.clone(), args.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outTplAbsynClassAbsynPathOptionComponentReplacement
}

fn renameComponentInClass(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut oldName: metamodelica::Ref<Absyn::ComponentRef>,
    mut newName: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut body: metamodelica::Ref<Absyn::ClassDef>;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Absyn::Class { body: __esc_body @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
            body = (*__esc_body).clone();
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = renameComponentInParts(var_field!((*body).classParts, Absyn::ClassDef::PARTS), oldName, newName)?);
            assign_field!(cls.body = body.clone());
            ()
        },
        Deref @ Absyn::Class { body: __esc_body @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
            body = (*__esc_body).clone();
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = renameComponentInParts(var_field!((*body).parts, Absyn::ClassDef::CLASS_EXTENDS), oldName, newName)?);
            assign_field!(cls.body = body.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

fn renameComponentInParts(
    mut inAbsynClassPartLst1: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outAbsynClassPartLst: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outAbsynClassPartLst = 'mc: {
        let __mc_input = (&**inAbsynClassPartLst1, inComponentRef2, inComponentRef3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elements }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut elements_1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    res_1 = renameComponentInParts(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    elements_1 = renameComponentInElements(metamodelica::AsArg::as_arg(&elements), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: elements_1.clone() }), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elements }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut elements_1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    res_1 = renameComponentInParts(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    elements_1 = renameComponentInElements(metamodelica::AsArg::as_arg(&elements), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: elements_1.clone() }), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: equations }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut equations_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    res_1 = renameComponentInParts(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    equations_1 = renameComponentInEquationList(metamodelica::AsArg::as_arg(&equations), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::EQUATIONS { contents: equations_1.clone() }), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: equations }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut equations_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    res_1 = renameComponentInParts(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    equations_1 = renameComponentInEquationList(metamodelica::AsArg::as_arg(&equations), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::INITIALEQUATIONS { contents: equations_1.clone() }), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::ALGORITHMS { contents: algorithms }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut algorithms_1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
                    res_1 = renameComponentInParts(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    algorithms_1 = renameComponentInAlgorithms(metamodelica::AsArg::as_arg(&algorithms), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::ALGORITHMS { contents: algorithms_1.clone() }), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algorithms }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut algorithms_1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
                    res_1 = renameComponentInParts(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    algorithms_1 = renameComponentInAlgorithms(metamodelica::AsArg::as_arg(&algorithms), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::INITIALALGORITHMS { contents: algorithms_1.clone() }), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::EXTERNAL { externalDecl: external_decl, annotation_: ano }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut external_decl_1: metamodelica::Ref<Absyn::ExternalDecl>;
                    res_1 = renameComponentInParts(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    external_decl_1 = renameComponentInExternalDecl(external_decl.clone(), metamodelica::AsArg::as_arg(&old_comp), metamodelica::AsArg::as_arg(&new_comp));
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::EXTERNAL { externalDecl: external_decl_1.clone(), annotation_: ano.clone() }), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: a, tail: res }, old_comp, new_comp) => {
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    res_1 = renameComponentInParts(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons(a.clone(), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAbsynClassPartLst)
}

fn renameComponentInElements(
    mut inAbsynElementItemLst1: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    let mut outAbsynElementItemLst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    outAbsynElementItemLst = 'mc: {
        let __mc_input = (&**inAbsynElementItemLst1, inComponentRef2, inComponentRef3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix, redeclareKeywords: redeclare_, innerOuter: inner_outer, specification: elementspec, info, constrainClass } }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut elementspec_1: metamodelica::Ref<Absyn::ElementSpec>;
                    let mut element_1: metamodelica::Ref<Absyn::ElementItem>;
                    res_1 = renameComponentInElements(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    elementspec_1 = renameComponentInElementSpec(elementspec.clone(), old_comp.clone(), new_comp.clone());
                    element_1 = metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: metamodelica::Ref::new(Absyn::Element::ELEMENT { finalPrefix: finalPrefix.clone(), redeclareKeywords: redeclare_.clone(), innerOuter: inner_outer.clone(), specification: elementspec_1.clone(), info: info.clone(), constrainClass: constrainClass.clone() }) });
                    Ok(metamodelica::cons(element_1.clone(), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: element, tail: res }, old_comp, new_comp) => {
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut element_1: metamodelica::Ref<Absyn::ElementItem>;
                    res_1 = renameComponentInElements(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    element_1 = element.clone();
                    Ok(metamodelica::cons(element_1.clone(), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAbsynElementItemLst)
}

fn renameComponentInElementSpec(
    mut inElementSpec1: metamodelica::Ref<Absyn::ElementSpec>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> metamodelica::Ref<Absyn::ElementSpec> {
    let mut outElementSpec: metamodelica::Ref<Absyn::ElementSpec>;
    outElementSpec = 'mc: {
        let __mc_input = (&*inElementSpec1, inComponentRef2, inComponentRef3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ElementSpec::COMPONENTS { attributes: attr, typeSpec: path, components: comps }, old_comp, new_comp) => {
                    let mut comps_1: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
                    comps_1 = renameComponentInComponentitems(metamodelica::AsArg::as_arg(&comps), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::ElementSpec::COMPONENTS { attributes: attr.clone(), typeSpec: path.clone(), components: comps_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inElementSpec1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outElementSpec
}

fn renameComponentInComponentitems(
    mut inAbsynComponentItemLst1: &metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>> {
    let mut outAbsynComponentItemLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    outAbsynComponentItemLst = 'mc: {
        let __mc_input = (&**inAbsynComponentItemLst1, inComponentRef2, inComponentRef3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name, arrayDim, modification: r#mod }, condition: cond, comment }, tail: res }, old_comp, new_comp) => {
                    let mut old_comp_path: metamodelica::Ref<Absyn::Path>;
                    let mut new_comp_path: metamodelica::Ref<Absyn::Path>;
                    let mut old_comp_string: ArcStr;
                    let mut new_comp_string: ArcStr;
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
                    let mut comp_1: metamodelica::Ref<Absyn::ComponentItem>;
                    old_comp_path = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&old_comp))?;
                    old_comp_string = AbsynUtil::pathString(old_comp_path.clone(), literal!("."), true, false)?;
                    let true = (stringEq(&name, &old_comp_string)) else { return Err("pattern mismatch") };
                    new_comp_path = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&new_comp))?;
                    new_comp_string = AbsynUtil::pathString(new_comp_path.clone(), literal!("."), true, false)?;
                    res_1 = renameComponentInComponentitems(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    comp_1 = metamodelica::Ref::new(Absyn::ComponentItem { component: Absyn::Component { name: new_comp_string.clone(), arrayDim: arrayDim.clone(), modification: r#mod.clone() }, condition: cond.clone(), comment: comment.clone() });
                    Ok(metamodelica::cons(comp_1.clone(), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: comp @ Deref @ Absyn::ComponentItem { component: Absyn::Component { .. }, .. }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
                    res_1 = renameComponentInComponentitems(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons(comp.clone(), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("-Interactive.renameComponentInComponentitems failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAbsynComponentItemLst)
}

fn renameComponentInEquationList(
    mut inAbsynEquationItemLst1: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>> {
    let mut outAbsynEquationItemLst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    outAbsynEquationItemLst = 'mc: {
        let __mc_input = (&**inAbsynEquationItemLst1, inComponentRef2, inComponentRef3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_, comment: cmt, info }, tail: res }, old_comp, new_comp) => {
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    let mut equation_1: metamodelica::Ref<Absyn::Equation>;
                    res_1 = renameComponentInEquationList(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    equation_1 = renameComponentInEquation(metamodelica::AsArg::as_arg(&equation_), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::EquationItem::EQUATIONITEM { equation_: equation_1.clone(), comment: cmt.clone(), info: info.clone() }), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: equation_item, tail: res }, old_comp, new_comp) => {
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    res_1 = renameComponentInEquationList(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons(equation_item.clone(), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAbsynEquationItemLst)
}

fn renameComponentInExpEquationitemList(
    mut inTplAbsynExpAbsynEquationItemLstLst1: &metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    )>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    )>,
> {
    let mut outTplAbsynExpAbsynEquationItemLstLst: metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    )>;
    outTplAbsynExpAbsynEquationItemLstLst = 'mc: {
        let __mc_input = (
            &**inTplAbsynExpAbsynEquationItemLstLst1,
            inComponentRef2,
            inComponentRef3,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (exp1, eqn_item), tail: res }, old_comp, new_comp) => {
                    let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut eqn_item_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    let mut res_1: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>)>;
                    exp1_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp1), old_comp.clone(), new_comp.clone())?;
                    eqn_item_1 = renameComponentInEquationList(metamodelica::AsArg::as_arg(&eqn_item), old_comp.clone(), new_comp.clone())?;
                    res_1 = renameComponentInExpEquationitemList(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons((exp1_1.clone(), eqn_item_1.clone()), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("-rename_component_in_exp_equationitem_list failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTplAbsynExpAbsynEquationItemLstLst)
}

fn renameComponentInEquation(
    mut inEquation1: &metamodelica::Ref<Absyn::Equation>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::Equation>> {
    let mut outEquation: metamodelica::Ref<Absyn::Equation>;
    outEquation = 'mc: {
        let __mc_input = (&**inEquation1, inComponentRef2, inComponentRef3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Equation::EQ_IF { ifExp: exp, equationTrueItems: true_items, elseIfBranches: exp_elseifs, equationElseItems: elses }, old_comp, new_comp) => {
                    let mut exp_1: metamodelica::Ref<Absyn::Exp>;
                    let mut true_items_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    let mut elses_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    let mut exp_elseifs_1: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>)>;
                    exp_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp), old_comp.clone(), new_comp.clone())?;
                    true_items_1 = renameComponentInEquationList(metamodelica::AsArg::as_arg(&true_items), old_comp.clone(), new_comp.clone())?;
                    exp_elseifs_1 = renameComponentInExpEquationitemList(metamodelica::AsArg::as_arg(&exp_elseifs), old_comp.clone(), new_comp.clone())?;
                    elses_1 = renameComponentInEquationList(metamodelica::AsArg::as_arg(&elses), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Equation::EQ_IF { ifExp: exp_1.clone(), equationTrueItems: true_items_1.clone(), elseIfBranches: exp_elseifs_1.clone(), equationElseItems: elses_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Equation::EQ_EQUALS { leftSide: exp1, rightSide: exp2 }, old_comp, new_comp) => {
                    let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp2_1: metamodelica::Ref<Absyn::Exp>;
                    exp1_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp1), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp2), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Equation::EQ_EQUALS { leftSide: exp1_1.clone(), rightSide: exp2_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Equation::EQ_PDE { leftSide: exp1, rightSide: exp2, domain: cref1 }, old_comp, new_comp) => {
                    let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp2_1: metamodelica::Ref<Absyn::Exp>;
                    let mut cref1_1: metamodelica::Ref<Absyn::ComponentRef>;
                    exp1_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp1), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp2), old_comp.clone(), new_comp.clone())?;
                    cref1_1 = replaceStartInComponentRef(metamodelica::AsArg::as_arg(&cref1), metamodelica::AsArg::as_arg(&old_comp), new_comp.clone());
                    Ok(metamodelica::Ref::new(Absyn::Equation::EQ_PDE { leftSide: exp1_1.clone(), rightSide: exp2_1.clone(), domain: cref1_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Equation::EQ_CONNECT { connector1: cref1, connector2: cref2 }, old_comp, new_comp) => {
                    let mut cref1_1: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut cref2_1: metamodelica::Ref<Absyn::ComponentRef>;
                    cref1_1 = replaceStartInComponentRef(metamodelica::AsArg::as_arg(&cref1), metamodelica::AsArg::as_arg(&old_comp), new_comp.clone());
                    cref2_1 = replaceStartInComponentRef(metamodelica::AsArg::as_arg(&cref2), metamodelica::AsArg::as_arg(&old_comp), new_comp.clone());
                    Ok(metamodelica::Ref::new(Absyn::Equation::EQ_CONNECT { connector1: cref1_1.clone(), connector2: cref2_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Equation::EQ_FOR { iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ForIterator { name: ident, guardExp: None, range: Some(exp) }, tail: Deref @ metamodelica::ListNode::Nil }, forEquations: equations }, old_comp, new_comp) => {
                    let mut exp_1: metamodelica::Ref<Absyn::Exp>;
                    let mut equations_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    exp_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp), old_comp.clone(), new_comp.clone())?;
                    equations_1 = renameComponentInEquationList(metamodelica::AsArg::as_arg(&equations), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Equation::EQ_FOR { iterators: list![metamodelica::Ref::new(Absyn::ForIterator { name: ident.clone(), guardExp: None, range: Some(exp_1.clone()) })], forEquations: equations_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Equation::EQ_WHEN_E { whenExp: exp, whenEquations: equations, elseWhenEquations: exp_equations }, old_comp, new_comp) => {
                    let mut exp_1: metamodelica::Ref<Absyn::Exp>;
                    let mut equations_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    let mut exp_equations_1: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>)>;
                    exp_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp), old_comp.clone(), new_comp.clone())?;
                    equations_1 = renameComponentInEquationList(metamodelica::AsArg::as_arg(&equations), old_comp.clone(), new_comp.clone())?;
                    exp_equations_1 = renameComponentInExpEquationitemList(metamodelica::AsArg::as_arg(&exp_equations), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Equation::EQ_WHEN_E { whenExp: exp_1.clone(), whenEquations: equations_1.clone(), elseWhenEquations: exp_equations_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Equation::EQ_NORETCALL { functionName: cref, functionArgs: function_args }, _, _) => {
                    metamodelica::print(literal!("-rename_component_in_equation EQ_NORETCALL not implemented yet\n"));
                    Ok(metamodelica::Ref::new(Absyn::Equation::EQ_NORETCALL { functionName: cref.clone(), functionArgs: function_args.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("-rename_component_in_equation failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outEquation)
}

fn renameComponentInExpList(
    mut inAbsynExpLst1: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Exp>>> {
    let mut outAbsynExpLst: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    outAbsynExpLst = 'mc: {
        let __mc_input = (&**inAbsynExpLst1, inComponentRef2, inComponentRef3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: exp, tail: res }, old_comp, new_comp) => {
                    let mut exp_1: metamodelica::Ref<Absyn::Exp>;
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    exp_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp), old_comp.clone(), new_comp.clone())?;
                    res_1 = renameComponentInExpList(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons(exp_1.clone(), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("-rename_component_in_exp_list failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAbsynExpLst)
}

fn renameComponentInExpListList(
    mut inAbsynExpLstLst1: &metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>> {
    let mut outAbsynExpLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>;
    outAbsynExpLstLst = 'mc: {
        let __mc_input = (&**inAbsynExpLstLst1, inComponentRef2, inComponentRef3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: exp, tail: res }, old_comp, new_comp) => {
                    let mut exp_1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut res_1: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>;
                    exp_1 = renameComponentInExpList(metamodelica::AsArg::as_arg(&exp), old_comp.clone(), new_comp.clone())?;
                    res_1 = renameComponentInExpListList(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons(exp_1.clone(), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("-rename_component_in_exp_list_list failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAbsynExpLstLst)
}

fn renameComponentInExpTupleList(
    mut inTplAbsynExpAbsynExpLst1: &metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>> {
    let mut outTplAbsynExpAbsynExpLst: metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::Ref<Absyn::Exp>,
    )>;
    outTplAbsynExpAbsynExpLst = 'mc: {
        let __mc_input = (&**inTplAbsynExpAbsynExpLst1, inComponentRef2, inComponentRef3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (exp1, exp2), tail: res }, old_comp, new_comp) => {
                    let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp2_1: metamodelica::Ref<Absyn::Exp>;
                    let mut res_1: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>;
                    exp1_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp1), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp2), old_comp.clone(), new_comp.clone())?;
                    res_1 = renameComponentInExpTupleList(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons((exp1_1.clone(), exp2_1.clone()), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("-rename_component_in_exp_tuple_list failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTplAbsynExpAbsynExpLst)
}

fn renameComponentInElementArgList(
    mut inAbsynElementArgLst1: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut outAbsynElementArgLst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    outAbsynElementArgLst = 'mc: {
        let __mc_input = (&**inAbsynElementArgLst1, inComponentRef2, inComponentRef3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: element_arg, tail: res }, old_comp, new_comp) => {
                    let mut element_arg_1: metamodelica::Ref<Absyn::ElementArg>;
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    element_arg_1 = renameComponentInElementArg(metamodelica::AsArg::as_arg(&element_arg), old_comp.clone(), new_comp.clone())?;
                    res_1 = renameComponentInElementArgList(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons(element_arg_1.clone(), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("-rename_component_in_element_arg_list failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAbsynElementArgLst)
}

fn renameComponentInElementArg(
    mut inElementArg1: &metamodelica::Ref<Absyn::ElementArg>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut outElementArg: metamodelica::Ref<Absyn::ElementArg>;
    outElementArg = (::match_deref::match_deref! { match inElementArg1 {
        Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: b, eachPrefix: each_, path: p, modification: Some(Deref @ Absyn::Modification { elementArgLst: element_args, eqMod: Deref @ Absyn::EqMod::EQMOD { exp, info } }), comment: r#str, info: mod_info } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut p_1: metamodelica::Ref<Absyn::Path>;
            let mut exp_1: metamodelica::Ref<Absyn::Exp>;
            let mut element_args_1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
            p_1 = AbsynUtil::crefToPath(&(replaceStartInComponentRef(&(AbsynUtil::pathToCref(p)), &old_comp, new_comp.clone())))?;
            exp_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp), old_comp.clone(), new_comp.clone())?;
            element_args_1 = renameComponentInElementArgList(metamodelica::AsArg::as_arg(&element_args), old_comp, new_comp)?;
            metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: b.clone(), eachPrefix: each_.clone(), path: p_1, modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: element_args_1, eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: exp_1, info: info.clone() }) })), comment: r#str.clone(), info: mod_info.clone() })
        },
        Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: b, eachPrefix: each_, path: p, modification: Some(Deref @ Absyn::Modification { elementArgLst: element_args, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } }), comment: r#str, info: mod_info } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut p_1: metamodelica::Ref<Absyn::Path>;
            let mut element_args_1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
            p_1 = AbsynUtil::crefToPath(&(replaceStartInComponentRef(&(AbsynUtil::pathToCref(p)), &old_comp, new_comp.clone())))?;
            element_args_1 = renameComponentInElementArgList(metamodelica::AsArg::as_arg(&element_args), old_comp, new_comp)?;
            metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: b.clone(), eachPrefix: each_.clone(), path: p_1, modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: element_args_1, eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), comment: r#str.clone(), info: mod_info.clone() })
        },
        Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: b, eachPrefix: each_, path: p, modification: None, comment: r#str, info: mod_info } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut p_1: metamodelica::Ref<Absyn::Path>;
            p_1 = AbsynUtil::crefToPath(&(replaceStartInComponentRef(&(AbsynUtil::pathToCref(p)), &old_comp, new_comp)))?;
            metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: b.clone(), eachPrefix: each_.clone(), path: p_1, modification: None, comment: r#str.clone(), info: mod_info.clone() })
        },
        Deref @ Absyn::ElementArg::REDECLARATION { finalPrefix: b, redeclareKeywords: redecl, eachPrefix: each_, elementSpec: element_spec, constrainClass: Some(Deref @ Absyn::ConstrainClass { elementSpec: element_spec2, comment: c }), info } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut element_spec_1: metamodelica::Ref<Absyn::ElementSpec>;
            let mut element_spec2_1: metamodelica::Ref<Absyn::ElementSpec>;
            element_spec_1 = renameComponentInElementSpec(element_spec.clone(), old_comp.clone(), new_comp.clone());
            element_spec2_1 = renameComponentInElementSpec(element_spec2.clone(), old_comp, new_comp);
            metamodelica::Ref::new(Absyn::ElementArg::REDECLARATION { finalPrefix: b.clone(), redeclareKeywords: redecl.clone(), eachPrefix: each_.clone(), elementSpec: element_spec_1, constrainClass: Some(metamodelica::Ref::new(Absyn::ConstrainClass { elementSpec: element_spec2_1, comment: c.clone() })), info: info.clone() })
        },
        Deref @ Absyn::ElementArg::REDECLARATION { finalPrefix: b, redeclareKeywords: redecl, eachPrefix: each_, elementSpec: element_spec, constrainClass: None, info } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut element_spec_1: metamodelica::Ref<Absyn::ElementSpec>;
            element_spec_1 = renameComponentInElementSpec(element_spec.clone(), old_comp, new_comp);
            metamodelica::Ref::new(Absyn::ElementArg::REDECLARATION { finalPrefix: b.clone(), redeclareKeywords: redecl.clone(), eachPrefix: each_.clone(), elementSpec: element_spec_1, constrainClass: None, info: info.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outElementArg)
}

fn renameComponentInCode(
    mut inCode1: &metamodelica::Ref<Absyn::CodeNode>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::CodeNode>> {
    let mut outCode: metamodelica::Ref<Absyn::CodeNode>;
    outCode = (::match_deref::match_deref! { match inCode1 {
        Deref @ Absyn::CodeNode::C_TYPENAME { path } => {
            metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: path.clone() })
        },
        Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut cr_1: metamodelica::Ref<Absyn::ComponentRef>;
            cr_1 = replaceStartInComponentRef(cr, &old_comp, new_comp);
            metamodelica::Ref::new(Absyn::CodeNode::C_VARIABLENAME { componentRef: cr_1 })
        },
        Deref @ Absyn::CodeNode::C_EQUATIONSECTION { boolean: b, equationItemLst: eqn_items } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut eqn_items_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            eqn_items_1 = renameComponentInEquationList(eqn_items, old_comp, new_comp)?;
            metamodelica::Ref::new(Absyn::CodeNode::C_EQUATIONSECTION { boolean: b.clone(), equationItemLst: eqn_items_1 })
        },
        Deref @ Absyn::CodeNode::C_ALGORITHMSECTION { boolean: b, algorithmItemLst: algs } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut algs_1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            algs_1 = renameComponentInAlgorithms(algs, old_comp, new_comp)?;
            metamodelica::Ref::new(Absyn::CodeNode::C_ALGORITHMSECTION { boolean: b.clone(), algorithmItemLst: algs_1 })
        },
        Deref @ Absyn::CodeNode::C_ELEMENT { element: Deref @ Absyn::Element::ELEMENT { finalPrefix, redeclareKeywords: redeclare_, innerOuter: inner_outer, specification: elementspec, info, constrainClass } } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut elementspec_1: metamodelica::Ref<Absyn::ElementSpec>;
            elementspec_1 = renameComponentInElementSpec(elementspec.clone(), old_comp, new_comp);
            metamodelica::Ref::new(Absyn::CodeNode::C_ELEMENT { element: metamodelica::Ref::new(Absyn::Element::ELEMENT { finalPrefix: finalPrefix.clone(), redeclareKeywords: redeclare_.clone(), innerOuter: inner_outer.clone(), specification: elementspec_1, info: info.clone(), constrainClass: constrainClass.clone() }) })
        },
        Deref @ Absyn::CodeNode::C_EXPRESSION { exp } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut exp_1: metamodelica::Ref<Absyn::Exp>;
            exp_1 = renameComponentInExp(exp, old_comp, new_comp)?;
            metamodelica::Ref::new(Absyn::CodeNode::C_EXPRESSION { exp: exp_1 })
        },
        Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { elementArgLst: element_args, eqMod: Deref @ Absyn::EqMod::EQMOD { exp, info } } } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut exp_1: metamodelica::Ref<Absyn::Exp>;
            let mut element_args_1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
            exp_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp), old_comp.clone(), new_comp.clone())?;
            element_args_1 = renameComponentInElementArgList(metamodelica::AsArg::as_arg(&element_args), old_comp, new_comp)?;
            metamodelica::Ref::new(Absyn::CodeNode::C_MODIFICATION { modification: metamodelica::Ref::new(Absyn::Modification { elementArgLst: element_args_1, eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: exp_1, info: info.clone() }) }) })
        },
        Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { elementArgLst: element_args, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } } } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut element_args_1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
            element_args_1 = renameComponentInElementArgList(metamodelica::AsArg::as_arg(&element_args), old_comp, new_comp)?;
            metamodelica::Ref::new(Absyn::CodeNode::C_MODIFICATION { modification: metamodelica::Ref::new(Absyn::Modification { elementArgLst: element_args_1, eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() }) })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outCode)
}

fn renameComponentInExp(
    mut inExp1: &metamodelica::Ref<Absyn::Exp>,
    mut oldPrefix: metamodelica::Ref<Absyn::ComponentRef>,
    mut newPrefix: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    outExp = 'mc: {
        let __mc_input = (&**inExp1, oldPrefix, newPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::INTEGER { .. }, _, _) => {
                    Ok(inExp1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::REAL { .. }, _, _) => {
                    Ok(inExp1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: cr }, old_comp, new_comp) => {
                    let mut cr_1: metamodelica::Ref<Absyn::ComponentRef>;
                    cr_1 = replaceStartInComponentRef(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&old_comp), new_comp.clone());
                    Ok(metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::STRING { .. }, _, _) => {
                    Ok(inExp1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::BOOL { .. }, _, _) => {
                    Ok(inExp1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::BINARY { exp1, op, exp2 }, old_comp, new_comp) => {
                    let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp2_1: metamodelica::Ref<Absyn::Exp>;
                    exp1_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp1), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp2), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: exp1_1.clone(), op: op.clone(), exp2: exp2_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::UNARY { op, exp }, old_comp, new_comp) => {
                    let mut exp = (*exp).clone();
                    exp = renameComponentInExp(metamodelica::AsArg::as_arg(&exp), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::UNARY { op: op.clone(), exp: exp.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::LBINARY { exp1, op, exp2 }, old_comp, new_comp) => {
                    let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp2_1: metamodelica::Ref<Absyn::Exp>;
                    exp1_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp1), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp2), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::LBINARY { exp1: exp1_1.clone(), op: op.clone(), exp2: exp2_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::LUNARY { op, exp }, old_comp, new_comp) => {
                    let mut exp = (*exp).clone();
                    exp = renameComponentInExp(metamodelica::AsArg::as_arg(&exp), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::LUNARY { op: op.clone(), exp: exp.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::RELATION { exp1, op, exp2 }, old_comp, new_comp) => {
                    let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp2_1: metamodelica::Ref<Absyn::Exp>;
                    exp1_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp1), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp2), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::RELATION { exp1: exp1_1.clone(), op: op.clone(), exp2: exp2_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::IFEXP { ifExp: exp1, trueBranch: exp2, elseBranch: exp3, elseIfBranch: exp_tuple_list }, old_comp, new_comp) => {
                    let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp2_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp3_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp_tuple_list_1: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>;
                    exp1_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp1), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp2), old_comp.clone(), new_comp.clone())?;
                    exp3_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp3), old_comp.clone(), new_comp.clone())?;
                    exp_tuple_list_1 = renameComponentInExpTupleList(metamodelica::AsArg::as_arg(&exp_tuple_list), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::IFEXP { ifExp: exp1_1.clone(), trueBranch: exp2_1.clone(), elseBranch: exp3_1.clone(), elseIfBranch: exp_tuple_list_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CALL { function_: cref, functionArgs: func_args, .. }, old_comp, new_comp) => {
                    let mut cref = (*cref).clone();
                    let mut func_args = (*func_args).clone();
                    cref = replaceStartInComponentRef(metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&old_comp), new_comp.clone());
                    func_args = renameComponentInFunctionArgs(metamodelica::AsArg::as_arg(&func_args), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::CALL { function_: cref.clone(), functionArgs: func_args.clone(), typeVars: var_field!((**inExp1).typeVars, Absyn::Exp::CALL).clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::ARRAY { arrayExp: exp_list }, old_comp, new_comp) => {
                    let mut exp_list_1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    exp_list_1 = renameComponentInExpList(metamodelica::AsArg::as_arg(&exp_list), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: exp_list_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::MATRIX { matrix: exp_list_list }, old_comp, new_comp) => {
                    let mut exp_list_list_1: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>;
                    exp_list_list_1 = renameComponentInExpListList(metamodelica::AsArg::as_arg(&exp_list_list), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::MATRIX { matrix: exp_list_list_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::RANGE { start: exp1, step: Some(exp2), stop: exp3 }, old_comp, new_comp) => {
                    let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp2_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp3_1: metamodelica::Ref<Absyn::Exp>;
                    exp1_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp1), old_comp.clone(), new_comp.clone())?;
                    exp2_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp2), old_comp.clone(), new_comp.clone())?;
                    exp3_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp3), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::RANGE { start: exp1_1.clone(), step: Some(exp2_1.clone()), stop: exp3_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::RANGE { start: exp1, step: None, stop: exp3 }, old_comp, new_comp) => {
                    let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp3_1: metamodelica::Ref<Absyn::Exp>;
                    exp1_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp1), old_comp.clone(), new_comp.clone())?;
                    exp3_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp3), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::RANGE { start: exp1_1.clone(), step: None, stop: exp3_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::TUPLE { expressions: exp_list }, old_comp, new_comp) => {
                    let mut exp_list_1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    exp_list_1 = renameComponentInExpList(metamodelica::AsArg::as_arg(&exp_list), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::TUPLE { expressions: exp_list_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::END { .. }, _, _) => {
                    Ok(openmodelica_ast::Absyn::Exp::interned_END())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CODE { code }, old_comp, new_comp) => {
                    let mut code_1: metamodelica::Ref<Absyn::CodeNode>;
                    code_1 = renameComponentInCode(metamodelica::AsArg::as_arg(&code), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::CODE { code: code_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("-rename_component_in_exp failed\n"));
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

fn renameComponentInAlgorithms(
    mut inAbsynAlgorithmItemLst1: &metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>> {
    let mut outAbsynAlgorithmItemLst: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
    outAbsynAlgorithmItemLst = (::match_deref::match_deref! { match inAbsynAlgorithmItemLst1 {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: algorithm_, tail: res } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            let mut algorithm_1: metamodelica::Ref<Absyn::AlgorithmItem>;
            res_1 = renameComponentInAlgorithms(res, old_comp, new_comp)?;
            algorithm_1 = algorithm_.clone();
            metamodelica::cons(algorithm_1, res_1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAbsynAlgorithmItemLst)
}

fn renameComponentInAlgorithm(
    mut inAlgorithm1: &metamodelica::Ref<Absyn::Algorithm>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::Algorithm>> {
    let mut outAlgorithm: metamodelica::Ref<Absyn::Algorithm>;
    outAlgorithm = (::match_deref::match_deref! { match inAlgorithm1 {
        Deref @ Absyn::Algorithm::ALG_ASSIGN { assignComponent: Deref @ Absyn::Exp::CREF { componentRef: cr }, value: exp } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut cr_1: metamodelica::Ref<Absyn::ComponentRef>;
            let mut exp_1: metamodelica::Ref<Absyn::Exp>;
            cr_1 = replaceStartInComponentRef(metamodelica::AsArg::as_arg(&cr), &old_comp, new_comp.clone());
            exp_1 = renameComponentInExp(exp, old_comp, new_comp)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_ASSIGN { assignComponent: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr_1 }), value: exp_1 })
        },
        Deref @ Absyn::Algorithm::ALG_ASSIGN { assignComponent: exp1 @ Deref @ Absyn::Exp::TUPLE { expressions: _ }, value: exp2 } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
            let mut exp2_1: metamodelica::Ref<Absyn::Exp>;
            exp1_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp1), old_comp.clone(), new_comp.clone())?;
            exp2_1 = renameComponentInExp(exp2, old_comp, new_comp)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_ASSIGN { assignComponent: exp1_1, value: exp2_1 })
        },
        Deref @ Absyn::Algorithm::ALG_IF { ifExp: exp, trueBranch: algs1, elseIfAlgorithmBranch: exp_algs_list, elseBranch: algs2 } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut exp_1: metamodelica::Ref<Absyn::Exp>;
            let mut algs1_1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            let mut algs2_1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            let mut exp_algs_list_1: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>)>;
            exp_1 = renameComponentInExp(exp, old_comp.clone(), new_comp.clone())?;
            algs1_1 = renameComponentInAlgorithms(algs1, old_comp.clone(), new_comp.clone())?;
            exp_algs_list_1 = renameComponentInExpAlgoritmsList(exp_algs_list, old_comp.clone(), new_comp.clone())?;
            algs2_1 = renameComponentInAlgorithms(algs2, old_comp, new_comp)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_IF { ifExp: exp_1, trueBranch: algs1_1, elseIfAlgorithmBranch: exp_algs_list_1, elseBranch: algs2_1 })
        },
        Deref @ Absyn::Algorithm::ALG_FOR { iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ForIterator { name: id, guardExp: None, range: Some(exp) }, tail: Deref @ metamodelica::ListNode::Nil }, forBody: algs } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut exp_1: metamodelica::Ref<Absyn::Exp>;
            let mut algs_1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            exp_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp), old_comp.clone(), new_comp.clone())?;
            algs_1 = renameComponentInAlgorithms(algs, old_comp, new_comp)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_FOR { iterators: list![metamodelica::Ref::new(Absyn::ForIterator { name: id.clone(), guardExp: None, range: Some(exp_1) })], forBody: algs_1 })
        },
        Deref @ Absyn::Algorithm::ALG_WHILE { boolExpr: exp, whileBody: algs } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut exp_1: metamodelica::Ref<Absyn::Exp>;
            let mut algs_1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            exp_1 = renameComponentInExp(exp, old_comp.clone(), new_comp.clone())?;
            algs_1 = renameComponentInAlgorithms(algs, old_comp, new_comp)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_WHILE { boolExpr: exp_1, whileBody: algs_1 })
        },
        Deref @ Absyn::Algorithm::ALG_WHEN_A { boolExpr: exp, whenBody: algs, elseWhenAlgorithmBranch: exp_algs_list } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut exp_1: metamodelica::Ref<Absyn::Exp>;
            let mut algs_1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            let mut exp_algs_list_1: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>)>;
            exp_1 = renameComponentInExp(exp, old_comp.clone(), new_comp.clone())?;
            algs_1 = renameComponentInAlgorithms(algs, old_comp.clone(), new_comp.clone())?;
            exp_algs_list_1 = renameComponentInExpAlgoritmsList(exp_algs_list, old_comp, new_comp)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_WHEN_A { boolExpr: exp_1, whenBody: algs_1, elseWhenAlgorithmBranch: exp_algs_list_1 })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: cr, functionArgs: func_args } => {
            let mut old_comp = inComponentRef2;
            let mut new_comp = inComponentRef3;
            let mut cr_1: metamodelica::Ref<Absyn::ComponentRef>;
            let mut func_args_1: metamodelica::Ref<Absyn::FunctionArgs>;
            cr_1 = replaceStartInComponentRef(cr, &old_comp, new_comp.clone());
            func_args_1 = renameComponentInFunctionArgs(func_args, old_comp, new_comp)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: cr_1, functionArgs: func_args_1 })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outAlgorithm)
}

fn renameComponentInExpAlgoritmsList(
    mut inTplAbsynExpAbsynAlgorithmItemLstLst1: &metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    )>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    )>,
> {
    let mut outTplAbsynExpAbsynAlgorithmItemLstLst: metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    )>;
    outTplAbsynExpAbsynAlgorithmItemLstLst = 'mc: {
        let __mc_input = (
            &**inTplAbsynExpAbsynAlgorithmItemLstLst1,
            inComponentRef2,
            inComponentRef3,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (exp, algs), tail: res }, old_comp, new_comp) => {
                    let mut exp_1: metamodelica::Ref<Absyn::Exp>;
                    let mut algs_1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
                    let mut res_1: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>)>;
                    exp_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp), old_comp.clone(), new_comp.clone())?;
                    algs_1 = renameComponentInAlgorithms(metamodelica::AsArg::as_arg(&algs), old_comp.clone(), new_comp.clone())?;
                    res_1 = renameComponentInExpAlgoritmsList(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons((exp_1.clone(), algs_1.clone()), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("-rename_component_in_exp_algoritms_list failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTplAbsynExpAbsynAlgorithmItemLstLst)
}

fn renameComponentInFunctionArgs(
    mut inFunctionArgs1: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::FunctionArgs>> {
    let mut outFunctionArgs: metamodelica::Ref<Absyn::FunctionArgs>;
    outFunctionArgs = 'mc: {
        let __mc_input = (&**inFunctionArgs1, inComponentRef2, inComponentRef3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: exps, argNames: namedArg }, old_comp, new_comp) => {
                    let mut exps_1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut namedArg_1: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    exps_1 = renameComponentInExpList(metamodelica::AsArg::as_arg(&exps), old_comp.clone(), new_comp.clone())?;
                    namedArg_1 = renameComponentInNamedArgs(metamodelica::AsArg::as_arg(&namedArg), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: exps_1.clone(), argNames: namedArg_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { exp, iterType, iterators }, old_comp, new_comp) => {
                    let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut iteratorsRenamed: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>;
                    exp1_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp), old_comp.clone(), new_comp.clone())?;
                    iteratorsRenamed = renameComponentInIterators(iterators.clone(), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::FunctionArgs::FOR_ITER_FARG { exp: exp1_1.clone(), iterType: iterType.clone(), iterators: iteratorsRenamed.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("-rename_component_in_function_args failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outFunctionArgs)
}

fn renameComponentInIterators(
    mut iterators: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>,
    mut oldComp: metamodelica::Ref<Absyn::ComponentRef>,
    mut newComp: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>> {
    let mut iteratorsRenamed: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>;
    iteratorsRenamed = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>> = metamodelica::nil();
        for mut it in (iterators).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(it.clone()) {
                Deref @ Absyn::ForIterator { name: i, guardExp: None, range: Some(exp) } => {
                    let mut exp = (*exp).clone();
                    exp = renameComponentInExp(metamodelica::AsArg::as_arg(&exp), oldComp.clone(), newComp.clone())?;
                    metamodelica::Ref::new(Absyn::ForIterator { name: i.clone(), guardExp: None, range: Some(exp.clone()) })
                },
                Deref @ Absyn::ForIterator { name: i, guardExp: None, range: None } => {
                    metamodelica::Ref::new(Absyn::ForIterator { name: i.clone(), guardExp: None, range: None })
                },
                _ => return Err("match: no arm matched"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(iteratorsRenamed)
}

fn renameComponentInNamedArgs(
    mut inAbsynNamedArgLst1: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>> {
    let mut outAbsynNamedArgLst: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
    outAbsynNamedArgLst = 'mc: {
        let __mc_input = (&**inAbsynNamedArgLst1, inComponentRef2, inComponentRef3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: id, argValue: exp }, tail: res }, old_comp, new_comp) => {
                    let mut exp_1: metamodelica::Ref<Absyn::Exp>;
                    let mut res_1: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    exp_1 = renameComponentInExp(metamodelica::AsArg::as_arg(&exp), old_comp.clone(), new_comp.clone())?;
                    res_1 = renameComponentInNamedArgs(metamodelica::AsArg::as_arg(&res), old_comp.clone(), new_comp.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: id.clone(), argValue: exp_1.clone() }), res_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("-rename_component_in_namedArgs failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAbsynNamedArgLst)
}

fn renameComponentInExternalDecl(
    mut external_: metamodelica::Ref<Absyn::ExternalDecl>,
    mut old_comp: &metamodelica::Ref<Absyn::ComponentRef>,
    mut new_comp: &metamodelica::Ref<Absyn::ComponentRef>,
) -> metamodelica::Ref<Absyn::ExternalDecl> {
    let mut external_1: metamodelica::Ref<Absyn::ExternalDecl>;
    metamodelica::print(literal!("-rename_component_in_external_decl not implemented yet\n"));
    external_1 = external_;
    external_1
}

fn replaceStartInComponentRef(
    mut cr1: &metamodelica::Ref<Absyn::ComponentRef>,
    mut cr2: &metamodelica::Ref<Absyn::ComponentRef>,
    mut cr3: metamodelica::Ref<Absyn::ComponentRef>,
) -> metamodelica::Ref<Absyn::ComponentRef> {
    let mut res: metamodelica::Ref<Absyn::ComponentRef>;
    res = replaceStartInComponentRef2(cr1, cr2, cr3);
    res
}

fn replaceStartInComponentRef2(
    mut inComponentRef1: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef2: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> metamodelica::Ref<Absyn::ComponentRef> {
    let mut outComponentRef: metamodelica::Ref<Absyn::ComponentRef>;
    outComponentRef = 'mc: {
        let __mc_input = (&**inComponentRef1, &**inComponentRef2, inComponentRef3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, .. }, Deref @ Absyn::ComponentRef::CREF_IDENT { name: id2, .. }, res @ Deref @ Absyn::ComponentRef::CREF_IDENT { .. }) => {
                    let true = (stringEq(&id, &id2)) else { return Err("pattern mismatch") };
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: id, subscripts: a, componentRef: cr1 }, Deref @ Absyn::ComponentRef::CREF_IDENT { name: id2, .. }, Deref @ Absyn::ComponentRef::CREF_IDENT { name: id3, .. }) => {
                    let true = (stringEq(&id, &id2)) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: id3.clone(), subscripts: a.clone(), componentRef: cr1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: id, subscripts: a, componentRef: cr1 }, Deref @ Absyn::ComponentRef::CREF_QUAL { name: id2, componentRef: cr2, .. }, Deref @ Absyn::ComponentRef::CREF_QUAL { name: id3, componentRef: cr3, .. }) => {
                    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
                    let true = (stringEq(&id, &id2)) else { return Err("pattern mismatch") };
                    cr = replaceStartInComponentRef2(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2), cr3.clone());
                    Ok(metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: id3.clone(), subscripts: a.clone(), componentRef: cr.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inComponentRef1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outComponentRef
}

fn getComponentreplacementsrules(
    mut inComponents: InteractiveTypes::Components,
    mut inComponentReplacementRules: InteractiveTypes::ComponentReplacementRules,
    mut inInteger: i32,
) -> Result<InteractiveTypes::ComponentReplacementRules> {
    let mut outComponentReplacementRules: InteractiveTypes::ComponentReplacementRules;
    outComponentReplacementRules = 'mc: {
        let __mc_input = (inComponents, inComponentReplacementRules, inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            let (_, mut comp_reps, mut old_len) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut len: i32;
            len = lengthComponentReplacementRules(&(comp_reps.clone()));
            let true = (len == old_len.clone()) else {
                return Err("pattern mismatch");
            };
            Ok(comp_reps.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut comps, mut comp_reps, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut old_len: i32;
            let mut comp_reps_1: InteractiveTypes::ComponentReplacementRules;
            let mut comp_reps_2: InteractiveTypes::ComponentReplacementRules;
            let mut comp_reps_res: InteractiveTypes::ComponentReplacementRules;
            old_len = lengthComponentReplacementRules(&(comp_reps.clone()));
            comp_reps_1 = getNewComponentreplacementsrulesForEachRule(comps.clone(), comp_reps.clone())?;
            comp_reps_2 = joinComponentReplacementRules(&comp_reps_1, &(comp_reps.clone()))?;
            comp_reps_res = getComponentreplacementsrules(comps.clone(), comp_reps_2.clone(), old_len)?;
            Ok(comp_reps_res.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("-get_componentreplacementsrules failed\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outComponentReplacementRules)
}

fn getNewComponentreplacementsrulesForEachRule(
    mut inComponents: InteractiveTypes::Components,
    mut inComponentReplacementRules: InteractiveTypes::ComponentReplacementRules,
) -> Result<InteractiveTypes::ComponentReplacementRules> {
    let mut outComponentReplacementRules: InteractiveTypes::ComponentReplacementRules;
    outComponentReplacementRules = 'mc: {
        let __mc_input = (inComponents, inComponentReplacementRules);
        if let Ok(__v) = (|| -> Result<_> {
            let (_, mut comp_reps) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (emptyComponentReplacementRules(&(comp_reps.clone()))) else {
                return Err("pattern mismatch");
            };
            Ok(comp_reps.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut comps, mut comp_reps) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut comps_1: InteractiveTypes::Components;
            let mut comp_reps_1: InteractiveTypes::ComponentReplacementRules;
            let mut res: InteractiveTypes::ComponentReplacementRules;
            let mut comp_reps_2: InteractiveTypes::ComponentReplacementRules;
            let mut comp_reps_3: InteractiveTypes::ComponentReplacementRules;
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut cr1: metamodelica::Ref<Absyn::ComponentRef>;
            let mut cr2: metamodelica::Ref<Absyn::ComponentRef>;
            let InteractiveTypes::COMPONENTREPLACEMENT {
                which1: __pa0,
                the2: __pa1,
                the3: __pa2,
            } = firstComponentReplacement(&(comp_reps.clone()))?;
            path = metamodelica::Own::own(__pa0);
            cr1 = metamodelica::Own::own(__pa1);
            cr2 = metamodelica::Own::own(__pa2);
            comps_1 = getComponentsWithType(&(comps.clone()), &path);
            comp_reps_1 = makeComponentsReplacementRulesFromComponents(comps_1.clone(), cr1.clone(), cr2.clone())?;
            res = restComponentReplacementRules(&(comp_reps.clone()))?;
            comp_reps_2 = getNewComponentreplacementsrulesForEachRule(comps.clone(), res.clone())?;
            comp_reps_3 = joinComponentReplacementRules(&comp_reps_1, &comp_reps_2)?;
            Ok(comp_reps_3.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("-get_new_componentreplacementsrules_for_each_rule failed\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outComponentReplacementRules)
}

fn makeComponentsReplacementRulesFromComponents(
    mut inComponents1: InteractiveTypes::Components,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<InteractiveTypes::ComponentReplacementRules> {
    let mut outComponentReplacementRules: InteractiveTypes::ComponentReplacementRules;
    outComponentReplacementRules = 'mc: {
        let __mc_input = (inComponents1, inComponentRef2, inComponentRef3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (comps, _, _) => {
                    let true = (emptyComponents(metamodelica::AsArg::as_arg(&comps))) else { return Err("pattern mismatch") };
                    Ok(InteractiveTypes::ComponentReplacementRules { componentReplacementLst: metamodelica::nil(), the: 0 })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (comps, cr_from, cr_to) => {
                    let mut res: InteractiveTypes::Components;
                    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut cr_from_1: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut cr_to_1: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut path_class: metamodelica::Ref<Absyn::Path>;
                    let mut comp_rep: InteractiveTypes::ComponentReplacement;
                    let mut comps_1: InteractiveTypes::ComponentReplacementRules;
                    let mut comp_reps_res: InteractiveTypes::ComponentReplacementRules;
                    let InteractiveTypes::COMPONENTITEM { the1: __pa0, the2: _, the3: __pa1 } = (firstComponent(metamodelica::AsArg::as_arg(&comps))?) else { return Err("pattern mismatch") };
                    path_class = metamodelica::Own::own(__pa0);
                    cr = metamodelica::Own::own(__pa1);
                    cr_from_1 = AbsynUtil::joinCrefs(&cr, cr_from.clone())?;
                    cr_to_1 = AbsynUtil::joinCrefs(&cr, cr_to.clone())?;
                    comp_rep = InteractiveTypes::ComponentReplacement { which1: path_class.clone(), the2: cr_from_1.clone(), the3: cr_to_1.clone() };
                    res = restComponents(metamodelica::AsArg::as_arg(&comps))?;
                    comps_1 = makeComponentsReplacementRulesFromComponents(res.clone(), cr_from.clone(), cr_to.clone())?;
                    comp_reps_res = joinComponentReplacementRules(&comps_1, &(InteractiveTypes::ComponentReplacementRules { componentReplacementLst: list![comp_rep.clone()], the: 1 }))?;
                    Ok(comp_reps_res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (comps, cr_from, cr_to) => {
                    let mut res: InteractiveTypes::Components;
                    let mut path_class: metamodelica::Ref<Absyn::Path>;
                    let mut comp_rep: InteractiveTypes::ComponentReplacement;
                    let mut comps_1: InteractiveTypes::ComponentReplacementRules;
                    let mut comp_reps_res: InteractiveTypes::ComponentReplacementRules;
                    let InteractiveTypes::EXTENDSITEM { the1: __pa0, the2: _ } = (firstComponent(metamodelica::AsArg::as_arg(&comps))?) else { return Err("pattern mismatch") };
                    path_class = metamodelica::Own::own(__pa0);
                    comp_rep = InteractiveTypes::ComponentReplacement { which1: path_class.clone(), the2: cr_from.clone(), the3: cr_to.clone() };
                    res = restComponents(metamodelica::AsArg::as_arg(&comps))?;
                    comps_1 = makeComponentsReplacementRulesFromComponents(res.clone(), cr_from.clone(), cr_to.clone())?;
                    comp_reps_res = joinComponentReplacementRules(&comps_1, &(InteractiveTypes::ComponentReplacementRules { componentReplacementLst: list![comp_rep.clone()], the: 1 }))?;
                    Ok(comp_reps_res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("-make_componentsReplacementRules_from_components failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outComponentReplacementRules)
}

fn emptyComponentReplacementRules(
    mut inComponentReplacementRules: &InteractiveTypes::ComponentReplacementRules,
) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inComponentReplacementRules) {
        InteractiveTypes::ComponentReplacementRules { componentReplacementLst: Deref @ metamodelica::ListNode::Nil, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn joinComponentReplacementRules(
    mut inComponentReplacementRules1: &InteractiveTypes::ComponentReplacementRules,
    mut inComponentReplacementRules2: &InteractiveTypes::ComponentReplacementRules,
) -> Result<InteractiveTypes::ComponentReplacementRules> {
    let mut outComponentReplacementRules: InteractiveTypes::ComponentReplacementRules;
    outComponentReplacementRules = (match (
        inComponentReplacementRules1.clone(),
        inComponentReplacementRules2.clone(),
    ) {
        (
            InteractiveTypes::ComponentReplacementRules {
                componentReplacementLst: ref comps1,
                ..
            },
            InteractiveTypes::ComponentReplacementRules {
                componentReplacementLst: ref comps2,
                ..
            },
        ) => {
            let mut comps: metamodelica::List<InteractiveTypes::ComponentReplacement>;
            let mut len: i32;
            comps = List::union(
                metamodelica::AsArg::as_arg(&comps1),
                metamodelica::AsArg::as_arg(&comps2),
            );
            len = ((comps).len() as i32);
            InteractiveTypes::ComponentReplacementRules {
                componentReplacementLst: comps,
                the: len,
            }
        }
    });
    Ok(outComponentReplacementRules)
}

fn lengthComponentReplacementRules(
    mut inComponentReplacementRules: &InteractiveTypes::ComponentReplacementRules,
) -> i32 {
    let mut outInteger: i32;
    outInteger = (match inComponentReplacementRules.clone() {
        InteractiveTypes::ComponentReplacementRules { the: mut len, .. } => len.clone(),
    });
    outInteger
}

fn firstComponentReplacement(
    mut inComponentReplacementRules: &InteractiveTypes::ComponentReplacementRules,
) -> Result<InteractiveTypes::ComponentReplacement> {
    let mut outComponentReplacement: InteractiveTypes::ComponentReplacement;
    outComponentReplacement = (::match_deref::match_deref! { match &(inComponentReplacementRules) {
        InteractiveTypes::ComponentReplacementRules { componentReplacementLst: Deref @ metamodelica::ListNode::Nil, .. } => {
            metamodelica::print(literal!("-first_componentReplacement failed: no componentReplacementReplacementRules\n"));
            return Err("fail")
        },
        InteractiveTypes::ComponentReplacementRules { componentReplacementLst: Deref @ metamodelica::ListNode::Cons { head: comp, tail: _ }, .. } => {
            comp.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outComponentReplacement)
}

fn restComponentReplacementRules(
    mut inComponentReplacementRules: &InteractiveTypes::ComponentReplacementRules,
) -> Result<InteractiveTypes::ComponentReplacementRules> {
    let mut outComponentReplacementRules: InteractiveTypes::ComponentReplacementRules;
    outComponentReplacementRules = (::match_deref::match_deref! { match &(inComponentReplacementRules) {
        InteractiveTypes::ComponentReplacementRules { componentReplacementLst: Deref @ metamodelica::ListNode::Nil, .. } => {
            InteractiveTypes::ComponentReplacementRules { componentReplacementLst: metamodelica::nil(), the: 0 }
        },
        InteractiveTypes::ComponentReplacementRules { componentReplacementLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: res }, the: len } => {
            let mut len_1: i32;
            len_1 = len.clone() - 1;
            InteractiveTypes::ComponentReplacementRules { componentReplacementLst: res.clone(), the: len_1 }
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outComponentReplacementRules)
}

fn getComponentsWithType(
    mut inComponents: &InteractiveTypes::Components,
    mut inPath: &metamodelica::Ref<Absyn::Path>,
) -> InteractiveTypes::Components {
    let mut outComponents: InteractiveTypes::Components;
    let mut comps: metamodelica::List<InteractiveTypes::Component>;
    comps = ({
        let mut __acc: metamodelica::List<InteractiveTypes::Component> = metamodelica::nil();
        for mut comp in (inComponents.componentLst.clone()).into_iter().cloned() {
            if !(AbsynUtil::pathEqual(inPath, &(componentTypePath(&(comp.clone()))))) {
                continue;
            }
            let __x = comp.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outComponents = InteractiveTypes::Components {
        componentLst: comps.clone(),
        the: ((comps).len() as i32),
    };
    outComponents
}

fn extractAllComponents(
    mut p: Absyn::Program,
    mut path: &metamodelica::Ref<Absyn::Path>,
) -> Result<InteractiveTypes::Components> {
    let mut comps: InteractiveTypes::Components;
    comps = (match &**path {
        _ => {
            let mut p_1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut env: FCore::Graph;
            p_1 = AbsynToSCode::translateAbsyn2SCode(p.clone())?;
            (_, env) = Inst::makeEnvFromProgram(&p_1)?;
            let (_, _, (__pa0, _, _)) = AbsynUtil::traverseClasses(
                p.clone(),
                None,
                (std::sync::Arc::new(
                    move |__a0: (
                        metamodelica::Ref<Absyn::Class>,
                        Option<metamodelica::Ref<Absyn::Path>>,
                        (InteractiveTypes::Components, Absyn::Program, FCore::Graph),
                    )|
                          -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(extractAllComponentsVisitor(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                (
                                    metamodelica::Ref<Absyn::Class>,
                                    Option<metamodelica::Ref<Absyn::Path>>,
                                    (InteractiveTypes::Components, Absyn::Program, FCore::Graph),
                                ),
                            ) -> Result<(
                                metamodelica::Ref<Absyn::Class>,
                                Option<metamodelica::Ref<Absyn::Path>>,
                                (InteractiveTypes::Components, Absyn::Program, FCore::Graph),
                            )> + 'static,
                    >),
                (
                    InteractiveTypes::Components {
                        componentLst: metamodelica::nil(),
                        the: 0,
                    },
                    p,
                    env,
                ),
                true,
            )?;
            comps = metamodelica::Own::own(__pa0);
            comps
        }
    });
    Ok(comps)
}

fn extractAllComponentsVisitor(
    mut inTplAbsynClassAbsynPathOptionTplComponentsAbsynProgramEnvEnv: &(
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        (InteractiveTypes::Components, Absyn::Program, FCore::Graph),
    ),
) -> (
    metamodelica::Ref<Absyn::Class>,
    Option<metamodelica::Ref<Absyn::Path>>,
    (InteractiveTypes::Components, Absyn::Program, FCore::Graph),
) {
    let mut outTplAbsynClassAbsynPathOptionTplComponentsAbsynProgramEnvEnv: (
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        (InteractiveTypes::Components, Absyn::Program, FCore::Graph),
    );
    outTplAbsynClassAbsynPathOptionTplComponentsAbsynProgramEnvEnv = 'mc: {
        let __mc_input = inTplAbsynClassAbsynPathOptionTplComponentsAbsynProgramEnvEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_ @ Deref @ Absyn::Class { name: id, info: file_info, .. }, Some(pa), (comps, p, env)) => {
                    let mut path_1: metamodelica::Ref<Absyn::Path>;
                    let mut pa_1: metamodelica::Ref<Absyn::Path>;
                    let mut cenv: FCore::Graph;
                    let mut comps_1: InteractiveTypes::Components;
                    let false = (isReadOnly(metamodelica::AsArg::as_arg(&file_info))?) else { return Err("pattern mismatch") };
                    path_1 = AbsynUtil::joinPaths(pa.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() }))?;
                    cenv = getClassEnvNoElaboration(metamodelica::AsArg::as_arg(&p), &path_1, metamodelica::AsArg::as_arg(&env))?;
                    (_, pa_1) = Inst::makeFullyQualified(FCore::emptyCache(), cenv.clone(), path_1.clone())?;
                    comps_1 = extractComponentsFromClass(metamodelica::AsArg::as_arg(&class_), pa_1.clone(), comps.clone(), cenv.clone())?;
                    Ok((class_.clone(), Some(pa.clone()), (comps_1.clone(), p.clone(), env.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_ @ Deref @ Absyn::Class { name: id, info: file_info, .. }, None, (comps, p, env)) => {
                    let mut path_1: metamodelica::Ref<Absyn::Path>;
                    let mut pa_1: metamodelica::Ref<Absyn::Path>;
                    let mut cenv: FCore::Graph;
                    let mut comps_1: InteractiveTypes::Components;
                    let false = (isReadOnly(metamodelica::AsArg::as_arg(&file_info))?) else { return Err("pattern mismatch") };
                    path_1 = metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() });
                    cenv = getClassEnvNoElaboration(metamodelica::AsArg::as_arg(&p), &path_1, metamodelica::AsArg::as_arg(&env))?;
                    (_, pa_1) = Inst::makeFullyQualified(FCore::emptyCache(), cenv.clone(), path_1.clone())?;
                    comps_1 = extractComponentsFromClass(metamodelica::AsArg::as_arg(&class_), pa_1.clone(), comps.clone(), cenv.clone())?;
                    Ok((class_.clone(), None, (comps_1.clone(), p.clone(), env.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_, paOpt, (comps, p, env)) => {
                    Ok((class_.clone(), paOpt.clone(), (comps.clone(), p.clone(), env.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outTplAbsynClassAbsynPathOptionTplComponentsAbsynProgramEnvEnv
}

fn isReadOnly(mut file_info: &SourceInfo) -> Result<bool> {
    let mut res: bool;
    res = (match file_info.clone() {
        SourceInfo {
            isReadOnly: mut __esc_res,
            ..
        } => {
            res = __esc_res.clone();
            res
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(res)
}

fn extractComponentsFromClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inComponents: InteractiveTypes::Components,
    mut inEnv: FCore::Graph,
) -> Result<InteractiveTypes::Components> {
    let mut outComponents: InteractiveTypes::Components;
    outComponents = (match &**inClass {
        Absyn::Class { body: classdef, .. } => {
            let mut pa = inPath;
            let mut comps = inComponents;
            let mut env = inEnv;
            let mut comps_1: InteractiveTypes::Components;
            comps_1 = extractComponentsFromClassdef(pa, classdef, comps, env);
            comps_1
        }
        _ => {
            metamodelica::print(literal!("-extract_components_from_class failed\n"));
            return Err("fail");
        }
    });
    Ok(outComponents)
}

fn extractComponentsFromClassdef(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inClassDef: &metamodelica::Ref<Absyn::ClassDef>,
    mut inComponents: InteractiveTypes::Components,
    mut inEnv: FCore::Graph,
) -> InteractiveTypes::Components {
    let mut outComponents: InteractiveTypes::Components;
    outComponents = 'mc: {
        let __mc_input = (inPath, &**inClassDef, inComponents.clone(), inEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (pa, Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, comps, env) => {
                    let mut comps_1: InteractiveTypes::Components;
                    comps_1 = extractComponentsFromClassparts(pa.clone(), metamodelica::AsArg::as_arg(&parts), comps.clone(), env.clone());
                    Ok(comps_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (pa, Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: _, arrayDim: _ }, arguments: elementargs, .. }, comps, env) => {
                    let mut comps_1: InteractiveTypes::Components;
                    comps_1 = extractComponentsFromElementargs(pa.clone(), metamodelica::AsArg::as_arg(&elementargs), comps.clone(), env.clone())?;
                    Ok(comps_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (pa, Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, comps, env) => {
                    let mut comps_1: InteractiveTypes::Components;
                    comps_1 = extractComponentsFromClassparts(pa.clone(), metamodelica::AsArg::as_arg(&parts), comps.clone(), env.clone());
                    Ok(comps_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inComponents.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outComponents
}

fn extractComponentsFromClassparts(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inAbsynClassPartLst: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inComponents: InteractiveTypes::Components,
    mut inEnv: FCore::Graph,
) -> InteractiveTypes::Components {
    let mut outComponents: InteractiveTypes::Components;
    outComponents = 'mc: {
        let __mc_input = (inPath, &**inAbsynClassPartLst, inComponents.clone(), inEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, comps, _) => {
                    Ok(comps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (pa, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elements }, tail: res }, comps, env) => {
                    let mut comps_1: InteractiveTypes::Components;
                    let mut comps_2: InteractiveTypes::Components;
                    comps_1 = extractComponentsFromClassparts(pa.clone(), metamodelica::AsArg::as_arg(&res), comps.clone(), env.clone());
                    comps_2 = extractComponentsFromElements(pa.clone(), metamodelica::AsArg::as_arg(&elements), comps_1.clone(), env.clone())?;
                    Ok(comps_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (pa, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elements }, tail: res }, comps, env) => {
                    let mut comps_1: InteractiveTypes::Components;
                    let mut comps_2: InteractiveTypes::Components;
                    comps_1 = extractComponentsFromClassparts(pa.clone(), metamodelica::AsArg::as_arg(&res), comps.clone(), env.clone());
                    comps_2 = extractComponentsFromElements(pa.clone(), metamodelica::AsArg::as_arg(&elements), comps_1.clone(), env.clone())?;
                    Ok(comps_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inComponents.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outComponents
}

fn extractComponentsFromElements(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inAbsynElementItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inComponents: InteractiveTypes::Components,
    mut inEnv: FCore::Graph,
) -> Result<InteractiveTypes::Components> {
    let mut outComponents: InteractiveTypes::Components;
    outComponents = 'mc: {
        let __mc_input = (inPath, &**inAbsynElementItemLst, inComponents, inEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, comps, _) => {
                    Ok(comps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (pa, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: elementspec, .. } }, tail: res }, comps, env) => {
                    let mut comps_1: InteractiveTypes::Components;
                    let mut comps_2: InteractiveTypes::Components;
                    comps_1 = extractComponentsFromElements(pa.clone(), metamodelica::AsArg::as_arg(&res), comps.clone(), env.clone())?;
                    comps_2 = extractComponentsFromElementspec(pa.clone(), metamodelica::AsArg::as_arg(&elementspec), comps_1.clone(), env.clone());
                    Ok(comps_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (pa, Deref @ metamodelica::ListNode::Cons { head: _, tail: res }, comps, env) => {
                    let mut comps = (*comps).clone();
                    comps = extractComponentsFromElements(pa.clone(), metamodelica::AsArg::as_arg(&res), comps.clone(), env.clone())?;
                    Ok(comps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outComponents)
}

fn extractComponentsFromElementspec(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inElementSpec: &metamodelica::Ref<Absyn::ElementSpec>,
    mut inComponents: InteractiveTypes::Components,
    mut inEnv: FCore::Graph,
) -> InteractiveTypes::Components {
    let mut outComponents: InteractiveTypes::Components;
    outComponents = 'mc: {
        let __mc_input = (inPath, &**inElementSpec, inComponents.clone(), inEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (pa, Deref @ Absyn::ElementSpec::COMPONENTS { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: path_1, arrayDim: _ }, components: comp_items, .. }, comps, env) => {
                    let mut id: ArcStr;
                    let mut cenv: FCore::Graph;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut comps_1: InteractiveTypes::Components;
                    let mut cache: FCore::Cache;
                    let mut path_1 = (*path_1).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(&(FCore::emptyCache()), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&path_1), None)?) {
                        (__pa0, Deref @ SCode::Element::CLASS { name: __pa1, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    id = metamodelica::Own::own(__pa1);
                    cenv = metamodelica::Own::own(__pa2);
                    path_1 = metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() });
                    (cache, path) = Inst::makeFullyQualified(cache.clone(), cenv.clone(), path_1.clone())?;
                    comps_1 = extractComponentsFromComponentitems(pa.clone(), path.clone(), metamodelica::AsArg::as_arg(&comp_items), comps.clone(), env.clone())?;
                    Ok(comps_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (pa, Deref @ Absyn::ElementSpec::EXTENDS { path: path_1, elementArg: elementargs, .. }, comps, env) => {
                    let mut cenv: FCore::Graph;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut comps_1: InteractiveTypes::Components;
                    let mut comps_2: InteractiveTypes::Components;
                    let mut comp: InteractiveTypes::Component;
                    let mut cache: FCore::Cache;
                    (cache, _, cenv) = Lookup::lookupClass(&(FCore::emptyCache()), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&path_1), None)?;
                    (_, path) = Inst::makeFullyQualified(cache.clone(), cenv.clone(), path_1.clone())?;
                    comp = InteractiveTypes::Component::EXTENDSITEM { the1: pa.clone(), the2: path.clone() };
                    comps_1 = addComponentToComponents(comp.clone(), metamodelica::AsArg::as_arg(&comps))?;
                    comps_2 = extractComponentsFromElementargs(pa.clone(), metamodelica::AsArg::as_arg(&elementargs), comps_1.clone(), env.clone())?;
                    Ok(comps_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inComponents.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outComponents
}

fn extractComponentsFromComponentitems(
    mut inPath1: metamodelica::Ref<Absyn::Path>,
    mut inPath2: metamodelica::Ref<Absyn::Path>,
    mut inAbsynComponentItemLst3: &metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut inComponents4: InteractiveTypes::Components,
    mut inEnv5: FCore::Graph,
) -> Result<InteractiveTypes::Components> {
    let mut outComponents: InteractiveTypes::Components;
    outComponents = 'mc: {
        let __mc_input = (inPath1, inPath2, &**inAbsynComponentItemLst3, inComponents4, inEnv5);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Nil, comps, _) => {
                    Ok(comps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (pa, path, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: id, modification: mod_opt, .. }, .. }, tail: res }, comps, env) => {
                    let mut comps_1: InteractiveTypes::Components;
                    let mut comps_2: InteractiveTypes::Components;
                    let mut comps_3: InteractiveTypes::Components;
                    let mut comp: metamodelica::Ref<Absyn::ComponentRef>;
                    comps_1 = extractComponentsFromComponentitems(pa.clone(), path.clone(), metamodelica::AsArg::as_arg(&res), comps.clone(), env.clone())?;
                    comp = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: id.clone(), subscripts: metamodelica::nil() });
                    comps_2 = addComponentToComponents(InteractiveTypes::Component::COMPONENTITEM { the1: pa.clone(), the2: path.clone(), the3: comp.clone() }, &comps_1)?;
                    comps_3 = extractComponentsFromModificationOption(pa.clone(), mod_opt.clone(), comps_2.clone(), env.clone())?;
                    Ok(comps_3.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("-extract_components_from_componentitems failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outComponents)
}

fn extractComponentsFromElementargs(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inAbsynElementArgLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inComponents: InteractiveTypes::Components,
    mut inEnv: FCore::Graph,
) -> Result<InteractiveTypes::Components> {
    let mut outComponents: InteractiveTypes::Components;
    outComponents = 'mc: {
        let __mc_input = (inPath, &**inAbsynElementArgLst, inComponents, inEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, comps, _) => {
                    Ok(comps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (pa, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::REDECLARATION { elementSpec: elementspec, constrainClass: Some(Deref @ Absyn::ConstrainClass { elementSpec: elementspec2, comment: _ }), .. }, tail: res }, comps, env) => {
                    let mut comps_1: InteractiveTypes::Components;
                    let mut comps_2: InteractiveTypes::Components;
                    let mut comps_3: InteractiveTypes::Components;
                    comps_1 = extractComponentsFromElementspec(pa.clone(), metamodelica::AsArg::as_arg(&elementspec), comps.clone(), env.clone());
                    comps_2 = extractComponentsFromElementspec(pa.clone(), metamodelica::AsArg::as_arg(&elementspec2), comps_1.clone(), env.clone());
                    comps_3 = extractComponentsFromElementargs(pa.clone(), metamodelica::AsArg::as_arg(&res), comps_2.clone(), env.clone())?;
                    Ok(comps_3.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (pa, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::REDECLARATION { elementSpec: elementspec, constrainClass: Some(_), .. }, tail: res }, comps, env) => {
                    let mut comps_1: InteractiveTypes::Components;
                    let mut comps_2: InteractiveTypes::Components;
                    comps_1 = extractComponentsFromElementspec(pa.clone(), metamodelica::AsArg::as_arg(&elementspec), comps.clone(), env.clone());
                    comps_2 = extractComponentsFromElementargs(pa.clone(), metamodelica::AsArg::as_arg(&res), comps_1.clone(), env.clone())?;
                    Ok(comps_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (pa, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: mod_opt, .. }, tail: res }, comps, env) => {
                    let mut comps_1: InteractiveTypes::Components;
                    let mut comps_2: InteractiveTypes::Components;
                    comps_1 = extractComponentsFromModificationOption(pa.clone(), mod_opt.clone(), comps.clone(), env.clone())?;
                    comps_2 = extractComponentsFromElementargs(pa.clone(), metamodelica::AsArg::as_arg(&res), comps_1.clone(), env.clone())?;
                    Ok(comps_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (pa, Deref @ metamodelica::ListNode::Cons { head: _, tail: res }, comps, env) => {
                    let mut comps_1: InteractiveTypes::Components;
                    comps_1 = extractComponentsFromElementargs(pa.clone(), metamodelica::AsArg::as_arg(&res), comps.clone(), env.clone())?;
                    Ok(comps_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outComponents)
}

fn extractComponentsFromModificationOption(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inAbsynModificationOption: Option<metamodelica::Ref<Absyn::Modification>>,
    mut inComponents: InteractiveTypes::Components,
    mut inEnv: FCore::Graph,
) -> Result<InteractiveTypes::Components> {
    let mut outComponents: InteractiveTypes::Components;
    outComponents = (::match_deref::match_deref! { match &(inAbsynModificationOption) {
        None => {
            let mut comps = inComponents;
            comps
        },
        Some(Deref @ Absyn::Modification { elementArgLst: elementargs, eqMod: _ }) => {
            let mut pa = inPath;
            let mut comps = inComponents;
            let mut env = inEnv;
            let mut comps_1: InteractiveTypes::Components;
            comps_1 = extractComponentsFromElementargs(pa, metamodelica::AsArg::as_arg(&elementargs), comps, env)?;
            comps_1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outComponents)
}

fn emptyComponents(mut inComponents: &InteractiveTypes::Components) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inComponents) {
        InteractiveTypes::Components { componentLst: Deref @ metamodelica::ListNode::Nil, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn firstComponent(mut inComponents: &InteractiveTypes::Components) -> Result<InteractiveTypes::Component> {
    let mut outComponent: InteractiveTypes::Component;
    outComponent = (::match_deref::match_deref! { match &(inComponents) {
        InteractiveTypes::Components { componentLst: Deref @ metamodelica::ListNode::Nil, .. } => {
            metamodelica::print(literal!("-first_component failed: no components\n"));
            return Err("fail")
        },
        InteractiveTypes::Components { componentLst: Deref @ metamodelica::ListNode::Cons { head: comp, tail: _ }, .. } => {
            comp.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outComponent)
}

fn restComponents(mut inComponents: &InteractiveTypes::Components) -> Result<InteractiveTypes::Components> {
    let mut outComponents: InteractiveTypes::Components;
    outComponents = (::match_deref::match_deref! { match &(inComponents) {
        InteractiveTypes::Components { componentLst: Deref @ metamodelica::ListNode::Nil, .. } => {
            InteractiveTypes::Components { componentLst: metamodelica::nil(), the: 0 }
        },
        InteractiveTypes::Components { componentLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: res }, the: len } => {
            let mut len_1: i32;
            len_1 = len.clone() - 1;
            InteractiveTypes::Components { componentLst: res.clone(), the: len_1 }
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outComponents)
}

fn addComponentToComponents(
    mut inComponent: InteractiveTypes::Component,
    mut inComponents: &InteractiveTypes::Components,
) -> Result<InteractiveTypes::Components> {
    let mut outComponents: InteractiveTypes::Components;
    outComponents = (match (inComponent, inComponents.clone()) {
        (
            mut comp,
            InteractiveTypes::Components {
                componentLst: ref comps,
                the: mut len,
            },
        ) => {
            let mut len_1: i32;
            len_1 = len.clone() + 1;
            InteractiveTypes::Components {
                componentLst: metamodelica::cons(comp, comps.clone()),
                the: len_1,
            }
        }
    });
    Ok(outComponents)
}

fn componentTypePath(mut comp: &InteractiveTypes::Component) -> metamodelica::Ref<Absyn::Path> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    path = (match comp.clone() {
        InteractiveTypes::Component::COMPONENTITEM { .. } => {
            var_field!(comp.the2, InteractiveTypes::Component::COMPONENTITEM).clone()
        }
        InteractiveTypes::Component::EXTENDSITEM { .. } => {
            var_field!(comp.the2, InteractiveTypes::Component::EXTENDSITEM).clone()
        }
    });
    path
}

fn isParameterElement(mut inElement: &metamodelica::Ref<Absyn::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inElement {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { variability: Absyn::Variability::PARAM { .. }, .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn getParameterNames(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
) -> metamodelica::List<ArcStr> {
    let mut outList: metamodelica::List<ArcStr>;
    outList = 'mc: {
        let __mc_input = inProgram;
        if let Ok(__v) = (|| -> Result<_> {
            let mut p = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut cdef: metamodelica::Ref<Absyn::Class>;
            let mut comps: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut compelts: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>>;
            let mut compelts_1: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
            let mut names: metamodelica::List<ArcStr>;
            cdef = ProgramUtil::getPathedClassInProgram(path.clone(), &(p.clone()), false, false)?;
            comps = InteractiveUtil::getComponentsInClass(&cdef, InteractiveUtil::Visibility::ANY.clone());
            compelts = ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>> =
                    metamodelica::nil();
                for mut c in (comps.clone()).into_iter().cloned() {
                    if !(isParameterElement(&(c.clone()))) {
                        continue;
                    }
                    let __x = InteractiveUtil::getComponentitemsInElement(&(c.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            compelts_1 = List::flatten(compelts.clone())?;
            names = List::map(compelts_1.clone(), &move |__a0: metamodelica::Ref<
                Absyn::ComponentItem,
            >| getComponentitemName(&__a0))?;
            Ok(names.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(metamodelica::nil())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outList
}

pub(crate) fn getClassEnv(
    mut p: Absyn::Program,
    mut p_class: metamodelica::Ref<Absyn::Path>,
) -> Result<GraphicEnvCache> {
    let mut env_2: GraphicEnvCache;
    let mut ocache: Option<metamodelica::List<(Absyn::Program, metamodelica::Ref<Absyn::Path>, GraphicEnvCache)>>;
    let mut cache: metamodelica::List<(Absyn::Program, metamodelica::Ref<Absyn::Path>, GraphicEnvCache)>;
    let mut po: Absyn::Program;
    let mut patho: metamodelica::Ref<Absyn::Path>;
    let mut envo: GraphicEnvCache;
    let mut invalidate: bool = false;
    let mut fcache: FCore::Cache;
    let mut env: FCore::Graph;
    if Flags::isSet(Flags::NF_API.clone())? {
        env_2 = GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE {
            program: p,
            modelPath: p_class,
            cache: FCore::emptyCache(),
            env: FGraph::empty(),
        };
        return Ok(env_2);
    }
    ocache = crate::Globals::interactiveCache.with(|__root| __root.borrow().clone());
    if (ocache).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(ocache.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        cache = metamodelica::Own::own(__pa0);
        for mut x in &*cache {
            (po, patho, envo) = x.clone();
            if AbsynUtil::pathEqual(&patho, &p_class) {
                if {
                    let __refeq_sl = &(po);
                    let __refeq_sr = &(p.clone());
                    metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.classes), &(__refeq_sr.classes))
                        && (match (&(__refeq_sl.within_), &(__refeq_sr.within_)) {
                            (Absyn::Within::TOP, Absyn::Within::TOP) => true,
                            (
                                Absyn::Within::WITHIN { path: __refeq_v0l },
                                Absyn::Within::WITHIN { path: __refeq_v0r },
                            ) => referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r)),
                            _ => false,
                        })
                } {
                    env_2 = envo;
                    return Ok(env_2);
                } else {
                    invalidate = true;
                    break;
                }
            }
        }
        if invalidate {
            let __pa1 = ::match_deref::match_deref! { match &(ocache) {
                Some(__pa1) => __pa1.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa1);
            (cache, _) = List::deleteMemberOnTrue(
                p_class.clone(),
                cache,
                &move |__a0: metamodelica::Ref<Absyn::Path>,
                       __a1: (Absyn::Program, metamodelica::Ref<Absyn::Path>, GraphicEnvCache)|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(matchPath(&__a0, &__a1))
                },
            )?;
            {
                let __v = Some(cache);
                crate::Globals::interactiveCache.with(|__root| *__root.borrow_mut() = __v)
            };
        }
    }
    (fcache, env) = getClassEnv_dispatch(p.clone(), &p_class)?;
    env_2 = GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE {
        program: p.clone(),
        modelPath: p_class.clone(),
        cache: fcache,
        env: env,
    };
    ocache = crate::Globals::interactiveCache.with(|__root| __root.borrow().clone());
    if (ocache).is_some() {
        let __pa2 = ::match_deref::match_deref! { match &(ocache) {
            Some(__pa2) => __pa2.clone(),
            _ => return Err("pattern mismatch"),
        } };
        cache = metamodelica::Own::own(__pa2);
        {
            let __v = Some(metamodelica::cons((p, p_class, env_2.clone()), cache));
            crate::Globals::interactiveCache.with(|__root| *__root.borrow_mut() = __v)
        };
    } else {
        {
            let __v = Some(metamodelica::cons((p, p_class, env_2.clone()), metamodelica::nil()));
            crate::Globals::interactiveCache.with(|__root| *__root.borrow_mut() = __v)
        };
    }
    Ok(env_2)
}

pub(crate) fn matchPath(
    mut p: &metamodelica::Ref<Absyn::Path>,
    mut entry: &(Absyn::Program, metamodelica::Ref<Absyn::Path>, GraphicEnvCache),
) -> bool {
    let mut matches: bool;
    let mut po: metamodelica::Ref<Absyn::Path>;
    (_, po, _) = entry.clone();
    matches = AbsynUtil::pathEqual(&po, p);
    matches
}

fn getClassEnv_dispatch(
    mut p: Absyn::Program,
    mut p_class: &metamodelica::Ref<Absyn::Path>,
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut cache: FCore::Cache;
    let mut env_2: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut p_1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut env: FCore::Graph;
    let mut env_1: FCore::Graph;
    let mut env2: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut cl: metamodelica::Ref<SCode::Element>;
    let mut id: ArcStr;
    let mut encflag: SCode::Encapsulated;
    let mut restr: SCode::Restriction;
    let mut ci_state: ClassInf::State = <ClassInf::State as ::std::default::Default>::default();
    p_1 = AbsynToSCode::translateAbsyn2SCode(p)?;
    (cache, env) = Inst::makeEnvFromProgram(&p_1)?;
    (cache, cl, env_1) = Lookup::lookupClass(&cache, &env, p_class, None)?;
    env_2 = 'mc: {
        let __mc_input = &*cl;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: id, encapsulatedPrefix: encflag, restriction: restr, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: _, arrayDim: _ }, .. }, .. } => {
                    Ok(env_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: id, encapsulatedPrefix: encflag, restriction: restr, .. } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut ci_state: ClassInf::State = ci_state.clone();
                    let mut env2: FCore::Graph = env2.clone();
                    let mut env_2: FCore::Graph = env_2.clone();
                    env2 = FGraph::openScope(env_1.clone(), encflag.clone(), id.clone(), FGraph::restrictionToScopeType(metamodelica::AsArg::as_arg(&restr)))?;
                    ci_state = ClassInfUtil::start(metamodelica::AsArg::as_arg(&restr), FGraph::getGraphName(&env2)?)?;
                    (cache, env_2, _, _, _) = Inst::partialInstClassIn(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), cl.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0)?;
                    Ok((env_2.clone(), cache.clone(), ci_state.clone(), env2.clone(), env_2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            ci_state = __wb1;
            env2 = __wb2;
            env_2 = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(FGraph::empty())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((cache, env_2))
}

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ComponentProperties {
    pub isFinal: bool,
    pub isFlow: bool,
    pub isStream: bool,
    pub isProtected: bool,
    pub isReplaceable: bool,
    pub variability: Absyn::Variability,
    pub innerOuter: Absyn::InnerOuter,
    pub direction: Absyn::Direction,
}

impl metamodelica::gc::MMTrace for ComponentProperties {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.isFinal, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isFlow, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isStream, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isProtected, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isReplaceable, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.variability, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.innerOuter, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.direction, __mmv)?;
        Ok(())
    }
}
impl Default for ComponentProperties {
    fn default() -> Self {
        Self {
            isFinal: Default::default(),
            isFlow: Default::default(),
            isStream: Default::default(),
            isProtected: Default::default(),
            isReplaceable: Default::default(),
            variability: Default::default(),
            innerOuter: Default::default(),
            direction: Default::default(),
        }
    }
}

pub type PROPERTIES = ComponentProperties;

pub(crate) fn setComponentProperties(
    mut classPath: &metamodelica::Ref<Absyn::Path>,
    mut component: &ArcStr,
    mut prefixes: &metamodelica::List<bool>,
    mut variability: &ArcStr,
    mut innerPrefix: bool,
    mut outerPrefix: bool,
    mut direction: &ArcStr,
    mut program: Absyn::Program,
) -> (Absyn::Program, metamodelica::Ref<Values::Value>) {
    let mut program: Absyn::Program = program;
    let mut result: metamodelica::Ref<Values::Value>;
    let mut is_final: bool;
    let mut is_flow: bool;
    let mut is_stream: bool;
    let mut is_protected: bool;
    let mut is_replaceable: bool;
    let mut props: ComponentProperties;
    match '__try0: {
        if ((prefixes).len() as i32) == 5 {
            let (__pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &((*prefixes)) {
                Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Nil } } } } } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            is_final = metamodelica::Own::own(__pa1);
            is_flow = metamodelica::Own::own(__pa2);
            is_stream = metamodelica::Own::own(__pa3);
            is_protected = metamodelica::Own::own(__pa4);
            is_replaceable = metamodelica::Own::own(__pa5);
            let false = (is_flow && is_stream) else {
                break '__try0 Err::<_, _>("pattern mismatch");
            };
        } else {
            let (__pa7, __pa8, __pa9, __pa10) = ::match_deref::match_deref! { match &((*prefixes)) {
                Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: Deref @ metamodelica::ListNode::Cons { head: __pa10, tail: Deref @ metamodelica::ListNode::Nil } } } } => (__pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            is_final = metamodelica::Own::own(__pa7);
            is_flow = metamodelica::Own::own(__pa8);
            is_protected = metamodelica::Own::own(__pa9);
            is_replaceable = metamodelica::Own::own(__pa10);
            is_stream = false;
        }
        props = ComponentProperties {
            isFinal: is_final,
            isFlow: is_flow,
            isStream: is_stream,
            isProtected: is_protected,
            isReplaceable: is_replaceable,
            variability: unwrap_break_err!(setElementVariability(variability), '__try0),
            innerOuter: setInnerOuterAttributes(innerPrefix, outerPrefix),
            direction: unwrap_break_err!(setElementCausality(direction), '__try0),
        };
        program = unwrap_break_err!(transformPathedClassInProgram(classPath, &program, (std::sync::Arc::new({ let __pe_b1 = component.clone(); let __pe_b2 = props; move |__pe_a0| setComponentPropertiesInClass(__pe_a0, &__pe_b1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static>)), '__try0);
        result = ValuesMake::makeBoolean(true);
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    (program, result)
}

fn setComponentPropertiesInClass(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut component: &ArcStr,
    mut properties: ComponentProperties,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut body: metamodelica::Ref<Absyn::ClassDef>;
    body = cls.body.clone();
    assign_field!(
        cls.body = (match &*body {
            Absyn::ClassDef::PARTS {
                classParts: __body_classParts,
                ..
            } => {
                assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = setComponentPropertiesInClassparts(__body_classParts.clone(), component, properties)?);
                body
            }
            Absyn::ClassDef::CLASS_EXTENDS {
                parts: __body_parts, ..
            } => {
                assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = setComponentPropertiesInClassparts(__body_parts.clone(), component, properties)?);
                body
            }
            _ => return Err("match: no arm matched"),
        })
    );
    Ok(cls)
}

fn setComponentPropertiesInClassparts(
    mut inParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut component: &ArcStr,
    mut properties: ComponentProperties,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outParts = 'mc: {
        let __mc_input = inParts;
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
                parts => {
                    if !((properties.isProtected.clone())) { return Err("guard") }
                    let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut protlst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut elt: metamodelica::Ref<Absyn::Element>;
                    let mut parts = (*parts).clone();
                    publst = ProgramUtil::getPublicList(metamodelica::AsArg::as_arg(&parts));
                    let __pa0 = ::match_deref::match_deref! { match &(List::getMemberOnTrue(component.clone(), &publst, &move |__a0: ArcStr, __a1: metamodelica::Ref<Absyn::ElementItem>| AbsynUtil::isElementItemNamed(&__a0, &__a1))?) {
                        Deref @ Absyn::ElementItem::ELEMENTITEM { element: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    elt = metamodelica::Own::own(__pa0);
                    elt = setComponentPropertiesInElement(elt.clone(), component, properties)?;
                    (publst, _) = deleteOrUpdateComponentFromElementitems(component.clone(), &publst, None)?;
                    protlst = ProgramUtil::getProtectedList(metamodelica::AsArg::as_arg(&parts));
                    protlst = List::appendElt(metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: elt.clone() }), protlst.clone());
                    parts = ProgramUtil::replaceProtectedList(metamodelica::AsArg::as_arg(&parts), protlst.clone())?;
                    parts = ProgramUtil::replacePublicList(metamodelica::AsArg::as_arg(&parts), publst.clone())?;
                    Ok(parts.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                parts => {
                    if !((!(properties.isProtected.clone()))) { return Err("guard") }
                    let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut protlst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut elt: metamodelica::Ref<Absyn::Element>;
                    let mut parts = (*parts).clone();
                    protlst = ProgramUtil::getProtectedList(metamodelica::AsArg::as_arg(&parts));
                    let __pa0 = ::match_deref::match_deref! { match &(List::getMemberOnTrue(component.clone(), &protlst, &move |__a0: ArcStr, __a1: metamodelica::Ref<Absyn::ElementItem>| AbsynUtil::isElementItemNamed(&__a0, &__a1))?) {
                        Deref @ Absyn::ElementItem::ELEMENTITEM { element: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    elt = metamodelica::Own::own(__pa0);
                    elt = setComponentPropertiesInElement(elt.clone(), component, properties)?;
                    (protlst, _) = deleteOrUpdateComponentFromElementitems(component.clone(), &protlst, None)?;
                    publst = ProgramUtil::getPublicList(metamodelica::AsArg::as_arg(&parts));
                    publst = List::appendElt(metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: elt.clone() }), publst.clone());
                    parts = ProgramUtil::replacePublicList(metamodelica::AsArg::as_arg(&parts), publst.clone())?;
                    parts = ProgramUtil::replaceProtectedList(metamodelica::AsArg::as_arg(&parts), protlst.clone())?;
                    Ok(parts.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: rest } => {
                    let mut elts = (*elts).clone();
                    let mut rest = (*rest).clone();
                    rest = setComponentPropertiesInClassparts(rest.clone(), component, properties)?;
                    elts = setComponentPropertiesInElementitems(elts.clone(), component, properties)?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: elts.clone() }), rest.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: rest } => {
                    let mut elts = (*elts).clone();
                    let mut rest = (*rest).clone();
                    rest = setComponentPropertiesInClassparts(rest.clone(), component, properties)?;
                    elts = setComponentPropertiesInElementitems(elts.clone(), component, properties)?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: elts.clone() }), rest.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: part, tail: rest } => {
                    let mut rest = (*rest).clone();
                    rest = setComponentPropertiesInClassparts(rest.clone(), component, properties)?;
                    Ok(metamodelica::cons(part.clone(), rest.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outParts)
}

fn setComponentPropertiesInElementitems(
    mut items: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut component: &ArcStr,
    mut properties: ComponentProperties,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    let mut items: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = items;
    (items, _) = List::findAndMap(
        items,
        &({
            let __pe_b0 = component.clone();
            move |__pe_a1| AbsynUtil::isElementItemNamed(&__pe_b0, &__pe_a1)
        }),
        &({
            let __pe_b1 = component.clone();
            let __pe_b2 = properties;
            move |__pe_a0| setComponentPropertiesInElementItem(__pe_a0, &__pe_b1, __pe_b2.clone())
        }),
    )?;
    Ok(items)
}

fn setComponentPropertiesInElementItem(
    mut item: metamodelica::Ref<Absyn::ElementItem>,
    mut component: &ArcStr,
    mut properties: ComponentProperties,
) -> Result<metamodelica::Ref<Absyn::ElementItem>> {
    let mut item: metamodelica::Ref<Absyn::ElementItem> = item;
    let () = (match &*item {
        Absyn::ElementItem::ELEMENTITEM {
            element: __item_element,
        } => {
            assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = setComponentPropertiesInElement(__item_element.clone(), component, properties)?);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(item)
}

fn setComponentPropertiesInElement(
    mut element: metamodelica::Ref<Absyn::Element>,
    mut component: &ArcStr,
    mut properties: ComponentProperties,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let mut spec: metamodelica::Ref<Absyn::ElementSpec>;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: __esc_spec @ Deref @ Absyn::ElementSpec::COMPONENTS { .. }, .. } => {
            spec = (*__esc_spec).clone();
            assign_variant_field!(element => Absyn::Element::ELEMENT;
                finalPrefix = properties.isFinal.clone(),
                redeclareKeywords = setReplaceableKeywordAttributes(var_field!((*element).redeclareKeywords, Absyn::Element::ELEMENT).clone(), properties.isReplaceable.clone())?,
                innerOuter = properties.innerOuter.clone()
            );
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; attributes = setElementAttributes(var_field!((*spec).attributes, Absyn::ElementSpec::COMPONENTS).clone(), properties));
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = spec.clone());
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(element)
}

fn setReplaceableKeywordAttributes(
    mut inAbsynRedeclareKeywordsOption: Option<Absyn::RedeclareKeywords>,
    mut inBoolean: bool,
) -> Result<Option<Absyn::RedeclareKeywords>> {
    let mut outAbsynRedeclareKeywordsOption: Option<Absyn::RedeclareKeywords>;
    outAbsynRedeclareKeywordsOption = (match (inAbsynRedeclareKeywordsOption, inBoolean) {
        (None, false) => None,
        (Some(Absyn::RedeclareKeywords::REPLACEABLE { .. }), false) => None,
        (Some(Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE { .. }), false) => {
            Some(openmodelica_ast::Absyn::RedeclareKeywords::REDECLARE)
        }
        (Some(Absyn::RedeclareKeywords::REDECLARE { .. }), false) => {
            Some(openmodelica_ast::Absyn::RedeclareKeywords::REDECLARE)
        }
        (None, true) => Some(openmodelica_ast::Absyn::RedeclareKeywords::REPLACEABLE),
        (Some(Absyn::RedeclareKeywords::REDECLARE { .. }), true) => {
            Some(openmodelica_ast::Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE)
        }
        (Some(Absyn::RedeclareKeywords::REPLACEABLE { .. }), true) => {
            Some(openmodelica_ast::Absyn::RedeclareKeywords::REPLACEABLE)
        }
        (Some(Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE { .. }), true) => {
            Some(openmodelica_ast::Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outAbsynRedeclareKeywordsOption)
}

fn setInnerOuterAttributes(mut isInner: bool, mut isOuter: bool) -> Absyn::InnerOuter {
    let mut outInnerOuter: Absyn::InnerOuter;
    outInnerOuter = (match (isInner, isOuter) {
        (false, false) => openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        (true, false) => openmodelica_ast::Absyn::InnerOuter::INNER,
        (false, true) => openmodelica_ast::Absyn::InnerOuter::OUTER,
        _ => openmodelica_ast::Absyn::InnerOuter::INNER_OUTER,
    });
    outInnerOuter
}

fn setElementVariability(mut inString: &ArcStr) -> Result<Absyn::Variability> {
    let mut outVariability: Absyn::Variability;
    outVariability = (::match_deref::match_deref! { match &(inString.clone()) {
        Deref @ "" => openmodelica_ast::Absyn::Variability::VAR,
        Deref @ "discrete" => openmodelica_ast::Absyn::Variability::DISCRETE,
        Deref @ "parameter" => openmodelica_ast::Absyn::Variability::PARAM,
        Deref @ "constant" => openmodelica_ast::Absyn::Variability::CONST,
        _ => return Err("match: no arm matched"),
    } });
    Ok(outVariability)
}

fn setElementCausality(mut inString: &ArcStr) -> Result<Absyn::Direction> {
    let mut outDirection: Absyn::Direction;
    outDirection = (::match_deref::match_deref! { match &(inString.clone()) {
        Deref @ "" => openmodelica_ast::Absyn::Direction::BIDIR,
        Deref @ "input" => openmodelica_ast::Absyn::Direction::INPUT,
        Deref @ "output" => openmodelica_ast::Absyn::Direction::OUTPUT,
        _ => return Err("match: no arm matched"),
    } });
    Ok(outDirection)
}

fn setElementAttributes(
    mut attributes: Absyn::ElementAttributes,
    mut properties: ComponentProperties,
) -> Absyn::ElementAttributes {
    let mut attributes: Absyn::ElementAttributes = attributes;
    attributes = Absyn::ElementAttributes {
        flowPrefix: properties.isFlow.clone(),
        streamPrefix: properties.isStream.clone(),
        parallelism: attributes.parallelism.clone(),
        variability: properties.variability.clone(),
        direction: properties.direction.clone(),
        isField: attributes.isField.clone(),
        arrayDim: attributes.arrayDim.clone(),
    };
    attributes
}

pub(crate) fn getCrefInfo(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut info: SourceInfo;
    match '__try0: {
        cls =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), program, false, false), '__try0);
        info = cls.info.clone();
        result = ValuesMake::makeArray(list![
            ValuesMake::makeCodeTypeNameStr(unwrap_break_err!(Testsuite::friendly(info.fileName.clone()), '__try0)),
            ValuesMake::makeCodeTypeNameStr(if (info.isReadOnly.clone()) {
                literal!("readonly")
            } else {
                literal!("writable")
            }),
            ValuesMake::makeInteger(info.lineNumberStart.clone()),
            ValuesMake::makeInteger(info.columnNumberStart.clone()),
            ValuesMake::makeInteger(info.lineNumberEnd.clone()),
            ValuesMake::makeInteger(info.columnNumberEnd.clone())
        ]);
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    result
}

fn getImportString(mut inImport: &Absyn::Import) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inImport.clone() {
        Absyn::Import::NAMED_IMPORT {
            name: mut id,
            path: mut path,
        } => {
            let mut path_str: ArcStr;
            let mut r#str: ArcStr;
            path_str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            r#str = stringAppendList(list![
                literal!("kind=named, id="),
                id.clone(),
                literal!(", path="),
                path_str
            ]);
            r#str
        }
        Absyn::Import::QUAL_IMPORT { path: mut path } => {
            let mut path_str: ArcStr;
            let mut r#str: ArcStr;
            path_str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            r#str = stringAppendList(list![literal!("kind=qualified, path="), path_str]);
            r#str
        }
        Absyn::Import::UNQUAL_IMPORT { path: mut path } => {
            let mut path_str: ArcStr;
            let mut r#str: ArcStr;
            path_str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            r#str = stringAppendList(list![literal!("kind=unqualified, path="), path_str]);
            r#str
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn getElementType(
    mut inElementSpec: &metamodelica::Ref<Absyn::ElementSpec>,
    mut inElement: &metamodelica::Ref<Absyn::Element>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inElementSpec {
        Absyn::ElementSpec::EXTENDS { path, .. } => {
            let mut path_str: ArcStr;
            let mut r#str: ArcStr;
            path_str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            r#str = stringAppendList(list![literal!("elementtype=extends, path="), path_str]);
            r#str
        }
        Absyn::ElementSpec::IMPORT { import_, .. } => {
            let mut r#str: ArcStr;
            let mut import_str: ArcStr;
            import_str = getImportString(import_)?;
            r#str = stringAppendList(list![literal!("elementtype=import, "), import_str]);
            r#str
        }
        Absyn::ElementSpec::COMPONENTS {
            attributes: attr,
            typeSpec,
            components: lst,
        } => {
            let mut r#str: ArcStr;
            let mut typename: ArcStr;
            let mut flowPrefixstr: ArcStr;
            let mut streamPrefixstr: ArcStr;
            let mut variability_str: ArcStr;
            let mut dir_str: ArcStr;
            let mut names_str: ArcStr;
            let mut names: metamodelica::List<ArcStr>;
            typename = Dump::unparseTypeSpec(typeSpec.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(InteractiveUtil::getComponentItemsNameAndComment(lst.clone(), inElement)) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            names = metamodelica::Own::own(__pa0);
            flowPrefixstr = InteractiveUtil::attrFlowStr(attr);
            streamPrefixstr = InteractiveUtil::attrStreamStr(attr);
            variability_str = InteractiveUtil::attrVariabilityStr(attr)?;
            dir_str = InteractiveUtil::attrDirectionStr(attr)?;
            names_str = stringDelimitList(names, literal!(", "));
            r#str = stringAppendList(list![
                literal!("elementtype=component, typename="),
                typename,
                literal!(", names={"),
                names_str,
                literal!("}, flow="),
                flowPrefixstr,
                literal!(", stream="),
                streamPrefixstr,
                literal!(", variability=\""),
                variability_str,
                literal!("\", direction=\""),
                dir_str,
                literal!("\"")
            ]);
            r#str
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn getElementInfo(mut inElementItem: &metamodelica::Ref<Absyn::ElementItem>) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = &**inElementItem;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: f, redeclareKeywords: r, innerOuter: inout, specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name: id, restriction: restr, info: SourceInfo { fileName: file, isReadOnly, lineNumberStart: sline, columnNumberStart: scol, lineNumberEnd: eline, columnNumberEnd: ecol, .. }, .. }, .. }, .. } } => {
                    let mut finalPrefix: ArcStr;
                    let mut repl: ArcStr;
                    let mut inout_str: ArcStr;
                    let mut str_restriction: ArcStr;
                    let mut element_str: ArcStr;
                    let mut sline_str: ArcStr;
                    let mut scol_str: ArcStr;
                    let mut eline_str: ArcStr;
                    let mut ecol_str: ArcStr;
                    let mut readonly_str: ArcStr;
                    let mut r#str: ArcStr;
                    let mut r_1: bool;
                    let mut file = (*file).clone();
                    finalPrefix = boolString(f.clone());
                    r_1 = keywordReplaceable(r.clone());
                    repl = boolString(r_1);
                    inout_str = InteractiveUtil::innerOuterStr(inout.clone());
                    str_restriction = AbsynUtil::restrString(metamodelica::AsArg::as_arg(&restr));
                    element_str = stringAppendList(list![literal!("elementtype=classdef, classname="), id.clone(), literal!(", classrestriction="), str_restriction.clone()]);
                    file = Testsuite::friendly(file.clone())?;
                    sline_str = intString(sline.clone());
                    scol_str = intString(scol.clone());
                    eline_str = intString(eline.clone());
                    ecol_str = intString(ecol.clone());
                    readonly_str = if (isReadOnly.clone()) {literal!("readonly")} else {literal!("writable")};
                    r#str = stringAppendList(list![literal!("elementfile=\""), file.clone(), literal!("\", elementreadonly=\""), readonly_str.clone(), literal!("\", elementStartLine="), sline_str.clone(), literal!(", elementStartColumn="), scol_str.clone(), literal!(", elementEndLine="), eline_str.clone(), literal!(", elementEndColumn="), ecol_str.clone(), literal!(", final="), finalPrefix.clone(), literal!(", replaceable="), repl.clone(), literal!(", inout=\""), inout_str.clone(), literal!("\", "), element_str.clone()]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementItem::ELEMENTITEM { element: el @ Deref @ Absyn::Element::ELEMENT { finalPrefix: f, redeclareKeywords: r, innerOuter: inout, specification: elementSpec, info: SourceInfo { fileName: file, isReadOnly, lineNumberStart: sline, columnNumberStart: scol, lineNumberEnd: eline, columnNumberEnd: ecol, .. }, .. } } => {
                    let mut finalPrefix: ArcStr;
                    let mut repl: ArcStr;
                    let mut inout_str: ArcStr;
                    let mut element_str: ArcStr;
                    let mut sline_str: ArcStr;
                    let mut scol_str: ArcStr;
                    let mut eline_str: ArcStr;
                    let mut ecol_str: ArcStr;
                    let mut readonly_str: ArcStr;
                    let mut r#str: ArcStr;
                    let mut r_1: bool;
                    let mut file = (*file).clone();
                    finalPrefix = boolString(f.clone());
                    r_1 = keywordReplaceable(r.clone());
                    repl = boolString(r_1);
                    inout_str = InteractiveUtil::innerOuterStr(inout.clone());
                    element_str = getElementType(metamodelica::AsArg::as_arg(&elementSpec), metamodelica::AsArg::as_arg(&el))?;
                    sline_str = intString(sline.clone());
                    scol_str = intString(scol.clone());
                    eline_str = intString(eline.clone());
                    ecol_str = intString(ecol.clone());
                    readonly_str = if (isReadOnly.clone()) {literal!("readonly")} else {literal!("writable")};
                    file = Testsuite::friendly(file.clone())?;
                    r#str = stringAppendList(list![literal!("elementfile=\""), file.clone(), literal!("\", elementreadonly=\""), readonly_str.clone(), literal!("\", elementStartLine="), sline_str.clone(), literal!(", elementStartColumn="), scol_str.clone(), literal!(", elementEndLine="), eline_str.clone(), literal!(", elementEndColumn="), ecol_str.clone(), literal!(", final="), finalPrefix.clone(), literal!(", replaceable="), repl.clone(), literal!(", inout=\""), inout_str.clone(), literal!("\", "), element_str.clone()]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementItem::LEXER_COMMENT { .. } => {
                    Ok(literal!("elementtype=comment"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("elementtype=annotation"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

fn constructElementsInfo(
    mut visibility: ArcStr,
    mut elements: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> ArcStr {
    let mut result: ArcStr;
    let mut elements_strl: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut element_str: ArcStr;
    for mut e in &**elements {
        element_str = getElementInfo(metamodelica::AsArg::as_arg(&e));
        element_str = stringAppendList(list![
            literal!("{ rec(elementvisibility="),
            visibility.clone(),
            literal!(", "),
            element_str,
            literal!(") }")
        ]);
        elements_strl = metamodelica::cons(element_str, elements_strl);
    }
    elements_strl = Dangerous::listReverseInPlace(elements_strl);
    result = stringDelimitList(elements_strl, literal!(",\n"));
    if !((elements).is_empty()) {
        result = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*result);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
    }
    result
}

fn appendNonEmptyStrings(mut str1: ArcStr, mut str2: ArcStr, mut delim: ArcStr) -> ArcStr {
    let mut outString: ArcStr;
    if stringEmpty(&str1) {
        outString = str2;
    } else if stringEmpty(&str2) {
        outString = str1;
    } else {
        outString = stringAppendList(list![str1, delim, str2]);
    }
    outString
}

pub(crate) fn getElementsInfo(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut result_str: ArcStr;
    let mut public_str: ArcStr;
    let mut protected_str: ArcStr;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    match '__try0: {
        cls =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), program, false, false), '__try0);
        parts = AbsynUtil::getClassPartsInClass(&cls);
        public_str = constructElementsInfo(literal!("public"), &(ProgramUtil::getPublicList(&parts)));
        protected_str = constructElementsInfo(literal!("protected"), &(ProgramUtil::getProtectedList(&parts)));
        result_str = appendNonEmptyStrings(public_str.clone(), protected_str.clone(), literal!(", "));
        result_str = stringAppendList(list![literal!("{ "), result_str.clone(), literal!(" }")]);
        Ok::<_, &'static str>((result_str.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result_str = __try0_o0;
        }
        Err(_) => {
            result_str = literal!("Error");
        }
    }
    result = ValuesMake::makeCodeTypeNameStr(result_str);
    result
}

pub(crate) fn getSourceFile(mut p_class: metamodelica::Ref<Absyn::Path>, mut inProgram: Absyn::Program) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = inProgram;
        if let Ok(__v) = (|| -> Result<_> {
            let mut p = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut cdef: metamodelica::Ref<Absyn::Class>;
            let mut filename: ArcStr;
            cdef = ProgramUtil::getPathedClassInProgram(p_class.clone(), &(p.clone()), false, false)?;
            filename = AbsynUtil::classFilename(&cdef)?;
            Ok(filename.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(literal!(""))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

pub(crate) fn setSourceFile(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut inString: ArcStr,
    mut inProgram: Absyn::Program,
) -> (bool, Absyn::Program) {
    let mut success: bool;
    let mut outProgram: Absyn::Program;
    (success, outProgram) = 'mc: {
        let __mc_input = (inString, inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut filename, mut p @ Absyn::Program { .. }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut cdef: metamodelica::Ref<Absyn::Class>;
            let mut cdef_1: metamodelica::Ref<Absyn::Class>;
            let mut within_: Absyn::Within;
            let mut newp: Absyn::Program;
            cdef = ProgramUtil::getPathedClassInProgram(path.clone(), &(p.clone()), false, false)?;
            within_ = ProgramUtil::buildWithin(path.clone())?;
            cdef_1 = AbsynUtil::setClassFilename(cdef.clone(), filename.clone());
            newp = ProgramUtil::updateProgram(
                Absyn::Program {
                    classes: list![cdef_1.clone()],
                    within_: within_.clone(),
                },
                p.clone(),
                false,
                true,
            )?;
            Ok((true, newp.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((false, inProgram.clone()))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (success, outProgram)
}

pub(crate) fn removeExtendsModifiers(
    mut inClassPath: metamodelica::Ref<Absyn::Path>,
    mut inBaseClassPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
    mut keepRedeclares: bool,
) -> (Absyn::Program, bool) {
    let mut outProgram: Absyn::Program;
    let mut outResult: bool;
    (outProgram, outResult) = 'mc: {
        let __mc_input = (inClassPath, inBaseClassPath, inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p_class, inherit_class, p @ Absyn::Program { .. }) => {
                    let mut within_: Absyn::Within;
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut cdef_1: metamodelica::Ref<Absyn::Class>;
                    let mut env: GraphicEnvCache;
                    let mut newp: Absyn::Program;
                    within_ = ProgramUtil::buildWithin(p_class.clone())?;
                    cdef = ProgramUtil::getPathedClassInProgram(p_class.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    env = getClassEnv(p.clone(), p_class.clone())?;
                    cdef_1 = removeExtendsModifiersInClass(cdef.clone(), inherit_class.clone(), env.clone(), keepRedeclares)?;
                    newp = ProgramUtil::updateProgram(Absyn::Program { classes: list![cdef_1.clone()], within_: within_.clone() }, p.clone(), false, false)?;
                    Ok((newp.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inProgram.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outProgram, outResult)
}

fn removeExtendsModifiersInClass(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inEnv: GraphicEnvCache,
    mut keepRedeclares: bool,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    outClass = (::match_deref::match_deref! { match &(inClass) {
        __esc_outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmt }, .. } => {
            let mut inherit_name = inPath;
            let mut env = inEnv;
            outClass = (*__esc_outClass).clone();
            let mut parts_1: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            parts_1 = removeExtendsModifiersInClassparts(metamodelica::AsArg::as_arg(&parts), inherit_name, env, keepRedeclares)?;
            assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts_1, ann: ann.clone(), comment: cmt.clone() }));
            outClass.clone()
        },
        __esc_outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname, parts, modifications: modif, ann, comment: cmt }, .. } => {
            let mut inherit_name = inPath;
            let mut env = inEnv;
            outClass = (*__esc_outClass).clone();
            let mut parts_1: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            parts_1 = removeExtendsModifiersInClassparts(metamodelica::AsArg::as_arg(&parts), inherit_name, env, keepRedeclares)?;
            assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname.clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts_1, ann: ann.clone() }));
            outClass.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outClass)
}

fn removeExtendsModifiersInClassparts(
    mut inAbsynClassPartLst: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inEnv: GraphicEnvCache,
    mut keepRedeclares: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outAbsynClassPartLst: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outAbsynClassPartLst = 'mc: {
        let __mc_input = (&**inAbsynClassPartLst, inPath, inEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: rest }, inherit, env) => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut elts_1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    res = removeExtendsModifiersInClassparts(metamodelica::AsArg::as_arg(&rest), inherit.clone(), env.clone(), keepRedeclares)?;
                    elts_1 = removeExtendsModifiersInElementitems(metamodelica::AsArg::as_arg(&elts), inherit.clone(), env.clone(), keepRedeclares)?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: elts_1.clone() }), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: rest }, inherit, env) => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut elts_1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    res = removeExtendsModifiersInClassparts(metamodelica::AsArg::as_arg(&rest), inherit.clone(), env.clone(), keepRedeclares)?;
                    elts_1 = removeExtendsModifiersInElementitems(metamodelica::AsArg::as_arg(&elts), inherit.clone(), env.clone(), keepRedeclares)?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: elts_1.clone() }), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: elt, tail: rest }, inherit, env) => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    res = removeExtendsModifiersInClassparts(metamodelica::AsArg::as_arg(&rest), inherit.clone(), env.clone(), keepRedeclares)?;
                    Ok(metamodelica::cons(elt.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAbsynClassPartLst)
}

fn removeExtendsModifiersInElementitems(
    mut inAbsynElementItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inEnv: GraphicEnvCache,
    mut keepRedeclares: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    let mut outAbsynElementItemLst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    outAbsynElementItemLst = 'mc: {
        let __mc_input = (&**inAbsynElementItemLst, inPath, inEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: elt }, tail: rest }, inherit, env) => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut elt_1: metamodelica::Ref<Absyn::Element>;
                    res = removeExtendsModifiersInElementitems(metamodelica::AsArg::as_arg(&rest), inherit.clone(), env.clone(), keepRedeclares)?;
                    elt_1 = removeExtendsModifiersInElement(elt.clone(), inherit.clone(), env.clone(), keepRedeclares);
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: elt_1.clone() }), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: elitem, tail: rest }, inherit, env) => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    res = removeExtendsModifiersInElementitems(metamodelica::AsArg::as_arg(&rest), inherit.clone(), env.clone(), keepRedeclares)?;
                    Ok(metamodelica::cons(elitem.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAbsynElementItemLst)
}

fn removeExtendsModifiersInElement(
    mut inElement: metamodelica::Ref<Absyn::Element>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inEnv: GraphicEnvCache,
    mut keepRedeclares: bool,
) -> metamodelica::Ref<Absyn::Element> {
    let mut outElement: metamodelica::Ref<Absyn::Element>;
    outElement = 'mc: {
        let __mc_input = (&*inElement, inPath, inEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ Absyn::Element::ELEMENT { finalPrefix: f, redeclareKeywords: r, innerOuter: i, specification: Deref @ Absyn::ElementSpec::EXTENDS { path, elementArg: eargs, annotationOpt: annOpt }, info, constrainClass: constr }, inherit, env) => {
                            let mut path_1: metamodelica::Ref<Absyn::Path>;
                            let mut eargs = (*eargs).clone();
                            (_, path_1) = mkFullyQual(env.clone(), path.clone(), false)?;
                            let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&inherit), &path_1)) else { return Err("pattern mismatch") };
                            eargs = if (!(keepRedeclares)) {metamodelica::nil()} else {({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
                for mut e in (eargs.clone()).into_iter().cloned() {
                            if !((match &*e.clone() {
                Absyn::ElementArg::REDECLARATION { .. } => true,
                _ => false,
            })) { continue; }
                            let __x = e.clone();
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })};
                            Ok(metamodelica::Ref::new(Absyn::Element::ELEMENT { finalPrefix: f.clone(), redeclareKeywords: r.clone(), innerOuter: i.clone(), specification: metamodelica::Ref::new(Absyn::ElementSpec::EXTENDS { path: path.clone(), elementArg: eargs.clone(), annotationOpt: annOpt.clone() }), info: info.clone(), constrainClass: constr.clone() }))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inElement.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outElement
}

pub(crate) fn mkFullyQual(
    mut env: GraphicEnvCache,
    mut ipath: metamodelica::Ref<Absyn::Path>,
    mut failOnError: bool,
) -> Result<(FCore::Cache, metamodelica::Ref<Absyn::Path>)> {
    let mut ocache: FCore::Cache;
    let mut opath: metamodelica::Ref<Absyn::Path>;
    let mut cpath: metamodelica::Ref<Absyn::Path>;
    let mut program: Absyn::Program;
    if Flags::isSet(Flags::NF_API.clone())? {
        ocache = cacheFromGraphicEnvCache(env.clone())?;
        (program, cpath) = cacheProgramAndPath(&env);
        opath = NFApi::mkFullyQual(program, cpath, ipath, failOnError)?;
    } else {
        (ocache, opath) = Inst::makeFullyQualified(
            cacheFromGraphicEnvCache(env.clone())?,
            envFromGraphicEnvCache(env)?,
            ipath,
        )?;
    }
    Ok((ocache, opath))
}

pub(crate) fn getExtendsModifierValue(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut extendsPath: &metamodelica::Ref<Absyn::Path>,
    mut modifierPath: &metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut ext_mod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(InteractiveUtil::getPathedExtendsInProgram(classPath.clone(), extendsPath, program.clone())) {
            Some(Deref @ Absyn::ElementSpec::EXTENDS { elementArg: __pa1, .. }) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        ext_mod = metamodelica::Own::own(__pa1);
        result = ValuesMake::makeCodeTypeNameStr(
            unwrap_break_err!(Dump::printExpStr(unwrap_break_err!(getModificationValue(ext_mod.clone(), modifierPath), '__try0)), '__try0),
        );
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeCodeTypeNameStr(literal!(""));
        }
    }
    result
}

pub(crate) fn isExtendsModifierFinal(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut extendsPath: &metamodelica::Ref<Absyn::Path>,
    mut modifierPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut ext_mod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(InteractiveUtil::getPathedExtendsInProgram(classPath.clone(), extendsPath, program.clone())) {
            Some(Deref @ Absyn::ElementSpec::EXTENDS { elementArg: __pa1, .. }) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        ext_mod = metamodelica::Own::own(__pa1);
        result =
            ValuesMake::makeBoolean(unwrap_break_err!(isModifierfinal(ext_mod.clone(), modifierPath.clone()), '__try0));
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    result
}

pub(crate) fn isModifierfinal(
    mut inAbsynElementArgLst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inAbsynElementArgLst, inPath.clone())) {
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: f, path: p1, modification: Some(_), .. }, tail: _ }, p2) if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2))) => {
                return Ok(f.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: name1 }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), .. }, tail: _ }, Deref @ Absyn::Path::QUALIFIED { name: name2, path: p2 }) if (stringEq(&name1, &name2)) => {
                let mut f: bool;
                { (inAbsynElementArgLst, inPath) = (args.clone(), p2.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _) => {
                let mut f: bool;
                { (inAbsynElementArgLst, inPath) = (rest.clone(), inPath); continue '__tco; }
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn makeExtendsFullyQualified(
    mut inElementSpec: &metamodelica::Ref<Absyn::ElementSpec>,
    mut inEnv: GraphicEnvCache,
) -> Result<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut outElementSpec: metamodelica::Ref<Absyn::ElementSpec>;
    outElementSpec = (match &**inElementSpec {
        Absyn::ElementSpec::EXTENDS {
            path,
            elementArg: earg,
            annotationOpt: annOpt,
        } => {
            let mut env = inEnv;
            let mut path_1: metamodelica::Ref<Absyn::Path>;
            (_, path_1) = mkFullyQual(env, path.clone(), false)?;
            metamodelica::Ref::new(Absyn::ElementSpec::EXTENDS {
                path: path_1,
                elementArg: earg.clone(),
                annotationOpt: annOpt.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outElementSpec)
}

pub(crate) fn removeComponentModifiers(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut inComponentName: &ArcStr,
    mut inProgram: Absyn::Program,
    mut keepRedeclares: bool,
) -> (Absyn::Program, bool) {
    let mut outProgram: Absyn::Program;
    let mut outResult: bool;
    let mut within_: Absyn::Within;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    match '__try0: {
        within_ = unwrap_break_err!(ProgramUtil::buildWithin(path.clone()), '__try0);
        cls = unwrap_break_err!(ProgramUtil::getPathedClassInProgram(path.clone(), &inProgram, false, false), '__try0);
        cls = unwrap_break_err!(InteractiveUtil::clearComponentModifiersInClass(cls.clone(), inComponentName, keepRedeclares), '__try0);
        outProgram = unwrap_break_err!(ProgramUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: within_.clone() }, inProgram.clone(), false, false), '__try0);
        outResult = true;
        Ok::<_, &'static str>((outProgram.clone(), outResult.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outProgram = __try0_o0;
            outResult = __try0_o1;
        }
        Err(_) => {
            outProgram = inProgram.clone();
            outResult = false;
        }
    }
    (outProgram, outResult)
}

pub(crate) fn getComponentModifierValue(
    mut classRef: &metamodelica::Ref<Absyn::ComponentRef>,
    mut varRef: &metamodelica::Ref<Absyn::ComponentRef>,
    mut subModRef: &metamodelica::Ref<Absyn::ComponentRef>,
    mut program: &Absyn::Program,
) -> ArcStr {
    let mut valueStr: ArcStr;
    let mut cls_path: metamodelica::Ref<Absyn::Path>;
    let mut name: ArcStr;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    match '__try0: {
        cls_path = unwrap_break_err!(AbsynUtil::crefToPath(classRef), '__try0);
        name = unwrap_break_err!(AbsynUtil::crefIdent(varRef), '__try0);
        cls = unwrap_break_err!(ProgramUtil::getPathedClassInProgram(cls_path.clone(), program, false, false), '__try0);
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(InteractiveUtil::getComponentInClass(&cls, &name), '__try0)) {
            Deref @ Absyn::ComponentItem { component: Absyn::Component { modification: Some(Deref @ Absyn::Modification { elementArgLst: __pa1, .. }), .. }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        args = metamodelica::Own::own(__pa1);
        valueStr = unwrap_break_err!(Dump::printExpStr(unwrap_break_err!(getModificationValue(args.clone(), &(unwrap_break_err!(AbsynUtil::crefToPath(subModRef), '__try0))), '__try0)), '__try0);
        Ok::<_, &'static str>((valueStr.clone(),))
    } {
        Ok((__try0_o0,)) => {
            valueStr = __try0_o0;
        }
        Err(_) => {
            valueStr = literal!("");
        }
    }
    valueStr
}

pub(crate) fn getModificationValue(
    mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut path: &metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut value: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 0 });
    let mut name: ArcStr;
    let mut rest_args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = args;
    let mut arg: metamodelica::Ref<Absyn::ElementArg>;
    let mut found: bool = false;
    while !(found) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        arg = metamodelica::Own::own(__pa0);
        rest_args = metamodelica::Own::own(__pa1);
        found = (::match_deref::match_deref! { match &(arg) {
            Deref @ Absyn::ElementArg::MODIFICATION { modification: __arg_modification, path: __arg_path, .. } if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&__arg_path), path)) => {
                let __pa0 = ::match_deref::match_deref! { match &(__arg_modification.clone()) {
                    Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: __pa0, .. }, .. }) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                value = metamodelica::Own::own(__pa0);
                true
            },
            Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name }, modification: __arg_modification, .. } if (metamodelica::stringEq(&name, &(AbsynUtil::pathFirstIdent(path)))) => {
                let __pa0 = ::match_deref::match_deref! { match &(__arg_modification.clone()) {
                    Some(Deref @ Absyn::Modification { elementArgLst: __pa0, .. }) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                rest_args = metamodelica::Own::own(__pa0);
                value = getModificationValue(rest_args.clone(), &(AbsynUtil::pathRest(path.clone())?))?;
                true
            },
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(value)
}

pub(crate) fn getComponentModifierValues(
    mut inComponentRef1: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
    mut inProgram4: Absyn::Program,
) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = (inComponentRef1, inComponentRef2, inComponentRef3, inProgram4);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_, ident, subident, p) => {
                    let mut p_class: metamodelica::Ref<Absyn::Path>;
                    let mut name: ArcStr;
                    let mut res: ArcStr;
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut comps: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
                    let mut compelts: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>>;
                    let mut compelts_1: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
                    let mut r#mod: metamodelica::Ref<Absyn::Modification>;
                    let mut elementArgLst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    p_class = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&class_))?;
                    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&ident))?) {
                        Deref @ Absyn::Path::IDENT { name: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    name = metamodelica::Own::own(__pa0);
                    cdef = ProgramUtil::getPathedClassInProgram(p_class.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    comps = InteractiveUtil::getComponentsInClass(&cdef, InteractiveUtil::Visibility::ANY.clone());
                    compelts = List::map(comps.clone(), &move |__a0: metamodelica::Ref<Absyn::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(InteractiveUtil::getComponentitemsInElement(&__a0)) })?;
                    compelts_1 = List::flatten(compelts.clone())?;
                    let __pa1 = ::match_deref::match_deref! { match &(List::select1(compelts_1.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::ComponentItem>, __a1: ArcStr| InteractiveUtil::componentitemNamed(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentItem>, ArcStr) -> Result<bool> + 'static>), name.clone())?) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { modification: Some(Deref @ Absyn::Modification { elementArgLst: __pa1, .. }), .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil } => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    elementArgLst = metamodelica::Own::own(__pa1);
                    r#mod = getModificationValues(elementArgLst.clone(), AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&subident))?)?;
                    res = Dump::unparseModificationStr(r#mod.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("Error"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

fn getModificationValues(
    mut inAbsynElementArgLst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Modification>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inAbsynElementArgLst, inPath.clone())) {
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: p1, modification: Some(r#mod), .. }, tail: _ }, p2) if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2))) => {
                return Ok(r#mod.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: name1 }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), .. }, tail: _ }, Deref @ Absyn::Path::QUALIFIED { name: name2, path: p2 }) if (stringEq(&name1, &name2)) => {
                let mut res: metamodelica::Ref<Absyn::Modification>;
                { (inAbsynElementArgLst, inPath) = (args.clone(), p2.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _) => {
                let mut r#mod: metamodelica::Ref<Absyn::Modification>;
                { (inAbsynElementArgLst, inPath) = (rest.clone(), inPath); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn getComponentModifierNames(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut inComponentName: ArcStr,
    mut inProgram3: Absyn::Program,
) -> metamodelica::List<ArcStr> {
    let mut outList: metamodelica::List<ArcStr>;
    outList = 'mc: {
        let __mc_input = inProgram3;
        if let Ok(__v) = (|| -> Result<_> {
            let mut p = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut cdef: metamodelica::Ref<Absyn::Class>;
            let mut comps: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut compelts: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>>;
            let mut compelts_1: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
            let mut r#mod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
            let mut res: metamodelica::List<ArcStr>;
            cdef = ProgramUtil::getPathedClassInProgram(path.clone(), &(p.clone()), false, false)?;
            comps = InteractiveUtil::getComponentsInClass(&cdef, InteractiveUtil::Visibility::ANY.clone());
            compelts = List::map(
                comps.clone(),
                &move |__a0: metamodelica::Ref<Absyn::Element>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(InteractiveUtil::getComponentitemsInElement(&__a0))
                },
            )?;
            compelts_1 = List::flatten(compelts.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(List::select1(compelts_1.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::ComponentItem>, __a1: ArcStr| InteractiveUtil::componentitemNamed(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentItem>, ArcStr) -> Result<bool> + 'static>), inComponentName.clone())?) {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: _, arrayDim: _, modification: Some(Deref @ Absyn::Modification { elementArgLst: __pa0, eqMod: _ }) }, condition: _, comment: _ }, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            r#mod = metamodelica::Own::own(__pa0);
            res = getModificationNames(&r#mod);
            Ok(res.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(metamodelica::nil())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outList
}

fn getModificationNames(
    mut inAbsynElementArgLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> metamodelica::List<ArcStr> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    outStringLst = 'mc: {
        let __mc_input = &**inAbsynElementArgLst;
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
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name }, modification: None, .. }, tail: rest } => {
                    let mut names: metamodelica::List<ArcStr>;
                    names = getModificationNames(metamodelica::AsArg::as_arg(&rest));
                    Ok(metamodelica::cons(name.clone(), names.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: p, modification: Some(Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::ListNode::Nil, eqMod: _ }), .. }, tail: rest } => {
                    let mut names: metamodelica::List<ArcStr>;
                    let mut name: ArcStr;
                    name = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                    names = getModificationNames(metamodelica::AsArg::as_arg(&rest));
                    Ok(metamodelica::cons(name.clone(), names.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: p, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod: Deref @ Absyn::EqMod::EQMOD { .. } }), .. }, tail: rest } => {
                            let mut names: metamodelica::List<ArcStr>;
                            let mut names2: metamodelica::List<ArcStr>;
                            let mut res: metamodelica::List<ArcStr>;
                            let mut name: ArcStr;
                            name = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                            names2 = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut n in (getModificationNames(metamodelica::AsArg::as_arg(&args))).into_iter().cloned() {
                            let __x = stringAppend(stringAppend(name.clone(), literal!(".")), n.clone());
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            names = getModificationNames(metamodelica::AsArg::as_arg(&rest));
                            res = listAppend(names2.clone(), names.clone());
                            Ok(metamodelica::cons(name.clone(), res.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: p, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod: _ }), .. }, tail: rest } => {
                            let mut names: metamodelica::List<ArcStr>;
                            let mut names2: metamodelica::List<ArcStr>;
                            let mut res: metamodelica::List<ArcStr>;
                            let mut name: ArcStr;
                            name = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                            names2 = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut n in (getModificationNames(metamodelica::AsArg::as_arg(&args))).into_iter().cloned() {
                            let __x = stringAppend(stringAppend(name.clone(), literal!(".")), n.clone());
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            names = getModificationNames(metamodelica::AsArg::as_arg(&rest));
                            res = listAppend(names2.clone(), names.clone());
                            Ok(res.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut names: metamodelica::List<ArcStr>;
                    names = getModificationNames(metamodelica::AsArg::as_arg(&rest));
                    Ok(names.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStringLst
}

pub(crate) fn getComponentBinding(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut parameterName: &ArcStr,
    mut program: &Absyn::Program,
) -> ArcStr {
    let mut bindingStr: ArcStr;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut component: metamodelica::Ref<Absyn::ComponentItem>;
    match '__try0: {
        cls = unwrap_break_err!(ProgramUtil::getPathedClassInProgram(path.clone(), program, false, false), '__try0);
        component = unwrap_break_err!(InteractiveUtil::getComponentInClass(&cls, parameterName), '__try0);
        bindingStr = unwrap_break_err!(Dump::printExpStr(unwrap_break_err!(InteractiveUtil::getVariableBindingInComponentitem(&component), '__try0)), '__try0);
        Ok::<_, &'static str>((bindingStr.clone(),))
    } {
        Ok((__try0_o0,)) => {
            bindingStr = __try0_o0;
        }
        Err(_) => {
            bindingStr = literal!("");
        }
    }
    bindingStr
}

fn getComponentitemName(mut inComponentItem: &metamodelica::Ref<Absyn::ComponentItem>) -> Result<ArcStr> {
    let mut outIdent: ArcStr;
    outIdent = (match &**inComponentItem {
        Absyn::ComponentItem {
            component: Absyn::Component { name: id, .. },
            ..
        } => id.clone(),
    });
    Ok(outIdent)
}

pub(crate) fn renameClass(
    mut oldName: metamodelica::Ref<Absyn::Path>,
    mut newName: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> Result<(Absyn::Program, metamodelica::Ref<Values::Value>)> {
    let mut program: Absyn::Program = program;
    let mut result: metamodelica::Ref<Values::Value>;
    let mut env: FCore::Graph;
    let mut new_name: metamodelica::Ref<Absyn::Path>;
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    if AbsynUtil::pathIsQual(&newName) {
        result = ValuesMake::makeBoolean(false);
    }
    if AbsynUtil::pathIsQual(&oldName) {
        new_name = AbsynUtil::joinPaths(AbsynUtil::stripLast(&oldName)?, newName)?;
    } else {
        new_name = newName;
    }
    (_, env) = Inst::makeEnvFromProgram(&(SymbolTable::getSCode()?))?;
    let (__pa0, _, (_, _, _, __pa1, _)) = AbsynUtil::traverseClasses(
        program.clone(),
        None,
        (std::sync::Arc::new(fnptr!(
            renameClassVisitor,
            (
                metamodelica::Ref<Absyn::Class>,
                Option<metamodelica::Ref<Absyn::Path>>,
                (
                    metamodelica::Ref<Absyn::Path>,
                    metamodelica::Ref<Absyn::Path>,
                    Absyn::Program,
                    metamodelica::List<metamodelica::Ref<Absyn::Path>>,
                    FCore::Graph
                )
            )
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        (
                            metamodelica::Ref<Absyn::Class>,
                            Option<metamodelica::Ref<Absyn::Path>>,
                            (
                                metamodelica::Ref<Absyn::Path>,
                                metamodelica::Ref<Absyn::Path>,
                                Absyn::Program,
                                metamodelica::List<metamodelica::Ref<Absyn::Path>>,
                                FCore::Graph,
                            ),
                        ),
                    ) -> Result<(
                        metamodelica::Ref<Absyn::Class>,
                        Option<metamodelica::Ref<Absyn::Path>>,
                        (
                            metamodelica::Ref<Absyn::Path>,
                            metamodelica::Ref<Absyn::Path>,
                            Absyn::Program,
                            metamodelica::List<metamodelica::Ref<Absyn::Path>>,
                            FCore::Graph,
                        ),
                    )> + 'static,
            >),
        (oldName, new_name, program, metamodelica::nil(), env),
        true,
    )?;
    program = metamodelica::Own::own(__pa0);
    paths = metamodelica::Own::own(__pa1);
    result = ValuesMake::makeCodeTypeNameArray(paths);
    Ok((program, result))
}

fn renameClassVisitor(
    mut tup: (
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        (
            metamodelica::Ref<Absyn::Path>,
            metamodelica::Ref<Absyn::Path>,
            Absyn::Program,
            metamodelica::List<metamodelica::Ref<Absyn::Path>>,
            FCore::Graph,
        ),
    ),
) -> (
    metamodelica::Ref<Absyn::Class>,
    Option<metamodelica::Ref<Absyn::Path>>,
    (
        metamodelica::Ref<Absyn::Path>,
        metamodelica::Ref<Absyn::Path>,
        Absyn::Program,
        metamodelica::List<metamodelica::Ref<Absyn::Path>>,
        FCore::Graph,
    ),
) {
    let mut tup: (
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        (
            metamodelica::Ref<Absyn::Path>,
            metamodelica::Ref<Absyn::Path>,
            Absyn::Program,
            metamodelica::List<metamodelica::Ref<Absyn::Path>>,
            FCore::Graph,
        ),
    ) = tup;
    tup = 'mc: {
        let __mc_input = tup.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { info: file_info, .. }, _, _) => {
                    if !((isReadOnly(metamodelica::AsArg::as_arg(&file_info))?)) { return Err("guard") }
                    Ok(tup.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_ @ Deref @ Absyn::Class { name: id, .. }, Some(pa), (old_class_path, new_class_path, p, path_lst, env)) => {
                    let mut path_1: metamodelica::Ref<Absyn::Path>;
                    let mut new_name: ArcStr;
                    let mut class_ = (*class_).clone();
                    path_1 = AbsynUtil::joinPaths(pa.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() }))?;
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&old_class_path), &path_1)) else { return Err("pattern mismatch") };
                    new_name = AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&new_class_path));
                    assign_field!(class_.name = new_name.clone());
                    Ok((class_.clone(), Some(pa.clone()), (old_class_path.clone(), new_class_path.clone(), p.clone(), metamodelica::cons(new_class_path.clone(), path_lst.clone()), env.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_ @ Deref @ Absyn::Class { name: id, .. }, None, (old_class_path, new_class_path, p, path_lst, env)) => {
                    let mut path_1: metamodelica::Ref<Absyn::Path>;
                    let mut new_name: ArcStr;
                    let mut class_ = (*class_).clone();
                    path_1 = metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() });
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&old_class_path), &path_1)) else { return Err("pattern mismatch") };
                    new_name = AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&new_class_path));
                    assign_field!(class_.name = new_name.clone());
                    Ok((class_.clone(), None, (old_class_path.clone(), new_class_path.clone(), p.clone(), metamodelica::cons(new_class_path.clone(), path_lst.clone()), env.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_ @ Deref @ Absyn::Class { name: id, .. }, Some(pa), (old_class_path, new_class_path, p, path_lst, env)) => {
                    let mut path_1: metamodelica::Ref<Absyn::Path>;
                    let mut changed: bool;
                    let mut cenv: FCore::Graph;
                    let mut class_1: metamodelica::Ref<Absyn::Class>;
                    let mut path_lst = (*path_lst).clone();
                    path_1 = AbsynUtil::joinPaths(pa.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() }))?;
                    cenv = getClassEnvNoElaboration(metamodelica::AsArg::as_arg(&p), &path_1, metamodelica::AsArg::as_arg(&env))?;
                    (class_1, changed) = renameClassInClass(class_.clone(), metamodelica::AsArg::as_arg(&old_class_path), new_class_path.clone(), &cenv);
                    if changed {
                        path_lst = metamodelica::cons(path_1.clone(), path_lst.clone());
                    }
                    Ok((class_1.clone(), Some(pa.clone()), (old_class_path.clone(), new_class_path.clone(), p.clone(), path_lst.clone(), env.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_ @ Deref @ Absyn::Class { name: id, .. }, None, (old_class_path, new_class_path, p, path_lst, env)) => {
                    let mut path_1: metamodelica::Ref<Absyn::Path>;
                    let mut changed: bool;
                    let mut cenv: FCore::Graph;
                    let mut class_1: metamodelica::Ref<Absyn::Class>;
                    let mut path_lst = (*path_lst).clone();
                    path_1 = metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() });
                    cenv = getClassEnvNoElaboration(metamodelica::AsArg::as_arg(&p), &path_1, metamodelica::AsArg::as_arg(&env))?;
                    (class_1, changed) = renameClassInClass(class_.clone(), metamodelica::AsArg::as_arg(&old_class_path), new_class_path.clone(), &cenv);
                    if changed {
                        path_lst = metamodelica::cons(path_1.clone(), path_lst.clone());
                    }
                    Ok((class_1.clone(), None, (old_class_path.clone(), new_class_path.clone(), p.clone(), path_lst.clone(), env.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(tup.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    tup
}

fn renameClassInClass(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut oldName: &metamodelica::Ref<Absyn::Path>,
    mut newName: metamodelica::Ref<Absyn::Path>,
    mut env: &FCore::Graph,
) -> (metamodelica::Ref<Absyn::Class>, bool) {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut changed: bool = false;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut name: ArcStr = arcstr::literal!("");
    let mut path: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut cenv: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut body: metamodelica::Ref<Absyn::ClassDef>;
    let mut ty: metamodelica::Ref<Absyn::TypeSpec>;
    body = cls.body.clone();
    changed = 'mc: {
        let __mc_input = body.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. } => {
                    let mut parts = (*parts).clone();
                    let mut body: metamodelica::Ref<Absyn::ClassDef> = body.clone();
                    let mut changed: bool = changed.clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    (parts, changed) = renameClassInParts(metamodelica::AsArg::as_arg(&parts), oldName, newName.clone(), env);
                    assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = parts.clone());
                    assign_field!(cls.body = body.clone());
                    Ok((changed, body.clone(), changed.clone(), cls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            body = __wb0;
            changed = __wb1;
            cls = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. } => {
                    let mut parts = (*parts).clone();
                    let mut body: metamodelica::Ref<Absyn::ClassDef> = body.clone();
                    let mut changed: bool = changed.clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    (parts, changed) = renameClassInParts(metamodelica::AsArg::as_arg(&parts), oldName, newName.clone(), env);
                    assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = parts.clone());
                    assign_field!(cls.body = body.clone());
                    Ok((changed, body.clone(), changed.clone(), cls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            body = __wb0;
            changed = __wb1;
            cls = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::DERIVED { typeSpec: ty @ Deref @ Absyn::TypeSpec::TPATH { .. }, .. } => {
                    let mut ty = (*ty).clone();
                    let mut body: metamodelica::Ref<Absyn::ClassDef> = body.clone();
                    let mut cache: FCore::Cache = cache.clone();
                    let mut cenv: FCore::Graph = cenv.clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    let mut name: ArcStr = name.clone();
                    let mut path: metamodelica::Ref<Absyn::Path> = path.clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(&(FCore::emptyCache()), env, var_field!((*ty).path, Absyn::TypeSpec::TPATH), None)?) {
                        (__pa0, Deref @ SCode::Element::CLASS { name: __pa1, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    name = metamodelica::Own::own(__pa1);
                    cenv = metamodelica::Own::own(__pa2);
                    path = metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() });
                    (_, path) = Inst::makeFullyQualified(cache.clone(), cenv.clone(), path.clone())?;
                    let true = (AbsynUtil::pathEqual(&path, oldName)) else { return Err("pattern mismatch") };
                    assign_variant_field!(ty => Absyn::TypeSpec::TPATH; path = changeLastIdent(path.clone(), newName.clone())?);
                    assign_variant_field!(body => Absyn::ClassDef::DERIVED; typeSpec = ty.clone());
                    assign_field!(cls.body = body.clone());
                    Ok((true, body.clone(), cache.clone(), cenv.clone(), cls.clone(), name.clone(), path.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            body = __wb0;
            cache = __wb1;
            cenv = __wb2;
            cls = __wb3;
            name = __wb4;
            path = __wb5;
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
    (cls, changed)
}

fn renameClassInParts(
    mut parts: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut oldName: &metamodelica::Ref<Absyn::Path>,
    mut newName: metamodelica::Ref<Absyn::Path>,
    mut env: &FCore::Graph,
) -> (metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>, bool) {
    let mut outParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
    let mut changed: bool = false;
    let mut c: bool;
    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    for mut part in &**parts {
        let mut part = part.clone();
        part = (match &*part {
            Absyn::ClassPart::PUBLIC {
                contents: __part_contents,
            } => {
                (elems, c) = renameClassInElements(
                    metamodelica::AsArg::as_arg(&__part_contents),
                    oldName,
                    newName.clone(),
                    env,
                );
                assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = elems);
                changed = changed || c;
                part
            }
            Absyn::ClassPart::PROTECTED {
                contents: __part_contents,
            } => {
                (elems, c) = renameClassInElements(
                    metamodelica::AsArg::as_arg(&__part_contents),
                    oldName,
                    newName.clone(),
                    env,
                );
                assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = elems);
                changed = changed || c;
                part
            }
            _ => part,
        });
        outParts = metamodelica::cons(part, outParts);
    }
    outParts = Dangerous::listReverseInPlace(outParts);
    (outParts, changed)
}

fn renameClassInElements(
    mut items: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut oldName: &metamodelica::Ref<Absyn::Path>,
    mut newName: metamodelica::Ref<Absyn::Path>,
    mut env: &FCore::Graph,
) -> (metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>, bool) {
    let mut outItems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
    let mut changed: bool = false;
    let mut elem: metamodelica::Ref<Absyn::Element>;
    let mut spec: metamodelica::Ref<Absyn::ElementSpec>;
    let mut c: bool;
    for mut item in &**items {
        let mut item = item.clone();
        (outItems, changed) = (::match_deref::match_deref! { match &(item.clone()) {
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: __esc_elem @ Deref @ Absyn::Element::ELEMENT { .. } } => {
                elem = (*__esc_elem).clone();
                (spec, c) = renameClassInElementSpec(var_field!((*elem).specification, Absyn::Element::ELEMENT).clone(), oldName, newName.clone(), env);
                assign_variant_field!(elem => Absyn::Element::ELEMENT; specification = spec);
                assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = elem.clone());
                (metamodelica::cons(item, outItems), changed || c)
            },
            _ => (metamodelica::cons(item, outItems), changed),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    outItems = Dangerous::listReverseInPlace(outItems);
    (outItems, changed)
}

fn renameClassInElementSpec(
    mut spec: metamodelica::Ref<Absyn::ElementSpec>,
    mut oldName: &metamodelica::Ref<Absyn::Path>,
    mut newName: metamodelica::Ref<Absyn::Path>,
    mut env: &FCore::Graph,
) -> (metamodelica::Ref<Absyn::ElementSpec>, bool) {
    let mut spec: metamodelica::Ref<Absyn::ElementSpec> = spec;
    let mut changed: bool = false;
    let mut ty: metamodelica::Ref<Absyn::TypeSpec>;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut id: ArcStr = arcstr::literal!("");
    let mut cenv: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut path: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut qpath: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    changed = 'mc: {
        let __mc_input = spec.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementSpec::COMPONENTS { typeSpec: ty @ Deref @ Absyn::TypeSpec::TPATH { .. }, .. } => {
                    let mut ty = (*ty).clone();
                    let mut cache: FCore::Cache = cache.clone();
                    let mut cenv: FCore::Graph = cenv.clone();
                    let mut changed: bool = changed.clone();
                    let mut id: ArcStr = id.clone();
                    let mut qpath: metamodelica::Ref<Absyn::Path> = qpath.clone();
                    let mut spec: metamodelica::Ref<Absyn::ElementSpec> = spec.clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(&(FCore::emptyCache()), env, var_field!((*ty).path, Absyn::TypeSpec::TPATH), None)?) {
                        (__pa0, Deref @ SCode::Element::CLASS { name: __pa1, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    id = metamodelica::Own::own(__pa1);
                    cenv = metamodelica::Own::own(__pa2);
                    (_, qpath) = Inst::makeFullyQualified(cache.clone(), cenv.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() }))?;
                    if AbsynUtil::pathEqual(&qpath, oldName) {
                        assign_variant_field!(ty => Absyn::TypeSpec::TPATH; path = changeLastIdent(qpath.clone(), newName.clone())?);
                        assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; typeSpec = ty.clone());
                        changed = true;
                    }
                    Ok((changed, cache.clone(), cenv.clone(), changed.clone(), id.clone(), qpath.clone(), spec.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            cenv = __wb1;
            changed = __wb2;
            id = __wb3;
            qpath = __wb4;
            spec = __wb5;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementSpec::EXTENDS { .. } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut cenv: FCore::Graph = cenv.clone();
                    let mut changed: bool = changed.clone();
                    let mut qpath: metamodelica::Ref<Absyn::Path> = qpath.clone();
                    let mut spec: metamodelica::Ref<Absyn::ElementSpec> = spec.clone();
                    (cache, _, cenv) = Lookup::lookupClass(&(FCore::emptyCache()), env, var_field!((*spec).path, Absyn::ElementSpec::EXTENDS), None)?;
                    (_, qpath) = Inst::makeFullyQualified(cache.clone(), cenv.clone(), var_field!((*spec).path, Absyn::ElementSpec::EXTENDS).clone())?;
                    if AbsynUtil::pathEqual(&qpath, oldName) {
                        assign_variant_field!(spec => Absyn::ElementSpec::EXTENDS; path = changeLastIdent(var_field!((*spec).path, Absyn::ElementSpec::EXTENDS).clone(), newName.clone())?);
                        changed = true;
                    }
                    Ok((changed, cache.clone(), cenv.clone(), changed.clone(), qpath.clone(), spec.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            cenv = __wb1;
            changed = __wb2;
            qpath = __wb3;
            spec = __wb4;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementSpec::IMPORT { .. } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut cenv: FCore::Graph = cenv.clone();
                    let mut changed: bool = changed.clone();
                    let mut path: metamodelica::Ref<Absyn::Path> = path.clone();
                    let mut qpath: metamodelica::Ref<Absyn::Path> = qpath.clone();
                    let mut spec: metamodelica::Ref<Absyn::ElementSpec> = spec.clone();
                    path = AbsynUtil::importPath(var_field!((*spec).import_, Absyn::ElementSpec::IMPORT));
                    (cache, _, cenv) = Lookup::lookupClass(&(FCore::emptyCache()), env, &path, None)?;
                    (_, qpath) = Inst::makeFullyQualified(cache.clone(), cenv.clone(), path.clone())?;
                    if AbsynUtil::pathEqual(&qpath, oldName) {
                        path = changeLastIdent(path.clone(), newName.clone())?;
                        assign_variant_field!(spec => Absyn::ElementSpec::IMPORT; import_ = AbsynUtil::setImportPath(var_field!((*spec).import_, Absyn::ElementSpec::IMPORT).clone(), path.clone()));
                        changed = true;
                    }
                    Ok((changed, cache.clone(), cenv.clone(), changed.clone(), path.clone(), qpath.clone(), spec.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            cenv = __wb1;
            changed = __wb2;
            path = __wb3;
            qpath = __wb4;
            spec = __wb5;
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
    (spec, changed)
}

pub(crate) fn refactorClass(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    fn r#impl(
        mut classPath: metamodelica::Ref<Absyn::Path>,
        mut program: Absyn::Program,
        mut accessLevel: Access,
    ) -> Result<metamodelica::Ref<Values::Value>> {
        let mut result: metamodelica::Ref<Values::Value>;
        let mut cls: metamodelica::Ref<Absyn::Class>;
        let mut p: Absyn::Program;
        let mut r#str: ArcStr;
        cls = ProgramUtil::getPathedClassInProgram(classPath, &program, false, false)?;
        cls = Refactor::refactorGraphicalAnnotation(program.clone(), cls)?;
        p = ProgramUtil::updateProgram(
            Absyn::Program {
                classes: list![cls.clone()],
                within_: openmodelica_ast::Absyn::Within::TOP,
            },
            program,
            false,
            false,
        )?;
        SymbolTable::setAbsyn(p)?;
        r#str = Dump::unparseStr(
            Absyn::Program {
                classes: list![cls],
                within_: openmodelica_ast::Absyn::Within::TOP,
            },
            false,
            Dump::defaultDumpOptions.clone(),
        )?;
        result = ValuesMake::makeString(r#str);
        Ok(result)
    }

    let mut result: metamodelica::Ref<Values::Value>;
    result = InteractiveUtil::accessClass(classPath, program, &r#impl, true, true, Access::icon.clone())?;
    Ok(result)
}

fn changeLastIdent(
    mut inPath1: metamodelica::Ref<Absyn::Path>,
    mut inPath2: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((inPath1, inPath2)) {
        (Deref @ Absyn::Path::IDENT { .. }, Deref @ Absyn::Path::IDENT { name: b }) => {
            metamodelica::Ref::new(Absyn::Path::IDENT { name: b.clone() })
        },
        (Deref @ Absyn::Path::IDENT { .. }, p2 @ Deref @ Absyn::Path::QUALIFIED { .. }) => {
            let mut b_1: ArcStr;
            b_1 = AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&p2));
            metamodelica::Ref::new(Absyn::Path::IDENT { name: b_1 })
        },
        (p1 @ Deref @ Absyn::Path::QUALIFIED { .. }, p2 @ Deref @ Absyn::Path::IDENT { .. }) => {
            let mut a_1: metamodelica::Ref<Absyn::Path>;
            let mut res: metamodelica::Ref<Absyn::Path>;
            a_1 = AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&p1))?;
            res = AbsynUtil::joinPaths(a_1, p2.clone())?;
            res
        },
        (p1 @ Deref @ Absyn::Path::QUALIFIED { .. }, p2 @ Deref @ Absyn::Path::QUALIFIED { .. }) => {
            let mut b_1: ArcStr;
            let mut a_1: metamodelica::Ref<Absyn::Path>;
            let mut res: metamodelica::Ref<Absyn::Path>;
            a_1 = AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&p1))?;
            b_1 = AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&p2));
            res = AbsynUtil::joinPaths(a_1, metamodelica::Ref::new(Absyn::Path::IDENT { name: b_1 }))?;
            res
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outPath)
}

pub(crate) fn isPrimitive(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(className.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "Real" } => {
            true
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "Integer" } => {
            true
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "String" } => {
            true
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "Boolean" } => {
            true
        },
        _ => {
            let mut class_: metamodelica::Ref<Absyn::Class>;
            class_ = ProgramUtil::getPathedClassInProgram(className, &inProgram, false, false)?;
            isPrimitiveClass(class_, inProgram)?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub(crate) fn createModel(
    mut className: &metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    let mut name: ArcStr;
    let mut w: Absyn::Within;
    let mut wp: metamodelica::Ref<Absyn::Path>;
    if AbsynUtil::pathIsIdent(className) {
        name = AbsynUtil::pathFirstIdent(className);
        w = openmodelica_ast::Absyn::Within::TOP;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AbsynUtil::splitQualAndIdentPath(className)?) {
            (__pa0, Deref @ Absyn::Path::IDENT { name: __pa1 }) => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        wp = metamodelica::Own::own(__pa0);
        name = metamodelica::Own::own(__pa1);
        w = Absyn::Within::WITHIN { path: wp };
    }
    outProgram = ProgramUtil::updateProgram(
        Absyn::Program {
            classes: list![metamodelica::Ref::new(Absyn::Class {
                name: name,
                partialPrefix: false,
                finalPrefix: false,
                encapsulatedPrefix: false,
                restriction: openmodelica_ast::Absyn::Restriction::R_MODEL,
                body: Absyn::dummyParts.clone(),
                commentsBeforeClass: metamodelica::nil(),
                commentsBeforeEnd: metamodelica::nil(),
                commentsAfterEnd: metamodelica::nil(),
                info: Absyn::dummyInfo.clone()
            })],
            within_: w,
        },
        inProgram,
        false,
        false,
    )?;
    Ok(outProgram)
}

pub(crate) fn newModel(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut withinPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> Result<Absyn::Program> {
    let mut program: Absyn::Program = program;
    program = createModel(&(AbsynUtil::joinPaths(withinPath, className)?), program)?;
    Ok(program)
}

pub(crate) fn deleteClass(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
) -> (bool, Absyn::Program) {
    let mut success: bool;
    let mut outProgram: Absyn::Program = inProgram.clone();
    (success, outProgram) = 'mc: {
        let __mc_input = inProgram.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut parentcpath: metamodelica::Ref<Absyn::Path>;
            let mut parentparentcpath: metamodelica::Ref<Absyn::Path>;
            let mut cdef: metamodelica::Ref<Absyn::Class>;
            let mut parentcdef: metamodelica::Ref<Absyn::Class>;
            let mut parentcdef_1: metamodelica::Ref<Absyn::Class>;
            let mut outProgram: Absyn::Program = outProgram.clone();
            parentcpath = AbsynUtil::stripLast(&classPath)?;
            parentparentcpath = AbsynUtil::stripLast(&parentcpath)?;
            cdef = ProgramUtil::getPathedClassInProgram(classPath.clone(), &inProgram, false, false)?;
            parentcdef = ProgramUtil::getPathedClassInProgram(parentcpath.clone(), &inProgram, false, false)?;
            parentcdef_1 = InteractiveUtil::removeInnerClass(cdef.clone(), parentcdef.clone())?;
            outProgram = ProgramUtil::updateProgram(
                Absyn::Program {
                    classes: list![parentcdef_1.clone()],
                    within_: Absyn::Within::WITHIN {
                        path: parentparentcpath.clone(),
                    },
                },
                inProgram.clone(),
                false,
                false,
            )?;
            Ok(((true, outProgram.clone()), outProgram.clone()))
        })() {
            outProgram = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut parentcpath: metamodelica::Ref<Absyn::Path>;
            let mut cdef: metamodelica::Ref<Absyn::Class>;
            let mut parentcdef: metamodelica::Ref<Absyn::Class>;
            let mut parentcdef_1: metamodelica::Ref<Absyn::Class>;
            let mut outProgram: Absyn::Program = outProgram.clone();
            parentcpath = AbsynUtil::stripLast(&classPath)?;
            cdef = ProgramUtil::getPathedClassInProgram(classPath.clone(), &inProgram, false, false)?;
            parentcdef = ProgramUtil::getPathedClassInProgram(parentcpath.clone(), &inProgram, false, false)?;
            parentcdef_1 = InteractiveUtil::removeInnerClass(cdef.clone(), parentcdef.clone())?;
            outProgram = ProgramUtil::updateProgram(
                Absyn::Program {
                    classes: list![parentcdef_1.clone()],
                    within_: openmodelica_ast::Absyn::Within::TOP,
                },
                inProgram.clone(),
                false,
                false,
            )?;
            Ok(((true, outProgram.clone()), outProgram.clone()))
        })() {
            outProgram = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut cdef: metamodelica::Ref<Absyn::Class>;
            let mut outProgram: Absyn::Program = outProgram.clone();
            cdef = ProgramUtil::getPathedClassInProgram(classPath.clone(), &inProgram, false, false)?;
            outProgram.classes = List::deleteMemberOnTrue(
                AbsynUtil::className(&cdef),
                outProgram.classes.clone(),
                &move |__a0: ArcStr, __a1: metamodelica::Ref<Absyn::Class>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(AbsynUtil::isClassNamed(&__a0, &__a1))
                },
            )?
            .0;
            Ok(((true, outProgram.clone()), outProgram.clone()))
        })() {
            outProgram = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((false, inProgram.clone()))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (success, outProgram)
}

pub(crate) fn setClassComment(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut inString: ArcStr,
    mut inProgram: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut outProgram: Absyn::Program;
    let mut success: bool;
    (outProgram, success) = 'mc: {
        let __mc_input = (path, inString, inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p_class, r#str, p @ Absyn::Program { .. }) => {
                    let mut within_: Absyn::Within;
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut cdef_1: metamodelica::Ref<Absyn::Class>;
                    let mut newp: Absyn::Program;
                    within_ = ProgramUtil::buildWithin(p_class.clone())?;
                    cdef = ProgramUtil::getPathedClassInProgram(p_class.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    cdef_1 = setClassCommentInClass(cdef.clone(), r#str.clone())?;
                    newp = ProgramUtil::updateProgram(Absyn::Program { classes: list![cdef_1.clone()], within_: within_.clone() }, p.clone(), false, false)?;
                    Ok((newp.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inProgram.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outProgram, success)
}

fn setClassCommentInClass(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut commentString: ArcStr,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    assign_field!(cls.body = setClassCommentInClassdef(cls.body.clone(), commentString)?);
    Ok(cls)
}

fn setClassCommentInClassdef(
    mut classDef: metamodelica::Ref<Absyn::ClassDef>,
    mut commentString: ArcStr,
) -> Result<metamodelica::Ref<Absyn::ClassDef>> {
    let mut classDef: metamodelica::Ref<Absyn::ClassDef> = classDef;
    let mut cmt_str: Option<ArcStr>;
    cmt_str = if (stringEmpty(&commentString)) {
        None
    } else {
        Some(commentString)
    };
    let () = (match &*classDef {
        Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(classDef => Absyn::ClassDef::PARTS; comment = cmt_str);
            ()
        }
        Absyn::ClassDef::DERIVED {
            comment: __classDef_comment,
            ..
        } => {
            assign_variant_field!(classDef => Absyn::ClassDef::DERIVED; comment = AbsynUtil::setCommentString(__classDef_comment.clone(), cmt_str)?);
            ()
        }
        Absyn::ClassDef::ENUMERATION {
            comment: __classDef_comment,
            ..
        } => {
            assign_variant_field!(classDef => Absyn::ClassDef::ENUMERATION; comment = AbsynUtil::setCommentString(__classDef_comment.clone(), cmt_str)?);
            ()
        }
        Absyn::ClassDef::OVERLOAD {
            comment: __classDef_comment,
            ..
        } => {
            assign_variant_field!(classDef => Absyn::ClassDef::OVERLOAD; comment = AbsynUtil::setCommentString(__classDef_comment.clone(), cmt_str)?);
            ()
        }
        Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(classDef => Absyn::ClassDef::CLASS_EXTENDS; comment = cmt_str);
            ()
        }
        Absyn::ClassDef::PDER {
            comment: __classDef_comment,
            ..
        } => {
            assign_variant_field!(classDef => Absyn::ClassDef::PDER; comment = AbsynUtil::setCommentString(__classDef_comment.clone(), cmt_str)?);
            ()
        }
        _ => (),
    });
    Ok(classDef)
}

pub(crate) fn getShortDefinitionBaseClassInformation(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut ty: metamodelica::Ref<Absyn::TypeSpec>;
    let mut attr: Absyn::ElementAttributes;
    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), program, false, false), '__try0)) {
            Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: __pa1, attributes: __pa2 @ Absyn::ElementAttributes { .. }, .. }, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        ty = metamodelica::Own::own(__pa1);
        attr = metamodelica::Own::own(__pa2);
        vals = metamodelica::cons(
            ValuesMake::makeArray(
                unwrap_break_err!(InteractiveUtil::dimensionListValues(AbsynUtil::typeSpecDimensions(&ty)), '__try0),
            ),
            vals.clone(),
        );
        vals = metamodelica::cons(
            ValuesMake::makeString(unwrap_break_err!(InteractiveUtil::attrDirectionStr(&attr), '__try0)),
            vals.clone(),
        );
        vals = metamodelica::cons(
            ValuesMake::makeString(unwrap_break_err!(InteractiveUtil::attrVariabilityStr(&attr), '__try0)),
            vals.clone(),
        );
        vals = metamodelica::cons(
            ValuesMake::makeString(if (attr.streamPrefix.clone()) {
                literal!("stream")
            } else {
                literal!("")
            }),
            vals.clone(),
        );
        vals = metamodelica::cons(
            ValuesMake::makeString(if (attr.flowPrefix.clone()) {
                literal!("flow")
            } else {
                literal!("")
            }),
            vals.clone(),
        );
        vals = metamodelica::cons(ValuesMake::makeCodeTypeName(AbsynUtil::typeSpecPath(&ty)), vals.clone());
        Ok::<_, &'static str>((vals.clone(),))
    } {
        Ok((__try0_o0,)) => {
            vals = __try0_o0;
        }
        Err(_) => {
            vals = metamodelica::nil();
        }
    }
    result = ValuesMake::makeArray(vals);
    result
}

pub(crate) fn getExternalFunctionSpecification(
    mut functionName: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut ext_decl: metamodelica::Ref<Absyn::ExternalDecl>;
    let mut ann: Option<metamodelica::Ref<Absyn::Annotation>>;
    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
    match '__try0: {
        cls = unwrap_break_err!(ProgramUtil::getPathedClassInProgram(functionName.clone(), program, false, false), '__try0);
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(AbsynUtil::getExternalDecl(&cls), '__try0)) {
            Deref @ Absyn::ClassPart::EXTERNAL { externalDecl: __pa1, annotation_: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        ext_decl = metamodelica::Own::own(__pa1);
        ann = metamodelica::Own::own(__pa2);
        vals = metamodelica::cons(
            ValuesMake::makeString(unwrap_break_err!(Dump::unparseAnnotationOption(ann.clone()), '__try0)),
            vals.clone(),
        );
        vals = metamodelica::cons(
            ValuesMake::makeString(
                unwrap_break_err!(Dump::unparseAnnotationOption(ext_decl.annotation_.clone()), '__try0),
            ),
            vals.clone(),
        );
        vals = metamodelica::cons(
            ValuesMake::makeString(unwrap_break_err!(Dump::printExpLstStr(ext_decl.args.clone()), '__try0)),
            vals.clone(),
        );
        vals = metamodelica::cons(
            ValuesMake::makeString(Util::getOptionOrDefault(ext_decl.funcName.clone(), literal!(""))),
            vals.clone(),
        );
        vals = metamodelica::cons(
            ValuesMake::makeString(
                unwrap_break_err!(Util::applyOptionOrDefault(ext_decl.output_.clone(), &move |__a0: metamodelica::Ref<Absyn::ComponentRef>| Dump::printComponentRefStr(&__a0), literal!("")), '__try0),
            ),
            vals.clone(),
        );
        vals = metamodelica::cons(
            ValuesMake::makeString(Util::getOptionOrDefault(ext_decl.lang.clone(), literal!(""))),
            vals.clone(),
        );
        Ok::<_, &'static str>((vals.clone(),))
    } {
        Ok((__try0_o0,)) => {
            vals = __try0_o0;
        }
        Err(_) => {
            vals = metamodelica::nil();
        }
    }
    result = ValuesMake::makeArray(vals);
    result
}

fn getClassDimensions(mut cdef: &metamodelica::Ref<Absyn::ClassDef>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut ad: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    r#str = (::match_deref::match_deref! { match cdef {
        Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: Some(__esc_ad), .. }, .. } => {
            ad = (*__esc_ad).clone();
            List::toString(ad.clone(), &move |__a0: metamodelica::Ref<Absyn::Subscript>| Dump::printSubscriptStr(&__a0), List::Style::FLAT_CURLY.clone())?
        },
        _ => literal!("{}"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#str)
}

pub(crate) fn getClassRestriction(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> ArcStr {
    let mut outRestriction: ArcStr;
    let mut restr: Absyn::Restriction;
    match '__try0: {
        let __arc2 =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(path.clone(), program, false, false), '__try0);
        let Absyn::CLASS { restriction: __pa1, .. } = &*__arc2;
        restr = metamodelica::Own::own(__pa1);
        outRestriction = unwrap_break_err!(Dump::unparseRestrictionStr(restr.clone()), '__try0);
        Ok::<_, &'static str>((outRestriction.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outRestriction = __try0_o0;
        }
        Err(_) => {
            outRestriction = literal!("");
        }
    }
    outRestriction
}

pub(crate) fn isType(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    res = (match InteractiveUtil::getPathedClassRestriction(path, program) {
        Absyn::Restriction::R_TYPE { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isConnector(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    res = (match InteractiveUtil::getPathedClassRestriction(path, program) {
        Absyn::Restriction::R_CONNECTOR { .. } => true,
        Absyn::Restriction::R_EXP_CONNECTOR { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isModel(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    res = (match InteractiveUtil::getPathedClassRestriction(path, program) {
        Absyn::Restriction::R_MODEL { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isOperator(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    res = (match InteractiveUtil::getPathedClassRestriction(path, program) {
        Absyn::Restriction::R_OPERATOR { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isOperatorRecord(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    res = (match InteractiveUtil::getPathedClassRestriction(path, program) {
        Absyn::Restriction::R_OPERATOR_RECORD { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isOperatorFunction(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    res = (match InteractiveUtil::getPathedClassRestriction(path, program) {
        Absyn::Restriction::R_FUNCTION {
            functionRestriction: Absyn::FunctionRestriction::FR_OPERATOR_FUNCTION { .. },
        } => true,
        _ => false,
    });
    res
}

pub(crate) fn isRecord(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    res = (match InteractiveUtil::getPathedClassRestriction(path, program) {
        Absyn::Restriction::R_RECORD { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isBlock(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    res = (match InteractiveUtil::getPathedClassRestriction(path, program) {
        Absyn::Restriction::R_BLOCK { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isOptimization(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    res = (match InteractiveUtil::getPathedClassRestriction(path, program) {
        Absyn::Restriction::R_OPTIMIZATION { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isFunction(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    res = (match InteractiveUtil::getPathedClassRestriction(path, program) {
        Absyn::Restriction::R_FUNCTION {
            functionRestriction: Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { .. },
        } => true,
        _ => false,
    });
    res
}

pub(crate) fn isPackage(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    res = (match InteractiveUtil::getPathedClassRestriction(path, program) {
        Absyn::Restriction::R_PACKAGE { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isClass(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    res = (match InteractiveUtil::getPathedClassRestriction(path, program) {
        Absyn::Restriction::R_CLASS { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isPartial(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    match '__try0: {
        ::match_deref::match_deref! { match &(unwrap_break_err!(ProgramUtil::getPathedClassInProgram(path.clone(), program, false, false), '__try0)) {
            Deref @ Absyn::Class { partialPrefix: true, .. } => (),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        res = true;
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub(crate) fn isReplaceable(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    match '__try0: {
        res = AbsynUtil::isElementReplaceable(
            &(unwrap_break_err!(InteractiveUtil::getPathedElementInProgram(path.clone(), program), '__try0)),
        );
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub(crate) fn isRedeclare(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    match '__try0: {
        res = AbsynUtil::isElementRedeclare(
            &(unwrap_break_err!(InteractiveUtil::getPathedElementInProgram(path.clone(), program), '__try0)),
        );
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub(crate) fn isParameter(
    mut componentName: metamodelica::Ref<Absyn::Path>,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> bool {
    let mut res: bool;
    let mut path: metamodelica::Ref<Absyn::Path>;
    match '__try0: {
        path = unwrap_break_err!(AbsynUtil::joinPaths(className.clone(), componentName.clone()), '__try0);
        ::match_deref::match_deref! { match &(unwrap_break_err!(InteractiveUtil::getPathedElementInProgram(path.clone(), program), '__try0)) {
            Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { variability: Absyn::Variability::PARAM { .. }, .. }, .. }, .. } => (),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        res = true;
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub(crate) fn isConstant(
    mut componentName: metamodelica::Ref<Absyn::Path>,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> bool {
    let mut res: bool;
    let mut path: metamodelica::Ref<Absyn::Path>;
    match '__try0: {
        path = unwrap_break_err!(AbsynUtil::joinPaths(className.clone(), componentName.clone()), '__try0);
        ::match_deref::match_deref! { match &(unwrap_break_err!(InteractiveUtil::getPathedElementInProgram(path.clone(), program), '__try0)) {
            Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { variability: Absyn::Variability::CONST { .. }, .. }, .. }, .. } => (),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        res = true;
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub(crate) fn isProtected(
    mut componentName: &metamodelica::Ref<Absyn::Path>,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> bool {
    let mut res: bool;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut items: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    match '__try0: {
        parts = AbsynUtil::getClassPartsInClass(
            &(unwrap_break_err!(ProgramUtil::getPathedClassInProgram(className.clone(), program, false, false), '__try0)),
        );
        items = ProgramUtil::getProtectedList(&parts);
        unwrap_break_err!(getComponentsContainsName(AbsynUtil::pathToCref(componentName), &items), '__try0);
        res = true;
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub(crate) fn isEnumeration(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    match '__try0: {
        ::match_deref::match_deref! { match &(unwrap_break_err!(ProgramUtil::getPathedClassInProgram(path.clone(), program, false, false), '__try0)) {
            Deref @ Absyn::Class { restriction: Absyn::Restriction::R_TYPE { .. }, body: Deref @ Absyn::ClassDef::ENUMERATION { .. }, .. } => (),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        res = true;
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub(crate) fn isProtectedClass(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut className: &ArcStr,
    mut program: &Absyn::Program,
) -> bool {
    let mut res: bool;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut items: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    match '__try0: {
        parts = AbsynUtil::getClassPartsInClass(
            &(unwrap_break_err!(ProgramUtil::getPathedClassInProgram(path.clone(), program, false, false), '__try0)),
        );
        items = ProgramUtil::getProtectedList(&parts);
        res = isProtectedClassInElements(&items, className);
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

fn isProtectedClassInElements(
    mut items: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut className: &ArcStr,
) -> bool {
    let mut res: bool = false;
    let mut name: ArcStr;
    for mut item in &**items {
        res = (::match_deref::match_deref! { match &(item.clone()) {
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name: __esc_name, .. }, .. }, .. } } => {
                name = (*__esc_name).clone();
                metamodelica::stringEq(&name, &className)
            },
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if res {
            break;
        }
    }
    res
}

pub(crate) fn getEnumerationLiterals(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut literals: metamodelica::List<metamodelica::Ref<Absyn::EnumLiteral>>;
    let mut names: metamodelica::List<ArcStr>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), program, false, false), '__try0)) {
            Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::ENUMERATION { enumLiterals: Deref @ Absyn::EnumDef::ENUMLITERALS { enumLiterals: __pa1 }, .. }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        literals = metamodelica::Own::own(__pa1);
        names = ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut l in (literals.clone()).into_iter().cloned() {
                let __x = AbsynUtil::enumLiteralName(&(l.clone()));
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        Ok::<_, &'static str>((names.clone(),))
    } {
        Ok((__try0_o0,)) => {
            names = __try0_o0;
        }
        Err(_) => {
            names = metamodelica::nil();
        }
    }
    result = ValuesMake::makeStringArray(names)?;
    Ok(result)
}

pub(crate) fn getDerivedClassModifierNames(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> metamodelica::List<ArcStr> {
    let mut outString: metamodelica::List<ArcStr>;
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    outString = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { restriction: Absyn::Restriction::R_TYPE { .. }, body: Deref @ Absyn::ClassDef::DERIVED { arguments: __esc_args, .. }, .. } => {
            args = (*__esc_args).clone();
            getModificationNames(metamodelica::AsArg::as_arg(&args))
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outString
}

pub(crate) fn getDerivedClassModifierValue(
    mut cls: &metamodelica::Ref<Absyn::Class>,
    mut path: &metamodelica::Ref<Absyn::Path>,
) -> ArcStr {
    let mut value: ArcStr;
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &((*cls)) {
            Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { arguments: __pa1, .. }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        args = metamodelica::Own::own(__pa1);
        value = unwrap_break_err!(Dump::printExpStr(unwrap_break_err!(getModificationValue(args.clone(), path), '__try0)), '__try0);
        Ok::<_, &'static str>((value.clone(),))
    } {
        Ok((__try0_o0,)) => {
            value = __try0_o0;
        }
        Err(_) => {
            value = literal!("");
        }
    }
    value
}

fn getElementitemContainsName(
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut inAbsynElementItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<metamodelica::Ref<Absyn::ElementItem>> {
    let mut outElementItem: metamodelica::Ref<Absyn::ElementItem>;
    outElementItem = 'mc: {
        let __mc_input = (inComponentRef, &**inAbsynElementItemLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, Deref @ metamodelica::ListNode::Cons { head: elt, tail: _ }) => {
                    getComponentsContainsName(cr.clone(), &(list![elt.clone()]))?;
                    Ok(elt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }) => {
                    let mut res: metamodelica::Ref<Absyn::ElementItem>;
                    res = getElementitemContainsName(cr.clone(), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outElementItem)
}

fn getComponentsContainsName(
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut inAbsynElementItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut outElementSpec: metamodelica::Ref<Absyn::ElementSpec>;
    outElementSpec = 'mc: {
        let __mc_input = (inComponentRef, &**inAbsynElementItemLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: res @ Deref @ Absyn::ElementSpec::COMPONENTS { components: ellst, .. }, .. } }, tail: _ }) => {
                    getCompitemNamed(cr.clone(), metamodelica::AsArg::as_arg(&ellst))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }) => {
                    let mut res: metamodelica::Ref<Absyn::ElementSpec>;
                    res = getComponentsContainsName(cr.clone(), metamodelica::AsArg::as_arg(&xs))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outElementSpec)
}

fn getElementContainsName(
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut inAbsynElementItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut outElement: metamodelica::Ref<Absyn::Element>;
    outElement = 'mc: {
        let __mc_input = (inComponentRef, &**inAbsynElementItemLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: res @ Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: ellst, .. }, .. } }, tail: _ }) => {
                    getCompitemNamed(cr.clone(), metamodelica::AsArg::as_arg(&ellst))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }) => {
                    let mut res: metamodelica::Ref<Absyn::Element>;
                    res = getElementContainsName(cr.clone(), metamodelica::AsArg::as_arg(&xs))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outElement)
}

fn getCompitemNamed(
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut inAbsynComponentItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
) -> Result<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut outComponentItem: metamodelica::Ref<Absyn::ComponentItem>;
    outComponentItem = 'mc: {
        let __mc_input = (inComponentRef, &**inAbsynComponentItemLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_IDENT { name: id1, .. }, Deref @ metamodelica::ListNode::Cons { head: x @ Deref @ Absyn::ComponentItem { component: Absyn::Component { name: id2, .. }, .. }, tail: _ }) => {
                    let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    Ok(x.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }) => {
                    let mut res: metamodelica::Ref<Absyn::ComponentItem>;
                    res = getCompitemNamed(cr.clone(), metamodelica::AsArg::as_arg(&xs))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outComponentItem)
}

pub(crate) fn existClass(mut classPath: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    match '__try0: {
        unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), program, false, false), '__try0);
        res = true;
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

pub(crate) fn isPrimitiveClass(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inProgram: Absyn::Program,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inClass, inProgram)) {
            (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_PREDEFINED_INTEGER { .. }, .. }, _) => {
                return Ok(true)
            },
            (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_PREDEFINED_REAL { .. }, .. }, _) => {
                return Ok(true)
            },
            (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_PREDEFINED_STRING { .. }, .. }, _) => {
                return Ok(true)
            },
            (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_PREDEFINED_BOOLEAN { .. }, .. }, _) => {
                return Ok(true)
            },
            (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_PREDEFINED_CLOCK { .. }, .. }, _) => {
                return Ok(true)
            },
            (Deref @ Absyn::Class { restriction: Absyn::Restriction::R_TYPE { .. }, .. }, _) => {
                return Ok(true)
            },
            (Deref @ Absyn::Class { name: cname, restriction: Absyn::Restriction::R_CLASS { .. }, body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path, arrayDim: _ }, .. }, .. }, p) => {
                let mut inmodel: metamodelica::Ref<Absyn::Path>;
                let mut cdef: metamodelica::Ref<Absyn::Class>;
                let mut res: bool;
                inmodel = AbsynUtil::crefToPath(&(metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: cname.clone(), subscripts: metamodelica::nil() })))?;
                (cdef, _) = lookupClassdef(path.clone(), inmodel, p.clone())?;
                { (inClass, inProgram) = (cdef, p.clone()); continue '__tco; }
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn addScope(
    mut inProgram: Absyn::Program,
    mut inVariableLst: metamodelica::List<InteractiveTypes::Variable>,
) -> Absyn::Program {
    let mut outProgram: Absyn::Program;
    outProgram = 'mc: {
        let __mc_input = (&inProgram, inVariableLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Absyn::Program { classes: cls, within_: Absyn::Within::TOP { .. } }, vars) => {
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let __pa0 = ::match_deref::match_deref! { match &(getVariableValue(literal!("scope"), metamodelica::AsArg::as_arg(&vars))?) {
                        Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa0 } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    path = metamodelica::Own::own(__pa0);
                    Ok(Absyn::Program { classes: cls.clone(), within_: Absyn::Within::WITHIN { path: path.clone() } })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Absyn::Program { classes: cls, within_: w }, vars) => {
                    if '__try0: {
                        unwrap_break_err!(getVariableValue(literal!("scope"), metamodelica::AsArg::as_arg(&vars)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok(Absyn::Program { classes: cls.clone(), within_: w.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Absyn::Program { classes: cls, within_: Absyn::Within::WITHIN { path: path2 } }, vars) => {
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut newpath: metamodelica::Ref<Absyn::Path>;
                    let __pa0 = ::match_deref::match_deref! { match &(getVariableValue(literal!("scope"), metamodelica::AsArg::as_arg(&vars))?) {
                        Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa0 } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    path = metamodelica::Own::own(__pa0);
                    newpath = AbsynUtil::joinPaths(path.clone(), path2.clone())?;
                    Ok(Absyn::Program { classes: cls.clone(), within_: Absyn::Within::WITHIN { path: newpath.clone() } })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inProgram.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outProgram
}

fn getVariableValue(
    mut inIdent: ArcStr,
    mut inVariableLst: &metamodelica::List<InteractiveTypes::Variable>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = (inIdent, &**inVariableLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id1, Deref @ metamodelica::ListNode::Cons { head: InteractiveTypes::Variable { varIdent: id2, value: v, .. }, tail: _ }) => {
                    let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id1, Deref @ metamodelica::ListNode::Cons { head: InteractiveTypes::Variable { varIdent: id2, .. }, tail: rest }) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let false = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    v = getVariableValue(id1.clone(), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(v.clone())
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

fn getVariableValueLst(
    mut ids: &metamodelica::List<ArcStr>,
    mut vars: &metamodelica::List<InteractiveTypes::Variable>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut val: metamodelica::Ref<Values::Value>;
    val = 'mc: {
        let __mc_input = (&**ids, &**vars);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: id1, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: InteractiveTypes::Variable { varIdent: id2, .. }, tail: rest }) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let false = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    v = getVariableValueLst(ids, metamodelica::AsArg::as_arg(&rest))?;
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: id1, tail: Deref @ metamodelica::ListNode::Cons { head: id2, tail: srest } }, Deref @ metamodelica::ListNode::Cons { head: InteractiveTypes::Variable { varIdent: id3, value: Deref @ Values::Value::RECORD { orderd: vals, comp, .. }, .. }, tail: _ }) => {
                    let mut ix: i32;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let true = (stringEq(&id1, &id3)) else { return Err("pattern mismatch") };
                    ix = List::position1OnTrue(metamodelica::AsArg::as_arg(&comp), &fnptr!(stringEq, ArcStr, ArcStr), id2.clone())?;
                    v = (vals).get(ix)?;
                    v = getVariableValueLst(&(metamodelica::cons(id2.clone(), srest.clone())), &(list![InteractiveTypes::Variable { varIdent: id2.clone(), value: v.clone(), type_: DAE::T_UNKNOWN_DEFAULT().clone() }]))?;
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: id1, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: InteractiveTypes::Variable { varIdent: id2, value: v, .. }, tail: _ }) => {
                    let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(val)
}

fn lookupClassdef(
    mut inPath1: metamodelica::Ref<Absyn::Path>,
    mut inPath2: metamodelica::Ref<Absyn::Path>,
    mut inProgram3: Absyn::Program,
) -> Result<(metamodelica::Ref<Absyn::Class>, metamodelica::Ref<Absyn::Path>)> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    (outClass, outPath) = 'mc: {
        let __mc_input = (inPath1, inPath2, inProgram3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, inmodel, p @ Absyn::Program { .. }) => {
                    let mut inmodeldef: metamodelica::Ref<Absyn::Class>;
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut newpath: metamodelica::Ref<Absyn::Path>;
                    let mut path = (*path).clone();
                    path = InstUtil::removeSelfReference(AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&inmodel)), path.clone())?;
                    inmodeldef = ProgramUtil::getPathedClassInProgram(inmodel.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    cdef = ProgramUtil::getPathedClassInProgram(path.clone(), &(Absyn::Program { classes: list![inmodeldef.clone()], within_: openmodelica_ast::Absyn::Within::TOP }), false, false)?;
                    newpath = AbsynUtil::joinPaths(inmodel.clone(), path.clone())?;
                    Ok((cdef.clone(), newpath.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, inmodel, p) => {
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut innewpath: metamodelica::Ref<Absyn::Path>;
                    let mut respath: metamodelica::Ref<Absyn::Path>;
                    innewpath = AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&inmodel))?;
                    (cdef, respath) = lookupClassdef(path.clone(), innewpath.clone(), p.clone())?;
                    Ok((cdef.clone(), respath.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, _, p) => {
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    cdef = ProgramUtil::getPathedClassInProgram(path.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    Ok((cdef.clone(), path.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: Deref @ "Real" }, _, _) => {
                    Ok((metamodelica::Ref::new(Absyn::Class { name: literal!("Real"), partialPrefix: false, finalPrefix: false, encapsulatedPrefix: false, restriction: openmodelica_ast::Absyn::Restriction::R_PREDEFINED_REAL, body: Absyn::dummyParts.clone(), commentsBeforeClass: metamodelica::nil(), commentsBeforeEnd: metamodelica::nil(), commentsAfterEnd: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Real") })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: Deref @ "Integer" }, _, _) => {
                    Ok((metamodelica::Ref::new(Absyn::Class { name: literal!("Integer"), partialPrefix: false, finalPrefix: false, encapsulatedPrefix: false, restriction: openmodelica_ast::Absyn::Restriction::R_PREDEFINED_INTEGER, body: Absyn::dummyParts.clone(), commentsBeforeClass: metamodelica::nil(), commentsBeforeEnd: metamodelica::nil(), commentsAfterEnd: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Integer") })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: Deref @ "String" }, _, _) => {
                    Ok((metamodelica::Ref::new(Absyn::Class { name: literal!("String"), partialPrefix: false, finalPrefix: false, encapsulatedPrefix: false, restriction: openmodelica_ast::Absyn::Restriction::R_PREDEFINED_STRING, body: Absyn::dummyParts.clone(), commentsBeforeClass: metamodelica::nil(), commentsBeforeEnd: metamodelica::nil(), commentsAfterEnd: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("String") })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: Deref @ "Boolean" }, _, _) => {
                    Ok((metamodelica::Ref::new(Absyn::Class { name: literal!("Boolean"), partialPrefix: false, finalPrefix: false, encapsulatedPrefix: false, restriction: openmodelica_ast::Absyn::Restriction::R_PREDEFINED_BOOLEAN, body: Absyn::dummyParts.clone(), commentsBeforeClass: metamodelica::nil(), commentsBeforeEnd: metamodelica::nil(), commentsAfterEnd: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Boolean") })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: Deref @ "Clock" }, _, _) => {
                    let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(Absyn::Class { name: literal!("Clock"), partialPrefix: false, finalPrefix: false, encapsulatedPrefix: false, restriction: openmodelica_ast::Absyn::Restriction::R_PREDEFINED_CLOCK, body: Absyn::dummyParts.clone(), commentsBeforeClass: metamodelica::nil(), commentsBeforeEnd: metamodelica::nil(), commentsAfterEnd: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Clock") })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, inmodel, _) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    s1 = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
                    s2 = AbsynUtil::pathString(inmodel.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![s1.clone(), s2.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outClass, outPath))
}

fn deleteOrUpdateComponent(
    mut componentName: ArcStr,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
    mut item: Option<(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::ComponentItem>)>,
) -> Result<Absyn::Program> {
    let mut program: Absyn::Program = program;
    let mut w: Absyn::Within;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    w = if (AbsynUtil::pathIsIdent(&classPath)) {
        openmodelica_ast::Absyn::Within::TOP
    } else {
        Absyn::Within::WITHIN {
            path: AbsynUtil::stripLast(&classPath)?,
        }
    };
    cls = ProgramUtil::getPathedClassInProgram(classPath, &program, false, false)?;
    cls = deleteOrUpdateComponentFromClass(componentName, cls, item)?;
    program = ProgramUtil::updateProgram(
        Absyn::Program {
            classes: list![cls],
            within_: w,
        },
        program,
        false,
        false,
    )?;
    Ok(program)
}

fn deleteOrUpdateComponentFromClass(
    mut inString: ArcStr,
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut item: Option<(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::ComponentItem>)>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    outClass = (::match_deref::match_deref! { match &(inClass) {
        __esc_outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmt }, info: _, .. } => {
            let mut name = inString;
            outClass = (*__esc_outClass).clone();
            let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut publst2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut protlst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut protlst2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut l2: i32;
            let mut l1: i32;
            let mut l1_1: i32;
            let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            let mut success: bool;
            publst = ProgramUtil::getPublicList(metamodelica::AsArg::as_arg(&parts));
            (publst2, success) = deleteOrUpdateComponentFromElementitems(name.clone(), &publst, item.clone())?;
            l2 = ((publst2).len() as i32);
            l1 = ((publst).len() as i32);
            l1_1 = l1 - 1;
            if intEq(l1_1, l2) && (item).is_none() && success || boolNot(intEq(l1_1, l2)) && (item).is_some() && success {
                parts2 = ProgramUtil::replacePublicList(metamodelica::AsArg::as_arg(&parts), publst2)?;
            } else {
                protlst = ProgramUtil::getProtectedList(metamodelica::AsArg::as_arg(&parts));
                (protlst2, _) = deleteOrUpdateComponentFromElementitems(name, &protlst, item)?;
                parts2 = ProgramUtil::replaceProtectedList(metamodelica::AsArg::as_arg(&parts), protlst2)?;
            }
            assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2, ann: ann.clone(), comment: cmt.clone() }));
            outClass.clone()
        },
        __esc_outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcpath, modifications: r#mod, parts, ann, comment: cmt }, info: _, .. } => {
            let mut name = inString;
            outClass = (*__esc_outClass).clone();
            let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut publst2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut protlst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut protlst2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut l2: i32;
            let mut l1: i32;
            let mut l1_1: i32;
            let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            let mut success: bool;
            publst = ProgramUtil::getPublicList(metamodelica::AsArg::as_arg(&parts));
            (publst2, success) = deleteOrUpdateComponentFromElementitems(name.clone(), &publst, item.clone())?;
            l2 = ((publst2).len() as i32);
            l1 = ((publst).len() as i32);
            l1_1 = l1 - 1;
            if intEq(l1_1, l2) && (item).is_none() && success || boolNot(intEq(l1_1, l2)) && (item).is_some() && success {
                parts2 = ProgramUtil::replacePublicList(metamodelica::AsArg::as_arg(&parts), publst2)?;
            } else {
                protlst = ProgramUtil::getProtectedList(metamodelica::AsArg::as_arg(&parts));
                (protlst2, _) = deleteOrUpdateComponentFromElementitems(name, &protlst, item)?;
                parts2 = ProgramUtil::replaceProtectedList(metamodelica::AsArg::as_arg(&parts), protlst2)?;
            }
            assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcpath.clone(), modifications: r#mod.clone(), comment: cmt.clone(), parts: parts2, ann: ann.clone() }));
            outClass.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outClass)
}

fn deleteOrUpdateComponentFromElementitems(
    mut inString: ArcStr,
    mut inAbsynElementItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut item: Option<(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::ComponentItem>)>,
) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>, bool)> {
    let mut outAbsynElementItemLst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut success: bool;
    (outAbsynElementItemLst, success) = (::match_deref::match_deref! { match inAbsynElementItemLst {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), false)
        },
        Deref @ metamodelica::ListNode::Cons { head: x @ Deref @ Absyn::ElementItem::ELEMENTITEM { element: elt @ Deref @ Absyn::Element::ELEMENT { specification: spec @ Deref @ Absyn::ElementSpec::COMPONENTS { typeSpec: typeSpec @ Deref @ Absyn::TypeSpec::TPATH { .. }, components: comps, .. }, .. } }, tail: xs } => {
            let mut name = inString;
            let mut name2: ArcStr;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut eltold: metamodelica::Ref<Absyn::Element>;
            let mut compitem: metamodelica::Ref<Absyn::ComponentItem>;
            let mut tppath: metamodelica::Ref<Absyn::Path>;
            let mut hasOtherComponents: bool;
            let mut successResult: bool;
            let mut elt = (*elt).clone();
            let mut spec = (*spec).clone();
            let mut typeSpec = (*typeSpec).clone();
            if ({
        let mut __acc: Option<bool> = None;
        for mut c in (comps.clone()).into_iter().cloned() {
            let __x = (match &*c.clone() {
        Absyn::ComponentItem { component: Absyn::Component { name: name2, .. }, .. } if (stringEq(&name, &name2)) => true,
        _ => false,
    });
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(false)
    }) {
                (res, successResult) = (::match_deref::match_deref! { match &(item) {
        Some((__esc_tppath, __esc_compitem)) => {
            tppath = (*__esc_tppath).clone();
            compitem = (*__esc_compitem).clone();
            if AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&tppath), &(AbsynUtil::typeSpecPath(var_field!((*spec).typeSpec, Absyn::ElementSpec::COMPONENTS)))) {
                assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; components = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> = metamodelica::nil();
        for mut c in (comps.clone()).into_iter().cloned() {
            let __x = (match &*c.clone() {
        Absyn::ComponentItem { component: Absyn::Component { name: name2, .. }, .. } if (stringEq(&name, &name2)) => compitem.clone(),
        _ => c.clone(),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
                assign_variant_field!(elt => Absyn::Element::ELEMENT; specification = spec.clone());
                res = metamodelica::cons(metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: elt.clone() }), xs.clone());
                successResult = true;
            } else {
                eltold = elt.clone();
                assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; components = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> = metamodelica::nil();
        for mut c in (comps.clone()).into_iter().cloned() {
            if !((match &*c.clone() {
        Absyn::ComponentItem { component: Absyn::Component { name: __esc_name2, .. }, .. } => {
            name2 = (*__esc_name2).clone();
            !(stringEq(&name, &name2))
        },
        _ => true,
    })) { continue; }
            let __x = c.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
                hasOtherComponents = !((var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS)).is_empty());
                if hasOtherComponents {
                    assign_variant_field!(elt => Absyn::Element::ELEMENT; specification = spec.clone());
                    eltold = elt.clone();
                }
                assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; components = list![compitem.clone()]);
                assign_variant_field!(typeSpec => Absyn::TypeSpec::TPATH; path = tppath.clone());
                assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; typeSpec = typeSpec.clone());
                assign_variant_field!(elt => Absyn::Element::ELEMENT; specification = spec.clone());
                res = metamodelica::cons(metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: elt.clone() }), xs.clone());
                if hasOtherComponents {
                    res = metamodelica::cons(metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: eltold }), res);
                }
                successResult = true;
            }
            (res, successResult)
        },
        _ => {
            if ((comps).len() as i32) == 1 {
                res = xs.clone();
                successResult = true;
            } else {
                assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; components = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> = metamodelica::nil();
        for mut c in (comps.clone()).into_iter().cloned() {
            if !((match &*c.clone() {
        Absyn::ComponentItem { component: Absyn::Component { name: __esc_name2, .. }, .. } => {
            name2 = (*__esc_name2).clone();
            !(stringEq(&name, &name2))
        },
        _ => true,
    })) { continue; }
            let __x = c.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
                assign_variant_field!(elt => Absyn::Element::ELEMENT; specification = spec.clone());
                res = metamodelica::cons(metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: elt.clone() }), xs.clone());
                successResult = true;
            }
            (res, successResult)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            } else {
                (res, successResult) = deleteOrUpdateComponentFromElementitems(name, xs, item)?;
                res = metamodelica::cons(x.clone(), res);
            }
            (res, successResult)
        },
        Deref @ metamodelica::ListNode::Cons { head: x, tail: xs } => {
            let mut name = inString;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut successResult: bool;
            (res, successResult) = deleteOrUpdateComponentFromElementitems(name, xs, item)?;
            (metamodelica::cons(x.clone(), res), successResult)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outAbsynElementItemLst, success))
}

pub(crate) fn addComponent(
    mut componentName: ArcStr,
    mut typeName: metamodelica::Ref<Absyn::Path>,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut bindingExp: metamodelica::Ref<Absyn::Exp>,
    mut modifier: metamodelica::Ref<Absyn::Modification>,
    mut commentExp: &metamodelica::Ref<Absyn::Exp>,
    mut annotationExp: &metamodelica::Ref<Absyn::Exp>,
    mut program: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = true;
    let mut filename: ArcStr;
    let mut cdef: metamodelica::Ref<Absyn::Class>;
    let mut annotation_: Option<metamodelica::Ref<Absyn::Comment>>;
    let mut modification: Option<metamodelica::Ref<Absyn::Modification>>;
    let mut w: Absyn::Within;
    let mut io: Absyn::InnerOuter;
    let mut redecl: Option<Absyn::RedeclareKeywords>;
    let mut attr: Absyn::ElementAttributes;
    let mut info: SourceInfo;
    let mut ty_path: metamodelica::Ref<Absyn::Path>;
    if '__try0: {
        w = (match &*classPath {
        Absyn::Path::IDENT { .. } => openmodelica_ast::Absyn::Within::TOP,
        _ => Absyn::Within::WITHIN { path: unwrap_break_err!(AbsynUtil::stripLast(&classPath), '__try0) },
    });
        let (__pa2, __pa1) = ::match_deref::match_deref! { match &(unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false), '__try0)) {
            __pa2 @ Deref @ Absyn::Class { info: SourceInfo { fileName: __pa1, .. }, .. } => (__pa2.clone(), __pa1.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        filename = metamodelica::Own::own(__pa1);
        cdef = metamodelica::Own::own(__pa2);
        info = SourceInfo { fileName: filename.clone(), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) };
        annotation_ = unwrap_break_err!(InteractiveUtil::makeCommentFromArgs(commentExp, annotationExp, None), '__try0);
        modification = unwrap_break_err!(InteractiveUtil::makeModifierFromArgs(bindingExp.clone(), modifier.clone(), info.clone(), None), '__try0);
        (io, redecl, attr) = unwrap_break_err!(getDefaultPrefixes(program.clone(), typeName.clone()), '__try0);
        let __pa3 = ::match_deref::match_deref! { match &(unwrap_break_err!(AbsynUtil::pathStripSamePrefix(typeName.clone(), classPath.clone()), '__try0)) {
            Some(__pa3) => __pa3.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        ty_path = metamodelica::Own::own(__pa3);
        if AbsynUtil::pathContains(&classPath, &(AbsynUtil::pathFirstIdent(&ty_path))) {
            ty_path = typeName.clone();
        }
        cdef = unwrap_break_err!(InteractiveUtil::addToPublic(cdef.clone(), metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: metamodelica::Ref::new(Absyn::Element::ELEMENT { finalPrefix: false, redeclareKeywords: redecl.clone(), innerOuter: io, specification: metamodelica::Ref::new(Absyn::ElementSpec::COMPONENTS { attributes: attr.clone(), typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: ty_path.clone(), arrayDim: None }), components: list![metamodelica::Ref::new(Absyn::ComponentItem { component: Absyn::Component { name: componentName.clone(), arrayDim: metamodelica::nil(), modification: modification.clone() }, condition: None, comment: annotation_.clone() })] }), info: info.clone(), constrainClass: None }) })), '__try0);
        program = unwrap_break_err!(ProgramUtil::updateProgram(Absyn::Program { classes: list![cdef.clone()], within_: w.clone() }, program.clone(), false, false), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
        success = false;
    }
    (program, success)
}

fn getDefaultPrefixes(
    mut p: Absyn::Program,
    mut className: metamodelica::Ref<Absyn::Path>,
) -> Result<(
    Absyn::InnerOuter,
    Option<Absyn::RedeclareKeywords>,
    Absyn::ElementAttributes,
)> {
    let mut io: Absyn::InnerOuter;
    let mut redecl: Option<Absyn::RedeclareKeywords>;
    let mut attr: Absyn::ElementAttributes;
    (io, redecl, attr) = (match &*className {
        _ => {
            let mut r#str: ArcStr;
            r#str = ProgramUtil::getNamedAnnotationExp(
                className.clone(),
                p.clone(),
                &(metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("defaultComponentPrefixes"),
                })),
                Some(literal!("{}")),
                &fnptr!(
                    ProgramUtil::getDefaultComponentPrefixesModStr,
                    Option<metamodelica::Ref<Absyn::Modification>>
                ),
            )?;
            io = getDefaultInnerOuter(r#str.clone())?;
            redecl = getDefaultReplaceable(r#str.clone())?;
            redecl = makeReplaceableIfPartial(&p, className, redecl)?;
            attr = getDefaultAttr(r#str);
            (io, redecl, attr)
        }
    });
    Ok((io, redecl, attr))
}

fn makeReplaceableIfPartial(
    mut p: &Absyn::Program,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut redecl: Option<Absyn::RedeclareKeywords>,
) -> Result<Option<Absyn::RedeclareKeywords>> {
    let mut new_redecl: Option<Absyn::RedeclareKeywords>;
    new_redecl = (match redecl.clone() {
        None if (isPartial(className.clone(), p)) => Some(openmodelica_ast::Absyn::RedeclareKeywords::REPLACEABLE),
        None => redecl,
        Some(Absyn::RedeclareKeywords::REPLACEABLE { .. }) => redecl,
        _ => return Err("match: no arm matched"),
    });
    Ok(new_redecl)
}

fn getDefaultInnerOuter(mut r#str: ArcStr) -> Result<Absyn::InnerOuter> {
    let mut io: Absyn::InnerOuter;
    io = 'mc: {
        let __mc_input = r#str.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let (-1) = (System::stringFind(r#str.clone(), literal!("inner"))?) else {
                return Err("pattern mismatch");
            };
            let (-1) = (System::stringFind(r#str.clone(), literal!("outer"))?) else {
                return Err("pattern mismatch");
            };
            Ok(openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let (-1) = (System::stringFind(r#str.clone(), literal!("outer"))?) else {
                return Err("pattern mismatch");
            };
            Ok(openmodelica_ast::Absyn::InnerOuter::INNER)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let (-1) = (System::stringFind(r#str.clone(), literal!("inner"))?) else {
                return Err("pattern mismatch");
            };
            Ok(openmodelica_ast::Absyn::InnerOuter::OUTER)
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(io)
}

fn getDefaultReplaceable(mut r#str: ArcStr) -> Result<Option<Absyn::RedeclareKeywords>> {
    let mut repl: Option<Absyn::RedeclareKeywords>;
    repl = 'mc: {
        let __mc_input = r#str.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let (-1) = (System::stringFind(r#str.clone(), literal!("replaceable"))?) else {
                return Err("pattern mismatch");
            };
            Ok(None)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if '__try0: {
                let (-1) = (unwrap_break_err!(System::stringFind(r#str.clone(), literal!("replaceable")), '__try0))
                else {
                    break '__try0 Err::<_, _>("pattern mismatch");
                };
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            Ok(Some(openmodelica_ast::Absyn::RedeclareKeywords::REPLACEABLE))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(repl)
}

fn getDefaultAttr(mut r#str: ArcStr) -> Absyn::ElementAttributes {
    let mut attr: Absyn::ElementAttributes;
    attr = 'mc: {
        let __mc_input = r#str.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if '__try0: {
                let (-1) = (unwrap_break_err!(System::stringFind(r#str.clone(), literal!("parameter")), '__try0))
                else {
                    break '__try0 Err::<_, _>("pattern mismatch");
                };
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            Ok(Absyn::ElementAttributes {
                flowPrefix: false,
                streamPrefix: false,
                parallelism: openmodelica_ast::Absyn::Parallelism::NON_PARALLEL,
                variability: openmodelica_ast::Absyn::Variability::PARAM,
                direction: openmodelica_ast::Absyn::Direction::BIDIR,
                isField: openmodelica_ast::Absyn::IsField::NONFIELD,
                arrayDim: metamodelica::nil(),
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if '__try0: {
                let (-1) = (unwrap_break_err!(System::stringFind(r#str.clone(), literal!("constant")), '__try0)) else {
                    break '__try0 Err::<_, _>("pattern mismatch");
                };
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            Ok(Absyn::ElementAttributes {
                flowPrefix: false,
                streamPrefix: false,
                parallelism: openmodelica_ast::Absyn::Parallelism::NON_PARALLEL,
                variability: openmodelica_ast::Absyn::Variability::CONST,
                direction: openmodelica_ast::Absyn::Direction::BIDIR,
                isField: openmodelica_ast::Absyn::IsField::NONFIELD,
                arrayDim: metamodelica::nil(),
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if '__try0: {
                let (-1) = (unwrap_break_err!(System::stringFind(r#str.clone(), literal!("discrete")), '__try0)) else {
                    break '__try0 Err::<_, _>("pattern mismatch");
                };
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            Ok(Absyn::ElementAttributes {
                flowPrefix: false,
                streamPrefix: false,
                parallelism: openmodelica_ast::Absyn::Parallelism::NON_PARALLEL,
                variability: openmodelica_ast::Absyn::Variability::DISCRETE,
                direction: openmodelica_ast::Absyn::Direction::BIDIR,
                isField: openmodelica_ast::Absyn::IsField::NONFIELD,
                arrayDim: metamodelica::nil(),
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(Absyn::ElementAttributes {
                flowPrefix: false,
                streamPrefix: false,
                parallelism: openmodelica_ast::Absyn::Parallelism::NON_PARALLEL,
                variability: openmodelica_ast::Absyn::Variability::VAR,
                direction: openmodelica_ast::Absyn::Direction::BIDIR,
                isField: openmodelica_ast::Absyn::IsField::NONFIELD,
                arrayDim: metamodelica::nil(),
            })
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    attr
}

pub(crate) fn updateComponent(
    mut componentName: ArcStr,
    mut typeName: metamodelica::Ref<Absyn::Path>,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut bindingExp: metamodelica::Ref<Absyn::Exp>,
    mut modifier: metamodelica::Ref<Absyn::Modification>,
    mut commentExp: &metamodelica::Ref<Absyn::Exp>,
    mut annotationExp: &metamodelica::Ref<Absyn::Exp>,
    mut program: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = true;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut protlst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut items: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    let mut arrayDimensions: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>;
    let mut modification: Option<metamodelica::Ref<Absyn::Modification>>;
    let mut cond: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut ann: Option<metamodelica::Ref<Absyn::Comment>>;
    let mut annotation_: Option<metamodelica::Ref<Absyn::Comment>>;
    if '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false), '__try0)) {
            Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: __pa1, .. }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        parts = metamodelica::Own::own(__pa1);
        publst = ProgramUtil::getPublicList(&parts);
        protlst = ProgramUtil::getProtectedList(&parts);
        let __pa3 = ::match_deref::match_deref! { match &(unwrap_break_err!(getElementContainsName(metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: componentName.clone(), subscripts: metamodelica::nil() }), &(listAppend(publst.clone(), protlst.clone()))), '__try0)) {
            Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: _, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: _, arrayDim: _ }, components: __pa3 }, info: _, constrainClass: _ } => __pa3.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        items = metamodelica::Own::own(__pa3);
        let __arc9 = unwrap_break_err!(getCompitemNamed(metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: componentName.clone(), subscripts: metamodelica::nil() }), &items), '__try0);
        let Absyn::COMPONENTITEM { component: Absyn::COMPONENT { name: _, arrayDim: __pa5, modification: __pa6 }, condition: __pa7, comment: __pa8 } = &*__arc9;
        arrayDimensions = metamodelica::Own::own(__pa5);
        r#mod = metamodelica::Own::own(__pa6);
        cond = metamodelica::Own::own(__pa7);
        ann = metamodelica::Own::own(__pa8);
        annotation_ = unwrap_break_err!(InteractiveUtil::makeCommentFromArgs(commentExp, annotationExp, ann.clone()), '__try0);
        modification = unwrap_break_err!(InteractiveUtil::makeModifierFromArgs(bindingExp.clone(), modifier.clone(), Absyn::dummyInfo.clone(), r#mod.clone()), '__try0);
        program = unwrap_break_err!(deleteOrUpdateComponent(componentName.clone(), classPath.clone(), program.clone(), Some((typeName.clone(), metamodelica::Ref::new(Absyn::ComponentItem { component: Absyn::Component { name: componentName.clone(), arrayDim: arrayDimensions.clone(), modification: modification.clone() }, condition: cond.clone(), comment: annotation_.clone() })))), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
        success = false;
    }
    (program, success)
}

pub(crate) fn deleteComponent(
    mut componentName: ArcStr,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = true;
    if '__try0: {
        program = unwrap_break_err!(deleteOrUpdateComponent(componentName.clone(), classPath.clone(), program.clone(), None), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
        success = false;
    }
    (program, success)
}

pub(crate) fn addClassAnnotation(
    mut inClass: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inAnnotation: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inProgram: Absyn::Program,
) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    let mut class_path: metamodelica::Ref<Absyn::Path>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut class_within: Absyn::Within;
    class_path = AbsynUtil::crefToPath(inClass)?;
    cls = ProgramUtil::getPathedClassInProgram(class_path.clone(), &inProgram, false, false)?;
    cls = addClassAnnotationToClass(cls, InteractiveUtil::annotationListToAbsyn(inAnnotation)?)?;
    class_within = if (AbsynUtil::pathIsIdent(&class_path)) {
        openmodelica_ast::Absyn::Within::TOP
    } else {
        Absyn::Within::WITHIN {
            path: AbsynUtil::stripLast(&class_path)?,
        }
    };
    outProgram = ProgramUtil::updateProgram(
        Absyn::Program {
            classes: list![cls],
            within_: class_within,
        },
        inProgram,
        false,
        false,
    )?;
    Ok(outProgram)
}

pub(crate) fn addClassAnnotationToClass(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inAnnotation: metamodelica::Ref<Absyn::Annotation>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    let mut body: metamodelica::Ref<Absyn::ClassDef>;
    let __arc1 = inClass.clone();
    let Absyn::CLASS { body: __pa0, .. } = &*__arc1;
    body = metamodelica::Own::own(__pa0);
    body = (match &*body {
        Absyn::ClassDef::PARTS { ann: __body_ann, .. } => {
            assign_variant_field!(body => Absyn::ClassDef::PARTS; ann = list![AbsynUtil::mergeAnnotationsList(inAnnotation, metamodelica::AsArg::as_arg(&__body_ann))?]);
            body
        }
        Absyn::ClassDef::DERIVED {
            comment: __body_comment,
            ..
        } => {
            assign_variant_field!(body => Absyn::ClassDef::DERIVED; comment = AbsynUtil::mergeCommentAnnotation(inAnnotation, __body_comment.clone())?);
            body
        }
        Absyn::ClassDef::ENUMERATION {
            comment: __body_comment,
            ..
        } => {
            assign_variant_field!(body => Absyn::ClassDef::ENUMERATION; comment = AbsynUtil::mergeCommentAnnotation(inAnnotation, __body_comment.clone())?);
            body
        }
        Absyn::ClassDef::OVERLOAD {
            comment: __body_comment,
            ..
        } => {
            assign_variant_field!(body => Absyn::ClassDef::OVERLOAD; comment = AbsynUtil::mergeCommentAnnotation(inAnnotation, __body_comment.clone())?);
            body
        }
        Absyn::ClassDef::CLASS_EXTENDS { ann: __body_ann, .. } => {
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; ann = list![AbsynUtil::mergeAnnotationsList(inAnnotation, metamodelica::AsArg::as_arg(&__body_ann))?]);
            body
        }
        Absyn::ClassDef::PDER {
            comment: __body_comment,
            ..
        } => {
            assign_variant_field!(body => Absyn::ClassDef::PDER; comment = AbsynUtil::mergeCommentAnnotation(inAnnotation, __body_comment.clone())?);
            body
        }
    });
    outClass = AbsynUtil::setClassBody(inClass, body);
    Ok(outClass)
}

fn getInheritedClassesHelper(
    mut inClass1: metamodelica::Ref<SCode::Element>,
    mut inClass2: metamodelica::Ref<Absyn::Class>,
    mut inEnv4: FCore::Graph,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut outAbsynComponentRefLst: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    outAbsynComponentRefLst = 'mc: {
        let __mc_input = (inClass1, inClass2, inEnv4);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c @ Deref @ SCode::Element::CLASS { name: id, encapsulatedPrefix: encflag, restriction: restr, .. }, cdef, env) => {
                    let mut lst: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    let mut env2: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut ci_state: ClassInf::State;
                    ErrorExt::setCheckpoint(literal!("getInheritedClassesHelper"));
                    if SCodeUtil::isDerivedClass(metamodelica::AsArg::as_arg(&c)) {
                        env_2 = env.clone();
                    } else {
                        env2 = FGraph::openScope(env.clone(), encflag.clone(), id.clone(), FGraph::restrictionToScopeType(metamodelica::AsArg::as_arg(&restr)))?;
                        ci_state = ClassInfUtil::start(metamodelica::AsArg::as_arg(&restr), FGraph::getGraphName(&env2)?)?;
                        (_, env_2, _, _, _) = Inst::partialInstClassIn(FCore::emptyCache(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0)?;
                    }
                    lst = getBaseClasses(metamodelica::AsArg::as_arg(&cdef), env_2.clone());
                    ErrorExt::rollBack(literal!("getInheritedClassesHelper"));
                    Ok(lst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, _, _) => {
                    ErrorExt::rollBack(literal!("getInheritedClassesHelper"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAbsynComponentRefLst)
}

pub(crate) fn getInheritedClasses(
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut outPaths: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
    if Flags::isSet(Flags::NF_API.clone())? {
        outPaths = NFApi::getInheritedClasses(inPath, SymbolTable::getAbsyn())?;
        return Ok(outPaths);
    }
    if '__try0: {
        if !(unwrap_break_err!(Flags::isSet(Flags::NF_API_NOISE.clone()), '__try0)) {
            ErrorExt::setCheckpoint(literal!("getInheritedClasses"));
        }
        outPaths = 'mc: {
        let __mc_input = inPath.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                modelpath => {
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut p_1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut env: FCore::Graph;
                    let mut env_1: FCore::Graph;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut cache: FCore::Cache;
                    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    cdef = ProgramUtil::getPathedClassInProgram(modelpath.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    p_1 = SymbolTable::getSCode()?;
                    (cache, env) = Inst::makeEnvFromProgram(&p_1)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupClass(&cache, &env, metamodelica::AsArg::as_arg(&modelpath), None)?) {
                        (_, __pa0 @ Deref @ SCode::Element::CLASS { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    c = metamodelica::Own::own(__pa0);
                    env_1 = metamodelica::Own::own(__pa1);
                    paths = getInheritedClassesHelper(c.clone(), cdef.clone(), env_1.clone())?;
                    if '__try3: {
                        ::match_deref::match_deref! { match &(paths.clone()) {
                            Deref @ metamodelica::ListNode::Nil => (),
                            _ => break '__try3 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok(paths.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                modelpath => {
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut extendsLst: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
                    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    cdef = ProgramUtil::getPathedClassInProgram(modelpath.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    extendsLst = getExtendsInClass(&cdef);
                    paths = List::map(extendsLst.clone(), &move |__a0: metamodelica::Ref<Absyn::ElementSpec>| AbsynUtil::elementSpecToPath(&__a0))?;
                    Ok(paths.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() { break 'mc __v; }
        break '__try0 Err::<_, _>("matchcontinue: no arm matched")
    };
        if !(unwrap_break_err!(Flags::isSet(Flags::NF_API_NOISE.clone()), '__try0)) {
            ErrorExt::rollBack(literal!("getInheritedClasses"));
        }
        Ok::<(), &'static str>(())
    }.is_err() {
        if !(Flags::isSet(Flags::NF_API_NOISE.clone())?) {
            ErrorExt::rollBack(literal!("getInheritedClasses"));
        }
    }
    Ok(outPaths)
}

pub(crate) fn getInheritanceCount(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    match '__try0: {
        cls =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), program, false, false), '__try0);
        result = ValuesMake::makeInteger(countBaseClasses(&cls));
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeInteger(0);
        }
    }
    result
}

fn getNthInheritedClassAnnotationOpt(
    mut inModelPath: &metamodelica::Ref<Absyn::Path>,
    mut inInteger: i32,
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inProgram: &Absyn::Program,
) -> (ArcStr, Option<metamodelica::Ref<Absyn::Annotation>>) {
    let mut outString: ArcStr;
    let mut annotationOpt: Option<metamodelica::Ref<Absyn::Annotation>>;
    (outString, annotationOpt) = 'mc: {
        let __mc_input = inInteger;
        if let Ok(__v) = (|| -> Result<_> {
            let mut n = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut cdef: metamodelica::Ref<Absyn::Class>;
            let mut s: ArcStr;
            let mut extends_: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
            let mut annOpt: Option<metamodelica::Ref<Absyn::Annotation>>;
            cdef = inClass.clone();
            extends_ = getExtendsInClass(&cdef);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &((extends_).get(n.clone())?) {
                Deref @ Absyn::ElementSpec::EXTENDS { path: __pa0, elementArg: _, annotationOpt: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            path = metamodelica::Own::own(__pa0);
            annOpt = metamodelica::Own::own(__pa1);
            s = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            Ok((s.clone(), annOpt.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((literal!("Error"), None))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outString, annotationOpt)
}

fn getMapAnnotationStr(
    mut inAbsynElementArgLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inMapType: &ArcStr,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inFullProgram: &Absyn::Program,
    mut inModelPath: &metamodelica::Ref<Absyn::Path>,
) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = &**inAbsynElementArgLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(literal!("{}"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: ann @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: mapType }, .. }, tail: _ } => {
                    let mut r#str: ArcStr;
                    let true = (stringEqual(&mapType, &inMapType)) else { return Err("pattern mismatch") };
                    r#str = getAnnotationString(metamodelica::Ref::new(Absyn::Annotation { elementArgs: list![ann.clone()] }), inClass, inFullProgram.clone(), inModelPath.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                    let mut r#str: ArcStr;
                    r#str = getMapAnnotationStr(metamodelica::AsArg::as_arg(&xs), inMapType, inClass, inFullProgram, inModelPath);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

pub(crate) fn getNthInheritedClassIconMapAnnotation(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut n: i32,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    fn r#impl(
        mut classPath: metamodelica::Ref<Absyn::Path>,
        mut n: i32,
        mut program: &Absyn::Program,
        mut accessLevel: Access,
    ) -> Result<metamodelica::Ref<Values::Value>> {
        let mut result: metamodelica::Ref<Values::Value>;
        result = getNthInheritedClassMapAnnotation(classPath, n, program, &(literal!("IconMap")))?;
        Ok(result)
    }

    let mut result: metamodelica::Ref<Values::Value>;
    result = InteractiveUtil::accessClass(
        classPath,
        program,
        &({
            let __pe_b1 = n;
            move |__pe_a0, __pe_a2, __pe_a3| r#impl(__pe_a0, __pe_b1.clone(), &__pe_a2, __pe_a3)
        }),
        true,
        false,
        Access::icon.clone(),
    )?;
    Ok(result)
}

pub(crate) fn getNthInheritedClassDiagramMapAnnotation(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut n: i32,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    fn r#impl(
        mut classPath: metamodelica::Ref<Absyn::Path>,
        mut n: i32,
        mut program: &Absyn::Program,
        mut accessLevel: Access,
    ) -> Result<metamodelica::Ref<Values::Value>> {
        let mut result: metamodelica::Ref<Values::Value>;
        result = getNthInheritedClassMapAnnotation(classPath, n, program, &(literal!("DiagramMap")))?;
        Ok(result)
    }

    let mut result: metamodelica::Ref<Values::Value>;
    result = InteractiveUtil::accessClass(
        classPath,
        program,
        &({
            let __pe_b1 = n;
            move |__pe_a0, __pe_a2, __pe_a3| r#impl(__pe_a0, __pe_b1.clone(), &__pe_a2, __pe_a3)
        }),
        true,
        false,
        Access::icon.clone(),
    )?;
    Ok(result)
}

fn getNthInheritedClassMapAnnotation(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut n: i32,
    mut program: &Absyn::Program,
    mut mapType: &ArcStr,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut s: ArcStr;
    let mut annStr: ArcStr;
    let mut opt_ann: Option<metamodelica::Ref<Absyn::Annotation>>;
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    cls = ProgramUtil::getPathedClassInProgram(classPath.clone(), program, false, false)?;
    (s, opt_ann) = getNthInheritedClassAnnotationOpt(&classPath, n, cls.clone(), program);
    annStr = (::match_deref::match_deref! { match &(opt_ann) {
        Some(Deref @ Absyn::Annotation { elementArgs: __esc_args }) => {
            args = (*__esc_args).clone();
            getMapAnnotationStr(metamodelica::AsArg::as_arg(&args), mapType, &cls, program, &classPath)
        },
        _ => literal!("{}"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result = InteractiveUtil::makeAnnotationArrayValue(list![s, annStr]);
    Ok(result)
}

fn getExtendsInClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut outExtends: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outExtends = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: __esc_parts, .. }, .. } => {
            parts = (*__esc_parts).clone();
            getExtendsInParts(metamodelica::AsArg::as_arg(&parts))
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: __esc_parts, .. }, .. } => {
            parts = (*__esc_parts).clone();
            getExtendsInParts(metamodelica::AsArg::as_arg(&parts))
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExtends
}

fn getExtendsInParts(
    mut parts: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut outExtends: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>> = metamodelica::nil();
    let mut spec: metamodelica::Ref<Absyn::ElementSpec>;
    for mut part in &**parts {
        for mut el in &*AbsynUtil::getElementItemsInClassPart(metamodelica::AsArg::as_arg(&part)) {
            outExtends = (::match_deref::match_deref! { match &(el.clone()) {
                Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: __esc_spec @ Deref @ Absyn::ElementSpec::EXTENDS { .. }, .. } } => {
                    spec = (*__esc_spec).clone();
                    metamodelica::cons(spec.clone(), outExtends)
                },
                _ => outExtends,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
    }
    outExtends = Dangerous::listReverseInPlace(outExtends);
    outExtends
}

pub(crate) fn getComponentCount(mut model_: metamodelica::Ref<Absyn::Path>, mut p: &Absyn::Program) -> Result<i32> {
    let mut count: i32;
    let mut cdef: metamodelica::Ref<Absyn::Class>;
    cdef = ProgramUtil::getPathedClassInProgram(model_, p, false, false)?;
    count = countComponents(cdef)?;
    Ok(count)
}

fn countComponents(mut inClass: metamodelica::Ref<Absyn::Class>) -> Result<i32> {
    let mut outInteger: i32;
    outInteger = 'mc: {
        let __mc_input = inClass;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cdef @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elt }, tail: lst }, ann, comment: cmt, .. }, .. } => {
                    let mut c1: i32;
                    let mut c2: i32;
                    let mut cdef = (*cdef).clone();
                    assign_field!(cdef.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: lst.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    c1 = countComponents(cdef.clone())?;
                    c2 = countComponentsInElts(metamodelica::AsArg::as_arg(&elt), 0);
                    Ok(c1 + c2)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cdef @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elt }, tail: lst }, ann, comment: cmt, .. }, .. } => {
                    let mut c1: i32;
                    let mut c2: i32;
                    let mut cdef = (*cdef).clone();
                    assign_field!(cdef.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: lst.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    c1 = countComponents(cdef.clone())?;
                    c2 = countComponentsInElts(metamodelica::AsArg::as_arg(&elt), 0);
                    Ok(c1 + c2)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cdef @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::ListNode::Cons { head: _, tail: lst }, ann, comment: cmt, .. }, .. } => {
                    let mut res: i32;
                    let mut cdef = (*cdef).clone();
                    assign_field!(cdef.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: lst.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    res = countComponents(cdef.clone())?;
                    Ok(res)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::ListNode::Nil, .. }, .. } => {
                    Ok(0)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cdef @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elt }, tail: lst }, ann, comment: cmt, .. }, .. } => {
                    let mut c1: i32;
                    let mut c2: i32;
                    let mut cdef = (*cdef).clone();
                    assign_field!(cdef.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: lst.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    c1 = countComponents(cdef.clone())?;
                    c2 = countComponentsInElts(metamodelica::AsArg::as_arg(&elt), 0);
                    Ok(c1 + c2)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cdef @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elt }, tail: lst }, ann, comment: cmt, .. }, .. } => {
                    let mut c1: i32;
                    let mut c2: i32;
                    let mut cdef = (*cdef).clone();
                    assign_field!(cdef.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: lst.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    c1 = countComponents(cdef.clone())?;
                    c2 = countComponentsInElts(metamodelica::AsArg::as_arg(&elt), 0);
                    Ok(c1 + c2)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cdef @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::ListNode::Cons { head: _, tail: lst }, ann, comment: cmt, .. }, .. } => {
                    let mut res: i32;
                    let mut cdef = (*cdef).clone();
                    assign_field!(cdef.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: lst.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    res = countComponents(cdef.clone())?;
                    Ok(res)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::ListNode::Nil, .. }, .. } => {
                    Ok(0)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
                    Ok(-1)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outInteger)
}

fn countComponentsInElts<'__b>(
    mut inAbsynElementItemLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inInteger: i32,
) -> i32 {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynElementItemLst {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: complst, .. }, .. } }, tail: lst } => {
                { (inAbsynElementItemLst, inInteger) = (lst, inInteger + ((complst).len() as i32)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: lst } => {
                { (inAbsynElementItemLst, inInteger) = (lst, inInteger); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return inInteger
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn getNthComponent(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
    mut n: i32,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut genv: GraphicEnvCache;
    let mut cdef: metamodelica::Ref<Absyn::Class>;
    match '__try0: {
        genv = unwrap_break_err!(InteractiveUtil::createEnvironment(SymbolTable::getAbsyn(), Some(unwrap_break_err!(SymbolTable::getSCode(), '__try0)), classPath.clone()), '__try0);
        cdef =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), program, false, false), '__try0);
        result = unwrap_break_err!(getNthComponent2(&cdef, n, genv.clone()), '__try0);
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    result
}

fn getNthComponent2(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut n: i32,
    mut genv: GraphicEnvCache,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut comp: metamodelica::Ref<Absyn::Element>;
    let mut comp_name: ArcStr;
    let mut cmt: ArcStr;
    let mut ty: metamodelica::Ref<Absyn::Path>;
    comp = InteractiveUtil::getNthComponentInClass(inClass, n)?;
    (comp_name, ty, cmt) = getComponentInfoOld(&comp, genv)?;
    result = metamodelica::Ref::new(Values::Value::ARRAY {
        valueLst: list![
            ValuesMake::makeCodeTypeName(ty),
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_VARIABLENAME {
                    componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                        name: comp_name,
                        subscripts: metamodelica::nil()
                    })
                })
            }),
            ValuesMake::makeString(cmt)
        ],
        dimLst: list![3],
    });
    Ok(result)
}

fn useQuotes<'__b>(mut inAbsynNamedArgLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynNamedArgLst {
            Deref @ metamodelica::ListNode::Nil => {
                return false
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "useQuotes", argValue: Deref @ Absyn::Exp::BOOL { value: b } }, tail: _ } => {
                return b.clone()
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: al } => {
                let mut res: bool;
                { inAbsynNamedArgLst = al; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn insertQuotesToList(mut inStringList: &metamodelica::List<ArcStr>) -> metamodelica::List<ArcStr> {
    let mut outStringList: metamodelica::List<ArcStr>;
    outStringList = (::match_deref::match_deref! { match inStringList {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: r#str, tail: rest } => {
            let mut res: metamodelica::List<ArcStr>;
            let mut str_1: ArcStr;
            str_1 = stringAppendList(list![literal!("\""), r#str.clone(), literal!("\"")]);
            res = insertQuotesToList(rest);
            metamodelica::cons(str_1, res)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStringList
}

pub(crate) fn getComponents(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut useQuotes: bool,
    mut program: Absyn::Program,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    result = getElements(classPath, useQuotes, program, true);
    result
}

pub(crate) fn getElements(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut useQuotes: bool,
    mut program: Absyn::Program,
    mut onlyComponents: bool,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut access: Access;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut env: GraphicEnvCache;
    let mut silent: bool = false;
    let mut infos: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
    match '__try0: {
        access = checkAccessAnnotationAndEncryption(classPath.clone(), program.clone());
        if access < Access::icon.clone() {
            unwrap_break_err!(Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil()), '__try0);
            result = ValuesMake::makeArray(metamodelica::nil());
            return result;
        }
        silent = !(unwrap_break_err!(Flags::isSet(Flags::NF_API_NOISE.clone()), '__try0));
        if silent {
            ErrorExt::setCheckpoint(literal!("Interactive.getElements"));
        }
        cls =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false), '__try0);
        env = unwrap_break_err!(InteractiveUtil::createEnvironment(program.clone(), Some(unwrap_break_err!(SymbolTable::getSCode(), '__try0)), classPath.clone()), '__try0);
        if access >= Access::diagram.clone() {
            elems = InteractiveUtil::getProtectedElementsInClass(&cls);
            infos = unwrap_break_err!(InteractiveUtil::getElementsInfo(elems.clone(), false, useQuotes, onlyComponents, env.clone(), metamodelica::nil()), '__try0);
        }
        elems = InteractiveUtil::getPublicElementsInClass(&cls);
        infos = unwrap_break_err!(InteractiveUtil::getElementsInfo(elems.clone(), true, useQuotes, onlyComponents, env.clone(), infos.clone()), '__try0);
        result = ValuesMake::makeArray(infos.clone());
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeArray(metamodelica::nil());
        }
    }
    if silent {
        ErrorExt::rollBack(literal!("Interactive.getElements"));
    }
    result
}

pub(crate) fn getComponentAnnotations(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    fn r#impl(
        mut classPath: metamodelica::Ref<Absyn::Path>,
        mut program: Absyn::Program,
        mut accessLevel: Access,
    ) -> Result<metamodelica::Ref<Values::Value>> {
        let mut result: metamodelica::Ref<Values::Value>;
        let mut cdef: metamodelica::Ref<Absyn::Class>;
        let mut comps: metamodelica::List<metamodelica::Ref<Absyn::Element>> = metamodelica::nil();
        cdef = ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false)?;
        if accessLevel >= Access::diagram.clone() {
            comps = InteractiveUtil::getProtectedComponentsInClass(&cdef);
        }
        comps = listAppend(InteractiveUtil::getPublicComponentsInClass(&cdef), comps);
        result = InteractiveUtil::getElementAnnotationsFromElts(comps, &cdef, program, classPath)?;
        Ok(result)
    }

    let mut result: metamodelica::Ref<Values::Value>;
    result = InteractiveUtil::accessClass(classPath, program, &r#impl, true, false, Access::icon.clone())?;
    Ok(result)
}

pub(crate) fn getElementAnnotations(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    fn r#impl(
        mut classPath: metamodelica::Ref<Absyn::Path>,
        mut program: Absyn::Program,
        mut accessLevel: Access,
    ) -> Result<metamodelica::Ref<Values::Value>> {
        let mut result: metamodelica::Ref<Values::Value>;
        let mut cdef: metamodelica::Ref<Absyn::Class>;
        let mut elts: metamodelica::List<metamodelica::Ref<Absyn::Element>> = metamodelica::nil();
        cdef = ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false)?;
        if accessLevel >= Access::diagram.clone() {
            elts = InteractiveUtil::getProtectedElementsInClass(&cdef);
        }
        elts = listAppend(InteractiveUtil::getPublicElementsInClass(&cdef), elts);
        result = InteractiveUtil::getElementAnnotationsFromElts(elts, &cdef, program, classPath)?;
        Ok(result)
    }

    let mut result: metamodelica::Ref<Values::Value>;
    result = InteractiveUtil::accessClass(classPath, program, &r#impl, true, false, Access::icon.clone())?;
    Ok(result)
}

pub(crate) fn getNthComponentAnnotation(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut n: i32,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    fn r#impl(
        mut classPath: metamodelica::Ref<Absyn::Path>,
        mut n: i32,
        mut program: Absyn::Program,
        mut accessLevel: Access,
    ) -> Result<metamodelica::Ref<Values::Value>> {
        let mut result: metamodelica::Ref<Values::Value>;
        let mut cdef: metamodelica::Ref<Absyn::Class>;
        let mut comp: metamodelica::Ref<Absyn::Element>;
        cdef = ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false)?;
        comp = InteractiveUtil::getNthComponentInClass(&cdef, n)?;
        result = InteractiveUtil::getElementAnnotationsFromElts(list![comp], &cdef, program, classPath)?;
        if ValuesUtil::isArray(&result) && ValuesUtil::arraySize(&result)? == 1 {
            result = ValuesUtil::arrayScalar(&result)?;
        }
        Ok(result)
    }

    let mut result: metamodelica::Ref<Values::Value>;
    result = InteractiveUtil::accessClass(
        classPath,
        program,
        &({
            let __pe_b1 = n;
            move |__pe_a0, __pe_a2, __pe_a3| r#impl(__pe_a0, __pe_b1.clone(), __pe_a2, __pe_a3)
        }),
        true,
        false,
        Access::icon.clone(),
    )?;
    Ok(result)
}

pub(crate) fn getNthComponentModification(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut n: i32,
    mut program: &Absyn::Program,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut comp: metamodelica::Ref<Absyn::Element>;
    match '__try0: {
        cls =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), program, false, false), '__try0);
        comp = unwrap_break_err!(InteractiveUtil::getNthComponentInClass(&cls, n), '__try0);
        result = unwrap_break_err!(getComponentModification(&comp), '__try0);
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    result
}

pub(crate) fn getNthComponentCondition(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut n: i32,
    mut program: &Absyn::Program,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut comp: metamodelica::Ref<Absyn::Element>;
    let mut r#str: ArcStr;
    match '__try0: {
        cls =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), program, false, false), '__try0);
        comp = unwrap_break_err!(InteractiveUtil::getNthComponentInClass(&cls, n), '__try0);
        r#str = getComponentCondition(&comp);
        r#str = System::trim(r#str.clone(), literal!(" "));
        result = ValuesMake::makeString(r#str.clone());
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    result
}

fn getComponentCondition(mut inElement: &metamodelica::Ref<Absyn::Element>) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = &**inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: lst, .. }, .. } => {
                    let mut r#str: ArcStr;
                    r#str = getComponentitemsCondition(metamodelica::AsArg::as_arg(&lst))?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

fn getComponentitemsCondition(
    mut inAbsynComponentItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inAbsynComponentItemLst {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { condition: cond, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut res: ArcStr;
            res = Dump::unparseComponentCondition(cond.clone())?;
            res
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

pub(crate) fn getNthConnection(
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut inProgram: Absyn::Program,
    mut inInteger: i32,
) -> metamodelica::List<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::List<metamodelica::Ref<Values::Value>>;
    outValue = 'mc: {
        let __mc_input = (inComponentRef, inProgram, inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (model_, p, n) => {
                    let mut modelpath: metamodelica::Ref<Absyn::Path>;
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut eq: metamodelica::Ref<Absyn::Equation>;
                    let mut cmt: Option<metamodelica::Ref<Absyn::Comment>>;
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut r#str: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    modelpath = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&model_))?;
                    cdef = ProgramUtil::getPathedClassInProgram(modelpath.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(((getConnections(&cdef))).get(n.clone())?) {
                        Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: __pa0, comment: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eq = metamodelica::Own::own(__pa0);
                    cmt = metamodelica::Own::own(__pa1);
                    r#str = getStringComment(cmt.clone());
                    (s1, s2) = getConnectionStr(&eq)?;
                    vals = list![metamodelica::Ref::new(Values::Value::STRING { string: s1.clone() }), metamodelica::Ref::new(Values::Value::STRING { string: s2.clone() }), metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() })];
                    Ok(vals.clone())
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
    outValue
}

pub(crate) fn getStringComment(mut inAbsynCommentOption: Option<metamodelica::Ref<Absyn::Comment>>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inAbsynCommentOption) {
        Some(Deref @ Absyn::Comment { annotation_: _, comment: Some(r#str) }) => {
            r#str.clone()
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outString
}

pub(crate) fn addConnection(
    mut classPath: &metamodelica::Ref<Absyn::Path>,
    mut connector1: metamodelica::Ref<Absyn::ComponentRef>,
    mut connector2: metamodelica::Ref<Absyn::ComponentRef>,
    mut commentExp: &metamodelica::Ref<Absyn::Exp>,
    mut annotationExp: &metamodelica::Ref<Absyn::Exp>,
    mut program: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool;
    let mut eq: metamodelica::Ref<Absyn::EquationItem>;
    let mut cmt: Option<metamodelica::Ref<Absyn::Comment>>;
    match '__try0: {
        cmt = unwrap_break_err!(InteractiveUtil::makeCommentFromArgs(commentExp, annotationExp, None), '__try0);
        eq = metamodelica::Ref::new(Absyn::EquationItem::EQUATIONITEM {
            equation_: metamodelica::Ref::new(Absyn::Equation::EQ_CONNECT {
                connector1: connector1.clone(),
                connector2: connector2.clone(),
            }),
            comment: cmt.clone(),
            info: Absyn::dummyInfo.clone(),
        });
        program = unwrap_break_err!(transformPathedClassInProgram(classPath, &program, (std::sync::Arc::new({ let __pe_b1 = eq.clone(); move |__pe_a0| InteractiveUtil::addToEquation(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static>)), '__try0);
        success = true;
        Ok::<_, &'static str>((success.clone(),))
    } {
        Ok((__try0_o0,)) => {
            success = __try0_o0;
        }
        Err(_) => {
            success = false;
        }
    }
    (program, success)
}

pub(crate) fn deleteConnection(
    mut classPath: &metamodelica::Ref<Absyn::Path>,
    mut connector1: &metamodelica::Ref<Absyn::ComponentRef>,
    mut connector2: &metamodelica::Ref<Absyn::ComponentRef>,
    mut program: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool;
    match '__try0: {
        program = unwrap_break_err!(transformPathedClassInProgram(classPath, &program, (std::sync::Arc::new({ let __pe_b1 = connector1.clone(); let __pe_b2 = connector2.clone(); move |__pe_a0| deleteConnectionInClass(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static>)), '__try0);
        success = true;
        Ok::<_, &'static str>((success.clone(),))
    } {
        Ok((__try0_o0,)) => {
            success = __try0_o0;
        }
        Err(_) => {
            success = false;
        }
    }
    (program, success)
}

fn deleteConnectionInClass(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut connector1: metamodelica::Ref<Absyn::ComponentRef>,
    mut connector2: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut eqlst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut cdef: metamodelica::Ref<Absyn::ClassDef>;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Absyn::Class { body: __esc_cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
            cdef = (*__esc_cdef).clone();
            eqlst = InteractiveUtil::getEquationList(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS))?;
            eqlst = deleteEquationInEqlist(eqlst, connector1, connector2)?;
            assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = InteractiveUtil::replaceEquationList(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS), eqlst)?);
            assign_field!(cls.body = cdef.clone());
            ()
        },
        Deref @ Absyn::Class { body: __esc_cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
            cdef = (*__esc_cdef).clone();
            eqlst = InteractiveUtil::getEquationList(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS))?;
            eqlst = deleteEquationInEqlist(eqlst, connector1, connector2)?;
            assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = InteractiveUtil::replaceEquationList(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS), eqlst)?);
            assign_field!(cls.body = cdef.clone());
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(cls)
}

fn deleteEquationInEqlist(
    mut inAbsynEquationItemLst1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inAbsynEquationItemLst1, inComponentRef2, inComponentRef3)) {
            (Deref @ metamodelica::ListNode::Nil, _, _) => {
                return Ok(metamodelica::nil())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_CONNECT { connector1: cn1, connector2: cn2 }, .. }, tail: xs }, c1, c2) if (AbsynUtil::crefEqual(metamodelica::AsArg::as_arg(&c1), metamodelica::AsArg::as_arg(&cn1))? && AbsynUtil::crefEqual(metamodelica::AsArg::as_arg(&c2), metamodelica::AsArg::as_arg(&cn2))?) => {
                { (inAbsynEquationItemLst1, inComponentRef2, inComponentRef3) = (xs.clone(), c1.clone(), c2.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_FOR { forEquations: forEqList, iterators: forIterator }, .. }, tail: xs }, c1, c2) => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                let mut loopRes: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                res = deleteEquationInEqlist(xs.clone(), c1.clone(), c2.clone())?;
                loopRes = deleteEquationInEqlist(forEqList.clone(), c1.clone(), c2.clone())?;
                if !((loopRes).is_empty()) {
                    loopRes = list![metamodelica::Ref::new(Absyn::EquationItem::EQUATIONITEM { equation_: metamodelica::Ref::new(Absyn::Equation::EQ_FOR { iterators: forIterator.clone(), forEquations: loopRes }), comment: None, info: Absyn::dummyInfo.clone() })];
                }
                return Ok(listAppend(loopRes, res))
            },
            (Deref @ metamodelica::ListNode::Cons { head: x, tail: xs }, c1, c2) => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                res = deleteEquationInEqlist(xs.clone(), c1.clone(), c2.clone())?;
                return Ok(metamodelica::cons(x.clone(), res))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn addTransition(
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut from: ArcStr,
    mut to: ArcStr,
    mut condition: ArcStr,
    mut immediate: bool,
    mut reset: bool,
    mut synchronize: bool,
    mut priority: i32,
    mut inAbsynNamedArgLst: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inProgram: Absyn::Program,
) -> Result<(bool, Absyn::Program)> {
    let mut b: bool;
    let mut outProgram: Absyn::Program;
    (b, outProgram) = addTransitionWithAnnotation(
        inComponentRef,
        from,
        to,
        condition,
        immediate,
        reset,
        synchronize,
        priority,
        InteractiveUtil::annotationListToAbsyn(inAbsynNamedArgLst)?,
        inProgram,
    )?;
    Ok((b, outProgram))
}

pub(crate) fn addTransitionWithAnnotation(
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut from: ArcStr,
    mut to: ArcStr,
    mut condition: ArcStr,
    mut immediate: bool,
    mut reset: bool,
    mut synchronize: bool,
    mut priority: i32,
    mut inAnnotation: metamodelica::Ref<Absyn::Annotation>,
    mut inProgram: Absyn::Program,
) -> Result<(bool, Absyn::Program)> {
    let mut b: bool;
    let mut outProgram: Absyn::Program;
    (b, outProgram) = (::match_deref::match_deref! { match &((inComponentRef, inProgram)) {
        (model_ @ Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, p @ Absyn::Program { .. }) => {
            let mut from_ = from;
            let mut to_ = to;
            let mut condition_ = condition;
            let mut immediate_ = immediate;
            let mut reset_ = reset;
            let mut synchronize_ = synchronize;
            let mut priority_ = priority;
            let mut ann = inAnnotation;
            let mut modelpath: metamodelica::Ref<Absyn::Path>;
            let mut cdef: metamodelica::Ref<Absyn::Class>;
            let mut newcdef: metamodelica::Ref<Absyn::Class>;
            let mut newp: Absyn::Program;
            let mut cmt: Option<metamodelica::Ref<Absyn::Comment>>;
            let mut conditionExp: metamodelica::Ref<Absyn::Exp>;
            modelpath = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&model_))?;
            cdef = ProgramUtil::getPathedClassInProgram(modelpath, metamodelica::AsArg::as_arg(&p), false, false)?;
            cmt = Some(metamodelica::Ref::new(Absyn::Comment { annotation_: Some(ann), comment: None }));
            let __pa0 = ::match_deref::match_deref! { match &(Parser::parsestringexp(condition_, literal!("<interactive>"))?) {
                GlobalScript::Statements { interactiveStmtLst: Deref @ metamodelica::ListNode::Cons { head: GlobalScript::Statement::IEXP { exp: __pa0, info: _ }, tail: Deref @ metamodelica::ListNode::Nil }, semicolon: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            conditionExp = metamodelica::Own::own(__pa0);
            newcdef = InteractiveUtil::addToEquation(cdef, metamodelica::Ref::new(Absyn::EquationItem::EQUATIONITEM { equation_: metamodelica::Ref::new(Absyn::Equation::EQ_NORETCALL { functionName: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("transition"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: from_, subscripts: metamodelica::nil() }) }), metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: to_, subscripts: metamodelica::nil() }) }), conditionExp], argNames: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("immediate"), argValue: metamodelica::Ref::new(Absyn::Exp::BOOL { value: immediate_ }) }), metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("reset"), argValue: metamodelica::Ref::new(Absyn::Exp::BOOL { value: reset_ }) }), metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("synchronize"), argValue: metamodelica::Ref::new(Absyn::Exp::BOOL { value: synchronize_ }) }), metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("priority"), argValue: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: priority_ }) })] }) }), comment: cmt, info: Absyn::dummyInfo.clone() }))?;
            newp = ProgramUtil::updateProgram(Absyn::Program { classes: list![newcdef], within_: p.within_.clone() }, p.clone(), false, false)?;
            (true, newp)
        },
        (model_ @ Deref @ Absyn::ComponentRef::CREF_QUAL { .. }, p @ Absyn::Program { .. }) => {
            let mut from_ = from;
            let mut to_ = to;
            let mut condition_ = condition;
            let mut immediate_ = immediate;
            let mut reset_ = reset;
            let mut synchronize_ = synchronize;
            let mut priority_ = priority;
            let mut ann = inAnnotation;
            let mut modelpath: metamodelica::Ref<Absyn::Path>;
            let mut package_: metamodelica::Ref<Absyn::Path>;
            let mut cdef: metamodelica::Ref<Absyn::Class>;
            let mut newcdef: metamodelica::Ref<Absyn::Class>;
            let mut newp: Absyn::Program;
            let mut cmt: Option<metamodelica::Ref<Absyn::Comment>>;
            let mut conditionExp: metamodelica::Ref<Absyn::Exp>;
            modelpath = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&model_))?;
            cdef = ProgramUtil::getPathedClassInProgram(modelpath.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
            package_ = AbsynUtil::stripLast(&modelpath)?;
            cmt = Some(metamodelica::Ref::new(Absyn::Comment { annotation_: Some(ann), comment: None }));
            let __pa0 = ::match_deref::match_deref! { match &(Parser::parsestringexp(condition_, literal!("<interactive>"))?) {
                GlobalScript::Statements { interactiveStmtLst: Deref @ metamodelica::ListNode::Cons { head: GlobalScript::Statement::IEXP { exp: __pa0, info: _ }, tail: Deref @ metamodelica::ListNode::Nil }, semicolon: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            conditionExp = metamodelica::Own::own(__pa0);
            newcdef = InteractiveUtil::addToEquation(cdef, metamodelica::Ref::new(Absyn::EquationItem::EQUATIONITEM { equation_: metamodelica::Ref::new(Absyn::Equation::EQ_NORETCALL { functionName: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("transition"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: from_, subscripts: metamodelica::nil() }) }), metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: to_, subscripts: metamodelica::nil() }) }), conditionExp], argNames: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("immediate"), argValue: metamodelica::Ref::new(Absyn::Exp::BOOL { value: immediate_ }) }), metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("reset"), argValue: metamodelica::Ref::new(Absyn::Exp::BOOL { value: reset_ }) }), metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("synchronize"), argValue: metamodelica::Ref::new(Absyn::Exp::BOOL { value: synchronize_ }) }), metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("priority"), argValue: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: priority_ }) })] }) }), comment: cmt, info: Absyn::dummyInfo.clone() }))?;
            newp = ProgramUtil::updateProgram(Absyn::Program { classes: list![newcdef], within_: Absyn::Within::WITHIN { path: package_ } }, p.clone(), false, false)?;
            (true, newp)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((b, outProgram))
}

pub(crate) fn deleteTransition(
    mut inComponentRef1: metamodelica::Ref<Absyn::ComponentRef>,
    mut from: ArcStr,
    mut to: ArcStr,
    mut condition: ArcStr,
    mut immediate: bool,
    mut reset: bool,
    mut synchronize: bool,
    mut priority: i32,
    mut inProgram: Absyn::Program,
) -> Result<(bool, Absyn::Program)> {
    let mut b: bool;
    let mut outProgram: Absyn::Program;
    (b, outProgram) = 'mc: {
        let __mc_input = (
            inComponentRef1,
            from,
            to,
            condition,
            immediate,
            reset,
            synchronize,
            priority,
            inProgram,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (model_, from_, to_, condition_, immediate_, reset_, synchronize_, priority_, p @ Absyn::Program { .. }) => {
                    let mut modelpath: metamodelica::Ref<Absyn::Path>;
                    let mut modelwithin: metamodelica::Ref<Absyn::Path>;
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut newcdef: metamodelica::Ref<Absyn::Class>;
                    let mut newp: Absyn::Program;
                    modelpath = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&model_))?;
                    modelwithin = AbsynUtil::stripLast(&modelpath)?;
                    cdef = ProgramUtil::getPathedClassInProgram(modelpath.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    newcdef = deleteTransitionInClass(cdef.clone(), from_.clone(), to_.clone(), condition_.clone(), immediate_.clone(), reset_.clone(), synchronize_.clone(), priority_.clone())?;
                    newp = ProgramUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: Absyn::Within::WITHIN { path: modelwithin.clone() } }, p.clone(), false, false)?;
                    Ok((true, newp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (model_, from_, to_, condition_, immediate_, reset_, synchronize_, priority_, p @ Absyn::Program { .. }) => {
                    let mut modelpath: metamodelica::Ref<Absyn::Path>;
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut newcdef: metamodelica::Ref<Absyn::Class>;
                    let mut newp: Absyn::Program;
                    modelpath = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&model_))?;
                    cdef = ProgramUtil::getPathedClassInProgram(modelpath.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    newcdef = deleteTransitionInClass(cdef.clone(), from_.clone(), to_.clone(), condition_.clone(), immediate_.clone(), reset_.clone(), synchronize_.clone(), priority_.clone())?;
                    newp = ProgramUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, p.clone(), false, false)?;
                    Ok((true, newp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, p @ Absyn::Program { .. }) => {
                    Ok((false, p.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((b, outProgram))
}

fn deleteTransitionInClass(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut from: ArcStr,
    mut to: ArcStr,
    mut condition: ArcStr,
    mut immediate: bool,
    mut reset: bool,
    mut synchronize: bool,
    mut priority: i32,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    outClass = (::match_deref::match_deref! { match &(inClass) {
        __esc_outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmt }, info: _, .. } => {
            let mut from_ = from;
            let mut to_ = to;
            let mut condition_ = condition;
            let mut immediate_ = immediate;
            let mut reset_ = reset;
            let mut synchronize_ = synchronize;
            let mut priority_ = priority;
            outClass = (*__esc_outClass).clone();
            let mut eqlst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqlst_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            eqlst = InteractiveUtil::getEquationList(metamodelica::AsArg::as_arg(&parts))?;
            eqlst_1 = deleteTransitionInEqlist(&eqlst, from_, to_, condition_, immediate_, reset_, synchronize_, priority_)?;
            parts2 = InteractiveUtil::replaceEquationList(metamodelica::AsArg::as_arg(&parts), eqlst_1)?;
            assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2, ann: ann.clone(), comment: cmt.clone() }));
            outClass.clone()
        },
        __esc_outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname, modifications: modif, parts, ann, comment: cmt }, .. } => {
            let mut from_ = from;
            let mut to_ = to;
            let mut condition_ = condition;
            let mut immediate_ = immediate;
            let mut reset_ = reset;
            let mut synchronize_ = synchronize;
            let mut priority_ = priority;
            outClass = (*__esc_outClass).clone();
            let mut eqlst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqlst_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            eqlst = InteractiveUtil::getEquationList(metamodelica::AsArg::as_arg(&parts))?;
            eqlst_1 = deleteTransitionInEqlist(&eqlst, from_, to_, condition_, immediate_, reset_, synchronize_, priority_)?;
            parts2 = InteractiveUtil::replaceEquationList(metamodelica::AsArg::as_arg(&parts), eqlst_1)?;
            assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname.clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2, ann: ann.clone() }));
            outClass.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outClass)
}

fn deleteTransitionInEqlist(
    mut inAbsynEquationItemLst: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut from: ArcStr,
    mut to: ArcStr,
    mut condition: ArcStr,
    mut immediate: bool,
    mut reset: bool,
    mut synchronize: bool,
    mut priority: i32,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>> {
    let mut outAbsynEquationItemLst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    outAbsynEquationItemLst = 'mc: {
        let __mc_input = (
            &**inAbsynEquationItemLst,
            from,
            to,
            condition,
            immediate,
            reset,
            synchronize,
            priority,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _, _, _, _, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_NORETCALL { functionName: name, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: expArgs, argNames: namedArgs } }, .. }, tail: xs }, from_, to_, condition_, immediate_, reset_, synchronize_, priority_) => {
                    if !((AbsynUtil::crefEqual(metamodelica::AsArg::as_arg(&name), &(metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("transition"), subscripts: metamodelica::nil() })))?)) { return Err("guard") }
                    let mut args: metamodelica::List<ArcStr>;
                    let mut conditionExp: metamodelica::Ref<Absyn::Exp>;
                    let mut condition_ = (*condition_).clone();
                    args = List::map(expArgs.clone(), &Dump::printExpStr)?;
                    args = addOrUpdateNamedArg(metamodelica::AsArg::as_arg(&namedArgs), &(literal!("immediate")), &(literal!("true")), args.clone(), 4)?;
                    args = addOrUpdateNamedArg(metamodelica::AsArg::as_arg(&namedArgs), &(literal!("reset")), &(literal!("true")), args.clone(), 5)?;
                    args = addOrUpdateNamedArg(metamodelica::AsArg::as_arg(&namedArgs), &(literal!("synchronize")), &(literal!("false")), args.clone(), 6)?;
                    args = addOrUpdateNamedArg(metamodelica::AsArg::as_arg(&namedArgs), &(literal!("priority")), &(literal!("1")), args.clone(), 7)?;
                    let __pa0 = ::match_deref::match_deref! { match &(Parser::parsestringexp(condition_.clone(), literal!("<interactive>"))?) {
                        GlobalScript::Statements { interactiveStmtLst: Deref @ metamodelica::ListNode::Cons { head: GlobalScript::Statement::IEXP { exp: __pa0, info: _ }, tail: Deref @ metamodelica::ListNode::Nil }, semicolon: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    conditionExp = metamodelica::Own::own(__pa0);
                    condition_ = Dump::printExpStr(conditionExp.clone())?;
                    let true = (compareTransitionFuncArgs(&args, from_.clone(), to_.clone(), condition_.clone(), immediate_.clone(), reset_.clone(), synchronize_.clone(), priority_.clone())) else { return Err("pattern mismatch") };
                    Ok(deleteTransitionInEqlist(metamodelica::AsArg::as_arg(&xs), from_.clone(), to_.clone(), condition_.clone(), immediate_.clone(), reset_.clone(), synchronize_.clone(), priority_.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: x, tail: xs }, from_, to_, condition_, immediate_, reset_, synchronize_, priority_) => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    res = deleteTransitionInEqlist(metamodelica::AsArg::as_arg(&xs), from_.clone(), to_.clone(), condition_.clone(), immediate_.clone(), reset_.clone(), synchronize_.clone(), priority_.clone())?;
                    Ok(metamodelica::cons(x.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAbsynEquationItemLst)
}

pub(crate) fn addOrUpdateNamedArg(
    mut inNamedArgLst: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut namedArg: &ArcStr,
    mut defaultValue: &ArcStr,
    mut inTransition: metamodelica::List<ArcStr>,
    mut position: i32,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outTransition: metamodelica::List<ArcStr>;
    let mut namedArgValue: ArcStr;
    let mut isDefault: bool;
    (namedArgValue, isDefault) = namedArgValueAsString(inNamedArgLst, namedArg, defaultValue)?;
    if ((inTransition).len() as i32) < position {
        outTransition = List::insert(inTransition, position, namedArgValue)?;
    } else if boolAnd(((inTransition).len() as i32) >= position, boolNot(isDefault)) {
        outTransition = List::replaceAt(namedArgValue, position, inTransition)?;
    } else {
        outTransition = inTransition;
    }
    Ok(outTransition)
}

fn namedArgValueAsString<'__b>(
    mut inAbsynNamedArgLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inNamedArg: &'__b ArcStr,
    mut inDefaultValue: &'__b ArcStr,
) -> Result<(ArcStr, bool)> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynNamedArgLst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((inDefaultValue.clone(), true))
            },
            Deref @ metamodelica::ListNode::Cons { head: namedArg @ Deref @ Absyn::NamedArg { argName: namedArgName, .. }, tail: _ } if (stringEq(&namedArgName, &inNamedArg)) => {
                return Ok((Dump::printNamedArgValueStr(metamodelica::AsArg::as_arg(&namedArg))?, false))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: al } => {
                { (inAbsynNamedArgLst, inNamedArg, inDefaultValue) = (al, inNamedArg, inDefaultValue); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn compareTransitionFuncArgs(
    mut args: &metamodelica::List<ArcStr>,
    mut from: ArcStr,
    mut to: ArcStr,
    mut condition: ArcStr,
    mut immediate: bool,
    mut reset: bool,
    mut synchronize: bool,
    mut priority: i32,
) -> bool {
    let mut b: bool;
    b = 'mc: {
        let __mc_input = (&**args, from, to, condition, immediate, reset, synchronize, priority);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: from1, tail: Deref @ metamodelica::ListNode::Cons { head: to1, tail: Deref @ metamodelica::ListNode::Cons { head: condition1, tail: Deref @ metamodelica::ListNode::Nil } } }, from2, to2, condition2, _, _, _, _) => {
                    if !((stringEq(&from1, &from2) && stringEq(&to1, &to2) && stringEq(&condition1, &condition2))) { return Err("guard") }
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: from1, tail: Deref @ metamodelica::ListNode::Cons { head: to1, tail: Deref @ metamodelica::ListNode::Cons { head: condition1, tail: Deref @ metamodelica::ListNode::Cons { head: immediate1, tail: Deref @ metamodelica::ListNode::Nil } } } }, from2, to2, condition2, immediate2, _, _, _) => {
                    if !((stringEq(&from1, &from2) && stringEq(&to1, &to2) && stringEq(&condition1, &condition2) && stringEq(&immediate1, &(boolString(immediate2.clone()))))) { return Err("guard") }
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: from1, tail: Deref @ metamodelica::ListNode::Cons { head: to1, tail: Deref @ metamodelica::ListNode::Cons { head: condition1, tail: Deref @ metamodelica::ListNode::Cons { head: immediate1, tail: Deref @ metamodelica::ListNode::Cons { head: reset1, tail: Deref @ metamodelica::ListNode::Nil } } } } }, from2, to2, condition2, immediate2, reset2, _, _) => {
                    if !((stringEq(&from1, &from2) && stringEq(&to1, &to2) && stringEq(&condition1, &condition2) && stringEq(&immediate1, &(boolString(immediate2.clone()))) && stringEq(&reset1, &(boolString(reset2.clone()))))) { return Err("guard") }
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: from1, tail: Deref @ metamodelica::ListNode::Cons { head: to1, tail: Deref @ metamodelica::ListNode::Cons { head: condition1, tail: Deref @ metamodelica::ListNode::Cons { head: immediate1, tail: Deref @ metamodelica::ListNode::Cons { head: reset1, tail: Deref @ metamodelica::ListNode::Cons { head: synchronize1, tail: Deref @ metamodelica::ListNode::Nil } } } } } }, from2, to2, condition2, immediate2, reset2, synchronize2, _) => {
                    if !((stringEq(&from1, &from2) && stringEq(&to1, &to2) && stringEq(&condition1, &condition2) && stringEq(&immediate1, &(boolString(immediate2.clone()))) && stringEq(&reset1, &(boolString(reset2.clone()))) && stringEq(&synchronize1, &(boolString(synchronize2.clone()))))) { return Err("guard") }
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: from1, tail: Deref @ metamodelica::ListNode::Cons { head: to1, tail: Deref @ metamodelica::ListNode::Cons { head: condition1, tail: Deref @ metamodelica::ListNode::Cons { head: immediate1, tail: Deref @ metamodelica::ListNode::Cons { head: reset1, tail: Deref @ metamodelica::ListNode::Cons { head: synchronize1, tail: Deref @ metamodelica::ListNode::Cons { head: priority1, tail: Deref @ metamodelica::ListNode::Nil } } } } } } }, from2, to2, condition2, immediate2, reset2, synchronize2, priority2) => {
                    if !((stringEq(&from1, &from2) && stringEq(&to1, &to2) && stringEq(&condition1, &condition2) && stringEq(&immediate1, &(boolString(immediate2.clone()))) && stringEq(&reset1, &(boolString(reset2.clone()))) && stringEq(&synchronize1, &(boolString(synchronize2.clone()))) && stringEq(&priority1, &(intString(priority2.clone()))))) { return Err("guard") }
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
    b
}

pub(crate) fn getComponentComment(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut componentName: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut comment: metamodelica::Ref<Values::Value>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut elem: metamodelica::Ref<Absyn::Element>;
    let mut comps: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    let mut comp: metamodelica::Ref<Absyn::ComponentItem>;
    let mut cmt: ArcStr;
    let mut comp_name: ArcStr;
    path = AbsynUtil::joinPaths(classPath, componentName.clone())?;
    comp_name = AbsynUtil::pathLastIdent(&componentName);
    elem = InteractiveUtil::getPathedElementInProgram(path, program)?;
    comps = AbsynUtil::getComponentItemsFromElement(&elem);
    comp = List::find(
        &comps,
        &({
            let __pe_b0 = comp_name;
            move |__pe_a1| Ok(AbsynUtil::isComponentItemNamed(&__pe_b0, &__pe_a1))
        }),
    )?;
    cmt = InteractiveUtil::getClassCommentInCommentOpt(comp.comment.clone());
    comment = ValuesMake::makeString(cmt);
    Ok(comment)
}

pub(crate) fn setComponentComment(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut componentName: metamodelica::Ref<Absyn::Path>,
    mut comment: &ArcStr,
    mut program: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut comp_name: ArcStr;
    match '__try0: {
        path = unwrap_break_err!(AbsynUtil::joinPaths(classPath.clone(), componentName.clone()), '__try0);
        comp_name = AbsynUtil::pathLastIdent(&componentName);
        (program, _, success) = unwrap_break_err!(InteractiveUtil::transformPathedElementInProgram(&path, (std::sync::Arc::new({ let __pe_b1 = comp_name.clone(); let __pe_b2 = comment.clone(); move |__pe_a0| setComponentCommentInElement(__pe_a0, &__pe_b1, &__pe_b2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static>), program.clone()), '__try0);
        Ok::<_, &'static str>((success.clone(),))
    } {
        Ok((__try0_o0,)) => {
            success = __try0_o0;
        }
        Err(_) => {
            success = false;
        }
    }
    (program, success)
}

fn setComponentCommentInElement(
    mut element: metamodelica::Ref<Absyn::Element>,
    mut componentName: &ArcStr,
    mut comment: &ArcStr,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    fn set_comment(
        mut item: metamodelica::Ref<Absyn::ComponentItem>,
        mut comment: ArcStr,
    ) -> Result<metamodelica::Ref<Absyn::ComponentItem>> {
        let mut item: metamodelica::Ref<Absyn::ComponentItem> = item;
        assign_field!(
            item.comment = AbsynUtil::setCommentString(
                item.comment.clone(),
                if (stringEmpty(&comment)) { None } else { Some(comment) }
            )?
        );
        Ok(item)
    }

    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let mut spec: metamodelica::Ref<Absyn::ElementSpec>;
    let mut comps: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: __esc_spec @ Deref @ Absyn::ElementSpec::COMPONENTS { .. }, .. } => {
            spec = (*__esc_spec).clone();
            let __pa0 = ::match_deref::match_deref! { match &(List::findAndMap(var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone(), &({ let __pe_b0 = componentName.clone(); move |__pe_a1| Ok(AbsynUtil::isComponentItemNamed(&__pe_b0, &__pe_a1)) }), &({ let __pe_b1 = comment.clone(); move |__pe_a0| set_comment(__pe_a0, __pe_b1.clone()) }))?) {
                (__pa0, true) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            comps = metamodelica::Own::own(__pa0);
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; components = comps);
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = spec.clone());
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(element)
}

pub(crate) fn setConnectionComment(
    mut classPath: &metamodelica::Ref<Absyn::Path>,
    mut connector1: &metamodelica::Ref<Absyn::ComponentRef>,
    mut connector2: &metamodelica::Ref<Absyn::ComponentRef>,
    mut comment: &ArcStr,
    mut program: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool;
    match '__try0: {
        (program, _, success) = unwrap_break_err!(InteractiveUtil::transformPathedElementInProgram(classPath, (std::sync::Arc::new({ let __pe_b1 = connector1.clone(); let __pe_b2 = connector2.clone(); let __pe_b3 = comment.clone(); move |__pe_a0| setConnectionCommentInElement(__pe_a0, &__pe_b1, &__pe_b2, &__pe_b3) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static>), program.clone()), '__try0);
        Ok::<_, &'static str>((success.clone(),))
    } {
        Ok((__try0_o0,)) => {
            success = __try0_o0;
        }
        Err(_) => {
            success = false;
        }
    }
    (program, success)
}

fn setConnectionCommentInElement(
    mut element: metamodelica::Ref<Absyn::Element>,
    mut connector1: &metamodelica::Ref<Absyn::ComponentRef>,
    mut connector2: &metamodelica::Ref<Absyn::ComponentRef>,
    mut comment: &ArcStr,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let mut spec: metamodelica::Ref<Absyn::ElementSpec>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: __esc_spec @ Deref @ Absyn::ElementSpec::CLASSDEF { .. }, .. } => {
            spec = (*__esc_spec).clone();
            cls = setConnectionCommentInClass(var_field!((*spec).class_, Absyn::ElementSpec::CLASSDEF).clone(), connector1, connector2, comment)?;
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = cls);
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = spec.clone());
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(element)
}

fn setConnectionCommentInClass(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut connector1: &metamodelica::Ref<Absyn::ComponentRef>,
    mut connector2: &metamodelica::Ref<Absyn::ComponentRef>,
    mut comment: &ArcStr,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
            let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            let mut cdef = (*cdef).clone();
            parts = setConnectionCommentInParts(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone(), connector1, connector2, comment)?;
            assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = parts);
            assign_field!(cls.body = cdef.clone());
            ()
        },
        Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
            let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            let mut cdef = (*cdef).clone();
            parts = setConnectionCommentInParts(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), connector1, connector2, comment)?;
            assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = parts);
            assign_field!(cls.body = cdef.clone());
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(cls)
}

fn setConnectionCommentInParts(
    mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut connector1: &metamodelica::Ref<Absyn::ComponentRef>,
    mut connector2: &metamodelica::Ref<Absyn::ComponentRef>,
    mut comment: &ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = parts;
    let __pa0 = ::match_deref::match_deref! { match &(List::findMap(parts, &({ let __pe_b1 = connector1.clone(); let __pe_b2 = connector2.clone(); let __pe_b3 = comment.clone(); move |__pe_a0| setConnectionCommentInEquationsPart(__pe_a0, &__pe_b1, &__pe_b2, &__pe_b3) }))?) {
        (__pa0, true) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    parts = metamodelica::Own::own(__pa0);
    Ok(parts)
}

fn setConnectionCommentInEquationsPart(
    mut part: metamodelica::Ref<Absyn::ClassPart>,
    mut connector1: &metamodelica::Ref<Absyn::ComponentRef>,
    mut connector2: &metamodelica::Ref<Absyn::ComponentRef>,
    mut comment: &ArcStr,
) -> Result<(metamodelica::Ref<Absyn::ClassPart>, bool)> {
    let mut part: metamodelica::Ref<Absyn::ClassPart> = part;
    let mut found: bool;
    let mut eql: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    (part, found) = (match &*part {
        Absyn::ClassPart::EQUATIONS {
            contents: __part_contents,
        } => {
            (eql, found) = List::findMap(
                __part_contents.clone(),
                &({
                    let __pe_b1 = connector1.clone();
                    let __pe_b2 = connector2.clone();
                    let __pe_b3 = comment.clone();
                    move |__pe_a0| setConnectionCommentInEquation(__pe_a0, &__pe_b1, &__pe_b2, __pe_b3.clone())
                }),
            )?;
            assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = eql);
            (part, found)
        }
        _ => (part, false),
    });
    Ok((part, found))
}

fn setConnectionCommentInEquation(
    mut eq: metamodelica::Ref<Absyn::EquationItem>,
    mut connector1: &metamodelica::Ref<Absyn::ComponentRef>,
    mut connector2: &metamodelica::Ref<Absyn::ComponentRef>,
    mut comment: ArcStr,
) -> Result<(metamodelica::Ref<Absyn::EquationItem>, bool)> {
    let mut eq: metamodelica::Ref<Absyn::EquationItem> = eq;
    let mut success: bool;
    let mut c1: metamodelica::Ref<Absyn::ComponentRef>;
    let mut c2: metamodelica::Ref<Absyn::ComponentRef>;
    success = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_CONNECT { connector1: c1, connector2: c2 }, comment: __eq_comment, .. } if (AbsynUtil::crefEqual(connector1, metamodelica::AsArg::as_arg(&c1))? && AbsynUtil::crefEqual(connector2, metamodelica::AsArg::as_arg(&c2))?) => {
            assign_variant_field!(eq => Absyn::EquationItem::EQUATIONITEM; comment = AbsynUtil::setCommentString(__eq_comment.clone(), if (stringEmpty(&comment)) {None} else {Some(comment)})?);
            true
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eq, success))
}

pub(crate) fn getNthConnectionAnnotation(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut n: i32,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    fn r#impl(
        mut classPath: metamodelica::Ref<Absyn::Path>,
        mut n: i32,
        mut program: &Absyn::Program,
        mut accessLevel: Access,
    ) -> Result<metamodelica::Ref<Values::Value>> {
        let mut result: metamodelica::Ref<Values::Value>;
        let mut cdef: metamodelica::Ref<Absyn::Class>;
        let mut conn: metamodelica::Ref<Absyn::EquationItem>;
        cdef = ProgramUtil::getPathedClassInProgram(classPath.clone(), program, false, false)?;
        conn = (getConnections(&cdef)).get(n)?;
        result = getConnectionAnnotationStr(&conn, &cdef, program, &classPath)?;
        Ok(result)
    }

    let mut result: metamodelica::Ref<Values::Value>;
    result = InteractiveUtil::accessClass(
        classPath,
        program,
        &({
            let __pe_b1 = n;
            move |__pe_a0, __pe_a2, __pe_a3| r#impl(__pe_a0, __pe_b1.clone(), &__pe_a2, __pe_a3)
        }),
        true,
        false,
        Access::diagram.clone(),
    )?;
    Ok(result)
}

pub(crate) fn getConnectorCount(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut cdef: metamodelica::Ref<Absyn::Class>;
    match '__try0: {
        cdef =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), program, false, false), '__try0);
        result = ValuesMake::makeInteger(
            unwrap_break_err!(countPublicConnectors(&classPath, program, cdef.clone()), '__try0),
        );
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    result
}

pub(crate) fn getNthConnector(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut n: i32,
    mut program: Absyn::Program,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut name: ArcStr;
    let mut ty: metamodelica::Ref<Absyn::Path>;
    match '__try0: {
        cls =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false), '__try0);
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(getNthPublicConnectorStr(classPath.clone(), &cls, program.clone(), n), '__try0)) {
            (Some((__pa1, __pa2)), _) => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        name = metamodelica::Own::own(__pa1);
        ty = metamodelica::Own::own(__pa2);
        result = ValuesMake::makeCodeTypeNameArray(list![
            metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }),
            ty.clone()
        ]);
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    result
}

pub(crate) fn getNthConnectorIconAnnotation(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut n: i32,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    fn r#impl(
        mut classPath: metamodelica::Ref<Absyn::Path>,
        mut n: i32,
        mut program: Absyn::Program,
        mut accessLevel: Access,
    ) -> Result<metamodelica::Ref<Values::Value>> {
        let mut result: metamodelica::Ref<Values::Value>;
        let mut cls: metamodelica::Ref<Absyn::Class>;
        let mut ty: metamodelica::Ref<Absyn::Path>;
        cls = ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false)?;
        let __pa0 = ::match_deref::match_deref! { match &(getNthPublicConnectorStr(classPath, &cls, program.clone(), n)?) {
            (Some((_, __pa0)), _) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        ty = metamodelica::Own::own(__pa0);
        result = getIconAnnotation(ty, program)?;
        Ok(result)
    }

    let mut result: metamodelica::Ref<Values::Value>;
    result = InteractiveUtil::accessClass(
        classPath,
        program,
        &({
            let __pe_b1 = n;
            move |__pe_a0, __pe_a2, __pe_a3| r#impl(__pe_a0, __pe_b1.clone(), __pe_a2, __pe_a3)
        }),
        true,
        false,
        Access::icon.clone(),
    )?;
    Ok(result)
}

pub(crate) fn getIconAnnotation(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    fn r#impl(
        mut classPath: metamodelica::Ref<Absyn::Path>,
        mut program: Absyn::Program,
        mut accessLevel: Access,
    ) -> Result<metamodelica::Ref<Values::Value>> {
        let mut result: metamodelica::Ref<Values::Value>;
        result = getNamedAnnotationValue(classPath, program, literal!("Icon"))?;
        Ok(result)
    }

    let mut result: metamodelica::Ref<Values::Value>;
    result = InteractiveUtil::accessClass(classPath, program, &r#impl, true, true, Access::icon.clone())?;
    Ok(result)
}

pub(crate) fn refactorIconAnnotation(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    fn r#impl(
        mut classPath: metamodelica::Ref<Absyn::Path>,
        mut program: Absyn::Program,
        mut accessLevel: Access,
    ) -> Result<metamodelica::Ref<Values::Value>> {
        let mut result: metamodelica::Ref<Values::Value>;
        let mut cls: metamodelica::Ref<Absyn::Class>;
        cls = ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false)?;
        cls = Refactor::refactorGraphicalAnnotation(program.clone(), cls)?;
        result = getNamedAnnotationValue(classPath, program, literal!("Icon"))?;
        Ok(result)
    }

    let mut result: metamodelica::Ref<Values::Value>;
    result = InteractiveUtil::accessClass(classPath, program, &r#impl, true, true, Access::icon.clone())?;
    Ok(result)
}

pub(crate) fn getDiagramAnnotation(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    fn r#impl(
        mut classPath: metamodelica::Ref<Absyn::Path>,
        mut program: Absyn::Program,
        mut accessLevel: Access,
    ) -> Result<metamodelica::Ref<Values::Value>> {
        let mut result: metamodelica::Ref<Values::Value>;
        result = getNamedAnnotationValue(classPath, program, literal!("Diagram"))?;
        Ok(result)
    }

    let mut result: metamodelica::Ref<Values::Value>;
    result = InteractiveUtil::accessClass(classPath, program, &r#impl, true, true, Access::icon.clone())?;
    Ok(result)
}

pub(crate) fn refactorDiagramAnnotation(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    fn r#impl(
        mut classPath: metamodelica::Ref<Absyn::Path>,
        mut program: Absyn::Program,
        mut accessLevel: Access,
    ) -> Result<metamodelica::Ref<Values::Value>> {
        let mut result: metamodelica::Ref<Values::Value>;
        let mut cls: metamodelica::Ref<Absyn::Class>;
        cls = ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false)?;
        cls = Refactor::refactorGraphicalAnnotation(program.clone(), cls)?;
        result = getNamedAnnotationValue(classPath, program, literal!("Diagram"))?;
        Ok(result)
    }

    let mut result: metamodelica::Ref<Values::Value>;
    result = InteractiveUtil::accessClass(classPath, program, &r#impl, true, true, Access::icon.clone())?;
    Ok(result)
}

pub(crate) fn getNamedAnnotation(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut annotationPath: &metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    fn r#impl(
        mut classPath: metamodelica::Ref<Absyn::Path>,
        mut annotationPath: &metamodelica::Ref<Absyn::Path>,
        mut program: Absyn::Program,
        mut accessLevel: Access,
    ) -> Result<metamodelica::Ref<Values::Value>> {
        let mut result: metamodelica::Ref<Values::Value>;
        let mut r#str: ArcStr;
        r#str = ProgramUtil::getNamedAnnotationExp(
            classPath,
            program,
            annotationPath,
            Some(literal!("{}")),
            &fnptr!(getAnnotationValue, Option<metamodelica::Ref<Absyn::Modification>>),
        )?;
        result = ValuesMake::makeCodeTypeNameStr(r#str);
        Ok(result)
    }

    let mut result: metamodelica::Ref<Values::Value>;
    result = InteractiveUtil::accessClass(
        classPath,
        program,
        &({
            let __pe_b1 = annotationPath.clone();
            move |__pe_a0, __pe_a2, __pe_a3| r#impl(__pe_a0, &__pe_b1, __pe_a2, __pe_a3)
        }),
        true,
        false,
        Access::icon.clone(),
    )?;
    Ok(result)
}

pub(crate) fn getStringNamedAnnotation(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
    mut id: &metamodelica::Ref<Absyn::Path>,
) -> ArcStr {
    let mut outString: ArcStr;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(ProgramUtil::getNamedAnnotationExp(inPath.clone(), inProgram.clone(), id, Some(metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("") })), &getAnnotationExp), '__try0)) {
            Deref @ Absyn::Exp::STRING { value: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        outString = metamodelica::Own::own(__pa1);
        Ok::<_, &'static str>((outString.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outString = __try0_o0;
        }
        Err(_) => {
            outString = literal!("");
        }
    }
    outString
}

pub(crate) fn getIntegerNamedAnnotation(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: &Absyn::Program,
    mut id: &metamodelica::Ref<Absyn::Path>,
) -> ArcStr {
    let mut outString: ArcStr;
    let mut cdef: metamodelica::Ref<Absyn::Class>;
    let mut exp: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut ann: i32;
    match '__try0: {
        cdef =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(inPath.clone(), inProgram, false, false), '__try0);
        exp = AbsynUtil::getNamedAnnotationInClass(&cdef, id, &getAnnotationExp);
        if (exp).is_some() {
            let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
                Some(Deref @ Absyn::Exp::INTEGER { value: __pa1 }) => __pa1.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            ann = metamodelica::Own::own(__pa1);
            outString = intString(ann);
        } else {
            outString = literal!("");
        }
        Ok::<_, &'static str>((outString.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outString = __try0_o0;
        }
        Err(_) => {
            outString = literal!("");
        }
    }
    outString
}

pub(crate) fn getNamedAnnotationValue(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
    mut name: ArcStr,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    cls = ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false)?;
    result = getNamedAnnotationValueInClass(classPath, &cls, program, name)?;
    Ok(result)
}

pub(crate) fn getNamedAnnotationValueInClass(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut cls: &metamodelica::Ref<Absyn::Class>,
    mut program: Absyn::Program,
    mut name: ArcStr,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>;
    let mut arg: metamodelica::Ref<Absyn::ElementArg>;
    let mut r#str: ArcStr;
    r#mod = AbsynUtil::lookupClassAnnotation(cls, &name)?;
    result = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(Deref @ Absyn::Modification { .. }) => {
            arg = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: name }), modification: r#mod, comment: None, info: Absyn::dummyInfo.clone() });
            r#str = getAnnotationString(metamodelica::Ref::new(Absyn::Annotation { elementArgs: list![arg] }), cls, program, classPath)?;
            InteractiveUtil::makeAnnotationArrayValue(list![r#str])
        },
        _ => ValuesMake::makeEmptyArray(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) static USES_PATH: std::sync::LazyLock<metamodelica::Ref<Absyn::Path>> =
    std::sync::LazyLock::new(|| metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("uses") }));

pub(crate) fn getUsesAnnotation(
    mut program: Absyn::Program,
) -> Result<metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)>> {
    pub(crate) type Annotation = (metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool);

    let mut outUses: metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)> =
        metamodelica::nil();
    let mut opt_uses: Option<
        metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)>,
    >;
    let mut uses: metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)>;
    let mut classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let Absyn::PROGRAM { classes: __pa0, .. } = program;
    classes = metamodelica::Own::own(__pa0);
    for mut cls in &*classes {
        opt_uses = AbsynUtil::getNamedAnnotationInClass(
            metamodelica::AsArg::as_arg(&cls),
            &(USES_PATH.clone()),
            &({
                let __pe_b1 = cls.name.clone();
                move |__pe_a0| getUsesAnnotationString(__pe_a0, &__pe_b1)
            }),
        );
        if (opt_uses).is_some() {
            let __pa1 = ::match_deref::match_deref! { match &(opt_uses) {
                Some(__pa1) => __pa1.clone(),
                _ => return Err("pattern mismatch"),
            } };
            uses = metamodelica::Own::own(__pa1);
            outUses = listAppend(uses, outUses);
        }
    }
    Ok(outUses)
}

pub(crate) fn getUsesAnnotationOrDefault(
    mut p: Absyn::Program,
    mut requireExactVersion: bool,
) -> Result<metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)>> {
    let mut usesStr: metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)>;
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut strs: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut fromVersions: metamodelica::List<ArcStr>;
    usesStr = getUsesAnnotation(p)?;
    paths = List::map(usesStr.clone(), &fnptr!(Util::tuple41, _))?;
    fromVersions = List::map(usesStr.clone(), &fnptr!(Util::tuple42, _))?;
    strs = List::map(usesStr, &fnptr!(Util::tuple43, _))?;
    usesStr = ({
        let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)> =
            metamodelica::nil();
        let __thr_src0 = paths;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = fromVersions;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        let __thr_src2 = strs;
        let mut __thr_it2 = (&__thr_src2).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next(), __thr_it2.next()) {
                (Some(p), Some(f), Some(s)) => {
                    let __x = (p.clone(), f.clone(), s.clone(), false);
                    __acc = cons(__x, __acc);
                }
                (None, None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(usesStr)
}

fn getUsesAnnotationString(
    mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>,
    mut classOrigin: &ArcStr,
) -> Result<metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)>> {
    let mut usesStr: metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)>;
    usesStr = (::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: arglst, .. }) => {
            getUsesAnnotationString2(metamodelica::AsArg::as_arg(&arglst), classOrigin)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(usesStr)
}

fn getUsesAnnotationString2<'__b>(
    mut eltArgs: &'__b metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut classOrigin: &'__b ArcStr,
) -> Result<metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)>> {
    '__tco: loop {
        ::match_deref::match_deref! { match eltArgs {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(metamodelica::nil())
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name }, modification: Some(Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "version" }, modification: omod, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }), info, .. }, tail: xs } => {
                let mut version: ArcStr;
                let mut ss: metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)>;
                version = (::match_deref::match_deref! { match &(omod.clone()) {
            Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::EXPRESSIONCOMMENT { exp: Deref @ Absyn::Exp::STRING { value: __esc_version }, .. }, .. }, .. }) => {
                version = (*__esc_version).clone();
                version.clone()
            },
            Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::STRING { value: __esc_version }, .. }, .. }) => {
                version = (*__esc_version).clone();
                version.clone()
            },
            _ => {
                Error::addSourceMessage(&(Error::USES_MISSING_VERSION.clone()), list![name.clone()], metamodelica::AsArg::as_arg(&info))?;
                literal!("default")
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } });
                ss = getUsesAnnotationString2(xs, classOrigin)?;
                return Ok(metamodelica::cons((metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), classOrigin.clone(), list![version], false), ss))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut ss: metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)>;
                { (eltArgs, classOrigin) = (xs, classOrigin); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn getUsedVersion(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut library: &metamodelica::Ref<Absyn::Path>,
) -> Result<Option<ArcStr>> {
    let mut version: Option<ArcStr> = None;
    let mut uses: metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)>;
    let mut lib: metamodelica::Ref<Absyn::Path>;
    let mut versions: metamodelica::List<ArcStr>;
    uses = getUsesAnnotationOrDefault(
        Absyn::Program {
            classes: list![cls],
            within_: openmodelica_ast::Absyn::Within::TOP,
        },
        true,
    )?;
    for mut u in &*uses {
        (lib, _, versions, _) = u.clone();
        if AbsynUtil::pathEqual(library, &lib) {
            if !((versions).is_empty()) {
                version = Some((versions).head().cloned()?);
                return Ok(version);
            }
        }
    }
    Ok(version)
}

pub(crate) fn updateUsedVersion(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut library: metamodelica::Ref<Absyn::Path>,
    mut newVersion: &ArcStr,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    fn make_version_exp(mut exp: &metamodelica::Ref<Absyn::Exp>, mut version: ArcStr) -> metamodelica::Ref<Absyn::Exp> {
        let mut outExp: metamodelica::Ref<Absyn::Exp> =
            metamodelica::Ref::new(Absyn::Exp::STRING { value: version.clone() });
        outExp
    }

    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut opt_ann: Option<metamodelica::Ref<Absyn::Annotation>>;
    let mut ann: metamodelica::Ref<Absyn::Annotation>;
    let mut found: bool;
    opt_ann = AbsynUtil::getClassAnnotation(&cls)?;
    if (opt_ann).is_none() {
        return Ok(cls);
    }
    let __pa0 = ::match_deref::match_deref! { match &(opt_ann) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    ann = metamodelica::Own::own(__pa0);
    (ann, found) = AbsynUtil::mapAnnotationBinding(
        ann,
        &(AbsynUtil::prefixPath(
            literal!("uses"),
            AbsynUtil::joinPaths(
                library,
                metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("version"),
                }),
            )?,
        )),
        (std::sync::Arc::new({
            let __pe_b1 = newVersion.clone();
            move |__pe_a0| Ok(make_version_exp(&__pe_a0, __pe_b1.clone()))
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static,
            >),
    )?;
    if found {
        cls = AbsynUtil::setClassAnnotation(cls, Some(ann))?;
    }
    Ok(cls)
}

pub(crate) fn getConversionAnnotation(
    mut cls: &metamodelica::Ref<Absyn::Class>,
) -> (metamodelica::List<ArcStr>, metamodelica::List<ArcStr>) {
    let mut withoutConversion: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut withConversion: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut opt_conversion: Option<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)>;
    opt_conversion = AbsynUtil::getNamedAnnotationInClass(
        cls,
        &(metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("conversion"),
        })),
        &getConversionAnnotationString,
    );
    (withoutConversion, withConversion) = (::match_deref::match_deref! { match &(opt_conversion) {
        Some((__esc_withoutConversion, __esc_withConversion)) => {
            withoutConversion = (*__esc_withoutConversion).clone();
            withConversion = (*__esc_withConversion).clone();
            (withoutConversion.clone(), withConversion.clone())
        },
        _ => (metamodelica::nil(), metamodelica::nil()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (withoutConversion, withConversion)
}

fn getConversionAnnotationString(
    mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut result: (metamodelica::List<ArcStr>, metamodelica::List<ArcStr>);
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut without: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut with: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut from: metamodelica::List<ArcStr>;
    let mut script: Option<ArcStr>;
    let __pa0 = ::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: __pa0, .. }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    args = metamodelica::Own::own(__pa0);
    for mut arg in &*args {
        (from, _, script) = parseConversionAnnotationElement(arg.clone())?;
        if (script).is_none() {
            without = List::append_reverse(&from, without);
        } else {
            with = List::append_reverse(&from, with);
        }
    }
    result = (without.reverse(), with.reverse());
    Ok(result)
}

pub(crate) fn getConversionsInClass(
    mut cls: &metamodelica::Ref<Absyn::Class>,
) -> metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)> {
    let mut result: metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>;
    let mut res: Option<metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>>;
    res = AbsynUtil::getNamedAnnotationInClass(
        cls,
        &(metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("conversion"),
        })),
        &getConversionsInClassMod,
    );
    result = Util::getOptionOrDefault(res, metamodelica::nil());
    result
}

fn getConversionsInClassMod(
    mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>,
) -> Result<metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>> {
    let mut res: metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)> = metamodelica::nil();
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut from: metamodelica::List<ArcStr>;
    let mut to: Option<ArcStr>;
    let mut script: Option<ArcStr>;
    let __pa0 = ::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: __pa0, .. }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    args = metamodelica::Own::own(__pa0);
    for mut arg in &*args {
        (from, to, script) = parseConversionAnnotationElement(arg.clone())?;
        for mut v in &*from {
            res = metamodelica::cons((v.clone(), to.clone(), script.clone()), res);
        }
    }
    Ok(res)
}

fn parseConversionAnnotationElement(
    mut r#mod: metamodelica::Ref<Absyn::ElementArg>,
) -> Result<(metamodelica::List<ArcStr>, Option<ArcStr>, Option<ArcStr>)> {
    let mut fromVersion: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut toVersion: Option<ArcStr> = None;
    let mut scriptFilename: Option<ArcStr> = None;
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut arg_mod: Option<metamodelica::Ref<Absyn::Modification>>;
    let mut name: ArcStr;
    let mut info: SourceInfo;
    let mut exp: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let () = (::match_deref::match_deref! { match &(&*r#mod) {
        Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noneFromVersion" }, modification: __mod_modification, .. } => {
            fromVersion = list![AbsynUtil::expString(&(AbsynUtil::stripCommentExpressions(getAnnotationExp(__mod_modification.clone())?, false)?))?];
            ()
        },
        Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "from" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: __esc_args, .. }), .. } => {
            args = (*__esc_args).clone();
            for mut arg in &*args.clone() {
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(arg.clone()) {
                    Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: __pa0 }, modification: __pa1, info: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                name = metamodelica::Own::own(__pa0);
                arg_mod = metamodelica::Own::own(__pa1);
                info = metamodelica::Own::own(__pa2);
                let () = 'mc: {
        let __mc_input = name.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "version" => {
                    let mut exp: metamodelica::Ref<Absyn::Exp> = exp.clone();
                    let mut fromVersion: metamodelica::List<ArcStr> = fromVersion.clone();
                    exp = AbsynUtil::stripCommentExpressions(getAnnotationExp(arg_mod.clone())?, false)?;
                    fromVersion = (match &*exp {
        Absyn::Exp::STRING { value: __exp_value } => list![__exp_value.clone()],
        Absyn::Exp::ARRAY { arrayExp: __exp_arrayExp } => ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut e in (__exp_arrayExp.clone()).into_iter().cloned() {
                    let __x = AbsynUtil::expString(&(e.clone()))?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        _ => return Err("match: no arm matched"),
    });
                    Ok(((), exp.clone(), fromVersion.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() { exp = __wb0; fromVersion = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "to" => {
                    let mut toVersion: Option<ArcStr> = toVersion.clone();
                    toVersion = Some(AbsynUtil::expString(&(AbsynUtil::stripCommentExpressions(getAnnotationExp(arg_mod.clone())?, false)?))?);
                    Ok(((), toVersion.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() { toVersion = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "script" => {
                    let mut scriptFilename: Option<ArcStr> = scriptFilename.clone();
                    scriptFilename = Some(AbsynUtil::expString(&(AbsynUtil::stripCommentExpressions(getAnnotationExp(arg_mod.clone())?, false)?))?);
                    Ok(((), scriptFilename.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() { scriptFilename = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !(StringUtil::startsWith(name.clone(), literal!("__"))) {
                        Error::addSourceMessage(&(Error::CONVERSION_UNKNOWN_ANNOTATION.clone()), list![name.clone()], &info)?;
                    }
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() { break 'mc __v; }
        return Err("matchcontinue: no arm matched")
    };
                if (fromVersion).is_empty() {
                    let __pa4 = ::match_deref::match_deref! { match &(r#mod.clone()) {
                        Deref @ Absyn::ElementArg::MODIFICATION { info: __pa4, .. } => __pa4.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    info = metamodelica::Own::own(__pa4);
                    Error::addSourceMessage(&(Error::CONVERSION_MISSING_FROM_VERSION.clone()), list![Dump::unparseElementArgStr(r#mod.clone())?], &info)?;
                }
            }
            ()
        },
        Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: __esc_name }, info: __esc_info, .. } => {
            name = (*__esc_name).clone();
            info = (*__esc_info).clone();
            if !(StringUtil::startsWith(name.clone(), literal!("__"))) {
                Error::addSourceMessage(&(Error::CONVERSION_UNKNOWN_ANNOTATION.clone()), list![name.clone()], metamodelica::AsArg::as_arg(&info))?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((fromVersion, toVersion, scriptFilename))
}

pub(crate) fn getPackagesInPath(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    paths = 'mc: {
        let __mc_input = (inPath, inProgram);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (modelpath, p) => {
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    cdef = ProgramUtil::getPathedClassInProgram(modelpath.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    Ok(getPackagesInClass(metamodelica::AsArg::as_arg(&modelpath), metamodelica::AsArg::as_arg(&p), &cdef)?)
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
    paths
}

pub(crate) fn getTopPackages(mut p: &Absyn::Program) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    paths = List::map(
        getTopPackagesInProgram(p)?,
        &fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr),
    )?;
    Ok(paths)
}

fn getTopPackagesInProgram(mut inProgram: &Absyn::Program) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    outStringLst = 'mc: {
        let __mc_input = inProgram;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { classes: Deref @ metamodelica::ListNode::Nil, .. } => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { classes: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Class { name: id, restriction: Absyn::Restriction::R_PACKAGE { .. }, .. }, tail: rest }, within_: w } => {
                    let mut res: metamodelica::List<ArcStr>;
                    res = getTopPackagesInProgram(&(Absyn::Program { classes: rest.clone(), within_: w.clone() }))?;
                    Ok(metamodelica::cons(id.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { classes: Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, within_: w } => {
                    let mut res: metamodelica::List<ArcStr>;
                    res = getTopPackagesInProgram(&(Absyn::Program { classes: rest.clone(), within_: w.clone() }))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStringLst)
}

fn getPackagesInClass(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inProgram: &Absyn::Program,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut outString: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    outString = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut strlist: metamodelica::List<ArcStr>;
            strlist = getPackagesInParts(metamodelica::AsArg::as_arg(&parts));
            List::map(strlist, &fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr))?
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut strlist: metamodelica::List<ArcStr>;
            strlist = getPackagesInParts(metamodelica::AsArg::as_arg(&parts));
            List::map(strlist, &fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr))?
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: _, arrayDim: _ }, .. }, .. } => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

fn getPackagesInParts<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: rest } => {
                let mut l1: metamodelica::List<ArcStr>;
                let mut l2: metamodelica::List<ArcStr>;
                let mut res: metamodelica::List<ArcStr>;
                l1 = getPackagesInElts(metamodelica::AsArg::as_arg(&elts));
                l2 = getPackagesInParts(rest);
                return listAppend(l1, l2)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: rest } => {
                let mut l1: metamodelica::List<ArcStr>;
                let mut l2: metamodelica::List<ArcStr>;
                let mut res: metamodelica::List<ArcStr>;
                l1 = getPackagesInElts(metamodelica::AsArg::as_arg(&elts));
                l2 = getPackagesInParts(rest);
                return listAppend(l1, l2)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut res: metamodelica::List<ArcStr>;
                { inAbsynClassPartLst = rest; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getPackagesInElts<'__b>(
    mut inAbsynElementItemLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> metamodelica::List<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynElementItemLst {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name: id, restriction: Absyn::Restriction::R_PACKAGE { .. }, .. }, .. }, .. } }, tail: rest } => {
                let mut res: metamodelica::List<ArcStr>;
                res = getPackagesInElts(rest);
                return metamodelica::cons(id.clone(), res)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut res: metamodelica::List<ArcStr>;
                { inAbsynElementItemLst = rest; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn getClassnamesInPath(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
    mut inShowProtected: bool,
    mut includeConstants: bool,
) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    paths = 'mc: {
        let __mc_input = (inPath, inProgram, inShowProtected, includeConstants);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (modelpath, p, b, c) => {
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    cdef = ProgramUtil::getPathedClassInProgram(modelpath.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    Ok(ProgramUtil::getClassnamesInClass(metamodelica::AsArg::as_arg(&modelpath), metamodelica::AsArg::as_arg(&p), &cdef, b.clone(), c.clone())?)
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
    paths
}

pub(crate) fn getTopClassnames(mut p: &Absyn::Program) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    paths = List::map(
        getTopClassnamesInProgram(p)?,
        &fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr),
    )?;
    Ok(paths)
}

pub(crate) fn getTopClassnamesInProgram(mut inProgram: &Absyn::Program) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    outStringLst = 'mc: {
        let __mc_input = inProgram;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { classes: Deref @ metamodelica::ListNode::Nil, .. } => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { classes: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Class { name: id, .. }, tail: rest }, within_: w } => {
                    let mut res: metamodelica::List<ArcStr>;
                    res = getTopClassnamesInProgram(&(Absyn::Program { classes: rest.clone(), within_: w.clone() }))?;
                    Ok(metamodelica::cons(id.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { classes: Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, within_: w } => {
                    let mut res: metamodelica::List<ArcStr>;
                    res = getTopClassnamesInProgram(&(Absyn::Program { classes: rest.clone(), within_: w.clone() }))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStringLst)
}

fn getTopQualifiedClassnames(
    mut inProgram: &Absyn::Program,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut outStringLst: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    outStringLst = 'mc: {
        let __mc_input = inProgram;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { classes: Deref @ metamodelica::ListNode::Nil, .. } => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { classes: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Class { name: id, .. }, tail: rest }, within_: w } => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    let mut p: metamodelica::Ref<Absyn::Path>;
                    p = AbsynUtil::joinWithinPath(metamodelica::AsArg::as_arg(&w), metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() }))?;
                    res = getTopQualifiedClassnames(&(Absyn::Program { classes: rest.clone(), within_: w.clone() }))?;
                    Ok(metamodelica::cons(p.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Absyn::Program { classes: Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, within_: w } => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    res = getTopQualifiedClassnames(&(Absyn::Program { classes: rest.clone(), within_: w.clone() }))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStringLst)
}

fn getBaseClasses(
    mut cls: &metamodelica::Ref<Absyn::Class>,
    mut env: FCore::Graph,
) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
    let mut baseClasses: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut base_class_name: ArcStr;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut cenv: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut env_path_opt: Option<metamodelica::Ref<Absyn::Path>> = None;
    let mut env_path: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut path: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    baseClasses = 'mc: {
        let __mc_input = &**cls;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
                    Ok(getBaseClassesFromParts(metamodelica::AsArg::as_arg(&parts), &env))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: base_class_name, parts, .. }, .. } => {
                    let mut cenv: FCore::Graph = cenv.clone();
                    let mut env_path: metamodelica::Ref<Absyn::Path> = env_path.clone();
                    let mut path: metamodelica::Ref<Absyn::Path> = path.clone();
                    (_, _, cenv) = Lookup::lookupClassIdent(FCore::emptyCache(), env.clone(), metamodelica::AsArg::as_arg(&base_class_name), Some(cls.info.clone()))?;
                    let __pa0 = ::match_deref::match_deref! { match &(FGraph::getScopePath(&cenv)?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    env_path = metamodelica::Own::own(__pa0);
                    path = AbsynUtil::suffixPath(&env_path, metamodelica::AsArg::as_arg(&base_class_name));
                    Ok((metamodelica::cons(path.clone(), getBaseClassesFromParts(metamodelica::AsArg::as_arg(&parts), &env)), cenv.clone(), env_path.clone(), path.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cenv = __wb0;
            env_path = __wb1;
            path = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path, .. }, .. }, .. } => {
                    let mut path = (*path).clone();
                    let mut cenv: FCore::Graph = cenv.clone();
                    let mut env_path_opt: Option<metamodelica::Ref<Absyn::Path>> = env_path_opt.clone();
                    (_, _, cenv) = Lookup::lookupClass(&(FCore::emptyCache()), &env, metamodelica::AsArg::as_arg(&path), Some(cls.info.clone()))?;
                    env_path_opt = FGraph::getScopePath(&cenv)?;
                    if (env_path_opt).is_some() {
                        path = AbsynUtil::suffixPath(&(Util::getOption(env_path_opt.clone())?), &(AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path))));
                    }
                    Ok((list![path.clone()], cenv.clone(), env_path_opt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cenv = __wb0;
            env_path_opt = __wb1;
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
    baseClasses
}

fn getBaseClassesFromParts(
    mut parts: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut env: &FCore::Graph,
) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
    let mut baseClasses: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
    for mut part in &**parts {
        for mut el in &*AbsynUtil::getElementItemsInClassPart(metamodelica::AsArg::as_arg(&part)) {
            baseClasses = getBaseClassesFromElt(metamodelica::AsArg::as_arg(&el), env, baseClasses);
        }
    }
    baseClasses = Dangerous::listReverseInPlace(baseClasses);
    baseClasses
}

fn getBaseClassesFromElt(
    mut element: &metamodelica::Ref<Absyn::ElementItem>,
    mut env: &FCore::Graph,
    mut baseClasses: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
    let mut baseClasses: metamodelica::List<metamodelica::Ref<Absyn::Path>> = baseClasses;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut info: SourceInfo;
    let mut cenv: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut env_path_opt: Option<metamodelica::Ref<Absyn::Path>> = None;
    baseClasses = 'mc: {
        let __mc_input = &**element;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::EXTENDS { path, .. }, info, .. } } => {
                    let mut path = (*path).clone();
                    let mut cenv: FCore::Graph = cenv.clone();
                    let mut env_path_opt: Option<metamodelica::Ref<Absyn::Path>> = env_path_opt.clone();
                    (_, _, cenv) = Lookup::lookupClass(&(FCore::emptyCache()), env, metamodelica::AsArg::as_arg(&path), Some(info.clone()))?;
                    env_path_opt = FGraph::getScopePath(&cenv)?;
                    if (env_path_opt).is_some() {
                        path = AbsynUtil::suffixPath(&(Util::getOption(env_path_opt.clone())?), &(AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path))));
                    }
                    Ok((metamodelica::cons(path.clone(), baseClasses.clone()), cenv.clone(), env_path_opt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cenv = __wb0;
            env_path_opt = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(baseClasses.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    baseClasses
}

fn countBaseClasses(mut inClass: &metamodelica::Ref<Absyn::Class>) -> i32 {
    let mut count: i32;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    count = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: __esc_parts, .. }, .. } => {
            parts = (*__esc_parts).clone();
            countBaseClassesFromParts(metamodelica::AsArg::as_arg(&parts))
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: __esc_parts, .. }, .. } => {
            parts = (*__esc_parts).clone();
            countBaseClassesFromParts(metamodelica::AsArg::as_arg(&parts))
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => 1,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    count
}

fn countBaseClassesFromParts(mut parts: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>) -> i32 {
    let mut count: i32 = 0;
    for mut part in &**parts {
        for mut el in &*AbsynUtil::getElementItemsInClassPart(metamodelica::AsArg::as_arg(&part)) {
            if AbsynUtil::isElementItemExtends(metamodelica::AsArg::as_arg(&el)) {
                count = count + 1;
            }
        }
    }
    count
}

pub(crate) fn getDocumentationClassAnnotation(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut p: Absyn::Program,
) -> Result<bool> {
    let mut isDocClass: bool;
    isDocClass = (match p.clone() {
        _ => {
            let mut docStr: ArcStr;
            docStr = ProgramUtil::getNamedAnnotationExp(
                className,
                p,
                &(metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("DocumentationClass"),
                })),
                Some(literal!("false")),
                &fnptr!(
                    getDocumentationClassAnnotationModStr,
                    Option<metamodelica::Ref<Absyn::Modification>>
                ),
            )?;
            stringEq(&docStr, &(literal!("true")))
        }
    });
    Ok(isDocClass)
}

fn getDocumentationClassAnnotationModStr(mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>) -> ArcStr {
    let mut docStr: ArcStr = arcstr::literal!("");
    docStr = 'mc: {
        let __mc_input = r#mod;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: e, .. }, .. }) => {
                    let mut docStr: ArcStr = docStr.clone();
                    docStr = Dump::printExpStr(e.clone())?;
                    Ok((docStr.clone(), docStr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            docStr = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("false"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    docStr
}

pub(crate) fn getDefaultComponentName(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut r#str: ArcStr;
    r#str = getStringNamedAnnotation(
        classPath,
        program,
        &(metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("defaultComponentName"),
        })),
    );
    result = ValuesMake::makeString(r#str);
    result
}

pub(crate) fn getDefaultComponentPrefixes(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> metamodelica::Ref<Values::Value> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut r#str: ArcStr;
    r#str = getStringNamedAnnotation(
        classPath,
        program,
        &(metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("defaultComponentPrefixes"),
        })),
    );
    result = ValuesMake::makeString(r#str);
    result
}

fn getAnnotationValue(mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>) -> ArcStr {
    let mut r#str: ArcStr;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    r#str = 'mc: {
        let __mc_input = r#mod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::ListNode::Nil, eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. } }) => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*Dump::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("{}"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    r#str
}

pub(crate) fn getAnnotationExp(
    mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::ListNode::Nil, eqMod: Deref @ Absyn::EqMod::EQMOD { exp: __pa0, .. } }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    exp = metamodelica::Own::own(__pa0);
    Ok(exp)
}

pub(crate) fn getAnnotationStringValueOrFail(
    mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::ListNode::Nil, eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. } }) => {
            AbsynUtil::getString(metamodelica::AsArg::as_arg(&exp))?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(r#str)
}

pub(crate) fn getExperimentAnnotationString(
    mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>,
) -> Result<ArcStr> {
    let mut experimentStr: ArcStr;
    experimentStr = (::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: arglst, .. }) => {
            let mut strs: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            strs = getExperimentAnnotationString2(metamodelica::AsArg::as_arg(&arglst));
            s = stringDelimitList(strs, literal!(","));
            s = stringAppendList(list![literal!("{"), s, literal!("}")]);
            s
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(experimentStr)
}

fn getExperimentAnnotationString2(
    mut eltArgs: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> metamodelica::List<ArcStr> {
    let mut strs: metamodelica::List<ArcStr>;
    strs = 'mc: {
        let __mc_input = &**eltArgs;
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
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), .. }, tail: xs } => {
                    let mut s: ArcStr;
                    let mut ss: metamodelica::List<ArcStr>;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("=")); __mm_s.push_str(&*Dump::printExpStr(exp.clone())?); ArcStr::from(__mm_s) };
                    ss = getExperimentAnnotationString2(metamodelica::AsArg::as_arg(&xs));
                    Ok(metamodelica::cons(s.clone(), ss.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                    let mut ss: metamodelica::List<ArcStr>;
                    ss = getExperimentAnnotationString2(metamodelica::AsArg::as_arg(&xs));
                    Ok(ss.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    strs
}

pub(crate) fn getDocumentationAnnotationString(
    mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>,
) -> Result<(ArcStr, ArcStr, ArcStr)> {
    let mut docStr: (ArcStr, ArcStr, ArcStr);
    docStr = (::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: arglst, .. }) => {
            let mut info: ArcStr;
            let mut revisions: ArcStr;
            let mut infoHeader: ArcStr;
            let mut partialInst: bool;
            partialInst = System::getPartialInstantiation();
            System::setPartialInstantiation(true);
            info = getDocumentationAnnotationInfo(metamodelica::AsArg::as_arg(&arglst));
            revisions = getDocumentationAnnotationRevision(metamodelica::AsArg::as_arg(&arglst));
            infoHeader = getDocumentationAnnotationInfoHeader(metamodelica::AsArg::as_arg(&arglst));
            System::setPartialInstantiation(partialInst);
            (info, revisions, infoHeader)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(docStr)
}

fn getDocumentationAnnotationInfo(mut eltArgs: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = 'mc: {
        let __mc_input = &**eltArgs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "info" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), .. }, tail: _ } => {
                    let mut dexp: metamodelica::Ref<DAE::Exp>;
                    let mut s: ArcStr;
                    (_, dexp, _) = StaticScript::elabGraphicsExp(FCore::emptyCache(), FGraph::empty(), exp.clone(), true, openmodelica_frontend_types::DAE::Prefix::NOPRE, Absyn::dummyInfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSimplify::simplify(dexp.clone())?) {
                        (Deref @ DAE::Exp::SCONST { string: __pa0 }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    s = metamodelica::Own::own(__pa0);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                    let mut ss: ArcStr;
                    ss = getDocumentationAnnotationInfo(metamodelica::AsArg::as_arg(&xs));
                    Ok(ss.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    r#str
}

fn getDocumentationAnnotationRevision(
    mut eltArgs: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = 'mc: {
        let __mc_input = &**eltArgs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "revisions" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), .. }, tail: _ } => {
                    let mut s: ArcStr;
                    let mut dexp: metamodelica::Ref<DAE::Exp>;
                    (_, dexp, _) = StaticScript::elabGraphicsExp(FCore::emptyCache(), FGraph::empty(), exp.clone(), true, openmodelica_frontend_types::DAE::Prefix::NOPRE, Absyn::dummyInfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSimplify::simplify(dexp.clone())?) {
                        (Deref @ DAE::Exp::SCONST { string: __pa0 }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    s = metamodelica::Own::own(__pa0);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                    let mut ss: ArcStr;
                    ss = getDocumentationAnnotationRevision(metamodelica::AsArg::as_arg(&xs));
                    Ok(ss.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    r#str
}

fn getDocumentationAnnotationInfoHeader(
    mut eltArgs: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = 'mc: {
        let __mc_input = &**eltArgs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "__OpenModelica_infoHeader" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), .. }, tail: _ } => {
                    let mut s: ArcStr;
                    let mut dexp: metamodelica::Ref<DAE::Exp>;
                    (_, dexp, _) = StaticScript::elabGraphicsExp(FCore::emptyCache(), FGraph::empty(), exp.clone(), true, openmodelica_frontend_types::DAE::Prefix::NOPRE, Absyn::dummyInfo.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSimplify::simplify(dexp.clone())?) {
                        (Deref @ DAE::Exp::SCONST { string: __pa0 }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    s = metamodelica::Own::own(__pa0);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                    let mut ss: ArcStr;
                    ss = getDocumentationAnnotationInfoHeader(metamodelica::AsArg::as_arg(&xs));
                    Ok(ss.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    r#str
}

fn getNthPublicConnectorStr(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut cls: &metamodelica::Ref<Absyn::Class>,
    mut program: Absyn::Program,
    mut n: i32,
) -> Result<(Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>, i32)> {
    let mut conn: Option<(ArcStr, metamodelica::Ref<Absyn::Path>)> = None;
    let mut n: i32 = n;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    parts = AbsynUtil::getClassPartsInClass(cls);
    for mut part in &*parts {
        (conn, n) = (match &*part.clone() {
            Absyn::ClassPart::PUBLIC {
                contents: __part_contents,
            } => getNthConnectorInfo(
                program.clone(),
                classPath.clone(),
                metamodelica::AsArg::as_arg(&__part_contents),
                n,
            )?,
            _ => (conn, n),
        });
        if n <= 0 {
            break;
        }
    }
    Ok((conn, n))
}

fn getNthConnectorInfo(
    mut program: Absyn::Program,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut items: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut n: i32,
) -> Result<(Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>, i32)> {
    let mut conn: Option<(ArcStr, metamodelica::Ref<Absyn::Path>)> = None;
    let mut n: i32 = n;
    let mut tp: metamodelica::Ref<Absyn::Path>;
    let mut cls_path: metamodelica::Ref<Absyn::Path>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut comps: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    let mut comp_count: i32;
    let mut name: ArcStr;
    for mut item in &**items {
        (conn, n) = (::match_deref::match_deref! { match &(item.clone()) {
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::EXTENDS { path: __esc_tp, .. }, .. } } => {
                tp = (*__esc_tp).clone();
                (cls, cls_path) = lookupClassdef(tp.clone(), classPath.clone(), program.clone())?;
                getNthPublicConnectorStr(cls_path, &cls, program.clone(), n)?
            },
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __esc_tp, .. }, components: __esc_comps, .. }, .. } } => {
                tp = (*__esc_tp).clone();
                comps = (*__esc_comps).clone();
                (cls, _) = lookupClassdef(tp.clone(), classPath.clone(), program.clone())?;
                if AbsynUtil::isConnector(&cls) || AbsynUtil::isExpandableConnector(&cls) {
                    comp_count = ((comps).len() as i32);
                    if n <= comp_count {
                        name = AbsynUtil::componentName(&((comps).get(n)?))?;
                        conn = Some((name, tp.clone()));
                    }
                    n = n - comp_count;
                }
                (conn, n)
            },
            _ => (conn, n),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if n <= 0 {
            break;
        }
    }
    Ok((conn, n))
}

fn countPublicConnectors<'__b>(
    mut classPath: &'__b metamodelica::Ref<Absyn::Path>,
    mut program: &'__b Absyn::Program,
    mut cls: metamodelica::Ref<Absyn::Class>,
) -> Result<i32> {
    '__tco: loop {
        let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
        let mut cdef: metamodelica::Ref<Absyn::Class>;
        let mut cls_name: metamodelica::Ref<Absyn::Path>;
        ::match_deref::match_deref! { match &(cls) {
            Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: __esc_parts, .. }, .. } => {
                parts = (*__esc_parts).clone();
                return Ok(countPublicConnectorsInParts(metamodelica::AsArg::as_arg(&parts), classPath.clone(), program.clone()))
            },
            Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: __esc_parts, .. }, .. } => {
                parts = (*__esc_parts).clone();
                return Ok(countPublicConnectorsInParts(metamodelica::AsArg::as_arg(&parts), classPath.clone(), program.clone()))
            },
            Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __esc_cls_name, .. }, .. }, .. } => {
                cls_name = (*__esc_cls_name).clone();
                (cdef, _) = lookupClassdef(cls_name.clone(), classPath.clone(), program.clone())?;
                { (classPath, program, cls) = (classPath, program, cdef); continue '__tco; }
            },
            _ => return Ok(0),
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn countPublicConnectorsInParts(
    mut parts: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> i32 {
    let mut count: i32 = 0;
    for mut part in &**parts {
        count = (match &*part.clone() {
            Absyn::ClassPart::PUBLIC {
                contents: __part_contents,
            } => {
                count
                    + countConnectors(
                        classPath.clone(),
                        program.clone(),
                        metamodelica::AsArg::as_arg(&__part_contents),
                    )
            }
            _ => count,
        });
    }
    count
}

fn countConnectors(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
    mut items: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> i32 {
    let mut count: i32 = 0;
    let mut cls: metamodelica::Ref<Absyn::Class> =
        <metamodelica::Ref<Absyn::Class> as ::std::default::Default>::default();
    let mut tp: metamodelica::Ref<Absyn::Path>;
    let mut cls_path: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut comps: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    let mut c: i32;
    for mut item in &**items {
        c = 'mc: {
            let __mc_input = item.clone();
            if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::EXTENDS { path: tp, .. }, .. } } => {
                        let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                        let mut cls_path: metamodelica::Ref<Absyn::Path> = cls_path.clone();
                        (cls, cls_path) = lookupClassdef(tp.clone(), classPath.clone(), program.clone())?;
                        Ok((countPublicConnectors(&cls_path, &program, cls.clone())?, cls.clone(), cls_path.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                cls = __wb0;
                cls_path = __wb1;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tp, .. }, components: comps, .. }, .. } } => {
                        let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                        (cls, _) = lookupClassdef(tp.clone(), classPath.clone(), program.clone())?;
                        Ok((if (AbsynUtil::isConnector(&cls) || AbsynUtil::isExpandableConnector(&cls)) {((comps).len() as i32)} else {0}, cls.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                cls = __wb0;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok(0)
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        };
        count = count + c;
    }
    count
}

fn getConnectionAnnotationStrElArgs(
    mut inElArgLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut info: &SourceInfo,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inFullProgram: &Absyn::Program,
    mut inModelPath: &metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    outStringLst = 'mc: {
        let __mc_input = &**inElArgLst;
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
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: annName }, modification: Some(Deref @ Absyn::Modification { elementArgLst: r#mod, eqMod: _ }), .. }, tail: rest } => {
                    let mut fargs: metamodelica::Ref<Absyn::FunctionArgs>;
                    let mut p_1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut env: FCore::Graph;
                    let mut newexp: metamodelica::Ref<DAE::Exp>;
                    let mut gexpstr: ArcStr;
                    let mut res: metamodelica::List<ArcStr>;
                    let mut cache: FCore::Cache;
                    let mut prop: DAE::Properties;
                    let mut lineProgram: Absyn::Program;
                    lineProgram = InteractiveUtil::modelicaAnnotationProgram(Config::getAnnotationVersion()?)?;
                    fargs = createFuncargsFromElementargs(metamodelica::AsArg::as_arg(&r#mod))?;
                    p_1 = AbsynToSCode::translateAbsyn2SCode(lineProgram.clone())?;
                    (cache, env) = Inst::makeEnvFromProgram(&p_1)?;
                    (_, newexp, prop) = StaticScript::elabGraphicsExp(cache.clone(), env.clone(), metamodelica::Ref::new(Absyn::Exp::CALL { function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: annName.clone(), subscripts: metamodelica::nil() }), functionArgs: fargs.clone(), typeVars: metamodelica::nil() }), false, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    (cache, newexp, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), newexp.clone(), prop.clone(), false, info.clone())?;
                    Print::clearErrorBuf();
                    gexpstr = ExpressionBasics::printExpStr(newexp.clone())?;
                    res = getConnectionAnnotationStrElArgs(metamodelica::AsArg::as_arg(&rest), info, inClass, inFullProgram, inModelPath)?;
                    Ok(metamodelica::cons(gexpstr.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: annName }, modification: Some(Deref @ Absyn::Modification { elementArgLst: _, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } }), .. }, tail: rest } => {
                    let mut gexpstr_1: ArcStr;
                    let mut res: metamodelica::List<ArcStr>;
                    gexpstr_1 = stringAppendList(list![annName.clone(), literal!("(error)")]);
                    res = getConnectionAnnotationStrElArgs(metamodelica::AsArg::as_arg(&rest), info, inClass, inFullProgram, inModelPath)?;
                    Ok(metamodelica::cons(gexpstr_1.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStringLst)
}

fn getConnectionAnnotationStr(
    mut inEquationItem: &metamodelica::Ref<Absyn::EquationItem>,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inFullProgram: &Absyn::Program,
    mut inModelPath: &metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut res: metamodelica::List<ArcStr>;
    let mut annotations: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut info: SourceInfo;
    result = (::match_deref::match_deref! { match inEquationItem {
        Deref @ Absyn::EquationItem::EQUATIONITEM { info: __esc_info, equation_: Deref @ Absyn::Equation::EQ_CONNECT { .. }, comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: __esc_annotations }), .. }) } => {
            info = (*__esc_info).clone();
            annotations = (*__esc_annotations).clone();
            res = getConnectionAnnotationStrElArgs(metamodelica::AsArg::as_arg(&annotations), metamodelica::AsArg::as_arg(&info), inClass, inFullProgram, inModelPath)?;
            InteractiveUtil::makeAnnotationArrayValue(res)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(result)
}

pub(crate) fn createFuncargsFromElementargs(
    mut inAbsynElementArgLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<metamodelica::Ref<Absyn::FunctionArgs>> {
    let mut outFunctionArgs: metamodelica::Ref<Absyn::FunctionArgs>;
    outFunctionArgs = 'mc: {
        let __mc_input = &**inAbsynElementArgLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: id }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), .. }, tail: xs } => {
                    let mut expl: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut narg: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(createFuncargsFromElementargs(metamodelica::AsArg::as_arg(&xs))?) {
                        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: __pa0, argNames: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    expl = metamodelica::Own::own(__pa0);
                    narg = metamodelica::Own::own(__pa1);
                    Ok(metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: expl.clone(), argNames: metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: id.clone(), argValue: exp.clone() }), narg.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                    let mut expl: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut narg: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(createFuncargsFromElementargs(metamodelica::AsArg::as_arg(&xs))?) {
                        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: __pa0, argNames: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    expl = metamodelica::Own::own(__pa0);
                    narg = metamodelica::Own::own(__pa1);
                    Ok(metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: expl.clone(), argNames: narg.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outFunctionArgs)
}

fn getConnectionStr(mut inEquation: &metamodelica::Ref<Absyn::Equation>) -> Result<(ArcStr, ArcStr)> {
    let mut outFromString: ArcStr;
    let mut outToString: ArcStr;
    (outFromString, outToString) = (match &**inEquation {
        Absyn::Equation::EQ_CONNECT {
            connector1: cr1,
            connector2: cr2,
        } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = Dump::printComponentRefStr(cr1)?;
            s2 = Dump::printComponentRefStr(cr2)?;
            (s1, s2)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outFromString, outToString))
}

pub(crate) fn getConnections(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> {
    let mut connections: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    connections = getConnectionsInClassparts(&(AbsynUtil::getClassPartsInClass(inClass)));
    connections
}

fn getConnectionsInClassparts<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: eqlist1 }, tail: xs } => {
                let mut eqlist2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                let mut eqlist1 = (*eqlist1).clone();
                eqlist1 = getConnectionsInEquations(eqlist1.clone());
                eqlist2 = getConnectionsInClassparts(xs);
                return listAppend(eqlist1.clone(), eqlist2)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut eqlist1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                { inAbsynClassPartLst = xs; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getConnectionsInEquations(
    mut inAbsynEquationItemLst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inAbsynEquationItemLst) {
            Deref @ metamodelica::ListNode::Cons { head: eq @ Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_CONNECT { .. }, .. }, tail: xs } => {
                let mut eqlist1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                eqlist1 = getConnectionsInEquations(xs.clone());
                return metamodelica::cons(eq.clone(), eqlist1)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_FOR { forEquations: forEqList, .. }, .. }, tail: xs } => {
                let mut eqlist1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                let mut eqlist2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                eqlist1 = getConnectionsInEquations(forEqList.clone());
                eqlist2 = getConnectionsInEquations(xs.clone());
                return listAppend(eqlist1, eqlist2)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut eqlist1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                { inAbsynEquationItemLst = xs.clone(); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn getComponentModification(
    mut element: &metamodelica::Ref<Absyn::Element>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut comps: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    let mut opt_mod: Option<metamodelica::Ref<Absyn::Modification>>;
    let mut r#mod: metamodelica::Ref<Absyn::Modification>;
    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
    result = (::match_deref::match_deref! { match element {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: __esc_comps, .. }, .. } => {
            comps = (*__esc_comps).clone();
            for mut c in &*comps.clone() {
                opt_mod = c.component.modification.clone();
                r#mod = if ((opt_mod).is_some()) {Util::getOption(opt_mod)?} else {Absyn::emptyMod.clone()};
                vals = metamodelica::cons(metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_MODIFICATION { modification: r#mod }) }), vals);
            }
            vals = Dangerous::listReverseInPlace(vals);
            ValuesMake::makeArray(vals)
        },
        _ => ValuesMake::makeEmptyArray(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn cacheProgramAndPath(mut inCache: &GraphicEnvCache) -> (Absyn::Program, metamodelica::Ref<Absyn::Path>) {
    let mut outProgram: Absyn::Program;
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    (outProgram, outPath) = (match inCache.clone() {
        GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE { .. } => (
            var_field!(inCache.program, GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE).clone(),
            var_field!(inCache.modelPath, GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE).clone(),
        ),
        GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE { .. } => (
            var_field!(inCache.program, GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone(),
            var_field!(inCache.modelPath, GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone(),
        ),
        GraphicEnvCache::GRAPHIC_ENV_NO_CACHE { .. } => (
            var_field!(inCache.program, GraphicEnvCache::GRAPHIC_ENV_NO_CACHE).clone(),
            var_field!(inCache.modelPath, GraphicEnvCache::GRAPHIC_ENV_NO_CACHE).clone(),
        ),
    });
    (outProgram, outPath)
}

pub(crate) fn envFromGraphicEnvCache(mut inEnvCache: GraphicEnvCache) -> Result<FCore::Graph> {
    let mut env: FCore::Graph;
    let GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE { env: __pa0, .. } = (inEnvCache) else {
        return Err("pattern mismatch");
    };
    env = metamodelica::Own::own(__pa0);
    Ok(env)
}

fn cacheFromGraphicEnvCache(mut inEnvCache: GraphicEnvCache) -> Result<FCore::Cache> {
    let mut cache: FCore::Cache;
    let GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE { cache: __pa0, .. } = (inEnvCache) else {
        return Err("pattern mismatch");
    };
    cache = metamodelica::Own::own(__pa0);
    Ok(cache)
}

fn getAnnotationString(
    mut inAnnotation: metamodelica::Ref<Absyn::Annotation>,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inFullProgram: Absyn::Program,
    mut inModelPath: metamodelica::Ref<Absyn::Path>,
) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut el: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
    if Flags::isSet(Flags::NF_API.clone())? {
        match '__try0: {
            outString = unwrap_break_err!(NFApi::evaluateAnnotation(inFullProgram.clone(), inModelPath.clone(), &inAnnotation), '__try0);
            Ok::<_, &'static str>((outString.clone(),))
        } {
            Ok((__try0_o0,)) => {
                outString = __try0_o0;
            }
            Err(_) => {
                outString = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*Dump::unparseAnnotation(inAnnotation.clone())?);
                    __mm_s.push_str(&*literal!(" "));
                    ArcStr::from(__mm_s)
                };
            }
        }
        return Ok(outString);
    }
    outString = 'mc: {
        let __mc_input = &*inAnnotation;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Annotation { elementArgs: el } => {
                    let mut outString: ArcStr = outString.clone();
                    outString = (((InteractiveUtil::getElementitemsAnnotationsElArgs(el.clone(), FGraph::emptyGraph().clone(), inClass, GraphicEnvCache::GRAPHIC_ENV_NO_CACHE { program: inFullProgram.clone(), modelPath: inModelPath.clone() }, false)?).0)).head().cloned()?;
                    Ok((outString.clone(), outString.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outString = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::unparseAnnotation(inAnnotation.clone())?); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outString)
}

pub(crate) fn keywordReplaceable(mut inAbsynRedeclareKeywordsOption: Option<Absyn::RedeclareKeywords>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match inAbsynRedeclareKeywordsOption {
        Some(Absyn::RedeclareKeywords::REPLACEABLE { .. }) => true,
        Some(Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE { .. }) => true,
        _ => false,
    });
    outBoolean
}

fn getComponentInfoOld(
    mut inElement: &metamodelica::Ref<Absyn::Element>,
    mut inEnv: GraphicEnvCache,
) -> Result<(ArcStr, metamodelica::Ref<Absyn::Path>, ArcStr)> {
    let mut componentName: ArcStr;
    let mut typeName: metamodelica::Ref<Absyn::Path>;
    let mut comment: ArcStr;
    let mut comp: metamodelica::Ref<Absyn::ComponentItem>;
    (componentName, typeName, comment) = (::match_deref::match_deref! { match inElement {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __esc_typeName, arrayDim: _ }, components: Deref @ metamodelica::ListNode::Cons { head: __esc_comp, tail: _ }, .. }, .. } => {
            typeName = (*__esc_typeName).clone();
            comp = (*__esc_comp).clone();
            componentName = comp.component.name.clone();
            typeName = InteractiveUtil::qualifyPath(inEnv, typeName.clone(), false)?;
            comment = InteractiveUtil::getComponentComment(metamodelica::AsArg::as_arg(&comp), inElement);
            (componentName, typeName.clone(), comment)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((componentName, typeName, comment))
}

pub(crate) fn transformPathedClassInProgram<'__b>(
    mut inPath: &'__b metamodelica::Ref<Absyn::Path>,
    mut inProgram: &'__b Absyn::Program,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static,
    >,
) -> Result<Absyn::Program> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static,
    >;

    '__tco: loop {
        match &**inPath {
            Absyn::Path::IDENT { .. } => {
                return Ok(transformClassInProgram(
                    var_field!((**inPath).name, Absyn::Path::IDENT),
                    inProgram.clone(),
                    &*inFunc,
                )?);
            }
            Absyn::Path::FULLYQUALIFIED { .. } => {
                (inPath, inProgram, inFunc) = (
                    var_field!((**inPath).path, Absyn::Path::FULLYQUALIFIED),
                    inProgram,
                    inFunc.clone(),
                );
                continue '__tco;
            }
            Absyn::Path::QUALIFIED { .. } => {
                return Ok(transformClassInProgram(
                    var_field!((**inPath).name, Absyn::Path::QUALIFIED),
                    inProgram.clone(),
                    &({
                        let __pe_b0 = var_field!((**inPath).path, Absyn::Path::QUALIFIED).clone();
                        let __pe_b2: Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Absyn::Class>,
                                )
                                    -> Result<metamodelica::Ref<Absyn::Class>>
                                + 'static,
                        > = inFunc.clone();
                        move |__pe_a1| transformPathedClassInClass(&__pe_b0, &__pe_a1, __pe_b2.clone())
                    }),
                )?);
            }
        }
    }
}

pub(crate) fn transformClassInProgram(
    mut inName: &ArcStr,
    mut inProgram: Absyn::Program,
    mut inFunc: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>>,
) -> Result<Absyn::Program> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static,
    >;

    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let mut acc: metamodelica::List<metamodelica::Ref<Absyn::Class>> = metamodelica::nil();
    let mut wi: Absyn::Within;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut name: ArcStr;
    let Absyn::PROGRAM {
        classes: __pa0,
        within_: __pa1,
    } = inProgram;
    classes = metamodelica::Own::own(__pa0);
    wi = metamodelica::Own::own(__pa1);
    loop {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(classes) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cls = metamodelica::Own::own(__pa2);
        classes = metamodelica::Own::own(__pa3);
        let __arc5 = cls.clone();
        let Absyn::CLASS { name: __pa4, .. } = &*__arc5;
        name = metamodelica::Own::own(__pa4);
        if metamodelica::stringEq(&name, &inName) {
            cls = inFunc(cls)?;
            classes = List::append_reverse(&acc, metamodelica::cons(cls, classes));
            outProgram = Absyn::Program {
                classes: classes,
                within_: wi,
            };
            break;
        }
        acc = metamodelica::cons(cls, acc);
    }
    Ok(outProgram)
}

fn transformPathedClassInClass<'__b>(
    mut inPath: &'__b metamodelica::Ref<Absyn::Path>,
    mut inClass: &'__b metamodelica::Ref<Absyn::Class>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static,
    >,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static,
    >;

    '__tco: loop {
        match &**inPath {
            Absyn::Path::IDENT { .. } => {
                return Ok(transformClassInClass(
                    var_field!((**inPath).name, Absyn::Path::IDENT),
                    inFunc.clone(),
                    inClass.clone(),
                )?);
            }
            Absyn::Path::QUALIFIED { .. } => {
                return Ok(transformClassInClass(
                    var_field!((**inPath).name, Absyn::Path::QUALIFIED),
                    (std::sync::Arc::new({
                        let __pe_b0 = var_field!((**inPath).path, Absyn::Path::QUALIFIED).clone();
                        let __pe_b2: Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Absyn::Class>,
                                )
                                    -> Result<metamodelica::Ref<Absyn::Class>>
                                + 'static,
                        > = inFunc.clone();
                        move |__pe_a1| transformPathedClassInClass(&__pe_b0, &__pe_a1, __pe_b2.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Absyn::Class>,
                                )
                                    -> Result<metamodelica::Ref<Absyn::Class>>
                                + 'static,
                        >),
                    inClass.clone(),
                )?);
            }
            Absyn::Path::FULLYQUALIFIED { .. } => {
                (inPath, inClass, inFunc) = (
                    var_field!((**inPath).path, Absyn::Path::FULLYQUALIFIED),
                    inClass,
                    inFunc.clone(),
                );
                continue '__tco;
            }
        }
    }
}

fn transformClassInClass(
    mut name: &ArcStr,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static,
    >,
    mut cls: metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static,
    >;

    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut body: metamodelica::Ref<Absyn::ClassDef> = cls.body.clone();
    let () = (match &*body {
        Absyn::ClassDef::PARTS {
            classParts: __body_classParts,
            ..
        } => {
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = List::findMap(__body_classParts.clone(), &({ let __pe_b0 = name.clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static> = func.clone(); move |__pe_a2| transformClassInClassPart(&__pe_b0, __pe_b1.clone(), __pe_a2) }))?.0);
            ()
        }
        Absyn::ClassDef::CLASS_EXTENDS {
            parts: __body_parts, ..
        } => {
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = List::findMap(__body_parts.clone(), &({ let __pe_b0 = name.clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static> = func.clone(); move |__pe_a2| transformClassInClassPart(&__pe_b0, __pe_b1.clone(), __pe_a2) }))?.0);
            ()
        }
        _ => (),
    });
    assign_field!(cls.body = body);
    Ok(cls)
}

fn transformClassInClassPart(
    mut name: &ArcStr,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static,
    >,
    mut part: metamodelica::Ref<Absyn::ClassPart>,
) -> Result<(metamodelica::Ref<Absyn::ClassPart>, bool)> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static,
    >;

    let mut part: metamodelica::Ref<Absyn::ClassPart> = part;
    let mut found: bool;
    found = (match &*part {
        Absyn::ClassPart::PUBLIC {
            contents: __part_contents,
        } => {
            let mut items: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            (items, found) = List::findMap(
                __part_contents.clone(),
                &({
                    let __pe_b0 = name.clone();
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>>
                            + 'static,
                    > = func.clone();
                    move |__pe_a2| transformClassInElementItem(&__pe_b0, &*__pe_b1, __pe_a2)
                }),
            )?;
            assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = items);
            found
        }
        Absyn::ClassPart::PROTECTED {
            contents: __part_contents,
        } => {
            let mut items: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            (items, found) = List::findMap(
                __part_contents.clone(),
                &({
                    let __pe_b0 = name.clone();
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>>
                            + 'static,
                    > = func.clone();
                    move |__pe_a2| transformClassInElementItem(&__pe_b0, &*__pe_b1, __pe_a2)
                }),
            )?;
            assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = items);
            found
        }
        _ => false,
    });
    Ok((part, found))
}

fn transformClassInElementItem(
    mut name: &ArcStr,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>>,
    mut item: metamodelica::Ref<Absyn::ElementItem>,
) -> Result<(metamodelica::Ref<Absyn::ElementItem>, bool)> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static,
    >;

    let mut item: metamodelica::Ref<Absyn::ElementItem> = item;
    let mut found: bool;
    found = (match &*item {
        Absyn::ElementItem::ELEMENTITEM {
            element: __item_element,
        } => {
            let mut e: metamodelica::Ref<Absyn::Element>;
            (e, found) = transformClassInElement(name, func, __item_element.clone())?;
            assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = e);
            found
        }
        _ => false,
    });
    Ok((item, found))
}

fn transformClassInElement(
    mut name: &ArcStr,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>>,
    mut element: metamodelica::Ref<Absyn::Element>,
) -> Result<(metamodelica::Ref<Absyn::Element>, bool)> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static,
    >;

    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let mut found: bool;
    found = (match &*element {
        Absyn::Element::ELEMENT {
            specification: __element_specification,
            ..
        } => {
            let mut spec: metamodelica::Ref<Absyn::ElementSpec>;
            (spec, found) = transformClassInElementSpec(name, func, __element_specification.clone())?;
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = spec);
            found
        }
        _ => false,
    });
    Ok((element, found))
}

fn transformClassInElementSpec(
    mut name: &ArcStr,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>>,
    mut spec: metamodelica::Ref<Absyn::ElementSpec>,
) -> Result<(metamodelica::Ref<Absyn::ElementSpec>, bool)> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static,
    >;

    let mut spec: metamodelica::Ref<Absyn::ElementSpec> = spec;
    let mut found: bool;
    found = (match &*spec {
        Absyn::ElementSpec::CLASSDEF { class_: cls, .. } if (metamodelica::stringEq(&cls.name, &name)) => {
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = func(cls.clone())?);
            true
        }
        _ => false,
    });
    Ok((spec, found))
}

pub(crate) fn getContainedClassAndFile(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
) -> Result<(Absyn::Program, ArcStr)> {
    let mut outProgram: Absyn::Program;
    let mut outString: ArcStr;
    (outProgram, outString) = (::match_deref::match_deref! { match &((inPath, inProgram)) {
        (classname, p) => {
            let mut cdef: metamodelica::Ref<Absyn::Class>;
            let mut filename: ArcStr;
            let mut p_1: Absyn::Program;
            let mut p_2: Absyn::Program;
            cdef = ProgramUtil::getPathedClassInProgram(classname.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
            filename = AbsynUtil::classFilename(&cdef)?;
            p_1 = getSurroundingPackage(metamodelica::AsArg::as_arg(&classname), p.clone())?;
            p_2 = removeInnerDiffFiledClasses(p_1)?;
            (p_2, filename)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outProgram, outString))
}

fn removeInnerDiffFiledClasses(mut inProgram: Absyn::Program) -> Result<Absyn::Program> {
    let mut p: Absyn::Program = inProgram;
    p = (match p.clone() {
        Absyn::Program { .. } => {
            p.classes = List::map(p.classes.clone(), &removeInnerDiffFiledClass)?;
            p
        }
    });
    Ok(p)
}

fn removeInnerDiffFiledClass(mut inClass: metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    outClass = (::match_deref::match_deref! { match &(inClass.clone()) {
        __esc_outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmt }, info: SourceInfo { fileName: file, .. }, .. } => {
            outClass = (*__esc_outClass).clone();
            let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut publst2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            publst = ProgramUtil::getPublicList(metamodelica::AsArg::as_arg(&parts));
            publst2 = removeClassDiffFiledInElementitemlist(publst, metamodelica::AsArg::as_arg(&file))?;
            parts2 = ProgramUtil::replacePublicList(metamodelica::AsArg::as_arg(&parts), publst2)?;
            assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2, ann: ann.clone(), comment: cmt.clone() }));
            outClass.clone()
        },
        __esc_outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName, modifications, parts, ann, comment: cmt }, info: SourceInfo { fileName: file, .. }, .. } => {
            outClass = (*__esc_outClass).clone();
            let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut publst2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            publst = ProgramUtil::getPublicList(metamodelica::AsArg::as_arg(&parts));
            publst2 = removeClassDiffFiledInElementitemlist(publst, metamodelica::AsArg::as_arg(&file))?;
            parts2 = ProgramUtil::replacePublicList(metamodelica::AsArg::as_arg(&parts), publst2)?;
            assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: baseClassName.clone(), modifications: modifications.clone(), comment: cmt.clone(), parts: parts2, ann: ann.clone() }));
            outClass.clone()
        },
        _ => {
            inClass
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outClass)
}

fn classIsInFile(mut inFilename: ArcStr, mut inElement: &metamodelica::Ref<Absyn::ElementItem>) -> Result<bool> {
    let mut outInFile: bool;
    outInFile = (::match_deref::match_deref! { match inElement {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { info: SourceInfo { fileName: filename, .. }, .. }, .. }, .. } } => {
            stringEq(&inFilename, &filename)
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outInFile)
}

fn removeClassDiffFiledInElementitemlist(
    mut inElements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inFilename: &ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    outElements = List::filterOnTrue(
        inElements,
        (std::sync::Arc::new({
            let __pe_b0 = inFilename.clone();
            move |__pe_a1| classIsInFile(__pe_b0.clone(), &__pe_a1)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ElementItem>) -> Result<bool> + 'static>),
    )?;
    Ok(outElements)
}

fn getSurroundingPackage(
    mut classpath: &metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
) -> Result<Absyn::Program> {
    let mut p: Absyn::Program = inProgram;
    p = 'mc: {
        let __mc_input = p.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut cdef: metamodelica::Ref<Absyn::Class>;
            let mut pdef: metamodelica::Ref<Absyn::Class>;
            let mut filename1: ArcStr;
            let mut filename2: ArcStr;
            let mut ppath: metamodelica::Ref<Absyn::Path>;
            let mut res: Absyn::Program;
            cdef = ProgramUtil::getPathedClassInProgram(classpath.clone(), &p, false, false)?;
            filename1 = AbsynUtil::classFilename(&cdef)?;
            ppath = AbsynUtil::stripLast(classpath)?;
            pdef = ProgramUtil::getPathedClassInProgram(ppath.clone(), &p, false, false)?;
            filename2 = AbsynUtil::classFilename(&pdef)?;
            let true = (stringEq(&filename1, &filename2)) else {
                return Err("pattern mismatch");
            };
            res = getSurroundingPackage(&ppath, p.clone())?;
            Ok(res.clone())
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let Absyn::Program { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut p: Absyn::Program = p.clone();
            p.classes = list![ProgramUtil::getPathedClassInProgram(
                classpath.clone(),
                &p,
                false,
                false
            )?];
            p.within_ = ProgramUtil::buildWithin(classpath.clone())?;
            Ok((p.clone(), p.clone()))
        })() {
            p = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(p)
}

pub(crate) fn transformFlatProgram(mut p: Absyn::Program) -> Result<Absyn::Program> {
    let mut newP: Absyn::Program;
    newP = (match p.clone() {
        _ => {
            (newP, _, _) = AbsynUtil::traverseClasses(
                p,
                None,
                (std::sync::Arc::new(
                    move |__a0: (
                        metamodelica::Ref<Absyn::Class>,
                        Option<metamodelica::Ref<Absyn::Path>>,
                        i32,
                    )| transformFlatClass(&__a0),
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                (
                                    metamodelica::Ref<Absyn::Class>,
                                    Option<metamodelica::Ref<Absyn::Path>>,
                                    i32,
                                ),
                            ) -> Result<(
                                metamodelica::Ref<Absyn::Class>,
                                Option<metamodelica::Ref<Absyn::Path>>,
                                i32,
                            )> + 'static,
                    >),
                0,
                true,
            )?;
            newP
        }
    });
    Ok(newP)
}

fn transformFlatClass(
    mut inTuple: &(
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<Absyn::Class>,
    Option<metamodelica::Ref<Absyn::Path>>,
    i32,
)> {
    let mut outTuple: (
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        i32,
    );
    outTuple = 'mc: {
        let __mc_input = inTuple;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cl @ Deref @ Absyn::Class { body: cdef, .. }, pa, i) => {
                    let mut cdef1: metamodelica::Ref<Absyn::ClassDef>;
                    let mut cl = (*cl).clone();
                    cdef1 = transformFlatClassDef(cdef.clone())?;
                    assign_field!(cl.body = cdef1.clone());
                    Ok((cl.clone(), pa.clone(), i.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("Interactive.transformFlatClass failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTuple)
}

fn transformFlatClassDef(mut cdef: metamodelica::Ref<Absyn::ClassDef>) -> Result<metamodelica::Ref<Absyn::ClassDef>> {
    let mut outCdef: metamodelica::Ref<Absyn::ClassDef>;
    outCdef = 'mc: {
        let __mc_input = &*cdef;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::DERIVED { .. } => {
                    Ok(cdef.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::ENUMERATION { .. } => {
                    Ok(cdef.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::OVERLOAD { .. } => {
                    Ok(cdef.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::PDER { .. } => {
                    Ok(cdef.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmt } => {
                    let mut partsTransformed: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    partsTransformed = List::map(parts.clone(), &transformFlatPart)?;
                    Ok(metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: partsTransformed.clone(), ann: ann.clone(), comment: cmt.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName, modifications, comment: cmt, ann, parts } => {
                    let mut partsTransformed: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    partsTransformed = List::map(parts.clone(), &transformFlatPart)?;
                    Ok(metamodelica::Ref::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: baseClassName.clone(), modifications: modifications.clone(), comment: cmt.clone(), parts: partsTransformed.clone(), ann: ann.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("Interactive.transformFlatClassDef failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCdef)
}

pub(crate) fn transformFlatPart(
    mut part: metamodelica::Ref<Absyn::ClassPart>,
) -> Result<metamodelica::Ref<Absyn::ClassPart>> {
    let mut outPart: metamodelica::Ref<Absyn::ClassPart>;
    outPart = 'mc: {
        let __mc_input = &*part;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::PUBLIC { contents: eitems } => {
                    let mut eitems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    eitems1 = List::map(eitems.clone(), &move |__a0: metamodelica::Ref<Absyn::ElementItem>| transformFlatElementItem(&__a0))?;
                    Ok(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: eitems1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::PROTECTED { contents: eitems } => {
                    let mut eitems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    eitems1 = List::map(eitems.clone(), &move |__a0: metamodelica::Ref<Absyn::ElementItem>| transformFlatElementItem(&__a0))?;
                    Ok(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: eitems1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::EQUATIONS { contents: eqnitems } => {
                    let mut eqnitems1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    eqnitems1 = List::map(eqnitems.clone(), &move |__a0: metamodelica::Ref<Absyn::EquationItem>| transformFlatEquationItem(&__a0))?;
                    Ok(metamodelica::Ref::new(Absyn::ClassPart::EQUATIONS { contents: eqnitems1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eqnitems } => {
                    let mut eqnitems1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    eqnitems1 = List::map(eqnitems.clone(), &move |__a0: metamodelica::Ref<Absyn::EquationItem>| transformFlatEquationItem(&__a0))?;
                    Ok(metamodelica::Ref::new(Absyn::ClassPart::INITIALEQUATIONS { contents: eqnitems1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::ALGORITHMS { contents: algitems } => {
                    let mut algitems1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
                    algitems1 = List::map(algitems.clone(), &move |__a0: metamodelica::Ref<Absyn::AlgorithmItem>| transformFlatAlgorithmItem(&__a0))?;
                    Ok(metamodelica::Ref::new(Absyn::ClassPart::ALGORITHMS { contents: algitems1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algitems } => {
                    let mut algitems1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
                    algitems1 = List::map(algitems.clone(), &move |__a0: metamodelica::Ref<Absyn::AlgorithmItem>| transformFlatAlgorithmItem(&__a0))?;
                    Ok(metamodelica::Ref::new(Absyn::ClassPart::INITIALALGORITHMS { contents: algitems1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::EXTERNAL { externalDecl: _, annotation_: _ } => {
                    Ok(part.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("Interactive.transformFlatPart failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outPart)
}

fn transformFlatElementItem(
    mut eitem: &metamodelica::Ref<Absyn::ElementItem>,
) -> Result<metamodelica::Ref<Absyn::ElementItem>> {
    let mut outEitem: metamodelica::Ref<Absyn::ElementItem>;
    outEitem = (match &**eitem {
        Absyn::ElementItem::ELEMENTITEM { element: elt } => {
            let mut elt1: metamodelica::Ref<Absyn::Element>;
            elt1 = transformFlatElement(elt.clone())?;
            metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: elt1 })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outEitem)
}

fn transformFlatElement(mut elt: metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut outElt: metamodelica::Ref<Absyn::Element>;
    outElt = (match &*elt {
        Absyn::Element::TEXT { .. } => elt,
        Absyn::Element::ELEMENT {
            finalPrefix: f,
            redeclareKeywords: r,
            innerOuter: io,
            specification: spec,
            info,
            constrainClass: constr,
        } => {
            let mut spec1: metamodelica::Ref<Absyn::ElementSpec>;
            spec1 = transformFlatElementSpec(spec.clone())?;
            metamodelica::Ref::new(Absyn::Element::ELEMENT {
                finalPrefix: f.clone(),
                redeclareKeywords: r.clone(),
                innerOuter: io.clone(),
                specification: spec1,
                info: info.clone(),
                constrainClass: constr.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outElt)
}

fn transformFlatElementSpec(
    mut eltSpec: metamodelica::Ref<Absyn::ElementSpec>,
) -> Result<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut outEltSpec: metamodelica::Ref<Absyn::ElementSpec>;
    outEltSpec = (match &*eltSpec {
        Absyn::ElementSpec::CLASSDEF {
            replaceable_: r,
            class_: cl,
        } => {
            let mut cl1: metamodelica::Ref<Absyn::Class>;
            (cl1, _, _) = transformFlatClass(&((cl.clone(), None, 0)))?;
            metamodelica::Ref::new(Absyn::ElementSpec::CLASSDEF {
                replaceable_: r.clone(),
                class_: cl1,
            })
        }
        Absyn::ElementSpec::EXTENDS {
            path,
            elementArg: eargs,
            annotationOpt: annOpt,
        } => {
            let mut eargs1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
            eargs1 = List::map(eargs.clone(), &transformFlatElementArg)?;
            metamodelica::Ref::new(Absyn::ElementSpec::EXTENDS {
                path: path.clone(),
                elementArg: eargs1,
                annotationOpt: annOpt.clone(),
            })
        }
        Absyn::ElementSpec::IMPORT { .. } => eltSpec,
        Absyn::ElementSpec::COMPONENTS {
            attributes: attr,
            typeSpec: tp,
            components: comps,
        } => {
            let mut comps1: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
            comps1 = List::map(comps.clone(), &move |__a0: metamodelica::Ref<Absyn::ComponentItem>| {
                transformFlatComponentItem(&__a0)
            })?;
            metamodelica::Ref::new(Absyn::ElementSpec::COMPONENTS {
                attributes: attr.clone(),
                typeSpec: tp.clone(),
                components: comps1,
            })
        }
    });
    Ok(outEltSpec)
}

fn transformFlatComponentItem(
    mut compitem: &metamodelica::Ref<Absyn::ComponentItem>,
) -> Result<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut outCompitem: metamodelica::Ref<Absyn::ComponentItem>;
    outCompitem = (match &**compitem {
        Absyn::ComponentItem {
            component: comp,
            condition: cond,
            comment: cmt,
        } => {
            let mut compTransformed: Absyn::Component;
            compTransformed = transformFlatComponent(comp)?;
            metamodelica::Ref::new(Absyn::ComponentItem {
                component: compTransformed,
                condition: cond.clone(),
                comment: cmt.clone(),
            })
        }
    });
    Ok(outCompitem)
}

fn transformFlatComponent(mut comp: &Absyn::Component) -> Result<Absyn::Component> {
    let mut outComp: Absyn::Component;
    outComp = (match comp.clone() {
        Absyn::Component {
            name: mut id,
            arrayDim: ref arraydim,
            modification: mut r#mod,
        } => {
            let mut arraydimTransformed: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            let mut modTransformed: Option<metamodelica::Ref<Absyn::Modification>>;
            modTransformed = transformFlatModificationOption(r#mod.clone())?;
            arraydimTransformed = transformFlatArrayDim(arraydim.clone())?;
            Absyn::Component {
                name: id.clone(),
                arrayDim: arraydimTransformed,
                modification: modTransformed,
            }
        }
    });
    Ok(outComp)
}

fn transformFlatArrayDim(
    mut ad: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>> {
    let mut outAd: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    outAd = (::match_deref::match_deref! { match &(ad.clone()) {
        _ => {
            let mut adTransformed: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            adTransformed = List::map(ad, &move |__a0: metamodelica::Ref<Absyn::Subscript>| transformFlatSubscript(&__a0))?;
            adTransformed
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAd)
}

fn transformFlatSubscript(mut s: &metamodelica::Ref<Absyn::Subscript>) -> Result<metamodelica::Ref<Absyn::Subscript>> {
    let mut outS: metamodelica::Ref<Absyn::Subscript>;
    outS = (match &**s {
        Absyn::Subscript::NOSUB { .. } => openmodelica_ast::Absyn::Subscript::interned_NOSUB(),
        Absyn::Subscript::SUBSCRIPT { subscript: e } => {
            let mut e1: metamodelica::Ref<Absyn::Exp>;
            (e1, _) = AbsynUtil::traverseExp(
                e.clone(),
                (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Exp>,
                                i32,
                            ) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)>
                            + 'static,
                    >),
                0,
            )?;
            metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: e1 })
        }
    });
    Ok(outS)
}

fn transformFlatElementArg(
    mut eltArg: metamodelica::Ref<Absyn::ElementArg>,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut outEltArg: metamodelica::Ref<Absyn::ElementArg>;
    outEltArg = (match &*eltArg {
        Absyn::ElementArg::MODIFICATION {
            finalPrefix: f,
            eachPrefix: e,
            path: p,
            modification: r#mod,
            comment: cmt,
            info,
        } => {
            let mut mod1: Option<metamodelica::Ref<Absyn::Modification>>;
            mod1 = transformFlatModificationOption(r#mod.clone())?;
            metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION {
                finalPrefix: f.clone(),
                eachPrefix: e.clone(),
                path: p.clone(),
                modification: mod1,
                comment: cmt.clone(),
                info: info.clone(),
            })
        }
        Absyn::ElementArg::REDECLARATION { .. } => eltArg,
        _ => return Err("match: no arm matched"),
    });
    Ok(outEltArg)
}

fn transformFlatModificationOption(
    mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>,
) -> Result<Option<metamodelica::Ref<Absyn::Modification>>> {
    let mut outMod: Option<metamodelica::Ref<Absyn::Modification>>;
    outMod = (::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: eltArgs, eqMod: Deref @ Absyn::EqMod::EQMOD { exp: e, info } }) => {
            let mut e1: metamodelica::Ref<Absyn::Exp>;
            let mut eltArgs1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
            eltArgs1 = List::map(eltArgs.clone(), &transformFlatElementArg)?;
            (e1, _) = AbsynUtil::traverseExp(e.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: eltArgs1, eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: e1, info: info.clone() }) }))
        },
        Some(Deref @ Absyn::Modification { elementArgLst: eltArgs, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } }) => {
            let mut eltArgs1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
            eltArgs1 = List::map(eltArgs.clone(), &transformFlatElementArg)?;
            Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: eltArgs1, eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() }))
        },
        None => {
            None
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outMod)
}

fn transformFlatComponentRef(
    mut cr: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCr: metamodelica::Ref<Absyn::ComponentRef>;
    outCr = (match &*cr {
        _ => {
            let mut cr1: metamodelica::Ref<Absyn::ComponentRef>;
            let mut ss: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            let mut s: ArcStr;
            ss = AbsynUtil::crefLastSubs(&cr)?;
            cr1 = AbsynUtil::crefStripLastSubs(cr)?;
            s = Dump::printComponentRefStr(&cr1)?;
            metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                name: s,
                subscripts: ss,
            })
        }
    });
    Ok(outCr)
}

fn transformFlatEquationItem(
    mut eqnitem: &metamodelica::Ref<Absyn::EquationItem>,
) -> Result<metamodelica::Ref<Absyn::EquationItem>> {
    let mut outEqnitem: metamodelica::Ref<Absyn::EquationItem>;
    outEqnitem = (match &**eqnitem {
        Absyn::EquationItem::EQUATIONITEM {
            equation_: eqn,
            comment: cmt,
            info,
        } => {
            let mut eqn1: metamodelica::Ref<Absyn::Equation>;
            eqn1 = transformFlatEquation(eqn)?;
            metamodelica::Ref::new(Absyn::EquationItem::EQUATIONITEM {
                equation_: eqn1,
                comment: cmt.clone(),
                info: info.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outEqnitem)
}

fn transformFlatEquation(mut eqn: &metamodelica::Ref<Absyn::Equation>) -> Result<metamodelica::Ref<Absyn::Equation>> {
    let mut outEqn: metamodelica::Ref<Absyn::Equation>;
    outEqn = (::match_deref::match_deref! { match eqn {
        Deref @ Absyn::Equation::EQ_IF { ifExp: e1, equationTrueItems: thenpart, elseIfBranches: elseifpart, equationElseItems: elsepart } => {
            let mut e11: metamodelica::Ref<Absyn::Exp>;
            let mut thenpart1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elsepart1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elseifpart1: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>)>;
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            thenpart1 = List::map(thenpart.clone(), &move |__a0: metamodelica::Ref<Absyn::EquationItem>| transformFlatEquationItem(&__a0))?;
            elsepart1 = List::map(elsepart.clone(), &move |__a0: metamodelica::Ref<Absyn::EquationItem>| transformFlatEquationItem(&__a0))?;
            elseifpart1 = List::map(elseifpart.clone(), &move |__a0: (metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>)| transformFlatElseIfPart(&__a0))?;
            metamodelica::Ref::new(Absyn::Equation::EQ_IF { ifExp: e11, equationTrueItems: thenpart1, elseIfBranches: elseifpart1, equationElseItems: elsepart1 })
        },
        Deref @ Absyn::Equation::EQ_EQUALS { leftSide: e1, rightSide: e2 } => {
            let mut e11: metamodelica::Ref<Absyn::Exp>;
            let mut e21: metamodelica::Ref<Absyn::Exp>;
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            (e21, _) = AbsynUtil::traverseExp(e2.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            metamodelica::Ref::new(Absyn::Equation::EQ_EQUALS { leftSide: e11, rightSide: e21 })
        },
        Deref @ Absyn::Equation::EQ_PDE { leftSide: e1, rightSide: e2, domain: cr1 } => {
            let mut e11: metamodelica::Ref<Absyn::Exp>;
            let mut e21: metamodelica::Ref<Absyn::Exp>;
            let mut cr11: metamodelica::Ref<Absyn::ComponentRef>;
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            (e21, _) = AbsynUtil::traverseExp(e2.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            cr11 = transformFlatComponentRef(cr1.clone())?;
            metamodelica::Ref::new(Absyn::Equation::EQ_PDE { leftSide: e11, rightSide: e21, domain: cr11 })
        },
        Deref @ Absyn::Equation::EQ_CONNECT { connector1: cr1, connector2: cr2 } => {
            let mut cr11: metamodelica::Ref<Absyn::ComponentRef>;
            let mut cr21: metamodelica::Ref<Absyn::ComponentRef>;
            cr11 = transformFlatComponentRef(cr1.clone())?;
            cr21 = transformFlatComponentRef(cr2.clone())?;
            metamodelica::Ref::new(Absyn::Equation::EQ_CONNECT { connector1: cr11, connector2: cr21 })
        },
        Deref @ Absyn::Equation::EQ_FOR { iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ForIterator { name: id, guardExp: None, range: Some(e1) }, tail: Deref @ metamodelica::ListNode::Nil }, forEquations: forEqns } => {
            let mut e11: metamodelica::Ref<Absyn::Exp>;
            let mut forEqns1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            forEqns1 = List::map(forEqns.clone(), &move |__a0: metamodelica::Ref<Absyn::EquationItem>| transformFlatEquationItem(&__a0))?;
            metamodelica::Ref::new(Absyn::Equation::EQ_FOR { iterators: list![metamodelica::Ref::new(Absyn::ForIterator { name: id.clone(), guardExp: None, range: Some(e11) })], forEquations: forEqns1 })
        },
        Deref @ Absyn::Equation::EQ_WHEN_E { whenExp: e1, whenEquations: whenEqns, elseWhenEquations: elseWhenEqns } => {
            let mut e11: metamodelica::Ref<Absyn::Exp>;
            let mut whenEqns1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elseWhenEqns1: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>)>;
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            elseWhenEqns1 = List::map(elseWhenEqns.clone(), &move |__a0: (metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>)| transformFlatElseIfPart(&__a0))?;
            whenEqns1 = List::map(whenEqns.clone(), &move |__a0: metamodelica::Ref<Absyn::EquationItem>| transformFlatEquationItem(&__a0))?;
            metamodelica::Ref::new(Absyn::Equation::EQ_WHEN_E { whenExp: e11, whenEquations: whenEqns1, elseWhenEquations: elseWhenEqns1 })
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionName: name, functionArgs: fargs } => {
            let mut fargs1: metamodelica::Ref<Absyn::FunctionArgs>;
            fargs1 = transformFlatFunctionArgs(fargs.clone())?;
            metamodelica::Ref::new(Absyn::Equation::EQ_NORETCALL { functionName: name.clone(), functionArgs: fargs1 })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outEqn)
}

fn transformFlatElseIfPart(
    mut elseIfPart: &(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    ),
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
)> {
    let mut outElseIfPart: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    );
    outElseIfPart = (::match_deref::match_deref! { match &(elseIfPart) {
        (e1, eqnitems) => {
            let mut e11: metamodelica::Ref<Absyn::Exp>;
            let mut eqnitems1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            eqnitems1 = List::map(eqnitems.clone(), &move |__a0: metamodelica::Ref<Absyn::EquationItem>| transformFlatEquationItem(&__a0))?;
            (e11, eqnitems1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElseIfPart)
}

fn transformFlatFunctionArgs(
    mut fargs: metamodelica::Ref<Absyn::FunctionArgs>,
) -> Result<metamodelica::Ref<Absyn::FunctionArgs>> {
    let mut outFargs: metamodelica::Ref<Absyn::FunctionArgs>;
    outFargs = (match &*fargs {
        Absyn::FunctionArgs::FUNCTIONARGS {
            args: expl,
            argNames: namedArgs,
        } => {
            let mut expl1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut namedArgs1: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
            expl1 = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
                for mut e in (expl.clone()).into_iter().cloned() {
                    let __x = (AbsynUtil::traverseExp(
                        e.clone(),
                        (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32))
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Absyn::Exp>,
                                        i32,
                                    )
                                        -> Result<(metamodelica::Ref<Absyn::Exp>, i32)>
                                    + 'static,
                            >),
                        0,
                    )?)
                    .0;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            namedArgs1 = List::map(namedArgs.clone(), &move |__a0: metamodelica::Ref<Absyn::NamedArg>| {
                transformFlatNamedArg(&__a0)
            })?;
            metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS {
                args: expl1,
                argNames: namedArgs1,
            })
        }
        Absyn::FunctionArgs::FOR_ITER_FARG { .. } => fargs,
    });
    Ok(outFargs)
}

fn transformFlatNamedArg(
    mut namedArg: &metamodelica::Ref<Absyn::NamedArg>,
) -> Result<metamodelica::Ref<Absyn::NamedArg>> {
    let mut outNamedArg: metamodelica::Ref<Absyn::NamedArg>;
    outNamedArg = (match &**namedArg {
        Absyn::NamedArg {
            argName: id,
            argValue: e1,
        } => {
            let mut e11: metamodelica::Ref<Absyn::Exp>;
            (e11, _) = AbsynUtil::traverseExp(
                e1.clone(),
                (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Exp>,
                                i32,
                            ) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)>
                            + 'static,
                    >),
                0,
            )?;
            metamodelica::Ref::new(Absyn::NamedArg {
                argName: id.clone(),
                argValue: e11,
            })
        }
    });
    Ok(outNamedArg)
}

fn transformFlatExp(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inDummy: i32,
) -> (metamodelica::Ref<Absyn::Exp>, i32) {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outDummy: i32;
    (outExp, outDummy) = 'mc: {
        let __mc_input = (&*inExp, inDummy);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: cr }, i) => {
                    let mut cr1: metamodelica::Ref<Absyn::ComponentRef>;
                    cr1 = transformFlatComponentRef(cr.clone())?;
                    Ok((metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr1.clone() }), i.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inDummy))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outDummy)
}

fn transformFlatAlgorithmItem(
    mut algitem: &metamodelica::Ref<Absyn::AlgorithmItem>,
) -> Result<metamodelica::Ref<Absyn::AlgorithmItem>> {
    let mut outAlgitem: metamodelica::Ref<Absyn::AlgorithmItem>;
    outAlgitem = (match &**algitem {
        Absyn::AlgorithmItem::ALGORITHMITEM {
            algorithm_: alg,
            comment: cmt,
            info,
        } => {
            let mut alg1: metamodelica::Ref<Absyn::Algorithm>;
            alg1 = transformFlatAlgorithm(alg)?;
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM {
                algorithm_: alg1,
                comment: cmt.clone(),
                info: info.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outAlgitem)
}

fn transformFlatAlgorithm(
    mut alg: &metamodelica::Ref<Absyn::Algorithm>,
) -> Result<metamodelica::Ref<Absyn::Algorithm>> {
    let mut outAlg: metamodelica::Ref<Absyn::Algorithm>;
    outAlg = (::match_deref::match_deref! { match alg {
        Deref @ Absyn::Algorithm::ALG_ASSIGN { assignComponent: Deref @ Absyn::Exp::CREF { componentRef: cr }, value: e1 } => {
            let mut cr1: metamodelica::Ref<Absyn::ComponentRef>;
            AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            cr1 = transformFlatComponentRef(cr.clone())?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_ASSIGN { assignComponent: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr1 }), value: e1.clone() })
        },
        Deref @ Absyn::Algorithm::ALG_ASSIGN { assignComponent: e1 @ Deref @ Absyn::Exp::TUPLE { expressions: _ }, value: e2 } => {
            let mut e11: metamodelica::Ref<Absyn::Exp>;
            let mut e21: metamodelica::Ref<Absyn::Exp>;
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            (e21, _) = AbsynUtil::traverseExp(e2.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_ASSIGN { assignComponent: e11, value: e21 })
        },
        Deref @ Absyn::Algorithm::ALG_IF { ifExp: e1, trueBranch: thenPart, elseIfAlgorithmBranch: elseIfPart, elseBranch: elsePart } => {
            let mut e11: metamodelica::Ref<Absyn::Exp>;
            let mut thenPart1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            let mut elsePart1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            let mut elseIfPart1: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>)>;
            thenPart1 = List::map(thenPart.clone(), &move |__a0: metamodelica::Ref<Absyn::AlgorithmItem>| transformFlatAlgorithmItem(&__a0))?;
            elseIfPart1 = List::map(elseIfPart.clone(), &move |__a0: (metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>)| transformFlatElseIfAlgorithm(&__a0))?;
            elsePart1 = List::map(elsePart.clone(), &move |__a0: metamodelica::Ref<Absyn::AlgorithmItem>| transformFlatAlgorithmItem(&__a0))?;
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_IF { ifExp: e11, trueBranch: thenPart1, elseIfAlgorithmBranch: elseIfPart1, elseBranch: elsePart1 })
        },
        Deref @ Absyn::Algorithm::ALG_FOR { iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ForIterator { name: id, guardExp: None, range: Some(e1) }, tail: Deref @ metamodelica::ListNode::Nil }, forBody: body } => {
            let mut e11: metamodelica::Ref<Absyn::Exp>;
            let mut body1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            body1 = List::map(body.clone(), &move |__a0: metamodelica::Ref<Absyn::AlgorithmItem>| transformFlatAlgorithmItem(&__a0))?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_FOR { iterators: list![metamodelica::Ref::new(Absyn::ForIterator { name: id.clone(), guardExp: None, range: Some(e11) })], forBody: body1 })
        },
        Deref @ Absyn::Algorithm::ALG_WHILE { boolExpr: e1, whileBody: body } => {
            let mut e11: metamodelica::Ref<Absyn::Exp>;
            let mut body1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            body1 = List::map(body.clone(), &move |__a0: metamodelica::Ref<Absyn::AlgorithmItem>| transformFlatAlgorithmItem(&__a0))?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_WHILE { boolExpr: e11, whileBody: body1 })
        },
        Deref @ Absyn::Algorithm::ALG_WHEN_A { boolExpr: e1, whenBody: body, elseWhenAlgorithmBranch: whenBranch } => {
            let mut e11: metamodelica::Ref<Absyn::Exp>;
            let mut body1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            let mut whenBranch1: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>)>;
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            body1 = List::map(body.clone(), &move |__a0: metamodelica::Ref<Absyn::AlgorithmItem>| transformFlatAlgorithmItem(&__a0))?;
            whenBranch1 = List::map(whenBranch.clone(), &move |__a0: (metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>)| transformFlatElseIfAlgorithm(&__a0))?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_WHEN_A { boolExpr: e11, whenBody: body1, elseWhenAlgorithmBranch: whenBranch1 })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: cr, functionArgs: fargs } => {
            let mut cr1: metamodelica::Ref<Absyn::ComponentRef>;
            let mut fargs1: metamodelica::Ref<Absyn::FunctionArgs>;
            cr1 = transformFlatComponentRef(cr.clone())?;
            fargs1 = transformFlatFunctionArgs(fargs.clone())?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: cr1, functionArgs: fargs1 })
        },
        Deref @ Absyn::Algorithm::ALG_BREAK { .. } => {
            openmodelica_ast::Absyn::Algorithm::interned_ALG_BREAK()
        },
        Deref @ Absyn::Algorithm::ALG_RETURN { .. } => {
            openmodelica_ast::Absyn::Algorithm::interned_ALG_RETURN()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outAlg)
}

fn transformFlatElseIfAlgorithm(
    mut elseIfbranch: &(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    ),
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
)> {
    let mut outElseIfbranch: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    );
    outElseIfbranch = (::match_deref::match_deref! { match &(elseIfbranch) {
        (e1, algitems) => {
            let mut e11: metamodelica::Ref<Absyn::Exp>;
            let mut algitems1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            (e11, _) = AbsynUtil::traverseExp(e1.clone(), (std::sync::Arc::new(fnptr!(transformFlatExp, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 0)?;
            algitems1 = List::map(algitems.clone(), &move |__a0: metamodelica::Ref<Absyn::AlgorithmItem>| transformFlatAlgorithmItem(&__a0))?;
            (e11, algitems1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElseIfbranch)
}

/* Start getDefinitions */
pub(crate) fn getDefinitions(
    mut ast: Absyn::Program,
    mut addFunctions: bool,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let mut handle: i32;
    let mut cl: metamodelica::Ref<Absyn::Class>;
    let Absyn::PROGRAM { classes: __pa0, .. } = MetaUtil::createMetaClassesInProgram(ast.clone())?;
    classes = metamodelica::Own::own(__pa0);
    handle = Print::saveAndClearBuf()?;
    Print::printBuf(literal!("(\n"))?;
    for mut c in &*classes {
        Print::printBuf(getDefinitionsClass(metamodelica::AsArg::as_arg(&c), addFunctions))?;
        Print::printBufNewLine()?;
    }
    cl = ProgramUtil::getPathedClassInProgram(
        metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("SourceInfo"),
        }),
        &ast,
        false,
        false,
    )?;
    Print::printBuf(getDefinitionsClass(&cl, false))?;
    Print::printBuf(literal!("\n\n)"))?;
    res = ValuesMake::makeString(Print::getString()?);
    Print::restoreBuf(handle)?;
    Ok(res)
}

fn getDefinitionsClass(mut class_: &metamodelica::Ref<Absyn::Class>, mut addFunctions: bool) -> ArcStr {
    let mut res: ArcStr;
    res = 'mc: {
        let __mc_input = (&**class_, addFunctions);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { name: ident, body: body @ Deref @ Absyn::ClassDef::PARTS { .. }, restriction: Absyn::Restriction::R_PACKAGE { .. }, .. }, _) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    let mut ident = (*ident).clone();
                    ident = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(package ")); __mm_s.push_str(&*ident); ArcStr::from(__mm_s) };
                    strs = getDefinitionParts(var_field!((**body).classParts, Absyn::ClassDef::PARTS), var_field!((**body).typeVars, Absyn::ClassDef::PARTS), addFunctions);
                    strs = metamodelica::cons(ident.clone(), strs.clone());
                    Ok(stringDelimitList(strs.clone(), literal!("\n")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { partialPrefix: true, name: ident, body: Deref @ Absyn::ClassDef::PARTS { .. }, restriction: Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE { .. } } }, .. }, _) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    strs = list![literal!("(partial impure function"), ident.clone(), literal!(")")];
                    Ok(stringDelimitList(strs.clone(), literal!(" ")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { partialPrefix: true, name: ident, body: Deref @ Absyn::ClassDef::PARTS { .. }, restriction: Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }, .. }, _) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    strs = list![literal!("(partial function"), ident.clone(), literal!(")")];
                    Ok(stringDelimitList(strs.clone(), literal!(" ")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { partialPrefix: false, name: ident, body: body @ Deref @ Absyn::ClassDef::PARTS { .. }, restriction: Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE { .. } } }, .. }, true) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    strs = getDefinitionParts(var_field!((**body).classParts, Absyn::ClassDef::PARTS), var_field!((**body).typeVars, Absyn::ClassDef::PARTS), true);
                    strs = metamodelica::cons(literal!("(impure function"), metamodelica::cons(ident.clone(), strs.clone()));
                    Ok(stringDelimitList(strs.clone(), literal!(" ")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { partialPrefix: false, name: ident, body: body @ Deref @ Absyn::ClassDef::PARTS { .. }, restriction: Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { .. } }, .. }, true) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    strs = getDefinitionParts(var_field!((**body).classParts, Absyn::ClassDef::PARTS), var_field!((**body).typeVars, Absyn::ClassDef::PARTS), true);
                    strs = metamodelica::cons(literal!("(function"), metamodelica::cons(ident.clone(), strs.clone()));
                    Ok(stringDelimitList(strs.clone(), literal!(" ")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { partialPrefix: false, name: ident, body: body @ Deref @ Absyn::ClassDef::PARTS { .. }, restriction: Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_OPERATOR_FUNCTION { .. } }, .. }, true) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    strs = getDefinitionParts(var_field!((**body).classParts, Absyn::ClassDef::PARTS), var_field!((**body).typeVars, Absyn::ClassDef::PARTS), true);
                    strs = metamodelica::cons(literal!("(operator function"), metamodelica::cons(ident.clone(), strs.clone()));
                    Ok(stringDelimitList(strs.clone(), literal!(" ")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { name: ident, body: Deref @ Absyn::ClassDef::PARTS { .. }, restriction: Absyn::Restriction::R_UNIONTYPE { .. }, .. }, _) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    strs = list![literal!("(uniontype"), ident.clone(), literal!(")")];
                    Ok(stringDelimitList(strs.clone(), literal!(" ")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { name: ident, body: body @ Deref @ Absyn::ClassDef::PARTS { .. }, restriction: Absyn::Restriction::R_RECORD { .. }, .. }, _) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    strs = getDefinitionParts(var_field!((**body).classParts, Absyn::ClassDef::PARTS), var_field!((**body).typeVars, Absyn::ClassDef::PARTS), false);
                    strs = metamodelica::cons(literal!("(record"), metamodelica::cons(ident.clone(), strs.clone()));
                    Ok(stringDelimitList(strs.clone(), literal!(" ")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { name: ident, body: body @ Deref @ Absyn::ClassDef::PARTS { .. }, restriction: Absyn::Restriction::R_METARECORD { name: path, index, .. }, .. }, _) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    let mut indexArg: ArcStr;
                    let mut pathArg: ArcStr;
                    indexArg = intString(index.clone());
                    pathArg = AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path));
                    strs = getDefinitionParts(var_field!((**body).classParts, Absyn::ClassDef::PARTS), var_field!((**body).typeVars, Absyn::ClassDef::PARTS), false);
                    strs = metamodelica::cons(literal!("(metarecord"), metamodelica::cons(ident.clone(), metamodelica::cons(indexArg.clone(), metamodelica::cons(pathArg.clone(), strs.clone()))));
                    Ok(stringDelimitList(strs.clone(), literal!(" ")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { name: ident, body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: ts, attributes: attr, .. }, .. }, _) => {
                    let mut tyStr: ArcStr;
                    let mut strs: metamodelica::List<ArcStr>;
                    let mut numDim: i32;
                    numDim = getDefinitionDimensions(metamodelica::AsArg::as_arg(&ts), metamodelica::AsArg::as_arg(&attr));
                    tyStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*if (numDim == 0) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(numDim)); ArcStr::from(__mm_s) }}); __mm_s.push_str(&*getDefinitionTypeSpecPathString(metamodelica::AsArg::as_arg(&ts))?); ArcStr::from(__mm_s) };
                    strs = list![literal!("(type"), ident.clone(), tyStr.clone(), literal!(")")];
                    Ok(stringDelimitList(strs.clone(), literal!(" ")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
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

fn getDefinitionsReplaceableClass(mut class_: &metamodelica::Ref<Absyn::Class>) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = (::match_deref::match_deref! { match class_ {
        Deref @ Absyn::Class { name: ident, body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" }, typeSpecs: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Any" }, arrayDim: None }, tail: Deref @ metamodelica::ListNode::Nil }, arrayDim: None }, .. }, restriction: Absyn::Restriction::R_TYPE { .. }, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(replaceable type ")); __mm_s.push_str(&*ident); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(res)
}

fn getDefinitionPathString(mut path: metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = AbsynUtil::pathString(path, literal!("."), true, false)?;
    Ok(out)
}

pub(crate) fn getDefinitionTypeSpecPathString(mut tp: &metamodelica::Ref<Absyn::TypeSpec>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = 'mc: {
        let __mc_input = &**tp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::TypeSpec::TCOMPLEX { path: p, typeSpecs: Deref @ metamodelica::ListNode::Nil, .. } => {
                    Ok(getDefinitionPathString(p.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::TypeSpec::TCOMPLEX { path: p, typeSpecs: tspecs, .. } => {
                    let mut tspecsStr: metamodelica::List<ArcStr>;
                    tspecsStr = List::map(tspecs.clone(), &move |__a0: metamodelica::Ref<Absyn::TypeSpec>| getDefinitionTypeSpecPathString(&__a0))?;
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*getDefinitionPathString(p.clone())?); __mm_s.push_str(&*literal!("<")); __mm_s.push_str(&*stringDelimitList(tspecsStr.clone(), literal!(","))); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::TypeSpec::TPATH { path: p, .. } => {
                    Ok(getDefinitionPathString(p.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(s)
}

fn getDefinitionDimensions(mut ts: &metamodelica::Ref<Absyn::TypeSpec>, mut attr: &Absyn::ElementAttributes) -> i32 {
    let mut out: i32;
    out = (::match_deref::match_deref! { match &((&**ts, attr)) {
        (Deref @ Absyn::TypeSpec::TPATH { arrayDim: Some(l1), .. }, Absyn::ElementAttributes { arrayDim: l2, .. }) => {
            ((l1).len() as i32) + ((l2).len() as i32)
        },
        (Deref @ Absyn::TypeSpec::TCOMPLEX { arrayDim: Some(l1), .. }, Absyn::ElementAttributes { arrayDim: l2, .. }) => {
            ((l1).len() as i32) + ((l2).len() as i32)
        },
        (Deref @ Absyn::TypeSpec::TPATH { arrayDim: None, .. }, Absyn::ElementAttributes { arrayDim: l2, .. }) => {
            ((l2).len() as i32)
        },
        (Deref @ Absyn::TypeSpec::TCOMPLEX { arrayDim: None, .. }, Absyn::ElementAttributes { arrayDim: l2, .. }) => {
            ((l2).len() as i32)
        },
        _ => {
            0
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn getDefinitionParts(
    mut parts: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inTypeVars: &metamodelica::List<ArcStr>,
    mut isFunction: bool,
) -> metamodelica::List<ArcStr> {
    let mut res: metamodelica::List<ArcStr>;
    res = 'mc: {
        let __mc_input = &**parts;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(getDefinitionTypeVars(inTypeVars.clone(), list![literal!(")")]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents }, tail: rest } => {
                    Ok(listAppend(getDefinitionContent(metamodelica::AsArg::as_arg(&contents), isFunction, true)?, getDefinitionParts(metamodelica::AsArg::as_arg(&rest), inTypeVars, isFunction)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents }, tail: rest } => {
                    Ok(listAppend(getDefinitionContent(metamodelica::AsArg::as_arg(&contents), isFunction, false)?, getDefinitionParts(metamodelica::AsArg::as_arg(&rest), inTypeVars, isFunction)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(getDefinitionParts(metamodelica::AsArg::as_arg(&rest), inTypeVars, isFunction))
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

fn getDefinitionContent(
    mut contents: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut addFunctions: bool,
    mut isPublic: bool,
) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr> = metamodelica::nil();
    res = 'mc: {
        let __mc_input = (&**contents, addFunctions, isPublic);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: false, class_ }, .. } }, tail: rest }, _, _) => {
                    let mut r#str: ArcStr;
                    let mut res: metamodelica::List<ArcStr> = res.clone();
                    res = getDefinitionContent(metamodelica::AsArg::as_arg(&rest), addFunctions, isPublic)?;
                    r#str = getDefinitionsClass(metamodelica::AsArg::as_arg(&class_), addFunctions);
                    Ok((metamodelica::cons(r#str.clone(), res.clone()), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: true, class_ }, .. } }, tail: rest }, _, _) => {
                    let mut ident: ArcStr;
                    let mut res: metamodelica::List<ArcStr> = res.clone();
                    res = getDefinitionContent(metamodelica::AsArg::as_arg(&rest), addFunctions, isPublic)?;
                    ident = getDefinitionsReplaceableClass(metamodelica::AsArg::as_arg(&class_))?;
                    Ok((metamodelica::cons(ident.clone(), res.clone()), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { typeSpec: ts, components, attributes: attr @ Absyn::ElementAttributes { direction, variability, .. } }, .. } }, tail: rest }, _, true) => {
                    let mut typeStr: ArcStr;
                    let mut dirStr: ArcStr;
                    let mut res2: metamodelica::List<ArcStr>;
                    let mut res: metamodelica::List<ArcStr> = res.clone();
                    typeStr = getDefinitionTypeSpecPathString(metamodelica::AsArg::as_arg(&ts))?;
                    dirStr = getDefinitionDirString(direction.clone(), variability.clone(), addFunctions)?;
                    res = getDefinitionComponents(&typeStr, &dirStr, getDefinitionDimensions(metamodelica::AsArg::as_arg(&ts), metamodelica::AsArg::as_arg(&attr)), components.clone());
                    res2 = getDefinitionContent(metamodelica::AsArg::as_arg(&rest), addFunctions, isPublic)?;
                    Ok((listAppend(res.clone(), res2.clone()), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::EXTENDS { path, .. }, .. } }, tail: rest }, false, true) => {
                    let mut typeStr: ArcStr;
                    let mut res: metamodelica::List<ArcStr> = res.clone();
                    typeStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(extends ")); __mm_s.push_str(&*getDefinitionPathString(path.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    res = getDefinitionContent(metamodelica::AsArg::as_arg(&rest), addFunctions, isPublic)?;
                    Ok((metamodelica::cons(typeStr.clone(), res.clone()), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _, _) => {
                    Ok(getDefinitionContent(metamodelica::AsArg::as_arg(&rest), addFunctions, isPublic)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(res)
}

fn getDefinitionDirString(
    mut dir: Absyn::Direction,
    mut variability: Absyn::Variability,
    mut isFunction: bool,
) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = (match (dir, isFunction) {
        (Absyn::Direction::INPUT { .. }, true) => literal!("input "),
        (Absyn::Direction::OUTPUT { .. }, true) => literal!("output "),
        (_, false) => {
            if '__try0: {
                let Absyn::CONST { .. } = (variability) else {
                    break '__try0 Err::<_, _>("pattern mismatch");
                };
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            literal!("")
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(res)
}

fn getDefinitionComponents<'__b>(
    mut typeStr: &'__b ArcStr,
    mut dirStr: &'__b ArcStr,
    mut numDim: i32,
    mut components: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
) -> metamodelica::List<ArcStr> {
    let mut res: metamodelica::List<ArcStr>;
    res = (::match_deref::match_deref! { match &(components) {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: ident, arrayDim: l, .. }, .. }, tail: rest } => {
            let mut sumDim: i32;
            let mut ident = (*ident).clone();
            sumDim = numDim + ((l).len() as i32);
            ident = { let mut __mm_s = String::new(); __mm_s.push_str(&*dirStr); __mm_s.push_str(&*if (numDim == 0) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(sumDim)); ArcStr::from(__mm_s) }}); __mm_s.push_str(&*typeStr); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*ident); ArcStr::from(__mm_s) };
            ident = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*ident); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
            res = getDefinitionComponents(typeStr, dirStr, numDim, rest.clone());
            metamodelica::cons(ident.clone(), res)
        },
        rest => {
            getDefinitionComponents(typeStr, dirStr, numDim, rest.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn getDefinitionTypeVars(
    mut inTypeVars: metamodelica::List<ArcStr>,
    mut inDefinitions: metamodelica::List<ArcStr>,
) -> metamodelica::List<ArcStr> {
    let mut outDefinitions: metamodelica::List<ArcStr> = inDefinitions;
    for mut ty_var in &*inTypeVars.reverse() {
        outDefinitions = metamodelica::cons(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("(replaceable type "));
                __mm_s.push_str(&*ty_var);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            },
            outDefinitions,
        );
    }
    outDefinitions
}

/* End getDefinitions */
pub(crate) fn parseFile(
    mut fileName: ArcStr,
    mut encoding: ArcStr,
    mut updateProgram: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut topClassNamesQualified: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut parsed: Absyn::Program;
    let mut dir: ArcStr;
    let mut filename: ArcStr;
    let mut lveStarted: bool = false;
    let mut lveInstance: Option<i32> = None;
    if !(System::regularFileExists(fileName.clone())) {
        topClassNamesQualified = metamodelica::nil();
        return Ok(topClassNamesQualified);
    }
    (dir, filename) = Util::getAbsoluteDirectoryAndFile(fileName.clone())?;
    if metamodelica::stringEq(&filename, &(literal!("package.moc"))) {
        (lveStarted, lveInstance) = Parser::startLibraryVendorExecutable(dir.clone());
        if !(lveStarted) {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!("Unable to start library vendor executable.")],
            )?;
            topClassNamesQualified = metamodelica::nil();
            return Ok(topClassNamesQualified);
        }
    }
    parsed = Parser::parse(
        fileName,
        encoding,
        dir,
        lveInstance.clone(),
        Config::acceptedGrammar()?,
        Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
        Flags::getConfigBool(Flags::STRICT.clone())?,
    )?;
    parsed = MetaUtil::createMetaClassesInProgram(parsed)?;
    topClassNamesQualified = getTopQualifiedClassnames(&parsed)?;
    if lveStarted {
        Parser::stopLibraryVendorExecutable(lveInstance);
    }
    if updateProgram {
        SymbolTable::setAbsyn(ProgramUtil::updateProgram(
            parsed,
            SymbolTable::getAbsyn(),
            false,
            false,
        )?)?;
    }
    Ok(topClassNamesQualified)
}

pub(crate) fn getSCodeClassNamesRecursive(
    mut inProgram: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    paths = List::fold1(
        inProgram,
        &move |__a0: metamodelica::Ref<SCode::Element>,
               __a1: Option<metamodelica::Ref<Absyn::Path>>,
               __a2: metamodelica::List<metamodelica::Ref<Absyn::Path>>| {
            getSCodeClassNamesRecursiveWork(&__a0, __a1, __a2)
        },
        None,
        metamodelica::nil(),
    )?;
    Ok(paths)
}

fn getSCodeClassNamesRecursiveWork(
    mut inElement: &metamodelica::Ref<SCode::Element>,
    mut inPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut inAcc: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    paths = (::match_deref::match_deref! { match &((inElement.clone(), inPath)) {
        (Deref @ SCode::Element::CLASS { name, .. }, None) => {
            let mut acc = inAcc.clone();
            let mut classes: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut path: metamodelica::Ref<Absyn::Path>;
            path = metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() });
            acc = metamodelica::cons(path.clone(), acc);
            classes = SCodeUtil::getClassElements(inElement);
            acc = List::fold1(&classes, &move |__a0: metamodelica::Ref<SCode::Element>, __a1: Option<metamodelica::Ref<Absyn::Path>>, __a2: metamodelica::List<metamodelica::Ref<Absyn::Path>>| getSCodeClassNamesRecursiveWork(&__a0, __a1, __a2), Some(path), acc)?;
            acc
        },
        (Deref @ SCode::Element::CLASS { name, .. }, Some(path)) => {
            let mut acc = inAcc.clone();
            let mut classes: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut path = (*path).clone();
            path = AbsynUtil::suffixPath(metamodelica::AsArg::as_arg(&path), metamodelica::AsArg::as_arg(&name));
            acc = metamodelica::cons(path.clone(), acc);
            classes = SCodeUtil::getClassElements(inElement);
            acc = List::fold1(&classes, &move |__a0: metamodelica::Ref<SCode::Element>, __a1: Option<metamodelica::Ref<Absyn::Path>>, __a2: metamodelica::List<metamodelica::Ref<Absyn::Path>>| getSCodeClassNamesRecursiveWork(&__a0, __a1, __a2), Some(path.clone()), acc)?;
            acc
        },
        _ => {
            inAcc
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(paths)
}

pub(crate) fn getAllInheritedClasses(
    mut inClassName: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
    let mut outBaseClassNames: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    outBaseClassNames = 'mc: {
        let __mc_input = (inClassName, inProgram);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p_class, p) => {
                    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut exts: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
                    cdef = ProgramUtil::getPathedClassInProgram(p_class.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    exts = InteractiveUtil::getExtendsElementspecInClass(&cdef);
                    paths = List::map(exts.clone(), &move |__a0: metamodelica::Ref<Absyn::ElementSpec>| InteractiveUtil::getBaseClassNameFromExtends(&__a0))?;
                    Ok(paths.clone())
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
    outBaseClassNames
}

pub(crate) fn printIstmtStr(mut inStatements: &GlobalScript::Statements) -> Result<ArcStr> {
    let mut strIstmt: ArcStr;
    strIstmt = GlobalScriptDump::printIstmtsStr(inStatements)?;
    Ok(strIstmt)
}

fn getClassEnvNoElaboration(
    mut inProgram: &Absyn::Program,
    mut inClassPath: &metamodelica::Ref<Absyn::Path>,
    mut inEnv: &FCore::Graph,
) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    let mut cl: metamodelica::Ref<SCode::Element>;
    let mut id: ArcStr;
    let mut encflag: SCode::Encapsulated;
    let mut restr: SCode::Restriction;
    let mut env: FCore::Graph;
    let mut ci_state: ClassInf::State;
    let mut cache: FCore::Cache;
    let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5) = ::match_deref::match_deref! { match &(Lookup::lookupClass(&(FCore::emptyCache()), inEnv, inClassPath, None)?) {
        (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { name: __pa1, encapsulatedPrefix: __pa2, restriction: __pa3, .. }, __pa5) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa0);
    id = metamodelica::Own::own(__pa1);
    encflag = metamodelica::Own::own(__pa2);
    restr = metamodelica::Own::own(__pa3);
    cl = metamodelica::Own::own(__pa4);
    env = metamodelica::Own::own(__pa5);
    env = FGraph::openScope(env, encflag, id, FGraph::restrictionToScopeType(&restr))?;
    ci_state = ClassInfUtil::start(&restr, FGraph::getGraphName(&env)?)?;
    match '__try7: {
        (_, outEnv, _, _, _) = unwrap_break_err!(Inst::partialInstClassIn(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), cl.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0), '__try7);
        Ok::<_, &'static str>((outEnv.clone(),))
    } {
        Ok((__try7_o0,)) => {
            outEnv = __try7_o0;
        }
        Err(_) => {
            (_, outEnv, _, _, _, _, _, _, _, _, _, _) = Inst::instClassIn(
                cache.clone(),
                env.clone(),
                InnerOuter::emptyInstHierarchy().clone(),
                UnitAbsyn::noStore().clone(),
                openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
                openmodelica_frontend_types::DAE::Prefix::NOPRE,
                ci_state.clone(),
                cl.clone(),
                openmodelica_frontend_types::SCode::Visibility::PUBLIC,
                metamodelica::nil(),
                false,
                openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL,
                ConnectionGraph::EMPTY().clone(),
                Connect::emptySet().clone(),
                None,
            )?;
        }
    }
    Ok(outEnv)
}

pub(crate) fn setComponentDimensions(
    mut inClass: metamodelica::Ref<Absyn::Path>,
    mut inComponentName: &metamodelica::Ref<Absyn::Path>,
    mut inDimensions: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inProgram: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut outProgram: Absyn::Program;
    let mut outResult: bool;
    let mut within_: Absyn::Within;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    match '__try0: {
        within_ = unwrap_break_err!(ProgramUtil::buildWithin(inClass.clone()), '__try0);
        cls =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(inClass.clone(), &inProgram, false, false), '__try0);
        cls = unwrap_break_err!(setComponentDimensionsInClass(cls.clone(), inComponentName, inDimensions), '__try0);
        outProgram = unwrap_break_err!(ProgramUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: within_.clone() }, inProgram.clone(), false, false), '__try0);
        outResult = true;
        Ok::<_, &'static str>((outProgram.clone(), outResult.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outProgram = __try0_o0;
            outResult = __try0_o1;
        }
        Err(_) => {
            outProgram = inProgram.clone();
            outResult = false;
        }
    }
    (outProgram, outResult)
}

fn setComponentDimensionsInClass(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inComponentName: &metamodelica::Ref<Absyn::Path>,
    mut inDimensions: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class> = inClass.clone();
    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::traverseClassComponents(inClass, (std::sync::Arc::new({ let __pe_b2 = inComponentName.clone(); let __pe_b3 = inDimensions.clone(); move |__pe_a0, __pe_a1| setComponentDimensionsInCompitems(__pe_a0, __pe_a1, &__pe_b2, __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, bool) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, bool, bool)> + 'static>), false)?) {
        (__pa0, true) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outClass = metamodelica::Own::own(__pa0);
    Ok(outClass)
}

fn setComponentDimensionsInCompitems(
    mut inComponents: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut inFound: bool,
    mut inComponentName: &metamodelica::Ref<Absyn::Path>,
    mut inDimensions: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, bool, bool)> {
    let mut outComponents: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> = metamodelica::nil();
    let mut outFound: bool;
    let mut outContinue: bool;
    let mut item: metamodelica::Ref<Absyn::ComponentItem>;
    let mut rest_items: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> = inComponents.clone();
    let mut comp: Absyn::Component;
    let mut comp_id: ArcStr;
    comp_id = AbsynUtil::pathFirstIdent(inComponentName);
    while !((rest_items).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_items) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        item = metamodelica::Own::own(__pa0);
        rest_items = metamodelica::Own::own(__pa1);
        if metamodelica::stringEq(&(AbsynUtil::componentName(&item)?), &comp_id) {
            let () = (match &*item {
                Absyn::ComponentItem {
                    component: __esc_comp @ Absyn::Component { .. },
                    ..
                } => {
                    comp = (*__esc_comp).clone();
                    comp.arrayDim = List::map(
                        inDimensions,
                        &fnptr!(AbsynUtil::makeSubscript, metamodelica::Ref<Absyn::Exp>),
                    )?;
                    assign_field!(item.component = comp.clone());
                    ()
                }
            });
            outComponents = List::append_reverse(&outComponents, metamodelica::cons(item, rest_items));
            outFound = true;
            outContinue = false;
            return Ok((outComponents, outFound, outContinue));
        }
        outComponents = metamodelica::cons(item, outComponents);
    }
    outComponents = inComponents;
    outFound = false;
    outContinue = true;
    Ok((outComponents, outFound, outContinue))
}

pub(crate) fn getInstantiatedParametersAndValues(mut odae: Option<DAE::DAElist>) -> Result<metamodelica::List<ArcStr>> {
    let mut parametersAndValues: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut els: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut params: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut strs: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut s: ArcStr;
    parametersAndValues = (match odae {
        Some(DAE::DAElist {
            elementLst: ref __esc_els,
        }) => {
            els = __esc_els.clone();
            params = DAEUtil::getParameters(els.clone(), metamodelica::nil());
            for mut p in &*params {
                strs = (::match_deref::match_deref! { match &(p.clone()) {
                    Deref @ DAE::Element::VAR { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_s, .. }, binding: __p_binding, .. } => {
                        s = (*__esc_s).clone();
                        metamodelica::cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*DAEDump::dumpVarBindingStr(__p_binding.clone())?); ArcStr::from(__mm_s) }, strs)
                    },
                    _ => strs,
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
            Dangerous::listReverseInPlace(strs)
        }
        _ => strs,
    });
    Ok(parametersAndValues)
}

pub(crate) fn getAccessAnnotation(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut p: Absyn::Program,
) -> Result<ArcStr> {
    let mut access: ArcStr;
    access = (match p.clone() {
        _ => {
            let mut accessStr: ArcStr;
            accessStr = ProgramUtil::getNamedAnnotationExp(
                className,
                p,
                &(metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("Protection"),
                })),
                Some(literal!("")),
                &getAccessAnnotationString,
            )?;
            accessStr
        }
        _ => {
            literal!("")
        }
    });
    Ok(access)
}

fn getAccessAnnotationString(mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>) -> Result<ArcStr> {
    let mut access: ArcStr;
    access = (::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: arglst, .. }) => {
            getAccessAnnotationString2(metamodelica::AsArg::as_arg(&arglst))?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(access)
}

fn getAccessAnnotationString2<'__b>(
    mut eltArgs: &'__b metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match eltArgs {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(literal!(""))
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "access" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::CREF { componentRef: cref }, .. }, .. }), .. }, tail: _ } => {
                let mut name: ArcStr;
                return Ok(Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&cref))?)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut name: ArcStr;
                { eltArgs = xs; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn checkAccessAnnotationAndEncryption(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut p: Absyn::Program,
) -> Access {
    let mut access: Access;
    let mut fileName: ArcStr;
    let mut encryptedClass: bool;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(ProgramUtil::getPathedClassInProgram(path.clone(), &p, false, false), '__try0)) {
            Deref @ Absyn::Class { info: SourceInfo { fileName: __pa1, .. }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        fileName = metamodelica::Own::own(__pa1);
        encryptedClass = StringUtil::endsWith(fileName.clone(), literal!(".moc"));
        if encryptedClass {
            access = (::match_deref::match_deref! { match &(unwrap_break_err!(getAccessAnnotation(path.clone(), p.clone()), '__try0)) {
                Deref @ "Access.hide" => Access::hide.clone(),
                Deref @ "Access.icon" => Access::icon.clone(),
                Deref @ "Access.documentation" => Access::documentation.clone(),
                Deref @ "Access.diagram" => Access::diagram.clone(),
                Deref @ "Access.nonPackageText" => Access::nonPackageText.clone(),
                Deref @ "Access.nonPackageDuplicate" => Access::nonPackageDuplicate.clone(),
                Deref @ "Access.packageText" => Access::packageText.clone(),
                Deref @ "Access.packageDuplicate" => Access::packageDuplicate.clone(),
                _ if (!(AbsynUtil::pathIsIdent(&path))) => checkAccessAnnotationAndEncryption(unwrap_break_err!(AbsynUtil::stripLast(&path), '__try0), p.clone()),
                _ => Access::documentation.clone(),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        } else {
            access = Access::all.clone();
        }
        Ok::<_, &'static str>((access.clone(),))
    } {
        Ok((__try0_o0,)) => {
            access = __try0_o0;
        }
        Err(_) => {
            access = Access::all.clone();
        }
    }
    access
}

pub(crate) fn astContainsEncryptedClass(mut inProgram: Absyn::Program) -> Result<bool> {
    let mut containsEncryptedClass: bool = false;
    let mut classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let mut fileName: ArcStr;
    let Absyn::PROGRAM { classes: __pa0, .. } = inProgram;
    classes = metamodelica::Own::own(__pa0);
    for mut c in &*classes {
        let __pa1 = ::match_deref::match_deref! { match &(c.clone()) {
            Deref @ Absyn::Class { info: SourceInfo { fileName: __pa1, .. }, .. } => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        fileName = metamodelica::Own::own(__pa1);
        containsEncryptedClass = containsEncryptedClass || StringUtil::endsWith(fileName, literal!(".moc"));
        if containsEncryptedClass {
            break;
        }
    }
    Ok(containsEncryptedClass)
}

pub(crate) fn addEquation(
    mut clsPath: &metamodelica::Ref<Absyn::Path>,
    mut eqStr: ArcStr,
    mut isInitial: bool,
) -> bool {
    let mut success: bool = false;
    let mut program: Absyn::Program;
    let mut eq: metamodelica::Ref<Absyn::EquationItem>;
    if '__try0: {
        eq = unwrap_break_err!(Parser::stringEq(eqStr.clone(), literal!("<internal>")), '__try0);
        program = unwrap_break_err!(transformPathedClassInProgram(clsPath, &(SymbolTable::getAbsyn()), (std::sync::Arc::new({ let __pe_b0 = eq.clone(); let __pe_b1 = isInitial; move |__pe_a2| AbsynUtil::appendEquation(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static>)), '__try0);
        unwrap_break_err!(SymbolTable::setAbsyn(program.clone()), '__try0);
        success = true;
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    success
}

pub(crate) fn updateEquation(
    mut clsPath: &metamodelica::Ref<Absyn::Path>,
    mut oldEq: ArcStr,
    mut newEq: ArcStr,
    mut matchAll: bool,
    mut matchShallow: bool,
    mut matchDescription: bool,
    mut mergeDescription: bool,
) -> bool {
    let mut success: bool;
    let mut program: Absyn::Program;
    let mut old_eq: metamodelica::Ref<Absyn::EquationItem>;
    let mut new_eq: Option<metamodelica::Ref<Absyn::EquationItem>>;
    match '__try0: {
        old_eq = unwrap_break_err!(Parser::stringEq(oldEq.clone(), literal!("<internal>")), '__try0);
        new_eq = if (stringEmpty(&newEq)) {
            None
        } else {
            Some(unwrap_break_err!(Parser::stringEq(newEq.clone(), literal!("<internal>")), '__try0))
        };
        program = unwrap_break_err!(transformPathedClassInProgram(clsPath, &(SymbolTable::getAbsyn()), (std::sync::Arc::new({ let __pe_b1 = old_eq.clone(); let __pe_b2 = new_eq.clone(); let __pe_b3 = matchAll; let __pe_b4 = matchShallow; let __pe_b5 = matchDescription; let __pe_b6 = mergeDescription; move |__pe_a0| updateEquation_impl(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static>)), '__try0);
        unwrap_break_err!(SymbolTable::setAbsyn(program.clone()), '__try0);
        success = true;
        Ok::<_, &'static str>((success.clone(),))
    } {
        Ok((__try0_o0,)) => {
            success = __try0_o0;
        }
        Err(_) => {
            success = false;
        }
    }
    success
}

fn updateEquation_impl(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut oldEq: &metamodelica::Ref<Absyn::EquationItem>,
    mut newEq: Option<metamodelica::Ref<Absyn::EquationItem>>,
    mut matchAll: bool,
    mut matchShallow: bool,
    mut matchDescription: bool,
    mut mergeDescription: bool,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    fn merge_desc(
        mut oldEq: &metamodelica::Ref<Absyn::EquationItem>,
        mut newEq: metamodelica::Ref<Absyn::EquationItem>,
    ) -> Result<metamodelica::Ref<Absyn::EquationItem>> {
        let mut newEq: metamodelica::Ref<Absyn::EquationItem> = newEq;
        let mut cmt: metamodelica::Ref<Absyn::Comment>;
        let () = (::match_deref::match_deref! { match &((oldEq.clone(), newEq.clone())) {
            (Deref @ Absyn::EquationItem::EQUATIONITEM { .. }, Deref @ Absyn::EquationItem::EQUATIONITEM { .. }) => {
                if (var_field!((**oldEq).comment, Absyn::EquationItem::EQUATIONITEM)).is_some() {
                    if (var_field!((*newEq).comment, Absyn::EquationItem::EQUATIONITEM)).is_none() {
                        assign_variant_field!(newEq => Absyn::EquationItem::EQUATIONITEM; comment = var_field!((**oldEq).comment, Absyn::EquationItem::EQUATIONITEM).clone());
                    } else {
                        let __pa0 = ::match_deref::match_deref! { match &(var_field!((*newEq).comment, Absyn::EquationItem::EQUATIONITEM).clone()) {
                            Some(__pa0) => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        cmt = metamodelica::Own::own(__pa0);
                        if (cmt.annotation_).is_none() {
                            assign_field!(cmt.annotation_ = AbsynUtil::getCommentOptAnnotation(var_field!((**oldEq).comment, Absyn::EquationItem::EQUATIONITEM).clone())?);
                        } else {
                            assign_field!(cmt.comment = AbsynUtil::getCommentOptComment(var_field!((**oldEq).comment, Absyn::EquationItem::EQUATIONITEM).clone())?);
                        }
                        assign_variant_field!(newEq => Absyn::EquationItem::EQUATIONITEM; comment = Some(cmt));
                    }
                }
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(newEq)
    }

    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut part: metamodelica::Ref<Absyn::ClassPart>;
    let mut rest_parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut accum_parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
    let mut eq: metamodelica::Ref<Absyn::EquationItem>;
    let mut new_eq: metamodelica::Ref<Absyn::EquationItem>;
    let mut rest_eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut accum_eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut found: bool = false;
    let mut found_in_part: bool;
    rest_parts = AbsynUtil::getClassPartsInClass(&cls);
    while !((rest_parts).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_parts) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        part = metamodelica::Own::own(__pa0);
        rest_parts = metamodelica::Own::own(__pa1);
        rest_eqs = AbsynUtil::getEquationItemsInPart(&part);
        accum_eqs = metamodelica::nil();
        found_in_part = false;
        while !((rest_eqs).is_empty()) {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_eqs) {
                Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            eq = metamodelica::Own::own(__pa2);
            rest_eqs = metamodelica::Own::own(__pa3);
            if AbsynUtil::equationItemEqual(&eq, oldEq, matchShallow, !(matchDescription))? {
                if (newEq).is_some() {
                    new_eq = Util::getOption(newEq.clone())?;
                    if mergeDescription {
                        new_eq = merge_desc(&eq, new_eq)?;
                    }
                    accum_eqs = metamodelica::cons(new_eq, accum_eqs);
                }
                found_in_part = true;
                found = true;
                if !(matchAll) {
                    accum_eqs = List::append_reverse(&rest_eqs, accum_eqs);
                    break;
                }
            } else {
                accum_eqs = metamodelica::cons(eq, accum_eqs);
            }
        }
        if found_in_part {
            part = AbsynUtil::setEquationItemsInPart(Dangerous::listReverseInPlace(accum_eqs), part)?;
            accum_parts = metamodelica::cons(part, accum_parts);
            if !(matchAll) {
                accum_parts = List::append_reverse(&rest_parts, accum_parts);
                break;
            }
        } else {
            accum_parts = metamodelica::cons(part, accum_parts);
        }
    }
    if !(found) {
        return Err("fail");
    }
    cls = AbsynUtil::setClassPartsInClass(Dangerous::listReverseInPlace(accum_parts), cls)?;
    Ok(cls)
}
