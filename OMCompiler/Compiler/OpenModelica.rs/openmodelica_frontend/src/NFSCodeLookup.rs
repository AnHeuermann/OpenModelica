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
use crate::NFSCodeEnv;
use crate::NFSCodeEnv::EnvTree;
use crate::NFSCodeFlattenImports;
use crate::NFSCodeFlattenRedeclare;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_inst::NFInstPrefix;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;

pub type Env = metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>;

pub type Item = metamodelica::Ref<NFSCodeEnv::Item>;

pub type Extends = metamodelica::Ref<NFSCodeEnv::Extends>;

pub type Frame = metamodelica::Ref<NFSCodeEnv::Frame>;

pub type FrameType = NFSCodeEnv::FrameType;

pub type Import = Absyn::Import;

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum RedeclareReplaceStrategy {
    INSERT_REDECLARES,
    IGNORE_REDECLARES,
}
impl metamodelica::gc::MMTrace for RedeclareReplaceStrategy {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            RedeclareReplaceStrategy::INSERT_REDECLARES => Ok(()),
            RedeclareReplaceStrategy::IGNORE_REDECLARES => Ok(()),
        }
    }
}
pub(crate) use self::RedeclareReplaceStrategy::{IGNORE_REDECLARES, INSERT_REDECLARES};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum LookupStrategy {
    NO_BUILTIN_TYPES,
    LOOKUP_ANY,
}
impl metamodelica::gc::MMTrace for LookupStrategy {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            LookupStrategy::NO_BUILTIN_TYPES => Ok(()),
            LookupStrategy::LOOKUP_ANY => Ok(()),
        }
    }
}
pub(crate) use self::LookupStrategy::{LOOKUP_ANY, NO_BUILTIN_TYPES};

// Default parts of the declarations for builtin elements and types.
pub(crate) static BUILTIN_PREFIXES: std::sync::LazyLock<metamodelica::Ref<SCode::Prefixes>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Prefixes {
            visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC,
            redeclarePrefix: openmodelica_frontend_types::SCode::Redeclare::NOT_REDECLARE,
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
            replaceablePrefix: openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE(),
        })
    });

pub(crate) static BUILTIN_ATTRIBUTES: std::sync::LazyLock<SCode::Attributes> =
    std::sync::LazyLock::new(|| SCode::Attributes {
        arrayDims: metamodelica::nil(),
        connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
        parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
        variability: openmodelica_frontend_types::SCode::Variability::VAR,
        direction: openmodelica_ast::Absyn::Direction::BIDIR,
        isField: openmodelica_ast::Absyn::IsField::NONFIELD,
    });

pub(crate) static BUILTIN_CONST_ATTRIBUTES: std::sync::LazyLock<SCode::Attributes> =
    std::sync::LazyLock::new(|| SCode::Attributes {
        arrayDims: metamodelica::nil(),
        connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
        parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
        variability: openmodelica_frontend_types::SCode::Variability::CONST,
        direction: openmodelica_ast::Absyn::Direction::BIDIR,
        isField: openmodelica_ast::Absyn::IsField::NONFIELD,
    });

pub(crate) static BUILTIN_EMPTY_CLASS: std::sync::LazyLock<metamodelica::Ref<SCode::ClassDef>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::ClassDef::PARTS {
            elementLst: metamodelica::nil(),
            normalEquationLst: metamodelica::nil(),
            initialEquationLst: metamodelica::nil(),
            normalAlgorithmLst: metamodelica::nil(),
            initialAlgorithmLst: metamodelica::nil(),
            constraintLst: metamodelica::nil(),
            clsattrs: metamodelica::nil(),
            externalDecl: None,
        })
    });

// Metatypes used to define the builtin types.
pub(crate) static BUILTIN_REALTYPE: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::CLASS {
            name: literal!("$RealType"),
            prefixes: BUILTIN_PREFIXES.clone(),
            encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
            partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
            restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL,
            classDef: BUILTIN_EMPTY_CLASS.clone(),
            cmt: SCode::noComment.clone(),
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_INTEGERTYPE: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::CLASS {
            name: literal!("$IntegerType"),
            prefixes: BUILTIN_PREFIXES.clone(),
            encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
            partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
            restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER,
            classDef: BUILTIN_EMPTY_CLASS.clone(),
            cmt: SCode::noComment.clone(),
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_BOOLEANTYPE: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::CLASS {
            name: literal!("$BooleanType"),
            prefixes: BUILTIN_PREFIXES.clone(),
            encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
            partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
            restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN,
            classDef: BUILTIN_EMPTY_CLASS.clone(),
            cmt: SCode::noComment.clone(),
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_STRINGTYPE: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::CLASS {
            name: literal!("$StringType"),
            prefixes: BUILTIN_PREFIXES.clone(),
            encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
            partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
            restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING,
            classDef: BUILTIN_EMPTY_CLASS.clone(),
            cmt: SCode::noComment.clone(),
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_ENUMTYPE: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::CLASS {
            name: literal!("$EnumType"),
            prefixes: BUILTIN_PREFIXES.clone(),
            encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
            partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
            restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_ENUMERATION,
            classDef: BUILTIN_EMPTY_CLASS.clone(),
            cmt: SCode::noComment.clone(),
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_REALTYPE_ITEM: std::sync::LazyLock<metamodelica::Ref<NFSCodeEnv::Item>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
            var: BUILTIN_REALTYPE.clone(),
            isUsed: None,
        })
    });

pub(crate) static BUILTIN_INTEGERTYPE_ITEM: std::sync::LazyLock<metamodelica::Ref<NFSCodeEnv::Item>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
            var: BUILTIN_INTEGERTYPE.clone(),
            isUsed: None,
        })
    });

pub(crate) static BUILTIN_BOOLEANTYPE_ITEM: std::sync::LazyLock<metamodelica::Ref<NFSCodeEnv::Item>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
            var: BUILTIN_BOOLEANTYPE.clone(),
            isUsed: None,
        })
    });

pub(crate) static BUILTIN_STRINGTYPE_ITEM: std::sync::LazyLock<metamodelica::Ref<NFSCodeEnv::Item>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
            var: BUILTIN_STRINGTYPE.clone(),
            isUsed: None,
        })
    });

pub(crate) static BUILTIN_ENUMTYPE_ITEM: std::sync::LazyLock<metamodelica::Ref<NFSCodeEnv::Item>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
            var: BUILTIN_ENUMTYPE.clone(),
            isUsed: None,
        })
    });

pub(crate) static BUILTIN_REALTYPE_SPEC: std::sync::LazyLock<metamodelica::Ref<Absyn::TypeSpec>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("$RealType"),
            }),
            arrayDim: None,
        })
    });

pub(crate) static BUILTIN_INTEGERTYPE_SPEC: std::sync::LazyLock<metamodelica::Ref<Absyn::TypeSpec>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("$IntegerType"),
            }),
            arrayDim: None,
        })
    });

pub(crate) static BUILTIN_BOOLEANTYPE_SPEC: std::sync::LazyLock<metamodelica::Ref<Absyn::TypeSpec>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("$BooleanType"),
            }),
            arrayDim: None,
        })
    });

pub(crate) static BUILTIN_STRINGTYPE_SPEC: std::sync::LazyLock<metamodelica::Ref<Absyn::TypeSpec>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("$StringType"),
            }),
            arrayDim: None,
        })
    });

pub(crate) static BUILTIN_ENUMTYPE_SPEC: std::sync::LazyLock<metamodelica::Ref<Absyn::TypeSpec>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("$EnumType"),
            }),
            arrayDim: None,
        })
    });

pub(crate) static BUILTIN_STATESELECT_SPEC: std::sync::LazyLock<metamodelica::Ref<Absyn::TypeSpec>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("StateSelect"),
            }),
            arrayDim: None,
        })
    });

// Parts of the builtin types.
// Generic elements:
pub(crate) static BUILTIN_ATTR_QUANTITY: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("quantity"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_STRINGTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_ATTR_UNIT: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("unit"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_STRINGTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_ATTR_DISPLAYUNIT: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("displayUnit"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_STRINGTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_ATTR_FIXED: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("fixed"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_BOOLEANTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_ATTR_STATESELECT: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("stateSelect"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_STATESELECT_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

// Real-specific elements:
pub(crate) static BUILTIN_REAL_MIN: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("min"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_REALTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_REAL_MAX: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("max"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_REALTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_REAL_START: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("start"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_REALTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_REAL_NOMINAL: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("nominal"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_REALTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

// Integer-specific elements:
pub(crate) static BUILTIN_INTEGER_MIN: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("min"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_INTEGERTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_INTEGER_MAX: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("max"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_INTEGERTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_INTEGER_START: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("start"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_INTEGERTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

// Boolean-specific elements:
pub(crate) static BUILTIN_BOOLEAN_START: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("start"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_BOOLEANTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

// String-specific elements:
pub(crate) static BUILTIN_STRING_START: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("start"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_STRINGTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

// StateSelect-specific elements:
pub(crate) static BUILTIN_ENUM_MIN: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("min"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_ENUM_MAX: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("max"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_ENUM_START: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("start"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_STATESELECT_NEVER: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("never"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_CONST_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_STATESELECT_AVOID: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("avoid"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_CONST_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_STATESELECT_DEFAULT: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("default"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_CONST_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_STATESELECT_PREFER: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("prefer"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_CONST_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static BUILTIN_STATESELECT_ALWAYS: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("always"),
            prefixes: BUILTIN_PREFIXES.clone(),
            attributes: BUILTIN_CONST_ATTRIBUTES.clone(),
            typeSpec: BUILTIN_ENUMTYPE_SPEC.clone(),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

// Environments for the builtin types:
pub(crate) static BUILTIN_REAL_ENV: std::sync::LazyLock<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>> =
    std::sync::LazyLock::new(|| {
        list![metamodelica::Ref::new(NFSCodeEnv::Frame {
            name: Some(literal!("Real")),
            frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE,
            clsAndVars: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::NODE {
                key: literal!("nominal"),
                value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                    var: BUILTIN_REAL_NOMINAL.clone(),
                    isUsed: None
                }),
                height: 3,
                left: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::NODE {
                    key: literal!("max"),
                    value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                        var: BUILTIN_REAL_MAX.clone(),
                        isUsed: None
                    }),
                    height: 2,
                    left: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::NODE {
                        key: literal!("fixed"),
                        value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                            var: BUILTIN_ATTR_FIXED.clone(),
                            isUsed: None
                        }),
                        height: 1,
                        left: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                            key: literal!("displayUnit"),
                            value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                                var: BUILTIN_ATTR_DISPLAYUNIT.clone(),
                                isUsed: None
                            })
                        }),
                        right: crate::NFSCodeEnv::EnvTree::Tree::interned_EMPTY()
                    }),
                    right: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                        key: literal!("min"),
                        value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                            var: BUILTIN_REAL_MIN.clone(),
                            isUsed: None
                        })
                    })
                }),
                right: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::NODE {
                    key: literal!("start"),
                    value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                        var: BUILTIN_REAL_START.clone(),
                        isUsed: None
                    }),
                    height: 2,
                    left: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                        key: literal!("quantity"),
                        value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                            var: BUILTIN_ATTR_QUANTITY.clone(),
                            isUsed: None
                        })
                    }),
                    right: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::NODE {
                        key: literal!("stateSelect"),
                        value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                            var: BUILTIN_ATTR_STATESELECT.clone(),
                            isUsed: None
                        }),
                        height: 1,
                        left: crate::NFSCodeEnv::EnvTree::Tree::interned_EMPTY(),
                        right: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                            key: literal!("unit"),
                            value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                                var: BUILTIN_ATTR_UNIT.clone(),
                                isUsed: None
                            })
                        })
                    })
                })
            }),
            extendsTable: metamodelica::Ref::new(NFSCodeEnv::ExtendsTable {
                baseClasses: metamodelica::nil(),
                redeclaredElements: metamodelica::nil(),
                classExtendsInfo: None
            }),
            importTable: NFSCodeEnv::ImportTable {
                hidden: false,
                qualifiedImports: metamodelica::nil(),
                unqualifiedImports: metamodelica::nil()
            },
            isUsed: None
        })]
    });

pub(crate) static BUILTIN_INTEGER_ENV: std::sync::LazyLock<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>> =
    std::sync::LazyLock::new(|| {
        list![metamodelica::Ref::new(NFSCodeEnv::Frame {
            name: Some(literal!("Integer")),
            frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE,
            clsAndVars: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::NODE {
                key: literal!("quantity"),
                value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                    var: BUILTIN_ATTR_QUANTITY.clone(),
                    isUsed: None
                }),
                height: 2,
                left: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::NODE {
                    key: literal!("max"),
                    value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                        var: BUILTIN_INTEGER_MAX.clone(),
                        isUsed: None
                    }),
                    height: 1,
                    left: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                        key: literal!("fixed"),
                        value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                            var: BUILTIN_ATTR_FIXED.clone(),
                            isUsed: None
                        })
                    }),
                    right: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                        key: literal!("min"),
                        value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                            var: BUILTIN_INTEGER_MIN.clone(),
                            isUsed: None
                        })
                    })
                }),
                right: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                    key: literal!("start"),
                    value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                        var: BUILTIN_INTEGER_START.clone(),
                        isUsed: None
                    })
                })
            }),
            extendsTable: metamodelica::Ref::new(NFSCodeEnv::ExtendsTable {
                baseClasses: metamodelica::nil(),
                redeclaredElements: metamodelica::nil(),
                classExtendsInfo: None
            }),
            importTable: NFSCodeEnv::ImportTable {
                hidden: false,
                qualifiedImports: metamodelica::nil(),
                unqualifiedImports: metamodelica::nil()
            },
            isUsed: None
        })]
    });

pub(crate) static BUILTIN_BOOLEAN_ENV: std::sync::LazyLock<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>> =
    std::sync::LazyLock::new(|| {
        list![metamodelica::Ref::new(NFSCodeEnv::Frame {
            name: Some(literal!("Boolean")),
            frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE,
            clsAndVars: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::NODE {
                key: literal!("quantity"),
                value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                    var: BUILTIN_ATTR_QUANTITY.clone(),
                    isUsed: None
                }),
                height: 1,
                left: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                    key: literal!("fixed"),
                    value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                        var: BUILTIN_ATTR_FIXED.clone(),
                        isUsed: None
                    })
                }),
                right: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                    key: literal!("start"),
                    value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                        var: BUILTIN_BOOLEAN_START.clone(),
                        isUsed: None
                    })
                })
            }),
            extendsTable: metamodelica::Ref::new(NFSCodeEnv::ExtendsTable {
                baseClasses: metamodelica::nil(),
                redeclaredElements: metamodelica::nil(),
                classExtendsInfo: None
            }),
            importTable: NFSCodeEnv::ImportTable {
                hidden: false,
                qualifiedImports: metamodelica::nil(),
                unqualifiedImports: metamodelica::nil()
            },
            isUsed: None
        })]
    });

pub(crate) static BUILTIN_STRING_ENV: std::sync::LazyLock<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>> =
    std::sync::LazyLock::new(|| {
        list![metamodelica::Ref::new(NFSCodeEnv::Frame {
            name: Some(literal!("String")),
            frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE,
            clsAndVars: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::NODE {
                key: literal!("quantity"),
                value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                    var: BUILTIN_ATTR_QUANTITY.clone(),
                    isUsed: None
                }),
                height: 2,
                left: crate::NFSCodeEnv::EnvTree::Tree::interned_EMPTY(),
                right: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                    key: literal!("start"),
                    value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                        var: BUILTIN_STRING_START.clone(),
                        isUsed: None
                    })
                })
            }),
            extendsTable: metamodelica::Ref::new(NFSCodeEnv::ExtendsTable {
                baseClasses: metamodelica::nil(),
                redeclaredElements: metamodelica::nil(),
                classExtendsInfo: None
            }),
            importTable: NFSCodeEnv::ImportTable {
                hidden: false,
                qualifiedImports: metamodelica::nil(),
                unqualifiedImports: metamodelica::nil()
            },
            isUsed: None
        })]
    });

pub(crate) static BUILTIN_STATESELECT_ENV: std::sync::LazyLock<
    metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
> = std::sync::LazyLock::new(|| {
    list![metamodelica::Ref::new(NFSCodeEnv::Frame {
        name: Some(literal!("StateSelect")),
        frameType: crate::NFSCodeEnv::FrameType::NORMAL_SCOPE,
        clsAndVars: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::NODE {
            key: literal!("max"),
            value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                var: BUILTIN_ENUM_MAX.clone(),
                isUsed: None
            }),
            height: 3,
            left: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::NODE {
                key: literal!("default"),
                value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                    var: BUILTIN_STATESELECT_DEFAULT.clone(),
                    isUsed: None
                }),
                height: 2,
                left: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::NODE {
                    key: literal!("avoid"),
                    value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                        var: BUILTIN_STATESELECT_AVOID.clone(),
                        isUsed: None
                    }),
                    height: 1,
                    left: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                        key: literal!("always"),
                        value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                            var: BUILTIN_STATESELECT_ALWAYS.clone(),
                            isUsed: None
                        })
                    }),
                    right: crate::NFSCodeEnv::EnvTree::Tree::interned_EMPTY()
                }),
                right: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                    key: literal!("fixed"),
                    value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                        var: BUILTIN_ATTR_FIXED.clone(),
                        isUsed: None
                    })
                })
            }),
            right: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::NODE {
                key: literal!("never"),
                value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                    var: BUILTIN_STATESELECT_NEVER.clone(),
                    isUsed: None
                }),
                height: 2,
                left: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                    key: literal!("min"),
                    value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                        var: BUILTIN_ENUM_MIN.clone(),
                        isUsed: None
                    })
                }),
                right: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::NODE {
                    key: literal!("quantity"),
                    value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                        var: BUILTIN_ATTR_QUANTITY.clone(),
                        isUsed: None
                    }),
                    height: 1,
                    left: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                        key: literal!("prefer"),
                        value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                            var: BUILTIN_STATESELECT_PREFER.clone(),
                            isUsed: None
                        })
                    }),
                    right: metamodelica::Ref::new(NFSCodeEnv::EnvTree::Tree::LEAF {
                        key: literal!("start"),
                        value: metamodelica::Ref::new(NFSCodeEnv::Item::VAR {
                            var: BUILTIN_ENUM_START.clone(),
                            isUsed: None
                        })
                    })
                })
            })
        }),
        extendsTable: metamodelica::Ref::new(NFSCodeEnv::ExtendsTable {
            baseClasses: metamodelica::nil(),
            redeclaredElements: metamodelica::nil(),
            classExtendsInfo: None
        }),
        importTable: NFSCodeEnv::ImportTable {
            hidden: false,
            qualifiedImports: metamodelica::nil(),
            unqualifiedImports: metamodelica::nil()
        },
        isUsed: None
    })]
});

// The builtin types:
pub(crate) static BUILTIN_REAL: std::sync::LazyLock<metamodelica::Ref<NFSCodeEnv::Item>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(NFSCodeEnv::Item::CLASS {
            cls: metamodelica::Ref::new(SCode::Element::CLASS {
                name: literal!("Real"),
                prefixes: SCode::defaultPrefixes.clone(),
                encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
                partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
                restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE,
                classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
                    elementLst: metamodelica::nil(),
                    normalEquationLst: metamodelica::nil(),
                    initialEquationLst: metamodelica::nil(),
                    normalAlgorithmLst: metamodelica::nil(),
                    initialAlgorithmLst: metamodelica::nil(),
                    constraintLst: metamodelica::nil(),
                    clsattrs: metamodelica::nil(),
                    externalDecl: None,
                }),
                cmt: SCode::noComment.clone(),
                info: Absyn::dummyInfo.clone(),
            }),
            env: BUILTIN_REAL_ENV.clone(),
            classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE,
        })
    });

pub(crate) static BUILTIN_INTEGER: std::sync::LazyLock<metamodelica::Ref<NFSCodeEnv::Item>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(NFSCodeEnv::Item::CLASS {
            cls: metamodelica::Ref::new(SCode::Element::CLASS {
                name: literal!("Integer"),
                prefixes: SCode::defaultPrefixes.clone(),
                encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
                partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
                restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE,
                classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
                    elementLst: metamodelica::nil(),
                    normalEquationLst: metamodelica::nil(),
                    initialEquationLst: metamodelica::nil(),
                    normalAlgorithmLst: metamodelica::nil(),
                    initialAlgorithmLst: metamodelica::nil(),
                    constraintLst: metamodelica::nil(),
                    clsattrs: metamodelica::nil(),
                    externalDecl: None,
                }),
                cmt: SCode::noComment.clone(),
                info: Absyn::dummyInfo.clone(),
            }),
            env: BUILTIN_INTEGER_ENV.clone(),
            classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE,
        })
    });

pub(crate) static BUILTIN_BOOLEAN: std::sync::LazyLock<metamodelica::Ref<NFSCodeEnv::Item>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(NFSCodeEnv::Item::CLASS {
            cls: metamodelica::Ref::new(SCode::Element::CLASS {
                name: literal!("Boolean"),
                prefixes: SCode::defaultPrefixes.clone(),
                encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
                partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
                restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE,
                classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
                    elementLst: metamodelica::nil(),
                    normalEquationLst: metamodelica::nil(),
                    initialEquationLst: metamodelica::nil(),
                    normalAlgorithmLst: metamodelica::nil(),
                    initialAlgorithmLst: metamodelica::nil(),
                    constraintLst: metamodelica::nil(),
                    clsattrs: metamodelica::nil(),
                    externalDecl: None,
                }),
                cmt: SCode::noComment.clone(),
                info: Absyn::dummyInfo.clone(),
            }),
            env: BUILTIN_BOOLEAN_ENV.clone(),
            classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE,
        })
    });

pub(crate) static BUILTIN_STRING: std::sync::LazyLock<metamodelica::Ref<NFSCodeEnv::Item>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(NFSCodeEnv::Item::CLASS {
            cls: metamodelica::Ref::new(SCode::Element::CLASS {
                name: literal!("String"),
                prefixes: SCode::defaultPrefixes.clone(),
                encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
                partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
                restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE,
                classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
                    elementLst: metamodelica::nil(),
                    normalEquationLst: metamodelica::nil(),
                    initialEquationLst: metamodelica::nil(),
                    normalAlgorithmLst: metamodelica::nil(),
                    initialAlgorithmLst: metamodelica::nil(),
                    constraintLst: metamodelica::nil(),
                    clsattrs: metamodelica::nil(),
                    externalDecl: None,
                }),
                cmt: SCode::noComment.clone(),
                info: Absyn::dummyInfo.clone(),
            }),
            env: BUILTIN_STRING_ENV.clone(),
            classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE,
        })
    });

pub(crate) static BUILTIN_STATESELECT: std::sync::LazyLock<metamodelica::Ref<NFSCodeEnv::Item>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(NFSCodeEnv::Item::CLASS {
            cls: metamodelica::Ref::new(SCode::Element::CLASS {
                name: literal!("StateSelect"),
                prefixes: SCode::defaultPrefixes.clone(),
                encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
                partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
                restriction: openmodelica_frontend_types::SCode::Restriction::R_CLASS,
                classDef: metamodelica::Ref::new(SCode::ClassDef::ENUMERATION {
                    enumLst: list![
                        metamodelica::Ref::new(SCode::Enum {
                            literal: literal!("never"),
                            comment: SCode::noComment.clone()
                        }),
                        metamodelica::Ref::new(SCode::Enum {
                            literal: literal!("avoid"),
                            comment: SCode::noComment.clone()
                        }),
                        metamodelica::Ref::new(SCode::Enum {
                            literal: literal!("default"),
                            comment: SCode::noComment.clone()
                        }),
                        metamodelica::Ref::new(SCode::Enum {
                            literal: literal!("prefer"),
                            comment: SCode::noComment.clone()
                        }),
                        metamodelica::Ref::new(SCode::Enum {
                            literal: literal!("always"),
                            comment: SCode::noComment.clone()
                        })
                    ],
                }),
                cmt: SCode::noComment.clone(),
                info: Absyn::dummyInfo.clone(),
            }),
            env: BUILTIN_STATESELECT_ENV.clone(),
            classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE,
        })
    });

pub(crate) static BUILTIN_EXTERNALOBJECT: std::sync::LazyLock<metamodelica::Ref<NFSCodeEnv::Item>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(NFSCodeEnv::Item::CLASS {
            cls: metamodelica::Ref::new(SCode::Element::CLASS {
                name: literal!("ExternalObject"),
                prefixes: SCode::defaultPrefixes.clone(),
                encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
                partialPrefix: openmodelica_frontend_types::SCode::Partial::PARTIAL,
                restriction: openmodelica_frontend_types::SCode::Restriction::R_CLASS,
                classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
                    elementLst: metamodelica::nil(),
                    normalEquationLst: metamodelica::nil(),
                    initialEquationLst: metamodelica::nil(),
                    normalAlgorithmLst: metamodelica::nil(),
                    initialAlgorithmLst: metamodelica::nil(),
                    constraintLst: metamodelica::nil(),
                    clsattrs: metamodelica::nil(),
                    externalDecl: None,
                }),
                cmt: SCode::noComment.clone(),
                info: Absyn::dummyInfo.clone(),
            }),
            env: NFSCodeEnv::emptyEnv.clone(),
            classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE,
        })
    });

pub(crate) static BUILTIN_CLOCK: std::sync::LazyLock<metamodelica::Ref<NFSCodeEnv::Item>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(NFSCodeEnv::Item::CLASS {
            cls: metamodelica::Ref::new(SCode::Element::CLASS {
                name: literal!("Clock"),
                prefixes: SCode::defaultPrefixes.clone(),
                encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
                partialPrefix: openmodelica_frontend_types::SCode::Partial::PARTIAL,
                restriction: openmodelica_frontend_types::SCode::Restriction::R_CLASS,
                classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
                    elementLst: metamodelica::nil(),
                    normalEquationLst: metamodelica::nil(),
                    initialEquationLst: metamodelica::nil(),
                    normalAlgorithmLst: metamodelica::nil(),
                    initialAlgorithmLst: metamodelica::nil(),
                    constraintLst: metamodelica::nil(),
                    clsattrs: metamodelica::nil(),
                    externalDecl: None,
                }),
                cmt: SCode::noComment.clone(),
                info: Absyn::dummyInfo.clone(),
            }),
            env: NFSCodeEnv::emptyEnv.clone(),
            classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE,
        })
    });

pub(crate) fn lookupSimpleName(
    mut inName: &ArcStr,
    mut inEnv: &Env,
) -> Result<(Item, metamodelica::Ref<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let mut outEnv: Env;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(lookupSimpleName2(inName, inEnv, &(metamodelica::nil()))?) {
        (Some(__pa0), Some(__pa1), Some(__pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outItem = metamodelica::Own::own(__pa0);
    outPath = metamodelica::Own::own(__pa1);
    outEnv = metamodelica::Own::own(__pa2);
    Ok((outItem, outPath, outEnv))
}

fn lookupSimpleName2(
    mut inName: &ArcStr,
    mut inEnv: &Env,
    mut inVisitedScopes: &metamodelica::List<ArcStr>,
) -> Result<(
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
)> {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outPath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = &**inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    (opt_item, opt_path, opt_env) = lookupInLocalScope(inName, inEnv, inVisitedScopes)?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { name: Some(scope_name), frameType: frame_type, .. }, tail: rest_env } => {
                    let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    frameNotEncapsulated(frame_type.clone())?;
                    (opt_item, opt_path, opt_env) = lookupSimpleName2(inName, metamodelica::AsArg::as_arg(&rest_env), &(metamodelica::cons(scope_name.clone(), inVisitedScopes.clone())))?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: NFSCodeEnv::FrameType::ENCAPSULATED_SCOPE { .. }, .. }, tail: rest_env } => {
                    let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    let mut rest_env = (*rest_env).clone();
                    rest_env = NFSCodeEnv::getEnvTopScope(rest_env.clone())?;
                    (opt_item, opt_path, opt_env) = lookupSimpleName2(inName, metamodelica::AsArg::as_arg(&rest_env), &(metamodelica::nil()))?;
                    checkBuiltinItem(opt_item.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn frameNotEncapsulated(mut frameType: FrameType) -> Result<()> {
    let () = (match frameType {
        NFSCodeEnv::FrameType::ENCAPSULATED_SCOPE { .. } => return Err("fail"),
        _ => (),
    });
    Ok(())
}

fn checkBuiltinItem(mut inItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inItem) {
        Some(Deref @ NFSCodeEnv::Item::CLASS { classType: NFSCodeEnv::ClassType::BUILTIN { .. }, .. }) => (),
        None => (),
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn lookupInLocalScope(
    mut inName: &ArcStr,
    mut inEnv: &Env,
    mut inVisitedScopes: &metamodelica::List<ArcStr>,
) -> Result<(
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
)> {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outPath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = &**inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut env: Env;
                    let mut item: Item;
                    (item, env) = lookupInClass(inName.clone(), inEnv.clone())?;
                    Ok((Some(item.clone()), Some(metamodelica::Ref::new(Absyn::Path::IDENT { name: inName.clone() })), Some(env.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    (opt_item, opt_path, opt_env) = lookupInBaseClasses(inName, inEnv, crate::NFSCodeLookup::RedeclareReplaceStrategy::INSERT_REDECLARES, inVisitedScopes)?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { importTable: NFSCodeEnv::ImportTable { hidden: false, qualifiedImports: imps, .. }, .. }, tail: _ } => {
                    let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    (opt_item, opt_path, opt_env) = lookupInQualifiedImports(inName, metamodelica::AsArg::as_arg(&imps), inEnv)?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { importTable: NFSCodeEnv::ImportTable { hidden: false, unqualifiedImports: imps, .. }, .. }, tail: _ } => {
                    let mut env: Env;
                    let mut item: Item;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    (item, path, env) = lookupInUnqualifiedImports(inName, metamodelica::AsArg::as_arg(&imps), inEnv)?;
                    Ok((Some(item.clone()), Some(path.clone()), Some(env.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: NFSCodeEnv::FrameType::IMPLICIT_SCOPE { .. }, .. }, tail: rest_env } => {
                    let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    (opt_item, opt_path, opt_env) = lookupInLocalScope(inName, metamodelica::AsArg::as_arg(&rest_env), inVisitedScopes)?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn lookupInClass(mut inName: ArcStr, mut inEnv: Env) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env;
    let mut tree: metamodelica::Ref<NFSCodeEnv::EnvTree::Tree>;
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: __pa0, .. }, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    tree = metamodelica::Own::own(__pa0);
    outItem = NFSCodeEnv::EnvTree::get(&tree, inName)?;
    (outItem, outEnv) = resolveAlias(outItem, inEnv)?;
    Ok((outItem, outEnv))
}

pub(crate) fn resolveAlias(mut inItem: Item, mut inEnv: Env) -> Result<(Item, Env)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inItem.clone(), inEnv.clone())) {
            (Deref @ NFSCodeEnv::Item::ALIAS { name, path: None, .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: tree, .. }, tail: _ }) => {
                let mut item: Item;
                let mut env: Env;
                item = NFSCodeEnv::EnvTree::get(metamodelica::AsArg::as_arg(&tree), name.clone())?;
                { (inItem, inEnv) = (item, inEnv); continue '__tco; }
            },
            (Deref @ NFSCodeEnv::Item::ALIAS { name, path: Some(path), .. }, _) => {
                let mut item: Item;
                let mut env: Env;
                let mut tree: metamodelica::Ref<NFSCodeEnv::EnvTree::Tree>;
                env = NFSCodeEnv::getEnvTopScope(inEnv)?;
                env = NFSCodeEnv::enterScopePath(env, metamodelica::AsArg::as_arg(&path))?;
                let __pa0 = ::match_deref::match_deref! { match &(env.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: __pa0, .. }, tail: _ } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                tree = metamodelica::Own::own(__pa0);
                item = NFSCodeEnv::EnvTree::get(&tree, name.clone())?;
                { (inItem, inEnv) = (item, env); continue '__tco; }
            },
            _ => {
                return Ok((inItem, inEnv))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lookupInBaseClasses(
    mut inName: &ArcStr,
    mut inEnv: &Env,
    mut inReplaceRedeclares: RedeclareReplaceStrategy,
    mut inVisitedScopes: &metamodelica::List<ArcStr>,
) -> Result<(
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
)> {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outPath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    let mut env: Env;
    let mut bcl: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
    let __pa0 = ::match_deref::match_deref! { match &((*inEnv)) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, .. }, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    bcl = metamodelica::Own::own(__pa0);
    env = NFSCodeEnv::removeExtendsFromLocalScope(inEnv)?;
    env = NFSCodeEnv::setImportTableHidden(&env, false)?;
    (outItem, outPath, outEnv) = lookupInBaseClasses2(inName, &bcl, &env, inEnv, inReplaceRedeclares, inVisitedScopes)?;
    Ok((outItem, outPath, outEnv))
}

fn lookupInBaseClasses2(
    mut inName: &ArcStr,
    mut inBaseClasses: &metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>,
    mut inEnv: &Env,
    mut inEnvWithExtends: &Env,
    mut inReplaceRedeclares: RedeclareReplaceStrategy,
    mut inVisitedScopes: &metamodelica::List<ArcStr>,
) -> Result<(
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
)> {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outPath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = &**inBaseClasses;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: ext, tail: _ } => {
                    let mut item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut path: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    (item, path, env) = lookupInBaseClasses3(inName.clone(), metamodelica::AsArg::as_arg(&ext), inEnv.clone(), inEnvWithExtends.clone(), inReplaceRedeclares, inVisitedScopes.clone())?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_ext } => {
                    let mut item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut path: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    (item, path, env) = lookupInBaseClasses2(inName, metamodelica::AsArg::as_arg(&rest_ext), inEnv, inEnvWithExtends, inReplaceRedeclares, inVisitedScopes)?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn lookupInBaseClasses3(
    mut inName: ArcStr,
    mut inBaseClass: &Extends,
    mut inEnv: Env,
    mut inEnvWithExtends: Env,
    mut inReplaceRedeclares: RedeclareReplaceStrategy,
    mut inVisitedScopes: metamodelica::List<ArcStr>,
) -> Result<(
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
)> {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outPath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    (outItem, outPath, outEnv) = (::match_deref::match_deref! { match inBaseClass {
        Deref @ NFSCodeEnv::Extends { baseClass: bc @ Deref @ Absyn::Path::QUALIFIED { name: Deref @ "$E", .. }, info, .. } => {
            NFEnvExtends::printExtendsError(bc.clone(), &inEnvWithExtends, info)?;
            (None, None, None)
        },
        Deref @ NFSCodeEnv::Extends { baseClass: bc, redeclareModifiers: redecls, info, .. } => {
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut item: Item;
            let mut env: Env;
            let mut opt_path: Option<metamodelica::Ref<Absyn::Path>>;
            let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
            let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
            (item, path, env) = lookupBaseClassName(bc.clone(), inEnv.clone(), info)?;
            let true = (checkVisitedScopes(inVisitedScopes, &inEnv, path)) else { return Err("pattern mismatch") };
            item = NFSCodeEnv::setImportsInItemHidden(item, true)?;
            (opt_item, opt_env) = NFSCodeFlattenRedeclare::replaceRedeclares(redecls.clone(), item, env, inEnvWithExtends, inReplaceRedeclares);
            (opt_item, opt_path, opt_env) = lookupInBaseClasses4(&(metamodelica::Ref::new(Absyn::Path::IDENT { name: inName })), opt_item, opt_env)?;
            (opt_item, opt_path, opt_env)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outItem, outPath, outEnv))
}

fn checkVisitedScopes(
    mut inVisitedScopes: metamodelica::List<ArcStr>,
    mut inEnv: &Env,
    mut inBaseClass: metamodelica::Ref<Absyn::Path>,
) -> bool {
    let mut outRes: bool;
    outRes = 'mc: {
        let __mc_input = &*inVisitedScopes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
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
                    let mut env_path: metamodelica::Ref<Absyn::Path>;
                    let mut visited_path: metamodelica::Ref<Absyn::Path>;
                    let mut bc_path: metamodelica::Ref<Absyn::Path>;
                    env_path = NFSCodeEnv::getEnvPath(inEnv)?;
                    bc_path = AbsynUtil::removePrefix(env_path.clone(), inBaseClass.clone())?;
                    visited_path = AbsynUtil::stringListPath(inVisitedScopes.clone())?;
                    let true = (AbsynUtil::pathPrefixOf(visited_path.clone(), bc_path.clone())) else { return Err("pattern mismatch") };
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outRes
}

fn lookupInBaseClasses4(
    mut inName: &metamodelica::Ref<Absyn::Path>,
    mut inItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    mut inEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
) -> Result<(
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
)> {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outPath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    (outItem, outPath, outEnv) = (::match_deref::match_deref! { match &((inItem, inEnv)) {
        (None, None) => {
            (None, None, None)
        },
        (Some(item), Some(env)) => {
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut item = (*item).clone();
            let mut env = (*env).clone();
            (item, path, env) = lookupNameInItem(inName, item.clone(), env.clone())?;
            (Some(item.clone()), Some(path), Some(env.clone()))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn lookupInQualifiedImports(
    mut inName: &ArcStr,
    mut inImports: &metamodelica::List<Absyn::Import>,
    mut inEnv: &Env,
) -> Result<(
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
)> {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outPath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = &**inImports;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::NAMED_IMPORT { name, .. }, tail: rest_imps } => {
                    let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    let false = (stringEqual(&inName, &name)) else { return Err("pattern mismatch") };
                    (opt_item, opt_path, opt_env) = lookupInQualifiedImports(inName, metamodelica::AsArg::as_arg(&rest_imps), inEnv)?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::NAMED_IMPORT { name, path }, tail: _ } => {
                    let mut item: Item;
                    let mut env: Env;
                    let mut path = (*path).clone();
                    let true = (stringEqual(&inName, &name)) else { return Err("pattern mismatch") };
                    (item, path, env) = lookupFullyQualified(metamodelica::AsArg::as_arg(&path), inEnv.clone())?;
                    Ok((Some(item.clone()), Some(path.clone()), Some(env.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::NAMED_IMPORT { name, .. }, tail: _ } => {
                    let true = (stringEqual(&inName, &name)) else { return Err("pattern mismatch") };
                    Ok((None, None, None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn lookupInUnqualifiedImports(
    mut inName: &ArcStr,
    mut inImports: &metamodelica::List<Absyn::Import>,
    mut inEnv: &Env,
) -> Result<(Item, metamodelica::Ref<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = &**inImports;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: _ } => {
                    let mut item: Item;
                    let mut path2: metamodelica::Ref<Absyn::Path>;
                    let mut env: Env;
                    let mut path = (*path).clone();
                    (item, path, env) = lookupFullyQualified(metamodelica::AsArg::as_arg(&path), inEnv.clone())?;
                    (item, path2, env) = lookupNameInItem(&(metamodelica::Ref::new(Absyn::Path::IDENT { name: inName.clone() })), item.clone(), env.clone())?;
                    path = joinPaths(metamodelica::AsArg::as_arg(&path), &path2)?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_imps } => {
                    let mut item: Item;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut env: Env;
                    (item, path, env) = lookupInUnqualifiedImports(inName, metamodelica::AsArg::as_arg(&rest_imps), inEnv)?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn lookupFullyQualified(
    mut inName: &metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
) -> Result<(Item, metamodelica::Ref<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let mut outEnv: Env;
    let mut env: Env;
    env = NFSCodeEnv::getEnvTopScope(inEnv)?;
    (outItem, outPath, outEnv) = lookupNameInPackage(inName, &env)?;
    outPath = AbsynUtil::makeFullyQualified(outPath);
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn lookupNameInPackage(
    mut inName: &metamodelica::Ref<Absyn::Path>,
    mut inEnv: &Env,
) -> Result<(Item, metamodelica::Ref<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outPath, outEnv) = (::match_deref::match_deref! { match (inName, inEnv) {
        (Deref @ Absyn::Path::IDENT { name }, _) => {
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut env: Env;
            let mut item: Item;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(lookupInLocalScope(name, inEnv, &(metamodelica::nil()))?) {
                (Some(__pa0), Some(__pa1), Some(__pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            item = metamodelica::Own::own(__pa0);
            path = metamodelica::Own::own(__pa1);
            env = metamodelica::Own::own(__pa2);
            env = NFSCodeEnv::setImportTableHidden(&env, false)?;
            (item, path, env)
        },
        (Deref @ Absyn::Path::QUALIFIED { name, path }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
            let mut new_path: metamodelica::Ref<Absyn::Path>;
            let mut env: Env;
            let mut item: Item;
            let mut path = (*path).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(lookupInLocalScope(name, inEnv, &(metamodelica::nil()))?) {
                (Some(__pa0), Some(__pa1), Some(__pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            item = metamodelica::Own::own(__pa0);
            new_path = metamodelica::Own::own(__pa1);
            env = metamodelica::Own::own(__pa2);
            env = NFSCodeEnv::setImportTableHidden(&env, false)?;
            (item, path, env) = lookupNameInItem(metamodelica::AsArg::as_arg(&path), item, env)?;
            path = joinPaths(&new_path, metamodelica::AsArg::as_arg(&path))?;
            (item, path.clone(), env)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outItem, outPath, outEnv))
}

pub(crate) fn lookupCrefInPackage(
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inEnv: &Env,
) -> Result<(Item, metamodelica::Ref<Absyn::ComponentRef>)> {
    let mut outItem: Item;
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    (outItem, outCref) = 'mc: {
        let __mc_input = &**inCref;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts: subs } => {
                    let mut new_path: metamodelica::Ref<Absyn::Path>;
                    let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut item: Item;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInLocalScope(metamodelica::AsArg::as_arg(&name), inEnv, &(metamodelica::nil()))?) {
                        (Some(__pa0), Some(__pa1), _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    item = metamodelica::Own::own(__pa0);
                    new_path = metamodelica::Own::own(__pa1);
                    cref = AbsynUtil::pathToCrefWithSubs(&new_path, metamodelica::AsArg::as_arg(&subs));
                    Ok((item.clone(), cref.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts: subs, componentRef: cref_rest } => {
                    let mut new_path: metamodelica::Ref<Absyn::Path>;
                    let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut item: Item;
                    let mut env: Env;
                    let mut cref_rest = (*cref_rest).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(lookupInLocalScope(metamodelica::AsArg::as_arg(&name), inEnv, &(metamodelica::nil()))?) {
                        (Some(__pa0), Some(__pa1), Some(__pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    item = metamodelica::Own::own(__pa0);
                    new_path = metamodelica::Own::own(__pa1);
                    env = metamodelica::Own::own(__pa2);
                    (item, cref_rest) = lookupCrefInItem(metamodelica::AsArg::as_arg(&cref_rest), item.clone(), env.clone())?;
                    if '__try3: {
                        ::match_deref::match_deref! { match &(cref_rest.clone()) {
                            Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: _ } => (),
                            _ => break '__try3 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    cref = AbsynUtil::pathToCrefWithSubs(&new_path, metamodelica::AsArg::as_arg(&subs));
                    cref = AbsynUtil::joinCrefs(&cref, cref_rest.clone())?;
                    Ok((item.clone(), cref.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { name, componentRef: cref_rest, .. } => {
                    let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut item: Item;
                    let mut env: Env;
                    let mut cref_rest = (*cref_rest).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInLocalScope(metamodelica::AsArg::as_arg(&name), inEnv, &(metamodelica::nil()))?) {
                        (Some(__pa0), Some(_), Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    item = metamodelica::Own::own(__pa0);
                    env = metamodelica::Own::own(__pa1);
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(lookupCrefInItem(metamodelica::AsArg::as_arg(&cref_rest), item.clone(), env.clone())?) {
                        (__pa2, __pa3 @ Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: _ }) => (__pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    item = metamodelica::Own::own(__pa2);
                    cref_rest = metamodelica::Own::own(__pa3);
                    cref = cref_rest.clone();
                    Ok((item.clone(), cref.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outCref))
}

pub(crate) fn lookupNameInItem<'__b>(
    mut inName: &'__b metamodelica::Ref<Absyn::Path>,
    mut inItem: Item,
    mut inEnv: Env,
) -> Result<(Item, metamodelica::Ref<Absyn::Path>, Env)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inItem, inEnv.clone())) {
            (Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { typeSpec: type_spec, modifications: mods, info, .. }, .. }, env) => {
                let mut item: Item;
                let mut path: metamodelica::Ref<Absyn::Path>;
                let mut type_env: Env;
                let mut redeclares: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>;
                let mut env = (*env).clone();
                (item, _, type_env) = lookupTypeSpec(type_spec.clone(), env.clone(), metamodelica::AsArg::as_arg(&info))?;
                redeclares = NFSCodeFlattenRedeclare::extractRedeclaresFromModifier(metamodelica::AsArg::as_arg(&mods))?;
                (item, type_env, _) = NFSCodeFlattenRedeclare::replaceRedeclaredElementsInEnv(redeclares, item, type_env, inEnv, NFInstPrefix::emptyPrefix().clone())?;
                { (inName, inItem, inEnv) = (inName, item, type_env); continue '__tco; }
            },
            (Deref @ NFSCodeEnv::Item::CLASS { env: Deref @ metamodelica::ListNode::Cons { head: class_env, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
                let mut item: Item;
                let mut path: metamodelica::Ref<Absyn::Path>;
                let mut env: Env;
                env = NFSCodeEnv::enterFrame(class_env.clone(), inEnv);
                return Ok(lookupNameInPackage(inName, &env)?)
            },
            (Deref @ NFSCodeEnv::Item::REDECLARED_ITEM { item, declaredEnv: env }, _) => {
                let mut path: metamodelica::Ref<Absyn::Path>;
                let mut item = (*item).clone();
                let mut env = (*env).clone();
                { (inName, inItem, inEnv) = (inName, item.clone(), env.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn lookupCrefInItem<'__b>(
    mut inCref: &'__b metamodelica::Ref<Absyn::ComponentRef>,
    mut inItem: Item,
    mut inEnv: Env,
) -> Result<(Item, metamodelica::Ref<Absyn::ComponentRef>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inItem) {
            Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { typeSpec: type_spec, modifications: mods, info, .. }, .. } => {
                let mut item: Item;
                let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
                let mut type_env: Env;
                let mut redeclares: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>;
                (item, _, type_env) = lookupTypeSpec(type_spec.clone(), inEnv.clone(), metamodelica::AsArg::as_arg(&info))?;
                redeclares = NFSCodeFlattenRedeclare::extractRedeclaresFromModifier(metamodelica::AsArg::as_arg(&mods))?;
                (item, type_env, _) = NFSCodeFlattenRedeclare::replaceRedeclaredElementsInEnv(redeclares, item, type_env, inEnv, NFInstPrefix::emptyPrefix().clone())?;
                { (inCref, inItem, inEnv) = (inCref, item, type_env); continue '__tco; }
            },
            Deref @ NFSCodeEnv::Item::CLASS { env: Deref @ metamodelica::ListNode::Cons { head: class_env, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                let mut item: Item;
                let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
                let mut env: Env;
                env = NFSCodeEnv::enterFrame(class_env.clone(), inEnv);
                return Ok(lookupCrefInPackage(inCref, &env)?)
            },
            Deref @ NFSCodeEnv::Item::REDECLARED_ITEM { item, declaredEnv: env } => {
                let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
                let mut item = (*item).clone();
                { (inCref, inItem, inEnv) = (inCref, item.clone(), env.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn lookupBaseClasses(
    mut inName: ArcStr,
    mut inEnv: Env,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut outBaseClasses: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut bcl: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, .. }, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    bcl = metamodelica::Own::own(__pa0);
    (_, outBaseClasses) = List::fold22(
        &bcl,
        &move |__a0: metamodelica::Ref<NFSCodeEnv::Extends>,
               __a1: ArcStr,
               __a2: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
               __a3: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Item>>,
               __a4: metamodelica::List<metamodelica::Ref<Absyn::Path>>|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(lookupBaseClasses2(&__a0, __a1, __a2, __a3, __a4))
        },
        inName,
        inEnv,
        metamodelica::nil(),
        metamodelica::nil(),
    )?;
    let false = ((outBaseClasses).is_empty()) else {
        return Err("pattern mismatch");
    };
    outBaseClasses = outBaseClasses.reverse();
    Ok(outBaseClasses)
}

fn lookupBaseClasses2(
    mut inBaseClass: &Extends,
    mut inName: ArcStr,
    mut inEnv: Env,
    mut items: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Item>>,
    mut bcl: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> (
    metamodelica::List<metamodelica::Ref<NFSCodeEnv::Item>>,
    metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) {
    let mut items: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Item>> = items;
    let mut bcl: metamodelica::List<metamodelica::Ref<Absyn::Path>> = bcl;
    (items, bcl) = 'mc: {
        let __mc_input = &**inBaseClass;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Extends { baseClass: bc, info, .. } => {
                    let mut env: Env;
                    let mut item: Item;
                    (item, _, env) = lookupBaseClassName(bc.clone(), inEnv.clone(), metamodelica::AsArg::as_arg(&info))?;
                    item = NFSCodeEnv::setImportsInItemHidden(item.clone(), true)?;
                    (item, _, _) = lookupNameInItem(&(metamodelica::Ref::new(Absyn::Path::IDENT { name: inName.clone() })), item.clone(), env.clone())?;
                    Ok((metamodelica::cons(item.clone(), items.clone()), metamodelica::cons(bc.clone(), bcl.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((items.clone(), bcl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (items, bcl)
}

pub(crate) fn lookupInheritedName(mut inName: &ArcStr, mut inEnv: &Env) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInBaseClasses(inName, inEnv, crate::NFSCodeLookup::RedeclareReplaceStrategy::INSERT_REDECLARES, &(metamodelica::nil()))?) {
        (Some(__pa0), _, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outItem = metamodelica::Own::own(__pa0);
    outEnv = metamodelica::Own::own(__pa1);
    Ok((outItem, outEnv))
}

pub(crate) fn lookupInheritedNameAndBC(
    mut inName: ArcStr,
    mut inEnv: Env,
) -> Result<(
    metamodelica::List<metamodelica::Ref<NFSCodeEnv::Item>>,
    metamodelica::List<metamodelica::Ref<Absyn::Path>>,
)> {
    let mut outItems: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outBaseClasses: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut bcl: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, .. }, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    bcl = metamodelica::Own::own(__pa0);
    (outItems, outBaseClasses) = List::fold22(
        &bcl,
        &move |__a0: metamodelica::Ref<NFSCodeEnv::Extends>,
               __a1: ArcStr,
               __a2: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
               __a3: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Item>>,
               __a4: metamodelica::List<metamodelica::Ref<Absyn::Path>>|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(lookupBaseClasses2(&__a0, __a1, __a2, __a3, __a4))
        },
        inName,
        inEnv,
        metamodelica::nil(),
        metamodelica::nil(),
    )?;
    outBaseClasses = outBaseClasses.reverse();
    outItems = outItems.reverse();
    Ok((outItems, outBaseClasses))
}

pub(crate) fn lookupRedeclaredClassByItem(
    mut inItem: &Item,
    mut inEnv: &Env,
    mut inInfo: &SourceInfo,
) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env;
    (outItem, outEnv) = 'mc: {
        let __mc_input = &**inItem;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name, .. }, .. } => {
                    let mut item: Item;
                    let mut env: Env;
                    let mut rdp: SCode::Redeclare;
                    let mut rpp: metamodelica::Ref<SCode::Replaceable>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInBaseClasses(metamodelica::AsArg::as_arg(&name), inEnv, crate::NFSCodeLookup::RedeclareReplaceStrategy::IGNORE_REDECLARES, &(metamodelica::nil()))?) {
                        (Some(__pa0), _, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    item = metamodelica::Own::own(__pa0);
                    env = metamodelica::Own::own(__pa1);
                    let __arc4 = NFSCodeEnv::getItemPrefixes(&item)?;
                    let SCode::PREFIXES { redeclarePrefix: __pa2, replaceablePrefix: __pa3, .. } = &*__arc4;
                    rdp = metamodelica::Own::own(__pa2);
                    rpp = metamodelica::Own::own(__pa3);
                    (item, env) = lookupRedeclaredClass2(&item, rdp, &rpp, &env, inInfo)?;
                    Ok((item.clone(), env.clone()))
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeLookup.lookupRedeclaredClassByItem failed on ")); __mm_s.push_str(&*NFSCodeEnv::getItemName(inItem)?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv)); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outEnv))
}

fn lookupRedeclaredClass2(
    mut inItem: &Item,
    mut inRedeclarePrefix: SCode::Redeclare,
    mut inReplaceablePrefix: &metamodelica::Ref<SCode::Replaceable>,
    mut inEnv: &Env,
    mut inInfo: &SourceInfo,
) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env;
    (outItem, outEnv) = 'mc: {
        let __mc_input = (&**inItem, inRedeclarePrefix, &**inReplaceablePrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, SCode::Redeclare::NOT_REDECLARE { .. }, Deref @ SCode::Replaceable::REPLACEABLE { .. }) => {
                    Ok((inItem.clone(), inEnv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name, .. }, .. }, SCode::Redeclare::REDECLARE { .. }, Deref @ SCode::Replaceable::REPLACEABLE { .. }) => {
                    let mut item: Item;
                    let mut env: Env;
                    let mut rdp: SCode::Redeclare;
                    let mut rpp: metamodelica::Ref<SCode::Replaceable>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInBaseClasses(metamodelica::AsArg::as_arg(&name), inEnv, crate::NFSCodeLookup::RedeclareReplaceStrategy::IGNORE_REDECLARES, &(metamodelica::nil()))?) {
                        (Some(__pa0), _, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    item = metamodelica::Own::own(__pa0);
                    env = metamodelica::Own::own(__pa1);
                    let __arc4 = NFSCodeEnv::getItemPrefixes(&item)?;
                    let SCode::PREFIXES { redeclarePrefix: __pa2, replaceablePrefix: __pa3, .. } = &*__arc4;
                    rdp = metamodelica::Own::own(__pa2);
                    rpp = metamodelica::Own::own(__pa3);
                    (item, env) = lookupRedeclaredClass2(&item, rdp, &rpp, &env, inInfo)?;
                    Ok((item.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::REDECLARED_ITEM { item, declaredEnv: env }, _, _) => {
                    let mut item = (*item).clone();
                    let mut env = (*env).clone();
                    (item, env) = lookupRedeclaredClass2(metamodelica::AsArg::as_arg(&item), inRedeclarePrefix, inReplaceablePrefix, metamodelica::AsArg::as_arg(&env), inInfo)?;
                    Ok((item.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name, info, .. }, .. }, _, Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. }) => {
                    Error::addSourceMessage(&(Error::ERROR_FROM_HERE.clone()), metamodelica::nil(), inInfo)?;
                    Error::addSourceMessage(&(Error::REDECLARE_NON_REPLACEABLE.clone()), list![name.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { name, info, .. }, .. }, _, _) => {
                    Error::addSourceMessage(&(Error::ERROR_FROM_HERE.clone()), metamodelica::nil(), inInfo)?;
                    Error::addSourceMessage(&(Error::INVALID_REDECLARE_AS.clone()), list![literal!("component"), name.clone(), literal!("a class")], metamodelica::AsArg::as_arg(&info))?;
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
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeLookup.lookupRedeclaredClass2 failed on ")); __mm_s.push_str(&*NFSCodeEnv::getItemName(inItem)?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv)); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outEnv))
}

pub(crate) fn lookupBuiltinType(mut inName: &ArcStr) -> Result<Item> {
    let mut outItem: Item;
    outItem = (::match_deref::match_deref! { match &(inName.clone()) {
        Deref @ "Real" => BUILTIN_REAL.clone(),
        Deref @ "Integer" => BUILTIN_INTEGER.clone(),
        Deref @ "Boolean" => BUILTIN_BOOLEAN.clone(),
        Deref @ "String" => BUILTIN_STRING.clone(),
        Deref @ "StateSelect" => BUILTIN_STATESELECT.clone(),
        Deref @ "ExternalObject" => BUILTIN_EXTERNALOBJECT.clone(),
        Deref @ "Clock" => {
            let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
            BUILTIN_CLOCK.clone()
        },
        Deref @ "$RealType" => BUILTIN_REALTYPE_ITEM.clone(),
        Deref @ "$IntegerType" => BUILTIN_INTEGERTYPE_ITEM.clone(),
        Deref @ "$BooleanType" => BUILTIN_BOOLEANTYPE_ITEM.clone(),
        Deref @ "$StringType" => BUILTIN_STRINGTYPE_ITEM.clone(),
        Deref @ "$EnumType" => BUILTIN_ENUMTYPE_ITEM.clone(),
        _ => return Err("match: no arm matched"),
    } });
    Ok(outItem)
}

fn lookupBuiltinName(mut inName: &metamodelica::Ref<Absyn::Path>) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env;
    (outItem, outEnv) = (::match_deref::match_deref! { match inName {
        Deref @ Absyn::Path::IDENT { name: id } => {
            let mut item: Item;
            item = lookupBuiltinType(id)?;
            (item, NFSCodeEnv::emptyEnv.clone())
        },
        Deref @ Absyn::Path::QUALIFIED { name: Deref @ "StateSelect", path: Deref @ Absyn::Path::IDENT { name: id } } => {
            let mut item: Item;
            (item, _) = lookupInClass(id.clone(), BUILTIN_STATESELECT_ENV.clone())?;
            (item, BUILTIN_STATESELECT_ENV.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outItem, outEnv))
}

fn lookupName(
    mut inName: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inLookupStrategy: LookupStrategy,
    mut inInfo: &SourceInfo,
    mut inErrorType: Option<ErrorTypes::Message>,
) -> Result<(Item, metamodelica::Ref<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: metamodelica::Ref<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outName, outEnv) = 'mc: {
        let __mc_input = (&*inName, inLookupStrategy, inErrorType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, LookupStrategy::LOOKUP_ANY { .. }, _) => {
                    let mut item: Item;
                    let mut env: Env;
                    (item, env) = lookupBuiltinName(&inName)?;
                    Ok((item.clone(), inName.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: id }, _, _) => {
                    let mut item: Item;
                    let mut new_path: metamodelica::Ref<Absyn::Path>;
                    let mut env: Env;
                    (item, new_path, env) = lookupSimpleName(metamodelica::AsArg::as_arg(&id), &inEnv)?;
                    Ok((item.clone(), new_path.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::QUALIFIED { name: id, path }, _, _) => {
                    let mut item: Item;
                    let mut new_path: metamodelica::Ref<Absyn::Path>;
                    let mut env: Env;
                    let mut path = (*path).clone();
                    (item, new_path, env) = lookupSimpleName(metamodelica::AsArg::as_arg(&id), &inEnv)?;
                    (item, path, env) = lookupNameInItem(metamodelica::AsArg::as_arg(&path), item.clone(), env.clone())?;
                    path = joinPaths(&new_path, metamodelica::AsArg::as_arg(&path))?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::FULLYQUALIFIED { path }, _, _) => {
                    let mut item: Item;
                    let mut env: Env;
                    let mut path = (*path).clone();
                    (item, path, env) = lookupFullyQualified(metamodelica::AsArg::as_arg(&path), inEnv.clone())?;
                    Ok((item.clone(), path.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(error_id)) => {
                    let mut name_str: ArcStr;
                    let mut env_str: ArcStr;
                    name_str = AbsynUtil::pathString(inName.clone(), literal!("."), true, false)?;
                    env_str = NFSCodeEnv::getEnvName(&inEnv);
                    Error::addSourceMessage(metamodelica::AsArg::as_arg(&error_id), list![name_str.clone(), env_str.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outName, outEnv))
}

fn joinPaths(
    mut inPath1: &metamodelica::Ref<Absyn::Path>,
    mut inPath2: &metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match (inPath1, inPath2) {
        (_, Deref @ Absyn::Path::FULLYQUALIFIED { .. }) => {
            inPath2.clone()
        },
        (Deref @ Absyn::Path::IDENT { name: id }, _) => {
            metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: id.clone(), path: inPath2.clone() })
        },
        (Deref @ Absyn::Path::QUALIFIED { name: id, path }, _) => {
            let mut path = (*path).clone();
            path = joinPaths(metamodelica::AsArg::as_arg(&path), inPath2)?;
            metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: id.clone(), path: path.clone() })
        },
        (Deref @ Absyn::Path::FULLYQUALIFIED { path }, _) => {
            let mut path = (*path).clone();
            path = joinPaths(metamodelica::AsArg::as_arg(&path), inPath2)?;
            AbsynUtil::makeFullyQualified(path.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outPath)
}

pub(crate) fn lookupNameSilent(
    mut inName: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
) -> Result<(Item, metamodelica::Ref<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: metamodelica::Ref<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outName, outEnv) = lookupName(
        inName,
        inEnv,
        crate::NFSCodeLookup::LookupStrategy::LOOKUP_ANY,
        inInfo,
        None,
    )?;
    Ok((outItem, outName, outEnv))
}

pub(crate) fn lookupNameSilentNoBuiltin(
    mut inName: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
) -> Result<(Item, metamodelica::Ref<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: metamodelica::Ref<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outName, outEnv) = lookupName(
        inName,
        inEnv,
        crate::NFSCodeLookup::LookupStrategy::NO_BUILTIN_TYPES,
        inInfo,
        None,
    )?;
    Ok((outItem, outName, outEnv))
}

pub fn lookupClassName(
    mut inName: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
) -> Result<(Item, metamodelica::Ref<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: metamodelica::Ref<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outName, outEnv) = lookupName(
        inName,
        inEnv,
        crate::NFSCodeLookup::LookupStrategy::LOOKUP_ANY,
        inInfo,
        Some(Error::LOOKUP_ERROR.clone()),
    )?;
    Ok((outItem, outName, outEnv))
}

pub(crate) fn lookupBaseClassName(
    mut inName: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
) -> Result<(Item, metamodelica::Ref<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: metamodelica::Ref<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outName, outEnv) = (::match_deref::match_deref! { match &((inName.clone(), inEnv.clone())) {
        (Deref @ Absyn::Path::QUALIFIED { name: Deref @ "$ce", path: path @ Deref @ Absyn::Path::IDENT { name: id } }, Deref @ metamodelica::ListNode::Cons { head: _, tail: env }) => {
            let mut item: Item;
            let mut env = (*env).clone();
            (item, env) = lookupInheritedName(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&env))?;
            (item, path.clone(), env.clone())
        },
        (Deref @ Absyn::Path::QUALIFIED { name: Deref @ "$E", .. }, _) => {
            NFEnvExtends::printExtendsError(inName, &inEnv, inInfo)?;
            return Err("fail")
        },
        _ => {
            let mut env: Env;
            let mut item: Item;
            let mut path: metamodelica::Ref<Absyn::Path>;
            (item, path, env) = lookupName(inName, inEnv, crate::NFSCodeLookup::LookupStrategy::LOOKUP_ANY, inInfo, Some(Error::LOOKUP_BASECLASS_ERROR.clone()))?;
            (item, path, env)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outItem, outName, outEnv))
}

pub(crate) fn lookupVariableName(
    mut inName: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
) -> Result<(Item, metamodelica::Ref<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: metamodelica::Ref<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outName, outEnv) = lookupName(
        inName,
        inEnv,
        crate::NFSCodeLookup::LookupStrategy::NO_BUILTIN_TYPES,
        inInfo,
        Some(Error::LOOKUP_VARIABLE_ERROR.clone()),
    )?;
    Ok((outItem, outName, outEnv))
}

pub(crate) fn lookupFunctionName(
    mut inName: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
) -> Result<(Item, metamodelica::Ref<Absyn::Path>, Env)> {
    let mut outItem: Item;
    let mut outName: metamodelica::Ref<Absyn::Path>;
    let mut outEnv: Env;
    (outItem, outName, outEnv) = lookupName(
        inName,
        inEnv,
        crate::NFSCodeLookup::LookupStrategy::NO_BUILTIN_TYPES,
        inInfo,
        Some(Error::LOOKUP_FUNCTION_ERROR.clone()),
    )?;
    Ok((outItem, outName, outEnv))
}

fn crefStripEnvPrefix(
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut inEnv: &Env,
) -> metamodelica::Ref<Absyn::ComponentRef> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = &**inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (Flags::isSet(Flags::STRIP_PREFIX.clone())?) else { return Err("pattern mismatch") };
                    Ok(inCref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut env_path: metamodelica::Ref<Absyn::Path>;
                    let mut cref1: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut cref2: metamodelica::Ref<Absyn::ComponentRef>;
                    let false = (Flags::isSet(Flags::SCODE_INST.clone())?) else { return Err("pattern mismatch") };
                    env_path = NFSCodeEnv::getEnvPath(inEnv)?;
                    cref1 = AbsynUtil::unqualifyCref(inCref.clone());
                    cref2 = crefStripEnvPrefix2(&cref1, &env_path)?;
                    let false = (AbsynUtil::crefEqual(&cref1, &cref2)?) else { return Err("pattern mismatch") };
                    Ok(cref2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outCref
}

fn crefStripEnvPrefix2(
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inEnvPath: &metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = (&**inCref, &**inEnvPath);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: id1, subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: cref }, Deref @ Absyn::Path::QUALIFIED { name: id2, path: env_path }) => {
                    let true = (stringEqual(&id1, &id2)) else { return Err("pattern mismatch") };
                    Ok(crefStripEnvPrefix2(metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&env_path))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: id1, subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: cref }, Deref @ Absyn::Path::IDENT { name: id2 }) => {
                    let true = (stringEqual(&id1, &id2)) else { return Err("pattern mismatch") };
                    Ok(cref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: id1, subscripts: Deref @ metamodelica::ListNode::Nil, .. }, Deref @ Absyn::Path::IDENT { name: id2 }) => {
                    let false = (stringEqual(&id1, &id2)) else { return Err("pattern mismatch") };
                    Ok(inCref.clone())
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

pub(crate) fn lookupComponentRef(
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
) -> metamodelica::Ref<Absyn::ComponentRef> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = &*inCref;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "StateSelect", subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { .. } } => {
                    Ok(inCref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::WILD { .. } => {
                    Ok(inCref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
                    cref = NFSCodeFlattenImports::flattenComponentRefSubs(&inCref, &inEnv, inInfo)?;
                    (cref, _) = lookupComponentRef2(&cref, inEnv.clone())?;
                    cref = crefStripEnvPrefix(cref.clone(), &inEnv);
                    Ok(cref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outCref
}

fn lookupComponentRef2(
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inEnv: Env,
) -> Result<(metamodelica::Ref<Absyn::ComponentRef>, Env)> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut outEnv: Env;
    (outCref, outEnv) = (match &**inCref {
        Absyn::ComponentRef::CREF_IDENT { name, subscripts: subs } => {
            let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut env: Env;
            (_, path, env) = lookupSimpleName(name, &inEnv)?;
            cref = AbsynUtil::pathToCrefWithSubs(&path, subs);
            (cref, env)
        }
        Absyn::ComponentRef::CREF_QUAL {
            name,
            subscripts: subs,
            componentRef: rest_cref,
        } => {
            let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
            let mut new_path: metamodelica::Ref<Absyn::Path>;
            let mut env: Env;
            let mut item: Item;
            let mut rest_cref = (*rest_cref).clone();
            (item, new_path, env) = lookupSimpleName(name, &inEnv)?;
            cref = AbsynUtil::pathToCrefWithSubs(&new_path, subs);
            (item, rest_cref) = lookupCrefInItem(metamodelica::AsArg::as_arg(&rest_cref), item, env.clone())?;
            cref = joinCrefs(&cref, rest_cref.clone())?;
            (cref, env)
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cref } => {
            let mut env: Env;
            let mut cref = (*cref).clone();
            cref = lookupCrefFullyQualified(metamodelica::AsArg::as_arg(&cref), inEnv.clone())?;
            env = NFSCodeEnv::getEnvTopScope(inEnv)?;
            (cref.clone(), env)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outCref, outEnv))
}

pub(crate) fn lookupCrefFullyQualified(
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inEnv: Env,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut env: Env;
    env = NFSCodeEnv::getEnvTopScope(inEnv.clone())?;
    (_, outCref) = lookupCrefInPackage(inCref, &inEnv)?;
    outCref = AbsynUtil::crefMakeFullyQualified(outCref);
    Ok(outCref)
}

pub(crate) fn joinCrefs(
    mut inCref1: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inCref2: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    outCref = (match &*inCref2 {
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => inCref2,
        _ => AbsynUtil::joinCrefs(inCref1, inCref2)?,
    });
    Ok(outCref)
}

pub(crate) fn lookupTypeSpec(
    mut inTypeSpec: metamodelica::Ref<Absyn::TypeSpec>,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
) -> Result<(Item, metamodelica::Ref<Absyn::TypeSpec>, Env)> {
    let mut outItem: Item;
    let mut outTypeSpec: metamodelica::Ref<Absyn::TypeSpec>;
    let mut outTypeEnv: Env;
    (outItem, outTypeSpec, outTypeEnv) = (::match_deref::match_deref! { match &(inTypeSpec.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { path, arrayDim: ad } => {
            let mut newpath: metamodelica::Ref<Absyn::Path>;
            let mut item: Item;
            let mut env: Env;
            (item, newpath, env) = lookupClassName(path.clone(), inEnv, inInfo)?;
            (item, metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: newpath, arrayDim: ad.clone() }), env)
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name }, .. } => {
            let mut cls: metamodelica::Ref<SCode::Element>;
            cls = makeDummyMetaType(name.clone());
            (metamodelica::Ref::new(NFSCodeEnv::Item::CLASS { cls: cls, env: NFSCodeEnv::emptyEnv.clone(), classType: crate::NFSCodeEnv::ClassType::BASIC_TYPE }), inTypeSpec, NFSCodeEnv::emptyEnv.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outItem, outTypeSpec, outTypeEnv))
}

fn makeDummyMetaType(mut inTypeName: ArcStr) -> metamodelica::Ref<SCode::Element> {
    let mut outClass: metamodelica::Ref<SCode::Element>;
    outClass = metamodelica::Ref::new(SCode::Element::CLASS {
        name: inTypeName,
        prefixes: SCode::defaultPrefixes.clone(),
        encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE,
        classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
            elementLst: metamodelica::nil(),
            normalEquationLst: metamodelica::nil(),
            initialEquationLst: metamodelica::nil(),
            normalAlgorithmLst: metamodelica::nil(),
            initialAlgorithmLst: metamodelica::nil(),
            constraintLst: metamodelica::nil(),
            clsattrs: metamodelica::nil(),
            externalDecl: None,
        }),
        cmt: SCode::noComment.clone(),
        info: Absyn::dummyInfo.clone(),
    });
    outClass
}

pub(crate) fn qualifyPath(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
    mut inErrorType: Option<ErrorTypes::Message>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = 'mc: {
        let __mc_input = &*inPath;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { name: id } => {
                    lookupBuiltinType(metamodelica::AsArg::as_arg(&id))?;
                    Ok(inPath.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut env: Env;
                    (_, path, env) = lookupName(inPath.clone(), inEnv.clone(), crate::NFSCodeLookup::LookupStrategy::NO_BUILTIN_TYPES, inInfo, inErrorType.clone())?;
                    path = NFSCodeEnv::mergePathWithEnvPath(path.clone(), &env);
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    Ok(path.clone())
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeLookup.qualifyPath failed on ")); __mm_s.push_str(&*AbsynUtil::pathString(inPath.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(&inEnv)); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
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
