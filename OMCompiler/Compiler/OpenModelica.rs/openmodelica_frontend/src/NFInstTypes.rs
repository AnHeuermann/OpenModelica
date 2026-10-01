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
use openmodelica_frontend_inst::NFInstPrefix;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;

//public import NFConnect2;
pub type Prefix = metamodelica::Ref<NFInstPrefix::Prefix>;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Element {
    ELEMENT {
        component: metamodelica::Ref<Component>,
        cls: metamodelica::Ref<Class>,
    },
    CONDITIONAL_ELEMENT {
        component: metamodelica::Ref<Component>,
    },
    /// This record is used by NFInst.instElementList to store elements from
    ///     extends, but is removed by instFlatten. Most functions which handle
    ///     elements should therefore be able to ignore this record.
    EXTENDED_ELEMENTS {
        baseClass: metamodelica::Ref<Absyn::Path>,
        cls: metamodelica::Ref<Class>,
        ty: metamodelica::Ref<DAE::Type>,
    },
}
impl metamodelica::gc::MMTrace for Element {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Element::ELEMENT { component, cls } => {
                metamodelica::gc::MMTrace::mm_accept(component, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cls, __mmv)?;
                Ok(())
            }
            Element::CONDITIONAL_ELEMENT { component } => {
                metamodelica::gc::MMTrace::mm_accept(component, __mmv)?;
                Ok(())
            }
            Element::EXTENDED_ELEMENTS { baseClass, cls, ty } => {
                metamodelica::gc::MMTrace::mm_accept(baseClass, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cls, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::Element::{CONDITIONAL_ELEMENT, ELEMENT, EXTENDED_ELEMENTS};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Class {
    COMPLEX_CLASS {
        name: metamodelica::Ref<Absyn::Path>,
        components: metamodelica::List<metamodelica::Ref<Element>>,
        equations: metamodelica::List<metamodelica::Ref<Equation>>,
        initialEquations: metamodelica::List<metamodelica::Ref<Equation>>,
        algorithms: metamodelica::List<metamodelica::List<metamodelica::Ref<Statement>>>,
        initialAlgorithms: metamodelica::List<metamodelica::List<metamodelica::Ref<Statement>>>,
    },
    BASIC_TYPE {
        name: metamodelica::Ref<Absyn::Path>,
    },
}
impl metamodelica::gc::MMTrace for Class {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Class::COMPLEX_CLASS {
                name,
                components,
                equations,
                initialEquations,
                algorithms,
                initialAlgorithms,
            } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(components, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(initialEquations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(algorithms, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(initialAlgorithms, __mmv)?;
                Ok(())
            }
            Class::BASIC_TYPE { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::Class::{BASIC_TYPE, COMPLEX_CLASS};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Function {
    /// A function has inputs, output and locals without binding.
    ///     These are resolved to statements in the algorithm section
    FUNCTION {
        path: metamodelica::Ref<Absyn::Path>,
        inputs: metamodelica::List<metamodelica::Ref<Element>>,
        outputs: metamodelica::List<metamodelica::Ref<Element>>,
        locals: metamodelica::List<metamodelica::Ref<Element>>,
        /// TODO: Add default bindings
        algorithms: metamodelica::List<metamodelica::Ref<Statement>>,
    },
    /// A record constructor has inputs and locals (with bindings)?
    RECORD_CONSTRUCTOR {
        path: metamodelica::Ref<Absyn::Path>,
        recType: metamodelica::Ref<DAE::Type>,
        /// componets of the original record which CAN be modified
        inputs: metamodelica::List<metamodelica::Ref<Element>>,
        /// componets of the original record which CAN NOT be modified (protected, final, constant WITH binding)
        locals: metamodelica::List<metamodelica::Ref<Element>>,
        /// TODO: Add default bindings
        algorithms: metamodelica::List<metamodelica::Ref<Statement>>,
    },
}
impl metamodelica::gc::MMTrace for Function {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Function::FUNCTION {
                path,
                inputs,
                outputs,
                locals,
                algorithms,
            } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(inputs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(outputs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(locals, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(algorithms, __mmv)?;
                Ok(())
            }
            Function::RECORD_CONSTRUCTOR {
                path,
                recType,
                inputs,
                locals,
                algorithms,
            } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(recType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(inputs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(locals, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(algorithms, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::Function::{FUNCTION, RECORD_CONSTRUCTOR};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Dimension {
    UNTYPED_DIMENSION {
        dimension: metamodelica::Ref<DAE::Dimension>,
        isProcessing: bool,
    },
    TYPED_DIMENSION {
        dimension: metamodelica::Ref<DAE::Dimension>,
    },
}
impl metamodelica::gc::MMTrace for Dimension {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Dimension::UNTYPED_DIMENSION {
                dimension,
                isProcessing,
            } => {
                metamodelica::gc::MMTrace::mm_accept(dimension, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isProcessing, __mmv)?;
                Ok(())
            }
            Dimension::TYPED_DIMENSION { dimension } => {
                metamodelica::gc::MMTrace::mm_accept(dimension, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::Dimension::{TYPED_DIMENSION, UNTYPED_DIMENSION};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Binding {
    UNBOUND,
    RAW_BINDING {
        bindingExp: metamodelica::Ref<Absyn::Exp>,
        env: Env,
        /// See NFSCodeMod.propagateMod.
        propagatedDims: i32,
        info: SourceInfo,
    },
    UNTYPED_BINDING {
        bindingExp: metamodelica::Ref<DAE::Exp>,
        isProcessing: bool,
        /// See NFSCodeMod.propagateMod.
        propagatedDims: i32,
        info: SourceInfo,
    },
    TYPED_BINDING {
        bindingExp: metamodelica::Ref<DAE::Exp>,
        bindingType: metamodelica::Ref<DAE::Type>,
        /// See NFSCodeMod.propagateMod.
        propagatedDims: i32,
        info: SourceInfo,
    },
}
impl metamodelica::gc::MMTrace for Binding {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Binding::UNBOUND => Ok(()),
            Binding::RAW_BINDING {
                bindingExp,
                env,
                propagatedDims,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(bindingExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(env, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(propagatedDims, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Binding::UNTYPED_BINDING {
                bindingExp,
                isProcessing,
                propagatedDims,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(bindingExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isProcessing, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(propagatedDims, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Binding::TYPED_BINDING {
                bindingExp,
                bindingType,
                propagatedDims,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(bindingExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(bindingType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(propagatedDims, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::Binding::{RAW_BINDING, TYPED_BINDING, UNBOUND, UNTYPED_BINDING};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Component {
    UNTYPED_COMPONENT {
        name: metamodelica::Ref<Absyn::Path>,
        baseType: metamodelica::Ref<DAE::Type>,
        dimensions: metamodelica::Array<Dimension>,
        prefixes: Prefixes,
        paramType: ParamType,
        binding: Binding,
        info: SourceInfo,
    },
    TYPED_COMPONENT {
        name: metamodelica::Ref<Absyn::Path>,
        ty: metamodelica::Ref<DAE::Type>,
        parent: Option<metamodelica::Ref<Component>>,
        prefixes: DaePrefixes,
        binding: Binding,
        info: SourceInfo,
    },
    CONDITIONAL_COMPONENT {
        name: metamodelica::Ref<Absyn::Path>,
        condition: metamodelica::Ref<DAE::Exp>,
        element: metamodelica::Ref<SCode::Element>,
        modifier: metamodelica::Ref<Modifier>,
        prefixes: Prefixes,
        env: Env,
        prefix: Prefix,
        info: SourceInfo,
    },
    DELETED_COMPONENT {
        name: metamodelica::Ref<Absyn::Path>,
    },
    OUTER_COMPONENT {
        name: metamodelica::Ref<Absyn::Path>,
        innerName: Option<metamodelica::Ref<Absyn::Path>>,
    },
    COMPONENT_ALIAS {
        componentName: metamodelica::Ref<Absyn::Path>,
    },
}
impl metamodelica::gc::MMTrace for Component {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Component::UNTYPED_COMPONENT {
                name,
                baseType,
                dimensions,
                prefixes,
                paramType,
                binding,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimensions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(paramType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Component::TYPED_COMPONENT {
                name,
                ty,
                parent,
                prefixes,
                binding,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(parent, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Component::CONDITIONAL_COMPONENT {
                name,
                condition,
                element,
                modifier,
                prefixes,
                env,
                prefix,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(element, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modifier, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(env, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Component::DELETED_COMPONENT { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
            Component::OUTER_COMPONENT { name, innerName } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(innerName, __mmv)?;
                Ok(())
            }
            Component::COMPONENT_ALIAS { componentName } => {
                metamodelica::gc::MMTrace::mm_accept(componentName, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::Component::{
    COMPONENT_ALIAS, CONDITIONAL_COMPONENT, DELETED_COMPONENT, OUTER_COMPONENT, TYPED_COMPONENT, UNTYPED_COMPONENT,
};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Condition {
    SINGLE_CONDITION {
        condition: bool,
    },
    ARRAY_CONDITION {
        conditions: metamodelica::List<metamodelica::Ref<Condition>>,
    },
}
impl metamodelica::gc::MMTrace for Condition {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Condition::SINGLE_CONDITION { condition } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                Ok(())
            }
            Condition::ARRAY_CONDITION { conditions } => {
                metamodelica::gc::MMTrace::mm_accept(conditions, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::Condition::{ARRAY_CONDITION, SINGLE_CONDITION};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum ParamType {
    /// Not a parameter.
    NON_PARAM,
    /// A non-structural parameter.
    NON_STRUCT_PARAM,
    /// A structural parameter.
    STRUCT_PARAM,
}
impl metamodelica::gc::MMTrace for ParamType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ParamType::NON_PARAM => Ok(()),
            ParamType::NON_STRUCT_PARAM => Ok(()),
            ParamType::STRUCT_PARAM => Ok(()),
        }
    }
}
pub(crate) use self::ParamType::{NON_PARAM, NON_STRUCT_PARAM, STRUCT_PARAM};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Modifier {
    MODIFIER {
        name: ArcStr,
        finalPrefix: SCode::Final,
        eachPrefix: SCode::Each,
        binding: Binding,
        subModifiers: metamodelica::List<metamodelica::Ref<Modifier>>,
        info: SourceInfo,
    },
    REDECLARE {
        finalPrefix: SCode::Final,
        eachPrefix: SCode::Each,
        element: metamodelica::Ref<SCode::Element>,
        env: Env,
        r#mod: metamodelica::Ref<Modifier>,
        constrainingClass: Option<metamodelica::Ref<ConstrainingClass>>,
    },
    NOMOD,
}
impl metamodelica::gc::MMTrace for Modifier {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Modifier::MODIFIER {
                name,
                finalPrefix,
                eachPrefix,
                binding,
                subModifiers,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eachPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(subModifiers, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Modifier::REDECLARE {
                finalPrefix,
                eachPrefix,
                element,
                env,
                r#mod,
                constrainingClass,
            } => {
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eachPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(element, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(env, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(r#mod, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(constrainingClass, __mmv)?;
                Ok(())
            }
            Modifier::NOMOD => Ok(()),
        }
    }
}
impl Modifier {
    pub fn interned_NOMOD() -> metamodelica::Ref<Modifier> {
        thread_local! {
            static INTERNED: metamodelica::Ref<Modifier> = metamodelica::Ref::new(Modifier::NOMOD);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_NOMOD() -> metamodelica::Ref<Modifier> {
    Modifier::interned_NOMOD()
}
pub(crate) use self::Modifier::{MODIFIER, NOMOD, REDECLARE};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ConstrainingClass {
    pub classPath: metamodelica::Ref<Absyn::Path>,
    pub r#mod: metamodelica::Ref<Modifier>,
}

impl metamodelica::gc::MMTrace for ConstrainingClass {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.classPath, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.r#mod, __mmv)?;
        Ok(())
    }
}
pub type CONSTRAINING_CLASS = ConstrainingClass;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Prefixes {
    NO_PREFIXES,
    PREFIXES {
        visibility: SCode::Visibility,
        variability: SCode::Variability,
        finalPrefix: SCode::Final,
        innerOuter: Absyn::InnerOuter,
        direction: (Absyn::Direction, SourceInfo),
        connectorType: (SCode::ConnectorType, SourceInfo),
        varArgs: VarArgs,
    },
}
impl metamodelica::gc::MMTrace for Prefixes {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Prefixes::NO_PREFIXES => Ok(()),
            Prefixes::PREFIXES {
                visibility,
                variability,
                finalPrefix,
                innerOuter,
                direction,
                connectorType,
                varArgs,
            } => {
                metamodelica::gc::MMTrace::mm_accept(visibility, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(innerOuter, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(direction, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(connectorType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(varArgs, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::Prefixes::{NO_PREFIXES, PREFIXES};

pub(crate) static DEFAULT_PROTECTED_PREFIXES: std::sync::LazyLock<Prefixes> =
    std::sync::LazyLock::new(|| Prefixes::PREFIXES {
        visibility: openmodelica_frontend_types::SCode::Visibility::PROTECTED,
        variability: openmodelica_frontend_types::SCode::Variability::VAR,
        finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
        innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        direction: (openmodelica_ast::Absyn::Direction::BIDIR, Absyn::dummyInfo.clone()),
        connectorType: (
            openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
            Absyn::dummyInfo.clone(),
        ),
        varArgs: crate::NFInstTypes::VarArgs::NO_VARARG,
    });

pub(crate) static DEFAULT_INPUT_PREFIXES: std::sync::LazyLock<Prefixes> =
    std::sync::LazyLock::new(|| Prefixes::PREFIXES {
        visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC,
        variability: openmodelica_frontend_types::SCode::Variability::VAR,
        finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
        innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        direction: (openmodelica_ast::Absyn::Direction::INPUT, Absyn::dummyInfo.clone()),
        connectorType: (
            openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
            Absyn::dummyInfo.clone(),
        ),
        varArgs: crate::NFInstTypes::VarArgs::NO_VARARG,
    });

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum VarArgs {
    NO_VARARG,
    IS_VARARG,
}
impl metamodelica::gc::MMTrace for VarArgs {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            VarArgs::NO_VARARG => Ok(()),
            VarArgs::IS_VARARG => Ok(()),
        }
    }
}
pub(crate) use self::VarArgs::{IS_VARARG, NO_VARARG};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum DaePrefixes {
    NO_DAE_PREFIXES,
    DAE_PREFIXES {
        visibility: DAE::VarVisibility,
        variability: DAE::VarKind,
        finalPrefix: SCode::Final,
        innerOuter: Absyn::InnerOuter,
        direction: DAE::VarDirection,
        connectorType: metamodelica::Ref<DAE::ConnectorType>,
    },
}
impl metamodelica::gc::MMTrace for DaePrefixes {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            DaePrefixes::NO_DAE_PREFIXES => Ok(()),
            DaePrefixes::DAE_PREFIXES {
                visibility,
                variability,
                finalPrefix,
                innerOuter,
                direction,
                connectorType,
            } => {
                metamodelica::gc::MMTrace::mm_accept(visibility, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(innerOuter, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(direction, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(connectorType, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::DaePrefixes::{DAE_PREFIXES, NO_DAE_PREFIXES};

thread_local! { static __DEFAULT_CONST_DAE_PREFIXES_TLS: DaePrefixes = DaePrefixes::DAE_PREFIXES { visibility: openmodelica_frontend_types::DAE::VarVisibility::PUBLIC, variability: openmodelica_frontend_types::DAE::VarKind::CONST, finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, direction: openmodelica_frontend_types::DAE::VarDirection::BIDIR, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR() }; }
pub(crate) fn DEFAULT_CONST_DAE_PREFIXES() -> DaePrefixes {
    __DEFAULT_CONST_DAE_PREFIXES_TLS.with(|__t| __t.clone())
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Equation {
    EQUALITY_EQUATION {
        /// The left hand side expression.
        lhs: metamodelica::Ref<DAE::Exp>,
        /// The right hand side expression.
        rhs: metamodelica::Ref<DAE::Exp>,
        info: SourceInfo,
    },
    CONNECT_EQUATION {
        /// The left hand side component.
        lhs: metamodelica::Ref<DAE::ComponentRef>,
        /// The type of the lhs component.
        lhsType: metamodelica::Ref<DAE::Type>,
        /// The right hand side component.
        rhs: metamodelica::Ref<DAE::ComponentRef>,
        /// The type of the rhs component.
        rhsType: metamodelica::Ref<DAE::Type>,
        prefix: Prefix,
        info: SourceInfo,
    },
    FOR_EQUATION {
        /// The name of the iterator variable.
        name: ArcStr,
        /// The index of the iterator variable.
        index: i32,
        /// The type of the index/iterator variable.
        indexType: metamodelica::Ref<DAE::Type>,
        /// The range expression to loop over.
        range: Option<metamodelica::Ref<DAE::Exp>>,
        /// The body of the for loop.
        body: metamodelica::List<metamodelica::Ref<Equation>>,
        info: SourceInfo,
    },
    IF_EQUATION {
        /// List of branches, where each branch is a tuple of a condition and a body.
        branches: metamodelica::List<(
            metamodelica::Ref<DAE::Exp>,
            metamodelica::List<metamodelica::Ref<Equation>>,
        )>,
        info: SourceInfo,
    },
    WHEN_EQUATION {
        /// List of branches, where each branch is a tuple of a condition and a body.
        branches: metamodelica::List<(
            metamodelica::Ref<DAE::Exp>,
            metamodelica::List<metamodelica::Ref<Equation>>,
        )>,
        info: SourceInfo,
    },
    ASSERT_EQUATION {
        /// The assert condition.
        condition: metamodelica::Ref<DAE::Exp>,
        /// The message to display if the assert fails.
        message: metamodelica::Ref<DAE::Exp>,
        /// Error or warning
        level: metamodelica::Ref<DAE::Exp>,
        info: SourceInfo,
    },
    TERMINATE_EQUATION {
        /// The message to display if the terminate triggers.
        message: metamodelica::Ref<DAE::Exp>,
        info: SourceInfo,
    },
    REINIT_EQUATION {
        /// The variable to reinitialize.
        cref: metamodelica::Ref<DAE::ComponentRef>,
        /// The new value of the variable.
        reinitExp: metamodelica::Ref<DAE::Exp>,
        info: SourceInfo,
    },
    NORETCALL_EQUATION {
        exp: metamodelica::Ref<DAE::Exp>,
        info: SourceInfo,
    },
}
impl metamodelica::gc::MMTrace for Equation {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Equation::EQUALITY_EQUATION { lhs, rhs, info } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::CONNECT_EQUATION {
                lhs,
                lhsType,
                rhs,
                rhsType,
                prefix,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lhsType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhsType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::FOR_EQUATION {
                name,
                index,
                indexType,
                range,
                body,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(indexType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(range, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::IF_EQUATION { branches, info } => {
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::WHEN_EQUATION { branches, info } => {
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::ASSERT_EQUATION {
                condition,
                message,
                level,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(level, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::TERMINATE_EQUATION { message, info } => {
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::REINIT_EQUATION { cref, reinitExp, info } => {
                metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(reinitExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Equation::NORETCALL_EQUATION { exp, info } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::Equation::{
    ASSERT_EQUATION, CONNECT_EQUATION, EQUALITY_EQUATION, FOR_EQUATION, IF_EQUATION, NORETCALL_EQUATION,
    REINIT_EQUATION, TERMINATE_EQUATION, WHEN_EQUATION,
};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Statement {
    ASSIGN_STMT {
        /// The asignee
        lhs: metamodelica::Ref<DAE::Exp>,
        /// The expression
        rhs: metamodelica::Ref<DAE::Exp>,
        info: SourceInfo,
    },
    /// Used to mark in which order local array variables in functions should be initialized
    FUNCTION_ARRAY_INIT {
        name: ArcStr,
        ty: metamodelica::Ref<DAE::Type>,
        info: SourceInfo,
    },
    FOR_STMT {
        /// The name of the iterator variable.
        name: ArcStr,
        /// The index of the scope of the iterator variable.
        index: i32,
        /// The type of the index/iterator variable.
        indexType: metamodelica::Ref<DAE::Type>,
        /// The range expression to loop over.
        range: Option<metamodelica::Ref<DAE::Exp>>,
        /// The body of the for loop.
        body: metamodelica::List<metamodelica::Ref<Statement>>,
        info: SourceInfo,
    },
    IF_STMT {
        /// List of branches, where each branch is a tuple of a condition and a body.
        branches: metamodelica::List<(
            metamodelica::Ref<DAE::Exp>,
            metamodelica::List<metamodelica::Ref<Statement>>,
        )>,
        info: SourceInfo,
    },
    WHEN_STMT {
        /// List of branches, where each branch is a tuple of a condition and a body.
        branches: metamodelica::List<(
            metamodelica::Ref<DAE::Exp>,
            metamodelica::List<metamodelica::Ref<Statement>>,
        )>,
        info: SourceInfo,
    },
    ASSERT_STMT {
        /// The assert condition.
        condition: metamodelica::Ref<DAE::Exp>,
        /// The message to display if the assert fails.
        message: metamodelica::Ref<DAE::Exp>,
        info: SourceInfo,
    },
    TERMINATE_STMT {
        /// The message to display if the terminate triggers.
        message: metamodelica::Ref<DAE::Exp>,
        info: SourceInfo,
    },
    REINIT_STMT {
        /// The variable to reinitialize.
        cref: metamodelica::Ref<DAE::ComponentRef>,
        /// The new value of the variable.
        reinitExp: metamodelica::Ref<DAE::Exp>,
        info: SourceInfo,
    },
    NORETCALL_STMT {
        exp: metamodelica::Ref<DAE::Exp>,
        info: SourceInfo,
    },
    WHILE_STMT {
        exp: metamodelica::Ref<DAE::Exp>,
        statementLst: metamodelica::List<metamodelica::Ref<Statement>>,
        info: SourceInfo,
    },
    RETURN_STMT {
        info: SourceInfo,
    },
    BREAK_STMT {
        info: SourceInfo,
    },
    FAILURE_STMT {
        body: metamodelica::List<metamodelica::Ref<Statement>>,
        info: SourceInfo,
    },
}
impl metamodelica::gc::MMTrace for Statement {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Statement::ASSIGN_STMT { lhs, rhs, info } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::FUNCTION_ARRAY_INIT { name, ty, info } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::FOR_STMT {
                name,
                index,
                indexType,
                range,
                body,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(indexType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(range, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::IF_STMT { branches, info } => {
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::WHEN_STMT { branches, info } => {
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::ASSERT_STMT {
                condition,
                message,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::TERMINATE_STMT { message, info } => {
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::REINIT_STMT { cref, reinitExp, info } => {
                metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(reinitExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::NORETCALL_STMT { exp, info } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::WHILE_STMT {
                exp,
                statementLst,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(statementLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::RETURN_STMT { info } => {
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::BREAK_STMT { info } => {
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            Statement::FAILURE_STMT { body, info } => {
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::Statement::{
    ASSERT_STMT, ASSIGN_STMT, BREAK_STMT, FAILURE_STMT, FOR_STMT, FUNCTION_ARRAY_INIT, IF_STMT, NORETCALL_STMT,
    REINIT_STMT, RETURN_STMT, TERMINATE_STMT, WHEN_STMT, WHILE_STMT,
};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FunctionSlot {
    pub name: ArcStr,
    pub arg: Option<metamodelica::Ref<DAE::Exp>>,
    pub defaultValue: Option<metamodelica::Ref<DAE::Exp>>,
}

impl metamodelica::gc::MMTrace for FunctionSlot {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.arg, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.defaultValue, __mmv)?;
        Ok(())
    }
}
pub type SLOT = FunctionSlot;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum EntryOrigin {
    /// An entry declared in the local scope.
    LOCAL_ORIGIN,
    /// An entry declared in the builtin scope.
    BUILTIN_ORIGIN,
    /// An entry that has been inherited through an extends clause.
    INHERITED_ORIGIN {
        /// The path of the baseclass the entry was inherited from.
        baseClass: metamodelica::Ref<Absyn::Path>,
        /// The info of the extends clause.
        info: SourceInfo,
        /// The origins of the element in the baseclass.
        origin: metamodelica::List<metamodelica::Ref<EntryOrigin>>,
        /// The environment the entry was inherited from.
        originEnv: Env,
        /// Index used to identify the extends clause for optimization.
        index: i32,
    },
    /// An entry that has replaced another entry through redeclare.
    REDECLARED_ORIGIN {
        /// The replaced entry.
        replacedEntry: metamodelica::Ref<Entry>,
        /// The environment the replacement came from.
        originEnv: Env,
    },
    /// An entry that has been imported with an import statement.
    IMPORTED_ORIGIN {
        imp: Absyn::Import,
        info: SourceInfo,
        /// The environment the entry was imported from.
        originEnv: Env,
    },
}
impl metamodelica::gc::MMTrace for EntryOrigin {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            EntryOrigin::LOCAL_ORIGIN => Ok(()),
            EntryOrigin::BUILTIN_ORIGIN => Ok(()),
            EntryOrigin::INHERITED_ORIGIN {
                baseClass,
                info,
                origin,
                originEnv,
                index,
            } => {
                metamodelica::gc::MMTrace::mm_accept(baseClass, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(origin, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(originEnv, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                Ok(())
            }
            EntryOrigin::REDECLARED_ORIGIN {
                replacedEntry,
                originEnv,
            } => {
                metamodelica::gc::MMTrace::mm_accept(replacedEntry, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(originEnv, __mmv)?;
                Ok(())
            }
            EntryOrigin::IMPORTED_ORIGIN { imp, info, originEnv } => {
                metamodelica::gc::MMTrace::mm_accept(imp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(originEnv, __mmv)?;
                Ok(())
            }
        }
    }
}
impl EntryOrigin {
    pub fn interned_LOCAL_ORIGIN() -> metamodelica::Ref<EntryOrigin> {
        thread_local! {
            static INTERNED: metamodelica::Ref<EntryOrigin> = metamodelica::Ref::new(EntryOrigin::LOCAL_ORIGIN);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_BUILTIN_ORIGIN() -> metamodelica::Ref<EntryOrigin> {
        thread_local! {
            static INTERNED: metamodelica::Ref<EntryOrigin> = metamodelica::Ref::new(EntryOrigin::BUILTIN_ORIGIN);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_LOCAL_ORIGIN() -> metamodelica::Ref<EntryOrigin> {
    EntryOrigin::interned_LOCAL_ORIGIN()
}
pub fn interned_BUILTIN_ORIGIN() -> metamodelica::Ref<EntryOrigin> {
    EntryOrigin::interned_BUILTIN_ORIGIN()
}
pub(crate) use self::EntryOrigin::{
    BUILTIN_ORIGIN, IMPORTED_ORIGIN, INHERITED_ORIGIN, LOCAL_ORIGIN, REDECLARED_ORIGIN,
};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Entry {
    pub name: ArcStr,
    pub element: metamodelica::Ref<SCode::Element>,
    pub r#mod: metamodelica::Ref<Modifier>,
    pub origins: metamodelica::List<metamodelica::Ref<EntryOrigin>>,
}

impl metamodelica::gc::MMTrace for Entry {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.element, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.r#mod, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.origins, __mmv)?;
        Ok(())
    }
}
pub type ENTRY = Entry;

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum ScopeType {
    BUILTIN_SCOPE,
    TOP_SCOPE,
    NORMAL_SCOPE {
        isEncapsulated: bool,
    },
    /// This scope contains one or more iterators; they are made unique by the following index (plus their name)
    IMPLICIT_SCOPE {
        iterIndex: i32,
    },
}
impl metamodelica::gc::MMTrace for ScopeType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ScopeType::BUILTIN_SCOPE => Ok(()),
            ScopeType::TOP_SCOPE => Ok(()),
            ScopeType::NORMAL_SCOPE { isEncapsulated } => {
                metamodelica::gc::MMTrace::mm_accept(isEncapsulated, __mmv)?;
                Ok(())
            }
            ScopeType::IMPLICIT_SCOPE { iterIndex } => {
                metamodelica::gc::MMTrace::mm_accept(iterIndex, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::ScopeType::{BUILTIN_SCOPE, IMPLICIT_SCOPE, NORMAL_SCOPE, TOP_SCOPE};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Frame {
    pub name: Option<ArcStr>,
    pub prefix: Option<metamodelica::Ref<NFInstPrefix::Prefix>>,
    pub scopeType: ScopeType,
    pub entries: metamodelica::Ref<AvlTree>,
}

impl metamodelica::gc::MMTrace for Frame {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.prefix, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.scopeType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.entries, __mmv)?;
        Ok(())
    }
}
pub type FRAME = Frame;

pub type Env = metamodelica::List<metamodelica::Ref<Frame>>;

pub type AvlKey = ArcStr;

pub type AvlValue = metamodelica::Ref<Entry>;

/// The binary tree data structure
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct AvlTree {
    /// Value
    pub value: Option<metamodelica::Ref<AvlTreeValue>>,
    /// height of tree, used for balancing
    pub height: i32,
    /// left subtree
    pub left: Option<metamodelica::Ref<AvlTree>>,
    /// right subtree
    pub right: Option<metamodelica::Ref<AvlTree>>,
}

impl metamodelica::gc::MMTrace for AvlTree {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.value, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.height, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.left, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.right, __mmv)?;
        Ok(())
    }
}
pub type AVLTREENODE = AvlTree;

/// Each node in the binary tree can have a value associated with it.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct AvlTreeValue {
    /// Key
    pub key: AvlKey,
    /// Value
    pub value: AvlValue,
}

impl metamodelica::gc::MMTrace for AvlTreeValue {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.key, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.value, __mmv)?;
        Ok(())
    }
}
pub type AVLTREEVALUE = AvlTreeValue;
