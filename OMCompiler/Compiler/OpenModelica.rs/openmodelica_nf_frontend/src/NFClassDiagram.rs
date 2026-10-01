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

use crate::NFUsedElements;
use crate::NFUsedElements::Definition;
use crate::NFUsedElements::Use;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util_datatypes_basic::Pointer;

pub(crate) const GENERALIZATION: i32 = 1;

pub(crate) const COMPOSITION: i32 = 2;

pub(crate) const DEPENDENCY: i32 = 3;

pub(crate) const NESTING: i32 = 4;

pub(crate) const INHERITED: i32 = 0;

pub(crate) const USED: i32 = 1;

pub(crate) const DRAWN: i32 = -1;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Node {
    /// The full name of the class.
    pub name: ArcStr,
    /// The name of the class in the diagram text.
    pub id: ArcStr,
    pub element: metamodelica::Ref<SCode::Element>,
    /// Levels of uses from the class the diagram is of.
    pub level: i32,
}

impl metamodelica::gc::MMTrace for Node {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.id, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.element, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.level, __mmv)?;
        Ok(())
    }
}
impl Default for Node {
    fn default() -> Self {
        Self {
            name: Default::default(),
            id: Default::default(),
            element: Default::default(),
            level: Default::default(),
        }
    }
}

pub type NODE = Node;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Relation {
    pub kind: i32,
    /// The full name of the class it points to.
    pub target: ArcStr,
    pub label: ArcStr,
    pub multiplicity: ArcStr,
    /// INHERITED, USED or DRAWN.
    pub step: i32,
    /// The short class definition it's declared by, if any.
    pub declaredBy: ArcStr,
}

impl metamodelica::gc::MMTrace for Relation {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.kind, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.target, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.label, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.multiplicity, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.step, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.declaredBy, __mmv)?;
        Ok(())
    }
}
impl Default for Relation {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            target: Default::default(),
            label: Default::default(),
            multiplicity: Default::default(),
            step: Default::default(),
            declaredBy: Default::default(),
        }
    }
}

pub type RELATION = Relation;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Diagram {
    /// By full name.
    pub nodes: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Node>>,
    /// The full names of the nodes, in reverse order.
    pub order: Pointer::Pointer<metamodelica::List<ArcStr>>,
    pub ids: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>,
    /// The full name a name refers to, by scope and name.
    pub resolved: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
    pub program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    pub depth: i32,
    pub exclude: metamodelica::List<ArcStr>,
    pub showModifiers: bool,
}

impl metamodelica::gc::MMTrace for Diagram {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.nodes, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.order, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ids, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.resolved, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.program, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.depth, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.exclude, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.showModifiers, __mmv)?;
        Ok(())
    }
}
impl Default for Diagram {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            order: Default::default(),
            ids: Default::default(),
            resolved: Default::default(),
            program: Default::default(),
            depth: Default::default(),
            exclude: Default::default(),
            showModifiers: Default::default(),
        }
    }
}

pub type DIAGRAM = Diagram;

pub fn generate(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut annotationProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut format: &ArcStr,
    mut depth: i32,
    mut exclude: metamodelica::List<ArcStr>,
    mut showModifiers: bool,
) -> Result<ArcStr> {
    let mut diagram: ArcStr = literal!("");
    let mut d: Diagram;
    let mut pending: metamodelica::List<ArcStr>;
    let mut batch: metamodelica::List<ArcStr>;
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut defs: metamodelica::List<Definition>;
    let mut uses: metamodelica::List<Use>;
    let mut name: ArcStr;
    d = Diagram {
        nodes: UnorderedMap::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            1,
        ),
        order: Pointer::create(metamodelica::nil()),
        ids: UnorderedSet::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            13,
        ),
        resolved: UnorderedMap::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            1,
        ),
        program: program.clone(),
        depth: depth,
        exclude: exclude,
        showModifiers: showModifiers,
    };
    name = AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(className), literal!("."), true, false)?;
    pending = addNode(name.clone(), 0, true, &d, metamodelica::nil())?;
    if (pending).is_empty() {
        return Ok(diagram);
    }
    while !((pending).is_empty()) {
        batch = pending.reverse();
        pending = metamodelica::nil();
        paths = metamodelica::nil();
        for mut n in &*batch {
            paths = walkedPaths(&(UnorderedMap::getOrFail(n.clone(), d.nodes.clone())?), paths)?;
        }
        (defs, uses, _) = NFUsedElements::collectUses(&(paths.reverse()), program.clone(), annotationProgram.clone())?;
        addResolved(&defs, &uses, d.resolved.clone())?;
        for mut n in &*batch {
            pending = expand(
                &(UnorderedMap::getOrFail(n.clone(), d.nodes.clone())?),
                &(d.clone()),
                pending,
            )?;
        }
    }
    diagram = if (metamodelica::stringEq(&format, &(literal!("drawio")))) {
        drawio(name, &d)?
    } else if (metamodelica::stringEq(&format, &(literal!("mermaid")))) {
        mermaid(&d)?
    } else {
        plantuml(&d)?
    };
    Ok(diagram)
}

fn addNode(
    mut name: ArcStr,
    mut level: i32,
    mut isRoot: bool,
    mut d: &Diagram,
    mut pending: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut pending: metamodelica::List<ArcStr> = pending;
    let mut cls: metamodelica::Ref<SCode::Element>;
    let mut node: Node;
    if !(isRoot) && (level > d.depth.clone() || isExcluded(name.clone(), &d.exclude)?) {
        return Ok(pending);
    }
    let () = (match UnorderedMap::get(name.clone(), d.nodes.clone())? {
        Some(mut __esc_node) => {
            node = __esc_node.clone();
            if level < node.level.clone() {
                node.level = level;
                UnorderedMap::add(name.clone(), node, d.nodes.clone())?;
                if !(listMember(name.clone(), pending.clone())) {
                    pending = metamodelica::cons(name, pending);
                }
            }
            ()
        }
        _ => {
            let () = (::match_deref::match_deref! { match &(findClass(name.clone(), d.program.clone())?) {
                Some(cls) if (isRoot || isDiagramClass(metamodelica::AsArg::as_arg(&cls))) => {
                    UnorderedMap::add(name.clone(), Node { name: name.clone(), id: newId(name.clone(), d.ids.clone())?, element: cls.clone(), level: level }, d.nodes.clone())?;
                    Pointer::update(d.order.clone(), metamodelica::cons(name.clone(), Pointer::access(d.order.clone())));
                    pending = metamodelica::cons(name, pending);
                    ()
                },
                _ => (),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            ()
        }
    });
    Ok(pending)
}

fn isExcluded(mut name: ArcStr, mut exclude: &metamodelica::List<ArcStr>) -> Result<bool> {
    let mut res: bool = false;
    for mut e in &**exclude {
        if metamodelica::stringEq(&name, &e)
            || ((name).len() as i32) > ((e).len() as i32)
                && substring(name.clone(), 1, ((e).len() as i32) + 1)? == {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*e);
                    __mm_s.push_str(&*literal!("."));
                    ArcStr::from(__mm_s)
                }
        {
            res = true;
            return Ok(res);
        }
    }
    Ok(res)
}

fn isDiagramClass(mut cls: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match cls {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::ENUMERATION { .. }, .. } => false,
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_TYPE { .. }, .. } => false,
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION { .. }, .. } => false,
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_INTEGER { .. }, .. } => false,
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_REAL { .. }, .. } => false,
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_STRING { .. }, .. } => false,
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_BOOLEAN { .. }, .. } => false,
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_CLOCK { .. }, .. } => false,
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PREDEFINED_ENUMERATION { .. }, .. } => false,
        Deref @ SCode::Element::CLASS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn newId(mut name: ArcStr, mut ids: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>) -> Result<ArcStr> {
    let mut id: ArcStr;
    let mut base: ArcStr;
    let mut i: i32 = 1;
    base = stringAppendList(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut c in (stringListStringChar(name)).into_iter().cloned() {
                let __x = if (isIdChar(c.clone())?) {
                    c.clone()
                } else {
                    literal!("_")
                };
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    );
    id = base.clone();
    while UnorderedSet::contains(id.clone(), ids.clone())? {
        i = i + 1;
        id = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*base);
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*intString(i));
            ArcStr::from(__mm_s)
        };
    }
    UnorderedSet::add(id.clone(), ids)?;
    Ok(id)
}

fn isIdChar(mut c: ArcStr) -> Result<bool> {
    let mut res: bool;
    let mut i: i32 = stringCharInt(c.clone())?;
    res = i >= 48 && i <= 57 || i >= 65 && i <= 90 || i >= 97 && i <= 122 || i == 95;
    Ok(res)
}

fn findClass(
    mut name: ArcStr,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<Option<metamodelica::Ref<SCode::Element>>> {
    let mut cls: Option<metamodelica::Ref<SCode::Element>> = None;
    let mut elements: metamodelica::List<metamodelica::Ref<SCode::Element>> = program;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut id: ArcStr;
    let mut found: bool;
    path = AbsynUtil::stringPath(name)?;
    loop {
        id = AbsynUtil::pathFirstIdent(&path);
        found = false;
        for mut e in &*elements.clone() {
            if SCodeUtil::elementIsClass(metamodelica::AsArg::as_arg(&e))
                && metamodelica::stringEq(&(SCodeUtil::elementName(metamodelica::AsArg::as_arg(&e))?), &id)
            {
                cls = Some(e.clone());
                elements = SCodeUtil::getClassElements(metamodelica::AsArg::as_arg(&e));
                found = true;
                break;
            }
        }
        if !(found) {
            cls = None;
            return Ok(cls);
        } else if !(AbsynUtil::pathIsQual(&path)) {
            return Ok(cls);
        }
        path = AbsynUtil::pathRest(path)?;
    }
    Ok(cls)
}

fn walkedPaths(
    mut node: &Node,
    mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>> = paths;
    let mut path: metamodelica::Ref<Absyn::Path> = AbsynUtil::stringPath(node.name.clone())?;
    paths = metamodelica::cons(path.clone(), paths);
    for mut e in &*SCodeUtil::getClassElements(&node.element) {
        if SCodeUtil::elementIsClass(metamodelica::AsArg::as_arg(&e))
            && SCodeUtil::isDerivedClass(metamodelica::AsArg::as_arg(&e))
        {
            paths = metamodelica::cons(
                AbsynUtil::suffixPath(&path, &(SCodeUtil::elementName(metamodelica::AsArg::as_arg(&e))?)),
                paths,
            );
        }
    }
    Ok(paths)
}

fn addResolved(
    mut defs: &metamodelica::List<Definition>,
    mut uses: &metamodelica::List<Use>,
    mut resolved: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<()> {
    let mut names: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>> = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    let mut role: ArcStr;
    for mut d in &**defs {
        UnorderedMap::add(d.key.clone(), d.name.clone(), names.clone())?;
    }
    for mut u in &**uses {
        role = u.site.role.clone();
        if !(u.candidate.clone())
            && !metamodelica::stringEq(&role, &(literal!("modifier")))
            && !metamodelica::stringEq(&role, &(literal!("end")))
            && u.index.clone() == pathLength(&u.written)
        {
            UnorderedMap::add(
                resolvedKey(
                    u.site.scope.clone(),
                    AbsynUtil::pathString(
                        AbsynUtil::makeNotFullyQualified(u.written.clone()),
                        literal!("."),
                        true,
                        false,
                    )?,
                    metamodelica::stringEq(&role, &(literal!("classExtends")))
                        || metamodelica::stringEq(&role, &(literal!("redeclare"))),
                ),
                UnorderedMap::getOrFail(u.key.clone(), names.clone())?,
                resolved.clone(),
            )?;
        }
    }
    Ok(())
}

fn pathLength<'__b>(mut path: &'__b metamodelica::Ref<Absyn::Path>) -> i32 {
    '__tco: loop {
        match &**path {
            Absyn::Path::QUALIFIED { .. } => return 1 + pathLength(var_field!((**path).path, Absyn::Path::QUALIFIED)),
            Absyn::Path::FULLYQUALIFIED { .. } => {
                path = var_field!((**path).path, Absyn::Path::FULLYQUALIFIED);
                continue '__tco;
            }
            _ => return 1,
        }
    }
}

fn resolvedKey(mut scope: ArcStr, mut name: ArcStr, mut classExtends: bool) -> ArcStr {
    let mut key: ArcStr = stringAppendList(list![
        scope.clone(),
        if (classExtends) {
            literal!(" extends ")
        } else {
            literal!(" ")
        },
        name.clone()
    ]);
    key
}

fn resolve(
    mut scope: ArcStr,
    mut name: metamodelica::Ref<Absyn::Path>,
    mut d: &Diagram,
    mut classExtends: bool,
) -> Result<ArcStr> {
    let mut fullName: ArcStr;
    fullName = UnorderedMap::getOrDefault(
        resolvedKey(
            scope,
            AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(name), literal!("."), true, false)?,
            classExtends,
        ),
        d.resolved.clone(),
        literal!(""),
    )?;
    Ok(fullName)
}

fn expand(
    mut node: &Node,
    mut d: &Diagram,
    mut pending: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut pending: metamodelica::List<ArcStr> = pending;
    for mut r in &*relations(node, d)? {
        if r.step.clone() != DRAWN.clone() && !(stringEmpty(&r.target)) {
            pending = addNode(r.target.clone(), node.level.clone() + r.step.clone(), false, d, pending)?;
        }
    }
    Ok(pending)
}

fn relations(mut node: &Node, mut d: &Diagram) -> Result<metamodelica::List<Relation>> {
    let mut rels: metamodelica::List<Relation> = metamodelica::nil();
    let mut scope: ArcStr = node.name.clone();
    let mut target: ArcStr;
    let mut name: ArcStr;
    let mut cls: metamodelica::Ref<SCode::Element> = node.element.clone();
    let mut ts: metamodelica::Ref<Absyn::TypeSpec>;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    rels = (::match_deref::match_deref! { match &(&*cls) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: __esc_ts, modifications: __esc_mod, .. }, .. } => {
            ts = (*__esc_ts).clone();
            r#mod = (*__esc_mod).clone();
            target = resolve(scope.clone(), AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&ts)), d, false)?;
            rels = metamodelica::cons(Relation { kind: GENERALIZATION.clone(), target: target, label: modString(r#mod.clone(), d)?, multiplicity: literal!(""), step: INHERITED.clone(), declaredBy: literal!("") }, rels);
            redeclares(metamodelica::AsArg::as_arg(&r#mod), &(literal!("")), &scope, d, rels)?
        },
        Deref @ SCode::Element::CLASS { name: __esc_name, classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: __esc_mod, .. }, .. } => {
            name = (*__esc_name).clone();
            r#mod = (*__esc_mod).clone();
            target = resolve(scope.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), d, true)?;
            rels = metamodelica::cons(Relation { kind: GENERALIZATION.clone(), target: target, label: modString(r#mod.clone(), d)?, multiplicity: literal!(""), step: INHERITED.clone(), declaredBy: literal!("") }, rels);
            redeclares(metamodelica::AsArg::as_arg(&r#mod), &(literal!("")), &scope, d, rels)?
        },
        _ => rels,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if SCodeUtil::isElementRedeclare(&cls)? && !(SCodeUtil::isClassExtends(&cls)) {
        target = resolve(
            scope.clone(),
            metamodelica::Ref::new(Absyn::Path::IDENT {
                name: SCodeUtil::elementName(&cls)?,
            }),
            d,
            true,
        )?;
        rels = metamodelica::cons(
            Relation {
                kind: DEPENDENCY.clone(),
                target: target,
                label: literal!("redeclares"),
                multiplicity: literal!(""),
                step: INHERITED.clone(),
                declaredBy: literal!(""),
            },
            rels,
        );
    }
    for mut e in &*SCodeUtil::getClassElements(&cls) {
        rels = elementRelations(metamodelica::AsArg::as_arg(&e), scope.clone(), d, rels)?;
    }
    rels = rels.reverse();
    Ok(rels)
}

fn elementRelations(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut scope: ArcStr,
    mut d: &Diagram,
    mut rels: metamodelica::List<Relation>,
) -> Result<metamodelica::List<Relation>> {
    let mut rels: metamodelica::List<Relation> = rels;
    let mut target: ArcStr;
    let mut name: ArcStr;
    let mut full_name: ArcStr;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut ts: metamodelica::Ref<Absyn::TypeSpec>;
    let mut dims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let mut prefixes: metamodelica::Ref<SCode::Prefixes>;
    let () = (::match_deref::match_deref! { match element {
        Deref @ SCode::Element::EXTENDS { baseClassPath: __esc_path, modifications: __esc_mod, .. } => {
            path = (*__esc_path).clone();
            r#mod = (*__esc_mod).clone();
            target = resolve(scope.clone(), path.clone(), d, false)?;
            rels = metamodelica::cons(Relation { kind: GENERALIZATION.clone(), target: target, label: modString(r#mod.clone(), d)?, multiplicity: literal!(""), step: INHERITED.clone(), declaredBy: literal!("") }, rels);
            rels = redeclares(metamodelica::AsArg::as_arg(&r#mod), &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("extends ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) }), &scope, d, rels)?;
            ()
        },
        Deref @ SCode::Element::COMPONENT { name: __esc_name, prefixes: __esc_prefixes, typeSpec: __esc_ts, modifications: __esc_mod, attributes: SCode::Attributes { arrayDims: __esc_dims, .. }, .. } => {
            name = (*__esc_name).clone();
            prefixes = (*__esc_prefixes).clone();
            ts = (*__esc_ts).clone();
            r#mod = (*__esc_mod).clone();
            dims = (*__esc_dims).clone();
            target = resolve(scope.clone(), AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&ts)), d, false)?;
            rels = metamodelica::cons(Relation { kind: COMPOSITION.clone(), target: target, label: name.clone(), multiplicity: Dump::printArraydimStr(dims.clone())?, step: USED.clone(), declaredBy: literal!("") }, rels);
            rels = constrainedBy(metamodelica::AsArg::as_arg(&prefixes), metamodelica::AsArg::as_arg(&name), &(list![scope.clone()]), d, rels)?;
            rels = redeclares(metamodelica::AsArg::as_arg(&r#mod), metamodelica::AsArg::as_arg(&name), &scope, d, rels)?;
            ()
        },
        Deref @ SCode::Element::CLASS { name: __esc_name, prefixes: __esc_prefixes, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: __esc_ts, modifications: __esc_mod, .. }, .. } => {
            name = (*__esc_name).clone();
            prefixes = (*__esc_prefixes).clone();
            ts = (*__esc_ts).clone();
            r#mod = (*__esc_mod).clone();
            full_name = { let mut __mm_s = String::new(); __mm_s.push_str(&*scope); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) };
            path = AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&ts));
            target = resolve(full_name.clone(), path.clone(), d, false)?;
            if stringEmpty(&target) {
                target = resolve(scope.clone(), path, d, false)?;
            }
            rels = metamodelica::cons(Relation { kind: NESTING.clone(), target: full_name.clone(), label: literal!(""), multiplicity: literal!(""), step: DRAWN.clone(), declaredBy: literal!("") }, rels);
            rels = metamodelica::cons(Relation { kind: DEPENDENCY.clone(), target: target, label: { let mut __mm_s = String::new(); __mm_s.push_str(&*stereotype(element)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, multiplicity: literal!(""), step: USED.clone(), declaredBy: full_name.clone() }, rels);
            rels = constrainedBy(metamodelica::AsArg::as_arg(&prefixes), metamodelica::AsArg::as_arg(&name), &(list![full_name.clone(), scope]), d, rels)?;
            rels = redeclares(metamodelica::AsArg::as_arg(&r#mod), metamodelica::AsArg::as_arg(&name), &full_name, d, rels)?;
            ()
        },
        Deref @ SCode::Element::CLASS { name: __esc_name, .. } => {
            name = (*__esc_name).clone();
            rels = metamodelica::cons(Relation { kind: NESTING.clone(), target: { let mut __mm_s = String::new(); __mm_s.push_str(&*scope); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, label: literal!(""), multiplicity: literal!(""), step: if (SCodeUtil::isClassExtends(element) || SCodeUtil::isElementRedeclare(element)?) {INHERITED.clone()} else {DRAWN.clone()}, declaredBy: literal!("") }, rels);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rels)
}

fn constrainedBy(
    mut prefixes: &metamodelica::Ref<SCode::Prefixes>,
    mut name: &ArcStr,
    mut scopes: &metamodelica::List<ArcStr>,
    mut d: &Diagram,
    mut rels: metamodelica::List<Relation>,
) -> Result<metamodelica::List<Relation>> {
    let mut rels: metamodelica::List<Relation> = rels;
    let mut target: ArcStr = literal!("");
    let mut path: metamodelica::Ref<Absyn::Path>;
    let () = (::match_deref::match_deref! { match prefixes {
        Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { constrainingClass: __esc_path, .. }) }, .. } => {
            path = (*__esc_path).clone();
            for mut s in &**scopes {
                target = resolve(s.clone(), path.clone(), d, false)?;
                if !(stringEmpty(&target)) {
                    break;
                }
            }
            rels = metamodelica::cons(Relation { kind: DEPENDENCY.clone(), target: target, label: { let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(" constrainedby")); ArcStr::from(__mm_s) }, multiplicity: literal!(""), step: USED.clone(), declaredBy: literal!("") }, rels);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rels)
}

fn redeclares(
    mut r#mod: &metamodelica::Ref<SCode::Mod>,
    mut owner: &ArcStr,
    mut scope: &ArcStr,
    mut d: &Diagram,
    mut rels: metamodelica::List<Relation>,
) -> Result<metamodelica::List<Relation>> {
    let mut rels: metamodelica::List<Relation> = rels;
    let mut target: ArcStr;
    let mut label: ArcStr;
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut e: metamodelica::Ref<SCode::Element>;
    let mut ts: metamodelica::Ref<Absyn::TypeSpec>;
    let () = (match &**r#mod {
        SCode::Mod::MOD {
            subModLst: __esc_submods,
            ..
        } => {
            submods = (*__esc_submods).clone();
            for mut sm in &*submods.clone() {
                rels = redeclares(&sm.r#mod, owner, scope, d, rels)?;
            }
            ()
        }
        SCode::Mod::REDECL { element: __esc_e, .. } => {
            e = (*__esc_e).clone();
            target = (::match_deref::match_deref! { match &(e.clone()) {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: __esc_ts, .. }, .. } => {
                    ts = (*__esc_ts).clone();
                    resolve(scope.clone(), AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&ts)), d, false)?
                },
                Deref @ SCode::Element::COMPONENT { typeSpec: __esc_ts, .. } => {
                    ts = (*__esc_ts).clone();
                    resolve(scope.clone(), AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&ts)), d, false)?
                },
                _ => literal!(""),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            label = elementString(e.clone(), d)?;
            label = if (stringEmpty(&owner)) {
                label
            } else {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*owner);
                    __mm_s.push_str(&*literal!("("));
                    __mm_s.push_str(&*label);
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }
            };
            rels = metamodelica::cons(
                Relation {
                    kind: DEPENDENCY.clone(),
                    target: target,
                    label: label,
                    multiplicity: literal!(""),
                    step: USED.clone(),
                    declaredBy: literal!(""),
                },
                rels,
            );
            ()
        }
        _ => (),
    });
    Ok(rels)
}

fn modString(mut r#mod: metamodelica::Ref<SCode::Mod>, mut d: &Diagram) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = if (d.showModifiers.clone()) {
        oneLine(SCodeDump::printModStr(
            r#mod,
            SCodeDump::generateOptions(false, false, false, false, true, true, true, false, false),
        )?)?
    } else {
        literal!("")
    };
    Ok(r#str)
}

fn elementString(mut element: metamodelica::Ref<SCode::Element>, mut d: &Diagram) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut e: metamodelica::Ref<SCode::Element> = element;
    let mut ts: metamodelica::Ref<Absyn::TypeSpec>;
    let mut attr: SCode::Attributes;
    let () = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(e => SCode::Element::COMPONENT; comment = SCode::noComment.clone());
            if !(d.showModifiers.clone()) {
                assign_variant_field!(e => SCode::Element::COMPONENT; modifications = openmodelica_frontend_types::SCode::Mod::interned_NOMOD());
            }
            ()
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: __esc_ts, attributes: __esc_attr, .. }, .. } => {
            ts = (*__esc_ts).clone();
            attr = (*__esc_attr).clone();
            assign_variant_field!(e => SCode::Element::CLASS; cmt = SCode::noComment.clone());
            if !(d.showModifiers.clone()) {
                assign_variant_field!(e => SCode::Element::CLASS; classDef = metamodelica::Ref::new(SCode::ClassDef::DERIVED { typeSpec: ts.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), attributes: attr.clone() }));
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#str = oneLine(SCodeDump::unparseElementStr(
        e,
        SCodeDump::generateOptions(false, false, false, false, true, true, true, false, false),
    )?)?;
    if ((r#str).len() as i32) > 0
        && substring(r#str.clone(), ((r#str).len() as i32), ((r#str).len() as i32))? == literal!(";")
    {
        r#str = substring(r#str.clone(), 1, ((r#str).len() as i32) - 1)?;
    }
    Ok(r#str)
}

fn oneLine(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = System::trim(
        System::stringReplace(
            System::stringReplace(r#str, literal!("\r"), literal!(""))?,
            literal!("\n"),
            literal!(" "),
        )?,
        literal!(" \u{c}\n\r\t\u{b}"),
    );
    while System::stringFind(res.clone(), literal!("  "))? >= 0 {
        res = System::stringReplace(res, literal!("  "), literal!(" "))?;
    }
    Ok(res)
}

fn stereotype(mut cls: &metamodelica::Ref<SCode::Element>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut strl: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut restriction: SCode::Restriction;
    if SCodeUtil::isElementRedeclare(cls)? {
        strl = metamodelica::cons(literal!("redeclare"), strl);
    }
    if SCodeUtil::isElementReplaceable(cls)? {
        strl = metamodelica::cons(literal!("replaceable"), strl);
    }
    let () = (match &**cls {
        SCode::Element::CLASS {
            restriction: __esc_restriction,
            ..
        } => {
            restriction = (*__esc_restriction).clone();
            strl = metamodelica::cons(SCodeDump::restrString(metamodelica::AsArg::as_arg(&restriction))?, strl);
            if SCodeUtil::isClassExtends(cls) {
                strl = metamodelica::cons(literal!("extends"), strl);
            }
            ()
        }
        _ => (),
    });
    r#str = stringDelimitList(strl.reverse(), literal!(" "));
    Ok(r#str)
}

fn members(
    mut node: &Node,
    mut d: &Diagram,
) -> Result<(
    metamodelica::List<ArcStr>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
)> {
    let mut lines: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut elements: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut vis: ArcStr;
    let mut name: ArcStr;
    for mut e in &*SCodeUtil::getClassElements(&node.element) {
        vis = if (SCodeUtil::isElementProtected(metamodelica::AsArg::as_arg(&e))) {
            literal!("- ")
        } else {
            literal!("+ ")
        };
        let () = (::match_deref::match_deref! { match &(e.clone()) {
            Deref @ SCode::Element::COMPONENT { .. } => {
                lines = metamodelica::cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*vis); __mm_s.push_str(&*elementString(e.clone(), d)?); ArcStr::from(__mm_s) }, lines);
                elements = metamodelica::cons(e.clone(), elements);
                ()
            },
            Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { .. }, .. } => {
                lines = metamodelica::cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*vis); __mm_s.push_str(&*elementString(e.clone(), d)?); ArcStr::from(__mm_s) }, lines);
                elements = metamodelica::cons(e.clone(), elements);
                ()
            },
            Deref @ SCode::Element::CLASS { name, .. } if (SCodeUtil::isElementReplaceable(metamodelica::AsArg::as_arg(&e))? && !(UnorderedMap::contains({ let mut __mm_s = String::new(); __mm_s.push_str(&*node.name); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, d.nodes.clone())?)) => {
                lines = metamodelica::cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*vis); __mm_s.push_str(&*stereotype(metamodelica::AsArg::as_arg(&e))?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, lines);
                elements = metamodelica::cons(e.clone(), elements);
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    lines = lines.reverse();
    elements = elements.reverse();
    Ok((lines, elements))
}

fn nodes(mut d: &Diagram) -> Result<metamodelica::List<Node>> {
    let mut res: metamodelica::List<Node>;
    res = ({
        let mut __acc: metamodelica::List<Node> = metamodelica::nil();
        for mut n in (Pointer::access(d.order.clone()).reverse()).into_iter().cloned() {
            let __x = UnorderedMap::getOrFail(n.clone(), d.nodes.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

fn edges(mut d: &Diagram) -> Result<metamodelica::List<(Node, Node, Relation)>> {
    let mut res: metamodelica::List<(Node, Node, Relation)> = metamodelica::nil();
    let mut seen: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>> = UnorderedSet::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        13,
    );
    let mut key: ArcStr;
    let mut target: Node;
    for mut n in &*nodes(d)? {
        for mut r in &*relations(metamodelica::AsArg::as_arg(&n), d)? {
            if !(stringEmpty(&r.declaredBy)) && UnorderedMap::contains(r.declaredBy.clone(), d.nodes.clone())? {
                continue;
            }
            let () = (match UnorderedMap::get(r.target.clone(), d.nodes.clone())? {
                Some(mut __esc_target) => {
                    target = __esc_target.clone();
                    key = stringDelimitList(
                        list![
                            intString(r.kind.clone()),
                            n.id.clone(),
                            target.id.clone(),
                            r.label.clone()
                        ],
                        literal!(" "),
                    );
                    if !(UnorderedSet::contains(key.clone(), seen.clone())?) {
                        UnorderedSet::add(key, seen.clone())?;
                        res = metamodelica::cons((n.clone(), target.clone(), r.clone()), res);
                    }
                    ()
                }
                _ => (),
            });
        }
    }
    res = res.reverse();
    Ok(res)
}

fn plantuml(mut d: &Diagram) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut strl: metamodelica::List<ArcStr> = list![
        literal!("hide empty members"),
        literal!("skinparam classAttributeIconSize 0"),
        literal!("@startuml")
    ];
    let mut lines: metamodelica::List<ArcStr>;
    let mut source: Node;
    let mut target: Node;
    let mut r: Relation;
    for mut n in &*nodes(d)? {
        (lines, _) = members(metamodelica::AsArg::as_arg(&n), d)?;
        strl = metamodelica::cons(
            stringAppendList(list![
                if (SCodeUtil::isPartial(&n.element)) {
                    literal!("abstract class \"")
                } else {
                    literal!("class \"")
                },
                n.name.clone(),
                literal!("\" as "),
                n.id.clone(),
                literal!(" <<"),
                stereotype(&n.element)?,
                literal!(">>"),
                if ((lines).is_empty()) {
                    literal!("")
                } else {
                    literal!(" {")
                }
            ]),
            strl,
        );
        if !((lines).is_empty()) {
            for mut l in &*lines {
                strl = metamodelica::cons(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("  {field} "));
                        __mm_s.push_str(&*l);
                        ArcStr::from(__mm_s)
                    },
                    strl,
                );
            }
            strl = metamodelica::cons(literal!("}"), strl);
        }
    }
    for mut e in &*edges(d)? {
        (source, target, r) = e.clone();
        strl = metamodelica::cons(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*stringAppendList(if (r.kind.clone() == GENERALIZATION.clone()) {
                    list![target.id.clone(), literal!(" <|-- "), source.id.clone()]
                } else if (r.kind.clone() == COMPOSITION.clone()) {
                    list![
                        source.id.clone(),
                        literal!(" *-- "),
                        if (stringEmpty(&r.multiplicity)) {
                            literal!("")
                        } else {
                            {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("\""));
                                __mm_s.push_str(&*r.multiplicity);
                                __mm_s.push_str(&*literal!("\" "));
                                ArcStr::from(__mm_s)
                            }
                        },
                        target.id.clone()
                    ]
                } else if (r.kind.clone() == DEPENDENCY.clone()) {
                    list![source.id.clone(), literal!(" ..> "), target.id.clone()]
                } else {
                    list![source.id.clone(), literal!(" +-- "), target.id.clone()]
                }));
                __mm_s.push_str(&*if (stringEmpty(&r.label)) {
                    literal!("")
                } else {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!(" : "));
                        __mm_s.push_str(&*r.label);
                        ArcStr::from(__mm_s)
                    }
                });
                ArcStr::from(__mm_s)
            },
            strl,
        );
    }
    strl = metamodelica::cons(literal!("@enduml\n"), strl);
    r#str = stringDelimitList(strl.reverse(), literal!("\n"));
    Ok(r#str)
}

fn mermaid(mut d: &Diagram) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut strl: metamodelica::List<ArcStr> = list![literal!("classDiagram")];
    let mut lines: metamodelica::List<ArcStr>;
    let mut source: Node;
    let mut target: Node;
    let mut r: Relation;
    let mut label: ArcStr;
    for mut n in &*nodes(d)? {
        strl = metamodelica::cons(
            stringAppendList(list![
                literal!("  class "),
                n.id.clone(),
                literal!("[\""),
                n.name.clone(),
                literal!("\"] {")
            ]),
            strl,
        );
        strl = metamodelica::cons(
            stringAppendList(list![
                literal!("    <<"),
                if (SCodeUtil::isPartial(&n.element)) {
                    literal!("partial ")
                } else {
                    literal!("")
                },
                stereotype(&n.element)?,
                literal!(">>")
            ]),
            strl,
        );
        (lines, _) = members(metamodelica::AsArg::as_arg(&n), d)?;
        for mut l in &*lines {
            strl = metamodelica::cons(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("    "));
                    __mm_s.push_str(&*mermaidEscape(l.clone())?);
                    ArcStr::from(__mm_s)
                },
                strl,
            );
        }
        strl = metamodelica::cons(literal!("  }"), strl);
    }
    for mut e in &*edges(d)? {
        (source, target, r) = e.clone();
        label = if (r.kind.clone() == NESTING.clone() && stringEmpty(&r.label)) {
            literal!("nested")
        } else {
            r.label.clone()
        };
        strl = metamodelica::cons(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*stringAppendList(if (r.kind.clone() == GENERALIZATION.clone()) {
                    list![literal!("  "), target.id.clone(), literal!(" <|-- "), source.id.clone()]
                } else if (r.kind.clone() == COMPOSITION.clone()) {
                    list![
                        literal!("  "),
                        source.id.clone(),
                        literal!(" *-- "),
                        if (stringEmpty(&r.multiplicity)) {
                            literal!("")
                        } else {
                            {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("\""));
                                __mm_s.push_str(&*mermaidEscape(r.multiplicity.clone())?);
                                __mm_s.push_str(&*literal!("\" "));
                                ArcStr::from(__mm_s)
                            }
                        },
                        target.id.clone()
                    ]
                } else if (r.kind.clone() == DEPENDENCY.clone()) {
                    list![literal!("  "), source.id.clone(), literal!(" ..> "), target.id.clone()]
                } else {
                    list![literal!("  "), source.id.clone(), literal!(" -- "), target.id.clone()]
                }));
                __mm_s.push_str(&*if (stringEmpty(&label)) {
                    literal!("")
                } else {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!(" : "));
                        __mm_s.push_str(&*mermaidEscape(label)?);
                        ArcStr::from(__mm_s)
                    }
                });
                ArcStr::from(__mm_s)
            },
            strl,
        );
    }
    r#str = stringDelimitList(metamodelica::cons(literal!(""), strl).reverse(), literal!("\n"));
    Ok(r#str)
}

fn mermaidEscape(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = System::stringReplace(r#str, literal!("#"), literal!("#35;"))?;
    res = System::stringReplace(res, literal!("("), literal!("#40;"))?;
    res = System::stringReplace(res, literal!(")"), literal!("#41;"))?;
    res = System::stringReplace(res, literal!("{"), literal!("#123;"))?;
    res = System::stringReplace(res, literal!("}"), literal!("#125;"))?;
    res = System::stringReplace(res, literal!("<"), literal!("#60;"))?;
    res = System::stringReplace(res, literal!(">"), literal!("#62;"))?;
    res = System::stringReplace(res, literal!("~"), literal!("#126;"))?;
    res = System::stringReplace(res, literal!(":"), literal!("#58;"))?;
    Ok(res)
}

fn drawio(mut name: ArcStr, mut d: &Diagram) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut ns: metamodelica::List<Node> = nodes(d)?;
    let mut es: metamodelica::List<(Node, Node, Relation)> = edges(d)?;
    let mut ranks: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>> = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    let mut rank: i32;
    let mut max_rank: i32 = 0;
    let mut x: i32;
    let mut y: i32 = 20;
    let mut w: i32;
    let mut h: i32;
    let mut row_h: i32;
    let mut i: i32 = 0;
    let mut strl: metamodelica::List<ArcStr>;
    let mut lines: metamodelica::List<ArcStr>;
    let mut links: metamodelica::List<ArcStr>;
    let mut elements: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut source: Node;
    let mut target: Node;
    let mut r: Relation;
    let mut label: ArcStr;
    let mut changed: bool = true;
    for mut n in &*ns {
        UnorderedMap::add(n.name.clone(), 0, ranks.clone())?;
    }
    while changed && i < ((ns).len() as i32) {
        changed = false;
        i = i + 1;
        for mut e in &*es {
            (source, target, r) = e.clone();
            if r.kind.clone() == GENERALIZATION.clone() {
                rank = UnorderedMap::getOrFail(target.name.clone(), ranks.clone())? + 1;
                if rank > UnorderedMap::getOrFail(source.name.clone(), ranks.clone())? {
                    UnorderedMap::add(source.name.clone(), rank, ranks.clone())?;
                    max_rank = std::cmp::max(max_rank, rank);
                    changed = true;
                }
            }
        }
    }
    strl = list![
        literal!("      <mxCell id=\"1\" parent=\"0\"/>"),
        literal!("      <mxCell id=\"0\"/>"),
        literal!("    <root>"),
        literal!("   <mxGraphModel>"),
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("  <diagram id=\""));
            __mm_s.push_str(&*xmlEscape(name.clone())?);
            __mm_s.push_str(&*literal!("\" name=\""));
            __mm_s.push_str(&*xmlEscape(name)?);
            __mm_s.push_str(&*literal!("\">"));
            ArcStr::from(__mm_s)
        },
        literal!("<mxfile host=\"OpenModelica\">")
    ];
    for mut rk in 0..=max_rank {
        x = 20;
        row_h = 0;
        for mut n in &*ns {
            if UnorderedMap::getOrFail(n.name.clone(), ranks.clone())? == rk {
                (lines, elements) = members(metamodelica::AsArg::as_arg(&n), d)?;
                w = 20
                    + 7 * ({
                        let mut __acc: Option<i32> = None;
                        for mut l in (metamodelica::cons(
                            stereotype(&n.element)?,
                            metamodelica::cons(n.name.clone(), lines.clone()),
                        ))
                        .into_iter()
                        .cloned()
                        {
                            let __x = ((l).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                h = 44
                    + if ((lines).is_empty()) {
                        0
                    } else {
                        8 + 16 * ((lines).len() as i32)
                    };
                label = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("<p style=\"margin:4px;text-align:center\"><i>&#171;"));
                    __mm_s.push_str(&*htmlEscape(stereotype(&n.element)?)?);
                    __mm_s.push_str(&*literal!("&#187;</i><br/>"));
                    __mm_s.push_str(&*if (SCodeUtil::isPartial(&n.element)) {
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("<b><i>"));
                            __mm_s.push_str(&*htmlEscape(n.name.clone())?);
                            __mm_s.push_str(&*literal!("</i></b>"));
                            ArcStr::from(__mm_s)
                        }
                    } else {
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("<b>"));
                            __mm_s.push_str(&*htmlEscape(n.name.clone())?);
                            __mm_s.push_str(&*literal!("</b>"));
                            ArcStr::from(__mm_s)
                        }
                    });
                    __mm_s.push_str(&*literal!("</p>"));
                    ArcStr::from(__mm_s)
                };
                if !((lines).is_empty()) {
                    links = ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        let __thr_src0 = lines;
                        let mut __thr_it0 = (&__thr_src0).into_iter();
                        let __thr_src1 = elements;
                        let mut __thr_it1 = (&__thr_src1).into_iter();
                        loop {
                            match (__thr_it0.next(), __thr_it1.next()) {
                                (Some(l), Some(e)) => {
                                    let __x = memberLink(metamodelica::AsArg::as_arg(&n), l.clone(), &(e.clone()))?;
                                    __acc = cons(__x, __acc);
                                }
                                (None, None) => break,
                                _ => return Err("threaded for: ranges of unequal length"),
                            }
                        }
                        __acc.reverse()
                    });
                    label = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*label);
                        __mm_s.push_str(&*literal!("<hr size=\"1\"/><p style=\"margin:0 4px\">"));
                        __mm_s.push_str(&*stringDelimitList(links, literal!("<br/>")));
                        __mm_s.push_str(&*literal!("</p>"));
                        ArcStr::from(__mm_s)
                    };
                }
                strl = metamodelica::cons(
                    stringAppendList(list![
                        literal!("      <UserObject id=\""),
                        n.id.clone(),
                        literal!("\" label=\""),
                        xmlEscape(label)?,
                        literal!("\" link=\""),
                        xmlEscape(classLink(n.name.clone())?)?,
                        literal!("\">"),
                        literal!(
                            "<mxCell style=\"verticalAlign=top;align=left;overflow=fill;html=1;whiteSpace=nowrap;\" vertex=\"1\" parent=\"1\">"
                        ),
                        literal!("<mxGeometry x=\""),
                        intString(x),
                        literal!("\" y=\""),
                        intString(y),
                        literal!("\" width=\""),
                        intString(w),
                        literal!("\" height=\""),
                        intString(h),
                        literal!("\" as=\"geometry\"/></mxCell></UserObject>")
                    ]),
                    strl,
                );
                x = x + w + 40;
                row_h = std::cmp::max(row_h, h);
            }
        }
        y = y + row_h + 80;
    }
    i = 0;
    for mut e in &*es {
        (source, target, r) = e.clone();
        i = i + 1;
        label = if (stringEmpty(&r.multiplicity)) {
            r.label.clone()
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r.label);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*r.multiplicity);
                ArcStr::from(__mm_s)
            }
        };
        strl = metamodelica::cons(
            stringAppendList(list![
                literal!("      <mxCell id=\"e"),
                intString(i),
                literal!("\" value=\""),
                xmlEscape(htmlEscape(label)?)?,
                literal!("\" style=\""),
                edgeStyle(r.kind.clone()),
                literal!("html=1;\" edge=\"1\" parent=\"1\" source=\""),
                source.id.clone(),
                literal!("\" target=\""),
                target.id.clone(),
                literal!("\"><mxGeometry relative=\"1\" as=\"geometry\"/></mxCell>")
            ]),
            strl,
        );
    }
    strl = metamodelica::cons(
        literal!("</mxfile>\n"),
        metamodelica::cons(
            literal!("  </diagram>"),
            metamodelica::cons(
                literal!("   </mxGraphModel>"),
                metamodelica::cons(literal!("    </root>"), strl),
            ),
        ),
    );
    r#str = stringDelimitList(strl.reverse(), literal!("\n"));
    Ok(r#str)
}

fn classLink(mut name: ArcStr) -> Result<ArcStr> {
    let mut link: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("modelica://"));
        __mm_s.push_str(&*urlEscape(name.clone())?);
        ArcStr::from(__mm_s)
    };
    Ok(link)
}

fn memberLink(mut node: &Node, mut line: ArcStr, mut element: &metamodelica::Ref<SCode::Element>) -> Result<ArcStr> {
    let mut link: ArcStr;
    let mut info: SourceInfo = SCodeUtil::elementInfo(element);
    let mut href: ArcStr;
    link = htmlEscape(line)?;
    if info.lineNumberStart.clone() > 0 {
        href = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*classLink(node.name.clone())?);
            __mm_s.push_str(&*literal!("?lineNumber="));
            __mm_s.push_str(&*intString(info.lineNumberStart.clone()));
            ArcStr::from(__mm_s)
        };
        if SCodeUtil::isComponent(element) {
            href = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*href);
                __mm_s.push_str(&*literal!("&element="));
                __mm_s.push_str(&*urlEscape(SCodeUtil::elementName(element)?)?);
                ArcStr::from(__mm_s)
            };
        }
        link = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("<a href=\""));
            __mm_s.push_str(&*htmlEscape(href)?);
            __mm_s.push_str(&*literal!("\" style=\"color:inherit;text-decoration:none\">"));
            __mm_s.push_str(&*link);
            __mm_s.push_str(&*literal!("</a>"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(link)
}

fn urlEscape(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = System::stringReplace(r#str, literal!("%"), literal!("%25"))?;
    res = System::stringReplace(res, literal!(" "), literal!("%20"))?;
    res = System::stringReplace(res, literal!("\""), literal!("%22"))?;
    res = System::stringReplace(res, literal!("#"), literal!("%23"))?;
    res = System::stringReplace(res, literal!("'"), literal!("%27"))?;
    res = System::stringReplace(res, literal!("<"), literal!("%3C"))?;
    res = System::stringReplace(res, literal!(">"), literal!("%3E"))?;
    res = System::stringReplace(res, literal!("?"), literal!("%3F"))?;
    Ok(res)
}

fn edgeStyle(mut kind: i32) -> ArcStr {
    let mut style: ArcStr;
    style = if (kind == GENERALIZATION.clone()) {
        literal!("endArrow=block;endFill=0;endSize=12;")
    } else if (kind == COMPOSITION.clone()) {
        literal!("startArrow=diamondThin;startFill=1;startSize=14;endArrow=none;")
    } else if (kind == DEPENDENCY.clone()) {
        literal!("dashed=1;endArrow=open;endSize=12;")
    } else {
        literal!("startArrow=circlePlus;startFill=0;endArrow=none;")
    };
    style
}

fn htmlEscape(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = System::stringReplace(r#str, literal!("&"), literal!("&amp;"))?;
    res = System::stringReplace(res, literal!("<"), literal!("&lt;"))?;
    res = System::stringReplace(res, literal!(">"), literal!("&gt;"))?;
    Ok(res)
}

fn xmlEscape(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = htmlEscape(r#str)?;
    res = System::stringReplace(res, literal!("\""), literal!("&quot;"))?;
    Ok(res)
}
