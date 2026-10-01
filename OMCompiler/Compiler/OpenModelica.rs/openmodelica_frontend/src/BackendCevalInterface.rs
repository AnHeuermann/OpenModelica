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

use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::BackendInterface;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::Global;

#[derive(Clone, metamodelica::MMCtor, metamodelica::ReferenceEq)]
pub struct BackendInterfaceFunctions {
    pub cevalInteractiveFunctions: partialCevalInteractiveFunctions,
    pub cevalCallFunction: partialCevalCallFunction,
    pub elabCallInteractive: partialElabCallInteractive,
}

impl metamodelica::gc::MMTrace for BackendInterfaceFunctions {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.cevalInteractiveFunctions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.cevalCallFunction, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.elabCallInteractive, __mmv)?;
        Ok(())
    }
}
impl PartialEq for BackendInterfaceFunctions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq((&self.cevalInteractiveFunctions), (&other.cevalInteractiveFunctions))
            && std::sync::Arc::ptr_eq((&self.cevalCallFunction), (&other.cevalCallFunction))
            && std::sync::Arc::ptr_eq((&self.elabCallInteractive), (&other.elabCallInteractive))
    }
}
impl Eq for BackendInterfaceFunctions {}
impl PartialOrd for BackendInterfaceFunctions {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for BackendInterfaceFunctions {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (std::sync::Arc::as_ptr((&self.cevalInteractiveFunctions)) as *const ())
            .cmp(&(std::sync::Arc::as_ptr((&other.cevalInteractiveFunctions)) as *const ()))
            .then_with(|| {
                (std::sync::Arc::as_ptr((&self.cevalCallFunction)) as *const ())
                    .cmp(&(std::sync::Arc::as_ptr((&other.cevalCallFunction)) as *const ()))
            })
            .then_with(|| {
                (std::sync::Arc::as_ptr((&self.elabCallInteractive)) as *const ())
                    .cmp(&(std::sync::Arc::as_ptr((&other.elabCallInteractive)) as *const ()))
            })
    }
}
impl std::hash::Hash for BackendInterfaceFunctions {
    fn hash<__H: std::hash::Hasher>(&self, __state: &mut __H) {
        (std::sync::Arc::as_ptr((&self.cevalInteractiveFunctions)) as *const ()).hash(__state);
        (std::sync::Arc::as_ptr((&self.cevalCallFunction)) as *const ()).hash(__state);
        (std::sync::Arc::as_ptr((&self.elabCallInteractive)) as *const ()).hash(__state);
    }
}
impl std::fmt::Debug for BackendInterfaceFunctions {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("BackendInterfaceFunctions");
        __ds.field(
            "cevalInteractiveFunctions",
            &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.cevalInteractiveFunctions))),
        );
        __ds.field(
            "cevalCallFunction",
            &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.cevalCallFunction))),
        );
        __ds.field(
            "elabCallInteractive",
            &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.elabCallInteractive))),
        );
        __ds.finish()
    }
}

impl Default for BackendInterfaceFunctions {
    fn default() -> Self {
        Self {
            cevalInteractiveFunctions: {
                let __placeholder: partialCevalInteractiveFunctions = std::sync::Arc::new(|_, _, _, _, _| {
                    panic!("default-constructed placeholder fn must not be called")
                });
                __placeholder
            },
            cevalCallFunction: {
                let __placeholder: partialCevalCallFunction = std::sync::Arc::new(|_, _, _, _, _, _, _| {
                    panic!("default-constructed placeholder fn must not be called")
                });
                __placeholder
            },
            elabCallInteractive: {
                let __placeholder: partialElabCallInteractive = std::sync::Arc::new(|_, _, _, _, _, _, _, _| {
                    panic!("default-constructed placeholder fn must not be called")
                });
                __placeholder
            },
        }
    }
}

pub type BACKEND_INTERFACE_FUNCTIONS = BackendInterfaceFunctions;

pub fn initializeBackendInterface(mut inFunctions: BackendInterfaceFunctions) -> () {
    {
        let __v = inFunctions;
        crate::Globals::backendCevalInterface.with(|__root| *__root.borrow_mut() = __v)
    };
    ()
}

pub(crate) fn cevalInteractiveFunctions(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inMsg: Absyn::Msg,
    mut inNumIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut functions: BackendInterfaceFunctions;
    let mut func: partialCevalInteractiveFunctions;
    functions = crate::Globals::backendCevalInterface.with(|__root| __root.borrow().clone());
    func = functions.cevalInteractiveFunctions.clone();
    (outCache, outValue) = func(inCache, inEnv, inExp, inMsg, inNumIter)?;
    Ok((outCache, outValue))
}

pub(crate) fn cevalCallFunction(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inImplInst: bool,
    mut inMsg: Absyn::Msg,
    mut inNumIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut functions: BackendInterfaceFunctions;
    let mut func: partialCevalCallFunction;
    functions = crate::Globals::backendCevalInterface.with(|__root| __root.borrow().clone());
    func = functions.cevalCallFunction.clone();
    (outCache, outValue) = func(inCache, inEnv, inExp, inValues, inImplInst, inMsg, inNumIter)?;
    Ok((outCache, outValue))
}

pub(crate) fn elabCallInteractive(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut inExps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplInst: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut functions: BackendInterfaceFunctions;
    let mut func: partialElabCallInteractive;
    functions = crate::Globals::backendCevalInterface.with(|__root| __root.borrow().clone());
    func = functions.elabCallInteractive.clone();
    (outCache, outExp, outProperties) = func(
        inCache,
        inEnv,
        inCref,
        inExps,
        inNamedArgs,
        inImplInst,
        inPrefix,
        inInfo,
    )?;
    Ok((outCache, outExp, outProperties))
}

pub type partialCevalInteractiveFunctions = std::sync::Arc<
    dyn ::std::ops::Fn(
            FCore::Cache,
            FCore::Graph,
            metamodelica::Ref<DAE::Exp>,
            Absyn::Msg,
            i32,
        ) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)>
        + 'static,
>;

pub type partialCevalCallFunction = std::sync::Arc<
    dyn ::std::ops::Fn(
            FCore::Cache,
            FCore::Graph,
            metamodelica::Ref<DAE::Exp>,
            metamodelica::List<metamodelica::Ref<Values::Value>>,
            bool,
            Absyn::Msg,
            i32,
        ) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)>
        + 'static,
>;

pub type partialElabCallInteractive = std::sync::Arc<
    dyn ::std::ops::Fn(
            FCore::Cache,
            FCore::Graph,
            metamodelica::Ref<Absyn::ComponentRef>,
            metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
            metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
            bool,
            DAE::Prefix,
            SourceInfo,
        ) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
        + 'static,
>;
