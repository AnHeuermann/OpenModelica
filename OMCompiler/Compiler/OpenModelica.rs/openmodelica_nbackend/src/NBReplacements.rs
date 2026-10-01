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

use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointers;
use crate::NBInline as Inline;
use crate::NBSlice as Slice;
use crate::NBSolve as Solve;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstContext as InstContext;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFPrefixes;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFTyping as Typing;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:        NBReplacements.mo
/// package:     NBReplacements
/// description:
///  Replacements consists of a mapping between variables and expressions, the first binary tree of this type.
///  To eliminate a variable from an equation system a replacement rule varname->expression is added to this
///  datatype.
///  To be able to update these replacement rules incrementally a backward lookup mechanism is also required.
///  For instance, having a rule a->b and adding a rule b->c requires to find the first rule a->b and update it to
///  a->c. This is what the second binary tree is used for.
pub struct NBReplacements;
pub(crate) fn single(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut old: &metamodelica::Ref<Expression::NFExpression>,
    mut new: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    fn traverse(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut old: metamodelica::Ref<Expression::NFExpression>,
        mut new: metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        exp = if (Expression::isEqual(exp.clone(), old.clone())?) {
            new.clone()
        } else {
            exp
        };
        Ok(exp)
    }

    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = Expression::map(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = old.clone();
            let __pe_b2 = new.clone();
            move |__pe_a0| traverse(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
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

pub(crate) fn simple(
    mut comps: &metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<()> {
    for mut comp in &**comps {
        addSimple(metamodelica::AsArg::as_arg(&comp), replacements.clone())?;
    }
    Ok(())
}

pub(crate) fn addSimple(
    mut comp: &metamodelica::Ref<StrongComponent::NBStrongComponent>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<()> {
    let () = (match &**comp {
        StrongComponent::SINGLE_COMPONENT {
            eqn: __comp_eqn,
            var: __comp_var,
            ..
        } => {
            let mut varName: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut solvedEq: metamodelica::Ref<Equation::Equation>;
            let mut status: Solve::Status;
            let mut replace_exp: metamodelica::Ref<Expression::NFExpression>;
            varName = BVariable::getVarName(__comp_var.clone());
            (solvedEq, status, _) = Solve::solveBody(
                Pointer::access(__comp_eqn.clone()),
                varName.clone(),
                UnorderedMap::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
                    })
                        as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static>),
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<Absyn::Path>,
                              __a1: metamodelica::Ref<Absyn::Path>|
                              -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Absyn::Path>,
                                    metamodelica::Ref<Absyn::Path>,
                                ) -> Result<bool>
                                + 'static,
                        >),
                    1,
                ),
            )?;
            if status == Solve::Status::EXPLICIT.clone() {
                let __pa0 = ::match_deref::match_deref! { match &(Equation::getRHS(solvedEq)?) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                replace_exp = metamodelica::Own::own(__pa0);
                replace_exp = Expression::map(
                    replace_exp,
                    (std::sync::Arc::new({
                        let __pe_b1 = replacements.clone();
                        move |__pe_a0| applySimpleExp(__pe_a0, __pe_b1.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?;
                replace_exp = SimplifyExp::simplifyDump(
                    replace_exp,
                    true,
                    &(literal!("NBReplacements.addSimple")),
                    &(literal!("")),
                )?;
                addInputArgTpl(&((varName, replace_exp)), replacements, true)?;
            } else {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBReplacements.addSimple"));
                        __mm_s.push_str(&*literal!(
                            " failed because strong component cannot be solved explicitly: "
                        ));
                        __mm_s.push_str(&*StrongComponent::toString(comp, -1)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
            ()
        }
        StrongComponent::SLICED_COMPONENT {
            eqn: __comp_eqn,
            var: __comp_var,
            ..
        } => {
            let mut varName: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut solvedEq: metamodelica::Ref<Equation::Equation>;
            let mut status: Solve::Status;
            let mut replace_exp: metamodelica::Ref<Expression::NFExpression>;
            varName = BVariable::getVarName(Slice::getT(__comp_var.clone()));
            (solvedEq, status, _) = Solve::solveBody(
                Pointer::access(Slice::getT(__comp_eqn.clone())),
                varName.clone(),
                UnorderedMap::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
                    })
                        as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static>),
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<Absyn::Path>,
                              __a1: metamodelica::Ref<Absyn::Path>|
                              -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Absyn::Path>,
                                    metamodelica::Ref<Absyn::Path>,
                                ) -> Result<bool>
                                + 'static,
                        >),
                    1,
                ),
            )?;
            if status == Solve::Status::EXPLICIT.clone() {
                let __pa0 = ::match_deref::match_deref! { match &(Equation::getRHS(solvedEq)?) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                replace_exp = metamodelica::Own::own(__pa0);
                replace_exp = Expression::map(
                    replace_exp,
                    (std::sync::Arc::new({
                        let __pe_b1 = replacements.clone();
                        move |__pe_a0| applySimpleExp(__pe_a0, __pe_b1.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?;
                replace_exp = SimplifyExp::simplifyDump(
                    replace_exp,
                    true,
                    &(literal!("NBReplacements.addSimple")),
                    &(literal!("")),
                )?;
                addInputArgTpl(&((varName, replace_exp)), replacements, true)?;
            } else {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBReplacements.addSimple"));
                        __mm_s.push_str(&*literal!(
                            " failed because strong component cannot be solved explicitly: "
                        ));
                        __mm_s.push_str(&*StrongComponent::toString(comp, -1)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
            ()
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBReplacements.addSimple"));
                    __mm_s.push_str(&*literal!(" failed because strong component is not simple: "));
                    __mm_s.push_str(&*StrongComponent::toString(comp, -1)?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(())
}

pub(crate) fn applySimple(
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<(metamodelica::Ref<EqData::EqData>, metamodelica::Ref<VarData::VarData>)> {
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut entries: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
    )>;
    let mut aliasCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut replacement: metamodelica::Ref<Expression::NFExpression>;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    if UnorderedMap::isEmpty(replacements.clone()) {
        return Ok((eqData, varData));
    }
    eqData = EqData::mapExp(
        eqData,
        (std::sync::Arc::new({
            let __pe_b1 = replacements.clone();
            move |__pe_a0| applySimpleExp(__pe_a0, __pe_b1.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
        Some(
            (std::sync::Arc::new({
                let __pe_b1 = replacements.clone();
                move |__pe_a0| applySimpleCref(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                        )
                            -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                        + 'static,
                >),
        ),
    )?;
    varData = (match &*varData {
        BVariable::VarData::VAR_DATA_SIM {
            variables: __varData_variables,
            ..
        } => {
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM;
                variables = BVariable::VariablePointers::map(__varData_variables.clone(), &({ let __pe_b1 = replacements.clone(); move |__pe_a0| applySimpleVar(__pe_a0, __pe_b1.clone()) }))?,
                aliasVars = BVariable::VariablePointers::map(var_field!((*varData).aliasVars, VarData::VarData::VAR_DATA_SIM).clone(), &({ let __pe_b1 = replacements.clone(); move |__pe_a0| applySimpleVar(__pe_a0, __pe_b1.clone()) }))?
            );
            varData
        }
        BVariable::VarData::VAR_DATA_JAC {
            variables: __varData_variables,
            ..
        } => {
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_JAC; variables = BVariable::VariablePointers::map(__varData_variables.clone(), &({ let __pe_b1 = replacements.clone(); move |__pe_a0| applySimpleVar(__pe_a0, __pe_b1.clone()) }))?);
            varData
        }
        BVariable::VarData::VAR_DATA_HES {
            variables: __varData_variables,
            ..
        } => {
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_HES; variables = BVariable::VariablePointers::map(__varData_variables.clone(), &({ let __pe_b1 = replacements.clone(); move |__pe_a0| applySimpleVar(__pe_a0, __pe_b1.clone()) }))?);
            varData
        }
        _ => return Err("match: no arm matched"),
    });
    entries = UnorderedMap::toList(replacements);
    for mut entry in &*entries {
        (aliasCref, replacement) = entry.clone();
        var_ptr = BVariable::getVarPointer(&aliasCref, metamodelica::sourceInfo!("NBackEnd/Util/NBReplacements.mo"))?;
        var = Pointer::access(var_ptr.clone());
        assign_field!(var.binding = Binding::update(var.binding.clone(), &replacement)?);
        Pointer::update(var_ptr, var);
    }
    Ok((eqData, varData))
}

pub(crate) fn applySimpleExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp.clone() {
        Expression::CREF { cref: __exp_cref, .. } => {
            let mut res: metamodelica::Ref<Expression::NFExpression>;
            let mut stripped: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            if UnorderedMap::contains(__exp_cref.clone(), replacements.clone())? {
                res = UnorderedMap::getOrFail(__exp_cref.clone(), replacements)?;
            } else {
                stripped = ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&__exp_cref));
                if UnorderedMap::contains(stripped.clone(), replacements.clone())? {
                    subs = ComponentRef::subscriptsAllWithWholeFlat(metamodelica::AsArg::as_arg(&__exp_cref))?;
                    res = UnorderedMap::getOrFail(stripped, replacements)?;
                    res = Expression::applySubscripts(&subs, res, true)?;
                } else {
                    res = exp;
                }
            }
            res
        }
        _ => exp,
    });
    Ok(exp)
}

pub(crate) fn applySimpleCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut replacement: metamodelica::Ref<Expression::NFExpression>;
    if UnorderedMap::contains(cref.clone(), replacements.clone())? {
        replacement = UnorderedMap::getOrFail(cref.clone(), replacements)?;
        cref = (match &*replacement {
            Expression::CREF {
                cref: __replacement_cref,
                ..
            } => __replacement_cref.clone(),
            _ => cref,
        });
    }
    Ok(cref)
}

pub(crate) fn applySimpleVar(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { binding: binding @ Deref @ Binding::TYPED_BINDING { .. }, .. } => {
            let mut binding = (*binding).clone();
            assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING; bindingExp = Expression::map(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(), (std::sync::Arc::new({ let __pe_b1 = replacements; move |__pe_a0| applySimpleExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?);
            assign_field!(var.binding = binding.clone());
            var
        },
        _ => {
            var
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub(crate) fn replaceVarPtr(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> = var_ptr;
    let mut cref: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    cref = UnorderedMap::get(BVariable::getVarName(var_ptr.clone()), replacements)?;
    if (cref).is_some() {
        var_ptr = BVariable::getVarPointer(
            &(Util::getOption(cref)?),
            metamodelica::sourceInfo!("NBackEnd/Util/NBReplacements.mo"),
        )?;
    }
    Ok(var_ptr)
}

pub(crate) fn simpleToString(
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<ArcStr> {
    let mut r#str: ArcStr = literal!("");
    let mut entries: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
    )>;
    let mut constStr: ArcStr = literal!("");
    let mut aliasStr: ArcStr = literal!("");
    let mut nonTrivialStr: ArcStr = literal!("");
    let mut key: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut value: metamodelica::Ref<Expression::NFExpression>;
    entries = UnorderedMap::toList(replacements);
    for mut entry in &*entries {
        (key, value) = entry.clone();
        if Expression::isConstNumber(&value) {
            constStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*constStr);
                __mm_s.push_str(&*literal!("\t"));
                __mm_s.push_str(&*ComponentRef::toString(&key)?);
                __mm_s.push_str(&*literal!("\t ==> \t"));
                __mm_s.push_str(&*Expression::toString(value)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        } else if !(Expression::isTrivialCref(&value)) {
            nonTrivialStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*nonTrivialStr);
                __mm_s.push_str(&*literal!("\t"));
                __mm_s.push_str(&*ComponentRef::toString(&key)?);
                __mm_s.push_str(&*literal!("\t ==> \t"));
                __mm_s.push_str(&*Expression::toString(value)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        } else {
            aliasStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*aliasStr);
                __mm_s.push_str(&*literal!("\t"));
                __mm_s.push_str(&*ComponentRef::toString(&key)?);
                __mm_s.push_str(&*literal!("\t ==> \t"));
                __mm_s.push_str(&*Expression::toString(value)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
    }
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*StringUtil::headline_4(
            &(literal!("[dumprepl] Constant Replacements:")),
        )?);
        __mm_s.push_str(&*constStr);
        ArcStr::from(__mm_s)
    };
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*StringUtil::headline_4(
            &(literal!("[dumprepl] Trivial Alias Replacements:")),
        )?);
        __mm_s.push_str(&*aliasStr);
        ArcStr::from(__mm_s)
    };
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*StringUtil::headline_4(
            &(literal!("[dumprepl] Nontrivial Alias Replacements:")),
        )?);
        __mm_s.push_str(&*nonTrivialStr);
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn replaceFunctions(
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Function::Function>>,
    >,
) -> Result<metamodelica::Ref<EqData::EqData>> {
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut prev_replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    > = UnorderedMap::new(
        (std::sync::Arc::new(Expression::hash)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(Expression::isEqual)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    if UnorderedMap::isEmpty(replacements.clone()) {
        return Ok(eqData);
    }
    eqData = EqData::mapExp(
        eqData,
        (std::sync::Arc::new({
            let __pe_b1 = replacements;
            let __pe_b2 = prev_replacements;
            let __pe_b3 = variables.clone();
            move |__pe_a0| applyFuncExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_b3)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
        None,
    )?;
    Ok(eqData)
}

pub(crate) fn applyFuncExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut prev_replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { r#fn, .. } } if (UnorderedMap::contains(r#fn.path.clone(), replacements.clone())?) => {
            let mut local_replacements: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Expression::NFExpression>>>;
            let mut input_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut local_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut binding_exp_opt: Option<metamodelica::Ref<Expression::NFExpression>>;
            let mut binding_exp: metamodelica::Ref<Expression::NFExpression>;
            let mut body_exp: metamodelica::Ref<Expression::NFExpression>;
            let mut res_exp: metamodelica::Ref<Expression::NFExpression>;
            let mut r#fn = (*r#fn).clone();
            res_exp = (::match_deref::match_deref! { match &(UnorderedMap::get(exp.clone(), prev_replacements.clone())?) {
        Some(__esc_res_exp) => {
            res_exp = (*__esc_res_exp).clone();
            res_exp.clone()
        },
        _ => {
            r#fn = UnorderedMap::getOrFail(r#fn.path.clone(), replacements.clone())?;
            local_replacements = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            input_crefs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        for mut node in (r#fn.inputs.clone()).into_iter().cloned() {
            let __x = ComponentRef::fromNode(node.clone(), InstNode::getType(node.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            for mut tpl in &*List::zip(input_crefs.clone(), var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()) {
                addInputArgTpl(&(tpl.clone()), local_replacements.clone(), false)?;
            }
            for mut local_node in &*r#fn.locals.clone() {
                local_cref = ComponentRef::fromNode(local_node.clone(), InstNode::getType(local_node.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone())?;
                binding_exp_opt = InstNode::getBindingExpOpt(metamodelica::AsArg::as_arg(&local_node))?;
                if (binding_exp_opt).is_some() {
                    binding_exp = Expression::map(Util::getOption(binding_exp_opt)?, (std::sync::Arc::new({ let __pe_b1 = local_replacements.clone(); move |__pe_a0| applySimpleExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                } else {
                    binding_exp = metamodelica::Ref::new(Expression::NFExpression::CREF { ty: openmodelica_nf_frontend::NFType::interned_UNKNOWN(), cref: openmodelica_nf_frontend::NFComponentRef::interned_WILD() });
                }
                addInputArgTpl(&((local_cref, binding_exp)), local_replacements.clone(), false)?;
            }
            body_exp = Function::getSingleBodyExp(metamodelica::AsArg::as_arg(&r#fn))?;
            body_exp = Expression::map(body_exp, (std::sync::Arc::new({ let __pe_b1 = local_replacements; move |__pe_a0| applySimpleExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            if !(List::all(&input_crefs, &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::sizeKnown(&__a0))?) {
                (body_exp, _, _, _) = Typing::typeExp(body_exp, InstContext::RHS.clone(), &(metamodelica::sourceInfo!("NBackEnd/Util/NBReplacements.mo")), true)?;
            }
            body_exp = SimplifyExp::combineBinaries(body_exp)?;
            body_exp = SimplifyExp::simplifyDump(body_exp, true, &(literal!("NBReplacements.applyFuncExp")), &(literal!("")))?;
            res_exp = Expression::map(body_exp.clone(), (std::sync::Arc::new({ let __pe_b1 = replacements.clone(); let __pe_b2 = prev_replacements.clone(); let __pe_b3 = variables.clone(); move |__pe_a0| applyFuncExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_b3) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            if !(r#fn.attributes.generateEvents.clone()) {
                res_exp = Expression::fakeMap(res_exp, &wrapEvents)?;
            }
            UnorderedMap::add(exp.clone(), res_exp.clone(), prev_replacements)?;
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*literal!("NBReplacements.applyFuncExp")); __mm_s.push_str(&*literal!("] Inlining: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Result: ")); __mm_s.push_str(&*Expression::toString(body_exp)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
            }
            res_exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            res_exp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn addInputArgTpl(
    mut tpl: &(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
    ),
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut lowered_lhs: bool,
) -> Result<()> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut children_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut children: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut tmp: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    (cref, arg) = tpl.clone();
    UnorderedMap::add(cref.clone(), arg.clone(), replacements.clone())?;
    children = if (lowered_lhs) {
        BVariable::getRecordChildrenCref(&cref)?
    } else {
        ComponentRef::getRecordChildren(cref)?
    };
    if !((children).is_empty()) {
        children_args = (::match_deref::match_deref! { match &(arg.clone()) {
            Deref @ Expression::CREF { cref: __arg_cref, .. } => {
                tmp = BVariable::getRecordChildrenCref(metamodelica::AsArg::as_arg(&__arg_cref))?;
                ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut child in (tmp).into_iter().cloned() {
                let __x = recordChildArg(child.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
            },
            Deref @ Expression::RECORD { elements: __arg_elements, .. } => __arg_elements.clone(),
            Deref @ Expression::TUPLE { elements: __arg_elements, .. } => __arg_elements.clone(),
            Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_CALL { r#fn: __esc_fn, .. } } => {
                call = (*__esc_call).clone();
                r#fn = (*__esc_fn).clone();
                if Function::isDefaultRecordConstructor(metamodelica::AsArg::as_arg(&r#fn))? {
                    children_args = var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone();
                } else if Function::isNonDefaultRecordConstructor(metamodelica::AsArg::as_arg(&r#fn)) {
                    children_args = var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone();
                } else {
                    children_args = Expression::getRecordElements(arg)?;
                }
                children_args
            },
            _ => Expression::getRecordElements(arg)?,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if List::compareLength(children.clone(), children_args.clone())? == 0 {
            for mut child_tpl in &*List::zip(children, children_args) {
                addInputArgTpl(&(child_tpl.clone()), replacements.clone(), lowered_lhs)?;
            }
        }
    }
    Ok(())
}

pub(crate) fn recordChildArg(
    mut child: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = Expression::fromCref(child.clone(), false)?;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut binding: Option<metamodelica::Ref<Expression::NFExpression>>;
    if (ComponentRef::subscriptsAllFlat(&child)?).is_empty() {
        var_ptr = BVariable::getVarPointer(&child, metamodelica::sourceInfo!("NBackEnd/Util/NBReplacements.mo"))?;
        if BVariable::isConst(var_ptr.clone()) {
            var = Pointer::access(var_ptr);
            binding = Binding::typedExp(&var.binding);
            if (binding).is_some() && Expression::isLiteral(&(Util::getOption(binding.clone())?))? {
                exp = Util::getOption(binding)?;
            }
        }
    }
    Ok(exp)
}

pub(crate) fn wrapEvents(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::IF {
            condition: __exp_condition,
            ..
        } => {
            assign_variant_field!(exp => Expression::NFExpression::IF;
                        condition = (match &*__exp_condition.clone() {
                Expression::CALL { .. } if (Expression::isCallNamed(metamodelica::AsArg::as_arg(&__exp_condition), &(literal!("noEvent")))?) => __exp_condition.clone(),
                _ => metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::NO_EVENT().clone(), list![__exp_condition.clone()], Expression::variability(__exp_condition.clone())?, NFPrefixes::Purity::PURE.clone(), NFBuiltinFuncs::NO_EVENT().returnType.clone()) }),
            }),
                        trueBranch = Expression::mapShallow(var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone(), (std::sync::Arc::new(wrapEvents) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
                        falseBranch = Expression::mapShallow(var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone(), (std::sync::Arc::new(wrapEvents) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?
                    );
            exp
        }
        Expression::RELATION { .. } => metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(
                NFBuiltinFuncs::NO_EVENT().clone(),
                list![exp.clone()],
                Expression::variability(exp)?,
                NFPrefixes::Purity::PURE.clone(),
                NFBuiltinFuncs::NO_EVENT().returnType.clone(),
            ),
        }),
        Expression::LBINARY { .. } => metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(
                NFBuiltinFuncs::NO_EVENT().clone(),
                list![exp.clone()],
                Expression::variability(exp)?,
                NFPrefixes::Purity::PURE.clone(),
                NFBuiltinFuncs::NO_EVENT().returnType.clone(),
            ),
        }),
        Expression::LUNARY { .. } => metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(
                NFBuiltinFuncs::NO_EVENT().clone(),
                list![exp.clone()],
                Expression::variability(exp)?,
                NFPrefixes::Purity::PURE.clone(),
                NFBuiltinFuncs::NO_EVENT().returnType.clone(),
            ),
        }),
        _ => Expression::mapShallow(
            exp,
            (std::sync::Arc::new(wrapEvents)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
    });
    Ok(exp)
}
