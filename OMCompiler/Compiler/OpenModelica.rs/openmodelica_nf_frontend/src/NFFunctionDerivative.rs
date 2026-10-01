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

use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInst as Inst;
use crate::NFInstContext as InstContext;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Variability;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTypeCheck::MatchKind;
use crate::NFTyping as Typing;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::MutableWeak;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NFFunctionDerivative {
    /// Weakly: the class tree owns the function
    ///      nodes; their caches hold the functions that name these back.
    pub derivativeFn: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    pub derivedFn: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    /// Is evaluated to a literal Integer during typing
    pub order: metamodelica::Ref<Expression::NFExpression>,
    pub conditions: metamodelica::List<(i32, ArcStr, Condition)>,
    pub lowerOrderDerivatives:
        metamodelica::List<Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>>,
}

impl metamodelica::gc::MMTrace for NFFunctionDerivative {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.derivativeFn, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.derivedFn, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.order, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.conditions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lowerOrderDerivatives, __mmv)?;
        Ok(())
    }
}
impl Default for NFFunctionDerivative {
    fn default() -> Self {
        Self {
            derivativeFn: Default::default(),
            derivedFn: Default::default(),
            order: Default::default(),
            conditions: Default::default(),
            lowerOrderDerivatives: Default::default(),
        }
    }
}

pub type FUNCTION_DER = NFFunctionDerivative;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum Condition {
    ZERO_DERIVATIVE = 1,
    NO_DERIVATIVE = 2,
}
impl PartialOrd for Condition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Condition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Condition {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for Condition {
    fn default() -> Self {
        Self::ZERO_DERIVATIVE
    }
}

pub(crate) fn instDerivatives(
    mut fnNode: metamodelica::Ref<InstNode::InstNode>,
    mut r#fn: &metamodelica::Ref<Function::Function>,
) -> Result<metamodelica::List<metamodelica::Ref<NFFunctionDerivative>>> {
    let mut ders: metamodelica::List<metamodelica::Ref<NFFunctionDerivative>> = metamodelica::nil();
    let mut der_mods: metamodelica::List<metamodelica::Ref<SCode::Mod>>;
    let mut scope: metamodelica::Ref<InstNode::InstNode>;
    der_mods = getDerivativeAnnotations(&(NFInstNode::InstNode::definition(fnNode.clone())?));
    scope = NFInstNode::InstNode::parent(&fnNode)?;
    for mut m in &*der_mods {
        ders = instDerivativeMod(
            metamodelica::AsArg::as_arg(&m),
            fnNode.clone(),
            r#fn,
            scope.clone(),
            ders,
        )?;
    }
    Ok(ders)
}

pub(crate) fn typeDerivative(mut fnDer: &metamodelica::Ref<NFFunctionDerivative>) -> Result<()> {
    let mut mk: MatchKind;
    let mut order: metamodelica::Ref<Expression::NFExpression>;
    let mut order_ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut info: SourceInfo;
    Function::typeNodeCache(
        NFInstNode::InstNode::borrow(fnDer.derivativeFn.clone())?,
        InstContext::FUNCTION.clone(),
    )?;
    info = NFInstNode::InstNode::info(&(NFInstNode::InstNode::borrow(fnDer.derivedFn.clone())?));
    (order, order_ty, var, _) = Typing::typeExp(fnDer.order.clone(), InstContext::FUNCTION.clone(), &info, false)?;
    (order, _, mk) = TypeCheck::matchTypes(
        order_ty.clone(),
        crate::NFType::interned_INTEGER(),
        order,
        TypeCheck::DEFAULT_OPTIONS.clone(),
    )?;
    if TypeCheck::isIncompatibleMatch(mk) {
        Error::addSourceMessage(
            &(Error::VARIABLE_BINDING_TYPE_MISMATCH.clone()),
            list![
                literal!("order"),
                Expression::toString(order.clone())?,
                literal!("Integer"),
                Type::toString(&order_ty)?
            ],
            &info,
        )?;
        return Err("fail");
    }
    if var > Variability::CONSTANT.clone() {
        Error::addSourceMessage(
            &(Error::HIGHER_VARIABILITY_BINDING.clone()),
            list![
                literal!("order"),
                Prefixes::variabilityString(Variability::CONSTANT.clone())?,
                Expression::toString(order.clone())?,
                Prefixes::variabilityString(var)?
            ],
            &info,
        )?;
        return Err("fail");
    }
    order = Ceval::evalExp(
        order,
        &(Ceval::EvalTarget::new(info, InstContext::NO_CONTEXT.clone(), None)),
    )?;
    Ok(())
}

pub(crate) fn toDAE(mut fnDer: &metamodelica::Ref<NFFunctionDerivative>) -> Result<DAE::FunctionDefinition> {
    let mut derDef: DAE::FunctionDefinition;
    let mut order: i32;
    let __pa0 = ::match_deref::match_deref! { match &(fnDer.order.clone()) {
        Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    order = metamodelica::Own::own(__pa0);
    derDef = DAE::FunctionDefinition::FUNCTION_DER_MAPPER {
        derivedFunction: Function::name(
            &((Function::getCachedFuncs(NFInstNode::InstNode::borrow(fnDer.derivedFn.clone())?)?)
                .head()
                .cloned()?),
        ),
        derivativeFunction: Function::name(
            &((Function::getCachedFuncs(NFInstNode::InstNode::borrow(fnDer.derivativeFn.clone())?)?)
                .head()
                .cloned()?),
        ),
        derivativeOrder: order,
        conditionRefs: ({
            let mut __acc: metamodelica::List<(i32, DAE::derivativeCond)> = metamodelica::nil();
            for mut c in (fnDer.conditions.clone()).into_iter().cloned() {
                let __x = conditionToDAE(&(c.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        defaultDerivative: None,
        lowerOrderDerivatives: ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
            for mut r#fn in (fnDer.lowerOrderDerivatives.clone()).into_iter().cloned() {
                let __x = Function::name(
                    &((Function::getCachedFuncs(NFInstNode::InstNode::borrow(r#fn.clone())?)?)
                        .head()
                        .cloned()?),
                );
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    };
    Ok(derDef)
}

pub(crate) fn conditionToDAE(mut cond: &(i32, ArcStr, Condition)) -> Result<(i32, DAE::derivativeCond)> {
    let mut daeCond: (i32, DAE::derivativeCond);
    let mut idx: i32;
    let mut c: Condition;
    (idx, _, c) = cond.clone();
    daeCond = (match c {
        Condition::ZERO_DERIVATIVE => (idx, openmodelica_frontend_types::DAE::derivativeCond::ZERO_DERIVATIVE),
        Condition::NO_DERIVATIVE { .. } => (
            idx,
            DAE::derivativeCond::NO_DERIVATIVE {
                binding: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 99 }),
            },
        ),
    });
    Ok(daeCond)
}

pub(crate) fn toSubMod(
    mut fnDer: &metamodelica::Ref<NFFunctionDerivative>,
) -> Result<metamodelica::Ref<SCode::SubMod>> {
    let mut subMod: metamodelica::Ref<SCode::SubMod>;
    let mut tpl: (i32, Condition) = (0, Condition::ZERO_DERIVATIVE);
    let mut condition: Condition;
    let mut id: ArcStr;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let mut orderMod: metamodelica::Ref<SCode::SubMod>;
    let mut subMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut order: i32;
    let mut info: SourceInfo;
    let mut func: metamodelica::Ref<Function::Function>;
    info = NFInstNode::InstNode::info(&(NFInstNode::InstNode::borrow(fnDer.derivedFn.clone())?));
    let __pa0 = ::match_deref::match_deref! { match &(fnDer.order.clone()) {
        Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    order = metamodelica::Own::own(__pa0);
    orderMod = metamodelica::Ref::new(SCode::SubMod {
        ident: literal!("order"),
        r#mod: metamodelica::Ref::new(SCode::Mod::MOD {
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
            subModLst: metamodelica::nil(),
            binding: Some(metamodelica::Ref::new(Absyn::Exp::INTEGER { value: order })),
            comment: None,
            info: info.clone(),
        }),
    });
    subMods = metamodelica::nil();
    for mut tpl in &*fnDer.conditions.clone() {
        let mut tpl = tpl.clone();
        (_, id, condition) = tpl;
        subMods = metamodelica::cons(
            metamodelica::Ref::new(SCode::SubMod {
                ident: conditionToString(condition),
                r#mod: metamodelica::Ref::new(SCode::Mod::MOD {
                    finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                    eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                    subModLst: metamodelica::nil(),
                    binding: Some(metamodelica::Ref::new(Absyn::Exp::CREF {
                        componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                            name: id,
                            subscripts: metamodelica::nil(),
                        }),
                    })),
                    comment: None,
                    info: info.clone(),
                }),
            }),
            subMods,
        );
    }
    func = (Function::getCachedFuncs(NFInstNode::InstNode::borrow(fnDer.derivativeFn.clone())?)?)
        .head()
        .cloned()?;
    r#mod = metamodelica::Ref::new(SCode::Mod::MOD {
        finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
        eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
        subModLst: metamodelica::cons(orderMod, subMods),
        binding: Some(metamodelica::Ref::new(Absyn::Exp::CREF {
            componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                name: AbsynUtil::pathString(func.path.clone(), literal!("."), true, false)?,
                subscripts: metamodelica::nil(),
            }),
        })),
        comment: None,
        info: info,
    });
    subMod = metamodelica::Ref::new(SCode::SubMod {
        ident: literal!("derivative"),
        r#mod: r#mod,
    });
    Ok(subMod)
}

pub(crate) fn perfectFit(
    mut fnDer: &metamodelica::Ref<NFFunctionDerivative>,
    mut interface_map: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, bool>>,
) -> Result<bool> {
    let mut b: bool = true;
    let mut name: ArcStr;
    let mut cond: Condition;
    for mut condition in &*fnDer.conditions.clone() {
        (_, name, cond) = condition.clone();
        if cond == Condition::ZERO_DERIVATIVE.clone() && !(UnorderedMap::contains(name, interface_map.clone())?) {
            b = false;
            return Ok(b);
        }
    }
    for mut condition in &*fnDer.conditions.clone() {
        (_, name, _) = condition.clone();
        UnorderedMap::add(name, true, interface_map.clone())?;
    }
    Ok(b)
}

pub fn conditionsFromMap(
    mut interface_map: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, bool>>,
) -> metamodelica::List<(i32, ArcStr, Condition)> {
    let mut conditions: metamodelica::List<(i32, ArcStr, Condition)> = metamodelica::nil();
    let mut name: ArcStr;
    let mut isZeroDer: bool;
    for mut tpl in &*UnorderedMap::toList(interface_map) {
        (name, isZeroDer) = tpl.clone();
        if isZeroDer {
            conditions = metamodelica::cons((0, name, Condition::ZERO_DERIVATIVE.clone()), conditions);
        }
    }
    conditions
}

fn conditionToString(mut condition: Condition) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match condition {
        Condition::NO_DERIVATIVE { .. } => literal!("noDerivative"),
        Condition::ZERO_DERIVATIVE => literal!("zeroDerivative"),
        _ => ArcStr::from(::std::format!("{:?}", condition)),
    });
    r#str
}

fn getDerivativeAnnotations(
    mut definition: &metamodelica::Ref<SCode::Element>,
) -> metamodelica::List<metamodelica::Ref<SCode::Mod>> {
    let mut derMods: metamodelica::List<metamodelica::Ref<SCode::Mod>>;
    derMods = (::match_deref::match_deref! { match definition {
        Deref @ SCode::Element::CLASS { cmt: Deref @ SCode::Comment { annotation_: Some(ann), .. }, .. } => {
            SCodeUtil::lookupAnnotations(metamodelica::AsArg::as_arg(&ann), &(literal!("derivative")))
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    derMods
}

fn instDerivativeMod(
    mut r#mod: &metamodelica::Ref<SCode::Mod>,
    mut fnNode: metamodelica::Ref<InstNode::InstNode>,
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut fnDers: metamodelica::List<metamodelica::Ref<NFFunctionDerivative>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFFunctionDerivative>>> {
    let mut fnDers: metamodelica::List<metamodelica::Ref<NFFunctionDerivative>> = fnDers;
    fnDers = (::match_deref::match_deref! { match r#mod {
        Deref @ SCode::Mod::MOD { subModLst: attrs, binding: Some(Deref @ Absyn::Exp::CREF { componentRef: acref }), info: __mod_info, .. } => {
            let mut der_node: metamodelica::Ref<InstNode::InstNode>;
            let mut order: metamodelica::Ref<Expression::NFExpression>;
            let mut conds: metamodelica::List<(i32, ArcStr, Condition)>;
            (_, der_node, _) = Function::instFunction(acref.clone(), scope, InstContext::NO_CONTEXT.clone(), __mod_info.clone())?;
            addLowerOrderDerivative(der_node.clone(), &fnNode)?;
            (order, conds) = getDerivativeAttributes(attrs, r#fn, &fnNode, metamodelica::AsArg::as_arg(&__mod_info))?;
            metamodelica::cons(metamodelica::Ref::new(NFFunctionDerivative { derivativeFn: NFInstNode::InstNode::identityCell(der_node), derivedFn: NFInstNode::InstNode::identityCell(fnNode), order: order, conditions: conds, lowerOrderDerivatives: metamodelica::nil() }), fnDers)
        },
        Deref @ SCode::Mod::MOD { info: __mod_info, .. } => {
            Error::addStrictMessage(Error::MISSING_FUNCTION_DERIVATIVE_NAME.clone(), list![AbsynUtil::pathString(Function::name(r#fn), literal!("."), true, false)?], metamodelica::AsArg::as_arg(&__mod_info))?;
            fnDers
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunctionDerivative.instDerivativeMod")); __mm_s.push_str(&*literal!(" got invalid modifier")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFFunctionDerivative.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(fnDers)
}

fn getDerivativeAttributes(
    mut attrs: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::List<(i32, ArcStr, Condition)>,
)> {
    let mut order: metamodelica::Ref<Expression::NFExpression> =
        metamodelica::Ref::new(Expression::NFExpression::EMPTY {
            ty: crate::NFType::interned_UNKNOWN(),
        });
    let mut conditions: metamodelica::List<(i32, ArcStr, Condition)> = metamodelica::nil();
    let mut id: ArcStr;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let mut aexp: metamodelica::Ref<Absyn::Exp>;
    let mut index: i32;
    for mut attr in &**attrs {
        let __arc2 = attr.clone();
        let SCode::SubMod {
            ident: __pa0,
            r#mod: __pa1,
        } = &*__arc2;
        id = metamodelica::Own::own(__pa0);
        r#mod = metamodelica::Own::own(__pa1);
        let () = (::match_deref::match_deref! { match &((id.clone(), r#mod.clone())) {
            (Deref @ "order", Deref @ SCode::Mod::MOD { binding: Some(__esc_aexp), .. }) => {
                aexp = (*__esc_aexp).clone();
                if !(Expression::isEmpty(&order)) {
                    Error::addSourceMessage(&(Error::DUPLICATE_MODIFICATIONS.clone()), list![id, literal!("derivative")], info)?;
                }
                order = Inst::instExp(aexp.clone(), scope, InstContext::NO_CONTEXT.clone(), info)?;
                ()
            },
            (Deref @ "noDerivative", Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_id, .. } }), .. }) => {
                id = (*__esc_id).clone();
                index = getInputIndex(id.clone(), r#fn, info)?;
                conditions = metamodelica::cons((index, id.clone(), Condition::NO_DERIVATIVE.clone()), conditions);
                ()
            },
            (Deref @ "zeroDerivative", Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_id, .. } }), .. }) => {
                id = (*__esc_id).clone();
                index = getInputIndex(id.clone(), r#fn, info)?;
                conditions = metamodelica::cons((index, id.clone(), Condition::ZERO_DERIVATIVE.clone()), conditions);
                ()
            },
            _ => {
                Error::addStrictMessage(Error::INVALID_FUNCTION_ANNOTATION_ATTR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*id); __mm_s.push_str(&*if (SCodeUtil::isEmptyMod(&r#mod)) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*SCodeDump::printModStr(r#mod, SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) }}); ArcStr::from(__mm_s) }, literal!("derivative")], info)?;
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    if Expression::isEmpty(&order) {
        order = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 });
    }
    Ok((order, conditions))
}

fn getInputIndex(
    mut name: ArcStr,
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut info: &SourceInfo,
) -> Result<i32> {
    let mut index: i32 = 1;
    for mut i in &*r#fn.inputs.clone() {
        if metamodelica::stringEq(&(NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&i))?), &name) {
            return Ok(index);
        }
        index = index + 1;
    }
    Error::addSourceMessage(
        &(Error::INVALID_FUNCTION_ANNOTATION_INPUT.clone()),
        list![
            name,
            AbsynUtil::pathString(Function::name(r#fn), literal!("."), true, false)?
        ],
        info,
    )?;
    return Err("fail");
    Ok(index)
}

fn addLowerOrderDerivative(
    mut fnNode: metamodelica::Ref<InstNode::InstNode>,
    mut lowerDerNode: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<()> {
    Function::mapCachedFuncs(
        fnNode,
        &({
            let __pe_b1 = lowerDerNode.clone();
            move |__pe_a0| Ok(addLowerOrderDerivative2(__pe_a0, __pe_b1.clone()))
        }),
    )?;
    Ok(())
}

fn addLowerOrderDerivative2(
    mut r#fn: metamodelica::Ref<Function::Function>,
    mut lowerDerNode: metamodelica::Ref<InstNode::InstNode>,
) -> metamodelica::Ref<Function::Function> {
    let mut r#fn: metamodelica::Ref<Function::Function> = r#fn;
    assign_field!(
        r#fn.derivatives = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<NFFunctionDerivative>> = metamodelica::nil();
            for mut fn_der in (r#fn.derivatives.clone()).into_iter().cloned() {
                let __x = (match &*fn_der.clone() {
                    NFFunctionDerivative { .. } => {
                        assign_field!(
                            fn_der.lowerOrderDerivatives = metamodelica::cons(
                                NFInstNode::InstNode::identityCell(lowerDerNode.clone()),
                                fn_der.lowerOrderDerivatives.clone()
                            )
                        );
                        fn_der.clone()
                    }
                });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    r#fn
}
