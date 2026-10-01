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

use crate::ClassInf;
use crate::SCode;
use crate::Values;
use openmodelica_ast::Absyn;

// public imports
pub type Ident = ArcStr;

pub type InstDims = metamodelica::List<metamodelica::Ref<Dimension>>;

pub type StartValue = Option<metamodelica::Ref<Exp>>;

pub const UNIQUEIO: &'static str = "$unique$outer$";

pub const derivativeNamePrefix: &'static str = "$DER";

pub const partialDerivativeNamePrefix: &'static str = "$pDER";

pub const preNamePrefix: &'static str = "$PRE";

pub const previousNamePrefix: &'static str = "$CLKPRE";

pub const startNamePrefix: &'static str = "$START";

pub const auxNamePrefix: &'static str = "$AUX";

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum VarKind {
    /// variable
    VARIABLE,
    /// discrete
    DISCRETE,
    /// parameter
    PARAM,
    /// constant
    CONST,
}
impl metamodelica::gc::MMTrace for VarKind {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            VarKind::VARIABLE => Ok(()),
            VarKind::DISCRETE => Ok(()),
            VarKind::PARAM => Ok(()),
            VarKind::CONST => Ok(()),
        }
    }
}
impl Default for VarKind {
    fn default() -> Self {
        Self::VARIABLE
    }
}
pub use self::VarKind::{CONST, DISCRETE, PARAM, VARIABLE};

/// The type of a connector element.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ConnectorType {
    POTENTIAL,
    FLOW,
    STREAM {
        associatedFlow: Option<metamodelica::Ref<ComponentRef>>,
    },
    NON_CONNECTOR,
}
impl metamodelica::gc::MMTrace for ConnectorType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ConnectorType::POTENTIAL => Ok(()),
            ConnectorType::FLOW => Ok(()),
            ConnectorType::STREAM { associatedFlow } => {
                metamodelica::gc::MMTrace::mm_accept(associatedFlow, __mmv)?;
                Ok(())
            }
            ConnectorType::NON_CONNECTOR => Ok(()),
        }
    }
}
impl ConnectorType {
    pub fn interned_POTENTIAL() -> metamodelica::Ref<ConnectorType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<ConnectorType> = metamodelica::Ref::new(ConnectorType::POTENTIAL);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_FLOW() -> metamodelica::Ref<ConnectorType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<ConnectorType> = metamodelica::Ref::new(ConnectorType::FLOW);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_NON_CONNECTOR() -> metamodelica::Ref<ConnectorType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<ConnectorType> = metamodelica::Ref::new(ConnectorType::NON_CONNECTOR);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_POTENTIAL() -> metamodelica::Ref<ConnectorType> {
    ConnectorType::interned_POTENTIAL()
}
pub fn interned_FLOW() -> metamodelica::Ref<ConnectorType> {
    ConnectorType::interned_FLOW()
}
pub fn interned_NON_CONNECTOR() -> metamodelica::Ref<ConnectorType> {
    ConnectorType::interned_NON_CONNECTOR()
}
impl Default for ConnectorType {
    fn default() -> Self {
        Self::POTENTIAL
    }
}
pub use self::ConnectorType::{FLOW, NON_CONNECTOR, POTENTIAL, STREAM};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum VarDirection {
    /// input
    INPUT,
    /// output
    OUTPUT,
    /// neither input or output
    BIDIR,
}
impl metamodelica::gc::MMTrace for VarDirection {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            VarDirection::INPUT => Ok(()),
            VarDirection::OUTPUT => Ok(()),
            VarDirection::BIDIR => Ok(()),
        }
    }
}
impl Default for VarDirection {
    fn default() -> Self {
        Self::INPUT
    }
}
pub use self::VarDirection::{BIDIR, INPUT, OUTPUT};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum VarParallelism {
    /// Global variables for CUDA and OpenCL
    PARGLOBAL,
    /// Shared for CUDA and local for OpenCL
    PARLOCAL,
    /// Non parallel/Normal variables
    NON_PARALLEL,
}
impl metamodelica::gc::MMTrace for VarParallelism {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            VarParallelism::PARGLOBAL => Ok(()),
            VarParallelism::PARLOCAL => Ok(()),
            VarParallelism::NON_PARALLEL => Ok(()),
        }
    }
}
impl Default for VarParallelism {
    fn default() -> Self {
        Self::PARGLOBAL
    }
}
pub use self::VarParallelism::{NON_PARALLEL, PARGLOBAL, PARLOCAL};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum VarVisibility {
    /// public variables
    PUBLIC,
    /// protected variables
    PROTECTED,
}
impl metamodelica::gc::MMTrace for VarVisibility {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            VarVisibility::PUBLIC => Ok(()),
            VarVisibility::PROTECTED => Ok(()),
        }
    }
}
impl Default for VarVisibility {
    fn default() -> Self {
        Self::PUBLIC
    }
}
pub use self::VarVisibility::{PROTECTED, PUBLIC};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum VarInnerOuter {
    /// an inner prefix
    INNER,
    /// an outer prefix
    OUTER,
    /// an inner outer prefix
    INNER_OUTER,
    /// no inner outer prefix
    NOT_INNER_OUTER,
}
impl metamodelica::gc::MMTrace for VarInnerOuter {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            VarInnerOuter::INNER => Ok(()),
            VarInnerOuter::OUTER => Ok(()),
            VarInnerOuter::INNER_OUTER => Ok(()),
            VarInnerOuter::NOT_INNER_OUTER => Ok(()),
        }
    }
}
impl Default for VarInnerOuter {
    fn default() -> Self {
        Self::INNER
    }
}
pub use self::VarInnerOuter::{INNER, INNER_OUTER, NOT_INNER_OUTER, OUTER};

/// gives information about the origin of the element
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ElementSource {
    /// the line and column numbers of the equations and algorithms this element came from
    pub info: SourceInfo,
    /// the model(s) this element came from
    pub partOfLst: metamodelica::List<Absyn::Within>,
    /// the instance(s) this element is part of
    pub instance: metamodelica::Ref<ComponentPrefix>,
    /// this element came from this connect(s)
    pub connectEquationOptLst: metamodelica::List<(metamodelica::Ref<ComponentRef>, metamodelica::Ref<ComponentRef>)>,
    /// the classes where the type(s) of the element is defined
    pub typeLst: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    /// the symbolic operations used to end up with the final state of the element
    pub operations: metamodelica::List<metamodelica::Ref<SymbolicOperation>>,
    pub comment: metamodelica::List<metamodelica::Ref<SCode::Comment>>,
}

impl metamodelica::gc::MMTrace for ElementSource {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.info, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.partOfLst, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.instance, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.connectEquationOptLst, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.typeLst, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.operations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.comment, __mmv)?;
        Ok(())
    }
}
impl Default for ElementSource {
    fn default() -> Self {
        Self {
            info: Default::default(),
            partOfLst: Default::default(),
            instance: Default::default(),
            connectEquationOptLst: Default::default(),
            typeLst: Default::default(),
            operations: Default::default(),
            comment: Default::default(),
        }
    }
}

pub type SOURCE = ElementSource;

thread_local! { static __emptyElementSource_TLS: metamodelica::Ref<ElementSource> = metamodelica::Ref::new(ElementSource { info: Absyn::dummyInfo.clone(), partOfLst: metamodelica::nil(), instance: crate::DAE::ComponentPrefix::interned_NOCOMPPRE(), connectEquationOptLst: metamodelica::nil(), typeLst: metamodelica::nil(), operations: metamodelica::nil(), comment: metamodelica::nil() }); }
pub fn emptyElementSource() -> metamodelica::Ref<ElementSource> {
    __emptyElementSource_TLS.with(|__t| __t.clone())
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum SymbolicOperation {
    /// From one equation/statement to an element
    FLATTEN {
        scode: metamodelica::Ref<SCode::Equation>,
        dae: Option<metamodelica::Ref<Element>>,
    },
    /// Before and after expression is equivalent
    SIMPLIFY {
        before: metamodelica::Ref<EquationExp>,
        after: metamodelica::Ref<EquationExp>,
    },
    /// A chain of substitutions
    SUBSTITUTION {
        substitutions: metamodelica::List<metamodelica::Ref<Exp>>,
        source: metamodelica::Ref<Exp>,
    },
    /// Before and after inlining of function calls
    OP_INLINE {
        before: metamodelica::Ref<EquationExp>,
        after: metamodelica::Ref<EquationExp>,
    },
    /// Convert array equation into scalar equations; x = {1,2}, [1] => x[1] = {1}
    OP_SCALARIZE {
        before: metamodelica::Ref<EquationExp>,
        index: i32,
        after: metamodelica::Ref<EquationExp>,
    },
    /// Differentiate w.r.t. cr
    OP_DIFFERENTIATE {
        cr: metamodelica::Ref<ComponentRef>,
        before: metamodelica::Ref<Exp>,
        after: metamodelica::Ref<Exp>,
    },
    /// Solve equation, exp1 = exp2 => cr = exp; note that assertions may have been generated for example in case of divisions
    SOLVE {
        cr: metamodelica::Ref<ComponentRef>,
        exp1: metamodelica::Ref<Exp>,
        exp2: metamodelica::Ref<Exp>,
        res: metamodelica::Ref<Exp>,
        assertConds: metamodelica::List<metamodelica::Ref<Exp>>,
    },
    /// Equation is solved
    SOLVED {
        cr: metamodelica::Ref<ComponentRef>,
        exp: metamodelica::Ref<Exp>,
    },
    /// Solved linear system of equations
    LINEAR_SOLVED {
        vars: metamodelica::List<metamodelica::Ref<ComponentRef>>,
        jac: metamodelica::List<metamodelica::List<metamodelica::Real>>,
        rhs: metamodelica::List<metamodelica::Real>,
        result: metamodelica::List<metamodelica::Real>,
    },
    /// Introduced a dummy derivative (from index reduction)
    NEW_DUMMY_DER {
        chosen: metamodelica::Ref<ComponentRef>,
        candidates: metamodelica::List<metamodelica::Ref<ComponentRef>>,
    },
    /// Converted the equation into residual form, to use nonlinear equation solvers 0=e (0=e1-e2)
    OP_RESIDUAL {
        e1: metamodelica::Ref<Exp>,
        e2: metamodelica::Ref<Exp>,
        e: metamodelica::Ref<Exp>,
    },
}
impl metamodelica::gc::MMTrace for SymbolicOperation {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            SymbolicOperation::FLATTEN { scode, dae } => {
                metamodelica::gc::MMTrace::mm_accept(scode, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dae, __mmv)?;
                Ok(())
            }
            SymbolicOperation::SIMPLIFY { before, after } => {
                metamodelica::gc::MMTrace::mm_accept(before, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(after, __mmv)?;
                Ok(())
            }
            SymbolicOperation::SUBSTITUTION { substitutions, source } => {
                metamodelica::gc::MMTrace::mm_accept(substitutions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            SymbolicOperation::OP_INLINE { before, after } => {
                metamodelica::gc::MMTrace::mm_accept(before, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(after, __mmv)?;
                Ok(())
            }
            SymbolicOperation::OP_SCALARIZE { before, index, after } => {
                metamodelica::gc::MMTrace::mm_accept(before, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(after, __mmv)?;
                Ok(())
            }
            SymbolicOperation::OP_DIFFERENTIATE { cr, before, after } => {
                metamodelica::gc::MMTrace::mm_accept(cr, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(before, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(after, __mmv)?;
                Ok(())
            }
            SymbolicOperation::SOLVE {
                cr,
                exp1,
                exp2,
                res,
                assertConds,
            } => {
                metamodelica::gc::MMTrace::mm_accept(cr, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(res, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(assertConds, __mmv)?;
                Ok(())
            }
            SymbolicOperation::SOLVED { cr, exp } => {
                metamodelica::gc::MMTrace::mm_accept(cr, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            SymbolicOperation::LINEAR_SOLVED { vars, jac, rhs, result } => {
                metamodelica::gc::MMTrace::mm_accept(vars, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(jac, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(result, __mmv)?;
                Ok(())
            }
            SymbolicOperation::NEW_DUMMY_DER { chosen, candidates } => {
                metamodelica::gc::MMTrace::mm_accept(chosen, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(candidates, __mmv)?;
                Ok(())
            }
            SymbolicOperation::OP_RESIDUAL { e1, e2, e } => {
                metamodelica::gc::MMTrace::mm_accept(e1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(e2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(e, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for SymbolicOperation {
    fn default() -> Self {
        Self::FLATTEN {
            scode: Default::default(),
            dae: Default::default(),
        }
    }
}
pub use self::SymbolicOperation::{
    FLATTEN, LINEAR_SOLVED, NEW_DUMMY_DER, OP_DIFFERENTIATE, OP_INLINE, OP_RESIDUAL, OP_SCALARIZE, SIMPLIFY, SOLVE,
    SOLVED, SUBSTITUTION,
};

/// An equation on residual or equality form has 1 or 2 expressions. For use with symbolic operation tracing.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum EquationExp {
    /// An expression that is part of the whole equation
    PARTIAL_EQUATION { exp: metamodelica::Ref<Exp> },
    /// 0 = exp
    RESIDUAL_EXP { exp: metamodelica::Ref<Exp> },
    /// lhs = rhs
    EQUALITY_EXPS {
        lhs: metamodelica::Ref<Exp>,
        rhs: metamodelica::Ref<Exp>,
    },
}
impl metamodelica::gc::MMTrace for EquationExp {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            EquationExp::PARTIAL_EQUATION { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            EquationExp::RESIDUAL_EXP { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            EquationExp::EQUALITY_EXPS { lhs, rhs } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for EquationExp {
    fn default() -> Self {
        Self::PARTIAL_EQUATION {
            exp: Default::default(),
        }
    }
}
pub use self::EquationExp::{EQUALITY_EXPS, PARTIAL_EQUATION, RESIDUAL_EXP};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Element {
    VAR {
        /// The variable name
        componentRef: metamodelica::Ref<ComponentRef>,
        /// varible kind: variable, constant, parameter, discrete etc.
        kind: VarKind,
        /// input, output or bidir
        direction: VarDirection,
        /// parglobal, parlocal, or non_parallel
        parallelism: VarParallelism,
        /// if protected or public
        protection: VarVisibility,
        /// Full type information required
        ty: metamodelica::Ref<Type>,
        /// Binding expression e.g. for parameters ; value of start attribute
        binding: Option<metamodelica::Ref<Exp>>,
        /// dimensions
        dims: InstDims,
        /// The connector type: flow, stream, no prefix, or not a connector element.
        connectorType: metamodelica::Ref<ConnectorType>,
        /// the origins of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
        variableAttributesOption: Option<metamodelica::Ref<VariableAttributes>>,
        comment: Option<metamodelica::Ref<SCode::Comment>>,
        /// inner/outer required to 'change' outer references
        innerOuter: Absyn::InnerOuter,
        /// true if the variable belongs to an encrypted class
        encrypted: bool,
    },
    /// A solved equation
    DEFINE {
        componentRef: metamodelica::Ref<ComponentRef>,
        exp: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// A solved initial equation
    INITIALDEFINE {
        componentRef: metamodelica::Ref<ComponentRef>,
        exp: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// Scalar equation
    EQUATION {
        exp: metamodelica::Ref<Exp>,
        scalar: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// effort variable equality
    EQUEQUATION {
        cr1: metamodelica::Ref<ComponentRef>,
        cr2: metamodelica::Ref<ComponentRef>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// an array equation
    ARRAY_EQUATION {
        /// dimension sizes
        dimension: Dimensions,
        exp: metamodelica::Ref<Exp>,
        array: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// An initial array equation
    INITIAL_ARRAY_EQUATION {
        /// dimension sizes
        dimension: Dimensions,
        exp: metamodelica::Ref<Exp>,
        array: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// a connect equation
    CONNECT_EQUATION {
        lhsElement: metamodelica::Ref<Element>,
        lhsFace: Connect::Face,
        rhsElement: metamodelica::Ref<Element>,
        rhsFace: Connect::Face,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// an equation of complex type, e.g. record = func(..)
    COMPLEX_EQUATION {
        lhs: metamodelica::Ref<Exp>,
        rhs: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// an initial equation of complex type, e.g. record = func(..)
    INITIAL_COMPLEX_EQUATION {
        lhs: metamodelica::Ref<Exp>,
        rhs: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// a when equation
    WHEN_EQUATION {
        /// Condition
        condition: metamodelica::Ref<Exp>,
        /// Equations
        equations: metamodelica::List<metamodelica::Ref<Element>>,
        /// Elsewhen should be of type WHEN_EQUATION
        elsewhen_: Option<metamodelica::Ref<Element>>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// an initial for-equation
    INITIAL_FOR_EQUATION {
        /// this is the type of the iterator
        type_: metamodelica::Ref<Type>,
        /// True if the iterator has an array type, otherwise false.
        iterIsArray: bool,
        /// the iterator variable
        iter: Ident,
        /// the index of the iterator variable, to make it unique; used by the new inst
        index: i32,
        /// range for the loop
        range: metamodelica::Ref<Exp>,
        /// Equations
        equations: metamodelica::List<metamodelica::Ref<Element>>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// a for-equation
    FOR_EQUATION {
        /// this is the type of the iterator
        type_: metamodelica::Ref<Type>,
        /// True if the iterator has an array type, otherwise false.
        iterIsArray: bool,
        /// the iterator variable
        iter: Ident,
        /// the index of the iterator variable, to make it unique; used by the new inst
        index: i32,
        /// range for the loop
        range: metamodelica::Ref<Exp>,
        /// Equations
        equations: metamodelica::List<metamodelica::Ref<Element>>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// an if-equation
    IF_EQUATION {
        /// Condition
        condition1: metamodelica::List<metamodelica::Ref<Exp>>,
        /// Equations of true branch
        equations2: metamodelica::List<metamodelica::List<metamodelica::Ref<Element>>>,
        /// Equations of false branch
        equations3: metamodelica::List<metamodelica::Ref<Element>>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// An initial if-equation
    INITIAL_IF_EQUATION {
        /// Condition
        condition1: metamodelica::List<metamodelica::Ref<Exp>>,
        /// Equations of true branch
        equations2: metamodelica::List<metamodelica::List<metamodelica::Ref<Element>>>,
        /// Equations of false branch
        equations3: metamodelica::List<metamodelica::Ref<Element>>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// An initial equaton
    INITIALEQUATION {
        exp1: metamodelica::Ref<Exp>,
        exp2: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// An algorithm section
    ALGORITHM {
        algorithm_: metamodelica::Ref<Algorithm>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// An initial algorithm section
    INITIALALGORITHM {
        algorithm_: metamodelica::Ref<Algorithm>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    COMP {
        ident: Ident,
        /// a component with subelements, normally only used at top level.
        dAElist: metamodelica::List<metamodelica::Ref<Element>>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
        comment: Option<metamodelica::Ref<SCode::Comment>>,
    },
    /// The 'class' of an external object
    EXTOBJECTCLASS {
        /// className of external object
        path: metamodelica::Ref<Absyn::Path>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// The Modelica builtin assert
    ASSERT {
        condition: metamodelica::Ref<Exp>,
        message: metamodelica::Ref<Exp>,
        level: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// The Modelica builtin assert
    INITIAL_ASSERT {
        condition: metamodelica::Ref<Exp>,
        message: metamodelica::Ref<Exp>,
        level: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// The Modelica builtin terminate(msg)
    TERMINATE {
        message: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// The Modelica builtin terminate(msg)
    INITIAL_TERMINATE {
        message: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// reinit operator for reinitialization of states
    REINIT {
        componentRef: metamodelica::Ref<ComponentRef>,
        exp: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// call with no return value, i.e. no equation.
    ///    Typically sideeffect call of external function but also
    ///    Connections.* i.e. Connections.root(...) functions.
    NORETCALL {
        exp: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// call with no return value, i.e. no equation.
    ///    Typically sideeffect call of external function but also
    ///    Connections.* i.e. Connections.root(...) functions.
    INITIAL_NORETCALL {
        exp: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// constraint section
    CONSTRAINT {
        constraints: metamodelica::Ref<Constraint>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    CLASS_ATTRIBUTES {
        classAttrs: metamodelica::Ref<ClassAttributes>,
    },
    /// Flat state machine section
    FLAT_SM {
        ident: Ident,
        /// The states/modes transitions and variable
        ///                      merging equations within the the flat state machine
        dAElist: metamodelica::List<metamodelica::Ref<Element>>,
    },
    /// A state/mode component in a state machine
    SM_COMP {
        componentRef: metamodelica::Ref<ComponentRef>,
        /// a component with subelements
        dAElist: metamodelica::List<metamodelica::Ref<Element>>,
    },
    COMMENT {
        /// Functions store the inherited class annotations in the DAE
        cmt: metamodelica::Ref<SCode::Comment>,
    },
}
impl metamodelica::gc::MMTrace for Element {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Element::VAR {
                componentRef,
                kind,
                direction,
                parallelism,
                protection,
                ty,
                binding,
                dims,
                connectorType,
                source,
                variableAttributesOption,
                comment,
                innerOuter,
                encrypted,
            } => {
                metamodelica::gc::MMTrace::mm_accept(componentRef, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(kind, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(direction, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(parallelism, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(protection, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dims, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(connectorType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variableAttributesOption, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(comment, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(innerOuter, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(encrypted, __mmv)?;
                Ok(())
            }
            Element::DEFINE {
                componentRef,
                exp,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(componentRef, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::INITIALDEFINE {
                componentRef,
                exp,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(componentRef, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::EQUATION { exp, scalar, source } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scalar, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::EQUEQUATION { cr1, cr2, source } => {
                metamodelica::gc::MMTrace::mm_accept(cr1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cr2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::ARRAY_EQUATION {
                dimension,
                exp,
                array,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(dimension, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(array, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::INITIAL_ARRAY_EQUATION {
                dimension,
                exp,
                array,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(dimension, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(array, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::CONNECT_EQUATION {
                lhsElement,
                lhsFace,
                rhsElement,
                rhsFace,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(lhsElement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lhsFace, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhsElement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhsFace, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::COMPLEX_EQUATION { lhs, rhs, source } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::INITIAL_COMPLEX_EQUATION { lhs, rhs, source } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::WHEN_EQUATION {
                condition,
                equations,
                elsewhen_,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elsewhen_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::INITIAL_FOR_EQUATION {
                type_,
                iterIsArray,
                iter,
                index,
                range,
                equations,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iterIsArray, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iter, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(range, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::FOR_EQUATION {
                type_,
                iterIsArray,
                iter,
                index,
                range,
                equations,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iterIsArray, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iter, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(range, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::IF_EQUATION {
                condition1,
                equations2,
                equations3,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(condition1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equations2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equations3, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::INITIAL_IF_EQUATION {
                condition1,
                equations2,
                equations3,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(condition1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equations2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equations3, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::INITIALEQUATION { exp1, exp2, source } => {
                metamodelica::gc::MMTrace::mm_accept(exp1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::ALGORITHM { algorithm_, source } => {
                metamodelica::gc::MMTrace::mm_accept(algorithm_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::INITIALALGORITHM { algorithm_, source } => {
                metamodelica::gc::MMTrace::mm_accept(algorithm_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::COMP {
                ident,
                dAElist,
                source,
                comment,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ident, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dAElist, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(comment, __mmv)?;
                Ok(())
            }
            Element::EXTOBJECTCLASS { path, source } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::ASSERT {
                condition,
                message,
                level,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(level, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::INITIAL_ASSERT {
                condition,
                message,
                level,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(level, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::TERMINATE { message, source } => {
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::INITIAL_TERMINATE { message, source } => {
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::REINIT {
                componentRef,
                exp,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(componentRef, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::NORETCALL { exp, source } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::INITIAL_NORETCALL { exp, source } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::CONSTRAINT { constraints, source } => {
                metamodelica::gc::MMTrace::mm_accept(constraints, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Element::CLASS_ATTRIBUTES { classAttrs } => {
                metamodelica::gc::MMTrace::mm_accept(classAttrs, __mmv)?;
                Ok(())
            }
            Element::FLAT_SM { ident, dAElist } => {
                metamodelica::gc::MMTrace::mm_accept(ident, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dAElist, __mmv)?;
                Ok(())
            }
            Element::SM_COMP { componentRef, dAElist } => {
                metamodelica::gc::MMTrace::mm_accept(componentRef, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dAElist, __mmv)?;
                Ok(())
            }
            Element::COMMENT { cmt } => {
                metamodelica::gc::MMTrace::mm_accept(cmt, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Element {
    fn default() -> Self {
        Self::CLASS_ATTRIBUTES {
            classAttrs: Default::default(),
        }
    }
}
pub use self::Element::{
    ALGORITHM, ARRAY_EQUATION, ASSERT, CLASS_ATTRIBUTES, COMMENT, COMP, COMPLEX_EQUATION, CONNECT_EQUATION, CONSTRAINT,
    DEFINE, EQUATION, EQUEQUATION, EXTOBJECTCLASS, FLAT_SM, FOR_EQUATION, IF_EQUATION, INITIAL_ARRAY_EQUATION,
    INITIAL_ASSERT, INITIAL_COMPLEX_EQUATION, INITIAL_FOR_EQUATION, INITIAL_IF_EQUATION, INITIAL_NORETCALL,
    INITIAL_TERMINATE, INITIALALGORITHM, INITIALDEFINE, INITIALEQUATION, NORETCALL, REINIT, SM_COMP, TERMINATE, VAR,
    WHEN_EQUATION,
};

thread_local! { static __T_ASSERTIONLEVEL_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_ENUMERATION { index: None, path: metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("AssertionLevel") }) }), names: list![literal!("warning"), literal!("error")], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }); }
pub fn T_ASSERTIONLEVEL() -> metamodelica::Ref<Type> {
    __T_ASSERTIONLEVEL_TLS.with(|__t| __t.clone())
}

thread_local! { static __ASSERTIONLEVEL_WARNING_TLS: metamodelica::Ref<Exp> = metamodelica::Ref::new(Exp::ENUM_LITERAL { name: metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: literal!("AssertionLevel"), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("warning") }) }), index: 1 }); }
pub fn ASSERTIONLEVEL_WARNING() -> metamodelica::Ref<Exp> {
    __ASSERTIONLEVEL_WARNING_TLS.with(|__t| __t.clone())
}

thread_local! { static __ASSERTIONLEVEL_ERROR_TLS: metamodelica::Ref<Exp> = metamodelica::Ref::new(Exp::ENUM_LITERAL { name: metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: literal!("AssertionLevel"), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("error") }) }), index: 2 }); }
pub fn ASSERTIONLEVEL_ERROR() -> metamodelica::Ref<Exp> {
    __ASSERTIONLEVEL_ERROR_TLS.with(|__t| __t.clone())
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Function {
    /// A Modelica function
    FUNCTION {
        path: metamodelica::Ref<Absyn::Path>,
        /// contains the body and an optional function derivative mapping
        functions: metamodelica::List<FunctionDefinition>,
        type_: metamodelica::Ref<Type>,
        visibility: SCode::Visibility,
        /// MetaModelica extension
        partialPrefix: bool,
        /// Modelica 3.3 impure/pure, by default isImpure = false all the time only if prefix *impure* function is specified
        isImpure: bool,
        inlineType: InlineType,
        /// The indices of any inputs not used in the function.
        unusedInputs: metamodelica::List<i32>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
        comment: Option<metamodelica::Ref<SCode::Comment>>,
    },
    /// A Modelica record constructor. The function can be generated from the Path and Type alone.
    RECORD_CONSTRUCTOR {
        path: metamodelica::Ref<Absyn::Path>,
        type_: metamodelica::Ref<Type>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
}
impl metamodelica::gc::MMTrace for Function {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Function::FUNCTION {
                path,
                functions,
                type_,
                visibility,
                partialPrefix,
                isImpure,
                inlineType,
                unusedInputs,
                source,
                comment,
            } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(functions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(visibility, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(partialPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isImpure, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(inlineType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(unusedInputs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(comment, __mmv)?;
                Ok(())
            }
            Function::RECORD_CONSTRUCTOR { path, type_, source } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Function {
    fn default() -> Self {
        Self::RECORD_CONSTRUCTOR {
            path: Default::default(),
            type_: Default::default(),
            source: Default::default(),
        }
    }
}
pub use self::Function::{FUNCTION, RECORD_CONSTRUCTOR};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum InlineType {
    /// Normal inline, inline as soon as possible
    NORM_INLINE,
    /// Inline even if inlining is globally disabled by flags.
    BUILTIN_EARLY_INLINE,
    /// Inline even earlier than NORM_INLINE. This will display the inlined code in the flattened model and also works for functions calling other functions that should be inlined.
    EARLY_INLINE,
    /// no user option, tool can inline this functio if necessary
    DEFAULT_INLINE,
    /// don't inline this function, set with Inline=false
    NO_INLINE,
    /// Try to inline after index reduction
    AFTER_INDEX_RED_INLINE,
}
impl metamodelica::gc::MMTrace for InlineType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            InlineType::NORM_INLINE => Ok(()),
            InlineType::BUILTIN_EARLY_INLINE => Ok(()),
            InlineType::EARLY_INLINE => Ok(()),
            InlineType::DEFAULT_INLINE => Ok(()),
            InlineType::NO_INLINE => Ok(()),
            InlineType::AFTER_INDEX_RED_INLINE => Ok(()),
        }
    }
}
impl Default for InlineType {
    fn default() -> Self {
        Self::NORM_INLINE
    }
}
pub use self::InlineType::{
    AFTER_INDEX_RED_INLINE, BUILTIN_EARLY_INLINE, DEFAULT_INLINE, EARLY_INLINE, NO_INLINE, NORM_INLINE,
};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum FunctionDefinition {
    /// Normal function body
    FUNCTION_DEF {
        body: metamodelica::List<metamodelica::Ref<Element>>,
    },
    /// Normal external function declaration
    FUNCTION_EXT {
        body: metamodelica::List<metamodelica::Ref<Element>>,
        externalDecl: ExternalDecl,
    },
    /// Contains derivatives for function
    FUNCTION_DER_MAPPER {
        /// Function that is derived
        derivedFunction: metamodelica::Ref<Absyn::Path>,
        /// Path to derivative function
        derivativeFunction: metamodelica::Ref<Absyn::Path>,
        /// in case a function have multiple derivatives, include all
        derivativeOrder: i32,
        conditionRefs: metamodelica::List<(i32, derivativeCond)>,
        /// if conditions fails, use default derivative if exists
        defaultDerivative: Option<metamodelica::Ref<Absyn::Path>>,
        lowerOrderDerivatives: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    },
    /// A function inverse declaration
    FUNCTION_INVERSE {
        /// The input parameter the inverse is for
        inputParam: metamodelica::Ref<ComponentRef>,
        /// The inverse function call
        inverseCall: metamodelica::Ref<Exp>,
    },
    FUNCTION_PARTIAL_DERIVATIVE {
        derivedFunction: metamodelica::Ref<Absyn::Path>,
        derivedVars: metamodelica::List<ArcStr>,
    },
}
impl metamodelica::gc::MMTrace for FunctionDefinition {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            FunctionDefinition::FUNCTION_DEF { body } => {
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                Ok(())
            }
            FunctionDefinition::FUNCTION_EXT { body, externalDecl } => {
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(externalDecl, __mmv)?;
                Ok(())
            }
            FunctionDefinition::FUNCTION_DER_MAPPER {
                derivedFunction,
                derivativeFunction,
                derivativeOrder,
                conditionRefs,
                defaultDerivative,
                lowerOrderDerivatives,
            } => {
                metamodelica::gc::MMTrace::mm_accept(derivedFunction, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(derivativeFunction, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(derivativeOrder, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(conditionRefs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(defaultDerivative, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lowerOrderDerivatives, __mmv)?;
                Ok(())
            }
            FunctionDefinition::FUNCTION_INVERSE {
                inputParam,
                inverseCall,
            } => {
                metamodelica::gc::MMTrace::mm_accept(inputParam, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(inverseCall, __mmv)?;
                Ok(())
            }
            FunctionDefinition::FUNCTION_PARTIAL_DERIVATIVE {
                derivedFunction,
                derivedVars,
            } => {
                metamodelica::gc::MMTrace::mm_accept(derivedFunction, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(derivedVars, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for FunctionDefinition {
    fn default() -> Self {
        Self::FUNCTION_DEF {
            body: Default::default(),
        }
    }
}
pub use self::FunctionDefinition::{
    FUNCTION_DEF, FUNCTION_DER_MAPPER, FUNCTION_EXT, FUNCTION_INVERSE, FUNCTION_PARTIAL_DERIVATIVE,
};

/// Different conditions on derivatives
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum derivativeCond {
    ZERO_DERIVATIVE,
    NO_DERIVATIVE { binding: metamodelica::Ref<Exp> },
}
impl metamodelica::gc::MMTrace for derivativeCond {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            derivativeCond::ZERO_DERIVATIVE => Ok(()),
            derivativeCond::NO_DERIVATIVE { binding } => {
                metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for derivativeCond {
    fn default() -> Self {
        Self::ZERO_DERIVATIVE
    }
}
pub use self::derivativeCond::{NO_DERIVATIVE, ZERO_DERIVATIVE};

/// where the start attribute of a variable was set
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum StartOrigin {
    UNDEFINED_ORIGIN,
    /// start set by the type's default
    TYPE_ORIGIN,
    /// start set by a modifier
    BINDING_ORIGIN,
    CONFIDENCE {
        /// parameter-followed confidence per MLS 8.6.2, lower = stronger
        actual: i32,
        /// confidence of the start attribute itself, used as tie-break
        raw: i32,
    },
    TYPE_CONFIDENCE {
        /// start set by the type, at the level where the type was determined
        level: i32,
    },
}
impl metamodelica::gc::MMTrace for StartOrigin {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            StartOrigin::UNDEFINED_ORIGIN => Ok(()),
            StartOrigin::TYPE_ORIGIN => Ok(()),
            StartOrigin::BINDING_ORIGIN => Ok(()),
            StartOrigin::CONFIDENCE { actual, raw } => {
                metamodelica::gc::MMTrace::mm_accept(actual, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(raw, __mmv)?;
                Ok(())
            }
            StartOrigin::TYPE_CONFIDENCE { level } => {
                metamodelica::gc::MMTrace::mm_accept(level, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::StartOrigin::{BINDING_ORIGIN, CONFIDENCE, TYPE_CONFIDENCE, TYPE_ORIGIN, UNDEFINED_ORIGIN};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum VariableAttributes {
    VAR_ATTR_REAL {
        /// quantity
        quantity: Option<metamodelica::Ref<Exp>>,
        /// unit
        unit: Option<metamodelica::Ref<Exp>>,
        /// displayUnit
        displayUnit: Option<metamodelica::Ref<Exp>>,
        min: Option<metamodelica::Ref<Exp>>,
        max: Option<metamodelica::Ref<Exp>>,
        /// start value
        start: Option<metamodelica::Ref<Exp>>,
        /// fixed - true: default for parameter/constant, false - default for other variables
        fixed: Option<metamodelica::Ref<Exp>>,
        /// nominal
        nominal: Option<metamodelica::Ref<Exp>>,
        stateSelectOption: Option<StateSelect>,
        uncertainOption: Option<Uncertainty>,
        distributionOption: Option<metamodelica::Ref<Distribution>>,
        equationBound: Option<metamodelica::Ref<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<StartOrigin>,
    },
    VAR_ATTR_INT {
        /// quantity
        quantity: Option<metamodelica::Ref<Exp>>,
        min: Option<metamodelica::Ref<Exp>>,
        max: Option<metamodelica::Ref<Exp>>,
        /// start value
        start: Option<metamodelica::Ref<Exp>>,
        /// fixed - true: default for parameter/constant, false - default for other variables
        fixed: Option<metamodelica::Ref<Exp>>,
        uncertainOption: Option<Uncertainty>,
        distributionOption: Option<metamodelica::Ref<Distribution>>,
        equationBound: Option<metamodelica::Ref<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<StartOrigin>,
    },
    VAR_ATTR_BOOL {
        /// quantity
        quantity: Option<metamodelica::Ref<Exp>>,
        /// start value
        start: Option<metamodelica::Ref<Exp>>,
        /// fixed - true: default for parameter/constant, false - default for other variables
        fixed: Option<metamodelica::Ref<Exp>>,
        equationBound: Option<metamodelica::Ref<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<StartOrigin>,
    },
    VAR_ATTR_CLOCK {
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
    },
    VAR_ATTR_STRING {
        /// quantity
        quantity: Option<metamodelica::Ref<Exp>>,
        /// start value
        start: Option<metamodelica::Ref<Exp>>,
        /// new in Modelica 3.4; fixed - true: default for parameter/constant, false - default for other variables
        fixed: Option<metamodelica::Ref<Exp>>,
        equationBound: Option<metamodelica::Ref<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<StartOrigin>,
    },
    VAR_ATTR_ENUMERATION {
        /// quantity
        quantity: Option<metamodelica::Ref<Exp>>,
        min: Option<metamodelica::Ref<Exp>>,
        max: Option<metamodelica::Ref<Exp>>,
        /// start
        start: Option<metamodelica::Ref<Exp>>,
        /// fixed - true: default for parameter/constant, false - default for other variables
        fixed: Option<metamodelica::Ref<Exp>>,
        equationBound: Option<metamodelica::Ref<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<StartOrigin>,
    },
}
impl metamodelica::gc::MMTrace for VariableAttributes {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            VariableAttributes::VAR_ATTR_REAL {
                quantity,
                unit,
                displayUnit,
                min,
                max,
                start,
                fixed,
                nominal,
                stateSelectOption,
                uncertainOption,
                distributionOption,
                equationBound,
                isProtected,
                finalPrefix,
                startOrigin,
            } => {
                metamodelica::gc::MMTrace::mm_accept(quantity, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(unit, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(displayUnit, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(min, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(max, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(nominal, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(stateSelectOption, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(uncertainOption, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(distributionOption, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equationBound, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isProtected, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startOrigin, __mmv)?;
                Ok(())
            }
            VariableAttributes::VAR_ATTR_INT {
                quantity,
                min,
                max,
                start,
                fixed,
                uncertainOption,
                distributionOption,
                equationBound,
                isProtected,
                finalPrefix,
                startOrigin,
            } => {
                metamodelica::gc::MMTrace::mm_accept(quantity, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(min, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(max, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(uncertainOption, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(distributionOption, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equationBound, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isProtected, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startOrigin, __mmv)?;
                Ok(())
            }
            VariableAttributes::VAR_ATTR_BOOL {
                quantity,
                start,
                fixed,
                equationBound,
                isProtected,
                finalPrefix,
                startOrigin,
            } => {
                metamodelica::gc::MMTrace::mm_accept(quantity, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equationBound, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isProtected, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startOrigin, __mmv)?;
                Ok(())
            }
            VariableAttributes::VAR_ATTR_CLOCK {
                isProtected,
                finalPrefix,
            } => {
                metamodelica::gc::MMTrace::mm_accept(isProtected, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                Ok(())
            }
            VariableAttributes::VAR_ATTR_STRING {
                quantity,
                start,
                fixed,
                equationBound,
                isProtected,
                finalPrefix,
                startOrigin,
            } => {
                metamodelica::gc::MMTrace::mm_accept(quantity, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equationBound, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isProtected, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startOrigin, __mmv)?;
                Ok(())
            }
            VariableAttributes::VAR_ATTR_ENUMERATION {
                quantity,
                min,
                max,
                start,
                fixed,
                equationBound,
                isProtected,
                finalPrefix,
                startOrigin,
            } => {
                metamodelica::gc::MMTrace::mm_accept(quantity, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(min, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(max, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equationBound, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isProtected, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startOrigin, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for VariableAttributes {
    fn default() -> Self {
        Self::VAR_ATTR_CLOCK {
            isProtected: Default::default(),
            finalPrefix: Default::default(),
        }
    }
}
pub use self::VariableAttributes::{
    VAR_ATTR_BOOL, VAR_ATTR_CLOCK, VAR_ATTR_ENUMERATION, VAR_ATTR_INT, VAR_ATTR_REAL, VAR_ATTR_STRING,
};

thread_local! { static __emptyVarAttrReal_TLS: metamodelica::Ref<VariableAttributes> = metamodelica::Ref::new(VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }); }
pub fn emptyVarAttrReal() -> metamodelica::Ref<VariableAttributes> {
    __emptyVarAttrReal_TLS.with(|__t| __t.clone())
}

thread_local! { static __emptyVarAttrInt_TLS: metamodelica::Ref<VariableAttributes> = metamodelica::Ref::new(VariableAttributes::VAR_ATTR_INT { quantity: None, min: None, max: None, start: None, fixed: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }); }
pub fn emptyVarAttrInt() -> metamodelica::Ref<VariableAttributes> {
    __emptyVarAttrInt_TLS.with(|__t| __t.clone())
}

thread_local! { static __emptyVarAttrBool_TLS: metamodelica::Ref<VariableAttributes> = metamodelica::Ref::new(VariableAttributes::VAR_ATTR_BOOL { quantity: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }); }
pub fn emptyVarAttrBool() -> metamodelica::Ref<VariableAttributes> {
    __emptyVarAttrBool_TLS.with(|__t| __t.clone())
}

thread_local! { static __emptyVarAttrClock_TLS: metamodelica::Ref<VariableAttributes> = metamodelica::Ref::new(VariableAttributes::VAR_ATTR_CLOCK { isProtected: None, finalPrefix: None }); }
pub fn emptyVarAttrClock() -> metamodelica::Ref<VariableAttributes> {
    __emptyVarAttrClock_TLS.with(|__t| __t.clone())
}

thread_local! { static __emptyVarAttrString_TLS: metamodelica::Ref<VariableAttributes> = metamodelica::Ref::new(VariableAttributes::VAR_ATTR_STRING { quantity: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }); }
pub fn emptyVarAttrString() -> metamodelica::Ref<VariableAttributes> {
    __emptyVarAttrString_TLS.with(|__t| __t.clone())
}

thread_local! { static __emptyVarAttrEnum_TLS: metamodelica::Ref<VariableAttributes> = metamodelica::Ref::new(VariableAttributes::VAR_ATTR_ENUMERATION { quantity: None, min: None, max: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }); }
pub fn emptyVarAttrEnum() -> metamodelica::Ref<VariableAttributes> {
    __emptyVarAttrEnum_TLS.with(|__t| __t.clone())
}

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum StateSelect {
    NEVER,
    AVOID,
    DEFAULT,
    PREFER,
    ALWAYS,
}
impl metamodelica::gc::MMTrace for StateSelect {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            StateSelect::NEVER => Ok(()),
            StateSelect::AVOID => Ok(()),
            StateSelect::DEFAULT => Ok(()),
            StateSelect::PREFER => Ok(()),
            StateSelect::ALWAYS => Ok(()),
        }
    }
}
impl Default for StateSelect {
    fn default() -> Self {
        Self::NEVER
    }
}
pub use self::StateSelect::{ALWAYS, AVOID, DEFAULT, NEVER, PREFER};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Uncertainty {
    GIVEN,
    SOUGHT,
    REFINE,
    PROPAGATE,
}
impl metamodelica::gc::MMTrace for Uncertainty {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Uncertainty::GIVEN => Ok(()),
            Uncertainty::SOUGHT => Ok(()),
            Uncertainty::REFINE => Ok(()),
            Uncertainty::PROPAGATE => Ok(()),
        }
    }
}
impl Default for Uncertainty {
    fn default() -> Self {
        Self::GIVEN
    }
}
pub use self::Uncertainty::{GIVEN, PROPAGATE, REFINE, SOUGHT};

/// see Distribution record in Distribution
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Distribution {
    pub name: metamodelica::Ref<Exp>,
    pub params: metamodelica::Ref<Exp>,
    pub paramNames: metamodelica::Ref<Exp>,
}

impl metamodelica::gc::MMTrace for Distribution {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.params, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.paramNames, __mmv)?;
        Ok(())
    }
}
impl Default for Distribution {
    fn default() -> Self {
        Self {
            name: Default::default(),
            params: Default::default(),
            paramNames: Default::default(),
        }
    }
}

pub type DISTRIBUTION = Distribution;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ExtArg {
    EXTARG {
        componentRef: metamodelica::Ref<ComponentRef>,
        direction: Absyn::Direction,
        type_: metamodelica::Ref<Type>,
    },
    EXTARGEXP {
        exp: metamodelica::Ref<Exp>,
        type_: metamodelica::Ref<Type>,
    },
    EXTARGSIZE {
        componentRef: metamodelica::Ref<ComponentRef>,
        type_: metamodelica::Ref<Type>,
        exp: metamodelica::Ref<Exp>,
    },
    NOEXTARG,
}
impl metamodelica::gc::MMTrace for ExtArg {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ExtArg::EXTARG {
                componentRef,
                direction,
                type_,
            } => {
                metamodelica::gc::MMTrace::mm_accept(componentRef, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(direction, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                Ok(())
            }
            ExtArg::EXTARGEXP { exp, type_ } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                Ok(())
            }
            ExtArg::EXTARGSIZE {
                componentRef,
                type_,
                exp,
            } => {
                metamodelica::gc::MMTrace::mm_accept(componentRef, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            ExtArg::NOEXTARG => Ok(()),
        }
    }
}
impl Default for ExtArg {
    fn default() -> Self {
        Self::NOEXTARG
    }
}
pub use self::ExtArg::{EXTARG, EXTARGEXP, EXTARGSIZE, NOEXTARG};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ExternalDecl {
    pub name: ArcStr,
    pub args: metamodelica::List<ExtArg>,
    pub returnArg: ExtArg,
    pub language: ArcStr,
    pub ann: Option<metamodelica::Ref<SCode::Annotation>>,
}

impl metamodelica::gc::MMTrace for ExternalDecl {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.args, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.returnArg, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.language, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ann, __mmv)?;
        Ok(())
    }
}
impl Default for ExternalDecl {
    fn default() -> Self {
        Self {
            name: Default::default(),
            args: Default::default(),
            returnArg: Default::default(),
            language: Default::default(),
            ann: Default::default(),
        }
    }
}

pub type EXTERNALDECL = ExternalDecl;

/// A DAElist is a list of Elements. Variables, equations, functions,
///  algorithms, etc. are all found in this list.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct DAElist {
    pub elementLst: metamodelica::List<metamodelica::Ref<Element>>,
}

impl metamodelica::gc::MMTrace for DAElist {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.elementLst, __mmv)?;
        Ok(())
    }
}
impl Default for DAElist {
    fn default() -> Self {
        Self {
            elementLst: Default::default(),
        }
    }
}

pub type DAE = DAElist;

/* -- Algorithm.mo -- */
/// The `Algorithm\' type corresponds to a whole algorithm section.
///  It is simple a list of algorithm statements.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Algorithm {
    pub statementLst: metamodelica::List<metamodelica::Ref<Statement>>,
}

impl metamodelica::gc::MMTrace for Algorithm {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.statementLst, __mmv)?;
        Ok(())
    }
}
impl Default for Algorithm {
    fn default() -> Self {
        Self {
            statementLst: Default::default(),
        }
    }
}

pub type ALGORITHM_STMTS = Algorithm;

/// Optimica extension: The `Constraints\' type corresponds to a whole Constraint section.
///  It is simple a list of expressions.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Constraint {
    CONSTRAINT_EXPS {
        constraintLst: metamodelica::List<metamodelica::Ref<Exp>>,
    },
    /// Constraints needed for proper Dynamic Tearing
    CONSTRAINT_DT {
        constraint: metamodelica::Ref<Exp>,
        /// local or global constraint; local constraints depend on variables that are computed within the algebraic loop itself
        localCon: bool,
    },
}
impl metamodelica::gc::MMTrace for Constraint {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Constraint::CONSTRAINT_EXPS { constraintLst } => {
                metamodelica::gc::MMTrace::mm_accept(constraintLst, __mmv)?;
                Ok(())
            }
            Constraint::CONSTRAINT_DT { constraint, localCon } => {
                metamodelica::gc::MMTrace::mm_accept(constraint, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(localCon, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Constraint {
    fn default() -> Self {
        Self::CONSTRAINT_EXPS {
            constraintLst: Default::default(),
        }
    }
}
pub use self::Constraint::{CONSTRAINT_DT, CONSTRAINT_EXPS};

/// currently for Optimica extension: these are the objectives of optimization class
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ClassAttributes {
    pub objetiveE: Option<metamodelica::Ref<Exp>>,
    pub objectiveIntegrandE: Option<metamodelica::Ref<Exp>>,
    pub startTimeE: Option<metamodelica::Ref<Exp>>,
    pub finalTimeE: Option<metamodelica::Ref<Exp>>,
}

impl metamodelica::gc::MMTrace for ClassAttributes {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.objetiveE, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.objectiveIntegrandE, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.startTimeE, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.finalTimeE, __mmv)?;
        Ok(())
    }
}
impl Default for ClassAttributes {
    fn default() -> Self {
        Self {
            objetiveE: Default::default(),
            objectiveIntegrandE: Default::default(),
            startTimeE: Default::default(),
            finalTimeE: Default::default(),
        }
    }
}

pub type OPTIMIZATION_ATTRS = ClassAttributes;

/* TODO: create a backend and a simcode uniontype */
/// There are four kinds of statements:
///    1. assignments ('a := b;')
///    2. if statements ('if A then B; elseif C; else D;')
///    3. for loops ('for i in 1:10 loop ...; end for;')
///    4. when statements ('when E do S; end when;')
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Statement {
    STMT_ASSIGN {
        type_: metamodelica::Ref<Type>,
        exp1: metamodelica::Ref<Exp>,
        exp: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    STMT_TUPLE_ASSIGN {
        type_: metamodelica::Ref<Type>,
        expExpLst: metamodelica::List<metamodelica::Ref<Exp>>,
        exp: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    STMT_ASSIGN_ARR {
        type_: metamodelica::Ref<Type>,
        lhs: metamodelica::Ref<Exp>,
        exp: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    STMT_IF {
        exp: metamodelica::Ref<Exp>,
        statementLst: metamodelica::List<metamodelica::Ref<Statement>>,
        else_: metamodelica::Ref<Else>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    STMT_FOR {
        /// this is the type of the iterator
        type_: metamodelica::Ref<Type>,
        /// True if the iterator has an array type, otherwise false.
        iterIsArray: bool,
        /// the iterator variable
        iter: Ident,
        /// range for the loop
        range: metamodelica::Ref<Exp>,
        statementLst: metamodelica::List<metamodelica::Ref<Statement>>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
        /// sub-iterators for ARRAY iterator case (NBackEnd only)
        sub_iters: metamodelica::List<(
            metamodelica::Ref<ComponentRef>,
            metamodelica::Array<metamodelica::Ref<Exp>>,
        )>,
    },
    STMT_PARFOR {
        /// this is the type of the iterator
        type_: metamodelica::Ref<Type>,
        /// True if the iterator has an array type, otherwise false.
        iterIsArray: bool,
        /// the iterator variable
        iter: Ident,
        /// range for the loop
        range: metamodelica::Ref<Exp>,
        statementLst: metamodelica::List<metamodelica::Ref<Statement>>,
        /// list of parallel variables used/referenced in the parfor loop
        loopPrlVars: metamodelica::List<(metamodelica::Ref<ComponentRef>, SourceInfo)>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    STMT_WHILE {
        exp: metamodelica::Ref<Exp>,
        statementLst: metamodelica::List<metamodelica::Ref<Statement>>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    STMT_WHEN {
        exp: metamodelica::Ref<Exp>,
        conditions: metamodelica::List<metamodelica::Ref<ComponentRef>>,
        initialCall: bool,
        statementLst: metamodelica::List<metamodelica::Ref<Statement>>,
        elseWhen: Option<metamodelica::Ref<Statement>>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// assert(cond,msg)
    STMT_ASSERT {
        cond: metamodelica::Ref<Exp>,
        msg: metamodelica::Ref<Exp>,
        level: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// terminate(msg)
    STMT_TERMINATE {
        msg: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    STMT_REINIT {
        /// Variable
        var: metamodelica::Ref<Exp>,
        /// Value
        value: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// call with no return value, i.e. no equation.
    ///       Typically sideeffect call of external function.
    STMT_NORETCALL {
        exp: metamodelica::Ref<Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    STMT_RETURN {
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    STMT_BREAK {
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    STMT_CONTINUE {
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    /// For function initialization
    STMT_ARRAY_INIT {
        name: ArcStr,
        ty: metamodelica::Ref<Type>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
    STMT_FAILURE {
        body: metamodelica::List<metamodelica::Ref<Statement>>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<ElementSource>,
    },
}
impl metamodelica::gc::MMTrace for Statement {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Statement::STMT_ASSIGN {
                type_,
                exp1,
                exp,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_TUPLE_ASSIGN {
                type_,
                expExpLst,
                exp,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(expExpLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_ASSIGN_ARR {
                type_,
                lhs,
                exp,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_IF {
                exp,
                statementLst,
                else_,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(statementLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(else_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_FOR {
                type_,
                iterIsArray,
                iter,
                range,
                statementLst,
                source,
                sub_iters,
            } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iterIsArray, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iter, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(range, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(statementLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(sub_iters, __mmv)?;
                Ok(())
            }
            Statement::STMT_PARFOR {
                type_,
                iterIsArray,
                iter,
                range,
                statementLst,
                loopPrlVars,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iterIsArray, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iter, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(range, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(statementLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(loopPrlVars, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_WHILE {
                exp,
                statementLst,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(statementLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_WHEN {
                exp,
                conditions,
                initialCall,
                statementLst,
                elseWhen,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(conditions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(initialCall, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(statementLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elseWhen, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_ASSERT {
                cond,
                msg,
                level,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(cond, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(msg, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(level, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_TERMINATE { msg, source } => {
                metamodelica::gc::MMTrace::mm_accept(msg, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_REINIT { var, value, source } => {
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_NORETCALL { exp, source } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_RETURN { source } => {
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_BREAK { source } => {
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_CONTINUE { source } => {
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_ARRAY_INIT { name, ty, source } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Statement::STMT_FAILURE { body, source } => {
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Statement {
    fn default() -> Self {
        Self::STMT_RETURN {
            source: Default::default(),
        }
    }
}
pub use self::Statement::{
    STMT_ARRAY_INIT, STMT_ASSERT, STMT_ASSIGN, STMT_ASSIGN_ARR, STMT_BREAK, STMT_CONTINUE, STMT_FAILURE, STMT_FOR,
    STMT_IF, STMT_NORETCALL, STMT_PARFOR, STMT_REINIT, STMT_RETURN, STMT_TERMINATE, STMT_TUPLE_ASSIGN, STMT_WHEN,
    STMT_WHILE,
};

/// An if statements can one or more `elseif\' branches and an
///    optional `else\' branch.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Else {
    NOELSE,
    ELSEIF {
        exp: metamodelica::Ref<Exp>,
        statementLst: metamodelica::List<metamodelica::Ref<Statement>>,
        else_: metamodelica::Ref<Else>,
    },
    ELSE {
        statementLst: metamodelica::List<metamodelica::Ref<Statement>>,
    },
}
impl metamodelica::gc::MMTrace for Else {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Else::NOELSE => Ok(()),
            Else::ELSEIF {
                exp,
                statementLst,
                else_,
            } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(statementLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(else_, __mmv)?;
                Ok(())
            }
            Else::ELSE { statementLst } => {
                metamodelica::gc::MMTrace::mm_accept(statementLst, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Else {
    pub fn interned_NOELSE() -> metamodelica::Ref<Else> {
        thread_local! {
            static INTERNED: metamodelica::Ref<Else> = metamodelica::Ref::new(Else::NOELSE);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_NOELSE() -> metamodelica::Ref<Else> {
    Else::interned_NOELSE()
}
impl Default for Else {
    fn default() -> Self {
        Self::NOELSE
    }
}
pub use self::Else::{ELSE, ELSEIF, NOELSE};

/* -- End Algorithm.mo -- */
/* -- Start Types.mo -- */
/// - Variables
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Var {
    /// name
    pub name: Ident,
    /// attributes
    pub attributes: metamodelica::Ref<Attributes>,
    /// type
    pub ty: metamodelica::Ref<Type>,
    /// equation modification
    pub binding: metamodelica::Ref<Binding>,
    /// true if the binding has come from out of scope. This happens for derived record classes.
    ///                                   e.g. record A = B(k=exp). here the modification 'exp' is a binding from outside. We need
    ///                                   this infor to correctly generate default constructors at codegen time. This binding exp
    ///                                   will have to be supplied from outside for a default constructor of the owner record type
    pub bind_from_outside: bool,
    /// the constant-ness of the range if this is a for iterator, NONE() if is NOT a for iterator
    pub constOfForIteratorRange: Option<Const>,
}

impl metamodelica::gc::MMTrace for Var {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.attributes, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ty, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.binding, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.bind_from_outside, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.constOfForIteratorRange, __mmv)?;
        Ok(())
    }
}
impl Default for Var {
    fn default() -> Self {
        Self {
            name: Default::default(),
            attributes: Default::default(),
            ty: Default::default(),
            binding: Default::default(),
            bind_from_outside: Default::default(),
            constOfForIteratorRange: Default::default(),
        }
    }
}

pub type TYPES_VAR = Var;

/// - Attributes
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Attributes {
    /// flow, stream or unspecified
    pub connectorType: metamodelica::Ref<ConnectorType>,
    /// parallelism
    pub parallelism: SCode::Parallelism,
    /// variability
    pub variability: SCode::Variability,
    /// direction
    pub direction: Absyn::Direction,
    /// inner, outer,  inner outer or unspecified
    pub innerOuter: Absyn::InnerOuter,
    /// public, protected
    pub visibility: SCode::Visibility,
}

impl metamodelica::gc::MMTrace for Attributes {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.connectorType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.parallelism, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.variability, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.direction, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.innerOuter, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.visibility, __mmv)?;
        Ok(())
    }
}
impl Default for Attributes {
    fn default() -> Self {
        Self {
            connectorType: Default::default(),
            parallelism: Default::default(),
            variability: Default::default(),
            direction: Default::default(),
            innerOuter: Default::default(),
            visibility: Default::default(),
        }
    }
}

pub type ATTR = Attributes;

thread_local! { static __dummyAttrVar_TLS: metamodelica::Ref<Attributes> = metamodelica::Ref::new(Attributes { connectorType: crate::DAE::ConnectorType::interned_NON_CONNECTOR(), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }); }
pub fn dummyAttrVar() -> metamodelica::Ref<Attributes> {
    __dummyAttrVar_TLS.with(|__t| __t.clone())
}

thread_local! { static __dummyAttrParam_TLS: metamodelica::Ref<Attributes> = metamodelica::Ref::new(Attributes { connectorType: crate::DAE::ConnectorType::interned_NON_CONNECTOR(), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }); }
pub fn dummyAttrParam() -> metamodelica::Ref<Attributes> {
    __dummyAttrParam_TLS.with(|__t| __t.clone())
}

thread_local! { static __dummyAttrConst_TLS: metamodelica::Ref<Attributes> = metamodelica::Ref::new(Attributes { connectorType: crate::DAE::ConnectorType::interned_NON_CONNECTOR(), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::CONST, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }); }
pub fn dummyAttrConst() -> metamodelica::Ref<Attributes> {
    __dummyAttrConst_TLS.with(|__t| __t.clone())
}

thread_local! { static __dummyAttrInput_TLS: metamodelica::Ref<Attributes> = metamodelica::Ref::new(Attributes { connectorType: crate::DAE::ConnectorType::interned_NON_CONNECTOR(), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::INPUT, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }); }
pub fn dummyAttrInput() -> metamodelica::Ref<Attributes> {
    __dummyAttrInput_TLS.with(|__t| __t.clone())
}

/// where this binding came from: either default binding or start value
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum BindingSource {
    /// the binding came from the default value
    BINDING_FROM_DEFAULT_VALUE,
    /// the binding came from the start value
    BINDING_FROM_START_VALUE,
    /// the EQ binding is created from the submods of a record VARIABLE declration e.g. 'R r(i=2)' tranformed by instantiation to 'R r = R(i=2)'
    BINDING_FROM_RECORD_SUBMODS,
    /// the binding is created from the submods of a DERIVED record DECLARATION e.g. 'record K = R(i=3)'
    BINDING_FROM_DERIVED_RECORD_DECL,
}
impl metamodelica::gc::MMTrace for BindingSource {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            BindingSource::BINDING_FROM_DEFAULT_VALUE => Ok(()),
            BindingSource::BINDING_FROM_START_VALUE => Ok(()),
            BindingSource::BINDING_FROM_RECORD_SUBMODS => Ok(()),
            BindingSource::BINDING_FROM_DERIVED_RECORD_DECL => Ok(()),
        }
    }
}
pub use self::BindingSource::{
    BINDING_FROM_DEFAULT_VALUE, BINDING_FROM_DERIVED_RECORD_DECL, BINDING_FROM_RECORD_SUBMODS, BINDING_FROM_START_VALUE,
};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Binding {
    UNBOUND,
    EQBOUND {
        exp: metamodelica::Ref<Exp>,
        evaluatedExp: Option<metamodelica::Ref<Values::Value>>,
        constant_: Const,
        source: BindingSource,
    },
    VALBOUND {
        valBound: metamodelica::Ref<Values::Value>,
        source: BindingSource,
    },
}
impl metamodelica::gc::MMTrace for Binding {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Binding::UNBOUND => Ok(()),
            Binding::EQBOUND {
                exp,
                evaluatedExp,
                constant_,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(evaluatedExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(constant_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            Binding::VALBOUND { valBound, source } => {
                metamodelica::gc::MMTrace::mm_accept(valBound, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Binding {
    pub fn interned_UNBOUND() -> metamodelica::Ref<Binding> {
        thread_local! {
            static INTERNED: metamodelica::Ref<Binding> = metamodelica::Ref::new(Binding::UNBOUND);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_UNBOUND() -> metamodelica::Ref<Binding> {
    Binding::interned_UNBOUND()
}
impl Default for Binding {
    fn default() -> Self {
        Self::UNBOUND
    }
}
pub use self::Binding::{EQBOUND, UNBOUND, VALBOUND};

/// contains the path to the equalityConstraint function,
///   the dimension of the output and the inline type of the function
pub type EqualityConstraint = Option<(metamodelica::Ref<Absyn::Path>, i32, InlineType)>;

// default constants that can be used
thread_local! { static __T_REAL_DEFAULT_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_REAL { varLst: metamodelica::nil() }); }
pub fn T_REAL_DEFAULT() -> metamodelica::Ref<Type> {
    __T_REAL_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_INTEGER_DEFAULT_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_INTEGER { varLst: metamodelica::nil() }); }
pub fn T_INTEGER_DEFAULT() -> metamodelica::Ref<Type> {
    __T_INTEGER_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_STRING_DEFAULT_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_STRING { varLst: metamodelica::nil() }); }
pub fn T_STRING_DEFAULT() -> metamodelica::Ref<Type> {
    __T_STRING_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_BOOL_DEFAULT_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_BOOL { varLst: metamodelica::nil() }); }
pub fn T_BOOL_DEFAULT() -> metamodelica::Ref<Type> {
    __T_BOOL_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_CLOCK_DEFAULT_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_CLOCK { varLst: metamodelica::nil() }); }
pub fn T_CLOCK_DEFAULT() -> metamodelica::Ref<Type> {
    __T_CLOCK_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_ENUMERATION_DEFAULT_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_ENUMERATION { index: None, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }); }
pub fn T_ENUMERATION_DEFAULT() -> metamodelica::Ref<Type> {
    __T_ENUMERATION_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_REAL_BOXED_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_METABOXED { ty: T_REAL_DEFAULT().clone() }); }
pub(crate) fn T_REAL_BOXED() -> metamodelica::Ref<Type> {
    __T_REAL_BOXED_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_INTEGER_BOXED_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_METABOXED { ty: T_INTEGER_DEFAULT().clone() }); }
pub(crate) fn T_INTEGER_BOXED() -> metamodelica::Ref<Type> {
    __T_INTEGER_BOXED_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_STRING_BOXED_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_METABOXED { ty: T_STRING_DEFAULT().clone() }); }
pub(crate) fn T_STRING_BOXED() -> metamodelica::Ref<Type> {
    __T_STRING_BOXED_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_BOOL_BOXED_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_METABOXED { ty: T_BOOL_DEFAULT().clone() }); }
pub(crate) fn T_BOOL_BOXED() -> metamodelica::Ref<Type> {
    __T_BOOL_BOXED_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_METABOXED_DEFAULT_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_METABOXED { ty: T_UNKNOWN_DEFAULT().clone() }); }
pub fn T_METABOXED_DEFAULT() -> metamodelica::Ref<Type> {
    __T_METABOXED_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_METALIST_DEFAULT_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_METALIST { ty: T_UNKNOWN_DEFAULT().clone() }); }
pub fn T_METALIST_DEFAULT() -> metamodelica::Ref<Type> {
    __T_METALIST_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_NONE_DEFAULT_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_METAOPTION { ty: T_UNKNOWN_DEFAULT().clone() }); }
pub fn T_NONE_DEFAULT() -> metamodelica::Ref<Type> {
    __T_NONE_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_ANYTYPE_DEFAULT_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_ANYTYPE { anyClassType: None }); }
pub fn T_ANYTYPE_DEFAULT() -> metamodelica::Ref<Type> {
    __T_ANYTYPE_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_UNKNOWN_DEFAULT_TLS: metamodelica::Ref<Type> = crate::DAE::Type::interned_T_UNKNOWN(); }
pub fn T_UNKNOWN_DEFAULT() -> metamodelica::Ref<Type> {
    __T_UNKNOWN_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_NORETCALL_DEFAULT_TLS: metamodelica::Ref<Type> = crate::DAE::Type::interned_T_NORETCALL(); }
pub fn T_NORETCALL_DEFAULT() -> metamodelica::Ref<Type> {
    __T_NORETCALL_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_METATYPE_DEFAULT_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_METATYPE { ty: T_UNKNOWN_DEFAULT().clone() }); }
pub fn T_METATYPE_DEFAULT() -> metamodelica::Ref<Type> {
    __T_METATYPE_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_COMPLEX_DEFAULT_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_COMPLEX { complexClassType: ClassInf::State::UNKNOWN { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }); }
pub fn T_COMPLEX_DEFAULT() -> metamodelica::Ref<Type> {
    __T_COMPLEX_DEFAULT_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_COMPLEX_DEFAULT_RECORD_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }); }
pub fn T_COMPLEX_DEFAULT_RECORD() -> metamodelica::Ref<Type> {
    __T_COMPLEX_DEFAULT_RECORD_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_SOURCEINFO_DEFAULT_METARECORD_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_METARECORD { path: metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: literal!("SourceInfo"), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("SOURCEINFO") }) }), utPath: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("SourceInfo") }), typeVars: metamodelica::nil(), index: 1, fields: list![metamodelica::Ref::new(Var { name: literal!("fileName"), attributes: dummyAttrVar().clone(), ty: T_STRING_DEFAULT().clone(), binding: crate::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(Var { name: literal!("isReadOnly"), attributes: dummyAttrVar().clone(), ty: T_BOOL_DEFAULT().clone(), binding: crate::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(Var { name: literal!("lineNumberStart"), attributes: dummyAttrVar().clone(), ty: T_INTEGER_DEFAULT().clone(), binding: crate::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(Var { name: literal!("columnNumberStart"), attributes: dummyAttrVar().clone(), ty: T_INTEGER_DEFAULT().clone(), binding: crate::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(Var { name: literal!("lineNumberEnd"), attributes: dummyAttrVar().clone(), ty: T_INTEGER_DEFAULT().clone(), binding: crate::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(Var { name: literal!("columnNumberEnd"), attributes: dummyAttrVar().clone(), ty: T_INTEGER_DEFAULT().clone(), binding: crate::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(Var { name: literal!("lastModification"), attributes: dummyAttrVar().clone(), ty: T_REAL_DEFAULT().clone(), binding: crate::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None })], knownSingleton: true }); }
pub(crate) fn T_SOURCEINFO_DEFAULT_METARECORD() -> metamodelica::Ref<Type> {
    __T_SOURCEINFO_DEFAULT_METARECORD_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_SOURCEINFO_DEFAULT_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_METAUNIONTYPE { paths: list![metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: literal!("SourceInfo"), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("SOURCEINFO") }) })], typeVars: metamodelica::nil(), knownSingleton: true, singletonType: metamodelica::Ref::new(EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: T_SOURCEINFO_DEFAULT_METARECORD().clone() }), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("SourceInfo") }) }); }
pub fn T_SOURCEINFO_DEFAULT() -> metamodelica::Ref<Type> {
    __T_SOURCEINFO_DEFAULT_TLS.with(|__t| __t.clone())
}

// Arrays of unknown dimension, eg. Real[:]
thread_local! { static __T_ARRAY_REAL_NODIM_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_ARRAY { ty: T_REAL_DEFAULT().clone(), dims: list![crate::DAE::Dimension::interned_DIM_UNKNOWN()] }); }
pub fn T_ARRAY_REAL_NODIM() -> metamodelica::Ref<Type> {
    __T_ARRAY_REAL_NODIM_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_ARRAY_INT_NODIM_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_ARRAY { ty: T_INTEGER_DEFAULT().clone(), dims: list![crate::DAE::Dimension::interned_DIM_UNKNOWN()] }); }
pub(crate) fn T_ARRAY_INT_NODIM() -> metamodelica::Ref<Type> {
    __T_ARRAY_INT_NODIM_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_ARRAY_BOOL_NODIM_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_ARRAY { ty: T_BOOL_DEFAULT().clone(), dims: list![crate::DAE::Dimension::interned_DIM_UNKNOWN()] }); }
pub(crate) fn T_ARRAY_BOOL_NODIM() -> metamodelica::Ref<Type> {
    __T_ARRAY_BOOL_NODIM_TLS.with(|__t| __t.clone())
}

thread_local! { static __T_ARRAY_STRING_NODIM_TLS: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_ARRAY { ty: T_STRING_DEFAULT().clone(), dims: list![crate::DAE::Dimension::interned_DIM_UNKNOWN()] }); }
pub fn T_ARRAY_STRING_NODIM() -> metamodelica::Ref<Type> {
    __T_ARRAY_STRING_NODIM_TLS.with(|__t| __t.clone())
}

/// models the different front-end and back-end types
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Type {
    T_INTEGER {
        varLst: metamodelica::List<metamodelica::Ref<Var>>,
    },
    T_REAL {
        varLst: metamodelica::List<metamodelica::Ref<Var>>,
    },
    T_STRING {
        varLst: metamodelica::List<metamodelica::Ref<Var>>,
    },
    T_BOOL {
        varLst: metamodelica::List<metamodelica::Ref<Var>>,
    },
    T_CLOCK {
        varLst: metamodelica::List<metamodelica::Ref<Var>>,
    },
    /// If the list of names is empty, this is the super-enumeration that is the super-class of all enumerations
    T_ENUMERATION {
        /// the enumeration value index, SOME for element, NONE() for type
        index: Option<i32>,
        /// enumeration path
        path: metamodelica::Ref<Absyn::Path>,
        /// names
        names: metamodelica::List<ArcStr>,
        literalVarLst: metamodelica::List<metamodelica::Ref<Var>>,
        attributeLst: metamodelica::List<metamodelica::Ref<Var>>,
    },
    /// an array can be represented in two equivalent ways:
    ///       1. T_ARRAY(non_array_type, {dim1, dim2, dim3})
    ///       2. T_ARRAY(T_ARRAY(T_ARRAY(non_array_type, {dim1}), {dim2}), {dim3})
    ///       In general Inst generates 1 and all the others generates 2
    T_ARRAY {
        /// Type
        ty: metamodelica::Ref<Type>,
        /// dims
        dims: Dimensions,
    },
    /// For functions not returning any values.
    T_NORETCALL,
    /// Used when type is not yet determined
    T_UNKNOWN,
    T_COMPLEX {
        /// The type of a class
        complexClassType: ClassInf::State,
        /// The variables of a complex type
        varLst: metamodelica::List<metamodelica::Ref<Var>>,
        equalityConstraint: EqualityConstraint,
        /// If the record is passed to an external function at any point, we need to generate conversion functions for it (for instance to convert 'modelica_integer' to 'int')
        usedExternally: bool,
    },
    T_SUBTYPE_BASIC {
        /// The type of a class
        complexClassType: ClassInf::State,
        /// complexVarLst; The variables of a complex type! Should be empty, kept here to verify!
        varLst: metamodelica::List<metamodelica::Ref<Var>>,
        /// complexType; A complex type can be a subtype of another (primitive) type (through extends)
        complexType: metamodelica::Ref<Type>,
        equalityConstraint: EqualityConstraint,
    },
    T_FUNCTION {
        /// funcArg
        funcArg: metamodelica::List<metamodelica::Ref<FuncArg>>,
        /// Only single-result
        funcResultType: metamodelica::Ref<Type>,
        functionAttributes: FunctionAttributes,
        path: metamodelica::Ref<Absyn::Path>,
    },
    /// MetaModelica Function Reference that is a variable
    T_FUNCTION_REFERENCE_VAR {
        /// the type of the function
        functionType: metamodelica::Ref<Type>,
    },
    /// MetaModelica Function Reference that is a direct reference to a function
    T_FUNCTION_REFERENCE_FUNC {
        builtin: bool,
        /// type of the non-boxptr function
        functionType: metamodelica::Ref<Type>,
    },
    T_TUPLE {
        /// For functions returning multiple values.
        types: metamodelica::List<metamodelica::Ref<Type>>,
        /// For tuples elements that have names (function outputs)
        names: Option<metamodelica::List<ArcStr>>,
    },
    T_CODE {
        ty: CodeType,
    },
    T_ANYTYPE {
        /// anyClassType - used for generic types. When class state present the type is assumed to be a complex type which has that restriction.
        anyClassType: Option<ClassInf::State>,
    },
    /// MetaModelica list type
    T_METALIST {
        /// listType
        ty: metamodelica::Ref<Type>,
    },
    /// MetaModelica tuple type
    T_METATUPLE {
        types: metamodelica::List<metamodelica::Ref<Type>>,
    },
    /// MetaModelica option type
    T_METAOPTION {
        ty: metamodelica::Ref<Type>,
    },
    /// MetaModelica Uniontype, added by simbj
    T_METAUNIONTYPE {
        paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
        typeVars: metamodelica::List<metamodelica::Ref<Type>>,
        /// The runtime system (dynload), does not know if the value is a singleton. But optimizations are safe if this is true.
        knownSingleton: bool,
        singletonType: metamodelica::Ref<EvaluateSingletonType>,
        path: metamodelica::Ref<Absyn::Path>,
    },
    /// MetaModelica Record, used by Uniontypes. added by simbj
    T_METARECORD {
        /// the path to the record
        path: metamodelica::Ref<Absyn::Path>,
        /// the path to its uniontype; this is what we match the type against
        utPath: metamodelica::Ref<Absyn::Path>,
        typeVars: metamodelica::List<metamodelica::Ref<Type>>,
        index: i32,
        fields: metamodelica::List<metamodelica::Ref<Var>>,
        /// The runtime system (dynload), does not know if the value is a singleton. But optimizations are safe if this is true.
        knownSingleton: bool,
    },
    T_METAARRAY {
        ty: metamodelica::Ref<Type>,
    },
    /// Used for MetaModelica generic types
    T_METABOXED {
        ty: metamodelica::Ref<Type>,
    },
    T_METAPOLYMORPHIC {
        name: ArcStr,
    },
    /// this type contains all the meta types
    T_METATYPE {
        ty: metamodelica::Ref<Type>,
    },
}
impl metamodelica::gc::MMTrace for Type {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Type::T_INTEGER { varLst } => {
                metamodelica::gc::MMTrace::mm_accept(varLst, __mmv)?;
                Ok(())
            }
            Type::T_REAL { varLst } => {
                metamodelica::gc::MMTrace::mm_accept(varLst, __mmv)?;
                Ok(())
            }
            Type::T_STRING { varLst } => {
                metamodelica::gc::MMTrace::mm_accept(varLst, __mmv)?;
                Ok(())
            }
            Type::T_BOOL { varLst } => {
                metamodelica::gc::MMTrace::mm_accept(varLst, __mmv)?;
                Ok(())
            }
            Type::T_CLOCK { varLst } => {
                metamodelica::gc::MMTrace::mm_accept(varLst, __mmv)?;
                Ok(())
            }
            Type::T_ENUMERATION {
                index,
                path,
                names,
                literalVarLst,
                attributeLst,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(names, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(literalVarLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attributeLst, __mmv)?;
                Ok(())
            }
            Type::T_ARRAY { ty, dims } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dims, __mmv)?;
                Ok(())
            }
            Type::T_NORETCALL => Ok(()),
            Type::T_UNKNOWN => Ok(()),
            Type::T_COMPLEX {
                complexClassType,
                varLst,
                equalityConstraint,
                usedExternally,
            } => {
                metamodelica::gc::MMTrace::mm_accept(complexClassType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(varLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equalityConstraint, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(usedExternally, __mmv)?;
                Ok(())
            }
            Type::T_SUBTYPE_BASIC {
                complexClassType,
                varLst,
                complexType,
                equalityConstraint,
            } => {
                metamodelica::gc::MMTrace::mm_accept(complexClassType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(varLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(complexType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equalityConstraint, __mmv)?;
                Ok(())
            }
            Type::T_FUNCTION {
                funcArg,
                funcResultType,
                functionAttributes,
                path,
            } => {
                metamodelica::gc::MMTrace::mm_accept(funcArg, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(funcResultType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(functionAttributes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            Type::T_FUNCTION_REFERENCE_VAR { functionType } => {
                metamodelica::gc::MMTrace::mm_accept(functionType, __mmv)?;
                Ok(())
            }
            Type::T_FUNCTION_REFERENCE_FUNC { builtin, functionType } => {
                metamodelica::gc::MMTrace::mm_accept(builtin, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(functionType, __mmv)?;
                Ok(())
            }
            Type::T_TUPLE { types, names } => {
                metamodelica::gc::MMTrace::mm_accept(types, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(names, __mmv)?;
                Ok(())
            }
            Type::T_CODE { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Type::T_ANYTYPE { anyClassType } => {
                metamodelica::gc::MMTrace::mm_accept(anyClassType, __mmv)?;
                Ok(())
            }
            Type::T_METALIST { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Type::T_METATUPLE { types } => {
                metamodelica::gc::MMTrace::mm_accept(types, __mmv)?;
                Ok(())
            }
            Type::T_METAOPTION { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Type::T_METAUNIONTYPE {
                paths,
                typeVars,
                knownSingleton,
                singletonType,
                path,
            } => {
                metamodelica::gc::MMTrace::mm_accept(paths, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(typeVars, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(knownSingleton, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(singletonType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            Type::T_METARECORD {
                path,
                utPath,
                typeVars,
                index,
                fields,
                knownSingleton,
            } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(utPath, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(typeVars, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fields, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(knownSingleton, __mmv)?;
                Ok(())
            }
            Type::T_METAARRAY { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Type::T_METABOXED { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Type::T_METAPOLYMORPHIC { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
            Type::T_METATYPE { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Type {
    pub fn interned_T_NORETCALL() -> metamodelica::Ref<Type> {
        thread_local! {
            static INTERNED: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_NORETCALL);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_T_UNKNOWN() -> metamodelica::Ref<Type> {
        thread_local! {
            static INTERNED: metamodelica::Ref<Type> = metamodelica::Ref::new(Type::T_UNKNOWN);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_T_NORETCALL() -> metamodelica::Ref<Type> {
    Type::interned_T_NORETCALL()
}
pub fn interned_T_UNKNOWN() -> metamodelica::Ref<Type> {
    Type::interned_T_UNKNOWN()
}
impl Default for Type {
    fn default() -> Self {
        Self::T_NORETCALL
    }
}
pub use self::Type::{
    T_ANYTYPE, T_ARRAY, T_BOOL, T_CLOCK, T_CODE, T_COMPLEX, T_ENUMERATION, T_FUNCTION, T_FUNCTION_REFERENCE_FUNC,
    T_FUNCTION_REFERENCE_VAR, T_INTEGER, T_METAARRAY, T_METABOXED, T_METALIST, T_METAOPTION, T_METAPOLYMORPHIC,
    T_METARECORD, T_METATUPLE, T_METATYPE, T_METAUNIONTYPE, T_NORETCALL, T_REAL, T_STRING, T_SUBTYPE_BASIC, T_TUPLE,
    T_UNKNOWN,
};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum CodeType {
    C_EXPRESSION,
    C_EXPRESSION_OR_MODIFICATION,
    C_MODIFICATION,
    C_TYPENAME,
    C_VARIABLENAME,
    /// Array of VariableName
    C_VARIABLENAMES,
}
impl metamodelica::gc::MMTrace for CodeType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            CodeType::C_EXPRESSION => Ok(()),
            CodeType::C_EXPRESSION_OR_MODIFICATION => Ok(()),
            CodeType::C_MODIFICATION => Ok(()),
            CodeType::C_TYPENAME => Ok(()),
            CodeType::C_VARIABLENAME => Ok(()),
            CodeType::C_VARIABLENAMES => Ok(()),
        }
    }
}
pub use self::CodeType::{
    C_EXPRESSION, C_EXPRESSION_OR_MODIFICATION, C_MODIFICATION, C_TYPENAME, C_VARIABLENAME, C_VARIABLENAMES,
};

/// Is here because constants are not allowed to contain function pointers for some reason
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum EvaluateSingletonType {
    EVAL_SINGLETON_TYPE_FUNCTION,
    EVAL_SINGLETON_KNOWN_TYPE { ty: metamodelica::Ref<Type> },
    NOT_SINGLETON,
}
impl metamodelica::gc::MMTrace for EvaluateSingletonType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            EvaluateSingletonType::EVAL_SINGLETON_TYPE_FUNCTION => Ok(()),
            EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            EvaluateSingletonType::NOT_SINGLETON => Ok(()),
        }
    }
}
impl EvaluateSingletonType {
    pub fn interned_EVAL_SINGLETON_TYPE_FUNCTION() -> metamodelica::Ref<EvaluateSingletonType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<EvaluateSingletonType> = metamodelica::Ref::new(EvaluateSingletonType::EVAL_SINGLETON_TYPE_FUNCTION);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_NOT_SINGLETON() -> metamodelica::Ref<EvaluateSingletonType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<EvaluateSingletonType> = metamodelica::Ref::new(EvaluateSingletonType::NOT_SINGLETON);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_EVAL_SINGLETON_TYPE_FUNCTION() -> metamodelica::Ref<EvaluateSingletonType> {
    EvaluateSingletonType::interned_EVAL_SINGLETON_TYPE_FUNCTION()
}
pub fn interned_NOT_SINGLETON() -> metamodelica::Ref<EvaluateSingletonType> {
    EvaluateSingletonType::interned_NOT_SINGLETON()
}
pub use self::EvaluateSingletonType::{EVAL_SINGLETON_KNOWN_TYPE, EVAL_SINGLETON_TYPE_FUNCTION, NOT_SINGLETON};

pub type EvaluateSingletonTypeFunction =
    std::sync::Arc<dyn ::std::ops::Fn() -> Result<metamodelica::Ref<Type>> + 'static>;

pub static FUNCTION_ATTRIBUTES_BUILTIN: std::sync::LazyLock<FunctionAttributes> =
    std::sync::LazyLock::new(|| FunctionAttributes {
        inline: crate::DAE::InlineType::NO_INLINE,
        generateEvents: false,
        purity: Purity::PURE.clone(),
        isFunctionPointer: false,
        isBuiltin: FunctionBuiltin::FUNCTION_BUILTIN {
            name: None,
            unboxArgs: false,
        },
        functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL,
        noReturn: NoReturn::RETURNS.clone(),
    });

pub static FUNCTION_ATTRIBUTES_DEFAULT: std::sync::LazyLock<FunctionAttributes> =
    std::sync::LazyLock::new(|| FunctionAttributes {
        inline: crate::DAE::InlineType::DEFAULT_INLINE,
        generateEvents: false,
        purity: Purity::PURE.clone(),
        isFunctionPointer: false,
        isBuiltin: crate::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN,
        functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL,
        noReturn: NoReturn::RETURNS.clone(),
    });

pub(crate) static FUNCTION_ATTRIBUTES_IMPURE: std::sync::LazyLock<FunctionAttributes> =
    std::sync::LazyLock::new(|| FunctionAttributes {
        inline: crate::DAE::InlineType::NO_INLINE,
        generateEvents: false,
        purity: Purity::IMPURE.clone(),
        isFunctionPointer: false,
        isBuiltin: crate::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN,
        functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL,
        noReturn: NoReturn::RETURNS.clone(),
    });

pub static FUNCTION_ATTRIBUTES_BUILTIN_IMPURE: std::sync::LazyLock<FunctionAttributes> =
    std::sync::LazyLock::new(|| FunctionAttributes {
        inline: crate::DAE::InlineType::NO_INLINE,
        generateEvents: false,
        purity: Purity::IMPURE.clone(),
        isFunctionPointer: false,
        isBuiltin: FunctionBuiltin::FUNCTION_BUILTIN {
            name: None,
            unboxArgs: false,
        },
        functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL,
        noReturn: NoReturn::RETURNS.clone(),
    });

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum Purity {
    PURE = 1,
    IMPURE = 2,
    UNDEFINED = 3,
    OM_IMPURE = 4,
}
impl PartialOrd for Purity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Purity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Purity {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for Purity {
    fn default() -> Self {
        Self::PURE
    }
}

// Function with pure prefix
// Function with impure prefix
// Function with neither pure nor impure prefix
// Function with __OpenModelica_Impure=true annotation (only used by the OF)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum NoReturn {
    RETURNS = 1,
    NORETURN = 2,
}
impl PartialOrd for NoReturn {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for NoReturn {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for NoReturn {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for NoReturn {
    fn default() -> Self {
        Self::RETURNS
    }
}

// The function may return normally to its caller
// The function never returns normally (always fails, e.g. via fail(), assert(false) or terminate)
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FunctionAttributes {
    pub inline: InlineType,
    pub generateEvents: bool,
    pub purity: Purity,
    /// if the function is a local variable
    pub isFunctionPointer: bool,
    pub isBuiltin: FunctionBuiltin,
    pub functionParallelism: FunctionParallelism,
    /// whether the function ever returns normally
    pub noReturn: NoReturn,
}

impl metamodelica::gc::MMTrace for FunctionAttributes {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.inline, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.generateEvents, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.purity, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isFunctionPointer, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isBuiltin, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.functionParallelism, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.noReturn, __mmv)?;
        Ok(())
    }
}
impl Default for FunctionAttributes {
    fn default() -> Self {
        Self {
            inline: Default::default(),
            generateEvents: Default::default(),
            purity: Default::default(),
            isFunctionPointer: Default::default(),
            isBuiltin: Default::default(),
            functionParallelism: Default::default(),
            noReturn: Default::default(),
        }
    }
}

pub type FUNCTION_ATTRIBUTES = FunctionAttributes;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum FunctionBuiltin {
    /// Function is not builtin
    FUNCTION_NOT_BUILTIN,
    /// Function is builtin
    FUNCTION_BUILTIN { name: Option<ArcStr>, unboxArgs: bool },
    /// The function has a body, but its function pointer is builtin. This means inline code+optimized pointer if need be.
    FUNCTION_BUILTIN_PTR,
}
impl metamodelica::gc::MMTrace for FunctionBuiltin {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            FunctionBuiltin::FUNCTION_NOT_BUILTIN => Ok(()),
            FunctionBuiltin::FUNCTION_BUILTIN { name, unboxArgs } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(unboxArgs, __mmv)?;
                Ok(())
            }
            FunctionBuiltin::FUNCTION_BUILTIN_PTR => Ok(()),
        }
    }
}
impl Default for FunctionBuiltin {
    fn default() -> Self {
        Self::FUNCTION_NOT_BUILTIN
    }
}
pub use self::FunctionBuiltin::{FUNCTION_BUILTIN, FUNCTION_BUILTIN_PTR, FUNCTION_NOT_BUILTIN};

//This was a function restriction in SCode and Absyn
//Now it is part of function attributes.
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum FunctionParallelism {
    /// a normal function i.e non_parallel
    FP_NON_PARALLEL,
    /// an OpenCL/CUDA parallel/device function
    FP_PARALLEL_FUNCTION,
    /// an OpenCL/CUDA kernel function
    FP_KERNEL_FUNCTION,
}
impl metamodelica::gc::MMTrace for FunctionParallelism {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            FunctionParallelism::FP_NON_PARALLEL => Ok(()),
            FunctionParallelism::FP_PARALLEL_FUNCTION => Ok(()),
            FunctionParallelism::FP_KERNEL_FUNCTION => Ok(()),
        }
    }
}
impl Default for FunctionParallelism {
    fn default() -> Self {
        Self::FP_NON_PARALLEL
    }
}
pub use self::FunctionParallelism::{FP_KERNEL_FUNCTION, FP_NON_PARALLEL, FP_PARALLEL_FUNCTION};

/// a list of dimensions
pub type Dimensions = metamodelica::List<metamodelica::Ref<Dimension>>;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Dimension {
    /// Dimension given by an integer.
    DIM_INTEGER { integer: i32 },
    /// Dimension given by Boolean
    DIM_BOOLEAN,
    /// Dimension given by an enumeration.
    DIM_ENUM {
        /// The enumeration type name.
        enumTypeName: metamodelica::Ref<Absyn::Path>,
        /// A list of the literals in the enumeration.
        literals: metamodelica::List<ArcStr>,
        /// The size of the enumeration.
        size: i32,
    },
    /// Dimension given by an expression.
    DIM_EXP { exp: metamodelica::Ref<Exp> },
    /// Dimension with unknown size.
    DIM_UNKNOWN,
}
impl metamodelica::gc::MMTrace for Dimension {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Dimension::DIM_INTEGER { integer } => {
                metamodelica::gc::MMTrace::mm_accept(integer, __mmv)?;
                Ok(())
            }
            Dimension::DIM_BOOLEAN => Ok(()),
            Dimension::DIM_ENUM {
                enumTypeName,
                literals,
                size,
            } => {
                metamodelica::gc::MMTrace::mm_accept(enumTypeName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(literals, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                Ok(())
            }
            Dimension::DIM_EXP { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            Dimension::DIM_UNKNOWN => Ok(()),
        }
    }
}
impl Dimension {
    pub fn interned_DIM_BOOLEAN() -> metamodelica::Ref<Dimension> {
        thread_local! {
            static INTERNED: metamodelica::Ref<Dimension> = metamodelica::Ref::new(Dimension::DIM_BOOLEAN);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_DIM_UNKNOWN() -> metamodelica::Ref<Dimension> {
        thread_local! {
            static INTERNED: metamodelica::Ref<Dimension> = metamodelica::Ref::new(Dimension::DIM_UNKNOWN);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_DIM_BOOLEAN() -> metamodelica::Ref<Dimension> {
    Dimension::interned_DIM_BOOLEAN()
}
pub fn interned_DIM_UNKNOWN() -> metamodelica::Ref<Dimension> {
    Dimension::interned_DIM_UNKNOWN()
}
impl Default for Dimension {
    fn default() -> Self {
        Self::DIM_BOOLEAN
    }
}
pub use self::Dimension::{DIM_BOOLEAN, DIM_ENUM, DIM_EXP, DIM_INTEGER, DIM_UNKNOWN};

// adrpo: this is used to bind unknown dimensions to an expression
//        and when we do subtyping we add constrains to this expression.
//        this should be used for typechecking with unknown dimensions
//        when running checkModel. the binding acts like a type variable.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum DimensionBinding {
    /// dimension is not bound
    DIM_UNBOUND,
    /// dimension is bound to an expression with constrains
    DIM_BOUND {
        /// the dimension is bound to this expression
        binding: metamodelica::Ref<Exp>,
        /// the bound has these constrains (collected when doing subtyping)
        constrains: Dimensions,
    },
}
impl metamodelica::gc::MMTrace for DimensionBinding {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            DimensionBinding::DIM_UNBOUND => Ok(()),
            DimensionBinding::DIM_BOUND { binding, constrains } => {
                metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(constrains, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::DimensionBinding::{DIM_BOUND, DIM_UNBOUND};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FuncArg {
    pub name: ArcStr,
    pub ty: metamodelica::Ref<Type>,
    pub r#const: Const,
    pub par: VarParallelism,
    pub defaultBinding: Option<metamodelica::Ref<Exp>>,
}

impl metamodelica::gc::MMTrace for FuncArg {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ty, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.r#const, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.par, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.defaultBinding, __mmv)?;
        Ok(())
    }
}
impl Default for FuncArg {
    fn default() -> Self {
        Self {
            name: Default::default(),
            ty: Default::default(),
            r#const: Default::default(),
            par: Default::default(),
            defaultBinding: Default::default(),
        }
    }
}

pub type FUNCARG = FuncArg;

/// The degree of constantness of an expression is determined by the Const
///    datatype. Variables declared as \'constant\' will get C_CONST constantness.
///    Variables declared as \'parameter\' will get C_PARAM constantness and
///    all other variables are not constant and will get C_VAR constantness.
///
///  - Variable properties
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Const {
    /// constant
    C_CONST,
    /// parameter
    C_PARAM,
    /// continuous
    C_VAR,
    C_UNKNOWN,
}
impl metamodelica::gc::MMTrace for Const {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Const::C_CONST => Ok(()),
            Const::C_PARAM => Ok(()),
            Const::C_VAR => Ok(()),
            Const::C_UNKNOWN => Ok(()),
        }
    }
}
impl Default for Const {
    fn default() -> Self {
        Self::C_CONST
    }
}
pub use self::Const::{C_CONST, C_PARAM, C_UNKNOWN, C_VAR};

/// A tuple is added to the Types. This is used by functions whom returns multiple arguments.
///  Used by split_props
///  - Tuple constants
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum TupleConst {
    SINGLE_CONST {
        r#const: Const,
    },
    TUPLE_CONST {
        tupleConstLst: metamodelica::List<metamodelica::Ref<TupleConst>>,
    },
}
impl metamodelica::gc::MMTrace for TupleConst {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            TupleConst::SINGLE_CONST { r#const } => {
                metamodelica::gc::MMTrace::mm_accept(r#const, __mmv)?;
                Ok(())
            }
            TupleConst::TUPLE_CONST { tupleConstLst } => {
                metamodelica::gc::MMTrace::mm_accept(tupleConstLst, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for TupleConst {
    fn default() -> Self {
        Self::SINGLE_CONST {
            r#const: Default::default(),
        }
    }
}
pub use self::TupleConst::{SINGLE_CONST, TUPLE_CONST};

/// P.R 1.1 for multiple return arguments from functions,
///    one constant flag for each return argument.
///
///  The datatype `Properties\' contain information about an
///    expression.  The properties are created by analyzing the
///    expressions.
///  - Expression properties
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Properties {
    PROP {
        /// type
        type_: metamodelica::Ref<Type>,
        /// constFlag; if the type is a tuple, each element
        ///                  have a const flag.
        constFlag: Const,
    },
    PROP_TUPLE {
        type_: metamodelica::Ref<Type>,
        /// tupleConst; The elements might be
        ///                  tuple themselfs.
        tupleConst: metamodelica::Ref<TupleConst>,
    },
}
impl metamodelica::gc::MMTrace for Properties {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Properties::PROP { type_, constFlag } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(constFlag, __mmv)?;
                Ok(())
            }
            Properties::PROP_TUPLE { type_, tupleConst } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(tupleConst, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Properties {
    fn default() -> Self {
        Self::PROP {
            type_: Default::default(),
            constFlag: Default::default(),
        }
    }
}
pub use self::Properties::{PROP, PROP_TUPLE};

/// To generate the correct set of equations, the translator has to
///  differentiate between the primitive types `Real\', `Integer\',
///  `String\', `Boolean\' and types directly derived from then from
///  other, complex types.  For arrays and matrices the type
///  `T_ARRAY\' is used, with the first argument being the number of
///  dimensions, and the second being the type of the objects in the
///  array.  The `Type\' type is used to store
///  information about whether a class is derived from a primitive
///  type, and whether a variable is of one of these types.
///  - Modification datatype, was originally in Mod
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum EqMod {
    TYPED {
        /// modifier as expression
        modifierAsExp: metamodelica::Ref<Exp>,
        /// modifier as Value option
        modifierAsValue: Option<metamodelica::Ref<Values::Value>>,
        /// properties
        properties: Properties,
        /// keep the untyped modifier as an absyn expression for modification comparison
        modifierAsAbsynExp: metamodelica::Ref<Absyn::Exp>,
        info: SourceInfo,
    },
    UNTYPED {
        exp: metamodelica::Ref<Absyn::Exp>,
    },
}
impl metamodelica::gc::MMTrace for EqMod {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            EqMod::TYPED {
                modifierAsExp,
                modifierAsValue,
                properties,
                modifierAsAbsynExp,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(modifierAsExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modifierAsValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(properties, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modifierAsAbsynExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            EqMod::UNTYPED { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for EqMod {
    fn default() -> Self {
        Self::UNTYPED {
            exp: Default::default(),
        }
    }
}
pub use self::EqMod::{TYPED, UNTYPED};

/// -Sub Modification
/// named modification, i.e. (a = 5)
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SubMod {
    /// component name
    pub ident: Ident,
    /// modification
    pub r#mod: metamodelica::Ref<Mod>,
}

impl metamodelica::gc::MMTrace for SubMod {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.ident, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.r#mod, __mmv)?;
        Ok(())
    }
}
impl Default for SubMod {
    fn default() -> Self {
        Self {
            ident: Default::default(),
            r#mod: Default::default(),
        }
    }
}

pub type NAMEMOD = SubMod;

/// Modification
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Mod {
    MOD {
        /// final prefix
        finalPrefix: SCode::Final,
        /// each prefix
        eachPrefix: SCode::Each,
        subModLst: metamodelica::List<metamodelica::Ref<SubMod>>,
        binding: Option<EqMod>,
        info: SourceInfo,
    },
    REDECL {
        /// final prefix
        finalPrefix: SCode::Final,
        /// each prefix
        eachPrefix: SCode::Each,
        element: metamodelica::Ref<SCode::Element>,
        r#mod: metamodelica::Ref<Mod>,
    },
    NOMOD,
}
impl metamodelica::gc::MMTrace for Mod {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Mod::MOD {
                finalPrefix,
                eachPrefix,
                subModLst,
                binding,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eachPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(subModLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Mod::REDECL {
                finalPrefix,
                eachPrefix,
                element,
                r#mod,
            } => {
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eachPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(element, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(r#mod, __mmv)?;
                Ok(())
            }
            Mod::NOMOD => Ok(()),
        }
    }
}
impl Mod {
    pub fn interned_NOMOD() -> metamodelica::Ref<Mod> {
        thread_local! {
            static INTERNED: metamodelica::Ref<Mod> = metamodelica::Ref::new(Mod::NOMOD);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_NOMOD() -> metamodelica::Ref<Mod> {
    Mod::interned_NOMOD()
}
impl Default for Mod {
    fn default() -> Self {
        Self::NOMOD
    }
}
pub use self::Mod::{MOD, NOMOD, REDECL};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ClockKind {
    INFERRED_CLOCK,
    RATIONAL_CLOCK {
        /// integer type >= 0
        intervalCounter: metamodelica::Ref<Exp>,
        /// integer type >= 1, defaults to 1
        resolution: metamodelica::Ref<Exp>,
    },
    REAL_CLOCK {
        /// real type > 0
        interval: metamodelica::Ref<Exp>,
    },
    EVENT_CLOCK {
        condition: metamodelica::Ref<Exp>,
        /// real type >= 0.0
        startInterval: metamodelica::Ref<Exp>,
    },
    SOLVER_CLOCK {
        /// clock type
        c: metamodelica::Ref<Exp>,
        /// string type
        solverMethod: metamodelica::Ref<Exp>,
    },
}
impl metamodelica::gc::MMTrace for ClockKind {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ClockKind::INFERRED_CLOCK => Ok(()),
            ClockKind::RATIONAL_CLOCK {
                intervalCounter,
                resolution,
            } => {
                metamodelica::gc::MMTrace::mm_accept(intervalCounter, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(resolution, __mmv)?;
                Ok(())
            }
            ClockKind::REAL_CLOCK { interval } => {
                metamodelica::gc::MMTrace::mm_accept(interval, __mmv)?;
                Ok(())
            }
            ClockKind::EVENT_CLOCK {
                condition,
                startInterval,
            } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startInterval, __mmv)?;
                Ok(())
            }
            ClockKind::SOLVER_CLOCK { c, solverMethod } => {
                metamodelica::gc::MMTrace::mm_accept(c, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(solverMethod, __mmv)?;
                Ok(())
            }
        }
    }
}
impl ClockKind {
    pub fn interned_INFERRED_CLOCK() -> metamodelica::Ref<ClockKind> {
        thread_local! {
            static INTERNED: metamodelica::Ref<ClockKind> = metamodelica::Ref::new(ClockKind::INFERRED_CLOCK);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_INFERRED_CLOCK() -> metamodelica::Ref<ClockKind> {
    ClockKind::interned_INFERRED_CLOCK()
}
impl Default for ClockKind {
    fn default() -> Self {
        Self::INFERRED_CLOCK
    }
}
pub use self::ClockKind::{EVENT_CLOCK, INFERRED_CLOCK, RATIONAL_CLOCK, REAL_CLOCK, SOLVER_CLOCK};

/* -- End Types.mo -- */
/// Expressions
///  The 'Exp' datatype closely corresponds to the 'Absyn.Exp' datatype, but
///  is used for statically analyzed expressions. It includes explicit type
///  promotions and typed (non-overloaded) operators. It also contains expression
///  indexing with the 'ASUB' constructor. Indexing arbitrary array expressions
///  is currently not supported in Modelica, but it is needed here.
///
///  When making additions, update at least the following functions:
///  * Expression.traverseExp
///  * Expression.traverseExpTopDown
///  * Expression.traverseExpBiDir
///  * ExpressionBasics.printExpStr
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Exp {
    ICONST {
        /// Integer constants
        integer: i32,
    },
    RCONST {
        /// Real constants
        real: metamodelica::Real,
    },
    SCONST {
        /// String constants
        string: ArcStr,
    },
    BCONST {
        /// Bool constants
        bool: bool,
    },
    /// Clock constructors
    CLKCONST {
        /// Clock kinds
        clk: metamodelica::Ref<ClockKind>,
    },
    /// Enumeration literal
    ENUM_LITERAL {
        name: metamodelica::Ref<Absyn::Path>,
        index: i32,
    },
    /// component references, e.g. a.b{2}.c{1}
    CREF {
        componentRef: metamodelica::Ref<ComponentRef>,
        ty: metamodelica::Ref<Type>,
    },
    /// Binary operations, e.g. a+4
    BINARY {
        exp1: metamodelica::Ref<Exp>,
        operator: Operator,
        exp2: metamodelica::Ref<Exp>,
    },
    /// Unary operations, -(4x)
    UNARY {
        operator: Operator,
        exp: metamodelica::Ref<Exp>,
    },
    /// Logical binary operations: and, or
    LBINARY {
        exp1: metamodelica::Ref<Exp>,
        operator: Operator,
        exp2: metamodelica::Ref<Exp>,
    },
    /// Logical unary operations: not
    LUNARY {
        operator: Operator,
        exp: metamodelica::Ref<Exp>,
    },
    /// Relation, e.g. a <= 0
    ///    Index contains normal an Integer for every ZeroCrossing
    ///    but if Relation is in algorithm with for loop the iterator and the range
    ///    of static iterator is needed for codegen
    RELATION {
        exp1: metamodelica::Ref<Exp>,
        operator: Operator,
        exp2: metamodelica::Ref<Exp>,
        /// Use -1 as a default; other indexes are used in the backend for some silly reasons
        index: i32,
        optionExpisASUB: Option<(metamodelica::Ref<Exp>, i32, i32)>,
    },
    /// If expressions
    IFEXP {
        expCond: metamodelica::Ref<Exp>,
        expThen: metamodelica::Ref<Exp>,
        expElse: metamodelica::Ref<Exp>,
    },
    CALL {
        path: metamodelica::Ref<Absyn::Path>,
        expLst: metamodelica::List<metamodelica::Ref<Exp>>,
        attr: metamodelica::Ref<CallAttributes>,
    },
    /// A record value cannot be represented as a call to its constructor. This record also contains the protected components.
    RECORD {
        path: metamodelica::Ref<Absyn::Path>,
        /// component values
        exps: metamodelica::List<metamodelica::Ref<Exp>>,
        /// component name
        comp: metamodelica::List<ArcStr>,
        ty: metamodelica::Ref<Type>,
    },
    PARTEVALFUNCTION {
        path: metamodelica::Ref<Absyn::Path>,
        expList: metamodelica::List<metamodelica::Ref<Exp>>,
        ty: metamodelica::Ref<Type>,
        origType: metamodelica::Ref<Type>,
    },
    ARRAY {
        ty: metamodelica::Ref<Type>,
        /// scalar for codegen
        scalar: bool,
        /// Array constructor, e.g. {1,3,4}
        array: metamodelica::List<metamodelica::Ref<Exp>>,
    },
    MATRIX {
        ty: metamodelica::Ref<Type>,
        /// Size of the first dimension
        integer: i32,
        matrix: metamodelica::List<metamodelica::List<metamodelica::Ref<Exp>>>,
    },
    RANGE {
        /// the (array) type of the expression
        ty: metamodelica::Ref<Type>,
        /// start value
        start: metamodelica::Ref<Exp>,
        /// step value
        step: Option<metamodelica::Ref<Exp>>,
        /// stop value
        stop: metamodelica::Ref<Exp>,
    },
    TUPLE {
        /// PR. Tuples, used in func calls returning several
        ///                  arguments
        PR: metamodelica::List<metamodelica::Ref<Exp>>,
    },
    /// Cast operator
    CAST {
        /// This is the full type of this expression, i.e. ET_ARRAY(...) for arrays and matrices
        ty: metamodelica::Ref<Type>,
        exp: metamodelica::Ref<Exp>,
    },
    /// Array subscripts
    ASUB {
        exp: metamodelica::Ref<Exp>,
        sub: metamodelica::List<metamodelica::Ref<Subscript>>,
    },
    /// Tuple 'subscript' (accessing only single values in calls)
    TSUB {
        exp: metamodelica::Ref<Exp>,
        ix: i32,
        ty: metamodelica::Ref<Type>,
    },
    /// Record field indexing
    RSUB {
        exp: metamodelica::Ref<Exp>,
        ix: i32,
        fieldName: ArcStr,
        ty: metamodelica::Ref<Type>,
    },
    /// The size operator
    SIZE {
        exp: metamodelica::Ref<Exp>,
        sz: Option<metamodelica::Ref<Exp>>,
    },
    /// Modelica AST constructor
    CODE {
        code: metamodelica::Ref<Absyn::CodeNode>,
        ty: metamodelica::Ref<Type>,
    },
    /// an empty expression, meaning a constant without a binding. is used to be able to continue the evaluation of a model even if there are
    ///     constants with no bindings. at the end, when we have the DAE we should have no EMPTY values or expressions in it when we need to simulate
    ///     the model.
    ///     From Modelica specification: a package may we look inside should not be partial in a simulation model!
    EMPTY {
        /// the scope where we could not find the binding
        scope: ArcStr,
        /// the name of the variable
        name: metamodelica::Ref<ComponentRef>,
        /// the type of the variable
        ty: metamodelica::Ref<Type>,
        tyStr: ArcStr,
    },
    /// e.g. sum(i*i+1 for i in 1:4)
    REDUCTION {
        reductionInfo: metamodelica::Ref<ReductionInfo>,
        /// expr, e.g i*i+1
        expr: metamodelica::Ref<Exp>,
        iterators: ReductionIterators,
    },
    /// MetaModelica list
    LIST {
        valList: metamodelica::List<metamodelica::Ref<Exp>>,
    },
    /// MetaModelica list cons
    CONS {
        car: metamodelica::Ref<Exp>,
        cdr: metamodelica::Ref<Exp>,
    },
    META_TUPLE {
        listExp: metamodelica::List<metamodelica::Ref<Exp>>,
    },
    META_OPTION {
        exp: Option<metamodelica::Ref<Exp>>,
    },
    METARECORDCALL {
        path: metamodelica::Ref<Absyn::Path>,
        args: metamodelica::List<metamodelica::Ref<Exp>>,
        fieldNames: metamodelica::List<ArcStr>,
        index: i32,
        typeVars: metamodelica::List<metamodelica::Ref<Type>>,
    },
    MATCHEXPRESSION {
        matchType: MatchType,
        inputs: metamodelica::List<metamodelica::Ref<Exp>>,
        /// input aliases (input as-bindings)
        aliases: metamodelica::List<metamodelica::List<ArcStr>>,
        localDecls: metamodelica::List<metamodelica::Ref<Element>>,
        cases: metamodelica::List<metamodelica::Ref<MatchCase>>,
        et: metamodelica::Ref<Type>,
    },
    /// MetaModelica boxed value
    BOX {
        exp: metamodelica::Ref<Exp>,
    },
    /// MetaModelica value unboxing (similar to a cast)
    UNBOX {
        exp: metamodelica::Ref<Exp>,
        ty: metamodelica::Ref<Type>,
    },
    /// Before code generation, we make a pass that replaces constant literals
    ///    with a SHARED_LITERAL expression. Any immutable type can be shared:
    ///    basic MetaModelica types and Modelica strings are fine. There is no point
    ///    to share Real, Integer, Boolean or Enum though.
    SHARED_LITERAL {
        /// A unique indexing that can be used to point to a single shared literal in generated code
        index: i32,
        /// For printing strings, code generators that do not support this kind of literal, or for getting the type in case the code generator needs that
        exp: metamodelica::Ref<Exp>,
    },
    /// (x,1,ROOT(a as _,false,_)) := rhs; MetaModelica extension
    PATTERN {
        pattern: metamodelica::Ref<Pattern>,
    },
}
impl metamodelica::gc::MMTrace for Exp {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Exp::ICONST { integer } => {
                metamodelica::gc::MMTrace::mm_accept(integer, __mmv)?;
                Ok(())
            }
            Exp::RCONST { real } => {
                metamodelica::gc::MMTrace::mm_accept(real, __mmv)?;
                Ok(())
            }
            Exp::SCONST { string } => {
                metamodelica::gc::MMTrace::mm_accept(string, __mmv)?;
                Ok(())
            }
            Exp::BCONST { bool } => {
                metamodelica::gc::MMTrace::mm_accept(bool, __mmv)?;
                Ok(())
            }
            Exp::CLKCONST { clk } => {
                metamodelica::gc::MMTrace::mm_accept(clk, __mmv)?;
                Ok(())
            }
            Exp::ENUM_LITERAL { name, index } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                Ok(())
            }
            Exp::CREF { componentRef, ty } => {
                metamodelica::gc::MMTrace::mm_accept(componentRef, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Exp::BINARY { exp1, operator, exp2 } => {
                metamodelica::gc::MMTrace::mm_accept(exp1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp2, __mmv)?;
                Ok(())
            }
            Exp::UNARY { operator, exp } => {
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            Exp::LBINARY { exp1, operator, exp2 } => {
                metamodelica::gc::MMTrace::mm_accept(exp1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp2, __mmv)?;
                Ok(())
            }
            Exp::LUNARY { operator, exp } => {
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            Exp::RELATION {
                exp1,
                operator,
                exp2,
                index,
                optionExpisASUB,
            } => {
                metamodelica::gc::MMTrace::mm_accept(exp1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(optionExpisASUB, __mmv)?;
                Ok(())
            }
            Exp::IFEXP {
                expCond,
                expThen,
                expElse,
            } => {
                metamodelica::gc::MMTrace::mm_accept(expCond, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(expThen, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(expElse, __mmv)?;
                Ok(())
            }
            Exp::CALL { path, expLst, attr } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(expLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                Ok(())
            }
            Exp::RECORD { path, exps, comp, ty } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exps, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(comp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Exp::PARTEVALFUNCTION {
                path,
                expList,
                ty,
                origType,
            } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(expList, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(origType, __mmv)?;
                Ok(())
            }
            Exp::ARRAY { ty, scalar, array } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scalar, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(array, __mmv)?;
                Ok(())
            }
            Exp::MATRIX { ty, integer, matrix } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(integer, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(matrix, __mmv)?;
                Ok(())
            }
            Exp::RANGE { ty, start, step, stop } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(step, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(stop, __mmv)?;
                Ok(())
            }
            Exp::TUPLE { PR } => {
                metamodelica::gc::MMTrace::mm_accept(PR, __mmv)?;
                Ok(())
            }
            Exp::CAST { ty, exp } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            Exp::ASUB { exp, sub } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(sub, __mmv)?;
                Ok(())
            }
            Exp::TSUB { exp, ix, ty } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Exp::RSUB { exp, ix, fieldName, ty } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fieldName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Exp::SIZE { exp, sz } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(sz, __mmv)?;
                Ok(())
            }
            Exp::CODE { code, ty } => {
                metamodelica::gc::MMTrace::mm_accept(code, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Exp::EMPTY { scope, name, ty, tyStr } => {
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(tyStr, __mmv)?;
                Ok(())
            }
            Exp::REDUCTION {
                reductionInfo,
                expr,
                iterators,
            } => {
                metamodelica::gc::MMTrace::mm_accept(reductionInfo, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(expr, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iterators, __mmv)?;
                Ok(())
            }
            Exp::LIST { valList } => {
                metamodelica::gc::MMTrace::mm_accept(valList, __mmv)?;
                Ok(())
            }
            Exp::CONS { car, cdr } => {
                metamodelica::gc::MMTrace::mm_accept(car, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cdr, __mmv)?;
                Ok(())
            }
            Exp::META_TUPLE { listExp } => {
                metamodelica::gc::MMTrace::mm_accept(listExp, __mmv)?;
                Ok(())
            }
            Exp::META_OPTION { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            Exp::METARECORDCALL {
                path,
                args,
                fieldNames,
                index,
                typeVars,
            } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(args, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fieldNames, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(typeVars, __mmv)?;
                Ok(())
            }
            Exp::MATCHEXPRESSION {
                matchType,
                inputs,
                aliases,
                localDecls,
                cases,
                et,
            } => {
                metamodelica::gc::MMTrace::mm_accept(matchType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(inputs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(aliases, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(localDecls, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cases, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(et, __mmv)?;
                Ok(())
            }
            Exp::BOX { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            Exp::UNBOX { exp, ty } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Exp::SHARED_LITERAL { index, exp } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            Exp::PATTERN { pattern } => {
                metamodelica::gc::MMTrace::mm_accept(pattern, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Exp {
    fn default() -> Self {
        Self::ICONST {
            integer: Default::default(),
        }
    }
}
pub use self::Exp::{
    ARRAY, ASUB, BCONST, BINARY, BOX, CALL, CAST, CLKCONST, CODE, CONS, CREF, EMPTY, ENUM_LITERAL, ICONST, IFEXP,
    LBINARY, LIST, LUNARY, MATCHEXPRESSION, MATRIX, META_OPTION, META_TUPLE, METARECORDCALL, PARTEVALFUNCTION, PATTERN,
    RANGE, RCONST, RECORD, REDUCTION, RELATION, RSUB, SCONST, SHARED_LITERAL, SIZE, TSUB, TUPLE, UNARY, UNBOX,
};

/* mathematica constants */
thread_local! { static __PI_TLS: metamodelica::Ref<Exp> = metamodelica::Ref::new(Exp::RCONST { real: metamodelica::OrderedFloat(3.1415926535897932384626433832795028841971693993751058_f64) }); }
pub fn PI() -> metamodelica::Ref<Exp> {
    __PI_TLS.with(|__t| __t.clone())
}

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum TailCall {
    /// Not tail-recursive
    NO_TAIL,
    TAIL {
        vars: metamodelica::List<ArcStr>,
        outVars: metamodelica::List<ArcStr>,
    },
}
impl metamodelica::gc::MMTrace for TailCall {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            TailCall::NO_TAIL => Ok(()),
            TailCall::TAIL { vars, outVars } => {
                metamodelica::gc::MMTrace::mm_accept(vars, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(outVars, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for TailCall {
    fn default() -> Self {
        Self::NO_TAIL
    }
}
pub use self::TailCall::{NO_TAIL, TAIL};

thread_local! { static __callAttrBuiltinBool_TLS: metamodelica::Ref<CallAttributes> = metamodelica::Ref::new(CallAttributes { ty: T_BOOL_DEFAULT().clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL, noReturn: NoReturn::RETURNS.clone() }); }
pub fn callAttrBuiltinBool() -> metamodelica::Ref<CallAttributes> {
    __callAttrBuiltinBool_TLS.with(|__t| __t.clone())
}

thread_local! { static __callAttrBuiltinInteger_TLS: metamodelica::Ref<CallAttributes> = metamodelica::Ref::new(CallAttributes { ty: T_INTEGER_DEFAULT().clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL, noReturn: NoReturn::RETURNS.clone() }); }
pub fn callAttrBuiltinInteger() -> metamodelica::Ref<CallAttributes> {
    __callAttrBuiltinInteger_TLS.with(|__t| __t.clone())
}

thread_local! { static __callAttrBuiltinReal_TLS: metamodelica::Ref<CallAttributes> = metamodelica::Ref::new(CallAttributes { ty: T_REAL_DEFAULT().clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL, noReturn: NoReturn::RETURNS.clone() }); }
pub fn callAttrBuiltinReal() -> metamodelica::Ref<CallAttributes> {
    __callAttrBuiltinReal_TLS.with(|__t| __t.clone())
}

thread_local! { static __callAttrBuiltinString_TLS: metamodelica::Ref<CallAttributes> = metamodelica::Ref::new(CallAttributes { ty: T_STRING_DEFAULT().clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL, noReturn: NoReturn::RETURNS.clone() }); }
pub fn callAttrBuiltinString() -> metamodelica::Ref<CallAttributes> {
    __callAttrBuiltinString_TLS.with(|__t| __t.clone())
}

thread_local! { static __callAttrBuiltinOther_TLS: metamodelica::Ref<CallAttributes> = metamodelica::Ref::new(CallAttributes { ty: T_UNKNOWN_DEFAULT().clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL, noReturn: NoReturn::RETURNS.clone() }); }
pub fn callAttrBuiltinOther() -> metamodelica::Ref<CallAttributes> {
    __callAttrBuiltinOther_TLS.with(|__t| __t.clone())
}

thread_local! { static __callAttrBuiltinImpureBool_TLS: metamodelica::Ref<CallAttributes> = metamodelica::Ref::new(CallAttributes { ty: T_BOOL_DEFAULT().clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL, noReturn: NoReturn::RETURNS.clone() }); }
pub fn callAttrBuiltinImpureBool() -> metamodelica::Ref<CallAttributes> {
    __callAttrBuiltinImpureBool_TLS.with(|__t| __t.clone())
}

thread_local! { static __callAttrBuiltinImpureInteger_TLS: metamodelica::Ref<CallAttributes> = metamodelica::Ref::new(CallAttributes { ty: T_INTEGER_DEFAULT().clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL, noReturn: NoReturn::RETURNS.clone() }); }
pub fn callAttrBuiltinImpureInteger() -> metamodelica::Ref<CallAttributes> {
    __callAttrBuiltinImpureInteger_TLS.with(|__t| __t.clone())
}

thread_local! { static __callAttrBuiltinImpureReal_TLS: metamodelica::Ref<CallAttributes> = metamodelica::Ref::new(CallAttributes { ty: T_REAL_DEFAULT().clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL, noReturn: NoReturn::RETURNS.clone() }); }
pub fn callAttrBuiltinImpureReal() -> metamodelica::Ref<CallAttributes> {
    __callAttrBuiltinImpureReal_TLS.with(|__t| __t.clone())
}

thread_local! { static __callAttrBuiltinImpureString_TLS: metamodelica::Ref<CallAttributes> = metamodelica::Ref::new(CallAttributes { ty: T_STRING_DEFAULT().clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL, noReturn: NoReturn::RETURNS.clone() }); }
pub fn callAttrBuiltinImpureString() -> metamodelica::Ref<CallAttributes> {
    __callAttrBuiltinImpureString_TLS.with(|__t| __t.clone())
}

thread_local! { static __callAttrOther_TLS: metamodelica::Ref<CallAttributes> = metamodelica::Ref::new(CallAttributes { ty: T_UNKNOWN_DEFAULT().clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL, noReturn: NoReturn::RETURNS.clone() }); }
pub fn callAttrOther() -> metamodelica::Ref<CallAttributes> {
    __callAttrOther_TLS.with(|__t| __t.clone())
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct CallAttributes {
    /// The type of the return value, if several return values this is undefined
    pub ty: metamodelica::Ref<Type>,
    /// tuple
    pub tuple_: bool,
    /// builtin Function call
    pub builtin: bool,
    /// if the function has prefix *impure* is true, else false
    pub isImpure: bool,
    pub isFunctionPointerCall: bool,
    pub inlineType: InlineType,
    /// Input variables of the function if the call is tail-recursive
    pub tailCall: TailCall,
    /// whether the called function ever returns normally
    pub noReturn: NoReturn,
}

impl metamodelica::gc::MMTrace for CallAttributes {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.ty, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.tuple_, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.builtin, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isImpure, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isFunctionPointerCall, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.inlineType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.tailCall, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.noReturn, __mmv)?;
        Ok(())
    }
}
impl Default for CallAttributes {
    fn default() -> Self {
        Self {
            ty: Default::default(),
            tuple_: Default::default(),
            builtin: Default::default(),
            isImpure: Default::default(),
            isFunctionPointerCall: Default::default(),
            inlineType: Default::default(),
            tailCall: Default::default(),
            noReturn: Default::default(),
        }
    }
}

pub type CALL_ATTR = CallAttributes;

/// A separate uniontype containing the information not required by traverseExp, etc
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ReductionInfo {
    /// array, sum,..
    pub path: metamodelica::Ref<Absyn::Path>,
    pub iterType: Absyn::ReductionIterType,
    pub exprType: metamodelica::Ref<Type>,
    /// if there is no default value, the reduction is not defined for 0-length arrays/lists
    pub defaultValue: Option<metamodelica::Ref<Values::Value>>,
    pub foldName: ArcStr,
    /// Unique identifier for the resulting expression
    pub resultName: ArcStr,
    /// For example, max(ident,$res) or ident+$res; array() does not use this feature; DO NOT TRAVERSE THIS EXPRESSION!
    pub foldExp: Option<metamodelica::Ref<Exp>>,
}

impl metamodelica::gc::MMTrace for ReductionInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.path, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.iterType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.exprType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.defaultValue, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.foldName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.resultName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.foldExp, __mmv)?;
        Ok(())
    }
}
pub type REDUCTIONINFO = ReductionInfo;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ReductionIterator {
    pub id: ArcStr,
    pub exp: metamodelica::Ref<Exp>,
    pub guardExp: Option<metamodelica::Ref<Exp>>,
    pub ty: metamodelica::Ref<Type>,
}

impl metamodelica::gc::MMTrace for ReductionIterator {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.id, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.exp, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.guardExp, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ty, __mmv)?;
        Ok(())
    }
}
impl Default for ReductionIterator {
    fn default() -> Self {
        Self {
            id: Default::default(),
            exp: Default::default(),
            guardExp: Default::default(),
            ty: Default::default(),
        }
    }
}

pub type REDUCTIONITER = ReductionIterator;

/// NOTE: OMC only handles one iterator for now
pub type ReductionIterators = metamodelica::List<metamodelica::Ref<ReductionIterator>>;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct MatchCase {
    /// ELSE is handled by not doing pattern-matching
    pub patterns: metamodelica::List<metamodelica::Ref<Pattern>>,
    /// Guard-expression
    pub patternGuard: Option<metamodelica::Ref<Exp>>,
    pub localDecls: metamodelica::List<metamodelica::Ref<Element>>,
    pub body: metamodelica::List<metamodelica::Ref<Statement>>,
    pub result: Option<metamodelica::Ref<Exp>>,
    /// We need to keep the line info here so we can set a breakpoint at the last statement of a match-expression
    pub resultInfo: SourceInfo,
    /// the number of iterations we should skip if we succeed with pattern-matching, but don't succeed
    pub jump: i32,
    pub info: SourceInfo,
}

impl metamodelica::gc::MMTrace for MatchCase {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.patterns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.patternGuard, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.localDecls, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.body, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.result, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.resultInfo, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.jump, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.info, __mmv)?;
        Ok(())
    }
}
impl Default for MatchCase {
    fn default() -> Self {
        Self {
            patterns: Default::default(),
            patternGuard: Default::default(),
            localDecls: Default::default(),
            body: Default::default(),
            result: Default::default(),
            resultInfo: Default::default(),
            jump: Default::default(),
            info: Default::default(),
        }
    }
}

pub type CASE = MatchCase;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum MatchType {
    MATCHCONTINUE,
    TRY_STACKOVERFLOW,
    MATCH {
        /// The index of the pattern to switch over, its type and the value to divide string hashes with
        switch: Option<(i32, metamodelica::Ref<Type>, i32)>,
    },
}
impl metamodelica::gc::MMTrace for MatchType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            MatchType::MATCHCONTINUE => Ok(()),
            MatchType::TRY_STACKOVERFLOW => Ok(()),
            MatchType::MATCH { switch } => {
                metamodelica::gc::MMTrace::mm_accept(switch, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for MatchType {
    fn default() -> Self {
        Self::MATCHCONTINUE
    }
}
pub use self::MatchType::{MATCH, MATCHCONTINUE, TRY_STACKOVERFLOW};

/// Patterns deconstruct expressions
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Pattern {
    /// _
    PAT_WILD,
    /// compare to this constant value using equality
    PAT_CONSTANT {
        /// so we can unbox if needed
        ty: Option<metamodelica::Ref<Type>>,
        exp: metamodelica::Ref<Exp>,
    },
    /// id as pat
    PAT_AS {
        id: ArcStr,
        /// so we can unbox if needed
        ty: Option<metamodelica::Ref<Type>>,
        /// so we know if the ident is parameter or assignable
        attr: metamodelica::Ref<Attributes>,
        pat: metamodelica::Ref<Pattern>,
    },
    /// id as pat
    PAT_AS_FUNC_PTR {
        id: ArcStr,
        pat: metamodelica::Ref<Pattern>,
    },
    /// (pat1,...,patn)
    PAT_META_TUPLE {
        patterns: metamodelica::List<metamodelica::Ref<Pattern>>,
    },
    /// (pat1,...,patn)
    PAT_CALL_TUPLE {
        patterns: metamodelica::List<metamodelica::Ref<Pattern>>,
    },
    /// head::tail
    PAT_CONS {
        head: metamodelica::Ref<Pattern>,
        tail: metamodelica::Ref<Pattern>,
    },
    /// RECORD(pat1,...,patn); all patterns are positional
    PAT_CALL {
        name: metamodelica::Ref<Absyn::Path>,
        index: i32,
        patterns: metamodelica::List<metamodelica::Ref<Pattern>>,
        fields: metamodelica::List<metamodelica::Ref<Var>>,
        typeVars: metamodelica::List<metamodelica::Ref<Type>>,
        /// The runtime system (dynload), does not know if the value is a singleton. But optimizations are safe if this is true.
        knownSingleton: bool,
    },
    /// RECORD(pat1,...,patn); all patterns are named
    PAT_CALL_NAMED {
        name: metamodelica::Ref<Absyn::Path>,
        patterns: metamodelica::List<(metamodelica::Ref<Pattern>, ArcStr, metamodelica::Ref<Type>)>,
    },
    /// SOME(pat)
    PAT_SOME { pat: metamodelica::Ref<Pattern> },
}
impl metamodelica::gc::MMTrace for Pattern {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Pattern::PAT_WILD => Ok(()),
            Pattern::PAT_CONSTANT { ty, exp } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            Pattern::PAT_AS { id, ty, attr, pat } => {
                metamodelica::gc::MMTrace::mm_accept(id, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(pat, __mmv)?;
                Ok(())
            }
            Pattern::PAT_AS_FUNC_PTR { id, pat } => {
                metamodelica::gc::MMTrace::mm_accept(id, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(pat, __mmv)?;
                Ok(())
            }
            Pattern::PAT_META_TUPLE { patterns } => {
                metamodelica::gc::MMTrace::mm_accept(patterns, __mmv)?;
                Ok(())
            }
            Pattern::PAT_CALL_TUPLE { patterns } => {
                metamodelica::gc::MMTrace::mm_accept(patterns, __mmv)?;
                Ok(())
            }
            Pattern::PAT_CONS { head, tail } => {
                metamodelica::gc::MMTrace::mm_accept(head, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(tail, __mmv)?;
                Ok(())
            }
            Pattern::PAT_CALL {
                name,
                index,
                patterns,
                fields,
                typeVars,
                knownSingleton,
            } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(patterns, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fields, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(typeVars, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(knownSingleton, __mmv)?;
                Ok(())
            }
            Pattern::PAT_CALL_NAMED { name, patterns } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(patterns, __mmv)?;
                Ok(())
            }
            Pattern::PAT_SOME { pat } => {
                metamodelica::gc::MMTrace::mm_accept(pat, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Pattern {
    pub fn interned_PAT_WILD() -> metamodelica::Ref<Pattern> {
        thread_local! {
            static INTERNED: metamodelica::Ref<Pattern> = metamodelica::Ref::new(Pattern::PAT_WILD);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_PAT_WILD() -> metamodelica::Ref<Pattern> {
    Pattern::interned_PAT_WILD()
}
impl Default for Pattern {
    fn default() -> Self {
        Self::PAT_WILD
    }
}
pub use self::Pattern::{
    PAT_AS, PAT_AS_FUNC_PTR, PAT_CALL, PAT_CALL_NAMED, PAT_CALL_TUPLE, PAT_CONS, PAT_CONSTANT, PAT_META_TUPLE,
    PAT_SOME, PAT_WILD,
};

/// Operators which are overloaded in the abstract syntax are here
///    made type-specific.  The integer addition operator (`ADD(INT)\')
///    and the real addition operator (`ADD(REAL)\') are two distinct
///    operators.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Operator {
    ADD {
        ty: metamodelica::Ref<Type>,
    },
    SUB {
        ty: metamodelica::Ref<Type>,
    },
    MUL {
        ty: metamodelica::Ref<Type>,
    },
    DIV {
        ty: metamodelica::Ref<Type>,
    },
    POW {
        ty: metamodelica::Ref<Type>,
    },
    UMINUS {
        ty: metamodelica::Ref<Type>,
    },
    UMINUS_ARR {
        ty: metamodelica::Ref<Type>,
    },
    ADD_ARR {
        ty: metamodelica::Ref<Type>,
    },
    SUB_ARR {
        ty: metamodelica::Ref<Type>,
    },
    /// Element-wise array multiplication
    MUL_ARR {
        ty: metamodelica::Ref<Type>,
    },
    DIV_ARR {
        ty: metamodelica::Ref<Type>,
    },
    /// {a,b,c} * s
    MUL_ARRAY_SCALAR {
        /// type of the array
        ty: metamodelica::Ref<Type>,
    },
    /// {a,b,c} .+ s
    ADD_ARRAY_SCALAR {
        /// type of the array
        ty: metamodelica::Ref<Type>,
    },
    /// s .- {a,b,c}
    SUB_SCALAR_ARRAY {
        /// type of the array
        ty: metamodelica::Ref<Type>,
    },
    /// {a,b,c} * {c,d,e} => a*c+b*d+c*e
    MUL_SCALAR_PRODUCT {
        /// type of the array
        ty: metamodelica::Ref<Type>,
    },
    /// M1 * M2, matrix dot product
    MUL_MATRIX_PRODUCT {
        /// {{..},..}  {{..},{..}}
        ty: metamodelica::Ref<Type>,
    },
    /// {a, b} / c
    DIV_ARRAY_SCALAR {
        /// type of the array
        ty: metamodelica::Ref<Type>,
    },
    /// c / {a,b}
    DIV_SCALAR_ARRAY {
        /// type of the array
        ty: metamodelica::Ref<Type>,
    },
    POW_ARRAY_SCALAR {
        /// type of the array
        ty: metamodelica::Ref<Type>,
    },
    POW_SCALAR_ARRAY {
        /// type of the array
        ty: metamodelica::Ref<Type>,
    },
    /// Power of a matrix: {{1,2,3},{4,5.0,6},{7,8,9}}^2
    POW_ARR {
        /// type of the array
        ty: metamodelica::Ref<Type>,
    },
    /// elementwise power of arrays: {1,2,3}.^{3,2,1}
    POW_ARR2 {
        /// type of the array
        ty: metamodelica::Ref<Type>,
    },
    AND {
        ty: metamodelica::Ref<Type>,
    },
    OR {
        ty: metamodelica::Ref<Type>,
    },
    NOT {
        ty: metamodelica::Ref<Type>,
    },
    LESS {
        ty: metamodelica::Ref<Type>,
    },
    LESSEQ {
        ty: metamodelica::Ref<Type>,
    },
    GREATER {
        ty: metamodelica::Ref<Type>,
    },
    GREATEREQ {
        ty: metamodelica::Ref<Type>,
    },
    EQUAL {
        ty: metamodelica::Ref<Type>,
    },
    NEQUAL {
        ty: metamodelica::Ref<Type>,
    },
    USERDEFINED {
        /// The FQ name of the overloaded operator function
        fqName: metamodelica::Ref<Absyn::Path>,
    },
}
impl metamodelica::gc::MMTrace for Operator {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Operator::ADD { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::SUB { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::MUL { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::DIV { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::POW { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::UMINUS { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::UMINUS_ARR { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::ADD_ARR { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::SUB_ARR { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::MUL_ARR { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::DIV_ARR { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::MUL_ARRAY_SCALAR { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::ADD_ARRAY_SCALAR { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::SUB_SCALAR_ARRAY { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::MUL_SCALAR_PRODUCT { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::MUL_MATRIX_PRODUCT { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::DIV_ARRAY_SCALAR { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::DIV_SCALAR_ARRAY { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::POW_ARRAY_SCALAR { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::POW_SCALAR_ARRAY { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::POW_ARR { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::POW_ARR2 { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::AND { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::OR { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::NOT { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::LESS { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::LESSEQ { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::GREATER { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::GREATEREQ { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::EQUAL { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::NEQUAL { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            Operator::USERDEFINED { fqName } => {
                metamodelica::gc::MMTrace::mm_accept(fqName, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Operator {
    fn default() -> Self {
        Self::ADD { ty: Default::default() }
    }
}
pub use self::Operator::{
    ADD, ADD_ARR, ADD_ARRAY_SCALAR, AND, DIV, DIV_ARR, DIV_ARRAY_SCALAR, DIV_SCALAR_ARRAY, EQUAL, GREATER, GREATEREQ,
    LESS, LESSEQ, MUL, MUL_ARR, MUL_ARRAY_SCALAR, MUL_MATRIX_PRODUCT, MUL_SCALAR_PRODUCT, NEQUAL, NOT, OR, POW,
    POW_ARR, POW_ARR2, POW_ARRAY_SCALAR, POW_SCALAR_ARRAY, SUB, SUB_ARR, SUB_SCALAR_ARRAY, UMINUS, UMINUS_ARR,
    USERDEFINED,
};

/// - Component references
///    CREF_QUAL(...) is used for qualified component names, e.g. a.b.c
///    CREF_IDENT(..) is used for non-qualifed component names, e.g. x
///    Outermost CREF_QUAL(...) is leftmost name. e.g. CREF_QUAL(a, CREF_IDENT(b)) -> a.b
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ComponentRef {
    CREF_QUAL {
        ident: Ident,
        /// type of the identifier, without considering the subscripts
        identType: metamodelica::Ref<Type>,
        subscriptLst: metamodelica::List<metamodelica::Ref<Subscript>>,
        componentRef: metamodelica::Ref<ComponentRef>,
    },
    CREF_IDENT {
        ident: Ident,
        /// type of the identifier, without considering the subscripts
        identType: metamodelica::Ref<Type>,
        subscriptLst: metamodelica::List<metamodelica::Ref<Subscript>>,
    },
    /// An Optimica component reference with the time instant in it. e.g x2(finalTime)
    OPTIMICA_ATTR_INST_CREF {
        componentRef: metamodelica::Ref<ComponentRef>,
        instant: ArcStr,
    },
    WILD,
}
impl metamodelica::gc::MMTrace for ComponentRef {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ComponentRef::CREF_QUAL {
                ident,
                identType,
                subscriptLst,
                componentRef,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ident, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(identType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(subscriptLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(componentRef, __mmv)?;
                Ok(())
            }
            ComponentRef::CREF_IDENT {
                ident,
                identType,
                subscriptLst,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ident, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(identType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(subscriptLst, __mmv)?;
                Ok(())
            }
            ComponentRef::OPTIMICA_ATTR_INST_CREF { componentRef, instant } => {
                metamodelica::gc::MMTrace::mm_accept(componentRef, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(instant, __mmv)?;
                Ok(())
            }
            ComponentRef::WILD => Ok(()),
        }
    }
}
impl ComponentRef {
    pub fn interned_WILD() -> metamodelica::Ref<ComponentRef> {
        thread_local! {
            static INTERNED: metamodelica::Ref<ComponentRef> = metamodelica::Ref::new(ComponentRef::WILD);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_WILD() -> metamodelica::Ref<ComponentRef> {
    ComponentRef::interned_WILD()
}
impl Default for ComponentRef {
    fn default() -> Self {
        Self::WILD
    }
}
pub use self::ComponentRef::{CREF_IDENT, CREF_QUAL, OPTIMICA_ATTR_INST_CREF, WILD};

thread_local! { static __crefTime_TLS: metamodelica::Ref<ComponentRef> = metamodelica::Ref::new(ComponentRef::CREF_IDENT { ident: literal!("time"), identType: T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }); }
pub fn crefTime() -> metamodelica::Ref<ComponentRef> {
    __crefTime_TLS.with(|__t| __t.clone())
}

thread_local! { static __crefTimeState_TLS: metamodelica::Ref<ComponentRef> = metamodelica::Ref::new(ComponentRef::CREF_IDENT { ident: literal!("$time"), identType: T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }); }
pub fn crefTimeState() -> metamodelica::Ref<ComponentRef> {
    __crefTimeState_TLS.with(|__t| __t.clone())
}

thread_local! { static __emptyCref_TLS: metamodelica::Ref<ComponentRef> = metamodelica::Ref::new(ComponentRef::CREF_IDENT { ident: literal!(""), identType: T_UNKNOWN_DEFAULT().clone(), subscriptLst: metamodelica::nil() }); }
pub fn emptyCref() -> metamodelica::Ref<ComponentRef> {
    __emptyCref_TLS.with(|__t| __t.clone())
}

/// The `Subscript\' and `ComponentRef\' datatypes are simple
///  translations of the corresponding types in the `Absyn\' module.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Subscript {
    /// a{:,1}
    WHOLEDIM,
    SLICE {
        /// a{1:3,1}, a{1:2:10,2}
        exp: metamodelica::Ref<Exp>,
    },
    INDEX {
        /// a[i+1]
        exp: metamodelica::Ref<Exp>,
    },
    /// Used for non-expanded arrays. Should probably be combined with WHOLEDIM
    ///    into one case with Option<Exp> argument.
    WHOLE_NONEXP { exp: metamodelica::Ref<Exp> },
}
impl metamodelica::gc::MMTrace for Subscript {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Subscript::WHOLEDIM => Ok(()),
            Subscript::SLICE { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            Subscript::INDEX { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            Subscript::WHOLE_NONEXP { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Subscript {
    pub fn interned_WHOLEDIM() -> metamodelica::Ref<Subscript> {
        thread_local! {
            static INTERNED: metamodelica::Ref<Subscript> = metamodelica::Ref::new(Subscript::WHOLEDIM);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_WHOLEDIM() -> metamodelica::Ref<Subscript> {
    Subscript::interned_WHOLEDIM()
}
impl Default for Subscript {
    fn default() -> Self {
        Self::WHOLEDIM
    }
}
pub use self::Subscript::{INDEX, SLICE, WHOLE_NONEXP, WHOLEDIM};

/* -- End Expression.mo -- */
/// array cref expansion strategy
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Expand {
    /// expand crefs
    EXPAND,
    /// not expand crefs
    NOT_EXPAND,
}
impl metamodelica::gc::MMTrace for Expand {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Expand::EXPAND => Ok(()),
            Expand::NOT_EXPAND => Ok(()),
        }
    }
}
impl Default for Expand {
    fn default() -> Self {
        Self::EXPAND
    }
}
pub use self::Expand::{EXPAND, NOT_EXPAND};

thread_local! { static __emptyDae_TLS: DAElist = DAElist { elementLst: metamodelica::nil() }; }
pub fn emptyDae() -> DAElist {
    __emptyDae_TLS.with(|__t| __t.clone())
}

/// A Prefix has a component prefix and a class prefix.
/// The component prefix consist of a name an a list of constant valued subscripts.
/// The class prefix contains the variability of the class, i.e unspecified, parameter or constant.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Prefix {
    /// No prefix information
    NOPRE,
    PREFIX {
        /// component prefixes are stored in inverse order c.b.a
        compPre: metamodelica::Ref<ComponentPrefix>,
        /// the class prefix, i.e. variability, var, discrete, param, const
        classPre: ClassPrefix,
    },
}
impl metamodelica::gc::MMTrace for Prefix {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Prefix::NOPRE => Ok(()),
            Prefix::PREFIX { compPre, classPre } => {
                metamodelica::gc::MMTrace::mm_accept(compPre, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(classPre, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Prefix {
    fn default() -> Self {
        Self::NOPRE
    }
}
pub use self::Prefix::{NOPRE, PREFIX};

/// a type alias for an optional component prefix
pub type ComponentPrefixOpt = Option<metamodelica::Ref<ComponentPrefix>>;

/// Prefix for component name, e.g. a.b[2].c.
/// NOTE: Component prefixes are stored in inverse order c.b[2].a!
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ComponentPrefix {
    PRE {
        /// prefix name
        prefix: ArcStr,
        /// dimensions
        dimensions: metamodelica::List<metamodelica::Ref<Dimension>>,
        /// subscripts
        subscripts: metamodelica::List<metamodelica::Ref<Subscript>>,
        /// next prefix
        next: metamodelica::Ref<ComponentPrefix>,
        /// to be able to at least partially fill in type information properly for DAE.VAR
        ci_state: ClassInf::State,
        info: SourceInfo,
    },
    NOCOMPPRE,
}
impl metamodelica::gc::MMTrace for ComponentPrefix {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ComponentPrefix::PRE {
                prefix,
                dimensions,
                subscripts,
                next,
                ci_state,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(prefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimensions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(subscripts, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(next, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ci_state, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            ComponentPrefix::NOCOMPPRE => Ok(()),
        }
    }
}
impl ComponentPrefix {
    pub fn interned_NOCOMPPRE() -> metamodelica::Ref<ComponentPrefix> {
        thread_local! {
            static INTERNED: metamodelica::Ref<ComponentPrefix> = metamodelica::Ref::new(ComponentPrefix::NOCOMPPRE);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_NOCOMPPRE() -> metamodelica::Ref<ComponentPrefix> {
    ComponentPrefix::interned_NOCOMPPRE()
}
impl Default for ComponentPrefix {
    fn default() -> Self {
        Self::NOCOMPPRE
    }
}
pub use self::ComponentPrefix::{NOCOMPPRE, PRE};

/// Prefix for classes is its variability
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ClassPrefix {
    /// VAR, DISCRETE, PARAM, or CONST
    pub variability: SCode::Variability,
}

impl metamodelica::gc::MMTrace for ClassPrefix {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.variability, __mmv)?;
        Ok(())
    }
}
pub type CLASSPRE = ClassPrefix;

pub mod Connect {
    use super::*;
    pub const NEW_SET: i32 = -1;

    /// This type indicates whether a connector is an inside or an outside connector.
    ///   Note: this is not the same as inner and outer references.
    ///   A connector is inside if it connects from the outside into a component and it
    ///   is outside if it connects out from the component.  This is important when
    ///   generating equations for flow variables, where outside connectors are
    ///   multiplied with -1 (since flow is always into a component).
    #[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Face {
        /// This is an inside connection
        INSIDE,
        /// This is an outside connection
        OUTSIDE,
        NO_FACE,
    }
    impl metamodelica::gc::MMTrace for Face {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Face::INSIDE => Ok(()),
                Face::OUTSIDE => Ok(()),
                Face::NO_FACE => Ok(()),
            }
        }
    }
    impl Default for Face {
        fn default() -> Self {
            Self::INSIDE
        }
    }
    pub use self::Face::{INSIDE, NO_FACE, OUTSIDE};

    /// The type of a connector element.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum ConnectorType {
        EQU,
        FLOW,
        STREAM {
            associatedFlow: Option<metamodelica::Ref<ComponentRef>>,
        },
        NO_TYPE,
    }
    impl metamodelica::gc::MMTrace for ConnectorType {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                ConnectorType::EQU => Ok(()),
                ConnectorType::FLOW => Ok(()),
                ConnectorType::STREAM { associatedFlow } => {
                    metamodelica::gc::MMTrace::mm_accept(associatedFlow, __mmv)?;
                    Ok(())
                }
                ConnectorType::NO_TYPE => Ok(()),
            }
        }
    }
    impl Default for ConnectorType {
        fn default() -> Self {
            Self::EQU
        }
    }
    pub use self::ConnectorType::{EQU, FLOW, NO_TYPE, STREAM};

    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct ConnectorElement {
        pub name: metamodelica::Ref<ComponentRef>,
        pub face: Face,
        pub ty: ConnectorType,
        pub source: metamodelica::Ref<ElementSource>,
        /// Which set this element belongs to.
        pub set: i32,
    }

    impl metamodelica::gc::MMTrace for ConnectorElement {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.face, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.ty, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.source, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.set, __mmv)?;
            Ok(())
        }
    }
    impl Default for ConnectorElement {
        fn default() -> Self {
            Self {
                name: Default::default(),
                face: Default::default(),
                ty: Default::default(),
                source: Default::default(),
                set: Default::default(),
            }
        }
    }

    pub type CONNECTOR_ELEMENT = ConnectorElement;

    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum SetTrieNode {
        /// A trie node has a name and contains a list of child nodes.
        SET_TRIE_NODE {
            name: ArcStr,
            cref: metamodelica::Ref<ComponentRef>,
            nodes: metamodelica::List<metamodelica::Ref<SetTrieNode>>,
            connectCount: i32,
        },
        /// A trie leaf contains information about a connector element. Each connector
        ///     might be connected as both inside and outside, and stream connector
        ///     elements have an associated flow element.
        SET_TRIE_LEAF {
            name: ArcStr,
            /// The inside element.
            insideElement: Option<ConnectorElement>,
            /// The outside element.
            outsideElement: Option<ConnectorElement>,
            /// The name of the associated flow
            ///      variable, if the leaf represents a stream variable.
            flowAssociation: Option<metamodelica::Ref<ComponentRef>>,
            /// How many times this connector has been connected.
            connectCount: i32,
        },
    }
    impl metamodelica::gc::MMTrace for SetTrieNode {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                SetTrieNode::SET_TRIE_NODE {
                    name,
                    cref,
                    nodes,
                    connectCount,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(nodes, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(connectCount, __mmv)?;
                    Ok(())
                }
                SetTrieNode::SET_TRIE_LEAF {
                    name,
                    insideElement,
                    outsideElement,
                    flowAssociation,
                    connectCount,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(insideElement, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(outsideElement, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(flowAssociation, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(connectCount, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for SetTrieNode {
        fn default() -> Self {
            Self::SET_TRIE_NODE {
                name: Default::default(),
                cref: Default::default(),
                nodes: Default::default(),
                connectCount: Default::default(),
            }
        }
    }
    pub use self::SetTrieNode::{SET_TRIE_LEAF, SET_TRIE_NODE};

    /// A trie, a.k.a. prefix tree, that maps crefs to sets.
    pub type SetTrie = metamodelica::Ref<SetTrieNode>;

    /// A connection between two sets.
    pub type SetConnection = (i32, i32);

    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct OuterConnect {
        /// the scope where this connect was created
        pub scope: Prefix,
        /// the lhs component reference
        pub cr1: metamodelica::Ref<ComponentRef>,
        /// inner/outer attribute for cr1 component
        pub io1: Absyn::InnerOuter,
        /// the face of the lhs component
        pub f1: Face,
        /// the rhs component reference
        pub cr2: metamodelica::Ref<ComponentRef>,
        /// inner/outer attribute for cr2 component
        pub io2: Absyn::InnerOuter,
        /// the face of the rhs component
        pub f2: Face,
        /// the element origin
        pub source: metamodelica::Ref<ElementSource>,
    }

    impl metamodelica::gc::MMTrace for OuterConnect {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.scope, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.cr1, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.io1, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.f1, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.cr2, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.io2, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.f2, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.source, __mmv)?;
            Ok(())
        }
    }
    impl Default for OuterConnect {
        fn default() -> Self {
            Self {
                scope: Default::default(),
                cr1: Default::default(),
                io1: Default::default(),
                f1: Default::default(),
                cr2: Default::default(),
                io2: Default::default(),
                f2: Default::default(),
                source: Default::default(),
            }
        }
    }

    pub type OUTERCONNECT = OuterConnect;

    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Sets {
        pub sets: SetTrie,
        /// How many sets the trie contains.
        pub setCount: i32,
        pub connections: metamodelica::List<(i32, i32)>,
        /// Connect statements to propagate upwards.
        pub outerConnects: metamodelica::List<OuterConnect>,
    }

    impl metamodelica::gc::MMTrace for Sets {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.sets, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.setCount, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.connections, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.outerConnects, __mmv)?;
            Ok(())
        }
    }
    impl Default for Sets {
        fn default() -> Self {
            Self {
                sets: Default::default(),
                setCount: Default::default(),
                connections: Default::default(),
                outerConnects: Default::default(),
            }
        }
    }

    pub type SETS = Sets;

    /// A set of connection elements.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Set {
        /// A set with a type and a list of elements.
        SET {
            ty: ConnectorType,
            elements: metamodelica::List<ConnectorElement>,
        },
        /// A pointer to another set.
        SET_POINTER { index: i32 },
    }
    impl metamodelica::gc::MMTrace for Set {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Set::SET { ty, elements } => {
                    metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                    Ok(())
                }
                Set::SET_POINTER { index } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for Set {
        fn default() -> Self {
            Self::SET_POINTER {
                index: Default::default(),
            }
        }
    }
    pub use self::Set::{SET, SET_POINTER};

    thread_local! { static __emptySet_TLS: Sets = Sets { sets: metamodelica::Ref::new(SetTrieNode::SET_TRIE_NODE { name: literal!(""), cref: crate::DAE::ComponentRef::interned_WILD(), nodes: metamodelica::nil(), connectCount: 0 }), setCount: 0, connections: metamodelica::nil(), outerConnects: metamodelica::nil() }; }
    pub fn emptySet() -> Sets {
        __emptySet_TLS.with(|__t| __t.clone())
    }
}
