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
use openmodelica_ast::Absyn::Path;
use openmodelica_error::ErrorExt;
use openmodelica_frontend::FBuiltin;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_loader::Parser;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFCeval as Ceval;
use openmodelica_nf_frontend::NFClass as Class;
use openmodelica_nf_frontend::NFClassTree::ClassTree;
use openmodelica_nf_frontend::NFComponent as Component;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFConnectBreakTree;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFEquation as Equation;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFImport as Import;
use openmodelica_nf_frontend::NFInst as Inst;
use openmodelica_nf_frontend::NFInst;
use openmodelica_nf_frontend::NFInst::InstSettings;
use openmodelica_nf_frontend::NFInstContext as InstContext;
use openmodelica_nf_frontend::NFInstContext;
use openmodelica_nf_frontend::NFInstNode;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFInstNode::InstNodeType;
use openmodelica_nf_frontend::NFLookup as Lookup;
use openmodelica_nf_frontend::NFModifier;
use openmodelica_nf_frontend::NFModifier::Modifier;
use openmodelica_nf_frontend::NFPrefixes::Purity;
use openmodelica_nf_frontend::NFPrefixes::Variability;
use openmodelica_nf_frontend::NFRestriction as Restriction;
use openmodelica_nf_frontend::NFSections as Sections;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFTyping as Typing;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::JSON;
use openmodelica_util::Settings;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::MutableWeak;

pub(crate) const ANNOTATION_CONTEXT: i32 = intBitOr(NFInstContext::RELAXED, NFInstContext::ANNOTATION);

pub(crate) const INST_API_ANNOTATION_CONTEXT: i32 = intBitOr(ANNOTATION_CONTEXT, NFInstContext::INSTANCE_API);

pub(crate) fn annotationProgram(mut annotationVersion: ArcStr) -> Result<Absyn::Program> {
    let mut program: Absyn::Program;
    let mut filename: ArcStr;
    filename = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
        __mm_s.push_str(&*literal!("/lib/omc/AnnotationsBuiltin_"));
        __mm_s.push_str(&*Util::stringReplaceChar(
            annotationVersion,
            literal!("."),
            literal!("_"),
        )?);
        __mm_s.push_str(&*literal!(".mo"));
        ArcStr::from(__mm_s)
    };
    program = Parser::parse(
        filename,
        literal!("UTF-8"),
        literal!(""),
        None,
        Config::acceptedGrammar()?,
        Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
        Flags::getConfigBool(Flags::STRICT.clone())?,
    )?;
    Ok(program)
}

pub fn mkTop(
    mut absynProgram: Absyn::Program,
    mut scodeProgram: Option<metamodelica::List<metamodelica::Ref<SCode::Element>>>,
    mut name: &ArcStr,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::Ref<InstNode::InstNode>,
)> {
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut top: metamodelica::Ref<InstNode::InstNode>;
    let mut scode_builtin: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut graphicProgramSCode: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut placementProgram: Absyn::Program;
    let mut cache: metamodelica::List<(
        Absyn::Program,
        (
            metamodelica::List<metamodelica::Ref<SCode::Element>>,
            metamodelica::Ref<InstNode::InstNode>,
        ),
    )>;
    let mut reuse: bool;
    cache = crate::Globals::instNFNodeCacheIndex.with(|__root| __root.borrow().clone());
    reuse = if ((cache).is_empty()) {
        false
    } else {
        {
            let __refeq_sl = &(absynProgram.clone());
            let __refeq_sr = &(Util::tuple21((cache).head().cloned()?));
            metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.classes), &(__refeq_sr.classes))
                && (match (&(__refeq_sl.within_), &(__refeq_sr.within_)) {
                    (Absyn::Within::TOP, Absyn::Within::TOP) => true,
                    (Absyn::Within::WITHIN { path: __refeq_v0l }, Absyn::Within::WITHIN { path: __refeq_v0r }) => {
                        referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                    }
                    _ => false,
                })
        }
    };
    if reuse {
        (program, top) = Util::tuple22((cache).head().cloned()?);
        NFInstNode::InstNode::clearGeneratedInners(top.clone())?;
        MutableWeak::useRoots(NFInstNode::InstNode::scopeRoots(top.clone())?);
    } else {
        if !((cache).is_empty()) {
            {
                let __v = metamodelica::nil();
                crate::Globals::instNFNodeCacheIndex.with(|__root| *__root.borrow_mut() = __v)
            };
        }
        (_, scode_builtin) = FBuiltin::getInitialFunctions()?;
        if (scodeProgram).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(scodeProgram) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            program = metamodelica::Own::own(__pa0);
        } else {
            program = AbsynToSCode::translateAbsyn2SCode(absynProgram.clone())?;
        }
        program = listAppend(scode_builtin, program);
        placementProgram = annotationProgram(Config::getAnnotationVersion()?)?;
        graphicProgramSCode = AbsynToSCode::translateAbsyn2SCode(placementProgram)?;
        NFInst::resetGlobalFlags()?;
        top = NFInst::makeTopNode(program.clone(), graphicProgramSCode)?;
        if Flags::isSet(Flags::EXEC_STAT.clone())? {
            execStat(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFInstanceAPI.mkTop("));
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }),
            )?;
        }
        {
            let __v = list![(absynProgram, (program.clone(), top.clone()))];
            crate::Globals::instNFNodeCacheIndex.with(|__root| *__root.borrow_mut() = __v)
        };
    }
    Ok((program, top))
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum InstanceTree {
    COMPONENT {
        node: metamodelica::Ref<InstNode::InstNode>,
        binding: Option<metamodelica::Ref<Binding::NFBinding>>,
        cls: metamodelica::Ref<InstanceTree>,
    },
    CLASS {
        node: metamodelica::Ref<InstNode::InstNode>,
        elements: metamodelica::List<metamodelica::Ref<InstanceTree>>,
        isExtends: bool,
    },
    BUILTIN_BASE_CLASS {
        name: ArcStr,
    },
    EMPTY,
}
impl metamodelica::gc::MMTrace for InstanceTree {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            InstanceTree::COMPONENT { node, binding, cls } => {
                metamodelica::gc::MMTrace::mm_accept(node, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cls, __mmv)?;
                Ok(())
            }
            InstanceTree::CLASS {
                node,
                elements,
                isExtends,
            } => {
                metamodelica::gc::MMTrace::mm_accept(node, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isExtends, __mmv)?;
                Ok(())
            }
            InstanceTree::BUILTIN_BASE_CLASS { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
            InstanceTree::EMPTY => Ok(()),
        }
    }
}
impl InstanceTree {
    pub fn interned_EMPTY() -> metamodelica::Ref<InstanceTree> {
        thread_local! {
            static INTERNED: metamodelica::Ref<InstanceTree> = metamodelica::Ref::new(InstanceTree::EMPTY);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_EMPTY() -> metamodelica::Ref<InstanceTree> {
    InstanceTree::interned_EMPTY()
}
impl Default for InstanceTree {
    fn default() -> Self {
        Self::EMPTY
    }
}
pub(crate) use self::InstanceTree::{BUILTIN_BASE_CLASS, CLASS, COMPONENT, EMPTY};

thread_local! { static __ENUM_BASE_TLS: metamodelica::Ref<InstanceTree> = metamodelica::Ref::new(InstanceTree::BUILTIN_BASE_CLASS { name: literal!("enumeration") }); }
pub(crate) fn ENUM_BASE() -> metamodelica::Ref<InstanceTree> {
    __ENUM_BASE_TLS.with(|__t| __t.clone())
}

pub fn buildModelInstanceJSON(
    mut absynProgram: Absyn::Program,
    mut scodeProgram: Option<metamodelica::List<metamodelica::Ref<SCode::Element>>>,
    mut classPath: metamodelica::Ref<Path>,
    mut contextPath: metamodelica::Ref<Path>,
    mut modifier: &ArcStr,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut top: metamodelica::Ref<InstNode::InstNode>;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut context: i32;
    let mut inst_tree: metamodelica::Ref<InstanceTree>;
    let mut inst_settings: metamodelica::Ref<InstSettings::InstSettings>;
    let mut r#mod: metamodelica::Ref<Modifier::Modifier>;
    context = NFInstContext::set(NFInstContext::RELAXED.clone(), NFInstContext::CLASS.clone());
    context = NFInstContext::set(context, NFInstContext::INSTANCE_API.clone());
    inst_settings = metamodelica::Ref::new(InstSettings::InstSettings {
        mergeExtendsSections: false,
        resizableArrays: false,
    });
    (_, top) = mkTop(
        absynProgram,
        scodeProgram,
        &(AbsynUtil::pathString(classPath.clone(), literal!("."), true, false)?),
    )?;
    r#mod = parseModifier(modifier, top.clone());
    cls_node = NFInst::lookupRootClass(classPath, top, context)?;
    if SCodeUtil::isFunction(&(NFInstNode::InstNode::definition(cls_node.clone())?)) {
        context = NFInstContext::unset(context, NFInstContext::CLASS.clone());
        context = NFInstContext::set(context, NFInstContext::FUNCTION.clone());
    }
    if !metamodelica::stringEq(&(AbsynUtil::pathFirstIdent(&contextPath)), &(literal!("__NoContext"))) {
        cls_node = NFInstNode::InstNode::setNodeType(
            metamodelica::Ref::new(InstNodeType::ROOT_CLASS {
                parent: NFInstNode::NO_SCOPE().clone(),
                context: Some(contextPath),
            }),
            cls_node,
        )?;
    }
    cls_node = NFInst::instantiateRootClass(cls_node, context, r#mod)?;
    execStat(&(literal!("Inst.instantiateRootClass")))?;
    inst_tree = buildInstanceTree(cls_node.clone(), false)?;
    execStat(&(literal!("NFInstanceAPI.buildInstanceTree")))?;
    NFInst::instExpressions(
        cls_node.clone(),
        &(cls_node.clone()),
        openmodelica_nf_frontend::NFSections::interned_EMPTY(),
        &(NFConnectBreakTree::new()),
        context,
        &inst_settings,
    )?;
    NFInst::updateImplicitVariability(cls_node.clone(), Flags::isSet(Flags::EVAL_PARAM.clone())?, context)?;
    execStat(&(literal!("Inst.instExpressions")))?;
    Typing::typeClassType(
        cls_node.clone(),
        &(Binding::EMPTY_BINDING().clone()),
        context,
        &(cls_node.clone()),
    )?;
    Typing::typeComponents(cls_node.clone(), context, false)?;
    execStat(&(literal!("Typing.typeComponents")))?;
    Typing::typeBindings(cls_node.clone(), context)?;
    execStat(&(literal!("Typing.typeBinding")))?;
    json = dumpJSONInstanceTree(&inst_tree, &cls_node, true, false, false)?;
    execStat(&(literal!("NFInstanceAPI.dumpJSONInstanceTree")))?;
    Ok(json)
}

pub fn buildModelInstanceAnnotationJSON(
    mut absynProgram: Absyn::Program,
    mut scodeProgram: Option<metamodelica::List<metamodelica::Ref<SCode::Element>>>,
    mut classPath: metamodelica::Ref<Path>,
    mut filter: &metamodelica::List<ArcStr>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut top: metamodelica::Ref<InstNode::InstNode>;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut context: i32;
    context = NFInstContext::set(NFInstContext::RELAXED.clone(), NFInstContext::CLASS.clone());
    context = NFInstContext::set(context, NFInstContext::INSTANCE_API.clone());
    (_, top) = mkTop(
        absynProgram,
        scodeProgram,
        &(AbsynUtil::pathString(classPath.clone(), literal!("."), true, false)?),
    )?;
    cls_node = NFInst::lookupRootClass(classPath, top, context)?;
    cls_node = NFInstNode::InstNode::resolveInner(cls_node);
    json = dumpJSONInstanceAnnotation(cls_node, filter, false, false)?;
    Ok(json)
}

pub fn buildModelInstanceIconJSON(
    mut absynProgram: Absyn::Program,
    mut scodeProgram: Option<metamodelica::List<metamodelica::Ref<SCode::Element>>>,
    mut classPath: metamodelica::Ref<Path>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut top: metamodelica::Ref<InstNode::InstNode>;
    (_, top) = mkTop(
        absynProgram,
        scodeProgram,
        &(AbsynUtil::pathString(classPath.clone(), literal!("."), true, false)?),
    )?;
    json = iconJSONFromTop(top, classPath)?;
    Ok(json)
}

pub fn diagramJSONFromTop(
    mut top: metamodelica::Ref<InstNode::InstNode>,
    mut classPath: metamodelica::Ref<Path>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut context: i32;
    let mut components: metamodelica::Ref<JSON::JSON> = JSON::emptyArray(0);
    let mut connections: metamodelica::Ref<JSON::JSON> = JSON::emptyArray(0);
    context = NFInstContext::set(NFInstContext::RELAXED.clone(), NFInstContext::CLASS.clone());
    context = NFInstContext::set(context, NFInstContext::INSTANCE_API.clone());
    cls_node = NFInst::lookupRootClass(classPath, top, context)?;
    cls_node = NFInstNode::InstNode::resolveInner(cls_node);
    json = dumpJSONInstanceAnnotation(cls_node.clone(), &(list![literal!("Diagram")]), false, true)?;
    (components, connections) = dumpJSONDiagramParts(cls_node, context, components, connections, 0)?;
    json = JSON::addPair(&(literal!("components")), &components, json)?;
    json = JSON::addPair(&(literal!("connections")), &connections, json)?;
    Ok(json)
}

pub(crate) fn dumpJSONDiagramParts(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut components: metamodelica::Ref<JSON::JSON>,
    mut connections: metamodelica::Ref<JSON::JSON>,
    mut depth: i32,
) -> Result<(metamodelica::Ref<JSON::JSON>, metamodelica::Ref<JSON::JSON>)> {
    let mut components: metamodelica::Ref<JSON::JSON> = components;
    let mut connections: metamodelica::Ref<JSON::JSON> = connections;
    let mut ty_node: metamodelica::Ref<InstNode::InstNode>;
    let mut e: metamodelica::Ref<JSON::JSON>;
    let mut def: metamodelica::Ref<SCode::Element>;
    let mut eqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    if depth > 16 {
        return Ok((components, connections));
    }
    let __range0 = ClassTree::getExtends(&(Class::classTree(NFInstNode::InstNode::getClass(node.clone())?)?))
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut ext in __range0 {
        (components, connections) = dumpJSONDiagramParts(ext, context, components, connections, depth + 1)?;
    }
    def = NFInstNode::InstNode::definition(node.clone())?;
    for mut el in &*SCodeUtil::getClassElements(&def) {
        let () = (match &*el.clone() {
            SCode::Element::COMPONENT {
                comment: __el_comment,
                info: __el_info,
                name: __el_name,
                typeSpec: __el_typeSpec,
                ..
            } => {
                if '__try0: {
                (ty_node, _) = unwrap_break_err!(Lookup::lookupClassName(AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&__el_typeSpec)), node.clone(), context, __el_info.clone(), true), '__try0);
                ty_node = unwrap_break_err!(NFInst::expand(ty_node.clone(), context), '__try0);
                e = unwrap_break_err!(JSON::addPair(&(literal!("$kind")), &(metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("component") })), JSON::makeNull()), '__try0);
                e = unwrap_break_err!(JSON::addPair(&(literal!("name")), &(JSON::makeString(__el_name.clone())), e.clone()), '__try0);
                e = unwrap_break_err!(dumpJSONAnnotationOpt(__el_comment.annotation_.clone(), &node, &(list![literal!("Placement")]), false, e.clone()), '__try0);
                e = unwrap_break_err!(JSON::addPair(&(literal!("type")), &(unwrap_break_err!(diagramComponentIcon(ty_node.clone()), '__try0)), e.clone()), '__try0);
                components = unwrap_break_err!(JSON::addElement(&e, components.clone()), '__try0);
                Ok::<(), &'static str>(())
            }.is_err() {
            }
                ()
            }
            _ => (),
        });
    }
    eqs = (::match_deref::match_deref! { match &(def) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { normalEquationLst: __esc_eqs, .. }, .. } => {
            eqs = (*__esc_eqs).clone();
            eqs.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    for mut eq in &*eqs {
        let () = (match &*eq.clone() {
            SCode::Equation::EQ_CONNECT {
                comment: __eq_comment, ..
            } => {
                e = dumpJSONAnnotationOpt(
                    __eq_comment.annotation_.clone(),
                    &node,
                    &(list![literal!("Line")]),
                    false,
                    JSON::makeNull(),
                )?;
                if !(JSON::isNull(&e)) {
                    connections = JSON::addElement(&e, connections)?;
                }
                ()
            }
            _ => (),
        });
    }
    Ok((components, connections))
}

pub fn builtinSCode() -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    (_, program) = FBuiltin::getInitialFunctions()?;
    Ok(program)
}

pub fn builtinAbsyn() -> Result<Absyn::Program> {
    let mut program: Absyn::Program;
    (program, _) = FBuiltin::getInitialFunctions()?;
    Ok(program)
}

pub fn programSCode(mut absynProgram: Absyn::Program) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    program = AbsynToSCode::translateAbsyn2SCode(absynProgram)?;
    Ok(program)
}

pub fn topFromSCode(
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut top: metamodelica::Ref<InstNode::InstNode>;
    let mut graphicProgramSCode: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    graphicProgramSCode = AbsynToSCode::translateAbsyn2SCode(annotationProgram(Config::getAnnotationVersion()?)?)?;
    NFInst::resetGlobalFlags()?;
    top = NFInst::makeTopNode(program, graphicProgramSCode)?;
    Ok(top)
}

pub(crate) fn diagramComponentIcon(
    mut ty_node: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut cached: Option<metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<JSON::JSON>>>>;
    let mut cache: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<JSON::JSON>>>;
    let mut key: ArcStr;
    cached = crate::Globals::nfDiagramIconCache.with(|__root| __root.borrow().clone());
    cache = (::match_deref::match_deref! { match &(cached) {
        Some(__esc_cache) => {
            cache = (*__esc_cache).clone();
            cache.clone()
        },
        _ => {
            cache = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1);
            { let __v = Some(cache.clone()); crate::Globals::nfDiagramIconCache.with(|__root| *__root.borrow_mut() = __v) };
            cache
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    key = AbsynUtil::pathString(
        NFInstNode::InstNode::fullPath(ty_node.clone(), false)?,
        literal!("."),
        true,
        false,
    )?;
    json = (::match_deref::match_deref! { match &(UnorderedMap::get(key.clone(), cache.clone())?) {
        Some(__esc_json) => {
            json = (*__esc_json).clone();
            json.clone()
        },
        _ => {
            json = dumpJSONInstanceAnnotation(ty_node, &(list![literal!("Icon")]), false, true)?;
            UnorderedMap::add(key, json.clone(), cache)?;
            json
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn resolveNamesFromTop(
    mut top: metamodelica::Ref<InstNode::InstNode>,
    mut classPath: metamodelica::Ref<Path>,
) -> Result<(
    metamodelica::List<(ArcStr, ArcStr)>,
    metamodelica::List<(ArcStr, ArcStr)>,
)> {
    let mut bases: metamodelica::List<(ArcStr, ArcStr)> = metamodelica::nil();
    let mut componentTypes: metamodelica::List<(ArcStr, ArcStr)> = metamodelica::nil();
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut context: i32;
    let mut def: metamodelica::Ref<SCode::Element>;
    let mut cdef: metamodelica::Ref<SCode::ClassDef>;
    let mut reference: metamodelica::Ref<Path>;
    context = NFInstContext::set(NFInstContext::RELAXED.clone(), NFInstContext::CLASS.clone());
    context = NFInstContext::set(context, NFInstContext::INSTANCE_API.clone());
    match '__try0: {
        cls_node = unwrap_break_err!(NFInst::lookupRootClass(classPath.clone(), top.clone(), context), '__try0);
        cls_node = NFInstNode::InstNode::resolveInner(cls_node.clone());
        cls_node = unwrap_break_err!(NFInst::expand(cls_node.clone(), context), '__try0);
        def = unwrap_break_err!(NFInstNode::InstNode::definition(cls_node.clone()), '__try0);
        Ok::<_, &'static str>((cls_node.clone(), def.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            cls_node = __try0_o0;
            def = __try0_o1;
        }
        Err(_) => {
            return Ok((bases, componentTypes));
        }
    }
    cdef = SCodeUtil::getClassDef(&def)?;
    let () = (match &*cdef {
        SCode::ClassDef::DERIVED {
            typeSpec: __cdef_typeSpec,
            ..
        } => {
            reference = AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&__cdef_typeSpec));
            bases = metamodelica::cons(
                resolveOne(reference, cls_node.clone(), context, SCodeUtil::elementInfo(&def))?,
                bases,
            );
            ()
        }
        _ => (),
    });
    for mut el in &*SCodeUtil::getClassElements(&def) {
        let () = (match &*el.clone() {
            SCode::Element::EXTENDS {
                baseClassPath: __el_baseClassPath,
                info: __el_info,
                ..
            } => {
                bases = metamodelica::cons(
                    resolveOne(__el_baseClassPath.clone(), cls_node.clone(), context, __el_info.clone())?,
                    bases,
                );
                ()
            }
            SCode::Element::COMPONENT {
                info: __el_info,
                name: __el_name,
                typeSpec: __el_typeSpec,
                ..
            } => {
                reference = AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&__el_typeSpec));
                componentTypes = metamodelica::cons(
                    (
                        __el_name.clone(),
                        Util::tuple22(resolveOne(reference, cls_node.clone(), context, __el_info.clone())?),
                    ),
                    componentTypes,
                );
                ()
            }
            _ => (),
        });
    }
    bases = metamodelica::Dangerous::listReverseInPlace(bases);
    componentTypes = metamodelica::Dangerous::listReverseInPlace(componentTypes);
    Ok((bases, componentTypes))
}

pub(crate) fn resolveOne(
    mut reference: metamodelica::Ref<Path>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(ArcStr, ArcStr)> {
    let mut resolved: (ArcStr, ArcStr);
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut qualified: ArcStr;
    match '__try0: {
        (node, _) = unwrap_break_err!(Lookup::lookupClassName(reference.clone(), scope.clone(), context, info.clone(), true), '__try0);
        node = unwrap_break_err!(NFInst::expand(node.clone(), context), '__try0);
        qualified = unwrap_break_err!(AbsynUtil::pathString(unwrap_break_err!(NFInstNode::InstNode::fullPath(node.clone(), false), '__try0), literal!("."), true, false), '__try0);
        Ok::<_, &'static str>((qualified.clone(),))
    } {
        Ok((__try0_o0,)) => {
            qualified = __try0_o0;
        }
        Err(_) => {
            qualified = literal!("");
        }
    }
    resolved = (AbsynUtil::pathString(reference, literal!("."), true, false)?, qualified);
    Ok(resolved)
}

pub fn clearTopScopeCache() -> () {
    {
        let __v = metamodelica::nil();
        crate::Globals::instNFNodeCacheIndex.with(|__root| *__root.borrow_mut() = __v)
    };
    {
        let __v = metamodelica::nil();
        openmodelica_nf_frontend::Globals::nfTopScope.with(|__root| *__root.borrow_mut() = __v)
    };
    {
        let __v = None;
        crate::Globals::nfDiagramIconCache.with(|__root| *__root.borrow_mut() = __v)
    };
    MutableWeak::clearRoots();
    ()
}

pub fn iconJSONFromTop(
    mut top: metamodelica::Ref<InstNode::InstNode>,
    mut classPath: metamodelica::Ref<Path>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut context: i32;
    context = NFInstContext::set(NFInstContext::RELAXED.clone(), NFInstContext::CLASS.clone());
    context = NFInstContext::set(context, NFInstContext::INSTANCE_API.clone());
    cls_node = NFInst::lookupRootClass(classPath, top, context)?;
    cls_node = NFInstNode::InstNode::resolveInner(cls_node);
    json = dumpJSONInstanceAnnotation(cls_node, &(list![literal!("Icon")]), true, true)?;
    Ok(json)
}

pub fn storeModelInstanceReference(mut json: metamodelica::Ref<JSON::JSON>) -> i32 {
    let mut handle: i32 = 0;
    handle = openmodelica_util::ModelInstanceReference::store(json.clone());
    handle
}

pub fn releaseModelInstanceReferenceImpl(mut handle: i32) -> bool {
    let mut success: bool = false;
    success = openmodelica_util::ModelInstanceReference::release(handle.clone());
    success
}

pub(crate) fn parseModifier(
    mut modifierValue: &ArcStr,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
) -> metamodelica::Ref<Modifier::Modifier> {
    let mut outMod: metamodelica::Ref<Modifier::Modifier>;
    let mut amod: metamodelica::Ref<Absyn::Modification>;
    let mut smod: metamodelica::Ref<SCode::Mod>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(Parser::stringMod({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("dummy")); __mm_s.push_str(&*modifierValue); ArcStr::from(__mm_s) }, literal!("<internal>")), '__try0)) {
            Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(__pa1), .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        amod = metamodelica::Own::own(__pa1);
        smod = unwrap_break_err!(AbsynToSCode::translateMod(Some(amod.clone()), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, Absyn::dummyInfo.clone(), false), '__try0);
        outMod = unwrap_break_err!(NFModifier::Modifier::create(&smod, literal!(""), &(metamodelica::Ref::new(NFModifier::ModifierScope::ModifierScope::COMPONENT { name: literal!("") })), scope.clone(), 0), '__try0);
        Ok::<_, &'static str>((outMod.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outMod = __try0_o0;
        }
        Err(_) => {
            outMod = openmodelica_nf_frontend::NFModifier::Modifier::interned_NOMOD();
        }
    }
    outMod
}

pub(crate) fn buildInstanceTree(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut isDerived: bool,
) -> Result<metamodelica::Ref<InstanceTree>> {
    let mut tree: metamodelica::Ref<InstanceTree>;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    let mut elems: metamodelica::List<metamodelica::Ref<InstanceTree>>;
    cls_node = NFInstNode::InstNode::resolveInner(node.clone());
    cls = NFInstNode::InstNode::getClass(cls_node.clone())?;
    if !(isDerived) && Class::isOnlyBuiltin(&cls) && !(Class::isEnumeration(cls.clone())?) {
        tree = crate::NFInstanceAPI::InstanceTree::interned_EMPTY();
        return Ok(tree);
    }
    cls_tree = Class::classTree(cls.clone())?;
    tree = (::match_deref::match_deref! { match &((cls.clone(), cls_tree.clone())) {
        (Deref @ Class::EXPANDED_DERIVED { .. }, _) => {
            elems = list![buildInstanceTree(var_field!((*cls).baseClass, Class::NFClass::EXPANDED_DERIVED).clone(), true)?];
            metamodelica::Ref::new(InstanceTree::CLASS { node: node, elements: elems, isExtends: isDerived })
        },
        (_, Deref @ ClassTree::INSTANTIATED_TREE { .. }) => {
            elems = buildInstanceTreeElements(&(NFInstNode::InstNode::definition(cls_node)?), &cls_tree)?;
            if NFInstNode::InstNode::isRootClass(&node) {
                elems = buildInstanceTreeGeneratedInners(&cls_tree, elems)?;
            }
            metamodelica::Ref::new(InstanceTree::CLASS { node: node, elements: elems, isExtends: isDerived })
        },
        (_, Deref @ ClassTree::FLAT_TREE { .. }) => metamodelica::Ref::new(InstanceTree::CLASS { node: node, elements: if (NFInstNode::InstNode::isEnumerationType(cls_node)?) {list![ENUM_BASE().clone()]} else {metamodelica::nil()}, isExtends: isDerived }),
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstanceAPI.buildInstanceTree")); __mm_s.push_str(&*literal!(" got unknown class tree")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFInstanceAPI.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tree)
}

pub(crate) fn buildInstanceTreeElements(
    mut classDefinition: &metamodelica::Ref<SCode::Element>,
    mut classTree: &metamodelica::Ref<ClassTree::ClassTree>,
) -> Result<metamodelica::List<metamodelica::Ref<InstanceTree>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<InstanceTree>> = metamodelica::nil();
    let mut scode_elems: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut clss: metamodelica::Array<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>;
    let mut comps: metamodelica::Array<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>;
    let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut cls_index: i32 = 1;
    let mut comp_index: i32 = 1;
    let mut ext_index: i32 = 1;
    let mut tree: metamodelica::Ref<InstanceTree>;
    let mut local_comps: metamodelica::List<i32>;
    let mut node: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::EMPTY_NODE);
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*classTree)) {
        Deref @ ClassTree::INSTANTIATED_TREE { classes: __pa0, components: __pa1, exts: __pa2, localComponents: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    clss = metamodelica::Own::own(__pa0);
    comps = metamodelica::Own::own(__pa1);
    exts = metamodelica::Own::own(__pa2);
    local_comps = metamodelica::Own::own(__pa3);
    scode_elems = SCodeUtil::getClassElements(classDefinition);
    if !((local_comps).is_empty()) {
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(local_comps) {
            Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        comp_index = metamodelica::Own::own(__pa4);
        local_comps = metamodelica::Own::own(__pa5);
    }
    for mut e in &*scode_elems {
        elements = (match &*e.clone() {
            SCode::Element::EXTENDS { .. } => {
                tree = buildInstanceTree(
                    ({
                        let __elt = (*metamodelica::index_checked(&exts.borrow(), ext_index)?).clone();
                        __elt
                    }),
                    true,
                )?;
                ext_index = ext_index + 1;
                metamodelica::cons(tree, elements)
            }
            SCode::Element::CLASS { name: __e_name, .. }
                if (SCodeUtil::isElementReplaceable(metamodelica::AsArg::as_arg(&e))?) =>
            {
                while !metamodelica::stringEq(
                    &(NFInstNode::InstNode::name(
                        &(Mutable::access(
                            ({
                                let __elt = (*metamodelica::index_checked(&clss.borrow(), cls_index)?).clone();
                                __elt
                            }),
                        )),
                    )?),
                    &__e_name,
                ) {
                    cls_index = cls_index + 1;
                }
                tree = metamodelica::Ref::new(InstanceTree::CLASS {
                    node: Mutable::access(
                        ({
                            let __elt = (*metamodelica::index_checked(&clss.borrow(), cls_index)?).clone();
                            __elt
                        }),
                    ),
                    elements: metamodelica::nil(),
                    isExtends: false,
                });
                cls_index = cls_index + 1;
                metamodelica::cons(tree, elements)
            }
            SCode::Element::COMPONENT { name: __e_name, .. } => {
                loop {
                    node = Mutable::access(
                        ({
                            let __elt = (*metamodelica::index_checked(&comps.borrow(), comp_index)?).clone();
                            __elt
                        }),
                    );
                    if metamodelica::stringEq(&(NFInstNode::InstNode::name(&node)?), &__e_name)
                        && !(NFInstNode::InstNode::isGeneratedInner(&node))
                    {
                        break;
                    }
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(local_comps) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    comp_index = metamodelica::Own::own(__pa0);
                    local_comps = metamodelica::Own::own(__pa1);
                }
                tree = buildInstanceTreeComponent(node.clone())?;
                elements = metamodelica::cons(tree, elements);
                if !((local_comps).is_empty()) {
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(local_comps) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    comp_index = metamodelica::Own::own(__pa2);
                    local_comps = metamodelica::Own::own(__pa3);
                }
                elements
            }
            _ => elements,
        });
    }
    elements = metamodelica::Dangerous::listReverseInPlace(elements);
    Ok(elements)
}

pub(crate) fn buildInstanceTreeGeneratedInners(
    mut classTree: &metamodelica::Ref<ClassTree::ClassTree>,
    mut elements: metamodelica::List<metamodelica::Ref<InstanceTree>>,
) -> Result<metamodelica::List<metamodelica::Ref<InstanceTree>>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<InstanceTree>>;
    let mut comps: metamodelica::Array<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>;
    let mut elems: metamodelica::List<metamodelica::Ref<InstanceTree>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &((*classTree)) {
        Deref @ ClassTree::INSTANTIATED_TREE { components: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    comps = metamodelica::Own::own(__pa0);
    for mut i in ({
        let __s = metamodelica::arrayLength(comps.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        if NFInstNode::InstNode::isGeneratedInner(
            &(Mutable::access(
                ({
                    let __elt = (*metamodelica::index_checked(&comps.borrow(), i)?).clone();
                    __elt
                }),
            )),
        ) {
            elems = metamodelica::cons(
                buildInstanceTreeComponent(Mutable::access(
                    ({
                        let __elt = (*metamodelica::index_checked(&comps.borrow(), i)?).clone();
                        __elt
                    }),
                ))?,
                elems,
            );
        } else {
            break;
        }
    }
    if (elems).is_empty() {
        outElements = elements;
    } else {
        outElements = listAppend(elements, elems);
    }
    Ok(outElements)
}

pub(crate) fn buildInstanceTreeComponent(
    mut node: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstanceTree>> {
    let mut tree: metamodelica::Ref<InstanceTree>;
    let mut inner_node: metamodelica::Ref<InstNode::InstNode>;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<InstanceTree>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut opt_binding: Option<metamodelica::Ref<Binding::NFBinding>>;
    inner_node = NFInstNode::InstNode::resolveOuter(node.clone());
    cls_node = NFInstNode::InstNode::classScope(inner_node.clone())?;
    if NFInstNode::InstNode::isEmpty(&cls_node) {
        cls = crate::NFInstanceAPI::InstanceTree::interned_EMPTY();
    } else {
        cls = buildInstanceTree(cls_node, false)?;
    }
    if NFInstNode::InstNode::isComponent(&inner_node)? {
        binding = Component::getBinding(&(NFInstNode::InstNode::component(&inner_node)?));
        opt_binding = if (Binding::isBound(&binding)) {
            Some(binding)
        } else {
            None
        };
    } else {
        opt_binding = None;
    }
    tree = metamodelica::Ref::new(InstanceTree::COMPONENT {
        node: node,
        binding: opt_binding,
        cls: cls,
    });
    Ok(tree)
}

pub(crate) fn dumpJSONInstanceTree(
    mut tree: &metamodelica::Ref<InstanceTree>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut root: bool,
    mut isDeleted: bool,
    mut isExtends: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut elems: metamodelica::List<metamodelica::Ref<InstanceTree>>;
    let mut sections: metamodelica::Ref<Sections::NFSections>;
    let mut cmt: Option<metamodelica::Ref<SCode::Comment>>;
    let mut def: metamodelica::Ref<SCode::Element>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*tree)) {
        Deref @ InstanceTree::CLASS { node: __pa0, elements: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    node = metamodelica::Own::own(__pa0);
    elems = metamodelica::Own::own(__pa1);
    node = NFInstNode::InstNode::resolveOuter(node);
    def = NFInstNode::InstNode::definition(node.clone())?;
    cmt = SCodeUtil::getElementComment(&def);
    json = JSON::addPair(&(literal!("name")), &(dumpJSONNodePath(node.clone(), false)?), json)?;
    json = JSON::addPairNotNull(&(literal!("dims")), &(dumpJSONClassDims(node.clone(), &def)?), json)?;
    json = JSON::addPair(
        &(literal!("restriction")),
        &(JSON::makeString(SCodeDump::restrictionStringPP(SCodeUtil::getClassRestriction(&def)?)?)),
        json,
    )?;
    json = JSON::addPairNotNull(
        &(literal!("prefixes")),
        &(dumpJSONClassPrefixes(&def, &(NFInstNode::InstNode::parent(&node)?))?),
        json,
    )?;
    json = dumpJSONCommentOpt(cmt, scope, json, true, true, false)?;
    json = JSON::addPairNotNull(
        &(literal!("elements")),
        &(dumpJSONElements(&elems, node.clone(), isDeleted)?),
        json,
    )?;
    if !(isDeleted) {
        json = dumpJSONImports(node.clone(), json)?;
        sections = Class::getSections(NFInstNode::InstNode::getClass(node.clone())?)?;
        json = dumpJSONEquations(&sections, &node, json)?;
    }
    json = JSON::addPair(
        &(literal!("source")),
        &(JSON::dumpJSONSourceInfo(&(NFInstNode::InstNode::info(&node)), true)?),
        json,
    )?;
    Ok(json)
}

pub(crate) fn dumpJSONInstanceAnnotation(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut filter: &metamodelica::List<ArcStr>,
    mut dumpConnectors: bool,
    mut dumpDerivedBase: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    let mut cmt: Option<metamodelica::Ref<SCode::Comment>>;
    let mut ann: metamodelica::Ref<SCode::Annotation>;
    let mut j: metamodelica::Ref<JSON::JSON>;
    let mut scope: metamodelica::Ref<InstNode::InstNode> = node.clone();
    let mut context: i32;
    let mut annotation_is_literal: bool = true;
    let mut any_element: bool = false;
    let mut def: metamodelica::Ref<SCode::Element>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    NFInst::expand(node.clone(), NFInstContext::RELAXED.clone())?;
    def = NFInstNode::InstNode::definition(node.clone())?;
    json = JSON::addPair(&(literal!("name")), &(dumpJSONNodePath(node.clone(), false)?), json)?;
    json = JSON::addPair(
        &(literal!("restriction")),
        &(JSON::makeString(Restriction::toString(
            &(NFInstNode::InstNode::restriction(node.clone())?),
        ))),
        json,
    )?;
    json = JSON::addPairNotNull(
        &(literal!("prefixes")),
        &(dumpJSONClassPrefixes(&def, &(NFInstNode::InstNode::parent(&node)?))?),
        json,
    )?;
    cls = NFInstNode::InstNode::getClass(node.clone())?;
    j = JSON::emptyArray(0);
    if dumpDerivedBase {
        let () = (match &*cls {
            Class::EXPANDED_DERIVED {
                baseClass: __cls_baseClass,
                ..
            } => {
                j = JSON::addElement(
                    &(dumpJSONInstanceAnnotationExtends(__cls_baseClass.clone(), filter, true)?),
                    j,
                )?;
                any_element = true;
                ()
            }
            Class::TYPED_DERIVED {
                baseClass: __cls_baseClass,
                ..
            } => {
                j = JSON::addElement(
                    &(dumpJSONInstanceAnnotationExtends(__cls_baseClass.clone(), filter, true)?),
                    j,
                )?;
                any_element = true;
                ()
            }
            _ => (),
        });
    }
    if !(any_element) {
        let __range0 = ClassTree::getExtends(&(Class::classTree(cls)?))
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for mut ext in __range0 {
            j = JSON::addElement(&(dumpJSONInstanceAnnotationExtends(ext, filter, dumpDerivedBase)?), j)?;
            any_element = true;
        }
    }
    if dumpConnectors {
        (j, any_element) = dumpJSONInstanceAnnotationConnectors(node.clone(), j, any_element)?;
    }
    if any_element {
        json = JSON::addPair(&(literal!("elements")), &j, json)?;
    }
    cmt = SCodeUtil::getElementComment(&(NFInstNode::InstNode::definition(node)?));
    cmt = (::match_deref::match_deref! { match &(cmt) {
        Some(Deref @ SCode::Comment { annotation_: Some(__esc_ann @ Deref @ SCode::Annotation { .. }), .. }) => {
            ann = (*__esc_ann).clone();
            if !((filter).is_empty()) {
                assign_field!(ann.modification = SCodeUtil::filterSubMods(ann.modification.clone(), &({ let __pe_b1 = filter.clone(); move |__pe_a0| Ok(SCodeUtil::filterGivenSubModNames(&__pe_a0, __pe_b1.clone())) }))?);
            }
            annotation_is_literal = SCodeUtil::onlyLiteralsInMod(&ann.modification)?;
            if (SCodeUtil::isEmptyMod(&ann.modification)) {None} else {Some(metamodelica::Ref::new(SCode::Comment { annotation_: Some(ann.clone()), comment: None }))}
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(annotation_is_literal) {
        ErrorExt::setCheckpoint(literal!("NFInstanceAPI.dumpJSONInstanceAnnotation"));
        if '__try1: {
            context = NFInstContext::set(NFInstContext::CLASS.clone(), NFInstContext::RELAXED.clone());
            scope = unwrap_break_err!(NFInstNode::InstNode::makeRootClass(scope.clone(), openmodelica_nf_frontend::NFInstNode::InstNode::interned_EMPTY_NODE(), None), '__try1);
            scope = unwrap_break_err!(NFInst::instantiate(scope.clone(), openmodelica_nf_frontend::NFModifier::Modifier::interned_NOMOD(), openmodelica_nf_frontend::NFInstNode::InstNode::interned_EMPTY_NODE(), context, true), '__try1);
            unwrap_break_err!(NFInst::insertGeneratedInners(scope.clone(), &(unwrap_break_err!(NFInstNode::InstNode::topScope(scope.clone()), '__try1)), context), '__try1);
            unwrap_break_err!(NFInst::instExpressions(scope.clone(), &(scope.clone()), openmodelica_nf_frontend::NFSections::interned_EMPTY(), &(NFConnectBreakTree::new()), context, &(NFInst::DEFAULT_SETTINGS.clone())), '__try1);
            Ok::<(), &'static str>(())
        }.is_err() {
        }
        ErrorExt::rollBack(literal!("NFInstanceAPI.dumpJSONInstanceAnnotation"));
    }
    json = dumpJSONCommentOpt(cmt, &scope, json, true, true, true)?;
    Ok(json)
}

pub(crate) fn dumpJSONInstanceAnnotationConnectors(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut json: metamodelica::Ref<JSON::JSON>,
    mut any: bool,
) -> Result<(metamodelica::Ref<JSON::JSON>, bool)> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    let mut any: bool = any;
    let mut context: i32 = NFInstContext::set(NFInstContext::RELAXED.clone(), NFInstContext::CLASS.clone());
    let mut ty_node: metamodelica::Ref<InstNode::InstNode>;
    let mut e: metamodelica::Ref<JSON::JSON>;
    for mut el in &*SCodeUtil::getClassElements(&(NFInstNode::InstNode::definition(node.clone())?)) {
        let () = (match &*el.clone() {
            SCode::Element::COMPONENT {
                comment: __el_comment,
                info: __el_info,
                name: __el_name,
                typeSpec: __el_typeSpec,
                ..
            } => {
                if '__try0: {
                (ty_node, _) = unwrap_break_err!(Lookup::lookupClassName(AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&__el_typeSpec)), node.clone(), context, __el_info.clone(), true), '__try0);
                if SCodeUtil::isConnector(&(unwrap_break_err!(SCodeUtil::getClassRestriction(&(unwrap_break_err!(NFInstNode::InstNode::definition(ty_node.clone()), '__try0))), '__try0))) {
                    ty_node = unwrap_break_err!(NFInst::expand(ty_node.clone(), context), '__try0);
                    e = unwrap_break_err!(JSON::addPair(&(literal!("$kind")), &(metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("component") })), JSON::makeNull()), '__try0);
                    e = unwrap_break_err!(JSON::addPair(&(literal!("name")), &(JSON::makeString(__el_name.clone())), e.clone()), '__try0);
                    e = unwrap_break_err!(dumpJSONAnnotationOpt(__el_comment.annotation_.clone(), &node, &(list![literal!("Placement")]), false, e.clone()), '__try0);
                    e = unwrap_break_err!(JSON::addPair(&(literal!("type")), &(unwrap_break_err!(dumpJSONInstanceAnnotation(ty_node.clone(), &(list![literal!("Icon")]), false, true), '__try0)), e.clone()), '__try0);
                    json = unwrap_break_err!(JSON::addElement(&e, json.clone()), '__try0);
                    any = true;
                }
                Ok::<(), &'static str>(())
            }.is_err() {
            }
                ()
            }
            _ => (),
        });
    }
    Ok((json, any))
}

pub(crate) fn dumpJSONInstanceAnnotationExtends(
    mut ext: metamodelica::Ref<InstNode::InstNode>,
    mut filter: &metamodelica::List<ArcStr>,
    mut dumpDerivedBase: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    json = JSON::addPair(
        &(literal!("$kind")),
        &(metamodelica::Ref::new(JSON::JSON::STRING {
            r#str: literal!("extends"),
        })),
        json,
    )?;
    json = JSON::addPair(
        &(literal!("baseClass")),
        &(dumpJSONInstanceAnnotation(ext, filter, false, dumpDerivedBase)?),
        json,
    )?;
    Ok(json)
}

pub(crate) fn dumpJSONNodePath(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut ignoreBaseClass: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = dumpJSONPath(NFInstNode::InstNode::enclosingScopePath(
        node.clone(),
        false,
        ignoreBaseClass,
    )?)?;
    Ok(json)
}

pub(crate) fn dumpJSONNodeEnclosingPath(
    mut node: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> =
        dumpJSONPath(NFInstNode::InstNode::enclosingScopePath(node.clone(), true, false)?)?;
    Ok(json)
}

pub(crate) fn dumpJSONPath(mut path: metamodelica::Ref<Path>) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> =
        JSON::makeString(AbsynUtil::pathString(path.clone(), literal!("."), true, false)?);
    Ok(json)
}

pub(crate) fn dumpJSONElements(
    mut elements: &metamodelica::List<metamodelica::Ref<InstanceTree>>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut isDeleted: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    let mut j: metamodelica::Ref<JSON::JSON>;
    if isDeleted {
        for mut e in &**elements {
            j = (match &*e.clone() {
                InstanceTree::CLASS { isExtends: true, .. } => {
                    dumpJSONExtends(metamodelica::AsArg::as_arg(&e), isDeleted)?
                }
                _ => JSON::makeNull(),
            });
            json = JSON::addElementNotNull(&j, json)?;
        }
    } else {
        for mut e in &**elements {
            j = (match &*e.clone() {
                InstanceTree::CLASS { isExtends: true, .. } => {
                    dumpJSONExtends(metamodelica::AsArg::as_arg(&e), isDeleted)?
                }
                InstanceTree::CLASS { node: __e_node, .. } => {
                    dumpJSONReplaceableClass(__e_node.clone(), scope.clone())?
                }
                InstanceTree::COMPONENT {
                    binding: __e_binding,
                    cls: __e_cls,
                    node: __e_node,
                } => dumpJSONComponent(
                    __e_node.clone(),
                    __e_binding.clone(),
                    metamodelica::AsArg::as_arg(&__e_cls),
                )?,
                InstanceTree::BUILTIN_BASE_CLASS { name: __e_name } => dumpJSONBuiltinBaseClass(__e_name.clone())?,
                _ => JSON::makeNull(),
            });
            json = JSON::addElementNotNull(&j, json)?;
        }
    }
    Ok(json)
}

pub(crate) fn dumpJSONExtends(
    mut ext: &metamodelica::Ref<InstanceTree>,
    mut isDeleted: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut cls_def: metamodelica::Ref<SCode::Element>;
    let mut ext_def: metamodelica::Ref<SCode::Element>;
    let __pa0 = ::match_deref::match_deref! { match &((*ext)) {
        Deref @ InstanceTree::CLASS { node: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    node = metamodelica::Own::own(__pa0);
    cls_def = NFInstNode::InstNode::definition(node.clone())?;
    let __pa1 = ::match_deref::match_deref! { match &(NFInstNode::InstNode::extendsDefinition(&node)?) {
        Some(__pa1) => __pa1.clone(),
        _ => return Err("pattern mismatch"),
    } };
    ext_def = metamodelica::Own::own(__pa1);
    json = JSON::addPair(
        &(literal!("$kind")),
        &(metamodelica::Ref::new(JSON::JSON::STRING {
            r#str: literal!("extends"),
        })),
        json,
    )?;
    json = dumpJSONSCodeMod(&(getExtendsModifier(&ext_def, node.clone())?), &(node.clone()), json)?;
    json = dumpJSONCommentOpt(SCodeUtil::getElementComment(&ext_def), &node, json, true, true, false)?;
    cls = NFInstNode::InstNode::getClass(node.clone())?;
    if Class::isOnlyBuiltin(&cls) && !(Class::isEnumeration(cls)?) {
        json = JSON::addPair(
            &(literal!("baseClass")),
            &(JSON::makeString(NFInstNode::InstNode::name(&node)?)),
            json,
        )?;
    } else {
        json = JSON::addPair(
            &(literal!("baseClass")),
            &(dumpJSONInstanceTree(ext, &node, false, isDeleted, true)?),
            json,
        )?;
    }
    Ok(json)
}

pub(crate) fn dumpJSONBuiltinBaseClass(mut name: ArcStr) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    json = JSON::addPair(
        &(literal!("$kind")),
        &(metamodelica::Ref::new(JSON::JSON::STRING {
            r#str: literal!("extends"),
        })),
        json,
    )?;
    json = JSON::addPair(&(literal!("baseClass")), &(JSON::makeString(name)), json)?;
    Ok(json)
}

pub(crate) fn getExtendsModifier(
    mut definition: &metamodelica::Ref<SCode::Element>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    r#mod = (match &**definition {
        SCode::Element::EXTENDS {
            modifications: __definition_modifications,
            ..
        } => __definition_modifications.clone(),
        SCode::Element::CLASS { .. } => SCodeUtil::elementMod(
            &(NFInstNode::InstNode::definition(NFInstNode::InstNode::getDerivedNode(node, false)?)?),
        ),
        _ => openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
    });
    Ok(r#mod)
}

pub(crate) fn dumpJSONReplaceableClass(
    mut cls: metamodelica::Ref<InstNode::InstNode>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    let mut elem: metamodelica::Ref<SCode::Element>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    node = NFInstNode::InstNode::getRedeclaredNode(cls);
    elem = NFInstNode::InstNode::definition(node.clone())?;
    json = dumpJSONSCodeClass(&elem, scope, node.clone(), true, json)?;
    json = JSON::addPair(
        &(literal!("source")),
        &(JSON::dumpJSONSourceInfo(&(NFInstNode::InstNode::info(&node)), true)?),
        json,
    )?;
    Ok(json)
}

pub(crate) fn dumpJSONComponent(
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut originalBinding: Option<metamodelica::Ref<Binding::NFBinding>>,
    mut cls: &metamodelica::Ref<InstanceTree>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut scope: metamodelica::Ref<InstNode::InstNode>;
    let mut ty_node: metamodelica::Ref<InstNode::InstNode>;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut elem: metamodelica::Ref<SCode::Element>;
    let mut is_constant: bool;
    let mut path: metamodelica::Ref<Path>;
    node = NFInstNode::InstNode::resolveOuter(component.clone());
    comp = NFInstNode::InstNode::component(&node)?;
    elem = NFInstNode::InstNode::definition(node.clone())?;
    scope = NFInstNode::InstNode::parent(&node)?;
    json = JSON::addPair(
        &(literal!("$kind")),
        &(metamodelica::Ref::new(JSON::JSON::STRING {
            r#str: literal!("component"),
        })),
        json,
    )?;
    json = JSON::addPair(
        &(literal!("name")),
        &(JSON::makeString(NFInstNode::InstNode::name(&node)?)),
        json,
    )?;
    let () = (::match_deref::match_deref! { match &((&*comp, &*elem)) {
        (Deref @ Component::COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { .. }) if (Component::isDeleted(&comp)?) => {
            json = JSON::addPair(&(literal!("type")), &(dumpJSONComponentType(cls, node, var_field!((*comp).ty, Component::NFComponent::COMPONENT), true)?), json)?;
            json = dumpJSONSCodeMod(var_field!((*elem).modifications, SCode::Element::COMPONENT), &scope, json)?;
            json = JSON::addPair(&(literal!("condition")), &(JSON::makeBoolean(false)), json)?;
            json = JSON::addPairNotNull(&(literal!("prefixes")), &(dumpJSONAttributes(var_field!((*elem).attributes, SCode::Element::COMPONENT), var_field!((*elem).prefixes, SCode::Element::COMPONENT), &scope)?), json)?;
            json = dumpJSONComment(var_field!((*elem).comment, SCode::Element::COMPONENT), &scope, json, true, true, false)?;
            ()
        },
        (Deref @ Component::INVALID_COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { .. }) => {
            json = JSON::addPair(&(literal!("type")), &(dumpJSONComponentType(cls, node, &(Component::getType(&comp)?), false)?), json)?;
            json = dumpJSONSCodeMod(var_field!((*elem).modifications, SCode::Element::COMPONENT), &scope, json)?;
            json = JSON::addPairNotNull(&(literal!("prefixes")), &(dumpJSONAttributes(var_field!((*elem).attributes, SCode::Element::COMPONENT), var_field!((*elem).prefixes, SCode::Element::COMPONENT), &scope)?), json)?;
            json = dumpJSONComment(var_field!((*elem).comment, SCode::Element::COMPONENT), &scope, json, true, true, false)?;
            json = JSON::addPair(&(literal!("$error")), &(JSON::makeString(var_field!((*comp).errors, Component::NFComponent::INVALID_COMPONENT).clone())), json)?;
            ()
        },
        (Deref @ Component::COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { .. }) => {
            json = JSON::addPair(&(literal!("type")), &(dumpJSONComponentType(cls, node.clone(), var_field!((*comp).ty, Component::NFComponent::COMPONENT), false)?), json)?;
            if Type::isArray(var_field!((*comp).ty, Component::NFComponent::COMPONENT)) {
                json = JSON::addPair(&(literal!("dims")), &(dumpJSONDims(&var_field!((*elem).attributes, SCode::Element::COMPONENT).arrayDims, &(Type::arrayDims(var_field!((*comp).ty, Component::NFComponent::COMPONENT).clone())))?), json)?;
            }
            json = dumpJSONSCodeMod(var_field!((*elem).modifications, SCode::Element::COMPONENT), &scope, json)?;
            is_constant = var_field!((*comp).attributes, Component::NFComponent::COMPONENT).variability.clone() <= Variability::PARAMETER.clone() && Binding::purity(var_field!((*comp).binding, Component::NFComponent::COMPONENT)) == Purity::PURE.clone();
            if Binding::isExplicitlyBound(var_field!((*comp).binding, Component::NFComponent::COMPONENT)) {
                json = JSON::addPair(&(literal!("value")), &(dumpJSONBinding(var_field!((*comp).binding, Component::NFComponent::COMPONENT).clone(), originalBinding, is_constant)?), json)?;
            }
            if Binding::isBound(var_field!((*comp).condition, Component::NFComponent::COMPONENT)) {
                json = JSON::addPair(&(literal!("condition")), &(dumpJSONBinding(var_field!((*comp).condition, Component::NFComponent::COMPONENT).clone(), None, true)?), json)?;
            }
            json = JSON::addPairNotNull(&(literal!("prefixes")), &(dumpJSONAttributes(var_field!((*elem).attributes, SCode::Element::COMPONENT), var_field!((*elem).prefixes, SCode::Element::COMPONENT), &scope)?), json)?;
            json = dumpJSONComment(var_field!((*comp).comment, Component::NFComponent::COMPONENT), &scope, json, true, true, false)?;
            if NFInstNode::InstNode::isGeneratedInner(&node) {
                json = JSON::addPair(&(literal!("generated")), &(JSON::makeBoolean(true)), json)?;
            }
            ()
        },
        (Deref @ Component::COMPONENT_DEF { .. }, Deref @ SCode::Element::COMPONENT { .. }) if (AbsynUtil::isOnlyOuter(var_field!((*elem).prefixes, SCode::Element::COMPONENT).innerOuter.clone())) => {
            path = AbsynUtil::typeSpecPath(var_field!((*elem).typeSpec, SCode::Element::COMPONENT));
            match '__try0: {
                (ty_node, _, _) = unwrap_break_err!(Lookup::lookupName(&path, scope.clone(), NFInstContext::set(NFInstContext::RELAXED.clone(), NFInstContext::FAST_LOOKUP.clone()), false), '__try0);
                json = unwrap_break_err!(JSON::addPair(&(literal!("type")), &(unwrap_break_err!(dumpJSONSCodeClass(&(unwrap_break_err!(NFInstNode::InstNode::definition(ty_node.clone()), '__try0)), ty_node.clone(), NFInstNode::InstNode::resolveInner(component.clone()), false, JSON::makeNull()), '__try0)), json.clone()), '__try0);
                Ok::<_, &'static str>((json.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    json = __try0_o0;
                }
                Err(_) => {
                    json = JSON::addPair(&(literal!("type")), &(dumpJSONPath(path.clone())?), json.clone())?;
                }
            }
            json = JSON::addPairNotNull(&(literal!("prefixes")), &(dumpJSONAttributes(var_field!((*elem).attributes, SCode::Element::COMPONENT), var_field!((*elem).prefixes, SCode::Element::COMPONENT), &scope)?), json)?;
            json = dumpJSONComment(var_field!((*elem).comment, SCode::Element::COMPONENT), &scope, json, true, true, false)?;
            ()
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstanceAPI.dumpJSONComponent")); __mm_s.push_str(&*literal!(" got unknown component ")); __mm_s.push_str(&*NFInstNode::InstNode::name(&node)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFInstanceAPI.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub(crate) fn dumpJSONComponentType(
    mut cls: &metamodelica::Ref<InstanceTree>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut isDeleted: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    json = (::match_deref::match_deref! { match &((cls.clone(), Type::arrayElementType(ty))) {
        (_, Deref @ Type::ENUMERATION { .. }) => dumpJSONEnumType(cls, node)?,
        (_, Deref @ Type::UNKNOWN) => dumpJSONSCodeElementType(&(NFInstNode::InstNode::definition(node)?))?,
        (Deref @ InstanceTree::CLASS { .. }, _) => dumpJSONInstanceTree(cls, &node, true, isDeleted, false)?,
        _ => dumpJSONTypeName(ty)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub(crate) fn dumpJSONSCodeElementType(
    mut elem: &metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    let () = (match &**elem {
        SCode::Element::COMPONENT {
            typeSpec: __elem_typeSpec,
            ..
        } => {
            json = JSON::addPair(
                &(literal!("name")),
                &(dumpJSONPath(AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&__elem_typeSpec)))?),
                json,
            )?;
            json = JSON::addPair(&(literal!("missing")), &(JSON::makeBoolean(true)), json)?;
            ()
        }
        _ => (),
    });
    Ok(json)
}

pub(crate) fn dumpJSONEnumType(
    mut tree: &metamodelica::Ref<InstanceTree>,
    mut enumNode: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut node: metamodelica::Ref<InstNode::InstNode> =
        NFInstNode::InstNode::resolveInner(NFInstNode::InstNode::classScope(enumNode.clone())?);
    let mut def: metamodelica::Ref<SCode::Element>;
    let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut json_elems: metamodelica::Ref<JSON::JSON>;
    let mut elems: metamodelica::List<metamodelica::Ref<InstanceTree>>;
    def = NFInstNode::InstNode::definition(node.clone())?;
    json = JSON::makeNull();
    json = JSON::addPair(&(literal!("name")), &(dumpJSONNodePath(node.clone(), false)?), json)?;
    json = JSON::addPairNotNull(&(literal!("dims")), &(dumpJSONClassDims(node.clone(), &def)?), json)?;
    json = JSON::addPair(
        &(literal!("restriction")),
        &(JSON::makeString(SCodeDump::restrictionStringPP(SCodeUtil::getClassRestriction(&def)?)?)),
        json,
    )?;
    json = dumpJSONCommentOpt(SCodeUtil::getElementComment(&def), &node, json, true, true, false)?;
    let __pa0 = ::match_deref::match_deref! { match &((*tree)) {
        Deref @ InstanceTree::CLASS { elements: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    elems = metamodelica::Own::own(__pa0);
    json_elems = dumpJSONElements(&elems, node.clone(), false)?;
    comps = ClassTree::getComponents(&(Class::classTree(NFInstNode::InstNode::getClass(node.clone())?)?))?;
    json_elems = dumpJSONEnumTypeLiterals(comps.clone(), &(NFInstNode::InstNode::parent(&node)?), json_elems)?;
    json = JSON::addPair(&(literal!("elements")), &json_elems, json)?;
    json = JSON::addPair(
        &(literal!("source")),
        &(JSON::dumpJSONSourceInfo(&(NFInstNode::InstNode::info(&node)), true)?),
        json,
    )?;
    Ok(json)
}

pub(crate) fn dumpJSONEnumTypeLiterals(
    mut literals: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut json: metamodelica::Ref<JSON::JSON>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    for mut i in 6..=metamodelica::arrayLength(literals.clone()) {
        json = JSON::addElement(
            &(dumpJSONEnumTypeLiteral(
                &({
                    let __elt = (*metamodelica::index_checked(&literals.borrow(), i)?).clone();
                    __elt
                }),
                scope,
            )?),
            json,
        )?;
    }
    Ok(json)
}

pub(crate) fn dumpJSONEnumTypeLiteral(
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    json = JSON::addPair(
        &(literal!("$kind")),
        &(metamodelica::Ref::new(JSON::JSON::STRING {
            r#str: literal!("component"),
        })),
        json,
    )?;
    json = JSON::addPair(
        &(literal!("name")),
        &(JSON::makeString(NFInstNode::InstNode::name(node)?)),
        json,
    )?;
    json = dumpJSONComment(
        &(Component::comment(&(NFInstNode::InstNode::component(node)?))?),
        scope,
        json,
        true,
        true,
        false,
    )?;
    Ok(json)
}

pub(crate) fn dumpJSONTypeName(mut ty: &metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    json = JSON::makeString(Type::toString(&(Type::arrayElementType(ty)))?);
    Ok(json)
}

pub(crate) fn dumpJSONBinding(
    mut binding: metamodelica::Ref<Binding::NFBinding>,
    mut originalBinding: Option<metamodelica::Ref<Binding::NFBinding>>,
    mut evaluate: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut bind: metamodelica::Ref<Binding::NFBinding> = binding.clone();
    let mut context: i32;
    if (originalBinding).is_some() && Binding::isEvaluated(&binding) {
        if '__try0: {
            context = NFInstContext::set(NFInstContext::RELAXED.clone(), NFInstContext::INSTANCE_API.clone());
            bind = unwrap_break_err!(NFInst::instBinding(unwrap_break_err!(originalBinding.clone().ok_or("pattern mismatch"), '__try0), context), '__try0);
            bind = unwrap_break_err!(Typing::typeBinding(bind.clone(), context), '__try0);
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    exp = Binding::getExp(&bind)?;
    exp = Expression::map(
        exp,
        (std::sync::Arc::new(Expression::expandSplitIndices)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    json = JSON::addPair(&(literal!("binding")), &(Expression::toJSON(exp.clone())?), json)?;
    if evaluate && !(Expression::isLiteral(&exp)?) {
        ErrorExt::setCheckpoint(literal!("NFInstanceAPI.dumpJSONBinding"));
        if '__try1: {
            exp = unwrap_break_err!(Ceval::evalExp(exp.clone(), &(Ceval::EvalTarget::new(Absyn::dummyInfo.clone(), NFInstContext::INSTANCE_API.clone(), None))), '__try1);
            exp = unwrap_break_err!(Expression::map(exp.clone(), (std::sync::Arc::new(Expression::expandSplitIndices) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>)), '__try1);
            json = unwrap_break_err!(JSON::addPair(&(literal!("value")), &(unwrap_break_err!(Expression::toJSON(exp.clone()), '__try1)), json.clone()), '__try1);
            Ok::<(), &'static str>(())
        }.is_err() {
        }
        ErrorExt::rollBack(literal!("NFInstanceAPI.dumpJSONBinding"));
    }
    Ok(json)
}

pub(crate) fn dumpJSONClassDims(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut element: &metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut absyn_dims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    ty = NFInstNode::InstNode::getType(node)?;
    if Type::isArray(&ty) {
        absyn_dims = (::match_deref::match_deref! { match element {
            Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: Some(__esc_absyn_dims), .. }, .. }, .. } => {
                absyn_dims = (*__esc_absyn_dims).clone();
                absyn_dims.clone()
            },
            _ => metamodelica::nil(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        json = dumpJSONDims(&absyn_dims, &(Type::arrayDims(ty)))?;
    } else {
        json = JSON::makeNull();
    }
    Ok(json)
}

pub(crate) fn dumpJSONDims(
    mut absynDims: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut typedDims: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    let mut ty_json: metamodelica::Ref<JSON::JSON>;
    json = JSON::addPairNotNull(&(literal!("absyn")), &(dumpJSONAbsynDims(absynDims)?), json)?;
    ty_json = JSON::makeNull();
    for mut d in &**typedDims {
        ty_json = JSON::addElement(
            &(JSON::makeString(Dimension::toString(metamodelica::AsArg::as_arg(&d))?)),
            ty_json,
        )?;
    }
    json = JSON::addPairNotNull(&(literal!("typed")), &ty_json, json)?;
    Ok(json)
}

pub(crate) fn dumpJSONAbsynDims(
    mut dims: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    for mut d in &**dims {
        json = JSON::addElement(
            &(JSON::makeString(Dump::printSubscriptStr(metamodelica::AsArg::as_arg(&d))?)),
            json,
        )?;
    }
    Ok(json)
}

pub(crate) fn dumpJSONAttributes(
    mut attrs: &SCode::Attributes,
    mut prefs: &metamodelica::Ref<SCode::Prefixes>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut s: ArcStr;
    json = dumpJSONSCodePrefixes(prefs, scope)?;
    s = SCodeDump::connectorTypeStr(attrs.connectorType.clone());
    if !(stringEmpty(&s)) {
        json = JSON::addPair(&(literal!("connector")), &(JSON::makeString(s)), json)?;
    }
    s = SCodeDump::unparseVariability(attrs.variability.clone());
    if !(stringEmpty(&s)) {
        json = JSON::addPair(&(literal!("variability")), &(JSON::makeString(s)), json)?;
    }
    if AbsynUtil::isInput(attrs.direction.clone()) {
        json = JSON::addPair(
            &(literal!("direction")),
            &(metamodelica::Ref::new(JSON::JSON::STRING {
                r#str: literal!("input"),
            })),
            json,
        )?;
    } else if AbsynUtil::isOutput(attrs.direction.clone()) {
        json = JSON::addPair(
            &(literal!("direction")),
            &(metamodelica::Ref::new(JSON::JSON::STRING {
                r#str: literal!("output"),
            })),
            json,
        )?;
    }
    Ok(json)
}

pub(crate) fn dumpJSONSCodePrefixes(
    mut prefixes: &metamodelica::Ref<SCode::Prefixes>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    if !(SCodeUtil::visibilityBool(prefixes.visibility.clone())) {
        json = JSON::addPair(&(literal!("public")), &(JSON::makeBoolean(false)), json)?;
    }
    if SCodeUtil::finalBool(prefixes.finalPrefix.clone()) {
        json = JSON::addPair(&(literal!("final")), &(JSON::makeBoolean(true)), json)?;
    }
    if AbsynUtil::isInner(prefixes.innerOuter.clone()) {
        json = JSON::addPair(&(literal!("inner")), &(JSON::makeBoolean(true)), json)?;
    }
    if AbsynUtil::isOuter(prefixes.innerOuter.clone()) {
        json = JSON::addPair(&(literal!("outer")), &(JSON::makeBoolean(true)), json)?;
    }
    json = JSON::addPairNotNull(
        &(literal!("replaceable")),
        &(dumpJSONReplaceable(&prefixes.replaceablePrefix, scope)?),
        json,
    )?;
    if SCodeUtil::redeclareBool(prefixes.redeclarePrefix.clone()) {
        json = JSON::addPair(&(literal!("redeclare")), &(JSON::makeBoolean(true)), json)?;
    }
    Ok(json)
}

pub(crate) fn dumpJSONClassPrefixes(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut cdef: metamodelica::Ref<SCode::ClassDef>;
    json = (match &**element {
        SCode::Element::CLASS {
            classDef: __esc_cdef,
            encapsulatedPrefix: __element_encapsulatedPrefix,
            partialPrefix: __element_partialPrefix,
            prefixes: __element_prefixes,
            ..
        } => {
            cdef = (*__esc_cdef).clone();
            json = (match &*cdef.clone() {
                SCode::ClassDef::DERIVED {
                    attributes: __cdef_attributes,
                    ..
                } => dumpJSONAttributes(
                    metamodelica::AsArg::as_arg(&__cdef_attributes),
                    metamodelica::AsArg::as_arg(&__element_prefixes),
                    scope,
                )?,
                _ => dumpJSONSCodePrefixes(metamodelica::AsArg::as_arg(&__element_prefixes), scope)?,
            });
            if SCodeUtil::partialBool(__element_partialPrefix.clone()) {
                json = JSON::addPair(&(literal!("partial")), &(JSON::makeBoolean(true)), json)?;
            }
            if SCodeUtil::encapsulatedBool(__element_encapsulatedPrefix.clone()) {
                json = JSON::addPair(&(literal!("encapsulated")), &(JSON::makeBoolean(true)), json)?;
            }
            json
        }
        _ => JSON::makeNull(),
    });
    Ok(json)
}

pub(crate) fn dumpJSONReplaceable(
    mut repl: &metamodelica::Ref<SCode::Replaceable>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut cc: metamodelica::Ref<SCode::ConstrainClass>;
    json = (::match_deref::match_deref! { match repl {
        Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(__esc_cc) } => {
            cc = (*__esc_cc).clone();
            json = JSON::makeNull();
            json = JSON::addPair(&(literal!("constrainedby")), &(dumpJSONPath(cc.constrainingClass.clone())?), json)?;
            json = dumpJSONSCodeMod(&cc.modifier, scope, json)?;
            json = dumpJSONCommentOpt(Some(cc.comment.clone()), scope, json, true, true, false)?;
            json
        },
        Deref @ SCode::Replaceable::REPLACEABLE { .. } => JSON::makeBoolean(true),
        _ => JSON::makeNull(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub(crate) fn dumpJSONCommentOpt(
    mut cmtOpt: Option<metamodelica::Ref<SCode::Comment>>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut json: metamodelica::Ref<JSON::JSON>,
    mut dumpComment: bool,
    mut dumpAnnotation: bool,
    mut failOnError: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    if (cmtOpt).is_some() {
        json = dumpJSONComment(
            &(cmtOpt.ok_or("pattern mismatch")?),
            scope,
            json,
            dumpComment,
            dumpAnnotation,
            failOnError,
        )?;
    }
    Ok(json)
}

pub(crate) fn dumpJSONComment(
    mut cmt: &metamodelica::Ref<SCode::Comment>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut json: metamodelica::Ref<JSON::JSON>,
    mut dumpComment: bool,
    mut dumpAnnotation: bool,
    mut failOnError: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    if (cmt.comment).is_some() && dumpComment {
        json = JSON::addPair(
            &(literal!("comment")),
            &(JSON::makeString(cmt.comment.clone().ok_or("pattern mismatch")?)),
            json,
        )?;
    }
    if dumpAnnotation {
        json = dumpJSONAnnotationOpt(
            cmt.annotation_.clone(),
            scope,
            &(metamodelica::nil()),
            failOnError,
            json,
        )?;
    }
    Ok(json)
}

pub(crate) fn dumpJSONCommentAnnotation(
    mut cmtOpt: Option<metamodelica::Ref<SCode::Comment>>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut json: metamodelica::Ref<JSON::JSON>,
    mut filter: &metamodelica::List<ArcStr>,
    mut failOnError: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    let mut cmt: metamodelica::Ref<SCode::Comment>;
    if (cmtOpt).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(cmtOpt) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        cmt = metamodelica::Own::own(__pa0);
        json = dumpJSONAnnotationOpt(cmt.annotation_.clone(), scope, filter, failOnError, json)?;
    }
    Ok(json)
}

pub(crate) fn dumpJSONAnnotationOpt(
    mut annOpt: Option<metamodelica::Ref<SCode::Annotation>>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut filter: &metamodelica::List<ArcStr>,
    mut failOnError: bool,
    mut json: metamodelica::Ref<JSON::JSON>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    let mut ann: metamodelica::Ref<SCode::Annotation>;
    if (annOpt).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(annOpt) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        ann = metamodelica::Own::own(__pa0);
        json = JSON::addPair(
            &(literal!("annotation")),
            &(dumpJSONAnnotationMod(&ann.modification, scope, filter, failOnError)?),
            json,
        )?;
    }
    Ok(json)
}

pub(crate) fn dumpJSONAnnotationMod(
    mut r#mod: &metamodelica::Ref<SCode::Mod>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut filter: &metamodelica::List<ArcStr>,
    mut failOnError: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    json = (match &**r#mod {
        SCode::Mod::MOD {
            subModLst: __mod_subModLst,
            ..
        } => dumpJSONAnnotationSubMods(
            metamodelica::AsArg::as_arg(&__mod_subModLst),
            scope,
            filter,
            failOnError,
        )?,
        _ => JSON::makeNull(),
    });
    Ok(json)
}

pub(crate) fn dumpJSONAnnotationSubMods(
    mut subMods: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut filter: &metamodelica::List<ArcStr>,
    mut failOnError: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    for mut m in &**subMods {
        if (filter).is_empty() || List::contains(filter, m.ident.clone(), &fnptr!(stringEq, ArcStr, ArcStr))? {
            json = dumpJSONAnnotationSubMod(metamodelica::AsArg::as_arg(&m), scope, failOnError, json)?;
        }
    }
    Ok(json)
}

pub(crate) fn dumpJSONAnnotationSubMod(
    mut subMod: &metamodelica::Ref<SCode::SubMod>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut failOnError: bool,
    mut json: metamodelica::Ref<JSON::JSON>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    let mut name: ArcStr;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let mut absyn_binding: metamodelica::Ref<Absyn::Exp>;
    let mut j: metamodelica::Ref<JSON::JSON>;
    let __arc2 = &(*subMod);
    let SCode::SubMod {
        ident: __pa0,
        r#mod: __pa1,
    } = &**__arc2;
    name = metamodelica::Own::own(__pa0);
    r#mod = metamodelica::Own::own(__pa1);
    let () = (::match_deref::match_deref! { match &((name.clone(), r#mod.clone())) {
        (Deref @ "choices", Deref @ SCode::Mod::MOD { .. }) => {
            j = dumpJSONChoicesAnnotation(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone(), scope, &(var_field!((*r#mod).info, SCode::Mod::MOD).clone()), failOnError)?;
            json = JSON::addPairNotNull(&name, &j, json)?;
            ()
        },
        (_, Deref @ SCode::Mod::MOD { binding: Some(__esc_absyn_binding), .. }) => {
            absyn_binding = (*__esc_absyn_binding).clone();
            j = dumpJSONAnnotationExp(metamodelica::AsArg::as_arg(&absyn_binding), scope, var_field!((*r#mod).info, SCode::Mod::MOD), failOnError)?;
            json = JSON::addPair(&name, &j, json)?;
            ()
        },
        (_, Deref @ SCode::Mod::MOD { .. }) => {
            json = JSON::addPair(&name, &(dumpJSONAnnotationSubMods(var_field!((*r#mod).subModLst, SCode::Mod::MOD), scope, &(metamodelica::nil()), failOnError)?), json)?;
            ()
        },
        (_, Deref @ SCode::Mod::NOMOD { .. }) => {
            json = JSON::addPair(&name, &(JSON::emptyListObject()), json)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub(crate) fn dumpJSONAnnotationExp(
    mut absynExp: &metamodelica::Ref<Absyn::Exp>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut info: &SourceInfo,
    mut failOnError: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut j: metamodelica::Ref<JSON::JSON>;
    json = (match &**absynExp {
        Absyn::Exp::INTEGER {
            value: __absynExp_value,
        } => JSON::makeInteger(__absynExp_value.clone()),
        Absyn::Exp::REAL {
            value: __absynExp_value,
        } => JSON::makeNumber(stringReal(__absynExp_value.clone())?),
        Absyn::Exp::STRING {
            value: __absynExp_value,
        } => JSON::makeString(__absynExp_value.clone()),
        Absyn::Exp::BOOL {
            value: __absynExp_value,
        } => JSON::makeBoolean(__absynExp_value.clone()),
        Absyn::Exp::ARRAY {
            arrayExp: __absynExp_arrayExp,
        } if (!(AbsynUtil::isLiteralExp(absynExp)?)) => {
            json = JSON::emptyArray(((__absynExp_arrayExp).len() as i32));
            for mut e in &*__absynExp_arrayExp.clone() {
                j = dumpJSONAnnotationExp(metamodelica::AsArg::as_arg(&e), scope, info, failOnError)?;
                json = JSON::addElement(&j, json)?;
            }
            json
        }
        _ => dumpJSONAnnotationExp2(absynExp.clone(), scope, info, failOnError)?,
    });
    Ok(json)
}

pub(crate) fn dumpJSONAnnotationExp2(
    mut absynExp: metamodelica::Ref<Absyn::Exp>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut info: &SourceInfo,
    mut failOnError: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    ErrorExt::setCheckpoint(literal!("NFInstanceAPI.dumpJSONAnnotationExp2"));
    match '__try0: {
        exp = unwrap_break_err!(NFInst::instExp(absynExp.clone(), scope, INST_API_ANNOTATION_CONTEXT.clone(), info), '__try0);
        (exp, _, _, _) =
            unwrap_break_err!(Typing::typeExp(exp.clone(), INST_API_ANNOTATION_CONTEXT.clone(), info, false), '__try0);
        exp = unwrap_break_err!(SimplifyExp::simplify(exp.clone(), false), '__try0);
        json = unwrap_break_err!(Expression::toJSON(exp.clone()), '__try0);
        Ok::<_, &'static str>((json.clone(),))
    } {
        Ok((__try0_o0,)) => {
            json = __try0_o0;
        }
        Err(_) => {
            if failOnError {
                return Err("fail");
            }
            json = JSON::makeNull();
            json = JSON::addPair(
                &(literal!("$error")),
                &(JSON::makeString(ErrorExt::printCheckpointMessagesStr(false))),
                json.clone(),
            )?;
            json = JSON::addPair(
                &(literal!("value")),
                &(dumpJSONAbsynExpression(&absynExp)?),
                json.clone(),
            )?;
        }
    }
    ErrorExt::delCheckpoint(literal!("NFInstanceAPI.dumpJSONAnnotationExp2"));
    Ok(json)
}

pub(crate) fn dumpJSONAbsynExpression(
    mut exp: &metamodelica::Ref<Absyn::Exp>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut i: i32;
    let mut r: ArcStr;
    json = (::match_deref::match_deref! { match exp {
        Deref @ Absyn::Exp::INTEGER { value: __exp_value } => JSON::makeInteger(__exp_value.clone()),
        Deref @ Absyn::Exp::REAL { value: __exp_value } => JSON::makeNumber(stringReal(__exp_value.clone())?),
        Deref @ Absyn::Exp::CREF { componentRef: __exp_componentRef } => dumpJSONAbsynCref(metamodelica::AsArg::as_arg(&__exp_componentRef))?,
        Deref @ Absyn::Exp::STRING { value: __exp_value } => JSON::makeString(__exp_value.clone()),
        Deref @ Absyn::Exp::BOOL { value: __exp_value } => JSON::makeBoolean(__exp_value.clone()),
        Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UMINUS { .. }, exp: Deref @ Absyn::Exp::INTEGER { value: __esc_i } } => {
            i = (*__esc_i).clone();
            JSON::makeInteger(-(i.clone()))
        },
        Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UMINUS { .. }, exp: Deref @ Absyn::Exp::REAL { value: __esc_r } } => {
            r = (*__esc_r).clone();
            JSON::makeNumber(-(stringReal(r.clone())?))
        },
        Deref @ Absyn::Exp::CALL { functionArgs: __exp_functionArgs, function_: __exp_function_, .. } => {
            json = JSON::makeNull();
            json = JSON::addPair(&(literal!("$kind")), &(metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("call") })), json)?;
            json = JSON::addPair(&(literal!("name")), &(dumpJSONAbsynCref(metamodelica::AsArg::as_arg(&__exp_function_))?), json)?;
            json = dumpJSONAbsynFunctionArgs(metamodelica::AsArg::as_arg(&__exp_functionArgs), json)?;
            json
        },
        Deref @ Absyn::Exp::ARRAY { arrayExp: __exp_arrayExp } => {
            json = JSON::emptyArray(((__exp_arrayExp).len() as i32));
            for mut e in &*__exp_arrayExp.clone() {
                json = JSON::addElement(&(dumpJSONAbsynExpression(metamodelica::AsArg::as_arg(&e))?), json)?;
            }
            json
        },
        _ => JSON::makeString(Dump::printExpStr(AbsynUtil::stripCommentExpressions(exp.clone(), true)?)?),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub(crate) fn dumpJSONAbsynCref(
    mut cref: &metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    json = JSON::makeString(Dump::printComponentRefStr(cref)?);
    Ok(json)
}

pub(crate) fn dumpJSONAbsynFunctionArgs(
    mut args: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut json: metamodelica::Ref<JSON::JSON>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    let mut json_args: metamodelica::Ref<JSON::JSON>;
    let () = (match &**args {
        Absyn::FunctionArgs::FUNCTIONARGS {
            argNames: __args_argNames,
            args: __args_args,
        } => {
            if !((__args_args).is_empty()) {
                json_args = JSON::makeNull();
                for mut arg in &*__args_args.clone() {
                    json_args = JSON::addElement(
                        &(dumpJSONAbsynExpression(metamodelica::AsArg::as_arg(&arg))?),
                        json_args,
                    )?;
                }
                json = JSON::addPair(&(literal!("args")), &json_args, json)?;
            }
            if !((__args_argNames).is_empty()) {
                json_args = JSON::makeNull();
                for mut arg in &*__args_argNames.clone() {
                    json_args = JSON::addPair(&arg.argName, &(dumpJSONAbsynExpression(&arg.argValue)?), json_args)?;
                }
                json = JSON::addPair(&(literal!("namedArgs")), &json_args, json)?;
            }
            ()
        }
        _ => (),
    });
    Ok(json)
}

pub(crate) fn dumpJSONImports(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut json: metamodelica::Ref<JSON::JSON>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    let mut n: metamodelica::Ref<InstNode::InstNode> = node;
    let mut imps: metamodelica::Array<metamodelica::Ref<Import::NFImport>>;
    let mut resolved_imps: metamodelica::List<metamodelica::Ref<Import::NFImport>>;
    let mut json_imp: metamodelica::Ref<JSON::JSON>;
    let mut json_imp_array: metamodelica::Ref<JSON::JSON>;
    json_imp_array = JSON::makeNull();
    while !(NFInstNode::InstNode::isEmpty(&n)) {
        imps = ClassTree::getImports(&(Class::classTree(NFInstNode::InstNode::getClass(n.clone())?)?))?;
        if !(imps.clone().borrow().is_empty()) {
            resolved_imps = Import::resolveList(imps.clone());
            resolved_imps = metamodelica::Dangerous::listReverseInPlace(resolved_imps);
            for mut imp in &*resolved_imps {
                let () = (match &*imp.clone() {
                    Import::RESOLVED_IMPORT {
                        node: __imp_node,
                        shortName: __imp_shortName,
                        ..
                    } => {
                        json_imp = JSON::makeNull();
                        json_imp = JSON::addPair(
                            &(literal!("path")),
                            &(dumpJSONPath(NFInstNode::InstNode::fullPath(
                                NFInstNode::InstNode::borrow(__imp_node.clone())?,
                                false,
                            )?)?),
                            json_imp,
                        )?;
                        if !(stringEmpty(&__imp_shortName)) {
                            json_imp = JSON::addPair(
                                &(literal!("shortName")),
                                &(JSON::makeString(__imp_shortName.clone())),
                                json_imp,
                            )?;
                        }
                        json_imp_array = JSON::addElement(&json_imp, json_imp_array)?;
                        ()
                    }
                    _ => (),
                });
            }
        }
        n = NFInstNode::InstNode::parent(&n)?;
    }
    json = JSON::addPairNotNull(&(literal!("imports")), &json_imp_array, json)?;
    Ok(json)
}

pub(crate) fn dumpJSONEquations(
    mut sections: &metamodelica::Ref<Sections::NFSections>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut json: metamodelica::Ref<JSON::JSON>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    let mut connections: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut transitions: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut initial_states: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut j: metamodelica::Ref<JSON::JSON>;
    let mut context: i32;
    (connections, transitions, initial_states) = sortEquations(
        Sections::equations(sections),
        metamodelica::nil(),
        metamodelica::nil(),
        metamodelica::nil(),
    )?;
    context = NFInstContext::set(NFInstContext::CLASS.clone(), NFInstContext::RELAXED.clone());
    transitions = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
        for mut e in (transitions).into_iter().cloned() {
            let __x = Typing::typeEquation(e.clone(), context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    initial_states = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
        for mut e in (initial_states).into_iter().cloned() {
            let __x = Typing::typeEquation(e.clone(), context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    j = dumpJSONConnections(&connections, scope)?;
    json = JSON::addPairNotNull(&(literal!("connections")), &j, json)?;
    j = dumpJSONStateCalls(&initial_states, scope)?;
    json = JSON::addPairNotNull(&(literal!("initialStates")), &j, json)?;
    j = dumpJSONStateCalls(&transitions, scope)?;
    json = JSON::addPairNotNull(&(literal!("transitions")), &j, json)?;
    Ok(json)
}

pub(crate) fn sortEquations(
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut connections: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut transitions: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut initialStates: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
)> {
    let mut connections: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = connections;
    let mut transitions: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = transitions;
    let mut initialStates: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = initialStates;
    for mut eq in &*equations.reverse() {
        let () = (match &*eq.clone() {
            Equation::CONNECT { .. } => {
                connections = metamodelica::cons(eq.clone(), connections);
                ()
            }
            Equation::FOR { body: __eq_body, .. } => {
                (connections, transitions, initialStates) =
                    sortEquations(__eq_body.clone(), connections, transitions, initialStates)?;
                ()
            }
            Equation::IF {
                branches: __eq_branches,
                ..
            } => {
                for mut b in &*__eq_branches.clone() {
                    let () = (match &*b.clone() {
                        Equation::Branch::BRANCH { body: __b_body, .. } => {
                            (connections, transitions, initialStates) =
                                sortEquations(__b_body.clone(), connections, transitions, initialStates)?;
                            ()
                        }
                        _ => (),
                    });
                }
                ()
            }
            Equation::NORETCALL { exp: __eq_exp, .. } => {
                if Expression::isCallNamed(metamodelica::AsArg::as_arg(&__eq_exp), &(literal!("transition")))? {
                    transitions = metamodelica::cons(eq.clone(), transitions);
                } else if Expression::isCallNamed(metamodelica::AsArg::as_arg(&__eq_exp), &(literal!("initialState")))?
                {
                    initialStates = metamodelica::cons(eq.clone(), initialStates);
                }
                ()
            }
            _ => (),
        });
    }
    Ok((connections, transitions, initialStates))
}

pub(crate) fn dumpJSONConnections(
    mut connections: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    for mut conn in &**connections {
        json = JSON::addElement(&(dumpJSONConnection(metamodelica::AsArg::as_arg(&conn), scope)?), json)?;
    }
    Ok(json)
}

pub(crate) fn dumpJSONConnection(
    mut connEq: &metamodelica::Ref<Equation::NFEquation>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*connEq)) {
        Deref @ Equation::CONNECT { lhs: __pa0, rhs: __pa1, source: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    lhs = metamodelica::Own::own(__pa0);
    rhs = metamodelica::Own::own(__pa1);
    src = metamodelica::Own::own(__pa2);
    json = JSON::addPair(&(literal!("lhs")), &(Expression::toJSON(lhs)?), json)?;
    json = JSON::addPair(&(literal!("rhs")), &(Expression::toJSON(rhs)?), json)?;
    json = dumpJSONCommentAnnotation(
        ElementSource::getOptComment(&src)?,
        scope,
        json,
        &(metamodelica::nil()),
        false,
    )?;
    Ok(json)
}

pub(crate) fn dumpJSONStateCalls(
    mut callEqs: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    for mut eq in &**callEqs {
        json = JSON::addElement(&(dumpJSONStateCall(metamodelica::AsArg::as_arg(&eq), scope)?), json)?;
    }
    Ok(json)
}

pub(crate) fn dumpJSONStateCall(
    mut callEq: &metamodelica::Ref<Equation::NFEquation>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let mut j: metamodelica::Ref<JSON::JSON>;
    let () = (::match_deref::match_deref! { match callEq {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_CALL { arguments: __esc_args, .. } }, source: __esc_src, .. } => {
            call = (*__esc_call).clone();
            args = (*__esc_args).clone();
            src = (*__esc_src).clone();
            j = JSON::emptyArray(((args).len() as i32));
            for mut arg in &*args.clone() {
                j = JSON::addElement(&(Expression::toJSON(arg.clone())?), j)?;
            }
            json = JSON::addPair(&(literal!("arguments")), &j, json)?;
            json = dumpJSONCommentAnnotation(ElementSource::getOptComment(metamodelica::AsArg::as_arg(&src))?, scope, json, &(metamodelica::nil()), false)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub(crate) fn dumpJSONReplaceableElements(
    mut clsNode: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    let mut j: metamodelica::Ref<JSON::JSON>;
    cls_tree = Class::classTree(NFInstNode::InstNode::getClass(clsNode)?)?;
    let __range0 = ClassTree::getComponents(&cls_tree)?
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut c in __range0 {
        if NFInstNode::InstNode::isReplaceable(&c)? {
            j = JSON::makeNull();
            j = JSON::addPair(
                &(literal!("name")),
                &(JSON::makeString(NFInstNode::InstNode::name(&c)?)),
                j,
            )?;
            j = JSON::addPair(
                &(literal!("type")),
                &(dumpJSONTypeName(&(NFInstNode::InstNode::getType(c)?))?),
                j,
            )?;
            json = JSON::addElement(&j, json)?;
        }
    }
    let __range1 = ClassTree::getClasses(&cls_tree)?
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut c in __range1 {
        if NFInstNode::InstNode::isReplaceable(&c)? {
            json = JSON::addElement(&(JSON::makeString(NFInstNode::InstNode::name(&c)?)), json)?;
        }
    }
    Ok(json)
}

pub(crate) fn dumpJSONSCodeMod(
    mut r#mod: &metamodelica::Ref<SCode::Mod>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut json: metamodelica::Ref<JSON::JSON>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    let mut j: metamodelica::Ref<JSON::JSON>;
    j = dumpJSONSCodeMod_impl(r#mod, scope, false)?;
    json = JSON::addPairNotNull(&(literal!("modifiers")), &j, json)?;
    Ok(json)
}

pub(crate) fn dumpJSONSCodeMod_impl(
    mut r#mod: &metamodelica::Ref<SCode::Mod>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut isChoices: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    let mut binding_json: metamodelica::Ref<JSON::JSON>;
    let () = (match &**r#mod {
        SCode::Mod::MOD {
            binding: __mod_binding,
            comment: __mod_comment,
            eachPrefix: __mod_eachPrefix,
            finalPrefix: __mod_finalPrefix,
            subModLst: __mod_subModLst,
            ..
        } => {
            for mut m in &*__mod_subModLst.clone() {
                json = JSON::addPair(&m.ident, &(dumpJSONSCodeMod_impl(&m.r#mod, scope, false)?), json)?;
            }
            if SCodeUtil::finalBool(__mod_finalPrefix.clone()) {
                json = JSON::addPair(&(literal!("final")), &(JSON::makeBoolean(true)), json)?;
            }
            if SCodeUtil::eachBool(__mod_eachPrefix.clone()) {
                json = JSON::addPair(&(literal!("each")), &(JSON::makeBoolean(true)), json)?;
            }
            if isChoices && (__mod_comment).is_some() {
                json = JSON::addPair(
                    &(literal!("comment")),
                    &(JSON::makeString(__mod_comment.clone().ok_or("pattern mismatch")?)),
                    json,
                )?;
            }
            if (__mod_binding).is_some() {
                binding_json = JSON::makeString(Dump::printExpStr(AbsynUtil::stripCommentExpressions(
                    __mod_binding.clone().ok_or("pattern mismatch")?,
                    true,
                )?)?);
                if JSON::isNull(&json) {
                    json = binding_json;
                } else {
                    json = JSON::addPair(&(literal!("$value")), &binding_json, json)?;
                }
            }
            ()
        }
        SCode::Mod::REDECL {
            eachPrefix: __mod_eachPrefix,
            element: __mod_element,
            finalPrefix: __mod_finalPrefix,
        } => {
            if SCodeUtil::finalBool(__mod_finalPrefix.clone()) {
                json = JSON::addPair(&(literal!("final")), &(JSON::makeBoolean(true)), json)?;
            }
            if SCodeUtil::eachBool(__mod_eachPrefix.clone()) {
                json = JSON::addPair(&(literal!("each")), &(JSON::makeBoolean(true)), json)?;
            }
            json = JSON::addPair(
                &(literal!("$value")),
                &(dumpJSONSCodeElement(
                    metamodelica::AsArg::as_arg(&__mod_element),
                    scope.clone(),
                    JSON::makeNull(),
                )?),
                json,
            )?;
            ()
        }
        _ => (),
    });
    Ok(json)
}

pub(crate) fn dumpJSONRedeclareType(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut json: metamodelica::Ref<JSON::JSON>,
) -> metamodelica::Ref<JSON::JSON> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    let mut path: metamodelica::Ref<Path> = <metamodelica::Ref<Path> as ::std::default::Default>::default();
    let mut context: i32 = 0;
    let mut cls: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::EMPTY_NODE);
    let () = 'mc: {
        let __mc_input = &**element;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { .. } => {
                    let mut cls: metamodelica::Ref<InstNode::InstNode> = cls.clone();
                    let mut context: i32 = context.clone();
                    let mut json: metamodelica::Ref<JSON::JSON> = json.clone();
                    let mut path: metamodelica::Ref<Path> = path.clone();
                    path = AbsynUtil::typeSpecPath(var_field!((**element).typeSpec, SCode::Element::COMPONENT));
                    context = NFInstContext::set(NFInstContext::RELAXED.clone(), NFInstContext::FAST_LOOKUP.clone());
                    (cls, _, _) = Lookup::lookupName(&path, scope.clone(), context, false)?;
                    json = JSON::addPair(&(literal!("$type")), &(dumpJSONNodePath(cls.clone(), false)?), json.clone())?;
                    Ok(((), cls.clone(), context.clone(), json.clone(), path.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cls = __wb0;
            context = __wb1;
            json = __wb2;
            path = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    json
}

pub(crate) fn dumpJSONSCodeElement(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut json: metamodelica::Ref<JSON::JSON>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    json = (match &**element {
        SCode::Element::COMPONENT {
            attributes: __element_attributes,
            comment: __element_comment,
            condition: __element_condition,
            modifications: __element_modifications,
            name: __element_name,
            prefixes: __element_prefixes,
            typeSpec: __element_typeSpec,
            ..
        } => {
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("component"),
                })),
                json,
            )?;
            json = JSON::addPair(&(literal!("name")), &(JSON::makeString(__element_name.clone())), json)?;
            json = JSON::addPair(
                &(literal!("type")),
                &(dumpJSONPath(AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(
                    &__element_typeSpec,
                )))?),
                json,
            )?;
            json = JSON::addPairNotNull(
                &(literal!("dims")),
                &(dumpJSONDims(&__element_attributes.arrayDims, &(metamodelica::nil()))?),
                json,
            )?;
            json = dumpJSONSCodeMod(metamodelica::AsArg::as_arg(&__element_modifications), &scope, json)?;
            json = JSON::addPairNotNull(
                &(literal!("prefixes")),
                &(dumpJSONAttributes(
                    metamodelica::AsArg::as_arg(&__element_attributes),
                    metamodelica::AsArg::as_arg(&__element_prefixes),
                    &scope,
                )?),
                json,
            )?;
            if (__element_condition).is_some() {
                json = JSON::addPair(
                    &(literal!("condition")),
                    &(dumpJSONAbsynExpression(&(__element_condition.clone().ok_or("pattern mismatch")?))?),
                    json,
                )?;
            }
            json = dumpJSONComment(
                metamodelica::AsArg::as_arg(&__element_comment),
                &scope,
                json,
                true,
                true,
                false,
            )?;
            json
        }
        SCode::Element::CLASS { .. } => dumpJSONSCodeClass(
            element,
            openmodelica_nf_frontend::NFInstNode::InstNode::interned_EMPTY_NODE(),
            scope,
            false,
            json,
        )?,
        _ => json,
    });
    Ok(json)
}

pub(crate) fn dumpJSONSCodeType(
    mut path: metamodelica::Ref<Path>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut json: metamodelica::Ref<JSON::JSON>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    let mut ty_node: metamodelica::Ref<InstNode::InstNode>;
    match '__try0: {
        (ty_node, _, _) = unwrap_break_err!(Lookup::lookupName(&path, scope.clone(), NFInstContext::set(NFInstContext::RELAXED.clone(), NFInstContext::FAST_LOOKUP.clone()), false), '__try0);
        json = unwrap_break_err!(JSON::addPair(&(literal!("type")), &(unwrap_break_err!(dumpJSONSCodeClass(&(unwrap_break_err!(NFInstNode::InstNode::definition(ty_node.clone()), '__try0)), ty_node.clone(), scope.clone(), false, JSON::makeNull()), '__try0)), json.clone()), '__try0);
        Ok::<_, &'static str>((json.clone(),))
    } {
        Ok((__try0_o0,)) => {
            json = __try0_o0;
        }
        Err(_) => {
            json = JSON::addPair(&(literal!("type")), &(dumpJSONPath(path.clone())?), json.clone())?;
        }
    }
    Ok(json)
}

pub(crate) fn dumpJSONSCodeClass(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut isRedeclare: bool,
    mut json: metamodelica::Ref<JSON::JSON>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    let () = (match &**element {
        SCode::Element::CLASS {
            classDef: __element_classDef,
            cmt: __element_cmt,
            name: __element_name,
            restriction: __element_restriction,
            ..
        } => {
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("class"),
                })),
                json,
            )?;
            if NFInstNode::InstNode::isEmpty(&node) || isRedeclare {
                json = JSON::addPair(&(literal!("name")), &(JSON::makeString(__element_name.clone())), json)?;
            } else {
                json = JSON::addPair(&(literal!("name")), &(dumpJSONNodeEnclosingPath(node.clone())?), json)?;
            }
            json = JSON::addPair(
                &(literal!("restriction")),
                &(JSON::makeString(SCodeDump::restrictionStringPP(__element_restriction.clone())?)),
                json,
            )?;
            json = JSON::addPairNotNull(
                &(literal!("prefixes")),
                &(dumpJSONClassPrefixes(element, &scope)?),
                json,
            )?;
            json = dumpJSONSCodeClassDef(
                metamodelica::AsArg::as_arg(&__element_classDef),
                scope.clone(),
                isRedeclare,
                json,
            )?;
            json = dumpJSONComment(
                metamodelica::AsArg::as_arg(&__element_cmt),
                &scope,
                json,
                true,
                !(isRedeclare),
                false,
            )?;
            if isRedeclare {
                json = dumpJSONCommentAnnotation(
                    Some(__element_cmt.clone()),
                    &scope,
                    json,
                    &(list![literal!("Dialog"), literal!("choices"), literal!("choicesAllMatching")]),
                    false,
                )?;
            }
            if !(isRedeclare) {
                json = dumpJSONSCodeTypeExtends(node, scope, json);
            }
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(json)
}

pub(crate) fn dumpJSONSCodeTypeExtends(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut json: metamodelica::Ref<JSON::JSON>,
) -> metamodelica::Ref<JSON::JSON> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    let mut expanded_node: metamodelica::Ref<InstNode::InstNode>;
    let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut json_elements: metamodelica::Ref<JSON::JSON>;
    let mut json_ext: metamodelica::Ref<JSON::JSON>;
    if NFInstNode::InstNode::isEmpty(&node) {
        return json;
    }
    if '__try0: {
        expanded_node = unwrap_break_err!(NFInst::expand(node.clone(), NFInstContext::RELAXED.clone()), '__try0);
        exts = ClassTree::getExtends(&(unwrap_break_err!(Class::classTree(unwrap_break_err!(NFInstNode::InstNode::getClass(expanded_node.clone()), '__try0)), '__try0)));
        if !(exts.clone().borrow().is_empty()) {
            json_elements = JSON::makeNull();
            let __range1 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut ext in __range1 {
                json_ext = JSON::makeNull();
                json_ext = unwrap_break_err!(JSON::addPair(&(literal!("$kind")), &(metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("extends") })), json_ext.clone()), '__try0);
                json_ext = unwrap_break_err!(JSON::addPair(&(literal!("baseClass")), &(unwrap_break_err!(dumpJSONSCodeClass(&(unwrap_break_err!(NFInstNode::InstNode::definition(ext.clone()), '__try0)), ext.clone(), scope.clone(), false, JSON::makeNull()), '__try0)), json_ext.clone()), '__try0);
                json_elements = unwrap_break_err!(JSON::addElement(&json_ext, json_elements.clone()), '__try0);
            }
            json = unwrap_break_err!(JSON::addPair(&(literal!("elements")), &json_elements, json.clone()), '__try0);
        }
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    json
}

pub(crate) fn dumpJSONSCodeClassDef(
    mut classDef: &metamodelica::Ref<SCode::ClassDef>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut qualifyPath: bool,
    mut json: metamodelica::Ref<JSON::JSON>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = json;
    let mut path: metamodelica::Ref<Path>;
    let mut odims: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>;
    let mut derivedNode: metamodelica::Ref<InstNode::InstNode>;
    let () = (::match_deref::match_deref! { match classDef {
        Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __esc_path, arrayDim: __esc_odims }, modifications: __classDef_modifications, .. } => {
            path = (*__esc_path).clone();
            odims = (*__esc_odims).clone();
            if qualifyPath {
                if '__try0: {
                    (derivedNode, _, _) = unwrap_break_err!(Lookup::lookupName(metamodelica::AsArg::as_arg(&path), scope.clone(), NFInstContext::RELAXED.clone(), false), '__try0);
                    json = unwrap_break_err!(JSON::addPair(&(literal!("baseClass")), &(unwrap_break_err!(dumpJSONNodeEnclosingPath(derivedNode.clone()), '__try0)), json.clone()), '__try0);
                    Ok::<(), &'static str>(())
                }.is_err() {
                }
            } else {
                json = JSON::addPair(&(literal!("baseClass")), &(dumpJSONPath(path.clone())?), json)?;
            }
            if (odims).is_some() {
                json = JSON::addPairNotNull(&(literal!("dims")), &(dumpJSONDims(&(odims.clone().ok_or("pattern mismatch")?), &(metamodelica::nil()))?), json)?;
            }
            json = dumpJSONSCodeMod(metamodelica::AsArg::as_arg(&__classDef_modifications), &scope, json)?;
            ()
        },
        Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: __classDef_modifications, .. } => {
            json = dumpJSONSCodeMod(metamodelica::AsArg::as_arg(&__classDef_modifications), &scope, json)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub(crate) fn dumpJSONChoicesAnnotation(
    mut mods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut info: &SourceInfo,
    mut failOnError: bool,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    let mut smod: metamodelica::Ref<SCode::SubMod>;
    let mut choices: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut others: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut j: metamodelica::Ref<JSON::JSON>;
    choices = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::SubMod>> = metamodelica::nil();
        for mut m in (mods.clone()).into_iter().cloned() {
            if !(metamodelica::stringEq(&m.ident, &(literal!("choice")))) {
                continue;
            }
            let __x = m.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    others = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::SubMod>> = metamodelica::nil();
        for mut m in (mods).into_iter().cloned() {
            if !(!metamodelica::stringEq(&m.ident, &(literal!("choice")))) {
                continue;
            }
            let __x = m.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if !((choices).is_empty()) {
        j = JSON::emptyArray(((choices).len() as i32));
        for mut m in &*choices {
            let mut m = m.clone();
            m = (::match_deref::match_deref! { match &(m.r#mod.clone()) {
                Deref @ SCode::Mod::MOD { binding: None, subModLst: Deref @ metamodelica::ListNode::Cons { head: __esc_smod, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    smod = (*__esc_smod).clone();
                    smod.clone()
                },
                _ => m,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            j = JSON::addElement(&(dumpJSONSCodeMod_impl(&m.r#mod, scope, true)?), j)?;
        }
        json = JSON::addPair(&(literal!("choice")), &j, json)?;
    }
    for mut m in &*others {
        json = dumpJSONAnnotationSubMod(metamodelica::AsArg::as_arg(&m), scope, failOnError, json)?;
    }
    Ok(json)
}

pub fn modifierJSON(mut modifier: &ArcStr) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let mut amod: metamodelica::Ref<Absyn::Modification>;
    let mut smod: metamodelica::Ref<SCode::Mod>;
    let __pa0 = ::match_deref::match_deref! { match &(Parser::stringMod({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("dummy")); __mm_s.push_str(&*modifier); ArcStr::from(__mm_s) }, literal!("<internal>"))?) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(__pa0), .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    amod = metamodelica::Own::own(__pa0);
    smod = AbsynToSCode::translateMod(
        Some(amod),
        openmodelica_frontend_types::SCode::Final::NOT_FINAL,
        openmodelica_frontend_types::SCode::Each::NOT_EACH,
        None,
        Absyn::dummyInfo.clone(),
        false,
    )?;
    json = dumpJSONSCodeMod_impl(
        &smod,
        &(openmodelica_nf_frontend::NFInstNode::InstNode::interned_EMPTY_NODE()),
        false,
    )?;
    Ok(json)
}
