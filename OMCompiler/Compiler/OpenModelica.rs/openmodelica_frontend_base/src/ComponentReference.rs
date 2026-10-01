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

use crate::Expression;
use crate::ExpressionDump;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::File;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
// do not make this public. instead use the function below.
thread_local! { static __dummyCref_TLS: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("dummy"), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: metamodelica::nil() }); }
pub(crate) fn dummyCref() -> metamodelica::Ref<DAE::ComponentRef> {
    __dummyCref_TLS.with(|__t| __t.clone())
}

pub(crate) fn createEmptyCrefMemory() -> metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut crefMemory: metamodelica::Array<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    crefMemory = arrayCreate(3, metamodelica::nil());
    crefMemory
}

/* **************************************************/
/* generate a ComponentRef */
/* **************************************************/
pub fn makeDummyCref() -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCrefIdent: metamodelica::Ref<DAE::ComponentRef>;
    outCrefIdent = dummyCref().clone();
    outCrefIdent
}

pub fn makeUntypedCrefIdent(mut ident: ArcStr) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCrefIdent: metamodelica::Ref<DAE::ComponentRef>;
    outCrefIdent = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
        ident: ident,
        identType: DAE::T_UNKNOWN_DEFAULT().clone(),
        subscriptLst: metamodelica::nil(),
    });
    outCrefIdent
}

/* **************************************************/
/* transform to other types */
/* **************************************************/
pub fn crefToPath(mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match inComponentRef {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: i, subscriptLst: Deref @ metamodelica::ListNode::Nil, .. } => {
            metamodelica::Ref::new(Absyn::Path::IDENT { name: i.clone() })
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: i, subscriptLst: Deref @ metamodelica::ListNode::Nil, componentRef: c, .. } => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            p = crefToPath(c)?;
            metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: i.clone(), path: p })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn crefToPathIgnoreSubs(
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (match &**inComponentRef {
        DAE::ComponentRef::CREF_IDENT { ident: i, .. } => {
            metamodelica::Ref::new(Absyn::Path::IDENT { name: i.clone() })
        }
        DAE::ComponentRef::CREF_QUAL {
            ident: i,
            componentRef: c,
            ..
        } => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            p = crefToPathIgnoreSubs(c)?;
            metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                name: i.clone(),
                path: p,
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outPath)
}

pub fn pathToCref<'__b>(mut inPath: &'__b metamodelica::Ref<Absyn::Path>) -> metamodelica::Ref<DAE::ComponentRef> {
    '__tco: loop {
        match &**inPath {
            Absyn::Path::IDENT { name: i } => {
                return ComponentReferenceBasics::makeCrefIdent(
                    i.clone(),
                    DAE::T_UNKNOWN_DEFAULT().clone(),
                    metamodelica::nil(),
                );
            }
            Absyn::Path::FULLYQUALIFIED { path: p } => {
                inPath = p;
                continue '__tco;
            }
            Absyn::Path::QUALIFIED { name: i, path: p } => {
                let mut c: metamodelica::Ref<DAE::ComponentRef>;
                c = pathToCref(p);
                return ComponentReferenceBasics::makeCrefQual(
                    i.clone(),
                    DAE::T_UNKNOWN_DEFAULT().clone(),
                    metamodelica::nil(),
                    c,
                );
            }
        }
    }
}

pub fn creffromVar(mut inVar: &metamodelica::Ref<DAE::Var>) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (match &**inVar {
        DAE::Var { name, ty, .. } => {
            ComponentReferenceBasics::makeCrefIdent(name.clone(), ty.clone(), metamodelica::nil())
        }
    });
    outComponentRef
}

pub fn unelabCref(
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<Absyn::ComponentRef>;
    outComponentRef = 'mc: {
        let __mc_input = &**inComponentRef;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, subscriptLst: subs, .. } => {
                    let mut subs_1: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    subs_1 = unelabSubscripts(metamodelica::AsArg::as_arg(&subs))?;
                    Ok(metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: id.clone(), subscripts: subs_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, subscriptLst: subs, componentRef: cr, .. } => {
                    let mut subs_1: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut cr_1: metamodelica::Ref<Absyn::ComponentRef>;
                    cr_1 = unelabCref(metamodelica::AsArg::as_arg(&cr))?;
                    subs_1 = unelabSubscripts(metamodelica::AsArg::as_arg(&subs))?;
                    Ok(metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: id.clone(), subscripts: subs_1.clone(), componentRef: cr_1.clone() }))
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
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ComponentReference.unelabCref failed on: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inComponentRef)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outComponentRef)
}

fn unelabSubscripts(
    mut inSubscriptLst: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>> {
    let mut outAbsynSubscriptLst: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    outAbsynSubscriptLst = (::match_deref::match_deref! { match inSubscriptLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: xs } => {
            let mut xs_1: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            xs_1 = unelabSubscripts(xs)?;
            metamodelica::cons(openmodelica_ast::Absyn::Subscript::interned_NOSUB(), xs_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e }, tail: xs } => {
            let mut xs_1: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            let mut e_1: metamodelica::Ref<Absyn::Exp>;
            xs_1 = unelabSubscripts(xs)?;
            e_1 = Expression::unelabExp(metamodelica::AsArg::as_arg(&e))?;
            metamodelica::cons(metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: e_1 }), xs_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: e }, tail: xs } => {
            let mut xs_1: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            let mut e_1: metamodelica::Ref<Absyn::Exp>;
            xs_1 = unelabSubscripts(xs)?;
            e_1 = Expression::unelabExp(metamodelica::AsArg::as_arg(&e))?;
            metamodelica::cons(metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: e_1 }), xs_1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLE_NONEXP { exp: e }, tail: xs } => {
            let mut xs_1: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            let mut e_1: metamodelica::Ref<Absyn::Exp>;
            xs_1 = unelabSubscripts(xs)?;
            e_1 = Expression::unelabExp(metamodelica::AsArg::as_arg(&e))?;
            metamodelica::cons(metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: e_1 }), xs_1)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outAbsynSubscriptLst)
}

pub fn toExpCref<'__b>(
    mut absynCref: &'__b metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    '__tco: loop {
        match &**absynCref {
            Absyn::ComponentRef::CREF_IDENT { .. } => {
                return Ok(ComponentReferenceBasics::makeCrefIdent(
                    var_field!((**absynCref).name, Absyn::ComponentRef::CREF_IDENT).clone(),
                    DAE::T_UNKNOWN_DEFAULT().clone(),
                    toExpCrefSubs(var_field!((**absynCref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone())?,
                ));
            }
            Absyn::ComponentRef::CREF_QUAL { .. } => {
                return Ok(ComponentReferenceBasics::makeCrefQual(
                    var_field!((**absynCref).name, Absyn::ComponentRef::CREF_QUAL).clone(),
                    DAE::T_UNKNOWN_DEFAULT().clone(),
                    toExpCrefSubs(var_field!((**absynCref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone())?,
                    toExpCref(var_field!((**absynCref).componentRef, Absyn::ComponentRef::CREF_QUAL))?,
                ));
            }
            Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
                absynCref = var_field!((**absynCref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED);
                continue '__tco;
            }
            Absyn::ComponentRef::WILD { .. } => {
                return Ok(openmodelica_frontend_types::DAE::ComponentRef::interned_WILD());
            }
            Absyn::ComponentRef::ALLWILD { .. } => {
                return Ok(openmodelica_frontend_types::DAE::ComponentRef::interned_WILD());
            }
        }
    }
}

fn toExpCrefSubs(
    mut absynSubs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    let mut daeSubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    daeSubs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
        for mut sub in (absynSubs).into_iter().cloned() {
            let __x = (match &*sub.clone() {
                Absyn::Subscript::SUBSCRIPT {
                    subscript: __sub_subscript,
                } => metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: Expression::fromAbsynExp(__sub_subscript.clone())?,
                }),
                Absyn::Subscript::NOSUB { .. } => openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(daeSubs)
}

pub fn crefStr(mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = stringDelimitList(
        toStringList(inComponentRef),
        if (Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?) {
            literal!("__")
        } else {
            literal!(".")
        },
    );
    Ok(outString)
}

pub(crate) fn crefListStr(mut crList: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<ArcStr> {
    let mut outString: ArcStr = literal!("");
    for mut cr in &**crList {
        outString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*outString);
            __mm_s.push_str(&*crefStr(metamodelica::AsArg::as_arg(&cr))?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(outString)
}

pub fn crefModelicaStr(mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>) -> ArcStr {
    let mut outString: ArcStr;
    outString = stringDelimitList(toStringList(inComponentRef), literal!("_"));
    outString
}

pub(crate) fn printComponentRefOptStr(
    mut inComponentRefOpt: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inComponentRefOpt) {
        None => {
            literal!("NONE()")
        },
        Some(cref) => {
            let mut r#str: ArcStr;
            r#str = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cref))?;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SOME(")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub(crate) fn printComponentRefStrFixDollarDer(
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inComponentRef {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$DER", subscriptLst: Deref @ metamodelica::ListNode::Nil, componentRef: cr, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("der(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => {
            ComponentReferenceBasics::printComponentRefStr(inComponentRef)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub fn debugPrintComponentRefTypeStr(mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inComponentRef {
        DAE::ComponentRef::WILD { .. } => {
            literal!("_")
        }
        DAE::ComponentRef::CREF_IDENT {
            ident: s,
            identType: ty,
            subscriptLst: subs,
        } => {
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            let mut str_1: ArcStr;
            str_1 = ExpressionBasics::printListStr(
                subs.clone(),
                &move |__a0: metamodelica::Ref<DAE::Subscript>| ExpressionDump::debugPrintSubscriptStr(&__a0),
                literal!(", "),
            )?;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*if (((str_1).len() as i32) > 0) {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("["));
                        __mm_s.push_str(&*str_1);
                        __mm_s.push_str(&*literal!("]"));
                        ArcStr::from(__mm_s)
                    }
                } else {
                    literal!("")
                });
                ArcStr::from(__mm_s)
            };
            str2 = TypesDump::unparseType(ty.clone())?;
            r#str = stringAppendList(list![r#str, literal!(" ["), str2, literal!("]")]);
            r#str
        }
        DAE::ComponentRef::CREF_QUAL {
            ident: s,
            identType: ty,
            subscriptLst: subs,
            componentRef: cr,
        } => {
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            let mut strrest: ArcStr;
            let mut str_1: ArcStr;
            if Config::modelicaOutput()? {
                r#str = ComponentReferenceBasics::printComponentRef2Str(s.clone(), subs.clone())?;
                str2 = TypesDump::unparseType(ty.clone())?;
                strrest = debugPrintComponentRefTypeStr(cr)?;
                r#str = stringAppendList(list![
                    r#str,
                    literal!(" ["),
                    str2,
                    literal!("] "),
                    literal!("__"),
                    strrest
                ]);
            } else {
                str_1 = ExpressionBasics::printListStr(
                    subs.clone(),
                    &move |__a0: metamodelica::Ref<DAE::Subscript>| ExpressionDump::debugPrintSubscriptStr(&__a0),
                    literal!(", "),
                )?;
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*s);
                    __mm_s.push_str(&*if (((str_1).len() as i32) > 0) {
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("["));
                            __mm_s.push_str(&*str_1);
                            __mm_s.push_str(&*literal!("]"));
                            ArcStr::from(__mm_s)
                        }
                    } else {
                        literal!("")
                    });
                    ArcStr::from(__mm_s)
                };
                str2 = TypesDump::unparseType(ty.clone())?;
                strrest = debugPrintComponentRefTypeStr(cr)?;
                r#str = stringAppendList(list![
                    r#str,
                    literal!(" ["),
                    str2,
                    literal!("] "),
                    literal!("."),
                    strrest
                ]);
            }
            r#str
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub fn crefIsIdent(mut cr: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut res: bool;
    res = (match &**cr {
        DAE::ComponentRef::CREF_IDENT { .. } => true,
        _ => false,
    });
    res
}

pub fn crefIsNotIdent(mut cr: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut res: bool;
    res = (match &**cr {
        DAE::ComponentRef::CREF_IDENT { .. } => false,
        _ => true,
    });
    res
}

pub fn isInternalCref(mut cr: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut b: bool;
    let mut s: ArcStr;
    b = (::match_deref::match_deref! { match cr {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$DER", .. } => false,
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$CLKPRE", .. } => false,
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_s, .. } => {
            s = (*__esc_s).clone();
            StringUtil::startsWith(s.clone(), literal!("$outputAlias_"))
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_s, .. } => {
            s = (*__esc_s).clone();
            StringUtil::startsWith(s.clone(), literal!("$"))
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: __esc_s, .. } => {
            s = (*__esc_s).clone();
            StringUtil::startsWith(s.clone(), literal!("$"))
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isRecord<'__b>(mut cr: &'__b metamodelica::Ref<DAE::ComponentRef>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match cr {
            Deref @ DAE::ComponentRef::CREF_IDENT { identType: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. } => {
                return true
            },
            Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: comp, .. } => {
                { cr = comp; continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn isArrayElement<'__b>(mut cr: &'__b metamodelica::Ref<DAE::ComponentRef>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match cr {
            Deref @ DAE::ComponentRef::CREF_IDENT { identType: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
                return true
            },
            Deref @ DAE::ComponentRef::CREF_QUAL { identType: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
                return true
            },
            Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: comp, .. } => {
                { cr = comp; continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn isPreCref(mut cr: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match cr {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$PRE", .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isPreviousCref(mut cr: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match cr {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$CLKPRE", .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isStartCref(mut cr: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match cr {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$START", .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn popPreCref(mut inCR: metamodelica::Ref<DAE::ComponentRef>) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCR: metamodelica::Ref<DAE::ComponentRef>;
    outCR = (::match_deref::match_deref! { match &(inCR.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$PRE", componentRef: cr, .. } => {
            cr.clone()
        },
        _ => {
            inCR
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCR
}

pub fn popCref(mut inCR: metamodelica::Ref<DAE::ComponentRef>) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCR: metamodelica::Ref<DAE::ComponentRef>;
    outCR = (match &*inCR {
        DAE::ComponentRef::CREF_QUAL { componentRef: cr, .. } => cr.clone(),
        _ => inCR,
    });
    outCR
}

pub(crate) fn crefIsFirstArrayElt(mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut outBoolean: bool;
    outBoolean = 'mc: {
        let __mc_input = inComponentRef;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cr => {
                    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    if stringEqual(&(Config::simCodeTarget()?), &(literal!("Cpp"))) {
                        let __pa0 = ::match_deref::match_deref! { match &(crefLastSubs(metamodelica::AsArg::as_arg(&cr))?) {
                            __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        subs = metamodelica::Own::own(__pa0);
                    } else {
                        let __pa1 = ::match_deref::match_deref! { match &(ComponentReferenceBasics::crefSubs(metamodelica::AsArg::as_arg(&cr))?) {
                            __pa1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => __pa1.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        subs = metamodelica::Own::own(__pa1);
                    }
                    Ok(List::all(&subs, &move |__a0: metamodelica::Ref<DAE::Subscript>| Expression::subscriptIsFirst(&__a0))?)
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
    outBoolean
}

pub fn crefHaveSubs(mut icr: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut ob: bool;
    ob = 'mc: {
        let __mc_input = &**icr;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { subscriptLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. } => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. } => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: r#str, subscriptLst: Deref @ metamodelica::ListNode::Nil, .. } => {
                    let mut idx: i32;
                    idx = System::stringFind(r#str.clone(), literal!("["))?;
                    let true = (idx > 0) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { subscriptLst: Deref @ metamodelica::ListNode::Nil, componentRef: cr, .. } => {
                    let mut b: bool;
                    b = crefHaveSubs(metamodelica::AsArg::as_arg(&cr));
                    Ok(b)
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
    ob
}

pub fn crefHasScalarSubscripts(mut cr: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut hasScalarSubs: bool;
    hasScalarSubs = 'mc: {
        let __mc_input = &**cr;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ::match_deref::match_deref! { match &(crefLastSubs(cr)?) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
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
                    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let __pa0 = ::match_deref::match_deref! { match &(crefLastSubs(cr)?) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    subs = metamodelica::Own::own(__pa0);
                    let true = (Expression::subscriptConstants(&subs)) else { return Err("pattern mismatch") };
                    tp = crefLastType(cr)?;
                    dims = Expression::arrayDimension(&tp);
                    let true = (((dims).len() as i32) <= ((subs).len() as i32)) else { return Err("pattern mismatch") };
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
    hasScalarSubs
}

pub fn crefIsScalarWithAllConstSubs(mut inCref: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut isScalar: bool;
    isScalar = 'mc: {
        let __mc_input = &**inCref;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ::match_deref::match_deref! { match &(ComponentReferenceBasics::crefSubs(inCref)?) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
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
                    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let __pa0 = ::match_deref::match_deref! { match &(ComponentReferenceBasics::crefSubs(inCref)?) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    subs = metamodelica::Own::own(__pa0);
                    dims = ComponentReferenceBasics::crefDims(inCref)?;
                    let true = (((dims).len() as i32) <= ((subs).len() as i32)) else { return Err("pattern mismatch") };
                    let true = (Expression::subscriptConstants(&subs)) else { return Err("pattern mismatch") };
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
    isScalar
}

pub fn crefIsScalarWithVariableSubs(mut inCref: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut isScalar: bool;
    isScalar = 'mc: {
        let __mc_input = &**inCref;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let __pa0 = ::match_deref::match_deref! { match &(ComponentReferenceBasics::crefSubs(inCref)?) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    subs = metamodelica::Own::own(__pa0);
                    dims = ComponentReferenceBasics::crefDims(inCref)?;
                    let true = (((dims).len() as i32) <= ((subs).len() as i32)) else { return Err("pattern mismatch") };
                    let false = (Expression::subscriptConstants(&subs)) else { return Err("pattern mismatch") };
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
    isScalar
}

pub(crate) fn containWholeDim<'__b>(mut inRef: &'__b metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> {
    let mut wholedim: bool;
    wholedim = (match &**inRef {
        DAE::ComponentRef::CREF_IDENT {
            ident: _,
            identType: ty,
            subscriptLst: ssl,
        } => {
            wholedim = containWholeDim2(ssl, ty)?;
            wholedim
        }
        DAE::ComponentRef::CREF_QUAL {
            ident: _,
            identType: _,
            subscriptLst: _,
            componentRef: cr,
        } => {
            wholedim = containWholeDim(cr)?;
            wholedim
        }
        _ => false,
    });
    Ok(wholedim)
}

pub fn traverseCref<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, Type_a) -> Result<Type_a>,
    mut argIn: Type_a,
) -> Result<Type_a> {
    pub type FuncType<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>;

    let mut argOut: Type_a;
    argOut = 'mc: {
        let __mc_input = &**cref;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ } => {
                    let mut arg: Type_a;
                    arg = func(cref.clone(), argIn.clone())?;
                    Ok(arg.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, identType: _, subscriptLst: _, componentRef: cr } => {
                    let mut arg: Type_a;
                    arg = func(cref.clone(), argIn.clone())?;
                    Ok(traverseCref(metamodelica::AsArg::as_arg(&cr), func, arg.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("traverseCref failed!"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(argOut)
}

pub fn crefIsRec(mut cref: &metamodelica::Ref<DAE::ComponentRef>, mut isRecIn: bool) -> Result<bool> {
    let mut isRec: bool;
    isRec = isRecIn || Types::isRecord(&(crefLastType(cref)?));
    Ok(isRec)
}

pub fn crefGetFirstRec(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(metamodelica::Ref<DAE::ComponentRef>, bool)> {
    let mut result: metamodelica::Ref<DAE::ComponentRef>;
    let mut isRec: bool;
    (result, isRec) = (match &**cref {
        DAE::ComponentRef::CREF_IDENT { .. } => (cref.clone(), Types::isRecord(&(crefType(cref)?))),
        DAE::ComponentRef::CREF_QUAL {
            componentRef: __cref_componentRef,
            ident: __cref_ident,
            identType: __cref_identType,
            subscriptLst: __cref_subscriptLst,
        } => {
            let mut innerCref: metamodelica::Ref<DAE::ComponentRef>;
            if Types::isRecord(&(crefType(cref)?)) {
                result = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: __cref_ident.clone(),
                    identType: __cref_identType.clone(),
                    subscriptLst: __cref_subscriptLst.clone(),
                });
                isRec = true;
            } else {
                (innerCref, isRec) = crefGetFirstRec(metamodelica::AsArg::as_arg(&__cref_componentRef))?;
                result = metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                    ident: __cref_ident.clone(),
                    identType: __cref_identType.clone(),
                    subscriptLst: __cref_subscriptLst.clone(),
                    componentRef: innerCref,
                });
            }
            (result, isRec)
        }
        _ => (cref.clone(), false),
    });
    Ok((result, isRec))
}

fn containWholeDim2(
    mut inRef: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> Result<bool> {
    let mut wholedim: bool = false;
    wholedim = 'mc: {
        let __mc_input = (&**inRef, &**inType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: _ }, Deref @ DAE::Type::T_ARRAY { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: es1 }, tail: _ }, Deref @ DAE::Type::T_ARRAY { dims: ad, .. }) => {
                    let true = (containWholeDim3(metamodelica::AsArg::as_arg(&es1), metamodelica::AsArg::as_arg(&ad))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: ssl }, Deref @ DAE::Type::T_ARRAY { ty: tty, dims: ad }) => {
                    let mut b: bool;
                    let mut ad = (*ad).clone();
                    ad = List::restOrEmpty(ad.clone())?;
                    b = containWholeDim2(metamodelica::AsArg::as_arg(&ssl), &(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: tty.clone(), dims: ad.clone() })))?;
                    Ok(b)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: ssl }, _) => {
                    let mut wholedim: bool = wholedim.clone();
                    wholedim = containWholeDim2(metamodelica::AsArg::as_arg(&ssl), inType)?;
                    Ok((wholedim, wholedim.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            wholedim = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(wholedim)
}

fn containWholeDim3(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut ad: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> bool {
    let mut ob: bool;
    ob = 'mc: {
        let __mc_input = (&**inExp, &**ad);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: expl, .. }, Deref @ metamodelica::ListNode::Cons { head: d, tail: _ }) => {
                    let mut x1: i32;
                    let mut x2: i32;
                    x1 = ((expl).len() as i32);
                    x2 = Expression::dimensionSize(metamodelica::AsArg::as_arg(&d))?;
                    let true = (intEq(x1, x2)) else { return Err("pattern mismatch") };
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
    ob
}

/* **************************************************/
/* Getter  */
/* **************************************************/
pub fn crefArrayGetFirstCref(
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (match &**inComponentRef {
        DAE::ComponentRef::CREF_IDENT {
            ident: i,
            identType: ty,
            subscriptLst: subs,
        } => {
            let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut newsubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut diff: i32;
            dims = TypesDump::getDimensions(ty);
            diff = ((dims).len() as i32) - ((subs).len() as i32);
            newsubs = List::fill(
                metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }),
                }),
                diff,
            );
            metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: i.clone(),
                identType: ty.clone(),
                subscriptLst: listAppend(subs.clone(), newsubs),
            })
        }
        DAE::ComponentRef::CREF_QUAL {
            ident: i,
            identType: ty,
            subscriptLst: subs,
            componentRef: cr,
        } => {
            let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut newsubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut diff: i32;
            let mut cr = (*cr).clone();
            dims = TypesDump::getDimensions(ty);
            diff = ((dims).len() as i32) - ((subs).len() as i32);
            newsubs = List::fill(
                metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }),
                }),
                diff,
            );
            cr = crefArrayGetFirstCref(metamodelica::AsArg::as_arg(&cr))?;
            metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                ident: i.clone(),
                identType: ty.clone(),
                subscriptLst: listAppend(subs.clone(), newsubs),
                componentRef: cr.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outComponentRef)
}

pub fn crefLastPath<'__b>(
    mut inComponentRef: &'__b metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inComponentRef {
            Deref @ DAE::ComponentRef::CREF_IDENT { ident: i, subscriptLst: Deref @ metamodelica::ListNode::Nil, .. } => {
                return Ok(metamodelica::Ref::new(Absyn::Path::IDENT { name: i.clone() }))
            },
            Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: c, subscriptLst: Deref @ metamodelica::ListNode::Nil, .. } => {
                { inComponentRef = c; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn crefRest(mut inCref: &metamodelica::Ref<DAE::ComponentRef>) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &((*inCref)) {
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outCref = metamodelica::Own::own(__pa0);
    Ok(outCref)
}

fn crefTypeFullComputeDims(
    mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inSubs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Dimension>>> {
    let mut outDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut dim: metamodelica::Ref<DAE::Dimension>;
    let mut slice_dim: metamodelica::Ref<DAE::Dimension>;
    dims = inDims;
    outDims = metamodelica::nil();
    for mut sub in &**inSubs {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dims) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        dim = metamodelica::Own::own(__pa0);
        dims = metamodelica::Own::own(__pa1);
        let () = (match &*sub.clone() {
            DAE::Subscript::INDEX { .. } => (),
            DAE::Subscript::SLICE { exp: __sub_exp } => {
                let __pa0 = ::match_deref::match_deref! { match &(TypesDump::getDimensions(&(Expression::r#typeof(__sub_exp.clone())?))) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                slice_dim = metamodelica::Own::own(__pa0);
                outDims = metamodelica::cons(slice_dim, outDims);
                ()
            }
            DAE::Subscript::WHOLEDIM { .. } => {
                outDims = metamodelica::cons(dim, outDims);
                ()
            }
            _ => return Err("match: no arm matched"),
        });
    }
    outDims = listAppend(outDims, dims);
    Ok(outDims)
}

pub fn crefTypeFull2<'__b>(
    mut inCref: &'__b metamodelica::Ref<DAE::ComponentRef>,
    mut accumDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<(
    metamodelica::Ref<DAE::Type>,
    metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
)> {
    '__tco: loop {
        match &**inCref {
            DAE::ComponentRef::CREF_IDENT {
                identType: ty,
                subscriptLst: subs,
                ..
            } => {
                let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                let mut ty = (*ty).clone();
                (ty, dims) = TypesDump::flattenArrayType(metamodelica::AsArg::as_arg(&ty));
                dims = crefTypeFullComputeDims(dims, subs)?;
                if !((accumDims).is_empty()) {
                    dims = List::append_reverse(&dims, accumDims).reverse();
                }
                return Ok((ty.clone(), dims));
            }
            DAE::ComponentRef::CREF_QUAL {
                identType: ty,
                subscriptLst: subs,
                componentRef: cr,
                ..
            } => {
                let mut basety: metamodelica::Ref<DAE::Type>;
                let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                let mut ty = (*ty).clone();
                (ty, dims) = TypesDump::flattenArrayType(metamodelica::AsArg::as_arg(&ty));
                dims = crefTypeFullComputeDims(dims, subs)?;
                {
                    (inCref, accumDims) = (cr, List::append_reverse(&dims, accumDims));
                    continue '__tco;
                }
            }
            _ => {
                let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                    return Err("pattern mismatch");
                };
                Debug::trace(literal!("ComponentReference.crefTypeFull2 failed on cref: "))?;
                Debug::traceln(ComponentReferenceBasics::printComponentRefStr(inCref)?)?;
                return Ok(return Err("fail"));
            }
        }
    }
}

pub fn crefTypeFull(mut inCref: &metamodelica::Ref<DAE::ComponentRef>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    (ty, dims) = crefTypeFull2(inCref, metamodelica::nil())?;
    if (dims).is_empty() {
        outType = ty;
    } else {
        outType = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty, dims: dims });
    }
    Ok(outType)
}

pub fn crefType(mut inCref: &metamodelica::Ref<DAE::ComponentRef>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &**inCref {
        DAE::ComponentRef::CREF_IDENT { identType: ty, .. } => ty.clone(),
        DAE::ComponentRef::CREF_QUAL { identType: ty, .. } => ty.clone(),
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("ComponentReference.crefType failed on cref: "))?;
            Debug::traceln(ComponentReferenceBasics::printComponentRefStr(inCref)?)?;
            return Err("fail");
        }
    });
    Ok(outType)
}

pub fn crefLastType<'__b>(
    mut inRef: &'__b metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    '__tco: loop {
        match &**inRef {
            DAE::ComponentRef::CREF_IDENT {
                ident: _,
                identType: t2,
                subscriptLst: _,
            } => return Ok(t2.clone()),
            DAE::ComponentRef::CREF_QUAL {
                ident: _,
                identType: _,
                subscriptLst: _,
                componentRef: cr,
            } => {
                inRef = cr;
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn crefFirstSubs(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> metamodelica::List<metamodelica::Ref<DAE::Subscript>> {
    let mut outSubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    outSubscripts = (match &**inCref {
        DAE::ComponentRef::CREF_IDENT {
            subscriptLst: __inCref_subscriptLst,
            ..
        } => __inCref_subscriptLst.clone(),
        DAE::ComponentRef::CREF_QUAL {
            subscriptLst: __inCref_subscriptLst,
            ..
        } => __inCref_subscriptLst.clone(),
        _ => metamodelica::nil(),
    });
    outSubscripts
}

pub fn crefLastSubs<'__b>(
    mut inComponentRef: &'__b metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    '__tco: loop {
        match &**inComponentRef {
            DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, .. } => return Ok(subs.clone()),
            DAE::ComponentRef::CREF_QUAL { componentRef: cr, .. } => {
                inComponentRef = cr;
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn crefTypeConsiderSubs(mut cr: &metamodelica::Ref<DAE::ComponentRef>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut res: metamodelica::Ref<DAE::Type>;
    res = Expression::unliftArrayTypeWithSubs(crefLastSubs(cr)?, crefLastType(cr)?)?;
    Ok(res)
}

pub(crate) fn crefNameType(
    mut inRef: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(ArcStr, metamodelica::Ref<DAE::Type>)> {
    let mut id: ArcStr;
    let mut res: metamodelica::Ref<DAE::Type>;
    (id, res) = (match &**inRef {
        DAE::ComponentRef::CREF_IDENT {
            ident: name,
            identType: t2,
            subscriptLst: _,
        } => (name.clone(), t2.clone()),
        DAE::ComponentRef::CREF_QUAL {
            ident: name,
            identType: t2,
            subscriptLst: _,
            componentRef: _,
        } => (name.clone(), t2.clone()),
        _ => {
            let mut s: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("-ComponentReference.crefType failed on Cref:"))?;
            s = ComponentReferenceBasics::printComponentRefStr(inRef)?;
            Debug::traceln(s)?;
            return Err("fail");
        }
    });
    Ok((id, res))
}

pub fn getArrayCref(mut name: metamodelica::Ref<DAE::ComponentRef>) -> Option<metamodelica::Ref<DAE::ComponentRef>> {
    let mut arrayCref: Option<metamodelica::Ref<DAE::ComponentRef>>;
    arrayCref = 'mc: {
        let __mc_input = &*name;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut arrayCrefInner: metamodelica::Ref<DAE::ComponentRef>;
                    let true = (crefIsFirstArrayElt(name.clone())) else { return Err("pattern mismatch") };
                    if stringEqual(&(Config::simCodeTarget()?), &(literal!("Cpp"))) {
                        arrayCrefInner = ComponentReferenceBasics::crefStripLastSubs(&name)?;
                    } else {
                        arrayCrefInner = crefStripSubs(&name)?;
                    }
                    Ok(Some(arrayCrefInner.clone()))
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
    arrayCref
}

pub(crate) fn getArraySubs(
    mut name: &metamodelica::Ref<DAE::ComponentRef>,
) -> metamodelica::List<metamodelica::Ref<DAE::Subscript>> {
    let mut arraySubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    arraySubs = 'mc: {
        let __mc_input = &**name;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut arrayCrefSubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    arrayCrefSubs = ComponentReferenceBasics::crefSubs(name)?;
                    Ok(arrayCrefSubs.clone())
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
    arraySubs
}

/* **************************************************/
/* Change  */
/* **************************************************/
pub fn crefPrependIdent(
    mut icr: &metamodelica::Ref<DAE::ComponentRef>,
    mut ident: &ArcStr,
    mut subs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut tp: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut newCr: metamodelica::Ref<DAE::ComponentRef>;
    newCr = (match &**icr {
        DAE::ComponentRef::CREF_IDENT {
            ident: id1,
            identType: tp1,
            subscriptLst: subs1,
        } => ComponentReferenceBasics::makeCrefQual(
            id1.clone(),
            tp1.clone(),
            subs1.clone(),
            ComponentReferenceBasics::makeCrefIdent(ident.clone(), tp.clone(), subs.clone()),
        ),
        DAE::ComponentRef::CREF_QUAL {
            ident: id1,
            identType: tp1,
            subscriptLst: subs1,
            componentRef: cr,
        } => {
            let mut cr = (*cr).clone();
            cr = crefPrependIdent(metamodelica::AsArg::as_arg(&cr), ident, subs, tp)?;
            ComponentReferenceBasics::makeCrefQual(id1.clone(), tp1.clone(), subs1.clone(), cr.clone())
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(newCr)
}

pub fn crefPrefixDer(mut inCref: metamodelica::Ref<DAE::ComponentRef>) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = ComponentReferenceBasics::makeCrefQual(
        arcstr::literal!(DAE::derivativeNamePrefix),
        DAE::T_REAL_DEFAULT().clone(),
        metamodelica::nil(),
        inCref,
    );
    outCref
}

pub fn crefPrefixPre(mut inCref: metamodelica::Ref<DAE::ComponentRef>) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = ComponentReferenceBasics::makeCrefQual(
        arcstr::literal!(DAE::preNamePrefix),
        DAE::T_UNKNOWN_DEFAULT().clone(),
        metamodelica::nil(),
        inCref,
    );
    outCref
}

pub fn getConcealedCref() -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut ident: ArcStr;
    ident = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("$concealed"));
        __mm_s.push_str(&*intString(System::tmpTick() + 1));
        ArcStr::from(__mm_s)
    };
    outCref = ComponentReferenceBasics::makeCrefIdent(ident, DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
    outCref
}

pub fn crefPrefixPrevious(mut inCref: metamodelica::Ref<DAE::ComponentRef>) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = ComponentReferenceBasics::makeCrefQual(
        arcstr::literal!(DAE::previousNamePrefix),
        DAE::T_UNKNOWN_DEFAULT().clone(),
        metamodelica::nil(),
        inCref,
    );
    outCref
}

pub fn crefPrefixAux(mut inCref: metamodelica::Ref<DAE::ComponentRef>) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = ComponentReferenceBasics::makeCrefQual(
        arcstr::literal!(DAE::auxNamePrefix),
        DAE::T_REAL_DEFAULT().clone(),
        metamodelica::nil(),
        inCref,
    );
    outCref
}

pub fn crefRemovePrePrefix(mut cref: metamodelica::Ref<DAE::ComponentRef>) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut cref: metamodelica::Ref<DAE::ComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$PRE", componentRef: __cref_componentRef, .. } => __cref_componentRef.clone(),
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$START", componentRef: __cref_componentRef, .. } => __cref_componentRef.clone(),
        _ => cref,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cref
}

pub fn crefPrefixStart(mut inCref: metamodelica::Ref<DAE::ComponentRef>) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = ComponentReferenceBasics::makeCrefQual(
        arcstr::literal!(DAE::startNamePrefix),
        DAE::T_UNKNOWN_DEFAULT().clone(),
        metamodelica::nil(),
        inCref,
    );
    outCref
}

pub fn crefPrefixString(
    mut inString: ArcStr,
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref =
        ComponentReferenceBasics::makeCrefQual(inString, DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil(), inCref);
    outCref
}

pub(crate) fn crefPrefixStringList(
    mut inStrings: &metamodelica::List<ArcStr>,
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (::match_deref::match_deref! { match inStrings {
        Deref @ metamodelica::ListNode::Cons { head: r#str, tail: rest_str } => {
            let mut cref = inCref.clone();
            cref = crefPrefixStringList(rest_str, cref);
            cref = crefPrefixString(r#str.clone(), cref);
            cref
        },
        _ => {
            inCref
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCref
}

pub(crate) fn prefixWithPath<'__b>(
    mut inCref: &'__b metamodelica::Ref<DAE::ComponentRef>,
    mut inPath: &'__b metamodelica::Ref<Absyn::Path>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    '__tco: loop {
        match &**inPath {
            Absyn::Path::IDENT { name } => {
                return metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                    ident: name.clone(),
                    identType: DAE::T_UNKNOWN_DEFAULT().clone(),
                    subscriptLst: metamodelica::nil(),
                    componentRef: inCref.clone(),
                });
            }
            Absyn::Path::QUALIFIED { name, path: rest_path } => {
                let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                cref = prefixWithPath(inCref, rest_path);
                return metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                    ident: name.clone(),
                    identType: DAE::T_UNKNOWN_DEFAULT().clone(),
                    subscriptLst: metamodelica::nil(),
                    componentRef: cref,
                });
            }
            Absyn::Path::FULLYQUALIFIED { path: rest_path } => {
                (inCref, inPath) = (inCref, rest_path);
                continue '__tco;
            }
        }
    }
}

pub fn prependStringCref(
    mut inString: ArcStr,
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (match &**inComponentRef {
        DAE::ComponentRef::CREF_QUAL {
            ident: i,
            identType: t2,
            subscriptLst: s,
            componentRef: c,
        } => {
            let mut p = inString;
            let mut i_1: ArcStr;
            i_1 = stringAppend(p, i.clone());
            ComponentReferenceBasics::makeCrefQual(i_1, t2.clone(), s.clone(), c.clone())
        }
        DAE::ComponentRef::CREF_IDENT {
            ident: i,
            identType: t2,
            subscriptLst: s,
        } => {
            let mut p = inString;
            let mut i_1: ArcStr;
            i_1 = stringAppend(p, i.clone());
            ComponentReferenceBasics::makeCrefIdent(i_1, t2.clone(), s.clone())
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outComponentRef)
}

pub fn appendStringCref(
    mut r#str: ArcStr,
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut ocr: metamodelica::Ref<DAE::ComponentRef>;
    ocr = joinCrefs(
        cr,
        metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
            ident: r#str,
            identType: DAE::T_UNKNOWN_DEFAULT().clone(),
            subscriptLst: metamodelica::nil(),
        }),
    )?;
    Ok(ocr)
}

pub fn appendStringFirstIdent(
    mut inString: ArcStr,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (match &**inCref {
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            identType: ty,
            subscriptLst: subs,
            componentRef: cr,
        } => {
            let mut id = (*id).clone();
            id = stringAppend(id.clone(), inString);
            metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                ident: id.clone(),
                identType: ty.clone(),
                subscriptLst: subs.clone(),
                componentRef: cr.clone(),
            })
        }
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            identType: ty,
            subscriptLst: subs,
        } => {
            let mut id = (*id).clone();
            id = stringAppend(id.clone(), inString);
            metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: id.clone(),
                identType: ty.clone(),
                subscriptLst: subs.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outCref)
}

pub fn appendStringLastIdent(
    mut inString: &ArcStr,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (match &**inCref {
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            identType: ty,
            subscriptLst: subs,
            componentRef: cr,
        } => {
            let mut cr = (*cr).clone();
            cr = appendStringLastIdent(inString, metamodelica::AsArg::as_arg(&cr))?;
            metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                ident: id.clone(),
                identType: ty.clone(),
                subscriptLst: subs.clone(),
                componentRef: cr.clone(),
            })
        }
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            identType: ty,
            subscriptLst: subs,
        } => {
            let mut id = (*id).clone();
            id = stringAppend(id.clone(), inString.clone());
            metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: id.clone(),
                identType: ty.clone(),
                subscriptLst: subs.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outCref)
}

pub fn joinCrefs(
    mut inComponentRef1: &metamodelica::Ref<DAE::ComponentRef>,
    mut inComponentRef2: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (match &**inComponentRef1 {
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            identType: t2,
            subscriptLst: sub,
        } => {
            let mut cr2 = inComponentRef2;
            ComponentReferenceBasics::makeCrefQual(id.clone(), t2.clone(), sub.clone(), cr2)
        }
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            identType: t2,
            subscriptLst: sub,
            componentRef: cr,
        } => {
            let mut cr2 = inComponentRef2;
            let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
            cr_1 = joinCrefs(cr, cr2)?;
            ComponentReferenceBasics::makeCrefQual(id.clone(), t2.clone(), sub.clone(), cr_1)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outComponentRef)
}

pub fn joinCrefsR(
    mut inComponentRef2: metamodelica::Ref<DAE::ComponentRef>,
    mut inComponentRef1: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (match &**inComponentRef1 {
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            identType: t2,
            subscriptLst: sub,
        } => {
            let mut cr2 = inComponentRef2;
            ComponentReferenceBasics::makeCrefQual(id.clone(), t2.clone(), sub.clone(), cr2)
        }
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            identType: t2,
            subscriptLst: sub,
            componentRef: cr,
        } => {
            let mut cr2 = inComponentRef2;
            let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
            cr_1 = joinCrefs(cr, cr2)?;
            ComponentReferenceBasics::makeCrefQual(id.clone(), t2.clone(), sub.clone(), cr_1)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outComponentRef)
}

pub fn joinCrefsExp(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>)> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut cref: metamodelica::Ref<DAE::ComponentRef> = cref;
    exp = (match &*exp {
        DAE::Exp::CREF {
            componentRef: cr,
            ty: tp,
        } => {
            let mut cr = (*cr).clone();
            cr = joinCrefs(&cref, cr.clone())?;
            metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: cr.clone(),
                ty: tp.clone(),
            })
        }
        _ => exp,
    });
    Ok((exp, cref))
}

pub fn subscriptCref(
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut inSubscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (match &**inComponentRef {
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            subscriptLst: sub,
            identType: t2,
        } => {
            let mut newsub = inSubscriptLst;
            let mut newsub_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            newsub_1 = listAppend(sub.clone(), newsub);
            ComponentReferenceBasics::makeCrefIdent(id.clone(), t2.clone(), newsub_1)
        }
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            subscriptLst: sub,
            componentRef: cref,
            identType: t2,
        } => {
            let mut newsub = inSubscriptLst;
            let mut cref_1: metamodelica::Ref<DAE::ComponentRef>;
            cref_1 = subscriptCref(cref, newsub)?;
            ComponentReferenceBasics::makeCrefQual(id.clone(), t2.clone(), sub.clone(), cref_1)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outComponentRef)
}

pub fn subscriptCrefWithInt(
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut inSubscript: i32,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (match &**inComponentRef {
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            subscriptLst: subs,
            identType: ty,
        } => {
            let mut new_sub: metamodelica::Ref<DAE::Subscript>;
            let mut subs = (*subs).clone();
            let mut ty = (*ty).clone();
            new_sub = metamodelica::Ref::new(DAE::Subscript::INDEX {
                exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: inSubscript }),
            });
            subs = List::appendElt(new_sub, subs.clone());
            ty = Expression::unliftArray(metamodelica::AsArg::as_arg(&ty))?;
            ComponentReferenceBasics::makeCrefIdent(id.clone(), ty.clone(), subs.clone())
        }
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            subscriptLst: subs,
            componentRef: rest_cref,
            identType: ty,
        } => {
            let mut rest_cref = (*rest_cref).clone();
            rest_cref = subscriptCrefWithInt(metamodelica::AsArg::as_arg(&rest_cref), inSubscript)?;
            ComponentReferenceBasics::makeCrefQual(id.clone(), ty.clone(), subs.clone(), rest_cref.clone())
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outComponentRef)
}

pub fn crefSetLastSubs(
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut inSubs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (match &**inComponentRef {
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            identType: tp,
            ..
        } => ComponentReferenceBasics::makeCrefIdent(id.clone(), tp.clone(), inSubs.clone()),
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            identType: tp,
            subscriptLst: subs,
            componentRef: cr,
        } => {
            let mut cr = (*cr).clone();
            cr = crefSetLastSubs(metamodelica::AsArg::as_arg(&cr), inSubs)?;
            ComponentReferenceBasics::makeCrefQual(id.clone(), tp.clone(), subs.clone(), cr.clone())
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outComponentRef)
}

pub fn crefApplySubs(
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut inSubs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match inComponentRef {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: tp @ Deref @ DAE::Type::T_ARRAY { dims, .. }, subscriptLst: subs } => {
            if ((subs).len() as i32) + ((inSubs).len() as i32) > ((dims).len() as i32) {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ComponentReference.crefApplySubs [")); __mm_s.push_str(&*ExpressionBasics::printListStr(inSubs.clone(), &move |__a0: metamodelica::Ref<DAE::Subscript>| ExpressionBasics::printSubscriptStr(&__a0), literal!(","))?); __mm_s.push_str(&*literal!("] to ident ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inComponentRef)?); __mm_s.push_str(&*literal!(" with ")); __mm_s.push_str(&*intString(((dims).len() as i32))); __mm_s.push_str(&*literal!(" dimensions\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/ComponentReference.mo"))?;
                return Err("fail");
            }
            ComponentReferenceBasics::makeCrefIdent(id.clone(), tp.clone(), listAppend(subs.clone(), inSubs.clone()))
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: tp @ Deref @ DAE::Type::T_ARRAY { dims, .. }, subscriptLst: subs, componentRef: cr } => {
            let mut subs1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut subs2: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut cr = (*cr).clone();
            if ((inSubs).len() as i32) > ((dims).len() as i32) - ((subs).len() as i32) {
                (subs1, subs2) = List::split(inSubs.clone(), ((dims).len() as i32) - ((subs).len() as i32))?;
                cr = crefApplySubs(metamodelica::AsArg::as_arg(&cr), &subs2)?;
            } else {
                subs1 = inSubs.clone();
            }
            if ((subs).len() as i32) + ((subs1).len() as i32) > ((dims).len() as i32) {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ComponentReference.crefApplySubs [")); __mm_s.push_str(&*ExpressionBasics::printListStr(inSubs.clone(), &move |__a0: metamodelica::Ref<DAE::Subscript>| ExpressionBasics::printSubscriptStr(&__a0), literal!(","))?); __mm_s.push_str(&*literal!("] to qual ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inComponentRef)?); __mm_s.push_str(&*literal!(" with ")); __mm_s.push_str(&*intString(((dims).len() as i32))); __mm_s.push_str(&*literal!(" dimensions\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/ComponentReference.mo"))?;
                return Err("fail");
            }
            ComponentReferenceBasics::makeCrefQual(id.clone(), tp.clone(), listAppend(subs.clone(), subs1), cr.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: tp, subscriptLst: subs, componentRef: cr } => {
            let mut cr = (*cr).clone();
            cr = crefApplySubs(metamodelica::AsArg::as_arg(&cr), inSubs)?;
            ComponentReferenceBasics::makeCrefQual(id.clone(), tp.clone(), subs.clone(), cr.clone())
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ComponentReference.crefApplySubs to non array ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inComponentRef)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/ComponentReference.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outComponentRef)
}

pub(crate) fn crefSetType(
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut ty: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut cref: metamodelica::Ref<DAE::ComponentRef> = cref;
    cref = (match &*cref {
        DAE::ComponentRef::CREF_IDENT { .. } => {
            assign_variant_field!(cref => DAE::ComponentRef::CREF_IDENT; identType = ty);
            cref
        }
        DAE::ComponentRef::CREF_QUAL { .. } => {
            assign_variant_field!(cref => DAE::ComponentRef::CREF_QUAL; identType = ty);
            cref
        }
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("ComponentReference.crefSetType"));
                    __mm_s.push_str(&*literal!(" was applied on a cref that has no type: "));
                    __mm_s.push_str(&*crefStr(&cref)?);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("FrontEnd/ComponentReference.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(cref)
}

pub fn crefSetLastType(
    mut inRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut newType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outRef: metamodelica::Ref<DAE::ComponentRef>;
    outRef = (match &**inRef {
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            identType: _,
            subscriptLst: subs,
        } => ComponentReferenceBasics::makeCrefIdent(id.clone(), newType.clone(), subs.clone()),
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            identType: ty,
            subscriptLst: subs,
            componentRef: child,
        } => {
            let mut child = (*child).clone();
            child = crefSetLastType(metamodelica::AsArg::as_arg(&child), newType)?;
            ComponentReferenceBasics::makeCrefQual(id.clone(), ty.clone(), subs.clone(), child.clone())
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outRef)
}

pub(crate) fn replaceCrefSliceSub(
    mut inCr: &metamodelica::Ref<DAE::ComponentRef>,
    mut newSub: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCr: metamodelica::Ref<DAE::ComponentRef>;
    outCr = 'mc: {
        let __mc_input = &**inCr;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, identType, subscriptLst: subs } => {
                    let mut subs = (*subs).clone();
                    subs = replaceSliceSub(metamodelica::AsArg::as_arg(&subs), newSub)?;
                    Ok(ComponentReferenceBasics::makeCrefIdent(name.clone(), identType.clone(), subs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { identType: t2, subscriptLst: subs, .. } => {
                    let mut child: metamodelica::Ref<DAE::ComponentRef>;
                    let true = ((((Expression::arrayTypeDimensions(metamodelica::AsArg::as_arg(&t2))?)).len() as i32) >= ((subs).len() as i32) + 1) else { return Err("pattern mismatch") };
                    child = subscriptCref(inCr, newSub.clone())?;
                    Ok(child.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { identType: t2, subscriptLst: subs, .. } => {
                    let mut child: metamodelica::Ref<DAE::ComponentRef>;
                    let false = ((((Expression::arrayTypeDimensions(metamodelica::AsArg::as_arg(&t2))?)).len() as i32) >= ((subs).len() as i32) + ((newSub).len() as i32)) else { return Err("pattern mismatch") };
                    child = subscriptCref(inCr, newSub.clone())?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("WARNING - Expression.replaceCref_SliceSub setting subscript last, not containing dimension\n"))?;
                    }
                    Ok(child.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType, subscriptLst: subs, componentRef: child } => {
                    let mut subs = (*subs).clone();
                    subs = replaceSliceSub(metamodelica::AsArg::as_arg(&subs), newSub)?;
                    Ok(ComponentReferenceBasics::makeCrefQual(name.clone(), identType.clone(), subs.clone(), child.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType, subscriptLst: subs, componentRef: child } => {
                    let true = ((((Expression::arrayTypeDimensions(metamodelica::AsArg::as_arg(&identType))?)).len() as i32) >= ((subs).len() as i32) + 1) else { return Err("pattern mismatch") };
                    Ok(ComponentReferenceBasics::makeCrefQual(name.clone(), identType.clone(), listAppend(subs.clone(), newSub.clone()), child.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType, subscriptLst: subs, componentRef: child } => {
                    let mut child = (*child).clone();
                    child = replaceCrefSliceSub(metamodelica::AsArg::as_arg(&child), newSub)?;
                    Ok(ComponentReferenceBasics::makeCrefQual(name.clone(), identType.clone(), subs.clone(), child.clone()))
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
                    Debug::trace(literal!("- Expression.replaceCref_SliceSub failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCr)
}

fn replaceSliceSub(
    mut inSubs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inSub: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    let mut osubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    osubs = (::match_deref::match_deref! { match inSubs {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: _ }, tail: subs } => {
            let mut subs = (*subs).clone();
            subs = listAppend(inSub.clone(), subs.clone());
            subs.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: subs } => {
            let mut subs = (*subs).clone();
            subs = listAppend(inSub.clone(), subs.clone());
            subs.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: sub, tail: subs } => {
            let mut subs = (*subs).clone();
            subs = replaceSliceSub(metamodelica::AsArg::as_arg(&subs), inSub)?;
            metamodelica::cons(sub.clone(), subs.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(osubs)
}

pub(crate) fn stripCrefIdentSliceSubs(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (match &**inCref {
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            subscriptLst: subs,
            identType: ty,
        } => {
            let mut subs = (*subs).clone();
            subs = removeSliceSubs(metamodelica::AsArg::as_arg(&subs));
            ComponentReferenceBasics::makeCrefIdent(id.clone(), ty.clone(), subs.clone())
        }
        DAE::ComponentRef::CREF_QUAL {
            componentRef: cr,
            identType: ty,
            subscriptLst: subs,
            ident: id,
        } => {
            outCref = stripCrefIdentSliceSubs(cr)?;
            ComponentReferenceBasics::makeCrefQual(id.clone(), ty.clone(), subs.clone(), outCref)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outCref)
}

pub fn stripArrayCref(
    mut crefIn: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(
    metamodelica::Ref<DAE::ComponentRef>,
    i32,
    Option<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut crefHead: metamodelica::Ref<DAE::ComponentRef>;
    let mut idxOut: i32;
    let mut crefTail: Option<metamodelica::Ref<DAE::ComponentRef>>;
    (crefHead, idxOut, crefTail) = (::match_deref::match_deref! { match crefIn {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: idx } }, tail: Deref @ metamodelica::ListNode::Nil }, identType: ty } => {
            (ComponentReferenceBasics::makeCrefIdent(id.clone(), ty.clone(), metamodelica::nil()), idx.clone(), None)
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, identType: ty, subscriptLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: idx } }, tail: Deref @ metamodelica::ListNode::Nil }, ident: id } => {
            (ComponentReferenceBasics::makeCrefIdent(id.clone(), ty.clone(), metamodelica::nil()), idx.clone(), Some(cr.clone()))
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, identType: ty, ident: id, .. } => {
            let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
            outCref = stripCrefIdentSliceSubs(cr)?;
            (ComponentReferenceBasics::makeCrefQual(id.clone(), ty.clone(), metamodelica::nil(), outCref), -1, None)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((crefHead, idxOut, crefTail))
}

fn removeSliceSubs(
    mut subs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Subscript>> {
    let mut osubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
    for mut s in &**subs {
        osubs = (match &*s.clone() {
            DAE::Subscript::SLICE { .. } => osubs,
            _ => metamodelica::cons(s.clone(), osubs),
        });
    }
    osubs = Dangerous::listReverseInPlace(osubs);
    osubs
}

pub fn crefStripSubs(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (match &**inCref {
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            identType: ty,
            ..
        } => ComponentReferenceBasics::makeCrefIdent(id.clone(), ty.clone(), metamodelica::nil()),
        DAE::ComponentRef::CREF_QUAL {
            componentRef: cr,
            identType: ty,
            ident: id,
            ..
        } => {
            outCref = crefStripSubs(cr)?;
            ComponentReferenceBasics::makeCrefQual(id.clone(), ty.clone(), metamodelica::nil(), outCref)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outCref)
}

pub fn crefRenameSeedRoot(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut newJacName: &ArcStr,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (match &*inCref {
        DAE::ComponentRef::CREF_IDENT {
            ident: __inCref_ident,
            identType: __inCref_identType,
            subscriptLst: __inCref_subscriptLst,
        } if (StringUtil::startsWith(__inCref_ident.clone(), literal!("$SEED_"))) => {
            ComponentReferenceBasics::makeCrefIdent(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("$SEED_"));
                    __mm_s.push_str(&*newJacName);
                    ArcStr::from(__mm_s)
                },
                __inCref_identType.clone(),
                __inCref_subscriptLst.clone(),
            )
        }
        DAE::ComponentRef::CREF_QUAL {
            componentRef: __inCref_componentRef,
            ident: __inCref_ident,
            identType: __inCref_identType,
            subscriptLst: __inCref_subscriptLst,
        } if (StringUtil::startsWith(__inCref_ident.clone(), literal!("$SEED_"))) => {
            ComponentReferenceBasics::makeCrefQual(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("$SEED_"));
                    __mm_s.push_str(&*newJacName);
                    ArcStr::from(__mm_s)
                },
                __inCref_identType.clone(),
                __inCref_subscriptLst.clone(),
                __inCref_componentRef.clone(),
            )
        }
        _ => inCref,
    });
    outCref
}

pub fn crefStripSubsExceptModelSubs(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    fn is_model_array(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
        let mut res: bool;
        let mut state: ClassInf::State;
        res = (::match_deref::match_deref! { match ty {
            Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: __esc_state, .. }, .. } => {
                state = (*__esc_state).clone();
                (match state.clone() {
            ClassInf::State::MODEL { .. } => true,
            ClassInf::State::BLOCK { .. } => true,
            _ => false,
        })
            },
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        res
    }

    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { identType: __inCref_identType, .. } if (is_model_array(metamodelica::AsArg::as_arg(&__inCref_identType))) => {
            inCref.clone()
        },
        cref @ Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, .. } if (is_model_array(var_field!((*inCref).identType, DAE::ComponentRef::CREF_QUAL))) => {
            let mut cref = (*cref).clone();
            outCref = crefStripSubsExceptModelSubs(cr.clone());
            assign_variant_field!(cref => DAE::ComponentRef::CREF_QUAL; componentRef = outCref);
            cref.clone()
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty, .. } => {
            ComponentReferenceBasics::makeCrefIdent(id.clone(), ty.clone(), metamodelica::nil())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr, identType: ty, ident: id, .. } => {
            outCref = crefStripSubsExceptModelSubs(cr.clone());
            ComponentReferenceBasics::makeCrefQual(id.clone(), ty.clone(), metamodelica::nil(), outCref)
        },
        _ => {
            inCref.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCref
}

pub fn crefStripPrefix<'__b>(
    mut cref: &'__b metamodelica::Ref<DAE::ComponentRef>,
    mut prefix: &'__b metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    '__tco: loop {
        ::match_deref::match_deref! { match (cref, prefix) {
            (Deref @ DAE::ComponentRef::CREF_QUAL { ident: id1, identType: _, subscriptLst: subs1, componentRef: cr1 }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: id2, identType: _, subscriptLst: subs2 }) => {
                let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                let true = (ExpressionBasics::subscriptEqual(subs1, subs2)?) else { return Err("pattern mismatch") };
                return Ok(cr1.clone())
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { ident: id1, identType: _, subscriptLst: subs1, componentRef: cr1 }, Deref @ DAE::ComponentRef::CREF_QUAL { ident: id2, identType: _, subscriptLst: subs2, componentRef: cr2 }) => {
                let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                let true = (ExpressionBasics::subscriptEqual(subs1, subs2)?) else { return Err("pattern mismatch") };
                { (cref, prefix) = (cr1, cr2); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn crefStripLastIdent(
    mut inCr: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCr: metamodelica::Ref<DAE::ComponentRef>;
    outCr = (::match_deref::match_deref! { match inCr {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: t2, subscriptLst: subs, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ } } => {
            ComponentReferenceBasics::makeCrefIdent(id.clone(), t2.clone(), subs.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: t2, subscriptLst: subs, componentRef: cr } => {
            let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
            cr1 = crefStripLastIdent(cr)?;
            ComponentReferenceBasics::makeCrefQual(id.clone(), t2.clone(), subs.clone(), cr1)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outCr)
}

pub fn crefStripIterSub(
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut iter: &ArcStr,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut ident: ArcStr;
    let mut index: ArcStr;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match inComponentRef {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_ident, identType: __esc_ty, subscriptLst: __esc_subs @ Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_index, .. }, .. } }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            ident = (*__esc_ident).clone();
            ty = (*__esc_ty).clone();
            subs = (*__esc_subs).clone();
            index = (*__esc_index).clone();
            ComponentReferenceBasics::makeCrefIdent(ident.clone(), ty.clone(), if (metamodelica::stringEq(&(literal!("")), &iter) || metamodelica::stringEq(&index, &iter)) {metamodelica::nil()} else {subs.clone()})
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: __esc_ident, identType: __esc_ty, componentRef: __esc_cref, subscriptLst: __esc_subs @ Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_index, .. }, .. } }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            ident = (*__esc_ident).clone();
            ty = (*__esc_ty).clone();
            cref = (*__esc_cref).clone();
            subs = (*__esc_subs).clone();
            index = (*__esc_index).clone();
            if metamodelica::stringEq(&(literal!("")), &iter) || metamodelica::stringEq(&index, &iter) {
                subs = metamodelica::nil();
            } else {
                cref = crefStripIterSub(metamodelica::AsArg::as_arg(&cref), iter);
            }
            ComponentReferenceBasics::makeCrefQual(ident.clone(), ty.clone(), subs.clone(), cref.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: __esc_ident, identType: __esc_ty, componentRef: __esc_cref, subscriptLst: __esc_subs } => {
            ident = (*__esc_ident).clone();
            ty = (*__esc_ty).clone();
            cref = (*__esc_cref).clone();
            subs = (*__esc_subs).clone();
            ComponentReferenceBasics::makeCrefQual(ident.clone(), ty.clone(), subs.clone(), crefStripIterSub(metamodelica::AsArg::as_arg(&cref), iter))
        },
        _ => inComponentRef.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outComponentRef
}

pub fn crefStripFirstIdent(
    mut inCr: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCr: metamodelica::Ref<DAE::ComponentRef>;
    outCr = (match &**inCr {
        DAE::ComponentRef::CREF_QUAL { componentRef: cr, .. } => cr.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outCr)
}

pub(crate) fn crefStripLastSubsStringified(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = 'mc: {
        let __mc_input = inComponentRef;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: t2, subscriptLst: Deref @ metamodelica::ListNode::Nil } => {
                    let mut lst: metamodelica::List<ArcStr>;
                    let mut lst_1: metamodelica::List<ArcStr>;
                    let mut id_1: ArcStr;
                    lst = Util::stringSplitAtChar(id.clone(), literal!("["))?;
                    lst_1 = List::stripLast(lst.clone())?;
                    id_1 = stringDelimitList(lst_1.clone(), literal!("["));
                    Ok(ComponentReferenceBasics::makeCrefIdent(id_1.clone(), t2.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cr => {
                    Ok(cr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outComponentRef
}

pub(crate) fn stringifyComponentRef(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
    let mut crs: ArcStr;
    let mut ty: metamodelica::Ref<DAE::Type>;
    subs = crefLastSubs(cr)?;
    cr_1 = ComponentReferenceBasics::crefStripLastSubs(cr)?;
    crs = ComponentReferenceBasics::printComponentRefStr(&cr_1)?;
    ty = crefLastType(cr)?;
    outComponentRef = ComponentReferenceBasics::makeCrefIdent(crs, ty, subs);
    Ok(outComponentRef)
}

/* **************************************************/
/* Print and Dump */
/* **************************************************/
pub(crate) fn printComponentRef(mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>) -> Result<()> {
    let () = (match &**inComponentRef {
        DAE::ComponentRef::WILD { .. } => {
            Print::printBuf(literal!("_"))?;
            ()
        }
        DAE::ComponentRef::CREF_IDENT {
            ident: s,
            subscriptLst: subs,
            ..
        } => {
            printComponentRef2(s.clone(), subs.clone())?;
            ()
        }
        DAE::ComponentRef::CREF_QUAL {
            ident: s,
            subscriptLst: subs,
            componentRef: cr,
            ..
        } => {
            if Config::modelicaOutput()? {
                printComponentRef2(s.clone(), subs.clone())?;
                Print::printBuf(literal!("__"))?;
                printComponentRef(cr)?;
            } else {
                printComponentRef2(s.clone(), subs.clone())?;
                Print::printBuf(literal!("."))?;
                printComponentRef(cr)?;
            }
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn printComponentRef2(
    mut inString: ArcStr,
    mut inSubscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inString, inSubscriptLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (s, Deref @ metamodelica::ListNode::Nil) => {
                    Print::printBuf(s.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (s, l) => {
                    if Config::modelicaOutput()? {
                        Print::printBuf(s.clone())?;
                        Print::printBuf(literal!("_L"))?;
                        ExpressionDump::printList(metamodelica::AsArg::as_arg(&l), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Subscript>| ExpressionDump::printSubscript(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Subscript>) -> Result<()> + 'static>), literal!(","))?;
                        Print::printBuf(literal!("_R"))?;
                    } else {
                        Print::printBuf(s.clone())?;
                        Print::printBuf(literal!("["))?;
                        ExpressionDump::printList(metamodelica::AsArg::as_arg(&l), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Subscript>| ExpressionDump::printSubscript(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Subscript>) -> Result<()> + 'static>), literal!(","))?;
                        Print::printBuf(literal!("]"))?;
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

pub fn printComponentRefList(mut crs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<()> {
    let mut buffer: ArcStr;
    buffer = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*stringDelimitList(
            List::map(crs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                ComponentReferenceBasics::printComponentRefStr(&__a0)
            })?,
            literal!(", "),
        ));
        __mm_s.push_str(&*literal!("}\n"));
        ArcStr::from(__mm_s)
    };
    metamodelica::print(buffer);
    Ok(())
}

pub fn replaceWholeDimSubscript(
    mut icr: &metamodelica::Ref<DAE::ComponentRef>,
    mut index: i32,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut ocr: metamodelica::Ref<DAE::ComponentRef>;
    ocr = 'mc: {
        let __mc_input = &**icr;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: et, subscriptLst: ss, componentRef: cr } => {
                    let mut ss = (*ss).clone();
                    ss = replaceWholeDimSubscript2(metamodelica::AsArg::as_arg(&ss), index)?;
                    Ok(metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL { ident: id.clone(), identType: et.clone(), subscriptLst: ss.clone(), componentRef: cr.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: et, subscriptLst: ss, componentRef: cr } => {
                    let mut cr = (*cr).clone();
                    cr = replaceWholeDimSubscript(metamodelica::AsArg::as_arg(&cr), index)?;
                    Ok(metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL { ident: id.clone(), identType: et.clone(), subscriptLst: ss.clone(), componentRef: cr.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: et, subscriptLst: ss } => {
                    let mut ss = (*ss).clone();
                    ss = replaceWholeDimSubscript2(metamodelica::AsArg::as_arg(&ss), index)?;
                    Ok(metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: id.clone(), identType: et.clone(), subscriptLst: ss.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(ocr)
}

pub(crate) fn replaceWholeDimSubscript2(
    mut isubs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut index: i32,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    let mut osubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    osubs = (::match_deref::match_deref! { match isubs {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: subs } => {
            let mut sub: metamodelica::Ref<DAE::Subscript>;
            sub = metamodelica::Ref::new(DAE::Subscript::INDEX { exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: index }) });
            metamodelica::cons(sub, subs.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: sub, tail: subs } => {
            let mut subs = (*subs).clone();
            subs = replaceWholeDimSubscript2(metamodelica::AsArg::as_arg(&subs), index)?;
            metamodelica::cons(sub.clone(), subs.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(osubs)
}

pub fn splitCrefLast(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::ComponentRef>,
)> {
    let mut outPrefixCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut outLastCref: metamodelica::Ref<DAE::ComponentRef>;
    (outPrefixCref, outLastCref) = (::match_deref::match_deref! { match inCref {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty, subscriptLst: subs, componentRef: last @ Deref @ DAE::ComponentRef::CREF_IDENT { .. } } => {
            (metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: id.clone(), identType: ty.clone(), subscriptLst: subs.clone() }), last.clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty, subscriptLst: subs, componentRef: last } => {
            let mut prefix: metamodelica::Ref<DAE::ComponentRef>;
            let mut last = (*last).clone();
            (prefix, last) = splitCrefLast(metamodelica::AsArg::as_arg(&last))?;
            (metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL { ident: id.clone(), identType: ty.clone(), subscriptLst: subs.clone(), componentRef: prefix }), last.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outPrefixCref, outLastCref))
}

pub fn firstNCrefs(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut nIn: i32,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outFirstCrefs: metamodelica::Ref<DAE::ComponentRef>;
    outFirstCrefs = (::match_deref::match_deref! { match &((inCref.clone(), nIn)) {
        (_, 0) => {
            inCref.clone()
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty, subscriptLst: subs, componentRef: _ }, 1) => {
            metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: id.clone(), identType: ty.clone(), subscriptLst: subs.clone() })
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ }, _) => {
            inCref.clone()
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty, subscriptLst: subs, componentRef: last }, _) => {
            let mut prefix: metamodelica::Ref<DAE::ComponentRef>;
            prefix = firstNCrefs(metamodelica::AsArg::as_arg(&last), nIn - 1);
            metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL { ident: id.clone(), identType: ty.clone(), subscriptLst: subs.clone(), componentRef: prefix })
        },
        _ => {
            inCref.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outFirstCrefs
}

pub(crate) fn splitCrefFirst(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::ComponentRef>,
)> {
    let mut outCrefFirst: metamodelica::Ref<DAE::ComponentRef>;
    let mut outCrefRest: metamodelica::Ref<DAE::ComponentRef>;
    let mut id: ArcStr;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*inCref)) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: __pa0, identType: __pa1, subscriptLst: __pa2, componentRef: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    id = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    subs = metamodelica::Own::own(__pa2);
    outCrefRest = metamodelica::Own::own(__pa3);
    outCrefFirst = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
        ident: id,
        identType: ty,
        subscriptLst: subs,
    });
    Ok((outCrefFirst, outCrefRest))
}

pub(crate) fn toStringList(mut inCref: &metamodelica::Ref<DAE::ComponentRef>) -> metamodelica::List<ArcStr> {
    let mut outStringList: metamodelica::List<ArcStr>;
    outStringList = Dangerous::listReverseInPlace(toStringList_tail(inCref, metamodelica::nil()));
    outStringList
}

fn toStringList_tail<'__b>(
    mut inCref: &'__b metamodelica::Ref<DAE::ComponentRef>,
    mut inAccumStrings: metamodelica::List<ArcStr>,
) -> metamodelica::List<ArcStr> {
    '__tco: loop {
        match &**inCref {
            DAE::ComponentRef::CREF_QUAL {
                ident: id,
                componentRef: cref,
                ..
            } => {
                (inCref, inAccumStrings) = (cref, metamodelica::cons(id.clone(), inAccumStrings));
                continue '__tco;
            }
            DAE::ComponentRef::CREF_IDENT { ident: id, .. } => return metamodelica::cons(id.clone(), inAccumStrings),
            _ => return metamodelica::nil(),
        }
    }
}

pub fn crefDepth(mut inCref: &metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> {
    let mut depth: i32;
    depth = (match &**inCref {
        DAE::ComponentRef::WILD { .. } => 0,
        DAE::ComponentRef::CREF_IDENT { .. } => 1,
        DAE::ComponentRef::CREF_QUAL { componentRef: n, .. } => crefDepth1(n, 1)?,
        _ => return Err("match: no arm matched"),
    });
    Ok(depth)
}

fn crefDepth1<'__b>(mut inCref: &'__b metamodelica::Ref<DAE::ComponentRef>, mut iDepth: i32) -> Result<i32> {
    '__tco: loop {
        match &**inCref {
            DAE::ComponentRef::WILD { .. } => return Ok(iDepth),
            DAE::ComponentRef::CREF_IDENT { .. } => return Ok(1 + iDepth),
            DAE::ComponentRef::CREF_QUAL { componentRef: n, .. } => {
                (inCref, iDepth) = (n, 1 + iDepth);
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn expandCref(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut expandRecord: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outCref: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outCref = (match expandRecord {
        _ => expandCref_impl(inCref, expandRecord),
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("- ComponentReference.expandCref failed on "));
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inCref)?);
                ArcStr::from(__mm_s)
            })?;
            return Err("fail");
        }
    });
    Ok(outCref)
}

pub(crate) fn expandCref_impl(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut expandRecord: bool,
) -> metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outCref = 'mc: {
        let __mc_input = (&**inCref, expandRecord);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: _ }, .. }, subscriptLst: Deref @ metamodelica::ListNode::Nil }, true) => {
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    crefs = List::map(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(creffromVar(&__a0)) })?;
                    crefs = List::map1r(crefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| joinCrefs(&__a0, __a1), inCref.clone())?;
                    Ok(List::mapFlat(&crefs, &({ let __pe_b1 = true; move |__pe_a0| Ok(expandCref_impl(&__pe_a0, __pe_b1.clone())) }))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty @ Deref @ DAE::Type::T_ARRAY { .. }, subscriptLst: Deref @ metamodelica::ListNode::Nil }, true) => {
                    let mut basety: metamodelica::Ref<DAE::Type>;
                    let mut correctTy: metamodelica::Ref<DAE::Type>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut varLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let (__pa1, __pa0, __pa2) = ::match_deref::match_deref! { match &(TypesDump::flattenArrayType(metamodelica::AsArg::as_arg(&ty))) {
                        (__pa1 @ Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, complexClassType: ClassInf::State::RECORD { .. }, .. }, __pa2) => (__pa1.clone(), __pa0.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    varLst = metamodelica::Own::own(__pa0);
                    basety = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    correctTy = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: basety.clone(), dims: dims.clone() });
                    subs = List::fill(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), ((dims).len() as i32));
                    crefs = expandCref2(id.clone(), correctTy.clone(), subs.clone(), dims.clone())?;
                    Ok(expandCrefLst(&crefs, &varLst, metamodelica::nil())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty @ Deref @ DAE::Type::T_ARRAY { .. }, subscriptLst: Deref @ metamodelica::ListNode::Nil }, _) => {
                    let mut basety: metamodelica::Ref<DAE::Type>;
                    let mut correctTy: metamodelica::Ref<DAE::Type>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    (basety, dims) = TypesDump::flattenArrayType(metamodelica::AsArg::as_arg(&ty));
                    correctTy = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: basety.clone(), dims: dims.clone() });
                    subs = List::fill(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), ((dims).len() as i32));
                    Ok(expandCref2(id.clone(), correctTy.clone(), subs.clone(), dims.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty @ Deref @ DAE::Type::T_ARRAY { .. }, subscriptLst: subs }, true) => {
                    let mut basety: metamodelica::Ref<DAE::Type>;
                    let mut correctTy: metamodelica::Ref<DAE::Type>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut varLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut missing_subs: i32;
                    let mut subs = (*subs).clone();
                    let (__pa1, __pa0, __pa2) = ::match_deref::match_deref! { match &(TypesDump::flattenArrayType(metamodelica::AsArg::as_arg(&ty))) {
                        (__pa1 @ Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, complexClassType: ClassInf::State::RECORD { .. }, .. }, __pa2) => (__pa1.clone(), __pa0.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    varLst = metamodelica::Own::own(__pa0);
                    basety = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    correctTy = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: basety.clone(), dims: dims.clone() });
                    missing_subs = ((dims).len() as i32) - ((subs).len() as i32);
                    if missing_subs > 0 {
                        subs = listAppend(subs.clone(), List::fill(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), missing_subs));
                    }
                    crefs = expandCref2(id.clone(), correctTy.clone(), subs.clone(), dims.clone())?;
                    Ok(expandCrefLst(&crefs, &varLst, metamodelica::nil())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty @ Deref @ DAE::Type::T_ARRAY { .. }, subscriptLst: subs }, _) => {
                    let mut basety: metamodelica::Ref<DAE::Type>;
                    let mut correctTy: metamodelica::Ref<DAE::Type>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut missing_subs: i32;
                    let mut subs = (*subs).clone();
                    (basety, dims) = TypesDump::flattenArrayType(metamodelica::AsArg::as_arg(&ty));
                    correctTy = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: basety.clone(), dims: dims.clone() });
                    missing_subs = ((dims).len() as i32) - ((subs).len() as i32);
                    if missing_subs > 0 {
                        subs = listAppend(subs.clone(), List::fill(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), missing_subs));
                    }
                    Ok(expandCref2(id.clone(), correctTy.clone(), subs.clone(), dims.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty @ Deref @ DAE::Type::T_ARRAY { .. }, subscriptLst: subs, componentRef: cref }, _) => {
                    let mut basety: metamodelica::Ref<DAE::Type>;
                    let mut correctTy: metamodelica::Ref<DAE::Type>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut crefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut cref = (*cref).clone();
                    crefs = expandCref_impl(metamodelica::AsArg::as_arg(&cref), expandRecord);
                    (basety, dims) = TypesDump::flattenArrayType(metamodelica::AsArg::as_arg(&ty));
                    correctTy = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: basety.clone(), dims: dims.clone() });
                    cref = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: id.clone(), identType: correctTy.clone(), subscriptLst: subs.clone() });
                    crefs2 = expandCref_impl(metamodelica::AsArg::as_arg(&cref), false);
                    crefs2 = crefs2.clone().reverse();
                    crefs = expandCrefQual(&crefs2, crefs.clone())?;
                    Ok(crefs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty, subscriptLst: subs, componentRef: cref }, _) => {
                            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            crefs = expandCref_impl(metamodelica::AsArg::as_arg(&cref), expandRecord);
                            crefs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
                for mut c in (crefs.clone()).into_iter().cloned() {
                            let __x = ComponentReferenceBasics::makeCrefQual(id.clone(), ty.clone(), subs.clone(), c.clone());
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            Ok(crefs.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(list![inCref.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outCref
}

fn expandCrefLst<'__b>(
    mut inCrefs: &'__b metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut varLst: &'__b metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inCrefsAcc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inCrefs {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(List::flatten(inCrefsAcc)?)
            },
            Deref @ metamodelica::ListNode::Cons { head: cr, tail: rest } => {
                let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                crefs = List::map(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(creffromVar(&__a0)) })?;
                crefs = List::map1r(crefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| joinCrefs(&__a0, __a1), cr.clone())?;
                { (inCrefs, varLst, inCrefsAcc) = (rest, varLst, metamodelica::cons(crefs, inCrefsAcc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn expandCrefQual(
    mut inHeadCrefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inRestCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    for mut cref in &**inHeadCrefs {
        crefs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
            for mut rest_cref in (inRestCrefs.clone()).into_iter().cloned() {
                let __x = joinCrefs(metamodelica::AsArg::as_arg(&cref), rest_cref.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        outCrefs = listAppend(crefs, outCrefs);
    }
    Ok(outCrefs)
}

fn expandCref2(
    mut inId: ArcStr,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inSubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut subslst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
    subslst = List::threadMap(inSubscripts, inDimensions, &move |__a0: metamodelica::Ref<
        DAE::Subscript,
    >,
                                                                 __a1: metamodelica::Ref<
        DAE::Dimension,
    >| {
        Expression::expandSubscript(__a0, &__a1)
    })?;
    subslst = List::combination(&subslst);
    for mut subs in &*subslst {
        outCrefs = metamodelica::cons(
            ComponentReferenceBasics::makeCrefIdent(inId.clone(), inType.clone(), subs.clone()),
            outCrefs,
        );
    }
    outCrefs = outCrefs.reverse();
    Ok(outCrefs)
}

pub fn replaceSubsWithString(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (::match_deref::match_deref! { match inCref {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType, subscriptLst: Deref @ metamodelica::ListNode::Nil, componentRef: cr } => {
            let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
            cr1 = replaceSubsWithString(cr)?;
            metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL { ident: ident.clone(), identType: identType.clone(), subscriptLst: metamodelica::nil(), componentRef: cr1 })
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType, subscriptLst, componentRef: cr } => {
            let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
            let mut identType = (*identType).clone();
            let mut cr = (*cr).clone();
            identType = Expression::unliftArrayTypeWithSubs(subscriptLst.clone(), identType.clone())?;
            cr1 = replaceSubsWithString(metamodelica::AsArg::as_arg(&cr))?;
            cr = makeCrefsFromSubScriptLst(subscriptLst, metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: ident.clone(), identType: identType.clone(), subscriptLst: metamodelica::nil() }))?;
            joinCrefs(metamodelica::AsArg::as_arg(&cr), cr1)?
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::ListNode::Nil, .. } => {
            inCref.clone()
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType, subscriptLst } => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut identType = (*identType).clone();
            identType = Expression::unliftArrayTypeWithSubs(subscriptLst.clone(), identType.clone())?;
            cr = makeCrefsFromSubScriptLst(subscriptLst, metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: ident.clone(), identType: identType.clone(), subscriptLst: metamodelica::nil() }))?;
            cr
        },
        Deref @ DAE::ComponentRef::WILD { .. } => {
            inCref.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outCref)
}

pub(crate) fn makeCrefsFromSubScriptLst(
    mut inSubscriptLst: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inPreCref: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef> = inPreCref;
    for mut subScript in &**inSubscriptLst {
        outCref = (match &*subScript.clone() {
            DAE::Subscript::INDEX { exp: e } => {
                let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                cr = makeCrefsFromSubScriptExp(metamodelica::AsArg::as_arg(&e))?;
                joinCrefs(&outCref, cr)?
            }
            _ => {
                let mut r#str: ArcStr;
                r#str = ExpressionBasics::printSubscriptStr(metamodelica::AsArg::as_arg(&subScript))?;
                Error::addInternalError(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("function ComponentReference.makeCrefsFromSubScriptLst for:"));
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    },
                    metamodelica::sourceInfo!("FrontEnd/ComponentReference.mo"),
                )?;
                return Err("fail");
            }
        });
    }
    Ok(outCref)
}

pub(crate) fn makeCrefsFromSubScriptExp(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (match &**inExp {
        DAE::Exp::ICONST { .. } => {
            let mut r#str: ArcStr;
            r#str = ExpressionBasics::printExpStr(inExp.clone())?;
            metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: r#str,
                identType: DAE::T_UNKNOWN_DEFAULT().clone(),
                subscriptLst: metamodelica::nil(),
            })
        }
        DAE::Exp::CREF { .. } => Expression::expCref(inExp)?,
        DAE::Exp::BINARY {
            operator: op,
            exp1: e1,
            exp2: e2,
        } => {
            let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
            let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
            let mut r#str: ArcStr;
            r#str = ExpressionDump::binopSymbol(op)?;
            cr1 = makeCrefsFromSubScriptExp(e1)?;
            cr2 = makeCrefsFromSubScriptExp(e2)?;
            outCref = prependStringCref(r#str, &cr1)?;
            outCref = joinCrefs(&outCref, cr2)?;
            outCref
        }
        DAE::Exp::ENUM_LITERAL { name: enum_lit, .. } => {
            let mut r#str: ArcStr;
            r#str = System::stringReplace(
                AbsynUtil::pathString(enum_lit.clone(), literal!("."), true, false)?,
                literal!("."),
                literal!("$P"),
            )?;
            metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: r#str,
                identType: DAE::T_UNKNOWN_DEFAULT().clone(),
                subscriptLst: metamodelica::nil(),
            })
        }
        _ => {
            let mut r#str: ArcStr;
            r#str = ExpressionDump::dumpExpStr(inExp.clone(), 0)?;
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("function ComponentReference.makeCrefsFromSubScriptExp for:"));
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("FrontEnd/ComponentReference.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(outCref)
}

pub(crate) fn replaceLast(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inNewLast: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (match &**inCref {
        DAE::ComponentRef::CREF_QUAL {
            ident,
            identType: ty,
            subscriptLst: subs,
            componentRef: cref,
        } => {
            let mut cref = (*cref).clone();
            cref = replaceLast(metamodelica::AsArg::as_arg(&cref), inNewLast)?;
            metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                ident: ident.clone(),
                identType: ty.clone(),
                subscriptLst: subs.clone(),
                componentRef: cref.clone(),
            })
        }
        DAE::ComponentRef::CREF_IDENT { .. } => inNewLast.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outCref)
}

pub fn expandArrayCref(
    mut inCr: &metamodelica::Ref<DAE::ComponentRef>,
    mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut lasttype: metamodelica::Ref<DAE::Type>;
    let mut tmpcref: metamodelica::Ref<DAE::ComponentRef>;
    lasttype = crefLastType(inCr)?;
    lasttype = Types::liftTypeWithDims(lasttype, inDims)?;
    tmpcref = crefSetLastType(inCr, &lasttype)?;
    outCrefs = expandCref(&tmpcref, false)?;
    Ok(outCrefs)
}

fn expandArrayCref1<'__b>(
    mut inCr: &'__b metamodelica::Ref<DAE::ComponentRef>,
    mut inSubscripts: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>,
    mut inAccumSubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inAccumCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inSubscripts) {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: sub, tail: subs }, tail: rest_subs } => {
                let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                crefs = expandArrayCref1(inCr, metamodelica::cons(subs.clone(), rest_subs.clone()), inAccumSubs.clone(), inAccumCrefs)?;
                { (inCr, inSubscripts, inAccumSubs, inAccumCrefs) = (inCr, rest_subs.clone(), metamodelica::cons(sub.clone(), inAccumSubs), crefs); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => {
                return Ok(inAccumCrefs)
            },
            _ => {
                let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                cref = crefSetLastSubs(inCr, &inAccumSubs)?;
                return Ok(metamodelica::cons(cref, inAccumCrefs))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn explode(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outParts: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outParts = Dangerous::listReverseInPlace(explode_tail(inCref, metamodelica::nil())?);
    Ok(outParts)
}

fn explode_tail<'__b>(
    mut inCref: &'__b metamodelica::Ref<DAE::ComponentRef>,
    mut inParts: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    '__tco: loop {
        match &**inCref {
            DAE::ComponentRef::CREF_QUAL {
                componentRef: rest_cr, ..
            } => {
                let mut first_cr: metamodelica::Ref<DAE::ComponentRef>;
                first_cr = ComponentReferenceBasics::crefFirstCref(inCref.clone())?;
                {
                    (inCref, inParts) = (rest_cr, metamodelica::cons(first_cr, inParts));
                    continue '__tco;
                }
            }
            _ => return Ok(metamodelica::cons(inCref.clone(), inParts)),
        }
    }
}

pub fn implode(
    mut inParts: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = implode_reverse(&(inParts.reverse()))?;
    Ok(outCref)
}

pub fn implode_reverse(
    mut inParts: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut first: metamodelica::Ref<DAE::ComponentRef>;
    let mut rest: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inParts)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    first = metamodelica::Own::own(__pa0);
    rest = metamodelica::Own::own(__pa1);
    outCref = implode_tail(&rest, first)?;
    Ok(outCref)
}

fn implode_tail<'__b>(
    mut inParts: &'__b metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inAccumCref: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inParts {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty, subscriptLst: subs }, tail: rest } => {
                let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL { ident: id.clone(), identType: ty.clone(), subscriptLst: subs.clone(), componentRef: inAccumCref });
                { (inParts, inAccumCref) = (rest, cr); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inAccumCref)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn identifierCount(mut inCref: &metamodelica::Ref<DAE::ComponentRef>) -> i32 {
    let mut outIdCount: i32;
    outIdCount = identifierCount_tail(inCref, 0);
    outIdCount
}

fn identifierCount_tail<'__b>(mut inCref: &'__b metamodelica::Ref<DAE::ComponentRef>, mut inAccumCount: i32) -> i32 {
    '__tco: loop {
        match &**inCref {
            DAE::ComponentRef::CREF_QUAL { componentRef: cr, .. } => {
                (inCref, inAccumCount) = (cr, inAccumCount + 1);
                continue '__tco;
            }
            _ => return inAccumCount + 1,
        }
    }
}

pub fn checkCrefSubscriptsBounds(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    checkCrefSubscriptsBounds2(inCref, inCref, inInfo)?;
    Ok(())
}

fn checkCrefSubscriptsBounds2(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inWholeCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = (match &**inCref {
        DAE::ComponentRef::CREF_QUAL {
            identType: ty,
            subscriptLst: subs,
            componentRef: rest_cr,
            ..
        } => {
            checkCrefSubscriptsBounds3(ty, subs.clone(), inWholeCref, inInfo)?;
            checkCrefSubscriptsBounds2(rest_cr, inWholeCref, inInfo)?;
            ()
        }
        DAE::ComponentRef::CREF_IDENT {
            identType: ty,
            subscriptLst: subs,
            ..
        } => {
            checkCrefSubscriptsBounds3(ty, subs.clone(), inWholeCref, inInfo)?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn checkCrefSubscriptsBounds3(
    mut inCrefType: &metamodelica::Ref<DAE::Type>,
    mut inSubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inWholeCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    dims = TypesDump::getDimensions(inCrefType);
    dims = dims.reverse();
    subs = inSubscripts.reverse();
    checkCrefSubscriptsBounds4(&subs, &dims, 1, inWholeCref, inInfo)?;
    Ok(())
}

fn checkCrefSubscriptsBounds4(
    mut inSubscripts: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inDimensions: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inIndex: i32,
    mut inWholeCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match (inSubscripts, inDimensions) {
        (Deref @ metamodelica::ListNode::Cons { head: sub, tail: rest_subs }, Deref @ metamodelica::ListNode::Cons { head: dim, tail: rest_dims }) => {
            let true = (checkCrefSubscriptBounds(metamodelica::AsArg::as_arg(&sub), metamodelica::AsArg::as_arg(&dim), inIndex, inWholeCref, inInfo)) else { return Err("pattern mismatch") };
            checkCrefSubscriptsBounds4(rest_subs, rest_dims, inIndex + 1, inWholeCref, inInfo)?;
            ()
        },
        (Deref @ metamodelica::ListNode::Nil, _) => {
            ()
        },
        (_, Deref @ metamodelica::ListNode::Nil) => {
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn checkCrefSubscriptBounds(
    mut inSubscript: &metamodelica::Ref<DAE::Subscript>,
    mut inDimension: &metamodelica::Ref<DAE::Dimension>,
    mut inIndex: i32,
    mut inWholeCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> bool {
    let mut outIsValid: bool;
    outIsValid = 'mc: {
        let __mc_input = (&**inSubscript, &**inDimension);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Subscript::INDEX { exp: exp @ Deref @ DAE::Exp::ICONST { integer: idx } }, Deref @ DAE::Dimension::DIM_INTEGER { integer: dim }) => {
                    let false = (idx.clone() > 0 && idx.clone() <= dim.clone()) else { return Err("pattern mismatch") };
                    printSubscriptBoundsError(exp.clone(), inDimension, inIndex, inWholeCref, inInfo)?;
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ARRAY { array: expl, .. } }, Deref @ DAE::Dimension::DIM_INTEGER { integer: dim }) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = List::getMemberOnTrue(dim.clone(), metamodelica::AsArg::as_arg(&expl), &move |__a0: i32, __a1: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(subscriptExpOutOfBounds(__a0, &__a1)) })?;
                    printSubscriptBoundsError(exp.clone(), inDimension, inIndex, inWholeCref, inInfo)?;
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
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outIsValid
}

fn subscriptExpOutOfBounds(mut inDimSize: i32, mut inSubscriptExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outOutOfBounds: bool;
    outOutOfBounds = (match &**inSubscriptExp {
        DAE::Exp::ICONST { integer: i } => i.clone() < 1 || i.clone() > inDimSize,
        _ => false,
    });
    outOutOfBounds
}

fn printSubscriptBoundsError(
    mut inSubscriptExp: metamodelica::Ref<DAE::Exp>,
    mut inDimension: &metamodelica::Ref<DAE::Dimension>,
    mut inIndex: i32,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let mut sub_str: ArcStr;
    let mut dim_str: ArcStr;
    let mut idx_str: ArcStr;
    let mut cref_str: ArcStr;
    sub_str = ExpressionBasics::printExpStr(inSubscriptExp)?;
    dim_str = ExpressionBasics::dimensionString(inDimension)?;
    idx_str = intString(inIndex);
    cref_str = ComponentReferenceBasics::printComponentRefStr(inCref)?;
    Error::addSourceMessage(
        &(Error::ARRAY_INDEX_OUT_OF_BOUNDS.clone()),
        list![sub_str, idx_str, dim_str, cref_str],
        inInfo,
    )?;
    Ok(())
}

pub(crate) fn crefAppendedSubs(mut cref: &metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut s: ArcStr;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    s1 = stringDelimitList(toStringList(cref), literal!("_P"));
    s2 = stringDelimitList(
        List::mapMap(
            ComponentReferenceBasics::crefSubs(cref)?,
            &move |__a0: metamodelica::Ref<DAE::Subscript>| Expression::getSubscriptExp(&__a0),
            &ExpressionBasics::printExpStr,
        )?,
        literal!(","),
    );
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*s1);
        __mm_s.push_str(&*literal!("["));
        __mm_s.push_str(&*s2);
        __mm_s.push_str(&*literal!("]"));
        ArcStr::from(__mm_s)
    };
    Ok(s)
}

pub fn writeCref(
    mut file: File::File,
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut escape: File::Escape,
) -> Result<()> {
    let mut c: metamodelica::Ref<DAE::ComponentRef> = cref;
    loop {
        c = (::match_deref::match_deref! { match &(c) {
            Deref @ DAE::ComponentRef::CREF_IDENT { ident: __c_ident, subscriptLst: __c_subscriptLst, .. } => {
                File::writeEscape(file.clone(), __c_ident.clone(), escape);
                writeSubscripts(file, metamodelica::AsArg::as_arg(&__c_subscriptLst), escape)?;
                return Ok(());
                return Err("fail")
            },
            Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$DER", componentRef: __c_componentRef, .. } => {
                File::write(file.clone(), literal!("der("));
                writeCref(file.clone(), __c_componentRef.clone(), escape)?;
                File::write(file, literal!(")"));
                return Ok(());
                return Err("fail")
            },
            Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$CLKPRE", componentRef: __c_componentRef, .. } => {
                File::write(file.clone(), literal!("previous("));
                writeCref(file.clone(), __c_componentRef.clone(), escape)?;
                File::write(file, literal!(")"));
                return Ok(());
                return Err("fail")
            },
            Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: __c_componentRef, ident: __c_ident, subscriptLst: __c_subscriptLst, .. } => {
                File::writeEscape(file.clone(), __c_ident.clone(), escape);
                writeSubscripts(file.clone(), metamodelica::AsArg::as_arg(&__c_subscriptLst), escape)?;
                File::write(file.clone(), literal!("."));
                __c_componentRef.clone()
            },
            _ => return Err("match: no arm matched"),
        } });
    }
    Ok(())
}

pub fn writeSubscripts(
    mut file: File::File,
    mut subs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut escape: File::Escape,
) -> Result<()> {
    let mut first: bool = true;
    let mut i: i32;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    if (subs).is_empty() {
        return Ok(());
    }
    File::write(file.clone(), literal!("["));
    for mut s in &**subs {
        if !(first) {
            File::write(file.clone(), literal!(","));
        } else {
            first = false;
        }
        let () = (::match_deref::match_deref! { match &(s.clone()) {
            Deref @ DAE::Subscript::WHOLEDIM { .. } => {
                File::write(file.clone(), literal!(":"));
                ()
            },
            Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ICONST { integer: __esc_i } } => {
                i = (*__esc_i).clone();
                File::writeInt(file.clone(), i.clone(), literal!("%d"));
                ()
            },
            Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __esc_i } } => {
                i = (*__esc_i).clone();
                File::writeInt(file.clone(), i.clone(), literal!("%d"));
                ()
            },
            Deref @ DAE::Subscript::WHOLE_NONEXP { exp: Deref @ DAE::Exp::ICONST { integer: __esc_i } } => {
                i = (*__esc_i).clone();
                File::writeInt(file.clone(), i.clone(), literal!("%d"));
                ()
            },
            Deref @ DAE::Subscript::SLICE { exp: __esc_exp } => {
                exp = (*__esc_exp).clone();
                File::write(file.clone(), ExpressionBasics::printExpStr(exp.clone())?);
                ()
            },
            Deref @ DAE::Subscript::INDEX { exp: __esc_exp } => {
                exp = (*__esc_exp).clone();
                File::write(file.clone(), ExpressionBasics::printExpStr(exp.clone())?);
                ()
            },
            Deref @ DAE::Subscript::WHOLE_NONEXP { exp: __esc_exp } => {
                exp = (*__esc_exp).clone();
                File::write(file.clone(), ExpressionBasics::printExpStr(exp.clone())?);
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    File::write(file, literal!("]"));
    Ok(())
}

pub(crate) fn getConsumedMemory(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
) -> (metamodelica::Real, metamodelica::Real, metamodelica::Real) {
    let mut szIdents: metamodelica::Real = metamodelica::OrderedFloat((0) as f64);
    let mut szTypes: metamodelica::Real = metamodelica::OrderedFloat((0) as f64);
    let mut szSubs: metamodelica::Real = metamodelica::OrderedFloat((0) as f64);
    let mut cr: metamodelica::Ref<DAE::ComponentRef> = inCref;
    let mut b: bool = true;
    while b {
        (b, cr) = (match &*cr {
            DAE::ComponentRef::CREF_IDENT {
                ident: __cr_ident,
                identType: __cr_identType,
                subscriptLst: __cr_subscriptLst,
            } => {
                szIdents = szIdents + (System::getSizeOfData(__cr_ident.clone())).0;
                szTypes = szTypes + (System::getSizeOfData(__cr_identType.clone())).0;
                szSubs = szSubs + (System::getSizeOfData(__cr_subscriptLst.clone())).0;
                (false, cr)
            }
            DAE::ComponentRef::CREF_QUAL {
                componentRef: __cr_componentRef,
                ident: __cr_ident,
                identType: __cr_identType,
                subscriptLst: __cr_subscriptLst,
            } => {
                szIdents = szIdents + (System::getSizeOfData(__cr_ident.clone())).0;
                szTypes = szTypes + (System::getSizeOfData(__cr_identType.clone())).0;
                szSubs = szSubs + (System::getSizeOfData(__cr_subscriptLst.clone())).0;
                (true, __cr_componentRef.clone())
            }
            _ => (false, cr),
        });
    }
    (szIdents, szTypes, szSubs)
}

pub fn createDifferentiatedCrefName(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inX: metamodelica::Ref<DAE::ComponentRef>,
    mut inMatrixName: &ArcStr,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let debug: bool = false;
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("inCref: "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inCref)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    subs = crefLastSubs(inCref)?;
    outCref = ComponentReferenceBasics::crefStripLastSubs(inCref)?;
    outCref = replaceSubsWithString(&outCref)?;
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("after full type: "));
            __mm_s.push_str(&*TypesDump::printTypeStr(crefTypeFull(
                &(crefStripIterSub(&outCref, &(literal!("")))),
            )?));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    outCref = crefSetLastType(&outCref, &(DAE::T_UNKNOWN_DEFAULT().clone()))?;
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("after strip: "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefListStr(expandCref(
                &outCref, true,
            )?)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    outCref = joinCrefs(
        &outCref,
        ComponentReferenceBasics::makeCrefIdent(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*arcstr::literal!(DAE::partialDerivativeNamePrefix));
                __mm_s.push_str(&*inMatrixName);
                ArcStr::from(__mm_s)
            },
            DAE::T_UNKNOWN_DEFAULT().clone(),
            metamodelica::nil(),
        ),
    )?;
    outCref = joinCrefs(&outCref, inX)?;
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("after join: "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefListStr(expandCref(
                &outCref, true,
            )?)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    outCref = crefSetLastSubs(&outCref, &subs)?;
    outCref = crefSetLastType(&outCref, &(crefLastType(inCref)?))?;
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("outCref: "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&outCref)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(outCref)
}

pub fn isTime(mut cref: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match cref {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isWild(mut cref: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut b: bool;
    b = (match &**cref {
        DAE::ComponentRef::WILD { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn uniqueList(
    mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut uniqueCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    uniqueCrefs = UnorderedSet::unique_list(
        crefs,
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
    )?;
    Ok(uniqueCrefs)
}
