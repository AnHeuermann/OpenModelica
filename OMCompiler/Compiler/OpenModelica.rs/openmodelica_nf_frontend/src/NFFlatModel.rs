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
use crate::NFBackendExtension;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFFlatten as Flatten;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFFunctionInverse as FunctionInverse;
use crate::NFInline as Inline;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Visibility;
use crate::NFScalarize as Scalarize;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::DAE::ElementSource;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NFFlatModel {
    pub name: metamodelica::Ref<Absyn::Path>,
    pub variables: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    pub equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    pub initialEquations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    pub algorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    pub initialAlgorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    pub source: metamodelica::Ref<ElementSource>,
}

impl metamodelica::gc::MMTrace for NFFlatModel {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.variables, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.equations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.initialEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.algorithms, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.initialAlgorithms, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.source, __mmv)?;
        Ok(())
    }
}
impl Default for NFFlatModel {
    fn default() -> Self {
        Self {
            name: Default::default(),
            variables: Default::default(),
            equations: Default::default(),
            initialEquations: Default::default(),
            algorithms: Default::default(),
            initialAlgorithms: Default::default(),
            source: Default::default(),
        }
    }
}

pub type FLAT_MODEL = NFFlatModel;

pub type TypeMap =
    metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>>;

pub(crate) fn mapExp(
    mut flatModel: metamodelica::Ref<NFFlatModel>,
    mut r#fn: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NFFlatModel>> {
    pub type MapFn = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut flatModel: metamodelica::Ref<NFFlatModel> = flatModel;
    assign_field!(
        flatModel.variables = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut v in (flatModel.variables.clone()).into_iter().cloned() {
                let __x = Variable::mapExpShallow(v.clone(), r#fn)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        flatModel.equations = Equation::mapExpList(flatModel.equations.clone(), r#fn)?,
        flatModel.initialEquations = Equation::mapExpList(flatModel.initialEquations.clone(), r#fn)?,
        flatModel.algorithms = Algorithm::mapExpList(flatModel.algorithms.clone(), r#fn)?,
        flatModel.initialAlgorithms = Algorithm::mapExpList(flatModel.initialAlgorithms.clone(), r#fn)?
    );
    Ok(flatModel)
}

pub(crate) fn mapEquations(
    mut flatModel: metamodelica::Ref<NFFlatModel>,
    mut r#fn: &dyn ::std::ops::Fn(metamodelica::Ref<Equation::NFEquation>) -> Result<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::Ref<NFFlatModel>> {
    pub type MapFn = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Equation::NFEquation>) -> Result<metamodelica::Ref<Equation::NFEquation>>
            + 'static,
    >;

    let mut flatModel: metamodelica::Ref<NFFlatModel> = flatModel;
    assign_field!(
        flatModel.equations = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
            for mut eq in (flatModel.equations.clone()).into_iter().cloned() {
                let __x = Equation::map(eq.clone(), r#fn)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        flatModel.initialEquations = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
            for mut eq in (flatModel.initialEquations.clone()).into_iter().cloned() {
                let __x = Equation::map(eq.clone(), r#fn)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(flatModel)
}

pub(crate) fn mapAlgorithms(
    mut flatModel: metamodelica::Ref<NFFlatModel>,
    mut r#fn: &dyn ::std::ops::Fn(metamodelica::Ref<Algorithm::NFAlgorithm>) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>>,
) -> Result<metamodelica::Ref<NFFlatModel>> {
    pub type MapFn = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Algorithm::NFAlgorithm>,
            ) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>>
            + 'static,
    >;

    let mut flatModel: metamodelica::Ref<NFFlatModel> = flatModel;
    assign_field!(
        flatModel.algorithms = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
            for mut alg in (flatModel.algorithms.clone()).into_iter().cloned() {
                let __x = r#fn(alg.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        flatModel.initialAlgorithms = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
            for mut alg in (flatModel.initialAlgorithms.clone()).into_iter().cloned() {
                let __x = r#fn(alg.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(flatModel)
}

pub(crate) fn fullName(mut flatModel: &metamodelica::Ref<NFFlatModel>) -> Result<ArcStr> {
    let mut name: ArcStr = AbsynUtil::pathString(flatModel.name.clone(), literal!("."), true, false)?;
    Ok(name)
}

pub(crate) fn className(mut flatModel: &metamodelica::Ref<NFFlatModel>) -> ArcStr {
    let mut name: ArcStr = AbsynUtil::pathLastIdent(&flatModel.name);
    name
}

pub(crate) fn toString(
    mut flatModel: &metamodelica::Ref<NFFlatModel>,
    mut functions: &metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>,
    mut printBindingTypes: bool,
) -> Result<ArcStr> {
    let mut r#str: ArcStr = IOStream::string(&(toStream(flatModel, functions, printBindingTypes)?))?;
    Ok(r#str)
}

pub(crate) fn printString(
    mut flatModel: &metamodelica::Ref<NFFlatModel>,
    mut functions: &metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>,
    mut printBindingTypes: bool,
) -> Result<()> {
    let mut s: IOStream::IOStream;
    s = toStream(flatModel, functions, printBindingTypes)?;
    IOStream::print(&s, IOStream::stdOutput.clone())?;
    Ok(())
}

pub(crate) fn toStream(
    mut flatModel: &metamodelica::Ref<NFFlatModel>,
    mut functions: &metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>,
    mut printBindingTypes: bool,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream;
    s = IOStream::create(
        literal!("NFFlatModel.toStream"),
        openmodelica_util::IOStream::IOStreamType::LIST,
    )?;
    s = appendStream(flatModel, functions, printBindingTypes, s)?;
    Ok(s)
}

pub(crate) fn appendStream(
    mut flatModel: &metamodelica::Ref<NFFlatModel>,
    mut functions: &metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>,
    mut printBindingTypes: bool,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut name: ArcStr = className(flatModel);
    for mut r#fn in &*Flatten::FunctionTreeImpl::listValues(functions, metamodelica::nil()) {
        s = Function::toStream(metamodelica::AsArg::as_arg(&r#fn), literal!(""), s)?;
        s = IOStream::append(s, literal!(";\n\n"))?;
    }
    s = IOStream::append(s, {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("class "));
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    })?;
    for mut v in &*flatModel.variables.clone() {
        s = Variable::toStream(metamodelica::AsArg::as_arg(&v), literal!("  "), printBindingTypes, s)?;
        s = IOStream::append(s, literal!(";\n"))?;
    }
    if !((flatModel.initialEquations).is_empty()) {
        s = IOStream::append(s, literal!("initial equation\n"))?;
        s = Equation::toStreamList(&flatModel.initialEquations, literal!("  "), s)?;
    }
    if !((flatModel.equations).is_empty()) {
        s = IOStream::append(s, literal!("equation\n"))?;
        s = Equation::toStreamList(&flatModel.equations, literal!("  "), s)?;
    }
    for mut alg in &*flatModel.initialAlgorithms.clone() {
        if !((alg.statements).is_empty()) {
            s = IOStream::append(s, literal!("initial algorithm\n"))?;
            s = Statement::toStreamList(&alg.statements, literal!("  "), s)?;
        }
    }
    for mut alg in &*flatModel.algorithms.clone() {
        if !((alg.statements).is_empty()) {
            s = IOStream::append(s, literal!("algorithm\n"))?;
            s = Statement::toStreamList(&alg.statements, literal!("  "), s)?;
        }
    }
    s = IOStream::append(s, {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("end "));
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*literal!(";\n"));
        ArcStr::from(__mm_s)
    })?;
    Ok(s)
}

pub(crate) fn toFlatString(
    mut flatModel: metamodelica::Ref<NFFlatModel>,
    mut functions: &metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>,
    mut printBindingTypes: bool,
) -> Result<ArcStr> {
    let mut r#str: ArcStr = IOStream::string(&(toFlatStream(flatModel.clone(), functions, printBindingTypes)?))?;
    Ok(r#str)
}

pub(crate) fn printFlatString(
    mut flatModel: metamodelica::Ref<NFFlatModel>,
    mut functions: &metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>,
    mut printBindingTypes: bool,
) -> Result<()> {
    let mut s: IOStream::IOStream;
    s = toFlatStream(flatModel, functions, printBindingTypes)?;
    IOStream::print(&s, IOStream::stdOutput.clone())?;
    Ok(())
}

pub(crate) fn toFlatStream(
    mut flatModel: metamodelica::Ref<NFFlatModel>,
    mut functions: &metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>,
    mut printBindingTypes: bool,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream;
    s = IOStream::create(className(&flatModel), openmodelica_util::IOStream::IOStreamType::LIST)?;
    s = appendFlatStream(flatModel, functions, printBindingTypes, s)?;
    Ok(s)
}

pub(crate) fn appendFlatStream(
    mut flatModel: metamodelica::Ref<NFFlatModel>,
    mut functions: &metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>,
    mut printBindingTypes: bool,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut flat_model: metamodelica::Ref<NFFlatModel> = flatModel.clone();
    let mut name: ArcStr = Util::makeQuotedIdentifier(className(&flatModel))?;
    let mut format: BaseModelica::OutputFormat;
    let mut scalarize: bool;
    let mut funcs: metamodelica::List<metamodelica::Ref<Function::Function>> =
        Flatten::FunctionTreeImpl::listValues(functions, metamodelica::nil());
    format = BaseModelica::formatFromFlags()?;
    scalarize = Flags::isConfigFlagSet(Flags::BASE_MODELICA_OPTIONS.clone(), literal!("scalarize"))?;
    if metamodelica::stringEq(
        &(Flags::getConfigString(Flags::OBFUSCATE.clone())?),
        &(literal!("protected")),
    ) || metamodelica::stringEq(
        &(Flags::getConfigString(Flags::OBFUSCATE.clone())?),
        &(literal!("encrypted")),
    ) {
        flat_model = obfuscate(flat_model)?;
    }
    if BaseModelica::inlineFunctions()? {
        (flat_model, funcs) = inlineFunctions(flat_model)?;
    }
    if scalarize {
        assign_field!(
            flat_model.variables = Scalarize::scalarizeVariables(&flat_model.variables, true)?,
            flat_model.equations = Equation::splitRecordEquations(&flat_model.equations)?
        );
        assign_field!(
            flat_model.equations = Scalarize::scalarizeEquations(&flat_model.equations, true)?,
            flat_model.initialEquations = Equation::splitRecordEquations(&flat_model.initialEquations)?
        );
        assign_field!(
            flat_model.initialEquations = Scalarize::scalarizeEquations(&flat_model.initialEquations, true)?,
            flat_model.algorithms = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
                for mut a in (flat_model.algorithms.clone()).into_iter().cloned() {
                    let __x = Flatten::unrollForStatementsInAlg(a.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            flat_model.initialAlgorithms = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
                for mut a in (flat_model.initialAlgorithms.clone()).into_iter().cloned() {
                    let __x = Flatten::unrollForStatementsInAlg(a.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        );
        flat_model = mapExp(flat_model, &ExpandExp::expandCallArgs)?;
    } else {
        assign_field!(flat_model.variables = reconstructRecordInstances(flat_model.variables.clone())?);
        assign_field!(
            flat_model.variables =
                List::filterOnFalse(flat_model.variables.clone(), &move |__a0: metamodelica::Ref<
                    Variable::NFVariable,
                >| Variable::isEmptyArray(
                    &__a0
                ))?
        );
    }
    if format.moveBindings.clone() {
        flat_model = moveBindings(flat_model)?;
    }
    s = IOStream::append(s, literal!("//! base 0.1.0\n"))?;
    s = IOStream::append(s, {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("package "));
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    })?;
    for mut r#fn in &*funcs {
        if !(Function::isDefaultRecordConstructor(metamodelica::AsArg::as_arg(&r#fn))?
            || Function::isExternalObjectConstructorOrDestructor(metamodelica::AsArg::as_arg(&r#fn))?)
        {
            s = Function::toFlatStream(
                metamodelica::AsArg::as_arg(&r#fn),
                BaseModelica::defaultFormat.clone(),
                literal!("  "),
                s,
                literal!(""),
            )?;
            s = IOStream::append(s, literal!(";\n\n"))?;
        }
    }
    for mut ty in &*collectFlatTypes(&flat_model, &funcs)? {
        s = Type::toFlatDeclarationStream(metamodelica::AsArg::as_arg(&ty), format, literal!("  "), s)?;
        s = IOStream::append(s, literal!(";\n\n"))?;
    }
    s = IOStream::append(s, {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("  model "));
        __mm_s.push_str(&*name);
        ArcStr::from(__mm_s)
    })?;
    s = FlatModelicaUtil::appendElementSourceCommentString(&flat_model.source, s)?;
    s = IOStream::append(s, literal!("\n"))?;
    for mut v in &*flat_model.variables.clone() {
        s = Variable::toFlatStream(
            metamodelica::AsArg::as_arg(&v),
            format,
            literal!("    "),
            printBindingTypes,
            s,
        )?;
        s = IOStream::append(s, literal!(";\n"))?;
    }
    if !((flat_model.initialEquations).is_empty()) {
        s = IOStream::append(s, literal!("  initial equation\n"))?;
        s = Equation::toFlatStreamList(&flat_model.initialEquations, format, literal!("    "), s)?;
    }
    if !((flat_model.equations).is_empty()) {
        s = IOStream::append(s, literal!("  equation\n"))?;
        s = Equation::toFlatStreamList(&flat_model.equations, format, literal!("    "), s)?;
    }
    for mut alg in &*flat_model.initialAlgorithms.clone() {
        if !((alg.statements).is_empty()) {
            s = IOStream::append(s, literal!("  initial algorithm\n"))?;
            s = Statement::toFlatStreamList(&alg.statements, format, literal!("    "), s)?;
        }
    }
    for mut alg in &*flat_model.algorithms.clone() {
        if !((alg.statements).is_empty()) {
            s = IOStream::append(s, literal!("  algorithm\n"))?;
            s = Statement::toFlatStreamList(&alg.statements, format, literal!("    "), s)?;
        }
    }
    s = FlatModelicaUtil::appendElementSourceCommentAnnotation(
        &flat_model.source,
        FlatModelicaUtil::ElementType::ROOT_CLASS.clone(),
        literal!("    "),
        literal!(";\n"),
        s,
    )?;
    s = IOStream::append(s, {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("  end "));
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*literal!(";\n"));
        ArcStr::from(__mm_s)
    })?;
    s = IOStream::append(s, {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("end "));
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*literal!(";\n"));
        ArcStr::from(__mm_s)
    })?;
    Ok(s)
}

pub(crate) fn inlineFunctions(
    mut flatModel: metamodelica::Ref<NFFlatModel>,
) -> Result<(
    metamodelica::Ref<NFFlatModel>,
    metamodelica::List<metamodelica::Ref<Function::Function>>,
)> {
    let mut flatModel: metamodelica::Ref<NFFlatModel> = flatModel;
    let mut remainingFuncs: metamodelica::List<metamodelica::Ref<Function::Function>>;
    let mut funcs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Function::Function>>>;
    funcs = UnorderedSet::new(
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Function::Function>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Function::nameHash(&__a0))
            },
        )
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Function::Function>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Function::Function>,
                  __a1: metamodelica::Ref<Function::Function>|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Function::nameEqual(&__a0, &__a1))
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Function::Function>,
                        metamodelica::Ref<Function::Function>,
                    ) -> Result<bool>
                    + 'static,
            >),
        13,
    );
    flatModel = mapExp(
        flatModel,
        &({
            let __pe_b1: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            > = (std::sync::Arc::new({
                let __pe_b1 = funcs.clone();
                move |__pe_a0| inlineFunctions_traverser(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >);
            move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone())
        }),
    )?;
    remainingFuncs = UnorderedSet::toList(funcs);
    Ok((flatModel, remainingFuncs))
}

pub(crate) fn inlineFunctions_traverser(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut funcs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Function::Function>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    outExp = (match &*exp {
        Expression::CALL { call: __exp_call } => {
            r#fn = Call::typedFunction(metamodelica::AsArg::as_arg(&__exp_call))?;
            if Function::isBuiltin(&r#fn) {
                outExp = exp;
            } else {
                outExp = Inline::inlineCallExp(exp.clone(), true)?;
                if referenceEq(&*(exp), &*(&*outExp)) {
                    collectFunction(r#fn, funcs)?;
                } else {
                    Expression::apply(
                        outExp.clone(),
                        &({
                            let __pe_b1 = funcs;
                            move |__pe_a0| collectFunctions(&__pe_a0, __pe_b1.clone())
                        }),
                    )?;
                }
            }
            outExp
        }
        _ => exp,
    });
    Ok(outExp)
}

pub(crate) fn collectFunctions(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut funcs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Function::Function>>>,
) -> Result<()> {
    let () = (match &**exp {
        Expression::CALL { call: __exp_call } => {
            collectFunction(Call::typedFunction(metamodelica::AsArg::as_arg(&__exp_call))?, funcs)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn collectFunction(
    mut r#fn: metamodelica::Ref<Function::Function>,
    mut funcs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Function::Function>>>,
) -> Result<()> {
    if !(Function::isBuiltin(&r#fn)) {
        UnorderedSet::add(r#fn.clone(), funcs.clone())?;
        for mut fn_der in &*r#fn.derivatives.clone() {
            for mut der_fn in &*Function::getCachedFuncs(InstNode::borrow(fn_der.derivativeFn.clone())?)? {
                UnorderedSet::add(der_fn.clone(), funcs.clone())?;
            }
        }
        let __range0 = r#fn.inverses.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut fn_inv in __range0 {
            UnorderedSet::add(FunctionInverse::getFunction(&fn_inv)?, funcs.clone())?;
        }
    }
    Ok(())
}

pub(crate) fn collectFlatTypes(
    mut flatModel: &metamodelica::Ref<NFFlatModel>,
    mut functions: &metamodelica::List<metamodelica::Ref<Function::Function>>,
) -> Result<metamodelica::List<metamodelica::Ref<Type::NFType>>> {
    let mut outTypes: metamodelica::List<metamodelica::Ref<Type::NFType>>;
    let mut types: TypeMap;
    types = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
        }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Absyn::Path>,
                  __a1: metamodelica::Ref<Absyn::Path>|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    List::map1_0(
        &flatModel.variables,
        &move |__a0: metamodelica::Ref<Variable::NFVariable>,
               __a1: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>,
        >| collectVariableFlatTypes(&__a0, __a1),
        types.clone(),
    )?;
    List::map1_0(
        &flatModel.equations,
        &move |__a0: metamodelica::Ref<Equation::NFEquation>,
               __a1: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>,
        >| collectEquationFlatTypes(&__a0, __a1),
        types.clone(),
    )?;
    List::map1_0(
        &flatModel.initialEquations,
        &move |__a0: metamodelica::Ref<Equation::NFEquation>,
               __a1: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>,
        >| collectEquationFlatTypes(&__a0, __a1),
        types.clone(),
    )?;
    List::map1_0(
        &flatModel.algorithms,
        &move |__a0: metamodelica::Ref<Algorithm::NFAlgorithm>,
               __a1: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>,
        >| collectAlgorithmFlatTypes(&__a0, __a1),
        types.clone(),
    )?;
    List::map1_0(
        &flatModel.initialAlgorithms,
        &move |__a0: metamodelica::Ref<Algorithm::NFAlgorithm>,
               __a1: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>,
        >| collectAlgorithmFlatTypes(&__a0, __a1),
        types.clone(),
    )?;
    List::map1_0(
        functions,
        &move |__a0: metamodelica::Ref<Function::Function>,
               __a1: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>,
        >| collectFunctionFlatTypes(&__a0, __a1),
        types.clone(),
    )?;
    outTypes = UnorderedMap::valueList(types);
    outTypes = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
        for mut ty in (outTypes).into_iter().cloned() {
            let __x = typeFlatType(ty.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outTypes)
}

pub(crate) fn collectVariableFlatTypes(
    mut var: &metamodelica::Ref<Variable::NFVariable>,
    mut types: TypeMap,
) -> Result<()> {
    collectFlatType(&var.ty, types.clone())?;
    collectBindingFlatTypes(&var.binding, types.clone())?;
    for mut attr in &*var.typeAttributes.clone() {
        collectBindingFlatTypes(&(Util::tuple22(attr.clone())), types.clone())?;
    }
    Ok(())
}

pub(crate) fn collectFlatType(mut ty: &metamodelica::Ref<Type::NFType>, mut types: TypeMap) -> Result<()> {
    let () = (::match_deref::match_deref! { match ty {
        Deref @ Type::ENUMERATION { typePath: __ty_typePath, .. } if (!(Type::isBuiltinEnumeration(ty))) => {
            UnorderedMap::tryAdd(__ty_typePath.clone(), ty.clone(), types)?;
            ()
        },
        Deref @ Type::ARRAY { dimensions: __ty_dimensions, elementType: __ty_elementType } => {
            Dimension::foldExpList(metamodelica::AsArg::as_arg(&__ty_dimensions), (std::sync::Arc::new(collectExpFlatTypes_traverse) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>>) -> Result<metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>>> + 'static>), types.clone())?;
            collectFlatType(metamodelica::AsArg::as_arg(&__ty_elementType), types)?;
            ()
        },
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. } => {
            UnorderedMap::tryAdd(InstNode::scopePath(Type::complexNode(ty)?, InstNode::ScopeType::RELATIVE.clone(), false)?, ty.clone(), types)?;
            ()
        },
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. } => {
            UnorderedMap::tryAdd(InstNode::scopePath(Type::complexNode(ty)?, InstNode::ScopeType::RELATIVE.clone(), false)?, ty.clone(), types)?;
            ()
        },
        Deref @ Type::FUNCTION { fnType: Type::FunctionType::FUNCTIONAL_PARAMETER, r#fn: __ty_fn } => {
            UnorderedMap::tryAdd(InstNode::scopePath(InstNode::fromHandle(&__ty_fn.node)?, InstNode::ScopeType::RELATIVE.clone(), false)?, ty.clone(), types)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn collectBindingFlatTypes(
    mut binding: &metamodelica::Ref<Binding::NFBinding>,
    mut types: TypeMap,
) -> Result<()> {
    if Binding::isExplicitlyBound(binding) {
        collectExpFlatTypes(Binding::getTypedExp(binding)?, types)?;
    }
    Ok(())
}

pub(crate) fn collectEquationFlatTypes(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut types: TypeMap,
) -> Result<()> {
    let () = (match &**eq {
        Equation::EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ty: __eq_ty,
            ..
        } => {
            collectExpFlatTypes(__eq_lhs.clone(), types.clone())?;
            collectExpFlatTypes(__eq_rhs.clone(), types.clone())?;
            collectFlatType(metamodelica::AsArg::as_arg(&__eq_ty), types)?;
            ()
        }
        Equation::FOR {
            body: __eq_body,
            range: __eq_range,
            ..
        } => {
            if (__eq_range).is_some() {
                collectExpFlatTypes(__eq_range.clone().ok_or("pattern mismatch")?, types.clone())?;
            }
            List::map1_0(
                metamodelica::AsArg::as_arg(&__eq_body),
                &move |__a0: metamodelica::Ref<Equation::NFEquation>,
                       __a1: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>,
                >| collectEquationFlatTypes(&__a0, __a1),
                types,
            )?;
            ()
        }
        Equation::IF {
            branches: __eq_branches,
            ..
        } => {
            List::map1_0(
                metamodelica::AsArg::as_arg(&__eq_branches),
                &move |__a0: metamodelica::Ref<Equation::Branch::Branch>,
                       __a1: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>,
                >| collectEqBranchFlatTypes(&__a0, __a1),
                types,
            )?;
            ()
        }
        Equation::WHEN {
            branches: __eq_branches,
            ..
        } => {
            List::map1_0(
                metamodelica::AsArg::as_arg(&__eq_branches),
                &move |__a0: metamodelica::Ref<Equation::Branch::Branch>,
                       __a1: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>,
                >| collectEqBranchFlatTypes(&__a0, __a1),
                types,
            )?;
            ()
        }
        Equation::ASSERT {
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            ..
        } => {
            collectExpFlatTypes(__eq_condition.clone(), types.clone())?;
            collectExpFlatTypes(__eq_message.clone(), types.clone())?;
            collectExpFlatTypes(__eq_level.clone(), types)?;
            ()
        }
        Equation::TERMINATE {
            message: __eq_message, ..
        } => {
            collectExpFlatTypes(__eq_message.clone(), types)?;
            ()
        }
        Equation::REINIT {
            reinitExp: __eq_reinitExp,
            ..
        } => {
            collectExpFlatTypes(__eq_reinitExp.clone(), types)?;
            ()
        }
        Equation::NORETCALL { exp: __eq_exp, .. } => {
            collectExpFlatTypes(__eq_exp.clone(), types)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn collectEqBranchFlatTypes(
    mut branch: &metamodelica::Ref<Equation::Branch::Branch>,
    mut types: TypeMap,
) -> Result<()> {
    let () = (match &**branch {
        Equation::Branch::BRANCH {
            body: __branch_body,
            condition: __branch_condition,
            ..
        } => {
            collectExpFlatTypes(__branch_condition.clone(), types.clone())?;
            List::map1_0(
                metamodelica::AsArg::as_arg(&__branch_body),
                &move |__a0: metamodelica::Ref<Equation::NFEquation>,
                       __a1: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>,
                >| collectEquationFlatTypes(&__a0, __a1),
                types,
            )?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn collectAlgorithmFlatTypes(
    mut alg: &metamodelica::Ref<Algorithm::NFAlgorithm>,
    mut types: TypeMap,
) -> Result<()> {
    collectStatementsFlatTypes(&alg.statements, types)?;
    Ok(())
}

pub(crate) fn collectStatementsFlatTypes(
    mut statements: &metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    mut types: TypeMap,
) -> Result<()> {
    for mut s in &**statements {
        collectStatementFlatTypes(metamodelica::AsArg::as_arg(&s), types.clone())?;
    }
    Ok(())
}

pub(crate) fn collectStatementFlatTypes(
    mut stmt: &metamodelica::Ref<Statement::NFStatement>,
    mut types: TypeMap,
) -> Result<()> {
    let () = (match &**stmt {
        Statement::ASSIGNMENT {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            ty: __stmt_ty,
            ..
        } => {
            collectExpFlatTypes(__stmt_lhs.clone(), types.clone())?;
            collectExpFlatTypes(__stmt_rhs.clone(), types.clone())?;
            collectFlatType(metamodelica::AsArg::as_arg(&__stmt_ty), types)?;
            ()
        }
        Statement::FOR {
            body: __stmt_body,
            range: __stmt_range,
            ..
        } => {
            collectStatementsFlatTypes(metamodelica::AsArg::as_arg(&__stmt_body), types.clone())?;
            collectExpFlatTypes(__stmt_range.clone().ok_or("pattern mismatch")?, types)?;
            ()
        }
        Statement::IF {
            branches: __stmt_branches,
            ..
        } => {
            List::map1_0(
                metamodelica::AsArg::as_arg(&__stmt_branches),
                &collectStmtBranchFlatTypes,
                types,
            )?;
            ()
        }
        Statement::WHEN {
            branches: __stmt_branches,
            ..
        } => {
            List::map1_0(
                metamodelica::AsArg::as_arg(&__stmt_branches),
                &collectStmtBranchFlatTypes,
                types,
            )?;
            ()
        }
        Statement::ASSERT {
            condition: __stmt_condition,
            level: __stmt_level,
            message: __stmt_message,
            ..
        } => {
            collectExpFlatTypes(__stmt_condition.clone(), types.clone())?;
            collectExpFlatTypes(__stmt_message.clone(), types.clone())?;
            collectExpFlatTypes(__stmt_level.clone(), types)?;
            ()
        }
        Statement::TERMINATE {
            message: __stmt_message,
            ..
        } => {
            collectExpFlatTypes(__stmt_message.clone(), types)?;
            ()
        }
        Statement::REINIT {
            cref: __stmt_cref,
            reinitExp: __stmt_reinitExp,
            ..
        } => {
            collectExpFlatTypes(__stmt_cref.clone(), types.clone())?;
            collectExpFlatTypes(__stmt_reinitExp.clone(), types)?;
            ()
        }
        Statement::NORETCALL { exp: __stmt_exp, .. } => {
            collectExpFlatTypes(__stmt_exp.clone(), types)?;
            ()
        }
        Statement::WHILE {
            body: __stmt_body,
            condition: __stmt_condition,
            ..
        } => {
            collectExpFlatTypes(__stmt_condition.clone(), types.clone())?;
            collectStatementsFlatTypes(metamodelica::AsArg::as_arg(&__stmt_body), types)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn collectStmtBranchFlatTypes(
    mut branch: (
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    ),
    mut types: TypeMap,
) -> Result<()> {
    collectExpFlatTypes(Util::tuple21(branch.clone()), types.clone())?;
    collectStatementsFlatTypes(&(Util::tuple22(branch)), types)?;
    Ok(())
}

pub(crate) fn collectExpFlatTypes(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut types: TypeMap,
) -> Result<()> {
    Expression::fold(
        exp,
        (std::sync::Arc::new(collectExpFlatTypes_traverse)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        metamodelica::Ref<
                            UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>,
                        >,
                    ) -> Result<
                        metamodelica::Ref<
                            UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Type::NFType>>,
                        >,
                    > + 'static,
            >),
        types,
    )?;
    Ok(())
}

pub(crate) fn collectExpFlatTypes_traverse(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut types: TypeMap,
) -> Result<TypeMap> {
    let mut types: TypeMap = types;
    collectFlatType(&(Expression::typeOf(exp)), types.clone())?;
    Ok(types)
}

pub(crate) fn collectFunctionFlatTypes(
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut types: TypeMap,
) -> Result<()> {
    ClassTree::applyComponents(
        &(Class::classTree(InstNode::getClass(InstNode::fromHandle(&r#fn.node)?)?)?),
        &({
            let __pe_b1 = types.clone();
            move |__pe_a0| collectComponentFlatTypes(&__pe_a0, __pe_b1.clone())
        }),
    )?;
    if !(Function::isExternal(r#fn)?) {
        collectStatementsFlatTypes(&(Function::getBody(r#fn)?), types)?;
    }
    Ok(())
}

pub(crate) fn collectComponentFlatTypes(
    mut component: &metamodelica::Ref<InstNode::InstNode>,
    mut types: TypeMap,
) -> Result<()> {
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    comp = InstNode::component(component)?;
    collectFlatType(&(Component::getType(&comp)?), types.clone())?;
    collectBindingFlatTypes(&(Component::getBinding(&comp)), types)?;
    Ok(())
}

pub(crate) fn reconstructRecordInstances(
    mut variables: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<metamodelica::List<metamodelica::Ref<Variable::NFVariable>>> {
    let mut outVariables: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    let mut rest_vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = variables;
    let mut record_vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut parent_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut parent_ty: metamodelica::Ref<Type::NFType>;
    let mut field_count: i32;
    while !((rest_vars).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_vars) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        var = metamodelica::Own::own(__pa0);
        rest_vars = metamodelica::Own::own(__pa1);
        parent_cr = ComponentRef::rest(&var.name)?;
        if !(ComponentRef::isEmpty(&parent_cr)) {
            parent_ty = ComponentRef::nodeType(&parent_cr)?;
            if Type::isRecord(&parent_ty) {
                field_count = ((Type::recordFields(&parent_ty)).len() as i32);
                (record_vars, rest_vars) = List::split(rest_vars, field_count - 1)?;
                record_vars = metamodelica::cons(var, record_vars);
                var = reconstructRecordInstance(parent_cr, record_vars)?;
            }
        }
        outVariables = metamodelica::cons(var, outVariables);
    }
    outVariables = metamodelica::Dangerous::listReverseInPlace(outVariables);
    Ok(outVariables)
}

pub(crate) fn reconstructRecordInstance(
    mut recordName: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut variables: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut recordVar: metamodelica::Ref<Variable::NFVariable>;
    let mut record_node: metamodelica::Ref<InstNode::InstNode>;
    let mut record_comp: metamodelica::Ref<Component::NFComponent>;
    let mut record_ty: metamodelica::Ref<Type::NFType>;
    let mut field_exps: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut record_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut record_binding: metamodelica::Ref<Binding::NFBinding>;
    record_node = ComponentRef::node(&recordName)?;
    record_comp = InstNode::component(&record_node)?;
    record_ty = ComponentRef::nodeType(&recordName)?;
    field_exps = metamodelica::nil();
    for mut v in &*variables {
        if Binding::hasExp(&v.binding) {
            field_exps = metamodelica::cons(Binding::getExp(&v.binding)?, field_exps);
        } else {
            field_exps = metamodelica::nil();
            break;
        }
    }
    if (field_exps).is_empty() {
        record_binding = Binding::EMPTY_BINDING().clone();
    } else {
        field_exps = metamodelica::Dangerous::listReverseInPlace(field_exps);
        record_exp = Expression::makeRecord(
            InstNode::scopePath(
                InstNode::classScope(record_node.clone())?,
                InstNode::ScopeType::RELATIVE.clone(),
                false,
            )?,
            record_ty.clone(),
            field_exps,
        );
        record_binding = Binding::makeFlat(
            record_exp,
            Component::variability(&record_comp)?,
            Binding::Source::GENERATED.clone(),
            Binding::NO_CONFIDENCE.clone(),
        );
    }
    recordVar = metamodelica::Ref::new(Variable::NFVariable {
        name: recordName,
        ty: record_ty,
        binding: record_binding,
        visibility: InstNode::visibility(&record_node),
        attributes: Component::getAttributes(&record_comp),
        typeAttributes: metamodelica::nil(),
        children: variables,
        comment: Component::comment(&record_comp)?,
        info: InstNode::info(&record_node),
        backendinfo: NFBackendExtension::DUMMY_BACKEND_INFO().clone(),
    });
    Ok(recordVar)
}

pub(crate) fn typeFlatType(mut ty: metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>> {
    let mut ty: metamodelica::Ref<Type::NFType> = ty;
    let () = (::match_deref::match_deref! { match &(&*ty) {
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. } => {
            Typing::typeBindings(Type::complexNode(&ty)?, InstContext::CLASS.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub type ObfuscationMap = metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<InstNode::InstNode>, ArcStr>>;

pub(crate) fn obfuscate(mut flatModel: metamodelica::Ref<NFFlatModel>) -> Result<metamodelica::Ref<NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<NFFlatModel> = flatModel;
    let mut obfuscation_map: ObfuscationMap;
    let mut only_encrypted: bool;
    only_encrypted = metamodelica::stringEq(
        &(Flags::getConfigString(Flags::OBFUSCATE.clone())?),
        &(literal!("encrypted")),
    );
    obfuscation_map = UnorderedMap::new(
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
    for mut v in &*flatModel.variables.clone() {
        addObfuscatedVariable(metamodelica::AsArg::as_arg(&v), only_encrypted, obfuscation_map.clone())?;
    }
    assign_field!(
        flatModel.variables = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut v in (flatModel.variables.clone()).into_iter().cloned() {
                let __x = obfuscateVariable(v.clone(), obfuscation_map.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    flatModel = mapEquations(
        flatModel,
        &({
            let __pe_b1 = obfuscation_map.clone();
            move |__pe_a0| obfuscateEquation(__pe_a0, __pe_b1.clone())
        }),
    )?;
    flatModel = mapAlgorithms(
        flatModel,
        &({
            let __pe_b1 = obfuscation_map;
            move |__pe_a0| obfuscateAlgorithm(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(flatModel)
}

pub(crate) fn addObfuscatedVariable(
    mut var: &metamodelica::Ref<Variable::NFVariable>,
    mut onlyEncrypted: bool,
    mut obfuscationMap: ObfuscationMap,
) -> Result<()> {
    let mut nodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    if Variable::isProtected(var) && (!(onlyEncrypted) || Variable::isEncrypted(var)?) {
        nodes = ComponentRef::nodes(&var.name, metamodelica::nil())?;
        nodes = List::trim(
            nodes,
            &move |__a0: metamodelica::Ref<InstNode::InstNode>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(InstNode::isPublic(&__a0))
            },
        )?;
        for mut node in &*nodes {
            UnorderedMap::tryAdd(
                node.clone(),
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("n"));
                    __mm_s.push_str(&*ArcStr::from(::std::format!(
                        "{}",
                        UnorderedMap::size(obfuscationMap.clone()) + 1
                    )));
                    ArcStr::from(__mm_s)
                },
                obfuscationMap.clone(),
            )?;
        }
    }
    Ok(())
}

pub(crate) fn obfuscateVariable(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    assign_field!(var.name = obfuscateCref(var.name.clone(), obfuscationMap.clone())?.0);
    assign_field!(
        var.comment = obfuscateComment(
            var.comment.clone(),
            &(ComponentRef::node(&var.name)?),
            obfuscationMap.clone(),
            !(Variable::isAccessible(&var)?)
        )?
    );
    var = Variable::mapExpShallow(
        var,
        &({
            let __pe_b1 = obfuscationMap;
            move |__pe_a0| obfuscateExp(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(var)
}

pub(crate) fn obfuscateCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<(metamodelica::Ref<ComponentRef::NFComponentRef>, bool)> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut insideRecord: bool = false;
    let mut name: Option<ArcStr>;
    let mut rest_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let () = (match &*cref {
        ComponentRef::CREF {
            restCref: __cref_restCref,
            ..
        } => {
            (rest_cref, insideRecord) = obfuscateCref(__cref_restCref.clone(), obfuscationMap.clone())?;
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = rest_cref);
            if !(insideRecord) {
                name = UnorderedMap::get(ComponentRef::node(&cref)?, obfuscationMap.clone())?;
                if (name).is_some() {
                    assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; node = ComponentRef::storeNode(InstNode::rename(name.ok_or("pattern mismatch")?, ComponentRef::node(&cref)?)?, false)?);
                }
            }
            insideRecord = InstNode::isRecord(ComponentRef::node(&cref)?)?;
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; subscripts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
                for mut s in (var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone()).into_iter().cloned() {
                    let __x = Subscript::mapShallowExp(s.clone(), &({ let __pe_b1 = obfuscationMap.clone(); move |__pe_a0| obfuscateExp(__pe_a0, __pe_b1.clone()) }))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    Ok((cref, insideRecord))
}

pub(crate) fn obfuscateExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = Expression::map(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = obfuscationMap;
            move |__pe_a0| obfuscateExp_impl(__pe_a0, __pe_b1.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(exp)
}

pub(crate) fn obfuscateExpOpt(
    mut exp: Option<metamodelica::Ref<Expression::NFExpression>>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
    let mut exp: Option<metamodelica::Ref<Expression::NFExpression>> = exp;
    if (exp).is_some() {
        exp = Some(obfuscateExp(exp.ok_or("pattern mismatch")?, obfuscationMap)?);
    }
    Ok(exp)
}

pub(crate) fn obfuscateExp_impl(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let () = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. } => {
            assign_variant_field!(exp => Expression::NFExpression::CREF; cref = obfuscateCref(__exp_cref.clone(), obfuscationMap)?.0);
            ()
        }
        _ => (),
    });
    Ok(exp)
}

pub(crate) fn obfuscateEquation(
    mut eq: metamodelica::Ref<Equation::NFEquation>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut eq: metamodelica::Ref<Equation::NFEquation> = eq;
    eq = Equation::setSource(
        obfuscateSource(Equation::source(&eq), &(Equation::scope(&eq)?), obfuscationMap.clone())?,
        eq,
    );
    eq = Equation::mapExpShallow(
        eq,
        &({
            let __pe_b1 = obfuscationMap;
            move |__pe_a0| obfuscateExp(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(eq)
}

pub(crate) fn obfuscateAlgorithm(
    mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>> {
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm> = alg;
    assign_field!(
        alg.source = obfuscateSource(
            alg.source.clone(),
            &(InstNode::fromCell(alg.scope.clone())?),
            obfuscationMap.clone()
        )?,
        alg.inputs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut e in (alg.inputs.clone()).into_iter().cloned() {
                let __x = (obfuscateCref(e.clone(), obfuscationMap.clone())?).0;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        alg.outputs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut e in (alg.outputs.clone()).into_iter().cloned() {
                let __x = (obfuscateCref(e.clone(), obfuscationMap.clone())?).0;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        alg.statements = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
            for mut s in (alg.statements.clone()).into_iter().cloned() {
                let __x = Statement::map(
                    s.clone(),
                    &({
                        let __pe_b1 = InstNode::fromCell(alg.scope.clone())?;
                        let __pe_b2 = obfuscationMap.clone();
                        move |__pe_a0| obfuscateStatement(__pe_a0, &__pe_b1, __pe_b2.clone())
                    }),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(alg)
}

pub(crate) fn obfuscateStatement(
    mut stmt: metamodelica::Ref<Statement::NFStatement>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<metamodelica::Ref<Statement::NFStatement>> {
    let mut stmt: metamodelica::Ref<Statement::NFStatement> = stmt;
    stmt = Statement::setSource(
        obfuscateSource(Statement::source(&stmt), scope, obfuscationMap.clone())?,
        stmt,
    )?;
    stmt = Statement::mapExpShallow(
        stmt,
        &({
            let __pe_b1 = obfuscationMap;
            move |__pe_a0| obfuscateExp(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(stmt)
}

pub(crate) fn obfuscateSource(
    mut source: metamodelica::Ref<ElementSource>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<metamodelica::Ref<ElementSource>> {
    let mut source: metamodelica::Ref<ElementSource> = source;
    assign_field!(
        source.comment = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Comment>> = metamodelica::nil();
            for mut c in (source.comment.clone()).into_iter().cloned() {
                let __x = obfuscateComment(c.clone(), scope, obfuscationMap.clone(), true)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(source)
}

pub(crate) fn obfuscateCommentOpt(
    mut comment: Option<metamodelica::Ref<SCode::Comment>>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut obfuscationMap: ObfuscationMap,
    mut stripComment: bool,
) -> Result<Option<metamodelica::Ref<SCode::Comment>>> {
    let mut comment: Option<metamodelica::Ref<SCode::Comment>> = comment;
    comment = Util::applyOption(
        comment,
        &({
            let __pe_b1 = scope.clone();
            let __pe_b2 = obfuscationMap;
            let __pe_b3 = stripComment;
            move |__pe_a0| obfuscateComment(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_b3.clone())
        }),
    )?;
    Ok(comment)
}

pub(crate) fn obfuscateComment(
    mut comment: metamodelica::Ref<SCode::Comment>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut obfuscationMap: ObfuscationMap,
    mut stripComment: bool,
) -> Result<metamodelica::Ref<SCode::Comment>> {
    let mut comment: metamodelica::Ref<SCode::Comment> = comment;
    assign_field!(comment.annotation_ = obfuscateAnnotationOpt(comment.annotation_.clone(), scope, obfuscationMap)?);
    if stripComment {
        assign_field!(comment.comment = None);
    }
    Ok(comment)
}

pub(crate) fn obfuscateAnnotationOpt(
    mut ann: Option<metamodelica::Ref<SCode::Annotation>>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<Option<metamodelica::Ref<SCode::Annotation>>> {
    let mut ann: Option<metamodelica::Ref<SCode::Annotation>> = ann;
    ann = Util::applyOption(
        ann,
        &({
            let __pe_b1 = scope.clone();
            let __pe_b2 = obfuscationMap;
            move |__pe_a0| obfuscateAnnotation(__pe_a0, &__pe_b1, __pe_b2.clone())
        }),
    )?;
    Ok(ann)
}

pub(crate) fn obfuscateAnnotation(
    mut ann: metamodelica::Ref<SCode::Annotation>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<metamodelica::Ref<SCode::Annotation>> {
    let mut ann: metamodelica::Ref<SCode::Annotation> = ann;
    assign_field!(ann.modification = obfuscateAnnotationMod(ann.modification.clone(), scope, obfuscationMap)?);
    Ok(ann)
}

pub(crate) fn obfuscateAnnotationMod(
    mut r#mod: metamodelica::Ref<SCode::Mod>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod;
    let () = (match &*r#mod {
        SCode::Mod::MOD {
            subModLst: __mod_subModLst,
            ..
        } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD;
                        subModLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::SubMod>> = metamodelica::nil();
                for mut s in (__mod_subModLst.clone()).into_iter().cloned() {
                    if !(isAllowedAnnotation(&(s.clone()))) { continue; }
                    let __x = obfuscateAnnotationSubMod(s.clone(), scope, obfuscationMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        binding = obfuscateAbsynExpOpt(var_field!((*r#mod).binding, SCode::Mod::MOD).clone(), scope, obfuscationMap)?
                    );
            ()
        }
        _ => (),
    });
    Ok(r#mod)
}

pub(crate) fn isAllowedAnnotation(mut r#mod: &metamodelica::Ref<SCode::SubMod>) -> bool {
    let mut allowed: bool;
    allowed = (::match_deref::match_deref! { match &(r#mod.ident.clone()) {
        Deref @ "Icon" => false,
        Deref @ "Diagram" => false,
        Deref @ "Dialog" => false,
        Deref @ "IconMap" => false,
        Deref @ "DiagramMap" => false,
        Deref @ "Placement" => false,
        Deref @ "Text" => false,
        Deref @ "Line" => false,
        Deref @ "defaultComponentName" => false,
        Deref @ "defaultComponentPrefixes" => false,
        Deref @ "missingInnerMessage" => false,
        Deref @ "obsolete" => false,
        Deref @ "unassignedMessage" => false,
        Deref @ "Protection" => false,
        Deref @ "Authorization" => false,
        _ => !(StringUtil::startsWith(r#mod.ident.clone(), literal!("__"))),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    allowed
}

pub(crate) fn obfuscateAnnotationSubMod(
    mut r#mod: metamodelica::Ref<SCode::SubMod>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<metamodelica::Ref<SCode::SubMod>> {
    let mut r#mod: metamodelica::Ref<SCode::SubMod> = r#mod;
    assign_field!(r#mod.r#mod = obfuscateAnnotationMod(r#mod.r#mod.clone(), scope, obfuscationMap)?);
    Ok(r#mod)
}

pub(crate) fn obfuscateAbsynExpOpt(
    mut exp: Option<metamodelica::Ref<Absyn::Exp>>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<Option<metamodelica::Ref<Absyn::Exp>>> {
    let mut exp: Option<metamodelica::Ref<Absyn::Exp>> = exp;
    exp = Util::applyOption(
        exp,
        &({
            let __pe_b1 = scope.clone();
            let __pe_b2 = obfuscationMap;
            move |__pe_a0| obfuscateAbsynExp(__pe_a0, &__pe_b1, __pe_b2.clone())
        }),
    )?;
    Ok(exp)
}

pub(crate) fn obfuscateAbsynExp(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    (exp, _) = AbsynUtil::traverseExp(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = scope.clone();
            move |__pe_a0, __pe_a2| Ok(obfuscateAbsynExpTraverse(__pe_a0, __pe_b1.clone(), __pe_a2))
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<InstNode::InstNode>, ArcStr>>,
                    ) -> Result<(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<InstNode::InstNode>, ArcStr>>,
                    )> + 'static,
            >),
        obfuscationMap,
    )?;
    Ok(exp)
}

pub(crate) fn obfuscateAbsynExpTraverse(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut obfuscationMap: ObfuscationMap,
) -> (metamodelica::Ref<Absyn::Exp>, ObfuscationMap) {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut obfuscationMap: ObfuscationMap = obfuscationMap;
    let () = (match &*exp {
        Absyn::Exp::CREF {
            componentRef: __exp_componentRef,
        } => {
            assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = obfuscateAbsynCref(__exp_componentRef.clone(), scope, obfuscationMap.clone()));
            ()
        }
        _ => (),
    });
    (exp, obfuscationMap)
}

pub(crate) fn obfuscateAbsynCref(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut obfuscationMap: ObfuscationMap,
) -> metamodelica::Ref<Absyn::ComponentRef> {
    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let mut inst_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut nodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    ErrorExt::setCheckpoint(literal!("NFFlatModel.obfuscateAbsynCref"));
    if '__try0: {
        (inst_cref, _, _) =
            unwrap_break_err!(Lookup::lookupCref(&cref, scope.clone(), InstContext::RELAXED.clone()), '__try0);
        nodes = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
            for mut c in (ComponentRef::toListReverse(&inst_cref, false, metamodelica::nil()))
                .into_iter()
                .cloned()
            {
                let __x = unwrap_break_err!(ComponentRef::node(&(c.clone())), '__try0);
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        cref = unwrap_break_err!(obfuscateAbsynCref2(cref.clone(), &nodes, obfuscationMap.clone()), '__try0);
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    ErrorExt::rollBack(literal!("NFFlatModel.obfuscateAbsynCref"));
    cref
}

pub(crate) fn obfuscateAbsynCref2(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut nodes: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    mut obfuscationMap: ObfuscationMap,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut rest_nodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let () = (::match_deref::match_deref! { match &((cref.clone(), nodes.clone())) {
        (Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. }, _) => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_FULLYQUALIFIED; componentRef = obfuscateAbsynCref2(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), nodes, obfuscationMap)?);
            ()
        },
        (Deref @ Absyn::ComponentRef::CREF_QUAL { .. }, Deref @ metamodelica::ListNode::Cons { head: node, tail: __esc_rest_nodes }) if (metamodelica::stringEq(&(InstNode::name(metamodelica::AsArg::as_arg(&node))?), &var_field!((*cref).name, Absyn::ComponentRef::CREF_QUAL))) => {
            rest_nodes = (*__esc_rest_nodes).clone();
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL;
                name = UnorderedMap::getOrDefault(node.clone(), obfuscationMap.clone(), var_field!((*cref).name, Absyn::ComponentRef::CREF_QUAL).clone())?,
                componentRef = obfuscateAbsynCref2(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), metamodelica::AsArg::as_arg(&rest_nodes), obfuscationMap)?
            );
            ()
        },
        (Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, Deref @ metamodelica::ListNode::Cons { head: node, tail: _ }) if (metamodelica::stringEq(&(InstNode::name(metamodelica::AsArg::as_arg(&node))?), &var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT))) => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; name = UnorderedMap::getOrDefault(node.clone(), obfuscationMap, var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub(crate) fn hasArrayConnections(mut flatModel: &metamodelica::Ref<NFFlatModel>, mut minSize: i32) -> Result<bool> {
    let mut hasArrays: bool = false;
    for mut eq in &*flatModel.equations.clone() {
        if Equation::contains(
            eq.clone(),
            &move |__a0: metamodelica::Ref<Equation::NFEquation>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Equation::isConnect(&__a0))
            },
        )? && Equation::sizeOf(metamodelica::AsArg::as_arg(&eq)) >= minSize
        {
            hasArrays = true;
            return Ok(hasArrays);
        }
    }
    Ok(hasArrays)
}

pub(crate) fn removeNonTopLevelDirections(
    mut flatModel: metamodelica::Ref<NFFlatModel>,
) -> Result<metamodelica::Ref<NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<NFFlatModel> = flatModel;
    if Flags::getConfigBool(Flags::USE_LOCAL_DIRECTION.clone())? {
        return Ok(flatModel);
    }
    assign_field!(
        flatModel.variables = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut v in (flatModel.variables.clone()).into_iter().cloned() {
                let __x = Variable::removeNonTopLevelDirection(v.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(flatModel)
}

pub(crate) fn moveBindings(mut flatModel: metamodelica::Ref<NFFlatModel>) -> Result<metamodelica::Ref<NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<NFFlatModel> = flatModel;
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    let mut eqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    for mut var in &*flatModel.variables.clone() {
        let mut var = var.clone();
        (var, eqs) = Variable::moveBinding(var, eqs)?;
        vars = metamodelica::cons(var, vars);
    }
    if !((eqs).is_empty()) {
        assign_field!(
            flatModel.variables = metamodelica::Dangerous::listReverseInPlace(vars),
            flatModel.equations = listAppend(
                metamodelica::Dangerous::listReverseInPlace(eqs),
                flatModel.equations.clone()
            )
        );
    }
    Ok(flatModel)
}
