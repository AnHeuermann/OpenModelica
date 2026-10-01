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
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn dumpFlatModelDebug(
    mut stage: ArcStr,
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
    mut functions: &metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree>,
) -> Result<()> {
    let mut flat_model: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel.clone();
    if metamodelica::stringEq(
        &(Flags::getConfigString(Flags::OBFUSCATE.clone())?),
        &(literal!("protected")),
    ) || metamodelica::stringEq(
        &(Flags::getConfigString(Flags::OBFUSCATE.clone())?),
        &(literal!("encrypted")),
    ) {
        flat_model = FlatModel::obfuscate(flat_model)?;
    }
    if Flags::isConfigFlagSet(Flags::DUMP_FLAT_MODEL.clone(), stage.clone())?
        || (Flags::getConfigStringList(Flags::DUMP_FLAT_MODEL.clone())?).is_empty()
    {
        flat_model = combineSubscripts(flatModel)?;
        metamodelica::print(literal!("########################################\n"));
        metamodelica::print(stage);
        metamodelica::print(literal!("\n########################################\n\n"));
        if Flags::getConfigBool(Flags::BASE_MODELICA.clone())? {
            FlatModel::printFlatString(flat_model, functions, false)?;
        } else {
            FlatModel::printString(&flat_model, functions, false)?;
        }
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

pub(crate) fn combineSubscripts(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    if Flags::isSet(Flags::COMBINE_SUBSCRIPTS.clone())? {
        flatModel = FlatModel::mapExp(flatModel, &combineSubscriptsExp)?;
    }
    Ok(flatModel)
}

pub(crate) fn combineSubscriptsExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    fn traverser(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let () = (match &*exp {
            Expression::CREF { cref: __exp_cref, .. } => {
                assign_variant_field!(exp => Expression::NFExpression::CREF; cref = ComponentRef::combineSubscripts(__exp_cref.clone())?);
                ()
            }
            _ => (),
        });
        Ok(exp)
    }

    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = Expression::map(
        exp,
        (std::sync::Arc::new(traverser)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(exp)
}

pub(crate) fn printStructuralParameters(mut flatModel: &metamodelica::Ref<FlatModel::NFFlatModel>) -> Result<()> {
    let mut params: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut names: metamodelica::List<ArcStr>;
    if Flags::isSet(Flags::PRINT_STRUCTURAL.clone())? {
        params = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut v in (flatModel.variables.clone()).into_iter().cloned() {
                if !(Variable::isStructural(&(v.clone()))) {
                    continue;
                }
                let __x = v.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        if !((params).is_empty()) {
            names = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut v in (params).into_iter().cloned() {
                    let __x = ComponentRef::toString(&(v.name.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            Error::addMessage(
                Error::NOTIFY_FRONTEND_STRUCTURAL_PARAMETERS.clone(),
                list![stringDelimitList(names, literal!(", "))],
            )?;
        }
    }
    Ok(())
}

pub(crate) fn dumpFlatModel(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
    mut functions: &metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree>,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut flat_model: metamodelica::Ref<FlatModel::NFFlatModel>;
    flat_model = combineSubscripts(flatModel)?;
    r#str = FlatModel::toFlatString(flat_model, functions, false)?;
    Ok(r#str)
}

pub(crate) fn replaceEmptyArrays(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    flatModel = FlatModel::mapExp(flatModel, &replaceEmptyArraysExp)?;
    Ok(flatModel)
}

pub(crate) fn replaceEmptyArraysExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    fn traverser(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut outExp: metamodelica::Ref<Expression::NFExpression>;
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
        let mut ty: metamodelica::Ref<Type::NFType>;
        outExp = (match &*exp {
            Expression::CREF { cref, ty: __exp_ty }
                if (ComponentRef::isEmptyArray(metamodelica::AsArg::as_arg(&cref))?) =>
            {
                let mut cref = (*cref).clone();
                if ComponentRef::hasSubscripts(metamodelica::AsArg::as_arg(&cref))? {
                    cref = ComponentRef::fillSubscripts(cref.clone());
                    cref = ComponentRef::replaceWholeSubscripts(cref.clone())?;
                    subs = ComponentRef::subscriptsAllFlat(metamodelica::AsArg::as_arg(&cref))?;
                    cref = ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&cref));
                    ty = ComponentRef::getSubscriptedType(metamodelica::AsArg::as_arg(&cref), false)?;
                } else {
                    subs = metamodelica::nil();
                    ty = __exp_ty.clone();
                }
                outExp = Expression::makeEmptyArray(ty)?;
                if !((subs).is_empty()) {
                    outExp = metamodelica::Ref::new(Expression::NFExpression::SUBSCRIPTED_EXP {
                        exp: outExp,
                        subscripts: subs,
                        ty: __exp_ty.clone(),
                        split: false,
                    });
                }
                outExp
            }
            _ => exp,
        });
        Ok(outExp)
    }

    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = Expression::map(
        exp,
        (std::sync::Arc::new(traverser)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(exp)
}

pub(crate) fn expandSlicedCrefs(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
    mut functions: metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree>,
) -> Result<(
    metamodelica::Ref<FlatModel::NFFlatModel>,
    metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree>,
)> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut functions: metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree> = functions;
    if Flags::isSet(Flags::COMBINE_SUBSCRIPTS.clone())? || !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
        return Ok((flatModel, functions));
    }
    assign_field!(
        flatModel.variables = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut v in (flatModel.variables.clone()).into_iter().cloned() {
                let __x = Variable::mapExpShallow(v.clone(), &expandSlicedCrefsExp)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    flatModel = FlatModel::mapEquations(flatModel, &expandSlicedCrefsEq)?;
    flatModel = FlatModel::mapAlgorithms(flatModel, &expandSlicedCrefsAlg)?;
    functions = NFFlatten::FunctionTreeImpl::map(
        functions,
        &move |__a0: metamodelica::Ref<Absyn::Path>, __a1: metamodelica::Ref<Function::Function>| {
            expandSlicedCrefsFunction(&__a0, __a1)
        },
    )?;
    Ok((flatModel, functions))
}

pub(crate) fn addTrailingWholeIndices(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. }
            if (ComponentRef::hasImplicitTrailingIndex(metamodelica::AsArg::as_arg(&__exp_cref))) =>
        {
            assign_variant_field!(exp => Expression::NFExpression::CREF; cref = ComponentRef::fillSubscripts(__exp_cref.clone()));
            exp
        }
        _ => exp,
    });
    exp
}

pub(crate) fn expandSlicedCrefsExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::CREF {
            cref: __exp_cref,
            ty: __exp_ty,
        } if (ComponentRef::isSliced(metamodelica::AsArg::as_arg(&__exp_cref))?) => {
            expandSlicedCrefsExp2(__exp_cref.clone(), __exp_ty.clone())?
        }
        _ => exp,
    });
    Ok(exp)
}

pub(crate) fn expandSlicedCrefsExp2(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut iterators: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>;
    (cr, iterators) = ComponentRef::iterate(cref.clone())?;
    outExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: metamodelica::Ref::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR {
            ty: ty,
            var: ComponentRef::variability(&cref)?,
            purity: ComponentRef::purity(&cref)?,
            exp: Expression::fromCref(cr, false)?,
            iters: iterators,
        }),
    });
    Ok(outExp)
}

pub(crate) fn expandSlicedCrefsEq(
    mut eq: metamodelica::Ref<Equation::NFEquation>,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut eq: metamodelica::Ref<Equation::NFEquation> = eq;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut eq2: metamodelica::Ref<Equation::NFEquation>;
    eq = (match &*eq {
        Equation::EQUALITY { rhs: __esc_e1, .. } => {
            e1 = (*__esc_e1).clone();
            e1 = Expression::map(
                e1.clone(),
                (std::sync::Arc::new(fnptr!(
                    addTrailingWholeIndices,
                    metamodelica::Ref<Expression::NFExpression>
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            e2 = Expression::map(
                e1.clone(),
                (std::sync::Arc::new(expandSlicedCrefsExp)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            if !(referenceEq(&*(e1.clone()), &*(&*e2))) {
                assign_variant_field!(eq => Equation::NFEquation::EQUALITY; rhs = e2);
            }
            eq
        }
        _ => {
            eq2 = Equation::mapExpShallow(
                eq,
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    > = (std::sync::Arc::new(fnptr!(
                        addTrailingWholeIndices,
                        metamodelica::Ref<Expression::NFExpression>
                    ))
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >);
                    move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone())
                }),
            )?;
            Equation::mapExpShallow(
                eq2,
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    > = (std::sync::Arc::new(expandSlicedCrefsExp)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >);
                    move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone())
                }),
            )?
        }
    });
    Ok(eq)
}

pub(crate) fn expandSlicedCrefsAlg(
    mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>,
) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>> {
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm> = alg;
    assign_field!(
        alg.statements = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
            for mut s in (alg.statements.clone()).into_iter().cloned() {
                let __x = Statement::map(s.clone(), &expandSlicedCrefsStmt)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(alg)
}

pub(crate) fn expandSlicedCrefsStmt(
    mut stmt: metamodelica::Ref<Statement::NFStatement>,
) -> Result<metamodelica::Ref<Statement::NFStatement>> {
    let mut stmt: metamodelica::Ref<Statement::NFStatement> = stmt;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    stmt = (match &*stmt {
        Statement::ASSIGNMENT {
            rhs: __esc_e1,
            lhs: __stmt_lhs,
            ..
        } => {
            e1 = (*__esc_e1).clone();
            assign_variant_field!(stmt => Statement::NFStatement::ASSIGNMENT; lhs = Expression::map(__stmt_lhs.clone(), (std::sync::Arc::new(fnptr!(addTrailingWholeIndices, metamodelica::Ref<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?);
            e1 = Expression::map(
                e1.clone(),
                (std::sync::Arc::new(fnptr!(
                    addTrailingWholeIndices,
                    metamodelica::Ref<Expression::NFExpression>
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            e2 = Expression::map(
                e1.clone(),
                (std::sync::Arc::new(expandSlicedCrefsExp)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            if !(referenceEq(&*(e1.clone()), &*(&*e2))) {
                assign_variant_field!(stmt => Statement::NFStatement::ASSIGNMENT; rhs = e2);
            }
            stmt
        }
        _ => {
            let mut stmt2: metamodelica::Ref<Statement::NFStatement>;
            stmt2 = Statement::mapExpShallow(
                stmt,
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    > = (std::sync::Arc::new(fnptr!(
                        addTrailingWholeIndices,
                        metamodelica::Ref<Expression::NFExpression>
                    ))
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >);
                    move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone())
                }),
            )?;
            Statement::mapExpShallow(
                stmt2,
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    > = (std::sync::Arc::new(expandSlicedCrefsExp)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >);
                    move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone())
                }),
            )?
        }
    });
    Ok(stmt)
}

pub(crate) fn expandSlicedCrefsFunction(
    mut fnPath: &metamodelica::Ref<Absyn::Path>,
    mut r#fn: metamodelica::Ref<Function::Function>,
) -> Result<metamodelica::Ref<Function::Function>> {
    let mut r#fn: metamodelica::Ref<Function::Function> = r#fn;
    r#fn = Function::mapExp(
        r#fn,
        (std::sync::Arc::new({
            let __pe_b1: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            > = (std::sync::Arc::new(expandSlicedCrefsExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >);
            move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
        (std::sync::Arc::new({
            let __pe_b1: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            > = (std::sync::Arc::new(expandSlicedCrefsExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >);
            move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
        true,
        false,
    )?;
    r#fn = Function::mapBody(r#fn, &expandSlicedCrefsAlg)?;
    Ok(r#fn)
}

pub type MergeNameMap = metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<Absyn::ComponentRef>>>;

pub fn makeMergeNameMap() -> MergeNameMap {
    let mut nameMap: MergeNameMap = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    nameMap
}

pub(crate) fn mergeScalars(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut isRootClass: bool,
    mut nameMap: MergeNameMap,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut elem: metamodelica::Ref<SCode::Element>;
    if !(Flags::isSet(Flags::MERGE_COMPONENTS.clone())?) {
        return Ok(node);
    }
    elem = InstNode::definition(node.clone())?;
    elem = mergeScalars2(elem, classPath, isRootClass, nameMap)?;
    node = InstNode::setDefinition(elem, node)?;
    execStat(&(literal!("NFInstUtil.mergeScalars")))?;
    Ok(node)
}

pub(crate) fn mergeScalars2(
    mut cls: metamodelica::Ref<SCode::Element>,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut isRootClass: bool,
    mut nameMap: MergeNameMap,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut cls: metamodelica::Ref<SCode::Element> = cls;
    let mut cdef: metamodelica::Ref<SCode::ClassDef>;
    let mut elems: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::Element::CLASS { classDef: __esc_cdef @ Deref @ SCode::ClassDef::PARTS { .. }, .. } => {
            cdef = (*__esc_cdef).clone();
            elems = mergeScalars3(var_field!((*cdef).elementLst, SCode::ClassDef::PARTS), nameMap.clone())?;
            elems = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
        for mut e in (elems).into_iter().cloned() {
            let __x = mergeScalarsElement(e.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            assign_variant_field!(cdef => SCode::ClassDef::PARTS;
                elementLst = elems,
                normalEquationLst = mergeScalarsEql(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), nameMap.clone())?,
                initialEquationLst = mergeScalarsEql(var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), nameMap.clone())?,
                normalAlgorithmLst = mergeScalarsAlgs(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), nameMap.clone())?,
                initialAlgorithmLst = mergeScalarsAlgs(var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), nameMap.clone())?
            );
            assign_variant_field!(cls => SCode::Element::CLASS; classDef = cdef.clone());
            if isRootClass {
                System::writeFile({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(classPath, literal!("."), true, false)?); __mm_s.push_str(&*literal!("_merged_table.json")); ArcStr::from(__mm_s) }, UnorderedMap::toJSON(nameMap, &fnptr!(Util::id, _), &move |__a0: metamodelica::Ref<Absyn::ComponentRef>| Dump::printComponentRefStr(&__a0))?)?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub(crate) fn mergeScalars3(
    mut elements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut nameMap: MergeNameMap,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut mergeable: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Element>>>;
    let mut merged_e: metamodelica::Ref<SCode::Element>;
    let mut i: i32 = UnorderedMap::size(nameMap.clone()) + 1;
    let mut prefix: ArcStr;
    (mergeable, outElements) = makeMergeMap(elements)?;
    for mut el in &*mergeable {
        prefix = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("$"));
            __mm_s.push_str(&*AbsynUtil::pathLastIdent(
                &(SCodeUtil::getElementTypePath(&((el).head().cloned()?))?),
            ));
            ArcStr::from(__mm_s)
        };
        merged_e = mergeComponents(
            el.clone(),
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*prefix);
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", i)));
                ArcStr::from(__mm_s)
            },
            nameMap.clone(),
        )?;
        i = i + 1;
        outElements = metamodelica::cons(merged_e, outElements);
    }
    outElements = metamodelica::Dangerous::listReverseInPlace(outElements);
    Ok(outElements)
}

pub(crate) fn makeMergeMap(
    mut elements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Element>>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
)> {
    pub(crate) type ElementList = metamodelica::List<metamodelica::Ref<SCode::Element>>;

    fn append_merge(
        mut oldValue: Option<metamodelica::List<metamodelica::Ref<SCode::Element>>>,
        mut elem: metamodelica::Ref<SCode::Element>,
    ) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
        let mut newValue: metamodelica::List<metamodelica::Ref<SCode::Element>>;
        if (oldValue).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(oldValue) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            newValue = metamodelica::Own::own(__pa0);
        } else {
            newValue = metamodelica::nil();
        }
        newValue = metamodelica::cons(elem, newValue);
        Ok(newValue)
    }

    let mut mergeable: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Element>>> = metamodelica::nil();
    let mut unmergeable: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut merge_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<ArcStr, metamodelica::List<metamodelica::Ref<SCode::Element>>>,
    >;
    let mut grouped_elems: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Element>>>;
    merge_map = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    for mut e in &**elements {
        let () = (match &*e.clone() {
            SCode::Element::COMPONENT { .. } if (isMergeableComponent(metamodelica::AsArg::as_arg(&e))) => {
                UnorderedMap::addUpdate(
                    getComponentSignature(metamodelica::AsArg::as_arg(&e))?,
                    &({
                        let __pe_b1 = e.clone();
                        move |__pe_a0| append_merge(__pe_a0, __pe_b1.clone())
                    }),
                    merge_map.clone(),
                )?;
                ()
            }
            _ => {
                unmergeable = metamodelica::cons(e.clone(), unmergeable);
                ()
            }
        });
    }
    grouped_elems = UnorderedMap::valueList(merge_map);
    for mut el in &*grouped_elems {
        if ((el).len() as i32) == 1 {
            unmergeable = metamodelica::cons((el).head().cloned()?, unmergeable);
        } else {
            mergeable = metamodelica::cons(metamodelica::Dangerous::listReverseInPlace(el.clone()), mergeable);
        }
    }
    Ok((mergeable, unmergeable))
}

pub(crate) fn isMergeableComponent(mut element: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut isMergeable: bool;
    isMergeable = (::match_deref::match_deref! { match element {
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: Deref @ metamodelica::ListNode::Nil, .. }, prefixes: Deref @ SCode::Prefixes { redeclarePrefix: SCode::Redeclare::NOT_REDECLARE { .. }, innerOuter: Absyn::InnerOuter::NOT_INNER_OUTER { .. }, replaceablePrefix: Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. }, .. }, condition: None, modifications: __element_modifications, typeSpec: __element_typeSpec, .. } => isMergeableType(metamodelica::AsArg::as_arg(&__element_typeSpec)) && isMergeableMod(metamodelica::AsArg::as_arg(&__element_modifications)),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isMergeable
}

pub(crate) fn isMergeableMod(mut r#mod: &metamodelica::Ref<SCode::Mod>) -> bool {
    let mut mergeable: bool;
    mergeable = (match &**r#mod {
        SCode::Mod::MOD {
            eachPrefix: SCode::Each::NOT_EACH { .. },
            subModLst: __mod_subModLst,
            ..
        } => {
            for mut m in &*__mod_subModLst.clone() {
                if !(isMergeableMod(&m.r#mod)) {
                    mergeable = false;
                    return mergeable;
                }
            }
            true
        }
        SCode::Mod::NOMOD { .. } => true,
        _ => false,
    });
    mergeable
}

pub(crate) fn isMergeableType(mut ty: &metamodelica::Ref<Absyn::TypeSpec>) -> bool {
    let mut mergeable: bool;
    mergeable = (::match_deref::match_deref! { match ty {
        Deref @ Absyn::TypeSpec::TPATH { arrayDim: None, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    mergeable
}

pub(crate) fn getComponentSignature(mut element: &metamodelica::Ref<SCode::Element>) -> Result<ArcStr> {
    let mut signature: ArcStr;
    let mut prefs: metamodelica::Ref<SCode::Prefixes>;
    let mut attrs: SCode::Attributes;
    let mut ty: metamodelica::Ref<Absyn::TypeSpec>;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*element)) {
        Deref @ SCode::Element::COMPONENT { prefixes: __pa0, attributes: __pa1, typeSpec: __pa2, modifications: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    prefs = metamodelica::Own::own(__pa0);
    attrs = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    r#mod = metamodelica::Own::own(__pa3);
    signature = stringAppendList(list![
        SCodeDump::visibilityStr(prefs.visibility.clone()),
        SCodeDump::finalStr(prefs.finalPrefix.clone()),
        SCodeDump::connectorTypeStr(attrs.connectorType.clone()),
        SCodeDump::variabilityString(attrs.variability.clone()),
        Dump::unparseDirectionSymbolStr(attrs.direction.clone())?,
        Dump::unparseTypeSpec(ty)?,
        getModSignature(&r#mod, &(literal!("")))?
    ]);
    Ok(signature)
}

pub(crate) fn getModSignature(mut r#mod: &metamodelica::Ref<SCode::Mod>, mut name: &ArcStr) -> Result<ArcStr> {
    fn sub_mod_lt(mut m1: &metamodelica::Ref<SCode::SubMod>, mut m2: &metamodelica::Ref<SCode::SubMod>) -> bool {
        let mut res: bool = m1.ident.clone() < m2.ident.clone();
        res
    }

    let mut signature: ArcStr;
    let mut strl: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut has_binding: bool;
    let mut has_submods: bool;
    signature = (match &**r#mod {
        SCode::Mod::MOD {
            binding: __mod_binding,
            eachPrefix: __mod_eachPrefix,
            finalPrefix: __mod_finalPrefix,
            subModLst: __mod_subModLst,
            ..
        } => {
            has_binding = (__mod_binding).is_some();
            has_submods = !((__mod_subModLst).is_empty());
            if has_binding {
                strl = metamodelica::cons(literal!("="), strl);
            }
            if has_submods {
                strl = metamodelica::cons(literal!(")"), strl);
                for mut m in &*List::sort(
                    __mod_subModLst.clone(),
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<SCode::SubMod>,
                              __a1: metamodelica::Ref<SCode::SubMod>|
                              -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(sub_mod_lt(&__a0, &__a1))
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<SCode::SubMod>,
                                    metamodelica::Ref<SCode::SubMod>,
                                ) -> Result<bool>
                                + 'static,
                        >),
                )? {
                    strl = metamodelica::cons(literal!(","), strl);
                    strl = metamodelica::cons(getModSignature(&m.r#mod, &m.ident)?, strl);
                }
                strl = metamodelica::cons(literal!("("), strl);
            }
            if has_binding || has_submods {
                strl = metamodelica::cons(name.clone(), strl);
            }
            if SCodeUtil::finalBool(__mod_finalPrefix.clone()) {
                strl = metamodelica::cons(literal!("final "), strl);
            }
            if SCodeUtil::eachBool(__mod_eachPrefix.clone()) {
                strl = metamodelica::cons(literal!("each "), strl);
            }
            stringAppendList(strl)
        }
        _ => literal!(""),
    });
    Ok(signature)
}

pub(crate) fn mergeComponents(
    mut components: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut prefix: ArcStr,
    mut nameMap: MergeNameMap,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut mergedComponent: metamodelica::Ref<SCode::Element>;
    let mut ty: metamodelica::Ref<Absyn::TypeSpec>;
    let mut prefs: metamodelica::Ref<SCode::Prefixes>;
    let mut attrs: SCode::Attributes;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let mut i: i32 = 1;
    let mut name: ArcStr;
    let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut mods: metamodelica::List<metamodelica::Ref<SCode::Mod>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((components).head().cloned()?) {
        Deref @ SCode::Element::COMPONENT { typeSpec: __pa0, prefixes: __pa1, attributes: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    prefs = metamodelica::Own::own(__pa1);
    attrs = metamodelica::Own::own(__pa2);
    attrs.arrayDims = list![AbsynUtil::makeIntegerSubscript(((components).len() as i32))];
    mods = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Mod>> = metamodelica::nil();
        for mut c in (components.clone()).into_iter().cloned() {
            let __x = SCodeUtil::componentMod(&(c.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    r#mod = mergeMods(mods)?;
    mergedComponent = metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: prefix.clone(),
        prefixes: prefs,
        attributes: attrs,
        typeSpec: ty,
        modifications: r#mod,
        comment: SCode::noComment.clone(),
        condition: None,
        info: Absyn::dummyInfo.clone(),
    });
    for mut c in &*components {
        let __pa3 = ::match_deref::match_deref! { match &(c.clone()) {
            Deref @ SCode::Element::COMPONENT { name: __pa3, .. } => __pa3.clone(),
            _ => return Err("pattern mismatch"),
        } };
        name = metamodelica::Own::own(__pa3);
        cref = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
            name: prefix.clone(),
            subscripts: list![AbsynUtil::makeIntegerSubscript(i)],
        });
        i = i + 1;
        UnorderedMap::addUnique(name, cref, nameMap.clone())?;
    }
    Ok(mergedComponent)
}

pub(crate) fn mergeMods(
    mut mods: metamodelica::List<metamodelica::Ref<SCode::Mod>>,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let mut names: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut bindings: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>;
    let mut binding_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Exp>>,
    >;
    if (mods).is_empty() {
        r#mod = openmodelica_frontend_types::SCode::Mod::interned_NOMOD();
        return Ok(r#mod);
    }
    r#mod = (mods).head().cloned()?;
    names = getModNames(&r#mod, &(metamodelica::nil()), metamodelica::nil())?;
    bindings = List::fill(metamodelica::nil(), ((names).len() as i32));
    for mut m in &*mods.reverse() {
        bindings = getModBindings(m.clone(), &names, bindings)?;
    }
    binding_map = UnorderedMap::fromLists(
        &names,
        ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
            for mut b in (bindings).into_iter().cloned() {
                let __x = metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: b.clone() });
                __acc = cons(__x, __acc);
            }
            __acc
        }),
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
    )?;
    r#mod = mergeMods2(r#mod, binding_map, &(metamodelica::nil()))?;
    Ok(r#mod)
}

pub(crate) fn getModNames(
    mut r#mod: &metamodelica::Ref<SCode::Mod>,
    mut name: &metamodelica::List<ArcStr>,
    mut names: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut names: metamodelica::List<metamodelica::Ref<Absyn::Path>> = names;
    names = (match &**r#mod {
        SCode::Mod::MOD {
            binding: __mod_binding,
            subModLst: __mod_subModLst,
            ..
        } => {
            if (__mod_binding).is_some() {
                names = metamodelica::cons(makeModPath(name)?, names);
            }
            for mut m in &*__mod_subModLst.clone() {
                names = getModNames(&m.r#mod, &(metamodelica::cons(m.ident.clone(), name.clone())), names)?;
            }
            names
        }
        _ => names,
    });
    Ok(names)
}

pub(crate) fn makeModPath(mut name: &metamodelica::List<ArcStr>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    if (name).is_empty() {
        path = metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("$") });
    } else {
        path = AbsynUtil::stringListPathReversed(name)?;
    }
    Ok(path)
}

pub(crate) fn mergeMods2(
    mut r#mod: metamodelica::Ref<SCode::Mod>,
    mut bindingMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Exp>>,
    >,
    mut name: &metamodelica::List<ArcStr>,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod;
    let mut new_binding: metamodelica::Ref<Absyn::Exp>;
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>> = metamodelica::nil();
    let () = (match &*r#mod {
        SCode::Mod::MOD { .. } => {
            if (var_field!((*r#mod).binding, SCode::Mod::MOD)).is_some() {
                new_binding = UnorderedMap::getOrFail(makeModPath(name)?, bindingMap.clone())?;
                assign_variant_field!(r#mod => SCode::Mod::MOD; binding = Some(new_binding));
            }
            if !((var_field!((*r#mod).subModLst, SCode::Mod::MOD)).is_empty()) {
                for mut m in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                    let mut m = m.clone();
                    assign_field!(
                        m.r#mod = mergeMods2(
                            m.r#mod.clone(),
                            bindingMap.clone(),
                            &(metamodelica::cons(m.ident.clone(), name.clone()))
                        )?
                    );
                    submods = metamodelica::cons(m, submods);
                }
                assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = metamodelica::Dangerous::listReverseInPlace(submods));
            }
            ()
        }
        _ => (),
    });
    Ok(r#mod)
}

pub(crate) fn getModBindings(
    mut r#mod: metamodelica::Ref<SCode::Mod>,
    mut names: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut bindings: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>> {
    let mut bindings: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>> = bindings;
    let mut mod_bindings: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
    for mut name in &**names {
        mod_bindings = metamodelica::cons(
            lookupModBinding(metamodelica::AsArg::as_arg(&name), r#mod.clone())?,
            mod_bindings,
        );
    }
    bindings = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        let __thr_src0 = mod_bindings;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = bindings;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(e1), Some(e2)) => {
                    let __x = cons(e1.clone(), e2.clone());
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(bindings)
}

pub(crate) fn lookupModBinding(
    mut name: &metamodelica::Ref<Absyn::Path>,
    mut r#mod: metamodelica::Ref<SCode::Mod>,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut binding: metamodelica::Ref<Absyn::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(lookupMod(name, r#mod)?) {
        Deref @ SCode::Mod::MOD { binding: Some(__pa0), .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    binding = metamodelica::Own::own(__pa0);
    Ok(binding)
}

pub(crate) fn lookupMod<'__b>(
    mut name: &'__b metamodelica::Ref<Absyn::Path>,
    mut r#mod: metamodelica::Ref<SCode::Mod>,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    outMod = (match &**name {
        Absyn::Path::IDENT { .. } => {
            if (metamodelica::stringEq(&var_field!((**name).name, Absyn::Path::IDENT), &(literal!("$")))) {
                r#mod
            } else {
                SCodeUtil::lookupModInMod(var_field!((**name).name, Absyn::Path::IDENT), &r#mod)
            }
        }
        Absyn::Path::QUALIFIED { .. } => {
            outMod = SCodeUtil::lookupModInMod(var_field!((**name).name, Absyn::Path::QUALIFIED), &r#mod);
            lookupMod(var_field!((**name).path, Absyn::Path::QUALIFIED), outMod)?
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outMod)
}

pub(crate) fn mergeScalarsElement(
    mut element: metamodelica::Ref<SCode::Element>,
    mut nameMap: MergeNameMap,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut element: metamodelica::Ref<SCode::Element> = element;
    let () = (match &*element {
        SCode::Element::EXTENDS {
            modifications: __element_modifications,
            ..
        } => {
            assign_variant_field!(element => SCode::Element::EXTENDS; modifications = mergeScalarsMod(__element_modifications.clone(), nameMap)?);
            ()
        }
        SCode::Element::COMPONENT {
            modifications: __element_modifications,
            ..
        } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; modifications = mergeScalarsMod(__element_modifications.clone(), nameMap)?);
            ()
        }
        _ => (),
    });
    Ok(element)
}

pub(crate) fn mergeScalarsEql(
    mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut nameMap: MergeNameMap,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Equation>>> {
    let mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>> = eql;
    eql = SCodeUtil::mapEquationsList(
        eql,
        &({
            let __pe_b1 = nameMap;
            move |__pe_a0| mergeScalarsEq(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(eql)
}

pub(crate) fn mergeScalarsEq(
    mut eq: metamodelica::Ref<SCode::Equation>,
    mut nameMap: MergeNameMap,
) -> Result<metamodelica::Ref<SCode::Equation>> {
    let mut eq: metamodelica::Ref<SCode::Equation> = eq;
    eq = SCodeUtil::mapEquationExps(
        eq,
        &({
            let __pe_b1 = nameMap.clone();
            move |__pe_a0| mergeScalarsExps(__pe_a0, __pe_b1.clone())
        }),
    )?;
    let () = (match &*eq {
        SCode::Equation::EQ_CONNECT {
            crefLeft: __eq_crefLeft,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_CONNECT;
                crefLeft = mergeScalarsCref(__eq_crefLeft.clone(), nameMap.clone())?,
                crefRight = mergeScalarsCref(var_field!((*eq).crefRight, SCode::Equation::EQ_CONNECT).clone(), nameMap)?
            );
            ()
        }
        _ => (),
    });
    Ok(eq)
}

pub(crate) fn mergeScalarsMod(
    mut r#mod: metamodelica::Ref<SCode::Mod>,
    mut nameMap: MergeNameMap,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod;
    let () = (match &*r#mod {
        SCode::Mod::MOD {
            binding: __mod_binding, ..
        } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD;
                        binding = Util::applyOption(__mod_binding.clone(), &({ let __pe_b1 = nameMap.clone(); move |__pe_a0| mergeScalarsExps(__pe_a0, __pe_b1.clone()) }))?,
                        subModLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::SubMod>> = metamodelica::nil();
                for mut m in (var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()).into_iter().cloned() {
                    let __x = mergeScalarsSubMod(m.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        _ => (),
    });
    Ok(r#mod)
}

pub(crate) fn mergeScalarsSubMod(
    mut r#mod: metamodelica::Ref<SCode::SubMod>,
    mut nameMap: MergeNameMap,
) -> Result<metamodelica::Ref<SCode::SubMod>> {
    let mut r#mod: metamodelica::Ref<SCode::SubMod> = r#mod;
    assign_field!(r#mod.r#mod = mergeScalarsMod(r#mod.r#mod.clone(), nameMap)?);
    Ok(r#mod)
}

pub(crate) fn mergeScalarsExps(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut nameMap: MergeNameMap,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    (exp, _) = AbsynUtil::traverseExp(
        exp,
        (std::sync::Arc::new(mergeScalarsExp)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<Absyn::ComponentRef>>>,
                    ) -> Result<(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<Absyn::ComponentRef>>>,
                    )> + 'static,
            >),
        nameMap,
    )?;
    Ok(exp)
}

pub(crate) fn mergeScalarsExp(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut nameMap: MergeNameMap,
) -> Result<(metamodelica::Ref<Absyn::Exp>, MergeNameMap)> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut nameMap: MergeNameMap = nameMap;
    let () = (match &*exp {
        Absyn::Exp::CREF {
            componentRef: __exp_componentRef,
        } if (!(AbsynUtil::crefIsWild(metamodelica::AsArg::as_arg(&__exp_componentRef)))) => {
            assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = mergeScalarsCref(__exp_componentRef.clone(), nameMap.clone())?);
            ()
        }
        _ => (),
    });
    Ok((exp, nameMap))
}

pub(crate) fn mergeScalarsCref(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut nameMap: MergeNameMap,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let mut repl_ocr: Option<metamodelica::Ref<Absyn::ComponentRef>>;
    let mut repl_cr: metamodelica::Ref<Absyn::ComponentRef>;
    let mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    repl_ocr = UnorderedMap::get(AbsynUtil::crefFirstIdent(&cref)?, nameMap)?;
    if (repl_ocr).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(repl_ocr) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        repl_cr = metamodelica::Own::own(__pa0);
        subs = AbsynUtil::crefFirstSubs(&cref)?;
        if !((subs).is_empty()) {
            subs = listAppend(AbsynUtil::crefFirstSubs(&repl_cr)?, subs);
            repl_cr = AbsynUtil::crefSetLastSubs(repl_cr, &subs)?;
        }
        cref = AbsynUtil::crefReplaceFirst(&cref, &repl_cr)?;
    }
    Ok(cref)
}

pub(crate) fn mergeScalarsAlgs(
    mut algs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    mut nameMap: MergeNameMap,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>> {
    let mut algs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>> = algs;
    algs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>> = metamodelica::nil();
        for mut a in (algs).into_iter().cloned() {
            let __x = SCodeUtil::mapAlgorithmStatements(
                a.clone(),
                &({
                    let __pe_b1 = nameMap.clone();
                    move |__pe_a0| mergeScalarsStmt(__pe_a0, __pe_b1.clone())
                }),
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(algs)
}

pub(crate) fn mergeScalarsStmt(
    mut stmt: metamodelica::Ref<SCode::Statement>,
    mut nameMap: MergeNameMap,
) -> Result<metamodelica::Ref<SCode::Statement>> {
    let mut stmt: metamodelica::Ref<SCode::Statement> = stmt;
    stmt = SCodeUtil::mapStatementExps(
        stmt,
        &({
            let __pe_b1 = nameMap;
            move |__pe_a0| mergeScalarsExps(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(stmt)
}

pub(crate) fn mergeScalarsComponentBindings(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut nameMap: MergeNameMap,
) -> Result<()> {
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    cls = InstNode::getClass(node.clone())?;
    cls_tree = Class::classTree(cls.clone())?;
    ClassTree::applyComponents(
        &cls_tree,
        &({
            let __pe_b1 = nameMap;
            move |__pe_a0| mergeScalarsComponentBinding(__pe_a0, __pe_b1.clone())
        }),
    )?;
    cls = Class::setClassTree(cls_tree, cls)?;
    InstNode::updateClass(cls, node)?;
    Ok(())
}

pub(crate) fn mergeScalarsComponentBinding(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut nameMap: MergeNameMap,
) -> Result<()> {
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    if !(InstNode::isComponent(&node)?) {
        return Ok(());
    }
    comp = InstNode::component(&node)?;
    let () = (match &*comp {
        Component::COMPONENT_DEF {
            definition: __comp_definition,
            ..
        } => {
            assign_variant_field!(comp => Component::NFComponent::COMPONENT_DEF; definition = mergeScalarsElement(__comp_definition.clone(), nameMap)?);
            InstNode::updateComponent(comp, node)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub fn createExtractorModel(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
    mut funcs: metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree>,
) -> Result<(
    metamodelica::Ref<FlatModel::NFFlatModel>,
    metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree>,
)> {
    let mut extractorModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel.clone();
    let mut outFuncs: metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree> = funcs;
    let mut top_level_connectors: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut flows: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut inputs: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut fn_template: metamodelica::Ref<Function::Function>;
    let mut index: i32 = 0;
    let mut eqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut eq: metamodelica::Ref<Equation::NFEquation>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    (top_level_connectors, flows, inputs) = collectExtractorModelVariables(flatModel.variables.clone())?;
    fn_template = createExtractorModelDummyFn(top_level_connectors.clone())?;
    args = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut c in (top_level_connectors).into_iter().cloned() {
            let __x = Expression::fromCref(Variable::name(&(c.clone())), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    for mut f in &*flows {
        (eq, outFuncs, index) = createExtractorModelDummyEq(
            metamodelica::AsArg::as_arg(&f),
            &(literal!("flow")),
            fn_template.clone(),
            args.clone(),
            outFuncs,
            index,
        )?;
        eqs = metamodelica::cons(eq, eqs);
    }
    for mut i in &*inputs {
        (eq, outFuncs, index) = createExtractorModelDummyEq(
            metamodelica::AsArg::as_arg(&i),
            &(literal!("input")),
            fn_template.clone(),
            args.clone(),
            outFuncs,
            index,
        )?;
        eqs = metamodelica::cons(eq, eqs);
    }
    eqs = metamodelica::Dangerous::listReverseInPlace(eqs);
    assign_field!(extractorModel.equations = listAppend(extractorModel.equations.clone(), eqs));
    Ok((extractorModel, outFuncs))
}

pub(crate) fn collectExtractorModelVariables(
    mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
)> {
    let mut topLevelConnectorVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    let mut flowVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    let mut inputVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    let mut top_node: metamodelica::Ref<InstNode::InstNode>;
    for mut var in &*vars.reverse() {
        if !(ComponentRef::isSimple(&var.name)) {
            top_node = ComponentRef::node(&(ComponentRef::last(&var.name)))?;
            if InstNode::isConnector(&top_node)? && InstNode::isPublic(&top_node) {
                topLevelConnectorVars = metamodelica::cons(var.clone(), topLevelConnectorVars);
                if Variable::isFlow(metamodelica::AsArg::as_arg(&var)) {
                    flowVars = metamodelica::cons(var.clone(), flowVars);
                } else if Variable::isInput(metamodelica::AsArg::as_arg(&var)) {
                    inputVars = metamodelica::cons(var.clone(), flowVars.clone());
                }
            }
        }
    }
    Ok((topLevelConnectorVars, flowVars, inputVars))
}

pub(crate) static REAL_TYPE_SPEC: std::sync::LazyLock<metamodelica::Ref<Absyn::TypeSpec>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Real") }),
            arrayDim: None,
        })
    });

pub(crate) fn createExtractorModelDummyFn(
    mut connectors: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<metamodelica::Ref<Function::Function>> {
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut cdef: metamodelica::Ref<SCode::ClassDef>;
    let mut output_param: metamodelica::Ref<SCode::Element>;
    let mut elem: metamodelica::Ref<SCode::Element>;
    let mut fn_node: metamodelica::Ref<InstNode::InstNode>;
    let mut params: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut output_binding: metamodelica::Ref<SCode::Mod>;
    let mut cmt: metamodelica::Ref<SCode::Comment>;
    output_binding = SCodeUtil::makeMod(
        false,
        false,
        metamodelica::nil(),
        Some(metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 0 })),
        None,
        Absyn::dummyInfo.clone(),
    );
    output_param = metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: literal!("dummy"),
        prefixes: SCode::defaultPrefixes.clone(),
        attributes: SCode::defaultOutputAttr.clone(),
        typeSpec: REAL_TYPE_SPEC.clone(),
        modifications: output_binding,
        comment: SCode::noComment.clone(),
        condition: None,
        info: Absyn::dummyInfo.clone(),
    });
    params = listAppend(
        ({
            let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
            for mut c in (connectors).into_iter().cloned() {
                let __x = createExtractorModelDummyFnInput(&(c.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        list![output_param],
    );
    cdef = metamodelica::Ref::new(SCode::ClassDef::PARTS {
        elementLst: params,
        normalEquationLst: metamodelica::nil(),
        initialEquationLst: metamodelica::nil(),
        normalAlgorithmLst: metamodelica::nil(),
        initialAlgorithmLst: metamodelica::nil(),
        constraintLst: metamodelica::nil(),
        clsattrs: metamodelica::nil(),
        externalDecl: None,
    });
    cmt = metamodelica::Ref::new(SCode::Comment {
        annotation_: Some(metamodelica::Ref::new(SCode::Annotation {
            modification: SCodeUtil::makeMod(
                false,
                false,
                list![metamodelica::Ref::new(SCode::SubMod {
                    ident: literal!("Inline"),
                    r#mod: SCodeUtil::makeMod(
                        false,
                        false,
                        metamodelica::nil(),
                        Some(metamodelica::Ref::new(Absyn::Exp::BOOL { value: false })),
                        None,
                        Absyn::dummyInfo.clone()
                    )
                })],
                None,
                None,
                Absyn::dummyInfo.clone(),
            ),
        })),
        comment: None,
    });
    elem = metamodelica::Ref::new(SCode::Element::CLASS {
        name: literal!("dummy"),
        prefixes: SCode::defaultPrefixes.clone(),
        encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        restriction: SCode::Restriction::R_FUNCTION {
            functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION {
                purity: openmodelica_ast::Absyn::FunctionPurity::PURE,
            },
        },
        classDef: cdef,
        cmt: cmt,
        info: Absyn::dummyInfo.clone(),
    });
    fn_node = InstNode::new(elem, crate::NFInstNode::InstNode::interned_EMPTY_NODE())?;
    fn_node = Function::instFunctionNode(fn_node, NFInstContext::FUNCTION.clone(), Absyn::dummyInfo.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(Function::typeNodeCache(fn_node, NFInstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa0);
    Ok(r#fn)
}

pub(crate) fn createExtractorModelDummyFnInput(
    mut var: &metamodelica::Ref<Variable::NFVariable>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut inputElem: metamodelica::Ref<SCode::Element>;
    inputElem = metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: ComponentRef::toFlatString(&var.name, BaseModelica::defaultFormat.clone())?,
        prefixes: SCode::defaultPrefixes.clone(),
        attributes: SCode::defaultInputAttr.clone(),
        typeSpec: REAL_TYPE_SPEC.clone(),
        modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
        comment: SCode::noComment.clone(),
        condition: None,
        info: Absyn::dummyInfo.clone(),
    });
    Ok(inputElem)
}

pub(crate) fn createExtractorModelDummyEq(
    mut var: &metamodelica::Ref<Variable::NFVariable>,
    mut varType: &ArcStr,
    mut r#fn: metamodelica::Ref<Function::Function>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut funcs: metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree>,
    mut index: i32,
) -> Result<(
    metamodelica::Ref<Equation::NFEquation>,
    metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree>,
    i32,
)> {
    let mut eq: metamodelica::Ref<Equation::NFEquation>;
    let mut funcs: metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree> = funcs;
    let mut index: i32 = index;
    let mut indexed_fn: metamodelica::Ref<Function::Function>;
    let mut fn_name: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut src: metamodelica::Ref<DAE::ElementSource> = DAE::emptyElementSource().clone();
    let mut var_name: ArcStr;
    loop {
        index = index + 1;
        fn_name = metamodelica::Ref::new(Absyn::Path::IDENT {
            name: {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("f"));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index)));
                ArcStr::from(__mm_s)
            },
        });
        if !(NFFlatten::FunctionTreeImpl::hasKey(funcs.clone(), fn_name.clone())?) {
            break;
        }
    }
    indexed_fn = Function::setName(fn_name.clone(), r#fn.clone());
    var_name = ComponentRef::toString(&(Variable::name(var)))?;
    src = ElementSource::addCommentToSource(
        src,
        Some(metamodelica::Ref::new(SCode::Comment {
            annotation_: None,
            comment: Some({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Dummy equation for "));
                __mm_s.push_str(&*var_name);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*varType);
                __mm_s.push_str(&*literal!(" variable"));
                ArcStr::from(__mm_s)
            }),
        })),
    );
    eq = Equation::makeEquality(
        metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: metamodelica::OrderedFloat((0) as f64),
        }),
        metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(
                indexed_fn.clone(),
                args,
                Variability::CONTINUOUS.clone(),
                Purity::PURE.clone(),
                indexed_fn.returnType.clone(),
            ),
        }),
        crate::NFType::interned_REAL(),
        src,
        InstNode::fromHandle(&r#fn.node)?,
        Equation::ScalarizeMode::NO_PREFERENCE.clone(),
    );
    funcs = NFFlatten::FunctionTreeImpl::add(
        funcs,
        &fn_name,
        &indexed_fn,
        &*(std::sync::Arc::new(fnptr!(NFFlatten::FunctionTreeImpl::addConflictDefault, _, _, _))
            as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
    )?;
    Ok((eq, funcs, index))
}
