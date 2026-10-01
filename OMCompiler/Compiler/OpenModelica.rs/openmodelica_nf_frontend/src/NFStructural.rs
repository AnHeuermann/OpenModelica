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

use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFSubscript as Subscript;
use openmodelica_util::Error;
use openmodelica_util::Util;

pub(crate) fn isStructuralComponent(
    mut component: &metamodelica::Ref<Component::NFComponent>,
    mut compAttrs: &metamodelica::Ref<Attributes::NFAttributes>,
    mut compBinding: metamodelica::Ref<Binding::NFBinding>,
    mut compNode: metamodelica::Ref<InstNode::InstNode>,
    mut compEval: bool,
    mut parentEval: bool,
    mut context: i32,
) -> Result<bool> {
    let mut isStructural: bool;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    if compAttrs.variability.clone() != Variability::PARAMETER.clone() {
        isStructural = false;
    } else if compEval || parentEval {
        binding = if (Binding::isBound(&compBinding)) {
            compBinding
        } else {
            Component::getTypeAttributeBinding(component, literal!("start"))
        };
        if !(Component::isFixed(component)?) {
            isStructural = false;
        } else if Component::isExternalObject(component)? {
            isStructural = false;
        } else if !(Binding::isBound(&binding) || InstNode::hasBinding(compNode.clone())?) {
            if !(parentEval) && !(InstContext::inRelaxed(context)) {
                Error::addSourceMessage(
                    &(Error::UNBOUND_PARAMETER_EVALUATE_TRUE.clone()),
                    list![InstNode::name(&compNode)?],
                    &(InstNode::info(&compNode)),
                )?;
            }
            isStructural = false;
        } else if isBindingNotFixed(&binding, false, 4)? {
            isStructural = false;
        } else {
            isStructural = true;
        }
    } else {
        isStructural = false;
    }
    Ok(isStructural)
}

pub(crate) fn isBindingNotFixed(
    mut binding: &metamodelica::Ref<Binding::NFBinding>,
    mut requireFinal: bool,
    mut maxDepth: i32,
) -> Result<bool> {
    let mut isNotFixed: bool;
    if maxDepth == 0 {
        isNotFixed = true;
        return Ok(isNotFixed);
    }
    if Binding::hasExp(binding) {
        isNotFixed = isExpressionNotFixed(&(Binding::getExp(binding)?), requireFinal, maxDepth)?;
    } else {
        isNotFixed = true;
    }
    Ok(isNotFixed)
}

pub(crate) fn isComponentBindingNotFixed(
    mut component: metamodelica::Ref<Component::NFComponent>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut requireFinal: bool,
    mut maxDepth: i32,
    mut isRecord: bool,
) -> Result<bool> {
    '__tco: loop {
        let mut binding: metamodelica::Ref<Binding::NFBinding>;
        let mut parent: metamodelica::Ref<InstNode::InstNode>;
        binding = Component::getBinding(&component);
        if Binding::isUnbound(&binding) {
            if isRecord || InstNode::isRecord(node.clone())? {
                return Ok(false);
            } else {
                parent = InstNode::parent(&node)?;
                if InstNode::isComponent(&parent)? && InstNode::isRecord(parent.clone())? {
                    {
                        (component, node, requireFinal, maxDepth, isRecord) =
                            (InstNode::component(&parent)?, parent, requireFinal, maxDepth, true);
                        continue '__tco;
                    }
                } else {
                    binding = Component::getTypeAttributeBinding(&component, literal!("start"));
                    return Ok(isBindingNotFixed(&binding, requireFinal, maxDepth)?);
                }
            }
        } else {
            return Ok(isBindingNotFixed(&binding, requireFinal, maxDepth)?);
        }
    }
}

pub(crate) fn isExpressionNotFixed(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut requireFinal: bool,
    mut maxDepth: i32,
) -> Result<bool> {
    let mut isNotFixed: bool;
    isNotFixed = (match &**exp {
        Expression::CREF { cref: __exp_cref, .. }
            if (ComponentRef::isCref(metamodelica::AsArg::as_arg(&__exp_cref))
                && !(ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__exp_cref)))) =>
        {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            let mut c: metamodelica::Ref<Component::NFComponent>;
            let mut var: Variability;
            node = ComponentRef::node(metamodelica::AsArg::as_arg(&__exp_cref))?;
            if InstNode::isComponent(&node)? {
                c = InstNode::component(&node)?;
                var = Component::variability(&c)?;
                if var <= Variability::STRUCTURAL_PARAMETER.clone()
                    || ComponentRef::isResizable(metamodelica::AsArg::as_arg(&__exp_cref))?
                {
                    isNotFixed = false;
                } else if var == Variability::PARAMETER.clone()
                    && (!(requireFinal) || Component::isFinal(&c)?)
                    && !(Component::isExternalObject(&c)?)
                    && Component::isFixed(&c)?
                {
                    isNotFixed = isComponentBindingNotFixed(c, node, requireFinal, maxDepth - 1, false)?;
                } else {
                    isNotFixed = true;
                }
            } else {
                isNotFixed = true;
            }
            isNotFixed
                || Expression::containsShallow(
                    exp,
                    &({
                        let __pe_b1 = requireFinal;
                        let __pe_b2 = maxDepth;
                        move |__pe_a0| isExpressionNotFixed(&__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?
        }
        Expression::SIZE {
            dimIndex: __exp_dimIndex,
            ..
        } => {
            if (__exp_dimIndex).is_some() {
                isNotFixed = isExpressionNotFixed(&(Util::getOption(__exp_dimIndex.clone())?), requireFinal, maxDepth)?;
            } else {
                isNotFixed = false;
            }
            isNotFixed
        }
        Expression::CALL { call: __exp_call } => {
            if Call::isImpure(metamodelica::AsArg::as_arg(&__exp_call))?
                || Call::isExternal(metamodelica::AsArg::as_arg(&__exp_call))?
            {
                isNotFixed = true;
            } else {
                isNotFixed = Expression::containsShallow(
                    exp,
                    &({
                        let __pe_b1 = requireFinal;
                        let __pe_b2 = maxDepth;
                        move |__pe_a0| isExpressionNotFixed(&__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?;
            }
            isNotFixed
        }
        _ => Expression::containsShallow(
            exp,
            &({
                let __pe_b1 = requireFinal;
                let __pe_b2 = maxDepth;
                move |__pe_a0| isExpressionNotFixed(&__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            }),
        )?,
    });
    Ok(isNotFixed)
}

pub(crate) fn markDimension(mut dimension: &metamodelica::Ref<Dimension::NFDimension>) -> Result<()> {
    let () = (match &**dimension {
        Dimension::UNTYPED {
            dimension: __dimension_dimension,
            ..
        } => {
            markExp(metamodelica::AsArg::as_arg(&__dimension_dimension))?;
            ()
        }
        Dimension::EXP {
            exp: __dimension_exp, ..
        } => {
            markExp(metamodelica::AsArg::as_arg(&__dimension_exp))?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn markExp(mut exp: &metamodelica::Ref<Expression::NFExpression>) -> Result<()> {
    use crate::NFComponentRef::Origin;
    let () = (::match_deref::match_deref! { match exp {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { origin: ComponentRef::Origin::CREF { .. }, .. }, .. } => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            let mut comp: metamodelica::Ref<Component::NFComponent>;
            node = ComponentRef::node(var_field!((**exp).cref, Expression::NFExpression::CREF))?;
            if InstNode::isComponent(&node)? {
                comp = InstNode::component(&node)?;
                if Component::variability(&comp)? == Variability::PARAMETER.clone() {
                    markComponent(comp, node)?;
                }
            }
            Expression::applyShallow(exp, &move |__a0: metamodelica::Ref<Expression::NFExpression>| markExp(&__a0))?;
            ()
        },
        Deref @ Expression::SIZE { dimIndex: __exp_dimIndex, exp: __exp_exp } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            markSubscriptsInExp(metamodelica::AsArg::as_arg(&__exp_exp))?;
            if (__exp_dimIndex).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(__exp_dimIndex.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                e = metamodelica::Own::own(__pa0);
                markExp(&e)?;
            }
            ()
        },
        _ => {
            Expression::applyShallow(exp, &move |__a0: metamodelica::Ref<Expression::NFExpression>| markExp(&__a0))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn markSubscriptsInExp(mut exp: &metamodelica::Ref<Expression::NFExpression>) -> Result<()> {
    let () = (match &**exp {
        Expression::CREF { cref: __exp_cref, .. } => {
            ComponentRef::applySubscripts(
                metamodelica::AsArg::as_arg(&__exp_cref),
                &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| markSubscript(&__a0),
                false,
            )?;
            ()
        }
        _ => {
            Expression::applyShallow(exp, &move |__a0: metamodelica::Ref<Expression::NFExpression>| {
                markSubscriptsInExp(&__a0)
            })?;
            ()
        }
    });
    Ok(())
}

pub(crate) fn markComponent(
    mut component: metamodelica::Ref<Component::NFComponent>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
) -> Result<()> {
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut binding: Option<metamodelica::Ref<Expression::NFExpression>>;
    comp = Component::setVariability(Variability::STRUCTURAL_PARAMETER.clone(), component);
    comp = Component::setFinal(comp, true);
    InstNode::updateComponent(comp.clone(), node)?;
    binding = Binding::getExpOpt(&(Component::getBinding(&comp)));
    if (binding).is_some() {
        markExp(&(Util::getOption(binding)?))?;
    }
    Ok(())
}

pub(crate) fn markExpSize(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<()> {
    Expression::apply(exp, &move |__a0: metamodelica::Ref<Expression::NFExpression>| {
        markExpSize_traverser(&__a0)
    })?;
    Ok(())
}

pub(crate) fn markExpSize_traverser(mut exp: &metamodelica::Ref<Expression::NFExpression>) -> Result<()> {
    let () = (::match_deref::match_deref! { match exp {
        Deref @ Expression::CALL { call: Deref @ Call::UNTYPED_ARRAY_CONSTRUCTOR { iters, .. } } => {
            for mut iter in &*iters.clone() {
                markExp(&(Util::tuple22(iter.clone())))?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn markSubscripts(mut exp: &metamodelica::Ref<Expression::NFExpression>) -> Result<()> {
    let () = (match &**exp {
        Expression::CREF { cref: __exp_cref, .. } => {
            ComponentRef::applySubscripts(
                metamodelica::AsArg::as_arg(&__exp_cref),
                &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| markSubscript(&__a0),
                false,
            )?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn markSubscript(mut sub: &metamodelica::Ref<Subscript::NFSubscript>) -> Result<()> {
    let () = (match &**sub {
        Subscript::UNTYPED { exp: __sub_exp } => {
            markExp(metamodelica::AsArg::as_arg(&__sub_exp))?;
            ()
        }
        Subscript::INDEX { index: __sub_index } => {
            markExp(metamodelica::AsArg::as_arg(&__sub_index))?;
            ()
        }
        Subscript::SLICE { slice: __sub_slice } => {
            markExp(metamodelica::AsArg::as_arg(&__sub_slice))?;
            ()
        }
        _ => (),
    });
    Ok(())
}
