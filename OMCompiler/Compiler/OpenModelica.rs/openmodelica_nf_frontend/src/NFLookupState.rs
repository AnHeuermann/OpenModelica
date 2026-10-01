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

use crate::NFClass as Class;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFInst as Inst;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFRestriction as Restriction;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::Flags;

pub mod LookupStateName {
    use super::*;
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum LookupStateName {
        PATH {
            path: metamodelica::Ref<Absyn::Path>,
        },
        CREF {
            cref: metamodelica::Ref<Absyn::ComponentRef>,
        },
    }
    impl metamodelica::gc::MMTrace for LookupStateName {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                LookupStateName::PATH { path } => {
                    metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                    Ok(())
                }
                LookupStateName::CREF { cref } => {
                    metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    pub use self::LookupStateName::{CREF, PATH};
    pub(crate) fn toString(mut name: &metamodelica::Ref<LookupStateName>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (match &**name {
            PATH { path: __name_path } => AbsynUtil::pathString(__name_path.clone(), literal!("."), true, false)?,
            CREF { cref: __name_cref } => Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&__name_cref))?,
        });
        Ok(r#str)
    }

    pub(crate) fn firstIdent(mut name: &metamodelica::Ref<LookupStateName>) -> Result<ArcStr> {
        let mut id: ArcStr;
        id = (match &**name {
            PATH { path: __name_path } => AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&__name_path)),
            CREF { cref: __name_cref } => AbsynUtil::crefFirstIdent(metamodelica::AsArg::as_arg(&__name_cref))?,
        });
        Ok(id)
    }

    pub(crate) fn secondIdent(mut name: &metamodelica::Ref<LookupStateName>) -> Result<ArcStr> {
        let mut id: ArcStr;
        id = (match &**name {
            PATH { path: __name_path } => AbsynUtil::pathSecondIdent(metamodelica::AsArg::as_arg(&__name_path))?,
            CREF { cref: __name_cref } => AbsynUtil::crefSecondIdent(metamodelica::AsArg::as_arg(&__name_cref))?,
        });
        Ok(id)
    }
}

pub mod LookupState {
    use super::*;
    /// LookupState is used by the lookup to keep track of what state it's in so that
    ///  the rules for composite name lookup can be enforced.
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum LookupState {
        /// The start state.
        BEGIN,
        /// A component.
        COMP,
        /// A component found in a class.
        CLASS_COMP,
        /// A class found in component.
        COMP_CLASS,
        /// A function found in component.
        COMP_FUNC,
        /// A package.
        PACKAGE,
        /// A class.
        CLASS,
        /// A function.
        FUNC,
        /// A predefined component.
        PREDEF_COMP,
        /// A predefined class.
        PREDEF_CLASS,
        IMPORT,
        /// A partial class.
        PARTIAL_CLASS,
        /// A nonconstant found in a context where a constant is required.
        NON_CONSTANT,
        /// A nonencapsulated element found in a context where encapsulated is required.
        NON_ENCAPSULATED,
        /// An error occured during lookup.
        ERROR {
            errorState: metamodelica::Ref<LookupState>,
        },
    }
    impl metamodelica::gc::MMTrace for LookupState {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                LookupState::BEGIN => Ok(()),
                LookupState::COMP => Ok(()),
                LookupState::CLASS_COMP => Ok(()),
                LookupState::COMP_CLASS => Ok(()),
                LookupState::COMP_FUNC => Ok(()),
                LookupState::PACKAGE => Ok(()),
                LookupState::CLASS => Ok(()),
                LookupState::FUNC => Ok(()),
                LookupState::PREDEF_COMP => Ok(()),
                LookupState::PREDEF_CLASS => Ok(()),
                LookupState::IMPORT => Ok(()),
                LookupState::PARTIAL_CLASS => Ok(()),
                LookupState::NON_CONSTANT => Ok(()),
                LookupState::NON_ENCAPSULATED => Ok(()),
                LookupState::ERROR { errorState } => {
                    metamodelica::gc::MMTrace::mm_accept(errorState, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl LookupState {
        pub fn interned_BEGIN() -> metamodelica::Ref<LookupState> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<LookupState>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(LookupState::BEGIN));
            (*INTERNED).clone()
        }
        pub fn interned_COMP() -> metamodelica::Ref<LookupState> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<LookupState>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(LookupState::COMP));
            (*INTERNED).clone()
        }
        pub fn interned_CLASS_COMP() -> metamodelica::Ref<LookupState> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<LookupState>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(LookupState::CLASS_COMP));
            (*INTERNED).clone()
        }
        pub fn interned_COMP_CLASS() -> metamodelica::Ref<LookupState> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<LookupState>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(LookupState::COMP_CLASS));
            (*INTERNED).clone()
        }
        pub fn interned_COMP_FUNC() -> metamodelica::Ref<LookupState> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<LookupState>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(LookupState::COMP_FUNC));
            (*INTERNED).clone()
        }
        pub fn interned_PACKAGE() -> metamodelica::Ref<LookupState> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<LookupState>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(LookupState::PACKAGE));
            (*INTERNED).clone()
        }
        pub fn interned_CLASS() -> metamodelica::Ref<LookupState> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<LookupState>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(LookupState::CLASS));
            (*INTERNED).clone()
        }
        pub fn interned_FUNC() -> metamodelica::Ref<LookupState> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<LookupState>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(LookupState::FUNC));
            (*INTERNED).clone()
        }
        pub fn interned_PREDEF_COMP() -> metamodelica::Ref<LookupState> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<LookupState>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(LookupState::PREDEF_COMP));
            (*INTERNED).clone()
        }
        pub fn interned_PREDEF_CLASS() -> metamodelica::Ref<LookupState> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<LookupState>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(LookupState::PREDEF_CLASS));
            (*INTERNED).clone()
        }
        pub fn interned_IMPORT() -> metamodelica::Ref<LookupState> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<LookupState>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(LookupState::IMPORT));
            (*INTERNED).clone()
        }
        pub fn interned_PARTIAL_CLASS() -> metamodelica::Ref<LookupState> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<LookupState>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(LookupState::PARTIAL_CLASS));
            (*INTERNED).clone()
        }
        pub fn interned_NON_CONSTANT() -> metamodelica::Ref<LookupState> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<LookupState>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(LookupState::NON_CONSTANT));
            (*INTERNED).clone()
        }
        pub fn interned_NON_ENCAPSULATED() -> metamodelica::Ref<LookupState> {
            static INTERNED: std::sync::LazyLock<metamodelica::Ref<LookupState>> =
                std::sync::LazyLock::new(|| metamodelica::Ref::new(LookupState::NON_ENCAPSULATED));
            (*INTERNED).clone()
        }
    }
    pub fn interned_BEGIN() -> metamodelica::Ref<LookupState> {
        LookupState::interned_BEGIN()
    }
    pub fn interned_COMP() -> metamodelica::Ref<LookupState> {
        LookupState::interned_COMP()
    }
    pub fn interned_CLASS_COMP() -> metamodelica::Ref<LookupState> {
        LookupState::interned_CLASS_COMP()
    }
    pub fn interned_COMP_CLASS() -> metamodelica::Ref<LookupState> {
        LookupState::interned_COMP_CLASS()
    }
    pub fn interned_COMP_FUNC() -> metamodelica::Ref<LookupState> {
        LookupState::interned_COMP_FUNC()
    }
    pub fn interned_PACKAGE() -> metamodelica::Ref<LookupState> {
        LookupState::interned_PACKAGE()
    }
    pub fn interned_CLASS() -> metamodelica::Ref<LookupState> {
        LookupState::interned_CLASS()
    }
    pub fn interned_FUNC() -> metamodelica::Ref<LookupState> {
        LookupState::interned_FUNC()
    }
    pub fn interned_PREDEF_COMP() -> metamodelica::Ref<LookupState> {
        LookupState::interned_PREDEF_COMP()
    }
    pub fn interned_PREDEF_CLASS() -> metamodelica::Ref<LookupState> {
        LookupState::interned_PREDEF_CLASS()
    }
    pub fn interned_IMPORT() -> metamodelica::Ref<LookupState> {
        LookupState::interned_IMPORT()
    }
    pub fn interned_PARTIAL_CLASS() -> metamodelica::Ref<LookupState> {
        LookupState::interned_PARTIAL_CLASS()
    }
    pub fn interned_NON_CONSTANT() -> metamodelica::Ref<LookupState> {
        LookupState::interned_NON_CONSTANT()
    }
    pub fn interned_NON_ENCAPSULATED() -> metamodelica::Ref<LookupState> {
        LookupState::interned_NON_ENCAPSULATED()
    }
    impl Default for LookupState {
        fn default() -> Self {
            Self::BEGIN
        }
    }
    pub use self::LookupState::{
        BEGIN, CLASS, CLASS_COMP, COMP, COMP_CLASS, COMP_FUNC, ERROR, FUNC, IMPORT, NON_CONSTANT, NON_ENCAPSULATED,
        PACKAGE, PARTIAL_CLASS, PREDEF_CLASS, PREDEF_COMP,
    };
    pub(crate) fn assertClass(
        mut endState: &metamodelica::Ref<LookupState>,
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut name: metamodelica::Ref<Absyn::Path>,
        mut context: i32,
        mut info: SourceInfo,
    ) -> Result<()> {
        assertState(
            endState,
            &(crate::NFLookupState::LookupState::interned_CLASS()),
            node,
            &(metamodelica::Ref::new(LookupStateName::LookupStateName::PATH { path: name })),
            context,
            info,
        )?;
        Ok(())
    }

    pub(crate) fn assertFunction(
        mut endState: &metamodelica::Ref<LookupState>,
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut name: metamodelica::Ref<Absyn::ComponentRef>,
        mut context: i32,
        mut info: SourceInfo,
    ) -> Result<()> {
        assertState(
            endState,
            &(crate::NFLookupState::LookupState::interned_FUNC()),
            node,
            &(metamodelica::Ref::new(LookupStateName::LookupStateName::CREF { cref: name })),
            context,
            info,
        )?;
        Ok(())
    }

    pub(crate) fn assertComponent(
        mut endState: &metamodelica::Ref<LookupState>,
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut name: metamodelica::Ref<Absyn::ComponentRef>,
        mut context: i32,
        mut info: SourceInfo,
    ) -> Result<()> {
        assertState(
            endState,
            &(crate::NFLookupState::LookupState::interned_COMP()),
            node,
            &(metamodelica::Ref::new(LookupStateName::LookupStateName::CREF { cref: name })),
            context,
            info,
        )?;
        Ok(())
    }

    pub(crate) fn assertImport(
        mut endState: &metamodelica::Ref<LookupState>,
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut name: metamodelica::Ref<Absyn::Path>,
        mut info: SourceInfo,
    ) -> Result<()> {
        assertState(
            endState,
            &(crate::NFLookupState::LookupState::interned_IMPORT()),
            node,
            &(metamodelica::Ref::new(LookupStateName::LookupStateName::PATH { path: name })),
            InstContext::NO_CONTEXT.clone(),
            info,
        )?;
        Ok(())
    }

    pub(crate) fn isCallableType(mut node: metamodelica::Ref<InstNode::InstNode>) -> Result<bool> {
        let mut callable: bool;
        let mut n: metamodelica::Ref<InstNode::InstNode>;
        if !(InstNode::isClass(&node)?) {
            callable = false;
            return Ok(callable);
        }
        n = InstNode::resolveInner(node);
        Inst::expand(n.clone(), InstContext::NO_CONTEXT.clone())?;
        callable = (match &*(InstNode::restriction(n.clone())?) {
            Restriction::RECORD { .. } => true,
            Restriction::OPERATOR => true,
            Restriction::ENUMERATION => true,
            Restriction::TYPE if (InstNode::isEnumerationType(n.clone())?) => true,
            _ => InstNode::isClockType(&n),
        });
        Ok(callable)
    }

    pub(crate) fn isCallableComponent(mut node: metamodelica::Ref<InstNode::InstNode>) -> Result<bool> {
        let mut callable: bool;
        callable = Class::isFunction(&(InstNode::getClass(node)?));
        Ok(callable)
    }

    pub(crate) fn isFunction(
        mut state: &metamodelica::Ref<LookupState>,
        mut node: metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<bool> {
        let mut isFunction: bool;
        isFunction = (match &**state {
            FUNC { .. } => true,
            COMP_FUNC { .. } => true,
            CLASS { .. } => isCallableType(node)?,
            COMP { .. } => isCallableComponent(node)?,
            _ => false,
        });
        Ok(isFunction)
    }

    pub(crate) fn isClass(mut state: &metamodelica::Ref<LookupState>) -> bool {
        let mut isClass: bool;
        isClass = (match &**state {
            COMP_CLASS { .. } => true,
            CLASS { .. } => true,
            PREDEF_CLASS { .. } => true,
            _ => false,
        });
        isClass
    }

    pub(crate) fn assertState(
        mut endState: &metamodelica::Ref<LookupState>,
        mut expectedState: &metamodelica::Ref<LookupState>,
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut name: &metamodelica::Ref<LookupStateName::LookupStateName>,
        mut context: i32,
        mut info: SourceInfo,
    ) -> Result<()> {
        let () = (::match_deref::match_deref! { match (endState, expectedState) {
            (Deref @ COMP { .. }, Deref @ COMP { .. }) => {
                ()
            },
            (Deref @ CLASS_COMP { .. }, Deref @ COMP { .. }) => {
                ()
            },
            (Deref @ PREDEF_COMP { .. }, Deref @ COMP { .. }) => {
                ()
            },
            (Deref @ FUNC { .. }, Deref @ COMP { .. }) => {
                ()
            },
            (Deref @ COMP_FUNC { .. }, Deref @ COMP { .. }) => {
                ()
            },
            (Deref @ PACKAGE { .. }, Deref @ CLASS { .. }) => {
                ()
            },
            (Deref @ CLASS { .. }, Deref @ CLASS { .. }) => {
                ()
            },
            (Deref @ PREDEF_CLASS { .. }, Deref @ CLASS { .. }) => {
                ()
            },
            (Deref @ FUNC { .. }, Deref @ CLASS { .. }) => {
                ()
            },
            (Deref @ FUNC { .. }, Deref @ FUNC { .. }) => {
                ()
            },
            (Deref @ COMP_FUNC { .. }, Deref @ FUNC { .. }) => {
                ()
            },
            (Deref @ CLASS { .. }, Deref @ FUNC { .. }) if (isCallableType(node.clone())?) => {
                ()
            },
            (Deref @ COMP { .. }, Deref @ FUNC { .. }) if (isCallableComponent(node.clone())?) => {
                ()
            },
            (Deref @ COMP_CLASS { .. }, Deref @ FUNC { .. }) => {
                printFoundWrongTypeError(endState, expectedState, name, &info)?;
                return Err("fail")
            },
            (Deref @ COMP_FUNC { .. }, _) => {
                let mut name_str: ArcStr;
                name_str = LookupStateName::toString(name)?;
                Error::addSourceMessage(&(Error::FOUND_FUNC_NAME_VIA_COMP_NONCALL.clone()), list![name_str], &info)?;
                return Err("fail")
            },
            (Deref @ COMP_CLASS { .. }, _) => {
                Error::addSourceMessage(&(Error::FOUND_CLASS_NAME_VIA_COMPONENT.clone()), list![LookupStateName::toString(name)?], &info)?;
                return Err("fail")
            },
            (Deref @ ERROR { errorState: Deref @ COMP_FUNC { .. } }, Deref @ FUNC { .. }) => {
                let mut name_str: ArcStr;
                let mut info2: SourceInfo;
                name_str = InstNode::name(&node)?;
                info2 = InstNode::info(&node);
                Error::addSourceMessage(&(Error::NON_CLASS_IN_COMP_FUNC_NAME.clone()), list![name_str], &info2)?;
                return Err("fail")
            },
            (Deref @ ERROR { errorState: Deref @ COMP_FUNC { .. } }, Deref @ COMP { .. }) => {
                let mut name_str: ArcStr;
                name_str = InstNode::name(&node)?;
                Error::addSourceMessage(&(Error::UNEXPECTED_COMPONENT_IN_COMPOSITE_NAME.clone()), list![name_str, LookupStateName::toString(name)?], &info)?;
                return Err("fail")
            },
            (Deref @ ERROR { errorState: Deref @ COMP_FUNC { .. } }, _) => {
                let mut name_str: ArcStr;
                name_str = InstNode::name(&node)?;
                Error::addSourceMessage(&(Error::LOOKUP_CLASS_VIA_COMP_COMP.clone()), list![name_str, LookupStateName::toString(name)?], &info)?;
                return Err("fail")
            },
            (Deref @ ERROR { errorState: Deref @ CLASS_COMP { .. } }, Deref @ COMP { .. }) => {
                let mut name_str: ArcStr;
                name_str = InstNode::name(&node)?;
                Error::addSourceMessage(&(Error::CLASS_IN_COMPOSITE_COMP_NAME.clone()), list![name_str, LookupStateName::toString(name)?], &info)?;
                return Err("fail")
            },
            (Deref @ ERROR { errorState: Deref @ CLASS_COMP { .. } }, _) => {
                let mut name_str: ArcStr;
                name_str = InstNode::name(&node)?;
                Error::addSourceMessage(&(Error::LOOKUP_CLASS_VIA_COMP_COMP.clone()), list![name_str, LookupStateName::toString(name)?], &info)?;
                return Err("fail")
            },
            (Deref @ ERROR { errorState: Deref @ IMPORT { .. } }, _) => {
                let mut name_str: ArcStr;
                name_str = InstNode::name(&node)?;
                Error::addSourceMessage(&(Error::IMPORT_IN_COMPOSITE_NAME.clone()), list![name_str, LookupStateName::toString(name)?], &info)?;
                return Err("fail")
            },
            (Deref @ ERROR { errorState: Deref @ PARTIAL_CLASS { .. } }, _) => {
                let mut node2: metamodelica::Ref<InstNode::InstNode>;
                if !(InstContext::inRelaxed(context) || InstContext::inRedeclared(context)) {
                    node2 = ((InstNode::scopeList(node.clone(), false, metamodelica::nil())?)).head().cloned()?;
                    if InstNode::isComponent(&node2)? {
                        Error::addMultiSourceMessage(&(Error::USE_OF_PARTIAL_CLASS.clone()), &(list![InstNode::name(&node2)?, InstNode::name(&node)?, AbsynUtil::pathString(Class::constrainingClassPath(node.clone())?, literal!("."), true, false)?]), &(list![InstNode::info(&node), InstNode::info(&node2)]))?;
                    } else {
                        Error::addSourceMessage(&(Error::LOOKUP_IN_PARTIAL_CLASS.clone()), list![InstNode::name(&node)?], &info)?;
                    }
                    return Err("fail");
                }
                ()
            },
            (Deref @ ERROR { errorState: Deref @ NON_CONSTANT { .. } }, _) => {
                Error::addMultiSourceMessage(&(Error::NON_CONSTANT_IN_ENCLOSING_SCOPE.clone()), &(list![InstNode::name(&node)?]), &(list![InstNode::info(&node), info]))?;
                return Err("fail")
            },
            (Deref @ ERROR { errorState: Deref @ NON_ENCAPSULATED { .. } }, _) => {
                Error::addMultiSourceMessage(&(Error::NON_ENCAPSULATED_CLASS_ACCESS.clone()), &(list![InstNode::name(&(InstNode::parent(&node)?))?, InstNode::name(&node)?]), &(list![InstNode::info(&node), info]))?;
                return Err("fail")
            },
            (_, Deref @ IMPORT { .. }) => {
                ()
            },
            _ => {
                printFoundWrongTypeError(endState, expectedState, name, &info)?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(())
    }

    pub(crate) fn isError(mut state: &metamodelica::Ref<LookupState>) -> bool {
        let mut isError: bool;
        isError = (match &**state {
            ERROR { .. } => true,
            _ => false,
        });
        isError
    }

    pub(crate) fn lookupStateString(mut state: &metamodelica::Ref<LookupState>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (match &**state {
            BEGIN { .. } => literal!("<begin>"),
            COMP { .. } => literal!("component"),
            CLASS_COMP { .. } => literal!("component"),
            COMP_CLASS { .. } => literal!("class"),
            COMP_FUNC { .. } => literal!("function"),
            PACKAGE { .. } => literal!("package"),
            CLASS { .. } => literal!("class"),
            FUNC { .. } => literal!("function"),
            PREDEF_COMP { .. } => literal!("component"),
            PREDEF_CLASS { .. } => literal!("class"),
            _ => return Err("match: no arm matched"),
        });
        Ok(r#str)
    }

    pub(crate) fn printFoundWrongTypeError(
        mut foundState: &metamodelica::Ref<LookupState>,
        mut expectedState: &metamodelica::Ref<LookupState>,
        mut name: &metamodelica::Ref<LookupStateName::LookupStateName>,
        mut info: &SourceInfo,
    ) -> Result<()> {
        let mut name_str: ArcStr;
        let mut found_str: ArcStr;
        let mut expected_str: ArcStr;
        name_str = LookupStateName::toString(name)?;
        found_str = lookupStateString(foundState)?;
        expected_str = lookupStateString(expectedState)?;
        Error::addSourceMessage(
            &(Error::LOOKUP_FOUND_WRONG_TYPE.clone()),
            list![name_str, expected_str, found_str],
            info,
        )?;
        Ok(())
    }

    pub(crate) fn next(
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut currentState: &metamodelica::Ref<LookupState>,
        mut context: i32,
        mut checkAccessViolations: bool,
    ) -> Result<metamodelica::Ref<LookupState>> {
        let mut nextState: metamodelica::Ref<LookupState>;
        let mut entry_ty: metamodelica::Ref<LookupState>;
        if checkAccessViolations && !(InstContext::inInstanceAPI(context)) {
            checkProtection(&node, currentState)?;
        }
        entry_ty = nodeState(node.clone())?;
        nextState = next2(entry_ty, currentState, &node)?;
        Ok(nextState)
    }

    pub(crate) fn checkProtection(
        mut node: &metamodelica::Ref<InstNode::InstNode>,
        mut currentState: &metamodelica::Ref<LookupState>,
    ) -> Result<()> {
        let () = (match &**currentState {
            BEGIN { .. } => (),
            _ => {
                if InstNode::isProtected(node)
                    && !(Flags::isConfigFlagSet(
                        Flags::ALLOW_NON_STANDARD_MODELICA.clone(),
                        literal!("protectedAccess"),
                    )?)
                {
                    Error::addSourceMessage(
                        &(Error::PROTECTED_ACCESS.clone()),
                        list![InstNode::name(node)?],
                        &(InstNode::info(node)),
                    )?;
                    return Err("fail");
                }
                ()
            }
        });
        Ok(())
    }

    pub(crate) fn nodeState(mut node: metamodelica::Ref<InstNode::InstNode>) -> Result<metamodelica::Ref<LookupState>> {
        let mut state: metamodelica::Ref<LookupState>;
        if InstNode::isComponent(&node)? || InstNode::isName(&node) || InstNode::isEmpty(&node) {
            state = crate::NFLookupState::LookupState::interned_COMP();
        } else {
            state = elementState(&(InstNode::definition(node)?))?;
        }
        Ok(state)
    }

    pub(crate) fn elementState(
        mut element: &metamodelica::Ref<SCode::Element>,
    ) -> Result<metamodelica::Ref<LookupState>> {
        let mut state: metamodelica::Ref<LookupState>;
        state = (match &**element {
            SCode::Element::CLASS {
                restriction: SCode::Restriction::R_PACKAGE { .. },
                ..
            } => crate::NFLookupState::LookupState::interned_PACKAGE(),
            SCode::Element::CLASS {
                restriction: SCode::Restriction::R_FUNCTION { .. },
                ..
            } => crate::NFLookupState::LookupState::interned_FUNC(),
            SCode::Element::CLASS { .. } => crate::NFLookupState::LookupState::interned_CLASS(),
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFLookupState.LookupState.elementState"));
                        __mm_s.push_str(&*literal!(" got unknown element."));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFLookupState.mo")),
                )?;
                return Err("fail");
            }
        });
        Ok(state)
    }

    pub(crate) fn next2(
        mut elementState: metamodelica::Ref<LookupState>,
        mut currentState: &metamodelica::Ref<LookupState>,
        mut node: &metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<metamodelica::Ref<LookupState>> {
        let mut nextState: metamodelica::Ref<LookupState>;
        nextState = (::match_deref::match_deref! { match &((elementState.clone(), currentState.clone())) {
            (_, Deref @ BEGIN { .. }) => elementState,
            (Deref @ COMP { .. }, Deref @ COMP { .. }) => crate::NFLookupState::LookupState::interned_COMP(),
            (Deref @ FUNC { .. }, Deref @ COMP { .. }) => crate::NFLookupState::LookupState::interned_COMP_FUNC(),
            (_, Deref @ COMP { .. }) => crate::NFLookupState::LookupState::interned_COMP_CLASS(),
            (Deref @ COMP { .. }, Deref @ CLASS_COMP { .. }) => crate::NFLookupState::LookupState::interned_CLASS_COMP(),
            (Deref @ CLASS_COMP { .. }, Deref @ CLASS_COMP { .. }) => crate::NFLookupState::LookupState::interned_CLASS_COMP(),
            (Deref @ COMP { .. }, Deref @ PACKAGE { .. }) => crate::NFLookupState::LookupState::interned_CLASS_COMP(),
            (_, Deref @ PACKAGE { .. }) => elementState,
            (Deref @ COMP { .. }, Deref @ CLASS { .. }) => crate::NFLookupState::LookupState::interned_CLASS_COMP(),
            (_, Deref @ CLASS { .. }) => elementState,
            (Deref @ COMP { .. }, Deref @ FUNC { .. }) => crate::NFLookupState::LookupState::interned_CLASS_COMP(),
            (_, Deref @ FUNC { .. }) => elementState,
            (Deref @ FUNC { .. }, Deref @ COMP_CLASS { .. }) => crate::NFLookupState::LookupState::interned_COMP_FUNC(),
            (Deref @ CLASS { .. }, Deref @ COMP_CLASS { .. }) => crate::NFLookupState::LookupState::interned_COMP_CLASS(),
            (Deref @ PACKAGE { .. }, Deref @ COMP_CLASS { .. }) => crate::NFLookupState::LookupState::interned_COMP_CLASS(),
            (Deref @ FUNC { .. }, Deref @ COMP_FUNC { .. }) => crate::NFLookupState::LookupState::interned_COMP_FUNC(),
            (Deref @ CLASS { .. }, Deref @ COMP_FUNC { .. }) => crate::NFLookupState::LookupState::interned_COMP_CLASS(),
            (Deref @ PACKAGE { .. }, Deref @ COMP_FUNC { .. }) => crate::NFLookupState::LookupState::interned_COMP_CLASS(),
            (Deref @ COMP { .. }, _) => metamodelica::Ref::new(LookupState::ERROR { errorState: crate::NFLookupState::LookupState::interned_COMP_FUNC() }),
            (_, Deref @ CLASS_COMP { .. }) => metamodelica::Ref::new(LookupState::ERROR { errorState: crate::NFLookupState::LookupState::interned_CLASS_COMP() }),
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFLookupState.LookupState.next2")); __mm_s.push_str(&*literal!(" failed on unknown transition for element ")); __mm_s.push_str(&*InstNode::name(node)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFLookupState.mo")))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(nextState)
    }

    pub(crate) fn checkCrefVariability(
        mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut inEnclosingScope: bool,
        mut context: i32,
        mut state: metamodelica::Ref<LookupState>,
    ) -> Result<metamodelica::Ref<LookupState>> {
        let mut state: metamodelica::Ref<LookupState> = state;
        if isError(&state) {
            return Ok(state);
        }
        if inEnclosingScope
            && !(InstContext::inRelaxed(context))
            && isNonConstantComponent(&(ComponentRef::node(cref)?))?
        {
            state = metamodelica::Ref::new(LookupState::ERROR {
                errorState: crate::NFLookupState::LookupState::interned_NON_CONSTANT(),
            });
        }
        Ok(state)
    }

    pub(crate) fn isNonConstantComponent(mut node: &metamodelica::Ref<InstNode::InstNode>) -> Result<bool> {
        let mut res: bool;
        res = InstNode::isComponent(node)? && !(Component::isConst(&(InstNode::component(node)?))?);
        Ok(res)
    }
}
