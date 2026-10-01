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

use openmodelica_ast::Absyn;

/// - Machine states, the string contains the classname.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum State {
    UNKNOWN {
        path: metamodelica::Ref<Absyn::Path>,
    },
    OPTIMIZATION {
        path: metamodelica::Ref<Absyn::Path>,
    },
    MODEL {
        path: metamodelica::Ref<Absyn::Path>,
    },
    RECORD {
        path: metamodelica::Ref<Absyn::Path>,
    },
    BLOCK {
        path: metamodelica::Ref<Absyn::Path>,
    },
    CONNECTOR {
        path: metamodelica::Ref<Absyn::Path>,
        isExpandable: bool,
    },
    TYPE {
        path: metamodelica::Ref<Absyn::Path>,
    },
    PACKAGE {
        path: metamodelica::Ref<Absyn::Path>,
    },
    FUNCTION {
        path: metamodelica::Ref<Absyn::Path>,
        isImpure: bool,
    },
    ENUMERATION {
        path: metamodelica::Ref<Absyn::Path>,
    },
    HAS_RESTRICTIONS {
        path: metamodelica::Ref<Absyn::Path>,
        hasEquations: bool,
        hasAlgorithms: bool,
        hasConstraints: bool,
    },
    TYPE_INTEGER {
        path: metamodelica::Ref<Absyn::Path>,
    },
    TYPE_REAL {
        path: metamodelica::Ref<Absyn::Path>,
    },
    TYPE_STRING {
        path: metamodelica::Ref<Absyn::Path>,
    },
    TYPE_BOOL {
        path: metamodelica::Ref<Absyn::Path>,
    },
    TYPE_CLOCK {
        path: metamodelica::Ref<Absyn::Path>,
    },
    TYPE_ENUM {
        path: metamodelica::Ref<Absyn::Path>,
    },
    EXTERNAL_OBJ {
        path: metamodelica::Ref<Absyn::Path>,
    },
    META_TUPLE {
        path: metamodelica::Ref<Absyn::Path>,
    },
    META_LIST {
        path: metamodelica::Ref<Absyn::Path>,
    },
    META_OPTION {
        path: metamodelica::Ref<Absyn::Path>,
    },
    META_RECORD {
        path: metamodelica::Ref<Absyn::Path>,
    },
    META_UNIONTYPE {
        path: metamodelica::Ref<Absyn::Path>,
        typeVars: metamodelica::List<ArcStr>,
    },
    META_ARRAY {
        path: metamodelica::Ref<Absyn::Path>,
    },
    META_POLYMORPHIC {
        path: metamodelica::Ref<Absyn::Path>,
    },
}
impl metamodelica::gc::MMTrace for State {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            State::UNKNOWN { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::OPTIMIZATION { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::MODEL { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::RECORD { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::BLOCK { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::CONNECTOR { path, isExpandable } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isExpandable, __mmv)?;
                Ok(())
            }
            State::TYPE { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::PACKAGE { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::FUNCTION { path, isImpure } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isImpure, __mmv)?;
                Ok(())
            }
            State::ENUMERATION { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::HAS_RESTRICTIONS {
                path,
                hasEquations,
                hasAlgorithms,
                hasConstraints,
            } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasEquations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasAlgorithms, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasConstraints, __mmv)?;
                Ok(())
            }
            State::TYPE_INTEGER { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::TYPE_REAL { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::TYPE_STRING { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::TYPE_BOOL { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::TYPE_CLOCK { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::TYPE_ENUM { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::EXTERNAL_OBJ { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::META_TUPLE { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::META_LIST { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::META_OPTION { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::META_RECORD { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::META_UNIONTYPE { path, typeVars } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(typeVars, __mmv)?;
                Ok(())
            }
            State::META_ARRAY { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            State::META_POLYMORPHIC { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for State {
    fn default() -> Self {
        Self::UNKNOWN {
            path: Default::default(),
        }
    }
}
pub use self::State::{
    BLOCK, CONNECTOR, ENUMERATION, EXTERNAL_OBJ, FUNCTION, HAS_RESTRICTIONS, META_ARRAY, META_LIST, META_OPTION,
    META_POLYMORPHIC, META_RECORD, META_TUPLE, META_UNIONTYPE, MODEL, OPTIMIZATION, PACKAGE, RECORD, TYPE, TYPE_BOOL,
    TYPE_CLOCK, TYPE_ENUM, TYPE_INTEGER, TYPE_REAL, TYPE_STRING, UNKNOWN,
};

/// - Events
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Event {
    /// There are equations inside the current definition
    FOUND_EQUATION,
    /// There are algorithms inside the current definition
    FOUND_ALGORITHM,
    /// There are constranit (equations) inside the current definition
    FOUND_CONSTRAINT,
    /// There is an external declaration inside the current definition
    FOUND_EXT_DECL,
    /// A definition with elements, i.e. a long definition
    NEWDEF,
    /// A Definition that contains components
    FOUND_COMPONENT {
        /// name of the component
        name: ArcStr,
    },
}
impl metamodelica::gc::MMTrace for Event {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Event::FOUND_EQUATION => Ok(()),
            Event::FOUND_ALGORITHM => Ok(()),
            Event::FOUND_CONSTRAINT => Ok(()),
            Event::FOUND_EXT_DECL => Ok(()),
            Event::NEWDEF => Ok(()),
            Event::FOUND_COMPONENT { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Event::{FOUND_ALGORITHM, FOUND_COMPONENT, FOUND_CONSTRAINT, FOUND_EQUATION, FOUND_EXT_DECL, NEWDEF};
