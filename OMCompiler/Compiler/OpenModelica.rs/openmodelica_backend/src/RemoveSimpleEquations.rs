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

use crate::AvlSetInt;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::EvaluateFunctions;
use crate::ExpressionSolve;
use crate::HashTableCrToCrEqLst;
use crate::SimCodeUtil;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::HashTableCrToExp;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

/// eqnAttributes(source,EquationAttributes)
pub type EquationSourceAndAttributes = (metamodelica::Ref<DAE::ElementSource>, BackendDAE::EquationAttributes);

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum SimpleContainer {
    ALIAS {
        cr1: metamodelica::Ref<DAE::ComponentRef>,
        negatedCr1: bool,
        i1: i32,
        cr2: metamodelica::Ref<DAE::ComponentRef>,
        negatedCr2: bool,
        i2: i32,
        eqnAttributes: EquationSourceAndAttributes,
        visited: i32,
    },
    PARAMETERALIAS {
        unknowncr: metamodelica::Ref<DAE::ComponentRef>,
        negatedCr1: bool,
        i1: i32,
        paramcr: metamodelica::Ref<DAE::ComponentRef>,
        negatedCr2: bool,
        i2: i32,
        eqnAttributes: EquationSourceAndAttributes,
        visited: i32,
    },
    TIMEALIAS {
        cr1: metamodelica::Ref<DAE::ComponentRef>,
        negatedCr1: bool,
        i1: i32,
        cr2: metamodelica::Ref<DAE::ComponentRef>,
        negatedCr2: bool,
        i2: i32,
        eqnAttributes: EquationSourceAndAttributes,
        visited: i32,
    },
    TIMEINDEPENTVAR {
        cr: metamodelica::Ref<DAE::ComponentRef>,
        i: i32,
        exp: metamodelica::Ref<DAE::Exp>,
        eqnAttributes: EquationSourceAndAttributes,
        visited: i32,
    },
}
impl metamodelica::gc::MMTrace for SimpleContainer {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            SimpleContainer::ALIAS {
                cr1,
                negatedCr1,
                i1,
                cr2,
                negatedCr2,
                i2,
                eqnAttributes,
                visited,
            } => {
                metamodelica::gc::MMTrace::mm_accept(cr1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(negatedCr1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(i1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cr2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(negatedCr2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(i2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqnAttributes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(visited, __mmv)?;
                Ok(())
            }
            SimpleContainer::PARAMETERALIAS {
                unknowncr,
                negatedCr1,
                i1,
                paramcr,
                negatedCr2,
                i2,
                eqnAttributes,
                visited,
            } => {
                metamodelica::gc::MMTrace::mm_accept(unknowncr, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(negatedCr1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(i1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(paramcr, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(negatedCr2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(i2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqnAttributes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(visited, __mmv)?;
                Ok(())
            }
            SimpleContainer::TIMEALIAS {
                cr1,
                negatedCr1,
                i1,
                cr2,
                negatedCr2,
                i2,
                eqnAttributes,
                visited,
            } => {
                metamodelica::gc::MMTrace::mm_accept(cr1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(negatedCr1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(i1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cr2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(negatedCr2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(i2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqnAttributes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(visited, __mmv)?;
                Ok(())
            }
            SimpleContainer::TIMEINDEPENTVAR {
                cr,
                i,
                exp,
                eqnAttributes,
                visited,
            } => {
                metamodelica::gc::MMTrace::mm_accept(cr, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(i, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqnAttributes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(visited, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for SimpleContainer {
    fn default() -> Self {
        Self::TIMEINDEPENTVAR {
            cr: Default::default(),
            i: Default::default(),
            exp: Default::default(),
            eqnAttributes: Default::default(),
            visited: Default::default(),
        }
    }
}
pub use self::SimpleContainer::{ALIAS, PARAMETERALIAS, TIMEALIAS, TIMEINDEPENTVAR};

pub type AccTuple = (
    BackendDAE::Variables,
    metamodelica::Ref<BackendDAE::Shared>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<SimpleContainer>,
    i32,
    metamodelica::Array<metamodelica::List<i32>>,
    bool,
);

/// strongest origin seen so far, start values with that origin
pub type StartValues = (
    Option<DAE::StartOrigin>,
    metamodelica::List<(
        Option<metamodelica::Ref<DAE::Exp>>,
        metamodelica::Ref<DAE::ComponentRef>,
    )>,
);

/// fixed, startvalues, nominal, (min, max)
pub type VarSetAttributes = (
    bool,
    (
        Option<DAE::StartOrigin>,
        metamodelica::List<(
            Option<metamodelica::Ref<DAE::Exp>>,
            metamodelica::Ref<DAE::ComponentRef>,
        )>,
    ),
    metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>,
    (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>),
);

thread_local! { static __EMPTYVARSETATTRIBUTES_TLS: (bool, (Option<DAE::StartOrigin>, metamodelica::List<(Option<metamodelica::Ref<DAE::Exp>>, metamodelica::Ref<DAE::ComponentRef>)>), metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>, (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>)) = (false, (None, metamodelica::nil()), metamodelica::nil(), (None, None)); }
pub(crate) fn EMPTYVARSETATTRIBUTES() -> (
    bool,
    (
        Option<DAE::StartOrigin>,
        metamodelica::List<(
            Option<metamodelica::Ref<DAE::Exp>>,
            metamodelica::Ref<DAE::ComponentRef>,
        )>,
    ),
    metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>,
    (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>),
) {
    __EMPTYVARSETATTRIBUTES_TLS.with(|__t| __t.clone())
}

// =============================================================================
// Starting point for preOpt and postOpt removeSimpleEquations module
//
// =============================================================================
pub(crate) fn removeSimpleEquations(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut rebuilt: bool;
    if BackendDAEUtil::hasDAEMatching(&inDAE)? {
        outDAE = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::REMOVE_SIMPLE_EQUATIONS.clone())?) {
            Deref @ "default" => causal(&inDAE)?,
            Deref @ "causal" => causal(&inDAE)?,
            Deref @ "new" => performAliasEliminationBB(&inDAE, true)?,
            _ => inDAE.clone(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        (outDAE, rebuilt) = fixAliasVars(outDAE)?;
        outDAE = fixAliasAndKnownVarsCausal(&inDAE, outDAE, !(rebuilt))?;
        outDAE = fixAliasVarsVariablity(outDAE)?;
    } else {
        outDAE = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::REMOVE_SIMPLE_EQUATIONS.clone())?) {
            Deref @ "default" => fastAcausal(&inDAE)?,
            Deref @ "fastAcausal" => fastAcausal(&inDAE)?,
            Deref @ "allAcausal" => allAcausal(&inDAE)?,
            Deref @ "new" => performAliasEliminationBB(&inDAE, true)?,
            _ => inDAE,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        (outDAE, _) = fixAliasVars(outDAE)?;
        outDAE = fixKnownVars(outDAE)?;
        outDAE = fixAliasVarsVariablity(outDAE)?;
    }
    Ok(outDAE)
}

pub(crate) fn removeVerySimpleEquations(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    if BackendDAEUtil::hasDAEMatching(&inDAE)? {
        Error::addInternalError(
            literal!("Cannot run removeVerySimpleEquations on a matched system (continuing anyway)"),
            metamodelica::sourceInfo!("BackEnd/RemoveSimpleEquations.mo"),
        )?;
        outDAE = inDAE;
    } else {
        outDAE = performAliasEliminationBB(&inDAE, true)?;
    }
    Ok(outDAE)
}

pub(crate) fn fixAliasVarsVariablity(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut aliasVars: BackendDAE::Variables;
    let mut systvars: BackendDAE::Variables;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut binding: metamodelica::Ref<DAE::Exp>;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut paramOrConst: bool;
    let mut r#const: bool;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut referencevar: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut tempreferencevar: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut knownVarList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut aliasVarList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut tempvar: metamodelica::Ref<BackendDAE::Var>;
    if !(Flags::getConfigBool(Flags::BUILDING_FMU.clone())?) {
        outDAE = inDAE;
        return Ok(outDAE);
    }
    aliasVars = BackendDAEUtil::getAliasVars(&inDAE)?;
    systvars = BackendVariable::listVar(BackendVariable::equationSystemsVarsLst(&inDAE.eqs)?)?;
    for mut var in &*BackendVariable::varList(&aliasVars)? {
        binding = BackendVariable::varBindExp(metamodelica::AsArg::as_arg(&var))?;
        crefs = Expression::getAllCrefs(binding)?;
        referencevar = metamodelica::nil();
        for mut cr in &*crefs {
            tempreferencevar = getVarsHelper(cr.clone(), &systvars);
            if (tempreferencevar).is_empty() {
                tempreferencevar = getVarsHelper(cr.clone(), &inDAE.shared.globalKnownVars);
            }
            referencevar = listAppend(tempreferencevar, referencevar);
        }
        if (referencevar).is_empty() {
            paramOrConst = false;
            r#const = false;
        } else {
            paramOrConst = List::all(
                &referencevar,
                &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BackendVariable::isParamOrConstant(&__a0))
                },
            )?;
            r#const = List::all(
                &referencevar,
                &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BackendVariable::isConst(&__a0))
                },
            )?;
        }
        if r#const {
            tempvar = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::CONST)?;
            knownVarList = metamodelica::cons(BackendVariable::setVarFixed(tempvar, true)?, knownVarList);
        } else if paramOrConst {
            tempvar = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::PARAM)?;
            knownVarList = metamodelica::cons(BackendVariable::setVarFixed(tempvar, true)?, knownVarList);
        } else {
            aliasVarList = metamodelica::cons(var.clone(), aliasVarList);
        }
    }
    globalKnownVars = BackendVariable::mergeVariables(
        inDAE.shared.globalKnownVars.clone(),
        BackendVariable::listVar(knownVarList)?,
        true,
    )?;
    outDAE = BackendDAEUtil::setAliasVars(&inDAE, BackendVariable::listVar(aliasVarList)?);
    outDAE = BackendDAEUtil::setDAEGlobalKnownVars(&outDAE, globalKnownVars);
    Ok(outDAE)
}

fn getVarsHelper(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut vars: &BackendDAE::Variables,
) -> metamodelica::List<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    match '__try0: {
        (outVars, _) = unwrap_break_err!(BackendVariable::getVar(cr.clone(), vars), '__try0);
        Ok::<_, &'static str>((outVars.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outVars = __try0_o0;
        }
        Err(_) => {
            outVars = metamodelica::nil();
        }
    }
    outVars
}

fn fixAliasVars(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<(metamodelica::Ref<BackendDAE::BackendDAE>, bool)> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut rebuilt: bool = false;
    let mut aliasVars: BackendDAE::Variables;
    let mut aliasVarList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut movedVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut binding: metamodelica::Ref<DAE::Exp>;
    aliasVars = BackendDAEUtil::getAliasVars(&inDAE)?;
    for mut var in &*BackendVariable::varList(&aliasVars)? {
        binding = BackendVariable::varBindExp(metamodelica::AsArg::as_arg(&var))?;
        if Expression::isConst(binding)? {
            movedVars = metamodelica::cons(var.clone(), movedVars);
        } else {
            aliasVarList = metamodelica::cons(var.clone(), aliasVarList);
        }
    }
    if (movedVars).is_empty() {
        outDAE = inDAE;
        return Ok((outDAE, rebuilt));
    }
    outDAE = BackendDAEUtil::setAliasVars(&inDAE, BackendVariable::listVar(aliasVarList)?);
    outDAE = BackendDAEUtil::setDAEGlobalKnownVars(
        &outDAE,
        BackendVariable::listVar(listAppend(
            movedVars,
            BackendVariable::varList(&(BackendDAEUtil::getGlobalKnownVarsFromDAE(&inDAE)?))?,
        ))?,
    );
    rebuilt = true;
    Ok((outDAE, rebuilt))
}

fn fixKnownVars(
    mut dae: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut dae: metamodelica::Ref<BackendDAE::BackendDAE> = dae;
    let mut eqs: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut binding: metamodelica::Ref<DAE::Exp>;
    let mut eqnList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut varList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut knownVarList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    globalKnownVars = dae.shared.globalKnownVars.clone();
    for mut var in &*BackendVariable::varList(&globalKnownVars)? {
        if BackendVariable::varHasBindExp(metamodelica::AsArg::as_arg(&var)) {
            binding = BackendVariable::varBindExp(metamodelica::AsArg::as_arg(&var))?;
            (_, crlst) = Expression::traverseExpTopDown(
                binding.clone(),
                &Expression::traversingComponentRefFinderNoPreDer,
                metamodelica::nil(),
            )?;
            if BackendDAEUtil::containAnyVar(&crlst, dae.shared.localKnownVars.clone())? {
                varList = metamodelica::cons(BackendVariable::setBindExp(var.clone(), None), varList);
                eqnList = metamodelica::cons(
                    metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                        exp: BackendVariable::varExp(metamodelica::AsArg::as_arg(&var))?,
                        scalar: binding,
                        source: DAE::emptyElementSource().clone(),
                        attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone(),
                    }),
                    eqnList,
                );
            } else {
                knownVarList = metamodelica::cons(var.clone(), knownVarList);
            }
        } else {
            knownVarList = metamodelica::cons(var.clone(), knownVarList);
        }
    }
    if !((varList).is_empty()) {
        eqs = BackendDAEUtil::createEqSystem(
            BackendVariable::listVar(varList)?,
            BackendEquation::listEquation(&eqnList)?,
            metamodelica::nil(),
            openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNSPECIFIED_PARTITION,
            BackendEquation::emptyEqns(),
        );
        assign_field!(dae.eqs = metamodelica::cons(eqs, dae.eqs.clone()));
    }
    dae = BackendDAEUtil::setDAEGlobalKnownVars(&dae, BackendVariable::listVar(knownVarList)?);
    Ok(dae)
}

fn fixAliasAndKnownVarsCausal(
    mut inDAE1: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inDAE2: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut visitReversed: bool,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE2.clone();
    let mut knownVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut aliasVars1: BackendDAE::Variables;
    let mut knownVars1: BackendDAE::Variables;
    let mut aliasVars2: BackendDAE::Variables;
    let mut knownVars2: BackendDAE::Variables;
    let mut aliasVarList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut knownVarList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    aliasVars1 = BackendDAEUtil::getAliasVars(inDAE1)?;
    aliasVars2 = BackendDAEUtil::getAliasVars(&inDAE2)?;
    knownVars1 = BackendDAEUtil::getGlobalKnownVarsFromDAE(inDAE1)?;
    knownVars2 = BackendDAEUtil::getGlobalKnownVarsFromDAE(&inDAE2)?;
    for mut var in &*BackendVariable::varList(&aliasVars2)? {
        cref = BackendVariable::varCref(metamodelica::AsArg::as_arg(&var));
        if !(BackendVariable::existsVar(cref, &aliasVars1, false)) {
            outDAE = fixAliasVarsCausal2(var.clone(), &outDAE)?;
        } else {
            aliasVarList = metamodelica::cons(var.clone(), aliasVarList);
        }
    }
    outDAE = BackendDAEUtil::setAliasVars(&outDAE, BackendVariable::listVar(aliasVarList)?);
    knownVars = BackendVariable::varList(&knownVars2)?;
    for mut var in &*if (visitReversed) {
        knownVars.reverse()
    } else {
        knownVars
    } {
        cref = BackendVariable::varCref(metamodelica::AsArg::as_arg(&var));
        if !(BackendVariable::existsVar(cref, &knownVars1, false))
            && !(BackendVariable::isInput(metamodelica::AsArg::as_arg(&var))
                || BackendVariable::isAlgebraicOldState(metamodelica::AsArg::as_arg(&var)))
        {
            outDAE = fixKnownVarsCausal2(var.clone(), &outDAE)?;
        } else {
            knownVarList = metamodelica::cons(var.clone(), knownVarList);
        }
    }
    outDAE = BackendDAEUtil::setDAEGlobalKnownVars(
        &outDAE,
        BackendVariable::listVar(if (visitReversed) {
            knownVarList.reverse()
        } else {
            knownVarList
        })?,
    );
    Ok(outDAE)
}

fn fixAliasVarsCausal2(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut binding: metamodelica::Ref<DAE::Exp>;
    let mut rightCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut eqs1: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut done: bool = false;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut orderedVars: BackendDAE::Variables;
    let mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    match '__try0: {
        binding = unwrap_break_err!(BackendVariable::varBindExp(&inVar), '__try0);
        rightCrefs = unwrap_break_err!(Expression::getAllCrefs(binding.clone()), '__try0);
        let __arc3 = &(*inDAE);
        let BackendDAE::DAE {
            eqs: __pa1,
            shared: __pa2,
        } = &**__arc3;
        eqs = metamodelica::Own::own(__pa1);
        shared = metamodelica::Own::own(__pa2);
        var = BackendVariable::setBindExp(inVar.clone(), None);
        var = unwrap_break_err!(BackendVariable::setVarFixed(var.clone(), false), '__try0);
        eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
            exp: unwrap_break_err!(BackendVariable::varExp(&var), '__try0),
            scalar: binding.clone(),
            source: DAE::emptyElementSource().clone(),
            attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone(),
        });
        for mut eq in &*eqs {
            let __arc6 = eq.clone();
            let BackendDAE::EQSYSTEM {
                orderedVars: __pa4,
                orderedEqs: __pa5,
                ..
            } = &*__arc6;
            orderedVars = metamodelica::Own::own(__pa4);
            orderedEqs = metamodelica::Own::own(__pa5);
            if BackendVariable::existsAnyVar(&rightCrefs, &orderedVars, false) {
                orderedVars = unwrap_break_err!(BackendVariable::addVar(var.clone(), orderedVars.clone()), '__try0);
                orderedEqs = unwrap_break_err!(BackendEquation::add(eqn.clone(), orderedEqs.clone()), '__try0);
                eqs1 = metamodelica::cons(
                    BackendDAEUtil::setEqSystEqs(
                        BackendDAEUtil::setEqSystVars(eq.clone(), orderedVars.clone()),
                        orderedEqs.clone(),
                    ),
                    eqs1.clone(),
                );
                let false = (done) else {
                    break '__try0 Err::<_, _>("pattern mismatch");
                };
                done = true;
            } else {
                eqs1 = metamodelica::cons(eq.clone(), eqs1.clone());
            }
        }
        if !(done) {
            eqs1 = metamodelica::cons(
                BackendDAEUtil::createEqSystem(
                    unwrap_break_err!(BackendVariable::listVar(list![var.clone()]), '__try0),
                    unwrap_break_err!(BackendEquation::listEquation(&(list![eqn.clone()])), '__try0),
                    metamodelica::nil(),
                    openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNSPECIFIED_PARTITION,
                    BackendEquation::emptyEqns(),
                ),
                eqs1.clone(),
            );
        }
        outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: eqs1.clone().reverse(),
            shared: shared.clone(),
        });
        Ok::<_, &'static str>((
            binding.clone(),
            eqn.clone(),
            eqs.clone(),
            outDAE.clone(),
            rightCrefs.clone(),
            shared.clone(),
            var.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6)) => {
            binding = __try0_o0;
            eqn = __try0_o1;
            eqs = __try0_o2;
            outDAE = __try0_o3;
            rightCrefs = __try0_o4;
            shared = __try0_o5;
            var = __try0_o6;
        }
        Err(__try0_err) => {
            BackendDump::dumpVarList(
                &(list![inVar.clone()]),
                &(literal!("fixAliasVarsCausal2 failed for ...")),
            )?;
            Error::addCompilerError(literal!("fixAliasVarsCausal2 failed"))?;
            return Err(__try0_err);
        }
    }
    Ok(outDAE)
}

fn fixKnownVarsCausal2(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut binding: metamodelica::Ref<DAE::Exp>;
    let mut rightCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut eqs1: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut done: bool = false;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut orderedVars: BackendDAE::Variables;
    let mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    match '__try0: {
        binding = unwrap_break_err!(BackendVariable::varBindExp(&inVar), '__try0);
        rightCrefs = unwrap_break_err!(Expression::getAllCrefs(binding.clone()), '__try0);
        var = BackendVariable::setBindExp(inVar.clone(), None);
        eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
            exp: unwrap_break_err!(BackendVariable::varExp(&var), '__try0),
            scalar: binding.clone(),
            source: DAE::emptyElementSource().clone(),
            attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone(),
        });
        for mut eq in &*inDAE.eqs.clone() {
            let __arc3 = eq.clone();
            let BackendDAE::EQSYSTEM {
                orderedVars: __pa1,
                orderedEqs: __pa2,
                ..
            } = &*__arc3;
            orderedVars = metamodelica::Own::own(__pa1);
            orderedEqs = metamodelica::Own::own(__pa2);
            if BackendVariable::existsAnyVar(&rightCrefs, &orderedVars, false) {
                orderedVars = unwrap_break_err!(BackendVariable::addVar(var.clone(), orderedVars.clone()), '__try0);
                orderedEqs = unwrap_break_err!(BackendEquation::add(eqn.clone(), orderedEqs.clone()), '__try0);
                eqs1 = metamodelica::cons(
                    BackendDAEUtil::setEqSystEqs(
                        BackendDAEUtil::setEqSystVars(eq.clone(), orderedVars.clone()),
                        orderedEqs.clone(),
                    ),
                    eqs1.clone(),
                );
                let false = (done) else {
                    break '__try0 Err::<_, _>("pattern mismatch");
                };
                done = true;
            } else {
                eqs1 = metamodelica::cons(eq.clone(), eqs1.clone());
            }
        }
        if !(done) {
            eqs1 = metamodelica::cons(
                BackendDAEUtil::createEqSystem(
                    unwrap_break_err!(BackendVariable::listVar(list![var.clone()]), '__try0),
                    unwrap_break_err!(BackendEquation::listEquation(&(list![eqn.clone()])), '__try0),
                    metamodelica::nil(),
                    openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNSPECIFIED_PARTITION,
                    BackendEquation::emptyEqns(),
                ),
                eqs1.clone(),
            );
        }
        outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: eqs1.clone().reverse(),
            shared: inDAE.shared.clone(),
        });
        Ok::<_, &'static str>((
            binding.clone(),
            eqn.clone(),
            outDAE.clone(),
            rightCrefs.clone(),
            var.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            binding = __try0_o0;
            eqn = __try0_o1;
            outDAE = __try0_o2;
            rightCrefs = __try0_o3;
            var = __try0_o4;
        }
        Err(__try0_err) => {
            BackendDump::dumpVarList(
                &(list![inVar.clone()]),
                &(literal!("fixKnownVarsCausal2 failed for ...")),
            )?;
            Error::addCompilerError(literal!("fixKnownVarsCausal2 failed"))?;
            return Err(__try0_err);
        }
    }
    Ok(outDAE)
}

// =============================================================================
// section for fastAcausal
//
// =============================================================================
pub(crate) fn fastAcausal(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut b: bool;
    let mut warnAliasConflicts: bool;
    let mut size: i32;
    let mut unReplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    size = BackendDAEUtil::daeSize(inDAE)?;
    size = intMax(
        BaseHashTable::defaultBucketSize.clone(),
        (((intReal(size)) * (metamodelica::OrderedFloat(0.7_f64))).0.floor() as i32),
    );
    repl = BackendVarTransform::emptyReplacementsSized(size);
    unReplaceable = HashSet::emptyHashSet();
    unReplaceable = BackendDAEUtil::foldEqSystem(
        inDAE,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: metamodelica::Ref<BackendDAE::Shared>,
               __a2: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        )| addUnreplaceable(&__a0, &__a1, __a2),
        unReplaceable,
    )?;
    (_, unReplaceable) = BackendDAEUtil::traverseBackendDAEExps(
        inDAE,
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                traverserExpUnreplaceable,
                metamodelica::Ref<DAE::Exp>,
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>
                    ),
                    i32,
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
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>
                    )
                )
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        )> + 'static,
                >),
            unReplaceable,
        ),
    )?;
    unReplaceable = addUnreplaceableFromWhens(inDAE, unReplaceable)?;
    if Flags::isSet(Flags::DUMP_REPL.clone())? {
        BackendDump::dumpHashSet(&unReplaceable, &(literal!("Unreplaceable Crefs:")))?;
    }
    let (__pa0, (__pa1, __pa2, _, _, __pa3)) = BackendDAEUtil::mapEqSystemAndFold(
        inDAE,
        &fnptr!(
            fastAcausal1,
            metamodelica::Ref<BackendDAE::EqSystem>,
            metamodelica::Ref<BackendDAE::Shared>,
            (
                BackendVarTransform::VariableReplacements,
                bool,
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>
                    ),
                    i32,
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
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>
                    )
                ),
                i32,
                bool
            )
        ),
        (
            repl,
            false,
            unReplaceable,
            Flags::getConfigInt(Flags::MAXTRAVERSALS.clone())?,
            false,
        ),
    )?;
    outDAE = metamodelica::Own::own(__pa0);
    repl = metamodelica::Own::own(__pa1);
    b = metamodelica::Own::own(__pa2);
    warnAliasConflicts = metamodelica::Own::own(__pa3);
    if warnAliasConflicts && BackendDAEUtil::isSimulationDAE(&inDAE.shared) {
        Error::addMessage(Error::REDUNDANT_ALIAS_SET.clone(), metamodelica::nil())?;
    }
    outDAE = removeSimpleEquationsShared(b, outDAE, repl)?;
    Ok(outDAE)
}

fn addUnreplaceable(
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
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
    ),
)> {
    let mut outUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ) = inUnreplaceable;
    let mut orderedVars: BackendDAE::Variables;
    let __arc1 = &(*syst);
    let BackendDAE::EQSYSTEM { orderedVars: __pa0, .. } = &**__arc1;
    orderedVars = metamodelica::Own::own(__pa0);
    for mut var in &*BackendVariable::varList(&orderedVars)? {
        if BackendVariable::varUnreplaceable(metamodelica::AsArg::as_arg(&var)) {
            outUnreplaceable = BaseHashSet::add(
                BackendVariable::varCref(metamodelica::AsArg::as_arg(&var)),
                &outUnreplaceable,
            )?;
        }
    }
    Ok(outUnreplaceable)
}

fn fastAcausal1(
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inTpl: (
        BackendVarTransform::VariableReplacements,
        bool,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
        i32,
        bool,
    ),
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendVarTransform::VariableReplacements,
        bool,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
        i32,
        bool,
    ),
) {
    let mut outSystem: metamodelica::Ref<BackendDAE::EqSystem> = BackendDAEUtil::copyEqSystem(&inSystem);
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outTpl: (
        BackendVarTransform::VariableReplacements,
        bool,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
        ),
        i32,
        bool,
    );
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut unReplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut simpleeqnslst: metamodelica::List<SimpleContainer>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut foundSimple: bool;
    let mut globalFoundSimple: bool;
    let mut warnAliasConflicts: bool;
    let mut maxTraversals: i32;
    if BackendDAEUtil::isClockedSyst(&inSystem) {
        outSystem = inSystem;
        outShared = inShared;
        outTpl = inTpl;
        return (outSystem, outShared, outTpl);
    }
    match '__try0: {
        let __arc3 = outSystem.clone();
        let BackendDAE::EQSYSTEM {
            orderedVars: __pa1,
            orderedEqs: __pa2,
            ..
        } = &*__arc3;
        vars = metamodelica::Own::own(__pa1);
        eqns = metamodelica::Own::own(__pa2);
        (
            repl,
            globalFoundSimple,
            unReplaceable,
            maxTraversals,
            warnAliasConflicts,
        ) = inTpl.clone();
        eqnslst = unwrap_break_err!(BackendEquation::equationList(eqns.clone()), '__try0);
        mT = arrayCreate(BackendVariable::varsSize(&vars), metamodelica::nil());
        (_, _, eqnslst, simpleeqnslst, _, _, foundSimple) = unwrap_break_err!(List::fold(&eqnslst, &fnptr!(simpleEquationsFinder, metamodelica::Ref<BackendDAE::Equation>, (BackendDAE::Variables, metamodelica::Ref<BackendDAE::Shared>, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<SimpleContainer>, i32, metamodelica::Array<metamodelica::List<i32>>, bool)), (vars.clone(), inShared.clone(), metamodelica::nil(), metamodelica::nil(), 1, mT.clone(), false)), '__try0);
        (
            vars,
            outShared,
            repl,
            unReplaceable,
            eqnslst,
            globalFoundSimple,
            warnAliasConflicts,
        ) = unwrap_break_err!(causalFinder(foundSimple, simpleeqnslst.clone(), eqnslst.clone(), 1, maxTraversals, vars.clone(), inShared.clone(), repl.clone(), unReplaceable.clone(), mT.clone(), metamodelica::nil(), globalFoundSimple, warnAliasConflicts), '__try0);
        outSystem = unwrap_break_err!(updateSystem(globalFoundSimple, eqnslst.clone(), vars.clone(), repl.clone(), outSystem.clone()), '__try0);
        outTpl = (
            repl.clone(),
            globalFoundSimple,
            unReplaceable.clone(),
            maxTraversals,
            warnAliasConflicts,
        );
        GCExt::free(mT.clone());
        Ok::<_, &'static str>((outShared.clone(), outSystem.clone(), outTpl.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            outShared = __try0_o0;
            outSystem = __try0_o1;
            outTpl = __try0_o2;
        }
        Err(_) => {
            outSystem = inSystem.clone();
            outShared = inShared.clone();
            outTpl = inTpl.clone();
        }
    }
    (outSystem, outShared, outTpl)
}

fn causalFinder(
    mut foundSimple: bool,
    mut simpleContainerIn: metamodelica::List<SimpleContainer>,
    mut iEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut traversalIdx: i32,
    mut maxTraversals: i32,
    mut iVars: BackendDAE::Variables,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut iRepl: BackendVarTransform::VariableReplacements,
    mut iUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut iMT: metamodelica::Array<metamodelica::List<i32>>,
    mut iGlobalEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inGlobalFoundSimple: bool,
    mut warnAliasConflicts: bool,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::Ref<BackendDAE::Shared>,
    BackendVarTransform::VariableReplacements,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    bool,
    bool,
)> {
    let mut outVars: BackendDAE::Variables = iVars.clone();
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = ishared.clone();
    let mut outRepl: BackendVarTransform::VariableReplacements = iRepl.clone();
    let mut outUnReplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ) = iUnreplaceable.clone();
    let mut outEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outGlobalFoundSimple: bool = inGlobalFoundSimple;
    let mut warnAliasConflicts: bool = warnAliasConflicts;
    let mut vars: BackendDAE::Variables;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut b: bool;
    let mut b1: bool;
    let mut simpleContainer: metamodelica::Array<SimpleContainer>;
    let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    if foundSimple {
        simpleContainer = List::listArrayReverse(simpleContainerIn)?;
        (vars, eqnslst, shared, repl, b) = handleSets(
            metamodelica::arrayLength(simpleContainer.clone()),
            1,
            simpleContainer.clone(),
            iMT.clone(),
            &iUnreplaceable,
            iVars,
            iEqnslst,
            ishared,
            iRepl,
        )?;
        warnAliasConflicts = warnAliasConflicts || b;
        (eqnslst, b1) = BackendVarTransform::replaceEquations(
            eqnslst,
            &repl,
            Some(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BackendVarTransform::skipPreChangeEdgeOperator(&__a0))
                })
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>),
            ),
        )?;
        (
            outVars,
            outShared,
            outRepl,
            outUnReplaceable,
            outEqnslst,
            warnAliasConflicts,
        ) = causalFinder1(
            intGt(traversalIdx, maxTraversals),
            b1,
            eqnslst,
            traversalIdx + 1,
            maxTraversals,
            vars,
            shared,
            repl,
            iUnreplaceable,
            iMT.clone(),
            iGlobalEqnslst,
            inGlobalFoundSimple,
            warnAliasConflicts,
        )?;
        outGlobalFoundSimple = true;
    } else {
        outEqnslst = listAppend(iEqnslst, iGlobalEqnslst);
    }
    Ok((
        outVars,
        outShared,
        outRepl,
        outUnReplaceable,
        outEqnslst,
        outGlobalFoundSimple,
        warnAliasConflicts,
    ))
}

fn causalFinder1(
    mut finished: bool,
    mut b: bool,
    mut iEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut index: i32,
    mut maxTraversals: i32,
    mut iVars: BackendDAE::Variables,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut iRepl: BackendVarTransform::VariableReplacements,
    mut iUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut iMT: metamodelica::Array<metamodelica::List<i32>>,
    mut iGlobalEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inGlobalFoundSimple: bool,
    mut warnAliasConflicts: bool,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::Ref<BackendDAE::Shared>,
    BackendVarTransform::VariableReplacements,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    bool,
)> {
    let mut outVars: BackendDAE::Variables = iVars.clone();
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = ishared.clone();
    let mut outRepl: BackendVarTransform::VariableReplacements = iRepl.clone();
    let mut outUnReplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ) = iUnreplaceable.clone();
    let mut outEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> =
        listAppend(iEqnslst.clone(), iGlobalEqnslst.clone());
    let mut warnAliasConflicts: bool = warnAliasConflicts;
    (outVars, outShared, outRepl, outUnReplaceable, outEqnslst) = (::match_deref::match_deref! { match &((finished, b, iEqnslst.clone())) {
        (true, _, _) => {
            (iVars, ishared, iRepl, iUnreplaceable, listAppend(iEqnslst, iGlobalEqnslst))
        },
        (_, false, Deref @ metamodelica::ListNode::Nil) => {
            (iVars, ishared, iRepl, iUnreplaceable, iGlobalEqnslst)
        },
        (_, false, _) => {
            (iVars, ishared, iRepl, iUnreplaceable, listAppend(iEqnslst, iGlobalEqnslst))
        },
        (_, true, _) => {
            let mut b1: bool;
            let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut simpleeqnslst: metamodelica::List<SimpleContainer>;
            let mut vars: BackendDAE::Variables;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            (vars, shared, eqnslst, simpleeqnslst, _, _, b1) = List::fold(&iEqnslst, &fnptr!(simpleEquationsFinder, metamodelica::Ref<BackendDAE::Equation>, (BackendDAE::Variables, metamodelica::Ref<BackendDAE::Shared>, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<SimpleContainer>, i32, metamodelica::Array<metamodelica::List<i32>>, bool)), (iVars, ishared, metamodelica::nil(), metamodelica::nil(), 1, iMT.clone(), false))?;
            (outVars, outShared, outRepl, outUnReplaceable, outEqnslst, _, warnAliasConflicts) = causalFinder(b1, simpleeqnslst, eqnslst, index, maxTraversals, vars, shared, iRepl, iUnreplaceable, iMT.clone(), iGlobalEqnslst, inGlobalFoundSimple, warnAliasConflicts)?;
            (outVars, outShared, outRepl, outUnReplaceable, outEqnslst)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((
        outVars,
        outShared,
        outRepl,
        outUnReplaceable,
        outEqnslst,
        warnAliasConflicts,
    ))
}

// =============================================================================
// section for allAcausal
//
// =============================================================================
pub(crate) fn allAcausal(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut b: bool;
    let mut warnAliasConflicts: bool;
    let mut size: i32;
    let mut unReplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    size = BackendDAEUtil::daeSize(inDAE)?;
    size = intMax(
        BaseHashTable::defaultBucketSize.clone(),
        (((intReal(size)) * (metamodelica::OrderedFloat(0.7_f64))).0.floor() as i32),
    );
    repl = BackendVarTransform::emptyReplacementsSized(size);
    unReplaceable = HashSet::emptyHashSet();
    unReplaceable = BackendDAEUtil::foldEqSystem(
        inDAE,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: metamodelica::Ref<BackendDAE::Shared>,
               __a2: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        )| addUnreplaceable(&__a0, &__a1, __a2),
        unReplaceable,
    )?;
    (_, unReplaceable) = BackendDAEUtil::traverseBackendDAEExps(
        inDAE,
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                traverserExpUnreplaceable,
                metamodelica::Ref<DAE::Exp>,
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>
                    ),
                    i32,
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
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>
                    )
                )
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        )> + 'static,
                >),
            unReplaceable,
        ),
    )?;
    unReplaceable = addUnreplaceableFromWhens(inDAE, unReplaceable)?;
    if Flags::isSet(Flags::DUMP_REPL.clone())? {
        BackendDump::dumpHashSet(&unReplaceable, &(literal!("Unreplaceable Crefs:")))?;
    }
    let (__pa0, (__pa1, _, __pa2, __pa3)) =
        BackendDAEUtil::mapEqSystemAndFold(inDAE, &allAcausal1, (repl, unReplaceable, false, false))?;
    outDAE = metamodelica::Own::own(__pa0);
    repl = metamodelica::Own::own(__pa1);
    b = metamodelica::Own::own(__pa2);
    warnAliasConflicts = metamodelica::Own::own(__pa3);
    if warnAliasConflicts && BackendDAEUtil::isSimulationDAE(&inDAE.shared) {
        Error::addMessage(Error::REDUNDANT_ALIAS_SET.clone(), metamodelica::nil())?;
    }
    outDAE = removeSimpleEquationsShared(b, outDAE, repl)?;
    Ok(outDAE)
}

fn allAcausal1(
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inTpl: (
        BackendVarTransform::VariableReplacements,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
        bool,
        bool,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendVarTransform::VariableReplacements,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
        bool,
        bool,
    ),
)> {
    let mut outSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outTpl: (
        BackendVarTransform::VariableReplacements,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
        ),
        bool,
        bool,
    );
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut unReplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    let mut b: bool;
    let mut b1: bool;
    let mut warnAliasConflicts: bool;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    if BackendDAEUtil::isClockedSyst(&inSystem) {
        outSystem = inSystem;
        outShared = inShared;
        outTpl = inTpl;
        return Ok((outSystem, outShared, outTpl));
    }
    let __arc2 = inSystem.clone();
    let BackendDAE::EQSYSTEM {
        orderedVars: __pa0,
        orderedEqs: __pa1,
        ..
    } = &*__arc2;
    vars = metamodelica::Own::own(__pa0);
    eqns = metamodelica::Own::own(__pa1);
    (repl, unReplaceable, b1, warnAliasConflicts) = inTpl;
    eqnslst = BackendEquation::equationList(eqns)?;
    mT = arrayCreate(BackendVariable::varsSize(&vars), metamodelica::nil());
    (vars, outShared, repl, unReplaceable, _, eqnslst, b, warnAliasConflicts) = allCausalFinder(
        eqnslst,
        &((
            vars,
            inShared,
            repl,
            unReplaceable,
            mT.clone(),
            metamodelica::nil(),
            false,
            warnAliasConflicts,
        )),
    )?;
    outSystem = updateSystem(b, eqnslst, vars, repl.clone(), inSystem)?;
    outTpl = (repl, unReplaceable, b || b1, warnAliasConflicts);
    Ok((outSystem, outShared, outTpl))
}

// =============================================================================
// section for causal
//
// =============================================================================
pub(crate) fn causal(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut b: bool;
    let mut warnAliasConflicts: bool;
    let mut size: i32;
    let mut unReplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    size = BackendDAEUtil::daeSize(inDAE)?;
    size = intMax(
        BaseHashTable::defaultBucketSize.clone(),
        (((intReal(size)) * (metamodelica::OrderedFloat(0.7_f64))).0.floor() as i32),
    );
    repl = BackendVarTransform::emptyReplacementsSized(size);
    unReplaceable = HashSet::emptyHashSet();
    unReplaceable = BackendDAEUtil::foldEqSystem(
        inDAE,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: metamodelica::Ref<BackendDAE::Shared>,
               __a2: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        )| addUnreplaceable(&__a0, &__a1, __a2),
        unReplaceable,
    )?;
    (_, unReplaceable) = BackendDAEUtil::traverseBackendDAEExps(
        inDAE,
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                traverserExpUnreplaceable,
                metamodelica::Ref<DAE::Exp>,
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>
                    ),
                    i32,
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
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>
                    )
                )
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        )> + 'static,
                >),
            unReplaceable,
        ),
    )?;
    unReplaceable = addUnreplaceableFromWhens(inDAE, unReplaceable)?;
    unReplaceable = addUnreplaceableFromStateSets(inDAE, unReplaceable)?;
    if Flags::isSet(Flags::DUMP_REPL.clone())? {
        BackendDump::dumpHashSet(&unReplaceable, &(literal!("Unreplaceable Crefs:")))?;
    }
    let (__pa0, (__pa1, _, __pa2, __pa3)) =
        BackendDAEUtil::mapEqSystemAndFold(inDAE, &causal1, (repl, unReplaceable, false, false))?;
    outDAE = metamodelica::Own::own(__pa0);
    repl = metamodelica::Own::own(__pa1);
    b = metamodelica::Own::own(__pa2);
    warnAliasConflicts = metamodelica::Own::own(__pa3);
    if warnAliasConflicts && BackendDAEUtil::isSimulationDAE(&inDAE.shared) {
        Error::addMessage(Error::REDUNDANT_ALIAS_SET.clone(), metamodelica::nil())?;
    }
    outDAE = removeSimpleEquationsShared(b, outDAE, repl)?;
    Ok(outDAE)
}

fn causal1(
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inTpl: (
        BackendVarTransform::VariableReplacements,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
        bool,
        bool,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendVarTransform::VariableReplacements,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
        bool,
        bool,
    ),
)> {
    let mut outSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outTpl: (
        BackendVarTransform::VariableReplacements,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
        ),
        bool,
        bool,
    );
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut unReplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    let mut b: bool;
    let mut b1: bool;
    let mut warnAliasConflicts: bool;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    if BackendDAEUtil::isClockedSyst(&inSystem) {
        outSystem = inSystem;
        outShared = inShared;
        outTpl = inTpl;
        return Ok((outSystem, outShared, outTpl));
    }
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inSystem.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa2, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    vars = metamodelica::Own::own(__pa0);
    eqns = metamodelica::Own::own(__pa1);
    comps = metamodelica::Own::own(__pa2);
    (repl, unReplaceable, b1, warnAliasConflicts) = inTpl;
    mT = arrayCreate(BackendVariable::varsSize(&vars), metamodelica::nil());
    (vars, outShared, repl, unReplaceable, _, eqnslst, b, warnAliasConflicts) = traverseComponents(
        &comps,
        eqns,
        &move |__a0: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
               __a1: (
            BackendDAE::Variables,
            metamodelica::Ref<BackendDAE::Shared>,
            BackendVarTransform::VariableReplacements,
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                ),
                i32,
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
                ),
            ),
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            bool,
            bool,
        )| allCausalFinder(__a0, &__a1),
        (
            vars,
            inShared,
            repl,
            unReplaceable,
            mT.clone(),
            metamodelica::nil(),
            false,
            warnAliasConflicts,
        ),
    )?;
    outSystem = updateSystem(b, eqnslst, vars, repl.clone(), inSystem)?;
    outTpl = (repl, unReplaceable, b || b1, warnAliasConflicts);
    Ok((outSystem, outShared, outTpl))
}

fn traverseComponents<'__b, Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inComps: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut iEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inFunc: &'__b dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, Type_a) -> Result<Type_a>,
    mut inTypeA: Type_a,
) -> Result<Type_a> {
    pub type FuncType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, Type_a) -> Result<Type_a>
            + 'static,
    >;

    '__tco: loop {
        ::match_deref::match_deref! { match inComps {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inTypeA)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: e, .. }, tail: rest } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut arg: Type_a;
                eqn = BackendEquation::get(iEqns.clone(), e.clone())?;
                arg = inFunc(list![eqn], inTypeA)?;
                { (inComps, iEqns, inFunc, inTypeA) = (rest, iEqns, inFunc, arg); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: elst, .. }, tail: rest } => {
                let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut arg: Type_a;
                eqnlst = BackendEquation::getList(elst.clone(), iEqns.clone())?;
                arg = inFunc(eqnlst, inTypeA)?;
                { (inComps, iEqns, inFunc, inTypeA) = (rest, iEqns, inFunc, arg); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: e, .. }, tail: rest } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut arg: Type_a;
                eqn = BackendEquation::get(iEqns.clone(), e.clone())?;
                arg = inFunc(list![eqn], inTypeA)?;
                { (inComps, iEqns, inFunc, inTypeA) = (rest, iEqns, inFunc, arg); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: e, .. }, tail: rest } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut arg: Type_a;
                eqn = BackendEquation::get(iEqns.clone(), e.clone())?;
                arg = inFunc(list![eqn], inTypeA)?;
                { (inComps, iEqns, inFunc, inTypeA) = (rest, iEqns, inFunc, arg); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: e, .. }, tail: rest } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut arg: Type_a;
                eqn = BackendEquation::get(iEqns.clone(), e.clone())?;
                arg = inFunc(list![eqn], inTypeA)?;
                { (inComps, iEqns, inFunc, inTypeA) = (rest, iEqns, inFunc, arg); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: e, .. }, tail: rest } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut arg: Type_a;
                eqn = BackendEquation::get(iEqns.clone(), e.clone())?;
                arg = inFunc(list![eqn], inTypeA)?;
                { (inComps, iEqns, inFunc, inTypeA) = (rest, iEqns, inFunc, arg); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: e, .. }, tail: rest } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut arg: Type_a;
                eqn = BackendEquation::get(iEqns.clone(), e.clone())?;
                arg = inFunc(list![eqn], inTypeA)?;
                { (inComps, iEqns, inFunc, inTypeA) = (rest, iEqns, inFunc, arg); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: elst, innerEquations, .. }, .. }, tail: rest } => {
                let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqnlst1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut arg: Type_a;
                let mut elst = (*elst).clone();
                eqnlst = BackendEquation::getList(elst.clone(), iEqns.clone())?;
                (elst, _, _) = List::map_3(metamodelica::AsArg::as_arg(&innerEquations), &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0)) })?;
                eqnlst1 = BackendEquation::getList(elst.clone(), iEqns.clone())?;
                arg = inFunc(listAppend(eqnlst, eqnlst1), inTypeA)?;
                { (inComps, iEqns, inFunc, inTypeA) = (rest, iEqns, inFunc, arg); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn allCausalFinder(
    mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inTpl: &(
        BackendDAE::Variables,
        metamodelica::Ref<BackendDAE::Shared>,
        BackendVarTransform::VariableReplacements,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        bool,
        bool,
    ),
) -> Result<(
    BackendDAE::Variables,
    metamodelica::Ref<BackendDAE::Shared>,
    BackendVarTransform::VariableReplacements,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    bool,
    bool,
)> {
    let mut outTpl: (
        BackendDAE::Variables,
        metamodelica::Ref<BackendDAE::Shared>,
        BackendVarTransform::VariableReplacements,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
        ),
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        bool,
        bool,
    );
    let mut vars: BackendDAE::Variables;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut unReplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    let mut mt: metamodelica::Array<metamodelica::List<i32>>;
    let mut b: bool;
    let mut b1: bool;
    let mut b2: bool;
    let mut b3: bool;
    let mut globalFoundSimple: bool;
    let mut warnAliasConflicts: bool;
    let mut globaleqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut simpleeqnslst: metamodelica::List<SimpleContainer>;
    (
        vars,
        shared,
        repl,
        unReplaceable,
        mt,
        globaleqnslst,
        b,
        warnAliasConflicts,
    ) = inTpl.clone();
    (eqnslst, b2) = BackendVarTransform::replaceEquations(
        eqns,
        &repl,
        Some(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVarTransform::skipPreChangeEdgeOperator(&__a0))
            })
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>),
        ),
    )?;
    (_, _, eqnslst, simpleeqnslst, _, _, b1) = List::fold(
        &eqnslst,
        &fnptr!(
            simpleEquationsFinder,
            metamodelica::Ref<BackendDAE::Equation>,
            (
                BackendDAE::Variables,
                metamodelica::Ref<BackendDAE::Shared>,
                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                metamodelica::List<SimpleContainer>,
                i32,
                metamodelica::Array<metamodelica::List<i32>>,
                bool
            )
        ),
        (
            vars.clone(),
            shared.clone(),
            metamodelica::nil(),
            metamodelica::nil(),
            1,
            mt.clone(),
            false,
        ),
    )?;
    (vars, shared, repl, unReplaceable, eqnslst, globalFoundSimple, b3) = allCausalFinder1(
        b1,
        b2,
        simpleeqnslst,
        eqnslst,
        vars,
        shared,
        repl,
        unReplaceable,
        mt.clone(),
        globaleqnslst,
        b,
        warnAliasConflicts,
    )?;
    warnAliasConflicts = warnAliasConflicts || b3;
    outTpl = (
        vars,
        shared,
        repl,
        unReplaceable,
        mt.clone(),
        eqnslst,
        globalFoundSimple,
        warnAliasConflicts,
    );
    Ok(outTpl)
}

fn allCausalFinder1(
    mut foundSimple: bool,
    mut didReplacement: bool,
    mut iSimpleeqnslst: metamodelica::List<SimpleContainer>,
    mut iEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iVars: BackendDAE::Variables,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut iRepl: BackendVarTransform::VariableReplacements,
    mut iUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut iMT: metamodelica::Array<metamodelica::List<i32>>,
    mut iGlobalEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut globalFoundSimple: bool,
    mut warnAliasConflicts: bool,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::Ref<BackendDAE::Shared>,
    BackendVarTransform::VariableReplacements,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    bool,
    bool,
)> {
    let mut outVars: BackendDAE::Variables;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outRepl: BackendVarTransform::VariableReplacements;
    let mut outUnReplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    let mut outEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outGlobalFoundSimple: bool;
    let mut warnAliasConflicts: bool = warnAliasConflicts;
    (
        outVars,
        outShared,
        outRepl,
        outUnReplaceable,
        outEqnslst,
        outGlobalFoundSimple,
        warnAliasConflicts,
    ) = (::match_deref::match_deref! { match &((foundSimple, iEqnslst.clone())) {
        (false, Deref @ metamodelica::ListNode::Nil) => {
            (iVars, ishared, iRepl, iUnreplaceable, iGlobalEqnslst, didReplacement || globalFoundSimple, warnAliasConflicts)
        },
        (false, _) => {
            (iVars, ishared, iRepl, iUnreplaceable, listAppend(iEqnslst, iGlobalEqnslst), didReplacement || globalFoundSimple, warnAliasConflicts)
        },
        (true, _) => {
            let mut vars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut b: bool;
            let mut b1: bool;
            let mut simpleeqns: metamodelica::Array<SimpleContainer>;
            let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            simpleeqns = List::listArrayReverse(iSimpleeqnslst)?;
            (vars, eqnslst, shared, repl, b) = handleSets(metamodelica::arrayLength(simpleeqns.clone()), 1, simpleeqns.clone(), iMT.clone(), &iUnreplaceable, iVars, iEqnslst, ishared, iRepl)?;
            warnAliasConflicts = warnAliasConflicts || b;
            (eqnslst, b1) = BackendVarTransform::replaceEquations(eqnslst, &repl, Some((std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVarTransform::skipPreChangeEdgeOperator(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>)))?;
            allCausalFinder2(b1, eqnslst, vars, shared, repl, iUnreplaceable, iMT.clone(), iGlobalEqnslst, true, warnAliasConflicts)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((
        outVars,
        outShared,
        outRepl,
        outUnReplaceable,
        outEqnslst,
        outGlobalFoundSimple,
        warnAliasConflicts,
    ))
}

fn allCausalFinder2(
    mut b: bool,
    mut iEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iVars: BackendDAE::Variables,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut iRepl: BackendVarTransform::VariableReplacements,
    mut iUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut iMT: metamodelica::Array<metamodelica::List<i32>>,
    mut iGlobalEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut globalFoundSimple: bool,
    mut warnAliasConflicts: bool,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::Ref<BackendDAE::Shared>,
    BackendVarTransform::VariableReplacements,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    bool,
    bool,
)> {
    let mut outVars: BackendDAE::Variables;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outRepl: BackendVarTransform::VariableReplacements;
    let mut outUnReplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    let mut outEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outGlobalFoundSimple: bool;
    let mut warnAliasConflicts: bool = warnAliasConflicts;
    (
        outVars,
        outShared,
        outRepl,
        outUnReplaceable,
        outEqnslst,
        outGlobalFoundSimple,
        warnAliasConflicts,
    ) = (::match_deref::match_deref! { match &((b, iEqnslst.clone())) {
        (false, Deref @ metamodelica::ListNode::Nil) => {
            (iVars, ishared, iRepl, iUnreplaceable, iGlobalEqnslst, globalFoundSimple, warnAliasConflicts)
        },
        (false, _) => {
            (iVars, ishared, iRepl, iUnreplaceable, listAppend(iEqnslst, iGlobalEqnslst), globalFoundSimple, warnAliasConflicts)
        },
        (true, _) => {
            let mut b1: bool;
            let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut simpleeqnslst: metamodelica::List<SimpleContainer>;
            let mut vars: BackendDAE::Variables;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            (vars, shared, eqnslst, simpleeqnslst, _, _, b1) = List::fold(&iEqnslst, &fnptr!(simpleEquationsFinder, metamodelica::Ref<BackendDAE::Equation>, (BackendDAE::Variables, metamodelica::Ref<BackendDAE::Shared>, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<SimpleContainer>, i32, metamodelica::Array<metamodelica::List<i32>>, bool)), (iVars, ishared, metamodelica::nil(), metamodelica::nil(), 1, iMT.clone(), false))?;
            allCausalFinder1(b1, false, simpleeqnslst, eqnslst, vars, shared, iRepl, iUnreplaceable, iMT.clone(), iGlobalEqnslst, globalFoundSimple, warnAliasConflicts)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((
        outVars,
        outShared,
        outRepl,
        outUnReplaceable,
        outEqnslst,
        outGlobalFoundSimple,
        warnAliasConflicts,
    ))
}

// =============================================================================
// functions to find simple equations
//
// =============================================================================
fn simpleEquationsFinder(mut eqn: metamodelica::Ref<BackendDAE::Equation>, mut inTpl: AccTuple) -> AccTuple {
    let mut outTpl: AccTuple;
    outTpl = 'mc: {
        let __mc_input = (&*eqn, &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr: eqAttr }, _) => {
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrExpStrExpStr(&(literal!("Found Equation ")), e1.clone(), &(literal!(" = ")), e2.clone(), &(literal!(" to handle.\n")))?;
                    }
                    Ok(simpleEquationAcausal(e1.clone(), e2.clone(), (source.clone(), eqAttr.clone()), false, inTpl.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, source, attr: eqAttr, .. }, _) => {
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrExpStrExpStr(&(literal!("Found Array Equation ")), e1.clone(), &(literal!(" = ")), e2.clone(), &(literal!(" to handle.\n")))?;
                    }
                    Ok(simpleArrayEquationAcausal(e1.clone(), e2.clone(), &(Expression::r#typeof(e1.clone())?), (source.clone(), eqAttr.clone()), inTpl.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, source, attr: eqAttr }, _) => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    e1 = Expression::crefExp(cr.clone())?;
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrExpStrExpStr(&(literal!("Found Solved Equation ")), e1.clone(), &(literal!(" = ")), e2.clone(), &(literal!(" to handle.\n")))?;
                    }
                    Ok(simpleEquationAcausal(e1.clone(), e2.clone(), (source.clone(), eqAttr.clone()), false, inTpl.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, source, attr: eqAttr }, _) => {
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrExpStr(&(literal!("Found Residual Equation ")), e1.clone(), &(literal!(" to handle.\n")))?;
                    }
                    Ok(simpleExpressionAcausal(e1.clone(), (source.clone(), eqAttr.clone()), false, inTpl.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, source, attr: eqAttr, .. }, _) => {
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrExpStrExpStr(&(literal!("Found Complex Equation ")), e1.clone(), &(literal!(" = ")), e2.clone(), &(literal!(" to handle.\n")))?;
                    }
                    Ok(simpleEquationAcausal(e1.clone(), e2.clone(), (source.clone(), eqAttr.clone()), false, inTpl.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (v, s, eqns, seqns, index, mT, b)) => {
                    Ok((v.clone(), s.clone(), metamodelica::cons(eqn.clone(), eqns.clone()), seqns.clone(), index.clone(), mT.clone(), b.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outTpl
}

fn simpleEquationAcausal(
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut eqnAttributes: EquationSourceAndAttributes,
    mut selfCalled: bool,
    mut inTpl: AccTuple,
) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
            addSimpleEquationAcausal(cr1.clone(), lhs, false, cr2.clone(), rhs, false, &eqnAttributes, selfCalled, &inTpl)?
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            addSimpleEquationAcausal(cr1.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: lhs }), false, cr2.clone(), rhs, true, &eqnAttributes, selfCalled, &inTpl)?
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            addSimpleEquationAcausal(cr1.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: lhs }), false, cr2.clone(), rhs, true, &eqnAttributes, selfCalled, &inTpl)?
        },
        (Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
            addSimpleEquationAcausal(cr1.clone(), lhs, true, cr2.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: rhs }), false, &eqnAttributes, selfCalled, &inTpl)?
        },
        (Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
            addSimpleEquationAcausal(cr1.clone(), lhs, true, cr2.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: rhs }), false, &eqnAttributes, selfCalled, &inTpl)?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), false, cr2.clone(), e2.clone(), false, &eqnAttributes, selfCalled, &inTpl)?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), false, cr2.clone(), e2.clone(), false, &eqnAttributes, selfCalled, &inTpl)?
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::LUNARY { operator: op @ DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            addSimpleEquationAcausal(cr1.clone(), metamodelica::Ref::new(DAE::Exp::LUNARY { operator: op.clone(), exp: lhs }), false, cr2.clone(), rhs, true, &eqnAttributes, selfCalled, &inTpl)?
        },
        (Deref @ DAE::Exp::LUNARY { operator: op @ DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
            addSimpleEquationAcausal(cr1.clone(), lhs.clone(), true, cr2.clone(), metamodelica::Ref::new(DAE::Exp::LUNARY { operator: op.clone(), exp: lhs }), false, &eqnAttributes, selfCalled, &inTpl)?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), false, cr2.clone(), e2.clone(), false, &eqnAttributes, selfCalled, &inTpl)?
        },
        (Deref @ DAE::Exp::ARRAY { array: elst1, .. }, Deref @ DAE::Exp::ARRAY { array: elst2, .. }) => {
            List::threadFold2(metamodelica::AsArg::as_arg(&elst1), elst2.clone(), &simpleEquationAcausal, eqnAttributes, true, inTpl)?
        },
        (Deref @ DAE::Exp::MATRIX { matrix: elstlst1, .. }, Deref @ DAE::Exp::MATRIX { matrix: elstlst2, .. }) => {
            List::threadFold2(metamodelica::AsArg::as_arg(&elstlst1), elstlst2.clone(), &move |__a0: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a1: metamodelica::List<metamodelica::Ref<DAE::Exp>>, __a2: (metamodelica::Ref<DAE::ElementSource>, BackendDAE::EquationAttributes), __a3: bool, __a4: (BackendDAE::Variables, metamodelica::Ref<BackendDAE::Shared>, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<SimpleContainer>, i32, metamodelica::Array<metamodelica::List<i32>>, bool)| simpleEquationAcausalLst(&__a0, __a1, __a2, __a3, __a4), eqnAttributes, true, inTpl)?
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::ARRAY { ty, .. }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::MATRIX { ty, .. }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::ARRAY { ty, .. }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::MATRIX { ty, .. }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::ARRAY { ty, .. } }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::MATRIX { ty, .. } }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e2 @ Deref @ DAE::Exp::ARRAY { ty, .. } }) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e2 @ Deref @ DAE::Exp::MATRIX { ty, .. } }) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::ARRAY { ty, .. }, Deref @ DAE::Exp::CREF { .. }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::MATRIX { ty, .. }, Deref @ DAE::Exp::CREF { .. }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::ARRAY { ty, .. } }, Deref @ DAE::Exp::CREF { .. }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::MATRIX { ty, .. } }, Deref @ DAE::Exp::CREF { .. }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::ARRAY { ty, .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::MATRIX { ty, .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::ARRAY { ty, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { .. } }) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::MATRIX { ty, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { .. } }) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::ARRAY { ty, .. }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::MATRIX { ty, .. }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::ARRAY { ty, .. } }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::MATRIX { ty, .. } }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 @ Deref @ DAE::Exp::ARRAY { ty, .. } }) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 @ Deref @ DAE::Exp::MATRIX { ty, .. } }) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::ARRAY { ty, .. }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::MATRIX { ty, .. }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::ARRAY { ty, .. } }, Deref @ DAE::Exp::CREF { .. }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::MATRIX { ty, .. } }, Deref @ DAE::Exp::CREF { .. }) => {
            simpleArrayEquationAcausal(lhs, rhs, metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 @ Deref @ DAE::Exp::ARRAY { ty, .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { .. } }) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        (Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 @ Deref @ DAE::Exp::MATRIX { ty, .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { .. } }) => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        _ => {
            simpleEquationAcausal1(lhs, rhs, eqnAttributes, selfCalled, inTpl)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

fn simpleArrayEquationAcausal(
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut ty: &metamodelica::Ref<DAE::Type>,
    mut eqnAttributes: EquationSourceAndAttributes,
    mut inTpl: AccTuple,
) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut ds: metamodelica::List<i32>;
    let mut subslst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
    let mut elst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut elst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut hasInlineAfterIndexReduction: bool;
    let mut expandLhs: bool;
    let mut expandRhs: bool;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut attr: BackendDAE::EquationAttributes;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    dims = Expression::arrayDimension(ty);
    ds = Expression::dimensionsSizes(dims)?;
    subslst = List::map(ds, &fnptr!(Expression::dimensionSizeSubscripts, i32))?;
    subslst = Expression::rangesToSubscripts(&subslst)?;
    if (subslst).is_empty() {
        (source, attr) = eqnAttributes;
        outTpl = inTpl;
        return Ok(outTpl);
        for mut e in &*list![lhs, rhs] {
            if Expression::isEvaluatedConst(metamodelica::AsArg::as_arg(&e)) {
                continue;
            }
            eq = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION {
                size: 0,
                whenEquation: metamodelica::Ref::new(BackendDAE::WhenEquation {
                    condition: metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
                    whenStmtLst: list![BackendDAE::WhenOperator::ASSERT {
                        condition: metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
                        message: metamodelica::Ref::new(DAE::Exp::SCONST {
                            string: literal!("Failed assertion exp is 0")
                        }),
                        level: DAE::ASSERTIONLEVEL_ERROR().clone(),
                        source: source.clone()
                    }],
                    elsewhenPart: None,
                }),
                source: source.clone(),
                attr: attr,
            });
            outTpl = simpleEquationsFinder(eq, outTpl);
        }
        return Ok(outTpl);
    }
    (_, hasInlineAfterIndexReduction) = Expression::traverseExpTopDown(
        lhs.clone(),
        &fnptr!(
            Expression::findCallIsInlineAfterIndexReduction,
            metamodelica::Ref<DAE::Exp>,
            bool
        ),
        false,
    )?;
    (_, hasInlineAfterIndexReduction) = Expression::traverseExpTopDown(
        rhs.clone(),
        &fnptr!(
            Expression::findCallIsInlineAfterIndexReduction,
            metamodelica::Ref<DAE::Exp>,
            bool
        ),
        hasInlineAfterIndexReduction,
    )?;
    (elst1, expandLhs) = List::mapFold(
        &subslst,
        &({
            let __pe_b0 = lhs.clone();
            move |__pe_a1, __pe_a2| {
                Ok(Expression::applyExpSubscriptsFoldCheckSimplify(
                    __pe_b0.clone(),
                    &__pe_a1,
                    __pe_a2,
                ))
            }
        }),
        false,
    )?;
    (elst2, expandRhs) = List::mapFold(
        &subslst,
        &({
            let __pe_b0 = rhs.clone();
            move |__pe_a1, __pe_a2| {
                Ok(Expression::applyExpSubscriptsFoldCheckSimplify(
                    __pe_b0.clone(),
                    &__pe_a1,
                    __pe_a2,
                ))
            }
        }),
        false,
    )?;
    if !(hasInlineAfterIndexReduction) {
        if false && !(expandLhs && expandRhs) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("RemoveSimpleEquations.simpleArrayEquationAcausal"));
                __mm_s.push_str(&*literal!(" not expanding "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(lhs)?);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        let true = (expandLhs && expandRhs) else {
            return Err("pattern mismatch");
        };
    }
    outTpl = List::threadFold2(&elst1, elst2, &simpleEquationAcausal, eqnAttributes, true, inTpl)?;
    Ok(outTpl)
}

fn simpleEquationAcausalLst(
    mut elst1: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut elst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut eqnAttributes: EquationSourceAndAttributes,
    mut selfCalled: bool,
    mut inTpl: AccTuple,
) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = List::threadFold2(elst1, elst2, &simpleEquationAcausal, eqnAttributes, selfCalled, inTpl)?;
    Ok(outTpl)
}

fn simpleEquationAcausal1(
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut eqnAttributes: EquationSourceAndAttributes,
    mut selfCalled: bool,
    mut inTpl: AccTuple,
) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = 'mc: {
        let __mc_input = (&*lhs, &*rhs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut elst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut elst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    elst1 = Expression::splitRecord(&(lhs.clone()), &(Expression::r#typeof(lhs.clone())?))?;
                    elst2 = Expression::splitRecord(&(rhs.clone()), &(Expression::r#typeof(rhs.clone())?))?;
                    Ok(List::threadFold2(&elst1, elst2.clone(), &simpleEquationAcausal, eqnAttributes.clone(), true, inTpl.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: elst1, .. }, _) => {
                    if !((Expression::isZero(&rhs)?)) { return Err("guard") }
                    Ok(List::fold2(metamodelica::AsArg::as_arg(&elst1), &simpleExpressionAcausal, eqnAttributes.clone(), true, inTpl.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::ARRAY { array: elst2, .. }) => {
                    if !((Expression::isZero(&lhs)?)) { return Err("guard") }
                    Ok(List::fold2(metamodelica::AsArg::as_arg(&elst2), &simpleExpressionAcausal, eqnAttributes.clone(), true, inTpl.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    if !((Expression::isZero(&rhs)?)) { return Err("guard") }
                    Ok(simpleExpressionAcausal(lhs.clone(), eqnAttributes.clone(), selfCalled, inTpl.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    if !((Expression::isZero(&lhs)?)) { return Err("guard") }
                    Ok(simpleExpressionAcausal(rhs.clone(), eqnAttributes.clone(), selfCalled, inTpl.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(timeIndependentEquationAcausal(lhs.clone(), rhs.clone(), eqnAttributes.clone(), selfCalled, inTpl.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTpl)
}

fn generateEquation(
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut ty: &metamodelica::Ref<DAE::Type>,
    mut eqnAttributes: &EquationSourceAndAttributes,
    mut inTpl: &AccTuple,
) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = 'mc: {
        let __mc_input = (eqnAttributes, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((source, eqAttr), (v, s, eqns, seqns, index, mT, b)) => {
                    if !((DAEUtil::expTypeComplex(ty))) { return Err("guard") }
                    let mut size: i32;
                    size = Expression::sizeOf(ty);
                    Ok((v.clone(), s.clone(), metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size, left: lhs.clone(), right: rhs.clone(), source: source.clone(), attr: eqAttr.clone() }), eqns.clone()), seqns.clone(), index.clone(), mT.clone(), b.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((source, eqAttr), (v, s, eqns, seqns, index, mT, b)) => {
                    if !((DAEUtil::expTypeArray(ty))) { return Err("guard") }
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut ds: metamodelica::List<i32>;
                    let mut recordSize: Option<i32>;
                    dims = Expression::arrayDimension(ty);
                    ds = Expression::dimensionsSizes(dims.clone())?;
                    tp = DAEUtil::expTypeElementType(ty);
                    if DAEUtil::expTypeComplex(&tp) {
                        recordSize = Some(Expression::sizeOf(&tp));
                    } else {
                        recordSize = None;
                    }
                    Ok((v.clone(), s.clone(), metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: ds.clone(), left: lhs.clone(), right: rhs.clone(), source: source.clone(), attr: eqAttr.clone(), recordSize: recordSize.clone() }), eqns.clone()), seqns.clone(), index.clone(), mT.clone(), b.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((source, eqAttr), (v, s, eqns, seqns, index, mT, b)) => {
                    let mut b1: bool;
                    let mut b2: bool;
                    b1 = DAEUtil::expTypeComplex(ty);
                    b2 = DAEUtil::expTypeArray(ty);
                    let false = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((v.clone(), s.clone(), metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: source.clone(), attr: eqAttr.clone() }), eqns.clone()), seqns.clone(), index.clone(), mT.clone(), b.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAEOptimize.generateEquation failed on: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(lhs.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTpl)
}

fn simpleExpressionAcausal(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut eqnAttributes: EquationSourceAndAttributes,
    mut selfCalled: bool,
    mut inTpl: AccTuple,
) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::ADD { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            addSimpleEquationAcausal(cr1.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e1.clone() }), false, cr2.clone(), e2.clone(), true, &eqnAttributes, selfCalled, &inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::ADD_ARR { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            addSimpleEquationAcausal(cr1.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: e1.clone() }), false, cr2.clone(), e2.clone(), true, &eqnAttributes, selfCalled, &inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::SUB { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), false, cr2.clone(), e2.clone(), false, &eqnAttributes, selfCalled, &inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, operator: DAE::Operator::SUB_ARR { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), false, cr2.clone(), e2.clone(), false, &eqnAttributes, selfCalled, &inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::ADD { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), false, cr2.clone(), e2.clone(), false, &eqnAttributes, selfCalled, &inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::ADD_ARR { .. }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), false, cr2.clone(), e2.clone(), false, &eqnAttributes, selfCalled, &inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::SUB { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), true, cr2.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e2.clone() }), false, &eqnAttributes, selfCalled, &inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, operator: DAE::Operator::SUB_ARR { ty }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } } => {
            addSimpleEquationAcausal(cr1.clone(), e1.clone(), true, cr2.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: e2.clone() }), false, &eqnAttributes, selfCalled, &inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD_ARR { ty: tp }, exp2: e2 @ Deref @ DAE::Exp::ARRAY { ty, .. } } => {
            simpleArrayEquationAcausal(e1.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp.clone() }, exp: e2.clone() }), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD_ARR { ty: tp }, exp2: e2 @ Deref @ DAE::Exp::MATRIX { ty, .. } } => {
            simpleArrayEquationAcausal(e1.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp.clone() }, exp: e2.clone() }), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB_ARR { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::ARRAY { ty, .. } } => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB_ARR { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::MATRIX { ty, .. } } => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD_ARR { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::ARRAY { ty, .. } } => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD_ARR { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::MATRIX { ty, .. } } => {
            simpleArrayEquationAcausal(e1.clone(), e2.clone(), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB_ARR { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::ARRAY { ty, .. } } => {
            simpleArrayEquationAcausal(e1.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: e2.clone() }), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB_ARR { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::MATRIX { ty, .. } } => {
            simpleArrayEquationAcausal(e1.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: e2.clone() }), metamodelica::AsArg::as_arg(&ty), eqnAttributes, inTpl)?
        },
        _ => {
            timeIndependentExpressionAcausal(exp, eqnAttributes, selfCalled, inTpl)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

fn addSimpleEquationAcausal(
    mut cr1: metamodelica::Ref<DAE::ComponentRef>,
    mut inE1: metamodelica::Ref<DAE::Exp>,
    mut negatedCr1: bool,
    mut cr2: metamodelica::Ref<DAE::ComponentRef>,
    mut inE2: metamodelica::Ref<DAE::Exp>,
    mut negatedCr2: bool,
    mut eqnAttributes: &EquationSourceAndAttributes,
    mut genEqn: bool,
    mut inTpl: &AccTuple,
) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = 'mc: {
        let __mc_input = (genEqn, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (vars, shared, eqns, seqns, index, mT, _)) => {
                    let mut vars1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut vars2: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut ilst1: metamodelica::List<i32>;
                    let mut ilst2: metamodelica::List<i32>;
                    let mut varskn1: bool;
                    let mut varskn2: bool;
                    let mut time1: bool;
                    let mut time2: bool;
                    let mut seqns = (*seqns).clone();
                    let mut index = (*index).clone();
                    let mut mT = (*mT).clone();
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrCrefStrCrefStr(&(literal!("Alias Equation ")), &cr1, &(literal!(" = ")), &cr2, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" found. Negated lhs[")); __mm_s.push_str(&*boolString(negatedCr1)); __mm_s.push_str(&*literal!("] = rhs[")); __mm_s.push_str(&*boolString(negatedCr2)); __mm_s.push_str(&*literal!("].\n")); ArcStr::from(__mm_s) }))?;
                    }
                    (vars1, ilst1, varskn1, time1) = getVars(cr1.clone(), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&shared))?;
                    (vars2, ilst2, varskn2, time2) = getVars(cr2.clone(), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&shared))?;
                    let true = (intEq(((vars1).len() as i32), ((vars2).len() as i32))) else { return Err("pattern mismatch") };
                    (seqns, index, mT) = generateSimpleContainters(vars1.clone(), negatedCr1, ilst1.clone(), varskn1, time1, vars2.clone(), negatedCr2, ilst2.clone(), varskn2, time2, eqnAttributes, seqns.clone(), index.clone(), mT.clone())?;
                    Ok((vars.clone(), shared.clone(), eqns.clone(), seqns.clone(), index.clone(), mT.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _) => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrExpStrExpStr(&(literal!("Non Alias Equation ")), inE1.clone(), &(literal!(" = ")), inE2.clone(), &(literal!(" to generate.\n")))?;
                    }
                    e1 = Expression::crefExp(cr1.clone())?;
                    ty = Expression::r#typeof(e1.clone())?;
                    e2 = inE2.clone();
                    Ok(generateEquation(e1.clone(), e2.clone(), &ty, eqnAttributes, inTpl)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTpl)
}

fn getVars(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut vars: &BackendDAE::Variables,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<i32>,
    bool,
    bool,
)> {
    let mut oVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut oIndexs: metamodelica::List<i32> = metamodelica::nil();
    let mut varskn: bool;
    let mut time_: bool;
    (oVars, oIndexs, varskn, time_) = 'mc: {
        let __mc_input = &*cr;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", subscriptLst: Deref @ metamodelica::ListNode::Nil, .. } => {
                    Ok((metamodelica::nil(), metamodelica::nil(), true, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut oIndexs: metamodelica::List<i32> = oIndexs.clone();
                    let mut oVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = oVars.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars)?) {
                        (__pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    oVars = metamodelica::Own::own(__pa0);
                    oIndexs = metamodelica::Own::own(__pa1);
                    Ok(((oVars.clone(), oIndexs.clone(), false, false), oIndexs.clone(), oVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oIndexs = __wb0;
            oVars = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut oIndexs: metamodelica::List<i32> = oIndexs.clone();
                    let mut oVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = oVars.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendVariable::getVarShared(cr.clone(), shared)?) {
                        (__pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    oVars = metamodelica::Own::own(__pa0);
                    oIndexs = metamodelica::Own::own(__pa1);
                    if ComponentReference::crefIsScalarWithVariableSubs(&cr) {
                        oVars = metamodelica::nil();
                        oIndexs = metamodelica::nil();
                    }
                    Ok(((oVars.clone(), oIndexs.clone(), true, false), oIndexs.clone(), oVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oIndexs = __wb0;
            oVars = __wb1;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((oVars, oIndexs, varskn, time_))
}

fn generateSimpleContainters<'__b>(
    mut vars1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut negatedCr1: bool,
    mut ilst1: metamodelica::List<i32>,
    mut varskn1: bool,
    mut time1: bool,
    mut vars2: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut negatedCr2: bool,
    mut ilst2: metamodelica::List<i32>,
    mut varskn2: bool,
    mut time2: bool,
    mut eqnAttributes: &'__b EquationSourceAndAttributes,
    mut iSeqns: metamodelica::List<SimpleContainer>,
    mut iIndex: i32,
    mut iMT: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<SimpleContainer>,
    i32,
    metamodelica::Array<metamodelica::List<i32>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((vars1, ilst1, varskn1, time1, vars2, ilst2, varskn2, time2)) {
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varName: cr1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: i1, tail: Deref @ metamodelica::ListNode::Nil }, true, true, Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varName: cr2, .. }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: i2, tail: Deref @ metamodelica::ListNode::Nil }, false, false) => {
                let mut colum: metamodelica::List<i32>;
                colum = ({let __elt = (*metamodelica::index_checked(&iMT.borrow(), i2.clone())?).clone(); __elt});
                metamodelica::arrayUpdate(iMT.clone(), i2.clone(), metamodelica::cons(iIndex, colum))?;
                return Ok((metamodelica::cons(SimpleContainer::TIMEALIAS { cr1: cr2.clone(), negatedCr1: negatedCr2, i1: i2.clone(), cr2: cr1.clone(), negatedCr2: negatedCr1, i2: i1.clone(), eqnAttributes: eqnAttributes.clone(), visited: -1 }, iSeqns), iIndex + 1, iMT.clone()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varName: cr1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: i1, tail: Deref @ metamodelica::ListNode::Nil }, false, false, Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varName: cr2, .. }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: i2, tail: Deref @ metamodelica::ListNode::Nil }, true, true) => {
                let mut colum: metamodelica::List<i32>;
                colum = ({let __elt = (*metamodelica::index_checked(&iMT.borrow(), i1.clone())?).clone(); __elt});
                metamodelica::arrayUpdate(iMT.clone(), i1.clone(), metamodelica::cons(iIndex, colum))?;
                return Ok((metamodelica::cons(SimpleContainer::TIMEALIAS { cr1: cr1.clone(), negatedCr1: negatedCr1, i1: i1.clone(), cr2: cr2.clone(), negatedCr2: negatedCr2, i2: i2.clone(), eqnAttributes: eqnAttributes.clone(), visited: -1 }, iSeqns), iIndex + 1, iMT.clone()))
            },
            (Deref @ metamodelica::ListNode::Nil, _, _, _, Deref @ metamodelica::ListNode::Nil, _, _, _) => {
                return Ok((iSeqns, iIndex, iMT.clone()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: v1, tail: vlst1 }, Deref @ metamodelica::ListNode::Cons { head: i1, tail: irest1 }, _, false, Deref @ metamodelica::ListNode::Cons { head: v2, tail: vlst2 }, Deref @ metamodelica::ListNode::Cons { head: i2, tail: irest2 }, _, false) => {
                let mut seqns: metamodelica::List<SimpleContainer>;
                let mut index: i32;
                let mut mT: metamodelica::Array<metamodelica::List<i32>>;
                (seqns, index, mT) = generateSimpleContainter(metamodelica::AsArg::as_arg(&v1), negatedCr1, i1.clone(), varskn1, metamodelica::AsArg::as_arg(&v2), negatedCr2, i2.clone(), varskn2, eqnAttributes.clone(), iSeqns, iIndex, iMT.clone())?;
                { (vars1, negatedCr1, ilst1, varskn1, time1, vars2, negatedCr2, ilst2, varskn2, time2, eqnAttributes, iSeqns, iIndex, iMT) = (vlst1.clone(), negatedCr1, irest1.clone(), varskn1, time1, vlst2.clone(), negatedCr2, irest2.clone(), varskn2, time2, eqnAttributes, seqns, index, mT.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn generateSimpleContainter(
    mut v1: &metamodelica::Ref<BackendDAE::Var>,
    mut negatedCr1: bool,
    mut i1: i32,
    mut varskn1: bool,
    mut v2: &metamodelica::Ref<BackendDAE::Var>,
    mut negatedCr2: bool,
    mut i2: i32,
    mut varskn2: bool,
    mut eqnAttributes: EquationSourceAndAttributes,
    mut iSeqns: metamodelica::List<SimpleContainer>,
    mut iIndex: i32,
    mut iMT: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<SimpleContainer>,
    i32,
    metamodelica::Array<metamodelica::List<i32>>,
)> {
    let mut oSeqns: metamodelica::List<SimpleContainer>;
    let mut oIndex: i32;
    let mut oMT: metamodelica::Array<metamodelica::List<i32>>;
    (oSeqns, oIndex, oMT) = (::match_deref::match_deref! { match &((v1.clone(), varskn1, v2.clone(), varskn2, eqnAttributes.clone())) {
        (Deref @ BackendDAE::Var { varName: cr1, .. }, false, Deref @ BackendDAE::Var { varName: cr2, .. }, false, _) => {
            let mut colum: metamodelica::List<i32>;
            checkEqualAlias(intEq(i1, i2), v1, negatedCr1, v2, negatedCr2, &eqnAttributes)?;
            colum = ({let __elt = (*metamodelica::index_checked(&iMT.borrow(), i1)?).clone(); __elt});
            metamodelica::arrayUpdate(iMT.clone(), i1, metamodelica::cons(iIndex, colum))?;
            colum = ({let __elt = (*metamodelica::index_checked(&iMT.borrow(), i2)?).clone(); __elt});
            metamodelica::arrayUpdate(iMT.clone(), i2, metamodelica::cons(iIndex, colum))?;
            (metamodelica::cons(SimpleContainer::ALIAS { cr1: cr1.clone(), negatedCr1: negatedCr1, i1: i1, cr2: cr2.clone(), negatedCr2: negatedCr2, i2: i2, eqnAttributes: eqnAttributes, visited: -1 }, iSeqns), iIndex + 1, iMT.clone())
        },
        (Deref @ BackendDAE::Var { varName: cr1, .. }, true, Deref @ BackendDAE::Var { varName: cr2, .. }, false, _) => {
            let mut colum: metamodelica::List<i32>;
            colum = ({let __elt = (*metamodelica::index_checked(&iMT.borrow(), i2)?).clone(); __elt});
            metamodelica::arrayUpdate(iMT.clone(), i2, metamodelica::cons(iIndex, colum))?;
            (metamodelica::cons(SimpleContainer::PARAMETERALIAS { unknowncr: cr2.clone(), negatedCr1: negatedCr2, i1: i2, paramcr: cr1.clone(), negatedCr2: negatedCr1, i2: i1, eqnAttributes: eqnAttributes, visited: -1 }, iSeqns), iIndex + 1, iMT.clone())
        },
        (Deref @ BackendDAE::Var { varName: cr1, .. }, false, Deref @ BackendDAE::Var { varName: cr2, .. }, true, _) => {
            let mut colum: metamodelica::List<i32>;
            colum = ({let __elt = (*metamodelica::index_checked(&iMT.borrow(), i1)?).clone(); __elt});
            metamodelica::arrayUpdate(iMT.clone(), i1, metamodelica::cons(iIndex, colum))?;
            (metamodelica::cons(SimpleContainer::PARAMETERALIAS { unknowncr: cr1.clone(), negatedCr1: negatedCr1, i1: i1, paramcr: cr2.clone(), negatedCr2: negatedCr2, i2: i2, eqnAttributes: eqnAttributes, visited: -1 }, iSeqns), iIndex + 1, iMT.clone())
        },
        (Deref @ BackendDAE::Var { varName: cr1, .. }, true, Deref @ BackendDAE::Var { varName: cr2, .. }, true, (source, _)) => {
            let mut crexp1: metamodelica::Ref<DAE::Exp>;
            let mut crexp2: metamodelica::Ref<DAE::Exp>;
            let mut lhs: ArcStr;
            let mut rhs: ArcStr;
            crexp1 = Expression::crefExp(cr1.clone())?;
            crexp2 = Expression::crefExp(cr2.clone())?;
            crexp1 = negateExpression(negatedCr1, crexp1.clone(), crexp1, &(literal!(" generateSimpleContainter ")))?;
            crexp2 = negateExpression(negatedCr2, crexp2.clone(), crexp2, &(literal!(" generateSimpleContainter ")))?;
            lhs = ExpressionBasics::printExpStr(crexp1)?;
            rhs = ExpressionBasics::printExpStr(crexp2)?;
            Error::addSourceMessage(&(Error::EQ_WITHOUT_TIME_DEP_VARS.clone()), list![lhs, rhs], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((oSeqns, oIndex, oMT))
}

fn checkEqualAlias(
    mut equal: bool,
    mut v1: &metamodelica::Ref<BackendDAE::Var>,
    mut negatedCr1: bool,
    mut v2: &metamodelica::Ref<BackendDAE::Var>,
    mut negatedCr2: bool,
    mut eqnAttributes: &EquationSourceAndAttributes,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((equal, v1.clone(), v2.clone(), eqnAttributes.clone())) {
        (false, _, _, _) => {
            ()
        },
        (true, Deref @ BackendDAE::Var { varName: cr1, .. }, Deref @ BackendDAE::Var { varName: cr2, .. }, (source, _)) => {
            let mut crexp1: metamodelica::Ref<DAE::Exp>;
            let mut crexp2: metamodelica::Ref<DAE::Exp>;
            let mut eqn_str: ArcStr;
            let mut var_str: ArcStr;
            let mut info: SourceInfo;
            var_str = BackendDump::varString(v1)?;
            crexp1 = Expression::crefExp(cr1.clone())?;
            crexp2 = Expression::crefExp(cr2.clone())?;
            crexp1 = negateExpression(negatedCr1, crexp1.clone(), crexp1, &(literal!(" checkEqualAlias ")))?;
            crexp2 = negateExpression(negatedCr2, crexp2.clone(), crexp2, &(literal!(" checkEqualAlias ")))?;
            eqn_str = { let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(crexp1)?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(crexp2)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
            info = ElementSource::getElementSourceFileInfo(source.clone());
            Error::addSourceMessage(&(Error::STRUCT_SINGULAR_SYSTEM.clone()), list![eqn_str, var_str], &info)?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn timeIndependentEquationAcausal(
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut eqnAttributes: EquationSourceAndAttributes,
    mut selfCalled: bool,
    mut inTpl: AccTuple,
) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = 'mc: {
        let __mc_input = (selfCalled, &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (vars, Deref @ BackendDAE::Shared { globalKnownVars, .. }, _, _, _, _, _)) => {
                    let mut ilst: metamodelica::List<i32>;
                    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut tree: metamodelica::Ref<AvlSetInt::Tree>;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(lhs.clone(), &fnptr!(traversingTimeVarsFinder, metamodelica::Ref<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool, metamodelica::List<i32>)), (false, vars.clone(), globalKnownVars.clone(), false, false, metamodelica::nil()))?) {
                        (_, (false, _, _, _, _, __pa0)) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ilst = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(rhs.clone(), &fnptr!(traversingTimeVarsFinder, metamodelica::Ref<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool, metamodelica::List<i32>)), (false, vars.clone(), globalKnownVars.clone(), false, false, ilst.clone()))?) {
                        (_, (false, _, _, _, _, __pa1)) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ilst = metamodelica::Own::own(__pa1);
                    tree = AvlSetInt::new();
                    tree = AvlSetInt::addList(tree.clone(), &ilst)?;
                    ilst = AvlSetInt::listKeys(&tree, metamodelica::nil());
                    vlst = List::map1r(ilst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    Ok(solveTimeIndependentAcausal(vlst.clone(), &ilst, lhs.clone(), rhs.clone(), eqnAttributes.clone(), inTpl.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _) => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    ty = Expression::r#typeof(lhs.clone())?;
                    Ok(generateEquation(lhs.clone(), rhs.clone(), &ty, &eqnAttributes, &inTpl)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTpl)
}

fn timeIndependentExpressionAcausal(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut eqnAttributes: EquationSourceAndAttributes,
    mut selfCalled: bool,
    mut inTpl: AccTuple,
) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = 'mc: {
        let __mc_input = (selfCalled, &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (vars, Deref @ BackendDAE::Shared { globalKnownVars, .. }, _, _, _, _, _)) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut ilst: metamodelica::List<i32>;
                    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut tree: metamodelica::Ref<AvlSetInt::Tree>;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(exp.clone(), &fnptr!(traversingTimeVarsFinder, metamodelica::Ref<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool, metamodelica::List<i32>)), (false, vars.clone(), globalKnownVars.clone(), false, false, metamodelica::nil()))?) {
                        (_, (false, _, _, _, _, __pa0)) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ilst = metamodelica::Own::own(__pa0);
                    tree = AvlSetInt::new();
                    tree = AvlSetInt::addList(tree.clone(), &ilst)?;
                    ilst = AvlSetInt::listKeys(&tree, metamodelica::nil());
                    vlst = List::map1r(ilst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    ty = Expression::r#typeof(exp.clone())?;
                    e2 = Expression::makeConstZero(&ty);
                    Ok(solveTimeIndependentAcausal(vlst.clone(), &ilst, exp.clone(), e2.clone(), eqnAttributes.clone(), inTpl.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    ty = Expression::r#typeof(exp.clone())?;
                    e2 = Expression::makeConstZero(&ty);
                    Ok(generateEquation(exp.clone(), e2.clone(), &ty, &eqnAttributes, &inTpl)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTpl)
}

fn toplevelInputOrUnfixed(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut b: bool;
    b = BackendVariable::isVarOnTopLevelAndInput(inVar)
        || BackendVariable::varUnreplaceable(inVar)
        || BackendVariable::isParam(inVar) && !(BackendVariable::varFixed(inVar));
    b
}

fn traversingTimeVarsFinder(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        bool,
        BackendDAE::Variables,
        BackendDAE::Variables,
        bool,
        bool,
        metamodelica::List<i32>,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        bool,
        BackendDAE::Variables,
        BackendDAE::Variables,
        bool,
        bool,
        metamodelica::List<i32>,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTuple: (
        bool,
        BackendDAE::Variables,
        BackendDAE::Variables,
        bool,
        bool,
        metamodelica::List<i32>,
    );
    (outExp, cont, outTuple) = 'mc: {
        let __mc_input = (&*inExp, &inTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", subscriptLst: Deref @ metamodelica::ListNode::Nil, .. }, ty: _ }, (b, vars, globalKnownVars, b1, b2, ilst)) => {
                    Ok((inExp.clone(), false, if (b.clone()) {inTuple.clone()} else {(true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone(), ilst.clone())}))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (b, vars, globalKnownVars, b1, b2, ilst)) => {
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&globalKnownVars))?) {
                        (__pa0, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    varlst = metamodelica::Own::own(__pa0);
                    let false = (List::none(&varlst, &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(toplevelInputOrUnfixed(&__a0)) })?) else { return Err("pattern mismatch") };
                    Ok((inExp.clone(), false, if (b.clone()) {inTuple.clone()} else {(true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone(), ilst.clone())}))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, (b, vars, globalKnownVars, b1, b2, ilst)) => {
                    Ok((inExp.clone(), false, if (b.clone()) {inTuple.clone()} else {(true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone(), ilst.clone())}))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, (b, vars, globalKnownVars, b1, b2, ilst)) => {
                    Ok((inExp.clone(), false, if (b.clone()) {inTuple.clone()} else {(true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone(), ilst.clone())}))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. }, (b, vars, globalKnownVars, b1, b2, ilst)) => {
                    Ok((inExp.clone(), false, if (b.clone()) {inTuple.clone()} else {(true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone(), ilst.clone())}))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. }, (b, vars, globalKnownVars, b1, b2, ilst)) => {
                    Ok((inExp.clone(), false, if (b.clone()) {inTuple.clone()} else {(true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone(), ilst.clone())}))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (b, vars, globalKnownVars, b1, b2, ilst)) => {
                    let mut vlst: metamodelica::List<i32>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    vlst = metamodelica::Own::own(__pa0);
                    vlst = listAppend(ilst.clone(), vlst.clone());
                    Ok((inExp.clone(), true, (b.clone(), vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone(), vlst.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (b, _, _, _, _, _)) => {
                    Ok((inExp.clone(), !(b.clone()), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTuple)
}

fn solveTimeIndependentAcausal(
    mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut ilst: &metamodelica::List<i32>,
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut eqnAttributes: EquationSourceAndAttributes,
    mut inTpl: AccTuple,
) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = (::match_deref::match_deref! { match &((vlst.clone(), ilst.clone(), eqnAttributes.clone(), inTpl.clone())) {
        (Deref @ metamodelica::ListNode::Cons { head: v @ Deref @ BackendDAE::Var { varName: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: i, tail: Deref @ metamodelica::ListNode::Nil }, _, _) => {
            let mut cre: metamodelica::Ref<DAE::Exp>;
            let mut es: metamodelica::Ref<DAE::Exp>;
            cre = Expression::crefExp(cr.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(ExpressionSolve::solve(lhs, rhs, cre, None)?) {
                (__pa0, Deref @ metamodelica::ListNode::Nil) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            es = metamodelica::Own::own(__pa0);
            constOrAliasAcausal(v.clone(), i.clone(), cr.clone(), es, eqnAttributes, &inTpl)?
        },
        (_, _, (source, eqAttr), (_, Deref @ BackendDAE::Shared { .. }, _, _, _, _, _)) => {
            let mut size: i32;
            size = Expression::sizeOf(&(Expression::r#typeof(lhs.clone())?));
            let true = (intEq(size, ((vlst).len() as i32))) else { return Err("pattern mismatch") };
            solveTimeIndependentAcausal1(vlst, ilst, lhs, rhs, &((source.clone(), eqAttr.clone())), inTpl)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

fn solveTimeIndependentAcausal1(
    mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut ilst: &metamodelica::List<i32>,
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut eqnAttributes: &EquationSourceAndAttributes,
    mut inTpl: AccTuple,
) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = (::match_deref::match_deref! { match &(inTpl.clone()) {
        _ => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut cre: metamodelica::Ref<DAE::Exp>;
            let mut es: metamodelica::Ref<DAE::Exp>;
            let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::map(vlst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::varCref(&__a0)) })?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            crlst = metamodelica::Own::own(__pa1);
            cr = ComponentReferenceBasics::crefStripLastSubs(&cr)?;
            let true = (List::all(&crlst, &({ let __pe_b0 = cr.clone(); move |__pe_a1| ComponentReferenceBasics::crefPrefixOf(&__pe_b0, &__pe_a1) }))?) else { return Err("pattern mismatch") };
            cre = Expression::crefExp(cr)?;
            let __pa2 = ::match_deref::match_deref! { match &(ExpressionSolve::solve(lhs, rhs, cre, None)?) {
                (__pa2, Deref @ metamodelica::ListNode::Nil) => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            es = metamodelica::Own::own(__pa2);
            constOrAliasArrayAcausal(&vlst, ilst, &es, eqnAttributes, inTpl)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

fn constOrAliasArrayAcausal<'__b>(
    mut vars: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut indxs: &'__b metamodelica::List<i32>,
    mut exp: &'__b metamodelica::Ref<DAE::Exp>,
    mut eqnAttributes: &'__b EquationSourceAndAttributes,
    mut inTpl: AccTuple,
) -> Result<AccTuple> {
    '__tco: loop {
        ::match_deref::match_deref! { match (vars, indxs) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(inTpl)
            },
            (Deref @ metamodelica::ListNode::Cons { head: v @ Deref @ BackendDAE::Var { varName: cr, .. }, tail: vlst }, Deref @ metamodelica::ListNode::Cons { head: i, tail: ilst }) => {
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                let mut tpl: AccTuple;
                subs = ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cr))?;
                e = Expression::applyExpSubscripts(exp.clone(), subs)?;
                tpl = constOrAliasAcausal(v.clone(), i.clone(), cr.clone(), e, eqnAttributes.clone(), &inTpl)?;
                { (vars, indxs, exp, eqnAttributes, inTpl) = (vlst, ilst, exp, eqnAttributes, tpl); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn constOrAliasAcausal(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut i: i32,
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut eqnAttributes: EquationSourceAndAttributes,
    mut inTpl: &AccTuple,
) -> Result<AccTuple> {
    let mut outTpl: AccTuple;
    outTpl = 'mc: {
        let __mc_input = inTpl;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, shared, eqns, seqns, index, mT, _) => {
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut cra: metamodelica::Ref<DAE::ComponentRef>;
                    let mut vars2: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut ilst2: metamodelica::List<i32>;
                    let mut negated: bool;
                    let mut seqns = (*seqns).clone();
                    let mut index = (*index).clone();
                    let mut mT = (*mT).clone();
                    (negated, cra) = aliasExp(&exp)?;
                    globalKnownVars = BackendVariable::daeGlobalKnownVars(metamodelica::AsArg::as_arg(&shared));
                    (vars2, ilst2) = BackendVariable::getVar(cra.clone(), &globalKnownVars)?;
                    (seqns, index, mT) = generateSimpleContainters(list![var.clone()], false, list![i], false, false, vars2.clone(), negated, ilst2.clone(), true, false, &eqnAttributes, seqns.clone(), index.clone(), mT.clone())?;
                    Ok((vars.clone(), shared.clone(), eqns.clone(), seqns.clone(), index.clone(), mT.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, shared, eqns, seqns, index, mT, _) => {
                    if !((Expression::isConstValue(&exp)?)) { return Err("guard") }
                    let mut colum: metamodelica::List<i32>;
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrCrefStrExpStr(&(literal!("Const Equation ")), &cr, &(literal!(" = ")), exp.clone(), &(literal!(" found.\n")))?;
                    }
                    colum = ({let __elt = (*metamodelica::index_checked(&mT.borrow(), i)?).clone(); __elt});
                    metamodelica::arrayUpdate(mT.clone(), i, metamodelica::cons(index.clone(), colum.clone()))?;
                    Ok((vars.clone(), shared.clone(), eqns.clone(), metamodelica::cons(SimpleContainer::TIMEINDEPENTVAR { cr: cr.clone(), i: i, exp: exp.clone(), eqnAttributes: eqnAttributes.clone(), visited: -1 }, seqns.clone()), index.clone() + 1, mT.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, shared @ Deref @ BackendDAE::Shared { functionTree: functions, .. }, eqns, seqns, index, mT, _) => {
                    if !((!(Expression::isImpure(exp.clone())?) && !(Expression::containsRecordType(exp.clone())?))) { return Err("guard") }
                    let mut colum: metamodelica::List<i32>;
                    let mut exp2: metamodelica::Ref<DAE::Exp>;
                    exp2 = EvaluateFunctions::evaluateConstantFunctionCallExp(&exp, metamodelica::AsArg::as_arg(&functions), false, Flags::getConfigInt(Flags::EVAL_RECURSION_LIMIT.clone())?);
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrCrefStrExpStr(&(literal!("Const Equation (through Ceval, case 1) ")), &cr, &(literal!(" = ")), exp.clone(), &(literal!(" found.\n")))?;
                    }
                    colum = ({let __elt = (*metamodelica::index_checked(&mT.borrow(), i)?).clone(); __elt});
                    metamodelica::arrayUpdate(mT.clone(), i, metamodelica::cons(index.clone(), colum.clone()))?;
                    Ok((vars.clone(), shared.clone(), eqns.clone(), metamodelica::cons(SimpleContainer::TIMEINDEPENTVAR { cr: cr.clone(), i: i, exp: exp2.clone(), eqnAttributes: eqnAttributes.clone(), visited: -1 }, seqns.clone()), index.clone() + 1, mT.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, shared, eqns, seqns, index, mT, _) => {
                    if !((!(Expression::isImpure(exp.clone())?) && !(Expression::containsRecordType(exp.clone())?))) { return Err("guard") }
                    let mut colum: metamodelica::List<i32>;
                    if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                        BackendDump::debugStrCrefStrExpStr(&(literal!("Const Equation (through Ceval, case 2) ")), &cr, &(literal!(" = ")), exp.clone(), &(literal!(" found.\n")))?;
                    }
                    colum = ({let __elt = (*metamodelica::index_checked(&mT.borrow(), i)?).clone(); __elt});
                    metamodelica::arrayUpdate(mT.clone(), i, metamodelica::cons(index.clone(), colum.clone()))?;
                    Ok((vars.clone(), shared.clone(), eqns.clone(), metamodelica::cons(SimpleContainer::TIMEINDEPENTVAR { cr: cr.clone(), i: i, exp: exp.clone(), eqnAttributes: eqnAttributes.clone(), visited: -1 }, seqns.clone()), index.clone() + 1, mT.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTpl)
}

fn aliasExp(mut exp: &metamodelica::Ref<DAE::Exp>) -> Result<(bool, metamodelica::Ref<DAE::ComponentRef>)> {
    let mut negate: bool;
    let mut outCr: metamodelica::Ref<DAE::ComponentRef>;
    (negate, outCr) = (::match_deref::match_deref! { match exp {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            (false, cr.clone())
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } } => {
            (true, cr.clone())
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } } => {
            (true, cr.clone())
        },
        Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } } => {
            (true, cr.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((negate, outCr))
}

fn handleSets(
    mut containerIdx: i32,
    mut inMark: i32,
    mut containerArr: metamodelica::Array<SimpleContainer>,
    mut iMT: metamodelica::Array<metamodelica::List<i32>>,
    mut unReplaceable: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut vars: BackendDAE::Variables,
    mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
    mut repl: BackendVarTransform::VariableReplacements,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::Ref<BackendDAE::Shared>,
    BackendVarTransform::VariableReplacements,
    bool,
)> {
    let mut vars: BackendDAE::Variables = vars;
    let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = eqnslst;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut warnAliasConflicts: bool = false;
    let mut rmax: Option<(i32, i32)>;
    let mut smax: Option<(i32, i32)>;
    let mut unremovable: Option<i32>;
    let mut r#const: Option<i32>;
    let mut mark: i32 = inMark;
    let mut b: bool;
    for mut idx in ({
        let __s = containerIdx;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        if !(intGt(
            getVisited(
                &({
                    let __elt = (*metamodelica::index_checked(&containerArr.borrow(), idx)?).clone();
                    __elt
                }),
            ),
            0,
        )) {
            (rmax, smax, unremovable, r#const, _) = getAlias(
                list![idx],
                None,
                mark,
                containerArr.clone(),
                iMT.clone(),
                &vars,
                unReplaceable,
                false,
                metamodelica::nil(),
                None,
                None,
                None,
                None,
            )?;
            (vars, eqnslst, shared, repl, b) = handleSet(
                rmax,
                smax,
                unremovable,
                r#const,
                mark + 1,
                containerArr.clone(),
                iMT.clone(),
                unReplaceable,
                vars,
                eqnslst,
                shared,
                repl,
            )?;
            mark = mark + 2;
            warnAliasConflicts = warnAliasConflicts || b;
        }
    }
    Ok((vars, eqnslst, shared, repl, warnAliasConflicts))
}

/// work item of the alias graph traversal in getAlias
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum AliasWork {
    /// containers still to visit on one level
    ALIAS_ROWS {
        rows: metamodelica::List<i32>,
        prevVar: Option<i32>,
        negate: bool,
        stack: metamodelica::List<i32>,
    },
    /// the second variable of an alias equation, once the first one is done
    ALIAS_SECOND {
        var: i32,
        container: i32,
        negate: bool,
        stack: metamodelica::List<i32>,
    },
}
impl metamodelica::gc::MMTrace for AliasWork {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            AliasWork::ALIAS_ROWS {
                rows,
                prevVar,
                negate,
                stack,
            } => {
                metamodelica::gc::MMTrace::mm_accept(rows, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prevVar, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(negate, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(stack, __mmv)?;
                Ok(())
            }
            AliasWork::ALIAS_SECOND {
                var,
                container,
                negate,
                stack,
            } => {
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(container, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(negate, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(stack, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for AliasWork {
    fn default() -> Self {
        Self::ALIAS_ROWS {
            rows: Default::default(),
            prevVar: Default::default(),
            negate: Default::default(),
            stack: Default::default(),
        }
    }
}
pub(crate) use self::AliasWork::{ALIAS_ROWS, ALIAS_SECOND};

fn getAlias(
    mut rows: metamodelica::List<i32>,
    mut prevVar: Option<i32>,
    mut mark: i32,
    mut containerArr: metamodelica::Array<SimpleContainer>,
    mut iMT: metamodelica::Array<metamodelica::List<i32>>,
    mut vars: &BackendDAE::Variables,
    mut unReplaceable: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut negate: bool,
    mut stack: metamodelica::List<i32>,
    mut iRmax: Option<(i32, i32)>,
    mut iSmax: Option<(i32, i32)>,
    mut iUnremovable: Option<i32>,
    mut iConst: Option<i32>,
) -> Result<(Option<(i32, i32)>, Option<(i32, i32)>, Option<i32>, Option<i32>, bool)> {
    let __ab_iMT = iMT.borrow();
    let mut oRmax: Option<(i32, i32)> = iRmax;
    let mut oSmax: Option<(i32, i32)> = iSmax;
    let mut oUnremovable: Option<i32> = iUnremovable;
    let mut oConst: Option<i32> = iConst;
    let mut oContinue: bool = true;
    let mut work: metamodelica::List<AliasWork> = list![AliasWork::ALIAS_ROWS {
        rows: rows.clone(),
        prevVar: prevVar.clone(),
        negate: negate,
        stack: stack.clone()
    }];
    let mut item: AliasWork;
    let mut container: SimpleContainer;
    let mut rest: metamodelica::List<i32>;
    let mut adjEqs: metamodelica::List<i32>;
    let mut currStack: metamodelica::List<i32>;
    let mut r: i32;
    let mut i: i32;
    let mut i1: i32;
    let mut i2: i32;
    let mut prevVarIdx: i32;
    let mut neg: bool;
    let mut negatedCr1: bool;
    let mut negatedCr2: bool;
    while !((work).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(work) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        item = metamodelica::Own::own(__pa0);
        work = metamodelica::Own::own(__pa1);
        let () = (match item.clone() {
            AliasWork::ALIAS_SECOND { .. } => {
                (oRmax, oSmax, oUnremovable) = getAliasScore(
                    var_field!(item.var, AliasWork::ALIAS_SECOND).clone(),
                    var_field!(item.container, AliasWork::ALIAS_SECOND).clone(),
                    vars,
                    unReplaceable,
                    oRmax,
                    oSmax,
                    oUnremovable,
                )?;
                if oContinue {
                    adjEqs = List::removeOnTrue(
                        var_field!(item.container, AliasWork::ALIAS_SECOND).clone(),
                        &fnptr!(intEq, i32, i32),
                        (*metamodelica::index_checked(
                            &__ab_iMT,
                            var_field!(item.var, AliasWork::ALIAS_SECOND).clone(),
                        )?)
                        .clone(),
                    )?;
                    work = metamodelica::cons(
                        AliasWork::ALIAS_ROWS {
                            rows: adjEqs,
                            prevVar: Some(var_field!(item.var, AliasWork::ALIAS_SECOND).clone()),
                            negate: var_field!(item.negate, AliasWork::ALIAS_SECOND).clone(),
                            stack: var_field!(item.stack, AliasWork::ALIAS_SECOND).clone(),
                        },
                        work,
                    );
                }
                ()
            }
            AliasWork::ALIAS_ROWS { .. }
                if (oContinue && !((var_field!(item.rows, AliasWork::ALIAS_ROWS)).is_empty())) =>
            {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(var_field!(item.rows, AliasWork::ALIAS_ROWS).clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                r = metamodelica::Own::own(__pa0);
                rest = metamodelica::Own::own(__pa1);
                container = ({
                    let __elt = (*metamodelica::index_checked(&containerArr.borrow(), r)?).clone();
                    __elt
                });
                if isVisited(mark, &container) {
                    oConst = getAliasCircular(
                        container,
                        r,
                        var_field!(item.negate, AliasWork::ALIAS_ROWS).clone(),
                        &(var_field!(item.stack, AliasWork::ALIAS_ROWS).clone()),
                        containerArr.clone(),
                        oUnremovable,
                    )?;
                    oRmax = None;
                    oSmax = None;
                    oUnremovable = None;
                    oContinue = false;
                } else {
                    metamodelica::arrayUpdate(containerArr.clone(), r, setVisited(mark, &container))?;
                    work = metamodelica::cons(
                        AliasWork::ALIAS_ROWS {
                            rows: rest,
                            prevVar: var_field!(item.prevVar, AliasWork::ALIAS_ROWS).clone(),
                            negate: var_field!(item.negate, AliasWork::ALIAS_ROWS).clone(),
                            stack: var_field!(item.stack, AliasWork::ALIAS_ROWS).clone(),
                        },
                        work,
                    );
                    currStack = metamodelica::cons(r, var_field!(item.stack, AliasWork::ALIAS_ROWS).clone());
                    let () = (match (container, var_field!(item.prevVar, AliasWork::ALIAS_ROWS).clone()) {
                        (
                            SimpleContainer::ALIAS {
                                i1: mut __esc_i1,
                                negatedCr1: mut __esc_negatedCr1,
                                i2: mut __esc_i2,
                                negatedCr2: mut __esc_negatedCr2,
                                ..
                            },
                            None,
                        ) => {
                            i1 = __esc_i1.clone();
                            negatedCr1 = __esc_negatedCr1.clone();
                            i2 = __esc_i2.clone();
                            negatedCr2 = __esc_negatedCr2.clone();
                            neg = boolOr(negatedCr1, negatedCr2);
                            neg = if (neg) {
                                !(var_field!(item.negate, AliasWork::ALIAS_ROWS).clone())
                            } else {
                                var_field!(item.negate, AliasWork::ALIAS_ROWS).clone()
                            };
                            (oRmax, oSmax, oUnremovable) =
                                getAliasScore(i1, r, vars, unReplaceable, oRmax, oSmax, oUnremovable)?;
                            adjEqs = List::removeOnTrue(
                                r,
                                &fnptr!(intEq, i32, i32),
                                (*metamodelica::index_checked(&__ab_iMT, i1)?).clone(),
                            )?;
                            work = metamodelica::cons(
                                AliasWork::ALIAS_SECOND {
                                    var: i2,
                                    container: r,
                                    negate: neg,
                                    stack: currStack.clone(),
                                },
                                work,
                            );
                            work = metamodelica::cons(
                                AliasWork::ALIAS_ROWS {
                                    rows: adjEqs,
                                    prevVar: Some(i1),
                                    negate: neg,
                                    stack: currStack,
                                },
                                work,
                            );
                            ()
                        }
                        (
                            SimpleContainer::ALIAS {
                                i1: mut __esc_i1,
                                negatedCr1: mut __esc_negatedCr1,
                                i2: mut __esc_i2,
                                negatedCr2: mut __esc_negatedCr2,
                                ..
                            },
                            Some(mut __esc_prevVarIdx),
                        ) => {
                            i1 = __esc_i1.clone();
                            negatedCr1 = __esc_negatedCr1.clone();
                            i2 = __esc_i2.clone();
                            negatedCr2 = __esc_negatedCr2.clone();
                            prevVarIdx = __esc_prevVarIdx.clone();
                            i = if (intEq(prevVarIdx, i1)) { i2 } else { i1 };
                            neg = boolOr(negatedCr1, negatedCr2);
                            neg = if (neg) {
                                !(var_field!(item.negate, AliasWork::ALIAS_ROWS).clone())
                            } else {
                                var_field!(item.negate, AliasWork::ALIAS_ROWS).clone()
                            };
                            (oRmax, oSmax, oUnremovable) =
                                getAliasScore(i, r, vars, unReplaceable, oRmax, oSmax, oUnremovable)?;
                            adjEqs = List::removeOnTrue(
                                r,
                                &fnptr!(intEq, i32, i32),
                                (*metamodelica::index_checked(&__ab_iMT, i)?).clone(),
                            )?;
                            work = metamodelica::cons(
                                AliasWork::ALIAS_ROWS {
                                    rows: adjEqs,
                                    prevVar: Some(i),
                                    negate: neg,
                                    stack: currStack,
                                },
                                work,
                            );
                            ()
                        }
                        _ => {
                            oRmax = None;
                            oSmax = None;
                            oUnremovable = None;
                            oConst = Some(r);
                            oContinue = false;
                            ()
                        }
                    });
                }
                ()
            }
            _ => (),
        });
    }
    Ok((oRmax, oSmax, oUnremovable, oConst, oContinue))
}

fn getAliasScore(
    mut varIdx: i32,
    mut containerIdx: i32,
    mut vars: &BackendDAE::Variables,
    mut unReplaceable: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut iRmax: Option<(i32, i32)>,
    mut iSmax: Option<(i32, i32)>,
    mut iUnremovable: Option<i32>,
) -> Result<(Option<(i32, i32)>, Option<(i32, i32)>, Option<i32>)> {
    let mut oRmax: Option<(i32, i32)>;
    let mut oSmax: Option<(i32, i32)>;
    let mut oUnremovable: Option<i32>;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut state: bool;
    let mut replaceable_: bool;
    let mut replaceble1: bool;
    v = BackendVariable::getVarAt(vars, varIdx)?;
    (replaceable_, replaceble1) = replaceableAlias(&v, unReplaceable);
    state = BackendVariable::isStateVar(&v) || BackendVariable::isClockedStateVar(&v);
    (oRmax, oSmax, oUnremovable) = getAlias3(
        &v,
        varIdx,
        state,
        replaceable_ && replaceble1,
        containerIdx,
        iRmax,
        iSmax,
        iUnremovable,
    )?;
    Ok((oRmax, oSmax, oUnremovable))
}

fn getAliasCircular(
    mut container: SimpleContainer,
    mut currIdx: i32,
    mut negate: bool,
    mut stack: &metamodelica::List<i32>,
    mut containerArr: metamodelica::Array<SimpleContainer>,
    mut iUnremovable: Option<i32>,
) -> Result<Option<i32>> {
    let mut oConst: Option<i32>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
    let mut msg: ArcStr = arcstr::literal!("");
    oConst = 'mc: {
        let __mc_input = (negate, iUnremovable.clone());
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (true, Some(_)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut cr: metamodelica::Ref<DAE::ComponentRef> = cr.clone();
            let SimpleContainer::ALIAS { cr1: __pa0, .. } = (container.clone()) else {
                return Err("pattern mismatch");
            };
            cr = metamodelica::Own::own(__pa0);
            let true = (Types::isIntegerOrRealOrSubTypeOfEither(ComponentReference::crefLastType(&cr)?)) else {
                return Err("pattern mismatch");
            };
            Ok((iUnremovable.clone(), cr.clone()))
        })() {
            cr = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (true, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut cr: metamodelica::Ref<DAE::ComponentRef> = cr.clone();
            let SimpleContainer::ALIAS { cr1: __pa0, .. } = (container.clone()) else {
                return Err("pattern mismatch");
            };
            cr = metamodelica::Own::own(__pa0);
            let true = (Types::isIntegerOrRealOrSubTypeOfEither(ComponentReference::crefLastType(&cr)?)) else {
                return Err("pattern mismatch");
            };
            Ok((Some(currIdx), cr.clone()))
        })() {
            cr = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut msg: ArcStr = msg.clone();
            msg = literal!("Circular Equalities Detected for Variables:\n");
            msg = circularEqualityMsg(stack, currIdx, containerArr.clone(), msg.clone())?;
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![msg.clone()])?;
            Ok((return Err("fail"), msg.clone()))
        })() {
            msg = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oConst)
}

fn circularEqualityMsg(
    mut stack: &metamodelica::List<i32>,
    mut iR: i32,
    mut simpleeqnsarr: metamodelica::Array<SimpleContainer>,
    mut iMsg: ArcStr,
) -> Result<ArcStr> {
    let mut oMsg: ArcStr;
    let mut lst: metamodelica::List<ArcStr>;
    let mut msg: ArcStr;
    lst = circularEqualityMsg_dispatch(stack, iR, simpleeqnsarr.clone())?;
    msg = stringDelimitList(lst, literal!("\n"));
    msg = stringAppendList(list![iMsg, msg, literal!("\n")]);
    oMsg = msg;
    Ok(oMsg)
}

fn circularEqualityMsg_dispatch(
    mut stack: &metamodelica::List<i32>,
    mut iR: i32,
    mut simpleeqnsarr: metamodelica::Array<SimpleContainer>,
) -> Result<metamodelica::List<ArcStr>> {
    let __ab_simpleeqnsarr = simpleeqnsarr.borrow();
    let mut oMsg: metamodelica::List<ArcStr> = metamodelica::nil();
    for mut r in &**stack {
        if r.clone() == iR {
            break;
        }
        for mut n in &*getVarsNames(&(*metamodelica::index_checked(&__ab_simpleeqnsarr, r.clone())?)) {
            oMsg = metamodelica::cons(
                ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&n))?,
                oMsg,
            );
        }
        oMsg = metamodelica::cons(literal!("----------------------------------"), oMsg);
    }
    metamodelica::Dangerous::listReverseInPlace(oMsg.clone());
    Ok(oMsg)
}

fn getVarsNames(mut iS: &SimpleContainer) -> metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> {
    let mut names: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    names = (match iS.clone() {
        SimpleContainer::ALIAS {
            cr1: mut cr1,
            cr2: mut cr2,
            ..
        } => {
            list![cr1.clone(), cr2.clone()]
        }
        SimpleContainer::PARAMETERALIAS {
            unknowncr: ref cr1,
            paramcr: ref cr2,
            ..
        } => {
            list![cr1.clone(), cr2.clone()]
        }
        SimpleContainer::TIMEALIAS {
            cr1: mut cr1,
            cr2: mut cr2,
            ..
        } => {
            list![cr1.clone(), cr2.clone()]
        }
        SimpleContainer::TIMEINDEPENTVAR { cr: ref cr1, .. } => {
            list![cr1.clone()]
        }
    });
    names
}

fn getAlias3(
    mut var: &metamodelica::Ref<BackendDAE::Var>,
    mut i: i32,
    mut state: bool,
    mut replaceable_: bool,
    mut r: i32,
    mut iRmax: Option<(i32, i32)>,
    mut iSmax: Option<(i32, i32)>,
    mut iUnremovable: Option<i32>,
) -> Result<(Option<(i32, i32)>, Option<(i32, i32)>, Option<i32>)> {
    let mut oRmax: Option<(i32, i32)>;
    let mut oSmax: Option<(i32, i32)>;
    let mut oUnremovable: Option<i32>;
    (oRmax, oSmax, oUnremovable) = (match (state, replaceable_, iRmax.clone(), iSmax.clone(), iUnremovable.clone()) {
        (false, false, _, _, None) => {
            let mut w1: i32;
            w1 = BackendVariable::calcAliasKey(var)?;
            (Some((i, w1)), iSmax, Some(i))
        }
        (true, false, _, _, None) => {
            let mut w1: i32;
            w1 = BackendVariable::varStateSelectPrioAlias(var);
            (iRmax, Some((i, w1)), Some(i))
        }
        (true, _, _, None, _) => {
            let mut w1: i32;
            w1 = BackendVariable::varStateSelectPrioAlias(var);
            (iRmax, Some((i, w1)), iUnremovable)
        }
        (true, _, _, Some((_, mut w2)), _) => {
            let mut w1: i32;
            let mut tpl: Option<(i32, i32)>;
            w1 = BackendVariable::varStateSelectPrioAlias(var);
            tpl = if (intGt(w1, w2.clone())) { Some((i, w1)) } else { iSmax };
            (iRmax, tpl, iUnremovable)
        }
        (false, _, None, _, _) => {
            let mut w1: i32;
            w1 = BackendVariable::calcAliasKey(var)?;
            (Some((i, w1)), iSmax, iUnremovable)
        }
        (false, _, Some((_, mut w2)), _, _) => {
            let mut w1: i32;
            let mut tpl: Option<(i32, i32)>;
            w1 = BackendVariable::calcAliasKey(var)?;
            tpl = if (intLt(w1, w2.clone())) { Some((i, w1)) } else { iRmax };
            (tpl, iSmax, iUnremovable)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((oRmax, oSmax, oUnremovable))
}

fn isVisited(mut mark: i32, mut iS: &SimpleContainer) -> bool {
    let mut visited: bool;
    visited = intEq(mark, getVisited(iS));
    visited
}

fn getVisited(mut iS: &SimpleContainer) -> i32 {
    let mut visited: i32;
    visited = (match iS.clone() {
        SimpleContainer::ALIAS {
            visited: mut __esc_visited,
            ..
        } => {
            visited = __esc_visited.clone();
            visited
        }
        SimpleContainer::PARAMETERALIAS {
            visited: mut __esc_visited,
            ..
        } => {
            visited = __esc_visited.clone();
            visited
        }
        SimpleContainer::TIMEALIAS {
            visited: mut __esc_visited,
            ..
        } => {
            visited = __esc_visited.clone();
            visited
        }
        SimpleContainer::TIMEINDEPENTVAR {
            visited: mut __esc_visited,
            ..
        } => {
            visited = __esc_visited.clone();
            visited
        }
    });
    visited
}

fn setVisited(mut visited: i32, mut iS: &SimpleContainer) -> SimpleContainer {
    let mut oS: SimpleContainer;
    oS = (match iS.clone() {
        SimpleContainer::ALIAS {
            cr1: mut cr1,
            negatedCr1: mut negatedCr1,
            i1: mut i1,
            cr2: mut cr2,
            negatedCr2: mut negatedCr2,
            i2: mut i2,
            eqnAttributes: mut eqnAttributes,
            visited: _,
        } => SimpleContainer::ALIAS {
            cr1: cr1.clone(),
            negatedCr1: negatedCr1.clone(),
            i1: i1.clone(),
            cr2: cr2.clone(),
            negatedCr2: negatedCr2.clone(),
            i2: i2.clone(),
            eqnAttributes: eqnAttributes.clone(),
            visited: visited,
        },
        SimpleContainer::PARAMETERALIAS {
            unknowncr: ref cr1,
            negatedCr1: mut negatedCr1,
            i1: mut i1,
            paramcr: ref cr2,
            negatedCr2: mut negatedCr2,
            i2: mut i2,
            eqnAttributes: mut eqnAttributes,
            visited: _,
        } => SimpleContainer::PARAMETERALIAS {
            unknowncr: cr1.clone(),
            negatedCr1: negatedCr1.clone(),
            i1: i1.clone(),
            paramcr: cr2.clone(),
            negatedCr2: negatedCr2.clone(),
            i2: i2.clone(),
            eqnAttributes: eqnAttributes.clone(),
            visited: visited,
        },
        SimpleContainer::TIMEALIAS {
            cr1: mut cr1,
            negatedCr1: mut negatedCr1,
            i1: mut i1,
            cr2: mut cr2,
            negatedCr2: mut negatedCr2,
            i2: mut i2,
            eqnAttributes: mut eqnAttributes,
            visited: _,
        } => SimpleContainer::TIMEALIAS {
            cr1: cr1.clone(),
            negatedCr1: negatedCr1.clone(),
            i1: i1.clone(),
            cr2: cr2.clone(),
            negatedCr2: negatedCr2.clone(),
            i2: i2.clone(),
            eqnAttributes: eqnAttributes.clone(),
            visited: visited,
        },
        SimpleContainer::TIMEINDEPENTVAR {
            cr: ref cr1,
            i: mut i1,
            exp: mut exp,
            eqnAttributes: mut eqnAttributes,
            visited: _,
        } => SimpleContainer::TIMEINDEPENTVAR {
            cr: cr1.clone(),
            i: i1.clone(),
            exp: exp.clone(),
            eqnAttributes: eqnAttributes.clone(),
            visited: visited,
        },
    });
    oS
}

fn replaceableAlias(
    mut var: &metamodelica::Ref<BackendDAE::Var>,
    mut unReplaceable: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> (bool, bool) {
    let mut res: bool;
    let mut res1: bool;
    (res, res1) = 'mc: {
        let __mc_input = &**var;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: cr, varKind: kind, .. } => {
                    let mut b: bool;
                    let mut cr = (*cr).clone();
                    BackendVariable::isVarKindVariable(metamodelica::AsArg::as_arg(&kind))?;
                    let false = (BackendVariable::isVarOnTopLevelAndOutput(var)) else { return Err("pattern mismatch") };
                    let false = (BackendVariable::isVarOnTopLevelAndInput(var)) else { return Err("pattern mismatch") };
                    let false = (BackendVariable::varHasUncertainValueRefine(var) && BackendDAEUtil::isDataReconciliationEnabled()?) else { return Err("pattern mismatch") };
                    cr = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&cr))?;
                    b = !(BaseHashSet::has(cr.clone(), unReplaceable)?);
                    Ok((true, b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((false, false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (res, res1)
}

fn handleSet(
    mut iRmax: Option<(i32, i32)>,
    mut iSmax: Option<(i32, i32)>,
    mut iUnremovable: Option<i32>,
    mut iConst: Option<i32>,
    mut mark: i32,
    mut simpleeqnsarr: metamodelica::Array<SimpleContainer>,
    mut iMT: metamodelica::Array<metamodelica::List<i32>>,
    mut unReplaceable: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut iVars: BackendDAE::Variables,
    mut iEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut iRepl: BackendVarTransform::VariableReplacements,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::Ref<BackendDAE::Shared>,
    BackendVarTransform::VariableReplacements,
    bool,
)> {
    let mut oVars: BackendDAE::Variables;
    let mut oEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut oRepl: BackendVarTransform::VariableReplacements;
    let mut warnAliasConflicts: bool = false;
    (oVars, oEqnslst, oshared, oRepl) = 'mc: {
        let __mc_input = (iRmax, iSmax, iUnremovable, iConst);
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, Some(mut r)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut s: SimpleContainer;
            let mut i1: i32;
            let mut i2: i32;
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut pv: metamodelica::Ref<BackendDAE::Var>;
            let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
            let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
            let mut eqnAttributes: EquationSourceAndAttributes;
            let mut negated: bool;
            let mut replaceable_: bool;
            let mut replaceble1: bool;
            let mut negatedCr1: bool;
            let mut negatedCr2: bool;
            let mut exp2: metamodelica::Ref<DAE::Exp>;
            let mut expcr: metamodelica::Ref<DAE::Exp>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut vars: BackendDAE::Variables;
            let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut vsattr: VarSetAttributes;
            let mut rows: metamodelica::List<i32>;
            s = ({
                let __elt = (*metamodelica::index_checked(&simpleeqnsarr.borrow(), r.clone())?).clone();
                __elt
            });
            let SimpleContainer::PARAMETERALIAS {
                unknowncr: __pa0,
                negatedCr1: __pa1,
                i1: __pa2,
                negatedCr2: __pa3,
                i2: __pa4,
                paramcr: __pa5,
                eqnAttributes: __pa6,
                ..
            } = (s.clone())
            else {
                return Err("pattern mismatch");
            };
            cr1 = metamodelica::Own::own(__pa0);
            negatedCr1 = metamodelica::Own::own(__pa1);
            i1 = metamodelica::Own::own(__pa2);
            negatedCr2 = metamodelica::Own::own(__pa3);
            i2 = metamodelica::Own::own(__pa4);
            cr2 = metamodelica::Own::own(__pa5);
            eqnAttributes = metamodelica::Own::own(__pa6);
            metamodelica::arrayUpdate(simpleeqnsarr.clone(), r.clone(), setVisited(mark, &s))?;
            negated = boolOr(negatedCr1, negatedCr2);
            exp = Expression::crefExp(cr2.clone())?;
            exp2 = negateExpression(negated, exp.clone(), exp.clone(), &(literal!(" PARAMETERALIAS ")))?;
            v = BackendVariable::getVarAt(&iVars, i1)?;
            (replaceable_, replaceble1) = replaceableAlias(&v, unReplaceable);
            (vars, eqnslst, shared, repl) = handleSetVar(
                replaceable_ && replaceble1,
                Some(metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(0.0_f64),
                })),
                v.clone(),
                i1,
                &eqnAttributes,
                exp2.clone(),
                iMT.clone(),
                iVars.clone(),
                iEqnslst.clone(),
                ishared.clone(),
                iRepl.clone(),
            )?;
            expcr = Expression::crefExp(cr1.clone())?;
            pv = BackendVariable::getVarSharedAt(i2, &ishared)?;
            vsattr = addVarSetAttributes(
                &pv,
                negated,
                mark,
                simpleeqnsarr.clone(),
                &(EMPTYVARSETATTRIBUTES().clone()),
            )?;
            vsattr = if (replaceable_ && replaceble1) {
                addVarSetAttributes(&v, negated, mark, simpleeqnsarr.clone(), &vsattr)?
            } else {
                vsattr.clone()
            };
            rows = List::removeOnTrue(
                r.clone(),
                &fnptr!(intEq, i32, i32),
                ({
                    let __elt = (*metamodelica::index_checked(&iMT.borrow(), i1)?).clone();
                    __elt
                }),
            )?;
            metamodelica::arrayUpdate(iMT.clone(), i1, metamodelica::nil())?;
            (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(
                &rows,
                i1,
                &exp,
                Some(expcr.clone()),
                negated,
                Some(metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(0.0_f64),
                })),
                mark,
                simpleeqnsarr.clone(),
                iMT.clone(),
                unReplaceable,
                vars.clone(),
                eqnslst.clone(),
                shared.clone(),
                repl.clone(),
                vsattr.clone(),
            )?;
            Ok((vars.clone(), eqnslst.clone(), shared.clone(), repl.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, Some(mut r)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut s: SimpleContainer;
            let mut i1: i32;
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
            let mut eqnAttributes: EquationSourceAndAttributes;
            let mut negated: bool;
            let mut replaceable_: bool;
            let mut replaceble1: bool;
            let mut negatedCr1: bool;
            let mut negatedCr2: bool;
            let mut exp1: metamodelica::Ref<DAE::Exp>;
            let mut expcr: metamodelica::Ref<DAE::Exp>;
            let mut dexp: metamodelica::Ref<DAE::Exp>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut vars: BackendDAE::Variables;
            let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut vsattr: VarSetAttributes;
            let mut rows: metamodelica::List<i32>;
            s = ({
                let __elt = (*metamodelica::index_checked(&simpleeqnsarr.borrow(), r.clone())?).clone();
                __elt
            });
            let SimpleContainer::TIMEALIAS {
                cr1: __pa0,
                i1: __pa1,
                negatedCr1: __pa2,
                negatedCr2: __pa3,
                eqnAttributes: __pa4,
                ..
            } = (s.clone())
            else {
                return Err("pattern mismatch");
            };
            cr1 = metamodelica::Own::own(__pa0);
            i1 = metamodelica::Own::own(__pa1);
            negatedCr1 = metamodelica::Own::own(__pa2);
            negatedCr2 = metamodelica::Own::own(__pa3);
            eqnAttributes = metamodelica::Own::own(__pa4);
            metamodelica::arrayUpdate(simpleeqnsarr.clone(), r.clone(), setVisited(mark, &s))?;
            negated = boolOr(negatedCr1, negatedCr2);
            exp = Expression::crefExp(DAE::crefTime().clone())?;
            exp1 = negateExpression(negated, exp.clone(), exp.clone(), &(literal!(" timealias ")))?;
            v = BackendVariable::getVarAt(&iVars, i1)?;
            (replaceable_, replaceble1) = replaceableAlias(&v, unReplaceable);
            dexp = negateExpression(
                negated,
                exp.clone(),
                metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(1.0_f64),
                }),
                &(literal!(" timealias der ")),
            )?;
            (vars, eqnslst, shared, repl) = handleSetVar(
                replaceable_ && replaceble1,
                Some(dexp.clone()),
                v.clone(),
                i1,
                &eqnAttributes,
                exp1.clone(),
                iMT.clone(),
                iVars.clone(),
                iEqnslst.clone(),
                ishared.clone(),
                iRepl.clone(),
            )?;
            expcr = Expression::crefExp(cr1.clone())?;
            vsattr = addVarSetAttributes(
                &v,
                negated,
                mark,
                simpleeqnsarr.clone(),
                &(EMPTYVARSETATTRIBUTES().clone()),
            )?;
            rows = List::removeOnTrue(
                r.clone(),
                &fnptr!(intEq, i32, i32),
                ({
                    let __elt = (*metamodelica::index_checked(&iMT.borrow(), i1)?).clone();
                    __elt
                }),
            )?;
            metamodelica::arrayUpdate(iMT.clone(), i1, metamodelica::nil())?;
            (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(
                &rows,
                i1,
                &exp,
                Some(expcr.clone()),
                negated,
                Some(dexp.clone()),
                mark,
                simpleeqnsarr.clone(),
                iMT.clone(),
                unReplaceable,
                vars.clone(),
                eqnslst.clone(),
                shared.clone(),
                repl.clone(),
                vsattr.clone(),
            )?;
            Ok((vars.clone(), eqnslst.clone(), shared.clone(), repl.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, Some(mut r)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut s: SimpleContainer;
            let mut i: i32;
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut eqnAttributes: EquationSourceAndAttributes;
            let mut replaceable_: bool;
            let mut replaceble1: bool;
            let mut constExp: bool;
            let mut isState: bool;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut vars: BackendDAE::Variables;
            let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut vsattr: VarSetAttributes;
            let mut rows: metamodelica::List<i32>;
            s = ({
                let __elt = (*metamodelica::index_checked(&simpleeqnsarr.borrow(), r.clone())?).clone();
                __elt
            });
            let SimpleContainer::TIMEINDEPENTVAR {
                cr: __pa0,
                i: __pa1,
                exp: __pa2,
                eqnAttributes: __pa3,
                ..
            } = (s.clone())
            else {
                return Err("pattern mismatch");
            };
            cr = metamodelica::Own::own(__pa0);
            i = metamodelica::Own::own(__pa1);
            exp = metamodelica::Own::own(__pa2);
            eqnAttributes = metamodelica::Own::own(__pa3);
            metamodelica::arrayUpdate(simpleeqnsarr.clone(), r.clone(), setVisited(mark, &s))?;
            let __arc6 = BackendVariable::getVarAt(&iVars, i)?;
            let __pa5 = (__arc6).clone();
            let BackendDAE::VAR { varName: __pa4, .. } = &*__arc6;
            cr = metamodelica::Own::own(__pa4);
            v = metamodelica::Own::own(__pa5);
            (replaceable_, replaceble1) = replaceableAlias(&v, unReplaceable);
            (vars, shared, isState, eqnslst) = optMoveVarShared(
                replaceable_,
                v.clone(),
                i,
                &eqnAttributes,
                exp.clone(),
                &BackendVariable::addGlobalKnownVarDAE,
                iMT.clone(),
                iVars.clone(),
                ishared.clone(),
                iEqnslst.clone(),
            )?;
            constExp = Expression::isConstValue(&exp)?;
            repl = if (replaceable_ && constExp && replaceble1) {
                BackendVarTransform::addReplacement(
                    iRepl.clone(),
                    cr.clone(),
                    exp.clone(),
                    Some(
                        (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(BackendVarTransform::skipPreChangeEdgeOperator(&__a0))
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static,
                            >),
                    ),
                )?
            } else {
                iRepl.clone()
            };
            repl = if (isState) {
                BackendVarTransform::addDerConstRepl(
                    cr.clone(),
                    metamodelica::Ref::new(DAE::Exp::RCONST {
                        real: metamodelica::OrderedFloat(0.0_f64),
                    }),
                    repl.clone(),
                )?
            } else {
                repl.clone()
            };
            exp = Expression::crefExp(cr.clone())?;
            vsattr = addVarSetAttributes(
                &v,
                false,
                mark,
                simpleeqnsarr.clone(),
                &(EMPTYVARSETATTRIBUTES().clone()),
            )?;
            rows = List::removeOnTrue(
                r.clone(),
                &fnptr!(intEq, i32, i32),
                ({
                    let __elt = (*metamodelica::index_checked(&iMT.borrow(), i)?).clone();
                    __elt
                }),
            )?;
            metamodelica::arrayUpdate(iMT.clone(), i, metamodelica::nil())?;
            (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(
                &rows,
                i,
                &exp,
                None,
                false,
                Some(metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(0.0_f64),
                })),
                mark,
                simpleeqnsarr.clone(),
                iMT.clone(),
                unReplaceable,
                vars.clone(),
                eqnslst.clone(),
                shared.clone(),
                repl.clone(),
                vsattr.clone(),
            )?;
            Ok((vars.clone(), eqnslst.clone(), shared.clone(), repl.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, Some(mut r)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut s: SimpleContainer;
            let mut i2: i32;
            let mut i: i32;
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut eqnAttributes: EquationSourceAndAttributes;
            let mut replaceable_: bool;
            let mut replaceble1: bool;
            let mut constExp: bool;
            let mut isState: bool;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut vars: BackendDAE::Variables;
            let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut vsattr: VarSetAttributes;
            let mut rows: metamodelica::List<i32>;
            s = ({
                let __elt = (*metamodelica::index_checked(&simpleeqnsarr.borrow(), r.clone())?).clone();
                __elt
            });
            let SimpleContainer::ALIAS {
                i1: __pa0,
                i2: __pa1,
                eqnAttributes: __pa2,
                ..
            } = (s.clone())
            else {
                return Err("pattern mismatch");
            };
            i = metamodelica::Own::own(__pa0);
            i2 = metamodelica::Own::own(__pa1);
            eqnAttributes = metamodelica::Own::own(__pa2);
            metamodelica::arrayUpdate(simpleeqnsarr.clone(), r.clone(), setVisited(mark, &s))?;
            let __arc5 = BackendVariable::getVarAt(&iVars, i)?;
            let __pa4 = (__arc5).clone();
            let BackendDAE::VAR { varName: __pa3, .. } = &*__arc5;
            cr = metamodelica::Own::own(__pa3);
            v = metamodelica::Own::own(__pa4);
            exp = if (Types::isRealOrSubTypeReal(ComponentReference::crefLastType(&cr)?)) {
                metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(0.0_f64),
                })
            } else {
                metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 })
            };
            (replaceable_, replaceble1) = replaceableAlias(&v, unReplaceable);
            (vars, shared, isState, eqnslst) = optMoveVarShared(
                replaceable_,
                v.clone(),
                i,
                &eqnAttributes,
                exp.clone(),
                &BackendVariable::addGlobalKnownVarDAE,
                iMT.clone(),
                iVars.clone(),
                ishared.clone(),
                iEqnslst.clone(),
            )?;
            constExp = Expression::isConstValue(&exp)?;
            repl = if (replaceable_ && constExp && replaceble1) {
                BackendVarTransform::addReplacement(
                    iRepl.clone(),
                    cr.clone(),
                    exp.clone(),
                    Some(
                        (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(BackendVarTransform::skipPreChangeEdgeOperator(&__a0))
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static,
                            >),
                    ),
                )?
            } else {
                iRepl.clone()
            };
            repl = if (isState) {
                BackendVarTransform::addDerConstRepl(
                    cr.clone(),
                    metamodelica::Ref::new(DAE::Exp::RCONST {
                        real: metamodelica::OrderedFloat(0.0_f64),
                    }),
                    repl.clone(),
                )?
            } else {
                repl.clone()
            };
            exp = Expression::crefExp(cr.clone())?;
            vsattr = addVarSetAttributes(
                &v,
                false,
                mark,
                simpleeqnsarr.clone(),
                &(EMPTYVARSETATTRIBUTES().clone()),
            )?;
            rows = List::removeOnTrue(
                r.clone(),
                &fnptr!(intEq, i32, i32),
                ({
                    let __elt = (*metamodelica::index_checked(&iMT.borrow(), i2)?).clone();
                    __elt
                }),
            )?;
            metamodelica::arrayUpdate(iMT.clone(), i2, rows.clone())?;
            rows = List::removeOnTrue(
                r.clone(),
                &fnptr!(intEq, i32, i32),
                ({
                    let __elt = (*metamodelica::index_checked(&iMT.borrow(), i)?).clone();
                    __elt
                }),
            )?;
            metamodelica::arrayUpdate(iMT.clone(), i, metamodelica::nil())?;
            (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(
                &rows,
                i,
                &exp,
                None,
                false,
                Some(metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(0.0_f64),
                })),
                mark,
                simpleeqnsarr.clone(),
                iMT.clone(),
                unReplaceable,
                vars.clone(),
                eqnslst.clone(),
                shared.clone(),
                repl.clone(),
                vsattr.clone(),
            )?;
            Ok((vars.clone(), eqnslst.clone(), shared.clone(), repl.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (_, Some((mut i, _)), _, None) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut vars: BackendDAE::Variables;
            let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut vsattr: VarSetAttributes;
            let mut oexp: Option<metamodelica::Ref<DAE::Exp>>;
            let mut warnAliasConflicts: bool = warnAliasConflicts.clone();
            let __arc2 = BackendVariable::getVarAt(&iVars, i.clone())?;
            let __pa1 = (__arc2).clone();
            let BackendDAE::VAR { varName: __pa0, .. } = &*__arc2;
            cr = metamodelica::Own::own(__pa0);
            v = metamodelica::Own::own(__pa1);
            exp = Expression::crefExp(cr.clone())?;
            vsattr = addVarSetAttributes(
                &v,
                false,
                mark,
                simpleeqnsarr.clone(),
                &(EMPTYVARSETATTRIBUTES().clone()),
            )?;
            oexp = varStateDerivative(&v)?;
            (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(
                &({
                    let __elt = (*metamodelica::index_checked(&iMT.borrow(), i.clone())?).clone();
                    __elt
                }),
                i.clone(),
                &exp,
                None,
                false,
                oexp.clone(),
                mark,
                simpleeqnsarr.clone(),
                iMT.clone(),
                unReplaceable,
                iVars.clone(),
                iEqnslst.clone(),
                ishared.clone(),
                iRepl.clone(),
                vsattr.clone(),
            )?;
            metamodelica::arrayUpdate(iMT.clone(), i.clone(), metamodelica::nil())?;
            (vars, warnAliasConflicts) = handleVarSetAttributes(&vsattr, v.clone(), vars.clone(), &shared)?;
            Ok((
                (vars.clone(), eqnslst.clone(), shared.clone(), repl.clone()),
                warnAliasConflicts.clone(),
            ))
        })() {
            warnAliasConflicts = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (_, None, Some(mut i), None) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut vars: BackendDAE::Variables;
            let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut vsattr: VarSetAttributes;
            let mut warnAliasConflicts: bool = warnAliasConflicts.clone();
            let __arc2 = BackendVariable::getVarAt(&iVars, i.clone())?;
            let __pa1 = (__arc2).clone();
            let BackendDAE::VAR { varName: __pa0, .. } = &*__arc2;
            cr = metamodelica::Own::own(__pa0);
            v = metamodelica::Own::own(__pa1);
            exp = Expression::crefExp(cr.clone())?;
            vsattr = addVarSetAttributes(
                &v,
                false,
                mark,
                simpleeqnsarr.clone(),
                &(EMPTYVARSETATTRIBUTES().clone()),
            )?;
            (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(
                &({
                    let __elt = (*metamodelica::index_checked(&iMT.borrow(), i.clone())?).clone();
                    __elt
                }),
                i.clone(),
                &exp,
                None,
                false,
                None,
                mark,
                simpleeqnsarr.clone(),
                iMT.clone(),
                unReplaceable,
                iVars.clone(),
                iEqnslst.clone(),
                ishared.clone(),
                iRepl.clone(),
                vsattr.clone(),
            )?;
            metamodelica::arrayUpdate(iMT.clone(), i.clone(), metamodelica::nil())?;
            (vars, warnAliasConflicts) = handleVarSetAttributes(&vsattr, v.clone(), vars.clone(), &shared)?;
            Ok((
                (vars.clone(), eqnslst.clone(), shared.clone(), repl.clone()),
                warnAliasConflicts.clone(),
            ))
        })() {
            warnAliasConflicts = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (Some((mut i, _)), None, _, None) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut vars: BackendDAE::Variables;
            let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut vsattr: VarSetAttributes;
            let mut warnAliasConflicts: bool = warnAliasConflicts.clone();
            let __arc2 = BackendVariable::getVarAt(&iVars, i.clone())?;
            let __pa1 = (__arc2).clone();
            let BackendDAE::VAR { varName: __pa0, .. } = &*__arc2;
            cr = metamodelica::Own::own(__pa0);
            v = metamodelica::Own::own(__pa1);
            exp = Expression::crefExp(cr.clone())?;
            vsattr = addVarSetAttributes(
                &v,
                false,
                mark,
                simpleeqnsarr.clone(),
                &(EMPTYVARSETATTRIBUTES().clone()),
            )?;
            (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(
                &({
                    let __elt = (*metamodelica::index_checked(&iMT.borrow(), i.clone())?).clone();
                    __elt
                }),
                i.clone(),
                &exp,
                None,
                false,
                None,
                mark,
                simpleeqnsarr.clone(),
                iMT.clone(),
                unReplaceable,
                iVars.clone(),
                iEqnslst.clone(),
                ishared.clone(),
                iRepl.clone(),
                vsattr.clone(),
            )?;
            metamodelica::arrayUpdate(iMT.clone(), i.clone(), metamodelica::nil())?;
            (vars, warnAliasConflicts) = handleVarSetAttributes(&vsattr, v.clone(), vars.clone(), &shared)?;
            Ok((
                (vars.clone(), eqnslst.clone(), shared.clone(), repl.clone()),
                warnAliasConflicts.clone(),
            ))
        })() {
            warnAliasConflicts = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((oVars, oEqnslst, oshared, oRepl, warnAliasConflicts))
}

fn varStateDerivative(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut outExp: Option<metamodelica::Ref<DAE::Exp>>;
    outExp = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(dcr), .. }, .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = Expression::crefExp(dcr.clone())?;
            Some(e)
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn handleSetVar(
    mut replaceable_: bool,
    mut derReplaceState: Option<metamodelica::Ref<DAE::Exp>>,
    mut v: metamodelica::Ref<BackendDAE::Var>,
    mut i: i32,
    mut eqnAttributes: &EquationSourceAndAttributes,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut iMT: metamodelica::Array<metamodelica::List<i32>>,
    mut iVars: BackendDAE::Variables,
    mut iEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut iRepl: BackendVarTransform::VariableReplacements,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::Ref<BackendDAE::Shared>,
    BackendVarTransform::VariableReplacements,
)> {
    let mut oVars: BackendDAE::Variables;
    let mut oEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut oRepl: BackendVarTransform::VariableReplacements;
    (oVars, oEqnslst, oshared, oRepl) = (::match_deref::match_deref! { match &((replaceable_, v.clone(), eqnAttributes.clone())) {
        (true, Deref @ BackendDAE::Var { varName: cr, .. }, (source, _)) => {
            let mut vars: BackendDAE::Variables;
            let mut bs: bool;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut repl: BackendVarTransform::VariableReplacements;
            (vars, shared, bs) = moveVarShared(v, i, metamodelica::AsArg::as_arg(&source), exp.clone(), &BackendVariable::addAliasVarDAE, iVars, ishared)?;
            repl = BackendVarTransform::addReplacement(iRepl, cr.clone(), exp, Some((std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVarTransform::skipPreChangeEdgeOperator(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>)))?;
            repl = addDerConstRepl(bs, derReplaceState, cr.clone(), repl)?;
            (vars, iEqnslst, shared, repl)
        },
        (false, Deref @ BackendDAE::Var { varName: cr, .. }, _) => {
            let mut crexp: metamodelica::Ref<DAE::Exp>;
            let mut vars: BackendDAE::Variables;
            let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            crexp = Expression::crefExp(cr.clone())?;
            (vars, shared, eqnslst, _, _, _, _) = generateEquation(crexp, exp.clone(), &(Expression::r#typeof(exp)?), eqnAttributes, &((iVars, ishared, iEqnslst, metamodelica::nil(), -1, iMT.clone(), false)))?;
            (vars, eqnslst, shared, iRepl)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((oVars, oEqnslst, oshared, oRepl))
}

fn addDerConstRepl(
    mut state: bool,
    mut derConstRepl: Option<metamodelica::Ref<DAE::Exp>>,
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut iRepl: BackendVarTransform::VariableReplacements,
) -> Result<BackendVarTransform::VariableReplacements> {
    let mut oRepl: BackendVarTransform::VariableReplacements;
    oRepl = (::match_deref::match_deref! { match &((state, derConstRepl)) {
        (true, Some(e)) => {
            BackendVarTransform::addDerConstRepl(cr, e.clone(), iRepl)?
        },
        _ => {
            iRepl
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oRepl)
}

fn optMoveVarShared(
    mut replaceable_: bool,
    mut v: metamodelica::Ref<BackendDAE::Var>,
    mut i: i32,
    mut eqnAttributes: &EquationSourceAndAttributes,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<BackendDAE::Var>,
        metamodelica::Ref<BackendDAE::Shared>,
    ) -> Result<metamodelica::Ref<BackendDAE::Shared>>,
    mut iMT: metamodelica::Array<metamodelica::List<i32>>,
    mut iVars: BackendDAE::Variables,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut iEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    pub type FuncMoveVarShared = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::Var>,
                metamodelica::Ref<BackendDAE::Shared>,
            ) -> Result<metamodelica::Ref<BackendDAE::Shared>>
            + 'static,
    >;

    let mut oVars: BackendDAE::Variables;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut bs: bool;
    let mut oEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    (oVars, oshared, bs, oEqnslst) = (::match_deref::match_deref! { match &((replaceable_, v.clone(), eqnAttributes.clone())) {
        (true, _, (source, _)) => {
            (oVars, oshared, bs) = moveVarShared(v, i, metamodelica::AsArg::as_arg(&source), exp, func, iVars, ishared)?;
            (oVars, oshared, bs, iEqnslst)
        },
        (false, Deref @ BackendDAE::Var { varName: cr, .. }, _) => {
            let mut crexp: metamodelica::Ref<DAE::Exp>;
            crexp = Expression::crefExp(cr.clone())?;
            (oVars, oshared, oEqnslst, _, _, _, _) = generateEquation(crexp, exp.clone(), &(Expression::r#typeof(exp)?), eqnAttributes, &((iVars, ishared, iEqnslst, metamodelica::nil(), -1, iMT.clone(), false)))?;
            (oVars, oshared, false, oEqnslst)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((oVars, oshared, bs, oEqnslst))
}

fn moveVarShared(
    mut v: metamodelica::Ref<BackendDAE::Var>,
    mut i: i32,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<BackendDAE::Var>,
        metamodelica::Ref<BackendDAE::Shared>,
    ) -> Result<metamodelica::Ref<BackendDAE::Shared>>,
    mut iVars: BackendDAE::Variables,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(BackendDAE::Variables, metamodelica::Ref<BackendDAE::Shared>, bool)> {
    pub type FuncMoveVarShared = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::Var>,
                metamodelica::Ref<BackendDAE::Shared>,
            ) -> Result<metamodelica::Ref<BackendDAE::Shared>>
            + 'static,
    >;

    let mut oVars: BackendDAE::Variables;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut bs: bool;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
    let mut v1: metamodelica::Ref<BackendDAE::Var>;
    let __arc1 = v.clone();
    let BackendDAE::VAR { varName: __pa0, .. } = &*__arc1;
    cr = metamodelica::Own::own(__pa0);
    v1 = BackendVariable::setBindExp(v.clone(), Some(exp.clone()));
    ops = ElementSource::getSymbolicTransformations(source);
    v1 = BackendVariable::mergeVariableOperations(
        v1,
        metamodelica::cons(
            metamodelica::Ref::new(DAE::SymbolicOperation::SOLVED { cr: cr, exp: exp }),
            ops,
        ),
    )?;
    bs = BackendVariable::isStateVar(&v);
    v1 = if (bs) {
        BackendVariable::setVarKind(v1, openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE)?
    } else {
        v1
    };
    (oVars, _) = BackendVariable::removeVar(i, iVars)?;
    oshared = func(v1, ishared)?;
    Ok((oVars, oshared, bs))
}

fn traverseAliasTree<'__b>(
    mut rows: &'__b metamodelica::List<i32>,
    mut ilast: i32,
    mut exp: &'__b metamodelica::Ref<DAE::Exp>,
    mut optExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut globalnegate: bool,
    mut derReplaceState: Option<metamodelica::Ref<DAE::Exp>>,
    mut mark: i32,
    mut simpleeqnsarr: metamodelica::Array<SimpleContainer>,
    mut iMT: metamodelica::Array<metamodelica::List<i32>>,
    mut unReplaceable: &'__b (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut iVars: BackendDAE::Variables,
    mut iEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut iRepl: BackendVarTransform::VariableReplacements,
    mut iAttributes: VarSetAttributes,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::Ref<BackendDAE::Shared>,
    BackendVarTransform::VariableReplacements,
    VarSetAttributes,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match rows {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((iVars, iEqnslst, ishared, iRepl, iAttributes))
            },
            Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                let mut vars: BackendDAE::Variables;
                let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                let mut repl: BackendVarTransform::VariableReplacements;
                let mut s: SimpleContainer;
                let mut vsattr: VarSetAttributes;
                s = ({let __elt = (*metamodelica::index_checked(&simpleeqnsarr.borrow(), r.clone())?).clone(); __elt});
                metamodelica::arrayUpdate(simpleeqnsarr.clone(), r.clone(), setVisited(mark, &s))?;
                (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree1(&s, r.clone(), ilast, exp.clone(), optExp.clone(), globalnegate, derReplaceState.clone(), mark, simpleeqnsarr.clone(), iMT.clone(), unReplaceable, iVars, iEqnslst, ishared, iRepl, iAttributes)?;
                { (rows, ilast, exp, optExp, globalnegate, derReplaceState, mark, simpleeqnsarr, iMT, unReplaceable, iVars, iEqnslst, ishared, iRepl, iAttributes) = (rest, ilast, exp, optExp, globalnegate, derReplaceState, mark, simpleeqnsarr.clone(), iMT.clone(), unReplaceable, vars, eqnslst, shared, repl, vsattr); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn traverseAliasTree1(
    mut sc: &SimpleContainer,
    mut r: i32,
    mut ilast: i32,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut optExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut globalnegated: bool,
    mut derReplaceState: Option<metamodelica::Ref<DAE::Exp>>,
    mut mark: i32,
    mut simpleeqnsarr: metamodelica::Array<SimpleContainer>,
    mut iMT: metamodelica::Array<metamodelica::List<i32>>,
    mut unReplaceable: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut iVars: BackendDAE::Variables,
    mut iEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut iRepl: BackendVarTransform::VariableReplacements,
    mut iAttributes: VarSetAttributes,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::Ref<BackendDAE::Shared>,
    BackendVarTransform::VariableReplacements,
    VarSetAttributes,
)> {
    let mut oVars: BackendDAE::Variables;
    let mut oEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut oRepl: BackendVarTransform::VariableReplacements;
    let mut oAttributes: VarSetAttributes;
    (oVars, oEqnslst, oshared, oRepl, oAttributes) = (::match_deref::match_deref! { match &(sc) {
        SimpleContainer::ALIAS { cr1: _, negatedCr1, i1, cr2: _, negatedCr2, i2, eqnAttributes: (source, eqAttr), visited: _ } => {
            let mut i: i32;
            let mut rows: metamodelica::List<i32>;
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut vars: BackendDAE::Variables;
            let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut replaceable_: bool;
            let mut globalnegated1: bool;
            let mut replaceble1: bool;
            let mut negated: bool;
            let mut crexp: metamodelica::Ref<DAE::Exp>;
            let mut exp1: metamodelica::Ref<DAE::Exp>;
            let mut derReplacement: Option<metamodelica::Ref<DAE::Exp>>;
            let mut vsattr: VarSetAttributes;
            let mut source = (*source).clone();
            i = if (intEq(i1.clone(), ilast)) {i2.clone()} else {i1.clone()};
            negated = boolOr(negatedCr2.clone(), negatedCr1.clone());
            let __arc2 = BackendVariable::getVarAt(&iVars, i)?;
            let __pa1 = (__arc2).clone();
            let BackendDAE::VAR { varName: __pa0, .. } = &*__arc2;
            cr = metamodelica::Own::own(__pa0);
            v = metamodelica::Own::own(__pa1);
            (replaceable_, replaceble1) = replaceableAlias(&v, unReplaceable);
            crexp = Expression::crefExp(cr)?;
            globalnegated1 = if (negated) {!(globalnegated)} else {globalnegated};
            exp1 = negateExpression(globalnegated1, exp.clone(), exp.clone(), &(literal!(" ALIAS_1 ")))?;
            derReplacement = if (globalnegated1) {negateOptExp(derReplaceState.clone())?} else {derReplaceState.clone()};
            source = if (replaceable_) {addSubstitutionOption(optExp, crexp.clone(), source.clone())?} else {source.clone()};
            (vars, eqnslst, shared, repl) = handleSetVar(replaceable_ && replaceble1, derReplacement, v.clone(), i, &((source.clone(), eqAttr.clone())), exp1, iMT.clone(), iVars, iEqnslst, ishared, iRepl)?;
            vsattr = if (replaceable_ && replaceble1) {addVarSetAttributes(&v, globalnegated1, mark, simpleeqnsarr.clone(), &iAttributes)?} else {iAttributes};
            crexp = negateExpression(negated, crexp.clone(), crexp, &(literal!(" ALIAS_2 ")))?;
            rows = List::removeOnTrue(r, &fnptr!(intEq, i32, i32), ({let __elt = (*metamodelica::index_checked(&iMT.borrow(), i)?).clone(); __elt}))?;
            metamodelica::arrayUpdate(iMT.clone(), i, metamodelica::nil())?;
            (vars, eqnslst, shared, repl, vsattr) = traverseAliasTree(&rows, i, &exp, Some(crexp), globalnegated1, derReplaceState, mark, simpleeqnsarr.clone(), iMT.clone(), unReplaceable, vars, eqnslst, shared, repl, vsattr)?;
            (vars, eqnslst, shared, repl, vsattr)
        },
        SimpleContainer::PARAMETERALIAS { unknowncr: cr1, negatedCr1, i1, paramcr: cr2, negatedCr2, i2: _, eqnAttributes: (source, _), visited: _ } => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut negated: bool;
            let mut crexp: metamodelica::Ref<DAE::Exp>;
            let mut lhs: ArcStr;
            let mut rhs: ArcStr;
            cr = if (intEq(i1.clone(), ilast)) {cr2.clone()} else {cr1.clone()};
            negated = boolOr(negatedCr1.clone(), negatedCr2.clone());
            crexp = Expression::crefExp(cr)?;
            crexp = negateExpression(negated, crexp.clone(), crexp, &(literal!(" PARAMETERLAIAS ")))?;
            lhs = ExpressionBasics::printExpStr(exp)?;
            rhs = ExpressionBasics::printExpStr(crexp)?;
            Error::addSourceMessage(&(Error::EQ_WITHOUT_TIME_DEP_VARS.clone()), list![lhs, rhs], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
            return Err("fail")
        },
        SimpleContainer::TIMEALIAS { eqnAttributes: (source, _), .. } => {
            let mut rhs: ArcStr;
            rhs = ExpressionBasics::printExpStr(exp)?;
            Error::addSourceMessage(&(Error::EQ_WITHOUT_TIME_DEP_VARS.clone()), list![literal!("time"), rhs], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
            return Err("fail")
        },
        SimpleContainer::TIMEINDEPENTVAR { exp: exp1, eqnAttributes: (source, _), .. } => {
            let mut lhs: ArcStr;
            let mut rhs: ArcStr;
            lhs = ExpressionBasics::printExpStr(exp)?;
            rhs = ExpressionBasics::printExpStr(exp1.clone())?;
            Error::addSourceMessage(&(Error::EQ_WITHOUT_TIME_DEP_VARS.clone()), list![lhs, rhs], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oVars, oEqnslst, oshared, oRepl, oAttributes))
}

fn negateOptExp(mut iExp: Option<metamodelica::Ref<DAE::Exp>>) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut oExp: Option<metamodelica::Ref<DAE::Exp>>;
    oExp = (::match_deref::match_deref! { match &(iExp.clone()) {
        Some(e) => {
            let mut e = (*e).clone();
            e = negateExpression(true, e.clone(), e.clone(), &(literal!(" in negateOptExp ")))?;
            Some(e.clone())
        },
        _ => {
            iExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oExp)
}

fn addSubstitutionOption(
    mut optExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    let mut e: metamodelica::Ref<DAE::Exp>;
    if (optExp).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(optExp) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        source = ElementSource::addSymbolicTransformationSubstitution(true, source, exp, e)?;
    }
    Ok(source)
}

fn addVarSetAttributes(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
    mut negate: bool,
    mut mark: i32,
    mut simpleeqnsarr: metamodelica::Array<SimpleContainer>,
    mut iAttributes: &VarSetAttributes,
) -> Result<VarSetAttributes> {
    let mut oAttributes: VarSetAttributes;
    let mut fixed: bool;
    let mut fixedset: bool;
    let mut start: Option<metamodelica::Ref<DAE::Exp>>;
    let mut origin: Option<DAE::StartOrigin>;
    let mut nominalset: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>;
    let mut minmaxset: (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>);
    let mut startvalues: StartValues;
    (fixedset, startvalues, nominalset, minmaxset) = iAttributes.clone();
    fixed = BackendVariable::varFixed(inVar);
    start = BackendVariable::varStartValueOption(inVar);
    origin = BackendVariable::varStartOrigin(inVar)?;
    (fixedset, startvalues) = addStartValue(
        fixed,
        fixedset,
        BackendVariable::varCref(inVar),
        start,
        origin,
        negate,
        mark,
        simpleeqnsarr.clone(),
        startvalues,
    )?;
    nominalset = addNominalValue(inVar, nominalset);
    minmaxset = addMinMaxAttribute(inVar, negate, mark, simpleeqnsarr.clone(), minmaxset)?;
    oAttributes = (fixedset, startvalues, nominalset, minmaxset);
    Ok(oAttributes)
}

fn addStartValue(
    mut fixed: bool,
    mut fixedset: bool,
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut start: Option<metamodelica::Ref<DAE::Exp>>,
    mut origin: Option<DAE::StartOrigin>,
    mut negate: bool,
    mut mark: i32,
    mut simpleeqnsarr: metamodelica::Array<SimpleContainer>,
    mut iStartvalues: StartValues,
) -> Result<(bool, StartValues)> {
    let mut oFixed: bool;
    let mut oStartvalues: StartValues;
    (oFixed, oStartvalues) = 'mc: {
        let __mc_input = (fixed, fixedset, &start, &iStartvalues);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, true, _, _) => {
                    Ok((fixedset, iStartvalues.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, false, None, _) => {
                    Ok((true, (origin.clone(), list![(start.clone(), cr.clone())])))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, false, Some(startexp), _) => {
                    let mut startexp = (*startexp).clone();
                    startexp = negateExpression(negate, startexp.clone(), startexp.clone(), &(literal!(" start_1 ")))?;
                    Ok((true, (origin.clone(), list![(Some(startexp.clone()), cr.clone())])))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, None, (setorigin, startvalues)) => {
                    let mut cmp: i32;
                    let mut setorigin = (*setorigin).clone();
                    let mut startvalues = (*startvalues).clone();
                    cmp = BackendVariable::startOriginCompare(origin.clone(), setorigin.clone())?;
                    if cmp < 0 {
                        setorigin = origin.clone();
                        startvalues = if (fixed) {list![(start.clone(), cr.clone())]} else {metamodelica::nil()};
                    } else if cmp == 0 && fixed {
                        startvalues = metamodelica::cons((start.clone(), cr.clone()), startvalues.clone());
                    }
                    Ok((fixedset, (setorigin.clone(), startvalues.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(startexp), (setorigin, startvalues)) => {
                    let mut cmp: i32;
                    let mut startexp = (*startexp).clone();
                    let mut setorigin = (*setorigin).clone();
                    let mut startvalues = (*startvalues).clone();
                    startexp = negateExpression(negate, startexp.clone(), startexp.clone(), &(literal!(" start_2 ")))?;
                    cmp = BackendVariable::startOriginCompare(origin.clone(), setorigin.clone())?;
                    if cmp < 0 {
                        setorigin = origin.clone();
                        startvalues = list![(Some(startexp.clone()), cr.clone())];
                    } else if cmp == 0 {
                        startvalues = metamodelica::cons((Some(startexp.clone()), cr.clone()), startvalues.clone());
                    }
                    Ok((fixedset, (setorigin.clone(), startvalues.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("RemoveSimpleEquations.addStartValue failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((oFixed, oStartvalues))
}

fn mergeStartFixedAttributes(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut fixed: bool,
    mut startvalues: &StartValues,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, bool)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut warnAliasConflicts: bool = false;
    outVar = 'mc: {
        let __mc_input = (fixed, startvalues, &**ishared);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, Deref @ metamodelica::ListNode::Nil), _) => {
                    Ok(inVar.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, (origin, Deref @ metamodelica::ListNode::Cons { head: (start, _), tail: Deref @ metamodelica::ListNode::Nil }), _) => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    v = BackendVariable::setVarFixed(inVar.clone(), true)?;
                    v = BackendVariable::setVarStartValueOption(v.clone(), start.clone())?;
                    Ok(BackendVariable::setVarStartOrigin(v.clone(), origin.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, (origin, Deref @ metamodelica::ListNode::Cons { head: (start, cr), tail: values }), Deref @ BackendDAE::Shared { globalKnownVars, .. }) => {
                    let mut start1: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut start = (*start).clone();
                    let mut warnAliasConflicts: bool = warnAliasConflicts.clone();
                    v = BackendVariable::setVarFixed(inVar.clone(), true)?;
                    start1 = optExpReplaceCrefWithBindExp(start.clone(), metamodelica::AsArg::as_arg(&globalKnownVars))?;
                    (_, start, _) = equalNonFreeStartValues(values.clone(), metamodelica::AsArg::as_arg(&globalKnownVars), (start1.clone(), start.clone(), cr.clone()))?;
                    warnAliasConflicts = !(Flags::isSet(Flags::ALIAS_CONFLICTS.clone())?);
                    v = BackendVariable::setVarStartValueOption(v.clone(), start.clone())?;
                    Ok((BackendVariable::setVarStartOrigin(v.clone(), origin.clone())?, warnAliasConflicts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            warnAliasConflicts = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, (_, values), Deref @ BackendDAE::Shared { .. }) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut startExp: metamodelica::Ref<DAE::Exp>;
                    let mut zerofreevalues: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>;
                    let mut r#str: ArcStr;
                    if !(Flags::isSet(Flags::ALIAS_CONFLICTS.clone())?) {
                        Error::addMessage(Error::CONFLICTING_ALIAS_SET.clone(), metamodelica::nil())?;
                    } else {
                        zerofreevalues = List::fold(metamodelica::AsArg::as_arg(&values), &move |__a0: (Option<metamodelica::Ref<DAE::Exp>>, metamodelica::Ref<DAE::ComponentRef>), __a1: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getZeroFreeValues(&__a0, __a1)) }, metamodelica::nil())?;
                        r#str = literal!("Conflicting start values for fixed states:\n");
                        for mut value in &*zerofreevalues {
                            (startExp, cr) = value.clone();
                            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(" * Candidate: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&cr)?); __mm_s.push_str(&*literal!("(start = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(startExp.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) };
                        }
                        Error::addCompilerError(r#str.clone())?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, (origin, Deref @ metamodelica::ListNode::Cons { head: (start, _), tail: Deref @ metamodelica::ListNode::Nil }), _) => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    v = BackendVariable::setVarStartValueOption(inVar.clone(), start.clone())?;
                    Ok(BackendVariable::setVarStartOrigin(v.clone(), origin.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, (origin, Deref @ metamodelica::ListNode::Cons { head: (start, cr), tail: values }), Deref @ BackendDAE::Shared { globalKnownVars, .. }) => {
                    let mut start1: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut start = (*start).clone();
                    start1 = optExpReplaceCrefWithBindExp(start.clone(), metamodelica::AsArg::as_arg(&globalKnownVars))?;
                    (_, start, _) = equalFreeStartValues(values.clone(), metamodelica::AsArg::as_arg(&globalKnownVars), (start1.clone(), start.clone(), cr.clone()))?;
                    v = BackendVariable::setVarStartValueOption(inVar.clone(), start.clone())?;
                    Ok(BackendVariable::setVarStartOrigin(v.clone(), origin.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, (origin, values), Deref @ BackendDAE::Shared { globalKnownVars, .. }) => {
                    let mut zerofreevalues: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>;
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut warnAliasConflicts: bool = warnAliasConflicts.clone();
                    zerofreevalues = List::fold(metamodelica::AsArg::as_arg(&values), &move |__a0: (Option<metamodelica::Ref<DAE::Exp>>, metamodelica::Ref<DAE::ComponentRef>), __a1: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getZeroFreeValues(&__a0, __a1)) }, metamodelica::nil())?;
                    (v, warnAliasConflicts) = selectFreeValue(zerofreevalues.clone(), inVar.clone(), metamodelica::AsArg::as_arg(&globalKnownVars))?;
                    if !((zerofreevalues).is_empty()) {
                        v = BackendVariable::setVarStartOrigin(v.clone(), origin.clone())?;
                    }
                    Ok((v.clone(), warnAliasConflicts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            warnAliasConflicts = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVar, warnAliasConflicts))
}

fn addNominalValue(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
    mut iNominal: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>,
) -> metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)> {
    let mut oNominal: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>;
    let mut nominal: metamodelica::Ref<DAE::Exp>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    match '__try0: {
        nominal = unwrap_break_err!(BackendVariable::varNominalValue(inVar), '__try0);
        cr = BackendVariable::varCref(inVar);
        oNominal = metamodelica::cons((nominal.clone(), cr.clone()), iNominal.clone());
        Ok::<_, &'static str>((oNominal.clone(),))
    } {
        Ok((__try0_o0,)) => {
            oNominal = __try0_o0;
        }
        Err(_) => {
            oNominal = iNominal.clone();
        }
    }
    oNominal
}

fn mergeNominalAttribute(
    mut nominalList: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>,
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut globalKnownVars: &BackendDAE::Variables,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, bool)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut warnAliasConflicts: bool = false;
    outVar = 'mc: {
        let __mc_input = &*nominalList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inVar.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut allExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    allExp = List::map(nominalList.clone(), &fnptr!(Util::tuple21, _))?;
                    let __pa0 = ::match_deref::match_deref! { match &(List::uniqueOnTrue(&allExp, &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>| ExpressionBasics::expEqual(&__a0, __a1))?) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    Ok(BackendVariable::setVarNominalValue(inVar.clone(), e.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut warnAliasConflicts: bool = warnAliasConflicts.clone();
                    warnAliasConflicts = !(Flags::isSet(Flags::ALIAS_CONFLICTS.clone())?);
                    Ok((selectFreeValue1(nominalList.clone(), literal!("Alias set with conflicting nominal values\n"), &(literal!("nominal")), &BackendVariable::setVarNominalValue, inVar.clone(), globalKnownVars)?, warnAliasConflicts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            warnAliasConflicts = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVar, warnAliasConflicts))
}

fn addMinMaxAttribute(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
    mut negate: bool,
    mut mark: i32,
    mut simpleeqnsarr: metamodelica::Array<SimpleContainer>,
    mut iMinMax: (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>),
) -> Result<(Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>)> {
    let mut oMinMax: (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>);
    let mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut ominmax: metamodelica::List<Option<metamodelica::Ref<DAE::Exp>>>;
    let __arc1 = &(*inVar);
    let BackendDAE::VAR { values: __pa0, .. } = &**__arc1;
    attr = metamodelica::Own::own(__pa0);
    ominmax = DAEUtil::getMinMax(attr);
    oMinMax = mergeMinMax(negate, &ominmax, iMinMax, mark, simpleeqnsarr.clone())?;
    Ok(oMinMax)
}

fn mergeMinMax(
    mut negate: bool,
    mut ominmax: &metamodelica::List<Option<metamodelica::Ref<DAE::Exp>>>,
    mut ominmax1: (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>),
    mut mark: i32,
    mut simpleeqnsarr: metamodelica::Array<SimpleContainer>,
) -> Result<(Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>)> {
    let mut outMinMax: (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>);
    outMinMax = (::match_deref::match_deref! { match &((negate, &**ominmax)) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            ominmax1
        },
        (false, Deref @ metamodelica::ListNode::Cons { head: omin, tail: Deref @ metamodelica::ListNode::Cons { head: omax, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut minMax: (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>);
            minMax = mergeMinMax1((omin.clone(), omax.clone()), ominmax1)?;
            checkMinMax(&minMax, mark, simpleeqnsarr.clone());
            minMax
        },
        (true, Deref @ metamodelica::ListNode::Cons { head: None, tail: Deref @ metamodelica::ListNode::Cons { head: None, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            ominmax1
        },
        (true, Deref @ metamodelica::ListNode::Cons { head: Some(min), tail: Deref @ metamodelica::ListNode::Cons { head: Some(max), tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut min1: metamodelica::Ref<DAE::Exp>;
            let mut max1: metamodelica::Ref<DAE::Exp>;
            let mut minMax: (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>);
            min1 = negateExpression(true, min.clone(), min.clone(), &(literal!(" min_1 ")))?;
            max1 = negateExpression(true, max.clone(), max.clone(), &(literal!(" max_1 ")))?;
            minMax = mergeMinMax1((Some(max1), Some(min1)), ominmax1)?;
            checkMinMax(&minMax, mark, simpleeqnsarr.clone());
            minMax
        },
        (true, Deref @ metamodelica::ListNode::Cons { head: None, tail: Deref @ metamodelica::ListNode::Cons { head: Some(max), tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut max1: metamodelica::Ref<DAE::Exp>;
            let mut minMax: (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>);
            max1 = negateExpression(true, max.clone(), max.clone(), &(literal!(" max_2 ")))?;
            minMax = mergeMinMax1((Some(max1), None), ominmax1)?;
            checkMinMax(&minMax, mark, simpleeqnsarr.clone());
            minMax
        },
        (true, Deref @ metamodelica::ListNode::Cons { head: Some(min), tail: Deref @ metamodelica::ListNode::Cons { head: None, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut min1: metamodelica::Ref<DAE::Exp>;
            let mut minMax: (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>);
            min1 = negateExpression(true, min.clone(), min.clone(), &(literal!(" min_2 ")))?;
            minMax = mergeMinMax1((None, Some(min1)), ominmax1)?;
            checkMinMax(&minMax, mark, simpleeqnsarr.clone());
            minMax
        },
        _ => {
            metamodelica::print(literal!("RemoveSimpleEquations.mergeMinMax failed!\n"));
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMinMax)
}

fn mergeMinMax1(
    mut ominmax: (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>),
    mut ominmax1: (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>),
) -> Result<(Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>)> {
    let mut minMax: (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>);
    let mut omin: Option<metamodelica::Ref<DAE::Exp>>;
    let mut omin1: Option<metamodelica::Ref<DAE::Exp>>;
    let mut omin2: Option<metamodelica::Ref<DAE::Exp>>;
    let mut omax: Option<metamodelica::Ref<DAE::Exp>>;
    let mut omax1: Option<metamodelica::Ref<DAE::Exp>>;
    let mut omax2: Option<metamodelica::Ref<DAE::Exp>>;
    (omin, omax) = ominmax.clone();
    (omin1, omax1) = ominmax1.clone();
    omin2 = Expression::expOptMaxScalar(omin.clone(), omin1.clone())?;
    omax2 = Expression::expOptMinScalar(omax.clone(), omax1.clone())?;
    if (match (&(omin2), &(omin)) {
        (None, None) => true,
        (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
        _ => false,
    }) && (match (&(omax2), &(omax)) {
        (None, None) => true,
        (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
        _ => false,
    }) {
        minMax = ominmax;
    } else if (match (&(omin2), &(omin1)) {
        (None, None) => true,
        (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
        _ => false,
    }) && (match (&(omax2), &(omax1)) {
        (None, None) => true,
        (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
        _ => false,
    }) {
        minMax = ominmax1;
    } else {
        minMax = (omin2, omax2);
    }
    Ok(minMax)
}

fn checkMinMax(
    mut minmax: &(Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>),
    mut mark: i32,
    mut simpleeqnsarr: metamodelica::Array<SimpleContainer>,
) -> () {
    let () = 'mc: {
        let __mc_input = minmax;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(min), Some(max)) => {
                    let mut s: ArcStr;
                    let mut s4: ArcStr;
                    let mut s5: ArcStr;
                    let mut rmin: metamodelica::Real;
                    let mut rmax: metamodelica::Real;
                    rmin = Expression::toReal(metamodelica::AsArg::as_arg(&min))?;
                    rmax = Expression::toReal(metamodelica::AsArg::as_arg(&max))?;
                    let true = (realGt(rmin, rmax)) else { return Err("pattern mismatch") };
                    s4 = ExpressionBasics::printExpStr(min.clone())?;
                    s5 = ExpressionBasics::printExpStr(max.clone())?;
                    s = stringAppendList(list![literal!("Alias variables with invalid limits min "), s4.clone(), literal!(" > max "), s5.clone()]);
                    Error::addMessage(Error::COMPILER_WARNING.clone(), list![s.clone()])?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn handleVarSetAttributes(
    mut inAttributes: &VarSetAttributes,
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVars: BackendDAE::Variables,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(BackendDAE::Variables, bool)> {
    let mut outVars: BackendDAE::Variables;
    let mut warnAliasConflicts: bool = false;
    outVars = ({
        let mut b1: bool = false;
        let mut b2: bool = false;
        let mut v: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
        'mc: {
            let __mc_input = (inAttributes, &**inShared);
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    ((fixedset, startvalues, nominalset, minmaxset), Deref @ BackendDAE::Shared { globalKnownVars, .. }) => {
                        let mut isdiscrete: bool;
                        let mut vars: BackendDAE::Variables;
                        let mut min: Option<metamodelica::Ref<DAE::Exp>>;
                        let mut max: Option<metamodelica::Ref<DAE::Exp>>;
                        let mut warnAliasConflicts: bool = warnAliasConflicts.clone();
                        isdiscrete = BackendVariable::isVarDiscrete(&inVar);
                        if !(isdiscrete) {
                            (v, b1) = mergeStartFixedAttributes(inVar.clone(), fixedset.clone(), &(startvalues.clone()), inShared)?;
                        }
                        (v, b2) = mergeNominalAttribute(nominalset.clone(), v.clone(), metamodelica::AsArg::as_arg(&globalKnownVars))?;
                        (min, max) = minmaxset.clone();
                        if (min).is_some() {
                            min = Some((ExpressionSimplify::simplify(min.clone().ok_or("pattern mismatch")?)?).0);
                        }
                        if (max).is_some() {
                            max = Some((ExpressionSimplify::simplify(max.clone().ok_or("pattern mismatch")?)?).0);
                        }
                        v = BackendVariable::setVarMinMax(v.clone(), min.clone(), max.clone())?;
                        vars = BackendVariable::addVar(v.clone(), inVars.clone())?;
                        warnAliasConflicts = b1 || b2;
                        Ok((vars.clone(), warnAliasConflicts.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                warnAliasConflicts = __wb0;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        metamodelica::print(literal!("RemoveSimpleEquations.handleVarSetAttributes failed!\n"));
                        Ok(return Err("fail"))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        }
    });
    Ok((outVars, warnAliasConflicts))
}

fn optExpReplaceCrefWithBindExp(
    mut iOExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut globalKnownVars: &BackendDAE::Variables,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut oOExp: Option<metamodelica::Ref<DAE::Exp>>;
    oOExp = (::match_deref::match_deref! { match &(iOExp.clone()) {
        Some(e) => {
            let mut b: bool;
            let mut e = (*e).clone();
            (e, b) = replaceCrefWithBindExp(e.clone(), globalKnownVars)?;
            (e, _) = ExpressionSimplify::condsimplify(b, e.clone())?;
            Some(e.clone())
        },
        _ => {
            iOExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oOExp)
}

fn equalNonFreeStartValues<'__b>(
    mut iValues: metamodelica::List<(
        Option<metamodelica::Ref<DAE::Exp>>,
        metamodelica::Ref<DAE::ComponentRef>,
    )>,
    mut globalKnownVars: &'__b BackendDAE::Variables,
    mut iValue: (
        Option<metamodelica::Ref<DAE::Exp>>,
        Option<metamodelica::Ref<DAE::Exp>>,
        metamodelica::Ref<DAE::ComponentRef>,
    ),
) -> Result<(
    Option<metamodelica::Ref<DAE::Exp>>,
    Option<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<DAE::ComponentRef>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((iValues, iValue.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(iValue)
            },
            (Deref @ metamodelica::ListNode::Cons { head: (None, _), tail: values }, _) => {
                { (iValues, globalKnownVars, iValue) = (values.clone(), globalKnownVars, iValue); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: (None, _), tail: values }, (None, _, _)) => {
                { (iValues, globalKnownVars, iValue) = (values.clone(), globalKnownVars, iValue); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: (None, cr), tail: values }, (Some(e2), _, _)) if (Expression::isZero(metamodelica::AsArg::as_arg(&e2))?) => {
                { (iValues, globalKnownVars, iValue) = (values.clone(), globalKnownVars, (None, None, cr.clone())); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: (Some(e), _), tail: values }, (Some(e2), _, _)) => {
                let mut e1: metamodelica::Ref<DAE::Exp>;
                let mut b: bool;
                (e1, b) = replaceCrefWithBindExp(e.clone(), globalKnownVars)?;
                (e1, _) = ExpressionSimplify::condsimplify(b, e1)?;
                let true = (ExpressionBasics::expEqual(&e1, e2.clone())?) else { return Err("pattern mismatch") };
                { (iValues, globalKnownVars, iValue) = (values.clone(), globalKnownVars, iValue); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn equalFreeStartValues<'__b>(
    mut iValues: metamodelica::List<(
        Option<metamodelica::Ref<DAE::Exp>>,
        metamodelica::Ref<DAE::ComponentRef>,
    )>,
    mut globalKnownVars: &'__b BackendDAE::Variables,
    mut iValue: (
        Option<metamodelica::Ref<DAE::Exp>>,
        Option<metamodelica::Ref<DAE::Exp>>,
        metamodelica::Ref<DAE::ComponentRef>,
    ),
) -> Result<(
    Option<metamodelica::Ref<DAE::Exp>>,
    Option<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<DAE::ComponentRef>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((iValues, iValue.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(iValue)
            },
            (Deref @ metamodelica::ListNode::Cons { head: (None, _), tail: values }, _) => {
                { (iValues, globalKnownVars, iValue) = (values.clone(), globalKnownVars, iValue); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: (Some(e), cr), tail: values }, (None, _, _)) => {
                let mut e1: metamodelica::Ref<DAE::Exp>;
                let mut b: bool;
                (e1, b) = replaceCrefWithBindExp(e.clone(), globalKnownVars)?;
                (e1, _) = ExpressionSimplify::condsimplify(b, e1)?;
                { (iValues, globalKnownVars, iValue) = (values.clone(), globalKnownVars, (Some(e1), Some(e.clone()), cr.clone())); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: (Some(e), _), tail: values }, (Some(e2), _, _)) => {
                let mut e1: metamodelica::Ref<DAE::Exp>;
                let mut b: bool;
                (e1, b) = replaceCrefWithBindExp(e.clone(), globalKnownVars)?;
                (e1, _) = ExpressionSimplify::condsimplify(b, e1)?;
                let true = (ExpressionBasics::expEqual(&e1, e2.clone())?) else { return Err("pattern mismatch") };
                { (iValues, globalKnownVars, iValue) = (values.clone(), globalKnownVars, iValue); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn replaceCrefWithBindExp(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut vars: &BackendDAE::Variables,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut replaced: bool;
    let mut replaced_crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<DAE::ComponentRef>>> =
        UnorderedSet::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                ComponentReferenceBasics::hashComponentRef(&__a0)
            })
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReferenceBasics::crefEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >),
            13,
        );
    (outExp, replaced) = Expression::traverseExpBottomUp(
        exp,
        &({
            let __pe_b2 = vars.clone();
            let __pe_b3 = replaced_crefs;
            move |__pe_a0, __pe_a1| {
                Ok(replaceCrefWithBindExp_traverser(
                    __pe_a0,
                    __pe_a1,
                    &__pe_b2,
                    __pe_b3.clone(),
                ))
            }
        }),
        false,
    )?;
    Ok((outExp, replaced))
}

fn replaceCrefWithBindExp_traverser(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut replaced: bool,
    mut vars: &BackendDAE::Variables,
    mut replacedCrefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<DAE::ComponentRef>>>,
) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outReplaced: bool;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut e: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    (outExp, outReplaced) = 'mc: {
        let __mc_input = &*exp;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
                    if !((!(UnorderedSet::contains(cr.clone(), replacedCrefs.clone())?))) { return Err("guard") }
                    let mut e: metamodelica::Ref<DAE::Exp> = e.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), vars)?) {
                        (Deref @ BackendDAE::Var { bindExp: Some(__pa0), .. }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    UnorderedSet::add(cr.clone(), replacedCrefs.clone())?;
                    (e, _) = Expression::traverseExpBottomUp(e.clone(), &({ let __pe_b2 = vars.clone(); let __pe_b3 = replacedCrefs.clone(); move |__pe_a0, __pe_a1| Ok(replaceCrefWithBindExp_traverser(__pe_a0, __pe_a1, &__pe_b2, __pe_b3.clone())) }), false)?;
                    Ok(((e.clone(), true), e.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            e = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { .. } => {
                    Ok((exp.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((exp.clone(), replaced))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outReplaced)
}

fn getZeroFreeValues(
    mut inTpl: &(
        Option<metamodelica::Ref<DAE::Exp>>,
        metamodelica::Ref<DAE::ComponentRef>,
    ),
    mut iAcc: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>,
) -> metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)> {
    let mut oAcc: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>;
    oAcc = (::match_deref::match_deref! { match &(inTpl) {
        (Some(e), cr) => {
            metamodelica::cons((e.clone(), cr.clone()), iAcc)
        },
        _ => {
            iAcc
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oAcc
}

fn selectFreeValue(
    mut iZeroFreeValues: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>,
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut globalKnownVars: &BackendDAE::Variables,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, bool)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut warnAliasConflicts: bool = false;
    outVar = (::match_deref::match_deref! { match &(iZeroFreeValues.clone()) {
        Deref @ metamodelica::ListNode::Nil => inVar,
        _ => {
            warnAliasConflicts = !(Flags::isSet(Flags::ALIAS_CONFLICTS.clone())?);
            selectFreeValue1(iZeroFreeValues, literal!("Alias set with conflicting start values\n"), &(literal!("start")), &BackendVariable::setVarStartValue, inVar, globalKnownVars)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVar, warnAliasConflicts))
}

fn selectNonZeroExpression<'__b>(
    mut iCandidates: &'__b metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match iCandidates {
            Deref @ metamodelica::ListNode::Cons { head: (e, cr), tail: Deref @ metamodelica::ListNode::Nil } => {
                return Ok((e.clone(), cr.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: (e, cr), tail: _ } if (!(Expression::isZero(metamodelica::AsArg::as_arg(&e))?)) => {
                return Ok((e.clone(), cr.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { iCandidates = rest; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn selectFreeValue1(
    mut iZeroFreeValues: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>,
    mut iStr: ArcStr,
    mut iAttributeName: &ArcStr,
    mut inFunc: &dyn ::std::ops::Fn(
        metamodelica::Ref<BackendDAE::Var>,
        metamodelica::Ref<DAE::Exp>,
    ) -> Result<metamodelica::Ref<BackendDAE::Var>>,
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut globalKnownVars: &BackendDAE::Variables,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    pub type FuncSetAttribute = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::Var>,
                metamodelica::Ref<DAE::Exp>,
            ) -> Result<metamodelica::Ref<BackendDAE::Var>>
            + 'static,
    >;

    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
    let mut crVar: metamodelica::Ref<DAE::ComponentRef>;
    let mut candidates: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)>;
    let mut b: bool;
    let mut s: ArcStr;
    let mut s2: ArcStr;
    if (iZeroFreeValues).is_empty() {
        outVar = inVar;
        return Ok(outVar);
    }
    crVar = BackendVariable::varCref(&inVar);
    candidates = iZeroFreeValues.clone().reverse();
    candidates = listAppend(
        ({
            let mut __acc: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)> =
                metamodelica::nil();
            for mut c in (candidates.clone()).into_iter().cloned() {
                if !(ComponentReferenceBasics::crefEqual(&crVar, &(Util::tuple22(c.clone())))?) {
                    continue;
                }
                let __x = c.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        ({
            let mut __acc: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)> =
                metamodelica::nil();
            for mut c in (candidates).into_iter().cloned() {
                if !(!(ComponentReferenceBasics::crefEqual(&crVar, &(Util::tuple22(c.clone())))?)) {
                    continue;
                }
                let __x = c.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    );
    (e, cr) = selectNonZeroExpression(&candidates)?;
    if Flags::isSet(Flags::ALIAS_CONFLICTS.clone())? {
        s = iStr;
        for mut c in &*iZeroFreeValues {
            (e1, cr1) = c.clone();
            (e1, b) = replaceCrefWithBindExp(e1, globalKnownVars)?;
            (e1, _) = ExpressionSimplify::condsimplify(b, e1)?;
            s2 = if (b) {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(" = "));
                    __mm_s.push_str(&*ExpressionBasics::printExpStr(e1)?);
                    ArcStr::from(__mm_s)
                }
            } else {
                literal!("")
            };
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!(" * Candidate: "));
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&cr1)?);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*iAttributeName);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(Util::tuple21(c.clone()))?);
                __mm_s.push_str(&*s2);
                __mm_s.push_str(&*literal!(")\n"));
                ArcStr::from(__mm_s)
            };
        }
        s = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*s);
            __mm_s.push_str(&*literal!("=> Select value from "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&cr)?);
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*iAttributeName);
            __mm_s.push_str(&*literal!(" = "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?);
            __mm_s.push_str(&*literal!(") for variable: "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&crVar)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
        Error::addMessage(Error::COMPILER_WARNING.clone(), list![s])?;
    }
    outVar = inFunc(inVar, e)?;
    Ok(outVar)
}

// =============================================================================
// functions to update equation system and shared
//
// =============================================================================
fn updateSystem(
    mut foundSimple: bool,
    mut iEqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iVars: BackendDAE::Variables,
    mut repl: BackendVarTransform::VariableReplacements,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    osyst = (::match_deref::match_deref! { match &((foundSimple, isyst.clone())) {
        (false, _) => {
            isyst
        },
        (true, syst @ Deref @ BackendDAE::EqSystem { .. }) => {
            let mut vars: BackendDAE::Variables;
            let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut syst = (*syst).clone();
            (vars, _) = BackendVariable::traverseBackendDAEVars(iVars, (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: (BackendDAE::Variables, BackendVarTransform::VariableReplacements)| updateVar(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (BackendDAE::Variables, BackendVarTransform::VariableReplacements)) -> Result<(metamodelica::Ref<BackendDAE::Var>, (BackendDAE::Variables, BackendVarTransform::VariableReplacements))> + 'static>), (BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), repl))?;
            eqns = BackendEquation::listEquation(&(iEqnslst.reverse()))?;
            assign_field!(
                syst.orderedEqs = eqns,
                syst.orderedVars = vars
            );
            BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(osyst)
}

fn updateVar(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: &(BackendDAE::Variables, BackendVarTransform::VariableReplacements),
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (BackendDAE::Variables, BackendVarTransform::VariableReplacements),
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut oTpl: (BackendDAE::Variables, BackendVarTransform::VariableReplacements);
    (outVar, oTpl) = 'mc: {
        let __mc_input = (inVar, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(cr), .. }, .. }, (vars, repl)) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut v = (*v).clone();
                    let mut vars = (*vars).clone();
                    e = BackendVarTransform::getReplacement(metamodelica::AsArg::as_arg(&repl), cr.clone())?;
                    v = updateStateOrder(&e, v.clone())?;
                    vars = BackendVariable::addVar(v.clone(), vars.clone())?;
                    Ok((v.clone(), (vars.clone(), repl.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, (vars, repl)) => {
                    let mut vars = (*vars).clone();
                    vars = BackendVariable::addVar(v.clone(), vars.clone())?;
                    Ok((v.clone(), (vars.clone(), repl.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVar, oTpl))
}

fn updateStateOrder(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = (match &**inExp {
        DAE::Exp::CREF { componentRef: cr, .. } => BackendVariable::setStateDerivative(inVar, Some(cr.clone()))?,
        _ => BackendVariable::setStateDerivative(inVar, None)?,
    });
    Ok(outVar)
}

fn removeSimpleEquationsShared(
    mut b: bool,
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut repl: BackendVarTransform::VariableReplacements,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match &((b, inDAE.clone())) {
        (false, _) => {
            inDAE
        },
        (true, Deref @ BackendDAE::BackendDAE { eqs: systs, shared: shared @ Deref @ BackendDAE::Shared { globalKnownVars, externalObjects, aliasVars, constraints: constraintsLst, classAttrs: clsAttrsLst, .. } }) => {
            let mut systs1: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
            let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut b1: bool;
            let mut shared = (*shared).clone();
            let mut aliasVars = (*aliasVars).clone();
            let mut constraintsLst = (*constraintsLst).clone();
            let mut clsAttrsLst = (*clsAttrsLst).clone();
            if Flags::isSet(Flags::DUMP_REPL.clone())? {
                BackendVarTransform::dumpReplacements(&repl)?;
                BackendVarTransform::dumpExtendReplacements(&repl)?;
                BackendVarTransform::dumpDerConstReplacements(&repl)?;
            }
            let (_, (_, __pa0)) = BackendVariable::traverseBackendDAEVarsWithUpdate(aliasVars.clone(), (std::sync::Arc::new(fnptr!(replaceAliasVarTraverser, metamodelica::Ref<BackendDAE::Var>, (BackendVarTransform::VariableReplacements, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (BackendVarTransform::VariableReplacements, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>)) -> Result<(metamodelica::Ref<BackendDAE::Var>, (BackendVarTransform::VariableReplacements, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>))> + 'static>), (repl.clone(), metamodelica::nil()))?;
            varlst = metamodelica::Own::own(__pa0);
            aliasVars = List::fold(&varlst, &fixAliasConstBindings, aliasVars.clone())?;
            assign_field!(shared.aliasVars = aliasVars.clone());
            BackendVariable::traverseBackendDAEVarsWithUpdate(globalKnownVars.clone(), (std::sync::Arc::new(fnptr!(replaceVarTraverser, metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements) -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements)> + 'static>), repl.clone())?;
            BackendVariable::traverseBackendDAEVarsWithUpdate(externalObjects.clone(), (std::sync::Arc::new(fnptr!(replaceVarTraverser, metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements) -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements)> + 'static>), repl.clone())?;
            (_, eqnslst, b1) = BackendEquation::traverseEquationArray(shared.initialEqs.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (BackendVarTransform::VariableReplacements, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, bool)| replaceEquationTraverser(__a0, &__a1), (repl.clone(), metamodelica::nil(), false))?;
            assign_field!(shared.initialEqs = if (b1) {BackendEquation::listEquation(&eqnslst)?} else {shared.initialEqs.clone()});
            (_, eqnslst, _) = BackendEquation::traverseEquationArray(shared.removedEqs.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (BackendVarTransform::VariableReplacements, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, bool)| replaceEquationTraverser(__a0, &__a1), (repl.clone(), metamodelica::nil(), false))?;
            eqnslst = List::select(eqnslst, (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Equation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendEquation::assertWithCondTrue(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Equation>) -> Result<bool> + 'static>))?;
            assign_field!(shared.removedEqs = BackendEquation::listEquation(&eqnslst)?);
            (constraintsLst, clsAttrsLst) = replaceOptimicaExps(metamodelica::AsArg::as_arg(&constraintsLst), metamodelica::AsArg::as_arg(&clsAttrsLst), &repl)?;
            assign_field!(
                shared.constraints = constraintsLst.clone(),
                shared.classAttrs = clsAttrsLst.clone()
            );
            systs1 = removeSimpleEquationsShared1(metamodelica::AsArg::as_arg(&systs), metamodelica::nil(), &repl, None, metamodelica::AsArg::as_arg(&aliasVars))?;
            metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: systs1, shared: shared.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outDAE)
}

fn fixAliasConstBindings(
    mut iAVar: metamodelica::Ref<BackendDAE::Var>,
    mut iAVars: BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    let mut oAVars: BackendDAE::Variables;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut avar: metamodelica::Ref<BackendDAE::Var>;
    cr = BackendVariable::varCref(&iAVar);
    e = BackendVariable::varBindExp(&iAVar)?;
    e = fixAliasConstBindings1(&cr, &e, &iAVars);
    avar = BackendVariable::setBindExp(iAVar, Some(e));
    oAVars = BackendVariable::addVar(avar, iAVars)?;
    Ok(oAVars)
}

fn fixAliasConstBindings1(
    mut iCr: &metamodelica::Ref<DAE::ComponentRef>,
    mut iExp: &metamodelica::Ref<DAE::Exp>,
    mut iAVars: &BackendDAE::Variables,
) -> metamodelica::Ref<DAE::Exp> {
    let mut oExp: metamodelica::Ref<DAE::Exp>;
    oExp = 'mc: {
        let __mc_input = iAVars.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(Expression::extractCrefsFromExp(iExp.clone())?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            let __pa1 = ::match_deref::match_deref! { match &(BackendVariable::getVarSingle(&cr, iAVars)?) {
                (Deref @ BackendDAE::Var { bindExp: Some(__pa1), .. }, _) => __pa1.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa1);
            Ok(fixAliasConstBindings1(&cr, &e, iAVars))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(iExp.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oExp
}

fn replaceAliasVarTraverser(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: (
        BackendVarTransform::VariableReplacements,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    (
        BackendVarTransform::VariableReplacements,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (
        BackendVarTransform::VariableReplacements,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    );
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { bindExp: Some(e), .. }, (repl, varlst)) => {
                    let mut v1: metamodelica::Ref<BackendDAE::Var>;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    let mut varlst = (*varlst).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&repl), None)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa0);
                    b = Expression::isConstValue(&e1)?;
                    v1 = if (!(b)) {BackendVariable::setBindExp(v.clone(), Some(e1.clone()))} else {v.clone()};
                    varlst = List::consOnTrue(b, v1.clone(), varlst.clone());
                    Ok((v1.clone(), (repl.clone(), varlst.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outTpl)
}

fn replaceVarTraverser(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inRepl: BackendVarTransform::VariableReplacements,
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    BackendVarTransform::VariableReplacements,
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut repl: BackendVarTransform::VariableReplacements;
    (outVar, repl) = 'mc: {
        let __mc_input = (inVar.clone(), inRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { bindExp: Some(e), .. }, repl) => {
                    let mut v1: metamodelica::Ref<BackendDAE::Var>;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&repl), None)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa0);
                    v1 = BackendVariable::setBindExp(v.clone(), Some(e1.clone()));
                    Ok((v1.clone(), repl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inRepl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, repl)
}

fn removeSimpleEquationsShared1<'__b>(
    mut inSysts: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut inSysts1: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut repl: &'__b BackendVarTransform::VariableReplacements,
    mut statesetrepl: Option<BackendVarTransform::VariableReplacements>,
    mut aliasVars: &'__b BackendDAE::Variables,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inSysts {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inSysts1)
            },
            Deref @ metamodelica::ListNode::Cons { head: syst, tail: rest } => {
                let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut b: bool;
                let mut b1: bool;
                let mut stateSets: metamodelica::List<BackendDAE::StateSet>;
                let mut statesetrepl1: Option<BackendVarTransform::VariableReplacements>;
                let mut syst = (*syst).clone();
                (_, eqnslst, b) = BackendEquation::traverseEquationArray(syst.orderedEqs.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (BackendVarTransform::VariableReplacements, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, bool)| replaceEquationTraverser(__a0, &__a1), (repl.clone(), metamodelica::nil(), false))?;
                (stateSets, b1, statesetrepl1) = removeAliasVarsStateSets(syst.stateSets.clone(), statesetrepl, &syst.orderedVars, aliasVars, metamodelica::nil(), false)?;
                if b || b1 {
                    eqns = BackendEquation::listEquation(&(eqnslst.reverse()))?;
                    assign_field!(
                        syst.stateSets = stateSets,
                        syst.orderedEqs = eqns
                    );
                    syst = BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst));
                }
                (_, eqnslst, _) = BackendEquation::traverseEquationArray(syst.removedEqs.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (BackendVarTransform::VariableReplacements, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, bool)| replaceEquationTraverser(__a0, &__a1), (repl.clone(), metamodelica::nil(), false))?;
                eqnslst = List::select(eqnslst, (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Equation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendEquation::assertWithCondTrue(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Equation>) -> Result<bool> + 'static>))?;
                assign_field!(syst.removedEqs = BackendEquation::listEquation(&eqnslst)?);
                { (inSysts, inSysts1, repl, statesetrepl, aliasVars) = (rest, metamodelica::cons(syst.clone(), inSysts1), repl, statesetrepl1, aliasVars); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn removeAliasVarsStateSets<'__b>(
    mut iStateSets: metamodelica::List<BackendDAE::StateSet>,
    mut iStatesetrepl: Option<BackendVarTransform::VariableReplacements>,
    mut vars: &'__b BackendDAE::Variables,
    mut aliasVars: &'__b BackendDAE::Variables,
    mut iAcc: metamodelica::List<BackendDAE::StateSet>,
    mut inB: bool,
) -> Result<(
    metamodelica::List<BackendDAE::StateSet>,
    bool,
    Option<BackendVarTransform::VariableReplacements>,
)> {
    let mut oStateSets: metamodelica::List<BackendDAE::StateSet>;
    let mut outB: bool;
    let mut oStatesetrepl: Option<BackendVarTransform::VariableReplacements>;
    (oStateSets, outB, oStatesetrepl) = (::match_deref::match_deref! { match &(iStateSets) {
        Deref @ metamodelica::ListNode::Nil => {
            (iAcc.reverse(), inB, iStatesetrepl)
        },
        Deref @ metamodelica::ListNode::Cons { head: BackendDAE::StateSet { index, rang, state: states, crA, varA, statescandidates, ovars, eqns, oeqns, crJ, varJ, jacobian: jac }, tail: stateSets } => {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut b: bool;
            let mut b1: bool;
            let mut ovars = (*ovars).clone();
            let mut eqns = (*eqns).clone();
            let mut oeqns = (*oeqns).clone();
            let mut stateSets = (*stateSets).clone();
            repl = getAliasReplacements(iStatesetrepl, aliasVars.clone())?;
            hs = HashSet::emptyHashSet();
            hs = List::applyAndFold(metamodelica::AsArg::as_arg(&statescandidates), &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::varCref(&__a0)) }, hs)?;
            ovars = replaceOtherStateSetVars(metamodelica::AsArg::as_arg(&ovars), vars, aliasVars, &hs, &(metamodelica::nil()));
            (eqns, b) = BackendVarTransform::replaceEquations(eqns.clone(), &repl, Some((std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVarTransform::skipPreChangeEdgeOperator(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>)))?;
            (oeqns, b1) = BackendVarTransform::replaceEquations(oeqns.clone(), &repl, Some((std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVarTransform::skipPreChangeEdgeOperator(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>)))?;
            oeqns = List::fold(metamodelica::AsArg::as_arg(&oeqns), &fnptr!(removeEqualLshRshEqns, metamodelica::Ref<BackendDAE::Equation>, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>), metamodelica::nil())?;
            oeqns = oeqns.clone().reverse();
            (stateSets, b, oStatesetrepl) = removeAliasVarsStateSets(stateSets.clone(), Some(repl), vars, aliasVars, metamodelica::cons(BackendDAE::StateSet { index: index.clone(), rang: rang.clone(), state: states.clone(), crA: crA.clone(), varA: varA.clone(), statescandidates: statescandidates.clone(), ovars: ovars.clone(), eqns: eqns.clone(), oeqns: oeqns.clone(), crJ: crJ.clone(), varJ: varJ.clone(), jacobian: jac.clone() }, iAcc), b || b1)?;
            (stateSets.clone(), b, oStatesetrepl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oStateSets, outB, oStatesetrepl))
}

fn removeEqualLshRshEqns(
    mut iEqn: metamodelica::Ref<BackendDAE::Equation>,
    mut iEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> {
    let mut oEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    oEqns = 'mc: {
        let __mc_input = &*iEqn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: lhs, scalar: rhs, .. } => {
                    let mut b: bool;
                    b = ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&lhs), rhs.clone())?;
                    Ok(List::consOnTrue(!(b), iEqn.clone(), iEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: lhs, right: rhs, .. } => {
                    let mut b: bool;
                    b = ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&lhs), rhs.clone())?;
                    Ok(List::consOnTrue(!(b), iEqn.clone(), iEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: lhs, right: rhs, .. } => {
                    let mut b: bool;
                    b = ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&lhs), rhs.clone())?;
                    Ok(List::consOnTrue(!(b), iEqn.clone(), iEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::cons(iEqn.clone(), iEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oEqns
}

fn replaceOtherStateSetVars(
    mut iVarLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut vars: &BackendDAE::Variables,
    mut aliasVars: &BackendDAE::Variables,
    mut hs: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut iAcc: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> metamodelica::List<metamodelica::Ref<BackendDAE::Var>> {
    let mut oVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    oVarLst = 'mc: {
        let __mc_input = &**iVarLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(iAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: var, tail: varlst } => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    let mut var = (*var).clone();
                    let mut varlst = (*varlst).clone();
                    cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&var));
                    let false = (BaseHashSet::has(cr.clone(), hs)?) else { return Err("pattern mismatch") };
                    (var, _) = BackendVariable::getVarSingle(&cr, aliasVars)?;
                    exp = BackendVariable::varBindExp(metamodelica::AsArg::as_arg(&var))?;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extractCrefsFromExp(exp.clone())?) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr = metamodelica::Own::own(__pa0);
                    b = BaseHashSet::has(cr.clone(), hs)?;
                    (var, _) = BackendVariable::getVarSingle(&cr, vars)?;
                    varlst = List::consOnTrue(!(b), var.clone(), iAcc.clone());
                    Ok(replaceOtherStateSetVars(metamodelica::AsArg::as_arg(&varlst), vars, aliasVars, hs, metamodelica::AsArg::as_arg(&varlst)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: var, tail: varlst } => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&var));
                    let true = (BaseHashSet::has(cr.clone(), hs)?) else { return Err("pattern mismatch") };
                    Ok(replaceOtherStateSetVars(metamodelica::AsArg::as_arg(&varlst), vars, aliasVars, hs, iAcc))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: var, tail: varlst } => {
                    Ok(replaceOtherStateSetVars(metamodelica::AsArg::as_arg(&varlst), vars, aliasVars, hs, &(metamodelica::cons(var.clone(), iAcc.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oVarLst
}

fn replaceOptimicaExps(
    mut icontraints: &metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    mut iclassAttributes: &metamodelica::List<metamodelica::Ref<DAE::ClassAttributes>>,
    mut irepl: &BackendVarTransform::VariableReplacements,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    metamodelica::List<metamodelica::Ref<DAE::ClassAttributes>>,
)> {
    let mut ocontraints: metamodelica::List<metamodelica::Ref<DAE::Constraint>>;
    let mut oclassAttributes: metamodelica::List<metamodelica::Ref<DAE::ClassAttributes>>;
    (ocontraints, oclassAttributes) = (::match_deref::match_deref! { match (icontraints, iclassAttributes) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            (metamodelica::nil(), metamodelica::nil())
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ClassAttributes { objetiveE, objectiveIntegrandE, startTimeE, finalTimeE }, tail: restClassAtr }) => {
            let mut classAttributes: metamodelica::List<metamodelica::Ref<DAE::ClassAttributes>>;
            let mut objetiveE = (*objetiveE).clone();
            let mut objectiveIntegrandE = (*objectiveIntegrandE).clone();
            let mut startTimeE = (*startTimeE).clone();
            let mut finalTimeE = (*finalTimeE).clone();
            let __pa0 = ::match_deref::match_deref! { match &(replaceOptExprTraverser(&((objetiveE.clone(), (irepl.clone(), metamodelica::nil(), false))))?) {
                (_, (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, _)) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            objetiveE = metamodelica::Own::own(__pa0);
            let __pa2 = ::match_deref::match_deref! { match &(replaceOptExprTraverser(&((objectiveIntegrandE.clone(), (irepl.clone(), metamodelica::nil(), false))))?) {
                (_, (_, Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil }, _)) => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            objectiveIntegrandE = metamodelica::Own::own(__pa2);
            let __pa4 = ::match_deref::match_deref! { match &(replaceOptExprTraverser(&((startTimeE.clone(), (irepl.clone(), metamodelica::nil(), false))))?) {
                (_, (_, Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil }, _)) => __pa4.clone(),
                _ => return Err("pattern mismatch"),
            } };
            startTimeE = metamodelica::Own::own(__pa4);
            let __pa6 = ::match_deref::match_deref! { match &(replaceOptExprTraverser(&((finalTimeE.clone(), (irepl.clone(), metamodelica::nil(), false))))?) {
                (_, (_, Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Nil }, _)) => __pa6.clone(),
                _ => return Err("pattern mismatch"),
            } };
            finalTimeE = metamodelica::Own::own(__pa6);
            (_, classAttributes) = replaceOptimicaExps(&(metamodelica::nil()), restClassAtr, irepl)?;
            classAttributes = metamodelica::cons(metamodelica::Ref::new(DAE::ClassAttributes { objetiveE: objetiveE.clone(), objectiveIntegrandE: objectiveIntegrandE.clone(), startTimeE: startTimeE.clone(), finalTimeE: finalTimeE.clone() }), classAttributes);
            (metamodelica::nil(), classAttributes)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: constraintLstExps }, tail: rest }, _) => {
            let mut constraintLst: metamodelica::List<metamodelica::Ref<DAE::Constraint>>;
            let mut constraintLstExps = (*constraintLstExps).clone();
            constraintLstExps = replaceOptimicaContraints(metamodelica::AsArg::as_arg(&constraintLstExps), irepl)?;
            (constraintLst, _) = replaceOptimicaExps(rest, iclassAttributes, irepl)?;
            constraintLst = metamodelica::cons(metamodelica::Ref::new(DAE::Constraint::CONSTRAINT_EXPS { constraintLst: constraintLstExps.clone() }), constraintLst);
            (constraintLst, iclassAttributes.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((ocontraints, oclassAttributes))
}

fn replaceOptimicaContraints(
    mut icontraints: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut irepl: &BackendVarTransform::VariableReplacements,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut ocontraints: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    ocontraints = (::match_deref::match_deref! { match icontraints {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: rest } => {
            let mut constraintLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut e = (*e).clone();
            let __pa0 = ::match_deref::match_deref! { match &(replaceExprTraverser(&((e.clone(), (irepl.clone(), metamodelica::nil(), false))))) {
                (_, (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, _)) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            constraintLst = replaceOptimicaContraints(rest, irepl)?;
            constraintLst = metamodelica::cons(e.clone(), constraintLst);
            constraintLst
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ocontraints)
}

fn getAliasReplacements(
    mut iStatesetrepl: Option<BackendVarTransform::VariableReplacements>,
    mut aliasVars: BackendDAE::Variables,
) -> Result<BackendVarTransform::VariableReplacements> {
    let mut oStatesetrepl: BackendVarTransform::VariableReplacements;
    oStatesetrepl = (match iStatesetrepl {
        Some(mut repl) => repl,
        _ => {
            let mut repl: BackendVarTransform::VariableReplacements;
            repl = BackendVarTransform::emptyReplacementsSized(BackendVariable::varsSize(&aliasVars));
            repl = BackendVariable::traverseBackendDAEVars(
                aliasVars,
                (std::sync::Arc::new(getAliasVarReplacements)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<BackendDAE::Var>,
                                BackendVarTransform::VariableReplacements,
                            ) -> Result<(
                                metamodelica::Ref<BackendDAE::Var>,
                                BackendVarTransform::VariableReplacements,
                            )> + 'static,
                    >),
                repl,
            )?;
            repl
        }
    });
    Ok(oStatesetrepl)
}

fn getAliasVarReplacements(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inRepl: BackendVarTransform::VariableReplacements,
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    BackendVarTransform::VariableReplacements,
)> {
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    v = inVar;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(v.clone()) {
        Deref @ BackendDAE::Var { varName: __pa0, bindExp: Some(__pa1), .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cr = metamodelica::Own::own(__pa0);
    exp = metamodelica::Own::own(__pa1);
    repl = BackendVarTransform::addReplacement(
        inRepl,
        cr,
        exp,
        Some(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVarTransform::skipPreChangeEdgeOperator(&__a0))
            })
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>),
        ),
    )?;
    Ok((v, repl))
}

fn replaceEquationTraverser(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTpl: &(
        BackendVarTransform::VariableReplacements,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        bool,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    (
        BackendVarTransform::VariableReplacements,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        bool,
    ),
)> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut outTpl: (
        BackendVarTransform::VariableReplacements,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        bool,
    );
    (outEq, outTpl) = (::match_deref::match_deref! { match &(inTpl) {
        (repl, eqns, b) => {
            let mut e = inEq;
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut lhs: metamodelica::Ref<DAE::Exp>;
            let mut rhs: metamodelica::Ref<DAE::Exp>;
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut eqns1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut b1: bool;
            let mut eqns = (*eqns).clone();
            (eqns1, b1) = BackendVarTransform::replaceEquations(list![e.clone()], metamodelica::AsArg::as_arg(&repl), Some((std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVarTransform::skipPreChangeEdgeOperator(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>)))?;
            if BackendEquation::isInitialEquation(&e)? && BackendEquation::isEquation(&e) {
                eqn = (eqns1).head().cloned()?;
                lhs = BackendEquation::getEquationLHS(&eqn)?;
                rhs = BackendEquation::getEquationRHS(&eqn)?;
                res = Expression::createResidualExp(lhs, rhs)?;
                if Expression::isConst(res.clone())? {
                    if Expression::isZero(&res)? {
                        Error::addCompilerNotification({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following initial equation is redundant and consistent due to simplifications in RemoveSimpleEquations and therefore removed from the initialization problem: ")); __mm_s.push_str(&*BackendDump::equationString(&e)?); __mm_s.push_str(&*if (b1) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*BackendDump::equationString(&eqn)?); ArcStr::from(__mm_s) }} else {literal!("")}); ArcStr::from(__mm_s) })?;
                        eqns1 = metamodelica::nil();
                        b1 = true;
                    } else {
                        Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following initial equation is inconsistent due to simplifications in RemoveSimpleEquations and therefore removed from the initialization problem: ")); __mm_s.push_str(&*BackendDump::equationString(&e)?); __mm_s.push_str(&*if (b1) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*BackendDump::equationString(&eqn)?); ArcStr::from(__mm_s) }} else {literal!("")}); ArcStr::from(__mm_s) })?;
                        eqns1 = metamodelica::nil();
                        b1 = true;
                    }
                }
            }
            eqns = listAppend(eqns1, eqns.clone());
            (e, (repl.clone(), eqns.clone(), b.clone() || b1))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEq, outTpl))
}

fn replaceExprTraverser(
    mut inTpl: &(
        metamodelica::Ref<DAE::Exp>,
        (
            BackendVarTransform::VariableReplacements,
            metamodelica::List<metamodelica::Ref<DAE::Exp>>,
            bool,
        ),
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        BackendVarTransform::VariableReplacements,
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
        bool,
    ),
) {
    let mut outTpl: (
        metamodelica::Ref<DAE::Exp>,
        (
            BackendVarTransform::VariableReplacements,
            metamodelica::List<metamodelica::Ref<DAE::Exp>>,
            bool,
        ),
    );
    outTpl = (::match_deref::match_deref! { match &(inTpl) {
        (exp, (repl, exps, b)) => {
            let mut exp1: metamodelica::Ref<DAE::Exp>;
            let mut b1: bool;
            let mut exps = (*exps).clone();
            (exp1, b1) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&exp), metamodelica::AsArg::as_arg(&repl), Some((std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVarTransform::skipPreChangeEdgeOperator(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>)));
            exps = metamodelica::cons(exp1, exps.clone());
            (exp.clone(), (repl.clone(), exps.clone(), b.clone() || b1))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTpl
}

fn replaceOptExprTraverser(
    mut inTpl: &(
        Option<metamodelica::Ref<DAE::Exp>>,
        (
            BackendVarTransform::VariableReplacements,
            metamodelica::List<Option<metamodelica::Ref<DAE::Exp>>>,
            bool,
        ),
    ),
) -> Result<(
    Option<metamodelica::Ref<DAE::Exp>>,
    (
        BackendVarTransform::VariableReplacements,
        metamodelica::List<Option<metamodelica::Ref<DAE::Exp>>>,
        bool,
    ),
)> {
    let mut outTpl: (
        Option<metamodelica::Ref<DAE::Exp>>,
        (
            BackendVarTransform::VariableReplacements,
            metamodelica::List<Option<metamodelica::Ref<DAE::Exp>>>,
            bool,
        ),
    );
    outTpl = (::match_deref::match_deref! { match &(inTpl) {
        (None, (repl, exps, b)) => {
            let mut exps = (*exps).clone();
            exps = metamodelica::cons(None, exps.clone());
            (None, (repl.clone(), exps.clone(), b.clone()))
        },
        (expOpt @ Some(exp), (repl, exps, b)) => {
            let mut exp1: metamodelica::Ref<DAE::Exp>;
            let mut b1: bool;
            let mut exps = (*exps).clone();
            (exp1, b1) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&exp), metamodelica::AsArg::as_arg(&repl), Some((std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVarTransform::skipPreChangeEdgeOperator(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>)));
            if referenceEq(&*(&*exp1),&*(exp.clone())) {
                exps = metamodelica::cons(expOpt.clone(), exps.clone());
            } else {
                exps = metamodelica::cons(Some(exp1), exps.clone());
            }
            (expOpt.clone(), (repl.clone(), exps.clone(), b.clone() || b1))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outTpl)
}

// =============================================================================
// functions to find unReplaceable variables
//
// unReplaceable:
//   - variables with variable subscribts
//   - variables set in when-clauses
//   - variables used in pre
//   - statescandidates of statesets
//   - lhs of array assign statement, because there is a cref used and this is not replaceable_ with array of crefs
// =============================================================================
fn addUnreplaceableFromStateSets(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
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
    ),
)> {
    let mut outUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let __arc1 = &(*inDAE);
    let BackendDAE::DAE { eqs: __pa0, .. } = &**__arc1;
    systs = metamodelica::Own::own(__pa0);
    outUnreplaceable = List::fold(
        &systs,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        )| addUnreplaceableFromStateSetSystem(&__a0, __a1),
        inUnreplaceable,
    )?;
    Ok(outUnreplaceable)
}

fn addUnreplaceableFromStateSetSystem(
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
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
    ),
)> {
    let mut outUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    outUnreplaceable = (::match_deref::match_deref! { match isyst {
        Deref @ BackendDAE::EqSystem { stateSets: Deref @ metamodelica::ListNode::Nil, .. } => {
            inUnreplaceable
        },
        Deref @ BackendDAE::EqSystem { stateSets, .. } => {
            let mut unReplaceable: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            unReplaceable = List::fold(stateSets, &addUnreplaceableFromStateSet, inUnreplaceable)?;
            unReplaceable
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outUnreplaceable)
}

fn addUnreplaceableFromStateSet(
    mut iStateSet: BackendDAE::StateSet,
    mut inUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
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
    ),
)> {
    let mut outUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    let mut statevars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let BackendDAE::STATESET {
        statescandidates: __pa0,
        ..
    } = iStateSet;
    statevars = metamodelica::Own::own(__pa0);
    crlst = List::map(
        statevars,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
        },
    )?;
    crlst = List::map(crlst, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
        ComponentReferenceBasics::crefStripLastSubs(&__a0)
    })?;
    outUnreplaceable = List::fold(
        &crlst,
        &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1),
        inUnreplaceable,
    )?;
    Ok(outUnreplaceable)
}

fn addUnreplaceableFromWhens(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
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
    ),
)> {
    let mut outUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let __arc3 = &(*inDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __t2,
    } = &**__arc3;
    let __arc4 = __t2.clone();
    let BackendDAE::SHARED { initialEqs: __pa1, .. } = &*__arc4;
    systs = metamodelica::Own::own(__pa0);
    eqns = metamodelica::Own::own(__pa1);
    outUnreplaceable = List::fold(
        &systs,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        )| addUnreplaceableFromWhensSystem(&__a0, __a1),
        inUnreplaceable,
    )?;
    (_, outUnreplaceable) = BackendDAEUtil::traverseBackendDAEExpsEqns(
        eqns,
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(addUnreplaceableFromEqnsExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        )> + 'static,
                >),
            outUnreplaceable,
        ),
    )?;
    Ok(outUnreplaceable)
}

fn addUnreplaceableFromEqnsExp(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut hs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut ohs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    (outExp, ohs) = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. } => {
            (e, hs)
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut cr = (*cr).clone();
            cr = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&cr))?;
            ohs = BaseHashSet::add(cr.clone(), &hs)?;
            (e, ohs)
        },
        _ => {
            (e, hs)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, ohs))
}

fn addUnreplaceableFromWhensSystem(
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
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
    ),
)> {
    let mut outUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    eqns = BackendEquation::getEqnsFromEqSystem(isyst);
    outUnreplaceable = BackendEquation::traverseEquationArray(eqns, &addUnreplaceableFromWhenEqn, inUnreplaceable)?;
    Ok(outUnreplaceable)
}

fn addUnreplaceableFromWhenEqn(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inHs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
)> {
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut hs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    (eqn, hs) = (::match_deref::match_deref! { match &((inEq.clone(), inHs.clone())) {
        (__esc_eqn @ Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: weqn, .. }, __esc_hs) => {
            eqn = (*__esc_eqn).clone();
            hs = (*__esc_hs).clone();
            hs = addUnreplaceableFromWhen(metamodelica::AsArg::as_arg(&weqn), &(hs.clone()))?;
            (eqn.clone(), hs.clone())
        },
        (__esc_eqn @ Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, __esc_hs) => {
            eqn = (*__esc_eqn).clone();
            hs = (*__esc_hs).clone();
            hs = List::fold(metamodelica::AsArg::as_arg(&stmts), &move |__a0: metamodelica::Ref<DAE::Statement>, __a1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))| -> metamodelica::Result<_> { ::std::result::Result::Ok(addUnreplaceableFromWhenStmt(&__a0, &__a1)) }, hs.clone())?;
            (eqn.clone(), hs.clone())
        },
        _ => {
            (inEq, inHs)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqn, hs))
}

fn addUnreplaceableFromWhenStmt(
    mut inStmt: &metamodelica::Ref<DAE::Statement>,
    mut inHS: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> (
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
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
    ),
) {
    let mut outHS: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    outHS = 'mc: {
        let __mc_input = &**inStmt;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHEN { statementLst: stmts, elseWhen: None, .. } => {
                    let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    hs = List::fold(metamodelica::AsArg::as_arg(&stmts), &move |__a0: metamodelica::Ref<DAE::Statement>, __a1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))| addUnreplaceableFromStmt(&__a0, __a1), inHS.clone())?;
                    Ok(hs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHEN { statementLst: stmts, elseWhen: Some(stmt), .. } => {
                    let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    hs = List::fold(metamodelica::AsArg::as_arg(&stmts), &move |__a0: metamodelica::Ref<DAE::Statement>, __a1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))| addUnreplaceableFromStmt(&__a0, __a1), inHS.clone())?;
                    hs = addUnreplaceableFromWhenStmt(metamodelica::AsArg::as_arg(&stmt), &hs);
                    Ok(hs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. } => {
                    let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cr = (*cr).clone();
                    cr = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&cr))?;
                    hs = BaseHashSet::add(cr.clone(), inHS)?;
                    Ok(hs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. } => {
                    let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cr = (*cr).clone();
                    cr = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&cr))?;
                    hs = BaseHashSet::add(cr.clone(), inHS)?;
                    Ok(hs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inHS.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outHS
}

fn addUnreplaceableFromStmt(
    mut inStmt: &metamodelica::Ref<DAE::Statement>,
    mut inHS: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
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
    ),
)> {
    let mut outHS: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    outHS = (::match_deref::match_deref! { match inStmt {
        Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. } => {
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut cr = (*cr).clone();
            cr = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&cr))?;
            hs = BaseHashSet::add(cr.clone(), &inHS)?;
            hs
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst, .. } => {
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            crlst = List::flatten(List::map(expExpLst.clone(), &Expression::extractCrefsFromExp)?)?;
            crlst = List::map(crlst, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefStripLastSubs(&__a0))?;
            hs = List::fold(&crlst, &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), inHS)?;
            hs
        },
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. } => {
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut cr = (*cr).clone();
            cr = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&cr))?;
            hs = BaseHashSet::add(cr.clone(), &inHS)?;
            hs
        },
        _ => {
            inHS
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outHS)
}

fn addUnreplaceableFromWhen(
    mut inWEqn: &metamodelica::Ref<BackendDAE::WhenEquation>,
    mut iHs: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
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
    ),
)> {
    let mut oHs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    oHs = (match &**inWEqn {
        BackendDAE::WhenEquation {
            whenStmtLst,
            elsewhenPart: oweqn,
            ..
        } => {
            let mut weqn: metamodelica::Ref<BackendDAE::WhenEquation>;
            let mut hs: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                ),
                i32,
                i32,
                (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
            );
            hs = addUnreplaceableFromWhenOps(whenStmtLst, iHs.clone())?;
            if (oweqn).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(oweqn.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                weqn = metamodelica::Own::own(__pa0);
                hs = addUnreplaceableFromWhen(&weqn, &hs)?;
            }
            hs
        }
    });
    Ok(oHs)
}

fn addUnreplaceableFromWhenOps<'__b>(
    mut inWhenOps: &'__b metamodelica::List<BackendDAE::WhenOperator>,
    mut iHs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
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
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match inWhenOps {
            Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: Deref @ DAE::Exp::CREF { componentRef: left, .. }, .. }, tail: rest } => {
                let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                let mut left = (*left).clone();
                left = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&left))?;
                hs = BaseHashSet::add(left.clone(), &iHs)?;
                { (inWhenOps, iHs) = (rest, hs); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: e, .. }, tail: rest } => {
                let mut left: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
                let mut crefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                crefLst = Expression::getAllCrefs(e.clone())?;
                hs = iHs;
                for mut left in &*crefLst {
                    let mut left = left.clone();
                    left = ComponentReferenceBasics::crefStripLastSubs(&left)?;
                    hs = BaseHashSet::add(left, &hs)?;
                }
                { (inWhenOps, iHs) = (rest, hs); continue '__tco; }
            },
            _ => {
                return Ok(iHs)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn traverserExpUnreplaceable(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut unReplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    (outExp, outHt) = 'mc: {
        let __mc_input = &*e;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
                    let mut outHt: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    outHt = traverseCrefUnreplaceable(cr.clone(), None, unReplaceable.clone())?;
                    Ok((e.clone(), outHt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: explst, .. } => {
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut outHt: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    crlst = List::flatten(List::map(explst.clone(), &Expression::extractCrefsFromExp)?)?;
                    crlst = List::map(crlst.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefStripLastSubs(&__a0))?;
                    outHt = List::fold(&crlst, &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), unReplaceable.clone())?;
                    Ok((e.clone(), outHt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((e.clone(), unReplaceable.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outHt)
}

fn traverseCrefUnreplaceable(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut preCref: Option<metamodelica::Ref<DAE::ComponentRef>>,
    mut iUnreplaceable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
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
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCref.clone(), preCref)) {
            (Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: ty, subscriptLst: subs, componentRef: cr }, Some(pcr)) => {
                let mut unReplaceable: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                let mut b: bool;
                let mut pcr = (*pcr).clone();
                (_, b) = Expression::traverseExpTopDownSubs(subs.clone(), &fnptr!(Expression::traversingComponentRefPresent, metamodelica::Ref<DAE::Exp>, bool), false)?;
                pcr = if (b) {ComponentReference::crefPrependIdent(metamodelica::AsArg::as_arg(&pcr), metamodelica::AsArg::as_arg(&name), &(metamodelica::nil()), metamodelica::AsArg::as_arg(&ty))?} else {pcr.clone()};
                unReplaceable = if (b) {BaseHashSet::add(pcr.clone(), &iUnreplaceable)?} else {iUnreplaceable};
                pcr = ComponentReference::crefPrependIdent(metamodelica::AsArg::as_arg(&pcr), metamodelica::AsArg::as_arg(&name), metamodelica::AsArg::as_arg(&subs), metamodelica::AsArg::as_arg(&ty))?;
                { (inCref, preCref, iUnreplaceable) = (cr.clone(), Some(pcr.clone()), unReplaceable); continue '__tco; }
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: ty, subscriptLst: subs, componentRef: cr }, None) => {
                let mut unReplaceable: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                let mut b: bool;
                (_, b) = Expression::traverseExpTopDownSubs(subs.clone(), &fnptr!(Expression::traversingComponentRefPresent, metamodelica::Ref<DAE::Exp>, bool), false)?;
                unReplaceable = if (b) {BaseHashSet::add(metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: name.clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() }), &iUnreplaceable)?} else {iUnreplaceable};
                { (inCref, preCref, iUnreplaceable) = (cr.clone(), Some(metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: name.clone(), identType: ty.clone(), subscriptLst: subs.clone() })), unReplaceable); continue '__tco; }
            },
            (Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, identType: ty, subscriptLst: subs }, Some(pcr)) => {
                let mut unReplaceable: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                let mut b: bool;
                (_, b) = Expression::traverseExpTopDownSubs(subs.clone(), &fnptr!(Expression::traversingComponentRefPresent, metamodelica::Ref<DAE::Exp>, bool), false)?;
                if (b) {return Ok(BaseHashSet::add(ComponentReference::crefPrependIdent(metamodelica::AsArg::as_arg(&pcr), metamodelica::AsArg::as_arg(&name), &(metamodelica::nil()), metamodelica::AsArg::as_arg(&ty))?, &iUnreplaceable)?)} else {return Ok(iUnreplaceable)}
            },
            (Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, identType: ty, .. }, None) => {
                let mut unReplaceable: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                let mut b: bool;
                (_, b) = Expression::traverseExpTopDownCrefHelper(&inCref, &fnptr!(Expression::traversingComponentRefPresent, metamodelica::Ref<DAE::Exp>, bool), false)?;
                if (b) {return Ok(BaseHashSet::add(metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: name.clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() }), &iUnreplaceable)?)} else {return Ok(iUnreplaceable)}
            },
            (Deref @ DAE::ComponentRef::OPTIMICA_ATTR_INST_CREF { .. }, _) => {
                return Ok(iUnreplaceable)
            },
            (Deref @ DAE::ComponentRef::WILD { .. }, _) => {
                return Ok(iUnreplaceable)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn negateExpression(
    mut negationFlag: bool,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inAlternative: metamodelica::Ref<DAE::Exp>,
    mut message: &ArcStr,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExpression: metamodelica::Ref<DAE::Exp>;
    outExpression = (match negationFlag {
        true => {
            let mut negatedExp: metamodelica::Ref<DAE::Exp>;
            negatedExp = Expression::negate(inExp.clone())?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrExpStr(
                    &(literal!("Negating: ")),
                    inExp,
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!(" "));
                        __mm_s.push_str(&*message);
                        __mm_s.push_str(&*literal!(".\n"));
                        ArcStr::from(__mm_s)
                    }),
                )?;
            }
            negatedExp
        }
        false => {
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrExpStrExpStr(
                    &(literal!("Not negating: ")),
                    inExp,
                    &(literal!(" returning: ")),
                    inAlternative.clone(),
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!(" "));
                        __mm_s.push_str(&*message);
                        __mm_s.push_str(&*literal!(".\n"));
                        ArcStr::from(__mm_s)
                    }),
                )?;
            }
            inAlternative
        }
    });
    Ok(outExpression)
}

fn performAliasEliminationBB(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut findAliases: bool,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(
        inDAE,
        &({
            let __pe_b2 = findAliases;
            move |__pe_a0, __pe_a1| Ok(eliminateTrivialEquations(__pe_a0, __pe_a1, __pe_b2.clone()))
        }),
    )?;
    outDAE = BackendDAEUtil::mapEqSystem(&outDAE, &getAliasAttributes)?;
    Ok(outDAE)
}

fn eliminateTrivialEquations(
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut findAliases: bool,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
) {
    let mut outSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    (outSystem, outShared) = 'mc: {
        let __mc_input = (inSystem.clone(), inShared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedVars, orderedEqs, .. }, shared @ Deref @ BackendDAE::Shared { globalKnownVars, aliasVars, initialEqs: inieqns, eventInfo, .. }) => {
                    let mut varList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut eqList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut initEqList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut remEqList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut simpleEqList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut HTCrToExp: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (HashTableCrToExp::FuncHashCref, HashTableCrToExp::FuncCrefEqual, HashTableCrToExp::FuncCrefStr, HashTableCrToExp::FuncExpStr));
                    let mut HTCrToCrEqLst: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<BackendDAE::Equation>)>)>>), i32, (HashTableCrToCrEqLst::FuncHashCref, HashTableCrToCrEqLst::FuncCrefEqual, HashTableCrToCrEqLst::FuncCrefStr, HashTableCrToCrEqLst::FuncExpStr));
                    let mut tplExp: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut tplCrEqLst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<BackendDAE::Equation>)>)>;
                    let mut countAliasEquations: i32;
                    let mut countSimpleEquations: i32;
                    let mut size: i32;
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut syst = (*syst).clone();
                    let mut orderedVars = (*orderedVars).clone();
                    let mut orderedEqs = (*orderedEqs).clone();
                    let mut shared = (*shared).clone();
                    let mut globalKnownVars = (*globalKnownVars).clone();
                    let mut aliasVars = (*aliasVars).clone();
                    let mut inieqns = (*inieqns).clone();
                    size = BackendVariable::varsSize(metamodelica::AsArg::as_arg(&orderedVars));
                    size = intMax(BaseHashTable::defaultBucketSize.clone(), (((intReal(size)) * (metamodelica::OrderedFloat(0.7_f64))).0.floor() as i32));
                    HTCrToExp = HashTableCrToExp::emptyHashTableSized(size);
                    HTCrToCrEqLst = HashTableCrToCrEqLst::emptyHashTableSized(size);
                    repl = BackendVarTransform::emptyReplacementsSized(size);
                    (_, HTCrToExp, HTCrToCrEqLst, eqList, simpleEqList) = BackendEquation::traverseEquationArray(orderedEqs.clone(), &({ let __pe_b2 = findAliases; move |__pe_a0, __pe_a1| Ok(findSimpleEquations(__pe_a0, __pe_a1, __pe_b2.clone())) }), (orderedVars.clone(), HTCrToExp.clone(), HTCrToCrEqLst.clone(), metamodelica::nil(), metamodelica::nil()))?;
                    tplExp = BaseHashTable::hashTableList(&HTCrToExp)?;
                    tplCrEqLst = BaseHashTable::hashTableList(&HTCrToCrEqLst)?;
                    HTCrToExp = addRestCrefs(&tplCrEqLst, HTCrToExp.clone(), &HTCrToCrEqLst)?;
                    tplExp = BaseHashTable::hashTableList(&HTCrToExp)?;
                    (aliasVars, orderedVars) = moveVars(&tplExp, aliasVars.clone(), orderedVars.clone());
                    varList = BackendVariable::varList(metamodelica::AsArg::as_arg(&orderedVars))?;
                    varList = removeStateDerInfo(varList.clone())?;
                    orderedVars = BackendVariable::listVar1(&varList)?;
                    (eqList, _) = BackendEquation::traverseExpsOfEquationList(&eqList, (std::sync::Arc::new(traverseExpTopDown) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>)))> + 'static>), HTCrToExp.clone())?;
                    orderedEqs = BackendEquation::listEquation(&eqList)?;
                    initEqList = BackendEquation::equationList(inieqns.clone())?;
                    (initEqList, _) = BackendEquation::traverseExpsOfEquationList(&initEqList, (std::sync::Arc::new(traverseExpTopDown) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>)))> + 'static>), HTCrToExp.clone())?;
                    inieqns = BackendEquation::listEquation(&initEqList)?;
                    remEqList = BackendEquation::equationList(syst.removedEqs.clone())?;
                    (remEqList, _) = BackendEquation::traverseExpsOfEquationList(&remEqList, (std::sync::Arc::new(traverseExpTopDown) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>)))> + 'static>), HTCrToExp.clone())?;
                    assign_field!(syst.removedEqs = BackendEquation::listEquation(&(List::select(remEqList.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Equation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendEquation::assertWithCondTrue(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Equation>) -> Result<bool> + 'static>))?))?);
                    remEqList = BackendEquation::equationList(shared.removedEqs.clone())?;
                    (remEqList, _) = BackendEquation::traverseExpsOfEquationList(&remEqList, (std::sync::Arc::new(traverseExpTopDown) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>)))> + 'static>), HTCrToExp.clone())?;
                    assign_field!(shared.removedEqs = BackendEquation::listEquation(&(List::select(remEqList.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Equation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendEquation::assertWithCondTrue(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Equation>) -> Result<bool> + 'static>))?))?);
                    repl = addVarReplacements(&tplExp, repl.clone())?;
                    let (__pa0, (_, __pa1)) = BackendVariable::traverseBackendDAEVarsWithUpdate(aliasVars.clone(), (std::sync::Arc::new(fnptr!(replaceAliasVarTraverser, metamodelica::Ref<BackendDAE::Var>, (BackendVarTransform::VariableReplacements, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (BackendVarTransform::VariableReplacements, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>)) -> Result<(metamodelica::Ref<BackendDAE::Var>, (BackendVarTransform::VariableReplacements, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>))> + 'static>), (repl.clone(), metamodelica::nil()))?;
                    aliasVars = metamodelica::Own::own(__pa0);
                    varlst = metamodelica::Own::own(__pa1);
                    aliasVars = List::fold(&varlst, &fixAliasConstBindings, aliasVars.clone())?;
                    (globalKnownVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(globalKnownVars.clone(), (std::sync::Arc::new(fnptr!(replaceVarTraverser, metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements) -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements)> + 'static>), repl.clone())?;
                    if Flags::isSet(Flags::DUMP_REPL.clone())? {
                        tplExp = BaseHashTable::hashTableList(&HTCrToExp)?;
                        countAliasEquations = ((tplExp).len() as i32);
                        countSimpleEquations = ((simpleEqList).len() as i32);
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of Unknowns:    ")); __mm_s.push_str(&*intString(BackendVariable::varsSize(metamodelica::AsArg::as_arg(&orderedVars)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of \"Complex\" Equations:   ")); __mm_s.push_str(&*intString(BackendEquation::equationLstSize(&eqList)?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of Alias Equations:   ")); __mm_s.push_str(&*intString(countAliasEquations)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of Simple Equations:   ")); __mm_s.push_str(&*intString(countSimpleEquations)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print(literal!("\nAliases:\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n"));
                        BaseHashTable::dumpHashTable(&HTCrToExp)?;
                    }
                    assign_field!(
                        syst.orderedVars = orderedVars.clone(),
                        syst.orderedEqs = orderedEqs.clone()
                    );
                    assign_field!(
                        shared.eventInfo = eventInfo.clone(),
                        shared.globalKnownVars = globalKnownVars.clone(),
                        shared.aliasVars = aliasVars.clone(),
                        shared.initialEqs = inieqns.clone()
                    );
                    Ok((BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst)), shared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inSystem.clone(), inShared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outSystem, outShared)
}

fn moveVars(
    mut cr_exp_lst: &metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
    mut inAliasVars: BackendDAE::Variables,
    mut inVars: BackendDAE::Variables,
) -> (BackendDAE::Variables, BackendDAE::Variables) {
    let mut outAliasVars: BackendDAE::Variables = inAliasVars;
    let mut outVars: BackendDAE::Variables = inVars;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut i: i32;
    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
    let mut bs: bool;
    for mut cr_exp in &**cr_exp_lst {
        (cr, e) = cr_exp.clone();
        match '__try0: {
            (v, i) = unwrap_break_err!(BackendVariable::getVarSingle(&cr, &outVars), '__try0);
            v = BackendVariable::setBindExp(v.clone(), Some(e.clone()));
            ops = ElementSource::getSymbolicTransformations(&(DAE::emptyElementSource().clone()));
            v = unwrap_break_err!(BackendVariable::mergeVariableOperations(v.clone(), metamodelica::cons(metamodelica::Ref::new(DAE::SymbolicOperation::SOLVED { cr: cr.clone(), exp: e.clone() }), ops.clone())), '__try0);
            bs = BackendVariable::isStateVar(&v);
            v = if (bs) {
                unwrap_break_err!(BackendVariable::setVarKind(v.clone(), openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE), '__try0)
            } else {
                v.clone()
            };
            (outVars, _) = unwrap_break_err!(BackendVariable::removeVar(i, outVars.clone()), '__try0);
            outAliasVars = unwrap_break_err!(BackendVariable::addVar(v.clone(), outAliasVars.clone()), '__try0);
            Ok::<_, &'static str>((outAliasVars.clone(), outVars.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                outAliasVars = __try0_o0;
                outVars = __try0_o1;
            }
            Err(_) => {
                outAliasVars = outAliasVars.clone();
                outVars = outVars.clone();
            }
        }
    }
    (outAliasVars, outVars)
}

fn addVarReplacements(
    mut cr_exp_lst: &metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
    mut inRepl: BackendVarTransform::VariableReplacements,
) -> Result<BackendVarTransform::VariableReplacements> {
    let mut outRepl: BackendVarTransform::VariableReplacements = inRepl;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    for mut cr_exp in &**cr_exp_lst {
        (cr, e) = cr_exp.clone();
        outRepl = BackendVarTransform::addReplacement(
            outRepl,
            cr,
            e,
            Some(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BackendVarTransform::skipPreChangeEdgeOperator(&__a0))
                })
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>),
            ),
        )?;
    }
    Ok(outRepl)
}

fn traverseExpTopDown(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inHTCrToExp: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outHTCrToExp: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrToExp::FuncHashCref,
            HashTableCrToExp::FuncCrefEqual,
            HashTableCrToExp::FuncCrefStr,
            HashTableCrToExp::FuncExpStr,
        ),
    ) = inHTCrToExp.clone();
    (outExp, _) = Expression::traverseExpTopDown(
        inExp,
        &fnptr!(
            insertReplacementsInEquations,
            metamodelica::Ref<DAE::Exp>,
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>
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
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>
                )
            )
        ),
        inHTCrToExp,
    )?;
    (outExp, _) = ExpressionSimplify::simplify(outExp)?;
    Ok((outExp, outHTCrToExp))
}

fn insertReplacementsInEquations(
    mut inE1: metamodelica::Ref<DAE::Exp>,
    mut inHTCrToExp: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) {
    let mut outE1: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outHTCrToExp: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrToExp::FuncHashCref,
            HashTableCrToExp::FuncCrefEqual,
            HashTableCrToExp::FuncCrefStr,
            HashTableCrToExp::FuncExpStr,
        ),
    );
    (outE1, cont, outHTCrToExp) = 'mc: {
        let __mc_input = &*inE1;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
                    let mut value: metamodelica::Ref<DAE::Exp>;
                    if BaseHashTable::hasKey(cr.clone(), &inHTCrToExp)? {
                        value = BaseHashTable::get(cr.clone(), &inHTCrToExp)?;
                    } else {
                        value = inE1.clone();
                    }
                    Ok((value.clone(), true, inHTCrToExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inE1.clone(), true, inHTCrToExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outE1, cont, outHTCrToExp)
}

fn removeStateDerInfo(
    mut inVarList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    vars = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
        for mut var in (inVarList).into_iter().cloned() {
            let __x = if (BackendVariable::isStateVar(&(var.clone()))) {
                BackendVariable::setStateDerivative(var.clone(), None)?
            } else {
                var.clone()
            };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(vars)
}

fn findSimpleEquations(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTuple: (
        BackendDAE::Variables,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::List<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<BackendDAE::Equation>,
                        )>,
                    )>,
                >,
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
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::List<(
                                metamodelica::Ref<DAE::ComponentRef>,
                                metamodelica::Ref<BackendDAE::Equation>,
                            )>,
                        ) -> Result<ArcStr>
                        + 'static,
                >,
            ),
        ),
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    ),
    mut findAliases: bool,
) -> (
    metamodelica::Ref<BackendDAE::Equation>,
    (
        BackendDAE::Variables,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::List<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<BackendDAE::Equation>,
                        )>,
                    )>,
                >,
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
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::List<(
                                metamodelica::Ref<DAE::ComponentRef>,
                                metamodelica::Ref<BackendDAE::Equation>,
                            )>,
                        ) -> Result<ArcStr>
                        + 'static,
                >,
            ),
        ),
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    ),
) {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut outTuple: (
        BackendDAE::Variables,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                HashTableCrToExp::FuncHashCref,
                HashTableCrToExp::FuncCrefEqual,
                HashTableCrToExp::FuncCrefStr,
                HashTableCrToExp::FuncExpStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::List<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<BackendDAE::Equation>,
                        )>,
                    )>,
                >,
            ),
            i32,
            (
                HashTableCrToCrEqLst::FuncHashCref,
                HashTableCrToCrEqLst::FuncCrefEqual,
                HashTableCrToCrEqLst::FuncCrefStr,
                HashTableCrToCrEqLst::FuncExpStr,
            ),
        ),
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    );
    (outEq, outTuple) = 'mc: {
        let __mc_input = (inEq.clone(), &inTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eq, (vars, HTCrToExp, HTCrToCrEqLst, eqList, simpleEqList)) => {
                    let mut eqSolved: metamodelica::Ref<BackendDAE::Equation>;
                    let mut cr_lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut count: i32;
                    let mut paramCount: i32;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let mut value: metamodelica::Ref<DAE::Exp>;
                    let mut exp1: metamodelica::Ref<DAE::Exp>;
                    let mut exp2: metamodelica::Ref<DAE::Exp>;
                    let mut keepEquation: bool;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut eqAttr: BackendDAE::EquationAttributes;
                    let mut HTCrToExp = (*HTCrToExp).clone();
                    let mut HTCrToCrEqLst = (*HTCrToCrEqLst).clone();
                    let mut eqList = (*eqList).clone();
                    let mut simpleEqList = (*simpleEqList).clone();
                    res = BackendEquation::getEquationRHS(metamodelica::AsArg::as_arg(&eq))?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(res.clone(), &fnptr!(findCrefs, metamodelica::Ref<DAE::Exp>, (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, BackendDAE::Variables, i32, i32, bool)), (metamodelica::nil(), vars.clone(), 0, 0, true))?) {
                        (_, (__pa0, _, __pa1, __pa2, true)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr_lst = metamodelica::Own::own(__pa0);
                    count = metamodelica::Own::own(__pa1);
                    paramCount = metamodelica::Own::own(__pa2);
                    res = BackendEquation::getEquationLHS(metamodelica::AsArg::as_arg(&eq))?;
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(res.clone(), &fnptr!(findCrefs, metamodelica::Ref<DAE::Exp>, (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, BackendDAE::Variables, i32, i32, bool)), (cr_lst.clone(), vars.clone(), count, paramCount, true))?) {
                        (_, (__pa3, _, __pa4, _, true)) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr_lst = metamodelica::Own::own(__pa3);
                    count = metamodelica::Own::own(__pa4);
                    keepEquation = true;
                    if count == 1 {
                        if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Found Equation knw0: ")); __mm_s.push_str(&*BackendDump::equationString(metamodelica::AsArg::as_arg(&eq))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        }
                        let __pa5 = ::match_deref::match_deref! { match &(cr_lst.clone()) {
                            Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Nil } => __pa5.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        cr = metamodelica::Own::own(__pa5);
                        let false = (BackendVariable::isState(cr.clone(), metamodelica::AsArg::as_arg(&vars))) else { return Err("pattern mismatch") };
                        let false = (BackendVariable::isClockedState(cr.clone(), metamodelica::AsArg::as_arg(&vars))) else { return Err("pattern mismatch") };
                        let false = (BackendVariable::isOutput(cr.clone(), metamodelica::AsArg::as_arg(&vars))) else { return Err("pattern mismatch") };
                        let false = (BackendVariable::isDiscrete(&cr, metamodelica::AsArg::as_arg(&vars))?) else { return Err("pattern mismatch") };
                        exp1 = Expression::crefExp(cr.clone())?;
                        let true = (Types::isSimpleType(&(Expression::r#typeof(exp1.clone())?))) else { return Err("pattern mismatch") };
                        let (__pa8, __pa7) = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eq.clone(), exp1.clone(), None)?) {
                            __pa8 @ Deref @ BackendDAE::Equation::EQUATION { scalar: __pa7, .. } => (__pa8.clone(), __pa7.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        res = metamodelica::Own::own(__pa7);
                        eqSolved = metamodelica::Own::own(__pa8);
                        let true = (isSimple(res.clone())?) else { return Err("pattern mismatch") };
                        if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Found Equation knw1: ")); __mm_s.push_str(&*BackendDump::equationString(metamodelica::AsArg::as_arg(&eq))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        }
                        HTCrToExp = addToCrToExp(cr.clone(), eqSolved.clone(), HTCrToExp.clone(), &(HTCrToCrEqLst.clone()))?;
                        keepEquation = false;
                    } else if count == 2 && findAliases {
                        if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Found Equation al0: ")); __mm_s.push_str(&*BackendDump::equationString(metamodelica::AsArg::as_arg(&eq))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        }
                        let (__pa9, __pa10) = ::match_deref::match_deref! { match &(cr_lst.clone()) {
                            Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: Deref @ metamodelica::ListNode::Cons { head: __pa10, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa9.clone(), __pa10.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        cr2 = metamodelica::Own::own(__pa9);
                        cr1 = metamodelica::Own::own(__pa10);
                        let false = (BackendVariable::isState(cr1.clone(), metamodelica::AsArg::as_arg(&vars)) || BackendVariable::isState(cr2.clone(), metamodelica::AsArg::as_arg(&vars))) else { return Err("pattern mismatch") };
                        let false = (BackendVariable::isClockedState(cr1.clone(), metamodelica::AsArg::as_arg(&vars)) || BackendVariable::isClockedState(cr2.clone(), metamodelica::AsArg::as_arg(&vars))) else { return Err("pattern mismatch") };
                        let false = (BackendVariable::isOutput(cr1.clone(), metamodelica::AsArg::as_arg(&vars)) || BackendVariable::isOutput(cr2.clone(), metamodelica::AsArg::as_arg(&vars))) else { return Err("pattern mismatch") };
                        let false = (BackendVariable::isDiscrete(&cr1, metamodelica::AsArg::as_arg(&vars))? || BackendVariable::isDiscrete(&cr2, metamodelica::AsArg::as_arg(&vars))?) else { return Err("pattern mismatch") };
                        exp1 = Expression::crefExp(cr1.clone())?;
                        let true = (Types::isSimpleType(&(Expression::r#typeof(exp1.clone())?))) else { return Err("pattern mismatch") };
                        exp2 = Expression::crefExp(cr2.clone())?;
                        let __pa12 = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eq.clone(), exp2.clone(), None)?) {
                            Deref @ BackendDAE::Equation::EQUATION { scalar: __pa12, .. } => __pa12.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        res = metamodelica::Own::own(__pa12);
                        let true = (isSimple(res.clone())?) else { return Err("pattern mismatch") };
                        let __pa13 = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eq.clone(), exp1.clone(), None)?) {
                            Deref @ BackendDAE::Equation::EQUATION { scalar: __pa13, .. } => __pa13.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        res = metamodelica::Own::own(__pa13);
                        let true = (isSimple(res.clone())?) else { return Err("pattern mismatch") };
                        if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Found Equation al1: ")); __mm_s.push_str(&*BackendDump::equationString(metamodelica::AsArg::as_arg(&eq))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        }
                        HTCrToCrEqLst = addToCrAndEqLists(cr2.clone(), cr1.clone(), inEq.clone(), HTCrToCrEqLst.clone())?;
                        HTCrToCrEqLst = addToCrAndEqLists(cr1.clone(), cr2.clone(), inEq.clone(), HTCrToCrEqLst.clone())?;
                        if BaseHashTable::hasKey(cr2.clone(), &(HTCrToExp.clone()))? {
                            value = BaseHashTable::get(cr2.clone(), &(HTCrToExp.clone()))?;
                            let (__pa14, __pa15, __pa16) = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eq.clone(), Expression::crefExp(cr1.clone())?, None)?) {
                                        Deref @ BackendDAE::Equation::EQUATION { scalar: __pa14, source: __pa15, attr: __pa16, .. } => (__pa14.clone(), __pa15.clone(), __pa16.clone()),
                                        _ => return Err("pattern mismatch"),
                            } };
                            res = metamodelica::Own::own(__pa14);
                            source = metamodelica::Own::own(__pa15);
                            eqAttr = metamodelica::Own::own(__pa16);
                            (res, _) = Expression::replaceExp(res.clone(), Expression::crefExp(cr2.clone())?, value.clone())?;
                            (res, _) = ExpressionSimplify::simplify(res.clone())?;
                            HTCrToExp = addToCrToExp(cr1.clone(), metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(cr1.clone())?, scalar: res.clone(), source: source.clone(), attr: eqAttr }), HTCrToExp.clone(), &(HTCrToCrEqLst.clone()))?;
                        } else {
                            if BaseHashTable::hasKey(cr1.clone(), &(HTCrToExp.clone()))? {
                                        value = BaseHashTable::get(cr1.clone(), &(HTCrToExp.clone()))?;
                                        let (__pa17, __pa18, __pa19) = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eq.clone(), Expression::crefExp(cr2.clone())?, None)?) {
                                            Deref @ BackendDAE::Equation::EQUATION { scalar: __pa17, source: __pa18, attr: __pa19, .. } => (__pa17.clone(), __pa18.clone(), __pa19.clone()),
                                            _ => return Err("pattern mismatch"),
                                        } };
                                        res = metamodelica::Own::own(__pa17);
                                        source = metamodelica::Own::own(__pa18);
                                        eqAttr = metamodelica::Own::own(__pa19);
                                        (res, _) = Expression::replaceExp(res.clone(), Expression::crefExp(cr1.clone())?, value.clone())?;
                                        (res, _) = ExpressionSimplify::simplify(res.clone())?;
                                        HTCrToExp = addToCrToExp(cr2.clone(), metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(cr2.clone())?, scalar: res.clone(), source: source.clone(), attr: eqAttr }), HTCrToExp.clone(), &(HTCrToCrEqLst.clone()))?;
                            }
                        }
                        keepEquation = false;
                    }
                    if keepEquation {
                        eqList = metamodelica::cons(inEq.clone(), eqList.clone());
                    } else {
                        simpleEqList = metamodelica::cons(inEq.clone(), simpleEqList.clone());
                    }
                    Ok((inEq.clone(), (vars.clone(), HTCrToExp.clone(), HTCrToCrEqLst.clone(), eqList.clone(), simpleEqList.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (vars, HTCrToExp, HTCrToCrEqLst, eqList, simpleEqList)) => {
                    let mut eqList = (*eqList).clone();
                    eqList = metamodelica::cons(inEq.clone(), eqList.clone());
                    Ok((inEq.clone(), (vars.clone(), HTCrToExp.clone(), HTCrToCrEqLst.clone(), eqList.clone(), simpleEqList.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("\n++++++++++ Error in RemoveSimpleEquations.findSimpleEquations ++++++++++\n"));
                    Ok((inEq.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outEq, outTuple)
}

fn findCrefs(
    mut inE1: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        BackendDAE::Variables,
        i32,
        i32,
        bool,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        BackendDAE::Variables,
        i32,
        i32,
        bool,
    ),
) {
    let mut outE1: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTuple: (
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        BackendDAE::Variables,
        i32,
        i32,
        bool,
    );
    (outE1, cont, outTuple) = 'mc: {
        let __mc_input = (&*inE1, &inTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, vars, count, _, _)) => {
                    if !((count.clone() < 0)) { return Err("guard") }
                    Ok((inE1.clone(), false, (metamodelica::nil(), vars.clone(), -1, -1, false)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (cr_lst, vars, count, paramCount, true)) => {
                    if !((count.clone() < 2 && !(ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr), &(DAE::crefTime().clone()))?))) { return Err("guard") }
                    BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?;
                    Ok((inE1.clone(), true, (metamodelica::cons(cr.clone(), cr_lst.clone()), vars.clone(), count.clone() + 1, paramCount.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (cr_lst, vars, count, paramCount, true)) => {
                    if !((count.clone() < 2 && !(ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr), &(DAE::crefTime().clone()))?))) { return Err("guard") }
                    Ok((inE1.clone(), true, (cr_lst.clone(), vars.clone(), count.clone(), paramCount.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, (_, vars, _, _, true)) => {
                    Ok((inE1.clone(), false, (metamodelica::nil(), vars.clone(), -1, -1, false)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { .. }, (_, vars, _, _, _)) => {
                    Ok((inE1.clone(), false, (metamodelica::nil(), vars.clone(), -1, -1, false)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { .. }, (_, vars, _, _, _)) => {
                    Ok((inE1.clone(), false, (metamodelica::nil(), vars.clone(), -1, -1, false)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { .. }, (_, vars, _, _, _)) => {
                    Ok((inE1.clone(), false, (metamodelica::nil(), vars.clone(), -1, -1, false)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RECORD { .. }, (_, vars, _, _, _)) => {
                    Ok((inE1.clone(), false, (metamodelica::nil(), vars.clone(), -1, -1, false)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inE1.clone(), true, inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outE1, cont, outTuple)
}

fn addToCrAndEqLists(
    mut cr1: metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: metamodelica::Ref<DAE::ComponentRef>,
    mut eq: metamodelica::Ref<BackendDAE::Equation>,
    mut inHTCrToCrEqLst: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<BackendDAE::Equation>,
                    )>,
                )>,
            >,
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
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::List<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<BackendDAE::Equation>,
                        )>,
                    ) -> Result<ArcStr>
                    + 'static,
            >,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<BackendDAE::Equation>,
                )>,
            )>,
        >,
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
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<BackendDAE::Equation>,
                    )>,
                ) -> Result<ArcStr>
                + 'static,
        >,
    ),
)> {
    let mut outHTCrToCrEqLst: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<BackendDAE::Equation>,
                    )>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCrToCrEqLst::FuncHashCref,
            HashTableCrToCrEqLst::FuncCrefEqual,
            HashTableCrToCrEqLst::FuncCrefStr,
            HashTableCrToCrEqLst::FuncExpStr,
        ),
    );
    let mut cr_eq_lst: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<BackendDAE::Equation>,
    )>;
    let mut eqSolved: metamodelica::Ref<BackendDAE::Equation>;
    outHTCrToCrEqLst = (match inHTCrToCrEqLst {
        mut HTCrToCrEqLst => {
            eqSolved = BackendEquation::solveEquation(eq, Expression::crefExp(cr2.clone())?, None)?;
            if BaseHashTable::hasKey(cr1.clone(), &HTCrToCrEqLst)? {
                cr_eq_lst = BaseHashTable::get(cr1.clone(), &HTCrToCrEqLst)?;
                cr_eq_lst = metamodelica::cons((cr2, eqSolved), cr_eq_lst);
            } else {
                cr_eq_lst = list![(cr2, eqSolved)];
            }
            HTCrToCrEqLst = BaseHashTable::add((cr1, cr_eq_lst), HTCrToCrEqLst)?;
            HTCrToCrEqLst
        }
        _ => {
            metamodelica::print(literal!(
                "\n++++++++++ Error in RemoveSimpleEquations.addToCrAndEqLists ++++++++++\n"
            ));
            BackendDump::printEquation(&eq)?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Solve for:"));
                __mm_s.push_str(&*ComponentReference::debugPrintComponentRefTypeStr(&cr1)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            return Err("fail");
        }
    });
    Ok(outHTCrToCrEqLst)
}

fn addToCrToExp(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut eq: metamodelica::Ref<BackendDAE::Equation>,
    mut inHTCrToExp: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHTCrToCrEqLst: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<BackendDAE::Equation>,
                    )>,
                )>,
            >,
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
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::List<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<BackendDAE::Equation>,
                        )>,
                    ) -> Result<ArcStr>
                    + 'static,
            >,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outHTCrToExp: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrToExp::FuncHashCref,
            HashTableCrToExp::FuncCrefEqual,
            HashTableCrToExp::FuncCrefStr,
            HashTableCrToExp::FuncExpStr,
        ),
    );
    let mut value: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    outHTCrToExp = 'mc: {
        let __mc_input = ();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let mut outHTCrToExp: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
                ),
                i32,
                (
                    HashTableCrToExp::FuncHashCref,
                    HashTableCrToExp::FuncCrefEqual,
                    HashTableCrToExp::FuncCrefStr,
                    HashTableCrToExp::FuncExpStr,
                ),
            );
            let mut value: metamodelica::Ref<DAE::Exp> = value.clone();
            let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eq.clone(), Expression::crefExp(cr.clone())?, None)?) {
                Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            value = metamodelica::Own::own(__pa0);
            outHTCrToExp = BaseHashTable::add((cr.clone(), value.clone()), inHTCrToExp.clone())?;
            outHTCrToExp = solveAllCrefs(cr.clone(), &value, outHTCrToExp.clone(), inHTCrToCrEqLst);
            Ok((outHTCrToExp.clone(), value.clone()))
        })() {
            value = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!(
                "\n++++++++++ Error in RemoveSimpleEquations.addToCrToExp ++++++++++\n"
            ));
            BackendDump::printEquation(&eq)?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentReference::debugPrintComponentRefTypeStr(&cr)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outHTCrToExp)
}

fn solveAllCrefs(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut value: &metamodelica::Ref<DAE::Exp>,
    mut inHTCrToExp: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHTCrToCrEqLst: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<BackendDAE::Equation>,
                    )>,
                )>,
            >,
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
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::List<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<BackendDAE::Equation>,
                        )>,
                    ) -> Result<ArcStr>
                    + 'static,
            >,
        ),
    ),
) -> (
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
    ),
) {
    let mut outHTCrToExp: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrToExp::FuncHashCref,
            HashTableCrToExp::FuncCrefEqual,
            HashTableCrToExp::FuncCrefStr,
            HashTableCrToExp::FuncExpStr,
        ),
    );
    outHTCrToExp = 'mc: {
        let __mc_input = inHTCrToExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut HTCrToExp = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut cr_eq_lst: metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::Ref<BackendDAE::Equation>,
            )>;
            if BaseHashTable::hasKey(cr.clone(), inHTCrToCrEqLst)? {
                cr_eq_lst = BaseHashTable::get(cr.clone(), inHTCrToCrEqLst)?;
                HTCrToExp = solveAllCrefs1(&cr, value, &cr_eq_lst, HTCrToExp.clone(), inHTCrToCrEqLst);
            }
            Ok(HTCrToExp.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!(
                "\n++++++++++ Error in RemoveSimpleEquations.solveAllCrefs ++++++++++\n"
            ));
            Ok(inHTCrToExp.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outHTCrToExp
}

fn solveAllCrefs1(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut value: &metamodelica::Ref<DAE::Exp>,
    mut cr_eq_lst: &metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<BackendDAE::Equation>,
    )>,
    mut inHTCrToExp: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHTCrToCrEqLst: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<BackendDAE::Equation>,
                    )>,
                )>,
            >,
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
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::List<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<BackendDAE::Equation>,
                        )>,
                    ) -> Result<ArcStr>
                    + 'static,
            >,
        ),
    ),
) -> (
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
    ),
) {
    let mut outHTCrToExp: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrToExp::FuncHashCref,
            HashTableCrToExp::FuncCrefEqual,
            HashTableCrToExp::FuncCrefStr,
            HashTableCrToExp::FuncExpStr,
        ),
    );
    outHTCrToExp = 'mc: {
        let __mc_input = (&**cr_eq_lst, inHTCrToExp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, HTCrToExp) => {
                    Ok(HTCrToExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (cr1, eq), tail: cr_eq_rest }, HTCrToExp) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut eqAttr: BackendDAE::EquationAttributes;
                    let mut HTCrToExp = (*HTCrToExp).clone();
                    if !(BaseHashTable::hasKey(cr1.clone(), &(HTCrToExp.clone()))?) && !(isCrefInValue(cr1.clone(), value.clone())?) {
                        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eq.clone(), Expression::crefExp(cr1.clone())?, None)?) {
                            Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, source: __pa1, attr: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        res = metamodelica::Own::own(__pa0);
                        source = metamodelica::Own::own(__pa1);
                        eqAttr = metamodelica::Own::own(__pa2);
                        (res, _) = Expression::replaceExp(res.clone(), Expression::crefExp(cr.clone())?, value.clone())?;
                        (res, _) = ExpressionSimplify::simplify(res.clone())?;
                        HTCrToExp = addToCrToExp(cr1.clone(), metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(cr1.clone())?, scalar: res.clone(), source: source.clone(), attr: eqAttr }), inHTCrToExp.clone(), inHTCrToCrEqLst)?;
                    }
                    HTCrToExp = solveAllCrefs1(cr, value, metamodelica::AsArg::as_arg(&cr_eq_rest), HTCrToExp.clone(), inHTCrToCrEqLst);
                    Ok(HTCrToExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("\n++++++++++ Error in RemoveSimpleEquations.solveAllCrefs1 ++++++++++\n"));
                    Ok(inHTCrToExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outHTCrToExp
}

fn isCrefInValue(mut cr: metamodelica::Ref<DAE::ComponentRef>, mut value: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut isInValue: bool;
    let mut cr_lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    cr_lst = Expression::extractCrefsFromExp(value)?;
    isInValue = listMember(cr, cr_lst);
    Ok(isInValue)
}

fn addRestCrefs(
    mut tplCrEqLst: &metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::Ref<BackendDAE::Equation>,
        )>,
    )>,
    mut inHTCrToExp: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHTCrToCrEqLst: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<BackendDAE::Equation>,
                    )>,
                )>,
            >,
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
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::List<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<BackendDAE::Equation>,
                        )>,
                    ) -> Result<ArcStr>
                    + 'static,
            >,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut HTCrToExp: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrToExp::FuncHashCref,
            HashTableCrToExp::FuncCrefEqual,
            HashTableCrToExp::FuncCrefStr,
            HashTableCrToExp::FuncExpStr,
        ),
    ) = inHTCrToExp;
    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
    let mut cr_eq_lst: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<BackendDAE::Equation>,
    )>;
    match '__try0: {
        for mut tpl in &**tplCrEqLst {
            (cr1, cr_eq_lst) = tpl.clone();
            if !(unwrap_break_err!(BaseHashTable::hasKey(cr1.clone(), &HTCrToExp), '__try0)) {
                HTCrToExp = addThisCrefs(&cr_eq_lst, HTCrToExp.clone(), inHTCrToCrEqLst);
            }
        }
        Ok::<(), &'static str>(())
    } {
        Ok(()) => {}
        Err(__try0_err) => {
            metamodelica::print(literal!(
                "\n++++++++++ Error in RemoveSimpleEquations.addRestCrefs ++++++++++\n"
            ));
            return Err(__try0_err);
        }
    }
    Ok(HTCrToExp)
}

fn addThisCrefs(
    mut cr_eq_lst: &metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<BackendDAE::Equation>,
    )>,
    mut inHTCrToExp: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHTCrToCrEqLst: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<BackendDAE::Equation>,
                    )>,
                )>,
            >,
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
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::List<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<BackendDAE::Equation>,
                        )>,
                    ) -> Result<ArcStr>
                    + 'static,
            >,
        ),
    ),
) -> (
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
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
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
    ),
) {
    let mut outHTCrToExp: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrToExp::FuncHashCref,
            HashTableCrToExp::FuncCrefEqual,
            HashTableCrToExp::FuncCrefStr,
            HashTableCrToExp::FuncExpStr,
        ),
    );
    outHTCrToExp = 'mc: {
        let __mc_input = (&**cr_eq_lst, inHTCrToExp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, HTCrToExp) => {
                    Ok(HTCrToExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (cr1, eq), tail: cr_eq_rest }, HTCrToExp) => {
                    let mut HTCrToExp = (*HTCrToExp).clone();
                    if !(BaseHashTable::hasKey(cr1.clone(), &(HTCrToExp.clone()))?) {
                        HTCrToExp = addToCrToExp(cr1.clone(), eq.clone(), HTCrToExp.clone(), inHTCrToCrEqLst)?;
                    }
                    HTCrToExp = addThisCrefs(metamodelica::AsArg::as_arg(&cr_eq_rest), HTCrToExp.clone(), inHTCrToCrEqLst);
                    Ok(HTCrToExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("\n++++++++++ Error in RemoveSimpleEquations.addThisCrefs ++++++++++\n"));
                    Ok(inHTCrToExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outHTCrToExp
}

fn isSimple(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut outIsSimple: bool;
    (_, outIsSimple) = Expression::traverseExpTopDown(
        inExp,
        &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: bool| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(checkOperator(&__a0, __a1))
        },
        true,
    )?;
    Ok(outIsSimple)
}

fn checkOperator(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inIsSimple: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outIsSimple: bool;
    (outExp, cont, outIsSimple) = 'mc: {
        let __mc_input = &**inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1, operator: op, exp2 } => {
                    let true = (checkOp(metamodelica::AsArg::as_arg(&op))) else { return Err("pattern mismatch") };
                    ::match_deref::match_deref! { match &(checkOperator(metamodelica::AsArg::as_arg(&exp1), inIsSimple)) {
                        (_, true, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(checkOperator(metamodelica::AsArg::as_arg(&exp2), inIsSimple)) {
                        (_, true, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok((inExp.clone(), true, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: _, exp: exp1 } => {
                    Ok(checkOperator(metamodelica::AsArg::as_arg(&exp1), inIsSimple))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: _, exp: exp1 } => {
                    Ok(checkOperator(metamodelica::AsArg::as_arg(&exp1), inIsSimple))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { .. } => {
                    Ok((inExp.clone(), true, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ICONST { .. } => {
                    Ok((inExp.clone(), true, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RCONST { .. } => {
                    Ok((inExp.clone(), true, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { .. } => {
                    Ok((inExp.clone(), true, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SCONST { .. } => {
                    Ok((inExp.clone(), true, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), false, false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outIsSimple)
}

fn checkOp(mut inOp: &DAE::Operator) -> bool {
    let mut outB: bool;
    outB = (match inOp.clone() {
        DAE::Operator::ADD { .. } => true,
        DAE::Operator::SUB { .. } => true,
        DAE::Operator::UMINUS { .. } => true,
        DAE::Operator::MUL { .. } => false,
        DAE::Operator::EQUAL { .. } => false,
        DAE::Operator::DIV { .. } => false,
        DAE::Operator::POW { .. } => false,
        _ => false,
    });
    outB
}

fn determineAliasLst(
    mut inAliasVars: BackendDAE::Variables,
    mut inVars: BackendDAE::Variables,
    mut inHTAliasLst: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<BackendDAE::Equation>,
                    )>,
                )>,
            >,
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
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::List<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<BackendDAE::Equation>,
                        )>,
                    ) -> Result<ArcStr>
                    + 'static,
            >,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<BackendDAE::Equation>,
                )>,
            )>,
        >,
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
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<BackendDAE::Equation>,
                    )>,
                ) -> Result<ArcStr>
                + 'static,
        >,
    ),
)> {
    let mut outHTAliasLst: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<BackendDAE::Equation>,
                    )>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCrToCrEqLst::FuncHashCref,
            HashTableCrToCrEqLst::FuncCrefEqual,
            HashTableCrToCrEqLst::FuncCrefStr,
            HashTableCrToCrEqLst::FuncExpStr,
        ),
    ) = inHTAliasLst;
    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
    let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
    let mut cr_lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut w: Option<metamodelica::Ref<BackendDAE::Var>> = None;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut count: i32;
    let mut vars: metamodelica::Array<Option<metamodelica::Ref<BackendDAE::Var>>>;
    let BackendDAE::VARIABLES {
        varArr: BackendDAE::VARIABLE_ARRAY { varOptArr: __pa0, .. },
        ..
    } = inAliasVars;
    vars = metamodelica::Own::own(__pa0);
    let __range1 = vars.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut w in __range1 {
        match '__try2: {
            let __pa3 = ::match_deref::match_deref! { match &(w.clone()) {
                Some(__pa3) => __pa3.clone(),
                _ => break '__try2 Err::<_, _>("pattern mismatch"),
            } };
            v = metamodelica::Own::own(__pa3);
            cr1 = BackendVariable::varCref(&v);
            e = unwrap_break_err!(BackendVariable::varBindExp(&v), '__try2);
            let (_, (__pa4, _, __pa5, _, _)) = unwrap_break_err!(Expression::traverseExpTopDown(e.clone(), &fnptr!(findCrefs, metamodelica::Ref<DAE::Exp>, (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, BackendDAE::Variables, i32, i32, bool)), (metamodelica::nil(), inVars.clone(), 0, 0, true)), '__try2);
            cr_lst = metamodelica::Own::own(__pa4);
            count = metamodelica::Own::own(__pa5);
            let 1 = (count) else {
                break '__try2 Err::<_, _>("pattern mismatch");
            };
            let __pa6 = ::match_deref::match_deref! { match &(cr_lst.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Nil } => __pa6.clone(),
                _ => break '__try2 Err::<_, _>("pattern mismatch"),
            } };
            cr2 = metamodelica::Own::own(__pa6);
            eq = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                exp: unwrap_break_err!(Expression::crefExp(cr1.clone()), '__try2),
                scalar: e.clone(),
                source: DAE::emptyElementSource().clone(),
                attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone(),
            });
            outHTAliasLst = unwrap_break_err!(addToCrAndEqLists(cr2.clone(), cr1.clone(), eq.clone(), outHTAliasLst.clone()), '__try2);
            Ok::<_, &'static str>((outHTAliasLst.clone(),))
        } {
            Ok((__try2_o0,)) => {
                outHTAliasLst = __try2_o0;
            }
            Err(_) => {
                outHTAliasLst = outHTAliasLst.clone();
            }
        }
    }
    Ok(outHTAliasLst)
}

fn getAliasAttributes(
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared.clone();
    let mut orderedVars: BackendDAE::Variables;
    let mut aliasVars: BackendDAE::Variables;
    let mut HTAliasLst: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<BackendDAE::Equation>,
                    )>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCrToCrEqLst::FuncHashCref,
            HashTableCrToCrEqLst::FuncCrefEqual,
            HashTableCrToCrEqLst::FuncCrefStr,
            HashTableCrToCrEqLst::FuncExpStr,
        ),
    );
    let mut tplAliasLst: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::Ref<BackendDAE::Equation>,
        )>,
    )>;
    let mut size: i32;
    let __arc1 = inSystem.clone();
    let BackendDAE::EQSYSTEM { orderedVars: __pa0, .. } = &*__arc1;
    orderedVars = metamodelica::Own::own(__pa0);
    let __arc3 = inShared;
    let BackendDAE::SHARED { aliasVars: __pa2, .. } = &*__arc3;
    aliasVars = metamodelica::Own::own(__pa2);
    size = BackendVariable::varsSize(&orderedVars);
    size = intMax(
        BaseHashTable::defaultBucketSize.clone(),
        (((intReal(size)) * (metamodelica::OrderedFloat(0.7_f64))).0.floor() as i32),
    );
    HTAliasLst = HashTableCrToCrEqLst::emptyHashTableSized(size);
    HTAliasLst = determineAliasLst(aliasVars.clone(), orderedVars.clone(), HTAliasLst)?;
    tplAliasLst = BaseHashTable::hashTableList(&HTAliasLst)?;
    orderedVars = setAttributes(&tplAliasLst, orderedVars, &aliasVars);
    outSystem = BackendDAEUtil::setEqSystVars(inSystem, orderedVars);
    Ok((outSystem, outShared))
}

fn setAttributes(
    mut tplCrEqLst: &metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::Ref<BackendDAE::Equation>,
        )>,
    )>,
    mut inVars: BackendDAE::Variables,
    mut inAliasVars: &BackendDAE::Variables,
) -> BackendDAE::Variables {
    let mut outVars: BackendDAE::Variables = inVars;
    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
    let mut cr_lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut i: i32;
    let mut j: i32;
    let mut cr_eq_lst: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<BackendDAE::Equation>,
    )>;
    let mut HTStartExpToInt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    );
    let mut HTNominalExpToInt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    );
    let mut tplExpIndList: metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>;
    if (tplCrEqLst).is_empty() {
        return outVars;
    }
    if '__try0: {
        HTStartExpToInt = HashTableExpToIndex::emptyHashTableSized(100);
        HTNominalExpToInt = HashTableExpToIndex::emptyHashTableSized(100);
        for mut tpl in &**tplCrEqLst {
            (cr1, cr_eq_lst) = tpl.clone();
            unwrap_break_err!(BaseHashTable::clear(HTStartExpToInt.clone()), '__try0);
            unwrap_break_err!(BaseHashTable::clear(HTNominalExpToInt.clone()), '__try0);
            (v, i) = unwrap_break_err!(BackendVariable::getVarSingle(&cr1, &outVars), '__try0);
            if BackendVariable::varHasStartValue(&v) {
                e = unwrap_break_err!(BackendVariable::varStartValue(&v), '__try0);
                if unwrap_break_err!(Expression::isZero(&e), '__try0) {
                    e = metamodelica::Ref::new(DAE::Exp::RCONST {
                        real: metamodelica::OrderedFloat(0.0_f64),
                    });
                }
                cr_lst = unwrap_break_err!(Expression::extractCrefsFromExp(e.clone()), '__try0);
                j = 2 - ((cr_lst).len() as i32);
                j = j * unwrap_break_err!(ComponentReference::crefDepth(&cr1), '__try0);
                HTStartExpToInt =
                    unwrap_break_err!(BaseHashTable::add((e.clone(), j), HTStartExpToInt.clone()), '__try0);
                if unwrap_break_err!(Flags::isSet(Flags::DEBUG_ALIAS.clone()), '__try0) {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("START: "));
                        __mm_s.push_str(
                            &*unwrap_break_err!(ComponentReferenceBasics::printComponentRefStr(&cr1), '__try0),
                        );
                        __mm_s.push_str(&*literal!(" = "));
                        __mm_s.push_str(&*unwrap_break_err!(ExpressionBasics::printExpStr(e.clone()), '__try0));
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
            }
            if BackendVariable::varHasNominalValue(&v) {
                e = unwrap_break_err!(BackendVariable::varNominalValue(&v), '__try0);
                cr_lst = unwrap_break_err!(Expression::extractCrefsFromExp(e.clone()), '__try0);
                j = 2 - ((cr_lst).len() as i32);
                j = j * unwrap_break_err!(ComponentReference::crefDepth(&cr1), '__try0);
                HTNominalExpToInt =
                    unwrap_break_err!(BaseHashTable::add((e.clone(), j), HTNominalExpToInt.clone()), '__try0);
                if unwrap_break_err!(Flags::isSet(Flags::DEBUG_ALIAS.clone()), '__try0) {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NOMINAL: "));
                        __mm_s.push_str(
                            &*unwrap_break_err!(ComponentReferenceBasics::printComponentRefStr(&cr1), '__try0),
                        );
                        __mm_s.push_str(&*literal!(" = "));
                        __mm_s.push_str(&*unwrap_break_err!(ExpressionBasics::printExpStr(e.clone()), '__try0));
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
            }
            (HTStartExpToInt, HTNominalExpToInt) = getThisAttributes(
                &cr1,
                &cr_eq_lst,
                inAliasVars,
                HTStartExpToInt.clone(),
                HTNominalExpToInt.clone(),
            );
            tplExpIndList = unwrap_break_err!(BaseHashTable::hashTableList(&HTStartExpToInt), '__try0);
            if !((tplExpIndList).is_empty()) {
                e = getDominantAttributeValue(&tplExpIndList);
                v = unwrap_break_err!(BackendVariable::setVarStartValue(v.clone(), e.clone()), '__try0);
                if unwrap_break_err!(Flags::isSet(Flags::DEBUG_ALIAS.clone()), '__try0) {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("START: "));
                        __mm_s.push_str(
                            &*unwrap_break_err!(ComponentReferenceBasics::printComponentRefStr(&cr1), '__try0),
                        );
                        __mm_s.push_str(&*literal!(" = "));
                        __mm_s.push_str(&*unwrap_break_err!(ExpressionBasics::printExpStr(e.clone()), '__try0));
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                    unwrap_break_err!(BaseHashTable::dumpHashTable(&HTStartExpToInt), '__try0);
                }
            }
            tplExpIndList = unwrap_break_err!(BaseHashTable::hashTableList(&HTNominalExpToInt), '__try0);
            if !((tplExpIndList).is_empty()) {
                e = getDominantAttributeValue(&tplExpIndList);
                v = unwrap_break_err!(BackendVariable::setVarNominalValue(v.clone(), e.clone()), '__try0);
                if unwrap_break_err!(Flags::isSet(Flags::DEBUG_ALIAS.clone()), '__try0) {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NOMINAL: "));
                        __mm_s.push_str(
                            &*unwrap_break_err!(ComponentReferenceBasics::printComponentRefStr(&cr1), '__try0),
                        );
                        __mm_s.push_str(&*literal!(" = "));
                        __mm_s.push_str(&*unwrap_break_err!(ExpressionBasics::printExpStr(e.clone()), '__try0));
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                    unwrap_break_err!(BaseHashTable::dumpHashTable(&HTNominalExpToInt), '__try0);
                }
            }
            outVars = unwrap_break_err!(BackendVariable::setVarAt(outVars.clone(), i, v.clone()), '__try0);
        }
        Ok::<(), &'static str>(())
    }
    .is_err()
    {
        metamodelica::print(literal!(
            "\n++++++++++ Error in RemoveSimpleEquations.setAttributes ++++++++++\n"
        ));
    }
    outVars
}

fn getThisAttributes(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr_eq_lst: &metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<BackendDAE::Equation>,
    )>,
    mut inAliasVars: &BackendDAE::Variables,
    mut inHTStartExpToInt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inHTNominalExpToInt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> (
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) {
    let mut outHTStartExpToInt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    ) = inHTStartExpToInt;
    let mut outHTNominalExpToInt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    ) = inHTNominalExpToInt;
    (outHTStartExpToInt, outHTNominalExpToInt) = ({
        let mut e1: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 });
        'mc: {
            let __mc_input = &**cr_eq_lst;
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ metamodelica::ListNode::Nil => {
                        Ok((outHTStartExpToInt.clone(), outHTNominalExpToInt.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ metamodelica::ListNode::Cons { head: (cr1, _), tail: cr_eq_rest } => {
                        let mut cr_lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                        let mut res: metamodelica::Ref<DAE::Exp>;
                        let mut e: metamodelica::Ref<DAE::Exp>;
                        let mut e2: metamodelica::Ref<DAE::Exp>;
                        let mut v: metamodelica::Ref<BackendDAE::Var>;
                        let mut j: i32;
                        let mut j1: i32;
                        let mut outHTNominalExpToInt: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)) = outHTNominalExpToInt.clone();
                        let mut outHTStartExpToInt: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)) = outHTStartExpToInt.clone();
                        (v, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr1), inAliasVars)?;
                        e = BackendVariable::varBindExp(&v)?;
                        if BackendVariable::varHasStartValue(&v) {
                            res = BackendVariable::varStartValue(&v)?;
                            let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(cr1.clone())?, scalar: e.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone() }), Expression::crefExp(cr.clone())?, None)?) {
                                Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, .. } => __pa0.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            e1 = metamodelica::Own::own(__pa0);
                            let __pa1 = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: res.clone(), scalar: e.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone() }), Expression::crefExp(cr.clone())?, None)?) {
                                Deref @ BackendDAE::Equation::EQUATION { scalar: __pa1, .. } => __pa1.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            e2 = metamodelica::Own::own(__pa1);
                            (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                            if Expression::isZero(&e2)? {
                                e2 = metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
                            }
                            cr_lst = Expression::extractCrefsFromExp(e2.clone())?;
                            j = 2 - ((cr_lst).len() as i32);
                            j = j * ComponentReference::crefDepth(metamodelica::AsArg::as_arg(&cr1))?;
                            if BaseHashTable::hasKey(e2.clone(), &outHTStartExpToInt)? {
                                j1 = BaseHashTable::get(e2.clone(), &outHTStartExpToInt)?;
                                if j1 < j {
                                            j = j1;
                                }
                            }
                            outHTStartExpToInt = BaseHashTable::add((e2.clone(), j), outHTStartExpToInt.clone())?;
                            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("START: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr)?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                            }
                        }
                        if BackendVariable::varHasNominalValue(&v) {
                            e2 = BackendVariable::varNominalValue(&v)?;
                            cr_lst = Expression::extractCrefsFromExp(e2.clone())?;
                            j = 2 - ((cr_lst).len() as i32);
                            j = j * ComponentReference::crefDepth(metamodelica::AsArg::as_arg(&cr1))?;
                            if BaseHashTable::hasKey(e2.clone(), &outHTNominalExpToInt)? {
                                j1 = BaseHashTable::get(e2.clone(), &outHTNominalExpToInt)?;
                                if j1 < j {
                                            j = j1;
                                }
                            }
                            outHTNominalExpToInt = BaseHashTable::add((e2.clone(), j), outHTNominalExpToInt.clone())?;
                            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NOMINAL: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr)?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                            }
                        }
                        (outHTStartExpToInt, outHTNominalExpToInt) = getThisAttributes(cr, metamodelica::AsArg::as_arg(&cr_eq_rest), inAliasVars, outHTStartExpToInt.clone(), outHTNominalExpToInt.clone());
                        Ok(((outHTStartExpToInt.clone(), outHTNominalExpToInt.clone()), outHTNominalExpToInt.clone(), outHTStartExpToInt.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                outHTNominalExpToInt = __wb0;
                outHTStartExpToInt = __wb1;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        metamodelica::print(literal!("\n++++++++++ Error in RemoveSimpleEquations.getThisAttributes ++++++++++\n"));
                        Ok((outHTStartExpToInt.clone(), outHTNominalExpToInt.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        }
    });
    (outHTStartExpToInt, outHTNominalExpToInt)
}

fn getDominantAttributeValue(
    mut tplExpIndList: &metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outE: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 });
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut tpl: (metamodelica::Ref<DAE::Exp>, i32) =
        (<metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default(), 0);
    let mut i: i32;
    let mut j: i32 = 111111;
    for mut tpl in &**tplExpIndList {
        let mut tpl = tpl.clone();
        (e, i) = tpl;
        if i < j {
            outE = e;
            j = i;
        }
    }
    outE
}

fn dumpSimpleContainer(mut container: &SimpleContainer) -> ArcStr {
    let mut sOut: ArcStr;
    sOut = 'mc: {
        let __mc_input = container.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let SimpleContainer::ALIAS {
                cr1: mut cr1,
                negatedCr1: mut n1,
                i1: mut i1,
                cr2: mut cr2,
                negatedCr2: mut n2,
                i2: mut i2,
                eqnAttributes: _,
                visited: _,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = if (n1.clone()) { literal!("(-)") } else { literal!("") };
            s2 = if (n2.clone()) { literal!("(-)") } else { literal!("") };
            s1 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s1);
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&(cr1.clone()))?);
                ArcStr::from(__mm_s)
            };
            s2 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s2);
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&(cr2.clone()))?);
                ArcStr::from(__mm_s)
            };
            Ok({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("ALIASE: \t\t"));
                __mm_s.push_str(&*s1);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*s2);
                __mm_s.push_str(&*literal!("  ("));
                __mm_s.push_str(&*intString(i1.clone()));
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*intString(i2.clone()));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let SimpleContainer::PARAMETERALIAS {
                unknowncr: ref cr1,
                negatedCr1: mut n1,
                i1: mut i1,
                paramcr: ref cr2,
                negatedCr2: mut n2,
                i2: mut i2,
                eqnAttributes: _,
                visited: _,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = if (n1.clone()) { literal!("(-)") } else { literal!("") };
            s2 = if (n2.clone()) { literal!("(-)") } else { literal!("") };
            s1 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s1);
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&(cr1.clone()))?);
                ArcStr::from(__mm_s)
            };
            s2 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s2);
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&(cr2.clone()))?);
                ArcStr::from(__mm_s)
            };
            Ok({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("PARAMETERALIASE: \t"));
                __mm_s.push_str(&*s1);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*s2);
                __mm_s.push_str(&*literal!("  ("));
                __mm_s.push_str(&*intString(i1.clone()));
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*intString(i2.clone()));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let SimpleContainer::TIMEALIAS {
                cr1: mut cr1,
                negatedCr1: mut n1,
                i1: mut i1,
                cr2: mut cr2,
                negatedCr2: mut n2,
                i2: mut i2,
                eqnAttributes: _,
                visited: _,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = if (n1.clone()) { literal!("(-)") } else { literal!("") };
            s2 = if (n2.clone()) { literal!("(-)") } else { literal!("") };
            s1 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s1);
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&(cr1.clone()))?);
                ArcStr::from(__mm_s)
            };
            s2 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s2);
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&(cr2.clone()))?);
                ArcStr::from(__mm_s)
            };
            Ok({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("TIMEALIASE: \t"));
                __mm_s.push_str(&*s1);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*s2);
                __mm_s.push_str(&*literal!("  ("));
                __mm_s.push_str(&*intString(i1.clone()));
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*intString(i2.clone()));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let SimpleContainer::TIMEINDEPENTVAR {
                cr: ref cr1,
                i: _,
                exp: ref e,
                eqnAttributes: _,
                visited: _,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            Ok({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("TIMEINDEPENT: \t"));
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&(cr1.clone()))?);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?);
                ArcStr::from(__mm_s)
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(literal!("----------"))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    sOut
}
