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

use crate::BaseModelica;
use crate::NFCall as Call;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFType as Type;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::MutableWeak;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFEquation {
    EQUALITY {
        /// The left hand side expression.
        lhs: metamodelica::Ref<Expression::NFExpression>,
        /// The right hand side expression.
        rhs: metamodelica::Ref<Expression::NFExpression>,
        ty: metamodelica::Ref<Type::NFType>,
        /// Weakly: that scope's sections hold
        ///      the equation.
        scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        source: metamodelica::Ref<DAE::ElementSource>,
        scalarizeMode: ScalarizeMode,
    },
    CONNECT {
        lhs: metamodelica::Ref<Expression::NFExpression>,
        rhs: metamodelica::Ref<Expression::NFExpression>,
        scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    FOR {
        iterator: metamodelica::Ref<InstNode::InstNode>,
        range: Option<metamodelica::Ref<Expression::NFExpression>>,
        /// The body of the for loop.
        body: metamodelica::List<metamodelica::Ref<NFEquation>>,
        scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    IF {
        branches: metamodelica::List<metamodelica::Ref<Branch::Branch>>,
        scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    WHEN {
        branches: metamodelica::List<metamodelica::Ref<Branch::Branch>>,
        scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    ASSERT {
        /// The assert condition.
        condition: metamodelica::Ref<Expression::NFExpression>,
        /// The message to display if the assert fails.
        message: metamodelica::Ref<Expression::NFExpression>,
        /// Error or warning
        level: metamodelica::Ref<Expression::NFExpression>,
        scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    TERMINATE {
        /// The message to display if the terminate triggers.
        message: metamodelica::Ref<Expression::NFExpression>,
        scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    REINIT {
        /// The variable to reinitialize.
        cref: metamodelica::Ref<Expression::NFExpression>,
        /// The new value of the variable.
        reinitExp: metamodelica::Ref<Expression::NFExpression>,
        scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    NORETCALL {
        exp: metamodelica::Ref<Expression::NFExpression>,
        scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
}
impl metamodelica::gc::MMTrace for NFEquation {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFEquation::EQUALITY {
                lhs,
                rhs,
                ty,
                scope,
                source,
                scalarizeMode,
            } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scalarizeMode, __mmv)?;
                Ok(())
            }
            NFEquation::CONNECT {
                lhs,
                rhs,
                scope,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFEquation::FOR {
                iterator,
                range,
                body,
                scope,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(iterator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(range, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFEquation::IF {
                branches,
                scope,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFEquation::WHEN {
                branches,
                scope,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFEquation::ASSERT {
                condition,
                message,
                level,
                scope,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(level, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFEquation::TERMINATE { message, scope, source } => {
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFEquation::REINIT {
                cref,
                reinitExp,
                scope,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(reinitExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFEquation::NORETCALL { exp, scope, source } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for NFEquation {
    fn default() -> Self {
        Self::IF {
            branches: Default::default(),
            scope: Default::default(),
            source: Default::default(),
        }
    }
}
pub use self::NFEquation::{ASSERT, CONNECT, EQUALITY, FOR, IF, NORETCALL, REINIT, TERMINATE, WHEN};
pub mod Branch {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Branch {
        BRANCH {
            condition: metamodelica::Ref<Expression::NFExpression>,
            conditionVar: Variability,
            body: metamodelica::List<metamodelica::Ref<NFEquation>>,
        },
        INVALID_BRANCH {
            branch: metamodelica::Ref<Branch>,
            errors: metamodelica::List<ErrorTypes::TotalMessage>,
        },
    }
    impl metamodelica::gc::MMTrace for Branch {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Branch::BRANCH {
                    condition,
                    conditionVar,
                    body,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(conditionVar, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                    Ok(())
                }
                Branch::INVALID_BRANCH { branch, errors } => {
                    metamodelica::gc::MMTrace::mm_accept(branch, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(errors, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for Branch {
        fn default() -> Self {
            Self::BRANCH {
                condition: Default::default(),
                conditionVar: Default::default(),
                body: Default::default(),
            }
        }
    }
    pub use self::Branch::{BRANCH, INVALID_BRANCH};
    pub(crate) fn mapExp(
        mut branch: metamodelica::Ref<Branch>,
        mut func: &dyn ::std::ops::Fn(
            metamodelica::Ref<Expression::NFExpression>,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
        mut mapBody: bool,
    ) -> Result<metamodelica::Ref<Branch>> {
        let mut branch: metamodelica::Ref<Branch> = branch;
        let mut cond: metamodelica::Ref<Expression::NFExpression>;
        let mut eql: metamodelica::List<metamodelica::Ref<NFEquation>>;
        branch = (match &*branch {
            BRANCH {
                body: __branch_body,
                condition: __branch_condition,
                conditionVar: __branch_conditionVar,
            } => {
                cond = func(__branch_condition.clone())?;
                if mapBody {
                    eql = ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<NFEquation>> = metamodelica::nil();
                        for mut e in (__branch_body.clone()).into_iter().cloned() {
                            let __x = super::mapExp(e.clone(), func)?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    });
                } else {
                    eql = __branch_body.clone();
                }
                metamodelica::Ref::new(Branch::BRANCH {
                    condition: cond,
                    conditionVar: __branch_conditionVar.clone(),
                    body: eql,
                })
            }
            INVALID_BRANCH {
                branch: __branch_branch,
                ..
            } => {
                assign_variant_field!(branch => Branch::INVALID_BRANCH; branch = mapExp(__branch_branch.clone(), func, false)?);
                branch
            }
            _ => branch,
        });
        Ok(branch)
    }

    pub(crate) fn isEmpty<'__b>(mut branch: &'__b metamodelica::Ref<Branch>) -> bool {
        '__tco: loop {
            match &**branch {
                BRANCH { .. } => return (var_field!((**branch).body, Branch::BRANCH)).is_empty(),
                INVALID_BRANCH { .. } => {
                    branch = var_field!((**branch).branch, Branch::INVALID_BRANCH);
                    continue '__tco;
                }
            }
        }
    }

    pub(crate) fn sizeOf(mut branch: &metamodelica::Ref<Branch>) -> i32 {
        let mut size: i32;
        size = (match &**branch {
            BRANCH {
                body: __branch_body, ..
            } => sizeOfList(metamodelica::AsArg::as_arg(&__branch_body)),
            _ => 0,
        });
        size
    }

    pub(crate) fn toStream<'__b>(
        mut branch: &'__b metamodelica::Ref<Branch>,
        mut header: &'__b ArcStr,
        mut potentialElse: bool,
        mut indent: &'__b ArcStr,
        mut s: IOStream::IOStream,
    ) -> Result<IOStream::IOStream> {
        '__tco: loop {
            match &**branch {
                BRANCH { .. } => {
                    if potentialElse && Expression::isTrue(var_field!((**branch).condition, Branch::BRANCH)) {
                        s = IOStream::append(s, literal!("else\n"))?;
                    } else {
                        s = IOStream::append(s, header.clone())?;
                        s = IOStream::append(
                            s,
                            Expression::toString(var_field!((**branch).condition, Branch::BRANCH).clone())?,
                        )?;
                        s = IOStream::append(s, literal!(" then\n"))?;
                    }
                    return Ok(toStreamList(
                        var_field!((**branch).body, Branch::BRANCH),
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*indent);
                            __mm_s.push_str(&*literal!("  "));
                            ArcStr::from(__mm_s)
                        },
                        s,
                    )?);
                }
                INVALID_BRANCH { .. } => {
                    (branch, header, potentialElse, indent, s) = (
                        var_field!((**branch).branch, Branch::INVALID_BRANCH),
                        header,
                        potentialElse,
                        indent,
                        s,
                    );
                    continue '__tco;
                }
            }
        }
    }

    pub(crate) fn toFlatStream<'__b>(
        mut branch: &'__b metamodelica::Ref<Branch>,
        mut header: &'__b ArcStr,
        mut format: BaseModelica::OutputFormat,
        mut potentialElse: bool,
        mut indent: &'__b ArcStr,
        mut s: IOStream::IOStream,
    ) -> Result<IOStream::IOStream> {
        '__tco: loop {
            match &**branch {
                BRANCH { .. } => {
                    if potentialElse && Expression::isTrue(var_field!((**branch).condition, Branch::BRANCH)) {
                        s = IOStream::append(s, literal!("else\n"))?;
                    } else {
                        s = IOStream::append(s, header.clone())?;
                        s = IOStream::append(
                            s,
                            Expression::toFlatString(var_field!((**branch).condition, Branch::BRANCH).clone(), format)?,
                        )?;
                        s = IOStream::append(s, literal!(" then\n"))?;
                    }
                    return Ok(toFlatStreamList(
                        var_field!((**branch).body, Branch::BRANCH),
                        format,
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*indent);
                            __mm_s.push_str(&*literal!("  "));
                            ArcStr::from(__mm_s)
                        },
                        s,
                    )?);
                }
                INVALID_BRANCH { .. } => {
                    (branch, header, format, potentialElse, indent, s) = (
                        var_field!((**branch).branch, Branch::INVALID_BRANCH),
                        header,
                        format,
                        potentialElse,
                        indent,
                        s,
                    );
                    continue '__tco;
                }
            }
        }
    }

    pub fn toString(mut branch: &metamodelica::Ref<Branch>, mut indent: &ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        let mut s: IOStream::IOStream;
        s = IOStream::create(
            literal!("NFEquation.Branch.toString"),
            openmodelica_util::IOStream::IOStreamType::LIST,
        )?;
        s = toStream(branch, &(literal!("")), false, indent, s)?;
        r#str = IOStream::string(&s)?;
        IOStream::delete(&s)?;
        Ok(r#str)
    }

    pub(crate) fn triggerErrors(mut branch: &metamodelica::Ref<Branch>) -> Result<()> {
        let () = (match &**branch {
            INVALID_BRANCH {
                errors: __branch_errors,
                ..
            } => {
                Error::addTotalMessages(metamodelica::AsArg::as_arg(&__branch_errors))?;
                return Err("fail");
            }
            _ => (),
        });
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum ScalarizeMode {
    DONT_SCALARIZE = 1,
    SCALARIZE = 2,
    NO_PREFERENCE = 3,
}
impl PartialOrd for ScalarizeMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ScalarizeMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for ScalarizeMode {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) fn makeEquality(
    mut lhs: metamodelica::Ref<Expression::NFExpression>,
    mut rhs: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut src: metamodelica::Ref<DAE::ElementSource>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut scalarizeMode: ScalarizeMode,
) -> metamodelica::Ref<NFEquation> {
    let mut eq: metamodelica::Ref<NFEquation>;
    eq = metamodelica::Ref::new(NFEquation::EQUALITY {
        lhs: lhs,
        rhs: rhs,
        ty: ty,
        scope: NFInstNode::InstNode::identityCell(scope),
        source: src,
        scalarizeMode: scalarizeMode,
    });
    eq
}

pub(crate) fn makeCrefEquality(
    mut lhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut rhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut src: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<NFEquation>> {
    let mut eq: metamodelica::Ref<NFEquation>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    e1 = Expression::fromCref(lhsCref, false)?;
    e2 = Expression::fromCref(rhsCref, false)?;
    eq = makeEquality(
        e1.clone(),
        e2,
        Expression::typeOf(e1),
        src,
        scope,
        ScalarizeMode::NO_PREFERENCE.clone(),
    );
    Ok(eq)
}

pub(crate) fn makeBranch(
    mut condition: metamodelica::Ref<Expression::NFExpression>,
    mut body: metamodelica::List<metamodelica::Ref<NFEquation>>,
    mut condVar: Variability,
) -> metamodelica::Ref<Branch::Branch> {
    let mut branch: metamodelica::Ref<Branch::Branch>;
    branch = metamodelica::Ref::new(Branch::Branch::BRANCH {
        condition: condition,
        conditionVar: condVar,
        body: body,
    });
    branch
}

pub(crate) fn makeIf(
    mut branches: metamodelica::List<metamodelica::Ref<Branch::Branch>>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut src: metamodelica::Ref<DAE::ElementSource>,
) -> metamodelica::Ref<NFEquation> {
    let mut eq: metamodelica::Ref<NFEquation>;
    eq = metamodelica::Ref::new(NFEquation::IF {
        branches: branches,
        scope: NFInstNode::InstNode::identityCell(scope),
        source: src,
    });
    eq
}

pub fn source(mut eq: &metamodelica::Ref<NFEquation>) -> metamodelica::Ref<DAE::ElementSource> {
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    source = (match &**eq {
        EQUALITY {
            source: __eq_source, ..
        } => __eq_source.clone(),
        CONNECT {
            source: __eq_source, ..
        } => __eq_source.clone(),
        FOR {
            source: __eq_source, ..
        } => __eq_source.clone(),
        IF {
            source: __eq_source, ..
        } => __eq_source.clone(),
        WHEN {
            source: __eq_source, ..
        } => __eq_source.clone(),
        ASSERT {
            source: __eq_source, ..
        } => __eq_source.clone(),
        TERMINATE {
            source: __eq_source, ..
        } => __eq_source.clone(),
        REINIT {
            source: __eq_source, ..
        } => __eq_source.clone(),
        NORETCALL {
            source: __eq_source, ..
        } => __eq_source.clone(),
    });
    source
}

pub(crate) fn setSource(
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut eq: metamodelica::Ref<NFEquation>,
) -> metamodelica::Ref<NFEquation> {
    let mut eq: metamodelica::Ref<NFEquation> = eq;
    let () = (match &*eq {
        EQUALITY { .. } => {
            assign_variant_field!(eq => NFEquation::EQUALITY; source = source);
            ()
        }
        CONNECT { .. } => {
            assign_variant_field!(eq => NFEquation::CONNECT; source = source);
            ()
        }
        FOR { .. } => {
            assign_variant_field!(eq => NFEquation::FOR; source = source);
            ()
        }
        IF { .. } => {
            assign_variant_field!(eq => NFEquation::IF; source = source);
            ()
        }
        WHEN { .. } => {
            assign_variant_field!(eq => NFEquation::WHEN; source = source);
            ()
        }
        ASSERT { .. } => {
            assign_variant_field!(eq => NFEquation::ASSERT; source = source);
            ()
        }
        TERMINATE { .. } => {
            assign_variant_field!(eq => NFEquation::TERMINATE; source = source);
            ()
        }
        REINIT { .. } => {
            assign_variant_field!(eq => NFEquation::REINIT; source = source);
            ()
        }
        NORETCALL { .. } => {
            assign_variant_field!(eq => NFEquation::NORETCALL; source = source);
            ()
        }
    });
    eq
}

pub fn scope(mut eq: &metamodelica::Ref<NFEquation>) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut scope: metamodelica::Ref<InstNode::InstNode>;
    scope = NFInstNode::InstNode::fromCell(scopeCell(eq))?;
    Ok(scope)
}

pub(crate) fn scopeCell(
    mut eq: &metamodelica::Ref<NFEquation>,
) -> Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>> {
    let mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    scope = (match &**eq {
        EQUALITY { scope: __eq_scope, .. } => __eq_scope.clone(),
        CONNECT { scope: __eq_scope, .. } => __eq_scope.clone(),
        FOR { scope: __eq_scope, .. } => __eq_scope.clone(),
        IF { scope: __eq_scope, .. } => __eq_scope.clone(),
        WHEN { scope: __eq_scope, .. } => __eq_scope.clone(),
        ASSERT { scope: __eq_scope, .. } => __eq_scope.clone(),
        TERMINATE { scope: __eq_scope, .. } => __eq_scope.clone(),
        REINIT { scope: __eq_scope, .. } => __eq_scope.clone(),
        NORETCALL { scope: __eq_scope, .. } => __eq_scope.clone(),
    });
    scope
}

pub(crate) fn info(mut eq: &metamodelica::Ref<NFEquation>) -> SourceInfo {
    let mut info: SourceInfo = ElementSource::getInfo(source(eq));
    info
}

pub type ApplyFn = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFEquation>) -> Result<()> + 'static>;

pub(crate) fn applyList(
    mut eql: &metamodelica::List<metamodelica::Ref<NFEquation>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFEquation>) -> Result<()>,
) -> Result<()> {
    for mut eq in &**eql {
        apply(eq.clone(), func)?;
    }
    Ok(())
}

pub(crate) fn apply(
    mut eq: metamodelica::Ref<NFEquation>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFEquation>) -> Result<()>,
) -> Result<()> {
    let () = (match &*eq {
        FOR { body: __eq_body, .. } => {
            for mut e in &*__eq_body.clone() {
                apply(e.clone(), func)?;
            }
            ()
        }
        IF {
            branches: __eq_branches,
            ..
        } => {
            for mut b in &*__eq_branches.clone() {
                let () = (match &*b.clone() {
                    Branch::BRANCH { body: __b_body, .. } => {
                        for mut e in &*__b_body.clone() {
                            apply(e.clone(), func)?;
                        }
                        ()
                    }
                    _ => (),
                });
            }
            ()
        }
        WHEN {
            branches: __eq_branches,
            ..
        } => {
            for mut b in &*__eq_branches.clone() {
                let () = (match &*b.clone() {
                    Branch::BRANCH { body: __b_body, .. } => {
                        for mut e in &*__b_body.clone() {
                            apply(e.clone(), func)?;
                        }
                        ()
                    }
                    _ => (),
                });
            }
            ()
        }
        _ => (),
    });
    func(eq)?;
    Ok(())
}

pub type MapFn = std::sync::Arc<
    dyn ::std::ops::Fn(metamodelica::Ref<NFEquation>) -> Result<metamodelica::Ref<NFEquation>> + 'static,
>;

pub(crate) fn map(
    mut eq: metamodelica::Ref<NFEquation>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFEquation>) -> Result<metamodelica::Ref<NFEquation>>,
) -> Result<metamodelica::Ref<NFEquation>> {
    let mut eq: metamodelica::Ref<NFEquation> = eq;
    let () = (match &*eq {
        FOR { body: __eq_body, .. } => {
            assign_variant_field!(eq => NFEquation::FOR; body = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFEquation>> = metamodelica::nil();
                for mut e in (__eq_body.clone()).into_iter().cloned() {
                    let __x = map(e.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        IF {
            branches: __eq_branches,
            ..
        } => {
            assign_variant_field!(eq => NFEquation::IF; branches = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Branch::Branch>> = metamodelica::nil();
                for mut b in (__eq_branches.clone()).into_iter().cloned() {
                    let __x = (match &*b.clone() {
                Branch::BRANCH { body: __b_body, .. } => {
                    assign_variant_field!(b => Branch::Branch::BRANCH; body = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFEquation>> = metamodelica::nil();
                for mut e in (__b_body.clone()).into_iter().cloned() {
                    let __x = map(e.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
                    b.clone()
                },
                _ => b.clone(),
            });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        WHEN {
            branches: __eq_branches,
            ..
        } => {
            assign_variant_field!(eq => NFEquation::WHEN; branches = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Branch::Branch>> = metamodelica::nil();
                for mut b in (__eq_branches.clone()).into_iter().cloned() {
                    let __x = (match &*b.clone() {
                Branch::BRANCH { body: __b_body, .. } => {
                    assign_variant_field!(b => Branch::Branch::BRANCH; body = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFEquation>> = metamodelica::nil();
                for mut e in (__b_body.clone()).into_iter().cloned() {
                    let __x = map(e.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
                    b.clone()
                },
                _ => b.clone(),
            });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    eq = func(eq)?;
    Ok(eq)
}

pub(crate) fn applyExpList(
    mut eq: &metamodelica::List<metamodelica::Ref<NFEquation>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    for mut e in &**eq {
        applyExp(metamodelica::AsArg::as_arg(&e), func)?;
    }
    Ok(())
}

pub(crate) fn applyExp(
    mut eq: &metamodelica::Ref<NFEquation>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (match &**eq {
        EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => {
            func(__eq_lhs.clone())?;
            func(__eq_rhs.clone())?;
            ()
        }
        CONNECT {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => {
            func(__eq_lhs.clone())?;
            func(__eq_rhs.clone())?;
            ()
        }
        FOR {
            body: __eq_body,
            range: __eq_range,
            ..
        } => {
            applyExpList(metamodelica::AsArg::as_arg(&__eq_body), func)?;
            if (__eq_range).is_some() {
                func(__eq_range.clone().ok_or("pattern mismatch")?)?;
            }
            ()
        }
        IF {
            branches: __eq_branches,
            ..
        } => {
            for mut b in &*__eq_branches.clone() {
                let () = (match &*b.clone() {
                    Branch::BRANCH {
                        body: __b_body,
                        condition: __b_condition,
                        ..
                    } => {
                        func(__b_condition.clone())?;
                        applyExpList(metamodelica::AsArg::as_arg(&__b_body), func)?;
                        ()
                    }
                    _ => (),
                });
            }
            ()
        }
        WHEN {
            branches: __eq_branches,
            ..
        } => {
            for mut b in &*__eq_branches.clone() {
                let () = (match &*b.clone() {
                    Branch::BRANCH {
                        body: __b_body,
                        condition: __b_condition,
                        ..
                    } => {
                        func(__b_condition.clone())?;
                        applyExpList(metamodelica::AsArg::as_arg(&__b_body), func)?;
                        ()
                    }
                    _ => (),
                });
            }
            ()
        }
        ASSERT {
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            ..
        } => {
            func(__eq_condition.clone())?;
            func(__eq_message.clone())?;
            func(__eq_level.clone())?;
            ()
        }
        TERMINATE {
            message: __eq_message, ..
        } => {
            func(__eq_message.clone())?;
            ()
        }
        REINIT {
            cref: __eq_cref,
            reinitExp: __eq_reinitExp,
            ..
        } => {
            func(__eq_cref.clone())?;
            func(__eq_reinitExp.clone())?;
            ()
        }
        NORETCALL { exp: __eq_exp, .. } => {
            func(__eq_exp.clone())?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn applyExpShallow(
    mut eq: &metamodelica::Ref<NFEquation>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (match &**eq {
        EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => {
            func(__eq_lhs.clone())?;
            func(__eq_rhs.clone())?;
            ()
        }
        CONNECT {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => {
            func(__eq_lhs.clone())?;
            func(__eq_rhs.clone())?;
            ()
        }
        FOR { range: __eq_range, .. } => {
            if (__eq_range).is_some() {
                func(__eq_range.clone().ok_or("pattern mismatch")?)?;
            }
            ()
        }
        IF {
            branches: __eq_branches,
            ..
        } => {
            for mut b in &*__eq_branches.clone() {
                let () = (match &*b.clone() {
                    Branch::BRANCH {
                        condition: __b_condition,
                        ..
                    } => {
                        func(__b_condition.clone())?;
                        ()
                    }
                    _ => (),
                });
            }
            ()
        }
        WHEN {
            branches: __eq_branches,
            ..
        } => {
            for mut b in &*__eq_branches.clone() {
                let () = (match &*b.clone() {
                    Branch::BRANCH {
                        condition: __b_condition,
                        ..
                    } => {
                        func(__b_condition.clone())?;
                        ()
                    }
                    _ => (),
                });
            }
            ()
        }
        ASSERT {
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            ..
        } => {
            func(__eq_condition.clone())?;
            func(__eq_message.clone())?;
            func(__eq_level.clone())?;
            ()
        }
        TERMINATE {
            message: __eq_message, ..
        } => {
            func(__eq_message.clone())?;
            ()
        }
        REINIT {
            cref: __eq_cref,
            reinitExp: __eq_reinitExp,
            ..
        } => {
            func(__eq_cref.clone())?;
            func(__eq_reinitExp.clone())?;
            ()
        }
        NORETCALL { exp: __eq_exp, .. } => {
            func(__eq_exp.clone())?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub type MapExpFn = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<Expression::NFExpression>,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
        + 'static,
>;

pub(crate) fn mapExpList(
    mut eql: metamodelica::List<metamodelica::Ref<NFEquation>>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFEquation>>> {
    let mut eql: metamodelica::List<metamodelica::Ref<NFEquation>> = eql;
    eql = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFEquation>> = metamodelica::nil();
        for mut eq in (eql).into_iter().cloned() {
            let __x = mapExp(eq.clone(), func)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(eql)
}

pub(crate) fn mapExp(
    mut eq: metamodelica::Ref<NFEquation>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NFEquation>> {
    let mut eq: metamodelica::Ref<NFEquation> = eq;
    eq = (match &*eq {
        EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            scalarizeMode: __eq_scalarizeMode,
            scope: __eq_scope,
            source: __eq_source,
            ty: __eq_ty,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__eq_lhs.clone())?;
            e2 = func(__eq_rhs.clone())?;
            if (referenceEq(&*(&*e1), &*(__eq_lhs.clone())) && referenceEq(&*(&*e2), &*(__eq_rhs.clone()))) {
                eq
            } else {
                metamodelica::Ref::new(NFEquation::EQUALITY {
                    lhs: e1,
                    rhs: e2,
                    ty: __eq_ty.clone(),
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                    scalarizeMode: __eq_scalarizeMode.clone(),
                })
            }
        }
        CONNECT {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__eq_lhs.clone())?;
            e2 = func(__eq_rhs.clone())?;
            if (referenceEq(&*(&*e1), &*(__eq_lhs.clone())) && referenceEq(&*(&*e2), &*(__eq_rhs.clone()))) {
                eq
            } else {
                metamodelica::Ref::new(NFEquation::CONNECT {
                    lhs: e1,
                    rhs: e2,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                })
            }
        }
        FOR { body: __eq_body, .. } => {
            assign_variant_field!(eq => NFEquation::FOR;
                        body = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFEquation>> = metamodelica::nil();
                for mut e in (__eq_body.clone()).into_iter().cloned() {
                    let __x = mapExp(e.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        range = Util::applyOption(var_field!((*eq).range, NFEquation::FOR).clone(), func)?
                    );
            eq
        }
        IF {
            branches: __eq_branches,
            ..
        } => {
            assign_variant_field!(eq => NFEquation::IF; branches = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Branch::Branch>> = metamodelica::nil();
                for mut b in (__eq_branches.clone()).into_iter().cloned() {
                    let __x = Branch::mapExp(b.clone(), func, true)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            eq
        }
        WHEN {
            branches: __eq_branches,
            ..
        } => {
            assign_variant_field!(eq => NFEquation::WHEN; branches = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Branch::Branch>> = metamodelica::nil();
                for mut b in (__eq_branches.clone()).into_iter().cloned() {
                    let __x = Branch::mapExp(b.clone(), func, true)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            eq
        }
        ASSERT {
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut e3: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__eq_condition.clone())?;
            e2 = func(__eq_message.clone())?;
            e3 = func(__eq_level.clone())?;
            if (referenceEq(&*(&*e1), &*(__eq_condition.clone()))
                && referenceEq(&*(&*e2), &*(__eq_message.clone()))
                && referenceEq(&*(&*e3), &*(__eq_level.clone())))
            {
                eq
            } else {
                metamodelica::Ref::new(NFEquation::ASSERT {
                    condition: e1,
                    message: e2,
                    level: e3,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                })
            }
        }
        TERMINATE {
            message: __eq_message,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__eq_message.clone())?;
            if (referenceEq(&*(&*e1), &*(__eq_message.clone()))) {
                eq
            } else {
                metamodelica::Ref::new(NFEquation::TERMINATE {
                    message: e1,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                })
            }
        }
        REINIT {
            cref: __eq_cref,
            reinitExp: __eq_reinitExp,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__eq_cref.clone())?;
            e2 = func(__eq_reinitExp.clone())?;
            if (referenceEq(&*(&*e1), &*(__eq_cref.clone())) && referenceEq(&*(&*e2), &*(__eq_reinitExp.clone()))) {
                eq
            } else {
                metamodelica::Ref::new(NFEquation::REINIT {
                    cref: e1,
                    reinitExp: e2,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                })
            }
        }
        NORETCALL {
            exp: __eq_exp,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__eq_exp.clone())?;
            if (referenceEq(&*(&*e1), &*(__eq_exp.clone()))) {
                eq
            } else {
                metamodelica::Ref::new(NFEquation::NORETCALL {
                    exp: e1,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                })
            }
        }
        _ => eq,
    });
    Ok(eq)
}

pub(crate) fn mapExpShallow(
    mut eq: metamodelica::Ref<NFEquation>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NFEquation>> {
    let mut eq: metamodelica::Ref<NFEquation> = eq;
    eq = (match &*eq {
        EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            scalarizeMode: __eq_scalarizeMode,
            scope: __eq_scope,
            source: __eq_source,
            ty: __eq_ty,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__eq_lhs.clone())?;
            e2 = func(__eq_rhs.clone())?;
            if (referenceEq(&*(&*e1), &*(__eq_lhs.clone())) && referenceEq(&*(&*e2), &*(__eq_rhs.clone()))) {
                eq
            } else {
                metamodelica::Ref::new(NFEquation::EQUALITY {
                    lhs: e1,
                    rhs: e2,
                    ty: __eq_ty.clone(),
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                    scalarizeMode: __eq_scalarizeMode.clone(),
                })
            }
        }
        CONNECT {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__eq_lhs.clone())?;
            e2 = func(__eq_rhs.clone())?;
            if (referenceEq(&*(&*e1), &*(__eq_lhs.clone())) && referenceEq(&*(&*e2), &*(__eq_rhs.clone()))) {
                eq
            } else {
                metamodelica::Ref::new(NFEquation::CONNECT {
                    lhs: e1,
                    rhs: e2,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                })
            }
        }
        FOR { range: __eq_range, .. } => {
            assign_variant_field!(eq => NFEquation::FOR; range = Util::applyOption(__eq_range.clone(), func)?);
            eq
        }
        IF {
            branches: __eq_branches,
            ..
        } => {
            assign_variant_field!(eq => NFEquation::IF; branches = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Branch::Branch>> = metamodelica::nil();
                for mut b in (__eq_branches.clone()).into_iter().cloned() {
                    let __x = Branch::mapExp(b.clone(), func, false)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            eq
        }
        WHEN {
            branches: __eq_branches,
            ..
        } => {
            assign_variant_field!(eq => NFEquation::WHEN; branches = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Branch::Branch>> = metamodelica::nil();
                for mut b in (__eq_branches.clone()).into_iter().cloned() {
                    let __x = Branch::mapExp(b.clone(), func, false)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            eq
        }
        ASSERT {
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut e3: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__eq_condition.clone())?;
            e2 = func(__eq_message.clone())?;
            e3 = func(__eq_level.clone())?;
            if (referenceEq(&*(&*e1), &*(__eq_condition.clone()))
                && referenceEq(&*(&*e2), &*(__eq_message.clone()))
                && referenceEq(&*(&*e3), &*(__eq_level.clone())))
            {
                eq
            } else {
                metamodelica::Ref::new(NFEquation::ASSERT {
                    condition: e1,
                    message: e2,
                    level: e3,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                })
            }
        }
        TERMINATE {
            message: __eq_message,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__eq_message.clone())?;
            if (referenceEq(&*(&*e1), &*(__eq_message.clone()))) {
                eq
            } else {
                metamodelica::Ref::new(NFEquation::TERMINATE {
                    message: e1,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                })
            }
        }
        REINIT {
            cref: __eq_cref,
            reinitExp: __eq_reinitExp,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__eq_cref.clone())?;
            e2 = func(__eq_reinitExp.clone())?;
            if (referenceEq(&*(&*e1), &*(__eq_cref.clone())) && referenceEq(&*(&*e2), &*(__eq_reinitExp.clone()))) {
                eq
            } else {
                metamodelica::Ref::new(NFEquation::REINIT {
                    cref: e1,
                    reinitExp: e2,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                })
            }
        }
        NORETCALL {
            exp: __eq_exp,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__eq_exp.clone())?;
            if (referenceEq(&*(&*e1), &*(__eq_exp.clone()))) {
                eq
            } else {
                metamodelica::Ref::new(NFEquation::NORETCALL {
                    exp: e1,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                })
            }
        }
        _ => eq,
    });
    Ok(eq)
}

pub(crate) fn foldExpList<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut eq: &metamodelica::List<metamodelica::Ref<NFEquation>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    for mut e in &**eq {
        arg = foldExp(metamodelica::AsArg::as_arg(&e), func, arg)?;
    }
    Ok(arg)
}

pub(crate) fn foldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut eq: &metamodelica::Ref<NFEquation>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    let () = (match &**eq {
        EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => {
            arg = func(__eq_lhs.clone(), arg)?;
            arg = func(__eq_rhs.clone(), arg)?;
            ()
        }
        CONNECT {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => {
            arg = func(__eq_lhs.clone(), arg)?;
            arg = func(__eq_rhs.clone(), arg)?;
            ()
        }
        FOR {
            body: __eq_body,
            range: __eq_range,
            ..
        } => {
            arg = foldExpList(metamodelica::AsArg::as_arg(&__eq_body), func, arg)?;
            if (__eq_range).is_some() {
                arg = func(__eq_range.clone().ok_or("pattern mismatch")?, arg)?;
            }
            ()
        }
        IF {
            branches: __eq_branches,
            ..
        } => {
            for mut b in &*__eq_branches.clone() {
                let () = (match &*b.clone() {
                    Branch::BRANCH {
                        body: __b_body,
                        condition: __b_condition,
                        ..
                    } => {
                        arg = func(__b_condition.clone(), arg)?;
                        arg = foldExpList(metamodelica::AsArg::as_arg(&__b_body), func, arg)?;
                        ()
                    }
                    _ => (),
                });
            }
            ()
        }
        WHEN {
            branches: __eq_branches,
            ..
        } => {
            for mut b in &*__eq_branches.clone() {
                let () = (match &*b.clone() {
                    Branch::BRANCH {
                        body: __b_body,
                        condition: __b_condition,
                        ..
                    } => {
                        arg = func(__b_condition.clone(), arg)?;
                        arg = foldExpList(metamodelica::AsArg::as_arg(&__b_body), func, arg)?;
                        ()
                    }
                    _ => (),
                });
            }
            ()
        }
        ASSERT {
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            ..
        } => {
            arg = func(__eq_condition.clone(), arg)?;
            arg = func(__eq_message.clone(), arg)?;
            arg = func(__eq_level.clone(), arg)?;
            ()
        }
        TERMINATE {
            message: __eq_message, ..
        } => {
            arg = func(__eq_message.clone(), arg)?;
            ()
        }
        REINIT {
            cref: __eq_cref,
            reinitExp: __eq_reinitExp,
            ..
        } => {
            arg = func(__eq_cref.clone(), arg)?;
            arg = func(__eq_reinitExp.clone(), arg)?;
            ()
        }
        NORETCALL { exp: __eq_exp, .. } => {
            arg = func(__eq_exp.clone(), arg)?;
            ()
        }
        _ => (),
    });
    Ok(arg)
}

pub(crate) fn contains(
    mut eq: metamodelica::Ref<NFEquation>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFEquation>) -> Result<bool>,
) -> Result<bool> {
    pub type PredFn = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFEquation>) -> Result<bool> + 'static>;

    let mut res: bool;
    if func(eq.clone())? {
        res = true;
        return Ok(res);
    }
    res = (match &*eq {
        FOR { body: __eq_body, .. } => containsList(metamodelica::AsArg::as_arg(&__eq_body), func)?,
        IF {
            branches: __eq_branches,
            ..
        } => {
            for mut b in &*__eq_branches.clone() {
                let () = (match &*b.clone() {
                    Branch::BRANCH { body: __b_body, .. } => {
                        if containsList(metamodelica::AsArg::as_arg(&__b_body), func)? {
                            res = true;
                            return Ok(res);
                        }
                        ()
                    }
                    _ => (),
                });
            }
            false
        }
        WHEN {
            branches: __eq_branches,
            ..
        } => {
            for mut b in &*__eq_branches.clone() {
                let () = (match &*b.clone() {
                    Branch::BRANCH { body: __b_body, .. } => {
                        if containsList(metamodelica::AsArg::as_arg(&__b_body), func)? {
                            res = true;
                            return Ok(res);
                        }
                        ()
                    }
                    _ => (),
                });
            }
            false
        }
        _ => false,
    });
    Ok(res)
}

pub(crate) fn containsList(
    mut eql: &metamodelica::List<metamodelica::Ref<NFEquation>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFEquation>) -> Result<bool>,
) -> Result<bool> {
    pub type PredFn = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFEquation>) -> Result<bool> + 'static>;

    let mut res: bool;
    for mut eq in &**eql {
        if contains(eq.clone(), func)? {
            res = true;
            return Ok(res);
        }
    }
    res = false;
    Ok(res)
}

pub(crate) fn containsExp(
    mut eq: &metamodelica::Ref<NFEquation>,
    mut r#fn: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type Predicate =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    res = (match &**eq {
        EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => r#fn(__eq_lhs.clone())? || r#fn(__eq_rhs.clone())?,
        CONNECT {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => r#fn(__eq_lhs.clone())? || r#fn(__eq_rhs.clone())?,
        FOR {
            body: __eq_body,
            range: __eq_range,
            ..
        } => {
            res = if ((__eq_range).is_some()) {
                r#fn(__eq_range.clone().ok_or("pattern mismatch")?)?
            } else {
                false
            };
            if !(res) {
                res = containsExpList(metamodelica::AsArg::as_arg(&__eq_body), r#fn)?;
            }
            res
        }
        IF {
            branches: __eq_branches,
            ..
        } => {
            res = false;
            for mut b in &*__eq_branches.clone() {
                let () = (match &*b.clone() {
                    Branch::BRANCH {
                        body: __b_body,
                        condition: __b_condition,
                        ..
                    } => {
                        if r#fn(__b_condition.clone())? {
                            res = true;
                            return Ok(res);
                        }
                        if containsExpList(metamodelica::AsArg::as_arg(&__b_body), r#fn)? {
                            res = true;
                            return Ok(res);
                        }
                        ()
                    }
                    _ => (),
                });
            }
            res
        }
        WHEN {
            branches: __eq_branches,
            ..
        } => {
            res = false;
            for mut b in &*__eq_branches.clone() {
                let () = (match &*b.clone() {
                    Branch::BRANCH {
                        body: __b_body,
                        condition: __b_condition,
                        ..
                    } => {
                        if r#fn(__b_condition.clone())? {
                            res = true;
                            return Ok(res);
                        }
                        if containsExpList(metamodelica::AsArg::as_arg(&__b_body), r#fn)? {
                            res = true;
                            return Ok(res);
                        }
                        ()
                    }
                    _ => (),
                });
            }
            res
        }
        ASSERT {
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            ..
        } => r#fn(__eq_condition.clone())? || r#fn(__eq_message.clone())? || r#fn(__eq_level.clone())?,
        TERMINATE {
            message: __eq_message, ..
        } => r#fn(__eq_message.clone())?,
        REINIT {
            cref: __eq_cref,
            reinitExp: __eq_reinitExp,
            ..
        } => r#fn(__eq_cref.clone())? || r#fn(__eq_reinitExp.clone())?,
        NORETCALL { exp: __eq_exp, .. } => r#fn(__eq_exp.clone())?,
        _ => false,
    });
    Ok(res)
}

pub(crate) fn containsExpList(
    mut eql: &metamodelica::List<metamodelica::Ref<NFEquation>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type Predicate =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    for mut eq in &**eql {
        if containsExp(metamodelica::AsArg::as_arg(&eq), func)? {
            res = true;
            return Ok(res);
        }
    }
    res = false;
    Ok(res)
}

pub(crate) fn replaceIteratorList(
    mut eql: metamodelica::List<metamodelica::Ref<NFEquation>>,
    mut iterator: &metamodelica::Ref<InstNode::InstNode>,
    mut value: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::List<metamodelica::Ref<NFEquation>>> {
    let mut eql: metamodelica::List<metamodelica::Ref<NFEquation>> = eql;
    eql = mapExpList(
        eql,
        &({
            let __pe_b1 = iterator.clone();
            let __pe_b2 = value.clone();
            move |__pe_a0| Expression::replaceIterator(__pe_a0, &__pe_b1, &__pe_b2)
        }),
    )?;
    Ok(eql)
}

pub(crate) fn isArrayEquality(mut eq: &metamodelica::Ref<NFEquation>) -> bool {
    let mut isArray: bool;
    isArray = (match &**eq {
        EQUALITY { ty: __eq_ty, .. } => Type::isArray(metamodelica::AsArg::as_arg(&__eq_ty)),
        _ => false,
    });
    isArray
}

pub(crate) fn isConnect(mut eq: &metamodelica::Ref<NFEquation>) -> bool {
    let mut isConnect: bool;
    isConnect = (match &**eq {
        CONNECT { .. } => true,
        _ => false,
    });
    isConnect
}

pub(crate) fn isConnection(mut eq: &metamodelica::Ref<NFEquation>) -> bool {
    let mut res: bool;
    let mut call: metamodelica::Ref<Call::NFCall>;
    res = (::match_deref::match_deref! { match eq {
        Deref @ CONNECT { .. } => true,
        Deref @ NORETCALL { exp: Deref @ Expression::CALL { call: __esc_call }, .. } => {
            call = (*__esc_call).clone();
            Call::isConnectionsOperator(metamodelica::AsArg::as_arg(&call))
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub(crate) fn sizeOfList(mut eqs: &metamodelica::List<metamodelica::Ref<NFEquation>>) -> i32 {
    let mut size: i32 = 0;
    for mut eq in &**eqs {
        size = size + sizeOf(metamodelica::AsArg::as_arg(&eq));
    }
    size
}

pub fn sizeOf(mut eq: &metamodelica::Ref<NFEquation>) -> i32 {
    let mut size: i32 = 0;
    size = 'mc: {
        let __mc_input = &**eq;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ EQUALITY { .. } => {
                    Ok(Type::sizeOf(var_field!((**eq).ty, NFEquation::EQUALITY), false)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ CONNECT { .. } => {
                    Ok(Type::sizeOf(&(Expression::typeOf(var_field!((**eq).lhs, NFEquation::CONNECT).clone())), false)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FOR { .. } => {
                    let mut size: i32 = size.clone();
                    size = Type::sizeOf(&(Expression::typeOf(var_field!((**eq).range, NFEquation::FOR).clone().ok_or("pattern mismatch")?)), false)?;
                    Ok((size * sizeOfList(var_field!((**eq).body, NFEquation::FOR)), size.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            size = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ IF { .. } => {
                    Ok(Branch::sizeOf(&((var_field!((**eq).branches, NFEquation::IF)).head().cloned()?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ WHEN { .. } => {
                    Ok(Branch::sizeOf(&((var_field!((**eq).branches, NFEquation::WHEN)).head().cloned()?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(0)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    size
}

pub fn toString(mut eq: &metamodelica::Ref<NFEquation>, mut indent: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: IOStream::IOStream;
    s = IOStream::create(
        literal!("NFEquation.toString"),
        openmodelica_util::IOStream::IOStreamType::LIST,
    )?;
    s = toStream(eq, indent, s)?;
    r#str = IOStream::string(&s)?;
    IOStream::delete(&s)?;
    Ok(r#str)
}

pub(crate) fn toStringList(
    mut eql: &metamodelica::List<metamodelica::Ref<NFEquation>>,
    mut indent: ArcStr,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: IOStream::IOStream;
    s = IOStream::create(
        literal!("NFEquation.toStringList"),
        openmodelica_util::IOStream::IOStreamType::LIST,
    )?;
    s = toStreamList(eql, indent, s)?;
    r#str = IOStream::string(&s)?;
    IOStream::delete(&s)?;
    Ok(r#str)
}

pub(crate) fn toStream(
    mut eq: &metamodelica::Ref<NFEquation>,
    mut indent: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut branches: metamodelica::List<metamodelica::Ref<Branch::Branch>>;
    let mut branch: metamodelica::Ref<Branch::Branch>;
    s = IOStream::append(s, indent.clone())?;
    s = (match &**eq {
        EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => {
            s = IOStream::append(s, Expression::toString(__eq_lhs.clone())?)?;
            s = IOStream::append(s, literal!(" = "))?;
            s = IOStream::append(s, Expression::toString(__eq_rhs.clone())?)?;
            s
        }
        CONNECT {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => {
            s = IOStream::append(s, literal!("connect("))?;
            s = IOStream::append(s, Expression::toString(__eq_lhs.clone())?)?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, Expression::toString(__eq_rhs.clone())?)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        FOR {
            body: __eq_body,
            iterator: __eq_iterator,
            range: __eq_range,
            ..
        } => {
            s = IOStream::append(s, literal!("for "))?;
            s = IOStream::append(
                s,
                NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&__eq_iterator))?,
            )?;
            if (__eq_range).is_some() {
                s = IOStream::append(s, literal!(" in "))?;
                s = IOStream::append(s, Expression::toString(__eq_range.clone().ok_or("pattern mismatch")?)?)?;
            }
            s = IOStream::append(s, literal!(" loop\n"))?;
            s = toStreamList(
                metamodelica::AsArg::as_arg(&__eq_body),
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("  "));
                    ArcStr::from(__mm_s)
                },
                s,
            )?;
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("end for"))?;
            s
        }
        IF {
            branches: __eq_branches,
            ..
        } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(__eq_branches.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            branch = metamodelica::Own::own(__pa0);
            branches = metamodelica::Own::own(__pa1);
            s = Branch::toStream(
                &((__eq_branches).head().cloned()?),
                &(literal!("if ")),
                false,
                &indent,
                s,
            )?;
            while !((branches).is_empty()) {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(branches) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                branch = metamodelica::Own::own(__pa2);
                branches = metamodelica::Own::own(__pa3);
                s = IOStream::append(s, indent.clone())?;
                s = Branch::toStream(&branch, &(literal!("elseif ")), (branches).is_empty(), &indent, s)?;
            }
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("end if"))?;
            s
        }
        WHEN {
            branches: __eq_branches,
            ..
        } => {
            s = Branch::toStream(
                &((__eq_branches).head().cloned()?),
                &(literal!("when ")),
                false,
                &indent,
                s,
            )?;
            for mut b in &*(__eq_branches).rest()? {
                s = IOStream::append(s, indent.clone())?;
                s = Branch::toStream(
                    metamodelica::AsArg::as_arg(&b),
                    &(literal!("elsewhen ")),
                    false,
                    &indent,
                    s,
                )?;
            }
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("end when"))?;
            s
        }
        ASSERT {
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            ..
        } => {
            s = IOStream::append(s, literal!("assert("))?;
            s = IOStream::append(s, Expression::toString(__eq_condition.clone())?)?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, Expression::toString(__eq_message.clone())?)?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, Expression::toString(__eq_level.clone())?)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        TERMINATE {
            message: __eq_message, ..
        } => {
            s = IOStream::append(s, literal!("terminate("))?;
            s = IOStream::append(s, Expression::toString(__eq_message.clone())?)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        REINIT {
            cref: __eq_cref,
            reinitExp: __eq_reinitExp,
            ..
        } => {
            s = IOStream::append(s, literal!("reinit("))?;
            s = IOStream::append(s, Expression::toString(__eq_cref.clone())?)?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, Expression::toString(__eq_reinitExp.clone())?)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        NORETCALL { exp: __eq_exp, .. } => IOStream::append(s, Expression::toString(__eq_exp.clone())?)?,
        _ => IOStream::append(s, literal!("#UNKNOWN EQUATION#"))?,
    });
    Ok(s)
}

pub(crate) fn toStreamList(
    mut eql: &metamodelica::List<metamodelica::Ref<NFEquation>>,
    mut indent: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut prev_multi_line: bool = false;
    let mut multi_line: bool;
    let mut first: bool = true;
    for mut eq in &**eql {
        multi_line = isMultiLine(metamodelica::AsArg::as_arg(&eq));
        if first {
            first = false;
        } else if prev_multi_line || multi_line {
            s = IOStream::append(s, literal!("\n"))?;
        }
        prev_multi_line = multi_line;
        s = toStream(metamodelica::AsArg::as_arg(&eq), indent.clone(), s)?;
        s = IOStream::append(s, literal!(";\n"))?;
    }
    Ok(s)
}

pub(crate) fn toFlatStream(
    mut eq: &metamodelica::Ref<NFEquation>,
    mut format: BaseModelica::OutputFormat,
    mut indent: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut branches: metamodelica::List<metamodelica::Ref<Branch::Branch>>;
    let mut branch: metamodelica::Ref<Branch::Branch>;
    s = IOStream::append(s, indent.clone())?;
    s = (match &**eq {
        EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => {
            s = IOStream::append(s, Expression::toFlatString(__eq_lhs.clone(), format)?)?;
            s = IOStream::append(s, literal!(" = "))?;
            s = IOStream::append(s, Expression::toFlatString(__eq_rhs.clone(), format)?)?;
            s
        }
        CONNECT {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => {
            s = IOStream::append(s, literal!("connect("))?;
            s = IOStream::append(s, Expression::toFlatString(__eq_lhs.clone(), format)?)?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, Expression::toFlatString(__eq_rhs.clone(), format)?)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        FOR {
            body: __eq_body,
            iterator: __eq_iterator,
            range: __eq_range,
            ..
        } => {
            s = IOStream::append(s, literal!("for "))?;
            s = IOStream::append(
                s,
                Util::makeQuotedIdentifier(NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&__eq_iterator))?)?,
            )?;
            if (__eq_range).is_some() {
                s = IOStream::append(s, literal!(" in "))?;
                s = IOStream::append(
                    s,
                    Expression::toFlatString(__eq_range.clone().ok_or("pattern mismatch")?, format)?,
                )?;
            }
            s = IOStream::append(s, literal!(" loop\n"))?;
            s = toFlatStreamList(
                metamodelica::AsArg::as_arg(&__eq_body),
                format,
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("  "));
                    ArcStr::from(__mm_s)
                },
                s,
            )?;
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("end for"))?;
            s
        }
        IF {
            branches: __eq_branches,
            ..
        } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(__eq_branches.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            branch = metamodelica::Own::own(__pa0);
            branches = metamodelica::Own::own(__pa1);
            s = Branch::toFlatStream(&branch, &(literal!("if ")), format, false, &indent, s)?;
            while !((branches).is_empty()) {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(branches) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                branch = metamodelica::Own::own(__pa2);
                branches = metamodelica::Own::own(__pa3);
                s = IOStream::append(s, indent.clone())?;
                s = Branch::toFlatStream(
                    &branch,
                    &(literal!("elseif ")),
                    format,
                    (branches).is_empty(),
                    &indent,
                    s,
                )?;
            }
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("end if"))?;
            s
        }
        WHEN {
            branches: __eq_branches,
            ..
        } => {
            s = Branch::toFlatStream(
                &((__eq_branches).head().cloned()?),
                &(literal!("when ")),
                format,
                false,
                &indent,
                s,
            )?;
            for mut b in &*(__eq_branches).rest()? {
                s = IOStream::append(s, indent.clone())?;
                s = Branch::toFlatStream(
                    metamodelica::AsArg::as_arg(&b),
                    &(literal!("elsewhen ")),
                    format,
                    false,
                    &indent,
                    s,
                )?;
            }
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("end when"))?;
            s
        }
        ASSERT {
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            ..
        } => {
            s = IOStream::append(s, literal!("assert("))?;
            s = IOStream::append(s, Expression::toFlatString(__eq_condition.clone(), format)?)?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, Expression::toFlatString(__eq_message.clone(), format)?)?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, Expression::toFlatString(__eq_level.clone(), format)?)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        TERMINATE {
            message: __eq_message, ..
        } => {
            s = IOStream::append(s, literal!("terminate("))?;
            s = IOStream::append(s, Expression::toFlatString(__eq_message.clone(), format)?)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        REINIT {
            cref: __eq_cref,
            reinitExp: __eq_reinitExp,
            ..
        } => {
            s = IOStream::append(s, literal!("reinit("))?;
            s = IOStream::append(s, Expression::toFlatString(__eq_cref.clone(), format)?)?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, Expression::toFlatString(__eq_reinitExp.clone(), format)?)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        NORETCALL { exp: __eq_exp, .. } => IOStream::append(s, Expression::toFlatString(__eq_exp.clone(), format)?)?,
        _ => IOStream::append(s, literal!("#UNKNOWN EQUATION#"))?,
    });
    s = FlatModelicaUtil::appendElementSourceComment(
        &(source(eq)),
        FlatModelicaUtil::ElementType::EQUATION.clone(),
        s,
    )?;
    Ok(s)
}

pub(crate) fn toFlatStreamList(
    mut eql: &metamodelica::List<metamodelica::Ref<NFEquation>>,
    mut format: BaseModelica::OutputFormat,
    mut indent: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut prev_multi_line: bool = false;
    let mut multi_line: bool;
    let mut first: bool = true;
    for mut eq in &**eql {
        multi_line = isMultiLine(metamodelica::AsArg::as_arg(&eq));
        if first {
            first = false;
        } else if prev_multi_line || multi_line {
            s = IOStream::append(s, literal!("\n"))?;
        }
        prev_multi_line = multi_line;
        s = toFlatStream(metamodelica::AsArg::as_arg(&eq), format, indent.clone(), s)?;
        s = IOStream::append(s, literal!(";\n"))?;
    }
    Ok(s)
}

pub(crate) fn isMultiLine(mut eq: &metamodelica::Ref<NFEquation>) -> bool {
    let mut singleLine: bool;
    singleLine = (match &**eq {
        FOR { .. } => true,
        IF { .. } => true,
        WHEN { .. } => true,
        _ => false,
    });
    singleLine
}

pub(crate) fn splitRecordEquations(
    mut equations: &metamodelica::List<metamodelica::Ref<NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFEquation>>> {
    let mut outEquations: metamodelica::List<metamodelica::Ref<NFEquation>> = metamodelica::nil();
    for mut eq in &**equations {
        outEquations = splitRecordEquation(eq.clone(), outEquations)?;
    }
    outEquations = metamodelica::Dangerous::listReverseInPlace(outEquations);
    Ok(outEquations)
}

pub(crate) fn splitRecordEquation(
    mut eq: metamodelica::Ref<NFEquation>,
    mut equations: metamodelica::List<metamodelica::Ref<NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<NFEquation>> = equations;
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    equations = (match &*eq {
        EQUALITY {
            lhs: __eq_lhs,
            ty: __eq_ty,
            ..
        } if (Type::isRecord(&(Type::arrayElementType(metamodelica::AsArg::as_arg(&__eq_ty))))) => {
            assign_variant_field!(eq => NFEquation::EQUALITY;
                lhs = ExpandExp::expand(__eq_lhs.clone(), false, false)?.0,
                rhs = ExpandExp::expand(var_field!((*eq).rhs, NFEquation::EQUALITY).clone(), false, false)?.0
            );
            for mut i in
                1..=Type::recordFieldCount(&(Type::arrayElementType(var_field!((*eq).ty, NFEquation::EQUALITY))))
            {
                lhs = Expression::nthRecordElement(i, var_field!((*eq).lhs, NFEquation::EQUALITY))?;
                rhs = Expression::nthRecordElement(i, var_field!((*eq).rhs, NFEquation::EQUALITY))?;
                equations = metamodelica::cons(
                    metamodelica::Ref::new(NFEquation::EQUALITY {
                        lhs: lhs.clone(),
                        rhs: rhs,
                        ty: Expression::typeOf(lhs),
                        scope: var_field!((*eq).scope, NFEquation::EQUALITY).clone(),
                        source: var_field!((*eq).source, NFEquation::EQUALITY).clone(),
                        scalarizeMode: var_field!((*eq).scalarizeMode, NFEquation::EQUALITY).clone(),
                    }),
                    equations,
                );
            }
            equations
        }
        FOR { body: __eq_body, .. } => {
            assign_variant_field!(eq => NFEquation::FOR; body = splitRecordEquations(metamodelica::AsArg::as_arg(&__eq_body))?);
            metamodelica::cons(eq, equations)
        }
        IF {
            branches: __eq_branches,
            ..
        } => {
            assign_variant_field!(eq => NFEquation::IF; branches = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Branch::Branch>> = metamodelica::nil();
                for mut b in (__eq_branches.clone()).into_iter().cloned() {
                    let __x = splitRecordEquationBranch(b.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            metamodelica::cons(eq, equations)
        }
        WHEN {
            branches: __eq_branches,
            ..
        } => {
            assign_variant_field!(eq => NFEquation::WHEN; branches = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Branch::Branch>> = metamodelica::nil();
                for mut b in (__eq_branches.clone()).into_iter().cloned() {
                    let __x = splitRecordEquationBranch(b.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            metamodelica::cons(eq, equations)
        }
        _ => metamodelica::cons(eq, equations),
    });
    Ok(equations)
}

pub(crate) fn splitRecordEquationBranch(
    mut branch: metamodelica::Ref<Branch::Branch>,
) -> Result<metamodelica::Ref<Branch::Branch>> {
    let mut branch: metamodelica::Ref<Branch::Branch> = branch;
    let () = (match &*branch {
        Branch::BRANCH {
            body: __branch_body, ..
        } => {
            assign_variant_field!(branch => Branch::Branch::BRANCH; body = splitRecordEquations(metamodelica::AsArg::as_arg(&__branch_body))?);
            ()
        }
        _ => (),
    });
    Ok(branch)
}
