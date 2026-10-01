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
use crate::NFUsedElements::Site;
use crate::NFUsedElements::Use;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::JSON;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub(crate) const IDENT: i32 = 1;

pub(crate) const DOT: i32 = 2;

pub(crate) const LBRACKET: i32 = 3;

pub(crate) const RBRACKET: i32 = 4;

pub(crate) const LBRACE: i32 = 5;

pub(crate) const RBRACE: i32 = 6;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Token {
    pub kind: i32,
    pub text: ArcStr,
    pub line: i32,
    /// The column of the first character, starting at 1.
    pub column: i32,
    /// The column of the last character.
    pub endColumn: i32,
}

impl metamodelica::gc::MMTrace for Token {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.kind, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.text, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.line, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.column, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.endColumn, __mmv)?;
        Ok(())
    }
}
impl Default for Token {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            text: Default::default(),
            line: Default::default(),
            column: Default::default(),
            endColumn: Default::default(),
        }
    }
}

pub type TOKEN = Token;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SourceFile {
    pub tokens: metamodelica::Array<Token>,
    pub taken: metamodelica::Array<bool>,
}

impl metamodelica::gc::MMTrace for SourceFile {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.tokens, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.taken, __mmv)?;
        Ok(())
    }
}
impl Default for SourceFile {
    fn default() -> Self {
        Self {
            tokens: Default::default(),
            taken: Default::default(),
        }
    }
}

pub type SOURCE_FILE = SourceFile;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Position {
    pub fileName: ArcStr,
    pub lineStart: i32,
    pub columnStart: i32,
    pub lineEnd: i32,
    pub columnEnd: i32,
    pub exact: bool,
}

impl metamodelica::gc::MMTrace for Position {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.fileName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lineStart, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.columnStart, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lineEnd, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.columnEnd, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.exact, __mmv)?;
        Ok(())
    }
}
impl Default for Position {
    fn default() -> Self {
        Self {
            fileName: Default::default(),
            lineStart: Default::default(),
            columnStart: Default::default(),
            lineEnd: Default::default(),
            columnEnd: Default::default(),
            exact: Default::default(),
        }
    }
}

pub type POSITION = Position;

pub type OptSourceFile = Option<SourceFile>;

pub type Files = metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Option<SourceFile>>>;

pub type Placed = (Use, Position);

pub type PlacedList = metamodelica::List<(Use, Position)>;

pub type OptLines = Option<metamodelica::Array<ArcStr>>;

pub type InfoList = metamodelica::List<SourceInfo>;

pub type StringSet = metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>;

// The moduli of the hashes of the source of a class, primes small enough that
// h * 263 + 255 stays below 2^30, so the hashes are the same on 32-bit targets
// (wasm) as on 64-bit ones.
pub(crate) const HASH_MOD1: i32 = 4082651;

pub(crate) const HASH_MOD2: i32 = 4082629;

pub fn toJSON(
    mut className: ArcStr,
    mut definitions: metamodelica::List<Definition>,
    mut uses: metamodelica::List<Use>,
    mut unresolved: metamodelica::List<Use>,
    mut scope: &ArcStr,
    mut prettyPrint: bool,
) -> Result<ArcStr> {
    let mut json: ArcStr;
    let mut defs: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Definition>>;
    let mut def_uses: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::List<(Use, Position)>>>;
    let mut names: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Position>>;
    let mut reported: metamodelica::List<Definition> = metamodelica::nil();
    let mut reported_uses: metamodelica::List<Use>;
    let mut placed: metamodelica::List<(Use, Position)>;
    let mut files: Files;
    let mut resolved: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>;
    let mut single: bool;
    let mut def: Definition;
    let mut jdefs: metamodelica::Ref<JSON::JSON>;
    let mut jdef: metamodelica::Ref<JSON::JSON>;
    let mut juses: metamodelica::Ref<JSON::JSON>;
    let mut obj: metamodelica::Ref<JSON::JSON>;
    let mut jlist: metamodelica::List<metamodelica::Ref<JSON::JSON>>;
    let mut u: Use = <Use as ::std::default::Default>::default();
    let mut pos: Position;
    defs = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    for mut d in &*definitions {
        UnorderedMap::add(d.key.clone(), d.clone(), defs.clone())?;
    }
    single = false;
    for mut d in &*definitions {
        if metamodelica::stringEq(&d.name, &className) && !metamodelica::stringEq(&d.kind, &(literal!("class"))) {
            reported = list![d.clone()];
            single = true;
            break;
        }
    }
    if !(single) {
        reported = ({
            let mut __acc: metamodelica::List<Definition> = metamodelica::nil();
            for mut d in (definitions.clone()).into_iter().cloned() {
                if !(d.declared.clone() && isInClass(d.name.clone(), &className)?) {
                    continue;
                }
                let __x = d.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        for mut u in &*uses {
            let mut u = u.clone();
            if isInClass(u.site.scope.clone(), &className)? {
                def = UnorderedMap::getOrFail(u.key.clone(), defs.clone())?;
                if !(def.declared.clone() && isInClass(def.name.clone(), &className)?) {
                    reported = metamodelica::cons(def, reported);
                }
            }
        }
        reported = uniqueDefinitions(&reported)?;
    }
    reported = List::sort(
        reported,
        (std::sync::Arc::new(move |__a0: Definition, __a1: Definition| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(definitionGt(&__a0, &__a1))
        }) as std::sync::Arc<dyn ::std::ops::Fn(Definition, Definition) -> Result<bool> + 'static>),
    )?;
    def_uses = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    for mut d in &*reported {
        UnorderedMap::add(d.key.clone(), metamodelica::nil(), def_uses.clone())?;
    }
    reported_uses = ({
        let mut __acc: metamodelica::List<Use> = metamodelica::nil();
        for mut u in (uses.clone()).into_iter().cloned() {
            if !(UnorderedMap::contains(u.key.clone(), def_uses.clone())?
                && (stringEmpty(&scope)
                    || isInClass(u.site.scope.clone(), scope)?
                    || isInClass(u.site.scope.clone(), &className)?))
            {
                continue;
            }
            let __x = u.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    files = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    names = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    for mut d in &*definitions {
        if UnorderedMap::contains(d.info.fileName.clone(), files.clone())?
            || UnorderedMap::contains(d.key.clone(), def_uses.clone())?
            || List::any(
                &reported_uses,
                &({
                    let __pe_b1 = d.info.fileName.clone();
                    move |__pe_a0| Ok(useInFile(&__pe_a0, __pe_b1.clone()))
                }),
            )?
        {
            UnorderedMap::add(
                d.key.clone(),
                placeDefinition(metamodelica::AsArg::as_arg(&d), files.clone())?,
                names.clone(),
            )?;
        }
    }
    placed = placeUses(&reported_uses, defs.clone(), files.clone())?;
    for mut p in &*placed {
        (u, _) = p.clone();
        UnorderedMap::add(
            u.key.clone(),
            metamodelica::cons(p.clone(), UnorderedMap::getOrFail(u.key.clone(), def_uses.clone())?),
            def_uses.clone(),
        )?;
    }
    for mut d in &*reported {
        if metamodelica::stringEq(&d.kind, &(literal!("class"))) {
            pos = placeEnd(metamodelica::AsArg::as_arg(&d), files.clone());
            if pos.exact.clone() {
                u = Use {
                    key: d.key.clone(),
                    written: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: lastIdent(d.name.clone())?,
                    }),
                    index: 1,
                    site: Site {
                        info: d.info.clone(),
                        role: literal!("end"),
                        scope: d.name.clone(),
                    },
                    candidate: false,
                };
                UnorderedMap::add(
                    d.key.clone(),
                    metamodelica::cons((u, pos), UnorderedMap::getOrFail(d.key.clone(), def_uses.clone())?),
                    def_uses.clone(),
                )?;
            }
        }
    }
    jlist = metamodelica::nil();
    for mut d in &*reported {
        jdef = definitionJSON(
            metamodelica::AsArg::as_arg(&d),
            &(UnorderedMap::getOrDefault(
                d.key.clone(),
                names.clone(),
                Position {
                    fileName: literal!(""),
                    lineStart: 0,
                    columnStart: 0,
                    lineEnd: 0,
                    columnEnd: 0,
                    exact: false,
                },
            )?),
        )?;
        juses = JSON::makeArray(
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<JSON::JSON>> = metamodelica::nil();
                for mut p in (List::sort(
                    UnorderedMap::getOrFail(d.key.clone(), def_uses.clone())?,
                    (std::sync::Arc::new(
                        move |__a0: (Use, Position), __a1: (Use, Position)| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(placedGt(&__a0, &__a1))
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn((Use, Position), (Use, Position)) -> Result<bool> + 'static,
                        >),
                )?)
                .into_iter()
                .cloned()
                {
                    let __x = useJSON(&(p.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        );
        jdef = JSON::addPair(&(literal!("uses")), &juses, jdef)?;
        jlist = metamodelica::cons(jdef, jlist);
    }
    obj = JSON::emptyListObject();
    obj = JSON::addPair(&(literal!("class")), &(JSON::makeString(className.clone())), obj)?;
    obj = JSON::addPair(&(literal!("definitions")), &(JSON::makeArray(jlist.reverse())), obj)?;
    resolved = UnorderedSet::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        13,
    );
    for mut u in &*uses {
        let mut u = u.clone();
        if u.candidate.clone() {
            UnorderedSet::add(siteSignature(&u)?, resolved.clone())?;
        }
    }
    obj = JSON::addPair(
        &(literal!("unresolved")),
        &(JSON::makeArray(
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<JSON::JSON>> = metamodelica::nil();
                for mut p in (placeUses(
                    &({
                        let mut __acc: metamodelica::List<Use> = metamodelica::nil();
                        for mut u in (unresolved).into_iter().cloned() {
                            if !(isInClass(u.site.scope.clone(), &className)?
                                && !(UnorderedSet::contains(siteSignature(&u)?, resolved.clone())?))
                            {
                                continue;
                            }
                            let __x = u.clone();
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    defs,
                    files,
                )?)
                .into_iter()
                .cloned()
                {
                    let __x = useJSON(&(p.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )),
        obj,
    )?;
    json = JSON::toString(&obj, prettyPrint)?;
    Ok(json)
}

pub fn dependencyGraphJSON(
    mut scope: ArcStr,
    mut definitions: &metamodelica::List<Definition>,
    mut uses: &metamodelica::List<Use>,
    mut prettyPrint: bool,
) -> Result<ArcStr> {
    let mut json: ArcStr;
    let mut defs: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Definition>> = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    let mut classes: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Definition>> = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    let mut deps: metamodelica::Ref<
        UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>>,
    > = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    let mut nested: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::List<SourceInfo>>> =
        UnorderedMap::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            1,
        );
    let mut texts: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Option<metamodelica::Array<ArcStr>>>> =
        UnorderedMap::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            1,
        );
    let mut def: Definition;
    let mut to: ArcStr;
    let mut parent: ArcStr;
    let mut used: StringSet;
    let mut jclasses: metamodelica::Ref<JSON::JSON> = JSON::emptyListObject();
    let mut jcls: metamodelica::Ref<JSON::JSON>;
    let mut obj: metamodelica::Ref<JSON::JSON>;
    for mut d in &**definitions {
        UnorderedMap::add(d.key.clone(), d.clone(), defs.clone())?;
        if metamodelica::stringEq(&d.kind, &(literal!("class")))
            && (d.declared.clone() || !(UnorderedMap::contains(d.name.clone(), classes.clone())?))
        {
            UnorderedMap::add(d.name.clone(), d.clone(), classes.clone())?;
        }
    }
    for mut d in &*UnorderedMap::valueList(classes.clone()) {
        parent = ownerName(d.name.clone())?;
        if UnorderedMap::contains(parent.clone(), classes.clone())? {
            def = UnorderedMap::getOrFail(parent.clone(), classes.clone())?;
            if def.info.fileName.clone() == d.info.fileName.clone() {
                UnorderedMap::add(
                    parent.clone(),
                    metamodelica::cons(
                        d.info.clone(),
                        UnorderedMap::getOrDefault(parent, nested.clone(), metamodelica::nil())?,
                    ),
                    nested.clone(),
                )?;
            }
        }
    }
    for mut u in &**uses {
        if !(stringEmpty(&u.key)) {
            def = UnorderedMap::getOrFail(u.key.clone(), defs.clone())?;
            if metamodelica::stringEq(&def.kind, &(literal!("iterator")))
                || metamodelica::stringEq(&def.kind, &(literal!("class")))
                    && u.index.clone()
                        < ((AbsynUtil::pathToStringList(&(AbsynUtil::makeNotFullyQualified(u.written.clone())))).len()
                            as i32)
            {
                continue;
            }
            to = if (metamodelica::stringEq(&def.kind, &(literal!("class")))) {
                def.name.clone()
            } else {
                ownerName(def.name.clone())?
            };
            if !(stringEmpty(&to)) && !metamodelica::stringEq(&to, &u.site.scope) {
                used = UnorderedMap::addUpdate(
                    u.site.scope.clone(),
                    &fnptr!(
                        newStringSet,
                        Option<metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>>
                    ),
                    deps.clone(),
                )?;
                UnorderedSet::add(to, used)?;
            }
        }
    }
    for mut d in &*List::sort(
        UnorderedMap::valueList(classes.clone()),
        (std::sync::Arc::new(move |__a0: Definition, __a1: Definition| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(definitionGt(&__a0, &__a1))
        }) as std::sync::Arc<dyn ::std::ops::Fn(Definition, Definition) -> Result<bool> + 'static>),
    )? {
        jcls = JSON::emptyListObject();
        jcls = JSON::addPair(&(literal!("restriction")), &(JSON::makeString(d.detail.clone())), jcls)?;
        jcls = addInfo(&d.info, jcls)?;
        jcls = JSON::addPair(
            &(literal!("hash")),
            &(JSON::makeString(sourceHash(
                &d.info,
                &(List::sort(
                    UnorderedMap::getOrDefault(d.name.clone(), nested.clone(), metamodelica::nil())?,
                    (std::sync::Arc::new(move |__a0: SourceInfo, __a1: SourceInfo| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(infoGt(&__a0, &__a1))
                    })
                        as std::sync::Arc<dyn ::std::ops::Fn(SourceInfo, SourceInfo) -> Result<bool> + 'static>),
                )?),
                texts.clone(),
            )?)),
            jcls,
        )?;
        jcls = JSON::addPair(
            &(literal!("uses")),
            &(JSON::makeArray(
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<JSON::JSON>> = metamodelica::nil();
                    for mut n in (List::sort(
                        UnorderedSet::toList(UnorderedMap::getOrDefault(
                            d.name.clone(),
                            deps.clone(),
                            newStringSet(None),
                        )?),
                        (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(stringGt(&__a0, &__a1))
                        })
                            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
                    )?)
                    .into_iter()
                    .cloned()
                    {
                        if !(UnorderedMap::contains(n.clone(), classes.clone())?) {
                            continue;
                        }
                        let __x = JSON::makeString(n.clone());
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )),
            jcls,
        )?;
        jclasses = JSON::addPair(&d.name, &jcls, jclasses)?;
    }
    obj = JSON::emptyListObject();
    obj = JSON::addPair(&(literal!("scope")), &(JSON::makeString(scope)), obj)?;
    obj = JSON::addPair(&(literal!("classes")), &jclasses, obj)?;
    json = JSON::toString(&obj, prettyPrint)?;
    Ok(json)
}

pub fn definitionAtJSON(
    mut fileName: ArcStr,
    mut line: i32,
    mut column: i32,
    mut definitions: &metamodelica::List<Definition>,
    mut uses: metamodelica::List<Use>,
    mut prettyPrint: bool,
) -> Result<ArcStr> {
    let mut json: ArcStr;
    let mut defs: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Definition>> = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    let mut files: Files = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    let mut found: Option<Definition> = None;
    let mut found_use: Option<(Use, Position)> = None;
    let mut names: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Position>> = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    let mut candidates: metamodelica::List<Definition> = metamodelica::nil();
    let mut pos: Position;
    let mut u: Use = <Use as ::std::default::Default>::default();
    let mut def: Definition;
    let mut obj: metamodelica::Ref<JSON::JSON>;
    let mut jdef: metamodelica::Ref<JSON::JSON>;
    for mut d in &**definitions {
        UnorderedMap::add(d.key.clone(), d.clone(), defs.clone())?;
    }
    for mut d in &**definitions {
        if d.info.fileName.clone() == fileName.clone() && isInSpan(&d.info, line, column) {
            pos = placeDefinition(metamodelica::AsArg::as_arg(&d), files.clone())?;
            UnorderedMap::add(d.key.clone(), pos.clone(), names.clone())?;
            if isAt(&pos, &fileName, line, column) {
                found = Some(d.clone());
            } else if metamodelica::stringEq(&d.kind, &(literal!("class"))) {
                pos = placeEnd(metamodelica::AsArg::as_arg(&d), files.clone());
                if isAt(&pos, &fileName, line, column) {
                    found = Some(d.clone());
                }
            }
        }
    }
    if (found).is_none() {
        for mut p in &*placeUses(
            &({
                let mut __acc: metamodelica::List<Use> = metamodelica::nil();
                for mut u in (uses).into_iter().cloned() {
                    if !(u.site.info.fileName.clone() == fileName.clone() && isInSpan(&u.site.info, line, column)) {
                        continue;
                    }
                    let __x = u.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            defs.clone(),
            files.clone(),
        )? {
            (u, pos) = p.clone();
            if isAt(&pos, &fileName, line, column) {
                def = UnorderedMap::getOrFail(u.key.clone(), defs.clone())?;
                if u.candidate.clone() {
                    candidates = metamodelica::cons(def.clone(), candidates);
                    if (found_use).is_none() {
                        found_use = Some(p.clone());
                    }
                } else if (found).is_none() {
                    found = Some(def.clone());
                    found_use = Some(p.clone());
                }
            }
        }
    }
    obj = JSON::emptyListObject();
    obj = JSON::addPair(&(literal!("file")), &(JSON::makeString(fileName)), obj)?;
    obj = JSON::addPair(&(literal!("line")), &(JSON::makeInteger(line)), obj)?;
    obj = JSON::addPair(&(literal!("column")), &(JSON::makeInteger(column)), obj)?;
    obj = JSON::addPair(
        &(literal!("definition")),
        &(match found {
            Some(mut __esc_def) => {
                def = __esc_def.clone();
                definitionJSON(&(def.clone()), &(namePosition(&def, names.clone(), files.clone())?))?
            }
            _ => JSON::makeNull(),
        }),
        obj,
    )?;
    obj = JSON::addPair(
        &(literal!("use")),
        &(match found_use {
            Some(mut placed) => useJSON(&placed)?,
            _ => JSON::makeNull(),
        }),
        obj,
    )?;
    obj = JSON::addPair(
        &(literal!("candidates")),
        &(JSON::makeArray(
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<JSON::JSON>> = metamodelica::nil();
                for mut c in (candidates.reverse()).into_iter().cloned() {
                    let __x = definitionJSON(
                        &(c.clone()),
                        &(namePosition(&(c.clone()), names.clone(), files.clone())?),
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )),
        obj,
    )?;
    json = JSON::toString(&obj, prettyPrint)?;
    Ok(json)
}

fn definitionJSON(mut d: &Definition, mut name: &Position) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut jdef: metamodelica::Ref<JSON::JSON> = JSON::emptyListObject();
    jdef = JSON::addPair(&(literal!("name")), &(JSON::makeString(d.name.clone())), jdef)?;
    jdef = JSON::addPair(&(literal!("kind")), &(JSON::makeString(d.kind.clone())), jdef)?;
    if !(stringEmpty(&d.detail)) {
        jdef = JSON::addPair(
            &(if (metamodelica::stringEq(&d.kind, &(literal!("class")))) {
                literal!("restriction")
            } else {
                literal!("type")
            }),
            &(JSON::makeString(d.detail.clone())),
            jdef,
        )?;
    }
    jdef = addInfo(&d.info, jdef)?;
    if name.exact.clone() {
        jdef = JSON::addPair(
            &(literal!("nameLine")),
            &(JSON::makeInteger(name.lineStart.clone())),
            jdef,
        )?;
        jdef = JSON::addPair(
            &(literal!("nameColumnStart")),
            &(JSON::makeInteger(name.columnStart.clone())),
            jdef,
        )?;
        jdef = JSON::addPair(
            &(literal!("nameColumnEnd")),
            &(JSON::makeInteger(name.columnEnd.clone())),
            jdef,
        )?;
    }
    Ok(jdef)
}

fn namePosition(
    mut d: &Definition,
    mut names: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Position>>,
    mut files: Files,
) -> Result<Position> {
    let mut pos: Position;
    if UnorderedMap::contains(d.key.clone(), names.clone())? {
        pos = UnorderedMap::getOrFail(d.key.clone(), names)?;
    } else {
        pos = placeDefinition(d, files)?;
        UnorderedMap::add(d.key.clone(), pos.clone(), names)?;
    }
    Ok(pos)
}

fn isInSpan(mut info: &SourceInfo, mut line: i32, mut column: i32) -> bool {
    let mut res: bool = (line > info.lineNumberStart.clone()
        || line == info.lineNumberStart.clone() && column >= info.columnNumberStart.clone())
        && (line < info.lineNumberEnd.clone()
            || line == info.lineNumberEnd.clone() && column <= info.columnNumberEnd.clone());
    res
}

fn isAt(mut pos: &Position, mut fileName: &ArcStr, mut line: i32, mut column: i32) -> bool {
    let mut res: bool = pos.exact.clone()
        && metamodelica::stringEq(&pos.fileName, &fileName)
        && pos.lineStart.clone() == line
        && column >= pos.columnStart.clone()
        && column <= pos.columnEnd.clone();
    res
}

fn newStringSet(mut old: Option<metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>>) -> StringSet {
    let mut set: StringSet;
    set = (::match_deref::match_deref! { match &(old) {
        Some(__esc_set) => {
            set = (*__esc_set).clone();
            set.clone()
        },
        _ => UnorderedSet::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 13),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    set
}

fn stringGt(mut s1: &ArcStr, mut s2: &ArcStr) -> bool {
    let mut res: bool = stringCompare(&s1, &s2) > 0;
    res
}

fn infoGt(mut i1: &SourceInfo, mut i2: &SourceInfo) -> bool {
    let mut res: bool = i1.lineNumberStart.clone() > i2.lineNumberStart.clone()
        || i1.lineNumberStart.clone() == i2.lineNumberStart.clone()
            && i1.columnNumberStart.clone() > i2.columnNumberStart.clone();
    res
}

fn ownerName(mut name: ArcStr) -> Result<ArcStr> {
    let mut owner: ArcStr = literal!("");
    for mut i in ({
        let __s = ((name).len() as i32);
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        if stringGet(&name, i)? == 46 {
            owner = substring(name, 1, i - 1)?;
            return Ok(owner);
        }
    }
    Ok(owner)
}

fn sourceHash(
    mut info: &SourceInfo,
    mut nested: &metamodelica::List<SourceInfo>,
    mut texts: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Option<metamodelica::Array<ArcStr>>>>,
) -> Result<ArcStr> {
    let mut hash: ArcStr = literal!("");
    let mut lines: metamodelica::Array<ArcStr> = Default::default();
    let mut line: i32;
    let mut col: i32;
    let mut len: i32 = 0;
    let mut h1: i32 = 0;
    let mut h2: i32 = 0;
    let mut i: i32;
    let mut n: i32 = 0;
    let mut c: i32;
    let mut last: i32 = 0;
    let mut in_string: bool = false;
    let mut in_line_comment: bool = false;
    let mut in_block_comment: bool = false;
    let mut space: bool = false;
    let mut pieces: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut p: ArcStr = arcstr::literal!("");
    if !(UnorderedMap::contains(info.fileName.clone(), texts.clone())?) {
        UnorderedMap::add(info.fileName.clone(), readLines(info.fileName.clone()), texts.clone())?;
    }
    let () = (match UnorderedMap::getOrFail(info.fileName.clone(), texts)? {
        Some(mut __esc_lines) => {
            lines = __esc_lines.clone();
            ()
        }
        _ => {
            return Ok(hash);
            ()
        }
    });
    line = info.lineNumberStart.clone();
    col = info.columnNumberStart.clone();
    for mut n in &**nested {
        let mut n = n.clone();
        pieces = metamodelica::cons(
            textRange(
                lines.clone(),
                line,
                col,
                n.lineNumberStart.clone(),
                n.columnNumberStart.clone() - 1,
            )?,
            pieces,
        );
        line = n.lineNumberEnd.clone();
        col = n.columnNumberEnd.clone() + 1;
    }
    pieces = metamodelica::cons(
        textRange(
            lines.clone(),
            line,
            col,
            info.lineNumberEnd.clone(),
            info.columnNumberEnd.clone(),
        )?,
        pieces,
    )
    .reverse();
    for mut p in &*pieces {
        let mut p = p.clone();
        n = ((p).len() as i32);
        i = 1;
        while i <= n {
            c = stringGet(&p, i)?;
            if in_line_comment {
                if c == 10 {
                    in_line_comment = false;
                    space = true;
                }
                i = i + 1;
            } else if in_block_comment {
                if c == 42 && i < n && stringGet(&p, i + 1)? == 47 {
                    in_block_comment = false;
                    space = true;
                    i = i + 2;
                } else {
                    i = i + 1;
                }
            } else if in_string {
                (h1, h2, len) = hashChar(c, h1, h2, len);
                if c == 92 && i < n {
                    (h1, h2, len) = hashChar(stringGet(&p, i + 1)?, h1, h2, len);
                    i = i + 1;
                } else if c == 34 {
                    in_string = false;
                }
                i = i + 1;
            } else if c == 47 && i < n && stringGet(&p, i + 1)? == 47 {
                in_line_comment = true;
                i = i + 2;
            } else if c == 47 && i < n && stringGet(&p, i + 1)? == 42 {
                in_block_comment = true;
                i = i + 2;
            } else if c == 32 || c == 9 || c == 10 || c == 13 {
                space = true;
                i = i + 1;
            } else {
                if space && isIdentChar(last) && isIdentChar(c) {
                    (h1, h2, len) = hashChar(32, h1, h2, len);
                }
                space = false;
                (h1, h2, len) = hashChar(c, h1, h2, len);
                last = c;
                in_string = c == 34;
                i = i + 1;
            }
        }
    }
    hash = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(len));
        __mm_s.push_str(&*literal!("-"));
        __mm_s.push_str(&*intString(h1));
        __mm_s.push_str(&*literal!("-"));
        __mm_s.push_str(&*intString(h2));
        ArcStr::from(__mm_s)
    };
    Ok(hash)
}

fn hashChar(mut c: i32, mut h1: i32, mut h2: i32, mut len: i32) -> (i32, i32, i32) {
    let mut h1: i32 = h1;
    let mut h2: i32 = h2;
    let mut len: i32 = len;
    h1 = intMod(h1 * 257 + c, HASH_MOD1.clone());
    h2 = intMod(h2 * 263 + c, HASH_MOD2.clone());
    len = len + 1;
    (h1, h2, len)
}

fn readLines(mut fileName: ArcStr) -> Option<metamodelica::Array<ArcStr>> {
    let mut lines: Option<metamodelica::Array<ArcStr>> = None;
    if '__try0: {
        if System::regularFileExists(fileName.clone()) {
            lines = Some(metamodelica::arrayFromVec(unwrap_break_err!(Util::stringSplitAtChar(unwrap_break_err!(System::readFile(fileName.clone()), '__try0), literal!("\n")), '__try0).into_iter().cloned().collect()));
        }
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    lines
}

fn textRange(
    mut lines: metamodelica::Array<ArcStr>,
    mut lineStart: i32,
    mut colStart: i32,
    mut lineEnd: i32,
    mut colEnd: i32,
) -> Result<ArcStr> {
    let mut text: ArcStr;
    let mut parts: metamodelica::List<ArcStr> = metamodelica::nil();
    if lineStart > lineEnd || lineStart < 1 || lineEnd > metamodelica::arrayLength(lines.clone()) {
        text = literal!("");
        return Ok(text);
    }
    if lineStart == lineEnd {
        text = lineRange(metamodelica::arrayGet(lines.clone(), lineStart)?, colStart, colEnd)?;
        return Ok(text);
    }
    parts = list![lineRange(
        metamodelica::arrayGet(lines.clone(), lineStart)?,
        colStart,
        ((metamodelica::arrayGet(lines.clone(), lineStart)?).len() as i32)
    )?];
    for mut l in lineStart + 1..=lineEnd - 1 {
        parts = metamodelica::cons(metamodelica::arrayGet(lines.clone(), l)?, parts);
    }
    parts = metamodelica::cons(
        lineRange(metamodelica::arrayGet(lines.clone(), lineEnd)?, 1, colEnd)?,
        parts,
    );
    text = stringDelimitList(parts.reverse(), literal!("\n"));
    Ok(text)
}

fn lineRange(mut line: ArcStr, mut colStart: i32, mut colEnd: i32) -> Result<ArcStr> {
    let mut text: ArcStr;
    let mut stop: i32 = std::cmp::min(colEnd, ((line).len() as i32));
    text = if (colStart < 1 || colStart > stop) {
        literal!("")
    } else {
        substring(line, colStart, stop)?
    };
    Ok(text)
}

fn isInClass(mut name: ArcStr, mut className: &ArcStr) -> Result<bool> {
    let mut res: bool;
    let mut len: i32 = ((className).len() as i32);
    res = metamodelica::stringEq(&name, &className)
        || ((name).len() as i32) > len
            && substring(name, 1, len + 1)? == {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*className);
                __mm_s.push_str(&*literal!("."));
                ArcStr::from(__mm_s)
            };
    Ok(res)
}

fn lastIdent(mut name: ArcStr) -> Result<ArcStr> {
    let mut id: ArcStr = List::last(&(Util::stringSplitAtChar(name.clone(), literal!("."))?))?;
    Ok(id)
}

fn uniqueDefinitions(mut defs: &metamodelica::List<Definition>) -> Result<metamodelica::List<Definition>> {
    let mut unique: metamodelica::List<Definition> = metamodelica::nil();
    let mut keys: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>> = UnorderedSet::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        13,
    );
    for mut d in &**defs {
        if !(UnorderedSet::contains(d.key.clone(), keys.clone())?) {
            UnorderedSet::add(d.key.clone(), keys.clone())?;
            unique = metamodelica::cons(d.clone(), unique);
        }
    }
    Ok(unique)
}

fn definitionGt(mut d1: &Definition, mut d2: &Definition) -> bool {
    let mut res: bool;
    let mut c: i32 = stringCompare(&d1.name, &d2.name);
    res = c > 0 || c == 0 && stringCompare(&d1.key, &d2.key) > 0;
    res
}

fn useInFile(mut u: &Use, mut fileName: ArcStr) -> bool {
    let mut res: bool = u.site.info.fileName.clone() == fileName.clone();
    res
}

fn placedGt(mut p1: &(Use, Position), mut p2: &(Use, Position)) -> bool {
    let mut res: bool;
    let mut a: Position;
    let mut b: Position;
    let mut c: i32;
    (_, a) = p1.clone();
    (_, b) = p2.clone();
    c = stringCompare(&a.fileName, &b.fileName);
    res = c > 0
        || c == 0
            && (a.lineStart.clone() > b.lineStart.clone()
                || a.lineStart.clone() == b.lineStart.clone() && a.columnStart.clone() > b.columnStart.clone());
    res
}

fn addInfo(mut info: &SourceInfo, mut obj: metamodelica::Ref<JSON::JSON>) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut obj: metamodelica::Ref<JSON::JSON> = obj;
    obj = JSON::addPair(&(literal!("file")), &(JSON::makeString(info.fileName.clone())), obj)?;
    obj = JSON::addPair(
        &(literal!("lineStart")),
        &(JSON::makeInteger(info.lineNumberStart.clone())),
        obj,
    )?;
    obj = JSON::addPair(
        &(literal!("columnStart")),
        &(JSON::makeInteger(info.columnNumberStart.clone())),
        obj,
    )?;
    obj = JSON::addPair(
        &(literal!("lineEnd")),
        &(JSON::makeInteger(info.lineNumberEnd.clone())),
        obj,
    )?;
    obj = JSON::addPair(
        &(literal!("columnEnd")),
        &(JSON::makeInteger(info.columnNumberEnd.clone())),
        obj,
    )?;
    Ok(obj)
}

fn useJSON(mut placed: &(Use, Position)) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut obj: metamodelica::Ref<JSON::JSON> = JSON::emptyListObject();
    let mut u: Use;
    let mut p: Position;
    (u, p) = placed.clone();
    obj = JSON::addPair(&(literal!("file")), &(JSON::makeString(p.fileName.clone())), obj)?;
    obj = JSON::addPair(&(literal!("lineStart")), &(JSON::makeInteger(p.lineStart.clone())), obj)?;
    obj = JSON::addPair(
        &(literal!("columnStart")),
        &(JSON::makeInteger(p.columnStart.clone())),
        obj,
    )?;
    obj = JSON::addPair(&(literal!("lineEnd")), &(JSON::makeInteger(p.lineEnd.clone())), obj)?;
    obj = JSON::addPair(&(literal!("columnEnd")), &(JSON::makeInteger(p.columnEnd.clone())), obj)?;
    obj = JSON::addPair(&(literal!("exact")), &(JSON::makeBoolean(p.exact.clone())), obj)?;
    obj = JSON::addPair(
        &(literal!("text")),
        &(JSON::makeString(writtenString(u.written.clone())?)),
        obj,
    )?;
    if !(p.exact.clone()) || AbsynUtil::pathIsQual(&(AbsynUtil::makeNotFullyQualified(u.written.clone()))) {
        obj = JSON::addPair(&(literal!("part")), &(JSON::makeInteger(u.index.clone())), obj)?;
    }
    obj = JSON::addPair(&(literal!("in")), &(JSON::makeString(u.site.scope.clone())), obj)?;
    obj = JSON::addPair(&(literal!("role")), &(JSON::makeString(u.site.role.clone())), obj)?;
    if u.candidate.clone() {
        obj = JSON::addPair(&(literal!("candidate")), &(JSON::makeBoolean(true)), obj)?;
    }
    Ok(obj)
}

fn writtenString(mut path: metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> {
    let mut r#str: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*if (AbsynUtil::pathIsFullyQualified(&path)) {
            literal!(".")
        } else {
            literal!("")
        });
        __mm_s.push_str(&*AbsynUtil::pathString(
            AbsynUtil::makeNotFullyQualified(path.clone()),
            literal!("."),
            true,
            false,
        )?);
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

fn placeUses(
    mut uses: &metamodelica::List<Use>,
    mut defs: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Definition>>,
    mut files: Files,
) -> Result<metamodelica::List<(Use, Position)>> {
    let mut placed: metamodelica::List<(Use, Position)> = metamodelica::nil();
    let mut order: metamodelica::List<(i32, i32, Use)> = metamodelica::nil();
    let mut candidates: metamodelica::List<Use> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut u: Use = <Use as ::std::default::Default>::default();
    let mut pos: Position;
    let mut sig: ArcStr;
    let mut found: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>> = UnorderedSet::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        13,
    );
    let mut primaries: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::List<(Use, Position)>>> =
        UnorderedMap::new(
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            1,
        );
    let mut pu: Use;
    let mut cu: Use;
    let mut pp: Position;
    let mut site: Site;
    for mut u in &**uses {
        let mut u = u.clone();
        i = i + 1;
        if u.candidate.clone() {
            candidates = metamodelica::cons(u, candidates);
        } else {
            order = metamodelica::cons((spanSize(&u.site.info), i, u), order);
        }
    }
    order = List::sort(
        order,
        (std::sync::Arc::new(
            move |__a0: (i32, i32, Use), __a1: (i32, i32, Use)| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(orderGt(&__a0, &__a1))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, Use), (i32, i32, Use)) -> Result<bool> + 'static>),
    )?;
    for mut o in &*order {
        (_, _, u) = o.clone();
        pos = placeUse(&u, files.clone());
        sig = useSignature(&u)?;
        if pos.exact.clone() && UnorderedSet::contains(positionSignature(u.key.clone(), &pos), found.clone())? {
            continue;
        }
        if pos.exact.clone() {
            UnorderedSet::add(sig, found.clone())?;
            UnorderedSet::add(positionSignature(u.key.clone(), &pos), found.clone())?;
            UnorderedMap::add(
                siteSignature(&u)?,
                metamodelica::cons(
                    (u.clone(), pos.clone()),
                    UnorderedMap::getOrDefault(siteSignature(&u)?, primaries.clone(), metamodelica::nil())?,
                ),
                primaries.clone(),
            )?;
            placed = metamodelica::cons((u, pos), placed);
        } else if !(UnorderedSet::contains(sig.clone(), found.clone())?) {
            placed = metamodelica::cons((u, pos), placed);
        }
    }
    for mut u in &*candidates.reverse() {
        let mut u = u.clone();
        if !(UnorderedMap::contains(siteSignature(&u)?, primaries.clone())?) {
            pos = placeUse(&u, files.clone());
            UnorderedMap::add(siteSignature(&u)?, list![(u.clone(), pos)], primaries.clone())?;
        }
        for mut p in &*UnorderedMap::getOrFail(siteSignature(&u)?, primaries.clone())? {
            (pu, pp) = p.clone();
            sig = positionSignature(u.key.clone(), &pp);
            if !(metamodelica::stringEq(&pu.key, &u.key) && !(pu.candidate.clone()))
                && !(UnorderedSet::contains(sig.clone(), found.clone())?)
            {
                UnorderedSet::add(sig, found.clone())?;
                cu = u.clone();
                site = cu.site.clone();
                site.role = pu.site.role.clone();
                cu.site = site;
                placed = metamodelica::cons((cu, pp), placed);
            }
        }
    }
    Ok(placed)
}

fn positionSignature(mut key: ArcStr, mut p: &Position) -> ArcStr {
    let mut sig: ArcStr = stringAppendList(list![
        key.clone(),
        literal!("|"),
        p.fileName.clone(),
        literal!(":"),
        intString(p.lineStart.clone()),
        literal!(":"),
        intString(p.columnStart.clone()),
        literal!("-"),
        intString(p.lineEnd.clone()),
        literal!(":"),
        intString(p.columnEnd.clone())
    ]);
    sig
}

fn siteSignature(mut u: &Use) -> Result<ArcStr> {
    let mut sig: ArcStr = stringAppendList(list![
        writtenString(u.written.clone())?,
        literal!("|"),
        intString(u.index.clone()),
        literal!("|"),
        u.site.info.fileName.clone(),
        literal!(":"),
        intString(u.site.info.lineNumberStart.clone()),
        literal!(":"),
        intString(u.site.info.columnNumberStart.clone()),
        literal!("-"),
        intString(u.site.info.lineNumberEnd.clone()),
        literal!(":"),
        intString(u.site.info.columnNumberEnd.clone())
    ]);
    Ok(sig)
}

fn useSignature(mut u: &Use) -> Result<ArcStr> {
    let mut sig: ArcStr = stringAppendList(list![
        u.key.clone(),
        literal!("|"),
        writtenString(u.written.clone())?,
        literal!("|"),
        intString(u.index.clone()),
        literal!("|"),
        u.site.role.clone(),
        literal!("|"),
        u.site.info.fileName.clone(),
        literal!(":"),
        intString(u.site.info.lineNumberStart.clone()),
        literal!(":"),
        intString(u.site.info.columnNumberStart.clone()),
        literal!("-"),
        intString(u.site.info.lineNumberEnd.clone()),
        literal!(":"),
        intString(u.site.info.columnNumberEnd.clone())
    ]);
    Ok(sig)
}

fn orderGt(mut o1: &(i32, i32, Use), mut o2: &(i32, i32, Use)) -> bool {
    let mut res: bool;
    let mut s1: i32;
    let mut s2: i32;
    let mut i1: i32;
    let mut i2: i32;
    (s1, i1, _) = o1.clone();
    (s2, i2, _) = o2.clone();
    res = s1 > s2 || s1 == s2 && i1 > i2;
    res
}

fn spanSize(mut info: &SourceInfo) -> i32 {
    let mut size: i32 = (info.lineNumberEnd.clone() - info.lineNumberStart.clone()) * 1000000
        + info.columnNumberEnd.clone()
        - info.columnNumberStart.clone()
        + 1000;
    size
}

fn spanPosition(mut info: &SourceInfo) -> Position {
    let mut pos: Position = Position {
        fileName: info.fileName.clone(),
        lineStart: info.lineNumberStart.clone(),
        columnStart: info.columnNumberStart.clone(),
        lineEnd: info.lineNumberEnd.clone(),
        columnEnd: info.columnNumberEnd.clone(),
        exact: false,
    };
    pos
}

fn tokenPosition(mut fileName: ArcStr, mut t: &Token) -> Position {
    let mut pos: Position = Position {
        fileName: fileName.clone(),
        lineStart: t.line.clone(),
        columnStart: t.column.clone(),
        lineEnd: t.line.clone(),
        columnEnd: t.endColumn.clone(),
        exact: true,
    };
    pos
}

fn placeUse(mut u: &Use, mut files: Files) -> Position {
    let mut pos: Position = spanPosition(&u.site.info);
    let mut file: SourceFile;
    let mut first: i32;
    let mut last: i32;
    let mut i: i32;
    let mut parts: metamodelica::List<ArcStr>;
    if '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(getFile(u.site.info.fileName.clone(), files.clone()), '__try0)) {
            Some(__pa1) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        file = metamodelica::Own::own(__pa1);
        (first, last) = unwrap_break_err!(tokenRange(&file, &u.site.info), '__try0);
        if metamodelica::stringEq(&u.site.role, &(literal!("classExtends"))) || metamodelica::stringEq(&u.site.role, &(literal!("redeclare"))) {
            i = unwrap_break_err!(findName(&file, first, last, &(AbsynUtil::pathFirstIdent(&u.written)), true), '__try0);
        } else {
            parts = AbsynUtil::pathToStringList(&u.written);
            i = unwrap_break_err!(findWritten(&file, first, last, &parts, u.index.clone(), AbsynUtil::pathIsFullyQualified(&u.written)), '__try0);
        }
        if i > 0 {
            unwrap_break_err!(metamodelica::arrayUpdate(file.taken.clone(), i, true), '__try0);
            pos = tokenPosition(u.site.info.fileName.clone(), &(unwrap_break_err!(metamodelica::arrayGet(file.tokens.clone(), i), '__try0)));
        }
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    pos
}

fn placeDefinition(mut def: &Definition, mut files: Files) -> Result<Position> {
    let mut pos: Position = spanPosition(&def.info);
    let mut file: SourceFile;
    let mut first: i32;
    let mut last: i32;
    let mut i: i32;
    let mut name: ArcStr = lastIdent(def.name.clone())?;
    if '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(getFile(def.info.fileName.clone(), files.clone()), '__try0)) {
            Some(__pa1) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        file = metamodelica::Own::own(__pa1);
        (first, last) = unwrap_break_err!(tokenRange(&file, &def.info), '__try0);
        if metamodelica::stringEq(&def.kind, &(literal!("iterator"))) {
            i = unwrap_break_err!(findAfter(&file, first, last, &(literal!("for")), &name), '__try0);
        } else {
            i = unwrap_break_err!(findName(&file, first, last, &name, false), '__try0);
        }
        if i > 0 {
            unwrap_break_err!(metamodelica::arrayUpdate(file.taken.clone(), i, true), '__try0);
            pos = tokenPosition(def.info.fileName.clone(), &(unwrap_break_err!(metamodelica::arrayGet(file.tokens.clone(), i), '__try0)));
        }
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    Ok(pos)
}

fn placeEnd(mut def: &Definition, mut files: Files) -> Position {
    let mut pos: Position = spanPosition(&def.info);
    let mut file: SourceFile;
    let mut first: i32;
    let mut last: i32;
    let mut t: Token;
    if '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(getFile(def.info.fileName.clone(), files.clone()), '__try0)) {
            Some(__pa1) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        file = metamodelica::Own::own(__pa1);
        (first, last) = unwrap_break_err!(tokenRange(&file, &def.info), '__try0);
        if last > first {
            t = unwrap_break_err!(metamodelica::arrayGet(file.tokens.clone(), last), '__try0);
            if t.kind.clone() == IDENT.clone() && metamodelica::stringEq(&t.text, &(unwrap_break_err!(lastIdent(def.name.clone()), '__try0))) && metamodelica::stringEq(&(unwrap_break_err!(tokenText(&file, last - 1), '__try0)), &(literal!("end"))) {
                pos = tokenPosition(def.info.fileName.clone(), &t);
            }
        }
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    pos
}

fn findName(
    mut file: &SourceFile,
    mut first: i32,
    mut last: i32,
    mut name: &ArcStr,
    mut ignoreTaken: bool,
) -> Result<i32> {
    let mut index: i32 = 0;
    let mut t: Token;
    for mut i in first..=last {
        t = metamodelica::arrayGet(file.tokens.clone(), i)?;
        if t.kind.clone() == IDENT.clone()
            && metamodelica::stringEq(&t.text, &name)
            && !(isAfterDot(file, i, first)?)
            && (ignoreTaken || !(metamodelica::arrayGet(file.taken.clone(), i)?))
        {
            index = i;
            return Ok(index);
        }
    }
    Ok(index)
}

fn findAfter(
    mut file: &SourceFile,
    mut first: i32,
    mut last: i32,
    mut keyword: &ArcStr,
    mut name: &ArcStr,
) -> Result<i32> {
    let mut index: i32 = 0;
    let mut t: Token;
    for mut i in first + 1..=last {
        t = metamodelica::arrayGet(file.tokens.clone(), i)?;
        if t.kind.clone() == IDENT.clone()
            && metamodelica::stringEq(&t.text, &name)
            && !(metamodelica::arrayGet(file.taken.clone(), i)?)
            && metamodelica::stringEq(&(tokenText(file, i - 1)?), &keyword)
        {
            index = i;
            return Ok(index);
        }
    }
    Ok(index)
}

fn tokenKind(mut file: &SourceFile, mut i: i32) -> Result<i32> {
    let mut kind: i32;
    let mut t: Token = metamodelica::arrayGet(file.tokens.clone(), i)?;
    kind = t.kind.clone();
    Ok(kind)
}

fn tokenText(mut file: &SourceFile, mut i: i32) -> Result<ArcStr> {
    let mut text: ArcStr;
    let mut t: Token = metamodelica::arrayGet(file.tokens.clone(), i)?;
    text = t.text.clone();
    Ok(text)
}

fn isAfterDot(mut file: &SourceFile, mut i: i32, mut first: i32) -> Result<bool> {
    let mut res: bool = i > first && tokenKind(file, i - 1)? == DOT.clone();
    Ok(res)
}

fn findWritten(
    mut file: &SourceFile,
    mut first: i32,
    mut last: i32,
    mut parts: &metamodelica::List<ArcStr>,
    mut part: i32,
    mut fullyQualified: bool,
) -> Result<i32> {
    let mut index: i32 = 0;
    let mut t: Token;
    let mut j: i32;
    let mut k: i32;
    let mut depth: i32;
    let mut n: i32;
    let mut prefix: i32;
    let mut matched: bool;
    for mut i in first..=last {
        t = metamodelica::arrayGet(file.tokens.clone(), i)?;
        if t.kind.clone() == IDENT.clone()
            && metamodelica::stringEq(&t.text, &((parts).head().cloned()?))
            && isAfterDot(file, i, first)? == fullyQualified
        {
            j = i;
            k = i;
            matched = true;
            n = 1;
            prefix = 0;
            for mut p in &*(parts).rest()? {
                if j < last && tokenKind(file, j + 1)? == LBRACKET.clone() {
                    depth = 0;
                    j = j + 1;
                    while j <= last {
                        t = metamodelica::arrayGet(file.tokens.clone(), j)?;
                        depth = if (t.kind.clone() == LBRACKET.clone()) {
                            depth + 1
                        } else if (t.kind.clone() == RBRACKET.clone()) {
                            depth - 1
                        } else {
                            depth
                        };
                        if depth == 0 {
                            break;
                        }
                        j = j + 1;
                    }
                }
                if j + 2 <= last
                    && tokenKind(file, j + 1)? == DOT.clone()
                    && metamodelica::stringEq(&(tokenText(file, j + 2)?), &p)
                {
                    j = j + 2;
                } else if j + 2 <= last
                    && tokenKind(file, j + 1)? == DOT.clone()
                    && tokenKind(file, j + 2)? == LBRACE.clone()
                {
                    prefix = n;
                    k = j + 3;
                    matched = false;
                    while k <= last && tokenKind(file, k)? != RBRACE.clone() {
                        if metamodelica::stringEq(&(tokenText(file, k)?), &p) {
                            j = k;
                            matched = true;
                            break;
                        }
                        k = k + 1;
                    }
                    if !(matched) {
                        break;
                    }
                } else {
                    matched = false;
                    break;
                }
                n = n + 1;
            }
            if matched {
                k = if (part == ((parts).len() as i32)) {
                    j
                } else {
                    partIndex(file, i, j, part)?
                };
                if k > 0 && (part <= prefix || !(metamodelica::arrayGet(file.taken.clone(), k)?)) {
                    index = k;
                    return Ok(index);
                }
            }
        }
    }
    Ok(index)
}

fn partIndex(mut file: &SourceFile, mut i: i32, mut j: i32, mut part: i32) -> Result<i32> {
    let mut index: i32 = 0;
    let mut n: i32 = 0;
    let mut depth: i32 = 0;
    let mut t: Token;
    for mut k in i..=j {
        t = metamodelica::arrayGet(file.tokens.clone(), k)?;
        if t.kind.clone() == LBRACKET.clone() {
            depth = depth + 1;
        } else if t.kind.clone() == RBRACKET.clone() {
            depth = depth - 1;
        } else if t.kind.clone() == IDENT.clone() && depth == 0 {
            n = n + 1;
            if n == part {
                index = k;
                return Ok(index);
            }
        }
    }
    Ok(index)
}

fn tokenRange(mut file: &SourceFile, mut info: &SourceInfo) -> Result<(i32, i32)> {
    let mut first: i32;
    let mut last: i32;
    let mut lo: i32 = 1;
    let mut hi: i32 = metamodelica::arrayLength(file.tokens.clone());
    let mut mid: i32;
    let mut t: Token;
    while lo <= hi {
        mid = intDiv(lo + hi, 2);
        t = metamodelica::arrayGet(file.tokens.clone(), mid)?;
        if t.line.clone() < info.lineNumberStart.clone()
            || t.line.clone() == info.lineNumberStart.clone() && t.column.clone() < info.columnNumberStart.clone()
        {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    first = lo;
    last = first - 1;
    while last < metamodelica::arrayLength(file.tokens.clone()) {
        t = metamodelica::arrayGet(file.tokens.clone(), last + 1)?;
        if t.line.clone() > info.lineNumberEnd.clone()
            || t.line.clone() == info.lineNumberEnd.clone() && t.endColumn.clone() > info.columnNumberEnd.clone()
        {
            break;
        }
        last = last + 1;
    }
    Ok((first, last))
}

fn getFile(mut fileName: ArcStr, mut files: Files) -> Result<Option<SourceFile>> {
    let mut file: Option<SourceFile>;
    if UnorderedMap::contains(fileName.clone(), files.clone())? {
        file = UnorderedMap::getOrFail(fileName, files)?;
    } else {
        file = readSourceFile(fileName.clone());
        UnorderedMap::add(fileName, file.clone(), files)?;
    }
    Ok(file)
}

fn readSourceFile(mut fileName: ArcStr) -> Option<SourceFile> {
    let mut file: Option<SourceFile> = None;
    let mut tokens: metamodelica::Array<Token>;
    if '__try0: {
        if System::regularFileExists(fileName.clone()) {
            tokens = metamodelica::arrayFromVec(
                unwrap_break_err!(tokenize(unwrap_break_err!(System::readFile(fileName.clone()), '__try0)), '__try0)
                    .into_iter()
                    .cloned()
                    .collect(),
            );
            file = Some(SourceFile {
                tokens: tokens.clone(),
                taken: arrayCreate(metamodelica::arrayLength(tokens.clone()), false),
            });
        }
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    file
}

fn tokenize(mut source: ArcStr) -> Result<metamodelica::List<Token>> {
    let mut tokens: metamodelica::List<Token> = metamodelica::nil();
    let mut len: i32 = ((source).len() as i32);
    let mut i: i32 = 1;
    let mut line: i32 = 1;
    let mut col: i32 = 1;
    let mut c: i32;
    let mut start: i32;
    let mut start_col: i32;
    let mut in_group: bool = false;
    let mut t: Token;
    while i <= len {
        c = stringGet(&source, i)?;
        if c == 10 {
            line = line + 1;
            col = 1;
            i = i + 1;
        } else if c == 47 && i < len && stringGet(&source, i + 1)? == 47 {
            while i <= len && stringGet(&source, i)? != 10 {
                i = i + 1;
                col = col + 1;
            }
        } else if c == 47 && i < len && stringGet(&source, i + 1)? == 42 {
            i = i + 2;
            col = col + 2;
            while i <= len && !(stringGet(&source, i)? == 42 && i < len && stringGet(&source, i + 1)? == 47) {
                if stringGet(&source, i)? == 10 {
                    line = line + 1;
                    col = 1;
                } else {
                    col = col + 1;
                }
                i = i + 1;
            }
            i = i + 2;
            col = col + 2;
        } else if c == 34 || c == 39 {
            start = i;
            start_col = col;
            i = i + 1;
            col = col + 1;
            while i <= len && stringGet(&source, i)? != c {
                if stringGet(&source, i)? == 92 {
                    i = i + 1;
                    col = col + 1;
                }
                if i <= len && stringGet(&source, i)? == 10 {
                    line = line + 1;
                    col = 1;
                } else {
                    col = col + 1;
                }
                i = i + 1;
            }
            if c == 39 && i <= len {
                tokens = metamodelica::cons(
                    Token {
                        kind: IDENT.clone(),
                        text: substring(source.clone(), start, i)?,
                        line: line,
                        column: start_col,
                        endColumn: col,
                    },
                    tokens,
                );
            }
            i = i + 1;
            col = col + 1;
        } else if isIdentStart(c) {
            start = i;
            start_col = col;
            while i <= len && isIdentChar(stringGet(&source, i)?) {
                i = i + 1;
                col = col + 1;
            }
            tokens = metamodelica::cons(
                Token {
                    kind: IDENT.clone(),
                    text: substring(source.clone(), start, i - 1)?,
                    line: line,
                    column: start_col,
                    endColumn: col - 1,
                },
                tokens,
            );
        } else if isDigit(c) {
            while i <= len && isDigit(stringGet(&source, i)?) {
                i = i + 1;
                col = col + 1;
            }
            if i <= len && stringGet(&source, i)? == 46 {
                i = i + 1;
                col = col + 1;
                while i <= len && isDigit(stringGet(&source, i)?) {
                    i = i + 1;
                    col = col + 1;
                }
            }
            if i <= len && (stringGet(&source, i)? == 101 || stringGet(&source, i)? == 69) {
                i = i + 1;
                col = col + 1;
                if i <= len && (stringGet(&source, i)? == 43 || stringGet(&source, i)? == 45) {
                    i = i + 1;
                    col = col + 1;
                }
                while i <= len && isDigit(stringGet(&source, i)?) {
                    i = i + 1;
                    col = col + 1;
                }
            }
        } else {
            if c == 46 && !(i < len && listMember(stringGet(&source, i + 1)?, list![42, 47, 43, 45, 94])) {
                tokens = metamodelica::cons(
                    Token {
                        kind: DOT.clone(),
                        text: literal!("."),
                        line: line,
                        column: col,
                        endColumn: col,
                    },
                    tokens,
                );
            } else if c == 91 {
                tokens = metamodelica::cons(
                    Token {
                        kind: LBRACKET.clone(),
                        text: literal!("["),
                        line: line,
                        column: col,
                        endColumn: col,
                    },
                    tokens,
                );
            } else if c == 93 {
                tokens = metamodelica::cons(
                    Token {
                        kind: RBRACKET.clone(),
                        text: literal!("]"),
                        line: line,
                        column: col,
                        endColumn: col,
                    },
                    tokens,
                );
            } else if c == 123 && !((tokens).is_empty()) {
                t = (tokens).head().cloned()?;
                if t.kind.clone() == DOT.clone() {
                    tokens = metamodelica::cons(
                        Token {
                            kind: LBRACE.clone(),
                            text: literal!("{"),
                            line: line,
                            column: col,
                            endColumn: col,
                        },
                        tokens,
                    );
                    in_group = true;
                }
            } else if c == 125 && in_group {
                tokens = metamodelica::cons(
                    Token {
                        kind: RBRACE.clone(),
                        text: literal!("}"),
                        line: line,
                        column: col,
                        endColumn: col,
                    },
                    tokens,
                );
                in_group = false;
            }
            i = i + 1;
            col = col + 1;
        }
    }
    tokens = tokens.reverse();
    Ok(tokens)
}

fn isIdentStart(mut c: i32) -> bool {
    let mut res: bool = c >= 65 && c <= 90 || c >= 97 && c <= 122 || c == 95;
    res
}

fn isIdentChar(mut c: i32) -> bool {
    let mut res: bool = isIdentStart(c) || isDigit(c);
    res
}

fn isDigit(mut c: i32) -> bool {
    let mut res: bool = c >= 48 && c <= 57;
    res
}
