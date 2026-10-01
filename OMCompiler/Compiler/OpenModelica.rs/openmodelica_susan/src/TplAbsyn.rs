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

use crate::TplCodegen;
use openmodelica_tpl::Tpl;
use openmodelica_util::AvlSetString;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

/* Input AST */
pub type Ident = ArcStr;

pub type TypedIdents = metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>;

pub type EscOption = (ArcStr, Option<(metamodelica::Ref<ExpressionBase>, SourceInfo)>);

pub type StringToken = metamodelica::Ref<Tpl::StringToken>;

pub type Tokens = metamodelica::List<metamodelica::Ref<Tpl::StringToken>>;

pub(crate) static dummySourceInfo: SourceInfo = SourceInfo {
    fileName: literal!("NoFileName.xxx"),
    isReadOnly: false,
    lineNumberStart: 0,
    columnNumberStart: 0,
    lineNumberEnd: 0,
    columnNumberEnd: 0,
    lastModification: metamodelica::OrderedFloat(0.0_f64),
};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum PathIdent {
    IDENT {
        ident: Ident,
    },
    PATH_IDENT {
        ident: Ident,
        path: metamodelica::Ref<PathIdent>,
    },
}
impl metamodelica::gc::MMTrace for PathIdent {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            PathIdent::IDENT { ident } => {
                metamodelica::gc::MMTrace::mm_accept(ident, __mmv)?;
                Ok(())
            }
            PathIdent::PATH_IDENT { ident, path } => {
                metamodelica::gc::MMTrace::mm_accept(ident, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for PathIdent {
    fn default() -> Self {
        Self::IDENT {
            ident: Default::default(),
        }
    }
}
pub use self::PathIdent::{IDENT, PATH_IDENT};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum TypeSignature {
    LIST_TYPE {
        ofType: metamodelica::Ref<TypeSignature>,
    },
    ARRAY_TYPE {
        ofType: metamodelica::Ref<TypeSignature>,
    },
    OPTION_TYPE {
        ofType: metamodelica::Ref<TypeSignature>,
    },
    TUPLE_TYPE {
        ofTypes: metamodelica::List<metamodelica::Ref<TypeSignature>>,
    },
    /// key/path to a TypeInfo list from an AST definition
    NAMED_TYPE {
        name: metamodelica::Ref<PathIdent>,
    },
    STRING_TYPE,
    TEXT_TYPE,
    /// Used only for internal string constants.
    STRING_TOKEN_TYPE,
    INTEGER_TYPE,
    REAL_TYPE,
    BOOLEAN_TYPE,
    /// Errorneous resolving type. Only used during elaboration phase.
    UNRESOLVED_TYPE {
        reason: ArcStr,
    },
}
impl metamodelica::gc::MMTrace for TypeSignature {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            TypeSignature::LIST_TYPE { ofType } => {
                metamodelica::gc::MMTrace::mm_accept(ofType, __mmv)?;
                Ok(())
            }
            TypeSignature::ARRAY_TYPE { ofType } => {
                metamodelica::gc::MMTrace::mm_accept(ofType, __mmv)?;
                Ok(())
            }
            TypeSignature::OPTION_TYPE { ofType } => {
                metamodelica::gc::MMTrace::mm_accept(ofType, __mmv)?;
                Ok(())
            }
            TypeSignature::TUPLE_TYPE { ofTypes } => {
                metamodelica::gc::MMTrace::mm_accept(ofTypes, __mmv)?;
                Ok(())
            }
            TypeSignature::NAMED_TYPE { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
            TypeSignature::STRING_TYPE => Ok(()),
            TypeSignature::TEXT_TYPE => Ok(()),
            TypeSignature::STRING_TOKEN_TYPE => Ok(()),
            TypeSignature::INTEGER_TYPE => Ok(()),
            TypeSignature::REAL_TYPE => Ok(()),
            TypeSignature::BOOLEAN_TYPE => Ok(()),
            TypeSignature::UNRESOLVED_TYPE { reason } => {
                metamodelica::gc::MMTrace::mm_accept(reason, __mmv)?;
                Ok(())
            }
        }
    }
}
impl TypeSignature {
    pub fn interned_STRING_TYPE() -> metamodelica::Ref<TypeSignature> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<TypeSignature>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(TypeSignature::STRING_TYPE));
        (*INTERNED).clone()
    }
    pub fn interned_TEXT_TYPE() -> metamodelica::Ref<TypeSignature> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<TypeSignature>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(TypeSignature::TEXT_TYPE));
        (*INTERNED).clone()
    }
    pub fn interned_STRING_TOKEN_TYPE() -> metamodelica::Ref<TypeSignature> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<TypeSignature>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(TypeSignature::STRING_TOKEN_TYPE));
        (*INTERNED).clone()
    }
    pub fn interned_INTEGER_TYPE() -> metamodelica::Ref<TypeSignature> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<TypeSignature>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(TypeSignature::INTEGER_TYPE));
        (*INTERNED).clone()
    }
    pub fn interned_REAL_TYPE() -> metamodelica::Ref<TypeSignature> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<TypeSignature>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(TypeSignature::REAL_TYPE));
        (*INTERNED).clone()
    }
    pub fn interned_BOOLEAN_TYPE() -> metamodelica::Ref<TypeSignature> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<TypeSignature>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(TypeSignature::BOOLEAN_TYPE));
        (*INTERNED).clone()
    }
}
pub fn interned_STRING_TYPE() -> metamodelica::Ref<TypeSignature> {
    TypeSignature::interned_STRING_TYPE()
}
pub fn interned_TEXT_TYPE() -> metamodelica::Ref<TypeSignature> {
    TypeSignature::interned_TEXT_TYPE()
}
pub fn interned_STRING_TOKEN_TYPE() -> metamodelica::Ref<TypeSignature> {
    TypeSignature::interned_STRING_TOKEN_TYPE()
}
pub fn interned_INTEGER_TYPE() -> metamodelica::Ref<TypeSignature> {
    TypeSignature::interned_INTEGER_TYPE()
}
pub fn interned_REAL_TYPE() -> metamodelica::Ref<TypeSignature> {
    TypeSignature::interned_REAL_TYPE()
}
pub fn interned_BOOLEAN_TYPE() -> metamodelica::Ref<TypeSignature> {
    TypeSignature::interned_BOOLEAN_TYPE()
}
impl Default for TypeSignature {
    fn default() -> Self {
        Self::STRING_TYPE
    }
}
pub use self::TypeSignature::{
    ARRAY_TYPE, BOOLEAN_TYPE, INTEGER_TYPE, LIST_TYPE, NAMED_TYPE, OPTION_TYPE, REAL_TYPE, STRING_TOKEN_TYPE,
    STRING_TYPE, TEXT_TYPE, TUPLE_TYPE, UNRESOLVED_TYPE,
};

pub type Expression = (metamodelica::Ref<ExpressionBase>, SourceInfo);

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ExpressionBase {
    TEMPLATE {
        items: metamodelica::List<(metamodelica::Ref<ExpressionBase>, SourceInfo)>,
        lquote: ArcStr,
        rquote: ArcStr,
    },
    STR_TOKEN {
        value: StringToken,
    },
    LITERAL {
        value: ArcStr,
        litType: metamodelica::Ref<TypeSignature>,
    },
    SOFT_NEW_LINE,
    BOUND_VALUE {
        boundPath: metamodelica::Ref<PathIdent>,
    },
    FUN_CALL {
        name: metamodelica::Ref<PathIdent>,
        args: metamodelica::List<(metamodelica::Ref<ExpressionBase>, SourceInfo)>,
    },
    CONDITION {
        /// Is not or inequal
        isNot: bool,
        lhsExp: Expression,
        /// always NONE() for now; it is a residuum from the form 'if exp is PATTERN then ...'
        rhsValue: Option<metamodelica::Ref<MatchingExp>>,
        trueBranch: Expression,
        elseBranch: Option<(metamodelica::Ref<ExpressionBase>, SourceInfo)>,
    },
    MATCH {
        matchExp: Expression,
        cases: metamodelica::List<(
            metamodelica::Ref<MatchingExp>,
            (metamodelica::Ref<ExpressionBase>, SourceInfo),
        )>,
    },
    MAP {
        argExp: Expression,
        ofBinding: metamodelica::Ref<MatchingExp>,
        mapExp: Expression,
        hasIndexIdentOpt: Option<ArcStr>,
    },
    MAP_ARG_LIST {
        parts: metamodelica::List<(metamodelica::Ref<ExpressionBase>, SourceInfo)>,
    },
    ESCAPED {
        exp: Expression,
        options: metamodelica::List<(ArcStr, Option<(metamodelica::Ref<ExpressionBase>, SourceInfo)>)>,
    },
    /// Indented block.
    INDENTATION {
        width: i32,
        items: metamodelica::List<(metamodelica::Ref<ExpressionBase>, SourceInfo)>,
    },
    LET {
        letExp: Expression,
        exp: Expression,
    },
    TEXT_CREATE {
        name: Ident,
        exp: Expression,
    },
    TEXT_ADD {
        name: Ident,
        exp: Expression,
    },
    NORET_CALL {
        name: metamodelica::Ref<PathIdent>,
        args: metamodelica::List<(metamodelica::Ref<ExpressionBase>, SourceInfo)>,
    },
    /// Parse error expression used when parser error occured.
    ERROR_EXP,
}
impl metamodelica::gc::MMTrace for ExpressionBase {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ExpressionBase::TEMPLATE { items, lquote, rquote } => {
                metamodelica::gc::MMTrace::mm_accept(items, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lquote, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rquote, __mmv)?;
                Ok(())
            }
            ExpressionBase::STR_TOKEN { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            ExpressionBase::LITERAL { value, litType } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(litType, __mmv)?;
                Ok(())
            }
            ExpressionBase::SOFT_NEW_LINE => Ok(()),
            ExpressionBase::BOUND_VALUE { boundPath } => {
                metamodelica::gc::MMTrace::mm_accept(boundPath, __mmv)?;
                Ok(())
            }
            ExpressionBase::FUN_CALL { name, args } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(args, __mmv)?;
                Ok(())
            }
            ExpressionBase::CONDITION {
                isNot,
                lhsExp,
                rhsValue,
                trueBranch,
                elseBranch,
            } => {
                metamodelica::gc::MMTrace::mm_accept(isNot, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lhsExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhsValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(trueBranch, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elseBranch, __mmv)?;
                Ok(())
            }
            ExpressionBase::MATCH { matchExp, cases } => {
                metamodelica::gc::MMTrace::mm_accept(matchExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cases, __mmv)?;
                Ok(())
            }
            ExpressionBase::MAP {
                argExp,
                ofBinding,
                mapExp,
                hasIndexIdentOpt,
            } => {
                metamodelica::gc::MMTrace::mm_accept(argExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ofBinding, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(mapExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasIndexIdentOpt, __mmv)?;
                Ok(())
            }
            ExpressionBase::MAP_ARG_LIST { parts } => {
                metamodelica::gc::MMTrace::mm_accept(parts, __mmv)?;
                Ok(())
            }
            ExpressionBase::ESCAPED { exp, options } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(options, __mmv)?;
                Ok(())
            }
            ExpressionBase::INDENTATION { width, items } => {
                metamodelica::gc::MMTrace::mm_accept(width, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(items, __mmv)?;
                Ok(())
            }
            ExpressionBase::LET { letExp, exp } => {
                metamodelica::gc::MMTrace::mm_accept(letExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            ExpressionBase::TEXT_CREATE { name, exp } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            ExpressionBase::TEXT_ADD { name, exp } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            ExpressionBase::NORET_CALL { name, args } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(args, __mmv)?;
                Ok(())
            }
            ExpressionBase::ERROR_EXP => Ok(()),
        }
    }
}
impl ExpressionBase {
    pub fn interned_SOFT_NEW_LINE() -> metamodelica::Ref<ExpressionBase> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<ExpressionBase>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(ExpressionBase::SOFT_NEW_LINE));
        (*INTERNED).clone()
    }
    pub fn interned_ERROR_EXP() -> metamodelica::Ref<ExpressionBase> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<ExpressionBase>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(ExpressionBase::ERROR_EXP));
        (*INTERNED).clone()
    }
}
pub fn interned_SOFT_NEW_LINE() -> metamodelica::Ref<ExpressionBase> {
    ExpressionBase::interned_SOFT_NEW_LINE()
}
pub fn interned_ERROR_EXP() -> metamodelica::Ref<ExpressionBase> {
    ExpressionBase::interned_ERROR_EXP()
}
impl Default for ExpressionBase {
    fn default() -> Self {
        Self::SOFT_NEW_LINE
    }
}
pub use self::ExpressionBase::{
    BOUND_VALUE, CONDITION, ERROR_EXP, ESCAPED, FUN_CALL, INDENTATION, LET, LITERAL, MAP, MAP_ARG_LIST, MATCH,
    NORET_CALL, SOFT_NEW_LINE, STR_TOKEN, TEMPLATE, TEXT_ADD, TEXT_CREATE,
};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum MatchingExp {
    BIND_AS_MATCH {
        bindIdent: Ident,
        matchingExp: metamodelica::Ref<MatchingExp>,
    },
    BIND_MATCH {
        bindIdent: Ident,
    },
    RECORD_MATCH {
        tagName: metamodelica::Ref<PathIdent>,
        fieldMatchings: metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>,
    },
    SOME_MATCH {
        value: metamodelica::Ref<MatchingExp>,
    },
    NONE_MATCH,
    TUPLE_MATCH {
        tupleArgs: metamodelica::List<metamodelica::Ref<MatchingExp>>,
    },
    LIST_MATCH {
        listElts: metamodelica::List<metamodelica::Ref<MatchingExp>>,
    },
    LIST_CONS_MATCH {
        head: metamodelica::Ref<MatchingExp>,
        rest: metamodelica::Ref<MatchingExp>,
    },
    STRING_MATCH {
        value: ArcStr,
    },
    LITERAL_MATCH {
        value: ArcStr,
        /// only INTEGER_TYPE, REAL_TYPE or BOOLEAN_TYPE
        litType: metamodelica::Ref<TypeSignature>,
    },
    REST_MATCH,
}
impl metamodelica::gc::MMTrace for MatchingExp {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            MatchingExp::BIND_AS_MATCH { bindIdent, matchingExp } => {
                metamodelica::gc::MMTrace::mm_accept(bindIdent, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(matchingExp, __mmv)?;
                Ok(())
            }
            MatchingExp::BIND_MATCH { bindIdent } => {
                metamodelica::gc::MMTrace::mm_accept(bindIdent, __mmv)?;
                Ok(())
            }
            MatchingExp::RECORD_MATCH {
                tagName,
                fieldMatchings,
            } => {
                metamodelica::gc::MMTrace::mm_accept(tagName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fieldMatchings, __mmv)?;
                Ok(())
            }
            MatchingExp::SOME_MATCH { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            MatchingExp::NONE_MATCH => Ok(()),
            MatchingExp::TUPLE_MATCH { tupleArgs } => {
                metamodelica::gc::MMTrace::mm_accept(tupleArgs, __mmv)?;
                Ok(())
            }
            MatchingExp::LIST_MATCH { listElts } => {
                metamodelica::gc::MMTrace::mm_accept(listElts, __mmv)?;
                Ok(())
            }
            MatchingExp::LIST_CONS_MATCH { head, rest } => {
                metamodelica::gc::MMTrace::mm_accept(head, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rest, __mmv)?;
                Ok(())
            }
            MatchingExp::STRING_MATCH { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            MatchingExp::LITERAL_MATCH { value, litType } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(litType, __mmv)?;
                Ok(())
            }
            MatchingExp::REST_MATCH => Ok(()),
        }
    }
}
impl MatchingExp {
    pub fn interned_NONE_MATCH() -> metamodelica::Ref<MatchingExp> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<MatchingExp>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(MatchingExp::NONE_MATCH));
        (*INTERNED).clone()
    }
    pub fn interned_REST_MATCH() -> metamodelica::Ref<MatchingExp> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<MatchingExp>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(MatchingExp::REST_MATCH));
        (*INTERNED).clone()
    }
}
pub fn interned_NONE_MATCH() -> metamodelica::Ref<MatchingExp> {
    MatchingExp::interned_NONE_MATCH()
}
pub fn interned_REST_MATCH() -> metamodelica::Ref<MatchingExp> {
    MatchingExp::interned_REST_MATCH()
}
impl Default for MatchingExp {
    fn default() -> Self {
        Self::NONE_MATCH
    }
}
pub use self::MatchingExp::{
    BIND_AS_MATCH, BIND_MATCH, LIST_CONS_MATCH, LIST_MATCH, LITERAL_MATCH, NONE_MATCH, RECORD_MATCH, REST_MATCH,
    SOME_MATCH, STRING_MATCH, TUPLE_MATCH,
};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum TypeInfo {
    TI_UNION_TYPE {
        recTags: metamodelica::List<(ArcStr, metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>)>,
    },
    TI_RECORD_TYPE {
        fields: TypedIdents,
    },
    TI_ALIAS_TYPE {
        aliasType: metamodelica::Ref<TypeSignature>,
    },
    /// Imported AST/builtin functions.
    TI_FUN_TYPE {
        inArgs: TypedIdents,
        outArgs: TypedIdents,
        tyVars: metamodelica::List<ArcStr>,
    },
    /// Imported AST constants.
    TI_CONST_TYPE {
        constType: metamodelica::Ref<TypeSignature>,
    },
}
impl metamodelica::gc::MMTrace for TypeInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            TypeInfo::TI_UNION_TYPE { recTags } => {
                metamodelica::gc::MMTrace::mm_accept(recTags, __mmv)?;
                Ok(())
            }
            TypeInfo::TI_RECORD_TYPE { fields } => {
                metamodelica::gc::MMTrace::mm_accept(fields, __mmv)?;
                Ok(())
            }
            TypeInfo::TI_ALIAS_TYPE { aliasType } => {
                metamodelica::gc::MMTrace::mm_accept(aliasType, __mmv)?;
                Ok(())
            }
            TypeInfo::TI_FUN_TYPE {
                inArgs,
                outArgs,
                tyVars,
            } => {
                metamodelica::gc::MMTrace::mm_accept(inArgs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(outArgs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(tyVars, __mmv)?;
                Ok(())
            }
            TypeInfo::TI_CONST_TYPE { constType } => {
                metamodelica::gc::MMTrace::mm_accept(constType, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for TypeInfo {
    fn default() -> Self {
        Self::TI_UNION_TYPE {
            recTags: Default::default(),
        }
    }
}
pub use self::TypeInfo::{TI_ALIAS_TYPE, TI_CONST_TYPE, TI_FUN_TYPE, TI_RECORD_TYPE, TI_UNION_TYPE};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ASTDef {
    pub importPackage: metamodelica::Ref<PathIdent>,
    /// names can be used unqualified
    pub isDefault: bool,
    /// from an interface file, always imported publicly
    pub isInterface: bool,
    pub types: metamodelica::List<(ArcStr, TypeInfo)>,
}

impl metamodelica::gc::MMTrace for ASTDef {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.importPackage, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isDefault, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isInterface, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.types, __mmv)?;
        Ok(())
    }
}
impl Default for ASTDef {
    fn default() -> Self {
        Self {
            importPackage: Default::default(),
            isDefault: Default::default(),
            isInterface: Default::default(),
            types: Default::default(),
        }
    }
}

pub type AST_DEF = ASTDef;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct TemplPackage {
    pub name: metamodelica::Ref<PathIdent>,
    pub astDefs: metamodelica::List<ASTDef>,
    pub templateDefs: metamodelica::List<(ArcStr, TemplateDef)>,
    pub annotationFooter: ArcStr,
}

impl metamodelica::gc::MMTrace for TemplPackage {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.astDefs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.templateDefs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.annotationFooter, __mmv)?;
        Ok(())
    }
}
impl Default for TemplPackage {
    fn default() -> Self {
        Self {
            name: Default::default(),
            astDefs: Default::default(),
            templateDefs: Default::default(),
            annotationFooter: Default::default(),
        }
    }
}

pub type TEMPL_PACKAGE = TemplPackage;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum TemplateDef {
    STR_TOKEN_DEF {
        value: StringToken,
    },
    LITERAL_DEF {
        value: ArcStr,
        litType: metamodelica::Ref<TypeSignature>,
    },
    TEMPLATE_DEF {
        args: TypedIdents,
        lesc: ArcStr,
        resc: ArcStr,
        exp: Expression,
    },
}
impl metamodelica::gc::MMTrace for TemplateDef {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            TemplateDef::STR_TOKEN_DEF { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            TemplateDef::LITERAL_DEF { value, litType } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(litType, __mmv)?;
                Ok(())
            }
            TemplateDef::TEMPLATE_DEF { args, lesc, resc, exp } => {
                metamodelica::gc::MMTrace::mm_accept(args, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lesc, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(resc, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for TemplateDef {
    fn default() -> Self {
        Self::STR_TOKEN_DEF {
            value: Default::default(),
        }
    }
}
pub(crate) use self::TemplateDef::{LITERAL_DEF, STR_TOKEN_DEF, TEMPLATE_DEF};

/* Output AST */
//type MMPublic = Boolean;
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct MMPackage {
    pub name: metamodelica::Ref<PathIdent>,
    pub mmDeclarations: metamodelica::List<MMDeclaration>,
    pub annotationFooter: ArcStr,
}

impl metamodelica::gc::MMTrace for MMPackage {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.mmDeclarations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.annotationFooter, __mmv)?;
        Ok(())
    }
}
impl Default for MMPackage {
    fn default() -> Self {
        Self {
            name: Default::default(),
            mmDeclarations: Default::default(),
            annotationFooter: Default::default(),
        }
    }
}

pub type MM_PACKAGE = MMPackage;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum MMDeclaration {
    MM_IMPORT {
        isPublic: bool,
        packageName: metamodelica::Ref<PathIdent>,
    },
    MM_STR_TOKEN_DECL {
        isPublic: bool,
        name: Ident,
        value: StringToken,
    },
    MM_LITERAL_DECL {
        isPublic: bool,
        name: Ident,
        value: ArcStr,
        litType: metamodelica::Ref<TypeSignature>,
    },
    MM_FUN {
        isPublic: bool,
        name: Ident,
        inArgs: TypedIdents,
        outArgs: TypedIdents,
        locals: TypedIdents,
        statements: metamodelica::List<metamodelica::Ref<MMExp>>,
        /// internal use only - a type of elaboration of the funtion.
        genInfoOpt: GenInfo,
    },
}
impl metamodelica::gc::MMTrace for MMDeclaration {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            MMDeclaration::MM_IMPORT { isPublic, packageName } => {
                metamodelica::gc::MMTrace::mm_accept(isPublic, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(packageName, __mmv)?;
                Ok(())
            }
            MMDeclaration::MM_STR_TOKEN_DECL { isPublic, name, value } => {
                metamodelica::gc::MMTrace::mm_accept(isPublic, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            MMDeclaration::MM_LITERAL_DECL {
                isPublic,
                name,
                value,
                litType,
            } => {
                metamodelica::gc::MMTrace::mm_accept(isPublic, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(litType, __mmv)?;
                Ok(())
            }
            MMDeclaration::MM_FUN {
                isPublic,
                name,
                inArgs,
                outArgs,
                locals,
                statements,
                genInfoOpt,
            } => {
                metamodelica::gc::MMTrace::mm_accept(isPublic, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(inArgs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(outArgs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(locals, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(statements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(genInfoOpt, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for MMDeclaration {
    fn default() -> Self {
        Self::MM_IMPORT {
            isPublic: Default::default(),
            packageName: Default::default(),
        }
    }
}
pub use self::MMDeclaration::{MM_FUN, MM_IMPORT, MM_LITERAL_DECL, MM_STR_TOKEN_DECL};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum MMExp {
    MM_ASSIGN {
        lhsArgs: metamodelica::List<ArcStr>,
        rhs: metamodelica::Ref<MMExp>,
    },
    MM_FN_CALL {
        fnName: metamodelica::Ref<PathIdent>,
        args: metamodelica::List<metamodelica::Ref<MMExp>>,
    },
    MM_IDENT {
        ident: metamodelica::Ref<PathIdent>,
    },
    /// constructor of type StringToken
    MM_STR_TOKEN {
        value: StringToken,
    },
    /// to pass a string constant as parameter of type String
    MM_STRING {
        value: ArcStr,
    },
    /// to pass a literal constant as parameter of type Integer, Real or Boolean
    MM_LITERAL {
        value: ArcStr,
    },
    MM_MATCH {
        matchCases: metamodelica::List<(
            metamodelica::List<metamodelica::Ref<MatchingExp>>,
            metamodelica::List<metamodelica::Ref<MMExp>>,
        )>,
    },
    MM_FOR_LOOP {
        idxName: Ident,
        arrName: Ident,
        eltName: Ident,
        statements: metamodelica::List<metamodelica::Ref<MMExp>>,
    },
    /// iterative list map: for eltName in listName loop match eltName ... end for;
    MM_LIST_FOR_LOOP {
        eltName: Ident,
        listName: Ident,
        /// pattern and body locals of the per-element match
        matchLocals: TypedIdents,
        /// the matched case plus an optional skip (else) case
        matchCases: metamodelica::List<(
            metamodelica::List<metamodelica::Ref<MatchingExp>>,
            metamodelica::List<metamodelica::Ref<MMExp>>,
        )>,
    },
}
impl metamodelica::gc::MMTrace for MMExp {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            MMExp::MM_ASSIGN { lhsArgs, rhs } => {
                metamodelica::gc::MMTrace::mm_accept(lhsArgs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                Ok(())
            }
            MMExp::MM_FN_CALL { fnName, args } => {
                metamodelica::gc::MMTrace::mm_accept(fnName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(args, __mmv)?;
                Ok(())
            }
            MMExp::MM_IDENT { ident } => {
                metamodelica::gc::MMTrace::mm_accept(ident, __mmv)?;
                Ok(())
            }
            MMExp::MM_STR_TOKEN { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            MMExp::MM_STRING { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            MMExp::MM_LITERAL { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            MMExp::MM_MATCH { matchCases } => {
                metamodelica::gc::MMTrace::mm_accept(matchCases, __mmv)?;
                Ok(())
            }
            MMExp::MM_FOR_LOOP {
                idxName,
                arrName,
                eltName,
                statements,
            } => {
                metamodelica::gc::MMTrace::mm_accept(idxName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(arrName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eltName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(statements, __mmv)?;
                Ok(())
            }
            MMExp::MM_LIST_FOR_LOOP {
                eltName,
                listName,
                matchLocals,
                matchCases,
            } => {
                metamodelica::gc::MMTrace::mm_accept(eltName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(listName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(matchLocals, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(matchCases, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for MMExp {
    fn default() -> Self {
        Self::MM_IDENT {
            ident: Default::default(),
        }
    }
}
pub use self::MMExp::{
    MM_ASSIGN, MM_FN_CALL, MM_FOR_LOOP, MM_IDENT, MM_LIST_FOR_LOOP, MM_LITERAL, MM_MATCH, MM_STR_TOKEN, MM_STRING,
};

pub type MMMatchCase = (
    metamodelica::List<metamodelica::Ref<MatchingExp>>,
    metamodelica::List<metamodelica::Ref<MMExp>>,
);

pub(crate) const imlicitTxt: &'static str = "txt";

pub(crate) const inPrefix: &'static str = "in_";

pub(crate) const outPrefix: &'static str = "out_";

//constant Ident imlicitInTxt = "intxt"; //not used ... there can be the same names for in/ou values
//constant Ident imlicitOutTxt = "outtxt";
pub(crate) const funArgNamePrefix: &'static str = "a_";

pub(crate) const extArgNamePrefix: &'static str = "e_";

pub(crate) const letValueNamePrefix: &'static str = "l_";

pub(crate) const indexNamePrefix: &'static str = "x_";

pub(crate) const caseBindingNamePrefix: &'static str = "i_";

pub(crate) const returnTempVarNamePrefix: &'static str = "ret_";

pub(crate) const constantNamePrefix: &'static str = "c_";

pub(crate) const textTempVarNamePrefix: &'static str = "txt_";

pub(crate) const textToStringNamePrefix: &'static str = "str_";

pub(crate) const matchFunPrefix: &'static str = "fun_";

pub(crate) const listMapFunPrefix: &'static str = "lm_";

pub(crate) const arrayMapFunPrefix: &'static str = "am_";

pub(crate) const scalarMapFunPrefix: &'static str = "smf_";

//constant Ident implicitTxtInArgName = "inTxt";
pub(crate) const matchDefaultArgName: &'static str = "mArg";

pub(crate) const impossibleIdent: &'static str = "*none*";

pub(crate) static imlicitTxtArg: std::sync::LazyLock<(ArcStr, metamodelica::Ref<TypeSignature>)> =
    std::sync::LazyLock::new(|| {
        (
            arcstr::literal!(imlicitTxt),
            crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE(),
        )
    });

//constant tuple<Ident,TypeSignature> imlicitTxtInputArg = (implicitTxtInArgName, TEXT_TYPE());
/* internal types */
pub(crate) static imlicitTxtMExp: std::sync::LazyLock<metamodelica::Ref<MatchingExp>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(MatchingExp::BIND_MATCH {
            bindIdent: arcstr::literal!(imlicitTxt),
        })
    });

pub(crate) static emptyExpression: std::sync::LazyLock<(metamodelica::Ref<ExpressionBase>, SourceInfo)> =
    std::sync::LazyLock::new(|| {
        (
            metamodelica::Ref::new(ExpressionBase::STR_TOKEN {
                value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!("") }),
            }),
            dummySourceInfo.clone(),
        )
    });

pub(crate) const emptyTxt: &'static str = "Tpl.emptyTxt";

pub(crate) const errorIdent: &'static str = "!error!";

pub(crate) static defaultIterOptions: std::sync::LazyLock<metamodelica::Ref<Tpl::IterOptions>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Tpl::IterOptions {
            startIndex0: 0,
            empty: None,
            separator: None,
            alignNum: 0,
            alignOfset: 0,
            alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(),
            wrapWidth: 0,
            wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(),
        })
    });

//only achievable by the 'from' clause
pub(crate) const indexOffsetOptionId: &'static str = "$indexOffset";

pub(crate) const emptyOptionId: &'static str = "empty";

pub(crate) const separatorOptionId: &'static str = "separator";

pub(crate) const alignNumOptionId: &'static str = "align";

pub(crate) const alignNumOffsetOptionId: &'static str = "alignOffset";

pub(crate) const alignSeparatorOptionId: &'static str = "alignSeparator";

pub(crate) const wrapWidthOptionId: &'static str = "wrap";

pub(crate) const wrapSeparatorOptionId: &'static str = "wrapSeparator";

pub(crate) const indentOptionId: &'static str = "indent";

pub(crate) const absIndentOptionId: &'static str = "absIndent";

pub(crate) const relIndentOptionId: &'static str = "relIndent";

pub(crate) const anchorOptionId: &'static str = "anchor";

//constant defaultMMOptions
pub(crate) static defaultEscOptions: std::sync::LazyLock<
    metamodelica::List<(ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>))>,
> = std::sync::LazyLock::new(|| {
    list![
        (
            arcstr::literal!(indexOffsetOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL { value: literal!("0") }),
                crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()
            )
        ),
        (
            arcstr::literal!(emptyOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_FN_CALL {
                    fnName: metamodelica::Ref::new(PathIdent::IDENT {
                        ident: literal!("SOME")
                    }),
                    args: list![metamodelica::Ref::new(MMExp::MM_STR_TOKEN {
                        value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!("") })
                    })]
                }),
                metamodelica::Ref::new(TypeSignature::OPTION_TYPE {
                    ofType: crate::TplAbsyn::TypeSignature::interned_STRING_TOKEN_TYPE()
                })
            )
        ),
        (
            arcstr::literal!(separatorOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL {
                    value: literal!("NONE()")
                }),
                metamodelica::Ref::new(TypeSignature::OPTION_TYPE {
                    ofType: crate::TplAbsyn::TypeSignature::interned_STRING_TOKEN_TYPE()
                })
            )
        ),
        (
            arcstr::literal!(alignNumOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL { value: literal!("10") }),
                crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()
            )
        ),
        (
            arcstr::literal!(alignNumOffsetOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL { value: literal!("0") }),
                crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()
            )
        ),
        (
            arcstr::literal!(alignSeparatorOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_STR_TOKEN {
                    value: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()
                }),
                crate::TplAbsyn::TypeSignature::interned_STRING_TOKEN_TYPE()
            )
        ),
        (
            arcstr::literal!(wrapWidthOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL { value: literal!("100") }),
                crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()
            )
        ),
        (
            arcstr::literal!(wrapSeparatorOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_STR_TOKEN {
                    value: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()
                }),
                crate::TplAbsyn::TypeSignature::interned_STRING_TOKEN_TYPE()
            )
        ),
        (
            arcstr::literal!(indentOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL { value: literal!("0") }),
                crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()
            )
        ),
        (
            arcstr::literal!(absIndentOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL { value: literal!("0") }),
                crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()
            )
        ),
        (
            arcstr::literal!(relIndentOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL { value: literal!("0") }),
                crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()
            )
        ),
        (
            arcstr::literal!(anchorOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL { value: literal!("0") }),
                crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()
            )
        )
    ]
});

pub(crate) static nonSpecifiedIterOptions: std::sync::LazyLock<
    metamodelica::List<(ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>))>,
> = std::sync::LazyLock::new(|| {
    list![
        (
            arcstr::literal!(indexOffsetOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL { value: literal!("0") }),
                crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()
            )
        ),
        (
            arcstr::literal!(emptyOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL {
                    value: literal!("NONE()")
                }),
                metamodelica::Ref::new(TypeSignature::OPTION_TYPE {
                    ofType: crate::TplAbsyn::TypeSignature::interned_STRING_TOKEN_TYPE()
                })
            )
        ),
        (
            arcstr::literal!(separatorOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL {
                    value: literal!("NONE()")
                }),
                metamodelica::Ref::new(TypeSignature::OPTION_TYPE {
                    ofType: crate::TplAbsyn::TypeSignature::interned_STRING_TOKEN_TYPE()
                })
            )
        ),
        (
            arcstr::literal!(alignNumOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL { value: literal!("0") }),
                crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()
            )
        ),
        (
            arcstr::literal!(alignNumOffsetOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL { value: literal!("0") }),
                crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()
            )
        ),
        (
            arcstr::literal!(alignSeparatorOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_STR_TOKEN {
                    value: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()
                }),
                crate::TplAbsyn::TypeSignature::interned_STRING_TOKEN_TYPE()
            )
        ),
        (
            arcstr::literal!(wrapWidthOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_LITERAL { value: literal!("0") }),
                crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()
            )
        ),
        (
            arcstr::literal!(wrapSeparatorOptionId),
            (
                metamodelica::Ref::new(MMExp::MM_STR_TOKEN {
                    value: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()
                }),
                crate::TplAbsyn::TypeSignature::interned_STRING_TOKEN_TYPE()
            )
        )
    ]
});

pub type MMEscOption = (ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>));

pub type ScopeEnv = metamodelica::List<Scope>;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Scope {
    FUN_SCOPE {
        args: TypedIdents,
        /// local encoded args; used to elaborate the actual args of closures
        localArgs: TypedIdents,
    },
    CASE_SCOPE {
        mExp: metamodelica::Ref<MatchingExp>,
        mType: metamodelica::Ref<TypeSignature>,
        /// source name -> local declaration name table
        localNames: metamodelica::List<(ArcStr, ArcStr)>,
        /// accumulated locals used by the cases in this match elaborated level
        accLocals: TypedIdents,
        /// local args from the upper scope - all of them are from their upper FUN_SCOPE()
        extArgs: TypedIdents,
        /// local name of the match argument
        matchArgName: Ident,
        /// true for 'match' or 'map', false for 'if' elaborated cases; desides if the implicit record fields' lookup can continue upwards the scope stack.
        hasImplicitScope: bool,
    },
    LET_SCOPE {
        /// original ident
        ident: Ident,
        idType: metamodelica::Ref<TypeSignature>,
        /// encoded ident with prefix and suffix unique for the local scope
        freshIdent: Ident,
        /// true when found by resolveBoundPath()
        isUsed: bool,
    },
    /// forbidden access - scope of a text add ident; to prevent recursive usage of texts;
    ///     or scope of an elaborated let binding; to force a fresh local ident to be created when the same name is re-bound inside the let expression.
    RECURSIVE_SCOPE {
        recIdent: Ident,
        /// local name
        freshIdent: Ident,
    },
}
impl metamodelica::gc::MMTrace for Scope {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Scope::FUN_SCOPE { args, localArgs } => {
                metamodelica::gc::MMTrace::mm_accept(args, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(localArgs, __mmv)?;
                Ok(())
            }
            Scope::CASE_SCOPE {
                mExp,
                mType,
                localNames,
                accLocals,
                extArgs,
                matchArgName,
                hasImplicitScope,
            } => {
                metamodelica::gc::MMTrace::mm_accept(mExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(mType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(localNames, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(accLocals, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(extArgs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(matchArgName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasImplicitScope, __mmv)?;
                Ok(())
            }
            Scope::LET_SCOPE {
                ident,
                idType,
                freshIdent,
                isUsed,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ident, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(idType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(freshIdent, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isUsed, __mmv)?;
                Ok(())
            }
            Scope::RECURSIVE_SCOPE { recIdent, freshIdent } => {
                metamodelica::gc::MMTrace::mm_accept(recIdent, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(freshIdent, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Scope::{CASE_SCOPE, FUN_SCOPE, LET_SCOPE, RECURSIVE_SCOPE};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct MapContext {
    pub ofBinding: metamodelica::Ref<MatchingExp>,
    pub mapExp: Expression,
    pub iterMMExpOptions: metamodelica::List<(ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>))>,
    /// used index variable
    pub hasIndexIdentOpt: Option<ArcStr>,
    /// Whether PushIter/NextIter/PopIter is necessary.
    pub useIter: bool,
}

impl metamodelica::gc::MMTrace for MapContext {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.ofBinding, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.mapExp, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.iterMMExpOptions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.hasIndexIdentOpt, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.useIter, __mmv)?;
        Ok(())
    }
}
pub type MAP_CONTEXT = MapContext;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum GenInfo {
    GI_TEMPL_FUN,
    GI_MATCH_FUN,
    GI_MAP_FUN {
        mapType: metamodelica::Ref<TypeSignature>,
        mapContext: MapContext,
    },
}
impl metamodelica::gc::MMTrace for GenInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            GenInfo::GI_TEMPL_FUN => Ok(()),
            GenInfo::GI_MATCH_FUN => Ok(()),
            GenInfo::GI_MAP_FUN { mapType, mapContext } => {
                metamodelica::gc::MMTrace::mm_accept(mapType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(mapContext, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::GenInfo::{GI_MAP_FUN, GI_MATCH_FUN, GI_TEMPL_FUN};

// *** functions ***
pub(crate) fn transformAST(mut inTplPackage: &TemplPackage) -> Result<MMPackage> {
    let mut outMMPackage: MMPackage;
    outMMPackage = (match inTplPackage.clone() {
        _ => {
            let mut name: metamodelica::Ref<PathIdent>;
            let mut templateDefs: metamodelica::List<(ArcStr, TemplateDef)>;
            let mut mmDeclarations: metamodelica::List<MMDeclaration>;
            let mut tp: TemplPackage;
            let mut astDefs: metamodelica::List<ASTDef>;
            let mut annotationFooter: ArcStr;
            tp = fullyQualifyTemplatePackage(inTplPackage)?;
            let TemplPackage {
                name: __pa0,
                astDefs: __pa1,
                templateDefs: __pa2,
                annotationFooter: __pa3,
            } = &tp;
            name = metamodelica::Own::own(__pa0);
            astDefs = metamodelica::Own::own(__pa1);
            templateDefs = metamodelica::Own::own(__pa2);
            annotationFooter = metamodelica::Own::own(__pa3);
            mmDeclarations = importDeclarations(&astDefs);
            mmDeclarations = transformTemplateDefs(templateDefs, tp, mmDeclarations)?;
            mmDeclarations = mmDeclarations.reverse();
            MMPackage {
                name: name,
                mmDeclarations: mmDeclarations,
                annotationFooter: annotationFooter,
            }
        }
    });
    Ok(outMMPackage)
}

pub(crate) fn fullyQualifyTemplatePackage(mut inTplPackage: &TemplPackage) -> Result<TemplPackage> {
    let mut outTplPackage: TemplPackage;
    outTplPackage = (match inTplPackage.clone() {
        TemplPackage {
            name: mut name,
            astDefs: mut astDefs,
            templateDefs: mut templateDefs,
            annotationFooter: mut ann,
        } => {
            let mut astDefs = astDefs.clone();
            let mut templateDefs = templateDefs.clone();
            astDefs = fullyQualifyASTDefs(metamodelica::AsArg::as_arg(&astDefs))?;
            templateDefs = listMap1Tuple22(
                metamodelica::AsArg::as_arg(&templateDefs),
                (std::sync::Arc::new(fullyQualifyTemplateDef)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(TemplateDef, metamodelica::List<ASTDef>) -> Result<TemplateDef> + 'static,
                    >),
                astDefs.clone(),
            )?;
            TemplPackage {
                name: name.clone(),
                astDefs: astDefs.clone(),
                templateDefs: templateDefs.clone(),
                annotationFooter: ann.clone(),
            }
        }
    });
    Ok(outTplPackage)
}

pub(crate) fn importDeclarations(mut inASTDefs: &metamodelica::List<ASTDef>) -> metamodelica::List<MMDeclaration> {
    let mut outMMDecls: metamodelica::List<MMDeclaration> = metamodelica::nil();
    for mut astDef in &**inASTDefs {
        outMMDecls = metamodelica::cons(
            MMDeclaration::MM_IMPORT {
                isPublic: astDef.isDefault.clone() || astDef.isInterface.clone(),
                packageName: astDef.importPackage.clone(),
            },
            outMMDecls,
        );
    }
    outMMDecls
}

pub(crate) fn transformTemplateDefs(
    mut inTemplateDefsRest: metamodelica::List<(ArcStr, TemplateDef)>,
    mut inTplPackage: TemplPackage,
    mut inAccMMDecls: metamodelica::List<MMDeclaration>,
) -> Result<metamodelica::List<MMDeclaration>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inTemplateDefsRest, inTplPackage, inAccMMDecls)) {
            (Deref @ metamodelica::ListNode::Nil, _, accMMDecls) => {
                return Ok(accMMDecls.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: (tplname, TemplateDef::STR_TOKEN_DEF { value: stvalue }), tail: restTDefs }, tplPackage, accMMDecls) => {
                let mut mmDecls: metamodelica::List<MMDeclaration>;
                let mut tplname = (*tplname).clone();
                tplname = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(constantNamePrefix)); __mm_s.push_str(&*tplname); ArcStr::from(__mm_s) };
                { (inTemplateDefsRest, inTplPackage, inAccMMDecls) = (restTDefs.clone(), tplPackage.clone(), metamodelica::cons(MMDeclaration::MM_STR_TOKEN_DECL { isPublic: true, name: tplname.clone(), value: stvalue.clone() }, accMMDecls.clone())); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: (tplname, TemplateDef::LITERAL_DEF { value: svalue, litType }), tail: restTDefs }, tplPackage, accMMDecls) => {
                let mut mmDecls: metamodelica::List<MMDeclaration>;
                let mut tplname = (*tplname).clone();
                tplname = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(constantNamePrefix)); __mm_s.push_str(&*tplname); ArcStr::from(__mm_s) };
                { (inTemplateDefsRest, inTplPackage, inAccMMDecls) = (restTDefs.clone(), tplPackage.clone(), metamodelica::cons(MMDeclaration::MM_LITERAL_DECL { isPublic: true, name: tplname.clone(), value: svalue.clone(), litType: litType.clone() }, accMMDecls.clone())); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: (tplname, TemplateDef::TEMPLATE_DEF { args: targs, exp: texp, .. }), tail: restTDefs }, tplPackage, accMMDecls) => {
                let mut encArgs: TypedIdents;
                let mut locals: TypedIdents;
                let mut iargs: TypedIdents;
                let mut oargs: TypedIdents;
                let mut stmts: metamodelica::List<metamodelica::Ref<MMExp>>;
                let mut mmFun: MMDeclaration;
                let mut accMMDecls = (*accMMDecls).clone();
                encArgs = List::map1(targs.clone(), &move |__a0: (ArcStr, metamodelica::Ref<TypeSignature>), __a1: ArcStr| encodeTypedIdent(&__a0, &__a1), arcstr::literal!(funArgNamePrefix))?;
                (stmts, locals, _, accMMDecls, _) = statementsFromExp(&(texp.clone()), metamodelica::nil(), metamodelica::nil(), arcstr::literal!(imlicitTxt), arcstr::literal!(imlicitTxt), metamodelica::nil(), list![Scope::FUN_SCOPE { args: targs.clone(), localArgs: encArgs.clone() }], tplPackage.clone(), accMMDecls.clone())?;
                iargs = metamodelica::cons(imlicitTxtArg.clone(), encArgs);
                oargs = List::filterOnTrue(iargs.clone(), (std::sync::Arc::new(move |__a0: (ArcStr, metamodelica::Ref<TypeSignature>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(isText(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, metamodelica::Ref<TypeSignature>)) -> Result<bool> + 'static>))?;
                stmts = stmts.reverse();
                stmts = addOutPrefixes(&stmts, oargs.clone(), metamodelica::nil())?;
                (stmts, locals, accMMDecls) = inlineLastFunIfSingleCall(iargs.clone(), oargs.clone(), stmts, locals, accMMDecls.clone())?;
                mmFun = MMDeclaration::MM_FUN { isPublic: true, name: tplname.clone(), inArgs: iargs, outArgs: oargs, locals: locals, statements: stmts, genInfoOpt: crate::TplAbsyn::GenInfo::GI_TEMPL_FUN };
                { (inTemplateDefsRest, inTplPackage, inAccMMDecls) = (restTDefs.clone(), tplPackage.clone(), metamodelica::cons(mmFun, accMMDecls.clone())); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn inlineLastFunIfSingleCall(
    mut inInArgs: TypedIdents,
    mut inOutArgs: TypedIdents,
    mut inStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inLocals: TypedIdents,
    mut inAccMMDecls: metamodelica::List<MMDeclaration>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<MMExp>>,
    TypedIdents,
    metamodelica::List<MMDeclaration>,
)> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outLocals: TypedIdents;
    let mut outMMDecls: metamodelica::List<MMDeclaration>;
    (outStmts, outLocals, outMMDecls) = 'mc: {
        let __mc_input = (inInArgs, inOutArgs, inStmts, inLocals, inAccMMDecls);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (iargs, oargs, Deref @ metamodelica::ListNode::Cons { head: Deref @ MMExp::MM_ASSIGN { rhs: Deref @ MMExp::MM_FN_CALL { fnName: Deref @ PathIdent::IDENT { ident: fidCalled }, .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: MMDeclaration::MM_FUN { isPublic: _, name: fidLast, inArgs: iargsL, outArgs: oargsL, locals, statements: stmts, genInfoOpt: genInfo }, tail: accMMDecls }) => {
                    let true = (stringEq(&fidCalled, &fidLast)) else { return Err("pattern mismatch") };
                    if '__try0: {
                        let GenInfo::GI_TEMPL_FUN { .. } = (genInfo.clone()) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    let true = (iargs.clone() == iargsL.clone()) else { return Err("pattern mismatch") };
                    let true = (oargs.clone() == oargsL.clone()) else { return Err("pattern mismatch") };
                    Ok((stmts.clone(), locals.clone(), accMMDecls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, stmts, locals, accMMDecls) => {
                    Ok((stmts.clone(), locals.clone(), accMMDecls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outStmts, outLocals, outMMDecls))
}

//prepend "i" in front of the ident to obey the MM rule that no identifier can start with "_"
pub(crate) fn encodeIdent(mut inIdent: Ident, mut prefix: &Ident) -> Result<Ident> {
    let mut outIdent: Ident;
    outIdent = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*prefix);
        __mm_s.push_str(&*encodeIdentNoPrefix(inIdent)?);
        ArcStr::from(__mm_s)
    };
    Ok(outIdent)
}

//every ident to be encoded as ".ident"
//where "." is encoded as "_" or "_0" in the case it is followed with "_" (idents starting with _)
fn encodeIdentNoPrefix(mut inIdent: Ident) -> Result<Ident> {
    let mut outIdent: Ident;
    outIdent = 'mc: {
        let __mc_input = inIdent;
        if let Ok(__v) = (|| -> Result<_> {
            let mut ident = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (((ident).len() as i32) > 0
                && metamodelica::stringEq(&(stringGetStringChar(ident.clone(), 1)?), &(literal!("_"))))
            else {
                return Err("pattern mismatch");
            };
            ident = System::stringReplace(ident.clone(), literal!("_"), literal!("__"))?;
            ident = System::stringReplace(ident.clone(), literal!("._"), literal!("_0"))?;
            ident = System::stringReplace(ident.clone(), literal!("."), literal!("_"))?;
            ident = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("0"));
                __mm_s.push_str(&*ident);
                ArcStr::from(__mm_s)
            };
            Ok(ident.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut ident = __mc_input.clone() else {
                return Err("nomatch");
            };
            ident = System::stringReplace(ident.clone(), literal!("_"), literal!("__"))?;
            ident = System::stringReplace(ident.clone(), literal!("._"), literal!("_0"))?;
            ident = System::stringReplace(ident.clone(), literal!("."), literal!("_"))?;
            Ok(ident.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("-!!!encodeIdentNoPrefix failed\n"))?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outIdent)
}

pub(crate) fn encodePathIdent(mut inPath: &metamodelica::Ref<PathIdent>, mut prefix: &Ident) -> Result<Ident> {
    let mut outEncIdent: Ident;
    outEncIdent = encodeIdent(pathIdentString(inPath)?, prefix)?;
    Ok(outEncIdent)
}

pub(crate) fn encodeTypedIdent(
    mut inTypedIdent: &(ArcStr, metamodelica::Ref<TypeSignature>),
    mut prefix: &Ident,
) -> Result<(ArcStr, metamodelica::Ref<TypeSignature>)> {
    let mut outTypedIdent: (ArcStr, metamodelica::Ref<TypeSignature>);
    outTypedIdent = 'mc: {
        let __mc_input = inTypedIdent;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, ts) => {
                    let mut ident = (*ident).clone();
                    ident = encodeIdent(ident.clone(), prefix)?;
                    Ok((ident.clone(), ts.clone()))
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
                    Debug::trace(literal!("-!!!encodeTypedIdent failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTypedIdent)
}

pub(crate) fn addOutPrefixes(
    mut inStmts: &metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inTextArgs: TypedIdents,
    mut inTranslatedTextArgs: metamodelica::List<(ArcStr, ArcStr)>,
) -> Result<metamodelica::List<metamodelica::Ref<MMExp>>> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    outStmts = 'mc: {
        let __mc_input = (&**inStmts, inTextArgs, inTranslatedTextArgs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, txtargs, trIdents) => {
                    let mut stmts: metamodelica::List<metamodelica::Ref<MMExp>>;
                    stmts = addOutTextAssigns(metamodelica::AsArg::as_arg(&txtargs), trIdents.clone());
                    Ok(stmts.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ MMExp::MM_ASSIGN { lhsArgs: largs, rhs }, tail: stmts }, txtargs, trIdents) => {
                    let mut largs = (*largs).clone();
                    let mut rhs = (*rhs).clone();
                    let mut stmts = (*stmts).clone();
                    let mut trIdents = (*trIdents).clone();
                    rhs = addOutPrefixesRhs(rhs.clone(), trIdents.clone());
                    (largs, trIdents) = addOutPrefixesLhs(metamodelica::AsArg::as_arg(&largs), txtargs.clone(), trIdents.clone())?;
                    stmts = addOutPrefixes(metamodelica::AsArg::as_arg(&stmts), txtargs.clone(), trIdents.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(MMExp::MM_ASSIGN { lhsArgs: largs.clone(), rhs: rhs.clone() }), stmts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: stmt, tail: stmts }, txtargs, trIdents) => {
                    let mut stmts = (*stmts).clone();
                    stmts = addOutPrefixes(metamodelica::AsArg::as_arg(&stmts), txtargs.clone(), trIdents.clone())?;
                    Ok(metamodelica::cons(stmt.clone(), stmts.clone()))
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
                    Debug::trace(literal!("-!!!addOutPrefixes failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStmts)
}

pub(crate) fn addOutPrefixesRhs(
    mut inStmt: metamodelica::Ref<MMExp>,
    mut inTranslatedTextArgs: metamodelica::List<(ArcStr, ArcStr)>,
) -> metamodelica::Ref<MMExp> {
    let mut outStmt: metamodelica::Ref<MMExp>;
    outStmt = 'mc: {
        let __mc_input = (&*inStmt, inTranslatedTextArgs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MMExp::MM_IDENT { ident: Deref @ PathIdent::IDENT { ident } }, trIdents) => {
                    let mut outident: Ident;
                    outident = lookupTupleList(trIdents.clone(), ident.clone())?;
                    Ok(metamodelica::Ref::new(MMExp::MM_IDENT { ident: metamodelica::Ref::new(PathIdent::IDENT { ident: outident.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MMExp::MM_FN_CALL { fnName: fpath, args: fargs }, trIdents) => {
                    let mut fargs = (*fargs).clone();
                    fargs = List::map1(fargs.clone(), &fnptr!(addOutPrefixesRhs, metamodelica::Ref<MMExp>, metamodelica::List<(ArcStr, ArcStr)>), trIdents.clone())?;
                    Ok(metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: fpath.clone(), args: fargs.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inStmt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStmt
}

pub(crate) fn addOutPrefixesLhs(
    mut inLhsArgs: &metamodelica::List<ArcStr>,
    mut inTextArgs: TypedIdents,
    mut inTranslatedTextArgs: metamodelica::List<(ArcStr, ArcStr)>,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<(ArcStr, ArcStr)>)> {
    let mut outLhsArgs: metamodelica::List<ArcStr>;
    let mut outTranslatedTextArgs: metamodelica::List<(ArcStr, ArcStr)>;
    (outLhsArgs, outTranslatedTextArgs) = 'mc: {
        let __mc_input = (&**inLhsArgs, inTextArgs, inTranslatedTextArgs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, trIdents) => {
                    Ok((metamodelica::nil(), trIdents.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: ident, tail: largs }, txtargs, trIdents) => {
                    let mut outident: ArcStr;
                    let mut largs = (*largs).clone();
                    let mut trIdents = (*trIdents).clone();
                    lookupTupleList(txtargs.clone(), ident.clone())?;
                    outident = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(outPrefix)); __mm_s.push_str(&*ident); ArcStr::from(__mm_s) };
                    trIdents = updateTupleList(trIdents.clone(), (ident.clone(), outident.clone()));
                    (largs, trIdents) = addOutPrefixesLhs(metamodelica::AsArg::as_arg(&largs), txtargs.clone(), trIdents.clone())?;
                    Ok((metamodelica::cons(outident.clone(), largs.clone()), trIdents.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: ident, tail: largs }, txtargs, trIdents) => {
                    let mut largs = (*largs).clone();
                    let mut trIdents = (*trIdents).clone();
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(txtargs.clone(), ident.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    (largs, trIdents) = addOutPrefixesLhs(metamodelica::AsArg::as_arg(&largs), txtargs.clone(), trIdents.clone())?;
                    Ok((metamodelica::cons(ident.clone(), largs.clone()), trIdents.clone()))
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
                    Debug::trace(literal!("-!!!addOutPrefixesLhs failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outLhsArgs, outTranslatedTextArgs))
}

pub(crate) fn addOutTextAssigns(
    mut inTextArgs: &TypedIdents,
    mut inTranslatedTextArgs: metamodelica::List<(ArcStr, ArcStr)>,
) -> metamodelica::List<metamodelica::Ref<MMExp>> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>> = metamodelica::nil();
    let mut outident: ArcStr;
    let mut id: (ArcStr, metamodelica::Ref<TypeSignature>) = (
        arcstr::literal!(""),
        metamodelica::Ref::new(TypeSignature::BOOLEAN_TYPE),
    );
    let mut ident: Ident;
    for mut id in &**inTextArgs {
        let mut id = id.clone();
        (ident, _) = id;
        if '__try0: {
            unwrap_break_err!(lookupTupleList(inTranslatedTextArgs.clone(), ident.clone()), '__try0);
            Ok::<(), &'static str>(())
        }
        .is_err()
        {
            outident = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*arcstr::literal!(outPrefix));
                __mm_s.push_str(&*ident);
                ArcStr::from(__mm_s)
            };
            outStmts = metamodelica::cons(
                metamodelica::Ref::new(MMExp::MM_ASSIGN {
                    lhsArgs: list![outident.clone()],
                    rhs: metamodelica::Ref::new(MMExp::MM_IDENT {
                        ident: metamodelica::Ref::new(PathIdent::IDENT { ident: ident.clone() }),
                    }),
                }),
                outStmts.clone(),
            );
        }
    }
    outStmts = metamodelica::Dangerous::listReverseInPlace(outStmts);
    outStmts
}

pub(crate) fn isAssignedIdent(
    mut inStatementList: &metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inIdent: Ident,
) -> Result<bool> {
    let mut outIsAssigned: bool;
    let mut largs: metamodelica::List<ArcStr>;
    for mut st in &**inStatementList {
        let __pa0 = ::match_deref::match_deref! { match &(st.clone()) {
            Deref @ MMExp::MM_ASSIGN { lhsArgs: __pa0, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        largs = metamodelica::Own::own(__pa0);
        if listMember(inIdent.clone(), largs) {
            outIsAssigned = true;
            return Ok(outIsAssigned);
        }
    }
    outIsAssigned = false;
    Ok(outIsAssigned)
}

pub(crate) fn statementsFromExp(
    mut inExp: &Expression,
    mut inMMEscOptions: metamodelica::List<(ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>))>,
    mut inStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inInText: Ident,
    mut inOutText: Ident,
    mut inLocals: TypedIdents,
    mut inScopeEnv: ScopeEnv,
    mut inTplPackage: TemplPackage,
    mut inAccMMDecls: metamodelica::List<MMDeclaration>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<MMExp>>,
    TypedIdents,
    ScopeEnv,
    metamodelica::List<MMDeclaration>,
    Ident,
)> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outLocals: TypedIdents;
    let mut outScopeEnv: ScopeEnv;
    let mut outMMDecls: metamodelica::List<MMDeclaration>;
    let mut outInText: Ident;
    (outStmts, outLocals, outScopeEnv, outMMDecls, outInText) = 'mc: {
        let __mc_input = (
            inExp,
            inMMEscOptions,
            inStmts,
            inInText,
            inOutText,
            inLocals,
            inScopeEnv,
            inTplPackage,
            inAccMMDecls,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::TEMPLATE { items: explst, .. }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromExpList(metamodelica::AsArg::as_arg(&explst), stmts.clone(), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::LITERAL { value: litvalue, .. }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, _, accMMDecls) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    stmt = tplStatement(literal!("writeTok"), list![metamodelica::Ref::new(MMExp::MM_STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: litvalue.clone() }) })], intxt.clone(), outtxt.clone());
                    Ok((metamodelica::cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::SOFT_NEW_LINE { .. }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, _, accMMDecls) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    stmt = tplStatement(literal!("softNewLine"), metamodelica::nil(), intxt.clone(), outtxt.clone());
                    Ok((metamodelica::cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING { value: Deref @ "" } }, _), mmopts, stmts, intxt, _, locals, scEnv, _, accMMDecls) => {
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::STR_TOKEN { value: st }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, _, accMMDecls) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    stmt = tplStatement(literal!("writeTok"), list![metamodelica::Ref::new(MMExp::MM_STR_TOKEN { value: st.clone() })], intxt.clone(), outtxt.clone());
                    Ok((metamodelica::cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::BOUND_VALUE { boundPath: path }, sinfo), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage @ TemplPackage { astDefs, .. }, accMMDecls) => {
                    let mut mmexp: metamodelica::Ref<MMExp>;
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut exptype: metamodelica::Ref<TypeSignature>;
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n BOUND_VALUE resolving boundPath = ")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&path))?); ArcStr::from(__mm_s) })?;
                    }
                    (mmexp, idtype, scEnv) = resolveBoundPath(path.clone(), scEnv.clone(), metamodelica::AsArg::as_arg(&tplPackage))?;
                    checkResolvedType(metamodelica::AsArg::as_arg(&path), &idtype, literal!("bound value"), metamodelica::AsArg::as_arg(&sinfo));
                    exptype = deAliasedType(&idtype, astDefs.clone());
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n BOUND_VALUE resolved mmexp = ")); __mm_s.push_str(&*mmExpString(mmexp.clone())?); __mm_s.push_str(&*literal!(" : ")); __mm_s.push_str(&*typeSignatureString(&idtype)?); __mm_s.push_str(&*literal!(" (dealiased: ")); __mm_s.push_str(&*typeSignatureString(&exptype)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })?;
                    }
                    (stmts, locals, scEnv, accMMDecls, intxt) = addWriteCallFromMMExp(true, mmexp.clone(), exptype.clone(), sinfo.clone(), mmopts.clone(), stmts.clone(), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::FUN_CALL { name: fname, args: explst }, sinfo), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage @ TemplPackage { astDefs, .. }, accMMDecls) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    let mut mmexp: metamodelica::Ref<MMExp>;
                    let mut tyVars: metamodelica::List<ArcStr>;
                    let mut iargs: TypedIdents;
                    let mut oargs: TypedIdents;
                    let mut rettype: metamodelica::Ref<TypeSignature>;
                    let mut argvals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
                    let mut hasretval: bool;
                    let mut fname = (*fname).clone();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n FUN_CALL fname = ")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&fname))?); ArcStr::from(__mm_s) })?;
                    }
                    (fname, iargs, oargs, tyVars) = getFunSignature(fname.clone(), metamodelica::AsArg::as_arg(&sinfo), metamodelica::AsArg::as_arg(&tplPackage))?;
                    (argvals, stmts, locals, scEnv, accMMDecls) = statementsFromArgList(metamodelica::AsArg::as_arg(&explst), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!(" FUN_CALL argList stmts generation passed\n"))?;
                    }
                    (hasretval, stmt, mmexp, rettype, locals, intxt) = statementFromFun(argvals.clone(), fname.clone(), iargs.clone(), oargs.clone(), tyVars.clone(), intxt.clone(), outtxt.clone(), locals.clone(), tplPackage.clone(), metamodelica::AsArg::as_arg(&sinfo))?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" FUN_CALL stmt =\n")); __mm_s.push_str(&*stmtsString(&(list![stmt.clone()]))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    rettype = deAliasedType(&rettype, astDefs.clone());
                    (stmts, locals, scEnv, accMMDecls, intxt) = addWriteCallFromMMExp(hasretval, mmexp.clone(), rettype.clone(), sinfo.clone(), mmopts.clone(), metamodelica::cons(stmt.clone(), stmts.clone()), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::MATCH { matchExp: exp, cases: mcases }, sinfo), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    let mut fname: metamodelica::Ref<PathIdent>;
                    let mut iargs: TypedIdents;
                    let mut oargs: TypedIdents;
                    let mut argval: (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo);
                    let mut argvals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
                    let mut exp = (*exp).clone();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    (argval, stmts, locals, scEnv, accMMDecls) = statementsFromArg(exp.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (argval, exp, stmts, locals) = adaptTextToString(argval.clone(), exp.clone(), stmts.clone(), locals.clone(), metamodelica::AsArg::as_arg(&tplPackage))?;
                    (argvals, fname, iargs, oargs, scEnv, accMMDecls) = makeMatchFun(argval.clone(), mcases.clone(), &(exp.clone()), true, scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (_, stmt, _, _, locals, intxt) = statementFromFun(argvals.clone(), fname.clone(), iargs.clone(), oargs.clone(), metamodelica::nil(), intxt.clone(), outtxt.clone(), locals.clone(), tplPackage.clone(), metamodelica::AsArg::as_arg(&sinfo))?;
                    Ok((metamodelica::cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::CONDITION { isNot: isnot, lhsExp: exp, rhsValue: rhsval, trueBranch: tbranch, elseBranch: ebranch }, sinfo), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage @ TemplPackage { astDefs, .. }, accMMDecls) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    let mut fname: metamodelica::Ref<PathIdent>;
                    let mut iargs: TypedIdents;
                    let mut oargs: TypedIdents;
                    let mut exptype: metamodelica::Ref<TypeSignature>;
                    let mut argval: (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo);
                    let mut argvals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
                    let mut mcases: metamodelica::List<(metamodelica::Ref<MatchingExp>, (metamodelica::Ref<ExpressionBase>, SourceInfo))>;
                    let mut exp = (*exp).clone();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    (argval, stmts, locals, scEnv, accMMDecls) = statementsFromArg(exp.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (_, exptype, _) = argval.clone();
                    exptype = deAliasedType(&exptype, astDefs.clone());
                    if isTextType(&exptype) {
                        (stmts, locals, argval) = textConditionToIsEmpty(&argval, stmts.clone(), locals.clone())?;
                        exp = emptyExpression.clone();
                    }
                    mcases = elabCasesFromCondition(&exptype, isnot.clone(), rhsval.clone(), tbranch.clone(), ebranch.clone(), metamodelica::AsArg::as_arg(&tplPackage))?;
                    (argvals, fname, iargs, oargs, scEnv, accMMDecls) = makeMatchFun(argval.clone(), mcases.clone(), &(exp.clone()), false, scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (_, stmt, _, _, locals, intxt) = statementFromFun(argvals.clone(), fname.clone(), iargs.clone(), oargs.clone(), metamodelica::nil(), intxt.clone(), outtxt.clone(), locals.clone(), tplPackage.clone(), metamodelica::AsArg::as_arg(&sinfo))?;
                    Ok((metamodelica::cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::MAP { argExp: argexp, ofBinding: ofbind, mapExp: mapexp, hasIndexIdentOpt: idxNmOpt }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut argvals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
                    let mut explst: metamodelica::List<(metamodelica::Ref<ExpressionBase>, SourceInfo)>;
                    let mut mapctx: MapContext;
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    explst = getExpListForMap(argexp.clone());
                    (argvals, stmts, locals, scEnv, accMMDecls) = statementsFromArgList(&explst, stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    mapctx = MapContext { ofBinding: ofbind.clone(), mapExp: mapexp.clone(), iterMMExpOptions: mmopts.clone(), hasIndexIdentOpt: idxNmOpt.clone(), useIter: false };
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromMapExp(true, &argvals, &mapctx, stmts.clone(), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::MAP_ARG_LIST { parts: explst }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut argvals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
                    let mut mapctx: MapContext;
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    (argvals, stmts, locals, scEnv, accMMDecls) = statementsFromArgList(metamodelica::AsArg::as_arg(&explst), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    mapctx = MapContext { ofBinding: metamodelica::Ref::new(MatchingExp::BIND_MATCH { bindIdent: literal!("it") }), mapExp: (metamodelica::Ref::new(ExpressionBase::BOUND_VALUE { boundPath: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("it") }) }), dummySourceInfo.clone()), iterMMExpOptions: mmopts.clone(), hasIndexIdentOpt: None, useIter: false };
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromMapExp(true, &argvals, &mapctx, stmts.clone(), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::ESCAPED { exp, options: opts }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut popstmts: metamodelica::List<metamodelica::Ref<MMExp>>;
                    let mut mmopts = (*mmopts).clone();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    (mmopts, stmts, locals, scEnv, accMMDecls) = statementsFromEscOptions(metamodelica::AsArg::as_arg(&opts), metamodelica::nil(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (mmopts, stmts, popstmts, intxt) = pushPopBlock(mmopts.clone(), arcstr::literal!(absIndentOptionId), literal!("BT_ABS_INDENT"), stmts.clone(), metamodelica::nil(), intxt.clone(), outtxt.clone())?;
                    (mmopts, stmts, popstmts, intxt) = pushPopBlock(mmopts.clone(), arcstr::literal!(indentOptionId), literal!("BT_INDENT"), stmts.clone(), popstmts.clone(), intxt.clone(), outtxt.clone())?;
                    (mmopts, stmts, popstmts, intxt) = pushPopBlock(mmopts.clone(), arcstr::literal!(relIndentOptionId), literal!("BT_REL_INDENT"), stmts.clone(), popstmts.clone(), intxt.clone(), outtxt.clone())?;
                    (mmopts, stmts, popstmts, intxt) = pushPopBlock(mmopts.clone(), arcstr::literal!(anchorOptionId), literal!("BT_ANCHOR"), stmts.clone(), popstmts.clone(), intxt.clone(), outtxt.clone())?;
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromExp(&(exp.clone()), mmopts.clone(), stmts.clone(), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    stmts = listAppend(popstmts.clone(), stmts.clone());
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::INDENTATION { width: n, items: explst }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    let mut istr: ArcStr;
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    istr = intString(n.clone());
                    stmt = pushBlockStatement(literal!("BT_INDENT"), metamodelica::Ref::new(MMExp::MM_LITERAL { value: istr.clone() }), intxt.clone(), outtxt.clone());
                    (stmts, locals, scEnv, accMMDecls, _) = statementsFromExpList(metamodelica::AsArg::as_arg(&explst), metamodelica::cons(stmt.clone(), stmts.clone()), outtxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    stmt = tplStatement(literal!("popBlock"), metamodelica::nil(), outtxt.clone(), outtxt.clone());
                    Ok((metamodelica::cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::LET { letExp: (Deref @ ExpressionBase::TEXT_CREATE { name: ident, exp: txtexp }, _), exp }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut encIdent: Ident;
                    let mut letOuttxt: Ident;
                    let mut freshIdent: Ident;
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n TEXT_CREATE ident = ")); __mm_s.push_str(&*ident); ArcStr::from(__mm_s) })?;
                    }
                    encIdent = encodeIdent(ident.clone(), &(arcstr::literal!(letValueNamePrefix)))?;
                    (freshIdent, locals) = updateLocalsForLetExp(metamodelica::AsArg::as_arg(&ident), &encIdent, 0, &(crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE()), metamodelica::AsArg::as_arg(&locals), metamodelica::AsArg::as_arg(&scEnv))?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(statementsFromExp(&(txtexp.clone()), metamodelica::nil(), stmts.clone(), arcstr::literal!(emptyTxt), freshIdent.clone(), locals.clone(), metamodelica::cons(Scope::RECURSIVE_SCOPE { recIdent: ident.clone(), freshIdent: freshIdent.clone() }, scEnv.clone()), tplPackage.clone(), accMMDecls.clone())?) {
                        (__pa0, __pa1, Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa2 }, __pa3, __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    stmts = metamodelica::Own::own(__pa0);
                    locals = metamodelica::Own::own(__pa1);
                    scEnv = metamodelica::Own::own(__pa2);
                    accMMDecls = metamodelica::Own::own(__pa3);
                    letOuttxt = metamodelica::Own::own(__pa4);
                    stmts = if (metamodelica::stringEq(&letOuttxt, &arcstr::literal!(emptyTxt))) {metamodelica::cons(metamodelica::Ref::new(MMExp::MM_ASSIGN { lhsArgs: list![freshIdent.clone()], rhs: metamodelica::Ref::new(MMExp::MM_IDENT { ident: metamodelica::Ref::new(PathIdent::IDENT { ident: arcstr::literal!(emptyTxt) }) }) }), stmts.clone())} else {stmts.clone()};
                    scEnv = metamodelica::cons(Scope::LET_SCOPE { ident: ident.clone(), idType: crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE(), freshIdent: freshIdent.clone(), isUsed: false }, scEnv.clone());
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromExp(&(exp.clone()), metamodelica::nil(), stmts.clone(), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    let __pa5 = ::match_deref::match_deref! { match &(scEnv.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Scope::LET_SCOPE { .. }, tail: __pa5 } => __pa5.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    scEnv = metamodelica::Own::own(__pa5);
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::LET { letExp: (Deref @ ExpressionBase::TEXT_ADD { name: ident, exp: txtexp }, sinfo2), exp }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut mmexp: metamodelica::Ref<MMExp>;
                    let mut encIdent: Ident;
                    let mut path: metamodelica::Ref<PathIdent>;
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    path = metamodelica::Ref::new(PathIdent::IDENT { ident: ident.clone() });
                    (mmexp, idtype, scEnv) = resolveBoundPath(path.clone(), scEnv.clone(), metamodelica::AsArg::as_arg(&tplPackage))?;
                    checkResolvedType(&path, &idtype, literal!("let +="), metamodelica::AsArg::as_arg(&sinfo2));
                    idtype = checkTextType(idtype.clone(), metamodelica::AsArg::as_arg(&ident), literal!("let +="), metamodelica::AsArg::as_arg(&sinfo2))?;
                    let __pa0 = ::match_deref::match_deref! { match &(mmexp.clone()) {
                        Deref @ MMExp::MM_IDENT { ident: Deref @ PathIdent::IDENT { ident: __pa0 } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    encIdent = metamodelica::Own::own(__pa0);
                    scEnv = metamodelica::cons(Scope::RECURSIVE_SCOPE { recIdent: ident.clone(), freshIdent: encIdent.clone() }, scEnv.clone());
                    (stmts, locals, scEnv, accMMDecls, _) = statementsFromExp(&(txtexp.clone()), metamodelica::nil(), stmts.clone(), encIdent.clone(), encIdent.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    let __pa2 = ::match_deref::match_deref! { match &(scEnv.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Scope::RECURSIVE_SCOPE { .. }, tail: __pa2 } => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    scEnv = metamodelica::Own::own(__pa2);
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromExp(&(exp.clone()), metamodelica::nil(), stmts.clone(), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::LET { letExp: (Deref @ ExpressionBase::NORET_CALL { name: fname, args: explst }, sinfo2), exp }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage @ TemplPackage { .. }, accMMDecls) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    let mut tyVars: metamodelica::List<ArcStr>;
                    let mut iargs: TypedIdents;
                    let mut oargs: TypedIdents;
                    let mut argvals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
                    let mut fname = (*fname).clone();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n NORET_CALL fname = ")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&fname))?); ArcStr::from(__mm_s) })?;
                    }
                    (fname, iargs, oargs, tyVars) = getFunSignature(fname.clone(), metamodelica::AsArg::as_arg(&sinfo2), metamodelica::AsArg::as_arg(&tplPackage))?;
                    ::match_deref::match_deref! { match &(oargs.clone()) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    (argvals, stmts, locals, scEnv, accMMDecls) = statementsFromArgList(metamodelica::AsArg::as_arg(&explst), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!(" NORET_CALL argList stmts generation passed.\n"))?;
                    }
                    (_, stmt, _, _, locals, intxt) = statementFromFun(argvals.clone(), fname.clone(), iargs.clone(), oargs.clone(), tyVars.clone(), intxt.clone(), outtxt.clone(), locals.clone(), tplPackage.clone(), metamodelica::AsArg::as_arg(&sinfo2))?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" NORET_CALL stmt =\n")); __mm_s.push_str(&*stmtsString(&(list![stmt.clone()]))?); ArcStr::from(__mm_s) })?;
                    }
                    stmts = metamodelica::cons(stmt.clone(), stmts.clone());
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromExp(&(exp.clone()), metamodelica::nil(), stmts.clone(), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::LET { letExp: (Deref @ ExpressionBase::NORET_CALL { name: fname, .. }, sinfo2), .. }, _), _, _, _, _, _, _, tplPackage @ TemplPackage { .. }, _) => {
                    let mut oargs: TypedIdents;
                    let mut fname = (*fname).clone();
                    (fname, _, oargs, _) = getFunSignature(fname.clone(), metamodelica::AsArg::as_arg(&sinfo2), metamodelica::AsArg::as_arg(&tplPackage))?;
                    ::match_deref::match_deref! { match &(oargs.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - NORET_CALL with a '")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&fname))?); __mm_s.push_str(&*literal!("' template or function that has output argument(s).\n")); ArcStr::from(__mm_s) })?;
                    }
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
                    Debug::trace(literal!("-!!!statementsFromExp failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outStmts, outLocals, outScopeEnv, outMMDecls, outInText))
}

pub(crate) fn statementsFromExpList(
    mut inExpLst: &metamodelica::List<(metamodelica::Ref<ExpressionBase>, SourceInfo)>,
    mut inStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inInText: Ident,
    mut inOutText: Ident,
    mut inLocals: TypedIdents,
    mut inScopeEnv: ScopeEnv,
    mut inTplPackage: TemplPackage,
    mut inAccMMDecls: metamodelica::List<MMDeclaration>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<MMExp>>,
    TypedIdents,
    ScopeEnv,
    metamodelica::List<MMDeclaration>,
    Ident,
)> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outLocals: TypedIdents;
    let mut outScopeEnv: ScopeEnv;
    let mut outMMDecls: metamodelica::List<MMDeclaration>;
    let mut outInText: Ident;
    (outStmts, outLocals, outScopeEnv, outMMDecls, outInText) = 'mc: {
        let __mc_input = (
            &**inExpLst,
            inStmts,
            inInText,
            inOutText,
            inLocals,
            inScopeEnv,
            inTplPackage,
            inAccMMDecls,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, stmts, intxt, _, locals, scEnv, _, accMMDecls) => {
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: exp, tail: explst }, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromExp(&(exp.clone()), metamodelica::nil(), stmts.clone(), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromExpList(metamodelica::AsArg::as_arg(&explst), stmts.clone(), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
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
                    Debug::trace(literal!("-!!!statementsFromExpList failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outStmts, outLocals, outScopeEnv, outMMDecls, outInText))
}

pub(crate) fn warnIfSomeOptions(
    mut inMMEscOptions: &metamodelica::List<(ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>))>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inMMEscOptions;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (optid, _), tail: _ } => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - more options specified than expected for an expression (first option is '")); __mm_s.push_str(&*optid); __mm_s.push_str(&*literal!("').\n")); ArcStr::from(__mm_s) })?;
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
                    Debug::trace(literal!("- warnIfSomeOptions failed.\n"))?;
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

pub(crate) fn statementsFromEscOptions(
    mut inOptions: &metamodelica::List<(ArcStr, Option<(metamodelica::Ref<ExpressionBase>, SourceInfo)>)>,
    mut inAccMMEscOptions: metamodelica::List<(ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>))>,
    mut inStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inLocals: TypedIdents,
    mut inScopeEnv: ScopeEnv,
    mut inTplPackage: TemplPackage,
    mut inAccMMDecls: metamodelica::List<MMDeclaration>,
) -> Result<(
    metamodelica::List<(ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>))>,
    metamodelica::List<metamodelica::Ref<MMExp>>,
    TypedIdents,
    ScopeEnv,
    metamodelica::List<MMDeclaration>,
)> {
    let mut outAccMMEscOptions: metamodelica::List<(
        ArcStr,
        (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>),
    )>;
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outLocals: TypedIdents;
    let mut outScopeEnv: ScopeEnv;
    let mut outMMDecls: metamodelica::List<MMDeclaration>;
    (outAccMMEscOptions, outStmts, outLocals, outScopeEnv, outMMDecls) = 'mc: {
        let __mc_input = (
            &**inOptions,
            inAccMMEscOptions,
            inStmts,
            inLocals,
            inScopeEnv,
            inTplPackage,
            inAccMMDecls,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, accMMEscOpts, stmts, locals, scEnv, _, accMMDecls) => {
                    Ok((accMMEscOpts.clone(), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (optid, None), tail: opts }, accMMEscOpts, stmts, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut defoptval: (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>);
                    let mut accMMEscOpts = (*accMMEscOpts).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    defoptval = lookupTupleList(defaultEscOptions.clone(), optid.clone())?;
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(accMMEscOpts.clone(), optid.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    (accMMEscOpts, stmts, locals, scEnv, accMMDecls) = statementsFromEscOptions(metamodelica::AsArg::as_arg(&opts), metamodelica::cons((optid.clone(), defoptval.clone()), accMMEscOpts.clone()), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((accMMEscOpts.clone(), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (optid, Some(optexp)), tail: opts }, accMMEscOpts, stmts, locals, scEnv, tplPackage @ TemplPackage { astDefs: astdefs, .. }, accMMDecls) => {
                    let mut mmarg: metamodelica::Ref<MMExp>;
                    let mut exptype: metamodelica::Ref<TypeSignature>;
                    let mut opttype: metamodelica::Ref<TypeSignature>;
                    let mut sinfo: SourceInfo;
                    let mut accMMEscOpts = (*accMMEscOpts).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    (_, opttype) = lookupTupleList(defaultEscOptions.clone(), optid.clone())?;
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(accMMEscOpts.clone(), optid.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    let ((__pa1, __pa2, __pa3), __pa4, __pa5, __pa6, __pa7) = statementsFromArg(optexp.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    mmarg = metamodelica::Own::own(__pa1);
                    exptype = metamodelica::Own::own(__pa2);
                    sinfo = metamodelica::Own::own(__pa3);
                    stmts = metamodelica::Own::own(__pa4);
                    locals = metamodelica::Own::own(__pa5);
                    scEnv = metamodelica::Own::own(__pa6);
                    accMMDecls = metamodelica::Own::own(__pa7);
                    (mmarg, stmts, locals) = typeAdaptMMOption(mmarg.clone(), exptype.clone(), &sinfo, opttype.clone(), stmts.clone(), locals.clone(), astdefs.clone())?;
                    (accMMEscOpts, stmts, locals, scEnv, accMMDecls) = statementsFromEscOptions(metamodelica::AsArg::as_arg(&opts), metamodelica::cons((optid.clone(), (mmarg.clone(), opttype.clone())), accMMEscOpts.clone()), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((accMMEscOpts.clone(), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (optid, _), tail: opts }, accMMEscOpts, stmts, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut accMMEscOpts = (*accMMEscOpts).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(defaultEscOptions.clone(), optid.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - an unknown option'")); __mm_s.push_str(&*optid); __mm_s.push_str(&*literal!("' was specified. \n")); ArcStr::from(__mm_s) })?;
                    (accMMEscOpts, stmts, locals, scEnv, accMMDecls) = statementsFromEscOptions(metamodelica::AsArg::as_arg(&opts), accMMEscOpts.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (optid, _), tail: opts }, accMMEscOpts, stmts, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut accMMEscOpts = (*accMMEscOpts).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    lookupTupleList(defaultEscOptions.clone(), optid.clone())?;
                    lookupTupleList(accMMEscOpts.clone(), optid.clone())?;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Warning - a duplicit option'")); __mm_s.push_str(&*optid); __mm_s.push_str(&*literal!("' was specified. It will be ignored (not evaluated).\n")); ArcStr::from(__mm_s) })?;
                    (accMMEscOpts, stmts, locals, scEnv, accMMDecls) = statementsFromEscOptions(metamodelica::AsArg::as_arg(&opts), accMMEscOpts.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((accMMEscOpts.clone(), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
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
                    Debug::trace(literal!(" -statementsFromEscOptions failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outAccMMEscOptions, outStmts, outLocals, outScopeEnv, outMMDecls))
}

pub(crate) fn getExpListForMap(
    mut inExp: Expression,
) -> metamodelica::List<(metamodelica::Ref<ExpressionBase>, SourceInfo)> {
    let mut outExpsForMap: metamodelica::List<(metamodelica::Ref<ExpressionBase>, SourceInfo)>;
    outExpsForMap = (::match_deref::match_deref! { match &(inExp.clone()) {
        (Deref @ ExpressionBase::MAP_ARG_LIST { parts: explst }, _) => {
            explst.clone()
        },
        _ => {
            list![inExp]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExpsForMap
}

pub(crate) fn pushPopBlock(
    mut inMMEscOptions: metamodelica::List<(ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>))>,
    mut inOptionIdent: Ident,
    mut inBlockTypeIdent: Ident,
    mut inStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inPopBlockStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inInText: Ident,
    mut inOutText: Ident,
) -> Result<(
    metamodelica::List<(ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>))>,
    metamodelica::List<metamodelica::Ref<MMExp>>,
    metamodelica::List<metamodelica::Ref<MMExp>>,
    Ident,
)> {
    let mut outMMEscOptions: metamodelica::List<(
        ArcStr,
        (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>),
    )>;
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outPopBlockStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outInText: Ident;
    (outMMEscOptions, outStmts, outPopBlockStmts, outInText) = 'mc: {
        let __mc_input = (
            inMMEscOptions,
            inOptionIdent,
            inBlockTypeIdent,
            inStmts,
            inPopBlockStmts,
            inInText,
            inOutText,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmopts, optid, btid, stmts, popstmts, intxt, outtxt) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    let mut pstmt: metamodelica::Ref<MMExp>;
                    let mut mmexp: metamodelica::Ref<MMExp>;
                    let mut mmopts = (*mmopts).clone();
                    let mut popstmts = (*popstmts).clone();
                    let ((__pa0, _), __pa1) = lookupDeleteTupleList(metamodelica::AsArg::as_arg(&mmopts), optid.clone())?;
                    mmexp = metamodelica::Own::own(__pa0);
                    mmopts = metamodelica::Own::own(__pa1);
                    stmt = pushBlockStatement(btid.clone(), mmexp.clone(), intxt.clone(), outtxt.clone());
                    pstmt = tplStatement(literal!("popBlock"), metamodelica::nil(), outtxt.clone(), outtxt.clone());
                    popstmts = List::appendElt(pstmt.clone(), popstmts.clone());
                    Ok((mmopts.clone(), metamodelica::cons(stmt.clone(), stmts.clone()), popstmts.clone(), outtxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmopts, _, _, stmts, popstmts, intxt, _) => {
                    Ok((mmopts.clone(), stmts.clone(), popstmts.clone(), intxt.clone()))
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
                    Debug::trace(literal!("-!!!pushPopBlock failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outMMEscOptions, outStmts, outPopBlockStmts, outInText))
}

/*
public function addImplicitArgument
  input list<Expression> inArgLst;
  input TypedIdents inInArgs;
  input TypedIdents inOutArgs;
  input TemplPackage inTplPackage;

  output list<Expression> outArgLst;
algorithm
  outArgLst := matchcontinue (inArgLst, inInArgs, inOutArgs, inTplPackage)
    local
      list<Expression> explst;
      tuple<Ident,TypeSignature> iarg, oarg;
      TemplPackage tplPackage;

    //when the function is a template function
    //and the signature has the only one argument and none is specified on call
    // assume the 'it'
    case ( {}, { iarg, _ }, oarg :: _ , tplPackage)
      algorithm
        areTextInOutArgs(iarg, oarg, tplPackage);
      then { BOUND_VALUE(IDENT("it")) };

    //when the function is a non-template function
    //and the signature has the only one argument and none is specified on the call
    // assume the 'it'
    //- case with an output argument (check if it is not a template function with no argument - i.e. only one text input argument)
    case ( {}, { iarg }, oarg :: _ , tplPackage)
      algorithm
        failure(areTextInOutArgs(iarg, oarg, tplPackage));
      then { BOUND_VALUE(IDENT("it")) };

    //when the function is a non-template function
    //and the signature has the only one argument and none is specified on the call
    // assume the 'it'
    //- case with no output argument (evidently a no-ret non-template function)
    case ( {}, { iarg }, {} , tplPackage)
      then { BOUND_VALUE(IDENT("it")) };


    //otherwise no change
    else inArgLst;

  end matchcontinue;
end addImplicitArgument;
*/
pub(crate) fn statementsFromArg(
    mut inExp: Expression,
    mut inStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inLocals: TypedIdents,
    mut inScopeEnv: ScopeEnv,
    mut inTplPackage: TemplPackage,
    mut inAccMMDecls: metamodelica::List<MMDeclaration>,
) -> Result<(
    (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo),
    metamodelica::List<metamodelica::Ref<MMExp>>,
    TypedIdents,
    ScopeEnv,
    metamodelica::List<MMDeclaration>,
)> {
    let mut outArgValue: (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo);
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outLocals: TypedIdents;
    let mut outScopeEnv: ScopeEnv;
    let mut outMMDecls: metamodelica::List<MMDeclaration>;
    (outArgValue, outStmts, outLocals, outScopeEnv, outMMDecls) = 'mc: {
        let __mc_input = (inExp, inStmts, inLocals, inScopeEnv, inTplPackage, inAccMMDecls);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::LITERAL { value: litvalue, litType: littype }, sinfo), stmts, locals, scEnv, _, accMMDecls) => {
                    Ok(((metamodelica::Ref::new(MMExp::MM_LITERAL { value: litvalue.clone() }), littype.clone(), sinfo.clone()), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::STR_TOKEN { value: st }, sinfo), stmts, locals, scEnv, _, accMMDecls) => {
                    Ok(((metamodelica::Ref::new(MMExp::MM_STR_TOKEN { value: st.clone() }), crate::TplAbsyn::TypeSignature::interned_STRING_TOKEN_TYPE(), sinfo.clone()), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::BOUND_VALUE { boundPath: path }, sinfo), stmts, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut mmexp: metamodelica::Ref<MMExp>;
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut scEnv = (*scEnv).clone();
                    (mmexp, idtype, scEnv) = resolveBoundPath(path.clone(), scEnv.clone(), metamodelica::AsArg::as_arg(&tplPackage))?;
                    checkResolvedType(metamodelica::AsArg::as_arg(&path), &idtype, literal!("argument"), metamodelica::AsArg::as_arg(&sinfo));
                    Ok(((mmexp.clone(), idtype.clone(), sinfo.clone()), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::FUN_CALL { name: Deref @ PathIdent::IDENT { ident: Deref @ "sourceInfo" }, args: Deref @ metamodelica::ListNode::Nil }, sinfo @ SourceInfo { fileName, lineNumberStart, columnNumberStart, .. }), stmts, locals, scEnv, _, accMMDecls) => {
                    let mut mmexp: metamodelica::Ref<MMExp>;
                    let mut fname: metamodelica::Ref<PathIdent>;
                    let mut rettype: metamodelica::Ref<TypeSignature>;
                    let mut lineStr: ArcStr;
                    let mut colStr: ArcStr;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!(" arg sourceInfo \n"))?;
                    }
                    fname = metamodelica::Ref::new(PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("sourceInfo") }) });
                    rettype = metamodelica::Ref::new(TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(PathIdent::PATH_IDENT { ident: literal!("builtin"), path: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("SourceInfo") }) }) });
                    lineStr = intString(lineNumberStart.clone());
                    colStr = intString(columnNumberStart.clone());
                    mmexp = metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: fname.clone(), args: list![metamodelica::Ref::new(MMExp::MM_STRING { value: fileName.clone() }), metamodelica::Ref::new(MMExp::MM_LITERAL { value: lineStr.clone() }), metamodelica::Ref::new(MMExp::MM_LITERAL { value: colStr.clone() })] });
                    Ok(((mmexp.clone(), rettype.clone(), sinfo.clone()), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::FUN_CALL { name: fname, args: explst }, sinfo), stmts, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    let mut outtxt: Ident;
                    let mut tyVars: metamodelica::List<ArcStr>;
                    let mut argvals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
                    let mut mmexp: metamodelica::Ref<MMExp>;
                    let mut rettype: metamodelica::Ref<TypeSignature>;
                    let mut iargs: TypedIdents;
                    let mut oargs: TypedIdents;
                    let mut fname = (*fname).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    (fname, iargs, oargs, tyVars) = getFunSignature(fname.clone(), metamodelica::AsArg::as_arg(&sinfo), metamodelica::AsArg::as_arg(&tplPackage))?;
                    (argvals, stmts, locals, scEnv, accMMDecls) = statementsFromArgList(metamodelica::AsArg::as_arg(&explst), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    outtxt = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(textTempVarNamePrefix)); __mm_s.push_str(&*intString(((locals).len() as i32))); ArcStr::from(__mm_s) };
                    (_, stmt, mmexp, rettype, locals, outtxt) = statementFromFun(argvals.clone(), fname.clone(), iargs.clone(), oargs.clone(), tyVars.clone(), arcstr::literal!(emptyTxt), outtxt.clone(), locals.clone(), tplPackage.clone(), metamodelica::AsArg::as_arg(&sinfo))?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" arg FUN_CALL stmt =\n")); __mm_s.push_str(&*stmtsString(&(list![stmt.clone()]))?); ArcStr::from(__mm_s) })?;
                    }
                    locals = addLocalValue(outtxt.clone(), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE(), locals.clone())?;
                    Ok(((mmexp.clone(), rettype.clone(), sinfo.clone()), metamodelica::cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ (_, sinfo), stmts, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut outtxt: Ident;
                    let mut mmexp: metamodelica::Ref<MMExp>;
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    outtxt = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(textTempVarNamePrefix)); __mm_s.push_str(&*intString(((locals).len() as i32))); ArcStr::from(__mm_s) };
                    (stmts, locals, scEnv, accMMDecls, outtxt) = statementsFromExp(&(exp.clone()), metamodelica::nil(), stmts.clone(), arcstr::literal!(emptyTxt), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    locals = addLocalValue(outtxt.clone(), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE(), locals.clone())?;
                    mmexp = metamodelica::Ref::new(MMExp::MM_IDENT { ident: metamodelica::Ref::new(PathIdent::IDENT { ident: outtxt.clone() }) });
                    Ok(((mmexp.clone(), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE(), sinfo.clone()), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
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
                    Debug::trace(literal!("-!!!statementsFromArg failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outArgValue, outStmts, outLocals, outScopeEnv, outMMDecls))
}

pub(crate) fn statementsFromArgList(
    mut inExpLst: &metamodelica::List<(metamodelica::Ref<ExpressionBase>, SourceInfo)>,
    mut inStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inLocals: TypedIdents,
    mut inScopeEnv: ScopeEnv,
    mut inTplPackage: TemplPackage,
    mut inAccMMDecls: metamodelica::List<MMDeclaration>,
) -> Result<(
    metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>,
    metamodelica::List<metamodelica::Ref<MMExp>>,
    TypedIdents,
    ScopeEnv,
    metamodelica::List<MMDeclaration>,
)> {
    let mut outArgValues: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outLocals: TypedIdents;
    let mut outScopeEnv: ScopeEnv;
    let mut outMMDecls: metamodelica::List<MMDeclaration>;
    (outArgValues, outStmts, outLocals, outScopeEnv, outMMDecls) = 'mc: {
        let __mc_input = (&**inExpLst, inStmts, inLocals, inScopeEnv, inTplPackage, inAccMMDecls);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, stmts, locals, scEnv, _, accMMDecls) => {
                    Ok((metamodelica::nil(), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: exp, tail: explst }, stmts, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut argval: (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo);
                    let mut argvals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    (argval, stmts, locals, scEnv, accMMDecls) = statementsFromArg(exp.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (argvals, stmts, locals, scEnv, accMMDecls) = statementsFromArgList(metamodelica::AsArg::as_arg(&explst), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((metamodelica::cons(argval.clone(), argvals.clone()), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
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
                    Debug::trace(literal!("-!!!statementsFromArgList failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outArgValues, outStmts, outLocals, outScopeEnv, outMMDecls))
}

pub(crate) fn tplStatement(
    mut inFunName: Ident,
    mut inArgs: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inInText: Ident,
    mut inOutArg: Ident,
) -> metamodelica::Ref<MMExp> {
    let mut outStmt: metamodelica::Ref<MMExp>;
    outStmt = metamodelica::Ref::new(MMExp::MM_ASSIGN {
        lhsArgs: list![inOutArg],
        rhs: metamodelica::Ref::new(MMExp::MM_FN_CALL {
            fnName: metamodelica::Ref::new(PathIdent::PATH_IDENT {
                ident: literal!("Tpl"),
                path: metamodelica::Ref::new(PathIdent::IDENT { ident: inFunName }),
            }),
            args: metamodelica::cons(
                metamodelica::Ref::new(MMExp::MM_IDENT {
                    ident: metamodelica::Ref::new(PathIdent::IDENT { ident: inInText }),
                }),
                inArgs,
            ),
        }),
    });
    outStmt
}

pub(crate) fn pushBlockStatement(
    mut inBlockType: Ident,
    mut inArg: metamodelica::Ref<MMExp>,
    mut inInText: Ident,
    mut inOutArg: Ident,
) -> metamodelica::Ref<MMExp> {
    let mut outStmt: metamodelica::Ref<MMExp>;
    outStmt = metamodelica::Ref::new(MMExp::MM_ASSIGN {
        lhsArgs: list![inOutArg],
        rhs: metamodelica::Ref::new(MMExp::MM_FN_CALL {
            fnName: metamodelica::Ref::new(PathIdent::PATH_IDENT {
                ident: literal!("Tpl"),
                path: metamodelica::Ref::new(PathIdent::IDENT {
                    ident: literal!("pushBlock"),
                }),
            }),
            args: list![
                metamodelica::Ref::new(MMExp::MM_IDENT {
                    ident: metamodelica::Ref::new(PathIdent::IDENT { ident: inInText })
                }),
                metamodelica::Ref::new(MMExp::MM_FN_CALL {
                    fnName: metamodelica::Ref::new(PathIdent::PATH_IDENT {
                        ident: literal!("Tpl"),
                        path: metamodelica::Ref::new(PathIdent::IDENT { ident: inBlockType })
                    }),
                    args: list![inArg]
                })
            ],
        }),
    });
    outStmt
}

pub(crate) fn addWriteCallFromMMExp(
    mut inHasRetValue: bool,
    mut inMMExp: metamodelica::Ref<MMExp>,
    mut inType: metamodelica::Ref<TypeSignature>,
    mut inSourceInfo: SourceInfo,
    mut inMMEscOptions: metamodelica::List<(ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>))>,
    mut inStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inInText: Ident,
    mut inOutText: Ident,
    mut inLocals: TypedIdents,
    mut inScopeEnv: ScopeEnv,
    mut inTplPackage: TemplPackage,
    mut inAccMMDecls: metamodelica::List<MMDeclaration>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<MMExp>>,
    TypedIdents,
    ScopeEnv,
    metamodelica::List<MMDeclaration>,
    Ident,
)> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outLocals: TypedIdents;
    let mut outScopeEnv: ScopeEnv;
    let mut outMMDecls: metamodelica::List<MMDeclaration>;
    let mut outInText: Ident;
    (outStmts, outLocals, outScopeEnv, outMMDecls, outInText) = 'mc: {
        let __mc_input = (
            inHasRetValue,
            inMMExp,
            inType,
            inMMEscOptions,
            inStmts,
            inInText,
            inOutText,
            inLocals,
            inScopeEnv,
            inTplPackage,
            inAccMMDecls,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, _, _, _, stmts, intxt, _, locals, scEnv, _, accMMDecls) => {
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, mmexp, exptype @ Deref @ TypeSignature::OPTION_TYPE { .. }, mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut fname: metamodelica::Ref<PathIdent>;
                    let mut stmt: metamodelica::Ref<MMExp>;
                    let mut iargs: TypedIdents;
                    let mut oargs: TypedIdents;
                    let mut argvals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    (argvals, fname, iargs, oargs, scEnv, accMMDecls) = makeMatchFun((mmexp.clone(), exptype.clone(), inSourceInfo.clone()), list![(metamodelica::Ref::new(MatchingExp::SOME_MATCH { value: metamodelica::Ref::new(MatchingExp::BIND_MATCH { bindIdent: literal!("val") }) }), (metamodelica::Ref::new(ExpressionBase::BOUND_VALUE { boundPath: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("val") }) }), dummySourceInfo.clone()))], &(emptyExpression.clone()), true, scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (_, stmt, _, _, locals, intxt) = statementFromFun(argvals.clone(), fname.clone(), iargs.clone(), oargs.clone(), metamodelica::nil(), intxt.clone(), outtxt.clone(), locals.clone(), tplPackage.clone(), &inSourceInfo)?;
                    Ok((metamodelica::cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, mmexp, exptype @ Deref @ TypeSignature::LIST_TYPE { .. }, mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut mapctx: MapContext;
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    mapctx = MapContext { ofBinding: metamodelica::Ref::new(MatchingExp::BIND_MATCH { bindIdent: literal!("it") }), mapExp: (metamodelica::Ref::new(ExpressionBase::BOUND_VALUE { boundPath: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("it") }) }), dummySourceInfo.clone()), iterMMExpOptions: mmopts.clone(), hasIndexIdentOpt: None, useIter: false };
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromMapExp(true, &(list![(mmexp.clone(), exptype.clone(), inSourceInfo.clone())]), &mapctx, stmts.clone(), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, mmexp, Deref @ TypeSignature::STRING_TOKEN_TYPE { .. }, mmopts, stmts, intxt, outtxt, locals, scEnv, _, accMMDecls) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    stmt = tplStatement(literal!("writeTok"), list![mmexp.clone()], intxt.clone(), outtxt.clone());
                    Ok((metamodelica::cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, mmexp, Deref @ TypeSignature::TEXT_TYPE { .. }, mmopts, stmts, intxt, outtxt, locals, scEnv, _, accMMDecls) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    stmt = tplStatement(literal!("writeText"), list![mmexp.clone()], intxt.clone(), outtxt.clone());
                    Ok((metamodelica::cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, mmexp, exptype, mmopts, stmts, intxt, outtxt, locals, scEnv, _, accMMDecls) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    let mut mmexp = (*mmexp).clone();
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&mmopts))?;
                    mmexp = mmExpToString(mmexp.clone(), exptype.clone(), &inSourceInfo)?;
                    stmt = tplStatement(literal!("writeStr"), list![mmexp.clone()], intxt.clone(), outtxt.clone());
                    Ok((metamodelica::cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
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
                    Debug::trace(literal!("-!!!addWriteCallFromMMExp failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outStmts, outLocals, outScopeEnv, outMMDecls, outInText))
}

//no fail
pub(crate) fn mmExpToString(
    mut inMMExp: metamodelica::Ref<MMExp>,
    mut inType: metamodelica::Ref<TypeSignature>,
    mut inSourceInfo: &SourceInfo,
) -> Result<metamodelica::Ref<MMExp>> {
    let mut outMMExp: metamodelica::Ref<MMExp>;
    outMMExp = 'mc: {
        let __mc_input = (inMMExp, inType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, Deref @ TypeSignature::STRING_TYPE { .. }) => {
                    Ok(mmexp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MMExp::MM_LITERAL { value: r#str }, _) => {
                    Ok(metamodelica::Ref::new(MMExp::MM_STRING { value: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MMExp::MM_STR_TOKEN { value: st }, _) => {
                    let mut r#str: ArcStr;
                    r#str = Tpl::strTokString(st.clone())?;
                    Ok(metamodelica::Ref::new(MMExp::MM_STRING { value: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, Deref @ TypeSignature::STRING_TOKEN_TYPE { .. }) => {
                    Ok(metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("strTokString") }) }), args: list![mmexp.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, Deref @ TypeSignature::TEXT_TYPE { .. }) => {
                    Ok(metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("textString") }) }), args: list![mmexp.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, Deref @ TypeSignature::INTEGER_TYPE { .. }) => {
                    Ok(metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("intString") }), args: list![mmexp.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, Deref @ TypeSignature::REAL_TYPE { .. }) => {
                    Ok(metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("realString") }), args: list![mmexp.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, Deref @ TypeSignature::BOOLEAN_TYPE { .. }) => {
                    Ok(metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("booleanString") }) }), args: list![mmexp.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, Deref @ TypeSignature::UNRESOLVED_TYPE { reason }) => {
                    let mut reason = (*reason).clone();
                    reason = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#UnresType# ")); __mm_s.push_str(&*reason); __mm_s.push_str(&*literal!(" #")); ArcStr::from(__mm_s) };
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - an unresolved value trying to convert to string. Unresolution reason:\n    ")); __mm_s.push_str(&*reason); ArcStr::from(__mm_s) })?;
                    }
                    Ok(metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(PathIdent::IDENT { ident: reason.clone() }), args: list![mmexp.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, ts) => {
                    let mut reason: ArcStr;
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Elaborated expression '")); __mm_s.push_str(&*mmExpString(mmexp.clone())?); __mm_s.push_str(&*literal!("' of type '")); __mm_s.push_str(&*typeSignatureString(metamodelica::AsArg::as_arg(&ts))?); __mm_s.push_str(&*literal!("' has no automatic to-string conversion.")); ArcStr::from(__mm_s) };
                    addSusanError(r#str.clone(), inSourceInfo)?;
                    reason = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error# ")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(" #")); ArcStr::from(__mm_s) };
                    Ok(metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(PathIdent::IDENT { ident: reason.clone() }), args: list![mmexp.clone()] }))
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
                    Debug::trace(literal!("-!!!mmExpToString failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMMExp)
}

pub(crate) fn statementFromFun(
    mut inArgValues: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>,
    mut inFunName: metamodelica::Ref<PathIdent>,
    mut inInArgs: TypedIdents,
    mut inOutArgs: TypedIdents,
    mut inTypeVars: metamodelica::List<ArcStr>,
    mut inInText: Ident,
    mut inOutText: Ident,
    mut inLocals: TypedIdents,
    mut inTplPackage: TemplPackage,
    mut inInfo: &SourceInfo,
) -> Result<(
    bool,
    metamodelica::Ref<MMExp>,
    metamodelica::Ref<MMExp>,
    metamodelica::Ref<TypeSignature>,
    TypedIdents,
    Ident,
)> {
    let mut outHasRetValue: bool;
    let mut outStmt: metamodelica::Ref<MMExp>;
    let mut outRetMMExp: metamodelica::Ref<MMExp>;
    let mut outRetType: metamodelica::Ref<TypeSignature>;
    let mut outLocals: TypedIdents;
    let mut outOutText: Ident;
    (outHasRetValue, outStmt, outRetMMExp, outRetType, outLocals, outOutText) = 'mc: {
        let __mc_input = (
            inArgValues,
            inFunName,
            inInArgs,
            inOutArgs,
            inTypeVars,
            inInText,
            inOutText,
            inLocals,
            inTplPackage,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argvals, fname, Deref @ metamodelica::ListNode::Cons { head: iarg, tail: iargs }, Deref @ metamodelica::ListNode::Cons { head: oarg, tail: Deref @ metamodelica::ListNode::Nil }, tyVars, intxt, outtxt, locals, tplPackage @ TemplPackage { astDefs, .. }) => {
                    let mut mmargs: metamodelica::List<metamodelica::Ref<MMExp>>;
                    let mut mmexp: metamodelica::Ref<MMExp>;
                    let mut mmtxt: metamodelica::Ref<MMExp>;
                    areTextInOutArgs(&(iarg.clone()), &(oarg.clone()), metamodelica::AsArg::as_arg(&tplPackage))?;
                    (mmargs, _) = typeAdaptMMArgsForFun(metamodelica::AsArg::as_arg(&argvals), metamodelica::AsArg::as_arg(&iargs), tyVars.clone(), metamodelica::nil(), astDefs.clone())?;
                    mmtxt = metamodelica::Ref::new(MMExp::MM_IDENT { ident: metamodelica::Ref::new(PathIdent::IDENT { ident: outtxt.clone() }) });
                    mmexp = metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: fname.clone(), args: metamodelica::cons(metamodelica::Ref::new(MMExp::MM_IDENT { ident: metamodelica::Ref::new(PathIdent::IDENT { ident: intxt.clone() }) }), mmargs.clone()) });
                    Ok((false, metamodelica::Ref::new(MMExp::MM_ASSIGN { lhsArgs: list![outtxt.clone()], rhs: mmexp.clone() }), mmtxt.clone(), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE(), locals.clone(), outtxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argvals, fname, Deref @ metamodelica::ListNode::Cons { head: iarg, tail: iargs }, Deref @ metamodelica::ListNode::Cons { head: oarg, tail: oargs @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, tyVars, intxt, outtxt, locals, tplPackage @ TemplPackage { astDefs, .. }) => {
                    let mut mmargs: metamodelica::List<metamodelica::Ref<MMExp>>;
                    let mut lhsArgs: metamodelica::List<ArcStr>;
                    let mut mmexp: metamodelica::Ref<MMExp>;
                    let mut mmtxt: metamodelica::Ref<MMExp>;
                    areTextInOutArgs(&(iarg.clone()), &(oarg.clone()), metamodelica::AsArg::as_arg(&tplPackage))?;
                    (mmargs, _) = typeAdaptMMArgsForFun(metamodelica::AsArg::as_arg(&argvals), metamodelica::AsArg::as_arg(&iargs), tyVars.clone(), metamodelica::nil(), astDefs.clone())?;
                    lhsArgs = elabOutTextArgs(&mmargs, metamodelica::AsArg::as_arg(&iargs), oargs.clone(), tplPackage.clone())?;
                    lhsArgs = metamodelica::cons(outtxt.clone(), lhsArgs.clone());
                    mmtxt = metamodelica::Ref::new(MMExp::MM_IDENT { ident: metamodelica::Ref::new(PathIdent::IDENT { ident: outtxt.clone() }) });
                    mmexp = metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: fname.clone(), args: metamodelica::cons(metamodelica::Ref::new(MMExp::MM_IDENT { ident: metamodelica::Ref::new(PathIdent::IDENT { ident: intxt.clone() }) }), mmargs.clone()) });
                    Ok((false, metamodelica::Ref::new(MMExp::MM_ASSIGN { lhsArgs: lhsArgs.clone(), rhs: mmexp.clone() }), mmtxt.clone(), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE(), locals.clone(), outtxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argvals, fname, iargs, Deref @ metamodelica::ListNode::Cons { head: (_, outtype), tail: Deref @ metamodelica::ListNode::Nil }, tyVars, intxt, _, locals, TemplPackage { astDefs, .. }) => {
                    let mut setTyVars: TypedIdents;
                    let mut mmargs: metamodelica::List<metamodelica::Ref<MMExp>>;
                    let mut retval: Ident;
                    let mut mmexp: metamodelica::Ref<MMExp>;
                    let mut outtype = (*outtype).clone();
                    let mut locals = (*locals).clone();
                    (mmargs, setTyVars) = typeAdaptMMArgsForFun(metamodelica::AsArg::as_arg(&argvals), metamodelica::AsArg::as_arg(&iargs), tyVars.clone(), metamodelica::nil(), astDefs.clone())?;
                    outtype = specializeType(outtype.clone(), tyVars.clone(), setTyVars.clone())?;
                    retval = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(returnTempVarNamePrefix)); __mm_s.push_str(&*intString(((locals).len() as i32))); ArcStr::from(__mm_s) };
                    locals = addLocalValue(retval.clone(), outtype.clone(), locals.clone())?;
                    mmexp = metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: fname.clone(), args: mmargs.clone() });
                    Ok((true, metamodelica::Ref::new(MMExp::MM_ASSIGN { lhsArgs: list![retval.clone()], rhs: mmexp.clone() }), metamodelica::Ref::new(MMExp::MM_IDENT { ident: metamodelica::Ref::new(PathIdent::IDENT { ident: retval.clone() }) }), outtype.clone(), locals.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argvals, fname, iargs, Deref @ metamodelica::ListNode::Nil, tyVars, intxt, _, locals, TemplPackage { astDefs, .. }) => {
                    let mut mmargs: metamodelica::List<metamodelica::Ref<MMExp>>;
                    let mut mmexp: metamodelica::Ref<MMExp>;
                    (mmargs, _) = typeAdaptMMArgsForFun(metamodelica::AsArg::as_arg(&argvals), metamodelica::AsArg::as_arg(&iargs), tyVars.clone(), metamodelica::nil(), astDefs.clone())?;
                    mmexp = metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: fname.clone(), args: mmargs.clone() });
                    Ok((false, mmexp.clone(), mmexp.clone(), metamodelica::Ref::new(TypeSignature::UNRESOLVED_TYPE { reason: literal!("No return value.") }), locals.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argvals, fname, iargs, oargs, _, _, _, _, _) => {
                    let mut errArgVals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>)>;
                    let mut r#str: ArcStr;
                    errArgVals = List::map(argvals.clone(), &fnptr!(Util::tuple312, _))?;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cannot elaborate function\n  ")); __mm_s.push_str(&*Tpl::tplString3((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<PathIdent>, __a2: metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>, __a3: metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>| TplCodegen::sFunSignature(__a0, __a1, &__a2, &__a3)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<PathIdent>, metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>, metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>) -> Result<Tpl::Text> + 'static>), fname.clone(), iargs.clone(), oargs.clone())?); __mm_s.push_str(&*literal!("\n  for actual parameters  ")); __mm_s.push_str(&*Tpl::tplString((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>)>| TplCodegen::sActualMMParams(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>)>) -> Result<Tpl::Text> + 'static>), errArgVals.clone())?); __mm_s.push_str(&*literal!("\n  --> Invalid types (cannot convert) or number of in/out arguments (text in/out arguments must match by order and name equality where prefixes 'in' and 'out' can be used; A function has valid template signature only if all text out params have corresponding in text arguments.).\n")); ArcStr::from(__mm_s) };
                    addSusanError(r#str.clone(), inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outHasRetValue, outStmt, outRetMMExp, outRetType, outLocals, outOutText))
}

pub(crate) fn areTextInOutArgs(
    mut inInArg: &(ArcStr, metamodelica::Ref<TypeSignature>),
    mut inOutArg: &(ArcStr, metamodelica::Ref<TypeSignature>),
    mut inTplPackage: &TemplPackage,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inInArg, inOutArg, inTplPackage);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((inid, itype), (outid, otype), TemplPackage { astDefs: astdefs, .. }) => {
                    let true = (stringEq(&inid, &outid)) else { return Err("pattern mismatch") };
                    ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&itype), astdefs.clone())) {
                        Deref @ TypeSignature::TEXT_TYPE { .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&otype), astdefs.clone())) {
                        Deref @ TypeSignature::TEXT_TYPE { .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((inid, itype), (outid, otype), TemplPackage { astDefs: astdefs, .. }) => {
                    let mut inlst: metamodelica::List<ArcStr>;
                    let mut outlst: metamodelica::List<ArcStr>;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inid.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: __pa0 } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    inlst = metamodelica::Own::own(__pa0);
                    let __pa2 = ::match_deref::match_deref! { match &(stringListStringChar(outid.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "u", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: __pa2 } } } => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    outlst = metamodelica::Own::own(__pa2);
                    let true = (inlst.clone() == outlst.clone()) else { return Err("pattern mismatch") };
                    ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&itype), astdefs.clone())) {
                        Deref @ TypeSignature::TEXT_TYPE { .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&otype), astdefs.clone())) {
                        Deref @ TypeSignature::TEXT_TYPE { .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(())
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

pub(crate) fn typeAdaptMMArgsForFun(
    mut inArgValues: &metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>,
    mut inInArgs: &TypedIdents,
    mut inTypeVars: metamodelica::List<ArcStr>,
    mut inSetTypeVars: TypedIdents,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<(metamodelica::List<metamodelica::Ref<MMExp>>, TypedIdents)> {
    let mut outMMArguments: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outSetTypeVars: TypedIdents;
    (outMMArguments, outSetTypeVars) = 'mc: {
        let __mc_input = (&**inArgValues, &**inInArgs, inTypeVars, inSetTypeVars, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _, setTyVars, _) => {
                    Ok((metamodelica::nil(), setTyVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (mmarg, argtype, sinfo), tail: argvals }, Deref @ metamodelica::ListNode::Cons { head: (_, sigArgtype), tail: iargs }, tyVars, setTyVars, astdefs) => {
                    let mut mmargs: metamodelica::List<metamodelica::Ref<MMExp>>;
                    let mut mmarg = (*mmarg).clone();
                    let mut argtype = (*argtype).clone();
                    let mut setTyVars = (*setTyVars).clone();
                    argtype = deAliasedType(metamodelica::AsArg::as_arg(&argtype), astdefs.clone());
                    (mmarg, setTyVars) = typeAdaptMMArg(mmarg.clone(), argtype.clone(), sinfo.clone(), true, sigArgtype.clone(), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    (mmargs, setTyVars) = typeAdaptMMArgsForFun(metamodelica::AsArg::as_arg(&argvals), metamodelica::AsArg::as_arg(&iargs), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    Ok((metamodelica::cons(mmarg.clone(), mmargs.clone()), setTyVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("Error - more arguments expected for a function.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Nil, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("Error - less number of arguments expected for a function.\n"))?;
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
                    Debug::trace(literal!("!!! - typeAdaptMMArgsForFun failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outMMArguments, outSetTypeVars))
}

pub(crate) fn typeAdaptMMArg(
    mut inMMArg: metamodelica::Ref<MMExp>,
    mut inArgType: metamodelica::Ref<TypeSignature>,
    mut inSourceInfo: SourceInfo,
    mut errorWhenFail: bool,
    mut inTargetType: metamodelica::Ref<TypeSignature>,
    mut inTypeVars: metamodelica::List<ArcStr>,
    mut inSetTypeVars: TypedIdents,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<(metamodelica::Ref<MMExp>, TypedIdents)> {
    let mut outMMArg: metamodelica::Ref<MMExp>;
    let mut outSetTypeVars: TypedIdents;
    (outMMArg, outSetTypeVars) = 'mc: {
        let __mc_input = (
            inMMArg,
            inArgType,
            inSourceInfo,
            errorWhenFail,
            inTargetType,
            inTypeVars,
            inSetTypeVars,
            inASTDefs,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, argtype @ Deref @ TypeSignature::STRING_TOKEN_TYPE { .. }, sinfo, _, targettype, tyVars, setTyVars, astdefs) => {
                    let mut mmarg: metamodelica::Ref<MMExp>;
                    let mut setTyVars = (*setTyVars).clone();
                    setTyVars = typesEqual(targettype.clone(), crate::TplAbsyn::TypeSignature::interned_STRING_TYPE(), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    mmarg = mmExpToString(mmexp.clone(), argtype.clone(), metamodelica::AsArg::as_arg(&sinfo))?;
                    Ok((mmarg.clone(), setTyVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, argtype @ Deref @ TypeSignature::TEXT_TYPE { .. }, sinfo, _, targettype, tyVars, setTyVars, astdefs) => {
                    let mut mmarg: metamodelica::Ref<MMExp>;
                    let mut setTyVars = (*setTyVars).clone();
                    setTyVars = typesEqual(targettype.clone(), crate::TplAbsyn::TypeSignature::interned_STRING_TYPE(), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    mmarg = mmExpToString(mmexp.clone(), argtype.clone(), metamodelica::AsArg::as_arg(&sinfo))?;
                    Ok((mmarg.clone(), setTyVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, argtype, _, _, targettype, tyVars, setTyVars, astdefs) => {
                    let mut setTyVars = (*setTyVars).clone();
                    setTyVars = typesEqual(targettype.clone(), argtype.clone(), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    Ok((mmarg.clone(), setTyVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, argtype, sinfo, _, targettype, tyVars, setTyVars, astdefs) => {
                    let mut mmarg: metamodelica::Ref<MMExp>;
                    let mut setTyVars = (*setTyVars).clone();
                    setTyVars = typesEqual(targettype.clone(), crate::TplAbsyn::TypeSignature::interned_STRING_TYPE(), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    mmarg = mmExpToString(mmexp.clone(), argtype.clone(), metamodelica::AsArg::as_arg(&sinfo))?;
                    Ok((mmarg.clone(), setTyVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, Deref @ TypeSignature::STRING_TOKEN_TYPE { .. }, _, _, targettype, tyVars, setTyVars, astdefs) => {
                    let mut setTyVars = (*setTyVars).clone();
                    setTyVars = typesEqual(targettype.clone(), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE(), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    Ok((metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("strTokText") }) }), args: list![mmarg.clone()] }), setTyVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, argtype, sinfo, _, targettype, tyVars, setTyVars, astdefs) => {
                    let mut mmarg = (*mmarg).clone();
                    let mut setTyVars = (*setTyVars).clone();
                    setTyVars = typesEqual(targettype.clone(), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE(), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    mmarg = mmExpToString(mmarg.clone(), argtype.clone(), metamodelica::AsArg::as_arg(&sinfo))?;
                    Ok((metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("stringText") }) }), args: list![mmarg.clone()] }), setTyVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, argtype, sinfo, true, targettype, _, setTyVars, _) => {
                    let mut msg: ArcStr;
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Elaborated expression '")); __mm_s.push_str(&*mmExpString(mmarg.clone())?); __mm_s.push_str(&*literal!("' of type '")); __mm_s.push_str(&*typeSignatureString(metamodelica::AsArg::as_arg(&argtype))?); __mm_s.push_str(&*literal!("' failed to type adapt to its inferred type '")); __mm_s.push_str(&*typeSignatureString(metamodelica::AsArg::as_arg(&targettype))?); __mm_s.push_str(&*literal!("'.")); ArcStr::from(__mm_s) };
                    addSusanError(msg.clone(), metamodelica::AsArg::as_arg(&sinfo))?;
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#Error# ")); __mm_s.push_str(&*msg); __mm_s.push_str(&*literal!(" #")); ArcStr::from(__mm_s) };
                    Ok((metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(PathIdent::IDENT { ident: msg.clone() }), args: list![mmarg.clone()] }), setTyVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, false, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("Fail branch- typeAdaptMMArg failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outMMArg, outSetTypeVars))
}

pub(crate) fn typeAdaptMMOption(
    mut inMMArg: metamodelica::Ref<MMExp>,
    mut inArgType: metamodelica::Ref<TypeSignature>,
    mut sinfo: &SourceInfo,
    mut inTargetType: metamodelica::Ref<TypeSignature>,
    mut inStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inLocals: TypedIdents,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<(
    metamodelica::Ref<MMExp>,
    metamodelica::List<metamodelica::Ref<MMExp>>,
    TypedIdents,
)> {
    let mut outMMArg: metamodelica::Ref<MMExp>;
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outLocals: TypedIdents;
    (outMMArg, outStmts, outLocals) = 'mc: {
        let __mc_input = (inMMArg, inArgType, inTargetType, inStmts, inLocals, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, argtype, Deref @ TypeSignature::OPTION_TYPE { ofType: targettype }, stmts, locals, astdefs) => {
                    let mut mmarg = (*mmarg).clone();
                    let mut targettype = (*targettype).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    targettype = deAliasedType(metamodelica::AsArg::as_arg(&targettype), astdefs.clone());
                    (mmarg, stmts, locals) = typeAdaptMMOption(mmarg.clone(), argtype.clone(), sinfo, targettype.clone(), stmts.clone(), locals.clone(), astdefs.clone())?;
                    mmarg = metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("SOME") }), args: list![mmarg.clone()] });
                    Ok((mmarg.clone(), stmts.clone(), locals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, argtype, targettype, stmts, locals, astdefs) => {
                    let mut mmarg = (*mmarg).clone();
                    let mut argtype = (*argtype).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    argtype = deAliasedType(metamodelica::AsArg::as_arg(&argtype), astdefs.clone());
                    (mmarg, _) = typeAdaptMMArg(mmarg.clone(), argtype.clone(), sinfo.clone(), false, targettype.clone(), metamodelica::nil(), metamodelica::nil(), astdefs.clone())?;
                    (mmarg, stmts, locals) = mmEnsureNonFunctionArg(mmarg.clone(), targettype.clone(), stmts.clone(), locals.clone())?;
                    Ok((mmarg.clone(), stmts.clone(), locals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, Deref @ TypeSignature::TEXT_TYPE { .. }, Deref @ TypeSignature::STRING_TOKEN_TYPE { .. }, stmts, locals, _) => {
                    let mut mmarg = (*mmarg).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    mmarg = metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("textStrTok") }) }), args: list![mmarg.clone()] });
                    (mmarg, stmts, locals) = mmEnsureNonFunctionArg(mmarg.clone(), crate::TplAbsyn::TypeSignature::interned_STRING_TOKEN_TYPE(), stmts.clone(), locals.clone())?;
                    Ok((mmarg.clone(), stmts.clone(), locals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, argtype, Deref @ TypeSignature::STRING_TOKEN_TYPE { .. }, stmts, locals, _) => {
                    let mut mmarg = (*mmarg).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    mmarg = mmExpToString(mmarg.clone(), argtype.clone(), sinfo)?;
                    (mmarg, stmts, locals) = mmEnsureNonFunctionArg(mmarg.clone(), crate::TplAbsyn::TypeSignature::interned_STRING_TYPE(), stmts.clone(), locals.clone())?;
                    mmarg = metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("ST_STRING") }) }), args: list![mmarg.clone()] });
                    Ok((mmarg.clone(), stmts.clone(), locals.clone()))
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
                    Debug::trace(literal!("Error - typeAdaptMMOption failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outMMArg, outStmts, outLocals))
}

pub(crate) fn mmEnsureNonFunctionArg(
    mut inMMArg: metamodelica::Ref<MMExp>,
    mut inTargetType: metamodelica::Ref<TypeSignature>,
    mut inStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inLocals: TypedIdents,
) -> Result<(
    metamodelica::Ref<MMExp>,
    metamodelica::List<metamodelica::Ref<MMExp>>,
    TypedIdents,
)> {
    let mut outMMArg: metamodelica::Ref<MMExp>;
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outLocals: TypedIdents;
    (outMMArg, outStmts, outLocals) = 'mc: {
        let __mc_input = (inMMArg, inTargetType, inStmts, inLocals);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg @ Deref @ MMExp::MM_FN_CALL { .. }, targettype, stmts, locals) => {
                    let mut retval: ArcStr;
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    retval = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(returnTempVarNamePrefix)); __mm_s.push_str(&*intString(((locals).len() as i32))); ArcStr::from(__mm_s) };
                    locals = addLocalValue(retval.clone(), targettype.clone(), locals.clone())?;
                    stmts = metamodelica::cons(metamodelica::Ref::new(MMExp::MM_ASSIGN { lhsArgs: list![retval.clone()], rhs: mmarg.clone() }), stmts.clone());
                    Ok((metamodelica::Ref::new(MMExp::MM_IDENT { ident: metamodelica::Ref::new(PathIdent::IDENT { ident: retval.clone() }) }), stmts.clone(), locals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, _, stmts, locals) => {
                    if '__try0: {
                        ::match_deref::match_deref! { match &(mmarg.clone()) {
                            Deref @ MMExp::MM_FN_CALL { .. } => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok((mmarg.clone(), stmts.clone(), locals.clone()))
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
                    Debug::trace(literal!("!!!- mmEnsureNonFunctionArg failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outMMArg, outStmts, outLocals))
}

pub(crate) fn elabOutTextArgs(
    mut inMMArguments: &metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inInArgs: &TypedIdents,
    mut inOutArgs: TypedIdents,
    mut inTplPackage: TemplPackage,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outLhsArgs: metamodelica::List<ArcStr>;
    outLhsArgs = 'mc: {
        let __mc_input = (&**inMMArguments, &**inInArgs, inOutArgs, inTplPackage);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: mmargs }, Deref @ metamodelica::ListNode::Cons { head: iarg, tail: iargs }, oargs @ Deref @ metamodelica::ListNode::Cons { head: oarg, tail: _ }, tplPackage) => {
                    let mut lhsArgs: metamodelica::List<ArcStr>;
                    if '__try0: {
                        unwrap_break_err!(areTextInOutArgs(&(iarg.clone()), &(oarg.clone()), metamodelica::AsArg::as_arg(&tplPackage)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    lhsArgs = elabOutTextArgs(metamodelica::AsArg::as_arg(&mmargs), metamodelica::AsArg::as_arg(&iargs), oargs.clone(), tplPackage.clone())?;
                    Ok(lhsArgs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: exp @ Deref @ MMExp::MM_IDENT { ident: Deref @ PathIdent::IDENT { ident: txtarg } }, tail: mmargs }, Deref @ metamodelica::ListNode::Cons { head: _, tail: iargs }, Deref @ metamodelica::ListNode::Cons { head: _, tail: oargs }, tplPackage) => {
                    let mut lhsArgs: metamodelica::List<ArcStr>;
                    let false = (listMember(exp.clone(), mmargs.clone())) else { return Err("pattern mismatch") };
                    lhsArgs = elabOutTextArgs(metamodelica::AsArg::as_arg(&mmargs), metamodelica::AsArg::as_arg(&iargs), oargs.clone(), tplPackage.clone())?;
                    Ok(metamodelica::cons(txtarg.clone(), lhsArgs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: mmargs }, Deref @ metamodelica::ListNode::Cons { head: _, tail: iargs }, Deref @ metamodelica::ListNode::Cons { head: _, tail: oargs }, tplPackage) => {
                    let mut lhsArgs: metamodelica::List<ArcStr>;
                    lhsArgs = elabOutTextArgs(metamodelica::AsArg::as_arg(&mmargs), metamodelica::AsArg::as_arg(&iargs), oargs.clone(), tplPackage.clone())?;
                    Ok(metamodelica::cons(literal!("_"), lhsArgs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("Error - inconsistent in/out Text arguments for a template function (Output texts are not a subset of input texts).\n"))?;
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
                    Debug::trace(literal!("!!!- elabOutTextArgs failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outLhsArgs)
}

pub(crate) fn statementsFromMapExp(
    mut inIsFirstArgToMap: bool,
    mut inArgValuesToMap: &metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>,
    mut inMapContext: &MapContext,
    mut inStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inInText: Ident,
    mut inOutText: Ident,
    mut inLocals: TypedIdents,
    mut inScopeEnv: ScopeEnv,
    mut inTplPackage: TemplPackage,
    mut inAccMMDecls: metamodelica::List<MMDeclaration>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<MMExp>>,
    TypedIdents,
    ScopeEnv,
    metamodelica::List<MMDeclaration>,
    Ident,
)> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outLocals: TypedIdents;
    let mut outScopeEnv: ScopeEnv;
    let mut outMMDecls: metamodelica::List<MMDeclaration>;
    let mut outInText: Ident;
    (outStmts, outLocals, outScopeEnv, outMMDecls, outInText) = 'mc: {
        let __mc_input = (
            inIsFirstArgToMap,
            &**inArgValuesToMap,
            inMapContext,
            inStmts,
            inInText,
            inOutText,
            inLocals,
            inScopeEnv,
            inTplPackage,
            inAccMMDecls,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, MapContext { useIter: true, .. }, stmts, intxt, outtxt, locals, scEnv, _, accMMDecls) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    stmt = tplStatement(literal!("popIter"), metamodelica::nil(), intxt.clone(), outtxt.clone());
                    Ok((metamodelica::cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, MapContext { useIter: false, .. }, stmts, intxt, _, locals, scEnv, _, accMMDecls) => {
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (isfirst, Deref @ metamodelica::ListNode::Cons { head: argtomap @ (_, argtype, _), tail: restargs }, MapContext { ofBinding: ofbind, mapExp: mapexp @ (_, sinfo), iterMMExpOptions: iopts, hasIndexIdentOpt, useIter: useiter }, stmts, intxt, outtxt, locals, scEnv, tplPackage @ TemplPackage { astDefs, .. }, accMMDecls) => {
                    let mut mapstmts: metamodelica::List<metamodelica::Ref<MMExp>>;
                    let mut stmt: metamodelica::Ref<MMExp>;
                    let mut oftype: metamodelica::Ref<TypeSignature>;
                    let mut fname: Ident;
                    let mut idxName: Ident;
                    let mut freshIdxName: Ident;
                    let mut eltName: Ident;
                    let mut localArgs: TypedIdents;
                    let mut encodedExtargs: TypedIdents;
                    let mut maplocals: TypedIdents;
                    let mut caseLocals: TypedIdents;
                    let mut iargs: TypedIdents;
                    let mut oargs: TypedIdents;
                    let mut matchLocals: TypedIdents;
                    let mut mapctx: MapContext;
                    let mut extargvals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
                    let mut ofbindEnc: metamodelica::Ref<MatchingExp>;
                    let mut mexp: metamodelica::Ref<MatchingExp>;
                    let mut mmmcMatched: MMMatchCase;
                    let mut isUsed: bool;
                    let mut mmFun: MMDeclaration;
                    let mut mmmcases: metamodelica::List<(metamodelica::List<metamodelica::Ref<MatchingExp>>, metamodelica::List<metamodelica::Ref<MMExp>>)>;
                    let mut assignedIdents: metamodelica::List<ArcStr>;
                    let mut localNames: metamodelica::List<(ArcStr, ArcStr)>;
                    let mut useiter = (*useiter).clone();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&argtype), astDefs.clone())) {
                        Deref @ TypeSignature::LIST_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    oftype = metamodelica::Own::own(__pa0);
                    ofbindEnc = typeCheckMatchingExp(ofbind.clone(), oftype.clone(), astDefs.clone())?;
                    idxName = Util::getOptionOrDefault(hasIndexIdentOpt.clone(), arcstr::literal!(impossibleIdent));
                    freshIdxName = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(indexNamePrefix)); __mm_s.push_str(&*idxName); ArcStr::from(__mm_s) };
                    (mapstmts, maplocals, scEnv, accMMDecls, _) = statementsFromExp(&(mapexp.clone()), metamodelica::nil(), metamodelica::nil(), arcstr::literal!(imlicitTxt), arcstr::literal!(imlicitTxt), metamodelica::nil(), metamodelica::cons(Scope::LET_SCOPE { ident: idxName.clone(), idType: crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE(), freshIdent: freshIdxName.clone(), isUsed: false }, metamodelica::cons(Scope::CASE_SCOPE { mExp: ofbindEnc.clone(), mType: oftype.clone(), localNames: metamodelica::nil(), accLocals: metamodelica::nil(), extArgs: metamodelica::nil(), matchArgName: arcstr::literal!(impossibleIdent), hasImplicitScope: true }, metamodelica::cons(Scope::FUN_SCOPE { args: metamodelica::nil(), localArgs: metamodelica::nil() }, scEnv.clone()))), tplPackage.clone(), accMMDecls.clone())?;
                    let (__pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(scEnv.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Scope::LET_SCOPE { ident: _, idType: _, freshIdent: _, isUsed: __pa1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Scope::CASE_SCOPE { mExp: __pa2, mType: _, localNames: __pa3, accLocals: __pa4, extArgs: __pa5, matchArgName: _, hasImplicitScope: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Scope::FUN_SCOPE { args: _, localArgs: __pa6 }, tail: __pa7 } } } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    isUsed = metamodelica::Own::own(__pa1);
                    mexp = metamodelica::Own::own(__pa2);
                    localNames = metamodelica::Own::own(__pa3);
                    caseLocals = metamodelica::Own::own(__pa4);
                    encodedExtargs = metamodelica::Own::own(__pa5);
                    localArgs = metamodelica::Own::own(__pa6);
                    scEnv = metamodelica::Own::own(__pa7);
                    (mexp, _) = rewriteMatchExpByLocalNames(mexp.clone(), oftype.clone(), &localNames, metamodelica::nil(), astDefs.clone())?;
                    maplocals = listAppend(caseLocals.clone(), maplocals.clone());
                    useiter = shouldUseIterFunctions(isfirst.clone(), useiter.clone(), true, isUsed, iopts.clone(), metamodelica::AsArg::as_arg(&restargs));
                    stmt = tplStatement(literal!("nextIter"), metamodelica::nil(), arcstr::literal!(imlicitTxt), arcstr::literal!(imlicitTxt));
                    mapstmts = if (useiter.clone()) {metamodelica::cons(stmt.clone(), mapstmts.clone())} else {mapstmts.clone()};
                    fname = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(listMapFunPrefix)); __mm_s.push_str(&*intString(((accMMDecls).len() as i32))); ArcStr::from(__mm_s) };
                    iargs = metamodelica::cons(imlicitTxtArg.clone(), metamodelica::cons((literal!("items"), argtype.clone()), encodedExtargs.clone()));
                    assignedIdents = getAssignedIdents(&mapstmts, metamodelica::nil())?;
                    oargs = List::filter1OnTrue(encodedExtargs.clone(), (std::sync::Arc::new(move |__a0: (ArcStr, metamodelica::Ref<TypeSignature>), __a1: metamodelica::List<ArcStr>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isAssignedText(&__a0, __a1)) }) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, metamodelica::Ref<TypeSignature>), metamodelica::List<ArcStr>) -> Result<bool> + 'static>), assignedIdents.clone())?;
                    oargs = metamodelica::cons(imlicitTxtArg.clone(), oargs.clone());
                    mapstmts = mapstmts.clone().reverse();
                    (mapstmts, maplocals) = addGetIndex(isUsed, freshIdxName.clone(), mapstmts.clone(), arcstr::literal!(imlicitTxt), maplocals.clone())?;
                    matchLocals = maplocals.clone();
                    eltName = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lstElt_")); __mm_s.push_str(&*intString(((accMMDecls).len() as i32))); ArcStr::from(__mm_s) };
                    mmmcMatched = (list![mexp.clone()], mapstmts.clone());
                    mmmcases = if (isAlwaysMatchedBool(mexp.clone())) {list![mmmcMatched.clone()]} else {list![mmmcMatched.clone(), (list![crate::TplAbsyn::MatchingExp::interned_REST_MATCH()], metamodelica::nil())]};
                    mapctx = MapContext { ofBinding: ofbind.clone(), mapExp: mapexp.clone(), iterMMExpOptions: iopts.clone(), hasIndexIdentOpt: hasIndexIdentOpt.clone(), useIter: useiter.clone() };
                    mmFun = MMDeclaration::MM_FUN { isPublic: false, name: fname.clone(), inArgs: iargs.clone(), outArgs: oargs.clone(), locals: metamodelica::nil(), statements: list![metamodelica::Ref::new(MMExp::MM_LIST_FOR_LOOP { eltName: eltName.clone(), listName: literal!("items"), matchLocals: matchLocals.clone(), matchCases: mmmcases.clone() })], genInfoOpt: GenInfo::GI_MAP_FUN { mapType: argtype.clone(), mapContext: mapctx.clone() } };
                    (stmts, intxt) = addPushIter(isfirst.clone() && useiter.clone(), iopts.clone(), stmts.clone(), intxt.clone(), outtxt.clone())?;
                    extargvals = List::map(localArgs.clone(), &move |__a0: (ArcStr, metamodelica::Ref<TypeSignature>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(makeMMArgValue(&__a0)) })?;
                    (_, stmt, _, _, locals, intxt) = statementFromFun(metamodelica::cons(argtomap.clone(), extargvals.clone()), metamodelica::Ref::new(PathIdent::IDENT { ident: fname.clone() }), iargs.clone(), oargs.clone(), metamodelica::nil(), intxt.clone(), outtxt.clone(), locals.clone(), tplPackage.clone(), metamodelica::AsArg::as_arg(&sinfo))?;
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromMapExp(false, metamodelica::AsArg::as_arg(&restargs), &mapctx, metamodelica::cons(stmt.clone(), stmts.clone()), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), metamodelica::cons(mmFun.clone(), accMMDecls.clone()))?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (isfirst, Deref @ metamodelica::ListNode::Cons { head: argtomap @ (_, argtype, _), tail: restargs }, MapContext { ofBinding: ofbind, mapExp: mapexp @ (_, sinfo), iterMMExpOptions: iopts, hasIndexIdentOpt, useIter: useiter }, stmts, intxt, outtxt, locals, scEnv, tplPackage @ TemplPackage { astDefs, .. }, accMMDecls) => {
                            let mut mapstmts: metamodelica::List<metamodelica::Ref<MMExp>>;
                            let mut stmt: metamodelica::Ref<MMExp>;
                            let mut oftype: metamodelica::Ref<TypeSignature>;
                            let mut fname: Ident;
                            let mut idxName: Ident;
                            let mut freshIdxName: Ident;
                            let mut arrName: Ident;
                            let mut eltName: Ident;
                            let mut localArgs: TypedIdents;
                            let mut encodedExtargs: TypedIdents;
                            let mut maplocals: TypedIdents;
                            let mut caseLocals: TypedIdents;
                            let mut iargs: TypedIdents;
                            let mut oargs: TypedIdents;
                            let mut mapctx: MapContext;
                            let mut extargvals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
                            let mut ofbindEnc: metamodelica::Ref<MatchingExp>;
                            let mut mexp: metamodelica::Ref<MatchingExp>;
                            let mut isUsed: bool;
                            let mut mmFun: MMDeclaration;
                            let mut assignedIdents: metamodelica::List<ArcStr>;
                            let mut localNames: metamodelica::List<(ArcStr, ArcStr)>;
                            let mut useiter = (*useiter).clone();
                            let mut stmts = (*stmts).clone();
                            let mut intxt = (*intxt).clone();
                            let mut locals = (*locals).clone();
                            let mut scEnv = (*scEnv).clone();
                            let mut accMMDecls = (*accMMDecls).clone();
                            let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&argtype), astDefs.clone())) {
                                Deref @ TypeSignature::ARRAY_TYPE { ofType: __pa0 } => __pa0.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            oftype = metamodelica::Own::own(__pa0);
                            ofbindEnc = typeCheckMatchingExp(ofbind.clone(), oftype.clone(), astDefs.clone())?;
                            idxName = Util::getOptionOrDefault(hasIndexIdentOpt.clone(), arcstr::literal!(impossibleIdent));
                            freshIdxName = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(indexNamePrefix)); __mm_s.push_str(&*idxName); ArcStr::from(__mm_s) };
                            (mapstmts, maplocals, scEnv, accMMDecls, _) = statementsFromExp(&(mapexp.clone()), metamodelica::nil(), metamodelica::nil(), arcstr::literal!(imlicitTxt), arcstr::literal!(imlicitTxt), metamodelica::nil(), metamodelica::cons(Scope::LET_SCOPE { ident: idxName.clone(), idType: crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE(), freshIdent: freshIdxName.clone(), isUsed: false }, metamodelica::cons(Scope::CASE_SCOPE { mExp: ofbindEnc.clone(), mType: oftype.clone(), localNames: metamodelica::nil(), accLocals: metamodelica::nil(), extArgs: metamodelica::nil(), matchArgName: arcstr::literal!(impossibleIdent), hasImplicitScope: true }, metamodelica::cons(Scope::FUN_SCOPE { args: metamodelica::nil(), localArgs: metamodelica::nil() }, scEnv.clone()))), tplPackage.clone(), accMMDecls.clone())?;
                            let (__pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(scEnv.clone()) {
                                Deref @ metamodelica::ListNode::Cons { head: Scope::LET_SCOPE { ident: _, idType: _, freshIdent: _, isUsed: __pa1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Scope::CASE_SCOPE { mExp: __pa2, mType: _, localNames: __pa3, accLocals: __pa4, extArgs: __pa5, matchArgName: _, hasImplicitScope: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Scope::FUN_SCOPE { args: _, localArgs: __pa6 }, tail: __pa7 } } } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            isUsed = metamodelica::Own::own(__pa1);
                            mexp = metamodelica::Own::own(__pa2);
                            localNames = metamodelica::Own::own(__pa3);
                            caseLocals = metamodelica::Own::own(__pa4);
                            encodedExtargs = metamodelica::Own::own(__pa5);
                            localArgs = metamodelica::Own::own(__pa6);
                            scEnv = metamodelica::Own::own(__pa7);
                            (mexp, _) = rewriteMatchExpByLocalNames(mexp.clone(), oftype.clone(), &localNames, metamodelica::nil(), astDefs.clone())?;
                            maplocals = listAppend(caseLocals.clone(), maplocals.clone());
                            useiter = shouldUseIterFunctions(isfirst.clone(), useiter.clone(), true, isUsed, iopts.clone(), metamodelica::AsArg::as_arg(&restargs));
                            stmt = tplStatement(literal!("nextIter"), metamodelica::nil(), arcstr::literal!(imlicitTxt), arcstr::literal!(imlicitTxt));
                            mapstmts = if (useiter.clone()) {metamodelica::cons(stmt.clone(), mapstmts.clone())} else {mapstmts.clone()};
                            fname = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(arrayMapFunPrefix)); __mm_s.push_str(&*intString(((accMMDecls).len() as i32))); ArcStr::from(__mm_s) };
                            iargs = metamodelica::cons(imlicitTxtArg.clone(), metamodelica::cons((literal!("items"), argtype.clone()), encodedExtargs.clone()));
                            assignedIdents = getAssignedIdents(&mapstmts, metamodelica::nil())?;
                            oargs = List::filter1OnTrue(encodedExtargs.clone(), (std::sync::Arc::new(move |__a0: (ArcStr, metamodelica::Ref<TypeSignature>), __a1: metamodelica::List<ArcStr>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isAssignedText(&__a0, __a1)) }) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, metamodelica::Ref<TypeSignature>), metamodelica::List<ArcStr>) -> Result<bool> + 'static>), assignedIdents.clone())?;
                            oargs = metamodelica::cons(imlicitTxtArg.clone(), oargs.clone());
                            mapstmts = mapstmts.clone().reverse();
                            (mapstmts, maplocals) = addGetIndex(isUsed, freshIdxName.clone(), mapstmts.clone(), arcstr::literal!(imlicitTxt), maplocals.clone())?;
                            idxName = literal!("i");
                            arrName = literal!("items");
                            eltName = (match &*mexp {
                MatchingExp::BIND_MATCH { bindIdent: __esc_eltName } => {
                            eltName = (*__esc_eltName).clone();
                            eltName.clone()
                },
                MatchingExp::REST_MATCH { .. } => literal!(""),
                _ => return Err("match: no arm matched"),
            });
                            mapctx = MapContext { ofBinding: ofbind.clone(), mapExp: mapexp.clone(), iterMMExpOptions: iopts.clone(), hasIndexIdentOpt: hasIndexIdentOpt.clone(), useIter: useiter.clone() };
                            mmFun = MMDeclaration::MM_FUN { isPublic: false, name: fname.clone(), inArgs: iargs.clone(), outArgs: oargs.clone(), locals: maplocals.clone(), statements: list![metamodelica::Ref::new(MMExp::MM_FOR_LOOP { idxName: idxName.clone(), arrName: arrName.clone(), eltName: eltName.clone(), statements: mapstmts.clone() })], genInfoOpt: GenInfo::GI_MAP_FUN { mapType: argtype.clone(), mapContext: mapctx.clone() } };
                            (stmts, intxt) = addPushIter(isfirst.clone() && useiter.clone(), iopts.clone(), stmts.clone(), intxt.clone(), outtxt.clone())?;
                            extargvals = List::map(localArgs.clone(), &move |__a0: (ArcStr, metamodelica::Ref<TypeSignature>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(makeMMArgValue(&__a0)) })?;
                            (_, stmt, _, _, locals, intxt) = statementFromFun(metamodelica::cons(argtomap.clone(), extargvals.clone()), metamodelica::Ref::new(PathIdent::IDENT { ident: fname.clone() }), iargs.clone(), oargs.clone(), metamodelica::nil(), intxt.clone(), outtxt.clone(), locals.clone(), tplPackage.clone(), metamodelica::AsArg::as_arg(&sinfo))?;
                            (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromMapExp(false, metamodelica::AsArg::as_arg(&restargs), &mapctx, metamodelica::cons(stmt.clone(), stmts.clone()), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), metamodelica::cons(mmFun.clone(), accMMDecls.clone()))?;
                            Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (isfirst, Deref @ metamodelica::ListNode::Cons { head: argtomap @ (_, argtype, _), tail: restargs }, MapContext { ofBinding: ofbind, mapExp: mapexp @ (_, sinfo), iterMMExpOptions: iopts, hasIndexIdentOpt, useIter: useiter }, stmts, intxt, outtxt, locals, scEnv, tplPackage @ TemplPackage { astDefs, .. }, accMMDecls) => {
                    let mut mapstmts: metamodelica::List<metamodelica::Ref<MMExp>>;
                    let mut stmt: metamodelica::Ref<MMExp>;
                    let mut fname: Ident;
                    let mut idxName: Ident;
                    let mut freshIdxName: Ident;
                    let mut localArgs: TypedIdents;
                    let mut encodedExtargs: TypedIdents;
                    let mut maplocals: TypedIdents;
                    let mut caseLocals: TypedIdents;
                    let mut iargs: TypedIdents;
                    let mut oargs: TypedIdents;
                    let mut mapctx: MapContext;
                    let mut extargvals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
                    let mut ofbindEnc: metamodelica::Ref<MatchingExp>;
                    let mut mexp: metamodelica::Ref<MatchingExp>;
                    let mut isUsed: bool;
                    let mut mmFun: MMDeclaration;
                    let mut elabcases: metamodelica::List<(metamodelica::Ref<MatchingExp>, metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>, metamodelica::List<metamodelica::Ref<MMExp>>)>;
                    let mut mmmcases: metamodelica::List<(metamodelica::List<metamodelica::Ref<MatchingExp>>, metamodelica::List<metamodelica::Ref<MMExp>>)>;
                    let mut assignedIdents: metamodelica::List<ArcStr>;
                    let mut localNames: metamodelica::List<(ArcStr, ArcStr)>;
                    let mut useiter = (*useiter).clone();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&argtype), astDefs.clone())) {
                            Deref @ TypeSignature::LIST_TYPE { .. } => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    if '__try1: {
                        ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&argtype), astDefs.clone())) {
                            Deref @ TypeSignature::ARRAY_TYPE { .. } => (),
                            _ => break '__try1 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    ofbindEnc = typeCheckMatchingExp(ofbind.clone(), argtype.clone(), astDefs.clone())?;
                    idxName = Util::getOptionOrDefault(hasIndexIdentOpt.clone(), arcstr::literal!(impossibleIdent));
                    freshIdxName = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(indexNamePrefix)); __mm_s.push_str(&*idxName); ArcStr::from(__mm_s) };
                    (mapstmts, maplocals, scEnv, accMMDecls, _) = statementsFromExp(&(mapexp.clone()), metamodelica::nil(), metamodelica::nil(), arcstr::literal!(imlicitTxt), arcstr::literal!(imlicitTxt), metamodelica::nil(), metamodelica::cons(Scope::LET_SCOPE { ident: idxName.clone(), idType: crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE(), freshIdent: freshIdxName.clone(), isUsed: false }, metamodelica::cons(Scope::CASE_SCOPE { mExp: ofbindEnc.clone(), mType: argtype.clone(), localNames: metamodelica::nil(), accLocals: metamodelica::nil(), extArgs: metamodelica::nil(), matchArgName: arcstr::literal!(impossibleIdent), hasImplicitScope: true }, metamodelica::cons(Scope::FUN_SCOPE { args: metamodelica::nil(), localArgs: metamodelica::nil() }, scEnv.clone()))), tplPackage.clone(), accMMDecls.clone())?;
                    let (__pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(scEnv.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Scope::LET_SCOPE { ident: _, idType: _, freshIdent: _, isUsed: __pa2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Scope::CASE_SCOPE { mExp: __pa3, mType: _, localNames: __pa4, accLocals: __pa5, extArgs: __pa6, matchArgName: _, hasImplicitScope: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Scope::FUN_SCOPE { args: _, localArgs: __pa7 }, tail: __pa8 } } } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    isUsed = metamodelica::Own::own(__pa2);
                    mexp = metamodelica::Own::own(__pa3);
                    localNames = metamodelica::Own::own(__pa4);
                    caseLocals = metamodelica::Own::own(__pa5);
                    encodedExtargs = metamodelica::Own::own(__pa6);
                    localArgs = metamodelica::Own::own(__pa7);
                    scEnv = metamodelica::Own::own(__pa8);
                    (mexp, _) = rewriteMatchExpByLocalNames(mexp.clone(), argtype.clone(), &localNames, metamodelica::nil(), astDefs.clone())?;
                    maplocals = listAppend(caseLocals.clone(), maplocals.clone());
                    useiter = shouldUseIterFunctions(isfirst.clone(), useiter.clone(), false, isUsed, iopts.clone(), metamodelica::AsArg::as_arg(&restargs));
                    stmt = tplStatement(literal!("nextIter"), metamodelica::nil(), arcstr::literal!(imlicitTxt), arcstr::literal!(imlicitTxt));
                    mapstmts = if (useiter.clone()) {metamodelica::cons(stmt.clone(), mapstmts.clone())} else {mapstmts.clone()};
                    fname = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(scalarMapFunPrefix)); __mm_s.push_str(&*intString(((accMMDecls).len() as i32))); ArcStr::from(__mm_s) };
                    iargs = metamodelica::cons(imlicitTxtArg.clone(), metamodelica::cons((literal!("it"), argtype.clone()), encodedExtargs.clone()));
                    assignedIdents = getAssignedIdents(&mapstmts, metamodelica::nil())?;
                    oargs = List::filter1OnTrue(encodedExtargs.clone(), (std::sync::Arc::new(move |__a0: (ArcStr, metamodelica::Ref<TypeSignature>), __a1: metamodelica::List<ArcStr>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isAssignedText(&__a0, __a1)) }) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, metamodelica::Ref<TypeSignature>), metamodelica::List<ArcStr>) -> Result<bool> + 'static>), assignedIdents.clone())?;
                    oargs = metamodelica::cons(imlicitTxtArg.clone(), oargs.clone());
                    mapstmts = mapstmts.clone().reverse();
                    (mapstmts, maplocals) = addGetIndex(isUsed, freshIdxName.clone(), mapstmts.clone(), arcstr::literal!(imlicitTxt), maplocals.clone())?;
                    elabcases = addRestElabCase(list![(mexp.clone(), encodedExtargs.clone(), mapstmts.clone())]);
                    mmmcases = List::map2(elabcases.clone(), &move |__a0: (metamodelica::Ref<MatchingExp>, metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>, metamodelica::List<metamodelica::Ref<MMExp>>), __a1: metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>, __a2: metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>| makeMMMatchCase(&__a0, __a1, __a2), encodedExtargs.clone(), oargs.clone())?;
                    mapctx = MapContext { ofBinding: ofbind.clone(), mapExp: mapexp.clone(), iterMMExpOptions: iopts.clone(), hasIndexIdentOpt: hasIndexIdentOpt.clone(), useIter: useiter.clone() };
                    maplocals = listAppend(encodedExtargs.clone(), maplocals.clone());
                    maplocals = metamodelica::cons(imlicitTxtArg.clone(), maplocals.clone());
                    mmFun = MMDeclaration::MM_FUN { isPublic: false, name: fname.clone(), inArgs: iargs.clone(), outArgs: oargs.clone(), locals: maplocals.clone(), statements: list![metamodelica::Ref::new(MMExp::MM_MATCH { matchCases: mmmcases.clone() })], genInfoOpt: GenInfo::GI_MAP_FUN { mapType: argtype.clone(), mapContext: mapctx.clone() } };
                    (stmts, intxt) = addPushIter(isfirst.clone() && useiter.clone(), iopts.clone(), stmts.clone(), intxt.clone(), outtxt.clone())?;
                    extargvals = List::map(localArgs.clone(), &move |__a0: (ArcStr, metamodelica::Ref<TypeSignature>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(makeMMArgValue(&__a0)) })?;
                    (_, stmt, _, _, locals, intxt) = statementFromFun(metamodelica::cons(argtomap.clone(), extargvals.clone()), metamodelica::Ref::new(PathIdent::IDENT { ident: fname.clone() }), iargs.clone(), oargs.clone(), metamodelica::nil(), intxt.clone(), outtxt.clone(), locals.clone(), tplPackage.clone(), metamodelica::AsArg::as_arg(&sinfo))?;
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromMapExp(false, metamodelica::AsArg::as_arg(&restargs), &mapctx, metamodelica::cons(stmt.clone(), stmts.clone()), intxt.clone(), outtxt.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), metamodelica::cons(mmFun.clone(), accMMDecls.clone()))?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
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
                    Debug::trace(literal!("-!!!statementsFromMapExp failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outStmts, outLocals, outScopeEnv, outMMDecls, outInText))
}

pub(crate) fn intersectInOutArgs(
    mut inList1: TypedIdents,
    mut inList2: TypedIdents,
) -> Result<(
    metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>,
    metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>,
    metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>,
)> {
    pub(crate) fn areTypedIdentsEqual(
        mut inTypedIdent1: &(ArcStr, metamodelica::Ref<TypeSignature>),
        mut inTypedIdent2: &(ArcStr, metamodelica::Ref<TypeSignature>),
    ) -> bool {
        let mut equal: bool;
        let mut ident1: Ident;
        let mut ident2: Ident;
        (ident1, _) = inTypedIdent1.clone();
        (ident2, _) = inTypedIdent2.clone();
        equal = stringEq(&ident1, &ident2);
        equal
    }

    let mut outIntersectionAndRests: (
        metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>,
        metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>,
        metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>,
    );
    let mut outIntersection: TypedIdents;
    let mut outList1Rest: TypedIdents;
    let mut outList2Rest: TypedIdents;
    (outIntersection, outList1Rest, outList2Rest) = List::intersection1OnTrue(
        inList1,
        inList2,
        &move |__a0: (ArcStr, metamodelica::Ref<TypeSignature>),
               __a1: (ArcStr, metamodelica::Ref<TypeSignature>)|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(areTypedIdentsEqual(&__a0, &__a1)) },
    )?;
    outIntersectionAndRests = (outIntersection, outList1Rest, outList2Rest);
    Ok(outIntersectionAndRests)
}

pub(crate) fn isTupleListMember(mut inId: Ident, mut inList: TypedIdents) -> bool {
    let mut outIsMember: bool;
    outIsMember = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            lookupTupleList(inList.clone(), inId.clone())?;
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(false)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outIsMember
}

/*
function isIndexArg
  input tuple<Ident, TypeSignature> inArg;
  output Boolean outIsIndexArg;
algorithm
  outIsIndexArg := match inArg
    case ( ("i_i0" , _) )  then true;
    case ( ("i_i1" , _) )  then true;
    case ( _ )            then false;
  end match;
end isIndexArg;
*/
pub(crate) fn shouldUseIterFunctions(
    mut inIsFirstArgToMap: bool,
    mut inUseIterLast: bool,
    mut inIsListArgToMap: bool,
    mut wasIndexVarUsed: bool,
    mut inIterOptions: metamodelica::List<(ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>))>,
    mut inRestArgValsToMap: &metamodelica::List<(
        metamodelica::Ref<MMExp>,
        metamodelica::Ref<TypeSignature>,
        SourceInfo,
    )>,
) -> bool {
    let mut outUseIterFuns: bool;
    outUseIterFuns = 'mc: {
        let __mc_input = (
            inIsFirstArgToMap,
            inUseIterLast,
            inIsListArgToMap,
            wasIndexVarUsed,
            inIterOptions,
            &**inRestArgValsToMap,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, useiter, _, _, _, _) => {
                    Ok(useiter.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, true, false, iopts, _) => {
                    let mut iopts = (*iopts).clone();
                    iopts = listAppend(iopts.clone(), nonSpecifiedIterOptions.clone());
                    ::match_deref::match_deref! { match &(lookupTupleList(iopts.clone(), arcstr::literal!(emptyOptionId))?) {
                        (Deref @ MMExp::MM_LITERAL { value: Deref @ "NONE()" }, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(lookupTupleList(iopts.clone(), arcstr::literal!(separatorOptionId))?) {
                        (Deref @ MMExp::MM_LITERAL { value: Deref @ "NONE()" }, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(lookupTupleList(iopts.clone(), arcstr::literal!(alignNumOptionId))?) {
                        (Deref @ MMExp::MM_LITERAL { value: Deref @ "0" }, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(lookupTupleList(iopts.clone(), arcstr::literal!(wrapWidthOptionId))?) {
                        (Deref @ MMExp::MM_LITERAL { value: Deref @ "0" }, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, false, false, iopts, Deref @ metamodelica::ListNode::Nil) => {
                    let mut iopts = (*iopts).clone();
                    iopts = listAppend(iopts.clone(), nonSpecifiedIterOptions.clone());
                    ::match_deref::match_deref! { match &(lookupTupleList(iopts.clone(), arcstr::literal!(emptyOptionId))?) {
                        (Deref @ MMExp::MM_LITERAL { value: Deref @ "NONE()" }, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
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
    outUseIterFuns
}

/*
public function addNextIter
  input Boolean inUseIterFun;
  input list<MMExp> inStmts;
  input Ident inInText;
  input Ident inOutText;

  output list<MMExp> outStmts;
  output Ident outInText;
algorithm
  (outStmts, outInText)
  := matchcontinue (inUseIterFun, inStmts, inInText, inOutText)
    local
      list<MMExp> stmts;
      MMExp stmt;
      Ident intxt, outtxt;

    case ( true, stmts, intxt, outtxt)
      algorithm
        stmt = tplStatement("nextIter", {}, intxt, outtxt);
      then ( stmt :: stmts, outtxt );

    case ( false, stmts, intxt, _)
      then ( stmts, intxt );

    //cannot happen
    else
      algorithm
        true = Flags.isSet(Flags.FAILTRACE); Debug.trace("-!!!addNextIter failed\n");
      then
        fail();
  end matchcontinue;
end addNextIter;
*/
pub(crate) fn addGetIndex(
    mut wasIndexUsed: bool,
    mut inLocalIdxValIdent: Ident,
    mut inStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inInText: Ident,
    mut inLocals: TypedIdents,
) -> Result<(metamodelica::List<metamodelica::Ref<MMExp>>, TypedIdents)> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outLocals: TypedIdents;
    (outStmts, outLocals) = 'mc: {
        let __mc_input = (wasIndexUsed, inLocalIdxValIdent, inStmts, inInText, inLocals);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, localidxid, stmts, intxt, locals) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    let mut locals = (*locals).clone();
                    stmt = tplStatement(literal!("getIteri_i0"), metamodelica::nil(), intxt.clone(), localidxid.clone());
                    locals = addLocalValue(localidxid.clone(), crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE(), locals.clone())?;
                    Ok((metamodelica::cons(stmt.clone(), stmts.clone()), locals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, _, stmts, _, locals) => {
                    Ok((stmts.clone(), locals.clone()))
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
                    Debug::trace(literal!("-!!!addGetIndex failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outStmts, outLocals))
}

pub(crate) fn addPushIter(
    mut inDoAddPushIter: bool,
    mut inMMEscOptions: metamodelica::List<(ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>))>,
    mut inStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inInText: Ident,
    mut inOutText: Ident,
) -> Result<(metamodelica::List<metamodelica::Ref<MMExp>>, Ident)> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outInText: Ident;
    (outStmts, outInText) = 'mc: {
        let __mc_input = (inDoAddPushIter, inMMEscOptions, inStmts, inInText, inOutText);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, _, stmts, intxt, _) => {
                    Ok((stmts.clone(), intxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, opts, stmts, intxt, outtxt) => {
                    let mut mmopts: metamodelica::List<metamodelica::Ref<MMExp>>;
                    let mut stmt: metamodelica::Ref<MMExp>;
                    (mmopts, _) = makeMMExpOptions(&(nonSpecifiedIterOptions.clone()), opts.clone())?;
                    stmt = tplStatement(literal!("pushIter"), list![metamodelica::Ref::new(MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("ITER_OPTIONS") }) }), args: mmopts.clone() })], intxt.clone(), outtxt.clone());
                    Ok((metamodelica::cons(stmt.clone(), stmts.clone()), outtxt.clone()))
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
                    Debug::trace(literal!("-!!!addNextIter failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outStmts, outInText))
}

pub(crate) fn makeMMExpOptions(
    mut inMMEscOptions: &metamodelica::List<(ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>))>,
    mut inSpecifiedMMEscOptions: metamodelica::List<(
        ArcStr,
        (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>),
    )>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<MMExp>>,
    metamodelica::List<(ArcStr, (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>))>,
)> {
    let mut outMMExpOpts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outRestSpecifiedMMExpOpts: metamodelica::List<(
        ArcStr,
        (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>),
    )>;
    (outMMExpOpts, outRestSpecifiedMMExpOpts) = 'mc: {
        let __mc_input = (&**inMMEscOptions, inSpecifiedMMEscOptions);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, specopts) => {
                    warnIfSomeOptions(metamodelica::AsArg::as_arg(&specopts))?;
                    Ok((metamodelica::nil(), specopts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (optid, _), tail: rest }, specopts) => {
                    let mut mexpOpts: metamodelica::List<metamodelica::Ref<MMExp>>;
                    let mut mexpopt: metamodelica::Ref<MMExp>;
                    let mut specopts = (*specopts).clone();
                    let ((__pa0, _), __pa1) = lookupDeleteTupleList(metamodelica::AsArg::as_arg(&specopts), optid.clone())?;
                    mexpopt = metamodelica::Own::own(__pa0);
                    specopts = metamodelica::Own::own(__pa1);
                    (mexpOpts, specopts) = makeMMExpOptions(metamodelica::AsArg::as_arg(&rest), specopts.clone())?;
                    Ok((metamodelica::cons(mexpopt.clone(), mexpOpts.clone()), specopts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (_, (mexpopt, _)), tail: rest }, specopts) => {
                    let mut mexpOpts: metamodelica::List<metamodelica::Ref<MMExp>>;
                    let mut specopts = (*specopts).clone();
                    (mexpOpts, specopts) = makeMMExpOptions(metamodelica::AsArg::as_arg(&rest), specopts.clone())?;
                    Ok((metamodelica::cons(mexpopt.clone(), mexpOpts.clone()), specopts.clone()))
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
                    Debug::trace(literal!("-!!!makeMMExpOptions failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outMMExpOpts, outRestSpecifiedMMExpOpts))
}

/*
public function mmexpFromStrTokOption
  input Option<StringToken> inStrTokOption;
  output MMExp outMMExp;
algorithm
  outMMExp := match inStrTokOption
    local
      StringToken st;

    case NONE()
      then MM_LITERAL("NONE()");

    case ( SOME(st) )
      then MM_FN_CALL(IDENT("SOME"), { MM_STR_TOKEN(st) });

  end match;
end mmexpFromStrTokOption;
*/
//fail and error
pub(crate) fn makeMatchFun(
    mut inArgval: (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo),
    mut inMCases: metamodelica::List<(
        metamodelica::Ref<MatchingExp>,
        (metamodelica::Ref<ExpressionBase>, SourceInfo),
    )>,
    mut inArgExp: &Expression,
    mut hasImplicitLookup: bool,
    mut inScopeEnv: ScopeEnv,
    mut inTplPackage: TemplPackage,
    mut inAccMMDecls: metamodelica::List<MMDeclaration>,
) -> Result<(
    metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>,
    metamodelica::Ref<PathIdent>,
    TypedIdents,
    TypedIdents,
    ScopeEnv,
    metamodelica::List<MMDeclaration>,
)> {
    let mut outArgvals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
    let mut outFunName: metamodelica::Ref<PathIdent>;
    let mut outInArgs: TypedIdents;
    let mut outOutArgs: TypedIdents;
    let mut outScopeEnv: ScopeEnv;
    let mut outMMDecls: metamodelica::List<MMDeclaration>;
    (outArgvals, outFunName, outInArgs, outOutArgs, outScopeEnv, outMMDecls) = 'mc: {
        let __mc_input = (inArgval, inMCases, inScopeEnv, inTplPackage, inAccMMDecls);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argval @ (mmexp, exptype, _), mcases, scEnv, tplPackage, accMMDecls) => {
                    let mut argvals: metamodelica::List<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo)>;
                    let mut iargs: TypedIdents;
                    let mut oargs: TypedIdents;
                    let mut extargs: TypedIdents;
                    let mut localArgs: TypedIdents;
                    let mut encodedExtargs: TypedIdents;
                    let mut funLocals: TypedIdents;
                    let mut elabcases: metamodelica::List<(metamodelica::Ref<MatchingExp>, metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>, metamodelica::List<metamodelica::Ref<MMExp>>)>;
                    let mut mmmcases: metamodelica::List<(metamodelica::List<metamodelica::Ref<MatchingExp>>, metamodelica::List<metamodelica::Ref<MMExp>>)>;
                    let mut mmFun: MMDeclaration;
                    let mut fname: Ident;
                    let mut matchArgName: Ident;
                    let mut implicitValueName: Ident;
                    let mut assignedIdents: metamodelica::List<ArcStr>;
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    (implicitValueName, matchArgName) = getMatchArgName(inArgExp);
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(elabMatchCases((mmexp.clone(), exptype.clone()), &implicitValueName, metamodelica::AsArg::as_arg(&mcases), hasImplicitLookup, metamodelica::nil(), metamodelica::nil(), metamodelica::cons(Scope::FUN_SCOPE { args: metamodelica::nil(), localArgs: metamodelica::nil() }, scEnv.clone()), tplPackage.clone(), accMMDecls.clone())?) {
                        (__pa0, __pa1, Deref @ metamodelica::ListNode::Cons { head: Scope::FUN_SCOPE { args: __pa2, localArgs: __pa3 }, tail: __pa4 }, __pa5, __pa6) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    elabcases = metamodelica::Own::own(__pa0);
                    funLocals = metamodelica::Own::own(__pa1);
                    extargs = metamodelica::Own::own(__pa2);
                    localArgs = metamodelica::Own::own(__pa3);
                    scEnv = metamodelica::Own::own(__pa4);
                    accMMDecls = metamodelica::Own::own(__pa5);
                    assignedIdents = metamodelica::Own::own(__pa6);
                    elabcases = addRestElabCase(elabcases.clone());
                    (extargs, localArgs) = alignExtArgsToScopeEnv(extargs.clone(), localArgs.clone(), metamodelica::AsArg::as_arg(&scEnv));
                    encodedExtargs = List::map1(extargs.clone(), &move |__a0: (ArcStr, metamodelica::Ref<TypeSignature>), __a1: ArcStr| encodeTypedIdent(&__a0, &__a1), arcstr::literal!(funArgNamePrefix))?;
                    iargs = metamodelica::cons(imlicitTxtArg.clone(), metamodelica::cons((matchArgName.clone(), exptype.clone()), encodedExtargs.clone()));
                    oargs = List::filter1OnTrue(encodedExtargs.clone(), (std::sync::Arc::new(move |__a0: (ArcStr, metamodelica::Ref<TypeSignature>), __a1: metamodelica::List<ArcStr>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isAssignedText(&__a0, __a1)) }) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, metamodelica::Ref<TypeSignature>), metamodelica::List<ArcStr>) -> Result<bool> + 'static>), assignedIdents.clone())?;
                    oargs = metamodelica::cons(imlicitTxtArg.clone(), oargs.clone());
                    funLocals = listAppend(encodedExtargs.clone(), funLocals.clone());
                    mmmcases = List::map2(elabcases.clone(), &move |__a0: (metamodelica::Ref<MatchingExp>, metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>, metamodelica::List<metamodelica::Ref<MMExp>>), __a1: metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>, __a2: metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>| makeMMMatchCase(&__a0, __a1, __a2), encodedExtargs.clone(), oargs.clone())?;
                    fname = stringAppend(arcstr::literal!(matchFunPrefix), intString(((accMMDecls).len() as i32)));
                    mmFun = MMDeclaration::MM_FUN { isPublic: false, name: fname.clone(), inArgs: iargs.clone(), outArgs: oargs.clone(), locals: metamodelica::cons(imlicitTxtArg.clone(), funLocals.clone()), statements: list![metamodelica::Ref::new(MMExp::MM_MATCH { matchCases: mmmcases.clone() })], genInfoOpt: crate::TplAbsyn::GenInfo::GI_MATCH_FUN };
                    argvals = List::map(localArgs.clone(), &move |__a0: (ArcStr, metamodelica::Ref<TypeSignature>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(makeMMArgValue(&__a0)) })?;
                    argvals = metamodelica::cons(argval.clone(), argvals.clone());
                    Ok((argvals.clone(), metamodelica::Ref::new(PathIdent::IDENT { ident: fname.clone() }), iargs.clone(), oargs.clone(), scEnv.clone(), metamodelica::cons(mmFun.clone(), accMMDecls.clone())))
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
                    Debug::trace(literal!("-!!!makeMatchFun failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outArgvals, outFunName, outInArgs, outOutArgs, outScopeEnv, outMMDecls))
}

//no fail
pub(crate) fn alignExtArgsToScopeEnv(
    mut inExtraArgs: TypedIdents,
    mut inEncExtraArgs: TypedIdents,
    mut inScopeEnv: &ScopeEnv,
) -> (TypedIdents, TypedIdents) {
    let mut outExtraArgs: TypedIdents;
    let mut outEncExtraArgs: TypedIdents;
    (outExtraArgs, outEncExtraArgs) = 'mc: {
        let __mc_input = (inExtraArgs.clone(), inEncExtraArgs.clone(), &**inScopeEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (extargs, encExtargs, Deref @ metamodelica::ListNode::Cons { head: Scope::FUN_SCOPE { args: fargs, localArgs }, tail: _ }) => {
                    let mut extargsAligned: TypedIdents;
                    let mut encExtargsAligned: TypedIdents;
                    extargsAligned = alignTupleList(extargs.clone(), metamodelica::AsArg::as_arg(&fargs))?;
                    encExtargsAligned = alignTupleList(encExtargs.clone(), metamodelica::AsArg::as_arg(&localArgs))?;
                    let true = (((extargsAligned).len() as i32) == ((extargs).len() as i32)) else { return Err("pattern mismatch") };
                    let true = (((encExtargsAligned).len() as i32) == ((encExtargs).len() as i32)) else { return Err("pattern mismatch") };
                    Ok((extargsAligned.clone(), encExtargsAligned.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExtraArgs.clone(), inEncExtraArgs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExtraArgs, outEncExtraArgs)
}

//no fail
pub(crate) fn getMatchArgName(mut inArgExp: &Expression) -> (Ident, Ident) {
    let mut outInputValueName: Ident = arcstr::literal!("");
    let mut outMatchArgName: Ident = arcstr::literal!("");
    (outInputValueName, outMatchArgName) = 'mc: {
        let __mc_input = inArgExp;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ ExpressionBase::BOUND_VALUE { boundPath: path }, _) => {
                    let mut outInputValueName: ArcStr = outInputValueName.clone();
                    let mut outMatchArgName: ArcStr = outMatchArgName.clone();
                    outInputValueName = pathIdentString(metamodelica::AsArg::as_arg(&path))?;
                    outMatchArgName = encodeIdent(outInputValueName.clone(), &(arcstr::literal!(funArgNamePrefix)))?;
                    Ok(((outInputValueName.clone(), outMatchArgName.clone()), outInputValueName.clone(), outMatchArgName.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outInputValueName = __wb0;
            outMatchArgName = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((arcstr::literal!(impossibleIdent), arcstr::literal!(matchDefaultArgName)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outInputValueName, outMatchArgName)
}

//no fail
pub(crate) fn makeMMArgValue(
    mut inTypedIdent: &(ArcStr, metamodelica::Ref<TypeSignature>),
) -> (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo) {
    let mut outArgValue: (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo);
    outArgValue = (::match_deref::match_deref! { match &(inTypedIdent) {
        (argname, ts) => {
            (metamodelica::Ref::new(MMExp::MM_IDENT { ident: metamodelica::Ref::new(PathIdent::IDENT { ident: argname.clone() }) }), ts.clone(), dummySourceInfo.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outArgValue
}

pub(crate) fn isText(mut inArg: &(ArcStr, metamodelica::Ref<TypeSignature>)) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &(inArg) {
        (_, Deref @ TypeSignature::TEXT_TYPE { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

fn isAssignedText(
    mut inArg: &(ArcStr, metamodelica::Ref<TypeSignature>),
    mut inAssignedTexts: metamodelica::List<ArcStr>,
) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &((inArg, inAssignedTexts)) {
        ((ident, Deref @ TypeSignature::TEXT_TYPE { .. }), assignedTexts) if (listMember(ident.clone(), assignedTexts.clone())) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub(crate) fn elabMatchCases(
    mut inItArgVal: (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>),
    mut inImplicitValueName: &Ident,
    mut inMCases: &metamodelica::List<(
        metamodelica::Ref<MatchingExp>,
        (metamodelica::Ref<ExpressionBase>, SourceInfo),
    )>,
    mut hasImplicitLookup: bool,
    mut inLocals: TypedIdents,
    mut inAccCaseLocals: TypedIdents,
    mut inScopeEnv: ScopeEnv,
    mut inTplPackage: TemplPackage,
    mut inAccMMDecls: metamodelica::List<MMDeclaration>,
) -> Result<(
    metamodelica::List<(
        metamodelica::Ref<MatchingExp>,
        metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>,
        metamodelica::List<metamodelica::Ref<MMExp>>,
    )>,
    TypedIdents,
    ScopeEnv,
    metamodelica::List<MMDeclaration>,
    metamodelica::List<ArcStr>,
)> {
    let mut outMMMCases: metamodelica::List<(
        metamodelica::Ref<MatchingExp>,
        metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>,
        metamodelica::List<metamodelica::Ref<MMExp>>,
    )>;
    let mut outLocals: TypedIdents;
    let mut outScopeEnv: ScopeEnv;
    let mut outMMDecls: metamodelica::List<MMDeclaration>;
    let mut outAssignedIdents: metamodelica::List<ArcStr>;
    (outMMMCases, outLocals, outScopeEnv, outMMDecls, outAssignedIdents) = 'mc: {
        let __mc_input = (
            inItArgVal,
            &**inMCases,
            inLocals,
            inAccCaseLocals.clone(),
            inScopeEnv,
            inTplPackage,
            inAccMMDecls,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, locals, _, scEnv, _, accMMDecls) => {
                    let mut locals = (*locals).clone();
                    locals = listAppend(inAccCaseLocals.clone(), locals.clone());
                    Ok((metamodelica::nil(), locals.clone(), scEnv.clone(), accMMDecls.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argval @ (_, exptype), Deref @ metamodelica::ListNode::Cons { head: (mexp, exp), tail: mcases }, locals, accCaseLocals, scEnv, tplPackage @ TemplPackage { astDefs: astdefs, .. }, accMMDecls) => {
                    let mut extargs: TypedIdents;
                    let mut elabcases: metamodelica::List<(metamodelica::Ref<MatchingExp>, metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>, metamodelica::List<metamodelica::Ref<MMExp>>)>;
                    let mut stmts: metamodelica::List<metamodelica::Ref<MMExp>>;
                    let mut assignedIdents: metamodelica::List<ArcStr>;
                    let mut localNames: metamodelica::List<(ArcStr, ArcStr)>;
                    let mut mexp = (*mexp).clone();
                    let mut locals = (*locals).clone();
                    let mut accCaseLocals = (*accCaseLocals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    mexp = typeCheckMatchingExp(mexp.clone(), exptype.clone(), astdefs.clone())?;
                    (stmts, locals, scEnv, accMMDecls, _) = statementsFromExp(&(exp.clone()), metamodelica::nil(), metamodelica::nil(), arcstr::literal!(imlicitTxt), arcstr::literal!(imlicitTxt), locals.clone(), metamodelica::cons(Scope::CASE_SCOPE { mExp: mexp.clone(), mType: exptype.clone(), localNames: metamodelica::nil(), accLocals: accCaseLocals.clone(), extArgs: metamodelica::nil(), matchArgName: inImplicitValueName.clone(), hasImplicitScope: hasImplicitLookup }, scEnv.clone()), tplPackage.clone(), accMMDecls.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(scEnv.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Scope::CASE_SCOPE { mExp: __pa0, mType: _, localNames: __pa1, accLocals: __pa2, extArgs: __pa3, matchArgName: _, hasImplicitScope: _ }, tail: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    mexp = metamodelica::Own::own(__pa0);
                    localNames = metamodelica::Own::own(__pa1);
                    accCaseLocals = metamodelica::Own::own(__pa2);
                    extargs = metamodelica::Own::own(__pa3);
                    scEnv = metamodelica::Own::own(__pa4);
                    stmts = stmts.clone().reverse();
                    (mexp, _) = rewriteMatchExpByLocalNames(mexp.clone(), exptype.clone(), &localNames, metamodelica::nil(), astdefs.clone())?;
                    (elabcases, locals, scEnv, accMMDecls, assignedIdents) = elabMatchCases(argval.clone(), inImplicitValueName, metamodelica::AsArg::as_arg(&mcases), hasImplicitLookup, locals.clone(), accCaseLocals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    assignedIdents = getAssignedIdents(&stmts, assignedIdents.clone())?;
                    Ok((metamodelica::cons((mexp.clone(), extargs.clone(), stmts.clone()), elabcases.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), assignedIdents.clone()))
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
                    Debug::trace(literal!("-!!!elabMatchCases failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outMMMCases, outLocals, outScopeEnv, outMMDecls, outAssignedIdents))
}

pub(crate) fn getAssignedIdents(
    mut inStatements: &metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inAssignedIdents: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outAssignedIdents: metamodelica::List<ArcStr>;
    outAssignedIdents = 'mc: {
        let __mc_input = (&**inStatements, inAssignedIdents);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, assignedIdents) => {
                    Ok(assignedIdents.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ MMExp::MM_ASSIGN { lhsArgs: largs, .. }, tail: stmts }, assignedIdents) => {
                    let mut assignedIdents = (*assignedIdents).clone();
                    assignedIdents = List::fold(metamodelica::AsArg::as_arg(&largs), &fnptr!(List::unionElt, _, _), assignedIdents.clone())?;
                    Ok(getAssignedIdents(metamodelica::AsArg::as_arg(&stmts), assignedIdents.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: stmts }, assignedIdents) => {
                    Ok(getAssignedIdents(metamodelica::AsArg::as_arg(&stmts), assignedIdents.clone())?)
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
                    Debug::trace(literal!("-!!!getAssignedTexts failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAssignedIdents)
}

/*
public function getItNameFromArg
  input MMExp inItArgMMExp;
  input TypeSignature inMType;
  input MatchingExp inMatchingExp;
  input list<ASTDef> inASTDefs;

  output Ident outItName;
algorithm
  outItName := matchcontinue (inItArgMMExp, inMType, inMatchingExp, inASTDefs)
    local
      TypeSignature exptype;
      MatchingExp mexp;
      list<ASTDef> astdefs;
      MMExp mmexp;
      PathIdent path;
      Ident argid;

    //name it by the arg name if the name is not bound
    case ( MM_IDENT(path as IDENT(argid)), exptype, mexp, astdefs)
      algorithm
        //only when the argid is not yet bound by the user to do it explicit or hide the name from the upper scope
        failure( (_,_) = lookupUpdateMatchingExp(argid, path, mexp, exptype, astdefs) );
      then
        argid;

    //otherwise return "it" as the name
    else "it";

  end matchcontinue;
end getItNameFromArg;
*/
//fail and error
pub(crate) fn typeCheckMatchingExp(
    mut inMatchingExp: metamodelica::Ref<MatchingExp>,
    mut inMType: metamodelica::Ref<TypeSignature>,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<metamodelica::Ref<MatchingExp>> {
    let mut outTransformedMatchingExp: metamodelica::Ref<MatchingExp>;
    outTransformedMatchingExp = 'mc: {
        let __mc_input = (inMatchingExp, inMType, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: bid, matchingExp: mexp }, mtype, astDefs) => {
                    let mut mexp = (*mexp).clone();
                    mexp = typeCheckMatchingExp(mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok(metamodelica::Ref::new(MatchingExp::BIND_AS_MATCH { bindIdent: bid.clone(), matchingExp: mexp.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp @ Deref @ MatchingExp::BIND_MATCH { .. }, _, _) => {
                    Ok(mexp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp @ Deref @ MatchingExp::RECORD_MATCH { .. }, Deref @ TypeSignature::TEXT_TYPE { .. }, _) => {
                    Ok(mexp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::RECORD_MATCH { tagName: tagpath, fieldMatchings: fms }, mtype, astDefs) => {
                    let mut fields: TypedIdents;
                    let mut tagpath = (*tagpath).clone();
                    let mut fms = (*fms).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone());
                    (fields, tagpath) = getFieldsForRecord(metamodelica::AsArg::as_arg(&mtype), tagpath.clone(), astDefs.clone())?;
                    fms = typeCheckMatchingExpRecord(metamodelica::AsArg::as_arg(&fms), &fields, astDefs.clone())?;
                    Ok(metamodelica::Ref::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: fms.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::SOME_MATCH { value: mexp }, mtype, astDefs) => {
                    let mut mexp = (*mexp).clone();
                    let mut mtype = (*mtype).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone())) {
                        Deref @ TypeSignature::OPTION_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    mtype = metamodelica::Own::own(__pa0);
                    mexp = typeCheckMatchingExp(mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok(metamodelica::Ref::new(MatchingExp::SOME_MATCH { value: mexp.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp @ Deref @ MatchingExp::NONE_MATCH { .. }, mtype, astDefs) => {
                    ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone())) {
                        Deref @ TypeSignature::OPTION_TYPE { .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(mexp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::TUPLE_MATCH { tupleArgs: mexpLst }, mtype, astDefs) => {
                    let mut otLst: metamodelica::List<metamodelica::Ref<TypeSignature>>;
                    let mut mexpLst = (*mexpLst).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone())) {
                        Deref @ TypeSignature::TUPLE_TYPE { ofTypes: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    otLst = metamodelica::Own::own(__pa0);
                    mexpLst = typeCheckMatchingExpList(metamodelica::AsArg::as_arg(&mexpLst), &otLst, astDefs.clone())?;
                    Ok(metamodelica::Ref::new(MatchingExp::TUPLE_MATCH { tupleArgs: mexpLst.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::LIST_MATCH { listElts: mexpLst }, mtype, astDefs) => {
                    let mut ot: metamodelica::Ref<TypeSignature>;
                    let mut otLst: metamodelica::List<metamodelica::Ref<TypeSignature>>;
                    let mut mexpLst = (*mexpLst).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone())) {
                        Deref @ TypeSignature::LIST_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ot = metamodelica::Own::own(__pa0);
                    otLst = List::fill(ot.clone(), ((mexpLst).len() as i32));
                    mexpLst = typeCheckMatchingExpList(metamodelica::AsArg::as_arg(&mexpLst), &otLst, astDefs.clone())?;
                    Ok(metamodelica::Ref::new(MatchingExp::LIST_MATCH { listElts: mexpLst.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::LIST_CONS_MATCH { head: mexp, rest: restmexp }, mtype, astDefs) => {
                    let mut ot: metamodelica::Ref<TypeSignature>;
                    let mut mexp = (*mexp).clone();
                    let mut restmexp = (*restmexp).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(mtype.clone()) {
                        Deref @ TypeSignature::LIST_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ot = metamodelica::Own::own(__pa0);
                    mexp = typeCheckMatchingExp(mexp.clone(), ot.clone(), astDefs.clone())?;
                    restmexp = typeCheckMatchingExp(restmexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok(metamodelica::Ref::new(MatchingExp::LIST_CONS_MATCH { head: mexp.clone(), rest: restmexp.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp @ Deref @ MatchingExp::STRING_MATCH { .. }, mtype, astDefs) => {
                    ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone())) {
                        Deref @ TypeSignature::STRING_TYPE { .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(mexp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp @ Deref @ MatchingExp::LITERAL_MATCH { litType: ot, .. }, mtype, astDefs) => {
                    typesEqualConcrete(deAliasedType(metamodelica::AsArg::as_arg(&ot), astDefs.clone()), deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone()), astDefs.clone())?;
                    Ok(mexp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp @ Deref @ MatchingExp::REST_MATCH { .. }, _, _) => {
                    Ok(mexp.clone())
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
                    Debug::trace(literal!("Error - typeCheckMatchingExp failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTransformedMatchingExp)
}

pub(crate) fn typeCheckMatchingExpRecord(
    mut inFieldMatchings: &metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>,
    mut fields: &TypedIdents,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>> {
    let mut outTransformedMatchingExp: metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>;
    outTransformedMatchingExp = 'mc: {
        let __mc_input = (&**inFieldMatchings, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (ident, mexp), tail: fms }, astDefs) => {
                    let mut mtype: metamodelica::Ref<TypeSignature>;
                    let mut mexp = (*mexp).clone();
                    let mut fms = (*fms).clone();
                    mtype = lookupTupleList(fields.clone(), ident.clone())?;
                    mexp = typeCheckMatchingExp(mexp.clone(), mtype.clone(), astDefs.clone())?;
                    fms = typeCheckMatchingExpRecord(metamodelica::AsArg::as_arg(&fms), fields, astDefs.clone())?;
                    Ok(metamodelica::cons((ident.clone(), mexp.clone()), fms.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (ident, _), tail: _ }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(fields.clone(), ident.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - typeCheckMatchingExpRecord failed to find field '")); __mm_s.push_str(&*ident); __mm_s.push_str(&*literal!("'\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTransformedMatchingExp)
}

pub(crate) fn typeCheckMatchingExpList(
    mut inMatchingExpLst: &metamodelica::List<metamodelica::Ref<MatchingExp>>,
    mut inTypeLst: &metamodelica::List<metamodelica::Ref<TypeSignature>>,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<metamodelica::List<metamodelica::Ref<MatchingExp>>> {
    let mut outTransformedMatchingExp: metamodelica::List<metamodelica::Ref<MatchingExp>>;
    outTransformedMatchingExp = (::match_deref::match_deref! { match (inMatchingExpLst, inTypeLst) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::ListNode::Cons { head: mexp, tail: mexpLst }, Deref @ metamodelica::ListNode::Cons { head: mtype, tail: tsLst }) => {
            let mut astDefs = inASTDefs;
            let mut mexp = (*mexp).clone();
            let mut mexpLst = (*mexpLst).clone();
            mexp = typeCheckMatchingExp(mexp.clone(), mtype.clone(), astDefs.clone())?;
            mexpLst = typeCheckMatchingExpList(metamodelica::AsArg::as_arg(&mexpLst), tsLst, astDefs)?;
            metamodelica::cons(mexp.clone(), mexpLst.clone())
        },
        (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Nil) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::trace(literal!("Error - typeCheckMatchingExpList more expressions to chceck than required (a tuple type has less arguments than provided?).\n"))?;
            return Err("fail")
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::trace(literal!("Error - typeCheckMatchingExpList more arguments expected (the tuple type has more arguments than provided).\n"))?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outTransformedMatchingExp)
}

pub(crate) fn eliminateWildAs(mut inMatchingExp: metamodelica::Ref<MatchingExp>) -> metamodelica::Ref<MatchingExp> {
    let mut outRewrittenMatchingExp: metamodelica::Ref<MatchingExp>;
    outRewrittenMatchingExp = (::match_deref::match_deref! { match &(inMatchingExp.clone()) {
        Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: bid, matchingExp: Deref @ MatchingExp::REST_MATCH { .. } } => {
            metamodelica::Ref::new(MatchingExp::BIND_MATCH { bindIdent: bid.clone() })
        },
        _ => {
            inMatchingExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outRewrittenMatchingExp
}

pub(crate) fn rewriteMatchExpByLocalNames(
    mut inMatchingExp: metamodelica::Ref<MatchingExp>,
    mut inMType: metamodelica::Ref<TypeSignature>,
    mut inLocalNames: &metamodelica::List<(ArcStr, ArcStr)>,
    mut inUsedLocals: TypedIdents,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<(metamodelica::Ref<MatchingExp>, TypedIdents)> {
    let mut outRewrittenMatchingExp: metamodelica::Ref<MatchingExp>;
    let mut outUsedLocals: TypedIdents;
    (outRewrittenMatchingExp, outUsedLocals) = 'mc: {
        let __mc_input = (inMatchingExp, inMType, inUsedLocals, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: bid, matchingExp: mexp }, mtype, usedLocals, astDefs) => {
                    let mut localIdent: Ident;
                    let mut mexp = (*mexp).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    localIdent = lookupTupleList(inLocalNames.clone(), bid.clone())?;
                    usedLocals = addLocalValue(bid.clone(), mtype.clone(), usedLocals.clone())?;
                    (mexp, usedLocals) = rewriteMatchExpByLocalNames(mexp.clone(), mtype.clone(), inLocalNames, usedLocals.clone(), astDefs.clone())?;
                    mexp = eliminateWildAs(metamodelica::Ref::new(MatchingExp::BIND_AS_MATCH { bindIdent: localIdent.clone(), matchingExp: mexp.clone() }));
                    Ok((mexp.clone(), usedLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: bid, matchingExp: mexp }, mtype, usedLocals, astDefs) => {
                    let mut mexp = (*mexp).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(inLocalNames.clone(), bid.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    (mexp, usedLocals) = rewriteMatchExpByLocalNames(mexp.clone(), mtype.clone(), inLocalNames, usedLocals.clone(), astDefs.clone())?;
                    Ok((mexp.clone(), usedLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::BIND_MATCH { bindIdent: bid }, mtype, usedLocals, _) => {
                    let mut localIdent: Ident;
                    let mut usedLocals = (*usedLocals).clone();
                    localIdent = lookupTupleList(inLocalNames.clone(), bid.clone())?;
                    usedLocals = addLocalValue(bid.clone(), mtype.clone(), usedLocals.clone())?;
                    Ok((metamodelica::Ref::new(MatchingExp::BIND_MATCH { bindIdent: localIdent.clone() }), usedLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::BIND_MATCH { bindIdent: bid }, _, usedLocals, _) => {
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(inLocalNames.clone(), bid.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok((crate::TplAbsyn::MatchingExp::interned_REST_MATCH(), usedLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::RECORD_MATCH { tagName: tagpath, fieldMatchings: Deref @ metamodelica::ListNode::Nil }, mtype, usedLocals, astDefs) => {
                    let mut fldId: Ident;
                    let mut tagpath = (*tagpath).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone());
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getFieldsForRecord(metamodelica::AsArg::as_arg(&mtype), tagpath.clone(), astDefs.clone())?) {
                        (Deref @ metamodelica::ListNode::Cons { head: (__pa0, _), tail: _ }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    fldId = metamodelica::Own::own(__pa0);
                    tagpath = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: list![(fldId.clone(), crate::TplAbsyn::MatchingExp::interned_REST_MATCH())] }), usedLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::RECORD_MATCH { tagName: tagpath, fieldMatchings: fms }, mtype, usedLocals, astDefs) => {
                    let mut fields: TypedIdents;
                    let mut tagpath = (*tagpath).clone();
                    let mut fms = (*fms).clone();
                    let mut mtype = (*mtype).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    mtype = deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone());
                    (fields, tagpath) = getFieldsForRecord(metamodelica::AsArg::as_arg(&mtype), tagpath.clone(), astDefs.clone())?;
                    (fms, usedLocals) = rewriteMatchExpByLocalNamesRecord(metamodelica::AsArg::as_arg(&fms), &fields, inLocalNames, usedLocals.clone(), astDefs.clone())?;
                    Ok((metamodelica::Ref::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: fms.clone() }), usedLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::SOME_MATCH { value: mexp }, mtype, usedLocals, astDefs) => {
                    let mut mexp = (*mexp).clone();
                    let mut mtype = (*mtype).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone())) {
                        Deref @ TypeSignature::OPTION_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    mtype = metamodelica::Own::own(__pa0);
                    (mexp, usedLocals) = rewriteMatchExpByLocalNames(mexp.clone(), mtype.clone(), inLocalNames, usedLocals.clone(), astDefs.clone())?;
                    Ok((metamodelica::Ref::new(MatchingExp::SOME_MATCH { value: mexp.clone() }), usedLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::TUPLE_MATCH { tupleArgs: mexpLst }, mtype, usedLocals, astDefs) => {
                    let mut otLst: metamodelica::List<metamodelica::Ref<TypeSignature>>;
                    let mut mexpLst = (*mexpLst).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone())) {
                        Deref @ TypeSignature::TUPLE_TYPE { ofTypes: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    otLst = metamodelica::Own::own(__pa0);
                    (mexpLst, usedLocals) = rewriteMatchExpByLocalNamesList(metamodelica::AsArg::as_arg(&mexpLst), &otLst, inLocalNames, usedLocals.clone(), astDefs.clone())?;
                    Ok((metamodelica::Ref::new(MatchingExp::TUPLE_MATCH { tupleArgs: mexpLst.clone() }), usedLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::LIST_MATCH { listElts: mexpLst }, mtype, usedLocals, astDefs) => {
                    let mut ot: metamodelica::Ref<TypeSignature>;
                    let mut otLst: metamodelica::List<metamodelica::Ref<TypeSignature>>;
                    let mut mexpLst = (*mexpLst).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone())) {
                        Deref @ TypeSignature::LIST_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ot = metamodelica::Own::own(__pa0);
                    otLst = List::fill(ot.clone(), ((mexpLst).len() as i32));
                    (mexpLst, usedLocals) = rewriteMatchExpByLocalNamesList(metamodelica::AsArg::as_arg(&mexpLst), &otLst, inLocalNames, usedLocals.clone(), astDefs.clone())?;
                    Ok((metamodelica::Ref::new(MatchingExp::LIST_MATCH { listElts: mexpLst.clone() }), usedLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::LIST_CONS_MATCH { head: mexp, rest: restmexp }, mtype, usedLocals, astDefs) => {
                    let mut ot: metamodelica::Ref<TypeSignature>;
                    let mut mexp = (*mexp).clone();
                    let mut restmexp = (*restmexp).clone();
                    let mut mtype = (*mtype).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    mtype = deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(mtype.clone()) {
                        Deref @ TypeSignature::LIST_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ot = metamodelica::Own::own(__pa0);
                    (mexp, usedLocals) = rewriteMatchExpByLocalNames(mexp.clone(), ot.clone(), inLocalNames, usedLocals.clone(), astDefs.clone())?;
                    (restmexp, usedLocals) = rewriteMatchExpByLocalNames(restmexp.clone(), mtype.clone(), inLocalNames, usedLocals.clone(), astDefs.clone())?;
                    Ok((metamodelica::Ref::new(MatchingExp::LIST_CONS_MATCH { head: mexp.clone(), rest: restmexp.clone() }), usedLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp, _, usedLocals, _) => {
                    Ok((mexp.clone(), usedLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outRewrittenMatchingExp, outUsedLocals))
}

pub(crate) fn rewriteMatchExpByLocalNamesRecord(
    mut inFieldMatchings: &metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>,
    mut fields: &TypedIdents,
    mut inLocalNames: &metamodelica::List<(ArcStr, ArcStr)>,
    mut inUsedLocals: TypedIdents,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<(
    metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>,
    TypedIdents,
)> {
    let mut outRewrittenMatchingExp: metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>;
    let mut outUsedLocals: TypedIdents;
    (outRewrittenMatchingExp, outUsedLocals) = 'mc: {
        let __mc_input = (&**inFieldMatchings, inUsedLocals.clone(), inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok((metamodelica::nil(), inUsedLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (ident, mexp), tail: fms }, usedLocals, astDefs) => {
                    let mut mtype: metamodelica::Ref<TypeSignature>;
                    let mut mexp = (*mexp).clone();
                    let mut fms = (*fms).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    mtype = lookupTupleList(fields.clone(), ident.clone())?;
                    (mexp, usedLocals) = rewriteMatchExpByLocalNames(mexp.clone(), mtype.clone(), inLocalNames, usedLocals.clone(), astDefs.clone())?;
                    (fms, usedLocals) = rewriteMatchExpByLocalNamesRecord(metamodelica::AsArg::as_arg(&fms), fields, inLocalNames, usedLocals.clone(), astDefs.clone())?;
                    Ok((metamodelica::cons((ident.clone(), mexp.clone()), fms.clone()), usedLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (ident, mexp), tail: fms }, usedLocals, astDefs) => {
                    let mut fms = (*fms).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(fields.clone(), ident.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - rewriteMatchExpByLocalNamesRecord failed to find field '")); __mm_s.push_str(&*ident); __mm_s.push_str(&*literal!("'\n")); ArcStr::from(__mm_s) })?;
                    }
                    (fms, usedLocals) = rewriteMatchExpByLocalNamesRecord(metamodelica::AsArg::as_arg(&fms), fields, inLocalNames, usedLocals.clone(), astDefs.clone())?;
                    Ok((metamodelica::cons((ident.clone(), mexp.clone()), fms.clone()), usedLocals.clone()))
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
                    Debug::trace(literal!("-!!!rewriteMatchExpByLocalNamesRecord failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outRewrittenMatchingExp, outUsedLocals))
}

pub(crate) fn rewriteMatchExpByLocalNamesList(
    mut inMatchingExpLst: &metamodelica::List<metamodelica::Ref<MatchingExp>>,
    mut inTypeLst: &metamodelica::List<metamodelica::Ref<TypeSignature>>,
    mut inLocalNames: &metamodelica::List<(ArcStr, ArcStr)>,
    mut inUsedLocals: TypedIdents,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<(metamodelica::List<metamodelica::Ref<MatchingExp>>, TypedIdents)> {
    let mut outRewrittenMatchingExp: metamodelica::List<metamodelica::Ref<MatchingExp>>;
    let mut outUsedLocals: TypedIdents;
    (outRewrittenMatchingExp, outUsedLocals) = 'mc: {
        let __mc_input = (&**inMatchingExpLst, &**inTypeLst, inUsedLocals, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, usedLocals, _) => {
                    Ok((metamodelica::nil(), usedLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: mexp, tail: mexpLst }, Deref @ metamodelica::ListNode::Cons { head: mtype, tail: tsLst }, usedLocals, astDefs) => {
                    let mut mexp = (*mexp).clone();
                    let mut mexpLst = (*mexpLst).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    (mexp, usedLocals) = rewriteMatchExpByLocalNames(mexp.clone(), mtype.clone(), inLocalNames, usedLocals.clone(), astDefs.clone())?;
                    (mexpLst, usedLocals) = rewriteMatchExpByLocalNamesList(metamodelica::AsArg::as_arg(&mexpLst), metamodelica::AsArg::as_arg(&tsLst), inLocalNames, usedLocals.clone(), astDefs.clone())?;
                    Ok((metamodelica::cons(mexp.clone(), mexpLst.clone()), usedLocals.clone()))
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
                    Debug::trace(literal!("-!!!localsFromMatchExpList failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outRewrittenMatchingExp, outUsedLocals))
}

pub(crate) fn addLocalValue(
    mut inIdent: Ident,
    mut inMType: metamodelica::Ref<TypeSignature>,
    mut inLocals: TypedIdents,
) -> Result<TypedIdents> {
    let mut outLocals: TypedIdents;
    outLocals = 'mc: {
        let __mc_input = (inIdent, inMType, inLocals);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, Deref @ TypeSignature::TEXT_TYPE { .. }, locals) => {
                    let true = (stringEq(&ident, &arcstr::literal!(emptyTxt))) else { return Err("pattern mismatch") };
                    Ok(locals.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, mtype, locals) => {
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(locals.clone(), ident.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok(metamodelica::cons((ident.clone(), mtype.clone()), locals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, mtype, locals) => {
                    let mut msg: ArcStr;
                    lookupTupleList(locals.clone(), ident.clone())?;
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("A duplicite identifier '")); __mm_s.push_str(&*ident); __mm_s.push_str(&*literal!("' bound in a matching expression.")); ArcStr::from(__mm_s) };
                    addSusanError(msg.clone(), &(dummySourceInfo.clone()))?;
                    Ok(metamodelica::cons((ident.clone(), mtype.clone()), locals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outLocals)
}

pub(crate) fn makeMMMatchCase(
    mut inElabCase: &(
        metamodelica::Ref<MatchingExp>,
        metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>,
        metamodelica::List<metamodelica::Ref<MMExp>>,
    ),
    mut inExtraArgs: TypedIdents,
    mut inOutArgs: TypedIdents,
) -> Result<MMMatchCase> {
    let mut outMMMCase: MMMatchCase;
    outMMMCase = 'mc: {
        let __mc_input = (inElabCase, inExtraArgs, inOutArgs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((mexp, caseargs, stmts), extargs, oargs) => {
                    let mut mmmcase: MMMatchCase;
                    let mut mexpLst: metamodelica::List<metamodelica::Ref<MatchingExp>>;
                    mexpLst = List::map2(extargs.clone(), &move |__a0: (ArcStr, metamodelica::Ref<TypeSignature>), __a1: metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>, __a2: metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>| makeExtraArgBinding(&__a0, __a1, __a2), caseargs.clone(), oargs.clone())?;
                    mmmcase = (metamodelica::cons(imlicitTxtMExp.clone(), metamodelica::cons(mexp.clone(), mexpLst.clone())), stmts.clone());
                    Ok(mmmcase.clone())
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
                    Debug::trace(literal!("-!!!makeMMMatchCase failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMMMCase)
}

pub(crate) fn makeExtraArgBinding(
    mut inExtraArg: &(ArcStr, metamodelica::Ref<TypeSignature>),
    mut inCaseArgs: TypedIdents,
    mut inOutArgs: TypedIdents,
) -> Result<metamodelica::Ref<MatchingExp>> {
    let mut outExtraArgBinding: metamodelica::Ref<MatchingExp>;
    outExtraArgBinding = 'mc: {
        let __mc_input = (inExtraArg, inCaseArgs, inOutArgs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((argname, _), _, oargs) => {
                    lookupTupleList(oargs.clone(), argname.clone())?;
                    Ok(metamodelica::Ref::new(MatchingExp::BIND_MATCH { bindIdent: argname.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((argname, _), caseargs, _) => {
                    lookupTupleList(caseargs.clone(), argname.clone())?;
                    Ok(metamodelica::Ref::new(MatchingExp::BIND_MATCH { bindIdent: argname.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    Ok(crate::TplAbsyn::MatchingExp::interned_REST_MATCH())
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
                    Debug::trace(literal!("-!!!makeExtraArgBinding failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExtraArgBinding)
}

pub(crate) fn addRestElabCase(
    mut inElabCases: metamodelica::List<(
        metamodelica::Ref<MatchingExp>,
        metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>,
        metamodelica::List<metamodelica::Ref<MMExp>>,
    )>,
) -> metamodelica::List<(
    metamodelica::Ref<MatchingExp>,
    metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>,
    metamodelica::List<metamodelica::Ref<MMExp>>,
)> {
    let mut outElabCases: metamodelica::List<(
        metamodelica::Ref<MatchingExp>,
        metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>,
        metamodelica::List<metamodelica::Ref<MMExp>>,
    )>;
    outElabCases = 'mc: {
        let __mc_input = inElabCases;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(list![(crate::TplAbsyn::MatchingExp::interned_REST_MATCH(), metamodelica::nil(), metamodelica::nil())])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                restcases @ Deref @ metamodelica::ListNode::Cons { head: (mexp, _, _), tail: _ } => {
                    isAlwaysMatched(metamodelica::AsArg::as_arg(&mexp))?;
                    Ok(restcases.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: elabcase @ _, tail: restcases } => {
                    let mut restcases = (*restcases).clone();
                    restcases = addRestElabCase(restcases.clone());
                    Ok(metamodelica::cons(elabcase.clone(), restcases.clone()))
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
                    Debug::trace(literal!("-!!!addRestElabCase failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outElabCases
}

pub(crate) fn isAlwaysMatched(mut inMatchingExp: &metamodelica::Ref<MatchingExp>) -> Result<()> {
    let () = (match &**inMatchingExp {
        MatchingExp::BIND_AS_MATCH { matchingExp: mexp, .. } => {
            isAlwaysMatched(mexp)?;
            ()
        }
        MatchingExp::BIND_MATCH { .. } => (),
        MatchingExp::TUPLE_MATCH { tupleArgs: mexplst } => {
            List::map_0(mexplst, &move |__a0: metamodelica::Ref<MatchingExp>| {
                isAlwaysMatched(&__a0)
            })?;
            ()
        }
        MatchingExp::REST_MATCH { .. } => (),
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub(crate) fn isAlwaysMatchedBool(mut inMatchingExp: metamodelica::Ref<MatchingExp>) -> bool {
    let mut isAlwaysMatched: bool;
    isAlwaysMatched = 'mc: {
        let __mc_input = inMatchingExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                mexp => {
                    self::isAlwaysMatched(metamodelica::AsArg::as_arg(&mexp))?;
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
    isAlwaysMatched
}

fn isTextType(mut ts: &metamodelica::Ref<TypeSignature>) -> bool {
    let mut b: bool;
    b = (match &**ts {
        TypeSignature::TEXT_TYPE { .. } => true,
        _ => false,
    });
    b
}

fn textConditionToIsEmpty(
    mut inArgValue: &(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo),
    mut stmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut locals: TypedIdents,
) -> Result<(
    metamodelica::List<metamodelica::Ref<MMExp>>,
    TypedIdents,
    (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo),
)> {
    let mut stmts: metamodelica::List<metamodelica::Ref<MMExp>> = stmts;
    let mut locals: TypedIdents = locals;
    let mut outArgValue: (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo);
    let mut mmexp: metamodelica::Ref<MMExp>;
    let mut sinfo: SourceInfo;
    let mut retid: Ident;
    (mmexp, _, sinfo) = inArgValue.clone();
    retid = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*arcstr::literal!(returnTempVarNamePrefix));
        __mm_s.push_str(&*intString(((locals).len() as i32)));
        ArcStr::from(__mm_s)
    };
    locals = addLocalValue(
        retid.clone(),
        crate::TplAbsyn::TypeSignature::interned_BOOLEAN_TYPE(),
        locals,
    )?;
    stmts = metamodelica::cons(
        metamodelica::Ref::new(MMExp::MM_ASSIGN {
            lhsArgs: list![retid.clone()],
            rhs: metamodelica::Ref::new(MMExp::MM_FN_CALL {
                fnName: metamodelica::Ref::new(PathIdent::PATH_IDENT {
                    ident: literal!("Tpl"),
                    path: metamodelica::Ref::new(PathIdent::IDENT {
                        ident: literal!("isEmpty"),
                    }),
                }),
                args: list![mmexp],
            }),
        }),
        stmts,
    );
    outArgValue = (
        metamodelica::Ref::new(MMExp::MM_IDENT {
            ident: metamodelica::Ref::new(PathIdent::IDENT { ident: retid }),
        }),
        crate::TplAbsyn::TypeSignature::interned_BOOLEAN_TYPE(),
        sinfo,
    );
    Ok((stmts, locals, outArgValue))
}

pub(crate) fn adaptTextToString(
    mut inArgValue: (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo),
    mut inArgExp: Expression,
    mut inStmts: metamodelica::List<metamodelica::Ref<MMExp>>,
    mut inLocals: TypedIdents,
    mut inTplPackage: &TemplPackage,
) -> Result<(
    (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo),
    Expression,
    metamodelica::List<metamodelica::Ref<MMExp>>,
    TypedIdents,
)> {
    let mut outArgValue: (metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, SourceInfo);
    let mut outArgExp: Expression;
    let mut outStmts: metamodelica::List<metamodelica::Ref<MMExp>>;
    let mut outLocals: TypedIdents;
    (outArgValue, outArgExp, outStmts, outLocals) = 'mc: {
        let __mc_input = (inArgValue, inStmts, inLocals, inTplPackage);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((mmexp, exptype, sinfo), stmts, locals, TemplPackage { astDefs: astdefs, .. }) => {
                    let mut stmt: metamodelica::Ref<MMExp>;
                    let mut strid: Ident;
                    let mut mmexp = (*mmexp).clone();
                    let mut locals = (*locals).clone();
                    ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&exptype), astdefs.clone())) {
                        Deref @ TypeSignature::TEXT_TYPE { .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    strid = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(textToStringNamePrefix)); __mm_s.push_str(&*intString(((locals).len() as i32))); ArcStr::from(__mm_s) };
                    locals = addLocalValue(strid.clone(), crate::TplAbsyn::TypeSignature::interned_STRING_TYPE(), locals.clone())?;
                    mmexp = mmExpToString(mmexp.clone(), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE(), metamodelica::AsArg::as_arg(&sinfo))?;
                    stmt = metamodelica::Ref::new(MMExp::MM_ASSIGN { lhsArgs: list![strid.clone()], rhs: mmexp.clone() });
                    Ok(((metamodelica::Ref::new(MMExp::MM_IDENT { ident: metamodelica::Ref::new(PathIdent::IDENT { ident: strid.clone() }) }), crate::TplAbsyn::TypeSignature::interned_STRING_TYPE(), sinfo.clone()), emptyExpression.clone(), metamodelica::cons(stmt.clone(), stmts.clone()), locals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argval, stmts, locals, _) => {
                    Ok((argval.clone(), inArgExp.clone(), stmts.clone(), locals.clone()))
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
                    Debug::trace(literal!("-!!!adaptTextToString failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outArgValue, outArgExp, outStmts, outLocals))
}

pub(crate) fn elabCasesFromCondition(
    mut inArgType: &metamodelica::Ref<TypeSignature>,
    mut inIsNot: bool,
    mut inRhsValue: Option<metamodelica::Ref<MatchingExp>>,
    mut inTrueBranch: Expression,
    mut inElseBranchOpt: Option<(metamodelica::Ref<ExpressionBase>, SourceInfo)>,
    mut inTplPackage: &TemplPackage,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<MatchingExp>,
        (metamodelica::Ref<ExpressionBase>, SourceInfo),
    )>,
> {
    let mut outMCases: metamodelica::List<(
        metamodelica::Ref<MatchingExp>,
        (metamodelica::Ref<ExpressionBase>, SourceInfo),
    )>;
    outMCases = 'mc: {
        let __mc_input = (&**inArgType, inIsNot, inRhsValue, inTrueBranch, inElseBranchOpt);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::LIST_TYPE { .. }, isnot, None, tbranch, ebranchOpt) => {
                    Ok(casesForTrueFalseCondition(isnot.clone(), metamodelica::Ref::new(MatchingExp::LIST_MATCH { listElts: metamodelica::nil() }), tbranch.clone(), ebranchOpt.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::OPTION_TYPE { .. }, isnot, None, tbranch, ebranchOpt) => {
                    Ok(casesForTrueFalseCondition(isnot.clone(), crate::TplAbsyn::MatchingExp::interned_NONE_MATCH(), tbranch.clone(), ebranchOpt.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::STRING_TYPE { .. }, isnot, None, tbranch, ebranchOpt) => {
                    Ok(casesForTrueFalseCondition(isnot.clone(), metamodelica::Ref::new(MatchingExp::STRING_MATCH { value: literal!("") }), tbranch.clone(), ebranchOpt.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::INTEGER_TYPE { .. }, isnot, None, tbranch, ebranchOpt) => {
                    Ok(casesForTrueFalseCondition(isnot.clone(), metamodelica::Ref::new(MatchingExp::LITERAL_MATCH { value: literal!("0"), litType: crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE() }), tbranch.clone(), ebranchOpt.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::REAL_TYPE { .. }, isnot, None, tbranch, ebranchOpt) => {
                    Ok(casesForTrueFalseCondition(isnot.clone(), metamodelica::Ref::new(MatchingExp::LITERAL_MATCH { value: literal!("0.0"), litType: crate::TplAbsyn::TypeSignature::interned_REAL_TYPE() }), tbranch.clone(), ebranchOpt.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::BOOLEAN_TYPE { .. }, isnot, None, tbranch, ebranchOpt) => {
                    Ok(casesForTrueFalseCondition(isnot.clone(), metamodelica::Ref::new(MatchingExp::LITERAL_MATCH { value: literal!("false"), litType: crate::TplAbsyn::TypeSignature::interned_BOOLEAN_TYPE() }), tbranch.clone(), ebranchOpt.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::TEXT_TYPE { .. }, isnot, None, tbranch, ebranchOpt) => {
                    Ok(casesForTrueFalseCondition(isnot.clone(), metamodelica::Ref::new(MatchingExp::LITERAL_MATCH { value: literal!("true"), litType: crate::TplAbsyn::TypeSignature::interned_BOOLEAN_TYPE() }), tbranch.clone(), ebranchOpt.clone())?)
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
                    Debug::trace(literal!("-!!!elabCasesFromCondition failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMCases)
}

pub(crate) fn casesForTrueFalseCondition(
    mut inIsNot: bool,
    mut inNotMatchingExp: metamodelica::Ref<MatchingExp>,
    mut inTrueBranch: Expression,
    mut inElseBranchOpt: Option<(metamodelica::Ref<ExpressionBase>, SourceInfo)>,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<MatchingExp>,
        (metamodelica::Ref<ExpressionBase>, SourceInfo),
    )>,
> {
    let mut outMCases: metamodelica::List<(
        metamodelica::Ref<MatchingExp>,
        (metamodelica::Ref<ExpressionBase>, SourceInfo),
    )>;
    outMCases = 'mc: {
        let __mc_input = (inIsNot, inNotMatchingExp, inTrueBranch, inElseBranchOpt);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, notmexp, tbranch, ebranchOpt) => {
                    let mut ebranch: Expression;
                    ebranch = getElseBranch(ebranchOpt.clone())?;
                    Ok(list![(notmexp.clone(), ebranch.clone()), (crate::TplAbsyn::MatchingExp::interned_REST_MATCH(), tbranch.clone())])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, notmexp, tbranch, ebranchOpt) => {
                    let mut ebranch: Expression;
                    ebranch = getElseBranch(ebranchOpt.clone())?;
                    Ok(list![(notmexp.clone(), tbranch.clone()), (crate::TplAbsyn::MatchingExp::interned_REST_MATCH(), ebranch.clone())])
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
                    Debug::trace(literal!("-!!!casesForTrueFalseCondition failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMCases)
}

pub(crate) fn getElseBranch(
    mut inElseBranchOpt: Option<(metamodelica::Ref<ExpressionBase>, SourceInfo)>,
) -> Result<Expression> {
    let mut outElseBranch: Expression;
    outElseBranch = (::match_deref::match_deref! { match &(inElseBranchOpt) {
        Some(ebranch) => {
            ebranch.clone()
        },
        None => {
            emptyExpression.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::trace(literal!("-!!!getElseBranch failed\n"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElseBranch)
}

//does not fail, when not resolved ... UNRESOLVED_TYPE() is returned
pub(crate) fn resolveBoundPath(
    mut inPath: metamodelica::Ref<PathIdent>,
    mut inScopeEnv: ScopeEnv,
    mut inTplPackage: &TemplPackage,
) -> Result<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>, ScopeEnv)> {
    let mut outMMExp: metamodelica::Ref<MMExp>;
    let mut outType: metamodelica::Ref<TypeSignature>;
    let mut outScopeEnv: ScopeEnv;
    (outMMExp, outType, outScopeEnv) = 'mc: {
        let __mc_input = (inPath, inScopeEnv, inTplPackage);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, scEnv, TemplPackage { astDefs, .. }) => {
                    let mut ident: Ident;
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut scEnv = (*scEnv).clone();
                    ident = pathIdentString(metamodelica::AsArg::as_arg(&path))?;
                    (ident, idtype, scEnv) = resolvePathInScopeEnv(ident.clone(), path.clone(), true, scEnv.clone(), astDefs.clone())?;
                    Ok((metamodelica::Ref::new(MMExp::MM_IDENT { ident: metamodelica::Ref::new(PathIdent::IDENT { ident: ident.clone() }) }), idtype.clone(), scEnv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ PathIdent::IDENT { ident }, scEnv, TemplPackage { templateDefs: tpldefs, .. }) => {
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut tpldef: TemplateDef;
                    let mut mmexp: metamodelica::Ref<MMExp>;
                    tpldef = lookupTupleList(tpldefs.clone(), ident.clone())?;
                    (mmexp, idtype) = makeMMExpFromTemplateConstant(&tpldef, ident.clone())?;
                    Ok((mmexp.clone(), idtype.clone(), scEnv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, scEnv, TemplPackage { astDefs, .. }) => {
                    let mut typepckg: metamodelica::Ref<PathIdent>;
                    let mut typeident: Ident;
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut typepckgOpt: Option<metamodelica::Ref<PathIdent>>;
                    let mut path = (*path).clone();
                    (typepckgOpt, typeident) = splitPackageAndIdent(metamodelica::AsArg::as_arg(&path))?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getTypeInfo(typepckgOpt.clone(), typeident.clone(), astDefs.clone())?) {
                        (__pa0, TypeInfo::TI_CONST_TYPE { constType: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    typepckg = metamodelica::Own::own(__pa0);
                    idtype = metamodelica::Own::own(__pa1);
                    path = makePathIdent(&typepckg, typeident.clone())?;
                    Ok((metamodelica::Ref::new(MMExp::MM_IDENT { ident: path.clone() }), idtype.clone(), scEnv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, scEnv, TemplPackage { astDefs, .. }) => {
                    let mut typepckg: metamodelica::Ref<PathIdent>;
                    let mut typeident: Ident;
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut typepckgOpt: Option<metamodelica::Ref<PathIdent>>;
                    let mut reason: ArcStr;
                    let mut path = (*path).clone();
                    (typepckgOpt, typeident) = splitPackageAndIdent(metamodelica::AsArg::as_arg(&path))?;
                    (typepckg, _) = getTypeInfo(typepckgOpt.clone(), typeident.clone(), astDefs.clone())?;
                    reason = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved path - imported symbol '")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&path))?); __mm_s.push_str(&*literal!("' other than a constant used in a value context (missing parenthesis ?).")); ArcStr::from(__mm_s) };
                    idtype = metamodelica::Ref::new(TypeSignature::UNRESOLVED_TYPE { reason: reason.clone() });
                    path = makePathIdent(&typepckg, typeident.clone())?;
                    Ok((metamodelica::Ref::new(MMExp::MM_IDENT { ident: path.clone() }), idtype.clone(), scEnv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, scEnv, _) => {
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut reason: ArcStr;
                    reason = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved path '")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&path))?); __mm_s.push_str(&*literal!("'.")); ArcStr::from(__mm_s) };
                    idtype = metamodelica::Ref::new(TypeSignature::UNRESOLVED_TYPE { reason: reason.clone() });
                    Ok((metamodelica::Ref::new(MMExp::MM_IDENT { ident: path.clone() }), idtype.clone(), scEnv.clone()))
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
                    Debug::trace(literal!("-!!!resolveBoundPath failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outMMExp, outType, outScopeEnv))
}

pub(crate) fn checkResolvedType(
    mut inPath: &metamodelica::Ref<PathIdent>,
    mut inType: &metamodelica::Ref<TypeSignature>,
    mut inUnresolvedMsg: ArcStr,
    mut inInfo: &SourceInfo,
) -> () {
    let () = 'mc: {
        let __mc_input = (&**inType, inUnresolvedMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::UNRESOLVED_TYPE { reason }, msg) => {
                    let mut msg = (*msg).clone();
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*msg); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*reason); ArcStr::from(__mm_s) };
                    addSusanError(msg.clone(), inInfo)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

pub(crate) fn checkTextType(
    mut inType: metamodelica::Ref<TypeSignature>,
    mut inIdent: &Ident,
    mut inUnresolvedMsg: ArcStr,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<TypeSignature>> {
    let mut outType: metamodelica::Ref<TypeSignature>;
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ TypeSignature::TEXT_TYPE { .. } => {
            inType
        },
        Deref @ TypeSignature::UNRESOLVED_TYPE { .. } => {
            inType
        },
        ts => {
            let mut msg = inUnresolvedMsg;
            msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*msg); __mm_s.push_str(&*literal!(") identifier '")); __mm_s.push_str(&*inIdent); __mm_s.push_str(&*literal!("' was expected to have Text& type but resolved to ")); __mm_s.push_str(&*typeSignatureString(metamodelica::AsArg::as_arg(&ts))?); __mm_s.push_str(&*literal!(".\n Only Text& typed variables can be appended to.")); ArcStr::from(__mm_s) };
            addSusanError(msg.clone(), inInfo)?;
            metamodelica::Ref::new(TypeSignature::UNRESOLVED_TYPE { reason: msg })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

pub(crate) fn makeMMExpFromTemplateConstant(
    mut inTplDef: &TemplateDef,
    mut inTemplIdent: Ident,
) -> Result<(metamodelica::Ref<MMExp>, metamodelica::Ref<TypeSignature>)> {
    let mut outMMExp: metamodelica::Ref<MMExp>;
    let mut outConstType: metamodelica::Ref<TypeSignature>;
    (outMMExp, outConstType) = (match (inTplDef.clone(), inTemplIdent) {
        (TemplateDef::STR_TOKEN_DEF { .. }, mut ident) => {
            ident = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*arcstr::literal!(constantNamePrefix));
                __mm_s.push_str(&*ident);
                ArcStr::from(__mm_s)
            };
            (
                metamodelica::Ref::new(MMExp::MM_IDENT {
                    ident: metamodelica::Ref::new(PathIdent::IDENT { ident: ident }),
                }),
                crate::TplAbsyn::TypeSignature::interned_STRING_TOKEN_TYPE(),
            )
        }
        (
            TemplateDef::LITERAL_DEF {
                value: mut litstr,
                litType: ref lt,
            },
            _,
        ) => (
            metamodelica::Ref::new(MMExp::MM_LITERAL { value: litstr.clone() }),
            lt.clone(),
        ),
        (TemplateDef::TEMPLATE_DEF { .. }, mut ident) => {
            let mut idtype: metamodelica::Ref<TypeSignature>;
            let mut reason: ArcStr;
            reason = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Unresolved identifier - the template '"));
                __mm_s.push_str(&*ident);
                __mm_s.push_str(&*literal!("'in a value context found (missing parenthesis ?) ."));
                ArcStr::from(__mm_s)
            };
            idtype = metamodelica::Ref::new(TypeSignature::UNRESOLVED_TYPE { reason: reason });
            (
                metamodelica::Ref::new(MMExp::MM_IDENT {
                    ident: metamodelica::Ref::new(PathIdent::IDENT { ident: ident }),
                }),
                idtype,
            )
        }
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("-!!!makeMMExpFromTemplateConstant failed\n"))?;
            return Err("fail");
        }
    });
    Ok((outMMExp, outConstType))
}

pub(crate) fn prepareMatchArgument(
    mut inMExp: metamodelica::Ref<MatchingExp>,
    mut inMatchArgName: Ident,
) -> (Ident, metamodelica::Ref<MatchingExp>) {
    let mut outIdent: Ident;
    let mut outMExp: metamodelica::Ref<MatchingExp>;
    (outIdent, outMExp) = (::match_deref::match_deref! { match &(inMExp.clone()) {
        mexp @ Deref @ MatchingExp::BIND_MATCH { bindIdent: ident } => {
            (ident.clone(), mexp.clone())
        },
        mexp @ Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: ident, .. } => {
            (ident.clone(), mexp.clone())
        },
        Deref @ MatchingExp::REST_MATCH { .. } => {
            (inMatchArgName.clone(), metamodelica::Ref::new(MatchingExp::BIND_MATCH { bindIdent: inMatchArgName }))
        },
        _ => {
            (inMatchArgName.clone(), metamodelica::Ref::new(MatchingExp::BIND_AS_MATCH { bindIdent: inMatchArgName, matchingExp: inMExp }))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outIdent, outMExp)
}

pub(crate) fn resolvePathInScopeEnv(
    mut inIdent: Ident,
    mut inPath: metamodelica::Ref<PathIdent>,
    mut canDoImplicitLookup: bool,
    mut inScopeEnv: ScopeEnv,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<(Ident, metamodelica::Ref<TypeSignature>, ScopeEnv)> {
    let mut outLocalIdent: Ident;
    let mut outType: metamodelica::Ref<TypeSignature>;
    let mut outScopeEnv: ScopeEnv;
    (outLocalIdent, outType, outScopeEnv) = 'mc: {
        let __mc_input = (inIdent, inPath, canDoImplicitLookup, inScopeEnv, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, _, _, Deref @ metamodelica::ListNode::Cons { head: Scope::RECURSIVE_SCOPE { recIdent: letIdent, .. }, tail: _ }, _) => {
                    let true = (stringEq(&ident, &letIdent)) else { return Err("pattern mismatch") };
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - trying to use '")); __mm_s.push_str(&*ident); __mm_s.push_str(&*literal!("' recursively inside a let scope or text addition. Use an additional Text variable if a self addition/duplication is needed, like  let b = a  let &a += b ... \n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, path, _, Deref @ metamodelica::ListNode::Cons { head: scope @ Scope::RECURSIVE_SCOPE { recIdent: letIdent, .. }, tail: restEnv }, astdefs) => {
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut ident = (*ident).clone();
                    let mut restEnv = (*restEnv).clone();
                    let false = (stringEq(&ident, &letIdent)) else { return Err("pattern mismatch") };
                    (ident, idtype, restEnv) = resolvePathInScopeEnv(ident.clone(), path.clone(), canDoImplicitLookup, restEnv.clone(), astdefs.clone())?;
                    Ok((ident.clone(), idtype.clone(), metamodelica::cons(scope.clone(), restEnv.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, _, _, Deref @ metamodelica::ListNode::Cons { head: Scope::LET_SCOPE { ident: letIdent, idType: idtype, freshIdent, .. }, tail: restEnv }, _) => {
                    let true = (stringEq(&ident, &letIdent)) else { return Err("pattern mismatch") };
                    Ok((freshIdent.clone(), idtype.clone(), metamodelica::cons(Scope::LET_SCOPE { ident: letIdent.clone(), idType: idtype.clone(), freshIdent: freshIdent.clone(), isUsed: true }, restEnv.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, path, _, Deref @ metamodelica::ListNode::Cons { head: scope @ Scope::LET_SCOPE { .. }, tail: restEnv }, astdefs) => {
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut ident = (*ident).clone();
                    let mut restEnv = (*restEnv).clone();
                    (ident, idtype, restEnv) = resolvePathInScopeEnv(ident.clone(), path.clone(), canDoImplicitLookup, restEnv.clone(), astdefs.clone())?;
                    Ok((ident.clone(), idtype.clone(), metamodelica::cons(scope.clone(), restEnv.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, path, _, scEnv @ Deref @ metamodelica::ListNode::Cons { head: Scope::FUN_SCOPE { args: fargs, .. }, tail: _ }, _) => {
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut ident = (*ident).clone();
                    idtype = lookupTupleList(fargs.clone(), ident.clone())?;
                    ident = encodePathIdent(metamodelica::AsArg::as_arg(&path), &(arcstr::literal!(funArgNamePrefix)))?;
                    Ok((ident.clone(), idtype.clone(), scEnv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, path, _, Deref @ metamodelica::ListNode::Cons { head: Scope::FUN_SCOPE { args: fargs, localArgs }, tail: restEnv }, astdefs) => {
                    let mut localIdent: Ident;
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut ident = (*ident).clone();
                    let mut fargs = (*fargs).clone();
                    let mut localArgs = (*localArgs).clone();
                    let mut restEnv = (*restEnv).clone();
                    (localIdent, idtype, restEnv) = resolvePathInScopeEnv(ident.clone(), path.clone(), canDoImplicitLookup, restEnv.clone(), astdefs.clone())?;
                    fargs = metamodelica::cons((ident.clone(), idtype.clone()), fargs.clone());
                    localArgs = metamodelica::cons((localIdent.clone(), idtype.clone()), localArgs.clone());
                    ident = encodeIdent(ident.clone(), &(arcstr::literal!(funArgNamePrefix)))?;
                    Ok((ident.clone(), idtype.clone(), metamodelica::cons(Scope::FUN_SCOPE { args: fargs.clone(), localArgs: localArgs.clone() }, restEnv.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, path, _, Deref @ metamodelica::ListNode::Cons { head: Scope::CASE_SCOPE { mExp: mexp, mType: mtype, localNames, accLocals, extArgs: extargs, matchArgName, hasImplicitScope }, tail: restEnv }, astdefs) => {
                    let mut encident: Ident;
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut mexp = (*mexp).clone();
                    let mut localNames = (*localNames).clone();
                    let mut accLocals = (*accLocals).clone();
                    (idtype, mexp) = lookupUpdateMatchingExp(ident.clone(), path.clone(), mexp.clone(), mtype.clone(), astdefs.clone())?;
                    encident = encodeIdent(ident.clone(), &(arcstr::literal!(caseBindingNamePrefix)))?;
                    (encident, localNames, accLocals) = updateLocalsForMatchingExp(ident.clone(), &encident, 0, &idtype, localNames.clone(), accLocals.clone())?;
                    Ok((encident.clone(), idtype.clone(), metamodelica::cons(Scope::CASE_SCOPE { mExp: mexp.clone(), mType: mtype.clone(), localNames: localNames.clone(), accLocals: accLocals.clone(), extArgs: extargs.clone(), matchArgName: matchArgName.clone(), hasImplicitScope: hasImplicitScope.clone() }, restEnv.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, path, true, Deref @ metamodelica::ListNode::Cons { head: Scope::CASE_SCOPE { mExp: mexp, mType: mtype, localNames, accLocals, extArgs: extargs, matchArgName, hasImplicitScope: true }, tail: restEnv }, astdefs) => {
                    let mut encident: Ident;
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut mexp = (*mexp).clone();
                    let mut localNames = (*localNames).clone();
                    let mut accLocals = (*accLocals).clone();
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n trying [it.]path for '")); __mm_s.push_str(&*ident); __mm_s.push_str(&*literal!(" / ")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&path))?); __mm_s.push_str(&*literal!("' : ")); __mm_s.push_str(&*typeSignatureString(metamodelica::AsArg::as_arg(&mtype))?); ArcStr::from(__mm_s) })?;
                    }
                    (idtype, mexp) = lookupUpdateMExpDotPath(ident.clone(), path.clone(), mexp.clone(), mtype.clone(), astdefs.clone())?;
                    if '__try0: {
                        ::match_deref::match_deref! { match &(idtype.clone()) {
                            Deref @ TypeSignature::UNRESOLVED_TYPE { .. } => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n [it.]path for '")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&path))?); __mm_s.push_str(&*literal!("' : ")); __mm_s.push_str(&*typeSignatureString(&idtype)?); ArcStr::from(__mm_s) })?;
                    }
                    encident = encodePathIdent(metamodelica::AsArg::as_arg(&path), &(arcstr::literal!(caseBindingNamePrefix)))?;
                    (encident, localNames, accLocals) = updateLocalsForMatchingExp(ident.clone(), &encident, 0, &idtype, localNames.clone(), accLocals.clone())?;
                    Ok((encident.clone(), idtype.clone(), metamodelica::cons(Scope::CASE_SCOPE { mExp: mexp.clone(), mType: mtype.clone(), localNames: localNames.clone(), accLocals: accLocals.clone(), extArgs: extargs.clone(), matchArgName: matchArgName.clone(), hasImplicitScope: true }, restEnv.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, _, _, Deref @ metamodelica::ListNode::Cons { head: Scope::CASE_SCOPE { mExp: mexp, mType: mtype, localNames, accLocals, extArgs: extargs, matchArgName, hasImplicitScope }, tail: restEnv }, _) => {
                    let mut encident: Ident;
                    let mut ident = (*ident).clone();
                    let mut mexp = (*mexp).clone();
                    let mut localNames = (*localNames).clone();
                    let mut accLocals = (*accLocals).clone();
                    let true = (stringEq(&ident, &matchArgName)) else { return Err("pattern mismatch") };
                    (ident, mexp) = prepareMatchArgument(mexp.clone(), matchArgName.clone());
                    encident = encodeIdent(ident.clone(), &(arcstr::literal!(caseBindingNamePrefix)))?;
                    (encident, localNames, accLocals) = updateLocalsForMatchingExp(ident.clone(), &encident, 0, metamodelica::AsArg::as_arg(&mtype), localNames.clone(), accLocals.clone())?;
                    Ok((encident.clone(), mtype.clone(), metamodelica::cons(Scope::CASE_SCOPE { mExp: mexp.clone(), mType: mtype.clone(), localNames: localNames.clone(), accLocals: accLocals.clone(), extArgs: extargs.clone(), matchArgName: matchArgName.clone(), hasImplicitScope: hasImplicitScope.clone() }, restEnv.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, path, _, Deref @ metamodelica::ListNode::Cons { head: Scope::CASE_SCOPE { mExp: mexp, mType: mtype, localNames, accLocals, extArgs: extargs, matchArgName, hasImplicitScope }, tail: restEnv }, astdefs) => {
                    let mut encident: Ident;
                    let mut idtype: metamodelica::Ref<TypeSignature>;
                    let mut extargs = (*extargs).clone();
                    let mut restEnv = (*restEnv).clone();
                    (encident, idtype, restEnv) = resolvePathInScopeEnv(ident.clone(), path.clone(), canDoImplicitLookup && !(hasImplicitScope.clone()), restEnv.clone(), astdefs.clone())?;
                    extargs = updateTupleList(extargs.clone(), (encident.clone(), idtype.clone()));
                    Ok((encident.clone(), idtype.clone(), metamodelica::cons(Scope::CASE_SCOPE { mExp: mexp.clone(), mType: mtype.clone(), localNames: localNames.clone(), accLocals: accLocals.clone(), extArgs: extargs.clone(), matchArgName: matchArgName.clone(), hasImplicitScope: hasImplicitScope.clone() }, restEnv.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outLocalIdent, outType, outScopeEnv))
}

pub(crate) fn addPostfixToIdent(mut inIdent: Ident, mut inPostfix: i32) -> Ident {
    let mut outPostfixedIdent: Ident;
    outPostfixedIdent = (match (inIdent.clone(), inPostfix) {
        (_, 0) => inIdent,
        (mut ident, _) => {
            ident = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ident);
                __mm_s.push_str(&*literal!("_"));
                __mm_s.push_str(&*intString(inPostfix));
                ArcStr::from(__mm_s)
            };
            ident
        }
    });
    outPostfixedIdent
}

pub(crate) fn updateLocalsForMatchingExp(
    mut inIdent: Ident,
    mut inEncIdent: &Ident,
    mut inPostfix: i32,
    mut inType: &metamodelica::Ref<TypeSignature>,
    mut inLocalNames: metamodelica::List<(ArcStr, ArcStr)>,
    mut inLocals: TypedIdents,
) -> Result<(Ident, metamodelica::List<(ArcStr, ArcStr)>, TypedIdents)> {
    let mut outLocalIdent: Ident;
    let mut outLocalNames: metamodelica::List<(ArcStr, ArcStr)>;
    let mut outLocals: TypedIdents;
    (outLocalIdent, outLocalNames, outLocals) = 'mc: {
        let __mc_input = (inIdent, inLocalNames, inLocals);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, localNames, locals) => {
                    let mut encIdent: Ident;
                    encIdent = lookupTupleList(localNames.clone(), ident.clone())?;
                    Ok((encIdent.clone(), localNames.clone(), locals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, localNames, locals) => {
                    let mut encIdent: Ident;
                    encIdent = addPostfixToIdent(inEncIdent.clone(), inPostfix);
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(locals.clone(), encIdent.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok((encIdent.clone(), metamodelica::cons((ident.clone(), encIdent.clone()), localNames.clone()), metamodelica::cons((encIdent.clone(), inType.clone()), locals.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, localNames, locals) => {
                    let mut encIdent: Ident;
                    let mut loctype: metamodelica::Ref<TypeSignature>;
                    encIdent = addPostfixToIdent(inEncIdent.clone(), inPostfix);
                    loctype = lookupTupleList(locals.clone(), encIdent.clone())?;
                    let true = (loctype.clone() == inType.clone()) else { return Err("pattern mismatch") };
                    Ok((encIdent.clone(), metamodelica::cons((ident.clone(), encIdent.clone()), localNames.clone()), locals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, localNames, locals) => {
                    let mut encIdent: Ident;
                    let mut loctype: metamodelica::Ref<TypeSignature>;
                    let mut localNames = (*localNames).clone();
                    let mut locals = (*locals).clone();
                    encIdent = addPostfixToIdent(inEncIdent.clone(), inPostfix);
                    loctype = lookupTupleList(locals.clone(), encIdent.clone())?;
                    let false = (loctype.clone() == inType.clone()) else { return Err("pattern mismatch") };
                    (encIdent, localNames, locals) = updateLocalsForMatchingExp(ident.clone(), inEncIdent, inPostfix + 1, inType, localNames.clone(), locals.clone())?;
                    Ok((encIdent.clone(), localNames.clone(), locals.clone()))
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
                    Debug::trace(literal!("-!!!updateLocalsForMatchingExp failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outLocalIdent, outLocalNames, outLocals))
}

pub(crate) fn usedInImmediateLetScope<'__b>(
    mut inIdent: &'__b Ident,
    mut inFreshIdent: &'__b Ident,
    mut inScopeEnv: &'__b ScopeEnv,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inScopeEnv {
            Deref @ metamodelica::ListNode::Cons { head: Scope::LET_SCOPE { ident: letIdent, freshIdent, .. }, tail: _ } if (stringEq(&inIdent, &letIdent) && stringEq(&inFreshIdent, &freshIdent)) => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Scope::LET_SCOPE { .. }, tail: restEnv } => {
                { (inIdent, inFreshIdent, inScopeEnv) = (inIdent, inFreshIdent, restEnv); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Scope::RECURSIVE_SCOPE { recIdent: letIdent, freshIdent }, tail: _ } if (stringEq(&inIdent, &letIdent) && stringEq(&inFreshIdent, &freshIdent)) => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Scope::RECURSIVE_SCOPE { .. }, tail: restEnv } => {
                { (inIdent, inFreshIdent, inScopeEnv) = (inIdent, inFreshIdent, restEnv); continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn updateLocalsForLetExp(
    mut inIdent: &Ident,
    mut inEncIdent: &Ident,
    mut inPostfix: i32,
    mut inType: &metamodelica::Ref<TypeSignature>,
    mut inLocals: &TypedIdents,
    mut inScopeEnv: &ScopeEnv,
) -> Result<(Ident, TypedIdents)> {
    let mut outLocalIdent: Ident;
    let mut outLocals: TypedIdents;
    (outLocalIdent, outLocals) = 'mc: {
        let __mc_input = &**inScopeEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut encIdent: Ident;
                    encIdent = addPostfixToIdent(inEncIdent.clone(), inPostfix);
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(inLocals.clone(), encIdent.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok((encIdent.clone(), metamodelica::cons((encIdent.clone(), inType.clone()), inLocals.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut encIdent: Ident;
                    let mut loctype: metamodelica::Ref<TypeSignature>;
                    let mut locals: TypedIdents;
                    encIdent = addPostfixToIdent(inEncIdent.clone(), inPostfix);
                    loctype = lookupTupleList(inLocals.clone(), encIdent.clone())?;
                    let false = (loctype.clone() == inType.clone()) else { return Err("pattern mismatch") };
                    (encIdent, locals) = updateLocalsForLetExp(inIdent, inEncIdent, inPostfix + 1, inType, inLocals, inScopeEnv)?;
                    Ok((encIdent.clone(), locals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut encIdent: Ident;
                    let mut loctype: metamodelica::Ref<TypeSignature>;
                    encIdent = addPostfixToIdent(inEncIdent.clone(), inPostfix);
                    loctype = lookupTupleList(inLocals.clone(), encIdent.clone())?;
                    let true = (loctype.clone() == inType.clone()) else { return Err("pattern mismatch") };
                    let false = (usedInImmediateLetScope(inIdent, &encIdent, inScopeEnv)) else { return Err("pattern mismatch") };
                    Ok((encIdent.clone(), inLocals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut encIdent: Ident;
                    let mut loctype: metamodelica::Ref<TypeSignature>;
                    let mut locals: TypedIdents;
                    encIdent = addPostfixToIdent(inEncIdent.clone(), inPostfix);
                    loctype = lookupTupleList(inLocals.clone(), encIdent.clone())?;
                    let true = (loctype.clone() == inType.clone()) else { return Err("pattern mismatch") };
                    let true = (usedInImmediateLetScope(inIdent, &encIdent, inScopeEnv)) else { return Err("pattern mismatch") };
                    (encIdent, locals) = updateLocalsForLetExp(inIdent, inEncIdent, inPostfix + 1, inType, inLocals, inScopeEnv)?;
                    Ok((encIdent.clone(), locals.clone()))
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
                    Debug::trace(literal!("-!!!updateLocalsForLetExp failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outLocalIdent, outLocals))
}

pub(crate) fn lookupUpdateMatchingExp(
    mut inIdent: Ident,
    mut inPathIdent: metamodelica::Ref<PathIdent>,
    mut inMatchingExp: metamodelica::Ref<MatchingExp>,
    mut inMType: metamodelica::Ref<TypeSignature>,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<(metamodelica::Ref<TypeSignature>, metamodelica::Ref<MatchingExp>)> {
    let mut outValueType: metamodelica::Ref<TypeSignature>;
    let mut outMatchingExp: metamodelica::Ref<MatchingExp>;
    (outValueType, outMatchingExp) = 'mc: {
        let __mc_input = (inIdent, inPathIdent, inMatchingExp, inMType, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ PathIdent::IDENT { ident: id }, inmexp @ Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: bid, .. }, mtype, _) => {
                    let true = (stringEq(&id, &bid)) else { return Err("pattern mismatch") };
                    Ok((mtype.clone(), inmexp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, Deref @ PathIdent::PATH_IDENT { ident: id, path }, Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: bid, matchingExp: mexp }, mtype, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut mexp = (*mexp).clone();
                    let true = (stringEq(&id, &bid)) else { return Err("pattern mismatch") };
                    (valtype, mexp) = lookupUpdateMExpDotPath(inid.clone(), path.clone(), mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), metamodelica::Ref::new(MatchingExp::BIND_AS_MATCH { bindIdent: bid.clone(), matchingExp: mexp.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: bid, matchingExp: mexp }, mtype, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut mexp = (*mexp).clone();
                    (valtype, mexp) = lookupUpdateMatchingExp(inid.clone(), path.clone(), mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), metamodelica::Ref::new(MatchingExp::BIND_AS_MATCH { bindIdent: bid.clone(), matchingExp: mexp.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ PathIdent::IDENT { ident: id }, inmexp @ Deref @ MatchingExp::BIND_MATCH { bindIdent: bid }, mtype, _) => {
                    let true = (stringEq(&id, &bid)) else { return Err("pattern mismatch") };
                    Ok((mtype.clone(), inmexp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, Deref @ PathIdent::PATH_IDENT { ident: id, .. }, inmexp @ Deref @ MatchingExp::BIND_MATCH { bindIdent: bid }, _, _) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut reason: ArcStr;
                    let true = (stringEq(&id, &bid)) else { return Err("pattern mismatch") };
                    reason = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved path '")); __mm_s.push_str(&*inid); __mm_s.push_str(&*literal!("' after first dot - only the first part '")); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!("' resolved as a bind match.")); ArcStr::from(__mm_s) };
                    valtype = metamodelica::Ref::new(TypeSignature::UNRESOLVED_TYPE { reason: reason.clone() });
                    Ok((valtype.clone(), inmexp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ MatchingExp::RECORD_MATCH { tagName: tagpath, fieldMatchings: fms }, mtype, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut fields: TypedIdents;
                    let mut fms = (*fms).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone());
                    (fields, _) = getFieldsForRecord(metamodelica::AsArg::as_arg(&mtype), tagpath.clone(), astDefs.clone())?;
                    (valtype, fms) = lookupUpdateMExpRecord(inid.clone(), path.clone(), metamodelica::AsArg::as_arg(&fms), fields.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), metamodelica::Ref::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: fms.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ MatchingExp::SOME_MATCH { value: mexp }, mtype, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut mexp = (*mexp).clone();
                    let mut mtype = (*mtype).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone())) {
                        Deref @ TypeSignature::OPTION_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    mtype = metamodelica::Own::own(__pa0);
                    (valtype, mexp) = lookupUpdateMatchingExp(inid.clone(), path.clone(), mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), metamodelica::Ref::new(MatchingExp::SOME_MATCH { value: mexp.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ MatchingExp::TUPLE_MATCH { tupleArgs: mexpLst }, mtype, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut mtypeLst: metamodelica::List<metamodelica::Ref<TypeSignature>>;
                    let mut mexpLst = (*mexpLst).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone())) {
                        Deref @ TypeSignature::TUPLE_TYPE { ofTypes: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    mtypeLst = metamodelica::Own::own(__pa0);
                    (valtype, mexpLst) = lookupUpdateMExpList(inid.clone(), path.clone(), metamodelica::AsArg::as_arg(&mexpLst), &mtypeLst, astDefs.clone())?;
                    Ok((valtype.clone(), metamodelica::Ref::new(MatchingExp::TUPLE_MATCH { tupleArgs: mexpLst.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ MatchingExp::LIST_MATCH { listElts: mexpLst }, mtype, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut mtypeLst: metamodelica::List<metamodelica::Ref<TypeSignature>>;
                    let mut mexpLst = (*mexpLst).clone();
                    let mut mtype = (*mtype).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone())) {
                        Deref @ TypeSignature::LIST_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    mtype = metamodelica::Own::own(__pa0);
                    mtypeLst = List::fill(mtype.clone(), ((mexpLst).len() as i32));
                    (valtype, mexpLst) = lookupUpdateMExpList(inid.clone(), path.clone(), metamodelica::AsArg::as_arg(&mexpLst), &mtypeLst, astDefs.clone())?;
                    Ok((valtype.clone(), metamodelica::Ref::new(MatchingExp::LIST_MATCH { listElts: mexpLst.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ MatchingExp::LIST_CONS_MATCH { head: mexp, rest: restmexp }, mtype, astDefs) => {
                    let mut otype: metamodelica::Ref<TypeSignature>;
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut mexp = (*mexp).clone();
                    let mut restmexp = (*restmexp).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone())) {
                        Deref @ TypeSignature::LIST_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    otype = metamodelica::Own::own(__pa0);
                    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(lookupUpdateMExpList(inid.clone(), path.clone(), &(list![mexp.clone(), restmexp.clone()]), &(list![otype.clone(), mtype.clone()]), astDefs.clone())?) {
                        (__pa1, Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } }) => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    valtype = metamodelica::Own::own(__pa1);
                    mexp = metamodelica::Own::own(__pa2);
                    restmexp = metamodelica::Own::own(__pa3);
                    Ok((valtype.clone(), metamodelica::Ref::new(MatchingExp::LIST_CONS_MATCH { head: mexp.clone(), rest: restmexp.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outValueType, outMatchingExp))
}

pub(crate) fn lookupUpdateMExpDotPath(
    mut inIdent: Ident,
    mut inPathIdent: metamodelica::Ref<PathIdent>,
    mut inMatchingExp: metamodelica::Ref<MatchingExp>,
    mut inMType: metamodelica::Ref<TypeSignature>,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<(metamodelica::Ref<TypeSignature>, metamodelica::Ref<MatchingExp>)> {
    let mut outValueType: metamodelica::Ref<TypeSignature>;
    let mut outMatchingExp: metamodelica::Ref<MatchingExp>;
    (outValueType, outMatchingExp) = 'mc: {
        let __mc_input = (inIdent.clone(), inPathIdent, inMatchingExp, inMType, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: bid, matchingExp: mexp }, mtype, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut mexp = (*mexp).clone();
                    (valtype, mexp) = lookupUpdateMExpDotPath(inid.clone(), path.clone(), mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), metamodelica::Ref::new(MatchingExp::BIND_AS_MATCH { bindIdent: bid.clone(), matchingExp: mexp.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, Deref @ PathIdent::IDENT { ident: id }, Deref @ MatchingExp::RECORD_MATCH { tagName: tagpath, fieldMatchings: fms }, mtype, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut fields: TypedIdents;
                    let mut fms = (*fms).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone());
                    (fields, _) = getFieldsForRecord(metamodelica::AsArg::as_arg(&mtype), tagpath.clone(), astDefs.clone())?;
                    valtype = lookupTupleList(fields.clone(), id.clone())?;
                    fms = updateFieldMatchingsForField(inid.clone(), id.clone(), metamodelica::AsArg::as_arg(&fms))?;
                    Ok((valtype.clone(), metamodelica::Ref::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: fms.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, Deref @ PathIdent::IDENT { ident: id }, Deref @ MatchingExp::RECORD_MATCH { tagName: tagpath, fieldMatchings: fms }, mtype, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut fields: TypedIdents;
                    let mut reason: ArcStr;
                    let mut tagpath = (*tagpath).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone());
                    (fields, tagpath) = getFieldsForRecord(metamodelica::AsArg::as_arg(&mtype), tagpath.clone(), astDefs.clone())?;
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(fields.clone(), id.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    reason = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved path - failed in lookup for field '")); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!("' at the end of the path '")); __mm_s.push_str(&*inid); __mm_s.push_str(&*literal!("', no such field in '")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&tagpath))?); __mm_s.push_str(&*literal!("' record fields.\n")); ArcStr::from(__mm_s) };
                    valtype = metamodelica::Ref::new(TypeSignature::UNRESOLVED_TYPE { reason: reason.clone() });
                    Ok((valtype.clone(), metamodelica::Ref::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: fms.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, Deref @ PathIdent::PATH_IDENT { ident: id, path }, Deref @ MatchingExp::RECORD_MATCH { tagName: tagpath, fieldMatchings: fms }, mtype, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut fields: TypedIdents;
                    let mut fms = (*fms).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone());
                    (fields, _) = getFieldsForRecord(metamodelica::AsArg::as_arg(&mtype), tagpath.clone(), astDefs.clone())?;
                    mtype = lookupTupleList(fields.clone(), id.clone())?;
                    (valtype, fms) = lookupUpdateMExpDotPathRecord(inid.clone(), id.clone(), path.clone(), metamodelica::AsArg::as_arg(&fms), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), metamodelica::Ref::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: fms.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, Deref @ PathIdent::PATH_IDENT { ident: id, .. }, Deref @ MatchingExp::RECORD_MATCH { tagName: tagpath, fieldMatchings: fms }, mtype, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut fields: TypedIdents;
                    let mut reason: ArcStr;
                    let mut tagpath = (*tagpath).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(metamodelica::AsArg::as_arg(&mtype), astDefs.clone());
                    (fields, tagpath) = getFieldsForRecord(metamodelica::AsArg::as_arg(&mtype), tagpath.clone(), astDefs.clone())?;
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(fields.clone(), id.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    reason = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved path - failed in lookup for field '")); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!("' inside the (encoded) path '")); __mm_s.push_str(&*inid); __mm_s.push_str(&*literal!("', no such field in '")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&tagpath))?); __mm_s.push_str(&*literal!("' record fields.\n")); ArcStr::from(__mm_s) };
                    valtype = metamodelica::Ref::new(TypeSignature::UNRESOLVED_TYPE { reason: reason.clone() });
                    Ok((valtype.clone(), metamodelica::Ref::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: fms.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, mexp, _, _) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut reason: ArcStr;
                    reason = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved path (encoded) '")); __mm_s.push_str(&*inid); __mm_s.push_str(&*literal!("', cannot follow the rest path '")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&path))?); __mm_s.push_str(&*literal!("', no record match available to look down the path.")); ArcStr::from(__mm_s) };
                    valtype = metamodelica::Ref::new(TypeSignature::UNRESOLVED_TYPE { reason: reason.clone() });
                    Ok((valtype.clone(), mexp.clone()))
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
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-!!!lookupUpdateMExpDotPath failed for ident '")); __mm_s.push_str(&*inIdent); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outValueType, outMatchingExp))
}

pub(crate) fn updateFieldMatchingsForField(
    mut inIdent: Ident,
    mut inField: Ident,
    mut inFieldMatchings: &metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>,
) -> Result<metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>> {
    let mut outFieldMatchings: metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>;
    outFieldMatchings = 'mc: {
        let __mc_input = (inIdent, inField, &**inFieldMatchings);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, fieldid, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(list![(fieldid.clone(), metamodelica::Ref::new(MatchingExp::BIND_MATCH { bindIdent: inid.clone() }))])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, fieldid, Deref @ metamodelica::ListNode::Cons { head: (ident, mexp), tail: fms }) => {
                    let mut mexp = (*mexp).clone();
                    let true = (stringEq(&fieldid, &ident)) else { return Err("pattern mismatch") };
                    mexp = makeBindAs(inid.clone(), mexp.clone())?;
                    Ok(metamodelica::cons((fieldid.clone(), mexp.clone()), fms.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, fieldid, Deref @ metamodelica::ListNode::Cons { head: fm, tail: fms }) => {
                    let mut fms = (*fms).clone();
                    fms = updateFieldMatchingsForField(inid.clone(), fieldid.clone(), metamodelica::AsArg::as_arg(&fms))?;
                    Ok(metamodelica::cons(fm.clone(), fms.clone()))
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
                    Debug::trace(literal!("-!!!updateFieldMatchingsForField failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outFieldMatchings)
}

pub(crate) fn makeBindAs(
    mut inIdent: Ident,
    mut inMExp: metamodelica::Ref<MatchingExp>,
) -> Result<metamodelica::Ref<MatchingExp>> {
    let mut outMExp: metamodelica::Ref<MatchingExp>;
    outMExp = 'mc: {
        let __mc_input = (inIdent, inMExp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, inmexp @ Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: bid, .. }) => {
                    let true = (stringEq(&inid, &bid)) else { return Err("pattern mismatch") };
                    Ok(inmexp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: bid, matchingExp: mexp }) => {
                    let mut mexp = (*mexp).clone();
                    mexp = makeBindAs(inid.clone(), mexp.clone())?;
                    Ok(metamodelica::Ref::new(MatchingExp::BIND_AS_MATCH { bindIdent: bid.clone(), matchingExp: mexp.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, inmexp @ Deref @ MatchingExp::BIND_MATCH { bindIdent: bid }) => {
                    let true = (stringEq(&inid, &bid)) else { return Err("pattern mismatch") };
                    Ok(inmexp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, mexp) => {
                    Ok(metamodelica::Ref::new(MatchingExp::BIND_AS_MATCH { bindIdent: inid.clone(), matchingExp: mexp.clone() }))
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
                    Debug::trace(literal!("-!!!makeBindAs failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMExp)
}

pub(crate) fn lookupUpdateMExpDotPathRecord(
    mut inIdent: Ident,
    mut inField: Ident,
    mut inPathIdent: metamodelica::Ref<PathIdent>,
    mut inFieldMatchings: &metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>,
    mut inMType: metamodelica::Ref<TypeSignature>,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<(
    metamodelica::Ref<TypeSignature>,
    metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>,
)> {
    let mut outValueType: metamodelica::Ref<TypeSignature>;
    let mut outFieldMatchings: metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>;
    (outValueType, outFieldMatchings) = 'mc: {
        let __mc_input = (
            inIdent.clone(),
            inField,
            inPathIdent,
            &**inFieldMatchings,
            inMType,
            inASTDefs,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, fieldid, _, Deref @ metamodelica::ListNode::Nil, _, _) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut reason: ArcStr;
                    reason = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved path '")); __mm_s.push_str(&*inid); __mm_s.push_str(&*literal!("', cannot follow the path after a dot, no record match available to look down the path after '")); __mm_s.push_str(&*fieldid); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) };
                    valtype = metamodelica::Ref::new(TypeSignature::UNRESOLVED_TYPE { reason: reason.clone() });
                    Ok((valtype.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, fieldid, path, Deref @ metamodelica::ListNode::Cons { head: (ident, mexp), tail: fms }, mtype, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut mexp = (*mexp).clone();
                    let true = (stringEq(&fieldid, &ident)) else { return Err("pattern mismatch") };
                    (valtype, mexp) = lookupUpdateMExpDotPath(inid.clone(), path.clone(), mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), metamodelica::cons((ident.clone(), mexp.clone()), fms.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, fieldid, path, Deref @ metamodelica::ListNode::Cons { head: fm, tail: fms }, mtype, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut fms = (*fms).clone();
                    (valtype, fms) = lookupUpdateMExpDotPathRecord(inid.clone(), fieldid.clone(), path.clone(), metamodelica::AsArg::as_arg(&fms), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), metamodelica::cons(fm.clone(), fms.clone())))
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
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-!!!lookupUpdateMExpDotPathRecord failed for ident '")); __mm_s.push_str(&*inIdent); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outValueType, outFieldMatchings))
}

pub(crate) fn lookupUpdateMExpRecord(
    mut inIdent: Ident,
    mut inPathIdent: metamodelica::Ref<PathIdent>,
    mut inFieldMatchings: &metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>,
    mut inFields: TypedIdents,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<(
    metamodelica::Ref<TypeSignature>,
    metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>,
)> {
    let mut outValueType: metamodelica::Ref<TypeSignature>;
    let mut outFieldMatchings: metamodelica::List<(ArcStr, metamodelica::Ref<MatchingExp>)>;
    (outValueType, outFieldMatchings) = 'mc: {
        let __mc_input = (inIdent, inPathIdent, &**inFieldMatchings, inFields, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ metamodelica::ListNode::Cons { head: (ident, mexp), tail: fms }, fields, astDefs) => {
                    let mut mtype: metamodelica::Ref<TypeSignature>;
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut mexp = (*mexp).clone();
                    mtype = lookupTupleList(fields.clone(), ident.clone())?;
                    (valtype, mexp) = lookupUpdateMatchingExp(inid.clone(), path.clone(), mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), metamodelica::cons((ident.clone(), mexp.clone()), fms.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Cons { head: (ident, _), tail: _ }, fields, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(fields.clone(), ident.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Error!!!lookupUpdateMExpRecord failed in lookup for field (type) ident '")); __mm_s.push_str(&*ident); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ metamodelica::ListNode::Cons { head: fm, tail: fms }, fields, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut fms = (*fms).clone();
                    (valtype, fms) = lookupUpdateMExpRecord(inid.clone(), path.clone(), metamodelica::AsArg::as_arg(&fms), fields.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), metamodelica::cons(fm.clone(), fms.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outValueType, outFieldMatchings))
}

pub(crate) fn lookupUpdateMExpList(
    mut inIdent: Ident,
    mut inPathIdent: metamodelica::Ref<PathIdent>,
    mut inMExpList: &metamodelica::List<metamodelica::Ref<MatchingExp>>,
    mut inMTypeList: &metamodelica::List<metamodelica::Ref<TypeSignature>>,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<(
    metamodelica::Ref<TypeSignature>,
    metamodelica::List<metamodelica::Ref<MatchingExp>>,
)> {
    let mut outValueType: metamodelica::Ref<TypeSignature>;
    let mut outMExpList: metamodelica::List<metamodelica::Ref<MatchingExp>>;
    (outValueType, outMExpList) = 'mc: {
        let __mc_input = (inIdent, inPathIdent, &**inMExpList, &**inMTypeList, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ metamodelica::ListNode::Cons { head: mexp, tail: mexpLst }, Deref @ metamodelica::ListNode::Cons { head: mtype, tail: _ }, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut mexp = (*mexp).clone();
                    (valtype, mexp) = lookupUpdateMatchingExp(inid.clone(), path.clone(), mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), metamodelica::cons(mexp.clone(), mexpLst.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ metamodelica::ListNode::Cons { head: mexp, tail: mexpLst }, Deref @ metamodelica::ListNode::Cons { head: _, tail: mtypeLst }, astDefs) => {
                    let mut valtype: metamodelica::Ref<TypeSignature>;
                    let mut mexpLst = (*mexpLst).clone();
                    (valtype, mexpLst) = lookupUpdateMExpList(inid.clone(), path.clone(), metamodelica::AsArg::as_arg(&mexpLst), metamodelica::AsArg::as_arg(&mtypeLst), astDefs.clone())?;
                    Ok((valtype.clone(), metamodelica::cons(mexp.clone(), mexpLst.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outValueType, outMExpList))
}

pub(crate) fn getFieldsForRecord(
    mut inMType: &metamodelica::Ref<TypeSignature>,
    mut inTagPath: metamodelica::Ref<PathIdent>,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<(TypedIdents, metamodelica::Ref<PathIdent>)> {
    let mut outFields: TypedIdents;
    let mut inFullyQualifiedTagPath: metamodelica::Ref<PathIdent>;
    (outFields, inFullyQualifiedTagPath) = 'mc: {
        let __mc_input = (&**inMType, inTagPath, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: typepath }, tagpath, astDefs) => {
                    let mut typeident: Ident;
                    let mut tagident: Ident;
                    let mut typepckg: metamodelica::Ref<PathIdent>;
                    let mut typepckgOpt: Option<metamodelica::Ref<PathIdent>>;
                    let mut tagpckgOpt: Option<metamodelica::Ref<PathIdent>>;
                    let mut typeinfo: TypeInfo;
                    let mut fields: TypedIdents;
                    let mut typepath = (*typepath).clone();
                    (typepckgOpt, typeident) = splitPackageAndIdent(metamodelica::AsArg::as_arg(&typepath))?;
                    (typepckg, typeinfo) = getTypeInfo(typepckgOpt.clone(), typeident.clone(), astDefs.clone())?;
                    (tagpckgOpt, tagident) = splitPackageAndIdent(metamodelica::AsArg::as_arg(&tagpath))?;
                    checkPackageOpt(typepckg.clone(), tagpckgOpt.clone())?;
                    fields = getFields(tagident.clone(), typeinfo.clone(), typeident.clone())?;
                    typepath = makePathIdent(&typepckg, tagident.clone())?;
                    Ok((fields.clone(), typepath.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { .. }, tagpath, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - (getFieldsForRecord) for case tag '")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&tagpath))?); __mm_s.push_str(&*literal!("' failed for reason above.\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, tagpath, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - for case tag '")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&tagpath))?); __mm_s.push_str(&*literal!("' the input type is not a NAME_TYPE hence not a union/record type.\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outFields, inFullyQualifiedTagPath))
}

pub(crate) fn splitPackageAndIdent(
    mut inTypePathIdent: &metamodelica::Ref<PathIdent>,
) -> Result<(Option<metamodelica::Ref<PathIdent>>, Ident)> {
    let mut outPackagePath: Option<metamodelica::Ref<PathIdent>>;
    let mut outTypeIdent: Ident;
    (outPackagePath, outTypeIdent) = 'mc: {
        let __mc_input = &**inTypePathIdent;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ PathIdent::IDENT { ident: typeident } => {
                    Ok((None, typeident.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ PathIdent::PATH_IDENT { ident: pckgident, path: Deref @ PathIdent::IDENT { ident: typeident } } => {
                    Ok((Some(metamodelica::Ref::new(PathIdent::IDENT { ident: pckgident.clone() })), typeident.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ PathIdent::PATH_IDENT { ident: pckgident, path: typepath @ Deref @ PathIdent::PATH_IDENT { .. } } => {
                    let mut typeident: Ident;
                    let mut typepckg: metamodelica::Ref<PathIdent>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(splitPackageAndIdent(metamodelica::AsArg::as_arg(&typepath))?) {
                        (Some(__pa0), __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    typepckg = metamodelica::Own::own(__pa0);
                    typeident = metamodelica::Own::own(__pa1);
                    Ok((Some(metamodelica::Ref::new(PathIdent::PATH_IDENT { ident: pckgident.clone(), path: typepckg.clone() })), typeident.clone()))
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
                    Debug::trace(literal!("-!!!splitPackageAndIdent failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outPackagePath, outTypeIdent))
}

fn getPackageIdent(mut inTypePathIdent: &metamodelica::Ref<PathIdent>) -> Result<Ident> {
    let mut outTypeIdent: Ident;
    (_, outTypeIdent) = splitPackageAndIdent(inTypePathIdent)?;
    Ok(outTypeIdent)
}

pub(crate) fn makePathIdent(
    mut inPackage: &metamodelica::Ref<PathIdent>,
    mut inIdent: Ident,
) -> Result<metamodelica::Ref<PathIdent>> {
    let mut outPathIdent: metamodelica::Ref<PathIdent>;
    outPathIdent = 'mc: {
        let __mc_input = (&**inPackage, inIdent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ PathIdent::IDENT { ident: pckgident }, ident) => {
                    Ok(metamodelica::Ref::new(PathIdent::PATH_IDENT { ident: pckgident.clone(), path: metamodelica::Ref::new(PathIdent::IDENT { ident: ident.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ PathIdent::PATH_IDENT { ident: pckgident, path: pckgpath }, ident) => {
                    let mut path: metamodelica::Ref<PathIdent>;
                    path = makePathIdent(metamodelica::AsArg::as_arg(&pckgpath), ident.clone())?;
                    Ok(metamodelica::Ref::new(PathIdent::PATH_IDENT { ident: pckgident.clone(), path: path.clone() }))
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
                    Debug::trace(literal!("-!!!makePathIdent failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outPathIdent)
}

pub(crate) fn getTypeInfo(
    mut inTypePackageOpt: Option<metamodelica::Ref<PathIdent>>,
    mut inTypeIdent: Ident,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<(metamodelica::Ref<PathIdent>, TypeInfo)> {
    let mut outTypePackage: metamodelica::Ref<PathIdent>;
    let mut outTypeInfo: TypeInfo;
    (outTypePackage, outTypeInfo) = lookupTypeInfo(inTypePackageOpt.clone(), inTypeIdent.clone(), &inASTDefs)?;
    if (inTypePackageOpt).is_none() {
        checkUnqualifiedAmbiguity(inTypeIdent, &outTypePackage, inASTDefs)?;
    }
    Ok((outTypePackage, outTypeInfo))
}

fn lookupTypeInfo(
    mut inTypePackageOpt: Option<metamodelica::Ref<PathIdent>>,
    mut inTypeIdent: Ident,
    mut inASTDefs: &metamodelica::List<ASTDef>,
) -> Result<(metamodelica::Ref<PathIdent>, TypeInfo)> {
    let mut outTypePackage: metamodelica::Ref<PathIdent>;
    let mut outTypeInfo: TypeInfo;
    (outTypePackage, outTypeInfo) = 'mc: {
        let __mc_input = (inTypePackageOpt, inTypeIdent, &**inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, typeident, Deref @ metamodelica::ListNode::Cons { head: ASTDef { importPackage: importckg, isDefault: true, types: typeLst, .. }, tail: _ }) => {
                    let mut typeinfo: TypeInfo;
                    typeinfo = lookupTupleList(typeLst.clone(), typeident.clone())?;
                    Ok((importckg.clone(), typeinfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(typepckg), typeident, Deref @ metamodelica::ListNode::Cons { head: ASTDef { importPackage: importckg, types: typeLst, .. }, tail: _ }) => {
                    let mut typeinfo: TypeInfo;
                    let true = (typepckg.clone() == importckg.clone()) else { return Err("pattern mismatch") };
                    typeinfo = lookupTupleList(typeLst.clone(), typeident.clone())?;
                    Ok((typepckg.clone(), typeinfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (typepckgOpt, typeident, Deref @ metamodelica::ListNode::Cons { head: _, tail: astDefs }) => {
                    let mut typepckg: metamodelica::Ref<PathIdent>;
                    let mut typeinfo: TypeInfo;
                    (typepckg, typeinfo) = lookupTypeInfo(typepckgOpt.clone(), typeident.clone(), metamodelica::AsArg::as_arg(&astDefs))?;
                    Ok((typepckg.clone(), typeinfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, typeident, Deref @ metamodelica::ListNode::Nil) => {
                    addSusanNotification({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - getTypeInfo failed to lookup the type '")); __mm_s.push_str(&*typeident); __mm_s.push_str(&*literal!("' after looking up all AST definitions.")); ArcStr::from(__mm_s) }, &(dummySourceInfo.clone()))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(typepckg), typeident, Deref @ metamodelica::ListNode::Nil) => {
                    addSusanNotification({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getTypeInfo failed to lookup the type '")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&typepckg))?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*typeident); __mm_s.push_str(&*literal!("' after looking up all AST definitions.")); ArcStr::from(__mm_s) }, &(dummySourceInfo.clone()))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outTypePackage, outTypeInfo))
}

fn checkUnqualifiedAmbiguity(
    mut inIdent: Ident,
    mut inPackage: &metamodelica::Ref<PathIdent>,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<()> {
    let mut canonical: ArcStr = canonicalTypeName(inPackage, inIdent.clone(), inASTDefs.clone())?;
    let mut other: ArcStr;
    let mut clashes: metamodelica::List<ArcStr> = metamodelica::nil();
    for mut astDef in &*inASTDefs {
        if astDef.isDefault.clone() {
            if '__try0: {
                unwrap_break_err!(lookupTupleList(astDef.types.clone(), inIdent.clone()), '__try0);
                other = unwrap_break_err!(canonicalTypeName(&astDef.importPackage, inIdent.clone(), inASTDefs.clone()), '__try0);
                if !metamodelica::stringEq(&other, &canonical) && !(listMember(other.clone(), clashes.clone())) {
                    clashes = metamodelica::cons(other.clone(), clashes.clone());
                }
                Ok::<(), &'static str>(())
            }.is_err() {
            }
        }
    }
    if !((clashes).is_empty()) {
        addSusanError(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Ambiguous unqualified name '"));
                __mm_s.push_str(&*inIdent);
                __mm_s.push_str(&*literal!("': it denotes "));
                __mm_s.push_str(&*canonical);
                __mm_s.push_str(&*literal!(" and "));
                __mm_s.push_str(&*stringDelimitList(clashes.reverse(), literal!(", ")));
                __mm_s.push_str(&*literal!(". Qualify it."));
                ArcStr::from(__mm_s)
            },
            &(dummySourceInfo.clone()),
        )?;
    }
    Ok(())
}

fn canonicalTypeName(
    mut inPackage: &metamodelica::Ref<PathIdent>,
    mut inIdent: Ident,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<ArcStr> {
    let mut outName: ArcStr;
    let mut ty: metamodelica::Ref<TypeSignature>;
    ty = deAliasedType(
        &(metamodelica::Ref::new(TypeSignature::NAMED_TYPE {
            name: makePathIdent(inPackage, inIdent)?,
        })),
        inASTDefs,
    );
    outName = (match &*ty {
        TypeSignature::NAMED_TYPE { name: path } => pathIdentString(metamodelica::AsArg::as_arg(&path))?,
        _ => typeSignatureString(&ty)?,
    });
    Ok(outName)
}

fn deAliasedType(
    mut inType: &metamodelica::Ref<TypeSignature>,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> metamodelica::Ref<TypeSignature> {
    let mut outType: metamodelica::Ref<TypeSignature>;
    outType = 'mc: {
        let __mc_input = (&**inType, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: typepath }, astDefs) => {
                    let mut dt: metamodelica::Ref<TypeSignature>;
                    let mut typeident: Ident;
                    let mut typepckgOpt: Option<metamodelica::Ref<PathIdent>>;
                    (typepckgOpt, typeident) = splitPackageAndIdent(metamodelica::AsArg::as_arg(&typepath))?;
                    let __pa0 = ::match_deref::match_deref! { match &(getTypeInfo(typepckgOpt.clone(), typeident.clone(), astDefs.clone())?) {
                        (_, TypeInfo::TI_ALIAS_TYPE { aliasType: __pa0 }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    dt = metamodelica::Own::own(__pa0);
                    Ok(deAliasedType(&dt, astDefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inType.clone())
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

fn typesEqual(
    mut inType: metamodelica::Ref<TypeSignature>,
    mut inTypeConcrete: metamodelica::Ref<TypeSignature>,
    mut inTypeVars: metamodelica::List<ArcStr>,
    mut inSetTypeVars: TypedIdents,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<TypedIdents> {
    let mut outSetTypeVars: TypedIdents;
    outSetTypeVars = 'mc: {
        let __mc_input = (inType.clone(), inTypeConcrete, inTypeVars, inSetTypeVars, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::LIST_TYPE { ofType: ota }, Deref @ TypeSignature::LIST_TYPE { ofType: otb }, tyVars, setTyVars, astDefs) => {
                    Ok(typesEqual(ota.clone(), otb.clone(), tyVars.clone(), setTyVars.clone(), astDefs.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::ARRAY_TYPE { ofType: ota }, Deref @ TypeSignature::ARRAY_TYPE { ofType: otb }, tyVars, setTyVars, astDefs) => {
                    Ok(typesEqual(ota.clone(), otb.clone(), tyVars.clone(), setTyVars.clone(), astDefs.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::OPTION_TYPE { ofType: ota }, Deref @ TypeSignature::OPTION_TYPE { ofType: otb }, tyVars, setTyVars, astDefs) => {
                    Ok(typesEqual(ota.clone(), otb.clone(), tyVars.clone(), setTyVars.clone(), astDefs.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::TUPLE_TYPE { ofTypes: otaLst }, Deref @ TypeSignature::TUPLE_TYPE { ofTypes: otbLst }, tyVars, setTyVars, astDefs) => {
                    Ok(typesEqualList(otaLst.clone(), otbLst.clone(), tyVars.clone(), setTyVars.clone(), astDefs.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, tyConcrete @ Deref @ TypeSignature::NAMED_TYPE { .. }, tyVars, setTyVars, astDefs) => {
                    let mut tyConcreteDA: metamodelica::Ref<TypeSignature>;
                    if '__try0: {
                        ::match_deref::match_deref! { match &(ty.clone()) {
                            Deref @ TypeSignature::NAMED_TYPE { .. } => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    tyConcreteDA = deAliasedType(metamodelica::AsArg::as_arg(&tyConcrete), astDefs.clone());
                    let false = (tyConcreteDA.clone() == tyConcrete.clone()) else { return Err("pattern mismatch") };
                    Ok(typesEqual(ty.clone(), tyConcreteDA.clone(), tyVars.clone(), setTyVars.clone(), astDefs.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::PATH_IDENT { .. } }, tyConcrete, _, setTyVars, astDefs) => {
                    let mut ty: metamodelica::Ref<TypeSignature>;
                    let mut tyConcrete = (*tyConcrete).clone();
                    ty = deAliasedType(&inType, astDefs.clone());
                    tyConcrete = deAliasedType(metamodelica::AsArg::as_arg(&tyConcrete), astDefs.clone());
                    typesEqualConcrete(ty.clone(), tyConcrete.clone(), astDefs.clone())?;
                    Ok(setTyVars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: tid } }, tyConcrete, tyVars, setTyVars, astDefs) => {
                    let mut ty: metamodelica::Ref<TypeSignature>;
                    let mut tyConcrete = (*tyConcrete).clone();
                    let false = (listMember(tid.clone(), tyVars.clone())) else { return Err("pattern mismatch") };
                    ty = deAliasedType(&inType, astDefs.clone());
                    tyConcrete = deAliasedType(metamodelica::AsArg::as_arg(&tyConcrete), astDefs.clone());
                    typesEqualConcrete(ty.clone(), tyConcrete.clone(), astDefs.clone())?;
                    Ok(setTyVars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: tid } }, tyConcrete, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, setTyVars, astDefs) => {
                    let mut ty: metamodelica::Ref<TypeSignature>;
                    let mut tyConcreteDA: metamodelica::Ref<TypeSignature>;
                    ty = lookupTupleList(setTyVars.clone(), tid.clone())?;
                    tyConcreteDA = deAliasedType(metamodelica::AsArg::as_arg(&tyConcrete), astDefs.clone());
                    typesEqualConcrete(ty.clone(), tyConcreteDA.clone(), astDefs.clone())?;
                    Ok(setTyVars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: tid } }, tyConcrete, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, setTyVars, astDefs) => {
                    let mut ty: metamodelica::Ref<TypeSignature>;
                    let mut tyConcreteDA: metamodelica::Ref<TypeSignature>;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    ty = lookupTupleList(setTyVars.clone(), tid.clone())?;
                    tyConcreteDA = deAliasedType(metamodelica::AsArg::as_arg(&tyConcrete), astDefs.clone());
                    if '__try0: {
                        unwrap_break_err!(typesEqualConcrete(ty.clone(), tyConcreteDA.clone(), astDefs.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - unmatched type for type variable '")); __mm_s.push_str(&*tid); __mm_s.push_str(&*literal!("'. Firstly inferred '")); __mm_s.push_str(&*typeSignatureString(&ty)?); __mm_s.push_str(&*literal!("', next inferred '")); __mm_s.push_str(&*typeSignatureString(metamodelica::AsArg::as_arg(&tyConcrete))?); __mm_s.push_str(&*literal!("'(dealiased '")); __mm_s.push_str(&*typeSignatureString(&tyConcreteDA)?); __mm_s.push_str(&*literal!("').\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: tid } }, tyConcrete, tyVars @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, setTyVars, astDefs) => {
                    let mut tyConcreteDA: metamodelica::Ref<TypeSignature>;
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(setTyVars.clone(), tid.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    let true = (listMember(tid.clone(), tyVars.clone())) else { return Err("pattern mismatch") };
                    tyConcreteDA = deAliasedType(metamodelica::AsArg::as_arg(&tyConcrete), astDefs.clone());
                    Ok(metamodelica::cons((tid.clone(), tyConcreteDA.clone()), setTyVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::UNRESOLVED_TYPE { .. }, Deref @ TypeSignature::UNRESOLVED_TYPE { reason: _ }, _, setTyVars, _) => {
                    Ok(setTyVars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, tyConcrete, _, setTyVars, _) => {
                    if '__try0: {
                        ::match_deref::match_deref! { match &(ty.clone()) {
                            Deref @ TypeSignature::NAMED_TYPE { .. } => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    let true = (ty.clone() == tyConcrete.clone()) else { return Err("pattern mismatch") };
                    Ok(setTyVars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outSetTypeVars)
}

fn typesEqualConcrete(
    mut inTypeA: metamodelica::Ref<TypeSignature>,
    mut inTypeB: metamodelica::Ref<TypeSignature>,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inTypeA, inTypeB)) {
        (Deref @ TypeSignature::NAMED_TYPE { name: na }, Deref @ TypeSignature::NAMED_TYPE { name: nb }) if (na.clone() == nb.clone()) => {
            ()
        },
        (tyA, tyB) => {
            let mut astDefs = inASTDefs;
            if '__try0: {
                ::match_deref::match_deref! { match &(tyA.clone()) {
                    Deref @ TypeSignature::NAMED_TYPE { .. } => (),
                    _ => break '__try0 Err::<_, _>("pattern mismatch"),
                } };
                Ok::<(), &'static str>(())
            }.is_ok() { return Err("failure(): body succeeded") }
            typesEqual(tyA.clone(), tyB.clone(), metamodelica::nil(), metamodelica::nil(), astDefs)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn typesEqualList(
    mut inTypeAList: metamodelica::List<metamodelica::Ref<TypeSignature>>,
    mut inTypeBList: metamodelica::List<metamodelica::Ref<TypeSignature>>,
    mut inTypeVars: metamodelica::List<ArcStr>,
    mut inSetTypeVars: TypedIdents,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<TypedIdents> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inTypeAList, inTypeBList, inTypeVars, inSetTypeVars, inASTDefs)) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _, setTyVars, _) => {
                return Ok(setTyVars.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: ota, tail: otaLst }, Deref @ metamodelica::ListNode::Cons { head: otb, tail: otbLst }, tyVars, setTyVars, astDefs) => {
                let mut setTyVars = (*setTyVars).clone();
                setTyVars = typesEqual(ota.clone(), otb.clone(), tyVars.clone(), setTyVars.clone(), astDefs.clone())?;
                { (inTypeAList, inTypeBList, inTypeVars, inSetTypeVars, inASTDefs) = (otaLst.clone(), otbLst.clone(), tyVars.clone(), setTyVars.clone(), astDefs.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn specializeType(
    mut inType: metamodelica::Ref<TypeSignature>,
    mut inTypeVars: metamodelica::List<ArcStr>,
    mut inSetTypeVars: TypedIdents,
) -> Result<metamodelica::Ref<TypeSignature>> {
    let mut outType: metamodelica::Ref<TypeSignature>;
    outType = 'mc: {
        let __mc_input = (inType, inTypeVars, inSetTypeVars);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::LIST_TYPE { ofType: ota }, tyVars, setTyVars) => {
                    let mut ota = (*ota).clone();
                    ota = specializeType(ota.clone(), tyVars.clone(), setTyVars.clone())?;
                    Ok(metamodelica::Ref::new(TypeSignature::LIST_TYPE { ofType: ota.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::ARRAY_TYPE { ofType: ota }, tyVars, setTyVars) => {
                    let mut ota = (*ota).clone();
                    ota = specializeType(ota.clone(), tyVars.clone(), setTyVars.clone())?;
                    Ok(metamodelica::Ref::new(TypeSignature::ARRAY_TYPE { ofType: ota.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::OPTION_TYPE { ofType: ota }, tyVars, setTyVars) => {
                    let mut ota = (*ota).clone();
                    ota = specializeType(ota.clone(), tyVars.clone(), setTyVars.clone())?;
                    Ok(metamodelica::Ref::new(TypeSignature::OPTION_TYPE { ofType: ota.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::TUPLE_TYPE { ofTypes: otaLst }, tyVars, setTyVars) => {
                    let mut otaLst = (*otaLst).clone();
                    otaLst = List::map2(otaLst.clone(), &specializeType, tyVars.clone(), setTyVars.clone())?;
                    Ok(metamodelica::Ref::new(TypeSignature::TUPLE_TYPE { ofTypes: otaLst.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (tyConcrete @ Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: tid } }, tyVars, _) => {
                    let false = (listMember(tid.clone(), tyVars.clone())) else { return Err("pattern mismatch") };
                    Ok(tyConcrete.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: tid } }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, setTyVars) => {
                    let mut tyConcrete: metamodelica::Ref<TypeSignature>;
                    tyConcrete = lookupTupleList(setTyVars.clone(), tid.clone())?;
                    Ok(tyConcrete.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: tid } }, tyVars @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, setTyVars) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    let true = (listMember(tid.clone(), tyVars.clone())) else { return Err("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(lookupTupleList(setTyVars.clone(), tid.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - cannot infer type variable '")); __mm_s.push_str(&*tid); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (tyConcrete, _, _) => {
                    if '__try0: {
                        ::match_deref::match_deref! { match &(tyConcrete.clone()) {
                            Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { .. } } => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok(tyConcrete.clone())
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

//for now, succeed or  error + fail
pub(crate) fn getFunSignature(
    mut inFunName: metamodelica::Ref<PathIdent>,
    mut inSourceInfo: &SourceInfo,
    mut inTplPackage: &TemplPackage,
) -> Result<(
    metamodelica::Ref<PathIdent>,
    TypedIdents,
    TypedIdents,
    metamodelica::List<ArcStr>,
)> {
    let mut outPath: metamodelica::Ref<PathIdent>;
    let mut outInArgs: TypedIdents;
    let mut outOutArgs: TypedIdents;
    let mut outTypeVars: metamodelica::List<ArcStr>;
    (outPath, outInArgs, outOutArgs, outTypeVars) = 'mc: {
        let __mc_input = (inFunName.clone(), inTplPackage);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fname @ Deref @ PathIdent::IDENT { ident: templname }, TemplPackage { templateDefs, .. }) => {
                    let mut iargs: TypedIdents;
                    let mut oargs: TypedIdents;
                    let TemplateDef::TEMPLATE_DEF { args: __pa0, .. } = (lookupTupleList(templateDefs.clone(), templname.clone())?) else { return Err("pattern mismatch") };
                    iargs = metamodelica::Own::own(__pa0);
                    iargs = metamodelica::cons(imlicitTxtArg.clone(), iargs.clone());
                    oargs = List::filterOnTrue(iargs.clone(), (std::sync::Arc::new(move |__a0: (ArcStr, metamodelica::Ref<TypeSignature>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(isText(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, metamodelica::Ref<TypeSignature>)) -> Result<bool> + 'static>))?;
                    Ok((fname.clone(), iargs.clone(), oargs.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ PathIdent::IDENT { ident: templname }, TemplPackage { templateDefs, .. }) => {
                    let mut msg: ArcStr;
                    lookupTupleList(templateDefs.clone(), templname.clone())?;
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Constant template '")); __mm_s.push_str(&*templname); __mm_s.push_str(&*literal!("' is used in a function/template context (while it is defined as a constant).")); ArcStr::from(__mm_s) };
                    addSusanError(msg.clone(), inSourceInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fname, TemplPackage { astDefs, .. }) => {
                    let mut funpckg: metamodelica::Ref<PathIdent>;
                    let mut funpckgOpt: Option<metamodelica::Ref<PathIdent>>;
                    let mut fident: Ident;
                    let mut tyVars: metamodelica::List<ArcStr>;
                    let mut iargs: TypedIdents;
                    let mut oargs: TypedIdents;
                    let mut fname = (*fname).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(&(metamodelica::Ref::new(TypeSignature::NAMED_TYPE { name: fname.clone() })), astDefs.clone())) {
                        Deref @ TypeSignature::NAMED_TYPE { name: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    fname = metamodelica::Own::own(__pa0);
                    (funpckgOpt, fident) = splitPackageAndIdent(metamodelica::AsArg::as_arg(&fname))?;
                    let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(getTypeInfo(funpckgOpt.clone(), fident.clone(), astDefs.clone())?) {
                        (__pa1, TypeInfo::TI_FUN_TYPE { inArgs: __pa2, outArgs: __pa3, tyVars: __pa4 }) => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    funpckg = metamodelica::Own::own(__pa1);
                    iargs = metamodelica::Own::own(__pa2);
                    oargs = metamodelica::Own::own(__pa3);
                    tyVars = metamodelica::Own::own(__pa4);
                    fname = if (metamodelica::Ref::new(PathIdent::IDENT { ident: literal!("builtin") }) == funpckg.clone()) {metamodelica::Ref::new(PathIdent::IDENT { ident: fident.clone() })} else {makePathIdent(&funpckg, fident.clone())?};
                    Ok((fname.clone(), iargs.clone(), oargs.clone(), tyVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut msg: ArcStr;
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved template/function name '")); __mm_s.push_str(&*pathIdentString(&inFunName)?); __mm_s.push_str(&*literal!("'.")); ArcStr::from(__mm_s) };
                    addSusanError(msg.clone(), inSourceInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outPath, outInArgs, outOutArgs, outTypeVars))
}

pub(crate) fn checkPackageOpt(
    mut inPackage: metamodelica::Ref<PathIdent>,
    mut inPackageOpt: Option<metamodelica::Ref<PathIdent>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inPackage, inPackageOpt)) {
        (_, None) => {
            ()
        },
        (path, Some(pckgpath)) if (path.clone() == pckgpath.clone()) => {
            ()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::trace(literal!("-!!!checkPackageOpt failed - package paths are not the same.\n"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn getFields(
    mut inTagIdent: Ident,
    mut inTypeInfo: TypeInfo,
    mut inTypeIdent: Ident,
) -> Result<TypedIdents> {
    let mut outFields: TypedIdents;
    outFields = 'mc: {
        let __mc_input = (inTagIdent, inTypeInfo, inTypeIdent);
        if let Ok(__v) = (|| -> Result<_> {
            let (mut tagident, TypeInfo::TI_UNION_TYPE { recTags: ref rectags }, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut fields: TypedIdents;
            fields = lookupTupleList(rectags.clone(), tagident.clone())?;
            Ok(fields.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut tagident, TypeInfo::TI_UNION_TYPE { recTags: ref rectags }, mut typeident) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            if '__try0: {
                unwrap_break_err!(lookupTupleList(rectags.clone(), tagident.clone()), '__try0);
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Error - getFields failed to lookup the union tag '"));
                __mm_s.push_str(&*tagident);
                __mm_s.push_str(&*literal!("', that is not found in type '"));
                __mm_s.push_str(&*typeident);
                __mm_s.push_str(&*literal!("'.\n"));
                ArcStr::from(__mm_s)
            })?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut tagident, TypeInfo::TI_RECORD_TYPE { fields: mut fields }, mut typeident) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let true = (stringEq(&tagident, &typeident)) else {
                return Err("pattern mismatch");
            };
            Ok(fields.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut tagident, TypeInfo::TI_RECORD_TYPE { .. }, mut typeident) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let false = (stringEq(&tagident, &typeident)) else {
                return Err("pattern mismatch");
            };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Error - getFields failed to match the tag '"));
                __mm_s.push_str(&*tagident);
                __mm_s.push_str(&*literal!("', the type '"));
                __mm_s.push_str(&*typeident);
                __mm_s.push_str(&*literal!("' expected.\n"));
                ArcStr::from(__mm_s)
            })?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, mut typeinfo, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            if '__try0: {
                let TypeInfo::TI_UNION_TYPE { .. } = (typeinfo.clone()) else {
                    break '__try0 Err::<_, _>("pattern mismatch");
                };
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            if '__try1: {
                let TypeInfo::TI_RECORD_TYPE { .. } = (typeinfo.clone()) else {
                    break '__try1 Err::<_, _>("pattern mismatch");
                };
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!(
                "- getFields failed - the typeinfo is neither union nor record type.\n"
            ))?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outFields)
}

pub(crate) fn isRecordTag(mut inTagIdent: Ident, mut inTypeInfo: &TypeInfo, mut inTypeIdent: Ident) -> Result<()> {
    let () = (match (inTagIdent, inTypeInfo.clone(), inTypeIdent) {
        (mut tagident, TypeInfo::TI_UNION_TYPE { recTags: ref rectags }, _) => {
            lookupTupleList(rectags.clone(), tagident)?;
            ()
        }
        (mut tagident, TypeInfo::TI_RECORD_TYPE { .. }, mut typeident) => {
            let true = (stringEq(&tagident, &typeident)) else {
                return Err("pattern mismatch");
            };
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub(crate) fn fullyQualifyASTDefs(mut inASTDefs: &metamodelica::List<ASTDef>) -> Result<metamodelica::List<ASTDef>> {
    let mut outFullyQualifiedASTDefs: metamodelica::List<ASTDef>;
    outFullyQualifiedASTDefs = 'mc: {
        let __mc_input = &**inASTDefs;
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
                Deref @ metamodelica::ListNode::Cons { head: ASTDef { importPackage: importckg, isDefault: isdefault, isInterface: isinterface, types: typeLst }, tail: restAstDefs } => {
                    let mut typeLst = (*typeLst).clone();
                    let mut restAstDefs = (*restAstDefs).clone();
                    typeLst = listMap1Tuple22(metamodelica::AsArg::as_arg(&typeLst), (std::sync::Arc::new(move |__a0: TypeInfo, __a1: metamodelica::Ref<PathIdent>| fullyQualifyAstTypeInfo(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(TypeInfo, metamodelica::Ref<PathIdent>) -> Result<TypeInfo> + 'static>), importckg.clone())?;
                    restAstDefs = fullyQualifyASTDefs(metamodelica::AsArg::as_arg(&restAstDefs))?;
                    Ok(metamodelica::cons(ASTDef { importPackage: importckg.clone(), isDefault: isdefault.clone(), isInterface: isinterface.clone(), types: typeLst.clone() }, restAstDefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: ASTDef { importPackage: importckg, types: typeLst, .. }, tail: _ } => {
                    let mut typeLst = (*typeLst).clone();
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    if '__try0: {
                        typeLst = unwrap_break_err!(listMap1Tuple22(metamodelica::AsArg::as_arg(&typeLst), (std::sync::Arc::new(move |__a0: TypeInfo, __a1: metamodelica::Ref<PathIdent>| fullyQualifyAstTypeInfo(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(TypeInfo, metamodelica::Ref<PathIdent>) -> Result<TypeInfo> + 'static>), importckg.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-fullyQualifyASTDefs failed for importckg = ")); __mm_s.push_str(&*pathIdentString(metamodelica::AsArg::as_arg(&importckg))?); __mm_s.push_str(&*literal!(" .\n")); ArcStr::from(__mm_s) })?;
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
                    Debug::trace(literal!("-!!! fullyQualifyASTDefs failed .\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outFullyQualifiedASTDefs)
}

pub(crate) fn fullyQualifyAstTypeInfo(
    mut inASTTypeInfo: &TypeInfo,
    mut inImportPackage: metamodelica::Ref<PathIdent>,
) -> Result<TypeInfo> {
    let mut outFullyQualifiedASTTypeInfo: TypeInfo;
    outFullyQualifiedASTTypeInfo = 'mc: {
        let __mc_input = (inASTTypeInfo, inImportPackage);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TypeInfo::TI_UNION_TYPE { recTags }, importpckg) => {
                    let mut recTags = (*recTags).clone();
                    recTags = listMap2Tuple22(metamodelica::AsArg::as_arg(&recTags), (std::sync::Arc::new(move |__a0: metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>, __a1: metamodelica::Ref<PathIdent>, __a2: metamodelica::List<ArcStr>| fullyQualifyAstTypedIdents(&__a0, __a1, __a2)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>, metamodelica::Ref<PathIdent>, metamodelica::List<ArcStr>) -> Result<metamodelica::List<(ArcStr, metamodelica::Ref<TypeSignature>)>> + 'static>), importpckg.clone(), metamodelica::nil())?;
                    Ok(TypeInfo::TI_UNION_TYPE { recTags: recTags.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TypeInfo::TI_RECORD_TYPE { fields }, importpckg) => {
                    let mut fields = (*fields).clone();
                    fields = fullyQualifyAstTypedIdents(metamodelica::AsArg::as_arg(&fields), importpckg.clone(), metamodelica::nil())?;
                    Ok(TypeInfo::TI_RECORD_TYPE { fields: fields.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TypeInfo::TI_ALIAS_TYPE { aliasType }, importpckg) => {
                    let mut aliasType = (*aliasType).clone();
                    aliasType = fullyQualifyAstTypeSignature(aliasType.clone(), importpckg.clone(), metamodelica::nil());
                    Ok(TypeInfo::TI_ALIAS_TYPE { aliasType: aliasType.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TypeInfo::TI_FUN_TYPE { inArgs, outArgs, tyVars: tyvars }, importpckg) => {
                    let mut inArgs = (*inArgs).clone();
                    let mut outArgs = (*outArgs).clone();
                    inArgs = fullyQualifyAstTypedIdents(metamodelica::AsArg::as_arg(&inArgs), importpckg.clone(), tyvars.clone())?;
                    outArgs = fullyQualifyAstTypedIdents(metamodelica::AsArg::as_arg(&outArgs), importpckg.clone(), tyvars.clone())?;
                    Ok(TypeInfo::TI_FUN_TYPE { inArgs: inArgs.clone(), outArgs: outArgs.clone(), tyVars: tyvars.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TypeInfo::TI_CONST_TYPE { constType }, importpckg) => {
                    let mut constType = (*constType).clone();
                    constType = fullyQualifyAstTypeSignature(constType.clone(), importpckg.clone(), metamodelica::nil());
                    Ok(TypeInfo::TI_CONST_TYPE { constType: constType.clone() })
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
                    Debug::trace(literal!("-!!! fullyQualifyAstTypeInfo failed .\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outFullyQualifiedASTTypeInfo)
}

pub(crate) fn fullyQualifyAstTypedIdents(
    mut inASTDefTypedIdents: &TypedIdents,
    mut inImportPackage: metamodelica::Ref<PathIdent>,
    mut inTypeVars: metamodelica::List<ArcStr>,
) -> Result<TypedIdents> {
    let mut outASTDefTypedIdents: TypedIdents;
    outASTDefTypedIdents = listMap2Tuple22(
        inASTDefTypedIdents,
        (std::sync::Arc::new(fnptr!(
            fullyQualifyAstTypeSignature,
            metamodelica::Ref<TypeSignature>,
            metamodelica::Ref<PathIdent>,
            metamodelica::List<ArcStr>
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<TypeSignature>,
                        metamodelica::Ref<PathIdent>,
                        metamodelica::List<ArcStr>,
                    ) -> Result<metamodelica::Ref<TypeSignature>>
                    + 'static,
            >),
        inImportPackage,
        inTypeVars,
    )?;
    Ok(outASTDefTypedIdents)
}

pub(crate) fn fullyQualifyAstTypeSignature(
    mut inASTDefTypeSignature: metamodelica::Ref<TypeSignature>,
    mut inImportPackage: metamodelica::Ref<PathIdent>,
    mut inTypeVars: metamodelica::List<ArcStr>,
) -> metamodelica::Ref<TypeSignature> {
    let mut outASTDefTypeSignature: metamodelica::Ref<TypeSignature>;
    outASTDefTypeSignature = 'mc: {
        let __mc_input = (inASTDefTypeSignature.clone(), inImportPackage, inTypeVars);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::LIST_TYPE { ofType: ota }, importpckg, tyVars) => {
                    let mut ota = (*ota).clone();
                    ota = fullyQualifyAstTypeSignature(ota.clone(), importpckg.clone(), tyVars.clone());
                    Ok(metamodelica::Ref::new(TypeSignature::LIST_TYPE { ofType: ota.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::ARRAY_TYPE { ofType: ota }, importpckg, tyVars) => {
                    let mut ota = (*ota).clone();
                    ota = fullyQualifyAstTypeSignature(ota.clone(), importpckg.clone(), tyVars.clone());
                    Ok(metamodelica::Ref::new(TypeSignature::ARRAY_TYPE { ofType: ota.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::OPTION_TYPE { ofType: ota }, importpckg, tyVars) => {
                    let mut ota = (*ota).clone();
                    ota = fullyQualifyAstTypeSignature(ota.clone(), importpckg.clone(), tyVars.clone());
                    Ok(metamodelica::Ref::new(TypeSignature::OPTION_TYPE { ofType: ota.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::TUPLE_TYPE { ofTypes: typeLst }, importpckg, tyVars) => {
                    let mut typeLst = (*typeLst).clone();
                    typeLst = List::map2(typeLst.clone(), &fnptr!(fullyQualifyAstTypeSignature, metamodelica::Ref<TypeSignature>, metamodelica::Ref<PathIdent>, metamodelica::List<ArcStr>), importpckg.clone(), tyVars.clone())?;
                    Ok(metamodelica::Ref::new(TypeSignature::TUPLE_TYPE { ofTypes: typeLst.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ts @ Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: typeident } }, _, tyVars) => {
                    let true = (listMember(typeident.clone(), tyVars.clone())) else { return Err("pattern mismatch") };
                    Ok(ts.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: typeident } }, importpckg, _) => {
                    let mut na: metamodelica::Ref<PathIdent>;
                    let mut ts: metamodelica::Ref<TypeSignature>;
                    na = makePathIdent(metamodelica::AsArg::as_arg(&importpckg), typeident.clone())?;
                    ts = convertNameTypeIfIntrinsic(na.clone());
                    Ok(ts.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: na @ Deref @ PathIdent::PATH_IDENT { .. } }, _, _) => {
                    let mut ts: metamodelica::Ref<TypeSignature>;
                    ts = convertNameTypeIfIntrinsic(na.clone());
                    Ok(ts.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inASTDefTypeSignature.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outASTDefTypeSignature
}

pub(crate) fn convertNameTypeIfIntrinsic(
    mut inNameOfType: metamodelica::Ref<PathIdent>,
) -> metamodelica::Ref<TypeSignature> {
    let mut outTypeSignature: metamodelica::Ref<TypeSignature>;
    outTypeSignature = (::match_deref::match_deref! { match &(inNameOfType.clone()) {
        Deref @ PathIdent::PATH_IDENT { ident: Deref @ "Tpl", path: Deref @ PathIdent::IDENT { ident: Deref @ "Text" } } => crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE(),
        _ => metamodelica::Ref::new(TypeSignature::NAMED_TYPE { name: inNameOfType }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTypeSignature
}

pub(crate) fn fullyQualifyTemplateDef(
    mut inTemplateDef: TemplateDef,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> Result<TemplateDef> {
    let mut outTemplateDef: TemplateDef;
    outTemplateDef = 'mc: {
        let __mc_input = (inTemplateDef, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TemplateDef::LITERAL_DEF { value: r#str, litType }, astDefs) => {
                    let mut litType = (*litType).clone();
                    litType = fullyQualifyTemplateTypeSignature(metamodelica::AsArg::as_arg(&litType), astDefs.clone());
                    Ok(TemplateDef::LITERAL_DEF { value: r#str.clone(), litType: litType.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (def @ TemplateDef::STR_TOKEN_DEF { .. }, _) => {
                    Ok(def.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TemplateDef::TEMPLATE_DEF { args: targs, lesc, resc, exp: texp }, astDefs) => {
                    let mut targs = (*targs).clone();
                    targs = listMap1Tuple22(metamodelica::AsArg::as_arg(&targs), (std::sync::Arc::new(move |__a0: metamodelica::Ref<TypeSignature>, __a1: metamodelica::List<ASTDef>| -> metamodelica::Result<_> { ::std::result::Result::Ok(fullyQualifyTemplateTypeSignature(&__a0, __a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<TypeSignature>, metamodelica::List<ASTDef>) -> Result<metamodelica::Ref<TypeSignature>> + 'static>), astDefs.clone())?;
                    Ok(TemplateDef::TEMPLATE_DEF { args: targs.clone(), lesc: lesc.clone(), resc: resc.clone(), exp: texp.clone() })
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
                    Debug::trace(literal!("- fullyQualifyTemplateDef failed .\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTemplateDef)
}

pub(crate) fn fullyQualifyTemplateTypeSignature(
    mut inTemplateTypeSignature: &metamodelica::Ref<TypeSignature>,
    mut inASTDefs: metamodelica::List<ASTDef>,
) -> metamodelica::Ref<TypeSignature> {
    let mut outFullyQualifiedTypeSignature: metamodelica::Ref<TypeSignature>;
    outFullyQualifiedTypeSignature = 'mc: {
        let __mc_input = (&**inTemplateTypeSignature, inASTDefs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::LIST_TYPE { ofType: ota }, astDefs) => {
                    let mut ota = (*ota).clone();
                    ota = fullyQualifyTemplateTypeSignature(metamodelica::AsArg::as_arg(&ota), astDefs.clone());
                    Ok(metamodelica::Ref::new(TypeSignature::LIST_TYPE { ofType: ota.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::ARRAY_TYPE { ofType: ota }, astDefs) => {
                    let mut ota = (*ota).clone();
                    ota = fullyQualifyTemplateTypeSignature(metamodelica::AsArg::as_arg(&ota), astDefs.clone());
                    Ok(metamodelica::Ref::new(TypeSignature::ARRAY_TYPE { ofType: ota.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::OPTION_TYPE { ofType: ota }, astDefs) => {
                    let mut ota = (*ota).clone();
                    ota = fullyQualifyTemplateTypeSignature(metamodelica::AsArg::as_arg(&ota), astDefs.clone());
                    Ok(metamodelica::Ref::new(TypeSignature::OPTION_TYPE { ofType: ota.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::TUPLE_TYPE { ofTypes: typeLst }, astDefs) => {
                    let mut typeLst = (*typeLst).clone();
                    typeLst = List::map1(typeLst.clone(), &move |__a0: metamodelica::Ref<TypeSignature>, __a1: metamodelica::List<ASTDef>| -> metamodelica::Result<_> { ::std::result::Result::Ok(fullyQualifyTemplateTypeSignature(&__a0, __a1)) }, astDefs.clone())?;
                    Ok(metamodelica::Ref::new(TypeSignature::TUPLE_TYPE { ofTypes: typeLst.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: Deref @ "Text" } }, _) => {
                    Ok(crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: typepath }, astDefs) => {
                    let mut typeident: Ident;
                    let mut typepckg: metamodelica::Ref<PathIdent>;
                    let mut typepckgOpt: Option<metamodelica::Ref<PathIdent>>;
                    let mut typepath = (*typepath).clone();
                    (typepckgOpt, typeident) = splitPackageAndIdent(metamodelica::AsArg::as_arg(&typepath))?;
                    (typepckg, _) = getTypeInfo(typepckgOpt.clone(), typeident.clone(), astDefs.clone())?;
                    typepath = makePathIdent(&typepckg, typeident.clone())?;
                    Ok(metamodelica::Ref::new(TypeSignature::NAMED_TYPE { name: typepath.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inTemplateTypeSignature.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outFullyQualifiedTypeSignature
}

fn lookupTupleList<
    Type_a: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
    Type_b: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<(Type_a, Type_b)>,
    mut inItemA: Type_a,
) -> Result<Type_b> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inList, inItemA)) {
            (Deref @ metamodelica::ListNode::Cons { head: (a, itemB), tail: _ }, itemA) if (a.clone() == itemA.clone()) => {
                return Ok(itemB.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, itemA) => {
                { (inList, inItemA) = (rest.clone(), itemA.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn updateTupleList<
    Type_a: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
    Type_b: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<(Type_a, Type_b)>,
    mut inTuple: (Type_a, Type_b),
) -> metamodelica::List<(Type_a, Type_b)> {
    let mut outList: metamodelica::List<(Type_a, Type_b)>;
    outList = 'mc: {
        let __mc_input = (inList.clone(), &inTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lst, (a, _)) => {
                    lookupTupleList(lst.clone(), a.clone())?;
                    Ok(lst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::cons(inTuple.clone(), inList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outList
}

fn lookupDeleteTupleList<
    Type_a: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
    Type_b: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<(Type_a, Type_b)>,
    mut inItemA: Type_a,
) -> Result<(Type_b, metamodelica::List<(Type_a, Type_b)>)> {
    let mut outItemB: Type_b;
    let mut outList: metamodelica::List<(Type_a, Type_b)>;
    (outItemB, outList) = (::match_deref::match_deref! { match &((&**inList, inItemA)) {
        (Deref @ metamodelica::ListNode::Cons { head: (a, itemB), tail: rest }, itemA) if (a.clone() == itemA.clone()) => {
            (itemB.clone(), rest.clone())
        },
        (Deref @ metamodelica::ListNode::Cons { head: h, tail: rest }, itemA) => {
            let mut itemB: Type_b;
            let mut rest = (*rest).clone();
            (itemB, rest) = lookupDeleteTupleList(metamodelica::AsArg::as_arg(&rest), itemA.clone())?;
            (itemB, metamodelica::cons(h.clone(), rest.clone()))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outItemB, outList))
}

fn alignTupleList<
    Type_a: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
    Type_b: Clone + 'static + metamodelica::gc::MMTrace,
    Type_c: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inListToAlign: metamodelica::List<(Type_a, Type_b)>,
    mut inListAlignBy: &metamodelica::List<(Type_a, Type_c)>,
) -> Result<metamodelica::List<(Type_a, Type_b)>> {
    let mut outAlignedList: metamodelica::List<(Type_a, Type_b)>;
    outAlignedList = 'mc: {
        let __mc_input = (inListToAlign, &**inListAlignBy);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lstAl, Deref @ metamodelica::ListNode::Cons { head: (a, _), tail: lstBy }) => {
                    let mut b: Type_b;
                    let mut lst: metamodelica::List<(Type_a, Type_b)>;
                    b = lookupTupleList(lstAl.clone(), a.clone())?;
                    lst = alignTupleList(lstAl.clone(), metamodelica::AsArg::as_arg(&lstBy))?;
                    Ok(metamodelica::cons((a.clone(), b.clone()), lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lstAl, Deref @ metamodelica::ListNode::Cons { head: _, tail: lstBy }) => {
                    let mut lst: metamodelica::List<(Type_a, Type_b)>;
                    lst = alignTupleList(lstAl.clone(), metamodelica::AsArg::as_arg(&lstBy))?;
                    Ok(lst.clone())
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
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAlignedList)
}

fn listMap1Tuple22<
    Type_a: Clone + 'static + metamodelica::gc::MMTrace,
    Type_b: Clone + 'static + metamodelica::gc::MMTrace,
    Type_d: Clone + 'static + metamodelica::gc::MMTrace,
    Type_c: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<(Type_a, Type_b)>,
    mut inFun_Tbd_to_Tc: Arc<dyn ::std::ops::Fn(Type_b, Type_d) -> Result<Type_c> + 'static>,
    mut inExtraArg: Type_d,
) -> Result<metamodelica::List<(Type_a, Type_c)>> {
    pub type Fun_Tbd_to_Tc<Type_b: Clone + 'static, Type_c: Clone + 'static, Type_d: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_b, Type_d) -> Result<Type_c> + 'static>;

    let mut outList: metamodelica::List<(Type_a, Type_c)>;
    outList = (::match_deref::match_deref! { match inList {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: (a, itemB), tail: restB } => {
            let mut funBDtoC = inFun_Tbd_to_Tc.clone();
            let mut extarg = inExtraArg;
            let mut itemC: Type_c;
            let mut restC: metamodelica::List<(Type_a, Type_c)>;
            itemC = funBDtoC(itemB.clone(), extarg.clone())?;
            restC = listMap1Tuple22(restB, funBDtoC.clone(), extarg)?;
            metamodelica::cons((a.clone(), itemC), restC)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outList)
}

fn listMap2Tuple22<
    Type_a: Clone + 'static + metamodelica::gc::MMTrace,
    Type_b: Clone + 'static + metamodelica::gc::MMTrace,
    Type_d: Clone + 'static + metamodelica::gc::MMTrace,
    Type_e: Clone + 'static + metamodelica::gc::MMTrace,
    Type_c: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<(Type_a, Type_b)>,
    mut inFun_Tbde_to_Tc: Arc<dyn ::std::ops::Fn(Type_b, Type_d, Type_e) -> Result<Type_c> + 'static>,
    mut inExtraArg: Type_d,
    mut inExtraArg2: Type_e,
) -> Result<metamodelica::List<(Type_a, Type_c)>> {
    pub type Fun_Tbde_to_Tc<
        Type_b: Clone + 'static,
        Type_c: Clone + 'static,
        Type_d: Clone + 'static,
        Type_e: Clone + 'static,
    > = std::sync::Arc<dyn ::std::ops::Fn(Type_b, Type_d, Type_e) -> Result<Type_c> + 'static>;

    let mut outList: metamodelica::List<(Type_a, Type_c)>;
    outList = (::match_deref::match_deref! { match inList {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: (a, itemB), tail: restB } => {
            let mut funBDEtoC = inFun_Tbde_to_Tc.clone();
            let mut extarg = inExtraArg;
            let mut extarg2 = inExtraArg2;
            let mut itemC: Type_c;
            let mut restC: metamodelica::List<(Type_a, Type_c)>;
            itemC = funBDEtoC(itemB.clone(), extarg.clone(), extarg2.clone())?;
            restC = listMap2Tuple22(restB, funBDEtoC.clone(), extarg, extarg2)?;
            metamodelica::cons((a.clone(), itemC), restC)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outList)
}

//**************************************
// *** debug output functions
//**************************************
pub(crate) fn addSusanError(mut inErrMsg: ArcStr, mut inInfo: &SourceInfo) -> Result<()> {
    if Flags::isSet(Flags::FAILTRACE.clone())? {
        Debug::traceln({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Error - "));
            __mm_s.push_str(&*inErrMsg);
            ArcStr::from(__mm_s)
        })?;
    }
    Error::addSourceMessage(&(Error::SUSAN_ERROR.clone()), list![inErrMsg], inInfo)?;
    Ok(())
}

fn addSusanNotification(mut inErrMsg: ArcStr, mut inInfo: &SourceInfo) -> Result<()> {
    Error::addSourceMessage(&(Error::SUSAN_NOTIFY.clone()), list![inErrMsg], inInfo)?;
    Ok(())
}

pub(crate) fn canBeEscapedUnquoted(mut inStringList: metamodelica::List<ArcStr>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inStringList) {
            Deref @ metamodelica::ListNode::Cons { head: r#str, tail: Deref @ metamodelica::ListNode::Nil } if (((r#str).len() as i32) > 0 && canBeEscapedUnquotedChars(&(stringListStringChar(r#str.clone())))) => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: r#str, tail: rest @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } if (((r#str).len() as i32) > 0 && canBeEscapedUnquotedChars(&(stringListStringChar(r#str.clone())))) => {
                { inStringList = rest.clone(); continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn canBeEscapedUnquotedChars<'__b>(mut inChars: &'__b metamodelica::List<ArcStr>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inChars {
            Deref @ metamodelica::ListNode::Nil => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: c, tail: chars } if (metamodelica::stringEq(&c, &(literal!("'"))) || metamodelica::stringEq(&c, &(literal!("\""))) || metamodelica::stringEq(&c, &(literal!("?"))) || metamodelica::stringEq(&c, &(literal!("\\"))) || metamodelica::stringEq(&c, &(literal!("\n"))) || metamodelica::stringEq(&c, &(literal!("\t"))) || metamodelica::stringEq(&c, &(literal!(" ")))) => {
                { inChars = chars; continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn canBeOnOneLine(mut inStringList: metamodelica::List<ArcStr>) -> bool {
    let mut outCanBeOnOneLine: bool;
    outCanBeOnOneLine = ((inStringList).len() as i32) <= 4 && ((stringAppendList(inStringList)).len() as i32) <= 10;
    outCanBeOnOneLine
}

pub(crate) fn pathIdentString(mut inPathIndent: &metamodelica::Ref<PathIdent>) -> Result<ArcStr> {
    let mut outPathIdentString: ArcStr;
    outPathIdentString = (match &**inPathIndent {
        PathIdent::IDENT { ident } => ident.clone(),
        PathIdent::PATH_IDENT { ident, path } => {
            let mut ident = (*ident).clone();
            ident = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ident);
                __mm_s.push_str(&*literal!("."));
                __mm_s.push_str(&*pathIdentString(path)?);
                ArcStr::from(__mm_s)
            };
            ident.clone()
        }
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("-!!!pathIdentString failed.\n"))?;
            return Err("fail");
        }
    });
    Ok(outPathIdentString)
}

pub(crate) static eTxt: std::sync::LazyLock<Tpl::Text> = std::sync::LazyLock::new(|| Tpl::emptyTxt.clone());

pub(crate) fn typeSignatureString(mut inTS: &metamodelica::Ref<TypeSignature>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    let mut txt: Tpl::Text;
    txt = TplCodegen::typeSig(eTxt.clone(), inTS)?;
    outStr = Tpl::textString(txt)?;
    Ok(outStr)
}

pub(crate) fn mmExpString(mut inMMExp: metamodelica::Ref<MMExp>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    let mut txt: Tpl::Text;
    txt = TplCodegen::mmExp(eTxt.clone(), inMMExp, literal!("="))?;
    outStr = Tpl::textString(txt)?;
    Ok(outStr)
}

pub(crate) fn stmtsString(mut inStmts: &metamodelica::List<metamodelica::Ref<MMExp>>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    let mut txt: Tpl::Text;
    txt = TplCodegen::mmStatements(eTxt.clone(), inStmts)?;
    outStr = Tpl::textString(txt)?;
    Ok(outStr)
}

pub(crate) fn removeUnusedImports(mut pkg: MMPackage) -> Result<MMPackage> {
    let mut pkg: MMPackage = pkg;
    let mut set: metamodelica::Ref<AvlSetString::Tree>;
    let mut name: metamodelica::Ref<PathIdent>;
    let mut b: bool;
    set = openmodelica_util::AvlSetString::Tree::interned_EMPTY();
    for mut e in &*pkg.mmDeclarations.clone() {
        let () = (match e.clone() {
            MMDeclaration::MM_FUN { .. } => {
                set = addTypedIdentsToSet(set, var_field!(e.inArgs, MMDeclaration::MM_FUN))?;
                set = addTypedIdentsToSet(set, var_field!(e.outArgs, MMDeclaration::MM_FUN))?;
                set = addTypedIdentsToSet(set, var_field!(e.locals, MMDeclaration::MM_FUN))?;
                for mut exp in &*var_field!(e.statements, MMDeclaration::MM_FUN).clone() {
                    set = addExpToSet(set, metamodelica::AsArg::as_arg(&exp))?;
                }
                ()
            }
            _ => (),
        });
    }
    pkg.mmDeclarations = ({
        let mut __acc: metamodelica::List<MMDeclaration> = metamodelica::nil();
        for mut elt in (pkg.mmDeclarations.clone()).into_iter().cloned() {
            if !(match elt.clone() {
                MMDeclaration::MM_IMPORT {
                    packageName: ref __esc_name,
                    ..
                } => {
                    name = __esc_name.clone();
                    b = AvlSetString::hasKey(set.clone(), getPackageIdent(metamodelica::AsArg::as_arg(&name))?)?;
                    if !(b) && Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("removeUnusedImports: "));
                            __mm_s.push_str(&*encodePathIdent(metamodelica::AsArg::as_arg(&name), &(literal!("")))?);
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        })?;
                    }
                    b
                }
                _ => true,
            }) {
                continue;
            }
            let __x = elt.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(pkg)
}

fn addTypedIdentsToSet(
    mut set: metamodelica::Ref<AvlSetString::Tree>,
    mut ids: &TypedIdents,
) -> Result<metamodelica::Ref<AvlSetString::Tree>> {
    let mut set: metamodelica::Ref<AvlSetString::Tree> = set;
    let mut sig: metamodelica::Ref<TypeSignature>;
    for mut tpl in &**ids {
        (_, sig) = tpl.clone();
        set = addTypeSignatureToSet(set, sig)?;
    }
    Ok(set)
}

fn addTypeSignatureToSet(
    mut set: metamodelica::Ref<AvlSetString::Tree>,
    mut sig: metamodelica::Ref<TypeSignature>,
) -> Result<metamodelica::Ref<AvlSetString::Tree>> {
    '__tco: loop {
        let mut sig2: metamodelica::Ref<TypeSignature>;
        let mut sigs: metamodelica::List<metamodelica::Ref<TypeSignature>>;
        let mut name: metamodelica::Ref<PathIdent>;
        match &*sig {
            TypeSignature::LIST_TYPE { ofType: __esc_sig2 } => {
                sig2 = (*__esc_sig2).clone();
                {
                    (set, sig) = (set, sig2.clone());
                    continue '__tco;
                }
            }
            TypeSignature::ARRAY_TYPE { ofType: __esc_sig2 } => {
                sig2 = (*__esc_sig2).clone();
                {
                    (set, sig) = (set, sig2.clone());
                    continue '__tco;
                }
            }
            TypeSignature::OPTION_TYPE { ofType: __esc_sig2 } => {
                sig2 = (*__esc_sig2).clone();
                {
                    (set, sig) = (set, sig2.clone());
                    continue '__tco;
                }
            }
            TypeSignature::TUPLE_TYPE { ofTypes: __esc_sigs } => {
                sigs = (*__esc_sigs).clone();
                return Ok(List::foldr(
                    metamodelica::AsArg::as_arg(&sigs),
                    &addTypeSignatureToSet,
                    set,
                )?);
            }
            TypeSignature::NAMED_TYPE { name: __esc_name } => {
                name = (*__esc_name).clone();
                return Ok(addPathIdentToSet(set, metamodelica::AsArg::as_arg(&name))?);
            }
            _ => return Ok(set),
        }
    }
}

fn addPathIdentToSet(
    mut set: metamodelica::Ref<AvlSetString::Tree>,
    mut name: &metamodelica::Ref<PathIdent>,
) -> Result<metamodelica::Ref<AvlSetString::Tree>> {
    let mut set: metamodelica::Ref<AvlSetString::Tree> = set;
    set = (match &**name {
        PathIdent::IDENT { ident: __name_ident } => AvlSetString::add(set, metamodelica::AsArg::as_arg(&__name_ident))?,
        PathIdent::PATH_IDENT {
            ident: __name_ident, ..
        } => AvlSetString::add(set, metamodelica::AsArg::as_arg(&__name_ident))?,
    });
    Ok(set)
}

fn addExpToSet<'__b>(
    mut set: metamodelica::Ref<AvlSetString::Tree>,
    mut exp: &'__b metamodelica::Ref<MMExp>,
) -> Result<metamodelica::Ref<AvlSetString::Tree>> {
    '__tco: loop {
        match &**exp {
            MMExp::MM_ASSIGN { .. } => {
                (set, exp) = (set, var_field!((**exp).rhs, MMExp::MM_ASSIGN));
                continue '__tco;
            }
            MMExp::MM_FN_CALL { .. } => {
                return Ok(List::foldr(
                    var_field!((**exp).args, MMExp::MM_FN_CALL),
                    &move |__a0: metamodelica::Ref<AvlSetString::Tree>, __a1: metamodelica::Ref<MMExp>| {
                        addExpToSet(__a0, &__a1)
                    },
                    addPathIdentToSet(set, var_field!((**exp).fnName, MMExp::MM_FN_CALL))?,
                )?);
            }
            MMExp::MM_IDENT { .. } => return Ok(addPathIdentToSet(set, var_field!((**exp).ident, MMExp::MM_IDENT))?),
            MMExp::MM_MATCH { .. } => {
                return Ok(List::foldr(
                    var_field!((**exp).matchCases, MMExp::MM_MATCH),
                    &move |__a0: metamodelica::Ref<AvlSetString::Tree>,
                           __a1: (
                        metamodelica::List<metamodelica::Ref<MatchingExp>>,
                        metamodelica::List<metamodelica::Ref<MMExp>>,
                    )| addMatchCaseToSet(__a0, &__a1),
                    set,
                )?);
            }
            MMExp::MM_LIST_FOR_LOOP { .. } => {
                return Ok(List::foldr(
                    var_field!((**exp).matchCases, MMExp::MM_LIST_FOR_LOOP),
                    &move |__a0: metamodelica::Ref<AvlSetString::Tree>,
                           __a1: (
                        metamodelica::List<metamodelica::Ref<MatchingExp>>,
                        metamodelica::List<metamodelica::Ref<MMExp>>,
                    )| addMatchCaseToSet(__a0, &__a1),
                    set,
                )?);
            }
            _ => return Ok(set),
        }
    }
}

fn addMatchCaseToSet(
    mut set: metamodelica::Ref<AvlSetString::Tree>,
    mut c: &MMMatchCase,
) -> Result<metamodelica::Ref<AvlSetString::Tree>> {
    let mut set: metamodelica::Ref<AvlSetString::Tree> = set;
    let mut mexps: metamodelica::List<metamodelica::Ref<MatchingExp>>;
    let mut exps: metamodelica::List<metamodelica::Ref<MMExp>>;
    (mexps, exps) = c.clone();
    set = List::foldr(
        &exps,
        &move |__a0: metamodelica::Ref<AvlSetString::Tree>, __a1: metamodelica::Ref<MMExp>| addExpToSet(__a0, &__a1),
        set,
    )?;
    set = List::foldr(&mexps, &addMatchingExpToSet, set)?;
    Ok(set)
}

fn addMatchingExpToSet(
    mut set: metamodelica::Ref<AvlSetString::Tree>,
    mut exp: metamodelica::Ref<MatchingExp>,
) -> Result<metamodelica::Ref<AvlSetString::Tree>> {
    '__tco: loop {
        let mut e: metamodelica::Ref<MatchingExp>;
        match &*exp {
            MatchingExp::BIND_AS_MATCH {
                matchingExp: __exp_matchingExp,
                ..
            } => {
                (set, exp) = (set, __exp_matchingExp.clone());
                continue '__tco;
            }
            MatchingExp::RECORD_MATCH {
                fieldMatchings: __exp_fieldMatchings,
                tagName: __exp_tagName,
            } => {
                set = addPathIdentToSet(set, metamodelica::AsArg::as_arg(&__exp_tagName))?;
                for mut tpl in &*__exp_fieldMatchings.clone() {
                    (_, e) = tpl.clone();
                    set = addMatchingExpToSet(set, e)?;
                }
                return Ok(set);
            }
            MatchingExp::SOME_MATCH { value: __exp_value } => {
                (set, exp) = (set, __exp_value.clone());
                continue '__tco;
            }
            MatchingExp::TUPLE_MATCH {
                tupleArgs: __exp_tupleArgs,
            } => {
                return Ok(List::foldr(
                    metamodelica::AsArg::as_arg(&__exp_tupleArgs),
                    &addMatchingExpToSet,
                    set,
                )?);
            }
            MatchingExp::LIST_MATCH {
                listElts: __exp_listElts,
            } => {
                return Ok(List::foldr(
                    metamodelica::AsArg::as_arg(&__exp_listElts),
                    &addMatchingExpToSet,
                    set,
                )?);
            }
            MatchingExp::LIST_CONS_MATCH {
                head: __exp_head,
                rest: __exp_rest,
            } => {
                (set, exp) = (addMatchingExpToSet(set, __exp_head.clone())?, __exp_rest.clone());
                continue '__tco;
            }
            _ => return Ok(set),
        }
    }
}
