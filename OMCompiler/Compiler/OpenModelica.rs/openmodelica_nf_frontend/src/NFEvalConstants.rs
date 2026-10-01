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

use crate::NFAlgorithm as Algorithm;
use crate::NFBinding as Binding;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFEquation::Branch;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten as Flatten;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFPackage as Package;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFSections as Sections;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_error::ErrorExt;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::Util;

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct EvalSettings {
    pub scalarize: bool,
}

impl metamodelica::gc::MMTrace for EvalSettings {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.scalarize, __mmv)?;
        Ok(())
    }
}
impl Default for EvalSettings {
    fn default() -> Self {
        Self {
            scalarize: Default::default(),
        }
    }
}

pub type SETTINGS = EvalSettings;

pub fn evaluate(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
    mut context: i32,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut settings: EvalSettings;
    settings = EvalSettings {
        scalarize: Flags::isSet(Flags::NF_SCALARIZE.clone())?,
    };
    assign_field!(
        flatModel.variables = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut v in (flatModel.variables.clone()).into_iter().cloned() {
                let __x = evaluateVariable(v.clone(), context, settings)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        flatModel.equations = evaluateEquations(flatModel.equations.clone())?,
        flatModel.initialEquations = evaluateEquations(flatModel.initialEquations.clone())?,
        flatModel.algorithms = evaluateAlgorithms(flatModel.algorithms.clone())?,
        flatModel.initialAlgorithms = evaluateAlgorithms(flatModel.initialAlgorithms.clone())?
    );
    execStat(&(literal!("NFEvalConstants.evaluate")))?;
    Ok(flatModel)
}

pub(crate) fn evaluateVariable(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut context: i32,
    mut settings: EvalSettings,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut structural: bool;
    let mut variability: Variability;
    variability = Variable::variability(&var);
    structural = variability <= Variability::STRUCTURAL_PARAMETER.clone() && !(Type::isExternalObject(&var.ty));
    binding = evaluateBinding(var.binding.clone(), var.name.clone(), structural, variability, context)?;
    if !(referenceEq(&*(&*binding), &*(var.binding.clone()))) {
        assign_field!(var.binding = binding);
    }
    assign_field!(
        var.typeAttributes = ({
            let mut __acc: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> = metamodelica::nil();
            for mut a in (var.typeAttributes.clone()).into_iter().cloned() {
                let __x = evaluateTypeAttribute(a.clone(), var.name.clone(), context)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        var.children = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut v in (var.children.clone()).into_iter().cloned() {
                let __x = evaluateVariable(v.clone(), context, settings)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(var)
}

pub(crate) fn evaluateBinding(
    mut binding: metamodelica::Ref<Binding::NFBinding>,
    mut prefix: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut structural: bool,
    mut variability: Variability,
    mut context: i32,
) -> Result<metamodelica::Ref<Binding::NFBinding>> {
    let mut binding: metamodelica::Ref<Binding::NFBinding> = binding;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut eexp: metamodelica::Ref<Expression::NFExpression>;
    let mut info: SourceInfo;
    if Binding::isBound(&binding) {
        exp = Binding::getTypedExp(&binding)?;
        if structural {
            info = Binding::getInfo(&binding);
            eexp = evaluateExp(exp.clone(), info.clone())?;
            eexp = SimplifyExp::simplify(eexp, false)?;
            if !(Expression::isLiteral(&eexp)? || Expression::isKnownSizeFill(&eexp)?) {
                if variability > Variability::CONSTANT.clone() || InstContext::inRelaxed(context) {
                    eexp = Ceval::tryEvalExp(eexp, &(Ceval::noTarget().clone()));
                } else {
                    eexp = Ceval::evalExp(eexp, &(Ceval::EvalTarget::new(info.clone(), context, None)))?;
                }
            }
            eexp = Flatten::flattenExp(
                eexp,
                &(metamodelica::Ref::new(Flatten::Prefix::Prefix::PREFIX {
                    root: crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
                    prefix: prefix,
                })),
                &info,
            )?;
        } else {
            eexp = evaluateExp(exp.clone(), Binding::getInfo(&binding))?;
        }
        if !(referenceEq(&*(exp), &*(&*eexp))) {
            binding = Binding::setTypedExp(eexp, binding)?;
        }
    }
    Ok(binding)
}

pub(crate) fn evaluateTypeAttribute(
    mut attribute: (ArcStr, metamodelica::Ref<Binding::NFBinding>),
    mut prefix: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut context: i32,
) -> Result<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> {
    let mut attribute: (ArcStr, metamodelica::Ref<Binding::NFBinding>) = attribute;
    let mut name: ArcStr;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut sbinding: metamodelica::Ref<Binding::NFBinding>;
    let mut structural: bool;
    (name, binding) = attribute.clone();
    structural = metamodelica::stringEq(&name, &(literal!("fixed")))
        || metamodelica::stringEq(&name, &(literal!("stateSelect")));
    sbinding = evaluateBinding(
        binding.clone(),
        prefix,
        structural,
        Variability::PARAMETER.clone(),
        context,
    )?;
    if !(referenceEq(&*(binding), &*(&*sbinding))) {
        attribute = (name, sbinding);
    }
    Ok(attribute)
}

pub fn evaluateExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    (outExp, _) = evaluateExpTraverser(exp, info, false)?;
    Ok(outExp)
}

pub(crate) fn evaluateExpTraverser(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut info: SourceInfo,
    mut changed: bool,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outChanged: bool = changed;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    (outExp, outChanged) = (match &*exp {
        Expression::CREF { .. } => {
            let (__pa2, __pa0, __pa1, __pa3) = ::match_deref::match_deref! { match &(Expression::mapFoldShallow(exp, (std::sync::Arc::new({ let __pe_b1 = info.clone(); move |__pe_a0, __pe_a2| evaluateExpTraverser(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, bool) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> + 'static>), false)?) {
                (__pa2 @ Deref @ Expression::CREF { cref: __pa0, ty: __pa1 }, __pa3) => (__pa2.clone(), __pa0.clone(), __pa1.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cref = metamodelica::Own::own(__pa0);
            ty = metamodelica::Own::own(__pa1);
            outExp = metamodelica::Own::own(__pa2);
            outChanged = metamodelica::Own::own(__pa3);
            var = ComponentRef::nodeVariability(&cref)?;
            if var <= Variability::STRUCTURAL_PARAMETER.clone() && !(Type::isExternalObject(&ty)) {
                if var > Variability::CONSTANT.clone() {
                    ErrorExt::setCheckpoint(literal!("NFEvalConstants.evaluateExpTraverser"));
                    if '__try5: {
                        e = unwrap_break_err!(Ceval::evalCref(&cref, outExp.clone(), &(Ceval::noTarget().clone()), false, true), '__try5);
                        e = unwrap_break_err!(Flatten::flattenExp(e.clone(), &(metamodelica::Ref::new(Flatten::Prefix::Prefix::PREFIX { root: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), prefix: cref.clone() })), &info), '__try5);
                        outExp = e.clone();
                        outChanged = true;
                        Ok::<(), &'static str>(())
                    }.is_err() {
                    }
                    ErrorExt::rollBack(literal!("NFEvalConstants.evaluateExpTraverser"));
                } else {
                    outExp = Ceval::evalCref(
                        &cref,
                        outExp,
                        &(Ceval::EvalTarget::new(info.clone(), InstContext::NO_CONTEXT.clone(), None)),
                        false,
                        true,
                    )?;
                    outExp = Flatten::flattenExp(
                        outExp,
                        &(metamodelica::Ref::new(Flatten::Prefix::Prefix::PREFIX {
                            root: crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
                            prefix: cref,
                        })),
                        &info,
                    )?;
                    outChanged = true;
                }
            } else if outChanged {
                ty = ComponentRef::getSubscriptedType(&cref, false)?;
            }
            ty2 = evaluateType(ty.clone(), info)?;
            if !(referenceEq(&*(ty), &*(&*ty2))) {
                outExp = Expression::setType(ty2, outExp)?;
            }
            (outExp, outChanged)
        }
        Expression::ARRAY { literal: true, .. } => (exp, false),
        Expression::IF { .. } => evaluateIfExp(&exp, info)?,
        Expression::SIZE { .. } => {
            if (var_field!((*exp).dimIndex, Expression::NFExpression::SIZE)).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                e = metamodelica::Own::own(__pa0);
                (e, outChanged) = Expression::mapFoldShallow(
                    e,
                    (std::sync::Arc::new({
                        let __pe_b1 = info;
                        move |__pe_a0, __pe_a2| evaluateExpTraverser(__pe_a0, __pe_b1.clone(), __pe_a2)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                    bool,
                                )
                                    -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)>
                                + 'static,
                        >),
                    false,
                )?;
                if outChanged {
                    assign_variant_field!(exp => Expression::NFExpression::SIZE; dimIndex = Some(e));
                }
            }
            (exp, outChanged)
        }
        Expression::RANGE { .. } => {
            (outExp, outChanged) = Expression::mapFoldShallow(
                exp,
                (std::sync::Arc::new({
                    let __pe_b1 = info;
                    move |__pe_a0, __pe_a2| evaluateExpTraverser(__pe_a0, __pe_b1.clone(), __pe_a2)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                                bool,
                            )
                                -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)>
                            + 'static,
                    >),
                false,
            )?;
            if outChanged {
                outExp = Expression::retype(outExp)?;
            }
            (outExp, outChanged)
        }
        _ => {
            (outExp, outChanged) = Expression::mapFoldShallow(
                exp,
                (std::sync::Arc::new({
                    let __pe_b1 = info.clone();
                    move |__pe_a0, __pe_a2| evaluateExpTraverser(__pe_a0, __pe_b1.clone(), __pe_a2)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                                bool,
                            )
                                -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)>
                            + 'static,
                    >),
                false,
            )?;
            ty = Expression::typeOf(outExp.clone());
            ty2 = evaluateType(ty.clone(), info)?;
            (
                if (referenceEq(&*(ty), &*(&*ty2))) {
                    outExp
                } else {
                    Expression::setType(ty2, outExp)?
                },
                outChanged,
            )
        }
    });
    outChanged = changed || outChanged;
    Ok((outExp, outChanged))
}

pub(crate) fn evaluateType(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Type::NFType>> {
    let mut ty: metamodelica::Ref<Type::NFType> = ty;
    ty = (match &*ty {
        Type::ARRAY {
            dimensions: __ty_dimensions,
            ..
        } => {
            assign_variant_field!(ty => Type::NFType::ARRAY; dimensions = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
                for mut d in (__ty_dimensions.clone()).into_iter().cloned() {
                    let __x = evaluateDimension(d.clone(), info.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ty
        }
        Type::CONDITIONAL_ARRAY { .. } => Type::simplifyConditionalArray(ty),
        _ => ty,
    });
    Ok(ty)
}

pub(crate) fn evaluateDimension(
    mut dim: metamodelica::Ref<Dimension::NFDimension>,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut outDim: metamodelica::Ref<Dimension::NFDimension>;
    outDim = (match &*dim {
        Dimension::EXP {
            exp: __dim_exp,
            var: __dim_var,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            e = evaluateExp(__dim_exp.clone(), info)?;
            if (referenceEq(&*(&*e), &*(__dim_exp.clone()))) {
                dim
            } else {
                Dimension::fromExp(e, __dim_var.clone())?
            }
        }
        _ => dim,
    });
    Ok(outDim)
}

pub(crate) fn evaluateIfExp(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut info: SourceInfo,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outChanged: bool;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut tb: metamodelica::Ref<Expression::NFExpression>;
    let mut fb: metamodelica::Ref<Expression::NFExpression>;
    let mut c1: bool;
    let mut c2: bool;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*exp)) {
        Deref @ Expression::IF { ty: __pa0, condition: __pa1, trueBranch: __pa2, falseBranch: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    cond = metamodelica::Own::own(__pa1);
    tb = metamodelica::Own::own(__pa2);
    fb = metamodelica::Own::own(__pa3);
    (cond, outChanged) = evaluateExpTraverser(cond, info.clone(), false)?;
    cond = SimplifyExp::simplify(cond, false)?;
    (outExp, outChanged) = (match &*cond {
        Expression::BOOLEAN { value: __cond_value } => {
            (outExp, _) = evaluateExpTraverser(if (__cond_value.clone()) { tb } else { fb }, info, false)?;
            (outExp, true)
        }
        _ => {
            (tb, c1) = evaluateExpTraverser(tb, info.clone(), false)?;
            (fb, c2) = evaluateExpTraverser(fb, info, false)?;
            (
                metamodelica::Ref::new(Expression::NFExpression::IF {
                    ty: ty,
                    condition: cond,
                    trueBranch: tb,
                    falseBranch: fb,
                }),
                outChanged || c1 || c2,
            )
        }
    });
    Ok((outExp, outChanged))
}

pub(crate) fn evaluateEquations(
    mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut outEql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
        for mut e in (eql.clone()).into_iter().cloned() {
            let __x = evaluateEquation(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outEql)
}

pub(crate) fn evaluateEquation(
    mut eq: metamodelica::Ref<Equation::NFEquation>,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut eq: metamodelica::Ref<Equation::NFEquation> = eq;
    let mut info: SourceInfo = Equation::info(&eq);
    eq = (match &*eq {
        Equation::EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            scalarizeMode: __eq_scalarizeMode,
            scope: __eq_scope,
            source: __eq_source,
            ty: __eq_ty,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut ty: metamodelica::Ref<Type::NFType>;
            ty = Type::mapDims(
                __eq_ty.clone(),
                &({
                    let __pe_b1 = info.clone();
                    move |__pe_a0| evaluateDimension(__pe_a0, __pe_b1.clone())
                }),
            )?;
            e1 = evaluateExp(__eq_lhs.clone(), info.clone())?;
            e2 = evaluateExp(__eq_rhs.clone(), info)?;
            metamodelica::Ref::new(Equation::NFEquation::EQUALITY {
                lhs: e1,
                rhs: e2,
                ty: ty,
                scope: __eq_scope.clone(),
                source: __eq_source.clone(),
                scalarizeMode: __eq_scalarizeMode.clone(),
            })
        }
        Equation::FOR { range: __eq_range, .. } => {
            assign_variant_field!(eq => Equation::NFEquation::FOR;
                range = Util::applyOption(__eq_range.clone(), &({ let __pe_b1 = info; move |__pe_a0| evaluateExp(__pe_a0, __pe_b1.clone()) }))?,
                body = evaluateEquations(var_field!((*eq).body, Equation::NFEquation::FOR).clone())?
            );
            eq
        }
        Equation::IF {
            branches: __eq_branches,
            ..
        } => {
            assign_variant_field!(eq => Equation::NFEquation::IF; branches = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Branch::Branch>> = metamodelica::nil();
                for mut b in (__eq_branches.clone()).into_iter().cloned() {
                    let __x = evaluateEqBranch(b.clone(), info.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            eq
        }
        Equation::WHEN {
            branches: __eq_branches,
            ..
        } => {
            assign_variant_field!(eq => Equation::NFEquation::WHEN; branches = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Branch::Branch>> = metamodelica::nil();
                for mut b in (__eq_branches.clone()).into_iter().cloned() {
                    let __x = evaluateEqBranch(b.clone(), info.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            eq
        }
        Equation::ASSERT {
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut e3: metamodelica::Ref<Expression::NFExpression>;
            e1 = evaluateExp(__eq_condition.clone(), info.clone())?;
            e2 = evaluateExp(__eq_message.clone(), info.clone())?;
            e3 = evaluateExp(__eq_level.clone(), info)?;
            metamodelica::Ref::new(Equation::NFEquation::ASSERT {
                condition: e1,
                message: e2,
                level: e3,
                scope: __eq_scope.clone(),
                source: __eq_source.clone(),
            })
        }
        Equation::TERMINATE {
            message: __eq_message, ..
        } => {
            assign_variant_field!(eq => Equation::NFEquation::TERMINATE; message = evaluateExp(__eq_message.clone(), info)?);
            eq
        }
        Equation::REINIT {
            reinitExp: __eq_reinitExp,
            ..
        } => {
            assign_variant_field!(eq => Equation::NFEquation::REINIT; reinitExp = evaluateExp(__eq_reinitExp.clone(), info)?);
            eq
        }
        Equation::NORETCALL { exp: __eq_exp, .. } => {
            assign_variant_field!(eq => Equation::NFEquation::NORETCALL; exp = evaluateExp(__eq_exp.clone(), info)?);
            eq
        }
        _ => eq,
    });
    Ok(eq)
}

pub(crate) fn evaluateEqBranch(
    mut branch: metamodelica::Ref<Branch::Branch>,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Branch::Branch>> {
    let mut outBranch: metamodelica::Ref<Branch::Branch>;
    outBranch = (match &*branch {
        Equation::Branch::BRANCH {
            condition,
            body,
            conditionVar: __branch_conditionVar,
        } => {
            let mut condition = (*condition).clone();
            let mut body = (*body).clone();
            condition = evaluateExp(condition.clone(), info)?;
            body = evaluateEquations(body.clone())?;
            metamodelica::Ref::new(Branch::Branch::BRANCH {
                condition: condition.clone(),
                conditionVar: __branch_conditionVar.clone(),
                body: body.clone(),
            })
        }
        _ => branch,
    });
    Ok(outBranch)
}

pub(crate) fn evaluateAlgorithms(
    mut algs: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
) -> Result<metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>> {
    let mut outAlgs: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
        for mut a in (algs.clone()).into_iter().cloned() {
            let __x = evaluateAlgorithm(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outAlgs)
}

pub(crate) fn evaluateAlgorithm(
    mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>,
) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>> {
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = evaluateStatements(alg.statements.clone())?);
    Ok(alg)
}

pub(crate) fn evaluateStatements(
    mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
        for mut s in (stmts.clone()).into_iter().cloned() {
            let __x = evaluateStatement(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outStmts)
}

pub(crate) fn evaluateStatement(
    mut stmt: metamodelica::Ref<Statement::NFStatement>,
) -> Result<metamodelica::Ref<Statement::NFStatement>> {
    let mut stmt: metamodelica::Ref<Statement::NFStatement> = stmt;
    let mut info: SourceInfo = Statement::info(&stmt);
    stmt = (match &*stmt {
        Statement::ASSIGNMENT {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            source: __stmt_source,
            ty: __stmt_ty,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut ty: metamodelica::Ref<Type::NFType>;
            ty = Type::mapDims(
                __stmt_ty.clone(),
                &({
                    let __pe_b1 = info.clone();
                    move |__pe_a0| evaluateDimension(__pe_a0, __pe_b1.clone())
                }),
            )?;
            e1 = evaluateExp(__stmt_lhs.clone(), info.clone())?;
            e2 = evaluateExp(__stmt_rhs.clone(), info)?;
            metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT {
                lhs: e1,
                rhs: e2,
                ty: ty,
                source: __stmt_source.clone(),
            })
        }
        Statement::FOR {
            range: __stmt_range, ..
        } => {
            assign_variant_field!(stmt => Statement::NFStatement::FOR;
                range = Util::applyOption(__stmt_range.clone(), &({ let __pe_b1 = info; move |__pe_a0| evaluateExp(__pe_a0, __pe_b1.clone()) }))?,
                body = evaluateStatements(var_field!((*stmt).body, Statement::NFStatement::FOR).clone())?
            );
            stmt
        }
        Statement::IF {
            branches: __stmt_branches,
            ..
        } => {
            assign_variant_field!(stmt => Statement::NFStatement::IF; branches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<Statement::NFStatement>>)> = metamodelica::nil();
                for mut b in (__stmt_branches.clone()).into_iter().cloned() {
                    let __x = evaluateStmtBranch(&(b.clone()), info.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            stmt
        }
        Statement::WHEN {
            branches: __stmt_branches,
            ..
        } => {
            assign_variant_field!(stmt => Statement::NFStatement::WHEN; branches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<Statement::NFStatement>>)> = metamodelica::nil();
                for mut b in (__stmt_branches.clone()).into_iter().cloned() {
                    let __x = evaluateStmtBranch(&(b.clone()), info.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            stmt
        }
        Statement::ASSERT {
            condition: __stmt_condition,
            level: __stmt_level,
            message: __stmt_message,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut e3: metamodelica::Ref<Expression::NFExpression>;
            e1 = evaluateExp(__stmt_condition.clone(), info.clone())?;
            e2 = evaluateExp(__stmt_message.clone(), info.clone())?;
            e3 = evaluateExp(__stmt_level.clone(), info)?;
            metamodelica::Ref::new(Statement::NFStatement::ASSERT {
                condition: e1,
                message: e2,
                level: e3,
                source: __stmt_source.clone(),
            })
        }
        Statement::TERMINATE {
            message: __stmt_message,
            ..
        } => {
            assign_variant_field!(stmt => Statement::NFStatement::TERMINATE; message = evaluateExp(__stmt_message.clone(), info)?);
            stmt
        }
        Statement::REINIT {
            reinitExp: __stmt_reinitExp,
            ..
        } => {
            assign_variant_field!(stmt => Statement::NFStatement::REINIT; reinitExp = evaluateExp(__stmt_reinitExp.clone(), info)?);
            stmt
        }
        Statement::NORETCALL { exp: __stmt_exp, .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::NORETCALL; exp = evaluateExp(__stmt_exp.clone(), info)?);
            stmt
        }
        Statement::WHILE {
            condition: __stmt_condition,
            ..
        } => {
            assign_variant_field!(stmt => Statement::NFStatement::WHILE;
                condition = evaluateExp(__stmt_condition.clone(), info)?,
                body = evaluateStatements(var_field!((*stmt).body, Statement::NFStatement::WHILE).clone())?
            );
            stmt
        }
        _ => stmt,
    });
    Ok(stmt)
}

pub(crate) fn evaluateStmtBranch(
    mut branch: &(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    ),
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
)> {
    let mut outBranch: (
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    );
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    (cond, body) = branch.clone();
    cond = evaluateExp(cond, info)?;
    body = evaluateStatements(body)?;
    outBranch = (cond, body);
    Ok(outBranch)
}

pub(crate) fn evaluateFunction(
    mut func: metamodelica::Ref<Function::Function>,
) -> Result<metamodelica::Ref<Function::Function>> {
    let mut func: metamodelica::Ref<Function::Function> = func;
    let mut is_con: bool;
    if !(Function::isEvaluated(&func)) {
        Function::markEvaluated(&func);
        is_con = Function::isDefaultRecordConstructor(&func)?;
        func = Function::mapExp(
            func.clone(),
            (std::sync::Arc::new({
                let __pe_b1 = NFInstNode::InstNode::fromHandle(&func.node)?;
                let __pe_b2 = is_con;
                move |__pe_a0| evaluateFuncExp(__pe_a0, &__pe_b1, __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
            (std::sync::Arc::new({
                let __pe_b1 = NFInstNode::InstNode::fromHandle(&func.node)?;
                let __pe_b2 = true;
                move |__pe_a0| evaluateFuncExp(__pe_a0, &__pe_b1, __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
            true,
            true,
        )?;
        if is_con {
            Record::checkLocalFieldOrder(
                func.locals.clone(),
                &(NFInstNode::InstNode::fromHandle(&func.node)?),
                &(NFInstNode::InstNode::info(&(NFInstNode::InstNode::fromHandle(&func.node)?))),
            )?;
        }
        for mut fn_der in &*func.derivatives.clone() {
            for mut der_fn in &*Function::getCachedFuncs(NFInstNode::InstNode::borrow(fn_der.derivativeFn.clone())?)? {
                evaluateFunction(der_fn.clone())?;
            }
        }
    }
    Ok(func)
}

pub(crate) fn evaluateFuncExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut fnNode: &metamodelica::Ref<InstNode::InstNode>,
    mut evaluateAll: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    (outExp, _) = evaluateFuncExpTraverser(exp, fnNode, evaluateAll, false)?;
    Ok(outExp)
}

pub(crate) fn evaluateFuncExpTraverser(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut fnNode: &metamodelica::Ref<InstNode::InstNode>,
    mut evaluateAll: bool,
    mut changed: bool,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outChanged: bool;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    (e, outChanged) = Expression::mapFoldShallow(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = fnNode.clone();
            let __pe_b2 = evaluateAll;
            move |__pe_a0, __pe_a3| evaluateFuncExpTraverser(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_a3)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        bool,
                    ) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)>
                    + 'static,
            >),
        false,
    )?;
    outExp = (match &*e.clone() {
        Expression::CREF { cref: __e_cref, .. } => {
            if evaluateAll || !(isLocalFunctionVariable(metamodelica::AsArg::as_arg(&__e_cref), fnNode)?) {
                ErrorExt::setCheckpoint(literal!("NFEvalConstants.evaluateFuncExpTraverser"));
                match '__try0: {
                    outExp = unwrap_break_err!(Ceval::evalCref(metamodelica::AsArg::as_arg(&__e_cref), e.clone(), &(Ceval::noTarget().clone()), false, true), '__try0);
                    Ok::<_, &'static str>((outExp.clone(),))
                } {
                    Ok((__try0_o0,)) => {
                        outExp = __try0_o0;
                    }
                    Err(_) => {
                        outExp = e.clone();
                    }
                }
                ErrorExt::rollBack(literal!("NFEvalConstants.evaluateFuncExpTraverser"));
                outChanged = true;
            } else if outChanged {
                outExp = metamodelica::Ref::new(Expression::NFExpression::CREF {
                    ty: ComponentRef::getSubscriptedType(metamodelica::AsArg::as_arg(&__e_cref), false)?,
                    cref: __e_cref.clone(),
                });
            } else {
                outExp = e;
            }
            outExp
        }
        _ => {
            if (outChanged) {
                Expression::retype(e)?
            } else {
                e
            }
        }
    });
    outChanged = changed || outChanged;
    Ok((outExp, outChanged))
}

pub(crate) fn isLocalFunctionVariable(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut fnNode: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<bool> {
    let mut res: bool;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut fnl: metamodelica::List<metamodelica::Ref<Function::Function>>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    if ComponentRef::isPackageConstant(cref)? {
        res = false;
    } else if ComponentRef::nodeVariability(cref)? <= Variability::PARAMETER.clone() && ComponentRef::isCref(cref) {
        node = NFInstNode::InstNode::instanceParent(ComponentRef::node(&(ComponentRef::last(cref)))?)?;
        if NFInstNode::InstNode::isClass(&node)? {
            fnl = Function::getCachedFuncs(node)?;
            if (fnl).is_empty() {
                res = false;
            } else {
                r#fn = (fnl).head().cloned()?;
                res = NFInstNode::InstNode::refEqual(fnNode, &(NFInstNode::InstNode::fromHandle(&r#fn.node)?))?;
            }
        } else {
            res = false;
        }
    } else {
        res = true;
    }
    Ok(res)
}

pub(crate) fn evaluateRecordDeclaration(mut recordNode: metamodelica::Ref<InstNode::InstNode>) -> Result<()> {
    ClassTree::applyComponents(
        &(Class::classTree(NFInstNode::InstNode::getClass(recordNode.clone())?)?),
        &({
            let __pe_b1 = recordNode;
            move |__pe_a0| evaluateRecordDeclarationField(__pe_a0, &__pe_b1)
        }),
    )?;
    Ok(())
}

pub(crate) fn evaluateRecordDeclarationField(
    mut fieldNode: metamodelica::Ref<InstNode::InstNode>,
    mut recordNode: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<()> {
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut cls_inst: metamodelica::Ref<InstNode::InstNode>;
    comp = NFInstNode::InstNode::component(&fieldNode)?;
    binding = Component::getBinding(&comp);
    if Binding::isBound(&binding) {
        binding = Binding::mapExp(
            binding,
            (std::sync::Arc::new({
                let __pe_b1 = fieldNode.clone();
                let __pe_b2 = false;
                move |__pe_a0| evaluateFuncExp(__pe_a0, &__pe_b1, __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
        comp = Component::setBinding(binding, comp)?;
    }
    cls_inst = Component::classInstance(&comp)?;
    if !(NFInstNode::InstNode::isEmpty(&cls_inst)) {
        ClassTree::applyComponents(
            &(Class::classTree(NFInstNode::InstNode::getClass(cls_inst)?)?),
            &({
                let __pe_b1 = recordNode.clone();
                move |__pe_a0| evaluateRecordDeclarationField(__pe_a0, &__pe_b1)
            }),
        )?;
    }
    NFInstNode::InstNode::updateComponent(comp, fieldNode)?;
    Ok(())
}
