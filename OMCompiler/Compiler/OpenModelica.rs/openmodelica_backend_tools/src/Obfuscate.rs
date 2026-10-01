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
use openmodelica_frontend::FBuiltin;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;

pub type Mapping = metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>;

pub type Builtins = metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ElementType>>;

// Most builtin elements are not reserved keywords and can be shadowed by
// user elements. To try and avoid issues when we have e.g. a component named
// abs we keep track of what types of builtin elements we have and what type
// of element we're looking for with this enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum ElementType {
    TYPE = 1,
    FUNCTION = 2,
    TYPE_AND_FUNCTION = 3,
    OTHER = 4,
}
impl PartialOrd for ElementType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ElementType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for ElementType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for ElementType {
    fn default() -> Self {
        Self::TYPE
    }
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Env {
    pub mapping: Mapping,
    pub builtins: Builtins,
}

impl metamodelica::gc::MMTrace for Env {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.mapping, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.builtins, __mmv)?;
        Ok(())
    }
}
impl Default for Env {
    fn default() -> Self {
        Self {
            mapping: Default::default(),
            builtins: Default::default(),
        }
    }
}

pub type ENV = Env;

pub fn obfuscateProgram(
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut classComment: metamodelica::Ref<SCode::Comment>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::Ref<Absyn::Path>,
    metamodelica::Ref<SCode::Comment>,
    ArcStr,
    Mapping,
)> {
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>> = program;
    let mut classPath: metamodelica::Ref<Absyn::Path> = classPath;
    let mut classComment: metamodelica::Ref<SCode::Comment> = classComment;
    let mut mapStr: ArcStr;
    let mut mapping: Mapping;
    let mut builtins: Builtins;
    let mut env: Env;
    mapping = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    builtins = makeBuiltins()?;
    env = Env {
        mapping: mapping.clone(),
        builtins: builtins,
    };
    program = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
        for mut e in (program).into_iter().cloned() {
            let __x = obfuscateElement(e.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    classPath = obfuscatePath(classPath, &env, ElementType::TYPE.clone())?;
    classComment = obfuscateComment(classComment, &env)?;
    mapStr = UnorderedMap::toJSON(env.mapping.clone(), &fnptr!(Util::id, _), &fnptr!(Util::id, _))?;
    Ok((program, classPath, classComment, mapStr, mapping))
}

pub(crate) fn makeBuiltins() -> Result<Builtins> {
    let mut builtins: Builtins;
    let mut builtin_scode: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut etype: ElementType;
    builtins = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    (_, builtin_scode) = FBuiltin::getInitialFunctions()?;
    for mut b in &*builtin_scode {
        etype = if (SCodeUtil::isFunction(metamodelica::AsArg::as_arg(&b))) {
            ElementType::FUNCTION.clone()
        } else {
            ElementType::TYPE.clone()
        };
        UnorderedMap::add(
            SCodeUtil::elementName(metamodelica::AsArg::as_arg(&b))?,
            etype,
            builtins.clone(),
        )?;
    }
    UnorderedMap::add(literal!("Boolean"), ElementType::TYPE.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("Clock"), ElementType::TYPE.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("Real"), ElementType::TYPE.clone(), builtins.clone())?;
    UnorderedMap::add(
        literal!("Integer"),
        ElementType::TYPE_AND_FUNCTION.clone(),
        builtins.clone(),
    )?;
    UnorderedMap::add(
        literal!("String"),
        ElementType::TYPE_AND_FUNCTION.clone(),
        builtins.clone(),
    )?;
    UnorderedMap::add(literal!("displayUnit"), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("fixed"), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("max"), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("min"), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("nominal"), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("quantity"), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("start"), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("stateSelect"), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("time"), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("unbounded"), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("uncertain"), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("unit"), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("constructor"), ElementType::FUNCTION.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("destructor"), ElementType::FUNCTION.clone(), builtins.clone())?;
    UnorderedMap::add(literal!("$array"), ElementType::FUNCTION.clone(), builtins.clone())?;
    UnorderedMap::add(
        literal!("equalityConstraint"),
        ElementType::FUNCTION.clone(),
        builtins.clone(),
    )?;
    Ok(builtins)
}

pub(crate) fn obfuscateElement(
    mut element: metamodelica::Ref<SCode::Element>,
    mut env: Env,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut element: metamodelica::Ref<SCode::Element> = element;
    let () = (match &*element {
        SCode::Element::IMPORT { imp: __element_imp, .. } => {
            assign_variant_field!(element => SCode::Element::IMPORT; imp = obfuscateImport(__element_imp.clone(), &env)?);
            ()
        }
        SCode::Element::EXTENDS {
            baseClassPath: __element_baseClassPath,
            ..
        } => {
            assign_variant_field!(element => SCode::Element::EXTENDS;
                baseClassPath = obfuscatePath(__element_baseClassPath.clone(), &env, ElementType::TYPE.clone())?,
                modifications = obfuscateMod(var_field!((*element).modifications, SCode::Element::EXTENDS).clone(), env.clone())?,
                ann = obfuscateAnnotationOpt(var_field!((*element).ann, SCode::Element::EXTENDS).clone(), &env)?
            );
            ()
        }
        SCode::Element::CLASS {
            name: __element_name, ..
        } => {
            assign_variant_field!(element => SCode::Element::CLASS;
                name = obfuscateIdentifier(__element_name.clone(), &env, ElementType::TYPE_AND_FUNCTION.clone())?.0,
                prefixes = obfuscatePrefixes(var_field!((*element).prefixes, SCode::Element::CLASS).clone(), env.clone())?,
                classDef = obfuscateClassDef(var_field!((*element).classDef, SCode::Element::CLASS).clone(), &env)?,
                cmt = obfuscateComment(var_field!((*element).cmt, SCode::Element::CLASS).clone(), &env)?
            );
            ()
        }
        SCode::Element::COMPONENT {
            name: __element_name, ..
        } => {
            assign_variant_field!(element => SCode::Element::COMPONENT;
                name = obfuscateIdentifier(__element_name.clone(), &env, ElementType::OTHER.clone())?.0,
                prefixes = obfuscatePrefixes(var_field!((*element).prefixes, SCode::Element::COMPONENT).clone(), env.clone())?,
                attributes = obfuscateAttributes(var_field!((*element).attributes, SCode::Element::COMPONENT).clone(), env.clone())?,
                typeSpec = obfuscateTypeSpec(var_field!((*element).typeSpec, SCode::Element::COMPONENT).clone(), &env)?,
                modifications = obfuscateMod(var_field!((*element).modifications, SCode::Element::COMPONENT).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*element).comment, SCode::Element::COMPONENT).clone(), &env)?,
                condition = obfuscateExpOpt(var_field!((*element).condition, SCode::Element::COMPONENT).clone(), &env)?
            );
            ()
        }
        _ => (),
    });
    Ok(element)
}

pub(crate) fn obfuscateImport(mut imp: Absyn::Import, mut env: &Env) -> Result<Absyn::Import> {
    let mut imp: Absyn::Import = imp;
    let () = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => {
            let __owned_variant_name_0 = obfuscateIdentifier(
                var_field!(imp.name, Absyn::Import::NAMED_IMPORT).clone(),
                env,
                ElementType::OTHER.clone(),
            )?
            .0;
            let __owned_variant_path_1 = obfuscatePath(
                var_field!(imp.path, Absyn::Import::NAMED_IMPORT).clone(),
                env,
                ElementType::TYPE.clone(),
            )?;
            if let Absyn::Import::NAMED_IMPORT { name, path, .. } = &mut imp {
                *name = __owned_variant_name_0;
                *path = __owned_variant_path_1;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Absyn::Import::NAMED_IMPORT");
            }
            ()
        }
        Absyn::Import::QUAL_IMPORT { .. } => {
            let __owned_variant_path_0 = obfuscatePath(
                var_field!(imp.path, Absyn::Import::QUAL_IMPORT).clone(),
                env,
                ElementType::TYPE.clone(),
            )?;
            if let Absyn::Import::QUAL_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Absyn::Import::QUAL_IMPORT");
            }
            ()
        }
        Absyn::Import::UNQUAL_IMPORT { .. } => {
            let __owned_variant_path_0 = obfuscatePath(
                var_field!(imp.path, Absyn::Import::UNQUAL_IMPORT).clone(),
                env,
                ElementType::TYPE.clone(),
            )?;
            if let Absyn::Import::UNQUAL_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Absyn::Import::UNQUAL_IMPORT");
            }
            ()
        }
        Absyn::Import::GROUP_IMPORT { .. } => {
            let __owned_variant_prefix_0 = obfuscatePath(
                var_field!(imp.prefix, Absyn::Import::GROUP_IMPORT).clone(),
                env,
                ElementType::TYPE.clone(),
            )?;
            let __owned_variant_groups_1 = ({
                let mut __acc: metamodelica::List<Absyn::GroupImport> = metamodelica::nil();
                for mut g in (var_field!(imp.groups, Absyn::Import::GROUP_IMPORT).clone())
                    .into_iter()
                    .cloned()
                {
                    let __x = obfuscateGroupImport(g.clone(), env)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            if let Absyn::Import::GROUP_IMPORT { prefix, groups, .. } = &mut imp {
                *prefix = __owned_variant_prefix_0;
                *groups = __owned_variant_groups_1;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Absyn::Import::GROUP_IMPORT");
            }
            ()
        }
    });
    Ok(imp)
}

pub(crate) fn obfuscateGroupImport(mut imp: Absyn::GroupImport, mut env: &Env) -> Result<Absyn::GroupImport> {
    let mut imp: Absyn::GroupImport = imp;
    let () = (match imp.clone() {
        Absyn::GroupImport::GROUP_IMPORT_NAME { .. } => {
            let __owned_variant_name_0 = obfuscateIdentifier(
                var_field!(imp.name, Absyn::GroupImport::GROUP_IMPORT_NAME).clone(),
                env,
                ElementType::TYPE.clone(),
            )?
            .0;
            if let Absyn::GroupImport::GROUP_IMPORT_NAME { name, .. } = &mut imp {
                *name = __owned_variant_name_0;
            } else {
                panic!(
                    "owned-variant field-assign: value held a different variant than Absyn::GroupImport::GROUP_IMPORT_NAME"
                );
            }
            ()
        }
        Absyn::GroupImport::GROUP_IMPORT_RENAME { .. } => {
            let __owned_variant_rename_0 = obfuscateIdentifier(
                var_field!(imp.rename, Absyn::GroupImport::GROUP_IMPORT_RENAME).clone(),
                env,
                ElementType::OTHER.clone(),
            )?
            .0;
            let __owned_variant_name_1 = obfuscateIdentifier(
                var_field!(imp.name, Absyn::GroupImport::GROUP_IMPORT_RENAME).clone(),
                env,
                ElementType::TYPE.clone(),
            )?
            .0;
            if let Absyn::GroupImport::GROUP_IMPORT_RENAME { rename, name, .. } = &mut imp {
                *rename = __owned_variant_rename_0;
                *name = __owned_variant_name_1;
            } else {
                panic!(
                    "owned-variant field-assign: value held a different variant than Absyn::GroupImport::GROUP_IMPORT_RENAME"
                );
            }
            ()
        }
    });
    Ok(imp)
}

pub(crate) fn obfuscateClassDef(
    mut cdef: metamodelica::Ref<SCode::ClassDef>,
    mut env: &Env,
) -> Result<metamodelica::Ref<SCode::ClassDef>> {
    let mut cdef: metamodelica::Ref<SCode::ClassDef> = cdef;
    let () = (match &*cdef {
        SCode::ClassDef::PARTS {
            elementLst: __cdef_elementLst,
            ..
        } => {
            assign_variant_field!(cdef => SCode::ClassDef::PARTS;
                        elementLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                for mut e in (__cdef_elementLst.clone()).into_iter().cloned() {
                    let __x = obfuscateElement(e.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        normalEquationLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
                for mut e in (var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
                    let __x = obfuscateEquation(e.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        initialEquationLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
                for mut e in (var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
                    let __x = obfuscateEquation(e.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        normalAlgorithmLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>> = metamodelica::nil();
                for mut a in (var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
                    let __x = obfuscateAlgorithm(a.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        initialAlgorithmLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>> = metamodelica::nil();
                for mut a in (var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
                    let __x = obfuscateAlgorithm(a.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        externalDecl = Util::applyOption(var_field!((*cdef).externalDecl, SCode::ClassDef::PARTS).clone(), &({ let __pe_b1 = env.clone(); move |__pe_a0| obfuscateExternalDecl(__pe_a0, __pe_b1.clone()) }))?
                    );
            ()
        }
        SCode::ClassDef::CLASS_EXTENDS {
            modifications: __cdef_modifications,
            ..
        } => {
            assign_variant_field!(cdef => SCode::ClassDef::CLASS_EXTENDS;
                modifications = obfuscateMod(__cdef_modifications.clone(), env.clone())?,
                composition = obfuscateClassDef(var_field!((*cdef).composition, SCode::ClassDef::CLASS_EXTENDS).clone(), env)?
            );
            ()
        }
        SCode::ClassDef::DERIVED {
            typeSpec: __cdef_typeSpec,
            ..
        } => {
            assign_variant_field!(cdef => SCode::ClassDef::DERIVED;
                typeSpec = obfuscateTypeSpec(__cdef_typeSpec.clone(), env)?,
                modifications = obfuscateMod(var_field!((*cdef).modifications, SCode::ClassDef::DERIVED).clone(), env.clone())?,
                attributes = obfuscateAttributes(var_field!((*cdef).attributes, SCode::ClassDef::DERIVED).clone(), env.clone())?
            );
            ()
        }
        SCode::ClassDef::ENUMERATION {
            enumLst: __cdef_enumLst,
        } => {
            assign_variant_field!(cdef => SCode::ClassDef::ENUMERATION; enumLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Enum>> = metamodelica::nil();
                for mut e in (__cdef_enumLst.clone()).into_iter().cloned() {
                    let __x = obfuscateEnum(e.clone(), env)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        SCode::ClassDef::OVERLOAD {
            pathLst: __cdef_pathLst,
        } => {
            assign_variant_field!(cdef => SCode::ClassDef::OVERLOAD; pathLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
                for mut p in (__cdef_pathLst.clone()).into_iter().cloned() {
                    let __x = obfuscatePath(p.clone(), env, ElementType::TYPE.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        SCode::ClassDef::PDER {
            functionPath: __cdef_functionPath,
            ..
        } => {
            assign_variant_field!(cdef => SCode::ClassDef::PDER;
                        functionPath = obfuscatePath(__cdef_functionPath.clone(), env, ElementType::FUNCTION.clone())?,
                        derivedVariables = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut v in (var_field!((*cdef).derivedVariables, SCode::ClassDef::PDER).clone()).into_iter().cloned() {
                    let __x = (obfuscateIdentifier(v.clone(), env, ElementType::OTHER.clone())?).0;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
    });
    Ok(cdef)
}

pub(crate) fn obfuscateTypeSpec(
    mut ty: metamodelica::Ref<Absyn::TypeSpec>,
    mut env: &Env,
) -> Result<metamodelica::Ref<Absyn::TypeSpec>> {
    let mut ty: metamodelica::Ref<Absyn::TypeSpec> = ty;
    let () = (match &*ty {
        Absyn::TypeSpec::TPATH { path: __ty_path, .. } => {
            assign_variant_field!(ty => Absyn::TypeSpec::TPATH;
                path = obfuscatePath(__ty_path.clone(), env, ElementType::TYPE.clone())?,
                arrayDim = obfuscateArrayDimsOpt(var_field!((*ty).arrayDim, Absyn::TypeSpec::TPATH).clone(), env)?
            );
            ()
        }
        Absyn::TypeSpec::TCOMPLEX { path: __ty_path, .. } => {
            assign_variant_field!(ty => Absyn::TypeSpec::TCOMPLEX;
                        path = obfuscatePath(__ty_path.clone(), env, ElementType::TYPE.clone())?,
                        typeSpecs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::TypeSpec>> = metamodelica::nil();
                for mut t in (var_field!((*ty).typeSpecs, Absyn::TypeSpec::TCOMPLEX).clone()).into_iter().cloned() {
                    let __x = obfuscateTypeSpec(t.clone(), env)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        arrayDim = obfuscateArrayDimsOpt(var_field!((*ty).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone(), env)?
                    );
            ()
        }
    });
    Ok(ty)
}

pub(crate) fn obfuscateEnum(
    mut r#enum: metamodelica::Ref<SCode::Enum>,
    mut env: &Env,
) -> Result<metamodelica::Ref<SCode::Enum>> {
    let mut r#enum: metamodelica::Ref<SCode::Enum> = r#enum;
    assign_field!(
        r#enum.literal = obfuscateIdentifier(r#enum.literal.clone(), env, ElementType::OTHER.clone())?.0,
        r#enum.comment = obfuscateComment(r#enum.comment.clone(), env)?
    );
    Ok(r#enum)
}

pub(crate) fn obfuscatePrefixes(
    mut prefixes: metamodelica::Ref<SCode::Prefixes>,
    mut env: Env,
) -> Result<metamodelica::Ref<SCode::Prefixes>> {
    let mut prefixes: metamodelica::Ref<SCode::Prefixes> = prefixes;
    assign_field!(prefixes.replaceablePrefix = obfuscateReplaceable(prefixes.replaceablePrefix.clone(), env)?);
    Ok(prefixes)
}

pub(crate) fn obfuscateReplaceable(
    mut repl: metamodelica::Ref<SCode::Replaceable>,
    mut env: Env,
) -> Result<metamodelica::Ref<SCode::Replaceable>> {
    let mut repl: metamodelica::Ref<SCode::Replaceable> = repl;
    let mut cc: metamodelica::Ref<SCode::ConstrainClass>;
    let () = (::match_deref::match_deref! { match &(repl.clone()) {
        Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(__esc_cc) } => {
            cc = (*__esc_cc).clone();
            assign_field!(
                cc.constrainingClass = obfuscatePath(cc.constrainingClass.clone(), &env, ElementType::OTHER.clone())?,
                cc.modifier = obfuscateMod(cc.modifier.clone(), env.clone())?,
                cc.comment = obfuscateComment(cc.comment.clone(), &env)?
            );
            assign_variant_field!(repl => SCode::Replaceable::REPLACEABLE; cc = Some(cc.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(repl)
}

pub(crate) fn obfuscateAttributes(mut attributes: SCode::Attributes, mut env: Env) -> Result<SCode::Attributes> {
    let mut attributes: SCode::Attributes = attributes;
    attributes.arrayDims = obfuscateArrayDims(attributes.arrayDims.clone(), env)?;
    Ok(attributes)
}

pub(crate) fn obfuscateMod(
    mut r#mod: metamodelica::Ref<SCode::Mod>,
    mut env: Env,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod;
    let () = (match &*r#mod {
        SCode::Mod::MOD {
            subModLst: __mod_subModLst,
            ..
        } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD;
                        subModLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::SubMod>> = metamodelica::nil();
                for mut s in (__mod_subModLst.clone()).into_iter().cloned() {
                    let __x = obfuscateSubMod(s.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        binding = obfuscateExpOpt(var_field!((*r#mod).binding, SCode::Mod::MOD).clone(), &env)?
                    );
            ()
        }
        SCode::Mod::REDECL {
            element: __mod_element, ..
        } => {
            assign_variant_field!(r#mod => SCode::Mod::REDECL; element = obfuscateElement(__mod_element.clone(), env)?);
            ()
        }
        _ => (),
    });
    Ok(r#mod)
}

pub(crate) fn obfuscateSubMod(
    mut r#mod: metamodelica::Ref<SCode::SubMod>,
    mut env: Env,
) -> Result<metamodelica::Ref<SCode::SubMod>> {
    let mut r#mod: metamodelica::Ref<SCode::SubMod> = r#mod;
    assign_field!(
        r#mod.ident = obfuscateIdentifier(r#mod.ident.clone(), &env, ElementType::OTHER.clone())?.0,
        r#mod.r#mod = obfuscateMod(r#mod.r#mod.clone(), env)?
    );
    Ok(r#mod)
}

pub(crate) fn obfuscatePath(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut env: &Env,
    mut etype: ElementType,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut path: metamodelica::Ref<Absyn::Path> = path;
    let mut name: ArcStr;
    let () = (match &*path {
        Absyn::Path::IDENT { name: __path_name } => {
            (name, _) = obfuscateIdentifier(__path_name.clone(), env, etype)?;
            if referenceEq(&*(name.clone()), &*(__path_name.clone())) {
                return Ok(path);
            }
            assign_variant_field!(path => Absyn::Path::IDENT; name = name);
            ()
        }
        Absyn::Path::QUALIFIED { name: __path_name, .. } => {
            (name, _) = obfuscateIdentifier(__path_name.clone(), env, etype)?;
            if referenceEq(&*(name.clone()), &*(__path_name.clone())) {
                return Ok(path);
            }
            assign_variant_field!(path => Absyn::Path::QUALIFIED;
                name = name,
                path = obfuscatePath(var_field!((*path).path, Absyn::Path::QUALIFIED).clone(), env, etype)?
            );
            ()
        }
        Absyn::Path::FULLYQUALIFIED { path: __path_path } => {
            assign_variant_field!(path => Absyn::Path::FULLYQUALIFIED; path = obfuscatePath(__path_path.clone(), env, etype)?);
            ()
        }
    });
    Ok(path)
}

pub(crate) fn obfuscateIdentifier(
    mut id: ArcStr,
    mut env: &Env,
    mut etype: ElementType,
) -> Result<(ArcStr, ElementType)> {
    let mut outId: ArcStr;
    let mut foundType: ElementType;
    let mut builtins: Builtins = env.builtins.clone();
    let mut mapping: Mapping = env.mapping.clone();
    let mut opt_ety: Option<ElementType>;
    opt_ety = UnorderedMap::get(id.clone(), builtins)?;
    if (opt_ety).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(opt_ety) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        foundType = metamodelica::Own::own(__pa0);
        if isBuiltinInContext(etype, foundType) {
            outId = id;
            return Ok((outId, foundType));
        }
    } else {
        foundType = ElementType::OTHER.clone();
    }
    outId = UnorderedMap::addUpdate(
        id,
        &({
            let __pe_b1 = UnorderedMap::size(mapping.clone());
            move |__pe_a0| makeId(__pe_a0, __pe_b1.clone())
        }),
        mapping,
    )?;
    Ok((outId, foundType))
}

pub(crate) fn isBuiltinInContext(mut expectedType: ElementType, mut actualType: ElementType) -> bool {
    let mut res: bool;
    res = (match (expectedType, actualType) {
        (ElementType::TYPE { .. }, ElementType::TYPE { .. }) => true,
        (ElementType::TYPE { .. }, ElementType::TYPE_AND_FUNCTION) => true,
        (ElementType::FUNCTION { .. }, ElementType::FUNCTION { .. }) => true,
        (ElementType::FUNCTION { .. }, ElementType::TYPE_AND_FUNCTION) => true,
        (ElementType::TYPE_AND_FUNCTION, ElementType::TYPE { .. }) => true,
        (ElementType::TYPE_AND_FUNCTION, ElementType::FUNCTION { .. }) => true,
        (ElementType::TYPE_AND_FUNCTION, ElementType::TYPE_AND_FUNCTION) => true,
        (_, ElementType::TYPE { .. }) => true,
        (_, ElementType::OTHER) => true,
        _ => false,
    });
    res
}

pub(crate) fn makeId(mut oldId: Option<ArcStr>, mut index: i32) -> Result<ArcStr> {
    let mut id: ArcStr;
    if (oldId).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(oldId) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        id = metamodelica::Own::own(__pa0);
    } else {
        id = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("n"));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index)));
            ArcStr::from(__mm_s)
        };
    }
    Ok(id)
}

pub(crate) fn obfuscateComment(
    mut comment: metamodelica::Ref<SCode::Comment>,
    mut env: &Env,
) -> Result<metamodelica::Ref<SCode::Comment>> {
    let mut comment: metamodelica::Ref<SCode::Comment> = comment;
    assign_field!(
        comment.annotation_ = obfuscateAnnotationOpt(comment.annotation_.clone(), env)?,
        comment.comment = None
    );
    Ok(comment)
}

pub(crate) fn obfuscateAnnotationOpt(
    mut ann: Option<metamodelica::Ref<SCode::Annotation>>,
    mut env: &Env,
) -> Result<Option<metamodelica::Ref<SCode::Annotation>>> {
    let mut ann: Option<metamodelica::Ref<SCode::Annotation>> = ann;
    ann = Util::applyOption(
        ann,
        &({
            let __pe_b1 = env.clone();
            move |__pe_a0| obfuscateAnnotation(__pe_a0, &__pe_b1)
        }),
    )?;
    Ok(ann)
}

pub(crate) fn obfuscateAnnotation(
    mut ann: metamodelica::Ref<SCode::Annotation>,
    mut env: &Env,
) -> Result<metamodelica::Ref<SCode::Annotation>> {
    let mut ann: metamodelica::Ref<SCode::Annotation> = ann;
    assign_field!(ann.modification = obfuscateAnnotationMod(ann.modification.clone(), env, false, true)?);
    Ok(ann)
}

pub(crate) fn obfuscateAnnotationMod(
    mut r#mod: metamodelica::Ref<SCode::Mod>,
    mut env: &Env,
    mut obfuscateName: bool,
    mut obfuscateBinding: bool,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod;
    let () = (match &*r#mod {
        SCode::Mod::MOD {
            subModLst: __mod_subModLst,
            ..
        } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::SubMod>> = metamodelica::nil();
                for mut s in (__mod_subModLst.clone()).into_iter().cloned() {
                    if !(isAllowedAnnotation(&(s.clone()))) { continue; }
                    let __x = obfuscateAnnotationSubMod(s.clone(), env, obfuscateName)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            if obfuscateBinding {
                assign_variant_field!(r#mod => SCode::Mod::MOD; binding = obfuscateExpOpt(var_field!((*r#mod).binding, SCode::Mod::MOD).clone(), env)?);
            }
            ()
        }
        _ => (),
    });
    Ok(r#mod)
}

pub(crate) fn isAllowedAnnotation(mut r#mod: &metamodelica::Ref<SCode::SubMod>) -> bool {
    let mut allowed: bool;
    allowed = (::match_deref::match_deref! { match &(r#mod.ident.clone()) {
        Deref @ "Icon" => false,
        Deref @ "Diagram" => false,
        Deref @ "Dialog" => false,
        Deref @ "IconMap" => false,
        Deref @ "DiagramMap" => false,
        Deref @ "Placement" => false,
        Deref @ "Text" => false,
        Deref @ "Line" => false,
        Deref @ "defaultComponentName" => false,
        Deref @ "defaultComponentPrefixes" => false,
        Deref @ "missingInnerMessage" => false,
        Deref @ "obsolete" => false,
        Deref @ "unassignedMessage" => false,
        Deref @ "Protection" => false,
        Deref @ "Authorization" => false,
        _ => StringUtil::startsWith(r#mod.ident.clone(), literal!("__OpenModelica")) || !(StringUtil::startsWith(r#mod.ident.clone(), literal!("__"))),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    allowed
}

pub(crate) fn obfuscateAnnotationSubMod(
    mut r#mod: metamodelica::Ref<SCode::SubMod>,
    mut env: &Env,
    mut obfuscateName: bool,
) -> Result<metamodelica::Ref<SCode::SubMod>> {
    let mut r#mod: metamodelica::Ref<SCode::SubMod> = r#mod;
    let mut obfuscate_name: bool;
    let mut obfuscate_binding: bool;
    if obfuscateName {
        assign_field!(r#mod.ident = obfuscateIdentifier(r#mod.ident.clone(), env, ElementType::OTHER.clone())?.0);
    }
    obfuscate_name = (::match_deref::match_deref! { match &(r#mod.ident.clone()) {
        Deref @ "inverse" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    obfuscate_binding = (::match_deref::match_deref! { match &(r#mod.ident.clone()) {
        Deref @ "__OpenModelica_tearingSelect" => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    assign_field!(r#mod.r#mod = obfuscateAnnotationMod(r#mod.r#mod.clone(), env, obfuscate_name, obfuscate_binding)?);
    Ok(r#mod)
}

pub(crate) fn obfuscateExpOpt(
    mut exp: Option<metamodelica::Ref<Absyn::Exp>>,
    mut env: &Env,
) -> Result<Option<metamodelica::Ref<Absyn::Exp>>> {
    let mut exp: Option<metamodelica::Ref<Absyn::Exp>> = exp;
    exp = Util::applyOption(
        exp,
        &({
            let __pe_b1 = env.clone();
            move |__pe_a0| obfuscateExp(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(exp)
}

pub(crate) fn obfuscateExp(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut env: Env,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    (exp, _) = AbsynUtil::traverseExp(
        exp,
        (std::sync::Arc::new(obfuscateExpTraverse)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Env) -> Result<(metamodelica::Ref<Absyn::Exp>, Env)>
                    + 'static,
            >),
        env,
    )?;
    Ok(exp)
}

pub(crate) fn obfuscateExpTraverse(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut env: Env,
) -> Result<(metamodelica::Ref<Absyn::Exp>, Env)> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut env: Env = env;
    let () = (match &*exp {
        Absyn::Exp::CREF {
            componentRef: __exp_componentRef,
        } => {
            assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = obfuscateCref(__exp_componentRef.clone(), &env, ElementType::OTHER.clone(), false)?);
            ()
        }
        Absyn::Exp::CALL {
            functionArgs: __exp_functionArgs,
            function_: __exp_function_,
            ..
        } => {
            assign_variant_field!(exp => Absyn::Exp::CALL;
                functionArgs = obfuscateFunctionArgs(__exp_functionArgs.clone(), metamodelica::AsArg::as_arg(&__exp_function_), &env)?,
                function_ = obfuscateCref(var_field!((*exp).function_, Absyn::Exp::CALL).clone(), &env, ElementType::FUNCTION.clone(), false)?
            );
            ()
        }
        Absyn::Exp::PARTEVALFUNCTION {
            functionArgs: __exp_functionArgs,
            function_: __exp_function_,
        } => {
            assign_variant_field!(exp => Absyn::Exp::PARTEVALFUNCTION;
                functionArgs = obfuscateFunctionArgs(__exp_functionArgs.clone(), metamodelica::AsArg::as_arg(&__exp_function_), &env)?,
                function_ = obfuscateCref(var_field!((*exp).function_, Absyn::Exp::PARTEVALFUNCTION).clone(), &env, ElementType::OTHER.clone(), false)?
            );
            ()
        }
        _ => (),
    });
    Ok((exp, env))
}

pub(crate) fn obfuscateCref(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut env: &Env,
    mut etype: ElementType,
    mut obfuscateSubs: bool,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let mut name: ArcStr;
    let mut ety: ElementType;
    let () = (match &*cref {
        Absyn::ComponentRef::CREF_IDENT { name: __cref_name, .. } => {
            (name, _) = obfuscateIdentifier(__cref_name.clone(), env, etype)?;
            if referenceEq(&*(name.clone()), &*(__cref_name.clone())) {
                return Ok(cref);
            }
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; name = name);
            if obfuscateSubs {
                assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; subscripts = obfuscateSubscripts(var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(), env.clone())?);
            }
            ()
        }
        Absyn::ComponentRef::CREF_QUAL { name: __cref_name, .. } => {
            (name, ety) = obfuscateIdentifier(__cref_name.clone(), env, etype)?;
            if !(referenceEq(
                &*(name.clone()),
                &*(var_field!((*cref).name, Absyn::ComponentRef::CREF_QUAL).clone()),
            )) {
                assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; name = name);
            }
            if ety == ElementType::OTHER.clone() {
                if obfuscateSubs {
                    assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; subscripts = obfuscateSubscripts(var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(), env.clone())?);
                }
                assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; componentRef = obfuscateCref(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), env, etype, obfuscateSubs)?);
            }
            ()
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED {
            componentRef: __cref_componentRef,
        } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_FULLYQUALIFIED; componentRef = obfuscateCref(__cref_componentRef.clone(), env, etype, obfuscateSubs)?);
            ()
        }
        _ => (),
    });
    Ok(cref)
}

pub fn obfuscateSubscripts(
    mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut env: Env,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>> {
    let mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = subs;
    subs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
        for mut s in (subs).into_iter().cloned() {
            let __x = obfuscateSubscript(s.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(subs)
}

pub(crate) fn obfuscateSubscript(
    mut sub: metamodelica::Ref<Absyn::Subscript>,
    mut env: Env,
) -> Result<metamodelica::Ref<Absyn::Subscript>> {
    let mut sub: metamodelica::Ref<Absyn::Subscript> = sub;
    let () = (match &*sub {
        Absyn::Subscript::SUBSCRIPT {
            subscript: __sub_subscript,
        } => {
            assign_variant_field!(sub => Absyn::Subscript::SUBSCRIPT; subscript = obfuscateExp(__sub_subscript.clone(), env)?);
            ()
        }
        _ => (),
    });
    Ok(sub)
}

pub(crate) fn obfuscateFunctionArgs(
    mut args: metamodelica::Ref<Absyn::FunctionArgs>,
    mut fnName: &metamodelica::Ref<Absyn::ComponentRef>,
    mut env: &Env,
) -> Result<metamodelica::Ref<Absyn::FunctionArgs>> {
    let mut args: metamodelica::Ref<Absyn::FunctionArgs> = args;
    let () = (match &*args {
        Absyn::FunctionArgs::FUNCTIONARGS {
            argNames: __args_argNames,
            ..
        } if (!((__args_argNames).is_empty()) && !(isBuiltinCall(fnName, env)?)) => {
            assign_variant_field!(args => Absyn::FunctionArgs::FUNCTIONARGS; argNames = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>> = metamodelica::nil();
                for mut a in (__args_argNames.clone()).into_iter().cloned() {
                    let __x = obfuscateNamedArg(a.clone(), env)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::FunctionArgs::FOR_ITER_FARG {
            iterators: __args_iterators,
            ..
        } => {
            assign_variant_field!(args => Absyn::FunctionArgs::FOR_ITER_FARG; iterators = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>> = metamodelica::nil();
                for mut i in (__args_iterators.clone()).into_iter().cloned() {
                    let __x = obfuscateForIterator(i.clone(), env)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    Ok(args)
}

pub(crate) fn obfuscateNamedArg(
    mut arg: metamodelica::Ref<Absyn::NamedArg>,
    mut env: &Env,
) -> Result<metamodelica::Ref<Absyn::NamedArg>> {
    let mut arg: metamodelica::Ref<Absyn::NamedArg> = arg;
    assign_field!(arg.argName = obfuscateIdentifier(arg.argName.clone(), env, ElementType::OTHER.clone())?.0);
    Ok(arg)
}

pub(crate) fn obfuscateForIterator(
    mut iterator: metamodelica::Ref<Absyn::ForIterator>,
    mut env: &Env,
) -> Result<metamodelica::Ref<Absyn::ForIterator>> {
    let mut iterator: metamodelica::Ref<Absyn::ForIterator> = iterator;
    assign_field!(iterator.name = obfuscateIdentifier(iterator.name.clone(), env, ElementType::OTHER.clone())?.0);
    Ok(iterator)
}

pub(crate) fn obfuscateArrayDimsOpt(
    mut dims: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>,
    mut env: &Env,
) -> Result<Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>> {
    let mut dims: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>> = dims;
    dims = Util::applyOption(
        dims,
        &({
            let __pe_b1 = env.clone();
            move |__pe_a0| obfuscateArrayDims(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(dims)
}

pub use obfuscateSubscripts as obfuscateArrayDims;

pub(crate) fn obfuscateExternalDecl(
    mut extDecl: metamodelica::Ref<SCode::ExternalDecl>,
    mut env: Env,
) -> Result<metamodelica::Ref<SCode::ExternalDecl>> {
    let mut extDecl: metamodelica::Ref<SCode::ExternalDecl> = extDecl;
    assign_field!(
        extDecl.args = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
            for mut a in (extDecl.args.clone()).into_iter().cloned() {
                let __x = obfuscateExp(a.clone(), env.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        extDecl.output_ = Util::applyOption(
            extDecl.output_.clone(),
            &({
                let __pe_b1 = env.clone();
                let __pe_b2 = ElementType::OTHER.clone();
                let __pe_b3 = true;
                move |__pe_a0| obfuscateCref(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_b3.clone())
            })
        )?,
        extDecl.annotation_ = obfuscateAnnotationOpt(extDecl.annotation_.clone(), &env)?
    );
    Ok(extDecl)
}

pub(crate) fn obfuscateEquations(
    mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut env: Env,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Equation>>> {
    let mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>> = eql;
    eql = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
        for mut eq in (eql).into_iter().cloned() {
            let __x = obfuscateEquation(eq.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(eql)
}

pub(crate) fn obfuscateEquation(
    mut eq: metamodelica::Ref<SCode::Equation>,
    mut env: Env,
) -> Result<metamodelica::Ref<SCode::Equation>> {
    let mut eq: metamodelica::Ref<SCode::Equation> = eq;
    let () = (match &*eq {
        SCode::Equation::EQ_IF {
            condition: __eq_condition,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_IF;
                        condition = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
                for mut e in (__eq_condition.clone()).into_iter().cloned() {
                    let __x = obfuscateExp(e.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        thenBranch = ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Equation>>> = metamodelica::nil();
                for mut e in (var_field!((*eq).thenBranch, SCode::Equation::EQ_IF).clone()).into_iter().cloned() {
                    let __x = obfuscateEquations(e.clone(), env.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        elseBranch = obfuscateEquations(var_field!((*eq).elseBranch, SCode::Equation::EQ_IF).clone(), env.clone())?,
                        comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_IF).clone(), &env)?
                    );
            ()
        }
        SCode::Equation::EQ_EQUALS {
            expLeft: __eq_expLeft, ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_EQUALS;
                expLeft = obfuscateExp(__eq_expLeft.clone(), env.clone())?,
                expRight = obfuscateExp(var_field!((*eq).expRight, SCode::Equation::EQ_EQUALS).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_EQUALS).clone(), &env)?
            );
            ()
        }
        SCode::Equation::EQ_PDE {
            expLeft: __eq_expLeft, ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_PDE;
                expLeft = obfuscateExp(__eq_expLeft.clone(), env.clone())?,
                expRight = obfuscateExp(var_field!((*eq).expRight, SCode::Equation::EQ_PDE).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_PDE).clone(), &env)?
            );
            ()
        }
        SCode::Equation::EQ_CONNECT {
            crefLeft: __eq_crefLeft,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_CONNECT;
                crefLeft = obfuscateCref(__eq_crefLeft.clone(), &env, ElementType::OTHER.clone(), true)?,
                crefRight = obfuscateCref(var_field!((*eq).crefRight, SCode::Equation::EQ_CONNECT).clone(), &env, ElementType::OTHER.clone(), true)?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_CONNECT).clone(), &env)?
            );
            ()
        }
        SCode::Equation::EQ_FOR { index: __eq_index, .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_FOR;
                index = obfuscateIdentifier(__eq_index.clone(), &env, ElementType::OTHER.clone())?.0,
                range = obfuscateExpOpt(var_field!((*eq).range, SCode::Equation::EQ_FOR).clone(), &env)?,
                eEquationLst = obfuscateEquations(var_field!((*eq).eEquationLst, SCode::Equation::EQ_FOR).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_FOR).clone(), &env)?
            );
            ()
        }
        SCode::Equation::EQ_WHEN {
            condition: __eq_condition,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_WHEN;
                        condition = obfuscateExp(__eq_condition.clone(), env.clone())?,
                        eEquationLst = obfuscateEquations(var_field!((*eq).eEquationLst, SCode::Equation::EQ_WHEN).clone(), env.clone())?,
                        elseBranches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Equation>>)> = metamodelica::nil();
                for mut b in (var_field!((*eq).elseBranches, SCode::Equation::EQ_WHEN).clone()).into_iter().cloned() {
                    let __x = (obfuscateExp(Util::tuple21(b.clone()), env.clone())?, obfuscateEquations(Util::tuple22(b.clone()), env.clone())?);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_WHEN).clone(), &env)?
                    );
            ()
        }
        SCode::Equation::EQ_ASSERT {
            condition: __eq_condition,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_ASSERT;
                condition = obfuscateExp(__eq_condition.clone(), env.clone())?,
                message = obfuscateMessage(var_field!((*eq).message, SCode::Equation::EQ_ASSERT).clone(), &(literal!("assert")))?,
                level = obfuscateExp(var_field!((*eq).level, SCode::Equation::EQ_ASSERT).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_ASSERT).clone(), &env)?
            );
            ()
        }
        SCode::Equation::EQ_TERMINATE {
            message: __eq_message, ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_TERMINATE;
                message = obfuscateMessage(__eq_message.clone(), &(literal!("terminate")))?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_TERMINATE).clone(), &env)?
            );
            ()
        }
        SCode::Equation::EQ_REINIT { cref: __eq_cref, .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_REINIT;
                cref = obfuscateExp(__eq_cref.clone(), env.clone())?,
                expReinit = obfuscateExp(var_field!((*eq).expReinit, SCode::Equation::EQ_REINIT).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_REINIT).clone(), &env)?
            );
            ()
        }
        SCode::Equation::EQ_NORETCALL { exp: __eq_exp, .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_NORETCALL;
                exp = obfuscateExp(__eq_exp.clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_NORETCALL).clone(), &env)?
            );
            ()
        }
    });
    Ok(eq)
}

pub(crate) fn obfuscateMessage(
    mut message: metamodelica::Ref<Absyn::Exp>,
    mut fnName: &ArcStr,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut message: metamodelica::Ref<Absyn::Exp> = message;
    let mut msg_str: ArcStr;
    msg_str = (match &*message {
        Absyn::Exp::STRING { value: __message_value } => __message_value.clone(),
        _ => Dump::printExpStr(message)?,
    });
    msg_str = ArcStr::from(::std::format!("{}", stringHashDjb2(&msg_str)));
    msg_str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*fnName);
        __mm_s.push_str(&*literal!(" message "));
        __mm_s.push_str(&*msg_str);
        ArcStr::from(__mm_s)
    };
    message = metamodelica::Ref::new(Absyn::Exp::STRING { value: msg_str });
    Ok(message)
}

pub(crate) fn obfuscateAlgorithm(
    mut alg: metamodelica::Ref<SCode::AlgorithmSection>,
    mut env: Env,
) -> Result<metamodelica::Ref<SCode::AlgorithmSection>> {
    let mut alg: metamodelica::Ref<SCode::AlgorithmSection> = alg;
    assign_field!(alg.statements = obfuscateStatements(alg.statements.clone(), env)?);
    Ok(alg)
}

pub(crate) fn obfuscateStatements(
    mut stmts: metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    mut env: Env,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Statement>>> {
    let mut stmts: metamodelica::List<metamodelica::Ref<SCode::Statement>> = stmts;
    stmts = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Statement>> = metamodelica::nil();
        for mut s in (stmts).into_iter().cloned() {
            let __x = obfuscateStatement(s.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(stmts)
}

pub(crate) fn obfuscateStatement(
    mut stmt: metamodelica::Ref<SCode::Statement>,
    mut env: Env,
) -> Result<metamodelica::Ref<SCode::Statement>> {
    let mut stmt: metamodelica::Ref<SCode::Statement> = stmt;
    let () = (match &*stmt {
        SCode::Statement::ALG_ASSIGN {
            assignComponent: __stmt_assignComponent,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSIGN;
                assignComponent = obfuscateExp(__stmt_assignComponent.clone(), env.clone())?,
                value = obfuscateExp(var_field!((*stmt).value, SCode::Statement::ALG_ASSIGN).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_ASSIGN).clone(), &env)?
            );
            ()
        }
        SCode::Statement::ALG_IF {
            boolExpr: __stmt_boolExpr,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_IF;
                        boolExpr = obfuscateExp(__stmt_boolExpr.clone(), env.clone())?,
                        trueBranch = obfuscateStatements(var_field!((*stmt).trueBranch, SCode::Statement::ALG_IF).clone(), env.clone())?,
                        elseIfBranch = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Statement>>)> = metamodelica::nil();
                for mut b in (var_field!((*stmt).elseIfBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
                    let __x = (obfuscateExp(Util::tuple21(b.clone()), env.clone())?, obfuscateStatements(Util::tuple22(b.clone()), env.clone())?);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        elseBranch = obfuscateStatements(var_field!((*stmt).elseBranch, SCode::Statement::ALG_IF).clone(), env.clone())?,
                        comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_IF).clone(), &env)?
                    );
            ()
        }
        SCode::Statement::ALG_FOR {
            index: __stmt_index, ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_FOR;
                index = obfuscateIdentifier(__stmt_index.clone(), &env, ElementType::OTHER.clone())?.0,
                range = obfuscateExpOpt(var_field!((*stmt).range, SCode::Statement::ALG_FOR).clone(), &env)?,
                forBody = obfuscateStatements(var_field!((*stmt).forBody, SCode::Statement::ALG_FOR).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_FOR).clone(), &env)?
            );
            ()
        }
        SCode::Statement::ALG_PARFOR {
            index: __stmt_index, ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_PARFOR;
                index = obfuscateIdentifier(__stmt_index.clone(), &env, ElementType::OTHER.clone())?.0,
                range = obfuscateExpOpt(var_field!((*stmt).range, SCode::Statement::ALG_PARFOR).clone(), &env)?,
                parforBody = obfuscateStatements(var_field!((*stmt).parforBody, SCode::Statement::ALG_PARFOR).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_PARFOR).clone(), &env)?
            );
            ()
        }
        SCode::Statement::ALG_WHILE {
            boolExpr: __stmt_boolExpr,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHILE;
                boolExpr = obfuscateExp(__stmt_boolExpr.clone(), env.clone())?,
                whileBody = obfuscateStatements(var_field!((*stmt).whileBody, SCode::Statement::ALG_WHILE).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_WHILE).clone(), &env)?
            );
            ()
        }
        SCode::Statement::ALG_WHEN_A {
            branches: __stmt_branches,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHEN_A;
                        branches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Statement>>)> = metamodelica::nil();
                for mut b in (__stmt_branches.clone()).into_iter().cloned() {
                    let __x = (obfuscateExp(Util::tuple21(b.clone()), env.clone())?, obfuscateStatements(Util::tuple22(b.clone()), env.clone())?);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_WHEN_A).clone(), &env)?
                    );
            ()
        }
        SCode::Statement::ALG_ASSERT {
            condition: __stmt_condition,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSERT;
                condition = obfuscateExp(__stmt_condition.clone(), env.clone())?,
                message = obfuscateMessage(var_field!((*stmt).message, SCode::Statement::ALG_ASSERT).clone(), &(literal!("assert")))?,
                level = obfuscateExp(var_field!((*stmt).level, SCode::Statement::ALG_ASSERT).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_ASSERT).clone(), &env)?
            );
            ()
        }
        SCode::Statement::ALG_TERMINATE {
            message: __stmt_message,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_TERMINATE;
                message = obfuscateMessage(__stmt_message.clone(), &(literal!("terminate")))?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_TERMINATE).clone(), &env)?
            );
            ()
        }
        SCode::Statement::ALG_REINIT { cref: __stmt_cref, .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_REINIT;
                cref = obfuscateExp(__stmt_cref.clone(), env.clone())?,
                newValue = obfuscateExp(var_field!((*stmt).newValue, SCode::Statement::ALG_REINIT).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_REINIT).clone(), &env)?
            );
            ()
        }
        SCode::Statement::ALG_NORETCALL { exp: __stmt_exp, .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_NORETCALL;
                exp = obfuscateExp(__stmt_exp.clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_NORETCALL).clone(), &env)?
            );
            ()
        }
        SCode::Statement::ALG_RETURN {
            comment: __stmt_comment,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_RETURN; comment = obfuscateComment(__stmt_comment.clone(), &env)?);
            ()
        }
        SCode::Statement::ALG_BREAK {
            comment: __stmt_comment,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_BREAK; comment = obfuscateComment(__stmt_comment.clone(), &env)?);
            ()
        }
        _ => (),
    });
    Ok(stmt)
}

pub(crate) fn isBuiltinCall(mut callName: &metamodelica::Ref<Absyn::ComponentRef>, mut env: &Env) -> Result<bool> {
    let mut res: bool;
    let mut name: ArcStr;
    let mut ety: ElementType;
    name = AbsynUtil::crefFirstIdent(callName)?;
    ety = UnorderedMap::getOrDefault(name, env.builtins.clone(), ElementType::OTHER.clone())?;
    res = ety == ElementType::FUNCTION.clone() || ety == ElementType::TYPE_AND_FUNCTION.clone();
    Ok(res)
}
