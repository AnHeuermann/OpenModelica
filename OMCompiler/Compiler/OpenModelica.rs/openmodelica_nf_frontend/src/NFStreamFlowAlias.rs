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

use crate::NFBinding as Binding;
use crate::NFCeval as Ceval;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes::Variability;
use crate::NFStructural as Structural;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::DisjointSets;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub mod FlowAlias {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct FlowAlias {
        pub name: metamodelica::Ref<ComponentRef::NFComponentRef>,
        pub negative: bool,
        pub variable: Option<metamodelica::Ref<Variable::NFVariable>>,
    }

    impl metamodelica::gc::MMTrace for FlowAlias {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.negative, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.variable, __mmv)?;
            Ok(())
        }
    }
    impl Default for FlowAlias {
        fn default() -> Self {
            Self {
                name: Default::default(),
                negative: Default::default(),
                variable: Default::default(),
            }
        }
    }

    pub type FLOW_ALIAS = FlowAlias;

    pub(crate) fn isNonFlow(mut alias: &metamodelica::Ref<FlowAlias>) -> Result<bool> {
        let mut isNonFlow: bool =
            (alias.variable).is_some() && !(Variable::isFlow(&(Util::getOption(alias.variable.clone())?)));
        Ok(isNonFlow)
    }
}

pub(crate) fn EntryHash(mut entry: Entry) -> Result<i32> {
    let mut hash: i32;
    hash = ComponentRef::hash(&entry.name)?;
    Ok(hash)
}

pub(crate) fn EntryEqual(mut entry1: Entry, mut entry2: Entry) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = ComponentRef::isEqual(&entry1.name, &entry2.name)?;
    Ok(isEqual)
}

pub(crate) fn EntryString(mut entry: Entry) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ComponentRef::toString(&entry.name)?;
    if entry.negative.clone() {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("-"));
            __mm_s.push_str(&*r#str);
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

pub type Replacements = metamodelica::Ref<
    UnorderedMap::UnorderedMap<
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
    >,
>;

pub(crate) fn eliminateAliases(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
    mut vars: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
) -> Result<(metamodelica::Ref<FlatModel::NFFlatModel>, Replacements)> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut replacements: Replacements;
    let mut sets: Sets;
    let mut aliases: metamodelica::List<(
        metamodelica::Ref<FlowAlias::FlowAlias>,
        metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>,
    )>;
    (flatModel, sets) = fromModel(flatModel)?;
    (flatModel, aliases) = createAliases(&sets, vars.clone(), flatModel)?;
    replacements = buildReplacements(&aliases)?;
    flatModel = applyReplacements(replacements.clone(), flatModel)?;
    findConstantBindings(&flatModel, vars)?;
    Ok((flatModel, replacements))
}

pub(crate) fn fromModel(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<(metamodelica::Ref<FlatModel::NFFlatModel>, Sets)> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut sets: Sets;
    let mut alias_eqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut other_eqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut flow_aliases: metamodelica::List<metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>>;
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    let mut opt_alias: Option<metamodelica::Ref<FlowAlias::FlowAlias>>;
    let mut alias: metamodelica::Ref<FlowAlias::FlowAlias> =
        <metamodelica::Ref<FlowAlias::FlowAlias> as ::std::default::Default>::default();
    sets = emptySets(0);
    (alias_eqs, flow_aliases, other_eqs) = sortEquations(&flatModel.equations)?;
    assign_field!(flatModel.equations = other_eqs);
    sets = List::threadFold(
        &flow_aliases,
        alias_eqs,
        &move |__a0: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>,
               __a1: metamodelica::Ref<Equation::NFEquation>,
               __a2: Sets| addAliasEquation(&__a0, &__a1, __a2),
        sets,
    )?;
    sets = List::fold(
        &flatModel.variables,
        &move |__a0: metamodelica::Ref<Variable::NFVariable>, __a1: Sets| addAliasBinding(&__a0, __a1),
        sets,
    )?;
    for mut v in &*flatModel.variables.clone() {
        alias = metamodelica::Ref::new(FlowAlias::FlowAlias {
            name: v.name.clone(),
            negative: false,
            variable: None,
        });
        opt_alias = getEntry(alias, &sets)?;
        if (opt_alias).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(opt_alias) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            alias = metamodelica::Own::own(__pa0);
            assign_field!(alias.variable = Some(v.clone()));
            UnorderedMap::updateKey(alias, sets.elements.clone())?;
        } else {
            vars = metamodelica::cons(v.clone(), vars);
        }
    }
    for mut alias in &*UnorderedMap::keyList(sets.elements.clone()) {
        let mut alias = alias.clone();
        if (alias.variable).is_none() {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFStreamFlowAlias.fromModel"));
                    __mm_s.push_str(&*literal!(": "));
                    __mm_s.push_str(&*ComponentRef::toString(&alias.name)?);
                    __mm_s.push_str(&*literal!(" has no associated variable"));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("NFFrontEnd/NFStreamFlowAlias.mo"),
            )?;
        }
    }
    assign_field!(flatModel.variables = metamodelica::Dangerous::listReverseInPlace(vars));
    Ok((flatModel, sets))
}

pub(crate) fn sortEquations(
    mut eqs: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>>,
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
)> {
    let mut aliasEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut flowAliases: metamodelica::List<metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>> =
        metamodelica::nil();
    let mut otherEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut aliases: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    for mut eq in &**eqs {
        let mut eq = eq.clone();
        aliases = getAliasVarsFromEq(&eq)?;
        if (aliases).is_empty() {
            otherEqs = metamodelica::cons(eq, otherEqs);
        } else {
            src = Equation::source(&eq);
            src = ElementSource::addAdditionalComment(&src, literal!("alias equation"));
            eq = Equation::setSource(src, eq);
            aliasEqs = metamodelica::cons(eq, aliasEqs);
            flowAliases = metamodelica::cons(aliases, flowAliases);
        }
    }
    aliasEqs = metamodelica::Dangerous::listReverseInPlace(aliasEqs);
    flowAliases = metamodelica::Dangerous::listReverseInPlace(flowAliases);
    otherEqs = metamodelica::Dangerous::listReverseInPlace(otherEqs);
    Ok((aliasEqs, flowAliases, otherEqs))
}

pub(crate) fn addAliasEquation(
    mut aliases: &metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>,
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut sets: Sets,
) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut scalar_aliases1: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>;
    let mut scalar_aliases2: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>;
    let mut alias1: metamodelica::Ref<FlowAlias::FlowAlias>;
    let mut alias2: metamodelica::Ref<FlowAlias::FlowAlias>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*aliases)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    alias1 = metamodelica::Own::own(__pa0);
    alias2 = metamodelica::Own::own(__pa1);
    if Equation::isArrayEquality(eq) {
        scalar_aliases1 = scalarizeAlias(&alias1)?;
        scalar_aliases2 = scalarizeAlias(&alias2)?;
        sets = List::threadFold(&scalar_aliases1, scalar_aliases2, &addAliasPair, sets)?;
    } else {
        sets = addAliasPair(alias1, alias2, sets)?;
    }
    Ok(sets)
}

pub(crate) fn addAliasBinding(mut var: &metamodelica::Ref<Variable::NFVariable>, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut bind_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut aliases: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>;
    let mut alias1: metamodelica::Ref<FlowAlias::FlowAlias>;
    let mut alias2: metamodelica::Ref<FlowAlias::FlowAlias>;
    if Binding::hasExp(&var.binding) {
        bind_exp = Binding::getExp(&var.binding)?;
        aliases = getAliasVarsFromExpPair(
            &(Expression::fromTypedCref(var.name.clone(), var.ty.clone())),
            &bind_exp,
        )?;
        if !((aliases).is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(aliases) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            alias1 = metamodelica::Own::own(__pa0);
            alias2 = metamodelica::Own::own(__pa1);
            sets = addAliasPair(alias1, alias2, sets)?;
        }
    }
    Ok(sets)
}

pub(crate) fn addAliasPair(
    mut alias1: metamodelica::Ref<FlowAlias::FlowAlias>,
    mut alias2: metamodelica::Ref<FlowAlias::FlowAlias>,
    mut sets: Sets,
) -> Result<Sets> {
    fn find_set(mut alias: metamodelica::Ref<FlowAlias::FlowAlias>, mut sets: Sets) -> Result<(i32, Sets, bool)> {
        let mut set: i32;
        let mut sets: Sets = sets;
        let mut flippedSign: bool;
        let mut entry: metamodelica::Ref<FlowAlias::FlowAlias>;
        (set, sets) = findSet(alias.clone(), sets)?;
        let __pa0 = ::match_deref::match_deref! { match &(getEntry(alias.clone(), &sets)?) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        entry = metamodelica::Own::own(__pa0);
        flippedSign = entry.negative.clone() != alias.negative.clone();
        Ok((set, sets, flippedSign))
    }

    let mut sets: Sets = sets;
    let mut set1: i32;
    let mut set2: i32;
    let mut root1: i32;
    let mut root2: i32;
    let mut flipped_sign1: bool;
    let mut flipped_sign2: bool;
    (set1, sets, flipped_sign1) = find_set(alias1.clone(), sets)?;
    (set2, sets, flipped_sign2) = find_set(alias2, sets)?;
    if flipped_sign1 != flipped_sign2 {
        root1 = findRoot(set1, sets.nodes.clone())?;
        root2 = findRoot(set2, sets.nodes.clone())?;
        if root1 == root2 {
            return Ok(sets);
        }
        sets = negateSet(if (alias1.negative.clone()) { root1 } else { root2 }, sets)?;
    }
    sets = union(set1, set2, sets)?;
    Ok(sets)
}

pub(crate) fn getAliasVarsFromEq(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
) -> Result<metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>> {
    let mut aliases: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>> = metamodelica::nil();
    aliases = (match &**eq {
        Equation::EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => getAliasVarsFromExpPair(
            metamodelica::AsArg::as_arg(&__eq_lhs),
            metamodelica::AsArg::as_arg(&__eq_rhs),
        )?,
        _ => metamodelica::nil(),
    });
    Ok(aliases)
}

pub(crate) fn getAliasVarsFromExpPair(
    mut exp1: &metamodelica::Ref<Expression::NFExpression>,
    mut exp2: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>> {
    let mut aliases: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>;
    aliases = getAliasVarsFromExp(exp1, exp2, metamodelica::nil())?;
    aliases = getAliasVarsFromExp(exp2, exp1, aliases)?;
    if ((aliases).len() as i32) != 2
        || List::none(&aliases, &move |__a0: metamodelica::Ref<FlowAlias::FlowAlias>| {
            isStreamConnectorFlow(&__a0)
        })?
    {
        aliases = metamodelica::nil();
    }
    Ok(aliases)
}

pub(crate) fn getAliasVarsFromExp(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut otherExp: &metamodelica::Ref<Expression::NFExpression>,
    mut aliases: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>,
) -> Result<metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>> {
    let mut aliases: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>> = aliases;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    aliases = (::match_deref::match_deref! { match exp {
        Deref @ Expression::CREF { cref: __exp_cref, .. } if (ComponentRef::nodeVariability(metamodelica::AsArg::as_arg(&__exp_cref))? > Variability::DISCRETE.clone()) => metamodelica::cons(metamodelica::Ref::new(FlowAlias::FlowAlias { name: __exp_cref.clone(), negative: false, variable: None }), aliases),
        Deref @ Expression::UNARY { exp: e @ Deref @ Expression::CREF { .. }, .. } if (ComponentRef::nodeVariability(var_field!((**e).cref, Expression::NFExpression::CREF))? > Variability::DISCRETE.clone()) => metamodelica::cons(metamodelica::Ref::new(FlowAlias::FlowAlias { name: var_field!((**e).cref, Expression::NFExpression::CREF).clone(), negative: true, variable: None }), aliases),
        Deref @ Expression::BINARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. }, exp1: __exp_exp1, exp2: __exp_exp2 } if (Expression::isZero(otherExp)?) => getAliasVarsFromSum(metamodelica::AsArg::as_arg(&__exp_exp1), false, metamodelica::AsArg::as_arg(&__exp_exp2), false, aliases)?,
        Deref @ Expression::MULTARY { arguments: __exp_arguments, inv_arguments: __exp_inv_arguments, operator: __exp_operator } if (Expression::isZero(otherExp)? && Operator::getMathClassification(metamodelica::AsArg::as_arg(&__exp_operator))? == Operator::MathClassification::ADDITION.clone()) => (::match_deref::match_deref! { match &((__exp_arguments.clone(), __exp_inv_arguments.clone())) {
        (Deref @ metamodelica::ListNode::Cons { head: __esc_e1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_e2, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Nil) => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            getAliasVarsFromSum(metamodelica::AsArg::as_arg(&e1), false, metamodelica::AsArg::as_arg(&e2), false, aliases)?
        },
        (Deref @ metamodelica::ListNode::Cons { head: __esc_e1, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: __esc_e2, tail: Deref @ metamodelica::ListNode::Nil }) => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            getAliasVarsFromSum(metamodelica::AsArg::as_arg(&e1), false, metamodelica::AsArg::as_arg(&e2), true, aliases)?
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: __esc_e1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_e2, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            getAliasVarsFromSum(metamodelica::AsArg::as_arg(&e1), true, metamodelica::AsArg::as_arg(&e2), true, aliases)?
        },
        _ => aliases,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => aliases,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(aliases)
}

pub(crate) fn getAliasVarsFromSum(
    mut exp1: &metamodelica::Ref<Expression::NFExpression>,
    mut inv1: bool,
    mut exp2: &metamodelica::Ref<Expression::NFExpression>,
    mut inv2: bool,
    mut aliases: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>,
) -> Result<metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>> {
    let mut aliases: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>> = aliases;
    let mut aliases1: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>;
    let mut aliases2: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>;
    let mut alias1: metamodelica::Ref<FlowAlias::FlowAlias>;
    let mut alias2: metamodelica::Ref<FlowAlias::FlowAlias>;
    aliases1 = getAliasVarsFromExp(exp1, exp2, metamodelica::nil())?;
    aliases2 = getAliasVarsFromExp(exp2, exp1, metamodelica::nil())?;
    if ((aliases1).len() as i32) == 1 && ((aliases2).len() as i32) == 1 {
        let __pa0 = ::match_deref::match_deref! { match &(aliases1) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        alias1 = metamodelica::Own::own(__pa0);
        let __pa2 = ::match_deref::match_deref! { match &(aliases2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } => __pa2.clone(),
            _ => return Err("pattern mismatch"),
        } };
        alias2 = metamodelica::Own::own(__pa2);
        assign_field!(alias1.negative = alias1.negative.clone() != inv1);
        assign_field!(alias2.negative = !(alias2.negative.clone() != inv2));
        aliases = metamodelica::cons(alias1, metamodelica::cons(alias2, aliases));
    }
    Ok(aliases)
}

pub(crate) fn isStreamConnectorFlow(mut alias: &metamodelica::Ref<FlowAlias::FlowAlias>) -> Result<bool> {
    let mut isStreamFlow: bool;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    if !(ComponentRef::isQualified(&alias.name)) {
        isStreamFlow = false;
        return Ok(isStreamFlow);
    }
    node = ComponentRef::node(&alias.name)?;
    if !(InstNode::isComponent(&node)?) || !(Component::isFlow(&(InstNode::component(&node)?))) {
        isStreamFlow = false;
        return Ok(isStreamFlow);
    }
    isStreamFlow = Type::isStreamConnector(&(ComponentRef::nodeType(&(ComponentRef::rest(&alias.name)?))?));
    Ok(isStreamFlow)
}

pub(crate) fn scalarizeAlias(
    mut alias: &metamodelica::Ref<FlowAlias::FlowAlias>,
) -> Result<metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>> {
    let mut scalarAliases: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>> = metamodelica::nil();
    let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    crefs = ComponentRef::scalarize(alias.name.clone(), false)?;
    crefs = metamodelica::Dangerous::listReverseInPlace(crefs);
    for mut cr in &*crefs {
        scalarAliases = metamodelica::cons(
            metamodelica::Ref::new(FlowAlias::FlowAlias {
                name: cr.clone(),
                negative: alias.negative.clone(),
                variable: None,
            }),
            scalarAliases,
        );
    }
    Ok(scalarAliases)
}

pub(crate) fn negateSet(mut set: i32, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut nodes: metamodelica::Array<i32>;
    let mut indices: metamodelica::Array<i32>;
    let mut elements: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<FlowAlias::FlowAlias>, i32>>;
    let mut root: i32;
    let mut alias: metamodelica::Ref<FlowAlias::FlowAlias>;
    nodes = sets.nodes.clone();
    elements = sets.elements.clone();
    root = findRoot(set, nodes.clone())?;
    indices = UnorderedMap::valueArray(elements.clone());
    for mut i in 1..=metamodelica::arrayLength(indices.clone()) {
        if findRoot(i, nodes.clone())? == root {
            alias = UnorderedMap::keyAt(elements.clone(), i)?;
            assign_field!(alias.negative = !(alias.negative.clone()));
            Vector::updateNoBounds(elements.keys.clone(), i, alias);
        }
    }
    Ok(sets)
}

pub(crate) fn createAliases(
    mut sets: &Sets,
    mut vars: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<(
    metamodelica::Ref<FlatModel::NFFlatModel>,
    metamodelica::List<(
        metamodelica::Ref<FlowAlias::FlowAlias>,
        metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>,
    )>,
)> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut aliases: metamodelica::List<(
        metamodelica::Ref<FlowAlias::FlowAlias>,
        metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>,
    )> = metamodelica::nil();
    let mut extracted_sets: metamodelica::Array<metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>>;
    let mut representative: metamodelica::Ref<FlowAlias::FlowAlias>;
    let mut repr_var: metamodelica::Ref<Variable::NFVariable>;
    let mut rest_aliases: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>;
    let mut repr_binding: metamodelica::Ref<Binding::NFBinding>;
    let mut alias_vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    let mut alias_eqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut negated: bool;
    (extracted_sets, _) = extractSets(sets)?;
    let __range0 = extracted_sets.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut set in __range0 {
        (representative, rest_aliases) = defineRepresentative(set)?;
        let __pa1 = ::match_deref::match_deref! { match &(representative.variable.clone()) {
            Some(__pa1) => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        repr_var = metamodelica::Own::own(__pa1);
        alias_vars = metamodelica::cons(repr_var.clone(), alias_vars);
        let true = (UnorderedMap::tryUpdate(repr_var.name.clone(), repr_var.clone(), vars.clone())?) else {
            return Err("pattern mismatch");
        };
        repr_binding = Variable::asBinding(&repr_var, Binding::Source::GENERATED.clone());
        for mut alias in &*rest_aliases {
            let mut alias = alias.clone();
            negated = representative.negative.clone() != alias.negative.clone();
            (alias, alias_eqs) = defineAlias(alias, repr_binding.clone(), negated, alias_eqs)?;
            alias_vars = metamodelica::cons(Util::getOption(alias.variable.clone())?, alias_vars);
        }
        aliases = metamodelica::cons((representative, rest_aliases), aliases);
    }
    aliases = metamodelica::Dangerous::listReverseInPlace(aliases);
    assign_field!(
        flatModel.variables = listAppend(
            flatModel.variables.clone(),
            metamodelica::Dangerous::listReverseInPlace(alias_vars)
        ),
        flatModel.equations = listAppend(
            flatModel.equations.clone(),
            metamodelica::Dangerous::listReverseInPlace(alias_eqs)
        )
    );
    Ok((flatModel, aliases))
}

pub(crate) fn buildReplacements(
    mut aliases: &metamodelica::List<(
        metamodelica::Ref<FlowAlias::FlowAlias>,
        metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>,
    )>,
) -> Result<Replacements> {
    let mut replacements: Replacements;
    let mut representative: metamodelica::Ref<FlowAlias::FlowAlias>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut negative_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut rest_aliases: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>;
    replacements = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                  __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    for mut set in &**aliases {
        (representative, rest_aliases) = set.clone();
        exp = Expression::fromCref(representative.name.clone(), false)?;
        exp = if (representative.negative.clone()) {
            Expression::negate(exp)
        } else {
            exp
        };
        negative_exp = Expression::negate(exp.clone());
        for mut alias in &*rest_aliases {
            UnorderedMap::addUnique(
                alias.name.clone(),
                if (alias.negative.clone()) {
                    negative_exp.clone()
                } else {
                    exp.clone()
                },
                replacements.clone(),
            )?;
        }
    }
    Ok(replacements)
}

pub(crate) fn applyReplacements(
    mut replacements: Replacements,
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    flatModel = FlatModel::mapExp(
        flatModel,
        &({
            let __pe_b0 = replacements;
            move |__pe_a1| applyReplacementsInExp(__pe_b0.clone(), __pe_a1)
        }),
    )?;
    Ok(flatModel)
}

pub(crate) fn applyReplacementsInEql(
    mut replacements: Replacements,
    mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = eql;
    eql = Equation::mapExpList(
        eql,
        &({
            let __pe_b0 = replacements;
            move |__pe_a1| applyReplacementsInExp(__pe_b0.clone(), __pe_a1)
        }),
    )?;
    Ok(eql)
}

pub(crate) fn applyReplacementsInExp(
    mut replacements: Replacements,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression> = Expression::map(
        exp.clone(),
        (std::sync::Arc::new({
            let __pe_b0 = replacements.clone();
            move |__pe_a1| applyReplacementsInExp_traverser(__pe_b0.clone(), __pe_a1)
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

pub(crate) fn applyReplacementsInExp_traverser(
    mut replacements: Replacements,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut opt_val: Option<metamodelica::Ref<Expression::NFExpression>>;
    exp = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. } => {
            opt_val = UnorderedMap::get(__exp_cref.clone(), replacements)?;
            if ((opt_val).is_some()) {
                Util::getOption(opt_val)?
            } else {
                exp
            }
        }
        _ => exp,
    });
    Ok(exp)
}

pub(crate) fn defineRepresentative(
    mut aliases: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>,
) -> Result<(
    metamodelica::Ref<FlowAlias::FlowAlias>,
    metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>,
)> {
    let mut representative: metamodelica::Ref<FlowAlias::FlowAlias>;
    let mut restAliases: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>;
    let mut start_values: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Binding::NFBinding>,
    )> = metamodelica::nil();
    let mut fixed_start_values: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Binding::NFBinding>,
    )> = metamodelica::nil();
    let mut nominal_values: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Binding::NFBinding>,
    )> = metamodelica::nil();
    let mut min_values: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut max_values: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut accum_aliases: metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>> = metamodelica::nil();
    let mut start_binding: metamodelica::Ref<Binding::NFBinding>;
    let mut nominal_binding: metamodelica::Ref<Binding::NFBinding>;
    let mut min_binding: metamodelica::Ref<Binding::NFBinding>;
    let mut max_binding: metamodelica::Ref<Binding::NFBinding>;
    let mut negated: bool;
    match '__try0: {
        (representative, restAliases) = unwrap_break_err!(List::findAndRemove(aliases.clone(), &move |__a0: metamodelica::Ref<FlowAlias::FlowAlias>| FlowAlias::isNonFlow(&__a0)), '__try0);
        Ok::<_, &'static str>((representative.clone(), restAliases.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            representative = __try0_o0;
            restAliases = __try0_o1;
        }
        Err(_) => {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(aliases.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            representative = metamodelica::Own::own(__pa1);
            restAliases = metamodelica::Own::own(__pa2);
        }
    }
    for mut alias in &*metamodelica::cons(representative.clone(), restAliases.clone()) {
        let mut alias = alias.clone();
        negated = representative.negative.clone() != alias.negative.clone();
        (
            alias,
            start_values,
            fixed_start_values,
            nominal_values,
            min_values,
            max_values,
        ) = evalAliasAttributes(
            alias,
            negated,
            start_values,
            fixed_start_values,
            nominal_values,
            min_values,
            max_values,
        )?;
        accum_aliases = metamodelica::cons(alias, accum_aliases);
    }
    if (fixed_start_values).is_empty() {
        start_binding = selectValue(&start_values)?;
    } else {
        start_binding = selectFixedStartValue(fixed_start_values)?;
    }
    nominal_binding = selectValue(&nominal_values)?;
    min_binding = computeLimit(&min_values, &Ceval::evalBuiltinMax2)?;
    max_binding = computeLimit(&max_values, &Ceval::evalBuiltinMin2)?;
    representative =
        setRepresentativeAttributes(representative, start_binding, nominal_binding, min_binding, max_binding)?;
    Ok((representative, restAliases))
}

pub(crate) fn computeLimit(
    mut values: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut reduceFn: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Binding::NFBinding>> {
    type ReduceFn = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut limit: metamodelica::Ref<Binding::NFBinding>;
    let mut res: metamodelica::Ref<Expression::NFExpression>;
    if (values).is_empty() {
        limit = Binding::EMPTY_BINDING().clone();
    } else {
        res = List::reduce(values, reduceFn)?;
        limit = Binding::makeFlat(
            res,
            Variability::CONSTANT.clone(),
            Binding::Source::GENERATED.clone(),
            Binding::NO_CONFIDENCE.clone(),
        );
    }
    Ok(limit)
}

pub(crate) fn selectValue(
    mut bindings: &metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Binding::NFBinding>,
    )>,
) -> Result<metamodelica::Ref<Binding::NFBinding>> {
    let mut value: metamodelica::Ref<Binding::NFBinding> = Binding::EMPTY_BINDING().clone();
    let mut confidence: i32;
    let mut max_confidence: i32 = -1;
    let mut name_len: i32;
    let mut cur_name_len: i32 = 0;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    for mut b in &**bindings {
        (name, binding) = b.clone();
        confidence = Binding::actualConfidence(binding.clone())?;
        if max_confidence < 0 || confidence < max_confidence {
            value = binding;
            cur_name_len = ComponentRef::depth(&name);
            max_confidence = confidence;
        } else if confidence == max_confidence {
            name_len = ComponentRef::depth(&name);
            if name_len < cur_name_len {
                value = binding;
                cur_name_len = name_len;
            }
        }
    }
    Ok(value)
}

pub(crate) fn selectFixedStartValue(
    mut fixedBindings: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Binding::NFBinding>,
    )>,
) -> Result<metamodelica::Ref<Binding::NFBinding>> {
    let mut value: metamodelica::Ref<Binding::NFBinding> = Binding::EMPTY_BINDING().clone();
    let mut bindings: metamodelica::List<metamodelica::Ref<Binding::NFBinding>>;
    let mut r#str: ArcStr;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    bindings = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Binding::NFBinding>> = metamodelica::nil();
        for mut b in (fixedBindings.clone()).into_iter().cloned() {
            let __x = Util::tuple22(b.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if List::allEqual(
        &bindings,
        &move |__a0: metamodelica::Ref<Binding::NFBinding>, __a1: metamodelica::Ref<Binding::NFBinding>| {
            Binding::isEqual(&__a0, &__a1)
        },
    )? {
        value = (bindings).head().cloned()?;
    } else {
        if Flags::isSet(Flags::ALIAS_CONFLICTS.clone())? {
            r#str = literal!("Conflicting start values for fixed states:\n");
            for mut b in &*fixedBindings {
                (cref, binding) = b.clone();
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!(" * Candidate: "));
                    __mm_s.push_str(&*ComponentRef::toString(&cref)?);
                    __mm_s.push_str(&*literal!("(start = "));
                    __mm_s.push_str(&*Binding::toString(&binding, &(literal!("")))?);
                    __mm_s.push_str(&*literal!(", confidence number = "));
                    __mm_s.push_str(&*ArcStr::from(::std::format!(
                        "{}",
                        Binding::actualConfidence(binding)?
                    )));
                    __mm_s.push_str(&*literal!(")\n"));
                    ArcStr::from(__mm_s)
                };
            }
            Error::addCompilerError(r#str)?;
        } else {
            Error::addMessage(Error::CONFLICTING_ALIAS_SET.clone(), metamodelica::nil())?;
        }
        return Err("fail");
    }
    Ok(value)
}

pub(crate) fn defineAlias(
    mut alias: metamodelica::Ref<FlowAlias::FlowAlias>,
    mut binding: metamodelica::Ref<Binding::NFBinding>,
    mut negated: bool,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<(
    metamodelica::Ref<FlowAlias::FlowAlias>,
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
)> {
    let mut alias: metamodelica::Ref<FlowAlias::FlowAlias> = alias;
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut var_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut bind_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut bind_eq: metamodelica::Ref<Equation::NFEquation>;
    let mut b: metamodelica::Ref<Binding::NFBinding>;
    let __pa0 = ::match_deref::match_deref! { match &(alias.variable.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    var = metamodelica::Own::own(__pa0);
    if Binding::isBound(&var.binding) {
        var_exp = Expression::fromTypedCref(var.name.clone(), var.ty.clone());
        bind_exp = Binding::getExp(&var.binding)?;
        bind_eq = Equation::makeEquality(
            var_exp,
            bind_exp,
            var.ty.clone(),
            DAE::emptyElementSource().clone(),
            crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
            Equation::ScalarizeMode::NO_PREFERENCE.clone(),
        );
        equations = metamodelica::cons(bind_eq, equations);
    }
    if negated {
        b = Binding::mapExpShallow(
            binding,
            &fnptr!(Expression::negate, metamodelica::Ref<Expression::NFExpression>),
        )?;
    } else {
        b = binding;
    }
    assign_field!(
        var.binding = b,
        var.comment = metamodelica::Ref::new(SCode::Comment {
            annotation_: var.comment.annotation_.clone(),
            comment: Some(literal!("Alias variable"))
        })
    );
    assign_field!(alias.variable = Some(var));
    Ok((alias, equations))
}

pub(crate) fn evalAliasAttributes(
    mut alias: metamodelica::Ref<FlowAlias::FlowAlias>,
    mut negated: bool,
    mut startValues: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Binding::NFBinding>,
    )>,
    mut fixedStartValues: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Binding::NFBinding>,
    )>,
    mut nominalValues: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Binding::NFBinding>,
    )>,
    mut minValues: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut maxValues: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(
    metamodelica::Ref<FlowAlias::FlowAlias>,
    metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Binding::NFBinding>,
    )>,
    metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Binding::NFBinding>,
    )>,
    metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Binding::NFBinding>,
    )>,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
)> {
    let mut alias: metamodelica::Ref<FlowAlias::FlowAlias> = alias;
    let mut startValues: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Binding::NFBinding>,
    )> = startValues;
    let mut fixedStartValues: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Binding::NFBinding>,
    )> = fixedStartValues;
    let mut nominalValues: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Binding::NFBinding>,
    )> = nominalValues;
    let mut minValues: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = minValues;
    let mut maxValues: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = maxValues;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut attrs: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>;
    let mut accum_attrs: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> = metamodelica::nil();
    let mut attr_name: ArcStr;
    let mut attr_binding: metamodelica::Ref<Binding::NFBinding>;
    let mut attr_exp: metamodelica::Ref<Expression::NFExpression>;
    let __pa0 = ::match_deref::match_deref! { match &(alias.variable.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    var = metamodelica::Own::own(__pa0);
    attrs = var.typeAttributes.clone();
    for mut attr in &*attrs {
        let mut attr = attr.clone();
        (attr_name, attr_binding) = attr.clone();
        if Binding::hasExp(&attr_binding) {
            attr = (::match_deref::match_deref! { match &(attr_name.clone()) {
                Deref @ "start" => {
                    (attr_binding, _) = evalAliasAttribute(attr_binding)?;
                    if negated {
                        attr_binding = Binding::mapExpShallow(attr_binding, &fnptr!(Expression::negate, metamodelica::Ref<Expression::NFExpression>))?;
                    }
                    if Variable::isFixed(&var)? {
                        fixedStartValues = metamodelica::cons((var.name.clone(), attr_binding.clone()), fixedStartValues);
                    } else {
                        startValues = metamodelica::cons((var.name.clone(), attr_binding.clone()), startValues);
                    }
                    (attr_name, attr_binding)
                },
                Deref @ "nominal" => {
                    (attr_binding, _) = evalAliasAttribute(attr_binding)?;
                    if negated {
                        attr_binding = Binding::mapExpShallow(attr_binding, &fnptr!(Expression::negate, metamodelica::Ref<Expression::NFExpression>))?;
                    }
                    nominalValues = metamodelica::cons((var.name.clone(), attr_binding.clone()), nominalValues);
                    (attr_name, attr_binding)
                },
                Deref @ "min" => {
                    (attr_binding, attr_exp) = evalAliasAttribute(attr_binding)?;
                    if negated {
                        maxValues = metamodelica::cons(Expression::negate(attr_exp), maxValues);
                    } else {
                        minValues = metamodelica::cons(attr_exp, minValues);
                    }
                    (attr_name, attr_binding)
                },
                Deref @ "max" => {
                    (attr_binding, attr_exp) = evalAliasAttribute(attr_binding)?;
                    if negated {
                        minValues = metamodelica::cons(Expression::negate(attr_exp), minValues);
                    } else {
                        maxValues = metamodelica::cons(attr_exp, maxValues);
                    }
                    (attr_name, attr_binding)
                },
                _ => attr,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        accum_attrs = metamodelica::cons(attr, accum_attrs);
    }
    accum_attrs = metamodelica::Dangerous::listReverseInPlace(accum_attrs);
    assign_field!(var.typeAttributes = accum_attrs);
    assign_field!(alias.variable = Some(var));
    Ok((
        alias,
        startValues,
        fixedStartValues,
        nominalValues,
        minValues,
        maxValues,
    ))
}

pub(crate) fn evalAliasAttribute(
    mut binding: metamodelica::Ref<Binding::NFBinding>,
) -> Result<(
    metamodelica::Ref<Binding::NFBinding>,
    metamodelica::Ref<Expression::NFExpression>,
)> {
    let mut binding: metamodelica::Ref<Binding::NFBinding> = binding;
    let mut bindingExp: metamodelica::Ref<Expression::NFExpression>;
    bindingExp = Binding::getExp(&binding)?;
    Structural::markExp(&bindingExp)?;
    bindingExp = Ceval::evalExp(bindingExp, &(Ceval::noTarget().clone()))?;
    binding = Binding::setExp(bindingExp.clone(), binding)?;
    Ok((binding, bindingExp))
}

pub(crate) fn setRepresentativeAttributes(
    mut alias: metamodelica::Ref<FlowAlias::FlowAlias>,
    mut startValue: metamodelica::Ref<Binding::NFBinding>,
    mut nominalValue: metamodelica::Ref<Binding::NFBinding>,
    mut minValue: metamodelica::Ref<Binding::NFBinding>,
    mut maxValue: metamodelica::Ref<Binding::NFBinding>,
) -> Result<metamodelica::Ref<FlowAlias::FlowAlias>> {
    fn add_attribute(
        mut name: ArcStr,
        mut binding: metamodelica::Ref<Binding::NFBinding>,
        mut attrs: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
    ) -> metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> {
        let mut attrs: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> = attrs;
        if Binding::isBound(&binding) {
            attrs = metamodelica::cons((name, binding), attrs);
        }
        attrs
    }

    let mut alias: metamodelica::Ref<FlowAlias::FlowAlias> = alias;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut attrs: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(alias.variable.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    var = metamodelica::Own::own(__pa0);
    attrs = ({
        let mut __acc: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> = metamodelica::nil();
        for mut attr in (var.typeAttributes.clone()).into_iter().cloned() {
            if !(!(listMember(
                Util::tuple21(attr.clone()),
                list![literal!("start"), literal!("nominal"), literal!("min"), literal!("max")],
            ))) {
                continue;
            }
            let __x = attr.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    attrs = add_attribute(literal!("max"), maxValue, attrs);
    attrs = add_attribute(literal!("min"), minValue, attrs);
    attrs = add_attribute(literal!("nominal"), nominalValue, attrs);
    attrs = add_attribute(literal!("start"), startValue, attrs);
    assign_field!(var.typeAttributes = attrs);
    assign_field!(alias.variable = Some(var));
    Ok(alias)
}

pub(crate) fn findConstantBindings(
    mut flatModel: &metamodelica::Ref<FlatModel::NFFlatModel>,
    mut vars: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
) -> Result<()> {
    for mut eq in &*flatModel.equations.clone() {
        findConstantBindingsInEq(metamodelica::AsArg::as_arg(&eq), vars.clone())?;
    }
    Ok(())
}

pub(crate) fn findConstantBindingsInEq(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut vars: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
) -> Result<()> {
    fn update_binding(
        mut var: Option<metamodelica::Ref<Variable::NFVariable>>,
        mut value: metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Variable::NFVariable>> {
        let mut outVar: metamodelica::Ref<Variable::NFVariable>;
        let __pa0 = ::match_deref::match_deref! { match &(var) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        outVar = metamodelica::Own::own(__pa0);
        if Variable::isFlow(&outVar) {
            assign_field!(
                outVar.binding = Binding::makeFlat(
                    value,
                    Variability::CONSTANT.clone(),
                    Binding::Source::GENERATED.clone(),
                    Binding::NO_CONFIDENCE.clone()
                )
            );
        }
        Ok(outVar)
    }

    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let () = (::match_deref::match_deref! { match eq {
        Deref @ Equation::EQUALITY { lhs: Deref @ Expression::CREF { cref, .. }, rhs: exp, .. } if (ComponentRef::isFlow(metamodelica::AsArg::as_arg(&cref))? && Expression::variability(exp.clone())? <= Variability::STRUCTURAL_PARAMETER.clone()) => {
            let mut exp = (*exp).clone();
            Structural::markExp(metamodelica::AsArg::as_arg(&exp))?;
            exp = Ceval::tryEvalExp(exp.clone(), &(Ceval::noTarget().clone()));
            UnorderedMap::tryAddUpdate(cref.clone(), &({ let __pe_b1 = exp.clone(); move |__pe_a0| update_binding(__pe_a0, __pe_b1.clone()) }), vars)?;
            ()
        },
        Deref @ Equation::EQUALITY { lhs: exp, rhs: Deref @ Expression::CREF { cref, .. }, .. } if (ComponentRef::isFlow(metamodelica::AsArg::as_arg(&cref))? && Expression::variability(exp.clone())? <= Variability::STRUCTURAL_PARAMETER.clone()) => {
            let mut exp = (*exp).clone();
            Structural::markExp(metamodelica::AsArg::as_arg(&exp))?;
            exp = Ceval::tryEvalExp(exp.clone(), &(Ceval::noTarget().clone()));
            UnorderedMap::tryAddUpdate(cref.clone(), &({ let __pe_b1 = exp.clone(); move |__pe_a0| update_binding(__pe_a0, __pe_b1.clone()) }), vars)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub type Entry = metamodelica::Ref<FlowAlias::FlowAlias>;

pub type IndexTable = metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<FlowAlias::FlowAlias>, i32>>;

/// This is a disjoint sets data structure. The nodes are stored in an array of
///   Integers. The root elements of a set is given a negative value that
///   corresponds to its rank, while other elements are given positive values that
///   corresponds to the index of their parent in the array. The hashtable is used
///   to look up the array index of a entry, and is also used to store the entries.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Sets {
    /// An array of nodes
    pub nodes: metamodelica::Array<i32>,
    /// An Entry->Integer table.
    pub elements: IndexTable,
    /// The number of nodes stored in the sets.
    pub nodeCount: i32,
}

impl metamodelica::gc::MMTrace for Sets {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.nodes, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.elements, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nodeCount, __mmv)?;
        Ok(())
    }
}
impl Default for Sets {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            elements: Default::default(),
            nodeCount: Default::default(),
        }
    }
}

pub type DISJOINT_SETS = Sets;

pub(crate) fn add(mut entry: Entry, mut sets: Sets) -> Result<(Sets, i32)> {
    let mut sets: Sets = sets;
    let mut index: i32;
    let mut nodes: metamodelica::Array<i32>;
    let mut elements: IndexTable;
    let mut node_count: i32;
    let Sets {
        nodes: __pa0,
        elements: __pa1,
        nodeCount: __pa2,
    } = sets;
    nodes = metamodelica::Own::own(__pa0);
    elements = metamodelica::Own::own(__pa1);
    node_count = metamodelica::Own::own(__pa2);
    index = node_count + 1;
    if index > metamodelica::arrayLength(nodes.clone()) {
        nodes = Array::expand(
            ((intReal(index) * metamodelica::OrderedFloat(1.4_f64)).0.floor() as i32),
            nodes.clone(),
            -1,
        )?;
    }
    UnorderedMap::addNew(entry, index, elements.clone())?;
    sets = Sets {
        nodes: nodes.clone(),
        elements: elements,
        nodeCount: index,
    };
    Ok((sets, index))
}

pub(crate) fn addList(
    mut entries: &metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>,
    mut sets: Sets,
) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut nodes: metamodelica::Array<i32>;
    let mut elements: IndexTable;
    let mut node_count: i32;
    let mut sz: i32;
    let mut index: i32;
    let Sets {
        nodes: __pa0,
        elements: __pa1,
        nodeCount: __pa2,
    } = sets;
    nodes = metamodelica::Own::own(__pa0);
    elements = metamodelica::Own::own(__pa1);
    node_count = metamodelica::Own::own(__pa2);
    sz = ((entries).len() as i32);
    index = node_count + 1;
    node_count = node_count + sz;
    if node_count > metamodelica::arrayLength(nodes.clone()) {
        nodes = Array::expand(
            ((intReal(node_count) * metamodelica::OrderedFloat(1.4_f64)).0.floor() as i32),
            nodes.clone(),
            -1,
        )?;
    }
    for mut e in &**entries {
        UnorderedMap::addNew(e.clone(), index, elements.clone())?;
        index = index + 1;
    }
    sets = Sets {
        nodes: nodes.clone(),
        elements: elements,
        nodeCount: node_count,
    };
    Ok(sets)
}

pub(crate) fn contains(mut entry: Entry, mut sets: &Sets) -> Result<bool> {
    let mut found: bool;
    found = (UnorderedMap::get(entry, sets.elements.clone())?).is_some();
    Ok(found)
}

pub(crate) fn emptySets(mut setCount: i32) -> Sets {
    let mut sets: Sets;
    let mut nodes: metamodelica::Array<i32>;
    let mut elements: IndexTable;
    let mut sz: i32;
    sz = std::cmp::max(setCount, 3);
    nodes = arrayCreate(sz, -1);
    elements = UnorderedMap::new(
        (std::sync::Arc::new(EntryHash)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<FlowAlias::FlowAlias>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(EntryEqual)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<FlowAlias::FlowAlias>,
                        metamodelica::Ref<FlowAlias::FlowAlias>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    sets = Sets {
        nodes: nodes.clone(),
        elements: elements,
        nodeCount: 0,
    };
    sets
}

pub(crate) fn extractSets(
    mut sets: &Sets,
) -> Result<(
    metamodelica::Array<metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>>,
    Sets,
)> {
    let mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<FlowAlias::FlowAlias>>>;
    let mut assignedSets: Sets;
    let mut nodes: metamodelica::Array<i32>;
    let mut set_idx: i32 = 0;
    let mut idx: i32;
    let mut entries: metamodelica::Array<(metamodelica::Ref<FlowAlias::FlowAlias>, i32)>;
    let mut e: Entry;
    nodes = sets.nodes.clone();
    for mut i in 1..=sets.nodeCount.clone() {
        if ({
            let __elt = (*metamodelica::index_checked(&nodes.borrow(), i)?).clone();
            __elt
        }) < 0
        {
            set_idx = set_idx + 1;
            {
                let __cell0 = -(set_idx);
                let __idx0 = i;
                *metamodelica::index_mut_checked(&mut nodes.clone().borrow_mut(), __idx0)? = __cell0;
            }
        }
    }
    setsArray = arrayCreate(set_idx, metamodelica::nil());
    entries = UnorderedMap::toArray(sets.elements.clone());
    for mut i in ({
        let __s = metamodelica::arrayLength(entries.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        (e, idx) = metamodelica::Dangerous::arrayGetNoBoundsChecking(entries.clone(), i);
        set_idx = ({
            let __elt = (*metamodelica::index_checked(&nodes.borrow(), idx)?).clone();
            __elt
        });
        while set_idx > 0 {
            set_idx = ({
                let __elt = (*metamodelica::index_checked(&nodes.borrow(), set_idx)?).clone();
                __elt
            });
        }
        set_idx = -(set_idx);
        {
            let __cell1 = metamodelica::cons(
                e,
                ({
                    let __elt = (*metamodelica::index_checked(&setsArray.borrow(), set_idx)?).clone();
                    __elt
                }),
            );
            let __idx1 = set_idx;
            *metamodelica::index_mut_checked(&mut setsArray.clone().borrow_mut(), __idx1)? = __cell1;
        }
    }
    assignedSets = Sets {
        nodes: nodes.clone(),
        elements: sets.elements.clone(),
        nodeCount: sets.nodeCount.clone(),
    };
    Ok((setsArray, assignedSets))
}

pub(crate) fn find(mut entry: Entry, mut sets: Sets) -> Result<(Sets, i32)> {
    let mut sets: Sets = sets;
    let mut index: i32;
    let mut oindex: Option<i32>;
    oindex = UnorderedMap::get(entry.clone(), sets.elements.clone())?;
    if (oindex).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(oindex) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        index = metamodelica::Own::own(__pa0);
    } else {
        (sets, index) = add(entry, sets)?;
    }
    Ok((sets, index))
}

pub(crate) fn findRoot(mut nodeIndex: i32, mut nodes: metamodelica::Array<i32>) -> Result<i32> {
    let mut rootIndex: i32 = nodeIndex;
    let mut parent: i32 = ({
        let __elt = (*metamodelica::index_checked(&nodes.borrow(), nodeIndex)?).clone();
        __elt
    });
    let mut idx: i32 = nodeIndex;
    while parent > 0 {
        rootIndex = parent;
        parent = ({
            let __elt = (*metamodelica::index_checked(&nodes.borrow(), parent)?).clone();
            __elt
        });
    }
    parent = ({
        let __elt = (*metamodelica::index_checked(&nodes.borrow(), nodeIndex)?).clone();
        __elt
    });
    while parent > 0 {
        metamodelica::arrayUpdate(nodes.clone(), idx, rootIndex)?;
        idx = parent;
        parent = ({
            let __elt = (*metamodelica::index_checked(&nodes.borrow(), parent)?).clone();
            __elt
        });
    }
    Ok(rootIndex)
}

pub(crate) fn findSet(mut entry: Entry, mut sets: Sets) -> Result<(i32, Sets)> {
    let mut set: i32;
    let mut updatedSets: Sets;
    let mut index: i32;
    (updatedSets, index) = find(entry, sets)?;
    set = findRoot(index, updatedSets.nodes.clone())?;
    Ok((set, updatedSets))
}

pub(crate) fn findSetArrayIndex(mut entry: Entry, mut sets: &Sets) -> Result<i32> {
    let mut set: i32;
    set = UnorderedMap::getOrFail(entry, sets.elements.clone())?;
    while set > 0 {
        set = ({
            let __elt = (*metamodelica::index_checked(&sets.nodes.borrow(), set)?).clone();
            __elt
        });
    }
    set = -(set);
    Ok(set)
}

pub(crate) fn getEntry(mut entry: Entry, mut sets: &Sets) -> Result<Option<metamodelica::Ref<FlowAlias::FlowAlias>>> {
    let mut outEntry: Option<metamodelica::Ref<FlowAlias::FlowAlias>>;
    outEntry = UnorderedMap::getKey(entry, sets.elements.clone())?;
    Ok(outEntry)
}

pub(crate) fn getNodeCount(mut sets: &Sets) -> i32 {
    let mut nodeCount: i32 = sets.nodeCount.clone();
    nodeCount
}

pub(crate) fn merge(mut entry1: Entry, mut entry2: Entry, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut set1: i32;
    let mut set2: i32;
    (set1, sets) = findSet(entry1, sets)?;
    (set2, sets) = findSet(entry2, sets)?;
    sets = union(set1, set2, sets)?;
    Ok(sets)
}

pub(crate) fn printSets(mut sets: &Sets) -> Result<()> {
    let mut nodes: metamodelica::Array<i32>;
    let mut entries: metamodelica::List<(metamodelica::Ref<FlowAlias::FlowAlias>, i32)>;
    let mut e: Entry;
    let mut i: i32;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(sets.nodeCount.clone()));
        __mm_s.push_str(&*literal!(" sets:\n"));
        ArcStr::from(__mm_s)
    });
    nodes = sets.nodes.clone();
    entries = UnorderedMap::toList(sets.elements.clone());
    for mut p in &*entries {
        (e, i) = p.clone();
        metamodelica::print(literal!("["));
        metamodelica::print(ArcStr::from(::std::format!("{}", i)));
        metamodelica::print(literal!("]"));
        metamodelica::print(EntryString(e)?);
        metamodelica::print(literal!(" -> "));
        metamodelica::print(ArcStr::from(::std::format!(
            "{}",
            ({
                let __elt = (*metamodelica::index_checked(&nodes.borrow(), i)?).clone();
                __elt
            })
        )));
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

pub(crate) fn union(mut set1: i32, mut set2: i32, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut rank1: i32;
    let mut rank2: i32;
    if set1 != set2 {
        rank1 = ({
            let __elt = (*metamodelica::index_checked(&sets.nodes.borrow(), set1)?).clone();
            __elt
        });
        rank2 = ({
            let __elt = (*metamodelica::index_checked(&sets.nodes.borrow(), set2)?).clone();
            __elt
        });
        if rank1 > rank2 {
            metamodelica::arrayUpdate(sets.nodes.clone(), set2, set1)?;
        } else if rank1 < rank2 {
            metamodelica::arrayUpdate(sets.nodes.clone(), set1, set2)?;
        } else {
            metamodelica::arrayUpdate(
                sets.nodes.clone(),
                set1,
                ({
                    let __elt = (*metamodelica::index_checked(&sets.nodes.borrow(), set1)?).clone();
                    __elt
                }) - 1,
            )?;
            metamodelica::arrayUpdate(sets.nodes.clone(), set2, set1)?;
        }
    }
    Ok(sets)
}
