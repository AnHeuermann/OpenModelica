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

use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFRestriction {
    BLOCK,
    CLASS,
    CLOCK,
    CONNECTOR { isExpandable: bool },
    ENUMERATION,
    EXTERNAL_OBJECT,
    FUNCTION,
    MODEL,
    PACKAGE,
    OPERATOR,
    RECORD { isOperator: bool, usedExternally: bool },
    RECORD_CONSTRUCTOR,
    TYPE,
    UNKNOWN,
}
impl metamodelica::gc::MMTrace for NFRestriction {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFRestriction::BLOCK => Ok(()),
            NFRestriction::CLASS => Ok(()),
            NFRestriction::CLOCK => Ok(()),
            NFRestriction::CONNECTOR { isExpandable } => {
                metamodelica::gc::MMTrace::mm_accept(isExpandable, __mmv)?;
                Ok(())
            }
            NFRestriction::ENUMERATION => Ok(()),
            NFRestriction::EXTERNAL_OBJECT => Ok(()),
            NFRestriction::FUNCTION => Ok(()),
            NFRestriction::MODEL => Ok(()),
            NFRestriction::PACKAGE => Ok(()),
            NFRestriction::OPERATOR => Ok(()),
            NFRestriction::RECORD {
                isOperator,
                usedExternally,
            } => {
                metamodelica::gc::MMTrace::mm_accept(isOperator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(usedExternally, __mmv)?;
                Ok(())
            }
            NFRestriction::RECORD_CONSTRUCTOR => Ok(()),
            NFRestriction::TYPE => Ok(()),
            NFRestriction::UNKNOWN => Ok(()),
        }
    }
}
impl NFRestriction {
    pub fn interned_BLOCK() -> metamodelica::Ref<NFRestriction> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<NFRestriction>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(NFRestriction::BLOCK));
        (*INTERNED).clone()
    }
    pub fn interned_CLASS() -> metamodelica::Ref<NFRestriction> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<NFRestriction>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(NFRestriction::CLASS));
        (*INTERNED).clone()
    }
    pub fn interned_CLOCK() -> metamodelica::Ref<NFRestriction> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<NFRestriction>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(NFRestriction::CLOCK));
        (*INTERNED).clone()
    }
    pub fn interned_ENUMERATION() -> metamodelica::Ref<NFRestriction> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<NFRestriction>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(NFRestriction::ENUMERATION));
        (*INTERNED).clone()
    }
    pub fn interned_EXTERNAL_OBJECT() -> metamodelica::Ref<NFRestriction> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<NFRestriction>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(NFRestriction::EXTERNAL_OBJECT));
        (*INTERNED).clone()
    }
    pub fn interned_FUNCTION() -> metamodelica::Ref<NFRestriction> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<NFRestriction>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(NFRestriction::FUNCTION));
        (*INTERNED).clone()
    }
    pub fn interned_MODEL() -> metamodelica::Ref<NFRestriction> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<NFRestriction>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(NFRestriction::MODEL));
        (*INTERNED).clone()
    }
    pub fn interned_PACKAGE() -> metamodelica::Ref<NFRestriction> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<NFRestriction>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(NFRestriction::PACKAGE));
        (*INTERNED).clone()
    }
    pub fn interned_OPERATOR() -> metamodelica::Ref<NFRestriction> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<NFRestriction>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(NFRestriction::OPERATOR));
        (*INTERNED).clone()
    }
    pub fn interned_RECORD_CONSTRUCTOR() -> metamodelica::Ref<NFRestriction> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<NFRestriction>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(NFRestriction::RECORD_CONSTRUCTOR));
        (*INTERNED).clone()
    }
    pub fn interned_TYPE() -> metamodelica::Ref<NFRestriction> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<NFRestriction>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(NFRestriction::TYPE));
        (*INTERNED).clone()
    }
    pub fn interned_UNKNOWN() -> metamodelica::Ref<NFRestriction> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<NFRestriction>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(NFRestriction::UNKNOWN));
        (*INTERNED).clone()
    }
}
pub fn interned_BLOCK() -> metamodelica::Ref<NFRestriction> {
    NFRestriction::interned_BLOCK()
}
pub fn interned_CLASS() -> metamodelica::Ref<NFRestriction> {
    NFRestriction::interned_CLASS()
}
pub fn interned_CLOCK() -> metamodelica::Ref<NFRestriction> {
    NFRestriction::interned_CLOCK()
}
pub fn interned_ENUMERATION() -> metamodelica::Ref<NFRestriction> {
    NFRestriction::interned_ENUMERATION()
}
pub fn interned_EXTERNAL_OBJECT() -> metamodelica::Ref<NFRestriction> {
    NFRestriction::interned_EXTERNAL_OBJECT()
}
pub fn interned_FUNCTION() -> metamodelica::Ref<NFRestriction> {
    NFRestriction::interned_FUNCTION()
}
pub fn interned_MODEL() -> metamodelica::Ref<NFRestriction> {
    NFRestriction::interned_MODEL()
}
pub fn interned_PACKAGE() -> metamodelica::Ref<NFRestriction> {
    NFRestriction::interned_PACKAGE()
}
pub fn interned_OPERATOR() -> metamodelica::Ref<NFRestriction> {
    NFRestriction::interned_OPERATOR()
}
pub fn interned_RECORD_CONSTRUCTOR() -> metamodelica::Ref<NFRestriction> {
    NFRestriction::interned_RECORD_CONSTRUCTOR()
}
pub fn interned_TYPE() -> metamodelica::Ref<NFRestriction> {
    NFRestriction::interned_TYPE()
}
pub fn interned_UNKNOWN() -> metamodelica::Ref<NFRestriction> {
    NFRestriction::interned_UNKNOWN()
}
impl Default for NFRestriction {
    fn default() -> Self {
        Self::BLOCK
    }
}
pub use self::NFRestriction::{
    BLOCK, CLASS, CLOCK, CONNECTOR, ENUMERATION, EXTERNAL_OBJECT, FUNCTION, MODEL, OPERATOR, PACKAGE, RECORD,
    RECORD_CONSTRUCTOR, TYPE, UNKNOWN,
};
pub(crate) fn fromSCode(mut sres: &SCode::Restriction) -> metamodelica::Ref<NFRestriction> {
    let mut res: metamodelica::Ref<NFRestriction>;
    res = (match sres.clone() {
        SCode::Restriction::R_BLOCK { .. } => crate::NFRestriction::interned_BLOCK(),
        SCode::Restriction::R_CLASS { .. } => crate::NFRestriction::interned_CLASS(),
        SCode::Restriction::R_PREDEFINED_CLOCK { .. } => crate::NFRestriction::interned_CLOCK(),
        SCode::Restriction::R_CONNECTOR { .. } => metamodelica::Ref::new(NFRestriction::CONNECTOR {
            isExpandable: var_field!(sres.isExpandable, SCode::Restriction::R_CONNECTOR).clone(),
        }),
        SCode::Restriction::R_ENUMERATION { .. } => crate::NFRestriction::interned_ENUMERATION(),
        SCode::Restriction::R_FUNCTION { .. } => crate::NFRestriction::interned_FUNCTION(),
        SCode::Restriction::R_MODEL { .. } => crate::NFRestriction::interned_MODEL(),
        SCode::Restriction::R_OPERATOR { .. } => crate::NFRestriction::interned_OPERATOR(),
        SCode::Restriction::R_PACKAGE { .. } => crate::NFRestriction::interned_PACKAGE(),
        SCode::Restriction::R_RECORD { .. } => metamodelica::Ref::new(NFRestriction::RECORD {
            isOperator: var_field!(sres.isOperator, SCode::Restriction::R_RECORD).clone(),
            usedExternally: false,
        }),
        SCode::Restriction::R_TYPE { .. } => crate::NFRestriction::interned_TYPE(),
        _ => crate::NFRestriction::interned_MODEL(),
    });
    res
}

pub(crate) fn toDAE(
    mut res: &metamodelica::Ref<NFRestriction>,
    mut path: metamodelica::Ref<Absyn::Path>,
) -> ClassInf::State {
    let mut state: ClassInf::State;
    state = (match &**res {
        BLOCK { .. } => ClassInf::State::BLOCK { path: path },
        CLOCK { .. } => ClassInf::State::TYPE_CLOCK { path: path },
        CONNECTOR {
            isExpandable: __res_isExpandable,
        } => ClassInf::State::CONNECTOR {
            path: path,
            isExpandable: __res_isExpandable.clone(),
        },
        ENUMERATION { .. } => ClassInf::State::ENUMERATION { path: path },
        EXTERNAL_OBJECT { .. } => ClassInf::State::EXTERNAL_OBJ { path: path },
        FUNCTION { .. } => ClassInf::State::FUNCTION {
            path: path,
            isImpure: false,
        },
        MODEL { .. } => ClassInf::State::MODEL { path: path },
        OPERATOR { .. } => ClassInf::State::FUNCTION {
            path: path,
            isImpure: false,
        },
        PACKAGE { .. } => ClassInf::State::PACKAGE { path: path },
        RECORD { .. } => ClassInf::State::RECORD { path: path },
        RECORD_CONSTRUCTOR { .. } => ClassInf::State::RECORD { path: path },
        TYPE { .. } => ClassInf::State::TYPE { path: path },
        _ => ClassInf::State::UNKNOWN { path: path },
    });
    state
}

pub(crate) fn isConnector(mut res: &metamodelica::Ref<NFRestriction>) -> bool {
    let mut isConnector: bool;
    isConnector = (match &**res {
        CONNECTOR { .. } => true,
        _ => false,
    });
    isConnector
}

pub(crate) fn isExpandableConnector(mut res: &metamodelica::Ref<NFRestriction>) -> bool {
    let mut isConnector: bool;
    isConnector = (match &**res {
        CONNECTOR {
            isExpandable: __res_isExpandable,
        } => __res_isExpandable.clone(),
        _ => false,
    });
    isConnector
}

pub(crate) fn isNonexpandableConnector(mut res: &metamodelica::Ref<NFRestriction>) -> bool {
    let mut isNonexpandable: bool;
    isNonexpandable = (match &**res {
        CONNECTOR {
            isExpandable: __res_isExpandable,
        } => !(__res_isExpandable.clone()),
        _ => false,
    });
    isNonexpandable
}

pub(crate) fn isExternalObject(mut res: &metamodelica::Ref<NFRestriction>) -> bool {
    let mut isExternalObject: bool;
    isExternalObject = (match &**res {
        EXTERNAL_OBJECT { .. } => true,
        _ => false,
    });
    isExternalObject
}

pub(crate) fn isFunction(mut res: &metamodelica::Ref<NFRestriction>) -> bool {
    let mut isFunction: bool;
    isFunction = (match &**res {
        FUNCTION { .. } => true,
        _ => false,
    });
    isFunction
}

pub(crate) fn isRecordConstructor(mut res: &metamodelica::Ref<NFRestriction>) -> bool {
    let mut isConstructor: bool;
    isConstructor = (match &**res {
        RECORD_CONSTRUCTOR { .. } => true,
        _ => false,
    });
    isConstructor
}

pub(crate) fn isRecord(mut res: &metamodelica::Ref<NFRestriction>) -> bool {
    let mut isRecord: bool;
    isRecord = (match &**res {
        RECORD { .. } => true,
        _ => false,
    });
    isRecord
}

pub(crate) fn isExternalRecord(mut res: &metamodelica::Ref<NFRestriction>) -> bool {
    let mut isExtRecord: bool;
    isExtRecord = (match &**res {
        RECORD {
            usedExternally: __res_usedExternally,
            ..
        } => __res_usedExternally.clone(),
        _ => false,
    });
    isExtRecord
}

pub(crate) fn setExternalRecord(mut res: metamodelica::Ref<NFRestriction>) -> metamodelica::Ref<NFRestriction> {
    let mut res: metamodelica::Ref<NFRestriction> = res;
    let () = (match &*res {
        RECORD {
            usedExternally: false, ..
        } => {
            assign_variant_field!(res => NFRestriction::RECORD; usedExternally = true);
            ()
        }
        _ => (),
    });
    res
}

pub fn isOperatorRecord(mut res: &metamodelica::Ref<NFRestriction>) -> bool {
    let mut isOpRecord: bool;
    isOpRecord = (match &**res {
        RECORD {
            isOperator: __res_isOperator,
            ..
        } => __res_isOperator.clone(),
        _ => false,
    });
    isOpRecord
}

pub(crate) fn isOperator(mut res: &metamodelica::Ref<NFRestriction>) -> bool {
    let mut isOperator: bool;
    isOperator = (match &**res {
        OPERATOR { .. } => true,
        _ => false,
    });
    isOperator
}

pub(crate) fn isType(mut res: &metamodelica::Ref<NFRestriction>) -> bool {
    let mut isType: bool;
    isType = (match &**res {
        TYPE { .. } => true,
        _ => false,
    });
    isType
}

pub(crate) fn isClock(mut res: &metamodelica::Ref<NFRestriction>) -> bool {
    let mut isClock: bool;
    isClock = (match &**res {
        CLOCK { .. } => true,
        _ => false,
    });
    isClock
}

pub(crate) fn isModel(mut res: &metamodelica::Ref<NFRestriction>) -> bool {
    let mut isModel: bool;
    isModel = (match &**res {
        MODEL { .. } => true,
        _ => false,
    });
    isModel
}

pub fn toString(mut res: &metamodelica::Ref<NFRestriction>) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match &**res {
        BLOCK { .. } => literal!("block"),
        CLASS { .. } => literal!("class"),
        CLOCK { .. } => literal!("clock"),
        CONNECTOR {
            isExpandable: __res_isExpandable,
        } => {
            if (__res_isExpandable.clone()) {
                literal!("expandable connector")
            } else {
                literal!("connector")
            }
        }
        ENUMERATION { .. } => literal!("enumeration"),
        EXTERNAL_OBJECT { .. } => literal!("ExternalObject"),
        FUNCTION { .. } => literal!("function"),
        MODEL { .. } => literal!("model"),
        OPERATOR { .. } => literal!("operator"),
        PACKAGE { .. } => literal!("package"),
        RECORD { .. } => literal!("record"),
        RECORD_CONSTRUCTOR { .. } => literal!("record"),
        TYPE { .. } => literal!("type"),
        _ => literal!("unknown"),
    });
    r#str
}

pub(crate) fn assertNoEquations(
    mut equations: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut initialEquations: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut res: &metamodelica::Ref<NFRestriction>,
    mut onlyDeprecated: bool,
) -> Result<()> {
    let mut eq: metamodelica::Ref<SCode::Equation>;
    if (equations).is_empty() && (initialEquations).is_empty() {
        return Ok(());
    }
    eq = (if ((equations).is_empty()) {
        initialEquations
    } else {
        equations
    })
    .head()
    .cloned()?;
    if onlyDeprecated {
        Error::addSourceMessage(
            &(Error::DEPRECATED_TRANSITION_FAILURE.clone()),
            list![literal!("Equation sections"), toString(res)],
            &(SCodeUtil::getEquationInfo(&eq)),
        )?;
    } else {
        Error::addSourceMessage(
            &(Error::EQUATION_TRANSITION_FAILURE.clone()),
            list![toString(res)],
            &(SCodeUtil::getEquationInfo(&eq)),
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn assertNoAlgorithms(
    mut algorithms: &metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    mut initialAlgorithms: &metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    mut res: &metamodelica::Ref<NFRestriction>,
    mut onlyDeprecated: bool,
) -> Result<()> {
    let mut alg_opt: Option<metamodelica::Ref<SCode::AlgorithmSection>> = None;
    let mut alg: metamodelica::Ref<SCode::AlgorithmSection>;
    let mut info: SourceInfo;
    alg_opt = List::findOption(
        algorithms,
        &move |__a0: metamodelica::Ref<SCode::AlgorithmSection>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(SCodeUtil::isNonEmptyAlgorithm(&__a0))
        },
    )?;
    if (alg_opt).is_none() {
        alg_opt = List::findOption(initialAlgorithms, &move |__a0: metamodelica::Ref<
            SCode::AlgorithmSection,
        >|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(SCodeUtil::isNonEmptyAlgorithm(&__a0))
        })?;
    }
    if (alg_opt).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(alg_opt) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        alg = metamodelica::Own::own(__pa0);
        info = SCodeUtil::getStatementInfo(&((alg.statements).head().cloned()?))?;
        if onlyDeprecated {
            Error::addSourceMessage(
                &(Error::DEPRECATED_TRANSITION_FAILURE.clone()),
                list![literal!("Algorithm sections"), toString(res)],
                &info,
            )?;
            return Ok(());
        } else {
            Error::addSourceMessage(
                &(Error::ALGORITHM_TRANSITION_FAILURE.clone()),
                list![toString(res)],
                &info,
            )?;
            return Err("fail");
        }
    }
    Ok(())
}

pub(crate) fn assertNoInitialAlgorithms(
    mut algs: &metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    mut res: &metamodelica::Ref<NFRestriction>,
) -> Result<()> {
    for mut alg in &**algs {
        if !((alg.statements).is_empty()) {
            Error::addSourceMessage(
                &(Error::INITIAL_ALGORITHM_TRANSITION_FAILURE.clone()),
                list![toString(res)],
                &(SCodeUtil::getStatementInfo(&((alg.statements).head().cloned()?))?),
            )?;
            return Err("fail");
        }
    }
    Ok(())
}

pub(crate) fn assertNoProtected(
    mut elements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut res: &metamodelica::Ref<NFRestriction>,
) -> Result<()> {
    for mut e in &**elements {
        if SCodeUtil::isElementProtected(metamodelica::AsArg::as_arg(&e)) {
            Error::addSourceMessage(
                &(Error::PROTECTED_TRANSITION_FAILURE.clone()),
                list![toString(res)],
                &(SCodeUtil::elementInfo(metamodelica::AsArg::as_arg(&e))),
            )?;
            return Err("fail");
        }
    }
    Ok(())
}

pub(crate) fn assertNoComponents(
    mut elements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut res: &metamodelica::Ref<NFRestriction>,
) -> Result<()> {
    for mut e in &**elements {
        if SCodeUtil::isComponent(metamodelica::AsArg::as_arg(&e)) {
            Error::addSourceMessage(
                &(Error::DEPRECATED_TRANSITION_FAILURE.clone()),
                list![literal!("Components"), toString(res)],
                &(SCodeUtil::elementInfo(metamodelica::AsArg::as_arg(&e))),
            )?;
        }
    }
    Ok(())
}

pub(crate) fn assertOnlyConstantComponents(
    mut elements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut clsNode: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<()> {
    for mut e in &**elements {
        let () = (match &*e.clone() {
            SCode::Element::COMPONENT {
                attributes: __e_attributes,
                info: __e_info,
                name: __e_name,
                ..
            } if (!(SCodeUtil::isConstant(SCodeUtil::attrVariability(metamodelica::AsArg::as_arg(
                &__e_attributes,
            ))))) =>
            {
                Error::addSourceMessage(
                    &(Error::PACKAGE_VARIABLE_NOT_CONSTANT.clone()),
                    list![__e_name.clone(), InstNode::name(clsNode)?],
                    metamodelica::AsArg::as_arg(&__e_info),
                )?;
                return Err("fail");
            }
            _ => (),
        });
    }
    Ok(())
}

pub(crate) fn assertOnlyFunctions(
    mut elements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut res: &metamodelica::Ref<NFRestriction>,
) -> () {
    for mut e in &**elements {
        if !(SCodeUtil::isFunction(metamodelica::AsArg::as_arg(&e))) {}
    }
    ()
}

pub(crate) fn checkClass(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut restriction: &metamodelica::Ref<NFRestriction>,
    mut context: i32,
) -> Result<()> {
    let mut cdef: metamodelica::Ref<SCode::ClassDef>;
    if InstContext::inRelaxed(context) {
        return Ok(());
    }
    cdef = SCodeUtil::getClassBody(&(InstNode::definition(node.clone())?))?;
    let () = (match &*cdef {
        SCode::ClassDef::PARTS {
            elementLst: __cdef_elementLst,
            initialAlgorithmLst: __cdef_initialAlgorithmLst,
            initialEquationLst: __cdef_initialEquationLst,
            normalAlgorithmLst: __cdef_normalAlgorithmLst,
            normalEquationLst: __cdef_normalEquationLst,
            ..
        } => {
            let () = (match &**restriction {
                CLASS => {
                    assertNoComponents(metamodelica::AsArg::as_arg(&__cdef_elementLst), restriction)?;
                    assertNoEquations(
                        __cdef_normalEquationLst.clone(),
                        __cdef_initialEquationLst.clone(),
                        restriction,
                        true,
                    )?;
                    assertNoAlgorithms(
                        metamodelica::AsArg::as_arg(&__cdef_normalAlgorithmLst),
                        metamodelica::AsArg::as_arg(&__cdef_initialAlgorithmLst),
                        restriction,
                        true,
                    )?;
                    ()
                }
                RECORD { .. } => {
                    assertNoProtected(metamodelica::AsArg::as_arg(&__cdef_elementLst), restriction)?;
                    assertNoEquations(
                        __cdef_normalEquationLst.clone(),
                        __cdef_initialEquationLst.clone(),
                        restriction,
                        false,
                    )?;
                    assertNoAlgorithms(
                        metamodelica::AsArg::as_arg(&__cdef_normalAlgorithmLst),
                        metamodelica::AsArg::as_arg(&__cdef_initialAlgorithmLst),
                        restriction,
                        false,
                    )?;
                    ()
                }
                TYPE => {
                    assertNoProtected(metamodelica::AsArg::as_arg(&__cdef_elementLst), restriction)?;
                    assertNoEquations(
                        __cdef_normalEquationLst.clone(),
                        __cdef_initialEquationLst.clone(),
                        restriction,
                        false,
                    )?;
                    assertNoAlgorithms(
                        metamodelica::AsArg::as_arg(&__cdef_normalAlgorithmLst),
                        metamodelica::AsArg::as_arg(&__cdef_initialAlgorithmLst),
                        restriction,
                        false,
                    )?;
                    ()
                }
                BLOCK => (),
                FUNCTION => {
                    assertNoEquations(
                        __cdef_normalEquationLst.clone(),
                        __cdef_initialEquationLst.clone(),
                        restriction,
                        false,
                    )?;
                    assertNoInitialAlgorithms(metamodelica::AsArg::as_arg(&__cdef_initialAlgorithmLst), restriction)?;
                    ()
                }
                CONNECTOR { .. } => {
                    assertNoProtected(metamodelica::AsArg::as_arg(&__cdef_elementLst), restriction)?;
                    assertNoEquations(
                        __cdef_normalEquationLst.clone(),
                        __cdef_initialEquationLst.clone(),
                        restriction,
                        false,
                    )?;
                    assertNoAlgorithms(
                        metamodelica::AsArg::as_arg(&__cdef_normalAlgorithmLst),
                        metamodelica::AsArg::as_arg(&__cdef_initialAlgorithmLst),
                        restriction,
                        false,
                    )?;
                    ()
                }
                PACKAGE => {
                    assertOnlyConstantComponents(metamodelica::AsArg::as_arg(&__cdef_elementLst), &node)?;
                    assertNoEquations(
                        __cdef_normalEquationLst.clone(),
                        __cdef_initialEquationLst.clone(),
                        restriction,
                        false,
                    )?;
                    assertNoAlgorithms(
                        metamodelica::AsArg::as_arg(&__cdef_normalAlgorithmLst),
                        metamodelica::AsArg::as_arg(&__cdef_initialAlgorithmLst),
                        restriction,
                        false,
                    )?;
                    ()
                }
                OPERATOR => {
                    assertNoEquations(
                        __cdef_normalEquationLst.clone(),
                        __cdef_initialEquationLst.clone(),
                        restriction,
                        false,
                    )?;
                    assertNoAlgorithms(
                        metamodelica::AsArg::as_arg(&__cdef_normalAlgorithmLst),
                        metamodelica::AsArg::as_arg(&__cdef_initialAlgorithmLst),
                        restriction,
                        false,
                    )?;
                    ()
                }
                _ => (),
            });
            ()
        }
        _ => (),
    });
    Ok(())
}
