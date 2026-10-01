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

use crate::NBBackendUtil as BackendUtil;
use crate::NBDetectStates as DetectStates;
use crate::NBEvaluation as Evaluation;
use crate::NBInline as Inline;
use crate::NBReplacements as Replacements;
use crate::NBResizable::EvalOrder;
use crate::NBSlice as Slice;
use crate::NBSolve as Solve;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn::Path;
use openmodelica_backend_types::BackendDAE as OldBackendDAE;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFBackendExtension::OptimizerExpression;
use openmodelica_nf_frontend::NFBackendExtension::VariableAttributes;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClass as Class;
use openmodelica_nf_frontend::NFComplexType as ComplexType;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstContext;
use openmodelica_nf_frontend::NFInstNode;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes::Purity;
use openmodelica_nf_frontend::NFPrefixes::Variability;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFSimplifyModel as SimplifyModel;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFTyping as Typing;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;
use openmodelica_util_datatypes_basic::Pointer;

// Old Frontend imports
// New Frontend imports
// Old Backend imports
// New Backend imports
// Util imports
pub(crate) const SIMULATION_STR: &'static str = "SIM";

pub(crate) const START_STR: &'static str = "SRT";

pub(crate) const PRE_STR: &'static str = "PRE";

pub(crate) const TMP_STR: &'static str = "TMP";

// mainly used for mapping purposes
pub type EquationPointer = Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;

pub type EqnSlice = metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>;

// used to process different outcomes of slicing from Util/Slice.mo
// have to be defined here and not in Util/Slice.mo because it is a uniontype and not a package
/// iterator-like tuple for array handling
pub type Frame = (
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<Expression::NFExpression>,
    Option<metamodelica::Ref<Iterator::Iterator>>,
);

/// sliced frame at specific sub locations
pub type FrameLocation = (
    metamodelica::Array<i32>,
    (
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    ),
);

/// final result of slicing
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum SlicingStatus {
    UNCHANGED = 1,
    TRIVIAL = 2,
    NONTRIVIAL = 3,
    FAILURE = 4,
}
impl PartialOrd for SlicingStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SlicingStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for SlicingStatus {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

/// result of sub-routine recollect
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum RecollectStatus {
    SUCCESS = 1,
    FAILURE = 2,
}
impl PartialOrd for RecollectStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RecollectStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for RecollectStatus {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

/// result of sub-routine frame ordering
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum FrameOrderingStatus {
    UNCHANGED = 1,
    CHANGED = 2,
    FAILURE = 3,
}
impl PartialOrd for FrameOrderingStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for FrameOrderingStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for FrameOrderingStatus {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

/// type for collecting data in hash maps
pub type CrefLst = metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;

pub type MapFuncEqn = std::sync::Arc<
    dyn ::std::ops::Fn(metamodelica::Ref<Equation::Equation>) -> Result<metamodelica::Ref<Equation::Equation>>
        + 'static,
>;

pub type MapFuncEqnPtr = std::sync::Arc<
    dyn ::std::ops::Fn(
            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        ) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>
        + 'static,
>;

pub type MapFuncExp = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<Expression::NFExpression>,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
        + 'static,
>;

pub type MapFuncExpWrapper = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<Expression::NFExpression>,
            MapFuncExp,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
        + 'static,
>;

pub type MapFuncCref = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
        + 'static,
>;

pub type checkEqn = std::sync::Arc<
    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<bool> + 'static,
>;

pub mod Iterator {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Iterator {
        SINGLE {
            /// the name of the iterator
            name: metamodelica::Ref<ComponentRef::NFComponentRef>,
            /// range as <start, step, stop>
            range: metamodelica::Ref<Expression::NFExpression>,
            /// maps to a second iterator if derived from a for-expression
            map: Option<metamodelica::Ref<Iterator>>,
        },
        NESTED {
            /// sorted iterator names
            names: metamodelica::Array<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            /// sorted ranges as <start, step, stop>
            ranges: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
            /// maps to a second iterator if derived from a for-expression
            maps: metamodelica::Array<Option<metamodelica::Ref<Iterator>>>,
        },
        EMPTY,
    }
    impl metamodelica::gc::MMTrace for Iterator {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Iterator::SINGLE { name, range, map } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(range, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(map, __mmv)?;
                    Ok(())
                }
                Iterator::NESTED { names, ranges, maps } => {
                    metamodelica::gc::MMTrace::mm_accept(names, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(ranges, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(maps, __mmv)?;
                    Ok(())
                }
                Iterator::EMPTY => Ok(()),
            }
        }
    }
    impl Iterator {
        pub fn interned_EMPTY() -> metamodelica::Ref<Iterator> {
            thread_local! {
                static INTERNED: metamodelica::Ref<Iterator> = metamodelica::Ref::new(Iterator::EMPTY);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_EMPTY() -> metamodelica::Ref<Iterator> {
        Iterator::interned_EMPTY()
    }
    impl Default for Iterator {
        fn default() -> Self {
            Self::EMPTY
        }
    }
    pub use self::Iterator::{EMPTY, NESTED, SINGLE};
    pub(crate) fn createFrame(
        mut iter: (
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        ),
        mut set: metamodelica::Ref<
            UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        >,
    ) -> Result<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator>>,
    )> {
        let mut frame: (
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator>>,
        );
        frame = (::match_deref::match_deref! { match &(iter.clone()) {
            (node, range @ Deref @ Expression::RANGE { .. }) => {
                (ComponentRef::makeIterator(node.clone(), openmodelica_nf_frontend::NFType::interned_INTEGER())?, range.clone(), None)
            },
            (node, range @ Deref @ Expression::ARRAY { .. }) => {
                let mut node2: metamodelica::Ref<InstNode::InstNode>;
                let mut range2: metamodelica::Ref<Expression::NFExpression>;
                let mut map: metamodelica::Ref<Iterator>;
                let mut iter_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut iter_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
                node2 = NFInstNode::InstNode::newIterator({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&node))?); ArcStr::from(__mm_s) }, openmodelica_nf_frontend::NFType::interned_INTEGER(), metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo"));
                range2 = Expression::makeRange(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }), None, metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: Type::sizeOf(&(Expression::typeOf(range.clone())), false)? }))?;
                map = fromFrames(list![(ComponentRef::makeIterator(node.clone(), Type::arrayElementType(&(Expression::typeOf(range.clone()))))?, range.clone(), None)]);
                iter_cref = ComponentRef::makeIterator(node2, openmodelica_nf_frontend::NFType::interned_INTEGER())?;
                iter_var = BackendDAE::lowerIterator(iter_cref)?;
                iter_cref = BVariable::getVarName(iter_var.clone());
                UnorderedSet::add(iter_var, set)?;
                (iter_cref, range2, Some(map))
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.createFrame")); __mm_s.push_str(&*literal!(" failed to inline iterator expression: ")); __mm_s.push_str(&*NFInstNode::InstNode::toString(Util::tuple21(iter.clone()))?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toString(Util::tuple22(iter))?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(frame)
    }

    pub(crate) fn fromFrames(
        mut frames: metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator>>,
        )>,
    ) -> metamodelica::Ref<Iterator> {
        let mut iter: metamodelica::Ref<Iterator>;
        let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator>>>;
        let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut range: metamodelica::Ref<Expression::NFExpression>;
        let mut map: Option<metamodelica::Ref<Iterator>>;
        if (frames).is_empty() {
            iter = crate::NBEquation::Iterator::interned_EMPTY();
        } else {
            (names, ranges, maps) = List::unzip3(frames);
            iter = (::match_deref::match_deref! { match &((names.clone(), ranges.clone(), maps.clone())) {
                (Deref @ metamodelica::ListNode::Cons { head: __esc_name, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: __esc_range, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: __esc_map, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    name = (*__esc_name).clone();
                    range = (*__esc_range).clone();
                    map = (*__esc_map).clone();
                    metamodelica::Ref::new(Iterator::SINGLE { name: name.clone(), range: range.clone(), map: map.clone() })
                },
                _ => metamodelica::Ref::new(Iterator::NESTED { names: metamodelica::arrayFromVec(names.into_iter().cloned().collect()), ranges: metamodelica::arrayFromVec(ranges.into_iter().cloned().collect()), maps: metamodelica::arrayFromVec(maps.into_iter().cloned().collect()) }),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        iter
    }

    pub(crate) fn addFrames(
        mut iter: metamodelica::Ref<Iterator>,
        mut frames: metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator>>,
        )>,
    ) -> metamodelica::Ref<Iterator> {
        let mut iter: metamodelica::Ref<Iterator> = iter;
        let mut names1: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut names2: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut ranges1: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        let mut ranges2: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        let mut maps1: metamodelica::List<Option<metamodelica::Ref<Iterator>>>;
        let mut maps2: metamodelica::List<Option<metamodelica::Ref<Iterator>>>;
        if !((frames).is_empty()) {
            (names1, ranges1, maps1) = getFrames(&iter);
            (names2, ranges2, maps2) = List::unzip3(frames);
            iter = fromFrames(List::zip3(
                listAppend(names1, names2),
                listAppend(ranges1, ranges2),
                listAppend(maps1, maps2),
            ));
        }
        iter
    }

    pub(crate) fn getFrames(
        mut iter: &metamodelica::Ref<Iterator>,
    ) -> (
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        metamodelica::List<Option<metamodelica::Ref<Iterator>>>,
    ) {
        let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator>>>;
        (names, ranges, maps) = (match &**iter {
            SINGLE {
                map: __iter_map,
                name: __iter_name,
                range: __iter_range,
            } => (
                list![__iter_name.clone()],
                list![__iter_range.clone()],
                list![__iter_map.clone()],
            ),
            NESTED { .. } => (
                var_field!((**iter).names, Iterator::NESTED)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<metamodelica::List<_>>(),
                var_field!((**iter).ranges, Iterator::NESTED)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<metamodelica::List<_>>(),
                var_field!((**iter).maps, Iterator::NESTED)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<metamodelica::List<_>>(),
            ),
            EMPTY { .. } => (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()),
        });
        (names, ranges, maps)
    }

    pub(crate) fn merge(
        mut iterators: metamodelica::List<metamodelica::Ref<Iterator>>,
    ) -> Result<metamodelica::Ref<Iterator>> {
        let mut result: metamodelica::Ref<Iterator>;
        let mut tmp_names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut tmp_ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        let mut tmp_maps: metamodelica::List<Option<metamodelica::Ref<Iterator>>>;
        let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator>>> = metamodelica::nil();
        if List::hasOneElement(&iterators) {
            result = (iterators).head().cloned()?;
        } else {
            for mut iter in &*iterators.reverse() {
                (tmp_names, tmp_ranges, tmp_maps) = getFrames(metamodelica::AsArg::as_arg(&iter));
                names = listAppend(tmp_names, names);
                ranges = listAppend(tmp_ranges, ranges);
                maps = listAppend(tmp_maps, maps);
            }
            result = metamodelica::Ref::new(Iterator::NESTED {
                names: metamodelica::arrayFromVec(names.into_iter().cloned().collect()),
                ranges: metamodelica::arrayFromVec(ranges.into_iter().cloned().collect()),
                maps: metamodelica::arrayFromVec(maps.into_iter().cloned().collect()),
            });
        }
        Ok(result)
    }

    pub(crate) fn split(mut iterator: &metamodelica::Ref<Iterator>) -> metamodelica::List<metamodelica::Ref<Iterator>> {
        let mut result: metamodelica::List<metamodelica::Ref<Iterator>> = metamodelica::nil();
        let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator>>>;
        (names, ranges, maps) = getFrames(iterator);
        for mut tpl in &*List::zip3(names, ranges, maps) {
            result = metamodelica::cons(fromFrames(list![tpl.clone()]), result);
        }
        result
    }

    pub(crate) fn rename(
        mut iter: metamodelica::Ref<Iterator>,
        mut newBaseName: &ArcStr,
        mut replacements: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    ) -> Result<metamodelica::Ref<Iterator>> {
        let mut iter: metamodelica::Ref<Iterator> = iter;
        iter = (match &*iter {
            SINGLE { name: __iter_name, .. } => {
                let mut replacor: metamodelica::Ref<ComponentRef::NFComponentRef>;
                replacor = ComponentRef::rename(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*newBaseName);
                        __mm_s.push_str(&*intString(1));
                        ArcStr::from(__mm_s)
                    },
                    __iter_name.clone(),
                )?;
                UnorderedMap::add(
                    __iter_name.clone(),
                    Expression::fromCref(replacor.clone(), false)?,
                    replacements,
                )?;
                assign_variant_field!(iter => Iterator::SINGLE; name = replacor);
                iter
            }
            NESTED { .. } => {
                let mut replacor: metamodelica::Ref<ComponentRef::NFComponentRef>;
                for mut i in 1..=metamodelica::arrayLength(var_field!((*iter).names, Iterator::NESTED).clone()) {
                    replacor = ComponentRef::rename(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*newBaseName);
                            __mm_s.push_str(&*intString(i));
                            ArcStr::from(__mm_s)
                        },
                        ({
                            let __elt = (*metamodelica::index_checked(
                                &var_field!((*iter).names, Iterator::NESTED).borrow(),
                                i,
                            )?)
                            .clone();
                            __elt
                        }),
                    )?;
                    UnorderedMap::add(
                        ({
                            let __elt = (*metamodelica::index_checked(
                                &var_field!((*iter).names, Iterator::NESTED).borrow(),
                                i,
                            )?)
                            .clone();
                            __elt
                        }),
                        Expression::fromCref(replacor.clone(), false)?,
                        replacements.clone(),
                    )?;
                    {
                        let __cell0 = replacor;
                        let __idx0 = i;
                        *metamodelica::index_mut_checked(
                            &mut var_field!((*iter).names, Iterator::NESTED).clone().borrow_mut(),
                            __idx0,
                        )? = __cell0;
                    }
                }
                iter
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Iterator.rename"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(iter)
    }

    pub(crate) fn isEqual(
        mut iter1: &metamodelica::Ref<Iterator>,
        mut iter2: &metamodelica::Ref<Iterator>,
    ) -> Result<bool> {
        let mut b: bool = true;
        b = (::match_deref::match_deref! { match (iter1, iter2) {
            (Deref @ EMPTY { .. }, Deref @ EMPTY { .. }) => true,
            (Deref @ SINGLE { .. }, Deref @ SINGLE { .. }) => Expression::isEqual(var_field!((**iter1).range, Iterator::SINGLE).clone(), var_field!((**iter2).range, Iterator::SINGLE).clone())? && Util::optionEqual(var_field!((**iter1).map, Iterator::SINGLE).clone(), var_field!((**iter2).map, Iterator::SINGLE).clone(), &move |__a0: metamodelica::Ref<Iterator>, __a1: metamodelica::Ref<Iterator>| isEqual(&__a0, &__a1))?,
            (Deref @ NESTED { .. }, Deref @ NESTED { .. }) => {
                if metamodelica::arrayLength(var_field!((**iter1).ranges, Iterator::NESTED).clone()) == metamodelica::arrayLength(var_field!((**iter2).ranges, Iterator::NESTED).clone()) && metamodelica::arrayLength(var_field!((**iter1).maps, Iterator::NESTED).clone()) == metamodelica::arrayLength(var_field!((**iter2).maps, Iterator::NESTED).clone()) {
                    for mut i in 1..=metamodelica::arrayLength(var_field!((**iter1).ranges, Iterator::NESTED).clone()) {
                        b = Expression::isEqual(({let __elt = (*metamodelica::index_checked(&var_field!((**iter1).ranges, Iterator::NESTED).borrow(), i)?).clone(); __elt}), ({let __elt = (*metamodelica::index_checked(&var_field!((**iter2).ranges, Iterator::NESTED).borrow(), i)?).clone(); __elt}))?;
                        if !(b) {
                            break;
                        }
                    }
                    for mut i in 1..=metamodelica::arrayLength(var_field!((**iter1).maps, Iterator::NESTED).clone()) {
                        b = Util::optionEqual(({let __elt = (*metamodelica::index_checked(&var_field!((**iter1).maps, Iterator::NESTED).borrow(), i)?).clone(); __elt}), ({let __elt = (*metamodelica::index_checked(&var_field!((**iter2).maps, Iterator::NESTED).borrow(), i)?).clone(); __elt}), &move |__a0: metamodelica::Ref<Iterator>, __a1: metamodelica::Ref<Iterator>| isEqual(&__a0, &__a1))?;
                        if !(b) {
                            break;
                        }
                    }
                } else {
                    b = false;
                }
                b
            },
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(b)
    }

    pub(crate) fn isEmpty(mut iter: &metamodelica::Ref<Iterator>) -> bool {
        let mut b: bool;
        b = (match &**iter {
            EMPTY { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn isResizable(mut iter: &metamodelica::Ref<Iterator>) -> Result<bool> {
        let mut b: bool;
        b = List::any(&(types(iter)?), &Type::isResizable)?;
        Ok(b)
    }

    pub(crate) fn intersect(
        mut iter1: metamodelica::Ref<Iterator>,
        mut iter2: metamodelica::Ref<Iterator>,
    ) -> Result<(
        metamodelica::Ref<Iterator>,
        (metamodelica::Ref<Iterator>, metamodelica::Ref<Iterator>),
        (metamodelica::Ref<Iterator>, metamodelica::Ref<Iterator>),
    )> {
        let mut intersection: metamodelica::Ref<Iterator>;
        let mut rest1: (metamodelica::Ref<Iterator>, metamodelica::Ref<Iterator>);
        let mut rest2: (metamodelica::Ref<Iterator>, metamodelica::Ref<Iterator>);
        (intersection, rest1, rest2) = (::match_deref::match_deref! { match &((iter1.clone(), iter2.clone())) {
            (Deref @ SINGLE { range: Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: start1 }, step: Some(Deref @ Expression::INTEGER { value: step1 }), stop: Deref @ Expression::INTEGER { value: stop1 }, .. }, .. }, Deref @ SINGLE { range: Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: start2 }, step: Some(Deref @ Expression::INTEGER { value: step2 }), stop: Deref @ Expression::INTEGER { value: stop2 }, .. }, .. }) if (step1.clone() == step2.clone() && intMod(start1.clone(), step1.clone()) == intMod(start2.clone(), step2.clone())) => {
                let mut start_max: i32;
                let mut stop_min: i32;
                intMin(start1.clone(), start2.clone());
                start_max = intMax(start1.clone(), start2.clone());
                stop_min = intMin(stop1.clone(), stop2.clone());
                intMax(stop1.clone(), stop2.clone());
                if start_max >= stop_min {
                    intersection = crate::NBEquation::Iterator::interned_EMPTY();
                } else {
                    intersection = metamodelica::Ref::new(Iterator::SINGLE { name: var_field!((*iter1).name, Iterator::SINGLE).clone(), range: metamodelica::Ref::new(Expression::NFExpression::RANGE { ty: Expression::typeOf(var_field!((*iter1).range, Iterator::SINGLE).clone()), start: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: start_max }), step: Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: step1.clone() })), stop: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: stop_min }) }), map: var_field!((*iter1).map, Iterator::SINGLE).clone() });
                }
                rest1 = intersectRest(var_field!((*iter1).name, Iterator::SINGLE).clone(), start1.clone(), step1.clone(), stop1.clone(), start_max - step1.clone(), stop_min + step1.clone(), var_field!((*iter1).map, Iterator::SINGLE).clone())?;
                rest2 = intersectRest(var_field!((*iter2).name, Iterator::SINGLE).clone(), start2.clone(), step2.clone(), stop2.clone(), start_max - step2.clone(), stop_min + step2.clone(), var_field!((*iter2).map, Iterator::SINGLE).clone())?;
                (intersection, rest1, rest2)
            },
            _ => {
                (crate::NBEquation::Iterator::interned_EMPTY(), (iter1, crate::NBEquation::Iterator::interned_EMPTY()), (crate::NBEquation::Iterator::interned_EMPTY(), iter2))
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((intersection, rest1, rest2))
    }

    pub(crate) fn intersectRest(
        mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut start: i32,
        mut step: i32,
        mut stop: i32,
        mut start_max: i32,
        mut stop_min: i32,
        mut map: Option<metamodelica::Ref<Iterator>>,
    ) -> Result<(metamodelica::Ref<Iterator>, metamodelica::Ref<Iterator>)> {
        let mut rest: (metamodelica::Ref<Iterator>, metamodelica::Ref<Iterator>);
        let mut rest_left: metamodelica::Ref<Iterator>;
        let mut rest_right: metamodelica::Ref<Iterator>;
        if start > start_max {
            rest_left = crate::NBEquation::Iterator::interned_EMPTY();
        } else {
            rest_left = metamodelica::Ref::new(Iterator::SINGLE {
                name: name.clone(),
                range: Expression::makeRange(
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: start }),
                    Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                        value: step,
                    })),
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: start_max }),
                )?,
                map: map.clone(),
            });
        }
        if stop_min > stop {
            rest_right = crate::NBEquation::Iterator::interned_EMPTY();
        } else {
            rest_right = metamodelica::Ref::new(Iterator::SINGLE {
                name: name,
                range: Expression::makeRange(
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: stop_min }),
                    Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                        value: step,
                    })),
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: stop }),
                )?,
                map: map,
            });
        }
        rest = (rest_left, rest_right);
        Ok(rest)
    }

    pub(crate) fn types(
        mut iter: &metamodelica::Ref<Iterator>,
    ) -> Result<metamodelica::List<metamodelica::Ref<Type::NFType>>> {
        let mut t: metamodelica::List<metamodelica::Ref<Type::NFType>>;
        t = (match &**iter {
            SINGLE {
                range: __iter_range, ..
            } => list![Expression::typeOf(__iter_range.clone())],
            NESTED { .. } => {
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
                    for mut i in (1..=metamodelica::arrayLength(var_field!((**iter).ranges, Iterator::NESTED).clone()))
                        .into_iter()
                    {
                        let __x = Expression::typeOf(
                            ({
                                let __elt = (*metamodelica::index_checked(
                                    &var_field!((**iter).ranges, Iterator::NESTED).borrow(),
                                    i.clone(),
                                )?)
                                .clone();
                                __elt
                            }),
                        );
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            }
            EMPTY { .. } => metamodelica::nil(),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Iterator.types"));
                        __mm_s.push_str(&*literal!(" could not get types for: "));
                        __mm_s.push_str(&*toString(iter)?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(t)
    }

    pub(crate) fn sizes(mut iter: &metamodelica::Ref<Iterator>, mut resize: bool) -> Result<metamodelica::List<i32>> {
        let mut sizes: metamodelica::List<i32>;
        sizes = (match &**iter {
            SINGLE {
                range: __iter_range, ..
            } => list![Expression::rangeSize(__iter_range.clone(), resize)?],
            NESTED { .. } => {
                ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut i in (1..=metamodelica::arrayLength(var_field!((**iter).ranges, Iterator::NESTED).clone()))
                        .into_iter()
                    {
                        let __x = Expression::rangeSize(
                            ({
                                let __elt = (*metamodelica::index_checked(
                                    &var_field!((**iter).ranges, Iterator::NESTED).borrow(),
                                    i.clone(),
                                )?)
                                .clone();
                                __elt
                            }),
                            resize,
                        )?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            }
            EMPTY { .. } => metamodelica::nil(),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Iterator.sizes"));
                        __mm_s.push_str(&*literal!(" could not get sizes for: "));
                        __mm_s.push_str(&*toString(iter)?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(sizes)
    }

    pub(crate) fn size(mut iter: &metamodelica::Ref<Iterator>, mut resize: bool) -> Result<i32> {
        let mut size: i32 = ({
            let mut __acc: i32 = 1;
            for mut i in (metamodelica::cons(1, sizes(iter, resize)?)).into_iter().cloned() {
                let __x = i.clone();
                __acc *= __x;
            }
            __acc
        });
        Ok(size)
    }

    pub(crate) fn dimensions(
        mut iter: &metamodelica::Ref<Iterator>,
    ) -> Result<metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>> {
        let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = List::flatten(
            ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>> =
                    metamodelica::nil();
                for mut t in (types(iter)?).into_iter().cloned() {
                    let __x = Type::arrayDims(t.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?;
        Ok(dims)
    }

    pub(crate) fn numDimensions(mut iter: &metamodelica::Ref<Iterator>) -> i32 {
        let mut num: i32;
        num = (match &**iter {
            SINGLE { .. } => 1,
            NESTED { .. } => metamodelica::arrayLength(var_field!((**iter).names, Iterator::NESTED).clone()),
            _ => 0,
        });
        num
    }

    pub(crate) fn dummy(mut iter: metamodelica::Ref<Iterator>) -> Result<metamodelica::Ref<Iterator>> {
        fn dummyRange(
            mut exp: metamodelica::Ref<Expression::NFExpression>,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
            let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
            exp = (match &*exp {
                Expression::RANGE { start: __exp_start, .. } => {
                    Expression::makeRange(__exp_start.clone(), None, __exp_start.clone())?
                }
                Expression::ARRAY { .. } => {
                    if (metamodelica::arrayLength(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone())
                        > 0)
                    {
                        Expression::makeArray(
                            metamodelica::Ref::new(Type::NFType::ARRAY {
                                elementType: openmodelica_nf_frontend::NFType::interned_INTEGER(),
                                dimensions: list![Dimension::fromInteger(1, Variability::CONSTANT.clone())],
                            }),
                            arrayCreate(
                                1,
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((*exp).elements, Expression::NFExpression::ARRAY).borrow(),
                                        1,
                                    )?)
                                    .clone();
                                    __elt
                                }),
                            ),
                            Expression::isLiteral(
                                &({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((*exp).elements, Expression::NFExpression::ARRAY).borrow(),
                                        1,
                                    )?)
                                    .clone();
                                    __elt
                                }),
                            )?,
                        )
                    } else {
                        exp
                    }
                }
                _ => exp,
            });
            Ok(exp)
        }

        let mut iter: metamodelica::Ref<Iterator> = iter;
        let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator>>>;
        (names, ranges, maps) = getFrames(&iter);
        ranges = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut e in (ranges).into_iter().cloned() {
                let __x = dummyRange(e.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        iter = fromFrames(List::zip3(names, ranges, maps));
        Ok(iter)
    }

    pub(crate) fn createLocationReplacements(
        mut iter: &metamodelica::Ref<Iterator>,
        mut location: metamodelica::Array<i32>,
        mut replacements: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    ) -> Result<()> {
        let () = (match &**iter {
            SINGLE {
                map: __iter_map,
                name: __iter_name,
                range: __iter_range,
            } if (metamodelica::arrayLength(location.clone()) == 1) => {
                let mut start: i32;
                let mut step: i32;
                (start, step, _) = Expression::getIntegerRange(__iter_range.clone(), true)?;
                UnorderedMap::add(
                    __iter_name.clone(),
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                        value: start
                            + ({
                                let __elt = (*metamodelica::index_checked(&location.borrow(), 1)?).clone();
                                __elt
                            }) * step,
                    }),
                    replacements.clone(),
                )?;
                createMappedLocationReplacement(
                    __iter_map.clone(),
                    ({
                        let __elt = (*metamodelica::index_checked(&location.borrow(), 1)?).clone();
                        __elt
                    }) + 1,
                    replacements,
                )?;
                ()
            }
            NESTED { .. }
                if (metamodelica::arrayLength(location.clone())
                    == metamodelica::arrayLength(var_field!((**iter).ranges, Iterator::NESTED).clone())) =>
            {
                let mut start: i32;
                let mut step: i32;
                for mut i in 1..=metamodelica::arrayLength(location.clone()) {
                    (start, step, _) = Expression::getIntegerRange(
                        ({
                            let __elt = (*metamodelica::index_checked(
                                &var_field!((**iter).ranges, Iterator::NESTED).borrow(),
                                i,
                            )?)
                            .clone();
                            __elt
                        }),
                        true,
                    )?;
                    UnorderedMap::add(
                        ({
                            let __elt = (*metamodelica::index_checked(
                                &var_field!((**iter).names, Iterator::NESTED).borrow(),
                                i,
                            )?)
                            .clone();
                            __elt
                        }),
                        metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                            value: start
                                + ({
                                    let __elt = (*metamodelica::index_checked(&location.borrow(), i)?).clone();
                                    __elt
                                }) * step,
                        }),
                        replacements.clone(),
                    )?;
                    createMappedLocationReplacement(
                        ({
                            let __elt = (*metamodelica::index_checked(
                                &var_field!((**iter).maps, Iterator::NESTED).borrow(),
                                i,
                            )?)
                            .clone();
                            __elt
                        }),
                        ({
                            let __elt = (*metamodelica::index_checked(&location.borrow(), i)?).clone();
                            __elt
                        }) + 1,
                        replacements.clone(),
                    )?;
                }
                ()
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Iterator.createLocationReplacements"));
                        __mm_s.push_str(&*literal!(" could not create replacements for location: "));
                        __mm_s.push_str(&*Array::toString(
                            location.clone(),
                            &fnptr!(intString, i32),
                            literal!(""),
                            literal!("["),
                            literal!(", "),
                            literal!("]"),
                            true,
                            0,
                        )?);
                        __mm_s.push_str(&*literal!(" and iterator: "));
                        __mm_s.push_str(&*toString(iter)?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(())
    }

    pub(crate) fn createMappedLocationReplacement(
        mut map: Option<metamodelica::Ref<Iterator>>,
        mut location: i32,
        mut replacements: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    ) -> Result<()> {
        let () = (::match_deref::match_deref! { match &(map) {
            Some(Deref @ SINGLE { name, range: arr @ Deref @ Expression::ARRAY { .. }, .. }) => {
                UnorderedMap::add(name.clone(), ({let __elt = (*metamodelica::index_checked(&var_field!((**arr).elements, Expression::NFExpression::ARRAY).borrow(), location)?).clone(); __elt}), replacements)?;
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(())
    }

    pub(crate) fn createReplacement(
        mut replacor: &metamodelica::Ref<Iterator>,
        mut replacee: &metamodelica::Ref<Iterator>,
        mut replacements: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    ) -> Result<()> {
        let mut failed: bool = false;
        failed = (::match_deref::match_deref! { match (replacor, replacee) {
            (Deref @ SINGLE { .. }, Deref @ SINGLE { .. }) => {
                failed = createSingleReplacement(var_field!((**replacor).name, Iterator::SINGLE).clone(), var_field!((**replacor).range, Iterator::SINGLE).clone(), var_field!((**replacee).name, Iterator::SINGLE).clone(), var_field!((**replacee).range, Iterator::SINGLE).clone(), replacements)?;
                failed
            },
            (Deref @ NESTED { .. }, Deref @ NESTED { .. }) => {
                if metamodelica::arrayLength(var_field!((**replacor).names, Iterator::NESTED).clone()) == metamodelica::arrayLength(var_field!((**replacee).names, Iterator::NESTED).clone()) {
                    for mut i in 1..=metamodelica::arrayLength(var_field!((**replacor).names, Iterator::NESTED).clone()) {
                        failed = createSingleReplacement(({let __elt = (*metamodelica::index_checked(&var_field!((**replacor).names, Iterator::NESTED).borrow(), i)?).clone(); __elt}), ({let __elt = (*metamodelica::index_checked(&var_field!((**replacor).ranges, Iterator::NESTED).borrow(), i)?).clone(); __elt}), ({let __elt = (*metamodelica::index_checked(&var_field!((**replacee).names, Iterator::NESTED).borrow(), i)?).clone(); __elt}), ({let __elt = (*metamodelica::index_checked(&var_field!((**replacee).ranges, Iterator::NESTED).borrow(), i)?).clone(); __elt}), replacements.clone())?;
                        if failed {
                            break;
                        }
                    }
                } else {
                    failed = true;
                }
                failed
            },
            _ => true,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if failed {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBEquation.Iterator.createReplacement"));
                    __mm_s.push_str(&*literal!(" could not create replacements for replacor: "));
                    __mm_s.push_str(&*toString(replacor)?);
                    __mm_s.push_str(&*literal!(" and replacee: "));
                    __mm_s.push_str(&*toString(replacee)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        Ok(())
    }

    pub(crate) fn createSingleReplacement(
        mut replacor_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut replacor_range: metamodelica::Ref<Expression::NFExpression>,
        mut replacee_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut replacee_range: metamodelica::Ref<Expression::NFExpression>,
        mut replacements: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    ) -> Result<bool> {
        fn rangeLength(mut start: i32, mut step: i32, mut stop: i32) -> Result<i32> {
            let mut length: i32 = ((metamodelica::real_div_checked(
                (metamodelica::OrderedFloat((stop - start + Util::intSign(step)) as f64)),
                metamodelica::OrderedFloat((step) as f64),
            )?)
            .0
            .floor() as i32);
            Ok(length)
        }

        let mut failed: bool = false;
        let mut or_start: i32;
        let mut or_step: i32;
        let mut or_stop: i32;
        let mut ee_start: i32;
        let mut ee_step: i32;
        let mut ee_stop: i32;
        let mut exp: metamodelica::Ref<Expression::NFExpression>;
        (or_start, or_step, or_stop) = Expression::getIntegerRange(replacor_range, true)?;
        (ee_start, ee_step, ee_stop) = Expression::getIntegerRange(replacee_range, true)?;
        if rangeLength(or_start, or_step, or_stop)? == rangeLength(ee_start, ee_step, ee_stop)? {
            exp = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                arguments: list![
                    metamodelica::Ref::new(Expression::NFExpression::REAL {
                        value: intReal(ee_start)
                    }),
                    metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![
                            metamodelica::Ref::new(Expression::NFExpression::REAL {
                                value: metamodelica::real_div_checked(intReal(ee_step), intReal(or_step))?
                            }),
                            metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                                arguments: list![Expression::fromCref(replacor_cref, false)?],
                                inv_arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL {
                                    value: intReal(or_start)
                                })],
                                operator: Operator::makeAdd(openmodelica_nf_frontend::NFType::interned_REAL())
                            })
                        ],
                        inv_arguments: metamodelica::nil(),
                        operator: Operator::makeMul(openmodelica_nf_frontend::NFType::interned_REAL())
                    })
                ],
                inv_arguments: metamodelica::nil(),
                operator: Operator::makeAdd(openmodelica_nf_frontend::NFType::interned_REAL()),
            });
            UnorderedMap::add(replacee_cref, exp, replacements)?;
        } else {
            failed = true;
        }
        Ok(failed)
    }

    pub(crate) fn expand(
        mut iter: metamodelica::Ref<Iterator>,
        mut call: &metamodelica::Ref<Call::NFCall>,
    ) -> Result<metamodelica::Ref<Iterator>> {
        let mut iter: metamodelica::Ref<Iterator> = iter;
        let mut new_iters: metamodelica::Ref<
            UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        > = UnorderedSet::new(
            (std::sync::Arc::new(BVariable::hash)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32>
                        + 'static,
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
        iter = (match &**call {
            Call::TYPED_ARRAY_CONSTRUCTOR {
                iters: __call_iters, ..
            } => {
                let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator>>>;
                (names, ranges, maps) = getFrames(&iter);
                fromFrames(listAppend(
                    ({
                        let mut __acc: metamodelica::List<(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            metamodelica::Ref<Expression::NFExpression>,
                            Option<metamodelica::Ref<Iterator>>,
                        )> = metamodelica::nil();
                        for mut tpl in (__call_iters.clone()).into_iter().cloned() {
                            let __x = createFrame(tpl.clone(), new_iters.clone())?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    List::zip3(names, ranges, maps),
                ))
            }
            Call::TYPED_REDUCTION {
                iters: __call_iters, ..
            } => {
                let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator>>>;
                (names, ranges, maps) = getFrames(&iter);
                fromFrames(listAppend(
                    ({
                        let mut __acc: metamodelica::List<(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            metamodelica::Ref<Expression::NFExpression>,
                            Option<metamodelica::Ref<Iterator>>,
                        )> = metamodelica::nil();
                        for mut tpl in (__call_iters.clone()).into_iter().cloned() {
                            let __x = createFrame(tpl.clone(), new_iters.clone())?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    List::zip3(names, ranges, maps),
                ))
            }
            _ => iter,
        });
        Ok(iter)
    }

    pub(crate) fn extract(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut new_iters: metamodelica::Ref<
            UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        >,
        mut dims_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >,
        >,
    ) -> Result<(metamodelica::Ref<Iterator>, metamodelica::Ref<Expression::NFExpression>)> {
        let mut iter: metamodelica::Ref<Iterator>;
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
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
        (exp, iter) = extractFromCall(
            exp,
            crate::NBEquation::Iterator::interned_EMPTY(),
            replacements.clone(),
            new_iters,
            dims_map,
        )?;
        exp = Expression::map(
            exp,
            (std::sync::Arc::new({
                let __pe_b1 = replacements;
                move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
        (exp, _, _, _) = Typing::typeExp(
            exp,
            NFInstContext::RHS.clone(),
            &(metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo")),
            true,
        )?;
        Ok((iter, exp))
    }

    pub(crate) fn extractFromCall(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut iter: metamodelica::Ref<Iterator>,
        mut replacements: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
        mut new_iters: metamodelica::Ref<
            UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        >,
        mut dims_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >,
        >,
    ) -> Result<(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Iterator>)> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let mut iter: metamodelica::Ref<Iterator> = iter;
        (exp, iter) = ({
            let mut frames: metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
                Option<metamodelica::Ref<Iterator>>,
            )> = metamodelica::nil();
            (::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
                    let mut tmp: metamodelica::Ref<Iterator>;
                    let mut tmp_inner: metamodelica::Ref<Iterator>;
                    let mut full_dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
                    for mut tpl in &*var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone().reverse() {
                        frames = metamodelica::cons(createFrame(tpl.clone(), new_iters.clone())?, frames);
                    }
                    tmp = fromFrames(frames.clone());
                    if !(isEmpty(&iter)) {
                        createReplacement(&iter, &tmp, replacements.clone())?;
                    } else {
                        iter = tmp;
                    }
                    for mut frame in &*frames {
                        let _ = (::match_deref::match_deref! { match &(Util::tuple33(frame.clone())) {
                Some(__esc_tmp_inner @ Deref @ SINGLE { .. }) => {
                    tmp_inner = (*__esc_tmp_inner).clone();
                    UnorderedMap::add(var_field!((*tmp_inner).name, Iterator::SINGLE).clone(), Expression::applySubscripts(&(list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: Expression::fromCref(Util::tuple31(frame.clone()), false)? })]), var_field!((*tmp_inner).range, Iterator::SINGLE).clone(), false)?, replacements.clone())?;
                    ()
                },
                _ => (),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                    }
                    full_dims = Type::arrayDims(Expression::typeOf(exp.clone()));
                    full_dims = List::firstN(full_dims.clone(), ((full_dims).len() as i32) - Type::dimensionCount(Expression::typeOf(var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone())))?;
                    UnorderedMap::tryAdd(full_dims, ({
                let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
                for mut f in (frames).into_iter().cloned() {
                    let __x = Util::tuple31(f.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), dims_map)?;
                    (var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), iter)
                },
                Deref @ Expression::CALL { .. } => {
                    (exp.clone(), iter)
                },
                Deref @ Expression::IF { .. } if (extractFromCallIfException(&exp)) => {
                    (exp.clone(), iter)
                },
                _ => {
                    (exp, iter) = Expression::mapFoldShallow(exp, (std::sync::Arc::new({ let __pe_b2 = replacements; let __pe_b3 = new_iters; let __pe_b4 = dims_map; move |__pe_a0, __pe_a1| extractFromCall(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Iterator>) -> Result<(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Iterator>)> + 'static>), iter)?;
                    (exp.clone(), iter)
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        });
        Ok((exp, iter))
    }

    pub(crate) fn extractFromCallIfException(mut exp: &metamodelica::Ref<Expression::NFExpression>) -> bool {
        let mut b: bool;
        b = (::match_deref::match_deref! { match exp {
            Deref @ Expression::CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => false,
            Deref @ Expression::IF { falseBranch: __exp_falseBranch, trueBranch: __exp_trueBranch, .. } => extractFromCallIfException(metamodelica::AsArg::as_arg(&__exp_trueBranch)) || extractFromCallIfException(metamodelica::AsArg::as_arg(&__exp_falseBranch)),
            _ => true,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        b
    }

    pub(crate) fn normalizedSubscripts(
        mut iter: &metamodelica::Ref<Iterator>,
        mut iter_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Subscript::NFSubscript>,
            >,
        >,
    ) -> Result<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>> {
        let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
        let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        (names, ranges, _) = getFrames(iter);
        subs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
            let __thr_src0 = names;
            let mut __thr_it0 = (&__thr_src0).into_iter();
            let __thr_src1 = ranges;
            let mut __thr_it1 = (&__thr_src1).into_iter();
            loop {
                match (__thr_it0.next(), __thr_it1.next()) {
                    (Some(name), Some(range)) => {
                        let __x = normalizedSubscript(name.clone(), range.clone(), iter_map.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    (None, None) => break,
                    _ => return Err("threaded for: ranges of unequal length"),
                }
            }
            __acc.reverse()
        });
        Ok(subs)
    }

    pub(crate) fn normalizedSubscript(
        mut iter_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut range: metamodelica::Ref<Expression::NFExpression>,
        mut iter_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Subscript::NFSubscript>,
            >,
        >,
    ) -> Result<metamodelica::Ref<Subscript::NFSubscript>> {
        let mut sub: metamodelica::Ref<Subscript::NFSubscript>;
        let mut step: metamodelica::Ref<Expression::NFExpression>;
        let mut sub_exp: metamodelica::Ref<Expression::NFExpression>;
        sub = (match &*range {
            Expression::RANGE {
                start: __range_start,
                step: __range_step,
                ..
            } => {
                step = Util::getOptionOrDefault(
                    __range_step.clone(),
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
                );
                sub_exp = Expression::fromCref(iter_name.clone(), false)?;
                if !(Expression::isOne(metamodelica::AsArg::as_arg(&__range_start))?) {
                    sub_exp = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![sub_exp],
                        inv_arguments: list![__range_start.clone()],
                        operator: Operator::makeAdd(openmodelica_nf_frontend::NFType::interned_INTEGER()),
                    });
                }
                if !(Expression::isOne(&step)?) {
                    sub_exp = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![sub_exp],
                        inv_arguments: list![step],
                        operator: Operator::makeMul(openmodelica_nf_frontend::NFType::interned_REAL()),
                    });
                }
                if !(Expression::isOne(metamodelica::AsArg::as_arg(&__range_start))?) {
                    sub_exp = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![
                            sub_exp.clone(),
                            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 })
                        ],
                        inv_arguments: metamodelica::nil(),
                        operator: Operator::makeAdd(Expression::typeOf(sub_exp)),
                    });
                }
                sub_exp = SimplifyExp::simplifyDump(
                    sub_exp,
                    true,
                    &(literal!("NBEquation.Iterator.normalizedSubscript")),
                    &(literal!("")),
                )?;
                if !(Type::isInteger(&(Expression::typeOf(sub_exp.clone())))?) {
                    sub_exp = metamodelica::Ref::new(Expression::NFExpression::CALL {
                        call: Call::makeTypedCall(
                            NFBuiltinFuncs::INTEGER_REAL().clone(),
                            list![sub_exp],
                            Variability::DISCRETE.clone(),
                            Purity::PURE.clone(),
                            NFBuiltinFuncs::INTEGER_REAL().returnType.clone(),
                        ),
                    });
                }
                metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: sub_exp })
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Iterator.normalizedSubscript"));
                        __mm_s.push_str(&*literal!(" failed because range is no range: "));
                        __mm_s.push_str(&*Expression::toString(range)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        UnorderedMap::add(iter_name, sub.clone(), iter_map)?;
        Ok(sub)
    }

    pub(crate) fn simplifyRangeCondition(
        mut iter: metamodelica::Ref<Iterator>,
        mut condition: metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<(metamodelica::Ref<Iterator>, Solve::Status)> {
        pub(crate) type IterOpt = Option<metamodelica::Ref<Iterator>>;

        let mut iter: metamodelica::Ref<Iterator> = iter;
        let mut status: Solve::Status = Solve::Status::UNSOLVABLE.clone();
        let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator>>>;
        let mut iter_map: metamodelica::Ref<
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
        let mut opt_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                Option<metamodelica::Ref<Iterator>>,
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
        (iter, status) = (match &*condition {
            Expression::RELATION {
                exp1: __condition_exp1,
                exp2: __condition_exp2,
                operator: __condition_operator,
                ..
            } => {
                let mut tmpEqn: metamodelica::Ref<Equation::Equation>;
                let mut occs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut invert: Solve::RelationInversion;
                let mut range: metamodelica::Ref<Expression::NFExpression>;
                let mut operator: metamodelica::Ref<Operator::NFOperator>;
                (names, ranges, maps) = getFrames(&iter);
                for mut frame in &*List::zip3(names.clone(), ranges, maps) {
                    UnorderedMap::add(
                        Util::tuple31(frame.clone()),
                        Util::tuple32(frame.clone()),
                        iter_map.clone(),
                    )?;
                    UnorderedMap::add(
                        Util::tuple31(frame.clone()),
                        Util::tuple33(frame.clone()),
                        opt_map.clone(),
                    )?;
                }
                tmpEqn = Pointer::access(Equation::makeAssignment(
                    __condition_exp1.clone(),
                    __condition_exp2.clone(),
                    Pointer::create(0),
                    &(arcstr::literal!(BVariable::TEMPORARY_STR)),
                    crate::NBEquation::Iterator::interned_EMPTY(),
                    default(EquationKind::UNKNOWN.clone(), false, None, None),
                )?);
                occs = Equation::collectCrefs(
                    tmpEqn.clone(),
                    (std::sync::Arc::new({
                        let __pe_b2 = iter_map.clone();
                        move |__pe_a0, __pe_a1| Equation::collectFromMap(__pe_a0, __pe_a1, __pe_b2.clone())
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
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?;
                if List::hasOneElement(&occs) {
                    cref = (occs).head().cloned()?;
                    (tmpEqn, status, invert) = Solve::solveBody(
                        tmpEqn,
                        cref.clone(),
                        UnorderedMap::new(
                            (std::sync::Arc::new(move |__a0: metamodelica::Ref<Path>| -> metamodelica::Result<_> {
                                ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
                            })
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<Path>) -> Result<i32> + 'static,
                                >),
                            (std::sync::Arc::new(
                                move |__a0: metamodelica::Ref<Path>,
                                      __a1: metamodelica::Ref<Path>|
                                      -> metamodelica::Result<_> {
                                    ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
                                },
                            )
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<Path>, metamodelica::Ref<Path>) -> Result<bool>
                                        + 'static,
                                >),
                            1,
                        ),
                    )?;
                    operator = if (invert == Solve::RelationInversion::TRUE.clone()) {
                        Operator::invert(__condition_operator.clone())?
                    } else {
                        __condition_operator.clone()
                    };
                    if status == Solve::Status::EXPLICIT.clone() && invert != Solve::RelationInversion::UNKNOWN.clone()
                    {
                        range = UnorderedMap::getSafe(
                            cref.clone(),
                            iter_map.clone(),
                            metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo"),
                        )?;
                        match '__try0: {
                            (range, status) = (match &*range {
                                Expression::RANGE { .. } => (
                                    unwrap_break_err!(adaptRange(unwrap_break_err!(UnorderedMap::getSafe(cref.clone(), iter_map.clone(), metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo")), '__try0), &(unwrap_break_err!(Util::getOption(unwrap_break_err!(Equation::getRHS(tmpEqn.clone()), '__try0)), '__try0)), &operator), '__try0),
                                    status,
                                ),
                                Expression::ARRAY { .. } => (
                                    unwrap_break_err!(adaptArray(unwrap_break_err!(UnorderedMap::getSafe(cref.clone(), iter_map.clone(), metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo")), '__try0), &(unwrap_break_err!(Util::getOption(unwrap_break_err!(Equation::getRHS(tmpEqn.clone()), '__try0)), '__try0)), &operator), '__try0),
                                    status,
                                ),
                                _ => (range.clone(), Solve::Status::UNSOLVABLE.clone()),
                            });
                            Ok::<_, &'static str>((range.clone(), status.clone()))
                        } {
                            Ok((__try0_o0, __try0_o1)) => {
                                range = __try0_o0;
                                status = __try0_o1;
                            }
                            Err(__try0_err) => {
                                Error::addMessage(
                                    Error::INTERNAL_ERROR.clone(),
                                    list![{
                                        let mut __mm_s = String::new();
                                        __mm_s.push_str(&*literal!("NBEquation.Iterator.simplifyRangeCondition"));
                                        __mm_s.push_str(&*literal!(" failed to combine iterator: "));
                                        __mm_s.push_str(&*toString(&iter)?);
                                        __mm_s.push_str(&*literal!(" with condition "));
                                        __mm_s.push_str(&*Expression::toString(condition.clone())?);
                                        __mm_s.push_str(&*literal!("."));
                                        ArcStr::from(__mm_s)
                                    }],
                                )?;
                                return Err(__try0_err);
                            }
                        }
                        UnorderedMap::add(cref, range, iter_map.clone())?;
                    } else {
                        status = Solve::Status::UNSOLVABLE.clone();
                    }
                }
                if status == Solve::Status::EXPLICIT.clone() {
                    iter = fromFrames(
                        ({
                            let mut __acc: metamodelica::List<(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<Expression::NFExpression>,
                                Option<metamodelica::Ref<Iterator>>,
                            )> = metamodelica::nil();
                            for mut name in (names).into_iter().cloned() {
                                let __x = (
                                    name.clone(),
                                    UnorderedMap::getSafe(
                                        name.clone(),
                                        iter_map.clone(),
                                        metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo"),
                                    )?,
                                    UnorderedMap::getSafe(
                                        name.clone(),
                                        opt_map.clone(),
                                        metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo"),
                                    )?,
                                );
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                    );
                }
                (iter, status)
            }
            _ => (iter, Solve::Status::UNSOLVABLE.clone()),
        });
        Ok((iter, status))
    }

    pub(crate) fn adaptRange(
        mut range: metamodelica::Ref<Expression::NFExpression>,
        mut rhs: &metamodelica::Ref<Expression::NFExpression>,
        mut operator: &metamodelica::Ref<Operator::NFOperator>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut range: metamodelica::Ref<Expression::NFExpression> = range;
        let mut thresh: i32;
        let mut start: i32;
        let mut step: i32;
        let mut stop: i32;
        let mut within_range: bool;
        (thresh, start, step, stop) = (::match_deref::match_deref! { match &((&**rhs, range.clone())) {
            (Deref @ Expression::INTEGER { value: __esc_thresh }, __esc_range @ Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: __esc_start }, step: Some(Deref @ Expression::INTEGER { value: __esc_step }), stop: Deref @ Expression::INTEGER { value: __esc_stop }, .. }) => {
                range = (*__esc_range).clone();
                thresh = (*__esc_thresh).clone();
                start = (*__esc_start).clone();
                step = (*__esc_step).clone();
                stop = (*__esc_stop).clone();
                (thresh.clone(), start.clone(), step.clone(), stop.clone())
            },
            (Deref @ Expression::INTEGER { value: __esc_thresh }, __esc_range @ Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: __esc_start }, stop: Deref @ Expression::INTEGER { value: __esc_stop }, .. }) => {
                range = (*__esc_range).clone();
                thresh = (*__esc_thresh).clone();
                start = (*__esc_start).clone();
                stop = (*__esc_stop).clone();
                (thresh.clone(), start.clone(), 1, stop.clone())
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.adaptRange")); __mm_s.push_str(&*literal!(" failed because range could not be evaluated: ")); __mm_s.push_str(&*Expression::toString(range.clone())?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        within_range = thresh * sign(metamodelica::OrderedFloat((step) as f64))
            > start * sign(metamodelica::OrderedFloat((step) as f64))
            && thresh * sign(metamodelica::OrderedFloat((step) as f64))
                < stop * sign(metamodelica::OrderedFloat((step) as f64));
        range = (match operator.op.clone() {
            Operator::Op::EQUAL => {
                if (within_range) {
                    Expression::makeRange(
                        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: thresh }),
                        None,
                        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: thresh }),
                    )?
                } else {
                    Expression::makeRange(
                        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                        Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 })),
                        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                    )?
                }
            }
            Operator::Op::NEQUAL => {
                if (within_range) {
                    Expression::makeExpArray(
                        metamodelica::arrayFromVec(
                            ({
                                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                                    metamodelica::nil();
                                for mut i in (List::intRange3(start, step, stop)?).into_iter().cloned() {
                                    if !(i.clone() != thresh) {
                                        continue;
                                    }
                                    let __x =
                                        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i.clone() });
                                    __acc = cons(__x, __acc);
                                }
                                __acc.reverse()
                            })
                            .into_iter()
                            .cloned()
                            .collect(),
                        ),
                        openmodelica_nf_frontend::NFType::interned_INTEGER(),
                        true,
                    )
                } else {
                    range
                }
            }
            Operator::Op::LESS => interceptRange(
                thresh - 1,
                start,
                step,
                stop,
                within_range,
                sign(metamodelica::OrderedFloat((step) as f64)) > 0,
                range,
                &fnptr!(intLe, i32, i32),
            )?,
            Operator::Op::LESSEQ => interceptRange(
                thresh,
                start,
                step,
                stop,
                within_range,
                sign(metamodelica::OrderedFloat((step) as f64)) > 0,
                range,
                &fnptr!(intLt, i32, i32),
            )?,
            Operator::Op::GREATER => interceptRange(
                thresh + 1,
                start,
                step,
                stop,
                within_range,
                sign(metamodelica::OrderedFloat((step) as f64)) < 0,
                range,
                &fnptr!(intGe, i32, i32),
            )?,
            Operator::Op::GREATEREQ => interceptRange(
                thresh,
                start,
                step,
                stop,
                within_range,
                sign(metamodelica::OrderedFloat((step) as f64)) < 0,
                range,
                &fnptr!(intGt, i32, i32),
            )?,
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Iterator.adaptRange"));
                        __mm_s.push_str(&*literal!(" failed for operator: "));
                        __mm_s.push_str(&*Operator::toDebugString(operator)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(range)
    }

    pub(crate) fn interceptRange(
        mut thresh: i32,
        mut start: i32,
        mut step: i32,
        mut stop: i32,
        mut within_range: bool,
        mut at_end: bool,
        mut range: metamodelica::Ref<Expression::NFExpression>,
        mut func: &dyn ::std::ops::Fn(i32, i32) -> Result<bool>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        type intComp = std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>;

        fn lowerBoundary(mut thresh: i32, mut start: i32, mut step: i32) -> i32 {
            let mut boundary: i32 = thresh + intMod(start - thresh, step);
            boundary
        }

        let mut range: metamodelica::Ref<Expression::NFExpression> = range;
        if within_range {
            if at_end {
                range = Expression::makeRange(
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: start }),
                    Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                        value: step,
                    })),
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: thresh }),
                )?;
            } else {
                range = Expression::makeRange(
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                        value: lowerBoundary(thresh, start, step),
                    }),
                    Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                        value: step,
                    })),
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: stop }),
                )?;
            }
        } else if func(if (at_end) { stop } else { start }, thresh)? {
            range = Expression::makeRange(
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 })),
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
            )?;
        }
        Ok(range)
    }

    pub(crate) fn adaptArray(
        mut array: metamodelica::Ref<Expression::NFExpression>,
        mut rhs: &metamodelica::Ref<Expression::NFExpression>,
        mut operator: &metamodelica::Ref<Operator::NFOperator>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut array: metamodelica::Ref<Expression::NFExpression> = array;
        let mut thresh: i32;
        let mut elems: metamodelica::List<i32>;
        (thresh, elems) = (::match_deref::match_deref! { match &((rhs.clone(), array.clone())) {
            (Deref @ Expression::INTEGER { value: __esc_thresh }, Deref @ Expression::ARRAY { literal: true, .. }) => {
                thresh = (*__esc_thresh).clone();
                (thresh.clone(), ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut e in (var_field!((*array).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
                let __x = Expression::integerValue(e.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }))
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Iterator.adaptArray")); __mm_s.push_str(&*literal!(" failed because array range is non literal: ")); __mm_s.push_str(&*Expression::toString(array)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        array = (match operator.op.clone() {
            Operator::Op::EQUAL => {
                if (List::contains(&elems, thresh, &fnptr!(intEq, i32, i32))?) {
                    Expression::makeRange(
                        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: thresh }),
                        None,
                        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: thresh }),
                    )?
                } else {
                    Expression::makeRange(
                        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                        Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 })),
                        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                    )?
                }
            }
            Operator::Op::NEQUAL => Expression::makeExpArray(
                metamodelica::arrayFromVec(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                            metamodelica::nil();
                        for mut i in (elems).into_iter().cloned() {
                            if !(i.clone() != thresh) {
                                continue;
                            }
                            let __x = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i.clone() });
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                    .into_iter()
                    .cloned()
                    .collect(),
                ),
                openmodelica_nf_frontend::NFType::interned_INTEGER(),
                true,
            ),
            Operator::Op::LESS => Expression::makeExpArray(
                metamodelica::arrayFromVec(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                            metamodelica::nil();
                        for mut i in (elems).into_iter().cloned() {
                            if !(i.clone() < thresh) {
                                continue;
                            }
                            let __x = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i.clone() });
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                    .into_iter()
                    .cloned()
                    .collect(),
                ),
                openmodelica_nf_frontend::NFType::interned_INTEGER(),
                true,
            ),
            Operator::Op::LESSEQ => Expression::makeExpArray(
                metamodelica::arrayFromVec(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                            metamodelica::nil();
                        for mut i in (elems).into_iter().cloned() {
                            if !(i.clone() <= thresh) {
                                continue;
                            }
                            let __x = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i.clone() });
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                    .into_iter()
                    .cloned()
                    .collect(),
                ),
                openmodelica_nf_frontend::NFType::interned_INTEGER(),
                true,
            ),
            Operator::Op::GREATER => Expression::makeExpArray(
                metamodelica::arrayFromVec(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                            metamodelica::nil();
                        for mut i in (elems).into_iter().cloned() {
                            if !(i.clone() > thresh) {
                                continue;
                            }
                            let __x = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i.clone() });
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                    .into_iter()
                    .cloned()
                    .collect(),
                ),
                openmodelica_nf_frontend::NFType::interned_INTEGER(),
                true,
            ),
            Operator::Op::GREATEREQ => Expression::makeExpArray(
                metamodelica::arrayFromVec(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                            metamodelica::nil();
                        for mut i in (elems).into_iter().cloned() {
                            if !(i.clone() >= thresh) {
                                continue;
                            }
                            let __x = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i.clone() });
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                    .into_iter()
                    .cloned()
                    .collect(),
                ),
                openmodelica_nf_frontend::NFType::interned_INTEGER(),
                true,
            ),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Iterator.adaptArray"));
                        __mm_s.push_str(&*literal!(" failed for operator: "));
                        __mm_s.push_str(&*Operator::toDebugString(operator)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(array)
    }

    pub(crate) fn applyOrder(
        mut iter: metamodelica::Ref<Iterator>,
        mut order: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, EvalOrder>,
        >,
    ) -> Result<metamodelica::Ref<Iterator>> {
        pub(crate) fn applySingleOrder(
            mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
            mut range: metamodelica::Ref<Expression::NFExpression>,
            mut order: metamodelica::Ref<
                UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, EvalOrder>,
            >,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
            let mut range: metamodelica::Ref<Expression::NFExpression> = range;
            let mut eo: EvalOrder =
                UnorderedMap::getOrDefault(name.clone(), order.clone(), EvalOrder::INDEPENDENT.clone())?;
            let mut step: metamodelica::Ref<Expression::NFExpression>;
            let mut res: metamodelica::Ref<Expression::NFExpression>;
            let mut elements: metamodelica::List<i32>;
            range = (match &*range {
                Expression::RANGE { step: __range_step, .. } => {
                    step = Util::getOptionOrDefault(
                        __range_step.clone(),
                        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
                    );
                    if Expression::isNegative(&step)? && eo == EvalOrder::FORWARD.clone()
                        || Expression::isPositive(&step)? && eo == EvalOrder::BACKWARD.clone()
                    {
                        res = Expression::revertRange(range)?;
                    } else {
                        res = range;
                    }
                    res
                }
                Expression::ARRAY { literal: true, .. } => {
                    if eo == EvalOrder::FORWARD.clone() {
                        elements = ({
                            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                            for mut e in (var_field!((*range).elements, Expression::NFExpression::ARRAY).clone())
                                .borrow()
                                .iter()
                            {
                                let __x = Expression::getInteger(e.clone(), true)?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        });
                        assign_variant_field!(range => Expression::NFExpression::ARRAY; elements = metamodelica::arrayFromVec(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut e in (List::sort(elements, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: e.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect()));
                    } else if eo == EvalOrder::BACKWARD.clone() {
                        elements = ({
                            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                            for mut e in (var_field!((*range).elements, Expression::NFExpression::ARRAY).clone())
                                .borrow()
                                .iter()
                            {
                                let __x = Expression::getInteger(e.clone(), true)?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        });
                        assign_variant_field!(range => Expression::NFExpression::ARRAY; elements = metamodelica::arrayFromVec(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut e in (List::sort(elements, (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: e.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect()));
                    }
                    range
                }
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NBEquation.Iterator.applyOrder.applySingleOrder"));
                            __mm_s.push_str(&*literal!(" failed for unhandled range expression: "));
                            __mm_s.push_str(&*Expression::toString(range)?);
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            });
            Ok(range)
        }

        let mut iter: metamodelica::Ref<Iterator> = iter;
        iter = (match &*iter {
            SINGLE {
                name: __iter_name,
                range: __iter_range,
                ..
            } => {
                assign_variant_field!(iter => Iterator::SINGLE; range = applySingleOrder(__iter_name.clone(), __iter_range.clone(), order)?);
                iter
            }
            NESTED { .. } => {
                for mut i in 1..=metamodelica::arrayLength(var_field!((*iter).names, Iterator::NESTED).clone()) {
                    {
                        let __cell0 = applySingleOrder(
                            ({
                                let __elt = (*metamodelica::index_checked(
                                    &var_field!((*iter).names, Iterator::NESTED).borrow(),
                                    i,
                                )?)
                                .clone();
                                __elt
                            }),
                            ({
                                let __elt = (*metamodelica::index_checked(
                                    &var_field!((*iter).ranges, Iterator::NESTED).borrow(),
                                    i,
                                )?)
                                .clone();
                                __elt
                            }),
                            order.clone(),
                        )?;
                        let __idx0 = i;
                        *metamodelica::index_mut_checked(
                            &mut var_field!((*iter).ranges, Iterator::NESTED).clone().borrow_mut(),
                            __idx0,
                        )? = __cell0;
                    }
                }
                iter
            }
            _ => iter,
        });
        Ok(iter)
    }

    pub(crate) fn toString(mut iter: &metamodelica::Ref<Iterator>) -> Result<ArcStr> {
        fn singleStr(
            mut name: &metamodelica::Ref<ComponentRef::NFComponentRef>,
            mut range: metamodelica::Ref<Expression::NFExpression>,
            mut map: Option<metamodelica::Ref<Iterator>>,
        ) -> Result<ArcStr> {
            let mut r#str: ArcStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentRef::toString(name)?);
                __mm_s.push_str(&*literal!(" in "));
                __mm_s.push_str(&*Expression::toString(range.clone())?);
                ArcStr::from(__mm_s)
            };
            let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            if (map).is_some() {
                (names, _, _) = getFrames(&(Util::getOption(map)?));
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!(" ("));
                    __mm_s.push_str(&*ComponentRef::toString(&((names).head().cloned()?))?);
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                };
            }
            Ok(r#str)
        }

        let mut r#str: ArcStr = literal!("");
        r#str = (match &**iter {
            SINGLE {
                map: __iter_map,
                name: __iter_name,
                range: __iter_range,
            } => singleStr(
                metamodelica::AsArg::as_arg(&__iter_name),
                __iter_range.clone(),
                __iter_map.clone(),
            )?,
            NESTED { .. } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut i in
                            (1..=metamodelica::arrayLength(var_field!((**iter).names, Iterator::NESTED).clone()))
                                .into_iter()
                        {
                            let __x = singleStr(
                                &({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((**iter).names, Iterator::NESTED).borrow(),
                                        i.clone(),
                                    )?)
                                    .clone();
                                    __elt
                                }),
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((**iter).ranges, Iterator::NESTED).borrow(),
                                        i.clone(),
                                    )?)
                                    .clone();
                                    __elt
                                }),
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((**iter).maps, Iterator::NESTED).borrow(),
                                        i.clone(),
                                    )?)
                                    .clone();
                                    __elt
                                }),
                            )?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!(", "),
                ));
                __mm_s.push_str(&*literal!("}"));
                ArcStr::from(__mm_s)
            }
            EMPTY { .. } => literal!("<EMPTY ITERATOR>"),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Iterator.toString"));
                        __mm_s.push_str(&*literal!(" failed for an unknown reason."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(r#str)
    }

    pub(crate) fn map(
        mut iter: metamodelica::Ref<Iterator>,
        mut funcExp: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
        mut funcCrefOpt: Option<
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                    + 'static,
            >,
        >,
        mut mapFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Expression::NFExpression>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
    ) -> Result<metamodelica::Ref<Iterator>> {
        let mut iter: metamodelica::Ref<Iterator> = iter;
        let mut funcCref: MapFuncCref;
        iter = (match &*iter {
            SINGLE { .. } => {
                if (funcCrefOpt).is_some() {
                    funcCref = Util::getOption(funcCrefOpt)?;
                    assign_variant_field!(iter => Iterator::SINGLE; name = funcCref(var_field!((*iter).name, Iterator::SINGLE).clone())?);
                }
                assign_variant_field!(iter => Iterator::SINGLE; range = mapFunc(var_field!((*iter).range, Iterator::SINGLE).clone(), funcExp.clone())?);
                iter
            }
            NESTED { .. } => {
                if (funcCrefOpt).is_some() {
                    funcCref = Util::getOption(funcCrefOpt)?;
                    for mut i in 1..=metamodelica::arrayLength(var_field!((*iter).names, Iterator::NESTED).clone()) {
                        {
                            let __cell0 = funcCref(
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((*iter).names, Iterator::NESTED).borrow(),
                                        i,
                                    )?)
                                    .clone();
                                    __elt
                                }),
                            )?;
                            let __idx0 = i;
                            *metamodelica::index_mut_checked(
                                &mut var_field!((*iter).names, Iterator::NESTED).clone().borrow_mut(),
                                __idx0,
                            )? = __cell0;
                        }
                    }
                }
                for mut i in 1..=metamodelica::arrayLength(var_field!((*iter).ranges, Iterator::NESTED).clone()) {
                    {
                        let __cell1 = mapFunc(
                            ({
                                let __elt = (*metamodelica::index_checked(
                                    &var_field!((*iter).ranges, Iterator::NESTED).borrow(),
                                    i,
                                )?)
                                .clone();
                                __elt
                            }),
                            funcExp.clone(),
                        )?;
                        let __idx1 = i;
                        *metamodelica::index_mut_checked(
                            &mut var_field!((*iter).ranges, Iterator::NESTED).clone().borrow_mut(),
                            __idx1,
                        )? = __cell1;
                    }
                }
                iter
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Iterator.map"));
                        __mm_s.push_str(&*literal!(" failed for an unknown reason."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(iter)
    }
}

pub mod Equation {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Equation {
        SCALAR_EQUATION {
            /// equality type
            ty: metamodelica::Ref<Type::NFType>,
            /// left hand side expression
            lhs: metamodelica::Ref<Expression::NFExpression>,
            /// right hand side expression
            rhs: metamodelica::Ref<Expression::NFExpression>,
            /// origin of equation
            source: metamodelica::Ref<DAE::ElementSource>,
            /// Additional Attributes
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        ARRAY_EQUATION {
            /// equality type containing dimensions
            ty: metamodelica::Ref<Type::NFType>,
            /// left hand side expression
            lhs: metamodelica::Ref<Expression::NFExpression>,
            /// right hand side expression
            rhs: metamodelica::Ref<Expression::NFExpression>,
            /// origin of equation
            source: metamodelica::Ref<DAE::ElementSource>,
            /// Additional Attributes
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
            /// NONE() if not a record
            recordSize: Option<i32>,
        },
        RECORD_EQUATION {
            /// equality type
            ty: metamodelica::Ref<Type::NFType>,
            /// left hand side expression
            lhs: metamodelica::Ref<Expression::NFExpression>,
            /// right hand side expression
            rhs: metamodelica::Ref<Expression::NFExpression>,
            /// origin of equation
            source: metamodelica::Ref<DAE::ElementSource>,
            /// Additional Attributes
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
            /// size of the record
            recordSize: i32,
        },
        ALGORITHM {
            /// output size
            size: i32,
            /// Algorithm statements
            alg: metamodelica::Ref<Algorithm::NFAlgorithm>,
            /// origin of algorithm
            source: metamodelica::Ref<DAE::ElementSource>,
            /// this algorithm was translated from an equation. we should not expand array crefs!
            expand: DAE::Expand,
            /// Additional Attributes
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        IF_EQUATION {
            /// size of equation
            size: i32,
            /// Actual equation body
            body: metamodelica::Ref<IfEquationBody::IfEquationBody>,
            /// origin of equation
            source: metamodelica::Ref<DAE::ElementSource>,
            /// Additional Attributes
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        FOR_EQUATION {
            /// size of equation
            size: i32,
            /// list of all: <iterator, range>
            iter: metamodelica::Ref<Iterator::Iterator>,
            /// iterated equations (only multiples if entwined)
            body: metamodelica::List<metamodelica::Ref<Equation>>,
            /// origin of equation
            source: metamodelica::Ref<DAE::ElementSource>,
            /// Additional Attributes
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        WHEN_EQUATION {
            /// size of equation
            size: i32,
            /// Actual equation body
            body: metamodelica::Ref<WhenEquationBody::WhenEquationBody>,
            /// origin of equation
            source: metamodelica::Ref<DAE::ElementSource>,
            /// Additional Attributes
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        /// Auxiliary equations are generated when auxiliary variables are generated
        ///      that are known to always be solved in this specific equation. E.G. $CSE
        ///      The variable binding contains the equation, but this equation is also
        ///      allowed to have a body for special cases.
        AUX_EQUATION {
            /// Corresponding auxiliary variable
            auxiliary: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
            /// Optional body equation
            body: Option<metamodelica::Ref<Equation>>,
        },
        DUMMY_EQUATION,
    }
    impl metamodelica::gc::MMTrace for Equation {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Equation::SCALAR_EQUATION {
                    ty,
                    lhs,
                    rhs,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Equation::ARRAY_EQUATION {
                    ty,
                    lhs,
                    rhs,
                    source,
                    attr,
                    recordSize,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(recordSize, __mmv)?;
                    Ok(())
                }
                Equation::RECORD_EQUATION {
                    ty,
                    lhs,
                    rhs,
                    source,
                    attr,
                    recordSize,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(recordSize, __mmv)?;
                    Ok(())
                }
                Equation::ALGORITHM {
                    size,
                    alg,
                    source,
                    expand,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(alg, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(expand, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Equation::IF_EQUATION {
                    size,
                    body,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Equation::FOR_EQUATION {
                    size,
                    iter,
                    body,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(iter, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Equation::WHEN_EQUATION {
                    size,
                    body,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Equation::AUX_EQUATION { auxiliary, body } => {
                    metamodelica::gc::MMTrace::mm_accept(auxiliary, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                    Ok(())
                }
                Equation::DUMMY_EQUATION => Ok(()),
            }
        }
    }
    impl Equation {
        pub fn interned_DUMMY_EQUATION() -> metamodelica::Ref<Equation> {
            thread_local! {
                static INTERNED: metamodelica::Ref<Equation> = metamodelica::Ref::new(Equation::DUMMY_EQUATION);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_DUMMY_EQUATION() -> metamodelica::Ref<Equation> {
        Equation::interned_DUMMY_EQUATION()
    }
    impl Default for Equation {
        fn default() -> Self {
            Self::DUMMY_EQUATION
        }
    }
    pub use self::Equation::{
        ALGORITHM, ARRAY_EQUATION, AUX_EQUATION, DUMMY_EQUATION, FOR_EQUATION, IF_EQUATION, RECORD_EQUATION,
        SCALAR_EQUATION, WHEN_EQUATION,
    };
    pub(crate) fn toString(mut eq: metamodelica::Ref<Equation>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut s: ArcStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(size(Pointer::create(eq.clone()), true)?));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        };
        let mut tupl_recd_str: ArcStr;
        r#str = (match &*eq {
            SCALAR_EQUATION {
                attr: __eq_attr,
                lhs: __eq_lhs,
                rhs: __eq_rhs,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("[SCAL] "));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*Expression::toString(__eq_lhs.clone())?);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*Expression::toString(__eq_rhs.clone())?);
                __mm_s.push_str(&*literal!(";"));
                __mm_s.push_str(&*EquationAttributes::toString(
                    metamodelica::AsArg::as_arg(&__eq_attr),
                    &(literal!(" ")),
                )?);
                ArcStr::from(__mm_s)
            }
            ARRAY_EQUATION {
                attr: __eq_attr,
                lhs: __eq_lhs,
                rhs: __eq_rhs,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("[ARRY] "));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*Expression::toString(__eq_lhs.clone())?);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*Expression::toString(__eq_rhs.clone())?);
                __mm_s.push_str(&*literal!(";"));
                __mm_s.push_str(&*EquationAttributes::toString(
                    metamodelica::AsArg::as_arg(&__eq_attr),
                    &(literal!(" ")),
                )?);
                ArcStr::from(__mm_s)
            }
            RECORD_EQUATION {
                attr: __eq_attr,
                lhs: __eq_lhs,
                rhs: __eq_rhs,
                ty: __eq_ty,
                ..
            } => {
                tupl_recd_str = if (Type::isTuple(metamodelica::AsArg::as_arg(&__eq_ty))) {
                    literal!("[TUPL] ")
                } else {
                    literal!("[RECD] ")
                };
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*tupl_recd_str);
                    __mm_s.push_str(&*s);
                    __mm_s.push_str(&*literal!(" "));
                    __mm_s.push_str(&*Expression::toString(__eq_lhs.clone())?);
                    __mm_s.push_str(&*literal!(" = "));
                    __mm_s.push_str(&*Expression::toString(__eq_rhs.clone())?);
                    __mm_s.push_str(&*literal!(";"));
                    __mm_s.push_str(&*EquationAttributes::toString(
                        metamodelica::AsArg::as_arg(&__eq_attr),
                        &(literal!(" ")),
                    )?);
                    ArcStr::from(__mm_s)
                }
            }
            ALGORITHM {
                alg: __eq_alg,
                attr: __eq_attr,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("[ALGO] "));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*EquationAttributes::toString(
                    metamodelica::AsArg::as_arg(&__eq_attr),
                    &(literal!(" ")),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*Algorithm::toString(metamodelica::AsArg::as_arg(&__eq_alg), {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!("[----] "));
                    ArcStr::from(__mm_s)
                })?);
                ArcStr::from(__mm_s)
            }
            IF_EQUATION {
                attr: __eq_attr,
                body: __eq_body,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*IfEquationBody::toString(
                    metamodelica::AsArg::as_arg(&__eq_body),
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!("[----] "));
                        ArcStr::from(__mm_s)
                    }),
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("[-IF-] "));
                        __mm_s.push_str(&*s);
                        __mm_s.push_str(&*EquationAttributes::toString(
                            metamodelica::AsArg::as_arg(&__eq_attr),
                            &(literal!(" ")),
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    },
                    false,
                )?);
                ArcStr::from(__mm_s)
            }
            FOR_EQUATION {
                attr: __eq_attr,
                body: __eq_body,
                iter: __eq_iter,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*forEquationToString(
                    metamodelica::AsArg::as_arg(&__eq_iter),
                    metamodelica::AsArg::as_arg(&__eq_body),
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!("[----] "));
                        ArcStr::from(__mm_s)
                    }),
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("[FOR-] "));
                        __mm_s.push_str(&*s);
                        __mm_s.push_str(&*EquationAttributes::toString(
                            metamodelica::AsArg::as_arg(&__eq_attr),
                            &(literal!(" ")),
                        )?);
                        ArcStr::from(__mm_s)
                    }),
                )?);
                ArcStr::from(__mm_s)
            }
            WHEN_EQUATION {
                attr: __eq_attr,
                body: __eq_body,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*WhenEquationBody::toString(
                    metamodelica::AsArg::as_arg(&__eq_body),
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!("[----] "));
                        ArcStr::from(__mm_s)
                    }),
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("[WHEN] "));
                        __mm_s.push_str(&*s);
                        __mm_s.push_str(&*EquationAttributes::toString(
                            metamodelica::AsArg::as_arg(&__eq_attr),
                            &(literal!(" ")),
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    },
                    false,
                )?);
                ArcStr::from(__mm_s)
            }
            AUX_EQUATION {
                auxiliary: __eq_auxiliary,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("[AUX-] "));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!("Auxiliary equation for "));
                __mm_s.push_str(&*Variable::toString(
                    &(Pointer::access(__eq_auxiliary.clone())),
                    literal!(""),
                    false,
                )?);
                ArcStr::from(__mm_s)
            }
            DUMMY_EQUATION { .. } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("[DUMY] (0) Dummy equation."));
                ArcStr::from(__mm_s)
            }
            _ => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("[FAIL] (0) "));
                __mm_s.push_str(&*literal!("NBEquation.Equation.toString"));
                __mm_s.push_str(&*literal!(" failed!"));
                ArcStr::from(__mm_s)
            }
        });
        Ok(r#str)
    }

    pub(crate) fn pointerToString(
        mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>,
        mut r#str: ArcStr,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = toString(Pointer::access(eqn_ptr), r#str)?;
        Ok(r#str)
    }

    pub(crate) fn source(mut eq: metamodelica::Ref<Equation>) -> Result<metamodelica::Ref<DAE::ElementSource>> {
        let mut src: metamodelica::Ref<DAE::ElementSource>;
        src = (match &*eq {
            SCALAR_EQUATION {
                source: __eq_source, ..
            } => __eq_source.clone(),
            ARRAY_EQUATION {
                source: __eq_source, ..
            } => __eq_source.clone(),
            RECORD_EQUATION {
                source: __eq_source, ..
            } => __eq_source.clone(),
            ALGORITHM {
                source: __eq_source, ..
            } => __eq_source.clone(),
            IF_EQUATION {
                source: __eq_source, ..
            } => __eq_source.clone(),
            FOR_EQUATION {
                source: __eq_source, ..
            } => __eq_source.clone(),
            WHEN_EQUATION {
                source: __eq_source, ..
            } => __eq_source.clone(),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Equation.source"));
                        __mm_s.push_str(&*literal!(" failed for:\n"));
                        __mm_s.push_str(&*toString(eq, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(src)
    }

    pub(crate) fn info(mut eq: metamodelica::Ref<Equation>) -> Result<SourceInfo> {
        let mut info: SourceInfo = ElementSource::getInfo(source(eq.clone())?);
        Ok(info)
    }

    pub(crate) fn size(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>, mut resize: bool) -> Result<i32> {
        let mut s: i32;
        let mut eqn: metamodelica::Ref<Equation>;
        eqn = Pointer::access(eqn_ptr);
        s = (::match_deref::match_deref! { match &(eqn.clone()) {
            Deref @ SCALAR_EQUATION { .. } => {
                1
            },
            Deref @ ARRAY_EQUATION { ty: __eqn_ty, .. } => {
                Type::sizeOf(metamodelica::AsArg::as_arg(&__eqn_ty), resize)?
            },
            Deref @ RECORD_EQUATION { ty: __eqn_ty, .. } => {
                Type::sizeOf(metamodelica::AsArg::as_arg(&__eqn_ty), resize)?
            },
            Deref @ ALGORITHM { size: __eqn_size, .. } => {
                __eqn_size.clone()
            },
            Deref @ IF_EQUATION { body: __eqn_body, size: __eqn_size, .. } => {
                if (resize) {IfEquationBody::size(metamodelica::AsArg::as_arg(&__eqn_body), resize)?} else {__eqn_size.clone()}
            },
            Deref @ FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: body, tail: Deref @ metamodelica::ListNode::Nil }, iter: __eqn_iter, size: __eqn_size, .. } => {
                if (resize) {Iterator::size(metamodelica::AsArg::as_arg(&__eqn_iter), resize)? * size(Pointer::create(body.clone()), resize)?} else {__eqn_size.clone()}
            },
            Deref @ WHEN_EQUATION { body: __eqn_body, size: __eqn_size, .. } => {
                if (resize) {WhenEquationBody::size(metamodelica::AsArg::as_arg(&__eqn_body), resize)?} else {__eqn_size.clone()}
            },
            Deref @ AUX_EQUATION { auxiliary: __eqn_auxiliary, .. } => {
                Variable::size(&(Pointer::access(__eqn_auxiliary.clone())), resize)?
            },
            Deref @ DUMMY_EQUATION { .. } => {
                0
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.size")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*toString(eqn, literal!(""))?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(s)
    }

    pub(crate) fn sizes(
        mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>,
        mut resize: bool,
    ) -> Result<metamodelica::List<i32>> {
        let mut size_lst: metamodelica::List<i32>;
        let mut eqn: metamodelica::Ref<Equation>;
        eqn = Pointer::access(eqn_ptr);
        size_lst = (match &*eqn {
            SCALAR_EQUATION { .. } => list![1],
            ARRAY_EQUATION { ty: __eqn_ty, .. } => {
                ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut dim in (Type::arrayDims(__eqn_ty.clone())).into_iter().cloned() {
                        let __x = Dimension::size(&(dim.clone()), resize)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            }
            RECORD_EQUATION { ty: __eqn_ty, .. } => {
                list![Type::sizeOf(metamodelica::AsArg::as_arg(&__eqn_ty), resize)?]
            }
            ALGORITHM { size: __eqn_size, .. } => list![__eqn_size.clone()],
            IF_EQUATION { size: __eqn_size, .. } => list![__eqn_size.clone()],
            FOR_EQUATION { iter: __eqn_iter, .. } => {
                Iterator::sizes(metamodelica::AsArg::as_arg(&__eqn_iter), resize)?.reverse()
            }
            WHEN_EQUATION { size: __eqn_size, .. } => list![__eqn_size.clone()],
            AUX_EQUATION {
                auxiliary: __eqn_auxiliary,
                ..
            } => list![Variable::size(&(Pointer::access(__eqn_auxiliary.clone())), resize)?],
            DUMMY_EQUATION { .. } => metamodelica::nil(),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Equation.sizes"));
                        __mm_s.push_str(&*literal!(" failed for:\n"));
                        __mm_s.push_str(&*toString(eqn, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(size_lst)
    }

    pub(crate) fn applyToType(
        mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>>,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Equation>>> {
        pub type typeFunc = std::sync::Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>> + 'static,
        >;

        let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>> = eqn_ptr;
        let mut new: metamodelica::Ref<Equation>;
        let mut eqn: metamodelica::Ref<Equation> = Pointer::access(eqn_ptr.clone());
        new = (::match_deref::match_deref! { match &(eqn.clone()) {
            __esc_new @ Deref @ ARRAY_EQUATION { .. } => {
                new = (*__esc_new).clone();
                assign_variant_field!(new => Equation::ARRAY_EQUATION; ty = func(var_field!((*new).ty, Equation::ARRAY_EQUATION).clone())?);
                new.clone()
            },
            __esc_new @ Deref @ RECORD_EQUATION { .. } => {
                new = (*__esc_new).clone();
                assign_variant_field!(new => Equation::RECORD_EQUATION; ty = func(var_field!((*new).ty, Equation::RECORD_EQUATION).clone())?);
                new.clone()
            },
            _ => eqn.clone(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if !(referenceEq(&*(eqn), &*(&*new))) {
            Pointer::update(eqn_ptr.clone(), new);
        }
        Ok(eqn_ptr)
    }

    pub(crate) fn hash(mut eqn: Pointer::Pointer<metamodelica::Ref<Equation>>) -> Result<i32> {
        let mut i: i32 = if (isDummy(&(Pointer::access(eqn.clone())))) {
            0
        } else {
            ComponentRef::hash(&(getEqnName(eqn.clone())?))?
        };
        Ok(i)
    }

    pub(crate) fn equalName(
        mut eqn1: Pointer::Pointer<metamodelica::Ref<Equation>>,
        mut eqn2: Pointer::Pointer<metamodelica::Ref<Equation>>,
    ) -> Result<bool> {
        let mut b: bool = ComponentRef::isEqual(&(getEqnName(eqn1.clone())?), &(getEqnName(eqn2.clone())?))?;
        Ok(b)
    }

    pub(crate) fn isEqualPtrTpl(
        mut tpl: (
            Pointer::Pointer<metamodelica::Ref<Equation>>,
            Pointer::Pointer<metamodelica::Ref<Equation>>,
        ),
    ) -> Result<bool> {
        let mut b: bool;
        let mut eqn1: Pointer::Pointer<metamodelica::Ref<Equation>>;
        let mut eqn2: Pointer::Pointer<metamodelica::Ref<Equation>>;
        (eqn1, eqn2) = tpl;
        b = isEqualPtr(eqn1, eqn2)?;
        Ok(b)
    }

    pub(crate) fn isEqualPtr(
        mut eqn1: Pointer::Pointer<metamodelica::Ref<Equation>>,
        mut eqn2: Pointer::Pointer<metamodelica::Ref<Equation>>,
    ) -> Result<bool> {
        let mut b: bool = isEqual(&(Pointer::access(eqn1.clone())), &(Pointer::access(eqn2.clone())))?;
        Ok(b)
    }

    pub(crate) fn isEqualTpl(mut tpl: &(metamodelica::Ref<Equation>, metamodelica::Ref<Equation>)) -> Result<bool> {
        let mut b: bool;
        let mut eqn1: metamodelica::Ref<Equation>;
        let mut eqn2: metamodelica::Ref<Equation>;
        (eqn1, eqn2) = tpl.clone();
        b = isEqual(&eqn1, &eqn2)?;
        Ok(b)
    }

    pub(crate) fn isEqual(
        mut eqn1: &metamodelica::Ref<Equation>,
        mut eqn2: &metamodelica::Ref<Equation>,
    ) -> Result<bool> {
        let mut b: bool;
        b = (::match_deref::match_deref! { match (eqn1, eqn2) {
            (Deref @ SCALAR_EQUATION { .. }, Deref @ SCALAR_EQUATION { .. }) => Expression::isEqual(var_field!((**eqn1).lhs, Equation::SCALAR_EQUATION).clone(), var_field!((**eqn2).lhs, Equation::SCALAR_EQUATION).clone())? && Expression::isEqual(var_field!((**eqn1).rhs, Equation::SCALAR_EQUATION).clone(), var_field!((**eqn2).rhs, Equation::SCALAR_EQUATION).clone())?,
            (Deref @ ARRAY_EQUATION { .. }, Deref @ ARRAY_EQUATION { .. }) => Expression::isEqual(var_field!((**eqn1).lhs, Equation::ARRAY_EQUATION).clone(), var_field!((**eqn2).lhs, Equation::ARRAY_EQUATION).clone())? && Expression::isEqual(var_field!((**eqn1).rhs, Equation::ARRAY_EQUATION).clone(), var_field!((**eqn2).rhs, Equation::ARRAY_EQUATION).clone())?,
            (Deref @ RECORD_EQUATION { .. }, Deref @ RECORD_EQUATION { .. }) => Expression::isEqual(var_field!((**eqn1).lhs, Equation::RECORD_EQUATION).clone(), var_field!((**eqn2).lhs, Equation::RECORD_EQUATION).clone())? && Expression::isEqual(var_field!((**eqn1).rhs, Equation::RECORD_EQUATION).clone(), var_field!((**eqn2).rhs, Equation::RECORD_EQUATION).clone())?,
            (Deref @ ALGORITHM { .. }, Deref @ ALGORITHM { .. }) => Algorithm::isEqual(var_field!((**eqn1).alg, Equation::ALGORITHM), var_field!((**eqn2).alg, Equation::ALGORITHM))?,
            (Deref @ IF_EQUATION { .. }, Deref @ IF_EQUATION { .. }) => IfEquationBody::isEqual(var_field!((**eqn1).body, Equation::IF_EQUATION), var_field!((**eqn2).body, Equation::IF_EQUATION))?,
            (Deref @ FOR_EQUATION { .. }, Deref @ FOR_EQUATION { .. }) => Iterator::isEqual(var_field!((**eqn1).iter, Equation::FOR_EQUATION), var_field!((**eqn2).iter, Equation::FOR_EQUATION))? && List::all(&(({
            let mut __acc: metamodelica::List<bool> = metamodelica::nil();
            let __thr_src0 = var_field!((**eqn1).body, Equation::FOR_EQUATION).clone();
            let mut __thr_it0 = (&__thr_src0).into_iter();
            let __thr_src1 = var_field!((**eqn2).body, Equation::FOR_EQUATION).clone();
            let mut __thr_it1 = (&__thr_src1).into_iter();
            loop {
                match (__thr_it0.next(), __thr_it1.next()) {
                    (Some(b1), Some(b2)) => {
                        let __x = isEqual(&(b1.clone()), &(b2.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    (None, None) => break,
                    _ => return Err("threaded for: ranges of unequal length"),
                }
            }
            __acc.reverse()
        })), &fnptr!(Util::id, _))?,
            (Deref @ WHEN_EQUATION { .. }, Deref @ WHEN_EQUATION { .. }) => WhenEquationBody::isEqual(var_field!((**eqn1).body, Equation::WHEN_EQUATION), var_field!((**eqn2).body, Equation::WHEN_EQUATION))?,
            (Deref @ AUX_EQUATION { .. }, Deref @ AUX_EQUATION { .. }) => BVariable::equalName(var_field!((**eqn1).auxiliary, Equation::AUX_EQUATION).clone(), var_field!((**eqn2).auxiliary, Equation::AUX_EQUATION).clone())? && Util::optionEqual(var_field!((**eqn1).body, Equation::AUX_EQUATION).clone(), var_field!((**eqn2).body, Equation::AUX_EQUATION).clone(), &move |__a0: metamodelica::Ref<Equation>, __a1: metamodelica::Ref<Equation>| isEqual(&__a0, &__a1))?,
            (Deref @ DUMMY_EQUATION { .. }, Deref @ DUMMY_EQUATION { .. }) => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(b)
    }

    pub(crate) fn getEqnName(
        mut eqn: Pointer::Pointer<metamodelica::Ref<Equation>>,
    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
        let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut residualVar: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        if isDummy(&(Pointer::access(eqn.clone()))) {
            name = openmodelica_nf_frontend::NFComponentRef::interned_EMPTY();
        } else {
            residualVar = getResidualVar(eqn)?;
            name = BVariable::getVarName(residualVar);
        }
        Ok(name)
    }

    pub(crate) fn getResidualVar(
        mut eqn: Pointer::Pointer<metamodelica::Ref<Equation>>,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
        let mut residualVar: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        if let Ok(__iflet0) = EquationAttributes::getResidualVar(&(getAttributes(Pointer::access(eqn.clone())))) {
            residualVar = __iflet0;
        } else {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBEquation.Equation.getResidualVar"));
                    __mm_s.push_str(&*literal!(" failed because of missing residual variable."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        Ok(residualVar)
    }

    pub(crate) fn getSolvedVar(
        mut eqn: &metamodelica::Ref<Equation>,
    ) -> Result<metamodelica::Ref<Variable::NFVariable>> {
        let mut var: metamodelica::Ref<Variable::NFVariable>;
        var = (::match_deref::match_deref! { match eqn {
            Deref @ SCALAR_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
                BVariable::getVar(metamodelica::AsArg::as_arg(&cref), metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo"))?
            },
            Deref @ ARRAY_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
                BVariable::getVar(metamodelica::AsArg::as_arg(&cref), metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo"))?
            },
            Deref @ RECORD_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
                BVariable::getVar(metamodelica::AsArg::as_arg(&cref), metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo"))?
            },
            _ => {
                BVariable::DUMMY_VARIABLE().clone()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(var)
    }

    pub(crate) fn makeAssignment(
        mut lhs: metamodelica::Ref<Expression::NFExpression>,
        mut rhs: metamodelica::Ref<Expression::NFExpression>,
        mut idx: Pointer::Pointer<i32>,
        mut r#str: &ArcStr,
        mut iter: metamodelica::Ref<Iterator::Iterator>,
        mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Equation>>> {
        let mut eq: Pointer::Pointer<metamodelica::Ref<Equation>>;
        let mut e: metamodelica::Ref<Equation>;
        e = makeAssignmentEqn(lhs, rhs, iter, attr)?;
        eq = Pointer::create(e);
        createName(eq.clone(), idx, r#str)?;
        Ok(eq)
    }

    pub(crate) fn makeAssignmentUpdate(
        mut eq: metamodelica::Ref<Equation>,
        mut lhs: metamodelica::Ref<Expression::NFExpression>,
        mut rhs: metamodelica::Ref<Expression::NFExpression>,
        mut iter: metamodelica::Ref<Iterator::Iterator>,
        mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
    ) -> Result<metamodelica::Ref<Equation>> {
        let mut eq: metamodelica::Ref<Equation> = eq;
        let mut res_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> =
            getResidualVar(Pointer::create(eq.clone()))?;
        eq = makeAssignmentEqn(lhs, rhs, iter, attr)?;
        eq = setResidualVar(eq, res_var)?;
        Ok(eq)
    }

    pub(crate) fn makeAssignmentEqn(
        mut lhs: metamodelica::Ref<Expression::NFExpression>,
        mut rhs: metamodelica::Ref<Expression::NFExpression>,
        mut iter: metamodelica::Ref<Iterator::Iterator>,
        mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
    ) -> Result<metamodelica::Ref<Equation>> {
        let mut e: metamodelica::Ref<Equation>;
        let mut ty: metamodelica::Ref<Type::NFType> = Expression::typeOf(lhs.clone());
        e = (::match_deref::match_deref! { match &(&*ty) {
            Deref @ Type::ARRAY { .. } => {
                metamodelica::Ref::new(Equation::ARRAY_EQUATION { ty: ty.clone(), lhs: lhs, rhs: rhs, source: DAE::emptyElementSource().clone(), attr: attr.clone(), recordSize: None })
            },
            Deref @ Type::TUPLE { .. } => {
                metamodelica::Ref::new(Equation::RECORD_EQUATION { ty: ty.clone(), lhs: lhs, rhs: rhs, source: DAE::emptyElementSource().clone(), attr: attr.clone(), recordSize: Type::sizeOf(&ty, false)? })
            },
            Deref @ Type::COMPLEX { complexTy: ct @ Deref @ ComplexType::RECORD { .. }, .. } => {
                metamodelica::Ref::new(Equation::RECORD_EQUATION { ty: ty.clone(), lhs: lhs, rhs: rhs, source: DAE::emptyElementSource().clone(), attr: attr.clone(), recordSize: metamodelica::arrayLength(var_field!((**ct).fields, ComplexType::NFComplexType::RECORD).clone()) })
            },
            _ => {
                metamodelica::Ref::new(Equation::SCALAR_EQUATION { ty: ty.clone(), lhs: lhs, rhs: rhs, source: DAE::emptyElementSource().clone(), attr: attr.clone() })
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if !(Iterator::isEmpty(&iter)) {
            e = metamodelica::Ref::new(Equation::FOR_EQUATION {
                size: Type::sizeOf(&ty, false)? * Iterator::size(&iter, false)?,
                iter: iter,
                body: list![e],
                source: DAE::emptyElementSource().clone(),
                attr: attr,
            });
            e = Inline::inlineForEquation(e)?;
        }
        Ok(e)
    }

    pub(crate) fn makeAlgorithm(
        mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        mut init: bool,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Equation>>> {
        let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation>>;
        let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>;
        alg = metamodelica::Ref::new(Algorithm::NFAlgorithm {
            statements: stmts,
            inputs: metamodelica::nil(),
            outputs: metamodelica::nil(),
            stmtDiffInfo: None,
            scope: NFInstNode::NO_SCOPE().clone(),
            source: DAE::emptyElementSource().clone(),
        });
        alg = Algorithm::setInputsOutputs(alg)?;
        eqn = BackendDAE::lowerAlgorithm(alg, init)?;
        Ok(eqn)
    }

    pub(crate) fn forEquationToString(
        mut iter: &metamodelica::Ref<Iterator::Iterator>,
        mut body: &metamodelica::List<metamodelica::Ref<Equation>>,
        mut indent: &ArcStr,
        mut indicator: &ArcStr,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        let mut iterators: ArcStr;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*indicator);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*indent);
            __mm_s.push_str(&*literal!("for "));
            __mm_s.push_str(&*Iterator::toString(iter)?);
            __mm_s.push_str(&*literal!(" loop\n"));
            ArcStr::from(__mm_s)
        };
        for mut eqn in &**body {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*toString(eqn.clone(), {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("  "));
                    ArcStr::from(__mm_s)
                })?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*indent);
            __mm_s.push_str(&*literal!("end for;"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn getAttributes(
        mut eq: metamodelica::Ref<Equation>,
    ) -> metamodelica::Ref<EquationAttributes::EquationAttributes> {
        '__tco: loop {
            ::match_deref::match_deref! { match &(eq) {
                Deref @ SCALAR_EQUATION { attr: __eq_attr, .. } => {
                    return __eq_attr.clone()
                },
                Deref @ ARRAY_EQUATION { attr: __eq_attr, .. } => {
                    return __eq_attr.clone()
                },
                Deref @ RECORD_EQUATION { attr: __eq_attr, .. } => {
                    return __eq_attr.clone()
                },
                Deref @ ALGORITHM { attr: __eq_attr, .. } => {
                    return __eq_attr.clone()
                },
                Deref @ IF_EQUATION { attr: __eq_attr, .. } => {
                    return __eq_attr.clone()
                },
                Deref @ FOR_EQUATION { attr: __eq_attr, .. } => {
                    return __eq_attr.clone()
                },
                Deref @ WHEN_EQUATION { attr: __eq_attr, .. } => {
                    return __eq_attr.clone()
                },
                Deref @ AUX_EQUATION { body: Some(body), .. } => {
                    { eq = body.clone(); continue '__tco; }
                },
                _ => {
                    return default(EquationKind::UNKNOWN.clone(), false, None, None)
                },
                _ => unreachable!("tail-call lowered match: no arm matched"),
            } }
        }
    }

    pub(crate) fn setAttributes(
        mut eq: metamodelica::Ref<Equation>,
        mut attr: &metamodelica::Ref<EquationAttributes::EquationAttributes>,
    ) -> Result<metamodelica::Ref<Equation>> {
        let mut eq: metamodelica::Ref<Equation> = eq;
        eq = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ SCALAR_EQUATION { .. } => {
                assign_variant_field!(eq => Equation::SCALAR_EQUATION; attr = attr.clone());
                eq
            },
            Deref @ ARRAY_EQUATION { .. } => {
                assign_variant_field!(eq => Equation::ARRAY_EQUATION; attr = attr.clone());
                eq
            },
            Deref @ RECORD_EQUATION { .. } => {
                assign_variant_field!(eq => Equation::RECORD_EQUATION; attr = attr.clone());
                eq
            },
            Deref @ ALGORITHM { .. } => {
                assign_variant_field!(eq => Equation::ALGORITHM; attr = attr.clone());
                eq
            },
            Deref @ IF_EQUATION { .. } => {
                assign_variant_field!(eq => Equation::IF_EQUATION; attr = attr.clone());
                eq
            },
            Deref @ FOR_EQUATION { .. } => {
                assign_variant_field!(eq => Equation::FOR_EQUATION; attr = attr.clone());
                eq
            },
            Deref @ WHEN_EQUATION { .. } => {
                assign_variant_field!(eq => Equation::WHEN_EQUATION; attr = attr.clone());
                eq
            },
            Deref @ AUX_EQUATION { body: Some(body), .. } => {
                assign_variant_field!(eq => Equation::AUX_EQUATION; body = Some(setAttributes(body.clone(), attr)?));
                eq
            },
            _ => return Err("match: no arm matched"),
        } });
        Ok(eq)
    }

    pub(crate) fn setKind(
        mut eq: metamodelica::Ref<Equation>,
        mut kind: EquationKind,
        mut clock_idx: Option<i32>,
    ) -> Result<metamodelica::Ref<Equation>> {
        let mut eq: metamodelica::Ref<Equation> = eq;
        eq = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ SCALAR_EQUATION { attr: __eq_attr, .. } => {
                assign_variant_field!(eq => Equation::SCALAR_EQUATION; attr = EquationAttributes::setKind(__eq_attr.clone(), kind, clock_idx));
                eq
            },
            Deref @ ARRAY_EQUATION { attr: __eq_attr, .. } => {
                assign_variant_field!(eq => Equation::ARRAY_EQUATION; attr = EquationAttributes::setKind(__eq_attr.clone(), kind, clock_idx));
                eq
            },
            Deref @ RECORD_EQUATION { attr: __eq_attr, .. } => {
                assign_variant_field!(eq => Equation::RECORD_EQUATION; attr = EquationAttributes::setKind(__eq_attr.clone(), kind, clock_idx));
                eq
            },
            Deref @ ALGORITHM { attr: __eq_attr, .. } => {
                assign_variant_field!(eq => Equation::ALGORITHM; attr = EquationAttributes::setKind(__eq_attr.clone(), kind, clock_idx));
                eq
            },
            Deref @ IF_EQUATION { attr: __eq_attr, .. } => {
                assign_variant_field!(eq => Equation::IF_EQUATION; attr = EquationAttributes::setKind(__eq_attr.clone(), kind, clock_idx));
                eq
            },
            Deref @ FOR_EQUATION { attr: __eq_attr, .. } => {
                assign_variant_field!(eq => Equation::FOR_EQUATION; attr = EquationAttributes::setKind(__eq_attr.clone(), kind, clock_idx));
                eq
            },
            Deref @ WHEN_EQUATION { attr: __eq_attr, .. } => {
                assign_variant_field!(eq => Equation::WHEN_EQUATION; attr = EquationAttributes::setKind(__eq_attr.clone(), kind, clock_idx));
                eq
            },
            Deref @ AUX_EQUATION { body: Some(body), .. } => {
                assign_variant_field!(eq => Equation::AUX_EQUATION; body = Some(setKind(body.clone(), kind, clock_idx)?));
                eq
            },
            _ => return Err("match: no arm matched"),
        } });
        Ok(eq)
    }

    pub(crate) fn getSource(mut eq: metamodelica::Ref<Equation>) -> metamodelica::Ref<DAE::ElementSource> {
        '__tco: loop {
            ::match_deref::match_deref! { match &(eq) {
                Deref @ SCALAR_EQUATION { source: __eq_source, .. } => {
                    return __eq_source.clone()
                },
                Deref @ ARRAY_EQUATION { source: __eq_source, .. } => {
                    return __eq_source.clone()
                },
                Deref @ RECORD_EQUATION { source: __eq_source, .. } => {
                    return __eq_source.clone()
                },
                Deref @ ALGORITHM { source: __eq_source, .. } => {
                    return __eq_source.clone()
                },
                Deref @ IF_EQUATION { source: __eq_source, .. } => {
                    return __eq_source.clone()
                },
                Deref @ FOR_EQUATION { source: __eq_source, .. } => {
                    return __eq_source.clone()
                },
                Deref @ WHEN_EQUATION { source: __eq_source, .. } => {
                    return __eq_source.clone()
                },
                Deref @ AUX_EQUATION { body: Some(body), .. } => {
                    { eq = body.clone(); continue '__tco; }
                },
                _ => {
                    return DAE::emptyElementSource().clone()
                },
                _ => unreachable!("tail-call lowered match: no arm matched"),
            } }
        }
    }

    pub(crate) fn setDerivative(
        mut eq: metamodelica::Ref<Equation>,
        mut derivative: Pointer::Pointer<metamodelica::Ref<Equation>>,
    ) -> Result<metamodelica::Ref<Equation>> {
        let mut eq: metamodelica::Ref<Equation> = eq;
        let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
        attr = getAttributes(eq.clone());
        assign_field!(attr.derivative = Some(derivative));
        eq = setAttributes(eq, &attr)?;
        Ok(eq)
    }

    pub(crate) fn map(
        mut eq: metamodelica::Ref<Equation>,
        mut funcExp: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
        mut funcCrefOpt: Option<
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                    + 'static,
            >,
        >,
        mut mapFunc: Arc<
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
        >,
    ) -> Result<metamodelica::Ref<Equation>> {
        let mut eq: metamodelica::Ref<Equation> = eq;
        eq = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ SCALAR_EQUATION { lhs: __eq_lhs, rhs: __eq_rhs, .. } => {
                let mut lhs: metamodelica::Ref<Expression::NFExpression>;
                let mut rhs: metamodelica::Ref<Expression::NFExpression>;
                lhs = mapFunc(__eq_lhs.clone(), funcExp.clone())?;
                rhs = mapFunc(__eq_rhs.clone(), funcExp.clone())?;
                if !(referenceEq(&*(&*lhs),&*(var_field!((*eq).lhs, Equation::SCALAR_EQUATION).clone()))) {
                    assign_variant_field!(eq => Equation::SCALAR_EQUATION; lhs = lhs);
                }
                if !(referenceEq(&*(&*rhs),&*(var_field!((*eq).rhs, Equation::SCALAR_EQUATION).clone()))) {
                    assign_variant_field!(eq => Equation::SCALAR_EQUATION; rhs = rhs);
                }
                eq
            },
            Deref @ ARRAY_EQUATION { lhs: __eq_lhs, rhs: __eq_rhs, .. } => {
                let mut lhs: metamodelica::Ref<Expression::NFExpression>;
                let mut rhs: metamodelica::Ref<Expression::NFExpression>;
                lhs = mapFunc(__eq_lhs.clone(), funcExp.clone())?;
                rhs = mapFunc(__eq_rhs.clone(), funcExp.clone())?;
                if !(referenceEq(&*(&*lhs),&*(var_field!((*eq).lhs, Equation::ARRAY_EQUATION).clone()))) {
                    assign_variant_field!(eq => Equation::ARRAY_EQUATION; lhs = lhs);
                }
                if !(referenceEq(&*(&*rhs),&*(var_field!((*eq).rhs, Equation::ARRAY_EQUATION).clone()))) {
                    assign_variant_field!(eq => Equation::ARRAY_EQUATION; rhs = rhs);
                }
                eq
            },
            Deref @ RECORD_EQUATION { lhs: __eq_lhs, rhs: __eq_rhs, .. } => {
                let mut lhs: metamodelica::Ref<Expression::NFExpression>;
                let mut rhs: metamodelica::Ref<Expression::NFExpression>;
                lhs = mapFunc(__eq_lhs.clone(), funcExp.clone())?;
                rhs = mapFunc(__eq_rhs.clone(), funcExp.clone())?;
                if !(referenceEq(&*(&*lhs),&*(var_field!((*eq).lhs, Equation::RECORD_EQUATION).clone()))) {
                    assign_variant_field!(eq => Equation::RECORD_EQUATION; lhs = lhs);
                }
                if !(referenceEq(&*(&*rhs),&*(var_field!((*eq).rhs, Equation::RECORD_EQUATION).clone()))) {
                    assign_variant_field!(eq => Equation::RECORD_EQUATION; rhs = rhs);
                }
                eq
            },
            Deref @ ALGORITHM { alg: __eq_alg, .. } => {
                let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>;
                alg = Algorithm::mapExp(__eq_alg.clone(), &({ let __pe_b1 = funcExp.clone(); move |__pe_a0| mapFunc(__pe_a0, __pe_b1.clone()) }))?;
                if !(referenceEq(&*(&*alg),&*(var_field!((*eq).alg, Equation::ALGORITHM).clone()))) {
                    assign_variant_field!(eq => Equation::ALGORITHM; alg = Algorithm::setInputsOutputs(alg)?);
                }
                eq
            },
            Deref @ IF_EQUATION { body: __eq_body, .. } => {
                let mut ifEqBody: metamodelica::Ref<IfEquationBody::IfEquationBody>;
                ifEqBody = IfEquationBody::map(__eq_body.clone(), funcExp.clone(), funcCrefOpt, mapFunc.clone())?;
                if !(referenceEq(&*(&*ifEqBody),&*(var_field!((*eq).body, Equation::IF_EQUATION).clone()))) {
                    assign_variant_field!(eq => Equation::IF_EQUATION; body = ifEqBody);
                }
                eq
            },
            Deref @ FOR_EQUATION { iter: __eq_iter, .. } => {
                let mut iter: metamodelica::Ref<Iterator::Iterator>;
                iter = Iterator::map(__eq_iter.clone(), funcExp.clone(), funcCrefOpt.clone(), &*mapFunc)?;
                if !(referenceEq(&*(&*iter),&*(var_field!((*eq).iter, Equation::FOR_EQUATION).clone()))) {
                    assign_variant_field!(eq => Equation::FOR_EQUATION; iter = iter);
                }
                assign_variant_field!(eq => Equation::FOR_EQUATION; body = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Equation>> = metamodelica::nil();
            for mut body_eqn in (var_field!((*eq).body, Equation::FOR_EQUATION).clone()).into_iter().cloned() {
                let __x = map(body_eqn.clone(), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }));
                eq
            },
            Deref @ WHEN_EQUATION { body: __eq_body, .. } => {
                let mut whenEqBody: metamodelica::Ref<WhenEquationBody::WhenEquationBody>;
                whenEqBody = WhenEquationBody::map(__eq_body.clone(), funcExp.clone(), funcCrefOpt, mapFunc.clone())?;
                if !(referenceEq(&*(&*whenEqBody),&*(var_field!((*eq).body, Equation::WHEN_EQUATION).clone()))) {
                    assign_variant_field!(eq => Equation::WHEN_EQUATION; body = whenEqBody);
                }
                eq
            },
            Deref @ AUX_EQUATION { body: Some(body), .. } => {
                let mut new_body: metamodelica::Ref<Equation>;
                new_body = map(body.clone(), funcExp.clone(), funcCrefOpt, mapFunc.clone())?;
                if !(referenceEq(&*(&*new_body),&*(body.clone()))) {
                    assign_variant_field!(eq => Equation::AUX_EQUATION; body = Some(new_body));
                }
                eq
            },
            Deref @ DUMMY_EQUATION { .. } => {
                eq
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.map")); __mm_s.push_str(&*literal!(" failed because there was no suitable case for: ")); __mm_s.push_str(&*toString(eq, literal!(""))?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(eq)
    }

    pub(crate) fn mapCondition(
        mut eq: metamodelica::Ref<Equation>,
        mut funcExp: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
        mut funcCrefOpt: Option<
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                    + 'static,
            >,
        >,
        mut mapFunc: Arc<
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
        >,
    ) -> Result<metamodelica::Ref<Equation>> {
        let mut eq: metamodelica::Ref<Equation> = eq;
        eq = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ IF_EQUATION { body: __eq_body, .. } => {
                let mut ifEqBody: metamodelica::Ref<IfEquationBody::IfEquationBody>;
                ifEqBody = IfEquationBody::mapCondition(__eq_body.clone(), funcExp.clone(), funcCrefOpt, mapFunc.clone())?;
                if !(referenceEq(&*(&*ifEqBody),&*(var_field!((*eq).body, Equation::IF_EQUATION).clone()))) {
                    assign_variant_field!(eq => Equation::IF_EQUATION; body = ifEqBody);
                }
                eq
            },
            Deref @ FOR_EQUATION { body: __eq_body, .. } => {
                assign_variant_field!(eq => Equation::FOR_EQUATION; body = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Equation>> = metamodelica::nil();
            for mut body_eqn in (__eq_body.clone()).into_iter().cloned() {
                let __x = mapCondition(body_eqn.clone(), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }));
                eq
            },
            Deref @ WHEN_EQUATION { body: __eq_body, .. } => {
                let mut whenEqBody: metamodelica::Ref<WhenEquationBody::WhenEquationBody>;
                whenEqBody = WhenEquationBody::mapCondition(__eq_body.clone(), funcExp.clone(), funcCrefOpt, mapFunc.clone())?;
                if !(referenceEq(&*(&*whenEqBody),&*(var_field!((*eq).body, Equation::WHEN_EQUATION).clone()))) {
                    assign_variant_field!(eq => Equation::WHEN_EQUATION; body = whenEqBody);
                }
                eq
            },
            Deref @ AUX_EQUATION { body: Some(body), .. } => {
                let mut new_body: metamodelica::Ref<Equation>;
                new_body = mapCondition(body.clone(), funcExp.clone(), funcCrefOpt, mapFunc.clone())?;
                if !(referenceEq(&*(&*new_body),&*(body.clone()))) {
                    assign_variant_field!(eq => Equation::AUX_EQUATION; body = Some(new_body));
                }
                eq
            },
            _ => {
                eq
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(eq)
    }

    pub(crate) fn collectCrefs(
        mut eq: metamodelica::Ref<Equation>,
        mut filter: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
                ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                + 'static,
        >,
        mut mapFunc: Arc<
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
        >,
    ) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
        let mut cref_lst: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut acc: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
            UnorderedSet::new(
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
                13,
            );
        map(
            eq,
            (std::sync::Arc::new({
                let __pe_b1 = filter.clone();
                let __pe_b2 = acc.clone();
                move |__pe_a0| Slice::filterExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
            Some(
                (std::sync::Arc::new({
                    let __pe_b1 = acc.clone();
                    move |__pe_a0| filter(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                            )
                                -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                            + 'static,
                    >),
            ),
            mapFunc.clone(),
        )?;
        cref_lst = UnorderedSet::toList(acc);
        Ok(cref_lst)
    }

    pub(crate) fn collectFromSet(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut acc: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        mut check_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
        if UnorderedSet::contains(cref.clone(), check_set)? {
            UnorderedSet::add(cref.clone(), acc)?;
        }
        Ok(cref)
    }

    pub(crate) fn collectFromMap<T: Clone + 'static + metamodelica::gc::MMTrace>(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut acc: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        mut check_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, T>,
        >,
    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
        if UnorderedMap::contains(cref.clone(), check_map)? {
            UnorderedSet::add(cref.clone(), acc)?;
        }
        Ok(cref)
    }

    pub(crate) fn getLHS(
        mut eq: metamodelica::Ref<Equation>,
    ) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
        '__tco: loop {
            ::match_deref::match_deref! { match &(&*eq) {
                Deref @ SCALAR_EQUATION { lhs: __eq_lhs, .. } => {
                    return Ok(Some(__eq_lhs.clone()))
                },
                Deref @ ARRAY_EQUATION { lhs: __eq_lhs, .. } => {
                    return Ok(Some(__eq_lhs.clone()))
                },
                Deref @ RECORD_EQUATION { lhs: __eq_lhs, .. } => {
                    return Ok(Some(__eq_lhs.clone()))
                },
                Deref @ FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    { eq = (var_field!((*eq).body, Equation::FOR_EQUATION)).head().cloned()?; continue '__tco; }
                },
                Deref @ IF_EQUATION { body: __eq_body, .. } => {
                    let mut exp: metamodelica::Ref<Expression::NFExpression>;
                    let mut success: bool;
                    (exp, success) = IfEquationBody::getLHS(metamodelica::AsArg::as_arg(&__eq_body), openmodelica_nf_frontend::NFExpression::interned_END())?;
                    if (success) {return Ok(Some(exp))} else {return Ok(None)}
                },
                _ => {
                    return Ok(None)
                },
                _ => return Err("match: no arm matched"),
            } }
        }
    }

    pub(crate) fn getRHS(
        mut eq: metamodelica::Ref<Equation>,
    ) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
        '__tco: loop {
            ::match_deref::match_deref! { match &(&*eq) {
                Deref @ SCALAR_EQUATION { rhs: __eq_rhs, .. } => {
                    return Ok(Some(__eq_rhs.clone()))
                },
                Deref @ ARRAY_EQUATION { rhs: __eq_rhs, .. } => {
                    return Ok(Some(__eq_rhs.clone()))
                },
                Deref @ RECORD_EQUATION { rhs: __eq_rhs, .. } => {
                    return Ok(Some(__eq_rhs.clone()))
                },
                Deref @ FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    { eq = (var_field!((*eq).body, Equation::FOR_EQUATION)).head().cloned()?; continue '__tco; }
                },
                Deref @ IF_EQUATION { body: __eq_body, .. } => {
                    let mut exp: metamodelica::Ref<Expression::NFExpression>;
                    let mut success: bool;
                    (exp, success) = IfEquationBody::getRHS(metamodelica::AsArg::as_arg(&__eq_body))?;
                    if (success) {return Ok(Some(exp))} else {return Ok(None)}
                },
                _ => {
                    return Ok(None)
                },
                _ => return Err("match: no arm matched"),
            } }
        }
    }

    pub(crate) fn setLHS(
        mut eq: metamodelica::Ref<Equation>,
        mut lhs: &metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Equation>> {
        let mut eq: metamodelica::Ref<Equation> = eq;
        eq = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ SCALAR_EQUATION { .. } => {
                assign_variant_field!(eq => Equation::SCALAR_EQUATION; lhs = lhs.clone());
                eq
            },
            Deref @ ARRAY_EQUATION { .. } => {
                assign_variant_field!(eq => Equation::ARRAY_EQUATION; lhs = lhs.clone());
                eq
            },
            Deref @ RECORD_EQUATION { .. } => {
                assign_variant_field!(eq => Equation::RECORD_EQUATION; lhs = lhs.clone());
                eq
            },
            Deref @ FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                assign_variant_field!(eq => Equation::FOR_EQUATION; body = list![setLHS((var_field!((*eq).body, Equation::FOR_EQUATION)).head().cloned()?, lhs)?]);
                eq
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.setLHS")); __mm_s.push_str(&*literal!(" failed because LHS ")); __mm_s.push_str(&*Expression::toString(lhs.clone())?); __mm_s.push_str(&*literal!(" could not be set for:\n ")); __mm_s.push_str(&*toString(eq, literal!(""))?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(eq)
    }

    pub(crate) fn setRHS(
        mut eq: metamodelica::Ref<Equation>,
        mut rhs: &metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Equation>> {
        let mut eq: metamodelica::Ref<Equation> = eq;
        eq = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ SCALAR_EQUATION { .. } => {
                assign_variant_field!(eq => Equation::SCALAR_EQUATION; rhs = rhs.clone());
                eq
            },
            Deref @ ARRAY_EQUATION { .. } => {
                assign_variant_field!(eq => Equation::ARRAY_EQUATION; rhs = rhs.clone());
                eq
            },
            Deref @ RECORD_EQUATION { .. } => {
                assign_variant_field!(eq => Equation::RECORD_EQUATION; rhs = rhs.clone());
                eq
            },
            Deref @ FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                assign_variant_field!(eq => Equation::FOR_EQUATION; body = list![setRHS((var_field!((*eq).body, Equation::FOR_EQUATION)).head().cloned()?, rhs)?]);
                eq
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.setRHS")); __mm_s.push_str(&*literal!(" failed because RHS could not be set for: ")); __mm_s.push_str(&*toString(eq, literal!(""))?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(eq)
    }

    pub(crate) fn updateLHSandRHS(
        mut eqn: metamodelica::Ref<Equation>,
        mut lhs: metamodelica::Ref<Expression::NFExpression>,
        mut rhs: metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Equation>> {
        let mut eqn: metamodelica::Ref<Equation> = eqn;
        let mut ty: metamodelica::Ref<Type::NFType>;
        let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes> = getAttributes(eqn.clone());
        let mut src: metamodelica::Ref<DAE::ElementSource> = source(eqn.clone())?;
        let mut opt_rec_size: Option<i32>;
        let mut rec_size: i32;
        ty = Expression::typeOf(lhs.clone());
        opt_rec_size = Type::complexSize(&ty, false)?;
        eqn = (::match_deref::match_deref! { match &((ty.clone(), opt_rec_size.clone())) {
            (Deref @ Type::ARRAY { .. }, _) => metamodelica::Ref::new(Equation::ARRAY_EQUATION { ty: ty, lhs: lhs, rhs: rhs, source: src, attr: attr, recordSize: opt_rec_size }),
            (Deref @ Type::COMPLEX { .. }, Some(__esc_rec_size)) => {
                rec_size = (*__esc_rec_size).clone();
                metamodelica::Ref::new(Equation::RECORD_EQUATION { ty: ty, lhs: lhs, rhs: rhs, source: src, attr: attr, recordSize: rec_size.clone() })
            },
            _ => metamodelica::Ref::new(Equation::SCALAR_EQUATION { ty: ty, lhs: lhs, rhs: rhs, source: src, attr: attr }),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(eqn)
    }

    pub(crate) fn swapLHSandRHS(mut eqn: metamodelica::Ref<Equation>) -> Result<metamodelica::Ref<Equation>> {
        let mut eqn: metamodelica::Ref<Equation> = eqn;
        eqn = (match &*eqn {
            SCALAR_EQUATION {
                lhs: __eqn_lhs,
                rhs: __eqn_rhs,
                ..
            } => {
                let mut tmpExp: metamodelica::Ref<Expression::NFExpression>;
                tmpExp = __eqn_rhs.clone();
                assign_variant_field!(eqn => Equation::SCALAR_EQUATION;
                    rhs = __eqn_lhs.clone(),
                    lhs = tmpExp
                );
                eqn
            }
            ARRAY_EQUATION {
                lhs: __eqn_lhs,
                rhs: __eqn_rhs,
                ..
            } => {
                let mut tmpExp: metamodelica::Ref<Expression::NFExpression>;
                tmpExp = __eqn_rhs.clone();
                assign_variant_field!(eqn => Equation::ARRAY_EQUATION;
                    rhs = __eqn_lhs.clone(),
                    lhs = tmpExp
                );
                eqn
            }
            RECORD_EQUATION {
                lhs: __eqn_lhs,
                rhs: __eqn_rhs,
                ..
            } => {
                let mut tmpExp: metamodelica::Ref<Expression::NFExpression>;
                tmpExp = __eqn_rhs.clone();
                assign_variant_field!(eqn => Equation::RECORD_EQUATION;
                    rhs = __eqn_lhs.clone(),
                    lhs = tmpExp
                );
                eqn
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Equation.swapLHSandRHS"));
                        __mm_s.push_str(&*literal!(" failed for: "));
                        __mm_s.push_str(&*toString(eqn, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(eqn)
    }

    pub(crate) fn getLHSVars(
        mut eqn: &metamodelica::Ref<Equation>,
    ) -> Result<
        metamodelica::List<
            metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
        >,
    > {
        pub(crate) fn getLHSVarsExp(
            mut exp: &metamodelica::Ref<Expression::NFExpression>,
        ) -> Result<
            metamodelica::List<
                metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
            >,
        > {
            let mut vars: metamodelica::List<
                metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
            >;
            vars = (match &**exp {
                Expression::CREF { cref, .. } => {
                    list![metamodelica::Ref::new(Slice::NBSlice {
                        t: BVariable::getVarPointer(cref, metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo"))?,
                        indices: metamodelica::nil()
                    })]
                }
                Expression::TUPLE {
                    elements: __exp_elements,
                    ..
                } => List::flatten(
                    ({
                        let mut __acc: metamodelica::List<_> = metamodelica::nil();
                        for mut elem in (__exp_elements.clone()).into_iter().cloned() {
                            let __x = getLHSVarsExp(&(elem.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                )?,
                Expression::ARRAY { .. } => List::flatten(
                    ({
                        let mut __acc: metamodelica::List<_> = metamodelica::nil();
                        for mut elem in (var_field!((**exp).elements, Expression::NFExpression::ARRAY).clone())
                            .borrow()
                            .iter()
                        {
                            let __x = getLHSVarsExp(&(elem.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                )?,
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NBEquation.Equation.getLHSVars.getLHSVarsExp"));
                            __mm_s.push_str(&*literal!(" failed for: "));
                            __mm_s.push_str(&*Expression::toString(exp.clone())?);
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            });
            Ok(vars)
        }

        let mut vars: metamodelica::List<
            metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
        >;
        vars = (match &**eqn {
            SCALAR_EQUATION { lhs: __eqn_lhs, .. } => getLHSVarsExp(metamodelica::AsArg::as_arg(&__eqn_lhs))?,
            ARRAY_EQUATION { lhs: __eqn_lhs, .. } => getLHSVarsExp(metamodelica::AsArg::as_arg(&__eqn_lhs))?,
            RECORD_EQUATION { lhs: __eqn_lhs, .. } => getLHSVarsExp(metamodelica::AsArg::as_arg(&__eqn_lhs))?,
            FOR_EQUATION { body: __eqn_body, .. } => List::flatten(
                ({
                    let mut __acc: metamodelica::List<
                        metamodelica::List<
                            metamodelica::Ref<
                                Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
                            >,
                        >,
                    > = metamodelica::nil();
                    for mut b in (__eqn_body.clone()).into_iter().cloned() {
                        let __x = getLHSVars(&(b.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?,
            IF_EQUATION { body: __eqn_body, .. } => List::flatten(
                ({
                    let mut __acc: metamodelica::List<
                        metamodelica::List<
                            metamodelica::Ref<
                                Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
                            >,
                        >,
                    > = metamodelica::nil();
                    for mut b in (__eqn_body.then_eqns.clone()).into_iter().cloned() {
                        let __x = getLHSVars(&(Pointer::access(b.clone())))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?,
            _ => metamodelica::nil(),
        });
        Ok(vars)
    }

    pub(crate) fn simplify(
        mut eq: metamodelica::Ref<Equation>,
        mut name: &ArcStr,
        mut indent: &ArcStr,
        mut acc_discrete_states: Pointer::Pointer<
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        >,
        mut acc_previous: Pointer::Pointer<
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        >,
        mut simplifyExp: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
    ) -> Result<metamodelica::Ref<Equation>> {
        pub type SimplifyFunc = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >;

        fn apply(
            mut e: metamodelica::Ref<Expression::NFExpression>,
            mut func: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
            let mut e: metamodelica::Ref<Expression::NFExpression> = e;
            e = func(e)?;
            Ok(e)
        }

        let mut eq: metamodelica::Ref<Equation> = eq;
        let mut old_eq: metamodelica::Ref<Equation>;
        if Flags::isSet(Flags::DUMP_SIMPLIFY.clone())? && !(stringEqual(&indent, &(literal!("")))) {
            metamodelica::print(literal!("\n"));
        }
        eq = map(
            eq,
            simplifyExp.clone(),
            None,
            (std::sync::Arc::new(apply)
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
        old_eq = eq.clone();
        eq = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ SCALAR_EQUATION { .. } => {
                if Expression::isEqual(var_field!((*eq).lhs, Equation::SCALAR_EQUATION).clone(), var_field!((*eq).rhs, Equation::SCALAR_EQUATION).clone())? {
                    assign_variant_field!(eq => Equation::SCALAR_EQUATION;
                        lhs = Expression::makeZero(var_field!((*eq).ty, Equation::SCALAR_EQUATION))?,
                        rhs = Expression::makeZero(var_field!((*eq).ty, Equation::SCALAR_EQUATION))?
                    );
                }
                eq
            },
            Deref @ ARRAY_EQUATION { .. } => {
                if Expression::isEqual(var_field!((*eq).lhs, Equation::ARRAY_EQUATION).clone(), var_field!((*eq).rhs, Equation::ARRAY_EQUATION).clone())? {
                    assign_variant_field!(eq => Equation::ARRAY_EQUATION;
                        lhs = Expression::makeZero(var_field!((*eq).ty, Equation::ARRAY_EQUATION))?,
                        rhs = Expression::makeZero(var_field!((*eq).ty, Equation::ARRAY_EQUATION))?
                    );
                }
                eq
            },
            Deref @ RECORD_EQUATION { .. } => {
                if Expression::isEqual(var_field!((*eq).lhs, Equation::RECORD_EQUATION).clone(), var_field!((*eq).rhs, Equation::RECORD_EQUATION).clone())? {
                    assign_variant_field!(eq => Equation::RECORD_EQUATION;
                        lhs = Expression::makeZero(var_field!((*eq).ty, Equation::RECORD_EQUATION))?,
                        rhs = Expression::makeZero(var_field!((*eq).ty, Equation::RECORD_EQUATION))?
                    );
                }
                eq
            },
            Deref @ ALGORITHM { alg: __eq_alg, .. } => {
                assign_variant_field!(eq => Equation::ALGORITHM; alg = SimplifyModel::simplifyAlgorithm(__eq_alg.clone())?);
                if (Algorithm::isEmpty(var_field!((*eq).alg, Equation::ALGORITHM))) {crate::NBEquation::Equation::interned_DUMMY_EQUATION()} else {eq}
            },
            Deref @ WHEN_EQUATION { .. } => {
                let mut new_eq: metamodelica::Ref<Equation>;
                let mut when_body: metamodelica::Ref<WhenEquationBody::WhenEquationBody>;
                new_eq = (::match_deref::match_deref! { match &(WhenEquationBody::simplify(Some(var_field!((*eq).body, Equation::WHEN_EQUATION).clone()))?) {
            Some(__esc_when_body) => {
                when_body = (*__esc_when_body).clone();
                assign_variant_field!(eq => Equation::WHEN_EQUATION; body = when_body.clone());
                eq
            },
            _ => {
                DetectStates::findDiscreteStatesFromWhenBody(var_field!((*eq).body, Equation::WHEN_EQUATION), acc_discrete_states, acc_previous)?;
                crate::NBEquation::Equation::interned_DUMMY_EQUATION()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                new_eq
            },
            Deref @ IF_EQUATION { .. } => {
                let mut new_eq: metamodelica::Ref<Equation>;
                let mut if_body: metamodelica::Ref<IfEquationBody::IfEquationBody>;
                new_eq = (::match_deref::match_deref! { match &(IfEquationBody::simplify(Some(var_field!((*eq).body, Equation::IF_EQUATION).clone()))) {
            Some(__esc_if_body) => {
                if_body = (*__esc_if_body).clone();
                if (if_body.else_if).is_none() && !(List::hasSeveralElements(&if_body.then_eqns)) {
                    new_eq = Pointer::access((if_body.then_eqns).head().cloned()?);
                } else {
                    assign_variant_field!(eq => Equation::IF_EQUATION; body = if_body.clone());
                    match '__try0: {
                        new_eq = unwrap_break_err!(IfEquationBody::inline(metamodelica::AsArg::as_arg(&if_body), eq.clone()), '__try0);
                        Ok::<_, &'static str>((new_eq.clone(),))
                    } {
                        Ok((__try0_o0,)) => {
                            new_eq = __try0_o0;
                        }
                        Err(_) => {
                            new_eq = eq.clone();
                        }
                    }
                }
                new_eq
            },
            _ => crate::NBEquation::Equation::interned_DUMMY_EQUATION(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                new_eq
            },
            Deref @ FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: Deref @ IF_EQUATION { body: if_body @ Deref @ IfEquationBody::IF_EQUATION_BODY { else_if: None, .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, iter: __eq_iter, .. } => {
                let mut iter: metamodelica::Ref<Iterator::Iterator>;
                let mut status: Solve::Status;
                (iter, status) = Iterator::simplifyRangeCondition(__eq_iter.clone(), if_body.condition.clone())?;
                if status == Solve::Status::EXPLICIT.clone() {
                    assign_variant_field!(eq => Equation::FOR_EQUATION;
                        iter = iter,
                        body = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Equation>> = metamodelica::nil();
            for mut be in (if_body.then_eqns.clone()).into_iter().cloned() {
                let __x = Pointer::access(be.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
                    );
                    assign_variant_field!(eq => Equation::FOR_EQUATION; size = size(Pointer::create(eq.clone()), true)?);
                }
                Inline::inlineForEquation(eq)?
            },
            Deref @ FOR_EQUATION { .. } => {
                Inline::inlineForEquation(eq)?
            },
            Deref @ AUX_EQUATION { .. } => {
                eq
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.simplify")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*toString(eq, literal!(""))?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if Flags::isSet(Flags::DUMP_SIMPLIFY.clone())? && !(isEqual(&old_eq, &eq)?) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*indent);
                __mm_s.push_str(&*literal!("### dumpSimplify | "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(" ###\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*indent);
                __mm_s.push_str(&*literal!("[BEFORE]\n"));
                __mm_s.push_str(&*toString(old_eq, {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("  "));
                    ArcStr::from(__mm_s)
                })?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*indent);
                __mm_s.push_str(&*literal!("[AFTER ]\n"));
                __mm_s.push_str(&*toString(eq.clone(), {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("  "));
                    ArcStr::from(__mm_s)
                })?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
        Ok(eq)
    }

    pub(crate) fn createName(
        mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>,
        mut idx: Pointer::Pointer<i32>,
        mut context: &ArcStr,
    ) -> Result<()> {
        let mut eqn: metamodelica::Ref<Equation> = Pointer::access(eqn_ptr.clone());
        let mut residualVar: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        let mut dummy_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation>>>;
        (residualVar, _) = BVariable::makeResidualVar(context, Pointer::access(idx.clone()), getType(&eqn, false)?)?;
        Pointer::update(idx.clone(), Pointer::access(idx.clone()) + 1);
        eqn = setResidualVar(eqn, residualVar)?;
        eqn = (match &*eqn {
            IF_EQUATION { body: __eqn_body, .. } => {
                IfEquationBody::createNames(metamodelica::AsArg::as_arg(&__eqn_body), idx, context)?;
                eqn
            }
            FOR_EQUATION { body: __eqn_body, .. } => {
                dummy_eqns = ({
                    let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation>>> =
                        metamodelica::nil();
                    for mut body_eqn in (__eqn_body.clone()).into_iter().cloned() {
                        let __x = Pointer::create(body_eqn.clone());
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                for mut body_eqn in &*dummy_eqns {
                    createName(body_eqn.clone(), idx.clone(), context)?;
                }
                assign_variant_field!(eqn => Equation::FOR_EQUATION; body = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Equation>> = metamodelica::nil();
                    for mut body_eqn in (dummy_eqns).into_iter().cloned() {
                        let __x = Pointer::access(body_eqn.clone());
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }));
                eqn
            }
            _ => eqn,
        });
        Pointer::update(eqn_ptr, eqn);
        Ok(())
    }

    pub(crate) fn setResidualVar(
        mut eqn: metamodelica::Ref<Equation>,
        mut residualVar: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    ) -> Result<metamodelica::Ref<Equation>> {
        let mut eqn: metamodelica::Ref<Equation> = eqn;
        eqn = (match &*eqn {
            SCALAR_EQUATION { attr: __eqn_attr, .. } => {
                assign_variant_field!(eqn => Equation::SCALAR_EQUATION; attr = EquationAttributes::setResidualVar(__eqn_attr.clone(), residualVar));
                eqn
            }
            ARRAY_EQUATION { attr: __eqn_attr, .. } => {
                assign_variant_field!(eqn => Equation::ARRAY_EQUATION; attr = EquationAttributes::setResidualVar(__eqn_attr.clone(), residualVar));
                eqn
            }
            RECORD_EQUATION { attr: __eqn_attr, .. } => {
                assign_variant_field!(eqn => Equation::RECORD_EQUATION; attr = EquationAttributes::setResidualVar(__eqn_attr.clone(), residualVar));
                eqn
            }
            ALGORITHM { attr: __eqn_attr, .. } => {
                assign_variant_field!(eqn => Equation::ALGORITHM; attr = EquationAttributes::setResidualVar(__eqn_attr.clone(), residualVar));
                eqn
            }
            IF_EQUATION { attr: __eqn_attr, .. } => {
                assign_variant_field!(eqn => Equation::IF_EQUATION; attr = EquationAttributes::setResidualVar(__eqn_attr.clone(), residualVar));
                eqn
            }
            FOR_EQUATION { attr: __eqn_attr, .. } => {
                assign_variant_field!(eqn => Equation::FOR_EQUATION; attr = EquationAttributes::setResidualVar(__eqn_attr.clone(), residualVar));
                eqn
            }
            WHEN_EQUATION { attr: __eqn_attr, .. } => {
                assign_variant_field!(eqn => Equation::WHEN_EQUATION; attr = EquationAttributes::setResidualVar(__eqn_attr.clone(), residualVar));
                eqn
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Equation.setResidualVar"));
                        __mm_s.push_str(&*literal!(" failed for\n"));
                        __mm_s.push_str(&*toString(eqn, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(eqn)
    }

    pub(crate) fn subIdxName(
        mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>,
        mut idx: Pointer::Pointer<i32>,
    ) -> Result<()> {
        let mut eqn: metamodelica::Ref<Equation> = Pointer::access(eqn_ptr.clone());
        let mut residualVar: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        eqn = (match &*eqn {
            SCALAR_EQUATION { attr: __eqn_attr, .. } => {
                residualVar = EquationAttributes::getResidualVar(metamodelica::AsArg::as_arg(&__eqn_attr))?;
                residualVar = BVariable::subIdxName(residualVar, idx.clone())?;
                assign_variant_field!(eqn => Equation::SCALAR_EQUATION; attr = EquationAttributes::setResidualVar(__eqn_attr.clone(), residualVar));
                eqn
            }
            ARRAY_EQUATION { attr: __eqn_attr, .. } => {
                residualVar = EquationAttributes::getResidualVar(metamodelica::AsArg::as_arg(&__eqn_attr))?;
                residualVar = BVariable::subIdxName(residualVar, idx.clone())?;
                assign_variant_field!(eqn => Equation::ARRAY_EQUATION; attr = EquationAttributes::setResidualVar(__eqn_attr.clone(), residualVar));
                eqn
            }
            RECORD_EQUATION { attr: __eqn_attr, .. } => {
                residualVar = EquationAttributes::getResidualVar(metamodelica::AsArg::as_arg(&__eqn_attr))?;
                residualVar = BVariable::subIdxName(residualVar, idx.clone())?;
                assign_variant_field!(eqn => Equation::RECORD_EQUATION; attr = EquationAttributes::setResidualVar(__eqn_attr.clone(), residualVar));
                eqn
            }
            ALGORITHM { attr: __eqn_attr, .. } => {
                residualVar = EquationAttributes::getResidualVar(metamodelica::AsArg::as_arg(&__eqn_attr))?;
                residualVar = BVariable::subIdxName(residualVar, idx.clone())?;
                assign_variant_field!(eqn => Equation::ALGORITHM; attr = EquationAttributes::setResidualVar(__eqn_attr.clone(), residualVar));
                eqn
            }
            IF_EQUATION { attr: __eqn_attr, .. } => {
                residualVar = EquationAttributes::getResidualVar(metamodelica::AsArg::as_arg(&__eqn_attr))?;
                residualVar = BVariable::subIdxName(residualVar, idx.clone())?;
                assign_variant_field!(eqn => Equation::IF_EQUATION; attr = EquationAttributes::setResidualVar(__eqn_attr.clone(), residualVar));
                eqn
            }
            FOR_EQUATION { attr: __eqn_attr, .. } => {
                residualVar = EquationAttributes::getResidualVar(metamodelica::AsArg::as_arg(&__eqn_attr))?;
                residualVar = BVariable::subIdxName(residualVar, idx.clone())?;
                assign_variant_field!(eqn => Equation::FOR_EQUATION; attr = EquationAttributes::setResidualVar(__eqn_attr.clone(), residualVar));
                eqn
            }
            WHEN_EQUATION { attr: __eqn_attr, .. } => {
                residualVar = EquationAttributes::getResidualVar(metamodelica::AsArg::as_arg(&__eqn_attr))?;
                residualVar = BVariable::subIdxName(residualVar, idx.clone())?;
                assign_variant_field!(eqn => Equation::WHEN_EQUATION; attr = EquationAttributes::setResidualVar(__eqn_attr.clone(), residualVar));
                eqn
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Equation.subIdxName"));
                        __mm_s.push_str(&*literal!(" failed for\n"));
                        __mm_s.push_str(&*toString(eqn, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Pointer::update(idx.clone(), Pointer::access(idx) + 1);
        Pointer::update(eqn_ptr, eqn);
        Ok(())
    }

    pub(crate) fn createResidual(
        mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>,
        mut residualCref_opt: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut new: bool,
        mut allowFail: bool,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Equation>>> {
        let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>> = eqn_ptr;
        let mut eqn: metamodelica::Ref<Equation> = Pointer::access(eqn_ptr.clone());
        let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
        let mut residualCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut lhs: metamodelica::Ref<Expression::NFExpression>;
        let mut rhs: metamodelica::Ref<Expression::NFExpression>;
        let mut failed: bool;
        if isResidual(eqn_ptr.clone()) {
            return Ok(eqn_ptr);
        }
        residualCref = (::match_deref::match_deref! { match &((&*eqn, residualCref_opt)) {
            (_, Some(__esc_residualCref)) => {
                residualCref = (*__esc_residualCref).clone();
                residualCref.clone()
            },
            (Deref @ FOR_EQUATION { .. }, None) => {
                let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
                residualCref = getEqnName(eqn_ptr.clone())?;
                subs = Iterator::normalizedSubscripts(var_field!((*eqn).iter, Equation::FOR_EQUATION), UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1))?;
                subs = listAppend(List::fill(openmodelica_nf_frontend::NFSubscript::interned_WHOLE(), Type::dimensionCount(getType(&((var_field!((*eqn).body, Equation::FOR_EQUATION)).head().cloned()?), false)?)), subs);
                residualCref = ComponentRef::setSubscripts(subs, residualCref)?;
                residualCref
            },
            _ => {
                getEqnName(eqn_ptr.clone())?
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        (eqn, failed) = (match &*eqn {
            IF_EQUATION { body: __eqn_body, .. } => {
                assign_variant_field!(eqn => Equation::IF_EQUATION; body = IfEquationBody::createResidual(metamodelica::AsArg::as_arg(&__eqn_body), residualCref, new, allowFail)?);
                (
                    IfEquationBody::inline(&(var_field!((*eqn).body, Equation::IF_EQUATION).clone()), eqn)?,
                    false,
                )
            }
            FOR_EQUATION { body: __eqn_body, .. } => {
                assign_variant_field!(eqn => Equation::FOR_EQUATION; body = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Equation>> = metamodelica::nil();
                    for mut body_eqn in (__eqn_body.clone()).into_iter().cloned() {
                        let __x = Pointer::access(createResidual(Pointer::create(body_eqn.clone()), Some(residualCref.clone()), new, allowFail)?);
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }));
                (eqn, false)
            }
            _ => {
                lhs = Expression::fromCref(residualCref, false)?;
                match '__try0: {
                    rhs = unwrap_break_err!(getResidualExp(&eqn, !(allowFail)), '__try0);
                    eqn = unwrap_break_err!(setLHS(eqn.clone(), &lhs), '__try0);
                    eqn = unwrap_break_err!(setRHS(eqn.clone(), &rhs), '__try0);
                    failed = false;
                    Ok::<_, &'static str>((failed.clone(),))
                } {
                    Ok((__try0_o0,)) => {
                        failed = __try0_o0;
                    }
                    Err(_) => {
                        failed = true;
                        if !(allowFail) {
                            return Err("fail");
                        }
                    }
                }
                (eqn, failed)
            }
        });
        if !(failed) {
            attr = getAttributes(eqn.clone());
            assign_field!(attr.residual = true);
            eqn = setAttributes(eqn, &attr)?;
        }
        if new {
            eqn_ptr = Pointer::create(eqn);
        } else {
            Pointer::update(eqn_ptr.clone(), eqn);
        }
        Ok(eqn_ptr)
    }

    pub(crate) fn getResidualExp(
        mut eqn: &metamodelica::Ref<Equation>,
        mut throwOnFail: bool,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression>;
        exp = (::match_deref::match_deref! { match eqn {
            Deref @ SCALAR_EQUATION { lhs: __eqn_lhs, rhs: __eqn_rhs, .. } => {
                let mut operator: metamodelica::Ref<Operator::NFOperator>;
                operator = metamodelica::Ref::new(Operator::NFOperator { ty: Expression::typeOf(__eqn_lhs.clone()), op: Operator::Op::ADD.clone() });
                metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![__eqn_rhs.clone()], inv_arguments: list![__eqn_lhs.clone()], operator: operator })
            },
            Deref @ ARRAY_EQUATION { lhs: __eqn_lhs, rhs: __eqn_rhs, .. } => {
                let mut operator: metamodelica::Ref<Operator::NFOperator>;
                operator = metamodelica::Ref::new(Operator::NFOperator { ty: Expression::typeOf(__eqn_lhs.clone()), op: Operator::Op::ADD_EW.clone() });
                metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![__eqn_rhs.clone()], inv_arguments: list![__eqn_lhs.clone()], operator: operator })
            },
            Deref @ RECORD_EQUATION { ty: Deref @ Type::COMPLEX { .. }, lhs: __eqn_lhs, rhs: __eqn_rhs, .. } => {
                let mut operator: metamodelica::Ref<Operator::NFOperator>;
                let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
                let mut cls: metamodelica::Ref<Class::NFClass>;
                cls_node = Type::complexNode(var_field!((**eqn).ty, Equation::RECORD_EQUATION))?;
                cls = NFInstNode::InstNode::getClass(cls_node)?;
                for mut op in &*list![literal!("'+'"), literal!("'0'"), literal!("'-'")] {
                    if !(Class::hasOperator(op.clone(), cls.clone())) {
                        if throwOnFail {
                            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Trying to construct residual expression of type ")); __mm_s.push_str(&*Type::toString(var_field!((**eqn).ty, Equation::RECORD_EQUATION))?); __mm_s.push_str(&*literal!(" for equation ")); __mm_s.push_str(&*toString(eqn.clone(), literal!(""))?); __mm_s.push_str(&*literal!(" but operator ")); __mm_s.push_str(&*op); __mm_s.push_str(&*literal!(" is not defined.")); ArcStr::from(__mm_s) }])?;
                        }
                        return Err("fail");
                    }
                }
                operator = metamodelica::Ref::new(Operator::NFOperator { ty: Expression::typeOf(__eqn_lhs.clone()), op: Operator::Op::ADD.clone() });
                metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![__eqn_rhs.clone()], inv_arguments: list![__eqn_lhs.clone()], operator: operator })
            },
            Deref @ FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                getResidualExp(&((var_field!((**eqn).body, Equation::FOR_EQUATION)).head().cloned()?), throwOnFail)?
            },
            _ => {
                if throwOnFail {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.getResidualExp")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*toString(eqn.clone(), literal!(""))?); ArcStr::from(__mm_s) }])?;
                }
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        exp = SimplifyExp::simplifyDump(
            exp,
            true,
            &(literal!("NBEquation.Equation.getResidualExp")),
            &(literal!("")),
        )?;
        Ok(exp)
    }

    pub(crate) fn tryGetResidualExp(
        mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>,
    ) -> Option<metamodelica::Ref<Expression::NFExpression>> {
        let mut residual: Option<metamodelica::Ref<Expression::NFExpression>>;
        residual = 'mc: {
            let __mc_input = eqn_ptr.clone();
            if let Ok(__v) = (|| -> Result<_> {
                let _ = __mc_input.clone() else { return Err("nomatch") };
                let mut exp: metamodelica::Ref<Expression::NFExpression>;
                exp = getResidualExp(&(Pointer::access(eqn_ptr.clone())), false)?;
                Ok(Some(exp.clone()))
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                let _ = __mc_input.clone() else { return Err("nomatch") };
                Ok(None)
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        };
        residual
    }

    pub(crate) fn getType(
        mut eq: &metamodelica::Ref<Equation>,
        mut skipIterator: bool,
    ) -> Result<metamodelica::Ref<Type::NFType>> {
        let mut ty: metamodelica::Ref<Type::NFType>;
        ty = (match &**eq {
            SCALAR_EQUATION { ty: __eq_ty, .. } => __eq_ty.clone(),
            ARRAY_EQUATION { ty: __eq_ty, .. } => __eq_ty.clone(),
            RECORD_EQUATION { ty: __eq_ty, .. } => __eq_ty.clone(),
            FOR_EQUATION {
                body: __eq_body,
                iter: __eq_iter,
                ..
            } => {
                ty = getType(&((__eq_body).head().cloned()?), false)?;
                if !(skipIterator) {
                    ty =
                        Type::liftArrayRightList(ty, &(Iterator::dimensions(metamodelica::AsArg::as_arg(&__eq_iter))?));
                }
                ty
            }
            WHEN_EQUATION { body: __eq_body, .. } => {
                WhenEquationBody::getType(metamodelica::AsArg::as_arg(&__eq_body))?
            }
            IF_EQUATION { body: __eq_body, .. } => IfEquationBody::getType(metamodelica::AsArg::as_arg(&__eq_body))?,
            _ => openmodelica_nf_frontend::NFType::interned_REAL(),
        });
        Ok(ty)
    }

    pub(crate) fn getForIterator(mut eqn: &metamodelica::Ref<Equation>) -> metamodelica::Ref<Iterator::Iterator> {
        let mut iterator: metamodelica::Ref<Iterator::Iterator>;
        iterator = (match &**eqn {
            FOR_EQUATION { iter: __eqn_iter, .. } => __eqn_iter.clone(),
            _ => crate::NBEquation::Iterator::interned_EMPTY(),
        });
        iterator
    }

    pub(crate) fn getForFrames(
        mut eqn: &metamodelica::Ref<Equation>,
    ) -> metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )> {
        let mut frames: metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator::Iterator>>,
        )>;
        frames = (match &**eqn {
            FOR_EQUATION { iter: __eqn_iter, .. } => {
                let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator::Iterator>>>;
                (names, ranges, maps) = Iterator::getFrames(metamodelica::AsArg::as_arg(&__eqn_iter));
                List::zip3(names, ranges, maps)
            }
            _ => metamodelica::nil(),
        });
        frames
    }

    pub(crate) fn applyForOrder(
        mut eqn: metamodelica::Ref<Equation>,
        mut order: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, EvalOrder>,
        >,
    ) -> Result<metamodelica::Ref<Equation>> {
        let mut eqn: metamodelica::Ref<Equation> = eqn;
        eqn = (match &*eqn {
            FOR_EQUATION { iter: __eqn_iter, .. } => {
                assign_variant_field!(eqn => Equation::FOR_EQUATION; iter = Iterator::applyOrder(__eqn_iter.clone(), order)?);
                eqn
            }
            _ => eqn,
        });
        Ok(eqn)
    }

    pub(crate) fn isDummy(mut eqn: &metamodelica::Ref<Equation>) -> bool {
        let mut b: bool;
        b = (match &**eqn {
            DUMMY_EQUATION { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn isResidual(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> bool {
        let mut b: bool;
        let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
        attr = getAttributes(Pointer::access(eqn_ptr));
        b = attr.residual.clone();
        b
    }

    pub(crate) fn isDiscrete(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> bool {
        let mut b: bool;
        let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
        attr = getAttributes(Pointer::access(eqn_ptr));
        b = attr.kind.clone() == EquationKind::DISCRETE.clone();
        b
    }

    pub(crate) fn isContinuous(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> bool {
        let mut b: bool;
        let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
        attr = getAttributes(Pointer::access(eqn_ptr));
        b = attr.kind.clone() == EquationKind::CONTINUOUS.clone();
        b
    }

    pub(crate) fn isDiscontinuous(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> bool {
        let mut b: bool;
        b = !(isContinuous(eqn_ptr));
        b
    }

    pub(crate) fn isContinousRecordAware(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> Result<bool> {
        let mut b: bool;
        let mut eqn: metamodelica::Ref<Equation> = Pointer::access(eqn_ptr.clone());
        b = (match &*eqn {
            RECORD_EQUATION { ty: __eqn_ty, .. } => Type::isContinuous(__eqn_ty.clone())?,
            _ => isContinuous(eqn_ptr),
        });
        Ok(b)
    }

    pub(crate) fn isInitial(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> bool {
        let mut b: bool;
        let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
        attr = getAttributes(Pointer::access(eqn_ptr));
        b = attr.exclusively_initial.clone();
        b
    }

    pub(crate) fn isWhenEquation(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> Result<bool> {
        let mut b: bool;
        let mut eqn: metamodelica::Ref<Equation> = Pointer::access(eqn_ptr.clone());
        b = (match &*eqn {
            WHEN_EQUATION { .. } => true,
            FOR_EQUATION { body: __eqn_body, .. } => List::any(
                &({
                    let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation>>> =
                        metamodelica::nil();
                    for mut e in (__eqn_body.clone()).into_iter().cloned() {
                        let __x = Pointer::create(e.clone());
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                &isWhenEquation,
            )?,
            _ => false,
        });
        Ok(b)
    }

    pub(crate) fn isIfEquation(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> bool {
        let mut b: bool;
        b = (match &*(Pointer::access(eqn_ptr)) {
            IF_EQUATION { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn isForEquation(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> bool {
        let mut b: bool;
        b = (match &*(Pointer::access(eqn_ptr)) {
            FOR_EQUATION { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn isArrayEquation(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> bool {
        let mut b: bool;
        b = (match &*(Pointer::access(eqn_ptr)) {
            ARRAY_EQUATION { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn isRecordOrTupleEquation(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> Result<bool> {
        let mut b: bool;
        b = (::match_deref::match_deref! { match &(Pointer::access(eqn_ptr)) {
            Deref @ RECORD_EQUATION { .. } => {
                true
            },
            Deref @ ARRAY_EQUATION { recordSize: Some(_), .. } => {
                true
            },
            Deref @ WHEN_EQUATION { body: when_body, .. } => {
                WhenEquationBody::isRecordOrTupleEquation(metamodelica::AsArg::as_arg(&when_body))?
            },
            Deref @ IF_EQUATION { body: if_body, .. } => {
                IfEquationBody::isRecordOrTupleEquation(metamodelica::AsArg::as_arg(&if_body))?
            },
            Deref @ FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: body_eqn, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                isTupleEquation(Pointer::create(body_eqn.clone()))
            },
            _ => {
                false
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(b)
    }

    pub(crate) fn isRecordEquation(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> bool {
        let mut b: bool;
        b = (::match_deref::match_deref! { match &(Pointer::access(eqn_ptr)) {
            e @ Deref @ RECORD_EQUATION { .. } => {
                !(Type::isTuple(var_field!((**e).ty, Equation::RECORD_EQUATION)))
            },
            Deref @ ARRAY_EQUATION { recordSize: Some(_), .. } => {
                true
            },
            _ => {
                false
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        b
    }

    pub(crate) fn isTupleEquation(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> bool {
        let mut b: bool;
        b = (::match_deref::match_deref! { match &(Pointer::access(eqn_ptr)) {
            e @ Deref @ RECORD_EQUATION { .. } => {
                Type::isTuple(var_field!((**e).ty, Equation::RECORD_EQUATION))
            },
            _ => {
                false
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        b
    }

    pub(crate) fn isAlgorithm(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> bool {
        let mut b: bool;
        b = (match &*(Pointer::access(eqn_ptr)) {
            ALGORITHM { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn isParameterEquation(mut eqn: metamodelica::Ref<Equation>) -> Result<bool> {
        let mut b: bool = true;
        let mut b_ptr: Pointer::Pointer<bool> = Pointer::create(b);
        map(
            eqn,
            (std::sync::Arc::new({
                let __pe_b1 = b_ptr.clone();
                move |__pe_a0| expIsParamOrConst(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
            Some(
                (std::sync::Arc::new({
                    let __pe_b1 = b_ptr.clone();
                    move |__pe_a0| crefIsParamOrConst(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                            )
                                -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                            + 'static,
                    >),
            ),
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
        b = Pointer::access(b_ptr);
        Ok(b)
    }

    pub(crate) fn isClocked(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> bool {
        let mut b: bool;
        b = (match &*(getAttributes(Pointer::access(eqn_ptr))) {
            EquationAttributes::EQUATION_ATTRIBUTES {
                kind: EquationKind::CLOCKED { .. },
                ..
            } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn isTypeClock(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> Result<bool> {
        let mut b: bool;
        let mut eq: metamodelica::Ref<Equation> = Pointer::access(eqn_ptr.clone());
        b = (match &*eq {
            SCALAR_EQUATION { ty: __eq_ty, .. } => Type::isClock(metamodelica::AsArg::as_arg(&__eq_ty))?,
            _ => false,
        });
        Ok(b)
    }

    pub(crate) fn isCompound(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> bool {
        let mut b: bool;
        b = (match &*(Pointer::access(eqn_ptr)) {
            ALGORITHM { .. } => true,
            IF_EQUATION { .. } => true,
            WHEN_EQUATION { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn isResizable(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> Result<bool> {
        let mut b: bool;
        b = Type::isResizable(getType(&(Pointer::access(eqn_ptr)), false)?)?;
        Ok(b)
    }

    pub(crate) fn hasDerivative(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>) -> bool {
        let mut b: bool;
        b = (match &*(getAttributes(Pointer::access(eqn_ptr))) {
            EquationAttributes::EQUATION_ATTRIBUTES {
                derivative: Some(_), ..
            } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn expIsParamOrConst(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut b_ptr: Pointer::Pointer<bool>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        if Pointer::access(b_ptr.clone()) {
            let () = (match &*exp {
                Expression::CREF { cref: __exp_cref, .. } => {
                    crefIsParamOrConst(__exp_cref.clone(), b_ptr)?;
                    ()
                }
                Expression::CALL { call: __exp_call } => {
                    Pointer::update(b_ptr, Call::isImpure(metamodelica::AsArg::as_arg(&__exp_call))?);
                    ()
                }
                _ => (),
            });
        }
        Ok(exp)
    }

    pub(crate) fn crefIsParamOrConst(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut b_ptr: Pointer::Pointer<bool>,
    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
        if Pointer::access(b_ptr.clone()) {
            Pointer::update(
                b_ptr,
                BVariable::isParamOrConst(BVariable::getVarPointer(
                    &cref,
                    metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo"),
                )?),
            );
        }
        Ok(cref)
    }

    pub(crate) fn generateBindingEquation(
        mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        mut idx: Pointer::Pointer<i32>,
        mut initial_: bool,
        mut new_iters: metamodelica::Ref<
            UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        >,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Equation>>> {
        let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation>>;
        let mut context: ArcStr = literal!("BND");
        let mut var: metamodelica::Ref<Variable::NFVariable>;
        let mut lhs: metamodelica::Ref<Expression::NFExpression>;
        let mut rhs: metamodelica::Ref<Expression::NFExpression>;
        let mut eqnAttr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
        let mut iter: metamodelica::Ref<Iterator::Iterator>;
        let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
        let mut dims_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >,
        > = UnorderedMap::new(
            (std::sync::Arc::new(
                move |__a0: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>| Dimension::hashList(&__a0),
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>) -> Result<i32>
                        + 'static,
                >),
            (std::sync::Arc::new({
                let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Dimension::NFDimension>,
                          __a1: metamodelica::Ref<Dimension::NFDimension>| {
                        Dimension::isEqual(&__a0, &__a1)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Dimension::NFDimension>,
                                metamodelica::Ref<Dimension::NFDimension>,
                            ) -> Result<bool>
                            + 'static,
                    >);
                move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, &*__pe_b2)
            }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>),
            1,
        );
        let mut iter_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Subscript::NFSubscript>,
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
        var = Pointer::access(var_ptr.clone());
        rhs = (::match_deref::match_deref! { match &(var.binding.clone()) {
            qual @ Deref @ Binding::TYPED_BINDING { .. } => {
                var_field!((**qual).bindingExp, Binding::NFBinding::TYPED_BINDING).clone()
            },
            qual @ Deref @ Binding::UNTYPED_BINDING { .. } => {
                var_field!((**qual).bindingExp, Binding::NFBinding::UNTYPED_BINDING).clone()
            },
            qual @ Deref @ Binding::FLAT_BINDING { .. } => {
                var_field!((**qual).bindingExp, Binding::NFBinding::FLAT_BINDING).clone()
            },
            Deref @ Binding::UNBOUND => {
                let mut start: Option<metamodelica::Ref<Expression::NFExpression>>;
                start = VariableAttributes::getStartAttribute(&var.backendinfo.attributes)?;
                (::match_deref::match_deref! { match &(start) {
            Some(start_exp) => {
                start_exp.clone()
            },
            _ => {
                Expression::makeZero(&(ComponentRef::getSubscriptedType(&var.name, true)?))?
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.generateBindingEquation")); __mm_s.push_str(&*literal!(" failed because of wrong binding type: ")); __mm_s.push_str(&*Binding::toDebugString(&var.binding)); __mm_s.push_str(&*literal!(" for variable ")); __mm_s.push_str(&*Variable::toString(&(Pointer::access(var_ptr.clone())), literal!(""), false)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if BVariable::isClock(var_ptr.clone()) {
            eqnAttr = default(EquationKind::CLOCKED.clone(), initial_, Some(-1), None);
        } else if BVariable::isContinuous(var_ptr, initial_)? {
            eqnAttr = default(
                EquationKind::CONTINUOUS.clone(),
                initial_,
                None,
                var.backendinfo.annotations.optimizerExpression.clone(),
            );
        } else {
            eqnAttr = default(EquationKind::DISCRETE.clone(), initial_, None, None);
        }
        (iter, rhs) = Iterator::extract(rhs, new_iters, dims_map.clone())?;
        rhs = SimplifyExp::simplifyDump(
            rhs,
            true,
            &(literal!("NBEquation.Equation.generateBindingEquation")),
            &(literal!("")),
        )?;
        if Iterator::isEmpty(&iter) {
            lhs = Expression::fromCref(var.name.clone(), false)?;
            eqn = makeAssignment(
                lhs,
                rhs,
                idx,
                &context,
                crate::NBEquation::Iterator::interned_EMPTY(),
                eqnAttr,
            )?;
        } else {
            rhs = Expression::map(
                rhs,
                (std::sync::Arc::new(Expression::repairOperator)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            subs = Iterator::normalizedSubscripts(&iter, iter_map.clone())?;
            lhs = Expression::fromCref(
                ComponentRef::mergeSubscriptsMapped(var.name.clone(), dims_map, iter_map)?,
                false,
            )?;
            eqn = makeAssignment(lhs, rhs, idx, &context, iter, eqnAttr)?;
            renameIterators(eqn.clone(), &(literal!("$i")))?;
        }
        Ok(eqn)
    }

    pub(crate) fn mergeIterators(
        mut eq: metamodelica::Ref<Equation>,
        mut top_level: bool,
    ) -> Result<(
        metamodelica::Ref<Equation>,
        metamodelica::List<metamodelica::Ref<Iterator::Iterator>>,
    )> {
        let mut eq: metamodelica::Ref<Equation> = eq;
        let mut acc: metamodelica::List<metamodelica::Ref<Iterator::Iterator>>;
        (eq, acc) = (match &*eq {
            FOR_EQUATION {
                attr: __eq_attr,
                body: __eq_body,
                iter: __eq_iter,
                size: __eq_size,
                source: __eq_source,
            } => {
                let mut body: metamodelica::Ref<Equation>;
                (body, acc) = mergeIterators((__eq_body).head().cloned()?, false)?;
                acc = metamodelica::cons(__eq_iter.clone(), acc);
                (
                    if (top_level) {
                        metamodelica::Ref::new(Equation::FOR_EQUATION {
                            size: __eq_size.clone(),
                            iter: Iterator::merge(acc.clone())?,
                            body: list![body],
                            source: __eq_source.clone(),
                            attr: __eq_attr.clone(),
                        })
                    } else {
                        body
                    },
                    acc,
                )
            }
            _ => (eq, metamodelica::nil()),
        });
        Ok((eq, acc))
    }

    pub(crate) fn splitIterators(mut eqn: metamodelica::Ref<Equation>) -> Result<metamodelica::Ref<Equation>> {
        let mut eqn: metamodelica::Ref<Equation> = eqn;
        eqn = (match &*eqn {
            FOR_EQUATION {
                attr: __eqn_attr,
                body: __eqn_body,
                iter: __eqn_iter,
                size: __eqn_size,
                source: __eqn_source,
            } => {
                let mut iterators: metamodelica::List<metamodelica::Ref<Iterator::Iterator>>;
                let mut body: metamodelica::Ref<Equation>;
                iterators = Iterator::split(metamodelica::AsArg::as_arg(&__eqn_iter));
                body = (__eqn_body).head().cloned()?;
                for mut iter in &*iterators {
                    body = metamodelica::Ref::new(Equation::FOR_EQUATION {
                        size: __eqn_size.clone(),
                        iter: iter.clone(),
                        body: list![body],
                        source: __eqn_source.clone(),
                        attr: __eqn_attr.clone(),
                    });
                }
                body
            }
            _ => eqn,
        });
        Ok(eqn)
    }

    pub(crate) fn renameIterators(
        mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>,
        mut newBaseName: &ArcStr,
    ) -> Result<()> {
        let mut eqn: metamodelica::Ref<Equation> = Pointer::access(eqn_ptr.clone());
        let () = (match &*eqn {
            FOR_EQUATION { iter: __eqn_iter, .. } => {
                let mut replacements: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Expression::NFExpression>,
                    >,
                >;
                replacements = UnorderedMap::new(
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
                );
                assign_variant_field!(eqn => Equation::FOR_EQUATION;
                            iter = Iterator::rename(__eqn_iter.clone(), newBaseName, replacements.clone())?,
                            body = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Equation>> = metamodelica::nil();
                    for mut body_eqn in (var_field!((*eqn).body, Equation::FOR_EQUATION).clone()).into_iter().cloned() {
                        let __x = map(body_eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
                        );
                Pointer::update(eqn_ptr, eqn);
                ()
            }
            _ => (),
        });
        Ok(())
    }

    pub(crate) fn entwine(
        mut eqn_lst: metamodelica::List<metamodelica::Ref<Equation>>,
        mut nesting_level: i32,
    ) -> Result<metamodelica::List<metamodelica::Ref<Equation>>> {
        let mut entwined: metamodelica::List<metamodelica::Ref<Equation>> = metamodelica::nil();
        let mut eqn1: metamodelica::Ref<Equation>;
        let mut eqn2: metamodelica::Ref<Equation>;
        let mut next: metamodelica::Ref<Equation>;
        let mut rest: metamodelica::List<metamodelica::Ref<Equation>>;
        let mut tmp: metamodelica::List<metamodelica::Ref<Equation>>;
        let mut intersection: metamodelica::Ref<Iterator::Iterator>;
        let mut rest1_left: metamodelica::Ref<Iterator::Iterator>;
        let mut rest1_right: metamodelica::Ref<Iterator::Iterator>;
        let mut rest2_left: metamodelica::Ref<Iterator::Iterator>;
        let mut rest2_right: metamodelica::Ref<Iterator::Iterator>;
        let mut shift: ArcStr = StringUtil::repeat(literal!("  "), nesting_level)?;
        if Flags::isSet(Flags::DUMP_SLICE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*shift);
                __mm_s.push_str(&*literal!("["));
                __mm_s.push_str(&*intString(nesting_level));
                __mm_s.push_str(&*literal!("] ### Entwining following equations:\n"));
                __mm_s.push_str(&*List::toString(
                    eqn_lst.clone(),
                    &({
                        let __pe_b1 = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*shift);
                            __mm_s.push_str(&*literal!("  "));
                            ArcStr::from(__mm_s)
                        };
                        move |__pe_a0| toString(__pe_a0, __pe_b1.clone())
                    }),
                    List::Style::NEWLINE.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn_lst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        eqn1 = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        while !((rest).is_empty()) {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest) {
                Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            eqn2 = metamodelica::Own::own(__pa2);
            rest = metamodelica::Own::own(__pa3);
            eqn1 = (::match_deref::match_deref! { match &((eqn1.clone(), eqn2.clone())) {
                (Deref @ FOR_EQUATION { .. }, Deref @ FOR_EQUATION { .. }) if (Iterator::isEqual(var_field!((*eqn1).iter, Equation::FOR_EQUATION), var_field!((*eqn2).iter, Equation::FOR_EQUATION))?) => {
                    assign_variant_field!(eqn1 => Equation::FOR_EQUATION; body = entwine(listAppend(var_field!((*eqn1).body, Equation::FOR_EQUATION).clone(), var_field!((*eqn2).body, Equation::FOR_EQUATION).clone()), nesting_level + 1)?);
                    eqn1.clone()
                },
                (Deref @ FOR_EQUATION { .. }, Deref @ FOR_EQUATION { .. }) => {
                    let (__pa0, (__pa1, __pa2), (__pa3, __pa4)) = Iterator::intersect(var_field!((*eqn1).iter, Equation::FOR_EQUATION).clone(), var_field!((*eqn2).iter, Equation::FOR_EQUATION).clone())?;
                    intersection = metamodelica::Own::own(__pa0);
                    rest1_left = metamodelica::Own::own(__pa1);
                    rest1_right = metamodelica::Own::own(__pa2);
                    rest2_left = metamodelica::Own::own(__pa3);
                    rest2_right = metamodelica::Own::own(__pa4);
                    tmp = metamodelica::nil();
                    if !(Iterator::isEmpty(&rest1_left)) {
                        tmp = metamodelica::cons(metamodelica::Ref::new(Equation::FOR_EQUATION { size: var_field!((*eqn1).size, Equation::FOR_EQUATION).clone(), iter: rest1_left, body: var_field!((*eqn1).body, Equation::FOR_EQUATION).clone(), source: var_field!((*eqn1).source, Equation::FOR_EQUATION).clone(), attr: var_field!((*eqn1).attr, Equation::FOR_EQUATION).clone() }), tmp);
                    }
                    if !(Iterator::isEmpty(&rest2_left)) {
                        tmp = metamodelica::cons(metamodelica::Ref::new(Equation::FOR_EQUATION { size: var_field!((*eqn2).size, Equation::FOR_EQUATION).clone(), iter: rest2_left, body: var_field!((*eqn2).body, Equation::FOR_EQUATION).clone(), source: var_field!((*eqn2).source, Equation::FOR_EQUATION).clone(), attr: var_field!((*eqn2).attr, Equation::FOR_EQUATION).clone() }), tmp);
                    }
                    if !(Iterator::isEmpty(&intersection)) {
                        tmp = metamodelica::cons(metamodelica::Ref::new(Equation::FOR_EQUATION { size: var_field!((*eqn1).size, Equation::FOR_EQUATION).clone(), iter: intersection, body: entwine(listAppend(var_field!((*eqn1).body, Equation::FOR_EQUATION).clone(), var_field!((*eqn2).body, Equation::FOR_EQUATION).clone()), nesting_level + 1)?, source: var_field!((*eqn1).source, Equation::FOR_EQUATION).clone(), attr: var_field!((*eqn1).attr, Equation::FOR_EQUATION).clone() }), tmp);
                    }
                    if !(Iterator::isEmpty(&rest1_right)) {
                        tmp = metamodelica::cons(metamodelica::Ref::new(Equation::FOR_EQUATION { size: var_field!((*eqn1).size, Equation::FOR_EQUATION).clone(), iter: rest1_right, body: var_field!((*eqn1).body, Equation::FOR_EQUATION).clone(), source: var_field!((*eqn1).source, Equation::FOR_EQUATION).clone(), attr: var_field!((*eqn1).attr, Equation::FOR_EQUATION).clone() }), tmp);
                    }
                    if !(Iterator::isEmpty(&rest2_right)) {
                        tmp = metamodelica::cons(metamodelica::Ref::new(Equation::FOR_EQUATION { size: var_field!((*eqn2).size, Equation::FOR_EQUATION).clone(), iter: rest2_right, body: var_field!((*eqn2).body, Equation::FOR_EQUATION).clone(), source: var_field!((*eqn2).source, Equation::FOR_EQUATION).clone(), attr: var_field!((*eqn2).attr, Equation::FOR_EQUATION).clone() }), tmp);
                    }
                    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(tmp) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: __pa6 } => (__pa5.clone(), __pa6.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    next = metamodelica::Own::own(__pa5);
                    tmp = metamodelica::Own::own(__pa6);
                    entwined = listAppend(tmp, entwined);
                    next
                },
                _ => {
                    entwined = metamodelica::cons(eqn1.clone(), entwined);
                    eqn2.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        entwined = metamodelica::cons(eqn1, entwined).reverse();
        if Flags::isSet(Flags::DUMP_SLICE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*shift);
                __mm_s.push_str(&*literal!("["));
                __mm_s.push_str(&*intString(nesting_level));
                __mm_s.push_str(&*literal!("] +++ Result of entwining:\n"));
                __mm_s.push_str(&*List::toString(
                    entwined.clone(),
                    &({
                        let __pe_b1 = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*shift);
                            __mm_s.push_str(&*literal!("  "));
                            ArcStr::from(__mm_s)
                        };
                        move |__pe_a0| toString(__pe_a0, __pe_b1.clone())
                    }),
                    List::Style::NEWLINE.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
        Ok(entwined)
    }

    pub(crate) fn slice(
        mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>,
        mut indices: metamodelica::List<i32>,
    ) -> Result<(
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation>>>,
        SlicingStatus,
    )> {
        let mut sliced_eqn: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation>>>;
        let mut slicing_status: SlicingStatus;
        let mut eqn: metamodelica::Ref<Equation>;
        let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
        let mut sizes: metamodelica::List<i32>;
        eqn = Pointer::access(eqn_ptr.clone());
        (sliced_eqn, slicing_status) = (match &*eqn.clone() {
            _ if ((indices).is_empty()) => (list![Pointer::create(eqn)], SlicingStatus::UNCHANGED.clone()),
            RECORD_EQUATION { .. } => {
                slicing_status = if (size(eqn_ptr, false)? == ((indices).len() as i32)) {
                    SlicingStatus::TRIVIAL.clone()
                } else {
                    SlicingStatus::NONTRIVIAL.clone()
                };
                (list![Pointer::create(eqn)], slicing_status)
            }
            ARRAY_EQUATION { .. } => {
                slicing_status = if (size(eqn_ptr, false)? == ((indices).len() as i32)) {
                    SlicingStatus::TRIVIAL.clone()
                } else {
                    SlicingStatus::NONTRIVIAL.clone()
                };
                (list![Pointer::create(eqn)], slicing_status)
            }
            FOR_EQUATION { body: __eqn_body, .. } => {
                dims = Type::arrayDims(getType(&eqn, false)?);
                sizes = ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut dim in (dims).into_iter().cloned() {
                        let __x = Dimension::size(&(dim.clone()), false)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                slicing_status = if (size(eqn_ptr, false)? == ((indices).len() as i32)) {
                    SlicingStatus::TRIVIAL.clone()
                } else {
                    SlicingStatus::NONTRIVIAL.clone()
                };
                if slicing_status == SlicingStatus::NONTRIVIAL.clone() {
                    sliced_eqn = sliceFor(
                        (__eqn_body).head().cloned()?,
                        &(getForIterator(&eqn)),
                        sizes,
                        getForFrames(&eqn).reverse(),
                        indices.clone(),
                        false,
                    )?;
                } else {
                    sliced_eqn = list![Pointer::create(eqn)];
                }
                (sliced_eqn, slicing_status)
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.Equation.slice"));
                        __mm_s.push_str(&*literal!(" failed because slicing is not yet supported for:\n"));
                        __mm_s.push_str(&*toString(eqn, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok((sliced_eqn, slicing_status))
    }

    pub(crate) fn sliceFor(
        mut body: metamodelica::Ref<Equation>,
        mut iter: &metamodelica::Ref<Iterator::Iterator>,
        mut sizes: metamodelica::List<i32>,
        mut frames: metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator::Iterator>>,
        )>,
        mut indices: metamodelica::List<i32>,
        mut naive: bool,
    ) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation>>>> {
        let mut result: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation>>>;
        let mut location: metamodelica::List<i32>;
        let mut new_frames: metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator::Iterator>>,
        )>;
        let mut locations: metamodelica::List<metamodelica::List<i32>>;
        let mut locations_T: metamodelica::List<metamodelica::Array<i32>>;
        let mut frame_locations: metamodelica::List<(
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
        let mut frame_status: FrameOrderingStatus;
        let mut recollect_status: RecollectStatus;
        let mut tmp: metamodelica::Ref<Equation>;
        let mut removed_diagonals_opt: Option<
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<Expression::NFExpression>,
                >,
            >,
        >;
        let mut size: i32;
        let mut new_iter: metamodelica::Ref<Iterator::Iterator>;
        if List::hasOneElement(&indices) {
            location = Slice::indexToLocation((indices).head().cloned()?, sizes);
            replacements = UnorderedMap::new(
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
            );
            Iterator::createLocationReplacements(
                iter,
                metamodelica::arrayFromVec(location.into_iter().cloned().collect()),
                replacements.clone(),
            )?;
            tmp = map(
                body,
                (std::sync::Arc::new({
                    let __pe_b1 = replacements;
                    move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
                None,
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
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            result = list![Pointer::create(tmp)];
        } else {
            locations = ({
                let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
                for mut idx in (indices.clone()).into_iter().cloned() {
                    let __x = Slice::indexToLocation(idx.clone(), sizes.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            locations_T = Slice::transposeLocations(&locations, ((sizes).len() as i32))?;
            frame_locations = List::zip(locations_T, frames.clone());
            (frame_locations, replacements, frame_status) = Slice::orderTransposedFrameLocations(frame_locations)?;
            if frame_status == FrameOrderingStatus::FAILURE.clone() {
                if naive {
                    result = List::flatten(
                        ({
                            let mut __acc: metamodelica::List<
                                metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation>>>,
                            > = metamodelica::nil();
                            for mut i in (indices).into_iter().cloned() {
                                let __x = sliceFor(
                                    body.clone(),
                                    iter,
                                    sizes.clone(),
                                    frames.clone(),
                                    list![i.clone()],
                                    true,
                                )?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                    )?;
                } else {
                    result = List::flatten(
                        ({
                            let mut __acc: metamodelica::List<
                                metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation>>>,
                            > = metamodelica::nil();
                            for mut subset in (Slice::naiveSeparation(indices)?).into_iter().cloned() {
                                let __x =
                                    sliceFor(body.clone(), iter, sizes.clone(), frames.clone(), subset.clone(), true)?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                    )?;
                }
            } else {
                (new_frames, removed_diagonals_opt, recollect_status) =
                    Slice::recollectRangesHeuristic(&frame_locations)?;
                if recollect_status == RecollectStatus::FAILURE.clone() || (removed_diagonals_opt).is_some() {
                    if naive {
                        result = List::flatten(
                            ({
                                let mut __acc: metamodelica::List<
                                    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation>>>,
                                > = metamodelica::nil();
                                for mut i in (indices).into_iter().cloned() {
                                    let __x = sliceFor(
                                        body.clone(),
                                        iter,
                                        sizes.clone(),
                                        frames.clone(),
                                        list![i.clone()],
                                        true,
                                    )?;
                                    __acc = cons(__x, __acc);
                                }
                                __acc.reverse()
                            }),
                        )?;
                    } else {
                        result = List::flatten(
                            ({
                                let mut __acc: metamodelica::List<
                                    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation>>>,
                                > = metamodelica::nil();
                                for mut subset in (Slice::naiveSeparation(indices)?).into_iter().cloned() {
                                    let __x = sliceFor(
                                        body.clone(),
                                        iter,
                                        sizes.clone(),
                                        frames.clone(),
                                        subset.clone(),
                                        true,
                                    )?;
                                    __acc = cons(__x, __acc);
                                }
                                __acc.reverse()
                            }),
                        )?;
                    }
                } else {
                    tmp = map(
                        body.clone(),
                        (std::sync::Arc::new({
                            let __pe_b1 = replacements;
                            move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone())
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Expression::NFExpression>,
                                    )
                                        -> Result<metamodelica::Ref<Expression::NFExpression>>
                                    + 'static,
                            >),
                        None,
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
                                    )
                                        -> Result<metamodelica::Ref<Expression::NFExpression>>
                                    + 'static,
                            >),
                    )?;
                    new_iter = Iterator::fromFrames(new_frames);
                    size = Iterator::size(&new_iter, false)? * self::size(Pointer::create(tmp.clone()), false)?;
                    tmp = metamodelica::Ref::new(Equation::FOR_EQUATION {
                        size: size,
                        iter: new_iter,
                        body: list![tmp],
                        source: getSource(body.clone()),
                        attr: getAttributes(body),
                    });
                    result = list![Pointer::create(tmp)];
                }
            }
        }
        Ok(result)
    }

    pub(crate) fn isArrayBodyFor(mut eqn: &metamodelica::Ref<Equation>) -> Result<bool> {
        let mut b: bool;
        b = (::match_deref::match_deref! { match eqn {
            Deref @ FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. } => size(Pointer::create((var_field!((**eqn).body, Equation::FOR_EQUATION)).head().cloned()?), false)? > 1,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(b)
    }

    pub(crate) fn isSingleBodyFor(mut eqn: &metamodelica::Ref<Equation>) -> bool {
        let mut b: bool;
        b = (::match_deref::match_deref! { match eqn {
            Deref @ FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        b
    }

    pub(crate) fn scalarizeElement(
        mut exp: &metamodelica::Ref<Expression::NFExpression>,
        mut subs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut elem: metamodelica::Ref<Expression::NFExpression>;
        elem = (match &**exp {
            Expression::BINARY {
                exp1: __exp_exp1,
                exp2: __exp_exp2,
                operator: __exp_operator,
            } => {
                let mut e1: metamodelica::Ref<Expression::NFExpression>;
                let mut e2: metamodelica::Ref<Expression::NFExpression>;
                e1 = if (Type::isArray(&(Expression::typeOf(__exp_exp1.clone())))) {
                    scalarizeElement(metamodelica::AsArg::as_arg(&__exp_exp1), subs)?
                } else {
                    __exp_exp1.clone()
                };
                e2 = if (Type::isArray(&(Expression::typeOf(__exp_exp2.clone())))) {
                    scalarizeElement(metamodelica::AsArg::as_arg(&__exp_exp2), subs)?
                } else {
                    __exp_exp2.clone()
                };
                Expression::repairOperator(metamodelica::Ref::new(Expression::NFExpression::BINARY {
                    exp1: e1,
                    operator: __exp_operator.clone(),
                    exp2: e2,
                }))?
            }
            Expression::UNARY {
                exp: __exp_exp,
                operator: __exp_operator,
            } => {
                let mut e1: metamodelica::Ref<Expression::NFExpression>;
                e1 = scalarizeElement(metamodelica::AsArg::as_arg(&__exp_exp), subs)?;
                Expression::repairOperator(metamodelica::Ref::new(Expression::NFExpression::UNARY {
                    operator: __exp_operator.clone(),
                    exp: e1,
                }))?
            }
            Expression::MULTARY {
                arguments: __exp_arguments,
                inv_arguments: __exp_inv_arguments,
                operator: __exp_operator,
            } => Expression::repairOperator(metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                arguments: ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                        metamodelica::nil();
                    for mut e in (__exp_arguments.clone()).into_iter().cloned() {
                        let __x = if (Type::isArray(&(Expression::typeOf(e.clone())))) {
                            scalarizeElement(&(e.clone()), subs)?
                        } else {
                            e.clone()
                        };
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                inv_arguments: ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                        metamodelica::nil();
                    for mut e in (__exp_inv_arguments.clone()).into_iter().cloned() {
                        let __x = if (Type::isArray(&(Expression::typeOf(e.clone())))) {
                            scalarizeElement(&(e.clone()), subs)?
                        } else {
                            e.clone()
                        };
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                operator: __exp_operator.clone(),
            }))?,
            _ => Expression::applySubscripts(subs, exp.clone(), false)?,
        });
        Ok(elem)
    }

    pub(crate) fn forArrayBodyRowResidual(
        mut eqn: &metamodelica::Ref<Equation>,
        mut idx: i32,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut residual: metamodelica::Ref<Expression::NFExpression>;
        let mut iter: metamodelica::Ref<Iterator::Iterator>;
        let mut body: metamodelica::Ref<Equation>;
        let mut sizes: metamodelica::List<i32>;
        let mut location: metamodelica::List<i32>;
        let mut n_body: i32;
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
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*eqn)) {
            Deref @ FOR_EQUATION { iter: __pa0, body: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        iter = metamodelica::Own::own(__pa0);
        body = metamodelica::Own::own(__pa1);
        sizes = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut dim in (Type::arrayDims(getType(eqn, false)?)).into_iter().cloned() {
                let __x = Dimension::size(&(dim.clone()), false)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        n_body = ((Type::arrayDims(getType(&body, false)?)).len() as i32);
        location = Slice::indexToLocation(idx, sizes).reverse();
        Iterator::createLocationReplacements(
            &iter,
            metamodelica::arrayFromVec(
                List::lastN(location.clone(), ((location).len() as i32) - n_body)?
                    .into_iter()
                    .cloned()
                    .collect(),
            ),
            replacements.clone(),
        )?;
        residual = Expression::map(
            getResidualExp(&body, true)?,
            (std::sync::Arc::new({
                let __pe_b1 = replacements;
                move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
        residual = scalarizeElement(
            &residual,
            &({
                let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
                for mut l in (List::firstN(location, n_body)?).into_iter().cloned() {
                    let __x = metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                        index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: l.clone() + 1 }),
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?;
        residual = SimplifyExp::simplifyDump(
            residual,
            true,
            &(literal!("NBEquation.Equation.forArrayBodyRowResidual")),
            &(literal!("")),
        )?;
        Ok(residual)
    }

    pub(crate) fn singleSlice(
        mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation>>,
        mut scal_idx: i32,
        mut sizes: metamodelica::List<i32>,
        mut cref_to_solve: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut replacements: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
        mut funcMap: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
        >,
    ) -> Result<(metamodelica::Ref<Equation>, Solve::Status)> {
        let mut sliced_eqn: metamodelica::Ref<Equation>;
        let mut solve_status: Solve::Status = Solve::Status::EXPLICIT.clone();
        let mut eqn: metamodelica::Ref<Equation>;
        let mut location: metamodelica::List<i32>;
        eqn = Pointer::access(eqn_ptr);
        (sliced_eqn, solve_status) = (match &*eqn {
            FOR_EQUATION {
                body: __eqn_body,
                iter: __eqn_iter,
                ..
            } => {
                location = Slice::indexToLocation(scal_idx, sizes);
                Iterator::createLocationReplacements(
                    metamodelica::AsArg::as_arg(&__eqn_iter),
                    metamodelica::arrayFromVec(location.into_iter().cloned().collect()),
                    replacements.clone(),
                )?;
                sliced_eqn = map(
                    (__eqn_body).head().cloned()?,
                    (std::sync::Arc::new({
                        let __pe_b1 = replacements;
                        move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                    None,
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
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?;
                if !(ComponentRef::isEmpty(&cref_to_solve)) {
                    (sliced_eqn, solve_status, _) = Solve::solveBody(sliced_eqn, cref_to_solve, funcMap)?;
                }
                (sliced_eqn, solve_status)
            }
            _ => (eqn, Solve::Status::UNPROCESSED.clone()),
        });
        Ok((sliced_eqn, solve_status))
    }

    fn makeInequality(
        mut tpl: &(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        ),
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut equality_exp: metamodelica::Ref<Expression::NFExpression>;
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut exp: metamodelica::Ref<Expression::NFExpression>;
        (cref, exp) = tpl.clone();
        equality_exp = metamodelica::Ref::new(Expression::NFExpression::RELATION {
            exp1: Expression::fromCref(cref.clone(), false)?,
            operator: metamodelica::Ref::new(Operator::NFOperator {
                ty: ComponentRef::nodeType(&cref)?,
                op: Operator::Op::NEQUAL.clone(),
            }),
            exp2: SimplifyExp::simplifyDump(
                exp,
                true,
                &(literal!("NBEquation.Equation.makeInequality")),
                &(literal!("")),
            )?,
            index: -1,
        });
        Ok(equality_exp)
    }

    pub(crate) fn toStatement(
        mut eqn: &metamodelica::Ref<Equation>,
    ) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
        let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
        stmts = (::match_deref::match_deref! { match eqn {
            Deref @ SCALAR_EQUATION { lhs: __eqn_lhs, rhs: __eqn_rhs, source: __eqn_source, ty: __eqn_ty, .. } => {
                list![metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT { lhs: __eqn_lhs.clone(), rhs: __eqn_rhs.clone(), ty: __eqn_ty.clone(), source: __eqn_source.clone() })]
            },
            Deref @ ARRAY_EQUATION { lhs: __eqn_lhs, rhs: __eqn_rhs, source: __eqn_source, ty: __eqn_ty, .. } => {
                list![metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT { lhs: __eqn_lhs.clone(), rhs: __eqn_rhs.clone(), ty: __eqn_ty.clone(), source: __eqn_source.clone() })]
            },
            Deref @ RECORD_EQUATION { lhs: Deref @ Expression::CREF { cref: lhs_rec, .. }, rhs: Deref @ Expression::CREF { cref: rhs_rec, .. }, attr: __eqn_attr, source: __eqn_source, ty: __eqn_ty, .. } => {
                let mut lhs_exp: metamodelica::Ref<Expression::NFExpression>;
                let mut rhs_exp: metamodelica::Ref<Expression::NFExpression>;
                let mut lhs: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
                let mut rhs: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
                let mut lhs_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                let mut rhs_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                let mut lhs_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
                let mut rhs_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
                lhs_lst = BVariable::getRecordChildren(BVariable::getVarPointer(metamodelica::AsArg::as_arg(&lhs_rec), metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo"))?)?;
                rhs_lst = BVariable::getRecordChildren(BVariable::getVarPointer(metamodelica::AsArg::as_arg(&rhs_rec), metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo"))?)?;
                lhs_subs = ComponentRef::subscriptsAllFlat(metamodelica::AsArg::as_arg(&lhs_rec))?;
                rhs_subs = ComponentRef::subscriptsAllFlat(metamodelica::AsArg::as_arg(&rhs_rec))?;
                if List::compareLength(lhs_lst.clone(), rhs_lst.clone())? == 0 && !(Type::isExternalObject(&(Type::arrayElementType(&(Expression::typeOf(var_field!((**eqn).lhs, Equation::RECORD_EQUATION).clone())))))) {
                    for mut tpl in &*List::zip(lhs_lst, rhs_lst) {
                        (lhs, rhs) = tpl.clone();
                        lhs_exp = Expression::fromCref(ComponentRef::mergeSubscripts(lhs_subs.clone(), BVariable::getVarName(lhs.clone()), true, false, false)?, false)?;
                        rhs_exp = Expression::fromCref(ComponentRef::mergeSubscripts(rhs_subs.clone(), BVariable::getVarName(rhs.clone()), true, false, false)?, false)?;
                        if BVariable::isRecord(lhs.clone()) && BVariable::isRecord(rhs) {
                            stmts = listAppend(toStatement(&(metamodelica::Ref::new(Equation::RECORD_EQUATION { ty: Expression::typeOf(lhs_exp.clone()), lhs: lhs_exp, rhs: rhs_exp, source: __eqn_source.clone(), attr: __eqn_attr.clone(), recordSize: (((BVariable::getRecordChildren(lhs)?)).len() as i32) })))?, stmts);
                        } else {
                            stmts = metamodelica::cons(metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT { lhs: lhs_exp.clone(), rhs: rhs_exp, ty: Expression::typeOf(lhs_exp), source: __eqn_source.clone() }), stmts);
                        }
                    }
                } else {
                    stmts = list![metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT { lhs: var_field!((**eqn).lhs, Equation::RECORD_EQUATION).clone(), rhs: var_field!((**eqn).rhs, Equation::RECORD_EQUATION).clone(), ty: __eqn_ty.clone(), source: __eqn_source.clone() })];
                }
                stmts
            },
            Deref @ RECORD_EQUATION { lhs: __eqn_lhs, rhs: __eqn_rhs, source: __eqn_source, ty: __eqn_ty, .. } => {
                list![metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT { lhs: __eqn_lhs.clone(), rhs: __eqn_rhs.clone(), ty: __eqn_ty.clone(), source: __eqn_source.clone() })]
            },
            Deref @ FOR_EQUATION { body: __eqn_body, iter: __eqn_iter, source: __eqn_source, .. } => {
                let mut iter_lst: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                let mut range_lst: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                let mut maps_lst: metamodelica::List<Option<metamodelica::Ref<Iterator::Iterator>>>;
                let mut map_opt: Option<metamodelica::Ref<Iterator::Iterator>>;
                let mut sub_iters_stmt: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>)>;
                let mut iter: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut iter_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut range: metamodelica::Ref<Expression::NFExpression>;
                let mut iter_elems: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
                let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
                (iter_lst, range_lst, maps_lst) = Iterator::getFrames(metamodelica::AsArg::as_arg(&__eqn_iter));
                body = List::flatten(({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> = metamodelica::nil();
            for mut body_eqn in (__eqn_body.clone()).into_iter().cloned() {
                let __x = toStatement(&(body_eqn.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }))?;
                for mut tpl in &*List::zip3(iter_lst, range_lst, maps_lst).reverse() {
                    (iter, range, map_opt) = tpl.clone();
                    sub_iters_stmt = (::match_deref::match_deref! { match &(map_opt) {
            Some(Deref @ Iterator::SINGLE { name: __esc_iter_name, range: Deref @ Expression::ARRAY { elements: __esc_iter_elems, .. }, map: None }) => {
                iter_name = (*__esc_iter_name).clone();
                iter_elems = (*__esc_iter_elems).clone();
                list![(iter_name.clone(), iter_elems.clone())]
            },
            Some(_) => metamodelica::nil(),
            _ => metamodelica::nil(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                    body = list![metamodelica::Ref::new(Statement::NFStatement::FOR { iterator: ComponentRef::node(&iter)?, range: Some(range), body: body, forType: openmodelica_nf_frontend::NFStatement::ForType::NORMAL, source: __eqn_source.clone(), sub_iters: sub_iters_stmt })];
                }
                body
            },
            Deref @ IF_EQUATION { body: __eqn_body, source: __eqn_source, .. } => {
                list![metamodelica::Ref::new(Statement::NFStatement::IF { branches: IfEquationBody::toStatement(metamodelica::AsArg::as_arg(&__eqn_body))?, source: __eqn_source.clone() })]
            },
            Deref @ WHEN_EQUATION { body: __eqn_body, source: __eqn_source, .. } => {
                list![metamodelica::Ref::new(Statement::NFStatement::WHEN { branches: WhenEquationBody::toStatement(metamodelica::AsArg::as_arg(&__eqn_body))?, source: __eqn_source.clone() })]
            },
            Deref @ ALGORITHM { alg: __eqn_alg, .. } => {
                __eqn_alg.statements.clone()
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.Equation.toStatement")); __mm_s.push_str(&*literal!(" failed it is not yet supported for:\n")); __mm_s.push_str(&*toString(eqn.clone(), literal!(""))?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(stmts)
    }
}

pub mod IfEquationBody {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct IfEquationBody {
        /// the if-condition
        pub condition: metamodelica::Ref<Expression::NFExpression>,
        /// body equations
        pub then_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        /// optional elseif equation
        pub else_if: Option<metamodelica::Ref<IfEquationBody>>,
    }

    impl metamodelica::gc::MMTrace for IfEquationBody {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.condition, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.then_eqns, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.else_if, __mmv)?;
            Ok(())
        }
    }
    impl Default for IfEquationBody {
        fn default() -> Self {
            Self {
                condition: Default::default(),
                then_eqns: Default::default(),
                else_if: Default::default(),
            }
        }
    }

    pub type IF_EQUATION_BODY = IfEquationBody;

    pub(crate) fn toEquation(
        mut body: metamodelica::Ref<IfEquationBody>,
        mut source: metamodelica::Ref<DAE::ElementSource>,
        mut init: bool,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> {
        let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
        let mut isAlgorithm: bool;
        let mut e: metamodelica::Ref<Equation::Equation>;
        let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>;
        let mut size: i32;
        (attr, isAlgorithm) = (::match_deref::match_deref! { match &(body.then_eqns.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: then_eqn, tail: Deref @ metamodelica::ListNode::Nil } => {
                (if (Equation::isDiscrete(then_eqn.clone())) {default(EquationKind::DISCRETE.clone(), init, None, None)} else {default(EquationKind::CONTINUOUS.clone(), init, None, None)}, Equation::isAlgorithm(then_eqn.clone()))
            },
            _ => {
                if Flags::isSet(Flags::FAILTRACE.clone())? {
                    Error::addMessage(Error::COMPILER_WARNING.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.IfEquationBody.toEquation")); __mm_s.push_str(&*literal!(": Creating if-equation with multiple body equations. Unsure of type:\n")); __mm_s.push_str(&*toString(&body, &(literal!("")), literal!(""), false)?); ArcStr::from(__mm_s) }])?;
                }
                (default(EquationKind::CONTINUOUS.clone(), init, None, None), false)
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        e = metamodelica::Ref::new(Equation::Equation::IF_EQUATION {
            size: self::size(&body, false)?,
            body: body,
            source: source.clone(),
            attr: attr.clone(),
        });
        if isAlgorithm {
            alg = metamodelica::Ref::new(Algorithm::NFAlgorithm {
                statements: Equation::toStatement(&e)?,
                inputs: metamodelica::nil(),
                outputs: metamodelica::nil(),
                stmtDiffInfo: None,
                scope: NFInstNode::NO_SCOPE().clone(),
                source: source,
            });
            alg = Algorithm::setInputsOutputs(alg)?;
            size = ({
                let mut __acc: i32 = 0;
                for mut out in (alg.outputs.clone()).into_iter().cloned() {
                    let __x = ComponentRef::size(&(out.clone()), false, false)?;
                    __acc += __x;
                }
                __acc
            });
            eqn = Pointer::create(metamodelica::Ref::new(Equation::Equation::ALGORITHM {
                size: size,
                alg: alg.clone(),
                source: alg.source.clone(),
                expand: openmodelica_frontend_types::DAE::Expand::EXPAND,
                attr: attr,
            }));
        } else {
            eqn = Pointer::create(e);
        }
        Ok(eqn)
    }

    pub(crate) fn makeIfEquation(
        mut body: metamodelica::Ref<IfEquationBody>,
        mut idx: Pointer::Pointer<i32>,
        mut r#str: &ArcStr,
        mut iter: metamodelica::Ref<Iterator::Iterator>,
        mut source: metamodelica::Ref<DAE::ElementSource>,
        mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> {
        let mut eq: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        let mut e: metamodelica::Ref<Equation::Equation>;
        e = makeIfEquationEqn(body, iter, source, attr)?;
        eq = Pointer::create(e);
        Equation::createName(eq.clone(), idx, r#str)?;
        Ok(eq)
    }

    fn makeIfEquationEqn(
        mut body: metamodelica::Ref<IfEquationBody>,
        mut iter: metamodelica::Ref<Iterator::Iterator>,
        mut source: metamodelica::Ref<DAE::ElementSource>,
        mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
    ) -> Result<metamodelica::Ref<Equation::Equation>> {
        let mut e: metamodelica::Ref<Equation::Equation>;
        e = metamodelica::Ref::new(Equation::Equation::IF_EQUATION {
            size: size(&body, false)?,
            body: body.clone(),
            source: source.clone(),
            attr: attr.clone(),
        });
        if !(Iterator::isEmpty(&iter)) {
            e = metamodelica::Ref::new(Equation::Equation::FOR_EQUATION {
                size: size(&body, false)? * Iterator::size(&iter, false)?,
                iter: iter,
                body: list![e],
                source: source,
                attr: attr,
            });
            e = Inline::inlineForEquation(e)?;
        }
        Ok(e)
    }

    pub(crate) fn toString(
        mut body: &metamodelica::Ref<IfEquationBody>,
        mut indent: &ArcStr,
        mut elseStr: ArcStr,
        mut selfCall: bool,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = elseStr;
        if !(selfCall) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*indent);
                ArcStr::from(__mm_s)
            };
        }
        if !(Expression::isEnd(&body.condition)) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("if "));
                __mm_s.push_str(&*Expression::toString(body.condition.clone())?);
                __mm_s.push_str(&*literal!(" then\n"));
                ArcStr::from(__mm_s)
            };
        } else {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        for mut eqn in &*body.then_eqns.clone() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*Equation::toString(Pointer::access(eqn.clone()), {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("  "));
                    ArcStr::from(__mm_s)
                })?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        if (body.else_if).is_some() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*toString(
                    &(Util::getOption(body.else_if.clone())?),
                    indent,
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*indent);
                        __mm_s.push_str(&*literal!("else"));
                        ArcStr::from(__mm_s)
                    },
                    true,
                )?);
                ArcStr::from(__mm_s)
            };
        }
        if !(selfCall) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*indent);
                __mm_s.push_str(&*literal!("end if;"));
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn map(
        mut ifBody: metamodelica::Ref<IfEquationBody>,
        mut funcExp: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
        mut funcCrefOpt: Option<
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                    + 'static,
            >,
        >,
        mut mapFunc: Arc<
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
        >,
    ) -> Result<metamodelica::Ref<IfEquationBody>> {
        let mut ifBody: metamodelica::Ref<IfEquationBody> = ifBody;
        ifBody = mapEqnExpCref(
            ifBody,
            &({
                let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static> = (std::sync::Arc::new({
                    let __pe_b1 = funcExp.clone();
                    let __pe_b2 = funcCrefOpt.clone();
                    let __pe_b3 = mapFunc.clone();
                    move |__pe_a0| Equation::map(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Equation::Equation>,
                            ) -> Result<metamodelica::Ref<Equation::Equation>>
                            + 'static,
                    >);
                move |__pe_a0| Pointer::apply(__pe_a0, __pe_b1.clone())
            }),
            funcExp.clone(),
            funcCrefOpt,
            &*mapFunc,
        )?;
        Ok(ifBody)
    }

    pub(crate) fn mapCondition(
        mut ifBody: metamodelica::Ref<IfEquationBody>,
        mut funcExp: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
        mut funcCrefOpt: Option<
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                    + 'static,
            >,
        >,
        mut mapFunc: Arc<
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
        >,
    ) -> Result<metamodelica::Ref<IfEquationBody>> {
        let mut ifBody: metamodelica::Ref<IfEquationBody> = ifBody;
        let mut condition: metamodelica::Ref<Expression::NFExpression>;
        condition = mapFunc(ifBody.condition.clone(), funcExp.clone())?;
        if !(referenceEq(&*(&*condition), &*(ifBody.condition.clone()))) {
            assign_field!(ifBody.condition = condition);
        }
        assign_field!(
            ifBody.else_if = Util::applyOption(
                ifBody.else_if.clone(),
                &({
                    let __pe_b1 = funcExp.clone();
                    let __pe_b2 = funcCrefOpt;
                    let __pe_b3 = mapFunc.clone();
                    move |__pe_a0| mapCondition(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
                })
            )?
        );
        Ok(ifBody)
    }

    pub(crate) fn mapEqnExpCref(
        mut ifBody: metamodelica::Ref<IfEquationBody>,
        mut func: &dyn ::std::ops::Fn(
            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        ) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        mut funcExp: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
        mut funcCrefOpt: Option<
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                    + 'static,
            >,
        >,
        mut mapFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Expression::NFExpression>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
    ) -> Result<metamodelica::Ref<IfEquationBody>> {
        let mut ifBody: metamodelica::Ref<IfEquationBody> = ifBody;
        let mut condition: metamodelica::Ref<Expression::NFExpression>;
        let mut else_if: metamodelica::Ref<IfEquationBody>;
        let mut old_else_if: metamodelica::Ref<IfEquationBody>;
        condition = mapFunc(ifBody.condition.clone(), funcExp.clone())?;
        if !(referenceEq(&*(&*condition), &*(ifBody.condition.clone()))) {
            assign_field!(ifBody.condition = condition);
        }
        assign_field!(ifBody.then_eqns = List::map(ifBody.then_eqns.clone(), func)?);
        if (ifBody.else_if).is_some() {
            old_else_if = Util::getOption(ifBody.else_if.clone())?;
            else_if = mapEqnExpCref(old_else_if.clone(), func, funcExp.clone(), funcCrefOpt, mapFunc)?;
            if !(referenceEq(&*(&*else_if), &*(old_else_if))) {
                assign_field!(ifBody.else_if = Some(else_if));
            }
        }
        Ok(ifBody)
    }

    pub(crate) fn size(mut body: &metamodelica::Ref<IfEquationBody>, mut resize: bool) -> Result<i32> {
        let mut size: i32 = ({
            let mut __acc: i32 = 0;
            for mut eqn in (body.then_eqns.clone()).into_iter().cloned() {
                let __x = Equation::size(eqn.clone(), resize)?;
                __acc += __x;
            }
            __acc
        });
        Ok(size)
    }

    pub(crate) fn isEqual(
        mut body1: &metamodelica::Ref<IfEquationBody>,
        mut body2: &metamodelica::Ref<IfEquationBody>,
    ) -> Result<bool> {
        let mut b: bool;
        b = List::all(
            &({
                let mut __acc: metamodelica::List<bool> = metamodelica::nil();
                let __thr_src0 = body1.then_eqns.clone();
                let mut __thr_it0 = (&__thr_src0).into_iter();
                let __thr_src1 = body2.then_eqns.clone();
                let mut __thr_it1 = (&__thr_src1).into_iter();
                loop {
                    match (__thr_it0.next(), __thr_it1.next()) {
                        (Some(b1), Some(b2)) => {
                            let __x = Equation::isEqualPtr(b1.clone(), b2.clone())?;
                            __acc = cons(__x, __acc);
                        }
                        (None, None) => break,
                        _ => return Err("threaded for: ranges of unequal length"),
                    }
                }
                __acc.reverse()
            }),
            &fnptr!(Util::id, _),
        )? && Util::optionEqual(
            body1.else_if.clone(),
            body2.else_if.clone(),
            &move |__a0: metamodelica::Ref<IfEquationBody>, __a1: metamodelica::Ref<IfEquationBody>| {
                isEqual(&__a0, &__a1)
            },
        )?;
        Ok(b)
    }

    pub(crate) fn createNames(
        mut body: &metamodelica::Ref<IfEquationBody>,
        mut idx: Pointer::Pointer<i32>,
        mut context: &ArcStr,
    ) -> Result<()> {
        for mut eqn in &*body.then_eqns.clone() {
            Equation::createName(eqn.clone(), idx.clone(), context)?;
        }
        if (body.else_if).is_some() {
            createNames(&(Util::getOption(body.else_if.clone())?), idx, context)?;
        }
        Ok(())
    }

    pub(crate) fn toStatement(
        mut body: &metamodelica::Ref<IfEquationBody>,
    ) -> Result<
        metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        )>,
    > {
        let mut stmts: metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        )>;
        let mut stmt: (
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        );
        let mut condition: metamodelica::Ref<Expression::NFExpression> = if (Expression::isEnd(&body.condition)) {
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true })
        } else {
            body.condition.clone()
        };
        stmt = (
            condition,
            List::flatten(
                ({
                    let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> =
                        metamodelica::nil();
                    for mut eqn in (body.then_eqns.clone()).into_iter().cloned() {
                        let __x = Equation::toStatement(&(Pointer::access(eqn.clone())))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?,
        );
        if (body.else_if).is_some() {
            stmts = metamodelica::cons(stmt, toStatement(&(Util::getOption(body.else_if.clone())?))?);
        } else {
            stmts = list![stmt];
        }
        Ok(stmts)
    }

    pub(crate) fn createResidual(
        mut body: &metamodelica::Ref<IfEquationBody>,
        mut res: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut new: bool,
        mut allowFail: bool,
    ) -> Result<metamodelica::Ref<IfEquationBody>> {
        let mut body_res: metamodelica::Ref<IfEquationBody>;
        let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        body_res = metamodelica::Ref::new(IfEquationBody {
            condition: body.condition.clone(),
            then_eqns: metamodelica::nil(),
            else_if: Util::applyOption(
                body.else_if.clone(),
                &({
                    let __pe_b1 = res.clone();
                    let __pe_b2 = new;
                    let __pe_b3 = allowFail;
                    move |__pe_a0| createResidual(&__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
                }),
            )?,
        });
        body_res = (::match_deref::match_deref! { match &(body.then_eqns.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __esc_eqn_ptr, tail: Deref @ metamodelica::ListNode::Nil } => {
                eqn_ptr = (*__esc_eqn_ptr).clone();
                assign_field!(body_res.then_eqns = metamodelica::cons(Equation::createResidual(eqn_ptr.clone(), Some(res), new, allowFail)?, body_res.then_eqns.clone()));
                body_res
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.IfEquationBody.createResidual")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*toString(body, &(literal!("")), literal!(""), false)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(body_res)
    }

    pub(crate) fn inline(
        mut body: &metamodelica::Ref<IfEquationBody>,
        mut eqn: metamodelica::Ref<Equation::Equation>,
    ) -> Result<metamodelica::Ref<Equation::Equation>> {
        let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
        let mut lhs: metamodelica::Ref<Expression::NFExpression>;
        let mut rhs: metamodelica::Ref<Expression::NFExpression>;
        let mut success: bool;
        (lhs, success) = getLHS(body, openmodelica_nf_frontend::NFExpression::interned_END())?;
        if success {
            rhs = SimplifyExp::simplify((getRHS(body)?).0, false)?;
            eqn = Equation::makeAssignmentUpdate(
                eqn.clone(),
                lhs,
                rhs,
                Equation::getForIterator(&eqn),
                Equation::getAttributes(eqn),
            )?;
        }
        Ok(eqn)
    }

    pub(crate) fn getLHS(
        mut body: &metamodelica::Ref<IfEquationBody>,
        mut exp: metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let mut success: bool = true;
        let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
        exp = (::match_deref::match_deref! { match &(body.then_eqns.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __esc_eqn_ptr, tail: Deref @ metamodelica::ListNode::Nil } => {
                eqn_ptr = (*__esc_eqn_ptr).clone();
                let __pa0 = ::match_deref::match_deref! { match &(Equation::getLHS(Pointer::access(eqn_ptr.clone()))?) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                new_exp = metamodelica::Own::own(__pa0);
                if Expression::isEnd(&exp) || Expression::isEqual(exp, new_exp.clone())? {
                    if (body.else_if).is_some() {
                        (new_exp, success) = getLHS(&(Util::getOption(body.else_if.clone())?), new_exp)?;
                    }
                } else {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.IfEquationBody.getLHS")); __mm_s.push_str(&*literal!(" failed because of ambiguous LHS for:\n")); __mm_s.push_str(&*toString(body, &(literal!("")), literal!(""), false)?); ArcStr::from(__mm_s) })?;
                    }
                    success = false;
                }
                new_exp
            },
            _ => {
                if Flags::isSet(Flags::FAILTRACE.clone())? {
                    Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.IfEquationBody.getLHS")); __mm_s.push_str(&*literal!(" failed because of un-split if-equation:\n")); __mm_s.push_str(&*toString(body, &(literal!("")), literal!(""), false)?); ArcStr::from(__mm_s) })?;
                }
                success = false;
                exp
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((exp, success))
    }

    pub(crate) fn getRHS(
        mut body: &metamodelica::Ref<IfEquationBody>,
    ) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> =
            openmodelica_nf_frontend::NFExpression::interned_END();
        let mut success: bool = false;
        let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
        let mut new_exp2: metamodelica::Ref<Expression::NFExpression>;
        exp = (::match_deref::match_deref! { match &(body.then_eqns.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __esc_eqn_ptr, tail: Deref @ metamodelica::ListNode::Nil } => {
                eqn_ptr = (*__esc_eqn_ptr).clone();
                let __pa0 = ::match_deref::match_deref! { match &(Equation::getRHS(Pointer::access(eqn_ptr.clone()))?) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                new_exp = metamodelica::Own::own(__pa0);
                if (body.else_if).is_some() {
                    (new_exp2, success) = getRHS(&(Util::getOption(body.else_if.clone())?))?;
                    if success {
                        new_exp = metamodelica::Ref::new(Expression::NFExpression::IF { ty: Expression::typeOf(new_exp.clone()), condition: body.condition.clone(), trueBranch: new_exp, falseBranch: new_exp2 });
                    } else {
                        new_exp = openmodelica_nf_frontend::NFExpression::interned_END();
                    }
                } else {
                    success = true;
                }
                new_exp
            },
            _ => {
                if Flags::isSet(Flags::FAILTRACE.clone())? {
                    Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.IfEquationBody.getRHS")); __mm_s.push_str(&*literal!(" failed because of un-split if-equation:\n")); __mm_s.push_str(&*toString(body, &(literal!("")), literal!(""), false)?); ArcStr::from(__mm_s) })?;
                }
                success = false;
                exp
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((exp, success))
    }

    pub(crate) fn split(
        mut body: metamodelica::Ref<IfEquationBody>,
    ) -> Result<metamodelica::List<metamodelica::Ref<IfEquationBody>>> {
        let mut bodies: metamodelica::List<metamodelica::Ref<IfEquationBody>> = metamodelica::nil();
        let mut conditions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        let mut s: i32 = ((body.then_eqns).len() as i32);
        let mut then_eqns: metamodelica::Array<
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        >;
        let mut condition: metamodelica::Ref<Expression::NFExpression>;
        let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        let mut tmp: Option<metamodelica::Ref<IfEquationBody>>;
        if isSplittable(&body, s)? {
            then_eqns = arrayCreate(s, metamodelica::nil());
            (conditions, then_eqns) = splitCollect(sortForSplit(body)?, conditions, then_eqns.clone())?;
            for mut i in 1..=metamodelica::arrayLength(then_eqns.clone()) {
                tmp = None;
                let __range0 = &*List::zip(
                    conditions.clone(),
                    ({
                        let __elt = (*metamodelica::index_checked(&then_eqns.borrow(), i)?).clone();
                        __elt
                    }),
                );
                for mut tpl in __range0 {
                    (condition, eqn) = tpl.clone();
                    tmp = Some(metamodelica::Ref::new(IfEquationBody {
                        condition: condition,
                        then_eqns: list![eqn],
                        else_if: tmp,
                    }));
                }
                bodies = metamodelica::cons(Util::getOption(tmp)?, bodies);
            }
        } else {
            bodies = list![body];
        }
        Ok(bodies)
    }

    pub(crate) fn isSplittable(mut body: &metamodelica::Ref<IfEquationBody>, mut s: i32) -> Result<bool> {
        let mut b: bool = ((body.then_eqns).len() as i32) == s;
        if b {
            b = Util::applyOptionOrDefault(
                body.else_if.clone(),
                &({
                    let __pe_b1 = s;
                    move |__pe_a0| isSplittable(&__pe_a0, __pe_b1.clone())
                }),
                true,
            )?;
        }
        Ok(b)
    }

    pub(crate) fn isSplit(mut body: &metamodelica::Ref<IfEquationBody>) -> Result<bool> {
        let mut b: bool = isSplittable(body, 1)?;
        Ok(b)
    }

    pub(crate) fn simplify(
        mut body: Option<metamodelica::Ref<IfEquationBody>>,
    ) -> Option<metamodelica::Ref<IfEquationBody>> {
        let mut body: Option<metamodelica::Ref<IfEquationBody>> = body;
        body = (::match_deref::match_deref! { match &(body.clone()) {
            Some(b) => {
                let mut b = (*b).clone();
                if Expression::isTrue(&b.condition) {
                    assign_field!(
                        b.condition = openmodelica_nf_frontend::NFExpression::interned_END(),
                        b.else_if = None
                    );
                } else {
                    assign_field!(b.else_if = simplify(b.else_if.clone()));
                }
                if Expression::isFalse(&b.condition) {
                    body = b.else_if.clone();
                } else {
                    body = Some(b.clone());
                }
                body
            },
            _ => {
                body
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        body
    }

    pub(crate) fn isRecordOrTupleEquation(mut body: &metamodelica::Ref<IfEquationBody>) -> Result<bool> {
        let mut b: bool;
        b = (::match_deref::match_deref! { match &(body.then_eqns.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: eqn_ptr, tail: Deref @ metamodelica::ListNode::Nil } => {
                Equation::isRecordOrTupleEquation(eqn_ptr.clone())?
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => {
                true
            },
            _ => {
                false
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(b)
    }

    pub(crate) fn getType(mut body: &metamodelica::Ref<IfEquationBody>) -> Result<metamodelica::Ref<Type::NFType>> {
        let mut ty: metamodelica::Ref<Type::NFType>;
        let mut body_types: metamodelica::List<metamodelica::Ref<Type::NFType>>;
        body_types = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
            for mut b in (body.then_eqns.clone()).into_iter().cloned() {
                let __x = Equation::getType(&(Pointer::access(b.clone())), false)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        ty = if (((body_types).len() as i32) == 1) {
            (body_types).head().cloned()?
        } else {
            metamodelica::Ref::new(Type::NFType::TUPLE {
                types: body_types,
                names: None,
            })
        };
        Ok(ty)
    }

    fn sortForSplit(mut body: metamodelica::Ref<IfEquationBody>) -> Result<metamodelica::Ref<IfEquationBody>> {
        fn compareLHS(
            mut eqn1: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
            mut eqn2: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        ) -> Result<bool> {
            let mut b: bool = 0 < Expression::compare(
                Util::getOption(Equation::getLHS(Pointer::access(eqn1.clone()))?)?,
                Util::getOption(Equation::getLHS(Pointer::access(eqn2.clone()))?)?,
            )?;
            Ok(b)
        }

        let mut body: metamodelica::Ref<IfEquationBody> = body;
        let mut discretes: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
        let mut continuous: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
        (discretes, continuous) = List::splitOnTrue(
            &body.then_eqns,
            &fnptr!(
                Equation::isDiscrete,
                Pointer::Pointer<metamodelica::Ref<Equation::Equation>>
            ),
        )?;
        discretes = List::sort(
            discretes,
            (std::sync::Arc::new(compareLHS)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                        ) -> Result<bool>
                        + 'static,
                >),
        )?;
        continuous = List::sort(
            continuous,
            (std::sync::Arc::new(compareLHS)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                        ) -> Result<bool>
                        + 'static,
                >),
        )?;
        assign_field!(
            body.then_eqns = listAppend(discretes, continuous),
            body.else_if = Util::applyOption(body.else_if.clone(), &sortForSplit)?
        );
        Ok(body)
    }

    fn splitCollect(
        mut body: metamodelica::Ref<IfEquationBody>,
        mut conditions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        mut then_eqns: metamodelica::Array<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        metamodelica::Array<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    )> {
        let mut conditions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = conditions;
        let mut then_eqns: metamodelica::Array<
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        > = then_eqns;
        let mut i: i32 = 1;
        conditions = metamodelica::cons(body.condition.clone(), conditions);
        for mut eqn in &*body.then_eqns.clone() {
            {
                let __cell0 = metamodelica::cons(
                    eqn.clone(),
                    ({
                        let __elt = (*metamodelica::index_checked(&then_eqns.borrow(), i)?).clone();
                        __elt
                    }),
                );
                let __idx0 = i;
                *metamodelica::index_mut_checked(&mut then_eqns.clone().borrow_mut(), __idx0)? = __cell0;
            }
            i = i + 1;
        }
        if (body.else_if).is_some() {
            (conditions, then_eqns) =
                splitCollect(Util::getOption(body.else_if.clone())?, conditions, then_eqns.clone())?;
        }
        Ok((conditions, then_eqns))
    }
}

pub mod WhenEquationBody {
    use super::*;
    /// equation when condition then cr = exp, reinit(...), terminate(...) or assert(...)
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct WhenEquationBody {
        /// the when-condition
        pub condition: metamodelica::Ref<Expression::NFExpression>,
        /// body statements
        pub when_stmts: metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>,
        /// optional elsewhen body
        pub else_when: Option<metamodelica::Ref<WhenEquationBody>>,
    }

    impl metamodelica::gc::MMTrace for WhenEquationBody {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.condition, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.when_stmts, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.else_when, __mmv)?;
            Ok(())
        }
    }
    impl Default for WhenEquationBody {
        fn default() -> Self {
            Self {
                condition: Default::default(),
                when_stmts: Default::default(),
                else_when: Default::default(),
            }
        }
    }

    pub type WHEN_EQUATION_BODY = WhenEquationBody;

    pub(crate) fn fromFlatList<'__b>(
        mut flat_list: &'__b metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>,
        )>,
        mut body: Option<metamodelica::Ref<WhenEquationBody>>,
    ) -> Option<metamodelica::Ref<WhenEquationBody>> {
        '__tco: loop {
            ::match_deref::match_deref! { match flat_list {
                Deref @ metamodelica::ListNode::Cons { head: (condition, stmts), tail: tail } => {
                    { (flat_list, body) = (tail, Some(metamodelica::Ref::new(WhenEquationBody { condition: condition.clone(), when_stmts: stmts.clone(), else_when: body }))); continue '__tco; }
                },
                _ => {
                    return body
                },
                _ => unreachable!("tail-call lowered match: no arm matched"),
            } }
        }
    }

    pub(crate) fn toString(
        mut body: &metamodelica::Ref<WhenEquationBody>,
        mut indent: &ArcStr,
        mut elseStr: ArcStr,
        mut selfCall: bool,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = elseStr;
        if !(selfCall) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*indent);
                ArcStr::from(__mm_s)
            };
        }
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("when "));
            __mm_s.push_str(&*Expression::toString(body.condition.clone())?);
            __mm_s.push_str(&*literal!(" then\n"));
            ArcStr::from(__mm_s)
        };
        for mut stmt in &*body.when_stmts.clone() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*WhenStatement::toString(metamodelica::AsArg::as_arg(&stmt), {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("  "));
                    ArcStr::from(__mm_s)
                })?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        if (body.else_when).is_some() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*toString(
                    &(Util::getOption(body.else_when.clone())?),
                    indent,
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*indent);
                        __mm_s.push_str(&*literal!("else"));
                        ArcStr::from(__mm_s)
                    },
                    true,
                )?);
                ArcStr::from(__mm_s)
            };
        }
        if !(selfCall) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*indent);
                __mm_s.push_str(&*literal!("end when;"));
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn size(mut body: &metamodelica::Ref<WhenEquationBody>, mut resize: bool) -> Result<i32> {
        let mut s: i32 = ({
            let mut __acc: i32 = 0;
            for mut stmt in (body.when_stmts.clone()).into_iter().cloned() {
                let __x = WhenStatement::size(&(stmt.clone()), resize)?;
                __acc += __x;
            }
            __acc
        });
        Ok(s)
    }

    pub(crate) fn getType(mut body: &metamodelica::Ref<WhenEquationBody>) -> Result<metamodelica::Ref<Type::NFType>> {
        let mut ty: metamodelica::Ref<Type::NFType>;
        ty = (::match_deref::match_deref! { match &(body.when_stmts.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: stmt, tail: Deref @ metamodelica::ListNode::Nil } => {
                WhenStatement::getType(metamodelica::AsArg::as_arg(&stmt))
            },
            _ if (List::all(&(({
            let mut __acc: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
            for mut st in (body.when_stmts.clone()).into_iter().cloned() {
                let __x = WhenStatement::getType(&(st.clone()));
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })), &move |__a0: metamodelica::Ref<Type::NFType>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Type::isAny(&__a0)) })?) => {
                openmodelica_nf_frontend::NFType::interned_ANY()
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.WhenEquationBody.getType")); __mm_s.push_str(&*literal!(" failed because of not properly split up when equation body: ")); __mm_s.push_str(&*toString(body, &(literal!("")), literal!(""), false)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(ty)
    }

    pub(crate) fn isEqual(
        mut body1: &metamodelica::Ref<WhenEquationBody>,
        mut body2: &metamodelica::Ref<WhenEquationBody>,
    ) -> Result<bool> {
        let mut b: bool;
        b = Expression::isEqual(body1.condition.clone(), body2.condition.clone())?
            && List::all(
                &({
                    let mut __acc: metamodelica::List<bool> = metamodelica::nil();
                    let __thr_src0 = body1.when_stmts.clone();
                    let mut __thr_it0 = (&__thr_src0).into_iter();
                    let __thr_src1 = body2.when_stmts.clone();
                    let mut __thr_it1 = (&__thr_src1).into_iter();
                    loop {
                        match (__thr_it0.next(), __thr_it1.next()) {
                            (Some(b1), Some(b2)) => {
                                let __x = WhenStatement::isEqual(&(b1.clone()), &(b2.clone()))?;
                                __acc = cons(__x, __acc);
                            }
                            (None, None) => break,
                            _ => return Err("threaded for: ranges of unequal length"),
                        }
                    }
                    __acc.reverse()
                }),
                &fnptr!(Util::id, _),
            )?
            && Util::optionEqual(
                body1.else_when.clone(),
                body2.else_when.clone(),
                &move |__a0: metamodelica::Ref<WhenEquationBody>, __a1: metamodelica::Ref<WhenEquationBody>| {
                    isEqual(&__a0, &__a1)
                },
            )?;
        Ok(b)
    }

    pub(crate) fn getBodyAttributes(
        mut body: &metamodelica::Ref<WhenEquationBody>,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>,
        Option<metamodelica::Ref<WhenEquationBody>>,
    )> {
        fn getConditions(
            mut cond: &metamodelica::Ref<Expression::NFExpression>,
        ) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
            let mut conditions: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            conditions = (match &**cond {
                Expression::CREF { cref, .. } => {
                    list![cref.clone()]
                }
                Expression::ARRAY { .. } => List::flatten(
                    ({
                        let mut __acc: metamodelica::List<_> = metamodelica::nil();
                        for mut elem in (var_field!((**cond).elements, Expression::NFExpression::ARRAY).clone())
                            .borrow()
                            .iter()
                        {
                            let __x = getConditions(&(elem.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                )?,
                Expression::CALL { call: __cond_call }
                    if (Call::isNamed(metamodelica::AsArg::as_arg(&__cond_call), &(literal!("initial")))?) =>
                {
                    metamodelica::nil()
                }
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!(
                                "NBEquation.WhenEquationBody.getBodyAttributes.getConditions"
                            ));
                            __mm_s.push_str(&*literal!(" failed for condition: "));
                            __mm_s.push_str(&*Expression::toString(cond.clone())?);
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            });
            Ok(conditions)
        }

        let mut conditions: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut when_stmts: metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>> =
            body.when_stmts.clone();
        let mut else_when: Option<metamodelica::Ref<WhenEquationBody>> = body.else_when.clone();
        conditions = getConditions(&body.condition)?;
        Ok((conditions, when_stmts, else_when))
    }

    pub(crate) fn toStatement(
        mut body: &metamodelica::Ref<WhenEquationBody>,
    ) -> Result<
        metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        )>,
    > {
        let mut stmts: metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        )>;
        let mut stmt: (
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        );
        stmt = (
            body.condition.clone(),
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
                for mut st in (body.when_stmts.clone()).into_iter().cloned() {
                    let __x = WhenStatement::toStatement(&(st.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        );
        if (body.else_when).is_some() {
            stmts = metamodelica::cons(stmt, toStatement(&(Util::getOption(body.else_when.clone())?))?);
        } else {
            stmts = list![stmt];
        }
        Ok(stmts)
    }

    pub(crate) fn map(
        mut whenBody: metamodelica::Ref<WhenEquationBody>,
        mut funcExp: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
        mut funcCrefOpt: Option<
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                    + 'static,
            >,
        >,
        mut mapFunc: Arc<
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
        >,
    ) -> Result<metamodelica::Ref<WhenEquationBody>> {
        let mut whenBody: metamodelica::Ref<WhenEquationBody> = whenBody;
        let mut condition: metamodelica::Ref<Expression::NFExpression>;
        condition = mapFunc(whenBody.condition.clone(), funcExp.clone())?;
        if !(referenceEq(&*(&*condition), &*(whenBody.condition.clone()))) {
            assign_field!(whenBody.condition = condition);
        }
        assign_field!(
            whenBody.when_stmts = List::map(
                whenBody.when_stmts.clone(),
                &({
                    let __pe_b1 = funcExp.clone();
                    let __pe_b2 = funcCrefOpt.clone();
                    let __pe_b3 = mapFunc.clone();
                    move |__pe_a0| WhenStatement::map(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &*__pe_b3)
                })
            )?,
            whenBody.else_when = Util::applyOption(
                whenBody.else_when.clone(),
                &({
                    let __pe_b1 = funcExp.clone();
                    let __pe_b2 = funcCrefOpt;
                    let __pe_b3 = mapFunc.clone();
                    move |__pe_a0| map(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
                })
            )?
        );
        Ok(whenBody)
    }

    pub(crate) fn mapCondition(
        mut whenBody: metamodelica::Ref<WhenEquationBody>,
        mut funcExp: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
        mut funcCrefOpt: Option<
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                    + 'static,
            >,
        >,
        mut mapFunc: Arc<
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
        >,
    ) -> Result<metamodelica::Ref<WhenEquationBody>> {
        let mut whenBody: metamodelica::Ref<WhenEquationBody> = whenBody;
        let mut condition: metamodelica::Ref<Expression::NFExpression>;
        condition = mapFunc(whenBody.condition.clone(), funcExp.clone())?;
        if !(referenceEq(&*(&*condition), &*(whenBody.condition.clone()))) {
            assign_field!(whenBody.condition = condition);
        }
        assign_field!(
            whenBody.else_when = Util::applyOption(
                whenBody.else_when.clone(),
                &({
                    let __pe_b1 = funcExp.clone();
                    let __pe_b2 = funcCrefOpt;
                    let __pe_b3 = mapFunc.clone();
                    move |__pe_a0| mapCondition(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
                })
            )?
        );
        Ok(whenBody)
    }

    pub(crate) fn split(
        mut body: metamodelica::Ref<WhenEquationBody>,
    ) -> Result<metamodelica::List<metamodelica::Ref<WhenEquationBody>>> {
        let mut bodies: metamodelica::List<metamodelica::Ref<WhenEquationBody>> = metamodelica::nil();
        let mut discr_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
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
        let mut state_set: metamodelica::Ref<
            UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        > = UnorderedSet::new(
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
        let mut discr_marks: metamodelica::Ref<
            UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        > = UnorderedSet::new(
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
        let mut flat_when: metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>,
        )>;
        let mut flat_new: metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>,
        )>;
        let mut discretes: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut states: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut set: CrefSet;
        let mut condition: metamodelica::Ref<Expression::NFExpression>;
        let mut acc_condition: metamodelica::Ref<Expression::NFExpression> =
            metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                ty: openmodelica_nf_frontend::NFType::interned_INTEGER(),
            });
        let mut stmts: metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>;
        let mut assigns: metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>;
        let mut stmt: Option<metamodelica::Ref<WhenStatement::WhenStatement>> = None;
        let mut new_body: Option<metamodelica::Ref<WhenEquationBody>>;
        flat_when = collectForSplit(Some(body), discr_map.clone(), state_set.clone())?;
        discretes = UnorderedMap::keyList(discr_map.clone());
        states = UnorderedSet::toList(state_set);
        for mut disc in &*discretes {
            if !(UnorderedSet::contains(disc.clone(), discr_marks.clone())?) {
                set = UnorderedMap::getSafe(
                    disc.clone(),
                    discr_map.clone(),
                    metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo"),
                )?;
                for mut marked in &*UnorderedSet::toList(set.clone()) {
                    UnorderedSet::add(marked.clone(), discr_marks.clone())?;
                }
                flat_new = metamodelica::nil();
                for mut tpl in &*flat_when {
                    (condition, stmts) = tpl.clone();
                    assigns = getAssignments(set.clone(), &stmts)?;
                    if !((assigns).is_empty()) {
                        condition = combineConditions(acc_condition, condition, false);
                        acc_condition = metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                            ty: openmodelica_nf_frontend::NFType::interned_INTEGER(),
                        });
                        flat_new = metamodelica::cons((condition, assigns), flat_new);
                    } else {
                        acc_condition = combineConditions(acc_condition, condition, true);
                    }
                }
                new_body = fromFlatList(&flat_new, None);
                if (new_body).is_some() {
                    bodies = metamodelica::cons(Util::getOption(new_body)?, bodies);
                } else {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NBEquation.WhenEquationBody.split"));
                            __mm_s.push_str(&*literal!(" failed because when partition for: "));
                            __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&disc))?);
                            __mm_s.push_str(&*literal!(" could not be recovered."));
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                }
            }
        }
        for mut state in &*states {
            flat_new = metamodelica::nil();
            for mut tpl in &*flat_when {
                (condition, stmts) = tpl.clone();
                stmt = getFirstReinit(metamodelica::AsArg::as_arg(&state), &stmts)?;
                if (stmt).is_some() {
                    condition = combineConditions(acc_condition, condition, false);
                    acc_condition = metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                        ty: openmodelica_nf_frontend::NFType::interned_INTEGER(),
                    });
                    flat_new = metamodelica::cons((condition, list![Util::getOption(stmt)?]), flat_new);
                } else {
                    acc_condition = combineConditions(acc_condition, condition, true);
                }
            }
            new_body = fromFlatList(&flat_new, None);
            if (new_body).is_some() {
                bodies = metamodelica::cons(Util::getOption(new_body)?, bodies);
            } else {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.WhenEquationBody.split"));
                        __mm_s.push_str(&*literal!(" failed because when partition for: "));
                        __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&state))?);
                        __mm_s.push_str(&*literal!(" could not be recovered."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
            }
        }
        flat_new = metamodelica::nil();
        for mut tpl in &*flat_when {
            (condition, stmts) = tpl.clone();
            stmts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>> =
                    metamodelica::nil();
                for mut stmt in (stmts).into_iter().cloned() {
                    if !(!(WhenStatement::isAssignOrReinit(&stmt))) {
                        continue;
                    }
                    let __x = stmt.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            if !((stmts).is_empty()) {
                condition = combineConditions(acc_condition, condition, false);
                acc_condition = metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                    ty: openmodelica_nf_frontend::NFType::interned_INTEGER(),
                });
                flat_new = metamodelica::cons((condition, stmts), flat_new);
                new_body = fromFlatList(&flat_new, None);
                if (new_body).is_some() {
                    bodies = metamodelica::cons(Util::getOption(new_body)?, bodies);
                }
            } else {
                acc_condition = combineConditions(acc_condition, condition, true);
            }
        }
        bodies = bodies.reverse();
        Ok(bodies)
    }

    pub(crate) fn simplify(
        mut body: Option<metamodelica::Ref<WhenEquationBody>>,
    ) -> Result<Option<metamodelica::Ref<WhenEquationBody>>> {
        let mut body: Option<metamodelica::Ref<WhenEquationBody>> = body;
        body = (::match_deref::match_deref! { match &(body.clone()) {
            Some(b @ Deref @ WhenEquationBody { condition: condition @ Deref @ Expression::ARRAY { .. }, .. }) => {
                let mut conditions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                let mut b = (*b).clone();
                assign_field!(b.else_when = simplify(b.else_when.clone())?);
                conditions = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut elem in (var_field!((**condition).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
                if !(!(Expression::isBoolean(&(elem.clone())))) { continue; }
                let __x = elem.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                if (conditions).is_empty() {
                    body = b.else_when.clone();
                } else if List::hasOneElement(&conditions) {
                    assign_field!(b.condition = (conditions).head().cloned()?);
                    body = Some(b.clone());
                } else {
                    assign_field!(b.condition = Expression::makeArrayCheckLiteral(metamodelica::Ref::new(Type::NFType::ARRAY { elementType: openmodelica_nf_frontend::NFType::interned_BOOLEAN(), dimensions: list![Dimension::fromInteger(((conditions).len() as i32), Variability::CONSTANT.clone())] }), metamodelica::arrayFromVec(conditions.into_iter().cloned().collect()))?);
                    body = Some(b.clone());
                }
                body
            },
            Some(b) => {
                let mut b = (*b).clone();
                assign_field!(b.else_when = simplify(b.else_when.clone())?);
                if Expression::isBoolean(&b.condition) {
                    body = b.else_when.clone();
                } else {
                    body = Some(b.clone());
                }
                body
            },
            _ => {
                body
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(body)
    }

    pub(crate) fn getAllAssigned(
        mut body: &metamodelica::Ref<WhenEquationBody>,
    ) -> metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> {
        let mut assigned: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        for mut stmt in &*body.when_stmts.clone() {
            assigned = (::match_deref::match_deref! { match &(stmt.clone()) {
                Deref @ WhenStatement::ASSIGN { lhs: Deref @ Expression::CREF { cref: lhs, .. }, .. } => {
                    metamodelica::cons(lhs.clone(), assigned)
                },
                _ => {
                    assigned
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        assigned
    }

    pub(crate) fn isRecordOrTupleEquation(mut body: &metamodelica::Ref<WhenEquationBody>) -> Result<bool> {
        let mut b: bool;
        b = (::match_deref::match_deref! { match &(body.when_stmts.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ WhenStatement::ASSIGN { lhs: Deref @ Expression::TUPLE { .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
                true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ WhenStatement::ASSIGN { lhs: Deref @ Expression::RECORD { .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
                true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ WhenStatement::ASSIGN { lhs: Deref @ Expression::CREF { cref, .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
                BVariable::checkCref(metamodelica::AsArg::as_arg(&cref), &fnptr!(BVariable::isRecord, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>), metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo"))?
            },
            _ if (List::count(&body.when_stmts, &move |__a0: metamodelica::Ref<WhenStatement::WhenStatement>| -> metamodelica::Result<_> { ::std::result::Result::Ok(WhenStatement::isAssign(&__a0)) })? > 1) => {
                true
            },
            _ => {
                false
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(b)
    }

    pub type CrefSet = metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;

    fn collectForSplit(
        mut body_opt: Option<metamodelica::Ref<WhenEquationBody>>,
        mut discr_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
            >,
        >,
        mut state_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<
        metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>,
        )>,
    > {
        let mut flat_when: metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>,
        )>;
        let mut body: metamodelica::Ref<WhenEquationBody>;
        if (body_opt).is_some() {
            body = Util::getOption(body_opt)?;
            for mut stmt in &*body.when_stmts.clone() {
                let () = (::match_deref::match_deref! { match &(stmt.clone()) {
                    Deref @ WhenStatement::ASSIGN { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
                        addCrefsMap(discr_map.clone(), &(list![cref.clone()]))?;
                        ()
                    },
                    Deref @ WhenStatement::ASSIGN { lhs: tpl @ Deref @ Expression::TUPLE { .. }, .. } => {
                        addCrefsMap(discr_map.clone(), &(UnorderedSet::toList(Expression::extractCrefs(tpl.clone())?)))?;
                        ()
                    },
                    Deref @ WhenStatement::REINIT { stateVar: cref, .. } => {
                        UnorderedSet::add(cref.clone(), state_set.clone())?;
                        ()
                    },
                    Deref @ WhenStatement::ASSIGN { .. } => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.WhenEquationBody.collectForSplit")); __mm_s.push_str(&*literal!(" failed because lhs of statement is not a cref: ")); __mm_s.push_str(&*WhenStatement::toString(metamodelica::AsArg::as_arg(&stmt), literal!(""))?); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => {
                        ()
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
            flat_when = metamodelica::cons(
                (body.condition.clone(), body.when_stmts.clone()),
                collectForSplit(body.else_when.clone(), discr_map, state_set)?,
            );
        } else {
            flat_when = metamodelica::nil();
        }
        Ok(flat_when)
    }

    fn addCrefsMap(
        mut discr_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
            >,
        >,
        mut crefs: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    ) -> Result<()> {
        let mut set_new: CrefSet;
        let mut set: CrefSet = UnorderedSet::new(
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
        for mut c in &**crefs {
            if UnorderedMap::contains(c.clone(), discr_map.clone())? {
                set_new = UnorderedMap::getSafe(
                    c.clone(),
                    discr_map.clone(),
                    metamodelica::sourceInfo!("NBackEnd/Classes/NBEquation.mo"),
                )?;
                if !(referenceEq(&*(&*set), &*(&*set_new))) {
                    set = UnorderedSet::union(set, set_new)?;
                }
            } else {
                UnorderedSet::add(c.clone(), set.clone())?;
            }
        }
        for mut c in &**crefs {
            UnorderedMap::add(c.clone(), set.clone(), discr_map.clone())?;
        }
        Ok(())
    }

    fn getAssignments(
        mut crefSet: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        mut stmts: &metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>,
    ) -> Result<metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>> {
        let mut assigns: metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>> = metamodelica::nil();
        for mut stmt in &**stmts {
            let () = (::match_deref::match_deref! { match &(stmt.clone()) {
                Deref @ WhenStatement::ASSIGN { lhs: Deref @ Expression::CREF { cref, .. }, .. } if (UnorderedSet::contains(cref.clone(), crefSet.clone())?) => {
                    assigns = metamodelica::cons(stmt.clone(), assigns);
                    ()
                },
                Deref @ WhenStatement::ASSIGN { lhs: tpl @ Deref @ Expression::TUPLE { .. }, .. } if (List::any(&(({
                let mut __acc: metamodelica::List<bool> = metamodelica::nil();
                for mut c in (UnorderedSet::toList(Expression::extractCrefs(tpl.clone())?)).into_iter().cloned() {
                    let __x = UnorderedSet::contains(c.clone(), crefSet.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })), &fnptr!(Util::id, _))?) => {
                    assigns = metamodelica::cons(stmt.clone(), assigns);
                    ()
                },
                _ => {
                    ()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        Ok(assigns)
    }

    fn getFirstReinit(
        mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut stmts: &metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>,
    ) -> Result<Option<metamodelica::Ref<WhenStatement::WhenStatement>>> {
        let mut assign: Option<metamodelica::Ref<WhenStatement::WhenStatement>> = None;
        for mut stmt in &**stmts {
            let () = (match &*stmt.clone() {
                WhenStatement::REINIT {
                    stateVar: __stmt_stateVar,
                    ..
                } if (ComponentRef::isEqual(cref, metamodelica::AsArg::as_arg(&__stmt_stateVar))?) => {
                    assign = Some(stmt.clone());
                    break;
                    ()
                }
                _ => (),
            });
        }
        Ok(assign)
    }

    fn combineConditions(
        mut acc_condition: metamodelica::Ref<Expression::NFExpression>,
        mut condition: metamodelica::Ref<Expression::NFExpression>,
        mut invert: bool,
    ) -> metamodelica::Ref<Expression::NFExpression> {
        let mut condition: metamodelica::Ref<Expression::NFExpression> = condition;
        if invert {
            condition = Expression::logicNegate(condition);
        }
        if !(Expression::isEmpty(&acc_condition)) {
            condition = metamodelica::Ref::new(Expression::NFExpression::LBINARY {
                exp1: acc_condition,
                operator: Operator::makeAnd(openmodelica_nf_frontend::NFType::interned_BOOLEAN()),
                exp2: condition,
            });
        }
        condition
    }
}

pub mod WhenStatement {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum WhenStatement {
        /// left_cr = right_exp
        ASSIGN {
            /// left hand side of assignment
            lhs: metamodelica::Ref<Expression::NFExpression>,
            /// right hand side of assignment
            rhs: metamodelica::Ref<Expression::NFExpression>,
            /// origin of assignment
            source: metamodelica::Ref<DAE::ElementSource>,
        },
        /// Reinit Statement
        REINIT {
            /// State variable to reinit
            stateVar: metamodelica::Ref<ComponentRef::NFComponentRef>,
            /// Value after reinit
            value: metamodelica::Ref<Expression::NFExpression>,
            /// origin of statement
            source: metamodelica::Ref<DAE::ElementSource>,
        },
        ASSERT {
            condition: metamodelica::Ref<Expression::NFExpression>,
            message: metamodelica::Ref<Expression::NFExpression>,
            level: metamodelica::Ref<Expression::NFExpression>,
            /// origin of statement
            source: metamodelica::Ref<DAE::ElementSource>,
        },
        /// The Modelica built-in terminate(msg)
        TERMINATE {
            message: metamodelica::Ref<Expression::NFExpression>,
            /// the origin of the component/equation/algorithm
            source: metamodelica::Ref<DAE::ElementSource>,
        },
        /// call with no return value, i.e. no equation.
        ///      Typically side effect call of external function but also
        ///      Connections.* i.e. Connections.root(...) functions.
        NORETCALL {
            exp: metamodelica::Ref<Expression::NFExpression>,
            /// the origin of the component/equation/algorithm
            source: metamodelica::Ref<DAE::ElementSource>,
        },
    }
    impl metamodelica::gc::MMTrace for WhenStatement {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                WhenStatement::ASSIGN { lhs, rhs, source } => {
                    metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    Ok(())
                }
                WhenStatement::REINIT {
                    stateVar,
                    value,
                    source,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(stateVar, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    Ok(())
                }
                WhenStatement::ASSERT {
                    condition,
                    message,
                    level,
                    source,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(level, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    Ok(())
                }
                WhenStatement::TERMINATE { message, source } => {
                    metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    Ok(())
                }
                WhenStatement::NORETCALL { exp, source } => {
                    metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for WhenStatement {
        fn default() -> Self {
            Self::TERMINATE {
                message: Default::default(),
                source: Default::default(),
            }
        }
    }
    pub use self::WhenStatement::{ASSERT, ASSIGN, NORETCALL, REINIT, TERMINATE};
    pub(crate) fn toString(mut stmt: &metamodelica::Ref<WhenStatement>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = (match &**stmt {
            ASSIGN { lhs, rhs, .. } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*Expression::toString(lhs.clone())?);
                __mm_s.push_str(&*literal!(" := "));
                __mm_s.push_str(&*Expression::toString(rhs.clone())?);
                ArcStr::from(__mm_s)
            }
            REINIT { stateVar, value, .. } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("reinit("));
                __mm_s.push_str(&*ComponentRef::toString(stateVar)?);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*Expression::toString(value.clone())?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
            ASSERT {
                condition,
                message,
                level,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("assert("));
                __mm_s.push_str(&*Expression::toString(condition.clone())?);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*Expression::toString(message.clone())?);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*Expression::toString(level.clone())?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
            TERMINATE { message, .. } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("terminate("));
                __mm_s.push_str(&*Expression::toString(message.clone())?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
            NORETCALL { exp: value, .. } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*Expression::toString(value.clone())?);
                ArcStr::from(__mm_s)
            }
            _ => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("NBEquation.WhenStatement.toString"));
                __mm_s.push_str(&*literal!(" failed."));
                ArcStr::from(__mm_s)
            }
        });
        Ok(r#str)
    }

    pub(crate) fn isEqualTpl(
        mut tpl: &(metamodelica::Ref<WhenStatement>, metamodelica::Ref<WhenStatement>),
    ) -> Result<bool> {
        let mut b: bool;
        let mut stmt1: metamodelica::Ref<WhenStatement>;
        let mut stmt2: metamodelica::Ref<WhenStatement>;
        (stmt1, stmt2) = tpl.clone();
        b = isEqual(&stmt1, &stmt2)?;
        Ok(b)
    }

    pub(crate) fn isEqual(
        mut stmt1: &metamodelica::Ref<WhenStatement>,
        mut stmt2: &metamodelica::Ref<WhenStatement>,
    ) -> Result<bool> {
        let mut b: bool;
        b = (::match_deref::match_deref! { match (stmt1, stmt2) {
            (Deref @ ASSIGN { .. }, Deref @ ASSIGN { .. }) => Expression::isEqual(var_field!((**stmt1).lhs, WhenStatement::ASSIGN).clone(), var_field!((**stmt2).lhs, WhenStatement::ASSIGN).clone())? && Expression::isEqual(var_field!((**stmt1).rhs, WhenStatement::ASSIGN).clone(), var_field!((**stmt2).rhs, WhenStatement::ASSIGN).clone())?,
            (Deref @ REINIT { .. }, Deref @ REINIT { .. }) => ComponentRef::isEqual(var_field!((**stmt1).stateVar, WhenStatement::REINIT), var_field!((**stmt2).stateVar, WhenStatement::REINIT))? && Expression::isEqual(var_field!((**stmt1).value, WhenStatement::REINIT).clone(), var_field!((**stmt2).value, WhenStatement::REINIT).clone())?,
            (Deref @ ASSERT { .. }, Deref @ ASSERT { .. }) => Expression::isEqual(var_field!((**stmt1).condition, WhenStatement::ASSERT).clone(), var_field!((**stmt2).condition, WhenStatement::ASSERT).clone())? && Expression::isEqual(var_field!((**stmt1).message, WhenStatement::ASSERT).clone(), var_field!((**stmt2).message, WhenStatement::ASSERT).clone())? && Expression::isEqual(var_field!((**stmt1).level, WhenStatement::ASSERT).clone(), var_field!((**stmt2).level, WhenStatement::ASSERT).clone())?,
            (Deref @ TERMINATE { .. }, Deref @ TERMINATE { .. }) => Expression::isEqual(var_field!((**stmt1).message, WhenStatement::TERMINATE).clone(), var_field!((**stmt2).message, WhenStatement::TERMINATE).clone())?,
            (Deref @ NORETCALL { .. }, Deref @ NORETCALL { .. }) => Expression::isEqual(var_field!((**stmt1).exp, WhenStatement::NORETCALL).clone(), var_field!((**stmt2).exp, WhenStatement::NORETCALL).clone())?,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(b)
    }

    pub(crate) fn toStatement(
        mut wstmt: &metamodelica::Ref<WhenStatement>,
    ) -> Result<metamodelica::Ref<Statement::NFStatement>> {
        let mut stmt: metamodelica::Ref<Statement::NFStatement>;
        stmt = (match &**wstmt {
            ASSIGN {
                lhs: __wstmt_lhs,
                rhs: __wstmt_rhs,
                source: __wstmt_source,
            } => metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT {
                lhs: __wstmt_lhs.clone(),
                rhs: __wstmt_rhs.clone(),
                ty: Expression::typeOf(__wstmt_lhs.clone()),
                source: __wstmt_source.clone(),
            }),
            REINIT {
                source: __wstmt_source,
                stateVar: __wstmt_stateVar,
                value: __wstmt_value,
            } => metamodelica::Ref::new(Statement::NFStatement::REINIT {
                cref: Expression::fromCref(__wstmt_stateVar.clone(), false)?,
                reinitExp: __wstmt_value.clone(),
                source: __wstmt_source.clone(),
            }),
            ASSERT {
                condition: __wstmt_condition,
                level: __wstmt_level,
                message: __wstmt_message,
                source: __wstmt_source,
            } => metamodelica::Ref::new(Statement::NFStatement::ASSERT {
                condition: __wstmt_condition.clone(),
                message: __wstmt_message.clone(),
                level: __wstmt_level.clone(),
                source: __wstmt_source.clone(),
            }),
            TERMINATE {
                message: __wstmt_message,
                source: __wstmt_source,
            } => metamodelica::Ref::new(Statement::NFStatement::TERMINATE {
                message: __wstmt_message.clone(),
                source: __wstmt_source.clone(),
            }),
            NORETCALL {
                exp: __wstmt_exp,
                source: __wstmt_source,
            } => metamodelica::Ref::new(Statement::NFStatement::NORETCALL {
                exp: __wstmt_exp.clone(),
                source: __wstmt_source.clone(),
            }),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.WhenStatement.toStatement"));
                        __mm_s.push_str(&*literal!(" failed because of unrecognized statement: "));
                        __mm_s.push_str(&*toString(wstmt, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(stmt)
    }

    pub(crate) fn toEquation(
        mut stmt: &metamodelica::Ref<WhenStatement>,
        mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        mut init: bool,
    ) -> Result<metamodelica::Ref<Equation::Equation>> {
        let mut eqn: metamodelica::Ref<Equation::Equation>;
        eqn = (match &**stmt {
            ASSIGN {
                lhs: __stmt_lhs,
                rhs: __stmt_rhs,
                ..
            } => Equation::makeAssignmentEqn(
                __stmt_lhs.clone(),
                __stmt_rhs.clone(),
                crate::NBEquation::Iterator::interned_EMPTY(),
                attr,
            )?,
            _ => Equation::setAttributes(
                Pointer::access(Equation::makeAlgorithm(list![toStatement(stmt)?], init)?),
                &attr,
            )?,
        });
        Ok(eqn)
    }

    pub(crate) fn size(mut stmt: &metamodelica::Ref<WhenStatement>, mut resize: bool) -> Result<i32> {
        let mut s: i32;
        s = (match &**stmt {
            ASSIGN { lhs: __stmt_lhs, .. } => Type::sizeOf(&(Expression::typeOf(__stmt_lhs.clone())), resize)?,
            _ => 0,
        });
        Ok(s)
    }

    pub(crate) fn isAssign(mut stmt: &metamodelica::Ref<WhenStatement>) -> bool {
        let mut b: bool;
        b = (match &**stmt {
            ASSIGN { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn isAssignOrReinit(mut stmt: &metamodelica::Ref<WhenStatement>) -> bool {
        let mut b: bool;
        b = (match &**stmt {
            ASSIGN { .. } => true,
            REINIT { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn getType(mut stmt: &metamodelica::Ref<WhenStatement>) -> metamodelica::Ref<Type::NFType> {
        let mut ty: metamodelica::Ref<Type::NFType>;
        ty = (match &**stmt {
            ASSIGN { lhs: __stmt_lhs, .. } => Expression::typeOf(__stmt_lhs.clone()),
            _ => openmodelica_nf_frontend::NFType::interned_ANY(),
        });
        ty
    }

    pub(crate) fn map(
        mut stmt: metamodelica::Ref<WhenStatement>,
        mut funcExp: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
        mut funcCrefOpt: Option<
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                    + 'static,
            >,
        >,
        mut mapFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Expression::NFExpression>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
    ) -> Result<metamodelica::Ref<WhenStatement>> {
        let mut stmt: metamodelica::Ref<WhenStatement> = stmt;
        stmt = (match &*stmt {
            ASSIGN {
                lhs: __stmt_lhs,
                rhs: __stmt_rhs,
                ..
            } => {
                let mut lhs: metamodelica::Ref<Expression::NFExpression>;
                let mut rhs: metamodelica::Ref<Expression::NFExpression>;
                lhs = mapFunc(__stmt_lhs.clone(), funcExp.clone())?;
                rhs = mapFunc(__stmt_rhs.clone(), funcExp.clone())?;
                if !(referenceEq(&*(&*lhs), &*(var_field!((*stmt).lhs, WhenStatement::ASSIGN).clone()))) {
                    assign_variant_field!(stmt => WhenStatement::ASSIGN; lhs = lhs);
                }
                if !(referenceEq(&*(&*rhs), &*(var_field!((*stmt).rhs, WhenStatement::ASSIGN).clone()))) {
                    assign_variant_field!(stmt => WhenStatement::ASSIGN; rhs = rhs);
                }
                stmt
            }
            REINIT { .. } => {
                let mut funcCref: MapFuncCref;
                let mut value: metamodelica::Ref<Expression::NFExpression>;
                let mut stateVar: metamodelica::Ref<ComponentRef::NFComponentRef>;
                if (funcCrefOpt).is_some() {
                    let __pa0 = ::match_deref::match_deref! { match &(funcCrefOpt) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    funcCref = metamodelica::Own::own(__pa0);
                    stateVar = funcCref(var_field!((*stmt).stateVar, WhenStatement::REINIT).clone())?;
                    if !(referenceEq(
                        &*(&*stateVar),
                        &*(var_field!((*stmt).stateVar, WhenStatement::REINIT).clone()),
                    )) {
                        assign_variant_field!(stmt => WhenStatement::REINIT; stateVar = stateVar);
                    }
                }
                value = mapFunc(
                    var_field!((*stmt).value, WhenStatement::REINIT).clone(),
                    funcExp.clone(),
                )?;
                if !(referenceEq(
                    &*(&*value),
                    &*(var_field!((*stmt).value, WhenStatement::REINIT).clone()),
                )) {
                    assign_variant_field!(stmt => WhenStatement::REINIT; value = value);
                }
                stmt
            }
            ASSERT {
                condition: __stmt_condition,
                ..
            } => {
                let mut condition: metamodelica::Ref<Expression::NFExpression>;
                let mut message: metamodelica::Ref<Expression::NFExpression>;
                condition = mapFunc(__stmt_condition.clone(), funcExp.clone())?;
                if !(referenceEq(
                    &*(&*condition),
                    &*(var_field!((*stmt).condition, WhenStatement::ASSERT).clone()),
                )) {
                    assign_variant_field!(stmt => WhenStatement::ASSERT; condition = condition);
                }
                message = mapFunc(
                    var_field!((*stmt).message, WhenStatement::ASSERT).clone(),
                    funcExp.clone(),
                )?;
                if !(referenceEq(
                    &*(&*message),
                    &*(var_field!((*stmt).message, WhenStatement::ASSERT).clone()),
                )) {
                    assign_variant_field!(stmt => WhenStatement::ASSERT; message = message);
                }
                stmt
            }
            TERMINATE { .. } => stmt,
            NORETCALL { exp: __stmt_exp, .. } => {
                let mut value: metamodelica::Ref<Expression::NFExpression>;
                value = mapFunc(__stmt_exp.clone(), funcExp.clone())?;
                if !(referenceEq(
                    &*(&*value),
                    &*(var_field!((*stmt).exp, WhenStatement::NORETCALL).clone()),
                )) {
                    assign_variant_field!(stmt => WhenStatement::NORETCALL; exp = value);
                }
                stmt
            }
            _ => stmt,
        });
        Ok(stmt)
    }

    pub(crate) fn convert(mut stmt: &metamodelica::Ref<WhenStatement>) -> Result<OldBackendDAE::WhenOperator> {
        let mut oldStmt: OldBackendDAE::WhenOperator;
        oldStmt = (match &**stmt {
            ASSIGN {
                lhs: __stmt_lhs,
                rhs: __stmt_rhs,
                source: __stmt_source,
            } => OldBackendDAE::WhenOperator::ASSIGN {
                left: Expression::toDAE(__stmt_lhs.clone(), false)?,
                right: Expression::toDAE(__stmt_rhs.clone(), false)?,
                source: __stmt_source.clone(),
            },
            REINIT {
                source: __stmt_source,
                stateVar: __stmt_stateVar,
                value: __stmt_value,
            } => OldBackendDAE::WhenOperator::REINIT {
                stateVar: ComponentRef::toDAE(metamodelica::AsArg::as_arg(&__stmt_stateVar))?,
                value: Expression::toDAE(__stmt_value.clone(), false)?,
                source: __stmt_source.clone(),
            },
            ASSERT {
                condition: __stmt_condition,
                level: __stmt_level,
                message: __stmt_message,
                source: __stmt_source,
            } => OldBackendDAE::WhenOperator::ASSERT {
                condition: Expression::toDAE(__stmt_condition.clone(), false)?,
                message: Expression::toDAE(__stmt_message.clone(), false)?,
                level: Expression::toDAE(__stmt_level.clone(), false)?,
                source: __stmt_source.clone(),
            },
            TERMINATE {
                message: __stmt_message,
                source: __stmt_source,
            } => OldBackendDAE::WhenOperator::TERMINATE {
                message: Expression::toDAE(__stmt_message.clone(), false)?,
                source: __stmt_source.clone(),
            },
            NORETCALL {
                exp: __stmt_exp,
                source: __stmt_source,
            } => OldBackendDAE::WhenOperator::NORETCALL {
                exp: Expression::toDAE(__stmt_exp.clone(), false)?,
                source: __stmt_source.clone(),
            },
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.WhenStatement.convert"));
                        __mm_s.push_str(&*literal!(" failed because of unrecognized statement: "));
                        __mm_s.push_str(&*toString(stmt, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(oldStmt)
    }
}

pub mod EquationAttributes {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct EquationAttributes {
        /// if the equation has been differentiated w.r.t time already
        pub derivative: Option<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        /// also used to represent the equation itself
        pub residualVar: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        /// only set if clocked eq
        pub clock_idx: Option<i32>,
        /// true if in residual form
        pub residual: bool,
        /// true if in initial equation block
        pub exclusively_initial: bool,
        /// evaluation stages (prior used for DAE mode, still necessary?)
        pub evalStages: metamodelica::Ref<Evaluation::Stages::Stages>,
        /// continuous, clocked, discrete, empty
        pub kind: EquationKind,
        /// dynamic optimization component: Mayer, Lagrange, Path, Boundary
        pub optimizerExpression: Option<OptimizerExpression>,
    }

    impl metamodelica::gc::MMTrace for EquationAttributes {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.derivative, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.residualVar, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.clock_idx, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.residual, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.exclusively_initial, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.evalStages, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.kind, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.optimizerExpression, __mmv)?;
            Ok(())
        }
    }
    impl Default for EquationAttributes {
        fn default() -> Self {
            Self {
                derivative: Default::default(),
                residualVar: Default::default(),
                clock_idx: Default::default(),
                residual: Default::default(),
                exclusively_initial: Default::default(),
                evalStages: Default::default(),
                kind: Default::default(),
                optimizerExpression: Default::default(),
            }
        }
    }

    pub type EQUATION_ATTRIBUTES = EquationAttributes;

    pub(crate) fn toString(mut attr: &metamodelica::Ref<EquationAttributes>, mut indent: &ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (match &**attr {
            EquationAttributes {
                residualVar: Some(residualVar),
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*indent);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*ComponentRef::toString(&(BVariable::getVarName(residualVar.clone())))?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
            _ => {
                literal!("")
            }
        });
        Ok(r#str)
    }

    pub(crate) fn setKind(
        mut attr: metamodelica::Ref<EquationAttributes>,
        mut kind: EquationKind,
        mut clock_idx: Option<i32>,
    ) -> metamodelica::Ref<EquationAttributes> {
        let mut attr: metamodelica::Ref<EquationAttributes> = attr;
        assign_field!(attr.kind = kind, attr.clock_idx = clock_idx);
        attr
    }

    pub(crate) fn setResidualVar(
        mut attr: metamodelica::Ref<EquationAttributes>,
        mut residualVar: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    ) -> metamodelica::Ref<EquationAttributes> {
        let mut attr: metamodelica::Ref<EquationAttributes> = attr;
        assign_field!(attr.residualVar = Some(residualVar));
        attr
    }

    pub(crate) fn getResidualVar(
        mut attr: &metamodelica::Ref<EquationAttributes>,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
        let mut residualVar: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        match '__try0: {
            let __pa1 = ::match_deref::match_deref! { match &(attr.residualVar.clone()) {
                Some(__pa1) => __pa1.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            residualVar = metamodelica::Own::own(__pa1);
            Ok::<_, &'static str>((residualVar.clone(),))
        } {
            Ok((__try0_o0,)) => {
                residualVar = __try0_o0;
            }
            Err(__try0_err) => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.EquationAttributes.getResidualVar"));
                        __mm_s.push_str(&*literal!(" failed because of missing residualVar!"));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err(__try0_err);
            }
        }
        Ok(residualVar)
    }

    pub(crate) fn convert(
        mut attributes: &metamodelica::Ref<EquationAttributes>,
    ) -> Result<OldBackendDAE::EquationAttributes> {
        let mut oldAttributes: OldBackendDAE::EquationAttributes;
        oldAttributes = OldBackendDAE::EquationAttributes {
            differentiated: (attributes.derivative).is_some(),
            kind: convertEquationKind(
                attributes.kind.clone(),
                attributes.clock_idx.clone(),
                attributes.exclusively_initial.clone(),
            )?,
            evalStages: Evaluation::Stages::convert(&attributes.evalStages),
        };
        Ok(oldAttributes)
    }
}

pub(crate) fn default(
    mut kind: EquationKind,
    mut exclusively_initial: bool,
    mut clock_idx: Option<i32>,
    mut optimizerExpression: Option<OptimizerExpression>,
) -> metamodelica::Ref<EquationAttributes::EquationAttributes> {
    let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
    attr = metamodelica::Ref::new(EquationAttributes::EquationAttributes {
        derivative: None,
        residualVar: None,
        clock_idx: clock_idx,
        residual: false,
        exclusively_initial: exclusively_initial,
        evalStages: Evaluation::DEFAULT_STAGES.clone(),
        kind: kind,
        optimizerExpression: optimizerExpression,
    });
    attr
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum EquationKind {
    CONTINUOUS = 1,
    DISCRETE = 2,
    CLOCKED = 3,
    EMPTY = 4,
    UNKNOWN = 5,
}
impl PartialOrd for EquationKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for EquationKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for EquationKind {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for EquationKind {
    fn default() -> Self {
        Self::CONTINUOUS
    }
}

pub(crate) fn convertEquationKind(
    mut eqKind: EquationKind,
    mut clock_idx: Option<i32>,
    mut exclusively_initial: bool,
) -> Result<OldBackendDAE::EquationKind> {
    let mut oldEqKind: OldBackendDAE::EquationKind;
    oldEqKind = (match (eqKind, clock_idx.clone()) {
        (_, _) if (exclusively_initial) => openmodelica_backend_types::BackendDAE::EquationKind::INITIAL_EQUATION,
        (EquationKind::CONTINUOUS { .. }, None) => {
            openmodelica_backend_types::BackendDAE::EquationKind::DYNAMIC_EQUATION
        }
        (EquationKind::CLOCKED { .. }, Some(mut clk)) => OldBackendDAE::EquationKind::CLOCKED_EQUATION { clk: clk },
        (EquationKind::DISCRETE, None) => openmodelica_backend_types::BackendDAE::EquationKind::DISCRETE_EQUATION,
        (EquationKind::EMPTY, None) => openmodelica_backend_types::BackendDAE::EquationKind::AUX_EQUATION,
        (EquationKind::UNKNOWN { .. }, None) => {
            openmodelica_backend_types::BackendDAE::EquationKind::UNKNOWN_EQUATION_KIND
        }
        (_, Some(_)) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBEquation.convertEquationKind"));
                    __mm_s.push_str(&*literal!(" failed because the non-clock equation kind "));
                    __mm_s.push_str(&*equationKindString(eqKind, clock_idx, exclusively_initial)?);
                    __mm_s.push_str(&*literal!(" has a clock index."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        (EquationKind::CLOCKED { .. }, None) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBEquation.convertEquationKind"));
                    __mm_s.push_str(&*literal!(
                        " failed because no clock index was provided for clocked equation."
                    ));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBEquation.convertEquationKind"));
                    __mm_s.push_str(&*literal!(" for an unknown reason."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(oldEqKind)
}

pub(crate) fn equationKindString(
    mut eqKind: EquationKind,
    mut clock_idx: Option<i32>,
    mut exclusively_initial: bool,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match eqKind {
        EquationKind::CONTINUOUS { .. } => literal!("[CONT"),
        EquationKind::CLOCKED { .. } => literal!("[CLCK"),
        EquationKind::DISCRETE => literal!("[DISC"),
        EquationKind::EMPTY => literal!("[EMTY"),
        _ => literal!("[UKWN"),
    });
    r#str = if (exclusively_initial) {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[INI]"));
            __mm_s.push_str(&*r#str);
            ArcStr::from(__mm_s)
        }
    } else {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[DAE]"));
            __mm_s.push_str(&*r#str);
            ArcStr::from(__mm_s)
        }
    };
    if (clock_idx).is_some() {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(Util::getOption(clock_idx)?));
            __mm_s.push_str(&*literal!(")]"));
            ArcStr::from(__mm_s)
        };
    } else {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

pub mod EquationPointers {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct EquationPointers {
        /// Map for cref->index
        pub map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        pub eqArr: metamodelica::Ref<
            ExpandableArray::ExpandableArray<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        >,
    }

    impl metamodelica::gc::MMTrace for EquationPointers {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.eqArr, __mmv)?;
            Ok(())
        }
    }
    impl Default for EquationPointers {
        fn default() -> Self {
            Self {
                map: Default::default(),
                eqArr: Default::default(),
            }
        }
    }

    pub type EQUATION_POINTERS = EquationPointers;

    pub(crate) fn toString(
        mut equations: &metamodelica::Ref<EquationPointers>,
        mut r#str: ArcStr,
        mut mapping_opt: Option<metamodelica::Array<(i32, i32)>>,
        mut printEmpty: bool,
        mut filter_opt: Option<metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>>,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut luI: i32 = lastUsedIndex(equations);
        let mut length: i32;
        let mut scal_start: i32;
        let mut current_index: i32 = 1;
        let mut index: ArcStr;
        let mut useMapping: bool = (mapping_opt).is_some();
        let mut filterEqs: bool = (filter_opt).is_some();
        let mut mapping: metamodelica::Array<(i32, i32)> =
            metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
        let mut filter: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>> = UnorderedSet::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            13,
        );
        let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        if useMapping {
            length = 15;
            mapping = Util::getOption(mapping_opt)?;
        } else {
            length = 10;
        }
        if filterEqs {
            filter = Util::getOption(filter_opt)?;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Filtered "));
                __mm_s.push_str(&*r#str);
                ArcStr::from(__mm_s)
            };
        }
        if printEmpty || luI > 0 {
            r#str = StringUtil::headline_4(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!(" Equations ("));
                    __mm_s.push_str(&*intString(size(equations)));
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*intString(scalarSize(equations, true)?));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }),
            )?;
            for mut i in 1..=luI {
                if ExpandableArray::occupied(i, equations.eqArr.clone()) {
                    eqn = ExpandableArray::get(i, equations.eqArr.clone())?;
                    if !(filterEqs)
                        || UnorderedSet::contains(
                            ComponentRef::toString(&(Equation::getEqnName(eqn.clone())?))?,
                            filter.clone(),
                        )?
                    {
                        if useMapping {
                            (scal_start, _) = ({
                                let __elt = (*metamodelica::index_checked(&mapping.borrow(), current_index)?).clone();
                                __elt
                            });
                            index = {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("("));
                                __mm_s.push_str(&*intString(current_index));
                                __mm_s.push_str(&*literal!("|"));
                                __mm_s.push_str(&*intString(scal_start));
                                __mm_s.push_str(&*literal!(")"));
                                ArcStr::from(__mm_s)
                            };
                        } else {
                            index = {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("("));
                                __mm_s.push_str(&*intString(current_index));
                                __mm_s.push_str(&*literal!(")"));
                                ArcStr::from(__mm_s)
                            };
                        }
                        index = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*index);
                            __mm_s.push_str(&*StringUtil::repeat(literal!(" "), length - ((index).len() as i32))?);
                            ArcStr::from(__mm_s)
                        };
                        r#str = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*Equation::toString(Pointer::access(eqn), index)?);
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        };
                    }
                    current_index = current_index + 1;
                }
            }
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        } else {
            r#str = literal!("");
        }
        Ok(r#str)
    }

    pub(crate) fn empty(mut size: i32) -> metamodelica::Ref<EquationPointers> {
        let mut equationPointers: metamodelica::Ref<EquationPointers>;
        let mut arr_size: i32;
        let mut bucketSize: i32;
        arr_size = std::cmp::max(size, BaseHashTable::lowBucketSize.clone());
        bucketSize = Util::nextPrime(arr_size);
        equationPointers = metamodelica::Ref::new(EquationPointers {
            map: UnorderedMap::new(
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
                bucketSize,
            ),
            eqArr: ExpandableArray::new(
                arr_size,
                Pointer::create(crate::NBEquation::Equation::interned_DUMMY_EQUATION()),
            ),
        });
        equationPointers
    }

    pub(crate) fn clone(
        mut equations: &metamodelica::Ref<EquationPointers>,
        mut shallow: bool,
    ) -> Result<metamodelica::Ref<EquationPointers>> {
        let mut new: metamodelica::Ref<EquationPointers>;
        if shallow {
            new = fromList(&(toList(equations)?))?;
        } else {
            new = fromList(
                &({
                    let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
                        metamodelica::nil();
                    for mut eqn in (toList(equations)?).into_iter().cloned() {
                        let __x = Pointer::create(Pointer::access(eqn.clone()));
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?;
        }
        Ok(new)
    }

    pub(crate) fn size(mut equations: &metamodelica::Ref<EquationPointers>) -> i32 {
        let mut sz: i32 = ExpandableArray::getNumberOfElements(equations.eqArr.clone());
        sz
    }

    pub(crate) fn scalarSize(mut equations: &metamodelica::Ref<EquationPointers>, mut resize: bool) -> Result<i32> {
        let mut sz: i32 = 0;
        for mut eqn_ptr in &*toList(equations)? {
            sz = sz + Equation::size(eqn_ptr.clone(), resize)?;
        }
        Ok(sz)
    }

    pub(crate) fn lastUsedIndex(mut equations: &metamodelica::Ref<EquationPointers>) -> i32 {
        let mut sz: i32 = ExpandableArray::getLastUsedIndex(equations.eqArr.clone());
        sz
    }

    pub(crate) fn toList(
        mut equations: &metamodelica::Ref<EquationPointers>,
    ) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> {
        let mut eqn_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
        eqn_lst = ExpandableArray::toList(equations.eqArr.clone())?;
        Ok(eqn_lst)
    }

    pub(crate) fn fromList(
        mut eq_lst: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    ) -> Result<metamodelica::Ref<EquationPointers>> {
        let mut equations: metamodelica::Ref<EquationPointers>;
        equations = empty(((eq_lst).len() as i32));
        equations = addList(eq_lst, equations)?;
        Ok(equations)
    }

    pub(crate) fn addList(
        mut eq_lst: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        mut equations: metamodelica::Ref<EquationPointers>,
    ) -> Result<metamodelica::Ref<EquationPointers>> {
        let mut equations: metamodelica::Ref<EquationPointers> = equations;
        equations = List::fold(eq_lst, &(move |__pe_a0, __pe_a1| add(__pe_a0, __pe_a1)), equations)?;
        Ok(equations)
    }

    pub(crate) fn removeList(
        mut eq_lst: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        mut equations: metamodelica::Ref<EquationPointers>,
    ) -> Result<metamodelica::Ref<EquationPointers>> {
        let mut equations: metamodelica::Ref<EquationPointers> = equations;
        equations = List::fold(eq_lst, &(move |__pe_a0, __pe_a1| remove(__pe_a0, __pe_a1)), equations)?;
        equations = compress(equations)?;
        Ok(equations)
    }

    pub(crate) fn removeCheck(
        mut equations: metamodelica::Ref<EquationPointers>,
        mut func: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<bool>,
    ) -> Result<metamodelica::Ref<EquationPointers>> {
        let mut equations: metamodelica::Ref<EquationPointers> = equations;
        let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
        eqns = ({
            let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
                metamodelica::nil();
            for mut eqn in (toList(&equations)?).into_iter().cloned() {
                if !(!(func(eqn.clone())?)) {
                    continue;
                }
                let __x = eqn.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        equations = fromList(&eqns)?;
        Ok(equations)
    }

    pub(crate) fn add(
        mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        mut equations: metamodelica::Ref<EquationPointers>,
    ) -> Result<metamodelica::Ref<EquationPointers>> {
        let mut equations: metamodelica::Ref<EquationPointers> = equations;
        let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut index: i32;
        name = Equation::getEqnName(eqn.clone())?;
        let () = (match UnorderedMap::get(name.clone(), equations.map.clone())? {
            Some(mut index) if (index > 0) => {
                ExpandableArray::update(index, eqn, equations.eqArr.clone())?;
                ()
            }
            _ => {
                (_, index) = ExpandableArray::add(eqn, equations.eqArr.clone())?;
                UnorderedMap::add(name, index, equations.map.clone())?;
                ()
            }
        });
        Ok(equations)
    }

    pub(crate) fn remove(
        mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        mut equations: metamodelica::Ref<EquationPointers>,
    ) -> Result<metamodelica::Ref<EquationPointers>> {
        let mut equations: metamodelica::Ref<EquationPointers> = equations;
        let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut index: i32;
        name = Equation::getEqnName(eqn)?;
        let () = (match UnorderedMap::get(name.clone(), equations.map.clone())? {
            Some(mut index) if (index > 0) => {
                ExpandableArray::delete(index, equations.eqArr.clone())?;
                UnorderedMap::add(name, -1, equations.map.clone())?;
                ()
            }
            _ => (),
        });
        Ok(equations)
    }

    pub(crate) fn map(
        mut equations: metamodelica::Ref<EquationPointers>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Equation::Equation>) -> Result<metamodelica::Ref<Equation::Equation>>,
    ) -> Result<metamodelica::Ref<EquationPointers>> {
        let mut equations: metamodelica::Ref<EquationPointers> = equations;
        let mut eq_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        let mut eq: metamodelica::Ref<Equation::Equation>;
        let mut new_eq: metamodelica::Ref<Equation::Equation>;
        let mut followEquations: metamodelica::List<ArcStr> =
            Flags::getConfigStringList(Flags::DEBUG_FOLLOW_EQUATIONS.clone())?;
        let mut debug: bool = !((followEquations).is_empty());
        let mut debug_eqns: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>> = UnorderedSet::fromList(
            &followEquations,
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        )?;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i, equations.eqArr.clone()) {
                eq_ptr = ExpandableArray::get(i, equations.eqArr.clone())?;
                eq = Pointer::access(eq_ptr.clone());
                new_eq = func(eq.clone())?;
                if !(referenceEq(&*(&*eq), &*(&*new_eq))) {
                    if debug
                        && (UnorderedSet::contains(
                            ComponentRef::toString(&(Equation::getEqnName(eq_ptr.clone())?))?,
                            debug_eqns.clone(),
                        )? || UnorderedSet::contains(
                            ComponentRef::toString(&(Equation::getEqnName(Pointer::create(new_eq.clone()))?))?,
                            debug_eqns.clone(),
                        )?)
                        && !(Equation::equalName(Pointer::create(eq.clone()), Pointer::create(new_eq.clone()))?)
                    {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("[debugFollowEquations] The equation:\n"));
                            __mm_s.push_str(&*Equation::toString(eq, literal!(""))?);
                            __mm_s.push_str(&*literal!("\nGets replaced by:\n"));
                            __mm_s.push_str(&*Equation::toString(new_eq.clone(), literal!(""))?);
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    Pointer::update(eq_ptr, new_eq);
                }
            }
        }
        Ok(equations)
    }

    pub(crate) fn mapPtr(
        mut equations: &metamodelica::Ref<EquationPointers>,
        mut func: &dyn ::std::ops::Fn(
            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        ) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    ) -> Result<()> {
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i, equations.eqArr.clone()) {
                func(ExpandableArray::get(i, equations.eqArr.clone())?)?;
            }
        }
        Ok(())
    }

    pub(crate) fn mapExp(
        mut equations: metamodelica::Ref<EquationPointers>,
        mut funcExp: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
        mut funcCrefOpt: Option<
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                    + 'static,
            >,
        >,
        mut mapFunc: Arc<
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
        >,
    ) -> Result<metamodelica::Ref<EquationPointers>> {
        let mut equations: metamodelica::Ref<EquationPointers> = equations;
        let mut eq_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        let mut eq: metamodelica::Ref<Equation::Equation>;
        let mut new_eq: metamodelica::Ref<Equation::Equation>;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i, equations.eqArr.clone()) {
                eq_ptr = ExpandableArray::get(i, equations.eqArr.clone())?;
                eq = Pointer::access(eq_ptr.clone());
                new_eq = Equation::map(eq.clone(), funcExp.clone(), funcCrefOpt.clone(), mapFunc.clone())?;
                if !(referenceEq(&*(eq), &*(&*new_eq))) {
                    Pointer::update(eq_ptr, new_eq);
                }
            }
        }
        Ok(equations)
    }

    pub(crate) fn mapRemovePtr(
        mut equations: metamodelica::Ref<EquationPointers>,
        mut func: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<bool>,
    ) -> Result<metamodelica::Ref<EquationPointers>> {
        let mut equations: metamodelica::Ref<EquationPointers> = equations;
        let mut eq_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i, equations.eqArr.clone()) {
                eq_ptr = ExpandableArray::get(i, equations.eqArr.clone())?;
                if func(eq_ptr.clone())? {
                    equations = remove(eq_ptr, equations)?;
                }
            }
        }
        equations = compress(equations)?;
        Ok(equations)
    }

    pub(crate) fn mapRes(
        mut equations: &metamodelica::Ref<EquationPointers>,
        mut func: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<()>,
    ) -> Result<()> {
        pub type mapFunc = std::sync::Arc<
            dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<()> + 'static,
        >;

        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i, equations.eqArr.clone()) {
                func(Equation::getResidualVar(ExpandableArray::get(
                    i,
                    equations.eqArr.clone(),
                )?)?)?;
            }
        }
        Ok(())
    }

    pub(crate) fn fold<T: Clone + 'static + metamodelica::gc::MMTrace>(
        mut equations: &metamodelica::Ref<EquationPointers>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Equation::Equation>, T) -> Result<T>,
        mut extArg: T,
    ) -> Result<T> {
        pub type MapFunc<T: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Equation::Equation>, T) -> Result<T> + 'static>;

        let mut extArg: T = extArg;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i, equations.eqArr.clone()) {
                extArg = func(
                    Pointer::access(ExpandableArray::get(i, equations.eqArr.clone())?),
                    extArg,
                )?;
            }
        }
        Ok(extArg)
    }

    pub(crate) fn foldPtr<T: Clone + 'static + metamodelica::gc::MMTrace>(
        mut equations: &metamodelica::Ref<EquationPointers>,
        mut func: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>, T) -> Result<T>,
        mut extArg: T,
    ) -> Result<T> {
        pub type MapFunc<T: Clone + 'static> = std::sync::Arc<
            dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>, T) -> Result<T> + 'static,
        >;

        let mut extArg: T = extArg;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i, equations.eqArr.clone()) {
                extArg = func(ExpandableArray::get(i, equations.eqArr.clone())?, extArg)?;
            }
        }
        Ok(extArg)
    }

    pub(crate) fn foldRemovePtr<T: Clone + 'static + metamodelica::gc::MMTrace>(
        mut equations: metamodelica::Ref<EquationPointers>,
        mut func: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>, T) -> Result<(T, bool)>,
        mut extArg: T,
    ) -> Result<(metamodelica::Ref<EquationPointers>, T)> {
        pub type MapFunc<T: Clone + 'static> = std::sync::Arc<
            dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>, T) -> Result<(T, bool)>
                + 'static,
        >;

        let mut equations: metamodelica::Ref<EquationPointers> = equations;
        let mut extArg: T = extArg;
        let mut eq_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        let mut delete: bool;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i, equations.eqArr.clone()) {
                eq_ptr = ExpandableArray::get(i, equations.eqArr.clone())?;
                (extArg, delete) = func(eq_ptr.clone(), extArg)?;
                if delete {
                    Pointer::update(eq_ptr, crate::NBEquation::Equation::interned_DUMMY_EQUATION());
                    assign_field!(equations.eqArr = ExpandableArray::delete(i, equations.eqArr.clone())?);
                }
            }
        }
        Ok((equations, extArg))
    }

    pub(crate) fn getEqnAt(
        mut equations: &metamodelica::Ref<EquationPointers>,
        mut index: i32,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> {
        let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        eqn = ExpandableArray::get(index, equations.eqArr.clone())?;
        Ok(eqn)
    }

    pub(crate) fn getEqnByName(
        mut equations: &metamodelica::Ref<EquationPointers>,
        mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> {
        let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        eqn = (match UnorderedMap::get(name.clone(), equations.map.clone())? {
            Some(mut index) if (index > 0) => getEqnAt(equations, index)?,
            Some(_) => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.EquationPointers.getEqnByName"));
                        __mm_s.push_str(&*literal!(" failed because the equation with the name "));
                        __mm_s.push_str(&*ComponentRef::toString(&name)?);
                        __mm_s.push_str(&*literal!(" has already been deleted."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.EquationPointers.getEqnByName"));
                        __mm_s.push_str(&*literal!(" failed because there is no equation with the name "));
                        __mm_s.push_str(&*ComponentRef::toString(&name)?);
                        __mm_s.push_str(&*literal!("."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(eqn)
    }

    pub(crate) fn getEqnIndex(
        mut equations: &metamodelica::Ref<EquationPointers>,
        mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    ) -> Result<i32> {
        let mut index: i32 = UnorderedMap::getOrDefault(name.clone(), equations.map.clone(), -1)?;
        Ok(index)
    }

    pub(crate) fn compress(
        mut equations: metamodelica::Ref<EquationPointers>,
    ) -> Result<metamodelica::Ref<EquationPointers>> {
        let mut equations: metamodelica::Ref<EquationPointers> = equations;
        let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = metamodelica::nil();
        for mut i in ({
            let __s = ExpandableArray::getLastUsedIndex(equations.eqArr.clone());
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        }) {
            if ExpandableArray::occupied(i, equations.eqArr.clone()) {
                eqn = ExpandableArray::get(i, equations.eqArr.clone())?;
                let () = (match &*(Pointer::access(eqn.clone())) {
                    Equation::DUMMY_EQUATION => (),
                    Equation::FOR_EQUATION { body, .. }
                        if (List::all(metamodelica::AsArg::as_arg(&body), &move |__a0: metamodelica::Ref<
                            Equation::Equation,
                        >|
                              -> metamodelica::Result<
                            _,
                        > {
                            ::std::result::Result::Ok(Equation::isDummy(&__a0))
                        })?) =>
                    {
                        ()
                    }
                    _ => {
                        eqns = metamodelica::cons(eqn, eqns);
                        ()
                    }
                });
            }
        }
        equations = fromList(&eqns)?;
        Ok(equations)
    }

    pub(crate) fn sort(
        mut equations: metamodelica::Ref<EquationPointers>,
    ) -> Result<metamodelica::Ref<EquationPointers>> {
        let mut equations: metamodelica::Ref<EquationPointers> = equations;
        let mut size: i32;
        let mut hash_lst: metamodelica::List<(i32, Pointer::Pointer<metamodelica::Ref<Equation::Equation>>)>;
        let mut hash_lst_ptr: Pointer::Pointer<
            metamodelica::List<(i32, Pointer::Pointer<metamodelica::Ref<Equation::Equation>>)>,
        > = Pointer::create(metamodelica::nil());
        let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        size = ExpandableArray::getNumberOfElements(equations.eqArr.clone());
        mapPtr(
            &equations,
            &({
                let __pe_b1 = ((metamodelica::OrderedFloat((size) as f64)
                    * (metamodelica::OrderedFloat((size) as f64)).ln())
                .0
                .floor() as i32);
                let __pe_b2 = hash_lst_ptr.clone();
                move |__pe_a0| createSortHashTpl(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            }),
        )?;
        hash_lst = List::sort(
            Pointer::access(hash_lst_ptr),
            std::sync::Arc::new(fnptr!(BackendUtil::indexTplGt, _, _)),
        )?;
        equations = empty(size);
        for mut tpl in &*hash_lst {
            (_, eqn_ptr) = tpl.clone();
            assign_field!(equations.eqArr = ExpandableArray::add(eqn_ptr, equations.eqArr.clone())?.0);
        }
        Ok(equations)
    }

    pub(crate) fn getResiduals(
        mut equations: &metamodelica::Ref<EquationPointers>,
    ) -> Result<metamodelica::Ref<VariablePointers::VariablePointers>> {
        let mut residuals: metamodelica::Ref<VariablePointers::VariablePointers>;
        residuals = BVariable::VariablePointers::fromList(
            &({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                    metamodelica::nil();
                for mut eqn in (toList(equations)?).into_iter().cloned() {
                    let __x = Equation::getResidualVar(eqn.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            false,
        )?;
        Ok(residuals)
    }

    fn createSortHashTpl(
        mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        mut r#mod: i32,
        mut hash_lst_ptr: Pointer::Pointer<
            metamodelica::List<(i32, Pointer::Pointer<metamodelica::Ref<Equation::Equation>>)>,
        >,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> {
        let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>> = eqn_ptr;
        let mut eqn: metamodelica::Ref<Equation::Equation>;
        let mut hash: i32;
        eqn = Pointer::access(eqn_ptr.clone());
        hash = BackendUtil::noNameHashEq(&eqn, r#mod)?;
        Pointer::update(
            hash_lst_ptr.clone(),
            metamodelica::cons((hash, eqn_ptr.clone()), Pointer::access(hash_lst_ptr)),
        );
        Ok(eqn_ptr)
    }
}

pub mod EqData {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum EqData {
        EQ_DATA_SIM {
            /// current index to be used for new identifier
            uniqueIndex: Pointer::Pointer<i32>,
            /// All equations
            equations: metamodelica::Ref<EquationPointers::EquationPointers>,
            /// All equations for simulation (without initial)
            simulation: metamodelica::Ref<EquationPointers::EquationPointers>,
            /// Continuous equations
            continuous: metamodelica::Ref<EquationPointers::EquationPointers>,
            /// Clocked equations
            clocked: metamodelica::Ref<EquationPointers::EquationPointers>,
            /// Discrete equations
            discretes: metamodelica::Ref<EquationPointers::EquationPointers>,
            /// (Exclusively) Initial equations
            initials: metamodelica::Ref<EquationPointers::EquationPointers>,
            /// Auxiliary equations
            auxiliaries: metamodelica::Ref<EquationPointers::EquationPointers>,
            /// Removed equations (alias and no return value)
            removed: metamodelica::Ref<EquationPointers::EquationPointers>,
        },
        EQ_DATA_JAC {
            /// current index to be used for new identifier
            uniqueIndex: Pointer::Pointer<i32>,
            /// All equations
            equations: metamodelica::Ref<EquationPointers::EquationPointers>,
            /// Result equations
            results: metamodelica::Ref<EquationPointers::EquationPointers>,
            /// Temporary inner equations
            temporary: metamodelica::Ref<EquationPointers::EquationPointers>,
            /// Auxiliary equations
            auxiliaries: metamodelica::Ref<EquationPointers::EquationPointers>,
            /// Removed equations (alias and no return value)
            removed: metamodelica::Ref<EquationPointers::EquationPointers>,
        },
        EQ_DATA_HES {
            /// current index to be used for new identifier
            uniqueIndex: Pointer::Pointer<i32>,
            /// All equations
            equations: metamodelica::Ref<EquationPointers::EquationPointers>,
            /// Result equation
            result: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
            /// Temporary inner equations
            temporary: metamodelica::Ref<EquationPointers::EquationPointers>,
            /// Auxiliary equations
            auxiliaries: metamodelica::Ref<EquationPointers::EquationPointers>,
            /// Removed equations (alias and no return value)
            removed: metamodelica::Ref<EquationPointers::EquationPointers>,
        },
        EQ_DATA_EMPTY,
    }
    impl metamodelica::gc::MMTrace for EqData {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                EqData::EQ_DATA_SIM {
                    uniqueIndex,
                    equations,
                    simulation,
                    continuous,
                    clocked,
                    discretes,
                    initials,
                    auxiliaries,
                    removed,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(uniqueIndex, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(equations, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(simulation, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(continuous, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(clocked, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(discretes, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(initials, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(auxiliaries, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(removed, __mmv)?;
                    Ok(())
                }
                EqData::EQ_DATA_JAC {
                    uniqueIndex,
                    equations,
                    results,
                    temporary,
                    auxiliaries,
                    removed,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(uniqueIndex, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(equations, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(results, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(temporary, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(auxiliaries, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(removed, __mmv)?;
                    Ok(())
                }
                EqData::EQ_DATA_HES {
                    uniqueIndex,
                    equations,
                    result,
                    temporary,
                    auxiliaries,
                    removed,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(uniqueIndex, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(equations, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(result, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(temporary, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(auxiliaries, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(removed, __mmv)?;
                    Ok(())
                }
                EqData::EQ_DATA_EMPTY => Ok(()),
            }
        }
    }
    impl EqData {
        pub fn interned_EQ_DATA_EMPTY() -> metamodelica::Ref<EqData> {
            thread_local! {
                static INTERNED: metamodelica::Ref<EqData> = metamodelica::Ref::new(EqData::EQ_DATA_EMPTY);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_EQ_DATA_EMPTY() -> metamodelica::Ref<EqData> {
        EqData::interned_EQ_DATA_EMPTY()
    }
    impl Default for EqData {
        fn default() -> Self {
            Self::EQ_DATA_EMPTY
        }
    }
    pub use self::EqData::{EQ_DATA_EMPTY, EQ_DATA_HES, EQ_DATA_JAC, EQ_DATA_SIM};
    pub(crate) fn size(mut eqData: &metamodelica::Ref<EqData>) -> Result<i32> {
        let mut s: i32;
        s = (match &**eqData {
            EQ_DATA_SIM {
                simulation: __eqData_simulation,
                ..
            } => EquationPointers::size(metamodelica::AsArg::as_arg(&__eqData_simulation)),
            EQ_DATA_JAC {
                equations: __eqData_equations,
                ..
            } => EquationPointers::size(metamodelica::AsArg::as_arg(&__eqData_equations)),
            EQ_DATA_HES {
                equations: __eqData_equations,
                ..
            } => EquationPointers::size(metamodelica::AsArg::as_arg(&__eqData_equations)),
            _ => return Err("match: no arm matched"),
        });
        Ok(s)
    }

    pub(crate) fn scalarSize(mut eqData: &metamodelica::Ref<EqData>, mut resize: bool) -> Result<i32> {
        let mut s: i32;
        s = (match &**eqData {
            EQ_DATA_SIM {
                simulation: __eqData_simulation,
                ..
            } => EquationPointers::scalarSize(metamodelica::AsArg::as_arg(&__eqData_simulation), resize)?,
            EQ_DATA_JAC {
                equations: __eqData_equations,
                ..
            } => EquationPointers::scalarSize(metamodelica::AsArg::as_arg(&__eqData_equations), resize)?,
            EQ_DATA_HES {
                equations: __eqData_equations,
                ..
            } => EquationPointers::scalarSize(metamodelica::AsArg::as_arg(&__eqData_equations), resize)?,
            _ => return Err("match: no arm matched"),
        });
        Ok(s)
    }

    pub(crate) fn map(
        mut eqData: metamodelica::Ref<EqData>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Equation::Equation>) -> Result<metamodelica::Ref<Equation::Equation>>,
    ) -> Result<metamodelica::Ref<EqData>> {
        let mut eqData: metamodelica::Ref<EqData> = eqData;
        eqData = (match &*eqData {
            EQ_DATA_SIM {
                simulation: __eqData_simulation,
                ..
            } => {
                assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                    simulation = EquationPointers::map(__eqData_simulation.clone(), func)?,
                    continuous = EquationPointers::map(var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone(), func)?,
                    clocked = EquationPointers::map(var_field!((*eqData).clocked, EqData::EQ_DATA_SIM).clone(), func)?,
                    discretes = EquationPointers::map(var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone(), func)?,
                    initials = EquationPointers::map(var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone(), func)?,
                    auxiliaries = EquationPointers::map(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_SIM).clone(), func)?
                );
                eqData
            }
            EQ_DATA_JAC {
                results: __eqData_results,
                ..
            } => {
                assign_variant_field!(eqData => EqData::EQ_DATA_JAC;
                    results = EquationPointers::map(__eqData_results.clone(), func)?,
                    temporary = EquationPointers::map(var_field!((*eqData).temporary, EqData::EQ_DATA_JAC).clone(), func)?,
                    auxiliaries = EquationPointers::map(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_JAC).clone(), func)?
                );
                eqData
            }
            EQ_DATA_HES {
                result: __eqData_result,
                temporary: __eqData_temporary,
                ..
            } => {
                Pointer::update(__eqData_result.clone(), func(Pointer::access(__eqData_result.clone()))?);
                assign_variant_field!(eqData => EqData::EQ_DATA_HES;
                    temporary = EquationPointers::map(__eqData_temporary.clone(), func)?,
                    auxiliaries = EquationPointers::map(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_HES).clone(), func)?
                );
                eqData
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(eqData)
    }

    pub(crate) fn mapExp(
        mut eqData: metamodelica::Ref<EqData>,
        mut func: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
        mut funcCrefOpt: Option<
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                    + 'static,
            >,
        >,
    ) -> Result<metamodelica::Ref<EqData>> {
        let mut eqData: metamodelica::Ref<EqData> = eqData;
        eqData = (match &*eqData {
            EQ_DATA_SIM {
                simulation: __eqData_simulation,
                ..
            } => {
                assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                    simulation = EquationPointers::mapExp(__eqData_simulation.clone(), func.clone(), funcCrefOpt.clone(), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
                    continuous = EquationPointers::mapExp(var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone(), func.clone(), funcCrefOpt.clone(), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
                    clocked = EquationPointers::mapExp(var_field!((*eqData).clocked, EqData::EQ_DATA_SIM).clone(), func.clone(), funcCrefOpt.clone(), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
                    discretes = EquationPointers::mapExp(var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone(), func.clone(), funcCrefOpt.clone(), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
                    initials = EquationPointers::mapExp(var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone(), func.clone(), funcCrefOpt.clone(), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
                    auxiliaries = EquationPointers::mapExp(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_SIM).clone(), func.clone(), funcCrefOpt.clone(), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
                    removed = EquationPointers::mapExp(var_field!((*eqData).removed, EqData::EQ_DATA_SIM).clone(), func.clone(), funcCrefOpt, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?
                );
                eqData
            }
            EQ_DATA_JAC {
                results: __eqData_results,
                ..
            } => {
                assign_variant_field!(eqData => EqData::EQ_DATA_JAC;
                    results = EquationPointers::mapExp(__eqData_results.clone(), func.clone(), funcCrefOpt.clone(), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
                    temporary = EquationPointers::mapExp(var_field!((*eqData).temporary, EqData::EQ_DATA_JAC).clone(), func.clone(), funcCrefOpt.clone(), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
                    auxiliaries = EquationPointers::mapExp(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_JAC).clone(), func.clone(), funcCrefOpt.clone(), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
                    removed = EquationPointers::mapExp(var_field!((*eqData).removed, EqData::EQ_DATA_JAC).clone(), func.clone(), funcCrefOpt, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?
                );
                eqData
            }
            EQ_DATA_HES {
                result: __eqData_result,
                temporary: __eqData_temporary,
                ..
            } => {
                Pointer::update(
                    __eqData_result.clone(),
                    Equation::map(
                        Pointer::access(__eqData_result.clone()),
                        func.clone(),
                        funcCrefOpt.clone(),
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
                                    )
                                        -> Result<metamodelica::Ref<Expression::NFExpression>>
                                    + 'static,
                            >),
                    )?,
                );
                assign_variant_field!(eqData => EqData::EQ_DATA_HES;
                    temporary = EquationPointers::mapExp(__eqData_temporary.clone(), func.clone(), funcCrefOpt.clone(), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
                    auxiliaries = EquationPointers::mapExp(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_HES).clone(), func.clone(), funcCrefOpt.clone(), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
                    removed = EquationPointers::mapExp(var_field!((*eqData).removed, EqData::EQ_DATA_HES).clone(), func.clone(), funcCrefOpt, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?
                );
                eqData
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(eqData)
    }

    pub(crate) fn toString(
        mut eqData: &metamodelica::Ref<EqData>,
        mut level: i32,
        mut filter_opt: Option<metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>>,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (match &**eqData {
            EQ_DATA_SIM {
                auxiliaries: __eqData_auxiliaries,
                clocked: __eqData_clocked,
                continuous: __eqData_continuous,
                discretes: __eqData_discretes,
                equations: __eqData_equations,
                initials: __eqData_initials,
                removed: __eqData_removed,
                simulation: __eqData_simulation,
                ..
            } => {
                let mut tmp: ArcStr;
                tmp = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Equation Data Simulation (scalar simulation equations: "));
                    __mm_s.push_str(&*intString(EquationPointers::scalarSize(
                        metamodelica::AsArg::as_arg(&__eqData_simulation),
                        true,
                    )?));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                };
                tmp = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*StringUtil::headline_2(&tmp)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
                if level == 0 {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*EquationPointers::toString(
                            metamodelica::AsArg::as_arg(&__eqData_equations),
                            literal!("Simulation"),
                            None,
                            false,
                            filter_opt,
                        )?);
                        ArcStr::from(__mm_s)
                    };
                } else {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*EquationPointers::toString(
                            metamodelica::AsArg::as_arg(&__eqData_continuous),
                            literal!("Continuous"),
                            None,
                            false,
                            filter_opt.clone(),
                        )?);
                        __mm_s.push_str(&*EquationPointers::toString(
                            metamodelica::AsArg::as_arg(&__eqData_clocked),
                            literal!("Clocked"),
                            None,
                            false,
                            filter_opt.clone(),
                        )?);
                        __mm_s.push_str(&*EquationPointers::toString(
                            metamodelica::AsArg::as_arg(&__eqData_discretes),
                            literal!("Discrete"),
                            None,
                            false,
                            filter_opt.clone(),
                        )?);
                        __mm_s.push_str(&*EquationPointers::toString(
                            metamodelica::AsArg::as_arg(&__eqData_initials),
                            literal!("(Exclusively) Initial"),
                            None,
                            false,
                            filter_opt.clone(),
                        )?);
                        __mm_s.push_str(&*EquationPointers::toString(
                            metamodelica::AsArg::as_arg(&__eqData_auxiliaries),
                            literal!("Auxiliary"),
                            None,
                            false,
                            filter_opt.clone(),
                        )?);
                        __mm_s.push_str(&*EquationPointers::toString(
                            metamodelica::AsArg::as_arg(&__eqData_removed),
                            literal!("Removed"),
                            None,
                            false,
                            filter_opt,
                        )?);
                        ArcStr::from(__mm_s)
                    };
                }
                tmp
            }
            EQ_DATA_JAC {
                auxiliaries: __eqData_auxiliaries,
                equations: __eqData_equations,
                results: __eqData_results,
                temporary: __eqData_temporary,
                ..
            } => {
                let mut tmp: ArcStr;
                if level == 0 {
                    tmp = EquationPointers::toString(
                        metamodelica::AsArg::as_arg(&__eqData_equations),
                        literal!("Jacobian"),
                        None,
                        false,
                        filter_opt,
                    )?;
                } else {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*EquationPointers::toString(
                            metamodelica::AsArg::as_arg(&__eqData_results),
                            literal!("Residual"),
                            None,
                            false,
                            filter_opt.clone(),
                        )?);
                        __mm_s.push_str(&*EquationPointers::toString(
                            metamodelica::AsArg::as_arg(&__eqData_temporary),
                            literal!("Inner"),
                            None,
                            false,
                            filter_opt.clone(),
                        )?);
                        __mm_s.push_str(&*EquationPointers::toString(
                            metamodelica::AsArg::as_arg(&__eqData_auxiliaries),
                            literal!("Auxiliary"),
                            None,
                            false,
                            filter_opt,
                        )?);
                        ArcStr::from(__mm_s)
                    };
                }
                tmp
            }
            EQ_DATA_HES {
                auxiliaries: __eqData_auxiliaries,
                equations: __eqData_equations,
                result: __eqData_result,
                temporary: __eqData_temporary,
                ..
            } => {
                let mut tmp: ArcStr;
                if level == 0 {
                    tmp = EquationPointers::toString(
                        metamodelica::AsArg::as_arg(&__eqData_equations),
                        literal!("Hessian"),
                        None,
                        false,
                        filter_opt,
                    )?;
                } else {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*StringUtil::headline_4(&(literal!("Result Equation")))?);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*Equation::toString(
                            Pointer::access(__eqData_result.clone()),
                            literal!(""),
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*EquationPointers::toString(
                            metamodelica::AsArg::as_arg(&__eqData_temporary),
                            literal!("Temporary Inner"),
                            None,
                            false,
                            filter_opt.clone(),
                        )?);
                        __mm_s.push_str(&*EquationPointers::toString(
                            metamodelica::AsArg::as_arg(&__eqData_auxiliaries),
                            literal!("Auxiliary"),
                            None,
                            false,
                            filter_opt,
                        )?);
                        ArcStr::from(__mm_s)
                    };
                }
                tmp
            }
            EQ_DATA_EMPTY { .. } => {
                literal!("Empty equation Data!\n")
            }
            _ => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBEquation.EqData.toString"));
                __mm_s.push_str(&*literal!(" failed!\n"));
                ArcStr::from(__mm_s)
            }
        });
        Ok(r#str)
    }

    pub(crate) fn getUniqueIndex(mut eqData: &metamodelica::Ref<EqData>) -> Result<Pointer::Pointer<i32>> {
        let mut uniqueIndex: Pointer::Pointer<i32>;
        uniqueIndex = (match &**eqData {
            EQ_DATA_SIM {
                uniqueIndex: __eqData_uniqueIndex,
                ..
            } => __eqData_uniqueIndex.clone(),
            EQ_DATA_JAC {
                uniqueIndex: __eqData_uniqueIndex,
                ..
            } => __eqData_uniqueIndex.clone(),
            EQ_DATA_HES {
                uniqueIndex: __eqData_uniqueIndex,
                ..
            } => __eqData_uniqueIndex.clone(),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.EqData.getUniqueIndex"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(uniqueIndex)
    }

    pub(crate) fn getEquations(
        mut eqData: &metamodelica::Ref<EqData>,
    ) -> Result<metamodelica::Ref<EquationPointers::EquationPointers>> {
        let mut equations: metamodelica::Ref<EquationPointers::EquationPointers>;
        equations = (match &**eqData {
            EQ_DATA_SIM {
                equations: __eqData_equations,
                ..
            } => __eqData_equations.clone(),
            EQ_DATA_JAC {
                equations: __eqData_equations,
                ..
            } => __eqData_equations.clone(),
            EQ_DATA_HES {
                equations: __eqData_equations,
                ..
            } => __eqData_equations.clone(),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.EqData.getEquations"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(equations)
    }

    pub(crate) fn setEquations(
        mut eqData: metamodelica::Ref<EqData>,
        mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    ) -> Result<metamodelica::Ref<EqData>> {
        let mut eqData: metamodelica::Ref<EqData> = eqData;
        eqData = (match &*eqData {
            EQ_DATA_SIM { .. } => {
                assign_variant_field!(eqData => EqData::EQ_DATA_SIM; equations = equations);
                eqData
            }
            EQ_DATA_JAC { .. } => {
                assign_variant_field!(eqData => EqData::EQ_DATA_JAC; equations = equations);
                eqData
            }
            EQ_DATA_HES { .. } => {
                assign_variant_field!(eqData => EqData::EQ_DATA_HES; equations = equations);
                eqData
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(eqData)
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
    #[repr(i32)]
    pub enum EqType {
        CONTINUOUS = 1,
        DISCRETE = 2,
        CLOCKED = 3,
        INITIAL = 4,
    }
    impl PartialOrd for EqType {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for EqType {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            (*self as i32).cmp(&(*other as i32))
        }
    }
    impl metamodelica::gc::MMTrace for EqType {
        fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            Ok(())
        }
    }

    pub(crate) fn addTypedList(
        mut eqData: metamodelica::Ref<EqData>,
        mut eq_lst: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        mut eqType: EqType,
        mut newName: bool,
    ) -> Result<metamodelica::Ref<EqData>> {
        let mut eqData: metamodelica::Ref<EqData> = eqData;
        eqData = (::match_deref::match_deref! { match &((eqData.clone(), eqType)) {
            (Deref @ EQ_DATA_SIM { .. }, EqType::CONTINUOUS { .. }) => {
                if newName {
                    for mut eqn_ptr in &**eq_lst {
                        Equation::createName(eqn_ptr.clone(), var_field!((*eqData).uniqueIndex, EqData::EQ_DATA_SIM).clone(), &(arcstr::literal!(SIMULATION_STR)))?;
                    }
                }
                assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                    equations = EquationPointers::addList(eq_lst, var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone())?,
                    simulation = EquationPointers::addList(eq_lst, var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone())?,
                    continuous = EquationPointers::addList(eq_lst, var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone())?
                );
                eqData
            },
            (Deref @ EQ_DATA_SIM { .. }, EqType::DISCRETE) => {
                if newName {
                    for mut eqn_ptr in &**eq_lst {
                        Equation::createName(eqn_ptr.clone(), var_field!((*eqData).uniqueIndex, EqData::EQ_DATA_SIM).clone(), &(arcstr::literal!(SIMULATION_STR)))?;
                    }
                }
                assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                    equations = EquationPointers::addList(eq_lst, var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone())?,
                    simulation = EquationPointers::addList(eq_lst, var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone())?,
                    discretes = EquationPointers::addList(eq_lst, var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone())?
                );
                eqData
            },
            (Deref @ EQ_DATA_SIM { .. }, EqType::CLOCKED { .. }) => {
                if newName {
                    for mut eqn_ptr in &**eq_lst {
                        Equation::createName(eqn_ptr.clone(), var_field!((*eqData).uniqueIndex, EqData::EQ_DATA_SIM).clone(), &(arcstr::literal!(SIMULATION_STR)))?;
                    }
                }
                assign_variant_field!(eqData => EqData::EQ_DATA_SIM; clocked = EquationPointers::addList(eq_lst, var_field!((*eqData).clocked, EqData::EQ_DATA_SIM).clone())?);
                eqData
            },
            (Deref @ EQ_DATA_SIM { .. }, EqType::INITIAL) => {
                if newName {
                    for mut eqn_ptr in &**eq_lst {
                        Equation::createName(eqn_ptr.clone(), var_field!((*eqData).uniqueIndex, EqData::EQ_DATA_SIM).clone(), &(arcstr::literal!(SIMULATION_STR)))?;
                    }
                }
                assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                    equations = EquationPointers::addList(eq_lst, var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone())?,
                    initials = EquationPointers::addList(eq_lst, var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone())?
                );
                eqData
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.EqData.addTypedList")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(eqData)
    }

    pub(crate) fn addUntypedList(
        mut eqData: metamodelica::Ref<EqData>,
        mut eq_lst: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        mut newName: bool,
    ) -> Result<metamodelica::Ref<EqData>> {
        let mut eqData: metamodelica::Ref<EqData> = eqData;
        let mut continuous_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
        let mut clocked_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
        let mut discretes_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
        let mut initials_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
        let mut auxiliaries_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
        let mut simulation_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
        let mut removed_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
        eqData = (match &*eqData {
            EQ_DATA_SIM {
                equations: __eqData_equations,
                uniqueIndex: __eqData_uniqueIndex,
                ..
            } => {
                if newName {
                    for mut eqn_ptr in &**eq_lst {
                        Equation::createName(
                            eqn_ptr.clone(),
                            __eqData_uniqueIndex.clone(),
                            &(arcstr::literal!(SIMULATION_STR)),
                        )?;
                    }
                }
                (
                    simulation_lst,
                    continuous_lst,
                    clocked_lst,
                    discretes_lst,
                    initials_lst,
                    auxiliaries_lst,
                    removed_lst,
                ) = typeList(eq_lst)?;
                assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                    equations = EquationPointers::addList(eq_lst, __eqData_equations.clone())?,
                    simulation = EquationPointers::addList(&simulation_lst, var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone())?,
                    continuous = EquationPointers::addList(&continuous_lst, var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone())?,
                    clocked = EquationPointers::addList(&clocked_lst, var_field!((*eqData).clocked, EqData::EQ_DATA_SIM).clone())?,
                    discretes = EquationPointers::addList(&discretes_lst, var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone())?,
                    initials = EquationPointers::addList(&initials_lst, var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone())?,
                    auxiliaries = EquationPointers::addList(&auxiliaries_lst, var_field!((*eqData).auxiliaries, EqData::EQ_DATA_SIM).clone())?,
                    removed = EquationPointers::addList(&removed_lst, var_field!((*eqData).removed, EqData::EQ_DATA_SIM).clone())?
                );
                eqData
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.EqData.addUntypedList"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(eqData)
    }

    pub(crate) fn removeList(
        mut eq_lst: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        mut eqData: metamodelica::Ref<EqData>,
    ) -> Result<metamodelica::Ref<EqData>> {
        let mut eqData: metamodelica::Ref<EqData> = eqData;
        eqData = (match &*eqData {
            EQ_DATA_SIM {
                equations: __eqData_equations,
                ..
            } => {
                assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                    equations = EquationPointers::removeList(eq_lst, __eqData_equations.clone())?,
                    simulation = EquationPointers::removeList(eq_lst, var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone())?,
                    continuous = EquationPointers::removeList(eq_lst, var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone())?,
                    discretes = EquationPointers::removeList(eq_lst, var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone())?,
                    clocked = EquationPointers::removeList(eq_lst, var_field!((*eqData).clocked, EqData::EQ_DATA_SIM).clone())?,
                    initials = EquationPointers::removeList(eq_lst, var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone())?,
                    auxiliaries = EquationPointers::removeList(eq_lst, var_field!((*eqData).auxiliaries, EqData::EQ_DATA_SIM).clone())?,
                    removed = EquationPointers::removeList(eq_lst, var_field!((*eqData).removed, EqData::EQ_DATA_SIM).clone())?
                );
                eqData
            }
            EQ_DATA_JAC {
                equations: __eqData_equations,
                ..
            } => {
                assign_variant_field!(eqData => EqData::EQ_DATA_JAC;
                    equations = EquationPointers::removeList(eq_lst, __eqData_equations.clone())?,
                    results = EquationPointers::removeList(eq_lst, var_field!((*eqData).results, EqData::EQ_DATA_JAC).clone())?,
                    temporary = EquationPointers::removeList(eq_lst, var_field!((*eqData).temporary, EqData::EQ_DATA_JAC).clone())?,
                    auxiliaries = EquationPointers::removeList(eq_lst, var_field!((*eqData).auxiliaries, EqData::EQ_DATA_JAC).clone())?,
                    removed = EquationPointers::removeList(eq_lst, var_field!((*eqData).removed, EqData::EQ_DATA_JAC).clone())?
                );
                eqData
            }
            EQ_DATA_HES {
                equations: __eqData_equations,
                ..
            } => {
                assign_variant_field!(eqData => EqData::EQ_DATA_HES;
                    equations = EquationPointers::removeList(eq_lst, __eqData_equations.clone())?,
                    temporary = EquationPointers::removeList(eq_lst, var_field!((*eqData).temporary, EqData::EQ_DATA_HES).clone())?,
                    auxiliaries = EquationPointers::removeList(eq_lst, var_field!((*eqData).auxiliaries, EqData::EQ_DATA_HES).clone())?,
                    removed = EquationPointers::removeList(eq_lst, var_field!((*eqData).removed, EqData::EQ_DATA_HES).clone())?
                );
                eqData
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.EqData.removeList"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(eqData)
    }

    pub(crate) fn removeTypedCheck(
        mut eqData: metamodelica::Ref<EqData>,
        mut func: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<bool>,
        mut eqType: EqType,
    ) -> Result<metamodelica::Ref<EqData>> {
        let mut eqData: metamodelica::Ref<EqData> = eqData;
        eqData = (::match_deref::match_deref! { match &((eqData.clone(), eqType)) {
            (Deref @ EQ_DATA_SIM { .. }, EqType::CONTINUOUS { .. }) => {
                assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                    equations = EquationPointers::removeCheck(var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone(), func)?,
                    simulation = EquationPointers::removeCheck(var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone(), func)?,
                    continuous = EquationPointers::removeCheck(var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone(), func)?
                );
                eqData
            },
            (Deref @ EQ_DATA_SIM { .. }, EqType::DISCRETE) => {
                assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                    equations = EquationPointers::removeCheck(var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone(), func)?,
                    simulation = EquationPointers::removeCheck(var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone(), func)?,
                    discretes = EquationPointers::removeCheck(var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone(), func)?
                );
                eqData
            },
            (Deref @ EQ_DATA_SIM { .. }, EqType::CLOCKED { .. }) => {
                assign_variant_field!(eqData => EqData::EQ_DATA_SIM; clocked = EquationPointers::removeCheck(var_field!((*eqData).clocked, EqData::EQ_DATA_SIM).clone(), func)?);
                eqData
            },
            (Deref @ EQ_DATA_SIM { .. }, EqType::INITIAL) => {
                assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                    equations = EquationPointers::removeCheck(var_field!((*eqData).equations, EqData::EQ_DATA_SIM).clone(), func)?,
                    initials = EquationPointers::removeCheck(var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone(), func)?
                );
                eqData
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEquation.EqData.removeTypedCheck")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(eqData)
    }

    pub(crate) fn compress(mut eqData: metamodelica::Ref<EqData>) -> Result<metamodelica::Ref<EqData>> {
        let mut eqData: metamodelica::Ref<EqData> = eqData;
        eqData = (match &*eqData {
            EQ_DATA_SIM {
                equations: __eqData_equations,
                ..
            } => {
                assign_variant_field!(eqData => EqData::EQ_DATA_SIM;
                    equations = EquationPointers::compress(__eqData_equations.clone())?,
                    simulation = EquationPointers::compress(var_field!((*eqData).simulation, EqData::EQ_DATA_SIM).clone())?,
                    continuous = EquationPointers::compress(var_field!((*eqData).continuous, EqData::EQ_DATA_SIM).clone())?,
                    discretes = EquationPointers::compress(var_field!((*eqData).discretes, EqData::EQ_DATA_SIM).clone())?,
                    initials = EquationPointers::compress(var_field!((*eqData).initials, EqData::EQ_DATA_SIM).clone())?,
                    auxiliaries = EquationPointers::compress(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_SIM).clone())?,
                    removed = EquationPointers::compress(var_field!((*eqData).removed, EqData::EQ_DATA_SIM).clone())?
                );
                eqData
            }
            EQ_DATA_JAC {
                equations: __eqData_equations,
                ..
            } => {
                assign_variant_field!(eqData => EqData::EQ_DATA_JAC;
                    equations = EquationPointers::compress(__eqData_equations.clone())?,
                    results = EquationPointers::compress(var_field!((*eqData).results, EqData::EQ_DATA_JAC).clone())?,
                    temporary = EquationPointers::compress(var_field!((*eqData).temporary, EqData::EQ_DATA_JAC).clone())?,
                    auxiliaries = EquationPointers::compress(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_JAC).clone())?,
                    removed = EquationPointers::compress(var_field!((*eqData).removed, EqData::EQ_DATA_JAC).clone())?
                );
                eqData
            }
            EQ_DATA_HES {
                equations: __eqData_equations,
                ..
            } => {
                assign_variant_field!(eqData => EqData::EQ_DATA_HES;
                    equations = EquationPointers::compress(__eqData_equations.clone())?,
                    temporary = EquationPointers::compress(var_field!((*eqData).temporary, EqData::EQ_DATA_HES).clone())?,
                    auxiliaries = EquationPointers::compress(var_field!((*eqData).auxiliaries, EqData::EQ_DATA_HES).clone())?,
                    removed = EquationPointers::compress(var_field!((*eqData).removed, EqData::EQ_DATA_HES).clone())?
                );
                eqData
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.EqData.compress"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(eqData)
    }
}

pub(crate) fn typeList(
    mut equations: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
) -> Result<(
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
)> {
    let mut simulation_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut continuous_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut clocked_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut discretes_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut initials_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut auxiliaries_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut removed_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    for mut eq in &**equations {
        let () = (match &*(Equation::getAttributes(Pointer::access(eq.clone()))) {
            EquationAttributes::EQUATION_ATTRIBUTES {
                exclusively_initial: true,
                ..
            } => {
                initials_lst = metamodelica::cons(eq.clone(), initials_lst);
                ()
            }
            EquationAttributes::EQUATION_ATTRIBUTES {
                kind: EquationKind::CONTINUOUS { .. },
                ..
            } => {
                continuous_lst = metamodelica::cons(eq.clone(), continuous_lst);
                simulation_lst = metamodelica::cons(eq.clone(), simulation_lst);
                ()
            }
            EquationAttributes::EQUATION_ATTRIBUTES {
                kind: EquationKind::CLOCKED { .. },
                ..
            } => {
                clocked_lst = metamodelica::cons(eq.clone(), clocked_lst);
                ()
            }
            EquationAttributes::EQUATION_ATTRIBUTES {
                kind: EquationKind::DISCRETE,
                ..
            } => {
                discretes_lst = metamodelica::cons(eq.clone(), discretes_lst);
                simulation_lst = metamodelica::cons(eq.clone(), simulation_lst);
                ()
            }
            EquationAttributes::EQUATION_ATTRIBUTES {
                kind: EquationKind::EMPTY,
                ..
            } => {
                removed_lst = metamodelica::cons(eq.clone(), removed_lst);
                ()
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEquation.typeList"));
                        __mm_s.push_str(&*literal!(" failed for\n"));
                        __mm_s.push_str(&*Equation::toString(Pointer::access(eq.clone()), literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
    }
    Ok((
        simulation_lst,
        continuous_lst,
        clocked_lst,
        discretes_lst,
        initials_lst,
        auxiliaries_lst,
        removed_lst,
    ))
}
