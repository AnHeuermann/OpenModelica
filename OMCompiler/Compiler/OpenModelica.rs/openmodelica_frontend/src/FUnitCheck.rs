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

use crate::FHashTableCrToUnit as HashTableCrToUnit;
use crate::FHashTableStringToUnit as HashTableStringToUnit;
use crate::FHashTableUnitToString as HashTableUnitToString;
use crate::FUnit as Unit;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

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
pub type FUNCTIONUNITS = Functionargs;

pub(crate) fn checkUnits(
    mut inDAE: DAE::DAElist,
    mut func: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<DAE::DAElist> {
    let mut outDAE: DAE::DAElist = inDAE.clone();
    let mut elts1: DAE::DAElist;
    let mut elts2: DAE::DAElist;
    let mut eqlist: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut varlist: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut newdaelist: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut functionlist: metamodelica::List<DAE::Function>;
    let mut args: metamodelica::List<Functionargs>;
    let mut HtCr2U1: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            HashTableCrToUnit::FuncHashKey,
            HashTableCrToUnit::FuncKeyEqual,
            HashTableCrToUnit::FuncKeyStr,
            HashTableCrToUnit::FuncValueStr,
        ),
    );
    let mut HtCr2U2: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            HashTableCrToUnit::FuncHashKey,
            HashTableCrToUnit::FuncKeyEqual,
            HashTableCrToUnit::FuncKeyStr,
            HashTableCrToUnit::FuncValueStr,
        ),
    );
    let mut HtS2U: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            HashTableStringToUnit::FuncHashKey,
            HashTableStringToUnit::FuncKeyEqual,
            HashTableStringToUnit::FuncKeyStr,
            HashTableStringToUnit::FuncValueStr,
        ),
    );
    let mut HtU2S: (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            HashTableUnitToString::FuncHashKey,
            HashTableUnitToString::FuncKeyEqual,
            HashTableUnitToString::FuncKeyStr,
            HashTableUnitToString::FuncValueStr,
        ),
    );
    if !(Flags::getConfigBool(Flags::UNIT_CHECKING.clone())?
        || Flags::getConfigBool(Flags::CHECK_MODEL.clone())? && Flags::isSet(Flags::SCODE_INST.clone())?)
    {
        return Ok(outDAE);
    }
    if '__try0: {
        (elts1, elts2) = unwrap_break_err!(DAEUtil::splitDAEIntoVarsAndEquations(inDAE.clone()), '__try0);
        varlist = GetVarList(&elts1);
        eqlist = GetElementList(&elts2);
        functionlist = unwrap_break_err!(DAEUtil::getFunctionList(func, false), '__try0);
        HtCr2U1 = HashTableCrToUnit::emptyHashTableSized(Util::nextPrime(((metamodelica::OrderedFloat((10) as f64) + metamodelica::OrderedFloat(1.4_f64) * metamodelica::OrderedFloat((((varlist).len() as i32)) as f64)).0.floor() as i32)));
        HtS2U = unwrap_break_err!(Unit::getKnownUnits(), '__try0);
        HtU2S = unwrap_break_err!(Unit::getKnownUnitsInverse(), '__try0);
        args = list![Functionargs { name: literal!(""), invars: metamodelica::nil(), outvars: metamodelica::nil(), inunits: metamodelica::nil(), outunits: metamodelica::nil() }];
        args = unwrap_break_err!(List::mapFlat(&functionlist, &move |__a0: DAE::Function| parseFunctionList(&__a0)), '__try0);
        (HtCr2U1, HtS2U, HtU2S) = unwrap_break_err!(List::fold(&varlist, &move |__a0: metamodelica::Ref<DAE::Element>, __a1: ((metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>)), (metamodelica::Array<metamodelica::List<(ArcStr, i32)>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>), i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>)), (metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>)))| convertUnitString2unit_old(&__a0, __a1), (HtCr2U1.clone(), HtS2U.clone(), HtU2S.clone())), '__try0);
        HtCr2U2 = BaseHashTable::copy(&HtCr2U1);
        (HtCr2U2, HtS2U, HtU2S) = unwrap_break_err!(algo(&varlist, &eqlist, args.clone(), HtCr2U2.clone(), HtS2U.clone(), HtU2S.clone()), '__try0);
        varlist = unwrap_break_err!(List::map2(varlist.clone(), &move |__a0: metamodelica::Ref<DAE::Element>, __a1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>)), __a2: (metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>, (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>), i32, (Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>))| returnVar(__a0, &__a1, &__a2), HtCr2U2.clone(), HtU2S.clone()), '__try0);
        newdaelist = listAppend(varlist.clone(), eqlist.clone());
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_UNIT.clone()), '__try0) {
            unwrap_break_err!(BaseHashTable::dumpHashTable(&HtCr2U2), '__try0);
            metamodelica::print(literal!("######## UnitCheck COMPLETED ########\n"));
        }
        unwrap_break_err!(notification(&HtCr2U1, &HtCr2U2, &HtU2S), '__try0);
        outDAE = unwrap_break_err!(updateDAElist(&inDAE, newdaelist.clone()), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
        Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FUnitCheck.checkUnits")); __mm_s.push_str(&*literal!(": unit check module failed")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/FUnitCheck.mo"))?;
    }
    execStat(&(literal!("FUnitCheck.checkUnits")))?;
    Ok(outDAE)
}

fn parseFunctionList(mut infunction: &DAE::Function) -> Result<metamodelica::List<Functionargs>> {
    let mut outTpl: metamodelica::List<Functionargs>;
    let mut inelt: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut outelt: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut inunits: metamodelica::List<ArcStr>;
    let mut outunits: metamodelica::List<ArcStr>;
    let mut inargs: metamodelica::List<ArcStr>;
    let mut outargs: metamodelica::List<ArcStr>;
    let mut s: ArcStr;
    s = getFunctionName(infunction)?;
    inelt = DAEUtil::getFunctionInputVars(infunction)?;
    outelt = DAEUtil::getFunctionOutputVars(infunction)?;
    inunits = List::filterMap(
        &inelt,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(getUnits(&__a0))
        },
    );
    outunits = List::filterMap(
        &outelt,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(getUnits(&__a0))
        },
    );
    inargs = List::filterMap(&inelt, &move |__a0: metamodelica::Ref<DAE::Element>| getVars(&__a0));
    outargs = List::filterMap(&outelt, &move |__a0: metamodelica::Ref<DAE::Element>| getVars(&__a0));
    outTpl = list![Functionargs {
        name: s,
        invars: inargs,
        outvars: outargs,
        inunits: inunits,
        outunits: outunits
    }];
    Ok(outTpl)
}

pub(crate) fn getFunctionName(mut inFunction: &DAE::Function) -> Result<ArcStr> {
    let mut outString: ArcStr = AbsynUtil::pathString(
        AbsynUtil::makeNotFullyQualified(DAEUtil::functionName(inFunction)),
        literal!("."),
        true,
        false,
    )?;
    Ok(outString)
}

pub(crate) fn getVars(mut inElement: &metamodelica::Ref<DAE::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inElement {
        DAE::Element::VAR { componentRef: cr, .. } => ComponentReference::crefStr(cr)?,
        _ => {
            literal!("")
        }
    });
    Ok(outString)
}

pub(crate) fn getUnits(mut inElement: &metamodelica::Ref<DAE::Element>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inElement {
        Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_REAL { .. }, variableAttributesOption: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { unit: Some(Deref @ DAE::Exp::SCONST { string: unitString }), .. }), .. } if (!metamodelica::stringEq(&unitString, &(literal!("")))) => {
            unitString.clone()
        },
        _ => {
            literal!("NONE")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outString
}

fn updateDAElist(
    mut indaelist: &DAE::DAElist,
    mut indaevarlist: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<DAE::DAElist> {
    let mut outdaelist: DAE::DAElist;
    outdaelist = (::match_deref::match_deref! { match &(indaelist) {
        DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMP { ident, source: eltsrc, comment, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut varlist2 = indaevarlist;
            let mut outdae: DAE::DAElist;
            outdae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::COMP { ident: ident.clone(), dAElist: varlist2, source: eltsrc.clone(), comment: comment.clone() })] };
            outdae
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outdaelist)
}

fn returnVar(
    mut inVar: metamodelica::Ref<DAE::Element>,
    mut inHtCr2U: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHtU2S: &(
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outVar: metamodelica::Ref<DAE::Element>;
    outVar = (::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Element::VAR { variableAttributesOption: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { unit: Some(_), .. }), .. } => {
            inVar
        },
        Deref @ DAE::Element::VAR { componentRef: cr, variableAttributesOption: attr, .. } => {
            let mut var: metamodelica::Ref<DAE::Element>;
            let mut ut: Unit::Unit;
            let mut s: ArcStr;
            let mut attr = (*attr).clone();
            if BaseHashTable::hasKey(cr.clone(), inHtCr2U)? {
                ut = BaseHashTable::get(cr.clone(), inHtCr2U)?;
                if Unit::isUnit(&ut) {
                    s = Unit::unitString(ut, inHtU2S)?;
                    attr = DAEUtil::setUnitAttr(attr.clone(), metamodelica::Ref::new(DAE::Exp::SCONST { string: s }))?;
                    assign_variant_field!(inVar => DAE::Element::VAR; variableAttributesOption = attr.clone());
                    var = inVar;
                } else {
                    var = inVar;
                }
            } else {
                var = inVar;
            }
            var
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outVar)
}

fn notification(
    mut inHtCr2U1: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHtCr2U2: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHtU2S: &(
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<()> {
    let mut r#str: ArcStr;
    let mut lt1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>;
    lt1 = BaseHashTable::hashTableList(inHtCr2U1)?;
    r#str = notification2(lt1, inHtCr2U2, inHtU2S)?;
    if Flags::isSet(Flags::DUMP_UNIT.clone())? && !metamodelica::stringEq(&r#str, &(literal!(""))) {
        Error::addCompilerNotification(r#str)?;
    }
    Ok(())
}

fn notification2(
    mut inLt1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>,
    mut inHtCr2U2: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHtU2S: &(
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<ArcStr> {
    let mut outS: ArcStr;
    let mut cr1: metamodelica::Ref<DAE::ComponentRef> = DAE::emptyCref().clone();
    let mut factor1: metamodelica::Real = metamodelica::OrderedFloat((0) as f64);
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut i3: i32 = 0;
    let mut i4: i32 = 0;
    let mut i5: i32 = 0;
    let mut i6: i32 = 0;
    let mut i7: i32 = 0;
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
                            let Unit::UNIT { factor: __pa1, mol: __pa2, cd: __pa3, m: __pa4, s: __pa5, A: __pa6, K: __pa7, g: __pa8 } = (unwrap_break_err!(BaseHashTable::get(cr1.clone(), inHtCr2U2), '__try0)) else { break '__try0 Err::<_, _>("pattern mismatch") };
                            factor1 = metamodelica::Own::own(__pa1);
                            i1 = metamodelica::Own::own(__pa2);
                            i2 = metamodelica::Own::own(__pa3);
                            i3 = metamodelica::Own::own(__pa4);
                            i4 = metamodelica::Own::own(__pa5);
                            i5 = metamodelica::Own::own(__pa6);
                            i6 = metamodelica::Own::own(__pa7);
                            i7 = metamodelica::Own::own(__pa8);
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
                    __mm_s.push_str(&*ComponentReference::crefStr(&cr1)?);
                    __mm_s.push_str(&*literal!("\" has the Unit \""));
                    __mm_s.push_str(&*Unit::unitString(
                        Unit::Unit::UNIT {
                            factor: factor1,
                            mol: i1,
                            cd: i2,
                            m: i3,
                            s: i4,
                            A: i5,
                            K: i6,
                            g: i7,
                        },
                        inHtU2S,
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

pub(crate) fn algo(
    mut invarlist: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut ineqList: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inargs: metamodelica::List<Functionargs>,
    mut inHtCr2U: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHtS2U: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHtU2S: (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outTpl: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
            ),
            i32,
            (
                HashTableCrToUnit::FuncHashKey,
                HashTableCrToUnit::FuncKeyEqual,
                HashTableCrToUnit::FuncKeyStr,
                HashTableCrToUnit::FuncValueStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
            (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
            i32,
            (
                HashTableStringToUnit::FuncHashKey,
                HashTableStringToUnit::FuncKeyEqual,
                HashTableStringToUnit::FuncKeyStr,
                HashTableStringToUnit::FuncValueStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
            (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
            i32,
            (
                HashTableUnitToString::FuncHashKey,
                HashTableUnitToString::FuncKeyEqual,
                HashTableUnitToString::FuncKeyStr,
                HashTableUnitToString::FuncValueStr,
            ),
        ),
    );
    let mut HtCr2U: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            HashTableCrToUnit::FuncHashKey,
            HashTableCrToUnit::FuncKeyEqual,
            HashTableCrToUnit::FuncKeyStr,
            HashTableCrToUnit::FuncValueStr,
        ),
    );
    let mut HtS2U: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            HashTableStringToUnit::FuncHashKey,
            HashTableStringToUnit::FuncKeyEqual,
            HashTableStringToUnit::FuncKeyStr,
            HashTableStringToUnit::FuncValueStr,
        ),
    );
    let mut HtU2S: (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            HashTableUnitToString::FuncHashKey,
            HashTableUnitToString::FuncKeyEqual,
            HashTableUnitToString::FuncKeyStr,
            HashTableUnitToString::FuncValueStr,
        ),
    );
    let mut b1: bool;
    let mut b2: bool;
    let mut b3: bool;
    (HtCr2U, b1, HtS2U, HtU2S) = List::fold(
        invarlist,
        &move |__a0: metamodelica::Ref<DAE::Element>,
               __a1: (
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::ComponentRef>,
                                metamodelica::Ref<DAE::ComponentRef>,
                            ) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
                ),
            ),
            bool,
            (
                metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
                (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
                    Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
                    Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
                ),
            ),
            (
                metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
                (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
                    Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
                    Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
                ),
            ),
        )| foldBindingExp(&__a0, __a1),
        (inHtCr2U, true, inHtS2U, inHtU2S),
    )?;
    (HtCr2U, b2, HtS2U, HtU2S) = List::fold1(
        ineqList,
        &move |__a0: metamodelica::Ref<DAE::Element>,
               __a1: metamodelica::List<Functionargs>,
               __a2: (
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::ComponentRef>,
                                metamodelica::Ref<DAE::ComponentRef>,
                            ) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
                ),
            ),
            bool,
            (
                metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
                (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
                    Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
                    Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
                ),
            ),
            (
                metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
                (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
                    Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
                    Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
                ),
            ),
        )| foldEquation(__a0, &__a1, &__a2),
        inargs,
        (HtCr2U, true, HtS2U, HtU2S),
    )?;
    b3 = BaseHashTable::hasKey(Unit::UPDATECREF().clone(), &HtCr2U)?;
    outTpl = (HtCr2U, HtS2U, HtU2S);
    Ok(outTpl)
}

fn foldBindingExp(
    mut inVar: &metamodelica::Ref<DAE::Element>,
    mut inTpl: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            ),
        ),
        bool,
        (
            metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
            (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
            i32,
            (
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
            (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
            i32,
            (
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
) -> Result<(
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    bool,
    (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outTpl: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
            ),
            i32,
            (
                HashTableCrToUnit::FuncHashKey,
                HashTableCrToUnit::FuncKeyEqual,
                HashTableCrToUnit::FuncKeyStr,
                HashTableCrToUnit::FuncValueStr,
            ),
        ),
        bool,
        (
            metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
            (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
            i32,
            (
                HashTableStringToUnit::FuncHashKey,
                HashTableStringToUnit::FuncKeyEqual,
                HashTableStringToUnit::FuncKeyStr,
                HashTableStringToUnit::FuncValueStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
            (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
            i32,
            (
                HashTableUnitToString::FuncHashKey,
                HashTableUnitToString::FuncKeyEqual,
                HashTableUnitToString::FuncKeyStr,
                HashTableUnitToString::FuncValueStr,
            ),
        ),
    );
    outTpl = (::match_deref::match_deref! { match &((inVar.clone(), inTpl.clone())) {
        (Deref @ DAE::Element::VAR { componentRef: cref, ty: Deref @ DAE::Type::T_REAL { .. }, binding: Some(exp), source, .. }, (HtCr2U, b, HtS2U, HtU2S)) => {
            let mut crefExp: metamodelica::Ref<DAE::Exp>;
            let mut eq: metamodelica::Ref<DAE::Element>;
            let mut HtCr2U = (*HtCr2U).clone();
            let mut b = (*b).clone();
            let mut HtS2U = (*HtS2U).clone();
            let mut HtU2S = (*HtU2S).clone();
            crefExp = Expression::crefExp(cref.clone())?;
            eq = metamodelica::Ref::new(DAE::Element::EQUATION { exp: crefExp, scalar: exp.clone(), source: source.clone() });
            (HtCr2U, b, HtS2U, HtU2S) = foldEquation(eq, &(metamodelica::nil()), &((HtCr2U.clone(), b.clone(), HtS2U.clone(), HtU2S.clone())))?;
            (HtCr2U.clone(), b.clone(), HtS2U.clone(), HtU2S.clone())
        },
        (Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_REAL { .. }, binding: Some(_), .. }, (HtCr2U, _, HtS2U, HtU2S)) => {
            (HtCr2U.clone(), false, HtS2U.clone(), HtU2S.clone())
        },
        _ => {
            inTpl
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

fn foldEquation(
    mut inEq: metamodelica::Ref<DAE::Element>,
    mut inargs: &metamodelica::List<Functionargs>,
    mut inTpl: &(
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            ),
        ),
        bool,
        (
            metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
            (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
            i32,
            (
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
            (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
            i32,
            (
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
) -> Result<(
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    bool,
    (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outTpl: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
            ),
            i32,
            (
                HashTableCrToUnit::FuncHashKey,
                HashTableCrToUnit::FuncKeyEqual,
                HashTableCrToUnit::FuncKeyStr,
                HashTableCrToUnit::FuncValueStr,
            ),
        ),
        bool,
        (
            metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
            (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
            i32,
            (
                HashTableStringToUnit::FuncHashKey,
                HashTableStringToUnit::FuncKeyEqual,
                HashTableStringToUnit::FuncKeyStr,
                HashTableStringToUnit::FuncValueStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
            (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
            i32,
            (
                HashTableUnitToString::FuncHashKey,
                HashTableUnitToString::FuncKeyEqual,
                HashTableUnitToString::FuncKeyStr,
                HashTableUnitToString::FuncValueStr,
            ),
        ),
    );
    let mut HtCr2U: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            HashTableCrToUnit::FuncHashKey,
            HashTableCrToUnit::FuncKeyEqual,
            HashTableCrToUnit::FuncKeyStr,
            HashTableCrToUnit::FuncValueStr,
        ),
    );
    let mut HtS2U: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            HashTableStringToUnit::FuncHashKey,
            HashTableStringToUnit::FuncKeyEqual,
            HashTableStringToUnit::FuncKeyStr,
            HashTableStringToUnit::FuncValueStr,
        ),
    );
    let mut HtU2S: (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            HashTableUnitToString::FuncHashKey,
            HashTableUnitToString::FuncKeyEqual,
            HashTableUnitToString::FuncKeyStr,
            HashTableUnitToString::FuncValueStr,
        ),
    );
    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
    let mut b: bool;
    (HtCr2U, b, HtS2U, HtU2S) = inTpl.clone();
    (HtCr2U, HtS2U, HtU2S, expListList) = foldEquation2(&inEq, HtCr2U, HtS2U, HtU2S, inargs)?;
    List::map2_0(
        &expListList,
        &move |__a0: metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>,
               __a1: metamodelica::Ref<DAE::Element>,
               __a2: (
            metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
            (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
            i32,
            (
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            ),
        )| Errorfunction(__a0, &__a1, &__a2),
        inEq,
        HtU2S.clone(),
    )?;
    outTpl = (HtCr2U, b, HtS2U, HtU2S);
    Ok(outTpl)
}

fn foldEquation2(
    mut eq: &metamodelica::Ref<DAE::Element>,
    mut htCr2U: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut htS2U: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut htU2S: (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut args: &metamodelica::List<Functionargs>,
) -> Result<(
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
    metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>,
)> {
    let mut htCr2U: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            HashTableCrToUnit::FuncHashKey,
            HashTableCrToUnit::FuncKeyEqual,
            HashTableCrToUnit::FuncKeyStr,
            HashTableCrToUnit::FuncValueStr,
        ),
    ) = htCr2U;
    let mut htS2U: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            HashTableStringToUnit::FuncHashKey,
            HashTableStringToUnit::FuncKeyEqual,
            HashTableStringToUnit::FuncKeyStr,
            HashTableStringToUnit::FuncValueStr,
        ),
    ) = htS2U;
    let mut htU2S: (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            HashTableUnitToString::FuncHashKey,
            HashTableUnitToString::FuncKeyEqual,
            HashTableUnitToString::FuncKeyStr,
            HashTableUnitToString::FuncValueStr,
        ),
    ) = htU2S;
    let mut inconsistentUnits: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>> =
        metamodelica::nil();
    inconsistentUnits = (::match_deref::match_deref! { match eq {
        Deref @ DAE::Element::DEFINE { componentRef: __eq_componentRef, exp: __eq_exp, .. } => {
            let mut temp: metamodelica::Ref<DAE::Exp>;
            let mut lhs: metamodelica::Ref<DAE::Exp>;
            lhs = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: __eq_componentRef.clone(), ty: DAE::T_REAL_DEFAULT().clone() });
            temp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: __eq_exp.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: lhs });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(&temp, &((htCr2U, htS2U, htU2S)), Unit::Unit::MASTER { varList: metamodelica::nil() }, args);
            htCr2U = metamodelica::Own::own(__pa0);
            htS2U = metamodelica::Own::own(__pa1);
            htU2S = metamodelica::Own::own(__pa2);
            inconsistentUnits = metamodelica::Own::own(__pa3);
            inconsistentUnits
        },
        Deref @ DAE::Element::INITIALDEFINE { componentRef: __eq_componentRef, exp: __eq_exp, .. } => {
            let mut temp: metamodelica::Ref<DAE::Exp>;
            let mut lhs: metamodelica::Ref<DAE::Exp>;
            lhs = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: __eq_componentRef.clone(), ty: DAE::T_REAL_DEFAULT().clone() });
            temp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: __eq_exp.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: lhs });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(&temp, &((htCr2U, htS2U, htU2S)), Unit::Unit::MASTER { varList: metamodelica::nil() }, args);
            htCr2U = metamodelica::Own::own(__pa0);
            htS2U = metamodelica::Own::own(__pa1);
            htU2S = metamodelica::Own::own(__pa2);
            inconsistentUnits = metamodelica::Own::own(__pa3);
            inconsistentUnits
        },
        Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::TUPLE { PR: expl }, scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::FULLYQUALIFIED { path }, .. }, .. } => {
            let mut expList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
            let mut expList3: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
            let mut s1: ArcStr;
            let mut outvars: metamodelica::List<ArcStr>;
            let mut outunitlist: metamodelica::List<ArcStr>;
            s1 = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            s1 = System::trim(s1, literal!("."));
            (_, outvars, _, outunitlist) = getNamedUnitlist(s1.clone(), args.clone());
            (htCr2U, htS2U, htU2S, expList2) = foldCallArg1(metamodelica::AsArg::as_arg(&expl), htCr2U, htS2U, htU2S, Unit::Unit::MASTER { varList: metamodelica::nil() }, &outunitlist, &outvars, s1)?;
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(var_field!((**eq).scalar, DAE::Element::EQUATION), &((htCr2U, htS2U, htU2S)), Unit::Unit::MASTER { varList: metamodelica::nil() }, args);
            htCr2U = metamodelica::Own::own(__pa0);
            htS2U = metamodelica::Own::own(__pa1);
            htU2S = metamodelica::Own::own(__pa2);
            expList3 = metamodelica::Own::own(__pa3);
            List::append_reverse(&expList2, expList3)
        },
        Deref @ DAE::Element::EQUATION { exp: lhs, scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::FULLYQUALIFIED { path }, .. }, .. } => {
            let mut temp: metamodelica::Ref<DAE::Exp>;
            let mut expList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
            let mut expList3: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
            let mut b: bool;
            let mut ut1: Unit::Unit;
            let mut ut2: Unit::Unit;
            let mut s1: ArcStr;
            let mut formalargs: ArcStr;
            let mut formalvar: ArcStr;
            let mut outvars: metamodelica::List<ArcStr>;
            let mut outunitlist: metamodelica::List<ArcStr>;
            s1 = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            s1 = System::trim(s1, literal!("."));
            (_, outvars, _, outunitlist) = getNamedUnitlist(s1.clone(), args.clone());
            let (__pa0, (__pa1, __pa2, __pa3), _) = insertUnitInEquation(lhs, &((htCr2U, htS2U, htU2S)), Unit::Unit::MASTER { varList: metamodelica::nil() }, args);
            ut1 = metamodelica::Own::own(__pa0);
            htCr2U = metamodelica::Own::own(__pa1);
            htS2U = metamodelica::Own::own(__pa2);
            htU2S = metamodelica::Own::own(__pa3);
            formalargs = (outunitlist).head().cloned()?;
            formalvar = (outvars).head().cloned()?;
            ut2 = if (metamodelica::stringEq(&formalargs, &(literal!("NONE")))) {Unit::Unit::MASTER { varList: metamodelica::nil() }} else {Unit::parseUnitString(formalargs, &(Unit::getKnownUnits()?))?};
            (b, _, _) = UnitTypesEqual(ut1.clone(), ut2.clone(), htCr2U.clone());
            if b {
                expList2 = metamodelica::nil();
            } else {
                temp = makenewcref(lhs.clone(), formalvar, s1)?;
                expList2 = metamodelica::cons(list![(lhs.clone(), ut1), (temp, ut2)], metamodelica::nil());
            }
            let (_, (__pa4, __pa5, __pa6), __pa7) = insertUnitInEquation(var_field!((**eq).scalar, DAE::Element::EQUATION), &((htCr2U, htS2U, htU2S)), Unit::Unit::MASTER { varList: metamodelica::nil() }, args);
            htCr2U = metamodelica::Own::own(__pa4);
            htS2U = metamodelica::Own::own(__pa5);
            htU2S = metamodelica::Own::own(__pa6);
            expList3 = metamodelica::Own::own(__pa7);
            List::append_reverse(&expList2, expList3)
        },
        Deref @ DAE::Element::EQUATION { exp: __eq_exp, scalar: __eq_scalar, .. } => {
            let mut temp: metamodelica::Ref<DAE::Exp>;
            temp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: __eq_scalar.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: __eq_exp.clone() });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(&temp, &((htCr2U, htS2U, htU2S)), Unit::Unit::MASTER { varList: metamodelica::nil() }, args);
            htCr2U = metamodelica::Own::own(__pa0);
            htS2U = metamodelica::Own::own(__pa1);
            htU2S = metamodelica::Own::own(__pa2);
            inconsistentUnits = metamodelica::Own::own(__pa3);
            inconsistentUnits
        },
        Deref @ DAE::Element::EQUEQUATION { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::INITIALEQUATION { exp1: __eq_exp1, exp2: __eq_exp2, .. } => {
            let mut temp: metamodelica::Ref<DAE::Exp>;
            temp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: __eq_exp2.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: __eq_exp1.clone() });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(&temp, &((htCr2U, htS2U, htU2S)), Unit::Unit::MASTER { varList: metamodelica::nil() }, args);
            htCr2U = metamodelica::Own::own(__pa0);
            htS2U = metamodelica::Own::own(__pa1);
            htU2S = metamodelica::Own::own(__pa2);
            inconsistentUnits = metamodelica::Own::own(__pa3);
            inconsistentUnits
        },
        Deref @ DAE::Element::ARRAY_EQUATION { array: __eq_array, exp: __eq_exp, .. } => {
            let mut temp: metamodelica::Ref<DAE::Exp>;
            temp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: __eq_array.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: __eq_exp.clone() });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(&temp, &((htCr2U, htS2U, htU2S)), Unit::Unit::MASTER { varList: metamodelica::nil() }, args);
            htCr2U = metamodelica::Own::own(__pa0);
            htS2U = metamodelica::Own::own(__pa1);
            htU2S = metamodelica::Own::own(__pa2);
            inconsistentUnits = metamodelica::Own::own(__pa3);
            inconsistentUnits
        },
        Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { array: __eq_array, exp: __eq_exp, .. } => {
            let mut temp: metamodelica::Ref<DAE::Exp>;
            temp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: __eq_array.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: __eq_exp.clone() });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(&temp, &((htCr2U, htS2U, htU2S)), Unit::Unit::MASTER { varList: metamodelica::nil() }, args);
            htCr2U = metamodelica::Own::own(__pa0);
            htS2U = metamodelica::Own::own(__pa1);
            htU2S = metamodelica::Own::own(__pa2);
            inconsistentUnits = metamodelica::Own::own(__pa3);
            inconsistentUnits
        },
        Deref @ DAE::Element::COMPLEX_EQUATION { lhs: __eq_lhs, rhs: __eq_rhs, .. } => {
            let mut temp: metamodelica::Ref<DAE::Exp>;
            temp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: __eq_rhs.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: __eq_lhs.clone() });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(&temp, &((htCr2U, htS2U, htU2S)), Unit::Unit::MASTER { varList: metamodelica::nil() }, args);
            htCr2U = metamodelica::Own::own(__pa0);
            htS2U = metamodelica::Own::own(__pa1);
            htU2S = metamodelica::Own::own(__pa2);
            inconsistentUnits = metamodelica::Own::own(__pa3);
            inconsistentUnits
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: __eq_lhs, rhs: __eq_rhs, .. } => {
            let mut temp: metamodelica::Ref<DAE::Exp>;
            temp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: __eq_rhs.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: __eq_lhs.clone() });
            if Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())? {
                ExpressionDump::dumpExp(temp.clone())?;
            }
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(&temp, &((htCr2U, htS2U, htU2S)), Unit::Unit::MASTER { varList: metamodelica::nil() }, args);
            htCr2U = metamodelica::Own::own(__pa0);
            htS2U = metamodelica::Own::own(__pa1);
            htU2S = metamodelica::Own::own(__pa2);
            inconsistentUnits = metamodelica::Own::own(__pa3);
            inconsistentUnits
        },
        Deref @ DAE::Element::WHEN_EQUATION { equations: __eq_equations, .. } => {
            for mut e in &*__eq_equations.clone() {
                (htCr2U, htS2U, htU2S, inconsistentUnits) = foldEquation2(metamodelica::AsArg::as_arg(&e), htCr2U, htS2U, htU2S, args)?;
            }
            inconsistentUnits
        },
        Deref @ DAE::Element::IF_EQUATION { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::NORETCALL { exp: __eq_exp, .. } => {
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(metamodelica::AsArg::as_arg(&__eq_exp), &((htCr2U, htS2U, htU2S)), Unit::Unit::MASTER { varList: metamodelica::nil() }, args);
            htCr2U = metamodelica::Own::own(__pa0);
            htS2U = metamodelica::Own::own(__pa1);
            htU2S = metamodelica::Own::own(__pa2);
            inconsistentUnits = metamodelica::Own::own(__pa3);
            inconsistentUnits
        },
        Deref @ DAE::Element::INITIAL_NORETCALL { exp: __eq_exp, .. } => {
            let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(metamodelica::AsArg::as_arg(&__eq_exp), &((htCr2U, htS2U, htU2S)), Unit::Unit::MASTER { varList: metamodelica::nil() }, args);
            htCr2U = metamodelica::Own::own(__pa0);
            htS2U = metamodelica::Own::own(__pa1);
            htU2S = metamodelica::Own::own(__pa2);
            inconsistentUnits = metamodelica::Own::own(__pa3);
            inconsistentUnits
        },
        Deref @ DAE::Element::INITIAL_ASSERT { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::ASSERT { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::TERMINATE { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::INITIAL_TERMINATE { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::REINIT { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::ALGORITHM { .. } => {
            metamodelica::nil()
        },
        Deref @ DAE::Element::INITIALALGORITHM { .. } => {
            metamodelica::nil()
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FUnitCheck.foldEquation2")); __mm_s.push_str(&*literal!(" failed on: ")); __mm_s.push_str(&*DAEDump::dumpEquationStr(eq)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/FUnitCheck.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((htCr2U, htS2U, htU2S, inconsistentUnits))
}

fn makenewcref(
    mut inexp: metamodelica::Ref<DAE::Exp>,
    mut instring: ArcStr,
    mut instring1: ArcStr,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outexp: metamodelica::Ref<DAE::Exp>;
    outexp = (::match_deref::match_deref! { match &(inexp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, .. } => {
            let mut s1 = instring;
            let mut s2 = instring1;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut name = (*name).clone();
            name = { let mut __mm_s = String::new(); __mm_s.push_str(&*s2); __mm_s.push_str(&*literal!("()")); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*s1); ArcStr::from(__mm_s) };
            cr = ComponentReference::makeUntypedCrefIdent(name.clone());
            assign_variant_field!(inexp => DAE::Exp::CREF; componentRef = cr);
            outexp = inexp;
            outexp
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outexp)
}

fn insertUnitInEquation(
    mut inEq: &metamodelica::Ref<DAE::Exp>,
    mut inTpl: &(
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
            (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
            i32,
            (
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
            (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
            i32,
            (
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
    mut inUt: Unit::Unit,
    mut inargs: &metamodelica::List<Functionargs>,
) -> (
    Unit::Unit,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
            (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
            i32,
            (
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
            (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
            i32,
            (
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
    metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>,
) {
    let mut outUt: Unit::Unit;
    let mut outTpl: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
            ),
            i32,
            (
                HashTableCrToUnit::FuncHashKey,
                HashTableCrToUnit::FuncKeyEqual,
                HashTableCrToUnit::FuncKeyStr,
                HashTableCrToUnit::FuncValueStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
            (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
            i32,
            (
                HashTableStringToUnit::FuncHashKey,
                HashTableStringToUnit::FuncKeyEqual,
                HashTableStringToUnit::FuncKeyStr,
                HashTableStringToUnit::FuncValueStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
            (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
            i32,
            (
                HashTableUnitToString::FuncHashKey,
                HashTableUnitToString::FuncKeyEqual,
                HashTableUnitToString::FuncKeyStr,
                HashTableUnitToString::FuncValueStr,
            ),
        ),
    );
    let mut outexpList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
    (outUt, outTpl, outexpList) = 'mc: {
        let __mc_input = (&**inEq, inTpl, inUt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::SUB { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs)) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), ut.clone(), inargs);
                    ut2 = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    let (true, __pa10, __pa11) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())) else { return Err("pattern mismatch") };
                    ut = metamodelica::Own::own(__pa10);
                    HtCr2U = metamodelica::Own::own(__pa11);
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::SUB { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs);
                    ut2 = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), ut2.clone(), inargs);
                    ut = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    let (true, __pa10, __pa11) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())) else { return Err("pattern mismatch") };
                    ut = metamodelica::Own::own(__pa10);
                    HtCr2U = metamodelica::Own::own(__pa11);
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::SUB { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs)) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), ut.clone(), inargs);
                    ut2 = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    let (false, _, _) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())) else { return Err("pattern mismatch") };
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    expListList = metamodelica::cons(list![(exp1.clone(), ut.clone()), (exp2.clone(), ut2.clone())], expListList.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::SUB { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs);
                    ut2 = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), ut2.clone(), inargs);
                    ut = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    let (false, _, _) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())) else { return Err("pattern mismatch") };
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    expListList = metamodelica::cons(list![(exp1.clone(), ut.clone()), (exp2.clone(), ut2.clone())], expListList.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs)) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), ut.clone(), inargs);
                    ut2 = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    let (true, __pa10, __pa11) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())) else { return Err("pattern mismatch") };
                    ut = metamodelica::Own::own(__pa10);
                    HtCr2U = metamodelica::Own::own(__pa11);
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs);
                    ut2 = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), ut2.clone(), inargs);
                    ut = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    let (true, __pa10, __pa11) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())) else { return Err("pattern mismatch") };
                    ut = metamodelica::Own::own(__pa10);
                    HtCr2U = metamodelica::Own::own(__pa11);
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs)) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), ut.clone(), inargs);
                    ut2 = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    let (false, _, _) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())) else { return Err("pattern mismatch") };
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    expListList = metamodelica::cons(list![(exp1.clone(), ut.clone()), (exp2.clone(), ut2.clone())], expListList.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs);
                    ut2 = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), ut2.clone(), inargs);
                    ut = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    let (false, _, _) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())) else { return Err("pattern mismatch") };
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    expListList = metamodelica::cons(list![(exp1.clone(), ut.clone()), (exp2.clone(), ut2.clone())], expListList.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut s1: ArcStr;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (__pa5 @ Unit::Unit::UNIT { .. }, (__pa6, __pa7, __pa8), __pa9) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut2 = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    ut = Unit::unitMul(ut.clone(), ut2.clone())?;
                    s1 = Unit::unitString(ut.clone(), &(HtU2S.clone()))?;
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    HtS2U = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    HtU2S = addUnit2HtU2S(&((s1.clone(), ut.clone())), HtU2S.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::MASTER { .. }) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { .. }, (__pa0, __pa1, __pa2), __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    HtCr2U = metamodelica::Own::own(__pa0);
                    HtS2U = metamodelica::Own::own(__pa1);
                    HtU2S = metamodelica::Own::own(__pa2);
                    expListList = metamodelica::Own::own(__pa3);
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::UNIT { .. }, (__pa4, __pa5, __pa6), __pa7) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    HtCr2U = metamodelica::Own::own(__pa4);
                    HtS2U = metamodelica::Own::own(__pa5);
                    HtU2S = metamodelica::Own::own(__pa6);
                    expListList2 = metamodelica::Own::own(__pa7);
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::UNIT { .. }) => {
                    let mut lcr: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut s1: ArcStr;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { varList: __pa0 }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    lcr = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (__pa5 @ Unit::Unit::UNIT { .. }, (__pa6, __pa7, __pa8), __pa9) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut2 = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    ut = Unit::unitDiv(inUt.clone(), ut2.clone())?;
                    s1 = Unit::unitString(ut.clone(), &(HtU2S.clone()))?;
                    HtCr2U = List::fold1(&lcr, &updateHtCr2U, ut.clone(), HtCr2U.clone())?;
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    HtS2U = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    HtU2S = addUnit2HtU2S(&((s1.clone(), ut.clone())), HtU2S.clone());
                    Ok((inUt.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::MASTER { .. }) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::UNIT { .. }, (__pa0, __pa1, __pa2), __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    HtCr2U = metamodelica::Own::own(__pa0);
                    HtS2U = metamodelica::Own::own(__pa1);
                    HtU2S = metamodelica::Own::own(__pa2);
                    expListList = metamodelica::Own::own(__pa3);
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { .. }, (__pa4, __pa5, __pa6), __pa7) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    HtCr2U = metamodelica::Own::own(__pa4);
                    HtS2U = metamodelica::Own::own(__pa5);
                    HtU2S = metamodelica::Own::own(__pa6);
                    expListList2 = metamodelica::Own::own(__pa7);
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::UNIT { .. }) => {
                    let mut lcr: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut s1: ArcStr;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut2 = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { varList: __pa5 }, (__pa6, __pa7, __pa8), __pa9) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    lcr = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    ut = Unit::unitDiv(inUt.clone(), ut2.clone())?;
                    s1 = Unit::unitString(ut.clone(), &(HtU2S.clone()))?;
                    HtCr2U = List::fold1(&lcr, &updateHtCr2U, ut.clone(), HtCr2U.clone())?;
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    HtS2U = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    HtU2S = addUnit2HtU2S(&((s1.clone(), ut.clone())), HtU2S.clone());
                    Ok((inUt.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { .. }, (__pa0, __pa1, __pa2), __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    HtCr2U = metamodelica::Own::own(__pa0);
                    HtS2U = metamodelica::Own::own(__pa1);
                    HtU2S = metamodelica::Own::own(__pa2);
                    expListList = metamodelica::Own::own(__pa3);
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { .. }, (__pa4, __pa5, __pa6), __pa7) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    HtCr2U = metamodelica::Own::own(__pa4);
                    HtS2U = metamodelica::Own::own(__pa5);
                    HtU2S = metamodelica::Own::own(__pa6);
                    expListList2 = metamodelica::Own::own(__pa7);
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::DIV { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut s1: ArcStr;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (__pa5 @ Unit::Unit::UNIT { .. }, (__pa6, __pa7, __pa8), __pa9) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut2 = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    ut = Unit::unitDiv(ut.clone(), ut2.clone())?;
                    s1 = Unit::unitString(ut.clone(), &(HtU2S.clone()))?;
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    HtS2U = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    HtU2S = addUnit2HtU2S(&((s1.clone(), ut.clone())), HtU2S.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::DIV { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::MASTER { .. }) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { .. }, (__pa0, __pa1, __pa2), __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    HtCr2U = metamodelica::Own::own(__pa0);
                    HtS2U = metamodelica::Own::own(__pa1);
                    HtU2S = metamodelica::Own::own(__pa2);
                    expListList = metamodelica::Own::own(__pa3);
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::UNIT { .. }, (__pa4, __pa5, __pa6), __pa7) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    HtCr2U = metamodelica::Own::own(__pa4);
                    HtS2U = metamodelica::Own::own(__pa5);
                    HtU2S = metamodelica::Own::own(__pa6);
                    expListList2 = metamodelica::Own::own(__pa7);
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::DIV { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::UNIT { .. }) => {
                    let mut lcr: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut s1: ArcStr;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { varList: __pa0 }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    lcr = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (__pa5 @ Unit::Unit::UNIT { .. }, (__pa6, __pa7, __pa8), __pa9) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut2 = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    ut = Unit::unitMul(inUt.clone(), ut2.clone())?;
                    s1 = Unit::unitString(ut.clone(), &(HtU2S.clone()))?;
                    HtCr2U = List::fold1(&lcr, &updateHtCr2U, ut.clone(), HtCr2U.clone())?;
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    HtS2U = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    HtU2S = addUnit2HtU2S(&((s1.clone(), ut.clone())), HtU2S.clone());
                    Ok((inUt.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::DIV { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::MASTER { .. }) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::UNIT { .. }, (__pa0, __pa1, __pa2), __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    HtCr2U = metamodelica::Own::own(__pa0);
                    HtS2U = metamodelica::Own::own(__pa1);
                    HtU2S = metamodelica::Own::own(__pa2);
                    expListList = metamodelica::Own::own(__pa3);
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { .. }, (__pa4, __pa5, __pa6), __pa7) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    HtCr2U = metamodelica::Own::own(__pa4);
                    HtS2U = metamodelica::Own::own(__pa5);
                    HtU2S = metamodelica::Own::own(__pa6);
                    expListList2 = metamodelica::Own::own(__pa7);
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::DIV { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::UNIT { .. }) => {
                    let mut lcr: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut s1: ArcStr;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut2 = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { varList: __pa5 }, (__pa6, __pa7, __pa8), __pa9) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    lcr = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    ut = Unit::unitDiv(ut2.clone(), inUt.clone())?;
                    s1 = Unit::unitString(ut.clone(), &(HtU2S.clone()))?;
                    HtCr2U = List::fold1(&lcr, &updateHtCr2U, ut.clone(), HtCr2U.clone())?;
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    HtS2U = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    HtU2S = addUnit2HtU2S(&((s1.clone(), ut.clone())), HtU2S.clone());
                    Ok((inUt.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::DIV { .. }, exp2 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { .. }, (__pa0, __pa1, __pa2), __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    HtCr2U = metamodelica::Own::own(__pa0);
                    HtS2U = metamodelica::Own::own(__pa1);
                    HtU2S = metamodelica::Own::own(__pa2);
                    expListList = metamodelica::Own::own(__pa3);
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { .. }, (__pa4, __pa5, __pa6), __pa7) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    HtCr2U = metamodelica::Own::own(__pa4);
                    HtS2U = metamodelica::Own::own(__pa5);
                    HtU2S = metamodelica::Own::own(__pa6);
                    expListList2 = metamodelica::Own::own(__pa7);
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::POW { .. }, exp2: Deref @ DAE::Exp::RCONST { real: r } }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut i: i32;
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut s1: ArcStr;
                    let mut ut: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    i = ((r.clone()).0.floor() as i32);
                    let true = (realEq(r.clone(), intReal(i))) else { return Err("pattern mismatch") };
                    ut = Unit::unitPow(ut.clone(), i)?;
                    s1 = Unit::unitString(ut.clone(), &(HtU2S.clone()))?;
                    HtS2U = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    HtU2S = addUnit2HtU2S(&((s1.clone(), ut.clone())), HtU2S.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::POW { .. }, exp2: Deref @ DAE::Exp::RCONST { real: r } }, (HtCr2U, HtS2U, HtU2S), ut @ Unit::Unit::UNIT { .. }) => {
                    let mut i1: i32;
                    let mut i2: i32;
                    let mut i3: i32;
                    let mut i4: i32;
                    let mut i5: i32;
                    let mut i6: i32;
                    let mut i7: i32;
                    let mut lcr: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut factor1: metamodelica::Real;
                    let mut s1: ArcStr;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { varList: __pa0 }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    lcr = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let Unit::UNIT { factor: __pa5, mol: __pa6, cd: __pa7, m: __pa8, s: __pa9, A: __pa10, K: __pa11, g: __pa12 } = (Unit::unitRoot(ut.clone(), r.clone())?) else { return Err("pattern mismatch") };
                    factor1 = metamodelica::Own::own(__pa5);
                    i1 = metamodelica::Own::own(__pa6);
                    i2 = metamodelica::Own::own(__pa7);
                    i3 = metamodelica::Own::own(__pa8);
                    i4 = metamodelica::Own::own(__pa9);
                    i5 = metamodelica::Own::own(__pa10);
                    i6 = metamodelica::Own::own(__pa11);
                    i7 = metamodelica::Own::own(__pa12);
                    HtCr2U = List::fold1(&lcr, &updateHtCr2U, Unit::Unit::UNIT { factor: factor1, mol: i1, cd: i2, m: i3, s: i4, A: i5, K: i6, g: i7 }, HtCr2U.clone())?;
                    s1 = Unit::unitString(Unit::Unit::UNIT { factor: factor1, mol: i1, cd: i2, m: i3, s: i4, A: i5, K: i6, g: i7 }, &(HtU2S.clone()))?;
                    HtS2U = addUnit2HtS2U((s1.clone(), Unit::Unit::UNIT { factor: factor1, mol: i1, cd: i2, m: i3, s: i4, A: i5, K: i6, g: i7 }), HtS2U.clone())?;
                    HtU2S = addUnit2HtU2S(&((s1.clone(), Unit::Unit::UNIT { factor: factor1, mol: i1, cd: i2, m: i3, s: i4, A: i5, K: i6, g: i7 })), HtU2S.clone());
                    Ok((inUt.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::POW { .. }, exp2: Deref @ DAE::Exp::RCONST { real: _ } }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs);
                    HtCr2U = metamodelica::Own::own(__pa0);
                    HtS2U = metamodelica::Own::own(__pa1);
                    HtU2S = metamodelica::Own::own(__pa2);
                    expListList = metamodelica::Own::own(__pa3);
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut ut: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs);
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::UNIT { .. }) => {
                    let mut lcr: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut s1: ArcStr;
                    let mut ut: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { varList: __pa0 }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    lcr = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    ut = Unit::unitMul(inUt.clone(), Unit::Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 1, A: 0, K: 0, g: 0 })?;
                    HtCr2U = List::fold1(&lcr, &updateHtCr2U, ut.clone(), HtCr2U.clone())?;
                    s1 = Unit::unitString(ut.clone(), &(HtU2S.clone()))?;
                    HtS2U = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    HtU2S = addUnit2HtU2S(&((s1.clone(), ut.clone())), HtU2S.clone());
                    Ok((inUt.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut s1: ArcStr;
                    let mut ut: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    ut = Unit::unitDiv(ut.clone(), Unit::Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 1, A: 0, K: 0, g: 0 })?;
                    s1 = Unit::unitString(ut.clone(), &(HtU2S.clone()))?;
                    HtS2U = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    HtU2S = addUnit2HtU2S(&((s1.clone(), ut.clone())), HtU2S.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::MASTER { .. }) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { .. }, (__pa0, __pa1, __pa2), __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    HtCr2U = metamodelica::Own::own(__pa0);
                    HtS2U = metamodelica::Own::own(__pa1);
                    HtU2S = metamodelica::Own::own(__pa2);
                    expListList = metamodelica::Own::own(__pa3);
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut i1: i32;
                    let mut i2: i32;
                    let mut i3: i32;
                    let mut i4: i32;
                    let mut i5: i32;
                    let mut i6: i32;
                    let mut i7: i32;
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut factor1: metamodelica::Real;
                    let mut s1: ArcStr;
                    let mut ut: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let Unit::UNIT { factor: __pa5, mol: __pa6, cd: __pa7, m: __pa8, s: __pa9, A: __pa10, K: __pa11, g: __pa12 } = (Unit::unitRoot(ut.clone(), metamodelica::OrderedFloat(2.0_f64))?) else { return Err("pattern mismatch") };
                    factor1 = metamodelica::Own::own(__pa5);
                    i1 = metamodelica::Own::own(__pa6);
                    i2 = metamodelica::Own::own(__pa7);
                    i3 = metamodelica::Own::own(__pa8);
                    i4 = metamodelica::Own::own(__pa9);
                    i5 = metamodelica::Own::own(__pa10);
                    i6 = metamodelica::Own::own(__pa11);
                    i7 = metamodelica::Own::own(__pa12);
                    s1 = Unit::unitString(Unit::Unit::UNIT { factor: factor1, mol: i1, cd: i2, m: i3, s: i4, A: i5, K: i6, g: i7 }, &(HtU2S.clone()))?;
                    HtS2U = addUnit2HtS2U((s1.clone(), Unit::Unit::UNIT { factor: factor1, mol: i1, cd: i2, m: i3, s: i4, A: i5, K: i6, g: i7 }), HtS2U.clone())?;
                    HtU2S = addUnit2HtU2S(&((s1.clone(), Unit::Unit::UNIT { factor: factor1, mol: i1, cd: i2, m: i3, s: i4, A: i5, K: i6, g: i7 })), HtU2S.clone());
                    Ok((Unit::Unit::UNIT { factor: factor1, mol: i1, cd: i2, m: i3, s: i4, A: i5, K: i6, g: i7 }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (HtCr2U, HtS2U, HtU2S), Unit::Unit::UNIT { .. }) => {
                    let mut lcr: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut s1: ArcStr;
                    let mut ut: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs)) {
                        (Unit::Unit::MASTER { varList: __pa0 }, (__pa1, __pa2, __pa3), __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    lcr = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    ut = Unit::unitPow(inUt.clone(), 2)?;
                    s1 = Unit::unitString(ut.clone(), &(HtU2S.clone()))?;
                    HtCr2U = List::fold1(&lcr, &updateHtCr2U, ut.clone(), HtCr2U.clone())?;
                    HtS2U = addUnit2HtS2U((s1.clone(), ut.clone()), HtS2U.clone())?;
                    HtU2S = addUnit2HtU2S(&((s1.clone(), ut.clone())), HtU2S.clone());
                    Ok((inUt.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), Unit::Unit::MASTER { varList: metamodelica::nil() }, inargs);
                    HtCr2U = metamodelica::Own::own(__pa0);
                    HtS2U = metamodelica::Own::own(__pa1);
                    HtU2S = metamodelica::Own::own(__pa2);
                    expListList = metamodelica::Own::own(__pa3);
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: _, expThen: exp2, expElse: exp3 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList3: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs);
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList2 = metamodelica::Own::own(__pa4);
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp3), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), ut.clone(), inargs);
                    ut2 = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList3 = metamodelica::Own::own(__pa9);
                    let (true, __pa10, __pa11) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())) else { return Err("pattern mismatch") };
                    ut = metamodelica::Own::own(__pa10);
                    HtCr2U = metamodelica::Own::own(__pa11);
                    expListList = List::append_reverse(&expListList2, expListList3.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: _, expThen: exp2, expElse: exp3 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList3: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp2), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs);
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList2 = metamodelica::Own::own(__pa4);
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp3), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), ut.clone(), inargs);
                    ut2 = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList3 = metamodelica::Own::own(__pa9);
                    let (false, _, _) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())) else { return Err("pattern mismatch") };
                    expListList = List::append_reverse(&expListList2, expListList3.clone());
                    expListList = metamodelica::cons(list![(exp2.clone(), ut.clone()), (exp3.clone(), ut2.clone())], expListList.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp1, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs);
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs);
                    ut2 = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    let (true, __pa10, __pa11) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())) else { return Err("pattern mismatch") };
                    ut = metamodelica::Own::own(__pa10);
                    HtCr2U = metamodelica::Own::own(__pa11);
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp1, exp2, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut expListList2: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut ut: Unit::Unit;
                    let mut ut2: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs);
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    let (__pa5, (__pa6, __pa7, __pa8), __pa9) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs);
                    ut2 = metamodelica::Own::own(__pa5);
                    HtCr2U = metamodelica::Own::own(__pa6);
                    HtS2U = metamodelica::Own::own(__pa7);
                    HtU2S = metamodelica::Own::own(__pa8);
                    expListList2 = metamodelica::Own::own(__pa9);
                    let (false, _, _) = (UnitTypesEqual(ut.clone(), ut2.clone(), HtCr2U.clone())) else { return Err("pattern mismatch") };
                    expListList = List::append_reverse(&expListList, expListList2.clone());
                    expListList = metamodelica::cons(list![(exp1.clone(), ut.clone()), (exp2.clone(), ut2.clone())], expListList.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { .. }, expLst: ExpList, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    (HtCr2U, HtS2U, HtU2S, expListList) = foldCallArg(metamodelica::AsArg::as_arg(&ExpList), HtCr2U.clone(), HtS2U.clone(), HtU2S.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::FULLYQUALIFIED { path }, expLst: ExpList, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut s1: ArcStr;
                    let mut invars: metamodelica::List<ArcStr>;
                    let mut inunitlist: metamodelica::List<ArcStr>;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    s1 = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
                    s1 = System::trim(s1.clone(), literal!("."));
                    (invars, _, inunitlist, _) = getNamedUnitlist(s1.clone(), inargs.clone());
                    (HtCr2U, HtS2U, HtU2S, expListList) = foldCallArg1(metamodelica::AsArg::as_arg(&ExpList), HtCr2U.clone(), HtS2U.clone(), HtU2S.clone(), inUt.clone(), &inunitlist, &invars, s1.clone())?;
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: exp1 }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
                    let mut ut: Unit::Unit;
                    let mut HtCr2U = (*HtCr2U).clone();
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(metamodelica::AsArg::as_arg(&exp1), &((HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())), inUt.clone(), inargs);
                    ut = metamodelica::Own::own(__pa0);
                    HtCr2U = metamodelica::Own::own(__pa1);
                    HtS2U = metamodelica::Own::own(__pa2);
                    HtU2S = metamodelica::Own::own(__pa3);
                    expListList = metamodelica::Own::own(__pa4);
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), expListList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (HtCr2U, HtS2U, HtU2S), _) => {
                    let mut ut: Unit::Unit;
                    let mut HtS2U = (*HtS2U).clone();
                    let mut HtU2S = (*HtU2S).clone();
                    let true = (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr), &(DAE::crefTime().clone()))?) else { return Err("pattern mismatch") };
                    ut = Unit::Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 1, A: 0, K: 0, g: 0 };
                    HtS2U = addUnit2HtS2U((literal!("time"), ut.clone()), HtS2U.clone())?;
                    HtU2S = addUnit2HtU2S(&((literal!("time"), ut.clone())), HtU2S.clone());
                    Ok((ut.clone(), (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone()), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_REAL { .. } }, (HtCr2U, _, _), _) => {
                    let mut ut: Unit::Unit;
                    ut = BaseHashTable::get(cr.clone(), &(HtCr2U.clone()))?;
                    Ok((ut.clone(), inTpl.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, inTpl.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outUt, outTpl, outexpList)
}

fn getNamedUnitlist(
    mut instring: ArcStr,
    mut inargs: metamodelica::List<Functionargs>,
) -> (
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
) {
    '__tco: loop {
        ::match_deref::match_deref! { match &((instring, inargs)) {
            (fnname, Deref @ metamodelica::ListNode::Cons { head: Functionargs { name: fnname1, invars, outvars, inunits: inunitlist, outunits: outunitlist }, tail: _ }) if (stringEq(&fnname, &fnname1)) => {
                let mut inunitlist = (*inunitlist).clone();
                let mut outunitlist = (*outunitlist).clone();
                inunitlist = inunitlist.clone();
                outunitlist = outunitlist.clone();
                return (invars.clone(), outvars.clone(), inunitlist.clone(), outunitlist.clone())
            },
            (fnname, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }) => {
                let mut invars: metamodelica::List<ArcStr>;
                let mut inunitlist: metamodelica::List<ArcStr>;
                let mut outunitlist: metamodelica::List<ArcStr>;
                let mut outvars: metamodelica::List<ArcStr>;
                { (instring, inargs) = (fnname.clone(), rest.clone()); continue '__tco; }
            },
            (_, _) => {
                return (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn UnitTypesEqual(
    mut inut: Unit::Unit,
    mut inut2: Unit::Unit,
    mut inHtCr2U: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> (
    bool,
    Unit::Unit,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
) {
    let mut b: bool;
    let mut outUt: Unit::Unit;
    let mut outHtCr2U: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            HashTableCrToUnit::FuncHashKey,
            HashTableCrToUnit::FuncKeyEqual,
            HashTableCrToUnit::FuncKeyStr,
            HashTableCrToUnit::FuncValueStr,
        ),
    );
    (b, outUt, outHtCr2U) = 'mc: {
        let __mc_input = (inut.clone(), inut2);
        if let Ok(__v) = (|| -> Result<_> {
            let (
                Unit::Unit::UNIT {
                    factor: mut factor1,
                    mol: mut i1,
                    cd: mut i2,
                    m: mut i3,
                    s: mut i4,
                    A: mut i5,
                    K: mut i6,
                    g: mut i7,
                },
                Unit::Unit::UNIT {
                    factor: mut factor2,
                    mol: mut j1,
                    cd: mut j2,
                    m: mut j3,
                    s: mut j4,
                    A: mut j5,
                    K: mut j6,
                    g: mut j7,
                },
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let true = (realEq(factor1.clone(), factor2.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i1.clone(), j1.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i2.clone(), j2.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i3.clone(), j3.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i4.clone(), j4.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i5.clone(), j5.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i6.clone(), j6.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i7.clone(), j7.clone())) else {
                return Err("pattern mismatch");
            };
            Ok((
                true,
                Unit::Unit::UNIT {
                    factor: factor1.clone(),
                    mol: i1.clone(),
                    cd: i2.clone(),
                    m: i3.clone(),
                    s: i4.clone(),
                    A: i5.clone(),
                    K: i6.clone(),
                    g: i7.clone(),
                },
                inHtCr2U.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                Unit::Unit::UNIT {
                    factor: mut factor1,
                    mol: mut i1,
                    cd: mut i2,
                    m: mut i3,
                    s: mut i4,
                    A: mut i5,
                    K: mut i6,
                    g: mut i7,
                },
                Unit::Unit::UNIT {
                    factor: mut factor2,
                    mol: mut j1,
                    cd: mut j2,
                    m: mut j3,
                    s: mut j4,
                    A: mut j5,
                    K: mut j6,
                    g: mut j7,
                },
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut r: metamodelica::Real;
            r = realMax(realAbs(factor1.clone()), realAbs(factor2.clone()));
            let true = (realLe(
                realDiv(realAbs((factor1.clone()) - (factor2.clone())), r),
                metamodelica::OrderedFloat(1e-3_f64),
            )) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i1.clone(), j1.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i2.clone(), j2.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i3.clone(), j3.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i4.clone(), j4.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i5.clone(), j5.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i6.clone(), j6.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i7.clone(), j7.clone())) else {
                return Err("pattern mismatch");
            };
            Ok((
                true,
                Unit::Unit::UNIT {
                    factor: factor1.clone(),
                    mol: i1.clone(),
                    cd: i2.clone(),
                    m: i3.clone(),
                    s: i4.clone(),
                    A: i5.clone(),
                    K: i6.clone(),
                    g: i7.clone(),
                },
                inHtCr2U.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut ut @ Unit::Unit::UNIT { .. }, Unit::Unit::MASTER { varList: ref lcr }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut HtCr2U: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
                ),
                i32,
                (
                    HashTableCrToUnit::FuncHashKey,
                    HashTableCrToUnit::FuncKeyEqual,
                    HashTableCrToUnit::FuncKeyStr,
                    HashTableCrToUnit::FuncValueStr,
                ),
            );
            HtCr2U = List::fold1(&(lcr.clone()), &updateHtCr2U, ut.clone(), inHtCr2U.clone())?;
            Ok((true, ut.clone(), HtCr2U.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::Unit::MASTER { varList: ref lcr }, mut ut @ Unit::Unit::UNIT { .. }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut HtCr2U: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
                ),
                i32,
                (
                    HashTableCrToUnit::FuncHashKey,
                    HashTableCrToUnit::FuncKeyEqual,
                    HashTableCrToUnit::FuncKeyStr,
                    HashTableCrToUnit::FuncValueStr,
                ),
            );
            HtCr2U = List::fold1(&(lcr.clone()), &updateHtCr2U, ut.clone(), inHtCr2U.clone())?;
            Ok((true, ut.clone(), HtCr2U.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::Unit::MASTER { varList: ref lcr }, Unit::Unit::MASTER { varList: ref lcr2 }) =
                __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut lcr2 = lcr2.clone();
            lcr2 = List::append_reverse(&(lcr.clone()), lcr2.clone());
            Ok((true, Unit::Unit::MASTER { varList: lcr2.clone() }, inHtCr2U.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::Unit::UNKNOWN { unit: mut s }, Unit::Unit::UNKNOWN { unit: mut s2 }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (stringEqual(&s, &s2)) else {
                return Err("pattern mismatch");
            };
            Ok((true, Unit::Unit::UNKNOWN { unit: s.clone() }, inHtCr2U.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::Unit::UNKNOWN { unit: mut s }, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok((true, Unit::Unit::UNKNOWN { unit: s.clone() }, inHtCr2U.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, Unit::Unit::UNKNOWN { unit: mut s }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok((true, Unit::Unit::UNKNOWN { unit: s.clone() }, inHtCr2U.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((false, inut.clone(), inHtCr2U.clone()))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (b, outUt, outHtCr2U)
}

fn updateHtCr2U(
    mut inCr: metamodelica::Ref<DAE::ComponentRef>,
    mut inUt: Unit::Unit,
    mut inHtCr2U: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
    ),
    i32,
    (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                ) -> Result<bool>
                + 'static,
        >,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outHtCr2U: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            HashTableCrToUnit::FuncHashKey,
            HashTableCrToUnit::FuncKeyEqual,
            HashTableCrToUnit::FuncKeyStr,
            HashTableCrToUnit::FuncValueStr,
        ),
    );
    outHtCr2U = 'mc: {
        let __mc_input = inHtCr2U.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (BaseHashTable::hasKey(Unit::UPDATECREF().clone(), &inHtCr2U)?) else {
                return Err("pattern mismatch");
            };
            BaseHashTable::update((inCr.clone(), inUt.clone()), &inHtCr2U)?;
            Ok(inHtCr2U.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut HtCr2U: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
                ),
                i32,
                (
                    HashTableCrToUnit::FuncHashKey,
                    HashTableCrToUnit::FuncKeyEqual,
                    HashTableCrToUnit::FuncKeyStr,
                    HashTableCrToUnit::FuncValueStr,
                ),
            );
            HtCr2U = BaseHashTable::add(
                (
                    Unit::UPDATECREF().clone(),
                    Unit::Unit::MASTER {
                        varList: metamodelica::nil(),
                    },
                ),
                inHtCr2U.clone(),
            )?;
            BaseHashTable::update((inCr.clone(), inUt.clone()), &HtCr2U)?;
            Ok(HtCr2U.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outHtCr2U)
}

fn Errorfunction(
    mut inexpList: metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>,
    mut inEq: &metamodelica::Ref<DAE::Element>,
    mut inHtU2S: &(
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inexpList) {
        expList => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut info: SourceInfo;
            info = getSourceInfo(inEq)?;
            s = DAEDump::dumpEquationStr(inEq);
            s1 = Errorfunction2(metamodelica::AsArg::as_arg(&expList), inHtU2S)?;
            s2 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following equation is INCONSISTENT due to specified unit information:")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
            Error::addSourceMessage(&(Error::COMPILER_WARNING.clone()), list![s2], &info)?;
            Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The units of following sub-expressions need to be equal:\n")); __mm_s.push_str(&*s1); ArcStr::from(__mm_s) })?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn getSourceInfo(mut inequation: &metamodelica::Ref<DAE::Element>) -> Result<SourceInfo> {
    let mut outinfo: SourceInfo;
    outinfo = (::match_deref::match_deref! { match inequation {
        Deref @ DAE::Element::EQUATION { source: Deref @ DAE::ElementSource { info, .. }, .. } => {
            info.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outinfo)
}

fn Errorfunction2(
    mut inexpList: &metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>,
    mut inHtU2S: &(
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<ArcStr> {
    let mut outS: ArcStr;
    outS = (::match_deref::match_deref! { match inexpList {
        Deref @ metamodelica::ListNode::Cons { head: (exp, ut), tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            s = ExpressionBasics::printExpStr(exp.clone())?;
            s1 = Unit::unitString(ut.clone(), inHtU2S)?;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- sub-expression \"")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("\" has unit \"")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) };
            s
        },
        Deref @ metamodelica::ListNode::Cons { head: (exp, ut), tail: expList } => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s = ExpressionBasics::printExpStr(exp.clone())?;
            s1 = Unit::unitString(ut.clone(), inHtU2S)?;
            s2 = Errorfunction2(expList, inHtU2S)?;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- sub-expression \"")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("\" has unit \"")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!("\"\n")); __mm_s.push_str(&*s2); ArcStr::from(__mm_s) };
            s
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outS)
}

pub(crate) fn GetVarList(mut indaelist: &DAE::DAElist) -> metamodelica::List<metamodelica::Ref<DAE::Element>> {
    let mut outstring: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut varlist: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    outstring = (::match_deref::match_deref! { match &(indaelist) {
        DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMP { dAElist: __esc_varlist, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            varlist = (*__esc_varlist).clone();
            varlist.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outstring
}

pub(crate) fn GetElementList(mut eqlist: &DAE::DAElist) -> metamodelica::List<metamodelica::Ref<DAE::Element>> {
    let mut outstring: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut eq1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    outstring = (match eqlist.clone() {
        DAE::DAElist {
            elementLst: ref __esc_eq1,
        } => {
            eq1 = __esc_eq1.clone();
            eq1.clone()
        }
    });
    outstring
}

fn foldCallArg(
    mut inExpList: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inHtCr2U: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHtS2U: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHtU2S: (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> (
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
    metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>,
) {
    let mut outHtCr2U: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            HashTableCrToUnit::FuncHashKey,
            HashTableCrToUnit::FuncKeyEqual,
            HashTableCrToUnit::FuncKeyStr,
            HashTableCrToUnit::FuncValueStr,
        ),
    ) = inHtCr2U;
    let mut outHtS2U: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            HashTableStringToUnit::FuncHashKey,
            HashTableStringToUnit::FuncKeyEqual,
            HashTableStringToUnit::FuncKeyStr,
            HashTableStringToUnit::FuncValueStr,
        ),
    ) = inHtS2U;
    let mut outHtU2S: (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            HashTableUnitToString::FuncHashKey,
            HashTableUnitToString::FuncKeyEqual,
            HashTableUnitToString::FuncKeyStr,
            HashTableUnitToString::FuncValueStr,
        ),
    ) = inHtU2S;
    let mut outExpListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>> =
        metamodelica::nil();
    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
    for mut exp in &**inExpList {
        let (_, (__pa0, __pa1, __pa2), __pa3) = insertUnitInEquation(
            metamodelica::AsArg::as_arg(&exp),
            &((outHtCr2U, outHtS2U, outHtU2S)),
            Unit::Unit::MASTER {
                varList: metamodelica::nil(),
            },
            &(metamodelica::nil()),
        );
        outHtCr2U = metamodelica::Own::own(__pa0);
        outHtS2U = metamodelica::Own::own(__pa1);
        outHtU2S = metamodelica::Own::own(__pa2);
        expListList = metamodelica::Own::own(__pa3);
        outExpListList = List::append_reverse(&expListList, outExpListList);
    }
    outExpListList = outExpListList.reverse();
    (outHtCr2U, outHtS2U, outHtU2S, outExpListList)
}

fn foldCallArg1(
    mut inExpList: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inHtCr2U: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHtS2U: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHtU2S: (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inunit: Unit::Unit,
    mut unitlist: &metamodelica::List<ArcStr>,
    mut invars: &metamodelica::List<ArcStr>,
    mut fname: ArcStr,
) -> Result<(
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
    metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>,
)> {
    let mut outHtCr2U: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            HashTableCrToUnit::FuncHashKey,
            HashTableCrToUnit::FuncKeyEqual,
            HashTableCrToUnit::FuncKeyStr,
            HashTableCrToUnit::FuncValueStr,
        ),
    ) = inHtCr2U;
    let mut outHtS2U: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            HashTableStringToUnit::FuncHashKey,
            HashTableStringToUnit::FuncKeyEqual,
            HashTableStringToUnit::FuncKeyStr,
            HashTableStringToUnit::FuncValueStr,
        ),
    ) = inHtS2U;
    let mut outHtU2S: (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            HashTableUnitToString::FuncHashKey,
            HashTableUnitToString::FuncKeyEqual,
            HashTableUnitToString::FuncKeyStr,
            HashTableUnitToString::FuncValueStr,
        ),
    ) = inHtU2S;
    let mut outExpListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>> =
        metamodelica::nil();
    let mut expListList: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Exp>, Unit::Unit)>>;
    let mut ut: Unit::Unit;
    let mut ut1: Unit::Unit;
    let mut s: ArcStr;
    let mut formalarg: ArcStr;
    let mut formalvar: ArcStr;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut temp: metamodelica::Ref<DAE::Exp>;
    let mut b: bool;
    for mut i in 1..=((inExpList).len() as i32) {
        exp = (inExpList).get(i)?;
        formalarg = (unitlist).get(i)?;
        formalvar = (invars).get(i)?;
        let (__pa0, (__pa1, __pa2, __pa3), __pa4) = insertUnitInEquation(
            &exp,
            &((outHtCr2U, outHtS2U, outHtU2S)),
            inunit.clone(),
            &(metamodelica::nil()),
        );
        ut = metamodelica::Own::own(__pa0);
        outHtCr2U = metamodelica::Own::own(__pa1);
        outHtS2U = metamodelica::Own::own(__pa2);
        outHtU2S = metamodelica::Own::own(__pa3);
        expListList = metamodelica::Own::own(__pa4);
        if metamodelica::stringEq(&formalarg, &(literal!("NONE"))) {
            ut1 = Unit::Unit::MASTER {
                varList: metamodelica::nil(),
            };
        } else {
            ut1 = Unit::parseUnitString(formalarg, &(Unit::getKnownUnits()?))?;
        }
        s = Unit::unitString(ut.clone(), &outHtU2S)?;
        (b, ut, _) = UnitTypesEqual(ut, ut1.clone(), outHtCr2U.clone());
        if b == true {
            expListList = metamodelica::nil();
        } else {
            temp = makenewcref(exp.clone(), formalvar, fname.clone())?;
            expListList = metamodelica::cons(list![(exp, ut), (temp, ut1)], metamodelica::nil());
        }
        outExpListList = List::append_reverse(&expListList, outExpListList);
    }
    Ok((outHtCr2U, outHtS2U, outHtU2S, outExpListList))
}

fn addUnit2HtS2U(
    mut inTpl: (ArcStr, Unit::Unit),
    mut inHtS2U: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
    (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
    i32,
    (
        Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
        Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
        Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outHtS2U: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            HashTableStringToUnit::FuncHashKey,
            HashTableStringToUnit::FuncKeyEqual,
            HashTableStringToUnit::FuncKeyStr,
            HashTableStringToUnit::FuncValueStr,
        ),
    );
    outHtS2U = BaseHashTable::add(inTpl, inHtS2U)?;
    Ok(outHtS2U)
}

fn addUnit2HtU2S(
    mut inTpl: &(ArcStr, Unit::Unit),
    mut inHtU2S: (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> (
    metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
    (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
    i32,
    (
        Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
        Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
        Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
    ),
) {
    let mut outHtU2S: (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            HashTableUnitToString::FuncHashKey,
            HashTableUnitToString::FuncKeyEqual,
            HashTableUnitToString::FuncKeyStr,
            HashTableUnitToString::FuncValueStr,
        ),
    );
    outHtU2S = 'mc: {
        let __mc_input = inTpl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let (mut s, mut ut) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut HtU2S: (
                metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
                (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
                i32,
                (
                    HashTableUnitToString::FuncHashKey,
                    HashTableUnitToString::FuncKeyEqual,
                    HashTableUnitToString::FuncKeyStr,
                    HashTableUnitToString::FuncValueStr,
                ),
            );
            let false = (BaseHashTable::hasKey(ut.clone(), &inHtU2S)?) else {
                return Err("pattern mismatch");
            };
            HtU2S = BaseHashTable::add((ut.clone(), s.clone()), inHtU2S.clone())?;
            Ok(HtU2S.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(inHtU2S.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outHtU2S
}

// get unit information based on old instantiation
fn convertUnitString2unit_old(
    mut var: &metamodelica::Ref<DAE::Element>,
    mut inTpl: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
            (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
            i32,
            (
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
            (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
            i32,
            (
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
) -> Result<(
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outTpl: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
            ),
            i32,
            (
                HashTableCrToUnit::FuncHashKey,
                HashTableCrToUnit::FuncKeyEqual,
                HashTableCrToUnit::FuncKeyStr,
                HashTableCrToUnit::FuncValueStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
            (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
            i32,
            (
                HashTableStringToUnit::FuncHashKey,
                HashTableStringToUnit::FuncKeyEqual,
                HashTableStringToUnit::FuncKeyStr,
                HashTableStringToUnit::FuncValueStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
            (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
            i32,
            (
                HashTableUnitToString::FuncHashKey,
                HashTableUnitToString::FuncKeyEqual,
                HashTableUnitToString::FuncKeyStr,
                HashTableUnitToString::FuncValueStr,
            ),
        ),
    );
    outTpl = (::match_deref::match_deref! { match &((var.clone(), inTpl.clone())) {
        (Deref @ DAE::Element::VAR { componentRef: cr, ty: Deref @ DAE::Type::T_REAL { .. }, variableAttributesOption: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { unit: Some(Deref @ DAE::Exp::SCONST { string: unitString }), .. }), .. }, (HtCr2U, HtS2U, HtU2S)) if (!metamodelica::stringEq(&unitString, &(literal!("")))) => {
            let mut ut: Unit::Unit;
            let mut HtCr2U = (*HtCr2U).clone();
            let mut HtS2U = (*HtS2U).clone();
            let mut HtU2S = (*HtU2S).clone();
            (ut, HtS2U, HtU2S) = parse(unitString.clone(), cr.clone(), HtS2U.clone(), HtU2S.clone())?;
            HtCr2U = BaseHashTable::add((cr.clone(), ut), HtCr2U.clone())?;
            (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())
        },
        (Deref @ DAE::Element::VAR { componentRef: cr, .. }, (HtCr2U, HtS2U, HtU2S)) => {
            let mut HtCr2U = (*HtCr2U).clone();
            let mut HtS2U = (*HtS2U).clone();
            let mut HtU2S = (*HtU2S).clone();
            HtCr2U = BaseHashTable::add((cr.clone(), Unit::Unit::MASTER { varList: list![cr.clone()] }), HtCr2U.clone())?;
            HtS2U = addUnit2HtS2U((literal!("-"), Unit::Unit::MASTER { varList: list![cr.clone()] }), HtS2U.clone())?;
            HtU2S = addUnit2HtU2S(&((literal!("-"), Unit::Unit::MASTER { varList: list![cr.clone()] })), HtU2S.clone());
            (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())
        },
        _ => {
            inTpl
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

//based on new Instantiation currently not fully operational
fn convertUnitString2unit(
    mut var: &metamodelica::Ref<DAE::Element>,
    mut inTpl: &(
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
            (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
            i32,
            (
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
            (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
            i32,
            (
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
                Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
) -> Result<(
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outTpl: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Unit::Unit)>>,
            ),
            i32,
            (
                HashTableCrToUnit::FuncHashKey,
                HashTableCrToUnit::FuncKeyEqual,
                HashTableCrToUnit::FuncKeyStr,
                HashTableCrToUnit::FuncValueStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
            (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
            i32,
            (
                HashTableStringToUnit::FuncHashKey,
                HashTableStringToUnit::FuncKeyEqual,
                HashTableStringToUnit::FuncKeyStr,
                HashTableStringToUnit::FuncValueStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
            (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
            i32,
            (
                HashTableUnitToString::FuncHashKey,
                HashTableUnitToString::FuncKeyEqual,
                HashTableUnitToString::FuncKeyStr,
                HashTableUnitToString::FuncValueStr,
            ),
        ),
    );
    outTpl = (::match_deref::match_deref! { match &((&**var, inTpl)) {
        (Deref @ DAE::Element::VAR { componentRef: cr, ty: Deref @ DAE::Type::T_REAL { varLst: varlst }, .. }, (HtCr2U, HtS2U, HtU2S)) if (false == (varlst).is_empty()) => {
            let mut unitString: ArcStr;
            let mut ut: Unit::Unit;
            let mut HtCr2U = (*HtCr2U).clone();
            let mut HtS2U = (*HtS2U).clone();
            let mut HtU2S = (*HtU2S).clone();
            unitString = parseVarList(metamodelica::AsArg::as_arg(&varlst));
            (ut, HtS2U, HtU2S) = parse(unitString, cr.clone(), HtS2U.clone(), HtU2S.clone())?;
            HtCr2U = BaseHashTable::add((cr.clone(), ut), HtCr2U.clone())?;
            (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())
        },
        (Deref @ DAE::Element::VAR { componentRef: cr, .. }, (HtCr2U, HtS2U, HtU2S)) => {
            let mut HtCr2U = (*HtCr2U).clone();
            let mut HtS2U = (*HtS2U).clone();
            let mut HtU2S = (*HtU2S).clone();
            HtCr2U = BaseHashTable::add((cr.clone(), Unit::Unit::MASTER { varList: list![cr.clone()] }), HtCr2U.clone())?;
            HtS2U = addUnit2HtS2U((literal!("-"), Unit::Unit::MASTER { varList: list![cr.clone()] }), HtS2U.clone())?;
            HtU2S = addUnit2HtU2S(&((literal!("-"), Unit::Unit::MASTER { varList: list![cr.clone()] })), HtU2S.clone());
            (HtCr2U.clone(), HtS2U.clone(), HtU2S.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outTpl)
}

fn parseVarList<'__b>(mut invarlist: &'__b metamodelica::List<metamodelica::Ref<DAE::Var>>) -> ArcStr {
    '__tco: loop {
        ::match_deref::match_deref! { match invarlist {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name, binding: eqbind, .. }, tail: _ } if (stringEq(&name, &(literal!("unit")))) => {
                let mut s: ArcStr;
                return getStringFromExp(metamodelica::AsArg::as_arg(&eqbind))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: varlist } => {
                let mut s: ArcStr;
                { invarlist = varlist; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return literal!("None")
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn getStringFromExp(mut binding: &metamodelica::Ref<DAE::Binding>) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match binding {
        Deref @ DAE::Binding::UNBOUND { .. } => {
            literal!("")
        },
        Deref @ DAE::Binding::EQBOUND { exp: Deref @ DAE::Exp::SCONST { string: str1 }, .. } => {
            str1.clone()
        },
        _ => {
            literal!("None")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#str
}

fn parse(
    mut inUnitString: ArcStr,
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inHtS2U: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHtU2S: (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    Unit::Unit,
    (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit, Unit::Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit::Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outUnit: Unit::Unit;
    let mut outHtS2U: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit::Unit)>>),
        i32,
        (
            HashTableStringToUnit::FuncHashKey,
            HashTableStringToUnit::FuncKeyEqual,
            HashTableStringToUnit::FuncKeyStr,
            HashTableStringToUnit::FuncValueStr,
        ),
    ) = inHtS2U.clone();
    let mut outHtU2S: (
        metamodelica::Array<metamodelica::List<(Unit::Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit::Unit, ArcStr)>>),
        i32,
        (
            HashTableUnitToString::FuncHashKey,
            HashTableUnitToString::FuncKeyEqual,
            HashTableUnitToString::FuncKeyStr,
            HashTableUnitToString::FuncValueStr,
        ),
    ) = inHtU2S;
    if metamodelica::stringEq(&inUnitString, &(literal!(""))) {
        outUnit = Unit::Unit::MASTER { varList: list![inCref] };
        return Ok((outUnit, outHtS2U, outHtU2S));
    }
    match '__try0: {
        outUnit = unwrap_break_err!(BaseHashTable::get(inUnitString.clone(), &inHtS2U), '__try0);
        Ok::<_, &'static str>((outUnit.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outUnit = __try0_o0;
        }
        Err(_) => {
            match '__try1: {
                outUnit = unwrap_break_err!(Unit::parseUnitString(inUnitString.clone(), &inHtS2U), '__try1);
                Ok::<_, &'static str>((outUnit.clone(),))
            } {
                Ok((__try1_o0,)) => {
                    outUnit = __try1_o0;
                }
                Err(_) => {
                    outUnit = Unit::Unit::UNKNOWN {
                        unit: inUnitString.clone(),
                    };
                }
            }
            outHtS2U = addUnit2HtS2U((inUnitString.clone(), outUnit.clone()), outHtS2U.clone())?;
            outHtU2S = addUnit2HtU2S(&((inUnitString.clone(), outUnit.clone())), outHtU2S.clone());
        }
    }
    Ok((outUnit, outHtS2U, outHtU2S))
}
