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
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes::Variability;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFType as Type;
use crate::NFUnit as Unit;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

//import DAE;
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Functionargs {
    pub name: ArcStr,
    pub invars: metamodelica::List<ArcStr>,
    pub outvars: metamodelica::List<ArcStr>,
    pub inunits: metamodelica::List<ArcStr>,
    pub outunits: metamodelica::List<ArcStr>,
}

impl metamodelica::gc::MMTrace for Functionargs {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.invars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outvars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.inunits, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outunits, __mmv)?;
        Ok(())
    }
}
impl Default for Functionargs {
    fn default() -> Self {
        Self {
            name: Default::default(),
            invars: Default::default(),
            outvars: Default::default(),
            inunits: Default::default(),
            outunits: Default::default(),
        }
    }
}

pub type FUNCTIONUNITS = Functionargs;

pub type FunctionUnitCache = metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Functionargs>>;

pub fn checkUnits(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut htCr2U1: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >;
    let mut htCr2U2: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >;
    let mut htS2U: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>;
    let mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>;
    let mut fn_cache: FunctionUnitCache;
    if !(Flags::getConfigBool(Flags::UNIT_CHECKING.clone())? || Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) {
        return Ok(flatModel);
    }
    if '__try0: {
        htCr2U1 = Unit::newCrefUnitTable(Util::nextPrime(((metamodelica::OrderedFloat((10) as f64) + metamodelica::OrderedFloat(1.4_f64) * metamodelica::OrderedFloat((((flatModel.variables).len() as i32)) as f64)).0.floor() as i32)));
        htS2U = unwrap_break_err!(Unit::getKnownUnits(), '__try0);
        htU2S = unwrap_break_err!(Unit::getKnownUnitsInverse(), '__try0);
        fn_cache = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1);
        for mut v in &*flatModel.variables.clone() {
            unwrap_break_err!(convertUnitStringToUnit(metamodelica::AsArg::as_arg(&v), htCr2U1.clone(), htS2U.clone(), htU2S.clone()), '__try0);
        }
        htCr2U2 = UnorderedMap::copy(htCr2U1.clone());
        htCr2U2 = unwrap_break_err!(checkModelConsistency(&flatModel.variables, &flatModel.equations, &flatModel.initialEquations, htCr2U2.clone(), htS2U.clone(), htU2S.clone(), fn_cache.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_UNIT.clone()), '__try0) {
            metamodelica::print(unwrap_break_err!(UnorderedMap::toString(htCr2U2.clone(), &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0), &move |__a0: Unit::Unit| Unit::unit2string(&__a0), literal!("\n"), &(literal!(", "))), '__try0));
            metamodelica::print(literal!("\n######## UnitCheck COMPLETED ########\n"));
        }
        unwrap_break_err!(notification(htCr2U1.clone(), htCr2U2.clone(), htU2S.clone()), '__try0);
        flatModel = unwrap_break_err!(updateModel(flatModel.clone(), htCr2U2.clone(), htU2S.clone()), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
        Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFUnitCheck.checkUnits")); __mm_s.push_str(&*literal!(": unit check module failed")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("NFFrontEnd/NFUnitCheck.mo"))?;
    }
    execStat(&(literal!("NFUnitCheck.checkUnits")))?;
    Ok(flatModel)
}

fn updateModel(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
    mut htCr2U: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
    mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    assign_field!(
        flatModel.variables = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut v in (flatModel.variables.clone()).into_iter().cloned() {
                let __x = updateVariable(v.clone(), htCr2U.clone(), htU2S.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(flatModel)
}

fn updateVariable(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut htCr2U: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
    mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    let mut name: ArcStr;
    let mut unit_str: ArcStr;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut unit_idx: i32 = 0;
    let mut unit: Unit::Unit;
    if Type::isReal(&var.ty)? {
        for mut attr in &*var.typeAttributes.clone() {
            (name, binding) = attr.clone();
            unit_idx = unit_idx + 1;
            if metamodelica::stringEq(&name, &(literal!("unit"))) {
                if Binding::isBound(&binding) {
                    return Ok(var.clone());
                } else {
                    assign_field!(var.typeAttributes = listDelete(var.typeAttributes.clone(), unit_idx)?);
                    break;
                }
            }
        }
        if '__try0: {
            unit = unwrap_break_err!(UnorderedMap::getOrFail(var.name.clone(), htCr2U.clone()), '__try0);
            if Unit::isUnit(&unit) {
                unit_str = unwrap_break_err!(Unit::unitString(unit.clone(), htU2S.clone()), '__try0);
                binding = Binding::makeFlat(
                    metamodelica::Ref::new(Expression::NFExpression::STRING {
                        value: unit_str.clone(),
                    }),
                    Variability::CONSTANT.clone(),
                    Binding::Source::GENERATED.clone(),
                    Binding::NO_CONFIDENCE.clone(),
                );
                assign_field!(
                    var.typeAttributes =
                        metamodelica::cons((literal!("unit"), binding.clone()), var.typeAttributes.clone())
                );
            }
            Ok::<(), &'static str>(())
        }
        .is_err()
        {}
    }
    Ok(var)
}

fn notification(
    mut inHtCr2U1: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
    mut inHtCr2U2: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
    mut inHtU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
) -> Result<()> {
    let mut r#str: ArcStr;
    let mut lt1: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit)>;
    lt1 = UnorderedMap::toList(inHtCr2U1);
    r#str = notification2(lt1, inHtCr2U2, inHtU2S)?;
    if Flags::isSet(Flags::DUMP_UNIT.clone())? && !metamodelica::stringEq(&r#str, &(literal!(""))) {
        Error::addCompilerNotification(r#str)?;
    }
    Ok(())
}

fn notification2(
    mut inLt1: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit)>,
    mut inHtCr2U2: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
    mut inHtU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
) -> Result<ArcStr> {
    let mut outS: ArcStr;
    let mut cr1: metamodelica::Ref<ComponentRef::NFComponentRef> = crate::NFComponentRef::interned_EMPTY();
    let mut factor: metamodelica::Real = metamodelica::OrderedFloat((0) as f64);
    let mut offset: metamodelica::Real = metamodelica::OrderedFloat((0) as f64);
    let mut s: i32 = 0;
    let mut m: i32 = 0;
    let mut g: i32 = 0;
    let mut A: i32 = 0;
    let mut K: i32 = 0;
    let mut mol: i32 = 0;
    let mut cd: i32 = 0;
    outS = stringAppendList(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut t1 in (inLt1).into_iter().cloned() {
                if !(::match_deref::match_deref! { match &(t1.clone()) {
                    (__esc_cr1, Unit::Unit::MASTER { .. }) => {
                        cr1 = (*__esc_cr1).clone();
                        let mut b: bool;
                        b = false;
                        if '__try0: {
                            let Unit::UNIT { s: __pa1, m: __pa2, g: __pa3, A: __pa4, K: __pa5, mol: __pa6, cd: __pa7, factor: __pa8, offset: __pa9 } = (unwrap_break_err!(UnorderedMap::getOrFail((ComponentRef::stripSubscripts(cr1.clone())).0, inHtCr2U2.clone()), '__try0)) else { break '__try0 Err::<_, _>("pattern mismatch") };
                            s = metamodelica::Own::own(__pa1);
                            m = metamodelica::Own::own(__pa2);
                            g = metamodelica::Own::own(__pa3);
                            A = metamodelica::Own::own(__pa4);
                            K = metamodelica::Own::own(__pa5);
                            mol = metamodelica::Own::own(__pa6);
                            cd = metamodelica::Own::own(__pa7);
                            factor = metamodelica::Own::own(__pa8);
                            offset = metamodelica::Own::own(__pa9);
                            b = true;
                            Ok::<(), &'static str>(())
                        }.is_err() {
                        }
                        b
                    },
                    _ => {
                        false
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } }) {
                    continue;
                }
                let __x = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("\""));
                    __mm_s.push_str(&*ComponentRef::toString(&cr1)?);
                    __mm_s.push_str(&*literal!("\" has the Unit \""));
                    __mm_s.push_str(&*Unit::unitString(
                        Unit::Unit::UNIT {
                            s: s,
                            m: m,
                            g: g,
                            A: A,
                            K: K,
                            mol: mol,
                            cd: cd,
                            factor: factor,
                            offset: offset,
                        },
                        inHtU2S.clone(),
                    )?);
                    __mm_s.push_str(&*literal!("\"\n"));
                    ArcStr::from(__mm_s)
                };
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    );
    Ok(outS)
}

fn checkModelConsistency(
    mut variables: &metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut equations: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut initialEquations: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut htCr2U: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
    mut htS2U: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>,
    mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
    mut fnCache: FunctionUnitCache,
) -> Result<metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>>>
{
    let mut htCr2U: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    > = htCr2U;
    let mut dump_eq_unit: bool = Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())?;
    for mut v in &**variables {
        foldBindingExp(
            metamodelica::AsArg::as_arg(&v),
            htCr2U.clone(),
            htS2U.clone(),
            htU2S.clone(),
            fnCache.clone(),
            dump_eq_unit,
        )?;
        for mut c in &*v.children.clone() {
            foldBindingExp(
                metamodelica::AsArg::as_arg(&c),
                htCr2U.clone(),
                htS2U.clone(),
                htU2S.clone(),
                fnCache.clone(),
                dump_eq_unit,
            )?;
        }
    }
    for mut eq in &**equations {
        foldEquation(
            metamodelica::AsArg::as_arg(&eq),
            htCr2U.clone(),
            htS2U.clone(),
            htU2S.clone(),
            fnCache.clone(),
            dump_eq_unit,
        )?;
    }
    for mut ieq in &**initialEquations {
        foldEquation(
            metamodelica::AsArg::as_arg(&ieq),
            htCr2U.clone(),
            htS2U.clone(),
            htU2S.clone(),
            fnCache.clone(),
            dump_eq_unit,
        )?;
    }
    Ok(htCr2U)
}

fn foldBindingExp(
    mut var: &metamodelica::Ref<Variable::NFVariable>,
    mut htCr2U: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
    mut htS2U: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>,
    mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
    mut fnCache: FunctionUnitCache,
    mut dumpEqInitStruct: bool,
) -> Result<()> {
    let mut binding_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut eq: metamodelica::Ref<Equation::NFEquation>;
    if Type::isReal(&var.ty)? && Binding::isBound(&var.binding) {
        binding_exp = Binding::getTypedExp(&var.binding)?;
        eq = Equation::makeEquality(
            Expression::fromCref(var.name.clone(), false)?,
            binding_exp,
            var.ty.clone(),
            ElementSource::createElementSource(
                var.info.clone(),
                None,
                &(openmodelica_frontend_types::DAE::Prefix::NOPRE),
                (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
            ),
            crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
            Equation::ScalarizeMode::NO_PREFERENCE.clone(),
        );
        foldEquation(&eq, htCr2U, htS2U, htU2S, fnCache, dumpEqInitStruct)?;
    }
    Ok(())
}

fn foldEquation(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut htCr2U: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
    mut htS2U: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>,
    mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
    mut fnCache: FunctionUnitCache,
    mut dumpEqInitStruct: bool,
) -> Result<()> {
    let mut inconsistent_units: metamodelica::List<
        metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>,
    >;
    inconsistent_units = foldEquation2(eq, dumpEqInitStruct, htCr2U, htS2U, htU2S.clone(), fnCache)?;
    for mut u in &*inconsistent_units {
        Errorfunction(u.clone(), eq, htU2S.clone())?;
    }
    Ok(())
}

fn foldEquation2(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut dumpEqInitStruct: bool,
    mut htCr2U: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
    mut htS2U: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>,
    mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
    mut fnCache: FunctionUnitCache,
) -> Result<metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>> {
    let mut inconsistentUnits: metamodelica::List<
        metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>,
    >;
    inconsistentUnits = (::match_deref::match_deref! { match eq {
        Deref @ Equation::EQUALITY { lhs: lhs @ Deref @ Expression::TUPLE { .. }, rhs: rhs @ Deref @ Expression::CALL { .. }, .. } if (!(Function::isBuiltin(&(Call::typedFunction(var_field!((**rhs).call, Expression::NFExpression::CALL))?)))) => {
            let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
            let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
            let mut fn_name: ArcStr;
            let mut out_vars: metamodelica::List<ArcStr>;
            let mut out_units: metamodelica::List<ArcStr>;
            fn_name = AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(Call::functionName(var_field!((**rhs).call, Expression::NFExpression::CALL))?), literal!("."), true, false)?;
            (_, out_vars, _, out_units) = getCallUnits(fn_name.clone(), var_field!((**rhs).call, Expression::NFExpression::CALL), fnCache.clone())?;
            icu1 = foldCallArg1(var_field!((**lhs).elements, Expression::NFExpression::TUPLE), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, out_units, out_vars, &fn_name)?;
            (_, icu2) = insertUnitInEquation(metamodelica::AsArg::as_arg(&rhs), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U, htS2U, htU2S, fnCache);
            List::append_reverse(&icu1, icu2)
        },
        Deref @ Equation::EQUALITY { rhs: rhs @ Deref @ Expression::CALL { .. }, lhs: __eq_lhs, .. } if (!(Function::isBuiltin(&(Call::typedFunction(var_field!((**rhs).call, Expression::NFExpression::CALL))?)))) => {
            let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
            let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
            let mut fn_name: ArcStr;
            let mut formal_args: ArcStr;
            let mut formal_var: ArcStr;
            let mut out_vars: metamodelica::List<ArcStr>;
            let mut out_units: metamodelica::List<ArcStr>;
            let mut unit1: Unit::Unit;
            let mut unit2: Unit::Unit;
            let mut b: bool;
            fn_name = AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(Call::functionName(var_field!((**rhs).call, Expression::NFExpression::CALL))?), literal!("."), true, false)?;
            (_, out_vars, _, out_units) = getCallUnits(fn_name.clone(), var_field!((**rhs).call, Expression::NFExpression::CALL), fnCache.clone())?;
            (unit1, _) = insertUnitInEquation(metamodelica::AsArg::as_arg(&__eq_lhs), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
            formal_args = (out_units).head().cloned()?;
            formal_var = (out_vars).head().cloned()?;
            unit2 = if (metamodelica::stringEq(&formal_args, &(literal!("NONE")))) {Unit::Unit::MASTER { varList: metamodelica::nil() }} else {Unit::parseUnitString(formal_args, htS2U.clone(), &(Equation::info(eq)))?};
            (b, _) = unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())?;
            if b {
                icu1 = metamodelica::nil();
            } else {
                icu1 = list![list![(__eq_lhs.clone(), unit1), (makeNewCref(formal_var, &fn_name)?, unit2)]];
            }
            (_, icu2) = insertUnitInEquation(metamodelica::AsArg::as_arg(&rhs), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U, htS2U, htU2S, fnCache);
            List::append_reverse(&icu1, icu2)
        },
        Deref @ Equation::EQUALITY { lhs: __eq_lhs, rhs: __eq_rhs, .. } => {
            let mut temp: metamodelica::Ref<Expression::NFExpression>;
            temp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: __eq_rhs.clone(), operator: Operator::makeSub(crate::NFType::interned_REAL()), exp2: __eq_lhs.clone() });
            if dumpEqInitStruct {
                metamodelica::print(Expression::toString(temp.clone())?);
                metamodelica::print(literal!("--------------------\n"));
            }
            (_, inconsistentUnits) = insertUnitInEquation(&temp, Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U, htS2U, htU2S, fnCache);
            inconsistentUnits
        },
        Deref @ Equation::WHEN { branches: Deref @ metamodelica::ListNode::Cons { head: Deref @ Equation::Branch::BRANCH { body: eql, .. }, tail: _ }, .. } => {
            let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
            inconsistentUnits = metamodelica::nil();
            for mut e in &*eql.clone() {
                icu1 = foldEquation2(metamodelica::AsArg::as_arg(&e), dumpEqInitStruct, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                inconsistentUnits = List::append_reverse(&icu1, inconsistentUnits);
            }
            inconsistentUnits
        },
        Deref @ Equation::NORETCALL { exp: __eq_exp, .. } => {
            (_, inconsistentUnits) = insertUnitInEquation(metamodelica::AsArg::as_arg(&__eq_exp), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U, htS2U, htU2S, fnCache);
            inconsistentUnits
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(inconsistentUnits)
}

fn makeNewCref(mut paramName: ArcStr, mut fnName: &ArcStr) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    outExp = metamodelica::Ref::new(Expression::NFExpression::CREF {
        ty: crate::NFType::interned_UNKNOWN(),
        cref: ComponentRef::prefixCref(
            metamodelica::Ref::new(InstNode::InstNode::NAME_NODE { name: paramName }),
            crate::NFType::interned_UNKNOWN(),
            metamodelica::nil(),
            ComponentRef::fromNode(
                metamodelica::Ref::new(InstNode::InstNode::NAME_NODE {
                    name: {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*fnName);
                        __mm_s.push_str(&*literal!("()"));
                        ArcStr::from(__mm_s)
                    },
                }),
                crate::NFType::interned_UNKNOWN(),
                metamodelica::nil(),
                ComponentRef::Origin::CREF.clone(),
            )?,
        )?,
    });
    Ok(outExp)
}

fn insertUnitInEquation(
    mut eq: &metamodelica::Ref<Expression::NFExpression>,
    mut unit: Unit::Unit,
    mut htCr2U: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
    mut htS2U: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>,
    mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
    mut fnCache: FunctionUnitCache,
) -> (
    Unit::Unit,
    metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>,
) {
    use crate::NFOperator::Op;
    let mut unit: Unit::Unit = unit;
    let mut inconsistentUnits: metamodelica::List<
        metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>,
    > = metamodelica::nil();
    (unit, inconsistentUnits) = 'mc: {
        let __mc_input = &**eq;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::MULTARY { .. } => {
                    Ok(insertUnitInEquation(&(SimplifyExp::splitMultary(eq.clone())?), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::SUB, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    unit2 = metamodelica::Own::own(__pa0);
                    icu2 = metamodelica::Own::own(__pa1);
                    (unit1, icu1) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), unit2.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    let (true, __pa2) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())?) else { return Err("pattern mismatch") };
                    op_unit = metamodelica::Own::own(__pa2);
                    Ok((op_unit.clone(), List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::SUB, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    (unit1, icu2) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    (unit2, icu1) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), unit1.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    let (true, __pa0) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())?) else { return Err("pattern mismatch") };
                    op_unit = metamodelica::Own::own(__pa0);
                    Ok((op_unit.clone(), List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::SUB, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    unit2 = metamodelica::Own::own(__pa0);
                    icu2 = metamodelica::Own::own(__pa1);
                    (unit1, icu1) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), unit2.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    let (false, _) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())?) else { return Err("pattern mismatch") };
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, metamodelica::cons(list![(exp1.clone(), unit1.clone()), (exp2.clone(), unit2.clone())], List::append_reverse(&icu1, icu2.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::SUB, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    (unit1, icu2) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    (unit2, icu1) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), unit1.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    let (false, _) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())?) else { return Err("pattern mismatch") };
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, metamodelica::cons(list![(exp1.clone(), unit1.clone()), (exp2.clone(), unit2.clone())], List::append_reverse(&icu1, icu2.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    unit2 = metamodelica::Own::own(__pa0);
                    icu2 = metamodelica::Own::own(__pa1);
                    (unit1, icu1) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), unit2.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    let (true, __pa2) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())?) else { return Err("pattern mismatch") };
                    op_unit = metamodelica::Own::own(__pa2);
                    Ok((op_unit.clone(), List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    (unit1, icu2) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    (unit2, icu1) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), unit1.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    let (true, __pa0) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())?) else { return Err("pattern mismatch") };
                    op_unit = metamodelica::Own::own(__pa0);
                    Ok((op_unit.clone(), List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    unit2 = metamodelica::Own::own(__pa0);
                    icu2 = metamodelica::Own::own(__pa1);
                    (unit1, icu1) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), unit2.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    let (false, _) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())?) else { return Err("pattern mismatch") };
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, metamodelica::cons(list![(exp1.clone(), unit1.clone()), (exp2.clone(), unit2.clone())], List::append_reverse(&icu1, icu2.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    (unit1, icu2) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    (unit2, icu1) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), unit1.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    let (false, _) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())?) else { return Err("pattern mismatch") };
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, metamodelica::cons(list![(exp1.clone(), unit1.clone()), (exp2.clone(), unit2.clone())], List::append_reverse(&icu1, icu2.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    unit1 = metamodelica::Own::own(__pa0);
                    icu1 = metamodelica::Own::own(__pa1);
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (__pa2 @ Unit::Unit::UNIT { .. }, __pa3) => (__pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    unit2 = metamodelica::Own::own(__pa2);
                    icu2 = metamodelica::Own::own(__pa3);
                    op_unit = Unit::unitMul(&unit1, &unit2)?;
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((op_unit.clone(), List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp2 } => {
                    if !((Unit::isMaster(&unit))) { return Err("guard") }
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let __pa0 = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::MASTER { .. }, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    icu1 = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::UNIT { .. }, __pa1) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    icu2 = metamodelica::Own::own(__pa1);
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp2 } => {
                    if !((Unit::isUnit(&unit))) { return Err("guard") }
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut vars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::MASTER { varList: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    vars = metamodelica::Own::own(__pa0);
                    icu1 = metamodelica::Own::own(__pa1);
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (__pa2 @ Unit::Unit::UNIT { .. }, __pa3) => (__pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    unit2 = metamodelica::Own::own(__pa2);
                    icu2 = metamodelica::Own::own(__pa3);
                    op_unit = Unit::unitDiv(&unit, &unit2)?;
                    List::map2_0(&vars, &updateHtCr2U, op_unit.clone(), htCr2U.clone())?;
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((unit.clone(), List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp2 } => {
                    if !((Unit::isMaster(&unit))) { return Err("guard") }
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let __pa0 = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::UNIT { .. }, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    icu1 = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::MASTER { .. }, __pa1) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    icu2 = metamodelica::Own::own(__pa1);
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp2 } => {
                    if !((Unit::isUnit(&unit))) { return Err("guard") }
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut vars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    unit2 = metamodelica::Own::own(__pa0);
                    icu1 = metamodelica::Own::own(__pa1);
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::MASTER { varList: __pa2 }, __pa3) => (__pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    vars = metamodelica::Own::own(__pa2);
                    icu2 = metamodelica::Own::own(__pa3);
                    op_unit = Unit::unitDiv(&unit, &unit2)?;
                    List::map2_0(&vars, &updateHtCr2U, op_unit.clone(), htCr2U.clone())?;
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((unit.clone(), List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp2 } => {
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let __pa0 = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::MASTER { .. }, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    icu1 = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::MASTER { .. }, __pa1) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    icu2 = metamodelica::Own::own(__pa1);
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    unit1 = metamodelica::Own::own(__pa0);
                    icu1 = metamodelica::Own::own(__pa1);
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (__pa2 @ Unit::Unit::UNIT { .. }, __pa3) => (__pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    unit2 = metamodelica::Own::own(__pa2);
                    icu2 = metamodelica::Own::own(__pa3);
                    op_unit = Unit::unitDiv(&unit1, &unit2)?;
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((op_unit.clone(), List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV, .. }, exp2 } => {
                    if !((Unit::isMaster(&unit))) { return Err("guard") }
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut inconsistentUnits: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>> = inconsistentUnits.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::MASTER { .. }, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    icu1 = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::UNIT { .. }, __pa1) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    icu2 = metamodelica::Own::own(__pa1);
                    inconsistentUnits = List::append_reverse(&icu1, icu2.clone());
                    Ok(((Unit::Unit::MASTER { varList: metamodelica::nil() }, List::append_reverse(&icu1, icu2.clone())), inconsistentUnits.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            inconsistentUnits = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV, .. }, exp2 } => {
                    if !((Unit::isUnit(&unit))) { return Err("guard") }
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut vars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::MASTER { varList: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    vars = metamodelica::Own::own(__pa0);
                    icu1 = metamodelica::Own::own(__pa1);
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (__pa2 @ Unit::Unit::UNIT { .. }, __pa3) => (__pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    unit2 = metamodelica::Own::own(__pa2);
                    icu2 = metamodelica::Own::own(__pa3);
                    op_unit = Unit::unitMul(&unit, &unit2)?;
                    List::map2_0(&vars, &updateHtCr2U, op_unit.clone(), htCr2U.clone())?;
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((unit.clone(), List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV, .. }, exp2 } => {
                    if !((Unit::isMaster(&unit))) { return Err("guard") }
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let __pa0 = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::UNIT { .. }, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    icu1 = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::MASTER { .. }, __pa1) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    icu2 = metamodelica::Own::own(__pa1);
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV, .. }, exp2 } => {
                    if !((Unit::isUnit(&unit))) { return Err("guard") }
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut vars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    unit2 = metamodelica::Own::own(__pa0);
                    icu1 = metamodelica::Own::own(__pa1);
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::MASTER { varList: __pa2 }, __pa3) => (__pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    vars = metamodelica::Own::own(__pa2);
                    icu2 = metamodelica::Own::own(__pa3);
                    op_unit = Unit::unitDiv(&unit2, &unit)?;
                    List::map2_0(&vars, &updateHtCr2U, op_unit.clone(), htCr2U.clone())?;
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((unit.clone(), List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV, .. }, exp2 } => {
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let __pa0 = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::MASTER { .. }, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    icu1 = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::MASTER { .. }, __pa1) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    icu2 = metamodelica::Own::own(__pa1);
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, List::append_reverse(&icu1, icu2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::POW, .. }, exp2: exp2 @ Deref @ Expression::REAL { .. } } => {
                    let mut unit1: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut i: i32;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    unit1 = metamodelica::Own::own(__pa0);
                    icu1 = metamodelica::Own::own(__pa1);
                    i = ((var_field!((**exp2).value, Expression::NFExpression::REAL).clone()).0.floor() as i32);
                    let true = (realEq(var_field!((**exp2).value, Expression::NFExpression::REAL).clone(), metamodelica::OrderedFloat((i) as f64))) else { return Err("pattern mismatch") };
                    op_unit = Unit::unitPow(&unit1, i)?;
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((op_unit.clone(), icu1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::POW, .. }, exp2: exp2 @ Deref @ Expression::REAL { .. } } => {
                    if !((Unit::isUnit(&unit))) { return Err("guard") }
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut vars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())) {
                        (Unit::Unit::MASTER { varList: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    vars = metamodelica::Own::own(__pa0);
                    icu1 = metamodelica::Own::own(__pa1);
                    op_unit = Unit::unitRoot(&unit, var_field!((**exp2).value, Expression::NFExpression::REAL).clone())?;
                    List::map2_0(&vars, &updateHtCr2U, op_unit.clone(), htCr2U.clone())?;
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((unit.clone(), icu1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::POW, .. }, exp2: Deref @ Expression::REAL { .. } } => {
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    (_, icu1) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, icu1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::CALL { .. } => {
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    (op_unit, icu1) = insertUnitInEquationCall(var_field!((**eq).call, Expression::NFExpression::CALL), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    Ok((op_unit.clone(), icu1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::IF { .. } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut b: bool;
                    let mut inconsistentUnits: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>> = inconsistentUnits.clone();
                    (unit1, icu1) = insertUnitInEquation(var_field!((**eq).trueBranch, Expression::NFExpression::IF), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    (unit2, icu2) = insertUnitInEquation(var_field!((**eq).falseBranch, Expression::NFExpression::IF), unit1.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    (b, op_unit) = unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())?;
                    inconsistentUnits = List::append_reverse(&icu1, icu2.clone());
                    if !(b) {
                        inconsistentUnits = metamodelica::cons(list![(var_field!((**eq).trueBranch, Expression::NFExpression::IF).clone(), unit1.clone()), (var_field!((**eq).falseBranch, Expression::NFExpression::IF).clone(), unit2.clone())], inconsistentUnits.clone());
                        op_unit = Unit::Unit::MASTER { varList: metamodelica::nil() };
                    }
                    Ok(((op_unit.clone(), inconsistentUnits.clone()), inconsistentUnits.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            inconsistentUnits = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::RELATION { .. } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut icu2: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    let mut b: bool;
                    let mut inconsistentUnits: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>> = inconsistentUnits.clone();
                    (unit1, icu1) = insertUnitInEquation(var_field!((**eq).exp1, Expression::NFExpression::RELATION), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    (unit2, icu2) = insertUnitInEquation(var_field!((**eq).exp2, Expression::NFExpression::RELATION), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    (b, op_unit) = unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())?;
                    inconsistentUnits = List::append_reverse(&icu1, icu2.clone());
                    if !(b) {
                        inconsistentUnits = metamodelica::cons(list![(var_field!((**eq).exp1, Expression::NFExpression::RELATION).clone(), unit1.clone()), (var_field!((**eq).exp2, Expression::NFExpression::RELATION).clone(), unit2.clone())], inconsistentUnits.clone());
                        op_unit = Unit::Unit::MASTER { varList: metamodelica::nil() };
                    }
                    Ok(((op_unit.clone(), inconsistentUnits.clone()), inconsistentUnits.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            inconsistentUnits = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::UNARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::UMINUS, .. }, .. } => {
                    let mut op_unit: Unit::Unit;
                    let mut icu1: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
                    (op_unit, icu1) = insertUnitInEquation(var_field!((**eq).exp, Expression::NFExpression::UNARY), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    Ok((op_unit.clone(), icu1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::CREF { .. } => {
                    if !((ComponentRef::isTime(var_field!((**eq).cref, Expression::NFExpression::CREF))?)) { return Err("guard") }
                    let mut op_unit: Unit::Unit;
                    op_unit = Unit::SECOND().clone();
                    addUnit2HtS2U(literal!("time"), op_unit.clone(), htS2U.clone())?;
                    addUnit2HtU2S(literal!("time"), op_unit.clone(), htU2S.clone())?;
                    Ok((op_unit.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::CREF { ty: Deref @ Type::REAL, .. } => {
                    Ok((UnorderedMap::getOrFail((ComponentRef::stripSubscripts(var_field!((**eq).cref, Expression::NFExpression::CREF).clone())).0, htCr2U.clone())?, metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (unit, inconsistentUnits)
}

fn insertUnitInEquationCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut unit: Unit::Unit,
    mut htCr2U: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
    mut htS2U: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>,
    mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
    mut fnCache: FunctionUnitCache,
) -> Result<(
    Unit::Unit,
    metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>,
)> {
    let mut unit: Unit::Unit = unit;
    let mut inconsistentUnits: metamodelica::List<
        metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>,
    > = metamodelica::nil();
    let mut fn_path: metamodelica::Ref<Absyn::Path>;
    let mut fn_name: ArcStr = arcstr::literal!("");
    let mut call_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut op_unit: Unit::Unit = <Unit::Unit as ::std::default::Default>::default();
    let mut vars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut var_names: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut unit_names: metamodelica::List<ArcStr> = metamodelica::nil();
    fn_path = Call::functionName(call)?;
    call_args = Call::arguments(call)?;
    (unit, inconsistentUnits) = 'mc: {
        let __mc_input = &*fn_path;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { name: Deref @ "pre" } => {
                    let mut inconsistentUnits: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>> = inconsistentUnits.clone();
                    let mut op_unit: Unit::Unit = op_unit.clone();
                    (op_unit, inconsistentUnits) = insertUnitInEquation(&((call_args).head().cloned()?), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    Ok(((Unit::Unit::MASTER { varList: metamodelica::nil() }, inconsistentUnits.clone()), inconsistentUnits.clone(), op_unit.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            inconsistentUnits = __wb0;
            op_unit = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { name: Deref @ "der" } => {
                    let mut inconsistentUnits: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>> = inconsistentUnits.clone();
                    let mut op_unit: Unit::Unit = op_unit.clone();
                    let mut vars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = vars.clone();
                    (op_unit, inconsistentUnits) = insertUnitInEquation(&((call_args).head().cloned()?), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    if Unit::isUnit(&op_unit) {
                        op_unit = Unit::unitDiv(&op_unit, &(Unit::SECOND().clone()))?;
                        insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    } else if Unit::isUnit(&unit) {
                        let Unit::MASTER { varList: __pa0 } = (op_unit.clone()) else { return Err("pattern mismatch") };
                        vars = metamodelica::Own::own(__pa0);
                        op_unit = Unit::unitMul(&unit, &(Unit::SECOND().clone()))?;
                        List::map2_0(&vars, &updateHtCr2U, op_unit.clone(), htCr2U.clone())?;
                        insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    } else {
                        op_unit = Unit::Unit::MASTER { varList: metamodelica::nil() };
                    }
                    Ok(((op_unit.clone(), inconsistentUnits.clone()), inconsistentUnits.clone(), op_unit.clone(), vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            inconsistentUnits = __wb0;
            op_unit = __wb1;
            vars = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" } => {
                    let mut inconsistentUnits: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>> = inconsistentUnits.clone();
                    let mut op_unit: Unit::Unit = op_unit.clone();
                    let mut vars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = vars.clone();
                    (op_unit, inconsistentUnits) = insertUnitInEquation(&((call_args).head().cloned()?), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    if Unit::isUnit(&op_unit) {
                        op_unit = Unit::unitRoot(&op_unit, metamodelica::OrderedFloat(2.0_f64))?;
                        insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    } else if Unit::isUnit(&unit) {
                        let Unit::MASTER { varList: __pa0 } = (op_unit.clone()) else { return Err("pattern mismatch") };
                        vars = metamodelica::Own::own(__pa0);
                        op_unit = Unit::unitPow(&unit, 2)?;
                        List::map2_0(&vars, &updateHtCr2U, op_unit.clone(), htCr2U.clone())?;
                        insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                        op_unit = unit.clone();
                    } else {
                        op_unit = Unit::Unit::MASTER { varList: metamodelica::nil() };
                    }
                    Ok(((op_unit.clone(), inconsistentUnits.clone()), inconsistentUnits.clone(), op_unit.clone(), vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            inconsistentUnits = __wb0;
            op_unit = __wb1;
            vars = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { .. } => {
                    if !((Function::isBuiltin(&(Call::typedFunction(call)?)))) { return Err("guard") }
                    let mut inconsistentUnits: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>> = inconsistentUnits.clone();
                    inconsistentUnits = foldCallArg(&call_args, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone());
                    Ok(((Unit::Unit::MASTER { varList: metamodelica::nil() }, inconsistentUnits.clone()), inconsistentUnits.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            inconsistentUnits = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut fn_name: ArcStr = fn_name.clone();
                    let mut inconsistentUnits: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>> = inconsistentUnits.clone();
                    let mut unit_names: metamodelica::List<ArcStr> = unit_names.clone();
                    let mut var_names: metamodelica::List<ArcStr> = var_names.clone();
                    fn_name = AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(fn_path.clone()), literal!("."), true, false)?;
                    (var_names, _, unit_names, _) = getCallUnits(fn_name.clone(), call, fnCache.clone())?;
                    inconsistentUnits = foldCallArg1(&call_args, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone(), unit.clone(), unit_names.clone(), var_names.clone(), &fn_name)?;
                    Ok(((Unit::Unit::MASTER { varList: metamodelica::nil() }, inconsistentUnits.clone()), fn_name.clone(), inconsistentUnits.clone(), unit_names.clone(), var_names.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            fn_name = __wb0;
            inconsistentUnits = __wb1;
            unit_names = __wb2;
            var_names = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((unit, inconsistentUnits))
}

fn insertUnitString(
    mut unit: Unit::Unit,
    mut htS2U: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>,
    mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
) -> Result<()> {
    let mut unit_str: ArcStr;
    unit_str = Unit::unitString(unit.clone(), htU2S.clone())?;
    addUnit2HtS2U(unit_str.clone(), unit.clone(), htS2U)?;
    addUnit2HtU2S(unit_str, unit, htU2S)?;
    Ok(())
}

fn getCallUnits(
    mut fnName: ArcStr,
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut fnCache: FunctionUnitCache,
) -> Result<(
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
)> {
    let mut inputVars: metamodelica::List<ArcStr>;
    let mut outputVars: metamodelica::List<ArcStr>;
    let mut inputUnits: metamodelica::List<ArcStr>;
    let mut outputUnits: metamodelica::List<ArcStr>;
    let mut opt_args: Option<Functionargs>;
    let mut args: Functionargs;
    opt_args = UnorderedMap::get(fnName.clone(), fnCache.clone())?;
    if (opt_args).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(opt_args) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        args = metamodelica::Own::own(__pa0);
    } else {
        args = parseFunctionUnits(fnName.clone(), &(Call::typedFunction(call)?))?;
        UnorderedMap::addUnique(fnName, args.clone(), fnCache)?;
    }
    let Functionargs {
        name: _,
        invars: __pa1,
        outvars: __pa2,
        inunits: __pa3,
        outunits: __pa4,
    } = args;
    inputVars = metamodelica::Own::own(__pa1);
    outputVars = metamodelica::Own::own(__pa2);
    inputUnits = metamodelica::Own::own(__pa3);
    outputUnits = metamodelica::Own::own(__pa4);
    Ok((inputVars, outputVars, inputUnits, outputUnits))
}

fn parseFunctionUnits(mut funcName: ArcStr, mut func: &metamodelica::Ref<Function::Function>) -> Result<Functionargs> {
    let mut outArgs: Functionargs;
    let mut in_units: metamodelica::List<ArcStr>;
    let mut out_units: metamodelica::List<ArcStr>;
    let mut in_args: metamodelica::List<ArcStr>;
    let mut out_args: metamodelica::List<ArcStr>;
    in_units = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut p in (func.inputs.clone()).into_iter().cloned() {
            let __x = Component::getUnitAttribute(&(InstNode::component(&(p.clone()))?), literal!("NONE"))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    out_units = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut p in (func.outputs.clone()).into_iter().cloned() {
            let __x = Component::getUnitAttribute(
                &(InstNode::component(&(InstNode::fromHandle(&(p.clone()))?))?),
                literal!("NONE"),
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    in_args = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut p in (func.inputs.clone()).into_iter().cloned() {
            let __x = InstNode::name(&(p.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    out_args = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut p in (func.outputs.clone()).into_iter().cloned() {
            let __x = InstNode::name(&(InstNode::fromHandle(&(p.clone()))?))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outArgs = Functionargs {
        name: funcName,
        invars: in_args,
        outvars: out_args,
        inunits: in_units,
        outunits: out_units,
    };
    Ok(outArgs)
}

fn unitTypesEqual(
    mut unit1: Unit::Unit,
    mut unit2: Unit::Unit,
    mut htCr2U: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
) -> Result<(bool, Unit::Unit)> {
    let mut isEqual: bool;
    let mut outUnit: Unit::Unit;
    (isEqual, outUnit) = (match (unit1.clone(), unit2.clone()) {
        (Unit::Unit::UNIT { .. }, Unit::Unit::UNIT { .. }) => (Unit::isEqual(&unit1, &unit2)?, unit1),
        (Unit::Unit::UNIT { .. }, Unit::Unit::MASTER { varList: ref vars2 }) => {
            List::map2_0(
                metamodelica::AsArg::as_arg(&vars2),
                &updateHtCr2U,
                unit1.clone(),
                htCr2U,
            )?;
            (true, unit1)
        }
        (Unit::Unit::MASTER { varList: ref vars1 }, Unit::Unit::UNIT { .. }) => {
            List::map2_0(
                metamodelica::AsArg::as_arg(&vars1),
                &updateHtCr2U,
                unit2.clone(),
                htCr2U,
            )?;
            (true, unit2)
        }
        (Unit::Unit::MASTER { varList: ref vars1 }, Unit::Unit::MASTER { varList: ref vars2 }) => (
            true,
            Unit::Unit::MASTER {
                varList: List::append_reverse(metamodelica::AsArg::as_arg(&vars1), vars2.clone()),
            },
        ),
        (Unit::Unit::UNKNOWN { unit: mut s1 }, Unit::Unit::UNKNOWN { unit: mut s2 }) => {
            (metamodelica::stringEq(&s1, &s2), unit1)
        }
        (Unit::Unit::UNKNOWN { .. }, _) => (true, unit1),
        (_, Unit::Unit::UNKNOWN { .. }) => (true, unit2),
        _ => (false, unit1),
    });
    Ok((isEqual, outUnit))
}

fn updateHtCr2U(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut unit: Unit::Unit,
    mut htCr2U: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
) -> Result<()> {
    UnorderedMap::tryAdd(
        Unit::UPDATECREF().clone(),
        Unit::Unit::MASTER {
            varList: metamodelica::nil(),
        },
        htCr2U.clone(),
    )?;
    UnorderedMap::add(cref, unit, htCr2U)?;
    Ok(())
}

fn Errorfunction(
    mut inexpList: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>,
    mut inEq: &metamodelica::Ref<Equation::NFEquation>,
    mut inHtU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inexpList) {
        expList => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut info: SourceInfo;
            info = Equation::info(inEq);
            s = Equation::toString(inEq, literal!(""))?;
            s1 = Errorfunction2(metamodelica::AsArg::as_arg(&expList), inHtU2S)?;
            s2 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following equation is INCONSISTENT due to specified unit information: ")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
            Error::addSourceMessage(&(Error::COMPILER_WARNING.clone()), list![s2], &info)?;
            Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The units of following sub-expressions need to be equal:\n")); __mm_s.push_str(&*s1); ArcStr::from(__mm_s) })?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn Errorfunction2(
    mut inexpList: &metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>,
    mut inHtU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
) -> Result<ArcStr> {
    let mut outS: ArcStr;
    outS = (::match_deref::match_deref! { match inexpList {
        Deref @ metamodelica::ListNode::Cons { head: (exp, ut), tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            s = Expression::toString(exp.clone())?;
            s1 = Unit::unitString(ut.clone(), inHtU2S)?;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- sub-expression \"")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("\" has unit \"")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) };
            s
        },
        Deref @ metamodelica::ListNode::Cons { head: (exp, ut), tail: expList } => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s = Expression::toString(exp.clone())?;
            s1 = Unit::unitString(ut.clone(), inHtU2S.clone())?;
            s2 = Errorfunction2(expList, inHtU2S)?;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- sub-expression \"")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("\" has unit \"")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!("\"\n")); __mm_s.push_str(&*s2); ArcStr::from(__mm_s) };
            s
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outS)
}

fn foldCallArg(
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut htCr2U: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
    mut htS2U: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>,
    mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
    mut fnCache: FunctionUnitCache,
) -> metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>> {
    let mut inconsistentUnits: metamodelica::List<
        metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>,
    > = metamodelica::nil();
    let mut icu: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
    for mut exp in &**args {
        (_, icu) = insertUnitInEquation(
            metamodelica::AsArg::as_arg(&exp),
            Unit::Unit::MASTER {
                varList: metamodelica::nil(),
            },
            htCr2U.clone(),
            htS2U.clone(),
            htU2S.clone(),
            fnCache.clone(),
        );
        inconsistentUnits = List::append_reverse(&icu, inconsistentUnits);
    }
    inconsistentUnits = inconsistentUnits.reverse();
    inconsistentUnits
}

fn foldCallArg1(
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut htCr2U: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
    mut htS2U: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>,
    mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
    mut fnCache: FunctionUnitCache,
    mut inUnit: Unit::Unit,
    mut units: metamodelica::List<ArcStr>,
    mut vars: metamodelica::List<ArcStr>,
    mut fnName: &ArcStr,
) -> Result<metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>> {
    let mut inconsistentUnits: metamodelica::List<
        metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>,
    > = metamodelica::nil();
    let mut unit: ArcStr;
    let mut var: ArcStr;
    let mut rest_units: metamodelica::List<ArcStr> = units;
    let mut rest_vars: metamodelica::List<ArcStr> = vars;
    let mut op_unit: Unit::Unit;
    let mut op_unit2: Unit::Unit;
    let mut icu: metamodelica::List<metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, Unit::Unit)>>;
    let mut temp: metamodelica::Ref<Expression::NFExpression>;
    let mut b: bool;
    for mut arg in &**args {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_vars) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        var = metamodelica::Own::own(__pa0);
        rest_vars = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_units) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        unit = metamodelica::Own::own(__pa2);
        rest_units = metamodelica::Own::own(__pa3);
        (op_unit, icu) = insertUnitInEquation(
            metamodelica::AsArg::as_arg(&arg),
            inUnit.clone(),
            htCr2U.clone(),
            htS2U.clone(),
            htU2S.clone(),
            fnCache.clone(),
        );
        if metamodelica::stringEq(&unit, &(literal!("NONE"))) {
            op_unit2 = Unit::Unit::MASTER {
                varList: metamodelica::nil(),
            };
        } else {
            op_unit2 = Unit::parseUnitString(unit, htS2U.clone(), &(Absyn::dummyInfo.clone()))?;
        }
        (b, op_unit) = unitTypesEqual(op_unit, op_unit2.clone(), htCr2U.clone())?;
        if b {
            icu = metamodelica::nil();
        } else {
            temp = makeNewCref(var, fnName)?;
            icu = list![list![(arg.clone(), op_unit), (temp, op_unit2)]];
        }
        inconsistentUnits = List::append_reverse(&icu, inconsistentUnits);
    }
    Ok(inconsistentUnits)
}

fn addUnit2HtS2U(
    mut name: ArcStr,
    mut unit: Unit::Unit,
    mut inHtS2U: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>,
) -> Result<()> {
    UnorderedMap::add(name, unit, inHtS2U)?;
    Ok(())
}

fn addUnit2HtU2S(
    mut name: ArcStr,
    mut unit: Unit::Unit,
    mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
) -> Result<()> {
    UnorderedMap::tryAdd(unit, name, htU2S)?;
    Ok(())
}

fn convertUnitStringToUnit(
    mut var: &metamodelica::Ref<Variable::NFVariable>,
    mut htCr2U: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, Unit::Unit>,
    >,
    mut htS2U: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>,
    mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
) -> Result<()> {
    let mut unit_binding: metamodelica::Ref<Binding::NFBinding>;
    let mut unit_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut unit_string: ArcStr;
    let mut unit: Unit::Unit;
    unit_binding = Variable::lookupTypeAttribute(&(literal!("unit")), var);
    unit_exp = Binding::typedExp(&unit_binding);
    unit_string = if ((unit_exp).is_some()) {
        getUnitStringFromExp(Util::getOption(unit_exp)?)?
    } else {
        literal!("")
    };
    if stringEmpty(&unit_string) {
        UnorderedMap::add(
            var.name.clone(),
            Unit::Unit::MASTER {
                varList: list![var.name.clone()],
            },
            htCr2U,
        )?;
        addUnit2HtS2U(
            literal!("-"),
            Unit::Unit::MASTER {
                varList: list![var.name.clone()],
            },
            htS2U,
        )?;
        addUnit2HtU2S(
            literal!("-"),
            Unit::Unit::MASTER {
                varList: list![var.name.clone()],
            },
            htU2S,
        )?;
    } else {
        unit = parse(unit_string, var.name.clone(), htS2U, htU2S, &var.info)?;
        UnorderedMap::add(var.name.clone(), unit, htCr2U)?;
    }
    Ok(())
}

fn getUnitStringFromExp(mut unitExp: metamodelica::Ref<Expression::NFExpression>) -> Result<ArcStr> {
    '__tco: loop {
        let mut exp: metamodelica::Ref<Expression::NFExpression>;
        ::match_deref::match_deref! { match &(&*unitExp) {
            Deref @ Expression::STRING { value: __unitExp_value } => return Ok(__unitExp_value.clone()),
            Deref @ Expression::ARRAY { literal: true, .. } if (Expression::isLiteral(&unitExp)? && !(Type::isEmptyArray(&(Expression::typeOf(unitExp.clone())))?)) => { unitExp = Expression::arrayFirstScalar(unitExp.clone())?; continue '__tco; },
            Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: __esc_exp, tail: _ }, .. } } if (Call::isNamed(var_field!((*unitExp).call, Expression::NFExpression::CALL), &(literal!("fill")))?) => {
                exp = (*__esc_exp).clone();
                { unitExp = exp.clone(); continue '__tco; }
            },
            _ if (!(Expression::isLiteral(&unitExp)?)) => {
                exp = Ceval::tryEvalExp(unitExp.clone(), &(Ceval::noTarget().clone()));
                if (Expression::isLiteral(&exp)?) {{ unitExp = exp; continue '__tco; }} else {return Ok(literal!(""))}
            },
            _ => return Ok(literal!("")),
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn parse(
    mut unitString: ArcStr,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut htS2U: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>,
    mut htU2S: metamodelica::Ref<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>,
    mut info: &SourceInfo,
) -> Result<Unit::Unit> {
    let mut unit: Unit::Unit;
    if stringEmpty(&unitString) {
        unit = Unit::Unit::MASTER { varList: list![cref] };
        return Ok(unit);
    }
    match '__try0: {
        unit = unwrap_break_err!(UnorderedMap::getOrFail(unitString.clone(), htS2U.clone()), '__try0);
        Ok::<_, &'static str>((unit.clone(),))
    } {
        Ok((__try0_o0,)) => {
            unit = __try0_o0;
        }
        Err(_) => {
            match '__try1: {
                unit = unwrap_break_err!(Unit::parseUnitString(unitString.clone(), htS2U.clone(), info), '__try1);
                Ok::<_, &'static str>((unit.clone(),))
            } {
                Ok((__try1_o0,)) => {
                    unit = __try1_o0;
                }
                Err(_) => {
                    unit = Unit::Unit::UNKNOWN {
                        unit: unitString.clone(),
                    };
                }
            }
            addUnit2HtS2U(unitString.clone(), unit.clone(), htS2U.clone())?;
            addUnit2HtU2S(unitString.clone(), unit.clone(), htU2S.clone())?;
        }
    }
    Ok(unit)
}
