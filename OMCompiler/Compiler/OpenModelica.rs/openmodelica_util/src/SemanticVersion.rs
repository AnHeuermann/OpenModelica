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

use crate::StringUtil;
use crate::System;
use crate::Util;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Version {
    /// Semantic version number MAJOR.MINOR.PATCH, see https://semver.org/.
    SEMVER {
        major: i32,
        minor: i32,
        patch: i32,
        prerelease: metamodelica::List<ArcStr>,
        meta: metamodelica::List<ArcStr>,
    },
    /// Non-semantic version number
    NONSEMVER { version: ArcStr },
}
impl metamodelica::gc::MMTrace for Version {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Version::SEMVER {
                major,
                minor,
                patch,
                prerelease,
                meta,
            } => {
                metamodelica::gc::MMTrace::mm_accept(major, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(minor, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(patch, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prerelease, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(meta, __mmv)?;
                Ok(())
            }
            Version::NONSEMVER { version } => {
                metamodelica::gc::MMTrace::mm_accept(version, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Version {
    fn default() -> Self {
        Self::NONSEMVER {
            version: Default::default(),
        }
    }
}
pub use self::Version::{NONSEMVER, SEMVER};

pub fn parse(mut s: ArcStr, mut nonsemverAsZeroZeroZero: bool) -> Result<Version> {
    let mut v: Version;
    let mut n: i32;
    let mut major: ArcStr;
    let mut minor: ArcStr;
    let mut patch: ArcStr;
    let mut versions: ArcStr;
    let mut prereleaseLst: metamodelica::List<ArcStr>;
    let mut metaLst: metamodelica::List<ArcStr>;
    let mut matches: metamodelica::List<ArcStr>;
    let mut split: metamodelica::List<ArcStr>;
    let mut versionsLst: metamodelica::List<ArcStr>;
    let semverRegex: ArcStr = literal!("^([0-9][0-9]*\\.?[0-9]*\\.?[0-9]*)([+-][0-9A-Za-z.-]*)?$");
    (n, matches) = System::regex(s.clone(), semverRegex, 5, true, false);
    if n < 2 {
        if ((s).len() as i32) == 0 {
            v = Version::NONSEMVER { version: literal!("") };
            return Ok(v);
        }
        if nonsemverAsZeroZeroZero {
            (prereleaseLst, metaLst) = splitPrereleaseAndMeta(s)?;
            v = Version::SEMVER {
                major: 0,
                minor: 0,
                patch: 0,
                prerelease: prereleaseLst,
                meta: metaLst,
            };
        } else {
            v = Version::NONSEMVER { version: s };
        }
        return Ok(v);
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(matches) {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    versions = metamodelica::Own::own(__pa0);
    split = metamodelica::Own::own(__pa1);
    versionsLst = Util::stringSplitAtChar(versions, literal!("."))?;
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(versionsLst) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    major = metamodelica::Own::own(__pa3);
    versionsLst = metamodelica::Own::own(__pa4);
    if !((versionsLst).is_empty()) {
        let (__pa5, __pa6) = ::match_deref::match_deref! { match &(versionsLst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: __pa6 } => (__pa5.clone(), __pa6.clone()),
            _ => return Err("pattern mismatch"),
        } };
        minor = metamodelica::Own::own(__pa5);
        versionsLst = metamodelica::Own::own(__pa6);
    } else {
        minor = literal!("0");
    }
    if !((versionsLst).is_empty()) {
        let (__pa7, __pa8) = ::match_deref::match_deref! { match &(versionsLst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: __pa8 } => (__pa7.clone(), __pa8.clone()),
            _ => return Err("pattern mismatch"),
        } };
        patch = metamodelica::Own::own(__pa7);
        versionsLst = metamodelica::Own::own(__pa8);
    } else {
        patch = literal!("0");
    }
    (prereleaseLst, metaLst) = splitPrereleaseAndMeta(if ((split).is_empty()) {
        literal!("")
    } else {
        (split).get(1)?
    })?;
    v = Version::SEMVER {
        major: stringInt(major)?,
        minor: stringInt(minor)?,
        patch: stringInt(patch)?,
        prerelease: prereleaseLst,
        meta: metaLst,
    };
    Ok(v)
}

pub fn compare(
    mut v1: &Version,
    mut v2: &Version,
    mut comparePrerelease: bool,
    mut compareBuildInformation: bool,
) -> Result<i32> {
    let mut c: i32;
    c = (match (v1.clone(), v2.clone()) {
        (Version::NONSEMVER { .. }, Version::NONSEMVER { .. }) => stringCompare(
            &var_field!(v1.version, Version::NONSEMVER),
            &var_field!(v2.version, Version::NONSEMVER),
        ),
        (Version::NONSEMVER { .. }, _) => -1,
        (_, Version::NONSEMVER { .. }) => 1,
        (Version::SEMVER { .. }, Version::SEMVER { .. }) => {
            if var_field!(v1.major, Version::SEMVER).clone() == 0
                && var_field!(v1.minor, Version::SEMVER).clone() == 0
                && var_field!(v1.patch, Version::SEMVER).clone() == 0
                || var_field!(v2.major, Version::SEMVER).clone() == 0
                    && var_field!(v2.minor, Version::SEMVER).clone() == 0
                    && var_field!(v2.patch, Version::SEMVER).clone() == 0
            {
                c = 0;
            } else {
                c = Util::intCompare(
                    var_field!(v1.major, Version::SEMVER).clone(),
                    var_field!(v2.major, Version::SEMVER).clone(),
                );
                if c != 0 {
                    return Ok(c);
                }
                c = Util::intCompare(
                    var_field!(v1.minor, Version::SEMVER).clone(),
                    var_field!(v2.minor, Version::SEMVER).clone(),
                );
                if c != 0 {
                    return Ok(c);
                }
                c = Util::intCompare(
                    var_field!(v1.patch, Version::SEMVER).clone(),
                    var_field!(v2.patch, Version::SEMVER).clone(),
                );
                if c != 0 {
                    return Ok(c);
                }
            }
            if comparePrerelease {
                c = compareIdentifierList(
                    var_field!(v1.prerelease, Version::SEMVER).clone(),
                    var_field!(v2.prerelease, Version::SEMVER).clone(),
                )?;
            }
            if c == 0 && compareBuildInformation {
                c = compareIdentifierList(
                    var_field!(v1.meta, Version::SEMVER).clone(),
                    var_field!(v2.meta, Version::SEMVER).clone(),
                )?;
            }
            c
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(c)
}

pub fn toString(mut v: &Version) -> ArcStr {
    let mut out: ArcStr;
    out = (match v.clone() {
        Version::SEMVER { .. } => {
            out = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ArcStr::from(::std::format!(
                    "{}",
                    var_field!(v.major, Version::SEMVER).clone()
                )));
                __mm_s.push_str(&*literal!("."));
                __mm_s.push_str(&*ArcStr::from(::std::format!(
                    "{}",
                    var_field!(v.minor, Version::SEMVER).clone()
                )));
                __mm_s.push_str(&*literal!("."));
                __mm_s.push_str(&*ArcStr::from(::std::format!(
                    "{}",
                    var_field!(v.patch, Version::SEMVER).clone()
                )));
                ArcStr::from(__mm_s)
            };
            if !((var_field!(v.prerelease, Version::SEMVER)).is_empty()) {
                out = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*out);
                    __mm_s.push_str(&*literal!("-"));
                    __mm_s.push_str(&*stringDelimitList(
                        var_field!(v.prerelease, Version::SEMVER).clone(),
                        literal!("."),
                    ));
                    ArcStr::from(__mm_s)
                };
            }
            if !((var_field!(v.meta, Version::SEMVER)).is_empty()) {
                out = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*out);
                    __mm_s.push_str(&*literal!("+"));
                    __mm_s.push_str(&*stringDelimitList(
                        var_field!(v.meta, Version::SEMVER).clone(),
                        literal!("."),
                    ));
                    ArcStr::from(__mm_s)
                };
            }
            out
        }
        Version::NONSEMVER { .. } => var_field!(v.version, Version::NONSEMVER).clone(),
    });
    out
}

pub fn isPrerelease(mut v: &Version) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(v) {
        Version::SEMVER { prerelease: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn hasMetaInformation(mut v: &Version) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(v) {
        Version::SEMVER { meta: Deref @ metamodelica::ListNode::Nil, .. } => false,
        Version::NONSEMVER { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isSemVer(mut v: &Version) -> bool {
    let mut b: bool;
    b = (match v.clone() {
        Version::SEMVER { .. } => true,
        _ => false,
    });
    b
}

fn splitPrereleaseAndMeta(mut s: ArcStr) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut prereleaseLst: metamodelica::List<ArcStr>;
    let mut metaLst: metamodelica::List<ArcStr>;
    let mut meta: ArcStr;
    let mut prerelease: ArcStr;
    let mut split: metamodelica::List<ArcStr>;
    prereleaseLst = metamodelica::nil();
    metaLst = metamodelica::nil();
    if stringEmpty(&s) {
        return Ok((prereleaseLst, metaLst));
    }
    if metamodelica::stringEq(&(stringGetStringChar(s.clone(), 1)?), &(literal!("+"))) {
        metaLst = if (((s).len() as i32) > 1) {
            Util::stringSplitAtChar(StringUtil::rest(s)?, literal!("."))?
        } else {
            metamodelica::nil()
        };
        return Ok((prereleaseLst, metaLst));
    }
    split = Util::stringSplitAtChar(s, literal!("+"))?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(split) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    prerelease = metamodelica::Own::own(__pa0);
    split = metamodelica::Own::own(__pa1);
    meta = if ((split).is_empty()) {
        literal!("")
    } else {
        (split).get(1)?
    };
    if metamodelica::stringEq(&(stringGetStringChar(prerelease.clone(), 1)?), &(literal!("-"))) {
        prerelease = StringUtil::rest(prerelease)?;
    }
    prereleaseLst = if (((prerelease).len() as i32) > 0) {
        Util::stringSplitAtChar(prerelease, literal!("."))?
    } else {
        metamodelica::nil()
    };
    metaLst = if (((meta).len() as i32) > 0) {
        Util::stringSplitAtChar(meta, literal!("."))?
    } else {
        metamodelica::nil()
    };
    Ok((prereleaseLst, metaLst))
}

fn compareIdentifierList(mut w1: metamodelica::List<ArcStr>, mut w2: metamodelica::List<ArcStr>) -> Result<i32> {
    let mut c: i32;
    let mut l1: metamodelica::List<ArcStr>;
    let mut l2: metamodelica::List<ArcStr>;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    l1 = w1;
    l2 = w2;
    if (l1).is_empty() && !((l2).is_empty()) {
        c = 1;
    }
    if (l2).is_empty() && !((l1).is_empty()) {
        c = -1;
    }
    while !((l1).is_empty() && (l2).is_empty()) {
        (c, l1, l2) = (::match_deref::match_deref! { match &((l1.clone(), l2.clone())) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => (-1, l1, l2),
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Nil) => (1, l1, l2),
            (Deref @ metamodelica::ListNode::Cons { head: __esc_s1, tail: __esc_l1 }, Deref @ metamodelica::ListNode::Cons { head: __esc_s2, tail: __esc_l2 }) => {
                s1 = (*__esc_s1).clone();
                l1 = (*__esc_l1).clone();
                s2 = (*__esc_s2).clone();
                l2 = (*__esc_l2).clone();
                (compareIdentifier(s1.clone(), s2.clone())?, l1.clone(), l2.clone())
            },
            _ => return Err("match: no arm matched"),
        } });
        if c != 0 {
            return Ok(c);
        }
    }
    c = 0;
    Ok(c)
}

fn compareIdentifier(mut s1: ArcStr, mut s2: ArcStr) -> Result<i32> {
    let mut c: i32;
    if Util::isIntegerString(s1.clone()) {
        c = if (Util::isIntegerString(s2.clone())) {
            Util::intCompare(stringInt(s1)?, stringInt(s2)?)
        } else {
            -1
        };
        return Ok(c);
    }
    if Util::isIntegerString(s2.clone()) {
        c = 1;
    }
    c = stringCompare(&s1, &s2);
    Ok(c)
}
