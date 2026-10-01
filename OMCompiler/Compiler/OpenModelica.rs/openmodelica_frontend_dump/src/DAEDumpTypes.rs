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

use crate::SCodeDump;
use crate::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::System;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct splitElements {
    pub v: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    pub ie: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    pub ia: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    pub e: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    pub a: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    pub co: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    pub o: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    pub ca: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    pub sm: metamodelica::List<metamodelica::Ref<compWithSplitElements>>,
}

impl metamodelica::gc::MMTrace for splitElements {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.v, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ie, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ia, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.e, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.a, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.co, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.o, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ca, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.sm, __mmv)?;
        Ok(())
    }
}
impl Default for splitElements {
    fn default() -> Self {
        Self {
            v: Default::default(),
            ie: Default::default(),
            ia: Default::default(),
            e: Default::default(),
            a: Default::default(),
            co: Default::default(),
            o: Default::default(),
            ca: Default::default(),
            sm: Default::default(),
        }
    }
}

pub type SPLIT_ELEMENTS = splitElements;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct compWithSplitElements {
    pub name: ArcStr,
    pub spltElems: metamodelica::Ref<splitElements>,
    pub comment: Option<metamodelica::Ref<SCode::Comment>>,
}

impl metamodelica::gc::MMTrace for compWithSplitElements {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.spltElems, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.comment, __mmv)?;
        Ok(())
    }
}
impl Default for compWithSplitElements {
    fn default() -> Self {
        Self {
            name: Default::default(),
            spltElems: Default::default(),
            comment: Default::default(),
        }
    }
}

pub type COMP_WITH_SPLIT = compWithSplitElements;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct functionList {
    pub funcs: metamodelica::List<DAE::Function>,
}

impl metamodelica::gc::MMTrace for functionList {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.funcs, __mmv)?;
        Ok(())
    }
}
impl Default for functionList {
    fn default() -> Self {
        Self {
            funcs: Default::default(),
        }
    }
}

pub type FUNCTION_LIST = functionList;

pub fn dumpCommentStr(mut inComment: Option<metamodelica::Ref<SCode::Comment>>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inComment) {
        Some(Deref @ SCode::Comment { comment: Some(cmt), .. }) => {
            let mut cmt = (*cmt).clone();
            cmt = System::escapedString(cmt.clone(), false);
            stringAppendList(list![literal!(" \""), cmt.clone(), literal!("\"")])
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outString
}

pub fn dumpClassAnnotationStr(mut inComment: Option<metamodelica::Ref<SCode::Comment>>) -> ArcStr {
    let mut outString: ArcStr;
    outString = dumpAnnotationStr(inComment, &(literal!("  ")), &(literal!(";\n")));
    outString
}

pub fn dumpCommentAnnotationStr(mut inComment: Option<metamodelica::Ref<SCode::Comment>>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inComment.clone()) {
        None => literal!(""),
        _ => { let mut __mm_s = String::new(); __mm_s.push_str(&*dumpCommentStr(inComment.clone())); __mm_s.push_str(&*dumpCompAnnotationStr(inComment)); ArcStr::from(__mm_s) },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outString
}

pub fn dumpCompAnnotationStr(mut inComment: Option<metamodelica::Ref<SCode::Comment>>) -> ArcStr {
    let mut outString: ArcStr;
    outString = dumpAnnotationStr(inComment, &(literal!(" ")), &(literal!("")));
    outString
}

fn dumpAnnotationStr(
    mut inComment: Option<metamodelica::Ref<SCode::Comment>>,
    mut inPrefix: &ArcStr,
    mut inSuffix: &ArcStr,
) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = inComment;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: ann_mod }), .. }) => {
                    let mut ann: ArcStr;
                    let mut ann_mod = (*ann_mod).clone();
                    if Config::showAnnotations()? {
                        ann = { let mut __mm_s = String::new(); __mm_s.push_str(&*inPrefix); __mm_s.push_str(&*literal!("annotation")); __mm_s.push_str(&*SCodeDump::printModStr(ann_mod.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*inSuffix); ArcStr::from(__mm_s) };
                    } else if Config::showStructuralAnnotations()? {
                        ann_mod = filterStructuralMods(ann_mod.clone())?;
                        if !(SCodeUtil::isEmptyMod(metamodelica::AsArg::as_arg(&ann_mod))) {
                            ann = { let mut __mm_s = String::new(); __mm_s.push_str(&*inPrefix); __mm_s.push_str(&*literal!("annotation")); __mm_s.push_str(&*SCodeDump::printModStr(ann_mod.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*inSuffix); ArcStr::from(__mm_s) };
                        } else {
                            ann = literal!("");
                        }
                    } else {
                        ann = literal!("");
                    }
                    Ok(ann.clone())
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

pub fn filterStructuralMods(mut r#mod: metamodelica::Ref<SCode::Mod>) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod;
    r#mod = SCodeUtil::filterSubMods(
        r#mod,
        &move |__a0: metamodelica::Ref<SCode::SubMod>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(filterStructuralMod(&__a0))
        },
    )?;
    Ok(r#mod)
}

fn filterStructuralMod(mut r#mod: &metamodelica::Ref<SCode::SubMod>) -> bool {
    let mut keep: bool;
    keep = (::match_deref::match_deref! { match &(r#mod.ident.clone()) {
        Deref @ "Evaluate" => true,
        Deref @ "Inline" => true,
        Deref @ "LateInline" => true,
        Deref @ "derivative" => true,
        Deref @ "inverse" => true,
        Deref @ "smoothOrder" => true,
        Deref @ "InlineAfterIndexReduction" => true,
        Deref @ "GenerateEvents" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    keep
}
