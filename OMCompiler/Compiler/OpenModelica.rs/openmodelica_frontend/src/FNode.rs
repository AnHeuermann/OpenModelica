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

use crate::FGraph;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::FCore::RefTree;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::MutableWeak;

// public imports
// protected imports
pub type Name = ArcStr;

pub type Names = metamodelica::List<ArcStr>;

pub type Id = i32;

pub type Seq = i32;

pub type Next = i32;

pub type Node = metamodelica::Ref<FCore::Node>;

pub type Data = metamodelica::Ref<FCore::Data>;

pub type Kind = FCore::Kind;

pub type Ref = Mutable::Mutable<metamodelica::Ref<FCore::Node>>;

pub type Refs = metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;

pub type Children = metamodelica::Ref<FCore::RefTree::Tree>;

pub type Parents = metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;

pub type Scope = metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;

pub type ImportTable = FCore::ImportTable;

pub type Graph = FCore::Graph;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Import = Absyn::Import;

pub(crate) const extendsPrefix: &'static str = "$ext_";

pub(crate) const topNodeName: &'static str = "$top";

// these names are used mostly for edges in the graph
// the edges are saved inside the AvlTree ("name", Ref)
pub(crate) const tyNodeName: &'static str = "$ty";

pub(crate) const ftNodeName: &'static str = "$ft";

pub(crate) const refNodeName: &'static str = "$ref";

pub(crate) const modNodeName: &'static str = "$mod";

pub(crate) const bndNodeName: &'static str = "$bnd";

pub(crate) const cndNodeName: &'static str = "$cnd";

pub(crate) const dimsNodeName: &'static str = "$dims";

pub(crate) const tydimsNodeName: &'static str = "$tydims";

pub(crate) const subsNodeName: &'static str = "$subs";

pub(crate) const ccNodeName: &'static str = "$cc";

pub(crate) const eqNodeName: &'static str = "$eq";

pub(crate) const ieqNodeName: &'static str = "$ieq";

pub(crate) const alNodeName: &'static str = "$al";

pub(crate) const ialNodeName: &'static str = "$ial";

pub(crate) const optNodeName: &'static str = "$opt";

pub(crate) const edNodeName: &'static str = "$ed";

pub(crate) const forNodeName: &'static str = "$for";

pub(crate) const matchNodeName: &'static str = "$match";

pub(crate) const cloneNodeName: &'static str = "$clone";

pub(crate) const origNodeName: &'static str = "$original";

pub(crate) const feNodeName: &'static str = "$functionEvaluation";

pub(crate) const duNodeName: &'static str = "$definedUnits";

pub(crate) const veNodeName: &'static str = "$ve";

pub(crate) const imNodeName: &'static str = "$imp";

pub(crate) const itNodeName: &'static str = "$it";

pub(crate) const assertNodeName: &'static str = "$assert";

pub(crate) const statusNodeName: &'static str = "$status";

pub(crate) fn toRef(mut inNode: Node) -> Ref {
    let mut outRef: Ref;
    outRef = Mutable::create(inNode);
    outRef
}

pub fn fromRef(mut inRef: Ref) -> Node {
    let mut outNode: Node;
    outNode = Mutable::access(inRef);
    outNode
}

pub(crate) fn updateRef(mut inRef: Ref, mut inNode: Node) -> Ref {
    let mut outRef: Ref;
    Mutable::update(inRef.clone(), inNode);
    outRef = inRef;
    outRef
}

pub fn id(mut inNode: &Node) -> Id {
    let mut id: Id;
    let __arc1 = &(*inNode);
    let FCore::N { id: __pa0, .. } = &**__arc1;
    id = metamodelica::Own::own(__pa0);
    id
}

pub(crate) fn parents(mut inNode: &Node) -> Result<Parents> {
    let mut p: Parents;
    let mut w: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
    let __arc1 = &(*inNode);
    let FCore::N { parents: __pa0, .. } = &**__arc1;
    w = metamodelica::Own::own(__pa0);
    p = ({
        let mut __acc: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>> = metamodelica::nil();
        for mut r in (w).into_iter().cloned() {
            let __x = MutableWeak::upgrade(r.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(p)
}

pub(crate) fn originalParent(mut inNode: &Node) -> Result<Ref> {
    let mut r: Ref;
    let mut w: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
    let __arc1 = &(*inNode);
    let FCore::N { parents: __pa0, .. } = &**__arc1;
    w = metamodelica::Own::own(__pa0);
    r = MutableWeak::upgrade(List::last(&w)?)?;
    Ok(r)
}

pub(crate) fn refOriginalParent(mut inRef: Ref) -> Result<Ref> {
    let mut r: Ref;
    r = originalParent(&(fromRef(inRef)))?;
    Ok(r)
}

pub(crate) fn contextualParent(mut inNode: &Node) -> Result<Ref> {
    let mut r: Ref;
    let mut w: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
    let __arc1 = &(*inNode);
    let FCore::N { parents: __pa0, .. } = &**__arc1;
    w = metamodelica::Own::own(__pa0);
    r = MutableWeak::upgrade((w).head().cloned()?)?;
    Ok(r)
}

pub(crate) fn hasParents(mut inNode: &Node) -> bool {
    let mut b: bool;
    let mut w: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
    let __arc1 = &(*inNode);
    let FCore::N { parents: __pa0, .. } = &**__arc1;
    w = metamodelica::Own::own(__pa0);
    b = !((w).is_empty());
    b
}

pub(crate) fn refParents(mut inRef: Ref) -> Result<Parents> {
    let mut p: Parents;
    p = parents(&(fromRef(inRef)))?;
    Ok(p)
}

pub(crate) fn refPushParents(mut inRef: Ref, mut inParents: Parents) -> Ref {
    let mut outRef: Ref;
    let mut n: Name;
    let mut i: Id;
    let mut p: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
    let mut c: Children;
    let mut d: Data;
    let __arc5 = fromRef(inRef.clone());
    let FCore::N {
        name: __pa0,
        id: __pa1,
        parents: __pa2,
        children: __pa3,
        data: __pa4,
    } = &*__arc5;
    n = metamodelica::Own::own(__pa0);
    i = metamodelica::Own::own(__pa1);
    p = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    d = metamodelica::Own::own(__pa4);
    p = listAppend(
        ({
            let mut __acc: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>> =
                metamodelica::nil();
            for mut r in (inParents).into_iter().cloned() {
                let __x = MutableWeak::downgrade(r.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        p,
    );
    outRef = updateRef(
        inRef,
        metamodelica::Ref::new(FCore::Node {
            name: n,
            id: i,
            parents: p,
            children: c,
            data: d,
        }),
    );
    outRef
}

pub(crate) fn setParents(mut inNode: &Node, mut inParents: Parents) -> Node {
    let mut outNode: Node;
    let mut n: Name;
    let mut i: Id;
    let mut p: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
    let mut c: Children;
    let mut d: Data;
    let __arc5 = &(*inNode);
    let FCore::N {
        name: __pa0,
        id: __pa1,
        parents: __pa2,
        children: __pa3,
        data: __pa4,
    } = &**__arc5;
    n = metamodelica::Own::own(__pa0);
    i = metamodelica::Own::own(__pa1);
    p = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    d = metamodelica::Own::own(__pa4);
    outNode = metamodelica::Ref::new(FCore::Node {
        name: n,
        id: i,
        parents: ({
            let mut __acc: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>> =
                metamodelica::nil();
            for mut r in (inParents).into_iter().cloned() {
                let __x = MutableWeak::downgrade(r.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        children: c,
        data: d,
    });
    outNode
}

pub(crate) fn target(mut inNode: &Node) -> Result<Ref> {
    let mut outRef: Ref;
    let __pa0 = ::match_deref::match_deref! { match &(targetScope(inNode)?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outRef = metamodelica::Own::own(__pa0);
    Ok(outRef)
}

pub(crate) fn targetScope(mut inNode: &Node) -> Result<Scope> {
    let mut outScope: Scope;
    outScope = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::REF { target: __esc_outScope }, .. } => {
            outScope = (*__esc_outScope).clone();
            outScope.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outScope)
}

pub(crate) fn new(mut inName: Name, mut inId: Id, mut inParents: Parents, mut inData: Data) -> Node {
    let mut node: Node;
    node = metamodelica::Ref::new(FCore::Node {
        name: inName,
        id: inId,
        parents: ({
            let mut __acc: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>> =
                metamodelica::nil();
            for mut r in (inParents).into_iter().cloned() {
                let __x = MutableWeak::downgrade(r.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        children: FCore::RefTree::new(),
        data: inData,
    });
    node
}

pub(crate) fn addImport(
    mut inImport: &metamodelica::Ref<SCode::Element>,
    mut inImportTable: &ImportTable,
) -> Result<ImportTable> {
    let mut outImportTable: ImportTable;
    outImportTable = (::match_deref::match_deref! { match &((&**inImport, inImportTable)) {
        (Deref @ SCode::Element::IMPORT { imp: imp @ Absyn::Import::UNQUAL_IMPORT { .. }, .. }, FCore::ImportTable { hidden, qualifiedImports: qual_imps, unqualifiedImports: unqual_imps }) => {
            let mut unqual_imps = (*unqual_imps).clone();
            unqual_imps = List::unionElt(imp.clone(), unqual_imps.clone());
            FCore::ImportTable { hidden: hidden.clone(), qualifiedImports: qual_imps.clone(), unqualifiedImports: unqual_imps.clone() }
        },
        (Deref @ SCode::Element::IMPORT { imp, info, .. }, FCore::ImportTable { hidden, qualifiedImports: qual_imps, unqualifiedImports: unqual_imps }) => {
            let mut imp = (*imp).clone();
            let mut qual_imps = (*qual_imps).clone();
            imp = translateQualifiedImportToNamed(imp.clone())?;
            checkUniqueQualifiedImport(imp.clone(), metamodelica::AsArg::as_arg(&qual_imps), metamodelica::AsArg::as_arg(&info))?;
            qual_imps = List::unionElt(imp.clone(), qual_imps.clone());
            FCore::ImportTable { hidden: hidden.clone(), qualifiedImports: qual_imps.clone(), unqualifiedImports: unqual_imps.clone() }
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outImportTable)
}

fn translateQualifiedImportToNamed(mut inImport: Import) -> Result<Import> {
    let mut outImport: Import;
    outImport = (match inImport.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => inImport,
        Absyn::Import::QUAL_IMPORT { path: mut path } => {
            let mut name: Name;
            name = AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path));
            Absyn::Import::NAMED_IMPORT {
                name: name,
                path: path.clone(),
            }
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outImport)
}

fn checkUniqueQualifiedImport(
    mut inImport: Import,
    mut inImports: &metamodelica::List<Absyn::Import>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inImport.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let false = (List::isMemberOnTrue(
                inImport.clone(),
                inImports,
                &move |__a0: Absyn::Import, __a1: Absyn::Import| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(compareQualifiedImportNames(&__a0, &__a1))
                },
            )?) else {
                return Err("pattern mismatch");
            };
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let Absyn::Import::NAMED_IMPORT { name: mut name, .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Error::addSourceMessage(
                &(Error::MULTIPLE_QUALIFIED_IMPORTS_WITH_SAME_NAME.clone()),
                list![name.clone()],
                inInfo,
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn compareQualifiedImportNames(mut inImport1: &Import, mut inImport2: &Import) -> bool {
    let mut outEqual: bool;
    outEqual = (match (inImport1.clone(), inImport2.clone()) {
        (Absyn::Import::NAMED_IMPORT { name: mut name1, .. }, Absyn::Import::NAMED_IMPORT { name: mut name2, .. })
            if (stringEqual(&name1, &name2)) =>
        {
            true
        }
        _ => false,
    });
    outEqual
}

pub(crate) fn addChildRef(
    mut inParentRef: Ref,
    mut inName: &Name,
    mut inChildRef: Ref,
    mut checkDuplicate: bool,
) -> Result<()> {
    let mut n: Name;
    let mut i: i32;
    let mut p: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
    let mut c: Children;
    let mut d: Data;
    let mut parent: Ref;
    let __arc5 = fromRef(inParentRef.clone());
    let FCore::N {
        name: __pa0,
        id: __pa1,
        parents: __pa2,
        children: __pa3,
        data: __pa4,
    } = &*__arc5;
    n = metamodelica::Own::own(__pa0);
    i = metamodelica::Own::own(__pa1);
    p = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    d = metamodelica::Own::own(__pa4);
    c = FCore::RefTree::add(
        c,
        inName,
        inChildRef,
        &*(if (checkDuplicate) {
            (std::sync::Arc::new(printElementConflictError)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                            Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                            ArcStr,
                        )
                            -> Result<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>
                        + 'static,
                >)
        } else {
            (std::sync::Arc::new(
                move |__a0: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                      __a1: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                      __a2: ArcStr|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(FCore::RefTree::addConflictReplace(__a0, __a1, &__a2))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                            Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                            ArcStr,
                        )
                            -> Result<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>
                        + 'static,
                >)
        }),
    )?;
    parent = updateRef(
        inParentRef,
        metamodelica::Ref::new(FCore::Node {
            name: n,
            id: i,
            parents: p,
            children: c,
            data: d,
        }),
    );
    Ok(())
}

fn printElementConflictError(mut newRef: Ref, mut oldRef: Ref, mut name: ArcStr) -> Result<Ref> {
    let mut dummy: Ref;
    let mut info1: SourceInfo;
    let mut info2: SourceInfo;
    if Config::acceptMetaModelicaGrammar()? {
        dummy = newRef;
    } else {
        info1 = SCodeUtil::elementInfo(&(getElementFromRef(newRef)?));
        info2 = SCodeUtil::elementInfo(&(getElementFromRef(oldRef)?));
        Error::addMultiSourceMessage(
            &(Error::DOUBLE_DECLARATION_OF_ELEMENTS.clone()),
            &(list![name]),
            &(list![info2, info1]),
        )?;
        return Err("fail");
    }
    Ok(dummy)
}

pub(crate) fn addImportToRef(mut r#ref: Ref, mut imp: &metamodelica::Ref<SCode::Element>) -> Result<()> {
    let mut n: Name;
    let mut id: i32;
    let mut p: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
    let mut c: Children;
    let mut it: ImportTable;
    let mut r: Ref;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(fromRef(r#ref.clone())) {
        Deref @ FCore::Node { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: Deref @ FCore::Data::IM { i: __pa4 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    n = metamodelica::Own::own(__pa0);
    id = metamodelica::Own::own(__pa1);
    p = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    it = metamodelica::Own::own(__pa4);
    it = addImport(imp, &it)?;
    r = updateRef(
        r#ref,
        metamodelica::Ref::new(FCore::Node {
            name: n,
            id: id,
            parents: p,
            children: c,
            data: metamodelica::Ref::new(FCore::Data::IM { i: it }),
        }),
    );
    Ok(())
}

pub(crate) fn addTypesToRef(mut r#ref: Ref, mut inTys: metamodelica::List<metamodelica::Ref<DAE::Type>>) -> Result<()> {
    let mut n: Name;
    let mut id: i32;
    let mut p: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
    let mut c: Children;
    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut r: Ref;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(fromRef(r#ref.clone())) {
        Deref @ FCore::Node { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: Deref @ FCore::Data::FT { tys: __pa4 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    n = metamodelica::Own::own(__pa0);
    id = metamodelica::Own::own(__pa1);
    p = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    tys = metamodelica::Own::own(__pa4);
    tys = List::unique(&(listAppend(inTys, tys)));
    tys = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
        for mut t in (tys.clone()).into_iter().cloned() {
            if !(!(isReturningFunctionWithNoReturnVariant(t.clone(), &tys)?)) {
                continue;
            }
            let __x = t.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    r = updateRef(
        r#ref,
        metamodelica::Ref::new(FCore::Node {
            name: n,
            id: id,
            parents: p,
            children: c,
            data: metamodelica::Ref::new(FCore::Data::FT { tys: tys }),
        }),
    );
    Ok(())
}

fn isReturningFunctionWithNoReturnVariant(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut tys: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<bool> {
    let mut b: bool;
    b = (match &*ty {
        DAE::Type::T_FUNCTION {
            functionAttributes:
                DAE::FunctionAttributes {
                    noReturn: DAE::NoReturn::RETURNS,
                    ..
                },
            ..
        } => List::isMemberOnTrue(
            ty,
            tys,
            &move |__a0: metamodelica::Ref<DAE::Type>, __a1: metamodelica::Ref<DAE::Type>| {
                isNoReturnVariantOf(&__a0, &__a1)
            },
        )?,
        _ => false,
    });
    Ok(b)
}

fn isNoReturnVariantOf(
    mut returning: &metamodelica::Ref<DAE::Type>,
    mut cand: &metamodelica::Ref<DAE::Type>,
) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match (returning, cand) {
        (Deref @ DAE::Type::T_FUNCTION { funcArg: fargs1, funcResultType: ret1, functionAttributes: attr1, path: path1 }, Deref @ DAE::Type::T_FUNCTION { funcArg: fargs2, funcResultType: ret2, functionAttributes: attr2 @ DAE::FunctionAttributes { noReturn: DAE::NoReturn::NORETURN, .. }, path: path2 }) => {
            AbsynUtil::pathEqual(path1, path2) && ret1.clone() == ret2.clone() && fargs1.clone() == fargs2.clone() && functionAttributesEqualModNoReturn(attr1, metamodelica::AsArg::as_arg(&attr2))?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn functionAttributesEqualModNoReturn(
    mut a1: &DAE::FunctionAttributes,
    mut a2: &DAE::FunctionAttributes,
) -> Result<bool> {
    let mut equal: bool;
    equal = (match (a1.clone(), a2.clone()) {
        (
            DAE::FunctionAttributes {
                inline: mut inl1,
                generateEvents: mut omp1,
                purity: mut pu1,
                isFunctionPointer: mut fp1,
                isBuiltin: mut bi1,
                functionParallelism: mut par1,
                noReturn: _,
            },
            DAE::FunctionAttributes {
                inline: mut inl2,
                generateEvents: mut omp2,
                purity: mut pu2,
                isFunctionPointer: mut fp2,
                isBuiltin: mut bi2,
                functionParallelism: mut par2,
                noReturn: _,
            },
        ) => {
            inl1.clone() == inl2.clone()
                && omp1.clone() == omp2.clone()
                && pu1.clone() == pu2.clone()
                && fp1.clone() == fp2.clone()
                && bi1.clone() == bi2.clone()
                && par1.clone() == par2.clone()
        }
    });
    Ok(equal)
}

pub(crate) fn addIteratorsToRef(
    mut r#ref: Ref,
    mut inIterators: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>,
) -> Result<()> {
    let mut n: Name;
    let mut id: i32;
    let mut p: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
    let mut c: Children;
    let mut it: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>;
    let mut r: Ref;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(fromRef(r#ref.clone())) {
        Deref @ FCore::Node { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: Deref @ FCore::Data::FS { fis: __pa4 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    n = metamodelica::Own::own(__pa0);
    id = metamodelica::Own::own(__pa1);
    p = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    it = metamodelica::Own::own(__pa4);
    r = updateRef(
        r#ref,
        metamodelica::Ref::new(FCore::Node {
            name: n,
            id: id,
            parents: p,
            children: c,
            data: metamodelica::Ref::new(FCore::Data::FS {
                fis: listAppend(it, inIterators),
            }),
        }),
    );
    Ok(())
}

pub(crate) fn addDefinedUnitToRef(mut r#ref: Ref, mut du: metamodelica::Ref<SCode::Element>) -> Result<()> {
    let mut n: Name;
    let mut id: i32;
    let mut p: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
    let mut c: Children;
    let mut r: Ref;
    let mut dus: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(fromRef(r#ref.clone())) {
        Deref @ FCore::Node { name: __pa0, id: __pa1, parents: __pa2, children: __pa3, data: Deref @ FCore::Data::DU { els: __pa4 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    n = metamodelica::Own::own(__pa0);
    id = metamodelica::Own::own(__pa1);
    p = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    dus = metamodelica::Own::own(__pa4);
    r = updateRef(
        r#ref,
        metamodelica::Ref::new(FCore::Node {
            name: n,
            id: id,
            parents: p,
            children: c,
            data: metamodelica::Ref::new(FCore::Data::DU {
                els: metamodelica::cons(du, dus),
            }),
        }),
    );
    Ok(())
}

pub fn name(mut n: &Node) -> ArcStr {
    let mut name: ArcStr;
    name = (match &**n {
        FCore::Node { name: s, .. } => s.clone(),
    });
    name
}

pub(crate) fn refName(mut r: Ref) -> ArcStr {
    let mut n: ArcStr;
    n = name(&(fromRef(r)));
    n
}

pub(crate) fn data(mut n: &Node) -> Data {
    let mut d: Data;
    d = (match &**n {
        FCore::Node { data: __esc_d, .. } => {
            d = (*__esc_d).clone();
            d.clone()
        }
    });
    d
}

pub(crate) fn refData(mut r: Ref) -> Data {
    let mut outData: Data;
    outData = data(&(fromRef(r)));
    outData
}

pub(crate) fn top(mut inRef: Ref) -> Result<Ref> {
    let mut outTop: Ref;
    outTop = inRef;
    while hasParents(&(fromRef(outTop.clone()))) {
        outTop = refOriginalParent(outTop)?;
    }
    Ok(outTop)
}

pub fn children(mut inNode: &Node) -> Children {
    let mut outChildren: Children;
    let __arc1 = &(*inNode);
    let FCore::N { children: __pa0, .. } = &**__arc1;
    outChildren = metamodelica::Own::own(__pa0);
    outChildren
}

pub(crate) fn hasChild(mut inNode: &Node, mut inName: Name) -> bool {
    let mut b: bool;
    b = 'mc: {
        let __mc_input = inName.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            childFromNode(inNode, inName.clone())?;
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(false)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    b
}

pub(crate) fn refHasChild(mut inRef: Ref, mut inName: Name) -> bool {
    let mut b: bool;
    b = hasChild(&(fromRef(inRef)), inName);
    b
}

pub(crate) fn setChildren(mut inNode: &Node, mut inChildren: Children) -> Node {
    let mut outNode: Node;
    let mut n: Name;
    let mut i: Id;
    let mut p: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
    let mut c: Children;
    let mut d: Data;
    let __arc5 = &(*inNode);
    let FCore::N {
        name: __pa0,
        id: __pa1,
        parents: __pa2,
        children: __pa3,
        data: __pa4,
    } = &**__arc5;
    n = metamodelica::Own::own(__pa0);
    i = metamodelica::Own::own(__pa1);
    p = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    d = metamodelica::Own::own(__pa4);
    outNode = metamodelica::Ref::new(FCore::Node {
        name: n,
        id: i,
        parents: p,
        children: inChildren,
        data: d,
    });
    outNode
}

pub(crate) fn setData(mut inNode: &Node, mut inData: Data) -> Node {
    let mut outNode: Node;
    let mut n: Name;
    let mut i: Id;
    let mut p: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
    let mut c: Children;
    let __arc4 = &(*inNode);
    let FCore::N {
        name: __pa0,
        id: __pa1,
        parents: __pa2,
        children: __pa3,
        data: _,
    } = &**__arc4;
    n = metamodelica::Own::own(__pa0);
    i = metamodelica::Own::own(__pa1);
    p = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    outNode = metamodelica::Ref::new(FCore::Node {
        name: n,
        id: i,
        parents: p,
        children: c,
        data: inData,
    });
    outNode
}

pub(crate) fn child(mut inParentRef: Ref, mut inName: Name) -> Result<Ref> {
    let mut outChildRef: Ref;
    outChildRef = childFromNode(&(fromRef(inParentRef)), inName)?;
    Ok(outChildRef)
}

pub(crate) fn childFromNode(mut inNode: &Node, mut inName: Name) -> Result<Ref> {
    let mut outChildRef: Ref;
    let mut c: Children;
    c = children(inNode);
    outChildRef = FCore::RefTree::get(&c, inName)?;
    Ok(outChildRef)
}

pub(crate) fn element2Data(
    mut inElement: metamodelica::Ref<SCode::Element>,
    mut inKind: Kind,
) -> Result<(Data, metamodelica::Ref<DAE::Var>)> {
    let mut outData: Data;
    let mut outVar: metamodelica::Ref<DAE::Var>;
    (outData, outVar) = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { name: n, prefixes: Deref @ SCode::Prefixes { visibility: vis, redeclarePrefix: _, finalPrefix: _, innerOuter: io, replaceablePrefix: _ }, attributes: SCode::Attributes { arrayDims: _, connectorType: ct, parallelism: prl, variability: var, direction: dir, .. }, typeSpec: _, modifications: _, comment: _, condition: _, info: _ } => {
            let mut nd: Data;
            let mut i: metamodelica::Ref<DAE::Var>;
            nd = metamodelica::Ref::new(FCore::Data::CO { e: inElement, r#mod: openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), kind: inKind, status: openmodelica_frontend_dump::FCore::Status::VAR_UNTYPED });
            i = metamodelica::Ref::new(DAE::Var { name: n.clone(), attributes: metamodelica::Ref::new(DAE::Attributes { connectorType: DAEUtil::toConnectorTypeNoState(ct.clone(), None), parallelism: prl.clone(), variability: var.clone(), direction: dir.clone(), innerOuter: io.clone(), visibility: vis.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None });
            (nd, i)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outData, outVar))
}

pub(crate) fn dataStr(mut inData: &Data) -> ArcStr {
    let mut outStr: ArcStr;
    outStr = (::match_deref::match_deref! { match inData {
        Deref @ FCore::Data::TOP { .. } => {
            literal!("TOP")
        },
        Deref @ FCore::Data::IT { i: _ } => {
            literal!("I")
        },
        Deref @ FCore::Data::CL { e: Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, .. } => {
            literal!("CE")
        },
        Deref @ FCore::Data::CL { .. } => {
            literal!("C")
        },
        Deref @ FCore::Data::CO { .. } => {
            literal!("c")
        },
        Deref @ FCore::Data::EX { .. } => {
            literal!("E")
        },
        Deref @ FCore::Data::DU { els: _ } => {
            literal!("U")
        },
        Deref @ FCore::Data::FT { tys: _ } => {
            literal!("FT")
        },
        Deref @ FCore::Data::AL { name: _, a: _ } => {
            literal!("ALG")
        },
        Deref @ FCore::Data::EQ { name: _, e: _ } => {
            literal!("EQ")
        },
        Deref @ FCore::Data::OT { constrainLst: _, clsAttrs: _ } => {
            literal!("OPT")
        },
        Deref @ FCore::Data::ED { ed: _ } => {
            literal!("ED")
        },
        Deref @ FCore::Data::FS { fis: _ } => {
            literal!("FS")
        },
        Deref @ FCore::Data::FI { fi: _ } => {
            literal!("FI")
        },
        Deref @ FCore::Data::MS { e: _ } => {
            literal!("MS")
        },
        Deref @ FCore::Data::MO { m: _ } => {
            literal!("M")
        },
        Deref @ FCore::Data::EXP { name: n, .. } => {
            n.clone()
        },
        Deref @ FCore::Data::DIMS { name: n, .. } => {
            n.clone()
        },
        Deref @ FCore::Data::CR { r: _ } => {
            literal!("r")
        },
        Deref @ FCore::Data::CC { cc: _ } => {
            literal!("CC")
        },
        Deref @ FCore::Data::ND { scopeType: _ } => {
            literal!("ND")
        },
        Deref @ FCore::Data::REF { target: _ } => {
            literal!("REF")
        },
        Deref @ FCore::Data::VR { .. } => {
            literal!("VR")
        },
        Deref @ FCore::Data::IM { i: _ } => {
            literal!("IM")
        },
        Deref @ FCore::Data::ASSERT { message: m } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("assert(")); __mm_s.push_str(&*m); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => {
            literal!("UKNOWN NODE DATA")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStr
}

pub(crate) fn toStr(mut inNode: &Node) -> ArcStr {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = 'mc: {
        let __mc_input = &**inNode;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ FCore::Node { name: _, id: i, parents: p, children: _, data: d } => {
                            let mut outStr: ArcStr = outStr.clone();
                            outStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[i:")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("] ")); __mm_s.push_str(&*literal!("[p:")); __mm_s.push_str(&*stringDelimitList(List::map(List::map(List::map(({
                let mut __acc: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>> = metamodelica::nil();
                for mut w in (p.clone()).into_iter().cloned() {
                            let __x = MutableWeak::upgrade(w.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), &fnptr!(fromRef, Mutable::Mutable<metamodelica::Ref<FCore::Node>>))?, &move |__a0: metamodelica::Ref<FCore::Node>| -> metamodelica::Result<_> { ::std::result::Result::Ok(id(&__a0)) })?, &fnptr!(intString, i32))?, literal!(", "))); __mm_s.push_str(&*literal!("] ")); __mm_s.push_str(&*literal!("[n:")); __mm_s.push_str(&*name(inNode)); __mm_s.push_str(&*literal!("] ")); __mm_s.push_str(&*literal!("[d:")); __mm_s.push_str(&*dataStr(metamodelica::AsArg::as_arg(&d))); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) };
                            Ok((outStr.clone(), outStr.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            outStr = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("Unhandled node!"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStr
}

pub(crate) fn toPathStr(mut inNode: &Node) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = 'mc: {
        let __mc_input = &**inNode;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: Deref @ metamodelica::ListNode::Nil, children: _, data: _ } => {
                    let mut outStr: ArcStr = outStr.clone();
                    outStr = name(inNode);
                    Ok((outStr.clone(), outStr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outStr = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: p, children: _, data: _ } => {
                    let mut nr: Ref;
                    let mut s: ArcStr;
                    let mut outStr: ArcStr = outStr.clone();
                    nr = MutableWeak::upgrade((p).head().cloned()?)?;
                    let true = (hasParents(&(fromRef(nr.clone())))) else { return Err("pattern mismatch") };
                    s = toPathStr(&(fromRef(nr.clone())))?;
                    outStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name(inNode)); ArcStr::from(__mm_s) };
                    Ok((outStr.clone(), outStr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outStr = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FCore::Node { name: _, id: _, parents: p, children: _, data: _ } => {
                    let mut nr: Ref;
                    let mut outStr: ArcStr = outStr.clone();
                    nr = MutableWeak::upgrade((p).head().cloned()?)?;
                    let false = (hasParents(&(fromRef(nr.clone())))) else { return Err("pattern mismatch") };
                    outStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name(inNode)); ArcStr::from(__mm_s) };
                    Ok((outStr.clone(), outStr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outStr = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStr)
}

pub(crate) fn scopeStr(mut sc: Scope) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = stringDelimitList(
        List::map(
            sc.reverse(),
            &fnptr!(refName, Mutable::Mutable<metamodelica::Ref<FCore::Node>>),
        )?,
        literal!("/"),
    );
    Ok(s)
}

pub(crate) fn isImplicitScope(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::TOP { .. }, .. } => false,
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { .. }, .. } => false,
        Deref @ FCore::Node { data: Deref @ FCore::Data::CO { .. }, .. } => false,
        Deref @ FCore::Node { data: Deref @ FCore::Data::CC { .. }, .. } => false,
        Deref @ FCore::Node { data: Deref @ FCore::Data::FS { .. }, .. } => false,
        Deref @ FCore::Node { data: Deref @ FCore::Data::MS { .. }, .. } => false,
        Deref @ FCore::Node { data: Deref @ FCore::Data::VR { .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isRefImplicitScope(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isImplicitScope(&(fromRef(inRef)));
    b
}

pub(crate) fn isEncapsulated(mut inNode: &Node) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e: Deref @ SCode::Element::CLASS { encapsulatedPrefix: SCode::Encapsulated::ENCAPSULATED { .. }, .. }, .. }, .. } => true,
        Deref @ FCore::Node { data: Deref @ FCore::Data::CO { .. }, .. } if (boolEq(Config::acceptMetaModelicaGrammar()?, false) && boolNot(Flags::isSet(Flags::GRAPH_INST.clone())?)) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isReference(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::REF { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isUserDefined(mut inNode: &Node) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { kind: FCore::Kind::USERDEFINED { .. }, .. }, .. } => {
            true
        },
        Deref @ FCore::Node { data: Deref @ FCore::Data::CO { kind: FCore::Kind::USERDEFINED { .. }, .. }, .. } => {
            true
        },
        _ if (hasParents(inNode)) => {
            b = isRefUserDefined(contextualParent(inNode)?)?;
            b
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isTop(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::TOP { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isExtends(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::EX { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isDerived(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e, .. }, .. } => {
            SCodeUtil::isDerivedClass(metamodelica::AsArg::as_arg(&e))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isClass(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isInstance(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { status: FCore::Status::CLS_INSTANCE { instanceOf: _ }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isRedeclare(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { redeclarePrefix: SCode::Redeclare::REDECLARE { .. }, .. }, .. }, .. }, .. } => true,
        Deref @ FCore::Node { data: Deref @ FCore::Data::CO { e: Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { redeclarePrefix: SCode::Redeclare::REDECLARE { .. }, .. }, .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isClassExtends(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e: Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isComponent(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CO { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isConstrainClass(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CC { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isCref(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CR { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isBasicType(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { kind: FCore::Kind::BASIC_TYPE { .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isBuiltin(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { kind: FCore::Kind::BUILTIN { .. }, .. }, .. } => true,
        Deref @ FCore::Node { data: Deref @ FCore::Data::CO { kind: FCore::Kind::BUILTIN { .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isFunction(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e, .. }, .. } if (SCodeUtil::isFunction(metamodelica::AsArg::as_arg(&e)) || SCodeUtil::isOperator(metamodelica::AsArg::as_arg(&e))) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isRecord(mut inNode: &Node) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e, .. }, .. } if (SCodeUtil::isRecord(metamodelica::AsArg::as_arg(&e))) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isSection(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::AL { .. }, .. } => true,
        Deref @ FCore::Node { data: Deref @ FCore::Data::EQ { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isMod(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::MO { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isModHolder(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { name: n, data: Deref @ FCore::Data::MO { .. }, .. } => {
            stringEq(&n, &arcstr::literal!(modNodeName))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isClone(mut inNode: &Node) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { parents: Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, .. } => {
            b = isRefVersion(MutableWeak::upgrade(r.clone())?);
            b
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isVersion(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::VR { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isDims(mut inNode: &Node) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::DIMS { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isIn(
    mut inNode: Node,
    mut inFunctionRefIs: &dyn ::std::ops::Fn(Mutable::Mutable<metamodelica::Ref<FCore::Node>>) -> Result<bool>,
) -> Result<bool> {
    pub type FunctionRefIs = std::sync::Arc<dyn ::std::ops::Fn(Ref) -> Result<bool> + 'static>;

    let mut b: bool;
    b = (match inFunctionRefIs.clone() {
        _ => {
            let mut s: Scope;
            let mut b1: bool;
            let mut b2: bool;
            s = originalScope(toRef(inNode.clone()))?;
            b1 = List::applyAndFold(&s, &fnptr!(boolOr, bool, bool), inFunctionRefIs, false)?;
            s = contextualScope(toRef(inNode))?;
            b2 = List::applyAndFold(&s, &fnptr!(boolOr, bool, bool), inFunctionRefIs, false)?;
            b = boolOr(b1, b2);
            b
        }
    });
    Ok(b)
}

pub(crate) fn nonImplicitRefFromScope<'__b>(mut inScope: &'__b Scope) -> Result<Ref> {
    '__tco: loop {
        ::match_deref::match_deref! { match inScope {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(return Err("fail"))
            },
            Deref @ metamodelica::ListNode::Cons { head: r, tail: _ } if (!(isRefImplicitScope(r.clone()))) => {
                return Ok(r.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { inScope = rest; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn namesUpToParentName(mut inRef: Ref, mut inName: Name) -> Result<Names> {
    let mut outNames: Names;
    outNames = namesUpToParentName_dispatch(inRef, inName, metamodelica::nil())?;
    Ok(outNames)
}

fn namesUpToParentName_dispatch(mut inRef: Ref, mut inName: Name, mut acc: Names) -> Result<Names> {
    '__tco: loop {
        match (inRef, inName.clone()) {
            (mut r, _) if (isRefTop(r.clone())) => return Ok(metamodelica::nil()),
            (mut r, _) if (stringEq(&inName, &(refName(r.clone())))) => return Ok(acc),
            (mut r, mut name) => {
                (inRef, inName, acc) = (
                    refOriginalParent(r.clone())?,
                    name,
                    metamodelica::cons(refName(r.clone()), acc),
                );
                continue '__tco;
            }
        }
    }
}

pub(crate) fn getModifierTarget(mut inRef: Ref) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = 'mc: {
        let __mc_input = inRef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut r = __mc_input.clone() else {
                return Err("nomatch");
            };
            if !(isRefTop(r.clone())) {
                return Err("guard");
            }
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut r = __mc_input.clone() else {
                return Err("nomatch");
            };
            if !(isRefModHolder(r.clone())) {
                return Err("guard");
            }
            r = refOriginalParent(r.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(refRefTargetScope(r.clone())?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            r = metamodelica::Own::own(__pa0);
            Ok(r.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(getModifierTarget(refOriginalParent(inRef.clone())?)?)
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outRef)
}

pub(crate) fn originalScope(mut inRef: Ref) -> Result<Scope> {
    let mut outScope: Scope;
    outScope = originalScope_dispatch(inRef, metamodelica::nil())?;
    Ok(outScope)
}

pub(crate) fn originalScope_dispatch(mut inRef: Ref, mut inAcc: Scope) -> Result<Scope> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inAcc) {
            acc if (isTop(&(fromRef(inRef.clone())))) => {
                return Ok(metamodelica::cons(inRef.clone(), acc.clone()).reverse())
            },
            acc => {
                let mut r: Ref;
                r = refOriginalParent(inRef.clone())?;
                { (inRef, inAcc) = (r, metamodelica::cons(inRef.clone(), acc.clone())); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn original(mut inParents: &Parents) -> Result<Ref> {
    let mut outOriginal: Ref;
    outOriginal = List::last(inParents)?;
    Ok(outOriginal)
}

pub(crate) fn contextualScope(mut inRef: Ref) -> Result<Scope> {
    let mut outScope: Scope;
    outScope = contextualScope_dispatch(inRef, metamodelica::nil())?;
    Ok(outScope)
}

pub(crate) fn contextualScope_dispatch(mut inRef: Ref, mut inAcc: Scope) -> Result<Scope> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inAcc) {
            acc if (isTop(&(fromRef(inRef.clone())))) => {
                return Ok(metamodelica::cons(inRef.clone(), acc.clone()).reverse())
            },
            acc => {
                let mut r: Ref;
                r = contextualParent(&(fromRef(inRef.clone())))?;
                { (inRef, inAcc) = (r, metamodelica::cons(inRef.clone(), acc.clone())); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn contextual(mut inParents: &Parents) -> Result<Ref> {
    let mut outContextual: Ref;
    outContextual = (inParents).head().cloned()?;
    Ok(outContextual)
}

pub(crate) fn lookupRef(mut inRef: Ref, mut inScope: Scope) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = (::match_deref::match_deref! { match &(inScope) {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } => {
            inRef
        },
        s => {
            let mut r: Ref;
            let mut s = (*s).clone();
            let __pa0 = ::match_deref::match_deref! { match &(s.clone().reverse()) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            s = metamodelica::Own::own(__pa0);
            r = lookupRef_dispatch(inRef, metamodelica::AsArg::as_arg(&s))?;
            r
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outRef)
}

pub(crate) fn lookupRef_dispatch<'__b>(mut inRef: Ref, mut inScope: &'__b Scope) -> Result<Ref> {
    '__tco: loop {
        ::match_deref::match_deref! { match inScope {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inRef)
            },
            Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                let mut n: Name;
                let mut r = (*r).clone();
                n = name(&(fromRef(r.clone())));
                r = child(inRef, n)?;
                { (inRef, inScope) = (r.clone(), rest); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn filter(
    mut inRef: Ref,
    mut inFilter: Arc<dyn ::std::ops::Fn(Mutable::Mutable<metamodelica::Ref<FCore::Node>>) -> Result<bool> + 'static>,
) -> Result<Refs> {
    pub type Filter = std::sync::Arc<dyn ::std::ops::Fn(Ref) -> Result<bool> + 'static>;

    let mut filtered: Refs;
    let mut c: Children;
    c = children(&(fromRef(inRef)));
    filtered = FCore::RefTree::fold(
        &c,
        &({
            let __pe_b2: Arc<
                dyn ::std::ops::Fn(Mutable::Mutable<metamodelica::Ref<FCore::Node>>) -> Result<bool> + 'static,
            > = inFilter.clone();
            move |__pe_a0, __pe_a1, __pe_a3| filter_work(&__pe_a0, __pe_a1, &*__pe_b2, __pe_a3)
        }),
        metamodelica::nil(),
    )?;
    filtered = filtered.reverse();
    Ok(filtered)
}

fn filter_work(
    mut name: &Name,
    mut r#ref: Ref,
    mut filter: &dyn ::std::ops::Fn(Mutable::Mutable<metamodelica::Ref<FCore::Node>>) -> Result<bool>,
    mut accum: Refs,
) -> Result<Refs> {
    pub type Filter = std::sync::Arc<dyn ::std::ops::Fn(Ref) -> Result<bool> + 'static>;

    let mut refs: Refs = accum;
    if filter(r#ref.clone())? {
        refs = metamodelica::cons(r#ref, refs);
    }
    Ok(refs)
}

pub(crate) fn isRefExtends(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isExtends(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefDerived(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isDerived(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefComponent(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isComponent(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefConstrainClass(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isConstrainClass(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefClass(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isClass(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefInstance(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isInstance(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefRedeclare(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isRedeclare(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefClassExtends(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isClassExtends(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefCref(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isCref(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefReference(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isReference(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefUserDefined(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isUserDefined(&(fromRef(inRef)))?;
    Ok(b)
}

pub(crate) fn isRefTop(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isTop(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefBasicType(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isBasicType(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefBuiltin(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isBuiltin(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefFunction(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isFunction(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefRecord(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isRecord(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefSection(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isSection(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefMod(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isMod(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefModHolder(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isModHolder(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefClone(mut inRef: Ref) -> Result<bool> {
    let mut b: bool;
    b = isClone(&(fromRef(inRef)))?;
    Ok(b)
}

pub(crate) fn isRefVersion(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isVersion(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefDims(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = isDims(&(fromRef(inRef)));
    b
}

pub(crate) fn isRefIn(
    mut inRef: Ref,
    mut inFunctionRefIs: &dyn ::std::ops::Fn(Mutable::Mutable<metamodelica::Ref<FCore::Node>>) -> Result<bool>,
) -> Result<bool> {
    pub type FunctionRefIs = std::sync::Arc<dyn ::std::ops::Fn(Ref) -> Result<bool> + 'static>;

    let mut b: bool;
    b = isIn(fromRef(inRef), inFunctionRefIs)?;
    Ok(b)
}

pub(crate) fn dfs(mut inRef: Ref) -> Result<Refs> {
    let mut outRefs: Refs;
    outRefs = (match inRef.clone() {
        _ => {
            let mut refs: Refs;
            let mut c: Children;
            c = children(&(fromRef(inRef.clone())));
            refs = FCore::RefTree::listValues(&c, metamodelica::nil());
            refs = List::flatten(List::map(refs, &dfs)?)?;
            refs = metamodelica::cons(inRef, refs);
            refs
        }
    });
    Ok(outRefs)
}

pub(crate) fn apply1<ExtraArg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inRef: Ref,
    mut inApply: &dyn ::std::ops::Fn(ArcStr, Mutable::Mutable<metamodelica::Ref<FCore::Node>>, ExtraArg) -> Result<ExtraArg>,
    mut inExtraArg: ExtraArg,
) -> Result<ExtraArg> {
    pub type Apply<ExtraArg: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Name, Ref, ExtraArg) -> Result<ExtraArg> + 'static>;

    let mut outExtraArg: ExtraArg;
    outExtraArg = FCore::RefTree::fold(&(children(&(fromRef(inRef.clone())))), inApply, inExtraArg)?;
    outExtraArg = inApply(refName(inRef.clone()), inRef, outExtraArg)?;
    Ok(outExtraArg)
}

pub(crate) fn hasImports(mut inNode: Node) -> Result<bool> {
    let mut b: bool;
    b = (match &*inNode {
        _ => {
            let mut qi: metamodelica::List<Absyn::Import>;
            let mut uqi: metamodelica::List<Absyn::Import>;
            let FCore::IMPORT_TABLE {
                hidden: _,
                qualifiedImports: __pa0,
                unqualifiedImports: __pa1,
            } = importTable(&(fromRef(refImport(toRef(inNode))?)))?;
            qi = metamodelica::Own::own(__pa0);
            uqi = metamodelica::Own::own(__pa1);
            b = boolOr(!((qi).is_empty()), !((uqi).is_empty()));
            b
        }
        _ => false,
    });
    Ok(b)
}

pub(crate) fn imports(
    mut inNode: Node,
) -> Result<(metamodelica::List<Absyn::Import>, metamodelica::List<Absyn::Import>)> {
    let mut outQualifiedImports: metamodelica::List<Absyn::Import>;
    let mut outUnQualifiedImports: metamodelica::List<Absyn::Import>;
    (outQualifiedImports, outUnQualifiedImports) = (match &*inNode {
        _ => {
            let mut qi: metamodelica::List<Absyn::Import>;
            let mut uqi: metamodelica::List<Absyn::Import>;
            let FCore::IMPORT_TABLE {
                hidden: _,
                qualifiedImports: __pa0,
                unqualifiedImports: __pa1,
            } = importTable(&(fromRef(refImport(toRef(inNode))?)))?;
            qi = metamodelica::Own::own(__pa0);
            uqi = metamodelica::Own::own(__pa1);
            (qi, uqi)
        }
        _ => (metamodelica::nil(), metamodelica::nil()),
    });
    Ok((outQualifiedImports, outUnQualifiedImports))
}

pub(crate) fn derivedRef(mut inRef: Ref) -> Result<Refs> {
    let mut outRefs: Refs;
    outRefs = (match inRef.clone() {
        _ if (isRefDerived(inRef.clone())) => list![child(inRef.clone(), arcstr::literal!(refNodeName))?],
        _ => metamodelica::nil(),
    });
    Ok(outRefs)
}

pub(crate) fn extendsRefs(mut inRef: Ref) -> Result<Refs> {
    let mut outRefs: Refs;
    outRefs = (match inRef.clone() {
        _ if (isRefClass(inRef.clone())) => {
            let mut refs: Refs;
            let mut rd: Refs;
            rd = derivedRef(inRef.clone())?;
            refs = filter(
                inRef.clone(),
                (std::sync::Arc::new(fnptr!(isRefExtends, Mutable::Mutable<metamodelica::Ref<FCore::Node>>))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Mutable::Mutable<metamodelica::Ref<FCore::Node>>) -> Result<bool> + 'static,
                    >),
            )?;
            refs = List::flatten(List::map1(
                refs,
                &filter,
                (std::sync::Arc::new(fnptr!(isRefReference, Mutable::Mutable<metamodelica::Ref<FCore::Node>>))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Mutable::Mutable<metamodelica::Ref<FCore::Node>>) -> Result<bool> + 'static,
                    >),
            )?)?;
            refs = listAppend(rd, refs);
            refs
        }
        _ => metamodelica::nil(),
    });
    Ok(outRefs)
}

pub(crate) fn cloneRef(
    mut inName: &Name,
    mut inRef: Ref,
    mut inParentRef: Ref,
    mut inGraph: Graph,
) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = (match inGraph {
        mut g => {
            let mut r: Ref;
            (g, r) = clone(&(fromRef(inRef)), inParentRef.clone(), g)?;
            addChildRef(inParentRef, inName, r.clone(), false)?;
            (g, r)
        }
    });
    Ok((outGraph, outRef))
}

pub(crate) fn clone(mut inNode: &Node, mut inParentRef: Ref, mut inGraph: Graph) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = (match &**inNode {
        FCore::Node {
            name,
            id,
            parents,
            children,
            data,
        } => {
            let mut g = inGraph;
            let mut n: Node;
            let mut r: Ref;
            let mut name = (*name).clone();
            let mut id = (*id).clone();
            let mut parents = (*parents).clone();
            let mut children = (*children).clone();
            let mut data = (*data).clone();
            parents = metamodelica::cons(MutableWeak::downgrade(inParentRef), parents.clone());
            let (__pa0, __pa5, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(FGraph::node(g, name.clone(), ({
                let mut __acc: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>> = metamodelica::nil();
                for mut p in (parents.clone()).into_iter().cloned() {
                    let __x = MutableWeak::upgrade(p.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), data.clone())) {
                        (__pa0, __pa5 @ Deref @ FCore::Node { name: __pa1, id: __pa2, parents: __pa3, children: _, data: __pa4 }) => (__pa0.clone(), __pa5.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => unreachable!(),
                    } };
            g = metamodelica::Own::own(__pa0);
            name = metamodelica::Own::own(__pa1);
            id = metamodelica::Own::own(__pa2);
            parents = metamodelica::Own::own(__pa3);
            data = metamodelica::Own::own(__pa4);
            n = metamodelica::Own::own(__pa5);
            r = toRef(n);
            (g, children) = cloneTree(children.clone(), r.clone(), g)?;
            r = updateRef(
                r,
                metamodelica::Ref::new(FCore::Node {
                    name: name.clone(),
                    id: id.clone(),
                    parents: parents.clone(),
                    children: children.clone(),
                    data: data.clone(),
                }),
            );
            (g, r)
        }
    });
    Ok((outGraph, outRef))
}

pub(crate) fn cloneTree(
    mut inChildren: Children,
    mut inParentRef: Ref,
    mut inGraph: Graph,
) -> Result<(Graph, Children)> {
    let mut outGraph: Graph;
    let mut outChildren: Children;
    (outChildren, outGraph) = FCore::RefTree::mapFold(
        inChildren,
        &({
            let __pe_b1 = inParentRef;
            move |__pe_a0, __pe_a2, __pe_a3| cloneChild(&__pe_a0, __pe_b1.clone(), __pe_a2, __pe_a3)
        }),
        inGraph,
    )?;
    Ok((outGraph, outChildren))
}

fn cloneChild(mut name: &Name, mut parentRef: Ref, mut inRef: Ref, mut inGraph: Graph) -> Result<(Ref, Graph)> {
    let mut r#ref: Ref;
    let mut graph: Graph;
    (graph, r#ref) = cloneRef(name, inRef, parentRef, inGraph)?;
    Ok((r#ref, graph))
}

pub(crate) fn copyRef(mut inRef: Ref, mut inGraph: Graph) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = (match inGraph {
        mut g => {
            let mut r: Ref;
            r = copyRefNoUpdate(inRef)?;
            (g, r) = updateRefs(r, g)?;
            (g, r)
        }
    });
    Ok((outGraph, outRef))
}

pub(crate) fn updateRefs(mut inRef: Ref, mut inGraph: Graph) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = (match inGraph {
        mut g => {
            let mut r: Ref;
            (r, g) = apply1(
                inRef.clone(),
                &move |__a0: ArcStr,
                       __a1: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                       __a2: (Mutable::Mutable<metamodelica::Ref<FCore::Node>>, FCore::Graph)| {
                    updateRefInGraph(&__a0, __a1, &__a2)
                },
                (inRef, g),
            )?;
            (g, r)
        }
    });
    Ok((outGraph, outRef))
}

fn updateRefInGraph(
    mut name: &Name,
    mut inRef: Ref,
    mut inTopRefAndGraph: &(Mutable::Mutable<metamodelica::Ref<FCore::Node>>, FCore::Graph),
) -> Result<(Mutable::Mutable<metamodelica::Ref<FCore::Node>>, FCore::Graph)> {
    let mut outTopRefAndGraph: (Mutable::Mutable<metamodelica::Ref<FCore::Node>>, FCore::Graph);
    outTopRefAndGraph = (match inTopRefAndGraph.clone() {
        (mut t, mut g) => {
            let mut n: Name;
            let mut i: Id;
            let mut p: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>>;
            let mut c: Children;
            let mut d: Data;
            let __arc5 = fromRef(inRef.clone());
            let FCore::N {
                name: __pa0,
                id: __pa1,
                parents: __pa2,
                children: __pa3,
                data: __pa4,
            } = &*__arc5;
            n = metamodelica::Own::own(__pa0);
            i = metamodelica::Own::own(__pa1);
            p = metamodelica::Own::own(__pa2);
            c = metamodelica::Own::own(__pa3);
            d = metamodelica::Own::own(__pa4);
            p = ({
                let mut __acc: metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<FCore::Node>>> =
                    metamodelica::nil();
                for mut w in (p).into_iter().cloned() {
                    let __x = MutableWeak::downgrade(lookupRefFromRef(t.clone(), MutableWeak::upgrade(w.clone())?)?);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            d = updateRefInData(d, t.clone())?;
            updateRef(
                inRef,
                metamodelica::Ref::new(FCore::Node {
                    name: n,
                    id: i,
                    parents: p,
                    children: c,
                    data: d,
                }),
            );
            (t, g)
        }
    });
    Ok(outTopRefAndGraph)
}

pub(crate) fn lookupRefFromRef(mut inRef: Ref, mut inOldRef: Ref) -> Result<Ref> {
    let mut outRef: Ref;
    outRef = (match inOldRef.clone() {
        _ => {
            let mut r: Ref;
            let mut s: Scope;
            s = originalScope(inOldRef)?;
            r = lookupRef(inRef, s)?;
            r
        }
    });
    Ok(outRef)
}

fn updateRefInData(mut inData: Data, mut inRef: Ref) -> Result<Data> {
    let mut outData: Data;
    outData = (match &*inData {
        FCore::Data::REF { target: sc } => {
            let mut sc = (*sc).clone();
            sc = List::map1r(sc.clone(), &lookupRefFromRef, inRef)?;
            metamodelica::Ref::new(FCore::Data::REF { target: sc.clone() })
        }
        _ => inData,
    });
    Ok(outData)
}

pub(crate) fn copyRefNoUpdate(mut inRef: Ref) -> Result<Ref> {
    let mut outRef: Ref = copy(fromRef(inRef.clone()))?;
    Ok(outRef)
}

fn copy(mut inNode: Node) -> Result<Ref> {
    let mut outRef: Ref;
    let mut node: Node = inNode;
    outRef = (match &*node {
        FCore::Node { .. } => {
            assign_field!(
                node.children = FCore::RefTree::map(node.children.clone(), &move |__a0: ArcStr,
                                                                                  __a1: Mutable::Mutable<
                    metamodelica::Ref<FCore::Node>,
                >| copyChild(&__a0, __a1))?
            );
            toRef(node)
        }
    });
    Ok(outRef)
}

fn copyChild(mut name: &Name, mut inRef: Ref) -> Result<Ref> {
    let mut r#ref: Ref = copyRefNoUpdate(inRef.clone())?;
    Ok(r#ref)
}

pub(crate) fn getElement(mut inNode: &Node) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outElement: metamodelica::Ref<SCode::Element>;
    outElement = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e, .. }, .. } => {
            e.clone()
        },
        Deref @ FCore::Node { data: Deref @ FCore::Data::CO { e, .. }, .. } => {
            e.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outElement)
}

pub(crate) fn getElementFromRef(mut inRef: Ref) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outElement: metamodelica::Ref<SCode::Element>;
    outElement = getElement(&(fromRef(inRef)))?;
    Ok(outElement)
}

pub(crate) fn isImplicitRefName(mut r: Ref) -> bool {
    let mut b: bool;
    b = (match r.clone() {
        _ if (!(isRefTop(r.clone()))) => FCore::isImplicitScope(refName(r.clone())),
        _ => false,
    });
    b
}

pub(crate) fn refInstVar(mut inRef: Ref) -> Result<metamodelica::Ref<DAE::Var>> {
    let mut v: metamodelica::Ref<DAE::Var>;
    let mut r: Ref;
    r = refInstance(inRef)?;
    let __pa0 = ::match_deref::match_deref! { match &(refData(r)) {
        Deref @ FCore::Data::IT { i: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    v = metamodelica::Own::own(__pa0);
    Ok(v)
}

pub(crate) fn refInstance(mut inRef: Ref) -> Result<Ref> {
    let mut r: Ref;
    r = child(inRef, arcstr::literal!(itNodeName))?;
    Ok(r)
}

pub(crate) fn isRefRefUnresolved(mut inRef: Ref) -> bool {
    let mut b: bool = false;
    b = 'mc: {
        let __mc_input = inRef.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut b: bool = b.clone();
            refRef(inRef.clone())?;
            b = (refRefTargetScope(inRef.clone())?).is_empty();
            Ok((b, b.clone()))
        })() {
            b = __wb0;
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
    b
}

pub(crate) fn isRefRefResolved(mut inRef: Ref) -> bool {
    let mut b: bool;
    b = !(isRefRefUnresolved(inRef));
    b
}

pub(crate) fn refRef(mut inRef: Ref) -> Result<Ref> {
    let mut r: Ref;
    r = child(inRef, arcstr::literal!(refNodeName))?;
    Ok(r)
}

pub(crate) fn refRefTargetScope(mut inRef: Ref) -> Result<Scope> {
    let mut sc: Scope;
    let mut r: Ref;
    r = refRef(inRef)?;
    sc = targetScope(&(fromRef(r)))?;
    Ok(sc)
}

pub(crate) fn refImport(mut inRef: Ref) -> Result<Ref> {
    let mut r: Ref;
    r = child(inRef, arcstr::literal!(imNodeName))?;
    Ok(r)
}

pub(crate) fn importTable(mut inNode: &Node) -> Result<ImportTable> {
    let mut it: ImportTable;
    it = (::match_deref::match_deref! { match inNode {
        Deref @ FCore::Node { data: Deref @ FCore::Data::IM { i: __esc_it }, .. } => {
            it = (*__esc_it).clone();
            it.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(it)
}

pub(crate) fn mkExtendsName(mut inPath: metamodelica::Ref<Absyn::Path>) -> Result<Name> {
    let mut outName: Name;
    outName = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*arcstr::literal!(extendsPrefix));
        __mm_s.push_str(&*AbsynUtil::pathString(inPath, literal!("."), true, false)?);
        ArcStr::from(__mm_s)
    };
    Ok(outName)
}

pub(crate) fn scopeHashWork(mut scope: &Scope, mut hash: i32) -> i32 {
    let mut hash: i32 = hash;
    for mut r in &**scope {
        hash = 31 * hash + stringHashDjb2(&(refName(r.clone())));
    }
    hash
}

pub(crate) fn scopePathEq(mut scope1: Scope, mut scope2: Scope) -> bool {
    let mut eq: bool;
    eq = ({
        let mut __acc: Option<bool> = None;
        let __thr_src0 = scope1;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = scope2;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(r1), Some(r2)) => {
                    let __x = metamodelica::stringEq(&(refName(r1.clone())), &(refName(r2.clone())));
                    __acc = Some(match __acc {
                        None => __x,
                        Some(__cur) => {
                            if __x < __cur {
                                __x
                            } else {
                                __cur
                            }
                        }
                    });
                }
                (None, None) => break,
                _ => panic!("threaded for: ranges of unequal length"),
            }
        }
        __acc.unwrap_or(true)
    });
    eq
}
