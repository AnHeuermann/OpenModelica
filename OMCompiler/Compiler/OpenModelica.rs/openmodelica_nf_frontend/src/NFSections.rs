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
use crate::NFAlgorithm as Algorithm;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFStatement as Statement;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFSections {
    SECTIONS {
        equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
        initialEquations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
        algorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
        initialAlgorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    },
    EXTERNAL {
        name: ArcStr,
        args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        outputRef: metamodelica::Ref<ComponentRef::NFComponentRef>,
        language: ArcStr,
        ann: Option<metamodelica::Ref<SCode::Annotation>>,
        explicit: bool,
        info: SourceInfo,
    },
    EMPTY,
}
impl metamodelica::gc::MMTrace for NFSections {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFSections::SECTIONS {
                equations,
                initialEquations,
                algorithms,
                initialAlgorithms,
            } => {
                metamodelica::gc::MMTrace::mm_accept(equations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(initialEquations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(algorithms, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(initialAlgorithms, __mmv)?;
                Ok(())
            }
            NFSections::EXTERNAL {
                name,
                args,
                outputRef,
                language,
                ann,
                explicit,
                info,
            } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(args, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(outputRef, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(language, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ann, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(explicit, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            NFSections::EMPTY => Ok(()),
        }
    }
}
impl NFSections {
    pub fn interned_EMPTY() -> metamodelica::Ref<NFSections> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFSections> = metamodelica::Ref::new(NFSections::EMPTY);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_EMPTY() -> metamodelica::Ref<NFSections> {
    NFSections::interned_EMPTY()
}
impl Default for NFSections {
    fn default() -> Self {
        Self::EMPTY
    }
}
pub use self::NFSections::{EMPTY, EXTERNAL, SECTIONS};
pub(crate) fn new(
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut initialEquations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut algorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut initialAlgorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
) -> metamodelica::Ref<NFSections> {
    let mut sections: metamodelica::Ref<NFSections>;
    if (equations).is_empty()
        && (initialEquations).is_empty()
        && (algorithms).is_empty()
        && (initialAlgorithms).is_empty()
    {
        sections = crate::NFSections::interned_EMPTY();
    } else {
        sections = metamodelica::Ref::new(NFSections::SECTIONS {
            equations: equations,
            initialEquations: initialEquations,
            algorithms: algorithms,
            initialAlgorithms: initialAlgorithms,
        });
    }
    sections
}

pub fn equations(
    mut sections: &metamodelica::Ref<NFSections>,
) -> metamodelica::List<metamodelica::Ref<Equation::NFEquation>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    equations = (match &**sections {
        SECTIONS {
            equations: __sections_equations,
            ..
        } => __sections_equations.clone(),
        _ => metamodelica::nil(),
    });
    equations
}

pub(crate) fn prepend(
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut initialEquations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut algorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut initialAlgorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut sections: metamodelica::Ref<NFSections>,
) -> metamodelica::Ref<NFSections> {
    let mut sections: metamodelica::Ref<NFSections> = sections;
    sections = (match &*sections {
        SECTIONS {
            algorithms: __sections_algorithms,
            equations: __sections_equations,
            initialAlgorithms: __sections_initialAlgorithms,
            initialEquations: __sections_initialEquations,
        } => metamodelica::Ref::new(NFSections::SECTIONS {
            equations: listAppend(equations, __sections_equations.clone()),
            initialEquations: listAppend(initialEquations, __sections_initialEquations.clone()),
            algorithms: listAppend(algorithms, __sections_algorithms.clone()),
            initialAlgorithms: listAppend(initialAlgorithms, __sections_initialAlgorithms.clone()),
        }),
        _ => metamodelica::Ref::new(NFSections::SECTIONS {
            equations: equations,
            initialEquations: initialEquations,
            algorithms: algorithms,
            initialAlgorithms: initialAlgorithms,
        }),
    });
    sections
}

pub(crate) fn prependEquation(
    mut eq: metamodelica::Ref<Equation::NFEquation>,
    mut sections: metamodelica::Ref<NFSections>,
    mut isInitial: bool,
) -> Result<metamodelica::Ref<NFSections>> {
    let mut sections: metamodelica::Ref<NFSections> = sections;
    sections = (match &*sections {
        SECTIONS { .. } => {
            if isInitial {
                assign_variant_field!(sections => NFSections::SECTIONS; initialEquations = metamodelica::cons(eq, var_field!((*sections).initialEquations, NFSections::SECTIONS).clone()));
            } else {
                assign_variant_field!(sections => NFSections::SECTIONS; equations = metamodelica::cons(eq, var_field!((*sections).equations, NFSections::SECTIONS).clone()));
            }
            sections
        }
        EMPTY { .. } => {
            if (isInitial) {
                metamodelica::Ref::new(NFSections::SECTIONS {
                    equations: metamodelica::nil(),
                    initialEquations: list![eq],
                    algorithms: metamodelica::nil(),
                    initialAlgorithms: metamodelica::nil(),
                })
            } else {
                metamodelica::Ref::new(NFSections::SECTIONS {
                    equations: list![eq],
                    initialEquations: metamodelica::nil(),
                    algorithms: metamodelica::nil(),
                    initialAlgorithms: metamodelica::nil(),
                })
            }
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFSections.prependEquation"));
                    __mm_s.push_str(&*literal!(" got invalid Sections to prepend equation to"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFSections.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(sections)
}

pub(crate) fn prependAlgorithm(
    mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>,
    mut sections: metamodelica::Ref<NFSections>,
    mut isInitial: bool,
) -> Result<metamodelica::Ref<NFSections>> {
    let mut sections: metamodelica::Ref<NFSections> = sections;
    sections = (match &*sections {
        SECTIONS { .. } => {
            if isInitial {
                assign_variant_field!(sections => NFSections::SECTIONS; initialAlgorithms = metamodelica::cons(alg, var_field!((*sections).initialAlgorithms, NFSections::SECTIONS).clone()));
            } else {
                assign_variant_field!(sections => NFSections::SECTIONS; algorithms = metamodelica::cons(alg, var_field!((*sections).algorithms, NFSections::SECTIONS).clone()));
            }
            sections
        }
        EMPTY { .. } => {
            if (isInitial) {
                metamodelica::Ref::new(NFSections::SECTIONS {
                    equations: metamodelica::nil(),
                    initialEquations: metamodelica::nil(),
                    algorithms: metamodelica::nil(),
                    initialAlgorithms: list![alg],
                })
            } else {
                metamodelica::Ref::new(NFSections::SECTIONS {
                    equations: metamodelica::nil(),
                    initialEquations: metamodelica::nil(),
                    algorithms: list![alg],
                    initialAlgorithms: metamodelica::nil(),
                })
            }
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFSections.prependAlgorithm"));
                    __mm_s.push_str(&*literal!(" got invalid Sections to prepend algorithm to"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFSections.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(sections)
}

pub(crate) fn append(
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut initialEquations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut algorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut initialAlgorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut sections: metamodelica::Ref<NFSections>,
) -> metamodelica::Ref<NFSections> {
    let mut sections: metamodelica::Ref<NFSections> = sections;
    sections = (match &*sections {
        SECTIONS {
            algorithms: __sections_algorithms,
            equations: __sections_equations,
            initialAlgorithms: __sections_initialAlgorithms,
            initialEquations: __sections_initialEquations,
        } => metamodelica::Ref::new(NFSections::SECTIONS {
            equations: listAppend(__sections_equations.clone(), equations),
            initialEquations: listAppend(__sections_initialEquations.clone(), initialEquations),
            algorithms: listAppend(__sections_algorithms.clone(), algorithms),
            initialAlgorithms: listAppend(__sections_initialAlgorithms.clone(), initialAlgorithms),
        }),
        _ => metamodelica::Ref::new(NFSections::SECTIONS {
            equations: equations,
            initialEquations: initialEquations,
            algorithms: algorithms,
            initialAlgorithms: initialAlgorithms,
        }),
    });
    sections
}

pub(crate) fn join(
    mut sections1: metamodelica::Ref<NFSections>,
    mut sections2: metamodelica::Ref<NFSections>,
) -> Result<metamodelica::Ref<NFSections>> {
    let mut sections: metamodelica::Ref<NFSections>;
    sections = (::match_deref::match_deref! { match &((sections1.clone(), sections2.clone())) {
        (Deref @ EMPTY { .. }, _) => sections2,
        (_, Deref @ EMPTY { .. }) => sections1,
        (Deref @ SECTIONS { .. }, Deref @ SECTIONS { .. }) => metamodelica::Ref::new(NFSections::SECTIONS { equations: listAppend(var_field!((*sections1).equations, NFSections::SECTIONS).clone(), var_field!((*sections2).equations, NFSections::SECTIONS).clone()), initialEquations: listAppend(var_field!((*sections1).initialEquations, NFSections::SECTIONS).clone(), var_field!((*sections2).initialEquations, NFSections::SECTIONS).clone()), algorithms: listAppend(var_field!((*sections1).algorithms, NFSections::SECTIONS).clone(), var_field!((*sections2).algorithms, NFSections::SECTIONS).clone()), initialAlgorithms: listAppend(var_field!((*sections1).initialAlgorithms, NFSections::SECTIONS).clone(), var_field!((*sections2).initialAlgorithms, NFSections::SECTIONS).clone()) }),
        _ => return Err("match: no arm matched"),
    } });
    Ok(sections)
}

pub(crate) fn map(
    mut sections: metamodelica::Ref<NFSections>,
    mut eqFn: &dyn ::std::ops::Fn(metamodelica::Ref<Equation::NFEquation>) -> Result<metamodelica::Ref<Equation::NFEquation>>,
    mut algFn: &dyn ::std::ops::Fn(metamodelica::Ref<Algorithm::NFAlgorithm>) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut ieqFn: &dyn ::std::ops::Fn(metamodelica::Ref<Equation::NFEquation>) -> Result<metamodelica::Ref<Equation::NFEquation>>,
    mut ialgFn: &dyn ::std::ops::Fn(metamodelica::Ref<Algorithm::NFAlgorithm>) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>>,
) -> Result<metamodelica::Ref<NFSections>> {
    pub type EquationFn = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Equation::NFEquation>) -> Result<metamodelica::Ref<Equation::NFEquation>>
            + 'static,
    >;

    pub type AlgorithmFn = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Algorithm::NFAlgorithm>,
            ) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>>
            + 'static,
    >;

    let mut sections: metamodelica::Ref<NFSections> = sections;
    let mut eq: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut ieq: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut alg: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
    let mut ialg: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
    let () = (match &*sections {
        SECTIONS {
            algorithms: __sections_algorithms,
            equations: __sections_equations,
            initialAlgorithms: __sections_initialAlgorithms,
            initialEquations: __sections_initialEquations,
        } => {
            eq = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
                for mut e in (__sections_equations.clone()).into_iter().cloned() {
                    let __x = eqFn(e.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            ieq = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
                for mut e in (__sections_initialEquations.clone()).into_iter().cloned() {
                    let __x = ieqFn(e.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            alg = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
                for mut a in (__sections_algorithms.clone()).into_iter().cloned() {
                    let __x = algFn(a.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            ialg = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
                for mut a in (__sections_initialAlgorithms.clone()).into_iter().cloned() {
                    let __x = ialgFn(a.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            sections = metamodelica::Ref::new(NFSections::SECTIONS {
                equations: eq,
                initialEquations: ieq,
                algorithms: alg,
                initialAlgorithms: ialg,
            });
            ()
        }
        _ => (),
    });
    Ok(sections)
}

pub fn eqId(mut eq: metamodelica::Ref<Equation::NFEquation>) -> metamodelica::Ref<Equation::NFEquation> {
    let mut eq: metamodelica::Ref<Equation::NFEquation> = eq;
    eq
}

pub fn algId(mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>) -> metamodelica::Ref<Algorithm::NFAlgorithm> {
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm> = alg;
    alg
}

pub(crate) fn map1<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut sections: metamodelica::Ref<NFSections>,
    mut arg: ArgT,
    mut eqFn: &dyn ::std::ops::Fn(
        metamodelica::Ref<Equation::NFEquation>,
        ArgT,
    ) -> Result<metamodelica::Ref<Equation::NFEquation>>,
    mut algFn: &dyn ::std::ops::Fn(
        metamodelica::Ref<Algorithm::NFAlgorithm>,
        ArgT,
    ) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut ieqFn: &dyn ::std::ops::Fn(
        metamodelica::Ref<Equation::NFEquation>,
        ArgT,
    ) -> Result<metamodelica::Ref<Equation::NFEquation>>,
    mut ialgFn: &dyn ::std::ops::Fn(
        metamodelica::Ref<Algorithm::NFAlgorithm>,
        ArgT,
    ) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>>,
) -> Result<metamodelica::Ref<NFSections>> {
    pub type EquationFn<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Equation::NFEquation>,
                ArgT,
            ) -> Result<metamodelica::Ref<Equation::NFEquation>>
            + 'static,
    >;

    pub type AlgorithmFn<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Algorithm::NFAlgorithm>,
                ArgT,
            ) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>>
            + 'static,
    >;

    let mut sections: metamodelica::Ref<NFSections> = sections;
    let mut eq: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut ieq: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut alg: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
    let mut ialg: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
    let () = (match &*sections {
        SECTIONS {
            algorithms: __sections_algorithms,
            equations: __sections_equations,
            initialAlgorithms: __sections_initialAlgorithms,
            initialEquations: __sections_initialEquations,
        } => {
            eq = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
                for mut e in (__sections_equations.clone()).into_iter().cloned() {
                    let __x = eqFn(e.clone(), arg.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            ieq = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
                for mut e in (__sections_initialEquations.clone()).into_iter().cloned() {
                    let __x = ieqFn(e.clone(), arg.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            alg = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
                for mut a in (__sections_algorithms.clone()).into_iter().cloned() {
                    let __x = algFn(a.clone(), arg.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            ialg = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
                for mut a in (__sections_initialAlgorithms.clone()).into_iter().cloned() {
                    let __x = ialgFn(a.clone(), arg.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            sections = metamodelica::Ref::new(NFSections::SECTIONS {
                equations: eq,
                initialEquations: ieq,
                algorithms: alg,
                initialAlgorithms: ialg,
            });
            ()
        }
        _ => (),
    });
    Ok(sections)
}

pub(crate) fn mapExp(
    mut sections: metamodelica::Ref<NFSections>,
    mut mapFn: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NFSections>> {
    pub type MapFn = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut sections: metamodelica::Ref<NFSections> = sections;
    let mut eq: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut ieq: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut alg: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
    let mut ialg: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
    sections = (match &*sections {
        SECTIONS {
            algorithms: __sections_algorithms,
            equations: __sections_equations,
            initialAlgorithms: __sections_initialAlgorithms,
            initialEquations: __sections_initialEquations,
        } => {
            eq = Equation::mapExpList(__sections_equations.clone(), mapFn)?;
            ieq = Equation::mapExpList(__sections_initialEquations.clone(), mapFn)?;
            alg = Algorithm::mapExpList(__sections_algorithms.clone(), mapFn)?;
            ialg = Algorithm::mapExpList(__sections_initialAlgorithms.clone(), mapFn)?;
            metamodelica::Ref::new(NFSections::SECTIONS {
                equations: eq,
                initialEquations: ieq,
                algorithms: alg,
                initialAlgorithms: ialg,
            })
        }
        EXTERNAL {
            args: __sections_args, ..
        } => {
            assign_variant_field!(sections => NFSections::EXTERNAL; args = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut e in (__sections_args.clone()).into_iter().cloned() {
                    let __x = mapFn(e.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            sections
        }
        _ => sections,
    });
    Ok(sections)
}

pub(crate) fn foldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut sections: &metamodelica::Ref<NFSections>,
    mut foldFn: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFn<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    arg = (match &**sections {
        SECTIONS {
            algorithms: __sections_algorithms,
            equations: __sections_equations,
            initialAlgorithms: __sections_initialAlgorithms,
            initialEquations: __sections_initialEquations,
        } => {
            arg = Equation::foldExpList(metamodelica::AsArg::as_arg(&__sections_equations), foldFn, arg)?;
            arg = Equation::foldExpList(metamodelica::AsArg::as_arg(&__sections_initialEquations), foldFn, arg)?;
            arg = Algorithm::foldExpList(metamodelica::AsArg::as_arg(&__sections_algorithms), foldFn, arg)?;
            arg = Algorithm::foldExpList(metamodelica::AsArg::as_arg(&__sections_initialAlgorithms), foldFn, arg)?;
            arg
        }
        EXTERNAL {
            args: __sections_args, ..
        } => List::fold(metamodelica::AsArg::as_arg(&__sections_args), foldFn, arg)?,
        _ => arg,
    });
    Ok(arg)
}

pub(crate) fn apply(
    mut sections: &metamodelica::Ref<NFSections>,
    mut eqFn: &dyn ::std::ops::Fn(metamodelica::Ref<Equation::NFEquation>) -> Result<()>,
    mut algFn: &dyn ::std::ops::Fn(metamodelica::Ref<Algorithm::NFAlgorithm>) -> Result<()>,
    mut ieqFn: &dyn ::std::ops::Fn(metamodelica::Ref<Equation::NFEquation>) -> Result<()>,
    mut ialgFn: &dyn ::std::ops::Fn(metamodelica::Ref<Algorithm::NFAlgorithm>) -> Result<()>,
) -> Result<()> {
    pub type EquationFn =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Equation::NFEquation>) -> Result<()> + 'static>;

    pub type AlgorithmFn =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Algorithm::NFAlgorithm>) -> Result<()> + 'static>;

    let () = (match &**sections {
        SECTIONS {
            algorithms: __sections_algorithms,
            equations: __sections_equations,
            initialAlgorithms: __sections_initialAlgorithms,
            initialEquations: __sections_initialEquations,
        } => {
            for mut eq in &*__sections_equations.clone() {
                eqFn(eq.clone())?;
            }
            for mut ieq in &*__sections_initialEquations.clone() {
                ieqFn(ieq.clone())?;
            }
            for mut alg in &*__sections_algorithms.clone() {
                algFn(alg.clone())?;
            }
            for mut ialg in &*__sections_initialAlgorithms.clone() {
                ialgFn(ialg.clone())?;
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn isEmpty(mut sections: &metamodelica::Ref<NFSections>) -> bool {
    let mut isEmpty: bool;
    isEmpty = (match &**sections {
        EMPTY { .. } => true,
        _ => false,
    });
    isEmpty
}

pub(crate) fn toStream(
    mut sections: &metamodelica::Ref<NFSections>,
    mut indent: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let () = (match &**sections {
        SECTIONS {
            algorithms: __sections_algorithms,
            ..
        } => {
            for mut alg in &*__sections_algorithms.clone() {
                s = IOStream::append(s, indent.clone())?;
                s = IOStream::append(s, literal!("algorithm\n"))?;
                s = Statement::toStreamList(
                    &alg.statements,
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*indent);
                        __mm_s.push_str(&*literal!("  "));
                        ArcStr::from(__mm_s)
                    },
                    s,
                )?;
            }
            ()
        }
        EXTERNAL {
            ann: __sections_ann,
            args: __sections_args,
            explicit: __sections_explicit,
            language: __sections_language,
            name: __sections_name,
            outputRef: __sections_outputRef,
            ..
        } => {
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("external \""))?;
            s = IOStream::append(s, __sections_language.clone())?;
            s = IOStream::append(s, literal!("\""))?;
            if __sections_explicit.clone() {
                if !(ComponentRef::isEmpty(metamodelica::AsArg::as_arg(&__sections_outputRef))) {
                    s = IOStream::append(s, literal!(" "))?;
                    s = IOStream::append(
                        s,
                        ComponentRef::toString(metamodelica::AsArg::as_arg(&__sections_outputRef))?,
                    )?;
                    s = IOStream::append(s, literal!(" ="))?;
                }
                s = IOStream::append(s, literal!(" "))?;
                s = IOStream::append(s, __sections_name.clone())?;
                s = IOStream::append(s, literal!("("))?;
                s = IOStream::append(
                    s,
                    stringDelimitList(
                        ({
                            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                            for mut e in (__sections_args.clone()).into_iter().cloned() {
                                let __x = Expression::toString(e.clone())?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                        literal!(", "),
                    ),
                )?;
                s = IOStream::append(s, literal!(")"))?;
            }
            if (__sections_ann).is_some() {
                s = IOStream::append(
                    s,
                    DAEDumpTypes::dumpCompAnnotationStr(Some(metamodelica::Ref::new(SCode::Comment {
                        annotation_: __sections_ann.clone(),
                        comment: None,
                    }))),
                )?;
            }
            s = IOStream::append(s, literal!(";\n"))?;
            ()
        }
        _ => (),
    });
    Ok(s)
}

pub(crate) fn toFlatStream(
    mut sections: &metamodelica::Ref<NFSections>,
    mut scopeName: &metamodelica::Ref<Absyn::Path>,
    mut format: BaseModelica::OutputFormat,
    mut indent: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut ann: metamodelica::Ref<SCode::Annotation>;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let mut modLib: metamodelica::Ref<SCode::Mod>;
    let mut modInc: metamodelica::Ref<SCode::Mod>;
    let mut modLibDir: metamodelica::Ref<SCode::Mod>;
    let mut modIncDir: metamodelica::Ref<SCode::Mod>;
    let () = (match &**sections {
        SECTIONS {
            algorithms: __sections_algorithms,
            ..
        } => {
            for mut alg in &*__sections_algorithms.clone() {
                s = IOStream::append(s, indent.clone())?;
                s = IOStream::append(s, literal!("algorithm\n"))?;
                s = Statement::toFlatStreamList(
                    &alg.statements,
                    format,
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*indent);
                        __mm_s.push_str(&*literal!("  "));
                        ArcStr::from(__mm_s)
                    },
                    s,
                )?;
            }
            ()
        }
        EXTERNAL {
            ann: __sections_ann,
            args: __sections_args,
            explicit: __sections_explicit,
            language: __sections_language,
            name: __sections_name,
            outputRef: __sections_outputRef,
            ..
        } => {
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("external \""))?;
            s = IOStream::append(s, __sections_language.clone())?;
            s = IOStream::append(s, literal!("\""))?;
            if __sections_explicit.clone() {
                if !(ComponentRef::isEmpty(metamodelica::AsArg::as_arg(&__sections_outputRef))) {
                    s = IOStream::append(s, literal!(" "))?;
                    s = IOStream::append(
                        s,
                        ComponentRef::toFlatString(metamodelica::AsArg::as_arg(&__sections_outputRef), format)?,
                    )?;
                    s = IOStream::append(s, literal!(" ="))?;
                }
                s = IOStream::append(s, literal!(" "))?;
                s = IOStream::append(s, __sections_name.clone())?;
                s = IOStream::append(s, literal!("("))?;
                s = IOStream::append(
                    s,
                    stringDelimitList(
                        ({
                            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                            for mut e in (__sections_args.clone()).into_iter().cloned() {
                                let __x = Expression::toFlatString(e.clone(), format)?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                        literal!(", "),
                    ),
                )?;
                s = IOStream::append(s, literal!(")"))?;
            }
            if (__sections_ann).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(__sections_ann.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                ann = metamodelica::Own::own(__pa0);
                r#mod = ann.modification.clone();
                modLib = SCodeUtil::filterSubMods(
                    r#mod.clone(),
                    &({
                        let __pe_b1 = list![literal!("Library")];
                        move |__pe_a0| Ok(SCodeUtil::filterGivenSubModNames(&__pe_a0, __pe_b1.clone()))
                    }),
                )?;
                modInc = SCodeUtil::filterSubMods(
                    r#mod.clone(),
                    &({
                        let __pe_b1 = list![literal!("Include")];
                        move |__pe_a0| Ok(SCodeUtil::filterGivenSubModNames(&__pe_a0, __pe_b1.clone()))
                    }),
                )?;
                if SCodeUtil::isEmptyMod(&modLib) {
                    modLibDir = openmodelica_frontend_types::SCode::Mod::interned_NOMOD();
                } else {
                    modLibDir = SCodeUtil::filterSubMods(
                        r#mod.clone(),
                        &({
                            let __pe_b1 = list![literal!("LibraryDirectory")];
                            move |__pe_a0| Ok(SCodeUtil::filterGivenSubModNames(&__pe_a0, __pe_b1.clone()))
                        }),
                    )?;
                    if SCodeUtil::isEmptyMod(&modLibDir) {
                        modLibDir = metamodelica::Ref::new(SCode::Mod::MOD {
                            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                            eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                            subModLst: list![metamodelica::Ref::new(SCode::SubMod {
                                ident: literal!("LibraryDirectory"),
                                r#mod: metamodelica::Ref::new(SCode::Mod::MOD {
                                    finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                                    eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                                    subModLst: metamodelica::nil(),
                                    binding: Some(metamodelica::Ref::new(Absyn::Exp::STRING {
                                        value: {
                                            let mut __mm_s = String::new();
                                            __mm_s.push_str(&*literal!("modelica://"));
                                            __mm_s.push_str(&*AbsynUtil::pathFirstIdent(scopeName));
                                            __mm_s.push_str(&*literal!("/Resources/Library"));
                                            ArcStr::from(__mm_s)
                                        }
                                    })),
                                    comment: None,
                                    info: Error::dummyInfo.clone()
                                })
                            })],
                            binding: None,
                            comment: None,
                            info: Error::dummyInfo.clone(),
                        });
                    }
                }
                if SCodeUtil::isEmptyMod(&modInc) {
                    modIncDir = openmodelica_frontend_types::SCode::Mod::interned_NOMOD();
                } else {
                    modIncDir = SCodeUtil::filterSubMods(
                        r#mod,
                        &({
                            let __pe_b1 = list![literal!("IncludeDirectory")];
                            move |__pe_a0| Ok(SCodeUtil::filterGivenSubModNames(&__pe_a0, __pe_b1.clone()))
                        }),
                    )?;
                    if SCodeUtil::isEmptyMod(&modLibDir) {
                        modLibDir = metamodelica::Ref::new(SCode::Mod::MOD {
                            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                            eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                            subModLst: list![metamodelica::Ref::new(SCode::SubMod {
                                ident: literal!("IncludeDirectory"),
                                r#mod: metamodelica::Ref::new(SCode::Mod::MOD {
                                    finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                                    eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                                    subModLst: metamodelica::nil(),
                                    binding: Some(metamodelica::Ref::new(Absyn::Exp::STRING {
                                        value: {
                                            let mut __mm_s = String::new();
                                            __mm_s.push_str(&*literal!("modelica://"));
                                            __mm_s.push_str(&*AbsynUtil::pathFirstIdent(scopeName));
                                            __mm_s.push_str(&*literal!("/Resources/Include"));
                                            ArcStr::from(__mm_s)
                                        }
                                    })),
                                    comment: None,
                                    info: Error::dummyInfo.clone()
                                })
                            })],
                            binding: None,
                            comment: None,
                            info: Error::dummyInfo.clone(),
                        });
                    }
                }
                assign_field!(
                    ann.modification = SCodeUtil::mergeSCodeMods(
                        SCodeUtil::mergeSCodeMods(modLib, modLibDir)?,
                        SCodeUtil::mergeSCodeMods(modInc, modIncDir)?
                    )?
                );
                s = IOStream::append(
                    s,
                    SCodeDump::printAnnotationStr(
                        &(metamodelica::Ref::new(SCode::Comment {
                            annotation_: Some(ann),
                            comment: None,
                        })),
                        SCodeDump::defaultOptions.clone(),
                    )?,
                )?;
            }
            s = IOStream::append(s, literal!(";\n"))?;
            ()
        }
        _ => (),
    });
    Ok(s)
}
