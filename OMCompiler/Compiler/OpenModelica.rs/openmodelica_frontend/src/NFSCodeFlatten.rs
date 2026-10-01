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

use crate::NFEnvExtends;
use crate::NFSCodeDependency;
use crate::NFSCodeEnv;
use crate::NFSCodeFlattenImports;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

pub type Env = metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>;

pub(crate) fn flattenProgram(
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut cls_path: metamodelica::Ref<Absyn::Path>;
    cls_path = getLastClassNameInProgram(inProgram.clone())?;
    (outProgram, _) = flattenClassInProgram(cls_path, inProgram)?;
    Ok(outProgram)
}

fn getLastClassNameInProgram(
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outClassName: metamodelica::Ref<Absyn::Path>;
    let mut prog: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut name: ArcStr;
    prog = inProgram.reverse();
    let __pa0 = ::match_deref::match_deref! { match &(List::find(&prog, &move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isClass(&__a0)) })?) {
        Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    outClassName = metamodelica::Ref::new(Absyn::Path::IDENT { name: name });
    Ok(outClassName)
}

fn isClass(mut inClass: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outIsClass: bool;
    outIsClass = (match &**inClass {
        SCode::Element::CLASS {
            restriction: SCode::Restriction::R_FUNCTION { functionRestriction: _ },
            ..
        } => false,
        _ => true,
    });
    outIsClass
}

pub(crate) fn flattenClass(
    mut inClass: metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let __pa0 = ::match_deref::match_deref! { match &(flattenProgram(list![inClass])?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outClass = metamodelica::Own::own(__pa0);
    Ok(outClass)
}

pub fn flattenClassInProgram(
    mut inClassName: metamodelica::Ref<Absyn::Path>,
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<(metamodelica::List<metamodelica::Ref<SCode::Element>>, Env)> {
    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut outEnv: Env;
    (outProgram, outEnv) = 'mc: {
        let __mc_input = inProgram;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                prog => {
                    let mut env: Env;
                    let mut prog = (*prog).clone();
                    System::tmpTickResetIndex(0, NFSCodeEnv::tmpTickIndex.clone());
                    System::tmpTickResetIndex(1, NFSCodeEnv::extendsTickIndex.clone());
                    System::setUsesCardinality(false);
                    env = NFSCodeEnv::buildInitialEnv()?;
                    env = NFSCodeEnv::extendEnvWithClasses(metamodelica::AsArg::as_arg(&prog), env.clone())?;
                    env = NFEnvExtends::update(env.clone())?;
                    (prog, env) = NFSCodeDependency::analyse(inClassName.clone(), env.clone(), metamodelica::AsArg::as_arg(&prog))?;
                    if !(Flags::isSet(Flags::SCODE_INST.clone())?) {
                        (prog, env) = NFSCodeFlattenImports::flattenProgram(metamodelica::AsArg::as_arg(&prog), env.clone())?;
                    }
                    Ok((prog.clone(), env.clone()))
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSCodeFlatten.flattenClassInProgram failed on ")); __mm_s.push_str(&*AbsynUtil::pathString(inClassName.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outProgram, outEnv))
}

pub(crate) fn flattenCompleteProgram(
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    outProgram = 'mc: {
        let __mc_input = inProgram;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                prog => {
                    let mut env: Env;
                    let mut prog = (*prog).clone();
                    env = NFSCodeEnv::buildInitialEnv()?;
                    env = NFSCodeEnv::extendEnvWithClasses(metamodelica::AsArg::as_arg(&prog), env.clone())?;
                    env = NFEnvExtends::update(env.clone())?;
                    (prog, env) = NFSCodeFlattenImports::flattenProgram(metamodelica::AsArg::as_arg(&prog), env.clone())?;
                    Ok(prog.clone())
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
                    Debug::trace(literal!("NFSCodeFlatten.flattenCompleteProgram failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outProgram)
}
