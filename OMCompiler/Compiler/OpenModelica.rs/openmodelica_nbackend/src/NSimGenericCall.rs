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

use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NSimCode::Identifier;
use openmodelica_backend_types::BackendDAE as OldBackendDAE;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFConvertDAE as ConvertDAE;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_simcode_types::SimCode as OldSimCode;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:        NSimGenericCall.mo
/// package:     NSimGenericCall
/// description: This file contains the data types and functions for generic for loop calls.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum NSimGenericCall {
    SINGLE_GENERIC_CALL {
        index: i32,
        iters: metamodelica::List<metamodelica::Ref<SimIterator::SimIterator>>,
        lhs: metamodelica::Ref<Expression::NFExpression>,
        rhs: metamodelica::Ref<Expression::NFExpression>,
        resizable: bool,
    },
    IF_GENERIC_CALL {
        index: i32,
        iters: metamodelica::List<metamodelica::Ref<SimIterator::SimIterator>>,
        branches: metamodelica::List<metamodelica::Ref<SimBranch::SimBranch>>,
        resizable: bool,
    },
    WHEN_GENERIC_CALL {
        index: i32,
        iters: metamodelica::List<metamodelica::Ref<SimIterator::SimIterator>>,
        branches: metamodelica::List<metamodelica::Ref<SimBranch::SimBranch>>,
        resizable: bool,
    },
}
impl metamodelica::gc::MMTrace for NSimGenericCall {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NSimGenericCall::SINGLE_GENERIC_CALL {
                index,
                iters,
                lhs,
                rhs,
                resizable,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iters, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(resizable, __mmv)?;
                Ok(())
            }
            NSimGenericCall::IF_GENERIC_CALL {
                index,
                iters,
                branches,
                resizable,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iters, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(resizable, __mmv)?;
                Ok(())
            }
            NSimGenericCall::WHEN_GENERIC_CALL {
                index,
                iters,
                branches,
                resizable,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iters, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(resizable, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for NSimGenericCall {
    fn default() -> Self {
        Self::IF_GENERIC_CALL {
            index: Default::default(),
            iters: Default::default(),
            branches: Default::default(),
            resizable: Default::default(),
        }
    }
}
pub(crate) use self::NSimGenericCall::{IF_GENERIC_CALL, SINGLE_GENERIC_CALL, WHEN_GENERIC_CALL};
pub(crate) fn mapShallow(
    mut call: metamodelica::Ref<NSimGenericCall>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NSimGenericCall>> {
    pub type mapExp = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut call: metamodelica::Ref<NSimGenericCall> = call;
    call = (match &*call {
        SINGLE_GENERIC_CALL { lhs: __call_lhs, .. } => {
            assign_variant_field!(call => NSimGenericCall::SINGLE_GENERIC_CALL;
                lhs = func(__call_lhs.clone())?,
                rhs = func(var_field!((*call).rhs, NSimGenericCall::SINGLE_GENERIC_CALL).clone())?
            );
            call
        }
        IF_GENERIC_CALL {
            branches: __call_branches,
            ..
        } => {
            assign_variant_field!(call => NSimGenericCall::IF_GENERIC_CALL; branches = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SimBranch::SimBranch>> = metamodelica::nil();
                for mut branch in (__call_branches.clone()).into_iter().cloned() {
                    let __x = SimBranch::mapShallow(branch.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            call
        }
        WHEN_GENERIC_CALL {
            branches: __call_branches,
            ..
        } => {
            assign_variant_field!(call => NSimGenericCall::WHEN_GENERIC_CALL; branches = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SimBranch::SimBranch>> = metamodelica::nil();
                for mut branch in (__call_branches.clone()).into_iter().cloned() {
                    let __x = SimBranch::mapShallow(branch.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            call
        }
        _ => call,
    });
    Ok(call)
}

pub(crate) fn toString(mut call: &metamodelica::Ref<NSimGenericCall>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**call {
        SINGLE_GENERIC_CALL {
            index: __call_index,
            iters: __call_iters,
            lhs: __call_lhs,
            rhs: __call_rhs,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(__call_index.clone()));
            __mm_s.push_str(&*literal!(") [SNGL]: "));
            __mm_s.push_str(&*List::toString(
                __call_iters.clone(),
                &move |__a0: metamodelica::Ref<SimIterator::SimIterator>| SimIterator::toString(&__a0),
                List::Style::FLAT_CURLY.clone(),
            )?);
            __mm_s.push_str(&*literal!("\n\t"));
            __mm_s.push_str(&*Expression::toString(__call_lhs.clone())?);
            __mm_s.push_str(&*literal!(" = "));
            __mm_s.push_str(&*Expression::toString(__call_rhs.clone())?);
            ArcStr::from(__mm_s)
        }
        IF_GENERIC_CALL {
            branches: __call_branches,
            index: __call_index,
            iters: __call_iters,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(__call_index.clone()));
            __mm_s.push_str(&*literal!(") [-IF-]: "));
            __mm_s.push_str(&*List::toString(
                __call_iters.clone(),
                &move |__a0: metamodelica::Ref<SimIterator::SimIterator>| SimIterator::toString(&__a0),
                List::Style::FLAT_CURLY.clone(),
            )?);
            __mm_s.push_str(&*literal!("\n\t"));
            __mm_s.push_str(&*List::toStringCustom(
                __call_branches.clone(),
                &move |__a0: metamodelica::Ref<SimBranch::SimBranch>| SimBranch::toString(&__a0),
                literal!(""),
                literal!(""),
                literal!("\telse"),
                literal!(""),
                true,
                0,
            )?);
            ArcStr::from(__mm_s)
        }
        WHEN_GENERIC_CALL {
            branches: __call_branches,
            index: __call_index,
            iters: __call_iters,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(__call_index.clone()));
            __mm_s.push_str(&*literal!(") [WHEN]: "));
            __mm_s.push_str(&*List::toString(
                __call_iters.clone(),
                &move |__a0: metamodelica::Ref<SimIterator::SimIterator>| SimIterator::toString(&__a0),
                List::Style::FLAT_CURLY.clone(),
            )?);
            __mm_s.push_str(&*literal!("\n\t"));
            __mm_s.push_str(&*List::toStringCustom(
                __call_branches.clone(),
                &move |__a0: metamodelica::Ref<SimBranch::SimBranch>| SimBranch::toString(&__a0),
                literal!(""),
                literal!(""),
                literal!("\telse"),
                literal!(""),
                true,
                0,
            )?);
            ArcStr::from(__mm_s)
        }
        _ => literal!("CALL_NOT_SUPPORTED"),
    });
    Ok(r#str)
}

pub(crate) fn fromIdentifier(
    mut ident_tpl: &(metamodelica::Ref<Identifier::Identifier>, i32),
) -> Result<metamodelica::Ref<NSimGenericCall>> {
    let mut call: metamodelica::Ref<NSimGenericCall>;
    let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut index: i32;
    let mut resizable: bool;
    let mut body: metamodelica::Ref<Equation::Equation>;
    let mut eqn: metamodelica::Ref<Equation::Equation>;
    let (__t3, __pa2) = ident_tpl.clone();
    let __arc4 = __t3.clone();
    let Identifier::IDENTIFIER {
        eqn: __pa0,
        resizable: __pa1,
        ..
    } = &*__arc4;
    eqn_ptr = metamodelica::Own::own(__pa0);
    resizable = metamodelica::Own::own(__pa1);
    index = metamodelica::Own::own(__pa2);
    eqn = Pointer::access(eqn_ptr);
    call = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: __esc_body @ Deref @ Equation::IF_EQUATION { .. }, tail: Deref @ metamodelica::ListNode::Nil }, iter: __eqn_iter, .. } => {
            body = (*__esc_body).clone();
            let mut iters: metamodelica::List<metamodelica::Ref<SimIterator::SimIterator>>;
            iters = SimIterator::fromIterator(metamodelica::AsArg::as_arg(&__eqn_iter))?;
            metamodelica::Ref::new(NSimGenericCall::IF_GENERIC_CALL { index: index, iters: iters, branches: SimBranch::fromIfBody(var_field!((*body).body, Equation::Equation::IF_EQUATION))?, resizable: resizable })
        },
        Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: __esc_body @ Deref @ Equation::WHEN_EQUATION { .. }, tail: Deref @ metamodelica::ListNode::Nil }, iter: __eqn_iter, .. } => {
            body = (*__esc_body).clone();
            let mut iters: metamodelica::List<metamodelica::Ref<SimIterator::SimIterator>>;
            iters = SimIterator::fromIterator(metamodelica::AsArg::as_arg(&__eqn_iter))?;
            metamodelica::Ref::new(NSimGenericCall::WHEN_GENERIC_CALL { index: index, iters: iters, branches: SimBranch::fromWhenBody(var_field!((*body).body, Equation::Equation::WHEN_EQUATION))?, resizable: resizable })
        },
        Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: __esc_body, tail: Deref @ metamodelica::ListNode::Nil }, iter: __eqn_iter, .. } => {
            body = (*__esc_body).clone();
            let mut iters: metamodelica::List<metamodelica::Ref<SimIterator::SimIterator>>;
            iters = SimIterator::fromIterator(metamodelica::AsArg::as_arg(&__eqn_iter))?;
            metamodelica::Ref::new(NSimGenericCall::SINGLE_GENERIC_CALL { index: index, iters: iters, lhs: (Equation::getLHS(body.clone())?).ok_or("pattern mismatch")?, rhs: (Equation::getRHS(body.clone())?).ok_or("pattern mismatch")?, resizable: resizable })
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimGenericCall.fromIdentifier")); __mm_s.push_str(&*literal!(" failed for incorrect equation: ")); __mm_s.push_str(&*Equation::toString(eqn, literal!(""))?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(call)
}

pub(crate) fn convert(mut call: &metamodelica::Ref<NSimGenericCall>) -> Result<OldSimCode::SimGenericCall> {
    let mut old_call: OldSimCode::SimGenericCall;
    old_call = (match &**call {
        SINGLE_GENERIC_CALL {
            index: __call_index,
            iters: __call_iters,
            lhs: __call_lhs,
            resizable: __call_resizable,
            rhs: __call_rhs,
        } => OldSimCode::SimGenericCall::SINGLE_GENERIC_CALL {
            index: __call_index.clone(),
            iters: ({
                let mut __acc: metamodelica::List<OldBackendDAE::SimIterator> = metamodelica::nil();
                for mut iter in (__call_iters.clone()).into_iter().cloned() {
                    let __x = SimIterator::convert(&(iter.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            lhs: Expression::toDAE(__call_lhs.clone(), false)?,
            rhs: Expression::toDAE(__call_rhs.clone(), false)?,
            resizable: __call_resizable.clone(),
        },
        IF_GENERIC_CALL {
            branches: __call_branches,
            index: __call_index,
            iters: __call_iters,
            resizable: __call_resizable,
        } => OldSimCode::SimGenericCall::IF_GENERIC_CALL {
            index: __call_index.clone(),
            iters: ({
                let mut __acc: metamodelica::List<OldBackendDAE::SimIterator> = metamodelica::nil();
                for mut iter in (__call_iters.clone()).into_iter().cloned() {
                    let __x = SimIterator::convert(&(iter.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            branches: ({
                let mut __acc: metamodelica::List<OldSimCode::SimBranch> = metamodelica::nil();
                for mut branch in (__call_branches.clone()).into_iter().cloned() {
                    let __x = SimBranch::convert(&(branch.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            resizable: __call_resizable.clone(),
        },
        WHEN_GENERIC_CALL {
            branches: __call_branches,
            index: __call_index,
            iters: __call_iters,
            resizable: __call_resizable,
        } => OldSimCode::SimGenericCall::WHEN_GENERIC_CALL {
            index: __call_index.clone(),
            iters: ({
                let mut __acc: metamodelica::List<OldBackendDAE::SimIterator> = metamodelica::nil();
                for mut iter in (__call_iters.clone()).into_iter().cloned() {
                    let __x = SimIterator::convert(&(iter.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            branches: ({
                let mut __acc: metamodelica::List<OldSimCode::SimBranch> = metamodelica::nil();
                for mut branch in (__call_branches.clone()).into_iter().cloned() {
                    let __x = SimBranch::convert(&(branch.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            resizable: __call_resizable.clone(),
        },
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NSimGenericCall.convert"));
                    __mm_s.push_str(&*literal!(" failed for incorrect call: "));
                    __mm_s.push_str(&*toString(call)?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(old_call)
}

pub mod SimIterator {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum SimIterator {
        SIM_ITERATOR_RANGE {
            name: metamodelica::Ref<ComponentRef::NFComponentRef>,
            start: metamodelica::Ref<Expression::NFExpression>,
            step: metamodelica::Ref<Expression::NFExpression>,
            stop: metamodelica::Ref<Expression::NFExpression>,
            size: metamodelica::Ref<Expression::NFExpression>,
            sub_iter: metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
            )>,
        },
        SIM_ITERATOR_LIST {
            name: metamodelica::Ref<ComponentRef::NFComponentRef>,
            lst: metamodelica::List<i32>,
            size: i32,
            sub_iter: metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
            )>,
        },
    }
    impl metamodelica::gc::MMTrace for SimIterator {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                SimIterator::SIM_ITERATOR_RANGE {
                    name,
                    start,
                    step,
                    stop,
                    size,
                    sub_iter,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(step, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(stop, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(sub_iter, __mmv)?;
                    Ok(())
                }
                SimIterator::SIM_ITERATOR_LIST {
                    name,
                    lst,
                    size,
                    sub_iter,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(lst, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(sub_iter, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for SimIterator {
        fn default() -> Self {
            Self::SIM_ITERATOR_LIST {
                name: Default::default(),
                lst: Default::default(),
                size: Default::default(),
                sub_iter: Default::default(),
            }
        }
    }
    pub use self::SimIterator::{SIM_ITERATOR_LIST, SIM_ITERATOR_RANGE};
    pub(crate) fn toString(mut iter: &metamodelica::Ref<SimIterator>) -> Result<ArcStr> {
        pub(crate) fn subIterString(
            mut sub_iter: metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
            )>,
        ) -> Result<ArcStr> {
            let mut r#str: ArcStr = List::toStringCustom(
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                        metamodelica::nil();
                    for mut tpl in (sub_iter.clone()).into_iter().cloned() {
                        let __x = Util::tuple21(tpl.clone());
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                literal!(""),
                literal!("("),
                literal!(", "),
                literal!(")"),
                false,
                0,
            )?;
            Ok(r#str)
        }

        let mut r#str: ArcStr;
        r#str = (match &**iter {
            SIM_ITERATOR_RANGE {
                name: __iter_name,
                size: __iter_size,
                start: __iter_start,
                step: __iter_step,
                stop: __iter_stop,
                sub_iter: __iter_sub_iter,
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&__iter_name))?);
                __mm_s.push_str(&*literal!(" | start:"));
                __mm_s.push_str(&*Expression::toString(__iter_start.clone())?);
                __mm_s.push_str(&*literal!(", step:"));
                __mm_s.push_str(&*Expression::toString(__iter_step.clone())?);
                __mm_s.push_str(&*literal!(", stop:"));
                __mm_s.push_str(&*Expression::toString(__iter_stop.clone())?);
                __mm_s.push_str(&*literal!(", size: "));
                __mm_s.push_str(&*Expression::toString(__iter_size.clone())?);
                __mm_s.push_str(&*literal!("}"));
                __mm_s.push_str(&*subIterString(__iter_sub_iter.clone())?);
                ArcStr::from(__mm_s)
            }
            SIM_ITERATOR_LIST {
                lst: __iter_lst,
                name: __iter_name,
                sub_iter: __iter_sub_iter,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&__iter_name))?);
                __mm_s.push_str(&*literal!(" | list: "));
                __mm_s.push_str(&*List::toString(
                    __iter_lst.clone(),
                    &fnptr!(intString, i32),
                    List::Style::FLAT_CURLY_SHORT.clone(),
                )?);
                __mm_s.push_str(&*literal!("}"));
                __mm_s.push_str(&*subIterString(__iter_sub_iter.clone())?);
                ArcStr::from(__mm_s)
            }
        });
        Ok(r#str)
    }

    pub(crate) fn fromIterator(
        mut iter: &metamodelica::Ref<Iterator::Iterator>,
    ) -> Result<metamodelica::List<metamodelica::Ref<SimIterator>>> {
        let mut sim_iter: metamodelica::List<metamodelica::Ref<SimIterator>> = metamodelica::nil();
        let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator::Iterator>>>;
        let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut addOp: metamodelica::Ref<Operator::NFOperator>;
        let mut mulOp: metamodelica::Ref<Operator::NFOperator>;
        let mut range: metamodelica::Ref<Expression::NFExpression>;
        let mut step: metamodelica::Ref<Expression::NFExpression>;
        let mut size: metamodelica::Ref<Expression::NFExpression>;
        let mut map: Option<metamodelica::Ref<Iterator::Iterator>>;
        let mut lst: metamodelica::List<i32>;
        let mut sub_iter: metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
        )>;
        (names, ranges, maps) = Iterator::getFrames(iter);
        for mut tpl in &*List::zip3(names, ranges, maps) {
            (name, range, map) = tpl.clone();
            sim_iter = (match &*range {
                Expression::RANGE {
                    start: __range_start,
                    step: __range_step,
                    stop: __range_stop,
                    ..
                } => {
                    step = __range_step
                        .clone()
                        .unwrap_or(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }));
                    addOp = Operator::makeAdd(Expression::typeOf(__range_start.clone()));
                    mulOp = Operator::makeMul(Expression::typeOf(__range_start.clone()));
                    size = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![__range_stop.clone()],
                        inv_arguments: list![__range_start.clone()],
                        operator: addOp.clone(),
                    });
                    size = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![size],
                        inv_arguments: list![step.clone()],
                        operator: mulOp,
                    });
                    size = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![
                            size,
                            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 })
                        ],
                        inv_arguments: metamodelica::nil(),
                        operator: addOp,
                    });
                    size = SimplifyExp::simplify(size, false)?;
                    sub_iter = if ((map).is_some()) {
                        subIterators(&(map.ok_or("pattern mismatch")?))?
                    } else {
                        metamodelica::nil()
                    };
                    metamodelica::cons(
                        metamodelica::Ref::new(SimIterator::SIM_ITERATOR_RANGE {
                            name: name,
                            start: __range_start.clone(),
                            step: step,
                            stop: __range_stop.clone(),
                            size: size,
                            sub_iter: sub_iter,
                        }),
                        sim_iter,
                    )
                }
                Expression::ARRAY {
                    literal: __range_literal,
                    ..
                } if (__range_literal.clone()) => {
                    lst = ({
                        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                        for mut e in (var_field!((*range).elements, Expression::NFExpression::ARRAY).clone())
                            .borrow()
                            .iter()
                        {
                            let __x = Expression::integerValue(e.clone())?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    });
                    sub_iter = if ((map).is_some()) {
                        subIterators(&(map.ok_or("pattern mismatch")?))?
                    } else {
                        metamodelica::nil()
                    };
                    metamodelica::cons(
                        metamodelica::Ref::new(SimIterator::SIM_ITERATOR_LIST {
                            name: name,
                            lst: lst.clone(),
                            size: ((lst).len() as i32),
                            sub_iter: sub_iter,
                        }),
                        sim_iter,
                    )
                }
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NSimGenericCall.SimIterator.fromIterator"));
                            __mm_s.push_str(&*literal!(" failed for incorrect iterator domain: "));
                            __mm_s.push_str(&*Expression::toString(range)?);
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            });
        }
        Ok(sim_iter)
    }

    pub(crate) fn subIterators(
        mut iter: &metamodelica::Ref<Iterator::Iterator>,
    ) -> Result<
        metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
        )>,
    > {
        let mut sub_iter: metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
        )> = metamodelica::nil();
        let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut range: metamodelica::Ref<Expression::NFExpression>;
        (names, ranges, _) = Iterator::getFrames(iter);
        for mut tpl in &*List::zip(names, ranges).reverse() {
            (name, range) = tpl.clone();
            sub_iter = (match &*range {
                Expression::ARRAY { .. } => metamodelica::cons(
                    (
                        name,
                        var_field!((*range).elements, Expression::NFExpression::ARRAY).clone(),
                    ),
                    sub_iter,
                ),
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NSimGenericCall.SimIterator.subIterators"));
                            __mm_s.push_str(&*literal!(" failed for incorrect iterator domain: "));
                            __mm_s.push_str(&*Expression::toString(range)?);
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            });
        }
        Ok(sub_iter)
    }

    pub(crate) fn convert(mut iter: &metamodelica::Ref<SimIterator>) -> Result<OldBackendDAE::SimIterator> {
        pub(crate) fn convertSubIterator(
            mut sub_iter: (
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
            ),
        ) -> Result<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
        )> {
            let mut old_sub_iter: (
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
            ) = (
                ComponentRef::toDAE(&(Util::tuple21(sub_iter.clone())))?,
                metamodelica::arrayFromVec(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                        for mut e in (Util::tuple22(sub_iter.clone())).borrow().iter() {
                            let __x = Expression::toDAE(e.clone(), false)?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                    .into_iter()
                    .cloned()
                    .collect(),
                ),
            );
            Ok(old_sub_iter)
        }

        let mut old_iter: OldBackendDAE::SimIterator;
        old_iter = (match &**iter {
            SIM_ITERATOR_RANGE {
                name: __iter_name,
                size: __iter_size,
                start: __iter_start,
                step: __iter_step,
                stop: __iter_stop,
                sub_iter: __iter_sub_iter,
            } => OldBackendDAE::SimIterator::SIM_ITERATOR_RANGE {
                name: ComponentRef::toDAE(metamodelica::AsArg::as_arg(&__iter_name))?,
                start: Expression::toDAE(__iter_start.clone(), false)?,
                step: Expression::toDAE(__iter_step.clone(), false)?,
                stop: Expression::toDAE(__iter_stop.clone(), false)?,
                size: Expression::toDAE(__iter_size.clone(), false)?,
                non_resizable_size: Expression::getInteger(__iter_size.clone(), false)?,
                sub_iter: ({
                    let mut __acc: metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
                    )> = metamodelica::nil();
                    for mut si in (__iter_sub_iter.clone()).into_iter().cloned() {
                        let __x = convertSubIterator(si.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            },
            SIM_ITERATOR_LIST {
                lst: __iter_lst,
                name: __iter_name,
                size: __iter_size,
                sub_iter: __iter_sub_iter,
            } => OldBackendDAE::SimIterator::SIM_ITERATOR_LIST {
                name: ComponentRef::toDAE(metamodelica::AsArg::as_arg(&__iter_name))?,
                lst: __iter_lst.clone(),
                size: __iter_size.clone(),
                sub_iter: ({
                    let mut __acc: metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
                    )> = metamodelica::nil();
                    for mut si in (__iter_sub_iter.clone()).into_iter().cloned() {
                        let __x = convertSubIterator(si.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            },
        });
        Ok(old_iter)
    }
}

/// represents a dependent sub iterator
pub type DependentIterator = (
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
);

pub mod SimBranch {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub(crate) enum SimBranch {
        SIM_BRANCH {
            condition: metamodelica::Ref<Expression::NFExpression>,
            body: metamodelica::List<(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::Ref<Expression::NFExpression>,
            )>,
        },
        SIM_BRANCH_STMT {
            condition: metamodelica::Ref<Expression::NFExpression>,
            body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        },
    }
    impl metamodelica::gc::MMTrace for SimBranch {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                SimBranch::SIM_BRANCH { condition, body } => {
                    metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                    Ok(())
                }
                SimBranch::SIM_BRANCH_STMT { condition, body } => {
                    metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for SimBranch {
        fn default() -> Self {
            Self::SIM_BRANCH {
                condition: Default::default(),
                body: Default::default(),
            }
        }
    }
    pub(crate) use self::SimBranch::{SIM_BRANCH, SIM_BRANCH_STMT};
    pub(crate) fn mapShallow(
        mut branch: metamodelica::Ref<SimBranch>,
        mut func: &dyn ::std::ops::Fn(
            metamodelica::Ref<Expression::NFExpression>,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
    ) -> Result<metamodelica::Ref<SimBranch>> {
        type mapExp = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >;

        let mut branch: metamodelica::Ref<SimBranch> = branch;
        branch = (match &*branch {
            SIM_BRANCH {
                body: __branch_body, ..
            } => {
                assign_variant_field!(branch => SimBranch::SIM_BRANCH; body = ({
                    let mut __acc: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Expression::NFExpression>)> = metamodelica::nil();
                    for mut tpl in (__branch_body.clone()).into_iter().cloned() {
                        let __x = (func(Util::tuple21(tpl.clone()))?, func(Util::tuple22(tpl.clone()))?);
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }));
                branch
            }
            SIM_BRANCH_STMT {
                body: __branch_body, ..
            } => {
                assign_variant_field!(branch => SimBranch::SIM_BRANCH_STMT; body = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
                    for mut stmt in (__branch_body.clone()).into_iter().cloned() {
                        let __x = Statement::mapExp(stmt.clone(), func)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }));
                branch
            }
            _ => branch,
        });
        Ok(branch)
    }

    pub(crate) fn toString(mut branch: &metamodelica::Ref<SimBranch>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        let mut lhs: metamodelica::Ref<Expression::NFExpression>;
        let mut rhs: metamodelica::Ref<Expression::NFExpression>;
        r#str = (match &**branch {
            SIM_BRANCH {
                body: __branch_body,
                condition: __branch_condition,
            } => {
                r#str = if (Expression::isEnd(metamodelica::AsArg::as_arg(&__branch_condition))) {
                    literal!("\n")
                } else {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("if "));
                        __mm_s.push_str(&*Expression::toString(__branch_condition.clone())?);
                        __mm_s.push_str(&*literal!(" then\n"));
                        ArcStr::from(__mm_s)
                    }
                };
                for mut tpl in &*__branch_body.clone() {
                    (lhs, rhs) = tpl.clone();
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!("\t  "));
                        __mm_s.push_str(&*Expression::toString(lhs)?);
                        __mm_s.push_str(&*literal!(" = "));
                        __mm_s.push_str(&*Expression::toString(rhs)?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    };
                }
                r#str
            }
            SIM_BRANCH_STMT {
                body: __branch_body,
                condition: __branch_condition,
            } => {
                r#str = if (Expression::isEnd(metamodelica::AsArg::as_arg(&__branch_condition))) {
                    literal!("\n")
                } else {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("when "));
                        __mm_s.push_str(&*Expression::toString(__branch_condition.clone())?);
                        __mm_s.push_str(&*literal!(" then\n"));
                        ArcStr::from(__mm_s)
                    }
                };
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*List::toString(
                        __branch_body.clone(),
                        &({
                            let __pe_b1 = literal!("");
                            move |__pe_a0| Statement::toString(&__pe_a0, __pe_b1.clone())
                        }),
                        List::Style::NEWLINE_TAB.clone(),
                    )?);
                    ArcStr::from(__mm_s)
                };
                r#str
            }
            _ => literal!("SIM BRANCH NOT KNOWN"),
        });
        Ok(r#str)
    }

    pub(crate) fn fromIfBody(
        mut if_body: &metamodelica::Ref<IfEquationBody::IfEquationBody>,
    ) -> Result<metamodelica::List<metamodelica::Ref<SimBranch>>> {
        let mut branches: metamodelica::List<metamodelica::Ref<SimBranch>>;
        let mut body: metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::Ref<Expression::NFExpression>,
        )> = metamodelica::nil();
        let mut branch: metamodelica::Ref<SimBranch>;
        for mut eqn in &*if_body.then_eqns.clone().reverse() {
            body = metamodelica::cons(
                (
                    (Equation::getLHS(Pointer::access(eqn.clone()))?).ok_or("pattern mismatch")?,
                    (Equation::getRHS(Pointer::access(eqn.clone()))?).ok_or("pattern mismatch")?,
                ),
                body,
            );
        }
        branch = metamodelica::Ref::new(SimBranch::SIM_BRANCH {
            condition: if_body.condition.clone(),
            body: body,
        });
        if (if_body.else_if).is_some() {
            branches = metamodelica::cons(
                branch,
                fromIfBody(&(if_body.else_if.clone().ok_or("pattern mismatch")?))?,
            );
        } else {
            branches = list![branch];
        }
        Ok(branches)
    }

    pub(crate) fn fromWhenBody(
        mut when_body: &metamodelica::Ref<WhenEquationBody::WhenEquationBody>,
    ) -> Result<metamodelica::List<metamodelica::Ref<SimBranch>>> {
        let mut branches: metamodelica::List<metamodelica::Ref<SimBranch>>;
        let mut branch: metamodelica::Ref<SimBranch>;
        branch = metamodelica::Ref::new(SimBranch::SIM_BRANCH_STMT {
            condition: when_body.condition.clone(),
            body: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
                for mut stmt in (when_body.when_stmts.clone()).into_iter().cloned() {
                    let __x = WhenStatement::toStatement(&(stmt.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        });
        if (when_body.else_when).is_some() {
            branches = metamodelica::cons(
                branch,
                fromWhenBody(&(when_body.else_when.clone().ok_or("pattern mismatch")?))?,
            );
        } else {
            branches = list![branch];
        }
        Ok(branches)
    }

    pub(crate) fn convert(mut branch: &metamodelica::Ref<SimBranch>) -> Result<OldSimCode::SimBranch> {
        let mut old_branch: OldSimCode::SimBranch;
        let mut old_condition: Option<metamodelica::Ref<DAE::Exp>>;
        let mut old_body: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)> =
            metamodelica::nil();
        let mut lhs: metamodelica::Ref<Expression::NFExpression>;
        let mut rhs: metamodelica::Ref<Expression::NFExpression>;
        old_branch = (match &**branch {
            SIM_BRANCH {
                body: __branch_body,
                condition: __branch_condition,
            } => {
                old_condition = (match &*__branch_condition.clone() {
                    Expression::END => None,
                    _ => Some(Expression::toDAE(__branch_condition.clone(), false)?),
                });
                for mut tpl in &*__branch_body.clone().reverse() {
                    (lhs, rhs) = tpl.clone();
                    old_body = metamodelica::cons(
                        (Expression::toDAE(lhs, false)?, Expression::toDAE(rhs, false)?),
                        old_body,
                    );
                }
                OldSimCode::SimBranch::SIM_BRANCH {
                    condition: old_condition,
                    body: old_body,
                }
            }
            SIM_BRANCH_STMT {
                body: __branch_body,
                condition: __branch_condition,
            } => {
                old_condition = (match &*__branch_condition.clone() {
                    Expression::END => None,
                    _ => Some(Expression::toDAE(__branch_condition.clone(), false)?),
                });
                OldSimCode::SimBranch::SIM_BRANCH_STMT {
                    condition: old_condition,
                    body: ConvertDAE::convertStatements(__branch_body.clone())?,
                }
            }
            _ => return Err("fail"),
        });
        Ok(old_branch)
    }
}
