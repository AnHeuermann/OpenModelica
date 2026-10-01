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

use crate::AbsynDumpTpl;
use openmodelica_ast::Absyn;
use openmodelica_tpl::Tpl;
use openmodelica_util::Config;
use openmodelica_util::File;
use openmodelica_util::File::Escape;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Print;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct DumpOptions {
    pub fileName: ArcStr,
}

impl metamodelica::gc::MMTrace for DumpOptions {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.fileName, __mmv)?;
        Ok(())
    }
}
pub type DUMPOPTIONS = DumpOptions;

pub static defaultDumpOptions: DumpOptions = DumpOptions { fileName: literal!("") };

pub fn boolUnparseFileFromInfo(mut info: &SourceInfo, mut options: &DumpOptions) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((options, info)) {
        (DumpOptions { fileName: Deref @ "" }, _) => true,
        (DumpOptions { .. }, SourceInfo { .. }) => options.fileName.clone() == info.fileName.clone(),
        _ => return Err("match: no arm matched"),
    } });
    Ok(b)
}

pub fn unparseStr(mut inProgram: Absyn::Program, mut markup: bool, mut options: DumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString2(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: Absyn::Program, __a2: DumpOptions| {
            AbsynDumpTpl::dump(__a0, &__a1, &__a2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, Absyn::Program, DumpOptions) -> Result<Tpl::Text> + 'static,
            >),
        inProgram,
        options,
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub(crate) fn unparseClassList(mut inClasses: metamodelica::List<metamodelica::Ref<Absyn::Class>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString2(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: Absyn::Program, __a2: DumpOptions| {
            AbsynDumpTpl::dump(__a0, &__a1, &__a2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, Absyn::Program, DumpOptions) -> Result<Tpl::Text> + 'static,
            >),
        Absyn::Program {
            classes: inClasses,
            within_: openmodelica_ast::Absyn::Within::TOP,
        },
        defaultDumpOptions.clone(),
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub fn unparseClassStr(mut inClass: metamodelica::Ref<Absyn::Class>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString3(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Ref<Absyn::Class>, __a2: ArcStr, __a3: DumpOptions| {
                AbsynDumpTpl::dumpClass(__a0, &__a1, &__a2, &__a3)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<Absyn::Class>, ArcStr, DumpOptions) -> Result<Tpl::Text>
                    + 'static,
            >),
        inClass,
        literal!(""),
        defaultDumpOptions.clone(),
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub fn unparseWithin(mut inWithin: Absyn::Within) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: Absyn::Within| AbsynDumpTpl::dumpWithin(__a0, &__a1))
            as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Absyn::Within) -> Result<Tpl::Text> + 'static>),
        inWithin,
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub fn unparseClassAttributesStr(mut inClass: &metamodelica::Ref<Absyn::Class>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inClass {
        Absyn::Class {
            partialPrefix: p,
            finalPrefix: f,
            encapsulatedPrefix: e,
            restriction: r,
            ..
        } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s2_1: ArcStr;
            let mut s3: ArcStr;
            let mut r#str: ArcStr;
            s1 = if (p.clone()) {
                literal!("partial ")
            } else {
                literal!("")
            };
            s2 = if (f.clone()) { literal!("final ") } else { literal!("") };
            s2_1 = if (e.clone()) {
                literal!("encapsulated ")
            } else {
                literal!("")
            };
            s3 = unparseRestrictionStr(r.clone())?;
            r#str = stringAppendList(list![s2_1, s1, s2, s3]);
            r#str
        }
    });
    Ok(outString)
}

pub(crate) fn unparseCommentOption(mut inComment: Option<metamodelica::Ref<Absyn::Comment>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: Option<metamodelica::Ref<Absyn::Comment>>| {
                AbsynDumpTpl::dumpCommentOpt(__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, Option<metamodelica::Ref<Absyn::Comment>>) -> Result<Tpl::Text> + 'static,
            >),
        inComment,
    )?;
    Ok(outString)
}

pub fn unparseRestrictionStr(mut inRestriction: Absyn::Restriction) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: Absyn::Restriction| {
            AbsynDumpTpl::dumpRestriction(__a0, &__a1)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Absyn::Restriction) -> Result<Tpl::Text> + 'static>),
        inRestriction,
    )?;
    Ok(outString)
}

pub(crate) fn unparseEachStr(mut inEach: Absyn::Each) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inEach {
        Absyn::Each::EACH { .. } => literal!("each "),
        Absyn::Each::NON_EACH { .. } => literal!(""),
    });
    outString
}

pub fn unparseElementArgStr(mut inElementArg: metamodelica::Ref<Absyn::ElementArg>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<Absyn::ElementArg>| {
            AbsynDumpTpl::dumpElementArg(__a0, &__a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<Absyn::ElementArg>) -> Result<Tpl::Text> + 'static,
            >),
        inElementArg,
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub(crate) fn shouldSeparateAfterElementArg(
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> metamodelica::List<(metamodelica::Ref<Absyn::ElementArg>, bool)> {
    let mut outArgs: metamodelica::List<(metamodelica::Ref<Absyn::ElementArg>, bool)>;
    let mut numNonComment: i32 = 0;
    let mut cur: i32 = 0;
    let mut b: bool;
    for mut arg in &**args {
        numNonComment = (match &*arg.clone() {
            Absyn::ElementArg::ELEMENTARGCOMMENT { .. } => numNonComment,
            _ => numNonComment + 1,
        });
    }
    outArgs = metamodelica::nil();
    for mut arg in &**args {
        b = (match &*arg.clone() {
            Absyn::ElementArg::ELEMENTARGCOMMENT { .. } => false,
            _ => {
                cur = cur + 1;
                cur < numNonComment
            }
        });
        outArgs = metamodelica::cons((arg.clone(), b), outArgs);
    }
    outArgs = outArgs.reverse();
    outArgs
}

pub(crate) fn unparseElementItemStr(mut inElementItem: metamodelica::Ref<Absyn::ElementItem>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString2(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Ref<Absyn::ElementItem>, __a2: DumpOptions| {
                AbsynDumpTpl::dumpElementItem(__a0, &__a1, &__a2)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<Absyn::ElementItem>, DumpOptions) -> Result<Tpl::Text>
                    + 'static,
            >),
        inElementItem,
        defaultDumpOptions.clone(),
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub fn unparseAnnotation(mut inAnnotation: metamodelica::Ref<Absyn::Annotation>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<Absyn::Annotation>| {
            AbsynDumpTpl::dumpAnnotation(__a0, &__a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<Absyn::Annotation>) -> Result<Tpl::Text> + 'static,
            >),
        inAnnotation,
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub fn unparseAnnotationOption(mut inAbsynAnnotation: Option<metamodelica::Ref<Absyn::Annotation>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inAbsynAnnotation) {
        Some(ann) => {
            unparseAnnotation(ann.clone())?
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub(crate) fn unparseInnerOuterStr(mut inInnerOuter: Absyn::InnerOuter) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inInnerOuter {
        Absyn::InnerOuter::INNER { .. } => literal!("inner "),
        Absyn::InnerOuter::OUTER { .. } => literal!("outer "),
        Absyn::InnerOuter::INNER_OUTER { .. } => literal!("inner outer "),
        Absyn::InnerOuter::NOT_INNER_OUTER { .. } => literal!(""),
    });
    outString
}

fn unparseGroupImport(mut gimp: &Absyn::GroupImport) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match gimp.clone() {
        Absyn::GroupImport::GROUP_IMPORT_NAME { name: mut name } => name.clone(),
        Absyn::GroupImport::GROUP_IMPORT_RENAME {
            rename: mut rename,
            name: mut name,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*rename);
            __mm_s.push_str(&*literal!(" = "));
            __mm_s.push_str(&*name);
            ArcStr::from(__mm_s)
        }
    });
    r#str
}

pub fn unparseImportStr(mut inImport: Absyn::Import) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: Absyn::Import| AbsynDumpTpl::dumpImport(__a0, &__a1))
            as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Absyn::Import) -> Result<Tpl::Text> + 'static>),
        inImport,
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

fn unparseVariabilitySymbolStr(mut inVariability: Absyn::Variability) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inVariability {
        Absyn::Variability::VAR { .. } => literal!(""),
        Absyn::Variability::DISCRETE { .. } => literal!("discrete "),
        Absyn::Variability::PARAM { .. } => literal!("parameter "),
        Absyn::Variability::CONST { .. } => literal!("constant "),
    });
    outString
}

pub fn unparseDirectionSymbolStr(mut inDirection: Absyn::Direction) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inDirection {
        Absyn::Direction::BIDIR { .. } => literal!(""),
        Absyn::Direction::INPUT { .. } => literal!("input "),
        Absyn::Direction::OUTPUT { .. } => literal!("output "),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub fn directionSymbol(mut inDirection: Absyn::Direction) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inDirection {
        Absyn::Direction::BIDIR { .. } => literal!(""),
        Absyn::Direction::INPUT { .. } => literal!("input"),
        Absyn::Direction::OUTPUT { .. } => literal!("output"),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub(crate) fn unparseParallelismSymbolStr(mut inParallelism: Absyn::Parallelism) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inParallelism {
        Absyn::Parallelism::NON_PARALLEL { .. } => literal!(""),
        Absyn::Parallelism::PARGLOBAL { .. } => literal!("parglobal "),
        Absyn::Parallelism::PARLOCAL { .. } => literal!("parlocal "),
    });
    outString
}

pub fn unparseComponentCondition(mut inComponentCondition: Option<metamodelica::Ref<Absyn::Exp>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: Option<metamodelica::Ref<Absyn::Exp>>| {
            AbsynDumpTpl::dumpComponentCondition(__a0, &__a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, Option<metamodelica::Ref<Absyn::Exp>>) -> Result<Tpl::Text> + 'static,
            >),
        inComponentCondition,
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub fn printArraydimStr(mut s: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = printSubscriptsStr(s)?;
    Ok(r#str)
}

pub fn printSubscriptStr(mut inSubscript: &metamodelica::Ref<Absyn::Subscript>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inSubscript {
        Absyn::Subscript::NOSUB { .. } => {
            literal!(":")
        }
        Absyn::Subscript::SUBSCRIPT { subscript: e1 } => {
            let mut s: ArcStr;
            s = printExpStr(e1.clone())?;
            s
        }
    });
    Ok(outString)
}

pub fn unparseModificationStr(mut inModification: metamodelica::Ref<Absyn::Modification>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<Absyn::Modification>| {
            AbsynDumpTpl::dumpModification(__a0, &__a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<Absyn::Modification>) -> Result<Tpl::Text> + 'static,
            >),
        inModification,
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub fn equationName(mut eq: &metamodelica::Ref<Absyn::Equation>) -> Result<ArcStr> {
    let mut name: ArcStr;
    name = (match &**eq {
        Absyn::Equation::EQ_IF { .. } => literal!("if"),
        Absyn::Equation::EQ_EQUALS { .. } => literal!("equals"),
        Absyn::Equation::EQ_PDE { .. } => literal!("pde"),
        Absyn::Equation::EQ_CONNECT { .. } => literal!("connect"),
        Absyn::Equation::EQ_WHEN_E { .. } => literal!("when"),
        Absyn::Equation::EQ_NORETCALL { .. } => literal!("function call"),
        Absyn::Equation::EQ_FAILURE { .. } => literal!("failure"),
        _ => return Err("match: no arm matched"),
    });
    Ok(name)
}

pub fn unparseClassPart(mut classPart: metamodelica::Ref<Absyn::ClassPart>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString3(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Ref<Absyn::ClassPart>, __a2: i32, __a3: DumpOptions| {
                AbsynDumpTpl::dumpClassPart(__a0, &__a1, __a2, &__a3)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        Tpl::Text,
                        metamodelica::Ref<Absyn::ClassPart>,
                        i32,
                        DumpOptions,
                    ) -> Result<Tpl::Text>
                    + 'static,
            >),
        classPart,
        0,
        defaultDumpOptions.clone(),
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub fn unparseEquationStr(mut inEquation: metamodelica::Ref<Absyn::Equation>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<Absyn::Equation>| {
            AbsynDumpTpl::dumpEquation(__a0, &__a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<Absyn::Equation>) -> Result<Tpl::Text> + 'static,
            >),
        inEquation,
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub(crate) fn unparseEquationItemStr(mut inEquation: metamodelica::Ref<Absyn::EquationItem>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<Absyn::EquationItem>| {
            AbsynDumpTpl::dumpEquationItem(__a0, &__a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<Absyn::EquationItem>) -> Result<Tpl::Text> + 'static,
            >),
        inEquation,
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub fn unparseEquationItemStrLst(
    mut inEquationItems: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut inSeparator: ArcStr,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = stringDelimitList(List::map(inEquationItems, &unparseEquationItemStr)?, inSeparator);
    Ok(outString)
}

pub fn unparseAlgorithmStrLst(
    mut inAlgorithmItems: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    mut inSeparator: ArcStr,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = stringDelimitList(List::map(inAlgorithmItems, &unparseAlgorithmStr)?, inSeparator);
    Ok(outString)
}

pub fn unparseAlgorithmStr(mut inAlgorithmItem: metamodelica::Ref<Absyn::AlgorithmItem>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<Absyn::AlgorithmItem>| {
            AbsynDumpTpl::dumpAlgorithmItem(__a0, &__a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<Absyn::AlgorithmItem>) -> Result<Tpl::Text> + 'static,
            >),
        inAlgorithmItem,
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub fn printComponentRefStr(mut inComponentRef: &metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inComponentRef {
        Absyn::ComponentRef::CREF_IDENT {
            name: s,
            subscripts: subs,
        } => {
            let mut subsstr: ArcStr;
            let mut s_1: ArcStr;
            subsstr = printSubscriptsStr(subs.clone())?;
            s_1 = stringAppend(s.clone(), subsstr);
            s_1
        }
        Absyn::ComponentRef::CREF_QUAL {
            name: s,
            subscripts: subs,
            componentRef: cr,
        } => {
            let mut subsstr: ArcStr;
            let mut s_1: ArcStr;
            let mut crs: ArcStr;
            let mut s_2: ArcStr;
            let mut s_3: ArcStr;
            crs = printComponentRefStr(cr)?;
            subsstr = printSubscriptsStr(subs.clone())?;
            s_1 = stringAppend(s.clone(), subsstr);
            s_2 = stringAppend(s_1, literal!("."));
            s_3 = stringAppend(s_2, crs);
            s_3
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cr } => {
            let mut crs: ArcStr;
            let mut s_3: ArcStr;
            crs = printComponentRefStr(cr)?;
            s_3 = stringAppend(literal!("."), crs);
            s_3
        }
        Absyn::ComponentRef::ALLWILD { .. } => {
            literal!("__")
        }
        Absyn::ComponentRef::WILD { .. } => {
            if (Config::acceptMetaModelicaGrammar()?) {
                literal!("_")
            } else {
                literal!("")
            }
        }
    });
    Ok(outString)
}

pub(crate) fn printSubscriptsStr(
    mut inAbsynSubscriptLst: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inAbsynSubscriptLst) {
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        l => {
            let mut s: ArcStr;
            let mut s_1: ArcStr;
            let mut s_2: ArcStr;
            s = printListStr(metamodelica::AsArg::as_arg(&l), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Subscript>| printSubscriptStr(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Subscript>) -> Result<ArcStr> + 'static>), literal!(","))?;
            s_1 = stringAppend(literal!("["), s);
            s_2 = stringAppend(s_1, literal!("]"));
            s_2
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub fn printFunctionArgsStr(mut inFunctionArgs: &metamodelica::Ref<Absyn::FunctionArgs>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = &**inFunctionArgs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: expargs @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, argNames: nargs @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut r#str: ArcStr;
                    s1 = printListStr(metamodelica::AsArg::as_arg(&expargs), (std::sync::Arc::new(printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<ArcStr> + 'static>), literal!(", "))?;
                    s2 = stringAppend(s1.clone(), literal!(", "));
                    s3 = printListStr(metamodelica::AsArg::as_arg(&nargs), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::NamedArg>| printNamedArgStr(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::NamedArg>) -> Result<ArcStr> + 'static>), literal!(", "))?;
                    r#str = stringAppend(s2.clone(), s3.clone());
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Nil, argNames: nargs } => {
                    let mut r#str: ArcStr;
                    r#str = printListStr(metamodelica::AsArg::as_arg(&nargs), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::NamedArg>| printNamedArgStr(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::NamedArg>) -> Result<ArcStr> + 'static>), literal!(", "))?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: expargs, argNames: Deref @ metamodelica::ListNode::Nil } => {
                    let mut r#str: ArcStr;
                    r#str = printListStr(metamodelica::AsArg::as_arg(&expargs), (std::sync::Arc::new(printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<ArcStr> + 'static>), literal!(", "))?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { exp, iterators, .. } => {
                    let mut r#str: ArcStr;
                    let mut estr: ArcStr;
                    let mut istr: ArcStr;
                    estr = printExpStr(exp.clone())?;
                    istr = printIteratorsStr(metamodelica::AsArg::as_arg(&iterators));
                    r#str = stringAppendList(list![estr.clone(), literal!(" for "), istr.clone()]);
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

pub fn printIteratorsStr(mut iterators: &metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>) -> ArcStr {
    let mut iteratorsStr: ArcStr;
    iteratorsStr = 'mc: {
        let __mc_input = &**iterators;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ForIterator { name: id, guardExp: Some(guardExp), range: Some(exp) }, tail: Deref @ metamodelica::ListNode::Nil } => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    s1 = printExpStr(exp.clone())?;
                    s2 = printExpStr(guardExp.clone())?;
                    s = stringAppendList(list![id.clone(), literal!(" guard "), s2.clone(), literal!(" in "), s1.clone()]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ForIterator { name: id, guardExp: None, range: Some(exp) }, tail: Deref @ metamodelica::ListNode::Nil } => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    s1 = printExpStr(exp.clone())?;
                    s = stringAppendList(list![id.clone(), literal!(" in "), s1.clone()]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ForIterator { name: id, guardExp: None, range: None }, tail: Deref @ metamodelica::ListNode::Nil } => {
                    Ok(id.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: x, tail: rest } => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    s1 = printIteratorsStr(&(list![x.clone()]));
                    s2 = printIteratorsStr(metamodelica::AsArg::as_arg(&rest));
                    s = stringAppendList(list![s1.clone(), literal!(", "), s2.clone()]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    iteratorsStr
}

pub fn printNamedArgStr(mut inNamedArg: &metamodelica::Ref<Absyn::NamedArg>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inNamedArg {
        Absyn::NamedArg {
            argName: ident,
            argValue: e,
        } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut r#str: ArcStr;
            s1 = stringAppend(ident.clone(), literal!(" = "));
            s2 = printExpStr(e.clone())?;
            r#str = stringAppend(s1, s2);
            r#str
        }
    });
    Ok(outString)
}

pub fn printNamedArgValueStr(mut inNamedArg: &metamodelica::Ref<Absyn::NamedArg>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inNamedArg {
        Absyn::NamedArg { argValue: e, .. } => {
            let mut r#str: ArcStr;
            r#str = printExpStr(e.clone())?;
            r#str
        }
    });
    Ok(outString)
}

pub fn shouldParenthesize(
    mut inOperand: &metamodelica::Ref<Absyn::Exp>,
    mut inOperator: &metamodelica::Ref<Absyn::Exp>,
    mut inLhs: bool,
) -> Result<bool> {
    let mut outShouldParenthesize: bool;
    outShouldParenthesize = (match &**inOperand {
        Absyn::Exp::UNARY { .. } => true,
        _ => {
            let mut diff: i32;
            diff = Util::intCompare(expPriority(inOperand, inLhs)?, expPriority(inOperator, inLhs)?);
            shouldParenthesize2(diff, inOperand, inLhs)
        }
    });
    Ok(outShouldParenthesize)
}

fn shouldParenthesize2(mut inPrioDiff: i32, mut inOperand: &metamodelica::Ref<Absyn::Exp>, mut inLhs: bool) -> bool {
    let mut outShouldParenthesize: bool;
    outShouldParenthesize = (match inPrioDiff {
        1 => true,
        0 => {
            if (inLhs) {
                isNonAssociativeExp(inOperand)
            } else {
                !(isAssociativeExp(inOperand))
            }
        }
        _ => false,
    });
    outShouldParenthesize
}

fn isAssociativeExp(mut inExp: &metamodelica::Ref<Absyn::Exp>) -> bool {
    let mut outIsAssociative: bool;
    outIsAssociative = (match &**inExp {
        Absyn::Exp::BINARY { op, .. } => isAssociativeOp(op.clone()),
        Absyn::Exp::LBINARY { .. } => true,
        _ => false,
    });
    outIsAssociative
}

fn isAssociativeOp(mut inOperator: Absyn::Operator) -> bool {
    let mut outIsAssociative: bool;
    outIsAssociative = (match inOperator {
        Absyn::Operator::ADD { .. } => true,
        Absyn::Operator::ADD_EW { .. } => true,
        Absyn::Operator::MUL_EW { .. } => true,
        _ => false,
    });
    outIsAssociative
}

fn isNonAssociativeExp(mut exp: &metamodelica::Ref<Absyn::Exp>) -> bool {
    let mut isNonAssociative: bool;
    isNonAssociative = (match &**exp {
        Absyn::Exp::BINARY { op: __exp_op, .. } => isNonAssociativeOp(__exp_op.clone()),
        _ => false,
    });
    isNonAssociative
}

fn isNonAssociativeOp(mut operator: Absyn::Operator) -> bool {
    let mut isNonAssociative: bool;
    isNonAssociative = (match operator {
        Absyn::Operator::POW { .. } => true,
        Absyn::Operator::POW_EW { .. } => true,
        _ => false,
    });
    isNonAssociative
}

pub(crate) fn expPriority(mut inExp: &metamodelica::Ref<Absyn::Exp>, mut inLhs: bool) -> Result<i32> {
    let mut outPriority: i32;
    outPriority = (::match_deref::match_deref! { match &((&**inExp, inLhs)) {
        (Deref @ Absyn::Exp::BINARY { op, .. }, false) => {
            priorityBinopRhs(op.clone())?
        },
        (Deref @ Absyn::Exp::BINARY { op, .. }, true) => {
            priorityBinopLhs(op.clone())?
        },
        (Deref @ Absyn::Exp::UNARY { .. }, _) => {
            4
        },
        (Deref @ Absyn::Exp::LBINARY { op, .. }, _) => {
            priorityLBinop(op.clone())?
        },
        (Deref @ Absyn::Exp::LUNARY { .. }, _) => {
            7
        },
        (Deref @ Absyn::Exp::RELATION { .. }, _) => {
            6
        },
        (Deref @ Absyn::Exp::RANGE { .. }, _) => {
            10
        },
        (Deref @ Absyn::Exp::IFEXP { .. }, _) => {
            11
        },
        _ => {
            0
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPriority)
}

fn priorityBinopLhs(mut inOp: Absyn::Operator) -> Result<i32> {
    let mut outPriority: i32;
    outPriority = (match inOp {
        Absyn::Operator::ADD { .. } => 5,
        Absyn::Operator::SUB { .. } => 5,
        Absyn::Operator::MUL { .. } => 2,
        Absyn::Operator::DIV { .. } => 2,
        Absyn::Operator::POW { .. } => 1,
        Absyn::Operator::ADD_EW { .. } => 5,
        Absyn::Operator::SUB_EW { .. } => 5,
        Absyn::Operator::MUL_EW { .. } => 2,
        Absyn::Operator::DIV_EW { .. } => 2,
        Absyn::Operator::POW_EW { .. } => 1,
        _ => return Err("match: no arm matched"),
    });
    Ok(outPriority)
}

fn priorityBinopRhs(mut inOp: Absyn::Operator) -> Result<i32> {
    let mut outPriority: i32;
    outPriority = (match inOp {
        Absyn::Operator::ADD { .. } => 6,
        Absyn::Operator::SUB { .. } => 5,
        Absyn::Operator::MUL { .. } => 2,
        Absyn::Operator::DIV { .. } => 2,
        Absyn::Operator::POW { .. } => 1,
        Absyn::Operator::ADD_EW { .. } => 6,
        Absyn::Operator::SUB_EW { .. } => 5,
        Absyn::Operator::MUL_EW { .. } => 3,
        Absyn::Operator::DIV_EW { .. } => 2,
        Absyn::Operator::POW_EW { .. } => 1,
        _ => return Err("match: no arm matched"),
    });
    Ok(outPriority)
}

fn priorityLBinop(mut inOp: Absyn::Operator) -> Result<i32> {
    let mut outPriority: i32;
    outPriority = (match inOp {
        Absyn::Operator::AND { .. } => 8,
        Absyn::Operator::OR { .. } => 9,
        _ => return Err("match: no arm matched"),
    });
    Ok(outPriority)
}

fn printOperandStr(
    mut inOperand: metamodelica::Ref<Absyn::Exp>,
    mut inOperation: &metamodelica::Ref<Absyn::Exp>,
    mut inLhs: bool,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = inLhs;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut op_str: ArcStr;
            let true = (shouldParenthesize(&inOperand, inOperation, inLhs)?) else {
                return Err("pattern mismatch");
            };
            op_str = printExpStr(inOperand.clone())?;
            op_str = stringAppendList(list![literal!("("), op_str.clone(), literal!(")")]);
            Ok(op_str.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(printExpStr(inOperand.clone())?)
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outString)
}

pub fn printExpLstStr(mut expl: metamodelica::List<metamodelica::Ref<Absyn::Exp>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = stringDelimitList(List::map(expl, &printExpStr)?, literal!(", "));
    Ok(outString)
}

pub fn printExpStr(mut inExp: metamodelica::Ref<Absyn::Exp>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<Absyn::Exp>| {
            AbsynDumpTpl::dumpExp(__a0, &__a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<Absyn::Exp>) -> Result<Tpl::Text> + 'static,
            >),
        inExp,
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub fn printCodeStr(mut inCode: metamodelica::Ref<Absyn::CodeNode>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<Absyn::CodeNode>| {
            AbsynDumpTpl::dumpCodeNode(__a0, &__a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<Absyn::CodeNode>) -> Result<Tpl::Text> + 'static,
            >),
        inCode,
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

fn printListStr<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTypeALst: &metamodelica::List<Type_a>,
    mut inFuncTypeTypeAToString: Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>,
    mut inString: ArcStr,
) -> Result<ArcStr> {
    pub type FuncTypeType_aToString<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = (&**inTypeALst, inFuncTypeTypeAToString.clone(), inString);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: h, tail: Deref @ metamodelica::ListNode::Nil }, r, _) => {
                    let mut s: ArcStr;
                    s = r(h.clone())?;
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: h, tail: t }, r, sep) => {
                    let mut s: ArcStr;
                    let mut srest: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    s = r(h.clone())?;
                    srest = printListStr(metamodelica::AsArg::as_arg(&t), r.clone(), sep.clone())?;
                    s_1 = stringAppend(s.clone(), sep.clone());
                    s_2 = stringAppend(s_1.clone(), srest.clone());
                    Ok(s_2.clone())
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

pub fn opSymbol(mut inOperator: Absyn::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator {
        Absyn::Operator::ADD { .. } => literal!(" + "),
        Absyn::Operator::SUB { .. } => literal!(" - "),
        Absyn::Operator::MUL { .. } => literal!(" * "),
        Absyn::Operator::DIV { .. } => literal!(" / "),
        Absyn::Operator::POW { .. } => literal!(" ^ "),
        Absyn::Operator::UMINUS { .. } => literal!("-"),
        Absyn::Operator::UPLUS { .. } => literal!("+"),
        Absyn::Operator::ADD_EW { .. } => literal!(" .+ "),
        Absyn::Operator::SUB_EW { .. } => literal!(" .- "),
        Absyn::Operator::MUL_EW { .. } => literal!(" .* "),
        Absyn::Operator::DIV_EW { .. } => literal!(" ./ "),
        Absyn::Operator::POW_EW { .. } => literal!(" .^ "),
        Absyn::Operator::UMINUS_EW { .. } => literal!(" .-"),
        Absyn::Operator::UPLUS_EW { .. } => literal!(" .+"),
        Absyn::Operator::AND { .. } => literal!(" and "),
        Absyn::Operator::OR { .. } => literal!(" or "),
        Absyn::Operator::NOT { .. } => literal!("not "),
        Absyn::Operator::LESS { .. } => literal!(" < "),
        Absyn::Operator::LESSEQ { .. } => literal!(" <= "),
        Absyn::Operator::GREATER { .. } => literal!(" > "),
        Absyn::Operator::GREATEREQ { .. } => literal!(" >= "),
        Absyn::Operator::EQUAL { .. } => literal!(" == "),
        Absyn::Operator::NEQUAL { .. } => literal!(" <> "),
    });
    Ok(outString)
}

pub fn opSymbolCompact(mut inOperator: Absyn::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator {
        Absyn::Operator::ADD { .. } => literal!("+"),
        Absyn::Operator::SUB { .. } => literal!("-"),
        Absyn::Operator::MUL { .. } => literal!("*"),
        Absyn::Operator::DIV { .. } => literal!("/"),
        Absyn::Operator::POW { .. } => literal!("^"),
        Absyn::Operator::UMINUS { .. } => literal!("-"),
        Absyn::Operator::UPLUS { .. } => literal!("+"),
        Absyn::Operator::ADD_EW { .. } => literal!("+"),
        Absyn::Operator::SUB_EW { .. } => literal!("-"),
        Absyn::Operator::MUL_EW { .. } => literal!("*"),
        Absyn::Operator::DIV_EW { .. } => literal!("/"),
        Absyn::Operator::POW_EW { .. } => literal!("^"),
        Absyn::Operator::UMINUS_EW { .. } => literal!("-"),
        Absyn::Operator::AND { .. } => literal!("and"),
        Absyn::Operator::OR { .. } => literal!("or"),
        Absyn::Operator::NOT { .. } => literal!("not"),
        Absyn::Operator::LESS { .. } => literal!("<"),
        Absyn::Operator::LESSEQ { .. } => literal!("<="),
        Absyn::Operator::GREATER { .. } => literal!(">"),
        Absyn::Operator::GREATEREQ { .. } => literal!(">="),
        Absyn::Operator::EQUAL { .. } => literal!("=="),
        Absyn::Operator::NEQUAL { .. } => literal!("<>"),
        _ => return Err("fail"),
    });
    Ok(outString)
}

/*
 *
 * Utility functions
 * These are utility functions used in some of the other functions.
 *
 */
pub(crate) fn printOption<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTypeAOption: Option<Type_a>,
    mut inFuncTypeTypeATo: &dyn ::std::ops::Fn(Type_a) -> Result<()>,
) -> Result<()> {
    pub type FuncTypeType_aTo<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>;

    let () = (match inTypeAOption {
        None => {
            Print::printBuf(literal!("NONE()"))?;
            ()
        }
        Some(mut x) => {
            Print::printBuf(literal!("SOME("))?;
            inFuncTypeTypeATo(x)?;
            Print::printBuf(literal!(")"))?;
            ()
        }
    });
    Ok(())
}

pub fn printList<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTypeALst: &metamodelica::List<Type_a>,
    mut inFuncTypeTypeATo: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>,
    mut inString: ArcStr,
) -> Result<()> {
    pub type FuncTypeType_aTo<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>;

    let () = 'mc: {
        let __mc_input = (&**inTypeALst, inFuncTypeTypeATo.clone(), inString);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: h, tail: Deref @ metamodelica::ListNode::Nil }, r, _) => {
                    r(h.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: h, tail: t }, r, sep) => {
                    r(h.clone())?;
                    Print::printBuf(sep.clone())?;
                    printList(metamodelica::AsArg::as_arg(&t), r.clone(), sep.clone())?;
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

fn printStringCommentOption(mut inStringOption: Option<ArcStr>) -> Result<()> {
    let () = (match inStringOption {
        None => {
            Print::printBuf(literal!("NONE()"))?;
            ()
        }
        Some(mut s) => {
            let mut r#str: ArcStr;
            r#str = stringAppendList(list![literal!("SOME(\""), s, literal!("\")")]);
            Print::printBuf(r#str)?;
            ()
        }
    });
    Ok(())
}

pub fn unparseTypeSpec(mut inTypeSpec: metamodelica::Ref<Absyn::TypeSpec>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<Absyn::TypeSpec>| {
            AbsynDumpTpl::dumpTypeSpec(__a0, &__a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<Absyn::TypeSpec>) -> Result<Tpl::Text> + 'static,
            >),
        inTypeSpec,
    )?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status)?;
    Ok(outString)
}

pub(crate) fn printTypeSpec(mut typeSpec: metamodelica::Ref<Absyn::TypeSpec>) -> Result<()> {
    let mut r#str: ArcStr;
    r#str = unparseTypeSpec(typeSpec)?;
    metamodelica::print(r#str);
    Ok(())
}

pub(crate) fn stdout() -> Result<()> {
    let mut r#str: ArcStr;
    r#str = Print::getString()?;
    metamodelica::print(r#str);
    Print::clearBuf();
    Ok(())
}

pub fn writePath(
    mut file: File::File,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut escape: Escape,
    mut delimiter: ArcStr,
    mut initialDot: bool,
) -> Result<()> {
    let mut p: metamodelica::Ref<Absyn::Path> = path;
    loop {
        p = (match &*p {
            Absyn::Path::IDENT { name: __p_name } => {
                File::writeEscape(file, __p_name.clone(), escape);
                return Ok(());
                return Err("fail");
            }
            Absyn::Path::QUALIFIED {
                name: __p_name,
                path: __p_path,
            } => {
                File::writeEscape(file.clone(), __p_name.clone(), escape);
                File::writeEscape(file.clone(), delimiter.clone(), escape);
                __p_path.clone()
            }
            Absyn::Path::FULLYQUALIFIED { path: __p_path } => {
                if initialDot {
                    File::writeEscape(file.clone(), delimiter.clone(), escape);
                }
                __p_path.clone()
            }
        });
    }
    Ok(())
}
