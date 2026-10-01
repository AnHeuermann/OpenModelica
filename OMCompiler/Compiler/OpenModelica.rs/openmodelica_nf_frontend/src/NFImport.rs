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

use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFInst as Inst;
use crate::NFInstContext;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::MutableWeak;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFImport {
    UNRESOLVED_IMPORT {
        imp: Absyn::Import,
        /// Weakly: the class tree owns the scope this
        ///      import sits in.
        scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        info: SourceInfo,
    },
    RESOLVED_IMPORT {
        /// Weakly: the class tree owns the imported node.
        node: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        shortName: ArcStr,
        info: SourceInfo,
    },
    CONFLICTING_IMPORT {
        imp1: metamodelica::Ref<NFImport>,
        imp2: metamodelica::Ref<NFImport>,
    },
}
impl metamodelica::gc::MMTrace for NFImport {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFImport::UNRESOLVED_IMPORT { imp, scope, info } => {
                metamodelica::gc::MMTrace::mm_accept(imp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            NFImport::RESOLVED_IMPORT { node, shortName, info } => {
                metamodelica::gc::MMTrace::mm_accept(node, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(shortName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            NFImport::CONFLICTING_IMPORT { imp1, imp2 } => {
                metamodelica::gc::MMTrace::mm_accept(imp1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(imp2, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for NFImport {
    fn default() -> Self {
        Self::UNRESOLVED_IMPORT {
            imp: Default::default(),
            scope: Default::default(),
            info: Default::default(),
        }
    }
}
pub use self::NFImport::{CONFLICTING_IMPORT, RESOLVED_IMPORT, UNRESOLVED_IMPORT};
pub(crate) fn name(mut imp: &metamodelica::Ref<NFImport>) -> Result<ArcStr> {
    let mut name: ArcStr;
    name = (match &**imp {
        UNRESOLVED_IMPORT { imp: __imp_imp, .. } => AbsynUtil::importName(metamodelica::AsArg::as_arg(&__imp_imp))?,
        RESOLVED_IMPORT { node: __imp_node, .. } => {
            NFInstNode::InstNode::name(&(NFInstNode::InstNode::borrow(__imp_node.clone())?))?
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(name)
}

pub(crate) fn info(mut imp: &metamodelica::Ref<NFImport>) -> Result<SourceInfo> {
    let mut info: SourceInfo;
    info = (match &**imp {
        UNRESOLVED_IMPORT { info: __imp_info, .. } => __imp_info.clone(),
        RESOLVED_IMPORT { info: __imp_info, .. } => __imp_info.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(info)
}

pub(crate) fn resolve(
    mut imp: metamodelica::Ref<NFImport>,
) -> Result<(metamodelica::Ref<InstNode::InstNode>, bool, metamodelica::Ref<NFImport>)> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut changed: bool;
    let mut outImport: metamodelica::Ref<NFImport>;
    (outImport, node, changed) = (match &*imp.clone() {
        UNRESOLVED_IMPORT {
            imp: __imp_imp,
            info: __imp_info,
            scope: __imp_scope,
        } => {
            (outImport, node) = instQualified(
                metamodelica::AsArg::as_arg(&__imp_imp),
                NFInstNode::InstNode::fromCell(__imp_scope.clone())?,
                __imp_info.clone(),
            )?;
            (outImport, node, true)
        }
        RESOLVED_IMPORT { node: __imp_node, .. } => (imp, NFInstNode::InstNode::fromCell(__imp_node.clone())?, false),
        CONFLICTING_IMPORT {
            imp1: __imp_imp1,
            imp2: __imp_imp2,
        } => {
            printImportError(
                metamodelica::AsArg::as_arg(&__imp_imp1),
                metamodelica::AsArg::as_arg(&__imp_imp2),
            )?;
            return Err("fail");
        }
    });
    Ok((node, changed, outImport))
}

pub fn resolveList(
    mut imps: metamodelica::Array<metamodelica::Ref<NFImport>>,
) -> metamodelica::List<metamodelica::Ref<NFImport>> {
    let mut resolvedImps: metamodelica::List<metamodelica::Ref<NFImport>> = metamodelica::nil();
    let __range0 = imps.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut imp in __range0 {
        if '__try1: {
            (_, _, imp) = unwrap_break_err!(resolve(imp.clone()), '__try1);
            resolvedImps = (match &*imp {
                UNRESOLVED_IMPORT {
                    imp: Absyn::Import::UNQUAL_IMPORT { .. },
                    ..
                } => unwrap_break_err!(instUnqualified(&imp, resolvedImps.clone()), '__try1),
                _ => metamodelica::cons(imp.clone(), resolvedImps.clone()),
            });
            Ok::<(), &'static str>(())
        }
        .is_err()
        {}
    }
    resolvedImps
}

pub(crate) fn instQualified(
    mut imp: &Absyn::Import,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut info: SourceInfo,
) -> Result<(metamodelica::Ref<NFImport>, metamodelica::Ref<InstNode::InstNode>)> {
    let mut outImport: metamodelica::Ref<NFImport>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut short_name: ArcStr;
    node = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => Lookup::lookupImport(
            var_field!(imp.path, Absyn::Import::NAMED_IMPORT).clone(),
            scope,
            info.clone(),
        )?,
        Absyn::Import::QUAL_IMPORT { .. } => Lookup::lookupImport(
            var_field!(imp.path, Absyn::Import::QUAL_IMPORT).clone(),
            scope,
            info.clone(),
        )?,
        _ => return Err("match: no arm matched"),
    });
    short_name = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => var_field!(imp.name, Absyn::Import::NAMED_IMPORT).clone(),
        _ => literal!(""),
    });
    outImport = metamodelica::Ref::new(NFImport::RESOLVED_IMPORT {
        node: NFInstNode::InstNode::scopeRef(node.clone()),
        shortName: short_name,
        info: info,
    });
    Ok((outImport, node))
}

pub(crate) fn instUnqualified(
    mut imp: &metamodelica::Ref<NFImport>,
    mut imps: metamodelica::List<metamodelica::Ref<NFImport>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFImport>>> {
    let mut imps: metamodelica::List<metamodelica::Ref<NFImport>> = imps;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut tree: metamodelica::Ref<ClassTree::ClassTree>;
    let mut info: SourceInfo;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*imp)) {
        Deref @ UNRESOLVED_IMPORT { imp: Absyn::Import::UNQUAL_IMPORT { path: __pa0 }, scope: __pa1, info: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    path = metamodelica::Own::own(__pa0);
    scope = metamodelica::Own::own(__pa1);
    info = metamodelica::Own::own(__pa2);
    node = Lookup::lookupImport(path, NFInstNode::InstNode::fromCell(scope)?, info.clone())?;
    node = Inst::instPackage(node, NFInstContext::NO_CONTEXT.clone())?;
    tree = Class::classTree(NFInstNode::InstNode::getClass(node)?)?;
    let () = (match &*tree {
        ClassTree::FLAT_TREE { .. } => {
            let __range0 = var_field!((*tree).classes, ClassTree::ClassTree::FLAT_TREE)
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut cls in __range0 {
                imps = metamodelica::cons(
                    metamodelica::Ref::new(NFImport::RESOLVED_IMPORT {
                        node: NFInstNode::InstNode::scopeRef(cls),
                        shortName: literal!(""),
                        info: info.clone(),
                    }),
                    imps,
                );
            }
            let __range1 = var_field!((*tree).components, ClassTree::ClassTree::FLAT_TREE)
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut comp in __range1 {
                imps = metamodelica::cons(
                    metamodelica::Ref::new(NFImport::RESOLVED_IMPORT {
                        node: NFInstNode::InstNode::scopeRef(comp),
                        shortName: literal!(""),
                        info: info.clone(),
                    }),
                    imps,
                );
            }
            ()
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFImport.instUnqualified"));
                    __mm_s.push_str(&*literal!(" got invalid class tree"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFImport.mo")),
            )?;
            ()
        }
    });
    Ok(imps)
}

pub(crate) fn printImportError(
    mut imp1: &metamodelica::Ref<NFImport>,
    mut imp2: &metamodelica::Ref<NFImport>,
) -> Result<()> {
    let mut err_msg: ErrorTypes::Message;
    Error::addSourceMessage(&(Error::ERROR_FROM_HERE.clone()), metamodelica::nil(), &(info(imp1)?))?;
    err_msg = (match &**imp2 {
        UNRESOLVED_IMPORT { .. } => Error::MULTIPLE_QUALIFIED_IMPORTS_WITH_SAME_NAME.clone(),
        RESOLVED_IMPORT { .. } => Error::IMPORT_SEVERAL_NAMES.clone(),
        _ => return Err("match: no arm matched"),
    });
    Error::addSourceMessage(&err_msg, list![name(imp2)?], &(info(imp2)?))?;
    Ok(())
}
