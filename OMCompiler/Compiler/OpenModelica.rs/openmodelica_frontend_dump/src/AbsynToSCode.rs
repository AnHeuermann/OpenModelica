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
use crate::BackendInterface;
use crate::Dump;
use crate::MetaUtil;
use crate::SCodeUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// Constant expression for AssertionLevel.error.
pub(crate) static ASSERTION_LEVEL_ERROR: std::sync::LazyLock<metamodelica::Ref<Absyn::Exp>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::Exp::CREF {
            componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED {
                componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                    name: literal!("AssertionLevel"),
                    subscripts: metamodelica::nil(),
                    componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                        name: literal!("error"),
                        subscripts: metamodelica::nil(),
                    }),
                }),
            }),
        })
    });

pub fn translateAbsyn2SCode(
    mut inProgram: Absyn::Program,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    outProgram = (match inProgram.clone() {
        _ => {
            let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut inClasses: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
            BackendInterface::initInstHashTable()?;
            let Absyn::PROGRAM { classes: __pa0, .. } = MetaUtil::createMetaClassesInProgram(inProgram)?;
            inClasses = metamodelica::Own::own(__pa0);
            System::setHasInnerOuterDefinitions(false);
            System::setHasExpandableConnectors(false);
            System::setHasOverconstrainedConnectors(false);
            System::setHasStreamConnectors(false);
            sp = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                for mut c in (inClasses).into_iter().cloned() {
                    let __x = translateClass(c.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            sp
        }
    });
    Ok(outProgram)
}

pub fn translateClass(mut inClass: metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outClass: metamodelica::Ref<SCode::Element>;
    outClass = translateClass2(inClass, Error::getNumMessages())?;
    Ok(outClass)
}

fn translateClass2(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inNumMessages: i32,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outClass: metamodelica::Ref<SCode::Element>;
    outClass = 'mc: {
        let __mc_input = inClass;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                c @ Deref @ Absyn::Class { name: n, partialPrefix: p, finalPrefix: f, encapsulatedPrefix: e, restriction: r, body: d, info: file_info, .. } => {
                    let mut d_1: metamodelica::Ref<SCode::ClassDef>;
                    let mut r_1: SCode::Restriction;
                    let mut scodeClass: metamodelica::Ref<SCode::Element>;
                    let mut sFin: SCode::Final;
                    let mut sEnc: SCode::Encapsulated;
                    let mut sPar: SCode::Partial;
                    let mut cmt: metamodelica::Ref<SCode::Comment>;
                    r_1 = translateRestriction(c.clone(), metamodelica::AsArg::as_arg(&r))?;
                    (d_1, cmt) = translateClassdef(metamodelica::AsArg::as_arg(&d), file_info.clone(), &r_1)?;
                    sFin = SCodeUtil::boolFinal(f.clone());
                    sEnc = SCodeUtil::boolEncapsulated(e.clone());
                    sPar = SCodeUtil::boolPartial(p.clone());
                    scodeClass = metamodelica::Ref::new(SCode::Element::CLASS { name: n.clone(), prefixes: metamodelica::Ref::new(SCode::Prefixes { visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, redeclarePrefix: openmodelica_frontend_types::SCode::Redeclare::NOT_REDECLARE, finalPrefix: sFin, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE() }), encapsulatedPrefix: sEnc, partialPrefix: sPar, restriction: r_1.clone(), classDef: d_1.clone(), cmt: cmt.clone(), info: file_info.clone() });
                    Ok(scodeClass.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { name: n, info: file_info, .. } => {
                    let mut n = (*n).clone();
                    let true = (intEq(Error::getNumMessages(), inNumMessages)) else { return Err("pattern mismatch") };
                    n = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AbsynToSCode.translateClass2 failed: ")); __mm_s.push_str(&*n); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![n.clone()], metamodelica::AsArg::as_arg(&file_info))?;
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

//mahge: FIX HERE. Check for proper input and output
//declarations in operators according to the specifications.
pub(crate) fn translateOperatorDef(
    mut inClassDef: &metamodelica::Ref<Absyn::ClassDef>,
    mut operatorName: &ArcStr,
    mut info: &SourceInfo,
) -> Result<(metamodelica::Ref<SCode::ClassDef>, metamodelica::Ref<SCode::Comment>)> {
    let mut outOperDef: metamodelica::Ref<SCode::ClassDef>;
    let mut cmt: metamodelica::Ref<SCode::Comment>;
    (outOperDef, cmt) = (match &**inClassDef {
        Absyn::ClassDef::PARTS {
            classParts: parts,
            ann: aann,
            comment: cmtString,
            ..
        } => {
            let mut els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            els = translateClassdefElements(parts)?;
            cmt = translateCommentList(aann, cmtString.clone())?;
            (
                metamodelica::Ref::new(SCode::ClassDef::PARTS {
                    elementLst: els,
                    normalEquationLst: metamodelica::nil(),
                    initialEquationLst: metamodelica::nil(),
                    normalAlgorithmLst: metamodelica::nil(),
                    initialAlgorithmLst: metamodelica::nil(),
                    constraintLst: metamodelica::nil(),
                    clsattrs: metamodelica::nil(),
                    externalDecl: None,
                }),
                cmt,
            )
        }
        _ => {
            Error::addSourceMessage(
                &(Error::INTERNAL_ERROR.clone()),
                list![literal!(
                    "Could not translate operator to SCode because it is not using class parts."
                )],
                info,
            )?;
            return Err("fail");
        }
    });
    Ok((outOperDef, cmt))
}

pub(crate) fn getOperatorGivenName(
    mut inOperatorFunction: &metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outName: metamodelica::Ref<Absyn::Path>;
    outName = (match &**inOperatorFunction {
        SCode::Element::CLASS {
            name,
            prefixes: _,
            encapsulatedPrefix: _,
            partialPrefix: _,
            restriction:
                SCode::Restriction::R_FUNCTION {
                    functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. },
                },
            classDef: _,
            cmt: _,
            info: _,
        } => metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }),
        _ => return Err("match: no arm matched"),
    });
    Ok(outName)
}

pub(crate) fn getOperatorQualName(
    mut inOperatorFunction: &metamodelica::Ref<SCode::Element>,
    mut operName: ArcStr,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outName: metamodelica::Ref<Absyn::Path>;
    outName = (match &**inOperatorFunction {
        SCode::Element::CLASS {
            name,
            prefixes: _,
            encapsulatedPrefix: _,
            partialPrefix: _,
            restriction: SCode::Restriction::R_FUNCTION { functionRestriction: _ },
            classDef: _,
            cmt: _,
            info: _,
        } => {
            let mut opname = operName;
            AbsynUtil::joinPaths(
                metamodelica::Ref::new(Absyn::Path::IDENT { name: opname }),
                metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }),
            )?
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outName)
}

pub fn getListofQualOperatorFuncsfromOperator(
    mut inOperator: &metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut outNames: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    outNames = (::match_deref::match_deref! { match inOperator {
        Deref @ SCode::Element::CLASS { name: opername, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_OPERATOR { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: els, .. }, cmt: _, info: _ } => {
            let mut names: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            names = List::map1(els.clone(), &move |__a0: metamodelica::Ref<SCode::Element>, __a1: ArcStr| getOperatorQualName(&__a0, __a1), opername.clone())?;
            names
        },
        Deref @ SCode::Element::CLASS { name: opername, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. } }, classDef: _, cmt: _, info: _ } => {
            let mut names: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            names = list![metamodelica::Ref::new(Absyn::Path::IDENT { name: opername.clone() })];
            names
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outNames)
}

pub(crate) fn translateRestriction(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inRestriction: &Absyn::Restriction,
) -> Result<SCode::Restriction> {
    let mut outRestriction: SCode::Restriction;
    outRestriction = (::match_deref::match_deref! { match &((inClass, inRestriction)) {
        (d, Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { purity } }) => {
            if (containsExternalFuncDecl(metamodelica::AsArg::as_arg(&d))?) {SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: purity.clone() } }} else {SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: purity.clone() } }}
        },
        (_, Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_OPERATOR_FUNCTION { .. } }) => {
            SCode::Restriction::R_FUNCTION { functionRestriction: openmodelica_frontend_types::SCode::FunctionRestriction::FR_OPERATOR_FUNCTION }
        },
        (_, Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_PARALLEL_FUNCTION { .. } }) => {
            SCode::Restriction::R_FUNCTION { functionRestriction: openmodelica_frontend_types::SCode::FunctionRestriction::FR_PARALLEL_FUNCTION }
        },
        (_, Absyn::Restriction::R_FUNCTION { functionRestriction: Absyn::FunctionRestriction::FR_KERNEL_FUNCTION { .. } }) => {
            SCode::Restriction::R_FUNCTION { functionRestriction: openmodelica_frontend_types::SCode::FunctionRestriction::FR_KERNEL_FUNCTION }
        },
        (_, Absyn::Restriction::R_CLASS { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_CLASS
        },
        (_, Absyn::Restriction::R_OPTIMIZATION { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_OPTIMIZATION
        },
        (_, Absyn::Restriction::R_MODEL { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_MODEL
        },
        (_, Absyn::Restriction::R_RECORD { .. }) => {
            SCode::Restriction::R_RECORD { isOperator: false }
        },
        (_, Absyn::Restriction::R_OPERATOR_RECORD { .. }) => {
            SCode::Restriction::R_RECORD { isOperator: true }
        },
        (_, Absyn::Restriction::R_BLOCK { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_BLOCK
        },
        (_, Absyn::Restriction::R_CONNECTOR { .. }) => {
            SCode::Restriction::R_CONNECTOR { isExpandable: false }
        },
        (_, Absyn::Restriction::R_EXP_CONNECTOR { .. }) => {
            System::setHasExpandableConnectors(true);
            SCode::Restriction::R_CONNECTOR { isExpandable: true }
        },
        (_, Absyn::Restriction::R_OPERATOR { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_OPERATOR
        },
        (_, Absyn::Restriction::R_TYPE { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_TYPE
        },
        (_, Absyn::Restriction::R_PACKAGE { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_PACKAGE
        },
        (_, Absyn::Restriction::R_ENUMERATION { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_ENUMERATION
        },
        (_, Absyn::Restriction::R_PREDEFINED_INTEGER { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_INTEGER
        },
        (_, Absyn::Restriction::R_PREDEFINED_REAL { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_REAL
        },
        (_, Absyn::Restriction::R_PREDEFINED_STRING { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_STRING
        },
        (_, Absyn::Restriction::R_PREDEFINED_BOOLEAN { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_BOOLEAN
        },
        (_, Absyn::Restriction::R_PREDEFINED_CLOCK { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_CLOCK
        },
        (_, Absyn::Restriction::R_PREDEFINED_ENUMERATION { .. }) => {
            openmodelica_frontend_types::SCode::Restriction::R_PREDEFINED_ENUMERATION
        },
        (_, Absyn::Restriction::R_METARECORD { name, index, singleton, moved, typeVars }) => {
            SCode::Restriction::R_METARECORD { name: name.clone(), index: index.clone(), singleton: singleton.clone(), moved: moved.clone(), typeVars: typeVars.clone() }
        },
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, .. }, .. }, Absyn::Restriction::R_UNIONTYPE { .. }) => {
            SCode::Restriction::R_UNIONTYPE { typeVars: typeVars.clone() }
        },
        (_, Absyn::Restriction::R_UNIONTYPE { .. }) => {
            SCode::Restriction::R_UNIONTYPE { typeVars: metamodelica::nil() }
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outRestriction)
}

fn containsExternalFuncDecl(mut inClass: &metamodelica::Ref<Absyn::Class>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            List::any(metamodelica::AsArg::as_arg(&parts), &move |__a0: metamodelica::Ref<Absyn::ClassPart>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::isExternalPart(&__a0)) })?
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            List::any(metamodelica::AsArg::as_arg(&parts), &move |__a0: metamodelica::Ref<Absyn::ClassPart>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::isExternalPart(&__a0)) })?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

fn translateAttributes(
    mut inEA: &Absyn::ElementAttributes,
    mut extraArrayDim: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> Result<SCode::Attributes> {
    let mut outA: SCode::Attributes;
    outA = (match inEA.clone() {
        Absyn::ElementAttributes {
            flowPrefix: mut f,
            streamPrefix: mut s,
            parallelism: mut p,
            variability: mut v,
            direction: mut dir,
            isField: mut fi,
            arrayDim: ref adim,
        } => {
            let mut extraADim = extraArrayDim;
            let mut ct: SCode::ConnectorType;
            let mut sp: SCode::Parallelism;
            let mut sv: SCode::Variability;
            let mut adim = adim.clone();
            ct = translateConnectorType(f.clone(), s.clone())?;
            sv = translateVariability(v.clone());
            sp = translateParallelism(p.clone());
            adim = listAppend(extraADim, adim.clone());
            SCode::Attributes {
                arrayDims: adim.clone(),
                connectorType: ct,
                parallelism: sp,
                variability: sv,
                direction: dir.clone(),
                isField: fi.clone(),
            }
        }
    });
    Ok(outA)
}

fn translateConnectorType(mut inFlow: bool, mut inStream: bool) -> Result<SCode::ConnectorType> {
    let mut outType: SCode::ConnectorType;
    outType = (match (inFlow, inStream) {
        (false, false) => openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
        (true, false) => openmodelica_frontend_types::SCode::ConnectorType::FLOW,
        (false, true) => openmodelica_frontend_types::SCode::ConnectorType::STREAM,
        (true, true) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!(
                    "AbsynToSCode.translateConnectorType got both flow and stream prefix."
                )],
            )?;
            return Err("fail");
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outType)
}

fn translateClassdef(
    mut inClassDef: &metamodelica::Ref<Absyn::ClassDef>,
    mut info: SourceInfo,
    mut re: &SCode::Restriction,
) -> Result<(metamodelica::Ref<SCode::ClassDef>, metamodelica::Ref<SCode::Comment>)> {
    let mut outClassDef: metamodelica::Ref<SCode::ClassDef>;
    let mut outComment: metamodelica::Ref<SCode::Comment>;
    (outClassDef, outComment) = (::match_deref::match_deref! { match inClassDef {
        Deref @ Absyn::ClassDef::DERIVED { typeSpec: t, attributes: attr, arguments: a, comment: cmt } => {
            let mut r#mod: metamodelica::Ref<SCode::Mod>;
            let mut scodeCmt: metamodelica::Ref<SCode::Comment>;
            let mut scodeAttr: SCode::Attributes;
            checkTypeSpec(t, &info)?;
            r#mod = translateMod(Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: a.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, info, false)?;
            scodeAttr = translateAttributes(attr, metamodelica::nil())?;
            scodeCmt = translateComment(cmt.clone())?;
            (metamodelica::Ref::new(SCode::ClassDef::DERIVED { typeSpec: t.clone(), modifications: r#mod, attributes: scodeAttr }), scodeCmt)
        },
        Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmtString } => {
            let mut els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut tvels: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut eqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
            let mut initeqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
            let mut als: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
            let mut initals: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
            let mut cos: metamodelica::List<SCode::ConstraintSection>;
            let mut decl: Option<metamodelica::Ref<SCode::ExternalDecl>>;
            let mut scodeCmt: metamodelica::Ref<SCode::Comment>;
            let mut typeVars = (*typeVars).clone();
            typeVars = (match re.clone() {
        SCode::Restriction::R_METARECORD { .. } => List::union(metamodelica::AsArg::as_arg(&typeVars), var_field!(re.typeVars, SCode::Restriction::R_METARECORD)),
        SCode::Restriction::R_UNIONTYPE { .. } => List::union(metamodelica::AsArg::as_arg(&typeVars), var_field!(re.typeVars, SCode::Restriction::R_UNIONTYPE)),
        _ => typeVars.clone(),
    });
            tvels = List::map1(typeVars.clone(), &fnptr!(makeTypeVarElement, ArcStr, SourceInfo), info.clone())?;
            els = translateClassdefElements(parts)?;
            els = listAppend(tvels, els);
            eqs = translateClassdefEquations(parts)?;
            initeqs = translateClassdefInitialequations(parts)?;
            als = translateClassdefAlgorithms(parts)?;
            initals = translateClassdefInitialalgorithms(parts)?;
            cos = translateClassdefConstraints(parts)?;
            scodeCmt = translateCommentList(ann, cmtString.clone())?;
            decl = translateClassdefExternaldecls(parts)?;
            decl = translateAlternativeExternalAnnotation(decl, &scodeCmt, &info)?;
            (metamodelica::Ref::new(SCode::ClassDef::PARTS { elementLst: els, normalEquationLst: eqs, initialEquationLst: initeqs, normalAlgorithmLst: als, initialAlgorithmLst: initals, constraintLst: cos, clsattrs: classAttrs.clone(), externalDecl: decl }), scodeCmt)
        },
        Deref @ Absyn::ClassDef::ENUMERATION { enumLiterals: Deref @ Absyn::EnumDef::ENUMLITERALS { enumLiterals: lst }, comment: cmt } => {
            let mut lst_1: metamodelica::List<metamodelica::Ref<SCode::Enum>>;
            let mut scodeCmt: metamodelica::Ref<SCode::Comment>;
            lst_1 = translateEnumlist(metamodelica::AsArg::as_arg(&lst))?;
            scodeCmt = translateComment(cmt.clone())?;
            (metamodelica::Ref::new(SCode::ClassDef::ENUMERATION { enumLst: lst_1 }), scodeCmt)
        },
        Deref @ Absyn::ClassDef::ENUMERATION { enumLiterals: Deref @ Absyn::EnumDef::ENUM_COLON { .. }, comment: cmt } => {
            let mut scodeCmt: metamodelica::Ref<SCode::Comment>;
            scodeCmt = translateComment(cmt.clone())?;
            (metamodelica::Ref::new(SCode::ClassDef::ENUMERATION { enumLst: metamodelica::nil() }), scodeCmt)
        },
        Deref @ Absyn::ClassDef::OVERLOAD { functionNames: pathLst, comment: cmt } => {
            let mut scodeCmt: metamodelica::Ref<SCode::Comment>;
            scodeCmt = translateComment(cmt.clone())?;
            (metamodelica::Ref::new(SCode::ClassDef::OVERLOAD { pathLst: pathLst.clone() }), scodeCmt)
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { modifications: cmod, ann, comment: cmtString, parts, .. } => {
            let mut r#mod: metamodelica::Ref<SCode::Mod>;
            let mut els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut eqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
            let mut initeqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
            let mut als: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
            let mut initals: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
            let mut cos: metamodelica::List<SCode::ConstraintSection>;
            let mut decl: Option<metamodelica::Ref<SCode::ExternalDecl>>;
            let mut scodeCmt: metamodelica::Ref<SCode::Comment>;
            els = translateClassdefElements(parts)?;
            eqs = translateClassdefEquations(parts)?;
            initeqs = translateClassdefInitialequations(parts)?;
            als = translateClassdefAlgorithms(parts)?;
            initals = translateClassdefInitialalgorithms(parts)?;
            cos = translateClassdefConstraints(parts)?;
            r#mod = translateMod(Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: cmod.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, Absyn::dummyInfo.clone(), false)?;
            scodeCmt = translateCommentList(ann, cmtString.clone())?;
            decl = translateClassdefExternaldecls(parts)?;
            decl = translateAlternativeExternalAnnotation(decl, &scodeCmt, &info)?;
            (metamodelica::Ref::new(SCode::ClassDef::CLASS_EXTENDS { modifications: r#mod, composition: metamodelica::Ref::new(SCode::ClassDef::PARTS { elementLst: els, normalEquationLst: eqs, initialEquationLst: initeqs, normalAlgorithmLst: als, initialAlgorithmLst: initals, constraintLst: cos, clsattrs: metamodelica::nil(), externalDecl: decl }) }), scodeCmt)
        },
        Deref @ Absyn::ClassDef::PDER { functionName: path, vars, comment: cmt } => {
            let mut scodeCmt: metamodelica::Ref<SCode::Comment>;
            scodeCmt = translateComment(cmt.clone())?;
            (metamodelica::Ref::new(SCode::ClassDef::PDER { functionPath: path.clone(), derivedVariables: vars.clone() }), scodeCmt)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("AbsynToSCode.translateClassdef failed")])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClassDef, outComment))
}

fn translateAlternativeExternalAnnotation(
    mut decl: Option<metamodelica::Ref<SCode::ExternalDecl>>,
    mut comment: &metamodelica::Ref<SCode::Comment>,
    mut info: &SourceInfo,
) -> Result<Option<metamodelica::Ref<SCode::ExternalDecl>>> {
    fn whitelist_mod(mut submod: &metamodelica::Ref<SCode::SubMod>) -> bool {
        let mut keep: bool;
        keep = (::match_deref::match_deref! { match &(submod.ident.clone()) {
            Deref @ "Library" => true,
            Deref @ "Include" => true,
            Deref @ "LibraryDirectory" => true,
            Deref @ "SourceDirectory" => true,
            Deref @ "License" => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        keep
    }

    let mut outDecl: Option<metamodelica::Ref<SCode::ExternalDecl>>;
    let mut ext_decl: metamodelica::Ref<SCode::ExternalDecl>;
    let mut ann: metamodelica::Ref<SCode::Annotation>;
    outDecl = (::match_deref::match_deref! { match &((decl.clone(), comment.clone())) {
        (Some(__esc_ext_decl @ Deref @ SCode::ExternalDecl { annotation_: None, .. }), Deref @ SCode::Comment { annotation_: Some(__esc_ann), .. }) => {
            ext_decl = (*__esc_ext_decl).clone();
            ann = (*__esc_ann).clone();
            assign_field!(ann.modification = SCodeUtil::filterSubMods(ann.modification.clone(), &move |__a0: metamodelica::Ref<SCode::SubMod>| -> metamodelica::Result<_> { ::std::result::Result::Ok(whitelist_mod(&__a0)) })?);
            if !(SCodeUtil::isEmptyMod(&ann.modification)) {
                if Config::languageStandardAtLeast(Config::LanguageStandard::_3_3.clone())? {
                    Error::addSourceMessage(&(Error::MISPLACED_EXTERNAL_ANNOTATION.clone()), metamodelica::nil(), info)?;
                }
                assign_field!(ext_decl.annotation_ = Some(ann.clone()));
            }
            Some(ext_decl.clone())
        },
        _ => decl,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDecl)
}

fn translateEnumlist(
    mut inAbsynEnumLiteralLst: &metamodelica::List<metamodelica::Ref<Absyn::EnumLiteral>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Enum>>> {
    let mut outEnumLst: metamodelica::List<metamodelica::Ref<SCode::Enum>>;
    outEnumLst = (::match_deref::match_deref! { match inAbsynEnumLiteralLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::EnumLiteral { literal: id, comment: cmtOpt }, tail: rest } => {
            let mut res: metamodelica::List<metamodelica::Ref<SCode::Enum>>;
            let mut cmt: metamodelica::Ref<SCode::Comment>;
            cmt = translateComment(cmtOpt.clone())?;
            res = translateEnumlist(rest)?;
            metamodelica::cons(metamodelica::Ref::new(SCode::Enum { literal: id.clone(), comment: cmt }), res)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEnumLst)
}

pub fn translateClassdefElements<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(metamodelica::nil())
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: es }, tail: rest } => {
                let mut els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                let mut es_1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                es_1 = translateEitemlist(es.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC)?;
                els = translateClassdefElements(rest)?;
                return Ok(listAppend(es_1, els))
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: es }, tail: rest } => {
                let mut els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                let mut es_1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                es_1 = translateEitemlist(es.clone(), openmodelica_frontend_types::SCode::Visibility::PROTECTED)?;
                els = translateClassdefElements(rest)?;
                return Ok(listAppend(es_1, els))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { inAbsynClassPartLst = rest; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn translateClassdefEquations<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Equation>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(metamodelica::nil())
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: eql }, tail: rest } => {
                let mut eqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                let mut eql_1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                let mut eqs_1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                eql_1 = translateEquations(eql.clone(), false)?;
                eqs = translateClassdefEquations(rest)?;
                return Ok(listAppend(eqs, eql_1))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut eqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                { inAbsynClassPartLst = rest; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn translateClassdefInitialequations<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Equation>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(metamodelica::nil())
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eql }, tail: rest } => {
                let mut eqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                let mut eql_1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                let mut eqs_1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                eql_1 = translateEquations(eql.clone(), true)?;
                eqs = translateClassdefInitialequations(rest)?;
                return Ok(listAppend(eqs, eql_1))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut eqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                { inAbsynClassPartLst = rest; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn translateClassdefAlgorithms<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(metamodelica::nil())
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::ALGORITHMS { contents: al }, tail: rest } => {
                let mut als: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                let mut als_1: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                let mut al_1: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
                al_1 = translateClassdefAlgorithmitems(al.clone())?;
                als = translateClassdefAlgorithms(rest)?;
                return Ok(metamodelica::cons(metamodelica::Ref::new(SCode::AlgorithmSection { statements: al_1 }), als))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut als: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                { inAbsynClassPartLst = rest; continue '__tco; }
            },
            _ => {
                let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                Debug::trace(literal!("- AbsynToSCode.translateClassdefAlgorithms failed\n"))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn translateClassdefConstraints<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<SCode::ConstraintSection>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(metamodelica::nil())
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::CONSTRAINTS { contents: consts }, tail: rest } => {
                let mut cos: metamodelica::List<SCode::ConstraintSection>;
                let mut cos_1: metamodelica::List<SCode::ConstraintSection>;
                cos = translateClassdefConstraints(rest)?;
                return Ok(metamodelica::cons(SCode::ConstraintSection { constraints: consts.clone() }, cos))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut cos: metamodelica::List<SCode::ConstraintSection>;
                { inAbsynClassPartLst = rest; continue '__tco; }
            },
            _ => {
                let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                Debug::trace(literal!("- AbsynToSCode.translateClassdefConstraints failed\n"))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn translateClassdefInitialalgorithms<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(metamodelica::nil())
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: al }, tail: rest } => {
                let mut als: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                let mut als_1: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                let mut stmts: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
                stmts = translateClassdefAlgorithmitems(al.clone())?;
                als = translateClassdefInitialalgorithms(rest)?;
                return Ok(metamodelica::cons(metamodelica::Ref::new(SCode::AlgorithmSection { statements: stmts }), als))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut als: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                { inAbsynClassPartLst = rest; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn translateClassdefAlgorithmitems(
    mut inStatements: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Statement>>> {
    let mut outStatements: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    outStatements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Statement>> = metamodelica::nil();
        for mut stmt in (inStatements).into_iter().cloned() {
            if !(AbsynUtil::isAlgorithmItem(&(stmt.clone()))) {
                continue;
            }
            let __x = translateClassdefAlgorithmItem(&(stmt.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outStatements)
}

fn translateClassdefAlgorithmItem(
    mut inAlgorithm: &metamodelica::Ref<Absyn::AlgorithmItem>,
) -> Result<metamodelica::Ref<SCode::Statement>> {
    let mut outStatement: metamodelica::Ref<SCode::Statement>;
    let mut absynComment: Option<metamodelica::Ref<Absyn::Comment>>;
    let mut comment: metamodelica::Ref<SCode::Comment>;
    let mut info: SourceInfo;
    let mut alg: metamodelica::Ref<Absyn::Algorithm>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*inAlgorithm)) {
        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: __pa0, comment: __pa1, info: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    alg = metamodelica::Own::own(__pa0);
    absynComment = metamodelica::Own::own(__pa1);
    info = metamodelica::Own::own(__pa2);
    (comment, info) = translateCommentWithLineInfoChanges(absynComment, info)?;
    outStatement = (::match_deref::match_deref! { match &(alg) {
        Deref @ Absyn::Algorithm::ALG_ASSIGN { assignComponent: __alg_assignComponent, value: __alg_value } => {
            metamodelica::Ref::new(SCode::Statement::ALG_ASSIGN { assignComponent: __alg_assignComponent.clone(), value: __alg_value.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_IF { elseBranch: __alg_elseBranch, elseIfAlgorithmBranch: __alg_elseIfAlgorithmBranch, ifExp: __alg_ifExp, trueBranch: __alg_trueBranch } => {
            let mut body: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
            let mut else_body: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
            let mut branches: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Statement>>)>;
            body = translateClassdefAlgorithmitems(__alg_trueBranch.clone())?;
            else_body = translateClassdefAlgorithmitems(__alg_elseBranch.clone())?;
            branches = translateAlgBranches(__alg_elseIfAlgorithmBranch.clone())?;
            metamodelica::Ref::new(SCode::Statement::ALG_IF { boolExpr: __alg_ifExp.clone(), trueBranch: body, elseIfBranch: branches, elseBranch: else_body, comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_FOR { forBody: __alg_forBody, iterators: __alg_iterators } => {
            let mut body: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<metamodelica::Ref<Absyn::Exp>>;
            body = translateClassdefAlgorithmitems(__alg_forBody.clone())?;
            for mut i in &*__alg_iterators.clone().reverse() {
                (iter_name, iter_range) = translateIterator(metamodelica::AsArg::as_arg(&i), &info)?;
                body = list![metamodelica::Ref::new(SCode::Statement::ALG_FOR { index: iter_name, range: iter_range, forBody: body, comment: comment.clone(), info: info.clone() })];
            }
            (body).head().cloned()?
        },
        Deref @ Absyn::Algorithm::ALG_PARFOR { iterators: __alg_iterators, parforBody: __alg_parforBody } => {
            let mut body: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<metamodelica::Ref<Absyn::Exp>>;
            body = translateClassdefAlgorithmitems(__alg_parforBody.clone())?;
            for mut i in &*__alg_iterators.clone().reverse() {
                (iter_name, iter_range) = translateIterator(metamodelica::AsArg::as_arg(&i), &info)?;
                body = list![metamodelica::Ref::new(SCode::Statement::ALG_PARFOR { index: iter_name, range: iter_range, parforBody: body, comment: comment.clone(), info: info.clone() })];
            }
            (body).head().cloned()?
        },
        Deref @ Absyn::Algorithm::ALG_WHILE { boolExpr: __alg_boolExpr, whileBody: __alg_whileBody } => {
            let mut body: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
            body = translateClassdefAlgorithmitems(__alg_whileBody.clone())?;
            metamodelica::Ref::new(SCode::Statement::ALG_WHILE { boolExpr: __alg_boolExpr.clone(), whileBody: body, comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_WHEN_A { boolExpr: __alg_boolExpr, elseWhenAlgorithmBranch: __alg_elseWhenAlgorithmBranch, whenBody: __alg_whenBody } => {
            let mut branches: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Statement>>)>;
            branches = translateAlgBranches(metamodelica::cons((__alg_boolExpr.clone(), __alg_whenBody.clone()), __alg_elseWhenAlgorithmBranch.clone()))?;
            metamodelica::Ref::new(SCode::Statement::ALG_WHEN_A { branches: branches, comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: Deref @ metamodelica::ListNode::Nil } } => {
            metamodelica::Ref::new(SCode::Statement::ALG_ASSERT { condition: e1.clone(), message: e2.clone(), level: ASSERTION_LEVEL_ERROR.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Cons { head: e3, tail: Deref @ metamodelica::ListNode::Nil } } }, argNames: Deref @ metamodelica::ListNode::Nil } } => {
            metamodelica::Ref::new(SCode::Statement::ALG_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "level", argValue: e3 }, tail: Deref @ metamodelica::ListNode::Nil } } } => {
            metamodelica::Ref::new(SCode::Statement::ALG_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "terminate", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, argNames: Deref @ metamodelica::ListNode::Nil } } => {
            metamodelica::Ref::new(SCode::Statement::ALG_TERMINATE { message: e1.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "reinit", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: Deref @ metamodelica::ListNode::Nil } } => {
            metamodelica::Ref::new(SCode::Statement::ALG_REINIT { cref: e1.clone(), newValue: e2.clone(), comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionArgs: __alg_functionArgs, functionCall: __alg_functionCall } => {
            let mut e1: metamodelica::Ref<Absyn::Exp>;
            e1 = metamodelica::Ref::new(Absyn::Exp::CALL { function_: __alg_functionCall.clone(), functionArgs: __alg_functionArgs.clone(), typeVars: metamodelica::nil() });
            metamodelica::Ref::new(SCode::Statement::ALG_NORETCALL { exp: e1, comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_FAILURE { equ: __alg_equ } => {
            let mut body: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
            body = translateClassdefAlgorithmitems(__alg_equ.clone())?;
            metamodelica::Ref::new(SCode::Statement::ALG_FAILURE { stmts: body, comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_TRY { body: __alg_body, elseBody: __alg_elseBody } => {
            let mut body: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
            let mut else_body: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
            body = translateClassdefAlgorithmitems(__alg_body.clone())?;
            else_body = translateClassdefAlgorithmitems(__alg_elseBody.clone())?;
            metamodelica::Ref::new(SCode::Statement::ALG_TRY { body: body, elseBody: else_body, comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_RETURN { .. } => {
            metamodelica::Ref::new(SCode::Statement::ALG_RETURN { comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_BREAK { .. } => {
            metamodelica::Ref::new(SCode::Statement::ALG_BREAK { comment: comment, info: info })
        },
        Deref @ Absyn::Algorithm::ALG_CONTINUE { .. } => {
            metamodelica::Ref::new(SCode::Statement::ALG_CONTINUE { comment: comment, info: info })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStatement)
}

fn translateAlgBranches(
    mut inBranches: metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    )>,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    )>,
> {
    let mut outBranches: metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    )>;
    let mut condition: metamodelica::Ref<Absyn::Exp>;
    let mut body: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
    outBranches = ({
        let mut __acc: metamodelica::List<(
            metamodelica::Ref<Absyn::Exp>,
            metamodelica::List<metamodelica::Ref<SCode::Statement>>,
        )> = metamodelica::nil();
        for mut branch in (inBranches).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(branch.clone()) {
                (__esc_condition, __esc_body) => {
                    condition = (*__esc_condition).clone();
                    body = (*__esc_body).clone();
                    (condition.clone(), translateClassdefAlgorithmitems(body.clone())?)
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outBranches)
}

fn translateClassdefExternaldecls<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<Option<metamodelica::Ref<SCode::ExternalDecl>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::EXTERNAL { externalDecl: Deref @ Absyn::ExternalDecl { funcName: fn_name, lang, output_, args, annotation_: aann }, .. }, tail: _ } => {
                let mut sann: Option<metamodelica::Ref<SCode::Annotation>>;
                sann = translateAnnotationOpt(aann.clone())?;
                return Ok(Some(metamodelica::Ref::new(SCode::ExternalDecl { funcName: fn_name.clone(), lang: lang.clone(), output_: output_.clone(), args: args.clone(), annotation_: sann })))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut res: Option<metamodelica::Ref<SCode::ExternalDecl>>;
                { inAbsynClassPartLst = rest; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(None)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn translateEitemlist(
    mut inAbsynElementItemLst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inVisibility: SCode::Visibility,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outElementLst: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut l: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut es: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = inAbsynElementItemLst;
    let mut ei: metamodelica::Ref<Absyn::ElementItem> =
        <metamodelica::Ref<Absyn::ElementItem> as ::std::default::Default>::default();
    let mut e: metamodelica::Ref<Absyn::Element>;
    for mut ei in &*es {
        let mut ei = ei.clone();
        let () = (match &*ei {
            Absyn::ElementItem::ELEMENTITEM { element: __esc_e } => {
                e = (*__esc_e).clone();
                let mut e_1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                e_1 = translateElement(metamodelica::AsArg::as_arg(&e), inVisibility)?;
                l = List::append_reverse(&e_1, l);
                ()
            }
            _ => (),
        });
    }
    outElementLst = Dangerous::listReverseInPlace(l);
    Ok(outElementLst)
}

// stefan
pub(crate) fn translateAnnotation(
    mut inAnnotation: &metamodelica::Ref<Absyn::Annotation>,
) -> Result<Option<metamodelica::Ref<SCode::Annotation>>> {
    let mut outAnnotation: Option<metamodelica::Ref<SCode::Annotation>>;
    outAnnotation = (::match_deref::match_deref! { match inAnnotation {
        Deref @ Absyn::Annotation { elementArgs: Deref @ metamodelica::ListNode::Nil } => {
            None
        },
        Deref @ Absyn::Annotation { elementArgs: args } => {
            let mut m: metamodelica::Ref<SCode::Mod>;
            m = translateMod(Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, Absyn::dummyInfo.clone(), true)?;
            if (SCodeUtil::isEmptyMod(&m)) {None} else {Some(metamodelica::Ref::new(SCode::Annotation { modification: m }))}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAnnotation)
}

pub(crate) fn translateAnnotationOpt(
    mut absynAnnotation: Option<metamodelica::Ref<Absyn::Annotation>>,
) -> Result<Option<metamodelica::Ref<SCode::Annotation>>> {
    let mut scodeAnnotation: Option<metamodelica::Ref<SCode::Annotation>>;
    scodeAnnotation = (::match_deref::match_deref! { match &(absynAnnotation) {
        Some(ann) => {
            translateAnnotation(metamodelica::AsArg::as_arg(&ann))?
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(scodeAnnotation)
}

pub fn translateElement(
    mut inElement: &metamodelica::Ref<Absyn::Element>,
    mut inVisibility: SCode::Visibility,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outElementLst: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    outElementLst = (match &**inElement {
        Absyn::Element::ELEMENT {
            constrainClass: cc,
            finalPrefix: f,
            innerOuter: io,
            redeclareKeywords: repl,
            specification: s,
            info,
        } => {
            let mut vis = inVisibility;
            let mut es: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            es = translateElementspec(cc.clone(), f.clone(), io.clone(), repl.clone(), vis, s, info.clone())?;
            es
        }
        Absyn::Element::DEFINEUNIT { name, args, info } => {
            let mut vis = inVisibility;
            let mut expOpt: Option<ArcStr>;
            let mut weightOpt: Option<metamodelica::Real>;
            expOpt = translateDefineunitParam(args.clone(), literal!("exp"))?;
            weightOpt = translateDefineunitParam2(args.clone(), literal!("weight"))?;
            list![metamodelica::Ref::new(SCode::Element::DEFINEUNIT {
                name: name.clone(),
                visibility: vis,
                exp: expOpt,
                weight: weightOpt,
                info: info.clone()
            })]
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outElementLst)
}

fn translateDefineunitParam(
    mut inArgs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inArg: ArcStr,
) -> Result<Option<ArcStr>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inArgs, inArg)) {
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: name, argValue: Deref @ Absyn::Exp::STRING { value: r#str } }, tail: _ }, arg) if (metamodelica::stringEq(&name, &arg)) => {
                return Ok(Some(r#str.clone()))
            },
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(None)
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: args }, arg) => {
                { (inArgs, inArg) = (args.clone(), arg.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn translateDefineunitParam2(
    mut inArgs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inArg: ArcStr,
) -> Result<Option<metamodelica::Real>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inArgs, inArg)) {
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: name, argValue: Deref @ Absyn::Exp::REAL { value: s } }, tail: _ }, arg) if (metamodelica::stringEq(&name, &arg)) => {
                let mut r: metamodelica::Real;
                r = stringReal(s.clone())?;
                return Ok(Some(r))
            },
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(None)
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: args }, arg) => {
                { (inArgs, inArg) = (args.clone(), arg.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn translateElementspec(
    mut cc: Option<metamodelica::Ref<Absyn::ConstrainClass>>,
    mut finalPrefix: bool,
    mut io: Absyn::InnerOuter,
    mut inRedeclareKeywords: Option<Absyn::RedeclareKeywords>,
    mut inVisibility: SCode::Visibility,
    mut inElementSpec4: &metamodelica::Ref<Absyn::ElementSpec>,
    mut inInfo: SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outElementLst: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    outElementLst = (::match_deref::match_deref! { match inElementSpec4 {
        Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: rp, class_: Deref @ Absyn::Class { name: n, partialPrefix: pa, encapsulatedPrefix: e, restriction: Absyn::Restriction::R_OPERATOR { .. }, body: de, info: i, .. } } => {
            let mut repl = inRedeclareKeywords;
            let mut vis = inVisibility;
            let mut de_1: metamodelica::Ref<SCode::ClassDef>;
            let mut redecl: bool;
            let mut cmt: metamodelica::Ref<SCode::Comment>;
            let mut cls: metamodelica::Ref<SCode::Element>;
            let mut sRed: SCode::Redeclare;
            let mut sFin: SCode::Final;
            let mut sRep: metamodelica::Ref<SCode::Replaceable>;
            let mut sEnc: SCode::Encapsulated;
            let mut sPar: SCode::Partial;
            let mut scc: Option<metamodelica::Ref<SCode::ConstrainClass>>;
            (de_1, cmt) = translateOperatorDef(metamodelica::AsArg::as_arg(&de), metamodelica::AsArg::as_arg(&n), metamodelica::AsArg::as_arg(&i))?;
            (_, redecl) = translateRedeclarekeywords(repl);
            sRed = SCodeUtil::boolRedeclare(redecl);
            sFin = SCodeUtil::boolFinal(finalPrefix);
            scc = translateConstrainClass(cc)?;
            sRep = if (rp.clone()) {metamodelica::Ref::new(SCode::Replaceable::REPLACEABLE { cc: scc })} else {openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE()};
            sEnc = SCodeUtil::boolEncapsulated(e.clone());
            sPar = SCodeUtil::boolPartial(pa.clone());
            cls = metamodelica::Ref::new(SCode::Element::CLASS { name: n.clone(), prefixes: metamodelica::Ref::new(SCode::Prefixes { visibility: vis, redeclarePrefix: sRed, finalPrefix: sFin, innerOuter: io, replaceablePrefix: sRep }), encapsulatedPrefix: sEnc, partialPrefix: sPar, restriction: openmodelica_frontend_types::SCode::Restriction::R_OPERATOR, classDef: de_1, cmt: cmt, info: i.clone() });
            list![cls]
        },
        Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: rp, class_: cl @ Deref @ Absyn::Class { name: n, partialPrefix: pa, encapsulatedPrefix: e, restriction: re, body: de, info: i, .. } } => {
            let mut repl = inRedeclareKeywords;
            let mut vis = inVisibility;
            let mut de_1: metamodelica::Ref<SCode::ClassDef>;
            let mut re_1: SCode::Restriction;
            let mut redecl: bool;
            let mut cmt: metamodelica::Ref<SCode::Comment>;
            let mut cls: metamodelica::Ref<SCode::Element>;
            let mut sRed: SCode::Redeclare;
            let mut sFin: SCode::Final;
            let mut sRep: metamodelica::Ref<SCode::Replaceable>;
            let mut sEnc: SCode::Encapsulated;
            let mut sPar: SCode::Partial;
            let mut scc: Option<metamodelica::Ref<SCode::ConstrainClass>>;
            re_1 = translateRestriction(cl.clone(), metamodelica::AsArg::as_arg(&re))?;
            (de_1, cmt) = translateClassdef(metamodelica::AsArg::as_arg(&de), i.clone(), &re_1)?;
            (_, redecl) = translateRedeclarekeywords(repl);
            sRed = SCodeUtil::boolRedeclare(redecl);
            sFin = SCodeUtil::boolFinal(finalPrefix);
            scc = translateConstrainClass(cc)?;
            sRep = if (rp.clone()) {metamodelica::Ref::new(SCode::Replaceable::REPLACEABLE { cc: scc })} else {openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE()};
            sEnc = SCodeUtil::boolEncapsulated(e.clone());
            sPar = SCodeUtil::boolPartial(pa.clone());
            cls = metamodelica::Ref::new(SCode::Element::CLASS { name: n.clone(), prefixes: metamodelica::Ref::new(SCode::Prefixes { visibility: vis, redeclarePrefix: sRed, finalPrefix: sFin, innerOuter: io, replaceablePrefix: sRep }), encapsulatedPrefix: sEnc, partialPrefix: sPar, restriction: re_1, classDef: de_1, cmt: cmt, info: i.clone() });
            list![cls]
        },
        Deref @ Absyn::ElementSpec::EXTENDS { path, elementArg: args, annotationOpt: None } => {
            let mut vis = inVisibility;
            let mut info = inInfo;
            let mut r#mod: metamodelica::Ref<SCode::Mod>;
            r#mod = translateMod(Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, Absyn::dummyInfo.clone(), false)?;
            list![metamodelica::Ref::new(SCode::Element::EXTENDS { baseClassPath: path.clone(), visibility: vis, modifications: r#mod, ann: None, info: info })]
        },
        Deref @ Absyn::ElementSpec::EXTENDS { path, elementArg: args, annotationOpt: Some(absann) } => {
            let mut vis = inVisibility;
            let mut info = inInfo;
            let mut r#mod: metamodelica::Ref<SCode::Mod>;
            let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
            r#mod = translateMod(Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, Absyn::dummyInfo.clone(), false)?;
            ann = translateAnnotation(metamodelica::AsArg::as_arg(&absann))?;
            list![metamodelica::Ref::new(SCode::Element::EXTENDS { baseClassPath: path.clone(), visibility: vis, modifications: r#mod, ann: ann, info: info })]
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { components: Deref @ metamodelica::ListNode::Nil, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { flowPrefix: fl, streamPrefix: st, parallelism, variability, direction: di, isField: isf, arrayDim: ad }, typeSpec: t, .. } => {
            let mut repl = inRedeclareKeywords;
            let mut vis = inVisibility;
            let mut info = inInfo;
            let mut repl_1: bool;
            let mut redecl: bool;
            let mut n: ArcStr;
            let mut r#mod: metamodelica::Ref<SCode::Mod>;
            let mut xs_1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut prl1: SCode::Parallelism;
            let mut var1: SCode::Variability;
            let mut tot_dim: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            let mut d: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            let mut m: Option<metamodelica::Ref<Absyn::Modification>>;
            let mut comment: Option<metamodelica::Ref<Absyn::Comment>>;
            let mut cmt: metamodelica::Ref<SCode::Comment>;
            let mut cond: Option<metamodelica::Ref<Absyn::Exp>>;
            let mut sRed: SCode::Redeclare;
            let mut sFin: SCode::Final;
            let mut sRep: metamodelica::Ref<SCode::Replaceable>;
            let mut ct: SCode::ConnectorType;
            let mut prefixes: metamodelica::Ref<SCode::Prefixes>;
            let mut scc: Option<metamodelica::Ref<SCode::ConstrainClass>>;
            xs_1 = metamodelica::nil();
            for mut comp in &*var_field!((**inElementSpec4).components, Absyn::ElementSpec::COMPONENTS).clone() {
                let __arc5 = comp.clone();
                let Absyn::COMPONENTITEM { component: Absyn::COMPONENT { name: __pa0, arrayDim: __pa1, modification: __pa2 }, comment: __pa3, condition: __pa4 } = &*__arc5;
                n = metamodelica::Own::own(__pa0);
                d = metamodelica::Own::own(__pa1);
                m = metamodelica::Own::own(__pa2);
                comment = metamodelica::Own::own(__pa3);
                cond = metamodelica::Own::own(__pa4);
                checkTypeSpec(t, &info)?;
                setHasInnerOuterDefinitionsHandler(io);
                setHasStreamConnectorsHandler(st.clone())?;
                r#mod = translateMod(m, openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, info.clone(), false)?;
                prl1 = translateParallelism(parallelism.clone());
                var1 = translateVariability(variability.clone());
                tot_dim = listAppend(d, ad.clone());
                (repl_1, redecl) = translateRedeclarekeywords(repl.clone());
                (cmt, info) = translateCommentWithLineInfoChanges(comment, info)?;
                sFin = SCodeUtil::boolFinal(finalPrefix);
                sRed = SCodeUtil::boolRedeclare(redecl);
                scc = translateConstrainClass(cc.clone())?;
                sRep = if (repl_1) {metamodelica::Ref::new(SCode::Replaceable::REPLACEABLE { cc: scc })} else {openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE()};
                ct = translateConnectorType(fl.clone(), st.clone())?;
                prefixes = metamodelica::Ref::new(SCode::Prefixes { visibility: vis, redeclarePrefix: sRed, finalPrefix: sFin, innerOuter: io, replaceablePrefix: sRep });
                xs_1 = (match di.clone() {
        Absyn::Direction::INPUT_OUTPUT { .. } if (!(Flags::isSet(Flags::SKIP_INPUT_OUTPUT_SYNTACTIC_SUGAR.clone())?)) => {
            let mut attr1: SCode::Attributes;
            let mut attr2: SCode::Attributes;
            let mut mod2: metamodelica::Ref<SCode::Mod>;
            let mut inName: ArcStr;
            inName = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$in_")); __mm_s.push_str(&*n); ArcStr::from(__mm_s) };
            attr1 = SCode::Attributes { arrayDims: tot_dim.clone(), connectorType: ct, parallelism: prl1, variability: var1, direction: openmodelica_ast::Absyn::Direction::INPUT, isField: isf.clone() };
            attr2 = SCode::Attributes { arrayDims: tot_dim, connectorType: ct, parallelism: prl1, variability: var1, direction: openmodelica_ast::Absyn::Direction::OUTPUT, isField: isf.clone() };
            mod2 = metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: inName.clone(), subscripts: metamodelica::nil() }) })), comment: None, info: info.clone() });
            metamodelica::cons(metamodelica::Ref::new(SCode::Element::COMPONENT { name: n, prefixes: prefixes.clone(), attributes: attr2, typeSpec: t.clone(), modifications: mod2, comment: cmt.clone(), condition: cond.clone(), info: info.clone() }), metamodelica::cons(metamodelica::Ref::new(SCode::Element::COMPONENT { name: inName, prefixes: prefixes, attributes: attr1, typeSpec: t.clone(), modifications: r#mod, comment: cmt, condition: cond, info: info.clone() }), xs_1))
        },
        _ => {
            metamodelica::cons(metamodelica::Ref::new(SCode::Element::COMPONENT { name: n, prefixes: prefixes, attributes: SCode::Attributes { arrayDims: tot_dim, connectorType: ct, parallelism: prl1, variability: var1, direction: di.clone(), isField: isf.clone() }, typeSpec: t.clone(), modifications: r#mod, comment: cmt, condition: cond, info: info.clone() }), xs_1)
        },
    });
            }
            xs_1 = Dangerous::listReverseInPlace(xs_1);
            xs_1
        },
        Deref @ Absyn::ElementSpec::IMPORT { import_: imp, info, .. } => {
            let mut vis = inVisibility;
            let mut xs_1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            xs_1 = translateImports(imp.clone(), vis, info)?;
            xs_1
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("AbsynToSCode.translateElementspec failed")])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElementLst)
}

fn translateImports<'__b>(
    mut imp: Absyn::Import,
    mut visibility: SCode::Visibility,
    mut info: &'__b SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(imp.clone()) {
            Absyn::Import::NAMED_IMPORT { name, path: Deref @ Absyn::Path::FULLYQUALIFIED { path: p } } => {
                { (imp, visibility, info) = (Absyn::Import::NAMED_IMPORT { name: name.clone(), path: p.clone() }, visibility, info); continue '__tco; }
            },
            Absyn::Import::QUAL_IMPORT { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: p } } => {
                { (imp, visibility, info) = (Absyn::Import::QUAL_IMPORT { path: p.clone() }, visibility, info); continue '__tco; }
            },
            Absyn::Import::UNQUAL_IMPORT { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: p } } => {
                { (imp, visibility, info) = (Absyn::Import::UNQUAL_IMPORT { path: p.clone() }, visibility, info); continue '__tco; }
            },
            Absyn::Import::GROUP_IMPORT { prefix: p, groups } => {
                return Ok(List::map3(groups.clone(), &move |__a0: Absyn::GroupImport, __a1: metamodelica::Ref<Absyn::Path>, __a2: SCode::Visibility, __a3: SourceInfo| translateGroupImport(&__a0, __a1, __a2, __a3), p.clone(), visibility, info.clone())?)
            },
            _ => {
                return Ok(list![metamodelica::Ref::new(SCode::Element::IMPORT { imp: imp, visibility: visibility, info: info.clone() })])
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn translateGroupImport(
    mut gimp: &Absyn::GroupImport,
    mut prefix: metamodelica::Ref<Absyn::Path>,
    mut visibility: SCode::Visibility,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut elt: metamodelica::Ref<SCode::Element>;
    elt = (match (gimp.clone(), visibility) {
        (Absyn::GroupImport::GROUP_IMPORT_NAME { name: mut name }, mut vis) => {
            let mut path: metamodelica::Ref<Absyn::Path>;
            path = AbsynUtil::joinPaths(
                prefix,
                metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }),
            )?;
            metamodelica::Ref::new(SCode::Element::IMPORT {
                imp: Absyn::Import::QUAL_IMPORT { path: path },
                visibility: vis,
                info: info,
            })
        }
        (
            Absyn::GroupImport::GROUP_IMPORT_RENAME {
                rename: mut rename,
                name: mut name,
            },
            mut vis,
        ) => {
            let mut path: metamodelica::Ref<Absyn::Path>;
            path = AbsynUtil::joinPaths(
                prefix,
                metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }),
            )?;
            metamodelica::Ref::new(SCode::Element::IMPORT {
                imp: Absyn::Import::NAMED_IMPORT {
                    name: rename.clone(),
                    path: path,
                },
                visibility: vis,
                info: info,
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(elt)
}

fn setHasInnerOuterDefinitionsHandler(mut io: Absyn::InnerOuter) -> () {
    let () = (match io {
        Absyn::InnerOuter::NOT_INNER_OUTER { .. } => (),
        _ => {
            System::setHasInnerOuterDefinitions(true);
            ()
        }
    });
    ()
}

fn setHasStreamConnectorsHandler(mut streamPrefix: bool) -> Result<()> {
    let () = (match streamPrefix {
        false => (),
        true => {
            System::setHasStreamConnectors(true);
            ()
        }
    });
    Ok(())
}

fn translateRedeclarekeywords(mut inRedeclKeywords: Option<Absyn::RedeclareKeywords>) -> (bool, bool) {
    let mut outIsReplaceable: bool;
    let mut outIsRedeclared: bool;
    (outIsReplaceable, outIsRedeclared) = (match inRedeclKeywords {
        Some(Absyn::RedeclareKeywords::REDECLARE { .. }) => (false, true),
        Some(Absyn::RedeclareKeywords::REPLACEABLE { .. }) => (true, false),
        Some(Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE { .. }) => (true, true),
        _ => (false, false),
    });
    (outIsReplaceable, outIsRedeclared)
}

fn translateConstrainClass(
    mut inConstrainClass: Option<metamodelica::Ref<Absyn::ConstrainClass>>,
) -> Result<Option<metamodelica::Ref<SCode::ConstrainClass>>> {
    let mut outConstrainClass: Option<metamodelica::Ref<SCode::ConstrainClass>>;
    outConstrainClass = (::match_deref::match_deref! { match &(inConstrainClass) {
        Some(Deref @ Absyn::ConstrainClass { elementSpec: Deref @ Absyn::ElementSpec::EXTENDS { path: cc_path, elementArg: eltargs, .. }, comment: cmt }) => {
            let mut cc_cmt: metamodelica::Ref<SCode::Comment>;
            let mut r#mod: metamodelica::Ref<Absyn::Modification>;
            let mut cc_mod: metamodelica::Ref<SCode::Mod>;
            r#mod = metamodelica::Ref::new(Absyn::Modification { elementArgLst: eltargs.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() });
            cc_mod = translateMod(Some(r#mod), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, Absyn::dummyInfo.clone(), false)?;
            cc_cmt = translateComment(cmt.clone())?;
            Some(metamodelica::Ref::new(SCode::ConstrainClass { constrainingClass: cc_path.clone(), modifier: cc_mod, comment: cc_cmt }))
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outConstrainClass)
}

fn translateParallelism(mut inParallelism: Absyn::Parallelism) -> SCode::Parallelism {
    let mut outParallelism: SCode::Parallelism;
    outParallelism = (match inParallelism {
        Absyn::Parallelism::PARGLOBAL { .. } => openmodelica_frontend_types::SCode::Parallelism::PARGLOBAL,
        Absyn::Parallelism::PARLOCAL { .. } => openmodelica_frontend_types::SCode::Parallelism::PARLOCAL,
        Absyn::Parallelism::NON_PARALLEL { .. } => openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
    });
    outParallelism
}

fn translateVariability(mut inVariability: Absyn::Variability) -> SCode::Variability {
    let mut outVariability: SCode::Variability;
    outVariability = (match inVariability {
        Absyn::Variability::VAR { .. } => openmodelica_frontend_types::SCode::Variability::VAR,
        Absyn::Variability::DISCRETE { .. } => openmodelica_frontend_types::SCode::Variability::DISCRETE,
        Absyn::Variability::PARAM { .. } => openmodelica_frontend_types::SCode::Variability::PARAM,
        Absyn::Variability::CONST { .. } => openmodelica_frontend_types::SCode::Variability::CONST,
    });
    outVariability
}

fn translateEquations(
    mut inAbsynEquationItemLst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut inIsInitial: bool,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Equation>>> {
    let mut outEquationLst: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    outEquationLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
        for mut eq in (inAbsynEquationItemLst).into_iter().cloned() {
            if !(match &*eq.clone() {
                Absyn::EquationItem::EQUATIONITEM { .. } => true,
                _ => false,
            }) {
                continue;
            }
            let __x = (match &*eq.clone() {
                Absyn::EquationItem::EQUATIONITEM {
                    comment: __eq_comment,
                    equation_: __eq_equation_,
                    info: __eq_info,
                } => {
                    let mut com: metamodelica::Ref<SCode::Comment>;
                    let mut info: SourceInfo;
                    (com, info) = translateCommentWithLineInfoChanges(__eq_comment.clone(), __eq_info.clone())?;
                    translateEquation(
                        metamodelica::AsArg::as_arg(&__eq_equation_),
                        com.clone(),
                        info.clone(),
                        inIsInitial,
                    )?
                }
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outEquationLst)
}

fn translateCommentWithLineInfoChanges(
    mut inComment: Option<metamodelica::Ref<Absyn::Comment>>,
    mut inInfo: SourceInfo,
) -> Result<(metamodelica::Ref<SCode::Comment>, SourceInfo)> {
    let mut outComment: metamodelica::Ref<SCode::Comment>;
    let mut outInfo: SourceInfo;
    outComment = translateComment(inComment)?;
    outInfo = getInfoAnnotationOrDefault(&outComment, inInfo);
    Ok((outComment, outInfo))
}

fn getInfoAnnotationOrDefault(mut comment: &metamodelica::Ref<SCode::Comment>, mut default: SourceInfo) -> SourceInfo {
    let mut info: SourceInfo;
    info = (::match_deref::match_deref! { match comment {
        Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: lst, .. } }), .. } => {
            getInfoAnnotationOrDefault2(metamodelica::AsArg::as_arg(&lst), &default)
        },
        _ => {
            default
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    info
}

fn getInfoAnnotationOrDefault2<'__b>(
    mut lst: &'__b metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut default: &'__b SourceInfo,
) -> SourceInfo {
    '__tco: loop {
        ::match_deref::match_deref! { match lst {
            Deref @ metamodelica::ListNode::Nil => {
                return default.clone()
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: Deref @ "__OpenModelica_FileInfo", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::STRING { value: fileName }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::INTEGER { value: line }, tail: Deref @ metamodelica::ListNode::Nil } } }), .. } }, tail: _ } => {
                return SourceInfo { fileName: fileName.clone(), isReadOnly: false, lineNumberStart: line.clone(), columnNumberStart: 0, lineNumberEnd: line.clone(), columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) }
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { (lst, default) = (rest, default); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn translateComment(
    mut inComment: Option<metamodelica::Ref<Absyn::Comment>>,
) -> Result<metamodelica::Ref<SCode::Comment>> {
    let mut outComment: metamodelica::Ref<SCode::Comment>;
    outComment = (::match_deref::match_deref! { match &(inComment) {
        None => {
            SCode::noComment.clone()
        },
        Some(Deref @ Absyn::Comment { annotation_: absann, comment: ostr }) => {
            let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
            let mut ostr = (*ostr).clone();
            ann = translateAnnotationOpt(absann.clone())?;
            ostr = Util::applyOption(ostr.clone(), &fnptr!(System::unescapedString, ArcStr))?;
            metamodelica::Ref::new(SCode::Comment { annotation_: ann, comment: ostr.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outComment)
}

fn translateCommentList(
    mut inAnns: &metamodelica::List<metamodelica::Ref<Absyn::Annotation>>,
    mut inString: Option<ArcStr>,
) -> Result<metamodelica::Ref<SCode::Comment>> {
    let mut outComment: metamodelica::Ref<SCode::Comment>;
    outComment = (::match_deref::match_deref! { match inAnns {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::Ref::new(SCode::Comment { annotation_: None, comment: inString })
        },
        Deref @ metamodelica::ListNode::Cons { head: absann, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
            let mut ostr: Option<ArcStr>;
            ann = translateAnnotation(metamodelica::AsArg::as_arg(&absann))?;
            ostr = Util::applyOption(inString, &fnptr!(System::unescapedString, ArcStr))?;
            metamodelica::Ref::new(SCode::Comment { annotation_: ann, comment: ostr })
        },
        Deref @ metamodelica::ListNode::Cons { head: absann, tail: anns } => {
            let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
            let mut ostr: Option<ArcStr>;
            let mut absann = (*absann).clone();
            absann = AbsynUtil::mergeAnnotationsList(absann.clone(), anns)?;
            ann = translateAnnotation(metamodelica::AsArg::as_arg(&absann))?;
            ostr = Util::applyOption(inString, &fnptr!(System::unescapedString, ArcStr))?;
            metamodelica::Ref::new(SCode::Comment { annotation_: ann, comment: ostr })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outComment)
}

fn translateCommentSeparate(
    mut inComment: Option<metamodelica::Ref<Absyn::Comment>>,
) -> Result<(Option<metamodelica::Ref<SCode::Annotation>>, Option<ArcStr>)> {
    let mut outAnn: Option<metamodelica::Ref<SCode::Annotation>>;
    let mut outStr: Option<ArcStr>;
    (outAnn, outStr) = (::match_deref::match_deref! { match &(inComment) {
        None => {
            (None, None)
        },
        Some(Deref @ Absyn::Comment { annotation_: None, comment: None }) => {
            (None, None)
        },
        Some(Deref @ Absyn::Comment { annotation_: None, comment: Some(r#str) }) => {
            (None, Some(r#str.clone()))
        },
        Some(Deref @ Absyn::Comment { annotation_: Some(absann), comment: None }) => {
            let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
            ann = translateAnnotation(metamodelica::AsArg::as_arg(&absann))?;
            (ann, None)
        },
        Some(Deref @ Absyn::Comment { annotation_: Some(absann), comment: Some(r#str) }) => {
            let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
            ann = translateAnnotation(metamodelica::AsArg::as_arg(&absann))?;
            (ann, Some(r#str.clone()))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outAnn, outStr))
}

fn translateEquation(
    mut inEquation: &metamodelica::Ref<Absyn::Equation>,
    mut inComment: metamodelica::Ref<SCode::Comment>,
    mut inInfo: SourceInfo,
    mut inIsInitial: bool,
) -> Result<metamodelica::Ref<SCode::Equation>> {
    let mut outEquation: metamodelica::Ref<SCode::Equation>;
    outEquation = (::match_deref::match_deref! { match inEquation {
        Deref @ Absyn::Equation::EQ_IF { elseIfBranches: __inEquation_elseIfBranches, equationElseItems: __inEquation_equationElseItems, equationTrueItems: __inEquation_equationTrueItems, ifExp: __inEquation_ifExp } => {
            let mut else_branch: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
            let mut body: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
            let mut conditions: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut bodies: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Equation>>>;
            body = translateEquations(__inEquation_equationTrueItems.clone(), inIsInitial)?;
            (conditions, bodies) = List::map1_2(metamodelica::AsArg::as_arg(&__inEquation_elseIfBranches), &move |__a0: (metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>), __a1: bool| translateEqBranch(&__a0, __a1), inIsInitial)?;
            conditions = metamodelica::cons(__inEquation_ifExp.clone(), conditions);
            else_branch = translateEquations(__inEquation_equationElseItems.clone(), inIsInitial)?;
            metamodelica::Ref::new(SCode::Equation::EQ_IF { condition: conditions, thenBranch: metamodelica::cons(body, bodies), elseBranch: else_branch, comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_WHEN_E { elseWhenEquations: __inEquation_elseWhenEquations, whenEquations: __inEquation_whenEquations, whenExp: __inEquation_whenExp } => {
            let mut body: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
            let mut branches: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Equation>>)>;
            let mut conditions: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut bodies: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Equation>>>;
            body = translateEquations(__inEquation_whenEquations.clone(), inIsInitial)?;
            (conditions, bodies) = List::map1_2(metamodelica::AsArg::as_arg(&__inEquation_elseWhenEquations), &move |__a0: (metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>), __a1: bool| translateEqBranch(&__a0, __a1), inIsInitial)?;
            branches = ({
        let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Equation>>)> = metamodelica::nil();
        let __thr_src0 = conditions;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = bodies;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(c), Some(b)) => {
                    let __x = (c.clone(), b.clone());
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
            metamodelica::Ref::new(SCode::Equation::EQ_WHEN { condition: __inEquation_whenExp.clone(), eEquationLst: body, elseBranches: branches, comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_EQUALS { leftSide: __inEquation_leftSide, rightSide: __inEquation_rightSide } => {
            metamodelica::Ref::new(SCode::Equation::EQ_EQUALS { expLeft: __inEquation_leftSide.clone(), expRight: __inEquation_rightSide.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_PDE { domain: __inEquation_domain, leftSide: __inEquation_leftSide, rightSide: __inEquation_rightSide } => {
            metamodelica::Ref::new(SCode::Equation::EQ_PDE { expLeft: __inEquation_leftSide.clone(), expRight: __inEquation_rightSide.clone(), domain: __inEquation_domain.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_CONNECT { connector1: __inEquation_connector1, connector2: __inEquation_connector2 } => {
            metamodelica::Ref::new(SCode::Equation::EQ_CONNECT { crefLeft: __inEquation_connector1.clone(), crefRight: __inEquation_connector2.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_FOR { forEquations: __inEquation_forEquations, iterators: __inEquation_iterators } => {
            let mut body: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<metamodelica::Ref<Absyn::Exp>>;
            body = translateEquations(__inEquation_forEquations.clone(), inIsInitial)?;
            for mut i in &*__inEquation_iterators.clone().reverse() {
                (iter_name, iter_range) = translateIterator(metamodelica::AsArg::as_arg(&i), &inInfo)?;
                body = list![metamodelica::Ref::new(SCode::Equation::EQ_FOR { index: iter_name, range: iter_range, eEquationLst: body, comment: inComment.clone(), info: inInfo.clone() })];
            }
            (body).head().cloned()?
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: Deref @ metamodelica::ListNode::Nil } } => {
            metamodelica::Ref::new(SCode::Equation::EQ_ASSERT { condition: e1.clone(), message: e2.clone(), level: ASSERTION_LEVEL_ERROR.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Cons { head: e3, tail: Deref @ metamodelica::ListNode::Nil } } }, argNames: Deref @ metamodelica::ListNode::Nil } } => {
            metamodelica::Ref::new(SCode::Equation::EQ_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "assert", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "level", argValue: e3 }, tail: Deref @ metamodelica::ListNode::Nil } } } => {
            metamodelica::Ref::new(SCode::Equation::EQ_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "terminate", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, argNames: Deref @ metamodelica::ListNode::Nil } } => {
            metamodelica::Ref::new(SCode::Equation::EQ_TERMINATE { message: e1.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "reinit", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: Deref @ metamodelica::ListNode::Nil } } => {
            metamodelica::Ref::new(SCode::Equation::EQ_REINIT { cref: e1.clone(), expReinit: e2.clone(), comment: inComment, info: inInfo })
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionArgs: __inEquation_functionArgs, functionName: __inEquation_functionName } => {
            metamodelica::Ref::new(SCode::Equation::EQ_NORETCALL { exp: metamodelica::Ref::new(Absyn::Exp::CALL { function_: __inEquation_functionName.clone(), functionArgs: __inEquation_functionArgs.clone(), typeVars: metamodelica::nil() }), comment: inComment, info: inInfo })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outEquation)
}

fn translateEqBranch(
    mut inBranch: &(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    ),
    mut inIsInitial: bool,
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<metamodelica::Ref<SCode::Equation>>,
)> {
    let mut outCondition: metamodelica::Ref<Absyn::Exp>;
    let mut outBody: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut body: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    (outCondition, body) = inBranch.clone();
    outBody = translateEquations(body, inIsInitial)?;
    Ok((outCondition, outBody))
}

fn translateIterator(
    mut inIterator: &metamodelica::Ref<Absyn::ForIterator>,
    mut inInfo: &SourceInfo,
) -> Result<(ArcStr, Option<metamodelica::Ref<Absyn::Exp>>)> {
    let mut outName: ArcStr;
    let mut outRange: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut guard_exp: Option<metamodelica::Ref<Absyn::Exp>>;
    let __arc3 = &(*inIterator);
    let Absyn::ITERATOR {
        name: __pa0,
        guardExp: __pa1,
        range: __pa2,
    } = &**__arc3;
    outName = metamodelica::Own::own(__pa0);
    guard_exp = metamodelica::Own::own(__pa1);
    outRange = metamodelica::Own::own(__pa2);
    if (guard_exp).is_some() {
        Error::addSourceMessageAndFail(
            &(Error::INTERNAL_ERROR.clone()),
            list![literal!("For loops with guards not yet implemented")],
            inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    Ok((outName, outRange))
}

fn translateElementAddinfo(
    mut elem: metamodelica::Ref<SCode::Element>,
    mut nfo: SourceInfo,
) -> metamodelica::Ref<SCode::Element> {
    let mut oelem: metamodelica::Ref<SCode::Element>;
    oelem = (match &*elem {
        SCode::Element::COMPONENT {
            name: a1,
            prefixes: p,
            attributes: a6,
            typeSpec: a7,
            modifications: a8,
            comment: a10,
            condition: a11,
            info: _,
        } => metamodelica::Ref::new(SCode::Element::COMPONENT {
            name: a1.clone(),
            prefixes: p.clone(),
            attributes: a6.clone(),
            typeSpec: a7.clone(),
            modifications: a8.clone(),
            comment: a10.clone(),
            condition: a11.clone(),
            info: nfo,
        }),
        _ => elem,
    });
    oelem
}

/* Modification management */
pub fn translateMod(
    mut inMod: Option<metamodelica::Ref<Absyn::Modification>>,
    mut finalPrefix: SCode::Final,
    mut eachPrefix: SCode::Each,
    mut comment: Option<ArcStr>,
    mut info: SourceInfo,
    mut keepEmpty: bool,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut eqmod: metamodelica::Ref<Absyn::EqMod>;
    let mut subs: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut binding: Option<metamodelica::Ref<Absyn::Exp>>;
    (args, eqmod) = (::match_deref::match_deref! { match &(inMod) {
        Some(Deref @ Absyn::Modification { elementArgLst: __esc_args, eqMod: __esc_eqmod }) => {
            args = (*__esc_args).clone();
            eqmod = (*__esc_eqmod).clone();
            (args.clone(), eqmod.clone())
        },
        _ => (metamodelica::nil(), openmodelica_ast::Absyn::EqMod::interned_NOMOD()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    subs = if ((args).is_empty()) {
        metamodelica::nil()
    } else {
        translateArgs(&args, keepEmpty)?
    };
    binding = (match &*eqmod {
        Absyn::EqMod::EQMOD { exp: __eqmod_exp, .. } => Some(__eqmod_exp.clone()),
        _ => None,
    });
    outMod = (::match_deref::match_deref! { match &((subs.clone(), binding.clone(), finalPrefix, eachPrefix)) {
        (Deref @ metamodelica::ListNode::Nil, None, SCode::Final::NOT_FINAL { .. }, SCode::Each::NOT_EACH { .. }) => openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
        _ => metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: finalPrefix, eachPrefix: eachPrefix, subModLst: subs, binding: binding, comment: comment, info: info }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

fn translateArgs(
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut keepEmpty: bool,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::SubMod>>> {
    let mut subMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>> = metamodelica::nil();
    let mut smod: metamodelica::Ref<SCode::Mod>;
    let mut elem: metamodelica::Ref<SCode::Element>;
    let mut sub: metamodelica::Ref<SCode::SubMod>;
    let mut cr1: metamodelica::Ref<Absyn::ComponentRef>;
    let mut cr2: metamodelica::Ref<Absyn::ComponentRef>;
    let mut name: ArcStr;
    for mut arg in &**args {
        subMods = (::match_deref::match_deref! { match &(arg.clone()) {
            Deref @ Absyn::ElementArg::MODIFICATION { comment: __arg_comment, eachPrefix: __arg_eachPrefix, finalPrefix: __arg_finalPrefix, info: __arg_info, modification: __arg_modification, path: __arg_path } => {
                smod = translateMod(__arg_modification.clone(), SCodeUtil::boolFinal(__arg_finalPrefix.clone()), translateEach(__arg_eachPrefix.clone()), __arg_comment.clone(), __arg_info.clone(), false)?;
                if !(SCodeUtil::isEmptyMod(&smod)) || keepEmpty {
                    sub = translateSub(metamodelica::AsArg::as_arg(&__arg_path), &smod, metamodelica::AsArg::as_arg(&__arg_info))?;
                    subMods = metamodelica::cons(sub, subMods);
                }
                subMods
            },
            Deref @ Absyn::ElementArg::REDECLARATION { constrainClass: __arg_constrainClass, eachPrefix: __arg_eachPrefix, elementSpec: __arg_elementSpec, finalPrefix: __arg_finalPrefix, info: __arg_info, redeclareKeywords: __arg_redeclareKeywords } => {
                let __pa0 = ::match_deref::match_deref! { match &(translateElementspec(__arg_constrainClass.clone(), __arg_finalPrefix.clone(), openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, Some(__arg_redeclareKeywords.clone()), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::AsArg::as_arg(&__arg_elementSpec), __arg_info.clone())?) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                elem = metamodelica::Own::own(__pa0);
                sub = metamodelica::Ref::new(SCode::SubMod { ident: AbsynUtil::elementSpecName(metamodelica::AsArg::as_arg(&__arg_elementSpec))?, r#mod: metamodelica::Ref::new(SCode::Mod::REDECL { finalPrefix: SCodeUtil::boolFinal(__arg_finalPrefix.clone()), eachPrefix: translateEach(__arg_eachPrefix.clone()), element: elem }) });
                metamodelica::cons(sub, subMods)
            },
            Deref @ Absyn::ElementArg::ELEMENTARGCOMMENT { .. } => subMods,
            Deref @ Absyn::ElementArg::INHERITANCEBREAK { cnct: Deref @ Absyn::Equation::EQ_CONNECT { connector1: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "break", .. }, connector2: Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_name, .. } }, info: __arg_info } => {
                name = (*__esc_name).clone();
                metamodelica::cons(metamodelica::Ref::new(SCode::SubMod { ident: name.clone(), r#mod: metamodelica::Ref::new(SCode::Mod::BREAK_COMPONENT { info: __arg_info.clone() }) }), subMods)
            },
            Deref @ Absyn::ElementArg::INHERITANCEBREAK { cnct: Deref @ Absyn::Equation::EQ_CONNECT { connector1: __esc_cr1, connector2: __esc_cr2 }, info: __arg_info } => {
                cr1 = (*__esc_cr1).clone();
                cr2 = (*__esc_cr2).clone();
                metamodelica::cons(metamodelica::Ref::new(SCode::SubMod { ident: literal!(""), r#mod: metamodelica::Ref::new(SCode::Mod::BREAK_CONNECT { lhs: cr1.clone(), rhs: cr2.clone(), info: __arg_info.clone() }) }), subMods)
            },
            _ => return Err("match: no arm matched"),
        } });
    }
    subMods = subMods.reverse();
    Ok(subMods)
}

fn translateSub(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inMod: &metamodelica::Ref<SCode::Mod>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<SCode::SubMod>> {
    let mut outSubMod: metamodelica::Ref<SCode::SubMod>;
    outSubMod = (match &**inPath {
        Absyn::Path::IDENT { name: i } => metamodelica::Ref::new(SCode::SubMod {
            ident: i.clone(),
            r#mod: inMod.clone(),
        }),
        Absyn::Path::QUALIFIED { name: i, path } => {
            let mut r#mod: metamodelica::Ref<SCode::Mod>;
            let mut sub: metamodelica::Ref<SCode::SubMod>;
            sub = translateSub(path, inMod, info)?;
            r#mod = metamodelica::Ref::new(SCode::Mod::MOD {
                finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                subModLst: list![sub],
                binding: None,
                comment: None,
                info: info.clone(),
            });
            metamodelica::Ref::new(SCode::SubMod {
                ident: i.clone(),
                r#mod: r#mod,
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outSubMod)
}

fn makeTypeVarElement(mut r#str: ArcStr, mut info: SourceInfo) -> metamodelica::Ref<SCode::Element> {
    let mut elt: metamodelica::Ref<SCode::Element>;
    let mut cd: metamodelica::Ref<SCode::ClassDef>;
    let mut ts: metamodelica::Ref<Absyn::TypeSpec>;
    ts = metamodelica::Ref::new(Absyn::TypeSpec::TCOMPLEX {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("polymorphic"),
        }),
        typeSpecs: list![metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Any") }),
            arrayDim: None
        })],
        arrayDim: None,
    });
    cd = metamodelica::Ref::new(SCode::ClassDef::DERIVED {
        typeSpec: ts,
        modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
        attributes: SCode::Attributes {
            arrayDims: metamodelica::nil(),
            connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
            parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
            variability: openmodelica_frontend_types::SCode::Variability::VAR,
            direction: openmodelica_ast::Absyn::Direction::BIDIR,
            isField: openmodelica_ast::Absyn::IsField::NONFIELD,
        },
    });
    elt = metamodelica::Ref::new(SCode::Element::CLASS {
        name: r#str,
        prefixes: metamodelica::Ref::new(SCode::Prefixes {
            visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC,
            redeclarePrefix: openmodelica_frontend_types::SCode::Redeclare::NOT_REDECLARE,
            finalPrefix: openmodelica_frontend_types::SCode::Final::FINAL,
            innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
            replaceablePrefix: openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE(),
        }),
        encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        restriction: openmodelica_frontend_types::SCode::Restriction::R_TYPE,
        classDef: cd,
        cmt: SCode::noComment.clone(),
        info: info,
    });
    elt
}

fn translateEach(mut inAEach: Absyn::Each) -> SCode::Each {
    let mut outSEach: SCode::Each;
    outSEach = (match inAEach {
        Absyn::Each::EACH { .. } => openmodelica_frontend_types::SCode::Each::EACH,
        Absyn::Each::NON_EACH { .. } => openmodelica_frontend_types::SCode::Each::NOT_EACH,
    });
    outSEach
}

fn checkTypeSpec(mut ts: &metamodelica::Ref<Absyn::TypeSpec>, mut info: &SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match ts {
        Deref @ Absyn::TypeSpec::TPATH { .. } => {
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tuple" }, typeSpecs: Deref @ metamodelica::ListNode::Cons { head: ts2, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut r#str: ArcStr;
            r#str = AbsynUtil::typeSpecString(ts.clone())?;
            Error::addSourceMessage(&(Error::TCOMPLEX_TUPLE_ONE_NAME.clone()), list![r#str], info)?;
            checkTypeSpec(metamodelica::AsArg::as_arg(&ts2), info)?;
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tuple" }, typeSpecs: tss @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, .. } => {
            List::map1_0(metamodelica::AsArg::as_arg(&tss), &move |__a0: metamodelica::Ref<Absyn::TypeSpec>, __a1: SourceInfo| checkTypeSpec(&__a0, &__a1), info.clone())?;
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { typeSpecs: Deref @ metamodelica::ListNode::Cons { head: ts2, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            checkTypeSpec(metamodelica::AsArg::as_arg(&ts2), info)?;
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { typeSpecs: tss, path: __ts_path, .. } => {
            let mut r#str: ArcStr;
            if listMember(__ts_path.clone(), list![metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("list") }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("List") }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("array") }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Array") }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("polymorphic") }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Option") })]) {
                r#str = AbsynUtil::typeSpecString(ts.clone())?;
                Error::addSourceMessage(&(Error::TCOMPLEX_MULTIPLE_NAMES.clone()), list![r#str], info)?;
                List::map1_0(tss, &move |__a0: metamodelica::Ref<Absyn::TypeSpec>, __a1: SourceInfo| checkTypeSpec(&__a0, &__a1), info.clone())?;
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}
