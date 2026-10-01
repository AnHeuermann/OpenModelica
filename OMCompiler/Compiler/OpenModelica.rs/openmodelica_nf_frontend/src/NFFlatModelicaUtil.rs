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
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::IOStream;
use openmodelica_util::System;
use openmodelica_util::Util;

// Used to indicate what type of element an annotation comes from, to allow
// filtering out specific annotations for dumping.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum ElementType {
    ROOT_CLASS = 1,
    CLASS = 2,
    FUNCTION = 3,
    COMPONENT = 4,
    EQUATION = 5,
    ALGORITHM = 6,
    OTHER = 7,
}
impl PartialOrd for ElementType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ElementType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for ElementType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) fn appendElementSourceCommentString(
    mut source: &metamodelica::Ref<DAE::ElementSource>,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut opt_cmt: Option<metamodelica::Ref<SCode::Comment>>;
    opt_cmt = ElementSource::getOptComment(source)?;
    if (opt_cmt).is_some() {
        s = appendCommentString(&(opt_cmt.ok_or("pattern mismatch")?), s)?;
    }
    Ok(s)
}

pub(crate) fn appendElementSourceCommentAnnotation(
    mut source: &metamodelica::Ref<DAE::ElementSource>,
    mut elementType: ElementType,
    mut indent: ArcStr,
    mut ending: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut opt_cmt: Option<metamodelica::Ref<SCode::Comment>>;
    opt_cmt = ElementSource::getOptComment(source)?;
    if (opt_cmt).is_some() {
        s = appendCommentAnnotation(&(opt_cmt.ok_or("pattern mismatch")?), elementType, indent, ending, s)?;
    }
    Ok(s)
}

pub(crate) fn appendElementSourceComment(
    mut source: &metamodelica::Ref<DAE::ElementSource>,
    mut elementType: ElementType,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    s = appendCommentOpt(ElementSource::getOptComment(source)?, elementType, s)?;
    Ok(s)
}

pub(crate) fn appendCommentOpt(
    mut comment: Option<metamodelica::Ref<SCode::Comment>>,
    mut elementType: ElementType,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    if (comment).is_some() {
        s = appendComment(&(comment.ok_or("pattern mismatch")?), elementType, s)?;
    }
    Ok(s)
}

pub(crate) fn appendComment(
    mut comment: &metamodelica::Ref<SCode::Comment>,
    mut elementType: ElementType,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    s = appendCommentString(comment, s)?;
    s = appendCommentAnnotation(comment, elementType, literal!(" "), literal!(""), s)?;
    Ok(s)
}

pub(crate) fn appendCommentString(
    mut comment: &metamodelica::Ref<SCode::Comment>,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut r#str: ArcStr;
    let () = (match &**comment {
        SCode::Comment {
            comment: Some(__esc_str),
            ..
        } => {
            r#str = (*__esc_str).clone();
            s = IOStream::append(s, literal!(" \""))?;
            s = IOStream::append(s, System::escapedString(r#str.clone(), false))?;
            s = IOStream::append(s, literal!("\""))?;
            ()
        }
        _ => (),
    });
    Ok(s)
}

pub(crate) fn appendCommentAnnotation(
    mut comment: &metamodelica::Ref<SCode::Comment>,
    mut elementType: ElementType,
    mut indent: ArcStr,
    mut ending: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let () = (::match_deref::match_deref! { match comment {
        Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: __esc_mod }), .. } => {
            r#mod = (*__esc_mod).clone();
            r#mod = (match elementType {
        ElementType::ROOT_CLASS { .. } => filterRootClassAnnotations(r#mod.clone())?,
        _ => DAEDumpTypes::filterStructuralMods(r#mod.clone())?,
    });
            if !(SCodeUtil::isEmptyMod(metamodelica::AsArg::as_arg(&r#mod))) {
                s = IOStream::append(s, indent)?;
                s = IOStream::append(s, literal!("annotation"))?;
                s = appendAnnotationMod(metamodelica::AsArg::as_arg(&r#mod), s)?;
                s = IOStream::append(s, ending)?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}

pub(crate) fn filterRootClassAnnotations(
    mut r#mod: metamodelica::Ref<SCode::Mod>,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    fn filter(mut smod: &metamodelica::Ref<SCode::SubMod>) -> bool {
        let mut keep: bool;
        keep = (::match_deref::match_deref! { match &(smod.ident.clone()) {
            Deref @ "experiment" => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        keep
    }

    let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod;
    r#mod = SCodeUtil::filterSubMods(
        r#mod,
        &move |__a0: metamodelica::Ref<SCode::SubMod>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(filter(&__a0))
        },
    )?;
    Ok(r#mod)
}

pub(crate) fn appendAnnotationMod(
    mut r#mod: &metamodelica::Ref<SCode::Mod>,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let () = (match &**r#mod {
        SCode::Mod::MOD {
            binding: __mod_binding,
            subModLst: __mod_subModLst,
            ..
        } => {
            if !((__mod_subModLst).is_empty()) {
                s = IOStream::append(s, literal!("("))?;
                s = appendAnnotationSubMod(&((__mod_subModLst).head().cloned()?), s)?;
                for mut m in &*(__mod_subModLst).rest()? {
                    s = IOStream::append(s, literal!(", "))?;
                    s = appendAnnotationSubMod(metamodelica::AsArg::as_arg(&m), s)?;
                }
                s = IOStream::append(s, literal!(")"))?;
            }
            if (__mod_binding).is_some() {
                s = IOStream::append(s, literal!(" = "))?;
                s = appendExp(__mod_binding.clone().ok_or("pattern mismatch")?, s)?;
            }
            ()
        }
        _ => (),
    });
    Ok(s)
}

pub(crate) fn appendAnnotationSubMod(
    mut r#mod: &metamodelica::Ref<SCode::SubMod>,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut m: metamodelica::Ref<SCode::Mod> = r#mod.r#mod.clone();
    let () = (match &*m {
        SCode::Mod::MOD {
            eachPrefix: __m_eachPrefix,
            finalPrefix: __m_finalPrefix,
            ..
        } => {
            if SCodeUtil::finalBool(__m_finalPrefix.clone()) {
                s = IOStream::append(s, literal!("final "))?;
            }
            if SCodeUtil::eachBool(__m_eachPrefix.clone()) {
                s = IOStream::append(s, literal!("each "))?;
            }
            s = IOStream::append(s, r#mod.ident.clone())?;
            s = appendAnnotationMod(&m, s)?;
            ()
        }
        _ => (),
    });
    Ok(s)
}

pub(crate) fn appendExp(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    (e, _) = AbsynUtil::traverseExp(
        exp,
        (std::sync::Arc::new(quoteCref)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, i32) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)>
                    + 'static,
            >),
        0,
    )?;
    s = IOStream::append(s, Dump::printExpStr(e)?)?;
    Ok(s)
}

pub(crate) fn quoteCref(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut dummy: i32,
) -> Result<(metamodelica::Ref<Absyn::Exp>, i32)> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut dummy: i32 = dummy;
    let mut r#str: ArcStr;
    let () = (match &*exp {
        Absyn::Exp::CREF {
            componentRef: __exp_componentRef,
        } if (!(AbsynUtil::crefIsWild(metamodelica::AsArg::as_arg(&__exp_componentRef)))) => {
            r#str = Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&__exp_componentRef))?;
            if !metamodelica::stringEq(&r#str, &(literal!("time"))) {
                r#str = Util::makeQuotedIdentifier(r#str)?;
                assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: r#str, subscripts: metamodelica::nil() }));
            }
            ()
        }
        _ => (),
    });
    Ok((exp, dummy))
}
