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
use crate::NFAttributes as Attributes;
use crate::NFBackendExtension::BackendInfo;
use crate::NFBackendExtension::VariableAttributes;
use crate::NFBinding as Binding;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten::FunctionTree;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::MutableWeak;

pub mod AttributeIterator {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct AttributeIterator {
        pub name: ArcStr,
        pub source: Binding::Source,
        pub confidence: i32,
        pub iterator: Mutable::Mutable<metamodelica::Ref<ExpressionIterator::NFExpressionIterator>>,
    }

    impl metamodelica::gc::MMTrace for AttributeIterator {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.source, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.confidence, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.iterator, __mmv)?;
            Ok(())
        }
    }
    impl Default for AttributeIterator {
        fn default() -> Self {
            Self {
                name: Default::default(),
                source: Default::default(),
                confidence: Default::default(),
                iterator: Default::default(),
            }
        }
    }

    pub type ATTRIBUTE_ITERATOR = AttributeIterator;

    pub(crate) fn create(
        mut attribute: &(ArcStr, metamodelica::Ref<Binding::NFBinding>),
    ) -> Result<metamodelica::Ref<AttributeIterator>> {
        let mut iter: metamodelica::Ref<AttributeIterator>;
        let mut name: ArcStr;
        let mut binding: metamodelica::Ref<Binding::NFBinding>;
        (name, binding) = attribute.clone();
        iter = metamodelica::Ref::new(AttributeIterator {
            name: name,
            source: Binding::source(&binding),
            confidence: Binding::confidence(&binding),
            iterator: Mutable::create(ExpressionIterator::fromBinding(&binding)?),
        });
        Ok(iter)
    }

    pub(crate) fn nextBinding(
        mut iter: &metamodelica::Ref<AttributeIterator>,
    ) -> Result<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> {
        let mut binding: (ArcStr, metamodelica::Ref<Binding::NFBinding>);
        let mut it: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>;
        let mut exp: metamodelica::Ref<Expression::NFExpression>;
        (it, exp) = ExpressionIterator::next(Mutable::access(iter.iterator.clone()))?;
        Mutable::update(iter.iterator.clone(), it);
        binding = (
            iter.name.clone(),
            Binding::makeFlat(
                exp,
                Variability::PARAMETER.clone(),
                iter.source.clone(),
                iter.confidence.clone(),
            ),
        );
        Ok(binding)
    }
}

pub fn scalarize(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    assign_field!(
        flatModel.variables = scalarizeVariables(&flatModel.variables, false)?,
        flatModel.equations = Equation::mapExpList(flatModel.equations.clone(), &expandComplexCref)?
    );
    assign_field!(
        flatModel.equations = scalarizeEquations(&flatModel.equations, false)?,
        flatModel.initialEquations = Equation::mapExpList(flatModel.initialEquations.clone(), &expandComplexCref)?
    );
    assign_field!(
        flatModel.initialEquations = scalarizeEquations(&flatModel.initialEquations, false)?,
        flatModel.algorithms = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
            for mut a in (flatModel.algorithms.clone()).into_iter().cloned() {
                let __x = scalarizeAlgorithm(a.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        flatModel.initialAlgorithms = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
            for mut a in (flatModel.initialAlgorithms.clone()).into_iter().cloned() {
                let __x = scalarizeAlgorithm(a.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    execStat(&(literal!("NFScalarize.scalarize")))?;
    Ok(flatModel)
}

pub(crate) fn scalarizeVariables(
    mut vars: &metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut forceScalarize: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Variable::NFVariable>>> {
    let mut outVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    for mut v in &**vars {
        outVars = scalarizeVariable(v.clone(), outVars, forceScalarize)?;
    }
    outVars = metamodelica::Dangerous::listReverseInPlace(outVars);
    Ok(outVars)
}

pub(crate) fn scalarizeVariable(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut forceScalarize: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Variable::NFVariable>>> {
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = vars;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut elem_ty: metamodelica::Ref<Type::NFType>;
    let mut vis: Visibility;
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    let mut ty_attr: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>;
    let mut cmt: metamodelica::Ref<SCode::Comment>;
    let mut info: SourceInfo;
    let mut binding_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> =
        crate::NFExpressionIterator::interned_NONE_ITERATOR();
    let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty_attr_iters: metamodelica::List<metamodelica::Ref<AttributeIterator::AttributeIterator>>;
    let mut backend_attributes: metamodelica::List<metamodelica::Ref<BackendInfo::BackendInfo>>;
    let mut bind_var: Variability;
    let mut binfo: metamodelica::Ref<BackendInfo::BackendInfo>;
    let mut bind_src: Binding::Source;
    let mut has_binding: bool;
    let mut confidence: i32;
    assign_field!(
        var.binding = Binding::mapExp(
            var.binding.clone(),
            (std::sync::Arc::new(expandComplexCref_traverser)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >)
        )?
    );
    if Type::isArray(&var.ty) && Type::hasKnownSize(var.ty.clone())? {
        if '__try0: {
            let __arc10 = var.clone();
            let Variable::VARIABLE { name: __pa1, ty: __pa2, binding: __pa3, visibility: __pa4, attributes: __pa5, typeAttributes: __pa6, children: _, comment: __pa7, info: __pa8, backendinfo: __pa9 } = &*__arc10;
            name = metamodelica::Own::own(__pa1);
            ty = metamodelica::Own::own(__pa2);
            binding = metamodelica::Own::own(__pa3);
            vis = metamodelica::Own::own(__pa4);
            attr = metamodelica::Own::own(__pa5);
            ty_attr = metamodelica::Own::own(__pa6);
            cmt = metamodelica::Own::own(__pa7);
            info = metamodelica::Own::own(__pa8);
            binfo = metamodelica::Own::own(__pa9);
            crefs = unwrap_break_err!(ComponentRef::scalarize(name.clone(), false), '__try0);
            if (crefs).is_empty() {
                return Ok(vars);
            }
            has_binding = Binding::isBound(&binding);
            bind_src = Binding::source(&binding);
            confidence = Binding::confidence(&binding);
            if has_binding {
                binding_iter = unwrap_break_err!(ExpressionIterator::fromExp(unwrap_break_err!(Binding::getTypedExp(&binding), '__try0), false, false), '__try0);
                bind_var = unwrap_break_err!(Binding::variability(&binding), '__try0);
                if !(forceScalarize) && unwrap_break_err!(ExpressionIterator::isSubscriptedArrayCall(&binding_iter, true), '__try0) && !(variableHasForcedScalarAttribute(&var)) {
                    vars = metamodelica::cons(var.clone(), vars.clone());
                    return Ok(vars);
                }
            } else {
                bind_var = Variability::CONSTANT.clone();
            }
            elem_ty = Type::arrayElementType(&ty);
            ty_attr_iters = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<AttributeIterator::AttributeIterator>> = metamodelica::nil();
        for mut a in (ty_attr.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(AttributeIterator::create(&(a.clone())), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            backend_attributes = unwrap_break_err!(BackendInfo::scalarize(binfo.clone(), ((crefs).len() as i32)), '__try0);
            for mut cr in &*crefs {
                if has_binding {
                    (binding_iter, exp) = unwrap_break_err!(ExpressionIterator::next(binding_iter.clone()), '__try0);
                    binding = Binding::makeFlat(exp.clone(), bind_var, bind_src, confidence);
                }
                ty_attr = ({
        let mut __acc: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> = metamodelica::nil();
        for mut i in (ty_attr_iters.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(AttributeIterator::nextBinding(&(i.clone())), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                let (__pa11, __pa12) = ::match_deref::match_deref! { match &(backend_attributes.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa11, tail: __pa12 } => (__pa11.clone(), __pa12.clone()),
                    _ => break '__try0 Err::<_, _>("pattern mismatch"),
                } };
                binfo = metamodelica::Own::own(__pa11);
                backend_attributes = metamodelica::Own::own(__pa12);
                vars = metamodelica::cons(metamodelica::Ref::new(Variable::NFVariable { name: cr.clone(), ty: elem_ty.clone(), binding: binding.clone(), visibility: vis, attributes: attr.clone(), typeAttributes: ty_attr.clone(), children: metamodelica::nil(), comment: cmt.clone(), info: info.clone(), backendinfo: binfo.clone() }), vars.clone());
            }
            Ok::<(), &'static str>(())
        }.is_err() {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFScalarize.scalarizeVariable")); __mm_s.push_str(&*literal!(" failed on ")); __mm_s.push_str(&*Variable::toString(&var, literal!(""), true)?); ArcStr::from(__mm_s) }, &(var.info.clone()))?;
        }
    } else {
        vars = metamodelica::cons(var, vars);
    }
    Ok(vars)
}

pub fn scalarizeBackendVariable(
    mut var: &metamodelica::Ref<Variable::NFVariable>,
    mut indices: metamodelica::List<i32>,
) -> Result<metamodelica::List<metamodelica::Ref<Variable::NFVariable>>> {
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut binding_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut bind_var: Variability;
    let mut bind_src: Binding::Source;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut elem_ty: metamodelica::Ref<Type::NFType>;
    let mut binfo: metamodelica::Ref<BackendInfo::BackendInfo>;
    let mut backend_attributes: metamodelica::List<metamodelica::Ref<BackendInfo::BackendInfo>>;
    let mut confidence: i32;
    if '__try0: {
        crefs = unwrap_break_err!(ComponentRef::scalarizeAll(ComponentRef::stripSubscriptsAll(&var.name), false), '__try0).reverse();
        elem_ty = Type::arrayElementType(&var.ty);
        backend_attributes = unwrap_break_err!(BackendInfo::scalarize(var.backendinfo.clone(), ((crefs).len() as i32)), '__try0);
        if Binding::isBound(&var.binding) {
            binding_iter = unwrap_break_err!(ExpressionIterator::fromExp(unwrap_break_err!(Binding::getTypedExp(&var.binding), '__try0), true, false), '__try0);
            bind_var = unwrap_break_err!(Binding::variability(&var.binding), '__try0);
            bind_src = Binding::source(&var.binding);
            confidence = Binding::confidence(&var.binding);
            vars = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
        for mut cr in (crefs.clone()).into_iter().cloned() {
            let __x = (match &*cr.clone() {
        _ => {
            (binding_iter, exp) = unwrap_break_err!(ExpressionIterator::next(binding_iter.clone()), '__try0);
            binding = Binding::makeFlat(exp.clone(), bind_var, bind_src, confidence);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(backend_attributes.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            binfo = metamodelica::Own::own(__pa0);
            backend_attributes = metamodelica::Own::own(__pa1);
            metamodelica::Ref::new(Variable::NFVariable { name: cr.clone(), ty: elem_ty.clone(), binding: binding.clone(), visibility: var.visibility.clone(), attributes: var.attributes.clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: var.comment.clone(), info: var.info.clone(), backendinfo: binfo.clone() })
        },
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        } else {
            vars = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
        for mut cr in (crefs.clone()).into_iter().cloned() {
            let __x = (match &*cr.clone() {
        _ => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(backend_attributes.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            binfo = metamodelica::Own::own(__pa0);
            backend_attributes = metamodelica::Own::own(__pa1);
            metamodelica::Ref::new(Variable::NFVariable { name: cr.clone(), ty: elem_ty.clone(), binding: var.binding.clone(), visibility: var.visibility.clone(), attributes: var.attributes.clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: var.comment.clone(), info: var.info.clone(), backendinfo: binfo.clone() })
        },
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        }
        if !((indices).is_empty() || ((indices).len() as i32) == ((vars).len() as i32)) {
            vars = unwrap_break_err!(List::keepPositions(vars.clone(), indices.clone(), true), '__try0);
        }
        Ok::<(), &'static str>(())
    }.is_err() {
        Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFScalarize.scalarizeBackendVariable")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Variable::toString(var, literal!(""), false)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFScalarize.mo")))?;
    }
    Ok(vars)
}

pub fn scalarizeComplexVariable(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<metamodelica::List<metamodelica::Ref<Variable::NFVariable>>> {
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = vars;
    vars = (::match_deref::match_deref! { match &(var.backendinfo.attributes.clone()) {
        attr @ Deref @ VariableAttributes::VAR_ATTR_RECORD { .. } => {
            let mut name: ArcStr;
            let mut index: i32;
            let mut elem_var: metamodelica::Ref<Variable::NFVariable>;
            for mut tpl in &*UnorderedMap::toList(var_field!((**attr).indexMap, VariableAttributes::VariableAttributes::VAR_ATTR_RECORD).clone()) {
                (name, index) = tpl.clone();
                elem_var = var.clone();
                assign_field!(
                    elem_var.name = ComponentRef::prepend(elem_var.name.clone(), ComponentRef::rename(name, elem_var.name.clone())?)?,
                    elem_var.backendinfo = BackendInfo::setAttributes(elem_var.backendinfo.clone(), ({let __elt = (*metamodelica::index_checked(&var_field!((**attr).childrenAttr, VariableAttributes::VariableAttributes::VAR_ATTR_RECORD).borrow(), index)?).clone(); __elt}), var.backendinfo.annotations.clone()),
                    elem_var.ty = VariableAttributes::elemType(&({let __elt = (*metamodelica::index_checked(&var_field!((**attr).childrenAttr, VariableAttributes::VariableAttributes::VAR_ATTR_RECORD).borrow(), index)?).clone(); __elt}))?
                );
                assign_field!(elem_var.name = ComponentRef::setNodeType(elem_var.ty.clone(), elem_var.name.clone()));
                vars = metamodelica::cons(elem_var, vars);
            }
            vars.reverse()
        },
        _ => {
            list![var]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(vars)
}

pub(crate) fn expandComplexCref(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = Expression::map(
        exp,
        (std::sync::Arc::new(expandComplexCref_traverser)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(exp)
}

pub(crate) fn expandComplexCref_traverser(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { ty: Deref @ Type::ARRAY { .. }, .. } => {
            if ComponentRef::isComplexArray(var_field!((*exp).cref, Expression::NFExpression::CREF))? {
                (exp, _) = ExpandExp::expand(exp, false, false)?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn scalarizeEquations(
    mut eql: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut forceScalarize: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    for mut eq in &**eql {
        equations = scalarizeEquation(eq.clone(), equations, forceScalarize)?;
    }
    equations = metamodelica::Dangerous::listReverseInPlace(equations);
    Ok(equations)
}

pub(crate) fn scalarizeEquation(
    mut eq: metamodelica::Ref<Equation::NFEquation>,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut forceScalarize: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    equations = (match &*eq.clone() {
        Equation::EQUALITY {
            lhs,
            rhs,
            ty,
            source: src,
            scalarizeMode: __eq_scalarizeMode,
            scope: __eq_scope,
        } if (Type::isArray(metamodelica::AsArg::as_arg(&ty))) => {
            let mut lhs_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>;
            let mut rhs_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>;
            let mut scalarize: bool;
            let mut lhs = (*lhs).clone();
            let mut rhs = (*rhs).clone();
            let mut ty = (*ty).clone();
            if forceScalarize || __eq_scalarizeMode.clone() == Equation::ScalarizeMode::SCALARIZE.clone() {
                scalarize = true;
            } else if __eq_scalarizeMode.clone() == Equation::ScalarizeMode::DONT_SCALARIZE.clone() {
                scalarize = false;
            } else if Expression::hasArrayCall(lhs.clone())? || Expression::hasArrayCall(rhs.clone())? {
                scalarize = false;
            } else {
                scalarize = true;
            }
            if scalarize {
                lhs_iter = ExpressionIterator::fromExp(lhs.clone(), false, false)?;
                rhs_iter = ExpressionIterator::fromExp(rhs.clone(), false, false)?;
                ty = Type::arrayElementType(metamodelica::AsArg::as_arg(&ty));
                while ExpressionIterator::hasNext(&lhs_iter) {
                    if !(ExpressionIterator::hasNext(&rhs_iter)) {
                        Error::addInternalError(
                            {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("NFScalarize.scalarizeEquation"));
                                __mm_s.push_str(&*literal!(" could not expand rhs "));
                                __mm_s.push_str(&*Expression::toString(
                                    var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone(),
                                )?);
                                ArcStr::from(__mm_s)
                            },
                            ElementSource::getInfo(src.clone()),
                        )?;
                    }
                    (lhs_iter, lhs) = ExpressionIterator::next(lhs_iter)?;
                    (rhs_iter, rhs) = ExpressionIterator::next(rhs_iter)?;
                    equations = metamodelica::cons(
                        Equation::makeEquality(
                            lhs.clone(),
                            rhs.clone(),
                            ty.clone(),
                            src.clone(),
                            NFInstNode::InstNode::fromCell(__eq_scope.clone())?,
                            Equation::ScalarizeMode::NO_PREFERENCE.clone(),
                        ),
                        equations,
                    );
                }
            } else {
                equations = metamodelica::cons(eq, equations);
            }
            equations
        }
        Equation::CONNECT { .. } => equations,
        Equation::IF {
            branches: __eq_branches,
            scope: __eq_scope,
            source: __eq_source,
        } => scalarizeIfEquation(
            metamodelica::AsArg::as_arg(&__eq_branches),
            __eq_scope.clone(),
            __eq_source.clone(),
            equations,
        )?,
        Equation::WHEN {
            branches: __eq_branches,
            scope: __eq_scope,
            source: __eq_source,
        } => scalarizeWhenEquation(
            metamodelica::AsArg::as_arg(&__eq_branches),
            __eq_scope.clone(),
            __eq_source.clone(),
            equations,
        )?,
        _ => metamodelica::cons(eq, equations),
    });
    Ok(equations)
}

pub(crate) fn scalarizeIfEquation(
    mut branches: &metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>,
    mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    let mut bl: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>> = metamodelica::nil();
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut var: Variability;
    for mut b in &**branches {
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(b.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __pa0, conditionVar: __pa1, body: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cond = metamodelica::Own::own(__pa0);
        var = metamodelica::Own::own(__pa1);
        body = metamodelica::Own::own(__pa2);
        body = scalarizeEquations(&body, false)?;
        if !((body).is_empty()) {
            bl = metamodelica::cons(Equation::makeBranch(cond, body, var), bl);
        }
    }
    if !((bl).is_empty()) {
        equations = metamodelica::cons(
            metamodelica::Ref::new(Equation::NFEquation::IF {
                branches: metamodelica::Dangerous::listReverseInPlace(bl),
                scope: scope,
                source: source,
            }),
            equations,
        );
    }
    Ok(equations)
}

pub(crate) fn scalarizeWhenEquation(
    mut branches: &metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>,
    mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    let mut bl: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>> = metamodelica::nil();
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut var: Variability;
    for mut b in &**branches {
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(b.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __pa0, conditionVar: __pa1, body: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cond = metamodelica::Own::own(__pa0);
        var = metamodelica::Own::own(__pa1);
        body = metamodelica::Own::own(__pa2);
        body = scalarizeEquations(&body, false)?;
        if Type::isArray(&(Expression::typeOf(cond.clone()))) {
            (cond, _) = ExpandExp::expand(cond, false, false)?;
        }
        bl = metamodelica::cons(Equation::makeBranch(cond, body, var), bl);
    }
    equations = metamodelica::cons(
        metamodelica::Ref::new(Equation::NFEquation::WHEN {
            branches: metamodelica::Dangerous::listReverseInPlace(bl),
            scope: scope,
            source: source,
        }),
        equations,
    );
    Ok(equations)
}

pub(crate) fn scalarizeAlgorithm(
    mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>,
) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>> {
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = scalarizeStatements(&alg.statements)?);
    Ok(alg)
}

pub(crate) fn scalarizeStatements(
    mut stmts: &metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
    for mut s in &**stmts {
        statements = scalarizeStatement(s.clone(), statements)?;
    }
    statements = metamodelica::Dangerous::listReverseInPlace(statements);
    Ok(statements)
}

pub(crate) fn scalarizeStatement(
    mut stmt: metamodelica::Ref<Statement::NFStatement>,
    mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = statements;
    statements = (match &*stmt {
        Statement::FOR {
            body: __stmt_body,
            forType: __stmt_forType,
            iterator: __stmt_iterator,
            range: __stmt_range,
            source: __stmt_source,
            sub_iters: __stmt_sub_iters,
        } => metamodelica::cons(
            metamodelica::Ref::new(Statement::NFStatement::FOR {
                iterator: __stmt_iterator.clone(),
                range: __stmt_range.clone(),
                body: scalarizeStatements(metamodelica::AsArg::as_arg(&__stmt_body))?,
                forType: __stmt_forType.clone(),
                source: __stmt_source.clone(),
                sub_iters: __stmt_sub_iters.clone(),
            }),
            statements,
        ),
        Statement::IF {
            branches: __stmt_branches,
            source: __stmt_source,
        } => scalarizeIfStatement(
            metamodelica::AsArg::as_arg(&__stmt_branches),
            __stmt_source.clone(),
            statements,
        )?,
        Statement::WHEN {
            branches: __stmt_branches,
            source: __stmt_source,
        } => scalarizeWhenStatement(
            metamodelica::AsArg::as_arg(&__stmt_branches),
            __stmt_source.clone(),
            statements,
        )?,
        Statement::WHILE {
            body: __stmt_body,
            condition: __stmt_condition,
            source: __stmt_source,
        } => metamodelica::cons(
            metamodelica::Ref::new(Statement::NFStatement::WHILE {
                condition: __stmt_condition.clone(),
                body: scalarizeStatements(metamodelica::AsArg::as_arg(&__stmt_body))?,
                source: __stmt_source.clone(),
            }),
            statements,
        ),
        _ => metamodelica::cons(stmt, statements),
    });
    Ok(statements)
}

pub(crate) fn scalarizeIfStatement(
    mut branches: &metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    )>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = statements;
    let mut bl: metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    )> = metamodelica::nil();
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    for mut b in &**branches {
        (cond, body) = b.clone();
        body = scalarizeStatements(&body)?;
        if !((body).is_empty()) {
            bl = metamodelica::cons((cond, body), bl);
        }
    }
    if !((bl).is_empty()) {
        statements = metamodelica::cons(
            metamodelica::Ref::new(Statement::NFStatement::IF {
                branches: metamodelica::Dangerous::listReverseInPlace(bl),
                source: source,
            }),
            statements,
        );
    }
    Ok(statements)
}

pub(crate) fn scalarizeWhenStatement(
    mut branches: &metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    )>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = statements;
    let mut bl: metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    )> = metamodelica::nil();
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    for mut b in &**branches {
        (cond, body) = b.clone();
        body = scalarizeStatements(&body)?;
        if Type::isArray(&(Expression::typeOf(cond.clone()))) {
            (cond, _) = ExpandExp::expand(cond, false, false)?;
        }
        bl = metamodelica::cons((cond, body), bl);
    }
    statements = metamodelica::cons(
        metamodelica::Ref::new(Statement::NFStatement::WHEN {
            branches: metamodelica::Dangerous::listReverseInPlace(bl),
            source: source,
        }),
        statements,
    );
    Ok(statements)
}

pub(crate) fn variableHasForcedScalarAttribute(mut var: &metamodelica::Ref<Variable::NFVariable>) -> bool {
    let mut res: bool;
    for mut attribute in &*list![literal!("min"), literal!("max"), literal!("nominal")] {
        if Binding::isBound(&(Variable::lookupTypeAttribute(metamodelica::AsArg::as_arg(&attribute), var))) {
            res = true;
            return res;
        }
    }
    res = false;
    res
}
