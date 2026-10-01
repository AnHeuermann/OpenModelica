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

use openmodelica_frontend_types::DAE;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ReplacePattern {
    /// from string (ie \".\"
    pub from: ArcStr,
    /// to string (ie \"$p\") ))
    pub to: ArcStr,
}

impl metamodelica::gc::MMTrace for ReplacePattern {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.from, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.to, __mmv)?;
        Ok(())
    }
}
pub type REPLACEPATTERN = ReplacePattern;

pub(crate) static replaceStringPatterns: std::sync::LazyLock<metamodelica::List<ReplacePattern>> =
    std::sync::LazyLock::new(|| {
        list![
            ReplacePattern {
                from: literal!("."),
                to: arcstr::literal!(pointStr)
            },
            ReplacePattern {
                from: literal!("["),
                to: arcstr::literal!(leftBraketStr)
            },
            ReplacePattern {
                from: literal!("]"),
                to: arcstr::literal!(rightBraketStr)
            },
            ReplacePattern {
                from: literal!("("),
                to: arcstr::literal!(leftParStr)
            },
            ReplacePattern {
                from: literal!(")"),
                to: arcstr::literal!(rightParStr)
            },
            ReplacePattern {
                from: literal!(","),
                to: arcstr::literal!(commaStr)
            },
            ReplacePattern {
                from: literal!("'"),
                to: arcstr::literal!(appostrophStr)
            }
        ]
    });

pub(crate) const pointStr: &'static str = "$P";

pub(crate) const leftBraketStr: &'static str = "$lB";

pub(crate) const rightBraketStr: &'static str = "$rB";

pub(crate) const leftParStr: &'static str = "$lP";

pub(crate) const rightParStr: &'static str = "$rP";

pub(crate) const commaStr: &'static str = "$c";

pub(crate) const appostrophStr: &'static str = "$a";

pub(crate) fn modelicaStringToCStr(mut r#str: ArcStr, mut changeDerCall: bool) -> Result<ArcStr> {
    let mut res_str: ArcStr;
    res_str = (match (r#str.clone(), changeDerCall) {
        (_, false) => {
            res_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("$"));
                __mm_s.push_str(&*modelicaStringToCStr1(r#str, &(replaceStringPatterns.clone()))?);
                ArcStr::from(__mm_s)
            };
            res_str
        }
        (mut s, true) => {
            s = modelicaStringToCStr2(s)?;
            s
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(res_str)
}

fn modelicaStringToCStr1(
    mut inString: ArcStr,
    mut inReplacePatternLst: &metamodelica::List<ReplacePattern>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = (inString.clone(), &**inReplacePatternLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#str, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#str, Deref @ metamodelica::ListNode::Cons { head: ReplacePattern { from, to }, tail: res }) => {
                    let mut str_1: ArcStr;
                    let mut res_str: ArcStr;
                    str_1 = modelicaStringToCStr1(r#str.clone(), metamodelica::AsArg::as_arg(&res))?;
                    res_str = System::stringReplace(str_1.clone(), from.clone(), to.clone())?;
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendUtil.modelicaStringToCStr1")); __mm_s.push_str(&*literal!(" failed for str:")); __mm_s.push_str(&*inString); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
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

fn modelicaStringToCStr2(mut inDerName: ArcStr) -> Result<ArcStr> {
    let mut outDerName: ArcStr;
    outDerName = 'mc: {
        let __mc_input = inDerName;
        if let Ok(__v) = (|| -> Result<_> {
            let mut derName = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut name: ArcStr;
            let mut names: metamodelica::List<ArcStr>;
            let 0 = (System::strncmp(derName.clone(), literal!("der("), 4)) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(System::strtok(derName.clone(), literal!("()"))) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            names = metamodelica::Own::own(__pa0);
            names = List::map1(names.clone(), &modelicaStringToCStr, false)?;
            name = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*arcstr::literal!(DAE::derivativeNamePrefix));
                __mm_s.push_str(&*stringAppendList(names.clone()));
                ArcStr::from(__mm_s)
            };
            Ok(name.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut derName = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut name: ArcStr;
            let 0 = (System::strncmp(derName.clone(), literal!("pre("), 4)) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(System::strtok(derName.clone(), literal!("()"))) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            name = metamodelica::Own::own(__pa0);
            name = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("pre("));
                __mm_s.push_str(&*modelicaStringToCStr(name.clone(), false)?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            Ok(name.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut derName = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(modelicaStringToCStr(derName.clone(), false)?)
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outDerName)
}
