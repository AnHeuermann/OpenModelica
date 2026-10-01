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

use crate::BaseModelica;
use crate::NFAlgorithm as Algorithm;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnectBreakTree;
use crate::NFDimension as Dimension;
use crate::NFEquation;
use crate::NFExpression as Expression;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFFunctionDerivative as FunctionDerivative;
use crate::NFFunctionInverse as FunctionInverse;
use crate::NFInst as Inst;
use crate::NFInst::InstSettings;
use crate::NFInstContext as InstContext;
use crate::NFInstNode;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFLookupState::LookupState;
use crate::NFModifier::Modifier;
use crate::NFOperatorOverloading as OperatorOverloading;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::*;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTypeCheck::MatchKind;
use crate::NFTyping as Typing;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::InstBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Graph;
use openmodelica_util::IOStream;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

pub type NamedArg = (ArcStr, metamodelica::Ref<Expression::NFExpression>);

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct TypedArg {
    pub name: Option<ArcStr>,
    pub value: metamodelica::Ref<Expression::NFExpression>,
    pub ty: metamodelica::Ref<Type::NFType>,
    pub var: Prefixes::Variability,
    pub purity: Prefixes::Purity,
}

impl metamodelica::gc::MMTrace for TypedArg {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.value, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ty, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.var, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.purity, __mmv)?;
        Ok(())
    }
}
impl Default for TypedArg {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value: Default::default(),
            ty: Default::default(),
            var: Default::default(),
            purity: Default::default(),
        }
    }
}

pub type TYPED_ARG = TypedArg;

/// Determines which type of argument a slot accepts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum SlotType {
    /// Only accepts positional arguments.
    POSITIONAL = 1,
    /// Only accepts named argument.
    NAMED = 2,
    /// Accepts both positional and named arguments.
    GENERIC = 3,
}
impl PartialOrd for SlotType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SlotType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for SlotType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for SlotType {
    fn default() -> Self {
        Self::POSITIONAL
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum SlotEvalStatus {
    NOT_EVALUATED = 1,
    EVALUATING = 2,
    EVALUATED = 3,
}
impl PartialOrd for SlotEvalStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SlotEvalStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for SlotEvalStatus {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for SlotEvalStatus {
    fn default() -> Self {
        Self::NOT_EVALUATED
    }
}

pub mod Slot {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Slot {
        pub node: metamodelica::Ref<InstNode::InstNode>,
        pub ty: SlotType,
        pub default: Option<metamodelica::Ref<Expression::NFExpression>>,
        pub arg: Option<metamodelica::Ref<TypedArg>>,
        pub index: i32,
        pub evalStatus: SlotEvalStatus,
    }

    impl metamodelica::gc::MMTrace for Slot {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.node, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.ty, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.default, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.arg, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.evalStatus, __mmv)?;
            Ok(())
        }
    }
    impl Default for Slot {
        fn default() -> Self {
            Self {
                node: Default::default(),
                ty: Default::default(),
                default: Default::default(),
                arg: Default::default(),
                index: Default::default(),
                evalStatus: Default::default(),
            }
        }
    }

    pub type SLOT = Slot;

    pub(crate) fn positional(mut slot: &metamodelica::Ref<Slot>) -> bool {
        let mut pos: bool;
        pos = (match slot.ty.clone() {
            SlotType::POSITIONAL => true,
            SlotType::GENERIC => true,
            _ => false,
        });
        pos
    }

    pub(crate) fn named(mut slot: &metamodelica::Ref<Slot>) -> bool {
        let mut pos: bool;
        pos = (match slot.ty.clone() {
            SlotType::NAMED => true,
            SlotType::GENERIC => true,
            _ => false,
        });
        pos
    }

    pub(crate) fn name(mut slot: &metamodelica::Ref<Slot>) -> Result<ArcStr> {
        let mut name: ArcStr = InstNode::name(&slot.node)?;
        Ok(name)
    }

    pub(crate) fn hasNode(
        mut node: &metamodelica::Ref<InstNode::InstNode>,
        mut slot: &metamodelica::Ref<Slot>,
    ) -> Result<bool> {
        let mut hasNode: bool = InstNode::refEqual(node, &slot.node)?;
        Ok(hasNode)
    }
}

pub mod FunctionMatchKind {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum FunctionMatchKind {
        /// Exact match.
        EXACT,
        /// Matched by casting one or more arguments. e.g. Integer to Real
        CAST,
        /// Matched with a generic type on one or more arguments e.g. function F<T> input T i; end F; F(1)
        GENERIC,
        /// Matched by vectorization
        VECTORIZED {
            vectDims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
            vectorizedArgs: metamodelica::List<i32>,
            baseMatch: metamodelica::Ref<FunctionMatchKind>,
        },
        NOT_COMPATIBLE,
    }
    impl metamodelica::gc::MMTrace for FunctionMatchKind {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                FunctionMatchKind::EXACT => Ok(()),
                FunctionMatchKind::CAST => Ok(()),
                FunctionMatchKind::GENERIC => Ok(()),
                FunctionMatchKind::VECTORIZED {
                    vectDims,
                    vectorizedArgs,
                    baseMatch,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(vectDims, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(vectorizedArgs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(baseMatch, __mmv)?;
                    Ok(())
                }
                FunctionMatchKind::NOT_COMPATIBLE => Ok(()),
            }
        }
    }
    impl FunctionMatchKind {
        pub fn interned_EXACT() -> metamodelica::Ref<FunctionMatchKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<FunctionMatchKind> = metamodelica::Ref::new(FunctionMatchKind::EXACT);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_CAST() -> metamodelica::Ref<FunctionMatchKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<FunctionMatchKind> = metamodelica::Ref::new(FunctionMatchKind::CAST);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_GENERIC() -> metamodelica::Ref<FunctionMatchKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<FunctionMatchKind> = metamodelica::Ref::new(FunctionMatchKind::GENERIC);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_NOT_COMPATIBLE() -> metamodelica::Ref<FunctionMatchKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<FunctionMatchKind> = metamodelica::Ref::new(FunctionMatchKind::NOT_COMPATIBLE);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_EXACT() -> metamodelica::Ref<FunctionMatchKind> {
        FunctionMatchKind::interned_EXACT()
    }
    pub fn interned_CAST() -> metamodelica::Ref<FunctionMatchKind> {
        FunctionMatchKind::interned_CAST()
    }
    pub fn interned_GENERIC() -> metamodelica::Ref<FunctionMatchKind> {
        FunctionMatchKind::interned_GENERIC()
    }
    pub fn interned_NOT_COMPATIBLE() -> metamodelica::Ref<FunctionMatchKind> {
        FunctionMatchKind::interned_NOT_COMPATIBLE()
    }
    impl Default for FunctionMatchKind {
        fn default() -> Self {
            Self::EXACT
        }
    }
    pub use self::FunctionMatchKind::{CAST, EXACT, GENERIC, NOT_COMPATIBLE, VECTORIZED};
    pub(crate) fn isValid(mut mk: &metamodelica::Ref<FunctionMatchKind>) -> bool {
        let mut b: bool;
        b = (match &**mk {
            NOT_COMPATIBLE { .. } => false,
            _ => true,
        });
        b
    }

    pub(crate) fn isExact(mut mk: &metamodelica::Ref<FunctionMatchKind>) -> bool {
        let mut b: bool;
        b = (match &**mk {
            EXACT { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn isVectorized(mut mk: &metamodelica::Ref<FunctionMatchKind>) -> bool {
        let mut b: bool;
        b = (match &**mk {
            VECTORIZED { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn isExactVectorized(mut mk: &metamodelica::Ref<FunctionMatchKind>) -> bool {
        let mut b: bool;
        b = (::match_deref::match_deref! { match mk {
            Deref @ VECTORIZED { baseMatch: Deref @ EXACT { .. }, .. } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        b
    }
}

thread_local! { static __EXACT_MATCH_TLS: metamodelica::Ref<FunctionMatchKind::FunctionMatchKind> = crate::NFFunction::FunctionMatchKind::interned_EXACT(); }
pub(crate) fn EXACT_MATCH() -> metamodelica::Ref<FunctionMatchKind::FunctionMatchKind> {
    __EXACT_MATCH_TLS.with(|__t| __t.clone())
}

thread_local! { static __CAST_MATCH_TLS: metamodelica::Ref<FunctionMatchKind::FunctionMatchKind> = crate::NFFunction::FunctionMatchKind::interned_CAST(); }
pub(crate) fn CAST_MATCH() -> metamodelica::Ref<FunctionMatchKind::FunctionMatchKind> {
    __CAST_MATCH_TLS.with(|__t| __t.clone())
}

thread_local! { static __GENERIC_MATCH_TLS: metamodelica::Ref<FunctionMatchKind::FunctionMatchKind> = crate::NFFunction::FunctionMatchKind::interned_GENERIC(); }
pub(crate) fn GENERIC_MATCH() -> metamodelica::Ref<FunctionMatchKind::FunctionMatchKind> {
    __GENERIC_MATCH_TLS.with(|__t| __t.clone())
}

thread_local! { static __NO_MATCH_TLS: metamodelica::Ref<FunctionMatchKind::FunctionMatchKind> = crate::NFFunction::FunctionMatchKind::interned_NOT_COMPATIBLE(); }
pub(crate) fn NO_MATCH() -> metamodelica::Ref<FunctionMatchKind::FunctionMatchKind> {
    __NO_MATCH_TLS.with(|__t| __t.clone())
}

pub mod MatchedFunction {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct MatchedFunction {
        pub func: metamodelica::Ref<Function::Function>,
        pub args: metamodelica::List<metamodelica::Ref<TypedArg>>,
        pub mk: metamodelica::Ref<FunctionMatchKind::FunctionMatchKind>,
    }

    impl metamodelica::gc::MMTrace for MatchedFunction {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.func, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.args, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.mk, __mmv)?;
            Ok(())
        }
    }
    impl Default for MatchedFunction {
        fn default() -> Self {
            Self {
                func: Default::default(),
                args: Default::default(),
                mk: Default::default(),
            }
        }
    }

    pub type MATCHED_FUNC = MatchedFunction;

    pub(crate) fn getExactMatches(
        mut matchedFunctions: metamodelica::List<metamodelica::Ref<MatchedFunction>>,
    ) -> metamodelica::List<metamodelica::Ref<MatchedFunction>> {
        let mut outFuncs: metamodelica::List<metamodelica::Ref<MatchedFunction>> = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<MatchedFunction>> = metamodelica::nil();
            for mut mf in (matchedFunctions.clone()).into_iter().cloned() {
                if !(FunctionMatchKind::isExact(&(mf.mk.clone()))) {
                    continue;
                }
                let __x = mf.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        outFuncs
    }

    pub(crate) fn getExactVectorizedMatches(
        mut matchedFunctions: metamodelica::List<metamodelica::Ref<MatchedFunction>>,
    ) -> metamodelica::List<metamodelica::Ref<MatchedFunction>> {
        let mut outFuncs: metamodelica::List<metamodelica::Ref<MatchedFunction>> = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<MatchedFunction>> = metamodelica::nil();
            for mut mf in (matchedFunctions.clone()).into_iter().cloned() {
                if !(FunctionMatchKind::isExactVectorized(&(mf.mk.clone()))) {
                    continue;
                }
                let __x = mf.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        outFuncs
    }

    pub(crate) fn isVectorized(mut mf: &metamodelica::Ref<MatchedFunction>) -> bool {
        let mut b: bool = FunctionMatchKind::isVectorized(&mf.mk);
        b
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum FunctionStatus {
    /// A builtin function.
    BUILTIN = 1,
    /// The initial status.
    INITIAL = 2,
    /// Constants in the function has been evaluated by EvalConstants.
    EVALUATED = 3,
    /// The function has been simplified by SimplifyModel.
    SIMPLIFIED = 4,
    /// The function has been added to the function tree.
    COLLECTED = 5,
}
impl PartialOrd for FunctionStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for FunctionStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for FunctionStatus {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for FunctionStatus {
    fn default() -> Self {
        Self::BUILTIN
    }
}

pub mod Function {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Function {
        pub path: metamodelica::Ref<Absyn::Path>,
        /// Weakly: the function's scope owns it, and its
        ///      cache holds this function.
        pub node: metamodelica::Ref<NFInstNode::NodeHandle>,
        pub inputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        /// Weakly: a record constructor's output
        ///      is typed by the constructor node, whose cache holds this function.
        pub outputs: metamodelica::List<metamodelica::Ref<NFInstNode::NodeHandle>>,
        pub locals: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        pub interfaceDiffInfo:
            Option<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>>,
        pub slots: metamodelica::List<metamodelica::Ref<Slot::Slot>>,
        pub returnType: metamodelica::Ref<Type::NFType>,
        pub attributes: DAE::FunctionAttributes,
        pub derivatives: metamodelica::List<metamodelica::Ref<FunctionDerivative::NFFunctionDerivative>>,
        pub derivedInputs: metamodelica::List<i32>,
        pub inverses: metamodelica::Array<metamodelica::Ref<FunctionInverse::NFFunctionInverse>>,
        pub status: Pointer::Pointer<FunctionStatus>,
        /// Used during function evaluation to limit recursion.
        pub callCounter: Pointer::Pointer<i32>,
    }

    impl metamodelica::gc::MMTrace for Function {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.path, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.node, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.inputs, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.outputs, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.locals, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.interfaceDiffInfo, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.slots, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.returnType, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.attributes, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.derivatives, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.derivedInputs, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.inverses, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.status, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.callCounter, __mmv)?;
            Ok(())
        }
    }
    impl Default for Function {
        fn default() -> Self {
            Self {
                path: Default::default(),
                node: Default::default(),
                inputs: Default::default(),
                outputs: Default::default(),
                locals: Default::default(),
                interfaceDiffInfo: Default::default(),
                slots: Default::default(),
                returnType: Default::default(),
                attributes: Default::default(),
                derivatives: Default::default(),
                derivedInputs: Default::default(),
                inverses: Default::default(),
                status: Default::default(),
                callCounter: Default::default(),
            }
        }
    }

    pub type FUNCTION = Function;

    pub(crate) fn new(
        mut path: metamodelica::Ref<Absyn::Path>,
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut comments: &metamodelica::List<metamodelica::Ref<SCode::Comment>>,
    ) -> Result<metamodelica::Ref<Function>> {
        let mut r#fn: metamodelica::Ref<Function>;
        let mut inputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
        let mut outputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
        let mut locals: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
        let mut attr: DAE::FunctionAttributes;
        let mut status: FunctionStatus;
        (inputs, outputs, locals) = collectParams(
            node.clone(),
            metamodelica::nil(),
            metamodelica::nil(),
            metamodelica::nil(),
        )?;
        attr = makeAttributes(node.clone(), inputs.clone(), outputs.clone(), comments)?;
        status = if (isBuiltinAttr(&attr)) {
            FunctionStatus::COLLECTED.clone()
        } else {
            FunctionStatus::INITIAL.clone()
        };
        r#fn = metamodelica::Ref::new(Function {
            path: path,
            node: InstNode::handle(node)?,
            inputs: inputs,
            outputs: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFInstNode::NodeHandle>> = metamodelica::nil();
                for mut o in (outputs).into_iter().cloned() {
                    let __x = InstNode::handle(o.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            locals: locals,
            interfaceDiffInfo: None,
            slots: metamodelica::nil(),
            returnType: crate::NFType::interned_UNKNOWN(),
            attributes: attr,
            derivatives: metamodelica::nil(),
            derivedInputs: metamodelica::nil(),
            inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
            status: Pointer::create(status),
            callCounter: Pointer::create(0),
        });
        Ok(r#fn)
    }

    pub(crate) fn lookupFunctionSimple(
        mut functionName: ArcStr,
        mut scope: metamodelica::Ref<InstNode::InstNode>,
        mut context: i32,
    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
        let mut functionRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut found_scope: metamodelica::Ref<InstNode::InstNode>;
        let mut prefix: metamodelica::Ref<ComponentRef::NFComponentRef>;
        (functionRef, found_scope) = Lookup::lookupFunctionNameSilent(
            &(metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                name: functionName,
                subscripts: metamodelica::nil(),
            })),
            scope,
            context,
        )?;
        prefix = ComponentRef::fromNodeList(&(InstNode::scopeList(found_scope, false, metamodelica::nil())?))?;
        functionRef = ComponentRef::append(functionRef, &prefix)?;
        Ok(functionRef)
    }

    pub(crate) fn lookupFunction(
        mut functionName: metamodelica::Ref<Absyn::ComponentRef>,
        mut scope: metamodelica::Ref<InstNode::InstNode>,
        mut context: i32,
        mut info: SourceInfo,
    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
        let mut functionRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut found_scope: metamodelica::Ref<InstNode::InstNode>;
        let mut functionPath: metamodelica::Ref<Absyn::Path>;
        let mut prefix: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut is_class: bool;
        if let Ok(__iflet0) = AbsynUtil::crefToPath(&functionName) {
            functionPath = __iflet0;
        } else {
            Error::addSourceMessageAndFail(
                &(Error::SUBSCRIPTED_FUNCTION_CALL.clone()),
                list![Dump::printComponentRefStr(&functionName)?],
                &info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        (functionRef, found_scope) = Lookup::lookupFunctionName(functionName, scope, context, info)?;
        is_class = InstNode::isClass(&(ComponentRef::node(&functionRef)?))?;
        prefix = ComponentRef::fromNodeList(&(InstNode::scopeList(found_scope, is_class, metamodelica::nil())?))?;
        functionRef = ComponentRef::append(functionRef, &prefix)?;
        Ok(functionRef)
    }

    pub(crate) fn instFunction(
        mut functionName: metamodelica::Ref<Absyn::ComponentRef>,
        mut scope: metamodelica::Ref<InstNode::InstNode>,
        mut context: i32,
        mut info: SourceInfo,
    ) -> Result<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<InstNode::InstNode>,
        bool,
    )> {
        let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut fn_node: metamodelica::Ref<InstNode::InstNode>;
        let mut specialBuiltin: bool;
        fn_ref = lookupFunction(functionName, scope, context, info.clone())?;
        (fn_ref, fn_node, specialBuiltin) = instFunctionRef(fn_ref, context, info)?;
        Ok((fn_ref, fn_node, specialBuiltin))
    }

    pub(crate) fn instFunctionRef(
        mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut context: i32,
        mut info: SourceInfo,
    ) -> Result<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<InstNode::InstNode>,
        bool,
    )> {
        let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef> = fn_ref;
        let mut fn_node: metamodelica::Ref<InstNode::InstNode>;
        let mut specialBuiltin: bool;
        let mut cache: metamodelica::Ref<CachedData::CachedData>;
        let mut parent: metamodelica::Ref<InstNode::InstNode>;
        fn_node = InstNode::classScope(ComponentRef::node(&fn_ref)?)?;
        cache = InstNode::getFuncCache(&fn_node)?;
        (fn_node, specialBuiltin) = (match &*cache {
            CachedData::FUNCTION {
                specialBuiltin: __cache_specialBuiltin,
                ..
            } => (fn_node, __cache_specialBuiltin.clone()),
            _ => {
                parent = if (InstNode::isRedeclare(ComponentRef::node(&fn_ref)?)? || ComponentRef::isSimple(&fn_ref)) {
                    crate::NFInstNode::InstNode::interned_EMPTY_NODE()
                } else {
                    ComponentRef::node(&(ComponentRef::rest(&fn_ref)?))?
                };
                if !(InstNode::isComponent(&parent)?) {
                    parent = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
                }
                instFunction2(ComponentRef::toPath(&fn_ref)?, fn_node, context, info, parent)?
            }
        });
        Ok((fn_ref, fn_node, specialBuiltin))
    }

    pub(crate) fn instFunctionNode(
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut context: i32,
        mut info: SourceInfo,
    ) -> Result<metamodelica::Ref<InstNode::InstNode>> {
        let mut node: metamodelica::Ref<InstNode::InstNode> = node;
        let mut cache: metamodelica::Ref<CachedData::CachedData>;
        cache = InstNode::getFuncCache(&node)?;
        let () = (match &*cache {
            CachedData::FUNCTION { .. } => (),
            _ => {
                (node, _) = instFunction2(
                    InstNode::fullPath(node.clone(), false)?,
                    node,
                    context,
                    info,
                    crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
                )?;
                ()
            }
        });
        Ok(node)
    }

    pub(crate) fn instFunction2(
        mut fnPath: metamodelica::Ref<Absyn::Path>,
        mut fnNode: metamodelica::Ref<InstNode::InstNode>,
        mut context: i32,
        mut info: SourceInfo,
        mut parent: metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<(metamodelica::Ref<InstNode::InstNode>, bool)> {
        let mut fnNode: metamodelica::Ref<InstNode::InstNode> = fnNode;
        let mut specialBuiltin: bool;
        let mut def: metamodelica::Ref<SCode::Element> = InstNode::definition(fnNode.clone())?;
        (fnNode, specialBuiltin) = (::match_deref::match_deref! { match &(&*def) {
            Deref @ SCode::Element::CLASS { .. } if (SCodeUtil::isOperatorRecord(&def)) => {
                (fnNode, _) = instFunction3(fnNode, context, &info)?;
                fnNode = OperatorOverloading::instConstructor(fnPath, fnNode, context, info)?;
                (fnNode.clone(), false)
            },
            Deref @ SCode::Element::CLASS { .. } if (SCodeUtil::isRecord(&def)) => {
                (fnNode, _) = instFunction3(fnNode, context, &info)?;
                fnNode = Record::instDefaultConstructor(fnPath, fnNode, context, &info)?;
                (fnNode.clone(), false)
            },
            Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_OPERATOR { .. }, classDef: Deref @ SCode::ClassDef::PARTS { .. }, .. } => {
                (fnNode, _) = instFunction3(fnNode, context, &info)?;
                fnNode = OperatorOverloading::instOperatorFunctions(fnNode, context, info)?;
                (fnNode.clone(), false)
            },
            Deref @ SCode::Element::CLASS { classDef: cdef @ Deref @ SCode::ClassDef::OVERLOAD { .. }, .. } => {
                let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
                let mut node: metamodelica::Ref<InstNode::InstNode>;
                for mut p in &*var_field!((**cdef).pathLst, SCode::ClassDef::OVERLOAD).clone() {
                    cr = AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&p));
                    (_, node, specialBuiltin) = instFunction(cr, fnNode.clone(), context, info.clone())?;
                    for mut f in &*getCachedFuncs(node)? {
                        fnNode = InstNode::cacheAddFunc(fnNode, f.clone(), specialBuiltin)?;
                    }
                }
                (fnNode.clone(), false)
            },
            Deref @ SCode::Element::CLASS { .. } if (InstNode::isEnumerationType(fnNode.clone())?) => {
                let mut r#fn: metamodelica::Ref<Function>;
                let mut node: metamodelica::Ref<InstNode::InstNode>;
                let mut cmts: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                node = makeEnumConversionOp(fnNode.clone())?;
                node = InstNode::makeRootClass(node, parent, None)?;
                (node, cmts) = instFunction3(node, context, &info)?;
                r#fn = new(fnPath, node, &cmts)?;
                fnNode = InstNode::cacheAddFunc(fnNode, r#fn, false)?;
                (fnNode.clone(), false)
            },
            Deref @ SCode::Element::CLASS { classDef: __def_classDef, .. } => {
                let mut r#fn: metamodelica::Ref<Function>;
                let mut cmts: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                if SCodeUtil::isOperator(&def) {
                    OperatorOverloading::checkOperatorRestrictions(fnNode.clone())?;
                }
                fnNode = InstNode::makeRootClass(fnNode, parent, None)?;
                (fnNode, cmts) = instFunction3(fnNode, context, &info)?;
                r#fn = new(fnPath, fnNode.clone(), &cmts)?;
                specialBuiltin = isSpecialBuiltin(&r#fn);
                assign_field!(r#fn.derivatives = FunctionDerivative::instDerivatives(fnNode.clone(), &r#fn)?);
                assign_field!(r#fn.inverses = FunctionInverse::instInverses(fnNode.clone(), &r#fn)?);
                assign_field!(r#fn.derivedInputs = instPartialDerivedVars(metamodelica::AsArg::as_arg(&__def_classDef), &r#fn.inputs, &r#fn, context, &info)?);
                fnNode = InstNode::cacheAddFunc(fnNode, r#fn, specialBuiltin)?;
                (fnNode.clone(), specialBuiltin)
            },
            _ => return Err("match: no arm matched"),
        } });
        Ok((fnNode, specialBuiltin))
    }

    pub(crate) fn instFunction3(
        mut fnNode: metamodelica::Ref<InstNode::InstNode>,
        mut context: i32,
        mut info: &SourceInfo,
    ) -> Result<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::List<metamodelica::Ref<SCode::Comment>>,
    )> {
        let mut fnNode: metamodelica::Ref<InstNode::InstNode> = fnNode;
        let mut cmts: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
        let mut def: metamodelica::Ref<SCode::Element>;
        let mut numError: i32 = Error::getNumErrorMessages();
        let mut fn_context: i32 = InstContext::set(context, InstContext::FUNCTION.clone());
        if let Ok(__iflet0) = Inst::instantiate(
            fnNode.clone(),
            crate::NFModifier::Modifier::interned_NOMOD(),
            crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
            fn_context,
            true,
        ) {
            fnNode = __iflet0;
        } else {
            let true = (Error::getNumErrorMessages() == numError) else {
                return Err("pattern mismatch");
            };
            def = InstNode::definition(fnNode.clone())?;
            Error::addSourceMessage(
                &(Error::UNKNOWN_ERROR_INST_FUNCTION.clone()),
                list![SCodeDump::unparseElementStr(
                    def.clone(),
                    SCodeDump::defaultOptions.clone()
                )?],
                &(SCodeUtil::elementInfo(&def)),
            )?;
            return Err("fail");
        }
        cmts = InstNode::getComments(&fnNode, metamodelica::nil())?;
        InstNode::cacheInitFunc(fnNode.clone())?;
        Inst::instExpressions(
            fnNode.clone(),
            &(fnNode.clone()),
            crate::NFSections::interned_EMPTY(),
            &(NFConnectBreakTree::new()),
            context,
            &(Inst::InstSettings::create()?),
        )?;
        Ok((fnNode, cmts))
    }

    pub(crate) fn makeEnumConversionOp(
        mut enumNode: metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<metamodelica::Ref<InstNode::InstNode>> {
        let mut fnNode: metamodelica::Ref<InstNode::InstNode>;
        let mut def: metamodelica::Ref<SCode::ClassDef>;
        let mut fn_def: metamodelica::Ref<SCode::ClassDef>;
        let mut elem: metamodelica::Ref<SCode::Element>;
        let mut fn_elem: metamodelica::Ref<SCode::Element>;
        let mut params: metamodelica::List<metamodelica::Ref<SCode::Element>>;
        let mut stmts: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
        let mut info: SourceInfo = InstNode::info(&enumNode);
        let mut enum_name: ArcStr = InstNode::name(&enumNode)?;
        elem = InstNode::definition(InstNode::resolveInner(Class::lastBaseClass(enumNode.clone())?))?;
        fn_def = (::match_deref::match_deref! { match &(elem) {
            Deref @ SCode::Element::CLASS { classDef: __esc_def @ Deref @ SCode::ClassDef::ENUMERATION { .. }, .. } => {
                def = (*__esc_def).clone();
                params = list![metamodelica::Ref::new(SCode::Element::COMPONENT { name: literal!("index"), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultInputAttr.clone(), typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Integer") }), arrayDim: None }), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: info.clone() }), metamodelica::Ref::new(SCode::Element::COMPONENT { name: literal!("value"), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultOutputAttr.clone(), typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: enum_name.clone() }), arrayDim: None }), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: info.clone() })];
                stmts = list![metamodelica::Ref::new(SCode::Statement::ALG_ASSERT { condition: metamodelica::Ref::new(Absyn::Exp::LBINARY { exp1: metamodelica::Ref::new(Absyn::Exp::RELATION { exp1: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("index"), subscripts: metamodelica::nil() }) }), op: openmodelica_ast::Absyn::Operator::GREATEREQ, exp2: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 1 }) }), op: openmodelica_ast::Absyn::Operator::AND, exp2: metamodelica::Ref::new(Absyn::Exp::RELATION { exp1: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("index"), subscripts: metamodelica::nil() }) }), op: openmodelica_ast::Absyn::Operator::LESSEQ, exp2: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: ((var_field!((*def).enumLst, SCode::ClassDef::ENUMERATION)).len() as i32) }) }) }), message: metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("Enumeration index '") }), op: openmodelica_ast::Absyn::Operator::ADD, exp2: metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: metamodelica::Ref::new(Absyn::Exp::CALL { function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("String"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("index"), subscripts: metamodelica::nil() }) })], argNames: metamodelica::nil() }), typeVars: metamodelica::nil() }), op: openmodelica_ast::Absyn::Operator::ADD, exp2: metamodelica::Ref::new(Absyn::Exp::STRING { value: { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("' out of bounds in call to ")); __mm_s.push_str(&*enum_name); __mm_s.push_str(&*literal!("()")); ArcStr::from(__mm_s) } }) }) }), level: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: literal!("AssertionLevel"), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("error"), subscripts: metamodelica::nil() }) }) }), comment: SCode::noComment.clone(), info: info.clone() }), metamodelica::Ref::new(SCode::Statement::ALG_ASSIGN { assignComponent: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("value"), subscripts: metamodelica::nil() }) }), value: metamodelica::Ref::new(Absyn::Exp::SUBSCRIPTED_EXP { exp: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
            for mut e in (var_field!((*def).enumLst, SCode::ClassDef::ENUMERATION).clone()).into_iter().cloned() {
                let __x = metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: enum_name.clone(), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: e.literal.clone(), subscripts: metamodelica::nil() }) }) });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }) }), subscripts: list![metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("index"), subscripts: metamodelica::nil() }) }) })] }), comment: SCode::noComment.clone(), info: info.clone() })];
                metamodelica::Ref::new(SCode::ClassDef::PARTS { elementLst: params, normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: list![metamodelica::Ref::new(SCode::AlgorithmSection { statements: stmts })], initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None })
            },
            _ => return Err("fail"),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        fn_elem = metamodelica::Ref::new(SCode::Element::CLASS {
            name: {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("$"));
                __mm_s.push_str(&*enum_name);
                ArcStr::from(__mm_s)
            },
            prefixes: SCode::defaultPrefixes.clone(),
            encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
            partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
            restriction: SCode::Restriction::R_FUNCTION {
                functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION {
                    purity: openmodelica_ast::Absyn::FunctionPurity::PURE,
                },
            },
            classDef: fn_def,
            cmt: metamodelica::Ref::new(SCode::Comment {
                annotation_: None,
                comment: Some({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Automatically generated conversion operator for "));
                    __mm_s.push_str(&*enum_name);
                    ArcStr::from(__mm_s)
                }),
            }),
            info: info,
        });
        fnNode = InstNode::new(fn_elem, InstNode::parentScope(enumNode, true)?)?;
        Ok(fnNode)
    }

    pub fn getCachedFuncs(
        mut inNode: metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<metamodelica::List<metamodelica::Ref<Function>>> {
        let mut outFuncs: metamodelica::List<metamodelica::Ref<Function>>;
        let mut cache: metamodelica::Ref<CachedData::CachedData>;
        cache = InstNode::getFuncCache(&(InstNode::classScope(inNode)?))?;
        outFuncs = (match &*cache {
            CachedData::FUNCTION {
                funcs: __cache_funcs, ..
            } => __cache_funcs.clone(),
            _ => metamodelica::nil(),
        });
        Ok(outFuncs)
    }

    pub(crate) fn mapCachedFuncs(
        mut inNode: metamodelica::Ref<InstNode::InstNode>,
        mut mapFn: &dyn ::std::ops::Fn(metamodelica::Ref<Function>) -> Result<metamodelica::Ref<Function>>,
    ) -> Result<()> {
        pub type MapFn = std::sync::Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<Function>) -> Result<metamodelica::Ref<Function>> + 'static,
        >;

        let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
        let mut cache: metamodelica::Ref<CachedData::CachedData>;
        cls_node = InstNode::classScope(inNode)?;
        cache = InstNode::getFuncCache(&cls_node)?;
        cache = (match &*cache {
            CachedData::FUNCTION {
                funcs: __cache_funcs, ..
            } => {
                assign_variant_field!(cache => CachedData::CachedData::FUNCTION; funcs = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Function>> = metamodelica::nil();
                    for mut r#fn in (__cache_funcs.clone()).into_iter().cloned() {
                        let __x = mapFn(r#fn.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }));
                cache
            }
            _ => return Err("fail"),
        });
        InstNode::setFuncCache(cls_node, cache)?;
        Ok(())
    }

    pub(crate) fn isEvaluated(mut r#fn: &metamodelica::Ref<Function>) -> bool {
        let mut evaluated: bool;
        evaluated = (match Pointer::access(r#fn.status.clone()) {
            FunctionStatus::BUILTIN => true,
            FunctionStatus::EVALUATED => true,
            _ => false,
        });
        evaluated
    }

    pub(crate) fn markEvaluated(mut r#fn: &metamodelica::Ref<Function>) -> () {
        if Pointer::access(r#fn.status.clone()) != FunctionStatus::BUILTIN.clone() {
            Pointer::update(r#fn.status.clone(), FunctionStatus::EVALUATED.clone());
        }
        ()
    }

    pub(crate) fn isSimplified(mut r#fn: &metamodelica::Ref<Function>) -> bool {
        let mut simplified: bool;
        simplified = (match Pointer::access(r#fn.status.clone()) {
            FunctionStatus::BUILTIN => true,
            FunctionStatus::SIMPLIFIED => true,
            _ => false,
        });
        simplified
    }

    pub(crate) fn markSimplified(mut r#fn: &metamodelica::Ref<Function>) -> () {
        if Pointer::access(r#fn.status.clone()) != FunctionStatus::BUILTIN.clone() {
            Pointer::update(r#fn.status.clone(), FunctionStatus::SIMPLIFIED.clone());
        }
        ()
    }

    pub(crate) fn isCollected(mut r#fn: &metamodelica::Ref<Function>) -> bool {
        let mut collected: bool;
        collected = (match Pointer::access(r#fn.status.clone()) {
            FunctionStatus::BUILTIN => true,
            FunctionStatus::COLLECTED => true,
            _ => false,
        });
        collected
    }

    pub(crate) fn collect(mut r#fn: &metamodelica::Ref<Function>) -> () {
        if Pointer::access(r#fn.status.clone()) != FunctionStatus::BUILTIN.clone() {
            Pointer::update(r#fn.status.clone(), FunctionStatus::COLLECTED.clone());
        }
        ()
    }

    pub fn name(mut r#fn: &metamodelica::Ref<Function>) -> metamodelica::Ref<Absyn::Path> {
        let mut path: metamodelica::Ref<Absyn::Path> = r#fn.path.clone();
        path
    }

    pub(crate) fn setName(
        mut name: metamodelica::Ref<Absyn::Path>,
        mut r#fn: metamodelica::Ref<Function>,
    ) -> metamodelica::Ref<Function> {
        let mut r#fn: metamodelica::Ref<Function> = r#fn;
        assign_field!(r#fn.path = name);
        r#fn
    }

    pub fn nameConsiderBuiltin(mut r#fn: &metamodelica::Ref<Function>) -> metamodelica::Ref<Absyn::Path> {
        let mut path: metamodelica::Ref<Absyn::Path>;
        path = (match r#fn.attributes.isBuiltin.clone() {
            DAE::FunctionBuiltin::FUNCTION_BUILTIN {
                name: Some(mut name), ..
            } => metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }),
            DAE::FunctionBuiltin::FUNCTION_BUILTIN { .. } => AbsynUtil::pathLast(r#fn.path.clone()),
            _ => r#fn.path.clone(),
        });
        path
    }

    pub fn nameEqual(mut fn1: &metamodelica::Ref<Function>, mut fn2: &metamodelica::Ref<Function>) -> bool {
        let mut equal: bool = AbsynUtil::pathEqual(&(name(fn1)), &(name(fn2)));
        equal
    }

    pub fn nameHash(mut r#fn: &metamodelica::Ref<Function>) -> i32 {
        let mut hash: i32 = AbsynUtil::pathHash(&(name(r#fn)));
        hash
    }

    pub fn signatureString(mut r#fn: &metamodelica::Ref<Function>, mut printTypes: bool) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        let mut fn_name: metamodelica::Ref<Absyn::Path>;
        let mut input_str: ArcStr;
        let mut output_str: ArcStr;
        let mut var_s: ArcStr;
        let mut inputs_strl: metamodelica::List<ArcStr> = metamodelica::nil();
        let mut inputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = r#fn.inputs.clone();
        let mut c: metamodelica::Ref<Component::NFComponent>;
        let mut def_exp: metamodelica::Ref<Expression::NFExpression>;
        let mut ty: metamodelica::Ref<Type::NFType>;
        for mut s in &*r#fn.slots.clone() {
            input_str = literal!("");
            c = InstNode::component(&((inputs).head().cloned()?))?;
            inputs = (inputs).rest()?;
            if (s.default).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(s.default.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                def_exp = metamodelica::Own::own(__pa0);
                input_str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(" = "));
                    __mm_s.push_str(&*Expression::toString(def_exp)?);
                    ArcStr::from(__mm_s)
                };
            }
            input_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*Slot::name(metamodelica::AsArg::as_arg(&s))?);
                __mm_s.push_str(&*input_str);
                ArcStr::from(__mm_s)
            };
            input_str = (match s.ty.clone() {
                SlotType::POSITIONAL => {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("$"));
                    __mm_s.push_str(&*input_str);
                    ArcStr::from(__mm_s)
                }
                _ => input_str,
            });
            if printTypes && Component::isTyped(&c) {
                ty = Component::getType(&c)?;
                var_s = Prefixes::unparseVariability(Component::variability(&c)?, ty.clone())?;
                input_str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*var_s);
                    __mm_s.push_str(&*Type::toString(&ty)?);
                    __mm_s.push_str(&*literal!(" "));
                    __mm_s.push_str(&*input_str);
                    ArcStr::from(__mm_s)
                };
            }
            inputs_strl = metamodelica::cons(input_str, inputs_strl);
        }
        input_str = stringDelimitList(inputs_strl.reverse(), literal!(", "));
        output_str = if (printTypes && isTyped(r#fn)) {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" => "));
                __mm_s.push_str(&*Type::toString(&r#fn.returnType)?);
                ArcStr::from(__mm_s)
            }
        } else {
            literal!("")
        };
        fn_name = nameConsiderBuiltin(r#fn);
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*AbsynUtil::pathString(fn_name, literal!("."), true, false)?);
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*input_str);
            __mm_s.push_str(&*literal!(")"));
            __mm_s.push_str(&*output_str);
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn candidateFuncListString(mut fns: metamodelica::List<metamodelica::Ref<Function>>) -> Result<ArcStr> {
        let mut s: ArcStr = stringDelimitList(
            ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut r#fn in (fns.clone()).into_iter().cloned() {
                    let __x = signatureString(&(r#fn.clone()), true)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            literal!("\n  "),
        );
        Ok(s)
    }

    pub(crate) fn callString(
        mut r#fn: &metamodelica::Ref<Function>,
        mut posArgs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        mut namedArgs: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = stringDelimitList(
            ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut arg in (posArgs).into_iter().cloned() {
                    let __x = Expression::toString(arg.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            literal!(", "),
        );
        if !((namedArgs).is_empty()) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut arg in (namedArgs).into_iter().cloned() {
                            let __x = {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*Util::tuple21(arg.clone()));
                                __mm_s.push_str(&*literal!(" = "));
                                __mm_s.push_str(&*Expression::toString(Util::tuple22(arg.clone()))?);
                                ArcStr::from(__mm_s)
                            };
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!(", "),
                ));
                ArcStr::from(__mm_s)
            };
        }
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*AbsynUtil::pathString(r#fn.path.clone(), literal!("."), true, false)?);
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn typeString(mut r#fn: &metamodelica::Ref<Function>) -> Result<ArcStr> {
        fn param_str(mut p: metamodelica::Ref<InstNode::InstNode>) -> Result<ArcStr> {
            let mut s: ArcStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*Type::toString(&(InstNode::getType(p.clone())?))?);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*InstNode::name(&p)?);
                ArcStr::from(__mm_s)
            };
            Ok(s)
        }

        fn param_cell_str(mut p: &metamodelica::Ref<NFInstNode::NodeHandle>) -> Result<ArcStr> {
            let mut s: ArcStr = param_str(InstNode::fromHandle(p)?)?;
            Ok(s)
        }

        let mut r#str: ArcStr;
        let mut inputs: ArcStr;
        let mut outputs: ArcStr;
        inputs = List::toString(r#fn.inputs.clone(), &param_str, List::Style::FLAT.clone())?;
        outputs = List::toString(
            r#fn.outputs.clone(),
            &move |__a0: metamodelica::Ref<NFInstNode::NodeHandle>| param_cell_str(&__a0),
            List::Style::FLAT.clone(),
        )?;
        if (r#fn.outputs).is_empty() || ((r#fn.outputs).len() as i32) > 1 {
            outputs = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*outputs);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
        }
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*AbsynUtil::pathString(name(r#fn), literal!("."), true, false)?);
            __mm_s.push_str(&*literal!("<function>("));
            __mm_s.push_str(&*inputs);
            __mm_s.push_str(&*literal!(") => "));
            __mm_s.push_str(&*outputs);
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn toStream(
        mut r#fn: &metamodelica::Ref<Function>,
        mut indent: ArcStr,
        mut s: IOStream::IOStream,
    ) -> Result<IOStream::IOStream> {
        let mut s: IOStream::IOStream = s;
        let mut fn_name: ArcStr;
        let mut cmt: Option<metamodelica::Ref<SCode::Comment>>;
        if isDefaultRecordConstructor(r#fn)? {
            s = Record::toDeclarationStream(InstNode::fromHandle(&r#fn.node)?, indent, s)?;
        } else if isPartialDerivative(r#fn) {
            fn_name = AbsynUtil::pathString(r#fn.path.clone(), literal!("."), true, false)?;
            cmt = SCodeUtil::getElementComment(&(InstNode::definition(InstNode::fromHandle(&r#fn.node)?)?));
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("function "))?;
            s = IOStream::append(s, fn_name)?;
            s = IOStream::append(s, literal!(" = der("))?;
            s = IOStream::append(
                s,
                AbsynUtil::pathString(getDerivedFunctionName(r#fn)?, literal!("."), true, false)?,
            )?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, stringDelimitList(getDerivedInputNames(r#fn)?, literal!(", ")))?;
            s = IOStream::append(s, DAEDumpTypes::dumpCommentAnnotationStr(cmt))?;
            s = IOStream::append(s, literal!(")"))?;
        } else {
            fn_name = AbsynUtil::pathString(r#fn.path.clone(), literal!("."), true, false)?;
            cmt = SCodeUtil::getElementComment(&(InstNode::definition(InstNode::fromHandle(&r#fn.node)?)?));
            s = IOStream::append(s, indent.clone())?;
            if InstNode::isPartial(&(InstNode::fromHandle(&r#fn.node)?))? {
                s = IOStream::append(s, literal!("partial "))?;
            }
            s = IOStream::append(s, literal!("function "))?;
            s = IOStream::append(s, fn_name.clone())?;
            s = IOStream::append(s, DAEDumpTypes::dumpCommentStr(cmt.clone()))?;
            s = IOStream::append(s, literal!("\n"))?;
            for mut i in &*r#fn.inputs.clone() {
                s = IOStream::append(s, {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("  "));
                    ArcStr::from(__mm_s)
                })?;
                s = IOStream::append(s, InstNode::toString(i.clone())?)?;
                s = IOStream::append(s, literal!(";\n"))?;
            }
            for mut o in &*r#fn.outputs.clone() {
                s = IOStream::append(s, {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("  "));
                    ArcStr::from(__mm_s)
                })?;
                s = IOStream::append(
                    s,
                    InstNode::toString(InstNode::fromHandle(metamodelica::AsArg::as_arg(&o))?)?,
                )?;
                s = IOStream::append(s, literal!(";\n"))?;
            }
            if !((r#fn.locals).is_empty()) {
                s = IOStream::append(s, indent.clone())?;
                s = IOStream::append(s, literal!("protected\n"))?;
                for mut l in &*r#fn.locals.clone() {
                    s = IOStream::append(s, {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*indent);
                        __mm_s.push_str(&*literal!("  "));
                        ArcStr::from(__mm_s)
                    })?;
                    s = IOStream::append(s, InstNode::toString(l.clone())?)?;
                    s = IOStream::append(s, literal!(";\n"))?;
                }
            }
            s = Sections::toStream(
                &(InstNode::getSections(InstNode::fromHandle(&r#fn.node)?)?),
                indent.clone(),
                s,
            )?;
            s = IOStream::append(s, DAEDumpTypes::dumpClassAnnotationStr(cmt))?;
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("end "))?;
            s = IOStream::append(s, fn_name)?;
        }
        Ok(s)
    }

    pub(crate) fn toFlatStream(
        mut r#fn: &metamodelica::Ref<Function>,
        mut format: BaseModelica::OutputFormat,
        mut indent: ArcStr,
        mut s: IOStream::IOStream,
        mut overrideName: ArcStr,
    ) -> Result<IOStream::IOStream> {
        let mut s: IOStream::IOStream = s;
        let mut fn_name: ArcStr;
        let mut cmt: metamodelica::Ref<SCode::Comment>;
        let mut annMod: metamodelica::Ref<SCode::Mod>;
        if isDefaultRecordConstructor(r#fn)? {
            s = Record::toFlatDeclarationStream(InstNode::fromHandle(&r#fn.node)?, format, indent, s)?;
        } else if isPartialDerivative(r#fn) {
            fn_name = if (stringEmpty(&overrideName)) {
                Util::makeQuotedIdentifier(AbsynUtil::pathString(r#fn.path.clone(), literal!("."), true, false)?)?
            } else {
                overrideName
            };
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("function "))?;
            s = IOStream::append(s, fn_name)?;
            s = IOStream::append(s, literal!(" = der("))?;
            s = IOStream::append(
                s,
                Util::makeQuotedIdentifier(AbsynUtil::pathString(
                    getDerivedFunctionName(r#fn)?,
                    literal!("."),
                    true,
                    false,
                )?)?,
            )?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, stringDelimitList(getDerivedInputNames(r#fn)?, literal!(", ")))?;
            s = FlatModelicaUtil::appendCommentOpt(
                SCodeUtil::getElementComment(&(InstNode::definition(InstNode::fromHandle(&r#fn.node)?)?)),
                FlatModelicaUtil::ElementType::FUNCTION.clone(),
                s,
            )?;
            s = IOStream::append(s, literal!(")"))?;
        } else {
            cmt = Util::getOptionOrDefault(
                SCodeUtil::getElementComment(&(InstNode::definition(InstNode::fromHandle(&r#fn.node)?)?)),
                metamodelica::Ref::new(SCode::Comment {
                    annotation_: None,
                    comment: None,
                }),
            );
            fn_name = if (stringEmpty(&overrideName)) {
                Util::makeQuotedIdentifier(AbsynUtil::pathString(r#fn.path.clone(), literal!("."), true, false)?)?
            } else {
                overrideName
            };
            s = IOStream::append(s, indent.clone())?;
            if InstNode::isPartial(&(InstNode::fromHandle(&r#fn.node)?))? {
                s = IOStream::append(s, literal!("partial "))?;
            }
            s = IOStream::append(s, literal!("function "))?;
            s = IOStream::append(s, fn_name.clone())?;
            s = FlatModelicaUtil::appendCommentString(&cmt, s)?;
            s = IOStream::append(s, literal!("\n"))?;
            for mut i in &*r#fn.inputs.clone() {
                s = IOStream::append(
                    s,
                    InstNode::toFlatString(i.clone(), format, {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*indent);
                        __mm_s.push_str(&*literal!("  "));
                        ArcStr::from(__mm_s)
                    })?,
                )?;
                s = IOStream::append(s, literal!(";\n"))?;
            }
            for mut o in &*r#fn.outputs.clone() {
                s = IOStream::append(
                    s,
                    InstNode::toFlatString(InstNode::fromHandle(metamodelica::AsArg::as_arg(&o))?, format, {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*indent);
                        __mm_s.push_str(&*literal!("  "));
                        ArcStr::from(__mm_s)
                    })?,
                )?;
                s = IOStream::append(s, literal!(";\n"))?;
            }
            if !((r#fn.locals).is_empty()) {
                for mut l in &*r#fn.locals.clone() {
                    s = IOStream::append(
                        s,
                        InstNode::toFlatString(l.clone(), format, {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*indent);
                            __mm_s.push_str(&*literal!("  "));
                            ArcStr::from(__mm_s)
                        })?,
                    )?;
                    s = IOStream::append(s, literal!(";\n"))?;
                }
            }
            s = Sections::toFlatStream(
                &(InstNode::getSections(InstNode::fromHandle(&r#fn.node)?)?),
                &r#fn.path,
                format,
                indent.clone(),
                s,
            )?;
            if (cmt.annotation_).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(cmt.annotation_.clone()) {
                    Some(Deref @ SCode::Annotation { modification: __pa0 }) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                annMod = metamodelica::Own::own(__pa0);
            } else {
                annMod = openmodelica_frontend_types::SCode::Mod::interned_NOMOD();
            }
            annMod = SCodeUtil::filterSubMods(
                annMod,
                &({
                    let __pe_b1 = list![literal!("derivative"), literal!("inverse")];
                    move |__pe_a0| Ok(SCodeUtil::removeGivenSubModNames(&__pe_a0, __pe_b1.clone()))
                }),
            )?;
            for mut derivative in &*r#fn.derivatives.clone().reverse() {
                annMod = SCodeUtil::prependSubModToMod(
                    FunctionDerivative::toSubMod(metamodelica::AsArg::as_arg(&derivative))?,
                    annMod,
                )?;
            }
            for mut i in ({
                let __s = metamodelica::arrayLength(r#fn.inverses.clone());
                let __e = 1;
                (0i32..)
                    .map(move |__k| __s + __k * (-1))
                    .take_while(move |&__v| __v >= __e)
            }) {
                annMod = SCodeUtil::prependSubModToMod(
                    FunctionInverse::toSubMod(
                        &({
                            let __elt = (*metamodelica::index_checked(&r#fn.inverses.borrow(), i)?).clone();
                            __elt
                        }),
                    )?,
                    annMod,
                )?;
            }
            if !(SCodeUtil::emptyModOrEquality(&annMod)) {
                cmt = metamodelica::Ref::new(SCode::Comment {
                    annotation_: Some(metamodelica::Ref::new(SCode::Annotation { modification: annMod })),
                    comment: None,
                });
                s = FlatModelicaUtil::appendCommentAnnotation(
                    &cmt,
                    FlatModelicaUtil::ElementType::FUNCTION.clone(),
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*indent);
                        __mm_s.push_str(&*literal!("  "));
                        ArcStr::from(__mm_s)
                    },
                    literal!(";\n"),
                    s,
                )?;
            }
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("end "))?;
            s = IOStream::append(s, fn_name)?;
        }
        Ok(s)
    }

    pub fn toFlatString(
        mut r#fn: &metamodelica::Ref<Function>,
        mut format: BaseModelica::OutputFormat,
        mut indent: ArcStr,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        let mut s: IOStream::IOStream;
        s = IOStream::create(
            literal!("NFFunction.Function.toFlatString"),
            openmodelica_util::IOStream::IOStreamType::LIST,
        )?;
        s = toFlatStream(r#fn, format, indent, s, literal!(""))?;
        r#str = IOStream::string(&s)?;
        IOStream::delete(&s)?;
        Ok(r#str)
    }

    pub(crate) fn instance(mut r#fn: &metamodelica::Ref<Function>) -> Result<metamodelica::Ref<InstNode::InstNode>> {
        let mut node: metamodelica::Ref<InstNode::InstNode> = InstNode::fromHandle(&r#fn.node)?;
        Ok(node)
    }

    pub fn returnType(mut r#fn: &metamodelica::Ref<Function>) -> metamodelica::Ref<Type::NFType> {
        let mut ty: metamodelica::Ref<Type::NFType> = r#fn.returnType.clone();
        ty
    }

    pub(crate) fn setReturnType(
        mut ty: metamodelica::Ref<Type::NFType>,
        mut r#fn: metamodelica::Ref<Function>,
    ) -> metamodelica::Ref<Function> {
        let mut r#fn: metamodelica::Ref<Function> = r#fn;
        assign_field!(r#fn.returnType = ty);
        r#fn
    }

    pub(crate) fn getSlots(
        mut r#fn: &metamodelica::Ref<Function>,
    ) -> metamodelica::List<metamodelica::Ref<Slot::Slot>> {
        let mut slots: metamodelica::List<metamodelica::Ref<Slot::Slot>> = r#fn.slots.clone();
        slots
    }

    pub(crate) fn fillArgs(
        mut posArgs: metamodelica::List<metamodelica::Ref<TypedArg>>,
        mut namedArgs: &metamodelica::List<metamodelica::Ref<TypedArg>>,
        mut r#fn: &metamodelica::Ref<Function>,
        mut context: i32,
        mut info: &SourceInfo,
    ) -> Result<(metamodelica::List<metamodelica::Ref<TypedArg>>, bool)> {
        let mut args: metamodelica::List<metamodelica::Ref<TypedArg>> = posArgs.clone();
        let mut matching: bool;
        let mut slot: metamodelica::Ref<Slot::Slot>;
        let mut slots: metamodelica::List<metamodelica::Ref<Slot::Slot>>;
        let mut slots_arr: metamodelica::Array<metamodelica::Ref<Slot::Slot>>;
        let mut pos_arg_count: i32;
        let mut slot_count: i32;
        let mut index: i32 = 1;
        slots = r#fn.slots.clone();
        pos_arg_count = ((posArgs).len() as i32);
        slot_count = ((slots).len() as i32);
        if pos_arg_count > slot_count {
            matching = false;
            return Ok((args, matching));
        } else if pos_arg_count == slot_count
            && (namedArgs).is_empty()
            && List::all(
                &slots,
                &move |__a0: metamodelica::Ref<Slot::Slot>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Slot::positional(&__a0))
                },
            )?
        {
            matching = true;
            return Ok((args, matching));
        }
        slots_arr = metamodelica::arrayFromVec(slots.into_iter().cloned().collect());
        for mut arg in &*args {
            slot = ({
                let __elt = (*metamodelica::index_checked(&slots_arr.borrow(), index)?).clone();
                __elt
            });
            if !(Slot::positional(&slot)) {
                matching = false;
                return Ok((args.clone(), matching));
            }
            assign_field!(slot.arg = Some(arg.clone()));
            metamodelica::arrayUpdate(slots_arr.clone(), index, slot)?;
            index = index + 1;
        }
        for mut narg in &**namedArgs {
            (slots_arr, matching) = fillNamedArg(narg.clone(), slots_arr.clone(), r#fn, info)?;
            if !(matching) {
                return Ok((args, matching));
            }
        }
        (args, matching) = collectArgs(slots_arr.clone(), context, info);
        Ok((args, matching))
    }

    pub(crate) fn fillNamedArg(
        mut arg: metamodelica::Ref<TypedArg>,
        mut slots: metamodelica::Array<metamodelica::Ref<Slot::Slot>>,
        mut r#fn: &metamodelica::Ref<Function>,
        mut info: &SourceInfo,
    ) -> Result<(metamodelica::Array<metamodelica::Ref<Slot::Slot>>, bool)> {
        let mut slots: metamodelica::Array<metamodelica::Ref<Slot::Slot>> = slots;
        let mut matching: bool = true;
        let mut s: metamodelica::Ref<Slot::Slot> =
            <metamodelica::Ref<Slot::Slot> as ::std::default::Default>::default();
        let mut arg_name: ArcStr;
        let __pa0 = ::match_deref::match_deref! { match &(arg.name.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        arg_name = metamodelica::Own::own(__pa0);
        for mut i in ({
            let __s = metamodelica::arrayLength(slots.clone());
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        }) {
            s = ({
                let __elt = (*metamodelica::index_checked(&slots.borrow(), i)?).clone();
                __elt
            });
            if metamodelica::stringEq(&(Slot::name(&s)?), &arg_name) {
                if !(Slot::named(&s)) {
                    matching = false;
                } else if (s.arg).is_none() {
                    assign_field!(s.arg = Some(arg));
                    {
                        let __cell1 = s;
                        let __idx1 = i;
                        *metamodelica::index_mut_checked(&mut slots.clone().borrow_mut(), __idx1)? = __cell1;
                    }
                } else {
                    Error::addSourceMessage(
                        &(Error::FUNCTION_SLOT_ALREADY_FILLED.clone()),
                        list![arg_name, literal!("")],
                        info,
                    )?;
                    matching = false;
                }
                return Ok((slots.clone(), matching));
            }
        }
        matching = false;
        for mut s in &*r#fn.slots.clone() {
            let mut s = s.clone();
            if metamodelica::stringEq(&arg_name, &(Slot::name(&s)?)) {
                Error::addSourceMessage(
                    &(Error::FUNCTION_SLOT_ALREADY_FILLED.clone()),
                    list![arg_name, literal!("")],
                    info,
                )?;
                return Ok((slots, matching));
            }
            Error::addSourceMessage(
                &(Error::NO_SUCH_INPUT_PARAMETER.clone()),
                list![InstNode::name(&(instance(r#fn)?))?, arg_name.clone()],
                info,
            )?;
        }
        Ok((slots, matching))
    }

    pub(crate) fn collectArgs(
        mut slots: metamodelica::Array<metamodelica::Ref<Slot::Slot>>,
        mut context: i32,
        mut info: &SourceInfo,
    ) -> (metamodelica::List<metamodelica::Ref<TypedArg>>, bool) {
        let mut args: metamodelica::List<metamodelica::Ref<TypedArg>> = metamodelica::nil();
        let mut matching: bool = true;
        let mut default: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut arg: Option<metamodelica::Ref<TypedArg>>;
        let mut a: metamodelica::Ref<TypedArg>;
        let __range0 = slots.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut s in __range0 {
            let __arc3 = s.clone();
            let Slot::SLOT {
                default: __pa1,
                arg: __pa2,
                ..
            } = &*__arc3;
            default = metamodelica::Own::own(__pa1);
            arg = metamodelica::Own::own(__pa2);
            args = 'mc: {
                let __mc_input = arg;
                if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Some(a) => {
                            Ok(metamodelica::cons(a.clone(), args.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    break 'mc __v;
                }
                if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            Ok(metamodelica::cons(fillDefaultSlot(s.clone(), slots.clone(), context, info)?, args.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    break 'mc __v;
                }
                if let Ok((__v, __wb0)) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            let mut matching: bool = matching.clone();
                            matching = false;
                            Ok((args.clone(), matching.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    matching = __wb0;
                    break 'mc __v;
                }
                panic!("matchcontinue: no arm matched")
            };
        }
        args = args.reverse();
        (args, matching)
    }

    pub(crate) fn fillDefaultSlot(
        mut slot: metamodelica::Ref<Slot::Slot>,
        mut slots: metamodelica::Array<metamodelica::Ref<Slot::Slot>>,
        mut context: i32,
        mut info: &SourceInfo,
    ) -> Result<metamodelica::Ref<TypedArg>> {
        let mut outArg: metamodelica::Ref<TypedArg>;
        outArg = (::match_deref::match_deref! { match &(slot.clone()) {
            Deref @ Slot::SLOT { arg: Some(__esc_outArg), .. } => {
                outArg = (*__esc_outArg).clone();
                outArg.clone()
            },
            Deref @ Slot::SLOT { default: Some(_), .. } => fillDefaultSlot2(slot, slots.clone(), context, info)?,
            _ => {
                Error::addSourceMessage(&(Error::UNFILLED_SLOT.clone()), list![Slot::name(&slot)?], info)?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(outArg)
    }

    pub(crate) fn fillDefaultSlot2(
        mut slot: metamodelica::Ref<Slot::Slot>,
        mut slots: metamodelica::Array<metamodelica::Ref<Slot::Slot>>,
        mut context: i32,
        mut info: &SourceInfo,
    ) -> Result<metamodelica::Ref<TypedArg>> {
        let mut outArg: metamodelica::Ref<TypedArg>;
        outArg = (match slot.evalStatus.clone() {
            SlotEvalStatus::EVALUATED => Util::getOption(slot.arg.clone())?,
            SlotEvalStatus::EVALUATING => {
                Error::addSourceMessage(&(Error::CYCLIC_DEFAULT_VALUE.clone()), list![Slot::name(&slot)?], info)?;
                return Err("fail");
            }
            SlotEvalStatus::NOT_EVALUATED => {
                let mut exp: metamodelica::Ref<Expression::NFExpression>;
                let mut ty: metamodelica::Ref<Type::NFType>;
                let mut var: Prefixes::Variability;
                let mut pur: Prefixes::Purity;
                assign_field!(slot.evalStatus = SlotEvalStatus::EVALUATING.clone());
                metamodelica::arrayUpdate(slots.clone(), slot.index.clone(), slot.clone())?;
                exp = evaluateSlotExp(Util::getOption(slot.default.clone())?, slots.clone(), context, info)?;
                (exp, ty, var, pur) = Typing::typeExp(exp, context, info, false)?;
                outArg = metamodelica::Ref::new(TypedArg {
                    name: None,
                    value: exp,
                    ty: ty,
                    var: var,
                    purity: pur,
                });
                assign_field!(
                    slot.arg = Some(outArg.clone()),
                    slot.evalStatus = SlotEvalStatus::EVALUATED.clone()
                );
                metamodelica::arrayUpdate(slots.clone(), slot.index.clone(), slot)?;
                outArg
            }
        });
        Ok(outArg)
    }

    pub(crate) fn evaluateSlotExp(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut slots: metamodelica::Array<metamodelica::Ref<Slot::Slot>>,
        mut context: i32,
        mut info: &SourceInfo,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut outExp: metamodelica::Ref<Expression::NFExpression>;
        outExp = Expression::map(
            exp,
            (std::sync::Arc::new({
                let __pe_b1 = slots.clone();
                let __pe_b2 = context;
                let __pe_b3 = info.clone();
                move |__pe_a0| evaluateSlotExp_traverser(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_b3)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
        Ok(outExp)
    }

    pub(crate) fn evaluateSlotExp_traverser(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut slots: metamodelica::Array<metamodelica::Ref<Slot::Slot>>,
        mut context: i32,
        mut info: &SourceInfo,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut outExp: metamodelica::Ref<Expression::NFExpression>;
        outExp = (match &*exp {
            Expression::CREF { .. } => evaluateSlotCref(exp, slots.clone(), context, info)?,
            _ => exp,
        });
        Ok(outExp)
    }

    pub(crate) fn evaluateSlotCref(
        mut crefExp: metamodelica::Ref<Expression::NFExpression>,
        mut slots: metamodelica::Array<metamodelica::Ref<Slot::Slot>>,
        mut context: i32,
        mut info: &SourceInfo,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut crefExp: metamodelica::Ref<Expression::NFExpression> = crefExp;
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut cref_ty: metamodelica::Ref<Type::NFType>;
        let mut cref_parts: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut slot: Option<metamodelica::Ref<Slot::Slot>>;
        let mut arg: metamodelica::Ref<TypedArg>;
        let mut cref_node: metamodelica::Ref<InstNode::InstNode>;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crefExp.clone()) {
            Deref @ Expression::CREF { cref: __pa0, ty: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cref = metamodelica::Own::own(__pa0);
        cref_ty = metamodelica::Own::own(__pa1);
        if !(ComponentRef::isCref(&cref)) {
            return Ok(crefExp);
        }
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ComponentRef::toListReverse(&cref, true, metamodelica::nil())) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cref = metamodelica::Own::own(__pa2);
        cref_parts = metamodelica::Own::own(__pa3);
        cref_node = ComponentRef::node(&cref)?;
        slot = lookupSlotInArray(cref_node, slots.clone());
        if (slot).is_some() {
            arg = fillDefaultSlot(Util::getOption(slot)?, slots.clone(), context, info)?;
            crefExp = arg.value.clone();
            crefExp = applyCrefSubs(&cref, crefExp)?;
            for mut cr in &*cref_parts {
                crefExp = Expression::recordElement(
                    &(ComponentRef::firstName(metamodelica::AsArg::as_arg(&cr), false)?),
                    &crefExp,
                )?;
                crefExp = applyCrefSubs(&cref, crefExp)?;
            }
            if Type::isKnown(&cref_ty) {
                (crefExp, _, _) = TypeCheck::matchTypes(
                    Expression::typeOf(crefExp.clone()),
                    cref_ty,
                    crefExp,
                    TypeCheck::DEFAULT_OPTIONS.clone(),
                )?;
            }
        }
        Ok(crefExp)
    }

    pub(crate) fn applyCrefSubs(
        mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut exp: metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
        subs = ComponentRef::getSubscripts(cref);
        if (subs).is_empty() {
            return Ok(exp);
        }
        match '__try0: {
            exp = unwrap_break_err!(Expression::applySubscripts(&subs, exp.clone(), false), '__try0);
            Ok::<_, &'static str>((exp.clone(),))
        } {
            Ok((__try0_o0,)) => {
                exp = __try0_o0;
            }
            Err(_) => {
                exp = metamodelica::Ref::new(Expression::NFExpression::SUBSCRIPTED_EXP {
                    exp: exp.clone(),
                    subscripts: subs.clone(),
                    ty: ComponentRef::getSubscriptedType(cref, false)?,
                    split: false,
                });
            }
        }
        Ok(exp)
    }

    pub(crate) fn lookupSlotInArray(
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut slots: metamodelica::Array<metamodelica::Ref<Slot::Slot>>,
    ) -> Option<metamodelica::Ref<Slot::Slot>> {
        let mut outSlot: Option<metamodelica::Ref<Slot::Slot>>;
        let mut slot: metamodelica::Ref<Slot::Slot>;
        match '__try0: {
            (slot, _) = unwrap_break_err!(Array::getMemberOnTrue(node.clone(), slots.clone(), &move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<Slot::Slot>| Slot::hasNode(&__a0, &__a1)), '__try0);
            outSlot = Some(slot.clone());
            Ok::<_, &'static str>((outSlot.clone(),))
        } {
            Ok((__try0_o0,)) => {
                outSlot = __try0_o0;
            }
            Err(_) => {
                outSlot = None;
            }
        }
        outSlot
    }

    pub(crate) fn matchArgs(
        mut func: &metamodelica::Ref<Function>,
        mut args: metamodelica::List<metamodelica::Ref<TypedArg>>,
        mut info: &SourceInfo,
        mut vectorize: bool,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<TypedArg>>,
        metamodelica::Ref<FunctionMatchKind::FunctionMatchKind>,
    )> {
        let mut args: metamodelica::List<metamodelica::Ref<TypedArg>> = args;
        let mut funcMatchKind: metamodelica::Ref<FunctionMatchKind::FunctionMatchKind> = EXACT_MATCH().clone();
        let mut comp: metamodelica::Ref<Component::NFComponent>;
        let mut inputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = func.inputs.clone();
        let mut input_node: metamodelica::Ref<InstNode::InstNode>;
        let mut arg_idx: i32 = 1;
        let mut checked_args: metamodelica::List<metamodelica::Ref<TypedArg>> = metamodelica::nil();
        let mut arg_exp: metamodelica::Ref<Expression::NFExpression>;
        let mut arg_ty: metamodelica::Ref<Type::NFType>;
        let mut input_ty: metamodelica::Ref<Type::NFType>;
        let mut ty: metamodelica::Ref<Type::NFType>;
        let mut arg_var: Prefixes::Variability;
        let mut mk: MatchKind;
        let mut vect_arg: metamodelica::Ref<Expression::NFExpression> =
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 });
        let mut vect_dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
        let mut matched: bool;
        let mut vectorized_args: metamodelica::List<i32> = metamodelica::nil();
        for mut arg in &*args {
            let __arc3 = arg.clone();
            let TypedArg {
                value: __pa0,
                ty: __pa1,
                var: __pa2,
                ..
            } = &*__arc3;
            arg_exp = metamodelica::Own::own(__pa0);
            arg_ty = metamodelica::Own::own(__pa1);
            arg_var = metamodelica::Own::own(__pa2);
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(inputs) {
                Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                _ => return Err("pattern mismatch"),
            } };
            input_node = metamodelica::Own::own(__pa4);
            inputs = metamodelica::Own::own(__pa5);
            comp = InstNode::component(&input_node)?;
            if arg_var > Component::variability(&comp)? {
                Error::addSourceMessage(
                    &(Error::FUNCTION_SLOT_VARIABILITY.clone()),
                    list![
                        InstNode::name(&input_node)?,
                        Expression::toString(arg_exp)?,
                        AbsynUtil::pathString(name(func), literal!("."), true, false)?,
                        Prefixes::variabilityString(arg_var)?,
                        Prefixes::variabilityString(Component::variability(&comp)?)?
                    ],
                    info,
                )?;
                funcMatchKind = NO_MATCH().clone();
                return Ok((args.clone(), funcMatchKind));
            }
            input_ty = Component::getType(&comp)?;
            (arg_exp, ty, mk) = TypeCheck::matchTypes(
                arg_ty.clone(),
                input_ty.clone(),
                arg_exp,
                TypeCheck::ALLOW_UNKNOWN.clone(),
            )?;
            matched = TypeCheck::isValidArgumentMatch(mk);
            if !(matched) && vectorize {
                (arg_exp, ty, vect_arg, vect_dims, mk) =
                    matchArgVectorized(arg_exp, arg_ty.clone(), input_ty.clone(), vect_arg, vect_dims, info)?;
                vectorized_args = metamodelica::cons(arg_idx, vectorized_args);
                matched = TypeCheck::isValidArgumentMatch(mk);
            }
            if !(matched) {
                Error::addSourceMessage(
                    &(Error::ARG_TYPE_MISMATCH.clone()),
                    list![
                        intString(arg_idx),
                        AbsynUtil::pathString(func.path.clone(), literal!("."), true, false)?,
                        InstNode::name(&input_node)?,
                        Expression::toString(arg_exp)?,
                        Type::toString(&arg_ty)?,
                        Type::toString(&input_ty)?
                    ],
                    info,
                )?;
                funcMatchKind = NO_MATCH().clone();
                return Ok((args.clone(), funcMatchKind));
            }
            if TypeCheck::isCastMatch(mk) {
                funcMatchKind = CAST_MATCH().clone();
            } else if TypeCheck::isGenericMatch(mk) {
                funcMatchKind = GENERIC_MATCH().clone();
            }
            checked_args = metamodelica::cons(
                metamodelica::Ref::new(TypedArg {
                    name: arg.name.clone(),
                    value: arg_exp,
                    ty: ty,
                    var: arg_var,
                    purity: arg.purity.clone(),
                }),
                checked_args,
            );
            arg_idx = arg_idx + 1;
        }
        if !((vectorized_args).is_empty()) {
            funcMatchKind = metamodelica::Ref::new(FunctionMatchKind::FunctionMatchKind::VECTORIZED {
                vectDims: vect_dims,
                vectorizedArgs: vectorized_args.reverse(),
                baseMatch: funcMatchKind,
            });
        }
        args = checked_args.reverse();
        Ok((args, funcMatchKind))
    }

    pub(crate) fn matchArgVectorized(
        mut argExp: metamodelica::Ref<Expression::NFExpression>,
        mut argTy: metamodelica::Ref<Type::NFType>,
        mut inputTy: metamodelica::Ref<Type::NFType>,
        mut vectArg: metamodelica::Ref<Expression::NFExpression>,
        mut vectDims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
        mut info: &SourceInfo,
    ) -> Result<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::Ref<Type::NFType>,
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
        MatchKind,
    )> {
        let mut argExp: metamodelica::Ref<Expression::NFExpression> = argExp;
        let mut argTy: metamodelica::Ref<Type::NFType> = argTy;
        let mut vectArg: metamodelica::Ref<Expression::NFExpression> = vectArg;
        let mut vectDims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = vectDims;
        let mut matchKind: MatchKind;
        let mut arg_dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
        let mut input_dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
        let mut vect_dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
        let mut rest_dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
        let mut rest_ty: metamodelica::Ref<Type::NFType>;
        let mut vect_dims_count: i32;
        arg_dims = Type::arrayDims(argTy.clone());
        input_dims = Type::arrayDims(inputTy.clone());
        vect_dims_count = ((arg_dims).len() as i32) - ((input_dims).len() as i32);
        if vect_dims_count < 1 {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            return Ok((argExp, argTy, vectArg, vectDims, matchKind));
        }
        (vect_dims, rest_dims) = List::split(arg_dims, vect_dims_count)?;
        if (vectDims).is_empty() {
            vectDims = fillUnknownVectorizedDims(&vect_dims, argExp.clone());
            vectArg = argExp.clone();
        } else if !(List::isEqualOnTrue(vectDims.clone(), vect_dims.clone(), &move |__a0: metamodelica::Ref<
            Dimension::NFDimension,
        >,
                                                                                    __a1: metamodelica::Ref<
            Dimension::NFDimension,
        >| {
            Dimension::isEqual(&__a0, &__a1)
        })?) {
            Error::addSourceMessage(
                &(Error::VECTORIZE_CALL_DIM_MISMATCH.clone()),
                list![
                    literal!(""),
                    Expression::toString(vectArg.clone())?,
                    literal!(""),
                    Expression::toString(argExp.clone())?,
                    Dimension::toStringList(vectDims.clone(), true)?,
                    Dimension::toStringList(vect_dims, true)?
                ],
                info,
            )?;
        }
        rest_ty = Type::liftArrayLeftList(Type::arrayElementType(&argTy), &rest_dims);
        (argExp, argTy, matchKind) =
            TypeCheck::matchTypes(rest_ty, inputTy, argExp, TypeCheck::DEFAULT_OPTIONS.clone())?;
        Ok((argExp, argTy, vectArg, vectDims, matchKind))
    }

    pub(crate) fn fillUnknownVectorizedDims(
        mut dims: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
        mut argExp: metamodelica::Ref<Expression::NFExpression>,
    ) -> metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> {
        let mut outDims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
        let mut i: i32 = 1;
        for mut dim in &**dims {
            let mut dim = dim.clone();
            if Dimension::isUnknown(&dim) {
                dim = metamodelica::Ref::new(Dimension::NFDimension::EXP {
                    exp: metamodelica::Ref::new(Expression::NFExpression::SIZE {
                        exp: argExp.clone(),
                        dimIndex: Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })),
                    }),
                    var: Variability::CONTINUOUS.clone(),
                });
            }
            outDims = metamodelica::cons(dim, outDims);
            i = i + 1;
        }
        outDims = metamodelica::Dangerous::listReverseInPlace(outDims);
        outDims
    }

    pub(crate) fn matchFunction(
        mut func: &metamodelica::Ref<Function>,
        mut args: metamodelica::List<metamodelica::Ref<TypedArg>>,
        mut named_args: &metamodelica::List<metamodelica::Ref<TypedArg>>,
        mut context: i32,
        mut info: &SourceInfo,
        mut vectorize: bool,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<TypedArg>>,
        metamodelica::Ref<FunctionMatchKind::FunctionMatchKind>,
    )> {
        let mut out_args: metamodelica::List<metamodelica::Ref<TypedArg>>;
        let mut matchKind: metamodelica::Ref<FunctionMatchKind::FunctionMatchKind> = NO_MATCH().clone();
        let mut slot_matched: bool;
        (out_args, slot_matched) = fillArgs(args, named_args, func, context, info)?;
        if slot_matched {
            (out_args, matchKind) = matchArgs(func, out_args, info, vectorize)?;
        }
        Ok((out_args, matchKind))
    }

    pub(crate) fn matchFunctions(
        mut funcs: &metamodelica::List<metamodelica::Ref<Function>>,
        mut args: metamodelica::List<metamodelica::Ref<TypedArg>>,
        mut named_args: &metamodelica::List<metamodelica::Ref<TypedArg>>,
        mut context: i32,
        mut info: &SourceInfo,
        mut vectorize: bool,
    ) -> Result<metamodelica::List<metamodelica::Ref<MatchedFunction::MatchedFunction>>> {
        let mut matchedFunctions: metamodelica::List<metamodelica::Ref<MatchedFunction::MatchedFunction>>;
        let mut m_args: metamodelica::List<metamodelica::Ref<TypedArg>>;
        let mut matchKind: metamodelica::Ref<FunctionMatchKind::FunctionMatchKind>;
        matchedFunctions = metamodelica::nil();
        for mut func in &**funcs {
            (m_args, matchKind) = matchFunction(
                metamodelica::AsArg::as_arg(&func),
                args.clone(),
                named_args,
                context,
                info,
                vectorize,
            )?;
            if FunctionMatchKind::isValid(&matchKind) {
                matchedFunctions = metamodelica::cons(
                    metamodelica::Ref::new(MatchedFunction::MatchedFunction {
                        func: func.clone(),
                        args: m_args,
                        mk: matchKind,
                    }),
                    matchedFunctions,
                );
            }
        }
        Ok(matchedFunctions)
    }

    pub(crate) fn matchFunctionsSilent(
        mut funcs: &metamodelica::List<metamodelica::Ref<Function>>,
        mut args: metamodelica::List<metamodelica::Ref<TypedArg>>,
        mut named_args: &metamodelica::List<metamodelica::Ref<TypedArg>>,
        mut context: i32,
        mut info: &SourceInfo,
        mut vectorize: bool,
    ) -> Result<metamodelica::List<metamodelica::Ref<MatchedFunction::MatchedFunction>>> {
        let mut matchedFunctions: metamodelica::List<metamodelica::Ref<MatchedFunction::MatchedFunction>>;
        ErrorExt::setCheckpoint(literal!("NFFunction:matchFunctions"));
        matchedFunctions = matchFunctions(funcs, args, named_args, context, info, vectorize)?;
        ErrorExt::rollBack(literal!("NFFunction:matchFunctions"));
        Ok(matchedFunctions)
    }

    pub(crate) fn isTyped(mut r#fn: &metamodelica::Ref<Function>) -> bool {
        let mut isTyped: bool;
        isTyped = (match &*r#fn.returnType.clone() {
            Type::UNKNOWN => false,
            _ => true,
        });
        isTyped
    }

    pub(crate) fn typeRefCache(
        mut functionRef: &metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut context: i32,
    ) -> Result<metamodelica::List<metamodelica::Ref<Function>>> {
        let mut functions: metamodelica::List<metamodelica::Ref<Function>>;
        functions = (match &**functionRef {
            ComponentRef::CREF { .. } => typeNodeCache(ComponentRef::node(functionRef)?, context)?,
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFFunction.Function.typeRefCache"));
                        __mm_s.push_str(&*literal!(" got invalid function call reference"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFFunction.mo")),
                )?;
                return Err("fail");
            }
        });
        Ok(functions)
    }

    pub(crate) fn typeNodeCache(
        mut functionNode: metamodelica::Ref<InstNode::InstNode>,
        mut context: i32,
    ) -> Result<metamodelica::List<metamodelica::Ref<Function>>> {
        let mut functions: metamodelica::List<metamodelica::Ref<Function>>;
        let mut fn_node: metamodelica::Ref<InstNode::InstNode>;
        let mut typed: bool;
        let mut special: bool;
        fn_node = InstNode::classScope(functionNode)?;
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(InstNode::getFuncCache(&fn_node)?) {
            Deref @ CachedData::FUNCTION { funcs: __pa0, typed: __pa1, specialBuiltin: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        functions = metamodelica::Own::own(__pa0);
        typed = metamodelica::Own::own(__pa1);
        special = metamodelica::Own::own(__pa2);
        if !(typed) {
            functions = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Function>> = metamodelica::nil();
                for mut f in (functions).into_iter().cloned() {
                    let __x = typeFunctionSignature(f.clone(), context)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            InstNode::setFuncCache(
                fn_node.clone(),
                metamodelica::Ref::new(CachedData::CachedData::FUNCTION {
                    funcs: functions.clone(),
                    typed: true,
                    specialBuiltin: special,
                }),
            )?;
            functions = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Function>> = metamodelica::nil();
                for mut f in (functions).into_iter().cloned() {
                    let __x = typeFunctionBody(f.clone(), context)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            InstNode::setFuncCache(
                fn_node,
                metamodelica::Ref::new(CachedData::CachedData::FUNCTION {
                    funcs: functions.clone(),
                    typed: true,
                    specialBuiltin: special,
                }),
            )?;
        }
        Ok(functions)
    }

    pub(crate) fn getRefCache(
        mut fnRef: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    ) -> Result<metamodelica::List<metamodelica::Ref<Function>>> {
        let mut functions: metamodelica::List<metamodelica::Ref<Function>>;
        let mut fn_node: metamodelica::Ref<InstNode::InstNode>;
        fn_node = InstNode::classScope(ComponentRef::node(fnRef)?)?;
        let __pa0 = ::match_deref::match_deref! { match &(InstNode::getFuncCache(&fn_node)?) {
            Deref @ CachedData::FUNCTION { funcs: __pa0, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        functions = metamodelica::Own::own(__pa0);
        Ok(functions)
    }

    pub(crate) fn typeFunction(
        mut r#fn: metamodelica::Ref<Function>,
        mut context: i32,
    ) -> Result<metamodelica::Ref<Function>> {
        let mut r#fn: metamodelica::Ref<Function> = r#fn;
        r#fn = typeFunctionSignature(r#fn, context)?;
        r#fn = typeFunctionBody(r#fn, context)?;
        Ok(r#fn)
    }

    pub(crate) fn typeFunctionSignature(
        mut r#fn: metamodelica::Ref<Function>,
        mut context: i32,
    ) -> Result<metamodelica::Ref<Function>> {
        let mut r#fn: metamodelica::Ref<Function> = r#fn;
        let mut node: metamodelica::Ref<InstNode::InstNode> = InstNode::fromHandle(&r#fn.node)?;
        let mut fn_context: i32;
        if !(isTyped(&r#fn)) {
            fn_context = InstContext::set(context, InstContext::FUNCTION.clone());
            assign_field!(r#fn.slots = makeSlots(&r#fn.inputs)?);
            Typing::typeClassType(
                node.clone(),
                &(Binding::EMPTY_BINDING().clone()),
                fn_context,
                &(node.clone()),
            )?;
            Typing::typeComponents(node.clone(), fn_context, isPartialDerivative(&r#fn))?;
            if InstNode::isPartial(&node)? {
                ClassTree::applyComponents(&(Class::classTree(InstNode::getClass(node)?)?), &boxFunctionParameter)?;
            }
            checkParamTypes(&r#fn)?;
            checkPartialDerivativeTypes(&r#fn)?;
            assign_field!(r#fn.returnType = makeReturnType(&r#fn)?);
        }
        Ok(r#fn)
    }

    pub(crate) fn typeFunctionBody(
        mut r#fn: metamodelica::Ref<Function>,
        mut context: i32,
    ) -> Result<metamodelica::Ref<Function>> {
        let mut r#fn: metamodelica::Ref<Function> = r#fn;
        let mut pure: bool;
        let mut attr: DAE::FunctionAttributes;
        let mut fn_context: i32;
        fn_context = InstContext::set(context, InstContext::FUNCTION.clone());
        for mut c in &*r#fn.inputs.clone() {
            Typing::typeComponentBinding(c.clone(), fn_context, true)?;
        }
        for mut c in &*r#fn.outputs.clone() {
            Typing::typeComponentBinding(InstNode::fromHandle(metamodelica::AsArg::as_arg(&c))?, fn_context, true)?;
        }
        for mut c in &*r#fn.locals.clone() {
            Typing::typeComponentBinding(c.clone(), fn_context, true)?;
        }
        Typing::typeFunctionSections(InstNode::fromHandle(&r#fn.node)?, fn_context)?;
        for mut fn_der in &*r#fn.derivatives.clone() {
            FunctionDerivative::typeDerivative(metamodelica::AsArg::as_arg(&fn_der))?;
        }
        Array::mapNoCopy(r#fn.inverses.clone(), &FunctionInverse::typeInverse)?;
        if !(isImpure(&r#fn)) {
            pure = foldExp(
                &(r#fn.clone()),
                (std::sync::Arc::new({
                    let __pe_b1 = r#fn.clone();
                    move |__pe_a0, __pe_a2| checkPureCall(__pe_a0, &__pe_b1, __pe_a2)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, bool) -> Result<bool> + 'static,
                    >),
                true,
                true,
                true,
            )?;
            if !(pure) {
                attr = r#fn.attributes.clone();
                attr.purity = DAE::Purity::IMPURE.clone();
                assign_field!(r#fn.attributes = attr);
            }
        }
        if !(InstContext::inRelaxed(fn_context)) {
            checkUseBeforeAssign(&r#fn)?;
        }
        assign_field!(
            r#fn.locals = sortLocals(
                r#fn.locals.clone(),
                &(InstNode::info(&(InstNode::fromHandle(&r#fn.node)?)))
            )?
        );
        Ok(r#fn)
    }

    pub(crate) fn checkPureCall(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut r#fn: &metamodelica::Ref<Function>,
        mut pure: bool,
    ) -> Result<bool> {
        let mut pure: bool = pure;
        if !(pure) {
            return Ok(pure);
        }
        if Expression::isImpureCall(&exp)? {
            pure = false;
            if Config::languageStandardAtLeast(Config::LanguageStandard::_3_3.clone())? {
                Error::addSourceMessage(
                    &(Error::PURE_FUNCTION_WITH_IMPURE_CALLS.clone()),
                    list![
                        AbsynUtil::pathString(name(r#fn), literal!("."), true, false)?,
                        Expression::getName(exp)?
                    ],
                    &(InstNode::info(&(InstNode::fromHandle(&r#fn.node)?))),
                )?;
            }
        }
        Ok(pure)
    }

    pub(crate) fn boxFunctionParameter(mut component: metamodelica::Ref<InstNode::InstNode>) -> Result<()> {
        let mut comp: metamodelica::Ref<Component::NFComponent>;
        comp = InstNode::component(&component)?;
        comp = Component::setType(Type::r#box(&(Component::getType(&comp)?)), comp)?;
        InstNode::updateComponent(comp, component)?;
        Ok(())
    }

    pub(crate) fn typePartialApplication(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut context: i32,
        mut info: &SourceInfo,
    ) -> Result<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::Ref<Type::NFType>,
        Prefixes::Variability,
        Prefixes::Purity,
    )> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let mut ty: metamodelica::Ref<Type::NFType>;
        let mut variability: Prefixes::Variability;
        let mut purity: Prefixes::Purity;
        let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        let mut arg_names: metamodelica::List<ArcStr>;
        let mut arg_name: ArcStr;
        let mut arg_ty: metamodelica::Ref<Type::NFType>;
        let mut arg_var: Prefixes::Variability;
        let mut arg_pur: Prefixes::Purity;
        let mut r#fn: metamodelica::Ref<Function>;
        let mut next_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
        let mut slots: metamodelica::List<metamodelica::Ref<Slot::Slot>>;
        let mut slots_arr: metamodelica::Array<metamodelica::Ref<Slot::Slot>>;
        let mut ty_arg: metamodelica::Ref<TypedArg>;
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp) {
            Deref @ Expression::PARTIAL_FUNCTION_APPLICATION { r#fn: __pa0, args: __pa1, argNames: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        fn_ref = metamodelica::Own::own(__pa0);
        args = metamodelica::Own::own(__pa1);
        arg_names = metamodelica::Own::own(__pa2);
        let __pa3 = ::match_deref::match_deref! { match &(typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
            Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: _ } => __pa3.clone(),
            _ => return Err("pattern mismatch"),
        } };
        r#fn = metamodelica::Own::own(__pa3);
        slots_arr = metamodelica::arrayFromVec(r#fn.slots.clone().into_iter().cloned().collect());
        purity = if (isImpure(&r#fn)) {
            Purity::IMPURE.clone()
        } else {
            Purity::PURE.clone()
        };
        variability = Variability::CONSTANT.clone();
        for mut arg in &*args {
            let mut arg = arg.clone();
            (arg, arg_ty, arg_var, arg_pur) = Typing::typeExp(arg, next_context, info, false)?;
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(arg_names) {
                Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                _ => return Err("pattern mismatch"),
            } };
            arg_name = metamodelica::Own::own(__pa4);
            arg_names = metamodelica::Own::own(__pa5);
            ty_arg = metamodelica::Ref::new(TypedArg {
                name: Some(arg_name),
                value: arg,
                ty: arg_ty,
                var: arg_var,
                purity: arg_pur,
            });
            let (__pa6, true) = (fillNamedArg(ty_arg, slots_arr.clone(), &r#fn, info)?) else {
                return Err("pattern mismatch");
            };
            slots_arr = metamodelica::Own::own(__pa6);
            variability = Prefixes::variabilityMax(variability, arg_var);
            purity = Prefixes::purityMin(purity, arg_pur);
        }
        exp = makePartialApplicationFromSlots(slots_arr.clone(), r#fn, fn_ref, info)?;
        ty = Expression::typeOf(exp.clone());
        Ok((exp, ty, variability, purity))
    }

    pub(crate) fn makePartialApplicationFromSlots(
        mut slotsArray: metamodelica::Array<metamodelica::Ref<Slot::Slot>>,
        mut r#fn: metamodelica::Ref<Function>,
        mut fnRef: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut info: &SourceInfo,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut outExp: metamodelica::Ref<Expression::NFExpression>;
        let mut inputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
        let mut slots: metamodelica::List<metamodelica::Ref<Slot::Slot>> = metamodelica::nil();
        let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        let mut arg_names: metamodelica::List<ArcStr> = metamodelica::nil();
        let mut ty_arg: metamodelica::Ref<TypedArg>;
        let mut arg: metamodelica::Ref<Expression::NFExpression>;
        let mut mk: MatchKind;
        let mut fn_ty: metamodelica::Ref<Type::NFType>;
        let __range0 = slotsArray.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut slot in __range0 {
            if (slot.arg).is_some() {
                let __pa1 = ::match_deref::match_deref! { match &(slot.arg.clone()) {
                    Some(__pa1) => __pa1.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                ty_arg = metamodelica::Own::own(__pa1);
                (arg, _, mk) = TypeCheck::matchTypes(
                    ty_arg.ty.clone(),
                    InstNode::getType(slot.node.clone())?,
                    ty_arg.value.clone(),
                    TypeCheck::ALLOW_UNKNOWN.clone(),
                )?;
                if TypeCheck::isIncompatibleMatch(mk) {
                    Error::addSourceMessage(
                        &(Error::NAMED_ARG_TYPE_MISMATCH.clone()),
                        list![
                            AbsynUtil::pathString(name(&r#fn), literal!("."), true, false)?,
                            Util::getOption(ty_arg.name.clone())?,
                            Expression::toString(ty_arg.value.clone())?,
                            Type::toString(&ty_arg.ty)?,
                            Type::toString(&(InstNode::getType(slot.node.clone())?))?
                        ],
                        info,
                    )?;
                    return Err("fail");
                }
                args = metamodelica::cons(Expression::r#box(&arg), args);
                arg_names = metamodelica::cons(Util::getOption(ty_arg.name.clone())?, arg_names);
            } else {
                inputs = metamodelica::cons(slot.node.clone(), inputs);
                slots = metamodelica::cons(slot, slots);
            }
        }
        assign_field!(
            r#fn.inputs = metamodelica::Dangerous::listReverseInPlace(inputs),
            r#fn.slots = metamodelica::Dangerous::listReverseInPlace(slots)
        );
        fn_ty = metamodelica::Ref::new(Type::NFType::FUNCTION {
            r#fn: r#fn,
            fnType: Type::FunctionType::FUNCTIONAL_VARIABLE.clone(),
        });
        args = metamodelica::Dangerous::listReverseInPlace(args);
        arg_names = metamodelica::Dangerous::listReverseInPlace(arg_names);
        outExp = metamodelica::Ref::new(Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION {
            r#fn: fnRef,
            args: args,
            argNames: arg_names,
            ty: fn_ty,
        });
        Ok(outExp)
    }

    pub fn isBuiltin(mut r#fn: &metamodelica::Ref<Function>) -> bool {
        let mut isBuiltin: bool = isBuiltinAttr(&r#fn.attributes);
        isBuiltin
    }

    pub(crate) fn isBuiltinAttr(mut attrs: &DAE::FunctionAttributes) -> bool {
        let mut isBuiltin: bool;
        isBuiltin = (match attrs.isBuiltin.clone() {
            DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN { .. } => false,
            _ => true,
        });
        isBuiltin
    }

    pub fn isSpecialBuiltin(mut r#fn: &metamodelica::Ref<Function>) -> bool {
        let mut special: bool;
        let mut path: metamodelica::Ref<Absyn::Path>;
        if !(isBuiltin(r#fn)) {
            special = false;
        } else {
            path = nameConsiderBuiltin(r#fn);
            if !(AbsynUtil::pathIsIdent(&path)) {
                special = false;
            } else {
                special = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(&path)) {
                    Deref @ "array" => true,
                    Deref @ "actualStream" => true,
                    Deref @ "backSample" => true,
                    Deref @ "branch" => true,
                    Deref @ "cardinality" => true,
                    Deref @ "cat" => true,
                    Deref @ "change" => true,
                    Deref @ "der" => true,
                    Deref @ "edge" => true,
                    Deref @ "fill" => true,
                    Deref @ "getInstanceName" => true,
                    Deref @ "initial" => true,
                    Deref @ "inStream" => true,
                    Deref @ "isRoot" => true,
                    Deref @ "matrix" => true,
                    Deref @ "max" => true,
                    Deref @ "min" => true,
                    Deref @ "ndims" => true,
                    Deref @ "noEvent" => true,
                    Deref @ "nthRoot" => true,
                    Deref @ "ones" => true,
                    Deref @ "potentialRoot" => true,
                    Deref @ "pre" => true,
                    Deref @ "promote" => true,
                    Deref @ "pure" => true,
                    Deref @ "root" => true,
                    Deref @ "rooted" => true,
                    Deref @ "uniqueRoot" => true,
                    Deref @ "uniqueRootIndices" => true,
                    Deref @ "scalar" => true,
                    Deref @ "size" => true,
                    Deref @ "shiftSample" => true,
                    Deref @ "smooth" => true,
                    Deref @ "spatialDistribution" => true,
                    Deref @ "subSample" => true,
                    Deref @ "superSample" => true,
                    Deref @ "symmetric" => true,
                    Deref @ "terminal" => true,
                    Deref @ "transpose" => true,
                    Deref @ "vector" => true,
                    Deref @ "zeros" => true,
                    Deref @ "sample" => true,
                    _ => false,
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
        }
        special
    }

    pub(crate) fn isSubscriptableBuiltin(mut r#fn: &metamodelica::Ref<Function>) -> bool {
        let mut scalarBuiltin: bool;
        if !(isBuiltin(r#fn)) {
            scalarBuiltin = false;
        } else {
            scalarBuiltin = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(&(nameConsiderBuiltin(r#fn)))) {
                Deref @ "change" => true,
                Deref @ "der" => true,
                Deref @ "pre" => true,
                _ => false,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        scalarBuiltin
    }

    pub fn isImpure(mut r#fn: &metamodelica::Ref<Function>) -> bool {
        let mut isImpure: bool = r#fn.attributes.purity.clone() == DAE::Purity::IMPURE.clone();
        isImpure
    }

    pub(crate) fn isFunctionPointer(mut r#fn: &metamodelica::Ref<Function>) -> bool {
        let mut isPointer: bool = r#fn.attributes.isFunctionPointer.clone();
        isPointer
    }

    pub(crate) fn setFunctionPointer(
        mut isPointer: bool,
        mut r#fn: metamodelica::Ref<Function>,
    ) -> metamodelica::Ref<Function> {
        let mut r#fn: metamodelica::Ref<Function> = r#fn;
        let mut attr: DAE::FunctionAttributes = r#fn.attributes.clone();
        attr.isFunctionPointer = isPointer;
        assign_field!(r#fn.attributes = attr);
        r#fn
    }

    pub fn isExternal(mut r#fn: &metamodelica::Ref<Function>) -> Result<bool> {
        let mut isExternal: bool = !(InstNode::isEmpty(&(InstNode::fromHandle(&r#fn.node)?)))
            && Class::isExternalFunction(InstNode::getClass(InstNode::fromHandle(&r#fn.node)?)?)?;
        Ok(isExternal)
    }

    pub(crate) fn isExternalObjectConstructorOrDestructor(mut r#fn: &metamodelica::Ref<Function>) -> Result<bool> {
        let mut isExternal: bool;
        let mut path: metamodelica::Ref<Absyn::Path>;
        let mut lastIdent: ArcStr;
        path = name(r#fn);
        lastIdent = AbsynUtil::pathLastIdent(&path);
        isExternal = false;
        if metamodelica::stringEq(&lastIdent, &(literal!("constructor"))) {
            isExternal = Type::isExternalObject(&r#fn.returnType);
        } else if metamodelica::stringEq(&lastIdent, &(literal!("destructor"))) {
            if ((r#fn.inputs).len() as i32) == 1 {
                isExternal = Type::isExternalObject(
                    &(Component::getType(&(InstNode::component(&((r#fn.inputs).head().cloned()?))?))?),
                );
            }
        }
        Ok(isExternal)
    }

    pub(crate) fn isPartialDerivative(mut r#fn: &metamodelica::Ref<Function>) -> bool {
        let mut res: bool = !((r#fn.derivedInputs).is_empty());
        res
    }

    pub(crate) fn getDerivedInputNames(mut r#fn: &metamodelica::Ref<Function>) -> Result<metamodelica::List<ArcStr>> {
        let mut names: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut i in &*r#fn.derivedInputs.clone() {
            names = metamodelica::cons(InstNode::name(&((r#fn.inputs).get(i.clone())?))?, names);
        }
        names = metamodelica::Dangerous::listReverseInPlace(names);
        Ok(names)
    }

    pub(crate) fn getDerivedFunctionName(
        mut r#fn: &metamodelica::Ref<Function>,
    ) -> Result<metamodelica::Ref<Absyn::Path>> {
        let mut name: metamodelica::Ref<Absyn::Path> =
            InstNode::fullPath(Class::lastBaseClass(InstNode::fromHandle(&r#fn.node)?)?, true)?;
        Ok(name)
    }

    pub fn inlineBuiltin(mut r#fn: &metamodelica::Ref<Function>) -> DAE::InlineType {
        let mut inlineType: DAE::InlineType;
        inlineType = (match r#fn.attributes.isBuiltin.clone() {
            DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR { .. } => {
                openmodelica_frontend_types::DAE::InlineType::BUILTIN_EARLY_INLINE
            }
            _ => r#fn.attributes.inline.clone(),
        });
        inlineType
    }

    pub fn isDefaultRecordConstructor(mut r#fn: &metamodelica::Ref<Function>) -> Result<bool> {
        let mut isConstructor: bool =
            Restriction::isRecordConstructor(&(InstNode::restriction(InstNode::fromHandle(&r#fn.node)?)?));
        Ok(isConstructor)
    }

    pub fn isNonDefaultRecordConstructor(mut r#fn: &metamodelica::Ref<Function>) -> bool {
        pub(crate) fn isNonDefaultRecordConstructorPath<'__b>(mut path: &'__b metamodelica::Ref<Absyn::Path>) -> bool {
            '__tco: loop {
                ::match_deref::match_deref! { match path {
                    Deref @ Absyn::Path::QUALIFIED { name: Deref @ "'constructor'", .. } => return true,
                    Deref @ Absyn::Path::QUALIFIED { .. } => { path = var_field!((**path).path, Absyn::Path::QUALIFIED); continue '__tco; },
                    _ => return false,
                    _ => unreachable!("tail-call lowered match: no arm matched"),
                } }
            }
        }

        let mut b: bool = isNonDefaultRecordConstructorPath(&r#fn.path);
        b
    }

    pub(crate) fn toDAE(
        mut r#fn: &metamodelica::Ref<Function>,
        mut def: DAE::FunctionDefinition,
    ) -> Result<DAE::Function> {
        let mut daeFn: DAE::Function;
        let mut vis: SCode::Visibility;
        let mut par: bool;
        let mut impr: bool;
        let mut ity: DAE::InlineType;
        let mut ty: metamodelica::Ref<DAE::Type>;
        let mut defs: metamodelica::List<DAE::FunctionDefinition>;
        let mut unused_inputs: metamodelica::List<i32>;
        vis = openmodelica_frontend_types::SCode::Visibility::PUBLIC;
        par = false;
        impr = r#fn.attributes.purity.clone() == DAE::Purity::IMPURE.clone();
        ity = r#fn.attributes.inline.clone();
        ty = makeDAEType(r#fn, false)?;
        unused_inputs = analyseUnusedParameters(r#fn)?;
        defs = ({
            let mut __acc: metamodelica::List<DAE::FunctionDefinition> = metamodelica::nil();
            for mut fn_inv in (r#fn.inverses.clone()).borrow().iter() {
                let __x = FunctionInverse::toDAE(&(fn_inv.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        defs = listAppend(
            ({
                let mut __acc: metamodelica::List<DAE::FunctionDefinition> = metamodelica::nil();
                for mut fn_der in (r#fn.derivatives.clone()).into_iter().cloned() {
                    let __x = FunctionDerivative::toDAE(&(fn_der.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            defs,
        );
        defs = metamodelica::cons(def, defs);
        daeFn = DAE::Function::FUNCTION {
            path: r#fn.path.clone(),
            functions: defs,
            type_: ty,
            visibility: vis,
            partialPrefix: par,
            isImpure: impr,
            inlineType: ity,
            unusedInputs: unused_inputs,
            source: ElementSource::createElementSource(
                InstNode::info(&(InstNode::fromHandle(&r#fn.node)?)),
                None,
                &(openmodelica_frontend_types::DAE::Prefix::NOPRE),
                (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
            ),
            comment: SCodeUtil::getElementComment(&(InstNode::definition(InstNode::fromHandle(&r#fn.node)?)?)),
        };
        Ok(daeFn)
    }

    pub(crate) fn makeDAEType(
        mut r#fn: &metamodelica::Ref<Function>,
        mut boxTypes: bool,
    ) -> Result<metamodelica::Ref<DAE::Type>> {
        let mut outType: metamodelica::Ref<DAE::Type>;
        let mut params: metamodelica::List<metamodelica::Ref<DAE::FuncArg>> = metamodelica::nil();
        let mut pname: ArcStr;
        let mut ty: metamodelica::Ref<Type::NFType>;
        let mut ptype: metamodelica::Ref<DAE::Type>;
        let mut pconst: DAE::Const;
        let mut ppar: DAE::VarParallelism;
        let mut pdefault: Option<metamodelica::Ref<DAE::Exp>>;
        let mut comp: metamodelica::Ref<Component::NFComponent>;
        for mut param in &*r#fn.inputs.clone() {
            comp = InstNode::component(metamodelica::AsArg::as_arg(&param))?;
            pname = InstNode::name(metamodelica::AsArg::as_arg(&param))?;
            ty = Component::getType(&comp)?;
            ptype = Type::toDAE(&(if (boxTypes) { Type::r#box(&ty) } else { ty }), true)?;
            pconst = Prefixes::variabilityToDAEConst(Component::variability(&comp)?);
            ppar = Prefixes::parallelismToDAE(Component::parallelism(&comp))?;
            pdefault = Util::applyOption(
                Binding::typedExp(&(Component::getBinding(&comp))),
                &({
                    let __pe_b1 = false;
                    move |__pe_a0| Expression::toDAE(__pe_a0, __pe_b1.clone())
                }),
            )?;
            params = metamodelica::cons(
                metamodelica::Ref::new(DAE::FuncArg {
                    name: pname,
                    ty: ptype,
                    r#const: pconst,
                    par: ppar,
                    defaultBinding: pdefault,
                }),
                params,
            );
        }
        params = params.reverse();
        ty = if (isDefaultRecordConstructor(r#fn)?) {
            InstNode::getType(InstNode::fromHandle(&r#fn.node)?)?
        } else {
            r#fn.returnType.clone()
        };
        ty = if (boxTypes) { Type::r#box(&ty) } else { ty };
        outType = metamodelica::Ref::new(DAE::Type::T_FUNCTION {
            funcArg: params,
            funcResultType: Type::toDAE(&ty, true)?,
            functionAttributes: r#fn.attributes.clone(),
            path: r#fn.path.clone(),
        });
        Ok(outType)
    }

    pub fn getSingleBodyExp(
        mut r#fn: &metamodelica::Ref<Function>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression>;
        let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
        body = getBody(r#fn)?;
        exp = (::match_deref::match_deref! { match &(body.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: stmt @ Deref @ Statement::ASSIGNMENT { .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
                var_field!((**stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone()
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.getSingleBodyExp")); __mm_s.push_str(&*literal!(" failed because the body of the function is not a single assignment:\n")); __mm_s.push_str(&*List::toStringCustom(body, &({ let __pe_b1 = literal!("\t"); move |__pe_a0| Statement::toString(&__pe_a0, __pe_b1.clone()) }), literal!(""), literal!(""), literal!("\n"), literal!(""), true, 0)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(exp)
    }

    pub fn getBody(
        mut r#fn: &metamodelica::Ref<Function>,
    ) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
        let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> =
            getBody2(InstNode::fromHandle(&r#fn.node)?)?;
        Ok(body)
    }

    pub(crate) fn hasUnboxArgs(mut r#fn: &metamodelica::Ref<Function>) -> bool {
        let mut res: bool;
        res = (match r#fn.attributes.clone() {
            DAE::FunctionAttributes {
                isBuiltin:
                    DAE::FunctionBuiltin::FUNCTION_BUILTIN {
                        unboxArgs: mut __esc_res,
                        ..
                    },
                ..
            } => {
                res = __esc_res.clone();
                res
            }
            _ => false,
        });
        res
    }

    pub(crate) fn hasUnboxArgsAnnotation(mut cmt: &metamodelica::Ref<SCode::Comment>) -> bool {
        let mut res: bool =
            SCodeUtil::commentHasBooleanNamedAnnotation(cmt, &(literal!("__OpenModelica_UnboxArguments")));
        res
    }

    pub(crate) fn hasOptionalArgument(mut component: &metamodelica::Ref<SCode::Element>) -> bool {
        let mut res: bool =
            SCodeUtil::hasBooleanNamedAnnotationInComponent(component, &(literal!("__OpenModelica_optionalArgument")));
        res
    }

    pub fn mapExp(
        mut r#fn: metamodelica::Ref<Function>,
        mut mapFn: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
        mut mapFnFields: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
        mut mapParameters: bool,
        mut mapBody: bool,
    ) -> Result<metamodelica::Ref<Function>> {
        pub type MapFunc = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >;

        let mut r#fn: metamodelica::Ref<Function> = r#fn;
        let mut cls: metamodelica::Ref<Class::NFClass>;
        let mut ctree: metamodelica::Ref<ClassTree::ClassTree>;
        let mut sections: metamodelica::Ref<Sections::NFSections>;
        cls = InstNode::getClass(InstNode::fromHandle(&r#fn.node)?)?;
        if mapParameters {
            ctree = Class::classTree(cls.clone())?;
            ClassTree::applyComponents(
                &ctree,
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    > = mapFn.clone();
                    let __pe_b2: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    > = mapFnFields.clone();
                    move |__pe_a0| mapExpParameter(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                }),
            )?;
            assign_field!(r#fn.returnType = makeReturnType(&r#fn)?);
        }
        if mapBody {
            sections = Sections::mapExp(Class::getSections(cls.clone())?, &*mapFn)?;
            cls = Class::setSections(sections, cls)?;
            InstNode::updateClass(cls, InstNode::fromHandle(&r#fn.node)?)?;
        }
        Ok(r#fn)
    }

    pub(crate) fn mapExpParameter(
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut mapFn: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
        mut mapFnFields: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
    ) -> Result<()> {
        pub type MapFunc = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >;

        let mut comp: metamodelica::Ref<Component::NFComponent>;
        let mut binding: metamodelica::Ref<Binding::NFBinding>;
        let mut binding2: metamodelica::Ref<Binding::NFBinding>;
        let mut cls: metamodelica::Ref<Class::NFClass>;
        let mut ty: metamodelica::Ref<Type::NFType>;
        let mut dirty: bool = false;
        comp = InstNode::component(&node)?;
        binding = Component::getBinding(&comp);
        binding2 = Binding::mapExpShallow(binding.clone(), &*mapFn)?;
        if !(referenceEq(&*(binding), &*(&*binding2))) {
            comp = Component::setBinding(binding2, comp)?;
            dirty = true;
        }
        let () = (match &*comp {
            Component::COMPONENT { ty: __comp_ty, .. } => {
                ty = Type::mapDims(
                    __comp_ty.clone(),
                    &({
                        let __pe_b1: Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        > = mapFn.clone();
                        move |__pe_a0| Dimension::mapExp(__pe_a0, __pe_b1.clone())
                    }),
                )?;
                if !(referenceEq(
                    &*(&*ty),
                    &*(var_field!((*comp).ty, Component::NFComponent::COMPONENT).clone()),
                )) {
                    assign_variant_field!(comp => Component::NFComponent::COMPONENT; ty = ty);
                    dirty = true;
                }
                cls = InstNode::getClass(var_field!((*comp).classInst, Component::NFComponent::COMPONENT).clone())?;
                ClassTree::applyComponents(
                    &(Class::classTree(cls)?),
                    &({
                        let __pe_b1: Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        > = mapFnFields.clone();
                        let __pe_b2: Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        > = mapFnFields.clone();
                        move |__pe_a0| mapExpParameter(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?;
                ()
            }
            _ => (),
        });
        if dirty {
            InstNode::updateComponent(comp, node)?;
        }
        Ok(())
    }

    pub(crate) fn mapBody(
        mut r#fn: metamodelica::Ref<Function>,
        mut mapFn: &dyn ::std::ops::Fn(
            metamodelica::Ref<Algorithm::NFAlgorithm>,
        ) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    ) -> Result<metamodelica::Ref<Function>> {
        pub type MapFn = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Algorithm::NFAlgorithm>,
                ) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>>
                + 'static,
        >;

        let mut r#fn: metamodelica::Ref<Function> = r#fn;
        let mut cls: metamodelica::Ref<Class::NFClass>;
        let mut sections: metamodelica::Ref<Sections::NFSections>;
        cls = InstNode::getClass(InstNode::fromHandle(&r#fn.node)?)?;
        sections = Sections::map(
            Class::getSections(cls.clone())?,
            &fnptr!(Sections::eqId, metamodelica::Ref<NFEquation::NFEquation>),
            mapFn,
            &fnptr!(Sections::eqId, metamodelica::Ref<NFEquation::NFEquation>),
            mapFn,
        )?;
        cls = Class::setSections(sections, cls)?;
        InstNode::updateClass(cls, InstNode::fromHandle(&r#fn.node)?)?;
        Ok(r#fn)
    }

    pub(crate) fn foldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut r#fn: &metamodelica::Ref<Function>,
        mut foldFn: Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static,
        >,
        mut arg: ArgT,
        mut mapParameters: bool,
        mut mapBody: bool,
    ) -> Result<ArgT> {
        pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static,
        >;

        let mut arg: ArgT = arg;
        let mut cls: metamodelica::Ref<Class::NFClass>;
        cls = InstNode::getClass(InstNode::fromHandle(&r#fn.node)?)?;
        if mapParameters {
            arg = ClassTree::foldComponents(
                &(Class::classTree(cls.clone())?),
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, _) -> Result<_> + 'static,
                    > = foldFn.clone();
                    move |__pe_a0, __pe_a2| foldExpParameter(&__pe_a0, __pe_b1.clone(), __pe_a2)
                }),
                arg,
            )?;
        }
        if mapBody {
            arg = Sections::foldExp(&(Class::getSections(cls)?), &*foldFn, arg)?;
        }
        Ok(arg)
    }

    pub(crate) fn foldExpParameter<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut node: &metamodelica::Ref<InstNode::InstNode>,
        mut foldFn: Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static,
        >,
        mut arg: ArgT,
    ) -> Result<ArgT> {
        pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static,
        >;

        let mut arg: ArgT = arg;
        let mut comp: metamodelica::Ref<Component::NFComponent>;
        let mut cls: metamodelica::Ref<Class::NFClass>;
        comp = InstNode::component(node)?;
        arg = Binding::foldExp(&(Component::getBinding(&comp)), foldFn.clone(), arg)?;
        let () = (match &*comp {
            Component::COMPONENT {
                classInst: __comp_classInst,
                ty: __comp_ty,
                ..
            } => {
                arg = Type::foldDims(
                    __comp_ty.clone(),
                    (std::sync::Arc::new({
                        let __pe_b1: Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, _) -> Result<_> + 'static,
                        > = foldFn.clone();
                        move |__pe_a0, __pe_a2| Dimension::foldExp(&__pe_a0, __pe_b1.clone(), __pe_a2)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<Dimension::NFDimension>, _) -> Result<_> + 'static,
                        >),
                    arg,
                )?;
                cls = InstNode::getClass(__comp_classInst.clone())?;
                arg = ClassTree::foldComponents(
                    &(Class::classTree(cls)?),
                    &({
                        let __pe_b1: Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, _) -> Result<_> + 'static,
                        > = foldFn.clone();
                        move |__pe_a0, __pe_a2| foldExpParameter(&__pe_a0, __pe_b1.clone(), __pe_a2)
                    }),
                    arg,
                )?;
                ()
            }
            _ => (),
        });
        Ok(arg)
    }

    pub(crate) fn isPartial(mut r#fn: &metamodelica::Ref<Function>) -> Result<bool> {
        let mut isPartial: bool = InstNode::isPartial(&(InstNode::fromHandle(&r#fn.node)?))?;
        Ok(isPartial)
    }

    pub(crate) fn getLocalArguments(
        mut r#fn: &metamodelica::Ref<Function>,
    ) -> Result<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> {
        let mut localArgs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        let mut binding: metamodelica::Ref<Binding::NFBinding>;
        for mut l in &*r#fn.locals.clone() {
            if InstNode::isComponent(metamodelica::AsArg::as_arg(&l))? {
                binding = Component::getBinding(&(InstNode::component(metamodelica::AsArg::as_arg(&l))?));
                Error::assertion(
                    Binding::hasExp(&binding),
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFFunction.Function.getLocalArguments"));
                        __mm_s.push_str(&*literal!(" got local component without binding"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFFunction.mo")),
                )?;
                localArgs = metamodelica::cons(Binding::getExp(&binding)?, localArgs);
            }
        }
        localArgs = metamodelica::Dangerous::listReverseInPlace(localArgs);
        Ok(localArgs)
    }

    fn collectParams(
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut inputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        mut outputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        mut locals: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    )> {
        let mut inputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = inputs;
        let mut outputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = outputs;
        let mut locals: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = locals;
        let mut cls: metamodelica::Ref<Class::NFClass>;
        let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut n: metamodelica::Ref<InstNode::InstNode>;
        let mut check_vis: bool;
        Error::assertion(
            InstNode::isClass(&node)? || InstNode::isComponent(&node)?,
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFFunction.Function.collectParams"));
                __mm_s.push_str(&*literal!(" got non-class/non-component node"));
                ArcStr::from(__mm_s)
            },
            &(metamodelica::sourceInfo!("NFFrontEnd/NFFunction.mo")),
        )?;
        cls = InstNode::getClass(node.clone())?;
        let () = (::match_deref::match_deref! { match &(cls) {
            Deref @ Class::INSTANCED_CLASS { elements: Deref @ ClassTree::FLAT_TREE { components: __esc_comps, .. }, .. } => {
                comps = (*__esc_comps).clone();
                for mut i in ({let __s=metamodelica::arrayLength(comps.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
                    n = ({let __elt = (*metamodelica::index_checked(&comps.borrow(), i)?).clone(); __elt});
                    check_vis = !(Flags::getConfigBool(Flags::BASE_MODELICA.clone())?);
                    let () = (match paramDirection(n.clone(), check_vis)? {
            Prefixes::Direction::INPUT => {
                inputs = metamodelica::cons(n, inputs);
                ()
            },
            Prefixes::Direction::OUTPUT => {
                outputs = metamodelica::cons(n, outputs);
                ()
            },
            Prefixes::Direction::NONE => {
                locals = metamodelica::cons(n, locals);
                ()
            },
        });
                }
                ()
            },
            Deref @ Class::EXPANDED_DERIVED { baseClass: __cls_baseClass, .. } => {
                (inputs, outputs, locals) = collectParams(__cls_baseClass.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
                ()
            },
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.collectParams")); __mm_s.push_str(&*literal!(" got non-instantiated function ")); __mm_s.push_str(&*AbsynUtil::pathString(InstNode::scopePath(node, InstNode::ScopeType::RELATIVE.clone(), false)?, literal!("."), true, false)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFFunction.mo")))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((inputs, outputs, locals))
    }

    fn paramDirection(
        mut component: metamodelica::Ref<InstNode::InstNode>,
        mut checkVisibility: bool,
    ) -> Result<Prefixes::Direction> {
        let mut direction: Prefixes::Direction;
        let mut comp: metamodelica::Ref<Component::NFComponent>;
        let mut cty: i32;
        let mut io: Prefixes::InnerOuter;
        let mut vis: Prefixes::Visibility;
        let mut var: Prefixes::Variability;
        comp = InstNode::component(&(InstNode::resolveOuter(component.clone())))?;
        io = Component::innerOuter(&comp)?;
        if io != InnerOuter::NOT_INNER_OUTER.clone() {
            Error::addSourceMessage(
                &(Error::INNER_OUTER_FORMAL_PARAMETER.clone()),
                list![Prefixes::innerOuterString(io), InstNode::name(&component)?],
                &(InstNode::info(&(InstNode::resolveOuter(component.clone())))),
            )?;
            return Err("fail");
        }
        let __arc3 = Component::getAttributes(&comp);
        let Attributes::ATTRIBUTES {
            connectorType: __pa0,
            direction: __pa1,
            variability: __pa2,
            ..
        } = &*__arc3;
        cty = metamodelica::Own::own(__pa0);
        direction = metamodelica::Own::own(__pa1);
        var = metamodelica::Own::own(__pa2);
        if Prefixes::ConnectorType::isFlowOrStream(cty) {
            Error::addSourceMessage(
                &(Error::INNER_OUTER_FORMAL_PARAMETER.clone()),
                list![Prefixes::ConnectorType::toString(cty), InstNode::name(&component)?],
                &(InstNode::info(&component)),
            )?;
            return Err("fail");
        }
        if checkVisibility {
            vis = InstNode::visibility(&component);
            if direction != Direction::NONE.clone() {
                if vis == Visibility::PROTECTED.clone() {
                    Error::addSourceMessage(
                        &(Error::PROTECTED_FORMAL_FUNCTION_VAR.clone()),
                        list![InstNode::name(&component)?],
                        &(InstNode::info(&component)),
                    )?;
                    return Err("fail");
                }
            } else if vis == Visibility::PUBLIC.clone() {
                Error::addSourceMessageAsError(
                    Error::NON_FORMAL_PUBLIC_FUNCTION_VAR.clone(),
                    list![InstNode::name(&component)?],
                    &(InstNode::info(&component)),
                )?;
                return Err("fail");
            }
        }
        Ok(direction)
    }

    fn makeSlots(
        mut inputs: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    ) -> Result<metamodelica::List<metamodelica::Ref<Slot::Slot>>> {
        let mut slots: metamodelica::List<metamodelica::Ref<Slot::Slot>> = metamodelica::nil();
        let mut index: i32 = 1;
        for mut i in &**inputs {
            slots = metamodelica::cons(makeSlot(i.clone(), index)?, slots);
            index = index + 1;
        }
        slots = metamodelica::Dangerous::listReverseInPlace(slots);
        Ok(slots)
    }

    fn makeSlot(
        mut component: metamodelica::Ref<InstNode::InstNode>,
        mut index: i32,
    ) -> Result<metamodelica::Ref<Slot::Slot>> {
        let mut slot: metamodelica::Ref<Slot::Slot> =
            <metamodelica::Ref<Slot::Slot> as ::std::default::Default>::default();
        let mut comp: metamodelica::Ref<Component::NFComponent>;
        let mut default: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut name: ArcStr;
        if '__try0: {
            comp = unwrap_break_err!(InstNode::component(&component), '__try0);
            default = Binding::getExpOpt(&(unwrap_break_err!(Component::getImplicitBinding(&comp, unwrap_break_err!(InstNode::instanceParent(component.clone()), '__try0)), '__try0)));
            name = unwrap_break_err!(InstNode::name(&component), '__try0);
            if StringUtil::startsWith(name.clone(), literal!("$in_")) {
                name = unwrap_break_err!(substring(name.clone(), 5, ((name).len() as i32)), '__try0);
            }
            slot = metamodelica::Ref::new(Slot::Slot { node: component.clone(), ty: SlotType::GENERIC.clone(), default: default.clone(), arg: None, index: index, evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() });
            Ok::<(), &'static str>(())
        }.is_err() {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.makeSlot")); __mm_s.push_str(&*literal!(" got invalid component")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFFunction.mo")))?;
        }
        Ok(slot)
    }

    fn hasOMPure(mut cmt: &metamodelica::Ref<SCode::Comment>) -> bool {
        let mut res: bool = !(SCodeUtil::commentHasBooleanNamedAnnotation(cmt, &(literal!("__OpenModelica_Impure"))));
        res
    }

    fn getBuiltinPtr(mut cmt: &metamodelica::Ref<SCode::Comment>) -> DAE::FunctionBuiltin {
        let mut builtin: DAE::FunctionBuiltin =
            if (SCodeUtil::commentHasBooleanNamedAnnotation(cmt, &(literal!("__OpenModelica_BuiltinPtr")))) {
                openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR
            } else {
                openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN
            };
        builtin
    }

    fn mergeFunctionAnnotations(
        mut comments: &metamodelica::List<metamodelica::Ref<SCode::Comment>>,
    ) -> metamodelica::Ref<SCode::Comment> {
        let mut outComment: metamodelica::Ref<SCode::Comment>;
        let mut comment: Option<ArcStr> = None;
        let mut r#mod: metamodelica::Ref<SCode::Mod> = openmodelica_frontend_types::SCode::Mod::interned_NOMOD();
        let mut mod2: metamodelica::Ref<SCode::Mod>;
        for mut cmt in &**comments {
            if (comment).is_none() {
                comment = cmt.comment.clone();
            }
            r#mod = (::match_deref::match_deref! { match &(cmt.clone()) {
                Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: __esc_mod2 }), .. } => {
                    mod2 = (*__esc_mod2).clone();
                    SCodeUtil::mergeModifiers(mod2.clone(), r#mod)
                },
                _ => r#mod,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        outComment = (match &*r#mod {
            SCode::Mod::NOMOD { .. } => metamodelica::Ref::new(SCode::Comment {
                annotation_: None,
                comment: comment,
            }),
            _ => metamodelica::Ref::new(SCode::Comment {
                annotation_: Some(metamodelica::Ref::new(SCode::Annotation { modification: r#mod })),
                comment: comment,
            }),
        });
        outComment
    }

    fn makeAttributes(
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut inputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        mut outputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        mut comments: &metamodelica::List<metamodelica::Ref<SCode::Comment>>,
    ) -> Result<DAE::FunctionAttributes> {
        let mut attr: DAE::FunctionAttributes;
        let mut def: metamodelica::Ref<SCode::Element>;
        let mut res: SCode::Restriction;
        let mut fres: SCode::FunctionRestriction;
        let mut is_partial: bool;
        let mut cmt: metamodelica::Ref<SCode::Comment>;
        let mut purity: DAE::Purity;
        def = InstNode::classDefinition(Class::lastBaseClass(node.clone())?)?;
        res = SCodeUtil::getClassRestriction(&def)?;
        Error::assertion(
            SCodeUtil::isFunctionRestriction(&res),
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFFunction.Function.makeAttributes"));
                __mm_s.push_str(&*literal!(" got non-function restriction"));
                ArcStr::from(__mm_s)
            },
            &(metamodelica::sourceInfo!("NFFrontEnd/NFFunction.mo")),
        )?;
        let SCode::Restriction::R_FUNCTION {
            functionRestriction: __pa0,
        } = (res)
        else {
            return Err("pattern mismatch");
        };
        fres = metamodelica::Own::own(__pa0);
        is_partial = InstNode::isPartial(&node)?;
        cmt = mergeFunctionAnnotations(comments);
        purity = InstBasics::getFunctionRestrictionPurity(SCodeUtil::getFunctionRestrictionPurity(fres), &cmt, true);
        attr = 'mc: {
            let __mc_input = fres;
            if let Ok(__v) = (|| -> Result<_> {
                let SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } = __mc_input.clone() else {
                    return Err("nomatch");
                };
                let mut has_unbox_args: bool;
                let mut name: ArcStr;
                let mut in_params: metamodelica::List<ArcStr>;
                let mut out_params: metamodelica::List<ArcStr>;
                let mut inline_ty: DAE::InlineType;
                let mut generateEvents: bool;
                in_params = ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut i in (inputs.clone()).into_iter().cloned() {
                        let __x = InstNode::name(&(i.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                out_params = ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut o in (outputs.clone()).into_iter().cloned() {
                        let __x = InstNode::name(&(o.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                name = SCodeUtil::isBuiltinFunction(&def, in_params.clone(), &out_params)?;
                inline_ty = InstBasics::commentIsInlineFunc(&cmt);
                generateEvents = InstBasics::commentGenerateEvents(&cmt);
                has_unbox_args = hasUnboxArgsAnnotation(&cmt);
                Ok(DAE::FunctionAttributes {
                    inline: inline_ty,
                    generateEvents: generateEvents,
                    purity: purity,
                    isFunctionPointer: is_partial,
                    isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN {
                        name: Some(name.clone()),
                        unboxArgs: has_unbox_args,
                    },
                    functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_NON_PARALLEL,
                    noReturn: DAE::NoReturn::RETURNS.clone(),
                })
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                let SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. } = __mc_input.clone() else {
                    return Err("nomatch");
                };
                let mut has_unbox_args: bool;
                let mut name: ArcStr;
                let mut in_params: metamodelica::List<ArcStr>;
                let mut out_params: metamodelica::List<ArcStr>;
                let mut inline_ty: DAE::InlineType;
                let mut generateEvents: bool;
                in_params = ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut i in (inputs.clone()).into_iter().cloned() {
                        let __x = InstNode::name(&(i.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                out_params = ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut o in (outputs.clone()).into_iter().cloned() {
                        let __x = InstNode::name(&(o.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                name = SCodeUtil::isBuiltinFunction(&def, in_params.clone(), &out_params)?;
                inline_ty = InstBasics::commentIsInlineFunc(&cmt);
                generateEvents = InstBasics::commentGenerateEvents(&cmt);
                has_unbox_args = hasUnboxArgsAnnotation(&cmt);
                Ok(DAE::FunctionAttributes {
                    inline: inline_ty,
                    generateEvents: generateEvents,
                    purity: purity,
                    isFunctionPointer: is_partial,
                    isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN {
                        name: Some(name.clone()),
                        unboxArgs: has_unbox_args,
                    },
                    functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_PARALLEL_FUNCTION,
                    noReturn: DAE::NoReturn::RETURNS.clone(),
                })
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                let SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. } = __mc_input.clone() else {
                    return Err("nomatch");
                };
                let mut inline_ty: DAE::InlineType;
                let mut generateEvents: bool;
                inline_ty = InstBasics::commentIsInlineFunc(&cmt);
                generateEvents = InstBasics::commentGenerateEvents(&cmt);
                Ok(DAE::FunctionAttributes {
                    inline: inline_ty,
                    generateEvents: generateEvents,
                    purity: purity,
                    isFunctionPointer: is_partial,
                    isBuiltin: getBuiltinPtr(&cmt),
                    functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_PARALLEL_FUNCTION,
                    noReturn: DAE::NoReturn::RETURNS.clone(),
                })
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                let SCode::FunctionRestriction::FR_KERNEL_FUNCTION { .. } = __mc_input.clone() else {
                    return Err("nomatch");
                };
                Ok(DAE::FunctionAttributes {
                    inline: openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
                    generateEvents: false,
                    purity: purity,
                    isFunctionPointer: is_partial,
                    isBuiltin: openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN,
                    functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_KERNEL_FUNCTION,
                    noReturn: DAE::NoReturn::RETURNS.clone(),
                })
            })() {
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                let _ = __mc_input.clone() else { return Err("nomatch") };
                let mut inline_ty: DAE::InlineType;
                let mut generateEvents: bool;
                let mut purity: DAE::Purity = purity.clone();
                inline_ty = InstBasics::commentIsInlineFunc(&cmt);
                generateEvents = InstBasics::commentGenerateEvents(&cmt);
                if purity == DAE::Purity::UNDEFINED.clone()
                    && Config::languageStandardAtLeast(Config::LanguageStandard::_3_3.clone())?
                {
                    purity = if (SCodeUtil::isExternalFunctionRestriction(fres)) {
                        DAE::Purity::IMPURE.clone()
                    } else {
                        DAE::Purity::PURE.clone()
                    };
                }
                if SCodeUtil::hasNamedExternalCall(&(literal!("ModelicaError")), &(SCodeUtil::getClassDef(&def)?)) {
                    purity = DAE::Purity::PURE.clone();
                }
                Ok((
                    DAE::FunctionAttributes {
                        inline: inline_ty,
                        generateEvents: generateEvents,
                        purity: purity,
                        isFunctionPointer: is_partial,
                        isBuiltin: getBuiltinPtr(&cmt),
                        functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_NON_PARALLEL,
                        noReturn: DAE::NoReturn::RETURNS.clone(),
                    },
                    purity.clone(),
                ))
            })() {
                purity = __wb0;
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        };
        Ok(attr)
    }

    fn checkParamTypes(mut r#fn: &metamodelica::Ref<Function>) -> Result<()> {
        checkParamTypes2(&r#fn.inputs)?;
        checkParamTypes2(
            &({
                let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
                for mut o in (r#fn.outputs.clone()).into_iter().cloned() {
                    let __x = InstNode::fromHandle(&(o.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?;
        checkParamTypes2(&r#fn.locals)?;
        Ok(())
    }

    fn checkParamTypes2(mut params: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>) -> Result<()> {
        let mut ty: metamodelica::Ref<Type::NFType>;
        for mut p in &**params {
            ty = InstNode::getType(p.clone())?;
            if !(isValidParamType(&ty)?) {
                Error::addSourceMessage(
                    &(Error::INVALID_FUNCTION_VAR_TYPE.clone()),
                    list![Type::toString(&ty)?, InstNode::name(metamodelica::AsArg::as_arg(&p))?],
                    &(InstNode::info(metamodelica::AsArg::as_arg(&p))),
                )?;
                return Err("fail");
            }
        }
        Ok(())
    }

    fn isValidParamType<'__b>(mut ty: &'__b metamodelica::Ref<Type::NFType>) -> Result<bool> {
        '__tco: loop {
            match &**ty {
                Type::INTEGER => return Ok(true),
                Type::REAL => return Ok(true),
                Type::STRING => return Ok(true),
                Type::BOOLEAN => return Ok(true),
                Type::CLOCK => return Ok(true),
                Type::ENUMERATION { .. } => return Ok(true),
                Type::POLYMORPHIC { .. } => return Ok(true),
                Type::ARRAY { .. } => {
                    ty = var_field!((**ty).elementType, Type::NFType::ARRAY);
                    continue '__tco;
                }
                Type::COMPLEX { .. } => return Ok(isValidParamState(Type::complexNode(ty)?)?),
                Type::FUNCTION { .. } => return Ok(true),
                Type::METABOXED { .. } => {
                    ty = var_field!((**ty).ty, Type::NFType::METABOXED);
                    continue '__tco;
                }
                _ => return Ok(false),
            }
        }
    }

    fn isValidParamState(mut cls: metamodelica::Ref<InstNode::InstNode>) -> Result<bool> {
        let mut isValid: bool;
        isValid = (match &*(Class::restriction(&(InstNode::getClass(cls)?))) {
            Restriction::RECORD { .. } => true,
            Restriction::TYPE => true,
            Restriction::OPERATOR => true,
            Restriction::FUNCTION => true,
            Restriction::EXTERNAL_OBJECT => true,
            _ => false,
        });
        Ok(isValid)
    }

    fn checkPartialDerivativeTypes(mut r#fn: &metamodelica::Ref<Function>) -> Result<()> {
        let mut node: metamodelica::Ref<InstNode::InstNode>;
        let mut ty: metamodelica::Ref<Type::NFType>;
        for mut i in &*r#fn.derivedInputs.clone() {
            node = (r#fn.inputs).get(i.clone())?;
            ty = InstNode::getType(node.clone())?;
            if !(Type::isReal(&ty)? && Type::isScalar(&ty)) {
                Error::addSourceMessage(
                    &(Error::PARTIAL_DERIVATIVE_INPUT_INVALID_TYPE.clone()),
                    list![
                        InstNode::name(&node)?,
                        AbsynUtil::pathString(getDerivedFunctionName(r#fn)?, literal!("."), true, false)?
                    ],
                    &(InstNode::info(&(InstNode::fromHandle(&r#fn.node)?))),
                )?;
                return Err("fail");
            }
        }
        Ok(())
    }

    pub(crate) fn makeReturnType(mut r#fn: &metamodelica::Ref<Function>) -> Result<metamodelica::Ref<Type::NFType>> {
        let mut returnType: metamodelica::Ref<Type::NFType>;
        let mut ret_tyl: metamodelica::List<metamodelica::Ref<Type::NFType>>;
        ret_tyl = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
            for mut o in (r#fn.outputs.clone()).into_iter().cloned() {
                let __x = InstNode::getType(InstNode::fromHandle(&(o.clone()))?)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        returnType = (::match_deref::match_deref! { match &(ret_tyl.clone()) {
            Deref @ metamodelica::ListNode::Nil => crate::NFType::interned_NORETCALL(),
            Deref @ metamodelica::ListNode::Cons { head: __esc_returnType, tail: Deref @ metamodelica::ListNode::Nil } => {
                returnType = (*__esc_returnType).clone();
                returnType.clone()
            },
            _ => metamodelica::Ref::new(Type::NFType::TUPLE { types: ret_tyl, names: None }),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(returnType)
    }

    pub(crate) fn getBody2(
        mut node: metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
        let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
        let mut fn_body: metamodelica::Ref<Algorithm::NFAlgorithm>;
        body = (::match_deref::match_deref! { match &(InstNode::getSections(node)?) {
            Deref @ Sections::SECTIONS { algorithms: Deref @ metamodelica::ListNode::Nil, .. } => metamodelica::nil(),
            Deref @ Sections::SECTIONS { algorithms: Deref @ metamodelica::ListNode::Cons { head: __esc_fn_body, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                fn_body = (*__esc_fn_body).clone();
                fn_body.statements.clone()
            },
            Deref @ Sections::EMPTY => metamodelica::nil(),
            Deref @ Sections::EXTERNAL { .. } => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.getBody2")); __mm_s.push_str(&*literal!(" got function with external section (not algorithm section)")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFFunction.mo")))?;
                return Err("fail")
            },
            Deref @ Sections::SECTIONS { .. } => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.getBody2")); __mm_s.push_str(&*literal!(" got function with multiple algorithm sections")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFFunction.mo")))?;
                return Err("fail")
            },
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.getBody2")); __mm_s.push_str(&*literal!(" got unknown sections")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFFunction.mo")))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(body)
    }

    pub fn hasSingleOrEmptyBody(mut r#fn: &metamodelica::Ref<Function>) -> bool {
        let mut b: bool = false;
        let mut algorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
        if '__try0: {
            if isBuiltin(r#fn) {
                return b;
            }
            b = (match &*(unwrap_break_err!(InstNode::getSections(unwrap_break_err!(InstNode::fromHandle(&r#fn.node), '__try0)), '__try0)) {
        Sections::SECTIONS { algorithms: __esc_algorithms, .. } => {
            algorithms = (*__esc_algorithms).clone();
            ((algorithms).len() as i32) < 2
        },
        Sections::EMPTY => true,
        _ => false,
    });
            Ok::<(), &'static str>(())
        }.is_err() {
        }
        b
    }

    pub(crate) fn analyseUnusedParameters(mut r#fn: &metamodelica::Ref<Function>) -> Result<metamodelica::List<i32>> {
        let mut unusedInputs: metamodelica::List<i32> = metamodelica::nil();
        let mut inputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
        let mut index: i32;
        inputs = foldExp(
            r#fn,
            (std::sync::Arc::new(analyseUnusedParametersExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
                        )
                            -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>>
                        + 'static,
                >),
            r#fn.inputs.clone(),
            true,
            true,
        )?;
        for mut i in &*inputs {
            index = List::positionOnTrue(
                &r#fn.inputs,
                &({
                    let __pe_b0 = i.clone();
                    move |__pe_a1| InstNode::refEqual(&__pe_b0, &__pe_a1)
                }),
            )?;
            unusedInputs = metamodelica::cons(index, unusedInputs);
        }
        Ok(unusedInputs)
    }

    pub(crate) fn analyseUnusedParametersExp(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut params: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    ) -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>> {
        let mut params: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = params;
        if !((params).is_empty()) {
            params = Expression::fold(
                exp,
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Expression::NFExpression>,
                          __a1: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>| {
                        analyseUnusedParametersExp2(&__a0, __a1)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                                metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
                            )
                                -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>>
                            + 'static,
                    >),
                params,
            )?;
        }
        Ok(params)
    }

    pub(crate) fn analyseUnusedParametersExp2(
        mut exp: &metamodelica::Ref<Expression::NFExpression>,
        mut params: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    ) -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>> {
        let mut params: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = params;
        let () = (match &**exp {
            Expression::CREF { cref: __exp_cref, .. } => {
                (params, _) = List::deleteMemberOnTrue(__exp_cref.clone(), params, &move |__a0: metamodelica::Ref<
                    ComponentRef::NFComponentRef,
                >,
                                                                                          __a1: metamodelica::Ref<
                    InstNode::InstNode,
                >| {
                    ComponentRef::containsNode(&__a0, &__a1)
                })?;
                ()
            }
            _ => (),
        });
        Ok(params)
    }

    pub(crate) fn sortLocals(
        mut locals: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        mut info: &SourceInfo,
    ) -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>> {
        let mut locals: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = locals;
        let mut locals_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>;
        let mut dep_graph: metamodelica::List<(
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        )>;
        let mut cycles: metamodelica::List<(
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        )>;
        let mut cycles_str: ArcStr;
        locals_set = UnorderedSet::fromList(
            &locals,
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<InstNode::InstNode>| InstNode::hash(&__a0))
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<i32> + 'static>),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<InstNode::InstNode>| {
                    InstNode::refEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<InstNode::InstNode>,
                            metamodelica::Ref<InstNode::InstNode>,
                        ) -> Result<bool>
                        + 'static,
                >),
        )?;
        dep_graph = Graph::buildGraph(locals, &getLocalDependencies, locals_set)?;
        (locals, cycles) = Graph::topologicalSort(
            &dep_graph,
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<InstNode::InstNode>| {
                    InstNode::refEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<InstNode::InstNode>,
                            metamodelica::Ref<InstNode::InstNode>,
                        ) -> Result<bool>
                        + 'static,
                >),
        )?;
        if !((cycles).is_empty()) {
            cycles_str = stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut cycle in (Graph::findCycles(&cycles, &move |__a0: metamodelica::Ref<
                        InstNode::InstNode,
                    >,
                                                                        __a1: metamodelica::Ref<
                        InstNode::InstNode,
                    >| {
                        InstNode::refEqual(&__a0, &__a1)
                    })?)
                    .into_iter()
                    .cloned()
                    {
                        let __x = List::toString(
                            cycle.clone(),
                            &move |__a0: metamodelica::Ref<InstNode::InstNode>| InstNode::name(&__a0),
                            List::Style::FLAT_CURLY.clone(),
                        )?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!(", "),
            );
            Error::addSourceMessage(&(Error::CYCLIC_FUNCTION_COMPONENTS.clone()), list![cycles_str], info)?;
            return Err("fail");
        }
        Ok(locals)
    }

    pub(crate) fn getLocalDependencies(
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut locals: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
    ) -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>> {
        let mut dependencies: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
        let mut deps: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>;
        deps = UnorderedSet::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<InstNode::InstNode>| InstNode::hash(&__a0))
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<i32> + 'static>),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<InstNode::InstNode>| {
                    InstNode::refEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<InstNode::InstNode>,
                            metamodelica::Ref<InstNode::InstNode>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        );
        deps = getLocalDependencies2(node.clone(), locals.clone(), deps)?;
        UnorderedSet::remove(node.clone(), deps.clone())?;
        deps = Type::foldDims(
            InstNode::getType(node)?,
            (std::sync::Arc::new({
                let __pe_b1 = locals;
                move |__pe_a0, __pe_a2| getLocalDependenciesDim(&__pe_a0, __pe_b1.clone(), __pe_a2)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Dimension::NFDimension>,
                            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
                        ) -> Result<
                            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
                        > + 'static,
                >),
            deps,
        )?;
        dependencies = UnorderedSet::toList(deps);
        Ok(dependencies)
    }

    pub(crate) fn getLocalDependencies2(
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut locals: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
        mut dependencies: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
    ) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>> {
        let mut dependencies: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>> =
            dependencies;
        let mut comp: metamodelica::Ref<Component::NFComponent>;
        let mut binding: metamodelica::Ref<Binding::NFBinding>;
        comp = InstNode::component(&node)?;
        binding = Component::getBinding(&comp);
        if Binding::hasExp(&binding) {
            dependencies = getLocalDependenciesExp(Binding::getExp(&binding)?, locals, dependencies)?;
        } else if Type::isRecord(&(Component::getType(&comp)?)) {
            dependencies = ClassTree::foldComponents(
                &(Class::classTree(InstNode::getClass(node)?)?),
                &({
                    let __pe_b1 = locals;
                    move |__pe_a0, __pe_a2| getLocalDependencies2(__pe_a0, __pe_b1.clone(), __pe_a2)
                }),
                dependencies,
            )?;
        }
        Ok(dependencies)
    }

    pub(crate) fn getLocalDependenciesExp(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut locals: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
        mut deps: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
    ) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>> {
        let mut deps: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>> = deps;
        deps = Expression::fold(
            exp,
            (std::sync::Arc::new({
                let __pe_b1 = locals;
                move |__pe_a0, __pe_a2| getLocalDependenciesExp2(&__pe_a0, __pe_b1.clone(), __pe_a2)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
                        ) -> Result<
                            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
                        > + 'static,
                >),
            deps,
        )?;
        Ok(deps)
    }

    pub(crate) fn getLocalDependenciesExp2(
        mut exp: &metamodelica::Ref<Expression::NFExpression>,
        mut locals: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
        mut deps: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
    ) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>> {
        let mut deps: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>> = deps;
        let () = (match &**exp {
            Expression::CREF { cref: __exp_cref, .. } => {
                let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut cr_node: metamodelica::Ref<InstNode::InstNode>;
                cr = ComponentRef::last(metamodelica::AsArg::as_arg(&__exp_cref));
                if ComponentRef::isCref(&cr) {
                    cr_node = ComponentRef::node(&cr)?;
                    if UnorderedSet::contains(cr_node.clone(), locals)? {
                        UnorderedSet::add(cr_node, deps.clone())?;
                    }
                }
                ()
            }
            _ => (),
        });
        Ok(deps)
    }

    pub(crate) fn getLocalDependenciesDim(
        mut dim: &metamodelica::Ref<Dimension::NFDimension>,
        mut locals: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
        mut deps: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
    ) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>> {
        let mut deps: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>> = deps;
        deps = Dimension::foldExp(
            dim,
            (std::sync::Arc::new({
                let __pe_b1 = locals;
                move |__pe_a0, __pe_a2| getLocalDependenciesExp(__pe_a0, __pe_b1.clone(), __pe_a2)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
                        ) -> Result<
                            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
                        > + 'static,
                >),
            deps,
        )?;
        Ok(deps)
    }

    pub fn getDerivative(
        mut original: &metamodelica::Ref<Function>,
        mut interface_map: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, bool>>,
    ) -> Result<Option<metamodelica::Ref<Function>>> {
        let mut derivative: Option<metamodelica::Ref<Function>> = None;
        let mut derivatives: metamodelica::List<metamodelica::Ref<FunctionDerivative::NFFunctionDerivative>>;
        for mut func in &*original.derivatives.clone() {
            if FunctionDerivative::perfectFit(metamodelica::AsArg::as_arg(&func), interface_map.clone())? {
                derivative = Some(
                    (getCachedFuncs(InstNode::borrow(func.derivativeFn.clone())?)?)
                        .head()
                        .cloned()?,
                );
                return Ok(derivative);
            }
        }
        for mut key in &*UnorderedMap::keyList(interface_map.clone()) {
            UnorderedMap::add(key.clone(), true, interface_map.clone())?;
        }
        Ok(derivative)
    }

    pub(crate) fn checkUseBeforeAssign(mut r#fn: &metamodelica::Ref<Function>) -> Result<()> {
        let mut unassigned: metamodelica::Ref<Vector::Vector<metamodelica::Ref<InstNode::InstNode>>>;
        let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
        let mut parent: metamodelica::Ref<InstNode::InstNode>;
        let mut sources: metamodelica::List<SourceInfo>;
        if isExternal(r#fn)? || isBuiltin(r#fn) {
            return Ok(());
        }
        unassigned = Vector::new(0);
        addUnassignedComponents(
            unassigned.clone(),
            &({
                let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
                for mut o in (r#fn.outputs.clone()).into_iter().cloned() {
                    let __x = InstNode::fromHandle(&(o.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?;
        addUnassignedComponents(unassigned.clone(), &r#fn.locals)?;
        body = getBody(r#fn)?;
        checkUseBeforeAssign2(unassigned.clone(), &body, None)?;
        for mut var in &*Vector::toList(unassigned) {
            if InstNode::isOutput(metamodelica::AsArg::as_arg(&var)) {
                parent = InstNode::parent(metamodelica::AsArg::as_arg(&var))?;
                sources = list![InstNode::info(metamodelica::AsArg::as_arg(&var))];
                if InstNode::isBaseClass(&parent) {
                    sources = metamodelica::cons(InstNode::info(&(InstNode::getDerivedNode(parent, true)?)), sources);
                }
                Error::addMultiSourceMessage(
                    &(Error::UNASSIGNED_FUNCTION_OUTPUT.clone()),
                    &(list![InstNode::name(metamodelica::AsArg::as_arg(&var))?]),
                    &sources,
                )?;
            }
        }
        Ok(())
    }

    pub(crate) fn addUnassignedComponents(
        mut unassigned: metamodelica::Ref<Vector::Vector<metamodelica::Ref<InstNode::InstNode>>>,
        mut variables: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    ) -> Result<()> {
        let mut ty: metamodelica::Ref<Type::NFType>;
        for mut var in &**variables {
            ty = InstNode::getType(var.clone())?;
            if Type::isScalarBuiltin(ty)?
                && !(Component::hasBinding(
                    &(InstNode::component(metamodelica::AsArg::as_arg(&var))?),
                    &(crate::NFInstNode::InstNode::interned_EMPTY_NODE()),
                )?)
            {
                Vector::push(unassigned.clone(), var.clone());
            }
        }
        Ok(())
    }

    pub(crate) fn checkUseBeforeAssign2(
        mut unassigned: metamodelica::Ref<Vector::Vector<metamodelica::Ref<InstNode::InstNode>>>,
        mut statements: &metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        mut generatedName: Option<ArcStr>,
    ) -> Result<()> {
        let mut info: SourceInfo;
        let mut shadowed: Option<metamodelica::Ref<InstNode::InstNode>>;
        let mut index: i32;
        for mut stmt in &**statements {
            info = Statement::info(metamodelica::AsArg::as_arg(&stmt));
            let () = (match &*stmt.clone() {
                Statement::ASSIGNMENT {
                    lhs: __stmt_lhs,
                    rhs: __stmt_rhs,
                    ..
                } => {
                    checkUseBeforeAssignExp(unassigned.clone(), __stmt_rhs.clone(), &info, generatedName.clone())?;
                    markAssignedOutput(
                        unassigned.clone(),
                        metamodelica::AsArg::as_arg(&__stmt_lhs),
                        (generatedName).is_some(),
                    )?;
                    ()
                }
                Statement::FOR {
                    body: __stmt_body,
                    iterator: __stmt_iterator,
                    range: __stmt_range,
                    ..
                } => {
                    if (__stmt_range).is_some() {
                        checkUseBeforeAssignExp(
                            unassigned.clone(),
                            Util::getOption(__stmt_range.clone())?,
                            &info,
                            generatedName.clone(),
                        )?;
                    }
                    shadowed = None;
                    if (generatedName).is_some() {
                        (shadowed, index) = Vector::find(
                            unassigned.clone(),
                            (std::sync::Arc::new({
                                let __pe_b0 = __stmt_iterator.clone();
                                move |__pe_a1| InstNode::nameEqual(&__pe_b0, &__pe_a1)
                            })
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<bool> + 'static,
                                >),
                        )?;
                        if index > 0 {
                            Vector::remove(unassigned.clone(), index)?;
                        }
                    }
                    checkUseBeforeAssign2(
                        unassigned.clone(),
                        metamodelica::AsArg::as_arg(&__stmt_body),
                        generatedName.clone(),
                    )?;
                    if (shadowed).is_some() {
                        Vector::push(unassigned.clone(), Util::getOption(shadowed)?);
                    }
                    ()
                }
                Statement::IF {
                    branches: __stmt_branches,
                    ..
                } => {
                    checkUseBeforeAssignIf(
                        unassigned.clone(),
                        metamodelica::AsArg::as_arg(&__stmt_branches),
                        &info,
                        generatedName.clone(),
                    )?;
                    ()
                }
                Statement::ASSERT {
                    condition: __stmt_condition,
                    level: __stmt_level,
                    message: __stmt_message,
                    ..
                } => {
                    checkUseBeforeAssignExp(
                        unassigned.clone(),
                        __stmt_condition.clone(),
                        &info,
                        generatedName.clone(),
                    )?;
                    checkUseBeforeAssignExp(unassigned.clone(), __stmt_message.clone(), &info, generatedName.clone())?;
                    checkUseBeforeAssignExp(unassigned.clone(), __stmt_level.clone(), &info, generatedName.clone())?;
                    ()
                }
                Statement::WHILE {
                    body: __stmt_body,
                    condition: __stmt_condition,
                    ..
                } => {
                    checkUseBeforeAssignExp(
                        unassigned.clone(),
                        __stmt_condition.clone(),
                        &info,
                        generatedName.clone(),
                    )?;
                    checkUseBeforeAssign2(
                        unassigned.clone(),
                        metamodelica::AsArg::as_arg(&__stmt_body),
                        generatedName.clone(),
                    )?;
                    ()
                }
                _ => (),
            });
        }
        Ok(())
    }

    pub(crate) fn markAssignedOutput(
        mut unassigned: metamodelica::Ref<Vector::Vector<metamodelica::Ref<InstNode::InstNode>>>,
        mut assignedExp: &metamodelica::Ref<Expression::NFExpression>,
        mut byName: bool,
    ) -> Result<()> {
        let mut node: metamodelica::Ref<InstNode::InstNode>;
        let mut index: i32;
        let () = (match &**assignedExp {
            Expression::CREF {
                cref: __assignedExp_cref,
                ..
            } if (ComponentRef::isCref(metamodelica::AsArg::as_arg(&__assignedExp_cref))) => {
                node = ComponentRef::node(&(ComponentRef::last(metamodelica::AsArg::as_arg(&__assignedExp_cref))))?;
                if byName {
                    (_, index) = Vector::find(
                        unassigned.clone(),
                        (std::sync::Arc::new({
                            let __pe_b0 = node;
                            move |__pe_a1| InstNode::nameEqual(&__pe_b0, &__pe_a1)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<bool> + 'static,
                            >),
                    )?;
                } else {
                    (_, index) = Vector::find(
                        unassigned.clone(),
                        (std::sync::Arc::new({
                            let __pe_b0 = node;
                            move |__pe_a1| InstNode::refEqual(&__pe_b0, &__pe_a1)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<bool> + 'static,
                            >),
                    )?;
                }
                if index > 0 {
                    Vector::remove(unassigned, index)?;
                }
                ()
            }
            Expression::TUPLE {
                elements: __assignedExp_elements,
                ..
            } => {
                for mut e in &*__assignedExp_elements.clone() {
                    markAssignedOutput(unassigned.clone(), metamodelica::AsArg::as_arg(&e), byName)?;
                }
                ()
            }
            _ => (),
        });
        Ok(())
    }

    pub(crate) fn checkUseBeforeAssignIf(
        mut unassigned: metamodelica::Ref<Vector::Vector<metamodelica::Ref<InstNode::InstNode>>>,
        mut branches: &metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        )>,
        mut info: &SourceInfo,
        mut generatedName: Option<ArcStr>,
    ) -> Result<()> {
        let mut unassigned_branch: metamodelica::Ref<Vector::Vector<metamodelica::Ref<InstNode::InstNode>>>;
        let mut assigned: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
        let mut index: i32;
        for mut b in &**branches {
            checkUseBeforeAssignExp(
                unassigned.clone(),
                Util::tuple21(b.clone()),
                info,
                generatedName.clone(),
            )?;
        }
        for mut b in &**branches {
            unassigned_branch = Vector::copy(unassigned.clone());
            checkUseBeforeAssign2(
                unassigned_branch.clone(),
                &(Util::tuple22(b.clone())),
                generatedName.clone(),
            )?;
            if Vector::size(unassigned.clone()) != Vector::size(unassigned_branch.clone()) {
                assigned = listAppend(
                    List::setDifferenceOnTrue(
                        Vector::toList(unassigned.clone()),
                        &(Vector::toList(unassigned_branch)),
                        &move |__a0: metamodelica::Ref<InstNode::InstNode>,
                               __a1: metamodelica::Ref<InstNode::InstNode>| {
                            InstNode::refEqual(&__a0, &__a1)
                        },
                    )?,
                    assigned,
                );
            }
        }
        if !((assigned).is_empty()) {
            assigned = List::uniqueOnTrue(
                &assigned,
                &move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<InstNode::InstNode>| {
                    InstNode::refEqual(&__a0, &__a1)
                },
            )?;
            for mut a in &*assigned {
                (_, index) = Vector::find(
                    unassigned.clone(),
                    (std::sync::Arc::new({
                        let __pe_b0 = a.clone();
                        move |__pe_a1| InstNode::refEqual(&__pe_b0, &__pe_a1)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<bool> + 'static,
                        >),
                )?;
                if index > 0 {
                    Vector::remove(unassigned.clone(), index)?;
                }
            }
        }
        Ok(())
    }

    pub(crate) fn checkUseBeforeAssignExp(
        mut unassigned: metamodelica::Ref<Vector::Vector<metamodelica::Ref<InstNode::InstNode>>>,
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut info: &SourceInfo,
        mut generatedName: Option<ArcStr>,
    ) -> Result<()> {
        Expression::apply(
            exp,
            &({
                let __pe_b0 = unassigned;
                let __pe_b2 = info.clone();
                let __pe_b3 = generatedName;
                move |__pe_a1| checkUseBeforeAssignExp_traverse(__pe_b0.clone(), &__pe_a1, &__pe_b2, __pe_b3.clone())
            }),
        )?;
        Ok(())
    }

    pub(crate) fn checkUseBeforeAssignExp_traverse(
        mut unassigned: metamodelica::Ref<Vector::Vector<metamodelica::Ref<InstNode::InstNode>>>,
        mut exp: &metamodelica::Ref<Expression::NFExpression>,
        mut info: &SourceInfo,
        mut generatedName: Option<ArcStr>,
    ) -> Result<()> {
        let mut index: i32;
        let mut node: metamodelica::Ref<InstNode::InstNode>;
        let mut fn_name: ArcStr;
        let () = (match &**exp {
            Expression::CREF { cref: __exp_cref, .. }
                if (ComponentRef::isCref(metamodelica::AsArg::as_arg(&__exp_cref))) =>
            {
                node = ComponentRef::node(&(ComponentRef::last(metamodelica::AsArg::as_arg(&__exp_cref))))?;
                if (generatedName).is_some() {
                    (_, index) = Vector::find(
                        unassigned.clone(),
                        (std::sync::Arc::new({
                            let __pe_b0 = node.clone();
                            move |__pe_a1| InstNode::nameEqual(&__pe_b0, &__pe_a1)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<bool> + 'static,
                            >),
                    )?;
                } else {
                    (_, index) = Vector::find(
                        unassigned.clone(),
                        (std::sync::Arc::new({
                            let __pe_b0 = node.clone();
                            move |__pe_a1| InstNode::refEqual(&__pe_b0, &__pe_a1)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<bool> + 'static,
                            >),
                    )?;
                }
                if index > 0 {
                    Vector::remove(unassigned, index)?;
                    let () = (match generatedName {
                        Some(mut __esc_fn_name) => {
                            fn_name = __esc_fn_name.clone();
                            Error::addSourceMessage(
                                &(Error::GENERATED_FUNCTION_USE_BEFORE_ASSIGN.clone()),
                                list![InstNode::name(&node)?, fn_name],
                                info,
                            )?;
                            ()
                        }
                        _ => {
                            Error::addSourceMessage(
                                &(Error::WARNING_DEF_USE.clone()),
                                list![InstNode::name(&node)?],
                                info,
                            )?;
                            ()
                        }
                    });
                }
                ()
            }
            _ => (),
        });
        Ok(())
    }

    pub fn checkUseBeforeAssignGenerated(
        mut r#fn: &metamodelica::Ref<Function>,
    ) -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>> {
        let mut uninitialized: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
        let mut unassigned: metamodelica::Ref<Vector::Vector<metamodelica::Ref<InstNode::InstNode>>>;
        let mut not_proven: metamodelica::Ref<Vector::Vector<metamodelica::Ref<InstNode::InstNode>>>;
        let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
        let mut fn_name: ArcStr;
        let mut index: i32;
        if isExternal(r#fn)? || isBuiltin(r#fn) {
            return Ok(uninitialized);
        }
        body = getBody(r#fn)?;
        if (body).is_empty() {
            return Ok(uninitialized);
        }
        fn_name = AbsynUtil::pathString(name(r#fn), literal!("."), true, false)?;
        unassigned = Vector::new(0);
        addUnassignedComponents(
            unassigned.clone(),
            &({
                let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
                for mut o in (r#fn.outputs.clone()).into_iter().cloned() {
                    let __x = InstNode::fromHandle(&(o.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?;
        addUnassignedComponents(unassigned.clone(), &r#fn.locals)?;
        not_proven = Vector::copy(unassigned.clone());
        checkUseBeforeAssign2(unassigned.clone(), &body, Some(fn_name.clone()))?;
        markProvenAssigned(not_proven.clone(), &body)?;
        for mut var in &*Vector::toList(unassigned.clone()) {
            if InstNode::isOutput(metamodelica::AsArg::as_arg(&var)) {
                if Type::isDiscrete(InstNode::getType(var.clone())?)? {
                    uninitialized = metamodelica::cons(var.clone(), uninitialized);
                } else {
                    Error::addSourceMessage(
                        &(Error::GENERATED_FUNCTION_UNASSIGNED_OUTPUT.clone()),
                        list![InstNode::name(metamodelica::AsArg::as_arg(&var))?, fn_name.clone()],
                        &(InstNode::info(metamodelica::AsArg::as_arg(&var))),
                    )?;
                }
            }
        }
        for mut var in &*Vector::toList(not_proven) {
            (_, index) = Vector::find(
                unassigned.clone(),
                (std::sync::Arc::new({
                    let __pe_b0 = var.clone();
                    move |__pe_a1| InstNode::nameEqual(&__pe_b0, &__pe_a1)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<bool> + 'static,
                    >),
            )?;
            if index <= 0 {
                uninitialized = metamodelica::cons(var.clone(), uninitialized);
            }
        }
        Ok(uninitialized)
    }

    pub fn initializeUninitialized(
        mut sections: metamodelica::Ref<Sections::NFSections>,
        mut variables: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        mut fn_name: ArcStr,
    ) -> Result<metamodelica::Ref<Sections::NFSections>> {
        let mut sections: metamodelica::Ref<Sections::NFSections> = sections;
        let mut inits: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
        let mut ty: metamodelica::Ref<Type::NFType>;
        let mut zero: metamodelica::Ref<Expression::NFExpression>;
        let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>;
        let mut rest: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
        let () = (::match_deref::match_deref! { match &(sections.clone()) {
            Deref @ Sections::SECTIONS { algorithms: Deref @ metamodelica::ListNode::Cons { head: __esc_alg, tail: __esc_rest }, .. } => {
                alg = (*__esc_alg).clone();
                rest = (*__esc_rest).clone();
                for mut var in &*variables.reverse() {
                    ty = InstNode::getType(var.clone())?;
                    zero = (match &*ty {
            Type::STRING => metamodelica::Ref::new(Expression::NFExpression::STRING { value: literal!("") }),
            _ => Expression::makeZero(&ty)?,
        });
                    Error::addSourceMessage(&(Error::GENERATED_FUNCTION_DEFAULT_INIT.clone()), list![InstNode::name(metamodelica::AsArg::as_arg(&var))?, fn_name.clone(), Expression::toString(zero.clone())?], &(InstNode::info(metamodelica::AsArg::as_arg(&var))))?;
                    inits = metamodelica::cons(metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT { lhs: Expression::fromCref(ComponentRef::fromNode(var.clone(), ty.clone(), metamodelica::nil(), ComponentRef::Origin::CREF.clone())?, false)?, rhs: zero, ty: ty, source: alg.source.clone() }), inits);
                }
                assign_field!(alg.statements = listAppend(inits, alg.statements.clone()));
                assign_variant_field!(sections => Sections::NFSections::SECTIONS; algorithms = metamodelica::cons(alg.clone(), rest.clone()));
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(sections)
    }

    pub(crate) fn markProvenAssigned(
        mut unassigned: metamodelica::Ref<Vector::Vector<metamodelica::Ref<InstNode::InstNode>>>,
        mut statements: &metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    ) -> Result<()> {
        for mut stmt in &**statements {
            let () = (match &*stmt.clone() {
                Statement::ASSIGNMENT { lhs: __stmt_lhs, .. } => {
                    markAssignedOutput(unassigned.clone(), metamodelica::AsArg::as_arg(&__stmt_lhs), true)?;
                    ()
                }
                Statement::IF {
                    branches: __stmt_branches,
                    ..
                } => {
                    markProvenAssignedIf(unassigned.clone(), metamodelica::AsArg::as_arg(&__stmt_branches))?;
                    ()
                }
                _ => (),
            });
        }
        Ok(())
    }

    pub(crate) fn markProvenAssignedIf(
        mut unassigned: metamodelica::Ref<Vector::Vector<metamodelica::Ref<InstNode::InstNode>>>,
        mut branches: &metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        )>,
    ) -> Result<()> {
        let mut has_else: bool = false;
        let mut unassigned_branch: metamodelica::Ref<Vector::Vector<metamodelica::Ref<InstNode::InstNode>>>;
        let mut still_unassigned: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
        let mut index: i32;
        for mut b in &**branches {
            has_else = Expression::isTrue(&(Util::tuple21(b.clone())));
            unassigned_branch = Vector::copy(unassigned.clone());
            markProvenAssigned(unassigned_branch.clone(), &(Util::tuple22(b.clone())))?;
            still_unassigned = listAppend(Vector::toList(unassigned_branch), still_unassigned);
        }
        if has_else {
            for mut node in &*Vector::toList(unassigned.clone()) {
                if !(List::isMemberOnTrue(node.clone(), &still_unassigned, &move |__a0: metamodelica::Ref<
                    InstNode::InstNode,
                >,
                                                                                  __a1: metamodelica::Ref<
                    InstNode::InstNode,
                >| {
                    InstNode::nameEqual(&__a0, &__a1)
                })?) {
                    (_, index) = Vector::find(
                        unassigned.clone(),
                        (std::sync::Arc::new({
                            let __pe_b0 = node.clone();
                            move |__pe_a1| InstNode::nameEqual(&__pe_b0, &__pe_a1)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<bool> + 'static,
                            >),
                    )?;
                    if index > 0 {
                        Vector::remove(unassigned.clone(), index)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn instPartialDerivedVars(
        mut classDef: &metamodelica::Ref<SCode::ClassDef>,
        mut inputs: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        mut r#fn: &metamodelica::Ref<Function>,
        mut context: i32,
        mut info: &SourceInfo,
    ) -> Result<metamodelica::List<i32>> {
        let mut derivedVars: metamodelica::List<i32> = metamodelica::nil();
        let mut index: i32;
        let () = (match &**classDef {
            SCode::ClassDef::PDER {
                derivedVariables: __classDef_derivedVariables,
                ..
            } => {
                for mut var in &*__classDef_derivedVariables.clone() {
                    index = List::positionOnTrue(
                        inputs,
                        &({
                            let __pe_b1 = var.clone();
                            move |__pe_a0| Ok(InstNode::isNamed(&__pe_a0, &__pe_b1))
                        }),
                    )?;
                    if index < 1 {
                        Error::addSourceMessage(
                            &(Error::PARTIAL_DERIVATIVE_INPUT_NOT_FOUND.clone()),
                            list![
                                var.clone(),
                                AbsynUtil::pathString(getDerivedFunctionName(r#fn)?, literal!("."), true, false)?
                            ],
                            info,
                        )?;
                        return Err("fail");
                    }
                    derivedVars = metamodelica::cons(index, derivedVars);
                }
                derivedVars = metamodelica::Dangerous::listReverseInPlace(derivedVars);
                ()
            }
            _ => (),
        });
        Ok(derivedVars)
    }
}
