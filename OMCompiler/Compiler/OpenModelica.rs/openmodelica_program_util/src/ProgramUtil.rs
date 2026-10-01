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
use openmodelica_frontend::FBuiltin;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_util::Autoconf;
use openmodelica_util::Error;
use openmodelica_util::Print;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

pub fn buildWithin(mut inPath: metamodelica::Ref<Absyn::Path>) -> Result<Absyn::Within> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inPath) {
            Deref @ Absyn::Path::IDENT { .. } => {
                return Ok(openmodelica_ast::Absyn::Within::TOP)
            },
            Deref @ Absyn::Path::FULLYQUALIFIED { path } => {
                { inPath = path.clone(); continue '__tco; }
            },
            path => {
                let mut w_path: metamodelica::Ref<Absyn::Path>;
                w_path = AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&path))?;
                return Ok(Absyn::Within::WITHIN { path: w_path })
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn updateProgram(
    mut inNewProgram: Absyn::Program,
    mut inOldProgram: Absyn::Program,
    mut mergeAST: bool,
    mut allowFilenameChange: bool,
) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    let mut cs: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let mut w: Absyn::Within;
    let Absyn::PROGRAM {
        classes: __pa0,
        within_: __pa1,
    } = inNewProgram;
    cs = metamodelica::Own::own(__pa0);
    w = metamodelica::Own::own(__pa1);
    outProgram = updateProgram2(cs.reverse(), &w, inOldProgram, mergeAST, allowFilenameChange)?;
    Ok(outProgram)
}

pub(crate) fn updateProgram2<'__b>(
    mut inNewClasses: metamodelica::List<metamodelica::Ref<Absyn::Class>>,
    mut w: &'__b Absyn::Within,
    mut inOldProgram: Absyn::Program,
    mut mergeAST: bool,
    mut allowFilenameChange: bool,
) -> Result<Absyn::Program> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inNewClasses, w.clone(), inOldProgram)) {
            (Deref @ metamodelica::ListNode::Nil, _, prg) => {
                return Ok(prg.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: c1 @ Deref @ Absyn::Class { name, .. }, tail: c2 }, Absyn::Within::TOP { .. }, p2 @ Absyn::Program { classes: c3, within_: w2 }) => {
                let mut newp: Absyn::Program;
                if classInProgram(metamodelica::AsArg::as_arg(&name), metamodelica::AsArg::as_arg(&p2)) {
                    newp = replaceClassInProgram(c1.clone(), p2.clone(), mergeAST)?;
                } else {
                    newp = Absyn::Program { classes: metamodelica::cons(c1.clone(), c3.clone()), within_: w2.clone() };
                }
                { (inNewClasses, w, inOldProgram, mergeAST, allowFilenameChange) = (c2.clone(), w, newp, mergeAST, allowFilenameChange); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: c1, tail: c2 }, Absyn::Within::WITHIN { .. }, p2) => {
                let mut newp: Absyn::Program;
                let mut newp_1: Absyn::Program;
                newp = insertClassInProgram(c1.clone(), w.clone(), p2.clone(), mergeAST, allowFilenameChange)?;
                { (inNewClasses, w, inOldProgram, mergeAST, allowFilenameChange) = (c2.clone(), w, newp, mergeAST, allowFilenameChange); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn getClassnamesInParts(
    mut inAbsynClassPartLst: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inShowProtected: bool,
    mut includeConstants: bool,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    outStringLst = 'mc: {
        let __mc_input = (&**inAbsynClassPartLst, inShowProtected, includeConstants);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: rest }, b, c) => {
                    let mut l1: metamodelica::List<ArcStr>;
                    let mut l2: metamodelica::List<ArcStr>;
                    let mut res: metamodelica::List<ArcStr>;
                    l1 = getClassnamesInElts(metamodelica::AsArg::as_arg(&elts), c.clone())?;
                    l2 = getClassnamesInParts(metamodelica::AsArg::as_arg(&rest), b.clone(), c.clone())?;
                    res = listAppend(l1.clone(), l2.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: rest }, true, c) => {
                    let mut l1: metamodelica::List<ArcStr>;
                    let mut l2: metamodelica::List<ArcStr>;
                    let mut res: metamodelica::List<ArcStr>;
                    l1 = getClassnamesInElts(metamodelica::AsArg::as_arg(&elts), c.clone())?;
                    l2 = getClassnamesInParts(metamodelica::AsArg::as_arg(&rest), true, c.clone())?;
                    res = listAppend(l1.clone(), l2.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, b, c) => {
                    let mut res: metamodelica::List<ArcStr>;
                    res = getClassnamesInParts(metamodelica::AsArg::as_arg(&rest), b.clone(), c.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStringLst)
}

pub(crate) fn getClassnamesInElts(
    mut inAbsynElementItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut includeConstants: bool,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    let mut delst: DoubleEnded::MutableList<ArcStr>;
    delst = DoubleEnded::fromList(&(metamodelica::nil()))?;
    for mut elt in &**inAbsynElementItemLst {
        let () = (::match_deref::match_deref! { match &(elt.clone()) {
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: id, .. }, .. }, .. }, .. } } => {
                DoubleEnded::push_back(delst.clone(), id.clone())?;
                ()
            },
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name: id, .. }, .. }, .. } } => {
                DoubleEnded::push_back(delst.clone(), id.clone())?;
                ()
            },
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { variability: Absyn::Variability::CONST { .. }, .. }, components: lst, .. }, .. } } if (includeConstants) => {
                DoubleEnded::push_list_back(delst.clone(), &(getComponentItemsName(lst.clone(), false)))?;
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    outStringLst = DoubleEnded::toListAndClear(delst, metamodelica::nil())?;
    Ok(outStringLst)
}

pub fn getComponentItemsName(
    mut inComponents: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut inQuoteNames: bool,
) -> metamodelica::List<ArcStr> {
    let mut outStrings: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut name: ArcStr;
    for mut comp in &*inComponents.reverse() {
        let () = (match &*comp.clone() {
            Absyn::ComponentItem {
                component: Absyn::Component { name: __esc_name, .. },
                ..
            } => {
                name = (*__esc_name).clone();
                outStrings = metamodelica::cons(
                    if (inQuoteNames) {
                        stringAppendList(list![literal!("\""), name.clone(), literal!("\"")])
                    } else {
                        stringAppendList(list![name.clone()])
                    },
                    outStrings,
                );
                ()
            }
            _ => (),
        });
    }
    outStrings
}

pub(crate) fn replaceClassInProgram2(mut inClass: &metamodelica::Ref<Absyn::Class>, mut inClassName: &ArcStr) -> bool {
    let mut outReplace: bool;
    let mut cls_name: ArcStr;
    let __arc1 = &(*inClass);
    let Absyn::CLASS { name: __pa0, .. } = &**__arc1;
    cls_name = metamodelica::Own::own(__pa0);
    outReplace = metamodelica::stringEq(&cls_name, &inClassName);
    outReplace
}

pub(crate) fn replaceClassInProgram(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inProgram: Absyn::Program,
    mut mergeAST: bool,
) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    let mut cls_name1: ArcStr;
    let mut clst: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let mut clsFilter: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let mut w: Absyn::Within;
    let mut replaced: bool;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let __arc1 = inClass.clone();
    let Absyn::CLASS { name: __pa0, .. } = &*__arc1;
    cls_name1 = metamodelica::Own::own(__pa0);
    let Absyn::PROGRAM {
        classes: __pa2,
        within_: __pa3,
    } = inProgram;
    clst = metamodelica::Own::own(__pa2);
    w = metamodelica::Own::own(__pa3);
    if mergeAST {
        clsFilter = List::filterOnTrue(
            clst.clone(),
            (std::sync::Arc::new({
                let __pe_b1 = cls_name1.clone();
                move |__pe_a0| Ok(replaceClassInProgram2(&__pe_a0, &__pe_b1))
            })
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<bool> + 'static>),
        )?;
        if (clsFilter).is_empty() {
            cls = inClass.clone();
        } else {
            let __pa4 = ::match_deref::match_deref! { match &(clsFilter) {
                Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: _ } => __pa4.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cls = metamodelica::Own::own(__pa4);
            cls = mergeClasses(inClass.clone(), &cls)?;
        }
    } else {
        cls = inClass.clone();
    }
    (clst, replaced) = List::replaceOnTrue(
        cls,
        clst,
        &({
            let __pe_b1 = cls_name1;
            move |__pe_a0| Ok(replaceClassInProgram2(&__pe_a0, &__pe_b1))
        }),
    )?;
    if !(replaced) {
        clst = List::appendElt(inClass, clst);
    }
    outProgram = Absyn::Program {
        classes: clst,
        within_: w,
    };
    Ok(outProgram)
}

pub(crate) fn insertClassInProgram(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inWithin: Absyn::Within,
    mut inProgram: Absyn::Program,
    mut mergeAST: bool,
    mut allowFilenameChange: bool,
) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    outProgram = 'mc: {
        let __mc_input = (inClass, inWithin, inProgram);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, w @ Absyn::Within::WITHIN { path: Deref @ Absyn::Path::QUALIFIED { name: n1, .. } }, p @ Absyn::Program { .. }) => {
                    let mut c2: metamodelica::Ref<Absyn::Class>;
                    let mut c3: metamodelica::Ref<Absyn::Class>;
                    let mut pnew: Absyn::Program;
                    c2 = getClassInProgram(metamodelica::AsArg::as_arg(&n1), metamodelica::AsArg::as_arg(&p))?;
                    c3 = insertClassInClass(c1.clone(), metamodelica::AsArg::as_arg(&w), c2.clone(), mergeAST, allowFilenameChange)?;
                    pnew = updateProgram(Absyn::Program { classes: list![c3.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, p.clone(), mergeAST, allowFilenameChange)?;
                    Ok(pnew.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, w @ Absyn::Within::WITHIN { path: Deref @ Absyn::Path::IDENT { name: n1 } }, p @ Absyn::Program { .. }) => {
                    let mut c2: metamodelica::Ref<Absyn::Class>;
                    let mut c3: metamodelica::Ref<Absyn::Class>;
                    let mut pnew: Absyn::Program;
                    c2 = getClassInProgram(metamodelica::AsArg::as_arg(&n1), metamodelica::AsArg::as_arg(&p))?;
                    c3 = insertClassInClass(c1.clone(), metamodelica::AsArg::as_arg(&w), c2.clone(), mergeAST, allowFilenameChange)?;
                    pnew = updateProgram(Absyn::Program { classes: list![c3.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, p.clone(), mergeAST, allowFilenameChange)?;
                    Ok(pnew.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Absyn::Within::WITHIN { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "OpenModelica", .. } }, p) => {
                    Ok(p.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ Absyn::Class { name, .. }, w, p) => {
                            let mut s1: ArcStr;
                            let mut s2: ArcStr;
                            let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                            s1 = Dump::unparseWithin(w.clone())?;
                            (_, paths) = getClassNamesRecursive(None, p.clone(), false, false, metamodelica::nil())?;
                            s2 = stringAppendList(List::map1r(({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut p in (paths.clone()).into_iter().cloned() {
                            let __x = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), &fnptr!(stringAppend, ArcStr, ArcStr), literal!("\n  "))?);
                            Error::addMessage(Error::INSERT_CLASS.clone(), list![name.clone(), s1.clone(), s2.clone()])?;
                            Ok(return Err("fail"))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outProgram)
}

pub(crate) fn insertClassInClass(
    mut inClass1: metamodelica::Ref<Absyn::Class>,
    mut inWithin2: &Absyn::Within,
    mut inClass3: metamodelica::Ref<Absyn::Class>,
    mut mergeAST: bool,
    mut allowFilenameChange: bool,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    outClass = (::match_deref::match_deref! { match &(inWithin2) {
        Absyn::Within::WITHIN { path: Deref @ Absyn::Path::IDENT { .. } } => {
            let mut c1 = inClass1;
            let mut c2 = inClass3;
            replaceInnerClass(c1, c2, mergeAST, allowFilenameChange)?
        },
        Absyn::Within::WITHIN { path: Deref @ Absyn::Path::QUALIFIED { path, .. } } => {
            let mut c1 = inClass1;
            let mut c2 = inClass3;
            let mut cnew: metamodelica::Ref<Absyn::Class>;
            let mut cinner: metamodelica::Ref<Absyn::Class>;
            let mut name2: ArcStr;
            name2 = AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&path));
            cinner = getInnerClass(&c2, name2)?;
            cnew = insertClassInClass(c1, &(Absyn::Within::WITHIN { path: path.clone() }), cinner, mergeAST, allowFilenameChange)?;
            replaceInnerClass(cnew, c2, mergeAST, allowFilenameChange)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outClass)
}

pub(crate) fn replaceInnerClass(
    mut inClass1: metamodelica::Ref<Absyn::Class>,
    mut inClass2: metamodelica::Ref<Absyn::Class>,
    mut mergeAST: bool,
    mut allowFilenameChange: bool,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    let mut enclosingFileName: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(inClass2.clone()) {
        Deref @ Absyn::Class { info: SourceInfo { fileName: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    enclosingFileName = metamodelica::Own::own(__pa0);
    outClass = 'mc: {
        let __mc_input = (inClass1, inClass2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmt }, .. }) => {
                    let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut publst2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut outClass = (*outClass).clone();
                    publst = getPublicList(metamodelica::AsArg::as_arg(&parts));
                    let __pa0 = ::match_deref::match_deref! { match &(replaceClassInElementitemlist(&publst, c1.clone(), mergeAST, allowFilenameChange, &enclosingFileName)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    publst2 = metamodelica::Own::own(__pa0);
                    parts2 = replacePublicList(metamodelica::AsArg::as_arg(&parts), publst2.clone())?;
                    assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    Ok(outClass.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmt }, .. }) => {
                    let mut prolst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut prolst2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut outClass = (*outClass).clone();
                    prolst = getProtectedList(metamodelica::AsArg::as_arg(&parts));
                    let __pa0 = ::match_deref::match_deref! { match &(replaceClassInElementitemlist(&prolst, c1.clone(), mergeAST, allowFilenameChange, &enclosingFileName)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    prolst2 = metamodelica::Own::own(__pa0);
                    parts2 = replaceProtectedList(metamodelica::AsArg::as_arg(&parts), prolst2.clone())?;
                    assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    Ok(outClass.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmt }, .. }) => {
                    let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut outClass = (*outClass).clone();
                    publst = getPublicList(metamodelica::AsArg::as_arg(&parts));
                    publst = addClassInElementitemlist(publst.clone(), c1.clone());
                    parts2 = replacePublicList(metamodelica::AsArg::as_arg(&parts), publst.clone())?;
                    assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    Ok(outClass.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname, modifications: modif, parts, ann, comment: cmt }, .. }) => {
                    let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut publst2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut outClass = (*outClass).clone();
                    publst = getPublicList(metamodelica::AsArg::as_arg(&parts));
                    let __pa0 = ::match_deref::match_deref! { match &(replaceClassInElementitemlist(&publst, c1.clone(), mergeAST, allowFilenameChange, &enclosingFileName)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    publst2 = metamodelica::Own::own(__pa0);
                    parts2 = replacePublicList(metamodelica::AsArg::as_arg(&parts), publst2.clone())?;
                    assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname.clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2.clone(), ann: ann.clone() }));
                    Ok(outClass.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname, modifications: modif, parts, ann, comment: cmt }, .. }) => {
                    let mut prolst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut prolst2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut outClass = (*outClass).clone();
                    prolst = getProtectedList(metamodelica::AsArg::as_arg(&parts));
                    let __pa0 = ::match_deref::match_deref! { match &(replaceClassInElementitemlist(&prolst, c1.clone(), mergeAST, allowFilenameChange, &enclosingFileName)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    prolst2 = metamodelica::Own::own(__pa0);
                    parts2 = replaceProtectedList(metamodelica::AsArg::as_arg(&parts), prolst2.clone())?;
                    assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname.clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2.clone(), ann: ann.clone() }));
                    Ok(outClass.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname, modifications: modif, parts, ann, comment: cmt }, .. }) => {
                    let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut outClass = (*outClass).clone();
                    publst = getPublicList(metamodelica::AsArg::as_arg(&parts));
                    publst = addClassInElementitemlist(publst.clone(), c1.clone());
                    parts2 = replacePublicList(metamodelica::AsArg::as_arg(&parts), publst.clone())?;
                    assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname.clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2.clone(), ann: ann.clone() }));
                    Ok(outClass.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Print::printBuf(literal!("Failed in replaceInnerClass\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outClass)
}

pub(crate) fn replaceClassInElementitemlist(
    mut inAbsynElementItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut mergeAST: bool,
    mut allowFilenameChange: bool,
    mut enclosingFileName: &ArcStr,
) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>, bool)> {
    let mut outAbsynElementItemLst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut replaced: bool;
    (outAbsynElementItemLst, replaced) = (::match_deref::match_deref! { match &((&**inAbsynElementItemLst, inClass)) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: a, redeclareKeywords: b, innerOuter: io, specification: Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: e, class_: c1 @ Deref @ Absyn::Class { name: name1, .. } }, constrainClass: h, .. } }, tail: xs }, c2 @ Deref @ Absyn::Class { name, .. }) if (stringEq(&name1, &name)) => {
            let mut c: metamodelica::Ref<Absyn::Class>;
            let mut oldFileName: ArcStr;
            let mut info: SourceInfo;
            c = if (mergeAST) {mergeClasses(c2.clone(), metamodelica::AsArg::as_arg(&c1))?} else {c2.clone()};
            let __pa0 = ::match_deref::match_deref! { match &(c1.clone()) {
                Deref @ Absyn::Class { info: SourceInfo { fileName: __pa0, .. }, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            oldFileName = metamodelica::Own::own(__pa0);
            if !(allowFilenameChange) && stringEq(&oldFileName, &enclosingFileName) {
                c = AbsynUtil::setClassFilename(c, oldFileName);
            }
            let __arc2 = c.clone();
            let Absyn::CLASS { info: __pa1, .. } = &*__arc2;
            info = metamodelica::Own::own(__pa1);
            (metamodelica::cons(metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: metamodelica::Ref::new(Absyn::Element::ELEMENT { finalPrefix: a.clone(), redeclareKeywords: b.clone(), innerOuter: io.clone(), specification: metamodelica::Ref::new(Absyn::ElementSpec::CLASSDEF { replaceable_: e.clone(), class_: c }), info: info, constrainClass: h.clone() }) }), xs.clone()), true)
        },
        (Deref @ metamodelica::ListNode::Cons { head: e1, tail: xs }, c) => {
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            (res, replaced) = replaceClassInElementitemlist(metamodelica::AsArg::as_arg(&xs), c.clone(), mergeAST, allowFilenameChange, enclosingFileName)?;
            (metamodelica::cons(e1.clone(), res), replaced)
        },
        _ => {
            (metamodelica::nil(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outAbsynElementItemLst, replaced))
}

pub(crate) fn addClassInElementitemlist(
    mut inAbsynElementItemLst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inClass: metamodelica::Ref<Absyn::Class>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> {
    let mut outAbsynElementItemLst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut info: SourceInfo;
    let __arc1 = inClass.clone();
    let Absyn::CLASS { info: __pa0, .. } = &*__arc1;
    info = metamodelica::Own::own(__pa0);
    outAbsynElementItemLst = listAppend(
        inAbsynElementItemLst,
        list![metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM {
            element: metamodelica::Ref::new(Absyn::Element::ELEMENT {
                finalPrefix: false,
                redeclareKeywords: None,
                innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
                specification: metamodelica::Ref::new(Absyn::ElementSpec::CLASSDEF {
                    replaceable_: false,
                    class_: inClass
                }),
                info: info,
                constrainClass: None
            })
        })],
    );
    outAbsynElementItemLst
}

pub(crate) fn getInnerClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inIdent: ArcStr,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    outClass = 'mc: {
        let __mc_input = (&**inClass, inIdent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. }, name) => {
                    let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut c1: metamodelica::Ref<Absyn::Class>;
                    publst = getPublicList(metamodelica::AsArg::as_arg(&parts));
                    c1 = getClassFromElementitemlist(&publst, name.clone())?;
                    Ok(c1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. }, name) => {
                    let mut prolst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut c1: metamodelica::Ref<Absyn::Class>;
                    prolst = getProtectedList(metamodelica::AsArg::as_arg(&parts));
                    c1 = getClassFromElementitemlist(&prolst, name.clone())?;
                    Ok(c1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. }, name) => {
                    let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut c1: metamodelica::Ref<Absyn::Class>;
                    publst = getPublicList(metamodelica::AsArg::as_arg(&parts));
                    c1 = getClassFromElementitemlist(&publst, name.clone())?;
                    Ok(c1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. }, name) => {
                    let mut prolst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut c1: metamodelica::Ref<Absyn::Class>;
                    prolst = getProtectedList(metamodelica::AsArg::as_arg(&parts));
                    c1 = getClassFromElementitemlist(&prolst, name.clone())?;
                    Ok(c1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outClass)
}

pub fn replacePublicList(
    mut inAbsynClassPartLst: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inAbsynElementItemLst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outAbsynClassPartLst: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outAbsynClassPartLst = (::match_deref::match_deref! { match inAbsynClassPartLst {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { .. }, tail: rest } => {
            let mut newpublst = inAbsynElementItemLst;
            let mut rest_1: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            rest_1 = deletePublicList(rest);
            metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: newpublst }), rest_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: x, tail: xs } => {
            let mut new = inAbsynElementItemLst;
            let mut ys: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            ys = replacePublicList(xs, new)?;
            metamodelica::cons(x.clone(), ys)
        },
        Deref @ metamodelica::ListNode::Nil => {
            let mut newpublist = inAbsynElementItemLst;
            list![metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: newpublist })]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAbsynClassPartLst)
}

pub fn replaceProtectedList(
    mut inAbsynClassPartLst: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inAbsynElementItemLst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outAbsynClassPartLst: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outAbsynClassPartLst = (::match_deref::match_deref! { match inAbsynClassPartLst {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { .. }, tail: rest } => {
            let mut newprotlist = inAbsynElementItemLst;
            let mut rest_1: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            rest_1 = deleteProtectedList(rest);
            metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: newprotlist }), rest_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: x, tail: xs } => {
            let mut new = inAbsynElementItemLst;
            let mut ys: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            ys = replaceProtectedList(xs, new)?;
            metamodelica::cons(x.clone(), ys)
        },
        Deref @ metamodelica::ListNode::Nil => {
            let mut newprotlist = inAbsynElementItemLst;
            list![metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: newprotlist })]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAbsynClassPartLst)
}

pub(crate) fn deletePublicList<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { .. }, tail: xs } => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                { inAbsynClassPartLst = xs; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: x, tail: xs } => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                res = deletePublicList(xs);
                return metamodelica::cons(x.clone(), res)
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn deleteProtectedList<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { .. }, tail: xs } => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                { inAbsynClassPartLst = xs; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: x, tail: xs } => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                res = deleteProtectedList(xs);
                return metamodelica::cons(x.clone(), res)
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn getPublicList<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: res1 }, tail: rest } => {
                let mut res2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                res2 = getPublicList(rest);
                return listAppend(res1.clone(), res2)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut ys: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                { inAbsynClassPartLst = xs; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn getProtectedList<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: res1 }, tail: rest } => {
                let mut res2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                res2 = getProtectedList(rest);
                return listAppend(res1.clone(), res2)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut ys: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                { inAbsynClassPartLst = xs; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn getClassFromElementitemlist(
    mut inElements: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inIdent: ArcStr,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    let mut elem: metamodelica::Ref<Absyn::ElementItem>;
    elem = List::getMemberOnTrue(inIdent, inElements, &move |__a0: ArcStr,
                                                             __a1: metamodelica::Ref<
        Absyn::ElementItem,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(classElementItemIsNamed(&__a0, &__a1))
    })?;
    let __pa0 = ::match_deref::match_deref! { match &(elem) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __pa0, .. }, .. } } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outClass = metamodelica::Own::own(__pa0);
    Ok(outClass)
}

pub(crate) fn classInProgram(mut name: &ArcStr, mut p: &Absyn::Program) -> bool {
    let mut b: bool;
    b = (match p.clone() {
        Absyn::Program { .. } => {
            let mut r#str: ArcStr;
            for mut cl in &*p.classes.clone() {
                let __arc1 = cl.clone();
                let Absyn::CLASS { name: __pa0, .. } = &*__arc1;
                r#str = metamodelica::Own::own(__pa0);
                if metamodelica::stringEq(&r#str, &name) {
                    b = true;
                    return b;
                }
            }
            false
        }
    });
    b
}

pub fn getPathedClassInProgram(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: &Absyn::Program,
    mut enclOnErr: bool,
    mut showError: bool,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    outClass = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            Ok(getPathedClassInProgramWork(&inPath, inProgram, enclOnErr)?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            Ok(getPathedClassInProgramWork(
                &inPath,
                &((FBuiltin::getInitialFunctions()?).0),
                enclOnErr,
            )?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if showError {
                Error::addMessage(
                    Error::LOOKUP_ERROR.clone(),
                    list![
                        AbsynUtil::pathString(inPath.clone(), literal!("."), true, false)?,
                        literal!("<TOP>")
                    ],
                )?;
            }
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outClass)
}

pub(crate) fn getPathedClassInProgramWork<'__b>(
    mut inPath: &'__b metamodelica::Ref<Absyn::Path>,
    mut inProgram: &'__b Absyn::Program,
    mut enclOnErr: bool,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    '__tco: loop {
        match &**inPath {
            Absyn::Path::IDENT { .. } => {
                return Ok(getClassInProgram(
                    var_field!((**inPath).name, Absyn::Path::IDENT),
                    inProgram,
                )?);
            }
            Absyn::Path::QUALIFIED { .. } => {
                let mut c: metamodelica::Ref<Absyn::Class>;
                c = getClassInProgram(var_field!((**inPath).name, Absyn::Path::QUALIFIED), inProgram)?;
                return Ok(getPathedClassInClass(
                    var_field!((**inPath).path, Absyn::Path::QUALIFIED),
                    &c,
                    enclOnErr,
                )?);
            }
            Absyn::Path::FULLYQUALIFIED { .. } => {
                (inPath, inProgram, enclOnErr) = (
                    var_field!((**inPath).path, Absyn::Path::FULLYQUALIFIED),
                    inProgram,
                    enclOnErr,
                );
                continue '__tco;
            }
        }
    }
}

pub(crate) fn getPathedClassInClass(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut enclOnError: bool,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    outClass = 'mc: {
        let __mc_input = &**inPath;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { name: r#str } => {
                    Ok(getClassInClass(metamodelica::AsArg::as_arg(&r#str), inClass)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::FULLYQUALIFIED { path } => {
                    Ok(getPathedClassInClass(metamodelica::AsArg::as_arg(&path), inClass, enclOnError)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::QUALIFIED { name: r#str, path } => {
                    let mut c: metamodelica::Ref<Absyn::Class>;
                    c = getClassInClass(metamodelica::AsArg::as_arg(&r#str), inClass)?;
                    Ok(getPathedClassInClass(metamodelica::AsArg::as_arg(&path), &c, enclOnError)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((enclOnError)) { return Err("guard") }
                    Ok(inClass.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outClass)
}

pub(crate) fn getClassInClass(
    mut name: &ArcStr,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    for mut part in &*AbsynUtil::getClassPartsInClass(inClass) {
        for mut item in &*AbsynUtil::getElementItemsInClassPart(metamodelica::AsArg::as_arg(&part)) {
            if AbsynUtil::isElementItemClassNamed(name, metamodelica::AsArg::as_arg(&item)) {
                outClass = AbsynUtil::elementItemClass(metamodelica::AsArg::as_arg(&item))?;
                return Ok(outClass);
            }
        }
    }
    return Err("fail");
    Ok(outClass)
}

pub fn getClassInProgram(mut name: &ArcStr, mut program: &Absyn::Program) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class>;
    cls = List::find(
        &program.classes,
        &({
            let __pe_b0 = name.clone();
            move |__pe_a1| Ok(AbsynUtil::isClassNamed(&__pe_b0, &__pe_a1))
        }),
    )?;
    Ok(cls)
}

pub(crate) fn getClassnamesInClassList(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inProgram: &Absyn::Program,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inShowProtected: bool,
    mut includeConstants: bool,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outString: metamodelica::List<ArcStr>;
    outString = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut b = inShowProtected;
            let mut c = includeConstants;
            let mut strlist: metamodelica::List<ArcStr>;
            strlist = getClassnamesInParts(metamodelica::AsArg::as_arg(&parts), b, c)?;
            strlist
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut b = inShowProtected;
            let mut c = includeConstants;
            let mut strlist: metamodelica::List<ArcStr>;
            strlist = getClassnamesInParts(metamodelica::AsArg::as_arg(&parts), b, c)?;
            strlist
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { .. }, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::OVERLOAD { .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::ENUMERATION { .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PDER { .. }, .. } => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

pub fn getClassNamesRecursive(
    mut inPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut inProgram: Absyn::Program,
    mut inShowProtected: bool,
    mut includeConstants: bool,
    mut inAcc: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> Result<(
    Option<metamodelica::Ref<Absyn::Path>>,
    metamodelica::List<metamodelica::Ref<Absyn::Path>>,
)> {
    let mut opath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    (opath, paths) = 'mc: {
        let __mc_input = (&inPath, inProgram, inShowProtected, includeConstants, inAcc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(pp), p, b, c, acc) => {
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut strlst: metamodelica::List<ArcStr>;
                    let mut result_path_lst: metamodelica::List<Option<metamodelica::Ref<Absyn::Path>>>;
                    let mut acc = (*acc).clone();
                    acc = metamodelica::cons(pp.clone(), acc.clone());
                    cdef = getPathedClassInProgram(pp.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    strlst = getClassnamesInClassList(metamodelica::AsArg::as_arg(&pp), metamodelica::AsArg::as_arg(&p), &cdef, b.clone(), c.clone())?;
                    result_path_lst = List::map(List::map1(strlst.clone(), &joinPaths, pp.clone())?, &fnptr!(Util::makeOption, _))?;
                    (_, acc) = List::map3Fold(&result_path_lst, &getClassNamesRecursive, p.clone(), b.clone(), c.clone(), acc.clone())?;
                    Ok((inPath.clone(), acc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, p @ Absyn::Program { classes, .. }, b, c, acc) => {
                    let mut strlst: metamodelica::List<ArcStr>;
                    let mut result_path_lst: metamodelica::List<Option<metamodelica::Ref<Absyn::Path>>>;
                    let mut acc = (*acc).clone();
                    strlst = List::map(classes.clone(), &move |__a0: metamodelica::Ref<Absyn::Class>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::getClassName(&__a0)) })?;
                    result_path_lst = List::mapMap(strlst.clone(), &fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr), &fnptr!(Util::makeOption, _))?;
                    (_, acc) = List::map3Fold(&result_path_lst, &getClassNamesRecursive, p.clone(), b.clone(), c.clone(), acc.clone())?;
                    Ok((inPath.clone(), acc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(pp), _, _, _, _) => {
                    let mut s1: ArcStr;
                    s1 = AbsynUtil::pathString(pp.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![s1.clone(), literal!("<TOP>")])?;
                    Ok((inPath.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((opath, paths))
}

pub(crate) fn mergeClasses(
    mut cNew: metamodelica::Ref<Absyn::Class>,
    mut cOld: &metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut c: metamodelica::Ref<Absyn::Class>;
    c = 'mc: {
        let __mc_input = (cNew.clone(), &**cOld);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::ListNode::Nil, .. }, .. }) => {
                    Ok(cNew.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::ListNode::Nil, .. }, .. }) => {
                    Ok(cNew.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars: typeVars1, classAttrs: classAttrs1, classParts: partsC1, ann: ann1, comment: cmt1 }, .. }, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: partsC2, .. }, info: SourceInfo { fileName: file, .. }, .. }) => {
                    let mut pubElementsC1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut pubElementsC2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut c = (*c).clone();
                    let mut partsC1 = (*partsC1).clone();
                    pubElementsC2 = getPublicList(metamodelica::AsArg::as_arg(&partsC2));
                    pubElementsC2 = excludeElementsFromFile(file.clone(), pubElementsC2.clone())?;
                    pubElementsC1 = getPublicList(metamodelica::AsArg::as_arg(&partsC1));
                    pubElementsC1 = mergeElements(pubElementsC1.clone(), pubElementsC2.clone())?;
                    partsC1 = replacePublicList(metamodelica::AsArg::as_arg(&partsC1), pubElementsC1.clone())?;
                    assign_field!(c.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars1.clone(), classAttrs: classAttrs1.clone(), classParts: partsC1.clone(), ann: ann1.clone(), comment: cmt1.clone() }));
                    Ok(c.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(cNew.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(c)
}

pub(crate) fn mergeElement(
    mut inEls: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inEl: metamodelica::Ref<Absyn::ElementItem>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    let mut outEls: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    outEls = 'mc: {
        let __mc_input = (&**inEls, inEl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::cons(inEl.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: f, redeclareKeywords: redecl, innerOuter: innout, specification: Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: r, class_: c1 @ Deref @ Absyn::Class { name: n1, .. } }, info: i, constrainClass: cc } }, tail: rest }, Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: _, class_: c2 @ Deref @ Absyn::Class { name: n2, .. } }, .. } }) => {
                    let mut c1 = (*c1).clone();
                    let true = (stringEqual(&n1, &n2)) else { return Err("pattern mismatch") };
                    c1 = mergeClasses(c1.clone(), metamodelica::AsArg::as_arg(&c2))?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: metamodelica::Ref::new(Absyn::Element::ELEMENT { finalPrefix: f.clone(), redeclareKeywords: redecl.clone(), innerOuter: innout.clone(), specification: metamodelica::Ref::new(Absyn::ElementSpec::CLASSDEF { replaceable_: r.clone(), class_: c1.clone() }), info: i.clone(), constrainClass: cc.clone() }) }), rest.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e1, tail: rest }, e2) => {
                    let mut filtered: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    filtered = mergeElement(metamodelica::AsArg::as_arg(&rest), e2.clone())?;
                    Ok(metamodelica::cons(e1.clone(), filtered.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outEls)
}

pub(crate) fn mergeElements(
    mut inEls1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inEls2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inEls1.clone(), inEls2.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(inEls2)
            },
            (_, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(inEls1)
            },
            (_, Deref @ metamodelica::ListNode::Cons { head: e2, tail: rest }) => {
                let mut merged: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                merged = mergeElement(&inEls1, e2.clone())?;
                { (inEls1, inEls2) = (merged, rest.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn excludeElementsFromFile(
    mut inFile: ArcStr,
    mut inEls: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    '__tco: loop {
        ({
            let mut b: bool = false;
            ::match_deref::match_deref! { match &((inFile, inEls)) {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    return Ok(metamodelica::nil())
                },
                (file, Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { info: SourceInfo { fileName: f, .. }, .. } }, tail: rest }) => {
                    let mut filtered: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    b = stringEqual(&file, &f);
                    filtered = excludeElementsFromFile(file.clone(), rest.clone())?;
                    if (!(b)) {return Ok(metamodelica::cons(e.clone(), filtered))} else {return Ok(filtered)}
                },
                (file, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::LEXER_COMMENT { comment: _ }, tail: rest }) => {
                    let mut filtered: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    { (inFile, inEls) = (file.clone(), rest.clone()); continue '__tco; }
                },
                _ => return Err("match: no arm matched"),
            } }
        })
    }
}

pub fn getClassnamesInClass(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inProgram: &Absyn::Program,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inShowProtected: bool,
    mut includeConstants: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    paths = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut b = inShowProtected;
            let mut c = includeConstants;
            let mut strlist: metamodelica::List<ArcStr>;
            strlist = getClassnamesInParts(metamodelica::AsArg::as_arg(&parts), b, c)?;
            List::map(strlist, &fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr))?
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut b = inShowProtected;
            let mut c = includeConstants;
            let mut strlist: metamodelica::List<ArcStr>;
            strlist = getClassnamesInParts(metamodelica::AsArg::as_arg(&parts), b, c)?;
            List::map(strlist, &fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr))?
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: _, arrayDim: _ }, .. }, .. } => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(paths)
}

pub fn classElementItemIsNamed(
    mut inClassName: &ArcStr,
    mut inElement: &metamodelica::Ref<Absyn::ElementItem>,
) -> bool {
    let mut outIsNamed: bool;
    outIsNamed = (::match_deref::match_deref! { match inElement {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name, .. }, .. }, .. } } => {
            metamodelica::stringEq(&inClassName, &name)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsNamed
}

pub fn joinPaths(
    mut child: ArcStr,
    mut parent: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((child, parent)) {
        (c, r) => {
            let mut res: metamodelica::Ref<Absyn::Path>;
            res = AbsynUtil::joinPaths(r.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: c.clone() }))?;
            res
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPath)
}

pub fn getDefaultComponentPrefixesModStr(mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>) -> ArcStr {
    let mut docStr: ArcStr = arcstr::literal!("");
    docStr = 'mc: {
        let __mc_input = r#mod;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: e, .. }, .. }) => {
                    let mut docStr: ArcStr = docStr.clone();
                    docStr = Dump::printExpStr(e.clone())?;
                    Ok((docStr.clone(), docStr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            docStr = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    docStr
}

pub fn getNamedAnnotationExp<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
    mut id: &metamodelica::Ref<Absyn::Path>,
    mut default: Option<T>,
    mut f: &dyn ::std::ops::Fn(Option<metamodelica::Ref<Absyn::Modification>>) -> Result<T>,
) -> Result<T> {
    pub type ModFunc<T: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Option<metamodelica::Ref<Absyn::Modification>>) -> Result<T> + 'static>;

    let mut outString: T;
    outString = 'mc: {
        let __mc_input = (inPath, inProgram, default);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (modelpath, p, _) => {
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut r#str: T;
                    cdef = getPathedClassInProgram(modelpath.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::getNamedAnnotationInClass(&cdef, id, f)) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    r#str = metamodelica::Own::own(__pa0);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(r#str)) => {
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outString)
}

pub fn getFileDir(mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>, mut inProgram: Absyn::Program) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = (inComponentRef, inProgram);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_, p) => {
                    let mut p_class: metamodelica::Ref<Absyn::Path>;
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut filename: ArcStr;
                    let mut pd: ArcStr;
                    let mut dir_1: ArcStr;
                    let mut pd_1: ArcStr;
                    let mut filename_1: metamodelica::List<ArcStr>;
                    let mut dir: metamodelica::List<ArcStr>;
                    p_class = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&class_))?;
                    cdef = getPathedClassInProgram(p_class.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    filename = AbsynUtil::classFilename(&cdef)?;
                    pd = arcstr::literal!(Autoconf::pathDelimiter);
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(pd.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    pd_1 = metamodelica::Own::own(__pa0);
                    filename_1 = Util::stringSplitAtChar(filename.clone(), pd_1.clone())?;
                    dir = List::stripLast(filename_1.clone())?;
                    dir_1 = stringDelimitList(dir.clone(), pd.clone());
                    Ok(dir_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut pd: ArcStr;
                    let mut dir_1: ArcStr;
                    let mut omhome: ArcStr;
                    let mut omhome_1: ArcStr;
                    omhome = Settings::getInstallationDirectoryPath()?;
                    omhome_1 = System::trim(omhome.clone(), literal!("\""));
                    pd = arcstr::literal!(Autoconf::pathDelimiter);
                    dir_1 = stringAppendList(list![literal!("\""), omhome_1.clone(), pd.clone(), literal!("work"), literal!("\"")]);
                    Ok(dir_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

pub fn getFullPathFromUri(mut program: &Absyn::Program, mut uri: ArcStr, mut printError: bool) -> Result<ArcStr> {
    let mut path: ArcStr;
    let mut str1: ArcStr;
    let mut str2: ArcStr;
    let mut str3: ArcStr;
    (str1, str2, str3) = System::uriToClassAndPath(uri)?;
    path = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*getBasePathFromUri(
            &str1,
            str2,
            program,
            Settings::getModelicaPath(Testsuite::isRunning()?)?,
            printError,
        )?);
        __mm_s.push_str(&*str3);
        ArcStr::from(__mm_s)
    };
    Ok(path)
}

pub(crate) fn getBasePathFromUri(
    mut scheme: &ArcStr,
    mut iname: ArcStr,
    mut program: &Absyn::Program,
    mut modelicaPath: ArcStr,
    mut printError: bool,
) -> Result<ArcStr> {
    let mut basePath: ArcStr;
    basePath = 'mc: {
        let __mc_input = (scheme.clone(), iname, modelicaPath, printError);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "modelica://", name, _, _) => {
                    let mut names: metamodelica::List<ArcStr>;
                    let mut mp: ArcStr;
                    let mut bp: ArcStr;
                    let mut fileName: ArcStr;
                    let mut name = (*name).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(System::strtok(name.clone(), literal!("."))) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    name = metamodelica::Own::own(__pa0);
                    names = metamodelica::Own::own(__pa1);
                    let __pa2 = ::match_deref::match_deref! { match &(getPathedClassInProgram(metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), program, false, false)?) {
                        Deref @ Absyn::Class { info: SourceInfo { fileName: __pa2, .. }, .. } => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    fileName = metamodelica::Own::own(__pa2);
                    mp = System::dirname(fileName.clone());
                    bp = findModelicaPath2(&mp, &names, &(literal!("")), true)?;
                    Ok(bp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "modelica://", name, mp, _) => {
                    let mut isDir: bool;
                    let mut mps: metamodelica::List<ArcStr>;
                    let mut names: metamodelica::List<ArcStr>;
                    let mut gd: ArcStr;
                    let mut bp: ArcStr;
                    let mut name = (*name).clone();
                    let mut mp = (*mp).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(System::strtok(name.clone(), literal!("."))) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    name = metamodelica::Own::own(__pa0);
                    names = metamodelica::Own::own(__pa1);
                    if '__try2: {
                        unwrap_break_err!(getPathedClassInProgram(metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), program, false, false), '__try2);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    gd = arcstr::literal!(Autoconf::groupDelimiter);
                    mps = System::strtok(mp.clone(), gd.clone());
                    (mp, name, isDir) = System::getLoadModelPath(name.clone(), list![literal!("default")], mps.clone(), false)?;
                    mp = if (isDir) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*mp); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }} else {mp.clone()};
                    bp = findModelicaPath2(metamodelica::AsArg::as_arg(&mp), &names, &(literal!("")), true)?;
                    Ok(bp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "file://", _, _, _) => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "modelica://", name, mp, true) => {
                    let mut r#str: ArcStr;
                    let mut name = (*name).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(System::strtok(name.clone(), literal!("."))) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    name = metamodelica::Own::own(__pa0);
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Could not resolve modelica://")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(" with OPENMODELICALIBRARY (MODELICAPATH in the language specification): ")); __mm_s.push_str(&*mp); ArcStr::from(__mm_s) };
                    Error::addMessage(Error::COMPILER_ERROR.clone(), list![r#str.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(basePath)
}

pub(crate) fn findModelicaPath(
    mut imps: &metamodelica::List<ArcStr>,
    mut names: &metamodelica::List<ArcStr>,
    mut version: &ArcStr,
) -> Result<ArcStr> {
    let mut basePath: ArcStr;
    basePath = 'mc: {
        let __mc_input = &**imps;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: mp, tail: _ } => {
                    Ok(findModelicaPath2(metamodelica::AsArg::as_arg(&mp), names, version, false)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: mps } => {
                    Ok(findModelicaPath(metamodelica::AsArg::as_arg(&mps), names, version)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(basePath)
}

pub(crate) fn findModelicaPath2(
    mut mp: &ArcStr,
    mut inames: &metamodelica::List<ArcStr>,
    mut version: &ArcStr,
    mut b: bool,
) -> Result<ArcStr> {
    let mut basePath: ArcStr;
    basePath = 'mc: {
        let __mc_input = (&**inames, b);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: name, tail: names }, _) => {
                    let mut file: ArcStr;
                    let false = (stringEq(&version, &(literal!("")))) else { return Err("pattern mismatch") };
                    file = { let mut __mm_s = String::new(); __mm_s.push_str(&*mp); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*version); ArcStr::from(__mm_s) };
                    let true = (System::directoryExists(file.clone())) else { return Err("pattern mismatch") };
                    Ok(findModelicaPath2(&file, metamodelica::AsArg::as_arg(&names), &(literal!("")), true)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: name, tail: _ }, _) => {
                    let mut file: ArcStr;
                    let false = (stringEq(&version, &(literal!("")))) else { return Err("pattern mismatch") };
                    file = { let mut __mm_s = String::new(); __mm_s.push_str(&*mp); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*version); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) };
                    let true = (System::regularFileExists(file.clone())) else { return Err("pattern mismatch") };
                    Ok(mp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: name, tail: names }, _) => {
                    let mut file: ArcStr;
                    file = { let mut __mm_s = String::new(); __mm_s.push_str(&*mp); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) };
                    let true = (System::directoryExists(file.clone())) else { return Err("pattern mismatch") };
                    Ok(findModelicaPath2(&file, metamodelica::AsArg::as_arg(&names), &(literal!("")), true)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: name, tail: _ }, _) => {
                    let mut file: ArcStr;
                    file = { let mut __mm_s = String::new(); __mm_s.push_str(&*mp); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) };
                    let true = (System::regularFileExists(file.clone())) else { return Err("pattern mismatch") };
                    Ok(mp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, true) => {
                    Ok(mp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(basePath)
}
