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

use crate::AbsynUtil;
use crate::Dump;
use crate::SCodeDumpTpl;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::SCode;
use openmodelica_tpl::Tpl;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SCodeDumpOptions {
    pub stripAlgorithmSections: bool,
    pub stripProtectedImports: bool,
    pub stripProtectedClasses: bool,
    pub stripProtectedComponents: bool,
    /// The automatically generated records that change scope from uniontype to the package
    pub stripMetaRecords: bool,
    pub stripGraphicalAnnotations: bool,
    pub stripStringComments: bool,
    pub stripExternalDecl: bool,
    pub stripOutputBindings: bool,
}

impl metamodelica::gc::MMTrace for SCodeDumpOptions {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.stripAlgorithmSections, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stripProtectedImports, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stripProtectedClasses, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stripProtectedComponents, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stripMetaRecords, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stripGraphicalAnnotations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stripStringComments, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stripExternalDecl, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stripOutputBindings, __mmv)?;
        Ok(())
    }
}
impl Default for SCodeDumpOptions {
    fn default() -> Self {
        Self {
            stripAlgorithmSections: Default::default(),
            stripProtectedImports: Default::default(),
            stripProtectedClasses: Default::default(),
            stripProtectedComponents: Default::default(),
            stripMetaRecords: Default::default(),
            stripGraphicalAnnotations: Default::default(),
            stripStringComments: Default::default(),
            stripExternalDecl: Default::default(),
            stripOutputBindings: Default::default(),
        }
    }
}

pub type OPTIONS = SCodeDumpOptions;

pub static defaultOptions: SCodeDumpOptions = SCodeDumpOptions {
    stripAlgorithmSections: false,
    stripProtectedImports: false,
    stripProtectedClasses: false,
    stripProtectedComponents: false,
    stripMetaRecords: true,
    stripGraphicalAnnotations: true,
    stripStringComments: false,
    stripExternalDecl: false,
    stripOutputBindings: false,
};

pub fn generateOptions(
    mut stripAlgorithmSections: bool,
    mut stripProtectedImports: bool,
    mut stripProtectedClasses: bool,
    mut stripProtectedComponents: bool,
    mut stripMetaRecords: bool,
    mut stripGraphicalAnnotations: bool,
    mut stripStringComments: bool,
    mut stripExternalDecl: bool,
    mut stripOutputBindings: bool,
) -> SCodeDumpOptions {
    let mut options: SCodeDumpOptions;
    options = SCodeDumpOptions {
        stripAlgorithmSections: stripAlgorithmSections,
        stripProtectedImports: stripProtectedImports,
        stripProtectedClasses: stripProtectedClasses,
        stripProtectedComponents: stripProtectedComponents,
        stripMetaRecords: stripMetaRecords,
        stripGraphicalAnnotations: stripGraphicalAnnotations,
        stripStringComments: stripStringComments,
        stripExternalDecl: stripExternalDecl,
        stripOutputBindings: stripOutputBindings,
    };
    options
}

pub fn programStr(
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut options: SCodeDumpOptions,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString2(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text,
                  __a1: metamodelica::List<metamodelica::Ref<SCode::Element>>,
                  __a2: SCodeDumpOptions| SCodeDumpTpl::dumpProgram(__a0, &__a1, &__a2),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        Tpl::Text,
                        metamodelica::List<metamodelica::Ref<SCode::Element>>,
                        SCodeDumpOptions,
                    ) -> Result<Tpl::Text>
                    + 'static,
            >),
        inProgram,
        options,
    )?;
    Ok(outString)
}

pub fn classDefStr(mut cd: metamodelica::Ref<SCode::ClassDef>, mut options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString2(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Ref<SCode::ClassDef>, __a2: SCodeDumpOptions| {
                SCodeDumpTpl::dumpClassDef(__a0, &__a1, &__a2)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SCode::ClassDef>, SCodeDumpOptions) -> Result<Tpl::Text>
                    + 'static,
            >),
        cd,
        options,
    )?;
    Ok(outString)
}

pub fn statementStr(mut stmt: metamodelica::Ref<SCode::Statement>, mut options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString2(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Ref<SCode::Statement>, __a2: SCodeDumpOptions| {
                SCodeDumpTpl::dumpStatement(__a0, &__a1, &__a2)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        Tpl::Text,
                        metamodelica::Ref<SCode::Statement>,
                        SCodeDumpOptions,
                    ) -> Result<Tpl::Text>
                    + 'static,
            >),
        stmt,
        options,
    )?;
    Ok(outString)
}

pub fn equationStr(
    mut inEquation: metamodelica::Ref<SCode::Equation>,
    mut options: SCodeDumpOptions,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString2(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Ref<SCode::Equation>, __a2: SCodeDumpOptions| {
                SCodeDumpTpl::dumpEquation(__a0, &__a1, &__a2)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SCode::Equation>, SCodeDumpOptions) -> Result<Tpl::Text>
                    + 'static,
            >),
        inEquation,
        options,
    )?;
    Ok(outString)
}

pub fn printModStr(mut inMod: metamodelica::Ref<SCode::Mod>, mut options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString2(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Ref<SCode::Mod>, __a2: SCodeDumpOptions| {
                SCodeDumpTpl::dumpModifier(__a0, &__a1, &__a2)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SCode::Mod>, SCodeDumpOptions) -> Result<Tpl::Text>
                    + 'static,
            >),
        inMod,
        options,
    )?;
    Ok(outString)
}

pub(crate) fn printCommentAndAnnotationStr(
    mut inComment: metamodelica::Ref<SCode::Comment>,
    mut options: SCodeDumpOptions,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString2(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Ref<SCode::Comment>, __a2: SCodeDumpOptions| {
                SCodeDumpTpl::dumpComment(__a0, &__a1, &__a2)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SCode::Comment>, SCodeDumpOptions) -> Result<Tpl::Text>
                    + 'static,
            >),
        inComment,
        options,
    )?;
    Ok(outString)
}

pub fn printCommentStr(
    mut inComment: &metamodelica::Ref<SCode::Comment>,
    mut options: SCodeDumpOptions,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inComment {
        SCode::Comment { comment, .. } => Tpl::tplString2(
            (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: Option<ArcStr>, __a2: SCodeDumpOptions| {
                SCodeDumpTpl::dumpCommentStr(__a0, &__a1, &__a2)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Tpl::Text, Option<ArcStr>, SCodeDumpOptions) -> Result<Tpl::Text> + 'static,
                >),
            comment.clone(),
            options,
        )?,
        _ => {
            literal!("")
        }
    });
    Ok(outString)
}

pub fn printAnnotationStr(
    mut inComment: &metamodelica::Ref<SCode::Comment>,
    mut options: SCodeDumpOptions,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inComment {
        SCode::Comment { annotation_, .. } => Tpl::tplString2(
            (std::sync::Arc::new(
                move |__a0: Tpl::Text, __a1: Option<metamodelica::Ref<SCode::Annotation>>, __a2: SCodeDumpOptions| {
                    SCodeDumpTpl::dumpAnnotationOpt(__a0, &__a1, &__a2)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            Tpl::Text,
                            Option<metamodelica::Ref<SCode::Annotation>>,
                            SCodeDumpOptions,
                        ) -> Result<Tpl::Text>
                        + 'static,
                >),
            annotation_.clone(),
            options,
        )?,
        _ => {
            literal!("")
        }
    });
    Ok(outString)
}

pub fn restrString(mut inRestriction: &SCode::Restriction) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inRestriction.clone() {
        SCode::Restriction::R_CLASS { .. } => literal!("class"),
        SCode::Restriction::R_OPTIMIZATION { .. } => literal!("optimization"),
        SCode::Restriction::R_MODEL { .. } => literal!("model"),
        SCode::Restriction::R_RECORD { isOperator: false } => literal!("record"),
        SCode::Restriction::R_RECORD { isOperator: true } => literal!("operator record"),
        SCode::Restriction::R_BLOCK { .. } => literal!("block"),
        SCode::Restriction::R_CONNECTOR { isExpandable: false } => literal!("connector"),
        SCode::Restriction::R_CONNECTOR { isExpandable: true } => literal!("expandable connector"),
        SCode::Restriction::R_OPERATOR { .. } => literal!("operator"),
        SCode::Restriction::R_FUNCTION { .. } => {
            (match var_field!(inRestriction.functionRestriction, SCode::Restriction::R_FUNCTION).clone() {
                SCode::FunctionRestriction::FR_NORMAL_FUNCTION {
                    purity: Absyn::FunctionPurity::PURE { .. },
                } => literal!("pure function"),
                SCode::FunctionRestriction::FR_NORMAL_FUNCTION {
                    purity: Absyn::FunctionPurity::IMPURE { .. },
                } => literal!("impure function"),
                SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. } => literal!("operator function"),
                SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION {
                    purity: Absyn::FunctionPurity::PURE { .. },
                } => literal!("pure external function"),
                SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION {
                    purity: Absyn::FunctionPurity::IMPURE { .. },
                } => literal!("impure external function"),
                SCode::FunctionRestriction::FR_RECORD_CONSTRUCTOR { .. } => literal!("record constructor"),
                SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. } => literal!("parallel function"),
                SCode::FunctionRestriction::FR_KERNEL_FUNCTION { .. } => literal!("kernel function"),
                _ => literal!("function"),
            })
        }
        SCode::Restriction::R_TYPE { .. } => literal!("type"),
        SCode::Restriction::R_PACKAGE { .. } => literal!("package"),
        SCode::Restriction::R_ENUMERATION { .. } => literal!("enumeration"),
        SCode::Restriction::R_METARECORD { .. } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("metarecord "));
            __mm_s.push_str(&*AbsynUtil::pathString(
                var_field!(inRestriction.name, SCode::Restriction::R_METARECORD).clone(),
                literal!("."),
                true,
                false,
            )?);
            ArcStr::from(__mm_s)
        }
        SCode::Restriction::R_UNIONTYPE { .. } => literal!("uniontype"),
        SCode::Restriction::R_PREDEFINED_INTEGER { .. } => literal!("Integer"),
        SCode::Restriction::R_PREDEFINED_REAL { .. } => literal!("Real"),
        SCode::Restriction::R_PREDEFINED_STRING { .. } => literal!("String"),
        SCode::Restriction::R_PREDEFINED_BOOLEAN { .. } => literal!("Boolean"),
        SCode::Restriction::R_PREDEFINED_CLOCK { .. } => literal!("Clock"),
        SCode::Restriction::R_PREDEFINED_ENUMERATION { .. } => literal!("enumeration"),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub fn restrictionStringPP(mut inRestriction: SCode::Restriction) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: SCode::Restriction| {
            SCodeDumpTpl::dumpRestriction(__a0, &__a1)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SCode::Restriction) -> Result<Tpl::Text> + 'static>),
        inRestriction,
    )?;
    Ok(outString)
}

pub(crate) const noEachStr: &'static str = "";

pub fn unparseElementStr(
    mut inElement: metamodelica::Ref<SCode::Element>,
    mut options: SCodeDumpOptions,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = Tpl::tplString3(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Ref<SCode::Element>, __a2: ArcStr, __a3: SCodeDumpOptions| {
                SCodeDumpTpl::dumpElement(__a0, &__a1, &__a2, &__a3)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        Tpl::Text,
                        metamodelica::Ref<SCode::Element>,
                        ArcStr,
                        SCodeDumpOptions,
                    ) -> Result<Tpl::Text>
                    + 'static,
            >),
        inElement,
        arcstr::literal!(noEachStr),
        options,
    )?;
    Ok(outString)
}

pub fn shortElementStr(mut inElement: metamodelica::Ref<SCode::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::EXTENDS { baseClassPath: path, modifications: r#mod, .. } => {
            let mut r#str: ArcStr;
            let mut res: ArcStr;
            r#str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*printModStr(r#mod.clone(), defaultOptions.clone())?); ArcStr::from(__mm_s) };
            res = stringAppendList(list![literal!("extends "), r#str, literal!(";")]);
            res
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            let mut res: ArcStr;
            res = unparseElementStr(inElement, defaultOptions.clone())?;
            res
        },
        Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { .. }, classDef: Deref @ SCode::ClassDef::DERIVED { .. }, .. } => {
            let mut res: ArcStr;
            res = unparseElementStr(inElement, defaultOptions.clone())?;
            res
        },
        Deref @ SCode::Element::CLASS { name: n, partialPrefix: pp, prefixes: Deref @ SCode::Prefixes { innerOuter: io, redeclarePrefix: rdp, replaceablePrefix: rpp, .. }, classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. } => {
            let mut res: ArcStr;
            let mut ioStr: ArcStr;
            ioStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::unparseInnerOuterStr(io.clone())); __mm_s.push_str(&*redeclareStr(rdp.clone())); __mm_s.push_str(&*replaceablePrefixStr(metamodelica::AsArg::as_arg(&rpp))); __mm_s.push_str(&*partialStr(pp.clone())); ArcStr::from(__mm_s) };
            res = stringAppendList(list![ioStr, literal!("class extends "), n.clone(), literal!(";")]);
            res
        },
        Deref @ SCode::Element::CLASS { name: n, partialPrefix: pp, prefixes: Deref @ SCode::Prefixes { innerOuter: io, redeclarePrefix: rdp, replaceablePrefix: rpp, .. }, classDef: Deref @ SCode::ClassDef::ENUMERATION { .. }, .. } => {
            let mut res: ArcStr;
            let mut ioStr: ArcStr;
            ioStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::unparseInnerOuterStr(io.clone())); __mm_s.push_str(&*redeclareStr(rdp.clone())); __mm_s.push_str(&*replaceablePrefixStr(metamodelica::AsArg::as_arg(&rpp))); __mm_s.push_str(&*partialStr(pp.clone())); ArcStr::from(__mm_s) };
            res = stringAppendList(list![ioStr, literal!("class "), n.clone(), literal!(" enumeration;")]);
            res
        },
        Deref @ SCode::Element::CLASS { name: n, partialPrefix: pp, prefixes: Deref @ SCode::Prefixes { innerOuter: io, redeclarePrefix: rdp, replaceablePrefix: rpp, .. }, .. } => {
            let mut res: ArcStr;
            let mut ioStr: ArcStr;
            ioStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::unparseInnerOuterStr(io.clone())); __mm_s.push_str(&*redeclareStr(rdp.clone())); __mm_s.push_str(&*replaceablePrefixStr(metamodelica::AsArg::as_arg(&rpp))); __mm_s.push_str(&*partialStr(pp.clone())); ArcStr::from(__mm_s) };
            res = stringAppendList(list![ioStr, literal!("class "), n.clone(), literal!(";")]);
            res
        },
        Deref @ SCode::Element::IMPORT { imp, .. } => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("import ")); __mm_s.push_str(&*AbsynUtil::printImportString(metamodelica::AsArg::as_arg(&imp))?); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) };
            r#str
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

pub(crate) fn printEnumStr(mut en: &metamodelica::Ref<SCode::Enum>) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match &**en {
        SCode::Enum { literal: s, comment: _ } => s.clone(),
    });
    r#str
}

pub fn variabilityString(mut inVariability: SCode::Variability) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inVariability {
        SCode::Variability::VAR { .. } => literal!("VAR"),
        SCode::Variability::DISCRETE { .. } => literal!("DISCRETE"),
        SCode::Variability::PARAM { .. } => literal!("PARAM"),
        SCode::Variability::CONST { .. } => literal!("CONST"),
    });
    outString
}

pub(crate) fn parallelismString(mut inParallelism: SCode::Parallelism) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inParallelism {
        SCode::Parallelism::PARGLOBAL { .. } => literal!("PARGLOBAL"),
        SCode::Parallelism::PARLOCAL { .. } => literal!("PARLOCAL"),
        SCode::Parallelism::NON_PARALLEL { .. } => literal!("NON_PARALLEL"),
    });
    outString
}

pub(crate) fn innerouterString(mut innerOuter: Absyn::InnerOuter) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match innerOuter {
        Absyn::InnerOuter::INNER_OUTER { .. } => literal!("INNER/OUTER"),
        Absyn::InnerOuter::INNER { .. } => literal!("INNER"),
        Absyn::InnerOuter::OUTER { .. } => literal!("OUTER"),
        Absyn::InnerOuter::NOT_INNER_OUTER { .. } => literal!(""),
    });
    outString
}

pub fn unparseVariability(mut inVariability: SCode::Variability) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inVariability {
        SCode::Variability::VAR { .. } => literal!(""),
        SCode::Variability::DISCRETE { .. } => literal!("discrete"),
        SCode::Variability::PARAM { .. } => literal!("parameter"),
        SCode::Variability::CONST { .. } => literal!("constant"),
    });
    outString
}

pub fn printInitialStr(mut initial_: SCode::Initial) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match initial_ {
        SCode::Initial::INITIAL { .. } => literal!("initial"),
        SCode::Initial::NON_INITIAL { .. } => literal!("non initial"),
    });
    r#str
}

pub fn connectorTypeStr(mut inConnectorType: SCode::ConnectorType) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match inConnectorType {
        SCode::ConnectorType::POTENTIAL { .. } => literal!(""),
        SCode::ConnectorType::FLOW { .. } => literal!("flow"),
        SCode::ConnectorType::STREAM { .. } => literal!("stream"),
    });
    r#str
}

pub(crate) fn encapsulatedStr(mut inEncapsulated: SCode::Encapsulated) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match inEncapsulated {
        SCode::Encapsulated::ENCAPSULATED { .. } => literal!("encapsulated "),
        SCode::Encapsulated::NOT_ENCAPSULATED { .. } => literal!(""),
    });
    r#str
}

pub(crate) fn partialStr(mut inPartial: SCode::Partial) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match inPartial {
        SCode::Partial::PARTIAL { .. } => literal!("partial "),
        SCode::Partial::NOT_PARTIAL { .. } => literal!(""),
    });
    r#str
}

pub fn visibilityStr(mut inVisibility: SCode::Visibility) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match inVisibility {
        SCode::Visibility::PUBLIC { .. } => literal!("public "),
        SCode::Visibility::PROTECTED { .. } => literal!("protected "),
    });
    r#str
}

pub fn finalStr(mut inFinal: SCode::Final) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match inFinal {
        SCode::Final::FINAL { .. } => literal!("final "),
        SCode::Final::NOT_FINAL { .. } => literal!(""),
    });
    r#str
}

pub fn eachStr(mut inEach: SCode::Each) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match inEach {
        SCode::Each::EACH { .. } => literal!("each "),
        SCode::Each::NOT_EACH { .. } => literal!(""),
    });
    r#str
}

pub(crate) fn redeclareStr(mut inRedeclare: SCode::Redeclare) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match inRedeclare {
        SCode::Redeclare::REDECLARE { .. } => literal!("redeclare "),
        SCode::Redeclare::NOT_REDECLARE { .. } => literal!(""),
    });
    r#str
}

pub(crate) fn replaceableStr(mut inReplaceable: &metamodelica::Ref<SCode::Replaceable>) -> Result<(ArcStr, ArcStr)> {
    let mut strReplaceable: ArcStr;
    let mut strConstraint: ArcStr;
    (strReplaceable, strConstraint) = (::match_deref::match_deref! { match inReplaceable {
        Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { constrainingClass: path, modifier: r#mod, .. }) } => {
            let mut path_str: ArcStr;
            let mut mod_str: ArcStr;
            path_str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            mod_str = printModStr(r#mod.clone(), defaultOptions.clone())?;
            (literal!("replaceable "), { let mut __mm_s = String::new(); __mm_s.push_str(&*path_str); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*mod_str); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
        },
        Deref @ SCode::Replaceable::REPLACEABLE { cc: None } => {
            (literal!("replaceable "), literal!(""))
        },
        Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. } => {
            (literal!(""), literal!(""))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((strReplaceable, strConstraint))
}

pub(crate) fn replaceablePrefixStr(mut inReplaceable: &metamodelica::Ref<SCode::Replaceable>) -> ArcStr {
    let mut strReplaceable: ArcStr;
    strReplaceable = (match &**inReplaceable {
        SCode::Replaceable::REPLACEABLE { cc: _ } => literal!("replaceable "),
        SCode::Replaceable::NOT_REPLACEABLE { .. } => literal!(""),
    });
    strReplaceable
}

pub(crate) fn replaceableConstrainClassStr(
    mut inReplaceable: &metamodelica::Ref<SCode::Replaceable>,
) -> Result<ArcStr> {
    let mut strReplaceable: ArcStr;
    (_, strReplaceable) = replaceableStr(inReplaceable)?;
    Ok(strReplaceable)
}

pub(crate) fn prefixesStr(mut prefixes: &metamodelica::Ref<SCode::Prefixes>) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match &**prefixes {
        SCode::Prefixes {
            visibility: v,
            redeclarePrefix: rd,
            finalPrefix: f,
            innerOuter: io,
            replaceablePrefix: rpl,
        } => {
            let mut s: ArcStr;
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*visibilityStr(v.clone()));
                __mm_s.push_str(&*redeclareStr(rd.clone()));
                __mm_s.push_str(&*finalStr(f.clone()));
                __mm_s.push_str(&*Dump::unparseInnerOuterStr(io.clone()));
                __mm_s.push_str(&*replaceablePrefixStr(rpl));
                ArcStr::from(__mm_s)
            };
            s
        }
    });
    r#str
}

pub fn filterElements(
    mut elements: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut options: SCodeDumpOptions,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    outElements = List::select1(
        elements,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<SCode::Element>, __a1: SCodeDumpOptions| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(filterElement(&__a0, __a1))
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>, SCodeDumpOptions) -> Result<bool> + 'static,
            >),
        options,
    )?;
    Ok(outElements)
}

fn filterElement(mut element: &metamodelica::Ref<SCode::Element>, mut options: SCodeDumpOptions) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((&**element, options)) {
        (Deref @ SCode::Element::IMPORT { visibility: SCode::Visibility::PROTECTED { .. }, .. }, SCodeDumpOptions { stripProtectedImports: true, .. }) => false,
        (Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { visibility: SCode::Visibility::PROTECTED { .. }, .. }, .. }, SCodeDumpOptions { stripProtectedClasses: true, .. }) => false,
        (Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { visibility: SCode::Visibility::PROTECTED { .. }, .. }, .. }, SCodeDumpOptions { stripProtectedComponents: true, .. }) => false,
        (Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_METARECORD { moved: true, .. }, .. }, SCodeDumpOptions { stripMetaRecords: true, .. }) => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}
