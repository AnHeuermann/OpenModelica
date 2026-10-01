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

use crate::FGraph;
use crate::FGraphBuild;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::MetaUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_loader::Parser;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Mutable;

// protected imports
/* These imports were used in e.g. MSL 1.6. They should not be here anymore...
   If you need them, add them to the initial environment and recompile; they are not standard Modelica.
  import arcsin = asin;
  import arccos = acos;
  import arctan = atan;
  import ln = log;
*/
// Predefined DAE.Types
// Real arrays
thread_local! { static __T_REAL_ARRAY_DEFAULT_TLS: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }); }
pub(crate) fn T_REAL_ARRAY_DEFAULT() -> metamodelica::Ref<DAE::Type> {
    __T_REAL_ARRAY_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_REAL_ARRAY_1_DEFAULT_TLS: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 })] }); }
pub(crate) fn T_REAL_ARRAY_1_DEFAULT() -> metamodelica::Ref<DAE::Type> {
    __T_REAL_ARRAY_1_DEFAULT_TLS.with(|__t| __t.clone())
}

// Integer arrays
thread_local! { static __T_INT_ARRAY_1_DEFAULT_TLS: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 })] }); }
pub(crate) fn T_INT_ARRAY_1_DEFAULT() -> metamodelica::Ref<DAE::Type> {
    __T_INT_ARRAY_1_DEFAULT_TLS.with(|__t| __t.clone())
}

pub(crate) static commonPrefixes: std::sync::LazyLock<metamodelica::Ref<SCode::Prefixes>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Prefixes {
            visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC,
            redeclarePrefix: openmodelica_frontend_types::SCode::Redeclare::NOT_REDECLARE,
            finalPrefix: openmodelica_frontend_types::SCode::Final::FINAL,
            innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
            replaceablePrefix: openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE(),
        })
    });

pub(crate) static commonPrefixesNotFinal: std::sync::LazyLock<metamodelica::Ref<SCode::Prefixes>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Prefixes {
            visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC,
            redeclarePrefix: openmodelica_frontend_types::SCode::Redeclare::NOT_REDECLARE,
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
            replaceablePrefix: openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE(),
        })
    });

pub(crate) static attrConst: std::sync::LazyLock<SCode::Attributes> = std::sync::LazyLock::new(|| SCode::Attributes {
    arrayDims: metamodelica::nil(),
    connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
    parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
    variability: openmodelica_frontend_types::SCode::Variability::CONST,
    direction: openmodelica_ast::Absyn::Direction::BIDIR,
    isField: openmodelica_ast::Absyn::IsField::NONFIELD,
});

pub(crate) static attrParam: std::sync::LazyLock<SCode::Attributes> = std::sync::LazyLock::new(|| SCode::Attributes {
    arrayDims: metamodelica::nil(),
    connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
    parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
    variability: openmodelica_frontend_types::SCode::Variability::PARAM,
    direction: openmodelica_ast::Absyn::Direction::BIDIR,
    isField: openmodelica_ast::Absyn::IsField::NONFIELD,
});

pub(crate) static attrParamVectorNoDim: std::sync::LazyLock<SCode::Attributes> =
    std::sync::LazyLock::new(|| SCode::Attributes {
        arrayDims: list![openmodelica_ast::Absyn::Subscript::interned_NOSUB()],
        connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
        parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
        variability: openmodelica_frontend_types::SCode::Variability::PARAM,
        direction: openmodelica_ast::Absyn::Direction::BIDIR,
        isField: openmodelica_ast::Absyn::IsField::NONFIELD,
    });

//
// The primitive types
// These are the primitive types that are used to build the types
// Real, Integer etc.
pub(crate) static rlType: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::CLASS {
        name: literal!("RealType"),
        prefixes: commonPrefixes.clone(),
        encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL,
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
    })
});

pub(crate) static intType: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::CLASS {
        name: literal!("IntegerType"),
        prefixes: commonPrefixes.clone(),
        encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER,
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
    })
});

pub(crate) static strType: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::CLASS {
        name: literal!("StringType"),
        prefixes: commonPrefixes.clone(),
        encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING,
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
    })
});

pub(crate) static boolType: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::CLASS {
        name: literal!("BooleanType"),
        prefixes: commonPrefixes.clone(),
        encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN,
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
    })
});

pub(crate) static enumType: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::CLASS {
        name: literal!("EnumType"),
        prefixes: commonPrefixes.clone(),
        encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_ENUMERATION,
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
    })
});

pub(crate) static unit: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: literal!("unit"),
        prefixes: commonPrefixes.clone(),
        attributes: attrParam.clone(),
        typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("StringType"),
            }),
            arrayDim: None,
        }),
        modifications: metamodelica::Ref::new(SCode::Mod::MOD {
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
            subModLst: metamodelica::nil(),
            binding: Some(metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("") })),
            comment: None,
            info: Absyn::dummyInfo.clone(),
        }),
        comment: SCode::noComment.clone(),
        condition: None,
        info: Absyn::dummyInfo.clone(),
    })
});

pub(crate) static quantity: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: literal!("quantity"),
        prefixes: commonPrefixes.clone(),
        attributes: attrParam.clone(),
        typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("StringType"),
            }),
            arrayDim: None,
        }),
        modifications: metamodelica::Ref::new(SCode::Mod::MOD {
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
            subModLst: metamodelica::nil(),
            binding: Some(metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("") })),
            comment: None,
            info: Absyn::dummyInfo.clone(),
        }),
        comment: SCode::noComment.clone(),
        condition: None,
        info: Absyn::dummyInfo.clone(),
    })
});

pub(crate) static displayUnit: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("displayUnit"),
            prefixes: commonPrefixes.clone(),
            attributes: attrParam.clone(),
            typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("StringType"),
                }),
                arrayDim: None,
            }),
            modifications: metamodelica::Ref::new(SCode::Mod::MOD {
                finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                subModLst: metamodelica::nil(),
                binding: Some(metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("") })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            }),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub static min: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: literal!("min"),
        prefixes: commonPrefixes.clone(),
        attributes: attrParam.clone(),
        typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("RealType"),
            }),
            arrayDim: None,
        }),
        modifications: metamodelica::Ref::new(SCode::Mod::MOD {
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
            subModLst: metamodelica::nil(),
            binding: Some(metamodelica::Ref::new(Absyn::Exp::REAL {
                value: literal!("-1e+099"),
            })),
            comment: None,
            info: Absyn::dummyInfo.clone(),
        }),
        comment: SCode::noComment.clone(),
        condition: None,
        info: Absyn::dummyInfo.clone(),
    })
});

pub static max: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: literal!("max"),
        prefixes: commonPrefixes.clone(),
        attributes: attrParam.clone(),
        typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("RealType"),
            }),
            arrayDim: None,
        }),
        modifications: metamodelica::Ref::new(SCode::Mod::MOD {
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
            subModLst: metamodelica::nil(),
            binding: Some(metamodelica::Ref::new(Absyn::Exp::REAL {
                value: literal!("1e+099"),
            })),
            comment: None,
            info: Absyn::dummyInfo.clone(),
        }),
        comment: SCode::noComment.clone(),
        condition: None,
        info: Absyn::dummyInfo.clone(),
    })
});

pub(crate) static startOrigin: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("startOrigin"),
            prefixes: commonPrefixes.clone(),
            attributes: attrParam.clone(),
            typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("StringType"),
                }),
                arrayDim: None,
            }),
            modifications: metamodelica::Ref::new(SCode::Mod::MOD {
                finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                subModLst: metamodelica::nil(),
                binding: Some(metamodelica::Ref::new(Absyn::Exp::STRING {
                    value: literal!("undefined"),
                })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            }),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static realStart: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: literal!("start"),
        prefixes: commonPrefixes.clone(),
        attributes: attrParam.clone(),
        typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("RealType"),
            }),
            arrayDim: None,
        }),
        modifications: metamodelica::Ref::new(SCode::Mod::MOD {
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
            subModLst: metamodelica::nil(),
            binding: Some(metamodelica::Ref::new(Absyn::Exp::REAL { value: literal!("0.0") })),
            comment: None,
            info: Absyn::dummyInfo.clone(),
        }),
        comment: SCode::noComment.clone(),
        condition: None,
        info: Absyn::dummyInfo.clone(),
    })
});

pub(crate) static integerStart: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("start"),
            prefixes: commonPrefixes.clone(),
            attributes: attrParam.clone(),
            typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("IntegerType"),
                }),
                arrayDim: None,
            }),
            modifications: metamodelica::Ref::new(SCode::Mod::MOD {
                finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                subModLst: metamodelica::nil(),
                binding: Some(metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 0 })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            }),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static stringStart: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("start"),
            prefixes: commonPrefixes.clone(),
            attributes: attrParam.clone(),
            typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("StringType"),
                }),
                arrayDim: None,
            }),
            modifications: metamodelica::Ref::new(SCode::Mod::MOD {
                finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                subModLst: metamodelica::nil(),
                binding: Some(metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("") })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            }),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static booleanStart: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("start"),
            prefixes: commonPrefixes.clone(),
            attributes: attrParam.clone(),
            typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("BooleanType"),
                }),
                arrayDim: None,
            }),
            modifications: metamodelica::Ref::new(SCode::Mod::MOD {
                finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                subModLst: metamodelica::nil(),
                binding: Some(metamodelica::Ref::new(Absyn::Exp::BOOL { value: false })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            }),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static fixed: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: literal!("fixed"),
        prefixes: commonPrefixes.clone(),
        attributes: attrParam.clone(),
        typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("BooleanType"),
            }),
            arrayDim: None,
        }),
        modifications: metamodelica::Ref::new(SCode::Mod::MOD {
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
            subModLst: metamodelica::nil(),
            binding: Some(metamodelica::Ref::new(Absyn::Exp::BOOL { value: false })),
            comment: None,
            info: Absyn::dummyInfo.clone(),
        }),
        comment: SCode::noComment.clone(),
        condition: None,
        info: Absyn::dummyInfo.clone(),
    })
});

pub(crate) static nominal: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: literal!("nominal"),
        prefixes: commonPrefixes.clone(),
        attributes: attrParam.clone(),
        typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("RealType"),
            }),
            arrayDim: None,
        }),
        modifications: metamodelica::Ref::new(SCode::Mod::MOD {
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
            subModLst: metamodelica::nil(),
            binding: None,
            comment: None,
            info: Absyn::dummyInfo.clone(),
        }),
        comment: SCode::noComment.clone(),
        condition: None,
        info: Absyn::dummyInfo.clone(),
    })
});

pub(crate) static stateSelect: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("stateSelect"),
            prefixes: commonPrefixes.clone(),
            attributes: attrParam.clone(),
            typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("StateSelect"),
                }),
                arrayDim: None,
            }),
            modifications: metamodelica::Ref::new(SCode::Mod::MOD {
                finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                subModLst: metamodelica::nil(),
                binding: Some(metamodelica::Ref::new(Absyn::Exp::CREF {
                    componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                        name: literal!("StateSelect"),
                        subscripts: metamodelica::nil(),
                        componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                            name: literal!("default"),
                            subscripts: metamodelica::nil(),
                        }),
                    }),
                })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            }),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

// Extensions for uncertainties
pub(crate) static uncertainty: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("uncertain"),
            prefixes: commonPrefixes.clone(),
            attributes: attrParam.clone(),
            typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("Uncertainty"),
                }),
                arrayDim: None,
            }),
            modifications: metamodelica::Ref::new(SCode::Mod::MOD {
                finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                subModLst: metamodelica::nil(),
                binding: Some(metamodelica::Ref::new(Absyn::Exp::CREF {
                    componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                        name: literal!("Uncertainty"),
                        subscripts: metamodelica::nil(),
                        componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                            name: literal!("given"),
                            subscripts: metamodelica::nil(),
                        }),
                    }),
                })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            }),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static distribution: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("distribution"),
            prefixes: commonPrefixes.clone(),
            attributes: attrParam.clone(),
            typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("Distribution"),
                }),
                arrayDim: None,
            }),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

// Distribution is declared in ModelicaBuiltin.mo
// END Extensions for uncertainties
pub(crate) static stateSelectComps: std::sync::LazyLock<metamodelica::List<metamodelica::Ref<SCode::Element>>> =
    std::sync::LazyLock::new(|| {
        list![
            metamodelica::Ref::new(SCode::Element::COMPONENT {
                name: literal!("never"),
                prefixes: commonPrefixes.clone(),
                attributes: attrConst.clone(),
                typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("EnumType")
                    }),
                    arrayDim: None
                }),
                modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                comment: SCode::noComment.clone(),
                condition: None,
                info: Absyn::dummyInfo.clone()
            }),
            metamodelica::Ref::new(SCode::Element::COMPONENT {
                name: literal!("avoid"),
                prefixes: commonPrefixes.clone(),
                attributes: attrConst.clone(),
                typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("EnumType")
                    }),
                    arrayDim: None
                }),
                modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                comment: SCode::noComment.clone(),
                condition: None,
                info: Absyn::dummyInfo.clone()
            }),
            metamodelica::Ref::new(SCode::Element::COMPONENT {
                name: literal!("default"),
                prefixes: commonPrefixes.clone(),
                attributes: attrConst.clone(),
                typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("EnumType")
                    }),
                    arrayDim: None
                }),
                modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                comment: SCode::noComment.clone(),
                condition: None,
                info: Absyn::dummyInfo.clone()
            }),
            metamodelica::Ref::new(SCode::Element::COMPONENT {
                name: literal!("prefer"),
                prefixes: commonPrefixes.clone(),
                attributes: attrConst.clone(),
                typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("EnumType")
                    }),
                    arrayDim: None
                }),
                modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                comment: SCode::noComment.clone(),
                condition: None,
                info: Absyn::dummyInfo.clone()
            }),
            metamodelica::Ref::new(SCode::Element::COMPONENT {
                name: literal!("always"),
                prefixes: commonPrefixes.clone(),
                attributes: attrConst.clone(),
                typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("EnumType")
                    }),
                    arrayDim: None
                }),
                modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                comment: SCode::noComment.clone(),
                condition: None,
                info: Absyn::dummyInfo.clone()
            })
        ]
    });

pub(crate) static uncertaintyComps: std::sync::LazyLock<metamodelica::List<metamodelica::Ref<SCode::Element>>> =
    std::sync::LazyLock::new(|| {
        list![
            metamodelica::Ref::new(SCode::Element::COMPONENT {
                name: literal!("given"),
                prefixes: commonPrefixes.clone(),
                attributes: attrConst.clone(),
                typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("EnumType")
                    }),
                    arrayDim: None
                }),
                modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                comment: SCode::noComment.clone(),
                condition: None,
                info: Absyn::dummyInfo.clone()
            }),
            metamodelica::Ref::new(SCode::Element::COMPONENT {
                name: literal!("sought"),
                prefixes: commonPrefixes.clone(),
                attributes: attrConst.clone(),
                typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("EnumType")
                    }),
                    arrayDim: None
                }),
                modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                comment: SCode::noComment.clone(),
                condition: None,
                info: Absyn::dummyInfo.clone()
            }),
            metamodelica::Ref::new(SCode::Element::COMPONENT {
                name: literal!("refine"),
                prefixes: commonPrefixes.clone(),
                attributes: attrConst.clone(),
                typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("EnumType")
                    }),
                    arrayDim: None
                }),
                modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                comment: SCode::noComment.clone(),
                condition: None,
                info: Absyn::dummyInfo.clone()
            }),
            metamodelica::Ref::new(SCode::Element::COMPONENT {
                name: literal!("propagate"),
                prefixes: commonPrefixes.clone(),
                attributes: attrConst.clone(),
                typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("EnumType")
                    }),
                    arrayDim: None
                }),
                modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                comment: SCode::noComment.clone(),
                condition: None,
                info: Absyn::dummyInfo.clone()
            })
        ]
    });

pub(crate) static stateSelectType: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::CLASS {
            name: literal!("StateSelect"),
            prefixes: commonPrefixes.clone(),
            encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
            partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
            restriction: openmodelica_frontend_types::SCode::Restriction::R_ENUMERATION,
            classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
                elementLst: stateSelectComps.clone(),
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
        })
    });

pub(crate) static uncertaintyType: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::CLASS {
            name: literal!("Uncertainty"),
            prefixes: commonPrefixes.clone(),
            encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
            partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
            restriction: openmodelica_frontend_types::SCode::Restriction::R_ENUMERATION,
            classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
                elementLst: uncertaintyComps.clone(),
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
        })
    });

pub(crate) static ExternalObjectType: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::CLASS {
            name: literal!("ExternalObject"),
            prefixes: commonPrefixes.clone(),
            encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
            partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
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
        })
    });

// The Real type
pub(crate) static realType: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::CLASS {
        name: literal!("Real"),
        prefixes: commonPrefixes.clone(),
        encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL,
        classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
            elementLst: list![
                unit.clone(),
                quantity.clone(),
                displayUnit.clone(),
                min.clone(),
                max.clone(),
                realStart.clone(),
                fixed.clone(),
                nominal.clone(),
                stateSelect.clone(),
                uncertainty.clone(),
                distribution.clone(),
                startOrigin.clone()
            ],
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
    })
});

// The Integer type
pub(crate) static integerType: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::CLASS {
            name: literal!("Integer"),
            prefixes: commonPrefixes.clone(),
            encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
            partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
            restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER,
            classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
                elementLst: list![
                    quantity.clone(),
                    min.clone(),
                    max.clone(),
                    integerStart.clone(),
                    fixed.clone(),
                    uncertainty.clone(),
                    distribution.clone(),
                    startOrigin.clone()
                ],
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
        })
    });

// The String type
pub(crate) static stringType: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::CLASS {
        name: literal!("String"),
        prefixes: commonPrefixes.clone(),
        encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING,
        classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
            elementLst: list![quantity.clone(), stringStart.clone(), startOrigin.clone()],
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
    })
});

// The Boolean type
pub(crate) static booleanType: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::CLASS {
            name: literal!("Boolean"),
            prefixes: commonPrefixes.clone(),
            encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
            partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
            restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN,
            classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS {
                elementLst: list![
                    quantity.clone(),
                    booleanStart.clone(),
                    fixed.clone(),
                    startOrigin.clone()
                ],
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
        })
    });

// BTH The Clock type
pub(crate) static clockType: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::CLASS {
        name: literal!("Clock"),
        prefixes: commonPrefixes.clone(),
        encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        restriction: openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_CLOCK,
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
    })
});

// The builtin variable time. See also variableIsBuiltin
thread_local! { static __timeVar_TLS: metamodelica::Ref<DAE::Var> = metamodelica::Ref::new(DAE::Var { name: literal!("time"), attributes: DAE::dummyAttrInput().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }); }
pub(crate) fn timeVar() -> metamodelica::Ref<DAE::Var> {
    __timeVar_TLS.with(|__t| __t.clone())
}

/* Optimica Extensions. Theses variables are considered builtin for Optimica: startTime, finalTime, objectiveIntegrand and objective */
/* Optimica Extensions. The builtin variable startTime. */
thread_local! { static __startTimeVar_TLS: metamodelica::Ref<DAE::Var> = metamodelica::Ref::new(DAE::Var { name: literal!("startTime"), attributes: DAE::dummyAttrInput().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }); }
pub(crate) fn startTimeVar() -> metamodelica::Ref<DAE::Var> {
    __startTimeVar_TLS.with(|__t| __t.clone())
}

/* Optimica Extensions. The builtin variable finalTime. */
thread_local! { static __finalTimeVar_TLS: metamodelica::Ref<DAE::Var> = metamodelica::Ref::new(DAE::Var { name: literal!("finalTime"), attributes: DAE::dummyAttrInput().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }); }
pub(crate) fn finalTimeVar() -> metamodelica::Ref<DAE::Var> {
    __finalTimeVar_TLS.with(|__t| __t.clone())
}

/* Optimica Extensions. The builtin variable objectiveIntegrand. */
thread_local! { static __objectiveIntegrandVar_TLS: metamodelica::Ref<DAE::Var> = metamodelica::Ref::new(DAE::Var { name: literal!("objectiveIntegrand"), attributes: DAE::dummyAttrInput().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }); }
pub(crate) fn objectiveIntegrandVar() -> metamodelica::Ref<DAE::Var> {
    __objectiveIntegrandVar_TLS.with(|__t| __t.clone())
}

/* Optimica Extensions. The builtin variable objective. */
thread_local! { static __objectiveVar_TLS: metamodelica::Ref<DAE::Var> = metamodelica::Ref::new(DAE::Var { name: literal!("objective"), attributes: DAE::dummyAttrInput().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }); }
pub(crate) fn objectiveVar() -> metamodelica::Ref<DAE::Var> {
    __objectiveVar_TLS.with(|__t| __t.clone())
}

thread_local! { static __argRealX_TLS: metamodelica::Ref<DAE::FuncArg> = metamodelica::Ref::new(DAE::FuncArg { name: literal!("x"), ty: DAE::T_REAL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }); }
pub(crate) fn argRealX() -> metamodelica::Ref<DAE::FuncArg> {
    __argRealX_TLS.with(|__t| __t.clone())
}

thread_local! { static __argRealY_TLS: metamodelica::Ref<DAE::FuncArg> = metamodelica::Ref::new(DAE::FuncArg { name: literal!("y"), ty: DAE::T_REAL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }); }
pub(crate) fn argRealY() -> metamodelica::Ref<DAE::FuncArg> {
    __argRealY_TLS.with(|__t| __t.clone())
}

thread_local! { static __argRealZ_TLS: metamodelica::Ref<DAE::FuncArg> = metamodelica::Ref::new(DAE::FuncArg { name: literal!("z"), ty: DAE::T_REAL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }); }
pub(crate) fn argRealZ() -> metamodelica::Ref<DAE::FuncArg> {
    __argRealZ_TLS.with(|__t| __t.clone())
}

thread_local! { static __argsRealX_TLS: metamodelica::List<metamodelica::Ref<DAE::FuncArg>> = list![argRealX().clone()]; }
pub(crate) fn argsRealX() -> metamodelica::List<metamodelica::Ref<DAE::FuncArg>> {
    __argsRealX_TLS.with(|__t| __t.clone())
}

thread_local! { static __argsRealXY_TLS: metamodelica::List<metamodelica::Ref<DAE::FuncArg>> = list![argRealX().clone(), argRealY().clone()]; }
pub(crate) fn argsRealXY() -> metamodelica::List<metamodelica::Ref<DAE::FuncArg>> {
    __argsRealXY_TLS.with(|__t| __t.clone())
}

thread_local! { static __argsRealXYZ_TLS: metamodelica::List<metamodelica::Ref<DAE::FuncArg>> = list![argRealX().clone(), argRealY().clone(), argRealZ().clone()]; }
pub(crate) fn argsRealXYZ() -> metamodelica::List<metamodelica::Ref<DAE::FuncArg>> {
    __argsRealXYZ_TLS.with(|__t| __t.clone())
}

pub(crate) static timeComp: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: literal!("time"),
        prefixes: SCode::defaultPrefixes.clone(),
        attributes: SCode::Attributes {
            arrayDims: metamodelica::nil(),
            connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
            parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
            variability: openmodelica_frontend_types::SCode::Variability::VAR,
            direction: openmodelica_ast::Absyn::Direction::INPUT,
            isField: openmodelica_ast::Absyn::IsField::NONFIELD,
        },
        typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Real") }),
            arrayDim: None,
        }),
        modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
        comment: SCode::noComment.clone(),
        condition: None,
        info: Absyn::dummyInfo.clone(),
    })
});

pub(crate) static startTimeComp: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("startTime"),
            prefixes: SCode::defaultPrefixes.clone(),
            attributes: SCode::Attributes {
                arrayDims: metamodelica::nil(),
                connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
                parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
                variability: openmodelica_frontend_types::SCode::Variability::VAR,
                direction: openmodelica_ast::Absyn::Direction::INPUT,
                isField: openmodelica_ast::Absyn::IsField::NONFIELD,
            },
            typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Real") }),
                arrayDim: None,
            }),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static finalTimeComp: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("finalTime"),
            prefixes: SCode::defaultPrefixes.clone(),
            attributes: SCode::Attributes {
                arrayDims: metamodelica::nil(),
                connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
                parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
                variability: openmodelica_frontend_types::SCode::Variability::VAR,
                direction: openmodelica_ast::Absyn::Direction::INPUT,
                isField: openmodelica_ast::Absyn::IsField::NONFIELD,
            },
            typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Real") }),
                arrayDim: None,
            }),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static objectiveIntegrandComp: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("objectiveIntegrand"),
            prefixes: SCode::defaultPrefixes.clone(),
            attributes: SCode::Attributes {
                arrayDims: metamodelica::nil(),
                connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
                parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
                variability: openmodelica_frontend_types::SCode::Variability::VAR,
                direction: openmodelica_ast::Absyn::Direction::INPUT,
                isField: openmodelica_ast::Absyn::IsField::NONFIELD,
            },
            typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Real") }),
                arrayDim: None,
            }),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static objectiveVarComp: std::sync::LazyLock<metamodelica::Ref<SCode::Element>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: literal!("objectiveVar"),
            prefixes: SCode::defaultPrefixes.clone(),
            attributes: SCode::Attributes {
                arrayDims: metamodelica::nil(),
                connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
                parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
                variability: openmodelica_frontend_types::SCode::Variability::VAR,
                direction: openmodelica_ast::Absyn::Direction::INPUT,
                isField: openmodelica_ast::Absyn::IsField::NONFIELD,
            },
            typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Real") }),
                arrayDim: None,
            }),
            modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
            comment: SCode::noComment.clone(),
            condition: None,
            info: Absyn::dummyInfo.clone(),
        })
    });

pub(crate) static basicTypes: std::sync::LazyLock<metamodelica::List<metamodelica::Ref<SCode::Element>>> =
    std::sync::LazyLock::new(|| {
        list![
            clockType.clone(),
            rlType.clone(),
            intType.clone(),
            strType.clone(),
            boolType.clone(),
            enumType.clone(),
            ExternalObjectType.clone(),
            realType.clone(),
            integerType.clone(),
            stringType.clone(),
            booleanType.clone(),
            uncertaintyType.clone()
        ]
    });

pub(crate) static basicTypesNF: std::sync::LazyLock<metamodelica::List<metamodelica::Ref<SCode::Element>>> =
    std::sync::LazyLock::new(|| {
        list![
            rlType.clone(),
            intType.clone(),
            strType.clone(),
            boolType.clone(),
            enumType.clone(),
            realType.clone(),
            integerType.clone(),
            stringType.clone(),
            booleanType.clone()
        ]
    });

pub(crate) fn getBasicTypes() -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut tys: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    tys = if (Flags::isSet(Flags::SCODE_INST.clone())?) {
        basicTypesNF.clone()
    } else {
        basicTypes.clone()
    };
    Ok(tys)
}

pub(crate) fn variableIsBuiltin(mut cref: &metamodelica::Ref<DAE::ComponentRef>, mut useOptimica: bool) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((&**cref, useOptimica)) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, _) => true,
        (_, false) => false,
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "startTime", .. }, true) => true,
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "finalTime", .. }, true) => true,
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "objective", .. }, true) => true,
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "objectiveIntegrand", .. }, true) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isDer(mut inPath: &metamodelica::Ref<Absyn::Path>) -> Result<()> {
    let () = (::match_deref::match_deref! { match inPath {
        Deref @ Absyn::Path::IDENT { name: Deref @ "der" } => {
            ()
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { path } => {
            isDer(path)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn mergePrograms(mut program1: Absyn::Program, mut program2: &Absyn::Program) -> Absyn::Program {
    let mut outProgram: Absyn::Program = program1.clone();
    outProgram.classes = listAppend(program1.classes.clone(), program2.classes.clone());
    outProgram
}

pub fn getInitialFunctions() -> Result<(Absyn::Program, metamodelica::List<metamodelica::Ref<SCode::Element>>)> {
    let mut initialProgram: Absyn::Program;
    let mut initialSCodeProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut fileModelicaNF: ArcStr;
    let mut fileModelicaCF: ArcStr;
    let mut fileMetaModelica: ArcStr;
    let mut fileParModelica: ArcStr;
    let mut filePDEModelica: ArcStr;
    let mut assocLst: metamodelica::List<(
        (i32, bool),
        (Absyn::Program, metamodelica::List<metamodelica::Ref<SCode::Element>>),
    )> = metamodelica::nil();
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut pNF: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut pCF: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut pMM: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut spNF: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut spCF: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    fileModelicaNF = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
        __mm_s.push_str(&*literal!("/lib/omc/NFModelicaBuiltin.mo"));
        ArcStr::from(__mm_s)
    };
    fileModelicaCF = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
        __mm_s.push_str(&*literal!("/lib/omc/ModelicaBuiltin.mo"));
        ArcStr::from(__mm_s)
    };
    fileMetaModelica = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
        __mm_s.push_str(&*literal!("/lib/omc/MetaModelicaBuiltin.mo"));
        ArcStr::from(__mm_s)
    };
    fileParModelica = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
        __mm_s.push_str(&*literal!("/lib/omc/ParModelicaBuiltin.mo"));
        ArcStr::from(__mm_s)
    };
    filePDEModelica = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
        __mm_s.push_str(&*literal!("/lib/omc/PDEModelicaBuiltin.mo"));
        ArcStr::from(__mm_s)
    };
    (initialProgram, initialSCodeProgram) = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            if '__try0: {
                crate::Globals::builtinIndex.with(|__root| __root.borrow().clone());
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            {
                let __v = metamodelica::nil();
                crate::Globals::builtinIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let mut assocLst: metamodelica::List<(
                (i32, bool),
                (Absyn::Program, metamodelica::List<metamodelica::Ref<SCode::Element>>),
            )> = assocLst.clone();
            let mut p: Absyn::Program = p.clone();
            let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>> = sp.clone();
            assocLst = crate::Globals::builtinIndex.with(|__root| __root.borrow().clone());
            (p, sp) = Util::assoc(
                Util::makeTuple(
                    Flags::getConfigEnum(Flags::GRAMMAR.clone())?,
                    Flags::isSet(Flags::SCODE_INST.clone())?,
                ),
                assocLst.clone(),
            )?;
            Ok(((p.clone(), sp.clone()), assocLst.clone(), p.clone(), sp.clone()))
        })() {
            assocLst = __wb0;
            p = __wb1;
            sp = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7)) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let mut assocLst: metamodelica::List<(
                (i32, bool),
                (Absyn::Program, metamodelica::List<metamodelica::Ref<SCode::Element>>),
            )> = assocLst.clone();
            let mut p: Absyn::Program = p.clone();
            let mut pCF: Absyn::Program = pCF.clone();
            let mut pMM: Absyn::Program = pMM.clone();
            let mut pNF: Absyn::Program = pNF.clone();
            let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>> = sp.clone();
            let mut spCF: metamodelica::List<metamodelica::Ref<SCode::Element>> = spCF.clone();
            let mut spNF: metamodelica::List<metamodelica::Ref<SCode::Element>> = spNF.clone();
            let true = (intEq(
                Flags::getConfigEnum(Flags::GRAMMAR.clone())?,
                Flags::METAMODELICA.clone(),
            )) else {
                return Err("pattern mismatch");
            };
            Error::assertionOrAddSourceMessage(
                System::regularFileExists(fileModelicaNF.clone()),
                &(Error::FILE_NOT_FOUND_ERROR.clone()),
                list![fileModelicaNF.clone()],
                &(Absyn::dummyInfo.clone()),
            )?;
            Error::assertionOrAddSourceMessage(
                System::regularFileExists(fileModelicaCF.clone()),
                &(Error::FILE_NOT_FOUND_ERROR.clone()),
                list![fileModelicaCF.clone()],
                &(Absyn::dummyInfo.clone()),
            )?;
            Error::assertionOrAddSourceMessage(
                System::regularFileExists(fileMetaModelica.clone()),
                &(Error::FILE_NOT_FOUND_ERROR.clone()),
                list![fileMetaModelica.clone()],
                &(Absyn::dummyInfo.clone()),
            )?;
            pNF = Parser::parse(
                fileModelicaNF.clone(),
                literal!("UTF-8"),
                literal!(""),
                None,
                Flags::METAMODELICA.clone(),
                Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
                Flags::getConfigBool(Flags::STRICT.clone())?,
            )?;
            pCF = Parser::parse(
                fileModelicaCF.clone(),
                literal!("UTF-8"),
                literal!(""),
                None,
                Flags::METAMODELICA.clone(),
                Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
                Flags::getConfigBool(Flags::STRICT.clone())?,
            )?;
            pMM = Parser::parse(
                fileMetaModelica.clone(),
                literal!("UTF-8"),
                literal!(""),
                None,
                Flags::METAMODELICA.clone(),
                Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
                Flags::getConfigBool(Flags::STRICT.clone())?,
            )?;
            pNF = mergePrograms(pNF.clone(), &pMM);
            pCF = mergePrograms(pCF.clone(), &pMM);
            pNF = MetaUtil::createMetaClassesInProgram(pNF.clone())?;
            pCF = MetaUtil::createMetaClassesInProgram(pCF.clone())?;
            spNF = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                for mut c in (pNF.classes.clone()).into_iter().cloned() {
                    let __x = AbsynToSCode::translateClass(c.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            spCF = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                for mut c in (pCF.classes.clone()).into_iter().cloned() {
                    let __x = AbsynToSCode::translateClass(c.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            assocLst = crate::Globals::builtinIndex.with(|__root| __root.borrow().clone());
            {
                let __v = metamodelica::cons(
                    ((Flags::METAMODELICA.clone(), true), (pNF.clone(), spNF.clone())),
                    metamodelica::cons(
                        ((Flags::METAMODELICA.clone(), false), (pCF.clone(), spCF.clone())),
                        assocLst.clone(),
                    ),
                );
                crate::Globals::builtinIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            (p, sp) = if (Flags::isSet(Flags::SCODE_INST.clone())?) {
                (pNF.clone(), spNF.clone())
            } else {
                (pCF.clone(), spCF.clone())
            };
            Ok((
                (p.clone(), sp.clone()),
                assocLst.clone(),
                p.clone(),
                pCF.clone(),
                pMM.clone(),
                pNF.clone(),
                sp.clone(),
                spCF.clone(),
                spNF.clone(),
            ))
        })() {
            assocLst = __wb0;
            p = __wb1;
            pCF = __wb2;
            pMM = __wb3;
            pNF = __wb4;
            sp = __wb5;
            spCF = __wb6;
            spNF = __wb7;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7)) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let mut assocLst: metamodelica::List<(
                (i32, bool),
                (Absyn::Program, metamodelica::List<metamodelica::Ref<SCode::Element>>),
            )> = assocLst.clone();
            let mut p: Absyn::Program = p.clone();
            let mut pCF: Absyn::Program = pCF.clone();
            let mut pMM: Absyn::Program = pMM.clone();
            let mut pNF: Absyn::Program = pNF.clone();
            let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>> = sp.clone();
            let mut spCF: metamodelica::List<metamodelica::Ref<SCode::Element>> = spCF.clone();
            let mut spNF: metamodelica::List<metamodelica::Ref<SCode::Element>> = spNF.clone();
            let true = (intEq(
                Flags::getConfigEnum(Flags::GRAMMAR.clone())?,
                Flags::PARMODELICA.clone(),
            )) else {
                return Err("pattern mismatch");
            };
            Error::assertionOrAddSourceMessage(
                System::regularFileExists(fileModelicaNF.clone()),
                &(Error::FILE_NOT_FOUND_ERROR.clone()),
                list![fileModelicaNF.clone()],
                &(Absyn::dummyInfo.clone()),
            )?;
            Error::assertionOrAddSourceMessage(
                System::regularFileExists(fileModelicaCF.clone()),
                &(Error::FILE_NOT_FOUND_ERROR.clone()),
                list![fileModelicaCF.clone()],
                &(Absyn::dummyInfo.clone()),
            )?;
            Error::assertionOrAddSourceMessage(
                System::regularFileExists(fileMetaModelica.clone()),
                &(Error::FILE_NOT_FOUND_ERROR.clone()),
                list![fileMetaModelica.clone()],
                &(Absyn::dummyInfo.clone()),
            )?;
            pNF = Parser::parse(
                fileModelicaNF.clone(),
                literal!("UTF-8"),
                literal!(""),
                None,
                Flags::METAMODELICA.clone(),
                Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
                Flags::getConfigBool(Flags::STRICT.clone())?,
            )?;
            pCF = Parser::parse(
                fileModelicaCF.clone(),
                literal!("UTF-8"),
                literal!(""),
                None,
                Flags::METAMODELICA.clone(),
                Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
                Flags::getConfigBool(Flags::STRICT.clone())?,
            )?;
            pMM = Parser::parse(
                fileParModelica.clone(),
                literal!("UTF-8"),
                literal!(""),
                None,
                Flags::METAMODELICA.clone(),
                Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
                Flags::getConfigBool(Flags::STRICT.clone())?,
            )?;
            pNF = mergePrograms(pNF.clone(), &pMM);
            pCF = mergePrograms(pCF.clone(), &pMM);
            spNF = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                for mut c in (pNF.classes.clone()).into_iter().cloned() {
                    let __x = AbsynToSCode::translateClass(c.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            spCF = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                for mut c in (pCF.classes.clone()).into_iter().cloned() {
                    let __x = AbsynToSCode::translateClass(c.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            assocLst = crate::Globals::builtinIndex.with(|__root| __root.borrow().clone());
            {
                let __v = metamodelica::cons(
                    ((Flags::PARMODELICA.clone(), true), (pNF.clone(), spNF.clone())),
                    metamodelica::cons(
                        ((Flags::PARMODELICA.clone(), false), (pCF.clone(), spCF.clone())),
                        assocLst.clone(),
                    ),
                );
                crate::Globals::builtinIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            (p, sp) = if (Flags::isSet(Flags::SCODE_INST.clone())?) {
                (pNF.clone(), spNF.clone())
            } else {
                (pCF.clone(), spCF.clone())
            };
            Ok((
                (p.clone(), sp.clone()),
                assocLst.clone(),
                p.clone(),
                pCF.clone(),
                pMM.clone(),
                pNF.clone(),
                sp.clone(),
                spCF.clone(),
                spNF.clone(),
            ))
        })() {
            assocLst = __wb0;
            p = __wb1;
            pCF = __wb2;
            pMM = __wb3;
            pNF = __wb4;
            sp = __wb5;
            spCF = __wb6;
            spNF = __wb7;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6)) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let mut assocLst: metamodelica::List<(
                (i32, bool),
                (Absyn::Program, metamodelica::List<metamodelica::Ref<SCode::Element>>),
            )> = assocLst.clone();
            let mut p: Absyn::Program = p.clone();
            let mut pCF: Absyn::Program = pCF.clone();
            let mut pNF: Absyn::Program = pNF.clone();
            let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>> = sp.clone();
            let mut spCF: metamodelica::List<metamodelica::Ref<SCode::Element>> = spCF.clone();
            let mut spNF: metamodelica::List<metamodelica::Ref<SCode::Element>> = spNF.clone();
            let true = (intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::MODELICA.clone())
                || intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::OPTIMICA.clone()))
            else {
                return Err("pattern mismatch");
            };
            Error::assertionOrAddSourceMessage(
                System::regularFileExists(fileModelicaNF.clone()),
                &(Error::FILE_NOT_FOUND_ERROR.clone()),
                list![fileModelicaNF.clone()],
                &(Absyn::dummyInfo.clone()),
            )?;
            Error::assertionOrAddSourceMessage(
                System::regularFileExists(fileModelicaCF.clone()),
                &(Error::FILE_NOT_FOUND_ERROR.clone()),
                list![fileModelicaCF.clone()],
                &(Absyn::dummyInfo.clone()),
            )?;
            pNF = Parser::parse(
                fileModelicaNF.clone(),
                literal!("UTF-8"),
                literal!(""),
                None,
                Flags::METAMODELICA.clone(),
                Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
                Flags::getConfigBool(Flags::STRICT.clone())?,
            )?;
            pCF = Parser::parse(
                fileModelicaCF.clone(),
                literal!("UTF-8"),
                literal!(""),
                None,
                Flags::METAMODELICA.clone(),
                Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
                Flags::getConfigBool(Flags::STRICT.clone())?,
            )?;
            spNF = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                for mut c in (pNF.classes.clone()).into_iter().cloned() {
                    let __x = AbsynToSCode::translateClass(c.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            spCF = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                for mut c in (pCF.classes.clone()).into_iter().cloned() {
                    let __x = AbsynToSCode::translateClass(c.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            assocLst = crate::Globals::builtinIndex.with(|__root| __root.borrow().clone());
            {
                let __v = metamodelica::cons(
                    ((Flags::MODELICA.clone(), true), (pNF.clone(), spNF.clone())),
                    metamodelica::cons(
                        ((Flags::MODELICA.clone(), false), (pCF.clone(), spCF.clone())),
                        assocLst.clone(),
                    ),
                );
                crate::Globals::builtinIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            (p, sp) = if (Flags::isSet(Flags::SCODE_INST.clone())?) {
                (pNF.clone(), spNF.clone())
            } else {
                (pCF.clone(), spCF.clone())
            };
            Ok((
                (p.clone(), sp.clone()),
                assocLst.clone(),
                p.clone(),
                pCF.clone(),
                pNF.clone(),
                sp.clone(),
                spCF.clone(),
                spNF.clone(),
            ))
        })() {
            assocLst = __wb0;
            p = __wb1;
            pCF = __wb2;
            pNF = __wb3;
            sp = __wb4;
            spCF = __wb5;
            spNF = __wb6;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7)) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let mut assocLst: metamodelica::List<(
                (i32, bool),
                (Absyn::Program, metamodelica::List<metamodelica::Ref<SCode::Element>>),
            )> = assocLst.clone();
            let mut p: Absyn::Program = p.clone();
            let mut pCF: Absyn::Program = pCF.clone();
            let mut pMM: Absyn::Program = pMM.clone();
            let mut pNF: Absyn::Program = pNF.clone();
            let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>> = sp.clone();
            let mut spCF: metamodelica::List<metamodelica::Ref<SCode::Element>> = spCF.clone();
            let mut spNF: metamodelica::List<metamodelica::Ref<SCode::Element>> = spNF.clone();
            let true = (intEq(
                Flags::getConfigEnum(Flags::GRAMMAR.clone())?,
                Flags::PDEMODELICA.clone(),
            )) else {
                return Err("pattern mismatch");
            };
            Error::assertionOrAddSourceMessage(
                System::regularFileExists(fileModelicaNF.clone()),
                &(Error::FILE_NOT_FOUND_ERROR.clone()),
                list![fileModelicaNF.clone()],
                &(Absyn::dummyInfo.clone()),
            )?;
            Error::assertionOrAddSourceMessage(
                System::regularFileExists(fileModelicaCF.clone()),
                &(Error::FILE_NOT_FOUND_ERROR.clone()),
                list![fileModelicaCF.clone()],
                &(Absyn::dummyInfo.clone()),
            )?;
            Error::assertionOrAddSourceMessage(
                System::regularFileExists(filePDEModelica.clone()),
                &(Error::FILE_NOT_FOUND_ERROR.clone()),
                list![filePDEModelica.clone()],
                &(Absyn::dummyInfo.clone()),
            )?;
            pNF = Parser::parse(
                fileModelicaNF.clone(),
                literal!("UTF-8"),
                literal!(""),
                None,
                Flags::METAMODELICA.clone(),
                Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
                Flags::getConfigBool(Flags::STRICT.clone())?,
            )?;
            pCF = Parser::parse(
                fileModelicaCF.clone(),
                literal!("UTF-8"),
                literal!(""),
                None,
                Flags::METAMODELICA.clone(),
                Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
                Flags::getConfigBool(Flags::STRICT.clone())?,
            )?;
            pMM = Parser::parse(
                filePDEModelica.clone(),
                literal!("UTF-8"),
                literal!(""),
                None,
                Flags::METAMODELICA.clone(),
                Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
                Flags::getConfigBool(Flags::STRICT.clone())?,
            )?;
            pNF = mergePrograms(pNF.clone(), &pMM);
            pCF = mergePrograms(pCF.clone(), &pMM);
            spNF = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                for mut c in (pNF.classes.clone()).into_iter().cloned() {
                    let __x = AbsynToSCode::translateClass(c.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            spCF = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                for mut c in (pCF.classes.clone()).into_iter().cloned() {
                    let __x = AbsynToSCode::translateClass(c.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            assocLst = crate::Globals::builtinIndex.with(|__root| __root.borrow().clone());
            {
                let __v = metamodelica::cons(
                    ((Flags::PDEMODELICA.clone(), true), (pNF.clone(), spNF.clone())),
                    metamodelica::cons(
                        ((Flags::PDEMODELICA.clone(), false), (pCF.clone(), spCF.clone())),
                        assocLst.clone(),
                    ),
                );
                crate::Globals::builtinIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            (p, sp) = if (Flags::isSet(Flags::SCODE_INST.clone())?) {
                (pNF.clone(), spNF.clone())
            } else {
                (pCF.clone(), spCF.clone())
            };
            Ok((
                (p.clone(), sp.clone()),
                assocLst.clone(),
                p.clone(),
                pCF.clone(),
                pMM.clone(),
                pNF.clone(),
                sp.clone(),
                spCF.clone(),
                spNF.clone(),
            ))
        })() {
            assocLst = __wb0;
            p = __wb1;
            pCF = __wb2;
            pMM = __wb3;
            pNF = __wb4;
            sp = __wb5;
            spCF = __wb6;
            spNF = __wb7;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addInternalError(
                literal!("FBuiltin.getInitialFunctions failed."),
                metamodelica::sourceInfo!("FFrontEnd/FBuiltin.mo"),
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((initialProgram, initialSCodeProgram))
}

pub(crate) fn initialGraph(mut inCache: FCore::Cache) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut graph: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut cache: FCore::Cache;
    (outCache, graph) = 'mc: {
        let __mc_input = inCache;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let mut cache = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut graph: FCore::Graph = graph.clone();
            graph = FCore::getCachedInitialGraph(&cache)?;
            Ok(((cache.clone(), graph.clone()), graph.clone()))
        })() {
            graph = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let mut cache = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut graph: FCore::Graph = graph.clone();
            graph = getSetInitialGraph(None)?;
            Ok(((cache.clone(), graph.clone()), graph.clone()))
        })() {
            graph = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let mut cache = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut initialProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut graph: FCore::Graph = graph.clone();
            graph = FGraph::new(literal!("graph"), FCore::dummyTopModel.clone());
            graph = FGraphBuild::mkProgramGraph(
                &(basicTypes.clone()),
                openmodelica_frontend_dump::FCore::Kind::BASIC_TYPE,
                graph.clone(),
            )?;
            graph = initialGraphOptimica(graph.clone(), &FGraphBuild::mkCompNode)?;
            graph = initialGraphMetaModelica(graph.clone(), &FGraphBuild::mkTypeNode)?;
            graph = initialGraphModelica(graph.clone(), &FGraphBuild::mkTypeNode, &FGraphBuild::mkCompNode)?;
            (_, initialProgram) = getInitialFunctions()?;
            graph = FGraphBuild::mkProgramGraph(
                &initialProgram,
                openmodelica_frontend_dump::FCore::Kind::BUILTIN,
                graph.clone(),
            )?;
            cache = FCore::setCachedInitialGraph(cache.clone(), graph.clone());
            getSetInitialGraph(Some(graph.clone()))?;
            Ok(((cache.clone(), graph.clone()), graph.clone()))
        })() {
            graph = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, graph))
}

fn getSetInitialGraph(mut inEnvOpt: Option<FCore::Graph>) -> Result<FCore::Graph> {
    let mut initialEnv: FCore::Graph;
    initialEnv = 'mc: {
        let __mc_input = inEnvOpt;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if '__try0: {
                crate::Globals::builtinGraphIndex.with(|__root| __root.borrow().clone());
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            {
                let __v = metamodelica::nil();
                crate::Globals::builtinGraphIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let None = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut assocLst: metamodelica::List<(i32, FCore::Graph)>;
            assocLst = crate::Globals::builtinGraphIndex.with(|__root| __root.borrow().clone());
            Ok(Util::assoc(
                Flags::getConfigEnum(Flags::GRAMMAR.clone())?,
                assocLst.clone(),
            )?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let Some(mut graph) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut assocLst: metamodelica::List<(i32, FCore::Graph)>;
            let true = (intEq(
                Flags::getConfigEnum(Flags::GRAMMAR.clone())?,
                Flags::METAMODELICA.clone(),
            )) else {
                return Err("pattern mismatch");
            };
            assocLst = crate::Globals::builtinGraphIndex.with(|__root| __root.borrow().clone());
            {
                let __v = metamodelica::cons((Flags::METAMODELICA.clone(), graph.clone()), assocLst.clone());
                crate::Globals::builtinGraphIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            Ok(graph.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let Some(mut graph) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut assocLst: metamodelica::List<(i32, FCore::Graph)>;
            let true = (intEq(
                Flags::getConfigEnum(Flags::GRAMMAR.clone())?,
                Flags::PARMODELICA.clone(),
            )) else {
                return Err("pattern mismatch");
            };
            assocLst = crate::Globals::builtinGraphIndex.with(|__root| __root.borrow().clone());
            {
                let __v = metamodelica::cons((Flags::PARMODELICA.clone(), graph.clone()), assocLst.clone());
                crate::Globals::builtinGraphIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            Ok(graph.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let Some(mut graph) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut assocLst: metamodelica::List<(i32, FCore::Graph)>;
            let true = (intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::MODELICA.clone())
                || intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::OPTIMICA.clone()))
            else {
                return Err("pattern mismatch");
            };
            assocLst = crate::Globals::builtinGraphIndex.with(|__root| __root.borrow().clone());
            {
                let __v = metamodelica::cons((Flags::MODELICA.clone(), graph.clone()), assocLst.clone());
                crate::Globals::builtinGraphIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            Ok(graph.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(initialEnv)
}

pub type MakeTypeNode = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::List<metamodelica::Ref<DAE::Type>>,
            Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
            ArcStr,
            FCore::Graph,
        ) -> Result<FCore::Graph>
        + 'static,
>;

pub type MakeCompNode = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<SCode::Element>,
            Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
            FCore::Kind,
            FCore::Graph,
        ) -> Result<FCore::Graph>
        + 'static,
>;

pub(crate) fn initialGraphModelica(
    mut graph: FCore::Graph,
    mut mkTypeNode: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
        ArcStr,
        FCore::Graph,
    ) -> Result<FCore::Graph>,
    mut mkCompNode: &dyn ::std::ops::Fn(
        metamodelica::Ref<SCode::Element>,
        Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
        FCore::Kind,
        FCore::Graph,
    ) -> Result<FCore::Graph>,
) -> Result<FCore::Graph> {
    let mut graph: FCore::Graph = graph;
    let enumeration2int: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_FUNCTION {
        funcArg: list![metamodelica::Ref::new(DAE::FuncArg {
            name: literal!("x"),
            ty: metamodelica::Ref::new(DAE::Type::T_ENUMERATION {
                index: None,
                path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
                names: metamodelica::nil(),
                literalVarLst: metamodelica::nil(),
                attributeLst: metamodelica::nil()
            }),
            r#const: openmodelica_frontend_types::DAE::Const::C_VAR,
            par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
            defaultBinding: None
        })],
        funcResultType: DAE::T_INTEGER_DEFAULT().clone(),
        functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(),
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("Integer"),
        }),
    });
    graph = mkCompNode(
        timeComp.clone(),
        FGraph::top(&graph)?,
        openmodelica_frontend_dump::FCore::Kind::BUILTIN,
        graph,
    )?;
    graph = FGraph::updateComp(
        graph,
        timeVar().clone(),
        &(openmodelica_frontend_dump::FCore::Status::VAR_UNTYPED),
        &(FGraph::empty()),
    );
    graph = mkTypeNode(
        list![
            metamodelica::Ref::new(DAE::Type::T_FUNCTION {
                funcArg: list![metamodelica::Ref::new(DAE::FuncArg {
                    name: literal!("x"),
                    ty: metamodelica::Ref::new(DAE::Type::T_ANYTYPE {
                        anyClassType: Some(ClassInf::State::CONNECTOR {
                            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                                name: literal!("$dummy$")
                            }),
                            isExpandable: false
                        })
                    }),
                    r#const: openmodelica_frontend_types::DAE::Const::C_VAR,
                    par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                    defaultBinding: None
                })],
                funcResultType: DAE::T_INTEGER_DEFAULT().clone(),
                functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(),
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("cardinality")
                })
            }),
            metamodelica::Ref::new(DAE::Type::T_FUNCTION {
                funcArg: list![metamodelica::Ref::new(DAE::FuncArg {
                    name: literal!("x"),
                    ty: metamodelica::Ref::new(DAE::Type::T_ANYTYPE {
                        anyClassType: Some(ClassInf::State::CONNECTOR {
                            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                                name: literal!("$dummy$")
                            }),
                            isExpandable: true
                        })
                    }),
                    r#const: openmodelica_frontend_types::DAE::Const::C_VAR,
                    par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                    defaultBinding: None
                })],
                funcResultType: DAE::T_INTEGER_DEFAULT().clone(),
                functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(),
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("cardinality")
                })
            })
        ],
        FGraph::top(&graph)?,
        literal!("cardinality"),
        graph,
    )?;
    graph = mkTypeNode(
        list![enumeration2int.clone()],
        FGraph::top(&graph)?,
        literal!("Integer"),
        graph,
    )?;
    graph = mkTypeNode(
        list![enumeration2int],
        FGraph::top(&graph)?,
        literal!("EnumToInteger"),
        graph,
    )?;
    graph = mkTypeNode(
        list![metamodelica::Ref::new(DAE::Type::T_FUNCTION {
            funcArg: argsRealX().clone(),
            funcResultType: DAE::T_REAL_DEFAULT().clone(),
            functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(),
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("noEvent")
            })
        })],
        FGraph::top(&graph)?,
        literal!("noEvent"),
        graph,
    )?;
    graph = mkTypeNode(
        list![metamodelica::Ref::new(DAE::Type::T_FUNCTION {
            funcArg: argsRealX().clone(),
            funcResultType: DAE::T_REAL_DEFAULT().clone(),
            functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(),
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("actualStream")
            })
        })],
        FGraph::top(&graph)?,
        literal!("actualStream"),
        graph,
    )?;
    graph = mkTypeNode(
        list![metamodelica::Ref::new(DAE::Type::T_FUNCTION {
            funcArg: argsRealX().clone(),
            funcResultType: DAE::T_REAL_DEFAULT().clone(),
            functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(),
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("inStream")
            })
        })],
        FGraph::top(&graph)?,
        literal!("inStream"),
        graph,
    )?;
    Ok(graph)
}

pub(crate) fn initialGraphMetaModelica(
    mut graph: FCore::Graph,
    mut mkTypeNode: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::Ref<DAE::Type>>,
        Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
        ArcStr,
        FCore::Graph,
    ) -> Result<FCore::Graph>,
) -> Result<FCore::Graph> {
    let mut graph: FCore::Graph = graph;
    if !(Config::acceptMetaModelicaGrammar()?) {
        return Ok(graph);
    }
    graph = mkTypeNode(
        list![metamodelica::Ref::new(DAE::Type::T_FUNCTION {
            funcArg: list![metamodelica::Ref::new(DAE::FuncArg {
                name: literal!("index"),
                ty: DAE::T_INTEGER_DEFAULT().clone(),
                r#const: openmodelica_frontend_types::DAE::Const::C_VAR,
                par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                defaultBinding: None
            })],
            funcResultType: DAE::T_METABOXED_DEFAULT().clone(),
            functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN.clone(),
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("getGlobalRoot")
            })
        })],
        FGraph::top(&graph)?,
        literal!("getGlobalRoot"),
        graph,
    )?;
    Ok(graph)
}

pub(crate) fn initialGraphOptimica(
    mut graph: FCore::Graph,
    mut mkCompNode: &dyn ::std::ops::Fn(
        metamodelica::Ref<SCode::Element>,
        Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
        FCore::Kind,
        FCore::Graph,
    ) -> Result<FCore::Graph>,
) -> Result<FCore::Graph> {
    let mut graph: FCore::Graph = graph;
    if !(Config::acceptOptimicaGrammar()?) {
        return Ok(graph);
    }
    graph = mkCompNode(
        objectiveVarComp.clone(),
        FGraph::top(&graph)?,
        openmodelica_frontend_dump::FCore::Kind::BUILTIN,
        graph,
    )?;
    graph = FGraph::updateComp(
        graph,
        objectiveVar().clone(),
        &(openmodelica_frontend_dump::FCore::Status::VAR_UNTYPED),
        &(FGraph::empty()),
    );
    graph = mkCompNode(
        objectiveIntegrandComp.clone(),
        FGraph::top(&graph)?,
        openmodelica_frontend_dump::FCore::Kind::BUILTIN,
        graph,
    )?;
    graph = FGraph::updateComp(
        graph,
        objectiveIntegrandVar().clone(),
        &(openmodelica_frontend_dump::FCore::Status::VAR_UNTYPED),
        &(FGraph::empty()),
    );
    graph = mkCompNode(
        startTimeComp.clone(),
        FGraph::top(&graph)?,
        openmodelica_frontend_dump::FCore::Kind::BUILTIN,
        graph,
    )?;
    graph = FGraph::updateComp(
        graph,
        startTimeVar().clone(),
        &(openmodelica_frontend_dump::FCore::Status::VAR_UNTYPED),
        &(FGraph::empty()),
    );
    graph = mkCompNode(
        finalTimeComp.clone(),
        FGraph::top(&graph)?,
        openmodelica_frontend_dump::FCore::Kind::BUILTIN,
        graph,
    )?;
    graph = FGraph::updateComp(
        graph,
        finalTimeVar().clone(),
        &(openmodelica_frontend_dump::FCore::Status::VAR_UNTYPED),
        &(FGraph::empty()),
    );
    Ok(graph)
}

pub fn getElementWithPathCheckBuiltin(
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inPath: &metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outElement: metamodelica::Ref<SCode::Element>;
    outElement = 'mc: {
        let __mc_input = &**inPath;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(SCodeUtil::getElementWithPath(inProgram.clone(), inPath)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    (_, sp) = getInitialFunctions()?;
                    Ok(SCodeUtil::getElementWithPath(sp.clone(), inPath)?)
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
