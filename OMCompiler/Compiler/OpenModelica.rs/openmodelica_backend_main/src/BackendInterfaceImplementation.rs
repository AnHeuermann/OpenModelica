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

use crate::CevalScript;
use crate::StaticScript;
use openmodelica_ast::Absyn;
use openmodelica_backend::RewriteRules;
use openmodelica_backend::SymbolTable;
use openmodelica_frontend::BackendCevalInterface;
use openmodelica_frontend::InstHashTable;
use openmodelica_frontend_dump::BackendInterface;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;

pub(crate) fn initializeBackendInterface() -> () {
    BackendInterface::initializeBackendInterface(BackendInterface::BackendInterfaceFunctions {
        noRewriteRulesFrontEnd: (std::sync::Arc::new(fnptr!(noRewriteRulesFrontEnd))
            as std::sync::Arc<dyn ::std::ops::Fn() -> Result<bool> + 'static>),
        rewriteFrontEnd: (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Exp>| rewriteFrontEnd(&__a0))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<(metamodelica::Ref<Absyn::Exp>, bool)>
                    + 'static,
            >),
        appendLibrary: (std::sync::Arc::new(appendLibrary)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, ArcStr) -> Result<(Absyn::Program, bool)> + 'static,
            >),
        initInstHashTable: (std::sync::Arc::new(InstHashTable::init)
            as std::sync::Arc<dyn ::std::ops::Fn() -> Result<()> + 'static>),
    });
    BackendCevalInterface::initializeBackendInterface(BackendCevalInterface::BackendInterfaceFunctions {
        cevalInteractiveFunctions: (std::sync::Arc::new(
            move |__a0: FCore::Cache,
                  __a1: FCore::Graph,
                  __a2: metamodelica::Ref<DAE::Exp>,
                  __a3: Absyn::Msg,
                  __a4: i32| cevalInteractiveFunctions(__a0, __a1, &__a2, __a3, __a4),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        FCore::Cache,
                        FCore::Graph,
                        metamodelica::Ref<DAE::Exp>,
                        Absyn::Msg,
                        i32,
                    ) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)>
                    + 'static,
            >),
        cevalCallFunction: (std::sync::Arc::new(cevalCallFunction)
            as std::sync::Arc<
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
            >),
        elabCallInteractive: (std::sync::Arc::new(elabCallInteractive)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        FCore::Cache,
                        FCore::Graph,
                        metamodelica::Ref<Absyn::ComponentRef>,
                        metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
                        metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
                        bool,
                        DAE::Prefix,
                        SourceInfo,
                    )
                        -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                    + 'static,
            >),
    });
    ()
}

fn cevalInteractiveFunctions(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inMsg: Absyn::Msg,
    mut inNumIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = CevalScript::cevalInteractiveFunctions(inCache, inEnv, inExp, inMsg, inNumIter)?;
    Ok((outCache, outValue))
}

fn cevalCallFunction(
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
    (outCache, outValue) =
        CevalScript::cevalCallFunction(inCache, inEnv, inExp, inValues, inImplInst, inMsg, inNumIter)?;
    Ok((outCache, outValue))
}

fn elabCallInteractive(
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
    (outCache, outExp, outProperties) = StaticScript::elabCallInteractive(
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

fn noRewriteRulesFrontEnd() -> bool {
    let mut noRules: bool;
    noRules = RewriteRules::noRewriteRulesFrontEnd();
    noRules
}

fn rewriteFrontEnd(mut inExp: &metamodelica::Ref<Absyn::Exp>) -> Result<(metamodelica::Ref<Absyn::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut isChanged: bool;
    (outExp, isChanged) = RewriteRules::rewriteFrontEnd(inExp)?;
    Ok((outExp, isChanged))
}

fn appendLibrary(
    mut modelName: metamodelica::Ref<Absyn::Path>,
    mut modelicaPath: ArcStr,
) -> Result<(Absyn::Program, bool)> {
    let mut program: Absyn::Program;
    let mut success: bool;
    program = SymbolTable::getAbsyn();
    (program, success) = CevalScript::loadModel(
        &(list![(modelName, literal!(""), list![literal!("default")], false)]),
        modelicaPath,
        program,
        true,
        true,
        true,
        false,
        false,
        literal!(""),
    )?;
    SymbolTable::setAbsyn(program.clone())?;
    Ok((program, success))
}
