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
use crate::NFExpression as Expression;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_util::JSON;
use openmodelica_util::Util;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFClockKind {
    /// Clock()
    INFERRED_CLOCK {
        /// unique index to correctly associate equal inferred clocks
        idx: i32,
    },
    RATIONAL_CLOCK {
        /// integer type >= 0
        intervalCounter: metamodelica::Ref<Expression::NFExpression>,
        /// integer type >= 1, defaults to 1
        resolution: metamodelica::Ref<Expression::NFExpression>,
    },
    REAL_CLOCK {
        /// real type > 0
        interval: metamodelica::Ref<Expression::NFExpression>,
    },
    EVENT_CLOCK {
        /// boolean type
        condition: metamodelica::Ref<Expression::NFExpression>,
        /// real type >= 0.0
        startInterval: metamodelica::Ref<Expression::NFExpression>,
    },
    SOLVER_CLOCK {
        /// clock type
        c: metamodelica::Ref<Expression::NFExpression>,
        /// string type
        solverMethod: metamodelica::Ref<Expression::NFExpression>,
    },
}
impl metamodelica::gc::MMTrace for NFClockKind {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFClockKind::INFERRED_CLOCK { idx } => {
                metamodelica::gc::MMTrace::mm_accept(idx, __mmv)?;
                Ok(())
            }
            NFClockKind::RATIONAL_CLOCK {
                intervalCounter,
                resolution,
            } => {
                metamodelica::gc::MMTrace::mm_accept(intervalCounter, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(resolution, __mmv)?;
                Ok(())
            }
            NFClockKind::REAL_CLOCK { interval } => {
                metamodelica::gc::MMTrace::mm_accept(interval, __mmv)?;
                Ok(())
            }
            NFClockKind::EVENT_CLOCK {
                condition,
                startInterval,
            } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startInterval, __mmv)?;
                Ok(())
            }
            NFClockKind::SOLVER_CLOCK { c, solverMethod } => {
                metamodelica::gc::MMTrace::mm_accept(c, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(solverMethod, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for NFClockKind {
    fn default() -> Self {
        Self::INFERRED_CLOCK {
            idx: Default::default(),
        }
    }
}
pub use self::NFClockKind::{EVENT_CLOCK, INFERRED_CLOCK, RATIONAL_CLOCK, REAL_CLOCK, SOLVER_CLOCK};
pub fn isInferred(mut ck: &metamodelica::Ref<NFClockKind>) -> bool {
    let mut b: bool;
    b = (match &**ck {
        INFERRED_CLOCK { .. } => true,
        _ => false,
    });
    b
}

pub fn compare(mut ck1: metamodelica::Ref<NFClockKind>, mut ck2: metamodelica::Ref<NFClockKind>) -> Result<i32> {
    fn compareInt(mut kind: &metamodelica::Ref<NFClockKind>) -> i32 {
        let mut i: i32;
        i = (match &**kind {
            INFERRED_CLOCK { .. } => 0,
            RATIONAL_CLOCK { .. } => 1,
            REAL_CLOCK { .. } => 2,
            EVENT_CLOCK { .. } => 3,
            SOLVER_CLOCK { .. } => 4,
            _ => 5,
        });
        i
    }

    let mut comp: i32;
    comp = (::match_deref::match_deref! { match &((ck1.clone(), ck2.clone())) {
        (Deref @ INFERRED_CLOCK { .. }, Deref @ INFERRED_CLOCK { .. }) => {
            Util::intCompare(var_field!((*ck1).idx, NFClockKind::INFERRED_CLOCK).clone(), var_field!((*ck2).idx, NFClockKind::INFERRED_CLOCK).clone())
        },
        (Deref @ RATIONAL_CLOCK { intervalCounter: i1, resolution: r1 }, Deref @ RATIONAL_CLOCK { intervalCounter: i2, resolution: r2 }) => {
            comp = Expression::compare(i1.clone(), i2.clone())?;
            if comp == 0 {
                comp = Expression::compare(r1.clone(), r2.clone())?;
            }
            comp
        },
        (Deref @ REAL_CLOCK { interval: i1 }, Deref @ REAL_CLOCK { interval: i2 }) => {
            Expression::compare(i1.clone(), i2.clone())?
        },
        (Deref @ EVENT_CLOCK { condition: c1, startInterval: si1 }, Deref @ EVENT_CLOCK { condition: c2, startInterval: si2 }) => {
            comp = Expression::compare(c1.clone(), c2.clone())?;
            if comp == 0 {
                comp = Expression::compare(si1.clone(), si2.clone())?;
            }
            comp
        },
        (Deref @ SOLVER_CLOCK { c: c1, solverMethod: sm2 }, Deref @ SOLVER_CLOCK { c: c2, solverMethod: sm1 }) => {
            comp = Expression::compare(c1.clone(), c2.clone())?;
            if comp == 0 {
                comp = Expression::compare(sm1.clone(), sm2.clone())?;
            }
            comp
        },
        _ => {
            if (compareInt(&ck1) < compareInt(&ck2)) {-1} else {1}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

pub(crate) fn containsExp(
    mut ck: &metamodelica::Ref<NFClockKind>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    res = (match &**ck {
        RATIONAL_CLOCK {
            intervalCounter: __ck_intervalCounter,
            resolution: __ck_resolution,
        } => {
            Expression::contains(__ck_intervalCounter.clone(), func)?
                || Expression::contains(__ck_resolution.clone(), func)?
        }
        REAL_CLOCK {
            interval: __ck_interval,
        } => Expression::contains(__ck_interval.clone(), func)?,
        EVENT_CLOCK {
            condition: __ck_condition,
            startInterval: __ck_startInterval,
        } => {
            Expression::contains(__ck_condition.clone(), func)?
                || Expression::contains(__ck_startInterval.clone(), func)?
        }
        SOLVER_CLOCK {
            c: __ck_c,
            solverMethod: __ck_solverMethod,
        } => Expression::contains(__ck_c.clone(), func)? || Expression::contains(__ck_solverMethod.clone(), func)?,
        _ => false,
    });
    Ok(res)
}

pub(crate) fn containsExpShallow(
    mut ck: &metamodelica::Ref<NFClockKind>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    res = (match &**ck {
        RATIONAL_CLOCK {
            intervalCounter: __ck_intervalCounter,
            resolution: __ck_resolution,
        } => func(__ck_intervalCounter.clone())? || func(__ck_resolution.clone())?,
        REAL_CLOCK {
            interval: __ck_interval,
        } => func(__ck_interval.clone())?,
        EVENT_CLOCK {
            condition: __ck_condition,
            startInterval: __ck_startInterval,
        } => func(__ck_condition.clone())? || func(__ck_startInterval.clone())?,
        SOLVER_CLOCK {
            c: __ck_c,
            solverMethod: __ck_solverMethod,
        } => func(__ck_c.clone())? || func(__ck_solverMethod.clone())?,
        _ => false,
    });
    Ok(res)
}

pub(crate) fn applyExp(
    mut ck: &metamodelica::Ref<NFClockKind>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (match &**ck {
        RATIONAL_CLOCK {
            intervalCounter: __ck_intervalCounter,
            resolution: __ck_resolution,
        } => {
            Expression::apply(__ck_intervalCounter.clone(), func)?;
            Expression::apply(__ck_resolution.clone(), func)?;
            ()
        }
        REAL_CLOCK {
            interval: __ck_interval,
        } => {
            Expression::apply(__ck_interval.clone(), func)?;
            ()
        }
        EVENT_CLOCK {
            condition: __ck_condition,
            startInterval: __ck_startInterval,
        } => {
            Expression::apply(__ck_condition.clone(), func)?;
            Expression::apply(__ck_startInterval.clone(), func)?;
            ()
        }
        SOLVER_CLOCK {
            c: __ck_c,
            solverMethod: __ck_solverMethod,
        } => {
            Expression::apply(__ck_c.clone(), func)?;
            Expression::apply(__ck_solverMethod.clone(), func)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn applyExpShallow(
    mut ck: &metamodelica::Ref<NFClockKind>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (match &**ck {
        RATIONAL_CLOCK {
            intervalCounter: __ck_intervalCounter,
            resolution: __ck_resolution,
        } => {
            func(__ck_intervalCounter.clone())?;
            func(__ck_resolution.clone())?;
            ()
        }
        REAL_CLOCK {
            interval: __ck_interval,
        } => {
            func(__ck_interval.clone())?;
            ()
        }
        EVENT_CLOCK {
            condition: __ck_condition,
            startInterval: __ck_startInterval,
        } => {
            func(__ck_condition.clone())?;
            func(__ck_startInterval.clone())?;
            ()
        }
        SOLVER_CLOCK {
            c: __ck_c,
            solverMethod: __ck_solverMethod,
        } => {
            func(__ck_c.clone())?;
            func(__ck_solverMethod.clone())?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn foldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut ck: &metamodelica::Ref<NFClockKind>,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut result: ArgT;
    result = (match &**ck {
        RATIONAL_CLOCK {
            intervalCounter: __ck_intervalCounter,
            resolution: __ck_resolution,
        } => {
            result = Expression::fold(__ck_intervalCounter.clone(), func.clone(), arg)?;
            Expression::fold(__ck_resolution.clone(), func.clone(), result)?
        }
        REAL_CLOCK {
            interval: __ck_interval,
        } => Expression::fold(__ck_interval.clone(), func.clone(), arg)?,
        EVENT_CLOCK {
            condition: __ck_condition,
            startInterval: __ck_startInterval,
        } => {
            result = Expression::fold(__ck_condition.clone(), func.clone(), arg)?;
            Expression::fold(__ck_startInterval.clone(), func.clone(), result)?
        }
        SOLVER_CLOCK {
            c: __ck_c,
            solverMethod: __ck_solverMethod,
        } => {
            result = Expression::fold(__ck_c.clone(), func.clone(), arg)?;
            Expression::fold(__ck_solverMethod.clone(), func.clone(), result)?
        }
        _ => arg,
    });
    Ok(result)
}

pub(crate) fn mapExp(
    mut ck: metamodelica::Ref<NFClockKind>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >,
) -> Result<metamodelica::Ref<NFClockKind>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut outCk: metamodelica::Ref<NFClockKind>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut e3: metamodelica::Ref<Expression::NFExpression>;
    let mut e4: metamodelica::Ref<Expression::NFExpression>;
    outCk = (match &*ck {
        RATIONAL_CLOCK {
            intervalCounter: __esc_e1,
            resolution: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            e3 = Expression::map(e1.clone(), func.clone())?;
            e4 = Expression::map(e2.clone(), func.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e3)) && referenceEq(&*(e2.clone()), &*(&*e4))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::RATIONAL_CLOCK {
                    intervalCounter: e3,
                    resolution: e4,
                })
            }
        }
        REAL_CLOCK { interval: __esc_e1 } => {
            e1 = (*__esc_e1).clone();
            e3 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e3))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::REAL_CLOCK { interval: e3 })
            }
        }
        EVENT_CLOCK {
            condition: __esc_e1,
            startInterval: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            e3 = Expression::map(e1.clone(), func.clone())?;
            e4 = Expression::map(e2.clone(), func.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e3)) && referenceEq(&*(e2.clone()), &*(&*e4))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::EVENT_CLOCK {
                    condition: e3,
                    startInterval: e4,
                })
            }
        }
        SOLVER_CLOCK {
            c: __esc_e1,
            solverMethod: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            e3 = Expression::map(e1.clone(), func.clone())?;
            e4 = Expression::map(e2.clone(), func.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e3)) && referenceEq(&*(e2.clone()), &*(&*e4))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::SOLVER_CLOCK {
                    c: e3,
                    solverMethod: e4,
                })
            }
        }
        _ => ck,
    });
    Ok(outCk)
}

pub(crate) fn mapExpShallow(
    mut ck: metamodelica::Ref<NFClockKind>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NFClockKind>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut outCk: metamodelica::Ref<NFClockKind>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut e3: metamodelica::Ref<Expression::NFExpression>;
    let mut e4: metamodelica::Ref<Expression::NFExpression>;
    outCk = (match &*ck {
        RATIONAL_CLOCK {
            intervalCounter: __esc_e1,
            resolution: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            e3 = func(e1.clone())?;
            e4 = func(e2.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e3)) && referenceEq(&*(e2.clone()), &*(&*e4))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::RATIONAL_CLOCK {
                    intervalCounter: e3,
                    resolution: e4,
                })
            }
        }
        REAL_CLOCK { interval: __esc_e1 } => {
            e1 = (*__esc_e1).clone();
            e3 = func(e1.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e3))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::REAL_CLOCK { interval: e3 })
            }
        }
        EVENT_CLOCK {
            condition: __esc_e1,
            startInterval: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            e3 = func(e1.clone())?;
            e4 = func(e2.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e3)) && referenceEq(&*(e2.clone()), &*(&*e4))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::EVENT_CLOCK {
                    condition: e3,
                    startInterval: e4,
                })
            }
        }
        SOLVER_CLOCK {
            c: __esc_e1,
            solverMethod: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            e3 = func(e1.clone())?;
            e4 = func(e2.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e3)) && referenceEq(&*(e2.clone()), &*(&*e4))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::SOLVER_CLOCK {
                    c: e3,
                    solverMethod: e4,
                })
            }
        }
        _ => ck,
    });
    Ok(outCk)
}

pub(crate) fn mapFoldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut ck: metamodelica::Ref<NFClockKind>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<NFClockKind>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >;

    let mut outCk: metamodelica::Ref<NFClockKind>;
    let mut arg: ArgT = arg;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut e3: metamodelica::Ref<Expression::NFExpression>;
    let mut e4: metamodelica::Ref<Expression::NFExpression>;
    outCk = (match &*ck {
        RATIONAL_CLOCK {
            intervalCounter: __esc_e1,
            resolution: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            (e3, arg) = Expression::mapFold(e1.clone(), func.clone(), arg)?;
            (e4, arg) = Expression::mapFold(e2.clone(), func.clone(), arg)?;
            if (referenceEq(&*(e1.clone()), &*(&*e3)) && referenceEq(&*(e2.clone()), &*(&*e4))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::RATIONAL_CLOCK {
                    intervalCounter: e3,
                    resolution: e4,
                })
            }
        }
        REAL_CLOCK { interval: __esc_e1 } => {
            e1 = (*__esc_e1).clone();
            (e3, arg) = Expression::mapFold(e1.clone(), func.clone(), arg)?;
            if (referenceEq(&*(e1.clone()), &*(&*e3))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::REAL_CLOCK { interval: e3 })
            }
        }
        EVENT_CLOCK {
            condition: __esc_e1,
            startInterval: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            (e3, arg) = Expression::mapFold(e1.clone(), func.clone(), arg)?;
            (e4, arg) = Expression::mapFold(e2.clone(), func.clone(), arg)?;
            if (referenceEq(&*(e1.clone()), &*(&*e3)) && referenceEq(&*(e2.clone()), &*(&*e4))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::EVENT_CLOCK {
                    condition: e3,
                    startInterval: e4,
                })
            }
        }
        SOLVER_CLOCK {
            c: __esc_e1,
            solverMethod: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            (e3, arg) = Expression::mapFold(e1.clone(), func.clone(), arg)?;
            (e4, arg) = Expression::mapFold(e2.clone(), func.clone(), arg)?;
            if (referenceEq(&*(e1.clone()), &*(&*e3)) && referenceEq(&*(e2.clone()), &*(&*e4))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::SOLVER_CLOCK {
                    c: e3,
                    solverMethod: e4,
                })
            }
        }
        _ => ck,
    });
    Ok((outCk, arg))
}

pub(crate) fn mapFoldExpShallow<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut ck: metamodelica::Ref<NFClockKind>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<NFClockKind>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >;

    let mut outCk: metamodelica::Ref<NFClockKind>;
    let mut arg: ArgT = arg;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut e3: metamodelica::Ref<Expression::NFExpression>;
    let mut e4: metamodelica::Ref<Expression::NFExpression>;
    outCk = (match &*ck {
        RATIONAL_CLOCK {
            intervalCounter: __esc_e1,
            resolution: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            (e3, arg) = Expression::mapFoldShallow(e1.clone(), func.clone(), arg)?;
            (e4, arg) = Expression::mapFoldShallow(e2.clone(), func.clone(), arg)?;
            if (referenceEq(&*(e1.clone()), &*(&*e3)) && referenceEq(&*(e2.clone()), &*(&*e4))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::RATIONAL_CLOCK {
                    intervalCounter: e3,
                    resolution: e4,
                })
            }
        }
        REAL_CLOCK { interval: __esc_e1 } => {
            e1 = (*__esc_e1).clone();
            (e3, arg) = Expression::mapFoldShallow(e1.clone(), func.clone(), arg)?;
            if (referenceEq(&*(e1.clone()), &*(&*e3))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::REAL_CLOCK { interval: e3 })
            }
        }
        EVENT_CLOCK {
            condition: __esc_e1,
            startInterval: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            (e3, arg) = Expression::mapFoldShallow(e1.clone(), func.clone(), arg)?;
            (e4, arg) = Expression::mapFoldShallow(e2.clone(), func.clone(), arg)?;
            if (referenceEq(&*(e1.clone()), &*(&*e3)) && referenceEq(&*(e2.clone()), &*(&*e4))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::EVENT_CLOCK {
                    condition: e3,
                    startInterval: e4,
                })
            }
        }
        SOLVER_CLOCK {
            c: __esc_e1,
            solverMethod: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            (e3, arg) = Expression::mapFoldShallow(e1.clone(), func.clone(), arg)?;
            (e4, arg) = Expression::mapFoldShallow(e2.clone(), func.clone(), arg)?;
            if (referenceEq(&*(e1.clone()), &*(&*e3)) && referenceEq(&*(e2.clone()), &*(&*e4))) {
                ck
            } else {
                metamodelica::Ref::new(NFClockKind::SOLVER_CLOCK {
                    c: e3,
                    solverMethod: e4,
                })
            }
        }
        _ => ck,
    });
    Ok((outCk, arg))
}

pub(crate) fn toAbsyn(mut clk: &metamodelica::Ref<NFClockKind>) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    args = (match &**clk {
        INFERRED_CLOCK { .. } => metamodelica::nil(),
        RATIONAL_CLOCK {
            intervalCounter: __clk_intervalCounter,
            resolution: __clk_resolution,
        } => list![
            Expression::toAbsyn(__clk_intervalCounter.clone())?,
            Expression::toAbsyn(__clk_resolution.clone())?
        ],
        REAL_CLOCK {
            interval: __clk_interval,
        } => list![Expression::toAbsyn(__clk_interval.clone())?],
        EVENT_CLOCK {
            condition: __clk_condition,
            startInterval: __clk_startInterval,
        } => list![
            Expression::toAbsyn(__clk_condition.clone())?,
            Expression::toAbsyn(__clk_startInterval.clone())?
        ],
        SOLVER_CLOCK {
            c: __clk_c,
            solverMethod: __clk_solverMethod,
        } => list![
            Expression::toAbsyn(__clk_c.clone())?,
            Expression::toAbsyn(__clk_solverMethod.clone())?
        ],
    });
    exp = AbsynUtil::makeCall(
        metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
            name: literal!("Clock"),
            subscripts: metamodelica::nil(),
        }),
        args,
        metamodelica::nil(),
    );
    Ok(exp)
}

pub fn toDAE(mut ick: &metamodelica::Ref<NFClockKind>) -> Result<metamodelica::Ref<DAE::ClockKind>> {
    let mut ock: metamodelica::Ref<DAE::ClockKind>;
    ock = (match &**ick {
        INFERRED_CLOCK { .. } => openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK(),
        RATIONAL_CLOCK {
            intervalCounter: i,
            resolution: r,
        } => metamodelica::Ref::new(DAE::ClockKind::RATIONAL_CLOCK {
            intervalCounter: Expression::toDAE(i.clone(), false)?,
            resolution: Expression::toDAE(r.clone(), false)?,
        }),
        REAL_CLOCK { interval: i } => metamodelica::Ref::new(DAE::ClockKind::REAL_CLOCK {
            interval: Expression::toDAE(i.clone(), false)?,
        }),
        EVENT_CLOCK {
            condition: c,
            startInterval: si,
        } => metamodelica::Ref::new(DAE::ClockKind::EVENT_CLOCK {
            condition: Expression::toDAE(c.clone(), false)?,
            startInterval: Expression::toDAE(si.clone(), false)?,
        }),
        SOLVER_CLOCK { c, solverMethod: sm } => metamodelica::Ref::new(DAE::ClockKind::SOLVER_CLOCK {
            c: Expression::toDAE(c.clone(), false)?,
            solverMethod: Expression::toDAE(sm.clone(), false)?,
        }),
    });
    Ok(ock)
}

pub fn toDebugString(mut ick: &metamodelica::Ref<NFClockKind>) -> Result<ArcStr> {
    let mut ock: ArcStr;
    ock = (match &**ick {
        INFERRED_CLOCK { idx: __ick_idx } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("INFERRED_CLOCK("));
            __mm_s.push_str(&*intString(__ick_idx.clone()));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        RATIONAL_CLOCK {
            intervalCounter: i,
            resolution: r,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("RATIONAL_CLOCK("));
            __mm_s.push_str(&*Expression::toString(i.clone())?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*Expression::toString(r.clone())?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        REAL_CLOCK { interval: i } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("REAL_CLOCK("));
            __mm_s.push_str(&*Expression::toString(i.clone())?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        EVENT_CLOCK {
            condition: c,
            startInterval: si,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("EVENT_CLOCK("));
            __mm_s.push_str(&*Expression::toString(c.clone())?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*Expression::toString(si.clone())?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        SOLVER_CLOCK { c, solverMethod: sm } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("SOLVER_CLOCK("));
            __mm_s.push_str(&*Expression::toString(c.clone())?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*Expression::toString(sm.clone())?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
    });
    Ok(ock)
}

pub(crate) fn toString(mut ck: &metamodelica::Ref<NFClockKind>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**ck {
        INFERRED_CLOCK { .. } => {
            literal!("")
        }
        RATIONAL_CLOCK {
            intervalCounter: e1,
            resolution: e2,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Expression::toString(e1.clone())?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*Expression::toString(e2.clone())?);
            ArcStr::from(__mm_s)
        }
        REAL_CLOCK { interval: e1 } => Expression::toString(e1.clone())?,
        EVENT_CLOCK {
            condition: e1,
            startInterval: e2,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Expression::toString(e1.clone())?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*Expression::toString(e2.clone())?);
            ArcStr::from(__mm_s)
        }
        SOLVER_CLOCK {
            c: e1,
            solverMethod: e2,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Expression::toString(e1.clone())?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*Expression::toString(e2.clone())?);
            ArcStr::from(__mm_s)
        }
    });
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Clock("));
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn toFlatString(
    mut ck: &metamodelica::Ref<NFClockKind>,
    mut format: BaseModelica::OutputFormat,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**ck {
        INFERRED_CLOCK { .. } => {
            literal!("")
        }
        RATIONAL_CLOCK {
            intervalCounter: e1,
            resolution: e2,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Expression::toFlatString(e1.clone(), format)?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*Expression::toFlatString(e2.clone(), format)?);
            ArcStr::from(__mm_s)
        }
        REAL_CLOCK { interval: e1 } => Expression::toFlatString(e1.clone(), format)?,
        EVENT_CLOCK {
            condition: e1,
            startInterval: e2,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Expression::toFlatString(e1.clone(), format)?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*Expression::toFlatString(e2.clone(), format)?);
            ArcStr::from(__mm_s)
        }
        SOLVER_CLOCK {
            c: e1,
            solverMethod: e2,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Expression::toFlatString(e1.clone(), format)?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*Expression::toFlatString(e2.clone(), format)?);
            ArcStr::from(__mm_s)
        }
    });
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Clock("));
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn toJSON(mut clk: &metamodelica::Ref<NFClockKind>) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::emptyListObject();
    json = JSON::addPair(&(literal!("kind")), &(JSON::makeString(literal!("clock"))), json)?;
    let () = (match &**clk {
        INFERRED_CLOCK { .. } => {
            json = JSON::addPair(&(literal!("type")), &(JSON::makeString(literal!("inferred"))), json)?;
            ()
        }
        RATIONAL_CLOCK {
            intervalCounter: __clk_intervalCounter,
            resolution: __clk_resolution,
        } => {
            json = JSON::addPair(&(literal!("type")), &(JSON::makeString(literal!("rational"))), json)?;
            json = JSON::addPair(
                &(literal!("intervalCounter")),
                &(Expression::toJSON(__clk_intervalCounter.clone())?),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("resolution")),
                &(Expression::toJSON(__clk_resolution.clone())?),
                json,
            )?;
            ()
        }
        REAL_CLOCK {
            interval: __clk_interval,
        } => {
            json = JSON::addPair(&(literal!("type")), &(JSON::makeString(literal!("real"))), json)?;
            json = JSON::addPair(
                &(literal!("interval")),
                &(Expression::toJSON(__clk_interval.clone())?),
                json,
            )?;
            ()
        }
        EVENT_CLOCK {
            condition: __clk_condition,
            startInterval: __clk_startInterval,
        } => {
            json = JSON::addPair(&(literal!("type")), &(JSON::makeString(literal!("event"))), json)?;
            json = JSON::addPair(
                &(literal!("condition")),
                &(Expression::toJSON(__clk_condition.clone())?),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("startInterval")),
                &(Expression::toJSON(__clk_startInterval.clone())?),
                json,
            )?;
            ()
        }
        SOLVER_CLOCK {
            c: __clk_c,
            solverMethod: __clk_solverMethod,
        } => {
            json = JSON::addPair(&(literal!("type")), &(JSON::makeString(literal!("solver"))), json)?;
            json = JSON::addPair(&(literal!("c")), &(Expression::toJSON(__clk_c.clone())?), json)?;
            json = JSON::addPair(
                &(literal!("solverMethod")),
                &(Expression::toJSON(__clk_solverMethod.clone())?),
                json,
            )?;
            ()
        }
    });
    Ok(json)
}

pub(crate) fn hashContinue(mut clk: &metamodelica::Ref<NFClockKind>, mut hash: i32) -> Result<i32> {
    let mut hash: i32 = hash;
    hash = stringHashDjb2Continue(&(literal!("Clock(")), hash);
    hash = (match &**clk {
        INFERRED_CLOCK { idx: __clk_idx } => hash + __clk_idx.clone(),
        RATIONAL_CLOCK {
            intervalCounter: __clk_intervalCounter,
            resolution: __clk_resolution,
        } => {
            hash = Expression::hashContinue(__clk_intervalCounter.clone(), hash)?;
            hash = stringHashDjb2Continue(&(literal!(", ")), hash);
            hash = Expression::hashContinue(__clk_resolution.clone(), hash)?;
            hash
        }
        REAL_CLOCK {
            interval: __clk_interval,
        } => Expression::hashContinue(__clk_interval.clone(), hash)?,
        EVENT_CLOCK {
            condition: __clk_condition,
            startInterval: __clk_startInterval,
        } => {
            hash = Expression::hashContinue(__clk_condition.clone(), hash)?;
            hash = stringHashDjb2Continue(&(literal!(", ")), hash);
            hash = Expression::hashContinue(__clk_startInterval.clone(), hash)?;
            hash
        }
        SOLVER_CLOCK {
            c: __clk_c,
            solverMethod: __clk_solverMethod,
        } => {
            hash = Expression::hashContinue(__clk_c.clone(), hash)?;
            hash = stringHashDjb2Continue(&(literal!(", ")), hash);
            hash = Expression::hashContinue(__clk_solverMethod.clone(), hash)?;
            hash
        }
    });
    hash = stringHashDjb2Continue(&(literal!(")")), hash);
    Ok(hash)
}
