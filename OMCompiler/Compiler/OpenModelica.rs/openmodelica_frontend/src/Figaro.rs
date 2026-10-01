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

use crate::FBuiltin;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Autoconf;
use openmodelica_util::Error;
use openmodelica_util::System;

// Imports
// Aliases
pub type Ident = ArcStr;

pub type Path = metamodelica::Ref<Absyn::Path>;

pub type TypeSpec = metamodelica::Ref<Absyn::TypeSpec>;

pub fn run(
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inPath: &Path,
    mut workingDir: &ArcStr,
    mut inDatabaseFile: ArcStr,
    mut inMode: &ArcStr,
    mut inOptions: &ArcStr,
    mut inFigaroProcessorFile: &ArcStr,
) -> Result<()> {
    let mut bdfFile: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*workingDir);
        __mm_s.push_str(&*literal!("/FigaroObjects.fi"));
        ArcStr::from(__mm_s)
    };
    let mut figaroFile: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*workingDir);
        __mm_s.push_str(&*literal!("/Figaro0.fi"));
        ArcStr::from(__mm_s)
    };
    let mut argumentFile: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*workingDir);
        __mm_s.push_str(&*literal!("/figp_commands.xml"));
        ArcStr::from(__mm_s)
    };
    let mut resultFile: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*System::pwd());
        __mm_s.push_str(&*literal!("/result.xml"));
        ArcStr::from(__mm_s)
    };
    let mut program: metamodelica::Ref<SCode::Element>;
    let mut figaro: ArcStr;
    let mut database: ArcStr;
    let mut xml: ArcStr;
    let mut xml2: ArcStr;
    let mut sl: metamodelica::List<ArcStr>;
    program = FBuiltin::getElementWithPathCheckBuiltin(inProgram.clone(), inPath)?;
    figaro = makeFigaro(&(inProgram.clone()), program, inProgram)?;
    if metamodelica::stringEq(&figaro, &(literal!(""))) {
        return Err("fail");
    }
    System::writeFile(bdfFile.clone(), figaro)?;
    database = inDatabaseFile;
    xml = makeXml(workingDir, database, &bdfFile, inMode, inOptions, &figaroFile)?;
    System::writeFile(argumentFile.clone(), xml)?;
    callFigaroProcessor(inFigaroProcessorFile, &argumentFile);
    if metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT"))) {
        System::systemCall(literal!("timeout 5"), literal!(""));
    } else {
        System::systemCall(literal!("sleep 5"), literal!(""));
    }
    xml2 = System::readFile(resultFile)?;
    sl = interpret(xml2)?;
    if reportErrors(&sl)? {
        return Err("fail");
    }
    Ok(())
}

/// A class that has a corresponding class in Figaro.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FigaroClass {
    pub className: Ident,
    /// Figaro type name
    pub typeName: ArcStr,
}

impl metamodelica::gc::MMTrace for FigaroClass {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.className, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.typeName, __mmv)?;
        Ok(())
    }
}
impl Default for FigaroClass {
    fn default() -> Self {
        Self {
            className: Default::default(),
            typeName: Default::default(),
        }
    }
}

pub type FIGAROCLASS = FigaroClass;

/// A component that will be an object in Figaro.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FigaroObject {
    pub objectName: ArcStr,
    /// Figaro type name
    pub typeName: ArcStr,
    /// a piece of Figaro code that belongs to the object
    pub figaroCode: ArcStr,
}

impl metamodelica::gc::MMTrace for FigaroObject {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.objectName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.typeName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.figaroCode, __mmv)?;
        Ok(())
    }
}
pub type FIGAROOBJECT = FigaroObject;

pub(crate) fn makeFigaro(
    mut inProgram: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inModel: metamodelica::Ref<SCode::Element>,
    mut env: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<ArcStr> {
    let mut outCode: ArcStr;
    let mut fcl: metamodelica::List<FigaroClass>;
    let mut fol: metamodelica::List<FigaroObject>;
    fcl = listAppend(
        fcElementList(
            literal!("Figaro_Object"),
            literal!(""),
            inModel.clone(),
            None,
            inProgram,
            env.clone(),
        )?,
        fcElementList(
            literal!("Figaro_Object_connector"),
            literal!(""),
            inModel.clone(),
            None,
            inProgram,
            env,
        )?,
    );
    printFigaroClassList(&fcl);
    metamodelica::print(literal!("\n\n"));
    fol = foElement(fcl, &inModel)?;
    printFigaroObjectList(&fol);
    outCode = figaroObjectListToString(&fol);
    Ok(outCode)
}

/* Finds all classes derived from the specified base class and also
carries along the Figaro type name in order to assign the correct Figaro type to a class if it
does not have an explicit fullClassName modifier. */
fn fcElement(
    mut inFigaroBase: Ident,
    mut inFigaroType: ArcStr,
    mut inProgram: metamodelica::Ref<SCode::Element>,
    mut inClassName: Option<ArcStr>,
    mut inElement: &metamodelica::Ref<SCode::Element>,
    mut env: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<FigaroClass>> {
    let mut outFigaroClassList: metamodelica::List<FigaroClass>;
    outFigaroClassList = 'mc: {
        let __mc_input = (inFigaroBase, inFigaroType, inProgram, inClassName, &**inElement, env);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, Some(cn), Deref @ SCode::Element::EXTENDS { baseClassPath: bcp, modifications: m, .. }, e) => {
                    let mut tn: ArcStr;
                    let true = (metamodelica::stringEq(&fb, &(getLastIdent(metamodelica::AsArg::as_arg(&bcp))))) else { return Err("pattern mismatch") };
                    tn = fcMod1(metamodelica::AsArg::as_arg(&m))?;
                    Ok(fcAddFigaroClass(ft.clone(), program.clone(), cn.clone(), tn.clone(), e.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, Some(cn), Deref @ SCode::Element::EXTENDS { baseClassPath: bcp, modifications: m, .. }, e) => {
                    let mut cdef: metamodelica::Ref<SCode::Element>;
                    let mut tn: ArcStr;
                    cdef = FBuiltin::getElementWithPathCheckBuiltin(e.clone(), metamodelica::AsArg::as_arg(&bcp))?;
                    let true = (fcExtends(fb.clone(), ft.clone(), program.clone(), Some(cn.clone()), &cdef, e.clone())?) else { return Err("pattern mismatch") };
                    tn = fcMod1(metamodelica::AsArg::as_arg(&m))?;
                    Ok(fcAddFigaroClass(ft.clone(), program.clone(), cn.clone(), tn.clone(), e.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, _, Deref @ SCode::Element::CLASS { name: n, classDef: cd, .. }, e) => {
                    Ok(fcClassDef(fb.clone(), ft.clone(), program.clone(), n.clone(), metamodelica::AsArg::as_arg(&cd), e.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outFigaroClassList)
}

fn fcExtends(
    mut inFigaroBase: Ident,
    mut inFigaroType: ArcStr,
    mut inProgram: metamodelica::Ref<SCode::Element>,
    mut inClassName: Option<ArcStr>,
    mut inElement: &metamodelica::Ref<SCode::Element>,
    mut env: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<bool> {
    let mut doExtend: bool;
    doExtend = 'mc: {
        let __mc_input = (inFigaroBase, inFigaroType, inProgram, inClassName, &**inElement, env);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, _, Deref @ SCode::Element::CLASS { name: n, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: el, .. }, .. }, e) => {
                    Ok(fcElementListExt(fb.clone(), ft.clone(), program.clone(), Some(n.clone()), metamodelica::AsArg::as_arg(&el), e.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, _, _, Some(_), Deref @ SCode::Element::EXTENDS { baseClassPath: bcp, .. }, _) => {
                    let true = (metamodelica::stringEq(&fb, &(getLastIdent(metamodelica::AsArg::as_arg(&bcp))))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, Some(cn), Deref @ SCode::Element::EXTENDS { baseClassPath: bcp, .. }, e) => {
                    let mut cdef: metamodelica::Ref<SCode::Element>;
                    cdef = FBuiltin::getElementWithPathCheckBuiltin(e.clone(), metamodelica::AsArg::as_arg(&bcp))?;
                    Ok(fcExtends(fb.clone(), ft.clone(), program.clone(), Some(cn.clone()), &cdef, e.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(doExtend)
}

fn fcElementListExt(
    mut inFigaroBase: Ident,
    mut inFigaroType: ArcStr,
    mut inProgram: metamodelica::Ref<SCode::Element>,
    mut inClassName: Option<ArcStr>,
    mut inElementList: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut env: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<bool> {
    let mut res: bool;
    res = 'mc: {
        let __mc_input = (
            inFigaroBase,
            inFigaroType,
            inProgram,
            inClassName,
            &**inElementList,
            env,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, cn, Deref @ metamodelica::ListNode::Cons { head: first, tail: _ }, e) => {
                    let true = (fcExtends(fb.clone(), ft.clone(), program.clone(), cn.clone(), metamodelica::AsArg::as_arg(&first), e.clone())?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, cn, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, e) => {
                    Ok(fcElementListExt(fb.clone(), ft.clone(), program.clone(), cn.clone(), metamodelica::AsArg::as_arg(&rest), e.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(res)
}

fn fcAddFigaroClass(
    mut inFigaroType: ArcStr,
    mut inProgram: metamodelica::Ref<SCode::Element>,
    mut inClassName: Ident,
    mut inTypeName: ArcStr,
    mut env: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<FigaroClass>> {
    let mut outFigaroClassList: metamodelica::List<FigaroClass>;
    let mut tn: ArcStr;
    let mut fc: FigaroClass;
    tn = if (metamodelica::stringEq(&inTypeName, &(literal!("")))) {
        inFigaroType
    } else {
        inTypeName
    };
    fc = FigaroClass {
        className: inClassName.clone(),
        typeName: tn.clone(),
    };
    outFigaroClassList = metamodelica::cons(
        fc,
        fcElement(inClassName, tn, inProgram.clone(), None, &(inProgram), env)?,
    );
    Ok(outFigaroClassList)
}

fn fcClassDef(
    mut inFigaroBase: Ident,
    mut inFigaroType: ArcStr,
    mut inProgram: metamodelica::Ref<SCode::Element>,
    mut inClassName: Ident,
    mut inClassDef: &metamodelica::Ref<SCode::ClassDef>,
    mut env: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<FigaroClass>> {
    let mut outFigaroClassList: metamodelica::List<FigaroClass>;
    outFigaroClassList = (match &**inClassDef {
        SCode::ClassDef::PARTS { elementLst: el, .. } => {
            let mut fb = inFigaroBase;
            let mut ft = inFigaroType;
            let mut program = inProgram;
            let mut cn = inClassName;
            let mut e = env;
            fcElementList(fb, ft, program, Some(cn), el, e)?
        }
        SCode::ClassDef::DERIVED {
            typeSpec: ts,
            modifications: m,
            ..
        } => {
            let mut fb = inFigaroBase;
            let mut ft = inFigaroType;
            let mut program = inProgram;
            let mut cn = inClassName;
            let mut e = env;
            let mut p: Path;
            let mut tn: ArcStr;
            p = AbsynUtil::typeSpecPath(ts);
            let true = (metamodelica::stringEq(&fb, &(getLastIdent(&p)))) else {
                return Err("pattern mismatch");
            };
            tn = fcMod1(m)?;
            fcAddFigaroClass(ft, program, cn, tn, e)?
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outFigaroClassList)
}

fn fcElementList(
    mut inFigaroBase: Ident,
    mut inFigaroType: ArcStr,
    mut inProgram: metamodelica::Ref<SCode::Element>,
    mut inClassName: Option<ArcStr>,
    mut inElementList: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut env: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<FigaroClass>> {
    let mut outFigaroClassList: metamodelica::List<FigaroClass>;
    outFigaroClassList = 'mc: {
        let __mc_input = (
            inFigaroBase,
            inFigaroType,
            inProgram,
            inClassName,
            &**inElementList,
            env,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, cn, Deref @ metamodelica::ListNode::Cons { head: first, tail: rest }, e) => {
                    let mut rf: metamodelica::List<FigaroClass>;
                    let mut rr: metamodelica::List<FigaroClass>;
                    rf = fcElement(fb.clone(), ft.clone(), program.clone(), cn.clone(), metamodelica::AsArg::as_arg(&first), e.clone())?;
                    rr = fcElementList(fb.clone(), ft.clone(), program.clone(), cn.clone(), metamodelica::AsArg::as_arg(&rest), e.clone())?;
                    Ok(listAppend(rf.clone(), rr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, cn, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, e) => {
                    Ok(fcElementList(fb.clone(), ft.clone(), program.clone(), cn.clone(), metamodelica::AsArg::as_arg(&rest), e.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outFigaroClassList)
}

fn fcMod1(mut inMod: &metamodelica::Ref<SCode::Mod>) -> Result<ArcStr> {
    let mut outTypeName: ArcStr;
    outTypeName = (match &**inMod {
        SCode::Mod::MOD { subModLst: sml, .. } => fcSubModList(sml),
        SCode::Mod::NOMOD { .. } => {
            literal!("")
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outTypeName)
}

fn fcSubModList(mut inSubModList: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>) -> ArcStr {
    let mut outTypeName: ArcStr;
    outTypeName = 'mc: {
        let __mc_input = &**inSubModList;
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
                Deref @ metamodelica::ListNode::Cons { head: first, tail: _ } => {
                    Ok(fcSubMod(metamodelica::AsArg::as_arg(&first))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(fcSubModList(metamodelica::AsArg::as_arg(&rest)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outTypeName
}

fn fcSubMod(mut inSubMod: &metamodelica::Ref<SCode::SubMod>) -> Result<ArcStr> {
    let mut outTypeName: ArcStr;
    outTypeName = (match &**inSubMod {
        SCode::SubMod { ident: n, r#mod: m } => {
            let true = (metamodelica::stringEq(&n, &(literal!("fullClassName")))) else {
                return Err("pattern mismatch");
            };
            fcMod2(m)?
        }
    });
    Ok(outTypeName)
}

fn fcMod2(mut inMod: &metamodelica::Ref<SCode::Mod>) -> Result<ArcStr> {
    let mut outTypeName: ArcStr;
    outTypeName = (::match_deref::match_deref! { match inMod {
        Deref @ SCode::Mod::MOD { binding: None, .. } => {
            literal!("")
        },
        Deref @ SCode::Mod::MOD { binding: Some(e), .. } => {
            fcExp(metamodelica::AsArg::as_arg(&e))?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outTypeName)
}

fn fcExp(mut inExp: &metamodelica::Ref<Absyn::Exp>) -> Result<ArcStr> {
    let mut outTypeName: ArcStr;
    outTypeName = (match &**inExp {
        Absyn::Exp::STRING { value: tn } => tn.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outTypeName)
}

/* Finds declarations and checks whether the type matches any of the Figaro classes.
If that is the case, then those objects are collected. */
fn foElement(
    mut inFigaroClassList: metamodelica::List<FigaroClass>,
    mut inElement: &metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::List<FigaroObject>> {
    let mut outFigaroObjectList: metamodelica::List<FigaroObject>;
    outFigaroObjectList = (match &**inElement {
        SCode::Element::CLASS { classDef: cd, .. } => {
            let mut fcl = inFigaroClassList;
            foClassDef(fcl, cd)?
        }
        SCode::Element::COMPONENT {
            name: n,
            typeSpec: ts,
            modifications: m,
            ..
        } => {
            let mut fcl = inFigaroClassList;
            let mut p: Path;
            let mut tn: ArcStr;
            let mut c: ArcStr;
            let mut tmp: ArcStr;
            let mut fo: FigaroObject;
            p = AbsynUtil::typeSpecPath(ts);
            tmp = foMod1(m, &(literal!("fullClassName")))?;
            tn = if (metamodelica::stringEq(&tmp, &(literal!("")))) {
                findFigaroTypeName(p, &fcl)?
            } else {
                tmp
            };
            c = foMod1(m, &(literal!("codeInstanceFigaro")))?;
            fo = FigaroObject {
                objectName: n.clone(),
                typeName: tn,
                figaroCode: c,
            };
            list![fo]
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outFigaroObjectList)
}

fn foClassDef(
    mut inFigaroClassList: metamodelica::List<FigaroClass>,
    mut inClassDef: &metamodelica::Ref<SCode::ClassDef>,
) -> Result<metamodelica::List<FigaroObject>> {
    let mut outFigaroObjectList: metamodelica::List<FigaroObject>;
    outFigaroObjectList = (match &**inClassDef {
        SCode::ClassDef::PARTS { elementLst: el, .. } => {
            let mut fcl = inFigaroClassList;
            foElementList(fcl, el)?
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outFigaroObjectList)
}

fn foElementList(
    mut inFigaroClassList: metamodelica::List<FigaroClass>,
    mut inElementList: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<FigaroObject>> {
    let mut outFigaroObjectList: metamodelica::List<FigaroObject>;
    outFigaroObjectList = 'mc: {
        let __mc_input = (inFigaroClassList, &**inElementList);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fcl, Deref @ metamodelica::ListNode::Cons { head: first, tail: rest }) => {
                    let mut rf: metamodelica::List<FigaroObject>;
                    let mut rr: metamodelica::List<FigaroObject>;
                    rf = foElement(fcl.clone(), metamodelica::AsArg::as_arg(&first))?;
                    rr = foElementList(fcl.clone(), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(listAppend(rf.clone(), rr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fcl, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }) => {
                    Ok(foElementList(fcl.clone(), metamodelica::AsArg::as_arg(&rest))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outFigaroObjectList)
}

fn findFigaroTypeName(
    mut inClassPath: Path,
    mut inFigaroClassList: &metamodelica::List<FigaroClass>,
) -> Result<ArcStr> {
    let mut outTypeName: ArcStr;
    outTypeName = 'mc: {
        let __mc_input = (inClassPath, &**inFigaroClassList);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, Deref @ metamodelica::ListNode::Cons { head: first, tail: _ }) => {
                    let mut tn: ArcStr;
                    tn = getFigaroTypeName(p.clone(), metamodelica::AsArg::as_arg(&first))?;
                    Ok(tn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }) => {
                    let mut tn: ArcStr;
                    tn = findFigaroTypeName(p.clone(), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(tn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTypeName)
}

fn getFigaroTypeName(mut inClassPath: Path, mut inFigaroClass: &FigaroClass) -> Result<ArcStr> {
    let mut outTypeName: ArcStr;
    outTypeName = (match inFigaroClass.clone() {
        FigaroClass {
            className: mut cn,
            typeName: mut tn,
        } => {
            let mut p = inClassPath;
            let true = (metamodelica::stringEq(&(getLastIdent(&p)), &cn)) else {
                return Err("pattern mismatch");
            };
            tn.clone()
        }
    });
    Ok(outTypeName)
}

fn foMod1(mut inMod: &metamodelica::Ref<SCode::Mod>, mut name: &ArcStr) -> Result<ArcStr> {
    let mut outCode: ArcStr;
    outCode = (match &**inMod {
        SCode::Mod::MOD { subModLst: sml, .. } => foSubModList(sml, name),
        SCode::Mod::NOMOD { .. } => {
            literal!("")
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outCode)
}

fn foSubModList(mut inSubModList: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>, mut name: &ArcStr) -> ArcStr {
    let mut outCode: ArcStr;
    outCode = 'mc: {
        let __mc_input = &**inSubModList;
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
                Deref @ metamodelica::ListNode::Cons { head: first, tail: _ } => {
                    Ok(foSubMod(metamodelica::AsArg::as_arg(&first), name)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(foSubModList(metamodelica::AsArg::as_arg(&rest), name))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outCode
}

fn foSubMod(mut inSubMod: &metamodelica::Ref<SCode::SubMod>, mut name: &ArcStr) -> Result<ArcStr> {
    let mut outCode: ArcStr;
    outCode = (match &**inSubMod {
        SCode::SubMod { ident: n, r#mod: m } => {
            let true = (metamodelica::stringEq(&n, &name)) else {
                return Err("pattern mismatch");
            };
            foMod2(m)?
        }
    });
    Ok(outCode)
}

fn foMod2(mut inMod: &metamodelica::Ref<SCode::Mod>) -> Result<ArcStr> {
    let mut outCode: ArcStr;
    outCode = (::match_deref::match_deref! { match inMod {
        Deref @ SCode::Mod::MOD { binding: None, .. } => {
            literal!("")
        },
        Deref @ SCode::Mod::MOD { binding: Some(e), .. } => {
            foExp(metamodelica::AsArg::as_arg(&e))?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outCode)
}

fn foExp(mut inExp: &metamodelica::Ref<Absyn::Exp>) -> Result<ArcStr> {
    let mut outCode: ArcStr;
    outCode = (match &**inExp {
        Absyn::Exp::STRING { value: c } => c.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outCode)
}

fn getLastIdent<'__b>(mut inPath: &'__b Path) -> Ident {
    '__tco: loop {
        match &**inPath {
            Absyn::Path::QUALIFIED { path: p, .. } => {
                inPath = p;
                continue '__tco;
            }
            Absyn::Path::IDENT { name: n } => return n.clone(),
            Absyn::Path::FULLYQUALIFIED { path: p } => {
                inPath = p;
                continue '__tco;
            }
        }
    }
}

fn figaroObjectListToString(mut inFigaroObjectList: &metamodelica::List<FigaroObject>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inFigaroObjectList {
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        Deref @ metamodelica::ListNode::Cons { head: first, tail: rest } => {
            let mut rf: ArcStr;
            let mut rr: ArcStr;
            rf = figaroObjectToString(metamodelica::AsArg::as_arg(&first));
            rr = figaroObjectListToString(rest);
            { let mut __mm_s = String::new(); __mm_s.push_str(&*rf); __mm_s.push_str(&*rr); ArcStr::from(__mm_s) }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outString
}

fn figaroObjectToString(mut inFigaroObject: &FigaroObject) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inFigaroObject.clone() {
        FigaroObject {
            objectName: mut on,
            typeName: mut tn,
            figaroCode: mut fc,
        } => {
            let mut middle: ArcStr;
            middle = if (metamodelica::stringEq(&fc, &(literal!("")))) {
                literal!("")
            } else {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*fc);
                    ArcStr::from(__mm_s)
                }
            };
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("OBJECT "));
                __mm_s.push_str(&*on);
                __mm_s.push_str(&*literal!(" IS_A "));
                __mm_s.push_str(&*tn);
                __mm_s.push_str(&*literal!(";"));
                __mm_s.push_str(&*middle);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            }
        }
    });
    outString
}

fn makeXml(
    mut workingDir: &ArcStr,
    mut inDatabase: ArcStr,
    mut inBdfFile: &ArcStr,
    mut inMode: &ArcStr,
    mut inOptions: &ArcStr,
    mut inFigaroFile: &ArcStr,
) -> Result<ArcStr> {
    let mut outXml: ArcStr;
    let mut xml: ArcStr;
    let mut newName: ArcStr;
    let mut sl: metamodelica::List<ArcStr>;
    xml = literal!("<REQUESTS>\n  ");
    xml = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*xml);
        __mm_s.push_str(&*literal!("\n\n<LOAD_BDC_FI>\n    <FILE_FI>"));
        ArcStr::from(__mm_s)
    };
    xml = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*xml);
        __mm_s.push_str(&*inDatabase);
        __mm_s.push_str(&*literal!("</FILE_FI>\n"));
        ArcStr::from(__mm_s)
    };
    sl = stringListStringChar(inDatabase);
    newName = truncateExtension(&sl)?;
    if System::regularFileExists({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*newName);
        __mm_s.push_str(&*literal!(".bdc"));
        ArcStr::from(__mm_s)
    }) {
        xml = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*xml);
            __mm_s.push_str(&*literal!("<FILE> "));
            __mm_s.push_str(&*newName);
            __mm_s.push_str(&*literal!(".bdc</FILE>\n"));
            ArcStr::from(__mm_s)
        };
    }
    xml = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*xml);
        __mm_s.push_str(&*literal!("</LOAD_BDC_FI>\n"));
        ArcStr::from(__mm_s)
    };
    xml = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*xml);
        __mm_s.push_str(&*literal!("\n\n<LOAD_BDF_FI>\n    <FILE>"));
        ArcStr::from(__mm_s)
    };
    xml = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*xml);
        __mm_s.push_str(&*inBdfFile);
        ArcStr::from(__mm_s)
    };
    xml = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*xml);
        __mm_s.push_str(&*literal!("</FILE>\n</LOAD_BDF_FI>\n"));
        ArcStr::from(__mm_s)
    };
    xml = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*xml);
        __mm_s.push_str(&*literal!("<RUN_TREATMENT>\n"));
        ArcStr::from(__mm_s)
    };
    if metamodelica::stringEq(&inMode, &(literal!("figaro0"))) {
        xml = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*xml);
            __mm_s.push_str(&*literal!("    <TREATMENT>GENERATE_FIG0</TREATMENT>\n    <FILE>"));
            ArcStr::from(__mm_s)
        };
        xml = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*xml);
            __mm_s.push_str(&*inFigaroFile);
            ArcStr::from(__mm_s)
        };
        xml = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*xml);
            __mm_s.push_str(&*literal!("</FILE>"));
            ArcStr::from(__mm_s)
        };
    } else if metamodelica::stringEq(&inMode, &(literal!("fault-tree"))) {
        xml = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*xml);
            __mm_s.push_str(&*literal!("    <TREATMENT>GENERATE_TREE</TREATMENT>\n    <FILE>"));
            ArcStr::from(__mm_s)
        };
        xml = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*xml);
            __mm_s.push_str(&*workingDir);
            __mm_s.push_str(&*literal!("/FaultTree.xml"));
            ArcStr::from(__mm_s)
        };
        xml = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*xml);
            __mm_s.push_str(&*literal!("</FILE>\n"));
            ArcStr::from(__mm_s)
        };
        xml = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*xml);
            __mm_s.push_str(&*literal!("    <FILE_MACRO>fiab_ADD.h</FILE_MACRO>"));
            ArcStr::from(__mm_s)
        };
        xml = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*xml);
            __mm_s.push_str(&*literal!("\n    <FILE_TREE_OPTIONS>"));
            __mm_s.push_str(&*inOptions);
            __mm_s.push_str(&*literal!("</FILE_TREE_OPTIONS>"));
            ArcStr::from(__mm_s)
        };
    }
    xml = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*xml);
        __mm_s.push_str(&*literal!("\n    <RESOLVE_CONST>VRAI</RESOLVE_CONST>\n    <RESOLVE_ATTR>FAUX</RESOLVE_ATTR>\n    <INST_RULE>VRAI</INST_RULE>\n"));
        ArcStr::from(__mm_s)
    };
    xml = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*xml);
        __mm_s.push_str(&*literal!("</RUN_TREATMENT>\n</REQUESTS>"));
        ArcStr::from(__mm_s)
    };
    outXml = xml;
    Ok(outXml)
}

fn truncateExtension(mut name: &metamodelica::List<ArcStr>) -> Result<ArcStr> {
    let mut newName: ArcStr;
    newName = (::match_deref::match_deref! { match name {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ ".", tail: _ } => {
            literal!("")
        },
        Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
            stringAppend(c.clone(), truncateExtension(rest)?)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(newName)
}

fn callFigaroProcessor(mut inFigaroProcessorFile: &ArcStr, mut inArgumentFile: &ArcStr) -> () {
    let mut command: ArcStr;
    command = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("start "));
        __mm_s.push_str(&*inFigaroProcessorFile);
        __mm_s.push_str(&*literal!(" -testxml "));
        __mm_s.push_str(&*inArgumentFile);
        ArcStr::from(__mm_s)
    };
    System::systemCall(command, literal!(""));
    ()
}

/// An XML token.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Token {
    OPENTAG { tagName: ArcStr },
    CLOSETAG { tagName: ArcStr },
    TEXT { text: ArcStr },
}
impl metamodelica::gc::MMTrace for Token {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Token::OPENTAG { tagName } => {
                metamodelica::gc::MMTrace::mm_accept(tagName, __mmv)?;
                Ok(())
            }
            Token::CLOSETAG { tagName } => {
                metamodelica::gc::MMTrace::mm_accept(tagName, __mmv)?;
                Ok(())
            }
            Token::TEXT { text } => {
                metamodelica::gc::MMTrace::mm_accept(text, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::Token::{CLOSETAG, OPENTAG, TEXT};

fn interpret(mut inString: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringList: metamodelica::List<ArcStr>;
    outStringList = (match inString {
        mut s => {
            let mut sl: metamodelica::List<ArcStr>;
            let mut sl2: metamodelica::List<ArcStr>;
            let mut tl: metamodelica::List<Token>;
            let mut tl2: metamodelica::List<Token>;
            let mut tl3: metamodelica::List<Token>;
            sl = stringListStringChar(s);
            tl = scan(sl)?;
            tl2 = removeFirstIfText(tl);
            tl3 = removeTokens(tl2);
            sl2 = parse(&tl3)?;
            sl2
        }
        _ => return Err("fail"),
    });
    Ok(outStringList)
}

fn scan(mut inStringList: metamodelica::List<ArcStr>) -> Result<metamodelica::List<Token>> {
    let mut outTokenList: metamodelica::List<Token>;
    outTokenList = 'mc: {
        let __mc_input = inStringList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "<", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "?", tail: rest } } => {
                    let mut r: metamodelica::List<ArcStr>;
                    r = scanDeclaration(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(scan(r.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "<", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "/", tail: rest } } => {
                    let mut r: metamodelica::List<ArcStr>;
                    let mut t: Token;
                    let mut s: ArcStr;
                    (r, s) = scanTagName(metamodelica::AsArg::as_arg(&rest), literal!(""))?;
                    t = Token::CLOSETAG { tagName: s.clone() };
                    Ok(metamodelica::cons(t.clone(), scan(r.clone())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "<", tail: rest } => {
                    let mut r: metamodelica::List<ArcStr>;
                    let mut t: Token;
                    let mut s: ArcStr;
                    (r, s) = scanTagName(metamodelica::AsArg::as_arg(&rest), literal!(""))?;
                    t = Token::OPENTAG { tagName: s.clone() };
                    Ok(metamodelica::cons(t.clone(), scan(r.clone())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                rest => {
                    let mut r: metamodelica::List<ArcStr>;
                    let mut t: Token;
                    let mut s: ArcStr;
                    (r, s) = scanText(metamodelica::AsArg::as_arg(&rest), literal!(""));
                    t = Token::TEXT { text: s.clone() };
                    Ok(metamodelica::cons(t.clone(), scan(r.clone())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTokenList)
}

fn scanDeclaration<'__b>(mut inStringList: &'__b metamodelica::List<ArcStr>) -> Result<metamodelica::List<ArcStr>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inStringList {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ "?", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ ">", tail: rest } } => {
                return Ok(rest.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { inStringList = rest; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn scanTagName<'__b>(
    mut inStringList: &'__b metamodelica::List<ArcStr>,
    mut inTagName: ArcStr,
) -> Result<(metamodelica::List<ArcStr>, ArcStr)> {
    '__tco: loop {
        ::match_deref::match_deref! { match inStringList {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ ">", tail: rest } => {
                return Ok((rest.clone(), inTagName))
            },
            Deref @ metamodelica::ListNode::Cons { head: first, tail: rest } => {
                { (inStringList, inTagName) = (rest, { let mut __mm_s = String::new(); __mm_s.push_str(&*inTagName); __mm_s.push_str(&*first); ArcStr::from(__mm_s) }); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn scanText<'__b>(
    mut inStringList: &'__b metamodelica::List<ArcStr>,
    mut inText: ArcStr,
) -> (metamodelica::List<ArcStr>, ArcStr) {
    '__tco: loop {
        ::match_deref::match_deref! { match inStringList {
            Deref @ metamodelica::ListNode::Nil => {
                return (metamodelica::nil(), literal!(""))
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ "<", tail: _ } => {
                return (inStringList.clone(), inText)
            },
            Deref @ metamodelica::ListNode::Cons { head: first, tail: rest } => {
                { (inStringList, inText) = (rest, { let mut __mm_s = String::new(); __mm_s.push_str(&*inText); __mm_s.push_str(&*first); ArcStr::from(__mm_s) }); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

/* These functions walk over the token sequence from the lexer and throw away tokens that will not
be usable. E. g., if a tag is not known, the tokens associated with it will be thrown away.
The purpose of this step is to return a very simple sequence for the parser to work on. */
fn removeTokens(mut inTokenList: metamodelica::List<Token>) -> metamodelica::List<Token> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inTokenList) {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: Token::OPENTAG { tagName: tn }, tail: rest } if (isKnownTag(tn.clone()) && !(isInfoTag(tn.clone()))) => {
                let mut r: metamodelica::List<Token>;
                r = removeFirstIfText(rest.clone());
                return metamodelica::cons(Token::OPENTAG { tagName: tn.clone() }, removeTokens(r))
            },
            Deref @ metamodelica::ListNode::Cons { head: Token::OPENTAG { tagName: tn }, tail: rest } if (!(isKnownTag(tn.clone()))) => {
                let mut r: metamodelica::List<Token>;
                r = removeUnknown(metamodelica::AsArg::as_arg(&rest), metamodelica::AsArg::as_arg(&tn));
                { inTokenList = r; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Token::CLOSETAG { tagName: tn }, tail: rest } => {
                let mut r: metamodelica::List<Token>;
                r = removeFirstIfText(rest.clone());
                return metamodelica::cons(Token::CLOSETAG { tagName: tn.clone() }, removeTokens(r))
            },
            Deref @ metamodelica::ListNode::Cons { head: first, tail: rest } => {
                return metamodelica::cons(first.clone(), removeTokens(rest.clone()))
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn removeFirstIfText(mut inTokenList: metamodelica::List<Token>) -> metamodelica::List<Token> {
    let mut outTokenList: metamodelica::List<Token>;
    outTokenList = (::match_deref::match_deref! { match &(inTokenList.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Token::TEXT { .. }, tail: rest } => {
            rest.clone()
        },
        _ => {
            inTokenList
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTokenList
}

fn removeUnknown<'__b>(
    mut inTokenList: &'__b metamodelica::List<Token>,
    mut inTagName: &'__b ArcStr,
) -> metamodelica::List<Token> {
    '__tco: loop {
        ::match_deref::match_deref! { match inTokenList {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: Token::CLOSETAG { tagName: tn }, tail: rest } if (metamodelica::stringEq(&tn, &inTagName)) => {
                return removeFirstIfText(rest.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { (inTokenList, inTagName) = (rest, inTagName); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn isKnownTag(mut inTagName: ArcStr) -> bool {
    let mut outBoolean: bool;
    let mut ktl: metamodelica::List<ArcStr> = list![
        literal!("ANSWERS"),
        literal!("ANSWER"),
        literal!("ERROR"),
        literal!("LABEL"),
        literal!("CRITICITY")
    ];
    outBoolean = listMember(inTagName, ktl);
    outBoolean
}

fn isInfoTag(mut inTagName: ArcStr) -> bool {
    let mut outBoolean: bool;
    let mut itl: metamodelica::List<ArcStr> = list![literal!("LABEL"), literal!("CRITICITY")];
    outBoolean = listMember(inTagName, itl);
    outBoolean
}

fn parse(mut inTokenList: &metamodelica::List<Token>) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringList: metamodelica::List<ArcStr>;
    outStringList = (::match_deref::match_deref! { match inTokenList {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Token::OPENTAG { tagName: tn }, tail: rest } => {
            let true = (metamodelica::stringEq(&tn, &(literal!("ANSWERS")))) else { return Err("pattern mismatch") };
            parseAnswers(rest)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outStringList)
}

fn parseAnswers(mut inTokenList: &metamodelica::List<Token>) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringList: metamodelica::List<ArcStr>;
    let mut sl: metamodelica::List<ArcStr>;
    (sl, _) = parseAnswerList(inTokenList)?;
    outStringList = sl;
    Ok(outStringList)
}

fn parseAnswerList(
    mut inTokenList: &metamodelica::List<Token>,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<Token>)> {
    let mut outStringList: metamodelica::List<ArcStr>;
    let mut outTokenList: metamodelica::List<Token>;
    (outStringList, outTokenList) = (::match_deref::match_deref! { match inTokenList {
        Deref @ metamodelica::ListNode::Cons { head: Token::OPENTAG { tagName: tn }, tail: rest } => {
            let mut sl: metamodelica::List<ArcStr>;
            let mut sl2: metamodelica::List<ArcStr>;
            let mut tl: metamodelica::List<Token>;
            let mut tl2: metamodelica::List<Token>;
            let true = (metamodelica::stringEq(&tn, &(literal!("ANSWER")))) else { return Err("pattern mismatch") };
            (sl, tl) = parseAnswer(rest)?;
            (sl2, tl2) = parseAnswerList(&tl)?;
            (listAppend(sl, sl2), tl2)
        },
        Deref @ metamodelica::ListNode::Cons { head: Token::CLOSETAG { tagName: tn }, tail: rest } => {
            let true = (metamodelica::stringEq(&tn, &(literal!("ANSWERS")))) else { return Err("pattern mismatch") };
            (metamodelica::nil(), rest.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outStringList, outTokenList))
}

fn parseAnswer(
    mut inTokenList: &metamodelica::List<Token>,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<Token>)> {
    let mut outStringList: metamodelica::List<ArcStr>;
    let mut outTokenList: metamodelica::List<Token>;
    (outStringList, outTokenList) = parseErrorList(inTokenList)?;
    Ok((outStringList, outTokenList))
}

fn parseErrorList(
    mut inTokenList: &metamodelica::List<Token>,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<Token>)> {
    let mut outStringList: metamodelica::List<ArcStr>;
    let mut outTokenList: metamodelica::List<Token>;
    (outStringList, outTokenList) = (::match_deref::match_deref! { match inTokenList {
        Deref @ metamodelica::ListNode::Cons { head: Token::OPENTAG { tagName: tn }, tail: rest } => {
            let mut sl: metamodelica::List<ArcStr>;
            let mut sl2: metamodelica::List<ArcStr>;
            let mut tl: metamodelica::List<Token>;
            let mut tl2: metamodelica::List<Token>;
            let true = (metamodelica::stringEq(&tn, &(literal!("ERROR")))) else { return Err("pattern mismatch") };
            (sl, tl) = parseError(rest)?;
            (sl2, tl2) = parseErrorList(&tl)?;
            (listAppend(sl, sl2), tl2)
        },
        Deref @ metamodelica::ListNode::Cons { head: Token::CLOSETAG { tagName: tn }, tail: rest } => {
            let true = (metamodelica::stringEq(&tn, &(literal!("ANSWER")))) else { return Err("pattern mismatch") };
            (metamodelica::nil(), rest.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outStringList, outTokenList))
}

fn parseError(
    mut inTokenList: &metamodelica::List<Token>,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<Token>)> {
    let mut outStringList: metamodelica::List<ArcStr>;
    let mut outTokenList: metamodelica::List<Token>;
    let mut stl: metamodelica::List<(ArcStr, ArcStr)>;
    let mut tl: metamodelica::List<Token>;
    let mut sl: metamodelica::List<ArcStr>;
    (stl, tl) = parseInfoList(inTokenList)?;
    sl = if (isToBeReported(&stl)) {
        list![getMessage(&stl)?]
    } else {
        metamodelica::nil()
    };
    (outStringList, outTokenList) = (sl, tl);
    Ok((outStringList, outTokenList))
}

fn parseInfoList(
    mut inTokenList: &metamodelica::List<Token>,
) -> Result<(metamodelica::List<(ArcStr, ArcStr)>, metamodelica::List<Token>)> {
    let mut outStringTupleList: metamodelica::List<(ArcStr, ArcStr)>;
    let mut outTokenList: metamodelica::List<Token>;
    (outStringTupleList, outTokenList) = (::match_deref::match_deref! { match inTokenList {
        Deref @ metamodelica::ListNode::Cons { head: Token::OPENTAG { tagName: tn }, tail: rest } => {
            let mut s: ArcStr;
            let mut stl: metamodelica::List<(ArcStr, ArcStr)>;
            let mut tl: metamodelica::List<Token>;
            let mut tl2: metamodelica::List<Token>;
            (s, tl) = parseInfo(rest)?;
            (stl, tl2) = parseInfoList(&tl)?;
            (metamodelica::cons((tn.clone(), s), stl), tl2)
        },
        Deref @ metamodelica::ListNode::Cons { head: Token::CLOSETAG { tagName: tn }, tail: rest } => {
            let true = (metamodelica::stringEq(&tn, &(literal!("ERROR")))) else { return Err("pattern mismatch") };
            (metamodelica::nil(), rest.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outStringTupleList, outTokenList))
}

fn parseInfo(mut inTokenList: &metamodelica::List<Token>) -> Result<(ArcStr, metamodelica::List<Token>)> {
    let mut outString: ArcStr;
    let mut outTokenList: metamodelica::List<Token>;
    (outString, outTokenList) = (::match_deref::match_deref! { match inTokenList {
        Deref @ metamodelica::ListNode::Cons { head: Token::TEXT { text: s }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } } => {
            (s.clone(), rest.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outString, outTokenList))
}

fn isToBeReported<'__b>(mut inStringTupleList: &'__b metamodelica::List<(ArcStr, ArcStr)>) -> bool {
    '__tco: loop {
        let mut errorsToReport: metamodelica::List<ArcStr> = list![literal!("FATAL")];
        ::match_deref::match_deref! { match inStringTupleList {
            Deref @ metamodelica::ListNode::Nil => {
                return false
            },
            Deref @ metamodelica::ListNode::Cons { head: (k, v), tail: _ } if (metamodelica::stringEq(&k, &(literal!("CRITICITY")))) => {
                return listMember(v.clone(), errorsToReport)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { inStringTupleList = rest; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getMessage<'__b>(mut inStringTupleList: &'__b metamodelica::List<(ArcStr, ArcStr)>) -> Result<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match inStringTupleList {
            Deref @ metamodelica::ListNode::Cons { head: (k, v), tail: _ } if (metamodelica::stringEq(&k, &(literal!("LABEL")))) => {
                return Ok(v.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { inStringTupleList = rest; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn reportErrors(mut inStringList: &metamodelica::List<ArcStr>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inStringList {
        Deref @ metamodelica::ListNode::Nil => {
            false
        },
        Deref @ metamodelica::ListNode::Cons { head: first, tail: rest } => {
            Error::addMessage(Error::FIGARO_ERROR.clone(), list![first.clone()])?;
            reportErrors(rest)?;
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

/* Debug */
fn printFigaroClassList(mut inFigaroClassList: &metamodelica::List<FigaroClass>) -> () {
    let () = (::match_deref::match_deref! { match inFigaroClassList {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: first, tail: rest } => {
            printFigaroClass(metamodelica::AsArg::as_arg(&first));
            printFigaroClassList(rest);
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
            printFigaroClassList(rest);
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn printFigaroClass(mut inFigaroClass: &FigaroClass) -> () {
    let () = (match inFigaroClass.clone() {
        FigaroClass {
            className: mut cn,
            typeName: mut tn,
        } => {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*cn);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*tn);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            ()
        }
    });
    ()
}

fn printFigaroObjectList(mut inFigaroObjectList: &metamodelica::List<FigaroObject>) -> () {
    let () = (::match_deref::match_deref! { match inFigaroObjectList {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: first, tail: rest } => {
            metamodelica::print(figaroObjectToString(metamodelica::AsArg::as_arg(&first)));
            printFigaroObjectList(rest);
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
            printFigaroObjectList(rest);
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn printTokenList(mut inTokenList: &metamodelica::List<Token>) -> () {
    let () = (::match_deref::match_deref! { match inTokenList {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: first, tail: rest } => {
            printToken(metamodelica::AsArg::as_arg(&first));
            metamodelica::print(literal!("\n"));
            printTokenList(rest);
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
            printTokenList(rest);
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn printToken(mut inToken: &Token) -> () {
    let () = (match inToken.clone() {
        Token::OPENTAG { tagName: mut s } => {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("OPEN: "));
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            });
            ()
        }
        Token::CLOSETAG { tagName: mut s } => {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("CLOSE: "));
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            });
            ()
        }
        Token::TEXT { text: mut s } => {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\""));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!("\""));
                ArcStr::from(__mm_s)
            });
            ()
        }
    });
    ()
}
