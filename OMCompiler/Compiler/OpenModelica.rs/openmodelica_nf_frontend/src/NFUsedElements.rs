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

use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFInst as Inst;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;
use openmodelica_util_datatypes_basic::Pointer;

pub(crate) const CONTEXT: i32 = intBitOr(InstContext::RELAXED, InstContext::FAST_LOOKUP);

pub type PathList = metamodelica::List<metamodelica::Ref<Absyn::Path>>;

pub type NodeList = metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;

pub type PendingList = metamodelica::List<Pending>;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Walk {
    /// Keys of the used elements, see elementKey.
    pub used: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>,
    /// Keys of the walked classes and constants.
    pub walked: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>,
    /// Names that were looked up.
    pub names: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>,
    /// Classes found but not walked yet.
    pub queue: Pointer::Pointer<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>>,
    /// The walked classes.
    pub classes: Pointer::Pointer<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>>,
    /// The rest of the names looked up through a replaceable class, by its key.
    pub rests:
        metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::List<metamodelica::Ref<Absyn::Path>>>>,
    /// The classes a replaceable class is redeclared as, by its key.
    pub replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<ArcStr, metamodelica::List<metamodelica::Ref<InstNode::InstNode>>>,
    >,
    /// Records the uses of the names, see collectUses.
    pub recorder: Option<Recorder>,
}

impl metamodelica::gc::MMTrace for Walk {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.used, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.walked, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.names, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.queue, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.classes, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.rests, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.replacements, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.recorder, __mmv)?;
        Ok(())
    }
}
impl Default for Walk {
    fn default() -> Self {
        Self {
            used: Default::default(),
            walked: Default::default(),
            names: Default::default(),
            queue: Default::default(),
            classes: Default::default(),
            rests: Default::default(),
            replacements: Default::default(),
            recorder: Default::default(),
        }
    }
}

pub type WALK = Walk;

/// The last element found by findElement or lookupFirstIdent.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Found {
    NOT_FOUND,
    FOUND_NODE {
        node: metamodelica::Ref<InstNode::InstNode>,
    },
    FOUND_ELEMENT {
        element: metamodelica::Ref<SCode::Element>,
        /// The class the element is declared in.
        scope: metamodelica::Ref<InstNode::InstNode>,
    },
}
impl metamodelica::gc::MMTrace for Found {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Found::NOT_FOUND => Ok(()),
            Found::FOUND_NODE { node } => {
                metamodelica::gc::MMTrace::mm_accept(node, __mmv)?;
                Ok(())
            }
            Found::FOUND_ELEMENT { element, scope } => {
                metamodelica::gc::MMTrace::mm_accept(element, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Found {
    fn default() -> Self {
        Self::NOT_FOUND
    }
}
pub(crate) use self::Found::{FOUND_ELEMENT, FOUND_NODE, NOT_FOUND};

/// A name looked up through a replaceable class, to look up the rest of it in
///     the classes the replaceable class is redeclared as.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Pending {
    pub rest: metamodelica::Ref<Absyn::Path>,
    pub written: metamodelica::Ref<Absyn::Path>,
    /// The part of the written name the rest begins with.
    pub index: i32,
    pub site: Site,
}

impl metamodelica::gc::MMTrace for Pending {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.rest, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.written, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.site, __mmv)?;
        Ok(())
    }
}
impl Default for Pending {
    fn default() -> Self {
        Self {
            rest: Default::default(),
            written: Default::default(),
            index: Default::default(),
            site: Default::default(),
        }
    }
}

pub type PENDING = Pending;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Recorder {
    /// By key.
    pub definitions: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Definition>>,
    /// In reverse order.
    pub uses: Pointer::Pointer<metamodelica::List<Use>>,
    /// Names that weren't found, in reverse order.
    pub unresolved: Pointer::Pointer<metamodelica::List<Use>>,
    /// Where the names looked up are used, none while not recording.
    pub site: Pointer::Pointer<Option<Site>>,
    /// The name of the class being walked.
    pub scope: Pointer::Pointer<ArcStr>,
    /// Looking up the rest of a name in a redeclared class.
    pub candidate: Pointer::Pointer<bool>,
    pub found: Pointer::Pointer<Found>,
    /// The names and keys of the iterators in scope.
    pub iterators: Pointer::Pointer<metamodelica::List<(ArcStr, ArcStr)>>,
    /// By the key of the replaceable class.
    pub pending: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::List<Pending>>>,
    pub pendingKeys: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>,
}

impl metamodelica::gc::MMTrace for Recorder {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.definitions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.uses, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.unresolved, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.site, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.scope, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.candidate, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.found, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.iterators, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.pending, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.pendingKeys, __mmv)?;
        Ok(())
    }
}
impl Default for Recorder {
    fn default() -> Self {
        Self {
            definitions: Default::default(),
            uses: Default::default(),
            unresolved: Default::default(),
            site: Default::default(),
            scope: Default::default(),
            candidate: Default::default(),
            found: Default::default(),
            iterators: Default::default(),
            pending: Default::default(),
            pendingKeys: Default::default(),
        }
    }
}

pub type RECORDER = Recorder;

pub fn collect(
    mut classPaths: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut annotationProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>> {
    let mut used: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>;
    let mut top: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<InstNode::InstNode>;
    let mut walk: Walk;
    let mut names_count: i32;
    Inst::resetGlobalFlags()?;
    top = Inst::makeTopNode(program, annotationProgram)?;
    walk = Walk {
        used: UnorderedSet::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            13,
        ),
        walked: UnorderedSet::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            13,
        ),
        names: UnorderedSet::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            13,
        ),
        queue: Pointer::create(metamodelica::nil()),
        classes: Pointer::create(metamodelica::nil()),
        rests: UnorderedMap::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            1,
        ),
        replacements: UnorderedMap::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            1,
        ),
        recorder: None,
    };
    for mut path in &**classPaths {
        if '__try0: {
            cls = unwrap_break_err!(walkPath(&(AbsynUtil::makeFullyQualified(path.clone())), &top, &walk, &(AbsynUtil::makeFullyQualified(path.clone())), 1), '__try0);
            if unwrap_break_err!(InstNode::isClass(&cls), '__try0) {
                unwrap_break_err!(walkClass(cls.clone(), walk.clone(), true), '__try0);
            }
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    loop {
        walkQueue(walk.clone());
        names_count = UnorderedSet::size(walk.names.clone());
        for mut c in &*Pointer::access(walk.classes.clone()) {
            if '__try1: {
                unwrap_break_err!(walkNamedElements(c.clone(), walk.clone()), '__try1);
                Ok::<(), &'static str>(())
            }
            .is_err()
            {}
        }
        if (Pointer::access(walk.queue.clone())).is_empty() && UnorderedSet::size(walk.names.clone()) == names_count {
            break;
        }
    }
    used = walk.used.clone();
    Inst::clearCaches()?;
    Ok(used)
}

/// Where a name is used.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Site {
    /// Of the element, equation, statement or class it's used in.
    pub info: SourceInfo,
    /// What it's used as, e.g. type, extends, modifier, equation.
    pub role: ArcStr,
    /// The class it's used in.
    pub scope: ArcStr,
}

impl metamodelica::gc::MMTrace for Site {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.info, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.role, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.scope, __mmv)?;
        Ok(())
    }
}
impl Default for Site {
    fn default() -> Self {
        Self {
            info: Default::default(),
            role: Default::default(),
            scope: Default::default(),
        }
    }
}

pub type SITE = Site;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Definition {
    /// See elementKey.
    pub key: ArcStr,
    /// The full name, e.g. P.Base.x.
    pub name: ArcStr,
    /// class, component or iterator.
    pub kind: ArcStr,
    /// The restriction of a class, the type of a component.
    pub detail: ArcStr,
    pub info: SourceInfo,
    /// Declared in one of the walked classes.
    pub declared: bool,
}

impl metamodelica::gc::MMTrace for Definition {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.key, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.kind, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.detail, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.info, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.declared, __mmv)?;
        Ok(())
    }
}
impl Default for Definition {
    fn default() -> Self {
        Self {
            key: Default::default(),
            name: Default::default(),
            kind: Default::default(),
            detail: Default::default(),
            info: Default::default(),
            declared: Default::default(),
        }
    }
}

pub type DEFINITION = Definition;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Use {
    /// The key of the definition, empty if the name wasn't found.
    pub key: ArcStr,
    /// The name as written, without subscripts.
    pub written: metamodelica::Ref<Absyn::Path>,
    /// The part of the written name that refers to the definition.
    pub index: i32,
    pub site: Site,
    /// Found in a class a replaceable class is redeclared as.
    pub candidate: bool,
}

impl metamodelica::gc::MMTrace for Use {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.key, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.written, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.site, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.candidate, __mmv)?;
        Ok(())
    }
}
impl Default for Use {
    fn default() -> Self {
        Self {
            key: Default::default(),
            written: Default::default(),
            index: Default::default(),
            site: Default::default(),
            candidate: Default::default(),
        }
    }
}

pub type USE = Use;

pub fn collectUses(
    mut classPaths: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut annotationProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<(
    metamodelica::List<Definition>,
    metamodelica::List<Use>,
    metamodelica::List<Use>,
)> {
    let mut definitions: metamodelica::List<Definition>;
    let mut uses: metamodelica::List<Use>;
    let mut unresolved: metamodelica::List<Use>;
    let mut top: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<InstNode::InstNode>;
    let mut walk: Walk;
    let mut rec: Recorder;
    Inst::resetGlobalFlags()?;
    top = Inst::makeTopNode(program, annotationProgram)?;
    rec = Recorder {
        definitions: UnorderedMap::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            1,
        ),
        uses: Pointer::create(metamodelica::nil()),
        unresolved: Pointer::create(metamodelica::nil()),
        site: Pointer::create(None),
        scope: Pointer::create(literal!("")),
        candidate: Pointer::create(false),
        found: Pointer::create(crate::NFUsedElements::Found::NOT_FOUND),
        iterators: Pointer::create(metamodelica::nil()),
        pending: UnorderedMap::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            1,
        ),
        pendingKeys: UnorderedSet::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            13,
        ),
    };
    walk = Walk {
        used: UnorderedSet::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            13,
        ),
        walked: UnorderedSet::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            13,
        ),
        names: UnorderedSet::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            13,
        ),
        queue: Pointer::create(metamodelica::nil()),
        classes: Pointer::create(metamodelica::nil()),
        rests: UnorderedMap::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            1,
        ),
        replacements: UnorderedMap::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            1,
        ),
        recorder: Some(rec.clone()),
    };
    for mut path in &**classPaths {
        if '__try0: {
            cls = unwrap_break_err!(declaredClass(path.clone(), &top, &walk), '__try0);
            if unwrap_break_err!(InstNode::isClass(&cls), '__try0) {
                Pointer::update(
                    rec.scope.clone(),
                    unwrap_break_err!(AbsynUtil::pathString(path.clone(), literal!("."), true, false), '__try0),
                );
                unwrap_break_err!(walkClass(cls.clone(), walk.clone(), true), '__try0);
            }
            Ok::<(), &'static str>(())
        }
        .is_err()
        {}
        Pointer::update(rec.site.clone(), None);
        Pointer::update(rec.iterators.clone(), metamodelica::nil());
    }
    definitions = UnorderedMap::valueList(rec.definitions.clone());
    uses = Pointer::access(rec.uses.clone()).reverse();
    unresolved = Pointer::access(rec.unresolved.clone()).reverse();
    Inst::clearCaches()?;
    Ok((definitions, uses, unresolved))
}

pub fn elementKey(mut definition: &metamodelica::Ref<SCode::Element>) -> Result<ArcStr> {
    let mut key: ArcStr;
    let mut info: SourceInfo = SCodeUtil::elementInfo(definition);
    key = stringAppendList(list![
        SCodeUtil::elementName(definition)?,
        literal!("@"),
        info.fileName.clone(),
        literal!(":"),
        intString(info.lineNumberStart.clone()),
        literal!(":"),
        intString(info.columnNumberStart.clone()),
        literal!("-"),
        intString(info.lineNumberEnd.clone()),
        literal!(":"),
        intString(info.columnNumberEnd.clone())
    ]);
    Ok(key)
}

fn declaredClass(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut top: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: &Walk,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut cls: metamodelica::Ref<InstNode::InstNode>;
    let mut parent: metamodelica::Ref<InstNode::InstNode>;
    let mut n: metamodelica::Ref<InstNode::InstNode>;
    let mut name: ArcStr;
    cls = walkPath(
        &(AbsynUtil::makeFullyQualified(path.clone())),
        top,
        walk,
        &(AbsynUtil::makeFullyQualified(path.clone())),
        1,
    )?;
    if !(AbsynUtil::pathIsQual(&path)) {
        return Ok(cls);
    }
    parent = walkPath(
        &(AbsynUtil::makeFullyQualified(AbsynUtil::stripLast(&path)?)),
        top,
        walk,
        &(AbsynUtil::makeFullyQualified(AbsynUtil::stripLast(&path)?)),
        1,
    )?;
    if !(InstNode::isClass(&parent)?) {
        return Ok(cls);
    }
    name = AbsynUtil::pathLastIdent(&path);
    for mut e in &*SCodeUtil::getClassElements(&(InstNode::definition(parent.clone())?)) {
        if SCodeUtil::elementIsClass(metamodelica::AsArg::as_arg(&e))
            && metamodelica::stringEq(&(SCodeUtil::elementName(metamodelica::AsArg::as_arg(&e))?), &name)
            && (isReplacingClass(metamodelica::AsArg::as_arg(&e))? || !(InstNode::isClass(&cls)?))
        {
            if !(InstNode::isClass(&cls)?
                && metamodelica::stringEq(
                    &(elementKey(&(InstNode::definition(cls.clone())?))?),
                    &(elementKey(metamodelica::AsArg::as_arg(&e))?),
                ))
            {
                parent = expandNode(parent)?;
                n = localClass(metamodelica::AsArg::as_arg(&e), parent.clone())?;
                cls = if (InstNode::isClass(&n)?) {
                    n
                } else {
                    InstNode::newClass(
                        e.clone(),
                        parent.clone(),
                        crate::NFInstNode::InstNodeType::interned_NORMAL_CLASS(),
                    )?
                };
            }
            break;
        }
    }
    Ok(cls)
}

fn isRecording(mut walk: &Walk) -> bool {
    let mut res: bool = (walk.recorder).is_some();
    res
}

fn setFound(mut found: Found, mut walk: &Walk) -> () {
    let () = (match walk.recorder.clone() {
        Some(mut rec) => {
            Pointer::update(rec.found.clone(), found);
            ()
        }
        _ => (),
    });
    ()
}

fn takeFound(mut walk: &Walk) -> Found {
    let mut found: Found = crate::NFUsedElements::Found::NOT_FOUND;
    let () = (match walk.recorder.clone() {
        Some(mut rec) => {
            found = Pointer::access(rec.found.clone());
            Pointer::update(rec.found.clone(), crate::NFUsedElements::Found::NOT_FOUND);
            ()
        }
        _ => (),
    });
    found
}

fn enterSite(mut info: SourceInfo, mut role: ArcStr, mut walk: &Walk) -> Option<Site> {
    let mut old: Option<Site> = None;
    let () = (match walk.recorder.clone() {
        Some(mut rec) => {
            old = Pointer::access(rec.site.clone());
            Pointer::update(
                rec.site.clone(),
                Some(Site {
                    info: info,
                    role: role,
                    scope: Pointer::access(rec.scope.clone()),
                }),
            );
            ()
        }
        _ => (),
    });
    old
}

fn setRole(mut role: ArcStr, mut walk: &Walk) -> Result<Option<Site>> {
    let mut old: Option<Site> = None;
    let () = (match walk.recorder.clone() {
        Some(mut rec) => {
            let mut site: Site;
            old = Pointer::access(rec.site.clone());
            if (old).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(old.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                site = metamodelica::Own::own(__pa0);
                site.role = role;
                Pointer::update(rec.site.clone(), Some(site));
            }
            ()
        }
        _ => (),
    });
    Ok(old)
}

fn leaveSite(mut old: Option<Site>, mut walk: &Walk) -> () {
    let () = (match walk.recorder.clone() {
        Some(mut rec) => {
            Pointer::update(rec.site.clone(), old);
            ()
        }
        _ => (),
    });
    ()
}

fn suspendRecording(mut walk: &Walk) -> Option<Site> {
    let mut old: Option<Site> = leaveSiteNone(walk);
    old
}

fn leaveSiteNone(mut walk: &Walk) -> Option<Site> {
    let mut old: Option<Site> = None;
    let () = (match walk.recorder.clone() {
        Some(mut rec) => {
            old = Pointer::access(rec.site.clone());
            Pointer::update(rec.site.clone(), None);
            ()
        }
        _ => (),
    });
    old
}

fn makeDefinition(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut name: ArcStr,
    mut declared: bool,
) -> Result<Definition> {
    let mut def: Definition;
    let mut kind: ArcStr;
    let mut detail: ArcStr;
    (kind, detail) = (match &**element {
        SCode::Element::CLASS {
            restriction: __element_restriction,
            ..
        } => (
            literal!("class"),
            SCodeDump::restrString(metamodelica::AsArg::as_arg(&__element_restriction))?,
        ),
        SCode::Element::COMPONENT {
            typeSpec: __element_typeSpec,
            ..
        } => (
            literal!("component"),
            Dump::unparseTypeSpec(__element_typeSpec.clone())?,
        ),
        _ => (literal!("element"), literal!("")),
    });
    def = Definition {
        key: elementKey(element)?,
        name: name,
        kind: kind,
        detail: detail,
        info: SCodeUtil::elementInfo(element),
        declared: declared,
    };
    Ok(def)
}

fn addDefinition(mut def: Definition, mut rec: &Recorder) -> Result<()> {
    let mut odef: Option<Definition>;
    let mut old: Definition;
    odef = UnorderedMap::get(def.key.clone(), rec.definitions.clone())?;
    if (odef).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(odef) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        old = metamodelica::Own::own(__pa0);
        if old.declared.clone() || !(def.declared.clone()) {
            return Ok(());
        }
    }
    UnorderedMap::add(def.key.clone(), def, rec.definitions.clone())?;
    Ok(())
}

fn declare(mut element: &metamodelica::Ref<SCode::Element>, mut name: ArcStr, mut walk: &Walk) -> () {
    let () = (match walk.recorder.clone() {
        Some(mut rec) => {
            if '__try0: {
                unwrap_break_err!(addDefinition(unwrap_break_err!(makeDefinition(element, name.clone(), true), '__try0), &rec), '__try0);
                Ok::<(), &'static str>(())
            }.is_err() {
            }
            ()
        }
        _ => (),
    });
    ()
}

fn foundDefinition(mut found: &Found) -> Option<Definition> {
    let mut def: Option<Definition> = None;
    let mut n: metamodelica::Ref<InstNode::InstNode>;
    let mut e: metamodelica::Ref<SCode::Element>;
    let mut name: ArcStr;
    if '__try0: {
        let () = (::match_deref::match_deref! { match &(found) {
        Found::FOUND_NODE { node: n @ Deref @ InstNode::COMPONENT_NODE { definition: None, .. } } if (!(InstNode::isBuiltin(&(unwrap_break_err!(InstNode::parent(metamodelica::AsArg::as_arg(&n)), '__try0))))) => {
            e = unwrap_break_err!(InstNode::definition(unwrap_break_err!(InstNode::parent(metamodelica::AsArg::as_arg(&n)), '__try0)), '__try0);
            name = unwrap_break_err!(AbsynUtil::pathString(unwrap_break_err!(InstNode::scopePath(n.clone(), InstNode::ScopeType::RELATIVE.clone(), true), '__try0), literal!("."), true, false), '__try0);
            def = Some(Definition { key: { let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(InstNode::name(metamodelica::AsArg::as_arg(&n)), '__try0)); __mm_s.push_str(&*literal!("@")); __mm_s.push_str(&*unwrap_break_err!(elementKey(&e), '__try0)); ArcStr::from(__mm_s) }, name: name.clone(), kind: literal!("enumerationLiteral"), detail: literal!(""), info: SCodeUtil::elementInfo(&e), declared: false });
            ()
        },
        Found::FOUND_NODE { node: n } if ((unwrap_break_err!(InstNode::isClass(metamodelica::AsArg::as_arg(&n)), '__try0) || unwrap_break_err!(InstNode::isComponent(metamodelica::AsArg::as_arg(&n)), '__try0)) && !(InstNode::isBuiltin(metamodelica::AsArg::as_arg(&n)))) => {
            e = unwrap_break_err!(InstNode::definition(n.clone()), '__try0);
            name = unwrap_break_err!(AbsynUtil::pathString(unwrap_break_err!(InstNode::scopePath(n.clone(), InstNode::ScopeType::RELATIVE.clone(), true), '__try0), literal!("."), true, false), '__try0);
            def = Some(unwrap_break_err!(makeDefinition(&e, name.clone(), false), '__try0));
            ()
        },
        Found::FOUND_ELEMENT { element: __esc_e, scope: __esc_n } => {
            e = (*__esc_e).clone();
            n = (*__esc_n).clone();
            name = { let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(AbsynUtil::pathString(unwrap_break_err!(InstNode::scopePath(n.clone(), InstNode::ScopeType::RELATIVE.clone(), true), '__try0), literal!("."), true, false), '__try0)); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*unwrap_break_err!(SCodeUtil::elementName(metamodelica::AsArg::as_arg(&e)), '__try0)); ArcStr::from(__mm_s) };
            def = Some(unwrap_break_err!(makeDefinition(metamodelica::AsArg::as_arg(&e), name.clone(), false), '__try0));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok::<(), &'static str>(())
    }.is_err() {
        def = None;
    }
    let () = (match def.clone() {
        Some(mut d) if (StringUtil::endsWith(d.info.fileName.clone(), literal!("ModelicaBuiltin.mo"))) => {
            def = None;
            ()
        }
        _ => (),
    });
    def
}

fn recordFound(
    mut found: &Found,
    mut written: metamodelica::Ref<Absyn::Path>,
    mut index: i32,
    mut walk: &Walk,
) -> Result<()> {
    let () = (match walk.recorder.clone() {
        Some(mut rec) if ((Pointer::access(rec.site.clone())).is_some()) => {
            let mut site: Site;
            let mut def: Definition;
            let __pa0 = ::match_deref::match_deref! { match &(Pointer::access(rec.site.clone())) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            site = metamodelica::Own::own(__pa0);
            let () = (match foundDefinition(found) {
                Some(mut __esc_def) => {
                    def = __esc_def.clone();
                    addDefinition(def.clone(), &rec)?;
                    Pointer::update(
                        rec.uses.clone(),
                        metamodelica::cons(
                            Use {
                                key: def.key.clone(),
                                written: written,
                                index: index,
                                site: site,
                                candidate: Pointer::access(rec.candidate.clone()),
                            },
                            Pointer::access(rec.uses.clone()),
                        ),
                    );
                    ()
                }
                _ => {
                    if isNotFound(found) && !(Pointer::access(rec.candidate.clone())) {
                        Pointer::update(
                            rec.unresolved.clone(),
                            metamodelica::cons(
                                Use {
                                    key: literal!(""),
                                    written: written,
                                    index: index,
                                    site: site,
                                    candidate: false,
                                },
                                Pointer::access(rec.unresolved.clone()),
                            ),
                        );
                    }
                    ()
                }
            });
            ()
        }
        _ => (),
    });
    Ok(())
}

fn recordIterator(mut name: &ArcStr, mut written: metamodelica::Ref<Absyn::Path>, mut walk: &Walk) -> Result<bool> {
    let mut isIterator: bool = false;
    let () = (match walk.recorder.clone() {
        Some(mut rec) if ((Pointer::access(rec.site.clone())).is_some()) => {
            let mut site: Site;
            let __pa0 = ::match_deref::match_deref! { match &(Pointer::access(rec.site.clone())) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            site = metamodelica::Own::own(__pa0);
            for mut it in &*Pointer::access(rec.iterators.clone()) {
                if metamodelica::stringEq(&(Util::tuple21(it.clone())), &name) {
                    Pointer::update(
                        rec.uses.clone(),
                        metamodelica::cons(
                            Use {
                                key: Util::tuple22(it.clone()),
                                written: written,
                                index: 1,
                                site: site,
                                candidate: Pointer::access(rec.candidate.clone()),
                            },
                            Pointer::access(rec.uses.clone()),
                        ),
                    );
                    isIterator = true;
                    return Ok(isIterator);
                }
            }
            ()
        }
        _ => (),
    });
    Ok(isIterator)
}

fn pushIterator(mut name: ArcStr, mut walk: &Walk) -> Result<()> {
    let () = (match walk.recorder.clone() {
        Some(mut rec) if ((Pointer::access(rec.site.clone())).is_some()) => {
            let mut site: Site;
            let mut key: ArcStr;
            let __pa0 = ::match_deref::match_deref! { match &(Pointer::access(rec.site.clone())) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            site = metamodelica::Own::own(__pa0);
            key = stringAppendList(list![
                name.clone(),
                literal!("@"),
                site.info.fileName.clone(),
                literal!(":"),
                intString(site.info.lineNumberStart.clone()),
                literal!(":"),
                intString(site.info.columnNumberStart.clone()),
                literal!("-"),
                intString(site.info.lineNumberEnd.clone()),
                literal!(":"),
                intString(site.info.columnNumberEnd.clone())
            ]);
            addDefinition(
                Definition {
                    key: key.clone(),
                    name: {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*site.scope);
                        __mm_s.push_str(&*literal!("."));
                        __mm_s.push_str(&*name);
                        ArcStr::from(__mm_s)
                    },
                    kind: literal!("iterator"),
                    detail: literal!(""),
                    info: site.info.clone(),
                    declared: true,
                },
                &rec,
            )?;
            Pointer::update(
                rec.iterators.clone(),
                metamodelica::cons((name, key), Pointer::access(rec.iterators.clone())),
            );
            ()
        }
        _ => (),
    });
    Ok(())
}

fn popIterator(mut walk: &Walk) -> Result<()> {
    let () = (match walk.recorder.clone() {
        Some(mut rec) => {
            if !((Pointer::access(rec.iterators.clone())).is_empty()) {
                Pointer::update(rec.iterators.clone(), (Pointer::access(rec.iterators.clone())).rest()?);
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

fn isNotFound(mut found: &Found) -> bool {
    let mut res: bool;
    res = (match found.clone() {
        Found::NOT_FOUND { .. } => true,
        _ => false,
    });
    res
}

fn foundLocalName(
    mut name: ArcStr,
    mut cls: metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
    mut needType: bool,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    (node, _) = findElement(name, cls, walk, needType)?;
    Ok(node)
}

fn findElement(
    mut name: ArcStr,
    mut cls: metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
    mut needType: bool,
) -> Result<(metamodelica::Ref<InstNode::InstNode>, bool)> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    let mut found: bool = false;
    if InstNode::isClass(&cls)? {
        match '__try0: {
            (node, found) = unwrap_break_err!(findElement2(name.clone(), cls.clone(), walk.clone(), needType), '__try0);
            Ok::<_, &'static str>((found.clone(), node.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                found = __try0_o0;
                node = __try0_o1;
            }
            Err(_) => {
                node = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
                found = false;
            }
        }
    }
    Ok((node, found))
}

fn findElement2(
    mut name: ArcStr,
    mut cls: metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
    mut needType: bool,
) -> Result<(metamodelica::Ref<InstNode::InstNode>, bool)> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    let mut found: bool = false;
    let mut c: metamodelica::Ref<InstNode::InstNode> = expandNode(cls.clone())?;
    let mut n: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    let mut def: metamodelica::Ref<SCode::Element> = InstNode::definition(c.clone())?;
    if '__try0: {
        (n, _) = unwrap_break_err!(Lookup::lookupLocalSimpleName(name.clone(), c.clone()), '__try0);
        found = true;
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    if isRecording(&walk) {
        node = redeclaredClass(&name, c.clone())?;
        if InstNode::isClass(&node)? {
            n = node;
            found = true;
        }
        node = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    }
    if found {
        UnorderedSet::add(name, walk.names.clone())?;
        foundNode(n.clone(), walk.clone())?;
        setFound(Found::FOUND_NODE { node: n.clone() }, &walk);
        if InstNode::isClass(&n)? {
            node = n;
        }
        return Ok((node, found));
    }
    for mut e in &*SCodeUtil::getClassElements(&def) {
        if SCodeUtil::isComponent(metamodelica::AsArg::as_arg(&e))
            && metamodelica::stringEq(&(SCodeUtil::elementName(metamodelica::AsArg::as_arg(&e))?), &name)
        {
            foundComponent(metamodelica::AsArg::as_arg(&e), c.clone(), walk.clone())?;
            if needType {
                node = componentClass(metamodelica::AsArg::as_arg(&e), &c, &walk)?;
            }
            setFound(
                Found::FOUND_ELEMENT {
                    element: e.clone(),
                    scope: c,
                },
                &walk,
            );
            found = true;
            return Ok((node, found));
        }
    }
    for mut b in &*baseClasses(c)? {
        (node, found) = findElement2(name.clone(), b.clone(), walk.clone(), needType)?;
        if found {
            return Ok((node, found));
        }
    }
    Ok((node, found))
}

fn foundComponent(
    mut component: &metamodelica::Ref<SCode::Element>,
    mut cls: metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
) -> Result<()> {
    let mut key: ArcStr = elementKey(component)?;
    foundNode(cls.clone(), walk.clone())?;
    UnorderedSet::add(SCodeUtil::elementName(component)?, walk.names.clone())?;
    UnorderedSet::add(key.clone(), walk.used.clone())?;
    if !(isRecording(&walk))
        && SCodeUtil::isPackage(&(InstNode::definition(cls.clone())?))
        && !(UnorderedSet::contains(key.clone(), walk.walked.clone())?)
    {
        UnorderedSet::add(key, walk.walked.clone())?;
        walkElement(component, cls, walk, false)?;
    }
    Ok(())
}

fn localClass(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut cls: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    let mut key: ArcStr = elementKey(element)?;
    let mut classes: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut structor_ref: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    match '__try0: {
        classes = unwrap_break_err!(ClassTree::getClasses(&(unwrap_break_err!(Class::classTree(unwrap_break_err!(InstNode::getClass(cls.clone()), '__try0)), '__try0))), '__try0);
        Ok::<_, &'static str>((classes.clone(),))
    } {
        Ok((__try0_o0,)) => {
            classes = __try0_o0;
        }
        Err(_) => {
            if '__try1: {
                (node, _) = unwrap_break_err!(Lookup::lookupLocalSimpleName(unwrap_break_err!(SCodeUtil::elementName(element), '__try1), cls.clone()), '__try1);
                Ok::<(), &'static str>(())
            }.is_err() {
            }
            classes = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
        }
    }
    let __range2 = classes.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut n in __range2 {
        if InstNode::isClass(&n)? && metamodelica::stringEq(&(elementKey(&(InstNode::definition(n.clone())?))?), &key) {
            node = n;
            break;
        }
    }
    if InstNode::isEmpty(&node) {
        if '__try3: {
            let __pa4 = ::match_deref::match_deref! { match &(unwrap_break_err!(InstNode::getType(cls.clone()), '__try3)) {
                Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { constructor: __pa4, .. }, .. } => __pa4.clone(),
                _ => break '__try3 Err::<_, _>("pattern mismatch"),
            } };
            structor_ref = metamodelica::Own::own(__pa4);
            if metamodelica::stringEq(&(unwrap_break_err!(SCodeUtil::elementName(element), '__try3)), &(literal!("destructor"))) {
                let __pa6 = ::match_deref::match_deref! { match &(unwrap_break_err!(InstNode::getType(cls.clone()), '__try3)) {
                    Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { destructor: __pa6, .. }, .. } => __pa6.clone(),
                    _ => break '__try3 Err::<_, _>("pattern mismatch"),
                } };
                structor_ref = metamodelica::Own::own(__pa6);
            }
            node = unwrap_break_err!(InstNode::borrow(structor_ref.clone()), '__try3);
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    Ok(node)
}

fn componentClass(
    mut component: &metamodelica::Ref<SCode::Element>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: &Walk,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut site: Option<Site>;
    let () = (::match_deref::match_deref! { match component {
        Deref @ SCode::Element::COMPONENT { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __esc_path, .. }, .. } => {
            path = (*__esc_path).clone();
            site = suspendRecording(walk);
            node = walkPath(metamodelica::AsArg::as_arg(&path), scope, walk, metamodelica::AsArg::as_arg(&path), 1)?;
            leaveSite(site, walk);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(node)
}

fn baseClasses(
    mut cls: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>> {
    let mut bases: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
    let mut path: metamodelica::Ref<Absyn::Path>;
    let () = (match &*(InstNode::getClass(cls.clone())?) {
        Class::EXPANDED_DERIVED { baseClass: base, .. } => {
            bases = list![base.clone()];
            ()
        }
        _ => {
            if SCodeUtil::isClassExtends(&(InstNode::definition(cls.clone())?)) {
                bases = inheritedClasses(InstNode::name(&cls)?, InstNode::parent(&cls)?)?.reverse();
            }
            if '__try0: {
                let __range1 = ClassTree::getExtends(&(unwrap_break_err!(Class::classTree(unwrap_break_err!(InstNode::getClass(cls.clone()), '__try0)), '__try0))).borrow().iter().cloned().collect::<Vec<_>>();
                for mut ext in __range1 {
                    let () = (match &*(unwrap_break_err!(InstNode::definition(ext.clone()), '__try0)) {
        SCode::Element::EXTENDS { baseClassPath: __esc_path, .. } => {
            path = (*__esc_path).clone();
            ext = unwrap_break_err!(findClassPath(path.clone(), unwrap_break_err!(InstNode::parent(&cls), '__try0)), '__try0);
            ()
        },
        _ => (),
    });
                    if unwrap_break_err!(InstNode::isClass(&ext), '__try0) {
                        bases = metamodelica::cons(ext.clone(), bases.clone());
                    }
                }
                Ok::<(), &'static str>(())
            }.is_err() {
            }
            bases = bases.reverse();
            ()
        }
    });
    Ok(bases)
}

fn inheritedClasses(
    mut name: ArcStr,
    mut cls: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>> {
    let mut classes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
    let mut n: metamodelica::Ref<InstNode::InstNode>;
    if !(InstNode::isClass(&cls)?) {
        return Ok(classes);
    }
    for mut b in &*baseClasses(expandNode(cls)?)? {
        n = findClass(name.clone(), b.clone())?;
        if !(InstNode::isEmpty(&n)) {
            classes = metamodelica::cons(n, classes);
        }
    }
    Ok(classes)
}

fn redeclaredClass(
    mut name: &ArcStr,
    mut cls: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    for mut e in &*SCodeUtil::getClassElements(&(InstNode::definition(cls.clone())?)) {
        if SCodeUtil::elementIsClass(metamodelica::AsArg::as_arg(&e))
            && isReplacingClass(metamodelica::AsArg::as_arg(&e))?
            && metamodelica::stringEq(&(SCodeUtil::elementName(metamodelica::AsArg::as_arg(&e))?), &name)
        {
            node = localClass(metamodelica::AsArg::as_arg(&e), cls.clone())?;
            if !(InstNode::isClass(&node)?
                && metamodelica::stringEq(
                    &(elementKey(&(InstNode::definition(node.clone())?))?),
                    &(elementKey(metamodelica::AsArg::as_arg(&e))?),
                ))
            {
                node = InstNode::newClass(e.clone(), cls, crate::NFInstNode::InstNodeType::interned_NORMAL_CLASS())?;
            }
            return Ok(node);
        }
    }
    Ok(node)
}

fn replacedClasses(
    mut cls: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>> {
    let mut classes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
    if isReplacingClass(&(InstNode::definition(cls.clone())?))? {
        for mut r in &*inheritedClasses(InstNode::name(&cls)?, InstNode::parent(&cls)?)? {
            classes = listAppend(metamodelica::cons(r.clone(), replacedClasses(r.clone())?), classes);
        }
    }
    Ok(classes)
}

fn isReplacingClass(mut element: &metamodelica::Ref<SCode::Element>) -> Result<bool> {
    let mut res: bool = SCodeUtil::isClassExtends(element) || SCodeUtil::isElementRedeclare(element)?;
    Ok(res)
}

fn findClass(
    mut name: ArcStr,
    mut cls: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    let mut c: metamodelica::Ref<InstNode::InstNode> = expandNode(cls.clone())?;
    node = redeclaredClass(&name, c.clone())?;
    if InstNode::isClass(&node)? {
        return Ok(node);
    }
    if '__try0: {
        (node, _) = unwrap_break_err!(Lookup::lookupLocalSimpleName(name.clone(), c.clone()), '__try0);
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    if !(InstNode::isClass(&node)?) {
        node = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
        for mut b in &*baseClasses(c)? {
            node = findClass(name.clone(), b.clone())?;
            if !(InstNode::isEmpty(&node)) {
                return Ok(node);
            }
        }
    }
    Ok(node)
}

fn findClassPath(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    let mut cur: metamodelica::Ref<InstNode::InstNode> = scope.clone();
    let mut first: ArcStr = AbsynUtil::pathFirstIdent(&path);
    if AbsynUtil::pathIsFullyQualified(&path) {
        cur = InstNode::topScope(scope.clone())?;
    } else {
        while InstNode::isClass(&cur)? && !(InstNode::isTopScope(&cur)) {
            node = findClass(first.clone(), cur.clone())?;
            if InstNode::isClass(&node)? {
                break;
            } else if metamodelica::stringEq(&first, &(InstNode::name(&cur)?)) {
                node = cur;
                break;
            } else if InstNode::isEncapsulated(cur.clone())? {
                break;
            }
            cur = InstNode::parentScope(cur, false)?;
        }
    }
    if !(InstNode::isClass(&node)?) {
        if let Ok(__iflet0) =
            Lookup::lookupSimpleName(first.clone(), InstNode::topScope(scope.clone())?, CONTEXT.clone())
        {
            node = __iflet0.0;
        } else {
            node = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
            return Ok(node);
        }
    }
    for mut id in &*(AbsynUtil::pathToStringList(&(AbsynUtil::makeNotFullyQualified(path)))).rest()? {
        node = findClass(id.clone(), node)?;
        if !(InstNode::isClass(&node)?) {
            return Ok(node);
        }
    }
    Ok(node)
}

fn lookupFirstIdent(
    mut name: ArcStr,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
    mut needType: bool,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    let mut cur: metamodelica::Ref<InstNode::InstNode> = scope.clone();
    let mut found: bool;
    while InstNode::isClass(&cur)? && !(InstNode::isTopScope(&cur)) {
        (node, found) = findElement(name.clone(), cur.clone(), walk.clone(), needType)?;
        if found {
            return Ok(node);
        }
        if metamodelica::stringEq(&name, &(InstNode::name(&cur)?)) {
            node = cur.clone();
            setFound(Found::FOUND_NODE { node: cur }, &walk);
            return Ok(node);
        }
        if InstNode::isEncapsulated(cur.clone())? {
            break;
        }
        cur = InstNode::parentScope(cur, false)?;
    }
    if '__try0: {
        (node, _) = unwrap_break_err!(Lookup::lookupSimpleName(name.clone(), unwrap_break_err!(InstNode::topScope(scope.clone()), '__try0), CONTEXT.clone()), '__try0);
        setFound(Found::FOUND_NODE { node: node.clone() }, &walk);
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    Ok(node)
}

fn isBuiltinName(mut name: &ArcStr) -> bool {
    let mut builtin: bool;
    match '__try0: {
        unwrap_break_err!(Lookup::lookupSimpleBuiltinName(name), '__try0);
        builtin = true;
        Ok::<_, &'static str>((builtin.clone(),))
    } {
        Ok((__try0_o0,)) => {
            builtin = __try0_o0;
        }
        Err(_) => {
            builtin = false;
        }
    }
    builtin
}

fn expandNode(mut node: metamodelica::Ref<InstNode::InstNode>) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut scopes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
    let mut n: metamodelica::Ref<InstNode::InstNode>;
    let () = (match &*(InstNode::getClass(node.clone())?) {
        Class::NOT_INSTANTIATED => {
            n = InstNode::parent(&node)?;
            while InstNode::isClass(&n)? && !(InstNode::isTopScope(&n)) {
                scopes = metamodelica::cons(n.clone(), scopes);
                n = InstNode::parent(&n)?;
            }
            for mut s in &*scopes {
                if '__try0: {
                    unwrap_break_err!(Inst::instPackage(s.clone(), CONTEXT.clone()), '__try0);
                    Ok::<(), &'static str>(())
                }
                .is_err()
                {}
            }
            ()
        }
        _ => (),
    });
    if '__try0: {
        node = unwrap_break_err!(Inst::expand(node.clone(), CONTEXT.clone()), '__try0);
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    Ok(node)
}

fn expandScopes(mut node: metamodelica::Ref<InstNode::InstNode>) -> Result<()> {
    let mut n: metamodelica::Ref<InstNode::InstNode> = node;
    while InstNode::isClass(&n)? && !(InstNode::isTopScope(&n)) {
        expandNode(n.clone())?;
        n = InstNode::parent(&n)?;
    }
    Ok(())
}

fn walkQueue(mut walk: Walk) -> () {
    let mut queue: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    while !((Pointer::access(walk.queue.clone())).is_empty()) {
        queue = Pointer::access(walk.queue.clone());
        Pointer::update(walk.queue.clone(), metamodelica::nil());
        for mut c in &*queue {
            if '__try0: {
                unwrap_break_err!(walkClass(c.clone(), walk.clone(), false), '__try0);
                Ok::<(), &'static str>(())
            }
            .is_err()
            {}
        }
    }
    ()
}

fn foundNode(mut node: metamodelica::Ref<InstNode::InstNode>, mut walk: Walk) -> Result<()> {
    let mut n: metamodelica::Ref<InstNode::InstNode> = node;
    let mut key: ArcStr;
    while InstNode::isClass(&n)? || InstNode::isComponent(&n)? {
        if InstNode::isTopScope(&n) || InstNode::isBuiltin(&n) {
            break;
        }
        match '__try0: {
            key =
                unwrap_break_err!(elementKey(&(unwrap_break_err!(InstNode::definition(n.clone()), '__try0))), '__try0);
            Ok::<_, &'static str>((key.clone(),))
        } {
            Ok((__try0_o0,)) => {
                key = __try0_o0;
            }
            Err(_) => {
                key = literal!("");
            }
        }
        if stringEmpty(&key) {
            break;
        }
        if UnorderedSet::contains(key.clone(), walk.used.clone())? || isRecording(&walk) {
            break;
        }
        UnorderedSet::add(key.clone(), walk.used.clone())?;
        if InstNode::isClass(&n)? {
            Pointer::update(
                walk.queue.clone(),
                metamodelica::cons(n.clone(), Pointer::access(walk.queue.clone())),
            );
        } else {
            walkPackageConstant(n.clone(), walk.clone())?;
        }
        n = InstNode::parent(&n)?;
    }
    Ok(())
}

fn walkPackageConstant(mut node: metamodelica::Ref<InstNode::InstNode>, mut walk: Walk) -> Result<()> {
    let mut scope: metamodelica::Ref<InstNode::InstNode> = InstNode::parent(&node)?;
    let mut def: metamodelica::Ref<SCode::Element> = InstNode::definition(node.clone())?;
    if InstNode::isClass(&scope)?
        && SCodeUtil::isPackage(&(InstNode::definition(scope.clone())?))
        && !(UnorderedSet::contains(elementKey(&def)?, walk.walked.clone())?)
    {
        UnorderedSet::add(elementKey(&def)?, walk.walked.clone())?;
        walkElement(&def, scope, walk, false)?;
    }
    Ok(())
}

fn walkClass(mut node: metamodelica::Ref<InstNode::InstNode>, mut walk: Walk, mut allConstants: bool) -> Result<()> {
    let mut cls: metamodelica::Ref<InstNode::InstNode> = node.clone();
    let mut scope: metamodelica::Ref<InstNode::InstNode>;
    let mut op: metamodelica::Ref<InstNode::InstNode>;
    let mut replacement: metamodelica::Ref<InstNode::InstNode>;
    let mut def: metamodelica::Ref<SCode::Element> = InstNode::definition(node.clone())?;
    let mut is_operator: bool;
    let mut key: ArcStr = elementKey(&def)?;
    let mut is_package: bool;
    let mut site: Option<Site> = None;
    if UnorderedSet::contains(key.clone(), walk.walked.clone())? {
        return Ok(());
    }
    UnorderedSet::add(key, walk.walked.clone())?;
    Pointer::update(
        walk.classes.clone(),
        metamodelica::cons(node.clone(), Pointer::access(walk.classes.clone())),
    );
    expandScopes(InstNode::parent(&node)?)?;
    cls = expandNode(node.clone())?;
    scope = InstNode::parent(&cls)?;
    if isRecording(&walk) {
        declareClass(&def, &walk)?;
        site = enterSite(SCodeUtil::elementInfo(&def), literal!("annotation"), &walk);
    }
    walkAnnotation(&def, &cls, &walk)?;
    if SCodeUtil::isClassExtends(&def) {
        UnorderedSet::add(InstNode::name(&node)?, walk.names.clone())?;
        setRole(literal!("classExtends"), &walk)?;
        for mut r in &*inheritedClasses(InstNode::name(&node)?, InstNode::parent(&node)?)? {
            recordFound(
                &(Found::FOUND_NODE { node: r.clone() }),
                metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: InstNode::name(&node)?,
                }),
                1,
                &walk,
            )?;
        }
    }
    if SCodeUtil::elementIsClass(&def) && SCodeUtil::isElementRedeclare(&def)? {
        replacement = (match &*(InstNode::getClass(cls.clone())?) {
            Class::EXPANDED_DERIVED {
                baseClass: __esc_replacement,
                ..
            } => {
                replacement = (*__esc_replacement).clone();
                replacement.clone()
            }
            _ => cls.clone(),
        });
        setRole(literal!("redeclare"), &walk)?;
        for mut r in &*inheritedClasses(InstNode::name(&node)?, InstNode::parent(&node)?)? {
            if !(SCodeUtil::isClassExtends(&def)) {
                recordFound(
                    &(Found::FOUND_NODE { node: r.clone() }),
                    metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: InstNode::name(&node)?,
                    }),
                    1,
                    &walk,
                )?;
            }
            for mut rr in &*metamodelica::cons(r.clone(), replacedClasses(r.clone())?) {
                redeclared(rr.clone(), replacement.clone(), walk.clone())?;
            }
        }
    }
    is_operator = SCodeUtil::isOperatorRecord(&def) || SCodeUtil::isOperator(&def);
    for mut e in &*SCodeUtil::getClassElements(&def) {
        if SCodeUtil::elementIsClass(metamodelica::AsArg::as_arg(&e))
            && (is_operator
                && (SCodeUtil::isOperator(metamodelica::AsArg::as_arg(&e))
                    || SCodeUtil::isFunction(metamodelica::AsArg::as_arg(&e)))
                || listMember(
                    SCodeUtil::elementName(metamodelica::AsArg::as_arg(&e))?,
                    list![
                        literal!("equalityConstraint"),
                        literal!("constructor"),
                        literal!("destructor")
                    ],
                ))
        {
            op = localClass(metamodelica::AsArg::as_arg(&e), cls.clone())?;
            if InstNode::isClass(&op)? {
                foundNode(op, walk.clone())?;
            }
        }
    }
    setRole(literal!("extends"), &walk)?;
    let () = (match &*def.clone() {
        SCode::Element::CLASS {
            classDef: __def_classDef,
            ..
        } => {
            is_package = SCodeUtil::isPackage(&def) && !(allConstants);
            walkClassDef(
                metamodelica::AsArg::as_arg(&__def_classDef),
                &cls,
                &scope,
                is_package,
                &walk,
            )?;
            ()
        }
        _ => (),
    });
    leaveSite(site, &walk);
    Ok(())
}

fn declareClass(mut def: &metamodelica::Ref<SCode::Element>, mut walk: &Walk) -> Result<()> {
    let mut name: ArcStr;
    let () = (match walk.recorder.clone() {
        Some(mut rec) => {
            name = Pointer::access(rec.scope.clone());
            declare(def, name.clone(), walk);
            for mut e in &*SCodeUtil::getClassElements(def) {
                if SCodeUtil::isComponent(metamodelica::AsArg::as_arg(&e)) {
                    declare(
                        metamodelica::AsArg::as_arg(&e),
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*name);
                            __mm_s.push_str(&*literal!("."));
                            __mm_s.push_str(&*SCodeUtil::elementName(metamodelica::AsArg::as_arg(&e))?);
                            ArcStr::from(__mm_s)
                        },
                        walk,
                    );
                }
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

fn walkClassDef(
    mut classDef: &metamodelica::Ref<SCode::ClassDef>,
    mut cls: &metamodelica::Ref<InstNode::InstNode>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut isPackage: bool,
    mut walk: &Walk,
) -> Result<()> {
    let mut stmts: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    let mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut dims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let mut target: metamodelica::Ref<InstNode::InstNode>;
    let mut ext_output: Option<metamodelica::Ref<Absyn::ComponentRef>>;
    let () = (match &**classDef {
        SCode::ClassDef::PARTS {
            constraintLst: __classDef_constraintLst,
            elementLst: __classDef_elementLst,
            externalDecl: __classDef_externalDecl,
            initialAlgorithmLst: __classDef_initialAlgorithmLst,
            initialEquationLst: __classDef_initialEquationLst,
            normalAlgorithmLst: __classDef_normalAlgorithmLst,
            normalEquationLst: __classDef_normalEquationLst,
            ..
        } => {
            for mut e in &*__classDef_elementLst.clone() {
                if !(isPackage && SCodeUtil::isComponent(metamodelica::AsArg::as_arg(&e))) {
                    if '__try0: {
                        unwrap_break_err!(walkElement(metamodelica::AsArg::as_arg(&e), cls.clone(), walk.clone(), false), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_err() {
                    }
                }
            }
            for mut eq in &*listAppend(
                __classDef_normalEquationLst.clone(),
                __classDef_initialEquationLst.clone(),
            ) {
                walkEquation(metamodelica::AsArg::as_arg(&eq), cls, walk.clone())?;
            }
            for mut alg in &*listAppend(
                __classDef_normalAlgorithmLst.clone(),
                __classDef_initialAlgorithmLst.clone(),
            ) {
                let __arc2 = alg.clone();
                let SCode::ALGORITHM { statements: __pa1 } = &*__arc2;
                stmts = metamodelica::Own::own(__pa1);
                for mut stmt in &*stmts {
                    if isRecording(walk) {
                        walkStatement(stmt.clone(), cls, walk)?;
                    } else {
                        SCodeUtil::foldStatementsExps(
                            metamodelica::AsArg::as_arg(&stmt),
                            (std::sync::Arc::new({
                                let __pe_b1 = cls.clone();
                                move |__pe_a0, __pe_a2| walkExp(__pe_a0, &__pe_b1, __pe_a2)
                            })
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Walk) -> Result<Walk> + 'static,
                                >),
                            walk.clone(),
                        )?;
                    }
                }
            }
            setRole(literal!("external"), walk)?;
            for mut c in &*__classDef_constraintLst.clone() {
                let SCode::CONSTRAINTS { constraints: __pa3 } = &c;
                exps = metamodelica::Own::own(__pa3);
                List::fold(
                    &exps,
                    &({
                        let __pe_b1 = cls.clone();
                        move |__pe_a0, __pe_a2| walkExp(__pe_a0, &__pe_b1, __pe_a2)
                    }),
                    walk.clone(),
                )?;
            }
            let () = (::match_deref::match_deref! { match &(__classDef_externalDecl.clone()) {
                Some(Deref @ SCode::ExternalDecl { args: __esc_exps, output_: __esc_ext_output, .. }) => {
                    exps = (*__esc_exps).clone();
                    ext_output = (*__esc_ext_output).clone();
                    if (ext_output).is_some() {
                        walkCref(&(Util::getOption(ext_output.clone())?), cls, walk.clone(), true)?;
                    }
                    List::fold(metamodelica::AsArg::as_arg(&exps), &({ let __pe_b1 = cls.clone(); move |__pe_a0, __pe_a2| walkExp(__pe_a0, &__pe_b1, __pe_a2) }), walk.clone())?;
                    ()
                },
                _ => (),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            ()
        }
        SCode::ClassDef::CLASS_EXTENDS {
            composition: __classDef_composition,
            modifications: __classDef_modifications,
        } => {
            walkMod(
                metamodelica::AsArg::as_arg(&__classDef_modifications),
                cls,
                walk,
                &(crate::NFInstNode::InstNode::interned_EMPTY_NODE()),
            )?;
            walkClassDef(
                metamodelica::AsArg::as_arg(&__classDef_composition),
                cls,
                scope,
                isPackage,
                walk,
            )?;
            ()
        }
        SCode::ClassDef::DERIVED {
            attributes: __classDef_attributes,
            modifications: __classDef_modifications,
            typeSpec: __classDef_typeSpec,
        } => {
            target = walkTypeSpec(metamodelica::AsArg::as_arg(&__classDef_typeSpec), scope, walk.clone())?;
            walkMod(
                metamodelica::AsArg::as_arg(&__classDef_modifications),
                scope,
                walk,
                &target,
            )?;
            let SCode::ATTR { arrayDims: __pa0, .. } = &__classDef_attributes;
            dims = metamodelica::Own::own(__pa0);
            walkDims(&dims, scope, walk.clone())?;
            ()
        }
        SCode::ClassDef::OVERLOAD {
            pathLst: __classDef_pathLst,
        } => {
            for mut p in &*__classDef_pathLst.clone() {
                walkPath(
                    metamodelica::AsArg::as_arg(&p),
                    scope,
                    walk,
                    metamodelica::AsArg::as_arg(&p),
                    1,
                )?;
            }
            ()
        }
        SCode::ClassDef::PDER {
            functionPath: __classDef_functionPath,
            ..
        } => {
            walkPath(
                metamodelica::AsArg::as_arg(&__classDef_functionPath),
                scope,
                walk,
                metamodelica::AsArg::as_arg(&__classDef_functionPath),
                1,
            )?;
            ()
        }
        _ => (),
    });
    Ok(())
}

fn walkElement(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
    mut inModifier: bool,
) -> Result<()> {
    let mut inherited: Found = crate::NFUsedElements::Found::NOT_FOUND;
    let mut dims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut target: metamodelica::Ref<InstNode::InstNode>;
    let mut site: Option<Site>;
    let mut groups: metamodelica::List<Absyn::GroupImport>;
    site = enterSite(SCodeUtil::elementInfo(element), literal!("type"), &walk);
    let () = (match &**element {
        SCode::Element::COMPONENT {
            attributes: __element_attributes,
            condition: __element_condition,
            modifications: __element_modifications,
            prefixes: __element_prefixes,
            typeSpec: __element_typeSpec,
            ..
        } => {
            if isRecording(&walk) && SCodeUtil::isElementRedeclare(element)? && !(inModifier) {
                inherited = recordInherited(SCodeUtil::elementName(element)?, scope.clone(), walk.clone())?;
                setRole(literal!("type"), &walk)?;
            }
            target = walkTypeSpec(metamodelica::AsArg::as_arg(&__element_typeSpec), &scope, walk.clone())?;
            if !(stringEmpty(&(replaceableComponentKey(&inherited)))) {
                redeclaredKey(replaceableComponentKey(&inherited), target.clone(), walk.clone())?;
            }
            setRole(literal!("dimension"), &walk)?;
            let SCode::ATTR { arrayDims: __pa0, .. } = &__element_attributes;
            dims = metamodelica::Own::own(__pa0);
            walkDims(&dims, &scope, walk.clone())?;
            setRole(literal!("type"), &walk)?;
            walkMod(
                metamodelica::AsArg::as_arg(&__element_modifications),
                &scope,
                &walk,
                &target,
            )?;
            if (__element_condition).is_some() {
                setRole(literal!("condition"), &walk)?;
                walkExp(Util::getOption(__element_condition.clone())?, &scope, walk.clone())?;
            }
            setRole(literal!("constrainedby"), &walk)?;
            walkConstrainingClass(metamodelica::AsArg::as_arg(&__element_prefixes), &scope, &walk)?;
            ()
        }
        SCode::Element::EXTENDS {
            baseClassPath: __element_baseClassPath,
            modifications: __element_modifications,
            ..
        } => {
            setRole(literal!("extends"), &walk)?;
            target = walkPath(
                metamodelica::AsArg::as_arg(&__element_baseClassPath),
                &scope,
                &walk,
                metamodelica::AsArg::as_arg(&__element_baseClassPath),
                1,
            )?;
            walkMod(
                metamodelica::AsArg::as_arg(&__element_modifications),
                &scope,
                &walk,
                &target,
            )?;
            ()
        }
        SCode::Element::IMPORT {
            imp: Absyn::Import::UNQUAL_IMPORT { path: __esc_path },
            ..
        } => {
            path = (*__esc_path).clone();
            setRole(literal!("import"), &walk)?;
            walkPath(
                &(metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: path.clone() })),
                &scope,
                &walk,
                metamodelica::AsArg::as_arg(&path),
                1,
            )?;
            ()
        }
        SCode::Element::IMPORT { imp: __element_imp, .. } if (isRecording(&walk)) => {
            setRole(literal!("import"), &walk)?;
            let () = (match __element_imp.clone() {
                Absyn::Import::QUAL_IMPORT { path: mut __esc_path } => {
                    path = __esc_path.clone();
                    walkPath(
                        &(metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: path.clone() })),
                        &scope,
                        &walk,
                        metamodelica::AsArg::as_arg(&path),
                        1,
                    )?;
                    ()
                }
                Absyn::Import::NAMED_IMPORT {
                    path: mut __esc_path, ..
                } => {
                    path = __esc_path.clone();
                    walkPath(
                        &(metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: path.clone() })),
                        &scope,
                        &walk,
                        metamodelica::AsArg::as_arg(&path),
                        1,
                    )?;
                    ()
                }
                Absyn::Import::GROUP_IMPORT {
                    prefix: ref __esc_path,
                    groups: mut __esc_groups,
                } => {
                    path = __esc_path.clone();
                    groups = __esc_groups.clone();
                    target = walkPath(
                        &(metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: path.clone() })),
                        &scope,
                        &walk,
                        metamodelica::AsArg::as_arg(&path),
                        1,
                    )?;
                    if InstNode::isClass(&target)? {
                        for mut g in &*groups.clone() {
                            let () = (match g.clone() {
                                Absyn::GroupImport::GROUP_IMPORT_NAME { .. } => {
                                    walkRest(
                                        metamodelica::Ref::new(Absyn::Path::IDENT {
                                            name: var_field!(g.name, Absyn::GroupImport::GROUP_IMPORT_NAME).clone(),
                                        }),
                                        target.clone(),
                                        walk.clone(),
                                        metamodelica::Ref::new(Absyn::Path::IDENT {
                                            name: var_field!(g.name, Absyn::GroupImport::GROUP_IMPORT_NAME).clone(),
                                        }),
                                        1,
                                    )?;
                                    ()
                                }
                                Absyn::GroupImport::GROUP_IMPORT_RENAME { .. } => {
                                    walkRest(
                                        metamodelica::Ref::new(Absyn::Path::IDENT {
                                            name: var_field!(g.name, Absyn::GroupImport::GROUP_IMPORT_RENAME).clone(),
                                        }),
                                        target.clone(),
                                        walk.clone(),
                                        metamodelica::Ref::new(Absyn::Path::IDENT {
                                            name: var_field!(g.name, Absyn::GroupImport::GROUP_IMPORT_RENAME).clone(),
                                        }),
                                        1,
                                    )?;
                                    ()
                                }
                                _ => (),
                            });
                        }
                    }
                    ()
                }
                _ => (),
            });
            ()
        }
        _ => (),
    });
    leaveSite(site, &walk);
    Ok(())
}

fn recordInherited(mut name: ArcStr, mut cls: metamodelica::Ref<InstNode::InstNode>, mut walk: Walk) -> Result<Found> {
    let mut found: Found = crate::NFUsedElements::Found::NOT_FOUND;
    let mut is_found: bool;
    for mut b in &*baseClasses(expandNode(cls)?)? {
        takeFound(&walk);
        (_, is_found) = findElement(name.clone(), b.clone(), walk.clone(), false)?;
        if is_found {
            found = takeFound(&walk);
            recordFound(
                &found,
                metamodelica::Ref::new(Absyn::Path::IDENT { name: name }),
                1,
                &walk,
            )?;
            return Ok(found);
        }
    }
    Ok(found)
}

fn walkNamedElements(mut cls: metamodelica::Ref<InstNode::InstNode>, mut walk: Walk) -> Result<()> {
    let mut def: metamodelica::Ref<SCode::Element> = InstNode::definition(cls.clone())?;
    let mut c: metamodelica::Ref<InstNode::InstNode> = expandNode(cls.clone())?;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    for mut e in &*SCodeUtil::getClassElements(&def) {
        if (SCodeUtil::elementIsClass(metamodelica::AsArg::as_arg(&e))
            || SCodeUtil::isComponent(metamodelica::AsArg::as_arg(&e)))
            && (SCodeUtil::isElementReplaceable(metamodelica::AsArg::as_arg(&e))?
                || SCodeUtil::isElementRedeclare(metamodelica::AsArg::as_arg(&e))?
                || SCodeUtil::isClassExtends(metamodelica::AsArg::as_arg(&e)))
            && UnorderedSet::contains(
                SCodeUtil::elementName(metamodelica::AsArg::as_arg(&e))?,
                walk.names.clone(),
            )?
            && !(UnorderedSet::contains(elementKey(metamodelica::AsArg::as_arg(&e))?, walk.used.clone())?)
        {
            if SCodeUtil::isComponent(metamodelica::AsArg::as_arg(&e)) {
                foundComponent(metamodelica::AsArg::as_arg(&e), c.clone(), walk.clone())?;
            } else if SCodeUtil::elementIsClass(metamodelica::AsArg::as_arg(&e)) {
                node = localClass(metamodelica::AsArg::as_arg(&e), c.clone())?;
                if InstNode::isClass(&node)? {
                    foundNode(node, walk.clone())?;
                }
            }
        }
    }
    Ok(())
}

fn walkConstrainingClass(
    mut prefixes: &metamodelica::Ref<SCode::Prefixes>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: &Walk,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match prefixes {
        Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(cc @ Deref @ SCode::ConstrainClass { .. }) }, .. } => {
            walkMod(&cc.modifier, scope, walk, &(walkPath(&cc.constrainingClass, scope, walk, &cc.constrainingClass, 1)?))?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn walkAnnotation(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: &Walk,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match element {
        Deref @ SCode::Element::CLASS { cmt: Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: r#mod @ Deref @ SCode::Mod::MOD { .. } }), .. }, .. } => {
            for mut sm in &*var_field!((**r#mod).subModLst, SCode::Mod::MOD).clone() {
                if metamodelica::stringEq(&sm.ident, &(literal!("derivative"))) || metamodelica::stringEq(&sm.ident, &(literal!("inverse"))) {
                    walkMod(&sm.r#mod, scope, walk, &(crate::NFInstNode::InstNode::interned_EMPTY_NODE()))?;
                }
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

fn walkMod(
    mut r#mod: &metamodelica::Ref<SCode::Mod>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: &Walk,
    mut target: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<()> {
    let mut cdef: metamodelica::Ref<SCode::ClassDef>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut site: Option<Site>;
    let mut dims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let mut found: Found;
    let mut elem: metamodelica::Ref<SCode::Element>;
    let () = (match &**r#mod {
        SCode::Mod::MOD {
            binding: __mod_binding,
            subModLst: __mod_subModLst,
            ..
        } => {
            for mut sm in &*__mod_subModLst.clone() {
                takeFound(walk);
                node = foundLocalName(sm.ident.clone(), target.clone(), walk.clone(), hasSubMods(&sm.r#mod))?;
                found = takeFound(walk);
                if InstNode::isClass(target)? {
                    site = setRole(literal!("modifier"), walk)?;
                    lookedUpThrough(
                        target.clone(),
                        metamodelica::Ref::new(Absyn::Path::IDENT { name: sm.ident.clone() }),
                        walk.clone(),
                        metamodelica::Ref::new(Absyn::Path::IDENT { name: sm.ident.clone() }),
                        1,
                    )?;
                    recordFound(
                        &found,
                        metamodelica::Ref::new(Absyn::Path::IDENT { name: sm.ident.clone() }),
                        1,
                        walk,
                    )?;
                    leaveSite(site, walk);
                }
                let () = (::match_deref::match_deref! { match &(sm.r#mod.clone()) {
                    Deref @ SCode::Mod::REDECL { element: __esc_elem @ Deref @ SCode::Element::COMPONENT { .. }, .. } if (isRecording(walk) && !(stringEmpty(&(replaceableComponentKey(&found))))) => {
                        elem = (*__esc_elem).clone();
                        redeclaredKey(replaceableComponentKey(&found), componentClass(metamodelica::AsArg::as_arg(&elem), scope, walk)?, walk.clone())?;
                        ()
                    },
                    _ => (),
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                walkMod(
                    &sm.r#mod,
                    scope,
                    walk,
                    &(if (InstNode::isClass(&node)?) {
                        node
                    } else {
                        crate::NFInstNode::InstNode::interned_EMPTY_NODE()
                    }),
                )?;
            }
            if (__mod_binding).is_some() {
                site = setRole(literal!("binding"), walk)?;
                walkExp(Util::getOption(__mod_binding.clone())?, scope, walk.clone())?;
                leaveSite(site, walk);
            }
            ()
        }
        SCode::Mod::REDECL {
            element: __mod_element, ..
        } => {
            site = setRole(literal!("type"), walk)?;
            let () = (::match_deref::match_deref! { match &(__mod_element.clone()) {
                Deref @ SCode::Element::CLASS { classDef: __esc_cdef @ Deref @ SCode::ClassDef::DERIVED { .. }, .. } => {
                    cdef = (*__esc_cdef).clone();
                    node = walkTypeSpec(var_field!((*cdef).typeSpec, SCode::ClassDef::DERIVED), scope, walk.clone())?;
                    redeclared(target.clone(), node.clone(), walk.clone())?;
                    walkMod(var_field!((*cdef).modifications, SCode::ClassDef::DERIVED), scope, walk, &node)?;
                    let SCode::ATTR { arrayDims: __pa0, .. } = var_field!((*cdef).attributes, SCode::ClassDef::DERIVED).clone();
                    dims = metamodelica::Own::own(__pa0);
                    walkDims(&dims, scope, walk.clone())?;
                    ()
                },
                Deref @ SCode::Element::CLASS { classDef: __esc_cdef, .. } => {
                    cdef = (*__esc_cdef).clone();
                    walkClassDef(metamodelica::AsArg::as_arg(&cdef), scope, scope, false, walk)?;
                    ()
                },
                _ => {
                    walkElement(metamodelica::AsArg::as_arg(&__mod_element), scope.clone(), walk.clone(), true)?;
                    ()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            leaveSite(site, walk);
            ()
        }
        _ => (),
    });
    Ok(())
}

fn hasSubMods(mut r#mod: &metamodelica::Ref<SCode::Mod>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match r#mod {
        Deref @ SCode::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn walkEquation(
    mut eq: &metamodelica::Ref<SCode::Equation>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
) -> Result<()> {
    let mut site: Option<Site>;
    if !(isRecording(&walk)) {
        SCodeUtil::foldEquationsExps(
            eq,
            (std::sync::Arc::new({
                let __pe_b1 = scope.clone();
                move |__pe_a0, __pe_a2| walkExp(__pe_a0, &__pe_b1, __pe_a2)
            })
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Walk) -> Result<Walk> + 'static>),
            walk,
        )?;
        return Ok(());
    }
    site = enterSite(SCodeUtil::getEquationInfo(eq), literal!("equation"), &walk);
    let () = (match &**eq {
        SCode::Equation::EQ_CONNECT {
            crefLeft: __eq_crefLeft,
            crefRight: __eq_crefRight,
            ..
        } => {
            walkCref(metamodelica::AsArg::as_arg(&__eq_crefLeft), scope, walk.clone(), true)?;
            walkCref(metamodelica::AsArg::as_arg(&__eq_crefRight), scope, walk.clone(), true)?;
            ()
        }
        _ => {
            SCodeUtil::mapEquationExps(
                eq.clone(),
                &({
                    let __pe_b1 = scope.clone();
                    let __pe_b2 = walk.clone();
                    move |__pe_a0| walkExpMap(__pe_a0, &__pe_b1, __pe_b2.clone())
                }),
            )?;
            ()
        }
    });
    let () = (match &**eq {
        SCode::Equation::EQ_IF {
            elseBranch: __eq_elseBranch,
            thenBranch: __eq_thenBranch,
            ..
        } => {
            for mut branch in &*__eq_thenBranch.clone() {
                for mut e in &*branch.clone() {
                    walkEquation(metamodelica::AsArg::as_arg(&e), scope, walk.clone())?;
                }
            }
            for mut e in &*__eq_elseBranch.clone() {
                walkEquation(metamodelica::AsArg::as_arg(&e), scope, walk.clone())?;
            }
            ()
        }
        SCode::Equation::EQ_FOR {
            eEquationLst: __eq_eEquationLst,
            index: __eq_index,
            ..
        } => {
            pushIterator(__eq_index.clone(), &walk)?;
            for mut e in &*__eq_eEquationLst.clone() {
                walkEquation(metamodelica::AsArg::as_arg(&e), scope, walk.clone())?;
            }
            popIterator(&walk)?;
            ()
        }
        SCode::Equation::EQ_WHEN {
            eEquationLst: __eq_eEquationLst,
            elseBranches: __eq_elseBranches,
            ..
        } => {
            for mut e in &*__eq_eEquationLst.clone() {
                walkEquation(metamodelica::AsArg::as_arg(&e), scope, walk.clone())?;
            }
            for mut b in &*__eq_elseBranches.clone() {
                for mut e in &*Util::tuple22(b.clone()) {
                    walkEquation(metamodelica::AsArg::as_arg(&e), scope, walk.clone())?;
                }
            }
            ()
        }
        _ => (),
    });
    leaveSite(site, &walk);
    Ok(())
}

fn walkStatement(
    mut stmt: metamodelica::Ref<SCode::Statement>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: &Walk,
) -> Result<()> {
    let mut site: Option<Site>;
    site = enterSite(SCodeUtil::getStatementInfo(&stmt)?, literal!("algorithm"), walk);
    SCodeUtil::mapStatementExps(
        stmt.clone(),
        &({
            let __pe_b1 = scope.clone();
            let __pe_b2 = walk.clone();
            move |__pe_a0| walkExpMap(__pe_a0, &__pe_b1, __pe_b2.clone())
        }),
    )?;
    let () = (match &*stmt {
        SCode::Statement::ALG_IF {
            elseBranch: __stmt_elseBranch,
            elseIfBranch: __stmt_elseIfBranch,
            trueBranch: __stmt_trueBranch,
            ..
        } => {
            walkStatements(metamodelica::AsArg::as_arg(&__stmt_trueBranch), scope, walk)?;
            for mut b in &*__stmt_elseIfBranch.clone() {
                walkStatements(&(Util::tuple22(b.clone())), scope, walk)?;
            }
            walkStatements(metamodelica::AsArg::as_arg(&__stmt_elseBranch), scope, walk)?;
            ()
        }
        SCode::Statement::ALG_FOR {
            forBody: __stmt_forBody,
            index: __stmt_index,
            ..
        } => {
            pushIterator(__stmt_index.clone(), walk)?;
            walkStatements(metamodelica::AsArg::as_arg(&__stmt_forBody), scope, walk)?;
            popIterator(walk)?;
            ()
        }
        SCode::Statement::ALG_PARFOR {
            index: __stmt_index,
            parforBody: __stmt_parforBody,
            ..
        } => {
            pushIterator(__stmt_index.clone(), walk)?;
            walkStatements(metamodelica::AsArg::as_arg(&__stmt_parforBody), scope, walk)?;
            popIterator(walk)?;
            ()
        }
        SCode::Statement::ALG_WHILE {
            whileBody: __stmt_whileBody,
            ..
        } => {
            walkStatements(metamodelica::AsArg::as_arg(&__stmt_whileBody), scope, walk)?;
            ()
        }
        SCode::Statement::ALG_WHEN_A {
            branches: __stmt_branches,
            ..
        } => {
            for mut b in &*__stmt_branches.clone() {
                walkStatements(&(Util::tuple22(b.clone())), scope, walk)?;
            }
            ()
        }
        SCode::Statement::ALG_FAILURE {
            stmts: __stmt_stmts, ..
        } => {
            walkStatements(metamodelica::AsArg::as_arg(&__stmt_stmts), scope, walk)?;
            ()
        }
        SCode::Statement::ALG_TRY {
            body: __stmt_body,
            elseBody: __stmt_elseBody,
            ..
        } => {
            walkStatements(metamodelica::AsArg::as_arg(&__stmt_body), scope, walk)?;
            walkStatements(metamodelica::AsArg::as_arg(&__stmt_elseBody), scope, walk)?;
            ()
        }
        _ => (),
    });
    leaveSite(site, walk);
    Ok(())
}

fn walkStatements(
    mut stmts: &metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: &Walk,
) -> Result<()> {
    for mut s in &**stmts {
        walkStatement(s.clone(), scope, walk)?;
    }
    Ok(())
}

fn walkDims(
    mut dims: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
) -> Result<()> {
    for mut d in &**dims {
        let () = (match &*d.clone() {
            Absyn::Subscript::SUBSCRIPT {
                subscript: __d_subscript,
            } => {
                walkExp(__d_subscript.clone(), scope, walk.clone())?;
                ()
            }
            _ => (),
        });
    }
    Ok(())
}

fn walkTypeSpec(
    mut ty: &metamodelica::Ref<Absyn::TypeSpec>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    let () = (match &**ty {
        Absyn::TypeSpec::TPATH {
            arrayDim: __ty_arrayDim,
            path: __ty_path,
        } => {
            node = walkPath(
                metamodelica::AsArg::as_arg(&__ty_path),
                scope,
                &walk,
                metamodelica::AsArg::as_arg(&__ty_path),
                1,
            )?;
            if (__ty_arrayDim).is_some() {
                walkDims(&(Util::getOption(__ty_arrayDim.clone())?), scope, walk)?;
            }
            ()
        }
        _ => (),
    });
    Ok(node)
}

fn walkExp(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
) -> Result<Walk> {
    let mut walk: Walk = walk;
    if isRecording(&walk) {
        AbsynUtil::traverseExpBidir(
            exp,
            (std::sync::Arc::new({
                let __pe_b1 = scope.clone();
                move |__pe_a0, __pe_a2| recordExpEnter(__pe_a0, &__pe_b1, __pe_a2)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Absyn::Exp>,
                            Walk,
                        ) -> Result<(metamodelica::Ref<Absyn::Exp>, Walk)>
                        + 'static,
                >),
            (std::sync::Arc::new(recordExpExit)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Absyn::Exp>,
                            Walk,
                        ) -> Result<(metamodelica::Ref<Absyn::Exp>, Walk)>
                        + 'static,
                >),
            walk.clone(),
        )?;
    } else {
        AbsynUtil::traverseExp(
            exp,
            (std::sync::Arc::new({
                let __pe_b1 = scope.clone();
                move |__pe_a0, __pe_a2| walkExpNode(__pe_a0, &__pe_b1, __pe_a2)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Absyn::Exp>,
                            Walk,
                        ) -> Result<(metamodelica::Ref<Absyn::Exp>, Walk)>
                        + 'static,
                >),
            walk.clone(),
        )?;
    }
    Ok(walk)
}

fn walkExpMap(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    walkExp(exp.clone(), scope, walk)?;
    Ok(exp)
}

fn recordExpEnter(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
) -> Result<(metamodelica::Ref<Absyn::Exp>, Walk)> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut walk: Walk = walk;
    let mut r#fn: metamodelica::Ref<InstNode::InstNode>;
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
    let mut iters: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>;
    let () = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "AssertionLevel", .. } } } => (),
        Deref @ Absyn::Exp::CREF { componentRef: __exp_componentRef } => {
            walkCref(metamodelica::AsArg::as_arg(&__exp_componentRef), scope, walk.clone(), false)?;
            ()
        },
        Deref @ Absyn::Exp::CALL { functionArgs: __exp_functionArgs, function_: __exp_function_, .. } => {
            r#fn = walkCref(metamodelica::AsArg::as_arg(&__exp_function_), scope, walk.clone(), true)?;
            let () = (match &*__exp_functionArgs.clone() {
        Absyn::FunctionArgs::FUNCTIONARGS { argNames: __esc_args, .. } => {
            args = (*__esc_args).clone();
            recordNamedArgs(metamodelica::AsArg::as_arg(&args), r#fn, walk.clone())?;
            ()
        },
        Absyn::FunctionArgs::FOR_ITER_FARG { iterators: __esc_iters, .. } => {
            iters = (*__esc_iters).clone();
            for mut it in &*iters.clone() {
                pushIterator(it.name.clone(), &walk)?;
            }
            ()
        },
        _ => (),
    });
            ()
        },
        Deref @ Absyn::Exp::PARTEVALFUNCTION { functionArgs: __exp_functionArgs, function_: __exp_function_ } => {
            r#fn = walkCref(metamodelica::AsArg::as_arg(&__exp_function_), scope, walk.clone(), true)?;
            let () = (match &*__exp_functionArgs.clone() {
        Absyn::FunctionArgs::FUNCTIONARGS { argNames: __esc_args, .. } => {
            args = (*__esc_args).clone();
            recordNamedArgs(metamodelica::AsArg::as_arg(&args), r#fn, walk.clone())?;
            ()
        },
        _ => (),
    });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, walk))
}

fn recordExpExit(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut walk: Walk,
) -> Result<(metamodelica::Ref<Absyn::Exp>, Walk)> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut walk: Walk = walk;
    let mut iters: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>;
    let () = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { iterators: __esc_iters, .. }, .. } => {
            iters = (*__esc_iters).clone();
            for mut it in &*iters.clone() {
                popIterator(&walk)?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, walk))
}

fn recordNamedArgs(
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut r#fn: metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
) -> Result<()> {
    let mut site: Option<Site>;
    let mut f: metamodelica::Ref<InstNode::InstNode> = r#fn.clone();
    let mut constructor: metamodelica::Ref<InstNode::InstNode>;
    if (args).is_empty() || !(InstNode::isClass(&r#fn)?) || InstNode::isBuiltin(&r#fn) {
        return Ok(());
    }
    for mut e in &*SCodeUtil::getClassElements(&(InstNode::definition(r#fn.clone())?)) {
        if SCodeUtil::elementIsClass(metamodelica::AsArg::as_arg(&e))
            && metamodelica::stringEq(
                &(SCodeUtil::elementName(metamodelica::AsArg::as_arg(&e))?),
                &(literal!("constructor")),
            )
        {
            constructor = localClass(metamodelica::AsArg::as_arg(&e), expandNode(r#fn.clone())?)?;
            if InstNode::isClass(&constructor)? {
                f = constructor;
            }
            break;
        }
    }
    site = setRole(literal!("argument"), &walk)?;
    for mut a in &**args {
        lookedUpThrough(
            f.clone(),
            metamodelica::Ref::new(Absyn::Path::IDENT {
                name: a.argName.clone(),
            }),
            walk.clone(),
            metamodelica::Ref::new(Absyn::Path::IDENT {
                name: a.argName.clone(),
            }),
            1,
        )?;
        takeFound(&walk);
        foundLocalName(a.argName.clone(), f.clone(), walk.clone(), false)?;
        recordFound(
            &(takeFound(&walk)),
            metamodelica::Ref::new(Absyn::Path::IDENT {
                name: a.argName.clone(),
            }),
            1,
            &(walk.clone()),
        )?;
    }
    leaveSite(site, &walk);
    Ok(())
}

fn walkExpNode(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
) -> Result<(metamodelica::Ref<Absyn::Exp>, Walk)> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut walk: Walk = walk;
    let () = (match &*exp {
        Absyn::Exp::CREF {
            componentRef: __exp_componentRef,
        } => {
            walkCref(
                metamodelica::AsArg::as_arg(&__exp_componentRef),
                scope,
                walk.clone(),
                true,
            )?;
            ()
        }
        Absyn::Exp::CALL {
            function_: __exp_function_,
            ..
        } => {
            walkCref(metamodelica::AsArg::as_arg(&__exp_function_), scope, walk.clone(), true)?;
            ()
        }
        Absyn::Exp::PARTEVALFUNCTION {
            function_: __exp_function_,
            ..
        } => {
            walkCref(metamodelica::AsArg::as_arg(&__exp_function_), scope, walk.clone(), true)?;
            ()
        }
        _ => (),
    });
    Ok((exp, walk))
}

fn walkCref(
    mut cref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
    mut walkSubscripts: bool,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    if '__try0: {
        node = unwrap_break_err!(walkPath(&(unwrap_break_err!(AbsynUtil::crefToPathIgnoreSubs(cref), '__try0)), scope, &walk, &(unwrap_break_err!(AbsynUtil::crefToPathIgnoreSubs(cref), '__try0)), 1), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    if !(walkSubscripts) {
        return Ok(node);
    }
    for mut s in &*AbsynUtil::getSubsFromCref(cref, true, true)? {
        let () = (match &*s.clone() {
            Absyn::Subscript::SUBSCRIPT {
                subscript: __s_subscript,
            } => {
                walkExp(__s_subscript.clone(), scope, walk.clone())?;
                ()
            }
            _ => (),
        });
    }
    Ok(node)
}

fn walkPath(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut walk: &Walk,
    mut written: &metamodelica::Ref<Absyn::Path>,
    mut index: i32,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
    let mut n: metamodelica::Ref<InstNode::InstNode>;
    let mut found: Found;
    let mut comp_key: ArcStr;
    let () = (match &**path {
        Absyn::Path::FULLYQUALIFIED { path: __path_path } => {
            node = walkPath(
                metamodelica::AsArg::as_arg(&__path_path),
                &(InstNode::topScope(scope.clone())?),
                walk,
                written,
                index,
            )?;
            ()
        }
        _ => {
            if isRecording(walk)
                && !(AbsynUtil::pathIsFullyQualified(written))
                && recordIterator(&(AbsynUtil::pathFirstIdent(path)), written.clone(), walk)?
            {
                return Ok(node);
            }
            if isBuiltinName(&(AbsynUtil::pathFirstIdent(path)))
                || metamodelica::stringEq(&(AbsynUtil::pathFirstIdent(path)), &(literal!("time")))
                || stringGet(&(AbsynUtil::pathFirstIdent(path)), 1)? == 36
            {
                return Ok(node);
            }
            for mut id in &*AbsynUtil::pathToStringList(path) {
                UnorderedSet::add(id.clone(), walk.names.clone())?;
            }
            takeFound(walk);
            n = lookupFirstIdent(
                AbsynUtil::pathFirstIdent(path),
                scope.clone(),
                walk.clone(),
                AbsynUtil::pathIsQual(path),
            )?;
            found = takeFound(walk);
            recordFound(&found, written.clone(), index, walk)?;
            comp_key = replaceableComponentKey(&found);
            if isRecording(walk) && AbsynUtil::pathIsQual(path) && !(stringEmpty(&comp_key)) {
                lookedUpThroughRecording(
                    comp_key,
                    AbsynUtil::pathRest(path.clone())?,
                    written.clone(),
                    index + 1,
                    walk.clone(),
                )?;
            }
            if InstNode::isEmpty(&n) {
                return Ok(node);
            }
            foundNode(n.clone(), walk.clone())?;
            if AbsynUtil::pathIsQual(path) {
                node = walkRest(
                    AbsynUtil::pathRest(path.clone())?,
                    n,
                    walk.clone(),
                    written.clone(),
                    index + 1,
                )?;
            } else {
                node = n;
            }
            ()
        }
    });
    Ok(node)
}

fn walkRest(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut cls: metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
    mut written: metamodelica::Ref<Absyn::Path>,
    mut index: i32,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = cls;
    let mut rest: metamodelica::Ref<Absyn::Path> = path;
    let mut i: i32 = index;
    let mut searched: bool;
    let mut found: Found;
    let mut comp_key: ArcStr;
    loop {
        lookedUpThrough(node.clone(), rest.clone(), walk.clone(), written.clone(), i)?;
        searched = InstNode::isClass(&node)?;
        takeFound(&walk);
        node = foundLocalName(
            AbsynUtil::pathFirstIdent(&rest),
            node,
            walk.clone(),
            AbsynUtil::pathIsQual(&rest),
        )?;
        found = takeFound(&walk);
        if searched {
            recordFound(&found, written.clone(), i, &walk)?;
        }
        comp_key = replaceableComponentKey(&found);
        if isRecording(&walk) && AbsynUtil::pathIsQual(&rest) && !(stringEmpty(&comp_key)) {
            lookedUpThroughRecording(
                comp_key,
                AbsynUtil::pathRest(rest.clone())?,
                written.clone(),
                i + 1,
                walk.clone(),
            )?;
        }
        if InstNode::isEmpty(&node) || !(AbsynUtil::pathIsQual(&rest)) {
            break;
        }
        rest = AbsynUtil::pathRest(rest)?;
        i = i + 1;
    }
    Ok(node)
}

fn lookedUpThrough(
    mut cls: metamodelica::Ref<InstNode::InstNode>,
    mut rest: metamodelica::Ref<Absyn::Path>,
    mut walk: Walk,
    mut written: metamodelica::Ref<Absyn::Path>,
    mut index: i32,
) -> Result<()> {
    let mut key: ArcStr;
    let mut rests: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut replacements: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    if !(InstNode::isClass(&cls)? && SCodeUtil::isElementReplaceable(&(InstNode::definition(cls.clone())?))?) {
        return Ok(());
    }
    key = elementKey(&(InstNode::definition(cls)?))?;
    if isRecording(&walk) {
        lookedUpThroughRecording(key, rest, written, index, walk)?;
        return Ok(());
    }
    rests = UnorderedMap::getOrDefault(key.clone(), walk.rests.clone(), metamodelica::nil())?;
    if !(List::isMemberOnTrue(rest.clone(), &rests, &move |__a0: metamodelica::Ref<Absyn::Path>,
                                                           __a1: metamodelica::Ref<Absyn::Path>|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
    })?) {
        UnorderedMap::add(key.clone(), metamodelica::cons(rest.clone(), rests), walk.rests.clone())?;
        replacements = UnorderedMap::getOrDefault(key, walk.replacements.clone(), metamodelica::nil())?;
        for mut r in &*replacements {
            walkRest(rest.clone(), r.clone(), walk.clone(), rest.clone(), 1)?;
        }
    }
    Ok(())
}

fn redeclared(
    mut replaced: metamodelica::Ref<InstNode::InstNode>,
    mut replacement: metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
) -> Result<()> {
    if InstNode::isClass(&replaced)? && InstNode::isClass(&replacement)? {
        redeclaredKey(elementKey(&(InstNode::definition(replaced)?))?, replacement, walk)?;
    }
    Ok(())
}

fn redeclaredKey(
    mut key: ArcStr,
    mut replacement: metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
) -> Result<()> {
    let mut replacements: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut rests: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut pending: metamodelica::List<Pending>;
    if !(InstNode::isClass(&replacement)?) {
        return Ok(());
    }
    replacements = UnorderedMap::getOrDefault(key.clone(), walk.replacements.clone(), metamodelica::nil())?;
    if !(List::isMemberOnTrue(replacement.clone(), &replacements, &move |__a0: metamodelica::Ref<
        InstNode::InstNode,
    >,
                                                                         __a1: metamodelica::Ref<
        InstNode::InstNode,
    >| {
        InstNode::refEqual(&__a0, &__a1)
    })?) {
        UnorderedMap::add(
            key.clone(),
            metamodelica::cons(replacement.clone(), replacements),
            walk.replacements.clone(),
        )?;
        let () = (match walk.recorder.clone() {
            Some(mut rec) => {
                pending = UnorderedMap::getOrDefault(key, rec.pending.clone(), metamodelica::nil())?;
                for mut p in &*pending {
                    walkPending(metamodelica::AsArg::as_arg(&p), replacement.clone(), walk.clone())?;
                }
                ()
            }
            _ => {
                rests = UnorderedMap::getOrDefault(key, walk.rests.clone(), metamodelica::nil())?;
                for mut rest in &*rests {
                    walkRest(rest.clone(), replacement.clone(), walk.clone(), rest.clone(), 1)?;
                }
                ()
            }
        });
    }
    Ok(())
}

fn replaceableComponentKey(mut found: &Found) -> ArcStr {
    let mut key: ArcStr = literal!("");
    let mut e: metamodelica::Ref<SCode::Element>;
    if '__try0: {
        e = (match found.clone() {
        Found::FOUND_ELEMENT { element: ref __esc_e, .. } => {
            e = __esc_e.clone();
            e.clone()
        },
        Found::FOUND_NODE { .. } if (unwrap_break_err!(InstNode::isComponent(var_field!(found.node, Found::FOUND_NODE)), '__try0)) => unwrap_break_err!(InstNode::definition(var_field!(found.node, Found::FOUND_NODE).clone()), '__try0),
        _ => break '__try0 Err::<_, _>("match: no arm matched"),
    });
        if SCodeUtil::isComponent(&e) && unwrap_break_err!(SCodeUtil::isElementReplaceable(&e), '__try0) {
            key = unwrap_break_err!(elementKey(&e), '__try0);
        }
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    key
}

fn lookedUpThroughRecording(
    mut key: ArcStr,
    mut rest: metamodelica::Ref<Absyn::Path>,
    mut written: metamodelica::Ref<Absyn::Path>,
    mut index: i32,
    mut walk: Walk,
) -> Result<()> {
    let mut site: Site;
    let mut p: Pending;
    let mut pkey: ArcStr;
    let mut replacements: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let () = (match walk.recorder.clone() {
        Some(mut rec) if ((Pointer::access(rec.site.clone())).is_some()) => {
            let __pa0 = ::match_deref::match_deref! { match &(Pointer::access(rec.site.clone())) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            site = metamodelica::Own::own(__pa0);
            pkey = stringAppendList(list![
                key.clone(),
                literal!("|"),
                AbsynUtil::pathString(written.clone(), literal!("."), true, false)?,
                literal!("|"),
                intString(index),
                literal!("|"),
                site.info.fileName.clone(),
                literal!(":"),
                intString(site.info.lineNumberStart.clone()),
                literal!(":"),
                intString(site.info.columnNumberStart.clone())
            ]);
            if !(UnorderedSet::contains(pkey.clone(), rec.pendingKeys.clone())?) {
                UnorderedSet::add(pkey, rec.pendingKeys.clone())?;
                p = Pending {
                    rest: rest,
                    written: written,
                    index: index,
                    site: site,
                };
                UnorderedMap::add(
                    key.clone(),
                    metamodelica::cons(
                        p.clone(),
                        UnorderedMap::getOrDefault(key.clone(), rec.pending.clone(), metamodelica::nil())?,
                    ),
                    rec.pending.clone(),
                )?;
                replacements = UnorderedMap::getOrDefault(key, walk.replacements.clone(), metamodelica::nil())?;
                for mut r in &*replacements {
                    walkPending(&p, r.clone(), walk.clone())?;
                }
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

fn walkPending(
    mut pending: &Pending,
    mut replacement: metamodelica::Ref<InstNode::InstNode>,
    mut walk: Walk,
) -> Result<()> {
    let mut site: Option<Site>;
    let mut candidate: bool;
    let () = (match walk.recorder.clone() {
        Some(mut rec) => {
            site = Pointer::access(rec.site.clone());
            candidate = Pointer::access(rec.candidate.clone());
            Pointer::update(rec.site.clone(), Some(pending.site.clone()));
            Pointer::update(rec.candidate.clone(), true);
            walkRest(
                pending.rest.clone(),
                replacement,
                walk,
                pending.written.clone(),
                pending.index.clone(),
            )?;
            Pointer::update(rec.site.clone(), site);
            Pointer::update(rec.candidate.clone(), candidate);
            ()
        }
        _ => (),
    });
    Ok(())
}
