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

use crate::HashTableCrefSimVar;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::List;

// public imports
// private imports
/// Root data structure containing information required for templates to
///  generate C functions for Modelica/MetaModelica functions.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FunctionCode {
    pub name: ArcStr,
    /// This function is special; the 'in'-function should be generated for it
    pub mainFunction: Option<metamodelica::Ref<Function::Function>>,
    pub functions: metamodelica::List<metamodelica::Ref<Function::Function>>,
    /// shared literals
    pub literals: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    pub externalFunctionIncludes: metamodelica::List<ArcStr>,
    pub makefileParams: MakefileParams,
    pub extraRecordDecls: metamodelica::List<RecordDeclaration>,
}

impl metamodelica::gc::MMTrace for FunctionCode {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.mainFunction, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.functions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.literals, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.externalFunctionIncludes, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.makefileParams, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.extraRecordDecls, __mmv)?;
        Ok(())
    }
}
pub type FUNCTIONCODE = FunctionCode;

// TODO: I believe some of these fields can be removed. Check to see what is
//       used in templates.
pub mod Function {
    use super::*;
    /// Represents a Modelica, MetaModelica or external function.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Function {
        FUNCTION {
            name: metamodelica::Ref<Absyn::Path>,
            outVars: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            functionArguments: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            variableDeclarations: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            body: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
            visibility: SCode::Visibility,
            info: SourceInfo,
        },
        PARALLEL_FUNCTION {
            name: metamodelica::Ref<Absyn::Path>,
            outVars: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            functionArguments: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            variableDeclarations: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            body: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
            info: SourceInfo,
        },
        KERNEL_FUNCTION {
            name: metamodelica::Ref<Absyn::Path>,
            outVars: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            functionArguments: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            variableDeclarations: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            body: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
            info: SourceInfo,
        },
        EXTERNAL_FUNCTION {
            name: metamodelica::Ref<Absyn::Path>,
            extName: ArcStr,
            funArgs: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            extArgs: metamodelica::List<metamodelica::Ref<SimExtArg::SimExtArg>>,
            extReturn: metamodelica::Ref<SimExtArg::SimExtArg>,
            inVars: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            outVars: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            biVars: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            /// this one is needed so that we know if we should generate the external function prototype or not
            includes: metamodelica::List<ArcStr>,
            /// need this one for C#
            libs: metamodelica::List<ArcStr>,
            /// C or Fortran
            language: ArcStr,
            visibility: SCode::Visibility,
            info: SourceInfo,
            dynamicLoad: bool,
        },
        RECORD_CONSTRUCTOR {
            name: metamodelica::Ref<Absyn::Path>,
            funArgs: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            locals: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
            visibility: SCode::Visibility,
            info: SourceInfo,
        },
    }
    impl metamodelica::gc::MMTrace for Function {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Function::FUNCTION {
                    name,
                    outVars,
                    functionArguments,
                    variableDeclarations,
                    body,
                    visibility,
                    info,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(outVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(functionArguments, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(variableDeclarations, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(visibility, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                    Ok(())
                }
                Function::PARALLEL_FUNCTION {
                    name,
                    outVars,
                    functionArguments,
                    variableDeclarations,
                    body,
                    info,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(outVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(functionArguments, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(variableDeclarations, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                    Ok(())
                }
                Function::KERNEL_FUNCTION {
                    name,
                    outVars,
                    functionArguments,
                    variableDeclarations,
                    body,
                    info,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(outVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(functionArguments, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(variableDeclarations, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                    Ok(())
                }
                Function::EXTERNAL_FUNCTION {
                    name,
                    extName,
                    funArgs,
                    extArgs,
                    extReturn,
                    inVars,
                    outVars,
                    biVars,
                    includes,
                    libs,
                    language,
                    visibility,
                    info,
                    dynamicLoad,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(extName, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(funArgs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(extArgs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(extReturn, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(inVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(outVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(biVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(includes, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(libs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(language, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(visibility, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(dynamicLoad, __mmv)?;
                    Ok(())
                }
                Function::RECORD_CONSTRUCTOR {
                    name,
                    funArgs,
                    locals,
                    visibility,
                    info,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(funArgs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(locals, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(visibility, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for Function {
        fn default() -> Self {
            Self::RECORD_CONSTRUCTOR {
                name: Default::default(),
                funArgs: Default::default(),
                locals: Default::default(),
                visibility: Default::default(),
                info: Default::default(),
            }
        }
    }
    pub use self::Function::{EXTERNAL_FUNCTION, FUNCTION, KERNEL_FUNCTION, PARALLEL_FUNCTION, RECORD_CONSTRUCTOR};
    pub(crate) fn toString(mut func: &metamodelica::Ref<Function>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        r#str = ({
            let mut tmp: ArcStr = literal!("");
            (match &**func {
                FUNCTION { name: __func_name, .. } => {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("name: "));
                        __mm_s.push_str(&*AbsynUtil::pathString(
                            __func_name.clone(),
                            literal!("."),
                            true,
                            false,
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("FUNCTION("));
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    }
                }
                PARALLEL_FUNCTION { name: __func_name, .. } => {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("name: "));
                        __mm_s.push_str(&*AbsynUtil::pathString(
                            __func_name.clone(),
                            literal!("."),
                            true,
                            false,
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("PARALLEL_FUNCTION("));
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    }
                }
                KERNEL_FUNCTION { name: __func_name, .. } => {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("name: "));
                        __mm_s.push_str(&*AbsynUtil::pathString(
                            __func_name.clone(),
                            literal!("."),
                            true,
                            false,
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("KERNEL_FUNCTION("));
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    }
                }
                EXTERNAL_FUNCTION {
                    biVars: __func_biVars,
                    extArgs: __func_extArgs,
                    extName: __func_extName,
                    extReturn: __func_extReturn,
                    funArgs: __func_funArgs,
                    inVars: __func_inVars,
                    includes: __func_includes,
                    language: __func_language,
                    libs: __func_libs,
                    name: __func_name,
                    outVars: __func_outVars,
                    ..
                } => {
                    let mut ls: metamodelica::List<ArcStr>;
                    tmp = literal!("\n");
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("  name: "));
                        __mm_s.push_str(&*AbsynUtil::pathString(
                            __func_name.clone(),
                            literal!("."),
                            true,
                            false,
                        )?);
                        __mm_s.push_str(&*literal!(",\n"));
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("  extName: "));
                        __mm_s.push_str(&*__func_extName);
                        __mm_s.push_str(&*literal!(",\n"));
                        ArcStr::from(__mm_s)
                    };
                    ls = List::map(__func_funArgs.clone(), &move |__a0: metamodelica::Ref<
                        Variable::Variable,
                    >| {
                        Variable::toString(&__a0)
                    })?;
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("  funArgs: {"));
                        __mm_s.push_str(&*stringDelimitList(ls, literal!(", ")));
                        __mm_s.push_str(&*literal!("},\n"));
                        ArcStr::from(__mm_s)
                    };
                    ls = List::map(__func_extArgs.clone(), &move |__a0: metamodelica::Ref<
                        SimExtArg::SimExtArg,
                    >| {
                        SimExtArg::toString(&__a0)
                    })?;
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("  extArgs: {"));
                        __mm_s.push_str(&*stringDelimitList(ls, literal!(", ")));
                        __mm_s.push_str(&*literal!("},\n"));
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("  extReturn: "));
                        __mm_s.push_str(&*SimExtArg::toString(metamodelica::AsArg::as_arg(&__func_extReturn))?);
                        __mm_s.push_str(&*literal!(",\n"));
                        ArcStr::from(__mm_s)
                    };
                    ls = List::map(__func_inVars.clone(), &move |__a0: metamodelica::Ref<
                        Variable::Variable,
                    >| Variable::toString(&__a0))?;
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("  inVars: {"));
                        __mm_s.push_str(&*stringDelimitList(ls, literal!(", ")));
                        __mm_s.push_str(&*literal!("},\n"));
                        ArcStr::from(__mm_s)
                    };
                    ls = List::map(__func_outVars.clone(), &move |__a0: metamodelica::Ref<
                        Variable::Variable,
                    >| {
                        Variable::toString(&__a0)
                    })?;
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("  outVars: {"));
                        __mm_s.push_str(&*stringDelimitList(ls, literal!(", ")));
                        __mm_s.push_str(&*literal!("},\n"));
                        ArcStr::from(__mm_s)
                    };
                    ls = List::map(__func_biVars.clone(), &move |__a0: metamodelica::Ref<
                        Variable::Variable,
                    >| Variable::toString(&__a0))?;
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("  biVars: {"));
                        __mm_s.push_str(&*stringDelimitList(ls, literal!(", ")));
                        __mm_s.push_str(&*literal!("},\n"));
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("  includes: {"));
                        __mm_s.push_str(&*stringDelimitList(__func_includes.clone(), literal!(", ")));
                        __mm_s.push_str(&*literal!("},\n"));
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("  libs: {"));
                        __mm_s.push_str(&*stringDelimitList(__func_libs.clone(), literal!(", ")));
                        __mm_s.push_str(&*literal!("},\n"));
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("  language: "));
                        __mm_s.push_str(&*__func_language);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    };
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("EXTERNAL_FUNCTION("));
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    }
                }
                RECORD_CONSTRUCTOR { name: __func_name, .. } => {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("name: "));
                        __mm_s.push_str(&*AbsynUtil::pathString(
                            __func_name.clone(),
                            literal!("."),
                            true,
                            false,
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("RECORD_CONSTRUCTOR("));
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    }
                }
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("SimCodeFunction.Function.toString"));
                            __mm_s.push_str(&*literal!(" failed for an unknown reason."));
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            })
        });
        Ok(r#str)
    }
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum RecordDeclaration {
    RECORD_DECL_FULL {
        /// struct (record) name ? encoded
        name: ArcStr,
        /// alias of struct (record) name ? encoded. Code generators can generate an aliasing typedef using this, and avoid problems when casting a record from one type to another (*(othertype*)(&var)), which only works if you have a lhs value.
        aliasName: Option<ArcStr>,
        /// definition path
        defPath: metamodelica::Ref<Absyn::Path>,
        /// only name and type
        variables: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
        /// If the record is passed to an external function at any point, we need to generate conversion functions for it (for instance to convert 'modelica_integer' to 'int')
        usedExternally: bool,
    },
    RECORD_DECL_ADD_CONSTRCTOR {
        /// A unique name for the new constor. e.g. R_1_3() if it needs the 1st an 3rd members as inputs
        ctor_name: ArcStr,
        /// The record's name
        name: ArcStr,
        /// The members with the ones that need outisde binding marked. e.g 1st and 3rd elements will have bind_from_outside=true
        variables: metamodelica::List<metamodelica::Ref<Variable::Variable>>,
        /// definition path
        defPath: metamodelica::Ref<Absyn::Path>,
        usedExternally: bool,
    },
    RECORD_DECL_DEF {
        /// definition path .. encoded?
        path: metamodelica::Ref<Absyn::Path>,
        fieldNames: metamodelica::List<ArcStr>,
    },
}
impl metamodelica::gc::MMTrace for RecordDeclaration {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            RecordDeclaration::RECORD_DECL_FULL {
                name,
                aliasName,
                defPath,
                variables,
                usedExternally,
            } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(aliasName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(defPath, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variables, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(usedExternally, __mmv)?;
                Ok(())
            }
            RecordDeclaration::RECORD_DECL_ADD_CONSTRCTOR {
                ctor_name,
                name,
                variables,
                defPath,
                usedExternally,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ctor_name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variables, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(defPath, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(usedExternally, __mmv)?;
                Ok(())
            }
            RecordDeclaration::RECORD_DECL_DEF { path, fieldNames } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fieldNames, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for RecordDeclaration {
    fn default() -> Self {
        Self::RECORD_DECL_DEF {
            path: Default::default(),
            fieldNames: Default::default(),
        }
    }
}
pub use self::RecordDeclaration::{RECORD_DECL_ADD_CONSTRCTOR, RECORD_DECL_DEF, RECORD_DECL_FULL};

/// Platform specific parameters used when generating makefiles.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct MakefileParams {
    pub ccompiler: ArcStr,
    pub cxxcompiler: ArcStr,
    pub linker: ArcStr,
    pub exeext: ArcStr,
    pub dllext: ArcStr,
    pub omhome: ArcStr,
    pub cflags: ArcStr,
    pub ldflags: ArcStr,
    /// Libraries that are required by the runtime library
    pub runtimelibs: ArcStr,
    pub includes: metamodelica::List<ArcStr>,
    pub libs: metamodelica::List<ArcStr>,
    pub libPaths: metamodelica::List<ArcStr>,
    pub platform: ArcStr,
    pub compileDir: ArcStr,
}

impl metamodelica::gc::MMTrace for MakefileParams {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.ccompiler, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.cxxcompiler, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.linker, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.exeext, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.dllext, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.omhome, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.cflags, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ldflags, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.runtimelibs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.includes, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.libs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.libPaths, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.platform, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.compileDir, __mmv)?;
        Ok(())
    }
}
impl Default for MakefileParams {
    fn default() -> Self {
        Self {
            ccompiler: Default::default(),
            cxxcompiler: Default::default(),
            linker: Default::default(),
            exeext: Default::default(),
            dllext: Default::default(),
            omhome: Default::default(),
            cflags: Default::default(),
            ldflags: Default::default(),
            runtimelibs: Default::default(),
            includes: Default::default(),
            libs: Default::default(),
            libPaths: Default::default(),
            platform: Default::default(),
            compileDir: Default::default(),
        }
    }
}

pub type MAKEFILE_PARAMS = MakefileParams;

pub mod SimExtArg {
    use super::*;
    /// Information about an argument to an external function.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum SimExtArg {
        SIMEXTARG {
            cref: metamodelica::Ref<DAE::ComponentRef>,
            isInput: bool,
            /// > 0 if output
            outputIndex: i32,
            isArray: bool,
            /// avoid double allocation
            hasBinding: bool,
            type_: metamodelica::Ref<DAE::Type>,
        },
        SIMEXTARGEXP {
            exp: metamodelica::Ref<DAE::Exp>,
            type_: metamodelica::Ref<DAE::Type>,
        },
        SIMEXTARGSIZE {
            cref: metamodelica::Ref<DAE::ComponentRef>,
            isInput: bool,
            /// > 0 if output
            outputIndex: i32,
            type_: metamodelica::Ref<DAE::Type>,
            exp: metamodelica::Ref<DAE::Exp>,
        },
        SIMNOEXTARG,
    }
    impl metamodelica::gc::MMTrace for SimExtArg {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                SimExtArg::SIMEXTARG {
                    cref,
                    isInput,
                    outputIndex,
                    isArray,
                    hasBinding,
                    type_,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(isInput, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(outputIndex, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(isArray, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(hasBinding, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                    Ok(())
                }
                SimExtArg::SIMEXTARGEXP { exp, type_ } => {
                    metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                    Ok(())
                }
                SimExtArg::SIMEXTARGSIZE {
                    cref,
                    isInput,
                    outputIndex,
                    type_,
                    exp,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(isInput, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(outputIndex, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                    Ok(())
                }
                SimExtArg::SIMNOEXTARG => Ok(()),
            }
        }
    }
    impl SimExtArg {
        pub fn interned_SIMNOEXTARG() -> metamodelica::Ref<SimExtArg> {
            thread_local! {
                static INTERNED: metamodelica::Ref<SimExtArg> = metamodelica::Ref::new(SimExtArg::SIMNOEXTARG);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_SIMNOEXTARG() -> metamodelica::Ref<SimExtArg> {
        SimExtArg::interned_SIMNOEXTARG()
    }
    impl Default for SimExtArg {
        fn default() -> Self {
            Self::SIMNOEXTARG
        }
    }
    pub use self::SimExtArg::{SIMEXTARG, SIMEXTARGEXP, SIMEXTARGSIZE, SIMNOEXTARG};
    pub(crate) fn toString(mut simExtArg: &metamodelica::Ref<SimExtArg>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        r#str = ({
            let mut tmp: ArcStr = literal!("");
            (match &**simExtArg {
                SIMEXTARG {
                    cref: __simExtArg_cref,
                    hasBinding: __simExtArg_hasBinding,
                    isArray: __simExtArg_isArray,
                    isInput: __simExtArg_isInput,
                    outputIndex: __simExtArg_outputIndex,
                    type_: __simExtArg_type_,
                } => {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("cref: "));
                        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
                            metamodelica::AsArg::as_arg(&__simExtArg_cref),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    tmp = if (__simExtArg_isInput.clone()) {
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*tmp);
                            __mm_s.push_str(&*literal!(", isInput: true"));
                            ArcStr::from(__mm_s)
                        }
                    } else {
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*tmp);
                            __mm_s.push_str(&*literal!(", isInput: false"));
                            ArcStr::from(__mm_s)
                        }
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(", outputIndex: "));
                        __mm_s.push_str(&*intString(__simExtArg_outputIndex.clone()));
                        ArcStr::from(__mm_s)
                    };
                    tmp = if (__simExtArg_isArray.clone()) {
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*tmp);
                            __mm_s.push_str(&*literal!(", isArray: true"));
                            ArcStr::from(__mm_s)
                        }
                    } else {
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*tmp);
                            __mm_s.push_str(&*literal!(", isArray: false"));
                            ArcStr::from(__mm_s)
                        }
                    };
                    tmp = if (__simExtArg_hasBinding.clone()) {
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*tmp);
                            __mm_s.push_str(&*literal!(", hasBinding: true"));
                            ArcStr::from(__mm_s)
                        }
                    } else {
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*tmp);
                            __mm_s.push_str(&*literal!(", hasBinding: false"));
                            ArcStr::from(__mm_s)
                        }
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(", type: "));
                        __mm_s.push_str(&*TypesDump::unparseType(__simExtArg_type_.clone())?);
                        ArcStr::from(__mm_s)
                    };
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("SIMEXTARG("));
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    }
                }
                SIMEXTARGEXP {
                    exp: __simExtArg_exp,
                    type_: __simExtArg_type_,
                } => {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("exp: "));
                        __mm_s.push_str(&*ExpressionBasics::printExpStr(__simExtArg_exp.clone())?);
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(", type: "));
                        __mm_s.push_str(&*TypesDump::unparseType(__simExtArg_type_.clone())?);
                        ArcStr::from(__mm_s)
                    };
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("SIMEXTARGEXP("));
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    }
                }
                SIMEXTARGSIZE {
                    cref: __simExtArg_cref,
                    exp: __simExtArg_exp,
                    isInput: __simExtArg_isInput,
                    outputIndex: __simExtArg_outputIndex,
                    type_: __simExtArg_type_,
                } => {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("cref: "));
                        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
                            metamodelica::AsArg::as_arg(&__simExtArg_cref),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    tmp = if (__simExtArg_isInput.clone()) {
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*tmp);
                            __mm_s.push_str(&*literal!(", isInput: true"));
                            ArcStr::from(__mm_s)
                        }
                    } else {
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*tmp);
                            __mm_s.push_str(&*literal!(", isInput: false"));
                            ArcStr::from(__mm_s)
                        }
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(", outputIndex: "));
                        __mm_s.push_str(&*intString(__simExtArg_outputIndex.clone()));
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(", type: "));
                        __mm_s.push_str(&*TypesDump::unparseType(__simExtArg_type_.clone())?);
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(", exp: "));
                        __mm_s.push_str(&*ExpressionBasics::printExpStr(__simExtArg_exp.clone())?);
                        ArcStr::from(__mm_s)
                    };
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("SIMEXTARGSIZE("));
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    }
                }
                SIMNOEXTARG { .. } => {
                    literal!("SIMNOEXTARG()")
                }
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("SimCodeFunction.SimExtArg.toString"));
                            __mm_s.push_str(&*literal!(" failed for an unknown reason."));
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            })
        });
        Ok(r#str)
    }
}

pub mod Variable {
    use super::*;
    /// A variable represents a name, a type and a possible default value
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Variable {
        VARIABLE {
            name: metamodelica::Ref<DAE::ComponentRef>,
            ty: metamodelica::Ref<DAE::Type>,
            /// default value
            value: Option<metamodelica::Ref<DAE::Exp>>,
            instDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
            parallelism: DAE::VarParallelism,
            kind: DAE::VarKind,
            bind_from_outside: bool,
        },
        FUNCTION_PTR {
            name: ArcStr,
            tys: metamodelica::List<metamodelica::Ref<DAE::Type>>,
            args: metamodelica::List<metamodelica::Ref<Variable>>,
            /// default value
            defaultValue: Option<metamodelica::Ref<DAE::Exp>>,
        },
    }
    impl metamodelica::gc::MMTrace for Variable {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Variable::VARIABLE {
                    name,
                    ty,
                    value,
                    instDims,
                    parallelism,
                    kind,
                    bind_from_outside,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(instDims, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(parallelism, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(kind, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(bind_from_outside, __mmv)?;
                    Ok(())
                }
                Variable::FUNCTION_PTR {
                    name,
                    tys,
                    args,
                    defaultValue,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(tys, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(args, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(defaultValue, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for Variable {
        fn default() -> Self {
            Self::FUNCTION_PTR {
                name: Default::default(),
                tys: Default::default(),
                args: Default::default(),
                defaultValue: Default::default(),
            }
        }
    }
    pub use self::Variable::{FUNCTION_PTR, VARIABLE};
    pub(crate) fn toString(mut variable: &metamodelica::Ref<Variable>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        r#str = ({
            let mut tmp: ArcStr = literal!("");
            (match &**variable {
                VARIABLE {
                    name: __variable_name,
                    ty: __variable_ty,
                    ..
                } => {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("name: "));
                        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
                            metamodelica::AsArg::as_arg(&__variable_name),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(", type: "));
                        __mm_s.push_str(&*TypesDump::unparseType(__variable_ty.clone())?);
                        ArcStr::from(__mm_s)
                    };
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("VARIABLE("));
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    }
                }
                FUNCTION_PTR {
                    name: __variable_name, ..
                } => {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!("name: "));
                        __mm_s.push_str(&*__variable_name);
                        ArcStr::from(__mm_s)
                    };
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("FUNCTION_PTR("));
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    }
                }
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("SimCodeFunction.Variable.toString"));
                            __mm_s.push_str(&*literal!(" failed for an unknown reason."));
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            })
        });
        Ok(r#str)
    }
}

/// Constants of this type defined below are used by templates to be able to
///  generate different code depending on the context it is generated in.
#[derive(Clone, metamodelica::MMCtor, metamodelica::ReferenceEq)]
pub enum Context {
    SIMULATION_CONTEXT {
        genDiscrete: bool,
    },
    FUNCTION_CONTEXT {
        cref_prefix: ArcStr,
        is_parallel: bool,
    },
    ALGLOOP_CONTEXT {
        genInitialisation: bool,
        genJacobian: bool,
    },
    JACOBIAN_CONTEXT {
        name: ArcStr,
        jacHT: Option<(
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<SimCodeVar::SimVar>,
                    )>,
                >,
            ),
            i32,
            (
                HashTableCrefSimVar::FuncHashCref,
                HashTableCrefSimVar::FuncCrefEqual,
                HashTableCrefSimVar::FuncCrefStr,
                HashTableCrefSimVar::FuncExpStr,
            ),
        )>,
    },
    OTHER_CONTEXT,
    ZEROCROSSINGS_CONTEXT,
    OPTIMIZATION_CONTEXT,
    FMI_CONTEXT,
    DAE_MODE_CONTEXT,
    OMSI_CONTEXT {
        /// used to get local SimVars and corresponding value references
        hashTable: Option<(
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<SimCodeVar::SimVar>,
                    )>,
                >,
            ),
            i32,
            (
                HashTableCrefSimVar::FuncHashCref,
                HashTableCrefSimVar::FuncCrefEqual,
                HashTableCrefSimVar::FuncCrefStr,
                HashTableCrefSimVar::FuncExpStr,
            ),
        )>,
    },
}
impl metamodelica::gc::MMTrace for Context {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Context::SIMULATION_CONTEXT { genDiscrete } => {
                metamodelica::gc::MMTrace::mm_accept(genDiscrete, __mmv)?;
                Ok(())
            }
            Context::FUNCTION_CONTEXT {
                cref_prefix,
                is_parallel,
            } => {
                metamodelica::gc::MMTrace::mm_accept(cref_prefix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(is_parallel, __mmv)?;
                Ok(())
            }
            Context::ALGLOOP_CONTEXT {
                genInitialisation,
                genJacobian,
            } => {
                metamodelica::gc::MMTrace::mm_accept(genInitialisation, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(genJacobian, __mmv)?;
                Ok(())
            }
            Context::JACOBIAN_CONTEXT { name, jacHT } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(jacHT, __mmv)?;
                Ok(())
            }
            Context::OTHER_CONTEXT => Ok(()),
            Context::ZEROCROSSINGS_CONTEXT => Ok(()),
            Context::OPTIMIZATION_CONTEXT => Ok(()),
            Context::FMI_CONTEXT => Ok(()),
            Context::DAE_MODE_CONTEXT => Ok(()),
            Context::OMSI_CONTEXT { hashTable } => {
                metamodelica::gc::MMTrace::mm_accept(hashTable, __mmv)?;
                Ok(())
            }
        }
    }
}
impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::SIMULATION_CONTEXT {
                    genDiscrete: __l_genDiscrete,
                },
                Self::SIMULATION_CONTEXT {
                    genDiscrete: __r_genDiscrete,
                },
            ) => __l_genDiscrete == __r_genDiscrete,
            (
                Self::FUNCTION_CONTEXT {
                    cref_prefix: __l_cref_prefix,
                    is_parallel: __l_is_parallel,
                },
                Self::FUNCTION_CONTEXT {
                    cref_prefix: __r_cref_prefix,
                    is_parallel: __r_is_parallel,
                },
            ) => __l_cref_prefix == __r_cref_prefix && __l_is_parallel == __r_is_parallel,
            (
                Self::ALGLOOP_CONTEXT {
                    genInitialisation: __l_genInitialisation,
                    genJacobian: __l_genJacobian,
                },
                Self::ALGLOOP_CONTEXT {
                    genInitialisation: __r_genInitialisation,
                    genJacobian: __r_genJacobian,
                },
            ) => __l_genInitialisation == __r_genInitialisation && __l_genJacobian == __r_genJacobian,
            (
                Self::JACOBIAN_CONTEXT {
                    name: __l_name,
                    jacHT: __l_jacHT,
                },
                Self::JACOBIAN_CONTEXT {
                    name: __r_name,
                    jacHT: __r_jacHT,
                },
            ) => {
                __l_name == __r_name
                    && (match (__l_jacHT, __r_jacHT) {
                        (Some(__lo), Some(__ro)) => {
                            (match (__lo, __ro) {
                                ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                                    (__lt0 == __rt0)
                                        && (__lt1 == __rt1)
                                        && (__lt2 == __rt2)
                                        && (match (__lt3, __rt3) {
                                            ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                                                std::sync::Arc::ptr_eq(__lt0, __rt0)
                                                    && std::sync::Arc::ptr_eq(__lt1, __rt1)
                                                    && std::sync::Arc::ptr_eq(__lt2, __rt2)
                                                    && std::sync::Arc::ptr_eq(__lt3, __rt3)
                                            }
                                        })
                                }
                            })
                        }
                        (None, None) => true,
                        _ => false,
                    })
            }
            (Self::OTHER_CONTEXT, Self::OTHER_CONTEXT) => true,
            (Self::ZEROCROSSINGS_CONTEXT, Self::ZEROCROSSINGS_CONTEXT) => true,
            (Self::OPTIMIZATION_CONTEXT, Self::OPTIMIZATION_CONTEXT) => true,
            (Self::FMI_CONTEXT, Self::FMI_CONTEXT) => true,
            (Self::DAE_MODE_CONTEXT, Self::DAE_MODE_CONTEXT) => true,
            (
                Self::OMSI_CONTEXT {
                    hashTable: __l_hashTable,
                },
                Self::OMSI_CONTEXT {
                    hashTable: __r_hashTable,
                },
            ) => {
                (match (__l_hashTable, __r_hashTable) {
                    (Some(__lo), Some(__ro)) => {
                        (match (__lo, __ro) {
                            ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                                (__lt0 == __rt0)
                                    && (__lt1 == __rt1)
                                    && (__lt2 == __rt2)
                                    && (match (__lt3, __rt3) {
                                        ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                                            std::sync::Arc::ptr_eq(__lt0, __rt0)
                                                && std::sync::Arc::ptr_eq(__lt1, __rt1)
                                                && std::sync::Arc::ptr_eq(__lt2, __rt2)
                                                && std::sync::Arc::ptr_eq(__lt3, __rt3)
                                        }
                                    })
                            }
                        })
                    }
                    (None, None) => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
impl Eq for Context {}
impl PartialOrd for Context {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Context {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn __variant_idx(__v: &Context) -> u32 {
            match __v {
                Context::SIMULATION_CONTEXT { .. } => 0,
                Context::FUNCTION_CONTEXT { .. } => 1,
                Context::ALGLOOP_CONTEXT { .. } => 2,
                Context::JACOBIAN_CONTEXT { .. } => 3,
                Context::OTHER_CONTEXT => 4,
                Context::ZEROCROSSINGS_CONTEXT => 5,
                Context::OPTIMIZATION_CONTEXT => 6,
                Context::FMI_CONTEXT => 7,
                Context::DAE_MODE_CONTEXT => 8,
                Context::OMSI_CONTEXT { .. } => 9,
            }
        }
        match __variant_idx(self).cmp(&__variant_idx(other)) {
            std::cmp::Ordering::Equal => {}
            non_eq => return non_eq,
        }
        match (self, other) {
            (
                Self::SIMULATION_CONTEXT {
                    genDiscrete: __l_genDiscrete,
                },
                Self::SIMULATION_CONTEXT {
                    genDiscrete: __r_genDiscrete,
                },
            ) => __l_genDiscrete.cmp(__r_genDiscrete),
            (
                Self::FUNCTION_CONTEXT {
                    cref_prefix: __l_cref_prefix,
                    is_parallel: __l_is_parallel,
                },
                Self::FUNCTION_CONTEXT {
                    cref_prefix: __r_cref_prefix,
                    is_parallel: __r_is_parallel,
                },
            ) => __l_cref_prefix
                .cmp(__r_cref_prefix)
                .then_with(|| __l_is_parallel.cmp(__r_is_parallel)),
            (
                Self::ALGLOOP_CONTEXT {
                    genInitialisation: __l_genInitialisation,
                    genJacobian: __l_genJacobian,
                },
                Self::ALGLOOP_CONTEXT {
                    genInitialisation: __r_genInitialisation,
                    genJacobian: __r_genJacobian,
                },
            ) => __l_genInitialisation
                .cmp(__r_genInitialisation)
                .then_with(|| __l_genJacobian.cmp(__r_genJacobian)),
            (
                Self::JACOBIAN_CONTEXT {
                    name: __l_name,
                    jacHT: __l_jacHT,
                },
                Self::JACOBIAN_CONTEXT {
                    name: __r_name,
                    jacHT: __r_jacHT,
                },
            ) => __l_name.cmp(__r_name).then_with(|| {
                (match (__l_jacHT, __r_jacHT) {
                    (Some(__lo), Some(__ro)) => {
                        (match (__lo, __ro) {
                            ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => __lt0
                                .cmp(__rt0)
                                .then_with(|| __lt1.cmp(__rt1))
                                .then_with(|| __lt2.cmp(__rt2))
                                .then_with(|| {
                                    (match (__lt3, __rt3) {
                                        ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                                            (std::sync::Arc::as_ptr(__lt0) as *const ())
                                                .cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ()))
                                                .then_with(|| {
                                                    (std::sync::Arc::as_ptr(__lt1) as *const ())
                                                        .cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ()))
                                                })
                                                .then_with(|| {
                                                    (std::sync::Arc::as_ptr(__lt2) as *const ())
                                                        .cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ()))
                                                })
                                                .then_with(|| {
                                                    (std::sync::Arc::as_ptr(__lt3) as *const ())
                                                        .cmp(&(std::sync::Arc::as_ptr(__rt3) as *const ()))
                                                })
                                        }
                                    })
                                }),
                        })
                    }
                    (None, None) => std::cmp::Ordering::Equal,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    (Some(_), None) => std::cmp::Ordering::Greater,
                })
            }),
            (Self::OTHER_CONTEXT, Self::OTHER_CONTEXT) => std::cmp::Ordering::Equal,
            (Self::ZEROCROSSINGS_CONTEXT, Self::ZEROCROSSINGS_CONTEXT) => std::cmp::Ordering::Equal,
            (Self::OPTIMIZATION_CONTEXT, Self::OPTIMIZATION_CONTEXT) => std::cmp::Ordering::Equal,
            (Self::FMI_CONTEXT, Self::FMI_CONTEXT) => std::cmp::Ordering::Equal,
            (Self::DAE_MODE_CONTEXT, Self::DAE_MODE_CONTEXT) => std::cmp::Ordering::Equal,
            (
                Self::OMSI_CONTEXT {
                    hashTable: __l_hashTable,
                },
                Self::OMSI_CONTEXT {
                    hashTable: __r_hashTable,
                },
            ) => {
                (match (__l_hashTable, __r_hashTable) {
                    (Some(__lo), Some(__ro)) => {
                        (match (__lo, __ro) {
                            ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => __lt0
                                .cmp(__rt0)
                                .then_with(|| __lt1.cmp(__rt1))
                                .then_with(|| __lt2.cmp(__rt2))
                                .then_with(|| {
                                    (match (__lt3, __rt3) {
                                        ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                                            (std::sync::Arc::as_ptr(__lt0) as *const ())
                                                .cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ()))
                                                .then_with(|| {
                                                    (std::sync::Arc::as_ptr(__lt1) as *const ())
                                                        .cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ()))
                                                })
                                                .then_with(|| {
                                                    (std::sync::Arc::as_ptr(__lt2) as *const ())
                                                        .cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ()))
                                                })
                                                .then_with(|| {
                                                    (std::sync::Arc::as_ptr(__lt3) as *const ())
                                                        .cmp(&(std::sync::Arc::as_ptr(__rt3) as *const ()))
                                                })
                                        }
                                    })
                                }),
                        })
                    }
                    (None, None) => std::cmp::Ordering::Equal,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    (Some(_), None) => std::cmp::Ordering::Greater,
                })
            }
            _ => unreachable!("variant-index equality already implies same variant"),
        }
    }
}
impl std::fmt::Debug for Context {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SIMULATION_CONTEXT {
                genDiscrete: __d_genDiscrete,
            } => {
                let mut __ds = __f.debug_struct("SIMULATION_CONTEXT");
                __ds.field("genDiscrete", __d_genDiscrete);
                __ds.finish()
            }
            Self::FUNCTION_CONTEXT {
                cref_prefix: __d_cref_prefix,
                is_parallel: __d_is_parallel,
            } => {
                let mut __ds = __f.debug_struct("FUNCTION_CONTEXT");
                __ds.field("cref_prefix", __d_cref_prefix);
                __ds.field("is_parallel", __d_is_parallel);
                __ds.finish()
            }
            Self::ALGLOOP_CONTEXT {
                genInitialisation: __d_genInitialisation,
                genJacobian: __d_genJacobian,
            } => {
                let mut __ds = __f.debug_struct("ALGLOOP_CONTEXT");
                __ds.field("genInitialisation", __d_genInitialisation);
                __ds.field("genJacobian", __d_genJacobian);
                __ds.finish()
            }
            Self::JACOBIAN_CONTEXT {
                name: __d_name,
                jacHT: __d_jacHT,
            } => {
                let mut __ds = __f.debug_struct("JACOBIAN_CONTEXT");
                __ds.field("name", __d_name);
                __ds.field("jacHT", &format_args!("<dyn-fn-container@{:p}>", __d_jacHT as *const _));
                __ds.finish()
            }
            Self::OTHER_CONTEXT => __f.debug_struct("OTHER_CONTEXT").finish(),
            Self::ZEROCROSSINGS_CONTEXT => __f.debug_struct("ZEROCROSSINGS_CONTEXT").finish(),
            Self::OPTIMIZATION_CONTEXT => __f.debug_struct("OPTIMIZATION_CONTEXT").finish(),
            Self::FMI_CONTEXT => __f.debug_struct("FMI_CONTEXT").finish(),
            Self::DAE_MODE_CONTEXT => __f.debug_struct("DAE_MODE_CONTEXT").finish(),
            Self::OMSI_CONTEXT {
                hashTable: __d_hashTable,
            } => {
                let mut __ds = __f.debug_struct("OMSI_CONTEXT");
                __ds.field(
                    "hashTable",
                    &format_args!("<dyn-fn-container@{:p}>", __d_hashTable as *const _),
                );
                __ds.finish()
            }
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::OTHER_CONTEXT
    }
}
pub use self::Context::{
    ALGLOOP_CONTEXT, DAE_MODE_CONTEXT, FMI_CONTEXT, FUNCTION_CONTEXT, JACOBIAN_CONTEXT, OMSI_CONTEXT,
    OPTIMIZATION_CONTEXT, OTHER_CONTEXT, SIMULATION_CONTEXT, ZEROCROSSINGS_CONTEXT,
};

pub const fn contextSimulationNonDiscrete() -> Context {
    Context::SIMULATION_CONTEXT { genDiscrete: false }
}

pub const fn contextSimulationDiscrete() -> Context {
    Context::SIMULATION_CONTEXT { genDiscrete: true }
}

pub const fn contextFunction() -> Context {
    Context::FUNCTION_CONTEXT {
        cref_prefix: literal!(""),
        is_parallel: false,
    }
}

pub const fn contextJacobian() -> Context {
    Context::JACOBIAN_CONTEXT {
        name: literal!(""),
        jacHT: None,
    }
}

pub const fn contextAlgloopJacobian() -> Context {
    Context::ALGLOOP_CONTEXT {
        genInitialisation: false,
        genJacobian: true,
    }
}

pub(crate) const fn contextAlgloopInitialisation() -> Context {
    Context::ALGLOOP_CONTEXT {
        genInitialisation: true,
        genJacobian: false,
    }
}

pub const fn contextAlgloop() -> Context {
    Context::ALGLOOP_CONTEXT {
        genInitialisation: false,
        genJacobian: false,
    }
}

pub const fn contextOther() -> Context {
    crate::SimCodeFunction::Context::OTHER_CONTEXT
}

pub const fn contextParallelFunction() -> Context {
    Context::FUNCTION_CONTEXT {
        cref_prefix: literal!(""),
        is_parallel: true,
    }
}

pub const fn contextZeroCross() -> Context {
    crate::SimCodeFunction::Context::ZEROCROSSINGS_CONTEXT
}

pub(crate) const fn contextOptimization() -> Context {
    crate::SimCodeFunction::Context::OPTIMIZATION_CONTEXT
}

pub(crate) const fn contextFMI() -> Context {
    crate::SimCodeFunction::Context::FMI_CONTEXT
}

pub const fn contextDAEmode() -> Context {
    crate::SimCodeFunction::Context::DAE_MODE_CONTEXT
}

pub(crate) const fn contextOMSI() -> Context {
    Context::OMSI_CONTEXT { hashTable: None }
}

thread_local! { static __listExpLength1_TLS: metamodelica::List<metamodelica::Ref<DAE::Exp>> = list![metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 })]; }
pub fn listExpLength1() -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    __listExpLength1_TLS.with(|__t| __t.clone())
}

thread_local! { static __boxedRecordOutVars_TLS: metamodelica::List<metamodelica::Ref<Variable::Variable>> = metamodelica::cons(metamodelica::Ref::new(Variable::Variable::VARIABLE { name: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!(""), identType: DAE::T_COMPLEX_DEFAULT_RECORD().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_COMPLEX_DEFAULT_RECORD().clone(), value: None, instDims: metamodelica::nil(), parallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, kind: openmodelica_frontend_types::DAE::VarKind::VARIABLE, bind_from_outside: false }), metamodelica::nil()); }
pub fn boxedRecordOutVars() -> metamodelica::List<metamodelica::Ref<Variable::Variable>> {
    __boxedRecordOutVars_TLS.with(|__t| __t.clone())
}
