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

use crate::NBAdjacency::Dependency;
use crate::NBAdjacency::IntMatrix;
use crate::NBAdjacency::Mapping;
use crate::NBAdjacency::Mode;
use crate::NBAdjacency::ModeTable;
use crate::NBAdjacency::Modes;
use crate::NBBackendUtil as BackendUtil;
use crate::NBEquation::Equation;
use crate::NBEquation::Frame;
use crate::NBEquation::FrameLocation;
use crate::NBEquation::FrameOrderingStatus;
use crate::NBEquation::Iterator;
use crate::NBEquation::RecollectStatus;
use crate::NBReplacements as Replacements;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointers;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComplexType as ComplexType;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes::Variability;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:         NBSlice.mo
///  package:      NBSlice
///  description:  This file contains util functions for slicing operations.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NBSlice<T: Clone> {
    pub t: T,
    pub indices: IntLst,
}

impl<T: Clone + metamodelica::gc::MMTrace> metamodelica::gc::MMTrace for NBSlice<T> {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.t, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.indices, __mmv)?;
        Ok(())
    }
}
pub type SLICE<T> = NBSlice<T>;

pub type IntLst = metamodelica::List<i32>;

pub(crate) fn getT<T: Clone + 'static + metamodelica::gc::MMTrace>(mut slice: metamodelica::Ref<NBSlice<T>>) -> T {
    let mut t: T = slice.t.clone();
    t
}

pub(crate) fn hash<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut slice: metamodelica::Ref<NBSlice<T>>,
    mut func: &dyn ::std::ops::Fn(T) -> Result<i32>,
) -> Result<i32> {
    let mut h: i32 = func(slice.t.clone())?;
    for mut i in &*List::firstOrEmpty(&slice.indices) {
        h = stringHashDjb2Continue(&(intString(i.clone())), h);
    }
    Ok(h)
}

pub(crate) fn isEqual<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut slice1: metamodelica::Ref<NBSlice<T>>,
    mut slice2: metamodelica::Ref<NBSlice<T>>,
    mut func: &dyn ::std::ops::Fn(T, T) -> Result<bool>,
) -> Result<bool> {
    let mut b: bool = func(slice1.t.clone(), slice2.t.clone())?
        && List::isEqualOnTrue(slice1.indices.clone(), slice2.indices.clone(), &fnptr!(intEq, i32, i32))?;
    Ok(b)
}

pub(crate) fn toString<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut slice: metamodelica::Ref<NBSlice<T>>,
    mut func: &dyn ::std::ops::Fn(T) -> Result<ArcStr>,
    mut maxLength: i32,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = func(slice.t.clone())?;
    if maxLength > 0 && !((slice.indices).is_empty()) {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("\n\tslice: "));
            __mm_s.push_str(&*List::toStringCustom(
                slice.indices.clone(),
                &fnptr!(intString, i32),
                literal!(""),
                literal!("{"),
                literal!(", "),
                literal!("}"),
                true,
                maxLength,
            )?);
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

pub(crate) fn lstToString<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut lst: metamodelica::List<metamodelica::Ref<NBSlice<T>>>,
    mut func: toStringT<T>,
    mut indent: ArcStr,
    mut maxLength: i32,
) -> Result<ArcStr> {
    pub use toStringT as toStringT_;

    let mut r#str: ArcStr = List::toStringCustom(
        lst.clone(),
        &({
            let __pe_b1 = func.clone();
            let __pe_b2 = maxLength;
            move |__pe_a0| toString(__pe_a0, &*__pe_b1, __pe_b2.clone())
        }),
        literal!(""),
        indent.clone(),
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*indent);
            ArcStr::from(__mm_s)
        },
        literal!(""),
        false,
        0,
    )?;
    Ok(r#str)
}

pub(crate) fn isFull<T: Clone + 'static + metamodelica::gc::MMTrace>(mut slice: metamodelica::Ref<NBSlice<T>>) -> bool {
    let mut b: bool = (slice.indices).is_empty();
    b
}

pub(crate) fn size<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut slice: metamodelica::Ref<NBSlice<T>>,
    mut func: &dyn ::std::ops::Fn(T) -> Result<i32>,
) -> Result<i32> {
    let mut s: i32;
    if (slice.indices).is_empty() {
        s = func(slice.t.clone())?;
    } else {
        s = ((slice.indices).len() as i32);
    }
    Ok(s)
}

pub(crate) fn simplify<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut slice: metamodelica::Ref<NBSlice<T>>,
    mut func: &dyn ::std::ops::Fn(T) -> Result<i32>,
) -> Result<metamodelica::Ref<NBSlice<T>>> {
    let mut slice: metamodelica::Ref<NBSlice<T>> = slice;
    if ((slice.indices).len() as i32) == func(slice.t.clone())? {
        assign_field!(slice.indices = metamodelica::nil());
    } else {
        assign_field!(
            slice.indices = List::sort(
                slice.indices.clone(),
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)
            )?
        );
    }
    Ok(slice)
}

pub(crate) fn addToSliceMap<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut t: T,
    mut i: i32,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<T, metamodelica::List<i32>>>,
) -> Result<()> {
    UnorderedMap::add(
        t.clone(),
        metamodelica::cons(i, UnorderedMap::getOrDefault(t, map.clone(), metamodelica::nil())?),
        map,
    )?;
    Ok(())
}

pub(crate) fn fromTpl<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut tpl: &(T, metamodelica::List<i32>),
) -> metamodelica::Ref<NBSlice<T>> {
    let mut slice: metamodelica::Ref<NBSlice<T>>;
    let mut t: T;
    let mut lst: IntLst;
    (t, lst) = tpl.clone();
    slice = metamodelica::Ref::new(NBSlice { t: t, indices: lst });
    slice
}

pub(crate) fn fromMap<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<T, metamodelica::List<i32>>>,
) -> metamodelica::List<metamodelica::Ref<NBSlice<T>>> {
    let mut slices: metamodelica::List<metamodelica::Ref<NBSlice<T>>> = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut tpl in (UnorderedMap::toList(map.clone())).into_iter().cloned() {
            let __x = fromTpl(&(tpl.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    slices
}

pub(crate) fn apply<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut slice: metamodelica::Ref<NBSlice<T>>,
    mut func: &dyn ::std::ops::Fn(T) -> Result<T>,
) -> Result<metamodelica::Ref<NBSlice<T>>> {
    let mut slice: metamodelica::Ref<NBSlice<T>> = slice;
    assign_field!(slice.t = func(slice.t.clone())?);
    Ok(slice)
}

pub(crate) fn applyMutable<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut slice: metamodelica::Ref<NBSlice<T>>,
    mut func: &dyn ::std::ops::Fn(T) -> Result<()>,
) -> Result<()> {
    func(slice.t.clone())?;
    Ok(())
}

pub(crate) fn check<T: Clone + 'static + metamodelica::gc::MMTrace, T2: Clone + 'static + metamodelica::gc::MMTrace>(
    mut slice: metamodelica::Ref<NBSlice<T>>,
    mut func: &dyn ::std::ops::Fn(T) -> Result<T2>,
) -> Result<T2> {
    let mut t2: T2 = func(slice.t.clone())?;
    Ok(t2)
}

pub type toStringT<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

pub type sizeT<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>;

pub type hashT<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>;

pub type isEqualT<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

pub type applyT<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>;

pub type applyMutableT<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<()> + 'static>;

pub type checkT<T2: Clone + 'static, T: Clone + 'static> =
    std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<T2> + 'static>;

pub type filterCref = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
        + 'static,
>;

pub type getDependentCrefIndices = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
            metamodelica::Ref<Mapping::Mapping>,
            i32,
        ) -> Result<(
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<metamodelica::Array<i32>>,
        )> + 'static,
>;

pub(crate) fn filterExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut filter: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
            ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
            + 'static,
    >,
    mut acc: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ Expression::CREF { cref: __exp_cref, .. } => {
            filter(__exp_cref.clone(), acc)?;
            ()
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { exp: call_exp, .. } } => {
            let mut call_exp = (*call_exp).clone();
            for mut iter in &*var_field!((**call).iters, Call::NFCall::TYPED_REDUCTION).clone() {
                call_exp = Expression::replaceIterator(call_exp.clone(), &(Util::tuple21(iter.clone())), &(Util::tuple22(iter.clone())))?;
            }
            Expression::mapShallow(call_exp.clone(), (std::sync::Arc::new({ let __pe_b1 = filter.clone(); let __pe_b2 = acc; move |__pe_a0| filterExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            ()
        },
        _ => {
            Expression::mapShallow(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = filter.clone(); let __pe_b2 = acc; move |__pe_a0| filterExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn getContinuous(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut acc: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut init: bool,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    if BVariable::checkCref(
        &cref,
        &({
            let __pe_b1 = init;
            move |__pe_a0| BVariable::isContinuous(__pe_a0, __pe_b1.clone())
        }),
        metamodelica::sourceInfo!("NBackEnd/Util/NBSlice.mo"),
    )? {
        UnorderedSet::add(cref.clone(), acc)?;
    }
    Ok(cref)
}

pub(crate) fn getSliceCandidates(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut acc: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut checkCref: metamodelica::Ref<ComponentRef::NFComponentRef> = ComponentRef::stripSubscriptsAll(&cref);
    if ComponentRef::isEqual(&name, &checkCref)?
        || ComponentRef::isEqual(&name, &cref)?
        || ComponentRef::isEqualRecordChild(&name, &checkCref)?
    {
        UnorderedSet::add(cref.clone(), acc)?;
    }
    Ok(cref)
}

pub(crate) fn resolveSlicedCref(
    mut base_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut target_size: i32,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = base_cref.clone();
    let mut candidates: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut filtered: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    if target_size > 0 {
        candidates = Equation::collectCrefs(
            eqn,
            (std::sync::Arc::new({
                let __pe_b2 = base_cref;
                move |__pe_a0, __pe_a1| getSliceCandidates(__pe_a0, __pe_a1, __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            metamodelica::Ref<
                                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                            >,
                        )
                            -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                        + 'static,
                >),
            (std::sync::Arc::new(Expression::map)
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
        filtered = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut c in (candidates).into_iter().cloned() {
                if !(Type::sizeOf(&(ComponentRef::getSubscriptedType(&(c.clone()), false)?), true)? == target_size) {
                    continue;
                }
                let __x = c.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        if List::hasOneElement(&filtered) {
            cref = (filtered).head().cloned()?;
        }
    }
    Ok(cref)
}

pub(crate) fn getDependentCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut acc: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut pseudo: bool,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut checkCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut childCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut record_children: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    checkCref = if (pseudo) {
        ComponentRef::stripSubscriptsAll(&cref)
    } else {
        cref.clone()
    };
    record_children = BVariable::getRecordChildren(BVariable::getVarPointer(
        &checkCref,
        metamodelica::sourceInfo!("NBackEnd/Util/NBSlice.mo"),
    )?)?;
    if (record_children).is_empty() {
        if UnorderedMap::contains(checkCref, map)? {
            UnorderedSet::add(cref.clone(), acc)?;
        }
    } else {
        for mut child in &*record_children {
            childCref = BVariable::getVarName(child.clone());
            if UnorderedMap::contains(childCref.clone(), map.clone())? {
                UnorderedSet::add(childCref, acc.clone())?;
            }
        }
    }
    Ok(cref)
}

pub(crate) fn getDependentCrefCausalized(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut acc: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut checkCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut childCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut record_children: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    checkCref = ComponentRef::stripSubscriptsAll(&cref);
    record_children = BVariable::getRecordChildren(BVariable::getVarPointer(
        &checkCref,
        metamodelica::sourceInfo!("NBackEnd/Util/NBSlice.mo"),
    )?)?;
    if (record_children).is_empty() {
        if UnorderedSet::contains(checkCref, set)? {
            UnorderedSet::add(cref.clone(), acc)?;
        }
    } else {
        for mut child in &*record_children {
            childCref = BVariable::getVarName(child.clone());
            if UnorderedSet::contains(childCref.clone(), set.clone())? {
                UnorderedSet::add(childCref, acc.clone())?;
            }
        }
    }
    Ok(cref)
}

pub(crate) fn getUnsolvableExpCrefs(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut acc: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut pseudo: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::RANGE { .. } => Expression::mapShallow(
            exp,
            (std::sync::Arc::new({
                let __pe_b1 = (std::sync::Arc::new({
                    let __pe_b2 = map;
                    let __pe_b3 = pseudo;
                    move |__pe_a0, __pe_a1| getDependentCref(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<
                                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                >,
                            )
                                -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                            + 'static,
                    >);
                let __pe_b2 = acc;
                move |__pe_a0| filterExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
        Expression::LBINARY { .. } => Expression::mapShallow(
            exp,
            (std::sync::Arc::new({
                let __pe_b1 = (std::sync::Arc::new({
                    let __pe_b2 = map;
                    let __pe_b3 = pseudo;
                    move |__pe_a0, __pe_a1| getDependentCref(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<
                                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                >,
                            )
                                -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                            + 'static,
                    >);
                let __pe_b2 = acc;
                move |__pe_a0| filterExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
        Expression::RELATION { .. } => Expression::mapShallow(
            exp,
            (std::sync::Arc::new({
                let __pe_b1 = (std::sync::Arc::new({
                    let __pe_b2 = map;
                    let __pe_b3 = pseudo;
                    move |__pe_a0, __pe_a1| getDependentCref(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<
                                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                >,
                            )
                                -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                            + 'static,
                    >);
                let __pe_b2 = acc;
                move |__pe_a0| filterExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
        _ => exp,
    });
    Ok(exp)
}

pub(crate) fn getDependentCrefIndicesPseudoScalar(
    mut dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
) -> Result<metamodelica::List<i32>> {
    let mut indices: metamodelica::List<i32> = metamodelica::nil();
    let mut scalarized_dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
        List::flatten(
            ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
                    metamodelica::nil();
                for mut dep in (dependencies.clone()).into_iter().cloned() {
                    let __x = ComponentRef::scalarizeAll(dep.clone(), true)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?;
    let mut stripped: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut var_arr_idx: i32;
    let mut var_start: i32;
    let mut var_scal_idx: i32;
    let mut sizes: metamodelica::List<i32>;
    let mut int_subs: metamodelica::List<i32>;
    for mut cref in &*scalarized_dependencies {
        stripped = ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&cref));
        var_arr_idx = UnorderedMap::getSafe(
            stripped.clone(),
            map.clone(),
            metamodelica::sourceInfo!("NBackEnd/Util/NBSlice.mo"),
        )?;
        (var_start, _) = ({
            let __elt = (*metamodelica::index_checked(&mapping.var_AtS.borrow(), var_arr_idx)?).clone();
            __elt
        });
        sizes = ComponentRef::sizes(&stripped, false, false, metamodelica::nil())?;
        int_subs = ComponentRef::subscriptsToInteger(metamodelica::AsArg::as_arg(&cref))?;
        var_scal_idx = locationToIndex(sizes, int_subs, var_start)?;
        indices = metamodelica::cons(var_scal_idx, indices);
    }
    if !((indices).is_empty()) {
        indices = List::sort(
            List::uniqueIntN(
                &(indices.clone()),
                ({
                    let mut __acc: Option<i32> = None;
                    for mut i in (indices).into_iter().cloned() {
                        let __x = i.clone();
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
                }),
            )?,
            (std::sync::Arc::new(fnptr!(intLt, i32, i32))
                as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        )?;
    }
    Ok(indices)
}

pub(crate) fn getDependentCrefIndicesPseudoFull(
    mut dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut mapping: metamodelica::Ref<Mapping::Mapping>,
    mut eqn_arr_idx: i32,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::Array<i32>>,
)> {
    let mut indices: metamodelica::Array<metamodelica::List<i32>>;
    let mut mode_to_var: metamodelica::Array<metamodelica::Array<i32>>;
    let mut scalarized_dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
        List::flatten(
            ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
                    metamodelica::nil();
                for mut dep in (dependencies.clone()).into_iter().cloned() {
                    let __x = ComponentRef::scalarizeAll(dep.clone(), true)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?;
    let mut stripped: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut eqn_start: i32;
    let mut eqn_size: i32;
    let mut var_arr_idx: i32;
    let mut var_scal_idx: i32 = 0;
    let mut mode: i32 = 1;
    let mut scal_lst: metamodelica::List<i32>;
    let mut idx: i32;
    let mut mode_to_var_row: metamodelica::Array<i32>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    (eqn_start, eqn_size) = ({
        let __elt = (*metamodelica::index_checked(&mapping.eqn_AtS.borrow(), eqn_arr_idx)?).clone();
        __elt
    });
    indices = arrayCreate(eqn_size, metamodelica::nil());
    mode_to_var = arrayCreate(eqn_size, arrayCreate(0, 0));
    for mut i in 1..=eqn_size {
        {
            let __cell0 = arrayCreate(((scalarized_dependencies).len() as i32), -1);
            let __idx0 = i;
            *metamodelica::index_mut_checked(&mut mode_to_var.clone().borrow_mut(), __idx0)? = __cell0;
        }
    }
    for mut cref in &*scalarized_dependencies {
        stripped = ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&cref));
        var_arr_idx = UnorderedMap::getSafe(
            stripped.clone(),
            map.clone(),
            metamodelica::sourceInfo!("NBackEnd/Util/NBSlice.mo"),
        )?;
        subs = ComponentRef::subscriptsAllWithWholeFlat(metamodelica::AsArg::as_arg(&cref))?;
        ty = ComponentRef::getSubscriptedType(&stripped, true)?;
        dims = Type::arrayDims(ty);
        scal_lst = Mapping::getVarScalIndices(var_arr_idx, &mapping, &subs, dims, true)?;
        if intMod(eqn_size, ((scal_lst).len() as i32)) != 0 {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBSlice.getDependentCrefIndicesPseudoFull"));
                    __mm_s.push_str(&*literal!(" failed because flattened indices "));
                    __mm_s.push_str(&*intString(((scal_lst).len() as i32)));
                    __mm_s.push_str(&*literal!(" could not be repeated to fit equation size "));
                    __mm_s.push_str(&*intString(eqn_size));
                    __mm_s.push_str(&*literal!(". lst: "));
                    __mm_s.push_str(&*List::toString(
                        scal_lst.clone(),
                        &fnptr!(intString, i32),
                        List::Style::FLAT_CURLY.clone(),
                    )?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        } else {
            scal_lst = List::repeat(scal_lst.clone(), intDiv(eqn_size, ((scal_lst).len() as i32)));
        }
        idx = 1;
        for mut var_scal_idx in &*scal_lst.reverse() {
            let mut var_scal_idx = var_scal_idx.clone();
            mode_to_var_row = ({
                let __elt = (*metamodelica::index_checked(&mode_to_var.borrow(), idx)?).clone();
                __elt
            });
            metamodelica::arrayUpdate(mode_to_var_row.clone(), mode, var_scal_idx)?;
            metamodelica::arrayUpdate(mode_to_var.clone(), idx, mode_to_var_row.clone())?;
            {
                let __cell1 = metamodelica::cons(
                    var_scal_idx,
                    ({
                        let __elt = (*metamodelica::index_checked(&indices.borrow(), idx)?).clone();
                        __elt
                    }),
                );
                let __idx1 = idx;
                *metamodelica::index_mut_checked(&mut indices.clone().borrow_mut(), __idx1)? = __cell1;
            }
            idx = idx + 1;
        }
        mode = mode + 1;
    }
    for mut i in 1..=metamodelica::arrayLength(indices.clone()) {
        {
            let __cell2 = List::sort(
                UnorderedSet::unique_list(
                    ({
                        let __elt = (*metamodelica::index_checked(&indices.borrow(), i)?).clone();
                        __elt
                    }),
                    std::sync::Arc::new(fnptr!(Util::id, _)),
                    (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                        as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                )?,
                (std::sync::Arc::new(fnptr!(intLt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
            let __idx2 = i;
            *metamodelica::index_mut_checked(&mut indices.clone().borrow_mut(), __idx2)? = __cell2;
        }
    }
    Ok((indices, mode_to_var))
}

pub(crate) fn getDependentCrefIndicesPseudoFor(
    mut dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut mapping: metamodelica::Ref<Mapping::Mapping>,
    mut eqn_arr_idx: i32,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::Array<i32>>,
)> {
    let mut indices: metamodelica::Array<metamodelica::List<i32>>;
    let mut mode_to_var: metamodelica::Array<metamodelica::Array<i32>>;
    let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator::Iterator>>>;
    let mut frames: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>;
    let mut eqn_size: i32;
    let mut iter_size: i32;
    let mut body_size: i32;
    let mut mode: i32 = 1;
    let mut func: updateDependencies;
    iter_size = Iterator::size(&iter, false)?;
    (names, ranges, maps) = Iterator::getFrames(&iter);
    frames = List::zip3(names, ranges, maps);
    (_, eqn_size) = ({
        let __elt = (*metamodelica::index_checked(&mapping.eqn_AtS.borrow(), eqn_arr_idx)?).clone();
        __elt
    });
    indices = arrayCreate(eqn_size, metamodelica::nil());
    mode_to_var = arrayCreate(eqn_size, arrayCreate(0, 0));
    if intMod(eqn_size, iter_size) == 0 {
        body_size = intDiv(eqn_size, iter_size);
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBSlice.getDependentCrefIndicesPseudoFor"));
                __mm_s.push_str(&*literal!(" failed because the equation size "));
                __mm_s.push_str(&*intString(eqn_size));
                __mm_s.push_str(&*literal!(" could not be divided by the iterator size "));
                __mm_s.push_str(&*intString(iter_size));
                __mm_s.push_str(&*literal!(" without rest."));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    for mut i in 1..=eqn_size {
        {
            let __cell0 = arrayCreate(((dependencies).len() as i32), -1);
            let __idx0 = i;
            *metamodelica::index_mut_checked(&mut mode_to_var.clone().borrow_mut(), __idx0)? = __cell0;
        }
    }
    for mut dep in &*dependencies {
        func = (std::sync::Arc::new({
            let __pe_b3 = mode;
            let __pe_b4 = mode_to_var.clone();
            let __pe_b5 = indices.clone();
            move |__pe_a0, __pe_a1, __pe_a2| {
                updateDependenciesInteger(
                    __pe_a0,
                    __pe_a1,
                    __pe_a2,
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                )
            }
        }) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32) -> Result<i32> + 'static>);
        fillDependencyArray(
            dep.clone(),
            body_size,
            &frames,
            &mapping,
            map.clone(),
            &*(func.clone()),
            0,
            true,
        )?;
        mode = mode + 1;
    }
    for mut i in 1..=metamodelica::arrayLength(indices.clone()) {
        {
            let __cell1 = List::sort(
                UnorderedSet::unique_list(
                    ({
                        let __elt = (*metamodelica::index_checked(&indices.borrow(), i)?).clone();
                        __elt
                    }),
                    std::sync::Arc::new(fnptr!(Util::id, _)),
                    (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                        as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                )?,
                (std::sync::Arc::new(fnptr!(intLt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
            let __idx1 = i;
            *metamodelica::index_mut_checked(&mut indices.clone().borrow_mut(), __idx1)? = __cell1;
        }
    }
    Ok((indices, mode_to_var))
}

pub(crate) fn getDependentCrefsPseudoForCausalized(
    mut row_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut dependencies: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut var_rep: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut eqn_rep: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut var_rep_mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut eqn_rep_mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
    mut eqn_size: i32,
    mut slice: metamodelica::List<i32>,
    mut implicit: bool,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    )>,
> {
    let mut tpl_lst: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    )>;
    let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator::Iterator>>>;
    let mut frames: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>;
    let mut iter_size: i32;
    let mut body_size: i32;
    let mut var_arr_idx: i32;
    let mut row_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut row_scal_lst: metamodelica::List<i32>;
    let mut accum_row_lst: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut accum_dep_arr: metamodelica::Array<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut accum_dep_lst: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut func_var: updateDependencies;
    let mut func_eqn: updateDependencies;
    let mut final_dep: metamodelica::Ref<ComponentRef::NFComponentRef>;
    accum_dep_arr = arrayCreate(eqn_size, metamodelica::nil());
    iter_size = Iterator::size(iter, false)?;
    (names, ranges, maps) = Iterator::getFrames(iter);
    frames = List::zip3(names, ranges, maps);
    if intMod(eqn_size, iter_size) == 0 {
        body_size = intDiv(eqn_size, iter_size);
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBSlice.getDependentCrefsPseudoForCausalized"));
                __mm_s.push_str(&*literal!(" failed because the equation size "));
                __mm_s.push_str(&*intString(eqn_size));
                __mm_s.push_str(&*literal!(" could not be divided by the iterator size "));
                __mm_s.push_str(&*intString(iter_size));
                __mm_s.push_str(&*literal!(" without rest."));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    if implicit {
        row_crefs = ComponentRef::scalarizeSlice(row_cref, slice.clone(), false)?;
    } else {
        for mut cref in &*ComponentRef::scalarizeAll(row_cref, false)? {
            row_scal_lst = getCrefInFrameIndices(cref.clone(), &frames, eqn_rep_mapping, eqn_rep.map.clone(), false)?;
            accum_row_lst = metamodelica::cons(row_scal_lst, accum_row_lst);
        }
        row_scal_lst = List::flatten(accum_row_lst)?;
        row_scal_lst = if ((slice).is_empty() || ((slice).len() as i32) > ((row_scal_lst).len() as i32)) {
            row_scal_lst
        } else {
            List::getAtIndexLst(row_scal_lst, slice.clone(), true)?
        };
        row_crefs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut i in (row_scal_lst).into_iter().cloned() {
                let __x = BVariable::VariablePointers::varSlice(
                    eqn_rep,
                    i.clone(),
                    ({
                        let __elt =
                            (*metamodelica::index_checked(&eqn_rep_mapping.var_StA.borrow(), i.clone())?).clone();
                        __elt
                    }),
                    eqn_rep_mapping,
                    false,
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    func_var = (std::sync::Arc::new({
        let __pe_b3 = accum_dep_arr.clone();
        let __pe_b4 = var_rep.clone();
        let __pe_b5 = var_rep_mapping.clone();
        let __pe_b6 = false;
        move |__pe_a0, __pe_a1, __pe_a2| {
            updateDependenciesCref(
                __pe_a0,
                __pe_a1,
                __pe_a2,
                __pe_b3.clone(),
                __pe_b4.clone(),
                __pe_b5.clone(),
                __pe_b6.clone(),
            )
        }
    }) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32) -> Result<i32> + 'static>);
    func_eqn = (std::sync::Arc::new({
        let __pe_b3 = accum_dep_arr.clone();
        let __pe_b4 = eqn_rep.clone();
        let __pe_b5 = eqn_rep_mapping.clone();
        let __pe_b6 = false;
        move |__pe_a0, __pe_a1, __pe_a2| {
            updateDependenciesCref(
                __pe_a0,
                __pe_a1,
                __pe_a2,
                __pe_b3.clone(),
                __pe_b4.clone(),
                __pe_b5.clone(),
                __pe_b6.clone(),
            )
        }
    }) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32) -> Result<i32> + 'static>);
    for mut dep in &**dependencies {
        if UnorderedMap::contains(dep.clone(), var_rep.map.clone())? {
            (final_dep, var_arr_idx) = getVarArrIdx(dep.clone(), var_rep_mapping, var_rep.map.clone())?;
            fillDependencyArray(
                final_dep,
                body_size,
                &frames,
                var_rep_mapping,
                var_rep.map.clone(),
                &*(func_var.clone()),
                var_arr_idx,
                false,
            )?;
        } else if UnorderedMap::contains(dep.clone(), eqn_rep.map.clone())? {
            (final_dep, var_arr_idx) = getVarArrIdx(dep.clone(), eqn_rep_mapping, eqn_rep.map.clone())?;
            fillDependencyArray(
                final_dep,
                body_size,
                &frames,
                eqn_rep_mapping,
                eqn_rep.map.clone(),
                &*(func_eqn.clone()),
                var_arr_idx,
                false,
            )?;
        }
    }
    accum_dep_lst = accum_dep_arr
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>()
        .reverse();
    accum_dep_lst = if ((slice).is_empty() || ((slice).len() as i32) > ((accum_dep_lst).len() as i32)) {
        accum_dep_lst
    } else {
        List::getAtIndexLst(accum_dep_lst, slice, true)?
    };
    tpl_lst = List::zip(row_crefs, accum_dep_lst);
    Ok(tpl_lst)
}

pub(crate) fn fillDependencyArray(
    mut dep: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut body_size: i32,
    mut frames: &metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut func: &dyn ::std::ops::Fn(i32, i32, i32) -> Result<i32>,
    mut var_arr_idx: i32,
    mut resize: bool,
) -> Result<()> {
    let mut scal_cref: metamodelica::Ref<ComponentRef::NFComponentRef> = metamodelica::Ref::new(ComponentRef::EMPTY);
    let mut scal_length: i32;
    let mut body_repeat: i32;
    let mut element_repeat: i32;
    let mut eqn_idx: i32;
    let mut scal_lst: metamodelica::List<i32>;
    let mut scal_tpl_lst: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::List<i32>,
    )> = metamodelica::nil();
    for mut scal_cref in &*ComponentRef::scalarizeAll(dep.clone(), true)? {
        let mut scal_cref = scal_cref.clone();
        scal_lst = getCrefInFrameIndices(scal_cref.clone(), frames, mapping, map.clone(), resize)?;
        scal_tpl_lst = metamodelica::cons((scal_cref, scal_lst), scal_tpl_lst);
    }
    scal_length = ((scal_tpl_lst).len() as i32);
    if intMod(scal_length, body_size) == 0 {
        body_repeat = intDiv(scal_length, body_size);
        element_repeat = 1;
    } else if intMod(body_size, scal_length) == 0 {
        body_repeat = 1;
        element_repeat = intDiv(body_size, scal_length);
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBSlice.fillDependencyArray"));
                __mm_s.push_str(&*literal!(" failed because number of flattened indices "));
                __mm_s.push_str(&*intString(scal_length));
                __mm_s.push_str(&*literal!(" for dependency "));
                __mm_s.push_str(&*ComponentRef::toString(&dep)?);
                __mm_s.push_str(&*literal!(" could not be divided by or repeated to fit the body size "));
                __mm_s.push_str(&*intString(body_size));
                __mm_s.push_str(&*literal!(" without rest."));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    eqn_idx = 1;
    for mut i in 1..=element_repeat {
        for mut tpl in &*scal_tpl_lst.clone().reverse() {
            (scal_cref, scal_lst) = tpl.clone();
            scal_lst = scal_lst.reverse();
            if body_repeat > 1 {
                eqn_idx = 1;
            }
            for mut var_idx in &*scal_lst {
                if var_idx.clone() > 0 {
                    eqn_idx = func(eqn_idx, var_idx.clone(), var_arr_idx)?;
                }
            }
        }
    }
    Ok(())
}

pub type updateDependencies = std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32) -> Result<i32> + 'static>;

pub(crate) fn updateDependenciesCref(
    mut eqn_idx: i32,
    mut var_idx: i32,
    mut var_arr_idx: i32,
    mut accum_dep_arr: metamodelica::Array<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut vars: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut mapping: metamodelica::Ref<Mapping::Mapping>,
    mut resize: bool,
) -> Result<i32> {
    let mut eqn_idx: i32 = eqn_idx;
    metamodelica::arrayUpdate(
        accum_dep_arr.clone(),
        eqn_idx,
        metamodelica::cons(
            BVariable::VariablePointers::varSlice(&vars, var_idx, var_arr_idx, &mapping, resize)?,
            ({
                let __elt = (*metamodelica::index_checked(&accum_dep_arr.borrow(), eqn_idx)?).clone();
                __elt
            }),
        ),
    )?;
    eqn_idx = eqn_idx + 1;
    Ok(eqn_idx)
}

pub(crate) fn updateDependenciesInteger(
    mut eqn_idx: i32,
    mut var_idx: i32,
    mut var_arr_idx: i32,
    mut mode: i32,
    mut mode_to_var: metamodelica::Array<metamodelica::Array<i32>>,
    mut indices: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<i32> {
    let __ab_mode_to_var = mode_to_var.borrow();
    let mut eqn_idx: i32 = eqn_idx;
    let mut mode_to_var_row: metamodelica::Array<i32>;
    mode_to_var_row = (*metamodelica::index_checked(&__ab_mode_to_var, eqn_idx)?).clone();
    metamodelica::arrayUpdate(mode_to_var_row.clone(), mode, var_idx)?;
    metamodelica::arrayUpdate(
        indices.clone(),
        eqn_idx,
        metamodelica::cons(
            var_idx,
            ({
                let __elt = (*metamodelica::index_checked(&indices.borrow(), eqn_idx)?).clone();
                __elt
            }),
        ),
    )?;
    eqn_idx = eqn_idx + 1;
    Ok(eqn_idx)
}

pub(crate) fn getDependentCrefsPseudoArrayCausalized(
    mut row_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut slice: metamodelica::List<i32>,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    )>,
> {
    fn fixSingleDep(
        mut row_size: i32,
        mut single_dep: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut full_deps: Pointer::Pointer<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
        let mut single_dep: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = single_dep;
        let mut dep_size: i32 = ((single_dep).len() as i32);
        if row_size > dep_size {
            if intMod(row_size, dep_size) == 0 {
                single_dep = List::repeat(single_dep, intDiv(row_size, dep_size));
            } else {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!(
                            "NBSlice.getDependentCrefsPseudoArrayCausalized.fixSingleDep"
                        ));
                        __mm_s.push_str(&*literal!(" failed because dependencies of size "));
                        __mm_s.push_str(&*intString(dep_size));
                        __mm_s.push_str(&*literal!(" could not be repeated to fit row size "));
                        __mm_s.push_str(&*intString(row_size));
                        __mm_s.push_str(&*literal!("."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        } else if row_size < dep_size {
            Pointer::update(full_deps.clone(), listAppend(single_dep, Pointer::access(full_deps)));
            single_dep = metamodelica::nil();
        }
        Ok(single_dep)
    }

    let mut tpl_lst: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    )>;
    let mut row_cref_scal: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut dependencies_resizable: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut row_size: i32;
    let mut dependencies_scal: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut full_deps: Pointer::Pointer<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        Pointer::create(metamodelica::nil());
    row_cref_scal = ComponentRef::scalarizeSlice(row_cref, slice.clone(), false)?;
    row_size = ((row_cref_scal).len() as i32);
    dependencies_resizable = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        for mut dep in (dependencies).into_iter().cloned() {
            let __x = ComponentRef::simplifySubscripts(
                ComponentRef::mapExp(
                    &(dep.clone()),
                    (std::sync::Arc::new(Expression::replaceResizableParameterWithOriginal)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?,
                false,
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    dependencies_scal = ({
        let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
            metamodelica::nil();
        for mut dep in (dependencies_resizable).into_iter().cloned() {
            let __x = ComponentRef::scalarizeSlice(dep.clone(), slice.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if !((dependencies_scal).is_empty()) {
        dependencies_scal = ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
                metamodelica::nil();
            for mut d in (dependencies_scal).into_iter().cloned() {
                let __x = fixSingleDep(row_size, d.clone(), full_deps.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        dependencies_scal = ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
                metamodelica::nil();
            for mut d in (dependencies_scal).into_iter().cloned() {
                if !(!((d).is_empty())) {
                    continue;
                }
                let __x = d.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        if (dependencies_scal).is_empty() {
            dependencies_scal = List::fill(Pointer::access(full_deps), row_size);
        } else {
            dependencies_scal = ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
                    metamodelica::nil();
                for mut d in (List::transposeList(dependencies_scal)?).into_iter().cloned() {
                    let __x = listAppend(Pointer::access(full_deps.clone()), d.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
        }
        tpl_lst = List::zip(row_cref_scal, dependencies_scal);
    } else {
        tpl_lst = ({
            let mut __acc: metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<_>,
            )> = metamodelica::nil();
            for mut cref in (row_cref_scal).into_iter().cloned() {
                let __x = (cref.clone(), metamodelica::nil());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    Ok(tpl_lst)
}

pub(crate) fn locationToIndex(
    mut sizes: metamodelica::List<i32>,
    mut values: metamodelica::List<i32>,
    mut index: i32,
) -> Result<i32> {
    let mut index: i32 = index;
    let mut factor: i32 = 1;
    let mut val: i32;
    let mut siz: i32;
    let mut val_trav: metamodelica::List<i32> = values;
    let mut siz_trav: metamodelica::List<i32> = sizes;
    while !((val_trav).is_empty() || (siz_trav).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(val_trav) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        val = metamodelica::Own::own(__pa0);
        val_trav = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(siz_trav) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        siz = metamodelica::Own::own(__pa2);
        siz_trav = metamodelica::Own::own(__pa3);
        if val > siz {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBSlice.locationToIndex"));
                    __mm_s.push_str(&*literal!(" failed because value of "));
                    __mm_s.push_str(&*intString(val));
                    __mm_s.push_str(&*literal!(" is too large for size "));
                    __mm_s.push_str(&*intString(siz));
                    __mm_s.push_str(&*literal!("."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        index = index + (val - 1) * factor;
        factor = factor * siz;
    }
    Ok(index)
}

pub(crate) fn indexToLocation(mut index: i32, mut sizes: metamodelica::List<i32>) -> metamodelica::List<i32> {
    let mut vals: metamodelica::List<i32> = metamodelica::nil();
    let mut iterator: i32 = index;
    let mut divisor: i32 = ({
        let mut __acc: i32 = 1;
        for mut s in (sizes.clone()).into_iter().cloned() {
            let __x = s.clone();
            __acc *= __x;
        }
        __acc
    });
    for mut size in &*sizes {
        divisor = intDiv(divisor, size.clone());
        vals = metamodelica::cons(intDiv(iterator, divisor), vals);
        iterator = intMod(iterator, divisor);
    }
    vals
}

pub(crate) fn transposeLocations(
    mut locations: &metamodelica::List<metamodelica::List<i32>>,
    mut out_size: i32,
) -> Result<metamodelica::List<metamodelica::Array<i32>>> {
    let mut locations_transposed: metamodelica::List<metamodelica::Array<i32>>;
    let mut lT_tmp: metamodelica::Array<metamodelica::List<i32>> = arrayCreate(out_size, metamodelica::nil());
    let mut lT_tmp2: metamodelica::Array<metamodelica::Array<i32>> = arrayCreate(out_size, arrayCreate(0, 0));
    let mut idx: i32;
    for mut location in &**locations {
        idx = 1;
        for mut i in &*location.clone() {
            {
                let __cell0 = metamodelica::cons(
                    i.clone(),
                    ({
                        let __elt = (*metamodelica::index_checked(&lT_tmp.borrow(), idx)?).clone();
                        __elt
                    }),
                );
                let __idx0 = idx;
                *metamodelica::index_mut_checked(&mut lT_tmp.clone().borrow_mut(), __idx0)? = __cell0;
            }
            idx = idx + 1;
        }
    }
    for mut j in 1..=metamodelica::arrayLength(lT_tmp.clone()) {
        {
            let __cell1 = metamodelica::arrayFromVec(
                ({
                    let __elt = (*metamodelica::index_checked(&lT_tmp.borrow(), j)?).clone();
                    __elt
                })
                .reverse()
                .into_iter()
                .cloned()
                .collect(),
            );
            let __idx1 = j;
            *metamodelica::index_mut_checked(&mut lT_tmp2.clone().borrow_mut(), __idx1)? = __cell1;
        }
    }
    locations_transposed = lT_tmp2
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>()
        .reverse();
    Ok(locations_transposed)
}

pub(crate) fn orderTransposedFrameLocations(
    mut frame_locations_transposed: metamodelica::List<(
        metamodelica::Array<i32>,
        (
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator::Iterator>>,
        ),
    )>,
) -> Result<(
    metamodelica::List<(
        metamodelica::Array<i32>,
        (
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator::Iterator>>,
        ),
    )>,
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    FrameOrderingStatus,
)> {
    let mut frame_locations_transposed: metamodelica::List<(
        metamodelica::Array<i32>,
        (
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator::Iterator>>,
        ),
    )> = frame_locations_transposed;
    let mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
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
    let mut status: FrameOrderingStatus;
    let mut frame_inertia_lst: metamodelica::List<(
        i32,
        (
            metamodelica::Array<i32>,
            (
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
                Option<metamodelica::Ref<Iterator::Iterator>>,
            ),
        ),
    )>;
    frame_inertia_lst = ({
        let mut __acc: metamodelica::List<(
            i32,
            (
                metamodelica::Array<i32>,
                (
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<Expression::NFExpression>,
                    Option<metamodelica::Ref<Iterator::Iterator>>,
                ),
            ),
        )> = metamodelica::nil();
        for mut frame in (frame_locations_transposed).into_iter().cloned() {
            let __x = (frameLocationInertia(frame.clone())?, frame.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    frame_inertia_lst = List::sort(
        frame_inertia_lst,
        std::sync::Arc::new(fnptr!(Util::compareTupleIntGt, _, _)),
    )?;
    (frame_inertia_lst, status) = resolveEqualInertia(&frame_inertia_lst, replacements.clone())?;
    frame_locations_transposed = ({
        let mut __acc: metamodelica::List<(
            metamodelica::Array<i32>,
            (
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
                Option<metamodelica::Ref<Iterator::Iterator>>,
            ),
        )> = metamodelica::nil();
        for mut frame_inertia in (frame_inertia_lst).into_iter().cloned() {
            let __x = Util::tuple22(frame_inertia.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((frame_locations_transposed, replacements, status))
}

fn frameLocationInertia(
    mut frameLocation: (
        metamodelica::Array<i32>,
        (
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator::Iterator>>,
        ),
    ),
) -> Result<i32> {
    let mut inertia: i32 = 1;
    let mut dim: metamodelica::Array<i32>;
    dim = Util::tuple21(frameLocation);
    while inertia < metamodelica::arrayLength(dim.clone())
        && ({
            let __elt = (*metamodelica::index_checked(&dim.borrow(), inertia)?).clone();
            __elt
        }) == ({
            let __elt = (*metamodelica::index_checked(&dim.borrow(), inertia + 1)?).clone();
            __elt
        })
    {
        inertia = inertia + 1;
    }
    Ok(inertia)
}

fn resolveEqualInertia(
    mut frame_inertia_lst: &metamodelica::List<(
        i32,
        (
            metamodelica::Array<i32>,
            (
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
                Option<metamodelica::Ref<Iterator::Iterator>>,
            ),
        ),
    )>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<(
    metamodelica::List<(
        i32,
        (
            metamodelica::Array<i32>,
            (
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
                Option<metamodelica::Ref<Iterator::Iterator>>,
            ),
        ),
    )>,
    FrameOrderingStatus,
)> {
    let mut resolved: metamodelica::List<(
        i32,
        (
            metamodelica::Array<i32>,
            (
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
                Option<metamodelica::Ref<Iterator::Iterator>>,
            ),
        ),
    )> = metamodelica::nil();
    let mut status: FrameOrderingStatus = FrameOrderingStatus::UNCHANGED.clone();
    let mut tpl1: (
        i32,
        (
            metamodelica::Array<i32>,
            (
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
                Option<metamodelica::Ref<Iterator::Iterator>>,
            ),
        ),
    );
    let mut tpl2: (
        i32,
        (
            metamodelica::Array<i32>,
            (
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
                Option<metamodelica::Ref<Iterator::Iterator>>,
            ),
        ),
    );
    let mut rest: metamodelica::List<(
        i32,
        (
            metamodelica::Array<i32>,
            (
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
                Option<metamodelica::Ref<Iterator::Iterator>>,
            ),
        ),
    )>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*frame_inertia_lst)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    tpl1 = metamodelica::Own::own(__pa0);
    rest = metamodelica::Own::own(__pa1);
    while !((rest).is_empty()) {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        tpl2 = metamodelica::Own::own(__pa2);
        rest = metamodelica::Own::own(__pa3);
        tpl1 = (::match_deref::match_deref! { match &((tpl1.clone(), tpl2.clone())) {
            ((inertia1, (loc1, (name1, _, _))), (inertia2, (loc2, (name2, _, _)))) if (inertia1.clone() == inertia2.clone()) => {
                let mut m: i32;
                let mut b: i32;
                let mut addOp: metamodelica::Ref<Operator::NFOperator>;
                let mut mulOp: metamodelica::Ref<Operator::NFOperator>;
                let mut linMap: metamodelica::Ref<Expression::NFExpression>;
                addOp = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), Operator::SizeClassification::SCALAR.clone()), openmodelica_nf_frontend::NFType::interned_INTEGER())?;
                mulOp = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::SCALAR.clone()), openmodelica_nf_frontend::NFType::interned_INTEGER())?;
                if metamodelica::arrayLength(loc1.clone()) != metamodelica::arrayLength(loc2.clone()) {
                    status = FrameOrderingStatus::FAILURE.clone();
                    return Ok((resolved, status));
                } else if metamodelica::arrayLength(loc1.clone()) == 1 {
                    b = ({let __elt = (*metamodelica::index_checked(&loc2.borrow(), 1)?).clone(); __elt}) - ({let __elt = (*metamodelica::index_checked(&loc1.borrow(), 1)?).clone(); __elt});
                    linMap = Expression::fromCref(name1.clone(), false)?;
                    if b != 0 {
                        linMap = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: b }), linMap], inv_arguments: metamodelica::nil(), operator: addOp });
                    }
                    UnorderedMap::add(name2.clone(), linMap, replacements.clone())?;
                    status = FrameOrderingStatus::CHANGED.clone();
                } else {
                    m = ((metamodelica::real_div_checked((metamodelica::OrderedFloat((({let __elt = (*metamodelica::index_checked(&loc2.borrow(), 1)?).clone(); __elt}) - ({let __elt = (*metamodelica::index_checked(&loc2.borrow(), 1 + inertia2.clone())?).clone(); __elt})) as f64)), (metamodelica::OrderedFloat((({let __elt = (*metamodelica::index_checked(&loc1.borrow(), 1)?).clone(); __elt}) - ({let __elt = (*metamodelica::index_checked(&loc1.borrow(), 1 + inertia1.clone())?).clone(); __elt})) as f64)))?).0.floor() as i32);
                    b = ({let __elt = (*metamodelica::index_checked(&loc2.borrow(), 1)?).clone(); __elt}) - m * ({let __elt = (*metamodelica::index_checked(&loc1.borrow(), 1)?).clone(); __elt});
                    for mut i in 2..=metamodelica::arrayLength(loc1.clone()) {
                        if ({let __elt = (*metamodelica::index_checked(&loc2.borrow(), i)?).clone(); __elt}) != m * ({let __elt = (*metamodelica::index_checked(&loc1.borrow(), i)?).clone(); __elt}) + b {
                            status = FrameOrderingStatus::FAILURE.clone();
                            return Ok((resolved, status));
                        }
                    }
                    linMap = Expression::fromCref(name1.clone(), false)?;
                    if m != 1 {
                        linMap = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: m }), linMap], inv_arguments: metamodelica::nil(), operator: mulOp });
                    }
                    if b != 0 {
                        linMap = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: b }), linMap], inv_arguments: metamodelica::nil(), operator: addOp });
                    }
                    UnorderedMap::add(name2.clone(), linMap, replacements.clone())?;
                    status = FrameOrderingStatus::CHANGED.clone();
                }
                tpl1
            },
            _ => {
                resolved = metamodelica::cons(tpl1, resolved);
                tpl2
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    resolved = metamodelica::cons(tpl1, resolved).reverse();
    Ok((resolved, status))
}

pub(crate) fn recollectRangesHeuristic(
    mut frame_locations_transposed: &metamodelica::List<(
        metamodelica::Array<i32>,
        (
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator::Iterator>>,
        ),
    )>,
) -> Result<(
    metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>,
    Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
    RecollectStatus,
)> {
    let mut frames: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )> = metamodelica::nil();
    let mut removed_diagonal: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    > = None;
    let mut status: RecollectStatus;
    let mut dim: metamodelica::Array<i32>;
    let mut frame: (
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    );
    let mut check_shift: i32;
    let mut pre_shift: i32;
    let mut shift: i32 = 1;
    let mut start: i32;
    let mut step: i32;
    let mut stop: i32;
    let mut max_size: i32;
    let mut new_step: i32;
    let mut new_stop: i32;
    let mut check_stop: i32;
    let mut fail_: bool;
    let mut starts: metamodelica::List<i32> = metamodelica::nil();
    let mut stops: metamodelica::List<i32> = metamodelica::nil();
    let mut steps: metamodelica::List<i32> = metamodelica::nil();
    let mut shifts: metamodelica::List<i32> = metamodelica::nil();
    let mut failed: metamodelica::List<bool> = metamodelica::nil();
    let mut min_dim: i32;
    let mut max_dim: i32;
    let mut diagonal: metamodelica::List<(
        metamodelica::Array<i32>,
        (
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator::Iterator>>,
        ),
    )>;
    let mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >;
    let mut fos: FrameOrderingStatus;
    for mut tpl in &**frame_locations_transposed {
        fail_ = false;
        (dim, frame) = tpl.clone();
        pre_shift = shift;
        max_size = metamodelica::arrayLength(dim.clone());
        if max_size == 1 {
            frames = metamodelica::cons(
                applyNewFrameRange(
                    frame,
                    (
                        ({
                            let __elt = (*metamodelica::index_checked(&dim.borrow(), 1)?).clone();
                            __elt
                        }),
                        1,
                        ({
                            let __elt = (*metamodelica::index_checked(&dim.borrow(), 1)?).clone();
                            __elt
                        }),
                    ),
                )?,
                frames,
            );
            starts = metamodelica::cons(
                ({
                    let __elt = (*metamodelica::index_checked(&dim.borrow(), 1)?).clone();
                    __elt
                }),
                starts,
            );
            steps = metamodelica::cons(0, steps);
            stops = metamodelica::cons(
                ({
                    let __elt = (*metamodelica::index_checked(&dim.borrow(), 1)?).clone();
                    __elt
                }),
                stops,
            );
            shifts = metamodelica::cons(shift, shifts);
        } else {
            start = ({
                let __elt = (*metamodelica::index_checked(&dim.borrow(), 1)?).clone();
                __elt
            });
            stop = ({
                let __elt = (*metamodelica::index_checked(&dim.borrow(), 1 + shift)?).clone();
                __elt
            });
            step = stop - start;
            if step == 0 {
                frames = metamodelica::cons(applyNewFrameRange(frame, (start, 1, stop))?, frames);
                starts = metamodelica::cons(start, starts);
                steps = metamodelica::cons(step, steps);
                stops = metamodelica::cons(stop, stops);
                shifts = metamodelica::cons(shift, shifts);
            } else {
                new_step = step;
                new_stop = stop;
                while new_step == step && shift + pre_shift < max_size {
                    stop = new_stop;
                    shift = shift + pre_shift;
                    new_stop = ({
                        let __elt = (*metamodelica::index_checked(&dim.borrow(), 1 + shift)?).clone();
                        __elt
                    });
                    new_step = new_stop - stop;
                }
                if new_step == step {
                    stop = new_stop;
                    shift = shift + pre_shift;
                } else {
                    check_shift = shift;
                    while check_shift + pre_shift < max_size {
                        new_step = step;
                        while new_step == step && check_shift + pre_shift < max_size {
                            check_stop = new_stop;
                            check_shift = check_shift + pre_shift;
                            new_stop = ({
                                let __elt = (*metamodelica::index_checked(&dim.borrow(), 1 + check_shift)?).clone();
                                __elt
                            });
                            new_step = new_stop - check_stop;
                        }
                        if check_shift + pre_shift == max_size {
                            check_shift = check_shift + pre_shift;
                        }
                        if !(intMod(check_shift, shift) == 0) {
                            fail_ = true;
                            break;
                        }
                    }
                }
                min_dim = ({
                    let mut __acc: Option<i32> = None;
                    for mut d in (dim.clone()).borrow().iter() {
                        let __x = d.clone();
                        __acc = Some(match __acc {
                            None => __x,
                            Some(__cur) => {
                                if __x < __cur {
                                    __x
                                } else {
                                    __cur
                                }
                            }
                        });
                    }
                    __acc.unwrap_or(i32::MAX)
                });
                max_dim = ({
                    let mut __acc: Option<i32> = None;
                    for mut d in (dim.clone()).borrow().iter() {
                        let __x = d.clone();
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
                });
                if fail_ {
                    if step > 0 {
                        frames = metamodelica::cons(applyNewFrameRange(frame, (min_dim, step, max_dim))?, frames);
                    } else {
                        frames = metamodelica::cons(applyNewFrameRange(frame, (max_dim, step, min_dim))?, frames);
                    }
                } else {
                    frames = metamodelica::cons(applyNewFrameRange(frame, (start, step, stop))?, frames);
                }
                steps = metamodelica::cons(step, steps);
                starts = if (step > 0) {
                    metamodelica::cons(min_dim, starts)
                } else {
                    metamodelica::cons(max_dim, starts)
                };
                stops = if (step > 0) {
                    metamodelica::cons(max_dim, stops)
                } else {
                    metamodelica::cons(min_dim, stops)
                };
                shifts = metamodelica::cons(shift, shifts);
                failed = metamodelica::cons(fail_, failed);
            }
        }
    }
    if List::fold(&failed, &fnptr!(boolOr, bool, bool), false)? {
        diagonal = reconstructDiagonal(
            frame_locations_transposed,
            starts.reverse(),
            steps.reverse(),
            stops.reverse(),
            shifts.reverse(),
            failed.reverse(),
        )?;
        (diagonal, replacements, fos) = orderTransposedFrameLocations(diagonal)?;
        if fos == FrameOrderingStatus::CHANGED.clone() {
            removed_diagonal = Some(replacements);
            status = RecollectStatus::SUCCESS.clone();
        } else {
            status = RecollectStatus::FAILURE.clone();
        }
    } else {
        status = RecollectStatus::SUCCESS.clone();
    }
    Ok((frames, removed_diagonal, status))
}

pub(crate) fn reconstructDiagonal(
    mut frame_locations_transposed: &metamodelica::List<(
        metamodelica::Array<i32>,
        (
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator::Iterator>>,
        ),
    )>,
    mut starts: metamodelica::List<i32>,
    mut steps: metamodelica::List<i32>,
    mut stops: metamodelica::List<i32>,
    mut shifts: metamodelica::List<i32>,
    mut failed: metamodelica::List<bool>,
) -> Result<
    metamodelica::List<(
        metamodelica::Array<i32>,
        (
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator::Iterator>>,
        ),
    )>,
> {
    let mut diagonal: metamodelica::List<(
        metamodelica::Array<i32>,
        (
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator::Iterator>>,
        ),
    )> = metamodelica::nil();
    let mut start: i32;
    let mut step: i32;
    let mut stop: i32;
    let mut pos: i32;
    let mut shift: i32 = 1;
    let mut fail_: bool;
    let mut start_rest: metamodelica::List<i32> = starts;
    let mut step_rest: metamodelica::List<i32> = steps;
    let mut stop_rest: metamodelica::List<i32> = stops;
    let mut shift_rest: metamodelica::List<i32> = shifts;
    let mut fail_rest: metamodelica::List<bool> = failed;
    let mut dim: metamodelica::Array<i32>;
    let mut missing_dims: metamodelica::List<i32>;
    let mut frame: (
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    );
    for mut tpl in &**frame_locations_transposed {
        (dim, frame) = tpl.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(start_rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        start = metamodelica::Own::own(__pa0);
        start_rest = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(step_rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        step = metamodelica::Own::own(__pa2);
        step_rest = metamodelica::Own::own(__pa3);
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(stop_rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        stop = metamodelica::Own::own(__pa4);
        stop_rest = metamodelica::Own::own(__pa5);
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(fail_rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => return Err("pattern mismatch"),
        } };
        fail_ = metamodelica::Own::own(__pa6);
        fail_rest = metamodelica::Own::own(__pa7);
        missing_dims = metamodelica::nil();
        pos = start;
        if fail_ {
            for mut i in ({
                let __s = 1;
                let __e = metamodelica::arrayLength(dim.clone());
                let __step = shift;
                (0i32..)
                    .map(move |__k| __s + __k * __step)
                    .take_while(move |&__v| __step != 0 && (if __step > 0 { __v <= __e } else { __v >= __e }))
            }) {
                while ({
                    let __elt = (*metamodelica::index_checked(&dim.borrow(), i)?).clone();
                    __elt
                }) != pos
                {
                    missing_dims = metamodelica::cons(pos, missing_dims);
                    pos = pos + step;
                    if sign(metamodelica::OrderedFloat((step) as f64)) * pos
                        > sign(metamodelica::OrderedFloat((step) as f64)) * stop
                    {
                        break;
                    }
                }
                if sign(metamodelica::OrderedFloat((step) as f64)) * (pos + step)
                    > sign(metamodelica::OrderedFloat((step) as f64)) * stop
                {
                    pos = start;
                } else {
                    pos = pos + step;
                }
            }
            while sign(metamodelica::OrderedFloat((step) as f64)) * pos
                <= sign(metamodelica::OrderedFloat((step) as f64)) * stop
            {
                missing_dims = metamodelica::cons(pos, missing_dims);
                pos = pos + step;
            }
        } else {
            for mut i in ({
                let __s = 1;
                let __e = metamodelica::arrayLength(dim.clone());
                let __step = shift;
                (0i32..)
                    .map(move |__k| __s + __k * __step)
                    .take_while(move |&__v| __step != 0 && (if __step > 0 { __v <= __e } else { __v >= __e }))
            }) {
                missing_dims = metamodelica::cons(
                    ({
                        let __elt = (*metamodelica::index_checked(&dim.borrow(), i)?).clone();
                        __elt
                    }),
                    missing_dims,
                );
            }
        }
        diagonal = metamodelica::cons(
            (
                metamodelica::arrayFromVec(missing_dims.reverse().into_iter().cloned().collect()),
                frame,
            ),
            diagonal,
        );
        let (__pa8, __pa9) = ::match_deref::match_deref! { match &(shift_rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: __pa9 } => (__pa8.clone(), __pa9.clone()),
            _ => return Err("pattern mismatch"),
        } };
        shift = metamodelica::Own::own(__pa8);
        shift_rest = metamodelica::Own::own(__pa9);
    }
    diagonal = diagonal.reverse();
    Ok(diagonal)
}

pub(crate) fn naiveSeparation(
    mut indices: metamodelica::List<i32>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut index_clusters: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut i: i32;
    let mut rest: metamodelica::List<i32>;
    let mut current: metamodelica::List<i32> = metamodelica::nil();
    if !((indices).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(indices.reverse()) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        i = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        current = list![i];
        for mut i2 in &*rest {
            if i - i2.clone() == 1 {
                current = metamodelica::cons(i2.clone(), current);
            } else {
                index_clusters = metamodelica::cons(current, index_clusters);
                current = list![i2.clone()];
            }
            i = i2.clone();
        }
        index_clusters = metamodelica::cons(current, index_clusters);
    }
    Ok(index_clusters)
}

pub(crate) fn upgradeRowFull(
    mut dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
) -> Result<metamodelica::List<i32>> {
    let mut indices: metamodelica::List<i32> = metamodelica::nil();
    let mut scalarized_dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
        List::flatten(
            ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
                    metamodelica::nil();
                for mut dep in (dependencies.clone()).into_iter().cloned() {
                    let __x = ComponentRef::scalarizeAll(dep.clone(), true)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?;
    let mut replaced: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut stripped: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut var_arr_idx: i32;
    let mut var_start: i32;
    let mut var_scal_idx: i32;
    for mut cref in &*scalarized_dependencies {
        (replaced, stripped) = getReplacedAndStripped(metamodelica::AsArg::as_arg(&cref))?;
        var_arr_idx = UnorderedMap::getSafe(
            stripped.clone(),
            map.clone(),
            metamodelica::sourceInfo!("NBackEnd/Util/NBSlice.mo"),
        )?;
        (var_start, _) = ({
            let __elt = (*metamodelica::index_checked(&mapping.var_AtS.borrow(), var_arr_idx)?).clone();
            __elt
        });
        var_scal_idx = indexFromReplacedStripped(&replaced, &stripped, var_start)?;
        indices = metamodelica::cons(var_scal_idx, indices);
    }
    Ok(indices)
}

pub(crate) fn upgradeRow(
    mut eqn_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut eqn_arr_idx: i32,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut dependencies: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut dep: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Dependency::Dependency>,
        >,
    >,
    mut rep: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut fullmap: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut m: &IntMatrix::Builder,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut modes: metamodelica::Ref<Vector::Vector<metamodelica::Ref<Mode::Mode>>>,
) -> Result<()> {
    for mut cref in &**dependencies {
        resolveDependency(
            cref.clone(),
            eqn_name.clone(),
            eqn_arr_idx,
            iter,
            ty.clone(),
            dep.clone(),
            rep.clone(),
            map.clone(),
            fullmap.clone(),
            m,
            mapping,
            modes.clone(),
        )?;
    }
    Ok(())
}

pub(crate) fn getSingleIndex(mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> {
    let mut index: i32;
    let mut replaced: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut stripped: metamodelica::Ref<ComponentRef::NFComponentRef>;
    (replaced, stripped) = getReplacedAndStripped(cref)?;
    index = indexFromReplacedStripped(&replaced, &stripped, 0)?;
    Ok(index)
}

fn getReplacedAndStripped(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
)> {
    let mut replaced: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut stripped: metamodelica::Ref<ComponentRef::NFComponentRef>;
    replaced = ComponentRef::mapExp(
        cref,
        (std::sync::Arc::new(Expression::replaceResizableParameter)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    replaced = ComponentRef::simplifySubscripts(replaced, false)?;
    stripped = ComponentRef::stripSubscriptsAll(&replaced);
    Ok((replaced, stripped))
}

fn indexFromReplacedStripped(
    mut replaced: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut stripped: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut var_start: i32,
) -> Result<i32> {
    let mut index: i32;
    let mut sizes: metamodelica::List<i32>;
    let mut int_subs: metamodelica::List<i32>;
    sizes = ComponentRef::sizes(stripped, false, false, metamodelica::nil())?;
    int_subs = ComponentRef::subscriptsToInteger(replaced)?;
    index = locationToIndex(sizes, int_subs, var_start)?;
    Ok(index)
}

fn resolveSkipsLst(
    mut index: i32,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut skips: &metamodelica::List<metamodelica::List<i32>>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut fullmap: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
) -> Result<metamodelica::List<(i32, metamodelica::Ref<Type::NFType>)>> {
    let mut skip_lst: metamodelica::List<(i32, metamodelica::Ref<Type::NFType>)> = metamodelica::nil();
    let mut combinations: metamodelica::List<metamodelica::List<i32>> = List::combination(skips);
    let mut sub_idx: i32;
    let mut sub_ty: metamodelica::Ref<Type::NFType>;
    if (combinations).is_empty() {
        skip_lst = list![(index, ty)];
    } else {
        for mut com in &*combinations {
            (sub_idx, sub_ty) = resolveSkips(index, ty.clone(), com.clone(), cref, fullmap.clone())?;
            skip_lst = metamodelica::cons((sub_idx, sub_ty), skip_lst);
        }
    }
    Ok(skip_lst)
}

fn resolveSkips<'__b>(
    mut index: i32,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut skips: metamodelica::List<i32>,
    mut cref: &'__b metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut fullmap: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
) -> Result<(i32, metamodelica::Ref<Type::NFType>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((ty.clone(), skips)) {
            (Deref @ Type::TUPLE { .. }, Deref @ metamodelica::ListNode::Cons { head: 0, tail: _ }) => {
                return Ok((index, ty.clone()))
            },
            (Deref @ Type::TUPLE { types: rest_ty, .. }, Deref @ metamodelica::ListNode::Cons { head: skip, tail: rest }) if (skip.clone() <= ((rest_ty).len() as i32)) => {
                let mut sub_ty: metamodelica::Ref<Type::NFType>;
                let mut rest_ty = (*rest_ty).clone();
                for mut i in 1..=skip.clone() - 1 {
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_ty.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    sub_ty = metamodelica::Own::own(__pa0);
                    rest_ty = metamodelica::Own::own(__pa1);
                    index = index + Type::sizeOf(&sub_ty, false)?;
                }
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_ty.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                sub_ty = metamodelica::Own::own(__pa2);
                rest_ty = metamodelica::Own::own(__pa3);
                { (index, ty, skips, cref, fullmap) = (index, sub_ty, rest.clone(), cref, fullmap); continue '__tco; }
            },
            (Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. }, Deref @ metamodelica::ListNode::Cons { head: skip, tail: rest }) => {
                let mut parent: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
                let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                let mut field: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut subs: metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>;
                field = (match BVariable::getParent(BVariable::getVarPointer(cref, metamodelica::sourceInfo!("NBackEnd/Util/NBSlice.mo"))?) {
            Some(mut __esc_parent) => {
                parent = __esc_parent.clone();
                subs = ComponentRef::subscriptsAll(cref);
                crefs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut child in (BVariable::getRecordChildren(parent)?).into_iter().cloned() {
                let __x = BVariable::getVarName(child.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                crefs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut c in (crefs).into_iter().cloned() {
                if !(UnorderedMap::contains(c.clone(), fullmap.clone())?) { continue; }
                let __x = c.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                if skip.clone() <= ((crefs).len() as i32) {
                    for mut i in 1..=skip.clone() - 1 {
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crefs) {
                            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        field = metamodelica::Own::own(__pa0);
                        crefs = metamodelica::Own::own(__pa1);
                        field = ComponentRef::setSubscriptsList(&subs, field)?;
                        index = index + Type::sizeOf(&(ComponentRef::getSubscriptedType(&field, false)?), false)?;
                    }
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(crefs) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    field = metamodelica::Own::own(__pa2);
                    crefs = metamodelica::Own::own(__pa3);
                    field = ComponentRef::setSubscriptsList(&subs, field)?;
                } else {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.resolveSkips")); __mm_s.push_str(&*literal!(" failed because skip of ")); __mm_s.push_str(&*intString(skip.clone())); __mm_s.push_str(&*literal!(" is too large for record elements ")); __mm_s.push_str(&*List::toString(crefs, &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0), List::Style::FLAT_CURLY.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                    return Err("fail");
                }
                field
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.resolveSkips")); __mm_s.push_str(&*literal!(" failed because skip of ")); __mm_s.push_str(&*intString(skip.clone())); __mm_s.push_str(&*literal!(" for type ")); __mm_s.push_str(&*Type::toString(&ty)?); __mm_s.push_str(&*literal!(" is requested, but the cref is not part of a record: ")); __mm_s.push_str(&*ComponentRef::toString(cref)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
        });
                { (index, ty, skips, cref, fullmap) = (index, ComponentRef::getSubscriptedType(&field, false)?, rest.clone(), cref, fullmap); continue '__tco; }
            },
            (Deref @ Type::ARRAY { .. }, rest) if (Dimension::sizesProduct(var_field!((*ty).dimensions, Type::NFType::ARRAY).clone(), true)? == 1) => {
                { (index, ty, skips, cref, fullmap) = (index, var_field!((*ty).elementType, Type::NFType::ARRAY).clone(), rest.clone(), cref, fullmap); continue '__tco; }
            },
            (Deref @ Type::ARRAY { .. }, rest) if (List::compareLength(rest.clone(), var_field!((*ty).dimensions, Type::NFType::ARRAY).clone())? >= 0) => {
                let mut tail: metamodelica::List<i32>;
                let mut rest = (*rest).clone();
                (rest, tail) = List::split(rest.clone(), ((var_field!((*ty).dimensions, Type::NFType::ARRAY)).len() as i32))?;
                index = locationToIndex(({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut dim in (var_field!((*ty).dimensions, Type::NFType::ARRAY).clone()).into_iter().cloned() {
                let __x = Dimension::size(&(dim.clone()), true)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), rest.clone(), index)?;
                { (index, ty, skips, cref, fullmap) = (index, var_field!((*ty).elementType, Type::NFType::ARRAY).clone(), tail, cref, fullmap); continue '__tco; }
            },
            (Deref @ Type::ARRAY { .. }, rest) => {
                let mut rest_dim: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
                let mut tail_dim: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
                (rest_dim, tail_dim) = List::split(var_field!((*ty).dimensions, Type::NFType::ARRAY).clone(), ((rest).len() as i32))?;
                index = locationToIndex(({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut dim in (rest_dim).into_iter().cloned() {
                let __x = Dimension::size(&(dim.clone()), true)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), rest.clone(), index)?;
                assign_variant_field!(ty => Type::NFType::ARRAY; dimensions = tail_dim);
                return Ok((index, ty.clone()))
            },
            (_, Deref @ metamodelica::ListNode::Cons { head: skip, tail: _ }) if (Type::isTuple(&ty) || Type::isArray(&ty)) => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.resolveSkips")); __mm_s.push_str(&*literal!(" failed because skip of ")); __mm_s.push_str(&*intString(skip.clone())); __mm_s.push_str(&*literal!(" for type ")); __mm_s.push_str(&*Type::toString(&ty)?); __mm_s.push_str(&*literal!(" is too large.")); ArcStr::from(__mm_s) }])?;
                return Ok(return Err("fail"))
            },
            (Deref @ Type::TUPLE { .. }, Deref @ metamodelica::ListNode::Nil) => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.resolveSkips")); __mm_s.push_str(&*literal!(" failed because there is no skip for type ")); __mm_s.push_str(&*Type::toString(&ty)?); ArcStr::from(__mm_s) }])?;
                return Ok(return Err("fail"))
            },
            (_, Deref @ metamodelica::ListNode::Cons { head: skip, tail: _ }) if (skip.clone() != 1) => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.resolveSkips")); __mm_s.push_str(&*literal!(" failed because skip of ")); __mm_s.push_str(&*intString(skip.clone())); __mm_s.push_str(&*literal!(" for type ")); __mm_s.push_str(&*Type::toString(&ty)?); __mm_s.push_str(&*literal!(" is invalid.")); ArcStr::from(__mm_s) }])?;
                return Ok(return Err("fail"))
            },
            _ => {
                return Ok((index, ty.clone()))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub type Key = metamodelica::List<i32>;

pub type Val1 = metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;

pub type Val2 = metamodelica::List<i32>;

fn keyString(mut key: Key) -> Result<ArcStr> {
    let mut r#str: ArcStr = List::toString(key.clone(), &fnptr!(intString, i32), List::Style::FLAT_CURLY.clone())?;
    Ok(r#str)
}

fn keyHash(mut key: &Key) -> i32 {
    let mut hash: i32 = Util::HASH_SEED.clone();
    for mut k in &**key {
        hash = stringHashDjb2Continue(&(intString(k.clone())), hash);
    }
    hash
}

fn keyEqual(mut key1: Key, mut key2: Key) -> Result<bool> {
    let mut b: bool = List::isEqualOnTrue(key1.clone(), key2.clone(), &fnptr!(intEq, i32, i32))?;
    Ok(b)
}

fn val1String(mut val: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = List::toString(
        val.clone(),
        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
        List::Style::FLAT_CURLY.clone(),
    )?;
    Ok(r#str)
}

fn resolveDependency(
    mut original_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut eqn_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut eqn_arr_idx: i32,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut dep: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Dependency::Dependency>,
        >,
    >,
    mut rep: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut fullmap: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut m: &IntMatrix::Builder,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut modes: metamodelica::Ref<Vector::Vector<metamodelica::Ref<Mode::Mode>>>,
) -> Result<()> {
    let mut d: metamodelica::Ref<Dependency::Dependency>;
    let mut skip_lst: metamodelica::List<(i32, metamodelica::Ref<Type::NFType>)>;
    let mut skip_ty: metamodelica::Ref<Type::NFType>;
    let mut skip_idx: i32;
    let mut start: i32;
    let mut size: i32;
    let mut body_size: i32;
    let mut iter_size: i32;
    let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator::Iterator>>>;
    let mut frames: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>;
    let mut regulars: metamodelica::List<bool>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    match '__try0: {
        cref = unwrap_break_err!(ComponentRef::mapExp(&original_cref, (std::sync::Arc::new(Expression::replaceResizableParameter) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>)), '__try0);
        cref = unwrap_break_err!(ComponentRef::simplifySubscripts(cref.clone(), false), '__try0);
        d = unwrap_break_err!(UnorderedMap::getSafe(original_cref.clone(), dep.clone(), metamodelica::sourceInfo!("NBackEnd/Util/NBSlice.mo")), '__try0);
        (start, _) = ({
            let __elt =
                (*unwrap_break_err!(metamodelica::index_checked(&mapping.eqn_AtS.borrow(), eqn_arr_idx), '__try0))
                    .clone();
            __elt
        });
        if !(unwrap_break_err!(UnorderedSet::contains(cref.clone(), rep.clone()), '__try0)) {
            skip_lst = unwrap_break_err!(resolveSkipsLst(start, ty.clone(), &(d.skips.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), &cref, fullmap.clone()), '__try0);
        } else {
            skip_lst = list![(start, ty.clone())];
        }
        for mut tpl in &*skip_lst {
            (skip_idx, skip_ty) = tpl.clone();
            body_size = unwrap_break_err!(Type::sizeOf(&skip_ty, true), '__try0);
            iter_size = unwrap_break_err!(Iterator::size(iter, true), '__try0);
            size = body_size * iter_size;
            (names, ranges, maps) = Iterator::getFrames(iter);
            frames = List::zip3(names.clone(), ranges.clone(), maps.clone());
            regulars = Dependency::toBoolean(&d);
            if unwrap_break_err!(List::all(&regulars, &fnptr!(Util::id, _)), '__try0) {
                unwrap_break_err!(resolveAllRegular(cref.clone(), original_cref.clone(), eqn_name.clone(), skip_idx, size, iter_size, &frames, rep.clone(), map.clone(), m, mapping, modes.clone(), unwrap_break_err!(Type::sizeOf(&ty, true), '__try0)), '__try0);
            } else if unwrap_break_err!(List::any(&regulars, &fnptr!(Util::id, _)), '__try0) {
                unwrap_break_err!(resolveMixed(&cref, original_cref.clone(), eqn_name.clone(), skip_idx, ty.clone(), &frames, regulars.clone(), map.clone(), m, mapping, modes.clone()), '__try0);
            } else {
                unwrap_break_err!(resolveAllReduced(cref.clone(), original_cref.clone(), eqn_name.clone(), skip_idx, size, iter_size, &frames, rep.clone(), map.clone(), m, mapping, modes.clone()), '__try0);
            }
        }
        Ok::<_, &'static str>((cref.clone(), d.clone(), skip_lst.clone(), start.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            cref = __try0_o0;
            d = __try0_o1;
            skip_lst = __try0_o2;
            start = __try0_o3;
        }
        Err(__try0_err) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBSlice.resolveDependency"));
                    __mm_s.push_str(&*literal!(" failed for: "));
                    __mm_s.push_str(&*ComponentRef::toString(&original_cref)?);
                    __mm_s.push_str(&*literal!("."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err(__try0_err);
        }
    }
    Ok(())
}

fn resolveAllRegular(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut original_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut eqn_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut skip_idx: i32,
    mut size: i32,
    mut iter_size: i32,
    mut frames: &metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>,
    mut rep: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut m: &IntMatrix::Builder,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut modes: metamodelica::Ref<Vector::Vector<metamodelica::Ref<Mode::Mode>>>,
    mut stride: i32,
) -> Result<()> {
    let mut mode: i32;
    let mut scalarized: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut scal_indices: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut idx_lst: Val2;
    let mut scal_size: i32 = 0;
    let mut shift: i32;
    let mut body_size: i32 = if (iter_size > 0) { intDiv(size, iter_size) } else { size };
    mode = Modes::add(
        modes.clone(),
        Mode::create(eqn_name.clone(), list![original_cref.clone()], false)?,
    );
    scalarized = ComponentRef::scalarizeAll(cref.clone(), true)?.reverse();
    for mut scal in &*scalarized {
        idx_lst = getCrefInFrameIndices(scal.clone(), frames, mapping, map.clone(), true)?;
        scal_indices = metamodelica::cons(idx_lst.clone(), scal_indices);
        scal_size = scal_size + ((idx_lst).len() as i32);
    }
    scal_indices = scal_indices.reverse();
    if scal_size > 0
        && (size == scal_size || UnorderedSet::contains(cref.clone(), rep.clone())? && intMod(size, scal_size) == 0)
    {
        shift = 0;
        for mut i in 1..=((metamodelica::real_div_checked(
            metamodelica::OrderedFloat((size) as f64),
            metamodelica::OrderedFloat((scal_size) as f64),
        )?)
        .0 as i32)
        {
            for mut indices in &*scal_indices {
                for mut scal_idx in &*indices.clone() {
                    if stride > body_size && body_size > 0 {
                        addMatrixEntry(
                            m,
                            skip_idx + intDiv(shift, body_size) * stride + intMod(shift, body_size),
                            scal_idx.clone(),
                            mode,
                        );
                    } else {
                        addMatrixEntry(m, skip_idx + shift, scal_idx.clone(), mode);
                    }
                    shift = shift + 1;
                }
            }
        }
    } else if scal_size > 0 && scal_size < size {
        resolveAllRegularPartial(
            cref,
            original_cref,
            eqn_name,
            skip_idx,
            size,
            iter_size,
            frames,
            map,
            m,
            mapping,
            modes,
        )?;
    } else if scal_size > size {
        resolveAllReduced(
            cref,
            original_cref,
            eqn_name,
            skip_idx,
            size,
            iter_size,
            frames,
            rep,
            map,
            m,
            mapping,
            modes,
        )?;
    }
    Ok(())
}

fn resolveMixed(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut original_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut eqn_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut skip_idx: i32,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut frames: &metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>,
    mut regulars: metamodelica::List<bool>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut m: &IntMatrix::Builder,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut modes: metamodelica::Ref<Vector::Vector<metamodelica::Ref<Mode::Mode>>>,
) -> Result<()> {
    let mut stripped: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut eq_dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut key: metamodelica::Array<i32>;
    let mut map1: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::List<i32>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >;
    let mut map2: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::List<i32>, metamodelica::List<i32>>>;
    let mut scalarized: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut scal_lst: metamodelica::List<i32>;
    let mut size_comp: i32;
    let mut eq_reg: metamodelica::List<bool>;
    let mut lst: metamodelica::List<(metamodelica::Ref<Dimension::NFDimension>, bool)>;
    subs = ComponentRef::subscriptsAllWithWholeFlat(cref)?;
    dims = Type::arrayDims(ComponentRef::getSubscriptedType(cref, false)?);
    eq_dims = Type::arrayDims(ty);
    if List::compareLength(subs.clone(), dims.clone())? == 0
        && List::compareLength(subs.clone(), regulars.clone())? == 0
    {
        stripped = ComponentRef::stripSubscriptsAll(cref);
        key = arrayCreate(((subs).len() as i32), 0);
        map1 = UnorderedMap::new(
            (std::sync::Arc::new(move |__a0: metamodelica::List<i32>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(keyHash(&__a0))
            }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<i32> + 'static>),
            (std::sync::Arc::new(keyEqual)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::List<i32>, metamodelica::List<i32>) -> Result<bool> + 'static,
                >),
            1,
        );
        resolveReductions(
            &(List::zip3(subs.clone(), dims, regulars.clone())),
            map1.clone(),
            key.clone(),
            &stripped,
            &(metamodelica::nil()),
            1,
        )?;
        map2 = UnorderedMap::new(
            (std::sync::Arc::new(move |__a0: metamodelica::List<i32>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(keyHash(&__a0))
            }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<i32> + 'static>),
            (std::sync::Arc::new(keyEqual)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::List<i32>, metamodelica::List<i32>) -> Result<bool> + 'static,
                >),
            1,
        );
        for mut k in &*UnorderedMap::keyList(map1.clone()) {
            scalarized = UnorderedMap::getSafe(
                k.clone(),
                map1.clone(),
                metamodelica::sourceInfo!("NBackEnd/Util/NBSlice.mo"),
            )?;
            scal_lst = List::flatten(
                ({
                    let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
                    for mut scal in (scalarized).into_iter().cloned() {
                        let __x = getCrefInFrameIndices(scal.clone(), frames, mapping, map.clone(), true)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?;
            UnorderedMap::add(k.clone(), scal_lst, map2.clone())?;
        }
        size_comp = List::compareLength(eq_dims.clone(), regulars.clone())?;
        if size_comp > 0 {
            eq_reg = listAppend(
                regulars.clone(),
                List::fill(false, ((eq_dims).len() as i32) - ((regulars).len() as i32)),
            );
            lst = List::zip(eq_dims, eq_reg);
        } else if size_comp < 0 {
            lst = resolveMixedDimensions(&eq_dims, &regulars, metamodelica::nil())?;
        } else {
            lst = List::zip(eq_dims, regulars);
        }
        key = arrayCreate(((subs).len() as i32), 0);
        resolveEquationDimensions(
            &lst,
            map2,
            key.clone(),
            m,
            Modes::add(modes, Mode::create(eqn_name, list![original_cref], false)?),
            Pointer::create(skip_idx),
            1,
        )?;
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBSlice.resolveMixed"));
                __mm_s.push_str(&*literal!(
                    " failed because subscripts, dimensions and dependencies were not of equal length.\n"
                ));
                __mm_s.push_str(&*literal!("variable subscripts("));
                __mm_s.push_str(&*intString(((subs).len() as i32)));
                __mm_s.push_str(&*literal!("): "));
                __mm_s.push_str(&*List::toString(
                    subs,
                    &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| Subscript::toString(&__a0),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*literal!("variable dimensions("));
                __mm_s.push_str(&*intString(((dims).len() as i32)));
                __mm_s.push_str(&*literal!("): "));
                __mm_s.push_str(&*List::toString(
                    dims,
                    &move |__a0: metamodelica::Ref<Dimension::NFDimension>| Dimension::toString(&__a0),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*literal!("equation dimensions("));
                __mm_s.push_str(&*intString(((eq_dims).len() as i32)));
                __mm_s.push_str(&*literal!("): "));
                __mm_s.push_str(&*List::toString(
                    eq_dims,
                    &move |__a0: metamodelica::Ref<Dimension::NFDimension>| Dimension::toString(&__a0),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*literal!("variable dependencies("));
                __mm_s.push_str(&*intString(((regulars).len() as i32)));
                __mm_s.push_str(&*literal!("): "));
                __mm_s.push_str(&*List::toString(
                    regulars,
                    &fnptr!(boolString, bool),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    Ok(())
}

fn resolveMixedDimensions(
    mut eq_dims: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut regulars: &metamodelica::List<bool>,
    mut lst: metamodelica::List<(metamodelica::Ref<Dimension::NFDimension>, bool)>,
) -> Result<metamodelica::List<(metamodelica::Ref<Dimension::NFDimension>, bool)>> {
    let mut lst: metamodelica::List<(metamodelica::Ref<Dimension::NFDimension>, bool)> = lst;
    lst = (::match_deref::match_deref! { match (eq_dims, regulars) {
        (_, Deref @ metamodelica::ListNode::Cons { head: false, tail: rest_reg }) => {
            metamodelica::cons((Dimension::fromExp(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }), Variability::CONSTANT.clone())?, false), resolveMixedDimensions(eq_dims, rest_reg, metamodelica::nil())?)
        },
        (Deref @ metamodelica::ListNode::Cons { head: dim, tail: rest_dim }, Deref @ metamodelica::ListNode::Cons { head: true, tail: rest_reg }) => {
            metamodelica::cons((dim.clone(), true), resolveMixedDimensions(rest_dim, rest_reg, metamodelica::nil())?)
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(lst)
}

fn resolveAllReduced(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut original_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut eqn_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut skip_idx: i32,
    mut size: i32,
    mut iter_size: i32,
    mut frames: &metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>,
    mut rep: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut m: &IntMatrix::Builder,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut modes: metamodelica::Ref<Vector::Vector<metamodelica::Ref<Mode::Mode>>>,
) -> Result<()> {
    let mut repeated: bool;
    let mut scalarized: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut scal_indices: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut shift: i32;
    let mut mode: i32;
    repeated = UnorderedSet::contains(cref.clone(), rep)?;
    scalarized = ComponentRef::scalarizeAll(cref, true)?.reverse();
    for mut scal in &*scalarized {
        scal_indices = metamodelica::cons(
            getCrefInFrameIndices(scal.clone(), frames, mapping, map.clone(), true)?,
            scal_indices,
        );
    }
    scal_indices = scal_indices.reverse();
    mode = Modes::add(modes, Mode::create(eqn_name, list![original_cref], !(repeated))?);
    for mut i in ({
        let __s = skip_idx;
        let __e = skip_idx + size - iter_size;
        let __step = iter_size;
        (0i32..)
            .map(move |__k| __s + __k * __step)
            .take_while(move |&__v| __step != 0 && (if __step > 0 { __v <= __e } else { __v >= __e }))
    }) {
        shift = 0;
        for mut indices in &*scal_indices {
            for mut scal_idx in &*indices.clone() {
                if intMod(shift, iter_size) == 0 {
                    shift = 0;
                }
                addMatrixEntry(m, i + shift, scal_idx.clone(), mode);
                shift = shift + 1;
            }
        }
    }
    Ok(())
}

fn resolveAllRegularPartial(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut original_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut eqn_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut skip_idx: i32,
    mut size: i32,
    mut iter_size: i32,
    mut frames: &metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut m: &IntMatrix::Builder,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut modes: metamodelica::Ref<Vector::Vector<metamodelica::Ref<Mode::Mode>>>,
) -> Result<()> {
    let mut mode: i32;
    let mut final_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut var_arr_idx: i32;
    let mut var_start: i32;
    let mut sizes: metamodelica::List<i32>;
    let mut subs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut body_size: i32;
    let mut row: Pointer::Pointer<i32>;
    let mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >;
    mode = Modes::add(modes, Mode::create(eqn_name, list![original_cref], false)?);
    (final_cref, var_arr_idx) = getVarArrIdx(cref.clone(), mapping, map)?;
    (var_start, _) = ({
        let __elt = (*metamodelica::index_checked(&mapping.var_AtS.borrow(), var_arr_idx)?).clone();
        __elt
    });
    sizes = ComponentRef::sizes(&final_cref, false, true, metamodelica::nil())?;
    subs = ComponentRef::subscriptsToExpression(&cref, true)?;
    body_size = intDiv(size, iter_size);
    row = Pointer::create(skip_idx);
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
    resolveFrames(
        frames,
        &sizes,
        &subs,
        var_start,
        replacements,
        true,
        m,
        mode,
        body_size,
        row,
    )?;
    Ok(())
}

fn resolveFrames(
    mut frames: &metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>,
    mut sizes: &metamodelica::List<i32>,
    mut subs: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut var_start: i32,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut resize: bool,
    mut m: &IntMatrix::Builder,
    mut mode: i32,
    mut body_size: i32,
    mut row: Pointer::Pointer<i32>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match frames {
        Deref @ metamodelica::ListNode::Nil => {
            let mut r: i32;
            let mut values: metamodelica::List<metamodelica::List<i32>>;
            r = Pointer::access(row.clone());
            values = resolveDimensionsSubscripts(sizes.clone(), subs.clone(), replacements, resize)?;
            for mut v in &*values.reverse() {
                addMatrixEntry(m, r, locationToIndex(sizes.clone(), v.clone(), var_start)?, mode);
            }
            Pointer::update(row, r + body_size);
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (iterator, range, fmap), tail: rest } => {
            let mut start: i32;
            let mut step: i32;
            let mut stop: i32;
            let mut sub_idx: i32;
            let mut iterator_exps: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut iterator_lst: metamodelica::List<i32>;
            iterator_lst = (match &*range.clone() {
        Expression::RANGE { .. } => {
            (start, step, stop) = Expression::getIntegerRange(range.clone(), resize)?;
            List::intRange3(start, step, stop)?
        },
        Expression::ARRAY { .. } => {
            iterator_exps = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut e in (var_field!((**range).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = Expression::map(e.clone(), (std::sync::Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            iterator_lst = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut e in (iterator_exps).into_iter().cloned() {
            let __x = Expression::integerValue(SimplifyExp::simplifyDump(e.clone(), true, &(literal!("NBSlice.resolveFrames")), &(literal!("")))?)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            iterator_lst
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.resolveFrames")); __mm_s.push_str(&*literal!(" failed to parse iterator range: ")); __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&iterator))?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toString(range.clone())?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
    });
            sub_idx = 1;
            for mut index in &*iterator_lst {
                UnorderedMap::add(iterator.clone(), metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: index.clone() }), replacements.clone())?;
                Iterator::createMappedLocationReplacement(fmap.clone(), sub_idx, replacements.clone())?;
                resolveFrames(rest, sizes, subs, var_start, replacements.clone(), resize, m, mode, body_size, row.clone())?;
                sub_idx = sub_idx + 1;
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.resolveFrames")); __mm_s.push_str(&*literal!(" failed for an unknown reason.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn resolveEquationDimensions(
    mut lst: &metamodelica::List<(metamodelica::Ref<Dimension::NFDimension>, bool)>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::List<i32>, metamodelica::List<i32>>>,
    mut key: metamodelica::Array<i32>,
    mut m: &IntMatrix::Builder,
    mut mode: i32,
    mut eqn_idx_ptr: Pointer::Pointer<i32>,
    mut index: i32,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match lst {
        Deref @ metamodelica::ListNode::Nil => {
            let mut eqn_idx: i32;
            let mut scal_lst: metamodelica::List<i32>;
            eqn_idx = Pointer::access(eqn_idx_ptr.clone());
            scal_lst = UnorderedMap::getSafe(key.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>(), map, metamodelica::sourceInfo!("NBackEnd/Util/NBSlice.mo"))?;
            for mut scal_idx in &*scal_lst {
                addMatrixEntry(m, eqn_idx, scal_idx.clone(), mode);
            }
            Pointer::update(eqn_idx_ptr, eqn_idx + 1);
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (dim, false), tail: rest } => {
            for mut i in 1..=Dimension::size(metamodelica::AsArg::as_arg(&dim), true)? {
                resolveEquationDimensions(rest, map.clone(), key.clone(), m, mode, eqn_idx_ptr.clone(), index + 1)?;
            }
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (dim, true), tail: rest } => {
            for mut i in 1..=Dimension::size(metamodelica::AsArg::as_arg(&dim), true)? {
                metamodelica::arrayUpdate(key.clone(), index, i)?;
                resolveEquationDimensions(rest, map.clone(), key.clone(), m, mode, eqn_idx_ptr.clone(), index + 1)?;
            }
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn addMatrixEntry(mut m: &IntMatrix::Builder, mut eqn_idx: i32, mut var_idx: i32, mut mode: i32) -> () {
    if var_idx > 0 {
        IntMatrix::builderAddAux(m, eqn_idx, var_idx, mode);
    }
    ()
}

fn resolveReductions(
    mut lst: &metamodelica::List<(
        metamodelica::Ref<Subscript::NFSubscript>,
        metamodelica::Ref<Dimension::NFDimension>,
        bool,
    )>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::List<i32>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
    mut key: metamodelica::Array<i32>,
    mut stripped: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut acc: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut index: i32,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match lst {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut val: Val1;
            cref = ComponentRef::mergeSubscripts(acc.clone().reverse(), stripped.clone(), true, false, false)?;
            val = ComponentRef::scalarizeAll(cref, true)?;
            UnorderedMap::add(key.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>(), val, map)?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (sub, _, false), tail: rest } => {
            resolveReductions(rest, map, key.clone(), stripped, &(metamodelica::cons(sub.clone(), acc.clone())), index + 1)?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (sub, dim, true), tail: rest } => {
            let mut sub_idx: i32;
            sub_idx = 1;
            for mut s in &*Subscript::scalarize(sub.clone(), metamodelica::AsArg::as_arg(&dim), true)? {
                metamodelica::arrayUpdate(key.clone(), index, sub_idx)?;
                resolveReductions(rest, map.clone(), key.clone(), stripped, &(metamodelica::cons(s.clone(), acc.clone())), index + 1)?;
                sub_idx = sub_idx + 1;
            }
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn combineFrames2Indices(
    mut first: i32,
    mut sizes: &metamodelica::List<i32>,
    mut subs: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut frames: &metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut resize: bool,
    mut indices: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut indices: metamodelica::List<i32> = indices;
    indices = (::match_deref::match_deref! { match frames {
        Deref @ metamodelica::ListNode::Nil => {
            let mut values: metamodelica::List<metamodelica::List<i32>>;
            values = resolveDimensionsSubscripts(sizes.clone(), subs.clone(), replacements, resize)?;
            ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut v in (values).into_iter().cloned() {
            let __x = locationToIndex(sizes.clone(), v.clone(), first)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        Deref @ metamodelica::ListNode::Cons { head: (iterator, range, map), tail: rest } => {
            let mut start: i32;
            let mut step: i32;
            let mut stop: i32;
            let mut values: metamodelica::List<metamodelica::List<i32>>;
            let mut iterator_exps: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut iterator_lst: metamodelica::List<i32>;
            let mut sub_idx: i32;
            iterator_lst = (match &*range.clone() {
        Expression::RANGE { .. } => {
            (start, step, stop) = Expression::getIntegerRange(range.clone(), resize)?;
            List::intRange3(start, step, stop)?
        },
        Expression::ARRAY { .. } => {
            iterator_exps = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut e in (var_field!((**range).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = Expression::map(e.clone(), (std::sync::Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            iterator_lst = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut e in (iterator_exps).into_iter().cloned() {
            let __x = Expression::integerValue(SimplifyExp::simplifyDump(e.clone(), true, &(literal!("NBSlice.combineFrames2Indices")), &(literal!("")))?)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            iterator_lst
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.combineFrames2Indices")); __mm_s.push_str(&*literal!(" failed because iterator binding could not be parsed: ")); __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&iterator))?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toString(range.clone())?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
    });
            sub_idx = 1;
            for mut index in &*iterator_lst {
                UnorderedMap::add(iterator.clone(), metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: index.clone() }), replacements.clone())?;
                Iterator::createMappedLocationReplacement(map.clone(), sub_idx, replacements.clone())?;
                if (rest).is_empty() {
                    values = resolveDimensionsSubscripts(sizes.clone(), subs.clone(), replacements.clone(), resize)?;
                    for mut v in &*values.reverse() {
                        indices = metamodelica::cons(locationToIndex(sizes.clone(), v.clone(), first)?, indices);
                    }
                } else {
                    indices = combineFrames2Indices(first, sizes, subs, rest, replacements.clone(), resize, indices)?;
                }
                sub_idx = sub_idx + 1;
            }
            indices
        },
        Deref @ metamodelica::ListNode::Cons { head: (iterator, range, _), tail: _ } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.combineFrames2Indices")); __mm_s.push_str(&*literal!(" failed because uniontype records are wrong: ")); __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&iterator))?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toString(range.clone())?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.combineFrames2Indices")); __mm_s.push_str(&*literal!(" failed for an unknown reason.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(indices)
}

fn combineFramesIndices(
    mut first: i32,
    mut sizes: metamodelica::List<i32>,
    mut subs: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut frames: &metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>,
    mut resize: bool,
) -> Result<metamodelica::List<i32>> {
    let mut indices: metamodelica::List<i32>;
    let mut ok: bool;
    (ok, indices) = combineFramesArithmetic(first, sizes.clone(), subs, frames)?;
    if !(ok) {
        indices = combineFrames2Indices(
            first,
            &sizes,
            subs,
            frames,
            UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            ),
            resize,
            metamodelica::nil(),
        )?
        .reverse();
    }
    Ok(indices)
}

fn combineFramesArithmetic(
    mut first: i32,
    mut sizes: metamodelica::List<i32>,
    mut subs: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut frames: &metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>,
) -> Result<(bool, metamodelica::List<i32>)> {
    let mut ok: bool = true;
    let mut indices: metamodelica::List<i32> = metamodelica::nil();
    let mut n: i32 = ((frames).len() as i32);
    let mut sz: i32 = if (n > 0) { n } else { 1 };
    let mut names: metamodelica::Array<metamodelica::Ref<ComponentRef::NFComponentRef>> =
        arrayCreate(sz, openmodelica_nf_frontend::NFComponentRef::interned_EMPTY());
    let mut values: metamodelica::Array<i32> = arrayCreate(sz, 0);
    let mut starts: metamodelica::Array<i32> = arrayCreate(sz, 0);
    let mut steps: metamodelica::Array<i32> = arrayCreate(sz, 0);
    let mut stops: metamodelica::Array<i32> = arrayCreate(sz, 0);
    let mut i: i32 = 1;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut map: Option<metamodelica::Ref<Iterator::Iterator>>;
    if (subs).is_empty() {
        return Ok((ok, indices));
    }
    for mut frame in &**frames {
        (name, range, map) = frame.clone();
        if (map).is_some() {
            ok = false;
            return Ok((ok, indices));
        }
        metamodelica::arrayUpdate(names.clone(), i, name)?;
        let () = (::match_deref::match_deref! { match &(range) {
            Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: rstart }, stop: Deref @ Expression::INTEGER { value: rstop }, step: __range_step, .. } => {
                let mut rstep: i32;
                rstep = (::match_deref::match_deref! { match &(__range_step.clone()) {
            None => if (rstart.clone() > rstop.clone()) {-1} else {1},
            Some(Deref @ Expression::INTEGER { value: __esc_rstep }) => {
                rstep = (*__esc_rstep).clone();
                rstep.clone()
            },
            _ => 0,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                if rstep == 0 {
                    ok = false;
                } else {
                    metamodelica::arrayUpdate(starts.clone(), i, rstart.clone())?;
                    metamodelica::arrayUpdate(steps.clone(), i, rstep)?;
                    metamodelica::arrayUpdate(stops.clone(), i, rstop.clone())?;
                }
                ()
            },
            _ => {
                ok = false;
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if !(ok) {
            return Ok((ok, indices));
        }
        i = i + 1;
    }
    (ok, indices) = combineFramesArithmeticWork(
        first,
        sizes,
        subs,
        names.clone(),
        values.clone(),
        starts.clone(),
        steps.clone(),
        stops.clone(),
        1,
        n,
        ok,
        indices,
    )?;
    if ok {
        indices = indices.reverse();
    }
    Ok((ok, indices))
}

fn combineFramesArithmeticWork(
    mut first: i32,
    mut sizes: metamodelica::List<i32>,
    mut subs: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut names: metamodelica::Array<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut values: metamodelica::Array<i32>,
    mut starts: metamodelica::Array<i32>,
    mut steps: metamodelica::Array<i32>,
    mut stops: metamodelica::Array<i32>,
    mut level: i32,
    mut nframes: i32,
    mut ok: bool,
    mut indices: metamodelica::List<i32>,
) -> Result<(bool, metamodelica::List<i32>)> {
    let mut ok: bool = ok;
    let mut indices: metamodelica::List<i32> = indices;
    let mut v: i32;
    let mut step: i32;
    let mut stop: i32;
    let mut index: i32;
    let mut inBounds: bool;
    if level > nframes {
        (ok, inBounds, index) = evalFrameIndex(subs, sizes, names.clone(), values.clone(), nframes, first)?;
        if ok && inBounds {
            indices = metamodelica::cons(index, indices);
        }
    } else {
        step = ({
            let __elt = (*metamodelica::index_checked(&steps.borrow(), level)?).clone();
            __elt
        });
        stop = ({
            let __elt = (*metamodelica::index_checked(&stops.borrow(), level)?).clone();
            __elt
        });
        v = ({
            let __elt = (*metamodelica::index_checked(&starts.borrow(), level)?).clone();
            __elt
        });
        while step > 0 && v <= stop || step < 0 && v >= stop {
            metamodelica::arrayUpdate(values.clone(), level, v)?;
            (ok, indices) = combineFramesArithmeticWork(
                first,
                sizes.clone(),
                subs,
                names.clone(),
                values.clone(),
                starts.clone(),
                steps.clone(),
                stops.clone(),
                level + 1,
                nframes,
                ok,
                indices,
            )?;
            if !(ok) {
                return Ok((ok, indices));
            }
            v = v + step;
        }
    }
    Ok((ok, indices))
}

fn evalFrameIndex(
    mut subs: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut sizes: metamodelica::List<i32>,
    mut names: metamodelica::Array<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut values: metamodelica::Array<i32>,
    mut nframes: i32,
    mut first: i32,
) -> Result<(bool, bool, i32)> {
    let mut ok: bool = true;
    let mut inBounds: bool = true;
    let mut index: i32 = first;
    let mut rest_sizes: metamodelica::List<i32> = sizes;
    let mut size: i32;
    let mut value: i32;
    let mut factor: i32 = 1;
    for mut sub in &**subs {
        if (rest_sizes).is_empty() {
            ok = false;
            return Ok((ok, inBounds, index));
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_sizes) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        size = metamodelica::Own::own(__pa0);
        rest_sizes = metamodelica::Own::own(__pa1);
        (ok, value) = evalFrameExp(
            metamodelica::AsArg::as_arg(&sub),
            names.clone(),
            values.clone(),
            nframes,
        )?;
        if !(ok) {
            return Ok((ok, inBounds, index));
        }
        if value < 1 || value > size {
            inBounds = false;
            return Ok((ok, inBounds, index));
        }
        index = index + (value - 1) * factor;
        factor = factor * size;
    }
    Ok((ok, inBounds, index))
}

fn evalFrameExp(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut names: metamodelica::Array<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut values: metamodelica::Array<i32>,
    mut nframes: i32,
) -> Result<(bool, i32)> {
    let mut ok: bool;
    let mut value: i32;
    (ok, value) = (match &**exp {
        Expression::INTEGER { value: __exp_value } => (true, __exp_value.clone()),
        Expression::CREF { cref: __exp_cref, .. } => {
            ok = false;
            value = 0;
            for mut i in 1..=nframes {
                if ComponentRef::isEqual(
                    metamodelica::AsArg::as_arg(&__exp_cref),
                    &({
                        let __elt = (*metamodelica::index_checked(&names.borrow(), i)?).clone();
                        __elt
                    }),
                )? {
                    value = ({
                        let __elt = (*metamodelica::index_checked(&values.borrow(), i)?).clone();
                        __elt
                    });
                    ok = true;
                    break;
                }
            }
            (ok, value)
        }
        Expression::BINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            operator: __exp_operator,
        } if (Type::isInteger(&(Operator::typeOf(metamodelica::AsArg::as_arg(&__exp_operator))))?) => {
            let mut v1: i32;
            let mut v2: i32;
            let mut ok1: bool;
            let mut ok2: bool;
            (ok1, v1) = evalFrameExp(
                metamodelica::AsArg::as_arg(&__exp_exp1),
                names.clone(),
                values.clone(),
                nframes,
            )?;
            (ok2, v2) = evalFrameExp(
                metamodelica::AsArg::as_arg(&__exp_exp2),
                names.clone(),
                values.clone(),
                nframes,
            )?;
            value = 0;
            if ok1 && ok2 {
                (ok, value) = (match __exp_operator.op.clone() {
                    Operator::Op::ADD => (true, v1 + v2),
                    Operator::Op::SUB => (true, v1 - v2),
                    Operator::Op::MUL => (true, v1 * v2),
                    _ => (false, 0),
                });
            } else {
                ok = false;
            }
            (ok, value)
        }
        Expression::UNARY {
            exp: __exp_exp,
            operator: __exp_operator,
        } if (Type::isInteger(&(Operator::typeOf(metamodelica::AsArg::as_arg(&__exp_operator))))?
            && __exp_operator.op.clone() == Operator::Op::UMINUS.clone()) =>
        {
            (ok, value) = evalFrameExp(
                metamodelica::AsArg::as_arg(&__exp_exp),
                names.clone(),
                values.clone(),
                nframes,
            )?;
            value = -(value);
            (ok, value)
        }
        _ => (false, 0),
    });
    Ok((ok, value))
}

fn getCrefInFrameIndices(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut frames: &metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut resize: bool,
) -> Result<metamodelica::List<i32>> {
    let mut scal_lst: metamodelica::List<i32>;
    let mut final_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut var_arr_idx: i32;
    let mut var_start: i32;
    (final_cref, var_arr_idx) = getVarArrIdx(cref.clone(), mapping, map)?;
    (var_start, _) = ({
        let __elt = (*metamodelica::index_checked(&mapping.var_AtS.borrow(), var_arr_idx)?).clone();
        __elt
    });
    scal_lst = getCrefInFrameIndicesLocal(&cref, &final_cref, frames, var_start, resize)?;
    Ok(scal_lst)
}

fn getVarArrIdx(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
) -> Result<(metamodelica::Ref<ComponentRef::NFComponentRef>, i32)> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut var_arr_idx: i32;
    (var_arr_idx, cref) = (match UnorderedMap::get(cref.clone(), map.clone())? {
        Some(mut __esc_var_arr_idx) => {
            var_arr_idx = __esc_var_arr_idx.clone();
            (var_arr_idx, cref)
        }
        _ => {
            cref = ComponentRef::stripSubscriptsAll(&cref);
            (
                UnorderedMap::getSafe(cref.clone(), map, metamodelica::sourceInfo!("NBackEnd/Util/NBSlice.mo"))?,
                cref,
            )
        }
    });
    Ok((cref, var_arr_idx))
}

pub(crate) fn getCrefInFrameIndicesLocal(
    mut subscripted_cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut stripped_cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut frames: &metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>,
    mut var_start: i32,
    mut resize: bool,
) -> Result<metamodelica::List<i32>> {
    let mut scal_lst: metamodelica::List<i32>;
    let mut sizes: metamodelica::List<i32>;
    let mut subs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut complex_size: i32;
    sizes = ComponentRef::sizes(stripped_cref, false, resize, metamodelica::nil())?;
    subs = ComponentRef::subscriptsToExpression(subscripted_cref, true)?;
    ty = Type::arrayElementType(&(ComponentRef::getComponentType(subscripted_cref)));
    scal_lst = (match Type::complexSize(&ty, false)? {
        Some(mut __esc_complex_size) => {
            complex_size = __esc_complex_size.clone();
            scal_lst = metamodelica::nil();
            for mut i in ({
                let __s = complex_size;
                let __e = 1;
                (0i32..)
                    .map(move |__k| __s + __k * (-1))
                    .take_while(move |&__v| __v >= __e)
            }) {
                scal_lst = listAppend(
                    combineFramesIndices(
                        var_start,
                        metamodelica::cons(complex_size, sizes.clone()),
                        &(metamodelica::cons(
                            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i }),
                            subs.clone(),
                        )),
                        frames,
                        resize,
                    )?,
                    scal_lst,
                );
            }
            scal_lst
        }
        _ => combineFramesIndices(var_start, sizes, &subs, frames, resize)?,
    });
    Ok(scal_lst)
}

fn resolveDimensionsSubscripts(
    mut sizes: metamodelica::List<i32>,
    mut subs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut resize: bool,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut values: metamodelica::List<metamodelica::List<i32>>;
    let mut replaced: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    replaced = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut sub in (subs).into_iter().cloned() {
            let __x = Expression::map(
                sub.clone(),
                (std::sync::Arc::new({
                    let __pe_b1 = replacements.clone();
                    move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    values = ({
        let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
        let __thr_src0 = replaced;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = sizes;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(exp), Some(size)) => {
                    let __x = resolveDimensionsSubscript(exp.clone(), size.clone(), resize)?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    values = List::combination(&values);
    Ok(values)
}

fn resolveDimensionsSubscript(
    mut replaced: metamodelica::Ref<Expression::NFExpression>,
    mut size: i32,
    mut resize: bool,
) -> Result<metamodelica::List<i32>> {
    let mut res: metamodelica::List<i32>;
    let mut rep: metamodelica::Ref<Expression::NFExpression>;
    rep = SimplifyExp::simplifyDump(
        replaced,
        true,
        &(literal!("NBSlice.resolveDimensionsSubscript")),
        &(literal!("")),
    )?;
    res = (match &*rep {
        Expression::INTEGER { value: __rep_value } => {
            if (__rep_value.clone() >= 1 && __rep_value.clone() <= size) {
                list![__rep_value.clone()]
            } else {
                metamodelica::nil()
            }
        }
        Expression::ENUM_LITERAL { index: __rep_index, .. } => {
            list![__rep_index.clone()]
        }
        Expression::RANGE { .. } => {
            let mut start: i32;
            let mut step: i32;
            let mut stop: i32;
            (start, step, stop) = Expression::getIntegerRange(rep, resize)?;
            List::intRange3(start, step, stop)?
        }
        Expression::ARRAY { .. } => List::flatten(
            ({
                let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
                for mut e in (var_field!((*rep).elements, Expression::NFExpression::ARRAY).clone())
                    .borrow()
                    .iter()
                {
                    let __x = resolveDimensionsSubscript(e.clone(), size, resize)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?,
        _ => List::intRange(size),
    });
    Ok(res)
}

fn applyNewFrameRange(
    mut frame: (
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    ),
    mut range: (i32, i32, i32),
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<Expression::NFExpression>,
    Option<metamodelica::Ref<Iterator::Iterator>>,
)> {
    let mut frame: (
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    ) = frame;
    frame = (::match_deref::match_deref! { match &(frame) {
        (name, exp @ Deref @ Expression::RANGE { .. }, map) => {
            (name.clone(), Expression::sliceRange(exp.clone(), range)?, map.clone())
        },
        (_, exp, _) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.applyNewFrameRange")); __mm_s.push_str(&*literal!(" failed because frame expression was not Expression.RANGE(): ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(frame)
}
