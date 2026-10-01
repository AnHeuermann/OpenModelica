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

use crate::CevalScriptBackend;
use crate::SimCodeMain;
use openmodelica_ast::Absyn;
use openmodelica_ast::Absyn::Path;
use openmodelica_backend::SymbolTable;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_nf_api::NFInstanceAPI;
use openmodelica_nf_frontend::NFAttributes;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFCeval as Ceval;
use openmodelica_nf_frontend::NFClass as Class;
use openmodelica_nf_frontend::NFClassTree::ClassTree;
use openmodelica_nf_frontend::NFComponent as Component;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFConnectBreakTree;
use openmodelica_nf_frontend::NFConnection as Connection;
use openmodelica_nf_frontend::NFConvertDAE as ConvertDAE;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFEquation as Equation;
use openmodelica_nf_frontend::NFEvalConstants as EvalConstants;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFlatModel as FlatModel;
use openmodelica_nf_frontend::NFFlatten as Flatten;
use openmodelica_nf_frontend::NFFlatten::FunctionTree;
use openmodelica_nf_frontend::NFImport as Import;
use openmodelica_nf_frontend::NFInst as Inst;
use openmodelica_nf_frontend::NFInst::InstSettings;
use openmodelica_nf_frontend::NFInstContext as InstContext;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFInstNode::InstNodeType;
use openmodelica_nf_frontend::NFInstUtil as InstUtil;
use openmodelica_nf_frontend::NFLookup as Lookup;
use openmodelica_nf_frontend::NFModifier::Modifier;
use openmodelica_nf_frontend::NFModifier::ModifierScope;
use openmodelica_nf_frontend::NFPackage as Package;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFPrefixes::Purity;
use openmodelica_nf_frontend::NFPrefixes::Variability;
use openmodelica_nf_frontend::NFRestriction as Restriction;
use openmodelica_nf_frontend::NFScalarize as Scalarize;
use openmodelica_nf_frontend::NFSections as Sections;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFSimplifyModel as SimplifyModel;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFTyping as Typing;
use openmodelica_nf_frontend::NFUnitCheck as UnitCheck;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_nf_frontend::NFVerifyModel as VerifyModel;
use openmodelica_simcode_types::SimCode;
use openmodelica_util::Config;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExecStat::execStatReset;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Global;
use openmodelica_util::JSON;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub(crate) const ANNOTATION_CONTEXT: i32 = intBitOr(InstContext::RELAXED, InstContext::ANNOTATION);

pub(crate) const INST_API_ANNOTATION_CONTEXT: i32 = intBitOr(ANNOTATION_CONTEXT, InstContext::INSTANCE_API);

pub(crate) const FAST_CONTEXT: i32 = intBitOr(InstContext::RELAXED, InstContext::FAST_LOOKUP);

pub(crate) fn evaluateAnnotation(
    mut absynProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Path>,
    mut inAnnotation: &metamodelica::Ref<Absyn::Annotation>,
) -> Result<ArcStr> {
    let mut outString: ArcStr = literal!("");
    let mut b: bool;
    let mut s: bool;
    b = FlagsUtil::set(Flags::SCODE_INST.clone(), true)?;
    s = FlagsUtil::set(Flags::NF_SCALARIZE.clone(), true)?;
    match '__try0: {
        outString = unwrap_break_err!(evaluateAnnotation_dispatch(absynProgram.clone(), classPath.clone(), inAnnotation, false), '__try0);
        unwrap_break_err!(FlagsUtil::set(Flags::SCODE_INST.clone(), b), '__try0);
        unwrap_break_err!(FlagsUtil::set(Flags::NF_SCALARIZE.clone(), s), '__try0);
        Ok::<_, &'static str>((outString.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outString = __try0_o0;
        }
        Err(__try0_err) => {
            FlagsUtil::set(Flags::SCODE_INST.clone(), b)?;
            FlagsUtil::set(Flags::NF_SCALARIZE.clone(), s)?;
            return Err(__try0_err);
        }
    }
    Ok(outString)
}

fn evaluateAnnotation_dispatch(
    mut absynProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Path>,
    mut inAnnotation: &metamodelica::Ref<Absyn::Annotation>,
    mut addAnnotationName: bool,
) -> Result<ArcStr> {
    let mut outString: ArcStr = literal!("");
    let mut top: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::EMPTY_NODE);
    let mut inst_cls: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::EMPTY_NODE);
    let mut anncls: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::EMPTY_NODE);
    let mut inst_anncls: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::EMPTY_NODE);
    let mut name: ArcStr = arcstr::literal!("");
    let mut annName: ArcStr;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut el: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
    let mut stringLst: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut absynExp: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let mut exp: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
    let mut save: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
    let mut info: SourceInfo;
    let mut r#mod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut stripped_mod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
    let mut graphics_mod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
    let mut eqmod: metamodelica::Ref<Absyn::EqMod>;
    let mut smod: metamodelica::Ref<SCode::Mod> = metamodelica::Ref::new(SCode::Mod::NOMOD);
    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut ty: metamodelica::Ref<Type::NFType> = metamodelica::Ref::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    stringLst = metamodelica::nil();
    let __arc1 = &(*inAnnotation);
    let Absyn::ANNOTATION { elementArgs: __pa0 } = &**__arc1;
    el = metamodelica::Own::own(__pa0);
    for mut e in &*el.clone().reverse() {
        let mut e = e.clone();
        e = AbsynUtil::createChoiceArray(e)?;
        r#str = 'mc: {
            let __mc_input = &*e;
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: annName }, modification: Some(Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::ListNode::Nil, eqMod: eqmod @ Deref @ Absyn::EqMod::EQMOD { exp: absynExp, .. } }), info, .. } => {
                        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp.clone();
                        let mut inst_cls: metamodelica::Ref<InstNode::InstNode> = inst_cls.clone();
                        let mut name: ArcStr = name.clone();
                        let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>> = program.clone();
                        let mut r#str: ArcStr = r#str.clone();
                        let mut top: metamodelica::Ref<InstNode::InstNode> = top.clone();
                        let mut ty: metamodelica::Ref<Type::NFType> = ty.clone();
                        let mut var: Variability = var.clone();
                        if AbsynUtil::onlyLiteralsInEqMod(metamodelica::AsArg::as_arg(&eqmod))? {
                            (program, top) = NFInstanceAPI::mkTop(absynProgram.clone(), scodeFor(&absynProgram)?, metamodelica::AsArg::as_arg(&annName))?;
                            inst_cls = top.clone();
                        } else {
                            (program, name, inst_cls) = frontEndFront(absynProgram.clone(), classPath.clone())?;
                        }
                        exp = Inst::instExp(absynExp.clone(), &inst_cls, ANNOTATION_CONTEXT.clone(), metamodelica::AsArg::as_arg(&info))?;
                        (exp, ty, var, _) = Typing::typeExp(exp.clone(), ANNOTATION_CONTEXT.clone(), metamodelica::AsArg::as_arg(&info), false)?;
                        exp = SimplifyExp::simplify(exp.clone(), false)?;
                        r#str = Expression::toString(exp.clone())?;
                        Ok((stringAppendList(list![annName.clone(), literal!("="), r#str.clone()]), exp.clone(), inst_cls.clone(), name.clone(), program.clone(), r#str.clone(), top.clone(), ty.clone(), var.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                exp = __wb0;
                inst_cls = __wb1;
                name = __wb2;
                program = __wb3;
                r#str = __wb4;
                top = __wb5;
                ty = __wb6;
                var = __wb7;
                break 'mc __v;
            }
            if let Ok((
                __v,
                __wb0,
                __wb1,
                __wb2,
                __wb3,
                __wb4,
                __wb5,
                __wb6,
                __wb7,
                __wb8,
                __wb9,
                __wb10,
                __wb11,
                __wb12,
                __wb13,
                __wb14,
                __wb15,
            )) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: annName }, modification: Some(Deref @ Absyn::Modification { elementArgLst: r#mod, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } }), info, .. } => {
                        let mut absynExp: metamodelica::Ref<Absyn::Exp> = absynExp.clone();
                        let mut anncls: metamodelica::Ref<InstNode::InstNode> = anncls.clone();
                        let mut dae: DAE::DAElist = dae.clone();
                        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp.clone();
                        let mut graphics_mod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = graphics_mod.clone();
                        let mut inst_anncls: metamodelica::Ref<InstNode::InstNode> = inst_anncls.clone();
                        let mut inst_cls: metamodelica::Ref<InstNode::InstNode> = inst_cls.clone();
                        let mut name: ArcStr = name.clone();
                        let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>> = program.clone();
                        let mut save: metamodelica::Ref<Expression::NFExpression> = save.clone();
                        let mut smod: metamodelica::Ref<SCode::Mod> = smod.clone();
                        let mut r#str: ArcStr = r#str.clone();
                        let mut stripped_mod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = stripped_mod.clone();
                        let mut top: metamodelica::Ref<InstNode::InstNode> = top.clone();
                        let mut ty: metamodelica::Ref<Type::NFType> = ty.clone();
                        let mut var: Variability = var.clone();
                        if AbsynUtil::onlyLiteralsInAnnotationMod(metamodelica::AsArg::as_arg(&r#mod)) {
                            (program, top) = NFInstanceAPI::mkTop(absynProgram.clone(), scodeFor(&absynProgram)?, metamodelica::AsArg::as_arg(&annName))?;
                            inst_cls = top.clone();
                        } else {
                            (program, name, inst_cls) = frontEndFront(absynProgram.clone(), classPath.clone())?;
                        }
                        (stripped_mod, graphics_mod) = AbsynUtil::stripGraphicsAndInteractionModification(metamodelica::AsArg::as_arg(&r#mod))?;
                        smod = AbsynToSCode::translateMod(Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: stripped_mod.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, info.clone(), false)?;
                        (anncls, _) = Lookup::lookupClassName(metamodelica::Ref::new(Path::IDENT { name: annName.clone() }), inst_cls.clone(), ANNOTATION_CONTEXT.clone(), Absyn::dummyInfo.clone(), false)?;
                        inst_anncls = Inst::expand(anncls.clone(), ANNOTATION_CONTEXT.clone())?;
                        (inst_anncls, _) = Inst::instClass(inst_anncls.clone(), Modifier::create(&smod, annName.clone(), &(metamodelica::Ref::new(ModifierScope::ModifierScope::CLASS { name: annName.clone() })), inst_cls.clone(), 0)?, NFAttributes::DEFAULT_ATTR().clone(), true, 0, 0, inst_cls.clone(), ANNOTATION_CONTEXT.clone())?;
                        Inst::instExpressions(inst_anncls.clone(), &(inst_anncls.clone()), openmodelica_nf_frontend::NFSections::interned_EMPTY(), &(NFConnectBreakTree::new()), ANNOTATION_CONTEXT.clone(), &(Inst::DEFAULT_SETTINGS.clone()))?;
                        Inst::updateImplicitVariability(inst_anncls.clone(), Flags::isSet(Flags::EVAL_PARAM.clone())?, ANNOTATION_CONTEXT.clone())?;
                        dae = frontEndBack(inst_anncls.clone(), annName.clone(), false)?;
                        r#str = DAEUtil::getVariableBindingsStr(DAEUtil::daeElements(&dae))?;
                        if listMember(annName.clone(), list![literal!("Icon"), literal!("Diagram"), literal!("choices")]) && !((graphics_mod).is_empty()) {
                            if '__try0: {
                                let __pa1 = ::match_deref::match_deref! { match &(graphics_mod.clone()) {
                                            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: __pa1, .. }, .. }), .. }, tail: Deref @ metamodelica::ListNode::Nil } => __pa1.clone(),
                                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                                } };
                                absynExp = metamodelica::Own::own(__pa1);
                                exp = unwrap_break_err!(Inst::instExp(absynExp.clone(), &inst_cls, ANNOTATION_CONTEXT.clone(), metamodelica::AsArg::as_arg(&info)), '__try0);
                                (exp, ty, var, _) = unwrap_break_err!(Typing::typeExp(exp.clone(), ANNOTATION_CONTEXT.clone(), metamodelica::AsArg::as_arg(&info), false), '__try0);
                                save = exp.clone();
                                match '__try4: {
                                            exp = unwrap_break_err!(Ceval::evalExp(save.clone(), &(Ceval::noTarget().clone())), '__try4);
                                            Ok::<_, &'static str>((exp.clone(),))
                                } {
                                            Ok((__try4_o0,)) => {
                                                exp = __try4_o0;
                                            }
                                            Err(_) => {
                                                exp = unwrap_break_err!(EvalConstants::evaluateExp(save.clone(), info.clone()), '__try0);
                                            }
                                }
                                exp = unwrap_break_err!(SimplifyExp::simplify(exp.clone(), false), '__try0);
                                r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*unwrap_break_err!(Expression::toString(exp.clone()), '__try0)); ArcStr::from(__mm_s) };
                                Ok::<(), &'static str>(())
                            }.is_err() {
                            }
                        }
                        Ok((if (addAnnotationName) {stringAppendList(list![annName.clone(), literal!("("), r#str.clone(), literal!(")")])} else {r#str.clone()}, absynExp.clone(), anncls.clone(), dae.clone(), exp.clone(), graphics_mod.clone(), inst_anncls.clone(), inst_cls.clone(), name.clone(), program.clone(), save.clone(), smod.clone(), r#str.clone(), stripped_mod.clone(), top.clone(), ty.clone(), var.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                absynExp = __wb0;
                anncls = __wb1;
                dae = __wb2;
                exp = __wb3;
                graphics_mod = __wb4;
                inst_anncls = __wb5;
                inst_cls = __wb6;
                name = __wb7;
                program = __wb8;
                save = __wb9;
                smod = __wb10;
                r#str = __wb11;
                stripped_mod = __wb12;
                top = __wb13;
                ty = __wb14;
                var = __wb15;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: annName }, modification: None, info, .. } => {
                        let mut anncls: metamodelica::Ref<InstNode::InstNode> = anncls.clone();
                        let mut dae: DAE::DAElist = dae.clone();
                        let mut inst_anncls: metamodelica::Ref<InstNode::InstNode> = inst_anncls.clone();
                        let mut inst_cls: metamodelica::Ref<InstNode::InstNode> = inst_cls.clone();
                        let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>> = program.clone();
                        let mut r#str: ArcStr = r#str.clone();
                        let mut top: metamodelica::Ref<InstNode::InstNode> = top.clone();
                        (program, top) = NFInstanceAPI::mkTop(absynProgram.clone(), scodeFor(&absynProgram)?, metamodelica::AsArg::as_arg(&annName))?;
                        inst_cls = top.clone();
                        (anncls, _) = Lookup::lookupClassName(metamodelica::Ref::new(Path::IDENT { name: annName.clone() }), inst_cls.clone(), ANNOTATION_CONTEXT.clone(), Absyn::dummyInfo.clone(), false)?;
                        inst_anncls = Inst::instantiate(anncls.clone(), openmodelica_nf_frontend::NFModifier::Modifier::interned_NOMOD(), openmodelica_nf_frontend::NFInstNode::InstNode::interned_EMPTY_NODE(), ANNOTATION_CONTEXT.clone(), false)?;
                        Inst::instExpressions(inst_anncls.clone(), &(inst_anncls.clone()), openmodelica_nf_frontend::NFSections::interned_EMPTY(), &(NFConnectBreakTree::new()), ANNOTATION_CONTEXT.clone(), &(Inst::DEFAULT_SETTINGS.clone()))?;
                        Inst::updateImplicitVariability(inst_anncls.clone(), Flags::isSet(Flags::EVAL_PARAM.clone())?, ANNOTATION_CONTEXT.clone())?;
                        dae = frontEndBack(inst_anncls.clone(), annName.clone(), false)?;
                        r#str = DAEUtil::getVariableBindingsStr(DAEUtil::daeElements(&dae))?;
                        Ok((if (addAnnotationName) {stringAppendList(list![annName.clone(), literal!("("), r#str.clone(), literal!(")")])} else {r#str.clone()}, anncls.clone(), dae.clone(), inst_anncls.clone(), inst_cls.clone(), program.clone(), r#str.clone(), top.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                anncls = __wb0;
                dae = __wb1;
                inst_anncls = __wb2;
                inst_cls = __wb3;
                program = __wb4;
                r#str = __wb5;
                top = __wb6;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: annName }, info, .. } => {
                        let mut r#str: ArcStr = r#str.clone();
                        r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("error evaluating: annotation(")); __mm_s.push_str(&*Dump::unparseElementArgStr(e.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                        r#str = Util::escapeQuotes(r#str.clone())?;
                        Ok((stringAppendList(list![annName.clone(), literal!("(\""), r#str.clone(), literal!("\")")]), r#str.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                r#str = __wb0;
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        };
        stringLst = metamodelica::cons(r#str.clone(), stringLst);
    }
    outString = stringDelimitList(stringLst, literal!(", "));
    if Flags::isSet(Flags::EXEC_STAT.clone())? {
        execStat(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFApi.evaluateAnnotation_dispatch("));
                __mm_s.push_str(&*AbsynUtil::pathString(classPath, literal!("."), true, false)?);
                __mm_s.push_str(&*literal!(" annotation("));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(el, &Dump::unparseElementArgStr)?,
                    literal!(", "),
                ));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    Ok(outString)
}

pub(crate) fn evaluateAnnotations(
    mut absynProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Path>,
    mut inElements: &metamodelica::List<metamodelica::Ref<Absyn::Element>>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringLst: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut b: bool;
    let mut s: bool;
    b = FlagsUtil::set(Flags::SCODE_INST.clone(), true)?;
    s = FlagsUtil::set(Flags::NF_SCALARIZE.clone(), true)?;
    match '__try0: {
        outStringLst = unwrap_break_err!(evaluateAnnotations_dispatch(absynProgram.clone(), classPath.clone(), inElements), '__try0);
        unwrap_break_err!(FlagsUtil::set(Flags::SCODE_INST.clone(), b), '__try0);
        unwrap_break_err!(FlagsUtil::set(Flags::NF_SCALARIZE.clone(), s), '__try0);
        Ok::<_, &'static str>((outStringLst.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outStringLst = __try0_o0;
        }
        Err(__try0_err) => {
            FlagsUtil::set(Flags::SCODE_INST.clone(), b)?;
            FlagsUtil::set(Flags::NF_SCALARIZE.clone(), s)?;
            return Err(__try0_err);
        }
    }
    Ok(outStringLst)
}

fn evaluateAnnotations_dispatch(
    mut absynProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Path>,
    mut inElements: &metamodelica::List<metamodelica::Ref<Absyn::Element>>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringLst: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut r#str: ArcStr;
    let mut elArgs: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> = metamodelica::nil();
    let mut el: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> = metamodelica::nil();
    let mut stringLst: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut items: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    let mut cc: Option<metamodelica::Ref<Absyn::ConstrainClass>>;
    let mut anns: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut cmt: Option<metamodelica::Ref<Absyn::Comment>>;
    for mut i in &**inElements {
        elArgs = (::match_deref::match_deref! { match &(i.clone()) {
            Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: __esc_items, .. }, constrainClass: __esc_cc, .. } => {
                items = (*__esc_items).clone();
                cc = (*__esc_cc).clone();
                el = AbsynUtil::getAnnotationsFromItems(items.clone(), AbsynUtil::getAnnotationsFromConstraintClass(cc.clone()));
                listAppend(el, elArgs)
            },
            Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { .. }, .. } => metamodelica::cons(metamodelica::nil(), elArgs),
            Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { comment: __esc_cmt, .. }, .. }, .. }, constrainClass: __esc_cc, .. } => {
                cmt = (*__esc_cmt).clone();
                cc = (*__esc_cc).clone();
                anns = (::match_deref::match_deref! { match &(cmt.clone()) {
            Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: __esc_anns }), .. }) => {
                anns = (*__esc_anns).clone();
                anns.clone()
            },
            _ => metamodelica::nil(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                metamodelica::cons(listAppend(anns.clone(), AbsynUtil::getAnnotationsFromConstraintClass(cc.clone())), elArgs)
            },
            Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { .. }, .. } => metamodelica::cons(metamodelica::nil(), elArgs),
            Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. }, .. }, .. } => metamodelica::cons(metamodelica::nil(), elArgs),
            _ => elArgs,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    for mut l in &*elArgs {
        stringLst = metamodelica::nil();
        for mut e in &*l.clone().reverse() {
            r#str = evaluateAnnotation_dispatch(
                absynProgram.clone(),
                classPath.clone(),
                &(metamodelica::Ref::new(Absyn::Annotation {
                    elementArgs: list![e.clone()],
                })),
                true,
            )?;
            stringLst = metamodelica::cons(r#str, stringLst);
        }
        r#str = stringDelimitList(stringLst, literal!(", "));
        outStringLst = metamodelica::cons(
            stringAppendList(list![literal!("{"), r#str, literal!("}")]),
            outStringLst,
        );
    }
    if Flags::isSet(Flags::EXEC_STAT.clone())? {
        execStat(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFApi.evaluateAnnotations_dispatch("));
                __mm_s.push_str(&*AbsynUtil::pathString(classPath, literal!("."), true, false)?);
                __mm_s.push_str(&*literal!(" annotation("));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(List::flatten(elArgs)?, &Dump::unparseElementArgStr)?,
                    literal!(", "),
                ));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    Ok(outStringLst)
}

pub(crate) fn mkFullyQual(
    mut absynProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Path>,
    mut pathToQualify: metamodelica::Ref<Path>,
    mut failOnError: bool,
) -> Result<metamodelica::Ref<Path>> {
    let mut qualPath: metamodelica::Ref<Path> = pathToQualify.clone();
    let mut expanded_cls: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<InstNode::InstNode>;
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut name: ArcStr;
    let mut id1: ArcStr;
    let mut id2: ArcStr;
    let mut b: bool;
    let mut s: bool;
    let mut context: i32;
    let () = (::match_deref::match_deref! { match &((&*classPath, &*pathToQualify)) {
        (Deref @ Absyn::Path::QUALIFIED { name: id1, path: _ }, Deref @ Absyn::Path::QUALIFIED { name: id2, path: _ }) if (metamodelica::stringEq(&id1, &id2)) => {
            return Ok(qualPath);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b = FlagsUtil::set(Flags::SCODE_INST.clone(), true)?;
    s = FlagsUtil::set(Flags::NF_SCALARIZE.clone(), true)?;
    match '__try0: {
        if !(unwrap_break_err!(Flags::isSet(Flags::NF_API_NOISE.clone()), '__try0)) {
            ErrorExt::setCheckpoint(literal!("NFApi.mkFullyQual"));
        }
        (program, name, expanded_cls) =
            unwrap_break_err!(frontEndLookup(absynProgram.clone(), classPath.clone()), '__try0);
        context = InstContext::set(InstContext::RELAXED.clone(), InstContext::FAST_LOOKUP.clone());
        if InstNode::isDerivedClass(&expanded_cls) {
            (cls, _) = unwrap_break_err!(Lookup::lookupClassName(pathToQualify.clone(), unwrap_break_err!(InstNode::classParent(&expanded_cls), '__try0), context, Absyn::dummyInfo.clone(), false), '__try0);
        } else {
            (cls, _) = unwrap_break_err!(Lookup::lookupClassName(pathToQualify.clone(), expanded_cls.clone(), context, Absyn::dummyInfo.clone(), false), '__try0);
        }
        qualPath = unwrap_break_err!(InstNode::fullPath(cls.clone(), false), '__try0);
        if !(unwrap_break_err!(Flags::isSet(Flags::NF_API_NOISE.clone()), '__try0)) {
            ErrorExt::rollBack(literal!("NFApi.mkFullyQual"));
        }
        unwrap_break_err!(FlagsUtil::set(Flags::SCODE_INST.clone(), b), '__try0);
        unwrap_break_err!(FlagsUtil::set(Flags::NF_SCALARIZE.clone(), s), '__try0);
        Ok::<_, &'static str>((qualPath.clone(),))
    } {
        Ok((__try0_o0,)) => {
            qualPath = __try0_o0;
        }
        Err(_) => {
            if !(Flags::isSet(Flags::NF_API_NOISE.clone())?) {
                ErrorExt::rollBack(literal!("NFApi.mkFullyQual"));
            }
            FlagsUtil::set(Flags::SCODE_INST.clone(), b)?;
            FlagsUtil::set(Flags::NF_SCALARIZE.clone(), s)?;
            if failOnError {
                return Err("fail");
            } else {
                qualPath = pathToQualify.clone();
            }
        }
    }
    if Flags::isSet(Flags::EXEC_STAT.clone())? {
        execStat(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFApi.mkFullyQual("));
                __mm_s.push_str(&*AbsynUtil::pathString(classPath, literal!("."), true, false)?);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*AbsynUtil::pathString(pathToQualify, literal!("."), true, false)?);
                __mm_s.push_str(&*literal!(") -> "));
                __mm_s.push_str(&*AbsynUtil::pathString(qualPath.clone(), literal!("."), true, false)?);
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    Ok(qualPath)
}

pub(crate) fn clearCache() -> () {
    {
        let __v = metamodelica::nil();
        crate::Globals::instNFInstCacheIndex.with(|__root| *__root.borrow_mut() = __v)
    };
    {
        let __v = metamodelica::nil();
        crate::Globals::instNFLookupCacheIndex.with(|__root| *__root.borrow_mut() = __v)
    };
    NFInstanceAPI::clearTopScopeCache();
    ()
}

fn frontEndFront(
    mut absynProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Path>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    ArcStr,
    metamodelica::Ref<InstNode::InstNode>,
)> {
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut name: ArcStr;
    let mut inst_cls: metamodelica::Ref<InstNode::InstNode>;
    let mut cache: metamodelica::List<(
        (Absyn::Program, metamodelica::Ref<Path>),
        (
            metamodelica::List<metamodelica::Ref<SCode::Element>>,
            ArcStr,
            metamodelica::Ref<InstNode::InstNode>,
        ),
    )>;
    cache = crate::Globals::instNFInstCacheIndex.with(|__root| __root.borrow().clone());
    if !((cache).is_empty()) {
        for mut i in &*cache.clone() {
            if {
                let __refeq_sl = &(absynProgram.clone());
                let __refeq_sr = &(Util::tuple21(Util::tuple21(i.clone())));
                metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.classes), &(__refeq_sr.classes))
                    && (match (&(__refeq_sl.within_), &(__refeq_sr.within_)) {
                        (Absyn::Within::TOP, Absyn::Within::TOP) => true,
                        (Absyn::Within::WITHIN { path: __refeq_v0l }, Absyn::Within::WITHIN { path: __refeq_v0r }) => {
                            referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                        }
                        _ => false,
                    })
            } {
                if AbsynUtil::pathEqual(&classPath, &(Util::tuple22(Util::tuple21(i.clone())))) {
                    (program, name, inst_cls) = Util::tuple22(i.clone());
                    return Ok((program, name, inst_cls));
                }
                cache = metamodelica::nil();
                {
                    let __v = cache.clone();
                    crate::Globals::instNFInstCacheIndex.with(|__root| *__root.borrow_mut() = __v)
                };
                break;
            } else {
                if AbsynUtil::pathEqual(&classPath, &(Util::tuple22(Util::tuple21(i.clone())))) {
                    cache = metamodelica::nil();
                    {
                        let __v = cache.clone();
                        crate::Globals::instNFInstCacheIndex.with(|__root| *__root.borrow_mut() = __v)
                    };
                    break;
                }
            }
        }
    }
    (program, name, inst_cls) = frontEndFront_dispatch(absynProgram.clone(), classPath.clone())?;
    if ((cache).len() as i32) > 100 {
        cache = List::firstN(cache, 10)?;
    }
    cache = metamodelica::cons(
        (
            (absynProgram, classPath),
            (program.clone(), name.clone(), inst_cls.clone()),
        ),
        cache,
    );
    {
        let __v = cache;
        crate::Globals::instNFInstCacheIndex.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok((program, name, inst_cls))
}

fn frontEndFront_dispatch(
    mut absynProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Path>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    ArcStr,
    metamodelica::Ref<InstNode::InstNode>,
)> {
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut name: ArcStr;
    let mut inst_cls: metamodelica::Ref<InstNode::InstNode>;
    let mut top: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<InstNode::InstNode>;
    name = AbsynUtil::pathString(classPath.clone(), literal!("."), true, false)?;
    (program, top) = NFInstanceAPI::mkTop(absynProgram.clone(), scodeFor(&absynProgram)?, &name)?;
    (cls, _) = Lookup::lookupClassName(
        classPath,
        top.clone(),
        InstContext::RELAXED.clone(),
        Absyn::dummyInfo.clone(),
        false,
    )?;
    cls = InstNode::makeRootClass(
        cls,
        openmodelica_nf_frontend::NFInstNode::InstNode::interned_EMPTY_NODE(),
        None,
    )?;
    inst_cls = Inst::instantiate(
        cls,
        openmodelica_nf_frontend::NFModifier::Modifier::interned_NOMOD(),
        openmodelica_nf_frontend::NFInstNode::InstNode::interned_EMPTY_NODE(),
        InstContext::RELAXED.clone(),
        false,
    )?;
    Inst::insertGeneratedInners(inst_cls.clone(), &top, InstContext::RELAXED.clone())?;
    Inst::instExpressions(
        inst_cls.clone(),
        &(inst_cls.clone()),
        openmodelica_nf_frontend::NFSections::interned_EMPTY(),
        &(NFConnectBreakTree::new()),
        InstContext::RELAXED.clone(),
        &(Inst::DEFAULT_SETTINGS.clone()),
    )?;
    Inst::updateImplicitVariability(
        inst_cls.clone(),
        Flags::isSet(Flags::EVAL_PARAM.clone())?,
        InstContext::RELAXED.clone(),
    )?;
    if Flags::isSet(Flags::EXEC_STAT.clone())? {
        execStat(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFApi.frontEndFront_dispatch("));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    Inst::clearCaches()?;
    Ok((program, name, inst_cls))
}

fn frontEndBack(
    mut inst_cls: metamodelica::Ref<InstNode::InstNode>,
    mut name: ArcStr,
    mut scalarize: bool,
) -> Result<DAE::DAElist> {
    let mut dae: DAE::DAElist;
    let mut flat_model: metamodelica::Ref<FlatModel::NFFlatModel>;
    let mut funcs: metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>;
    let mut daeFuncs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    Typing::typeClass(inst_cls.clone(), InstContext::RELAXED.clone())?;
    flat_model = Flatten::flatten(
        inst_cls.clone(),
        metamodelica::Ref::new(Path::IDENT { name: name.clone() }),
        true,
    )?;
    flat_model = EvalConstants::evaluate(flat_model, InstContext::RELAXED.clone())?;
    flat_model = UnitCheck::checkUnits(flat_model)?;
    flat_model = SimplifyModel::simplify(flat_model)?;
    flat_model = Package::collectConstants(flat_model)?;
    funcs = Flatten::collectFunctions(&flat_model)?;
    if Flags::isSet(Flags::NF_SCALARIZE.clone())? {
        flat_model = Scalarize::scalarize(flat_model)?;
    } else {
        assign_field!(
            flat_model.variables =
                List::filterOnFalse(flat_model.variables.clone(), &move |__a0: metamodelica::Ref<
                    Variable::NFVariable,
                >| Variable::isEmptyArray(
                    &__a0
                ))?
        );
    }
    VerifyModel::verify(&flat_model, InstNode::isPartial(&inst_cls)?)?;
    (dae, daeFuncs) = ConvertDAE::convert(&flat_model, &funcs)?;
    if Flags::isSet(Flags::EXEC_STAT.clone())? {
        execStat(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFApi.frontEndBack("));
                __mm_s.push_str(&*AbsynUtil::pathString(
                    InstNode::enclosingScopePath(inst_cls, false, false)?,
                    literal!("."),
                    true,
                    false,
                )?);
                __mm_s.push_str(&*literal!(", name: "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(", scalarize: "));
                __mm_s.push_str(&*boolString(scalarize));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    Ok(dae)
}

fn frontEndLookup(
    mut absynProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Path>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    ArcStr,
    metamodelica::Ref<InstNode::InstNode>,
)> {
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut name: ArcStr;
    let mut expanded_cls: metamodelica::Ref<InstNode::InstNode>;
    let mut cache: metamodelica::List<(
        (Absyn::Program, metamodelica::Ref<Path>),
        (
            metamodelica::List<metamodelica::Ref<SCode::Element>>,
            ArcStr,
            metamodelica::Ref<InstNode::InstNode>,
        ),
    )>;
    cache = crate::Globals::instNFLookupCacheIndex.with(|__root| __root.borrow().clone());
    if !((cache).is_empty()) {
        for mut i in &*cache.clone() {
            if {
                let __refeq_sl = &(absynProgram.clone());
                let __refeq_sr = &(Util::tuple21(Util::tuple21(i.clone())));
                metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.classes), &(__refeq_sr.classes))
                    && (match (&(__refeq_sl.within_), &(__refeq_sr.within_)) {
                        (Absyn::Within::TOP, Absyn::Within::TOP) => true,
                        (Absyn::Within::WITHIN { path: __refeq_v0l }, Absyn::Within::WITHIN { path: __refeq_v0r }) => {
                            referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                        }
                        _ => false,
                    })
            } {
                if AbsynUtil::pathEqual(&classPath, &(Util::tuple22(Util::tuple21(i.clone())))) {
                    (program, name, expanded_cls) = Util::tuple22(i.clone());
                    return Ok((program, name, expanded_cls));
                }
                cache = metamodelica::nil();
                {
                    let __v = cache.clone();
                    crate::Globals::instNFLookupCacheIndex.with(|__root| *__root.borrow_mut() = __v)
                };
                break;
            } else {
                if AbsynUtil::pathEqual(&classPath, &(Util::tuple22(Util::tuple21(i.clone())))) {
                    cache = metamodelica::nil();
                    {
                        let __v = cache.clone();
                        crate::Globals::instNFLookupCacheIndex.with(|__root| *__root.borrow_mut() = __v)
                    };
                    break;
                }
            }
        }
    }
    (program, name, expanded_cls) = frontEndLookup_dispatch(absynProgram.clone(), classPath.clone())?;
    if ((cache).len() as i32) > 100 {
        cache = List::firstN(cache, 10)?;
    }
    cache = metamodelica::cons(
        (
            (absynProgram, classPath),
            (program.clone(), name.clone(), expanded_cls.clone()),
        ),
        cache,
    );
    {
        let __v = cache;
        crate::Globals::instNFLookupCacheIndex.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok((program, name, expanded_cls))
}

fn frontEndLookup_dispatch(
    mut absynProgram: Absyn::Program,
    mut classPath: metamodelica::Ref<Path>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    ArcStr,
    metamodelica::Ref<InstNode::InstNode>,
)> {
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut name: ArcStr;
    let mut expanded_cls: metamodelica::Ref<InstNode::InstNode>;
    let mut top: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<InstNode::InstNode>;
    name = AbsynUtil::pathString(classPath.clone(), literal!("."), true, false)?;
    (program, top) = NFInstanceAPI::mkTop(absynProgram.clone(), scodeFor(&absynProgram)?, &name)?;
    if AbsynUtil::pathEqual(
        &classPath,
        &(metamodelica::Ref::new(Path::IDENT {
            name: literal!("AllLoadedClasses"),
        })),
    ) {
        expanded_cls = top;
    } else {
        cls = Inst::lookupRootClass(classPath, top, FAST_CONTEXT.clone())?;
        expanded_cls = Inst::expand(cls, FAST_CONTEXT.clone())?;
    }
    if Flags::isSet(Flags::EXEC_STAT.clone())? {
        execStat(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFApi.frontEndLookup_dispatch("));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    Inst::clearCaches()?;
    Ok((program, name, expanded_cls))
}

pub(crate) fn getInheritedClasses(
    mut classPath: metamodelica::Ref<Path>,
    mut program: Absyn::Program,
) -> Result<metamodelica::List<metamodelica::Ref<Path>>> {
    let mut extendsPaths: metamodelica::List<metamodelica::Ref<Path>>;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut start_idx: i32;
    if !(Flags::isSet(Flags::SCODE_INST.clone())?) {
        extendsPaths = metamodelica::nil();
        return Ok(extendsPaths);
    }
    (_, _, cls_node) = frontEndLookup(program, classPath)?;
    if !(InstNode::isClass(&cls_node)?) {
        extendsPaths = metamodelica::nil();
        return Ok(extendsPaths);
    }
    cls = InstNode::getClass(cls_node.clone())?;
    extendsPaths = (match &*cls {
        Class::EXPANDED_DERIVED {
            baseClass: __cls_baseClass,
            ..
        } => list![InstNode::fullPath(__cls_baseClass.clone(), true)?],
        _ => {
            exts = ClassTree::getExtends(&(Class::classTree(cls)?));
            start_idx = if (SCodeUtil::isClassExtends(&(InstNode::definition(cls_node)?))) {
                2
            } else {
                1
            };
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Path>> = metamodelica::nil();
                for mut i in (start_idx..=metamodelica::arrayLength(exts.clone())).into_iter() {
                    let __x = InstNode::fullPath(
                        ({
                            let __elt = (*metamodelica::index_checked(&exts.borrow(), i.clone())?).clone();
                            __elt
                        }),
                        true,
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
    });
    Ok(extendsPaths)
}

pub(crate) fn getNthInheritedClass(
    mut classPath: metamodelica::Ref<Path>,
    mut index: i32,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    if !(Flags::isSet(Flags::SCODE_INST.clone())?) {
        result = ValuesMake::makeBoolean(false);
        return Ok(result);
    }
    (_, _, cls_node) = frontEndLookup(program, classPath)?;
    if !(InstNode::isClass(&cls_node)?) {
        result = ValuesMake::makeBoolean(false);
        return Ok(result);
    }
    cls = InstNode::getClass(cls_node)?;
    exts = (match &*cls {
        Class::EXPANDED_DERIVED {
            baseClass: __cls_baseClass,
            ..
        } => metamodelica::arrayFromVec(list![__cls_baseClass.clone()].into_iter().cloned().collect()),
        _ => ClassTree::getExtends(&(Class::classTree(cls)?)),
    });
    if index < 1 || index > metamodelica::arrayLength(exts.clone()) {
        result = ValuesMake::makeBoolean(false);
        return Ok(result);
    }
    result = ValuesMake::makeCodeTypeName(InstNode::fullPath(
        ({
            let __elt = (*metamodelica::index_checked(&exts.borrow(), index)?).clone();
            __elt
        }),
        true,
    )?);
    Ok(result)
}

pub(crate) fn getModelInstance(
    mut classPath: metamodelica::Ref<Path>,
    mut contextPath: metamodelica::Ref<Path>,
    mut modifier: &ArcStr,
    mut prettyPrint: bool,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut json: metamodelica::Ref<JSON::JSON>;
    match '__try0: {
        json = unwrap_break_err!(NFInstanceAPI::buildModelInstanceJSON(SymbolTable::getAbsyn(), Some(unwrap_break_err!(SymbolTable::getSCode(), '__try0)), classPath.clone(), contextPath.clone(), modifier), '__try0);
        res = metamodelica::Ref::new(Values::Value::STRING {
            string: unwrap_break_err!(JSON::toString(&json, prettyPrint), '__try0),
        });
        unwrap_break_err!(execStat(&(literal!("JSON.toString"))), '__try0);
        unwrap_break_err!(Inst::clearCaches(), '__try0);
        Ok::<_, &'static str>((json.clone(), res.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            json = __try0_o0;
            res = __try0_o1;
        }
        Err(__try0_err) => {
            Inst::clearCaches()?;
            return Err(__try0_err);
        }
    }
    Ok(res)
}

pub(crate) fn getModelInstanceReference(
    mut classPath: metamodelica::Ref<Path>,
    mut contextPath: metamodelica::Ref<Path>,
    mut modifier: &ArcStr,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut handle: i32;
    match '__try0: {
        json = unwrap_break_err!(NFInstanceAPI::buildModelInstanceJSON(SymbolTable::getAbsyn(), Some(unwrap_break_err!(SymbolTable::getSCode(), '__try0)), classPath.clone(), contextPath.clone(), modifier), '__try0);
        json = unwrap_break_err!(JSON::toListForm(&json), '__try0);
        unwrap_break_err!(execStat(&(literal!("NFApi.toListForm"))), '__try0);
        handle = NFInstanceAPI::storeModelInstanceReference(json.clone());
        res = metamodelica::Ref::new(Values::Value::INTEGER { integer: handle });
        unwrap_break_err!(Inst::clearCaches(), '__try0);
        Ok::<_, &'static str>((handle.clone(), json.clone(), res.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            handle = __try0_o0;
            json = __try0_o1;
            res = __try0_o2;
        }
        Err(__try0_err) => {
            Inst::clearCaches()?;
            return Err(__try0_err);
        }
    }
    Ok(res)
}

pub(crate) fn getModelInstanceIconReference(
    mut classPath: metamodelica::Ref<Path>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut handle: i32;
    match '__try0: {
        json = unwrap_break_err!(NFInstanceAPI::buildModelInstanceIconJSON(SymbolTable::getAbsyn(), Some(unwrap_break_err!(SymbolTable::getSCode(), '__try0)), classPath.clone()), '__try0);
        json = unwrap_break_err!(JSON::toListForm(&json), '__try0);
        handle = NFInstanceAPI::storeModelInstanceReference(json.clone());
        res = metamodelica::Ref::new(Values::Value::INTEGER { integer: handle });
        unwrap_break_err!(Inst::clearCaches(), '__try0);
        Ok::<_, &'static str>((handle.clone(), json.clone(), res.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            handle = __try0_o0;
            json = __try0_o1;
            res = __try0_o2;
        }
        Err(__try0_err) => {
            Inst::clearCaches()?;
            return Err(__try0_err);
        }
    }
    Ok(res)
}

pub(crate) fn getModelInstanceAnnotation(
    mut classPath: metamodelica::Ref<Path>,
    mut filter: &metamodelica::List<ArcStr>,
    mut prettyPrint: bool,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut json: metamodelica::Ref<JSON::JSON>;
    match '__try0: {
        json = unwrap_break_err!(NFInstanceAPI::buildModelInstanceAnnotationJSON(SymbolTable::getAbsyn(), Some(unwrap_break_err!(SymbolTable::getSCode(), '__try0)), classPath.clone(), filter), '__try0);
        res = metamodelica::Ref::new(Values::Value::STRING {
            string: unwrap_break_err!(JSON::toString(&json, prettyPrint), '__try0),
        });
        unwrap_break_err!(Inst::clearCaches(), '__try0);
        Ok::<_, &'static str>((json.clone(), res.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            json = __try0_o0;
            res = __try0_o1;
        }
        Err(__try0_err) => {
            Inst::clearCaches()?;
            return Err(__try0_err);
        }
    }
    Ok(res)
}

pub(crate) fn getModelInstanceAnnotationReference(
    mut classPath: metamodelica::Ref<Path>,
    mut filter: &metamodelica::List<ArcStr>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut handle: i32;
    match '__try0: {
        json = unwrap_break_err!(NFInstanceAPI::buildModelInstanceAnnotationJSON(SymbolTable::getAbsyn(), Some(unwrap_break_err!(SymbolTable::getSCode(), '__try0)), classPath.clone(), filter), '__try0);
        json = unwrap_break_err!(JSON::toListForm(&json), '__try0);
        handle = NFInstanceAPI::storeModelInstanceReference(json.clone());
        res = metamodelica::Ref::new(Values::Value::INTEGER { integer: handle });
        unwrap_break_err!(Inst::clearCaches(), '__try0);
        Ok::<_, &'static str>((handle.clone(), json.clone(), res.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            handle = __try0_o0;
            json = __try0_o1;
            res = __try0_o2;
        }
        Err(__try0_err) => {
            Inst::clearCaches()?;
            return Err(__try0_err);
        }
    }
    Ok(res)
}

pub(crate) fn releaseModelInstanceReference(mut handle: i32) -> metamodelica::Ref<Values::Value> {
    let mut res: metamodelica::Ref<Values::Value>;
    res = metamodelica::Ref::new(Values::Value::BOOL {
        boolean: NFInstanceAPI::releaseModelInstanceReferenceImpl(handle),
    });
    res
}

pub(crate) fn modifierToJSON(mut modifier: &ArcStr, mut prettyPrint: bool) -> Result<metamodelica::Ref<Values::Value>> {
    let mut jsonString: metamodelica::Ref<Values::Value>;
    jsonString = metamodelica::Ref::new(Values::Value::STRING {
        string: JSON::toString(&(NFInstanceAPI::modifierJSON(modifier)?), prettyPrint)?,
    });
    Ok(jsonString)
}

pub(crate) fn scodeFor(
    mut absynProgram: &Absyn::Program,
) -> Result<Option<metamodelica::List<metamodelica::Ref<SCode::Element>>>> {
    let mut scodeProgram: Option<metamodelica::List<metamodelica::Ref<SCode::Element>>> = if ({
        let __refeq_sl = &(absynProgram.clone());
        let __refeq_sr = &(SymbolTable::getAbsyn());
        metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.classes), &(__refeq_sr.classes))
            && (match (&(__refeq_sl.within_), &(__refeq_sr.within_)) {
                (Absyn::Within::TOP, Absyn::Within::TOP) => true,
                (Absyn::Within::WITHIN { path: __refeq_v0l }, Absyn::Within::WITHIN { path: __refeq_v0r }) => {
                    referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                }
                _ => false,
            })
    }) {
        Some(SymbolTable::getSCode()?)
    } else {
        None
    };
    Ok(scodeProgram)
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct MoveEnv {
    pub scope: metamodelica::Ref<InstNode::InstNode>,
    pub destinationPath: metamodelica::Ref<Path>,
    pub destination: metamodelica::Ref<InstNode::InstNode>,
}

impl metamodelica::gc::MMTrace for MoveEnv {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.scope, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.destinationPath, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.destination, __mmv)?;
        Ok(())
    }
}
impl Default for MoveEnv {
    fn default() -> Self {
        Self {
            scope: Default::default(),
            destinationPath: Default::default(),
            destination: Default::default(),
        }
    }
}

pub type MOVE_ENV = MoveEnv;

pub(crate) fn updateMovedClassPaths(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut clsPath: metamodelica::Ref<Path>,
    mut destination: &Absyn::Within,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut top: metamodelica::Ref<InstNode::InstNode>;
    let mut src_node: metamodelica::Ref<InstNode::InstNode>;
    let mut dst_node: metamodelica::Ref<InstNode::InstNode> =
        openmodelica_nf_frontend::NFInstNode::InstNode::interned_EMPTY_NODE();
    let mut env: MoveEnv;
    let mut dst_path: metamodelica::Ref<Path>;
    let mut p: metamodelica::Ref<Path>;
    let mut found: bool = false;
    (_, top) = NFInstanceAPI::mkTop(
        SymbolTable::getAbsyn(),
        Some(SymbolTable::getSCode()?),
        &(AbsynUtil::pathString(clsPath.clone(), literal!("."), true, false)?),
    )?;
    src_node = Inst::lookupRootClass(clsPath, top.clone(), FAST_CONTEXT.clone())?;
    Inst::expand(src_node.clone(), FAST_CONTEXT.clone())?;
    dst_path = (match destination.clone() {
        Absyn::Within::WITHIN { .. } => AbsynUtil::suffixPath(
            var_field!(destination.path, Absyn::Within::WITHIN),
            &(InstNode::name(&src_node)?),
        ),
        _ => metamodelica::Ref::new(Path::IDENT {
            name: InstNode::name(&src_node)?,
        }),
    });
    dst_node = top.clone();
    p = dst_path.clone();
    while !(found) && !(AbsynUtil::pathIsIdent(&p)) {
        if '__try0: {
            p = unwrap_break_err!(AbsynUtil::pathPrefix(&p), '__try0);
            (dst_node, _, _) =
                unwrap_break_err!(Lookup::lookupName(&p, top.clone(), FAST_CONTEXT.clone(), false), '__try0);
            unwrap_break_err!(Inst::expand(dst_node.clone(), FAST_CONTEXT.clone()), '__try0);
            found = true;
            Ok::<(), &'static str>(())
        }
        .is_err()
        {}
    }
    env = MoveEnv {
        scope: src_node,
        destinationPath: dst_path,
        destination: dst_node,
    };
    assign_field!(cls.body = updateMovedClassDef(cls.body.clone(), env)?);
    Ok(cls)
}

pub(crate) fn updateMovedClass(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut cls_env: MoveEnv;
    if classHasScope(&cls) {
        (cls_node, _) = Lookup::lookupLocalSimpleName(cls.name.clone(), env.scope.clone())?;
        Inst::expand(cls_node.clone(), FAST_CONTEXT.clone())?;
        cls_env = MoveEnv {
            scope: cls_node,
            destinationPath: AbsynUtil::suffixPath(&env.destinationPath, &cls.name),
            destination: env.destination.clone(),
        };
    } else {
        cls_env = env;
    }
    assign_field!(cls.body = updateMovedClassDef(cls.body.clone(), cls_env)?);
    Ok(cls)
}

pub(crate) fn classHasScope(mut cls: &metamodelica::Ref<Absyn::Class>) -> bool {
    let mut hasScope: bool;
    hasScope = (match &*cls.body.clone() {
        Absyn::ClassDef::PARTS { .. } => true,
        Absyn::ClassDef::CLASS_EXTENDS { .. } => true,
        _ => false,
    });
    hasScope
}

pub(crate) fn updateMovedClassDef(
    mut cdef: metamodelica::Ref<Absyn::ClassDef>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::ClassDef>> {
    let mut cdef: metamodelica::Ref<Absyn::ClassDef> = cdef;
    let () = (match &*cdef {
        Absyn::ClassDef::PARTS {
            classParts: __cdef_classParts,
            ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::PARTS;
                        classParts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
                for mut p in (__cdef_classParts.clone()).into_iter().cloned() {
                    let __x = updateMovedClassPart(p.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        ann = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Annotation>> = metamodelica::nil();
                for mut a in (var_field!((*cdef).ann, Absyn::ClassDef::PARTS).clone()).into_iter().cloned() {
                    let __x = updateMovedAnnotation(a.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        Absyn::ClassDef::DERIVED {
            typeSpec: __cdef_typeSpec,
            ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::DERIVED;
                        typeSpec = updateMovedTypeSpec(__cdef_typeSpec.clone(), env.clone())?,
                        attributes = updateMovedElementAttributes(var_field!((*cdef).attributes, Absyn::ClassDef::DERIVED).clone(), env.clone())?,
                        arguments = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
                for mut a in (var_field!((*cdef).arguments, Absyn::ClassDef::DERIVED).clone()).into_iter().cloned() {
                    let __x = updateMovedElementArg(a.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        comment = updateMovedCommentOpt(var_field!((*cdef).comment, Absyn::ClassDef::DERIVED).clone(), env)?
                    );
            ()
        }
        Absyn::ClassDef::CLASS_EXTENDS {
            modifications: __cdef_modifications,
            ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS;
                        modifications = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
                for mut a in (__cdef_modifications.clone()).into_iter().cloned() {
                    let __x = updateMovedElementArg(a.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        parts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
                for mut p in (var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
                    let __x = updateMovedClassPart(p.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        ann = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Annotation>> = metamodelica::nil();
                for mut a in (var_field!((*cdef).ann, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
                    let __x = updateMovedAnnotation(a.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        Absyn::ClassDef::PDER {
            functionName: __cdef_functionName,
            ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::PDER;
                functionName = updateMovedPath(__cdef_functionName.clone(), &env)?,
                comment = updateMovedCommentOpt(var_field!((*cdef).comment, Absyn::ClassDef::PDER).clone(), env)?
            );
            ()
        }
        _ => (),
    });
    Ok(cdef)
}

pub(crate) fn updateMovedClassPart(
    mut part: metamodelica::Ref<Absyn::ClassPart>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::ClassPart>> {
    let mut part: metamodelica::Ref<Absyn::ClassPart> = part;
    let () = (match &*part {
        Absyn::ClassPart::PUBLIC {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
                for mut i in (__part_contents.clone()).into_iter().cloned() {
                    let __x = updateMovedElementItem(i.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::ClassPart::PROTECTED {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
                for mut i in (__part_contents.clone()).into_iter().cloned() {
                    let __x = updateMovedElementItem(i.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::ClassPart::EQUATIONS {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = updateMovedEquationItems(__part_contents.clone(), env)?);
            ()
        }
        Absyn::ClassPart::INITIALEQUATIONS {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALEQUATIONS; contents = updateMovedEquationItems(__part_contents.clone(), env)?);
            ()
        }
        Absyn::ClassPart::ALGORITHMS {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::ALGORITHMS; contents = updateMovedAlgorithmItems(__part_contents.clone(), env)?);
            ()
        }
        Absyn::ClassPart::INITIALALGORITHMS {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALALGORITHMS; contents = updateMovedAlgorithmItems(__part_contents.clone(), env)?);
            ()
        }
        Absyn::ClassPart::EXTERNAL {
            annotation_: __part_annotation_,
            ..
        } => {
            assign_variant_field!(part => Absyn::ClassPart::EXTERNAL; annotation_ = updateMovedAnnotationOpt(__part_annotation_.clone(), env)?);
            ()
        }
        _ => (),
    });
    Ok(part)
}

pub(crate) fn updateMovedElementItem(
    mut item: metamodelica::Ref<Absyn::ElementItem>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::ElementItem>> {
    let mut item: metamodelica::Ref<Absyn::ElementItem> = item;
    let () = (match &*item {
        Absyn::ElementItem::ELEMENTITEM {
            element: __item_element,
        } => {
            assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = updateMovedElement(__item_element.clone(), env)?);
            ()
        }
        _ => (),
    });
    Ok(item)
}

pub(crate) fn updateMovedElement(
    mut element: metamodelica::Ref<Absyn::Element>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let () = (match &*element {
        Absyn::Element::ELEMENT {
            specification: __element_specification,
            ..
        } => {
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = updateMovedElementSpec(__element_specification.clone(), env.clone())?);
            if (var_field!((*element).constrainClass, Absyn::Element::ELEMENT)).is_some() {
                assign_variant_field!(element => Absyn::Element::ELEMENT; constrainClass = Some(updateMovedConstrainClass(Util::getOption(var_field!((*element).constrainClass, Absyn::Element::ELEMENT).clone())?, env)?));
            }
            ()
        }
        _ => (),
    });
    Ok(element)
}

pub(crate) fn updateMovedConstrainClass(
    mut cc: metamodelica::Ref<Absyn::ConstrainClass>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::ConstrainClass>> {
    let mut cc: metamodelica::Ref<Absyn::ConstrainClass> = cc;
    assign_field!(
        cc.elementSpec = updateMovedElementSpec(cc.elementSpec.clone(), env.clone())?,
        cc.comment = updateMovedCommentOpt(cc.comment.clone(), env)?
    );
    Ok(cc)
}

pub(crate) fn updateMovedElementSpec(
    mut spec: metamodelica::Ref<Absyn::ElementSpec>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut spec: metamodelica::Ref<Absyn::ElementSpec> = spec;
    let () = (match &*spec {
        Absyn::ElementSpec::CLASSDEF {
            class_: __spec_class_, ..
        } => {
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = updateMovedClass(__spec_class_.clone(), env)?);
            ()
        }
        Absyn::ElementSpec::EXTENDS { path: __spec_path, .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::EXTENDS;
                        path = updateMovedPath(__spec_path.clone(), &env)?,
                        elementArg = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
                for mut a in (var_field!((*spec).elementArg, Absyn::ElementSpec::EXTENDS).clone()).into_iter().cloned() {
                    let __x = updateMovedElementArg(a.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        annotationOpt = updateMovedAnnotationOpt(var_field!((*spec).annotationOpt, Absyn::ElementSpec::EXTENDS).clone(), env)?
                    );
            ()
        }
        Absyn::ElementSpec::COMPONENTS {
            attributes: __spec_attributes,
            ..
        } => {
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS;
                        attributes = updateMovedElementAttributes(__spec_attributes.clone(), env.clone())?,
                        typeSpec = updateMovedTypeSpec(var_field!((*spec).typeSpec, Absyn::ElementSpec::COMPONENTS).clone(), env.clone())?,
                        components = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> = metamodelica::nil();
                for mut c in (var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone()).into_iter().cloned() {
                    let __x = updateMovedComponentItem(c.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        _ => (),
    });
    Ok(spec)
}

pub(crate) fn updateMovedElementAttributes(
    mut attr: Absyn::ElementAttributes,
    mut env: MoveEnv,
) -> Result<Absyn::ElementAttributes> {
    let mut attr: Absyn::ElementAttributes = attr;
    if !((attr.arrayDim).is_empty()) {
        attr.arrayDim = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
            for mut s in (attr.arrayDim.clone()).into_iter().cloned() {
                let __x = updateMovedSubscript(s.clone(), env.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    Ok(attr)
}

pub(crate) fn updateMovedElementArg(
    mut arg: metamodelica::Ref<Absyn::ElementArg>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut arg: metamodelica::Ref<Absyn::ElementArg> = arg;
    let () = (match &*arg {
        Absyn::ElementArg::MODIFICATION { .. } => {
            if (var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION)).is_some() {
                assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(updateMovedModification(Util::getOption(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone())?, env)?));
            }
            ()
        }
        Absyn::ElementArg::REDECLARATION {
            elementSpec: __arg_elementSpec,
            ..
        } => {
            assign_variant_field!(arg => Absyn::ElementArg::REDECLARATION; elementSpec = updateMovedElementSpec(__arg_elementSpec.clone(), env.clone())?);
            if (var_field!((*arg).constrainClass, Absyn::ElementArg::REDECLARATION)).is_some() {
                assign_variant_field!(arg => Absyn::ElementArg::REDECLARATION; constrainClass = Some(updateMovedConstrainClass(Util::getOption(var_field!((*arg).constrainClass, Absyn::ElementArg::REDECLARATION).clone())?, env)?));
            }
            ()
        }
        _ => (),
    });
    Ok(arg)
}

pub(crate) fn updateMovedModification(
    mut r#mod: metamodelica::Ref<Absyn::Modification>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::Modification>> {
    let mut r#mod: metamodelica::Ref<Absyn::Modification> = r#mod;
    let mut eq_mod: metamodelica::Ref<Absyn::EqMod>;
    assign_field!(
        r#mod.elementArgLst = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
            for mut a in (r#mod.elementArgLst.clone()).into_iter().cloned() {
                let __x = updateMovedElementArg(a.clone(), env.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    eq_mod = r#mod.eqMod.clone();
    let () = (match &*eq_mod {
        Absyn::EqMod::EQMOD { exp: __eq_mod_exp, .. } => {
            assign_variant_field!(eq_mod => Absyn::EqMod::EQMOD; exp = updateMovedExp(__eq_mod_exp.clone(), env)?);
            assign_field!(r#mod.eqMod = eq_mod);
            ()
        }
        _ => (),
    });
    Ok(r#mod)
}

pub(crate) fn updateMovedComponentItem(
    mut item: metamodelica::Ref<Absyn::ComponentItem>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut item: metamodelica::Ref<Absyn::ComponentItem> = item;
    assign_field!(item.component = updateMovedComponent(item.component.clone(), env.clone())?);
    if (item.condition).is_some() {
        assign_field!(item.condition = Some(updateMovedExp(Util::getOption(item.condition.clone())?, env)?));
    }
    Ok(item)
}

pub(crate) fn updateMovedComponent(mut component: Absyn::Component, mut env: MoveEnv) -> Result<Absyn::Component> {
    let mut component: Absyn::Component = component;
    if !((component.arrayDim).is_empty()) {
        component.arrayDim = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
            for mut d in (component.arrayDim.clone()).into_iter().cloned() {
                let __x = updateMovedSubscript(d.clone(), env.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    if (component.modification).is_some() {
        component.modification = Some(updateMovedModification(
            Util::getOption(component.modification.clone())?,
            env,
        )?);
    }
    Ok(component)
}

pub(crate) fn updateMovedEquationItems(
    mut items: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut env: MoveEnv,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>> {
    let mut items: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> = items;
    (items, _) = AbsynUtil::traverseEquationItemListBidir(
        items,
        (std::sync::Arc::new(updateMovedExp_traverser)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        MoveEnv,
                    ) -> Result<(metamodelica::Ref<Absyn::Exp>, MoveEnv)>
                    + 'static,
            >),
        std::sync::Arc::new(fnptr!(AbsynUtil::dummyTraverseExp, metamodelica::Ref<Absyn::Exp>, _)),
        env,
    )?;
    Ok(items)
}

pub(crate) fn updateMovedAlgorithmItems(
    mut items: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    mut env: MoveEnv,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>> {
    let mut items: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>> = items;
    (items, _) = AbsynUtil::traverseAlgorithmItemListBidir(
        items,
        (std::sync::Arc::new(updateMovedExp_traverser)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        MoveEnv,
                    ) -> Result<(metamodelica::Ref<Absyn::Exp>, MoveEnv)>
                    + 'static,
            >),
        std::sync::Arc::new(fnptr!(AbsynUtil::dummyTraverseExp, metamodelica::Ref<Absyn::Exp>, _)),
        env,
    )?;
    Ok(items)
}

pub(crate) fn updateMovedTypeSpec(
    mut ty: metamodelica::Ref<Absyn::TypeSpec>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::TypeSpec>> {
    let mut ty: metamodelica::Ref<Absyn::TypeSpec> = ty;
    let () = (match &*ty {
        Absyn::TypeSpec::TPATH { path: __ty_path, .. } => {
            assign_variant_field!(ty => Absyn::TypeSpec::TPATH; path = updateMovedPath(__ty_path.clone(), &env)?);
            if (var_field!((*ty).arrayDim, Absyn::TypeSpec::TPATH)).is_some() {
                assign_variant_field!(ty => Absyn::TypeSpec::TPATH; arrayDim = Some(({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
                    for mut s in (Util::getOption(var_field!((*ty).arrayDim, Absyn::TypeSpec::TPATH).clone())?).into_iter().cloned() {
                        let __x = updateMovedSubscript(s.clone(), env.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })));
            }
            ()
        }
        Absyn::TypeSpec::TCOMPLEX { path: __ty_path, .. } => {
            assign_variant_field!(ty => Absyn::TypeSpec::TCOMPLEX; path = updateMovedPath(__ty_path.clone(), &env)?);
            if (var_field!((*ty).arrayDim, Absyn::TypeSpec::TCOMPLEX)).is_some() {
                assign_variant_field!(ty => Absyn::TypeSpec::TCOMPLEX; arrayDim = Some(({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
                    for mut s in (Util::getOption(var_field!((*ty).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone())?).into_iter().cloned() {
                        let __x = updateMovedSubscript(s.clone(), env.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })));
            }
            ()
        }
    });
    Ok(ty)
}

pub(crate) fn updateMovedPath(mut path: metamodelica::Ref<Path>, mut env: &MoveEnv) -> Result<metamodelica::Ref<Path>> {
    let mut outPath: metamodelica::Ref<Path> = path.clone();
    let mut qualified_path: metamodelica::Ref<Path>;
    let mut new_path: metamodelica::Ref<Path>;
    let mut opt_path: Option<metamodelica::Ref<Path>>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    if let Ok(__iflet0) = Lookup::lookupSimpleNameRootPath(
        AbsynUtil::pathFirstIdent(&path),
        env.scope.clone(),
        FAST_CONTEXT.clone(),
    ) {
        qualified_path = __iflet0;
    } else {
        return Ok(outPath);
    }
    if AbsynUtil::pathIsFullyQualified(&qualified_path) {
        qualified_path = AbsynUtil::makeNotFullyQualified(qualified_path);
        if AbsynUtil::pathIsIdent(&qualified_path)
            && metamodelica::stringEq(
                &(AbsynUtil::pathFirstIdent(&qualified_path)),
                &(AbsynUtil::pathFirstIdent(&env.destinationPath)),
            )
        {
            outPath = AbsynUtil::pathRest(path.clone())?;
        } else {
            opt_path = AbsynUtil::pathStripSamePrefix(qualified_path.clone(), env.destinationPath.clone())?;
            if (opt_path).is_some() {
                let __pa1 = ::match_deref::match_deref! { match &(opt_path) {
                    Some(__pa1) => __pa1.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                new_path = metamodelica::Own::own(__pa1);
                outPath = AbsynUtil::pathReplaceFirst(&path, &new_path)?;
            }
        }
        if '__try2: {
            (node, _) = unwrap_break_err!(Lookup::lookupSimpleName(AbsynUtil::pathFirstIdent(&outPath), env.destination.clone(), FAST_CONTEXT.clone()), '__try2);
            let false = (AbsynUtil::pathPrefixOf(unwrap_break_err!(InstNode::fullPath(node.clone(), false), '__try2), qualified_path.clone())) else { break '__try2 Err::<_, _>("pattern mismatch") };
            outPath = unwrap_break_err!(AbsynUtil::pathReplaceFirst(&path, &qualified_path), '__try2);
            outPath = AbsynUtil::makeFullyQualified(outPath.clone());
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    Ok(outPath)
}

pub(crate) fn updateMovedCommentOpt(
    mut cmt: Option<metamodelica::Ref<Absyn::Comment>>,
    mut env: MoveEnv,
) -> Result<Option<metamodelica::Ref<Absyn::Comment>>> {
    let mut cmt: Option<metamodelica::Ref<Absyn::Comment>> = cmt;
    if (cmt).is_some() {
        cmt = Some(updateMovedComment(Util::getOption(cmt)?, env)?);
    }
    Ok(cmt)
}

pub(crate) fn updateMovedComment(
    mut cmt: metamodelica::Ref<Absyn::Comment>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::Comment>> {
    let mut cmt: metamodelica::Ref<Absyn::Comment> = cmt;
    assign_field!(cmt.annotation_ = updateMovedAnnotationOpt(cmt.annotation_.clone(), env)?);
    Ok(cmt)
}

pub(crate) fn updateMovedAnnotationOpt(
    mut ann: Option<metamodelica::Ref<Absyn::Annotation>>,
    mut env: MoveEnv,
) -> Result<Option<metamodelica::Ref<Absyn::Annotation>>> {
    let mut ann: Option<metamodelica::Ref<Absyn::Annotation>> = ann;
    if (ann).is_some() {
        ann = Some(updateMovedAnnotation(Util::getOption(ann)?, env)?);
    }
    Ok(ann)
}

pub(crate) fn updateMovedAnnotation(
    mut ann: metamodelica::Ref<Absyn::Annotation>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::Annotation>> {
    let mut ann: metamodelica::Ref<Absyn::Annotation> = ann;
    assign_field!(
        ann.elementArgs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
            for mut a in (ann.elementArgs.clone()).into_iter().cloned() {
                let __x = updateMovedElementArg(a.clone(), env.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(ann)
}

pub(crate) fn updateMovedSubscript(
    mut sub: metamodelica::Ref<Absyn::Subscript>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::Subscript>> {
    let mut sub: metamodelica::Ref<Absyn::Subscript> = sub;
    let () = (match &*sub {
        Absyn::Subscript::SUBSCRIPT {
            subscript: __sub_subscript,
        } => {
            assign_variant_field!(sub => Absyn::Subscript::SUBSCRIPT; subscript = updateMovedExp(__sub_subscript.clone(), env)?);
            ()
        }
        _ => (),
    });
    Ok(sub)
}

pub(crate) fn updateMovedExp(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut env: MoveEnv,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    (exp, _) = AbsynUtil::traverseExp(
        exp,
        (std::sync::Arc::new(updateMovedExp_traverser)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        MoveEnv,
                    ) -> Result<(metamodelica::Ref<Absyn::Exp>, MoveEnv)>
                    + 'static,
            >),
        env,
    )?;
    Ok(exp)
}

pub(crate) fn updateMovedExp_traverser(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut env: MoveEnv,
) -> Result<(metamodelica::Ref<Absyn::Exp>, MoveEnv)> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut env: MoveEnv = env;
    let () = (match &*exp {
        Absyn::Exp::CREF {
            componentRef: __exp_componentRef,
        } => {
            assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = updateMovedCref(__exp_componentRef.clone(), &env)?);
            ()
        }
        Absyn::Exp::CALL {
            function_: __exp_function_,
            ..
        } => {
            assign_variant_field!(exp => Absyn::Exp::CALL; function_ = updateMovedCref(__exp_function_.clone(), &env)?);
            ()
        }
        _ => (),
    });
    Ok((exp, env))
}

pub(crate) fn updateMovedCref(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut env: &MoveEnv,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let mut qualified_path: metamodelica::Ref<Path>;
    let mut qualified_cref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut opt_path: Option<metamodelica::Ref<Path>>;
    if AbsynUtil::crefIsFullyQualified(&cref) || AbsynUtil::crefIsWild(&cref) {
        return Ok(cref);
    }
    if let Ok(__iflet0) = Lookup::lookupSimpleNameRootPath(
        AbsynUtil::crefFirstIdent(&cref)?,
        env.scope.clone(),
        FAST_CONTEXT.clone(),
    ) {
        qualified_path = __iflet0;
    } else {
        return Ok(cref);
    }
    if AbsynUtil::pathIsFullyQualified(&qualified_path) {
        qualified_path = AbsynUtil::makeNotFullyQualified(qualified_path);
        if AbsynUtil::pathIsIdent(&qualified_path)
            && metamodelica::stringEq(
                &(AbsynUtil::pathFirstIdent(&qualified_path)),
                &(AbsynUtil::pathFirstIdent(&env.destinationPath)),
            )
        {
            cref = AbsynUtil::crefStripFirst(&cref)?;
        } else {
            opt_path = AbsynUtil::pathStripSamePrefix(qualified_path, env.destinationPath.clone())?;
            if (opt_path).is_some() {
                let __pa1 = ::match_deref::match_deref! { match &(opt_path) {
                    Some(__pa1) => __pa1.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                qualified_path = metamodelica::Own::own(__pa1);
                qualified_cref = AbsynUtil::pathToCref(&qualified_path);
                if AbsynUtil::crefIsQual(&cref) {
                    cref = AbsynUtil::joinCrefs(&qualified_cref, AbsynUtil::crefStripFirst(&cref)?)?;
                } else {
                    cref = qualified_cref;
                }
            }
        }
    }
    Ok(cref)
}

pub(crate) fn translateResidualsDAE(mut path: metamodelica::Ref<Path>, mut fileNamePrefix: ArcStr) -> Result<bool> {
    let mut success: bool = true;
    let mut disable_single_flow_eq: bool;
    let mut non_std_flags: metamodelica::List<ArcStr>;
    let mut flat_model: metamodelica::Ref<FlatModel::NFFlatModel>;
    let mut funcs: metamodelica::Ref<Flatten::FunctionTreeImpl::Tree>;
    let mut simSettings: Option<SimCode::SimulationSettings>;
    disable_single_flow_eq = FlagsUtil::set(Flags::DISABLE_SINGLE_FLOW_EQ.clone(), true)?;
    non_std_flags = FlagsUtil::appendConfigStringList(
        Flags::ALLOW_NON_STANDARD_MODELICA.clone(),
        literal!("implicitParameterStartAttribute"),
    )?;
    if '__try0: {
        (flat_model, funcs, _) = unwrap_break_err!(CevalScriptBackend::runFrontEndNF(path.clone(), false, false), '__try0);
        (flat_model, funcs) = unwrap_break_err!(InstUtil::createExtractorModel(flat_model.clone(), funcs.clone()), '__try0);
        unwrap_break_err!(InstUtil::dumpFlatModelDebug(literal!("translateResidualsDAE"), flat_model.clone(), &funcs), '__try0);
        simSettings = Some(unwrap_break_err!(CevalScriptBackend::convertSimulationOptionsToSimCode(&(unwrap_break_err!(CevalScriptBackend::buildSimulationOptionsFromModelExperimentAnnotation(path.clone(), fileNamePrefix.clone(), None), '__try0))), '__try0));
        unwrap_break_err!(SimCodeMain::translateModelCallBackend(&flat_model, &funcs, path.clone(), fileNamePrefix.clone(), true, simSettings.clone()), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    FlagsUtil::setConfigStringList(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), non_std_flags)?;
    FlagsUtil::set(Flags::DISABLE_SINGLE_FLOW_EQ.clone(), disable_single_flow_eq)?;
    Ok(success)
}
