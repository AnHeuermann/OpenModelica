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

use crate::ConnectionGraph;
use crate::OperatorOverloading;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Flags;
use openmodelica_util::Global;

pub type Key = metamodelica::Ref<Absyn::Path>;

pub type Value = metamodelica::List<Option<CachedInstItem>>;

pub type CachedInstItemInputs = (
    metamodelica::Ref<DAE::Mod>,
    DAE::Prefix,
    DAE::Connect::Sets,
    ClassInf::State,
    metamodelica::Ref<SCode::Element>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    bool,
    Option<metamodelica::Ref<DAE::ComponentRef>>,
    InstTypes::CallingScope,
);

pub type CachedInstItemOutputs = (
    FCore::Graph,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
    Option<metamodelica::Ref<DAE::Type>>,
    Option<SCode::Attributes>,
    Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>,
    ConnectionGraph::ConnectionGraph,
);

pub type CachedPartialInstItemInputs = (
    metamodelica::Ref<DAE::Mod>,
    DAE::Prefix,
    ClassInf::State,
    metamodelica::Ref<SCode::Element>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
);

pub type CachedPartialInstItemOutputs = (
    FCore::Graph,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
);

pub type CachedInstItems = metamodelica::List<Option<CachedInstItem>>;

pub fn init() -> Result<()> {
    let mut ht: HashTable;
    if '__try0: {
        ht = crate::Globals::instHashIndex.with(|__root| __root.borrow().clone());
        ht = unwrap_break_err!(BaseHashTable::clear(ht.clone()), '__try0);
        {
            let __v = ht.clone();
            crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = __v)
        };
        Ok::<(), &'static str>(())
    }
    .is_err()
    {
        {
            let __v = emptyInstHashTable()?;
            crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = __v)
        };
    }
    Ok(())
}

pub fn release() -> Result<()> {
    {
        let __v = emptyInstHashTable()?;
        crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = __v)
    };
    OperatorOverloading::initCache();
    Ok(())
}

pub(crate) fn get(mut k: Key) -> Result<Value> {
    let mut v: Value;
    let mut ht: HashTable;
    ht = crate::Globals::instHashIndex.with(|__root| __root.borrow().clone());
    v = BaseHashTable::get(k, &ht)?;
    Ok(v)
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum CachedInstItem {
    FUNC_instClassIn {
        inputs: CachedInstItemInputs,
        outputs: CachedInstItemOutputs,
    },
    FUNC_partialInstClassIn {
        inputs: CachedPartialInstItemInputs,
        outputs: CachedPartialInstItemOutputs,
    },
}
impl metamodelica::gc::MMTrace for CachedInstItem {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            CachedInstItem::FUNC_instClassIn { inputs, outputs } => {
                metamodelica::gc::MMTrace::mm_accept(inputs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(outputs, __mmv)?;
                Ok(())
            }
            CachedInstItem::FUNC_partialInstClassIn { inputs, outputs } => {
                metamodelica::gc::MMTrace::mm_accept(inputs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(outputs, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::CachedInstItem::{FUNC_instClassIn, FUNC_partialInstClassIn};

pub(crate) fn addToInstCache(
    mut fullEnvPathPlusClass: metamodelica::Ref<Absyn::Path>,
    mut fullInstOpt: Option<CachedInstItem>,
    mut partialInstOpt: Option<CachedInstItem>,
) -> () {
    let () = 'mc: {
        let __mc_input = (fullInstOpt.clone(), partialInstOpt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let false = (Flags::isSet(Flags::CACHE.clone())?) else {
                return Err("pattern mismatch");
            };
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(_), Some(_)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut instHash: HashTable;
            instHash = crate::Globals::instHashIndex.with(|__root| __root.borrow().clone());
            instHash = BaseHashTable::add(
                (
                    fullEnvPathPlusClass.clone(),
                    list![fullInstOpt.clone(), partialInstOpt.clone()],
                ),
                instHash.clone(),
            )?;
            {
                let __v = instHash.clone();
                crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (None, Some(_)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut instHash: HashTable;
            let mut opt: Option<CachedInstItem>;
            instHash = crate::Globals::instHashIndex.with(|__root| __root.borrow().clone());
            let __pa0 = ::match_deref::match_deref! { match &(BaseHashTable::get(fullEnvPathPlusClass.clone(), &instHash)?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            opt = metamodelica::Own::own(__pa0);
            instHash = BaseHashTable::add(
                (fullEnvPathPlusClass.clone(), list![opt.clone(), partialInstOpt.clone()]),
                instHash.clone(),
            )?;
            {
                let __v = instHash.clone();
                crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (None, Some(_)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut instHash: HashTable;
            instHash = crate::Globals::instHashIndex.with(|__root| __root.borrow().clone());
            instHash = BaseHashTable::add(
                (fullEnvPathPlusClass.clone(), list![None, partialInstOpt.clone()]),
                instHash.clone(),
            )?;
            {
                let __v = instHash.clone();
                crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(_), None) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut instHash: HashTable;
            let mut lst: metamodelica::List<Option<CachedInstItem>>;
            instHash = crate::Globals::instHashIndex.with(|__root| __root.borrow().clone());
            let __pa0 = ::match_deref::match_deref! { match &(BaseHashTable::get(fullEnvPathPlusClass.clone(), &instHash)?) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 @ Deref @ metamodelica::ListNode::Cons { head: Some(_), tail: Deref @ metamodelica::ListNode::Nil } } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            lst = metamodelica::Own::own(__pa0);
            instHash = BaseHashTable::add(
                (
                    fullEnvPathPlusClass.clone(),
                    metamodelica::cons(fullInstOpt.clone(), lst.clone()),
                ),
                instHash.clone(),
            )?;
            {
                let __v = instHash.clone();
                crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(_), None) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut instHash: HashTable;
            instHash = crate::Globals::instHashIndex.with(|__root| __root.borrow().clone());
            instHash = BaseHashTable::add(
                (fullEnvPathPlusClass.clone(), list![fullInstOpt.clone(), None]),
                instHash.clone(),
            )?;
            {
                let __v = instHash.clone();
                crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = __v)
            };
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

pub type HashTableKeyFunctionsType = (FuncHashKey, FuncKeyEqual, FuncKeyStr, FuncValueStr);

pub type HashTable = (
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<Absyn::Path>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<Absyn::Path>,
                metamodelica::List<Option<CachedInstItem>>,
            )>,
        >,
    ),
    i32,
    (FuncHashKey, FuncKeyEqual, FuncKeyStr, FuncValueStr),
);

type FuncHashKey = std::sync::Arc<dyn ::std::ops::Fn(Key) -> Result<i32> + 'static>;

type FuncKeyEqual = std::sync::Arc<dyn ::std::ops::Fn(Key, Key) -> Result<bool> + 'static>;

type FuncKeyStr = std::sync::Arc<dyn ::std::ops::Fn(Key) -> Result<ArcStr> + 'static>;

type FuncValueStr = std::sync::Arc<dyn ::std::ops::Fn(Value) -> Result<ArcStr> + 'static>;

fn opaqVal(mut v: &Value) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = literal!("OPAQUE_VALUE");
    r#str
}

fn emptyInstHashTable() -> Result<HashTable> {
    let mut hashTable: HashTable;
    hashTable = emptyInstHashTableSized(Flags::getConfigInt(Flags::INST_CACHE_SIZE.clone())?);
    OperatorOverloading::initCache();
    Ok(hashTable)
}

fn emptyInstHashTableSized(mut size: i32) -> HashTable {
    let mut hashTable: HashTable;
    hashTable = BaseHashTable::emptyHashTableWork(
        size,
        (
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
            })
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static>),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<Absyn::Path>,
                      __a1: metamodelica::Ref<Absyn::Path>|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>) -> Result<bool>
                        + 'static,
                >),
            (std::sync::Arc::new(AbsynUtil::pathStringDefault)
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>),
            (std::sync::Arc::new(
                move |__a0: metamodelica::List<Option<CachedInstItem>>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(opaqVal(&__a0))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::List<Option<CachedInstItem>>) -> Result<ArcStr> + 'static,
                >),
        ),
    );
    hashTable
}
