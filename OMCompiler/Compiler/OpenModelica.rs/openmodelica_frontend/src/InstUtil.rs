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

use crate::Ceval;
use crate::FGraph;
use crate::FNode;
use crate::HashSet;
use crate::InnerOuter;
use crate::Inst;
use crate::InstExtends;
use crate::InstFunction;
use crate::Lookup;
use crate::Mod;
use crate::NFSCodeFlatten;
use crate::Patternm;
use crate::PrefixUtil;
use crate::Static;
use crate::UnitAbsyn;
use crate::UnitAbsynBuilder;
use crate::UnitChecker;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::HashTable5;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlSetCR;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::InstBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Graph;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

/// an identifier
pub type Ident = ArcStr;

/// an instance hierarchy
pub type InstanceHierarchy = metamodelica::List<InnerOuter::TopInstance>;

pub type InstDims = metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>;

pub(crate) fn newIdent() -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut i: i32;
    let mut is: ArcStr;
    let mut s: ArcStr;
    i = tick();
    is = intString(i);
    s = stringAppend(literal!("__TMP__"), is);
    outComponentRef = ComponentReferenceBasics::makeCrefIdent(s, DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
    outComponentRef
}

fn isNotFunction(mut cls: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut res: bool;
    res = SCodeUtil::isFunction(cls);
    res = boolNot(res);
    res
}

pub(crate) fn scodeFlatten(
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    outProgram = 'mc: {
        let __mc_input = &*inPath;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (Flags::isSet(Flags::DO_SCODE_DEP.clone())?) else { return Err("pattern mismatch") };
                    Ok(inProgram.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { name: Deref @ "" } => {
                    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>> = outProgram.clone();
                    outProgram = scodeFlattenProgram(inProgram.clone());
                    Ok((outProgram.clone(), outProgram.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outProgram = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>> = outProgram.clone();
                    (outProgram, _) = NFSCodeFlatten::flattenClassInProgram(inPath.clone(), inProgram.clone())?;
                    Ok((outProgram.clone(), outProgram.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outProgram = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outProgram)
}

fn scodeFlattenProgram(
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> metamodelica::List<metamodelica::Ref<SCode::Element>> {
    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    outProgram = 'mc: {
        let __mc_input = &*inProgram;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>> = outProgram.clone();
                    ErrorExt::setCheckpoint(literal!("scodeFlattenProgram"));
                    outProgram = NFSCodeFlatten::flattenCompleteProgram(inProgram.clone())?;
                    ErrorExt::delCheckpoint(literal!("scodeFlattenProgram"));
                    Ok((outProgram.clone(), outProgram.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outProgram = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::rollBack(literal!("scodeFlattenProgram"));
                    Ok(inProgram.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outProgram
}

pub(crate) fn reEvaluateInitialIfEqns(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut dae: DAE::DAElist,
    mut isTopCall: bool,
) -> Result<DAE::DAElist> {
    let mut odae: DAE::DAElist;
    odae = (match (dae.clone(), isTopCall) {
        (DAE::DAElist { elementLst: ref elems }, true) => {
            let mut elems = elems.clone();
            elems = List::fold2r(
                metamodelica::AsArg::as_arg(&elems),
                &fnptr!(
                    reEvaluateInitialIfEqns2,
                    metamodelica::List<metamodelica::Ref<DAE::Element>>,
                    metamodelica::Ref<DAE::Element>,
                    FCore::Cache,
                    FCore::Graph
                ),
                cache,
                env,
                metamodelica::nil(),
            )?
            .reverse();
            DAE::DAElist {
                elementLst: elems.clone(),
            }
        }
        (_, false) => dae,
        _ => return Err("match: no arm matched"),
    });
    Ok(odae)
}

fn reEvaluateInitialIfEqns2(
    mut acc: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut elem: metamodelica::Ref<DAE::Element>,
    mut inCache: FCore::Cache,
    mut env: FCore::Graph,
) -> metamodelica::List<metamodelica::Ref<DAE::Element>> {
    let mut oelems: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    oelems = 'mc: {
        let __mc_input = (&*elem, inCache);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: conds, equations2: tbs, equations3: fb, .. }, cache) => {
                    let mut valList: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut selectedBranch: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut blist: metamodelica::List<bool>;
                    (_, valList) = Ceval::cevalList(cache.clone(), env.clone(), conds.clone(), true, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    blist = List::map(valList.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::valueBool(&__a0))?;
                    selectedBranch = List::findBoolList(&blist, tbs.clone(), fb.clone())?;
                    selectedBranch = makeDAEElementInitial(&selectedBranch)?;
                    Ok(listAppend(selectedBranch.clone(), acc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::cons(elem.clone(), acc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oelems
}

fn makeDAEElementInitial(
    mut inElems: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outElems: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    outElems = (::match_deref::match_deref! { match inElems {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cr, exp: e1, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::INITIALDEFINE { componentRef: cr.clone(), exp: e1.clone(), source: s.clone() }), outElems)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { dimension: dims, exp: e1, array: e2, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: dims.clone(), exp: e1.clone(), array: e2.clone(), source: s.clone() }), outElems)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::INITIALEQUATION { exp1: e1.clone(), exp2: e2.clone(), source: s.clone() }), outElems)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::IF_EQUATION { condition1: expl, equations2: tbs, equations3: fb, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::INITIAL_IF_EQUATION { condition1: expl.clone(), equations2: tbs.clone(), equations3: fb.clone(), source: s.clone() }), outElems)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: al, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::INITIALALGORITHM { algorithm_: al.clone(), source: s.clone() }), outElems)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, source: s }, tail: elems } => {
            outElems = makeDAEElementInitial(elems)?;
            metamodelica::cons(metamodelica::Ref::new(DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1.clone(), rhs: e2.clone(), source: s.clone() }), outElems)
        },
        Deref @ metamodelica::ListNode::Cons { head: elem, tail: elems } => {
            outElems = makeDAEElementInitial(elems)?;
            metamodelica::cons(elem.clone(), outElems)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElems)
}

pub(crate) fn lookupTopLevelClass(
    mut inName: ArcStr,
    mut inProgram: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inPrintError: bool,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outClass: metamodelica::Ref<SCode::Element>;
    outClass = 'mc: {
        let __mc_input = inPrintError;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut cls: metamodelica::Ref<SCode::Element>;
            cls = List::getMemberOnTrue(
                inName.clone(),
                inProgram,
                &move |__a0: ArcStr, __a1: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(SCodeUtil::isClassNamed(&__a0, &__a1))
                },
            )?;
            Ok(cls.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let true = __mc_input.clone() else {
                return Err("nomatch");
            };
            Error::addMessage(Error::LOAD_MODEL_ERROR.clone(), list![inName.clone()])?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outClass)
}

pub(crate) fn fixInstClassType(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut isPartialFn: bool,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (&*ty, isPartialFn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::TYPE { path: path1 }, .. }, _) => {
                    let mut name: ArcStr;
                    let mut path2: metamodelica::Ref<Absyn::Path>;
                    name = AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path1));
                    path2 = AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&path1))?;
                    ::match_deref::match_deref! { match &(AbsynUtil::pathLastIdent(&path2)) {
                        Deref @ "$Code" => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    path2 = AbsynUtil::stripLast(&path2)?;
                    ::match_deref::match_deref! { match &(AbsynUtil::pathLastIdent(&path2)) {
                        Deref @ "OpenModelica" => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(Util::assoc(name.clone(), list![(literal!("Expression"), metamodelica::Ref::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_EXPRESSION })), (literal!("ExpressionOrModification"), metamodelica::Ref::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_EXPRESSION_OR_MODIFICATION })), (literal!("TypeName"), metamodelica::Ref::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_TYPENAME })), (literal!("VariableName"), metamodelica::Ref::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_VARIABLENAME })), (literal!("VariableNames"), metamodelica::Ref::new(DAE::Type::T_CODE { ty: openmodelica_frontend_types::DAE::CodeType::C_VARIABLENAMES }))])?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, false) => {
                    Ok(ty.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, true) => {
                    Ok(Types::makeFunctionPolymorphicReference(&ty)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outType)
}

pub(crate) fn updateEnumerationEnvironment(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inClass: &metamodelica::Ref<SCode::Element>,
    mut inCi_State: &ClassInf::State,
) -> (FCore::Cache, FCore::Graph) {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inType, inCi_State);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Type::T_ENUMERATION { names, literalVarLst: vars, path: p, .. }, ClassInf::State::ENUMERATION { path: pname }) => {
                    let mut env_1: FCore::Graph;
                    let mut cache = (*cache).clone();
                    (cache, env_1) = updateEnumerationEnvironment1(cache.clone(), env.clone(), AbsynUtil::pathString(pname.clone(), literal!("."), true, false)?, names.clone(), vars.clone(), p.clone())?;
                    Ok((cache.clone(), env_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _) => {
                    Ok((cache.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outCache, outEnv)
}

fn updateEnumerationEnvironment1(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inName: ArcStr,
    mut inNames: metamodelica::List<ArcStr>,
    mut inVars: metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<(FCore::Cache, FCore::Graph)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCache, inEnv, inNames, inVars, inPath)) {
            (cache, env, Deref @ metamodelica::ListNode::Cons { head: nn, tail: names }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { ty, .. }, tail: vars }, p) => {
                let mut env_1: FCore::Graph;
                let mut env_2: FCore::Graph;
                let mut compenv: FCore::Graph;
                let mut var: metamodelica::Ref<DAE::Var>;
                let mut cache = (*cache).clone();
                (cache, var, _, _, _, compenv) = Lookup::lookupIdentLocal(cache.clone(), metamodelica::AsArg::as_arg(&env), nn.clone())?;
                assign_field!(var.ty = ty.clone());
                env_1 = FGraph::updateComp(env.clone(), var.clone(), &(openmodelica_frontend_dump::FCore::Status::VAR_DAE), &compenv);
                { (inCache, inEnv, inName, inNames, inVars, inPath) = (cache.clone(), env_1, var.name.clone(), names.clone(), vars.clone(), p.clone()); continue '__tco; }
            },
            (cache, env, Deref @ metamodelica::ListNode::Nil, _, _) => {
                return Ok((cache.clone(), env.clone()))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn updateDeducedUnits(
    mut callScope: bool,
    mut store: &UnitAbsyn::InstStore,
    mut dae: DAE::DAElist,
) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (match (callScope, store.clone(), dae.clone()) {
        (
            true,
            UnitAbsyn::InstStore::INSTSTORE {
                store:
                    UnitAbsyn::Store {
                        storeVector: mut vec,
                        numElts: _,
                    },
                ht: mut ht,
                checkResult: _,
            },
            DAE::DAElist { elementLst: ref elts },
        ) => {
            let mut elts = elts.clone();
            elts = List::map2(
                elts.clone(),
                &move |__a0: metamodelica::Ref<DAE::Element>,
                       __a1: metamodelica::Array<Option<UnitAbsyn::Unit>>,
                       __a2: (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
                        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                    ),
                )|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(updateDeducedUnits2(__a0, __a1, &__a2))
                },
                vec.clone(),
                ht.clone(),
            )?;
            DAE::DAElist {
                elementLst: elts.clone(),
            }
        }
        _ => dae,
    });
    Ok(outDae)
}

fn updateDeducedUnits2(
    mut elt: metamodelica::Ref<DAE::Element>,
    mut vec: metamodelica::Array<Option<UnitAbsyn::Unit>>,
    mut ht: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> metamodelica::Ref<DAE::Element> {
    let __ab_vec = vec.borrow();
    let mut oelt: metamodelica::Ref<DAE::Element>;
    oelt = 'mc: {
        let __mc_input = &*elt;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cr, variableAttributesOption: varOpt @ Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { unit: None, .. }), .. } => {
                    let mut indx: i32;
                    let mut unitStr: ArcStr;
                    let mut unit: UnitAbsyn::Unit;
                    let mut varOpt = (*varOpt).clone();
                    indx = BaseHashTable::get(cr.clone(), ht)?;
                    let __pa0 = ::match_deref::match_deref! { match &((*metamodelica::index_checked(&__ab_vec, indx)?).clone()) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    unit = metamodelica::Own::own(__pa0);
                    unitStr = UnitAbsynBuilder::unit2str(&unit)?;
                    varOpt = DAEUtil::setUnitAttr(varOpt.clone(), metamodelica::Ref::new(DAE::Exp::SCONST { string: unitStr.clone() }))?;
                    Ok(DAEUtil::setVariableAttributes(elt.clone(), varOpt.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(elt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oelt
}

pub(crate) fn reportUnitConsistency(mut topScope: bool, mut store: &UnitAbsyn::InstStore) -> () {
    let () = 'mc: {
        let __mc_input = (topScope, store.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                true,
                UnitAbsyn::InstStore::INSTSTORE {
                    store: mut st,
                    ht: _,
                    checkResult: Some(UnitAbsyn::UnitCheckResult::CONSISTENT { .. }),
                },
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut complete: bool;
            (complete, _) = UnitChecker::isComplete(&(st.clone()))?;
            Error::addMessage(
                if (complete) {
                    Error::CONSISTENT_UNITS.clone()
                } else {
                    Error::INCOMPLETE_UNITS.clone()
                },
                metamodelica::nil(),
            )?;
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn extractConnectorPrefix(
    mut connectorRef: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut prefixCon: metamodelica::Ref<DAE::ComponentRef>;
    prefixCon = 'mc: {
        let __mc_input = &**connectorRef;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ } => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: ty @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: _, isExpandable: _ }, .. }, subscriptLst: subs, componentRef: _ } => {
                    Ok(ComponentReferenceBasics::makeCrefIdent(name.clone(), ty.clone(), subs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: ty, subscriptLst: subs, componentRef: child } => {
                    let mut child = (*child).clone();
                    child = extractConnectorPrefix(metamodelica::AsArg::as_arg(&child))?;
                    Ok(ComponentReferenceBasics::makeCrefQual(name.clone(), ty.clone(), subs.clone(), child.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(prefixCon)
}

fn updateCrefTypesWithConnectorPrefix(
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
    outCref = 'mc: {
        let __mc_input = (&**cr1, &**cr2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, identType: ty, subscriptLst: subs }, Deref @ DAE::ComponentRef::CREF_QUAL { ident: name2, identType: _, subscriptLst: _, componentRef: child2 }) => {
                    let true = (stringEq(&name, &name2)) else { return Err("pattern mismatch") };
                    Ok(ComponentReferenceBasics::makeCrefQual(name.clone(), ty.clone(), subs.clone(), child2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: ty, subscriptLst: subs, componentRef: child }, Deref @ DAE::ComponentRef::CREF_QUAL { ident: name2, identType: _, subscriptLst: _, componentRef: child2 }) => {
                    let mut outCref: metamodelica::Ref<DAE::ComponentRef> = outCref.clone();
                    let true = (stringEq(&name, &name2)) else { return Err("pattern mismatch") };
                    outCref = updateCrefTypesWithConnectorPrefix(metamodelica::AsArg::as_arg(&child), metamodelica::AsArg::as_arg(&child2))?;
                    Ok((ComponentReferenceBasics::makeCrefQual(name.clone(), ty.clone(), subs.clone(), outCref.clone()), outCref.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCref = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ***** FAILURE with ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr1)?); __mm_s.push_str(&*literal!(" _and_ ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr2)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCref)
}

fn checkClassEqual(mut c1: metamodelica::Ref<SCode::Element>, mut c2: metamodelica::Ref<SCode::Element>) -> bool {
    let mut areEqual: bool;
    areEqual = 'mc: {
        let __mc_input = (&*c1, &*c2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    if !((Config::acceptMetaModelicaGrammar()? && !(c1.clone() == c2.clone()))) { return Err("guard") }
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_TYPE { .. }, .. }, _) => {
                    if !((!(c1.clone() == c2.clone()))) { return Err("guard") }
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { restriction: r, .. }, _) => {
                    let false = (SCodeUtil::isFunctionRestriction(metamodelica::AsArg::as_arg(&r))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { normalAlgorithmLst: normalAlgorithmLst1, initialAlgorithmLst: initialAlgorithmLst1, .. }, .. }, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { normalAlgorithmLst: normalAlgorithmLst2, initialAlgorithmLst: initialAlgorithmLst2, .. }, .. }) => {
                    let true = (intEq(((normalAlgorithmLst1).len() as i32), ((normalAlgorithmLst2).len() as i32))) else { return Err("pattern mismatch") };
                    let true = (intEq(((initialAlgorithmLst1).len() as i32), ((initialAlgorithmLst2).len() as i32))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { classDef: cd1 @ Deref @ SCode::ClassDef::DERIVED { .. }, .. }, Deref @ SCode::Element::CLASS { classDef: cd2 @ Deref @ SCode::ClassDef::DERIVED { .. }, .. }) => {
                    Ok(cd1.clone() == cd2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    areEqual
}

pub(crate) fn prefixEqualUnlessBasicType(
    mut pre1: DAE::Prefix,
    mut pre2: DAE::Prefix,
    mut cls: &metamodelica::Ref<SCode::Element>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**cls;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION { .. }, .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_ENUMERATION { .. }, .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_INTEGER { .. }, .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_REAL { .. }, .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_STRING { .. }, .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_BOOLEAN { .. }, .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_CLOCK { .. }, .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: idn, .. } => {
                    if !((metamodelica::stringEq(&idn, &(literal!("Real"))) || metamodelica::stringEq(&idn, &(literal!("Integer"))) || metamodelica::stringEq(&idn, &(literal!("String"))) || metamodelica::stringEq(&idn, &(literal!("Boolean"))))) { return Err("guard") }
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "Clock", .. } => {
                    let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
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
                    let true = (pre1.clone() == pre2.clone()) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

pub(crate) fn isBuiltInClass(mut className: &ArcStr) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(className.clone()) {
        Deref @ "Real" => true,
        Deref @ "Integer" => true,
        Deref @ "String" => true,
        Deref @ "Boolean" => true,
        Deref @ "Clock" => Config::synchronousFeaturesAllowed()?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn equalityConstraintOutputDimension<'__b>(
    mut inElements: &'__b metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> i32 {
    '__tco: loop {
        ::match_deref::match_deref! { match inElements {
            Deref @ metamodelica::ListNode::Nil => {
                return 0
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::OUTPUT { .. }, arrayDims: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: Deref @ Absyn::Exp::INTEGER { value: dim } }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, tail: _ } => {
                return dim.clone()
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: tail } => {
                let mut dim: i32;
                { inElements = tail; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn equalityConstraint(
    mut inEnv: &FCore::Graph,
    mut inCdefelts: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut info: &SourceInfo,
) -> Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)> {
    let mut outResult: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)> = None;
    let mut els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut dimension: i32;
    let mut inlineType: DAE::InlineType;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(FGraph::getScopePath(inEnv), '__try0)) {
            Some(__pa1) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        path = metamodelica::Own::own(__pa1);
        path = unwrap_break_err!(AbsynUtil::joinPaths(path.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("equalityConstraint") })), '__try0);
        path = AbsynUtil::makeFullyQualified(path.clone());
        Ok::<_, &'static str>((path.clone(),))
    } {
        Ok((__try0_o0,)) => {
            path = __try0_o0;
        }
        Err(_) => {
            return outResult;
        }
    }
    for mut el in &**inCdefelts {
        if '__try2: {
            let __pa3 = ::match_deref::match_deref! { match &(el.clone()) {
                Deref @ SCode::Element::CLASS { name: Deref @ "equalityConstraint", restriction: SCode::Restriction::R_FUNCTION { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa3, .. }, .. } => __pa3.clone(),
                _ => break '__try2 Err::<_, _>("pattern mismatch"),
            } };
            els = metamodelica::Own::own(__pa3);
            dimension = equalityConstraintOutputDimension(&els);
            inlineType = classIsInlineFunc(metamodelica::AsArg::as_arg(&el));
            outResult = Some((path.clone(), dimension, inlineType));
            return outResult;
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    outResult
}

pub(crate) fn handleUnitChecking(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut inStore: UnitAbsyn::InstStore,
    mut pre: &DAE::Prefix,
    mut compDAE: &DAE::DAElist,
    mut daes: &metamodelica::List<DAE::DAElist>,
    mut className: &ArcStr,
) -> Result<(FCore::Cache, FCore::Graph, UnitAbsyn::InstStore)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outStore: UnitAbsyn::InstStore;
    (outCache, outEnv, outStore) = (match inStore {
        mut store => (cache, env, store),
        mut store => {
            let mut daetemp: DAE::DAElist;
            let mut ut: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
            daetemp = DAEUtil::joinDaeLst(daes)?;
            (store, ut) = UnitAbsynBuilder::instBuildUnitTerms(&env, &daetemp, compDAE, &store)?;
            UnitAbsynBuilder::registerUnitWeights(cache.clone(), env.clone(), compDAE)?;
            store = UnitChecker::check(&ut, store);
            (cache, env, store)
        }
    });
    Ok((outCache, outEnv, outStore))
}

fn checkExtendsRestrictionMatch(mut r1: &SCode::Restriction, mut r2: &SCode::Restriction) -> Result<()> {
    let () = (match (r1.clone(), r2.clone()) {
        (SCode::Restriction::R_PACKAGE { .. }, SCode::Restriction::R_PACKAGE { .. }) => (),
        (
            SCode::Restriction::R_FUNCTION {
                functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ },
            },
            SCode::Restriction::R_FUNCTION {
                functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ },
            },
        ) => (),
        (
            SCode::Restriction::R_FUNCTION {
                functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: _ },
            },
            SCode::Restriction::R_FUNCTION {
                functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ },
            },
        ) => (),
        (
            SCode::Restriction::R_FUNCTION {
                functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. },
            },
            SCode::Restriction::R_FUNCTION {
                functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ },
            },
        ) => (),
        (
            SCode::Restriction::R_FUNCTION {
                functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. },
            },
            SCode::Restriction::R_FUNCTION {
                functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. },
            },
        ) => (),
        (SCode::Restriction::R_TYPE { .. }, SCode::Restriction::R_TYPE { .. }) => (),
        (SCode::Restriction::R_RECORD { isOperator: _ }, SCode::Restriction::R_RECORD { isOperator: _ }) => (),
        (SCode::Restriction::R_CONNECTOR { isExpandable: _ }, SCode::Restriction::R_TYPE { .. }) => (),
        (SCode::Restriction::R_CONNECTOR { isExpandable: _ }, SCode::Restriction::R_RECORD { isOperator: _ }) => (),
        (SCode::Restriction::R_CONNECTOR { isExpandable: _ }, SCode::Restriction::R_CONNECTOR { isExpandable: _ }) => {
            ()
        }
        (SCode::Restriction::R_BLOCK { .. }, SCode::Restriction::R_RECORD { isOperator: false }) => (),
        (SCode::Restriction::R_BLOCK { .. }, SCode::Restriction::R_BLOCK { .. }) => (),
        (SCode::Restriction::R_MODEL { .. }, SCode::Restriction::R_RECORD { isOperator: false }) => (),
        (SCode::Restriction::R_MODEL { .. }, SCode::Restriction::R_BLOCK { .. }) => (),
        (SCode::Restriction::R_MODEL { .. }, SCode::Restriction::R_MODEL { .. }) => (),
        (SCode::Restriction::R_MODEL { .. }, SCode::Restriction::R_CLASS { .. }) => (),
        (SCode::Restriction::R_CLASS { .. }, SCode::Restriction::R_MODEL { .. }) => (),
        (SCode::Restriction::R_CLASS { .. }, SCode::Restriction::R_RECORD { isOperator: _ }) => (),
        (SCode::Restriction::R_CLASS { .. }, SCode::Restriction::R_BLOCK { .. }) => (),
        (SCode::Restriction::R_CLASS { .. }, SCode::Restriction::R_CLASS { .. }) => (),
        (SCode::Restriction::R_OPERATOR { .. }, SCode::Restriction::R_OPERATOR { .. }) => (),
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn checkExtendsForTypeRestiction(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inRestriction: SCode::Restriction,
    mut inSCodeElementLst: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inRestriction, &**inSCodeElementLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: id }, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let true = (listMember(r.clone(), list![openmodelica_frontend_types::SCode::Restriction::R_TYPE, SCode::Restriction::R_CONNECTOR { isExpandable: false }, SCode::Restriction::R_CONNECTOR { isExpandable: true }])) else { return Err("pattern mismatch") };
                    let true = (listMember(id.clone(), list![literal!("Real"), literal!("Integer"), literal!("Boolean"), literal!("String")])) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: id }, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
                    let true = (listMember(r.clone(), list![openmodelica_frontend_types::SCode::Restriction::R_TYPE, SCode::Restriction::R_CONNECTOR { isExpandable: false }, SCode::Restriction::R_CONNECTOR { isExpandable: true }])) else { return Err("pattern mismatch") };
                    let true = (listMember(id.clone(), list![literal!("Real"), literal!("Integer"), literal!("Boolean"), literal!("String"), literal!("Clock")])) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: p, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupClass(inCache, inEnv, metamodelica::AsArg::as_arg(&p), None), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r1, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: p, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut r2: SCode::Restriction;
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupClass(inCache, inEnv, metamodelica::AsArg::as_arg(&p), None)?) {
                        (_, Deref @ SCode::Element::CLASS { restriction: __pa0, .. }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    r2 = metamodelica::Own::own(__pa0);
                    checkExtendsRestrictionMatch(metamodelica::AsArg::as_arg(&r1), &r2)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r1, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: p, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut r2: SCode::Restriction;
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupClass(inCache, inEnv, metamodelica::AsArg::as_arg(&p), None)?) {
                        (_, Deref @ SCode::Element::CLASS { restriction: __pa0, .. }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    r2 = metamodelica::Own::own(__pa0);
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error!: ")); __mm_s.push_str(&*SCodeDump::restrString(metamodelica::AsArg::as_arg(&r1))?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*FGraph::printGraphPathStr(inEnv)); __mm_s.push_str(&*literal!(" cannot be extended by ")); __mm_s.push_str(&*SCodeDump::restrString(&r2)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" due to derived/base class restrictions.\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

pub(crate) fn checkDerivedRestriction(
    mut parentRestriction: SCode::Restriction,
    mut childRestriction: SCode::Restriction,
    mut childName: ArcStr,
) -> Result<bool> {
    let mut b: bool;
    let mut b1: bool;
    let mut b2: bool;
    let mut b3: bool;
    let mut b4: bool;
    let mut strLst: metamodelica::List<ArcStr>;
    let mut rstLst: metamodelica::List<SCode::Restriction>;
    strLst = if (Config::synchronousFeaturesAllowed()?) {
        list![
            literal!("Real"),
            literal!("Integer"),
            literal!("String"),
            literal!("Boolean"),
            literal!("Clock")
        ]
    } else {
        list![
            literal!("Real"),
            literal!("Integer"),
            literal!("String"),
            literal!("Boolean")
        ]
    };
    b1 = listMember(childName, strLst);
    rstLst = if (Config::synchronousFeaturesAllowed()?) {
        list![
            openmodelica_frontend_types::SCode::Restriction::R_TYPE,
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER,
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL,
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING,
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN,
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_CLOCK
        ]
    } else {
        list![
            openmodelica_frontend_types::SCode::Restriction::R_TYPE,
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER,
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL,
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING,
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN
        ]
    };
    b2 = listMember(childRestriction, rstLst);
    b3 = parentRestriction.clone() == openmodelica_frontend_types::SCode::Restriction::R_TYPE;
    b4 = parentRestriction.clone() == SCode::Restriction::R_CONNECTOR { isExpandable: false }
        || parentRestriction == SCode::Restriction::R_CONNECTOR { isExpandable: true };
    b = boolOr(b1, boolOr(b2, boolOr(b3, boolAnd(boolOr(b1, b2), b4))));
    Ok(b)
}

pub(crate) fn matchModificationToComponents(
    mut inElems: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inmod: metamodelica::Ref<DAE::Mod>,
    mut callingScope: &ArcStr,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inElems.clone(), inmod.clone())) {
        (_, Deref @ DAE::Mod::NOMOD { .. }) => {
            ()
        },
        (_, Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Nil, .. }) => {
            ()
        },
        (Deref @ metamodelica::ListNode::Nil, _) => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = Mod::prettyPrintMod(&inmod, 0)?;
            s2 = { let mut __mm_s = String::new(); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(" not found in <")); __mm_s.push_str(&*callingScope); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) };
            Error::addMessage(Error::UNUSED_MODIFIER.clone(), list![s2])?;
            return Err("fail")
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::COMPONENT { name: cn, .. }, tail: elems }, r#mod) => {
            let mut r#mod = (*r#mod).clone();
            r#mod = Mod::removeMod(r#mod.clone(), metamodelica::AsArg::as_arg(&cn))?;
            matchModificationToComponents(metamodelica::AsArg::as_arg(&elems), r#mod.clone(), callingScope)?;
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { .. }, tail: elems }, _) => {
            matchModificationToComponents(metamodelica::AsArg::as_arg(&elems), inmod, callingScope)?;
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::CLASS { name: cn, prefixes: Deref @ SCode::Prefixes { .. }, .. }, tail: elems }, r#mod) => {
            let mut r#mod = (*r#mod).clone();
            r#mod = Mod::removeMod(r#mod.clone(), metamodelica::AsArg::as_arg(&cn))?;
            matchModificationToComponents(metamodelica::AsArg::as_arg(&elems), r#mod.clone(), callingScope)?;
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::IMPORT { .. }, tail: elems }, _) => {
            matchModificationToComponents(metamodelica::AsArg::as_arg(&elems), inmod, callingScope)?;
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. }, .. }, .. }, tail: elems }, _) => {
            matchModificationToComponents(metamodelica::AsArg::as_arg(&elems), inmod, callingScope)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn elementNameMember(
    mut inElement: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
    mut els: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> bool {
    let mut isNamed: bool;
    isNamed = listMember(Util::tuple21(inElement), els);
    isNamed
}

pub(crate) fn extractConstantPlusDepsTpl(
    mut inComps: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut ocr: Option<metamodelica::Ref<DAE::ComponentRef>>,
    mut allComps: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut className: &ArcStr,
    mut ieql: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut iieql: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut ialgs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    mut iialgs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
) -> Result<(
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
)> {
    let mut oel: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)> =
        metamodelica::nil();
    let mut oeql: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut oieql: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut oalgs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
    let mut oialgs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
    (oel, oeql, oieql, oalgs, oialgs) = 'mc: {
        let __mc_input = (&*inComps, &ocr);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok((metamodelica::nil(), ieql.clone(), iieql.clone(), ialgs.clone(), iialgs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None) => {
                    Ok((inComps.clone(), ieql.clone(), iieql.clone(), ialgs.clone(), iialgs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(_)) => {
                    let mut lst: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut oel: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)> = oel.clone();
                    lst = List::map(inComps.clone(), &fnptr!(Util::tuple21, _))?;
                    lst = extractConstantPlusDeps2(&lst, ocr.clone(), allComps.clone(), className, metamodelica::nil())?;
                    let false = ((lst).is_empty()) else { return Err("pattern mismatch") };
                    lst = lst.clone().reverse();
                    oel = List::filter1OnTrue(inComps.clone(), (std::sync::Arc::new(fnptr!(elementNameMember, (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>), metamodelica::List<metamodelica::Ref<SCode::Element>>)) as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>), metamodelica::List<metamodelica::Ref<SCode::Element>>) -> Result<bool> + 'static>), lst.clone())?;
                    Ok(((oel.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), oel.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oel = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(cr)) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.extractConstantPlusDeps failure to find ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?); __mm_s.push_str(&*literal!(", returning \n")); ArcStr::from(__mm_s) })?;
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.extractConstantPlusDeps elements to instantiate:")); __mm_s.push_str(&*intString(((inComps).len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((oel, oeql, oieql, oalgs, oialgs))
}

pub(crate) fn extractConstantPlusDeps(
    mut inComps: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut ocr: Option<metamodelica::Ref<DAE::ComponentRef>>,
    mut allComps: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut className: &ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outComps: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    outComps = 'mc: {
        let __mc_input = &ocr;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                None => {
                    Ok(inComps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(_) => {
                    let mut outComps: metamodelica::List<metamodelica::Ref<SCode::Element>> = outComps.clone();
                    outComps = extractConstantPlusDeps2(&inComps, ocr.clone(), allComps.clone(), className, metamodelica::nil())?;
                    let false = ((outComps).is_empty()) else { return Err("pattern mismatch") };
                    outComps = outComps.clone().reverse();
                    Ok((outComps.clone(), outComps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outComps = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(cr) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.extractConstantPlusDeps failure to find ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?); __mm_s.push_str(&*literal!(", returning \n")); ArcStr::from(__mm_s) })?;
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.extractConstantPlusDeps elements to instantiate:")); __mm_s.push_str(&*intString(((inComps).len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outComps)
}

fn extractConstantPlusDeps2(
    mut inComps: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut ocr: Option<metamodelica::Ref<DAE::ComponentRef>>,
    mut inAllComps: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut className: &ArcStr,
    mut inExisting: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outComps: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    outComps = 'mc: {
        let __mc_input = (&**inComps, &ocr, inAllComps, inExisting);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Some(_), _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _, _) => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, _, _) => {
                    Ok(inComps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: selem @ Deref @ SCode::Element::CLASS { name: name2, .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { .. }), allComps, existing) => {
                    let mut allComps = (*allComps).clone();
                    let mut existing = (*existing).clone();
                    let mut outComps: metamodelica::List<metamodelica::Ref<SCode::Element>> = outComps.clone();
                    allComps = metamodelica::cons(selem.clone(), allComps.clone());
                    existing = metamodelica::cons(name2.clone(), existing.clone());
                    outComps = extractConstantPlusDeps2(metamodelica::AsArg::as_arg(&comps), ocr.clone(), allComps.clone(), className, existing.clone())?;
                    Ok((metamodelica::cons(selem.clone(), outComps.clone()), outComps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outComps = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: selem @ Deref @ SCode::Element::COMPONENT { name: name2, modifications: scmod, .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }), allComps, existing) => {
                    let mut recDeps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut allComps = (*allComps).clone();
                    let mut existing = (*existing).clone();
                    let true = (stringEq(&name, &name2)) else { return Err("pattern mismatch") };
                    crefs = getCrefFromMod(scmod.clone())?;
                    allComps = listAppend(comps.clone(), allComps.clone());
                    existing = metamodelica::cons(name2.clone(), existing.clone());
                    recDeps = extractConstantPlusDeps3(&crefs, metamodelica::AsArg::as_arg(&allComps), className, existing.clone())?;
                    Ok(metamodelica::cons(selem.clone(), recDeps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: selem @ Deref @ SCode::Element::COMPONENT { name: name2, .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }), allComps, existing) => {
                    let mut allComps = (*allComps).clone();
                    let false = (stringEq(&name, &name2)) else { return Err("pattern mismatch") };
                    allComps = metamodelica::cons(selem.clone(), allComps.clone());
                    Ok(extractConstantPlusDeps2(metamodelica::AsArg::as_arg(&comps), ocr.clone(), allComps.clone(), className, existing.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: compMod @ Deref @ SCode::Element::EXTENDS { .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { .. }), allComps, existing) => {
                    let mut recDeps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut allComps = (*allComps).clone();
                    allComps = metamodelica::cons(compMod.clone(), allComps.clone());
                    recDeps = extractConstantPlusDeps2(metamodelica::AsArg::as_arg(&comps), ocr.clone(), allComps.clone(), className, existing.clone())?;
                    Ok(metamodelica::cons(compMod.clone(), recDeps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: compMod @ Deref @ SCode::Element::IMPORT { .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { .. }), allComps, existing) => {
                    let mut recDeps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut allComps = (*allComps).clone();
                    allComps = metamodelica::cons(compMod.clone(), allComps.clone());
                    recDeps = extractConstantPlusDeps2(metamodelica::AsArg::as_arg(&comps), ocr.clone(), allComps.clone(), className, existing.clone())?;
                    Ok(metamodelica::cons(compMod.clone(), recDeps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: compMod @ Deref @ SCode::Element::DEFINEUNIT { .. }, tail: comps }, Some(Deref @ DAE::ComponentRef::CREF_IDENT { .. }), allComps, existing) => {
                    let mut recDeps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut allComps = (*allComps).clone();
                    allComps = metamodelica::cons(compMod.clone(), allComps.clone());
                    recDeps = extractConstantPlusDeps2(metamodelica::AsArg::as_arg(&comps), ocr.clone(), allComps.clone(), className, existing.clone())?;
                    Ok(metamodelica::cons(compMod.clone(), recDeps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!(" failure in get_Constant_PlusDeps \n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outComps)
}

fn extractConstantPlusDeps3(
    mut inAcrefs: &metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    mut remainingComps: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut className: &ArcStr,
    mut inExisting: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outComps: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    outComps = 'mc: {
        let __mc_input = (&**inAcrefs, inExisting);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: acr }, tail: acrefs }, existing) => {
                    Ok(extractConstantPlusDeps3(&(metamodelica::cons(acr.clone(), acrefs.clone())), remainingComps, className, existing.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentRef::CREF_QUAL { name: s1, subscripts: _, componentRef: acr @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: _, subscripts: _ } }, tail: acrefs }, existing) => {
                    if !((stringEq(&className, &s1))) { return Err("guard") }
                    Ok(extractConstantPlusDeps3(&(metamodelica::cons(acr.clone(), acrefs.clone())), remainingComps, className, existing.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentRef::CREF_QUAL { name: _, subscripts: _, componentRef: _ }, tail: acrefs }, existing) => {
                    let mut outComps: metamodelica::List<metamodelica::Ref<SCode::Element>> = outComps.clone();
                    outComps = extractConstantPlusDeps3(metamodelica::AsArg::as_arg(&acrefs), remainingComps, className, existing.clone())?;
                    Ok((outComps.clone(), outComps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outComps = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentRef::CREF_IDENT { name: s1, subscripts: _ }, tail: acrefs }, existing) => {
                    if !((List::isMemberOnTrue(s1.clone(), metamodelica::AsArg::as_arg(&existing), &fnptr!(stringEq, ArcStr, ArcStr))?)) { return Err("guard") }
                    Ok(extractConstantPlusDeps3(metamodelica::AsArg::as_arg(&acrefs), remainingComps, className, existing.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentRef::CREF_IDENT { name: s1, subscripts: _ }, tail: acrefs }, existing) => {
                    let mut localComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut names: metamodelica::List<ArcStr>;
                    let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
                    let mut existing = (*existing).clone();
                    let mut outComps: metamodelica::List<metamodelica::Ref<SCode::Element>> = outComps.clone();
                    cref_ = ComponentReferenceBasics::makeCrefIdent(s1.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    localComps = extractConstantPlusDeps2(remainingComps, Some(cref_.clone()), metamodelica::nil(), className, existing.clone())?;
                    names = SCodeUtil::componentNamesFromElts(&localComps);
                    existing = listAppend(names.clone(), existing.clone());
                    outComps = extractConstantPlusDeps3(metamodelica::AsArg::as_arg(&acrefs), remainingComps, className, existing.clone())?;
                    outComps = listAppend(localComps.clone(), outComps.clone());
                    Ok((outComps.clone(), outComps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outComps = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outComps)
}

pub fn removeSelfReference(
    mut className: ArcStr,
    mut path: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = if (stringEq(&className, &(AbsynUtil::pathFirstIdent(&path)))) {
        AbsynUtil::removePrefix(metamodelica::Ref::new(Absyn::Path::IDENT { name: className }), path)?
    } else {
        path
    };
    Ok(outPath)
}

pub(crate) fn printExtcomps(
    mut inElements: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inElements {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (el, r#mod), tail: els } => {
            let mut s: ArcStr;
            s = SCodeDump::unparseElementStr(el.clone(), SCodeDump::defaultOptions.clone())?;
            metamodelica::print(s);
            metamodelica::print(literal!(", "));
            metamodelica::print(Mod::printModStr(metamodelica::AsArg::as_arg(&r#mod))?);
            metamodelica::print(literal!("\n"));
            printExtcomps(els)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn constantEls(
    mut elements: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> metamodelica::List<metamodelica::Ref<SCode::Element>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut attr: SCode::Attributes;
    outElements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
        for mut el in (elements).into_iter().cloned() {
            if !(match &*el.clone() {
                SCode::Element::COMPONENT {
                    attributes: __esc_attr, ..
                } => {
                    attr = (*__esc_attr).clone();
                    SCodeUtil::isConstant(SCodeUtil::attrVariability(metamodelica::AsArg::as_arg(&attr)))
                }
                _ => false,
            }) {
                continue;
            }
            let __x = el.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outElements
}

pub(crate) fn constantAndParameterEls(
    mut elements: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> metamodelica::List<metamodelica::Ref<SCode::Element>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut attr: SCode::Attributes;
    outElements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
        for mut el in (elements).into_iter().cloned() {
            if !(match &*el.clone() {
                SCode::Element::COMPONENT {
                    attributes: __esc_attr, ..
                } => {
                    attr = (*__esc_attr).clone();
                    SCodeUtil::isParameterOrConst(SCodeUtil::attrVariability(metamodelica::AsArg::as_arg(&attr)))
                }
                _ => false,
            }) {
                continue;
            }
            let __x = el.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outElements
}

fn removeBindings(
    mut elements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> metamodelica::List<metamodelica::Ref<SCode::Element>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    outElements = (::match_deref::match_deref! { match elements {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::COMPONENT { name, prefixes, attributes, typeSpec, modifications: _, comment, condition, info }, tail: els } => {
            let mut els1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            els1 = removeBindings(els);
            metamodelica::cons(metamodelica::Ref::new(SCode::Element::COMPONENT { name: name.clone(), prefixes: prefixes.clone(), attributes: attributes.clone(), typeSpec: typeSpec.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: comment.clone(), condition: condition.clone(), info: info.clone() }), els1)
        },
        Deref @ metamodelica::ListNode::Cons { head: el, tail: els } => {
            let mut els1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            els1 = removeBindings(els);
            metamodelica::cons(el.clone(), els1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElements
}

fn removeExtBindings(
    mut elements: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) -> metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)> {
    let mut outElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    outElements = (::match_deref::match_deref! { match elements {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: (Deref @ SCode::Element::COMPONENT { name, prefixes, attributes, typeSpec, modifications: _, comment, condition, info }, _), tail: els } => {
            let mut els1: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
            els1 = removeExtBindings(els);
            metamodelica::cons((metamodelica::Ref::new(SCode::Element::COMPONENT { name: name.clone(), prefixes: prefixes.clone(), attributes: attributes.clone(), typeSpec: typeSpec.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: comment.clone(), condition: condition.clone(), info: info.clone() }), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()), els1)
        },
        Deref @ metamodelica::ListNode::Cons { head: el, tail: els } => {
            let mut els1: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
            els1 = removeExtBindings(els);
            metamodelica::cons(el.clone(), els1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElements
}

pub(crate) fn getModsForDep(
    mut inDepCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut inElems: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut omods: metamodelica::Ref<DAE::Mod>;
    omods = 'mc: {
        let __mc_input = (inDepCref, &**inElems);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(openmodelica_frontend_types::DAE::Mod::interned_NOMOD())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dep, Deref @ metamodelica::ListNode::Cons { head: (Deref @ SCode::Element::COMPONENT { .. }, Deref @ DAE::Mod::NOMOD { .. }), tail: elems }) => {
                    Ok(getModsForDep(dep.clone(), metamodelica::AsArg::as_arg(&elems))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dep, Deref @ metamodelica::ListNode::Cons { head: (Deref @ SCode::Element::COMPONENT { name: name1, .. }, cmod), tail: _ }) => {
                    let mut name2: ArcStr;
                    let mut cmod = (*cmod).clone();
                    name2 = Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&dep))?;
                    let true = (stringEq(&name2, &name1)) else { return Err("pattern mismatch") };
                    cmod = metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: list![metamodelica::Ref::new(DAE::SubMod { ident: name2.clone(), r#mod: cmod.clone() })], binding: None, info: Absyn::dummyInfo.clone() });
                    Ok(cmod.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dep, Deref @ metamodelica::ListNode::Cons { head: _, tail: elems }) => {
                    let mut cmod: metamodelica::Ref<DAE::Mod>;
                    cmod = getModsForDep(dep.clone(), metamodelica::AsArg::as_arg(&elems))?;
                    Ok(cmod.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(omods)
}

fn getOptionArraydim(
    mut inAbsynArrayDimOption: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Subscript>> {
    let mut outArrayDim: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    outArrayDim = (::match_deref::match_deref! { match &(inAbsynArrayDimOption) {
        Some(dim) => {
            dim.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outArrayDim
}

pub(crate) fn addNomod(
    mut inElements: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)> {
    let mut outElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    outElements = ({
        let mut __acc: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)> =
            metamodelica::nil();
        for mut x in (inElements).into_iter().cloned() {
            let __x = (x.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outElements
}

pub(crate) fn sortElementList(
    mut inElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut inEnv: &FCore::Graph,
    mut isFunctionScope: bool,
) -> Result<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>> {
    pub(crate) type Element = (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>);

    let mut inElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)> =
        inElements;
    let mut outE: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    let mut cycles: metamodelica::List<(
        (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    )>;
    let mut g: metamodelica::List<(
        (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    )>;
    g = Graph::buildGraph(
        inElements.clone(),
        &move |__a0: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
               __a1: (
            metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
            bool,
        )|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(getElementDependencies(&__a0, &__a1)) },
        (inElements.clone(), isFunctionScope),
    )?;
    (outE, cycles) = Graph::topologicalSort(
        &g,
        (std::sync::Arc::new(isElementEqual)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
                        (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    if !(Config::acceptMetaModelicaGrammar()?) {
        inElements = listAppend(outE, List::map(cycles.clone(), &fnptr!(Util::tuple21, _))?);
    }
    checkCyclicalComponents(&cycles, inEnv)?;
    Ok(inElements)
}

fn printGraph(
    mut env: &FCore::Graph,
    mut g: metamodelica::List<(
        (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    )>,
    mut order: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut cycles: metamodelica::List<(
        (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    )>,
) -> Result<()> {
    pub(crate) type Element = (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>);

    let () = (::match_deref::match_deref! { match &(g.clone()) {
        Deref @ metamodelica::ListNode::Nil => (),
        _ => {
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Graph for env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*Graph::printGraph(g, (std::sync::Arc::new(elementName) as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)) -> Result<ArcStr> + 'static>))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Element order:\n\t")); __mm_s.push_str(&*stringDelimitList(List::map(order, &elementName)?, literal!("\n\t"))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cycles:\n")); __mm_s.push_str(&*Graph::printGraph(cycles, (std::sync::Arc::new(elementName) as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)) -> Result<ArcStr> + 'static>))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn getDepsFromExps<'__b>(
    mut inExps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inAllElements: &'__b metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut inDependencies: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut isFunction: bool,
) -> Result<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExps, inDependencies.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(inDependencies)
            },
            (Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }, deps) => {
                let mut deps = (*deps).clone();
                let (_, (_, _, __pa0, _)) = AbsynUtil::traverseExpBidir(e.clone(), (std::sync::Arc::new(fnptr!(getElementDependenciesTraverserEnter, metamodelica::Ref<Absyn::Exp>, (metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>, metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, (metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>, metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, bool)) -> Result<(metamodelica::Ref<Absyn::Exp>, (metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>, metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, bool))> + 'static>), (std::sync::Arc::new(fnptr!(getElementDependenciesTraverserExit, metamodelica::Ref<Absyn::Exp>, (metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>, metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, (metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>, metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, bool)) -> Result<(metamodelica::Ref<Absyn::Exp>, (metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>, metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, bool))> + 'static>), (inAllElements.clone(), metamodelica::nil(), deps.clone(), isFunction))?;
                deps = metamodelica::Own::own(__pa0);
                { (inExps, inAllElements, inDependencies, isFunction) = (rest.clone(), inAllElements, deps.clone(), isFunction); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn removeCurrentElementFromArrayDimDeps(
    mut name: &ArcStr,
    mut inDependencies: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) -> Result<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>> {
    let mut outDependencies: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    outDependencies = ({
        let mut __acc: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)> =
            metamodelica::nil();
        for mut dep in (inDependencies).into_iter().cloned() {
            if !(!(stringEq(&name, &(SCodeUtil::elementName(&(Util::tuple21(dep.clone())))?)))) {
                continue;
            }
            let __x = dep.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outDependencies)
}

pub(crate) fn getExpsFromConstrainClass(
    mut inRP: &metamodelica::Ref<SCode::Replaceable>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
)> {
    let mut outBindingExp: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut outSubsExps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    (outBindingExp, outSubsExps) = (::match_deref::match_deref! { match inRP {
        Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ SCode::Replaceable::REPLACEABLE { cc: None } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: m, .. }) } => {
            let mut l1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut l2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            (l1, l2) = getExpsFromMod(metamodelica::AsArg::as_arg(&m))?;
            (l1, l2)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outBindingExp, outSubsExps))
}

fn getExpsFromSubMods(
    mut inSubMods: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Exp>>> {
    let mut outSubsExps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    outSubsExps = (::match_deref::match_deref! { match inSubMods {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { r#mod, .. }, tail: rest } => {
            let mut e: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut sm: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            (e, sm) = getExpsFromMod(metamodelica::AsArg::as_arg(&r#mod))?;
            exps = getExpsFromSubMods(rest)?;
            exps = listAppend(e, listAppend(sm, exps));
            exps
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSubsExps)
}

pub(crate) fn getCrefFromMod(
    mut inMod: metamodelica::Ref<SCode::Mod>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>> {
    let mut outCrefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>> = metamodelica::nil();
    outCrefs = 'mc: {
        let __mc_input = &*inMod;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut l1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut l2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut outCrefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>> = outCrefs.clone();
                    (l1, l2) = getExpsFromMod(&inMod)?;
                    outCrefs = List::flatten(List::map2(listAppend(l1.clone(), l2.clone()), &AbsynUtil::getCrefFromExp, true, true)?)?;
                    Ok((outCrefs.clone(), outCrefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCrefs = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstUtil.getCrefFromMod")); __mm_s.push_str(&*literal!(": could not retrieve crefs from SCode.Mod: ")); __mm_s.push_str(&*SCodeDump::printModStr(inMod.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCrefs)
}

pub(crate) fn getExpsFromMod(
    mut inMod: &metamodelica::Ref<SCode::Mod>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
)> {
    let mut outBindingExp: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut outSubsExps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    (outBindingExp, outSubsExps) = (::match_deref::match_deref! { match inMod {
        Deref @ SCode::Mod::NOMOD { .. } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ SCode::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Nil, binding: None, .. } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ SCode::Mod::MOD { subModLst: subs, binding: Some(e), .. } => {
            let mut se: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            se = getExpsFromSubMods(subs)?;
            (list![e.clone()], se)
        },
        Deref @ SCode::Mod::MOD { subModLst: subs, binding: None, .. } => {
            let mut se: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            se = getExpsFromSubMods(subs)?;
            (metamodelica::nil(), se)
        },
        Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: _, arrayDim: ado }, modifications: m, attributes: _ }, .. }, .. } => {
            let mut se: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut l1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut l2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut l3: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut l4: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            (l1, l2) = getExpsFromConstrainClass(metamodelica::AsArg::as_arg(&rp))?;
            (_, se) = AbsynUtil::getExpsFromArrayDimOpt(ado.clone())?;
            (l3, l4) = getExpsFromMod(metamodelica::AsArg::as_arg(&m))?;
            l1 = listAppend(se, listAppend(l1, l3));
            l4 = listAppend(l2, l4);
            (l1, l4)
        },
        Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: m, .. }, .. }, .. } => {
            let mut l1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut l2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut l3: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut l4: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            (l1, l2) = getExpsFromConstrainClass(metamodelica::AsArg::as_arg(&rp))?;
            (l3, l4) = getExpsFromMod(metamodelica::AsArg::as_arg(&m))?;
            l3 = listAppend(l1, l3);
            l4 = listAppend(l2, l4);
            (l3, l4)
        },
        Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, .. }, .. } => {
            let mut l1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut l2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            (l1, l2) = getExpsFromConstrainClass(metamodelica::AsArg::as_arg(&rp))?;
            (l1, l2)
        },
        Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, modifications: m, attributes: SCode::Attributes { arrayDims: ad, .. }, .. }, .. } => {
            let mut se: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut l1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut l2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut l3: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut l4: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            (l1, l2) = getExpsFromConstrainClass(metamodelica::AsArg::as_arg(&rp))?;
            (_, se) = AbsynUtil::getExpsFromArrayDim(ad.clone())?;
            (l3, l4) = getExpsFromMod(metamodelica::AsArg::as_arg(&m))?;
            l1 = listAppend(se, listAppend(l1, l3));
            l4 = listAppend(l2, l4);
            (l1, l4)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outBindingExp, outSubsExps))
}

pub(crate) fn getCrefFromDim(
    mut inArrayDim: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>> {
    let mut outAbsynComponentRefLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    outAbsynComponentRefLst = 'mc: {
        let __mc_input = &**inArrayDim;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: exp }, tail: rest } => {
                    let mut l1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut l2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    l1 = getCrefFromDim(metamodelica::AsArg::as_arg(&rest))?;
                    l2 = AbsynUtil::getCrefFromExp(exp.clone(), true, true)?;
                    res = List::union(&l1, &l2);
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::NOSUB { .. }, tail: rest } => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    res = getCrefFromDim(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
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
                    Debug::trace(literal!("- InstUtil.getCrefFromDim failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAbsynComponentRefLst)
}

pub(crate) fn getElementDependencies(
    mut inElement: &(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
    mut inAllElementsAndIsFunctionScope: &(
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
        bool,
    ),
) -> metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)> {
    let mut outDependencies: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    outDependencies = 'mc: {
        let __mc_input = (inElement, inAllElementsAndIsFunctionScope);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::COMPONENT { name, condition: cExpOpt, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, attributes: SCode::Attributes { arrayDims: ad, variability: var, .. }, modifications: r#mod, .. }, daeMod), (inAllElements, isFunction)) => {
                    let mut deps: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut sexps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut bexps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let true = (SCodeUtil::isParameterOrConst(var.clone())) else { return Err("pattern mismatch") };
                    (_, exps) = AbsynUtil::getExpsFromArrayDim(ad.clone())?;
                    (bexps, sexps) = getExpsFromMod(metamodelica::AsArg::as_arg(&r#mod))?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (bexps, sexps) = getExpsFromConstrainClass(metamodelica::AsArg::as_arg(&rp))?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (bexps, sexps) = getExpsFromMod(&(Mod::unelabMod(daeMod.clone())?))?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    deps = getDepsFromExps(exps.clone(), metamodelica::AsArg::as_arg(&inAllElements), metamodelica::nil(), isFunction.clone())?;
                    deps = removeCurrentElementFromArrayDimDeps(metamodelica::AsArg::as_arg(&name), deps.clone())?;
                    deps = getDepsFromExps(List::fromOption(cExpOpt.clone()), metamodelica::AsArg::as_arg(&inAllElements), deps.clone(), isFunction.clone())?;
                    deps = List::union(&deps, &(metamodelica::nil()));
                    Ok(deps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction, .. }, .. }, _), (_, true)) => {
                    let true = (AbsynUtil::isInputOrOutput(direction.clone())) else { return Err("pattern mismatch") };
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::COMPONENT { name, condition: cExpOpt, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, attributes: SCode::Attributes { arrayDims: ad, .. }, modifications: r#mod, .. }, daeMod), (inAllElements, isFunction)) => {
                    let mut deps: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut sexps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut bexps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    (_, exps) = AbsynUtil::getExpsFromArrayDim(ad.clone())?;
                    (bexps, sexps) = getExpsFromMod(metamodelica::AsArg::as_arg(&r#mod))?;
                    exps = listAppend(sexps.clone(), exps.clone());
                    exps = listAppend(bexps.clone(), exps.clone());
                    (bexps, sexps) = getExpsFromConstrainClass(metamodelica::AsArg::as_arg(&rp))?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (bexps, sexps) = getExpsFromMod(&(Mod::unelabMod(daeMod.clone())?))?;
                    exps = listAppend(sexps.clone(), exps.clone());
                    exps = listAppend(bexps.clone(), exps.clone());
                    deps = getDepsFromExps(exps.clone(), metamodelica::AsArg::as_arg(&inAllElements), metamodelica::nil(), isFunction.clone())?;
                    deps = removeCurrentElementFromArrayDimDeps(metamodelica::AsArg::as_arg(&name), deps.clone())?;
                    deps = getDepsFromExps(List::fromOption(cExpOpt.clone()), metamodelica::AsArg::as_arg(&inAllElements), deps.clone(), isFunction.clone())?;
                    deps = List::union(&deps, &(metamodelica::nil()));
                    Ok(deps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::CLASS { name, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, classDef: Deref @ SCode::ClassDef::DERIVED { modifications: r#mod, attributes: SCode::Attributes { arrayDims: ad, .. }, .. }, .. }, daeMod), (inAllElements, isFunction)) => {
                    let mut deps: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut sexps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut bexps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    (_, exps) = AbsynUtil::getExpsFromArrayDim(ad.clone())?;
                    (_, sexps) = getExpsFromMod(metamodelica::AsArg::as_arg(&r#mod))?;
                    exps = listAppend(sexps.clone(), exps.clone());
                    (bexps, sexps) = getExpsFromConstrainClass(metamodelica::AsArg::as_arg(&rp))?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (_, sexps) = getExpsFromMod(&(Mod::unelabMod(daeMod.clone())?))?;
                    exps = listAppend(sexps.clone(), exps.clone());
                    deps = getDepsFromExps(exps.clone(), metamodelica::AsArg::as_arg(&inAllElements), metamodelica::nil(), isFunction.clone())?;
                    deps = removeCurrentElementFromArrayDimDeps(metamodelica::AsArg::as_arg(&name), deps.clone())?;
                    deps = List::union(&deps, &(metamodelica::nil()));
                    Ok(deps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ SCode::Element::CLASS { name, prefixes: Deref @ SCode::Prefixes { .. }, classDef: Deref @ SCode::ClassDef::PARTS { externalDecl, .. }, .. }, _), (inAllElements, isFunction)) => {
                    let mut deps: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    exps = getExpsFromExternalDecl(externalDecl.clone())?;
                    deps = getDepsFromExps(exps.clone(), metamodelica::AsArg::as_arg(&inAllElements), metamodelica::nil(), isFunction.clone())?;
                    deps = removeCurrentElementFromArrayDimDeps(metamodelica::AsArg::as_arg(&name), deps.clone())?;
                    deps = List::union(&deps, &(metamodelica::nil()));
                    Ok(deps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outDependencies
}

fn getExpsFromExternalDecl(
    mut inExternalDecl: Option<metamodelica::Ref<SCode::ExternalDecl>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Exp>>> {
    let mut outExps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    outExps = (::match_deref::match_deref! { match &(inExternalDecl) {
        None => {
            metamodelica::nil()
        },
        Some(Deref @ SCode::ExternalDecl { args: exps, .. }) => {
            exps.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExps)
}

fn getExpsFromDefaults(
    mut inEls: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inAcc: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Exp>> {
    let mut outExps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    outExps = 'mc: {
        let __mc_input = &**inEls;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rp, .. }, modifications: m, .. }, tail: rest } => {
                    let mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut sexps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut bexps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    exps = inAcc.clone();
                    (bexps, sexps) = getExpsFromConstrainClass(metamodelica::AsArg::as_arg(&rp))?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    (bexps, sexps) = getExpsFromMod(metamodelica::AsArg::as_arg(&m))?;
                    exps = listAppend(bexps.clone(), listAppend(sexps.clone(), exps.clone()));
                    exps = getExpsFromDefaults(metamodelica::AsArg::as_arg(&rest), &exps);
                    Ok(exps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    exps = getExpsFromDefaults(metamodelica::AsArg::as_arg(&rest), inAcc);
                    Ok(exps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExps
}

fn getElementDependenciesTraverserEnter(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inTuple: (
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
        metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>,
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
        bool,
    ),
) -> (
    metamodelica::Ref<Absyn::Exp>,
    (
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
        metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>,
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
        bool,
    ),
) {
    pub(crate) type ElementList = metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;

    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outTuple: (
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
        metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>,
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
        bool,
    );
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), &inTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ Absyn::Exp::CREF { componentRef: cref }, (all_el, stack, accum_el, b)) => {
                    let mut id: ArcStr;
                    let mut e: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>);
                    id = AbsynUtil::crefFirstIdent(metamodelica::AsArg::as_arg(&cref))?;
                    e = List::find1(metamodelica::AsArg::as_arg(&all_el), &move |__a0: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>), __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(isElementNamed(&__a0, &__a1)) }, id.clone())?;
                    Ok((exp.clone(), (all_el.clone(), stack.clone(), metamodelica::cons(e.clone(), accum_el.clone()), b.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ Absyn::Exp::CALL { function_: cref, .. }, (all_el, stack, accum_el, b)) => {
                    let mut id: ArcStr;
                    let mut e: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>);
                    id = AbsynUtil::crefFirstIdent(metamodelica::AsArg::as_arg(&cref))?;
                    e = List::find1(metamodelica::AsArg::as_arg(&all_el), &move |__a0: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>), __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(isElementNamed(&__a0, &__a1)) }, id.clone())?;
                    Ok((exp.clone(), (all_el.clone(), stack.clone(), metamodelica::cons(e.clone(), accum_el.clone()), b.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ Absyn::Exp::IFEXP { .. }, (all_el, stack, accum_el, false)) => {
                    Ok((exp.clone(), (all_el.clone(), metamodelica::cons(accum_el.clone(), stack.clone()), metamodelica::nil(), false)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTuple)
}

fn getElementDependenciesTraverserExit(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inTuple: (
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
        metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>,
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
        bool,
    ),
) -> (
    metamodelica::Ref<Absyn::Exp>,
    (
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
        metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>,
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
        bool,
    ),
) {
    pub(crate) type ElementList = metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;

    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outTuple: (
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
        metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>,
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
        bool,
    );
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), &inTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ Absyn::Exp::IFEXP { ifExp, .. }, (all_el, Deref @ metamodelica::ListNode::Cons { head: stack_el, tail: rest_stack }, _, false)) => {
                    let mut deps: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let (_, (_, _, __pa0, _)) = AbsynUtil::traverseExpBidir(ifExp.clone(), (std::sync::Arc::new(fnptr!(getElementDependenciesTraverserEnter, metamodelica::Ref<Absyn::Exp>, (metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>, metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, (metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>, metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, bool)) -> Result<(metamodelica::Ref<Absyn::Exp>, (metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>, metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, bool))> + 'static>), (std::sync::Arc::new(fnptr!(getElementDependenciesTraverserExit, metamodelica::Ref<Absyn::Exp>, (metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>, metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, (metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>, metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, bool)) -> Result<(metamodelica::Ref<Absyn::Exp>, (metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>, metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>, bool))> + 'static>), (all_el.clone(), metamodelica::nil(), metamodelica::nil(), false))?;
                    deps = metamodelica::Own::own(__pa0);
                    Ok((exp.clone(), (all_el.clone(), rest_stack.clone(), listAppend(deps.clone(), stack_el.clone()), false)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTuple)
}

fn isElementNamed(
    mut inElement: &(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
    mut inName: &ArcStr,
) -> bool {
    let mut isNamed: bool;
    isNamed = (::match_deref::match_deref! { match &(inElement) {
        (Deref @ SCode::Element::COMPONENT { name, .. }, _) => {
            metamodelica::stringEq(&name, &inName)
        },
        (Deref @ SCode::Element::CLASS { name, .. }, _) => {
            metamodelica::stringEq(&name, &inName)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNamed
}

fn isElementEqual(
    mut inElement1: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
    mut inElement2: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = (::match_deref::match_deref! { match &((inElement1.clone(), inElement2.clone())) {
        ((Deref @ SCode::Element::COMPONENT { name: id1, .. }, _), (Deref @ SCode::Element::COMPONENT { name: id2, .. }, _)) => {
            stringEqual(&id1, &id2)
        },
        ((Deref @ SCode::Element::CLASS { name: id1, .. }, _), (Deref @ SCode::Element::CLASS { name: id2, .. }, _)) => {
            stringEqual(&id1, &id2)
        },
        _ => {
            stringEq(&(elementName(inElement1)?), &(elementName(inElement2)?))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

fn checkCyclicalComponents(
    mut inCycles: &metamodelica::List<(
        (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
        metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    )>,
    mut inEnv: &FCore::Graph,
) -> Result<()> {
    pub(crate) type Element = (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>);

    let () = 'mc: {
        let __mc_input = &**inCycles;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
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
                    let mut graph: metamodelica::List<((metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>), metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>)>;
                    graph = Graph::filterGraph(inCycles, (std::sync::Arc::new(move |__a0: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(isElementParamOrConst(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)) -> Result<bool> + 'static>))?;
                    ::match_deref::match_deref! { match &(Graph::findCycles(&graph, &isElementEqual)?) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
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
                    let mut cycles: metamodelica::List<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>>;
                    let mut names: metamodelica::List<metamodelica::List<ArcStr>>;
                    let mut cycles_strs: metamodelica::List<ArcStr>;
                    let mut cycles_str: ArcStr;
                    let mut scope_str: ArcStr;
                    cycles = Graph::findCycles(inCycles, &isElementEqual)?;
                    names = List::mapList(cycles.clone(), &elementName)?;
                    cycles_strs = List::map1(names.clone(), &fnptr!(stringDelimitList, metamodelica::List<ArcStr>, ArcStr), literal!(","))?;
                    cycles_str = stringDelimitList(cycles_strs.clone(), literal!("}, {"));
                    cycles_str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*cycles_str); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) };
                    scope_str = FGraph::printGraphPathStr(inEnv);
                    Error::addMessage(Error::CIRCULAR_COMPONENTS.clone(), list![scope_str.clone(), cycles_str.clone()])?;
                    if !(Flags::isSet(Flags::IGNORE_CYCLES.clone())?) {
                        return Err("fail");
                    }
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn isElementParamOrConst(mut inElement: &(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)) -> bool {
    let mut outIsParamOrConst: bool;
    outIsParamOrConst = (::match_deref::match_deref! { match &(inElement) {
        (Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { variability: var, .. }, .. }, _) => {
            SCodeUtil::isParameterOrConst(var.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsParamOrConst
}

fn elementName(mut inElement: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)) -> Result<ArcStr> {
    let mut outName: ArcStr = arcstr::literal!("");
    let mut elem: metamodelica::Ref<SCode::Element> =
        <metamodelica::Ref<SCode::Element> as ::std::default::Default>::default();
    outName = 'mc: {
        let __mc_input = &inElement;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut elem: metamodelica::Ref<SCode::Element> = elem.clone();
                    let mut outName: ArcStr = outName.clone();
                    (elem, _) = inElement.clone();
                    outName = SCodeUtil::elementName(&elem)?;
                    Ok((outName.clone(), elem.clone(), outName.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            elem = __wb0;
            outName = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    r#str = SCodeDump::shortElementStr(Util::tuple21(inElement.clone()))?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outName)
}

pub(crate) fn classdefElts2(
    mut inElements: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut partialPrefix: SCode::Partial,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
)> {
    let mut outClassDefs: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut outConstEls: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    (outClassDefs, outConstEls) = 'mc: {
        let __mc_input = (&**inElements, partialPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (cdef @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PACKAGE { .. }, .. }, _), tail: xs }, SCode::Partial::PARTIAL { .. }) => {
                    let mut cdefs: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut els: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    (cdefs, els) = classdefElts2(metamodelica::AsArg::as_arg(&xs), partialPrefix.clone())?;
                    Ok((metamodelica::cons(cdef.clone(), cdefs.clone()), els.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (cdef @ Deref @ SCode::Element::CLASS { .. }, _), tail: xs }, SCode::Partial::NOT_PARTIAL { .. }) => {
                    let mut cdefs: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut els: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    (cdefs, els) = classdefElts2(metamodelica::AsArg::as_arg(&xs), partialPrefix.clone())?;
                    Ok((metamodelica::cons(cdef.clone(), cdefs.clone()), els.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: el @ (Deref @ SCode::Element::COMPONENT { attributes: attr, .. }, _), tail: xs }, SCode::Partial::NOT_PARTIAL { .. }) => {
                    let mut cdefs: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut els: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let SCode::CONST { .. } = (SCodeUtil::attrVariability(metamodelica::AsArg::as_arg(&attr))) else { return Err("pattern mismatch") };
                    (cdefs, els) = classdefElts2(metamodelica::AsArg::as_arg(&xs), partialPrefix.clone())?;
                    Ok((cdefs.clone(), metamodelica::cons(el.clone(), els.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, _) => {
                    let mut cdefs: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut els: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    (cdefs, els) = classdefElts2(metamodelica::AsArg::as_arg(&xs), partialPrefix)?;
                    Ok((cdefs.clone(), els.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outClassDefs, outConstEls))
}

pub(crate) fn classdefAndImpElts(
    mut elts: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> (
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
) {
    let mut cdefElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut restElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    (cdefElts, restElts) = (::match_deref::match_deref! { match elts {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: cdef @ Deref @ SCode::Element::CLASS { .. }, tail: xs } => {
            (_, restElts) = classdefAndImpElts(xs);
            (metamodelica::cons(cdef.clone(), restElts.clone()), restElts)
        },
        Deref @ metamodelica::ListNode::Cons { head: imp @ Deref @ SCode::Element::IMPORT { .. }, tail: xs } => {
            (cdefElts, restElts) = classdefAndImpElts(xs);
            (metamodelica::cons(imp.clone(), cdefElts), restElts)
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: xs } => {
            (cdefElts, restElts) = classdefAndImpElts(xs);
            (cdefElts, metamodelica::cons(e.clone(), restElts))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (cdefElts, restElts)
}

/*
protected function extendsElts
"author: PA
  This function filters out the extends Element in an Element list"
  input list<SCode.Element> inSCodeElementLst;
  output list<SCode.Element> outSCodeElementLst;
algorithm
  outSCodeElementLst := match (inSCodeElementLst)
    local
      list<SCode.Element> res,xs;
      SCode.Element cdef;
    case ({}) then {};
    case (((cdef as SCode.EXTENDS(baseClassPath = _)) :: xs))
      algorithm
        res = extendsElts(xs);
      then
        (cdef :: res);
    case ((_ :: xs))
      algorithm
        res = extendsElts(xs);
      then
        res;
  end match;
end extendsElts;
*/
pub(crate) fn componentElts<'__b>(
    mut inSCodeElementLst: &'__b metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> metamodelica::List<metamodelica::Ref<SCode::Element>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inSCodeElementLst {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: cdef @ Deref @ SCode::Element::COMPONENT { .. }, tail: xs } => {
                let mut res: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                res = componentElts(xs);
                return metamodelica::cons(cdef.clone(), res)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut res: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                { inSCodeElementLst = xs; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn addClassdefsToEnv(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inClasses: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inImpl: bool,
    mut inRedeclareMod: Option<metamodelica::Ref<DAE::Mod>>,
    mut checkDuplicates: bool,
) -> Result<(FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outEnv: FCore::Graph = inEnv;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = inIH;
    for mut c in &**inClasses {
        (outCache, outEnv, outIH) = addClassdefToEnv(
            outCache,
            outEnv,
            outIH,
            inPrefix.clone(),
            c.clone(),
            inImpl,
            inRedeclareMod.clone(),
            checkDuplicates,
        )?;
    }
    Ok((outCache, outEnv, outIH))
}

fn addClassdefToEnv(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inSCodeElement: metamodelica::Ref<SCode::Element>,
    mut inBoolean: bool,
    mut redeclareMod: Option<metamodelica::Ref<DAE::Mod>>,
    mut checkDuplicates: bool,
) -> Result<(FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    (outCache, outEnv, outIH) = 'mc: {
        let __mc_input = (inCache, inEnv, inIH, inPrefix, inSCodeElement, &redeclareMod);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, sel1 @ Deref @ SCode::Element::CLASS { .. }, Some(m)) => {
                    let mut env_1: FCore::Graph;
                    let mut cl2: metamodelica::Ref<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut m = (*m).clone();
                    m = Mod::lookupCompModification(metamodelica::AsArg::as_arg(&m), var_field!((**sel1).name, SCode::Element::CLASS).clone())?;
                    let false = (m.clone() == openmodelica_frontend_types::DAE::Mod::interned_NOMOD()) else { return Err("pattern mismatch") };
                    env_1 = FGraph::mkClassNode(env.clone(), sel1.clone(), pre.clone(), m.clone(), false)?;
                    (cache, env_1, ih, cl2) = addClassdefsToEnv3(cache.clone(), env_1.clone(), ih.clone(), pre.clone(), redeclareMod.clone(), sel1.clone())?;
                    ih = InnerOuter::addClassIfInner(cl2.clone(), pre.clone(), &env_1, ih.clone());
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, sel1 @ Deref @ SCode::Element::CLASS { .. }, _) => {
                    let mut env_1: FCore::Graph;
                    let mut ih = (*ih).clone();
                    env_1 = FGraph::mkClassNode(env.clone(), sel1.clone(), pre.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), checkDuplicates)?;
                    ih = InnerOuter::addClassIfInner(sel1.clone(), pre.clone(), &env_1, ih.clone());
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, imp @ Deref @ SCode::Element::IMPORT { .. }, _) => {
                    let mut env_1: FCore::Graph;
                    env_1 = FGraph::mkImportNode(env.clone(), imp.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, elt @ Deref @ SCode::Element::DEFINEUNIT { .. }, _) => {
                    let mut env_1: FCore::Graph;
                    env_1 = FGraph::mkDefunitNode(env.clone(), elt.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
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
                    Debug::trace(literal!("- InstUtil.addClassdefToEnv2 failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH))
}

fn checkCompEnvPathVsCompTypePath(
    mut inCompEnvPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut inCompTypePath: metamodelica::Ref<Absyn::Path>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inCompEnvPath, inCompTypePath)) {
        (_, Deref @ Absyn::Path::IDENT { name: _ }) => {
            ()
        },
        (Some(ep), tp) => {
            let mut tp = (*tp).clone();
            tp = AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&tp))?;
            let true = (AbsynUtil::pathPrefixOf(tp.clone(), ep.clone())) else { return Err("pattern mismatch") };
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn addComponentsToEnv(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut ih: metamodelica::List<InnerOuter::TopInstance>,
    mut r#mod: metamodelica::Ref<DAE::Mod>,
    mut prefix: DAE::Prefix,
    mut state: ClassInf::State,
    mut components: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut r#impl: bool,
) -> Result<(FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>)> {
    let mut cache: FCore::Cache = cache;
    let mut env: FCore::Graph = env;
    let mut ih: metamodelica::List<InnerOuter::TopInstance> = ih;
    let mut comp: metamodelica::Ref<SCode::Element>;
    let mut comp2: metamodelica::Ref<SCode::Element> =
        <metamodelica::Ref<SCode::Element> as ::std::default::Default>::default();
    let mut cmod: metamodelica::Ref<DAE::Mod>;
    let mut local_mod: metamodelica::Ref<DAE::Mod> = metamodelica::Ref::new(DAE::Mod::NOMOD);
    let mut comp_mod: metamodelica::Ref<DAE::Mod> = metamodelica::Ref::new(DAE::Mod::NOMOD);
    let mut mod2: metamodelica::Ref<DAE::Mod> = metamodelica::Ref::new(DAE::Mod::NOMOD);
    let mut ty_path: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut prefs: metamodelica::Ref<SCode::Prefixes>;
    let mut attr: SCode::Attributes;
    let mut dattr: metamodelica::Ref<DAE::Attributes> =
        <metamodelica::Ref<DAE::Attributes> as ::std::default::Default>::default();
    let mut error: bool = false;
    let mut err_msg: ArcStr = arcstr::literal!("");
    for mut compmod in &**components {
        (comp, cmod) = compmod.clone();
        error = 'mc: {
            let __mc_input = comp.clone();
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ SCode::Element::COMPONENT { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: ty_path, .. }, .. } => {
                        if !((metamodelica::stringEq(&var_field!((*comp).name, SCode::Element::COMPONENT), &(AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&ty_path)))))) { return Err("guard") }
                        let mut err_msg: ArcStr = err_msg.clone();
                        checkCompEnvPathVsCompTypePath(FGraph::getScopePath(&env)?, ty_path.clone())?;
                        err_msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*comp).name, SCode::Element::COMPONENT)); __mm_s.push_str(&*literal!(" in env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(&env)); ArcStr::from(__mm_s) };
                        Error::addSourceMessage(&(Error::COMPONENT_NAME_SAME_AS_TYPE_NAME.clone()), list![err_msg.clone(), AbsynUtil::pathString(ty_path.clone(), literal!("."), true, false)?], var_field!((*comp).info, SCode::Element::COMPONENT))?;
                        Ok((true, err_msg.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                err_msg = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10)) =
                (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Deref @ SCode::Element::COMPONENT { prefixes: prefs @ Deref @ SCode::Prefixes { .. }, attributes: attr @ SCode::Attributes { .. }, .. } => {
                            let mut cache: FCore::Cache = cache.clone();
                            let mut cmod: metamodelica::Ref<DAE::Mod> = cmod.clone();
                            let mut comp: metamodelica::Ref<SCode::Element> = comp.clone();
                            let mut comp2: metamodelica::Ref<SCode::Element> = comp2.clone();
                            let mut comp_mod: metamodelica::Ref<DAE::Mod> = comp_mod.clone();
                            let mut dattr: metamodelica::Ref<DAE::Attributes> = dattr.clone();
                            let mut env: FCore::Graph = env.clone();
                            let mut ih: metamodelica::List<InnerOuter::TopInstance> = ih.clone();
                            let mut local_mod: metamodelica::Ref<DAE::Mod> = local_mod.clone();
                            let mut mod2: metamodelica::Ref<DAE::Mod> = mod2.clone();
                            let mut ty_path: metamodelica::Ref<Absyn::Path> = ty_path.clone();
                            ty_path = AbsynUtil::typeSpecPath(var_field!((*comp).typeSpec, SCode::Element::COMPONENT));
                            local_mod = Mod::lookupModificationP(r#mod.clone(), &ty_path)?;
                            if SCodeUtil::finalBool(SCodeUtil::prefixesFinal(metamodelica::AsArg::as_arg(&prefs))) {
                                assign_variant_field!(comp => SCode::Element::COMPONENT; modifications = traverseModAddFinal(var_field!((*comp).modifications, SCode::Element::COMPONENT).clone())?);
                            }
                            (cache, env, ih, comp2, mod2) = Inst::redeclareType(cache.clone(), env.clone(), ih.clone(), local_mod.clone(), comp.clone(), prefix.clone(), state.clone(), r#impl, cmod.clone())?;
                            comp_mod = Mod::lookupCompModification(&r#mod, var_field!((*comp).name, SCode::Element::COMPONENT).clone())?;
                            cmod = Mod::merge(comp_mod.clone(), cmod.clone(), literal!(""), true)?;
                            dattr = DAEUtil::translateSCodeAttrToDAEAttr(attr.clone(), metamodelica::AsArg::as_arg(&prefs));
                            env = FGraph::mkComponentNode(env.clone(), metamodelica::Ref::new(DAE::Var { name: var_field!((*comp).name, SCode::Element::COMPONENT).clone(), attributes: dattr.clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), comp.clone(), cmod.clone(), openmodelica_frontend_dump::FCore::Status::VAR_UNTYPED, FGraph::empty())?;
                            Ok((false, cache.clone(), cmod.clone(), comp.clone(), comp2.clone(), comp_mod.clone(), dattr.clone(), env.clone(), ih.clone(), local_mod.clone(), mod2.clone(), ty_path.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
                })()
            {
                cache = __wb0;
                cmod = __wb1;
                comp = __wb2;
                comp2 = __wb3;
                comp_mod = __wb4;
                dattr = __wb5;
                env = __wb6;
                ih = __wb7;
                local_mod = __wb8;
                mod2 = __wb9;
                ty_path = __wb10;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ SCode::Element::COMPONENT { .. } => {
                        Ok(true)
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok(false)
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        };
        if error {
            return Err("fail");
        }
    }
    Ok((cache, env, ih))
}

fn getCrefsFromCompdims(
    mut inComponents: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCrefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    outCrefs = 'mc: {
        let __mc_input = &**inComponents;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: arraydim, .. }, .. }, _), tail: xs } => {
                    let mut crefs1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut crefs2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    crefs1 = getCrefFromDim(metamodelica::AsArg::as_arg(&arraydim))?;
                    crefs2 = getCrefsFromCompdims(metamodelica::AsArg::as_arg(&xs));
                    crefs = listAppend(crefs1.clone(), crefs2.clone());
                    Ok(crefs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                    let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    crefs = getCrefsFromCompdims(metamodelica::AsArg::as_arg(&xs));
                    Ok(crefs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outCrefs
}

fn memberCrefs(
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRefs: &metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
) -> Result<bool> {
    let mut outIsMember: bool;
    outIsMember = List::isMemberOnTrue(inComponentRef, inComponentRefs, &move |__a0: metamodelica::Ref<
        Absyn::ComponentRef,
    >,
                                                                               __a1: metamodelica::Ref<
        Absyn::ComponentRef,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(AbsynUtil::crefEqualNoSubs(&__a0, &__a1))
    })?;
    Ok(outIsMember)
}

pub(crate) fn chainRedeclares(
    mut inModOuter: &metamodelica::Ref<DAE::Mod>,
    mut inModInner: metamodelica::Ref<SCode::Mod>,
) -> metamodelica::Ref<SCode::Mod> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    let mut b: bool;
    outMod = (match &*inModInner {
        _ => {
            (outMod, b) = chainRedeclare_dispatch(inModOuter, &inModInner);
            if (b) { outMod } else { inModInner }
        }
    });
    outMod
}

pub(crate) fn chainRedeclare_dispatch(
    mut inModOuter: &metamodelica::Ref<DAE::Mod>,
    mut inModInner: &metamodelica::Ref<SCode::Mod>,
) -> (metamodelica::Ref<SCode::Mod>, bool) {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    let mut change: bool = false;
    (outMod, change) = 'mc: {
        let __mc_input = &**inModInner;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::REDECL { finalPrefix: f, eachPrefix: e, element: Deref @ SCode::Element::CLASS { name: nInner, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: nDerivedInner }, .. }, .. }, .. } } => {
                    let mut cls: metamodelica::Ref<SCode::Element>;
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::lookupCompModification(inModOuter, nDerivedInner.clone())?) {
                        Deref @ DAE::Mod::REDECL { element: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cls = metamodelica::Own::own(__pa0);
                    cls = SCodeUtil::setClassName(nInner.clone(), cls.clone())?;
                    Ok((metamodelica::Ref::new(SCode::Mod::REDECL { finalPrefix: f.clone(), eachPrefix: e.clone(), element: cls.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::REDECL { finalPrefix: f, eachPrefix: e, element: Deref @ SCode::Element::CLASS { name: nInner, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: _ }, .. }, .. }, .. } } => {
                    let mut cls: metamodelica::Ref<SCode::Element>;
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::lookupCompModification(inModOuter, nInner.clone())?) {
                        Deref @ DAE::Mod::REDECL { element: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cls = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(SCode::Mod::REDECL { finalPrefix: f.clone(), eachPrefix: e.clone(), element: cls.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::MOD { finalPrefix: f, eachPrefix: e, subModLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: name, r#mod: m @ Deref @ SCode::Mod::REDECL { .. } }, tail: rest }, binding: b, comment: cmt, info } => {
                    let mut subs: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
                    let mut m2: metamodelica::Ref<SCode::Mod>;
                    (m2, _) = chainRedeclare_dispatch(inModOuter, metamodelica::AsArg::as_arg(&m));
                    let __pa0 = ::match_deref::match_deref! { match &(chainRedeclare_dispatch(inModOuter, &(metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: rest.clone(), binding: b.clone(), comment: cmt.clone(), info: info.clone() })))) {
                        (Deref @ SCode::Mod::MOD { subModLst: __pa0, .. }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    subs = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: metamodelica::cons(metamodelica::Ref::new(SCode::SubMod { ident: name.clone(), r#mod: m2.clone() }), subs.clone()), binding: b.clone(), comment: cmt.clone(), info: info.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::MOD { finalPrefix: f, eachPrefix: e, subModLst: Deref @ metamodelica::ListNode::Cons { head: sm, tail: rest }, binding: b, comment: cmt, info } => {
                    let mut subs: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
                    let mut change: bool = change.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(chainRedeclare_dispatch(inModOuter, &(metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: rest.clone(), binding: b.clone(), comment: cmt.clone(), info: info.clone() })))) {
                        (Deref @ SCode::Mod::MOD { subModLst: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    subs = metamodelica::Own::own(__pa0);
                    change = metamodelica::Own::own(__pa1);
                    Ok(((metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: metamodelica::cons(sm.clone(), subs.clone()), binding: b.clone(), comment: cmt.clone(), info: info.clone() }), change), change.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            change = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inModInner.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outMod, change)
}

fn addRecordConstructorsToTheCache(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inState: &ClassInf::State,
    mut inDirection: Absyn::Direction,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
) -> (FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>) {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    (outCache, outEnv, outIH) = 'mc: {
        let __mc_input = (inState, &*inClass);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ClassInf::State::FUNCTION { path, .. }, Deref @ SCode::Element::CLASS { name, restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. }) => {
                    let mut cache: FCore::Cache;
                    let mut env: FCore::Graph;
                    let mut ih: InstanceHierarchy;
                    metamodelica::print(literal!("Depreciated record constructor used: Inst.addRecordConstructorsToTheCache"));
                    let true = (AbsynUtil::isInputOrOutput(inDirection)) else { return Err("pattern mismatch") };
                    let false = (stringEq(&(AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path))), &name)) else { return Err("pattern mismatch") };
                    (cache, env, ih) = InstFunction::implicitFunctionInstantiation(inCache.clone(), inEnv.clone(), inIH.clone(), inMod.clone(), inPrefix.clone(), inClass.clone(), inInstDims.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inCache.clone(), inEnv.clone(), inIH.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outCache, outEnv, outIH)
}

pub(crate) fn checkMultiplyDeclared(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut r#mod: &metamodelica::Ref<DAE::Mod>,
    mut prefix: &DAE::Prefix,
    mut ciState: &ClassInf::State,
    mut compTuple: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
    mut instDims: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut r#impl: bool,
) -> Result<bool> {
    let mut alreadyDeclared: bool = false;
    alreadyDeclared = 'mc: {
        let __mc_input = compTuple;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::setCheckpoint(literal!("checkMultiplyDeclared"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, .. }, _) => {
                    ErrorExt::rollBack(literal!("checkMultiplyDeclared"));
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { .. }, Deref @ DAE::Mod::REDECL { finalPrefix: _, eachPrefix: _, element: _, .. }) => {
                    ErrorExt::rollBack(literal!("checkMultiplyDeclared"));
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                newComp @ (Deref @ SCode::Element::COMPONENT { name: n, .. }, _) => {
                    let mut oldElt: metamodelica::Ref<SCode::Element>;
                    let mut oldMod: metamodelica::Ref<DAE::Mod>;
                    let mut instStatus: FCore::Status;
                    let mut alreadyDeclared: bool = alreadyDeclared.clone();
                    (_, _, oldElt, oldMod, instStatus, _) = Lookup::lookupIdentLocal(cache.clone(), &env, n.clone())?;
                    checkMultipleElementsIdentical(cache.clone(), env.clone(), &((oldElt.clone(), oldMod.clone())), &(newComp.clone()))?;
                    alreadyDeclared = instStatusToBool(&instStatus)?;
                    ErrorExt::delCheckpoint(literal!("checkMultiplyDeclared"));
                    Ok((alreadyDeclared, alreadyDeclared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            alreadyDeclared = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { name: n, .. }, _) => {
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupIdentLocal(cache.clone(), &env, n.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    ErrorExt::rollBack(literal!("checkMultiplyDeclared"));
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, .. }, _) => {
                    ErrorExt::rollBack(literal!("checkMultiplyDeclared"));
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, Deref @ DAE::Mod::REDECL { finalPrefix: _, eachPrefix: _, element: _, .. }) => {
                    ErrorExt::rollBack(literal!("checkMultiplyDeclared"));
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { name: n, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: n2 }, .. }, tail: _ }, .. }, .. }, _) => {
                    let mut n = (*n).clone();
                    n = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$parent")); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*n); ArcStr::from(__mm_s) };
                    let 0 = (System::stringFind(n.clone(), n2.clone())?) else { return Err("pattern mismatch") };
                    ErrorExt::rollBack(literal!("checkMultiplyDeclared"));
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (newClass @ Deref @ SCode::Element::CLASS { name: n, .. }, _) => {
                    let mut oldClass: metamodelica::Ref<SCode::Element>;
                    (oldClass, _) = Lookup::lookupClassLocal(env.clone(), n.clone())?;
                    checkMultipleClassesEquivalent(oldClass.clone(), newClass.clone())?;
                    ErrorExt::delCheckpoint(literal!("checkMultiplyDeclared"));
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { name: n, .. }, _) => {
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupClassLocal(env.clone(), n.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    ErrorExt::rollBack(literal!("checkMultiplyDeclared"));
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::delCheckpoint(literal!("checkMultiplyDeclared"));
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("-Inst.checkMultiplyDeclared failed\n"))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(alreadyDeclared)
}

fn instStatusToBool(mut instStatus: &FCore::Status) -> Result<bool> {
    let mut alreadyDeclared: bool;
    alreadyDeclared = (match instStatus.clone() {
        FCore::Status::VAR_DAE { .. } => true,
        FCore::Status::VAR_UNTYPED { .. } => false,
        FCore::Status::VAR_TYPED { .. } => false,
        _ => return Err("match: no arm matched"),
    });
    Ok(alreadyDeclared)
}

fn checkMultipleElementsIdentical(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut oldComponent: &(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
    mut newComponent: &(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inCache, inEnv, oldComponent, newComponent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (oldElt, _), (newElt, _)) => {
                    let true = (SCodeUtil::elementEqual(metamodelica::AsArg::as_arg(&oldElt), metamodelica::AsArg::as_arg(&newElt))) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, (Deref @ SCode::Element::COMPONENT { name: n1, prefixes: prefixes1, attributes: attr1, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tpath1, arrayDim: ad1 }, modifications: smod1, comment: _, condition: cond1, info: _ }, _), (Deref @ SCode::Element::COMPONENT { name: n2, prefixes: prefixes2, attributes: attr2, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tpath2, arrayDim: ad2 }, modifications: smod2, comment: _, condition: cond2, info: _ }, _)) => {
                    let mut env1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut c1: metamodelica::Ref<SCode::Element>;
                    let mut c2: metamodelica::Ref<SCode::Element>;
                    let true = (stringEq(&n1, &n2)) else { return Err("pattern mismatch") };
                    let true = (SCodeUtil::prefixesEqual(metamodelica::AsArg::as_arg(&prefixes1), metamodelica::AsArg::as_arg(&prefixes2))) else { return Err("pattern mismatch") };
                    let true = (SCodeUtil::attributesEqual(metamodelica::AsArg::as_arg(&attr1), metamodelica::AsArg::as_arg(&attr2))) else { return Err("pattern mismatch") };
                    let true = (SCodeUtil::modEqual(metamodelica::AsArg::as_arg(&smod1), metamodelica::AsArg::as_arg(&smod2))) else { return Err("pattern mismatch") };
                    let true = (ad1.clone() == ad2.clone()) else { return Err("pattern mismatch") };
                    let true = (cond1.clone() == cond2.clone()) else { return Err("pattern mismatch") };
                    (_, c1, env1) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&tpath1), None)?;
                    (_, c2, env2) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&tpath2), None)?;
                    let true = (stringEq(&(FGraph::printGraphPathStr(&env1)), &(FGraph::printGraphPathStr(&env2)))) else { return Err("pattern mismatch") };
                    let true = (SCodeUtil::elementEqual(&c1, &c2)) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, (oldElt @ Deref @ SCode::Element::COMPONENT { name: n1, prefixes: prefixes1, attributes: attr1, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tpath1, arrayDim: ad1 }, modifications: smod1, comment: _, condition: cond1, info: old_info }, _), (newElt @ Deref @ SCode::Element::COMPONENT { name: n2, prefixes: prefixes2, attributes: attr2, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tpath2, arrayDim: ad2 }, modifications: smod2, comment: _, condition: cond2, info: new_info }, _)) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s: ArcStr;
                    let mut env1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut c1: metamodelica::Ref<SCode::Element>;
                    let mut c2: metamodelica::Ref<SCode::Element>;
                    let true = (stringEq(&n1, &n2)) else { return Err("pattern mismatch") };
                    let true = (stringEq(&n1, &(literal!("m_flow")))) else { return Err("pattern mismatch") };
                    let true = (SCodeUtil::prefixesEqual(metamodelica::AsArg::as_arg(&prefixes1), metamodelica::AsArg::as_arg(&prefixes2))) else { return Err("pattern mismatch") };
                    let true = (SCodeUtil::attributesEqual(metamodelica::AsArg::as_arg(&attr1), metamodelica::AsArg::as_arg(&attr2))) else { return Err("pattern mismatch") };
                    let false = (SCodeUtil::modEqual(metamodelica::AsArg::as_arg(&smod1), metamodelica::AsArg::as_arg(&smod2))) else { return Err("pattern mismatch") };
                    let true = (ad1.clone() == ad2.clone()) else { return Err("pattern mismatch") };
                    let true = (cond1.clone() == cond2.clone()) else { return Err("pattern mismatch") };
                    (_, c1, env1) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&tpath1), None)?;
                    (_, c2, env2) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&tpath2), None)?;
                    let true = (stringEq(&(FGraph::printGraphPathStr(&env1)), &(FGraph::printGraphPathStr(&env2)))) else { return Err("pattern mismatch") };
                    let true = (SCodeUtil::elementEqual(&c1, &c2)) else { return Err("pattern mismatch") };
                    s1 = SCodeDump::unparseElementStr(oldElt.clone(), SCodeDump::defaultOptions.clone())?;
                    s2 = SCodeDump::unparseElementStr(newElt.clone(), SCodeDump::defaultOptions.clone())?;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Inherited elements are not identical: bug: https://trac.modelica.org/Modelica/ticket/627\n\tfirst:  ")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!("\n\tsecond: ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*literal!("\nContinue ....")); ArcStr::from(__mm_s) };
                    Error::addMultiSourceMessage(&(Error::COMPILER_WARNING.clone()), &(list![s.clone()]), &(list![old_info.clone(), new_info.clone()]))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (oldElt @ Deref @ SCode::Element::COMPONENT { info: old_info, .. }, _), (newElt @ Deref @ SCode::Element::COMPONENT { info: new_info, .. }, _)) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    s1 = SCodeDump::unparseElementStr(oldElt.clone(), SCodeDump::defaultOptions.clone())?;
                    s2 = SCodeDump::unparseElementStr(newElt.clone(), SCodeDump::defaultOptions.clone())?;
                    Error::addMultiSourceMessage(&(Error::DUPLICATE_ELEMENTS_NOT_IDENTICAL.clone()), &(list![s1.clone(), s2.clone()]), &(list![old_info.clone(), new_info.clone()]))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn checkMultipleClassesEquivalent(
    mut oldClass: metamodelica::Ref<SCode::Element>,
    mut newClass: metamodelica::Ref<SCode::Element>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (oldClass, newClass);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::ENUMERATION { enumLst }, .. }, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst, .. }, .. }) => {
                    let mut sl1: metamodelica::List<ArcStr>;
                    let mut sl2: metamodelica::List<ArcStr>;
                    sl1 = List::map(enumLst.clone(), &move |__a0: metamodelica::Ref<SCode::Enum>| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::enumName(&__a0)) })?;
                    sl2 = List::map(elementLst.clone(), &move |__a0: metamodelica::Ref<SCode::Element>| SCodeUtil::elementName(&__a0))?;
                    let true = (List::isEqualOnTrue(sl1.clone(), sl2.clone(), &fnptr!(stringEq, ArcStr, ArcStr))?) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst, .. }, .. }, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::ENUMERATION { enumLst }, .. }) => {
                    let mut sl1: metamodelica::List<ArcStr>;
                    let mut sl2: metamodelica::List<ArcStr>;
                    sl1 = List::map(enumLst.clone(), &move |__a0: metamodelica::Ref<SCode::Enum>| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::enumName(&__a0)) })?;
                    sl2 = List::map(elementLst.clone(), &move |__a0: metamodelica::Ref<SCode::Element>| SCodeUtil::elementName(&__a0))?;
                    let true = (List::isEqualOnTrue(sl1.clone(), sl2.clone(), &fnptr!(stringEq, ArcStr, ArcStr))?) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (oldCl, newCl) => {
                    let true = (SCodeUtil::elementEqual(metamodelica::AsArg::as_arg(&oldCl), metamodelica::AsArg::as_arg(&newCl))) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (oldCl, newCl) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut info1: SourceInfo;
                    let mut info2: SourceInfo;
                    s1 = SCodeDump::unparseElementStr(oldCl.clone(), SCodeDump::defaultOptions.clone())?;
                    s2 = SCodeDump::unparseElementStr(newCl.clone(), SCodeDump::defaultOptions.clone())?;
                    info1 = SCodeUtil::elementInfo(metamodelica::AsArg::as_arg(&oldCl));
                    info2 = SCodeUtil::elementInfo(metamodelica::AsArg::as_arg(&newCl));
                    Error::addMultiSourceMessage(&(Error::DUPLICATE_CLASSES_NOT_EQUIVALENT.clone()), &(list![s1.clone(), s2.clone()]), &(list![info1.clone(), info2.clone()]))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

pub(crate) fn removeOptCrefFromCrefs(
    mut inCrefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    mut inCref: Option<metamodelica::Ref<Absyn::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>> {
    let mut outCrefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    outCrefs = (::match_deref::match_deref! { match &(inCref) {
        Some(cref) => {
            removeCrefFromCrefs(inCrefs, cref.clone())?
        },
        _ => {
            inCrefs
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCrefs)
}

pub(crate) fn removeCrefFromCrefs(
    mut inAbsynComponentRefLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inAbsynComponentRefLst, inComponentRef)) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(metamodelica::nil())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentRef::CREF_IDENT { name: n1, subscripts: Deref @ metamodelica::ListNode::Nil }, tail: rest }, cr2 @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: n2, subscripts: Deref @ metamodelica::ListNode::Nil }) if (stringEq(&n1, &n2)) => {
                let mut rest_1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                { (inAbsynComponentRefLst, inComponentRef) = (rest.clone(), cr2.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentRef::CREF_QUAL { name: n1, .. }, tail: rest }, cr2 @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: n2, .. }) if (stringEq(&n1, &n2)) => {
                let mut rest_1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                { (inAbsynComponentRefLst, inComponentRef) = (rest.clone(), cr2.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: cr1, tail: rest }, cr2) => {
                let mut rest_1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                rest_1 = removeCrefFromCrefs(rest.clone(), cr2.clone())?;
                return Ok(metamodelica::cons(cr1.clone(), rest_1))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn keepConstrainingTypeModifersOnly(
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut elems: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::Ref<DAE::Mod>> {
    let mut filteredMod: metamodelica::Ref<DAE::Mod>;
    filteredMod = (::match_deref::match_deref! { match &((inMod.clone(), elems.clone())) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            inMod
        },
        (Deref @ DAE::Mod::NOMOD { .. }, _) => {
            openmodelica_frontend_types::DAE::Mod::interned_NOMOD()
        },
        (Deref @ DAE::Mod::REDECL { finalPrefix: _, eachPrefix: _, element: _, .. }, _) => {
            inMod
        },
        (Deref @ DAE::Mod::MOD { finalPrefix: f, eachPrefix: e, subModLst: subs, binding: oe, info }, _) => {
            let mut compNames: metamodelica::List<ArcStr>;
            let mut subs = (*subs).clone();
            compNames = List::map(elems, &move |__a0: metamodelica::Ref<SCode::Element>| SCodeUtil::elementName(&__a0))?;
            subs = keepConstrainingTypeModifersOnly2(subs.clone(), &compNames)?;
            metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: f.clone(), eachPrefix: e.clone(), subModLst: subs.clone(), binding: oe.clone(), info: info.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(filteredMod)
}

fn keepConstrainingTypeModifersOnly2<'__b>(
    mut isubs: metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut elems: &'__b metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::SubMod>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((isubs, &**elems)) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(metamodelica::nil())
            },
            (subs, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(subs.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: sub @ Deref @ DAE::SubMod { ident: n, .. }, tail: subs }, _) if (List::isMemberOnTrue(n.clone(), elems, &fnptr!(stringEq, ArcStr, ArcStr))?) => {
                return Ok(metamodelica::cons(sub.clone(), keepConstrainingTypeModifersOnly2(subs.clone(), elems)?))
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: subs }, _) => {
                { (isubs, elems) = (subs.clone(), elems); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn extractConstrainingComps(
    mut cc: Option<metamodelica::Ref<SCode::ConstrainClass>>,
    mut env: &FCore::Graph,
    mut pre: &DAE::Prefix,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut elems: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    elems = 'mc: {
        let __mc_input = cc;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                None => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ SCode::ConstrainClass { constrainingClass: path, .. }) => {
                    let mut name: ArcStr;
                    let mut selems: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut extendselts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut compelts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut extcompelts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut classextendselts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut classes: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut extcomps: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupClass(&(FCore::emptyCache()), env, metamodelica::AsArg::as_arg(&path), None)?) {
                        (_, Deref @ SCode::Element::CLASS { name: __pa0, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa1, .. }, .. }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    name = metamodelica::Own::own(__pa0);
                    selems = metamodelica::Own::own(__pa1);
                    (classes, classextendselts, extendselts, compelts) = splitElts(&selems)?;
                    (_, _, _, _, extcomps, _, _, _, _, _) = InstExtends::instExtendsAndClassExtendsList(FCore::emptyCache(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), pre.clone(), extendselts.clone(), &classextendselts, selems.clone(), &(ClassInf::State::UNKNOWN { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }) }), name.clone(), true, false)?;
                    extcompelts = List::map(extcomps.clone(), &fnptr!(Util::tuple21, _))?;
                    compelts = listAppend(classes.clone(), listAppend(compelts.clone(), extcompelts.clone()));
                    Ok(compelts.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ SCode::ConstrainClass { constrainingClass: path, modifier: r#mod, comment: cmt }) => {
                    let mut compelts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut path = (*path).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupClass(&(FCore::emptyCache()), env, metamodelica::AsArg::as_arg(&path), None)?) {
                        (_, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa0, .. }, .. }, .. }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    path = metamodelica::Own::own(__pa0);
                    compelts = extractConstrainingComps(Some(metamodelica::Ref::new(SCode::ConstrainClass { constrainingClass: path.clone(), modifier: r#mod.clone(), comment: cmt.clone() })), env, pre)?;
                    Ok(compelts.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(elems)
}

pub(crate) fn moveBindings(mut inEquations: DAE::DAElist, mut inVariables: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outVariables: DAE::DAElist;
    let mut eqs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    if Config::getGraphicsExpMode()? {
        outVariables = inVariables;
        return Ok(outVariables);
    }
    let DAE::DAE { elementLst: __pa0 } = inEquations;
    eqs = metamodelica::Own::own(__pa0);
    let DAE::DAE { elementLst: __pa1 } = inVariables;
    vars = metamodelica::Own::own(__pa1);
    Error::assertion(
        intEq(((eqs).len() as i32), ((vars).len() as i32)),
        literal!("- InstUtil.moveBindings: Mismatched number of equations and variables."),
        &(Absyn::dummyInfo.clone()),
    )?;
    vars = List::threadMap(
        eqs,
        vars,
        &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::Element>| {
            moveBindings2(&__a0, &__a1)
        },
    )?;
    outVariables = DAE::DAElist { elementLst: vars };
    Ok(outVariables)
}

fn moveBindings2(
    mut inEquation: &metamodelica::Ref<DAE::Element>,
    mut inVariable: &metamodelica::Ref<DAE::Element>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outVariable: metamodelica::Ref<DAE::Element>;
    outVariable = 'mc: {
        let __mc_input = &**inVariable;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cref, kind, direction: dir, parallelism: prl, protection: vis, ty, binding: _, dims, connectorType: ct, source: src, variableAttributesOption: attr, comment: cmt, innerOuter: io, encrypted: e } => {
                    let mut bind_exp: metamodelica::Ref<DAE::Exp>;
                    bind_exp = moveBindings3(inEquation)?;
                    Ok(metamodelica::Ref::new(DAE::Element::VAR { componentRef: cref.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: prl.clone(), protection: vis.clone(), ty: ty.clone(), binding: Some(bind_exp.clone()), dims: dims.clone(), connectorType: ct.clone(), source: src.clone(), variableAttributesOption: attr.clone(), comment: cmt.clone(), innerOuter: io.clone(), encrypted: e.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cref, .. } => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.moveBindings failed on ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cref))?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVariable)
}

fn moveBindings3(mut inEquation: &metamodelica::Ref<DAE::Element>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outBinding: metamodelica::Ref<DAE::Exp>;
    outBinding = (match &**inEquation {
        DAE::Element::EQUATION { scalar: bind_exp, .. } => bind_exp.clone(),
        DAE::Element::DEFINE { exp: bind_exp, .. } => bind_exp.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outBinding)
}

pub(crate) fn checkModificationOnOuter(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inName: &ArcStr,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut inVariability: SCode::Variability,
    mut inInnerOuter: Absyn::InnerOuter,
    mut inImpl: bool,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&**inIH, &**inMod, inVariability, inInnerOuter);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, SCode::Variability::CONST { .. }, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, SCode::Variability::PARAM { .. }, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: InnerOuter::TopInstance { sm, .. }, tail: _ }, Deref @ DAE::Mod::MOD { .. }, _, Absyn::InnerOuter::OUTER { .. }) => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    cref = PrefixUtil::prefixToCref(inPrefix.clone())?;
                    let true = (BaseHashSet::has(cref.clone(), &(sm.clone()))?) else { return Err("pattern mismatch") };
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
                    let false = (InnerOuter::modificationOnOuter(inCache, inEnv, inIH, &inPrefix, inName, inCref, inMod, inInnerOuter, inImpl, inInfo)) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

pub(crate) fn checkFunctionVar(
    mut inName: ArcStr,
    mut inAttributes: &SCode::Attributes,
    mut inPrefixes: &metamodelica::Ref<SCode::Prefixes>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inAttributes, &**inPrefixes)) {
        (SCode::Attributes { direction: Absyn::Direction::BIDIR { .. }, .. }, Deref @ SCode::Prefixes { visibility: SCode::Visibility::PUBLIC { .. }, .. }) => {
            Error::addSourceMessage(&(Error::NON_FORMAL_PUBLIC_FUNCTION_VAR.clone()), list![inName], inInfo)?;
            ()
        },
        (SCode::Attributes { direction: Absyn::Direction::BIDIR { .. }, .. }, Deref @ SCode::Prefixes { visibility: SCode::Visibility::PROTECTED { .. }, .. }) => (),
        (SCode::Attributes { .. }, Deref @ SCode::Prefixes { visibility: SCode::Visibility::PROTECTED { .. }, .. }) => {
            Error::addSourceMessage(&(Error::PROTECTED_FORMAL_FUNCTION_VAR.clone()), list![inName], inInfo)?;
            return Err("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn checkFunctionVarType(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inState: &ClassInf::State,
    mut inVarName: ArcStr,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    if !(Types::isValidFunctionVarType(&inType)) {
        Error::addSourceMessage(
            &(Error::INVALID_FUNCTION_VAR_TYPE.clone()),
            list![TypesDump::getTypeName(inType), inVarName],
            inInfo,
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn liftNonBasicTypes(
    mut tp: metamodelica::Ref<DAE::Type>,
    mut dimt: metamodelica::Ref<DAE::Dimension>,
) -> metamodelica::Ref<DAE::Type> {
    let mut outTp: metamodelica::Ref<DAE::Type>;
    outTp = (match &*tp {
        DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. }
            if (!((TypesDump::getDimensions(metamodelica::AsArg::as_arg(&ty))).is_empty())) =>
        {
            tp
        }
        _ => Types::liftArray(tp, dimt),
    });
    outTp
}

pub(crate) fn checkHigherVariability(
    mut compConst: DAE::Const,
    mut bindConst: DAE::Const,
    mut pre: DAE::Prefix,
    mut name: ArcStr,
    mut binding: metamodelica::Ref<DAE::Exp>,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (compConst, bindConst, name, binding);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c, c1, _, _) => {
                    if !((c.clone() == c1.clone())) { return Err("guard") }
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Const::C_PARAM { .. }, DAE::Const::C_UNKNOWN { .. }, _, _) => {
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c, c1, n, e) => {
                    let mut sc: ArcStr;
                    let mut sc1: ArcStr;
                    let mut se: ArcStr;
                    let mut sn: ArcStr;
                    sn = { let mut __mm_s = String::new(); __mm_s.push_str(&*PrefixUtil::printPrefixStr2(pre.clone())?); __mm_s.push_str(&*n); ArcStr::from(__mm_s) };
                    sc = DAEUtil::constStr(c.clone())?;
                    sc1 = DAEUtil::constStr(c1.clone())?;
                    se = ExpressionBasics::printExpStr(e.clone())?;
                    Error::addSourceMessage(&(Error::HIGHER_VARIABILITY_BINDING.clone()), list![sn.clone(), sc.clone(), se.clone(), sc1.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

pub(crate) fn makeArrayType(
    mut inDimensionLst: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (&**inDimensionLst, inType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, ty) => {
                    Ok(ty.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: dim, tail: xs }, tty) => {
                    let mut ty_1: metamodelica::Ref<DAE::Type>;
                    ty_1 = makeArrayType(metamodelica::AsArg::as_arg(&xs), tty.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty_1.clone(), dims: list![dim.clone()] }))
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
                    Debug::trace(literal!("- InstUtil.makeArrayType failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outType)
}

pub(crate) fn getUsertypeDimensions(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut inBoolean: bool,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    metamodelica::Ref<SCode::Element>,
    metamodelica::Ref<DAE::Mod>,
)> {
    let mut outCache: FCore::Cache;
    let mut outDimensionLst: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut classToInstantiate: metamodelica::Ref<SCode::Element>;
    let mut outMods: metamodelica::Ref<DAE::Mod>;
    (outCache, outDimensionLst, classToInstantiate, outMods) = 'mc: {
        let __mc_input = (inCache, inEnv, inIH, inPrefix, inClass.clone(), inInstDims, inBoolean);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { name: id, .. }, _, _) => {
                    if !((metamodelica::stringEq(&id, &(literal!("Real"))) || metamodelica::stringEq(&id, &(literal!("Integer"))) || metamodelica::stringEq(&id, &(literal!("String"))) || metamodelica::stringEq(&id, &(literal!("Boolean"))))) { return Err("guard") }
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { name: Deref @ "Clock", .. }, _, _) => {
                    let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, classDef: Deref @ SCode::ClassDef::PARTS { .. }, .. }, _, _) => {
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, pre, cl @ Deref @ SCode::Element::CLASS { name: id, info, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { .. }, arrayDim: ad, .. }, .. }, .. }, dims, r#impl) => {
                    let mut owncref: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut ad_1: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut dim1: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut cache = (*cache).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    owncref = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: id.clone(), subscripts: metamodelica::nil() });
                    ad_1 = getOptionArraydim(ad.clone());
                    (cache, dim1) = elabArraydim(cache.clone(), env.clone(), owncref.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Integer") }), ad_1.clone(), None, r#impl.clone(), true, false, pre.clone(), info.clone(), dims.clone())?;
                    Ok((cache.clone(), dim1.clone(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }, partialPrefix: SCode::Partial::PARTIAL { .. }, .. }, _, _) => {
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ SCode::Element::CLASS { name: id, info, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: _ } }, partialPrefix: SCode::Partial::NOT_PARTIAL { .. }, .. }, _, _) => {
                    Error::addSourceMessage(&(Error::META_FUNCTION_TYPE_NO_PARTIAL_PREFIX.clone()), list![id.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_UNIONTYPE { .. }, .. }, _, _) => {
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ SCode::Element::CLASS { name: id, restriction: SCode::Restriction::R_TYPE { .. }, info, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: cn, arrayDim: ad }, modifications: r#mod, .. }, .. }, dims, r#impl) => {
                    let mut cl: metamodelica::Ref<SCode::Element>;
                    let mut cenv: FCore::Graph;
                    let mut owncref: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut ad_1: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut mod_1: metamodelica::Ref<DAE::Mod>;
                    let mut type_mods: metamodelica::Ref<DAE::Mod>;
                    let mut eq: Option<DAE::EqMod>;
                    let mut dim1: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut dim2: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    (cache, cl, cenv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cn), Some(info.clone()))?;
                    owncref = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: id.clone(), subscripts: metamodelica::nil() });
                    ad_1 = getOptionArraydim(ad.clone());
                    env = addEnumerationLiteralsToEnv(env.clone(), &cl);
                    (cache, mod_1) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), r#mod.clone(), r#impl.clone(), Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                    eq = Mod::modEquation(&mod_1);
                    (cache, dim1, cl, type_mods) = getUsertypeDimensions(cache.clone(), cenv.clone(), ih.clone(), pre.clone(), cl.clone(), dims.clone(), r#impl.clone())?;
                    (cache, dim2) = elabArraydim(cache.clone(), env.clone(), owncref.clone(), cn.clone(), ad_1.clone(), eq.clone(), r#impl.clone(), true, false, pre.clone(), info.clone(), dims.clone())?;
                    type_mods = Mod::addEachIfNeeded(type_mods.clone(), &dim2)?;
                    type_mods = Mod::merge(mod_1.clone(), type_mods.clone(), literal!(""), true)?;
                    res = listAppend(dim2.clone(), dim1.clone());
                    Ok((cache.clone(), res.clone(), cl.clone(), type_mods.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: els, normalEquationLst: Deref @ metamodelica::ListNode::Nil, initialEquationLst: Deref @ metamodelica::ListNode::Nil, normalAlgorithmLst: Deref @ metamodelica::ListNode::Nil, initialAlgorithmLst: Deref @ metamodelica::ListNode::Nil, .. }, .. }, _, r#impl) => {
                    let mut cl: metamodelica::Ref<SCode::Element>;
                    let mut mod_1: metamodelica::Ref<DAE::Mod>;
                    let mut type_mods: metamodelica::Ref<DAE::Mod>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut r#mod: metamodelica::Ref<SCode::Mod>;
                    let mut info: SourceInfo;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(splitElts(metamodelica::AsArg::as_arg(&els))?) {
                        (_, _, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: __pa0, visibility: _, modifications: __pa1, ann: _, info: __pa2 }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    path = metamodelica::Own::own(__pa0);
                    r#mod = metamodelica::Own::own(__pa1);
                    info = metamodelica::Own::own(__pa2);
                    (cache, mod_1) = Mod::elabModForBasicType(cache.clone(), env.clone(), ih.clone(), pre.clone(), r#mod.clone(), r#impl.clone(), Mod::ModScope::EXTENDS { path: path.clone() }, info.clone())?;
                    (cache, cl, _) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &path, None)?;
                    (cache, res, cl, type_mods) = getUsertypeDimensions(cache.clone(), env.clone(), ih.clone(), pre.clone(), cl.clone(), metamodelica::nil(), r#impl.clone())?;
                    type_mods = Mod::merge(mod_1.clone(), type_mods.clone(), literal!(""), true)?;
                    Ok((cache.clone(), res.clone(), cl.clone(), type_mods.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, cl @ Deref @ SCode::Element::CLASS { .. }, _, _) => {
                    Ok((cache.clone(), metamodelica::nil(), cl.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ SCode::Element::CLASS { .. }, _, _) => {
                    let mut id: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    id = SCodeDump::unparseElementStr(inClass.clone(), SCodeDump::defaultOptions.clone())?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstUtil.getUsertypeDimensions failed: ")); __mm_s.push_str(&*id); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outDimensionLst, classToInstantiate, outMods))
}

fn addEnumerationLiteralsToEnv(
    mut inEnv: FCore::Graph,
    mut inClass: &metamodelica::Ref<SCode::Element>,
) -> FCore::Graph {
    let mut outEnv: FCore::Graph;
    outEnv = 'mc: {
        let __mc_input = &**inClass;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: enums, .. }, .. } => {
                    let mut env: FCore::Graph;
                    env = List::fold(metamodelica::AsArg::as_arg(&enums), &addEnumerationLiteralToEnv, inEnv.clone())?;
                    Ok(env.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inEnv.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outEnv
}

fn addEnumerationLiteralToEnv(
    mut inEnum: metamodelica::Ref<SCode::Element>,
    mut inEnv: FCore::Graph,
) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    outEnv = 'mc: {
        let __mc_input = &*inEnum;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { name: lit, .. } => {
                    let mut env: FCore::Graph;
                    env = FGraph::mkComponentNode(inEnv.clone(), metamodelica::Ref::new(DAE::Var { name: lit.clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), inEnum.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_dump::FCore::Status::VAR_UNTYPED, FGraph::empty())?;
                    Ok(env.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("InstUtil.addEnumerationLiteralToEnv: Unknown enumeration type!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outEnv)
}

pub(crate) fn updateClassInfState(
    mut inCache: FCore::Cache,
    mut inNewEnv: FCore::Graph,
    mut inOldEnv: &FCore::Graph,
    mut inCIState: ClassInf::State,
) -> ClassInf::State {
    let mut outCIState: ClassInf::State;
    outCIState = 'mc: {
        let __mc_input = inCIState.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut ci_state = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (FGraph::isTopScope(&inNewEnv)) else {
                return Err("pattern mismatch");
            };
            Ok(ci_state.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut ci_state = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (stringEq(
                &(FGraph::getGraphNameStr(&inNewEnv)),
                &(FGraph::getGraphNameStr(inOldEnv)),
            )) else {
                return Err("pattern mismatch");
            };
            Ok(ci_state.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut ci_state: ClassInf::State;
            let mut rest: FCore::Graph;
            let mut id: ArcStr;
            let mut cls: metamodelica::Ref<SCode::Element>;
            let false = (FGraph::isTopScope(&inNewEnv)) else {
                return Err("pattern mismatch");
            };
            id = FNode::refName(FGraph::lastScopeRef(&inNewEnv)?);
            (rest, _) = FGraph::stripLastScopeRef(inNewEnv.clone())?;
            (_, cls, _) = Lookup::lookupClassIdent(inCache.clone(), rest.clone(), &id, None)?;
            ci_state = ClassInfUtil::start(
                &(SCodeUtil::getClassRestriction(&cls)?),
                FGraph::getGraphName(&inNewEnv)?,
            )?;
            Ok(ci_state.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(inCIState.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outCIState
}

pub(crate) fn evalEnumAndBoolDim(
    mut inDimension: metamodelica::Ref<DAE::Dimension>,
) -> metamodelica::Ref<DAE::Dimension> {
    let mut outDimension: metamodelica::Ref<DAE::Dimension>;
    outDimension = (match &*inDimension {
        DAE::Dimension::DIM_BOOLEAN { .. } => metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 2 }),
        _ => inDimension,
    });
    outDimension
}

/*TODO: mahge: Remove me*/
pub(crate) fn instDimExpNonSplit(
    mut inDimension: &metamodelica::Ref<DAE::Dimension>,
    mut inBoolean: bool,
) -> Result<metamodelica::Ref<DAE::Subscript>> {
    let mut outSubscript: metamodelica::Ref<DAE::Subscript>;
    outSubscript = (match &**inDimension {
        DAE::Dimension::DIM_UNKNOWN { .. } => openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(),
        DAE::Dimension::DIM_INTEGER { integer: i } => metamodelica::Ref::new(DAE::Subscript::WHOLE_NONEXP {
            exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }),
        }),
        DAE::Dimension::DIM_ENUM { size: i, .. } => metamodelica::Ref::new(DAE::Subscript::WHOLE_NONEXP {
            exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }),
        }),
        DAE::Dimension::DIM_BOOLEAN { .. } => metamodelica::Ref::new(DAE::Subscript::WHOLE_NONEXP {
            exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 2 }),
        }),
        DAE::Dimension::DIM_EXP { exp: e } => metamodelica::Ref::new(DAE::Subscript::WHOLE_NONEXP { exp: e.clone() }),
    });
    Ok(outSubscript)
}

pub(crate) fn instWholeDimFromMod(
    mut dimensionExp: &metamodelica::Ref<DAE::Dimension>,
    mut modifier: &metamodelica::Ref<DAE::Mod>,
    mut inVarName: ArcStr,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Dimension>> {
    let mut outDimension: metamodelica::Ref<DAE::Dimension>;
    outDimension = 'mc: {
        let __mc_input = (&**dimensionExp, &**modifier);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: exp, .. }), .. }) => {
                    let mut d: metamodelica::Ref<DAE::Dimension>;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::expDimensions(metamodelica::AsArg::as_arg(&exp))?) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    d = metamodelica::Own::own(__pa0);
                    Ok(d.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: exp, .. }), .. }) => {
                    let mut exp_str: ArcStr;
                    exp_str = ExpressionBasics::printExpStr(exp.clone())?;
                    Error::addSourceMessage(&(Error::FAILURE_TO_DEDUCE_DIMS_FROM_MOD.clone()), list![inVarName.clone(), exp_str.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- InstUtil.instWholeDimFromMod failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outDimension)
}

pub(crate) fn propagateAttributes(
    mut inDae: DAE::DAElist,
    mut inAttributes: SCode::Attributes,
    mut inPrefixes: metamodelica::Ref<SCode::Prefixes>,
    mut inInfo: SourceInfo,
) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    let mut elts: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let DAE::DAE { elementLst: __pa0 } = inDae;
    elts = metamodelica::Own::own(__pa0);
    elts = List::map3(elts, &propagateAllAttributes, inAttributes, inPrefixes, inInfo)?;
    outDae = DAE::DAElist { elementLst: elts };
    Ok(outDae)
}

fn propagateAllAttributes(
    mut inElement: metamodelica::Ref<DAE::Element>,
    mut inAttributes: SCode::Attributes,
    mut inPrefixes: metamodelica::Ref<SCode::Prefixes>,
    mut inInfo: SourceInfo,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outElement: metamodelica::Ref<DAE::Element>;
    outElement = (::match_deref::match_deref! { match &((inElement.clone(), inAttributes.clone(), inPrefixes.clone())) {
        (_, SCode::Attributes { connectorType: SCode::ConnectorType::POTENTIAL { .. }, parallelism: SCode::Parallelism::NON_PARALLEL { .. }, variability: SCode::Variability::VAR { .. }, direction: Absyn::Direction::BIDIR { .. }, .. }, Deref @ SCode::Prefixes { visibility: SCode::Visibility::PUBLIC { .. }, finalPrefix: SCode::Final::NOT_FINAL { .. }, innerOuter: Absyn::InnerOuter::NOT_INNER_OUTER { .. }, .. }) => {
            inElement
        },
        (Deref @ DAE::Element::VAR { componentRef: cr, kind: vk, direction: vdir, parallelism: vprl, protection: vvis, ty, binding, dims, connectorType: ct2, source, variableAttributesOption: var_attrs, comment: cmt, innerOuter: io2, encrypted: e }, SCode::Attributes { connectorType: ct1, parallelism: sprl, variability: var, direction: dir, .. }, Deref @ SCode::Prefixes { visibility: vis, finalPrefix: fp, innerOuter: io1, .. }) => {
            let mut vk = (*vk).clone();
            let mut vdir = (*vdir).clone();
            let mut vprl = (*vprl).clone();
            let mut vvis = (*vvis).clone();
            let mut ct2 = (*ct2).clone();
            let mut var_attrs = (*var_attrs).clone();
            let mut io2 = (*io2).clone();
            vdir = propagateDirection(vdir.clone(), dir.clone(), metamodelica::AsArg::as_arg(&cr), &inInfo)?;
            vk = propagateVariability(vk.clone(), var.clone());
            vprl = propagateParallelism(vprl.clone(), sprl.clone(), metamodelica::AsArg::as_arg(&cr), &inInfo)?;
            vvis = propagateVisibility(vvis.clone(), vis.clone());
            var_attrs = propagateFinal(var_attrs.clone(), fp.clone())?;
            io2 = propagateInnerOuter(io2.clone(), io1.clone());
            ct2 = propagateConnectorType(ct2.clone(), ct1.clone(), metamodelica::AsArg::as_arg(&cr), &inInfo)?;
            metamodelica::Ref::new(DAE::Element::VAR { componentRef: cr.clone(), kind: vk.clone(), direction: vdir.clone(), parallelism: vprl.clone(), protection: vvis.clone(), ty: ty.clone(), binding: binding.clone(), dims: dims.clone(), connectorType: ct2.clone(), source: source.clone(), variableAttributesOption: var_attrs.clone(), comment: cmt.clone(), innerOuter: io2.clone(), encrypted: e.clone() })
        },
        (Deref @ DAE::Element::COMP { ident, dAElist: el, source, comment: cmt }, _, _) => {
            let mut el = (*el).clone();
            el = List::map3(el.clone(), &propagateAllAttributes, inAttributes, inPrefixes, inInfo)?;
            metamodelica::Ref::new(DAE::Element::COMP { ident: ident.clone(), dAElist: el.clone(), source: source.clone(), comment: cmt.clone() })
        },
        _ => {
            inElement
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElement)
}

fn propagateDirection(
    mut inVarDirection: DAE::VarDirection,
    mut inDirection: Absyn::Direction,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> Result<DAE::VarDirection> {
    let mut outVarDirection: DAE::VarDirection;
    outVarDirection = (match (inVarDirection, inDirection) {
        (_, Absyn::Direction::BIDIR { .. }) => inVarDirection,
        (DAE::VarDirection::BIDIR { .. }, _) => absynDirToDaeDir(inDirection)?,
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            s1 = Dump::directionSymbol(inDirection)?;
            s2 = ComponentReferenceBasics::printComponentRefStr(inCref)?;
            s3 = DAEDump::dumpDirectionStr(inVarDirection);
            Error::addSourceMessage(
                &(Error::COMPONENT_INPUT_OUTPUT_MISMATCH.clone()),
                list![s1, s2, s3],
                inInfo,
            )?;
            return Err("fail");
        }
    });
    Ok(outVarDirection)
}

fn propagateParallelism(
    mut inVarParallelism: DAE::VarParallelism,
    mut inParallelism: SCode::Parallelism,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> Result<DAE::VarParallelism> {
    let mut outVarParallelism: DAE::VarParallelism;
    outVarParallelism = 'mc: {
        let __mc_input = (inVarParallelism, inParallelism);
        if let Ok(__v) = (|| -> Result<_> {
            let (_, SCode::Parallelism::NON_PARALLEL { .. }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(inVarParallelism)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::VarParallelism::NON_PARALLEL { .. }, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(DAEUtil::scodePrlToDaePrl(inParallelism))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut daeprl1, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut daeprl2: DAE::VarParallelism;
            daeprl2 = DAEUtil::scodePrlToDaePrl(inParallelism);
            let true = (DAEUtil::daeParallelismEqual(daeprl1.clone(), daeprl2)) else {
                return Err("pattern mismatch");
            };
            Ok(daeprl1.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut s4: ArcStr;
            let mut daeprl2: DAE::VarParallelism;
            daeprl2 = DAEUtil::scodePrlToDaePrl(inParallelism);
            s1 = DAEDump::dumpVarParallelismStr(daeprl2);
            s2 = ComponentReferenceBasics::printComponentRefStr(inCref)?;
            s3 = DAEDump::dumpVarParallelismStr(inVarParallelism);
            s4 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*literal!("- Component declared as '"));
                __mm_s.push_str(&*s1);
                __mm_s.push_str(&*literal!("' when having the variable '"));
                __mm_s.push_str(&*s2);
                __mm_s.push_str(&*literal!("' declared as '"));
                __mm_s.push_str(&*s3);
                __mm_s.push_str(&*literal!("' : Subcomponent parallelism modified to."));
                __mm_s.push_str(&*s1);
                ArcStr::from(__mm_s)
            };
            Error::addSourceMessage(&(Error::PARMODELICA_WARNING.clone()), list![s4.clone()], inInfo)?;
            Ok(daeprl2)
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVarParallelism)
}

fn propagateVisibility(
    mut inVarVisibility: DAE::VarVisibility,
    mut inVisibility: SCode::Visibility,
) -> DAE::VarVisibility {
    let mut outVarVisibility: DAE::VarVisibility;
    outVarVisibility = (match inVisibility {
        SCode::Visibility::PROTECTED { .. } => openmodelica_frontend_types::DAE::VarVisibility::PROTECTED,
        _ => inVarVisibility,
    });
    outVarVisibility
}

fn propagateVariability(mut inVarKind: DAE::VarKind, mut inVariability: SCode::Variability) -> DAE::VarKind {
    let mut outVarKind: DAE::VarKind;
    outVarKind = (match (inVarKind, inVariability) {
        (_, SCode::Variability::VAR { .. }) => inVarKind,
        (DAE::VarKind::DISCRETE { .. }, _) => inVarKind,
        (_, SCode::Variability::DISCRETE { .. }) => openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        (DAE::VarKind::CONST { .. }, _) => inVarKind,
        (_, SCode::Variability::CONST { .. }) => openmodelica_frontend_types::DAE::VarKind::CONST,
        (DAE::VarKind::PARAM { .. }, _) => inVarKind,
        (_, SCode::Variability::PARAM { .. }) => openmodelica_frontend_types::DAE::VarKind::PARAM,
        _ => inVarKind,
    });
    outVarKind
}

fn propagateFinal(
    mut inVarAttributes: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut inFinal: SCode::Final,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut outVarAttributes: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    outVarAttributes = (match inFinal {
        SCode::Final::FINAL { .. } => DAEUtil::setFinalAttr(inVarAttributes, SCodeUtil::finalBool(inFinal))?,
        _ => inVarAttributes,
    });
    Ok(outVarAttributes)
}

fn propagateInnerOuter(
    mut inVarInnerOuter: Absyn::InnerOuter,
    mut inInnerOuter: Absyn::InnerOuter,
) -> Absyn::InnerOuter {
    let mut outVarInnerOuter: Absyn::InnerOuter;
    outVarInnerOuter = (match (inVarInnerOuter, inInnerOuter) {
        (_, Absyn::InnerOuter::NOT_INNER_OUTER { .. }) => inVarInnerOuter,
        (Absyn::InnerOuter::NOT_INNER_OUTER { .. }, _) => inInnerOuter,
        _ => inVarInnerOuter,
    });
    outVarInnerOuter
}

fn propagateConnectorType(
    mut inVarConnectorType: metamodelica::Ref<DAE::ConnectorType>,
    mut inConnectorType: SCode::ConnectorType,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::ConnectorType>> {
    let mut outVarConnectorType: metamodelica::Ref<DAE::ConnectorType>;
    outVarConnectorType = (::match_deref::match_deref! { match &((inVarConnectorType.clone(), inConnectorType)) {
        (_, SCode::ConnectorType::POTENTIAL { .. }) => {
            inVarConnectorType
        },
        (Deref @ DAE::ConnectorType::POTENTIAL { .. }, SCode::ConnectorType::FLOW { .. }) => {
            openmodelica_frontend_types::DAE::ConnectorType::interned_FLOW()
        },
        (Deref @ DAE::ConnectorType::NON_CONNECTOR { .. }, SCode::ConnectorType::FLOW { .. }) => {
            openmodelica_frontend_types::DAE::ConnectorType::interned_FLOW()
        },
        (Deref @ DAE::ConnectorType::POTENTIAL { .. }, SCode::ConnectorType::STREAM { .. }) => {
            metamodelica::Ref::new(DAE::ConnectorType::STREAM { associatedFlow: None })
        },
        (Deref @ DAE::ConnectorType::NON_CONNECTOR { .. }, SCode::ConnectorType::STREAM { .. }) => {
            metamodelica::Ref::new(DAE::ConnectorType::STREAM { associatedFlow: None })
        },
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            s1 = SCodeDump::connectorTypeStr(inConnectorType);
            s2 = ComponentReferenceBasics::printComponentRefStr(inCref)?;
            s3 = DAEDump::dumpConnectorType(&inVarConnectorType);
            Error::addSourceMessage(&(Error::INVALID_TYPE_PREFIX.clone()), list![s1, literal!("variable"), s2, s3], inInfo)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVarConnectorType)
}

fn absynDirToDaeDir(mut inDirection: Absyn::Direction) -> Result<DAE::VarDirection> {
    let mut outVarDirection: DAE::VarDirection;
    outVarDirection = (match inDirection {
        Absyn::Direction::INPUT { .. } => openmodelica_frontend_types::DAE::VarDirection::INPUT,
        Absyn::Direction::OUTPUT { .. } => openmodelica_frontend_types::DAE::VarDirection::OUTPUT,
        Absyn::Direction::BIDIR { .. } => openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        _ => return Err("match: no arm matched"),
    });
    Ok(outVarDirection)
}

fn attrIsParam(mut inAttributes: &SCode::Attributes) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match inAttributes.clone() {
        SCode::Attributes {
            variability: SCode::Variability::PARAM { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn elabComponentArraydimFromEnv(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Dimension>>)> {
    let mut outCache: FCore::Cache;
    let mut outDimensionLst: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    (outCache, outDimensionLst) = 'mc: {
        let __mc_input = (inCache, inEnv, inComponentRef);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. }) => {
                    let mut m: metamodelica::Ref<SCode::Mod>;
                    let mut m_1: metamodelica::Ref<SCode::Mod>;
                    let mut cmod: metamodelica::Ref<DAE::Mod>;
                    let mut cmod_1: metamodelica::Ref<DAE::Mod>;
                    let mut m_2: metamodelica::Ref<DAE::Mod>;
                    let mut mod_2: metamodelica::Ref<DAE::Mod>;
                    let mut eq: DAE::EqMod;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupIdent(cache.clone(), metamodelica::AsArg::as_arg(&env), id.clone())?) {
                        (__pa0, _, Deref @ SCode::Element::COMPONENT { modifications: __pa1, .. }, __pa2, _, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    m = metamodelica::Own::own(__pa1);
                    cmod = metamodelica::Own::own(__pa2);
                    cmod_1 = Mod::stripSubmod(cmod.clone());
                    m_1 = SCodeUtil::stripSubmod(m.clone());
                    (cache, m_2) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, m_1.clone(), false, Mod::ModScope::COMPONENT { name: id.clone() }, info.clone())?;
                    mod_2 = Mod::merge(cmod_1.clone(), m_2.clone(), literal!(""), true)?;
                    let __pa4 = ::match_deref::match_deref! { match &(Mod::modEquation(&mod_2)) {
                        Some(__pa4) => __pa4.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    eq = metamodelica::Own::own(__pa4);
                    (cache, dims) = elabComponentArraydimFromEnv2(cache.clone(), &eq, metamodelica::AsArg::as_arg(&env))?;
                    Ok((cache.clone(), dims.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. }) => {
                    let mut ad: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupIdent(cache.clone(), metamodelica::AsArg::as_arg(&env), id.clone())?) {
                        (__pa0, _, Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: __pa1, .. }, .. }, _, _, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    ad = metamodelica::Own::own(__pa1);
                    (cache, subs, _) = Static::elabSubscripts(cache.clone(), env.clone(), &ad, true, openmodelica_frontend_types::DAE::Prefix::NOPRE, &info)?;
                    dims = Expression::subscriptDimensions(subs.clone())?;
                    Ok((cache.clone(), dims.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, cref) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstUtil.elabComponentArraydimFromEnv failed: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cref))?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outDimensionLst))
}

fn elabComponentArraydimFromEnv2(
    mut inCache: FCore::Cache,
    mut inEqMod: &DAE::EqMod,
    mut inEnv: &FCore::Graph,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Dimension>>)> {
    let mut outCache: FCore::Cache;
    let mut outDimensionLst: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    (outCache, outDimensionLst) = (match (inCache, inEqMod.clone()) {
        (
            mut cache,
            DAE::EqMod::TYPED {
                properties: DAE::Properties::PROP { type_: ref t, .. },
                ..
            },
        ) => {
            let mut lst: metamodelica::List<i32>;
            let mut lst_1: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            lst = Types::getDimensionSizes(metamodelica::AsArg::as_arg(&t))?;
            lst_1 = List::map(lst, &fnptr!(Expression::intDimension, i32))?;
            (cache, lst_1)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outCache, outDimensionLst))
}

pub(crate) fn elabArraydimOpt(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut inAbsynArrayDimOption: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>,
    mut inTypesEqModOption: Option<DAE::EqMod>,
    mut inBoolean: bool,
    mut performVectorization: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Dimension>>)> {
    let mut outCache: FCore::Cache;
    let mut outDimensionLst: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    (outCache, outDimensionLst) = (::match_deref::match_deref! { match &(inAbsynArrayDimOption) {
        Some(ad) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut owncref = inComponentRef;
            let mut eq = inTypesEqModOption;
            let mut r#impl = inBoolean;
            let mut doVect = performVectorization;
            let mut pre = inPrefix;
            let mut inst_dims = inInstDims;
            let mut res: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            (cache, res) = elabArraydim(cache, env, owncref, path, ad.clone(), eq, r#impl, doVect, false, pre, info, inst_dims)?;
            (cache, res)
        },
        None => {
            let mut cache = inCache;
            (cache, metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outDimensionLst))
}

pub(crate) fn elabArraydim(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut inArrayDim: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inTypesEqModOption: Option<DAE::EqMod>,
    mut inBoolean: bool,
    mut performVectorization: bool,
    mut isFunctionInput: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Dimension>>)> {
    let mut outCache: FCore::Cache;
    let mut outDimensionLst: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    (outCache, outDimensionLst) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inComponentRef,
            inArrayDim,
            inTypesEqModOption,
            inBoolean,
            performVectorization,
            isFunctionInput,
            inPrefix,
            inInfo,
            inInstDims,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, ad, _, _, doVect, true, pre, info, _) => {
                    let mut dim: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut cache = (*cache).clone();
                    (cache, dim) = Static::elabArrayDims(cache.clone(), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&ad), true, doVect.clone(), metamodelica::AsArg::as_arg(&pre), metamodelica::AsArg::as_arg(&info))?;
                    Ok((cache.clone(), dim.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, Deref @ metamodelica::ListNode::Nil, _, _, _, _, _, _, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, ad, None, r#impl, doVect, _, pre, info, _) => {
                    let mut dim: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut cache = (*cache).clone();
                    (cache, dim) = Static::elabArrayDims(cache.clone(), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&ad), r#impl.clone(), doVect.clone(), metamodelica::AsArg::as_arg(&pre), metamodelica::AsArg::as_arg(&info))?;
                    Ok((cache.clone(), dim.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, ad, Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: _, properties: prop, modifierAsAbsynExp: _, .. }), r#impl, doVect, _, pre, info, inst_dims) => {
                    let mut dim1: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut dim2: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut dim3: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    t = Types::getPropType(metamodelica::AsArg::as_arg(&prop));
                    (cache, dim1) = Static::elabArrayDims(cache.clone(), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&ad), r#impl.clone(), doVect.clone(), metamodelica::AsArg::as_arg(&pre), metamodelica::AsArg::as_arg(&info))?;
                    dim2 = elabArraydimType(&t, ad.clone(), e.clone(), path.clone(), pre.clone(), metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&info), inst_dims.clone())?;
                    dim3 = List::threadMap(dim1.clone(), dim2.clone(), &compatibleArraydim)?;
                    Ok((cache.clone(), dim3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, ad, Some(DAE::EqMod::UNTYPED { exp: aexp }), r#impl, doVect, _, pre, info, inst_dims) => {
                    let mut dim1: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut dim2: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut dim3: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = Static::elabExp(cache.clone(), env.clone(), aexp.clone(), r#impl.clone(), doVect.clone(), pre.clone(), info.clone())?;
                    (cache, e_1, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), e_1.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    t = Types::getPropType(&prop);
                    (cache, dim1) = Static::elabArrayDims(cache.clone(), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&ad), r#impl.clone(), doVect.clone(), metamodelica::AsArg::as_arg(&pre), metamodelica::AsArg::as_arg(&info))?;
                    dim2 = elabArraydimType(&t, ad.clone(), e_1.clone(), path.clone(), pre.clone(), metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&info), inst_dims.clone())?;
                    dim3 = List::threadMap(dim1.clone(), dim2.clone(), &compatibleArraydim)?;
                    Ok((cache.clone(), dim3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cref, ad, Some(DAE::EqMod::TYPED { modifierAsExp: e, modifierAsValue: _, properties: DAE::Properties::PROP { type_: t, constFlag: _ }, modifierAsAbsynExp: _, info: info2 }), r#impl, doVect, _, pre, info, inst_dims) => {
                    let mut dim1: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut dim2: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut e_str: ArcStr;
                    let mut t_str: ArcStr;
                    let mut dim_str: ArcStr;
                    let false = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    (_, dim1) = Static::elabArrayDims(cache.clone(), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&ad), r#impl.clone(), doVect.clone(), metamodelica::AsArg::as_arg(&pre), metamodelica::AsArg::as_arg(&info))?;
                    dim2 = elabArraydimType(metamodelica::AsArg::as_arg(&t), ad.clone(), e.clone(), path.clone(), pre.clone(), metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&info), inst_dims.clone())?;
                    if '__try0: {
                        unwrap_break_err!(List::threadMap(dim1.clone(), dim2.clone(), &compatibleArraydim), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    e_str = ExpressionBasics::printExpStr(e.clone())?;
                    t_str = TypesDump::unparseType(t.clone())?;
                    dim_str = ExpressionBasics::dimensionsString(dim1.clone())?;
                    Error::addMultiSourceMessage(&(Error::ARRAY_DIMENSION_MISMATCH.clone()), &(list![e_str.clone(), t_str.clone(), dim_str.clone()]), &(metamodelica::cons(info2.clone(), metamodelica::cons(info.clone(), metamodelica::nil()))))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, cref, ad, eq, _, _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- InstUtil.elabArraydim failed on: \n\tcref:"))?;
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&cref))?); ArcStr::from(__mm_s) })?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::printArraydimStr(ad.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*TypesDump::unparseOptionEqMod(eq.clone())?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outDimensionLst))
}

fn compatibleArraydim(
    mut inDimension1: metamodelica::Ref<DAE::Dimension>,
    mut inDimension2: metamodelica::Ref<DAE::Dimension>,
) -> Result<metamodelica::Ref<DAE::Dimension>> {
    let mut outDimension: metamodelica::Ref<DAE::Dimension>;
    outDimension = (::match_deref::match_deref! { match &((inDimension1.clone(), inDimension2.clone())) {
        (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, Deref @ DAE::Dimension::DIM_UNKNOWN { .. }) => openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN(),
        (_, Deref @ DAE::Dimension::DIM_UNKNOWN { .. }) => inDimension1,
        (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, _) => inDimension2,
        (_, Deref @ DAE::Dimension::DIM_EXP { .. }) => inDimension1,
        (Deref @ DAE::Dimension::DIM_EXP { .. }, _) => inDimension2,
        (_, _) => {
            let true = (intEq(Expression::dimensionSize(&inDimension1)?, Expression::dimensionSize(&inDimension2)?)) else { return Err("pattern mismatch") };
            inDimension1
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::trace(literal!("- InstUtil.compatibleArraydim failed\n"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDimension)
}

fn elabArraydimType(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inArrayDim: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inPrefix: DAE::Prefix,
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inInfo: &SourceInfo,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Dimension>>> {
    let mut outDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut flat_id: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut ad_str: ArcStr;
    let mut ty_str: ArcStr;
    let mut exp_str: ArcStr;
    let mut name_str: ArcStr;
    flat_id = if (Config::splitArrays()?) {
        metamodelica::nil()
    } else {
        List::flatten(inInstDims)?
    };
    match '__try0: {
        let true = (Types::numberOfDimensions(inType) >= ((inArrayDim).len() as i32) + ((flat_id).len() as i32)) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        outDimensions = unwrap_break_err!(elabArraydimType2(inType, &inArrayDim, &flat_id), '__try0);
        Ok::<_, &'static str>((outDimensions.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outDimensions = __try0_o0;
        }
        Err(_) => {
            ad_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*AbsynUtil::pathString(inPath.clone(), literal!("."), true, false)?);
                __mm_s.push_str(&*Dump::printArraydimStr(inArrayDim.clone())?);
                ArcStr::from(__mm_s)
            };
            ty_str = TypesDump::unparseTypeNoAttr(inType)?;
            exp_str = ExpressionBasics::printExpStr(inExp.clone())?;
            name_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*PrefixUtil::printPrefixStrIgnoreNoPre(inPrefix.clone())?);
                __mm_s.push_str(&*Dump::printComponentRefStr(inCref)?);
                ArcStr::from(__mm_s)
            };
            Error::addSourceMessageAndFail(
                &(Error::MODIFIER_DECLARATION_TYPE_MISMATCH_ERROR.clone()),
                list![name_str.clone(), ad_str.clone(), exp_str.clone(), ty_str.clone()],
                inInfo,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    Ok(outDimensions)
}

fn elabArraydimType2(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inArrayDim: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inDims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Dimension>>> {
    let mut outDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    outDimensions = 'mc: {
        let __mc_input = (&**inType, &**inArrayDim, &**inDims);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: d, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, _, Deref @ metamodelica::ListNode::Cons { head: dim, tail: rest_dims }) => {
                    compatibleArraydim(d.clone(), dim.clone())?;
                    Ok(elabArraydimType2(metamodelica::AsArg::as_arg(&t), inArrayDim, metamodelica::AsArg::as_arg(&rest_dims))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: d, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, _, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::cons(d.clone(), elabArraydimType2(metamodelica::AsArg::as_arg(&t), &((inArrayDim).rest()?), &(metamodelica::nil()))?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("Undefined! The type detected: "))?;
                    Debug::traceln(TypesDump::printTypeStr(inType.clone()))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outDimensions)
}

pub(crate) fn addFunctionsToDAE(
    mut inCache: FCore::Cache,
    mut funcs: &metamodelica::List<DAE::Function>,
    mut inPartialPrefix: SCode::Partial,
) -> Result<FCore::Cache> {
    let mut outCache: FCore::Cache;
    outCache = (match inCache {
        mut cache => {
            cache = FCore::addDaeFunction(cache, funcs)?;
            cache
        }
    });
    Ok(outCache)
}

pub(crate) fn addNameToDerivativeMapping(
    mut inElts: metamodelica::List<DAE::Function>,
    mut path: metamodelica::Ref<Absyn::Path>,
) -> metamodelica::List<DAE::Function> {
    let mut outElts: metamodelica::List<DAE::Function>;
    outElts = ({
        let mut __acc: metamodelica::List<DAE::Function> = metamodelica::nil();
        for mut r#fn in (inElts).into_iter().cloned() {
            let __x = (match r#fn.clone() {
                DAE::Function::FUNCTION { .. } => {
                    let __owned_variant_functions_0 = addNameToDerivativeMappingFunctionDefs(
                        var_field!(r#fn.functions, DAE::Function::FUNCTION).clone(),
                        path.clone(),
                    );
                    if let DAE::Function::FUNCTION { functions, .. } = &mut r#fn {
                        *functions = __owned_variant_functions_0;
                    } else {
                        panic!(
                            "owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION"
                        );
                    }
                    r#fn.clone()
                }
                _ => r#fn.clone(),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outElts
}

fn addNameToDerivativeMappingFunctionDefs(
    mut inFuncs: metamodelica::List<DAE::FunctionDefinition>,
    mut path: metamodelica::Ref<Absyn::Path>,
) -> metamodelica::List<DAE::FunctionDefinition> {
    let mut outFuncs: metamodelica::List<DAE::FunctionDefinition>;
    outFuncs = ({
        let mut __acc: metamodelica::List<DAE::FunctionDefinition> = metamodelica::nil();
        for mut r#fn in (inFuncs).into_iter().cloned() {
            let __x = (match r#fn.clone() {
                DAE::FunctionDefinition::FUNCTION_DER_MAPPER { .. } => {
                    let __owned_variant_lowerOrderDerivatives_0 = metamodelica::cons(
                        path.clone(),
                        var_field!(r#fn.lowerOrderDerivatives, DAE::FunctionDefinition::FUNCTION_DER_MAPPER).clone(),
                    );
                    if let DAE::FunctionDefinition::FUNCTION_DER_MAPPER {
                        lowerOrderDerivatives, ..
                    } = &mut r#fn
                    {
                        *lowerOrderDerivatives = __owned_variant_lowerOrderDerivatives_0;
                    } else {
                        panic!(
                            "owned-variant field-assign: value held a different variant than DAE::FunctionDefinition::FUNCTION_DER_MAPPER"
                        );
                    }
                    r#fn.clone()
                }
                _ => r#fn.clone(),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outFuncs
}

pub(crate) fn getDeriveAnnotation(
    mut cd: &metamodelica::Ref<SCode::ClassDef>,
    mut cmt: &metamodelica::Ref<SCode::Comment>,
    mut baseFunc: &metamodelica::Ref<Absyn::Path>,
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: &DAE::Prefix,
    mut info: &SourceInfo,
) -> metamodelica::List<DAE::FunctionDefinition> {
    let mut element: metamodelica::List<DAE::FunctionDefinition>;
    element = 'mc: {
        let __mc_input = (&**cd, &**cmt);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PARTS { elementLst: elemDecl, externalDecl: Some(Deref @ SCode::ExternalDecl { annotation_: Some(ann), .. }), .. }, _) => {
                    Ok(getDeriveAnnotation2(metamodelica::AsArg::as_arg(&ann), metamodelica::AsArg::as_arg(&elemDecl), baseFunc, inCache, inEnv, inIH, inPrefix, info)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PARTS { elementLst: elemDecl, .. }, Deref @ SCode::Comment { annotation_: Some(ann), .. }) => {
                    Ok(getDeriveAnnotation2(metamodelica::AsArg::as_arg(&ann), metamodelica::AsArg::as_arg(&elemDecl), baseFunc, inCache, inEnv, inIH, inPrefix, info)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    element
}

fn getDeriveAnnotation2(
    mut ann: &metamodelica::Ref<SCode::Annotation>,
    mut elemDecl: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut baseFunc: &metamodelica::Ref<Absyn::Path>,
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: &DAE::Prefix,
    mut info: &SourceInfo,
) -> Result<metamodelica::List<DAE::FunctionDefinition>> {
    let mut element: metamodelica::List<DAE::FunctionDefinition>;
    element = (::match_deref::match_deref! { match ann {
        Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: smlst, .. } } => {
            getDeriveAnnotation3(metamodelica::AsArg::as_arg(&smlst), elemDecl, baseFunc, inCache, inEnv, inIH, inPrefix, info)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(element)
}

fn getDeriveAnnotation3(
    mut inSubs: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut elemDecl: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut baseFunc: &metamodelica::Ref<Absyn::Path>,
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: &DAE::Prefix,
    mut info: &SourceInfo,
) -> Result<metamodelica::List<DAE::FunctionDefinition>> {
    let mut element: metamodelica::List<DAE::FunctionDefinition>;
    element = 'mc: {
        let __mc_input = &**inSubs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "derivative", r#mod: Deref @ SCode::Mod::MOD { subModLst: subs2, binding: Some(Deref @ Absyn::Exp::CREF { componentRef: acr }), .. } }, tail: subs } => {
                    let mut deriveFunc: metamodelica::Ref<Absyn::Path>;
                    let mut defaultDerivative: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut order: i32;
                    let mut conditionRefs: metamodelica::List<(i32, DAE::derivativeCond)>;
                    let mut mapper: DAE::FunctionDefinition;
                    deriveFunc = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&acr))?;
                    (_, deriveFunc) = Inst::makeFullyQualified(inCache.clone(), inEnv.clone(), deriveFunc.clone())?;
                    order = getDerivativeOrder(metamodelica::AsArg::as_arg(&subs2));
                    ErrorExt::setCheckpoint(literal!("getDeriveAnnotation3"));
                    conditionRefs = getDeriveCondition(metamodelica::AsArg::as_arg(&subs2), elemDecl, inCache, inEnv, inIH, inPrefix, info);
                    ErrorExt::rollBack(literal!("getDeriveAnnotation3"));
                    conditionRefs = List::sort(conditionRefs.clone(), (std::sync::Arc::new(move |__a0: (i32, DAE::derivativeCond), __a1: (i32, DAE::derivativeCond)| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::derivativeOrder(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn((i32, DAE::derivativeCond), (i32, DAE::derivativeCond)) -> Result<bool> + 'static>))?;
                    defaultDerivative = getDerivativeSubModsOptDefault(metamodelica::AsArg::as_arg(&subs), inCache, inEnv, inPrefix);
                    mapper = DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivedFunction: baseFunc.clone(), derivativeFunction: deriveFunc.clone(), derivativeOrder: order, conditionRefs: conditionRefs.clone(), defaultDerivative: defaultDerivative.clone(), lowerOrderDerivatives: metamodelica::nil() };
                    Ok(list![mapper.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: subs } => {
                    Ok(getDeriveAnnotation3(metamodelica::AsArg::as_arg(&subs), elemDecl, baseFunc, inCache, inEnv, inIH, inPrefix, info)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(element)
}

fn getDeriveCondition(
    mut inSubs: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut elemDecl: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: &DAE::Prefix,
    mut info: &SourceInfo,
) -> metamodelica::List<(i32, DAE::derivativeCond)> {
    let mut outconds: metamodelica::List<(i32, DAE::derivativeCond)> = metamodelica::nil();
    outconds = 'mc: {
        let __mc_input = &**inSubs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "noDerivative", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: acr }), .. } }, tail: subs } => {
                    let mut name: ArcStr;
                    let mut varPos: i32;
                    let mut outconds: metamodelica::List<(i32, DAE::derivativeCond)> = outconds.clone();
                    name = Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&acr))?;
                    outconds = getDeriveCondition(metamodelica::AsArg::as_arg(&subs), elemDecl, inCache, inEnv, inIH, inPrefix, info);
                    varPos = setFunctionInputIndex(elemDecl, &name, 1)?;
                    Ok((metamodelica::cons((varPos, DAE::derivativeCond::NO_DERIVATIVE { binding: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 99 }) }), outconds.clone()), outconds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outconds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "zeroDerivative", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: acr }), .. } }, tail: subs } => {
                    let mut name: ArcStr;
                    let mut varPos: i32;
                    let mut outconds: metamodelica::List<(i32, DAE::derivativeCond)> = outconds.clone();
                    name = Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&acr))?;
                    outconds = getDeriveCondition(metamodelica::AsArg::as_arg(&subs), elemDecl, inCache, inEnv, inIH, inPrefix, info);
                    varPos = setFunctionInputIndex(elemDecl, &name, 1)?;
                    Ok((metamodelica::cons((varPos, openmodelica_frontend_types::DAE::derivativeCond::ZERO_DERIVATIVE), outconds.clone()), outconds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outconds = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "noDerivative", r#mod: m @ Deref @ SCode::Mod::MOD { .. } }, tail: subs } => {
                    let mut sub: metamodelica::Ref<DAE::SubMod>;
                    let mut name: ArcStr;
                    let mut cond: DAE::derivativeCond;
                    let mut varPos: i32;
                    let mut cache: FCore::Cache;
                    let mut outconds: metamodelica::List<(i32, DAE::derivativeCond)> = outconds.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Mod::elabMod(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), m.clone(), false, Mod::ModScope::COMPONENT { name: literal!("noDerivative") }, info.clone())?) {
                        (__pa0, Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    sub = metamodelica::Own::own(__pa1);
                    (name, cond) = extractNameAndExp(&sub);
                    outconds = getDeriveCondition(metamodelica::AsArg::as_arg(&subs), elemDecl, &cache, inEnv, inIH, inPrefix, info);
                    varPos = setFunctionInputIndex(elemDecl, &name, 1)?;
                    Ok((metamodelica::cons((varPos, cond.clone()), outconds.clone()), outconds.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outconds = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: subs } => {
                    Ok(getDeriveCondition(metamodelica::AsArg::as_arg(&subs), elemDecl, inCache, inEnv, inIH, inPrefix, info))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outconds
}

fn setFunctionInputIndex(
    mut inElemDecl: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut r#str: &ArcStr,
    mut currPos: i32,
) -> Result<i32> {
    let mut index: i32;
    index = 'mc: {
        let __mc_input = &**inElemDecl;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" failure in setFunctionInputIndex, didn't find any index for: ")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::COMPONENT { name: str2, attributes: SCode::Attributes { direction: Absyn::Direction::INPUT { .. }, .. }, .. }, tail: _ } => {
                    let true = (stringEq(&str2, &r#str)) else { return Err("pattern mismatch") };
                    Ok(currPos)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::INPUT { .. }, .. }, .. }, tail: elemDecl } => {
                    Ok(setFunctionInputIndex(metamodelica::AsArg::as_arg(&elemDecl), r#str, currPos + 1)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: elemDecl } => {
                    Ok(setFunctionInputIndex(metamodelica::AsArg::as_arg(&elemDecl), r#str, currPos)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(index)
}

fn extractNameAndExp(mut m: &metamodelica::Ref<DAE::SubMod>) -> (ArcStr, DAE::derivativeCond) {
    let mut inputVar: ArcStr;
    let mut cond: DAE::derivativeCond;
    (inputVar, cond) = (::match_deref::match_deref! { match m {
        Deref @ DAE::SubMod { ident: __esc_inputVar, r#mod: Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: e, .. }), .. } } => {
            inputVar = (*__esc_inputVar).clone();
            (inputVar.clone(), DAE::derivativeCond::NO_DERIVATIVE { binding: e.clone() })
        },
        Deref @ DAE::SubMod { ident: __esc_inputVar, r#mod: Deref @ DAE::Mod::MOD { binding: None, .. } } => {
            inputVar = (*__esc_inputVar).clone();
            (inputVar.clone(), DAE::derivativeCond::NO_DERIVATIVE { binding: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }) })
        },
        Deref @ DAE::SubMod { ident: __esc_inputVar, r#mod: Deref @ DAE::Mod::MOD { binding: None, .. } } => {
            inputVar = (*__esc_inputVar).clone();
            (inputVar.clone(), openmodelica_frontend_types::DAE::derivativeCond::ZERO_DERIVATIVE)
        },
        _ => {
            (literal!(""), openmodelica_frontend_types::DAE::derivativeCond::ZERO_DERIVATIVE)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (inputVar, cond)
}

fn getDerivativeSubModsOptDefault(
    mut inSubs: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inPrefix: &DAE::Prefix,
) -> Option<metamodelica::Ref<Absyn::Path>> {
    let mut defaultDerivative: Option<metamodelica::Ref<Absyn::Path>>;
    defaultDerivative = 'mc: {
        let __mc_input = &**inSubs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(None)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "derivative", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: acr }), .. } }, tail: _ } => {
                    let mut p: metamodelica::Ref<Absyn::Path>;
                    p = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&acr))?;
                    (_, p) = Inst::makeFullyQualified(inCache.clone(), inEnv.clone(), p.clone())?;
                    Ok(Some(p.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: subs } => {
                    Ok(getDerivativeSubModsOptDefault(metamodelica::AsArg::as_arg(&subs), inCache, inEnv, inPrefix))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    defaultDerivative
}

fn getDerivativeOrder<'__b>(mut inSubs: &'__b metamodelica::List<metamodelica::Ref<SCode::SubMod>>) -> i32 {
    let mut order: i32;
    order = (::match_deref::match_deref! { match inSubs {
        Deref @ metamodelica::ListNode::Nil => {
            1
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "order", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::INTEGER { value: __esc_order }), .. } }, tail: _ } => {
            order = (*__esc_order).clone();
            order.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: subs } => {
            getDerivativeOrder(subs)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    order
}

pub(crate) fn setFullyQualifiedTypename(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut path: metamodelica::Ref<Absyn::Path>,
) -> metamodelica::Ref<DAE::Type> {
    let mut resType: metamodelica::Ref<DAE::Type>;
    resType = (::match_deref::match_deref! { match &(inType.clone()) {
        __esc_resType @ Deref @ DAE::Type::T_FUNCTION { .. } => {
            resType = (*__esc_resType).clone();
            assign_variant_field!(resType => DAE::Type::T_FUNCTION; path = path);
            resType.clone()
        },
        _ => inType,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    resType
}

pub(crate) fn classIsInlineFunc(mut elt: &metamodelica::Ref<SCode::Element>) -> DAE::InlineType {
    let mut outInlineType: DAE::InlineType;
    outInlineType = (match &**elt {
        SCode::Element::CLASS { cmt: __elt_cmt, .. } => {
            InstBasics::commentIsInlineFunc(metamodelica::AsArg::as_arg(&__elt_cmt))
        }
        _ => openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE,
    });
    outInlineType
}

pub(crate) fn stripFuncOutputsMod(mut elem: metamodelica::Ref<SCode::Element>) -> metamodelica::Ref<SCode::Element> {
    let mut stripped_elem: metamodelica::Ref<SCode::Element>;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    stripped_elem = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::OUTPUT { .. }, .. }, modifications: __esc_mod @ Deref @ SCode::Mod::MOD { binding: Some(_), .. }, .. } => {
            r#mod = (*__esc_mod).clone();
            assign_variant_field!(r#mod => SCode::Mod::MOD; binding = None);
            assign_variant_field!(elem => SCode::Element::COMPONENT; modifications = r#mod.clone());
            elem
        },
        _ => elem,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    stripped_elem
}

pub(crate) fn checkExternalFunction(
    mut els: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut decl: DAE::ExternalDecl,
    mut name: ArcStr,
) -> Result<()> {
    if metamodelica::stringEq(&decl.language, &(literal!("builtin"))) {
        return Ok(());
    }
    List::map2_0(
        els,
        &move |__a0: metamodelica::Ref<DAE::Element>, __a1: DAE::ExternalDecl, __a2: ArcStr| {
            checkExternalFunctionOutputAssigned(__a0, &__a1, __a2)
        },
        decl.clone(),
        name.clone(),
    )?;
    checkFunctionInputUsed(els, Some(decl), name)?;
    Ok(())
}

pub(crate) fn checkFunctionInputUsed(
    mut elts: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut decl: Option<DAE::ExternalDecl>,
    mut name: ArcStr,
) -> Result<()> {
    let mut invars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut algs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    (vars, _, _, _, algs, _, _, _, _, _) = DAEUtil::splitElements(elts)?;
    invars = List::filterOnTrue(
        vars.clone(),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(DAEUtil::isInputVar(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
    )?;
    invars = List::select(
        invars,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(checkInputUsedAnnotation(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
    )?;
    invars = checkExternalDeclInputUsed(invars, decl)?;
    invars = List::select1(
        invars,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::List<metamodelica::Ref<DAE::Element>>| {
                checkVarBindingsInputUsed(__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Element>,
                        metamodelica::List<metamodelica::Ref<DAE::Element>>,
                    ) -> Result<bool>
                    + 'static,
            >),
        vars,
    )?;
    let (_, (_, __pa0)) = DAEUtil::traverseDAEElementList(
        algs,
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                checkExpInputUsed,
                metamodelica::Ref<DAE::Exp>,
                metamodelica::List<metamodelica::Ref<DAE::Element>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            metamodelica::List<metamodelica::Ref<DAE::Element>>,
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            metamodelica::List<metamodelica::Ref<DAE::Element>>,
                        )> + 'static,
                >),
            invars,
        ),
    )?;
    invars = metamodelica::Own::own(__pa0);
    List::map1_0(
        &invars,
        &move |__a0: metamodelica::Ref<DAE::Element>, __a1: ArcStr| warnUnusedFunctionVar(&__a0, __a1),
        name,
    )?;
    Ok(())
}

pub(crate) fn checkInputUsedAnnotation(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut result: bool;
    result = (match &**inElement {
        DAE::Element::VAR { comment: cmt, .. } => {
            result = SCodeUtil::optCommentHasBooleanNamedAnnotation(
                cmt.clone(),
                &(literal!("__OpenModelica_UnusedVariable")),
            );
            !(result)
        }
        _ => true,
    });
    result
}

fn warnUnusedFunctionVar(mut v: &metamodelica::Ref<DAE::Element>, mut name: ArcStr) -> Result<()> {
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut r#str: ArcStr;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*v)) {
        Deref @ DAE::Element::VAR { componentRef: __pa0, source: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cr = metamodelica::Own::own(__pa0);
    source = metamodelica::Own::own(__pa1);
    r#str = ComponentReferenceBasics::printComponentRefStr(&cr)?;
    Error::addSourceMessage(
        &(Error::FUNCTION_UNUSED_INPUT.clone()),
        list![r#str, name],
        &(ElementSource::getElementSourceFileInfo(source)),
    )?;
    Ok(())
}

fn checkExternalDeclInputUsed(
    mut inames: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut decl: Option<DAE::ExternalDecl>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut onames: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    onames = (::match_deref::match_deref! { match &((inames, decl)) {
        (names, None) => {
            names.clone()
        },
        (Deref @ metamodelica::ListNode::Nil, _) => {
            metamodelica::nil()
        },
        (names, Some(DAE::ExternalDecl { returnArg: arg, args, .. })) => {
            let mut names = (*names).clone();
            names = List::select1(names.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::List<DAE::ExtArg>| checkExternalDeclArgs(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>, metamodelica::List<DAE::ExtArg>) -> Result<bool> + 'static>), metamodelica::cons(arg.clone(), args.clone()))?;
            names.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(onames)
}

fn checkExpInputUsed(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inEls: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
) {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut els: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    (exp, els) = 'mc: {
        let __mc_input = (inExp.clone(), inEls.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, els) => {
                    let mut els = (*els).clone();
                    els = List::select1(els.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::ComponentRef>| checkExpInputUsed3(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>), cr.clone())?;
                    Ok((exp.clone(), els.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CALL { path, .. }, els) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut els = (*els).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    cr = ComponentReference::pathToCref(metamodelica::AsArg::as_arg(&path));
                    els = List::select1(els.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::ComponentRef>| checkExpInputUsed3(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>), cr.clone())?;
                    Ok((exp.clone(), els.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inEls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (exp, els)
}

fn checkExpInputUsed3(
    mut el: &metamodelica::Ref<DAE::Element>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut noteq: bool;
    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &((*el)) {
        Deref @ DAE::Element::VAR { componentRef: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cr1 = metamodelica::Own::own(__pa0);
    noteq = !(ComponentReferenceBasics::crefEqualNoStringCompare(&cr1, cr2)?);
    Ok(noteq)
}

fn checkVarBindingsInputUsed(
    mut v: metamodelica::Ref<DAE::Element>,
    mut els: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<bool> {
    let mut notfound: bool;
    notfound = !(List::isMemberOnTrue(
        v,
        els,
        &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::Element>| {
            checkVarBindingInputUsed(&__a0, &__a1)
        },
    )?);
    Ok(notfound)
}

fn checkVarBindingInputUsed(
    mut v: &metamodelica::Ref<DAE::Element>,
    mut el: &metamodelica::Ref<DAE::Element>,
) -> Result<bool> {
    let mut found: bool;
    found = (::match_deref::match_deref! { match (v, el) {
        (Deref @ DAE::Element::VAR { .. }, Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT { .. }, .. }) => {
            false
        },
        (Deref @ DAE::Element::VAR { componentRef: cr, .. }, Deref @ DAE::Element::VAR { binding: Some(exp), .. }) => {
            Expression::expHasCref(exp.clone(), cr.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(found)
}

fn checkExternalDeclArgs(
    mut v: metamodelica::Ref<DAE::Element>,
    mut args: &metamodelica::List<DAE::ExtArg>,
) -> Result<bool> {
    let mut notfound: bool;
    notfound = !(List::isMemberOnTrue(
        v,
        args,
        &move |__a0: metamodelica::Ref<DAE::Element>, __a1: DAE::ExtArg| extArgCrefEq(&__a0, &__a1),
    )?);
    Ok(notfound)
}

fn checkExternalFunctionOutputAssigned(
    mut v: metamodelica::Ref<DAE::Element>,
    mut decl: &DAE::ExternalDecl,
    mut name: ArcStr,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((v.clone(), decl.clone())) {
        (Deref @ DAE::Element::VAR { direction: DAE::VarDirection::OUTPUT { .. }, componentRef: cr, binding, source, .. }, DAE::ExternalDecl { returnArg: arg, args, .. }) => {
            let mut r#str: ArcStr;
            if !(List::isMemberOnTrue(v, &(metamodelica::cons(arg.clone(), args.clone())), &move |__a0: metamodelica::Ref<DAE::Element>, __a1: DAE::ExtArg| extArgCrefEq(&__a0, &__a1))? || (binding).is_some()) {
                r#str = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
                Error::addSourceMessage(&(Error::EXTERNAL_NOT_SINGLE_RESULT.clone()), list![r#str, name], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                return Err("fail");
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn extArgCrefEq(mut v: &metamodelica::Ref<DAE::Element>, mut arg: &DAE::ExtArg) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((&**v, arg)) {
        (Deref @ DAE::Element::VAR { componentRef: cr1, .. }, DAE::ExtArg::EXTARG { componentRef: cr2, .. }) => {
            let mut cr2 = (*cr2).clone();
            cr2 = ComponentReferenceBasics::crefFirstCref(cr2.clone())?;
            ComponentReferenceBasics::crefEqualNoStringCompare(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2))?
        },
        (Deref @ DAE::Element::VAR { direction: DAE::VarDirection::OUTPUT { .. }, .. }, _) => {
            false
        },
        (Deref @ DAE::Element::VAR { componentRef: cr1, .. }, DAE::ExtArg::EXTARGSIZE { componentRef: cr2, .. }) => {
            let mut cr2 = (*cr2).clone();
            cr2 = ComponentReferenceBasics::crefFirstCref(cr2.clone())?;
            ComponentReferenceBasics::crefEqualNoStringCompare(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2))?
        },
        (Deref @ DAE::Element::VAR { componentRef: cr1, .. }, DAE::ExtArg::EXTARGEXP { exp, .. }) => {
            Expression::expHasCref(exp.clone(), cr1.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isExtExplicitCall(mut inExternalDecl: &metamodelica::Ref<SCode::ExternalDecl>) -> bool {
    let mut isExplicit: bool;
    isExplicit = (match &**inExternalDecl {
        SCode::ExternalDecl { funcName: Some(_), .. } => true,
        _ => false,
    });
    isExplicit
}

fn isInoutVar(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut b: bool;
    b = isOutputVar(inElement) || isInputVar(inElement);
    b
}

fn isOutputVar(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut b: bool;
    b = (match &**inElement {
        SCode::Element::COMPONENT {
            attributes:
                SCode::Attributes {
                    direction: Absyn::Direction::OUTPUT { .. },
                    ..
                },
            ..
        } => true,
        _ => false,
    });
    b
}

fn isInputVar(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut b: bool;
    b = (match &**inElement {
        SCode::Element::COMPONENT {
            attributes:
                SCode::Attributes {
                    direction: Absyn::Direction::INPUT { .. },
                    ..
                },
            ..
        } => true,
        _ => false,
    });
    b
}

pub(crate) fn instExtGetFname(
    mut inExternalDecl: &metamodelica::Ref<SCode::ExternalDecl>,
    mut inIdent: ArcStr,
) -> Result<ArcStr> {
    let mut outIdent: ArcStr;
    outIdent = (match &**inExternalDecl {
        SCode::ExternalDecl { funcName: Some(id), .. } => id.clone(),
        SCode::ExternalDecl { funcName: None, .. } => {
            let mut fid = inIdent;
            fid
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outIdent)
}

pub(crate) fn instExtGetAnnotation(
    mut inExternalDecl: &metamodelica::Ref<SCode::ExternalDecl>,
) -> Option<metamodelica::Ref<SCode::Annotation>> {
    let mut outAnnotation: Option<metamodelica::Ref<SCode::Annotation>>;
    outAnnotation = (match &**inExternalDecl {
        SCode::ExternalDecl { annotation_: ann, .. } => ann.clone(),
    });
    outAnnotation
}

pub(crate) fn instExtGetLang(mut inExternalDecl: &metamodelica::Ref<SCode::ExternalDecl>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inExternalDecl {
        SCode::ExternalDecl { lang: Some(lang), .. } => lang.clone(),
        SCode::ExternalDecl { lang: None, .. } => {
            literal!("C")
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn elabExpListExt(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynExpLst: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: &SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<DAE::Properties>,
)> {
    let mut outCache: FCore::Cache;
    let mut outExpExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outTypesPropertiesLst: metamodelica::List<DAE::Properties>;
    (outCache, outExpExpLst, outTypesPropertiesLst) = (::match_deref::match_deref! { match inAbsynExpLst {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache;
            (cache, metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: rest } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut p: DAE::Properties;
            let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut props: metamodelica::List<DAE::Properties>;
            (cache, exp, p) = elabExpExt(cache, env.clone(), e.clone(), r#impl, pre.clone(), info.clone())?;
            (cache, exps, props) = elabExpListExt(cache, env, rest, r#impl, pre, info)?;
            (cache, metamodelica::cons(exp, exps), metamodelica::cons(p, props))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outExpExpLst, outTypesPropertiesLst))
}

fn elabExpExt(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache, inEnv, inExp, inBoolean, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "size", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: arraycr, tail: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. }, r#impl, pre) => {
                    let mut dimp: metamodelica::Ref<DAE::Exp>;
                    let mut arraycrefe: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut arraycrprop: DAE::Properties;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Static::elabExp(cache.clone(), env.clone(), dim.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, __pa2 @ DAE::Properties::PROP { type_: _, constFlag: _ }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    dimp = metamodelica::Own::own(__pa1);
                    prop = metamodelica::Own::own(__pa2);
                    (cache, dimp, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), dimp.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    (cache, arraycrefe, arraycrprop) = Static::elabExp(cache.clone(), env.clone(), arraycr.clone(), r#impl.clone(), false, pre.clone(), info.clone())?;
                    (cache, arraycrefe, arraycrprop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), arraycrefe.clone(), arraycrprop.clone(), r#impl.clone(), info.clone())?;
                    exp = metamodelica::Ref::new(DAE::Exp::SIZE { exp: arraycrefe.clone(), sz: Some(dimp.clone()) });
                    Ok((cache.clone(), exp.clone(), DAE::Properties::PROP { type_: DAE::T_INTEGER_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, absynExp, r#impl, pre) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e, prop) = Static::elabExp(cache.clone(), env.clone(), absynExp.clone(), r#impl.clone(), false, pre.clone(), info.clone())?;
                    (cache, e, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), e.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    Ok((cache.clone(), e.clone(), prop.clone()))
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
                    Debug::traceln(literal!("-Inst.elabExpExt failed"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outProperties))
}

pub(crate) fn instExtGetFargs(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExternalDecl: &metamodelica::Ref<SCode::ExternalDecl>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::List<DAE::ExtArg>)> {
    let mut outCache: FCore::Cache;
    let mut outDAEExtArgLst: metamodelica::List<DAE::ExtArg>;
    (outCache, outDAEExtArgLst) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inExternalDecl, inBoolean, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ SCode::ExternalDecl { lang, args: absexps, .. }, r#impl, pre) => {
                    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut props: metamodelica::List<DAE::Properties>;
                    let mut extargs: metamodelica::List<DAE::ExtArg>;
                    let mut cache = (*cache).clone();
                    (cache, exps, props) = elabExpListExt(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&absexps), r#impl.clone(), pre.clone(), info)?;
                    (cache, extargs) = instExtGetFargs2(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&absexps), &exps, &props, lang.clone(), info)?;
                    Ok((cache.clone(), extargs.clone()))
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
                    Debug::traceln(literal!("- InstUtil.instExtGetFargs failed"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outDAEExtArgLst))
}

fn instExtGetFargs2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut absynExps: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inTypesPropertiesLst: &metamodelica::List<DAE::Properties>,
    mut lang: Option<ArcStr>,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::List<DAE::ExtArg>)> {
    let mut outCache: FCore::Cache;
    let mut outDAEExtArgLst: metamodelica::List<DAE::ExtArg>;
    (outCache, outDAEExtArgLst) = (::match_deref::match_deref! { match (absynExps, inExpExpLst, inTypesPropertiesLst) {
        (_, Deref @ metamodelica::ListNode::Nil, _) => {
            let mut cache = inCache;
            (cache, metamodelica::nil())
        },
        (Deref @ metamodelica::ListNode::Cons { head: ae, tail: aes }, Deref @ metamodelica::ListNode::Cons { head: e, tail: exps }, Deref @ metamodelica::ListNode::Cons { head: p, tail: props }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut extargs: metamodelica::List<DAE::ExtArg>;
            let mut extarg: DAE::ExtArg;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(instExtGetFargsSingle(cache, env.clone(), metamodelica::AsArg::as_arg(&ae), e.clone(), p.clone(), lang.clone(), info.clone())?) {
                (__pa0, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            extarg = metamodelica::Own::own(__pa1);
            (cache, extargs) = instExtGetFargs2(cache, env, aes, exps, props, lang, info)?;
            (cache, metamodelica::cons(extarg, extargs))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outDAEExtArgLst))
}

fn instExtGetFargsSingle(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut absynExp: &metamodelica::Ref<Absyn::Exp>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inProperties: DAE::Properties,
    mut lang: Option<ArcStr>,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, Option<DAE::ExtArg>)> {
    let mut outCache: FCore::Cache;
    let mut outExtArg: Option<DAE::ExtArg>;
    (outCache, outExtArg) = 'mc: {
        let __mc_input = (
            inCache.clone(),
            inEnv.clone(),
            &**absynExp,
            inExp.clone(),
            &inProperties,
            lang,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Exp::CREF { componentRef: cref @ Deref @ DAE::ComponentRef::CREF_QUAL { .. }, .. }, DAE::Properties::PROP { constFlag: DAE::Const::C_VAR { .. }, .. }, _) => {
                    let mut fattr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut fcr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cache: FCore::Cache;
                    (cache, _, ty, _, _, _, _, _, _) = Lookup::lookupVarLocal(inCache.clone(), &inEnv, cref.clone())?;
                    fcr = ComponentReferenceBasics::crefFirstCref(cref.clone())?;
                    (cache, fattr, _, _, _, _, _, _, _) = Lookup::lookupVarLocal(cache.clone(), &inEnv, fcr.clone())?;
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARG { componentRef: cref.clone(), direction: DAEUtil::getAttrDirection(&fattr), type_: ty.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Exp::CREF { componentRef: cref @ Deref @ DAE::ComponentRef::CREF_IDENT { .. }, .. }, DAE::Properties::PROP { constFlag: DAE::Const::C_VAR { .. }, .. }, _) => {
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut cache: FCore::Cache;
                    (cache, attr, ty, _, _, _, _, _, _) = Lookup::lookupVarLocal(inCache.clone(), &inEnv, cref.clone())?;
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARG { componentRef: cref.clone(), direction: DAEUtil::getAttrDirection(&attr), type_: ty.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, Deref @ DAE::Exp::CREF { componentRef: cref, .. }, DAE::Properties::PROP { .. }, _) => {
                    let mut crefstr: ArcStr;
                    let mut scope: ArcStr;
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupVarLocal(cache.clone(), metamodelica::AsArg::as_arg(&env), cref.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    crefstr = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cref))?;
                    scope = FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env));
                    Error::addMessage(Error::LOOKUP_VARIABLE_ERROR.clone(), list![crefstr.clone(), scope.clone()])?;
                    Ok((cache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, Deref @ DAE::Exp::SIZE { exp: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, sz: Some(dim) }, DAE::Properties::PROP { .. }, _) => {
                    let mut varty: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    (cache, _, varty, _, _, _, _, _, _) = Lookup::lookupVarLocal(cache.clone(), metamodelica::AsArg::as_arg(&env), cref.clone())?;
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGSIZE { componentRef: cref.clone(), type_: varty.clone(), exp: dim.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, DAE::Properties::PROP { type_: ty, constFlag: DAE::Const::C_CONST { .. } }, _) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, exp, _) = Ceval::cevalIfConstant(cache.clone(), env.clone(), inExp.clone(), inProperties.clone(), false, info.clone())?;
                    let true = (Expression::isScalarConst(&exp)) else { return Err("pattern mismatch") };
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGEXP { exp: exp.clone(), type_: ty.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, DAE::Properties::PROP { type_: ty, .. }, Some(Deref @ "builtin")) => {
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGEXP { exp: inExp.clone(), type_: ty.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, DAE::Properties::PROP { type_: ty, .. }, _) => {
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGEXP { exp: inExp.clone(), type_: ty.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Exp::CREF { componentRef: _ }, _, DAE::Properties::PROP { type_: ty, .. }, _) => {
                    Ok((cache.clone(), Some(DAE::ExtArg::EXTARGEXP { exp: inExp.clone(), type_: ty.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, exp, DAE::Properties::PROP { .. }, _) => {
                    let mut r#str: ArcStr;
                    r#str = ExpressionBasics::printExpStr(exp.clone())?;
                    Error::addSourceMessage(&(Error::EXTERNAL_ARG_WRONG_EXP.clone()), list![r#str.clone()], &info)?;
                    Ok((cache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExtArg))
}

pub(crate) fn instExtGetRettype(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExternalDecl: &metamodelica::Ref<SCode::ExternalDecl>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, DAE::ExtArg)> {
    let mut outCache: FCore::Cache;
    let mut outExtArg: DAE::ExtArg;
    (outCache, outExtArg) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inExternalDecl, inBoolean, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ SCode::ExternalDecl { output_: None, .. }, _, _) => {
                    Ok((cache.clone(), openmodelica_frontend_types::DAE::ExtArg::NOEXTARG))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ SCode::ExternalDecl { lang, output_: Some(cref), .. }, r#impl, pre) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut extarg: DAE::ExtArg;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cref.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa1, __pa2, _))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    exp = metamodelica::Own::own(__pa1);
                    prop = metamodelica::Own::own(__pa2);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(instExtGetFargsSingle(cache.clone(), env.clone(), &(metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cref.clone() })), exp.clone(), prop.clone(), lang.clone(), info.clone())?) {
                        (__pa3, Some(__pa4)) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    extarg = metamodelica::Own::own(__pa4);
                    assertExtArgOutputIsCrefVariable(lang.clone(), &extarg, Types::getPropType(&prop), Types::propAllConst(prop.clone())?, &info)?;
                    Ok((cache.clone(), extarg.clone()))
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
                    Debug::traceln(literal!("- InstUtil.instExtRettype failed"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExtArg))
}

fn assertExtArgOutputIsCrefVariable(
    mut lang: Option<ArcStr>,
    mut arg: &DAE::ExtArg,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut c: DAE::Const,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((lang, arg.clone(), ty.clone(), c)) {
        (Some(Deref @ "builtin"), _, _, _) => {
            ()
        },
        (_, _, Deref @ DAE::Type::T_ARRAY { .. }, _) => {
            let mut r#str: ArcStr;
            r#str = TypesDump::unparseType(ty)?;
            Error::addSourceMessage(&(Error::EXTERNAL_FUNCTION_RESULT_ARRAY_TYPE.clone()), list![r#str], info)?;
            return Err("fail")
        },
        (_, DAE::ExtArg::EXTARG { .. }, _, DAE::Const::C_VAR { .. }) => {
            ()
        },
        (_, _, _, DAE::Const::C_VAR { .. }) => {
            let mut r#str: ArcStr;
            r#str = DAEDump::dumpExtArgStr(arg)?;
            Error::addSourceMessage(&(Error::EXTERNAL_FUNCTION_RESULT_NOT_CREF.clone()), list![r#str], info)?;
            return Err("fail")
        },
        _ => {
            Error::addSourceMessage(&(Error::EXTERNAL_FUNCTION_RESULT_NOT_VAR.clone()), metamodelica::nil(), info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn makeDaeProt(mut visibility: SCode::Visibility) -> DAE::VarVisibility {
    let mut res: DAE::VarVisibility;
    res = (match visibility {
        SCode::Visibility::PROTECTED { .. } => openmodelica_frontend_types::DAE::VarVisibility::PROTECTED,
        SCode::Visibility::PUBLIC { .. } => openmodelica_frontend_types::DAE::VarVisibility::PUBLIC,
    });
    res
}

pub(crate) fn makeDaeVariability(mut inVariability: SCode::Variability) -> DAE::VarKind {
    let mut outVariability: DAE::VarKind;
    outVariability = (match inVariability {
        SCode::Variability::VAR { .. } => openmodelica_frontend_types::DAE::VarKind::VARIABLE,
        SCode::Variability::PARAM { .. } => openmodelica_frontend_types::DAE::VarKind::PARAM,
        SCode::Variability::CONST { .. } => openmodelica_frontend_types::DAE::VarKind::CONST,
        SCode::Variability::DISCRETE { .. } => openmodelica_frontend_types::DAE::VarKind::DISCRETE,
    });
    outVariability
}

pub(crate) fn makeDaeDirection(mut inDirection: Absyn::Direction) -> Result<DAE::VarDirection> {
    let mut outDirection: DAE::VarDirection;
    outDirection = (match inDirection {
        Absyn::Direction::INPUT { .. } => openmodelica_frontend_types::DAE::VarDirection::INPUT,
        Absyn::Direction::OUTPUT { .. } => openmodelica_frontend_types::DAE::VarDirection::OUTPUT,
        Absyn::Direction::BIDIR { .. } => openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        _ => return Err("match: no arm matched"),
    });
    Ok(outDirection)
}

pub(crate) fn mktype(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inState: ClassInf::State,
    mut inTypesVarLst: metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inTypesTypeOption: Option<metamodelica::Ref<DAE::Type>>,
    mut inEqualityConstraint: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inheritedComment: &metamodelica::Ref<SCode::Comment>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (
            inPath.clone(),
            inState.clone(),
            inTypesVarLst.clone(),
            &inTypesTypeOption,
            inEqualityConstraint.clone(),
            inClass.clone(),
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_INTEGER { .. }, v, _, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_INTEGER { varLst: v.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_REAL { .. }, v, _, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_REAL { varLst: v.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_STRING { .. }, v, _, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_STRING { varLst: v.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_BOOL { .. }, v, _, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_BOOL { varLst: v.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_CLOCK { .. }, v, _, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_CLOCK { varLst: v.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::TYPE_ENUM { .. }, _, _, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: None, path: p.clone(), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::FUNCTION { .. }, vl, _, _, cl) => {
                    let mut functype: metamodelica::Ref<DAE::Type>;
                    let mut funcattr: DAE::FunctionAttributes;
                    funcattr = getFunctionAttributes(metamodelica::AsArg::as_arg(&cl), vl.clone(), inheritedComment)?;
                    functype = Types::makeFunctionType(p.clone(), vl.clone(), funcattr.clone())?;
                    Ok(functype.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::ENUMERATION { path: p }, _, Some(enumtype), _, _) => {
                    let mut enumtype = (*enumtype).clone();
                    enumtype = Types::makeEnumerationType(metamodelica::AsArg::as_arg(&p), metamodelica::AsArg::as_arg(&enumtype))?;
                    Ok(enumtype.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE { .. }, _, Some(Deref @ DAE::Type::T_ARRAY { ty: arrayType, .. }), None, _) => {
                    let mut resType: metamodelica::Ref<DAE::Type>;
                    let mut classState: ClassInf::State;
                    classState = arrayTTypeToClassInfState(metamodelica::AsArg::as_arg(&arrayType))?;
                    resType = mktype(inPath.clone(), classState.clone(), inTypesVarLst.clone(), inTypesTypeOption.clone(), inEqualityConstraint.clone(), inClass.clone(), inheritedComment)?;
                    Ok(resType.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE { .. }, _, Some(Deref @ DAE::Type::T_ARRAY { ty: arrayType, .. }), Some(_), _) => {
                    let mut resType: metamodelica::Ref<DAE::Type>;
                    let mut classState: ClassInf::State;
                    classState = arrayTTypeToClassInfState(metamodelica::AsArg::as_arg(&arrayType))?;
                    resType = mktype(inPath.clone(), classState.clone(), inTypesVarLst.clone(), inTypesTypeOption.clone(), inEqualityConstraint.clone(), inClass.clone(), inheritedComment)?;
                    resType = metamodelica::Ref::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: inState.clone(), varLst: metamodelica::nil(), complexType: resType.clone(), equalityConstraint: inEqualityConstraint.clone() });
                    Ok(resType.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_TUPLE { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_OPTION { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_LIST { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_POLYMORPHIC { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_ARRAY { path: _ }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::META_UNIONTYPE { path: _, .. }, _, Some(bc2), _, _) => {
                    Ok(bc2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::META_UNIONTYPE { path: _, .. }, _, _, _, _) => {
                    let mut pstr: ArcStr;
                    let mut info: SourceInfo;
                    pstr = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                    info = SCodeUtil::elementInfo(&inClass);
                    Error::addSourceMessage(&(Error::META_UNIONTYPE_ALIAS_MODS.clone()), list![pstr.clone()], &info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, st, l, None, equalityConstraint, _) => {
                    if '__try0: {
                        let ClassInf::META_UNIONTYPE { path: _, .. } = (st.clone()) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok(metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: st.clone(), varLst: l.clone(), equalityConstraint: equalityConstraint.clone(), usedExternally: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, st, l, Some(bc), equalityConstraint, _) => {
                    if '__try0: {
                        let ClassInf::META_UNIONTYPE { path: _, .. } = (st.clone()) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok(metamodelica::Ref::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: st.clone(), varLst: l.clone(), complexType: bc.clone(), equalityConstraint: equalityConstraint.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outType)
}

fn arrayTTypeToClassInfState<'__b>(mut arrayType: &'__b metamodelica::Ref<DAE::Type>) -> Result<ClassInf::State> {
    '__tco: loop {
        match &**arrayType {
            DAE::Type::T_INTEGER { .. } => {
                return Ok(ClassInf::State::TYPE_INTEGER {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
                });
            }
            DAE::Type::T_REAL { .. } => {
                return Ok(ClassInf::State::TYPE_REAL {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
                });
            }
            DAE::Type::T_STRING { .. } => {
                return Ok(ClassInf::State::TYPE_STRING {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
                });
            }
            DAE::Type::T_BOOL { .. } => {
                return Ok(ClassInf::State::TYPE_BOOL {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
                });
            }
            DAE::Type::T_CLOCK { .. } => {
                return Ok(ClassInf::State::TYPE_CLOCK {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
                });
            }
            DAE::Type::T_ARRAY { ty: t, .. } => {
                let mut cs: ClassInf::State;
                {
                    arrayType = t;
                    continue '__tco;
                }
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub(crate) fn mktypeWithArrays(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inState: ClassInf::State,
    mut inTypesVarLst: metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inTypesTypeOption: Option<metamodelica::Ref<DAE::Type>>,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inheritedComment: &metamodelica::Ref<SCode::Comment>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (inPath, inState, inTypesVarLst, inTypesTypeOption, inClass);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ci, _, Some(tp), _) => {
                    let true = (Types::isArray(metamodelica::AsArg::as_arg(&tp))) else { return Err("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(ClassInfUtil::isConnector(metamodelica::AsArg::as_arg(&ci)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::TYPE_INTEGER { .. }, v, _, _) => {
                    getOptPath(p.clone());
                    Ok(metamodelica::Ref::new(DAE::Type::T_INTEGER { varLst: v.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_REAL { .. }, v, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_REAL { varLst: v.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_STRING { .. }, v, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_STRING { varLst: v.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_BOOL { .. }, v, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_BOOL { varLst: v.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ClassInf::State::TYPE_CLOCK { .. }, v, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_CLOCK { varLst: v.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::TYPE_ENUM { .. }, _, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: None, path: p.clone(), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::FUNCTION { .. }, vl, _, cl) => {
                    let mut functype: metamodelica::Ref<DAE::Type>;
                    let mut funcattr: DAE::FunctionAttributes;
                    funcattr = getFunctionAttributes(metamodelica::AsArg::as_arg(&cl), vl.clone(), inheritedComment)?;
                    functype = Types::makeFunctionType(p.clone(), vl.clone(), funcattr.clone())?;
                    Ok(functype.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, ClassInf::State::ENUMERATION { .. }, _, Some(enumtype), _) => {
                    let mut enumtype = (*enumtype).clone();
                    enumtype = Types::makeEnumerationType(metamodelica::AsArg::as_arg(&p), metamodelica::AsArg::as_arg(&enumtype))?;
                    Ok(enumtype.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, st, l, None, _) => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: st.clone(), varLst: l.clone(), equalityConstraint: None, usedExternally: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, st, l, Some(bc), _) => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: st.clone(), varLst: l.clone(), complexType: bc.clone(), equalityConstraint: None }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("InstUtil.mktypeWithArrays failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outType)
}

fn getOptPath(mut inPath: metamodelica::Ref<Absyn::Path>) -> Option<metamodelica::Ref<Absyn::Path>> {
    let mut outAbsynPathOption: Option<metamodelica::Ref<Absyn::Path>>;
    outAbsynPathOption = (::match_deref::match_deref! { match &(inPath) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "" } => {
            None
        },
        p => {
            Some(p.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynPathOption
}

fn checkProt(
    mut inVisibility: SCode::Visibility,
    mut inMod: &metamodelica::Ref<DAE::Mod>,
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inVisibility, inMod.clone())) {
        (SCode::Visibility::PUBLIC { .. }, _) => {
            ()
        },
        (_, Deref @ DAE::Mod::NOMOD { .. }) => {
            ()
        },
        (_, Deref @ DAE::Mod::MOD { finalPrefix: _, eachPrefix: _, subModLst: Deref @ metamodelica::ListNode::Nil, binding: None, .. }) => {
            ()
        },
        (SCode::Visibility::PROTECTED { .. }, _) => {
            let mut cref = inComponentRef;
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            str1 = ComponentReferenceBasics::printComponentRefStr(&cref)?;
            str2 = Mod::prettyPrintMod(inMod, 0)?;
            Error::addSourceMessage(&(Error::MODIFY_PROTECTED.clone()), list![str1, str2], info)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn getStateSelectFromExpOption(
    mut inExpExpOption: Option<metamodelica::Ref<DAE::Exp>>,
) -> Option<DAE::StateSelect> {
    let mut outDAEStateSelectOption: Option<DAE::StateSelect>;
    outDAEStateSelectOption = (::match_deref::match_deref! { match &(inExpExpOption) {
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "StateSelect", path: Deref @ Absyn::Path::IDENT { name: Deref @ "never" } }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::NEVER),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "StateSelect", path: Deref @ Absyn::Path::IDENT { name: Deref @ "avoid" } }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::AVOID),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "StateSelect", path: Deref @ Absyn::Path::IDENT { name: Deref @ "default" } }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::DEFAULT),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "StateSelect", path: Deref @ Absyn::Path::IDENT { name: Deref @ "prefer" } }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::PREFER),
        Some(Deref @ DAE::Exp::ENUM_LITERAL { name: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "StateSelect", path: Deref @ Absyn::Path::IDENT { name: Deref @ "always" } }, .. }) => Some(openmodelica_frontend_types::DAE::StateSelect::ALWAYS),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDAEStateSelectOption
}

pub(crate) fn isSubModNamed(mut inName: &ArcStr, mut inSubMod: &metamodelica::Ref<DAE::SubMod>) -> bool {
    let mut isNamed: bool;
    isNamed = (match &**inSubMod {
        DAE::SubMod { ident: submod_name, .. } => stringEqual(&inName, &submod_name),
        _ => false,
    });
    isNamed
}

pub(crate) fn liftRecordBinding(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inValue: &metamodelica::Ref<Values::Value>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<Values::Value>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outExp, outValue) = 'mc: {
        let __mc_input = &**inType;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, ty } => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut val: metamodelica::Ref<Values::Value>;
                    let mut ety: metamodelica::Ref<DAE::Type>;
                    let mut int_dim: i32;
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    int_dim = Expression::dimensionSize(metamodelica::AsArg::as_arg(&dim))?;
                    (exp, val) = liftRecordBinding(metamodelica::AsArg::as_arg(&ty), inExp, inValue)?;
                    ety = Types::simplifyType(inType.clone())?;
                    expl = List::fill(exp.clone(), int_dim);
                    vals = List::fill(val.clone(), int_dim);
                    exp = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ety.clone(), scalar: true, array: expl.clone() });
                    val = metamodelica::Ref::new(Values::Value::ARRAY { valueLst: vals.clone(), dimLst: list![int_dim] });
                    Ok((exp.clone(), val.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (Types::isArray(inType)) else { return Err("pattern mismatch") };
                    Ok((inExp.clone(), inValue.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outValue))
}

pub(crate) fn isTopCall(mut inCallingScope: InstTypes::CallingScope) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match inCallingScope {
        InstTypes::CallingScope::TOP_CALL { .. } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn extractCurrentName(mut sele: &metamodelica::Ref<SCode::Element>) -> Result<(ArcStr, SourceInfo)> {
    let mut ostring: ArcStr;
    let mut oinfo: SourceInfo;
    (ostring, oinfo) = (match &**sele {
        SCode::Element::CLASS { name, info, .. } => (name.clone(), info.clone()),
        SCode::Element::COMPONENT { name, info, .. } => (name.clone(), info.clone()),
        SCode::Element::EXTENDS {
            baseClassPath: path,
            info,
            ..
        } => {
            let mut ret: ArcStr;
            ret = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            (ret, info.clone())
        }
        SCode::Element::IMPORT { imp, info, .. } => {
            let mut name: ArcStr;
            name = AbsynUtil::printImportString(imp)?;
            (name, info.clone())
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((ostring, oinfo))
}

pub(crate) fn reorderConnectEquationsExpandable(
    mut cache: FCore::Cache,
    mut env: &FCore::Graph,
    mut inEquations: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<SCode::Equation>>)> {
    let mut cache: FCore::Cache = cache;
    let mut outEquations: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut delst: DoubleEnded::MutableList<metamodelica::Ref<SCode::Equation>>;
    let mut expandableEqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut crefLeft: metamodelica::Ref<Absyn::ComponentRef>;
    let mut crefRight: metamodelica::Ref<Absyn::ComponentRef>;
    let mut ty1: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_NORETCALL);
    let mut ty2: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_NORETCALL);
    if if ((inEquations).is_empty()) {
        true
    } else {
        !(System::getHasExpandableConnectors())
    } {
        outEquations = inEquations;
        return Ok((cache, outEquations));
    }
    ErrorExt::setCheckpoint(literal!("expandableConnectorsOrder"));
    delst = DoubleEnded::fromList(&(metamodelica::nil()))?;
    expandableEqs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
        for mut eq in (inEquations.clone()).into_iter().cloned() {
            if !('mc: {
                let __mc_input = eq.clone();
                if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Deref @ SCode::Equation::EQ_CONNECT { crefLeft, crefRight, .. } => {
                            let mut ty1: metamodelica::Ref<DAE::Type> = ty1.clone();
                            let mut ty2: metamodelica::Ref<DAE::Type> = ty2.clone();
                            (_, ty1, _, _) = Lookup::lookupConnectorVar(env, &(ComponentReference::toExpCref(metamodelica::AsArg::as_arg(&crefLeft))?), true)?;
                            let true = (Types::isExpandableConnector(&ty1)) else { return Err("pattern mismatch") };
                            (_, ty2, _, _) = Lookup::lookupConnectorVar(env, &(ComponentReference::toExpCref(metamodelica::AsArg::as_arg(&crefRight))?), true)?;
                            let true = (Types::isExpandableConnector(&ty2)) else { return Err("pattern mismatch") };
                            Ok((true, ty1.clone(), ty2.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    ty1 = __wb0;
                    ty2 = __wb1;
                    break 'mc __v;
                }
                if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            DoubleEnded::push_back(delst.clone(), eq.clone())?;
                            Ok(false)
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    break 'mc __v;
                }
                return Err("matchcontinue: no arm matched");
            }) {
                continue;
            }
            let __x = eq.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if (expandableEqs).is_empty() {
        ErrorExt::delCheckpoint(literal!("expandableConnectorsOrder"));
        outEquations = inEquations;
        return Ok((cache, outEquations));
    }
    ErrorExt::rollBack(literal!("expandableConnectorsOrder"));
    DoubleEnded::push_list_front(delst.clone(), &expandableEqs)?;
    DoubleEnded::push_list_back(delst.clone(), &expandableEqs)?;
    let () = (::match_deref::match_deref! { match &(expandableEqs.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => {
            DoubleEnded::push_list_back(delst.clone(), &expandableEqs)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEquations = DoubleEnded::toListAndClear(delst, metamodelica::nil())?;
    Ok((cache, outEquations))
}

pub(crate) fn sortInnerFirstTplLstElementMod(
    mut inTplLstElementMod: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) -> metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)> {
    let mut outTplLstElementMod: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    let mut innerElts: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    let mut innerouterElts: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    let mut otherElts: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    let mut innerModelicaServices: metamodelica::List<(
        metamodelica::Ref<SCode::Element>,
        metamodelica::Ref<DAE::Mod>,
    )>;
    let mut innerModelica: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    let mut innerOthers: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    if !(System::getHasInnerOuterDefinitions()) {
        outTplLstElementMod = inTplLstElementMod;
        return outTplLstElementMod;
    }
    (innerElts, innerouterElts, otherElts) = splitInnerAndOtherTplLstElementMod(inTplLstElementMod);
    (innerModelicaServices, innerModelica, innerOthers) = splitInners(
        &innerElts,
        &(metamodelica::nil()),
        &(metamodelica::nil()),
        &(metamodelica::nil()),
    );
    outTplLstElementMod = listAppend(
        innerModelicaServices,
        listAppend(
            innerModelica,
            listAppend(innerOthers, listAppend(innerouterElts, otherElts)),
        ),
    );
    outTplLstElementMod
}

fn splitInners(
    mut inTplLstElementMod: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut inAcc1: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut inAcc2: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut inAcc3: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) -> (
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) {
    let mut outModelicaServices: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    let mut outModelica: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    let mut outOthers: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    (outModelicaServices, outModelica, outOthers) = 'mc: {
        let __mc_input = &**inTplLstElementMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((inAcc1.clone().reverse(), inAcc2.clone().reverse(), inAcc3.clone().reverse()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: em, tail: rest } => {
                    let mut acc1: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut acc2: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut acc3: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut e: metamodelica::Ref<SCode::Element>;
                    let mut p: metamodelica::Ref<Absyn::Path>;
                    e = Util::tuple21(em.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(SCodeUtil::getComponentTypeSpec(&e)?) {
                        Deref @ Absyn::TypeSpec::TPATH { path: __pa0, arrayDim: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    p = metamodelica::Own::own(__pa0);
                    let true = (stringEq(&(literal!("ModelicaServices")), &(AbsynUtil::pathFirstIdent(&p)))) else { return Err("pattern mismatch") };
                    (acc1, acc2, acc3) = splitInners(metamodelica::AsArg::as_arg(&rest), &(metamodelica::cons(em.clone(), inAcc1.clone())), inAcc2, inAcc3);
                    Ok((acc1.clone(), acc2.clone(), acc3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: em, tail: rest } => {
                    let mut acc1: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut acc2: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut acc3: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut e: metamodelica::Ref<SCode::Element>;
                    let mut p: metamodelica::Ref<Absyn::Path>;
                    e = Util::tuple21(em.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(SCodeUtil::getComponentTypeSpec(&e)?) {
                        Deref @ Absyn::TypeSpec::TPATH { path: __pa0, arrayDim: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    p = metamodelica::Own::own(__pa0);
                    let true = (stringEq(&(literal!("Modelica")), &(AbsynUtil::pathFirstIdent(&p)))) else { return Err("pattern mismatch") };
                    (acc1, acc2, acc3) = splitInners(metamodelica::AsArg::as_arg(&rest), inAcc1, &(metamodelica::cons(em.clone(), inAcc2.clone())), inAcc3);
                    Ok((acc1.clone(), acc2.clone(), acc3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: em @ _, tail: rest } => {
                    let mut acc1: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut acc2: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    let mut acc3: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
                    (acc1, acc2, acc3) = splitInners(metamodelica::AsArg::as_arg(&rest), inAcc1, inAcc2, &(metamodelica::cons(em.clone(), inAcc3.clone())));
                    Ok((acc1.clone(), acc2.clone(), acc3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outModelicaServices, outModelica, outOthers)
}

pub(crate) fn splitInnerAndOtherTplLstElementMod(
    mut inTplLstElementMod: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) -> (
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) {
    let mut outInnerTplLstElementMod: metamodelica::List<(
        metamodelica::Ref<SCode::Element>,
        metamodelica::Ref<DAE::Mod>,
    )> = metamodelica::nil();
    let mut outInnerOuterTplLstElementMod: metamodelica::List<(
        metamodelica::Ref<SCode::Element>,
        metamodelica::Ref<DAE::Mod>,
    )> = metamodelica::nil();
    let mut outOtherTplLstElementMod: metamodelica::List<(
        metamodelica::Ref<SCode::Element>,
        metamodelica::Ref<DAE::Mod>,
    )> = metamodelica::nil();
    let mut comp: metamodelica::Ref<SCode::Element>;
    let mut io: Absyn::InnerOuter;
    for mut e in &*inTplLstElementMod.reverse() {
        (comp, _) = e.clone();
        let () = (::match_deref::match_deref! { match &(comp) {
            Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, .. } if (AbsynUtil::isInner(io.clone())) => {
                if AbsynUtil::isOuter(io.clone()) {
                    outInnerOuterTplLstElementMod = metamodelica::cons(e.clone(), outInnerOuterTplLstElementMod);
                } else {
                    outInnerTplLstElementMod = metamodelica::cons(e.clone(), outInnerTplLstElementMod);
                }
                ()
            },
            _ => {
                outOtherTplLstElementMod = metamodelica::cons(e.clone(), outOtherTplLstElementMod);
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    (
        outInnerTplLstElementMod,
        outInnerOuterTplLstElementMod,
        outOtherTplLstElementMod,
    )
}

pub(crate) fn splitEltsOrderInnerOuter(
    mut elts: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
)> {
    let mut cdefImpElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut classextendsElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut extElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut compElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    (cdefImpElts, classextendsElts, extElts, compElts) = (::match_deref::match_deref! { match elts {
        _ => {
            let mut innerComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut otherComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut comps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(elts)?;
            comps = listAppend(innerComps, otherComps);
            (cdefImpElts, classextendsElts, extElts, comps)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cdefImpElts, classextendsElts, extElts, compElts))
}

pub(crate) fn splitElts(
    mut elts: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
)> {
    let mut cdefImpElts: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut classextendsElts: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut extElts: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut compElts: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    for mut elt in &**elts {
        let () = (match &*elt.clone() {
            SCode::Element::CLASS {
                classDef: __elt_classDef,
                ..
            } => {
                if (match &*__elt_classDef.clone() {
                    SCode::ClassDef::CLASS_EXTENDS { .. } => true,
                    _ => false,
                }) {
                    classextendsElts = metamodelica::cons(elt.clone(), classextendsElts);
                } else {
                    cdefImpElts = metamodelica::cons(elt.clone(), cdefImpElts);
                }
                ()
            }
            SCode::Element::IMPORT { .. } => {
                cdefImpElts = metamodelica::cons(elt.clone(), cdefImpElts);
                ()
            }
            SCode::Element::DEFINEUNIT { .. } => {
                cdefImpElts = metamodelica::cons(elt.clone(), cdefImpElts);
                ()
            }
            SCode::Element::EXTENDS { .. } => {
                extElts = metamodelica::cons(elt.clone(), extElts);
                ()
            }
            SCode::Element::COMPONENT { .. } => {
                compElts = metamodelica::cons(elt.clone(), compElts);
                ()
            }
        });
    }
    cdefImpElts = metamodelica::Dangerous::listReverseInPlace(cdefImpElts);
    classextendsElts = metamodelica::Dangerous::listReverseInPlace(classextendsElts);
    extElts = metamodelica::Dangerous::listReverseInPlace(extElts);
    compElts = metamodelica::Dangerous::listReverseInPlace(compElts);
    Ok((cdefImpElts, classextendsElts, extElts, compElts))
}

pub(crate) fn splitEltsNoComponents(
    mut elts: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
)> {
    let mut impElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut defElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut classextendsElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut filtered: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    (impElts, defElts, classextendsElts, filtered) = (::match_deref::match_deref! { match elts {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: elt @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs)?;
            (impElts, defElts, metamodelica::cons(elt.clone(), classextendsElts), filtered)
        },
        Deref @ metamodelica::ListNode::Cons { head: elt @ Deref @ SCode::Element::CLASS { .. }, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs)?;
            (impElts, metamodelica::cons(elt.clone(), defElts), classextendsElts, metamodelica::cons(elt.clone(), filtered))
        },
        Deref @ metamodelica::ListNode::Cons { head: elt @ Deref @ SCode::Element::IMPORT { .. }, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs)?;
            (metamodelica::cons(elt.clone(), impElts), defElts, classextendsElts, filtered)
        },
        Deref @ metamodelica::ListNode::Cons { head: elt @ Deref @ SCode::Element::DEFINEUNIT { .. }, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs)?;
            (impElts, metamodelica::cons(elt.clone(), defElts), classextendsElts, metamodelica::cons(elt.clone(), filtered))
        },
        Deref @ metamodelica::ListNode::Cons { head: elt, tail: xs } => {
            (impElts, defElts, classextendsElts, filtered) = splitEltsNoComponents(xs)?;
            (impElts, defElts, classextendsElts, metamodelica::cons(elt.clone(), filtered))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((impElts, defElts, classextendsElts, filtered))
}

pub(crate) fn splitEltsInnerAndOther(
    mut elts: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
)> {
    let mut cdefImpElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut classextendsElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut extElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut innerCompElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut otherCompElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    (cdefImpElts, classextendsElts, extElts, innerCompElts, otherCompElts) = (::match_deref::match_deref! { match elts {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: cdef @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, tail: xs } => {
            let mut innerComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut otherComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs)?;
            (cdefImpElts, metamodelica::cons(cdef.clone(), classextendsElts), extElts, innerComps, otherComps)
        },
        Deref @ metamodelica::ListNode::Cons { head: cdef @ Deref @ SCode::Element::CLASS { .. }, tail: xs } => {
            let mut innerComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut otherComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs)?;
            (metamodelica::cons(cdef.clone(), cdefImpElts), classextendsElts, extElts, innerComps, otherComps)
        },
        Deref @ metamodelica::ListNode::Cons { head: imp @ Deref @ SCode::Element::IMPORT { .. }, tail: xs } => {
            let mut innerComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut otherComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs)?;
            (metamodelica::cons(imp.clone(), cdefImpElts), classextendsElts, extElts, innerComps, otherComps)
        },
        Deref @ metamodelica::ListNode::Cons { head: imp @ Deref @ SCode::Element::DEFINEUNIT { .. }, tail: xs } => {
            let mut innerComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut otherComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs)?;
            (metamodelica::cons(imp.clone(), cdefImpElts), classextendsElts, extElts, innerComps, otherComps)
        },
        Deref @ metamodelica::ListNode::Cons { head: ext @ Deref @ SCode::Element::EXTENDS { .. }, tail: xs } => {
            let mut innerComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut otherComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs)?;
            (cdefImpElts, classextendsElts, metamodelica::cons(ext.clone(), extElts), innerComps, otherComps)
        },
        Deref @ metamodelica::ListNode::Cons { head: comp @ Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, .. }, tail: xs } => {
            let mut innerComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut otherComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let true = (AbsynUtil::isInner(io.clone())) else { return Err("pattern mismatch") };
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs)?;
            (cdefImpElts, classextendsElts, extElts, metamodelica::cons(comp.clone(), innerComps), otherComps)
        },
        Deref @ metamodelica::ListNode::Cons { head: comp @ Deref @ SCode::Element::COMPONENT { .. }, tail: xs } => {
            let mut innerComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut otherComps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            (cdefImpElts, classextendsElts, extElts, innerComps, otherComps) = splitEltsInnerAndOther(xs)?;
            (cdefImpElts, classextendsElts, extElts, innerComps, metamodelica::cons(comp.clone(), otherComps))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((cdefImpElts, classextendsElts, extElts, innerCompElts, otherCompElts))
}

fn orderComponents(
    mut inComp: metamodelica::Ref<SCode::Element>,
    mut inCompElts: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outCompElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    outCompElts = (::match_deref::match_deref! { match &(inComp.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::INPUT { .. }, .. }, .. } => {
            metamodelica::cons(inComp, inCompElts)
        },
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { direction: Absyn::Direction::OUTPUT { .. }, .. }, .. } => {
            metamodelica::cons(inComp, inCompElts)
        },
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { innerOuter: Absyn::InnerOuter::INNER { .. }, .. }, .. } => {
            metamodelica::cons(inComp, inCompElts)
        },
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { innerOuter: Absyn::InnerOuter::INNER_OUTER { .. }, .. }, .. } => {
            metamodelica::cons(inComp, inCompElts)
        },
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { variability: SCode::Variability::CONST { .. }, .. }, .. } => {
            metamodelica::cons(inComp, inCompElts)
        },
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { variability: SCode::Variability::PARAM { .. }, .. }, .. } => {
            metamodelica::cons(inComp, inCompElts)
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            let mut compElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            compElts = listAppend(inCompElts, list![inComp]);
            compElts
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outCompElts)
}

fn splitClassExtendsElts(
    mut elts: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> (
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
) {
    let mut classextendsElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut outElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    (classextendsElts, outElts) = (::match_deref::match_deref! { match elts {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: cdef @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, tail: xs } => {
            let mut res: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            (classextendsElts, res) = splitClassExtendsElts(xs);
            (metamodelica::cons(cdef.clone(), classextendsElts), res)
        },
        Deref @ metamodelica::ListNode::Cons { head: cdef, tail: xs } => {
            let mut res: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            (classextendsElts, res) = splitClassExtendsElts(xs);
            (classextendsElts, metamodelica::cons(cdef.clone(), res))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (classextendsElts, outElts)
}

fn addClassdefsToEnv3(
    mut inCache: FCore::Cache,
    mut env: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inMod: Option<metamodelica::Ref<DAE::Mod>>,
    mut sele: metamodelica::Ref<SCode::Element>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    metamodelica::Ref<SCode::Element>,
)> {
    let mut outCache: FCore::Cache;
    let mut oenv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut osele: metamodelica::Ref<SCode::Element>;
    (outCache, oenv, outIH, osele) = (::match_deref::match_deref! { match &((inMod, sele.clone())) {
        (None, _) => {
            return Err("fail")
        },
        (Some(Deref @ DAE::Mod::MOD { subModLst: lsm, .. }), Deref @ SCode::Element::CLASS { name: r#str, .. }) => {
            let mut cache = inCache;
            let mut ih = inIH;
            let mut pre = inPrefix;
            let mut mo2: metamodelica::Ref<DAE::Mod>;
            let mut sele2: metamodelica::Ref<SCode::Element>;
            let mut env2: FCore::Graph;
            (mo2, _) = extractCorrectClassMod2(metamodelica::AsArg::as_arg(&lsm), metamodelica::AsArg::as_arg(&r#str), &(metamodelica::nil()));
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Inst::redeclareType(cache, env, ih, mo2, sele, pre, ClassInf::State::MODEL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: r#str.clone() }) }, true, openmodelica_frontend_types::DAE::Mod::interned_NOMOD())?) {
                (__pa0, __pa1, __pa2, __pa3 @ Deref @ SCode::Element::CLASS { .. }, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            env2 = metamodelica::Own::own(__pa1);
            ih = metamodelica::Own::own(__pa2);
            sele2 = metamodelica::Own::own(__pa3);
            (cache, env2, ih, sele2)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, oenv, outIH, osele))
}

fn extractCorrectClassMod2(
    mut smod: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
    mut name: &ArcStr,
    mut premod: &metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
) -> (
    metamodelica::Ref<DAE::Mod>,
    metamodelica::List<metamodelica::Ref<DAE::SubMod>>,
) {
    let mut omod: metamodelica::Ref<DAE::Mod>;
    let mut restmods: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
    (omod, restmods) = (::match_deref::match_deref! { match smod {
        Deref @ metamodelica::ListNode::Nil => {
            (openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), premod.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::SubMod { ident: id, r#mod }, tail: rest } if (stringEq(&id, &name)) => {
            let mut rest2: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
            rest2 = listAppend(premod.clone(), rest.clone());
            (r#mod.clone(), rest2)
        },
        Deref @ metamodelica::ListNode::Cons { head: sub, tail: rest } => {
            let mut r#mod: metamodelica::Ref<DAE::Mod>;
            let mut rest2: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
            (r#mod, rest2) = extractCorrectClassMod2(rest, name, premod);
            (r#mod, metamodelica::cons(sub.clone(), rest2))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (omod, restmods)
}

pub(crate) fn traverseModAddFinal(mut r#mod: metamodelica::Ref<SCode::Mod>) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod;
    r#mod = 'mc: {
        let __mc_input = r#mod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::NOMOD { .. } => {
                    Ok(r#mod.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::REDECL { eachPrefix: each_, element: element1, .. } => {
                    let mut element2: metamodelica::Ref<SCode::Element>;
                    element2 = traverseModAddFinal3(element1.clone())?;
                    Ok(if (referenceEq(&*(element1.clone()),&*(&*element2))) {r#mod.clone()} else {metamodelica::Ref::new(SCode::Mod::REDECL { finalPrefix: openmodelica_frontend_types::SCode::Final::FINAL, eachPrefix: each_.clone(), element: element2.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::MOD { finalPrefix: f, eachPrefix: each_, subModLst: subs1, binding: eq, comment: cmt, info } => {
                    let mut subs2: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
                    subs2 = List::mapCheckReferenceEq(subs1.clone(), &traverseModAddFinal4)?;
                    Ok(if (openmodelica_frontend_types::SCode::Final::FINAL == f.clone() && metamodelica::ReferenceEq::reference_eq(&(subs1.clone()), &(subs2))) {r#mod.clone()} else {metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::FINAL, eachPrefix: each_.clone(), subModLst: subs2.clone(), binding: eq.clone(), comment: cmt.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("InstUtil.traverseModAddFinal"), metamodelica::sourceInfo!("FrontEnd/InstUtil.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(r#mod)
}

fn traverseModAddFinal3(mut inElement: metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outElement: metamodelica::Ref<SCode::Element>;
    outElement = 'mc: {
        let __mc_input = &*inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { name, prefixes, attributes: attr, typeSpec: tySpec, modifications: oldmod, comment: cmt, condition: cond, info } => {
                    let mut r#mod: metamodelica::Ref<SCode::Mod>;
                    r#mod = traverseModAddFinal(oldmod.clone())?;
                    Ok(if (referenceEq(&*(oldmod.clone()),&*(&*r#mod))) {inElement.clone()} else {metamodelica::Ref::new(SCode::Element::COMPONENT { name: name.clone(), prefixes: prefixes.clone(), attributes: attr.clone(), typeSpec: tySpec.clone(), modifications: r#mod.clone(), comment: cmt.clone(), condition: cond.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::IMPORT { .. } => {
                    Ok(inElement.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { .. } => {
                    Ok(inElement.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::EXTENDS { baseClassPath: p, visibility: vis, modifications: oldmod, ann, info } => {
                    let mut r#mod: metamodelica::Ref<SCode::Mod>;
                    r#mod = traverseModAddFinal(oldmod.clone())?;
                    Ok(if (referenceEq(&*(oldmod.clone()),&*(&*r#mod))) {inElement.clone()} else {metamodelica::Ref::new(SCode::Element::EXTENDS { baseClassPath: p.clone(), visibility: vis.clone(), modifications: r#mod.clone(), ann: ann.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!(" we failed with traverseModAddFinal3\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outElement)
}

fn traverseModAddFinal4(mut sub: metamodelica::Ref<SCode::SubMod>) -> Result<metamodelica::Ref<SCode::SubMod>> {
    let mut sub: metamodelica::Ref<SCode::SubMod> = sub;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    r#mod = traverseModAddFinal(sub.r#mod.clone())?;
    if !(referenceEq(&*(sub.r#mod.clone()), &*(&*r#mod))) {
        assign_field!(sub.r#mod = r#mod);
    }
    Ok(sub)
}

pub(crate) fn traverseModAddDims(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPrefix: DAE::Prefix,
    mut inMod: metamodelica::Ref<SCode::Mod>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    outMod = 'mc: {
        let __mc_input = (inCache, inEnv, inPrefix, inMod.clone(), inInstDims);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, r#mod, _) => {
                    let true = (Config::splitArrays()?) else { return Err("pattern mismatch") };
                    Ok(r#mod.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(inMod.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, pre, r#mod, inst_dims) => {
                    let mut mod2: metamodelica::Ref<SCode::Mod>;
                    let mut exps: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let mut aexps: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>;
                    exps = List::map(inst_dims.clone(), &move |__a0: metamodelica::List<metamodelica::Ref<DAE::Dimension>>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::dimensionsToExps(&__a0)) })?;
                    aexps = List::mapList(exps.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::unelabExp(&__a0))?;
                    mod2 = traverseModAddDims4(cache.clone(), env.clone(), pre.clone(), r#mod.clone(), aexps.clone(), true)?;
                    Ok(mod2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMod)
}

fn traverseModAddDims4(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPrefix: DAE::Prefix,
    mut inMod: metamodelica::Ref<SCode::Mod>,
    mut inExps: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
    mut inIsTop: bool,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    outMod = (match &*inMod {
        SCode::Mod::NOMOD { .. } => inMod,
        SCode::Mod::REDECL { .. } => inMod,
        SCode::Mod::MOD {
            eachPrefix: SCode::Each::NOT_EACH { .. },
            ..
        } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut pre = inPrefix;
            let mut exps = inExps;
            let mut submods2: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
            let mut binding: Option<metamodelica::Ref<Absyn::Exp>>;
            submods2 = traverseModAddDims5(cache, env, pre, var_field!((*inMod).subModLst, SCode::Mod::MOD), &exps)?;
            binding = insertSubsInBinding(var_field!((*inMod).binding, SCode::Mod::MOD).clone(), exps)?;
            metamodelica::Ref::new(SCode::Mod::MOD {
                finalPrefix: var_field!((*inMod).finalPrefix, SCode::Mod::MOD).clone(),
                eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                subModLst: submods2,
                binding: binding,
                comment: var_field!((*inMod).comment, SCode::Mod::MOD).clone(),
                info: var_field!((*inMod).info, SCode::Mod::MOD).clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outMod)
}

fn traverseModAddDims5(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPrefix: DAE::Prefix,
    mut inMods: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut inExps: &metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::SubMod>>> {
    let mut outMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    outMods = (::match_deref::match_deref! { match inMods {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: n, r#mod }, tail: smods } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut pre = inPrefix;
            let mut mod2: metamodelica::Ref<SCode::Mod>;
            let mut smods2: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
            mod2 = traverseModAddDims4(cache.clone(), env.clone(), pre.clone(), r#mod.clone(), inExps.clone(), false)?;
            smods2 = traverseModAddDims5(cache, env, pre, smods, inExps)?;
            metamodelica::cons(metamodelica::Ref::new(SCode::SubMod { ident: n.clone(), r#mod: mod2 }), smods2)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMods)
}

fn insertSubsInBinding(
    mut inOpt: Option<metamodelica::Ref<Absyn::Exp>>,
    mut inExps: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
) -> Result<Option<metamodelica::Ref<Absyn::Exp>>> {
    let mut outOpt: Option<metamodelica::Ref<Absyn::Exp>>;
    outOpt = (::match_deref::match_deref! { match &(inOpt) {
        None => {
            None
        },
        Some(e) => {
            let mut exps = inExps;
            let mut e2: metamodelica::Ref<Absyn::Exp>;
            let mut subs: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>;
            let mut vars: metamodelica::List<metamodelica::List<ArcStr>>;
            vars = generateUnusedNamesLstCall(metamodelica::AsArg::as_arg(&e), &exps)?;
            subs = List::mapList(vars.clone(), &fnptr!(stringSub, ArcStr))?;
            (e2, _) = AbsynUtil::traverseExp(e.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::crefInsertSubscriptLstLst, metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>)> + 'static>), subs)?;
            e2 = wrapIntoForLst(e2, &vars, &exps)?;
            Some(e2)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outOpt)
}

fn generateUnusedNames(
    mut inExp: &metamodelica::Ref<Absyn::Exp>,
    mut inList: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outNames: metamodelica::List<ArcStr>;
    (outNames, _) = generateUnusedNames2(inList, 1)?;
    Ok(outNames)
}

fn generateUnusedNames2(
    mut inList: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inInt: i32,
) -> Result<(metamodelica::List<ArcStr>, i32)> {
    let mut outNames: metamodelica::List<ArcStr>;
    let mut outInt: i32;
    (outNames, outInt) = (::match_deref::match_deref! { match inList {
        Deref @ metamodelica::ListNode::Nil => {
            let mut i = inInt;
            (metamodelica::nil(), i)
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: exps } => {
            let mut i = inInt;
            let mut i1: i32;
            let mut i2: i32;
            let mut s: ArcStr;
            let mut names: metamodelica::List<ArcStr>;
            s = intString(i);
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("i")); __mm_s.push_str(&*s); ArcStr::from(__mm_s) };
            i1 = i + 1;
            (names, i2) = generateUnusedNames2(exps, i1)?;
            (metamodelica::cons(s, names), i2)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outNames, outInt))
}

fn generateUnusedNamesLst(
    mut inList: &metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
    mut inInt: i32,
) -> Result<(metamodelica::List<metamodelica::List<ArcStr>>, i32)> {
    let mut outNames: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut outInt: i32;
    (outNames, outInt) = (::match_deref::match_deref! { match inList {
        Deref @ metamodelica::ListNode::Nil => {
            let mut i = inInt;
            (metamodelica::nil(), i)
        },
        Deref @ metamodelica::ListNode::Cons { head: e0, tail: exps } => {
            let mut i = inInt;
            let mut i1: i32;
            let mut i2: i32;
            let mut names: metamodelica::List<metamodelica::List<ArcStr>>;
            let mut ns: metamodelica::List<ArcStr>;
            (ns, i1) = generateUnusedNames2(metamodelica::AsArg::as_arg(&e0), i)?;
            (names, i2) = generateUnusedNamesLst(exps, i1)?;
            (metamodelica::cons(ns, names), i2)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outNames, outInt))
}

fn generateUnusedNamesLstCall(
    mut inExp: &metamodelica::Ref<Absyn::Exp>,
    mut inList: &metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
) -> Result<metamodelica::List<metamodelica::List<ArcStr>>> {
    let mut outNames: metamodelica::List<metamodelica::List<ArcStr>>;
    (outNames, _) = generateUnusedNamesLst(inList, 1)?;
    Ok(outNames)
}

fn stringsSubs(mut inNames: &metamodelica::List<ArcStr>) -> metamodelica::List<metamodelica::Ref<Absyn::Subscript>> {
    let mut outSubs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    outSubs = (::match_deref::match_deref! { match inNames {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: n, tail: names } => {
            let mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            subs = stringsSubs(names);
            metamodelica::cons(metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: n.clone(), subscripts: metamodelica::nil() }) }) }), subs)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outSubs
}

fn stringSub(mut inName: ArcStr) -> metamodelica::Ref<Absyn::Subscript> {
    let mut outSub: metamodelica::Ref<Absyn::Subscript>;
    outSub = (match inName {
        mut n => metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT {
            subscript: metamodelica::Ref::new(Absyn::Exp::CREF {
                componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                    name: n,
                    subscripts: metamodelica::nil(),
                }),
            }),
        }),
    });
    outSub
}

fn wrapIntoFor(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inNames: &metamodelica::List<ArcStr>,
    mut inRanges: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    outExp = (::match_deref::match_deref! { match (inNames, inRanges) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            let mut e = inExp;
            e
        },
        (Deref @ metamodelica::ListNode::Cons { head: n, tail: names }, Deref @ metamodelica::ListNode::Cons { head: r, tail: ranges }) => {
            let mut e = inExp;
            let mut e2: metamodelica::Ref<Absyn::Exp>;
            e2 = wrapIntoFor(e, names, ranges)?;
            metamodelica::Ref::new(Absyn::Exp::CALL { function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("array"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FOR_ITER_FARG { exp: e2, iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, iterators: list![metamodelica::Ref::new(Absyn::ForIterator { name: n.clone(), guardExp: None, range: Some(metamodelica::Ref::new(Absyn::Exp::RANGE { start: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 1 }), step: None, stop: r.clone() })) })] }), typeVars: metamodelica::nil() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

fn wrapIntoForLst(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inNames: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut inRanges: &metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    outExp = (::match_deref::match_deref! { match (inNames, inRanges) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            let mut e = inExp;
            e
        },
        (Deref @ metamodelica::ListNode::Cons { head: n, tail: names }, Deref @ metamodelica::ListNode::Cons { head: r, tail: ranges }) => {
            let mut e = inExp;
            let mut e2: metamodelica::Ref<Absyn::Exp>;
            let mut e3: metamodelica::Ref<Absyn::Exp>;
            e2 = wrapIntoForLst(e, names, ranges)?;
            e3 = wrapIntoFor(e2, metamodelica::AsArg::as_arg(&n), metamodelica::AsArg::as_arg(&r))?;
            e3
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

pub(crate) fn componentHasCondition(
    mut component: &(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
) -> bool {
    let mut hasCondition: bool;
    hasCondition = (::match_deref::match_deref! { match &(component) {
        (Deref @ SCode::Element::COMPONENT { condition: Some(_), .. }, _) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasCondition
}

pub(crate) fn instElementCondExp(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut component: &metamodelica::Ref<SCode::Element>,
    mut prefix: DAE::Prefix,
    mut info: SourceInfo,
) -> (Option<bool>, FCore::Cache) {
    let mut outCondValue: Option<bool>;
    let mut outCache: FCore::Cache;
    (outCondValue, outCache) = 'mc: {
        let __mc_input = &**component;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { condition: Some(cond_exp), .. } => {
                    let mut cond_val: bool;
                    let mut cache: FCore::Cache;
                    (cond_val, cache) = instConditionalDeclaration(inCache.clone(), inEnv.clone(), cond_exp.clone(), prefix.clone(), info.clone())?;
                    Ok((Some(cond_val), cache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { condition: Some(_), .. } => {
                    Ok((None, inCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((Some(true), inCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outCondValue, outCache)
}

fn instConditionalDeclaration(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inCondition: metamodelica::Ref<Absyn::Exp>,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(bool, FCore::Cache)> {
    let mut outIsConditional: bool;
    let mut outCache: FCore::Cache;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut t: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut b: bool;
    let mut val: metamodelica::Ref<Values::Value>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Static::elabExp(inCache, inEnv.clone(), inCondition.clone(), false, false, inPrefix, inInfo.clone())?) {
        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa0);
    e = metamodelica::Own::own(__pa1);
    t = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    if !(Types::isBoolean(&t)) {
        Error::addSourceMessageAndFail(
            &(Error::IF_CONDITION_TYPE_ERROR.clone()),
            list![
                Dump::printExpStr(inCondition.clone())?,
                TypesDump::unparseTypeNoAttr(&t)?
            ],
            &inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if !(Types::isParameterOrConstant(c)) {
        Error::addSourceMessageAndFail(
            &(Error::COMPONENT_CONDITION_VARIABILITY.clone()),
            list![Dump::printExpStr(inCondition.clone())?],
            &inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (outCache, val) = Ceval::ceval(outCache, inEnv, e, false, Absyn::Msg::MSG { info: inInfo.clone() }, 0)?;
    outIsConditional = (match &*val {
        Values::Value::BOOL { boolean: __esc_b } => {
            b = (*__esc_b).clone();
            b.clone()
        }
        Values::Value::EMPTY { .. } => {
            if !(Config::getGraphicsExpMode()?) {
                Error::addSourceMessage(
                    &(Error::CONDITIONAL_EXP_WITHOUT_VALUE.clone()),
                    list![Dump::printExpStr(inCondition)?],
                    &inInfo,
                )?;
                return Err("fail");
            }
            true
        }
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("InstUtil.instConditionalDeclaration got unexpected value "));
                    __mm_s.push_str(&*ValuesDump::valString(&val)?);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("FrontEnd/InstUtil.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok((outIsConditional, outCache))
}

pub(crate) fn propagateClassPrefix(mut attr: SCode::Attributes, mut pre: &DAE::Prefix) -> SCode::Attributes {
    let mut outAttr: SCode::Attributes;
    outAttr = (match (attr.clone(), pre.clone()) {
        (
            _,
            DAE::Prefix::PREFIX {
                compPre: _,
                classPre:
                    DAE::ClassPrefix {
                        variability: SCode::Variability::VAR { .. },
                    },
            },
        ) => attr,
        (
            SCode::Attributes {
                variability: SCode::Variability::CONST { .. },
                ..
            },
            _,
        ) => attr,
        (
            SCode::Attributes {
                arrayDims: ref ad,
                connectorType: mut ct,
                parallelism: mut prl,
                variability: _,
                direction: mut dir,
                isField: mut isf,
            },
            DAE::Prefix::PREFIX {
                compPre: _,
                classPre: DAE::ClassPrefix { variability: mut vt },
            },
        ) => SCode::Attributes {
            arrayDims: ad.clone(),
            connectorType: ct.clone(),
            parallelism: prl.clone(),
            variability: vt.clone(),
            direction: dir.clone(),
            isField: isf.clone(),
        },
        _ => attr,
    });
    outAttr
}

pub(crate) fn checkUseConstValue(
    mut useConstValue: bool,
    mut ie: metamodelica::Ref<DAE::Exp>,
    mut v: Option<metamodelica::Ref<Values::Value>>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outE: metamodelica::Ref<DAE::Exp>;
    outE = 'mc: {
        let __mc_input = (useConstValue, ie, v);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, e, _) => {
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, Some(val)) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = ValuesUtil::valueExp(val.clone(), None)?;
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e, _) => {
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outE
}

pub(crate) fn propagateAbSCDirection(
    mut inVariability: SCode::Variability,
    mut inAttributes: SCode::Attributes,
    mut inClassAttributes: Option<SCode::Attributes>,
    mut inInfo: &SourceInfo,
) -> Result<SCode::Attributes> {
    let mut outAttributes: SCode::Attributes;
    outAttributes = (match inVariability {
        SCode::Variability::CONST { .. } => inAttributes,
        SCode::Variability::PARAM { .. } => inAttributes,
        _ => {
            let mut dir: Absyn::Direction;
            let SCode::ATTR { direction: __pa0, .. } = &inAttributes;
            dir = metamodelica::Own::own(__pa0);
            dir = propagateAbSCDirection2(dir, inClassAttributes, inInfo)?;
            SCodeUtil::setAttributesDirection(inAttributes, dir)
        }
    });
    Ok(outAttributes)
}

pub(crate) fn propagateAbSCDirection2(
    mut v1: Absyn::Direction,
    mut optDerAttr: Option<SCode::Attributes>,
    mut inInfo: &SourceInfo,
) -> Result<Absyn::Direction> {
    let mut v3: Absyn::Direction;
    v3 = (match (v1, optDerAttr) {
        (_, None) => v1,
        (Absyn::Direction::BIDIR { .. }, Some(SCode::Attributes { direction: mut v2, .. })) => v2.clone(),
        (
            _,
            Some(SCode::Attributes {
                direction: Absyn::Direction::BIDIR { .. },
                ..
            }),
        ) => v1,
        (_, Some(SCode::Attributes { direction: mut v2, .. })) if (v1 == v2.clone()) => v1,
        _ => {
            metamodelica::print(literal!(
                " failure in propagateAbSCDirection2, Absyn.DIRECTION mismatch"
            ));
            Error::addSourceMessage(
                &(Error::COMPONENT_INPUT_OUTPUT_MISMATCH.clone()),
                list![literal!(""), literal!("")],
                inInfo,
            )?;
            return Err("fail");
        }
    });
    Ok(v3)
}

pub(crate) fn makeCrefBaseType(
    mut inBaseType: metamodelica::Ref<DAE::Type>,
    mut inDimensions: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = Types::simplifyType(makeCrefBaseType2(inBaseType, inDimensions)?)?;
    Ok(outType)
}

fn makeCrefBaseType2(
    mut inBaseType: metamodelica::Ref<DAE::Type>,
    mut inDimensions: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (&*inBaseType, &**inDimensions);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty, .. }, _) => {
                    let false = (((TypesDump::getDimensions(metamodelica::AsArg::as_arg(&ty)))).is_empty()) else { return Err("pattern mismatch") };
                    Ok(ty.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(inBaseType.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    dims = List::last(inDimensions)?;
                    ty = Expression::liftArrayLeftList(inBaseType.clone(), dims.clone());
                    Ok(ty.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outType)
}

pub(crate) fn getCrefFromCompDim(
    mut inEle: &metamodelica::Ref<SCode::Element>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut cref: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    cref = 'mc: {
        let __mc_input = &**inEle;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: ads, .. }, .. } => {
                    Ok(AbsynUtil::getCrefsFromSubs(metamodelica::AsArg::as_arg(&ads), true, true)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    cref
}

pub(crate) fn getCrefFromCond(
    mut cond: Option<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    crefs = (::match_deref::match_deref! { match &(cond) {
        None => {
            metamodelica::nil()
        },
        Some(e) => {
            AbsynUtil::getCrefFromExp(e.clone(), true, true)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefs)
}

fn checkVariabilityOfUpdatedComponent(
    mut variability: SCode::Variability,
    mut cref: &metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<()> {
    let () = (match variability {
        SCode::Variability::VAR { .. } => (),
        SCode::Variability::DISCRETE { .. } => (),
        _ => return Err("fail"),
    });
    Ok(())
}

pub(crate) fn propagateBinding(mut inVarsDae: DAE::DAElist, mut inEquationsDae: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outVarsDae: DAE::DAElist;
    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut vars1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut equations: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut equations1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut v1: metamodelica::Ref<DAE::Element>;
    let mut i: i32 = 0;
    let mut is: metamodelica::List<i32>;
    let mut e: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let DAE::DAE { elementLst: __pa0 } = &inVarsDae;
    vars = metamodelica::Own::own(__pa0);
    let DAE::DAE { elementLst: __pa1 } = inEquationsDae;
    equations = metamodelica::Own::own(__pa1);
    if (vars).is_empty() || (equations).is_empty() {
        outVarsDae = inVarsDae;
        return Ok(outVarsDae);
    }
    vars1 = metamodelica::nil();
    is = metamodelica::nil();
    for mut v in &*vars {
        v1 = 'mc: {
            let __mc_input = v.clone();
            if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    v1 @ Deref @ DAE::Element::VAR { .. } => {
                        let mut v1 = (*v1).clone();
                        let mut e: metamodelica::Ref<DAE::Exp> = e.clone();
                        let mut i: i32 = i.clone();
                        let mut is: metamodelica::List<i32> = is.clone();
                        (e, i) = findCorrespondingBinding(var_field!((*v1).componentRef, DAE::Element::VAR).clone(), &equations, 1)?;
                        assign_variant_field!(v1 => DAE::Element::VAR; binding = Some(e.clone()));
                        is = metamodelica::cons(i, is.clone());
                        Ok((v1.clone(), e.clone(), i.clone(), is.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                e = __wb0;
                i = __wb1;
                is = __wb2;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok(v.clone())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        };
        vars1 = metamodelica::cons(v1.clone(), vars1);
    }
    vars1 = vars1.reverse();
    equations1 = List::deletePositions(equations.clone(), is.clone(), false)?;
    equations1 = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
        for mut eq in (equations).into_iter().cloned() {
            let __x = DAEUtil::moveElementToInitialSection(eq.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    i = 1;
    for mut eq in &*equations1 {
        let () = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ DAE::Element::INITIALEQUATION { exp1: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { .. }, .. }, exp2: Deref @ DAE::Exp::CALL { path, .. }, .. } if (metamodelica::stringEq(&(AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path))), &(literal!("constructor")))) => {
                is = metamodelica::cons(i, is);
                ()
            },
            Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, source: __eq_source, .. } => {
                cr = (*__esc_cr).clone();
                vars1 = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
            for mut v in (vars1).into_iter().cloned() {
                let __x = (::match_deref::match_deref! { match &(v.clone()) {
            Deref @ DAE::Element::VAR { binding: Some(_), componentRef: __v_componentRef, .. } if (ComponentReferenceBasics::crefPrefixOf(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&__v_componentRef))?) => {
                is = metamodelica::cons(i, is.clone());
                v.clone()
            },
            Deref @ DAE::Element::VAR { binding: None, componentRef: __v_componentRef, variableAttributesOption: __v_variableAttributesOption, .. } if (ComponentReferenceBasics::crefPrefixOf(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&__v_componentRef))?) => {
                assign_variant_field!(v => DAE::Element::VAR; variableAttributesOption = DAEUtil::setFixedAttr(__v_variableAttributesOption.clone(), Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })))?);
                Error::addSourceMessage(&(Error::MOVING_PARAMETER_BINDING_TO_INITIAL_EQ_SECTION.clone()), list![ComponentReferenceBasics::printComponentRefStr(&(var_field!((*v).componentRef, DAE::Element::VAR).clone()))?], &__eq_source.info)?;
                v.clone()
            },
            _ => v.clone(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        i = i + 1;
    }
    equations1 = List::deletePositions(equations1, is, false)?;
    outVarsDae = DAE::DAElist {
        elementLst: listAppend(equations1, vars1),
    };
    Ok(outVarsDae)
}

fn findCorrespondingBinding(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inEquations: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut i: i32,
) -> Result<(metamodelica::Ref<DAE::Exp>, i32)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut i: i32 = i;
    outExp = (::match_deref::match_deref! { match &((inCref, &**inEquations)) {
        (cref, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cref2, exp: e, .. }, tail: _ }) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&cref2))?) => {
            e.clone()
        },
        (cref, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cref2, ty: _ }, scalar: e, .. }, tail: _ }) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&cref2))?) => {
            e.clone()
        },
        (cref, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1: cref2, cr2: cref3, .. }, tail: _ }) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&cref2))?) => {
            Expression::crefExp(cref3.clone())?
        },
        (cref, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: Deref @ DAE::Exp::CREF { componentRef: cref2, ty: _ }, rhs: e, .. }, tail: _ }) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cref), metamodelica::AsArg::as_arg(&cref2))?) => {
            e.clone()
        },
        (cref, Deref @ metamodelica::ListNode::Cons { head: _, tail: equations }) => {
            (outExp, i) = findCorrespondingBinding(cref.clone(), metamodelica::AsArg::as_arg(&equations), i + 1)?;
            outExp
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outExp, i))
}

pub(crate) fn isPartial(mut partialPrefix: SCode::Partial, mut mods: &metamodelica::Ref<DAE::Mod>) -> SCode::Partial {
    let mut outPartial: SCode::Partial;
    outPartial = (::match_deref::match_deref! { match &((partialPrefix, &**mods)) {
        (SCode::Partial::PARTIAL { .. }, Deref @ DAE::Mod::NOMOD { .. }) => openmodelica_frontend_types::SCode::Partial::PARTIAL,
        _ => openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outPartial
}

pub(crate) fn isFunctionInput(mut classState: &ClassInf::State, mut direction: Absyn::Direction) -> bool {
    let mut functionInput: bool;
    functionInput = (match (classState.clone(), direction) {
        (ClassInf::State::FUNCTION { .. }, Absyn::Direction::INPUT { .. }) => true,
        _ => false,
    });
    functionInput
}

pub(crate) fn extractComment(
    mut elts: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::Ref<SCode::Comment>> {
    let mut cmt: metamodelica::Ref<SCode::Comment> = metamodelica::Ref::new(SCode::Comment {
        annotation_: None,
        comment: None,
    });
    for mut elt in &**elts {
        let () = (match &*elt.clone() {
            DAE::Element::COMMENT { cmt: __esc_cmt } => {
                cmt = (*__esc_cmt).clone();
                return Ok(cmt.clone());
                return Err("fail");
            }
            _ => (),
        });
    }
    Ok(cmt)
}

fn mergeClassComments(
    mut comment1: &metamodelica::Ref<SCode::Comment>,
    mut comment2: &metamodelica::Ref<SCode::Comment>,
) -> Result<metamodelica::Ref<SCode::Comment>> {
    let mut outComment: metamodelica::Ref<SCode::Comment>;
    outComment = (::match_deref::match_deref! { match (comment1, comment2) {
        (Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: mods1, info, .. } }), comment: str1 }, Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: mods2, .. } }), comment: str2 }) => {
            let mut r#str: Option<ArcStr>;
            let mut mods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
            r#str = if ((str1).is_some()) {str1.clone()} else {str2.clone()};
            mods = listAppend(mods1.clone(), mods2.clone());
            metamodelica::Ref::new(SCode::Comment { annotation_: Some(metamodelica::Ref::new(SCode::Annotation { modification: metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: mods, binding: None, comment: None, info: info.clone() }) })), comment: r#str })
        },
        (Deref @ SCode::Comment { annotation_: ann1, comment: str1 }, Deref @ SCode::Comment { annotation_: ann2, comment: str2 }) => {
            let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
            let mut r#str: Option<ArcStr>;
            r#str = if ((str1).is_some()) {str1.clone()} else {str2.clone()};
            ann = if ((ann1).is_some()) {ann1.clone()} else {ann2.clone()};
            metamodelica::Ref::new(SCode::Comment { annotation_: ann, comment: r#str })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outComment)
}

pub(crate) fn makeNonExpSubscript(
    mut inSubscript: metamodelica::Ref<DAE::Subscript>,
) -> Result<metamodelica::Ref<DAE::Subscript>> {
    let mut outSubscript: metamodelica::Ref<DAE::Subscript>;
    outSubscript = (::match_deref::match_deref! { match &(inSubscript) {
        Deref @ DAE::Subscript::INDEX { exp: e } => {
            metamodelica::Ref::new(DAE::Subscript::WHOLE_NONEXP { exp: e.clone() })
        },
        subscript @ Deref @ DAE::Subscript::WHOLE_NONEXP { exp: _ } => {
            subscript.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outSubscript)
}

fn getFunctionAttributes(
    mut cl: &metamodelica::Ref<SCode::Element>,
    mut vl: metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inheritedComment: &metamodelica::Ref<SCode::Comment>,
) -> Result<DAE::FunctionAttributes> {
    let mut attr: DAE::FunctionAttributes;
    let mut restriction: SCode::Restriction;
    let mut fres: SCode::FunctionRestriction;
    let mut isOpenModelicaPure: bool = false;
    let mut isImpure: bool = false;
    let mut hasOutVars: bool = false;
    let mut unboxArgs: bool = false;
    let mut noReturn: DAE::NoReturn;
    let mut isBuiltin: DAE::FunctionBuiltin = DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR;
    let mut inlineType: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
    let mut name: ArcStr = arcstr::literal!("");
    let mut inVars: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
    let mut outVars: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
    let mut purity: Absyn::FunctionPurity;
    let mut daePurity: DAE::Purity;
    restriction = SCodeUtil::getClassRestriction(cl)?;
    let SCode::Restriction::R_FUNCTION {
        functionRestriction: __pa0,
    } = (restriction)
    else {
        return Err("pattern mismatch");
    };
    fres = metamodelica::Own::own(__pa0);
    daePurity = InstBasics::getFunctionRestrictionPurity(
        SCodeUtil::getFunctionRestrictionPurity(fres),
        inheritedComment,
        false,
    );
    noReturn =
        if (SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment, &(literal!("__OpenModelica_NoReturn")))) {
            DAE::NoReturn::NORETURN.clone()
        } else {
            DAE::NoReturn::RETURNS.clone()
        };
    attr = 'mc: {
        let __mc_input = fres;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: mut purity } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut inVars: metamodelica::List<metamodelica::Ref<DAE::Var>> = inVars.clone();
            let mut inlineType: DAE::InlineType = inlineType.clone();
            let mut isImpure: bool = isImpure.clone();
            let mut name: ArcStr = name.clone();
            let mut outVars: metamodelica::List<metamodelica::Ref<DAE::Var>> = outVars.clone();
            let mut unboxArgs: bool = unboxArgs.clone();
            isImpure = AbsynUtil::isImpure(purity, false);
            inVars = List::select(
                vl.clone(),
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Types::isInputVar(&__a0))
                })
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Var>) -> Result<bool> + 'static>),
            )?;
            outVars = List::select(
                vl.clone(),
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Types::isOutputVar(&__a0))
                })
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Var>) -> Result<bool> + 'static>),
            )?;
            name = SCodeUtil::isBuiltinFunction(
                cl,
                List::map(
                    inVars.clone(),
                    &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(TypesDump::getVarName(&__a0))
                    },
                )?,
                &(List::map(
                    outVars.clone(),
                    &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(TypesDump::getVarName(&__a0))
                    },
                )?),
            )?;
            inlineType = InstBasics::commentIsInlineFunc(inheritedComment);
            unboxArgs = SCodeUtil::commentHasBooleanNamedAnnotation(
                inheritedComment,
                &(literal!("__OpenModelica_UnboxArguments")),
            );
            Ok((
                DAE::FunctionAttributes {
                    inline: inlineType,
                    generateEvents: false,
                    purity: daePurity,
                    isFunctionPointer: false,
                    isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN {
                        name: Some(name.clone()),
                        unboxArgs: unboxArgs,
                    },
                    functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_NON_PARALLEL,
                    noReturn: noReturn,
                },
                inVars.clone(),
                inlineType.clone(),
                isImpure.clone(),
                name.clone(),
                outVars.clone(),
                unboxArgs.clone(),
            ))
        })() {
            inVars = __wb0;
            inlineType = __wb1;
            isImpure = __wb2;
            name = __wb3;
            outVars = __wb4;
            unboxArgs = __wb5;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut inVars: metamodelica::List<metamodelica::Ref<DAE::Var>> = inVars.clone();
            let mut inlineType: DAE::InlineType = inlineType.clone();
            let mut isOpenModelicaPure: bool = isOpenModelicaPure.clone();
            let mut name: ArcStr = name.clone();
            let mut outVars: metamodelica::List<metamodelica::Ref<DAE::Var>> = outVars.clone();
            let mut unboxArgs: bool = unboxArgs.clone();
            inVars = List::select(
                vl.clone(),
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Types::isInputVar(&__a0))
                })
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Var>) -> Result<bool> + 'static>),
            )?;
            outVars = List::select(
                vl.clone(),
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Types::isOutputVar(&__a0))
                })
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Var>) -> Result<bool> + 'static>),
            )?;
            name = SCodeUtil::isBuiltinFunction(
                cl,
                List::map(
                    inVars.clone(),
                    &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(TypesDump::getVarName(&__a0))
                    },
                )?,
                &(List::map(
                    outVars.clone(),
                    &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(TypesDump::getVarName(&__a0))
                    },
                )?),
            )?;
            inlineType = InstBasics::commentIsInlineFunc(inheritedComment);
            isOpenModelicaPure =
                !(SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment, &(literal!("__OpenModelica_Impure"))));
            unboxArgs = SCodeUtil::commentHasBooleanNamedAnnotation(
                inheritedComment,
                &(literal!("__OpenModelica_UnboxArguments")),
            );
            Ok((
                DAE::FunctionAttributes {
                    inline: inlineType,
                    generateEvents: false,
                    purity: daePurity,
                    isFunctionPointer: false,
                    isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN {
                        name: Some(name.clone()),
                        unboxArgs: unboxArgs,
                    },
                    functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_PARALLEL_FUNCTION,
                    noReturn: noReturn,
                },
                inVars.clone(),
                inlineType.clone(),
                isOpenModelicaPure.clone(),
                name.clone(),
                outVars.clone(),
                unboxArgs.clone(),
            ))
        })() {
            inVars = __wb0;
            inlineType = __wb1;
            isOpenModelicaPure = __wb2;
            name = __wb3;
            outVars = __wb4;
            unboxArgs = __wb5;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut inlineType: DAE::InlineType = inlineType.clone();
            let mut isBuiltin: DAE::FunctionBuiltin = isBuiltin.clone();
            let mut isOpenModelicaPure: bool = isOpenModelicaPure.clone();
            inlineType = InstBasics::commentIsInlineFunc(inheritedComment);
            isBuiltin = if (SCodeUtil::commentHasBooleanNamedAnnotation(
                inheritedComment,
                &(literal!("__OpenModelica_BuiltinPtr")),
            )) {
                openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR
            } else {
                openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN
            };
            isOpenModelicaPure =
                !(SCodeUtil::commentHasBooleanNamedAnnotation(inheritedComment, &(literal!("__OpenModelica_Impure"))));
            Ok((
                DAE::FunctionAttributes {
                    inline: inlineType,
                    generateEvents: false,
                    purity: daePurity,
                    isFunctionPointer: false,
                    isBuiltin: isBuiltin.clone(),
                    functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_PARALLEL_FUNCTION,
                    noReturn: noReturn,
                },
                inlineType.clone(),
                isBuiltin.clone(),
                isOpenModelicaPure.clone(),
            ))
        })() {
            inlineType = __wb0;
            isBuiltin = __wb1;
            isOpenModelicaPure = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_KERNEL_FUNCTION { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(DAE::FunctionAttributes {
                inline: openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
                generateEvents: false,
                purity: daePurity,
                isFunctionPointer: false,
                isBuiltin: openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN,
                functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_KERNEL_FUNCTION,
                noReturn: noReturn,
            })
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut daePurity: DAE::Purity = daePurity.clone();
            let mut hasOutVars: bool = hasOutVars.clone();
            let mut inlineType: DAE::InlineType = inlineType.clone();
            let mut isBuiltin: DAE::FunctionBuiltin = isBuiltin.clone();
            inlineType = InstBasics::commentIsInlineFunc(inheritedComment);
            hasOutVars = List::any(
                &vl,
                &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Types::isOutputVar(&__a0))
                },
            )?;
            isBuiltin = if (SCodeUtil::commentHasBooleanNamedAnnotation(
                inheritedComment,
                &(literal!("__OpenModelica_BuiltinPtr")),
            )) {
                openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR
            } else {
                openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN
            };
            if daePurity == DAE::Purity::UNDEFINED.clone()
                && SCodeUtil::isExternalFunctionRestriction(fres)
                && !(hasOutVars || Config::languageStandardAtLeast(Config::LanguageStandard::_3_3.clone())?)
            {
                daePurity = DAE::Purity::IMPURE.clone();
            }
            Ok((
                DAE::FunctionAttributes {
                    inline: inlineType,
                    generateEvents: false,
                    purity: daePurity,
                    isFunctionPointer: false,
                    isBuiltin: isBuiltin.clone(),
                    functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_NON_PARALLEL,
                    noReturn: noReturn,
                },
                daePurity.clone(),
                hasOutVars.clone(),
                inlineType.clone(),
                isBuiltin.clone(),
            ))
        })() {
            daePurity = __wb0;
            hasOutVars = __wb1;
            inlineType = __wb2;
            isBuiltin = __wb3;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(attr)
}

pub(crate) fn checkFunctionElement(
    mut elt: metamodelica::Ref<DAE::Element>,
    mut isExternal: bool,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((elt.clone(), isExternal)) {
        (Deref @ DAE::Element::VAR { .. }, _) => {
            ()
        },
        (Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp: Deref @ DAE::Exp::METARECORDCALL { .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, _) => {
            ()
        },
        (Deref @ DAE::Element::ALGORITHM { .. }, false) => {
            ()
        },
        (Deref @ DAE::Element::COMMENT { .. }, _) => {
            ()
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = DAEDump::dumpElementsStr(&(list![elt]))?;
            Error::addSourceMessage(&(Error::FUNCTION_ELEMENT_WRONG_KIND.clone()), list![r#str], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn printElementAndModList(
    mut inLstElAndMod: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = (::match_deref::match_deref! { match inLstElAndMod {
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        Deref @ metamodelica::ListNode::Cons { head: (e, m), tail: rest } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut s: ArcStr;
            s1 = SCodeDump::unparseElementStr(e.clone(), SCodeDump::defaultOptions.clone())?;
            s2 = Mod::printModStr(metamodelica::AsArg::as_arg(&m))?;
            s3 = printElementAndModList(rest)?;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Element:\n")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!("\nModifier: ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*s3); ArcStr::from(__mm_s) };
            s
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStr)
}

fn splitClassDefsAndComponents(
    mut inLstElAndMod: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) -> Result<(
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
)> {
    let mut outClassDefs: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    let mut outComponentDefs: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    (outClassDefs, outComponentDefs) = (::match_deref::match_deref! { match inLstElAndMod {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: (e @ Deref @ SCode::Element::COMPONENT { .. }, m), tail: rest } => {
            let mut clsdefs: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
            let mut compdefs: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
            (clsdefs, compdefs) = splitClassDefsAndComponents(rest)?;
            (clsdefs, metamodelica::cons((e.clone(), m.clone()), compdefs))
        },
        Deref @ metamodelica::ListNode::Cons { head: (e, m), tail: rest } => {
            let mut clsdefs: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
            let mut compdefs: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
            (clsdefs, compdefs) = splitClassDefsAndComponents(rest)?;
            (metamodelica::cons((e.clone(), m.clone()), clsdefs), compdefs)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClassDefs, outComponentDefs))
}

pub(crate) fn selectModifiers(
    mut fromMerging: metamodelica::Ref<DAE::Mod>,
    mut fromRedeclareType: metamodelica::Ref<DAE::Mod>,
    mut typePath: &metamodelica::Ref<Absyn::Path>,
) -> (metamodelica::Ref<DAE::Mod>, metamodelica::Ref<DAE::Mod>) {
    let mut bindingMod: metamodelica::Ref<DAE::Mod>;
    let mut classMod: metamodelica::Ref<DAE::Mod>;
    bindingMod = if (redeclareBasicType(&fromMerging)) {
        fromRedeclareType.clone()
    } else {
        fromMerging
    };
    classMod = fromRedeclareType;
    (bindingMod, classMod)
}

pub(crate) fn redeclareBasicType(mut r#mod: &metamodelica::Ref<DAE::Mod>) -> bool {
    let mut isRedeclareOfBasicType: bool;
    isRedeclareOfBasicType = 'mc: {
        let __mc_input = &**r#mod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path, .. }, .. }, .. } => {
                    let mut name: ArcStr;
                    let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
                    name = AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&path));
                    let true = (listMember(name.clone(), list![literal!("Real"), literal!("Integer"), literal!("Boolean"), literal!("String"), literal!("Clock")])) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path, .. }, .. }, .. } => {
                    let mut name: ArcStr;
                    let false = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
                    name = AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&path));
                    let true = (listMember(name.clone(), list![literal!("Real"), literal!("Integer"), literal!("Boolean"), literal!("String")])) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    isRedeclareOfBasicType
}

pub(crate) fn optimizeFunctionCheckForLocals<'__b>(
    mut path: &'__b metamodelica::Ref<Absyn::Path>,
    mut inElts: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut oalg: Option<metamodelica::Ref<DAE::Element>>,
    mut acc: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut invars: metamodelica::List<ArcStr>,
    mut outvars: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inElts, oalg.clone())) {
            (Deref @ metamodelica::ListNode::Nil, None) => {
                return Ok(acc.reverse())
            },
            (Deref @ metamodelica::ListNode::Nil, Some(Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, source })) => {
                let mut stmts = (*stmts).clone();
                stmts = optimizeLastStatementTail(path, metamodelica::AsArg::as_arg(&stmts), &(invars.reverse()), &(outvars.reverse()), metamodelica::nil())?;
                return Ok(metamodelica::cons(metamodelica::Ref::new(DAE::Element::ALGORITHM { algorithm_: metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts.clone() }), source: source.clone() }), acc).reverse())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::ListNode::Nil }, .. }, tail: elts }, _) => {
                { (path, inElts, oalg, acc, invars, outvars) = (path, elts.clone(), oalg, acc, invars, outvars); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: elt1 @ Deref @ DAE::Element::ALGORITHM { source, .. }, tail: elts }, Some(elt2)) => {
                let mut r#str: ArcStr;
                r#str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
                if !(Config::acceptMetaModelicaGrammar()?) {
                    Error::addSourceMessage(&(Error::FUNCTION_MULTIPLE_ALGORITHM.clone()), list![r#str], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                }
                { (path, inElts, oalg, acc, invars, outvars) = (path, elts.clone(), Some(elt1.clone()), metamodelica::cons(elt2.clone(), acc), invars, outvars); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: elt @ Deref @ DAE::Element::ALGORITHM { .. }, tail: elts }, None) => {
                { (path, inElts, oalg, acc, invars, outvars) = (path, elts.clone(), Some(elt.clone()), acc, invars, outvars); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: elt @ Deref @ DAE::Element::VAR { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, direction: DAE::VarDirection::OUTPUT { .. }, .. }, tail: elts }, _) => {
                { (path, inElts, oalg, acc, invars, outvars) = (path, elts.clone(), oalg, metamodelica::cons(elt.clone(), acc), invars, metamodelica::cons(name.clone(), outvars)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: elt @ Deref @ DAE::Element::VAR { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, direction: DAE::VarDirection::INPUT { .. }, .. }, tail: elts }, _) => {
                { (path, inElts, oalg, acc, invars, outvars) = (path, elts.clone(), oalg, metamodelica::cons(elt.clone(), acc), metamodelica::cons(name.clone(), invars), outvars); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: elt, tail: elts }, _) => {
                { (path, inElts, oalg, acc, invars, outvars) = (path, elts.clone(), oalg, metamodelica::cons(elt.clone(), acc), invars, outvars); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn optimizeLastStatementTail<'__b>(
    mut path: &'__b metamodelica::Ref<Absyn::Path>,
    mut inStmts: &'__b metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut invars: &'__b metamodelica::List<ArcStr>,
    mut outvars: &'__b metamodelica::List<ArcStr>,
    mut acc: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inStmts {
            Deref @ metamodelica::ListNode::Cons { head: stmt, tail: Deref @ metamodelica::ListNode::Nil } => {
                let mut stmt = (*stmt).clone();
                stmt = optimizeStatementTail(path.clone(), stmt.clone(), invars, outvars.clone());
                return Ok(metamodelica::cons(stmt.clone(), acc).reverse())
            },
            Deref @ metamodelica::ListNode::Cons { head: stmt, tail: stmts } => {
                { (path, inStmts, invars, outvars, acc) = (path, stmts, invars, outvars, metamodelica::cons(stmt.clone(), acc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn optimizeStatementTail(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut inStmt: metamodelica::Ref<DAE::Statement>,
    mut invars: &metamodelica::List<ArcStr>,
    mut outvars: metamodelica::List<ArcStr>,
) -> metamodelica::Ref<DAE::Statement> {
    let mut ostmt: metamodelica::Ref<DAE::Statement>;
    ostmt = 'mc: {
        let __mc_input = (&*inStmt, &*outvars);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN { type_: tp, exp1: lhs, exp: rhs, source }, _) => {
                    let mut name: ArcStr;
                    let mut stmt: metamodelica::Ref<DAE::Statement>;
                    let mut rhs = (*rhs).clone();
                    name = Expression::simpleCrefName(metamodelica::AsArg::as_arg(&lhs))?;
                    rhs = optimizeStatementTail2(path.clone(), rhs.clone(), list![name.clone()], invars, outvars.clone(), metamodelica::AsArg::as_arg(&source))?;
                    stmt = if (Expression::isTailCall(metamodelica::AsArg::as_arg(&rhs))) {metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() })} else {metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: lhs.clone(), exp: rhs.clone(), source: source.clone() })};
                    Ok(stmt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp, expExpLst: lhsLst, exp: rhs, source }, _) => {
                    let mut lhsNames: metamodelica::List<ArcStr>;
                    let mut stmt: metamodelica::Ref<DAE::Statement>;
                    let mut rhs = (*rhs).clone();
                    lhsNames = List::map(lhsLst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::simpleCrefName(&__a0))?;
                    rhs = optimizeStatementTail2(path.clone(), rhs.clone(), lhsNames.clone(), invars, outvars.clone(), metamodelica::AsArg::as_arg(&source))?;
                    stmt = if (Expression::isTailCall(metamodelica::AsArg::as_arg(&rhs))) {metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() })} else {metamodelica::Ref::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp.clone(), expExpLst: lhsLst.clone(), exp: rhs.clone(), source: source.clone() })};
                    Ok(stmt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { exp: cond, statementLst: stmts, else_, source }, _) => {
                    let mut stmts = (*stmts).clone();
                    let mut else_ = (*else_).clone();
                    stmts = optimizeLastStatementTail(&path, metamodelica::AsArg::as_arg(&stmts), invars, &outvars, metamodelica::nil())?;
                    else_ = optimizeElseTail(&path, metamodelica::AsArg::as_arg(&else_), invars, &outvars);
                    Ok(metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: cond.clone(), statementLst: stmts.clone(), else_: else_.clone(), source: source.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_NORETCALL { exp: rhs, source }, Deref @ metamodelica::ListNode::Nil) => {
                    let mut stmt: metamodelica::Ref<DAE::Statement>;
                    let mut rhs = (*rhs).clone();
                    rhs = optimizeStatementTail2(path.clone(), rhs.clone(), metamodelica::nil(), invars, metamodelica::nil(), metamodelica::AsArg::as_arg(&source))?;
                    stmt = metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() });
                    Ok(stmt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inStmt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ostmt
}

fn optimizeElseTail(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut inElse: &metamodelica::Ref<DAE::Else>,
    mut invars: &metamodelica::List<ArcStr>,
    mut outvars: &metamodelica::List<ArcStr>,
) -> metamodelica::Ref<DAE::Else> {
    let mut outElse: metamodelica::Ref<DAE::Else>;
    outElse = 'mc: {
        let __mc_input = &**inElse;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Else::ELSEIF { exp: cond, statementLst: stmts, else_ } => {
                    let mut stmts = (*stmts).clone();
                    let mut else_ = (*else_).clone();
                    stmts = optimizeLastStatementTail(path, metamodelica::AsArg::as_arg(&stmts), invars, outvars, metamodelica::nil())?;
                    else_ = optimizeElseTail(path, metamodelica::AsArg::as_arg(&else_), invars, outvars);
                    Ok(metamodelica::Ref::new(DAE::Else::ELSEIF { exp: cond.clone(), statementLst: stmts.clone(), else_: else_.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Else::ELSE { statementLst: stmts } => {
                    let mut stmts = (*stmts).clone();
                    stmts = optimizeLastStatementTail(path, metamodelica::AsArg::as_arg(&stmts), invars, outvars, metamodelica::nil())?;
                    Ok(metamodelica::Ref::new(DAE::Else::ELSE { statementLst: stmts.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inElse.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outElse
}

fn optimizeStatementTail2(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut lhsVars: metamodelica::List<ArcStr>,
    mut invars: &metamodelica::List<ArcStr>,
    mut outvars: metamodelica::List<ArcStr>,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut orhs: metamodelica::Ref<DAE::Exp>;
    let true = (lhsVars.clone() == outvars) else {
        return Err("pattern mismatch");
    };
    let __pa0 = ::match_deref::match_deref! { match &(optimizeStatementTail3(path, rhs, invars, &lhsVars, source)) {
        (__pa0, true) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    orhs = metamodelica::Own::own(__pa0);
    Ok(orhs)
}

fn optimizeStatementTail3(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut vars: &metamodelica::List<ArcStr>,
    mut lhsVars: &metamodelica::List<ArcStr>,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut orhs: metamodelica::Ref<DAE::Exp>;
    let mut isTailRecursive: bool;
    (orhs, isTailRecursive) = 'mc: {
        let __mc_input = (path.clone(), rhs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path1, call @ Deref @ DAE::Exp::CALL { path: path2, attr: attr @ Deref @ DAE::CallAttributes { tailCall: DAE::TailCall::NO_TAIL { .. }, .. }, .. }) => {
                    let mut r#str: ArcStr;
                    let mut call = (*call).clone();
                    let mut attr = (*attr).clone();
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&path1), metamodelica::AsArg::as_arg(&path2))) else { return Err("pattern mismatch") };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tail recursion of: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?); __mm_s.push_str(&*literal!(" with input vars: ")); __mm_s.push_str(&*stringDelimitList(vars.clone(), literal!(","))); ArcStr::from(__mm_s) };
                    if Flags::isSet(Flags::TAIL.clone())? {
                        Error::addSourceMessage(&(Error::COMPILER_NOTIFICATION.clone()), list![r#str.clone()], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                    }
                    assign_field!(attr.tailCall = DAE::TailCall::TAIL { vars: vars.clone(), outVars: lhsVars.clone() });
                    assign_variant_field!(call => DAE::Exp::CALL; attr = attr.clone());
                    Ok((call.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 }) => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut e2 = (*e2).clone();
                    let mut e3 = (*e3).clone();
                    (e2, b1) = optimizeStatementTail3(path.clone(), e2.clone(), vars, lhsVars, source);
                    (e3, b2) = optimizeStatementTail3(path.clone(), e3.clone(), vars, lhsVars, source);
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: e2.clone(), expElse: e3.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::MATCHEXPRESSION { matchType: matchType @ DAE::MatchType::MATCH { switch: _ }, inputs, aliases, localDecls, cases, et }) => {
                    let mut cases = (*cases).clone();
                    cases = optimizeStatementTailMatchCases(&path, metamodelica::AsArg::as_arg(&cases), false, metamodelica::nil(), vars, lhsVars, source)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::MATCHEXPRESSION { matchType: matchType.clone(), inputs: inputs.clone(), aliases: aliases.clone(), localDecls: localDecls.clone(), cases: cases.clone(), et: et.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((rhs.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (orhs, isTailRecursive)
}

fn optimizeStatementTailMatchCases(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut inCases: &metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut changed: bool,
    mut inAcc: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut vars: &metamodelica::List<ArcStr>,
    mut lhsVars: &metamodelica::List<ArcStr>,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::MatchCase>>> {
    let mut ocases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
    ocases = 'mc: {
        let __mc_input = (&**inCases, changed, inAcc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, true, acc) => {
                    Ok(acc.clone().reverse())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns, patternGuard, localDecls, body, result: Some(exp), resultInfo, jump, info }, tail: cases }, _, acc) => {
                    let mut case_: metamodelica::Ref<DAE::MatchCase>;
                    let mut exp = (*exp).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(optimizeStatementTail3(path.clone(), exp.clone(), vars, lhsVars, source)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp = metamodelica::Own::own(__pa0);
                    case_ = metamodelica::Ref::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: patternGuard.clone(), localDecls: localDecls.clone(), body: body.clone(), result: Some(exp.clone()), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info.clone() });
                    Ok(optimizeStatementTailMatchCases(path, metamodelica::AsArg::as_arg(&cases), true, metamodelica::cons(case_.clone(), acc.clone()), vars, lhsVars, source)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns, patternGuard, localDecls, body, result: Some(Deref @ DAE::Exp::TUPLE { PR: Deref @ metamodelica::ListNode::Nil }), resultInfo, jump, info }, tail: cases }, _, acc) => {
                    let mut case_: metamodelica::Ref<DAE::MatchCase>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut sourceStmt: metamodelica::Ref<DAE::ElementSource>;
                    let mut body = (*body).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::last(metamodelica::AsArg::as_arg(&body))?) {
                        Deref @ DAE::Statement::STMT_NORETCALL { exp: __pa0, source: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp = metamodelica::Own::own(__pa0);
                    sourceStmt = metamodelica::Own::own(__pa1);
                    let __pa2 = ::match_deref::match_deref! { match &(optimizeStatementTail3(path.clone(), exp.clone(), vars, lhsVars, source)) {
                        (__pa2, true) => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp = metamodelica::Own::own(__pa2);
                    body = List::set(body.clone(), ((body).len() as i32), metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: exp.clone(), source: sourceStmt.clone() }))?;
                    case_ = metamodelica::Ref::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: patternGuard.clone(), localDecls: localDecls.clone(), body: body.clone(), result: Some(metamodelica::Ref::new(DAE::Exp::TUPLE { PR: metamodelica::nil() })), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info.clone() });
                    Ok(optimizeStatementTailMatchCases(path, metamodelica::AsArg::as_arg(&cases), true, metamodelica::cons(case_.clone(), acc.clone()), vars, lhsVars, source)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: case_, tail: cases }, _, acc) => {
                    Ok(optimizeStatementTailMatchCases(path, metamodelica::AsArg::as_arg(&cases), changed, metamodelica::cons(case_.clone(), acc.clone()), vars, lhsVars, source)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(ocases)
}

pub(crate) fn pushStructuralParameters(mut cache: FCore::Cache) -> FCore::Cache {
    let mut ocache: FCore::Cache;
    ocache = (::match_deref::match_deref! { match &(cache.clone()) {
        FCore::Cache::CACHE { initialGraph: ie, functions: f, evaluatedParams: (ht, crs), modelName: p } => {
            FCore::Cache::CACHE { initialGraph: ie.clone(), functions: f.clone(), evaluatedParams: (ht.clone(), metamodelica::cons(metamodelica::nil(), crs.clone())), modelName: p.clone() }
        },
        _ => {
            cache
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ocache
}

pub(crate) fn popStructuralParameters(mut cache: FCore::Cache, mut pre: DAE::Prefix) -> Result<FCore::Cache> {
    let mut ocache: FCore::Cache;
    ocache = (::match_deref::match_deref! { match &(cache.clone()) {
        FCore::Cache::CACHE { initialGraph: ie, functions: f, evaluatedParams: (ht, Deref @ metamodelica::ListNode::Cons { head: crs, tail: crss }), modelName: p } => {
            let mut ht = (*ht).clone();
            ht = prefixAndAddCrefsToHt(cache, ht.clone(), pre, metamodelica::AsArg::as_arg(&crs))?;
            FCore::Cache::CACHE { initialGraph: ie.clone(), functions: f.clone(), evaluatedParams: (ht.clone(), crss.clone()), modelName: p.clone() }
        },
        FCore::Cache::CACHE { initialGraph: _, functions: _, evaluatedParams: (_, Deref @ metamodelica::ListNode::Nil), modelName: _ } => {
            cache
        },
        FCore::Cache::NO_CACHE { .. } => {
            cache
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(ocache)
}

fn prefixAndAddCrefsToHt(
    mut cache: FCore::Cache,
    mut set: metamodelica::Ref<AvlSetCR::Tree>,
    mut pre: DAE::Prefix,
    mut icrs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::Ref<AvlSetCR::Tree>> {
    let mut set: metamodelica::Ref<AvlSetCR::Tree> = set;
    for mut cr in &**icrs {
        let mut cr = cr.clone();
        (_, cr) = PrefixUtil::prefixCref(
            cache.clone(),
            FGraph::empty(),
            &(InnerOuter::emptyInstHierarchy().clone()),
            pre.clone(),
            cr,
        )?;
        set = AvlSetCR::add(set, &cr)?;
    }
    Ok(set)
}

fn numStructuralParameterScopes(mut cache: FCore::Cache) -> Result<i32> {
    let mut i: i32;
    let mut lst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let FCore::CACHE {
        evaluatedParams: (_, __pa0),
        ..
    } = (cache)
    else {
        return Err("pattern mismatch");
    };
    lst = metamodelica::Own::own(__pa0);
    i = ((lst).len() as i32);
    Ok(i)
}

pub(crate) fn functionAlwaysFails(mut elts: &metamodelica::List<metamodelica::Ref<DAE::Element>>) -> Result<bool> {
    let mut alwaysFails: bool;
    let mut sections: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>> = metamodelica::nil();
    for mut e in &**elts {
        let _ = (::match_deref::match_deref! { match &(e.clone()) {
            Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: ss }, .. } => {
                sections = metamodelica::cons(ss.clone(), sections);
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    alwaysFails = stmtsAlwaysFail(&(List::flatten(sections.reverse())?));
    Ok(alwaysFails)
}

fn stmtsAlwaysFail(mut stmts: &metamodelica::List<metamodelica::Ref<DAE::Statement>>) -> bool {
    let mut alwaysFails: bool = false;
    for mut s in &**stmts {
        if statementGuaranteesFail(metamodelica::AsArg::as_arg(&s)) {
            alwaysFails = true;
            return alwaysFails;
        } else if !(statementIsTransparent(metamodelica::AsArg::as_arg(&s))) {
            return alwaysFails;
        }
    }
    alwaysFails
}

fn statementGuaranteesFail(mut s: &metamodelica::Ref<DAE::Statement>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match s {
        Deref @ DAE::Statement::STMT_NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fail" }, expLst: Deref @ metamodelica::ListNode::Nil, .. }, .. } => {
            true
        },
        Deref @ DAE::Statement::STMT_ASSERT { cond: Deref @ DAE::Exp::BCONST { bool: false }, .. } => {
            true
        },
        Deref @ DAE::Statement::STMT_TERMINATE { .. } => {
            true
        },
        Deref @ DAE::Statement::STMT_IF { statementLst: ss, else_: el, .. } => {
            let mut b1: bool;
            let mut b2: bool;
            b1 = stmtsAlwaysFail(ss);
            b2 = elseGuaranteesFail(el);
            b1 && b2
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn elseGuaranteesFail(mut inElse: &metamodelica::Ref<DAE::Else>) -> bool {
    let mut b: bool;
    b = (match &**inElse {
        DAE::Else::NOELSE { .. } => false,
        DAE::Else::ELSE { statementLst: ss } => stmtsAlwaysFail(ss),
        DAE::Else::ELSEIF {
            statementLst: ss,
            else_: el,
            ..
        } => {
            let mut b1: bool;
            let mut b2: bool;
            b1 = stmtsAlwaysFail(ss);
            b2 = elseGuaranteesFail(el);
            b1 && b2
        }
    });
    b
}

fn statementIsTransparent(mut s: &metamodelica::Ref<DAE::Statement>) -> bool {
    let mut b: bool;
    b = (match &**s {
        DAE::Statement::STMT_ASSIGN { .. } => true,
        DAE::Statement::STMT_TUPLE_ASSIGN { .. } => true,
        DAE::Statement::STMT_ASSIGN_ARR { .. } => true,
        DAE::Statement::STMT_ARRAY_INIT { .. } => true,
        DAE::Statement::STMT_NORETCALL { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn checkFunctionDefUse(
    mut elts: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = info.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            checkFunctionDefUse2(elts.clone(), None, metamodelica::nil(), metamodelica::nil(), info)?;
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addSourceMessage(
                &(Error::INTERNAL_ERROR.clone()),
                list![literal!("InstUtil.checkFunctionDefUse failed")],
                info,
            )?;
            Ok(())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn checkFunctionDefUse2<'__b>(
    mut elts: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut alg: Option<metamodelica::List<metamodelica::Ref<DAE::Statement>>>,
    mut inUnbound: metamodelica::List<ArcStr>,
    mut inOutputs: metamodelica::List<ArcStr>,
    mut inInfo: &'__b SourceInfo,
) -> Result<metamodelica::List<ArcStr>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((elts, alg.clone(), inUnbound.clone(), inOutputs.clone())) {
            (Deref @ metamodelica::ListNode::Nil, None, _, _) => {
                return Ok(inUnbound)
            },
            (Deref @ metamodelica::ListNode::Nil, Some(stmts), unbound, outputs) => {
                let mut maybeUnbound: metamodelica::List<ArcStr>;
                let mut name: ArcStr = arcstr::literal!("");
                let mut unbound = (*unbound).clone();
                (_, _, unbound, maybeUnbound) = List::fold1(metamodelica::AsArg::as_arg(&stmts), &checkFunctionDefUseStmt, false, (false, false, unbound.clone(), metamodelica::nil()))?;
                unbound = List::fold1(metamodelica::AsArg::as_arg(&outputs), &move |__a0: ArcStr, __a1: SourceInfo, __a2: metamodelica::List<ArcStr>| checkOutputDefUse(__a0, &__a1, __a2), inInfo.clone(), unbound.clone())?;
                if Flags::isSet(Flags::CHECK_DEF_USE.clone())? {
                    for mut name in &*outputs.clone() {
                        let mut name = name.clone();
                        if listMember(name.clone(), maybeUnbound.clone()) {
                            Error::addSourceMessage(&(Error::UNASSIGNED_FUNCTION_OUTPUT_UNPROVEN.clone()), list![name], inInfo)?;
                        }
                    }
                }
                return Ok(unbound.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT { .. }, .. }, tail: rest }, _, unbound, _) => {
                let mut unbound = (*unbound).clone();
                { (elts, alg, inUnbound, inOutputs, inInfo) = (rest.clone(), alg, unbound.clone(), inOutputs, inInfo); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { direction: dir, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, varLst: vars, .. }, dims, binding: None, .. }, tail: rest }, _, unbound, _) => {
                let mut outputs: metamodelica::List<ArcStr>;
                let mut names: metamodelica::List<ArcStr>;
                let mut outNames: metamodelica::List<ArcStr>;
                let mut vars = (*vars).clone();
                let mut unbound = (*unbound).clone();
                vars = List::filterOnTrue(vars.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::varIsVariable(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Var>) -> Result<bool> + 'static>))?;
                names = List::map1r(List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(TypesDump::getVarName(&__a0)) })?, &fnptr!(stringAppend, ArcStr, ArcStr), { let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) })?;
                outNames = if (DAEUtil::varDirectionEqual(dir.clone(), openmodelica_frontend_types::DAE::VarDirection::OUTPUT)) {names.clone()} else {metamodelica::nil()};
                names = if (Expression::dimensionsKnownAndNonZero(metamodelica::AsArg::as_arg(&dims))?) {names} else {metamodelica::nil()};
                unbound = listAppend(names, unbound.clone());
                outputs = listAppend(outNames, inOutputs);
                { (elts, alg, inUnbound, inOutputs, inInfo) = (rest.clone(), alg, unbound.clone(), outputs, inInfo); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { direction: dir, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, dims, binding: None, .. }, tail: rest }, _, unbound, _) => {
                let mut outputs: metamodelica::List<ArcStr>;
                let mut unbound = (*unbound).clone();
                unbound = List::consOnTrue(Expression::dimensionsKnownAndNonZero(metamodelica::AsArg::as_arg(&dims))?, name.clone(), unbound.clone());
                outputs = List::consOnTrue(DAEUtil::varDirectionEqual(dir.clone(), openmodelica_frontend_types::DAE::VarDirection::OUTPUT), name.clone(), inOutputs);
                { (elts, alg, inUnbound, inOutputs, inInfo) = (rest.clone(), alg, unbound.clone(), outputs, inInfo); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, tail: rest }, None, unbound, _) => {
                let mut unbound = (*unbound).clone();
                { (elts, alg, inUnbound, inOutputs, inInfo) = (rest.clone(), Some(stmts.clone()), unbound.clone(), inOutputs, inInfo); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, tail: rest }, Some(prevStmts), unbound, _) => {
                let mut unbound = (*unbound).clone();
                { (elts, alg, inUnbound, inOutputs, inInfo) = (rest.clone(), Some(listAppend(prevStmts.clone(), stmts.clone())), unbound.clone(), inOutputs, inInfo); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _, unbound, _) => {
                let mut unbound = (*unbound).clone();
                { (elts, alg, inUnbound, inOutputs, inInfo) = (rest.clone(), alg, unbound.clone(), inOutputs, inInfo); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn checkOutputDefUse(
    mut name: ArcStr,
    mut info: &SourceInfo,
    mut inUnbound: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outUnbound: metamodelica::List<ArcStr>;
    let mut b: bool;
    b = listMember(name.clone(), inUnbound.clone());
    Error::assertionOrAddSourceMessage(!(b), &(Error::WARNING_DEF_USE.clone()), list![name.clone()], info)?;
    outUnbound = List::filter1OnTrue(
        inUnbound,
        (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1))
        }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        name,
    )?;
    Ok(outUnbound)
}

fn checkFunctionDefUseStmt(
    mut inStmt: metamodelica::Ref<DAE::Statement>,
    mut inLoop: bool,
    mut inUnbound: (bool, bool, metamodelica::List<ArcStr>, metamodelica::List<ArcStr>),
) -> Result<(bool, bool, metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut outUnbound: (bool, bool, metamodelica::List<ArcStr>, metamodelica::List<ArcStr>);
    outUnbound = (::match_deref::match_deref! { match &((inStmt.clone(), inUnbound.clone())) {
        (_, (true, _, _, _)) => {
            inUnbound
        },
        (_, (false, true, _, _)) => {
            let mut info: SourceInfo;
            info = ElementSource::getElementSourceFileInfo(ElementSource::getStatementSource(&inStmt)?);
            Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![literal!("InstUtil.checkFunctionDefUseStmt failed")], &info)?;
            return Err("fail")
        },
        (Deref @ DAE::Statement::STMT_ASSIGN { exp1: lhs, exp: rhs, source, .. }, (_, _, unbound, maybe)) => {
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, __pa1, _)) = Expression::traverseExpTopDown(rhs.clone(), &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info.clone()))?;
            unbound = metamodelica::Own::own(__pa0);
            maybe = metamodelica::Own::own(__pa1);
            (unbound, maybe) = traverseCrefSubs(metamodelica::AsArg::as_arg(&lhs), info, unbound.clone(), maybe.clone())?;
            unbound = crefFiltering(lhs.clone(), unbound.clone())?;
            maybe = crefFiltering(lhs.clone(), maybe.clone())?;
            (false, false, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: lhss, exp: rhs, source, .. }, (_, _, unbound, maybe)) => {
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, __pa1, _)) = Expression::traverseExpTopDown(rhs.clone(), &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info.clone()))?;
            unbound = metamodelica::Own::own(__pa0);
            maybe = metamodelica::Own::own(__pa1);
            for mut l in &*lhss.clone() {
                (unbound, maybe) = traverseCrefSubs(metamodelica::AsArg::as_arg(&l), info.clone(), unbound.clone(), maybe.clone())?;
            }
            unbound = List::fold(metamodelica::AsArg::as_arg(&lhss), &crefFiltering, unbound.clone())?;
            maybe = List::fold(metamodelica::AsArg::as_arg(&lhss), &crefFiltering, maybe.clone())?;
            (false, false, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs, exp: rhs, source, .. }, (_, _, unbound, maybe)) => {
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, __pa1, _)) = Expression::traverseExpTopDown(rhs.clone(), &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info.clone()))?;
            unbound = metamodelica::Own::own(__pa0);
            maybe = metamodelica::Own::own(__pa1);
            (unbound, maybe) = traverseCrefSubs(metamodelica::AsArg::as_arg(&lhs), info, unbound.clone(), maybe.clone())?;
            unbound = crefFiltering(lhs.clone(), unbound.clone())?;
            maybe = crefFiltering(lhs.clone(), maybe.clone())?;
            (false, false, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_IF { exp, statementLst: stmts, else_, source }, (_, _, unbound, maybe)) => {
            let mut b1: bool;
            let mut b2: bool;
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            (b1, b2, unbound, maybe) = checkFunctionDefUseElse(&(metamodelica::Ref::new(DAE::Else::ELSEIF { exp: exp.clone(), statementLst: stmts.clone(), else_: else_.clone() })), unbound.clone(), metamodelica::AsArg::as_arg(&maybe), inLoop, &info)?;
            (b1, b2, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_FOR { iter, range: exp, statementLst: stmts, source, .. }, (_, _, unbound, maybe)) => {
            let mut maybeBody: metamodelica::List<ArcStr>;
            let mut unboundBefore: metamodelica::List<ArcStr>;
            let mut b: bool;
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), iter.clone())?;
            maybe = List::filter1OnTrue(maybe.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), iter.clone())?;
            let (_, (__pa0, __pa1, _)) = Expression::traverseExpTopDown(exp.clone(), &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info))?;
            unbound = metamodelica::Own::own(__pa0);
            maybe = metamodelica::Own::own(__pa1);
            unboundBefore = unbound.clone();
            (_, b, unbound, maybeBody) = List::fold1(metamodelica::AsArg::as_arg(&stmts), &checkFunctionDefUseStmt, true, (false, false, unbound.clone(), maybe.clone()))?;
            maybe = List::unionOnTrue(metamodelica::AsArg::as_arg(&maybe), &maybeBody, &fnptr!(stringEq, ArcStr, ArcStr))?;
            maybe = List::unionOnTrue(metamodelica::AsArg::as_arg(&maybe), &(List::setDifferenceOnTrue(unboundBefore, metamodelica::AsArg::as_arg(&unbound), &fnptr!(stringEq, ArcStr, ArcStr))?), &fnptr!(stringEq, ArcStr, ArcStr))?;
            (b, b, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_PARFOR { iter, range: exp, statementLst: stmts, source, .. }, (_, _, unbound, maybe)) => {
            let mut maybeBody: metamodelica::List<ArcStr>;
            let mut unboundBefore: metamodelica::List<ArcStr>;
            let mut b: bool;
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), iter.clone())?;
            maybe = List::filter1OnTrue(maybe.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), iter.clone())?;
            let (_, (__pa0, __pa1, _)) = Expression::traverseExpTopDown(exp.clone(), &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info))?;
            unbound = metamodelica::Own::own(__pa0);
            maybe = metamodelica::Own::own(__pa1);
            unboundBefore = unbound.clone();
            (_, b, unbound, maybeBody) = List::fold1(metamodelica::AsArg::as_arg(&stmts), &checkFunctionDefUseStmt, true, (false, false, unbound.clone(), maybe.clone()))?;
            maybe = List::unionOnTrue(metamodelica::AsArg::as_arg(&maybe), &maybeBody, &fnptr!(stringEq, ArcStr, ArcStr))?;
            maybe = List::unionOnTrue(metamodelica::AsArg::as_arg(&maybe), &(List::setDifferenceOnTrue(unboundBefore, metamodelica::AsArg::as_arg(&unbound), &fnptr!(stringEq, ArcStr, ArcStr))?), &fnptr!(stringEq, ArcStr, ArcStr))?;
            (b, b, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_WHILE { exp: Deref @ DAE::Exp::BCONST { bool: true }, statementLst: stmts, .. }, (_, _, unbound, maybe)) => {
            let mut maybeBody: metamodelica::List<ArcStr>;
            let mut unboundBefore: metamodelica::List<ArcStr>;
            let mut guaranteed: metamodelica::List<ArcStr>;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            guaranteed = whileTrueBreakAssigned(metamodelica::AsArg::as_arg(&stmts), unbound.clone())?;
            unboundBefore = unbound.clone();
            (_, _, unbound, maybeBody) = List::fold1(metamodelica::AsArg::as_arg(&stmts), &checkFunctionDefUseStmt, true, (false, false, unbound.clone(), maybe.clone()))?;
            maybe = List::unionOnTrue(metamodelica::AsArg::as_arg(&maybe), &maybeBody, &fnptr!(stringEq, ArcStr, ArcStr))?;
            maybe = List::unionOnTrue(metamodelica::AsArg::as_arg(&maybe), &(List::setDifferenceOnTrue(unboundBefore, metamodelica::AsArg::as_arg(&unbound), &fnptr!(stringEq, ArcStr, ArcStr))?), &fnptr!(stringEq, ArcStr, ArcStr))?;
            maybe = List::setDifferenceOnTrue(maybe.clone(), &guaranteed, &fnptr!(stringEq, ArcStr, ArcStr))?;
            (false, false, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_WHILE { exp, statementLst: stmts, source }, (_, _, unbound, maybe)) => {
            let mut maybeBody: metamodelica::List<ArcStr>;
            let mut unboundBefore: metamodelica::List<ArcStr>;
            let mut b: bool;
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, __pa1, _)) = Expression::traverseExpTopDown(exp.clone(), &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info))?;
            unbound = metamodelica::Own::own(__pa0);
            maybe = metamodelica::Own::own(__pa1);
            unboundBefore = unbound.clone();
            (_, b, unbound, maybeBody) = List::fold1(metamodelica::AsArg::as_arg(&stmts), &checkFunctionDefUseStmt, true, (false, false, unbound.clone(), maybe.clone()))?;
            maybe = List::unionOnTrue(metamodelica::AsArg::as_arg(&maybe), &maybeBody, &fnptr!(stringEq, ArcStr, ArcStr))?;
            maybe = List::unionOnTrue(metamodelica::AsArg::as_arg(&maybe), &(List::setDifferenceOnTrue(unboundBefore, metamodelica::AsArg::as_arg(&unbound), &fnptr!(stringEq, ArcStr, ArcStr))?), &fnptr!(stringEq, ArcStr, ArcStr))?;
            (b, b, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_ASSERT { cond: Deref @ DAE::Exp::BCONST { bool: false }, .. }, _) => {
            (true, true, metamodelica::nil(), metamodelica::nil())
        },
        (Deref @ DAE::Statement::STMT_ASSERT { cond: exp1, msg: exp2, source, .. }, (_, _, unbound, maybe)) => {
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, __pa1, _)) = Expression::traverseExpTopDown(exp1.clone(), &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info.clone()))?;
            unbound = metamodelica::Own::own(__pa0);
            maybe = metamodelica::Own::own(__pa1);
            let (_, (__pa2, __pa3, _)) = Expression::traverseExpTopDown(exp2.clone(), &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info))?;
            unbound = metamodelica::Own::own(__pa2);
            maybe = metamodelica::Own::own(__pa3);
            (false, false, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_TERMINATE { msg: exp, source }, (_, _, unbound, maybe)) => {
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, __pa1, _)) = Expression::traverseExpTopDown(exp.clone(), &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info))?;
            unbound = metamodelica::Own::own(__pa0);
            maybe = metamodelica::Own::own(__pa1);
            (true, true, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fail" }, expLst: Deref @ metamodelica::ListNode::Nil, .. }, .. }, _) => {
            (true, true, metamodelica::nil(), metamodelica::nil())
        },
        (Deref @ DAE::Statement::STMT_NORETCALL { exp: Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { noReturn: DAE::NoReturn::NORETURN, .. }, .. }, .. }, _) => {
            (true, true, metamodelica::nil(), metamodelica::nil())
        },
        (Deref @ DAE::Statement::STMT_NORETCALL { exp: exp @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { tailCall: DAE::TailCall::TAIL { outVars: lhsNames, .. }, .. }, .. }, source }, (_, _, unbound, maybe)) => {
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, __pa1, _)) = Expression::traverseExpTopDown(exp.clone(), &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info))?;
            unbound = metamodelica::Own::own(__pa0);
            maybe = metamodelica::Own::own(__pa1);
            unbound = List::setDifferenceOnTrue(unbound.clone(), metamodelica::AsArg::as_arg(&lhsNames), &fnptr!(stringEq, ArcStr, ArcStr))?;
            maybe = List::setDifferenceOnTrue(maybe.clone(), metamodelica::AsArg::as_arg(&lhsNames), &fnptr!(stringEq, ArcStr, ArcStr))?;
            (false, false, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_NORETCALL { exp, source }, (_, _, unbound, maybe)) => {
            let mut info: SourceInfo;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            info = ElementSource::getElementSourceFileInfo(source.clone());
            let (_, (__pa0, __pa1, _)) = Expression::traverseExpTopDown(exp.clone(), &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info))?;
            unbound = metamodelica::Own::own(__pa0);
            maybe = metamodelica::Own::own(__pa1);
            (false, false, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_BREAK { .. }, (_, _, unbound, maybe)) => {
            (true, false, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_RETURN { .. }, (_, _, unbound, maybe)) => {
            (true, true, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_CONTINUE { .. }, (_, _, unbound, maybe)) => {
            (false, false, unbound.clone(), maybe.clone())
        },
        (Deref @ DAE::Statement::STMT_ARRAY_INIT { .. }, _) => {
            inUnbound
        },
        (Deref @ DAE::Statement::STMT_FAILURE { body: stmts, .. }, (_, _, unbound, maybe)) => {
            let mut maybeBody: metamodelica::List<ArcStr>;
            let mut unboundBefore: metamodelica::List<ArcStr>;
            let mut b: bool;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            unboundBefore = unbound.clone();
            (_, b, unbound, maybeBody) = List::fold1(metamodelica::AsArg::as_arg(&stmts), &checkFunctionDefUseStmt, inLoop, (false, false, unbound.clone(), maybe.clone()))?;
            maybe = List::unionOnTrue(metamodelica::AsArg::as_arg(&maybe), &maybeBody, &fnptr!(stringEq, ArcStr, ArcStr))?;
            maybe = List::unionOnTrue(metamodelica::AsArg::as_arg(&maybe), &(List::setDifferenceOnTrue(unboundBefore, metamodelica::AsArg::as_arg(&unbound), &fnptr!(stringEq, ArcStr, ArcStr))?), &fnptr!(stringEq, ArcStr, ArcStr))?;
            (b, b, unbound.clone(), maybe.clone())
        },
        _ => {
            let mut r#str: ArcStr;
            let mut info: SourceInfo;
            r#str = DAEDump::ppStatementStr(inStmt.clone());
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstUtil.checkFunctionDefUseStmt failed: ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) };
            info = ElementSource::getElementSourceFileInfo(ElementSource::getStatementSource(&inStmt)?);
            Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![r#str], &info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outUnbound)
}

fn whileTrueBreakAssigned(
    mut stmts: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut entryUnbound: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut assigned: metamodelica::List<ArcStr>;
    let mut atBreak: Option<metamodelica::List<ArcStr>>;
    (_, _, atBreak) = walkBreakAssigned(stmts, entryUnbound.clone())?;
    assigned = (::match_deref::match_deref! { match &(atBreak) {
        Some(u) => {
            List::setDifferenceOnTrue(entryUnbound, metamodelica::AsArg::as_arg(&u), &fnptr!(stringEq, ArcStr, ArcStr))?
        },
        _ => {
            entryUnbound
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(assigned)
}

fn walkBreakAssigned(
    mut stmts: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inUnbound: metamodelica::List<ArcStr>,
) -> Result<(metamodelica::List<ArcStr>, bool, Option<metamodelica::List<ArcStr>>)> {
    let mut outUnbound: metamodelica::List<ArcStr> = inUnbound;
    let mut exits: bool = false;
    let mut atBreak: Option<metamodelica::List<ArcStr>> = None;
    let mut ub: metamodelica::List<ArcStr>;
    let mut ue: metamodelica::List<ArcStr>;
    let mut xt: bool;
    let mut xe: bool;
    let mut bt: Option<metamodelica::List<ArcStr>>;
    let mut be: Option<metamodelica::List<ArcStr>>;
    let mut ifBreak: Option<metamodelica::List<ArcStr>>;
    for mut s in &**stmts {
        let _ = (::match_deref::match_deref! { match &(s.clone()) {
            Deref @ DAE::Statement::STMT_ASSIGN { exp1: lhs, .. } => {
                outUnbound = crefFiltering(lhs.clone(), outUnbound)?;
                ()
            },
            Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs, .. } => {
                outUnbound = crefFiltering(lhs.clone(), outUnbound)?;
                ()
            },
            Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: lhss, .. } => {
                outUnbound = List::fold(metamodelica::AsArg::as_arg(&lhss), &crefFiltering, outUnbound)?;
                ()
            },
            Deref @ DAE::Statement::STMT_BREAK { .. } => {
                atBreak = mergeBreakUnbound(atBreak, Some(outUnbound.clone()))?;
                exits = true;
                ()
            },
            Deref @ DAE::Statement::STMT_RETURN { .. } => {
                exits = true;
                ()
            },
            Deref @ DAE::Statement::STMT_TERMINATE { .. } => {
                exits = true;
                ()
            },
            Deref @ DAE::Statement::STMT_NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fail" }, expLst: Deref @ metamodelica::ListNode::Nil, .. }, .. } => {
                exits = true;
                ()
            },
            Deref @ DAE::Statement::STMT_IF { statementLst: ss, else_: el, .. } => {
                (ub, xt, bt) = walkBreakAssigned(metamodelica::AsArg::as_arg(&ss), outUnbound.clone())?;
                (ue, xe, be) = walkBreakAssignedElse(metamodelica::AsArg::as_arg(&el), &outUnbound)?;
                (outUnbound, exits, ifBreak) = mergeBranches(outUnbound, ub, xt, bt, ue, xe, be)?;
                atBreak = mergeBreakUnbound(atBreak, ifBreak)?;
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if exits {
            break;
        }
    }
    Ok((outUnbound, exits, atBreak))
}

fn walkBreakAssignedElse(
    mut inElse: &metamodelica::Ref<DAE::Else>,
    mut inUnbound: &metamodelica::List<ArcStr>,
) -> Result<(metamodelica::List<ArcStr>, bool, Option<metamodelica::List<ArcStr>>)> {
    let mut outUnbound: metamodelica::List<ArcStr>;
    let mut exits: bool;
    let mut atBreak: Option<metamodelica::List<ArcStr>>;
    let mut ub: metamodelica::List<ArcStr>;
    let mut ue: metamodelica::List<ArcStr>;
    let mut xt: bool;
    let mut xe: bool;
    let mut bt: Option<metamodelica::List<ArcStr>>;
    let mut be: Option<metamodelica::List<ArcStr>>;
    (outUnbound, exits, atBreak) = (match &**inElse {
        DAE::Else::NOELSE { .. } => (inUnbound.clone(), false, None),
        DAE::Else::ELSE { statementLst: ss } => walkBreakAssigned(ss, inUnbound.clone())?,
        DAE::Else::ELSEIF {
            statementLst: ss,
            else_: el,
            ..
        } => {
            (ub, xt, bt) = walkBreakAssigned(ss, inUnbound.clone())?;
            (ue, xe, be) = walkBreakAssignedElse(el, inUnbound)?;
            mergeBranches(inUnbound.clone(), ub, xt, bt, ue, xe, be)?
        }
    });
    Ok((outUnbound, exits, atBreak))
}

fn mergeBranches(
    mut entryUnbound: metamodelica::List<ArcStr>,
    mut thenUnbound: metamodelica::List<ArcStr>,
    mut thenExits: bool,
    mut thenBreak: Option<metamodelica::List<ArcStr>>,
    mut elseUnbound: metamodelica::List<ArcStr>,
    mut elseExits: bool,
    mut elseBreak: Option<metamodelica::List<ArcStr>>,
) -> Result<(metamodelica::List<ArcStr>, bool, Option<metamodelica::List<ArcStr>>)> {
    let mut outUnbound: metamodelica::List<ArcStr>;
    let mut exits: bool;
    let mut atBreak: Option<metamodelica::List<ArcStr>>;
    atBreak = mergeBreakUnbound(thenBreak, elseBreak)?;
    if thenExits && elseExits {
        exits = true;
        outUnbound = entryUnbound;
    } else if thenExits {
        exits = false;
        outUnbound = elseUnbound;
    } else if elseExits {
        exits = false;
        outUnbound = thenUnbound;
    } else {
        exits = false;
        outUnbound = List::unionOnTrue(&thenUnbound, &elseUnbound, &fnptr!(stringEq, ArcStr, ArcStr))?;
    }
    Ok((outUnbound, exits, atBreak))
}

fn mergeBreakUnbound(
    mut a: Option<metamodelica::List<ArcStr>>,
    mut b: Option<metamodelica::List<ArcStr>>,
) -> Result<Option<metamodelica::List<ArcStr>>> {
    let mut res: Option<metamodelica::List<ArcStr>>;
    res = (::match_deref::match_deref! { match &((a.clone(), b.clone())) {
        (None, _) => {
            b
        },
        (_, None) => {
            a
        },
        (Some(la), Some(lb)) => {
            Some(List::unionOnTrue(metamodelica::AsArg::as_arg(&la), metamodelica::AsArg::as_arg(&lb), &fnptr!(stringEq, ArcStr, ArcStr))?)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(res)
}

fn checkFunctionDefUseElse(
    mut inElse: &metamodelica::Ref<DAE::Else>,
    mut inUnbound: metamodelica::List<ArcStr>,
    mut inMaybeUnbound: &metamodelica::List<ArcStr>,
    mut inLoop: bool,
    mut info: &SourceInfo,
) -> Result<(bool, bool, metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut outUnbound: (bool, bool, metamodelica::List<ArcStr>, metamodelica::List<ArcStr>);
    outUnbound = (match &**inElse {
        DAE::Else::NOELSE { .. } => (false, false, inUnbound, inMaybeUnbound.clone()),
        DAE::Else::ELSEIF {
            exp,
            statementLst: stmts,
            else_,
        } => {
            let mut unbound = inUnbound.clone();
            let mut unboundBranch: metamodelica::List<ArcStr>;
            let mut maybe: metamodelica::List<ArcStr>;
            let mut maybeBranch: metamodelica::List<ArcStr>;
            let mut assignedThen: metamodelica::List<ArcStr>;
            let mut assignedElse: metamodelica::List<ArcStr>;
            let mut proven: metamodelica::List<ArcStr>;
            let mut entryUnbound: metamodelica::List<ArcStr>;
            let mut b1: bool;
            let mut b2: bool;
            let mut b3: bool;
            let mut b4: bool;
            maybe = inMaybeUnbound.clone();
            let (_, (__pa0, __pa1, _)) =
                Expression::traverseExpTopDown(exp.clone(), &findUnboundVariableUse, (unbound, maybe, info.clone()))?;
            unbound = metamodelica::Own::own(__pa0);
            maybe = metamodelica::Own::own(__pa1);
            entryUnbound = unbound.clone();
            (b1, b2, unboundBranch, maybeBranch) =
                checkFunctionDefUseElse(else_, unbound.clone(), &maybe, inLoop, info)?;
            (b3, b4, unbound, maybe) =
                List::fold1(stmts, &checkFunctionDefUseStmt, inLoop, (false, false, unbound, maybe))?;
            assignedThen = if (b4) {
                metamodelica::nil()
            } else {
                List::setDifferenceOnTrue(entryUnbound.clone(), &unbound, &fnptr!(stringEq, ArcStr, ArcStr))?
            };
            assignedElse = if (b2) {
                metamodelica::nil()
            } else {
                List::setDifferenceOnTrue(entryUnbound, &unboundBranch, &fnptr!(stringEq, ArcStr, ArcStr))?
            };
            if b2 && !(b4) {
                proven = assignedThen.clone();
            } else if b4 && !(b2) {
                proven = assignedElse.clone();
            } else {
                proven = List::intersectionOnTrue(&assignedThen, &assignedElse, &fnptr!(stringEq, ArcStr, ArcStr))?;
            }
            maybe = List::unionOnTrue(
                &(if (b4) { metamodelica::nil() } else { maybe }),
                &(if (b2) { metamodelica::nil() } else { maybeBranch }),
                &fnptr!(stringEq, ArcStr, ArcStr),
            )?;
            maybe = List::unionOnTrue(
                &maybe,
                &(List::setDifferenceOnTrue(
                    List::unionOnTrue(&assignedThen, &assignedElse, &fnptr!(stringEq, ArcStr, ArcStr))?,
                    &proven,
                    &fnptr!(stringEq, ArcStr, ArcStr),
                )?),
                &fnptr!(stringEq, ArcStr, ArcStr),
            )?;
            unbound = List::intersectionOnTrue(&unboundBranch, &unbound, &fnptr!(stringEq, ArcStr, ArcStr))?;
            b1 = b1 && b3;
            b2 = b2 && b4;
            (b1, b2, unbound, maybe)
        }
        DAE::Else::ELSE { statementLst: stmts } => {
            let mut unbound = inUnbound.clone();
            let mut maybe: metamodelica::List<ArcStr>;
            let mut b1: bool;
            let mut b2: bool;
            (b1, b2, unbound, maybe) = List::fold1(
                stmts,
                &checkFunctionDefUseStmt,
                inLoop,
                (false, false, unbound, inMaybeUnbound.clone()),
            )?;
            (b1, b2, unbound, maybe)
        }
    });
    Ok(outUnbound)
}

fn crefFiltering(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inUnbound: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExp, inUnbound.clone())) {
            (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, _) => {
                return Ok(inUnbound)
            },
            (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident: id1, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id2, .. }, .. }, .. }, unbound) => {
                let mut unbound = (*unbound).clone();
                return Ok(List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), { let mut __mm_s = String::new(); __mm_s.push_str(&*id1); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*id2); ArcStr::from(__mm_s) })?)
            },
            (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id1, .. }, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } }, unbound) => {
                let mut id1 = (*id1).clone();
                let mut unbound = (*unbound).clone();
                id1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*id1); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) };
                return Ok(List::filter2OnTrue(unbound.clone(), (std::sync::Arc::new(fnptr!(Util::notStrncmp, ArcStr, ArcStr, i32)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, i32) -> Result<bool> + 'static>), id1.clone(), ((id1).len() as i32))?)
            },
            (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, unbound) => {
                let mut unbound = (*unbound).clone();
                return Ok(List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), ComponentReferenceBasics::crefFirstIdent(metamodelica::AsArg::as_arg(&cr))?)?)
            },
            (Deref @ DAE::Exp::ASUB { exp, .. }, unbound) => {
                { (inExp, inUnbound) = (exp.clone(), unbound.clone()); continue '__tco; }
            },
            (Deref @ DAE::Exp::PATTERN { pattern }, unbound) => {
                let mut unbound = (*unbound).clone();
                (_, unbound) = Patternm::traversePattern(pattern.clone(), &patternFiltering, unbound.clone())?;
                return Ok(unbound.clone())
            },
            _ => {
                return Ok(inUnbound)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn patternFiltering(
    mut inPat: metamodelica::Ref<DAE::Pattern>,
    mut inLst: metamodelica::List<ArcStr>,
) -> Result<(metamodelica::Ref<DAE::Pattern>, metamodelica::List<ArcStr>)> {
    let mut outPat: metamodelica::Ref<DAE::Pattern> = inPat.clone();
    let mut unbound: metamodelica::List<ArcStr> = inLst;
    unbound = (match &*inPat {
        DAE::Pattern::PAT_AS { id: __inPat_id, .. } => List::filter1OnTrue(
            unbound,
            (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1))
            }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            __inPat_id.clone(),
        )?,
        DAE::Pattern::PAT_AS_FUNC_PTR { id: __inPat_id, .. } => List::filter1OnTrue(
            unbound,
            (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1))
            }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            __inPat_id.clone(),
        )?,
        _ => unbound,
    });
    Ok((outPat, unbound))
}

fn traverseCrefSubs(
    mut exp: &metamodelica::Ref<DAE::Exp>,
    mut info: SourceInfo,
    mut inUnbound: metamodelica::List<ArcStr>,
    mut inMaybeUnbound: metamodelica::List<ArcStr>,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut outUnbound: metamodelica::List<ArcStr>;
    let mut outMaybeUnbound: metamodelica::List<ArcStr>;
    (outUnbound, outMaybeUnbound) = (match &**exp {
        DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut unbound = inUnbound.clone();
            let mut maybe: metamodelica::List<ArcStr>;
            let (_, (__pa0, __pa1, _)) =
                Expression::traverseExpTopDownCrefHelper(cr, &findUnboundVariableUse, (unbound, inMaybeUnbound, info))?;
            unbound = metamodelica::Own::own(__pa0);
            maybe = metamodelica::Own::own(__pa1);
            (unbound, maybe)
        }
        _ => (inUnbound, inMaybeUnbound),
    });
    Ok((outUnbound, outMaybeUnbound))
}

fn findUnboundVariableUse(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (metamodelica::List<ArcStr>, metamodelica::List<ArcStr>, SourceInfo),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::List<ArcStr>, metamodelica::List<ArcStr>, SourceInfo),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (metamodelica::List<ArcStr>, metamodelica::List<ArcStr>, SourceInfo);
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (exp @ Deref @ DAE::Exp::SIZE { .. }, arg) => {
            (exp.clone(), false, arg.clone())
        },
        (exp @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "isPresent" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, arg) => {
            (exp.clone(), false, arg.clone())
        },
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, (_, _, info)) => {
            Error::addSourceMessage(&(Error::WARNING_DEF_USE.clone()), list![literal!("_")], metamodelica::AsArg::as_arg(&info))?;
            (inExp, true, inTpl)
        },
        (exp @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (unbound, maybe, info)) => {
            let mut r#str: ArcStr;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            r#str = ComponentReferenceBasics::crefFirstIdent(metamodelica::AsArg::as_arg(&cr))?;
            if listMember(r#str.clone(), unbound.clone()) {
                Error::addSourceMessage(&(Error::WARNING_DEF_USE.clone()), list![r#str.clone()], metamodelica::AsArg::as_arg(&info))?;
                unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), r#str)?;
            } else if listMember(r#str.clone(), maybe.clone()) {
                if Flags::isSet(Flags::CHECK_DEF_USE.clone())? {
                    Error::addSourceMessage(&(Error::WARNING_DEF_USE_UNPROVEN.clone()), list![r#str.clone()], metamodelica::AsArg::as_arg(&info))?;
                }
                maybe = List::filter1OnTrue(maybe.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), r#str)?;
            }
            (exp.clone(), true, (unbound.clone(), maybe.clone(), info.clone()))
        },
        (exp @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, .. }, (unbound, maybe, info)) => {
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            if listMember(name.clone(), unbound.clone()) {
                Error::addSourceMessage(&(Error::WARNING_DEF_USE.clone()), list![name.clone()], metamodelica::AsArg::as_arg(&info))?;
                unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), name.clone())?;
            } else if listMember(name.clone(), maybe.clone()) {
                if Flags::isSet(Flags::CHECK_DEF_USE.clone())? {
                    Error::addSourceMessage(&(Error::WARNING_DEF_USE_UNPROVEN.clone()), list![name.clone()], metamodelica::AsArg::as_arg(&info))?;
                }
                maybe = List::filter1OnTrue(maybe.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), name.clone())?;
            }
            (exp.clone(), true, (unbound.clone(), maybe.clone(), info.clone()))
        },
        (exp @ Deref @ DAE::Exp::MATCHEXPRESSION { inputs, localDecls, cases, .. }, (unbound, maybe, info)) => {
            let mut unboundLocal: metamodelica::List<ArcStr>;
            let mut assigned: metamodelica::List<ArcStr>;
            let mut assignedUnion: metamodelica::List<ArcStr>;
            let mut proven: metamodelica::List<ArcStr>;
            let mut maybeMerged: metamodelica::List<ArcStr>;
            let mut caseUnbound: metamodelica::List<ArcStr>;
            let mut caseMaybe: metamodelica::List<ArcStr>;
            let mut caseReturns: bool;
            let mut provenInit: bool;
            let mut caseResults: metamodelica::List<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>, bool)>;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            let (_, (__pa0, __pa1, _)) = Expression::traverseExpTopDown(metamodelica::Ref::new(DAE::Exp::LIST { valList: inputs.clone() }), &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info.clone()))?;
            unbound = metamodelica::Own::own(__pa0);
            maybe = metamodelica::Own::own(__pa1);
            unboundLocal = checkFunctionDefUse2(localDecls.clone(), None, unbound.clone(), metamodelica::nil(), metamodelica::AsArg::as_arg(&info))?;
            caseResults = ({
        let mut __acc: metamodelica::List<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>, bool)> = metamodelica::nil();
        for mut c in (cases.clone()).into_iter().cloned() {
            let __x = findUnboundVariableUseInCase(&(c.clone()), unboundLocal.clone(), maybe.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            unbound = List::fold1r(&(({
        let mut __acc: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut t in (caseResults.clone()).into_iter().cloned() {
            let __x = Util::tuple31(t.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), &move |__a0: _, __a1: _, __a2: _| List::intersectionOnTrue(&__a0, &__a1, metamodelica::arc_ref(&__a2)), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), unbound.clone())?;
            provenInit = true;
            proven = metamodelica::nil();
            assignedUnion = metamodelica::nil();
            maybeMerged = metamodelica::nil();
            for mut t in &*caseResults {
                (caseUnbound, caseMaybe, caseReturns) = t.clone();
                if !(caseReturns) {
                    assigned = List::setDifferenceOnTrue(unboundLocal.clone(), &caseUnbound, &fnptr!(stringEq, ArcStr, ArcStr))?;
                    proven = if (provenInit) {assigned.clone()} else {List::intersectionOnTrue(&proven, &assigned, &fnptr!(stringEq, ArcStr, ArcStr))?};
                    provenInit = false;
                    assignedUnion = List::unionOnTrue(&assignedUnion, &assigned, &fnptr!(stringEq, ArcStr, ArcStr))?;
                    maybeMerged = List::unionOnTrue(&maybeMerged, &caseMaybe, &fnptr!(stringEq, ArcStr, ArcStr))?;
                }
            }
            if !(provenInit) {
                maybe = List::unionOnTrue(&maybeMerged, &(List::setDifferenceOnTrue(assignedUnion, &proven, &fnptr!(stringEq, ArcStr, ArcStr))?), &fnptr!(stringEq, ArcStr, ArcStr))?;
            }
            (exp.clone(), false, (unbound.clone(), maybe.clone(), info.clone()))
        },
        (exp @ Deref @ DAE::Exp::REDUCTION { expr: redExpr, iterators, .. }, (unbound, maybe, info)) => {
            let mut name: ArcStr;
            let mut iterExp: metamodelica::Ref<DAE::Exp>;
            let mut guardExp: Option<metamodelica::Ref<DAE::Exp>>;
            let mut iterUnbound: metamodelica::List<ArcStr>;
            let mut iterMaybe: metamodelica::List<ArcStr>;
            let mut unbound = (*unbound).clone();
            let mut maybe = (*maybe).clone();
            for mut it in &*iterators.clone() {
                let __arc1 = it.clone();
                let DAE::REDUCTIONITER { exp: __pa0, .. } = &*__arc1;
                iterExp = metamodelica::Own::own(__pa0);
                let (_, (__pa2, __pa3, _)) = Expression::traverseExpTopDown(iterExp, &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info.clone()))?;
                unbound = metamodelica::Own::own(__pa2);
                maybe = metamodelica::Own::own(__pa3);
            }
            iterUnbound = metamodelica::nil();
            iterMaybe = metamodelica::nil();
            for mut it in &*iterators.clone() {
                let __arc5 = it.clone();
                let DAE::REDUCTIONITER { id: __pa4, .. } = &*__arc5;
                name = metamodelica::Own::own(__pa4);
                if listMember(name.clone(), unbound.clone()) {
                    iterUnbound = metamodelica::cons(name.clone(), iterUnbound);
                    unbound = List::filter1OnTrue(unbound.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), name.clone())?;
                }
                if listMember(name.clone(), maybe.clone()) {
                    iterMaybe = metamodelica::cons(name.clone(), iterMaybe);
                    maybe = List::filter1OnTrue(maybe.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::stringNotEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), name)?;
                }
            }
            for mut it in &*iterators.clone() {
                let __arc7 = it.clone();
                let DAE::REDUCTIONITER { guardExp: __pa6, .. } = &*__arc7;
                guardExp = metamodelica::Own::own(__pa6);
                let (_, (__pa8, __pa9, _)) = Expression::traverseExpTopDown(metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: guardExp }), &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info.clone()))?;
                unbound = metamodelica::Own::own(__pa8);
                maybe = metamodelica::Own::own(__pa9);
            }
            let (_, (__pa10, __pa11, _)) = Expression::traverseExpTopDown(redExpr.clone(), &findUnboundVariableUse, (unbound.clone(), maybe.clone(), info.clone()))?;
            unbound = metamodelica::Own::own(__pa10);
            maybe = metamodelica::Own::own(__pa11);
            unbound = listAppend(iterUnbound, unbound.clone());
            maybe = listAppend(iterMaybe, maybe.clone());
            (exp.clone(), false, (unbound.clone(), maybe.clone(), info.clone()))
        },
        (exp, arg) => {
            (exp.clone(), true, arg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

fn findUnboundVariableUseInCase(
    mut case_: &metamodelica::Ref<DAE::MatchCase>,
    mut inUnbound: metamodelica::List<ArcStr>,
    mut inMaybeUnbound: metamodelica::List<ArcStr>,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>, bool)> {
    let mut outResult: (metamodelica::List<ArcStr>, metamodelica::List<ArcStr>, bool);
    outResult = (match &**case_ {
        DAE::MatchCase {
            patterns,
            patternGuard,
            body,
            result,
            info,
            resultInfo,
            ..
        } => {
            let mut unbound = inUnbound;
            let mut maybe: metamodelica::List<ArcStr>;
            let mut returned: bool;
            maybe = inMaybeUnbound;
            (_, unbound) = Patternm::traversePatternList(patterns, &patternFiltering, unbound)?;
            (_, maybe) = Patternm::traversePatternList(patterns, &patternFiltering, maybe)?;
            let (_, (__pa0, __pa1, _)) = Expression::traverseExpTopDown(
                metamodelica::Ref::new(DAE::Exp::META_OPTION {
                    exp: patternGuard.clone(),
                }),
                &findUnboundVariableUse,
                (unbound, maybe, info.clone()),
            )?;
            unbound = metamodelica::Own::own(__pa0);
            maybe = metamodelica::Own::own(__pa1);
            (_, returned, unbound, maybe) =
                List::fold1(body, &checkFunctionDefUseStmt, true, (false, false, unbound, maybe))?;
            let (_, (__pa2, __pa3, _)) = Expression::traverseExpTopDown(
                metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: result.clone() }),
                &findUnboundVariableUse,
                (unbound, maybe, resultInfo.clone()),
            )?;
            unbound = metamodelica::Own::own(__pa2);
            maybe = metamodelica::Own::own(__pa3);
            returned = returned || !((result).is_some());
            (unbound, maybe, returned)
        }
    });
    Ok(outResult)
}

pub(crate) fn checkParallelismWRTEnv(
    mut inEnv: &FCore::Graph,
    mut inName: &ArcStr,
    mut inAttr: &SCode::Attributes,
    mut inInfo: &SourceInfo,
) -> bool {
    let mut isValid: bool;
    isValid = 'mc: {
        let __mc_input = inAttr.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let SCode::Attributes {
                parallelism: mut prl,
                direction: mut dir,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut errorString: ArcStr;
            let mut scopeName: ArcStr;
            let mut isparglobal: bool;
            let mut hasnodir: bool;
            let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
            r = FGraph::lastScopeRef(inEnv)?;
            let false = (FNode::isRefTop(r.clone())) else {
                return Err("pattern mismatch");
            };
            scopeName = FNode::refName(r.clone());
            let true = (FGraph::checkScopeType(
                &(list![r.clone()]),
                Some(openmodelica_frontend_dump::FCore::ScopeType::PARALLEL_SCOPE),
            )) else {
                return Err("pattern mismatch");
            };
            isparglobal =
                SCodeUtil::parallelismEqual(prl.clone(), openmodelica_frontend_types::SCode::Parallelism::PARGLOBAL);
            hasnodir = !(AbsynUtil::isInputOrOutput(dir.clone()));
            let true = (isparglobal && hasnodir) else {
                return Err("pattern mismatch");
            };
            errorString = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*literal!("- local parglobal component '"));
                __mm_s.push_str(&*inName);
                __mm_s.push_str(&*literal!("' is declared in parallel/parkernel function '"));
                __mm_s.push_str(&*scopeName);
                __mm_s.push_str(&*literal!("'. \n"));
                __mm_s.push_str(&*literal!(
                    "- parglobal variables can be declared only in normal functions. \n"
                ));
                ArcStr::from(__mm_s)
            };
            Error::addSourceMessage(&(Error::PARMODELICA_ERROR.clone()), list![errorString.clone()], inInfo)?;
            Ok(false)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(true)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    isValid
}

pub(crate) fn instDimsHasZeroDims(
    mut inInstDims: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
) -> bool {
    let mut outHasZeroDims: bool;
    outHasZeroDims = 'mc: {
        let __mc_input = &**inInstDims;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: dims, tail: _ } => {
                    let true = (List::any(metamodelica::AsArg::as_arg(&dims), &move |__a0: metamodelica::Ref<DAE::Dimension>| Expression::dimensionIsZero(&__a0))?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_dims } => {
                    Ok(instDimsHasZeroDims(metamodelica::AsArg::as_arg(&rest_dims)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outHasZeroDims
}

pub(crate) fn noModForUpdatedComponents(
    mut variability: SCode::Variability,
    mut updatedComps: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::ComponentRef>,
                        metamodelica::Ref<Absyn::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut mods: metamodelica::Ref<DAE::Mod>,
    mut cmod: metamodelica::Ref<DAE::Mod>,
    mut m: metamodelica::Ref<SCode::Mod>,
) -> (
    metamodelica::Ref<DAE::Mod>,
    metamodelica::Ref<DAE::Mod>,
    metamodelica::Ref<SCode::Mod>,
) {
    let mut outMods: metamodelica::Ref<DAE::Mod>;
    let mut outCmod: metamodelica::Ref<DAE::Mod>;
    let mut outM: metamodelica::Ref<SCode::Mod>;
    (outMods, outCmod, outM) = 'mc: {
        let __mc_input = &*m;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((BaseHashTable::hasKey(cref.clone(), updatedComps)?)) { return Err("guard") }
                    checkVariabilityOfUpdatedComponent(variability, &cref)?;
                    Ok((openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::SCode::Mod::interned_NOMOD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((mods.clone(), cmod.clone(), m.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outMods, outCmod, outM)
}

pub(crate) fn propagateModFinal(mut inMod: &metamodelica::Ref<DAE::Mod>, mut inFinal: SCode::Final) -> SCode::Final {
    let mut outFinal: SCode::Final;
    outFinal = (::match_deref::match_deref! { match &((inMod.clone(), inFinal)) {
        (_, SCode::Final::FINAL { .. }) => {
            inFinal
        },
        (Deref @ DAE::Mod::MOD { finalPrefix: fp, .. }, _) => {
            fp.clone()
        },
        _ => {
            inFinal
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outFinal
}

//------------------------------
//------  PDE extension:  ------
//------------------------------
pub type DomainFieldOpt = Option<(
    metamodelica::Ref<Absyn::ComponentRef>,
    metamodelica::Ref<DAE::ComponentRef>,
)>;

pub type DomainFieldsLst = metamodelica::List<(
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
)>;

pub(crate) fn addGhostCells(
    mut inCompelts: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    mut inEqs: &metamodelica::List<metamodelica::Ref<SCode::Equation>>,
) -> Result<metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>> {
    let mut outCompelts: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    let mut fieldNamesP: metamodelica::List<ArcStr>;
    let mut ghostCompelts: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    fieldNamesP = List::fold(
        inEqs,
        &move |__a0: metamodelica::Ref<SCode::Equation>, __a1: metamodelica::List<ArcStr>| fieldsInPderEq(&__a0, __a1),
        metamodelica::nil(),
    )?;
    ghostCompelts = List::fold1(
        &inCompelts,
        &move |__a0: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
               __a1: metamodelica::List<ArcStr>,
               __a2: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(addGhostCells2(&__a0, __a1, __a2)) },
        fieldNamesP,
        metamodelica::nil(),
    )?;
    outCompelts = listAppend(inCompelts, ghostCompelts);
    Ok(outCompelts)
}

pub(crate) fn fieldsInPderEq(
    mut eq: &metamodelica::Ref<SCode::Equation>,
    mut inFieldNames: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outFieldNames: metamodelica::List<ArcStr>;
    outFieldNames = (match &**eq {
        SCode::Equation::EQ_PDE {
            expLeft: lhs_exp,
            expRight: rhs_exp,
            ..
        } => {
            let mut fieldNames1: metamodelica::List<ArcStr>;
            (_, fieldNames1) = AbsynUtil::traverseExpTopDown(
                lhs_exp.clone(),
                (std::sync::Arc::new(fnptr!(
                    fieldInPderExp,
                    metamodelica::Ref<Absyn::Exp>,
                    metamodelica::List<ArcStr>
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Exp>,
                                metamodelica::List<ArcStr>,
                            )
                                -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<ArcStr>)>
                            + 'static,
                    >),
                inFieldNames.clone(),
            )?;
            (_, fieldNames1) = AbsynUtil::traverseExpTopDown(
                rhs_exp.clone(),
                (std::sync::Arc::new(fnptr!(
                    fieldInPderExp,
                    metamodelica::Ref<Absyn::Exp>,
                    metamodelica::List<ArcStr>
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Exp>,
                                metamodelica::List<ArcStr>,
                            )
                                -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<ArcStr>)>
                            + 'static,
                    >),
                fieldNames1,
            )?;
            listAppend(inFieldNames, fieldNames1)
        }
        _ => inFieldNames,
    });
    Ok(outFieldNames)
}

pub(crate) fn fieldInPderExp(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inFieldNames: metamodelica::List<ArcStr>,
) -> (metamodelica::Ref<Absyn::Exp>, metamodelica::List<ArcStr>) {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outFieldNames: metamodelica::List<ArcStr>;
    outFieldNames = (::match_deref::match_deref! { match &(&*inExp) {
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: newFieldName, .. } }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. } => {
            List::unionElt(newFieldName.clone(), inFieldNames)
        },
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: newFieldName, .. } }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, .. } => {
            List::unionElt(newFieldName.clone(), inFieldNames)
        },
        _ => {
            inFieldNames
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp = inExp;
    (outExp, outFieldNames)
}

pub(crate) fn addGhostCells2(
    mut inCompelt: &(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>),
    mut fieldNamesP: metamodelica::List<ArcStr>,
    mut inGhosts: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
) -> metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)> {
    let mut outGhosts: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    outGhosts = 'mc: {
        let __mc_input = inCompelt;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { name, prefixes, attributes: SCode::Attributes { arrayDims, connectorType, parallelism, variability, direction, isField: Absyn::IsField::FIELD { .. } }, typeSpec, modifications: r#mod @ Deref @ SCode::Mod::MOD { .. }, comment, condition, info }, daeMod) => {
                    if !((listMember(name.clone(), fieldNamesP.clone()))) { return Err("guard") }
                    let mut ghostL: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>);
                    let mut ghostR: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>);
                    let mut r#mod = (*r#mod).clone();
                    assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = List::filterOnFalse(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone(), &move |__a0: metamodelica::Ref<SCode::SubMod>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isSubModDomainOrStart(&__a0)) })?);
                    ghostL = (metamodelica::Ref::new(SCode::Element::COMPONENT { name: stringAppend(name.clone(), literal!("$ghostL")), prefixes: prefixes.clone(), attributes: SCode::Attributes { arrayDims: arrayDims.clone(), connectorType: connectorType.clone(), parallelism: parallelism.clone(), variability: variability.clone(), direction: direction.clone(), isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: typeSpec.clone(), modifications: r#mod.clone(), comment: comment.clone(), condition: condition.clone(), info: info.clone() }), daeMod.clone());
                    ghostR = (metamodelica::Ref::new(SCode::Element::COMPONENT { name: stringAppend(name.clone(), literal!("$ghostR")), prefixes: prefixes.clone(), attributes: SCode::Attributes { arrayDims: arrayDims.clone(), connectorType: connectorType.clone(), parallelism: parallelism.clone(), variability: variability.clone(), direction: direction.clone(), isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: typeSpec.clone(), modifications: r#mod.clone(), comment: comment.clone(), condition: condition.clone(), info: info.clone() }), daeMod.clone());
                    Ok(metamodelica::cons(ghostL.clone(), metamodelica::cons(ghostR.clone(), inGhosts.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inGhosts.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outGhosts
}

pub(crate) fn isSubModDomainOrStart(mut subMod: &metamodelica::Ref<SCode::SubMod>) -> bool {
    let mut isNotDomain: bool;
    isNotDomain = (match &**subMod {
        SCode::SubMod { ident: idn, .. }
            if (metamodelica::stringEq(&idn, &(literal!("domain")))
                || metamodelica::stringEq(&idn, &(literal!("start")))) =>
        {
            true
        }
        _ => false,
    });
    isNotDomain
}

pub(crate) fn elabField(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut name: ArcStr,
    mut attr: &SCode::Attributes,
    mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inInfo: &SourceInfo,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    metamodelica::Ref<DAE::Mod>,
    DomainFieldOpt,
)> {
    let mut outDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    let mut outFieldDomOpt: DomainFieldOpt;
    (outDims, outMod, outFieldDomOpt) = ({
        let mut N: i32 = -1;
        (::match_deref::match_deref! { match &((attr.clone(), inMod.clone())) {
            (SCode::Attributes { isField: Absyn::IsField::NONFIELD { .. }, .. }, _) => {
                (inDims, inMod, None)
            },
            (SCode::Attributes { isField: Absyn::IsField::FIELD { .. }, .. }, Deref @ DAE::Mod::MOD { finalPrefix, eachPrefix, subModLst, binding, info }) => {
                let mut dim_f: metamodelica::Ref<DAE::Dimension>;
                let mut dcr: metamodelica::Ref<DAE::ComponentRef>;
                let mut domainSubMod: metamodelica::Ref<DAE::SubMod>;
                let mut subModLst = (*subModLst).clone();
                (domainSubMod, subModLst) = List::findAndRemove(subModLst.clone(), &move |__a0: metamodelica::Ref<DAE::SubMod>| -> metamodelica::Result<_> { ::std::result::Result::Ok(findDomainSubMod(&__a0)) })?;
                dcr = getQualDcr(&domainSubMod, inInfo)?;
                (N, dcr) = getNDcr(&dcr)?;
                if N == -1 {
                    Error::addSourceMessageAndFail(&(Error::PDEModelica_ERROR.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Domain of the field variable '")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("' not found.")); ArcStr::from(__mm_s) }], inInfo)?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                }
                subModLst = subModLst.clone().reverse();
                subModLst = List::map(subModLst.clone(), &move |__a0: metamodelica::Ref<DAE::SubMod>| addEach(&__a0))?;
                outMod = metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: eachPrefix.clone(), subModLst: subModLst.clone(), binding: binding.clone(), info: info.clone() });
                dim_f = metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: N });
                (metamodelica::cons(dim_f, inDims), outMod, Some((metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name, subscripts: metamodelica::nil() }), dcr)))
            },
            (_, Deref @ DAE::Mod::NOMOD { .. }) => {
                Error::addSourceMessageAndFail(&(Error::PDEModelica_ERROR.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Field variable '")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("' has no domain modifier.")); ArcStr::from(__mm_s) }], inInfo)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                (inDims, inMod, None)
            },
            _ => return Err("match: no arm matched"),
        } })
    });
    Ok((outDims, outMod, outFieldDomOpt))
}

fn findDomainSubMod(mut subMod: &metamodelica::Ref<DAE::SubMod>) -> bool {
    let mut isDomain: bool;
    isDomain = (::match_deref::match_deref! { match subMod {
        Deref @ DAE::SubMod { ident: Deref @ "domain", .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isDomain
}

fn getQualDcr(
    mut domainSubMod: &metamodelica::Ref<DAE::SubMod>,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut dcr: metamodelica::Ref<DAE::ComponentRef>;
    dcr = (::match_deref::match_deref! { match domainSubMod {
        Deref @ DAE::SubMod { ident: Deref @ "domain", r#mod: Deref @ DAE::Mod::MOD { binding: Some(DAE::EqMod::TYPED { modifierAsExp: Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "DomainLineSegment1D" } } }, .. } }, .. }), .. } } => {
            cr.clone()
        },
        _ => {
            Error::addSourceMessageAndFail(&(Error::PDEModelica_ERROR.clone()), list![literal!("The domain type is wrong.\n")], inInfo)?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dcr)
}

fn getNDcr<'__b>(
    mut dcr: &'__b metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(i32, metamodelica::Ref<DAE::ComponentRef>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match dcr {
            Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr1, .. } => {
                { dcr = cr1; continue '__tco; }
            },
            Deref @ DAE::ComponentRef::CREF_IDENT { identType: Deref @ DAE::Type::T_COMPLEX { varLst, .. }, .. } => {
                let mut N: i32;
                let __pa0 = ::match_deref::match_deref! { match &(List::findSome(metamodelica::AsArg::as_arg(&varLst), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(findN(&__a0)) })?) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                N = metamodelica::Own::own(__pa0);
                return Ok((N, dcr.clone()))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn findN(mut inVar: &metamodelica::Ref<DAE::Var>) -> Option<i32> {
    let mut optN: Option<i32>;
    optN = (::match_deref::match_deref! { match inVar {
        Deref @ DAE::Var { name: Deref @ "N", binding: Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(Deref @ Values::Value::INTEGER { integer: N }), .. }, .. } => {
            Some(N.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    optN
}

fn addEach(mut inSubMod: &metamodelica::Ref<DAE::SubMod>) -> Result<metamodelica::Ref<DAE::SubMod>> {
    let mut outSubMod: metamodelica::Ref<DAE::SubMod>;
    let mut ident: ArcStr;
    let mut finalPrefix: SCode::Final;
    let mut subModLst: metamodelica::List<metamodelica::Ref<DAE::SubMod>>;
    let mut binding: Option<DAE::EqMod>;
    let mut info: SourceInfo;
    outSubMod = (::match_deref::match_deref! { match inSubMod {
        Deref @ DAE::SubMod { ident: __esc_ident, r#mod: Deref @ DAE::Mod::MOD { finalPrefix: __esc_finalPrefix, eachPrefix: _, subModLst: __esc_subModLst, binding: __esc_binding, info: __esc_info } } => {
            ident = (*__esc_ident).clone();
            finalPrefix = (*__esc_finalPrefix).clone();
            subModLst = (*__esc_subModLst).clone();
            binding = (*__esc_binding).clone();
            info = (*__esc_info).clone();
            metamodelica::Ref::new(DAE::SubMod { ident: ident.clone(), r#mod: metamodelica::Ref::new(DAE::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: openmodelica_frontend_types::SCode::Each::EACH, subModLst: subModLst.clone(), binding: binding.clone(), info: info.clone() }) })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outSubMod)
}

//----end elabField and sub funs
pub(crate) fn optAppendField(
    mut inDomFieldsLst: DomainFieldsLst,
    mut fieldDomOpt: DomainFieldOpt,
) -> Result<DomainFieldsLst> {
    let mut outDomFieldsLst: DomainFieldsLst;
    outDomFieldsLst = (::match_deref::match_deref! { match &(fieldDomOpt) {
        None => {
            inDomFieldsLst
        },
        Some((fieldCr, domainCr)) => {
            let mut found: bool;
            (outDomFieldsLst, found) = List::map2Fold(&inDomFieldsLst, &move |__a0: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>), __a1: metamodelica::Ref<DAE::ComponentRef>, __a2: metamodelica::Ref<Absyn::ComponentRef>, __a3: bool| -> metamodelica::Result<_> { ::std::result::Result::Ok(optAppendFieldMapFun(__a0, &__a1, __a2, __a3)) }, domainCr.clone(), fieldCr.clone(), false, metamodelica::nil())?;
            if !(found) {
                outDomFieldsLst = metamodelica::cons((domainCr.clone(), list![fieldCr.clone()]), inDomFieldsLst);
            }
            outDomFieldsLst
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDomFieldsLst)
}

fn optAppendFieldMapFun(
    mut inDomainFields: (
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    ),
    mut domainCrToAdd: &metamodelica::Ref<DAE::ComponentRef>,
    mut fieldCrToAdd: metamodelica::Ref<Absyn::ComponentRef>,
    mut inFound: bool,
) -> (
    (
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    ),
    bool,
) {
    let mut outDomainFields: (
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    );
    let mut outFound: bool;
    (outDomainFields, outFound) = 'mc: {
        let __mc_input = (&inDomainFields, inFound);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((domainCr, fieldCrLst), false) => {
                    let true = (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&domainCr), domainCrToAdd)?) else { return Err("pattern mismatch") };
                    Ok(((domainCr.clone(), metamodelica::cons(fieldCrToAdd.clone(), fieldCrLst.clone())), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inDomainFields.clone(), inFound))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outDomainFields, outFound)
}

//----end optAppendField and sub funs
pub(crate) fn discretizePDE(
    mut inEQ: metamodelica::Ref<SCode::Equation>,
    mut inDomFieldLst: &DomainFieldsLst,
    mut inDiscretizedEQs: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Equation>>> {
    let mut outDiscretizedEQs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut newDiscretizedEQs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    newDiscretizedEQs = (::match_deref::match_deref! { match &(inEQ.clone()) {
        Deref @ SCode::Equation::EQ_PDE { expLeft: lhs_exp, expRight: rhs_exp, domain: domainCr @ Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, comment, info } => {
            let mut N: i32;
            let mut fieldLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            (N, fieldLst) = getDomNFields(inDomFieldLst, metamodelica::AsArg::as_arg(&domainCr), metamodelica::AsArg::as_arg(&info))?;
            creatFieldEqs(lhs_exp.clone(), rhs_exp.clone(), domainCr.clone(), N, fieldLst, comment.clone(), info.clone())?
        },
        Deref @ SCode::Equation::EQ_PDE { expLeft: lhs_exp, expRight: rhs_exp, domain: domainCr @ Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "interior", .. } }, comment, info } => {
            let mut domainCr1: metamodelica::Ref<Absyn::ComponentRef>;
            let mut N: i32;
            let mut fieldLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            domainCr1 = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: subscripts.clone() });
            (N, fieldLst) = getDomNFields(inDomFieldLst, &domainCr1, metamodelica::AsArg::as_arg(&info))?;
            creatFieldEqs(lhs_exp.clone(), rhs_exp.clone(), domainCr.clone(), N, fieldLst, comment.clone(), info.clone())?
        },
        Deref @ SCode::Equation::EQ_PDE { expLeft: lhs_exp, expRight: rhs_exp, domain: Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "left", .. } }, comment, info } => {
            let mut domainCr1: metamodelica::Ref<Absyn::ComponentRef>;
            let mut N: i32;
            let mut fieldLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            let mut lhs_exp = (*lhs_exp).clone();
            let mut rhs_exp = (*rhs_exp).clone();
            domainCr1 = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: subscripts.clone() });
            (N, fieldLst) = getDomNFields(inDomFieldLst, &domainCr1, metamodelica::AsArg::as_arg(&info))?;
            (lhs_exp, _) = AbsynUtil::traverseExp(lhs_exp.clone(), (std::sync::Arc::new(fnptr!(extrapFieldTraverseFun, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 1)?;
            (rhs_exp, _) = AbsynUtil::traverseExp(rhs_exp.clone(), (std::sync::Arc::new(fnptr!(extrapFieldTraverseFun, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), 1)?;
            list![newEQFun(1, lhs_exp.clone(), rhs_exp.clone(), domainCr1, N, true, fieldLst, comment.clone(), info.clone())?]
        },
        Deref @ SCode::Equation::EQ_PDE { expLeft: lhs_exp, expRight: rhs_exp, domain: Deref @ Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "right", .. } }, comment, info } => {
            let mut domainCr1: metamodelica::Ref<Absyn::ComponentRef>;
            let mut N: i32;
            let mut fieldLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            let mut lhs_exp = (*lhs_exp).clone();
            let mut rhs_exp = (*rhs_exp).clone();
            domainCr1 = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: subscripts.clone() });
            (N, fieldLst) = getDomNFields(inDomFieldLst, &domainCr1, metamodelica::AsArg::as_arg(&info))?;
            (lhs_exp, _) = AbsynUtil::traverseExp(lhs_exp.clone(), (std::sync::Arc::new(fnptr!(extrapFieldTraverseFun, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), N)?;
            (rhs_exp, _) = AbsynUtil::traverseExp(rhs_exp.clone(), (std::sync::Arc::new(fnptr!(extrapFieldTraverseFun, metamodelica::Ref<Absyn::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> + 'static>), N)?;
            list![newEQFun(N, lhs_exp.clone(), rhs_exp.clone(), domainCr1, N, true, fieldLst, comment.clone(), info.clone())?]
        },
        Deref @ SCode::Equation::EQ_PDE { .. } => {
            metamodelica::print(literal!("Unhandled type of EQ_PDE in discretizePDE\n"));
            return Err("fail");
            metamodelica::nil()
        },
        _ => {
            list![inEQ]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDiscretizedEQs = listAppend(inDiscretizedEQs, newDiscretizedEQs);
    Ok(outDiscretizedEQs)
}

fn extrapFieldTraverseFun(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inN: i32,
) -> (metamodelica::Ref<Absyn::Exp>, i32) {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outN: i32 = inN;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "extrapolateField", subscripts: Deref @ metamodelica::ListNode::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts } }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. } => {
            let mut i: i32;
            if inN == 1 {
                i = 1;
            } else {
                i = -1;
            }
            metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 2 }), op: openmodelica_ast::Absyn::Operator::MUL, exp2: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::cons(metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: inN }) }), subscripts.clone()) }) }) }), op: openmodelica_ast::Absyn::Operator::SUB, exp2: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::cons(metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: inN + i }) }), subscripts.clone()) }) }) })
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outN)
}

fn getDomNFields(
    mut inDomFieldLst: &DomainFieldsLst,
    mut inDomainCr: &metamodelica::Ref<Absyn::ComponentRef>,
    mut info: &SourceInfo,
) -> Result<(i32, metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>)> {
    let mut outN: i32 = 0;
    let mut outFieldLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::findSome(inDomFieldLst, &({ let __pe_b1 = inDomainCr.clone(); move |__pe_a0| Ok(domNFieldsFindFun(&__pe_a0, &__pe_b1)) }))) {
        Ok(Some((__pa0, __pa1))) => (__pa0.clone(), __pa1.clone()),
        _ => {
        Error::addSourceMessageAndFail(&(Error::COMPILER_ERROR.clone()), list![literal!("There are no fields defined within the domain of this equation.")], info)?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        },
    } };
    outN = metamodelica::Own::own(__pa0);
    outFieldLst = metamodelica::Own::own(__pa1);
    Ok((outN, outFieldLst))
}

fn domNFieldsFindFun(
    mut inDomFields: &(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    ),
    mut inDomainCr: &metamodelica::Ref<Absyn::ComponentRef>,
) -> Option<(i32, metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>)> {
    let mut outOptNFields: Option<(i32, metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>)>;
    outOptNFields = 'mc: {
        let __mc_input = inDomFields;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (domainCr, fieldCrLst) => {
                    let mut varLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut N: i32;
                    let true = (absynDAECrefEqualName(inDomainCr, metamodelica::AsArg::as_arg(&domainCr))) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(domainCr.clone()) {
                        Deref @ DAE::ComponentRef::CREF_IDENT { identType: Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, .. }, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    varLst = metamodelica::Own::own(__pa0);
                    let __pa2 = ::match_deref::match_deref! { match &(List::findSome(&varLst, &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(findN(&__a0)) })?) {
                        Some(__pa2) => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    N = metamodelica::Own::own(__pa2);
                    Ok(Some((N, fieldCrLst.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(None)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outOptNFields
}

fn absynDAECrefEqualName(
    mut domainCr1: &metamodelica::Ref<Absyn::ComponentRef>,
    mut domainCr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> bool {
    let mut equal: bool;
    let mut name1: ArcStr;
    let mut name2: ArcStr;
    equal = (::match_deref::match_deref! { match (domainCr1, domainCr2) {
        (Deref @ Absyn::ComponentRef::CREF_IDENT { name: name1, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: name2, .. }) if (stringEqual(&name1, &name2)) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    equal
}

fn extrapolateFieldEq(
    mut isRight: bool,
    mut fieldCr: metamodelica::Ref<Absyn::ComponentRef>,
    mut domainCr: &metamodelica::Ref<Absyn::ComponentRef>,
    mut N: i32,
    mut comment: metamodelica::Ref<SCode::Comment>,
    mut info: SourceInfo,
    mut fieldLst: &metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
) -> Result<metamodelica::Ref<SCode::Equation>> {
    let mut outEQ: metamodelica::Ref<SCode::Equation>;
    let mut name: ArcStr;
    let mut subscripts: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let mut i1: i32 = 1;
    let mut i2: i32 = 2;
    let mut i3: i32 = 3;
    if List::isMemberOnTrue(fieldCr.clone(), fieldLst, &move |__a0: metamodelica::Ref<
        Absyn::ComponentRef,
    >,
                                                              __a1: metamodelica::Ref<
        Absyn::ComponentRef,
    >| AbsynUtil::crefEqual(&__a0, &__a1))?
    {
        (name, subscripts) = (match &*fieldCr {
            Absyn::ComponentRef::CREF_IDENT {
                name: __esc_name,
                subscripts: __esc_subscripts,
            } => {
                name = (*__esc_name).clone();
                subscripts = (*__esc_subscripts).clone();
                (name.clone(), subscripts.clone())
            }
            _ => return Err("match: no arm matched"),
        });
        if isRight {
            i1 = N;
            i2 = N - 1;
            i3 = N - 2;
        }
        outEQ = metamodelica::Ref::new(SCode::Equation::EQ_EQUALS {
            expLeft: metamodelica::Ref::new(Absyn::Exp::CREF {
                componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                    name: name.clone(),
                    subscripts: metamodelica::cons(
                        metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT {
                            subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i1 }),
                        }),
                        subscripts.clone(),
                    ),
                }),
            }),
            expRight: metamodelica::Ref::new(Absyn::Exp::BINARY {
                exp1: metamodelica::Ref::new(Absyn::Exp::BINARY {
                    exp1: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 2 }),
                    op: openmodelica_ast::Absyn::Operator::MUL,
                    exp2: metamodelica::Ref::new(Absyn::Exp::CREF {
                        componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                            name: name.clone(),
                            subscripts: metamodelica::cons(
                                metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT {
                                    subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i2 }),
                                }),
                                subscripts.clone(),
                            ),
                        }),
                    }),
                }),
                op: openmodelica_ast::Absyn::Operator::SUB,
                exp2: metamodelica::Ref::new(Absyn::Exp::CREF {
                    componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                        name: name,
                        subscripts: metamodelica::cons(
                            metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT {
                                subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i3 }),
                            }),
                            subscripts,
                        ),
                    }),
                }),
            }),
            comment: comment,
            info: info,
        });
    } else {
        return Err("fail");
    }
    Ok(outEQ)
}

fn creatFieldEqs(
    mut lhs_exp: metamodelica::Ref<Absyn::Exp>,
    mut rhs_exp: metamodelica::Ref<Absyn::Exp>,
    mut domainCr: metamodelica::Ref<Absyn::ComponentRef>,
    mut N: i32,
    mut fieldLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    mut comment: metamodelica::Ref<SCode::Comment>,
    mut info: SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Equation>>> {
    let mut outDiscretizedEQs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut bl: bool;
    let mut br: bool;
    (_, bl) = AbsynUtil::traverseExp(
        lhs_exp.clone(),
        (std::sync::Arc::new(fnptr!(hasPderTraverseFun, metamodelica::Ref<Absyn::Exp>, bool))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, bool) -> Result<(metamodelica::Ref<Absyn::Exp>, bool)>
                    + 'static,
            >),
        false,
    )?;
    (_, br) = AbsynUtil::traverseExp(
        rhs_exp.clone(),
        (std::sync::Arc::new(fnptr!(hasPderTraverseFun, metamodelica::Ref<Absyn::Exp>, bool))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, bool) -> Result<(metamodelica::Ref<Absyn::Exp>, bool)>
                    + 'static,
            >),
        false,
    )?;
    outDiscretizedEQs = (match (bl, br) {
        (false, false) => {
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
                for mut i in (1..=N).into_iter() {
                    let __x = newEQFun(
                        i.clone(),
                        lhs_exp.clone(),
                        rhs_exp.clone(),
                        domainCr.clone(),
                        N,
                        false,
                        fieldLst.clone(),
                        comment.clone(),
                        info.clone(),
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
        _ => {
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
                for mut i in (1..=N).into_iter() {
                    let __x = newEQFun(
                        i.clone(),
                        lhs_exp.clone(),
                        rhs_exp.clone(),
                        domainCr.clone(),
                        N,
                        false,
                        fieldLst.clone(),
                        comment.clone(),
                        info.clone(),
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
    });
    Ok(outDiscretizedEQs)
}

fn hasPderTraverseFun(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inHasPder: bool,
) -> (metamodelica::Ref<Absyn::Exp>, bool) {
    let mut outExp: metamodelica::Ref<Absyn::Exp> = inExp.clone();
    let mut outHasPder: bool;
    outHasPder = (::match_deref::match_deref! { match &((inExp, inHasPder)) {
        (_, true) => true,
        (Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", .. }, .. }, _) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outHasPder)
}

fn newEQFun(
    mut i: i32,
    mut inLhs_exp: metamodelica::Ref<Absyn::Exp>,
    mut inRhs_exp: metamodelica::Ref<Absyn::Exp>,
    mut domainCr: metamodelica::Ref<Absyn::ComponentRef>,
    mut N: i32,
    mut isBC: bool,
    mut fieldLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    mut comment: metamodelica::Ref<SCode::Comment>,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<SCode::Equation>> {
    let mut outEQ: metamodelica::Ref<SCode::Equation>;
    let mut outLhs_exp: metamodelica::Ref<Absyn::Exp>;
    let mut outRhs_exp: metamodelica::Ref<Absyn::Exp>;
    (outLhs_exp, _) = AbsynUtil::traverseExpTopDown(
        inLhs_exp,
        (std::sync::Arc::new(discretizeTraverseFun)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        (
                            i32,
                            metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
                            metamodelica::Ref<Absyn::ComponentRef>,
                            SourceInfo,
                            bool,
                            i32,
                            bool,
                        ),
                    ) -> Result<(
                        metamodelica::Ref<Absyn::Exp>,
                        (
                            i32,
                            metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
                            metamodelica::Ref<Absyn::ComponentRef>,
                            SourceInfo,
                            bool,
                            i32,
                            bool,
                        ),
                    )> + 'static,
            >),
        (i, fieldLst.clone(), domainCr.clone(), info.clone(), false, N, isBC),
    )?;
    (outRhs_exp, _) = AbsynUtil::traverseExpTopDown(
        inRhs_exp,
        (std::sync::Arc::new(discretizeTraverseFun)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        (
                            i32,
                            metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
                            metamodelica::Ref<Absyn::ComponentRef>,
                            SourceInfo,
                            bool,
                            i32,
                            bool,
                        ),
                    ) -> Result<(
                        metamodelica::Ref<Absyn::Exp>,
                        (
                            i32,
                            metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
                            metamodelica::Ref<Absyn::ComponentRef>,
                            SourceInfo,
                            bool,
                            i32,
                            bool,
                        ),
                    )> + 'static,
            >),
        (i, fieldLst, domainCr, info.clone(), false, N, isBC),
    )?;
    outEQ = metamodelica::Ref::new(SCode::Equation::EQ_EQUALS {
        expLeft: outLhs_exp,
        expRight: outRhs_exp,
        comment: comment,
        info: info,
    });
    Ok(outEQ)
}

fn discretizeTraverseFun(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inTup: (
        i32,
        metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
        metamodelica::Ref<Absyn::ComponentRef>,
        SourceInfo,
        bool,
        i32,
        bool,
    ),
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    (
        i32,
        metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
        metamodelica::Ref<Absyn::ComponentRef>,
        SourceInfo,
        bool,
        i32,
        bool,
    ),
)> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outTup: (
        i32,
        metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
        metamodelica::Ref<Absyn::ComponentRef>,
        SourceInfo,
        bool,
        i32,
        bool,
    );
    let mut i: i32;
    let mut N: i32;
    let mut fieldLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    let mut info: SourceInfo;
    let mut skip: bool;
    let mut failVar: bool;
    let mut isBC: bool;
    let mut domainCr: metamodelica::Ref<Absyn::ComponentRef>;
    let mut domName: ArcStr;
    failVar = false;
    (i, fieldLst, domainCr, info, skip, N, isBC) = inTup.clone();
    let __pa0 = ::match_deref::match_deref! { match &(domainCr.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    domName = metamodelica::Own::own(__pa0);
    if skip {
        outExp = inExp;
        outTup = inTup;
        return Ok((outExp, outTup));
    }
    outExp = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_QUAL { name: domName, subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "x", subscripts: Deref @ metamodelica::ListNode::Nil } } } => {
                    Ok(metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: domName.clone(), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("x"), subscripts: list![metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i }) })] }) }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CREF { componentRef: fieldCr @ Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts } } => {
                    let mut exp: metamodelica::Ref<Absyn::Exp>;
                    let true = (List::isMemberOnTrue(fieldCr.clone(), &fieldLst, &move |__a0: metamodelica::Ref<Absyn::ComponentRef>, __a1: metamodelica::Ref<Absyn::ComponentRef>| AbsynUtil::crefEqual(&__a0, &__a1))?) else { return Err("pattern mismatch") };
                    exp = if (isBC && i == 1) {metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: stringAppend(name.clone(), literal!("$ghostL")), subscripts: subscripts.clone() }) })} else if (isBC && i == N) {metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: stringAppend(name.clone(), literal!("$ghostR")), subscripts: subscripts.clone() }) })} else {metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::cons(metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i }) }), subscripts.clone()) }) })};
                    Ok(exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", subscripts: Deref @ metamodelica::ListNode::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: fieldCr @ Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "x", .. } }, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: _ }, typeVars: Deref @ metamodelica::ListNode::Nil } => {
                    let mut leftVar: metamodelica::Ref<Absyn::Exp>;
                    let mut rightVar: metamodelica::Ref<Absyn::Exp>;
                    let mut failVar: bool = failVar.clone();
                    if !(List::isMemberOnTrue(fieldCr.clone(), &fieldLst, &move |__a0: metamodelica::Ref<Absyn::ComponentRef>, __a1: metamodelica::Ref<Absyn::ComponentRef>| AbsynUtil::crefEqual(&__a0, &__a1))?) {
                        failVar = true;
                        Error::addSourceMessageAndFail(&(Error::COMPILER_ERROR.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Field variable '")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("' has different domain than the equation or is not a field.")); ArcStr::from(__mm_s) }], &info)?;
                        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    }
                    leftVar = if (i == 1) {metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: stringAppend(name.clone(), literal!("$ghostL")), subscripts: subscripts.clone() }) })} else {metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::cons(metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i - 1 }) }), subscripts.clone()) }) })};
                    rightVar = if (i == N) {metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: stringAppend(name.clone(), literal!("$ghostR")), subscripts: subscripts.clone() }) })} else {metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::cons(metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i + 1 }) }), subscripts.clone()) }) })};
                    Ok((metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: rightVar.clone(), op: openmodelica_ast::Absyn::Operator::SUB, exp2: leftVar.clone() }), op: openmodelica_ast::Absyn::Operator::DIV, exp2: metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 2 }), op: openmodelica_ast::Absyn::Operator::MUL, exp2: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: domName.clone(), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("dx"), subscripts: metamodelica::nil() }) }) }) }) }), failVar.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            failVar = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", subscripts: Deref @ metamodelica::ListNode::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: fieldCr @ Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "x", .. } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "x", .. } }, tail: Deref @ metamodelica::ListNode::Nil } } }, argNames: _ }, typeVars: Deref @ metamodelica::ListNode::Nil } => {
                    let mut leftVar: metamodelica::Ref<Absyn::Exp>;
                    let mut actualVar: metamodelica::Ref<Absyn::Exp>;
                    let mut rightVar: metamodelica::Ref<Absyn::Exp>;
                    let mut failVar: bool = failVar.clone();
                    if !(List::isMemberOnTrue(fieldCr.clone(), &fieldLst, &move |__a0: metamodelica::Ref<Absyn::ComponentRef>, __a1: metamodelica::Ref<Absyn::ComponentRef>| AbsynUtil::crefEqual(&__a0, &__a1))?) {
                        failVar = true;
                        Error::addSourceMessageAndFail(&(Error::COMPILER_ERROR.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Field variable '")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("' has different domain than the equation or is not a field.")); ArcStr::from(__mm_s) }], &info)?;
                        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    }
                    leftVar = if (i == 1) {metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: stringAppend(name.clone(), literal!("$ghostL")), subscripts: subscripts.clone() }) })} else {metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::cons(metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i - 1 }) }), subscripts.clone()) }) })};
                    actualVar = metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::cons(metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i }) }), subscripts.clone()) }) });
                    rightVar = if (i == N) {metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: stringAppend(name.clone(), literal!("$ghostR")), subscripts: subscripts.clone() }) })} else {metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::cons(metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i + 1 }) }), subscripts.clone()) }) })};
                    Ok((metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: leftVar.clone(), op: openmodelica_ast::Absyn::Operator::SUB, exp2: metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 2 }), op: openmodelica_ast::Absyn::Operator::MUL, exp2: actualVar.clone() }) }), op: openmodelica_ast::Absyn::Operator::ADD, exp2: rightVar.clone() }), op: openmodelica_ast::Absyn::Operator::DIV, exp2: metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: domName.clone(), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("dx"), subscripts: metamodelica::nil() }) }) }), op: openmodelica_ast::Absyn::Operator::POW, exp2: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 2 }) }) }), failVar.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            failVar = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", subscripts: Deref @ metamodelica::ListNode::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: _ }, .. } => {
                    Error::addSourceMessageAndFail(&(Error::COMPILER_ERROR.clone()), list![literal!("You are differentiating with respect to variable that is not a coordinate.")], &info)?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    Ok(inExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "pder", subscripts: Deref @ metamodelica::ListNode::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: _ }, .. } => {
                    Error::addSourceMessageAndFail(&(Error::COMPILER_ERROR.clone()), list![literal!("Unsupported partial derivative.")], &info)?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    Ok(inExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    if failVar {
        return Err("fail");
    }
    outTup = (i, fieldLst, domainCr, info, skip, N, isBC);
    Ok((outExp, outTup))
}

fn findDomF<T: Clone + 'static + metamodelica::gc::MMTrace>(mut inTup: &(ArcStr, T), mut name: &ArcStr) -> bool {
    let mut found: bool;
    found = (match inTup.clone() {
        (mut nameLoc, _) if (stringEqual(&nameLoc, &name)) => true,
        _ => false,
    });
    found
}

/*
public function findDomains
  input SCode.Element el;
  input list<tuple<String,Integer>> domainLstIn;
  output list<tuple<String,Integer>> domainLstOut;
algorithm
//TODO: rewrite to use instantiated domain elements
  domainLstOut := match el
    local
      String name;
      list<SCode.SubMod> subModLst;
      Integer N;
    case SCode.COMPONENT(typeSpec=Absyn.TPATH(path=Absyn.IDENT(name="DomainLineSegment1D")), name = name,
      modifications = SCode.MOD(subModLst = subModLst))
      then
        (name,findDomains1(subModLst))::domainLstIn;
      else
      domainLstIn;
  end match;
end findDomains;


protected function findDomains1
  input list<SCode.SubMod> subModLst;
  output Integer N;
algorithm
  try
    N := match List.find(subModLst,findDomains2)
    local
      Integer n;
    case SCode.NAMEMOD("N",SCode.MOD(binding = SOME(Absyn.INTEGER(n))))
    then
      n;
    end match;
  else
    print("\nError: Variable N not found in the domain.\n");
    fail();
  end try;
end findDomains1;

protected function findDomains2
  input SCode.SubMod subMod;
  output Boolean found;
algorithm
  found := match subMod
    case SCode.NAMEMOD(ident = "N")
      then
        true;
      else
        false;
  end match;
end findDomains2;
*/
