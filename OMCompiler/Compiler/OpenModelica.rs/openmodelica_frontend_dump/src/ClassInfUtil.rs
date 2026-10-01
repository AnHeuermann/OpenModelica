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

use crate::AbsynUtil;
use crate::SCodeDump;
use crate::SCodeUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Print;

pub fn printStateStr(mut inState: &ClassInf::State) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inState.clone() {
        ClassInf::State::UNKNOWN { .. } => {
            literal!("unknown")
        }
        ClassInf::State::OPTIMIZATION { .. } => {
            literal!("optimization")
        }
        ClassInf::State::MODEL { .. } => {
            literal!("model")
        }
        ClassInf::State::RECORD { .. } => {
            literal!("record")
        }
        ClassInf::State::BLOCK { .. } => {
            literal!("block")
        }
        ClassInf::State::CONNECTOR { .. } => {
            literal!("connector")
        }
        ClassInf::State::TYPE { .. } => {
            literal!("type")
        }
        ClassInf::State::PACKAGE { .. } => {
            literal!("package")
        }
        ClassInf::State::FUNCTION { isImpure: true, .. } => {
            literal!("impure function")
        }
        ClassInf::State::FUNCTION { .. } => {
            literal!("function")
        }
        ClassInf::State::TYPE_INTEGER { .. } => {
            literal!("Integer")
        }
        ClassInf::State::TYPE_REAL { .. } => {
            literal!("Real")
        }
        ClassInf::State::TYPE_STRING { .. } => {
            literal!("String")
        }
        ClassInf::State::TYPE_BOOL { .. } => {
            literal!("Boolean")
        }
        ClassInf::State::TYPE_CLOCK { .. } => {
            literal!("Clock")
        }
        ClassInf::State::HAS_RESTRICTIONS {
            hasEquations: false,
            hasAlgorithms: false,
            hasConstraints: false,
            ..
        } => {
            literal!("new def")
        }
        ClassInf::State::HAS_RESTRICTIONS {
            hasEquations: mut b1,
            hasAlgorithms: mut b2,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("has"));
            __mm_s.push_str(&*if (b1.clone()) {
                literal!(" equations")
            } else {
                literal!("")
            });
            __mm_s.push_str(&*if (b2.clone()) {
                literal!(" algorithms")
            } else {
                literal!("")
            });
            __mm_s.push_str(&*if (b1.clone()) {
                literal!(" constraints")
            } else {
                literal!("")
            });
            ArcStr::from(__mm_s)
        }
        ClassInf::State::EXTERNAL_OBJ { .. } => {
            literal!("ExternalObject")
        }
        ClassInf::State::META_TUPLE { .. } => {
            literal!("tuple")
        }
        ClassInf::State::META_LIST { .. } => {
            literal!("list")
        }
        ClassInf::State::META_OPTION { .. } => {
            literal!("Option")
        }
        ClassInf::State::META_RECORD { .. } => {
            literal!("meta_record")
        }
        ClassInf::State::META_POLYMORPHIC { .. } => {
            literal!("polymorphic")
        }
        ClassInf::State::META_ARRAY { .. } => {
            literal!("meta_array")
        }
        ClassInf::State::META_UNIONTYPE { .. } => {
            literal!("uniontype")
        }
        _ => {
            literal!("#printStateStr failed#")
        }
    });
    outString
}

pub(crate) fn printState(mut inState: &ClassInf::State) -> Result<()> {
    let () = (match inState.clone() {
        ClassInf::State::UNKNOWN { path: ref p } => {
            Print::printBuf(literal!("UNKNOWN "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::OPTIMIZATION { path: ref p } => {
            Print::printBuf(literal!("OPTIMIZATION "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::MODEL { path: ref p } => {
            Print::printBuf(literal!("MODEL "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::RECORD { path: ref p } => {
            Print::printBuf(literal!("RECORD "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::BLOCK { path: ref p } => {
            Print::printBuf(literal!("BLOCK "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::CONNECTOR { path: ref p, .. } => {
            Print::printBuf(literal!("CONNECTOR "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::TYPE { path: ref p } => {
            Print::printBuf(literal!("TYPE "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::PACKAGE { path: ref p } => {
            Print::printBuf(literal!("PACKAGE "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::FUNCTION {
            path: ref p,
            isImpure: true,
        } => {
            Print::printBuf(literal!("IMPURE FUNCTION "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::FUNCTION { path: ref p, .. } => {
            Print::printBuf(literal!("FUNCTION "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::TYPE_INTEGER { path: ref p } => {
            Print::printBuf(literal!("TYPE_INTEGER "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::TYPE_REAL { path: ref p } => {
            Print::printBuf(literal!("TYPE_REAL "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::TYPE_STRING { path: ref p } => {
            Print::printBuf(literal!("TYPE_STRING "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::TYPE_BOOL { path: ref p } => {
            Print::printBuf(literal!("TYPE_BOOL "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::TYPE_CLOCK { path: ref p } => {
            Print::printBuf(literal!("TYPE_CLOCK "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            ()
        }
        ClassInf::State::HAS_RESTRICTIONS { path: ref p, .. } => {
            Print::printBuf(literal!("HAS_RESTRICTIONS "))?;
            Print::printBuf(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)?;
            Print::printBuf(printStateStr(inState))?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub fn getStateName(mut inState: &ClassInf::State) -> metamodelica::Ref<Absyn::Path> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (match inState.clone() {
        ClassInf::State::UNKNOWN { path: ref p } => p.clone(),
        ClassInf::State::OPTIMIZATION { path: ref p } => p.clone(),
        ClassInf::State::MODEL { path: ref p } => p.clone(),
        ClassInf::State::RECORD { path: ref p } => p.clone(),
        ClassInf::State::BLOCK { path: ref p } => p.clone(),
        ClassInf::State::CONNECTOR { path: ref p, .. } => p.clone(),
        ClassInf::State::TYPE { path: ref p } => p.clone(),
        ClassInf::State::PACKAGE { path: ref p } => p.clone(),
        ClassInf::State::FUNCTION { path: ref p, .. } => p.clone(),
        ClassInf::State::ENUMERATION { path: ref p } => p.clone(),
        ClassInf::State::HAS_RESTRICTIONS { path: ref p, .. } => p.clone(),
        ClassInf::State::TYPE_INTEGER { path: ref p } => p.clone(),
        ClassInf::State::TYPE_REAL { path: ref p } => p.clone(),
        ClassInf::State::TYPE_STRING { path: ref p } => p.clone(),
        ClassInf::State::TYPE_BOOL { path: ref p } => p.clone(),
        ClassInf::State::TYPE_CLOCK { path: ref p } => p.clone(),
        ClassInf::State::TYPE_ENUM { path: ref p } => p.clone(),
        ClassInf::State::EXTERNAL_OBJ { path: ref p } => p.clone(),
        ClassInf::State::META_TUPLE { path: ref p } => p.clone(),
        ClassInf::State::META_LIST { path: ref p } => p.clone(),
        ClassInf::State::META_OPTION { path: ref p } => p.clone(),
        ClassInf::State::META_RECORD { path: ref p } => p.clone(),
        ClassInf::State::META_UNIONTYPE { path: ref p, .. } => p.clone(),
        ClassInf::State::META_ARRAY { path: ref p } => p.clone(),
        ClassInf::State::META_POLYMORPHIC { path: ref p } => p.clone(),
        _ => metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("#getStateName failed#"),
        }),
    });
    outPath
}

fn printEventStr(mut inEvent: &ClassInf::Event) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match inEvent.clone() {
        ClassInf::Event::FOUND_EQUATION { .. } => {
            literal!("equation")
        }
        ClassInf::Event::FOUND_CONSTRAINT { .. } => {
            literal!("constraint")
        }
        ClassInf::Event::NEWDEF { .. } => {
            literal!("new definition")
        }
        ClassInf::Event::FOUND_COMPONENT { name: mut name } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("component "));
            __mm_s.push_str(&*name);
            ArcStr::from(__mm_s)
        }
        ClassInf::Event::FOUND_EXT_DECL { .. } => {
            literal!("external function declaration")
        }
        _ => {
            literal!("Unknown event")
        }
    });
    r#str
}

pub fn start(
    mut inRestriction: &SCode::Restriction,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<ClassInf::State> {
    let mut outState: ClassInf::State;
    outState = start_dispatch(inRestriction, AbsynUtil::makeFullyQualified(inPath))?;
    Ok(outState)
}

// Transitions
fn start_dispatch(
    mut inRestriction: &SCode::Restriction,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<ClassInf::State> {
    let mut outState: ClassInf::State;
    outState = (match inRestriction.clone() {
        SCode::Restriction::R_CLASS { .. } => {
            let mut p = inPath;
            ClassInf::State::UNKNOWN { path: p }
        }
        SCode::Restriction::R_OPTIMIZATION { .. } => {
            let mut p = inPath;
            ClassInf::State::OPTIMIZATION { path: p }
        }
        SCode::Restriction::R_MODEL { .. } => {
            let mut p = inPath;
            ClassInf::State::MODEL { path: p }
        }
        SCode::Restriction::R_RECORD { isOperator: _ } => {
            let mut p = inPath;
            ClassInf::State::RECORD { path: p }
        }
        SCode::Restriction::R_BLOCK { .. } => {
            let mut p = inPath;
            ClassInf::State::BLOCK { path: p }
        }
        SCode::Restriction::R_CONNECTOR {
            isExpandable: mut isExpandable,
        } => {
            let mut p = inPath;
            ClassInf::State::CONNECTOR {
                path: p,
                isExpandable: isExpandable.clone(),
            }
        }
        SCode::Restriction::R_TYPE { .. } => {
            let mut p = inPath;
            ClassInf::State::TYPE { path: p }
        }
        SCode::Restriction::R_PACKAGE { .. } => {
            let mut p = inPath;
            ClassInf::State::PACKAGE { path: p }
        }
        SCode::Restriction::R_FUNCTION { .. } => {
            let mut p = inPath;
            ClassInf::State::FUNCTION {
                path: p,
                isImpure: SCodeUtil::isRestrictionImpure(inRestriction, true),
            }
        }
        SCode::Restriction::R_OPERATOR { .. } => {
            let mut p = inPath;
            ClassInf::State::FUNCTION {
                path: p,
                isImpure: false,
            }
        }
        SCode::Restriction::R_ENUMERATION { .. } => {
            let mut p = inPath;
            ClassInf::State::ENUMERATION { path: p }
        }
        SCode::Restriction::R_PREDEFINED_INTEGER { .. } => {
            let mut p = inPath;
            ClassInf::State::TYPE_INTEGER { path: p }
        }
        SCode::Restriction::R_PREDEFINED_REAL { .. } => {
            let mut p = inPath;
            ClassInf::State::TYPE_REAL { path: p }
        }
        SCode::Restriction::R_PREDEFINED_STRING { .. } => {
            let mut p = inPath;
            ClassInf::State::TYPE_STRING { path: p }
        }
        SCode::Restriction::R_PREDEFINED_BOOLEAN { .. } => {
            let mut p = inPath;
            ClassInf::State::TYPE_BOOL { path: p }
        }
        SCode::Restriction::R_PREDEFINED_CLOCK { .. } => {
            let mut p = inPath;
            let true = (Config::synchronousFeaturesAllowed()?) else {
                return Err("pattern mismatch");
            };
            ClassInf::State::TYPE_CLOCK { path: p }
        }
        SCode::Restriction::R_PREDEFINED_ENUMERATION { .. } => {
            let mut p = inPath;
            ClassInf::State::TYPE_ENUM { path: p }
        }
        SCode::Restriction::R_UNIONTYPE { .. } => {
            let mut p = inPath;
            ClassInf::State::META_UNIONTYPE {
                path: p,
                typeVars: var_field!(inRestriction.typeVars, SCode::Restriction::R_UNIONTYPE).clone(),
            }
        }
        SCode::Restriction::R_METARECORD { .. } => {
            let mut p = inPath;
            ClassInf::State::META_RECORD { path: p }
        }
    });
    Ok(outState)
}

pub fn trans(mut inState: ClassInf::State, mut inEvent: ClassInf::Event) -> Result<ClassInf::State> {
    let mut outState: ClassInf::State;
    outState = (match (inState.clone(), inEvent) {
        (ClassInf::State::UNKNOWN { path: ref p }, ClassInf::Event::NEWDEF { .. }) => {
            ClassInf::State::HAS_RESTRICTIONS {
                path: p.clone(),
                hasEquations: false,
                hasAlgorithms: false,
                hasConstraints: false,
            }
        }
        (ClassInf::State::OPTIMIZATION { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::MODEL { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::RECORD { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::BLOCK { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::CONNECTOR { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::TYPE { path: ref p }, ClassInf::Event::NEWDEF { .. }) => {
            ClassInf::State::TYPE { path: p.clone() }
        }
        (ClassInf::State::PACKAGE { path: ref p }, ClassInf::Event::NEWDEF { .. }) => {
            ClassInf::State::PACKAGE { path: p.clone() }
        }
        (ClassInf::State::FUNCTION { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::ENUMERATION { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::TYPE_INTEGER { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::TYPE_REAL { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::TYPE_STRING { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::TYPE_BOOL { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::TYPE_CLOCK { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::TYPE_ENUM { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::META_UNIONTYPE { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::META_RECORD { .. }, ClassInf::Event::NEWDEF { .. }) => inState,
        (ClassInf::State::UNKNOWN { path: ref p }, ClassInf::Event::FOUND_COMPONENT { .. }) => {
            ClassInf::State::HAS_RESTRICTIONS {
                path: p.clone(),
                hasEquations: false,
                hasAlgorithms: false,
                hasConstraints: false,
            }
        }
        (ClassInf::State::OPTIMIZATION { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::MODEL { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::RECORD { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::BLOCK { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::CONNECTOR { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::TYPE { path: ref p }, ClassInf::Event::FOUND_COMPONENT { name: mut s }) => {
            if !(isBasicTypeComponentName(s.clone())) {
                Error::addMessage(
                    Error::TYPE_NOT_FROM_PREDEFINED.clone(),
                    list![AbsynUtil::pathString(p.clone(), literal!("."), true, false)?],
                )?;
                return Err("fail");
            }
            ClassInf::State::TYPE { path: p.clone() }
        }
        (ClassInf::State::PACKAGE { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::FUNCTION { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::ENUMERATION { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::HAS_RESTRICTIONS { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::TYPE_INTEGER { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::TYPE_REAL { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::TYPE_STRING { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::TYPE_BOOL { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::TYPE_CLOCK { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::TYPE_ENUM { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::META_RECORD { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::META_UNIONTYPE { .. }, ClassInf::Event::FOUND_COMPONENT { .. }) => inState,
        (ClassInf::State::UNKNOWN { path: ref p }, ClassInf::Event::FOUND_EQUATION { .. }) => {
            ClassInf::State::HAS_RESTRICTIONS {
                path: p.clone(),
                hasEquations: true,
                hasAlgorithms: false,
                hasConstraints: false,
            }
        }
        (ClassInf::State::OPTIMIZATION { .. }, ClassInf::Event::FOUND_EQUATION { .. }) => inState,
        (ClassInf::State::OPTIMIZATION { .. }, ClassInf::Event::FOUND_CONSTRAINT { .. }) => inState,
        (ClassInf::State::OPTIMIZATION { .. }, ClassInf::Event::FOUND_ALGORITHM { .. }) => inState,
        (ClassInf::State::MODEL { .. }, ClassInf::Event::FOUND_EQUATION { .. }) => inState,
        (ClassInf::State::BLOCK { .. }, ClassInf::Event::FOUND_EQUATION { .. }) => inState,
        (ClassInf::State::MODEL { .. }, ClassInf::Event::FOUND_ALGORITHM { .. }) => inState,
        (ClassInf::State::BLOCK { .. }, ClassInf::Event::FOUND_ALGORITHM { .. }) => inState,
        (ClassInf::State::FUNCTION { .. }, ClassInf::Event::FOUND_ALGORITHM { .. }) => inState,
        (
            ClassInf::State::HAS_RESTRICTIONS {
                path: ref p,
                hasAlgorithms: mut b2,
                hasConstraints: mut b3,
                ..
            },
            ClassInf::Event::FOUND_EQUATION { .. },
        ) => ClassInf::State::HAS_RESTRICTIONS {
            path: p.clone(),
            hasEquations: true,
            hasAlgorithms: b2.clone(),
            hasConstraints: b3.clone(),
        },
        (
            ClassInf::State::HAS_RESTRICTIONS {
                path: ref p,
                hasEquations: mut b1,
                hasAlgorithms: mut b2,
                ..
            },
            ClassInf::Event::FOUND_CONSTRAINT { .. },
        ) => ClassInf::State::HAS_RESTRICTIONS {
            path: p.clone(),
            hasEquations: b1.clone(),
            hasAlgorithms: b2.clone(),
            hasConstraints: true,
        },
        (
            ClassInf::State::HAS_RESTRICTIONS {
                path: ref p,
                hasEquations: mut b1,
                hasConstraints: mut b3,
                ..
            },
            ClassInf::Event::FOUND_ALGORITHM { .. },
        ) => ClassInf::State::HAS_RESTRICTIONS {
            path: p.clone(),
            hasEquations: b1.clone(),
            hasAlgorithms: true,
            hasConstraints: b3.clone(),
        },
        (ClassInf::State::FUNCTION { .. }, ClassInf::Event::FOUND_EXT_DECL { .. }) => inState,
        (_, ClassInf::Event::FOUND_EXT_DECL { .. }) => return Err("fail"),
        (_, ClassInf::Event::FOUND_EQUATION { .. }) => return Err("fail"),
        (_, ClassInf::Event::FOUND_CONSTRAINT { .. }) => return Err("fail"),
        (mut st, mut ev) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("- ClassInfUtil.trans failed: "));
                __mm_s.push_str(&*printStateStr(&st));
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*printEventStr(&ev));
                ArcStr::from(__mm_s)
            })?;
            return Err("fail");
        }
    });
    Ok(outState)
}

pub fn valid(mut inState: &ClassInf::State, mut inRestriction: &SCode::Restriction) -> Result<()> {
    let () = (match (inState.clone(), inRestriction.clone()) {
        (ClassInf::State::UNKNOWN { .. }, _) => (),
        (ClassInf::State::HAS_RESTRICTIONS { .. }, SCode::Restriction::R_CLASS { .. }) => (),
        (ClassInf::State::HAS_RESTRICTIONS { .. }, SCode::Restriction::R_MODEL { .. }) => (),
        (ClassInf::State::HAS_RESTRICTIONS { .. }, SCode::Restriction::R_OPTIMIZATION { .. }) => (),
        (ClassInf::State::MODEL { .. }, SCode::Restriction::R_MODEL { .. }) => (),
        (ClassInf::State::RECORD { .. }, SCode::Restriction::R_RECORD { isOperator: _ }) => (),
        (ClassInf::State::RECORD { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (
            ClassInf::State::HAS_RESTRICTIONS {
                hasEquations: false,
                hasConstraints: false,
                hasAlgorithms: false,
                ..
            },
            SCode::Restriction::R_RECORD { isOperator: _ },
        ) => (),
        (ClassInf::State::BLOCK { .. }, SCode::Restriction::R_BLOCK { .. }) => (),
        (ClassInf::State::MODEL { .. }, SCode::Restriction::R_MODEL { .. }) => (),
        (ClassInf::State::CONNECTOR { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (
            ClassInf::State::CONNECTOR {
                isExpandable: false, ..
            },
            SCode::Restriction::R_CONNECTOR { isExpandable: false },
        ) => (),
        (
            ClassInf::State::CONNECTOR { isExpandable: true, .. },
            SCode::Restriction::R_CONNECTOR { isExpandable: true },
        ) => (),
        (
            ClassInf::State::HAS_RESTRICTIONS {
                hasEquations: false,
                hasConstraints: false,
                hasAlgorithms: false,
                ..
            },
            SCode::Restriction::R_CONNECTOR { isExpandable: _ },
        ) => (),
        (ClassInf::State::TYPE_INTEGER { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::TYPE_REAL { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::TYPE_STRING { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::TYPE_BOOL { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::TYPE_CLOCK { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::TYPE_ENUM { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::ENUMERATION { .. }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => (),
        (ClassInf::State::TYPE { .. }, SCode::Restriction::R_CONNECTOR { .. }) => (),
        (ClassInf::State::TYPE { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (ClassInf::State::TYPE_INTEGER { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (ClassInf::State::TYPE_REAL { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (ClassInf::State::TYPE_STRING { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (ClassInf::State::TYPE_BOOL { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (ClassInf::State::TYPE_CLOCK { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (ClassInf::State::TYPE_ENUM { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (ClassInf::State::ENUMERATION { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (ClassInf::State::PACKAGE { .. }, SCode::Restriction::R_PACKAGE { .. }) => (),
        (
            ClassInf::State::HAS_RESTRICTIONS {
                hasEquations: false,
                hasConstraints: false,
                hasAlgorithms: false,
                ..
            },
            SCode::Restriction::R_PACKAGE { .. },
        ) => (),
        (ClassInf::State::FUNCTION { .. }, SCode::Restriction::R_FUNCTION { functionRestriction: _ }) => (),
        (
            ClassInf::State::HAS_RESTRICTIONS {
                hasEquations: false,
                hasConstraints: false,
                ..
            },
            SCode::Restriction::R_FUNCTION { functionRestriction: _ },
        ) => (),
        (ClassInf::State::META_TUPLE { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (ClassInf::State::META_LIST { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (ClassInf::State::META_OPTION { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (ClassInf::State::META_RECORD { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (ClassInf::State::META_ARRAY { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (ClassInf::State::META_UNIONTYPE { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub fn assertValid(
    mut inState: ClassInf::State,
    mut inRestriction: SCode::Restriction,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inState, inRestriction);
        if let Ok(__v) = (|| -> Result<_> {
            let (mut st, mut re) = __mc_input.clone() else {
                return Err("nomatch");
            };
            valid(&(st.clone()), &(re.clone()))?;
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut st, mut re) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            let mut str3: ArcStr;
            str1 = AbsynUtil::pathString(getStateName(&(st.clone())), literal!("."), true, false)?;
            str2 = printStateStr(&(st.clone()));
            str3 = SCodeDump::restrictionStringPP(re.clone())?;
            Error::addSourceMessage(
                &(Error::RESTRICTION_VIOLATION.clone()),
                list![str1.clone(), str2.clone(), str3.clone()],
                info,
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

pub fn assertTrans(
    mut inState: ClassInf::State,
    mut event: ClassInf::Event,
    mut info: &SourceInfo,
) -> Result<ClassInf::State> {
    let mut outState: ClassInf::State;
    outState = 'mc: {
        let __mc_input = inState;
        if let Ok(__v) = (|| -> Result<_> {
            let mut st = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(trans(st.clone(), event.clone())?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut st = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            let mut str3: ArcStr;
            str1 = AbsynUtil::pathString(getStateName(&(st.clone())), literal!("."), true, false)?;
            str2 = printStateStr(&(st.clone()));
            str3 = printEventStr(&event);
            Error::addSourceMessage(
                &(Error::TRANS_VIOLATION.clone()),
                list![str1.clone(), str2.clone(), str3.clone()],
                info,
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outState)
}

pub(crate) fn matchingState<'__b>(
    mut inState: &'__b ClassInf::State,
    mut inStateLst: metamodelica::List<ClassInf::State>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inState.clone(), inStateLst)) {
            (_, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(false)
            },
            (ClassInf::State::UNKNOWN { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::UNKNOWN { .. }, tail: _ }) => {
                return Ok(true)
            },
            (ClassInf::State::MODEL { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::MODEL { .. }, tail: _ }) => {
                return Ok(true)
            },
            (ClassInf::State::RECORD { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::RECORD { .. }, tail: _ }) => {
                return Ok(true)
            },
            (ClassInf::State::BLOCK { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::BLOCK { .. }, tail: _ }) => {
                return Ok(true)
            },
            (ClassInf::State::CONNECTOR { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::CONNECTOR { .. }, tail: _ }) => {
                return Ok(true)
            },
            (ClassInf::State::TYPE { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::TYPE { .. }, tail: _ }) => {
                return Ok(true)
            },
            (ClassInf::State::PACKAGE { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::PACKAGE { .. }, tail: _ }) => {
                return Ok(true)
            },
            (ClassInf::State::FUNCTION { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::FUNCTION { .. }, tail: _ }) => {
                return Ok(true)
            },
            (ClassInf::State::ENUMERATION { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::ENUMERATION { .. }, tail: _ }) => {
                return Ok(true)
            },
            (ClassInf::State::TYPE_INTEGER { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::TYPE_INTEGER { .. }, tail: _ }) => {
                return Ok(true)
            },
            (ClassInf::State::TYPE_REAL { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::TYPE_REAL { .. }, tail: _ }) => {
                return Ok(true)
            },
            (ClassInf::State::TYPE_STRING { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::TYPE_STRING { .. }, tail: _ }) => {
                return Ok(true)
            },
            (ClassInf::State::TYPE_BOOL { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::TYPE_BOOL { .. }, tail: _ }) => {
                return Ok(true)
            },
            (ClassInf::State::TYPE_CLOCK { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::TYPE_CLOCK { .. }, tail: _ }) => {
                return Ok(true)
            },
            (ClassInf::State::TYPE_ENUM { .. }, Deref @ metamodelica::ListNode::Cons { head: ClassInf::State::TYPE_ENUM { .. }, tail: _ }) => {
                return Ok(true)
            },
            (_, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }) => {
                let mut res: bool;
                { (inState, inStateLst) = (inState, rest.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn isFunction(mut inState: &ClassInf::State) -> bool {
    let mut b: bool;
    b = (match inState.clone() {
        ClassInf::State::FUNCTION { .. } => true,
        _ => false,
    });
    b
}

pub fn isFunctionOrRecord(mut inState: &ClassInf::State) -> bool {
    let mut b: bool;
    b = (match inState.clone() {
        ClassInf::State::FUNCTION { .. } => true,
        ClassInf::State::RECORD { .. } => true,
        _ => false,
    });
    b
}

pub fn isConnector(mut inState: &ClassInf::State) -> Result<()> {
    let () = (match inState.clone() {
        ClassInf::State::CONNECTOR { .. } => (),
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub(crate) static basicTypeMods: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("quantity"),
        literal!("unit"),
        literal!("displayUnit"),
        literal!("min"),
        literal!("max"),
        literal!("start"),
        literal!("fixed"),
        literal!("nominal"),
        literal!("stateSelect"),
        literal!("uncertain"),
        literal!("distribution")
    ]
});

pub fn isBasicTypeComponentName(mut name: ArcStr) -> bool {
    let mut res: bool;
    res = listMember(name, basicTypeMods.clone());
    res
}

pub fn isTypeOrRecord(mut inState: &ClassInf::State) -> bool {
    let mut outIsTypeOrRecord: bool;
    outIsTypeOrRecord = (match inState.clone() {
        ClassInf::State::TYPE { .. } => true,
        ClassInf::State::RECORD { .. } => true,
        _ => false,
    });
    outIsTypeOrRecord
}

pub fn isRecord(mut inState: &ClassInf::State) -> bool {
    let mut outIsRecord: bool;
    outIsRecord = (match inState.clone() {
        ClassInf::State::RECORD { .. } => true,
        _ => false,
    });
    outIsRecord
}

pub(crate) fn isMetaRecord(mut inState: &ClassInf::State) -> bool {
    let mut outIsRecord: bool;
    outIsRecord = (match inState.clone() {
        ClassInf::State::META_RECORD { .. } => true,
        _ => false,
    });
    outIsRecord
}
