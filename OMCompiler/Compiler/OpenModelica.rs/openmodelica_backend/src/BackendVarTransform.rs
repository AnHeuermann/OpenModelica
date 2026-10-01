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

use crate::BackendDAETransform;
use crate::BackendEquation;
use crate::BackendVariable;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend::HashSet;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

pub type ExpOpt = Option<metamodelica::Ref<DAE::Exp>>;

pub type CrefExpTable = metamodelica::Ref<
    UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, Option<metamodelica::Ref<DAE::Exp>>>,
>;

pub type CrefList = metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;

pub type CrefListOpt = Option<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;

pub type CrefCrefListTable = metamodelica::Ref<
    UnorderedMap::UnorderedMap<
        metamodelica::Ref<DAE::ComponentRef>,
        Option<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
    >,
>;

pub type CrefSet = metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<DAE::ComponentRef>>>;

/// VariableReplacements consists of a mapping between variables and expressions, the first binary tree of this type.
/// To eliminate a variable from an equation system a replacement rule varname->expression is added to this
/// datatype.
/// To be able to update these replacement rules incrementally a backward lookup mechanism is also required.
/// For instance, having a rule a->b and adding a rule b->c requires to find the first rule a->b and update it to
/// a->c. This is what the second binary tree is used for.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct VariableReplacements {
    /// src -> dst, used for replacing. src is variable, dst is expression.
    pub hashTable: CrefExpTable,
    /// dst -> list of sources. dst is a variable, sources are variables.
    pub invHashTable: CrefCrefListTable,
    /// src -> nothing, used for extend arrays and records.
    pub extendhashTable: CrefSet,
    /// this are the implicit declerate iteration variables for for and range expressions
    pub iterationVars: metamodelica::List<ArcStr>,
    /// this is used if states are constant to replace der(state) with 0.0
    pub derConst: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, Option<metamodelica::Ref<DAE::Exp>>>,
        >,
    >,
}

impl metamodelica::gc::MMTrace for VariableReplacements {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.hashTable, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.invHashTable, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.extendhashTable, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.iterationVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.derConst, __mmv)?;
        Ok(())
    }
}
impl Default for VariableReplacements {
    fn default() -> Self {
        Self {
            hashTable: Default::default(),
            invHashTable: Default::default(),
            extendhashTable: Default::default(),
            iterationVars: Default::default(),
            derConst: Default::default(),
        }
    }
}

pub type REPLACEMENTS = VariableReplacements;

pub type FuncTypeExp_ExpToBoolean =
    std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

pub(crate) fn newCrefExpTable() -> CrefExpTable {
    let mut table: CrefExpTable = UnorderedMap::new(
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
        1,
    );
    table
}

pub(crate) fn newCrefExpTableSized(mut size: i32) -> CrefExpTable {
    let mut table: CrefExpTable = UnorderedMap::new(
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
        size,
    );
    table
}

pub(crate) fn newCrefCrefListTable() -> CrefCrefListTable {
    let mut table: CrefCrefListTable = UnorderedMap::new(
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
        1,
    );
    table
}

pub(crate) fn newCrefCrefListTableSized(mut size: i32) -> CrefCrefListTable {
    let mut table: CrefCrefListTable = UnorderedMap::new(
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
        size,
    );
    table
}

pub(crate) fn newCrefSet() -> CrefSet {
    let mut set: CrefSet = UnorderedSet::new(
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
    set
}

pub(crate) fn newCrefSetSized(mut size: i32) -> CrefSet {
    let mut set: CrefSet = UnorderedSet::new(
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
        size,
    );
    set
}

pub fn emptyReplacements() -> VariableReplacements {
    let mut outVariableReplacements: VariableReplacements = VariableReplacements {
        hashTable: newCrefExpTable(),
        invHashTable: newCrefCrefListTable(),
        extendhashTable: newCrefSet(),
        iterationVars: metamodelica::nil(),
        derConst: None,
    };
    outVariableReplacements
}

pub(crate) fn emptyReplacementsSized(mut size: i32) -> VariableReplacements {
    let mut outVariableReplacements: VariableReplacements = VariableReplacements {
        hashTable: newCrefExpTableSized(size),
        invHashTable: newCrefCrefListTableSized(size),
        extendhashTable: newCrefSetSized(size),
        iterationVars: metamodelica::nil(),
        derConst: None,
    };
    outVariableReplacements
}

pub(crate) fn removeReplacement(
    mut repl: &VariableReplacements,
    mut inSrc: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<()> {
    let mut dst_opt: Option<metamodelica::Ref<DAE::Exp>>;
    dst_opt = UnorderedMap::getOrDefault(inSrc.clone(), repl.hashTable.clone(), None)?;
    if (dst_opt).is_none() {
        return Ok(());
    }
    if '__try0: {
        unwrap_break_err!(UnorderedMap::add(inSrc.clone(), None, repl.hashTable.clone()), '__try0);
        unwrap_break_err!(removeReplacementInv(repl.invHashTable.clone(), unwrap_break_err!(Util::getOption(dst_opt.clone()), '__try0)), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
        Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-BackendVarTransform.removeReplacement failed for ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&inSrc)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/BackendVarTransform.mo"))?;
    }
    Ok(())
}

pub(crate) fn removeReplacements(
    mut iRepl: &VariableReplacements,
    mut inSrcs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<()> {
    for mut cr in &**inSrcs {
        removeReplacement(iRepl, cr.clone())?;
    }
    Ok(())
}

pub(crate) fn addReplacements<'__b>(
    mut iRepl: VariableReplacements,
    mut inSrcs: &'__b metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inDsts: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> Result<VariableReplacements> {
    '__tco: loop {
        ::match_deref::match_deref! { match (inSrcs, inDsts) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(iRepl)
            },
            (Deref @ metamodelica::ListNode::Cons { head: cr, tail: crlst }, Deref @ metamodelica::ListNode::Cons { head: exp, tail: explst }) => {
                let mut repl: VariableReplacements;
                repl = addReplacement(iRepl, cr.clone(), exp.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                { (iRepl, inSrcs, inDsts, inFuncTypeExpExpToBooleanOption) = (repl, crlst, explst, inFuncTypeExpExpToBooleanOption); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn addReplacement(
    mut repl: VariableReplacements,
    mut inSrc: metamodelica::Ref<DAE::ComponentRef>,
    mut inDst: metamodelica::Ref<DAE::Exp>,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> Result<VariableReplacements> {
    let mut outRepl: VariableReplacements;
    outRepl = 'mc: {
        let __mc_input = (inSrc.clone(), inDst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (src, dst) => {
                    let mut ht: CrefExpTable;
                    let mut eht: CrefSet;
                    let mut invHt: CrefCrefListTable;
                    let mut iv: metamodelica::List<ArcStr>;
                    let mut derConst: Option<metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, Option<metamodelica::Ref<DAE::Exp>>>>>;
                    let mut src = (*src).clone();
                    let mut dst = (*dst).clone();
                    let (VariableReplacements { hashTable: __pa0, invHashTable: __pa1, extendhashTable: __pa2, iterationVars: __pa3, derConst: __pa4 }, __pa5, __pa6) = makeTransitive(repl.clone(), src.clone(), dst.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    ht = metamodelica::Own::own(__pa0);
                    invHt = metamodelica::Own::own(__pa1);
                    eht = metamodelica::Own::own(__pa2);
                    iv = metamodelica::Own::own(__pa3);
                    derConst = metamodelica::Own::own(__pa4);
                    src = metamodelica::Own::own(__pa5);
                    dst = metamodelica::Own::own(__pa6);
                    UnorderedMap::add(src.clone(), Some(dst.clone()), ht.clone())?;
                    invHt = addReplacementInv(invHt.clone(), src.clone(), dst.clone())?;
                    eht = addExtendReplacement(eht.clone(), src.clone(), None)?;
                    Ok(VariableReplacements { hashTable: ht.clone(), invHashTable: invHt.clone(), extendhashTable: eht.clone(), iterationVars: iv.clone(), derConst: derConst.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut s: ArcStr;
                    s = ComponentReferenceBasics::printComponentRefStr(&inSrc)?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-BackendVarTransform.addReplacement failed for ")); __mm_s.push_str(&*s); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outRepl)
}

pub(crate) fn performReplacementsEqSystem(
    mut inEqs: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inRepl: VariableReplacements,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut outEqs: metamodelica::Ref<BackendDAE::EqSystem> = inEqs.clone();
    let mut eqArr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    eqArr = inEqs.orderedEqs.clone();
    BackendVariable::traverseBackendDAEVarsWithUpdate(
        inEqs.orderedVars.clone(),
        (std::sync::Arc::new(replaceVarTraverser)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        VariableReplacements,
                    )
                        -> Result<(metamodelica::Ref<BackendDAE::Var>, VariableReplacements)>
                    + 'static,
            >),
        inRepl.clone(),
    )?;
    (eqArr, _) = replaceEquationsArr(eqArr, inRepl, None);
    assign_field!(outEqs.orderedEqs = eqArr);
    Ok(outEqs)
}

fn addReplacementNoTransitive(
    mut repl: &VariableReplacements,
    mut inSrc: metamodelica::Ref<DAE::ComponentRef>,
    mut inDst: metamodelica::Ref<DAE::Exp>,
) -> Result<VariableReplacements> {
    let mut outRepl: VariableReplacements;
    outRepl = 'mc: {
        let __mc_input = (repl, inSrc.clone(), inDst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (VariableReplacements { hashTable: ht, .. }, src, _) => {
                    if !((((UnorderedMap::getOrDefault(src.clone(), ht.clone(), None)?)).is_some())) { return Err("guard") }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (VariableReplacements { hashTable: ht, invHashTable: invHt, extendhashTable: eht, iterationVars: iv, derConst }, src, dst) => {
                    let mut invHt = (*invHt).clone();
                    let mut eht = (*eht).clone();
                    UnorderedMap::add(src.clone(), Some(dst.clone()), ht.clone())?;
                    invHt = addReplacementInv(invHt.clone(), src.clone(), dst.clone())?;
                    eht = addExtendReplacement(eht.clone(), src.clone(), None)?;
                    Ok(VariableReplacements { hashTable: ht.clone(), invHashTable: invHt.clone(), extendhashTable: eht.clone(), iterationVars: iv.clone(), derConst: derConst.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-add_replacement failed for ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&inSrc)?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inDst.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outRepl)
}

fn removeReplacementInv(mut invHt: CrefCrefListTable, mut dst: metamodelica::Ref<DAE::Exp>) -> Result<()> {
    for mut d in &*Expression::extractCrefsFromExp(dst)? {
        UnorderedMap::tryUpdate(d.clone(), None, invHt.clone())?;
    }
    Ok(())
}

fn addReplacementInv(
    mut invHt: CrefCrefListTable,
    mut src: metamodelica::Ref<DAE::ComponentRef>,
    mut dst: metamodelica::Ref<DAE::Exp>,
) -> Result<CrefCrefListTable> {
    let mut invHt: CrefCrefListTable = invHt;
    for mut d in &*Expression::extractCrefsFromExp(dst)? {
        invHt = addReplacementInv2(invHt, d.clone(), src.clone())?;
    }
    Ok(invHt)
}

fn addReplacementInv2(
    mut invHt: CrefCrefListTable,
    mut dst: metamodelica::Ref<DAE::ComponentRef>,
    mut src: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<CrefCrefListTable> {
    let mut invHt: CrefCrefListTable = invHt;
    let mut srcs_opt: Option<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let mut srcs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    srcs_opt = UnorderedMap::getOrDefault(dst.clone(), invHt.clone(), None)?;
    if (srcs_opt).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(srcs_opt) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        srcs = metamodelica::Own::own(__pa0);
        srcs = metamodelica::cons(src, srcs);
        UnorderedMap::add(dst, Some(srcs), invHt.clone())?;
    } else {
        UnorderedMap::add(dst, Some(list![src]), invHt.clone())?;
    }
    Ok(invHt)
}

fn makeTransitive(
    mut repl: VariableReplacements,
    mut src: metamodelica::Ref<DAE::ComponentRef>,
    mut dst: metamodelica::Ref<DAE::Exp>,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> Result<(
    VariableReplacements,
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::Exp>,
)> {
    let mut outRepl: VariableReplacements;
    let mut outSrc: metamodelica::Ref<DAE::ComponentRef>;
    let mut outDst: metamodelica::Ref<DAE::Exp>;
    (outRepl, outSrc, outDst) = (match inFuncTypeExpExpToBooleanOption.clone() {
        _ => {
            let mut repl_1: VariableReplacements;
            let mut repl_2: VariableReplacements;
            let mut src_1: metamodelica::Ref<DAE::ComponentRef>;
            let mut src_2: metamodelica::Ref<DAE::ComponentRef>;
            let mut dst_1: metamodelica::Ref<DAE::Exp>;
            let mut dst_2: metamodelica::Ref<DAE::Exp>;
            let mut dst_3: metamodelica::Ref<DAE::Exp>;
            (repl_1, src_1, dst_1) = makeTransitive1(repl, src, dst, inFuncTypeExpExpToBooleanOption.clone());
            (repl_2, src_2, dst_2) = makeTransitive2(repl_1, src_1, dst_1, inFuncTypeExpExpToBooleanOption);
            (dst_3, _) = ExpressionSimplify::simplify1(dst_2)?;
            (repl_2, src_2, dst_3)
        }
    });
    Ok((outRepl, outSrc, outDst))
}

fn makeTransitive1(
    mut repl: VariableReplacements,
    mut src: metamodelica::Ref<DAE::ComponentRef>,
    mut dst: metamodelica::Ref<DAE::Exp>,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> (
    VariableReplacements,
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::Exp>,
) {
    let mut outRepl: VariableReplacements;
    let mut outSrc: metamodelica::Ref<DAE::ComponentRef>;
    let mut outDst: metamodelica::Ref<DAE::Exp>;
    (outRepl, outSrc, outDst) = 'mc: {
        let __mc_input = repl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let VariableReplacements {
                invHashTable: ref invHt,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut repl_1: VariableReplacements;
            let mut singleRepl: VariableReplacements;
            let __pa0 = ::match_deref::match_deref! { match &(UnorderedMap::getOrFail(src.clone(), invHt.clone())?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            lst = metamodelica::Own::own(__pa0);
            singleRepl = addReplacementNoTransitive(&(emptyReplacementsSized(53)), src.clone(), dst.clone())?;
            repl_1 = makeTransitive12(
                &lst,
                repl.clone(),
                &singleRepl,
                inFuncTypeExpExpToBooleanOption.clone(),
                HashSet::emptyHashSet(),
            )?;
            Ok((repl_1.clone(), src.clone(), dst.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((repl.clone(), src.clone(), dst.clone()))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outRepl, outSrc, outDst)
}

fn makeTransitive12(
    mut lst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut repl: VariableReplacements,
    mut singleRepl: &VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
    mut inSet: (
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
) -> Result<VariableReplacements> {
    let mut outRepl: VariableReplacements = repl;
    let mut set: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ) = inSet;
    let mut crDst: metamodelica::Ref<DAE::Exp>;
    for mut cr in &**lst {
        if !(BaseHashSet::has(cr.clone(), &set)?) {
            if '__try0: {
                set = unwrap_break_err!(BaseHashSet::add(cr.clone(), &set), '__try0);
                let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(UnorderedMap::getOrFail(cr.clone(), outRepl.hashTable.clone()), '__try0)) {
                    Some(__pa1) => __pa1.clone(),
                    _ => break '__try0 Err::<_, _>("pattern mismatch"),
                } };
                crDst = metamodelica::Own::own(__pa1);
                (crDst, _) = replaceExp(&crDst, singleRepl, inFuncTypeExpExpToBooleanOption.clone());
                outRepl = unwrap_break_err!(addReplacementNoTransitive(&outRepl, cr.clone(), crDst.clone()), '__try0);
                Ok::<(), &'static str>(())
            }.is_err() {
            }
        }
    }
    Ok(outRepl)
}

fn makeTransitive2(
    mut repl: VariableReplacements,
    mut src: metamodelica::Ref<DAE::ComponentRef>,
    mut dst: metamodelica::Ref<DAE::Exp>,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> (
    VariableReplacements,
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::Exp>,
) {
    let mut outRepl: VariableReplacements;
    let mut outSrc: metamodelica::Ref<DAE::ComponentRef>;
    let mut outDst: metamodelica::Ref<DAE::Exp>;
    (outRepl, outSrc, outDst) = (match inFuncTypeExpExpToBooleanOption.clone() {
        _ => {
            let mut dst_1: metamodelica::Ref<DAE::Exp>;
            (dst_1, _) = replaceExp(&dst, &repl, inFuncTypeExpExpToBooleanOption);
            (repl, src, dst_1)
        }
        _ => (repl, src, dst),
    });
    (outRepl, outSrc, outDst)
}

fn addExtendReplacement(
    mut extendrepl: CrefSet,
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut preCr: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<CrefSet> {
    let mut outExtendrepl: CrefSet = extendrepl.clone();
    let mut worklist: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        Option<metamodelica::Ref<DAE::ComponentRef>>,
    )> = list![(cr.clone(), preCr.clone())];
    let mut wcr: metamodelica::Ref<DAE::ComponentRef>;
    let mut wpre: Option<metamodelica::Ref<DAE::ComponentRef>>;
    while !((worklist).is_empty()) {
        (wcr, wpre) = (worklist).head().cloned()?;
        worklist = (worklist).rest()?;
        let _ = (::match_deref::match_deref! { match &((wcr.clone(), wpre)) {
            (Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType: ty @ Deref @ DAE::Type::T_ARRAY { .. }, .. }, None) => {
                let mut precr: metamodelica::Ref<DAE::ComponentRef>;
                precr = ComponentReferenceBasics::makeCrefIdent(ident.clone(), ty.clone(), metamodelica::nil());
                UnorderedSet::add(precr, extendrepl.clone())?;
                ()
            },
            (Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType: ty @ Deref @ DAE::Type::T_ARRAY { .. }, .. }, Some(pcr)) => {
                let mut precr: metamodelica::Ref<DAE::ComponentRef>;
                let mut precr1: metamodelica::Ref<DAE::ComponentRef>;
                precr = ComponentReferenceBasics::makeCrefIdent(ident.clone(), ty.clone(), metamodelica::nil());
                precr1 = ComponentReference::joinCrefs(metamodelica::AsArg::as_arg(&pcr), precr)?;
                UnorderedSet::add(precr1, extendrepl.clone())?;
                ()
            },
            (Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType: ty @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, varLst, .. }, .. }, None) => {
                let mut precr: metamodelica::Ref<DAE::ComponentRef>;
                let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                precr = ComponentReferenceBasics::makeCrefIdent(ident.clone(), ty.clone(), metamodelica::nil());
                if !(UnorderedSet::contains(precr.clone(), extendrepl.clone())?) {
                    UnorderedSet::add(precr.clone(), extendrepl.clone())?;
                    crefs = List::map(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(ComponentReference::creffromVar(&__a0)) })?;
                    for mut c in &*crefs {
                        worklist = metamodelica::cons((c.clone(), Some(precr.clone())), worklist);
                    }
                }
                ()
            },
            (Deref @ DAE::ComponentRef::CREF_IDENT { identType: ty @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, varLst, .. }, .. }, Some(pcr)) => {
                let mut precr1: metamodelica::Ref<DAE::ComponentRef>;
                let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                precr1 = ComponentReference::joinCrefs(metamodelica::AsArg::as_arg(&pcr), wcr)?;
                if !(UnorderedSet::contains(precr1.clone(), extendrepl.clone())?) {
                    UnorderedSet::add(precr1.clone(), extendrepl.clone())?;
                    crefs = List::map(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(ComponentReference::creffromVar(&__a0)) })?;
                    for mut c in &*crefs {
                        worklist = metamodelica::cons((c.clone(), Some(precr1.clone())), worklist);
                    }
                }
                ()
            },
            (Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType: ty, subscriptLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, None) => {
                let mut precr: metamodelica::Ref<DAE::ComponentRef>;
                precr = ComponentReferenceBasics::makeCrefIdent(ident.clone(), ty.clone(), metamodelica::nil());
                UnorderedSet::add(precr, extendrepl.clone())?;
                ()
            },
            (Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType: ty, subscriptLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, Some(pcr)) => {
                let mut precr: metamodelica::Ref<DAE::ComponentRef>;
                let mut precr1: metamodelica::Ref<DAE::ComponentRef>;
                precr = ComponentReferenceBasics::makeCrefIdent(ident.clone(), ty.clone(), metamodelica::nil());
                precr1 = ComponentReference::joinCrefs(metamodelica::AsArg::as_arg(&pcr), precr)?;
                UnorderedSet::add(precr1, extendrepl.clone())?;
                ()
            },
            (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, _) => {
                ()
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType: ty, subscriptLst, componentRef: subcr }, None) => {
                let mut precr: metamodelica::Ref<DAE::ComponentRef>;
                let mut precrn: metamodelica::Ref<DAE::ComponentRef>;
                precr = ComponentReferenceBasics::makeCrefIdent(ident.clone(), ty.clone(), metamodelica::nil());
                UnorderedSet::add(precr, extendrepl.clone())?;
                precrn = ComponentReferenceBasics::makeCrefIdent(ident.clone(), ty.clone(), subscriptLst.clone());
                worklist = metamodelica::cons((subcr.clone(), Some(precrn)), worklist);
                ()
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType: ty, subscriptLst, componentRef: subcr }, Some(pcr)) => {
                let mut precr: metamodelica::Ref<DAE::ComponentRef>;
                let mut precr1: metamodelica::Ref<DAE::ComponentRef>;
                let mut precrn: metamodelica::Ref<DAE::ComponentRef>;
                let mut precrn1: metamodelica::Ref<DAE::ComponentRef>;
                precr = ComponentReferenceBasics::makeCrefIdent(ident.clone(), ty.clone(), metamodelica::nil());
                precr1 = ComponentReference::joinCrefs(metamodelica::AsArg::as_arg(&pcr), precr)?;
                UnorderedSet::add(precr1, extendrepl.clone())?;
                precrn = ComponentReferenceBasics::makeCrefIdent(ident.clone(), ty.clone(), subscriptLst.clone());
                precrn1 = ComponentReference::joinCrefs(metamodelica::AsArg::as_arg(&pcr), precrn)?;
                worklist = metamodelica::cons((subcr.clone(), Some(precrn1)), worklist);
                ()
            },
            _ => return Err("match: no arm matched"),
        } });
    }
    Ok(outExtendrepl)
}

fn addIterationVar(mut repl: VariableReplacements, mut inVar: ArcStr) -> VariableReplacements {
    let mut repl: VariableReplacements = repl;
    repl.iterationVars = metamodelica::cons(inVar, repl.iterationVars.clone());
    repl
}

fn removeIterationVar(mut repl: VariableReplacements, mut inVar: ArcStr) -> Result<VariableReplacements> {
    let mut repl: VariableReplacements = repl;
    repl.iterationVars =
        List::deleteMemberOnTrue(inVar, repl.iterationVars.clone(), &fnptr!(stringEq, ArcStr, ArcStr))?.0;
    Ok(repl)
}

fn isIterationVar(mut repl: &VariableReplacements, mut inVar: ArcStr) -> bool {
    let mut is: bool;
    is = (match repl.clone() {
        VariableReplacements {
            iterationVars: ref iv, ..
        } => listMember(inVar, iv.clone()),
    });
    is
}

pub(crate) fn addDerConstRepl(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut repl: VariableReplacements,
) -> Result<VariableReplacements> {
    let mut repl: VariableReplacements = repl;
    let mut derConst: CrefExpTable;
    if (repl.derConst).is_some() {
        UnorderedMap::add(inComponentRef, Some(inExp), Util::getOption(repl.derConst.clone())?)?;
    } else {
        derConst = newCrefExpTable();
        UnorderedMap::add(inComponentRef, Some(inExp), derConst.clone())?;
        repl.derConst = Some(derConst);
    }
    Ok(repl)
}

pub(crate) fn getReplacement(
    mut inVariableReplacements: &VariableReplacements,
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(UnorderedMap::getOrFail(inComponentRef, inVariableReplacements.hashTable.clone())?) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outExp = metamodelica::Own::own(__pa0);
    Ok(outExp)
}

pub(crate) fn hasReplacement(
    mut repl: &VariableReplacements,
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut bOut: bool;
    bOut = (UnorderedMap::getOrDefault(inComponentRef, repl.hashTable.clone(), None)?).is_some();
    Ok(bOut)
}

pub(crate) fn hasNoReplacement(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut repl: &VariableReplacements,
) -> Result<bool> {
    let mut bOut: bool;
    bOut = (UnorderedMap::getOrDefault(inComponentRef, repl.hashTable.clone(), None)?).is_none();
    Ok(bOut)
}

pub(crate) fn getAllReplacements(
    mut repl: &VariableReplacements,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
)> {
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut dsts: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    (crefs, dsts) = getCrefExpTableEntries(repl.hashTable.clone())?;
    Ok((crefs, dsts))
}

fn getCrefExpTableEntries(
    mut table: CrefExpTable,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
)> {
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut dsts: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut opt_dsts: metamodelica::List<Option<metamodelica::Ref<DAE::Exp>>>;
    crefs = UnorderedMap::keyList(table.clone());
    opt_dsts = UnorderedMap::valueList(table);
    (opt_dsts, crefs) = List::filterOnTrueSync(&opt_dsts, &fnptr!(isSome, _), crefs)?;
    dsts = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut d in (opt_dsts).into_iter().cloned() {
            let __x = Util::getOption(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((crefs, dsts))
}

pub(crate) fn hasExtendReplacement(
    mut repl: &VariableReplacements,
    mut src: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut exists: bool;
    exists = UnorderedSet::contains(
        ComponentReferenceBasics::crefStripLastSubs(src)?,
        repl.extendhashTable.clone(),
    )?;
    Ok(exists)
}

fn avoidDoubleHashLookup(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_UNKNOWN { .. } } => {
                    Ok(Expression::makeCrefExp(cr.clone(), inType.clone())?)
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
        panic!("matchcontinue: no arm matched")
    };
    outExp
}

pub(crate) fn isReplacementEmpty(mut repl: &VariableReplacements) -> Result<bool> {
    let mut empty: bool = UnorderedMap::none(repl.hashTable.clone(), std::sync::Arc::new(fnptr!(isSome, _)))?
        && (repl.derConst).is_none();
    Ok(empty)
}

/* ********************************************************/
/* replace Expression with condition function */
/* ********************************************************/
pub fn replaceExp(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inVariableReplacements: &VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut replacementPerformed: bool;
    (outExp, replacementPerformed) = 'mc: {
        let __mc_input = (&**inExp, inFuncTypeExpExpToBooleanOption.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident, .. }, .. }, _) => {
                    if !((isIterationVar(inVariableReplacements, ident.clone()))) { return Err("guard") }
                    Ok((inExp.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ DAE::Exp::CREF { componentRef: cr, ty: t }, cond) => {
                            if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                            let mut e: metamodelica::Ref<DAE::Exp>;
                            let mut e1: metamodelica::Ref<DAE::Exp>;
                            let mut e2: metamodelica::Ref<DAE::Exp>;
                            let mut c: bool;
                            let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                            let mut cr = (*cr).clone();
                            (cr, c) = replaceCrefSubs(metamodelica::AsArg::as_arg(&cr), inVariableReplacements, cond.clone());
                            match '__try0: {
                                e1 = unwrap_break_err!(getReplacement(inVariableReplacements, cr.clone()), '__try0);
                                e = avoidDoubleHashLookup(e1.clone(), t.clone());
                                Ok::<_, &'static str>((e.clone(),))
                            } {
                                Ok((__try0_o0,)) => {
                                    e = __try0_o0;
                                }
                                Err(_) => {
                                    match '__try1: {
                                                (_, dims) = TypesDump::flattenArrayType(metamodelica::AsArg::as_arg(&t));
                                                let true = (unwrap_break_err!(List::none(&(({
                let mut __acc: metamodelica::List<bool> = metamodelica::nil();
                for mut dim in (dims.clone()).into_iter().cloned() {
                            let __x = Types::dimNotFixed(&(dim.clone()));
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })), &fnptr!(Util::id, _)), '__try1)) else { break '__try1 Err::<_, _>("pattern mismatch") };
                                                let true = (unwrap_break_err!(hasExtendReplacement(inVariableReplacements, metamodelica::AsArg::as_arg(&cr)), '__try1)) else { break '__try1 Err::<_, _>("pattern mismatch") };
                                                let __pa2 = ::match_deref::match_deref! { match &(Expression::extendArrExp(inExp.clone(), false)) {
                                                    (__pa2, true) => __pa2.clone(),
                                                    _ => break '__try1 Err::<_, _>("pattern mismatch"),
                                                } };
                                                e2 = metamodelica::Own::own(__pa2);
                                                (e, _) = replaceExp(&e2, inVariableReplacements, cond.clone());
                                                Ok::<_, &'static str>((e.clone(),))
                                    } {
                                                Ok((__try1_o0,)) => {
                                                    e = __try1_o0;
                                                }
                                                Err(_) => {
                                                    let true = (c) else { return Err("pattern mismatch") };
                                                    e = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: t.clone() });
                                                }
                                    }
                                }
                            }
                            Ok((e.clone(), true))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut c1: bool;
                    let mut c2: bool;
                    (e1_1, c1) = replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone());
                    (e2_1, c2) = replaceExp(metamodelica::AsArg::as_arg(&e2), inVariableReplacements, cond.clone());
                    let true = (c1 || c2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut c1: bool;
                    let mut c2: bool;
                    (e1_1, c1) = replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone());
                    (e2_1, c2) = replaceExp(metamodelica::AsArg::as_arg(&e2), inVariableReplacements, cond.clone());
                    let true = (c1 || c2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: op, exp: e1 }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: e1_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, index: index_, optionExpisASUB: isExpisASUB }, cond) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut c1: bool;
                    let mut c2: bool;
                    (e1_1, c1) = replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone());
                    (e2_1, c2) = replaceExp(metamodelica::AsArg::as_arg(&e2), inVariableReplacements, cond.clone());
                    let true = (c1 || c2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone(), index: index_.clone(), optionExpisASUB: isExpisASUB.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e3_1: metamodelica::Ref<DAE::Exp>;
                    let mut c1: bool;
                    let mut c2: bool;
                    let mut c3: bool;
                    (e1_1, c1) = replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone());
                    (e2_1, c2) = replaceExp(metamodelica::AsArg::as_arg(&e2), inVariableReplacements, cond.clone());
                    (e3_1, c3) = replaceExp(metamodelica::AsArg::as_arg(&e3), inVariableReplacements, cond.clone());
                    let true = (c1 || c2 || c3) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e1_1.clone(), expThen: e2_1.clone(), expElse: e3_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, cond) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut derConst: CrefExpTable;
                    let __pa0 = ::match_deref::match_deref! { match &(inVariableReplacements.derConst.clone()) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    derConst = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(UnorderedMap::getOrFail(cr.clone(), derConst.clone())?) {
                        Some(__pa1) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa1);
                    (e, _) = replaceExp(&e, inVariableReplacements, cond.clone());
                    Ok((e.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path, expLst: expl, attr }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut path = (*path).clone();
                    let mut expl = (*expl).clone();
                    cr = ComponentReference::toExpCref(&(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&path))))?;
                    if hasReplacement(inVariableReplacements, cr.clone())? {
                        e1_1 = getReplacement(inVariableReplacements, cr.clone())?;
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(e1_1.clone()) {
                            Deref @ DAE::Exp::PARTEVALFUNCTION { path: __pa0, expList: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        path = metamodelica::Own::own(__pa0);
                        expl_1 = metamodelica::Own::own(__pa1);
                        expl = listAppend(expl_1.clone(), expl.clone());
                    }
                    let __pa2 = ::match_deref::match_deref! { match &(replaceExpList(expl.clone(), inVariableReplacements, cond.clone())) {
                        (__pa2, true) => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    expl_1 = metamodelica::Own::own(__pa2);
                    Ok((metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: expl_1.clone(), attr: attr.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RECORD { path, exps: expl, comp: fields, ty: t }, cond) => {
                    let mut repl: VariableReplacements;
                    let mut expl = (*expl).clone();
                    repl = addConstantRecordReplacements(metamodelica::AsArg::as_arg(&t), metamodelica::AsArg::as_arg(&expl), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExpList(expl.clone(), &repl, cond.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    expl = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Exp::RECORD { path: path.clone(), exps: expl.clone(), comp: fields.clone(), ty: t.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: e, resolution } }, cond) => {
                    let mut c1: bool;
                    let mut c2: bool;
                    let mut c3: bool;
                    let mut e = (*e).clone();
                    let mut resolution = (*resolution).clone();
                    (e, c1) = replaceExp(metamodelica::AsArg::as_arg(&e), inVariableReplacements, cond.clone());
                    (resolution, c2) = replaceExp(metamodelica::AsArg::as_arg(&resolution), inVariableReplacements, cond.clone());
                    c3 = c1 || c2;
                    Ok((if (c3) {metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: e.clone(), resolution: resolution.clone() }) })} else {inExp.clone()}, c3))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::REAL_CLOCK { interval: e } }, cond) => {
                    let mut c1: bool;
                    let mut e = (*e).clone();
                    (e, c1) = replaceExp(metamodelica::AsArg::as_arg(&e), inVariableReplacements, cond.clone());
                    Ok((if (c1) {metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::REAL_CLOCK { interval: e.clone() }) })} else {inExp.clone()}, c1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::EVENT_CLOCK { condition: e, startInterval } }, cond) => {
                    let mut c1: bool;
                    let mut c2: bool;
                    let mut c3: bool;
                    let mut e = (*e).clone();
                    let mut startInterval = (*startInterval).clone();
                    (e, c1) = replaceExp(metamodelica::AsArg::as_arg(&e), inVariableReplacements, cond.clone());
                    (startInterval, c2) = replaceExp(metamodelica::AsArg::as_arg(&startInterval), inVariableReplacements, cond.clone());
                    c3 = c1 || c2;
                    Ok((if (c3) {metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::EVENT_CLOCK { condition: e.clone(), startInterval: startInterval.clone() }) })} else {inExp.clone()}, c3))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::SOLVER_CLOCK { c: e, solverMethod } }, cond) => {
                    let mut c1: bool;
                    let mut c2: bool;
                    let mut c3: bool;
                    let mut e = (*e).clone();
                    let mut solverMethod = (*solverMethod).clone();
                    (e, c1) = replaceExp(metamodelica::AsArg::as_arg(&e), inVariableReplacements, cond.clone());
                    (solverMethod, c2) = replaceExp(metamodelica::AsArg::as_arg(&solverMethod), inVariableReplacements, cond.clone());
                    c3 = c1 || c2;
                    Ok((if (c3) {metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::SOLVER_CLOCK { c: e.clone(), solverMethod: solverMethod.clone() }) })} else {inExp.clone()}, c3))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PARTEVALFUNCTION { path, expList: expl, ty: tp, origType: t }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExpList(expl.clone(), inVariableReplacements, cond.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    expl_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Exp::PARTEVALFUNCTION { path: path.clone(), expList: expl_1.clone(), ty: tp.clone(), origType: t.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { ty: tp, scalar: c, array: expl }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExpList(expl.clone(), inVariableReplacements, cond.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    expl_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: c.clone(), array: expl_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { ty: t, integer: b, matrix: bexpl }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut bexpl_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExpMatrix(bexpl.clone(), inVariableReplacements, cond.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    bexpl_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Exp::MATRIX { ty: t.clone(), integer: b.clone(), matrix: bexpl_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { ty: tp, start: e1, step: None, stop: e2 }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut c1: bool;
                    let mut c2: bool;
                    (e1_1, c1) = replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone());
                    (e2_1, c2) = replaceExp(metamodelica::AsArg::as_arg(&e2), inVariableReplacements, cond.clone());
                    let true = (c1 || c2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Exp::RANGE { ty: tp.clone(), start: e1_1.clone(), step: None, stop: e2_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { ty: tp, start: e1, step: Some(e3), stop: e2 }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e3_1: metamodelica::Ref<DAE::Exp>;
                    let mut c1: bool;
                    let mut c2: bool;
                    let mut c3: bool;
                    (e1_1, c1) = replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone());
                    (e2_1, c2) = replaceExp(metamodelica::AsArg::as_arg(&e2), inVariableReplacements, cond.clone());
                    (e3_1, c3) = replaceExp(metamodelica::AsArg::as_arg(&e3), inVariableReplacements, cond.clone());
                    let true = (c1 || c2 || c3) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Exp::RANGE { ty: tp.clone(), start: e1_1.clone(), step: Some(e3_1.clone()), stop: e2_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: expl }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExpList(expl.clone(), inVariableReplacements, cond.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    expl_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expl_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { ty: tp, exp: e1 }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Exp::CAST { ty: tp.clone(), exp: e1_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { exp: e1, sub: subs }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut c1: bool;
                    let mut c2: bool;
                    expl = List::map(subs.clone(), &move |__a0: metamodelica::Ref<DAE::Subscript>| Expression::getSubscriptExp(&__a0))?;
                    (e1_1, c1) = replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone());
                    (expl, c2) = replaceExpList(expl.clone(), inVariableReplacements, cond.clone());
                    let true = (c1 || c2) else { return Err("pattern mismatch") };
                    Ok((Expression::makeASUB(e1_1.clone(), expl.clone())?, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TSUB { exp: e1, ix: i, ty: tp }, cond) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let true = (replaceExpCond(cond.clone(), e1.clone())?) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Exp::TSUB { exp: e1_1.clone(), ix: i.clone(), ty: tp.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RSUB { exp: e1, ix: i, fieldName: ident, ty: tp }, cond) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let true = (replaceExpCond(cond.clone(), e1.clone())?) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Exp::RSUB { exp: e1_1.clone(), ix: i.clone(), fieldName: ident.clone(), ty: tp.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SIZE { exp: e1, sz: Some(e2) }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut c1: bool;
                    let mut c2: bool;
                    (e1_1, c1) = replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone());
                    (e2_1, c2) = replaceExp(metamodelica::AsArg::as_arg(&e2), inVariableReplacements, cond.clone());
                    let true = (c1 || c2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Exp::SIZE { exp: e1_1.clone(), sz: Some(e2_1.clone()) }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CODE { code: a, ty: tp }, _) => {
                    metamodelica::print(literal!("replace_exp on CODE not impl.\n"));
                    Ok((metamodelica::Ref::new(DAE::Exp::CODE { code: a.clone(), ty: tp.clone() }), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { reductionInfo, expr: e1, iterators: iters }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut iters = (*iters).clone();
                    (e1_1, _) = replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExpIters(iters.clone(), inVariableReplacements, cond.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    iters = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: reductionInfo.clone(), expr: e1_1.clone(), iterators: iters.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BOX { exp: e1 }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Exp::BOX { exp: e1_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNBOX { ty: tp, exp: e1 }, cond) => {
                    if !((replaceExpCond(cond.clone(), inExp.clone())?)) { return Err("guard") }
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(metamodelica::AsArg::as_arg(&e1), inVariableReplacements, cond.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Exp::UNBOX { exp: e1_1.clone(), ty: tp.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, replacementPerformed)
}

pub(crate) fn addConstantRecordReplacements(
    mut ty: &metamodelica::Ref<DAE::Type>,
    mut expl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut repl: VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> Result<VariableReplacements> {
    let mut repl: VariableReplacements = repl;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    repl = (match &**ty {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::RECORD { path },
            varLst: __ty_varLst,
            ..
        } => {
            let mut bind: metamodelica::Ref<DAE::Exp>;
            for mut var in &*__ty_varLst.clone() {
                if DAEUtil::isBound(&var.binding) {
                    let __pa0 = ::match_deref::match_deref! { match &(DAEUtil::bindingExp(&var.binding)?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    bind = metamodelica::Own::own(__pa0);
                    cref = getRecordElement(&var.name, expl, metamodelica::AsArg::as_arg(&path))?;
                    if Expression::isConst(bind.clone())? && !(ComponentReference::isWild(&cref)) {
                        repl = addReplacement(repl, cref, bind, inFuncTypeExpExpToBooleanOption.clone())?;
                    }
                }
            }
            repl
        }
        _ => repl,
    });
    Ok(repl)
}

pub(crate) fn getRecordElement(
    mut name: &ArcStr,
    mut expl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut recordPath: &metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut cref: metamodelica::Ref<DAE::ComponentRef> =
        openmodelica_frontend_types::DAE::ComponentRef::interned_WILD();
    for mut e in &**expl {
        let () = (match &*e.clone() {
            DAE::Exp::CREF {
                componentRef: __e_componentRef,
                ..
            } if (metamodelica::stringEq(
                &(ComponentReferenceBasics::crefLastIdent(metamodelica::AsArg::as_arg(&__e_componentRef))?),
                &name,
            ) && isElementOfRecord(metamodelica::AsArg::as_arg(&__e_componentRef), recordPath)) =>
            {
                cref = __e_componentRef.clone();
                return Ok(cref);
                ()
            }
            _ => (),
        });
    }
    Ok(cref)
}

pub(crate) fn isElementOfRecord<'__b>(
    mut cref: &'__b metamodelica::Ref<DAE::ComponentRef>,
    mut recordPath: &'__b metamodelica::Ref<Absyn::Path>,
) -> bool {
    let mut res: bool;
    let mut path: metamodelica::Ref<Absyn::Path>;
    res = (::match_deref::match_deref! { match cref {
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { .. }, .. } => {
            res = (match &*(Types::arrayElementType(var_field!((**cref).identType, DAE::ComponentRef::CREF_QUAL))) {
        DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __esc_path }, .. } => {
            path = (*__esc_path).clone();
            AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&path), recordPath)
        },
        _ => false,
    });
            res
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => isElementOfRecord(var_field!((**cref).componentRef, DAE::ComponentRef::CREF_QUAL), recordPath),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub(crate) fn replaceCref(
    mut crefIn: metamodelica::Ref<DAE::ComponentRef>,
    mut replIn: &VariableReplacements,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut expOut: metamodelica::Ref<DAE::Exp>;
    let mut changedOut: bool;
    (expOut, changedOut) = (match replIn.clone() {
        _ if (hasReplacement(replIn, crefIn.clone())?) => {
            expOut = getReplacement(replIn, crefIn.clone())?;
            (expOut, true)
        }
        _ => {
            expOut = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: crefIn.clone(),
                ty: ComponentReference::crefType(&crefIn)?,
            });
            (expOut, false)
        }
    });
    Ok((expOut, changedOut))
}

fn replaceCrefSubs(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut repl: &VariableReplacements,
    mut cond: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> (metamodelica::Ref<DAE::ComponentRef>, bool) {
    let mut outCr: metamodelica::Ref<DAE::ComponentRef>;
    let mut replacementPerformed: bool;
    (outCr, replacementPerformed) = (match &**inCref {
        DAE::ComponentRef::CREF_QUAL {
            ident: name,
            identType: ty,
            subscriptLst: subs,
            componentRef: cr,
        } => {
            let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
            let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut c1: bool;
            let mut c2: bool;
            let mut subs = (*subs).clone();
            let mut cr = (*cr).clone();
            (subs_1, c1) = replaceCrefSubs2(subs.clone(), repl, cond.clone());
            (cr_1, c2) = replaceCrefSubs(metamodelica::AsArg::as_arg(&cr), repl, cond);
            subs = if (c1) { subs_1 } else { subs.clone() };
            cr = if (c2) { cr_1 } else { cr.clone() };
            cr = if (c1 || c2) {
                metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                    ident: name.clone(),
                    identType: ty.clone(),
                    subscriptLst: subs.clone(),
                    componentRef: cr.clone(),
                })
            } else {
                inCref.clone()
            };
            (cr.clone(), c1 || c2)
        }
        DAE::ComponentRef::CREF_IDENT {
            ident: name,
            identType: ty,
            subscriptLst: subs,
        } => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut c1: bool;
            let mut subs = (*subs).clone();
            (subs, c1) = replaceCrefSubs2(subs.clone(), repl, cond);
            cr = if (c1) {
                metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: name.clone(),
                    identType: ty.clone(),
                    subscriptLst: subs.clone(),
                })
            } else {
                inCref.clone()
            };
            (cr, c1)
        }
        _ => (inCref.clone(), false),
    });
    (outCr, replacementPerformed)
}

fn replaceCrefSubs2(
    mut isubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut repl: &VariableReplacements,
    mut cond: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> (metamodelica::List<metamodelica::Ref<DAE::Subscript>>, bool) {
    let mut outSubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut replacementPerformed: bool = false;
    outSubs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
        for mut sub in (isubs).into_iter().cloned() {
            let __x = (match &*sub.clone() {
                DAE::Subscript::WHOLEDIM { .. } => sub.clone(),
                DAE::Subscript::SLICE { exp } => {
                    let mut c1: bool;
                    let mut exp = (*exp).clone();
                    (exp, c1) = replaceExp(metamodelica::AsArg::as_arg(&exp), repl, cond.clone());
                    replacementPerformed = replacementPerformed || c1;
                    if (c1) {
                        metamodelica::Ref::new(DAE::Subscript::SLICE { exp: exp.clone() })
                    } else {
                        sub.clone()
                    }
                }
                DAE::Subscript::INDEX { exp } => {
                    let mut c1: bool;
                    let mut exp = (*exp).clone();
                    (exp, c1) = replaceExp(metamodelica::AsArg::as_arg(&exp), repl, cond.clone());
                    replacementPerformed = replacementPerformed || c1;
                    if (c1) {
                        metamodelica::Ref::new(DAE::Subscript::INDEX { exp: exp.clone() })
                    } else {
                        sub.clone()
                    }
                }
                DAE::Subscript::WHOLE_NONEXP { exp } => {
                    let mut c1: bool;
                    let mut exp = (*exp).clone();
                    (exp, c1) = replaceExp(metamodelica::AsArg::as_arg(&exp), repl, cond.clone());
                    replacementPerformed = replacementPerformed || c1;
                    if (c1) {
                        metamodelica::Ref::new(DAE::Subscript::WHOLE_NONEXP { exp: exp.clone() })
                    } else {
                        sub.clone()
                    }
                }
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    (outSubs, replacementPerformed)
}

pub(crate) fn replaceExpList(
    mut iexpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut repl: &VariableReplacements,
    mut cond: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> (metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool) {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut replacementPerformed: bool = false;
    let mut exp_: metamodelica::Ref<DAE::Exp>;
    let mut c: bool;
    outExpl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut exp in (iexpl).into_iter().cloned() {
            let __x = (match &*exp.clone() {
                _ => {
                    (exp_, c) = replaceExp(&(exp.clone()), repl, cond.clone());
                    if c {
                        replacementPerformed = true;
                    } else {
                        exp_ = exp.clone();
                    }
                    exp_.clone()
                }
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    (outExpl, replacementPerformed)
}

pub(crate) fn replaceExpList1(
    mut iexpl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut repl: &VariableReplacements,
    mut cond: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> (
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<bool>,
) {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut replacementPerformed: metamodelica::List<bool>;
    let mut acc1: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut acc2: metamodelica::List<bool> = metamodelica::nil();
    let mut c: bool;
    for mut exp in &**iexpl {
        let mut exp = exp.clone();
        (exp, c) = replaceExp(&exp, repl, cond.clone());
        acc2 = metamodelica::cons(c, acc2);
        acc1 = metamodelica::cons(exp, acc1);
    }
    outExpl = metamodelica::Dangerous::listReverseInPlace(acc1);
    replacementPerformed = metamodelica::Dangerous::listReverseInPlace(acc2);
    (outExpl, replacementPerformed)
}

fn replaceExpIters(
    mut inIters: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>,
    mut repl: &VariableReplacements,
    mut cond: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> (metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>, bool) {
    let mut outIter: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>;
    let mut replacementPerformed: bool = false;
    let mut it: metamodelica::Ref<DAE::ReductionIterator>;
    outIter = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>> = metamodelica::nil();
        for mut iter in (inIters).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(iter.clone()) {
                Deref @ DAE::ReductionIterator { id, exp, guardExp: None, ty } => {
                    let mut b1: bool;
                    let mut exp = (*exp).clone();
                    (exp, b1) = replaceExp(metamodelica::AsArg::as_arg(&exp), repl, cond.clone());
                    if b1 {
                        it = metamodelica::Ref::new(DAE::ReductionIterator { id: id.clone(), exp: exp.clone(), guardExp: None, ty: ty.clone() });
                        replacementPerformed = true;
                    } else {
                        it = iter.clone();
                    }
                    it.clone()
                },
                Deref @ DAE::ReductionIterator { id, exp, guardExp: Some(gexp), ty } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut exp = (*exp).clone();
                    let mut gexp = (*gexp).clone();
                    (exp, b1) = replaceExp(metamodelica::AsArg::as_arg(&exp), repl, cond.clone());
                    (gexp, b2) = replaceExp(metamodelica::AsArg::as_arg(&gexp), repl, cond.clone());
                    if b1 || b2 {
                        it = metamodelica::Ref::new(DAE::ReductionIterator { id: id.clone(), exp: exp.clone(), guardExp: Some(gexp.clone()), ty: ty.clone() });
                        replacementPerformed = true;
                    } else {
                        it = iter.clone();
                    }
                    it.clone()
                },
                _ => {
                    iter.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    (outIter, replacementPerformed)
}

fn replaceExpCond(
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = (match inFuncTypeExpExpToBooleanOption {
        Some(mut cond) => {
            let mut e = inExp;
            let mut res: bool;
            res = cond(e)?;
            res
        }
        _ => true,
    });
    Ok(outBoolean)
}

fn replaceExpMatrix(
    mut inTplExpExpBooleanLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inVariableReplacements: &VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> (
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    bool,
) {
    let mut outTplExpExpBooleanLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut replacementPerformed: bool = false;
    let mut exp_: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut c: bool;
    outTplExpExpBooleanLstLst = ({
        let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = metamodelica::nil();
        for mut exp in (inTplExpExpBooleanLstLst).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(exp.clone()) {
                _ => {
                    (exp_, c) = replaceExpList(exp.clone(), inVariableReplacements, inFuncTypeExpExpToBooleanOption.clone());
                    if c {
                        replacementPerformed = true;
                    } else {
                        exp_ = exp.clone();
                    }
                    exp_.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    (outTplExpExpBooleanLstLst, replacementPerformed)
}

/* ********************************************************/
/* condition function for replace Expression  */
/* ********************************************************/
pub(crate) fn skipPreOperator(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, .. } if (metamodelica::stringEq(&idn, &(literal!("pre"))) || metamodelica::stringEq(&idn, &(literal!("previous")))) => {
            false
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn skipPreChangeEdgeOperator(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } if (metamodelica::stringEq(&idn, &(literal!("pre"))) || metamodelica::stringEq(&idn, &(literal!("previous"))) || metamodelica::stringEq(&idn, &(literal!("change"))) || metamodelica::stringEq(&idn, &(literal!("edge")))) => {
            selfGeneratedVar(metamodelica::AsArg::as_arg(&cr))
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, .. } if (metamodelica::stringEq(&idn, &(literal!("pre"))) || metamodelica::stringEq(&idn, &(literal!("previous"))) || metamodelica::stringEq(&idn, &(literal!("change"))) || metamodelica::stringEq(&idn, &(literal!("edge")))) => {
            false
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn selfGeneratedVar(mut inCref: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut b: bool;
    b = (match &**inCref {
        DAE::ComponentRef::CREF_QUAL { ident: idn, .. }
            if (metamodelica::stringEq(&idn, &(literal!("$ZERO")))
                || metamodelica::stringEq(&idn, &(literal!("$_DER")))
                || metamodelica::stringEq(&idn, &(literal!("$pDER")))) =>
        {
            true
        }
        _ => false,
    });
    b
}

/* ********************************************************/
/* replace Equations  */
/* ********************************************************/
pub fn replaceEquationsArr(
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut repl: VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> (
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    bool,
) {
    let mut outEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> = <metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> as ::std::default::Default>::default();
    let mut replacementPerformed: bool = false;
    (outEqns, replacementPerformed) = 'mc: {
        let __mc_input = inFuncTypeExpExpToBooleanOption.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut outEqns: metamodelica::Ref<
                ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
            > = outEqns.clone();
            let mut replacementPerformed: bool = replacementPerformed.clone();
            let false = (isReplacementEmpty(&repl)?) else {
                return Err("pattern mismatch");
            };
            (_, _, eqns, replacementPerformed) = BackendEquation::traverseEquationArray(
                inEqns.clone(),
                &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
                       __a1: (
                    VariableReplacements,
                    Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
                    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                    bool,
                )|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(replaceEquationTraverser(__a0, &__a1))
                },
                (
                    repl.clone(),
                    inFuncTypeExpExpToBooleanOption.clone(),
                    metamodelica::nil(),
                    false,
                ),
            )?;
            outEqns = if (replacementPerformed) {
                BackendEquation::listEquation(&eqns)?
            } else {
                inEqns.clone()
            };
            Ok((
                (outEqns.clone(), replacementPerformed),
                outEqns.clone(),
                replacementPerformed.clone(),
            ))
        })() {
            outEqns = __wb0;
            replacementPerformed = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((inEqns.clone(), false))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outEqns, replacementPerformed)
}

fn replaceEquationTraverser(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTpl: &(
        VariableReplacements,
        Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        bool,
    ),
) -> (
    metamodelica::Ref<BackendDAE::Equation>,
    (
        VariableReplacements,
        Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        bool,
    ),
) {
    let mut e: metamodelica::Ref<BackendDAE::Equation>;
    let mut outTpl: (
        VariableReplacements,
        Option<FuncTypeExp_ExpToBoolean>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        bool,
    );
    let mut repl: VariableReplacements;
    let mut optfunc: Option<FuncTypeExp_ExpToBoolean>;
    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut b: bool;
    e = inEq;
    (repl, optfunc, eqns, b) = inTpl.clone();
    (eqns, b) = replaceEquation(e.clone(), repl.clone(), optfunc.clone(), eqns, b);
    outTpl = (repl, optfunc, eqns, b);
    (e, outTpl)
}

pub fn replaceEquations(
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut repl: &VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> Result<(metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, bool)> {
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut replacementPerformed: bool;
    if isReplacementEmpty(repl)? {
        outEqns = inEqns;
        replacementPerformed = false;
    } else {
        (outEqns, replacementPerformed) = replaceEquations2(
            inEqns,
            repl,
            inFuncTypeExpExpToBooleanOption,
            metamodelica::nil(),
            false,
        );
        if replacementPerformed && false {
            (outEqns, _) = BackendDAETransform::traverseBackendDAEExpsEqnLstWithSymbolicOperation(
                &outEqns,
                (std::sync::Arc::new(BackendDAETransform::collapseArrayCrefExp)
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
                0,
                metamodelica::nil(),
            )?;
        }
    }
    Ok((outEqns, replacementPerformed))
}

fn replaceEquations2<'__b>(
    mut inBackendDAEEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inVariableReplacements: &'__b VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
    mut inAcc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iReplacementPerformed: bool,
) -> (metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, bool) {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inBackendDAEEquationLst) {
            Deref @ metamodelica::ListNode::Nil => {
                return (inAcc.reverse(), iReplacementPerformed)
            },
            Deref @ metamodelica::ListNode::Cons { head: a, tail: es } => {
                let mut acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut b: bool;
                let mut es = (*es).clone();
                (acc, b) = replaceEquation(a.clone(), inVariableReplacements.clone(), inFuncTypeExpExpToBooleanOption.clone(), inAcc, iReplacementPerformed);
                { (inBackendDAEEquationLst, inVariableReplacements, inFuncTypeExpExpToBooleanOption, inAcc, iReplacementPerformed) = (es.clone(), inVariableReplacements, inFuncTypeExpExpToBooleanOption, acc, b); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn replaceEquation(
    mut inBackendDAEEquation: metamodelica::Ref<BackendDAE::Equation>,
    mut inVariableReplacements: VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
    mut inAcc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iReplacementPerformed: bool,
) -> (metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, bool) {
    let mut outBackendDAEEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut replacementPerformed: bool;
    (outBackendDAEEquationLst, replacementPerformed) = 'mc: {
        let __mc_input = (inBackendDAEEquation, inVariableReplacements);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize, left: e1, right: e2, source, attr: eqAttr, recordSize }, repl) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e1_2: metamodelica::Ref<DAE::Exp>;
                    let mut e2_2: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (e1_1, b1) = replaceExp(metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&repl), inFuncTypeExpExpToBooleanOption.clone());
                    (e2_1, b2) = replaceExp(metamodelica::AsArg::as_arg(&e2), metamodelica::AsArg::as_arg(&repl), inFuncTypeExpExpToBooleanOption.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformationSubstitution(b1, source.clone(), e1.clone(), e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), e2.clone(), e2_1.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1_1.clone(), rhs: e2_1.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1_2 = metamodelica::Own::own(__pa0);
                    e2_2 = metamodelica::Own::own(__pa1);
                    source = metamodelica::Own::own(__pa2);
                    Ok((metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimSize.clone(), left: e1_2.clone(), right: e2_2.clone(), source: source.clone(), attr: eqAttr.clone(), recordSize: recordSize.clone() }), inAcc.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size, left: e1, right: e2, source, attr: eqAttr }, repl) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e1_2: metamodelica::Ref<DAE::Exp>;
                    let mut e2_2: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (e1_1, b1) = replaceExp(metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&repl), inFuncTypeExpExpToBooleanOption.clone());
                    (e2_1, b2) = replaceExp(metamodelica::AsArg::as_arg(&e2), metamodelica::AsArg::as_arg(&repl), inFuncTypeExpExpToBooleanOption.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformationSubstitution(b1, source.clone(), e1.clone(), e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), e2.clone(), e2_1.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1_1.clone(), rhs: e2_1.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1_2 = metamodelica::Own::own(__pa0);
                    e2_2 = metamodelica::Own::own(__pa1);
                    source = metamodelica::Own::own(__pa2);
                    Ok((metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: e1_2.clone(), right: e2_2.clone(), source: source.clone(), attr: eqAttr.clone() }), inAcc.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr: eqAttr }, repl) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e1_2: metamodelica::Ref<DAE::Exp>;
                    let mut e2_2: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (e1_1, b1) = replaceExp(metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&repl), inFuncTypeExpExpToBooleanOption.clone());
                    (e2_1, b2) = replaceExp(metamodelica::AsArg::as_arg(&e2), metamodelica::AsArg::as_arg(&repl), inFuncTypeExpExpToBooleanOption.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformationSubstitution(b1, source.clone(), e1.clone(), e1_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), e2.clone(), e2_1.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1_1.clone(), rhs: e2_1.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1_2 = metamodelica::Own::own(__pa0);
                    e2_2 = metamodelica::Own::own(__pa1);
                    source = metamodelica::Own::own(__pa2);
                    Ok((metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: e1_2.clone(), scalar: e2_2.clone(), source: source.clone(), attr: eqAttr.clone() }), inAcc.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst: stmts }, source, expand: crefExpand, attr: eqAttr }, repl) => {
                    let mut hasArrayCref: bool;
                    let mut alg: metamodelica::Ref<DAE::Algorithm>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut stmts1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut crefExpand = (*crefExpand).clone();
                    crefs = Expression::getLhsCrefsFromStatements(stmts.clone())?;
                    hasArrayCref = List::any(&crefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| -> metamodelica::Result<_> { ::std::result::Result::Ok(ComponentReference::isArrayElement(&__a0)) })?;
                    crefExpand = if (hasArrayCref) {crefExpand.clone()} else {openmodelica_frontend_types::DAE::Expand::NOT_EXPAND};
                    let __pa0 = ::match_deref::match_deref! { match &(replaceStatementLst(metamodelica::AsArg::as_arg(&stmts), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), &(metamodelica::nil()), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    stmts1 = metamodelica::Own::own(__pa0);
                    alg = metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts1.clone() });
                    eqns = if (!((stmts1).is_empty())) {metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), inAcc.clone())} else {inAcc.clone()};
                    Ok((eqns.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e, source, attr: eqAttr }, repl) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut source = (*source).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&repl), inFuncTypeExpExpToBooleanOption.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e_1 = metamodelica::Own::own(__pa0);
                    (e_2, _) = ExpressionSimplify::simplify(e_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(true, source.clone(), e.clone(), e_2.clone())?;
                    Ok((metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr.clone(), exp: e_2.clone(), source: source.clone(), attr: eqAttr.clone() }), inAcc.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, source, attr: eqAttr }, repl) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut source = (*source).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&repl), inFuncTypeExpExpToBooleanOption.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e_1 = metamodelica::Own::own(__pa0);
                    (e_2, _) = ExpressionSimplify::simplify(e_1.clone())?;
                    source = ElementSource::addSymbolicTransformationSubstitution(true, source.clone(), e.clone(), e_2.clone())?;
                    Ok((metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e_2.clone(), source: source.clone(), attr: eqAttr.clone() }), inAcc.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::WHEN_EQUATION { size, whenEquation: whenEqn, source, attr: eqAttr }, repl) => {
                    let mut whenEqn1: metamodelica::Ref<BackendDAE::WhenEquation>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(replaceWhenEquation(metamodelica::AsArg::as_arg(&whenEqn), metamodelica::AsArg::as_arg(&repl), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::AsArg::as_arg(&source))?) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    whenEqn1 = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEqn1.clone(), source: source.clone(), attr: eqAttr.clone() }), inAcc.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::IF_EQUATION { conditions: expl, eqnstrue: eqnslst, eqnsfalse: eqns, source, attr: eqAttr }, repl) => {
                    let mut expl1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut blst: metamodelica::List<bool>;
                    let mut eqnslst = (*eqnslst).clone();
                    let mut eqns = (*eqns).clone();
                    let mut source = (*source).clone();
                    (expl1, blst) = replaceExpList1(metamodelica::AsArg::as_arg(&expl), metamodelica::AsArg::as_arg(&repl), inFuncTypeExpExpToBooleanOption.clone());
                    b1 = List::any(&blst, &fnptr!(Util::id, _))?;
                    source = ElementSource::addSymbolicTransformationSubstitutionLst(&blst, source.clone(), metamodelica::AsArg::as_arg(&expl), &expl1)?;
                    (expl2, blst) = ExpressionSimplify::condsimplifyList1(&blst, expl1.clone())?;
                    source = ElementSource::addSymbolicTransformationSimplifyLst(&blst, source.clone(), &expl1, &expl2)?;
                    (eqnslst, b2) = List::map3Fold(metamodelica::AsArg::as_arg(&eqnslst), &move |__a0: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, __a1: VariableReplacements, __a2: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>, __a3: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, __a4: bool| -> metamodelica::Result<_> { ::std::result::Result::Ok(replaceEquations2(__a0, &__a1, __a2, __a3, __a4)) }, repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false)?;
                    (eqns, b3) = replaceEquations2(eqns.clone(), metamodelica::AsArg::as_arg(&repl), inFuncTypeExpExpToBooleanOption.clone(), metamodelica::nil(), false);
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    eqns = optimizeIfEquation(&expl2, metamodelica::AsArg::as_arg(&eqnslst), metamodelica::AsArg::as_arg(&eqns), &(metamodelica::nil()), &(metamodelica::nil()), metamodelica::AsArg::as_arg(&source), eqAttr.clone(), &inAcc)?;
                    Ok((eqns.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (a, _) => {
                    Ok((metamodelica::cons(a.clone(), inAcc.clone()), iReplacementPerformed))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outBackendDAEEquationLst, replacementPerformed)
}

fn optimizeIfEquation(
    mut conditions: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqns: &metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut elseenqs: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut conditions1: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqns1: &metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
    mut inEqAttr: BackendDAE::EquationAttributes,
    mut inEqns: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    outEqns = 'mc: {
        let __mc_input = (&**conditions, &**theneqns, &**conditions1, &**theneqns1);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(listAppend(elseenqs.clone(), inEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _, _) => {
                    let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut eqnslst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                    explst = conditions1.clone().reverse();
                    eqnslst = theneqns1.clone().reverse();
                    Ok(metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::IF_EQUATION { conditions: explst.clone(), eqnstrue: eqnslst.clone(), eqnsfalse: elseenqs.clone(), source: source.clone(), attr: inEqAttr }), inEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BCONST { bool: true }, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: eqns, tail: _ }, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(listAppend(eqns.clone(), inEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BCONST { bool: true }, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: eqns, tail: _ }, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut eqnslst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                    explst = conditions1.clone().reverse();
                    eqnslst = theneqns1.clone().reverse();
                    Ok(metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::IF_EQUATION { conditions: explst.clone(), eqnstrue: eqnslst.clone(), eqnsfalse: eqns.clone(), source: source.clone(), attr: inEqAttr }), inEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BCONST { bool: false }, tail: explst }, Deref @ metamodelica::ListNode::Cons { head: _, tail: eqnslst }, _, _) => {
                    Ok(optimizeIfEquation(metamodelica::AsArg::as_arg(&explst), metamodelica::AsArg::as_arg(&eqnslst), elseenqs, conditions1, theneqns1, source, inEqAttr, inEqns)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: explst }, Deref @ metamodelica::ListNode::Cons { head: eqns, tail: eqnslst }, _, _) => {
                    Ok(optimizeIfEquation(metamodelica::AsArg::as_arg(&explst), metamodelica::AsArg::as_arg(&eqnslst), elseenqs, &(metamodelica::cons(e.clone(), conditions1.clone())), &(metamodelica::cons(eqns.clone(), theneqns1.clone())), source, inEqAttr, inEqns)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outEqns)
}

fn validWhenLeftHandSide(
    mut inLhs: metamodelica::Ref<DAE::Exp>,
    mut inRhs: metamodelica::Ref<DAE::Exp>,
    mut oldCr: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)> {
    let mut outCr: metamodelica::Ref<DAE::ComponentRef>;
    let mut oRhs: metamodelica::Ref<DAE::Exp>;
    (outCr, oRhs) = (::match_deref::match_deref! { match &(inLhs.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            (cr.clone(), inRhs)
        },
        Deref @ DAE::Exp::UNARY { operator: op, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } } => {
            (cr.clone(), metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: inRhs }))
        },
        Deref @ DAE::Exp::LUNARY { operator: op, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } } => {
            (cr.clone(), metamodelica::Ref::new(DAE::Exp::LUNARY { operator: op.clone(), exp: inRhs }))
        },
        _ => {
            let mut msg: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendVarTransform: failed to replace left hand side of when equation ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(oldCr)?); __mm_s.push_str(&*literal!(" with ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inLhs)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
            Debug::trace(msg)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCr, oRhs))
}

fn replaceWhenEquation(
    mut whenEqn: &metamodelica::Ref<BackendDAE::WhenEquation>,
    mut repl: &VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
    mut isource: &metamodelica::Ref<DAE::ElementSource>,
) -> Result<(
    metamodelica::Ref<BackendDAE::WhenEquation>,
    metamodelica::Ref<DAE::ElementSource>,
    bool,
)> {
    let mut outWhenEqn: metamodelica::Ref<BackendDAE::WhenEquation>;
    let mut osource: metamodelica::Ref<DAE::ElementSource>;
    let mut replacementPerformed: bool;
    (outWhenEqn, osource, replacementPerformed) = (match &**whenEqn {
        BackendDAE::WhenEquation {
            condition: cond,
            whenStmtLst,
            elsewhenPart: oelsewhenPart,
        } => {
            let mut cond1: metamodelica::Ref<DAE::Exp>;
            let mut cond2: metamodelica::Ref<DAE::Exp>;
            let mut weqn: metamodelica::Ref<BackendDAE::WhenEquation>;
            let mut b1: bool;
            let mut b2: bool;
            let mut b3: bool;
            let mut b4: bool;
            let mut source: metamodelica::Ref<DAE::ElementSource>;
            let mut elsewhenPart: metamodelica::Ref<BackendDAE::WhenEquation>;
            let mut whenStmtLst = (*whenStmtLst).clone();
            let mut oelsewhenPart = (*oelsewhenPart).clone();
            (cond1, b1) = replaceExp(cond, repl, inFuncTypeExpExpToBooleanOption.clone());
            (cond2, _) = ExpressionSimplify::condsimplify(b1, cond1)?;
            source =
                ElementSource::addSymbolicTransformationSubstitution(b1, isource.clone(), cond.clone(), cond2.clone())?;
            (whenStmtLst, b2) = replaceWhenOperator(
                metamodelica::AsArg::as_arg(&whenStmtLst),
                repl,
                inFuncTypeExpExpToBooleanOption.clone(),
                false,
                metamodelica::nil(),
            )?;
            if (oelsewhenPart).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(oelsewhenPart.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                elsewhenPart = metamodelica::Own::own(__pa0);
                (elsewhenPart, source, b3) =
                    replaceWhenEquation(&elsewhenPart, repl, inFuncTypeExpExpToBooleanOption, &source)?;
                oelsewhenPart = Some(elsewhenPart);
            } else {
                oelsewhenPart = None;
                b3 = false;
            }
            b4 = b1 || b2 || b3;
            weqn = if (b4) {
                metamodelica::Ref::new(BackendDAE::WhenEquation {
                    condition: cond2,
                    whenStmtLst: whenStmtLst.clone(),
                    elsewhenPart: oelsewhenPart.clone(),
                })
            } else {
                whenEqn.clone()
            };
            (weqn, source, b4)
        }
    });
    Ok((outWhenEqn, osource, replacementPerformed))
}

fn replaceWhenOperator<'__b>(
    mut inReinitStmtLst: &'__b metamodelica::List<BackendDAE::WhenOperator>,
    mut repl: &'__b VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
    mut replacementPerformed: bool,
    mut iAcc: metamodelica::List<BackendDAE::WhenOperator>,
) -> Result<(metamodelica::List<BackendDAE::WhenOperator>, bool)> {
    '__tco: loop {
        ::match_deref::match_deref! { match inReinitStmtLst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((iAcc.reverse(), replacementPerformed))
            },
            Deref @ metamodelica::ListNode::Cons { head: wop @ BackendDAE::WhenOperator::ASSIGN { left: cre @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, right: exp, source }, tail: res } => {
                let mut res1: metamodelica::List<BackendDAE::WhenOperator>;
                let mut wop1: BackendDAE::WhenOperator;
                let mut cre1: metamodelica::Ref<DAE::Exp>;
                let mut exp1: metamodelica::Ref<DAE::Exp>;
                let mut b: bool;
                let mut b1: bool;
                let mut b2: bool;
                let mut source = (*source).clone();
                (cre1, b1) = replaceExp(metamodelica::AsArg::as_arg(&cre), repl, inFuncTypeExpExpToBooleanOption.clone());
                validWhenLeftHandSide(cre1.clone(), cre.clone(), metamodelica::AsArg::as_arg(&cr))?;
                source = ElementSource::addSymbolicTransformationSubstitution(b1, source.clone(), cre.clone(), cre1.clone())?;
                (exp1, b2) = replaceExp(metamodelica::AsArg::as_arg(&exp), repl, inFuncTypeExpExpToBooleanOption.clone());
                (exp1, _) = ExpressionSimplify::condsimplify(b2, exp1)?;
                source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), exp.clone(), exp1.clone())?;
                b = b1 || b2;
                wop1 = if (b) {BackendDAE::WhenOperator::ASSIGN { left: cre1, right: exp1, source: source.clone() }} else {wop.clone()};
                { (inReinitStmtLst, repl, inFuncTypeExpExpToBooleanOption, replacementPerformed, iAcc) = (res, repl, inFuncTypeExpExpToBooleanOption, replacementPerformed || b, metamodelica::cons(wop1, iAcc)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: wop @ BackendDAE::WhenOperator::ASSIGN { left: cre, right: exp, source }, tail: res } => {
                let mut res1: metamodelica::List<BackendDAE::WhenOperator>;
                let mut wop1: BackendDAE::WhenOperator;
                let mut cre1: metamodelica::Ref<DAE::Exp>;
                let mut exp1: metamodelica::Ref<DAE::Exp>;
                let mut b: bool;
                let mut b1: bool;
                let mut b2: bool;
                let mut source = (*source).clone();
                (cre1, b1) = replaceExp(metamodelica::AsArg::as_arg(&cre), repl, inFuncTypeExpExpToBooleanOption.clone());
                source = ElementSource::addSymbolicTransformationSubstitution(b1, source.clone(), cre.clone(), cre1.clone())?;
                (exp1, b2) = replaceExp(metamodelica::AsArg::as_arg(&exp), repl, inFuncTypeExpExpToBooleanOption.clone());
                (exp1, _) = ExpressionSimplify::condsimplify(b2, exp1)?;
                source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), exp.clone(), exp1.clone())?;
                b = b1 || b2;
                wop1 = if (b) {BackendDAE::WhenOperator::ASSIGN { left: cre1, right: exp1, source: source.clone() }} else {wop.clone()};
                { (inReinitStmtLst, repl, inFuncTypeExpExpToBooleanOption, replacementPerformed, iAcc) = (res, repl, inFuncTypeExpExpToBooleanOption, replacementPerformed || b, metamodelica::cons(wop1, iAcc)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: wop @ BackendDAE::WhenOperator::REINIT { stateVar: cr, value: cond, source }, tail: res } => {
                let mut res1: metamodelica::List<BackendDAE::WhenOperator>;
                let mut wop1: BackendDAE::WhenOperator;
                let mut cond1: metamodelica::Ref<DAE::Exp>;
                let mut cre: metamodelica::Ref<DAE::Exp>;
                let mut cre1: metamodelica::Ref<DAE::Exp>;
                let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
                let mut b: bool;
                let mut b1: bool;
                let mut b2: bool;
                let mut source = (*source).clone();
                cre = Expression::crefExp(cr.clone())?;
                (cre1, b1) = replaceExp(&cre, repl, inFuncTypeExpExpToBooleanOption.clone());
                (cr1, _) = validWhenLeftHandSide(cre1.clone(), cre.clone(), metamodelica::AsArg::as_arg(&cr))?;
                source = ElementSource::addSymbolicTransformationSubstitution(b1, source.clone(), cre, cre1)?;
                (cond1, b2) = replaceExp(metamodelica::AsArg::as_arg(&cond), repl, inFuncTypeExpExpToBooleanOption.clone());
                (cond1, _) = ExpressionSimplify::condsimplify(b2, cond1)?;
                source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), cond.clone(), cond1.clone())?;
                b = b1 || b2;
                wop1 = if (b) {BackendDAE::WhenOperator::REINIT { stateVar: cr1, value: cond1, source: source.clone() }} else {wop.clone()};
                { (inReinitStmtLst, repl, inFuncTypeExpExpToBooleanOption, replacementPerformed, iAcc) = (res, repl, inFuncTypeExpExpToBooleanOption, replacementPerformed || b, metamodelica::cons(wop1, iAcc)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: wop @ BackendDAE::WhenOperator::ASSERT { condition: cond, message: exp, level, source }, tail: res } => {
                let mut res1: metamodelica::List<BackendDAE::WhenOperator>;
                let mut wop1: BackendDAE::WhenOperator;
                let mut cond1: metamodelica::Ref<DAE::Exp>;
                let mut exp1: metamodelica::Ref<DAE::Exp>;
                let mut b: bool;
                let mut b1: bool;
                let mut b2: bool;
                let mut source = (*source).clone();
                (cond1, b1) = replaceExp(metamodelica::AsArg::as_arg(&cond), repl, inFuncTypeExpExpToBooleanOption.clone());
                (cond1, _) = ExpressionSimplify::condsimplify(b1, cond1)?;
                (exp1, b2) = replaceExp(metamodelica::AsArg::as_arg(&exp), repl, inFuncTypeExpExpToBooleanOption.clone());
                b = b1 || b2;
                source = ElementSource::addSymbolicTransformationSubstitution(b, source.clone(), cond.clone(), cond1.clone())?;
                wop1 = if (b) {BackendDAE::WhenOperator::ASSERT { condition: cond1, message: exp1, level: level.clone(), source: source.clone() }} else {wop.clone()};
                { (inReinitStmtLst, repl, inFuncTypeExpExpToBooleanOption, replacementPerformed, iAcc) = (res, repl, inFuncTypeExpExpToBooleanOption, replacementPerformed || b, metamodelica::cons(wop1, iAcc)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: wop @ BackendDAE::WhenOperator::TERMINATE { message: exp, source }, tail: res } => {
                let mut res1: metamodelica::List<BackendDAE::WhenOperator>;
                let mut wop1: BackendDAE::WhenOperator;
                let mut exp1: metamodelica::Ref<DAE::Exp>;
                let mut b: bool;
                let mut source = (*source).clone();
                (exp1, b) = replaceExp(metamodelica::AsArg::as_arg(&exp), repl, inFuncTypeExpExpToBooleanOption.clone());
                source = ElementSource::addSymbolicTransformationSubstitution(b, source.clone(), exp.clone(), exp1.clone())?;
                wop1 = if (b) {BackendDAE::WhenOperator::TERMINATE { message: exp1, source: source.clone() }} else {wop.clone()};
                { (inReinitStmtLst, repl, inFuncTypeExpExpToBooleanOption, replacementPerformed, iAcc) = (res, repl, inFuncTypeExpExpToBooleanOption, replacementPerformed || b, metamodelica::cons(wop1, iAcc)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: wop @ BackendDAE::WhenOperator::NORETCALL { exp, source }, tail: res } => {
                let mut res1: metamodelica::List<BackendDAE::WhenOperator>;
                let mut wop1: BackendDAE::WhenOperator;
                let mut exp1: metamodelica::Ref<DAE::Exp>;
                let mut b: bool;
                let mut source = (*source).clone();
                (exp1, b) = replaceExp(metamodelica::AsArg::as_arg(&exp), repl, inFuncTypeExpExpToBooleanOption.clone());
                (exp1, _) = ExpressionSimplify::condsimplify(b, exp1)?;
                source = ElementSource::addSymbolicTransformationSubstitution(b, source.clone(), exp.clone(), exp1.clone())?;
                wop1 = if (b) {BackendDAE::WhenOperator::NORETCALL { exp: exp1, source: source.clone() }} else {wop.clone()};
                { (inReinitStmtLst, repl, inFuncTypeExpExpToBooleanOption, replacementPerformed, iAcc) = (res, repl, inFuncTypeExpExpToBooleanOption, replacementPerformed || b, metamodelica::cons(wop1, iAcc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

/* ********************************************************/
/* replace statements  */
/* ********************************************************/
pub(crate) fn replaceStatementLst(
    mut inStatementLst: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inVariableReplacements: VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
    mut inAcc: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inBAcc: bool,
) -> (metamodelica::List<metamodelica::Ref<DAE::Statement>>, bool) {
    let mut outStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut replacementPerformed: bool = inBAcc;
    let mut repl: VariableReplacements = inVariableReplacements;
    let mut statementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut statementLst_1: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut statement: metamodelica::Ref<DAE::Statement>;
    let mut statement_1: metamodelica::Ref<DAE::Statement> =
        <metamodelica::Ref<DAE::Statement> as ::std::default::Default>::default();
    let mut type_: metamodelica::Ref<DAE::Type>;
    let mut e1_1: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut e2_1: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut e1_2: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut e2_2: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut e3: metamodelica::Ref<DAE::Exp>;
    let mut e3_1: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut e3_2: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut expExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expExpLst_1: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut else_: metamodelica::Ref<DAE::Else>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut iterIsArray: bool;
    let mut ident: ArcStr;
    let mut conditions: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut initialCall: bool;
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut b3: bool = false;
    let mut loopPrlVars: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>;
    let mut sub_iters: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    )>;
    for mut stmt in &**inStatementLst {
        (outStatementLst, replacementPerformed) = 'mc: {
            let __mc_input = stmt.clone();
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_ASSIGN { type_, exp1: e1, exp: e2, source } => {
                        let mut source = (*source).clone();
                        let mut b1: bool = b1.clone();
                        let mut b2: bool = b2.clone();
                        let mut e1_1: metamodelica::Ref<DAE::Exp> = e1_1.clone();
                        let mut e1_2: metamodelica::Ref<DAE::Exp> = e1_2.clone();
                        let mut e2_1: metamodelica::Ref<DAE::Exp> = e2_1.clone();
                        let mut e2_2: metamodelica::Ref<DAE::Exp> = e2_2.clone();
                        (e1_1, b1) = replaceExp(metamodelica::AsArg::as_arg(&e1), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        (e2_1, b2) = replaceExp(metamodelica::AsArg::as_arg(&e2), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        let true = (b1 || b2) else { return Err("pattern mismatch") };
                        (e1_2, _) = ExpressionSimplify::simplify(e1_1.clone())?;
                        (e2_2, _) = ExpressionSimplify::simplify(e2_1.clone())?;
                        (e1_2, e2_2) = moveNegateRhs(e1_2.clone(), e2_2.clone());
                        source = ElementSource::addSymbolicTransformationSubstitution(b1, source.clone(), e1.clone(), e1_2.clone())?;
                        source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), e2.clone(), e2_2.clone())?;
                        Ok(((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: type_.clone(), exp1: e1_2.clone(), exp: e2_2.clone(), source: source.clone() }), outStatementLst.clone()), true), b1.clone(), b2.clone(), e1_1.clone(), e1_2.clone(), e2_1.clone(), e2_2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                b1 = __wb0;
                b2 = __wb1;
                e1_1 = __wb2;
                e1_2 = __wb3;
                e2_1 = __wb4;
                e2_2 = __wb5;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_, expExpLst, exp: e2, source } => {
                        let mut source = (*source).clone();
                        let mut b1: bool = b1.clone();
                        let mut b2: bool = b2.clone();
                        let mut e2_1: metamodelica::Ref<DAE::Exp> = e2_1.clone();
                        let mut e2_2: metamodelica::Ref<DAE::Exp> = e2_2.clone();
                        let mut expExpLst_1: metamodelica::List<metamodelica::Ref<DAE::Exp>> = expExpLst_1.clone();
                        (expExpLst_1, b1) = replaceExpList(expExpLst.clone(), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        (e2_1, b2) = replaceExp(metamodelica::AsArg::as_arg(&e2), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        let true = (b1 || b2) else { return Err("pattern mismatch") };
                        source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), e2.clone(), e2_1.clone())?;
                        (e2_2, b1) = ExpressionSimplify::simplify(e2_1.clone())?;
                        source = ElementSource::addSymbolicTransformationSimplify(b1, source.clone(), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e2_1.clone() }), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e2_2.clone() }))?;
                        Ok(((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: type_.clone(), expExpLst: expExpLst_1.clone(), exp: e2_2.clone(), source: source.clone() }), outStatementLst.clone()), true), b1.clone(), b2.clone(), e2_1.clone(), e2_2.clone(), expExpLst_1.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                b1 = __wb0;
                b2 = __wb1;
                e2_1 = __wb2;
                e2_2 = __wb3;
                expExpLst_1 = __wb4;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_, lhs: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, exp: e2, source } => {
                        let mut source = (*source).clone();
                        let mut b1: bool = b1.clone();
                        let mut b2: bool = b2.clone();
                        let mut e1_1: metamodelica::Ref<DAE::Exp> = e1_1.clone();
                        let mut e2_1: metamodelica::Ref<DAE::Exp> = e2_1.clone();
                        let mut e2_2: metamodelica::Ref<DAE::Exp> = e2_2.clone();
                        (e1_1, b1) = replaceExp(metamodelica::AsArg::as_arg(&e1), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        (e2_1, b2) = replaceExp(metamodelica::AsArg::as_arg(&e2), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        let true = (b1 || b2) else { return Err("pattern mismatch") };
                        source = ElementSource::addSymbolicTransformationSubstitution(b1, source.clone(), e1.clone(), e1_1.clone())?;
                        source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), e2.clone(), e2_1.clone())?;
                        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1_1.clone(), rhs: e2_1.clone() }), source.clone())?) {
                            (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        e1_1 = metamodelica::Own::own(__pa0);
                        e2_2 = metamodelica::Own::own(__pa1);
                        source = metamodelica::Own::own(__pa2);
                        Ok(((validLhsArrayAssignSTMT(metamodelica::AsArg::as_arg(&cr), e1_1.clone(), e2_2.clone(), type_.clone(), source.clone(), outStatementLst.clone()), true), b1.clone(), b2.clone(), e1_1.clone(), e2_1.clone(), e2_2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                b1 = __wb0;
                b2 = __wb1;
                e1_1 = __wb2;
                e2_1 = __wb3;
                e2_2 = __wb4;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_IF { exp: e1, statementLst, else_, source } => {
                        let mut source = (*source).clone();
                        let mut b1: bool = b1.clone();
                        let mut e1_1: metamodelica::Ref<DAE::Exp> = e1_1.clone();
                        let mut e1_2: metamodelica::Ref<DAE::Exp> = e1_2.clone();
                        (e1_1, b1) = replaceExp(metamodelica::AsArg::as_arg(&e1), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        (e1_2, _) = ExpressionSimplify::condsimplify(b1, e1_1.clone())?;
                        source = ElementSource::addSymbolicTransformationSubstitution(b1, source.clone(), e1.clone(), e1_2.clone())?;
                        Ok((replaceSTMT_IF(e1_2.clone(), statementLst.clone(), else_.clone(), source.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), &outStatementLst, replacementPerformed || b1), b1.clone(), e1_1.clone(), e1_2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                b1 = __wb0;
                e1_1 = __wb1;
                e1_2 = __wb2;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_FOR { type_, iterIsArray, iter: ident, range: e1, statementLst, source, sub_iters } => {
                        let mut source = (*source).clone();
                        let mut b1: bool = b1.clone();
                        let mut b2: bool = b2.clone();
                        let mut e1_1: metamodelica::Ref<DAE::Exp> = e1_1.clone();
                        let mut e1_2: metamodelica::Ref<DAE::Exp> = e1_2.clone();
                        let mut repl: VariableReplacements = repl.clone();
                        let mut statementLst_1: metamodelica::List<metamodelica::Ref<DAE::Statement>> = statementLst_1.clone();
                        repl = addIterationVar(repl.clone(), ident.clone());
                        (statementLst_1, b1) = replaceStatementLst(metamodelica::AsArg::as_arg(&statementLst), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), &(metamodelica::nil()), false);
                        (e1_1, b2) = replaceExp(metamodelica::AsArg::as_arg(&e1), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        let true = (b1 || b2) else { return Err("pattern mismatch") };
                        source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), e1.clone(), e1_1.clone())?;
                        (e1_2, b1) = ExpressionSimplify::condsimplify(b2, e1_1.clone())?;
                        source = ElementSource::addSymbolicTransformationSimplify(b1, source.clone(), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_1.clone() }), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_2.clone() }))?;
                        repl = removeIterationVar(repl.clone(), ident.clone())?;
                        Ok(((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_FOR { type_: type_.clone(), iterIsArray: iterIsArray.clone(), iter: ident.clone(), range: e1_2.clone(), statementLst: statementLst_1.clone(), source: source.clone(), sub_iters: sub_iters.clone() }), outStatementLst.clone()), true), b1.clone(), b2.clone(), e1_1.clone(), e1_2.clone(), repl.clone(), statementLst_1.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                b1 = __wb0;
                b2 = __wb1;
                e1_1 = __wb2;
                e1_2 = __wb3;
                repl = __wb4;
                statementLst_1 = __wb5;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_PARFOR { type_, iterIsArray, iter: ident, range: e1, statementLst, loopPrlVars, source } => {
                        let mut source = (*source).clone();
                        let mut b1: bool = b1.clone();
                        let mut b2: bool = b2.clone();
                        let mut e1_1: metamodelica::Ref<DAE::Exp> = e1_1.clone();
                        let mut e1_2: metamodelica::Ref<DAE::Exp> = e1_2.clone();
                        let mut statementLst_1: metamodelica::List<metamodelica::Ref<DAE::Statement>> = statementLst_1.clone();
                        (statementLst_1, b1) = replaceStatementLst(metamodelica::AsArg::as_arg(&statementLst), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), &(metamodelica::nil()), false);
                        (e1_1, b2) = replaceExp(metamodelica::AsArg::as_arg(&e1), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        let true = (b1 || b2) else { return Err("pattern mismatch") };
                        source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), e1.clone(), e1_1.clone())?;
                        (e1_2, b1) = ExpressionSimplify::condsimplify(b2, e1_1.clone())?;
                        source = ElementSource::addSymbolicTransformationSimplify(b1, source.clone(), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_1.clone() }), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_2.clone() }))?;
                        Ok(((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_PARFOR { type_: type_.clone(), iterIsArray: iterIsArray.clone(), iter: ident.clone(), range: e1_2.clone(), statementLst: statementLst_1.clone(), loopPrlVars: loopPrlVars.clone(), source: source.clone() }), outStatementLst.clone()), true), b1.clone(), b2.clone(), e1_1.clone(), e1_2.clone(), statementLst_1.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                b1 = __wb0;
                b2 = __wb1;
                e1_1 = __wb2;
                e1_2 = __wb3;
                statementLst_1 = __wb4;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_WHILE { exp: e1, statementLst, source } => {
                        let mut source = (*source).clone();
                        let mut b1: bool = b1.clone();
                        let mut b2: bool = b2.clone();
                        let mut e1_1: metamodelica::Ref<DAE::Exp> = e1_1.clone();
                        let mut e1_2: metamodelica::Ref<DAE::Exp> = e1_2.clone();
                        let mut statementLst_1: metamodelica::List<metamodelica::Ref<DAE::Statement>> = statementLst_1.clone();
                        (statementLst_1, b1) = replaceStatementLst(metamodelica::AsArg::as_arg(&statementLst), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), &(metamodelica::nil()), false);
                        (e1_1, b2) = replaceExp(metamodelica::AsArg::as_arg(&e1), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        let true = (b1 || b2) else { return Err("pattern mismatch") };
                        source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), e1.clone(), e1_1.clone())?;
                        (e1_2, b1) = ExpressionSimplify::condsimplify(b2, e1_1.clone())?;
                        source = ElementSource::addSymbolicTransformationSimplify(b1, source.clone(), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_1.clone() }), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_2.clone() }))?;
                        Ok(((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_WHILE { exp: e1_2.clone(), statementLst: statementLst_1.clone(), source: source.clone() }), outStatementLst.clone()), true), b1.clone(), b2.clone(), e1_1.clone(), e1_2.clone(), statementLst_1.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                b1 = __wb0;
                b2 = __wb1;
                e1_1 = __wb2;
                e1_2 = __wb3;
                statementLst_1 = __wb4;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_WHEN { exp: e1, conditions, initialCall, statementLst, elseWhen: None, source } => {
                        let mut source = (*source).clone();
                        let mut b1: bool = b1.clone();
                        let mut b2: bool = b2.clone();
                        let mut e1_1: metamodelica::Ref<DAE::Exp> = e1_1.clone();
                        let mut e1_2: metamodelica::Ref<DAE::Exp> = e1_2.clone();
                        let mut statementLst_1: metamodelica::List<metamodelica::Ref<DAE::Statement>> = statementLst_1.clone();
                        (statementLst_1, b1) = replaceStatementLst(metamodelica::AsArg::as_arg(&statementLst), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), &(metamodelica::nil()), false);
                        (e1_1, b2) = replaceExp(metamodelica::AsArg::as_arg(&e1), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        let true = (b1 || b2) else { return Err("pattern mismatch") };
                        source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), e1.clone(), e1_1.clone())?;
                        (e1_2, b1) = ExpressionSimplify::condsimplify(b2, e1_1.clone())?;
                        source = ElementSource::addSymbolicTransformationSimplify(b1, source.clone(), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_1.clone() }), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_2.clone() }))?;
                        Ok(((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e1_2.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: statementLst_1.clone(), elseWhen: None, source: source.clone() }), outStatementLst.clone()), true), b1.clone(), b2.clone(), e1_1.clone(), e1_2.clone(), statementLst_1.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                b1 = __wb0;
                b2 = __wb1;
                e1_1 = __wb2;
                e1_2 = __wb3;
                statementLst_1 = __wb4;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_WHEN { exp: e1, conditions, initialCall, statementLst, elseWhen: Some(statement), source } => {
                        let mut source = (*source).clone();
                        let mut b1: bool = b1.clone();
                        let mut b2: bool = b2.clone();
                        let mut b3: bool = b3.clone();
                        let mut e1_1: metamodelica::Ref<DAE::Exp> = e1_1.clone();
                        let mut e1_2: metamodelica::Ref<DAE::Exp> = e1_2.clone();
                        let mut statementLst_1: metamodelica::List<metamodelica::Ref<DAE::Statement>> = statementLst_1.clone();
                        let mut statement_1: metamodelica::Ref<DAE::Statement> = statement_1.clone();
                        (statementLst_1, b1) = replaceStatementLst(metamodelica::AsArg::as_arg(&statementLst), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), &(metamodelica::nil()), false);
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(replaceStatementLst(&(list![statement.clone()]), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), &(metamodelica::nil()), false)) {
                            (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        statement_1 = metamodelica::Own::own(__pa0);
                        b2 = metamodelica::Own::own(__pa1);
                        (e1_1, b3) = replaceExp(metamodelica::AsArg::as_arg(&e1), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                        source = ElementSource::addSymbolicTransformationSubstitution(b3, source.clone(), e1.clone(), e1_1.clone())?;
                        (e1_2, b1) = ExpressionSimplify::condsimplify(b3, e1_1.clone())?;
                        source = ElementSource::addSymbolicTransformationSimplify(b1, source.clone(), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_1.clone() }), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_2.clone() }))?;
                        Ok(((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e1_2.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: statementLst_1.clone(), elseWhen: Some(statement_1.clone()), source: source.clone() }), outStatementLst.clone()), true), b1.clone(), b2.clone(), b3.clone(), e1_1.clone(), e1_2.clone(), statementLst_1.clone(), statement_1.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                b1 = __wb0;
                b2 = __wb1;
                b3 = __wb2;
                e1_1 = __wb3;
                e1_2 = __wb4;
                statementLst_1 = __wb5;
                statement_1 = __wb6;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_ASSERT { cond: e1, msg: e2, level: e3, source } => {
                        let mut source = (*source).clone();
                        let mut b1: bool = b1.clone();
                        let mut b2: bool = b2.clone();
                        let mut b3: bool = b3.clone();
                        let mut e1_1: metamodelica::Ref<DAE::Exp> = e1_1.clone();
                        let mut e1_2: metamodelica::Ref<DAE::Exp> = e1_2.clone();
                        let mut e2_1: metamodelica::Ref<DAE::Exp> = e2_1.clone();
                        let mut e2_2: metamodelica::Ref<DAE::Exp> = e2_2.clone();
                        let mut e3_1: metamodelica::Ref<DAE::Exp> = e3_1.clone();
                        let mut e3_2: metamodelica::Ref<DAE::Exp> = e3_2.clone();
                        (e1_1, b1) = replaceExp(metamodelica::AsArg::as_arg(&e1), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        (e2_1, b2) = replaceExp(metamodelica::AsArg::as_arg(&e2), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        (e3_1, b3) = replaceExp(metamodelica::AsArg::as_arg(&e3), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                        (e1_2, _) = ExpressionSimplify::condsimplify(b1, e1_1.clone())?;
                        (e2_2, _) = ExpressionSimplify::condsimplify(b2, e2_1.clone())?;
                        (e3_2, _) = ExpressionSimplify::condsimplify(b3, e3_1.clone())?;
                        source = ElementSource::addSymbolicTransformationSubstitution(b1, source.clone(), e1.clone(), e1_2.clone())?;
                        source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), e2.clone(), e2_2.clone())?;
                        source = ElementSource::addSymbolicTransformationSubstitution(b3, source.clone(), e3.clone(), e3_2.clone())?;
                        Ok(((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSERT { cond: e1_2.clone(), msg: e2_2.clone(), level: e3_2.clone(), source: source.clone() }), outStatementLst.clone()), true), b1.clone(), b2.clone(), b3.clone(), e1_1.clone(), e1_2.clone(), e2_1.clone(), e2_2.clone(), e3_1.clone(), e3_2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                b1 = __wb0;
                b2 = __wb1;
                b3 = __wb2;
                e1_1 = __wb3;
                e1_2 = __wb4;
                e2_1 = __wb5;
                e2_2 = __wb6;
                e3_1 = __wb7;
                e3_2 = __wb8;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_TERMINATE { msg: e1, source } => {
                        let mut source = (*source).clone();
                        let mut b1: bool = b1.clone();
                        let mut e1_1: metamodelica::Ref<DAE::Exp> = e1_1.clone();
                        let mut e1_2: metamodelica::Ref<DAE::Exp> = e1_2.clone();
                        let __pa0 = ::match_deref::match_deref! { match &(replaceExp(metamodelica::AsArg::as_arg(&e1), &repl, inFuncTypeExpExpToBooleanOption.clone())) {
                            (__pa0, true) => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        e1_1 = metamodelica::Own::own(__pa0);
                        source = ElementSource::addSymbolicTransformationSubstitution(true, source.clone(), e1.clone(), e1_1.clone())?;
                        (e1_2, b1) = ExpressionSimplify::simplify(e1_1.clone())?;
                        source = ElementSource::addSymbolicTransformationSimplify(b1, source.clone(), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_1.clone() }), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_2.clone() }))?;
                        Ok(((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE { msg: e1_2.clone(), source: source.clone() }), outStatementLst.clone()), true), b1.clone(), e1_1.clone(), e1_2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                b1 = __wb0;
                e1_1 = __wb1;
                e1_2 = __wb2;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_REINIT { var: e1, value: e2, source } => {
                        let mut source = (*source).clone();
                        let mut b1: bool = b1.clone();
                        let mut b2: bool = b2.clone();
                        let mut e1_1: metamodelica::Ref<DAE::Exp> = e1_1.clone();
                        let mut e1_2: metamodelica::Ref<DAE::Exp> = e1_2.clone();
                        let mut e2_1: metamodelica::Ref<DAE::Exp> = e2_1.clone();
                        let mut e2_2: metamodelica::Ref<DAE::Exp> = e2_2.clone();
                        (e1_1, b1) = replaceExp(metamodelica::AsArg::as_arg(&e1), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        (e2_1, b2) = replaceExp(metamodelica::AsArg::as_arg(&e2), &repl, inFuncTypeExpExpToBooleanOption.clone());
                        let true = (b1 || b2) else { return Err("pattern mismatch") };
                        (e1_2, _) = ExpressionSimplify::condsimplify(b1, e1_1.clone())?;
                        (e2_2, _) = ExpressionSimplify::condsimplify(b2, e2_1.clone())?;
                        source = ElementSource::addSymbolicTransformationSubstitution(b1, source.clone(), e1.clone(), e1_2.clone())?;
                        source = ElementSource::addSymbolicTransformationSubstitution(b2, source.clone(), e2.clone(), e2_2.clone())?;
                        Ok(((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_REINIT { var: e1_2.clone(), value: e2_2.clone(), source: source.clone() }), outStatementLst.clone()), true), b1.clone(), b2.clone(), e1_1.clone(), e1_2.clone(), e2_1.clone(), e2_2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                b1 = __wb0;
                b2 = __wb1;
                e1_1 = __wb2;
                e1_2 = __wb3;
                e2_1 = __wb4;
                e2_2 = __wb5;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_NORETCALL { exp: e1, source } => {
                        let mut source = (*source).clone();
                        let mut b1: bool = b1.clone();
                        let mut e1_1: metamodelica::Ref<DAE::Exp> = e1_1.clone();
                        let mut e1_2: metamodelica::Ref<DAE::Exp> = e1_2.clone();
                        let __pa0 = ::match_deref::match_deref! { match &(replaceExp(metamodelica::AsArg::as_arg(&e1), &repl, inFuncTypeExpExpToBooleanOption.clone())) {
                            (__pa0, true) => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        e1_1 = metamodelica::Own::own(__pa0);
                        source = ElementSource::addSymbolicTransformationSubstitution(true, source.clone(), e1.clone(), e1_1.clone())?;
                        (e1_2, b1) = ExpressionSimplify::simplify(e1_1.clone())?;
                        source = ElementSource::addSymbolicTransformationSimplify(b1, source.clone(), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_1.clone() }), metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1_2.clone() }))?;
                        Ok(((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: e1_2.clone(), source: source.clone() }), outStatementLst.clone()), true), b1.clone(), e1_1.clone(), e1_2.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                b1 = __wb0;
                e1_1 = __wb1;
                e1_2 = __wb2;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Statement::STMT_FAILURE { body: statementLst, source } => {
                        let mut statementLst_1: metamodelica::List<metamodelica::Ref<DAE::Statement>> = statementLst_1.clone();
                        let __pa0 = ::match_deref::match_deref! { match &(replaceStatementLst(metamodelica::AsArg::as_arg(&statementLst), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), &(metamodelica::nil()), false)) {
                            (__pa0, true) => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        statementLst_1 = metamodelica::Own::own(__pa0);
                        Ok(((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_FAILURE { body: statementLst_1.clone(), source: source.clone() }), outStatementLst.clone()), true), statementLst_1.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                statementLst_1 = __wb0;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok((metamodelica::cons(stmt.clone(), outStatementLst.clone()), replacementPerformed))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        };
    }
    outStatementLst = metamodelica::Dangerous::listReverseInPlace(outStatementLst);
    (outStatementLst, replacementPerformed)
}

pub(crate) fn replaceStatementLstRHS(
    mut inStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inVariableReplacements: VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
    mut inAcc: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inBAcc: bool,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Statement>>, bool)> {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut replacementPerformed: bool;
    let (__pa0, (_, _, __pa1)) = DAEUtil::traverseDAEEquationsStmtsRhsOnly(
        inStatementLst,
        (std::sync::Arc::new(fnptr!(
            replaceExpWrapper,
            metamodelica::Ref<DAE::Exp>,
            (
                VariableReplacements,
                Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
                bool
            )
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            VariableReplacements,
                            Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
                            bool,
                        ),
                    ) -> Result<(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            VariableReplacements,
                            Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
                            bool,
                        ),
                    )> + 'static,
            >),
        (inVariableReplacements, inFuncTypeExpExpToBooleanOption, false),
    )?;
    outStatementLst = metamodelica::Own::own(__pa0);
    replacementPerformed = metamodelica::Own::own(__pa1);
    Ok((outStatementLst, replacementPerformed))
}

fn replaceExpWrapper(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        VariableReplacements,
        Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
        bool,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        VariableReplacements,
        Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
        bool,
    ),
) {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut tpl: (
        VariableReplacements,
        Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
        bool,
    );
    let mut b1: bool;
    let mut b2: bool;
    let mut repl: VariableReplacements;
    let mut opt: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>;
    exp = inExp;
    tpl = inTpl;
    (repl, opt, b1) = tpl;
    (exp, b2) = replaceExp(&exp, &repl, opt.clone());
    b2 = b1 || b2;
    tpl = (repl, opt, b2);
    (exp, tpl)
}

fn moveNegateRhs(
    mut inLhs: metamodelica::Ref<DAE::Exp>,
    mut inRhs: metamodelica::Ref<DAE::Exp>,
) -> (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) {
    let mut outLhs: metamodelica::Ref<DAE::Exp>;
    let mut outRhs: metamodelica::Ref<DAE::Exp>;
    (outLhs, outRhs) = (match &*inLhs {
        DAE::Exp::LUNARY {
            operator: DAE::Operator::NOT { ty },
            exp: e,
        } => (
            e.clone(),
            metamodelica::Ref::new(DAE::Exp::LUNARY {
                operator: DAE::Operator::NOT { ty: ty.clone() },
                exp: inRhs,
            }),
        ),
        DAE::Exp::UNARY {
            operator: DAE::Operator::UMINUS { ty },
            exp: e,
        } => (
            e.clone(),
            metamodelica::Ref::new(DAE::Exp::UNARY {
                operator: DAE::Operator::UMINUS { ty: ty.clone() },
                exp: inRhs,
            }),
        ),
        DAE::Exp::UNARY {
            operator: DAE::Operator::UMINUS_ARR { ty },
            exp: e,
        } => (
            e.clone(),
            metamodelica::Ref::new(DAE::Exp::UNARY {
                operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() },
                exp: inRhs,
            }),
        ),
        _ => (inLhs, inRhs),
    });
    (outLhs, outRhs)
}

fn validLhsArrayAssignSTMT(
    mut oldCr: &metamodelica::Ref<DAE::ComponentRef>,
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut type_: metamodelica::Ref<DAE::Type>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut inStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Statement>> {
    let mut outStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    outStatementLst = 'mc: {
        let __mc_input = lhs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                crefexp => {
                    Ok(metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: type_.clone(), lhs: crefexp.clone(), exp: rhs.clone(), source: source.clone() }), inStatementLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp }, exp: crefexp } => {
                    Ok(metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: type_.clone(), lhs: crefexp.clone(), exp: metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp.clone() }, exp: rhs.clone() }), source: source.clone() }), inStatementLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp }, exp: crefexp } => {
                    Ok(metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: type_.clone(), lhs: crefexp.clone(), exp: metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp.clone() }, exp: rhs.clone() }), source: source.clone() }), inStatementLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: tp }, exp: crefexp } => {
                    Ok(metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: type_.clone(), lhs: crefexp.clone(), exp: metamodelica::Ref::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: tp.clone() }, exp: rhs.clone() }), source: source.clone() }), inStatementLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: elst, .. } => {
                    let mut statementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut elst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut ds: metamodelica::List<i32>;
                    let mut subslst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
                    ds = Expression::dimensionsSizes(Expression::arrayDimension(&type_))?;
                    subslst = Expression::dimensionSizesSubscripts(ds.clone())?;
                    subslst = Expression::rangesToSubscripts(&subslst)?;
                    elst1 = List::map1r(subslst.clone(), &Expression::applyExpSubscripts, rhs.clone())?;
                    e = (elst1).head().cloned()?;
                    tp = Expression::r#typeof(e.clone())?;
                    statementLst = List::threadFold2(metamodelica::AsArg::as_arg(&elst), elst1.clone(), &validLhsAssignSTMT, tp.clone(), source.clone(), inStatementLst.clone())?;
                    Ok(statementLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut msg: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendVarTransform: failed to replace left hand side of array assign statement ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(oldCr)?); __mm_s.push_str(&*literal!(" with ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(lhs.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                    Debug::trace(msg.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStatementLst
}

fn validLhsAssignSTMT(
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut type_: metamodelica::Ref<DAE::Type>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut inStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    let mut outStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    outStatementLst = (::match_deref::match_deref! { match &(lhs.clone()) {
        Deref @ DAE::Exp::CREF { .. } => {
            metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: type_, exp1: lhs, exp: rhs, source: source }), inStatementLst)
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp }, exp: Deref @ DAE::Exp::CREF { .. } } => {
            metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: type_, exp1: lhs, exp: metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp.clone() }, exp: rhs }), source: source }), inStatementLst)
        },
        Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: tp }, exp: Deref @ DAE::Exp::CREF { .. } } => {
            metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: type_, exp1: lhs, exp: metamodelica::Ref::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: tp.clone() }, exp: rhs }), source: source }), inStatementLst)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outStatementLst)
}

fn replaceElse(
    mut inElse: metamodelica::Ref<DAE::Else>,
    mut inVariableReplacements: VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> (metamodelica::Ref<DAE::Else>, bool) {
    let mut outElse: metamodelica::Ref<DAE::Else>;
    let mut replacementPerformed: bool;
    (outElse, replacementPerformed) = 'mc: {
        let __mc_input = (&*inElse, inVariableReplacements);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Else::ELSEIF { exp: e1, statementLst, else_ }, repl) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e1_2: metamodelica::Ref<DAE::Exp>;
                    let mut else_1: metamodelica::Ref<DAE::Else>;
                    let mut b1: bool;
                    let mut b2: bool;
                    (e1_1, b1) = replaceExp(metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&repl), inFuncTypeExpExpToBooleanOption.clone());
                    (e1_2, _) = ExpressionSimplify::condsimplify(b1, e1_1.clone())?;
                    (else_1, b2) = replaceElse1(e1_2.clone(), statementLst.clone(), else_.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((else_1.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Else::ELSE { statementLst }, repl) => {
                    let mut statementLst_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceStatementLst(metamodelica::AsArg::as_arg(&statementLst), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), &(metamodelica::nil()), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    statementLst_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Else::ELSE { statementLst: statementLst_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inElse.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outElse, replacementPerformed)
}

fn replaceElse1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inElse: metamodelica::Ref<DAE::Else>,
    mut inVariableReplacements: VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> (metamodelica::Ref<DAE::Else>, bool) {
    let mut outElse: metamodelica::Ref<DAE::Else>;
    let mut replacementPerformed: bool;
    (outElse, replacementPerformed) = 'mc: {
        let __mc_input = (inExp, inStatementLst, inElse, inVariableReplacements);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: true }, statementLst, _, repl) => {
                    let mut statementLst_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (statementLst_1, _) = replaceStatementLst(metamodelica::AsArg::as_arg(&statementLst), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), &(metamodelica::nil()), false);
                    Ok((metamodelica::Ref::new(DAE::Else::ELSE { statementLst: statementLst_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, else_, repl) => {
                    let mut else_1: metamodelica::Ref<DAE::Else>;
                    (else_1, _) = replaceElse(else_.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone());
                    Ok((else_1.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, statementLst, else_, repl) => {
                    let mut statementLst_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut else_1: metamodelica::Ref<DAE::Else>;
                    let mut b1: bool;
                    let mut b2: bool;
                    (statementLst_1, b1) = replaceStatementLst(metamodelica::AsArg::as_arg(&statementLst), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), &(metamodelica::nil()), false);
                    (else_1, b2) = replaceElse(else_.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Else::ELSEIF { exp: e1.clone(), statementLst: statementLst_1.clone(), else_: else_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, statementLst, else_, _) => {
                    Ok((metamodelica::Ref::new(DAE::Else::ELSEIF { exp: e1.clone(), statementLst: statementLst.clone(), else_: else_.clone() }), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outElse, replacementPerformed)
}

fn replaceSTMT_IF(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inElse: metamodelica::Ref<DAE::Else>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inVariableReplacements: VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
    mut inAcc: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inBAcc: bool,
) -> (metamodelica::List<metamodelica::Ref<DAE::Statement>>, bool) {
    let mut outStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut replacementPerformed: bool;
    (outStatementLst, replacementPerformed) = 'mc: {
        let __mc_input = (inExp, inStatementLst, inElse, inSource, inVariableReplacements);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: true }, statementLst, _, _, repl) => {
                    Ok(replaceStatementLst(metamodelica::AsArg::as_arg(&statementLst), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), inAcc, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, Deref @ DAE::Else::NOELSE { .. }, _, _) => {
                    Ok((inAcc.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, Deref @ DAE::Else::ELSEIF { exp: exp_e, statementLst: statementLst_e, else_: else_e }, source, repl) => {
                    Ok(replaceSTMT_IF(exp_e.clone(), statementLst_e.clone(), else_e.clone(), source.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), inAcc, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, Deref @ DAE::Else::ELSE { statementLst: statementLst_e }, _, repl) => {
                    Ok(replaceStatementLst(metamodelica::AsArg::as_arg(&statementLst_e), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), inAcc, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, statementLst, else_, source, repl) => {
                    let mut statementLst_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut else_1: metamodelica::Ref<DAE::Else>;
                    let mut b1: bool;
                    let mut b2: bool;
                    (statementLst_1, b1) = replaceStatementLst(metamodelica::AsArg::as_arg(&statementLst), repl.clone(), inFuncTypeExpExpToBooleanOption.clone(), &(metamodelica::nil()), false);
                    (else_1, b2) = replaceElse(else_.clone(), repl.clone(), inFuncTypeExpExpToBooleanOption.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: statementLst_1.clone(), else_: else_1.clone(), source: source.clone() }), inAcc.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, statementLst, else_, source, _) => {
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: statementLst.clone(), else_: else_.clone(), source: source.clone() }), inAcc.clone()), inBAcc))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outStatementLst, replacementPerformed)
}

/* ********************************************************/
/* variable replacements  */
/* ********************************************************/
pub(crate) fn replaceVarTraverser(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inRepl: VariableReplacements,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, VariableReplacements)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut repl: VariableReplacements = inRepl.clone();
    outVar = replaceBindingExp(inVar, &inRepl)?;
    outVar = replaceVariableAttributesInVar(outVar, &inRepl)?;
    Ok((outVar, repl))
}

pub(crate) fn replaceBindingExp(
    mut varIn: metamodelica::Ref<BackendDAE::Var>,
    mut repl: &VariableReplacements,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut varOut: metamodelica::Ref<BackendDAE::Var>;
    varOut = (::match_deref::match_deref! { match &(varIn.clone()) {
        Deref @ BackendDAE::Var { bindExp: Some(exp), .. } => {
            let mut exp = (*exp).clone();
            (exp, _) = replaceExp(metamodelica::AsArg::as_arg(&exp), repl, None);
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            BackendVariable::setBindExp(varIn, Some(exp.clone()))
        },
        Deref @ BackendDAE::Var { bindExp: None, .. } => {
            varIn
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(varOut)
}

pub(crate) fn replaceVariableAttributes(
    mut attrIn: metamodelica::Ref<DAE::VariableAttributes>,
    mut repl: &VariableReplacements,
) -> Result<metamodelica::Ref<DAE::VariableAttributes>> {
    let mut attrOut: metamodelica::Ref<DAE::VariableAttributes>;
    attrOut = (match &*attrIn {
        DAE::VariableAttributes::VAR_ATTR_REAL {
            quantity,
            unit,
            displayUnit,
            min,
            max,
            start,
            fixed,
            nominal,
            stateSelectOption,
            uncertainOption,
            distributionOption,
            equationBound,
            isProtected,
            finalPrefix,
            startOrigin,
        } => {
            let mut quantity = (*quantity).clone();
            let mut unit = (*unit).clone();
            let mut displayUnit = (*displayUnit).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut nominal = (*nominal).clone();
            let mut equationBound = (*equationBound).clone();
            quantity = replaceOptionExp(quantity.clone(), repl)?;
            unit = replaceOptionExp(unit.clone(), repl)?;
            displayUnit = replaceOptionExp(displayUnit.clone(), repl)?;
            min = replaceOptionExp(min.clone(), repl)?;
            max = replaceOptionExp(max.clone(), repl)?;
            start = replaceOptionExp(start.clone(), repl)?;
            fixed = replaceOptionExp(fixed.clone(), repl)?;
            nominal = replaceOptionExp(nominal.clone(), repl)?;
            equationBound = replaceOptionExp(equationBound.clone(), repl)?;
            metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL {
                quantity: quantity.clone(),
                unit: unit.clone(),
                displayUnit: displayUnit.clone(),
                min: min.clone(),
                max: max.clone(),
                start: start.clone(),
                fixed: fixed.clone(),
                nominal: nominal.clone(),
                stateSelectOption: stateSelectOption.clone(),
                uncertainOption: uncertainOption.clone(),
                distributionOption: distributionOption.clone(),
                equationBound: equationBound.clone(),
                isProtected: isProtected.clone(),
                finalPrefix: finalPrefix.clone(),
                startOrigin: startOrigin.clone(),
            })
        }
        DAE::VariableAttributes::VAR_ATTR_INT {
            quantity,
            min,
            max,
            start,
            fixed,
            uncertainOption,
            distributionOption,
            equationBound,
            isProtected,
            finalPrefix,
            startOrigin,
        } => {
            let mut quantity = (*quantity).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut equationBound = (*equationBound).clone();
            quantity = replaceOptionExp(quantity.clone(), repl)?;
            min = replaceOptionExp(min.clone(), repl)?;
            max = replaceOptionExp(max.clone(), repl)?;
            start = replaceOptionExp(start.clone(), repl)?;
            fixed = replaceOptionExp(fixed.clone(), repl)?;
            equationBound = replaceOptionExp(equationBound.clone(), repl)?;
            metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_INT {
                quantity: quantity.clone(),
                min: min.clone(),
                max: max.clone(),
                start: start.clone(),
                fixed: fixed.clone(),
                uncertainOption: uncertainOption.clone(),
                distributionOption: distributionOption.clone(),
                equationBound: equationBound.clone(),
                isProtected: isProtected.clone(),
                finalPrefix: finalPrefix.clone(),
                startOrigin: startOrigin.clone(),
            })
        }
        DAE::VariableAttributes::VAR_ATTR_BOOL {
            quantity,
            start,
            fixed,
            equationBound,
            isProtected,
            finalPrefix,
            startOrigin,
        } => {
            let mut quantity = (*quantity).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut equationBound = (*equationBound).clone();
            quantity = replaceOptionExp(quantity.clone(), repl)?;
            start = replaceOptionExp(start.clone(), repl)?;
            fixed = replaceOptionExp(fixed.clone(), repl)?;
            equationBound = replaceOptionExp(equationBound.clone(), repl)?;
            metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_BOOL {
                quantity: quantity.clone(),
                start: start.clone(),
                fixed: fixed.clone(),
                equationBound: equationBound.clone(),
                isProtected: isProtected.clone(),
                finalPrefix: finalPrefix.clone(),
                startOrigin: startOrigin.clone(),
            })
        }
        DAE::VariableAttributes::VAR_ATTR_STRING {
            quantity,
            start,
            fixed,
            equationBound,
            isProtected,
            finalPrefix,
            startOrigin,
        } => {
            let mut quantity = (*quantity).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut equationBound = (*equationBound).clone();
            quantity = replaceOptionExp(quantity.clone(), repl)?;
            start = replaceOptionExp(start.clone(), repl)?;
            fixed = replaceOptionExp(fixed.clone(), repl)?;
            equationBound = replaceOptionExp(equationBound.clone(), repl)?;
            metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_STRING {
                quantity: quantity.clone(),
                start: start.clone(),
                fixed: fixed.clone(),
                equationBound: equationBound.clone(),
                isProtected: isProtected.clone(),
                finalPrefix: finalPrefix.clone(),
                startOrigin: startOrigin.clone(),
            })
        }
        DAE::VariableAttributes::VAR_ATTR_ENUMERATION {
            quantity,
            min,
            max,
            start,
            fixed,
            equationBound,
            isProtected,
            finalPrefix,
            startOrigin,
        } => {
            let mut quantity = (*quantity).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut start = (*start).clone();
            let mut fixed = (*fixed).clone();
            let mut equationBound = (*equationBound).clone();
            quantity = replaceOptionExp(quantity.clone(), repl)?;
            min = replaceOptionExp(min.clone(), repl)?;
            max = replaceOptionExp(max.clone(), repl)?;
            start = replaceOptionExp(start.clone(), repl)?;
            fixed = replaceOptionExp(fixed.clone(), repl)?;
            equationBound = replaceOptionExp(equationBound.clone(), repl)?;
            metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION {
                quantity: quantity.clone(),
                min: min.clone(),
                max: max.clone(),
                start: start.clone(),
                fixed: fixed.clone(),
                equationBound: equationBound.clone(),
                isProtected: isProtected.clone(),
                finalPrefix: finalPrefix.clone(),
                startOrigin: startOrigin.clone(),
            })
        }
        _ => attrIn,
    });
    Ok(attrOut)
}

pub(crate) fn replaceOptionExp(
    mut optIn: Option<metamodelica::Ref<DAE::Exp>>,
    mut repl: &VariableReplacements,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut optOut: Option<metamodelica::Ref<DAE::Exp>>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    if (optIn).is_some() {
        exp = Util::getOption(optIn)?;
        (exp, _) = replaceExp(&exp, repl, None);
        optOut = Some(exp);
    } else {
        optOut = None;
    }
    Ok(optOut)
}

pub(crate) fn replaceVariableAttributesInVar(
    mut varIn: metamodelica::Ref<BackendDAE::Var>,
    mut repl: &VariableReplacements,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut varOut: metamodelica::Ref<BackendDAE::Var>;
    varOut = (::match_deref::match_deref! { match &(varIn.clone()) {
        Deref @ BackendDAE::Var { values: Some(values), .. } => {
            let mut values = (*values).clone();
            values = replaceVariableAttributes(values.clone(), repl)?;
            BackendVariable::setVarAttributes(varIn, Some(values.clone()))
        },
        _ => {
            varIn
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(varOut)
}

fn negateOperator(mut inOp: DAE::Operator) -> DAE::Operator {
    let mut outOp: DAE::Operator;
    outOp = (match inOp.clone() {
        DAE::Operator::UMINUS { ty: mut ty } => DAE::Operator::ADD { ty: ty.clone() },
        DAE::Operator::SUB { ty: mut ty } => DAE::Operator::ADD { ty: ty.clone() },
        DAE::Operator::ADD { ty: mut ty } => DAE::Operator::SUB { ty: ty.clone() },
        _ => inOp,
    });
    outOp
}

pub(crate) fn replaceEventInfo(
    mut eInfoIn: BackendDAE::EventInfo,
    mut inVariableReplacements: VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> Result<BackendDAE::EventInfo> {
    type Func = std::sync::Arc<
        dyn ::std::ops::Fn(
                BackendDAE::ZeroCrossing,
                Option<FuncTypeExp_ExpToBoolean>,
            ) -> Result<BackendDAE::ZeroCrossing>
            + 'static,
    >;

    let mut eInfoOut: BackendDAE::EventInfo;
    let mut numberMathEvents: i32;
    let mut timeEvents: metamodelica::List<BackendDAE::TimeEvent>;
    let mut zeroCrossingLst: BackendDAE::ZeroCrossingSet;
    let mut sampleLst: BackendDAE::ZeroCrossingSet;
    let mut relationsLst: BackendDAE::ZeroCrossingSet;
    let mut zc: Arc<
        dyn ::std::ops::Fn(
                BackendDAE::ZeroCrossing,
                Option<FuncTypeExp_ExpToBoolean>,
            ) -> Result<BackendDAE::ZeroCrossing>
            + 'static,
    >;
    let BackendDAE::EVENT_INFO {
        timeEvents: __pa0,
        zeroCrossings: __pa1,
        relations: __pa2,
        samples: __pa3,
        numberMathEvents: __pa4,
    } = eInfoIn;
    timeEvents = metamodelica::Own::own(__pa0);
    zeroCrossingLst = metamodelica::Own::own(__pa1);
    relationsLst = metamodelica::Own::own(__pa2);
    sampleLst = metamodelica::Own::own(__pa3);
    numberMathEvents = metamodelica::Own::own(__pa4);
    timeEvents = List::map2(
        timeEvents,
        &move |__a0: BackendDAE::TimeEvent,
               __a1: VariableReplacements,
               __a2: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(replaceTimeEvents(__a0, &__a1, __a2)) },
        inVariableReplacements.clone(),
        inFuncTypeExpExpToBooleanOption.clone(),
    )?;
    zc = (std::sync::Arc::new({
        let __pe_b1 = inVariableReplacements;
        move |__pe_a0, __pe_a2| Ok(replaceZeroCrossing(__pe_a0, &__pe_b1, __pe_a2))
    })
        as std::sync::Arc<
            dyn ::std::ops::Fn(
                    BackendDAE::ZeroCrossing,
                    Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
                ) -> Result<BackendDAE::ZeroCrossing>
                + 'static,
        >);
    DoubleEnded::mapNoCopy_1(
        zeroCrossingLst.zc.clone(),
        &*(zc.clone()),
        inFuncTypeExpExpToBooleanOption.clone(),
    )?;
    DoubleEnded::mapNoCopy_1(
        sampleLst.zc.clone(),
        &*(zc.clone()),
        inFuncTypeExpExpToBooleanOption.clone(),
    )?;
    DoubleEnded::mapNoCopy_1(relationsLst.zc.clone(), &*(zc.clone()), inFuncTypeExpExpToBooleanOption)?;
    eInfoOut = BackendDAE::EventInfo {
        timeEvents: timeEvents,
        zeroCrossings: zeroCrossingLst,
        relations: relationsLst,
        samples: sampleLst,
        numberMathEvents: numberMathEvents,
    };
    Ok(eInfoOut)
}

fn replaceTimeEvents(
    mut teIn: BackendDAE::TimeEvent,
    mut inVariableReplacements: &VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> BackendDAE::TimeEvent {
    let mut teOut: BackendDAE::TimeEvent;
    teOut = (match teIn.clone() {
        BackendDAE::TimeEvent::SAMPLE_TIME_EVENT {
            index: mut index,
            startExp: mut startExp,
            intervalExp: mut intervalExp,
            ..
        } => {
            let mut startExp = startExp.clone();
            let mut intervalExp = intervalExp.clone();
            (startExp, _) = replaceExp(
                metamodelica::AsArg::as_arg(&startExp),
                inVariableReplacements,
                inFuncTypeExpExpToBooleanOption.clone(),
            );
            (intervalExp, _) = replaceExp(
                metamodelica::AsArg::as_arg(&intervalExp),
                inVariableReplacements,
                inFuncTypeExpExpToBooleanOption,
            );
            BackendDAE::TimeEvent::SAMPLE_TIME_EVENT {
                index: index.clone(),
                startExp: startExp.clone(),
                intervalExp: intervalExp.clone(),
                iter: var_field!(teIn.iter, BackendDAE::TimeEvent::SAMPLE_TIME_EVENT).clone(),
            }
        }
        _ => teIn,
    });
    teOut
}

fn replaceZeroCrossing(
    mut zcIn: BackendDAE::ZeroCrossing,
    mut inVariableReplacements: &VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> BackendDAE::ZeroCrossing {
    let mut zcOut: BackendDAE::ZeroCrossing;
    zcOut = (match zcIn.clone() {
        BackendDAE::ZeroCrossing {
            relation_: mut relation_,
            ..
        } => {
            let mut relation_ = relation_.clone();
            (relation_, _) = replaceExp(
                metamodelica::AsArg::as_arg(&relation_),
                inVariableReplacements,
                inFuncTypeExpExpToBooleanOption,
            );
            BackendDAE::ZeroCrossing {
                index: zcIn.index.clone(),
                relation_: relation_.clone(),
                occurEquLst: zcIn.occurEquLst.clone(),
                iter: zcIn.iter.clone(),
            }
        }
        _ => zcIn,
    });
    zcOut
}

/* ********************************************************/
/* dump replacements  */
/* ********************************************************/
pub(crate) fn dumpReplacements(mut repl: &VariableReplacements) -> Result<()> {
    let mut srcs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut dsts: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut tplLst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
    (srcs, dsts) = getAllReplacements(repl)?;
    tplLst = List::zip(srcs, dsts);
    metamodelica::print(literal!("\nReplacements: ("));
    metamodelica::print(ArcStr::from(::std::format!("{}", ((tplLst).len() as i32))));
    metamodelica::print(literal!(")\n"));
    metamodelica::print(literal!("========================================\n"));
    metamodelica::print(stringDelimitList(
        List::map(tplLst, &printReplacementTupleStr)?,
        literal!("\n"),
    ));
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn dumpExtendReplacements(mut repl: &VariableReplacements) -> Result<()> {
    let mut names: metamodelica::List<ArcStr>;
    names = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut c in (UnorderedSet::toList(repl.extendhashTable.clone()))
            .into_iter()
            .cloned()
        {
            let __x = ComponentReferenceBasics::printComponentRefStr(&(c.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    names = List::sort(
        names,
        (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Util::strcmpBool(&__a0, &__a1))
        }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
    )?;
    metamodelica::print(literal!("\nExtendReplacements: ("));
    metamodelica::print(ArcStr::from(::std::format!("{}", ((names).len() as i32))));
    metamodelica::print(literal!(")\n"));
    metamodelica::print(literal!("========================================\n"));
    metamodelica::print(stringDelimitList(names, literal!("\n")));
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn dumpDerConstReplacements(mut repl: &VariableReplacements) -> Result<()> {
    let mut srcs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut dsts: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut tplLst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
    if (repl.derConst).is_some() {
        (srcs, dsts) = getCrefExpTableEntries(Util::getOption(repl.derConst.clone())?)?;
        tplLst = List::zip(srcs, dsts);
        metamodelica::print(literal!("\nDerConstReplacements: ("));
        metamodelica::print(ArcStr::from(::std::format!("{}", ((tplLst).len() as i32))));
        metamodelica::print(literal!(")\n"));
        metamodelica::print(literal!("========================================\n"));
        metamodelica::print(stringDelimitList(
            List::map(tplLst, &printReplacementTupleStr)?,
            literal!("\n"),
        ));
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

fn printReplacementTupleStr(
    mut tpl: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>),
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
            &(Util::tuple21(tpl.clone())),
        )?);
        __mm_s.push_str(&*literal!(" -> "));
        __mm_s.push_str(&*ExpressionBasics::printExpStr(Util::tuple22(tpl))?);
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn getConstantReplacements(mut replIn: &VariableReplacements) -> Result<VariableReplacements> {
    let mut replOut: VariableReplacements;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    (crefs, exps) = getAllReplacements(replIn)?;
    (exps, crefs) = List::filterOnTrueSync(
        &exps,
        &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Expression::isEvaluatedConst(&__a0))
        },
        crefs,
    )?;
    replOut = emptyReplacements();
    replOut = addReplacements(replOut, &crefs, &exps, None)?;
    Ok(replOut)
}
