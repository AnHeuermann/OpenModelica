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

use crate::SimCodeFunctionUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::HashTableCrIListArray;
use openmodelica_frontend_dump::HashTableCrILst;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

fn simulationFindLiterals(
    mut fns: metamodelica::List<DAE::Function>,
) -> Result<(
    metamodelica::List<DAE::Function>,
    (
        i32,
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
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
)> {
    let mut ofns: metamodelica::List<DAE::Function>;
    let mut literals: (
        i32,
        (
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
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    );
    (ofns, literals) = DAEUtil::traverseDAEFunctions(
        fns,
        (std::sync::Arc::new(SimCodeFunctionUtil::findLiteralsHelper)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            i32,
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
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::Exp>,
                                                metamodelica::Ref<DAE::Exp>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                            metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                        ),
                    ) -> Result<(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            i32,
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
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::Exp>,
                                                metamodelica::Ref<DAE::Exp>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                            metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                        ),
                    )> + 'static,
            >),
        (
            0,
            HashTableExpToIndex::emptyHashTableSized(BaseHashTable::bigBucketSize.clone()),
            metamodelica::nil(),
        ),
    )?;
    Ok((ofns, literals))
}

pub fn createFunctions(
    mut inProgram: &Absyn::Program,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<(
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<SimCodeFunction::RecordDeclaration>,
    metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>,
    (
        i32,
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
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
)> {
    let mut outLibs: metamodelica::List<ArcStr>;
    let mut outLibPaths: metamodelica::List<ArcStr>;
    let mut outIncludes: metamodelica::List<ArcStr>;
    let mut outIncludeDirs: metamodelica::List<ArcStr>;
    let mut outRecordDecls: metamodelica::List<SimCodeFunction::RecordDeclaration>;
    let mut outFunctions: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>;
    let mut outLiterals: (
        i32,
        (
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
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    );
    let mut funcelems: metamodelica::List<DAE::Function>;
    let mut lits: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    match '__try0: {
        funcelems = unwrap_break_err!(DAEUtil::getFunctionList(functionTree, false), '__try0);
        funcelems = Inline::inlineCallsInFunctions(
            funcelems.clone(),
            &((
                None,
                list![
                    openmodelica_frontend_types::DAE::InlineType::NORM_INLINE,
                    openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE
                ],
            )),
        );
        let (__pa1, ref __pa3 @ (_, _, ref __pa2)) =
            unwrap_break_err!(simulationFindLiterals(funcelems.clone()), '__try0);
        funcelems = metamodelica::Own::own(__pa1);
        lits = metamodelica::Own::own(__pa2);
        outLiterals = metamodelica::Own::own(__pa3);
        (
            outFunctions,
            outRecordDecls,
            outIncludes,
            outIncludeDirs,
            outLibs,
            outLibPaths,
        ) = unwrap_break_err!(SimCodeFunctionUtil::elaborateFunctions(inProgram, funcelems.clone(), &(metamodelica::nil()), &lits, metamodelica::nil()), '__try0);
        Ok::<_, &'static str>((
            funcelems.clone(),
            lits.clone(),
            outFunctions.clone(),
            outIncludeDirs.clone(),
            outIncludes.clone(),
            outLibPaths.clone(),
            outLibs.clone(),
            outLiterals.clone(),
            outRecordDecls.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8)) => {
            funcelems = __try0_o0;
            lits = __try0_o1;
            outFunctions = __try0_o2;
            outIncludeDirs = __try0_o3;
            outIncludes = __try0_o4;
            outLibPaths = __try0_o5;
            outLibs = __try0_o6;
            outLiterals = __try0_o7;
            outRecordDecls = __try0_o8;
        }
        Err(__try0_err) => {
            Error::addInternalError(
                literal!("Creation of Modelica functions failed."),
                metamodelica::sourceInfo!("SimCode/SimCodeUtilShared.mo"),
            )?;
            return Err(__try0_err);
        }
    }
    Ok((
        outLibs,
        outLibPaths,
        outIncludes,
        outIncludeDirs,
        outRecordDecls,
        outFunctions,
        outLiterals,
    ))
}

pub fn createVarToArrayIndexMapping(
    mut iModelInfo: &SimCode::ModelInfo,
) -> Result<(
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::List<i32>, metamodelica::Array<i32>),
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
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut oVarToArrayIndexMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::List<i32>, metamodelica::Array<i32>),
                )>,
            >,
        ),
        i32,
        (
            HashTableCrIListArray::FuncHashCref,
            HashTableCrIListArray::FuncCrefEqual,
            HashTableCrIListArray::FuncCrefStr,
            HashTableCrIListArray::FuncExpStr,
        ),
    );
    let mut oVarToIndexMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
        ),
        i32,
        (
            HashTableCrILst::FuncHashCref,
            HashTableCrILst::FuncCrefEqual,
            HashTableCrILst::FuncCrefStr,
            HashTableCrILst::FuncExpStr,
        ),
    );
    let mut sim_vars: SimCodeVar::SimVars;
    let mut vars: metamodelica::List<(metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>, i32)>;
    let mut table_size: i32 = 0;
    let mut var_lst: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut var_type: i32;
    let mut currentVarIndices: metamodelica::Array<i32>;
    if !(Flags::isSet(Flags::HPCOM.clone())?
        || listMember(
            Config::simCodeTarget()?,
            list![literal!("Cpp"), literal!("omsic"), literal!("omsicpp")],
        ))
    {
        oVarToArrayIndexMapping = HashTableCrIListArray::emptyHashTableSized(1);
        oVarToIndexMapping = HashTableCrILst::emptyHashTableSized(1);
        return Ok((oVarToArrayIndexMapping, oVarToIndexMapping));
    }
    sim_vars = iModelInfo.vars.clone();
    vars = list![
        (sim_vars.stateVars.clone(), 1),
        (sim_vars.derivativeVars.clone(), 1),
        (sim_vars.algVars.clone(), 1),
        (sim_vars.discreteAlgVars.clone(), 1),
        (sim_vars.intAlgVars.clone(), 2),
        (sim_vars.boolAlgVars.clone(), 3),
        (sim_vars.stringAlgVars.clone(), 4),
        (sim_vars.paramVars.clone(), 1),
        (sim_vars.intParamVars.clone(), 2),
        (sim_vars.boolParamVars.clone(), 3),
        (sim_vars.stringParamVars.clone(), 4),
        (sim_vars.constVars.clone(), 1),
        (sim_vars.intConstVars.clone(), 2),
        (sim_vars.boolConstVars.clone(), 3),
        (sim_vars.stringConstVars.clone(), 4),
        (sim_vars.realOptimizeConstraintsVars.clone(), 1),
        (sim_vars.realOptimizeFinalConstraintsVars.clone(), 1),
        (sim_vars.aliasVars.clone(), 1),
        (sim_vars.intAliasVars.clone(), 2),
        (sim_vars.boolAliasVars.clone(), 3),
        (sim_vars.stringAliasVars.clone(), 4)
    ];
    for mut vl in &*vars {
        (var_lst, _) = vl.clone();
        table_size = table_size + ((var_lst).len() as i32);
    }
    table_size = Util::nextPrime(
        ((metamodelica::OrderedFloat((table_size) as f64) * metamodelica::OrderedFloat(1.4_f64))
            .0
            .floor() as i32),
    );
    oVarToArrayIndexMapping = HashTableCrIListArray::emptyHashTableSized(table_size);
    oVarToIndexMapping = HashTableCrILst::emptyHashTableSized(table_size);
    currentVarIndices = arrayCreate(4, 1);
    for mut vl in &*vars {
        (var_lst, var_type) = vl.clone();
        (currentVarIndices, oVarToArrayIndexMapping, oVarToIndexMapping) = addVarToArrayIndexMappings(
            &var_lst,
            var_type,
            currentVarIndices.clone(),
            oVarToArrayIndexMapping,
            oVarToIndexMapping,
        )?;
    }
    Ok((oVarToArrayIndexMapping, oVarToIndexMapping))
}

pub fn addVarToArrayIndexMappings(
    mut vars: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut iVarType: i32,
    mut currentVarIndices: metamodelica::Array<i32>,
    mut varToArrayIndexMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::List<i32>, metamodelica::Array<i32>),
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
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut varToIndexMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<i32>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::List<i32>, metamodelica::Array<i32>),
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
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut currentVarIndices: metamodelica::Array<i32> = currentVarIndices;
    let mut varToArrayIndexMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::List<i32>, metamodelica::Array<i32>),
                )>,
            >,
        ),
        i32,
        (
            HashTableCrIListArray::FuncHashCref,
            HashTableCrIListArray::FuncCrefEqual,
            HashTableCrIListArray::FuncCrefStr,
            HashTableCrIListArray::FuncExpStr,
        ),
    ) = varToArrayIndexMapping;
    let mut varToIndexMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
        ),
        i32,
        (
            HashTableCrILst::FuncHashCref,
            HashTableCrILst::FuncCrefEqual,
            HashTableCrILst::FuncCrefStr,
            HashTableCrILst::FuncExpStr,
        ),
    ) = varToIndexMapping;
    for mut v in &**vars {
        (currentVarIndices, varToArrayIndexMapping, varToIndexMapping) = addVarToArrayIndexMapping(
            metamodelica::AsArg::as_arg(&v),
            iVarType,
            currentVarIndices.clone(),
            varToArrayIndexMapping,
            varToIndexMapping,
        )?;
    }
    Ok((currentVarIndices, varToArrayIndexMapping, varToIndexMapping))
}

pub fn addVarToArrayIndexMapping(
    mut iVar: &metamodelica::Ref<SimCodeVar::SimVar>,
    mut iVarType: i32,
    mut currentVarIndices: metamodelica::Array<i32>,
    mut varToArrayIndexMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::List<i32>, metamodelica::Array<i32>),
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
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut varToIndexMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<i32>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::List<i32>, metamodelica::Array<i32>),
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
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut currentVarIndices: metamodelica::Array<i32> = currentVarIndices;
    let mut varToArrayIndexMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::List<i32>, metamodelica::Array<i32>),
                )>,
            >,
        ),
        i32,
        (
            HashTableCrIListArray::FuncHashCref,
            HashTableCrIListArray::FuncCrefEqual,
            HashTableCrIListArray::FuncCrefStr,
            HashTableCrIListArray::FuncExpStr,
        ),
    ) = varToArrayIndexMapping;
    let mut varToIndexMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
        ),
        i32,
        (
            HashTableCrILst::FuncHashCref,
            HashTableCrILst::FuncCrefEqual,
            HashTableCrILst::FuncCrefStr,
            HashTableCrILst::FuncExpStr,
        ),
    ) = varToIndexMapping;
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    let mut arrayName: metamodelica::Ref<DAE::ComponentRef>;
    let mut varIdx: i32;
    let mut arrayIndex: i32;
    let mut varIndices: metamodelica::Array<i32>;
    let mut arrayDimensions: metamodelica::List<i32>;
    let mut numArrayElement: metamodelica::List<ArcStr>;
    let mut arraySubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let () = (match &**iVar {
        SimCodeVar::SimVar {
            name: __esc_name,
            numArrayElement: __esc_numArrayElement,
            ..
        } => {
            name = (*__esc_name).clone();
            numArrayElement = (*__esc_numArrayElement).clone();
            (currentVarIndices, varIdx) =
                getArrayIdxByVar(iVar, iVarType, &varToIndexMapping, currentVarIndices.clone())?;
            varToIndexMapping = BaseHashTable::add((name.clone(), list![varIdx]), varToIndexMapping)?;
            arraySubscripts = ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&name))?;
            if (numArrayElement).is_empty() || checkIfSubscriptsContainsUnhandlableIndices(&arraySubscripts) {
                arrayName = name.clone();
            } else {
                arrayName = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&name))?;
            }
            if isArrayVar(iVar) {
                arrayDimensions = ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut e in (List::lastN(numArrayElement.clone(), ((numArrayElement).len() as i32))?)
                        .into_iter()
                        .cloned()
                    {
                        let __x = stringInt(e.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                varIndices = arrayCreate(1, varIdx);
                varToArrayIndexMapping = BaseHashTable::add(
                    (arrayName, (arrayDimensions, varIndices.clone())),
                    varToArrayIndexMapping,
                )?;
            } else if ComponentReferenceBasics::crefEqual(&arrayName, metamodelica::AsArg::as_arg(&name))? {
                varIndices = arrayCreate(1, varIdx);
                varToArrayIndexMapping =
                    BaseHashTable::add((arrayName, (list![1], varIndices.clone())), varToArrayIndexMapping)?;
            } else {
                if BaseHashTable::hasKey(arrayName.clone(), &varToArrayIndexMapping)? {
                    (arrayDimensions, varIndices) = BaseHashTable::get(arrayName.clone(), &varToArrayIndexMapping)?;
                } else {
                    arrayDimensions = ({
                        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                        for mut e in (List::lastN(numArrayElement.clone(), ((arraySubscripts).len() as i32))?)
                            .into_iter()
                            .cloned()
                        {
                            let __x = stringInt(e.clone())?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    });
                    varIndices = arrayCreate(List::fold(&arrayDimensions, &fnptr!(intMul, i32, i32), 1)?, 0);
                }
                arrayIndex = getScalarElementIndex(&arraySubscripts, &arrayDimensions)?;
                varIndices = metamodelica::arrayUpdate(varIndices.clone(), arrayIndex, varIdx)?;
                varToArrayIndexMapping = BaseHashTable::add(
                    (arrayName, (arrayDimensions, varIndices.clone())),
                    varToArrayIndexMapping,
                )?;
            }
            ()
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!("Unknown case for addVarToArrayIndexMapping.\n")],
            )?;
            ()
        }
    });
    Ok((currentVarIndices, varToArrayIndexMapping, varToIndexMapping))
}

fn checkIfSubscriptsContainsUnhandlableIndices(
    mut iSubscripts: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> bool {
    let mut oContainsUnhandledSubscripts: bool = false;
    let mut subscript: metamodelica::Ref<DAE::Subscript> = metamodelica::Ref::new(DAE::Subscript::WHOLEDIM);
    for mut subscript in &**iSubscripts {
        let mut subscript = subscript.clone();
        if DAEUtil::getSubscriptIndex(&subscript) < 0 {
            oContainsUnhandledSubscripts = true;
            break;
        }
    }
    oContainsUnhandledSubscripts
}

fn getArrayIdxByVar(
    mut iVar: &metamodelica::Ref<SimCodeVar::SimVar>,
    mut iVarType: i32,
    mut iVarToIndexMapping: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iCurrentVarIndices: metamodelica::Array<i32>,
) -> Result<(metamodelica::Array<i32>, i32)> {
    let mut iCurrentVarIndices: metamodelica::Array<i32> = iCurrentVarIndices;
    let mut oVarIndex: i32;
    let mut varName: metamodelica::Ref<DAE::ComponentRef>;
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    let mut varIdx: i32;
    let mut tmpCurrentVarIndices: metamodelica::Array<i32>;
    oVarIndex = (::match_deref::match_deref! { match &((iVar.clone(), iCurrentVarIndices.clone())) {
        (Deref @ SimCodeVar::SimVar { name: __esc_name, aliasvar: SimCodeVar::AliasVariable::NOALIAS { .. }, .. }, __esc_tmpCurrentVarIndices) => {
            name = (*__esc_name).clone();
            tmpCurrentVarIndices = (*__esc_tmpCurrentVarIndices).clone();
            (varIdx, tmpCurrentVarIndices) = getVarToArrayIndexByType(iVar, iVarType, tmpCurrentVarIndices.clone())?;
            varIdx
        },
        (Deref @ SimCodeVar::SimVar { name: __esc_name, aliasvar: SimCodeVar::AliasVariable::NEGATEDALIAS { varName: __esc_varName }, .. }, _) => {
            name = (*__esc_name).clone();
            varName = (*__esc_varName).clone();
            if BaseHashTable::hasKey(varName.clone(), iVarToIndexMapping)? {
                let __pa0 = ::match_deref::match_deref! { match &(BaseHashTable::get(varName.clone(), iVarToIndexMapping)?) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                varIdx = metamodelica::Own::own(__pa0);
                varIdx = intMul(varIdx, -1);
            } else if ComponentReference::isTime(metamodelica::AsArg::as_arg(&varName)) {
                varIdx = 0;
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("Negated alias to unknown variable given.")])?;
                return Err("fail");
            }
            varIdx
        },
        (Deref @ SimCodeVar::SimVar { name: __esc_name, aliasvar: SimCodeVar::AliasVariable::ALIAS { varName: __esc_varName }, .. }, _) => {
            name = (*__esc_name).clone();
            varName = (*__esc_varName).clone();
            if BaseHashTable::hasKey(varName.clone(), iVarToIndexMapping)? {
                let __pa0 = ::match_deref::match_deref! { match &(BaseHashTable::get(varName.clone(), iVarToIndexMapping)?) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                varIdx = metamodelica::Own::own(__pa0);
            } else if ComponentReference::isTime(metamodelica::AsArg::as_arg(&varName)) {
                varIdx = 0;
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("Alias to unknown variable given.")])?;
                return Err("fail");
            }
            varIdx
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((iCurrentVarIndices, oVarIndex))
}

fn getVarToArrayIndexByType(
    mut iVar: &metamodelica::Ref<SimCodeVar::SimVar>,
    mut iVarType: i32,
    mut iCurrentVarIndices: metamodelica::Array<i32>,
) -> Result<(i32, metamodelica::Array<i32>)> {
    let mut oVarIdx: i32;
    let mut iCurrentVarIndices: metamodelica::Array<i32> = iCurrentVarIndices;
    match '__try0: {
        oVarIdx = unwrap_break_err!(metamodelica::arrayGet(iCurrentVarIndices.clone(), iVarType), '__try0);
        unwrap_break_err!(metamodelica::arrayUpdate(iCurrentVarIndices.clone(), iVarType, oVarIdx + unwrap_break_err!(getNumElems(iVar), '__try0)), '__try0);
        Ok::<_, &'static str>((oVarIdx.clone(),))
    } {
        Ok((__try0_o0,)) => {
            oVarIdx = __try0_o0;
        }
        Err(_) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!("GetVarToArrayIndexByType with unknown type called.")],
            )?;
            oVarIdx = -1;
        }
    }
    Ok((oVarIdx, iCurrentVarIndices))
}

pub fn getScalarElementIndex(
    mut arraySubscripts: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut arrayDimensions: &metamodelica::List<i32>,
) -> Result<i32> {
    let mut arrayIndex: i32;
    let mut idx: i32;
    let mut fac: i32;
    arrayIndex = 1;
    fac = 1;
    for mut i in ({
        let __s = ((arraySubscripts).len() as i32);
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        idx = DAEUtil::getSubscriptIndex(&((arraySubscripts).get(i)?));
        arrayIndex = arrayIndex + (idx - 1) * fac;
        fac = fac * (arrayDimensions).get(i)?;
    }
    Ok(arrayIndex)
}

pub fn getNumElems(mut var: &metamodelica::Ref<SimCodeVar::SimVar>) -> Result<i32> {
    let mut numElems: i32;
    numElems = (::match_deref::match_deref! { match var {
        Deref @ SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            numElems = 1;
            for mut d in &*var.numArrayElement.clone() {
                numElems = numElems * stringInt(d.clone())?;
            }
            numElems
        },
        _ => 1,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(numElems)
}

pub(crate) fn isArrayVar(mut var: &metamodelica::Ref<SimCodeVar::SimVar>) -> bool {
    let mut isArray: bool;
    isArray = (::match_deref::match_deref! { match var {
        Deref @ SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_ARRAY { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isArray
}
