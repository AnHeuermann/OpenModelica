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

use crate::Interactive;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Inst;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::FCore;
use openmodelica_program_util::ProgramUtil;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

// stringReal
pub(crate) fn refactorGraphicalAnnotation(
    mut wholeAST: Absyn::Program,
    mut classToRefactor: metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut changedClass: metamodelica::Ref<Absyn::Class>;
    changedClass = (match &*classToRefactor {
        _ => {
            let mut c: metamodelica::Ref<Absyn::Class>;
            c = refactorGraphAnnInClass(
                classToRefactor,
                wholeAST,
                metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
            )?;
            c
        }
    });
    Ok(changedClass)
}

fn refactorGraphAnnInClass(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    outClass = 'mc: {
        let __mc_input = (inClass, inProgram, classPath);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (outClass @ Deref @ Absyn::Class { name: n, body: d, .. }, p, Deref @ Absyn::Path::IDENT { name: Deref @ "" }) => {
                    let mut resultClassDef: metamodelica::Ref<Absyn::ClassDef>;
                    let mut cPath: metamodelica::Ref<Absyn::Path>;
                    let mut env: Interactive::GraphicEnvCache;
                    let mut outClass = (*outClass).clone();
                    cPath = metamodelica::Ref::new(Absyn::Path::IDENT { name: n.clone() });
                    env = Interactive::getClassEnv(p.clone(), cPath.clone())?;
                    resultClassDef = refactorGraphAnnInClassDef(d.clone(), p.clone(), cPath.clone(), env.clone());
                    assign_field!(outClass.body = resultClassDef.clone());
                    Ok(outClass.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (outClass @ Deref @ Absyn::Class { name: n, body: d, .. }, p, cPath) => {
                    let mut env: Interactive::GraphicEnvCache;
                    let mut cPath = (*cPath).clone();
                    cPath = AbsynUtil::joinPaths(cPath.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: n.clone() }))?;
                    env = Interactive::getClassEnv(p.clone(), cPath.clone())?;
                    refactorGraphAnnInClassDef(d.clone(), p.clone(), cPath.clone(), env.clone());
                    Ok(outClass.clone())
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

fn refactorGraphAnnInClassDef(
    mut inDef: metamodelica::Ref<Absyn::ClassDef>,
    mut inProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut inClassEnv: Interactive::GraphicEnvCache,
) -> metamodelica::Ref<Absyn::ClassDef> {
    let mut outDef: metamodelica::Ref<Absyn::ClassDef>;
    outDef = 'mc: {
        let __mc_input = (&*inDef, inProgram, classPath, inClassEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: cp, ann, comment: cmt }, p, cPath, env) => {
                    let mut resultPart: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    resultPart = refactorGraphAnnInClassParts(metamodelica::AsArg::as_arg(&cp), p.clone(), cPath.clone(), metamodelica::AsArg::as_arg(&env))?;
                    Ok(metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: resultPart.clone(), ann: ann.clone(), comment: cmt.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassDef::DERIVED { typeSpec: ts, attributes: attrs, arguments: args, comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annList }), comment: cmt }) }, p, _, _) => {
                    let mut resAnnList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    resAnnList = transformClassAnnList(metamodelica::AsArg::as_arg(&annList), list![literal!("Class")], metamodelica::nil(), p.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::ClassDef::DERIVED { typeSpec: ts.clone(), attributes: attrs.clone(), arguments: args.clone(), comment: Some(metamodelica::Ref::new(Absyn::Comment { annotation_: Some(metamodelica::Ref::new(Absyn::Annotation { elementArgs: resAnnList.clone() })), comment: cmt.clone() })) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inDef.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outDef
}

fn refactorGraphAnnInClassParts(
    mut inParts: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut env: &Interactive::GraphicEnvCache,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outParts = (::match_deref::match_deref! { match inParts {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: firstPart, tail: restParts } => {
            let mut p = inProgram;
            let mut cPath = classPath;
            let mut resParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            let mut resultPart: metamodelica::Ref<Absyn::ClassPart>;
            resultPart = refactorGraphAnnInClassPart(firstPart.clone(), p.clone(), cPath.clone(), env.clone());
            resParts = refactorGraphAnnInClassParts(restParts, p, cPath, env)?;
            metamodelica::cons(resultPart, resParts)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outParts)
}

fn refactorGraphAnnInClassPart(
    mut inPart: metamodelica::Ref<Absyn::ClassPart>,
    mut inProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut inClassEnv: Interactive::GraphicEnvCache,
) -> metamodelica::Ref<Absyn::ClassPart> {
    let mut outPart: metamodelica::Ref<Absyn::ClassPart>;
    outPart = 'mc: {
        let __mc_input = (&*inPart, inProgram, classPath, inClassEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassPart::PUBLIC { contents: elContent }, p, cPath, env) => {
                    let mut resultElContent: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    resultElContent = refactorGraphAnnInContentList(metamodelica::AsArg::as_arg(&elContent), &move |__a0: metamodelica::Ref<Absyn::ElementItem>, __a1: Absyn::Program, __a2: metamodelica::Ref<Absyn::Path>, __a3: Interactive::GraphicEnvCache| refactorGraphAnnInElItem(&__a0, __a1, __a2, __a3), p.clone(), cPath.clone(), env.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: resultElContent.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassPart::PROTECTED { contents: elContent }, p, cPath, env) => {
                    let mut resultElContent: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    resultElContent = refactorGraphAnnInContentList(metamodelica::AsArg::as_arg(&elContent), &move |__a0: metamodelica::Ref<Absyn::ElementItem>, __a1: Absyn::Program, __a2: metamodelica::Ref<Absyn::Path>, __a3: Interactive::GraphicEnvCache| refactorGraphAnnInElItem(&__a0, __a1, __a2, __a3), p.clone(), cPath.clone(), env.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: resultElContent.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassPart::EQUATIONS { contents: eqContent }, p, cPath, env) => {
                    let mut resultEqContent: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    resultEqContent = refactorGraphAnnInContentList(metamodelica::AsArg::as_arg(&eqContent), &move |__a0: metamodelica::Ref<Absyn::EquationItem>, __a1: Absyn::Program, __a2: metamodelica::Ref<Absyn::Path>, __a3: Interactive::GraphicEnvCache| -> metamodelica::Result<_> { ::std::result::Result::Ok(refactorGraphAnnInEqItem(__a0, __a1, &__a2, &__a3)) }, p.clone(), cPath.clone(), env.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::ClassPart::EQUATIONS { contents: resultEqContent.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassPart::ALGORITHMS { contents: algContent }, p, cPath, env) => {
                    let mut resultAlgContent: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
                    resultAlgContent = refactorGraphAnnInContentList(metamodelica::AsArg::as_arg(&algContent), &move |__a0: metamodelica::Ref<Absyn::AlgorithmItem>, __a1: Absyn::Program, __a2: metamodelica::Ref<Absyn::Path>, __a3: Interactive::GraphicEnvCache| -> metamodelica::Result<_> { ::std::result::Result::Ok(refactorGraphAnnInAlgItem(__a0, &__a1, &__a2, &__a3)) }, p.clone(), cPath.clone(), env.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::ClassPart::ALGORITHMS { contents: resultAlgContent.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eqContent }, p, cPath, env) => {
                    let mut resultEqContent: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    resultEqContent = refactorGraphAnnInContentList(metamodelica::AsArg::as_arg(&eqContent), &move |__a0: metamodelica::Ref<Absyn::EquationItem>, __a1: Absyn::Program, __a2: metamodelica::Ref<Absyn::Path>, __a3: Interactive::GraphicEnvCache| -> metamodelica::Result<_> { ::std::result::Result::Ok(refactorGraphAnnInEqItem(__a0, __a1, &__a2, &__a3)) }, p.clone(), cPath.clone(), env.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::ClassPart::INITIALEQUATIONS { contents: resultEqContent.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algContent }, p, cPath, env) => {
                    let mut resultAlgContent: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
                    resultAlgContent = refactorGraphAnnInContentList(metamodelica::AsArg::as_arg(&algContent), &move |__a0: metamodelica::Ref<Absyn::AlgorithmItem>, __a1: Absyn::Program, __a2: metamodelica::Ref<Absyn::Path>, __a3: Interactive::GraphicEnvCache| -> metamodelica::Result<_> { ::std::result::Result::Ok(refactorGraphAnnInAlgItem(__a0, &__a1, &__a2, &__a3)) }, p.clone(), cPath.clone(), env.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::ClassPart::INITIALALGORITHMS { contents: resultAlgContent.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inPart.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outPart
}

fn refactorGraphAnnInContentList<contentType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: &metamodelica::List<contentType>,
    mut refactorGraphAnnInItem: &dyn ::std::ops::Fn(
        contentType,
        Absyn::Program,
        metamodelica::Ref<Absyn::Path>,
        Interactive::GraphicEnvCache,
    ) -> Result<contentType>,
    mut inProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut inClassEnv: Interactive::GraphicEnvCache,
) -> Result<metamodelica::List<contentType>> {
    pub type refactorGraphAnnInContent<contentType: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                contentType,
                Absyn::Program,
                metamodelica::Ref<Absyn::Path>,
                Interactive::GraphicEnvCache,
            ) -> Result<contentType>
            + 'static,
    >;

    let mut outList: metamodelica::List<contentType>;
    outList = (::match_deref::match_deref! { match inList {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: firstItem, tail: restList } => {
            let mut p = inProgram;
            let mut cPath = classPath;
            let mut env = inClassEnv;
            let mut resList: metamodelica::List<contentType>;
            let mut resultItem: contentType;
            resultItem = refactorGraphAnnInItem(firstItem.clone(), p.clone(), cPath.clone(), env.clone())?;
            resList = refactorGraphAnnInContentList(restList, refactorGraphAnnInItem, p, cPath, env)?;
            metamodelica::cons(resultItem, resList)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outList)
}

fn refactorGraphAnnInElItem(
    mut inItem: &metamodelica::Ref<Absyn::ElementItem>,
    mut inProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut inClassEnv: Interactive::GraphicEnvCache,
) -> Result<metamodelica::Ref<Absyn::ElementItem>> {
    let mut outItem: metamodelica::Ref<Absyn::ElementItem>;
    outItem = (match &**inItem {
        Absyn::ElementItem::ELEMENTITEM { element: el } => {
            let mut p = inProgram;
            let mut cPath = classPath;
            let mut env = inClassEnv;
            let mut resultElement: metamodelica::Ref<Absyn::Element>;
            resultElement = refactorGraphAnnInElement(el, p, cPath, env)?;
            metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: resultElement })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outItem)
}

fn refactorGraphAnnInEqItem(
    mut inItem: metamodelica::Ref<Absyn::EquationItem>,
    mut inProgram: Absyn::Program,
    mut classPath: &metamodelica::Ref<Absyn::Path>,
    mut inClassEnv: &Interactive::GraphicEnvCache,
) -> metamodelica::Ref<Absyn::EquationItem> {
    let mut outItem: metamodelica::Ref<Absyn::EquationItem>;
    outItem = 'mc: {
        let __mc_input = (&*inItem, inProgram);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: e, info, comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annList }), comment: com }) }, p) => {
                    let mut annList = (*annList).clone();
                    annList = transformConnectAnnList(metamodelica::AsArg::as_arg(&annList), list![literal!("Connect")], metamodelica::nil(), p.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::EquationItem::EQUATIONITEM { equation_: e.clone(), comment: Some(metamodelica::Ref::new(Absyn::Comment { annotation_: Some(metamodelica::Ref::new(Absyn::Annotation { elementArgs: annList.clone() })), comment: com.clone() })), info: info.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inItem.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outItem
}

fn refactorGraphAnnInAlgItem(
    mut inItem: metamodelica::Ref<Absyn::AlgorithmItem>,
    mut inProgram: &Absyn::Program,
    mut classPath: &metamodelica::Ref<Absyn::Path>,
    mut inClassEnv: &Interactive::GraphicEnvCache,
) -> metamodelica::Ref<Absyn::AlgorithmItem> {
    let mut outItem: metamodelica::Ref<Absyn::AlgorithmItem>;
    outItem = (::match_deref::match_deref! { match &(inItem.clone()) {
        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: alg, info, comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annList }), comment: com }) } => {
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: alg.clone(), comment: Some(metamodelica::Ref::new(Absyn::Comment { annotation_: Some(metamodelica::Ref::new(Absyn::Annotation { elementArgs: annList.clone() })), comment: com.clone() })), info: info.clone() })
        },
        _ => {
            inItem
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outItem
}

fn refactorGraphAnnInElement(
    mut inElement: &metamodelica::Ref<Absyn::Element>,
    mut inProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut inClassEnv: Interactive::GraphicEnvCache,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut outElement: metamodelica::Ref<Absyn::Element>;
    outElement = (match &**inElement {
        Absyn::Element::ELEMENT {
            finalPrefix: f,
            redeclareKeywords: rdk,
            innerOuter: io,
            specification: es,
            info: i,
            constrainClass: cc,
        } => {
            let mut p = inProgram;
            let mut cPath = classPath;
            let mut env = inClassEnv;
            let mut resultSpec: metamodelica::Ref<Absyn::ElementSpec>;
            let mut cc = (*cc).clone();
            cc = refactorConstrainClass(cc.clone(), p.clone(), cPath.clone(), env.clone())?;
            resultSpec = refactorGraphAnnInElSpec(es, p, cPath, env);
            metamodelica::Ref::new(Absyn::Element::ELEMENT {
                finalPrefix: f.clone(),
                redeclareKeywords: rdk.clone(),
                innerOuter: io.clone(),
                specification: resultSpec,
                info: i.clone(),
                constrainClass: cc.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outElement)
}

fn refactorConstrainClass(
    mut inCC: Option<metamodelica::Ref<Absyn::ConstrainClass>>,
    mut inProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut inClassEnv: Interactive::GraphicEnvCache,
) -> Result<Option<metamodelica::Ref<Absyn::ConstrainClass>>> {
    let mut outCC: Option<metamodelica::Ref<Absyn::ConstrainClass>>;
    outCC = (::match_deref::match_deref! { match &(inCC) {
        Some(Deref @ Absyn::ConstrainClass { elementSpec: es, comment: com }) => {
            let mut p = inProgram;
            let mut cPath = classPath;
            let mut env = inClassEnv;
            let mut resultSpec: metamodelica::Ref<Absyn::ElementSpec>;
            resultSpec = refactorGraphAnnInElSpec(metamodelica::AsArg::as_arg(&es), p, cPath, env);
            Some(metamodelica::Ref::new(Absyn::ConstrainClass { elementSpec: resultSpec, comment: com.clone() }))
        },
        None => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCC)
}

fn refactorGraphAnnInElSpec(
    mut inSpec: &metamodelica::Ref<Absyn::ElementSpec>,
    mut inProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut inClassEnv: Interactive::GraphicEnvCache,
) -> metamodelica::Ref<Absyn::ElementSpec> {
    let mut outSpec: metamodelica::Ref<Absyn::ElementSpec>;
    outSpec = 'mc: {
        let __mc_input = (&**inSpec, inProgram, classPath, inClassEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: r, class_: cl }, p, cPath, _) => {
                    let mut cl1: metamodelica::Ref<Absyn::Class>;
                    cl1 = refactorGraphAnnInClass(cl.clone(), p.clone(), cPath.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::ElementSpec::CLASSDEF { replaceable_: r.clone(), class_: cl1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ElementSpec::COMPONENTS { attributes: at, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path, arrayDim: z }, components: Deref @ metamodelica::ListNode::Cons { head: firstComp, tail: restCompList } }, p, cPath, env) => {
                    let mut resultComp: metamodelica::Ref<Absyn::ComponentItem>;
                    let mut resCompList: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
                    let mut at = (*at).clone();
                    let mut path = (*path).clone();
                    let mut z = (*z).clone();
                    resultComp = refactorGraphAnnInComponentItem(firstComp.clone(), cPath.clone(), path.clone(), p.clone(), env.clone());
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(refactorGraphAnnInElSpec(&(metamodelica::Ref::new(Absyn::ElementSpec::COMPONENTS { attributes: at.clone(), typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: path.clone(), arrayDim: z.clone() }), components: restCompList.clone() })), p.clone(), cPath.clone(), env.clone())) {
                        Deref @ Absyn::ElementSpec::COMPONENTS { attributes: __pa0, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa1, arrayDim: __pa2 }, components: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    at = metamodelica::Own::own(__pa0);
                    path = metamodelica::Own::own(__pa1);
                    z = metamodelica::Own::own(__pa2);
                    resCompList = metamodelica::Own::own(__pa3);
                    Ok(metamodelica::Ref::new(Absyn::ElementSpec::COMPONENTS { attributes: at.clone(), typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: path.clone(), arrayDim: z.clone() }), components: metamodelica::cons(resultComp.clone(), resCompList.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inSpec.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outSpec
}

fn refactorGraphAnnInComponentItem(
    mut inCom: metamodelica::Ref<Absyn::ComponentItem>,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
    mut inClassEnv: Interactive::GraphicEnvCache,
) -> metamodelica::Ref<Absyn::ComponentItem> {
    let mut outCom: metamodelica::Ref<Absyn::ComponentItem>;
    outCom = 'mc: {
        let __mc_input = (&*inCom, classPath, inPath, inProgram, inClassEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentItem { component: comp, condition: con, comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annList }), comment: r#str }) }, cPath, path, p, env) => {
                    let mut annList = (*annList).clone();
                    annList = transformComponentAnnList(metamodelica::AsArg::as_arg(&annList), list![literal!("Component")], metamodelica::nil(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::ComponentItem { component: comp.clone(), condition: con.clone(), comment: Some(metamodelica::Ref::new(Absyn::Comment { annotation_: Some(metamodelica::Ref::new(Absyn::Annotation { elementArgs: annList.clone() })), comment: r#str.clone() })) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCom.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outCom
}

fn transformComponentAnnList(
    mut inArgs: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inCon: Context,
    mut resultList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
    mut inClassEnv: Interactive::GraphicEnvCache,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut outArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    outArgs = 'mc: {
        let __mc_input = (&**inArgs, inCon, resultList, classPath, inPath, inProgram, inClassEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, res, _, _, _, _) => {
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "extent" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: x1, tail: Deref @ metamodelica::ListNode::Cons { head: y1, tail: Deref @ metamodelica::ListNode::Nil } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: x2, tail: Deref @ metamodelica::ListNode::Cons { head: y2, tail: Deref @ metamodelica::ListNode::Nil } }, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, .. }), comment: com, info }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Component", tail: _ }, res, cPath, path, p, env) => {
                    let mut trans: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let mut iconTrans: metamodelica::Ref<Absyn::ElementArg>;
                    let mut diagramTrans: metamodelica::Ref<Absyn::ElementArg>;
                    let mut rot: Option<metamodelica::Real>;
                    let mut res = (*res).clone();
                    let Absyn::R_CONNECTOR { .. } = (getRestrictionFromPath(cPath.clone(), path.clone(), p.clone(), env.clone())?) else { return Err("pattern mismatch") };
                    rot = getRotationDegree(&(listAppend(res.clone(), rest.clone())));
                    iconTrans = getIconTransformation(x1.clone(), y1.clone(), x2.clone(), y2.clone(), rot.clone(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    diagramTrans = getDiagramTransformation(x1.clone(), y1.clone(), x2.clone(), y2.clone(), rot.clone(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    trans = list![diagramTrans.clone(), iconTrans.clone()];
                    res = transformComponentAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    res = list![metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Placement") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: trans.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), comment: com.clone(), info: info.clone() })];
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "extent" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: x1, tail: Deref @ metamodelica::ListNode::Cons { head: y1, tail: Deref @ metamodelica::ListNode::Nil } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: x2, tail: Deref @ metamodelica::ListNode::Cons { head: y2, tail: Deref @ metamodelica::ListNode::Nil } }, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, .. }), comment: com, info }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Component", tail: _ }, res, cPath, path, p, env) => {
                    let mut diagramTrans: metamodelica::Ref<Absyn::ElementArg>;
                    let mut rot: Option<metamodelica::Real>;
                    let mut res = (*res).clone();
                    rot = getRotationDegree(&(listAppend(res.clone(), rest.clone())));
                    diagramTrans = getDiagramTransformation(x1.clone(), y1.clone(), x2.clone(), y2.clone(), rot.clone(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    res = transformComponentAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    res = list![metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Placement") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: list![diagramTrans.clone()], eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), comment: com.clone(), info: info.clone() })];
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: arg, tail: rest }, context, res, cPath, path, p, env) => {
                    let mut res = (*res).clone();
                    res = metamodelica::cons(arg.clone(), res.clone());
                    res = transformComponentAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outArgs)
}

fn getRestrictionFromPath(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
    mut inClassEnv: Interactive::GraphicEnvCache,
) -> Result<Absyn::Restriction> {
    let mut outRestriction: Absyn::Restriction;
    outRestriction = 'mc: {
        let __mc_input = (classPath, inPath, inProgram, inClassEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cPath, path, p, _) => {
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut fullPath: metamodelica::Ref<Absyn::Path>;
                    let mut restriction: Absyn::Restriction;
                    fullPath = fixPaths(cPath.clone(), path.clone());
                    cdef = ProgramUtil::getPathedClassInProgram(fullPath.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    restriction = getRestrictionInClass(&cdef);
                    Ok(restriction.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, path, p, env) => {
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut fullPath: metamodelica::Ref<Absyn::Path>;
                    let mut restriction: Absyn::Restriction;
                    (_, fullPath) = Interactive::mkFullyQual(env.clone(), path.clone(), false)?;
                    cdef = ProgramUtil::getPathedClassInProgram(fullPath.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    restriction = getRestrictionInClass(&cdef);
                    Ok(restriction.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outRestriction)
}

fn getRestrictionInClass(mut inClass: &metamodelica::Ref<Absyn::Class>) -> Absyn::Restriction {
    let mut outRestriction: Absyn::Restriction;
    outRestriction = (match &**inClass {
        Absyn::Class { restriction, .. } => restriction.clone(),
    });
    outRestriction
}

fn getRotationDegree(
    mut inList: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Option<metamodelica::Real> {
    let mut degrees: Option<metamodelica::Real>;
    degrees = 'mc: {
        let __mc_input = &**inList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(None)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "rotation" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: ex, .. }, .. }), .. }, tail: _ } => {
                    let mut rot: metamodelica::Real;
                    rot = getValueFromExp(metamodelica::AsArg::as_arg(&ex))?;
                    Ok(Some(rot))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut res: Option<metamodelica::Real>;
                    res = getRotationDegree(metamodelica::AsArg::as_arg(&rest));
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    degrees
}

fn getIconTransformation(
    mut ax1: metamodelica::Ref<Absyn::Exp>,
    mut ay1: metamodelica::Ref<Absyn::Exp>,
    mut ax2: metamodelica::Ref<Absyn::Exp>,
    mut ay2: metamodelica::Ref<Absyn::Exp>,
    mut inRotation: Option<metamodelica::Real>,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProg: Absyn::Program,
    mut inClassEnv: Interactive::GraphicEnvCache,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut iconTrans: metamodelica::Ref<Absyn::ElementArg>;
    iconTrans = (match inRotation {
        None => {
            let mut x1 = ax1;
            let mut y1 = ay1;
            let mut x2 = ax2;
            let mut y2 = ay2;
            let mut cPath = classPath;
            let mut path = inPath;
            let mut p = inProg;
            let mut env = inClassEnv;
            let mut rcx1: metamodelica::Real;
            let mut rcy1: metamodelica::Real;
            let mut rcx2: metamodelica::Real;
            let mut rcy2: metamodelica::Real;
            let mut rax1: metamodelica::Real;
            let mut ray1: metamodelica::Real;
            let mut rax2: metamodelica::Real;
            let mut ray2: metamodelica::Real;
            let mut scale: metamodelica::Ref<Absyn::ElementArg>;
            let mut aspectRatio: metamodelica::Ref<Absyn::ElementArg>;
            let mut x: metamodelica::Ref<Absyn::ElementArg>;
            let mut y: metamodelica::Ref<Absyn::ElementArg>;
            let mut flipHorizontal: metamodelica::Ref<Absyn::ElementArg>;
            let mut flipVertical: metamodelica::Ref<Absyn::ElementArg>;
            rax1 = getValueFromExp(&x1)?;
            ray1 = getValueFromExp(&y1)?;
            rax2 = getValueFromExp(&x2)?;
            ray2 = getValueFromExp(&y2)?;
            (x1, y1, x2, y2) = getCoordsInPath(cPath, path, p, list![literal!("Icon")], env)?;
            rcx1 = getValueFromExp(&x1)?;
            rcy1 = getValueFromExp(&y1)?;
            rcx2 = getValueFromExp(&x2)?;
            rcy2 = getValueFromExp(&y2)?;
            aspectRatio = getAspectRatioAnn(rax1, rax2, ray1, ray2, rcx1, rcy1, rcx2, rcy2)?;
            x = getXYAnn(rax1, rax2, literal!("x"))?;
            y = getXYAnn(ray1, ray2, literal!("y"))?;
            scale = getScaleAnn(rax1, rax2, rcx1, rcx2)?;
            flipHorizontal = getFlipAnn(rax1, rax2, literal!("flipHorizontal"));
            flipVertical = getFlipAnn(ray1, ray2, literal!("flipVertical"));
            metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION {
                finalPrefix: false,
                eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH,
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("iconTransformation"),
                }),
                modification: Some(metamodelica::Ref::new(Absyn::Modification {
                    elementArgLst: list![x, y, scale, aspectRatio, flipHorizontal, flipVertical],
                    eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD(),
                })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            })
        }
        Some(mut rot) => {
            let mut x1 = ax1;
            let mut y1 = ay1;
            let mut x2 = ax2;
            let mut y2 = ay2;
            let mut cPath = classPath;
            let mut path = inPath;
            let mut p = inProg;
            let mut env = inClassEnv;
            let mut rcx1: metamodelica::Real;
            let mut rcy1: metamodelica::Real;
            let mut rcx2: metamodelica::Real;
            let mut rcy2: metamodelica::Real;
            let mut rax1: metamodelica::Real;
            let mut ray1: metamodelica::Real;
            let mut rax2: metamodelica::Real;
            let mut ray2: metamodelica::Real;
            let mut scale: metamodelica::Ref<Absyn::ElementArg>;
            let mut aspectRatio: metamodelica::Ref<Absyn::ElementArg>;
            let mut x: metamodelica::Ref<Absyn::ElementArg>;
            let mut y: metamodelica::Ref<Absyn::ElementArg>;
            let mut flipHorizontal: metamodelica::Ref<Absyn::ElementArg>;
            let mut flipVertical: metamodelica::Ref<Absyn::ElementArg>;
            let mut rotation: metamodelica::Ref<Absyn::ElementArg>;
            rax1 = getValueFromExp(&x1)?;
            ray1 = getValueFromExp(&y1)?;
            rax2 = getValueFromExp(&x2)?;
            ray2 = getValueFromExp(&y2)?;
            (x1, y1, x2, y2) = getCoordsInPath(cPath, path, p, list![literal!("Icon")], env)?;
            rcx1 = getValueFromExp(&x1)?;
            rcy1 = getValueFromExp(&y1)?;
            rcx2 = getValueFromExp(&x2)?;
            rcy2 = getValueFromExp(&y2)?;
            aspectRatio = getAspectRatioAnn(rax1, rax2, ray1, ray2, rcx1, rcy1, rcx2, rcy2)?;
            x = getXYAnn(rax1, rax2, literal!("x"))?;
            y = getXYAnn(ray1, ray2, literal!("y"))?;
            scale = getScaleAnn(rax1, rax2, rcx1, rcx2)?;
            flipHorizontal = getFlipAnn(rax1, rax2, literal!("flipHorizontal"));
            flipVertical = getFlipAnn(ray1, ray2, literal!("flipVertical"));
            rotation = getRotationAnn(rot);
            metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION {
                finalPrefix: false,
                eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH,
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("iconTransformation"),
                }),
                modification: Some(metamodelica::Ref::new(Absyn::Modification {
                    elementArgLst: list![x, y, scale, aspectRatio, flipHorizontal, flipVertical, rotation],
                    eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD(),
                })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            })
        }
    });
    Ok(iconTrans)
}

fn getDiagramTransformation(
    mut ax1: metamodelica::Ref<Absyn::Exp>,
    mut ay1: metamodelica::Ref<Absyn::Exp>,
    mut ax2: metamodelica::Ref<Absyn::Exp>,
    mut ay2: metamodelica::Ref<Absyn::Exp>,
    mut inRotation: Option<metamodelica::Real>,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProg: Absyn::Program,
    mut inClassEnv: Interactive::GraphicEnvCache,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut trans: metamodelica::Ref<Absyn::ElementArg>;
    trans = (match inRotation {
        None => {
            let mut x1 = ax1;
            let mut y1 = ay1;
            let mut x2 = ax2;
            let mut y2 = ay2;
            let mut cPath = classPath;
            let mut path = inPath;
            let mut p = inProg;
            let mut env = inClassEnv;
            let mut rcx1: metamodelica::Real;
            let mut rcy1: metamodelica::Real;
            let mut rcx2: metamodelica::Real;
            let mut rcy2: metamodelica::Real;
            let mut rax1: metamodelica::Real;
            let mut ray1: metamodelica::Real;
            let mut rax2: metamodelica::Real;
            let mut ray2: metamodelica::Real;
            let mut scale: metamodelica::Ref<Absyn::ElementArg>;
            let mut aspectRatio: metamodelica::Ref<Absyn::ElementArg>;
            let mut x: metamodelica::Ref<Absyn::ElementArg>;
            let mut y: metamodelica::Ref<Absyn::ElementArg>;
            let mut flipHorizontal: metamodelica::Ref<Absyn::ElementArg>;
            let mut flipVertical: metamodelica::Ref<Absyn::ElementArg>;
            rax1 = getValueFromExp(&x1)?;
            ray1 = getValueFromExp(&y1)?;
            rax2 = getValueFromExp(&x2)?;
            ray2 = getValueFromExp(&y2)?;
            (x1, y1, x2, y2) = getCoordsInPath(cPath, path, p, list![literal!("Diagram")], env)?;
            rcx1 = getValueFromExp(&x1)?;
            rcy1 = getValueFromExp(&y1)?;
            rcx2 = getValueFromExp(&x2)?;
            rcy2 = getValueFromExp(&y2)?;
            aspectRatio = getAspectRatioAnn(rax1, rax2, ray1, ray2, rcx1, rcy1, rcx2, rcy2)?;
            x = getXYAnn(rax1, rax2, literal!("x"))?;
            y = getXYAnn(ray1, ray2, literal!("y"))?;
            scale = getScaleAnn(rax1, rax2, rcx1, rcx2)?;
            flipHorizontal = getFlipAnn(rax1, rax2, literal!("flipHorizontal"));
            flipVertical = getFlipAnn(ray1, ray2, literal!("flipVertical"));
            metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION {
                finalPrefix: false,
                eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH,
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("transformation"),
                }),
                modification: Some(metamodelica::Ref::new(Absyn::Modification {
                    elementArgLst: list![x, y, scale, aspectRatio, flipHorizontal, flipVertical],
                    eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD(),
                })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            })
        }
        Some(mut rot) => {
            let mut x1 = ax1;
            let mut y1 = ay1;
            let mut x2 = ax2;
            let mut y2 = ay2;
            let mut cPath = classPath;
            let mut path = inPath;
            let mut p = inProg;
            let mut env = inClassEnv;
            let mut rcx1: metamodelica::Real;
            let mut rcy1: metamodelica::Real;
            let mut rcx2: metamodelica::Real;
            let mut rcy2: metamodelica::Real;
            let mut rax1: metamodelica::Real;
            let mut ray1: metamodelica::Real;
            let mut rax2: metamodelica::Real;
            let mut ray2: metamodelica::Real;
            let mut scale: metamodelica::Ref<Absyn::ElementArg>;
            let mut aspectRatio: metamodelica::Ref<Absyn::ElementArg>;
            let mut x: metamodelica::Ref<Absyn::ElementArg>;
            let mut y: metamodelica::Ref<Absyn::ElementArg>;
            let mut flipHorizontal: metamodelica::Ref<Absyn::ElementArg>;
            let mut flipVertical: metamodelica::Ref<Absyn::ElementArg>;
            let mut rotation: metamodelica::Ref<Absyn::ElementArg>;
            rax1 = getValueFromExp(&x1)?;
            ray1 = getValueFromExp(&y1)?;
            rax2 = getValueFromExp(&x2)?;
            ray2 = getValueFromExp(&y2)?;
            (x1, y1, x2, y2) = getCoordsInPath(cPath, path, p, list![literal!("Diagram")], env)?;
            rcx1 = getValueFromExp(&x1)?;
            rcy1 = getValueFromExp(&y1)?;
            rcx2 = getValueFromExp(&x2)?;
            rcy2 = getValueFromExp(&y2)?;
            aspectRatio = getAspectRatioAnn(rax1, rax2, ray1, ray2, rcx1, rcy1, rcx2, rcy2)?;
            x = getXYAnn(rax1, rax2, literal!("x"))?;
            y = getXYAnn(ray1, ray2, literal!("y"))?;
            scale = getScaleAnn(rax1, rax2, rcx1, rcx2)?;
            flipHorizontal = getFlipAnn(rax1, rax2, literal!("flipHorizontal"));
            flipVertical = getFlipAnn(ray1, ray2, literal!("flipVertical"));
            rotation = getRotationAnn(rot);
            metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION {
                finalPrefix: false,
                eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH,
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("transformation"),
                }),
                modification: Some(metamodelica::Ref::new(Absyn::Modification {
                    elementArgLst: list![x, y, scale, aspectRatio, flipHorizontal, flipVertical, rotation],
                    eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD(),
                })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            })
        }
    });
    Ok(trans)
}

fn getAspectRatioAnn(
    mut x1: metamodelica::Real,
    mut x2: metamodelica::Real,
    mut y1: metamodelica::Real,
    mut y2: metamodelica::Real,
    mut cx1: metamodelica::Real,
    mut cy1: metamodelica::Real,
    mut cx2: metamodelica::Real,
    mut cy2: metamodelica::Real,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut aspectRatio: metamodelica::Ref<Absyn::ElementArg>;
    aspectRatio = (match (x1, x2, y1, y2, cx1, cy1, cx2, cy2) {
        (mut rx1, mut rx2, mut ry1, mut ry2, mut crx1, mut cry1, mut crx2, mut cry2) => {
            let mut aspect: metamodelica::Real;
            let mut s: ArcStr;
            aspect = metamodelica::real_div_checked(
                realAbs(ry2 - ry1) * realAbs(cry2 - cry1),
                (realAbs(rx2 - rx1) * realAbs(crx2 - crx1)),
            )?;
            s = realString(aspect);
            metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION {
                finalPrefix: false,
                eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH,
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("aspectRatio"),
                }),
                modification: Some(metamodelica::Ref::new(Absyn::Modification {
                    elementArgLst: metamodelica::nil(),
                    eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD {
                        exp: metamodelica::Ref::new(Absyn::Exp::REAL { value: s }),
                        info: Absyn::dummyInfo.clone(),
                    }),
                })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            })
        }
    });
    Ok(aspectRatio)
}

fn getXYAnn(
    mut val1: metamodelica::Real,
    mut val2: metamodelica::Real,
    mut name: ArcStr,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut res: metamodelica::Ref<Absyn::ElementArg>;
    res = (match (val1, val2, name) {
        (mut x1, mut x2, mut n) => {
            let mut value: metamodelica::Real;
            let mut s: ArcStr;
            value = metamodelica::real_div_checked((x1 + x2), metamodelica::OrderedFloat(2.0_f64))?;
            s = realString(value);
            metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION {
                finalPrefix: false,
                eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH,
                path: metamodelica::Ref::new(Absyn::Path::IDENT { name: n }),
                modification: Some(metamodelica::Ref::new(Absyn::Modification {
                    elementArgLst: metamodelica::nil(),
                    eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD {
                        exp: metamodelica::Ref::new(Absyn::Exp::REAL { value: s }),
                        info: Absyn::dummyInfo.clone(),
                    }),
                })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            })
        }
    });
    Ok(res)
}

fn getScaleAnn(
    mut ax1: metamodelica::Real,
    mut ax2: metamodelica::Real,
    mut cx1: metamodelica::Real,
    mut cx2: metamodelica::Real,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut scale: metamodelica::Ref<Absyn::ElementArg>;
    scale = (match (ax1, ax2, cx1, cx2) {
        (mut arx1, mut arx2, mut crx1, mut crx2) => {
            let mut scaleFac: metamodelica::Real;
            let mut s: ArcStr;
            scaleFac = metamodelica::real_div_checked(realAbs(arx1 - arx2), realAbs(crx1 - crx2))?;
            s = realString(scaleFac);
            metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION {
                finalPrefix: false,
                eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH,
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("scale"),
                }),
                modification: Some(metamodelica::Ref::new(Absyn::Modification {
                    elementArgLst: metamodelica::nil(),
                    eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD {
                        exp: metamodelica::Ref::new(Absyn::Exp::REAL { value: s }),
                        info: Absyn::dummyInfo.clone(),
                    }),
                })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            })
        }
    });
    Ok(scale)
}

fn getFlipAnn(
    mut val1: metamodelica::Real,
    mut val2: metamodelica::Real,
    mut name: ArcStr,
) -> metamodelica::Ref<Absyn::ElementArg> {
    let mut flip: metamodelica::Ref<Absyn::ElementArg>;
    let mut value: bool;
    value = val1 > val2;
    flip = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION {
        finalPrefix: false,
        eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH,
        path: metamodelica::Ref::new(Absyn::Path::IDENT { name: name }),
        modification: Some(metamodelica::Ref::new(Absyn::Modification {
            elementArgLst: metamodelica::nil(),
            eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD {
                exp: metamodelica::Ref::new(Absyn::Exp::BOOL { value: value }),
                info: Absyn::dummyInfo.clone(),
            }),
        })),
        comment: None,
        info: Absyn::dummyInfo.clone(),
    });
    flip
}

fn getRotationAnn(mut rot: metamodelica::Real) -> metamodelica::Ref<Absyn::ElementArg> {
    let mut rotation: metamodelica::Ref<Absyn::ElementArg>;
    let mut r: metamodelica::Real;
    let mut s: ArcStr;
    r = rot * metamodelica::OrderedFloat(-1.0_f64);
    s = realString(r);
    rotation = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION {
        finalPrefix: false,
        eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH,
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("rotation"),
        }),
        modification: Some(metamodelica::Ref::new(Absyn::Modification {
            elementArgLst: metamodelica::nil(),
            eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD {
                exp: metamodelica::Ref::new(Absyn::Exp::REAL { value: s }),
                info: Absyn::dummyInfo.clone(),
            }),
        })),
        comment: None,
        info: Absyn::dummyInfo.clone(),
    });
    rotation
}

fn getCoordsInPath(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
    mut contextToGetCoordsFrom: Context,
    mut inClassEnv: Interactive::GraphicEnvCache,
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
)> {
    let mut posX1: metamodelica::Ref<Absyn::Exp>;
    let mut posY1: metamodelica::Ref<Absyn::Exp>;
    let mut posX2: metamodelica::Ref<Absyn::Exp>;
    let mut posY2: metamodelica::Ref<Absyn::Exp>;
    (posX1, posY1, posX2, posY2) = 'mc: {
        let __mc_input = (classPath, inPath, inProgram, contextToGetCoordsFrom, inClassEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cPath, path, p, context, _) => {
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut x1: metamodelica::Ref<Absyn::Exp>;
                    let mut y1: metamodelica::Ref<Absyn::Exp>;
                    let mut x2: metamodelica::Ref<Absyn::Exp>;
                    let mut y2: metamodelica::Ref<Absyn::Exp>;
                    let mut fullPath: metamodelica::Ref<Absyn::Path>;
                    fullPath = fixPaths(cPath.clone(), path.clone());
                    cdef = ProgramUtil::getPathedClassInProgram(fullPath.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    (x1, y1, x2, y2) = getCoordsInClass(&cdef, context.clone())?;
                    Ok((x1.clone(), y1.clone(), x2.clone(), y2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, path, p, context, env) => {
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut x1: metamodelica::Ref<Absyn::Exp>;
                    let mut y1: metamodelica::Ref<Absyn::Exp>;
                    let mut x2: metamodelica::Ref<Absyn::Exp>;
                    let mut y2: metamodelica::Ref<Absyn::Exp>;
                    let mut fullPath: metamodelica::Ref<Absyn::Path>;
                    (_, fullPath) = Interactive::mkFullyQual(env.clone(), path.clone(), false)?;
                    cdef = ProgramUtil::getPathedClassInProgram(fullPath.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    (x1, y1, x2, y2) = getCoordsInClass(&cdef, context.clone())?;
                    Ok((x1.clone(), y1.clone(), x2.clone(), y2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((posX1, posY1, posX2, posY2))
}

fn getCoordsInClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut contextToGetCoordsFrom: Context,
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
)> {
    let mut x1: metamodelica::Ref<Absyn::Exp>;
    let mut y1: metamodelica::Ref<Absyn::Exp>;
    let mut x2: metamodelica::Ref<Absyn::Exp>;
    let mut y2: metamodelica::Ref<Absyn::Exp>;
    (x1, y1, x2, y2) = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { ann, .. }, .. } => {
            let mut context = contextToGetCoordsFrom;
            let mut annlst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
            annlst = List::flatten(List::map(ann.clone(), &move |__a0: metamodelica::Ref<Absyn::Annotation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::annotationToElementArgs(&__a0)) })?)?;
            (x1, y1, x2, y2) = getCoordsInAnnList(&annlst, context)?;
            (x1, y1, x2, y2)
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annlst }), .. }), .. }, .. } => {
            let mut context = contextToGetCoordsFrom;
            (x1, y1, x2, y2) = getCoordsInAnnList(metamodelica::AsArg::as_arg(&annlst), context)?;
            (x1, y1, x2, y2)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((x1, y1, x2, y2))
}

fn getCoordsInAnnList(
    mut inAnns: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut contextToGetCoordsFrom: Context,
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
)> {
    let mut x1: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let mut y1: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let mut x2: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let mut y2: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    (x1, y1, x2, y2) = 'mc: {
        let __mc_input = (&**inAnns, contextToGetCoordsFrom);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok((metamodelica::Ref::new(Absyn::Exp::REAL { value: literal!("-100.0") }), metamodelica::Ref::new(Absyn::Exp::REAL { value: literal!("-100.0") }), metamodelica::Ref::new(Absyn::Exp::REAL { value: literal!("100.0") }), metamodelica::Ref::new(Absyn::Exp::REAL { value: literal!("100.0") })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Coordsys" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), .. }, tail: _ }, _) => {
                    let mut x1: metamodelica::Ref<Absyn::Exp> = x1.clone();
                    let mut x2: metamodelica::Ref<Absyn::Exp> = x2.clone();
                    let mut y1: metamodelica::Ref<Absyn::Exp> = y1.clone();
                    let mut y2: metamodelica::Ref<Absyn::Exp> = y2.clone();
                    (x1, y1, x2, y2) = getCoordsFromCoordSysArgs(metamodelica::AsArg::as_arg(&args))?;
                    Ok(((x1.clone(), y1.clone(), x2.clone(), y2.clone()), x1.clone(), x2.clone(), y1.clone(), y2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            x1 = __wb0;
            x2 = __wb1;
            y1 = __wb2;
            y2 = __wb3;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Icon" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), .. }, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: Deref @ "Icon", tail: _ }) => {
                    let mut x1: metamodelica::Ref<Absyn::Exp> = x1.clone();
                    let mut x2: metamodelica::Ref<Absyn::Exp> = x2.clone();
                    let mut y1: metamodelica::Ref<Absyn::Exp> = y1.clone();
                    let mut y2: metamodelica::Ref<Absyn::Exp> = y2.clone();
                    (x1, y1, x2, y2) = getCoordsFromLayerArgs(metamodelica::AsArg::as_arg(&args))?;
                    Ok(((x1.clone(), y1.clone(), x2.clone(), y2.clone()), x1.clone(), x2.clone(), y1.clone(), y2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            x1 = __wb0;
            x2 = __wb1;
            y1 = __wb2;
            y2 = __wb3;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Diagram" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), .. }, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: Deref @ "Diagram", tail: _ }) => {
                    let mut x1: metamodelica::Ref<Absyn::Exp> = x1.clone();
                    let mut x2: metamodelica::Ref<Absyn::Exp> = x2.clone();
                    let mut y1: metamodelica::Ref<Absyn::Exp> = y1.clone();
                    let mut y2: metamodelica::Ref<Absyn::Exp> = y2.clone();
                    (x1, y1, x2, y2) = getCoordsFromLayerArgs(metamodelica::AsArg::as_arg(&args))?;
                    Ok(((x1.clone(), y1.clone(), x2.clone(), y2.clone()), x1.clone(), x2.clone(), y1.clone(), y2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            x1 = __wb0;
            x2 = __wb1;
            y1 = __wb2;
            y2 = __wb3;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, context) => {
                    let mut x1: metamodelica::Ref<Absyn::Exp> = x1.clone();
                    let mut x2: metamodelica::Ref<Absyn::Exp> = x2.clone();
                    let mut y1: metamodelica::Ref<Absyn::Exp> = y1.clone();
                    let mut y2: metamodelica::Ref<Absyn::Exp> = y2.clone();
                    (x1, y1, x2, y2) = getCoordsInAnnList(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(((x1.clone(), y1.clone(), x2.clone(), y2.clone()), x1.clone(), x2.clone(), y1.clone(), y2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            x1 = __wb0;
            x2 = __wb1;
            y1 = __wb2;
            y2 = __wb3;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((x1, y1, x2, y2))
}

fn getCoordsFromCoordSysArgs<'__b>(
    mut inAnns: &'__b metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
)> {
    let mut x1: metamodelica::Ref<Absyn::Exp>;
    let mut y1: metamodelica::Ref<Absyn::Exp>;
    let mut x2: metamodelica::Ref<Absyn::Exp>;
    let mut y2: metamodelica::Ref<Absyn::Exp>;
    (x1, y1, x2, y2) = (::match_deref::match_deref! { match inAnns {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "extent" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: __esc_x1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_y1, tail: Deref @ metamodelica::ListNode::Nil } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: __esc_x2, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_y2, tail: Deref @ metamodelica::ListNode::Nil } }, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, .. }), .. }, tail: _ } => {
            x1 = (*__esc_x1).clone();
            y1 = (*__esc_y1).clone();
            x2 = (*__esc_x2).clone();
            y2 = (*__esc_y2).clone();
            (x1.clone(), y1.clone(), x2.clone(), y2.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
            (x1, y1, x2, y2) = getCoordsFromCoordSysArgs(rest)?;
            (x1, y1, x2, y2)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((x1, y1, x2, y2))
}

fn getExtentModification<'__b>(
    mut elementArgLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
)> {
    let mut x1: metamodelica::Ref<Absyn::Exp>;
    let mut y1: metamodelica::Ref<Absyn::Exp>;
    let mut x2: metamodelica::Ref<Absyn::Exp>;
    let mut y2: metamodelica::Ref<Absyn::Exp>;
    (x1, y1, x2, y2) = (::match_deref::match_deref! { match elementArgLst {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "extent" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::ListNode::Cons { head: __esc_x1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_y1, tail: Deref @ metamodelica::ListNode::Nil } } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::ListNode::Cons { head: __esc_x2, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_y2, tail: Deref @ metamodelica::ListNode::Nil } } }, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, .. }), .. }, tail: _ } => {
            x1 = (*__esc_x1).clone();
            y1 = (*__esc_y1).clone();
            x2 = (*__esc_x2).clone();
            y2 = (*__esc_y2).clone();
            (x1.clone(), y1.clone(), x2.clone(), y2.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
            (x1, y1, x2, y2) = getExtentModification(rest)?;
            (x1, y1, x2, y2)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((x1, y1, x2, y2))
}

fn getCoordsFromLayerArgs(
    mut inAnns: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<Absyn::Exp>,
)> {
    let mut x1: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let mut y1: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let mut x2: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let mut y2: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    (x1, y1, x2, y2) = 'mc: {
        let __mc_input = &**inAnns;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "coordinateSystem" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), .. }, tail: _ } => {
                    let mut x1: metamodelica::Ref<Absyn::Exp> = x1.clone();
                    let mut x2: metamodelica::Ref<Absyn::Exp> = x2.clone();
                    let mut y1: metamodelica::Ref<Absyn::Exp> = y1.clone();
                    let mut y2: metamodelica::Ref<Absyn::Exp> = y2.clone();
                    (x1, y1, x2, y2) = getExtentModification(metamodelica::AsArg::as_arg(&args))?;
                    Ok(((x1.clone(), y1.clone(), x2.clone(), y2.clone()), x1.clone(), x2.clone(), y1.clone(), y2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            x1 = __wb0;
            x2 = __wb1;
            y1 = __wb2;
            y2 = __wb3;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut x1: metamodelica::Ref<Absyn::Exp> = x1.clone();
                    let mut x2: metamodelica::Ref<Absyn::Exp> = x2.clone();
                    let mut y1: metamodelica::Ref<Absyn::Exp> = y1.clone();
                    let mut y2: metamodelica::Ref<Absyn::Exp> = y2.clone();
                    (x1, y1, x2, y2) = getCoordsFromLayerArgs(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(((x1.clone(), y1.clone(), x2.clone(), y2.clone()), x1.clone(), x2.clone(), y1.clone(), y2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            x1 = __wb0;
            x2 = __wb1;
            y1 = __wb2;
            y2 = __wb3;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((x1, y1, x2, y2))
}

fn transformConnectAnnList(
    mut inArgs: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inCon: Context,
    mut resultList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inProgram: Absyn::Program,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut outArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    outArgs = 'mc: {
        let __mc_input = (&**inArgs, inCon, resultList, inProgram);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, res, _) => {
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "points" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: expMatrix }, info }, .. }), comment: com, info: mod_info }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Connect", tail: _ }, res, p) => {
                    let mut expLst: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut context = (*context).clone();
                    let mut res = (*res).clone();
                    context = addContext(context.clone(), literal!("Line"));
                    expLst = List::map(expMatrix.clone(), &fnptr!(matrixToArray, metamodelica::List<metamodelica::Ref<Absyn::Exp>>))?;
                    res = transformConnectAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(list![metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Line") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("points") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: expLst.clone() }), info: info.clone() }) })), comment: None, info: mod_info.clone() }), res.clone()), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), comment: com.clone(), info: mod_info.clone() })])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "points" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: expMatrix }, info }, .. }), comment: com, info: mod_info }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Line", tail: _ }, res, p) => {
                    let mut expLst: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut res = (*res).clone();
                    expLst = List::map(expMatrix.clone(), &fnptr!(matrixToArray, metamodelica::List<metamodelica::Ref<Absyn::Exp>>))?;
                    res = transformConnectAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("points") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: expLst.clone() }), info: info.clone() }) })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "style" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod }), comment: com, info: mod_info }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Connect", tail: _ }, res, p) => {
                    let mut args = (*args).clone();
                    let mut rest = (*rest).clone();
                    let mut context = (*context).clone();
                    let mut res = (*res).clone();
                    context = addContext(context.clone(), literal!("Line"));
                    args = cleanStyleAttrs(args.clone(), metamodelica::nil(), context.clone())?;
                    rest = listAppend(args.clone(), rest.clone());
                    res = transformConnectAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(list![metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Line") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: res.clone(), eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() })])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "style" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), .. }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Line", tail: _ }, res, p) => {
                    let mut args = (*args).clone();
                    let mut rest = (*rest).clone();
                    let mut res = (*res).clone();
                    args = cleanStyleAttrs(args.clone(), metamodelica::nil(), context.clone())?;
                    rest = listAppend(args.clone(), rest.clone());
                    res = transformConnectAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "color" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, info } }), comment: com, info: mod_info }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Line", tail: _ }, res, p) => {
                    let mut color1: i32;
                    let mut color2: i32;
                    let mut color3: i32;
                    let mut res = (*res).clone();
                    (color1, color2, color3) = getMappedColor(x.clone())?;
                    res = transformConnectAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("color") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color1 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color2 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color3 })] }), info: info.clone() }) })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pattern" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. } }), comment: com, info: mod_info }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Line", tail: _ }, res, p) => {
                    let mut val: ArcStr;
                    let mut res = (*res).clone();
                    val = (patternMapList).get(x.clone() + 1)?;
                    res = transformConnectAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("pattern") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: literal!("LinePattern"), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: val.clone(), subscripts: metamodelica::nil() }) }) }), info: Absyn::dummyInfo.clone() }) })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "thickness" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. } }), comment: com, info: mod_info }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Line", tail: _ }, res, p) => {
                    let mut s: ArcStr;
                    let mut thick: metamodelica::Real;
                    let mut res = (*res).clone();
                    thick = (thicknessMapList).get(x.clone())?;
                    res = transformConnectAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    s = realString(thick);
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("thickness") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: metamodelica::Ref::new(Absyn::Exp::REAL { value: s.clone() }), info: Absyn::dummyInfo.clone() }) })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod }), comment: com, info: mod_info }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Line", tail: _ }, res, p) => {
                    let mut res = (*res).clone();
                    res = transformConnectAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("smooth") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "arrow" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. } }), comment: com, info: mod_info }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Line", tail: _ }, res, p) => {
                    let mut val1: ArcStr;
                    let mut val2: ArcStr;
                    let mut arrows: metamodelica::List<ArcStr>;
                    let mut res = (*res).clone();
                    arrows = (arrowMapList).get(x.clone() + 1)?;
                    val1 = (arrows).get(1)?;
                    val2 = (arrows).get(2)?;
                    res = transformConnectAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("arrow") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: literal!("Arrow"), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: val1.clone(), subscripts: metamodelica::nil() }) }) }), metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: literal!("Arrow"), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: val2.clone(), subscripts: metamodelica::nil() }) }) })] }), info: Absyn::dummyInfo.clone() }) })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: arg, tail: rest }, context, res, p) => {
                    let mut res = (*res).clone();
                    res = transformConnectAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(metamodelica::cons(arg.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outArgs)
}

fn transformClassAnnList(
    mut inArgs: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inCon: Context,
    mut resultList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inProgram: Absyn::Program,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut outArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    outArgs = 'mc: {
        let __mc_input = (&**inArgs, inCon, resultList, inProgram);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, res, _) => {
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "Icon" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod }), comment: com, info: mod_info }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Class", tail: c }, res, p) => {
                    let mut argRes: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut coord: metamodelica::Ref<Absyn::ElementArg>;
                    let mut c = (*c).clone();
                    let mut res = (*res).clone();
                    c = addContext(context.clone(), literal!("Layer"));
                    argRes = transAnnLstToCalls(metamodelica::AsArg::as_arg(&args), c.clone())?;
                    coord = getCoordSysAnn(&(listAppend(res.clone(), rest.clone())), p.clone())?;
                    res = transformClassAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Icon") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: list![coord.clone(), metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("graphics") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: argRes.clone() }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: mod_info.clone() })], eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "Diagram" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod }), comment: com, info: mod_info }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Class", tail: c }, res, p) => {
                    let mut argRes: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut coord: metamodelica::Ref<Absyn::ElementArg>;
                    let mut c = (*c).clone();
                    let mut res = (*res).clone();
                    c = addContext(context.clone(), literal!("Layer"));
                    argRes = transAnnLstToCalls(metamodelica::AsArg::as_arg(&args), c.clone())?;
                    coord = getCoordSysAnn(&(listAppend(res.clone(), rest.clone())), p.clone())?;
                    res = transformClassAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Diagram") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: list![coord.clone(), metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("graphics") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: argRes.clone() }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: mod_info.clone() })], eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "Coordsys" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod }), comment: com, info: mod_info }, tail: rest }, context, res, p) => {
                    let mut res = (*res).clone();
                    let true = (isLayerAnnInList(&(listAppend(res.clone(), rest.clone())))) else { return Err("pattern mismatch") };
                    res = metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Coordsys") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() }), res.clone());
                    res = transformClassAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    (res, _) = List::deleteMemberOnTrue(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Coordsys") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() }), res.clone(), &fnptr!(valueEq, _, _))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "Coordsys" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod }), comment: com, info: mod_info }, tail: rest }, context, res, p) => {
                    let mut coord: metamodelica::Ref<Absyn::ElementArg>;
                    let mut res = (*res).clone();
                    res = metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Coordsys") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() }), res.clone());
                    coord = getCoordSysAnn(&(listAppend(res.clone(), rest.clone())), p.clone())?;
                    res = metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Diagram") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: list![coord.clone()], eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), comment: None, info: mod_info.clone() }), metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Icon") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: list![coord.clone()], eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), comment: None, info: mod_info.clone() }), res.clone()));
                    res = transformClassAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    (res, _) = List::deleteMemberOnTrue(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Coordsys") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() }), res.clone(), &fnptr!(valueEq, _, _))?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "extent" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: x1, tail: Deref @ metamodelica::ListNode::Cons { head: y1, tail: Deref @ metamodelica::ListNode::Nil } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: x2, tail: Deref @ metamodelica::ListNode::Cons { head: y2, tail: Deref @ metamodelica::ListNode::Nil } }, tail: Deref @ metamodelica::ListNode::Nil } } }, info } }), comment: com, info: mod_info }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Coordsys", tail: _ }, res, p) => {
                    let mut res = (*res).clone();
                    res = metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("extent") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![x1.clone(), y1.clone()] }), metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![x2.clone(), y2.clone()] })] }), info: info.clone() }) })), comment: com.clone(), info: mod_info.clone() }), res.clone());
                    res = transformClassAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "grid" }, .. }, tail: rest }, context, res, p) => {
                    let mut res = (*res).clone();
                    res = transformClassAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "component" }, .. }, tail: rest }, context, res, p) => {
                    let mut res = (*res).clone();
                    res = transformClassAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Window" }, .. }, tail: rest }, context, res, p) => {
                    let mut res = (*res).clone();
                    res = transformClassAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Terminal" }, .. }, tail: rest }, context, res, p) => {
                    let mut res = (*res).clone();
                    res = transformClassAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: arg, tail: rest }, context, res, p) => {
                    let mut res = (*res).clone();
                    res = transformClassAnnList(metamodelica::AsArg::as_arg(&rest), context.clone(), res.clone(), p.clone())?;
                    Ok(metamodelica::cons(arg.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outArgs)
}

fn isLayerAnnInList<'__b>(mut inList: &'__b metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inList {
            Deref @ metamodelica::ListNode::Nil => {
                return false
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Diagram" }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Icon" }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut res: bool;
                { inList = rest; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getCoordSysAnn(
    mut inArgs: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inProgram: Absyn::Program,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut coordSys: metamodelica::Ref<Absyn::ElementArg>;
    coordSys = 'mc: {
        let __mc_input = (&**inArgs, inProgram);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("coordinateSystem") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: list![metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("extent") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::INTEGER { value: -100 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: -100 })] }), metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 100 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 100 })] })] }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() })], eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), comment: None, info: Absyn::dummyInfo.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "Coordsys" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod }), comment: com, info }, tail: _ }, p) => {
                    let mut args = (*args).clone();
                    args = transformClassAnnList(metamodelica::AsArg::as_arg(&args), metamodelica::cons(literal!("Coordsys"), metamodelica::nil()), metamodelica::nil(), p.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("coordinateSystem") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: eqMod.clone() })), comment: com.clone(), info: info.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, p) => {
                    let mut res: metamodelica::Ref<Absyn::ElementArg>;
                    res = getCoordSysAnn(metamodelica::AsArg::as_arg(&rest), p.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(coordSys)
}

fn transAnnLstToCalls(
    mut inArgs: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inCon: Context,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Exp>>> {
    let mut outArgs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    outArgs = 'mc: {
        let __mc_input = (&**inArgs, inCon);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Line" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), .. }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Layer", tail: c }) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut argRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut c = (*c).clone();
                    c = addContext(context.clone(), literal!("Line"));
                    argRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&args), c.clone())?;
                    ::match_deref::match_deref! { match &(List::select1(argRes.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::NamedArg>, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(nameArgWithName(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::NamedArg>, ArcStr) -> Result<bool> + 'static>), literal!("color"))?) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    restRes = transAnnLstToCalls(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::Exp::CALL { function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("Line"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("color"), argValue: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 0 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 0 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 255 })] }) }), argRes.clone()) }), typeVars: metamodelica::nil() }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: n }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), .. }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Layer", tail: c }) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut argRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut c = (*c).clone();
                    c = addContext(context.clone(), n.clone());
                    let true = (isLinebasedGraphic(metamodelica::AsArg::as_arg(&c))) else { return Err("pattern mismatch") };
                    argRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&args), c.clone())?;
                    ::match_deref::match_deref! { match &(List::select1(argRes.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::NamedArg>, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(nameArgWithName(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::NamedArg>, ArcStr) -> Result<bool> + 'static>), literal!("lineColor"))?) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    restRes = transAnnLstToCalls(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::Exp::CALL { function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: n.clone(), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("lineColor"), argValue: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 0 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 0 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 255 })] }) }), argRes.clone()) }), typeVars: metamodelica::nil() }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: n }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), .. }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Layer", tail: c }) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut argRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut c = (*c).clone();
                    c = addContext(context.clone(), n.clone());
                    argRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&args), c.clone())?;
                    restRes = transAnnLstToCalls(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::Exp::CALL { function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: n.clone(), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: argRes.clone() }), typeVars: metamodelica::nil() }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, context) => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    res = transAnnLstToCalls(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outArgs)
}

fn nameArgWithName(mut narg: &metamodelica::Ref<Absyn::NamedArg>, mut argName: &ArcStr) -> bool {
    let mut res: bool;
    res = (match &**narg {
        Absyn::NamedArg {
            argName: name,
            argValue: _,
        } => {
            res = metamodelica::stringEq(&name, &argName);
            res
        }
    });
    res
}

fn transAnnLstToNamedArgs(
    mut inArgs: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inCon: Context,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>> {
    let mut outArgs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
    outArgs = 'mc: {
        let __mc_input = (&**inArgs, inCon);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "extent" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: x1, tail: Deref @ metamodelica::ListNode::Cons { head: y1, tail: Deref @ metamodelica::ListNode::Nil } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: x2, tail: Deref @ metamodelica::ListNode::Cons { head: y2, tail: Deref @ metamodelica::ListNode::Nil } }, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, .. }), .. }, tail: rest }, context) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("extent"), argValue: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![x1.clone(), y1.clone()] }), metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![x2.clone(), y2.clone()] })] }) }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "style" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), .. }, tail: rest }, context) => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut argRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut args = (*args).clone();
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    args = cleanStyleAttrs(args.clone(), metamodelica::nil(), context.clone())?;
                    argRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&args), context.clone())?;
                    res = listAppend(argRes.clone(), restRes.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "color" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), .. }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Text", tail: _ }) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut color1: i32;
                    let mut color2: i32;
                    let mut color3: i32;
                    (color1, color2, color3) = getMappedColor(x.clone())?;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("fillColor"), argValue: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color1 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color2 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color3 })] }) }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "color" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), .. }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Line", tail: _ }) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut color1: i32;
                    let mut color2: i32;
                    let mut color3: i32;
                    (color1, color2, color3) = getMappedColor(x.clone())?;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("color"), argValue: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color1 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color2 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color3 })] }) }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "color" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), .. }, tail: rest }, context) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut color1: i32;
                    let mut color2: i32;
                    let mut color3: i32;
                    (color1, color2, color3) = getMappedColor(x.clone())?;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("lineColor"), argValue: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color1 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color2 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color3 })] }) }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), .. }, tail: rest }, context) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut color1: i32;
                    let mut color2: i32;
                    let mut color3: i32;
                    (color1, color2, color3) = getMappedColor(x.clone())?;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("fillColor"), argValue: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color1 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color2 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: color3 })] }) }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pattern" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), .. }, tail: rest }, context) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut val: ArcStr;
                    val = (patternMapList).get(x.clone() + 1)?;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("pattern"), argValue: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: literal!("LinePattern"), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: val.clone(), subscripts: metamodelica::nil() }) }) }) }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillPattern" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), .. }, tail: rest }, context) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut val: ArcStr;
                    val = (fillPatternMapList).get(x.clone() + 1)?;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("fillPattern"), argValue: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: literal!("FillPattern"), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: val.clone(), subscripts: metamodelica::nil() }) }) }) }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "thickness" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), .. }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Line", tail: _ }) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut s: ArcStr;
                    let mut thick: metamodelica::Real;
                    thick = (thicknessMapList).get(x.clone())?;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    s = realString(thick);
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("thickness"), argValue: metamodelica::Ref::new(Absyn::Exp::REAL { value: s.clone() }) }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "thickness" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), .. }, tail: rest }, context) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut s: ArcStr;
                    let mut thick: metamodelica::Real;
                    thick = (thicknessMapList).get(x.clone())?;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    s = realString(thick);
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("lineThickness"), argValue: metamodelica::Ref::new(Absyn::Exp::REAL { value: s.clone() }) }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "gradient" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), .. }, tail: rest }, context) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut val: ArcStr;
                    val = (gradientMapList).get(x.clone() + 1)?;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("fillPattern"), argValue: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: literal!("FillPattern"), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: val.clone(), subscripts: metamodelica::nil() }) }) }) }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), .. }, tail: rest }, context) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("smooth"), argValue: exp.clone() }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "arrow" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), .. }, tail: rest }, context) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut arrows: metamodelica::List<ArcStr>;
                    let mut val1: ArcStr;
                    let mut val2: ArcStr;
                    arrows = (arrowMapList).get(x.clone() + 1)?;
                    val1 = (arrows).get(1)?;
                    val2 = (arrows).get(2)?;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("arrow"), argValue: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: literal!("Arrow"), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: val1.clone(), subscripts: metamodelica::nil() }) }) }), metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: literal!("Arrow"), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: val2.clone(), subscripts: metamodelica::nil() }) }) })] }) }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "textStyle" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), .. }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Text", tail: _ }) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("textStyle"), argValue: exp.clone() }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "font" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), .. }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Text", tail: _ }) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("font"), argValue: exp.clone() }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "string" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), .. }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Text", tail: _ }) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("textString"), argValue: exp.clone() }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "name" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), .. }, tail: rest }, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Bitmap", tail: _ }) => {
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("fileName"), argValue: exp.clone() }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "points" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: expMatrix }, .. }, .. }), .. }, tail: rest }, context) => {
                    let mut expLst: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut restRes: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    expLst = List::map(expMatrix.clone(), &fnptr!(matrixToArray, metamodelica::List<metamodelica::Ref<Absyn::Exp>>))?;
                    restRes = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("points"), argValue: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: expLst.clone() }) }), restRes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, context) => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    res = transAnnLstToNamedArgs(metamodelica::AsArg::as_arg(&rest), context.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outArgs)
}

fn cleanStyleAttrs(
    mut inArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut resultList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inCon: Context,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut outArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
    outArgs = 'mc: {
        let __mc_input = inCon.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                context => {
                    let mut outArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = outArgs.clone();
                    let true = (isLinebasedGraphic(metamodelica::AsArg::as_arg(&context))) else { return Err("pattern mismatch") };
                    ::match_deref::match_deref! { match &(List::select(inArgs.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::ElementArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isLineColorModifier(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ElementArg>) -> Result<bool> + 'static>))?) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    outArgs = cleanStyleAttrs2(metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("lineColor") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 0 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 0 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 255 })] }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() }), inArgs.clone()), resultList.clone(), context.clone())?;
                    Ok((outArgs.clone(), outArgs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outArgs = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                context => {
                    let mut outArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = outArgs.clone();
                    let true = (isLineGraphic(metamodelica::AsArg::as_arg(&context))) else { return Err("pattern mismatch") };
                    ::match_deref::match_deref! { match &(List::select(inArgs.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::ElementArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isLineColorModifier(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ElementArg>) -> Result<bool> + 'static>))?) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    outArgs = cleanStyleAttrs2(metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("color") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 0 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 0 }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 255 })] }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() }), inArgs.clone()), resultList.clone(), context.clone())?;
                    Ok((outArgs.clone(), outArgs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outArgs = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = outArgs.clone();
                    outArgs = cleanStyleAttrs2(inArgs.clone(), resultList.clone(), inCon.clone())?;
                    Ok((outArgs.clone(), outArgs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outArgs = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outArgs)
}

fn isLineColorModifier(mut arg: &metamodelica::Ref<Absyn::ElementArg>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match arg {
        Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "color" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: _, eqMod: _ }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn isStyleModifier(mut arg: &metamodelica::Ref<Absyn::ElementArg>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match arg {
        Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "style" }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn isLinebasedGraphic(mut context: &Context) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match context {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "Rectangle", tail: _ } => true,
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "Ellipse", tail: _ } => true,
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "Polygon", tail: _ } => true,
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "Text", tail: _ } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn isLineGraphic(mut context: &Context) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match context {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "Line", tail: _ } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn cleanStyleAttrs2(
    mut inArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inResultList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inCon: Context,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inArgs, inResultList, inCon)) {
            (Deref @ metamodelica::ListNode::Nil, resultList, _) => {
                return Ok(resultList.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "color" }, .. }, tail: rest }, resultList, context) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Rectangle", tail: _ }) if (!(isGradientInList(&(listAppend(rest.clone(), resultList.clone())))) && !(isFillPatternInList(&(listAppend(rest.clone(), resultList.clone()))))) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = insertFillPatternInList(resultList.clone());
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Ellipse", tail: _ }) if (!(isGradientInList(&(listAppend(rest.clone(), resultList.clone())))) && !(isFillPatternInList(&(listAppend(rest.clone(), resultList.clone()))))) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = insertFillPatternInList(resultList.clone());
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Polygon", tail: _ }) if (!(isGradientInList(&(listAppend(rest.clone(), resultList.clone())))) && !(isFillPatternInList(&(listAppend(rest.clone(), resultList.clone()))))) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = insertFillPatternInList(resultList.clone());
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Rectangle", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Ellipse", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Polygon", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pattern" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Rectangle", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pattern" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Ellipse", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pattern" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Polygon", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pattern" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Line", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillPattern" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Rectangle", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillPattern" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Ellipse", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillPattern" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Polygon", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "thickness" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Bitmap", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "thickness" }, .. }, tail: rest }, resultList, context) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "gradient" }, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: 0 }, .. }, .. }), .. }, tail: rest }, resultList, context) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "gradient" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Rectangle", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut rest = (*rest).clone();
                let mut resultList = (*resultList).clone();
                rest = removeFillPatternInList(metamodelica::AsArg::as_arg(&rest));
                resultList = removeFillPatternInList(metamodelica::AsArg::as_arg(&resultList));
                rest = setDefaultLineInList(metamodelica::AsArg::as_arg(&rest));
                resultList = setDefaultLineInList(metamodelica::AsArg::as_arg(&resultList));
                (rest, resultList) = setDefaultFillColor(rest.clone(), resultList.clone());
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "gradient" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Ellipse", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut rest = (*rest).clone();
                let mut resultList = (*resultList).clone();
                rest = removeFillPatternInList(metamodelica::AsArg::as_arg(&rest));
                resultList = removeFillPatternInList(metamodelica::AsArg::as_arg(&resultList));
                rest = setDefaultLineInList(metamodelica::AsArg::as_arg(&rest));
                resultList = setDefaultLineInList(metamodelica::AsArg::as_arg(&resultList));
                (rest, resultList) = setDefaultFillColor(rest.clone(), resultList.clone());
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Polygon", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Line", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "arrow" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Line", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "textStyle" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Text", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "font" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "Text", tail: _ }) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut resultList = (*resultList).clone();
                resultList = List::appendElt(arg.clone(), resultList.clone());
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, resultList, context) => {
                let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                { (inArgs, inResultList, inCon) = (rest.clone(), resultList.clone(), context.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn insertFillPatternInList(
    mut inArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> {
    let mut outArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    outArgs = (::match_deref::match_deref! { match &(inArgs) {
        lst => {
            let mut lst = (*lst).clone();
            lst = metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("fillPattern") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 1 }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() }), lst.clone());
            lst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outArgs
}

fn isGradientInList<'__b>(mut inArgs: &'__b metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inArgs {
            Deref @ metamodelica::ListNode::Nil => {
                return false
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "gradient" }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut res: bool;
                { inArgs = rest; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn isFillPatternInList<'__b>(mut inArgs: &'__b metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inArgs {
            Deref @ metamodelica::ListNode::Nil => {
                return false
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillPattern" }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut res: bool;
                { inArgs = rest; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn removeFillPatternInList(
    mut inList: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> {
    let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    outList = (::match_deref::match_deref! { match inList {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillPattern" }, .. }, tail: rest } => {
            rest.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: arg, tail: rest } => {
            let mut lst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
            lst = removeFillPatternInList(rest);
            metamodelica::cons(arg.clone(), lst)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outList
}

fn setDefaultFillColor(
    mut oldList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut transformedList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> (
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) {
    let mut oList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut tList: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    oList = oldList.clone();
    tList = transformedList.clone();
    if !(isFillColorInList(&(listAppend(oldList, transformedList)))) {
        tList = metamodelica::cons(
            metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION {
                finalPrefix: false,
                eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH,
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("fillColor"),
                }),
                modification: Some(metamodelica::Ref::new(Absyn::Modification {
                    elementArgLst: metamodelica::nil(),
                    eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD {
                        exp: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 3 }),
                        info: Absyn::dummyInfo.clone(),
                    }),
                })),
                comment: None,
                info: Absyn::dummyInfo.clone(),
            }),
            tList,
        );
    }
    (oList, tList)
}

fn isFillColorInList<'__b>(mut inList: &'__b metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inList {
            Deref @ metamodelica::ListNode::Nil => {
                return false
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { inList = rest; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn setDefaultLineInList<'__b>(
    mut inList: &'__b metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inList {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "thickness" }, modification: Some(Deref @ Absyn::Modification { .. }), .. }, tail: rest } => {
                let mut lst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                { inList = rest; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pattern" }, modification: Some(Deref @ Absyn::Modification { .. }), .. }, tail: rest } => {
                let mut lst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                { inList = rest; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: fi, eachPrefix: e, path: Deref @ Absyn::Path::IDENT { name: Deref @ "color" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), comment: com, info }, tail: rest } => {
                let mut lst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                lst = setDefaultLineInList(rest);
                return metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("color") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 0 }), info: Absyn::dummyInfo.clone() }) })), comment: com.clone(), info: info.clone() }), lst)
            },
            Deref @ metamodelica::ListNode::Cons { head: arg, tail: rest } => {
                let mut lst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                lst = setDefaultLineInList(rest);
                return metamodelica::cons(arg.clone(), lst)
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getMappedColor(mut inColor: i32) -> Result<(i32, i32, i32)> {
    let mut color1: i32;
    let mut color2: i32;
    let mut color3: i32;
    (color1, color2, color3) = (match inColor {
        mut color => {
            let mut rcol: rgbColor;
            rcol = (colorMapList).get(color + 1)?;
            color1 = (rcol).get(1)?;
            color2 = (rcol).get(2)?;
            color3 = (rcol).get(3)?;
            (color1, color2, color3)
        }
    });
    Ok((color1, color2, color3))
}

fn matrixToArray(mut inLst: metamodelica::List<metamodelica::Ref<Absyn::Exp>>) -> metamodelica::Ref<Absyn::Exp> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    outExp = metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: inLst });
    outExp
}

/*
protected function getValueFromIntExp

  input Absyn.Exp intExpr;
  output Integer value;
algorithm
  value := match(intExpr)
    local
      Integer val;
    case(Absyn.INTEGER(value = val))
      then val;

    case(Absyn.UNARY(exp = Absyn.INTEGER(value = val)))
      then (-val);
  end match;
end getValueFromIntExp;

protected function getValueFromRealExp
  input Absyn.Exp realExpr;
  output Real value;
algorithm
  value := match(realExpr)
    local
      Real val;
    case(Absyn.REAL(value = val))
      then val;
    case(Absyn.UNARY(exp = Absyn.REAL(value = val)))
      then -val;
  end match;
end getValueFromRealExp;  */
fn getValueFromExp(mut expr: &metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Real> {
    let mut value: metamodelica::Real;
    value = (::match_deref::match_deref! { match expr {
        Deref @ Absyn::Exp::REAL { value: realVal } => {
            stringReal(realVal.clone())?
        },
        Deref @ Absyn::Exp::UNARY { exp: Deref @ Absyn::Exp::REAL { value: realVal }, .. } => {
            -(stringReal(realVal.clone())?)
        },
        Deref @ Absyn::Exp::INTEGER { value: intVal } => {
            intReal(intVal.clone())
        },
        Deref @ Absyn::Exp::UNARY { exp: Deref @ Absyn::Exp::INTEGER { value: intVal }, .. } => {
            -(intReal(intVal.clone()))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(value)
}

fn addContext(mut inList: metamodelica::List<ArcStr>, mut newCon: ArcStr) -> metamodelica::List<ArcStr> {
    let mut outList: metamodelica::List<ArcStr>;
    outList = (::match_deref::match_deref! { match &((inList, newCon)) {
        (strLst, r#str) => {
            metamodelica::cons(r#str.clone(), strLst.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outList
}

pub type Context = metamodelica::List<ArcStr>;

pub type rgbColor = metamodelica::List<i32>;

pub type rgbColorMapList = metamodelica::List<metamodelica::List<i32>>;

pub(crate) static colorMapList: std::sync::LazyLock<metamodelica::List<metamodelica::List<i32>>> =
    std::sync::LazyLock::new(|| {
        list![
            list![0, 0, 0],
            list![255, 0, 0],
            list![0, 255, 0],
            list![0, 0, 255],
            list![0, 255, 255],
            list![255, 0, 255],
            list![255, 255, 0],
            list![255, 255, 255],
            list![192, 192, 192],
            list![160, 160, 160],
            list![128, 128, 128],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![235, 235, 235],
            list![240, 255, 255],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![0, 0, 0],
            list![255, 0, 0],
            list![191, 0, 0],
            list![255, 127, 127],
            list![223, 159, 159],
            list![255, 127, 0],
            list![191, 95, 0],
            list![255, 191, 127],
            list![223, 191, 159],
            list![255, 255, 0],
            list![191, 191, 0],
            list![255, 255, 127],
            list![223, 223, 159],
            list![127, 255, 0],
            list![95, 191, 0],
            list![191, 255, 127],
            list![191, 223, 159],
            list![0, 255, 0],
            list![0, 191, 0],
            list![127, 255, 127],
            list![159, 223, 159],
            list![0, 255, 127],
            list![0, 191, 95],
            list![127, 255, 191],
            list![159, 223, 191],
            list![0, 255, 255],
            list![0, 191, 191],
            list![127, 255, 255],
            list![159, 223, 223],
            list![0, 127, 255],
            list![0, 95, 191],
            list![127, 191, 255],
            list![159, 191, 223],
            list![0, 0, 255],
            list![0, 0, 191],
            list![127, 127, 255],
            list![159, 159, 223],
            list![127, 0, 255],
            list![95, 0, 191],
            list![191, 127, 255],
            list![191, 159, 223],
            list![255, 0, 255],
            list![191, 0, 191],
            list![255, 127, 255],
            list![223, 159, 223],
            list![255, 0, 127],
            list![191, 0, 95],
            list![255, 127, 191],
            list![223, 159, 191]
        ]
    });

pub(crate) const None_: &'static str = "None";

pub(crate) const Solid: &'static str = "Solid";

pub(crate) const Horizontal: &'static str = "Horizontal";

pub(crate) const Vertical: &'static str = "Vertical";

pub(crate) const Cross: &'static str = "Cross";

pub(crate) const Forward: &'static str = "Forward";

pub(crate) const Backward: &'static str = "Backward";

pub(crate) const CrossDiag: &'static str = "CrossDiag";

pub(crate) const HorizontalCylinder: &'static str = "HorizontalCylinder";

pub(crate) const VerticalCylinder: &'static str = "VerticalCylinder";

pub(crate) const Sphere: &'static str = "Sphere";

pub(crate) const Dash: &'static str = "Dash";

pub(crate) const Dot: &'static str = "Dot";

pub(crate) const DashDot: &'static str = "DashDot";

pub(crate) const DashDotDot: &'static str = "DashDotDot";

pub(crate) const Filled: &'static str = "Filled";

pub(crate) const Half: &'static str = "Half";

pub(crate) static fillPatternMapList: std::sync::LazyLock<metamodelica::List<ArcStr>> =
    std::sync::LazyLock::new(|| {
        list![
            arcstr::literal!(None_),
            arcstr::literal!(Solid),
            arcstr::literal!(None_),
            arcstr::literal!(None_),
            arcstr::literal!(None_),
            arcstr::literal!(Horizontal),
            arcstr::literal!(Vertical),
            arcstr::literal!(Forward),
            arcstr::literal!(Backward),
            arcstr::literal!(Cross),
            arcstr::literal!(CrossDiag)
        ]
    });

pub(crate) static gradientMapList: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        arcstr::literal!(None_),
        arcstr::literal!(VerticalCylinder),
        arcstr::literal!(HorizontalCylinder),
        arcstr::literal!(Sphere)
    ]
});

pub(crate) static patternMapList: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        arcstr::literal!(None_),
        arcstr::literal!(Solid),
        arcstr::literal!(Dash),
        arcstr::literal!(Dot),
        arcstr::literal!(DashDot),
        arcstr::literal!(DashDotDot)
    ]
});

pub(crate) static thicknessMapList: std::sync::LazyLock<metamodelica::List<metamodelica::Real>> =
    std::sync::LazyLock::new(|| {
        list![
            metamodelica::OrderedFloat(0.25_f64),
            metamodelica::OrderedFloat(0.5_f64),
            metamodelica::OrderedFloat(0.0_f64),
            metamodelica::OrderedFloat(1.0_f64)
        ]
    });

pub(crate) static arrowMapList: std::sync::LazyLock<metamodelica::List<metamodelica::List<ArcStr>>> =
    std::sync::LazyLock::new(|| {
        list![
            list![arcstr::literal!(None_), arcstr::literal!(None_)],
            list![arcstr::literal!(None_), arcstr::literal!(Filled)],
            list![arcstr::literal!(Filled), arcstr::literal!(None_)],
            list![arcstr::literal!(Filled), arcstr::literal!(Filled)],
            list![arcstr::literal!(None_), arcstr::literal!(Half)]
        ]
    });

fn fixPaths(
    mut inPath1: metamodelica::Ref<Absyn::Path>,
    mut inPath2: metamodelica::Ref<Absyn::Path>,
) -> metamodelica::Ref<Absyn::Path> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = 'mc: {
        let __mc_input = (inPath1, inPath2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ip1, ip2) => {
                    let mut p1: metamodelica::Ref<Absyn::Path>;
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    let mut out: metamodelica::Ref<Absyn::Path>;
                    str1 = AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&ip1));
                    str2 = AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&ip2));
                    let false = (stringEq(&str1, &str2)) else { return Err("pattern mismatch") };
                    p1 = AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&ip1))?;
                    out = fixPaths(p1.clone(), ip2.clone());
                    Ok(out.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ip1, ip2) => {
                    let mut p1: metamodelica::Ref<Absyn::Path>;
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    let mut out: metamodelica::Ref<Absyn::Path>;
                    str1 = AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&ip1));
                    str2 = AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&ip2));
                    let true = (stringEq(&str1, &str2)) else { return Err("pattern mismatch") };
                    p1 = AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&ip1))?;
                    out = AbsynUtil::joinPaths(p1.clone(), ip2.clone())?;
                    Ok(out.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inPath2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outPath
}
