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
use openmodelica_frontend::CevalFunction;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::Inst;
use openmodelica_frontend::InteractiveTypes;
use openmodelica_frontend::Lookup;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::MetaUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::AvlTreeStringString;
use openmodelica_util::Error;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::List;

/// file:        SymbolTable.mo
///  package:     SymbolTable
///  description: Thread-local, mutable symbol table. Set this at the start
///               of any interactive call or in Main.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SymbolTable {
    /// ast ; The ast
    pub ast: Absyn::Program,
    /// the explodedAst is invalidated every time the program is updated
    pub explodedAst: Option<metamodelica::List<metamodelica::Ref<SCode::Element>>>,
    /// List of variables with values
    pub vars: metamodelica::List<InteractiveTypes::Variable>,
    pub cachedAsts: metamodelica::Ref<Vector::Vector<Absyn::Program>>,
    pub cacheIndex: i32,
    /// has inner/outer, expandable, overconstrained and stream connectors; a side
    ///     effect of translating explodedAst that the old frontend reads later. Stored
    ///     here so getSCode can re-assert it on a cache hit, since an intervening
    ///     translation or NFInst.resetGlobalFlags may have cleared the global flags.
    pub connectorFlags: (bool, bool, bool, bool),
}

impl metamodelica::gc::MMTrace for SymbolTable {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.ast, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.explodedAst, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.vars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.cachedAsts, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.cacheIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.connectorFlags, __mmv)?;
        Ok(())
    }
}
impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            ast: Default::default(),
            explodedAst: Default::default(),
            vars: Default::default(),
            cachedAsts: Default::default(),
            cacheIndex: Default::default(),
            connectorFlags: Default::default(),
        }
    }
}

pub type SYMBOLTABLE = SymbolTable;

pub(crate) const AST_CACHE_MAX_SIZE: i32 = 1000;

pub fn reset() -> Result<()> {
    pub(crate) type Program = Absyn::Program;

    {
        let __v = metamodelica::Ref::new(SymbolTable {
            ast: Absyn::Program {
                classes: metamodelica::nil(),
                within_: openmodelica_ast::Absyn::Within::TOP,
            },
            explodedAst: None,
            vars: metamodelica::nil(),
            cachedAsts: Vector::new(0),
            cacheIndex: 0,
            connectorFlags: (false, false, false, false),
        });
        crate::Globals::symbolTable.with(|__root| *__root.borrow_mut() = __v)
    };
    updateUriMapping(metamodelica::nil())?;
    Ok(())
}

pub(crate) fn currentConnectorFlags() -> (bool, bool, bool, bool) {
    let mut flags: (bool, bool, bool, bool);
    flags = (
        System::getHasInnerOuterDefinitions(),
        System::getHasExpandableConnectors(),
        System::getHasOverconstrainedConnectors(),
        System::getHasStreamConnectors(),
    );
    flags
}

pub(crate) fn applyConnectorFlags(mut flags: (bool, bool, bool, bool)) -> () {
    let mut io: bool;
    let mut ec: bool;
    let mut oc: bool;
    let mut sc: bool;
    (io, ec, oc, sc) = flags;
    System::setHasInnerOuterDefinitions(io);
    System::setHasExpandableConnectors(ec);
    System::setHasOverconstrainedConnectors(oc);
    System::setHasStreamConnectors(sc);
    ()
}

pub fn update(mut table: metamodelica::Ref<SymbolTable>) -> () {
    {
        let __v = table;
        crate::Globals::symbolTable.with(|__root| *__root.borrow_mut() = __v)
    };
    ()
}

pub fn get() -> metamodelica::Ref<SymbolTable> {
    let mut table: metamodelica::Ref<SymbolTable>;
    table = crate::Globals::symbolTable.with(|__root| __root.borrow().clone());
    table
}

pub fn getAbsyn() -> Absyn::Program {
    let mut ast: Absyn::Program;
    let mut table: metamodelica::Ref<SymbolTable>;
    table = get();
    ast = table.ast.clone();
    ast
}

pub fn setAbsyn(mut ast: Absyn::Program) -> Result<()> {
    let mut table: metamodelica::Ref<SymbolTable>;
    table = get();
    if {
        let __refeq_sl = &(table.ast.clone());
        let __refeq_sr = &(ast.clone());
        metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.classes), &(__refeq_sr.classes))
            && (match (&(__refeq_sl.within_), &(__refeq_sr.within_)) {
                (Absyn::Within::TOP, Absyn::Within::TOP) => true,
                (Absyn::Within::WITHIN { path: __refeq_v0l }, Absyn::Within::WITHIN { path: __refeq_v0r }) => {
                    referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                }
                _ => false,
            })
    } {
        return Ok(());
    }
    assign_field!(table.ast = ast.clone());
    updateUriMapping(ast.classes.clone())?;
    if (table.explodedAst).is_some() {
        assign_field!(table.explodedAst = None);
    }
    update(table);
    Ok(())
}

pub fn setAbsynElement(
    mut ast: Absyn::Program,
    mut element: &metamodelica::Ref<Absyn::Element>,
    mut path: &metamodelica::Ref<Absyn::Path>,
) -> Result<()> {
    fn update_element(
        mut oldElement: &metamodelica::Ref<SCode::Element>,
        mut newElement: metamodelica::Ref<SCode::Element>,
    ) -> metamodelica::Ref<SCode::Element> {
        let mut newElement: metamodelica::Ref<SCode::Element> = newElement;
        if SCodeUtil::isElementProtected(oldElement) {
            newElement = SCodeUtil::makeElementProtected(newElement);
        }
        newElement
    }

    let mut table: metamodelica::Ref<SymbolTable>;
    let mut scode_elem: metamodelica::Ref<SCode::Element>;
    let mut scode_elems: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut scode_prog: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    table = get();
    if {
        let __refeq_sl = &(table.ast.clone());
        let __refeq_sr = &(ast.clone());
        metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.classes), &(__refeq_sr.classes))
            && (match (&(__refeq_sl.within_), &(__refeq_sr.within_)) {
                (Absyn::Within::TOP, Absyn::Within::TOP) => true,
                (Absyn::Within::WITHIN { path: __refeq_v0l }, Absyn::Within::WITHIN { path: __refeq_v0r }) => {
                    referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                }
                _ => false,
            })
    } {
        return Ok(());
    }
    assign_field!(table.ast = ast.clone());
    updateUriMapping(ast.classes.clone())?;
    if (table.explodedAst).is_some() {
        applyConnectorFlags(table.connectorFlags.clone());
        scode_elems = AbsynToSCode::translateElement(element, openmodelica_frontend_types::SCode::Visibility::PUBLIC)?;
        if ((scode_elems).len() as i32) > 1 {
            let __pa0 = ::match_deref::match_deref! { match &(List::findOption(&scode_elems, &({ let __pe_b0 = AbsynUtil::pathLastIdent(path); move |__pe_a1| Ok(SCodeUtil::isElementNamed(&__pe_b0, &__pe_a1)) }))?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            scode_elem = metamodelica::Own::own(__pa0);
        } else {
            scode_elem = (scode_elems).head().cloned()?;
        }
        let __pa1 = ::match_deref::match_deref! { match &(table.explodedAst.clone()) {
            Some(__pa1) => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        scode_prog = metamodelica::Own::own(__pa1);
        let __pa2 = ::match_deref::match_deref! { match &(SCodeUtil::transformPathedElementInProgram(path, (std::sync::Arc::new({ let __pe_b1 = scode_elem; move |__pe_a0| Ok(update_element(&__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>> + 'static>), scode_prog)?) {
            (__pa2, true) => __pa2.clone(),
            _ => return Err("pattern mismatch"),
        } };
        scode_prog = metamodelica::Own::own(__pa2);
        assign_field!(
            table.explodedAst = Some(scode_prog),
            table.connectorFlags = currentConnectorFlags()
        );
    }
    update(table);
    Ok(())
}

pub fn setAbsynClass(
    mut ast: Absyn::Program,
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut path: &metamodelica::Ref<Absyn::Path>,
) -> Result<()> {
    fn update_element(
        mut oldElement: &metamodelica::Ref<SCode::Element>,
        mut newElement: metamodelica::Ref<SCode::Element>,
    ) -> Result<metamodelica::Ref<SCode::Element>> {
        let mut newElement: metamodelica::Ref<SCode::Element> = newElement;
        newElement = SCodeUtil::setElementPrefixes(SCodeUtil::elementPrefixes(oldElement)?, newElement)?;
        Ok(newElement)
    }

    let mut table: metamodelica::Ref<SymbolTable>;
    let mut scode_elem: metamodelica::Ref<SCode::Element>;
    let mut scode_prog: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    table = get();
    if {
        let __refeq_sl = &(table.ast.clone());
        let __refeq_sr = &(ast.clone());
        metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.classes), &(__refeq_sr.classes))
            && (match (&(__refeq_sl.within_), &(__refeq_sr.within_)) {
                (Absyn::Within::TOP, Absyn::Within::TOP) => true,
                (Absyn::Within::WITHIN { path: __refeq_v0l }, Absyn::Within::WITHIN { path: __refeq_v0r }) => {
                    referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                }
                _ => false,
            })
    } {
        return Ok(());
    }
    assign_field!(table.ast = ast.clone());
    updateUriMapping(ast.classes.clone())?;
    if (table.explodedAst).is_some() {
        applyConnectorFlags(table.connectorFlags.clone());
        scode_elem = AbsynToSCode::translateClass(cls)?;
        let __pa0 = ::match_deref::match_deref! { match &(table.explodedAst.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        scode_prog = metamodelica::Own::own(__pa0);
        let __pa1 = ::match_deref::match_deref! { match &(SCodeUtil::transformPathedElementInProgram(path, (std::sync::Arc::new({ let __pe_b1 = scode_elem; move |__pe_a0| update_element(&__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>> + 'static>), scode_prog)?) {
            (__pa1, true) => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        scode_prog = metamodelica::Own::own(__pa1);
        assign_field!(
            table.explodedAst = Some(scode_prog),
            table.connectorFlags = currentConnectorFlags()
        );
    }
    update(table);
    Ok(())
}

pub fn setAbsynLoaded(mut ast: Absyn::Program, mut loaded: Absyn::Program) -> Result<()> {
    fn update_element(
        mut oldElement: &metamodelica::Ref<SCode::Element>,
        mut newElement: metamodelica::Ref<SCode::Element>,
    ) -> Result<metamodelica::Ref<SCode::Element>> {
        let mut newElement: metamodelica::Ref<SCode::Element> = newElement;
        newElement = SCodeUtil::setElementPrefixes(SCodeUtil::elementPrefixes(oldElement)?, newElement)?;
        Ok(newElement)
    }

    let mut table: metamodelica::Ref<SymbolTable>;
    let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut newElems: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut se: metamodelica::Ref<SCode::Element>;
    let mut found: bool;
    let mut topLevel: bool;
    let mut metaLoaded: Absyn::Program;
    table = get();
    if {
        let __refeq_sl = &(table.ast.clone());
        let __refeq_sr = &(ast.clone());
        metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.classes), &(__refeq_sr.classes))
            && (match (&(__refeq_sl.within_), &(__refeq_sr.within_)) {
                (Absyn::Within::TOP, Absyn::Within::TOP) => true,
                (Absyn::Within::WITHIN { path: __refeq_v0l }, Absyn::Within::WITHIN { path: __refeq_v0r }) => {
                    referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                }
                _ => false,
            })
    } {
        return Ok(());
    }
    assign_field!(table.ast = ast.clone());
    updateUriMapping(ast.classes.clone())?;
    topLevel = (match loaded.within_.clone() {
        Absyn::Within::TOP { .. } => true,
        _ => false,
    });
    if topLevel && (table.explodedAst).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(table.explodedAst.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        sp = metamodelica::Own::own(__pa0);
        applyConnectorFlags(table.connectorFlags.clone());
        metaLoaded = MetaUtil::createMetaClassesInProgram(loaded)?;
        for mut cls in &*metaLoaded.classes.clone() {
            se = AbsynToSCode::translateClass(cls.clone())?;
            (sp, found) = SCodeUtil::transformPathedElementInProgram(
                &(metamodelica::Ref::new(Absyn::Path::IDENT { name: cls.name.clone() })),
                (std::sync::Arc::new({
                    let __pe_b1 = se.clone();
                    move |__pe_a0| update_element(&__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<SCode::Element>,
                            ) -> Result<metamodelica::Ref<SCode::Element>>
                            + 'static,
                    >),
                sp,
            )?;
            if !(found) {
                newElems = metamodelica::cons(se, newElems);
            }
        }
        newElems = newElems.reverse();
        assign_field!(table.connectorFlags = currentConnectorFlags());
        metaLoaded = MetaUtil::createMetaClassesInProgram(table.ast.clone())?;
        assign_field!(
            table.explodedAst =
                if (((sp).len() as i32) + ((newElems).len() as i32) == ((metaLoaded.classes).len() as i32)) {
                    Some(listAppend(sp, newElems))
                } else {
                    None
                }
        );
    } else if (table.explodedAst).is_some() {
        assign_field!(table.explodedAst = None);
    }
    update(table);
    Ok(())
}

pub fn setAbsynDeleted(mut ast: Absyn::Program, mut path: &metamodelica::Ref<Absyn::Path>) -> Result<()> {
    let mut table: metamodelica::Ref<SymbolTable>;
    let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut name: ArcStr;
    let mut topLevel: bool;
    table = get();
    assign_field!(table.ast = ast.clone());
    updateUriMapping(ast.classes.clone())?;
    (topLevel, name) = (match &**path {
        Absyn::Path::IDENT { name: __esc_name } => {
            name = (*__esc_name).clone();
            (true, name.clone())
        }
        _ => (false, literal!("")),
    });
    if (table.explodedAst).is_some() {
        if topLevel {
            let __pa0 = ::match_deref::match_deref! { match &(table.explodedAst.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            sp = metamodelica::Own::own(__pa0);
            assign_field!(
                table.explodedAst = Some(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                        for mut e in (sp).into_iter().cloned() {
                            if !(!(SCodeUtil::isElementNamed(&name, &(e.clone())))) {
                                continue;
                            }
                            let __x = e.clone();
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                )
            );
        } else {
            assign_field!(table.explodedAst = None);
        }
    }
    update(table);
    Ok(())
}

pub fn getSCode() -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut ast: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut table: metamodelica::Ref<SymbolTable>;
    table = get();
    if (table.explodedAst).is_none() {
        ast = AbsynToSCode::translateAbsyn2SCode(table.ast.clone())?;
        assign_field!(
            table.explodedAst = Some(ast.clone()),
            table.connectorFlags = currentConnectorFlags()
        );
        update(table);
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(table.explodedAst.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        ast = metamodelica::Own::own(__pa0);
        applyConnectorFlags(table.connectorFlags.clone());
    }
    Ok(ast)
}

pub fn setSCode(mut ast: Option<metamodelica::List<metamodelica::Ref<SCode::Element>>>) -> () {
    let mut table: metamodelica::Ref<SymbolTable>;
    table = get();
    if (match (&(table.explodedAst), &(ast)) {
        (None, None) => true,
        (Some(__refeq_l), Some(__refeq_r)) => metamodelica::ReferenceEq::reference_eq(&(*__refeq_l), &(*__refeq_r)),
        _ => false,
    }) {
        return ();
    }
    assign_field!(table.explodedAst = ast);
    update(table);
    ()
}

pub fn clearSCode() -> () {
    let mut table: metamodelica::Ref<SymbolTable>;
    table = get();
    if (table.explodedAst).is_some() {
        assign_field!(table.explodedAst = None);
        update(table);
    }
    ()
}

pub fn clearProgram() -> Result<()> {
    let mut table: metamodelica::Ref<SymbolTable>;
    table = get();
    reset()?;
    setVars(table.vars.clone());
    Ok(())
}

pub fn getVars() -> metamodelica::List<InteractiveTypes::Variable> {
    let mut vars: metamodelica::List<InteractiveTypes::Variable>;
    let mut table: metamodelica::Ref<SymbolTable>;
    table = get();
    vars = table.vars.clone();
    vars
}

pub fn setVars(mut vars: metamodelica::List<InteractiveTypes::Variable>) -> () {
    let mut table: metamodelica::Ref<SymbolTable>;
    table = get();
    assign_field!(table.vars = vars);
    update(table);
    ()
}

pub fn addVars(
    mut inCref: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inEnv: &FCore::Graph,
) -> Result<()> {
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut v: metamodelica::Ref<Values::Value>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    crefs = inCref;
    vals = inValues;
    while !((crefs).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crefs) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cr = metamodelica::Own::own(__pa0);
        crefs = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(vals) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        v = metamodelica::Own::own(__pa2);
        vals = metamodelica::Own::own(__pa3);
        addVar(&cr, v, inEnv)?;
    }
    Ok(())
}

pub fn addVar(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inEnv: &FCore::Graph,
) -> Result<()> {
    let mut vars: metamodelica::List<InteractiveTypes::Variable>;
    let mut table: metamodelica::Ref<SymbolTable>;
    table = get();
    vars = addVarToVarList(inCref, inValue.clone(), inEnv, table.vars.clone())?;
    assign_field!(table.vars = addVarToVarList(inCref, inValue, inEnv, vars)?);
    update(table);
    Ok(())
}

pub fn appendVar(
    mut inIdent: ArcStr,
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> () {
    let mut table: metamodelica::Ref<SymbolTable>;
    table = get();
    assign_field!(
        table.vars = metamodelica::cons(
            InteractiveTypes::Variable {
                varIdent: inIdent,
                value: inValue,
                type_: inType
            },
            table.vars.clone()
        )
    );
    update(table);
    ()
}

pub fn deleteVarFirstEntry(mut inIdent: ArcStr) -> Result<()> {
    let mut table: metamodelica::Ref<SymbolTable>;
    table = get();
    assign_field!(
        table.vars = List::deleteMemberOnTrue(
            inIdent,
            table.vars.clone(),
            &move |__a0: ArcStr, __a1: InteractiveTypes::Variable| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isVarNamed(&__a0, &__a1))
            }
        )?
        .0
    );
    update(table);
    Ok(())
}

pub fn storeAST() -> Result<i32> {
    let mut id: i32;
    let mut table: metamodelica::Ref<SymbolTable>;
    table = get();
    id = table.cacheIndex.clone() + 1;
    if id < 0 {
        id = 1;
    }
    assign_field!(table.cacheIndex = id);
    update(table.clone());
    if Vector::size(table.cachedAsts.clone()) >= AST_CACHE_MAX_SIZE.clone() {
        Vector::update(
            table.cachedAsts.clone(),
            intMod(id - 1, AST_CACHE_MAX_SIZE.clone()) + 1,
            getAbsyn(),
        )?;
    } else {
        Vector::push(table.cachedAsts.clone(), getAbsyn());
    }
    Ok(id)
}

pub fn restoreAST(mut id: i32) -> Result<bool> {
    let mut success: bool;
    let mut table: metamodelica::Ref<SymbolTable>;
    table = get();
    success = id <= table.cacheIndex.clone() && id > table.cacheIndex.clone() - AST_CACHE_MAX_SIZE.clone() && id > 0;
    if success {
        setAbsyn(Vector::get(
            table.cachedAsts.clone(),
            intMod(id - 1, AST_CACHE_MAX_SIZE.clone()) + 1,
        )?)?;
    }
    Ok(success)
}

fn isVarNamed(mut id: &ArcStr, mut v: &InteractiveTypes::Variable) -> bool {
    let mut b: bool;
    b = metamodelica::stringEq(&v.varIdent, &id);
    b
}

fn addVarToVarList(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inEnv: &FCore::Graph,
    mut inVariables: metamodelica::List<InteractiveTypes::Variable>,
) -> Result<metamodelica::List<InteractiveTypes::Variable>> {
    let mut outVariables: metamodelica::List<InteractiveTypes::Variable>;
    let mut found: bool;
    (outVariables, found) = List::findMap(
        inVariables,
        &({
            let __pe_b1 = inCref.clone();
            let __pe_b2 = inValue.clone();
            let __pe_b3 = inEnv.clone();
            move |__pe_a0| addVarToVarList2(__pe_a0, &__pe_b1, __pe_b2.clone(), &__pe_b3)
        }),
    )?;
    outVariables = addVarToVarList4(found, inCref, inValue, outVariables)?;
    Ok(outVariables)
}

fn addVarToVarList2(
    mut inOldVariable: InteractiveTypes::Variable,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inEnv: &FCore::Graph,
) -> Result<(InteractiveTypes::Variable, bool)> {
    let mut outVariable: InteractiveTypes::Variable;
    let mut outFound: bool;
    let mut id1: ArcStr;
    let mut id2: ArcStr;
    let InteractiveTypes::IVAR { varIdent: __pa0, .. } = &inOldVariable;
    id1 = metamodelica::Own::own(__pa0);
    let __pa1 = ::match_deref::match_deref! { match &((*inCref)) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: __pa1, .. } => __pa1.clone(),
        _ => return Err("pattern mismatch"),
    } };
    id2 = metamodelica::Own::own(__pa1);
    outFound = stringEq(&id1, &id2);
    outVariable = addVarToVarList3(outFound, inOldVariable, inCref, inValue, inEnv)?;
    Ok((outVariable, outFound))
}

fn addVarToVarList3(
    mut inFound: bool,
    mut inOldVariable: InteractiveTypes::Variable,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inEnv: &FCore::Graph,
) -> Result<InteractiveTypes::Variable> {
    let mut outVariable: InteractiveTypes::Variable;
    outVariable = (::match_deref::match_deref! { match &((inFound, inOldVariable.clone(), inCref.clone())) {
        (false, _, _) => {
            inOldVariable
        },
        (true, _, Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty, subscriptLst: Deref @ metamodelica::ListNode::Nil }) => {
            InteractiveTypes::Variable { varIdent: id.clone(), value: inValue, type_: ty.clone() }
        },
        (true, InteractiveTypes::Variable { varIdent: id, value: val, type_: ty }, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, .. }) => {
            let mut val = (*val).clone();
            (_, val) = CevalFunction::assignVector(&inValue, metamodelica::AsArg::as_arg(&val), metamodelica::AsArg::as_arg(&subs), &(FCore::emptyCache()), inEnv)?;
            InteractiveTypes::Variable { varIdent: id.clone(), value: val.clone(), type_: ty.clone() }
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outVariable)
}

fn addVarToVarList4(
    mut inFound: bool,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inVariables: metamodelica::List<InteractiveTypes::Variable>,
) -> Result<metamodelica::List<InteractiveTypes::Variable>> {
    let mut outVariables: metamodelica::List<InteractiveTypes::Variable>;
    outVariables = (::match_deref::match_deref! { match &((inFound, &**inCref)) {
        (true, _) => {
            inVariables
        },
        (false, Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty, subscriptLst: Deref @ metamodelica::ListNode::Nil }) => {
            metamodelica::cons(InteractiveTypes::Variable { varIdent: id.clone(), value: inValue, type_: ty.clone() }, inVariables)
        },
        (false, Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, subscriptLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }) => {
            Error::addMessage(Error::SLICE_ASSIGN_NON_ARRAY.clone(), list![id.clone()])?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outVariables)
}

pub fn buildEnv() -> Result<FCore::Graph> {
    let mut env: FCore::Graph;
    let mut table: metamodelica::Ref<SymbolTable>;
    table = get();
    (_, env) = Inst::makeEnvFromProgram(&(getSCode()?))?;
    env = addVarsToEnv(&(table.vars.clone().reverse()), env)?;
    Ok(env)
}

fn addVarsToEnv(
    mut inVariableLst: &metamodelica::List<InteractiveTypes::Variable>,
    mut inEnv: FCore::Graph,
) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    outEnv = List::fold(
        inVariableLst,
        &move |__a0: InteractiveTypes::Variable, __a1: FCore::Graph| addVarToEnv(&__a0, __a1),
        inEnv,
    )?;
    Ok(outEnv)
}

fn addVarToEnv(mut inVariable: &InteractiveTypes::Variable, mut inEnv: FCore::Graph) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    outEnv = 'mc: {
        let __mc_input = (inVariable.clone(), inEnv);
        if let Ok(__v) = (|| -> Result<_> {
            let (
                InteractiveTypes::Variable {
                    varIdent: mut id,
                    value: ref v,
                    type_: ref tp,
                },
                mut env,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut empty_env: FCore::Graph;
            let mut cref: metamodelica::Ref<DAE::ComponentRef>;
            cref = ComponentReferenceBasics::makeCrefIdent(
                id.clone(),
                DAE::T_UNKNOWN_DEFAULT().clone(),
                metamodelica::nil(),
            );
            empty_env = FGraph::empty();
            Lookup::lookupVar(FCore::emptyCache(), env.clone(), cref.clone())?;
            env = FGraph::updateComp(
                env.clone(),
                metamodelica::Ref::new(DAE::Var {
                    name: id.clone(),
                    attributes: DAE::dummyAttrVar().clone(),
                    ty: tp.clone(),
                    binding: metamodelica::Ref::new(DAE::Binding::VALBOUND {
                        valBound: v.clone(),
                        source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE,
                    }),
                    bind_from_outside: false,
                    constOfForIteratorRange: None,
                }),
                &(openmodelica_frontend_dump::FCore::Status::VAR_TYPED),
                &empty_env,
            );
            Ok(env.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                InteractiveTypes::Variable {
                    varIdent: mut id,
                    value: ref v,
                    type_: ref tp,
                },
                mut env,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut empty_env: FCore::Graph;
            empty_env = FGraph::empty();
            env = FGraph::mkComponentNode(
                env.clone(),
                metamodelica::Ref::new(DAE::Var {
                    name: id.clone(),
                    attributes: DAE::dummyAttrVar().clone(),
                    ty: tp.clone(),
                    binding: metamodelica::Ref::new(DAE::Binding::VALBOUND {
                        valBound: v.clone(),
                        source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE,
                    }),
                    bind_from_outside: false,
                    constOfForIteratorRange: None,
                }),
                metamodelica::Ref::new(SCode::Element::COMPONENT {
                    name: id.clone(),
                    prefixes: SCode::defaultPrefixes.clone(),
                    attributes: SCode::Attributes {
                        arrayDims: metamodelica::nil(),
                        connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
                        parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
                        variability: openmodelica_frontend_types::SCode::Variability::VAR,
                        direction: openmodelica_ast::Absyn::Direction::BIDIR,
                        isField: openmodelica_ast::Absyn::IsField::NONFIELD,
                    },
                    typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                        path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
                        arrayDim: None,
                    }),
                    modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                    comment: SCode::noComment.clone(),
                    condition: None,
                    info: Absyn::dummyInfo.clone(),
                }),
                openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
                openmodelica_frontend_dump::FCore::Status::VAR_UNTYPED,
                empty_env.clone(),
            )?;
            Ok(env.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outEnv)
}

fn updateUriMapping(mut classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>) -> Result<()> {
    let mut tree: metamodelica::Ref<AvlTreeStringString::Tree>;
    let mut name: ArcStr;
    let mut fileName: ArcStr;
    let mut dir: ArcStr;
    let mut b: bool;
    let mut namesAndDirs: metamodelica::Array<ArcStr>;
    let mut infos: metamodelica::List<SourceInfo>;
    tree = openmodelica_util::AvlTreeStringString::Tree::interned_EMPTY();
    for mut cl in &*classes {
        let () = (::match_deref::match_deref! { match &(cl.clone()) {
            Deref @ Absyn::Class { info: SourceInfo { fileName: Deref @ "<interactive>", .. }, .. } => (),
            Deref @ Absyn::Class { name: __esc_name, info: SourceInfo { fileName: __esc_fileName, .. }, .. } => {
                name = (*__esc_name).clone();
                fileName = (*__esc_fileName).clone();
                dir = System::dirname(fileName.clone());
                fileName = System::basename(fileName.clone());
                b = stringEq(&fileName, &(literal!("ModelicaBuiltin.mo"))) || stringEq(&fileName, &(literal!("MetaModelicaBuiltin.mo"))) || stringEq(&dir, &(literal!(".")));
                if !(b) {
                    if AvlTreeStringString::hasKey(tree.clone(), name.clone())? {
                        infos = ({
            let mut __acc: metamodelica::List<SourceInfo> = metamodelica::nil();
            for mut cl in (classes.clone()).into_iter().cloned() {
                let __x = cl.info.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                        Error::addMultiSourceMessage(&(Error::DOUBLE_DECLARATION_OF_ELEMENTS.clone()), &(list![name.clone()]), &infos)?;
                    }
                    tree = AvlTreeStringString::add(tree, metamodelica::AsArg::as_arg(&name), &dir, &*((std::sync::Arc::new(AvlTreeStringString::addConflictDefault) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>)))?;
                }
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    namesAndDirs = metamodelica::arrayFromVec(
        List::thread(
            &(AvlTreeStringString::listValues(&tree, metamodelica::nil())),
            AvlTreeStringString::listKeys(&tree, metamodelica::nil()),
            &(metamodelica::nil()),
        )?
        .into_iter()
        .cloned()
        .collect(),
    );
    System::updateUriMapping(namesAndDirs.clone());
    Ok(())
}
