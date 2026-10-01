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

use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBInline as Inline;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBPartitioning as Partitioning;
use crate::NBPartitioning::BClock;
use crate::NBSlice as Slice;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// Backend imports
// Util imports
pub(crate) fn main(
    mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE>,
    mut kind: Partition::Kind,
) -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>> {
    let mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE> = bdae;
    let mut func: Module::aliasInterface;
    func = getModule()?;
    bdae = (match &*bdae {
        BackendDAE::MAIN { varData, eqData, .. } => {
            let mut varData = (*varData).clone();
            let mut eqData = (*eqData).clone();
            (varData, eqData) = func(varData.clone(), eqData.clone(), kind)?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                varData = varData.clone(),
                eqData = eqData.clone()
            );
            bdae
        }
        BackendDAE::HESSIAN { varData, eqData } => {
            let mut varData = (*varData).clone();
            let mut eqData = (*eqData).clone();
            (varData, eqData) = func(varData.clone(), eqData.clone(), kind)?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::HESSIAN;
                varData = varData.clone(),
                eqData = eqData.clone()
            );
            bdae
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBFunctionAlias.main"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(bdae)
}

pub(crate) fn getModule() -> Result<
    Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<VarData::VarData>,
                metamodelica::Ref<EqData::EqData>,
                Partition::Kind,
            )
                -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>)>
            + 'static,
    >,
> {
    let mut func: Module::functionAliasInterface;
    let mut flag: ArcStr = literal!("default");
    func = (::match_deref::match_deref! { match &(flag) {
        Deref @ "default" => (std::sync::Arc::new(functionAliasDefault) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>, Partition::Kind) -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>)> + 'static>),
        _ => return Err("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

pub(crate) fn introduceSlicedStateAlias(
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut kind: Partition::Kind,
) -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>)> {
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut aux_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Call_Id::Call_Id>, metamodelica::Ref<Call_Aux::Call_Aux>>,
    > = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<Call_Id::Call_Id>| Call_Id::hash(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Call_Id::Call_Id>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Call_Id::Call_Id>, __a1: metamodelica::Ref<Call_Id::Call_Id>| {
                Call_Id::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Call_Id::Call_Id>,
                        metamodelica::Ref<Call_Id::Call_Id>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    let mut new_vars_cont: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut new_vars_recd: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut new_eqns_cont: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let () = ({
        let mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>,
            >,
        > = UnorderedMap::new(
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
        let mut aux_index: Pointer::Pointer<i32> = Pointer::create(1);
        (::match_deref::match_deref! { match &((eqData.clone(), varData.clone())) {
            (Deref @ BEquation::EqData::EQ_DATA_SIM { .. }, Deref @ BVariable::VarData::VAR_DATA_SIM { .. }) => {
                let mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                BEquation::EquationPointers::map(var_field!((*eqData).simulation, EqData::EqData::EQ_DATA_SIM).clone(), &({ let __pe_b1 = map.clone(); move |__pe_a0| collectSlicedStatesAliasEquation(__pe_a0, __pe_b1.clone()) }))?;
                set = getSlicedStatesSet(map)?;
                if !(UnorderedSet::isEmpty(set.clone())) {
                    assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; simulation = BEquation::EquationPointers::map(var_field!((*eqData).simulation, EqData::EqData::EQ_DATA_SIM).clone(), &({ let __pe_b1 = set; let __pe_b2 = aux_map.clone(); let __pe_b3 = aux_index; move |__pe_a0| introduceSlicedStateAliasEquation(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?);
                    (_, new_vars_cont, _, new_vars_recd, _, new_eqns_cont, _) = resolveAux(aux_map.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), false, metamodelica::nil(), new_vars_cont, metamodelica::nil(), new_vars_recd, metamodelica::nil(), new_eqns_cont, metamodelica::nil())?;
                }
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    varData = BVariable::VarData::addTypedList(varData, &new_vars_cont, VarData::VarType::ALGEBRAIC.clone())?;
    varData = BVariable::VarData::addTypedList(varData, &new_vars_recd, VarData::VarType::RECORD.clone())?;
    eqData = BEquation::EqData::addTypedList(eqData, &new_eqns_cont, EqData::EqType::CONTINUOUS.clone(), false)?;
    for mut var in &*new_vars_recd {
        BackendDAE::lowerRecordChildren(var.clone(), &(BVariable::VarData::getVariables(&varData)?))?;
    }
    if Flags::isSet(Flags::DUMP_CSE.clone())? {
        metamodelica::print(aliasListToString(
            UnorderedMap::toList(aux_map),
            &move |__a0: metamodelica::Ref<Call_Id::Call_Id>| Call_Id::toString(&__a0),
            &move |__a0: metamodelica::Ref<Call_Aux::Call_Aux>| Call_Aux::toString(&__a0),
            &(literal!("Sliced State")),
        )?);
    }
    Ok((varData, eqData))
}

pub mod Call_Id {
    use super::*;
    /// key for UnorderedMap.
    ///    used to uniquely identify a function call
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Call_Id {
        pub call: metamodelica::Ref<Expression::NFExpression>,
        pub iter: metamodelica::Ref<Iterator::Iterator>,
    }

    impl metamodelica::gc::MMTrace for Call_Id {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.call, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.iter, __mmv)?;
            Ok(())
        }
    }
    impl Default for Call_Id {
        fn default() -> Self {
            Self {
                call: Default::default(),
                iter: Default::default(),
            }
        }
    }

    pub type CALL_ID = Call_Id;

    pub(crate) fn toString(mut id: &metamodelica::Ref<Call_Id>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = if (!(BEquation::Iterator::isEmpty(&id.iter))) {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" ["));
                __mm_s.push_str(&*BEquation::Iterator::toString(&id.iter)?);
                __mm_s.push_str(&*literal!("]"));
                ArcStr::from(__mm_s)
            }
        } else {
            literal!("")
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Expression::toString(id.call.clone())?);
            __mm_s.push_str(&*r#str);
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn hash(mut id: &metamodelica::Ref<Call_Id>) -> Result<i32> {
        let mut hash: i32;
        hash = stringHashDjb2(&(toString(id)?));
        Ok(hash)
    }

    pub(crate) fn isEqual(mut id1: &metamodelica::Ref<Call_Id>, mut id2: &metamodelica::Ref<Call_Id>) -> Result<bool> {
        let mut b: bool;
        b = Expression::isEqual(id1.call.clone(), id2.call.clone())?
            && BEquation::Iterator::isEqual(&id1.iter, &id2.iter)?;
        Ok(b)
    }
}

pub mod Call_Aux {
    use super::*;
    /// value for UnorderedMap.
    ///    represents the auxilliary variable that will be created and has
    ///    the equation kind for auxilliary equation.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Call_Aux {
        pub replacer: metamodelica::Ref<Expression::NFExpression>,
        pub kind: EquationKind,
        pub parsed: bool,
    }

    impl metamodelica::gc::MMTrace for Call_Aux {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.replacer, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.kind, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.parsed, __mmv)?;
            Ok(())
        }
    }
    impl Default for Call_Aux {
        fn default() -> Self {
            Self {
                replacer: Default::default(),
                kind: Default::default(),
                parsed: Default::default(),
            }
        }
    }

    pub type CALL_AUX = Call_Aux;

    pub(crate) fn toString(mut aux: &metamodelica::Ref<Call_Aux>) -> Result<ArcStr> {
        let mut r#str: ArcStr = Expression::toString(aux.replacer.clone())?;
        Ok(r#str)
    }

    pub(crate) fn getVars(
        mut aux: &metamodelica::Ref<Call_Aux>,
    ) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
        fn getVarsExp(
            mut exp: &metamodelica::Ref<Expression::NFExpression>,
        ) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
            let mut vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
            vars = (::match_deref::match_deref! { match exp {
                Deref @ Expression::CREF { cref: Deref @ ComponentRef::WILD, .. } => metamodelica::nil(),
                Deref @ Expression::CREF { cref: __exp_cref, .. } => list![BVariable::getVarPointer(metamodelica::AsArg::as_arg(&__exp_cref), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBFunctionAlias.mo"))?],
                Deref @ Expression::TUPLE { elements: __exp_elements, .. } => List::flatten(({
                let mut __acc: metamodelica::List<_> = metamodelica::nil();
                for mut elem in (__exp_elements.clone()).into_iter().cloned() {
                    let __x = getVarsExp(&(elem.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }))?,
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBFunctionAlias.Call_Aux.getVars.getVarsExp")); __mm_s.push_str(&*literal!(" failed because function alias auxilliary has a return type that currently cannot be parsed: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }])?;
                    return Err("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            Ok(vars)
        }

        let mut vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            getVarsExp(&aux.replacer)?;
        Ok(vars)
    }

    pub(crate) fn createName(
        mut ty: metamodelica::Ref<Type::NFType>,
        mut iter: &metamodelica::Ref<Iterator::Iterator>,
        mut aux_index: Pointer::Pointer<i32>,
        mut aux_name: &ArcStr,
        mut init: bool,
    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
        let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut new_ty: metamodelica::Ref<Type::NFType> = ty.clone();
        let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
        if !(BEquation::Iterator::isEmpty(iter)) {
            new_ty = Type::liftArrayRightList(ty, &(BEquation::Iterator::dimensions(iter)?));
            (_, name) = BVariable::makeAuxVar(aux_name, Pointer::access(aux_index.clone()), new_ty.clone(), init)?;
            subs = BEquation::Iterator::normalizedSubscripts(
                iter,
                UnorderedMap::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
                ),
            )?;
            subs = Subscript::fillWithWholeLeft(subs, Type::dimensionCount(new_ty));
            name = ComponentRef::mergeSubscripts(subs, name, true, true, false)?;
        } else {
            (_, name) = BVariable::makeAuxVar(aux_name, Pointer::access(aux_index.clone()), new_ty, init)?;
        }
        Pointer::update(aux_index.clone(), Pointer::access(aux_index) + 1);
        Ok(name)
    }
}

fn functionAliasTplString(mut tpl: (ArcStr, ArcStr), mut max_length: i32) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Util::tuple21(tpl.clone()));
        __mm_s.push_str(&*literal!(" "));
        __mm_s.push_str(&*StringUtil::repeat(
            literal!("."),
            max_length - ((Util::tuple21(tpl.clone())).len() as i32),
        )?);
        __mm_s.push_str(&*literal!(" "));
        __mm_s.push_str(&*Util::tuple22(tpl));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

fn functionAliasDefault(
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut kind: Partition::Kind,
) -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>)> {
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Call_Id::Call_Id>, metamodelica::Ref<Call_Aux::Call_Aux>>,
    > = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<Call_Id::Call_Id>| Call_Id::hash(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Call_Id::Call_Id>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Call_Id::Call_Id>, __a1: metamodelica::Ref<Call_Id::Call_Id>| {
                Call_Id::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Call_Id::Call_Id>,
                        metamodelica::Ref<Call_Id::Call_Id>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers> =
        BVariable::VarData::getVariables(&varData)?;
    let mut set: metamodelica::Ref<
        UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = UnorderedSet::new(
        (std::sync::Arc::new(BVariable::hash)
            as std::sync::Arc<
                dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(BVariable::equalName)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                    ) -> Result<bool>
                    + 'static,
            >),
        13,
    );
    let mut clock_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<BClock::BClock>, metamodelica::Ref<ComponentRef::NFComponentRef>>,
    > = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<BClock::BClock>| Partitioning::BClock::hash(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BClock::BClock>, __a1: metamodelica::Ref<BClock::BClock>| {
                Partitioning::BClock::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>, metamodelica::Ref<BClock::BClock>) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    let mut infer_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<BClock::BClock>, metamodelica::Ref<ComponentRef::NFComponentRef>>,
    > = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<BClock::BClock>| Partitioning::BClock::hash(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BClock::BClock>, __a1: metamodelica::Ref<BClock::BClock>| {
                Partitioning::BClock::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>, metamodelica::Ref<BClock::BClock>) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    let mut aux_index: Pointer::Pointer<i32> = Pointer::create(1);
    let mut new_vars_disc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut new_vars_cont: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut new_vars_init: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut new_vars_recd: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut new_vars_clck: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut new_vars_infr: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut new_eqns_disc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut new_eqns_cont: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut new_eqns_init: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut new_eqns_clck: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut new_eqns_infr: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut debug_lst_sim: metamodelica::List<(
        metamodelica::Ref<Call_Id::Call_Id>,
        metamodelica::Ref<Call_Aux::Call_Aux>,
    )> = metamodelica::nil();
    let mut debug_lst_ini: metamodelica::List<(
        metamodelica::Ref<Call_Id::Call_Id>,
        metamodelica::Ref<Call_Aux::Call_Aux>,
    )>;
    let mut sim_aliases: metamodelica::List<(
        metamodelica::Ref<Call_Id::Call_Id>,
        metamodelica::Ref<Call_Aux::Call_Aux>,
    )> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &((eqData.clone(), varData.clone())) {
        (Deref @ BEquation::EqData::EQ_DATA_SIM { .. }, Deref @ BVariable::VarData::VAR_DATA_SIM { .. }) => {
            assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM;
                simulation = BEquation::EquationPointers::map(var_field!((*eqData).simulation, EqData::EqData::EQ_DATA_SIM).clone(), &({ let __pe_b1 = map.clone(); let __pe_b2 = variables.clone(); let __pe_b3 = set.clone(); let __pe_b4 = aux_index.clone(); let __pe_b5 = var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(); let __pe_b6 = false; move |__pe_a0| introduceFunctionAliasEquation(__pe_a0, __pe_b1.clone(), &__pe_b2, __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }))?,
                removed = BEquation::EquationPointers::map(var_field!((*eqData).removed, EqData::EqData::EQ_DATA_SIM).clone(), &({ let __pe_b1 = map.clone(); let __pe_b2 = variables.clone(); let __pe_b3 = set.clone(); let __pe_b4 = aux_index.clone(); let __pe_b5 = var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(); let __pe_b6 = false; move |__pe_a0| introduceFunctionAliasEquation(__pe_a0, __pe_b1.clone(), &__pe_b2, __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }))?
            );
            (new_vars_disc, new_vars_cont, new_vars_init, new_vars_recd, new_eqns_disc, new_eqns_cont, new_eqns_init) = resolveAux(map.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), false, new_vars_disc, new_vars_cont, new_vars_init, new_vars_recd, new_eqns_disc, new_eqns_cont, new_eqns_init)?;
            sim_aliases = UnorderedMap::toList(map.clone());
            if Flags::isSet(Flags::DUMP_CSE.clone())? {
                debug_lst_sim = sim_aliases.clone();
            }
            assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; initials = BEquation::EquationPointers::map(var_field!((*eqData).initials, EqData::EqData::EQ_DATA_SIM).clone(), &({ let __pe_b1 = map.clone(); let __pe_b2 = variables; let __pe_b3 = set.clone(); let __pe_b4 = aux_index.clone(); let __pe_b5 = var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(); let __pe_b6 = true; move |__pe_a0| introduceFunctionAliasEquation(__pe_a0, __pe_b1.clone(), &__pe_b2, __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }))?);
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM; parameters = BVariable::VariablePointers::mapPtr(var_field!((*varData).parameters, VarData::VarData::VAR_DATA_SIM).clone(), &({ let __pe_b1 = (std::sync::Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = aux_index; let __pe_b3 = crate::NBEquation::Iterator::interned_EMPTY(); let __pe_b4 = true; move |__pe_a0| introduceFunctionAlias(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>); let __pe_b2 = (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>); move |__pe_a0| BVariable::mapExp(__pe_a0, __pe_b1.clone(), &*__pe_b2) }))?);
            (new_vars_disc, new_vars_cont, new_vars_init, new_vars_recd, new_eqns_disc, new_eqns_cont, new_eqns_init) = resolveAux(map.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), true, new_vars_disc, new_vars_cont, new_vars_init, new_vars_recd, new_eqns_disc, new_eqns_cont, new_eqns_init)?;
            (new_eqns_clck, new_eqns_infr, new_vars_clck, new_vars_infr, clock_map, infer_map) = addClockedAlias(var_field!((*eqData).simulation, EqData::EqData::EQ_DATA_SIM).clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    varData = BVariable::VarData::addTypedList(varData, &new_vars_cont, VarData::VarType::ALGEBRAIC.clone())?;
    varData = BVariable::VarData::addTypedList(varData, &new_vars_disc, VarData::VarType::DISCRETE.clone())?;
    varData = BVariable::VarData::addTypedList(varData, &new_vars_init, VarData::VarType::PARAMETER.clone())?;
    varData = BVariable::VarData::addTypedList(varData, &new_vars_recd, VarData::VarType::RECORD.clone())?;
    varData = BVariable::VarData::addTypedList(varData, &new_vars_clck, VarData::VarType::CLOCK.clone())?;
    varData = BVariable::VarData::addTypedList(varData, &new_vars_infr, VarData::VarType::DISCRETE.clone())?;
    varData = BVariable::VarData::addTypedList(
        varData,
        &(UnorderedSet::toList(set)),
        VarData::VarType::ITERATOR.clone(),
    )?;
    eqData = BEquation::EqData::addTypedList(eqData, &new_eqns_cont, EqData::EqType::CONTINUOUS.clone(), false)?;
    eqData = BEquation::EqData::addTypedList(eqData, &new_eqns_disc, EqData::EqType::DISCRETE.clone(), false)?;
    eqData = BEquation::EqData::addTypedList(eqData, &new_eqns_init, EqData::EqType::INITIAL.clone(), false)?;
    eqData = BEquation::EqData::addTypedList(eqData, &new_eqns_clck, EqData::EqType::CLOCKED.clone(), false)?;
    eqData = BEquation::EqData::addTypedList(eqData, &new_eqns_infr, EqData::EqType::DISCRETE.clone(), false)?;
    for mut var in &*new_vars_recd {
        BackendDAE::lowerRecordChildren(var.clone(), &(BVariable::VarData::getVariables(&varData)?))?;
    }
    for mut tpl in &*sim_aliases {
        addAuxStartValue(&(Util::tuple21(tpl.clone())), &(Util::tuple22(tpl.clone())))?;
    }
    if Flags::isSet(Flags::DUMP_CSE.clone())? {
        for mut tpl in &*debug_lst_sim {
            UnorderedMap::remove(Util::tuple21(tpl.clone()), map.clone())?;
        }
        debug_lst_ini = UnorderedMap::toList(map);
        metamodelica::print(aliasListToString(
            debug_lst_sim,
            &move |__a0: metamodelica::Ref<Call_Id::Call_Id>| Call_Id::toString(&__a0),
            &move |__a0: metamodelica::Ref<Call_Aux::Call_Aux>| Call_Aux::toString(&__a0),
            &(literal!("Simulation Function")),
        )?);
        metamodelica::print(aliasListToString(
            debug_lst_ini,
            &move |__a0: metamodelica::Ref<Call_Id::Call_Id>| Call_Id::toString(&__a0),
            &move |__a0: metamodelica::Ref<Call_Aux::Call_Aux>| Call_Aux::toString(&__a0),
            &(literal!("Initial Function")),
        )?);
        metamodelica::print(aliasListToString(
            UnorderedMap::toList(clock_map),
            &move |__a0: metamodelica::Ref<BClock::BClock>| Partitioning::BClock::toString(&__a0),
            &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
            &(literal!("Clocked Function")),
        )?);
        metamodelica::print(aliasListToString(
            UnorderedMap::toList(infer_map),
            &move |__a0: metamodelica::Ref<BClock::BClock>| Partitioning::BClock::toString(&__a0),
            &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
            &(literal!("Inferred Clocked Function")),
        )?);
    }
    Ok((varData, eqData))
}

fn addAuxStartValue(
    mut id: &metamodelica::Ref<Call_Id::Call_Id>,
    mut aux: &metamodelica::Ref<Call_Aux::Call_Aux>,
) -> Result<()> {
    if aux.kind.clone() == EquationKind::CONTINUOUS.clone()
        && (BEquation::Iterator::isEmpty(&id.iter) || hasPlainIterators(&id.iter))
    {
        let () = ({
            let mut i: i32 = 0;
            (::match_deref::match_deref! { match &((id.call.clone(), aux.replacer.clone())) {
                (call_exp @ Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { .. } }, replacer @ Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { .. }, .. }) if (isStartUsefulFunction(&(Call::typedFunction(var_field!((**call_exp).call, Expression::NFExpression::CALL))?))?) => {
                    setAuxStartValue(BVariable::getVarPointer(var_field!((**replacer).cref, Expression::NFExpression::CREF), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBFunctionAlias.mo"))?, iterateStart(id.call.clone(), &id.iter)?)?;
                    ()
                },
                (call_exp @ Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { .. } }, Deref @ Expression::TUPLE { elements, .. }) if (isStartUsefulFunction(&(Call::typedFunction(var_field!((**call_exp).call, Expression::NFExpression::CALL))?))?) => {
                    for mut elem in &*elements.clone() {
                        i = i + 1;
                        let () = (::match_deref::match_deref! { match &(elem.clone()) {
                Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } => {
                    setAuxStartValue(BVariable::getVarPointer(var_field!((**elem).cref, Expression::NFExpression::CREF), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBFunctionAlias.mo"))?, iterateStart(Expression::tupleElement(id.call.clone(), i)?, &id.iter)?)?;
                    ()
                },
                _ => (),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                    }
                    ()
                },
                _ => {
                    ()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        });
    }
    Ok(())
}

fn hasPlainIterators(mut iter: &metamodelica::Ref<Iterator::Iterator>) -> bool {
    let mut b: bool;
    let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator::Iterator>>>;
    (_, _, maps) = BEquation::Iterator::getFrames(iter);
    b = true;
    for mut map in &*maps {
        b = b && (map).is_none();
    }
    b
}

fn iterateStart(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut elements: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut n: i32;
    if !(BEquation::Iterator::isEmpty(iter)) {
        dims = Type::arrayDims(Expression::typeOf(exp.clone()));
        if (dims).is_empty() {
            exp = iterateScalarStart(exp, iter)?;
        } else if ((dims).len() as i32) == 1 && Dimension::isKnown(&((dims).head().cloned()?), false) {
            n = Dimension::size(&((dims).head().cloned()?), false)?;
            elements = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut k in (1..=n).into_iter() {
                    let __x = iterateScalarStart(
                        Expression::applySubscripts(
                            &(list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                                index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: k.clone() })
                            })]),
                            exp.clone(),
                            false,
                        )?,
                        iter,
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            exp = if (List::any(
                &elements,
                &move |__a0: metamodelica::Ref<Expression::NFExpression>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Expression::isEmpty(&__a0))
                },
            )?) {
                metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                    ty: Expression::typeOf(exp),
                })
            } else {
                Expression::makeExpArray(
                    metamodelica::arrayFromVec(elements.clone().into_iter().cloned().collect()),
                    Expression::typeOf((elements).head().cloned()?),
                    false,
                )
            };
        } else {
            exp = metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                ty: Expression::typeOf(exp),
            });
        }
    }
    Ok(exp)
}

fn iterateScalarStart(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let max_size: i32 = 1000;
    let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut values: metamodelica::List<i32>;
    let mut elements: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut size: i32 = 1;
    let mut ty: metamodelica::Ref<Type::NFType>;
    (names, ranges, _) = BEquation::Iterator::getFrames(iter);
    for mut tpl in &*List::zip(names, ranges).reverse() {
        values = rangeValues(&(Util::tuple22(tpl.clone())));
        size = size * ((values).len() as i32);
        if (values).is_empty() || size > max_size {
            exp = metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                ty: Expression::typeOf(exp),
            });
            return Ok(exp);
        }
        elements = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut v in (values.clone()).into_iter().cloned() {
                let __x = SimplifyExp::simplify(
                    Expression::replaceIterator(
                        exp.clone(),
                        &(ComponentRef::node(&(Util::tuple21(tpl.clone())))?),
                        &(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: v.clone() })),
                    )?,
                    false,
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        ty = Type::liftArrayLeft(
            Expression::typeOf(exp),
            &(Dimension::fromInteger(((values).len() as i32), NFPrefixes::Variability::CONSTANT.clone())),
        );
        exp = Expression::makeArray(
            ty,
            metamodelica::arrayFromVec(elements.into_iter().cloned().collect()),
            false,
        );
    }
    Ok(exp)
}

fn rangeValues(mut range: &metamodelica::Ref<Expression::NFExpression>) -> metamodelica::List<i32> {
    let mut values: metamodelica::List<i32> = metamodelica::nil();
    values = (::match_deref::match_deref! { match range {
        Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: start }, step: None, stop: Deref @ Expression::INTEGER { value: stop }, .. } => {
            List::intRange2(start.clone(), stop.clone())
        },
        Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: start }, step: Some(Deref @ Expression::INTEGER { value: step }), stop: Deref @ Expression::INTEGER { value: stop }, .. } if (step.clone() != 0) => {
            ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut i in (({let __s=start.clone(); let __e=stop.clone(); let __step=step.clone(); (0i32..).map(move |__k| __s + __k * __step).take_while(move |&__v| __step != 0 && (if __step > 0 { __v <= __e } else { __v >= __e }))})).into_iter() {
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    values
}

fn setAuxStartValue(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<()> {
    let mut children: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        BVariable::getRecordChildren(var_ptr.clone())?;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    if (children).is_empty() {
        var = Pointer::access(var_ptr.clone());
        if Type::isReal(&(Type::arrayElementType(&(Variable::typeOf(&var)))))?
            && !(BVariable::isRecord(var_ptr.clone()))
            && !(Expression::isEmpty(&exp))
            && (!(BVariable::isArray(var_ptr.clone()))
                || Type::isEqual(&(Variable::typeOf(&var)), &(Expression::typeOf(exp.clone())))?)
            && (BVariable::getStartAttribute(var_ptr.clone())?).is_none()
        {
            Pointer::update(var_ptr, BVariable::setStartAttribute(var, exp, false)?);
        }
    } else {
        for mut child in &*children {
            var = Pointer::access(child.clone());
            if Type::isReal(&(Variable::typeOf(&var)))?
                && !(BVariable::isArray(child.clone()))
                && !(BVariable::isRecord(child.clone()))
                && (BVariable::getStartAttribute(child.clone())?).is_none()
            {
                Pointer::update(
                    child.clone(),
                    BVariable::setStartAttribute(
                        var.clone(),
                        Expression::recordElement(&(ComponentRef::firstName(&var.name, false)?), &exp)?,
                        false,
                    )?,
                );
            }
        }
    }
    Ok(())
}

fn isStartUsefulFunction(mut r#fn: &metamodelica::Ref<Function::Function>) -> Result<bool> {
    let mut b: bool = !(Function::isBuiltin(r#fn)) && isStartSafeFunction(r#fn, 0)?;
    Ok(b)
}

fn isStartSafeFunction(mut r#fn: &metamodelica::Ref<Function::Function>, mut depth: i32) -> Result<bool> {
    let mut safe: bool;
    let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut r#unsafe: Pointer::Pointer<bool>;
    if Function::isBuiltin(r#fn) || Function::isDefaultRecordConstructor(r#fn)? {
        safe = true;
    } else if depth > 4 || Function::isExternal(r#fn)? || Function::isImpure(r#fn) {
        safe = false;
    } else {
        body = Function::getBody(r#fn)?;
        r#unsafe = Pointer::create(List::any(
            &body,
            &({
                let __pe_b1: Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<Statement::NFStatement>) -> Result<bool> + 'static,
                > = (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Statement::NFStatement>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(isUnsafeStatement(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Statement::NFStatement>) -> Result<bool> + 'static,
                    >);
                move |__pe_a0| Statement::contains(__pe_a0, &*__pe_b1)
            }),
        )?);
        if !(Pointer::access(r#unsafe.clone())) {
            Statement::applyExpList(
                &body,
                &({
                    let __pe_b1 = r#unsafe.clone();
                    let __pe_b2 = depth;
                    move |__pe_a0| markUnsafeCalls(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                }),
            )?;
        }
        safe = !(Pointer::access(r#unsafe));
    }
    Ok(safe)
}

fn isUnsafeStatement(mut stmt: &metamodelica::Ref<Statement::NFStatement>) -> bool {
    let mut b: bool;
    b = (match &**stmt {
        Statement::ASSERT { .. } => true,
        Statement::TERMINATE { .. } => true,
        _ => false,
    });
    b
}

fn markUnsafeCalls(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut r#unsafe: Pointer::Pointer<bool>,
    mut depth: i32,
) -> Result<()> {
    if !(Pointer::access(r#unsafe.clone()))
        && Expression::contains(
            exp,
            &({
                let __pe_b1 = depth;
                move |__pe_a0| isUnsafeCallExp(&__pe_a0, __pe_b1.clone())
            }),
        )?
    {
        Pointer::update(r#unsafe, true);
    }
    Ok(())
}

fn isUnsafeCallExp(mut exp: &metamodelica::Ref<Expression::NFExpression>, mut depth: i32) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match exp {
        Deref @ Expression::BINARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV, .. }, exp2: __exp_exp2, .. } => {
            !(Expression::isLiteral(metamodelica::AsArg::as_arg(&__exp_exp2))?)
        },
        Deref @ Expression::BINARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV_EW, .. }, exp2: __exp_exp2, .. } => {
            !(Expression::isLiteral(metamodelica::AsArg::as_arg(&__exp_exp2))?)
        },
        Deref @ Expression::BINARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV_SCALAR_ARRAY, .. }, exp2: __exp_exp2, .. } => {
            !(Expression::isLiteral(metamodelica::AsArg::as_arg(&__exp_exp2))?)
        },
        Deref @ Expression::BINARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV_ARRAY_SCALAR, .. }, exp2: __exp_exp2, .. } => {
            !(Expression::isLiteral(metamodelica::AsArg::as_arg(&__exp_exp2))?)
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { .. } } => {
            let mut r#fn: metamodelica::Ref<Function::Function>;
            r#fn = Call::typedFunction(var_field!((**exp).call, Expression::NFExpression::CALL))?;
            if (Function::isBuiltin(&r#fn)) {List::contains(&(list![literal!("sqrt"), literal!("log"), literal!("log10")]), AbsynUtil::pathLastIdent(&(Function::name(&r#fn))), &fnptr!(stringEq, ArcStr, ArcStr))?} else {!(isStartSafeFunction(&r#fn, depth + 1)?)}
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn aliasListToString<
    T1: Clone + 'static + metamodelica::gc::MMTrace,
    T2: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut aux_lst: metamodelica::List<(T1, T2)>,
    mut func1: &dyn ::std::ops::Fn(T1) -> Result<ArcStr>,
    mut func2: &dyn ::std::ops::Fn(T2) -> Result<ArcStr>,
    mut name: &ArcStr,
) -> Result<ArcStr> {
    type idToString<T1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1) -> Result<ArcStr> + 'static>;

    type auxToString<T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T2) -> Result<ArcStr> + 'static>;

    let mut r#str: ArcStr;
    let mut str_lst: metamodelica::List<(ArcStr, ArcStr)>;
    let mut max_length: i32;
    r#str = StringUtil::headline_3(
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!(" Alias"));
            ArcStr::from(__mm_s)
        }),
    )?;
    if (aux_lst).is_empty() {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("  <no alias>\n\n"));
            ArcStr::from(__mm_s)
        };
    } else {
        str_lst = ({
            let mut __acc: metamodelica::List<(ArcStr, ArcStr)> = metamodelica::nil();
            for mut tpl in (aux_lst).into_iter().cloned() {
                let __x = (func2(Util::tuple22(tpl.clone()))?, func1(Util::tuple21(tpl.clone()))?);
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        max_length = ({
            let mut __acc: Option<i32> = None;
            for mut tpl in (str_lst.clone()).into_iter().cloned() {
                let __x = ((Util::tuple21(tpl.clone())).len() as i32);
                __acc = Some(match __acc {
                    None => __x,
                    Some(__cur) => {
                        if __x > __cur {
                            __x
                        } else {
                            __cur
                        }
                    }
                });
            }
            __acc.unwrap_or((-i32::MAX))
        }) + 3;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*List::toStringCustom(
                str_lst,
                &({
                    let __pe_b1 = max_length;
                    move |__pe_a0| functionAliasTplString(__pe_a0, __pe_b1.clone())
                }),
                literal!(""),
                literal!("  "),
                literal!("\n  "),
                literal!("\n\n"),
                true,
                0,
            )?);
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

fn resolveAux(
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Call_Id::Call_Id>, metamodelica::Ref<Call_Aux::Call_Aux>>,
    >,
    mut eq_index: Pointer::Pointer<i32>,
    mut init: bool,
    mut new_vars_disc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut new_vars_cont: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut new_vars_init: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut new_vars_recd: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut new_eqns_disc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    mut new_eqns_cont: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    mut new_eqns_init: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
) -> Result<(
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
)> {
    let mut new_vars_disc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        new_vars_disc;
    let mut new_vars_cont: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        new_vars_cont;
    let mut new_vars_init: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        new_vars_init;
    let mut new_vars_recd: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        new_vars_recd;
    let mut new_eqns_disc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = new_eqns_disc;
    let mut new_eqns_cont: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = new_eqns_cont;
    let mut new_eqns_init: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = new_eqns_init;
    let mut id: metamodelica::Ref<Call_Id::Call_Id>;
    let mut aux: metamodelica::Ref<Call_Aux::Call_Aux>;
    let mut disc: bool;
    let mut new_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut new_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    for mut tpl in &*UnorderedMap::toList(map.clone()).reverse() {
        (id, aux) = tpl.clone();
        if !(aux.parsed.clone()) {
            new_vars = Call_Aux::getVars(&aux)?;
            disc = true;
            for mut new_var in &*new_vars {
                (disc, new_vars_disc, new_vars_cont, new_vars_init, new_vars_recd) = addAuxVar(
                    new_var.clone(),
                    disc,
                    new_vars_disc,
                    new_vars_cont,
                    new_vars_init,
                    new_vars_recd,
                    init,
                )?;
            }
            new_eqn = BEquation::Equation::makeAssignment(
                aux.replacer.clone(),
                id.call.clone(),
                eq_index.clone(),
                &(literal!("AUX")),
                id.iter.clone(),
                BEquation::default(aux.kind.clone(), init, None, None),
            )?;
            if init {
                new_eqns_init = metamodelica::cons(new_eqn, new_eqns_init);
            } else if disc {
                new_eqns_disc = metamodelica::cons(new_eqn, new_eqns_disc);
            } else {
                new_eqns_cont = metamodelica::cons(new_eqn, new_eqns_cont);
            }
            assign_field!(aux.parsed = true);
            UnorderedMap::add(id, aux, map.clone())?;
        }
    }
    Ok((
        new_vars_disc,
        new_vars_cont,
        new_vars_init,
        new_vars_recd,
        new_eqns_disc,
        new_eqns_cont,
        new_eqns_init,
    ))
}

fn introduceFunctionAliasEquation(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Call_Id::Call_Id>, metamodelica::Ref<Call_Aux::Call_Aux>>,
    >,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut aux_index: Pointer::Pointer<i32>,
    mut eqn_index: Pointer::Pointer<i32>,
    mut init: bool,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
    #[repr(i32)]
    pub(crate) enum Depth {
        FULL = 1,
        CONDITION = 2,
        STOP = 3,
    }
    impl PartialOrd for Depth {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Depth {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            (*self as i32).cmp(&(*other as i32))
        }
    }
    impl metamodelica::gc::MMTrace for Depth {
        fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            Ok(())
        }
    }

    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut iter: metamodelica::Ref<Iterator::Iterator>;
    let mut depth: Depth;
    (eqn, _) = Inline::inlineArrayConstructorSingle(
        eqn,
        &(crate::NBEquation::Iterator::interned_EMPTY()),
        variables,
        set,
        eqn_index,
        Pointer::create(metamodelica::nil()),
    )?;
    (iter, depth) = (::match_deref::match_deref! { match &(&*eqn) {
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: body, tail: Deref @ metamodelica::ListNode::Nil }, iter: __eqn_iter, .. } => {
            (__eqn_iter.clone(), if (BEquation::Equation::isWhenEquation(Pointer::create(body.clone()))? || BEquation::Equation::isIfEquation(Pointer::create(body.clone()))) {Depth::CONDITION.clone()} else {Depth::FULL.clone()})
        },
        Deref @ BEquation::Equation::WHEN_EQUATION { .. } => {
            (crate::NBEquation::Iterator::interned_EMPTY(), Depth::CONDITION.clone())
        },
        Deref @ BEquation::Equation::IF_EQUATION { .. } => {
            (crate::NBEquation::Iterator::interned_EMPTY(), Depth::CONDITION.clone())
        },
        Deref @ BEquation::Equation::ALGORITHM { .. } => {
            (crate::NBEquation::Iterator::interned_EMPTY(), Depth::STOP.clone())
        },
        _ => {
            (crate::NBEquation::Iterator::interned_EMPTY(), Depth::FULL.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if depth == Depth::FULL.clone() {
        eqn = BEquation::Equation::map(
            eqn,
            (std::sync::Arc::new({
                let __pe_b1 = map;
                let __pe_b2 = aux_index;
                let __pe_b3 = iter;
                let __pe_b4 = init;
                move |__pe_a0| {
                    introduceFunctionAlias(
                        __pe_a0,
                        __pe_b1.clone(),
                        __pe_b2.clone(),
                        __pe_b3.clone(),
                        __pe_b4.clone(),
                    )
                }
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
            None,
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<Expression::NFExpression>,
                      __a1: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1)),
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Expression::NFExpression>,
                                    )
                                        -> Result<metamodelica::Ref<Expression::NFExpression>>
                                    + 'static,
                            >,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
    } else if depth == Depth::CONDITION.clone() {
        eqn = BEquation::Equation::mapCondition(
            eqn,
            (std::sync::Arc::new({
                let __pe_b1 = map;
                let __pe_b2 = aux_index;
                let __pe_b3 = iter;
                let __pe_b4 = init;
                move |__pe_a0| {
                    introduceFunctionAlias(
                        __pe_a0,
                        __pe_b1.clone(),
                        __pe_b2.clone(),
                        __pe_b3.clone(),
                        __pe_b4.clone(),
                    )
                }
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
            None,
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<Expression::NFExpression>,
                      __a1: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1)),
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Expression::NFExpression>,
                                    )
                                        -> Result<metamodelica::Ref<Expression::NFExpression>>
                                    + 'static,
                            >,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
    }
    Ok(eqn)
}

fn introduceFunctionAlias(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Call_Id::Call_Id>, metamodelica::Ref<Call_Aux::Call_Aux>>,
    >,
    mut aux_index: Pointer::Pointer<i32>,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut init: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut deep_iter: metamodelica::Ref<Iterator::Iterator>;
    deep_iter = (match &*exp {
        Expression::CALL { call: __exp_call } => {
            BEquation::Iterator::expand(iter.clone(), metamodelica::AsArg::as_arg(&__exp_call))?
        }
        _ => iter.clone(),
    });
    exp = Expression::mapShallow(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = map.clone();
            let __pe_b2 = aux_index.clone();
            let __pe_b3 = deep_iter;
            let __pe_b4 = init;
            move |__pe_a0| {
                introduceFunctionAlias(
                    __pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                )
            }
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: __exp_call } if (checkCallReplacement(metamodelica::AsArg::as_arg(&__exp_call))?) => {
            introduceAlias(exp, map, aux_index, &(arcstr::literal!(BVariable::FUNCTION_STR)), iter, init)?
        },
        new_exp @ Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut new_exp = (*new_exp).clone();
            let mut call = (*call).clone();
            assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::map(arg.clone(), (std::sync::Arc::new({ let __pe_b1 = map.clone(); let __pe_b2 = aux_index.clone(); let __pe_b3 = iter.clone(); let __pe_b4 = init; move |__pe_a0| introduceArrayConstructorAlias(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            assign_variant_field!(new_exp => Expression::NFExpression::CALL; call = call.clone());
            new_exp.clone()
        },
        Deref @ Expression::MULTARY { arguments: __exp_arguments, .. } => {
            assign_variant_field!(exp => Expression::NFExpression::MULTARY;
                arguments = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut arg in (__exp_arguments.clone()).into_iter().cloned() {
            let __x = introduceArrayConstructorAlias(arg.clone(), map.clone(), aux_index.clone(), iter.clone(), init)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                inv_arguments = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = introduceArrayConstructorAlias(arg.clone(), map.clone(), aux_index.clone(), iter.clone(), init)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
            );
            exp
        },
        Deref @ Expression::BINARY { exp1: __exp_exp1, .. } => {
            assign_variant_field!(exp => Expression::NFExpression::BINARY;
                exp1 = introduceArrayConstructorAlias(__exp_exp1.clone(), map.clone(), aux_index.clone(), iter.clone(), init)?,
                exp2 = introduceArrayConstructorAlias(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), map, aux_index, iter, init)?
            );
            exp
        },
        Deref @ Expression::TUPLE_ELEMENT { tupleExp: sub_exp @ Deref @ Expression::TUPLE { .. }, index: __exp_index, .. } => {
            let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
            if __exp_index.clone() > ((var_field!((**sub_exp).elements, Expression::NFExpression::TUPLE)).len() as i32) {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBFunctionAlias.introduceFunctionAlias")); __mm_s.push_str(&*literal!(" failed to get subscripted tuple element: ")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }])?;
                return Err("fail");
            } else {
                new_exp = (var_field!((**sub_exp).elements, Expression::NFExpression::TUPLE)).get(__exp_index.clone())?;
            }
            new_exp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn introduceArrayConstructorAlias(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Call_Id::Call_Id>, metamodelica::Ref<Call_Aux::Call_Aux>>,
    >,
    mut aux_index: Pointer::Pointer<i32>,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut init: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => introduceAlias(exp, map, aux_index, &(arcstr::literal!(BVariable::FUNCTION_STR)), iter, init)?,
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_REDUCTION { .. } } => introduceAlias(exp, map, aux_index, &(arcstr::literal!(BVariable::FUNCTION_STR)), iter, init)?,
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn introduceAliasCrefConditional(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Call_Id::Call_Id>, metamodelica::Ref<Call_Aux::Call_Aux>>,
    >,
    mut aux_index: Pointer::Pointer<i32>,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut init: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. }
            if (UnorderedSet::contains(
                ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&__exp_cref)),
                set.clone(),
            )?) =>
        {
            introduceAlias(
                exp,
                map,
                aux_index,
                &(arcstr::literal!(BVariable::STATE_ALIAS_STR)),
                iter,
                init,
            )?
        }
        _ => exp,
    });
    Ok(exp)
}

fn introduceAlias(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Call_Id::Call_Id>, metamodelica::Ref<Call_Aux::Call_Aux>>,
    >,
    mut aux_index: Pointer::Pointer<i32>,
    mut aux_name: &ArcStr,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut init: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator::Iterator>>>;
    let mut new_iter: metamodelica::Ref<Iterator::Iterator>;
    let mut id: metamodelica::Ref<Call_Id::Call_Id>;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut aux: metamodelica::Ref<Call_Aux::Call_Aux>;
    let mut aux_opt: Option<metamodelica::Ref<Call_Aux::Call_Aux>>;
    let mut tpl_lst: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    if !(BEquation::Iterator::isEmpty(&iter)) {
        (names, ranges, maps) = BEquation::Iterator::getFrames(&iter);
        new_iter = BEquation::Iterator::fromFrames(filterFrames(exp.clone(), names, ranges, maps)?);
    } else {
        new_iter = iter.clone();
    }
    id = metamodelica::Ref::new(Call_Id::Call_Id {
        call: exp.clone(),
        iter: new_iter.clone(),
    });
    aux_opt = UnorderedMap::get(id.clone(), map.clone())?;
    if (aux_opt).is_some() {
        aux = Util::getOption(aux_opt)?;
        exp = aux.replacer.clone();
    } else {
        (exp, aux_opt) = (::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg2: metamodelica::Ref<Expression::NFExpression>;
                let mut call = (*call).clone();
                let () = (::match_deref::match_deref! { match &((Call::functionName(metamodelica::AsArg::as_arg(&call))?, var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone())) {
            (Deref @ Absyn::Path::IDENT { name: Deref @ "cat" }, _) => {
                assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = metamodelica::cons((var_field!((*call).arguments, Call::NFCall::TYPED_CALL)).head().cloned()?, ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut arg in ((var_field!((*call).arguments, Call::NFCall::TYPED_CALL)).rest()?).into_iter().cloned() {
                let __x = if (Expression::isLiteral(&(arg.clone()))? || Expression::isCref(&(arg.clone()))) {arg.clone()} else {introduceAlias(arg.clone(), map.clone(), aux_index.clone(), aux_name, iter.clone(), init)?};
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })));
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
                id = metamodelica::Ref::new(Call_Id::Call_Id { call: exp.clone(), iter: new_iter.clone() });
                aux_opt = UnorderedMap::get(id.clone(), map.clone())?;
                ()
            },
            (Deref @ Absyn::Path::IDENT { name: Deref @ "promote" }, Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                arg1 = (*__esc_arg1).clone();
                arg2 = (*__esc_arg2).clone();
                assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = list![if (Expression::isLiteral(metamodelica::AsArg::as_arg(&arg1))? || Expression::isCref(metamodelica::AsArg::as_arg(&arg1))) {arg1.clone()} else {introduceAlias(arg1.clone(), map.clone(), aux_index.clone(), aux_name, iter, init)?}, arg2.clone()]);
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
                id = metamodelica::Ref::new(Call_Id::Call_Id { call: exp.clone(), iter: new_iter.clone() });
                aux_opt = UnorderedMap::get(id.clone(), map.clone())?;
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                (exp, aux_opt)
            },
            _ => {
                (exp, aux_opt)
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        ty = Expression::typeOf(exp);
        exp = (::match_deref::match_deref! { match &((&aux_opt, &*ty)) {
            (Some(__esc_aux), _) => {
                aux = (*__esc_aux).clone();
                aux.replacer.clone()
            },
            (_, Deref @ Type::TUPLE { .. }) => {
                names = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut sub_ty in (var_field!((*ty).types, Type::NFType::TUPLE).clone()).into_iter().cloned() {
                let __x = Call_Aux::createName(sub_ty.clone(), &new_iter, aux_index.clone(), aux_name, init)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                tpl_lst = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut cref in (names).into_iter().cloned() {
                let __x = if (ComponentRef::size(&(cref.clone()), true, false)? == 0) {Expression::fromCref(openmodelica_nf_frontend::NFComponentRef::interned_WILD(), false)?} else {Expression::fromCref(cref.clone(), false)?};
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                metamodelica::Ref::new(Expression::NFExpression::TUPLE { ty: ty.clone(), elements: tpl_lst })
            },
            _ => {
                name = Call_Aux::createName(ty.clone(), &new_iter, aux_index, aux_name, init)?;
                Expression::fromCref(name, false)?
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if (aux_opt).is_none() {
            aux = metamodelica::Ref::new(Call_Aux::Call_Aux {
                replacer: exp.clone(),
                kind: if (Type::isDiscrete(ty)?) {
                    EquationKind::DISCRETE.clone()
                } else {
                    EquationKind::CONTINUOUS.clone()
                },
                parsed: false,
            });
            UnorderedMap::add(id, aux, map)?;
        }
    }
    Ok(exp)
}

fn checkCallReplacement(mut call: &metamodelica::Ref<Call::NFCall>) -> Result<bool> {
    let mut b: bool;
    let mut r#fn: metamodelica::Ref<Function::Function> = Call::typedFunction(call)?;
    b = forceReplacement(&r#fn) || !(Function::isSpecialBuiltin(&r#fn) || replaceException(&r#fn)?);
    Ok(b)
}

fn forceReplacement(mut r#fn: &metamodelica::Ref<Function::Function>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(&(Function::nameConsiderBuiltin(r#fn)))) {
        Deref @ "cat" => true,
        Deref @ "terminal" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn replaceException(mut r#fn: &metamodelica::Ref<Function::Function>) -> Result<bool> {
    let mut b: bool;
    let mut path: metamodelica::Ref<Absyn::Path>;
    if Function::isDefaultRecordConstructor(r#fn)?
        || Function::isNonDefaultRecordConstructor(r#fn)
        || Function::isImpure(r#fn)
        || (r#fn.outputs).is_empty()
    {
        b = true;
        return Ok(b);
    }
    if !(Function::isBuiltin(r#fn)) {
        b = false;
    } else {
        path = Function::nameConsiderBuiltin(r#fn);
        if !(AbsynUtil::pathIsIdent(&path)) {
            b = false;
        } else {
            b = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(&path)) {
                Deref @ "integer" => true,
                Deref @ "String" => true,
                Deref @ "$OMC$PositiveMax" => true,
                Deref @ "$OMC$inStreamDiv" => true,
                _ => false,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
    }
    Ok(b)
}

fn filterFrames(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator::Iterator>>>,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>,
> {
    pub(crate) type FrameTuple = (
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    );

    fn collectFrames(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut frame_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                (
                    metamodelica::Ref<Expression::NFExpression>,
                    Option<metamodelica::Ref<Iterator::Iterator>>,
                ),
            >,
        >,
        mut new_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                (
                    metamodelica::Ref<Expression::NFExpression>,
                    Option<metamodelica::Ref<Iterator::Iterator>>,
                ),
            >,
        >,
        mut sub_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            >,
        >,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let () = (match &*exp {
            Expression::CREF { cref: __exp_cref, .. } => {
                let mut frame_tpl; // TODO: local with unresolved type
                let mut parent: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let () = (::match_deref::match_deref! { match &(UnorderedMap::get(__exp_cref.clone(), frame_map.clone())?) {
                    Some(__esc_frame_tpl) => {
                        frame_tpl = (*__esc_frame_tpl).clone();
                        UnorderedMap::add(__exp_cref.clone(), frame_tpl.clone(), new_map)?;
                        ()
                    },
                    _ => {
                        let () = (::match_deref::match_deref! { match &(UnorderedMap::get(__exp_cref.clone(), sub_map)?) {
                    Some(__esc_parent) => {
                        parent = (*__esc_parent).clone();
                        let () = (::match_deref::match_deref! { match &(UnorderedMap::get(parent.clone(), frame_map)?) {
                    Some(__esc_frame_tpl) => {
                        frame_tpl = (*__esc_frame_tpl).clone();
                        UnorderedMap::add(parent.clone(), frame_tpl.clone(), new_map)?;
                        ()
                    },
                    _ => (),
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                        ()
                    },
                    _ => (),
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                        ()
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                ()
            }
            _ => (),
        });
        Ok(exp)
    }

    let mut frames: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>;
    let mut frame_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            (
                metamodelica::Ref<Expression::NFExpression>,
                Option<metamodelica::Ref<Iterator::Iterator>>,
            ),
        >,
    > = UnorderedMap::new(
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
    let mut new_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            (
                metamodelica::Ref<Expression::NFExpression>,
                Option<metamodelica::Ref<Iterator::Iterator>>,
            ),
        >,
    > = UnorderedMap::new(
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
    let mut sub_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    > = UnorderedMap::new(
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
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut map: Option<metamodelica::Ref<Iterator::Iterator>>;
    let mut n: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = names;
    let mut local_n: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut r: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = ranges;
    let mut m: metamodelica::List<Option<metamodelica::Ref<Iterator::Iterator>>> = maps;
    while !((n).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(n) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        name = metamodelica::Own::own(__pa0);
        n = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(r) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        range = metamodelica::Own::own(__pa2);
        r = metamodelica::Own::own(__pa3);
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(m) {
            Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        map = metamodelica::Own::own(__pa4);
        m = metamodelica::Own::own(__pa5);
        UnorderedMap::add(name.clone(), (range, map.clone()), frame_map.clone())?;
        if (map).is_some() {
            (local_n, _, _) = BEquation::Iterator::getFrames(&(Util::getOption(map)?));
            for mut cref in &*local_n {
                UnorderedMap::add(cref.clone(), name.clone(), sub_map.clone())?;
            }
        }
    }
    Expression::map(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = frame_map;
            let __pe_b2 = new_map.clone();
            let __pe_b3 = sub_map;
            move |__pe_a0| collectFrames(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    n = UnorderedMap::keyList(new_map.clone());
    (r, m) = List::unzip(&(UnorderedMap::valueList(new_map)));
    frames = List::zip3(n, r, m);
    Ok(frames)
}

fn addAuxVar(
    mut new_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut disc: bool,
    mut new_vars_disc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut new_vars_cont: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut new_vars_init: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut new_vars_recd: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut init: bool,
) -> Result<(
    bool,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
)> {
    let mut disc: bool = disc;
    let mut new_vars_disc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        new_vars_disc;
    let mut new_vars_cont: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        new_vars_cont;
    let mut new_vars_init: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        new_vars_init;
    let mut new_vars_recd: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        new_vars_recd;
    let mut children: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    if BVariable::isRecord(new_var.clone()) {
        new_vars_recd = metamodelica::cons(new_var.clone(), new_vars_recd);
        let __pa0 = ::match_deref::match_deref! { match &(Variable::expandChildren(Pointer::access(new_var), &(metamodelica::nil()), false)?) {
            Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        children = metamodelica::Own::own(__pa0);
        for mut child in &*children {
            (disc, new_vars_disc, new_vars_cont, new_vars_init, new_vars_recd) = addAuxVar(
                (BVariable::makeVarPtr(child.clone(), child.name.clone())?).0,
                disc,
                new_vars_disc,
                new_vars_cont,
                new_vars_init,
                new_vars_recd,
                init,
            )?;
        }
    } else if init {
        new_vars_init = metamodelica::cons(BVariable::setFixed(new_var, false, false)?, new_vars_init);
    } else if BVariable::isContinuous(new_var.clone(), false)? {
        disc = false;
        new_vars_cont = metamodelica::cons(new_var, new_vars_cont);
    } else {
        new_vars_disc = metamodelica::cons(new_var, new_vars_disc);
    }
    Ok((disc, new_vars_disc, new_vars_cont, new_vars_init, new_vars_recd))
}

fn addClockedAlias(
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut eqn_idx: Pointer::Pointer<i32>,
) -> Result<(
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<BClock::BClock>, metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >,
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<BClock::BClock>, metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >,
)> {
    let mut clock_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut infer_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut clock_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut infer_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut clck_coll: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<BClock::BClock>, metamodelica::Ref<ComponentRef::NFComponentRef>>,
    > = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<BClock::BClock>| Partitioning::BClock::hash(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BClock::BClock>, __a1: metamodelica::Ref<BClock::BClock>| {
                Partitioning::BClock::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>, metamodelica::Ref<BClock::BClock>) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    let mut infr_coll: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<BClock::BClock>, metamodelica::Ref<ComponentRef::NFComponentRef>>,
    > = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<BClock::BClock>| Partitioning::BClock::hash(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BClock::BClock>, __a1: metamodelica::Ref<BClock::BClock>| {
                Partitioning::BClock::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>, metamodelica::Ref<BClock::BClock>) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    let mut new_clocks: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut new_infers: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut idx: Pointer::Pointer<i32> = Pointer::create(0);
    let mut clock: metamodelica::Ref<BClock::BClock>;
    let mut clock_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    BEquation::EquationPointers::map(
        equations,
        &({
            let __pe_b1 = clck_coll.clone();
            let __pe_b2 = infr_coll.clone();
            let __pe_b3 = new_clocks.clone();
            let __pe_b4 = new_infers.clone();
            let __pe_b5 = idx;
            move |__pe_a0| {
                Partitioning::extractClocksEqn(
                    __pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                )
            }
        }),
    )?;
    clock_vars = Pointer::access(new_clocks);
    for mut tpl in &*UnorderedMap::toList(clck_coll.clone()) {
        (clock, clock_name) = tpl.clone();
        clock_eqns = metamodelica::cons(
            BEquation::Equation::makeAssignment(
                Expression::fromCref(clock_name, false)?,
                Partitioning::BClock::toExp(&clock)?,
                eqn_idx.clone(),
                &(literal!("AUX")),
                crate::NBEquation::Iterator::interned_EMPTY(),
                BEquation::default(EquationKind::CLOCKED.clone(), false, None, None),
            )?,
            clock_eqns,
        );
    }
    infer_vars = Pointer::access(new_infers);
    for mut tpl in &*UnorderedMap::toList(infr_coll.clone()) {
        (clock, clock_name) = tpl.clone();
        infer_eqns = metamodelica::cons(
            BEquation::Equation::makeAssignment(
                Expression::fromCref(clock_name, false)?,
                Partitioning::BClock::toExp(&clock)?,
                eqn_idx.clone(),
                &(literal!("AUX")),
                crate::NBEquation::Iterator::interned_EMPTY(),
                BEquation::default(EquationKind::CLOCKED.clone(), false, None, None),
            )?,
            infer_eqns,
        );
    }
    Ok((clock_eqns, infer_eqns, clock_vars, infer_vars, clck_coll, infr_coll))
}

// type for slice collection
pub type Indices = metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>;

fn collectSlicedStatesAliasEquation(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>,
        >,
    >,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut iter: metamodelica::Ref<Iterator::Iterator> = BEquation::Equation::getForIterator(&eqn);
    BEquation::Equation::map(
        eqn.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = iter;
            let __pe_b2 = map;
            move |__pe_a0| collectSlicedStatesAlias(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
        None,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Expression::NFExpression>,
                  __a1: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1)),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(eqn)
}

fn collectSlicedStatesAlias(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { r#fn: Deref @ Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, arguments: Deref @ metamodelica::ListNode::Cons { head: arg, tail: Deref @ metamodelica::ListNode::Nil }, .. } } => {
            let mut iter_size: i32;
            let mut cref_size: i32;
            let mut var_size: i32;
            let mut call_crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
            let mut stripped_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut indices: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>;
            let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator::Iterator>>>;
            iter_size = BEquation::Iterator::size(&iter, true)?;
            call_crefs = UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            Slice::filterExp(arg.clone(), (std::sync::Arc::new({ let __pe_b2 = false; move |__pe_a0, __pe_a1| Slice::getContinuous(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> + 'static>), call_crefs.clone())?;
            for mut cref in &*UnorderedSet::toList(call_crefs) {
                cref_size = Type::sizeOf(&(ComponentRef::getSubscriptedType(metamodelica::AsArg::as_arg(&cref), false)?), true)?;
                var_size = BVariable::size(BVariable::getVarPointer(metamodelica::AsArg::as_arg(&cref), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBFunctionAlias.mo"))?, true)?;
                if var_size != cref_size * iter_size {
                    stripped_cref = ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&cref));
                    indices = UnorderedMap::getOrDefault(stripped_cref.clone(), map.clone(), UnorderedSet::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 13))?;
                    (names, ranges, maps) = BEquation::Iterator::getFrames(&iter);
                    for mut index in &*Slice::getCrefInFrameIndicesLocal(metamodelica::AsArg::as_arg(&cref), &stripped_cref, &(List::zip3(names, ranges, maps)), 0, true)? {
                        UnorderedSet::add(index.clone(), indices.clone())?;
                    }
                    UnorderedMap::add(stripped_cref, indices, map.clone())?;
                }
            }
            exp
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
            Expression::mapShallow(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = BEquation::Iterator::expand(iter, var_field!((*exp).call, Expression::NFExpression::CALL))?; let __pe_b2 = map; move |__pe_a0| collectSlicedStatesAlias(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            exp
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_REDUCTION { .. } } => {
            Expression::mapShallow(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = BEquation::Iterator::expand(iter, var_field!((*exp).call, Expression::NFExpression::CALL))?; let __pe_b2 = map; move |__pe_a0| collectSlicedStatesAlias(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            exp
        },
        _ => {
            Expression::mapShallow(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = iter; let __pe_b2 = map; move |__pe_a0| collectSlicedStatesAlias(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn getSlicedStatesSet(
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>,
        >,
    >,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>> {
    let mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        UnorderedSet::new(
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
            13,
        );
    let mut state: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut indices: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>;
    for mut tpl in &*UnorderedMap::toList(map) {
        (state, indices) = tpl.clone();
        if BVariable::size(
            BVariable::getVarPointer(
                &state,
                metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBFunctionAlias.mo"),
            )?,
            true,
        )? != UnorderedSet::size(indices)
        {
            UnorderedSet::add(state, set.clone())?;
        }
    }
    Ok(set)
}

fn introduceSlicedStateAliasEquation(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Call_Id::Call_Id>, metamodelica::Ref<Call_Aux::Call_Aux>>,
    >,
    mut aux_index: Pointer::Pointer<i32>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut iter: metamodelica::Ref<Iterator::Iterator> = BEquation::Equation::getForIterator(&eqn);
    eqn = BEquation::Equation::map(
        eqn,
        (std::sync::Arc::new({
            let __pe_b1 = set;
            let __pe_b2 = map;
            let __pe_b3 = iter;
            let __pe_b4 = aux_index;
            move |__pe_a0| {
                introduceSlicedStateAliasExp(
                    __pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                )
            }
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
        None,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Expression::NFExpression>,
                  __a1: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1)),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(eqn)
}

fn introduceSlicedStateAliasExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Call_Id::Call_Id>, metamodelica::Ref<Call_Aux::Call_Aux>>,
    >,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut aux_index: Pointer::Pointer<i32>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { r#fn: Deref @ Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, arguments: Deref @ metamodelica::ListNode::Cons { head: arg, tail: Deref @ metamodelica::ListNode::Nil }, .. } } => {
            let mut call = (*call).clone();
            assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = list![Expression::map(arg.clone(), (std::sync::Arc::new({ let __pe_b1 = set; let __pe_b2 = map; let __pe_b3 = aux_index; let __pe_b4 = iter; let __pe_b5 = false; move |__pe_a0| introduceAliasCrefConditional(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?]);
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            exp
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
            let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
            new_exp = Expression::mapShallow(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = set; let __pe_b2 = map; let __pe_b3 = BEquation::Iterator::expand(iter, var_field!((*exp).call, Expression::NFExpression::CALL))?; let __pe_b4 = aux_index; move |__pe_a0| introduceSlicedStateAliasExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            new_exp
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_REDUCTION { .. } } => {
            let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
            new_exp = Expression::mapShallow(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = set; let __pe_b2 = map; let __pe_b3 = BEquation::Iterator::expand(iter, var_field!((*exp).call, Expression::NFExpression::CALL))?; let __pe_b4 = aux_index; move |__pe_a0| introduceSlicedStateAliasExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            new_exp
        },
        _ => {
            let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
            new_exp = Expression::mapShallow(exp, (std::sync::Arc::new({ let __pe_b1 = set; let __pe_b2 = map; let __pe_b3 = iter; let __pe_b4 = aux_index; move |__pe_a0| introduceSlicedStateAliasExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            new_exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}
