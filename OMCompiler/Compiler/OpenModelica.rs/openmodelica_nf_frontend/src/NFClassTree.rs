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

use crate::NFBuiltin;
use crate::NFClass as Class;
use crate::NFComponent as Component;
use crate::NFDuplicateTree as DuplicateTree;
use crate::NFImport as Import;
use crate::NFInst as Inst;
use crate::NFInstContext;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFLookup as Lookup;
use crate::NFModifier::Modifier;
use crate::NFRestriction as Restriction;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::NFLookupTree as LookupTree;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::MutableWeak;

thread_local! { static __EMPTY_TLS: metamodelica::Ref<ClassTree::ClassTree> = metamodelica::Ref::new(ClassTree::ClassTree::PARTIAL_TREE { tree: openmodelica_util::NFLookupTree::Tree::interned_EMPTY(), classes: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), components: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), exts: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), imports: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), duplicates: crate::NFDuplicateTree::Tree::interned_EMPTY() }); }
pub(crate) fn EMPTY() -> metamodelica::Ref<ClassTree::ClassTree> {
    __EMPTY_TLS.with(|__t| __t.clone())
}

thread_local! { static __EMPTY_FLAT_TLS: metamodelica::Ref<ClassTree::ClassTree> = metamodelica::Ref::new(ClassTree::ClassTree::FLAT_TREE { tree: openmodelica_util::NFLookupTree::Tree::interned_EMPTY(), classes: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), components: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), imports: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), duplicates: crate::NFDuplicateTree::Tree::interned_EMPTY() }); }
pub(crate) fn EMPTY_FLAT() -> metamodelica::Ref<ClassTree::ClassTree> {
    __EMPTY_FLAT_TLS.with(|__t| __t.clone())
}

pub type LookupEntry = metamodelica::Ref<LookupTree::Entry::Entry>;

pub type LookupTable =
    metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<LookupTree::Entry::Entry>>>;

pub mod ClassTree {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum ClassTree {
        /// A partial tree allows lookup of local classes and imported elements.
        PARTIAL_TREE {
            tree: metamodelica::Ref<LookupTree::Tree>,
            classes: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
            components: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
            exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
            imports: metamodelica::Array<metamodelica::Ref<Import::NFImport>>,
            duplicates: metamodelica::Ref<DuplicateTree::Tree>,
        },
        /// Like partial tree, but the lookup tree is populated with all named
        ///       elements. The elements have not yet been added to the arrays though, so
        ///       lookup is still restricted to local classes and imported elements.
        EXPANDED_TREE {
            tree: metamodelica::Ref<LookupTree::Tree>,
            classes: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
            components: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
            exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
            imports: metamodelica::Array<metamodelica::Ref<Import::NFImport>>,
            duplicates: metamodelica::Ref<DuplicateTree::Tree>,
        },
        /// Allows lookup of both local and inherited elements.
        INSTANTIATED_TREE {
            tree: metamodelica::Ref<LookupTree::Tree>,
            classes: metamodelica::Array<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>,
            components: metamodelica::Array<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>,
            localComponents: metamodelica::List<i32>,
            exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
            imports: metamodelica::Array<metamodelica::Ref<Import::NFImport>>,
            duplicates: metamodelica::Ref<DuplicateTree::Tree>,
        },
        /// A flattened version of an instantiated tree.
        FLAT_TREE {
            tree: metamodelica::Ref<LookupTree::Tree>,
            classes: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
            components: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
            imports: metamodelica::Array<metamodelica::Ref<Import::NFImport>>,
            duplicates: metamodelica::Ref<DuplicateTree::Tree>,
        },
        EMPTY_TREE,
    }
    impl metamodelica::gc::MMTrace for ClassTree {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                ClassTree::PARTIAL_TREE {
                    tree,
                    classes,
                    components,
                    exts,
                    imports,
                    duplicates,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(tree, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(classes, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(components, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(exts, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(imports, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(duplicates, __mmv)?;
                    Ok(())
                }
                ClassTree::EXPANDED_TREE {
                    tree,
                    classes,
                    components,
                    exts,
                    imports,
                    duplicates,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(tree, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(classes, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(components, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(exts, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(imports, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(duplicates, __mmv)?;
                    Ok(())
                }
                ClassTree::INSTANTIATED_TREE {
                    tree,
                    classes,
                    components,
                    localComponents,
                    exts,
                    imports,
                    duplicates,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(tree, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(classes, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(components, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(localComponents, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(exts, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(imports, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(duplicates, __mmv)?;
                    Ok(())
                }
                ClassTree::FLAT_TREE {
                    tree,
                    classes,
                    components,
                    imports,
                    duplicates,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(tree, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(classes, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(components, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(imports, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(duplicates, __mmv)?;
                    Ok(())
                }
                ClassTree::EMPTY_TREE => Ok(()),
            }
        }
    }
    impl ClassTree {
        pub fn interned_EMPTY_TREE() -> metamodelica::Ref<ClassTree> {
            thread_local! {
                static INTERNED: metamodelica::Ref<ClassTree> = metamodelica::Ref::new(ClassTree::EMPTY_TREE);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_EMPTY_TREE() -> metamodelica::Ref<ClassTree> {
        ClassTree::interned_EMPTY_TREE()
    }
    impl Default for ClassTree {
        fn default() -> Self {
            Self::EMPTY_TREE
        }
    }
    pub use self::ClassTree::{EMPTY_TREE, EXPANDED_TREE, FLAT_TREE, INSTANTIATED_TREE, PARTIAL_TREE};
    pub(crate) fn fromSCode(
        mut elements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
        mut isClassExtends: bool,
        mut parent: metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<metamodelica::Ref<ClassTree>> {
        let mut tree: metamodelica::Ref<ClassTree>;
        let mut ltree: metamodelica::Ref<LookupTree::Tree>;
        let mut lentry: metamodelica::Ref<LookupTree::Entry::Entry>;
        let mut clsc: i32;
        let mut compc: i32;
        let mut extc: i32;
        let mut clss: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut cls_idx: i32 = 0;
        let mut ext_idx: i32 = 0;
        let mut comp_idx: i32 = 0;
        let mut dups: metamodelica::Ref<DuplicateTree::Tree>;
        let mut imps: metamodelica::List<metamodelica::Ref<Import::NFImport>> = metamodelica::nil();
        let mut info: SourceInfo;
        ltree = LookupTree::new();
        (clsc, compc, extc) = countElements(elements);
        if isClassExtends {
            extc = extc + 1;
        }
        clss = metamodelica::arrayCreate(clsc, crate::NFInstNode::InstNode::interned_EMPTY_NODE());
        comps = metamodelica::arrayCreate(compc + extc, crate::NFInstNode::InstNode::interned_EMPTY_NODE());
        exts = metamodelica::arrayCreate(extc, crate::NFInstNode::InstNode::interned_EMPTY_NODE());
        dups = DuplicateTree::new();
        tree = metamodelica::Ref::new(ClassTree::PARTIAL_TREE {
            tree: ltree.clone(),
            classes: clss.clone(),
            components: comps.clone(),
            exts: exts.clone(),
            imports: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
            duplicates: dups.clone(),
        });
        if isClassExtends {
            {
                let __cell0 = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
                let __idx0 = 1;
                let _ =
                    unsafe { metamodelica::Dangerous::arrayInitSlotChecked(exts.clone().clone(), __idx0, __cell0) }?;
            }
            {
                let __cell1 = metamodelica::Ref::new(InstNode::InstNode::REF_NODE { index: 1 });
                let __idx1 = 1;
                let _ =
                    unsafe { metamodelica::Dangerous::arrayInitSlotChecked(comps.clone().clone(), __idx1, __cell1) }?;
            }
            ext_idx = ext_idx + 1;
            comp_idx = comp_idx + 1;
        }
        for mut e in &**elements {
            let () = (match &*e.clone() {
                SCode::Element::CLASS { name: __e_name, .. } => {
                    cls_idx = cls_idx + 1;
                    unsafe {
                        metamodelica::Dangerous::arrayInitSlot(
                            clss.clone(),
                            cls_idx,
                            NFInstNode::InstNode::newClass(
                                e.clone(),
                                parent.clone(),
                                crate::NFInstNode::InstNodeType::interned_NORMAL_CLASS(),
                            )?,
                        )
                    };
                    lentry = metamodelica::Ref::new(LookupTree::Entry::Entry::CLASS { index: cls_idx });
                    ltree = addLocalElement(metamodelica::AsArg::as_arg(&__e_name), &lentry, &tree, ltree)?;
                    if SCodeUtil::isElementRedeclare(metamodelica::AsArg::as_arg(&e))?
                        || SCodeUtil::isClassExtends(metamodelica::AsArg::as_arg(&e))
                    {
                        dups = DuplicateTree::add(
                            dups,
                            metamodelica::AsArg::as_arg(&__e_name),
                            &(DuplicateTree::newRedeclare(lentry)),
                            &*(std::sync::Arc::new(DuplicateTree::addConflictDefault)
                                as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
                        )?;
                    }
                    ()
                }
                SCode::Element::COMPONENT { .. } => {
                    comp_idx = comp_idx + 1;
                    unsafe {
                        metamodelica::Dangerous::arrayInitSlot(
                            comps.clone(),
                            comp_idx,
                            NFInstNode::InstNode::newComponent(
                                e.clone(),
                                crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
                            )?,
                        )
                    };
                    ()
                }
                SCode::Element::EXTENDS { .. } => {
                    ext_idx = ext_idx + 1;
                    unsafe {
                        metamodelica::Dangerous::arrayInitSlot(
                            exts.clone(),
                            ext_idx,
                            NFInstNode::InstNode::newExtends(e.clone(), parent.clone())?,
                        )
                    };
                    comp_idx = comp_idx + 1;
                    unsafe {
                        metamodelica::Dangerous::arrayInitSlot(
                            comps.clone(),
                            comp_idx,
                            metamodelica::Ref::new(InstNode::InstNode::REF_NODE { index: ext_idx }),
                        )
                    };
                    ()
                }
                SCode::Element::IMPORT {
                    imp: __e_imp,
                    info: __e_info,
                    ..
                } => {
                    imps = metamodelica::cons(
                        metamodelica::Ref::new(Import::NFImport::UNRESOLVED_IMPORT {
                            imp: __e_imp.clone(),
                            scope: NFInstNode::InstNode::scopeRef(parent.clone()),
                            info: __e_info.clone(),
                        }),
                        imps,
                    );
                    ()
                }
                _ => return Err("match: no arm matched"),
            });
        }
        tree = metamodelica::Ref::new(ClassTree::PARTIAL_TREE {
            tree: ltree,
            classes: clss.clone(),
            components: comps.clone(),
            exts: exts.clone(),
            imports: metamodelica::arrayFromVec(imps.into_iter().cloned().collect()),
            duplicates: dups,
        });
        Ok(tree)
    }

    pub(crate) fn initImports(
        mut tree: metamodelica::Ref<ClassTree>,
        mut parent: &metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<metamodelica::Ref<ClassTree>> {
        let mut tree: metamodelica::Ref<ClassTree> = tree;
        let mut imports: metamodelica::Array<metamodelica::Ref<Import::NFImport>>;
        let mut init_imports: metamodelica::List<metamodelica::Ref<Import::NFImport>>;
        let mut imp: metamodelica::Ref<Import::NFImport> =
            <metamodelica::Ref<Import::NFImport> as ::std::default::Default>::default();
        let mut ltree: metamodelica::Ref<LookupTree::Tree>;
        let () = (match &*tree {
            PARTIAL_TREE {
                tree: __esc_ltree,
                imports,
                ..
            } if (!(imports.clone().borrow().is_empty())) => {
                ltree = (*__esc_ltree).clone();
                let mut imports = (*imports).clone();
                init_imports = metamodelica::nil();
                let __range0 = imports.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut imp in __range0 {
                    init_imports = (match &*imp {
                        Import::UNRESOLVED_IMPORT {
                            imp: Absyn::Import::UNQUAL_IMPORT { .. },
                            ..
                        } => Import::instUnqualified(&imp, init_imports)?,
                        _ => metamodelica::cons(imp, init_imports),
                    });
                }
                imports = metamodelica::arrayFromVec(init_imports.into_iter().cloned().collect());
                for mut i in ({
                    let __s = metamodelica::arrayLength(imports.clone());
                    let __e = 1;
                    (0i32..)
                        .map(move |__k| __s + __k * (-1))
                        .take_while(move |&__v| __v >= __e)
                }) {
                    ltree = addImport(
                        &({
                            let __elt = (*metamodelica::index_checked(&imports.borrow(), i)?).clone();
                            __elt
                        }),
                        i,
                        ltree.clone(),
                        imports.clone(),
                    )?;
                }
                assign_variant_field!(tree => ClassTree::PARTIAL_TREE;
                    imports = imports.clone(),
                    tree = ltree.clone()
                );
                ()
            }
            _ => (),
        });
        Ok(tree)
    }

    pub(crate) fn fromEnumeration(
        mut literals: &metamodelica::List<metamodelica::Ref<SCode::Enum>>,
        mut enumType: metamodelica::Ref<Type::NFType>,
        mut enumClass: metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<metamodelica::Ref<ClassTree>> {
        let mut tree: metamodelica::Ref<ClassTree>;
        let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut attr_count: i32 = 5;
        let mut i: i32 = 0;
        let mut comp: metamodelica::Ref<InstNode::InstNode>;
        let mut ltree: metamodelica::Ref<LookupTree::Tree>;
        let mut name: ArcStr;
        comps = metamodelica::arrayCreate(
            ((literals).len() as i32) + attr_count,
            crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        );
        ltree = NFBuiltin::ENUM_LOOKUP_TREE.clone();
        unsafe {
            metamodelica::Dangerous::arrayInitSlot(
                comps.clone(),
                1,
                NFInstNode::InstNode::fromComponent(
                    literal!("quantity"),
                    metamodelica::Ref::new(Component::NFComponent::TYPE_ATTRIBUTE {
                        ty: crate::NFType::interned_STRING(),
                        modifier: crate::NFModifier::Modifier::interned_NOMOD(),
                    }),
                    enumClass.clone(),
                ),
            )
        };
        unsafe {
            metamodelica::Dangerous::arrayInitSlot(
                comps.clone(),
                2,
                NFInstNode::InstNode::fromComponent(
                    literal!("min"),
                    metamodelica::Ref::new(Component::NFComponent::TYPE_ATTRIBUTE {
                        ty: enumType.clone(),
                        modifier: crate::NFModifier::Modifier::interned_NOMOD(),
                    }),
                    enumClass.clone(),
                ),
            )
        };
        unsafe {
            metamodelica::Dangerous::arrayInitSlot(
                comps.clone(),
                3,
                NFInstNode::InstNode::fromComponent(
                    literal!("max"),
                    metamodelica::Ref::new(Component::NFComponent::TYPE_ATTRIBUTE {
                        ty: enumType.clone(),
                        modifier: crate::NFModifier::Modifier::interned_NOMOD(),
                    }),
                    enumClass.clone(),
                ),
            )
        };
        unsafe {
            metamodelica::Dangerous::arrayInitSlot(
                comps.clone(),
                4,
                NFInstNode::InstNode::fromComponent(
                    literal!("start"),
                    metamodelica::Ref::new(Component::NFComponent::TYPE_ATTRIBUTE {
                        ty: enumType.clone(),
                        modifier: crate::NFModifier::Modifier::interned_NOMOD(),
                    }),
                    enumClass.clone(),
                ),
            )
        };
        unsafe {
            metamodelica::Dangerous::arrayInitSlot(
                comps.clone(),
                5,
                NFInstNode::InstNode::fromComponent(
                    literal!("fixed"),
                    metamodelica::Ref::new(Component::NFComponent::TYPE_ATTRIBUTE {
                        ty: crate::NFType::interned_BOOLEAN(),
                        modifier: crate::NFModifier::Modifier::interned_NOMOD(),
                    }),
                    enumClass.clone(),
                ),
            )
        };
        for mut l in &**literals {
            name = l.literal.clone();
            i = i + 1;
            comp = NFInstNode::InstNode::fromComponent(
                name.clone(),
                Component::newEnum(enumType.clone(), name.clone(), l.comment.clone(), i),
                enumClass.clone(),
            );
            unsafe { metamodelica::Dangerous::arrayInitSlot(comps.clone(), i + attr_count, comp.clone()) };
            ltree = LookupTree::add(
                ltree,
                &name,
                &(metamodelica::Ref::new(LookupTree::Entry::Entry::COMPONENT { index: i + attr_count })),
                &({
                    let __pe_b3 = comp;
                    move |__pe_a0, __pe_a1, __pe_a2| addEnumConflict(&__pe_a0, &__pe_a1, &__pe_a2, &__pe_b3)
                }),
            )?;
        }
        tree = metamodelica::Ref::new(ClassTree::FLAT_TREE {
            tree: ltree,
            classes: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
            components: comps.clone(),
            imports: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
            duplicates: crate::NFDuplicateTree::Tree::interned_EMPTY(),
        });
        Ok(tree)
    }

    pub(crate) fn addElementsToFlatTree(
        mut elements: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        mut tree: metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<ClassTree>> {
        let mut tree: metamodelica::Ref<ClassTree> = tree;
        let mut ltree: metamodelica::Ref<LookupTree::Tree>;
        let mut cls_arr: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut comp_arr: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut cls_lst: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
        let mut comp_lst: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
        let mut imports: metamodelica::Array<metamodelica::Ref<Import::NFImport>>;
        let mut duplicates: metamodelica::Ref<DuplicateTree::Tree>;
        let mut cls_idx: i32;
        let mut comp_idx: i32;
        let mut lentry: metamodelica::Ref<LookupTree::Entry::Entry>;
        let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(tree.clone()) {
            Deref @ FLAT_TREE { tree: __pa0, classes: __pa1, components: __pa2, imports: __pa3, duplicates: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ltree = metamodelica::Own::own(__pa0);
        cls_arr = metamodelica::Own::own(__pa1);
        comp_arr = metamodelica::Own::own(__pa2);
        imports = metamodelica::Own::own(__pa3);
        duplicates = metamodelica::Own::own(__pa4);
        cls_idx = metamodelica::arrayLength(cls_arr.clone());
        comp_idx = metamodelica::arrayLength(comp_arr.clone());
        for mut e in &**elements {
            if NFInstNode::InstNode::isComponent(metamodelica::AsArg::as_arg(&e))? {
                comp_idx = comp_idx + 1;
                lentry = metamodelica::Ref::new(LookupTree::Entry::Entry::COMPONENT { index: comp_idx });
                comp_lst = metamodelica::cons(e.clone(), comp_lst);
            } else {
                cls_idx = cls_idx + 1;
                lentry = metamodelica::Ref::new(LookupTree::Entry::Entry::CLASS { index: cls_idx });
                cls_lst = metamodelica::cons(e.clone(), cls_lst);
            }
            ltree = addLocalElement(
                &(NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&e))?),
                &lentry,
                &tree,
                ltree,
            )?;
        }
        cls_arr = Array::appendList(cls_arr.clone(), metamodelica::Dangerous::listReverseInPlace(cls_lst))?;
        comp_arr = Array::appendList(comp_arr.clone(), metamodelica::Dangerous::listReverseInPlace(comp_lst))?;
        tree = metamodelica::Ref::new(ClassTree::FLAT_TREE {
            tree: ltree,
            classes: cls_arr.clone(),
            components: comp_arr.clone(),
            imports: imports.clone(),
            duplicates: duplicates,
        });
        Ok(tree)
    }

    pub(crate) fn expand(mut tree: metamodelica::Ref<ClassTree>) -> Result<metamodelica::Ref<ClassTree>> {
        let mut tree: metamodelica::Ref<ClassTree> = tree;
        let mut ltree: metamodelica::Ref<LookupTree::Tree>;
        let mut lentry: metamodelica::Ref<LookupTree::Entry::Entry>;
        let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut clss: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut imps: metamodelica::Array<metamodelica::Ref<Import::NFImport>>;
        let mut ext_idxs: metamodelica::List<(i32, i32)> = metamodelica::nil();
        let mut cls_idx: i32;
        let mut comp_idx: i32 = 1;
        let mut dups: metamodelica::Ref<DuplicateTree::Tree>;
        let mut dups_ptr: Mutable::Mutable<metamodelica::Ref<DuplicateTree::Tree>>;
        let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(tree.clone()) {
            Deref @ PARTIAL_TREE { tree: __pa0, classes: __pa1, components: __pa2, exts: __pa3, imports: __pa4, duplicates: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ltree = metamodelica::Own::own(__pa0);
        clss = metamodelica::Own::own(__pa1);
        comps = metamodelica::Own::own(__pa2);
        exts = metamodelica::Own::own(__pa3);
        imps = metamodelica::Own::own(__pa4);
        dups = metamodelica::Own::own(__pa5);
        cls_idx = metamodelica::arrayLength(clss.clone()) + 1;
        let __range6 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut c in __range6 {
            let () = (match &*c.clone() {
                NFInstNode::InstNode::COMPONENT_NODE { name: __c_name, .. } => {
                    lentry = metamodelica::Ref::new(LookupTree::Entry::Entry::COMPONENT { index: comp_idx });
                    ltree = addLocalElement(&(NFInstNode::InstNode::name(&c)?), &lentry, &tree, ltree)?;
                    if NFInstNode::InstNode::isRedeclare(c)? {
                        dups = DuplicateTree::add(
                            dups,
                            metamodelica::AsArg::as_arg(&__c_name),
                            &(DuplicateTree::newRedeclare(lentry)),
                            &*(std::sync::Arc::new(DuplicateTree::addConflictDefault)
                                as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
                        )?;
                    }
                    comp_idx = comp_idx + 1;
                    ()
                }
                NFInstNode::InstNode::REF_NODE { index: __c_index } => {
                    ext_idxs = metamodelica::cons((cls_idx - 1, comp_idx - 1), ext_idxs);
                    (cls_idx, comp_idx) = countInheritedElements(
                        ({
                            let __elt = (*metamodelica::index_checked(&exts.borrow(), __c_index.clone())?).clone();
                            __elt
                        }),
                        cls_idx,
                        comp_idx,
                    )?;
                    ()
                }
                _ => {
                    Error::terminate(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NFClassTree.ClassTree.expand"));
                            __mm_s.push_str(&*literal!(" got invalid component"));
                            ArcStr::from(__mm_s)
                        },
                        &(metamodelica::sourceInfo!("NFFrontEnd/NFClassTree.mo")),
                    )?;
                    return Err("fail");
                }
            });
        }
        dups_ptr = Mutable::create(dups);
        if !((ext_idxs).is_empty()) {
            ext_idxs = metamodelica::Dangerous::listReverseInPlace(ext_idxs);
            let __range7 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut ext in __range7 {
                let (__pa8, __pa9, __pa10) = ::match_deref::match_deref! { match &(ext_idxs) {
                    Deref @ metamodelica::ListNode::Cons { head: (__pa8, __pa9), tail: __pa10 } => (__pa8.clone(), __pa9.clone(), __pa10.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                cls_idx = metamodelica::Own::own(__pa8);
                comp_idx = metamodelica::Own::own(__pa9);
                ext_idxs = metamodelica::Own::own(__pa10);
                ltree = expandExtends(ext, ltree, cls_idx, comp_idx, dups_ptr.clone())?;
            }
        }
        tree = metamodelica::Ref::new(ClassTree::EXPANDED_TREE {
            tree: ltree,
            classes: clss.clone(),
            components: comps.clone(),
            exts: exts.clone(),
            imports: imps.clone(),
            duplicates: Mutable::access(dups_ptr),
        });
        Ok(tree)
    }

    pub(crate) fn instantiate(
        mut clsNode: metamodelica::Ref<InstNode::InstNode>,
        mut instance: metamodelica::Ref<InstNode::InstNode>,
        mut scope: &metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<InstNode::InstNode>,
        i32,
        i32,
    )> {
        let mut clsNode: metamodelica::Ref<InstNode::InstNode> = clsNode;
        let mut instance: metamodelica::Ref<InstNode::InstNode> = instance;
        let mut classCount: i32 = 0;
        let mut compCount: i32 = 0;
        let mut cls: metamodelica::Ref<Class::NFClass>;
        let mut tree: metamodelica::Ref<ClassTree>;
        let mut ltree: metamodelica::Ref<LookupTree::Tree>;
        let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut old_clss: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut old_comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut imps: metamodelica::Array<metamodelica::Ref<Import::NFImport>>;
        let mut clss: metamodelica::Array<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>;
        let mut comps: metamodelica::Array<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>;
        let mut ext_clss: metamodelica::Array<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>;
        let mut local_comps: metamodelica::List<i32> = metamodelica::nil();
        let mut cls_idx: i32 = 1;
        let mut comp_idx: i32 = 1;
        let mut cls_count: i32;
        let mut comp_count: i32;
        let mut node: metamodelica::Ref<InstNode::InstNode>;
        let mut parent_scope: metamodelica::Ref<InstNode::InstNode>;
        let mut inst_scope: metamodelica::Ref<InstNode::InstNode>;
        let mut inst_ref: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
        let mut dups: metamodelica::Ref<DuplicateTree::Tree>;
        let mut ext_def: metamodelica::Ref<SCode::Element>;
        let mut is_typish: bool;
        let mut inst_ty: metamodelica::Ref<InstNodeType>;
        cls = NFInstNode::InstNode::getClass(clsNode.clone())?;
        clsNode = NFInstNode::InstNode::replaceClass(cls.clone(), clsNode)?;
        clsNode = NFInstNode::InstNode::reidentify(clsNode);
        let () = (::match_deref::match_deref! { match &(cls.clone()) {
            Deref @ Class::EXPANDED_CLASS { elements: Deref @ INSTANTIATED_TREE { .. }, .. } => (),
            Deref @ Class::EXPANDED_CLASS { elements: __cls_elements, restriction: __cls_restriction, .. } => {
                if NFInstNode::InstNode::isEmpty(&instance) {
                    instance = clsNode.clone();
                    parent_scope = NFInstNode::InstNode::instanceParent(clsNode.clone())?;
                } else {
                    parent_scope = instance.clone();
                    inst_scope = scope.clone();
                }
                inst_scope = if (NFInstNode::InstNode::isEmpty(scope)) {instance.clone()} else {scope.clone()};
                let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(__cls_elements.clone()) {
                    Deref @ EXPANDED_TREE { tree: __pa0, classes: __pa1, components: __pa2, exts: __pa3, imports: __pa4, duplicates: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                ltree = metamodelica::Own::own(__pa0);
                old_clss = metamodelica::Own::own(__pa1);
                old_comps = metamodelica::Own::own(__pa2);
                exts = metamodelica::Own::own(__pa3);
                imps = metamodelica::Own::own(__pa4);
                dups = metamodelica::Own::own(__pa5);
                classCount = metamodelica::arrayLength(old_clss.clone());
                compCount = metamodelica::arrayLength(old_comps.clone()) - metamodelica::arrayLength(exts.clone());
                exts = metamodelica::arrayFromVec(exts.clone().borrow().clone());
                for mut i in 1..=metamodelica::arrayLength(exts.clone()) {
                    node = ({let __elt = (*metamodelica::index_checked(&exts.borrow(), i)?).clone(); __elt});
                    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(NFInstNode::InstNode::nodeType(&node)?) {
                        Deref @ NFInstNode::InstNodeType::BASE_CLASS { definition: __pa6, ty: __pa7, .. } => (__pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ext_def = metamodelica::Own::own(__pa6);
                    inst_ty = metamodelica::Own::own(__pa7);
                    node = NFInstNode::InstNode::setNodeType(metamodelica::Ref::new(InstNodeType::BASE_CLASS { parent: NFInstNode::InstNode::identityCell(instance.clone()), definition: ext_def, ty: inst_ty }), node)?;
                    (node, _, cls_count, comp_count) = instantiate(node, crate::NFInstNode::InstNode::interned_EMPTY_NODE(), &inst_scope)?;
                    {
                        let __cell8 = node;
                        let __idx8 = i;
                        *metamodelica::index_mut_checked(&mut exts.clone().borrow_mut(), __idx8)? = __cell8;
                    }
                    classCount = cls_count + classCount;
                    compCount = comp_count + compCount;
                }
                comps = metamodelica::arrayCreate(compCount, Mutable::create(crate::NFInstNode::InstNode::interned_EMPTY_NODE()));
                clss = metamodelica::arrayCreate(classCount, Mutable::create(crate::NFInstNode::InstNode::interned_EMPTY_NODE()));
                is_typish = Restriction::isType(metamodelica::AsArg::as_arg(&__cls_restriction)) || Restriction::isOperatorRecord(metamodelica::AsArg::as_arg(&__cls_restriction)) || Restriction::isOperator(metamodelica::AsArg::as_arg(&__cls_restriction));
                let __range9 = old_clss.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range9 {
                    if is_typish {
                        c = NFInstNode::InstNode::setParent(clsNode.clone(), c)?;
                    } else {
                        c = NFInstNode::InstNode::clone(c)?;
                        c = NFInstNode::InstNode::setParent(instance.clone(), c)?;
                    }
                    if NFInstNode::InstNode::isOuter(&c)? {
                        checkOuterClass(c.clone())?;
                        c = linkInnerOuter(c, parent_scope.clone())?;
                    }
                    unsafe { metamodelica::Dangerous::arrayInitSlot(clss.clone(), cls_idx, Mutable::create(c)) };
                    cls_idx = cls_idx + 1;
                }
                let __range10 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut ext in __range10 {
                    let () = (match &*(Class::classTree(NFInstNode::InstNode::getClass(ext)?)?) {
            INSTANTIATED_TREE { classes: __esc_ext_clss, .. } => {
                ext_clss = (*__esc_ext_clss).clone();
                cls_count = metamodelica::arrayLength(ext_clss.clone());
                if cls_count > 0 {
                    Array::copyRange(ext_clss.clone(), clss.clone(), 1, cls_count, cls_idx)?;
                    cls_idx = cls_idx + cls_count;
                }
                ()
            },
            _ => (),
        });
                }
                inst_ref = NFInstNode::InstNode::identityCell(instance.clone());
                let __range11 = old_comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range11 {
                    let () = (match &*c {
            NFInstNode::InstNode::COMPONENT_NODE { .. } => {
                node = NFInstNode::InstNode::cloneComponentInScope(&c, inst_ref.clone())?;
                if NFInstNode::InstNode::isOuter(&node)? {
                    if '__try0: {
                        node = unwrap_break_err!(linkInnerOuter(node.clone(), inst_scope.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_err() {
                        if !(Flags::isSet(Flags::NF_API.clone())?) {
                            return Err("fail");
                        }
                    }
                }
                unsafe { metamodelica::Dangerous::arrayInitSlot(comps.clone(), comp_idx, Mutable::create(node)) };
                local_comps = metamodelica::cons(comp_idx, local_comps);
                comp_idx = comp_idx + 1;
                ()
            },
            NFInstNode::InstNode::REF_NODE { index: __c_index } => {
                comp_idx = instExtendsComps(({let __elt = (*metamodelica::index_checked(&exts.borrow(), __c_index.clone())?).clone(); __elt}), comps.clone(), comp_idx)?;
                ()
            },
            _ => return Err("match: no arm matched"),
        });
                }
                breakComponents(instance.clone(), comps.clone(), &ltree, &dups)?;
                if comp_idx != compCount + 1 {
                    Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClassTree.ClassTree.instantiate")); __mm_s.push_str(&*literal!(" miscounted components in ")); __mm_s.push_str(&*NFInstNode::InstNode::name(&clsNode)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFClassTree.mo")))?;
                }
                if cls_idx != classCount + 1 {
                    Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClassTree.ClassTree.instantiate")); __mm_s.push_str(&*literal!(" miscounted classes in ")); __mm_s.push_str(&*NFInstNode::InstNode::name(&clsNode)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFClassTree.mo")))?;
                }
                local_comps = metamodelica::Dangerous::listReverseInPlace(local_comps);
                assign_variant_field!(cls => Class::NFClass::EXPANDED_CLASS; elements = metamodelica::Ref::new(ClassTree::INSTANTIATED_TREE { tree: ltree, classes: clss.clone(), components: comps.clone(), localComponents: local_comps, exts: exts.clone(), imports: imps.clone(), duplicates: dups }));
                ()
            },
            Deref @ Class::EXPANDED_DERIVED { baseClass: __esc_node, .. } => {
                node = (*__esc_node).clone();
                node = NFInstNode::InstNode::setNodeType(metamodelica::Ref::new(InstNodeType::BASE_CLASS { parent: NFInstNode::InstNode::identityCell(clsNode.clone()), definition: NFInstNode::InstNode::definition(node.clone())?, ty: NFInstNode::InstNode::nodeType(metamodelica::AsArg::as_arg(&node))? }), node.clone())?;
                (node, instance, classCount, compCount) = instantiate(node.clone(), instance, scope)?;
                assign_variant_field!(cls => Class::NFClass::EXPANDED_DERIVED; baseClass = node.clone());
                ()
            },
            Deref @ Class::PARTIAL_BUILTIN { elements: __esc_tree @ Deref @ FLAT_TREE { components: __esc_old_comps, .. }, .. } => {
                tree = (*__esc_tree).clone();
                old_comps = (*__esc_old_comps).clone();
                instance = if (NFInstNode::InstNode::isEmpty(&instance)) {clsNode.clone()} else {instance};
                inst_ref = NFInstNode::InstNode::identityCell(instance.clone());
                assign_variant_field!(tree => ClassTree::FLAT_TREE; components = Array::map(old_comps.clone(), &({ let __pe_b1 = inst_ref; move |__pe_a0| NFInstNode::InstNode::cloneComponentInScope(&__pe_a0, __pe_b1.clone()) }))?);
                assign_variant_field!(cls => Class::NFClass::PARTIAL_BUILTIN; elements = tree.clone());
                compCount = metamodelica::arrayLength(old_comps.clone());
                for mut bm in &*getBreakModsInExtend(&instance)? {
                    Error::addSourceMessage(&(Error::NON_BREAKABLE_ELEMENT.clone()), list![bm.ident.clone()], &(SCodeUtil::getModifierInfo(&bm.r#mod)))?;
                    return Err("fail");
                }
                ()
            },
            Deref @ Class::PARTIAL_BUILTIN { .. } => (),
            Deref @ Class::INSTANCED_CLASS { .. } if (NFInstNode::InstNode::isBaseClass(&clsNode)) => {
                let __pa0 = ::match_deref::match_deref! { match &(NFInstNode::InstNode::nodeType(&clsNode)?) {
                    Deref @ NFInstNode::InstNodeType::BASE_CLASS { definition: __pa0, .. } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                ext_def = metamodelica::Own::own(__pa0);
                Error::addSourceMessage(&(Error::EXTENDS_LOOP.clone()), list![SCodeUtil::getElementName(&ext_def)?], &(NFInstNode::InstNode::info(&clsNode)))?;
                return Err("fail")
            },
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClassTree.ClassTree.instantiate")); __mm_s.push_str(&*literal!(" got invalid class")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFClassTree.mo")))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        NFInstNode::InstNode::updateClass(cls, clsNode.clone())?;
        Ok((clsNode, instance, classCount, compCount))
    }

    pub(crate) fn fromRecordConstructor(
        mut fields: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        mut out: metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<metamodelica::Ref<ClassTree>> {
        let mut tree: metamodelica::Ref<ClassTree> = EMPTY().clone();
        let mut ltree: metamodelica::Ref<LookupTree::Tree> = LookupTree::new();
        let mut i: i32 = 1;
        let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        comps = metamodelica::arrayCreate(
            ((fields).len() as i32) + 1,
            crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        );
        for mut ci in &**fields {
            {
                let __cell0 = ci.clone();
                let __idx0 = i;
                let _ =
                    unsafe { metamodelica::Dangerous::arrayInitSlotChecked(comps.clone().clone(), __idx0, __cell0) }?;
            }
            ltree = addLocalElement(
                &(NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&ci))?),
                &(metamodelica::Ref::new(LookupTree::Entry::Entry::COMPONENT { index: i })),
                &tree,
                ltree,
            )?;
            i = i + 1;
        }
        {
            let __cell1 = out.clone();
            let __idx1 = i;
            let _ = unsafe { metamodelica::Dangerous::arrayInitSlotChecked(comps.clone().clone(), __idx1, __cell1) }?;
        }
        ltree = addLocalElement(
            &(NFInstNode::InstNode::name(&out)?),
            &(metamodelica::Ref::new(LookupTree::Entry::Entry::COMPONENT { index: i })),
            &tree,
            ltree,
        )?;
        tree = metamodelica::Ref::new(ClassTree::FLAT_TREE {
            tree: ltree,
            classes: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
            components: comps.clone(),
            imports: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
            duplicates: DuplicateTree::new(),
        });
        Ok(tree)
    }

    pub(crate) fn clone(mut tree: metamodelica::Ref<ClassTree>) -> Result<metamodelica::Ref<ClassTree>> {
        let mut outTree: metamodelica::Ref<ClassTree>;
        outTree = (match &*tree {
            EXPANDED_TREE {
                duplicates: __tree_duplicates,
                tree: __tree_tree,
                ..
            } => {
                let mut clss: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
                clss = metamodelica::arrayFromVec(
                    var_field!((*tree).classes, ClassTree::EXPANDED_TREE)
                        .clone()
                        .borrow()
                        .clone(),
                );
                clss = Array::mapNoCopy(clss.clone(), &NFInstNode::InstNode::clone)?;
                metamodelica::Ref::new(ClassTree::EXPANDED_TREE {
                    tree: __tree_tree.clone(),
                    classes: clss.clone(),
                    components: var_field!((*tree).components, ClassTree::EXPANDED_TREE).clone(),
                    exts: var_field!((*tree).exts, ClassTree::EXPANDED_TREE).clone(),
                    imports: var_field!((*tree).imports, ClassTree::EXPANDED_TREE).clone(),
                    duplicates: __tree_duplicates.clone(),
                })
            }
            _ => tree,
        });
        Ok(outTree)
    }

    pub(crate) fn mapRedeclareChains(
        mut tree: &metamodelica::Ref<ClassTree>,
        mut func: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>,
                ) -> Result<()>
                + 'static,
        >,
    ) -> Result<()> {
        pub type FuncT = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>,
                ) -> Result<()>
                + 'static,
        >;

        let () = (match &**tree {
            INSTANTIATED_TREE {
                duplicates: __tree_duplicates,
                ..
            } if (!(DuplicateTree::isEmpty(metamodelica::AsArg::as_arg(&__tree_duplicates)))) => {
                DuplicateTree::map(
                    __tree_duplicates.clone(),
                    &({
                        let __pe_b2: Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>,
                                ) -> Result<()>
                                + 'static,
                        > = func.clone();
                        let __pe_b3 = tree.clone();
                        move |__pe_a0, __pe_a1| mapRedeclareChain(&__pe_a0, __pe_a1, &*__pe_b2, &__pe_b3)
                    }),
                )?;
                ()
            }
            _ => (),
        });
        Ok(())
    }

    pub(crate) fn replaceDuplicates(mut tree: metamodelica::Ref<ClassTree>) -> Result<metamodelica::Ref<ClassTree>> {
        let mut tree: metamodelica::Ref<ClassTree> = tree;
        let mut duplicates: metamodelica::Ref<DuplicateTree::Tree>;
        let () = (match &*tree.clone() {
            INSTANTIATED_TREE {
                duplicates: __tree_duplicates,
                ..
            } if (!(DuplicateTree::isEmpty(metamodelica::AsArg::as_arg(&__tree_duplicates)))) => {
                (duplicates, tree) = DuplicateTree::mapFold(
                    __tree_duplicates.clone(),
                    &move |__a0: ArcStr,
                           __a1: metamodelica::Ref<DuplicateTree::Entry>,
                           __a2: metamodelica::Ref<ClassTree>| {
                        replaceDuplicates2(&__a0, __a1, __a2)
                    },
                    tree,
                )?;
                assign_variant_field!(tree => ClassTree::INSTANTIATED_TREE; duplicates = duplicates);
                ()
            }
            _ => (),
        });
        Ok(tree)
    }

    pub(crate) fn appendComponentsToInstTree(
        mut components: metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>,
        mut tree: metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<ClassTree>> {
        let mut tree: metamodelica::Ref<ClassTree> = tree;
        if (components).is_empty() {
            return Ok(tree);
        } else {
            let () = (match &*tree {
                INSTANTIATED_TREE { .. } => {
                    let mut comp_idx: i32;
                    let mut local_comps: metamodelica::List<i32>;
                    comp_idx =
                        metamodelica::arrayLength(var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone());
                    assign_variant_field!(tree => ClassTree::INSTANTIATED_TREE; components = Array::appendList(var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone(), components.clone())?);
                    local_comps = var_field!((*tree).localComponents, ClassTree::INSTANTIATED_TREE).clone();
                    for mut i in comp_idx + 1..=comp_idx + ((components).len() as i32) {
                        local_comps = metamodelica::cons(i, local_comps);
                    }
                    assign_variant_field!(tree => ClassTree::INSTANTIATED_TREE; localComponents = local_comps);
                    ()
                }
                _ => {
                    Error::terminate(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NFClassTree.ClassTree.appendComponentsToInstTree"));
                            __mm_s.push_str(&*literal!(" failed for non-instantiated tree."));
                            ArcStr::from(__mm_s)
                        },
                        &(metamodelica::sourceInfo!("NFFrontEnd/NFClassTree.mo")),
                    )?;
                    return Err("fail");
                }
            });
        }
        Ok(tree)
    }

    pub fn appendComponentsToFlatTree(
        mut components: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        mut tree: metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<ClassTree>> {
        let mut tree: metamodelica::Ref<ClassTree> = tree;
        if (components).is_empty() {
            return Ok(tree);
        } else {
            let () = (match &*tree {
                FLAT_TREE { .. } => {
                    assign_variant_field!(tree => ClassTree::FLAT_TREE; components = Array::appendList(var_field!((*tree).components, ClassTree::FLAT_TREE).clone(), components)?);
                    ()
                }
                _ => {
                    Error::terminate(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NFClassTree.ClassTree.appendComponentsToFlatTree"));
                            __mm_s.push_str(&*literal!(" failed for non-flat tree."));
                            ArcStr::from(__mm_s)
                        },
                        &(metamodelica::sourceInfo!("NFFrontEnd/NFClassTree.mo")),
                    )?;
                    return Err("fail");
                }
            });
        }
        Ok(tree)
    }

    pub(crate) fn flatten(mut tree: metamodelica::Ref<ClassTree>) -> Result<metamodelica::Ref<ClassTree>> {
        let mut tree: metamodelica::Ref<ClassTree> = tree;
        tree = (match &*tree {
            INSTANTIATED_TREE {
                duplicates: __tree_duplicates,
                tree: __tree_tree,
                ..
            } => {
                let mut clss: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
                let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
                let mut comp_offsets: metamodelica::Array<i32>;
                let mut clsc: i32;
                let mut compc: i32;
                let mut dup_comp: metamodelica::List<i32>;
                let mut ltree: metamodelica::Ref<LookupTree::Tree>;
                (_, dup_comp) = enumerateDuplicates(metamodelica::AsArg::as_arg(&__tree_duplicates))?;
                clsc = metamodelica::arrayLength(var_field!((*tree).classes, ClassTree::INSTANTIATED_TREE).clone());
                compc = metamodelica::arrayLength(var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone())
                    - ((dup_comp).len() as i32);
                clss = metamodelica::arrayCreate(clsc, crate::NFInstNode::InstNode::interned_EMPTY_NODE());
                comps = metamodelica::arrayCreate(compc, crate::NFInstNode::InstNode::interned_EMPTY_NODE());
                flattenElements(
                    var_field!((*tree).classes, ClassTree::INSTANTIATED_TREE).clone(),
                    clss.clone(),
                );
                if (dup_comp).is_empty() {
                    flattenElements(
                        var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone(),
                        comps.clone(),
                    );
                    ltree = __tree_tree.clone();
                } else {
                    comp_offsets = createFlatOffsets(
                        metamodelica::arrayLength(var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone()),
                        &dup_comp,
                    )?;
                    flattenElementsWithOffset(
                        var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone(),
                        comps.clone(),
                        comp_offsets.clone(),
                    );
                    ltree = flattenLookupTree(__tree_tree.clone(), comp_offsets.clone())?;
                }
                metamodelica::Ref::new(ClassTree::FLAT_TREE {
                    tree: ltree,
                    classes: clss.clone(),
                    components: comps.clone(),
                    imports: var_field!((*tree).imports, ClassTree::INSTANTIATED_TREE).clone(),
                    duplicates: __tree_duplicates.clone(),
                })
            }
            _ => tree,
        });
        Ok(tree)
    }

    pub(crate) fn flattenElements(
        mut elements: metamodelica::Array<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>,
        mut flatElements: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
    ) -> () {
        for mut i in 1..=metamodelica::arrayLength(elements.clone()) {
            metamodelica::Dangerous::arrayUpdateNoBoundsChecking(
                flatElements.clone(),
                i,
                Mutable::access(metamodelica::Dangerous::arrayGetNoBoundsChecking(elements.clone(), i)),
            );
        }
        ()
    }

    pub(crate) fn flattenElementsWithOffset(
        mut elements: metamodelica::Array<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>,
        mut flatElements: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
        mut offsets: metamodelica::Array<i32>,
    ) -> () {
        let mut offset: i32;
        for mut i in 1..=metamodelica::arrayLength(elements.clone()) {
            offset = metamodelica::Dangerous::arrayGetNoBoundsChecking(offsets.clone(), i);
            if offset >= 0 {
                metamodelica::Dangerous::arrayUpdateNoBoundsChecking(
                    flatElements.clone(),
                    i - offset,
                    Mutable::access(metamodelica::Dangerous::arrayGetNoBoundsChecking(elements.clone(), i)),
                );
            }
        }
        ()
    }

    pub(crate) fn createFlatOffsets(
        mut elementCount: i32,
        mut duplicates: &metamodelica::List<i32>,
    ) -> Result<metamodelica::Array<i32>> {
        let mut offsets: metamodelica::Array<i32>;
        let mut offset: i32 = 0;
        let mut dup: i32;
        let mut rest_dups: metamodelica::List<i32>;
        offsets = metamodelica::arrayCreate(elementCount, 0);
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*duplicates)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        dup = metamodelica::Own::own(__pa0);
        rest_dups = metamodelica::Own::own(__pa1);
        for mut i in 1..=elementCount {
            if i == dup {
                if (rest_dups).is_empty() {
                    dup = 0;
                } else {
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_dups) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    dup = metamodelica::Own::own(__pa2);
                    rest_dups = metamodelica::Own::own(__pa3);
                }
                offset = offset + 1;
                unsafe { metamodelica::Dangerous::arrayInitSlot(offsets.clone(), i, -1) };
            } else {
                unsafe { metamodelica::Dangerous::arrayInitSlot(offsets.clone(), i, offset) };
            }
        }
        Ok(offsets)
    }

    pub(crate) fn flattenLookupTree(
        mut tree: metamodelica::Ref<LookupTree::Tree>,
        mut offsets: metamodelica::Array<i32>,
    ) -> Result<metamodelica::Ref<LookupTree::Tree>> {
        let mut tree: metamodelica::Ref<LookupTree::Tree> = tree;
        tree = LookupTree::map(
            tree,
            &({
                let __pe_b2 = offsets.clone();
                move |__pe_a0, __pe_a1| Ok(flattenLookupTree2(&__pe_a0, __pe_a1, __pe_b2.clone()))
            }),
        )?;
        Ok(tree)
    }

    pub(crate) fn flattenLookupTree2(
        mut key: &ArcStr,
        mut entry: metamodelica::Ref<LookupTree::Entry::Entry>,
        mut offsets: metamodelica::Array<i32>,
    ) -> metamodelica::Ref<LookupTree::Entry::Entry> {
        let mut outEntry: metamodelica::Ref<LookupTree::Entry::Entry>;
        outEntry = (match &*entry {
            LookupTree::Entry::COMPONENT { index: __entry_index } => {
                metamodelica::Ref::new(LookupTree::Entry::Entry::COMPONENT {
                    index: __entry_index.clone()
                        - metamodelica::Dangerous::arrayGetNoBoundsChecking(offsets.clone(), __entry_index.clone()),
                })
            }
            _ => entry,
        });
        outEntry
    }

    pub(crate) fn lookupElement(
        mut name: ArcStr,
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<(metamodelica::Ref<InstNode::InstNode>, bool)> {
        let mut element: metamodelica::Ref<InstNode::InstNode>;
        let mut isImport: bool;
        let mut entry: metamodelica::Ref<LookupTree::Entry::Entry>;
        entry = LookupTree::get(&(lookupTree(tree)?), name)?;
        (element, isImport) = resolveEntry(&entry, tree)?;
        Ok((element, isImport))
    }

    pub(crate) fn lookupElementPtr(
        mut name: ArcStr,
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>> {
        let mut element: Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>;
        let mut entry: metamodelica::Ref<LookupTree::Entry::Entry>;
        entry = LookupTree::get(&(lookupTree(tree)?), name)?;
        element = resolveEntryPtr(&entry, tree)?;
        Ok(element)
    }

    pub(crate) fn lookupElementsPtr(
        mut name: ArcStr,
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>> {
        let mut elements: metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>;
        let mut dup_entry: metamodelica::Ref<DuplicateTree::Entry>;
        match '__try0: {
            dup_entry = unwrap_break_err!(DuplicateTree::get(&(unwrap_break_err!(getDuplicates(tree), '__try0)), name.clone()), '__try0);
            elements = unwrap_break_err!(resolveDuplicateEntriesPtr(&dup_entry, tree, metamodelica::nil()), '__try0);
            Ok::<_, &'static str>((elements.clone(),))
        } {
            Ok((__try0_o0,)) => {
                elements = __try0_o0;
            }
            Err(_) => {
                elements = list![lookupElementPtr(name.clone(), tree)?];
            }
        }
        Ok(elements)
    }

    pub(crate) fn lookupComponentIndex(mut name: ArcStr, mut tree: &metamodelica::Ref<ClassTree>) -> Result<i32> {
        let mut index: i32;
        let __pa0 = ::match_deref::match_deref! { match &(LookupTree::get(&(lookupTree(tree)?), name)?) {
            Deref @ LookupTree::Entry::COMPONENT { index: __pa0 } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        index = metamodelica::Own::own(__pa0);
        Ok(index)
    }

    pub(crate) fn nthComponent(
        mut index: i32,
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<InstNode::InstNode>> {
        let mut component: metamodelica::Ref<InstNode::InstNode>;
        component = (match &**tree {
            PARTIAL_TREE { .. } => {
                metamodelica::arrayGet(var_field!((**tree).components, ClassTree::PARTIAL_TREE).clone(), index)?
            }
            EXPANDED_TREE { .. } => {
                metamodelica::arrayGet(var_field!((**tree).components, ClassTree::EXPANDED_TREE).clone(), index)?
            }
            INSTANTIATED_TREE { .. } => Mutable::access(metamodelica::arrayGet(
                var_field!((**tree).components, ClassTree::INSTANTIATED_TREE).clone(),
                index,
            )?),
            FLAT_TREE { .. } => {
                metamodelica::arrayGet(var_field!((**tree).components, ClassTree::FLAT_TREE).clone(), index)?
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(component)
    }

    pub(crate) fn mapClasses(
        mut tree: &metamodelica::Ref<ClassTree>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<metamodelica::Ref<InstNode::InstNode>>,
    ) -> Result<()> {
        pub type FuncT = std::sync::Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<metamodelica::Ref<InstNode::InstNode>>
                + 'static,
        >;

        let mut clss: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>> = getClasses(tree)?;
        for mut i in 1..=metamodelica::arrayLength(clss.clone()) {
            metamodelica::Dangerous::arrayUpdateNoBoundsChecking(
                clss.clone(),
                i,
                func(metamodelica::Dangerous::arrayGetNoBoundsChecking(clss.clone(), i))?,
            );
        }
        Ok(())
    }

    pub(crate) fn foldClasses<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut tree: &metamodelica::Ref<ClassTree>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>, ArgT) -> Result<ArgT>,
        mut arg: ArgT,
    ) -> Result<ArgT> {
        pub type FuncT<ArgT: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>, ArgT) -> Result<ArgT> + 'static>;

        let mut arg: ArgT = arg;
        let mut clss: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>> = getClasses(tree)?;
        let __range0 = clss.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut cls in __range0 {
            arg = func(cls, arg)?;
        }
        Ok(arg)
    }

    pub(crate) fn applyExtends(
        mut tree: &metamodelica::Ref<ClassTree>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<()>,
    ) -> Result<()> {
        pub type FuncT =
            std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<()> + 'static>;

        let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>> = getExtends(tree);
        let __range0 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut ext in __range0 {
            func(ext)?;
        }
        Ok(())
    }

    pub(crate) fn mapExtends(
        mut tree: &metamodelica::Ref<ClassTree>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<metamodelica::Ref<InstNode::InstNode>>,
    ) -> Result<()> {
        pub type FuncT = std::sync::Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<metamodelica::Ref<InstNode::InstNode>>
                + 'static,
        >;

        let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>> = getExtends(tree);
        for mut i in 1..=metamodelica::arrayLength(exts.clone()) {
            metamodelica::Dangerous::arrayUpdateNoBoundsChecking(
                exts.clone(),
                i,
                func(metamodelica::Dangerous::arrayGetNoBoundsChecking(exts.clone(), i))?,
            );
        }
        Ok(())
    }

    pub(crate) fn foldExtends<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut tree: &metamodelica::Ref<ClassTree>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>, ArgT) -> Result<ArgT>,
        mut arg: ArgT,
    ) -> Result<ArgT> {
        pub type FuncT<ArgT: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>, ArgT) -> Result<ArgT> + 'static>;

        let mut arg: ArgT = arg;
        let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>> = getExtends(tree);
        let __range0 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut ext in __range0 {
            arg = func(ext, arg)?;
        }
        Ok(arg)
    }

    pub(crate) fn mapFoldExtends<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut tree: &metamodelica::Ref<ClassTree>,
        mut func: &dyn ::std::ops::Fn(
            metamodelica::Ref<InstNode::InstNode>,
            ArgT,
        ) -> Result<(metamodelica::Ref<InstNode::InstNode>, ArgT)>,
        mut arg: ArgT,
    ) -> Result<ArgT> {
        pub type FuncT<ArgT: Clone + 'static> = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<InstNode::InstNode>,
                    ArgT,
                ) -> Result<(metamodelica::Ref<InstNode::InstNode>, ArgT)>
                + 'static,
        >;

        let mut arg: ArgT = arg;
        let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>> = getExtends(tree);
        let mut ext: metamodelica::Ref<InstNode::InstNode>;
        for mut i in 1..=metamodelica::arrayLength(exts.clone()) {
            (ext, arg) = func(metamodelica::Dangerous::arrayGetNoBoundsChecking(exts.clone(), i), arg)?;
            metamodelica::Dangerous::arrayUpdateNoBoundsChecking(exts.clone(), i, ext);
        }
        Ok(arg)
    }

    pub(crate) fn applyLocalComponents(
        mut tree: &metamodelica::Ref<ClassTree>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<()>,
    ) -> Result<()> {
        pub type FuncT =
            std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<()> + 'static>;

        let () = (match &**tree {
            INSTANTIATED_TREE {
                localComponents: __tree_localComponents,
                ..
            } => {
                for mut i in &*__tree_localComponents.clone() {
                    func(Mutable::access(metamodelica::Dangerous::arrayGetNoBoundsChecking(
                        var_field!((**tree).components, ClassTree::INSTANTIATED_TREE).clone(),
                        i.clone(),
                    )))?;
                }
                ()
            }
            PARTIAL_TREE { .. } => {
                let __range0 = var_field!((**tree).components, ClassTree::PARTIAL_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range0 {
                    func(c)?;
                }
                ()
            }
            EXPANDED_TREE { .. } => {
                let __range0 = var_field!((**tree).components, ClassTree::EXPANDED_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range0 {
                    func(c)?;
                }
                ()
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(())
    }

    pub(crate) fn applyComponents(
        mut tree: &metamodelica::Ref<ClassTree>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<()>,
    ) -> Result<()> {
        pub type FuncT =
            std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<()> + 'static>;

        let () = (match &**tree {
            PARTIAL_TREE { .. } => {
                let __range0 = var_field!((**tree).components, ClassTree::PARTIAL_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range0 {
                    func(c)?;
                }
                ()
            }
            EXPANDED_TREE { .. } => {
                let __range0 = var_field!((**tree).components, ClassTree::EXPANDED_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range0 {
                    func(c)?;
                }
                ()
            }
            INSTANTIATED_TREE { .. } => {
                let __range0 = var_field!((**tree).components, ClassTree::INSTANTIATED_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range0 {
                    func(Mutable::access(c))?;
                }
                ()
            }
            FLAT_TREE { .. } => {
                let __range0 = var_field!((**tree).components, ClassTree::FLAT_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range0 {
                    func(c)?;
                }
                ()
            }
            _ => (),
        });
        Ok(())
    }

    pub(crate) fn foldComponents<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut tree: &metamodelica::Ref<ClassTree>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>, ArgT) -> Result<ArgT>,
        mut arg: ArgT,
    ) -> Result<ArgT> {
        pub type FuncT<ArgT: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>, ArgT) -> Result<ArgT> + 'static>;

        let mut arg: ArgT = arg;
        let () = (match &**tree {
            PARTIAL_TREE { .. } => {
                let __range0 = var_field!((**tree).components, ClassTree::PARTIAL_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range0 {
                    arg = func(c, arg)?;
                }
                ()
            }
            EXPANDED_TREE { .. } => {
                let __range0 = var_field!((**tree).components, ClassTree::EXPANDED_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range0 {
                    arg = func(c, arg)?;
                }
                ()
            }
            INSTANTIATED_TREE { .. } => {
                let __range0 = var_field!((**tree).components, ClassTree::INSTANTIATED_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range0 {
                    arg = func(Mutable::access(c), arg)?;
                }
                ()
            }
            FLAT_TREE { .. } => {
                let __range0 = var_field!((**tree).components, ClassTree::FLAT_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range0 {
                    arg = func(c, arg)?;
                }
                ()
            }
            _ => (),
        });
        Ok(arg)
    }

    pub(crate) fn findComponent(
        mut tree: &metamodelica::Ref<ClassTree>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<bool>,
    ) -> Result<Option<metamodelica::Ref<InstNode::InstNode>>> {
        pub type FuncT =
            std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<bool> + 'static>;

        let mut component: Option<metamodelica::Ref<InstNode::InstNode>> = None;
        let () = (match &**tree {
            PARTIAL_TREE { .. } => {
                let __range0 = var_field!((**tree).components, ClassTree::PARTIAL_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range0 {
                    if func(c.clone())? {
                        component = Some(c);
                        break;
                    }
                }
                ()
            }
            EXPANDED_TREE { .. } => {
                let __range0 = var_field!((**tree).components, ClassTree::EXPANDED_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range0 {
                    if func(c.clone())? {
                        component = Some(c);
                        break;
                    }
                }
                ()
            }
            INSTANTIATED_TREE { .. } => {
                let __range0 = var_field!((**tree).components, ClassTree::INSTANTIATED_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range0 {
                    if func(Mutable::access(c.clone()))? {
                        component = Some(Mutable::access(c));
                        break;
                    }
                }
                ()
            }
            FLAT_TREE { .. } => {
                let __range0 = var_field!((**tree).components, ClassTree::FLAT_TREE)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut c in __range0 {
                    if func(c.clone())? {
                        component = Some(c);
                        break;
                    }
                }
                ()
            }
            _ => (),
        });
        Ok(component)
    }

    pub(crate) fn classCount(mut tree: &metamodelica::Ref<ClassTree>) -> i32 {
        let mut count: i32;
        count = (match &**tree {
            PARTIAL_TREE { .. } => {
                metamodelica::arrayLength(var_field!((**tree).classes, ClassTree::PARTIAL_TREE).clone())
            }
            EXPANDED_TREE { .. } => {
                metamodelica::arrayLength(var_field!((**tree).classes, ClassTree::EXPANDED_TREE).clone())
            }
            INSTANTIATED_TREE { .. } => {
                metamodelica::arrayLength(var_field!((**tree).classes, ClassTree::INSTANTIATED_TREE).clone())
            }
            FLAT_TREE { .. } => metamodelica::arrayLength(var_field!((**tree).classes, ClassTree::FLAT_TREE).clone()),
            _ => 0,
        });
        count
    }

    pub(crate) fn componentCount(mut tree: &metamodelica::Ref<ClassTree>) -> i32 {
        let mut count: i32;
        count = (match &**tree {
            PARTIAL_TREE { .. } => {
                metamodelica::arrayLength(var_field!((**tree).components, ClassTree::PARTIAL_TREE).clone())
                    - metamodelica::arrayLength(var_field!((**tree).exts, ClassTree::PARTIAL_TREE).clone())
            }
            EXPANDED_TREE { .. } => {
                metamodelica::arrayLength(var_field!((**tree).components, ClassTree::EXPANDED_TREE).clone())
                    - metamodelica::arrayLength(var_field!((**tree).exts, ClassTree::EXPANDED_TREE).clone())
            }
            INSTANTIATED_TREE { .. } => {
                metamodelica::arrayLength(var_field!((**tree).components, ClassTree::INSTANTIATED_TREE).clone())
            }
            FLAT_TREE { .. } => {
                metamodelica::arrayLength(var_field!((**tree).components, ClassTree::FLAT_TREE).clone())
            }
            _ => 0,
        });
        count
    }

    pub(crate) fn extendsCount(mut tree: &metamodelica::Ref<ClassTree>) -> i32 {
        let mut count: i32 = metamodelica::arrayLength(getExtends(tree));
        count
    }

    pub(crate) fn recursiveElementCount(mut tree: &metamodelica::Ref<ClassTree>) -> Result<i32> {
        let mut count: i32;
        count = classCount(tree) + componentCount(tree);
        let __range0 = getExtends(tree).borrow().iter().cloned().collect::<Vec<_>>();
        for mut ext in __range0 {
            count = count + recursiveElementCount(&(Class::classTree(NFInstNode::InstNode::getClass(ext)?)?))?;
        }
        Ok(count)
    }

    pub(crate) fn checkDuplicates(mut tree: metamodelica::Ref<ClassTree>) -> Result<()> {
        let () = (match &*tree.clone() {
            INSTANTIATED_TREE {
                duplicates: __tree_duplicates,
                ..
            } if (!(DuplicateTree::isEmpty(metamodelica::AsArg::as_arg(&__tree_duplicates)))) => {
                DuplicateTree::fold(
                    metamodelica::AsArg::as_arg(&__tree_duplicates),
                    &move |__a0: ArcStr,
                           __a1: metamodelica::Ref<DuplicateTree::Entry>,
                           __a2: metamodelica::Ref<ClassTree>| {
                        checkDuplicates2(&__a0, &__a1, __a2)
                    },
                    tree,
                )?;
                ()
            }
            _ => (),
        });
        Ok(())
    }

    pub(crate) fn checkDuplicates2(
        mut name: &ArcStr,
        mut entry: &metamodelica::Ref<DuplicateTree::Entry>,
        mut tree: metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<ClassTree>> {
        let mut tree: metamodelica::Ref<ClassTree> = tree;
        let mut kept: metamodelica::Ref<InstNode::InstNode>;
        let mut dup: metamodelica::Ref<InstNode::InstNode>;
        if (entry.node).is_none() {
            return Ok(tree);
        }
        let __pa0 = ::match_deref::match_deref! { match &(entry.node.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        kept = metamodelica::Own::own(__pa0);
        let () = (match entry.ty.clone() {
            DuplicateTree::EntryType::REDECLARE => (),
            _ => {
                for mut c in &*entry.children.clone() {
                    let __pa0 = ::match_deref::match_deref! { match &(c.node.clone()) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    dup = metamodelica::Own::own(__pa0);
                    if !(NFInstNode::InstNode::isEmpty(&dup)) {
                        NFInstNode::InstNode::checkIdentical(kept.clone(), dup)?;
                    }
                }
                ()
            }
        });
        Ok(tree)
    }

    pub(crate) fn isIdentical(
        mut tree1: &metamodelica::Ref<ClassTree>,
        mut tree2: &metamodelica::Ref<ClassTree>,
    ) -> bool {
        let mut identical: bool;
        identical = true;
        identical
    }

    pub(crate) fn getRedeclaredNode(
        mut name: ArcStr,
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<InstNode::InstNode>> {
        let mut node: metamodelica::Ref<InstNode::InstNode> = metamodelica::Ref::new(InstNode::EMPTY_NODE);
        let mut entry: metamodelica::Ref<DuplicateTree::Entry>;
        if '__try0: {
            entry = unwrap_break_err!(DuplicateTree::get(&(unwrap_break_err!(getDuplicates(tree), '__try0)), name.clone()), '__try0);
            entry = unwrap_break_err!((entry.children).head().cloned(), '__try0);
            if (entry.node).is_some() {
                let __pa1 = ::match_deref::match_deref! { match &(entry.node.clone()) {
                    Some(__pa1) => __pa1.clone(),
                    _ => break '__try0 Err::<_, _>("pattern mismatch"),
                } };
                node = metamodelica::Own::own(__pa1);
            } else {
                (node, _) = unwrap_break_err!(resolveEntry(&entry.entry, tree), '__try0);
            }
            Ok::<(), &'static str>(())
        }.is_err() {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClassTree.ClassTree.getRedeclaredNode")); __mm_s.push_str(&*literal!(" failed on ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFClassTree.mo")))?;
        }
        Ok(node)
    }

    pub(crate) fn setClassExtends(
        mut extNode: metamodelica::Ref<InstNode::InstNode>,
        mut tree: metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<ClassTree>> {
        let mut tree: metamodelica::Ref<ClassTree> = tree;
        metamodelica::arrayUpdate(getExtends(&tree), 1, extNode)?;
        Ok(tree)
    }

    pub(crate) fn enumerateComponents(
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>> {
        let mut components: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
        let mut ltree: metamodelica::Ref<LookupTree::Tree>;
        let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*tree)) {
            Deref @ FLAT_TREE { tree: __pa0, components: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ltree = metamodelica::Own::own(__pa0);
        comps = metamodelica::Own::own(__pa1);
        components = LookupTree::fold(
            &ltree,
            &({
                let __pe_b2 = comps.clone();
                move |__pe_a0, __pe_a1, __pe_a3| enumerateComponents2(&__pe_a0, &__pe_a1, __pe_b2.clone(), __pe_a3)
            }),
            metamodelica::nil(),
        )?;
        Ok(components)
    }

    pub(crate) fn enumerateComponents2(
        mut name: &ArcStr,
        mut entry: &metamodelica::Ref<LookupTree::Entry::Entry>,
        mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
        mut components: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    ) -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>> {
        let __ab_comps = comps.borrow();
        let mut components: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = components;
        let () = (match &**entry {
            LookupTree::Entry::COMPONENT { index: __entry_index } => {
                components = metamodelica::cons(
                    (*metamodelica::index_checked(&__ab_comps, __entry_index.clone())?).clone(),
                    components,
                );
                ()
            }
            _ => (),
        });
        Ok(components)
    }

    pub fn getClasses(
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>> {
        let mut clss: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        clss = (match &**tree {
            PARTIAL_TREE { .. } => var_field!((**tree).classes, ClassTree::PARTIAL_TREE).clone(),
            EXPANDED_TREE { .. } => var_field!((**tree).classes, ClassTree::EXPANDED_TREE).clone(),
            FLAT_TREE { .. } => var_field!((**tree).classes, ClassTree::FLAT_TREE).clone(),
            _ => return Err("match: no arm matched"),
        });
        Ok(clss)
    }

    pub fn getExtends(
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> metamodelica::Array<metamodelica::Ref<InstNode::InstNode>> {
        let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        exts = (match &**tree {
            PARTIAL_TREE { .. } => var_field!((**tree).exts, ClassTree::PARTIAL_TREE).clone(),
            EXPANDED_TREE { .. } => var_field!((**tree).exts, ClassTree::EXPANDED_TREE).clone(),
            INSTANTIATED_TREE { .. } => var_field!((**tree).exts, ClassTree::INSTANTIATED_TREE).clone(),
            _ => metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
        });
        exts
    }

    pub fn getComponents(
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>> {
        let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        comps = (match &**tree {
            PARTIAL_TREE { .. } => var_field!((**tree).components, ClassTree::PARTIAL_TREE).clone(),
            EXPANDED_TREE { .. } => var_field!((**tree).components, ClassTree::EXPANDED_TREE).clone(),
            FLAT_TREE { .. } => var_field!((**tree).components, ClassTree::FLAT_TREE).clone(),
            _ => return Err("match: no arm matched"),
        });
        Ok(comps)
    }

    pub fn getImports(
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Array<metamodelica::Ref<Import::NFImport>>> {
        let mut imps: metamodelica::Array<metamodelica::Ref<Import::NFImport>>;
        imps = (match &**tree {
            PARTIAL_TREE { .. } => var_field!((**tree).imports, ClassTree::PARTIAL_TREE).clone(),
            EXPANDED_TREE { .. } => var_field!((**tree).imports, ClassTree::EXPANDED_TREE).clone(),
            INSTANTIATED_TREE { .. } => var_field!((**tree).imports, ClassTree::INSTANTIATED_TREE).clone(),
            FLAT_TREE { .. } => var_field!((**tree).imports, ClassTree::FLAT_TREE).clone(),
            _ => return Err("match: no arm matched"),
        });
        Ok(imps)
    }

    pub(crate) fn isEmptyTree(mut tree: &metamodelica::Ref<ClassTree>) -> bool {
        let mut isEmpty: bool;
        isEmpty = (match &**tree {
            EMPTY_TREE { .. } => true,
            _ => false,
        });
        isEmpty
    }

    pub(crate) fn appendClasses(
        mut clsNodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        mut tree: metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<ClassTree>> {
        let mut tree: metamodelica::Ref<ClassTree> = tree;
        let mut classes: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut ltree: metamodelica::Ref<LookupTree::Tree>;
        let () = (match &*tree {
            PARTIAL_TREE { tree: __tree_tree, .. } => {
                (ltree, classes) = appendClasses2(
                    clsNodes,
                    __tree_tree.clone(),
                    var_field!((*tree).classes, ClassTree::PARTIAL_TREE).clone(),
                )?;
                assign_variant_field!(tree => ClassTree::PARTIAL_TREE;
                    tree = ltree,
                    classes = classes.clone()
                );
                ()
            }
            EXPANDED_TREE { tree: __tree_tree, .. } => {
                (ltree, classes) = appendClasses2(
                    clsNodes,
                    __tree_tree.clone(),
                    var_field!((*tree).classes, ClassTree::EXPANDED_TREE).clone(),
                )?;
                assign_variant_field!(tree => ClassTree::EXPANDED_TREE;
                    tree = ltree,
                    classes = classes.clone()
                );
                ()
            }
            FLAT_TREE { tree: __tree_tree, .. } => {
                (ltree, classes) = appendClasses2(
                    clsNodes,
                    __tree_tree.clone(),
                    var_field!((*tree).classes, ClassTree::FLAT_TREE).clone(),
                )?;
                assign_variant_field!(tree => ClassTree::FLAT_TREE;
                    tree = ltree,
                    classes = classes.clone()
                );
                ()
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(tree)
    }

    pub(crate) fn appendClasses2(
        mut clsNodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
        mut tree: metamodelica::Ref<LookupTree::Tree>,
        mut classes: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
    ) -> Result<(
        metamodelica::Ref<LookupTree::Tree>,
        metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
    )> {
        let mut tree: metamodelica::Ref<LookupTree::Tree> = tree;
        let mut classes: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>> = classes;
        let mut index: i32;
        index = metamodelica::arrayLength(classes.clone());
        classes = Array::appendList(classes.clone(), clsNodes.clone())?;
        for mut c in &*clsNodes {
            index = index + 1;
            tree = LookupTree::add(
                tree,
                &(NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&c))?),
                &(metamodelica::Ref::new(LookupTree::Entry::Entry::CLASS { index: index })),
                &*(std::sync::Arc::new(LookupTree::addConflictDefault)
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
        }
        Ok((tree, classes))
    }

    pub(crate) fn replaceClass(
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut tree: metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<ClassTree>> {
        let mut tree: metamodelica::Ref<ClassTree> = tree;
        let mut index: i32;
        let __pa0 = ::match_deref::match_deref! { match &(LookupTree::get(&(lookupTree(&tree)?), NFInstNode::InstNode::name(&node)?)?) {
            Deref @ LookupTree::Entry::CLASS { index: __pa0 } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        index = metamodelica::Own::own(__pa0);
        metamodelica::arrayUpdate(getClasses(&tree)?, index, node)?;
        Ok(tree)
    }

    fn instExtendsComps(
        mut extNode: metamodelica::Ref<InstNode::InstNode>,
        mut comps: metamodelica::Array<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>,
        mut index: i32,
    ) -> Result<i32> {
        let mut index: i32 = index;
        let mut ext_comps_ptrs: metamodelica::Array<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>;
        let mut ext_comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut comp_count: i32;
        let () = (match &*(Class::classTree(NFInstNode::InstNode::getClass(extNode)?)?) {
            INSTANTIATED_TREE {
                components: __esc_ext_comps_ptrs,
                ..
            } => {
                ext_comps_ptrs = (*__esc_ext_comps_ptrs).clone();
                comp_count = metamodelica::arrayLength(ext_comps_ptrs.clone());
                if comp_count > 0 {
                    Array::copyRange(ext_comps_ptrs.clone(), comps.clone(), 1, comp_count, index)?;
                    index = index + comp_count;
                }
                ()
            }
            FLAT_TREE {
                components: __esc_ext_comps,
                ..
            } => {
                ext_comps = (*__esc_ext_comps).clone();
                comp_count = metamodelica::arrayLength(ext_comps.clone());
                if comp_count > 0 {
                    for mut i in index..=index + comp_count - 1 {
                        metamodelica::arrayUpdate(
                            comps.clone(),
                            i,
                            Mutable::create(
                                ({
                                    let __elt = (*metamodelica::index_checked(&ext_comps.borrow(), i)?).clone();
                                    __elt
                                }),
                            ),
                        )?;
                    }
                    index = index + comp_count;
                }
                ()
            }
            _ => (),
        });
        Ok(index)
    }

    fn getDuplicates(mut tree: &metamodelica::Ref<ClassTree>) -> Result<metamodelica::Ref<DuplicateTree::Tree>> {
        let mut duplicates: metamodelica::Ref<DuplicateTree::Tree>;
        duplicates = (match &**tree {
            PARTIAL_TREE {
                duplicates: __tree_duplicates,
                ..
            } => __tree_duplicates.clone(),
            EXPANDED_TREE {
                duplicates: __tree_duplicates,
                ..
            } => __tree_duplicates.clone(),
            INSTANTIATED_TREE {
                duplicates: __tree_duplicates,
                ..
            } => __tree_duplicates.clone(),
            FLAT_TREE {
                duplicates: __tree_duplicates,
                ..
            } => __tree_duplicates.clone(),
            _ => return Err("match: no arm matched"),
        });
        Ok(duplicates)
    }

    fn lookupTree(mut ctree: &metamodelica::Ref<ClassTree>) -> Result<metamodelica::Ref<LookupTree::Tree>> {
        let mut ltree: metamodelica::Ref<LookupTree::Tree>;
        ltree = (match &**ctree {
            PARTIAL_TREE { tree: __ctree_tree, .. } => __ctree_tree.clone(),
            EXPANDED_TREE { tree: __ctree_tree, .. } => __ctree_tree.clone(),
            INSTANTIATED_TREE { tree: __ctree_tree, .. } => __ctree_tree.clone(),
            FLAT_TREE { tree: __ctree_tree, .. } => __ctree_tree.clone(),
            _ => return Err("match: no arm matched"),
        });
        Ok(ltree)
    }

    fn setLookupTree(
        mut ltree: metamodelica::Ref<LookupTree::Tree>,
        mut ctree: metamodelica::Ref<ClassTree>,
    ) -> metamodelica::Ref<ClassTree> {
        let mut ctree: metamodelica::Ref<ClassTree> = ctree;
        let () = (match &*ctree {
            PARTIAL_TREE { .. } => {
                assign_variant_field!(ctree => ClassTree::PARTIAL_TREE; tree = ltree);
                ()
            }
            EXPANDED_TREE { .. } => {
                assign_variant_field!(ctree => ClassTree::EXPANDED_TREE; tree = ltree);
                ()
            }
            INSTANTIATED_TREE { .. } => {
                assign_variant_field!(ctree => ClassTree::INSTANTIATED_TREE; tree = ltree);
                ()
            }
            FLAT_TREE { .. } => {
                assign_variant_field!(ctree => ClassTree::FLAT_TREE; tree = ltree);
                ()
            }
            _ => (),
        });
        ctree
    }

    fn addLocalElement(
        mut name: &ArcStr,
        mut entry: &metamodelica::Ref<LookupTree::Entry::Entry>,
        mut classTree: &metamodelica::Ref<ClassTree>,
        mut tree: metamodelica::Ref<LookupTree::Tree>,
    ) -> Result<metamodelica::Ref<LookupTree::Tree>> {
        let mut tree: metamodelica::Ref<LookupTree::Tree> = tree;
        tree = LookupTree::add(
            tree,
            name,
            entry,
            &({
                let __pe_b3 = classTree.clone();
                move |__pe_a0, __pe_a1, __pe_a2| addLocalElementConflict(__pe_a0, &__pe_a1, __pe_a2, &__pe_b3)
            }),
        )?;
        Ok(tree)
    }

    fn addLocalElementConflict(
        mut newEntry: metamodelica::Ref<LookupTree::Entry::Entry>,
        mut oldEntry: &metamodelica::Ref<LookupTree::Entry::Entry>,
        mut name: ArcStr,
        mut classTree: &metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<LookupTree::Entry::Entry>> {
        let mut entry: metamodelica::Ref<LookupTree::Entry::Entry>;
        let mut n1: metamodelica::Ref<InstNode::InstNode>;
        let mut n2: metamodelica::Ref<InstNode::InstNode>;
        entry = (match &**oldEntry {
            LookupTree::Entry::IMPORT { .. } => newEntry,
            _ => {
                n1 = findLocalConflictElement(&newEntry, classTree)?;
                n2 = findLocalConflictElement(oldEntry, classTree)?;
                Error::addMultiSourceMessage(
                    &(Error::DOUBLE_DECLARATION_OF_ELEMENTS.clone()),
                    &(list![name]),
                    &(list![NFInstNode::InstNode::info(&n2), NFInstNode::InstNode::info(&n1)]),
                )?;
                return Err("fail");
            }
        });
        Ok(entry)
    }

    fn findLocalConflictElement(
        mut entry: &metamodelica::Ref<LookupTree::Entry::Entry>,
        mut classTree: &metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<InstNode::InstNode>> {
        let mut node: metamodelica::Ref<InstNode::InstNode> = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
        node = (match &**entry {
            LookupTree::Entry::CLASS { index: __entry_index } => resolveClass(__entry_index.clone(), classTree)?,
            LookupTree::Entry::COMPONENT { index: __entry_index } => {
                let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
                let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
                let mut i: i32;
                i = 0;
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*classTree)) {
                    Deref @ PARTIAL_TREE { components: __pa0, exts: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                comps = metamodelica::Own::own(__pa0);
                exts = metamodelica::Own::own(__pa1);
                let __range2 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range2 {
                    i = (match &*c {
                        NFInstNode::InstNode::COMPONENT_NODE { .. } => i + 1,
                        NFInstNode::InstNode::REF_NODE { index: __c_index } => {
                            (_, i) = countInheritedElements(
                                ({
                                    let __elt =
                                        (*metamodelica::index_checked(&exts.borrow(), __c_index.clone())?).clone();
                                    __elt
                                }),
                                0,
                                i,
                            )?;
                            i
                        }
                        _ => return Err("match: no arm matched"),
                    });
                    if i == __entry_index.clone() {
                        node = c;
                        break;
                    }
                }
                Error::assertion(
                    i == __entry_index.clone(),
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFClassTree.ClassTree.findLocalConflictElement"));
                        __mm_s.push_str(&*literal!(" got invalid entry index"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFClassTree.mo")),
                )?;
                node
            }
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFClassTree.ClassTree.findLocalConflictElement"));
                        __mm_s.push_str(&*literal!(" got invalid entry"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFClassTree.mo")),
                )?;
                return Err("fail");
            }
        });
        Ok(node)
    }

    fn addEnumConflict(
        mut newEntry: &metamodelica::Ref<LookupTree::Entry::Entry>,
        mut oldEntry: &metamodelica::Ref<LookupTree::Entry::Entry>,
        mut name: &ArcStr,
        mut literal: &metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<metamodelica::Ref<LookupTree::Entry::Entry>> {
        let mut entry: metamodelica::Ref<LookupTree::Entry::Entry>;
        Error::addSourceMessage(
            &(Error::DOUBLE_DECLARATION_OF_ELEMENTS.clone()),
            list![NFInstNode::InstNode::name(literal)?],
            &(NFInstNode::InstNode::info(literal)),
        )?;
        return Err("fail");
        Ok(entry)
    }

    fn addImport(
        mut imp: &metamodelica::Ref<Import::NFImport>,
        mut index: i32,
        mut tree: metamodelica::Ref<LookupTree::Tree>,
        mut imports: metamodelica::Array<metamodelica::Ref<Import::NFImport>>,
    ) -> Result<metamodelica::Ref<LookupTree::Tree>> {
        let mut tree: metamodelica::Ref<LookupTree::Tree> = tree;
        tree = LookupTree::add(
            tree,
            &(Import::name(imp)?),
            &(metamodelica::Ref::new(LookupTree::Entry::Entry::IMPORT { index: index })),
            &({
                let __pe_b3 = imports.clone();
                move |__pe_a0, __pe_a1, __pe_a2| addImportConflict(__pe_a0, __pe_a1, &__pe_a2, __pe_b3.clone())
            }),
        )?;
        Ok(tree)
    }

    fn addImportConflict(
        mut newEntry: metamodelica::Ref<LookupTree::Entry::Entry>,
        mut oldEntry: metamodelica::Ref<LookupTree::Entry::Entry>,
        mut name: &ArcStr,
        mut imports: metamodelica::Array<metamodelica::Ref<Import::NFImport>>,
    ) -> Result<metamodelica::Ref<LookupTree::Entry::Entry>> {
        let mut entry: metamodelica::Ref<LookupTree::Entry::Entry>;
        entry = (::match_deref::match_deref! { match &((newEntry.clone(), oldEntry.clone())) {
            (Deref @ LookupTree::Entry::IMPORT { .. }, Deref @ LookupTree::Entry::IMPORT { .. }) => {
                let mut imp1: metamodelica::Ref<Import::NFImport>;
                let mut imp2: metamodelica::Ref<Import::NFImport>;
                imp1 = ({let __elt = (*metamodelica::index_checked(&imports.borrow(), var_field!((*newEntry).index, LookupTree::Entry::Entry::IMPORT).clone())?).clone(); __elt});
                imp2 = ({let __elt = (*metamodelica::index_checked(&imports.borrow(), var_field!((*oldEntry).index, LookupTree::Entry::Entry::IMPORT).clone())?).clone(); __elt});
                entry = (::match_deref::match_deref! { match &((imp1.clone(), imp2.clone())) {
            (Deref @ Import::UNRESOLVED_IMPORT { .. }, Deref @ Import::UNRESOLVED_IMPORT { .. }) => {
                metamodelica::arrayUpdate(imports.clone(), var_field!((*oldEntry).index, LookupTree::Entry::Entry::IMPORT).clone(), metamodelica::Ref::new(Import::NFImport::CONFLICTING_IMPORT { imp1: imp1, imp2: imp2 }))?;
                oldEntry
            },
            (Deref @ Import::RESOLVED_IMPORT { .. }, Deref @ Import::RESOLVED_IMPORT { .. }) => {
                metamodelica::arrayUpdate(imports.clone(), var_field!((*oldEntry).index, LookupTree::Entry::Entry::IMPORT).clone(), metamodelica::Ref::new(Import::NFImport::CONFLICTING_IMPORT { imp1: imp1, imp2: imp2 }))?;
                oldEntry
            },
            (Deref @ Import::UNRESOLVED_IMPORT { .. }, _) => newEntry,
            _ => oldEntry,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                entry
            },
            _ => {
                oldEntry
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(entry)
    }

    fn addDuplicate(
        mut name: &ArcStr,
        mut duplicateEntry: metamodelica::Ref<LookupTree::Entry::Entry>,
        mut keptEntry: metamodelica::Ref<LookupTree::Entry::Entry>,
        mut duplicates: Mutable::Mutable<metamodelica::Ref<DuplicateTree::Tree>>,
    ) -> Result<Mutable::Mutable<metamodelica::Ref<DuplicateTree::Tree>>> {
        let mut duplicates: Mutable::Mutable<metamodelica::Ref<DuplicateTree::Tree>> = duplicates;
        Mutable::update(
            duplicates.clone(),
            DuplicateTree::add(
                Mutable::access(duplicates.clone()),
                name,
                &(DuplicateTree::newDuplicate(keptEntry, duplicateEntry)),
                &move |__a0: metamodelica::Ref<DuplicateTree::Entry>,
                       __a1: metamodelica::Ref<DuplicateTree::Entry>,
                       __a2: ArcStr| addDuplicateConflict(&__a0, &__a1, &__a2),
            )?,
        );
        Ok(duplicates)
    }

    fn addDuplicateConflict(
        mut newEntry: &metamodelica::Ref<DuplicateTree::Entry>,
        mut oldEntry: &metamodelica::Ref<DuplicateTree::Entry>,
        mut name: &ArcStr,
    ) -> Result<metamodelica::Ref<DuplicateTree::Entry>> {
        let mut entry: metamodelica::Ref<DuplicateTree::Entry>;
        entry = metamodelica::Ref::new(DuplicateTree::Entry {
            entry: newEntry.entry.clone(),
            node: None,
            children: metamodelica::cons((newEntry.children).head().cloned()?, oldEntry.children.clone()),
            ty: DuplicateTree::EntryType::DUPLICATE.clone(),
        });
        Ok(entry)
    }

    fn resolveEntry(
        mut entry: &metamodelica::Ref<LookupTree::Entry::Entry>,
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<(metamodelica::Ref<InstNode::InstNode>, bool)> {
        let mut element: metamodelica::Ref<InstNode::InstNode>;
        let mut isImport: bool;
        (element, isImport) = (match &**entry {
            LookupTree::Entry::CLASS { index: __entry_index } => (resolveClass(__entry_index.clone(), tree)?, false),
            LookupTree::Entry::COMPONENT { index: __entry_index } => {
                (resolveComponent(__entry_index.clone(), tree)?, false)
            }
            LookupTree::Entry::IMPORT { index: __entry_index } => (resolveImport(__entry_index.clone(), tree)?, true),
        });
        Ok((element, isImport))
    }

    fn resolveEntryPtr(
        mut entry: &metamodelica::Ref<LookupTree::Entry::Entry>,
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>> {
        let mut element: Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>;
        let mut elems: metamodelica::Array<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>;
        element = (match &**entry {
            LookupTree::Entry::CLASS { index: __entry_index } => {
                let __pa0 = ::match_deref::match_deref! { match &((*tree)) {
                    Deref @ INSTANTIATED_TREE { classes: __pa0, .. } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                elems = metamodelica::Own::own(__pa0);
                metamodelica::arrayGet(elems.clone(), __entry_index.clone())?
            }
            LookupTree::Entry::COMPONENT { index: __entry_index } => {
                let __pa0 = ::match_deref::match_deref! { match &((*tree)) {
                    Deref @ INSTANTIATED_TREE { components: __pa0, .. } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                elems = metamodelica::Own::own(__pa0);
                metamodelica::arrayGet(elems.clone(), __entry_index.clone())?
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(element)
    }

    fn resolveDuplicateEntriesPtr(
        mut entry: &metamodelica::Ref<DuplicateTree::Entry>,
        mut tree: &metamodelica::Ref<ClassTree>,
        mut elements: metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>,
    ) -> Result<metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>> {
        let mut elements: metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>> = elements;
        let mut node_ptr: Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>;
        node_ptr = resolveEntryPtr(&entry.entry, tree)?;
        elements = metamodelica::cons(node_ptr, elements);
        for mut child in &*entry.children.clone() {
            elements = resolveDuplicateEntriesPtr(metamodelica::AsArg::as_arg(&child), tree, elements)?;
        }
        Ok(elements)
    }

    fn resolveClass(
        mut index: i32,
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<InstNode::InstNode>> {
        let mut element: metamodelica::Ref<InstNode::InstNode>;
        element = (match &**tree {
            PARTIAL_TREE { .. } => {
                metamodelica::arrayGet(var_field!((**tree).classes, ClassTree::PARTIAL_TREE).clone(), index)?
            }
            EXPANDED_TREE { .. } => {
                metamodelica::arrayGet(var_field!((**tree).classes, ClassTree::EXPANDED_TREE).clone(), index)?
            }
            INSTANTIATED_TREE { .. } => Mutable::access(metamodelica::arrayGet(
                var_field!((**tree).classes, ClassTree::INSTANTIATED_TREE).clone(),
                index,
            )?),
            FLAT_TREE { .. } => {
                metamodelica::arrayGet(var_field!((**tree).classes, ClassTree::FLAT_TREE).clone(), index)?
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(element)
    }

    fn resolveComponent(
        mut index: i32,
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<InstNode::InstNode>> {
        let mut element: metamodelica::Ref<InstNode::InstNode>;
        element = (match &**tree {
            INSTANTIATED_TREE { .. } => Mutable::access(metamodelica::arrayGet(
                var_field!((**tree).components, ClassTree::INSTANTIATED_TREE).clone(),
                index,
            )?),
            FLAT_TREE { .. } => {
                metamodelica::arrayGet(var_field!((**tree).components, ClassTree::FLAT_TREE).clone(), index)?
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(element)
    }

    fn resolveImport(
        mut index: i32,
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<InstNode::InstNode>> {
        let mut element: metamodelica::Ref<InstNode::InstNode>;
        let mut imports: metamodelica::Array<metamodelica::Ref<Import::NFImport>>;
        let mut imp: metamodelica::Ref<Import::NFImport>;
        let mut changed: bool;
        imports = (match &**tree {
            PARTIAL_TREE { .. } => var_field!((**tree).imports, ClassTree::PARTIAL_TREE).clone(),
            EXPANDED_TREE { .. } => var_field!((**tree).imports, ClassTree::EXPANDED_TREE).clone(),
            INSTANTIATED_TREE { .. } => var_field!((**tree).imports, ClassTree::INSTANTIATED_TREE).clone(),
            FLAT_TREE { .. } => var_field!((**tree).imports, ClassTree::FLAT_TREE).clone(),
            _ => return Err("match: no arm matched"),
        });
        (element, changed, imp) = Import::resolve(
            ({
                let __elt = (*metamodelica::index_checked(&imports.borrow(), index)?).clone();
                __elt
            }),
        )?;
        if changed {
            metamodelica::arrayUpdate(imports.clone(), index, imp)?;
        }
        Ok(element)
    }

    fn countElements(mut elements: &metamodelica::List<metamodelica::Ref<SCode::Element>>) -> (i32, i32, i32) {
        let mut classCount: i32 = 0;
        let mut compCount: i32 = 0;
        let mut extCount: i32 = 0;
        for mut e in &**elements {
            let () = (match &*e.clone() {
                SCode::Element::CLASS { .. } => {
                    classCount = classCount + 1;
                    ()
                }
                SCode::Element::COMPONENT { .. } => {
                    compCount = compCount + 1;
                    ()
                }
                SCode::Element::EXTENDS { .. } => {
                    extCount = extCount + 1;
                    ()
                }
                _ => (),
            });
        }
        (classCount, compCount, extCount)
    }

    fn countInheritedElements(
        mut extendsNode: metamodelica::Ref<InstNode::InstNode>,
        mut classCount: i32,
        mut componentCount: i32,
    ) -> Result<(i32, i32)> {
        let mut classCount: i32 = classCount;
        let mut componentCount: i32 = componentCount;
        let mut clss: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut exts: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let () = (match &*(Class::classTree(NFInstNode::InstNode::getClass(extendsNode)?)?) {
            EXPANDED_TREE {
                classes: __esc_clss,
                components: __esc_comps,
                exts: __esc_exts,
                ..
            } => {
                clss = (*__esc_clss).clone();
                comps = (*__esc_comps).clone();
                exts = (*__esc_exts).clone();
                componentCount =
                    componentCount + metamodelica::arrayLength(comps.clone()) - metamodelica::arrayLength(exts.clone());
                classCount = classCount + metamodelica::arrayLength(clss.clone());
                let __range0 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut ext in __range0 {
                    (classCount, componentCount) = countInheritedElements(ext, classCount, componentCount)?;
                }
                ()
            }
            FLAT_TREE {
                classes: __esc_clss,
                components: __esc_comps,
                ..
            } => {
                clss = (*__esc_clss).clone();
                comps = (*__esc_comps).clone();
                componentCount = componentCount + metamodelica::arrayLength(comps.clone());
                classCount = classCount + metamodelica::arrayLength(clss.clone());
                ()
            }
            _ => (),
        });
        Ok((classCount, componentCount))
    }

    fn expandExtends(
        mut extendsNode: metamodelica::Ref<InstNode::InstNode>,
        mut tree: metamodelica::Ref<LookupTree::Tree>,
        mut classOffset: i32,
        mut componentOffset: i32,
        mut duplicates: Mutable::Mutable<metamodelica::Ref<DuplicateTree::Tree>>,
    ) -> Result<metamodelica::Ref<LookupTree::Tree>> {
        let mut tree: metamodelica::Ref<LookupTree::Tree> = tree;
        let mut cls_tree: metamodelica::Ref<ClassTree>;
        let mut ext_tree: metamodelica::Ref<LookupTree::Tree>;
        let mut ext_dups: metamodelica::Ref<DuplicateTree::Tree>;
        let mut dups: metamodelica::Ref<DuplicateTree::Tree>;
        let mut conf_func: LookupTree::ConflictFunc;
        cls_tree = Class::classTree(NFInstNode::InstNode::getClass(extendsNode)?)?;
        (ext_tree, ext_dups) = (match &*cls_tree {
            EXPANDED_TREE {
                duplicates: __cls_tree_duplicates,
                tree: __cls_tree_tree,
                ..
            } => (__cls_tree_tree.clone(), __cls_tree_duplicates.clone()),
            FLAT_TREE {
                duplicates: __cls_tree_duplicates,
                tree: __cls_tree_tree,
                ..
            } => (__cls_tree_tree.clone(), __cls_tree_duplicates.clone()),
            _ => {
                return Ok(tree);
                (tree.clone(), DuplicateTree::new())
            }
        });
        if !(DuplicateTree::isEmpty(&ext_dups)) {
            dups = DuplicateTree::map(
                ext_dups.clone(),
                &({
                    let __pe_b2 = classOffset;
                    let __pe_b3 = componentOffset;
                    move |__pe_a0, __pe_a1| offsetDuplicates(&__pe_a0, &__pe_a1, __pe_b2.clone(), __pe_b3.clone())
                }),
            )?;
            dups = DuplicateTree::join(
                Mutable::access(duplicates.clone()),
                &dups,
                &move |__a0: metamodelica::Ref<DuplicateTree::Entry>,
                       __a1: metamodelica::Ref<DuplicateTree::Entry>,
                       __a2: ArcStr|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(joinDuplicates(__a0, __a1, &__a2))
                },
            )?;
            Mutable::update(duplicates.clone(), dups);
        }
        conf_func = (std::sync::Arc::new({
            let __pe_b3 = duplicates;
            let __pe_b4 = ext_dups;
            move |__pe_a0, __pe_a1, __pe_a2| {
                addInheritedElementConflict(__pe_a0, __pe_a1, __pe_a2, __pe_b3.clone(), &__pe_b4)
            }
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<LookupTree::Entry::Entry>,
                        metamodelica::Ref<LookupTree::Entry::Entry>,
                        ArcStr,
                    ) -> Result<metamodelica::Ref<LookupTree::Entry::Entry>>
                    + 'static,
            >);
        tree = LookupTree::fold(
            &ext_tree,
            &({
                let __pe_b2 = classOffset;
                let __pe_b3 = componentOffset;
                let __pe_b4 = conf_func.clone();
                move |__pe_a0, __pe_a1, __pe_a5| {
                    addInheritedElement(&__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), &*__pe_b4, __pe_a5)
                }
            }),
            tree,
        )?;
        Ok(tree)
    }

    fn addInheritedElement(
        mut name: &ArcStr,
        mut entry: metamodelica::Ref<LookupTree::Entry::Entry>,
        mut classOffset: i32,
        mut componentOffset: i32,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<LookupTree::Entry::Entry>,
            metamodelica::Ref<LookupTree::Entry::Entry>,
            ArcStr,
        ) -> Result<metamodelica::Ref<LookupTree::Entry::Entry>>,
        mut tree: metamodelica::Ref<LookupTree::Tree>,
    ) -> Result<metamodelica::Ref<LookupTree::Tree>> {
        let mut tree: metamodelica::Ref<LookupTree::Tree> = tree;
        let () = (match &*entry {
            LookupTree::Entry::CLASS { index: __entry_index } => {
                assign_variant_field!(entry => LookupTree::Entry::Entry::CLASS; index = __entry_index.clone() + classOffset);
                tree = LookupTree::add(tree, name, &entry, conflictFunc)?;
                ()
            }
            LookupTree::Entry::COMPONENT { index: __entry_index } => {
                assign_variant_field!(entry => LookupTree::Entry::Entry::COMPONENT; index = __entry_index.clone() + componentOffset);
                tree = LookupTree::add(tree, name, &entry, conflictFunc)?;
                ()
            }
            _ => (),
        });
        Ok(tree)
    }

    fn addInheritedElementConflict(
        mut newEntry: metamodelica::Ref<LookupTree::Entry::Entry>,
        mut oldEntry: metamodelica::Ref<LookupTree::Entry::Entry>,
        mut name: ArcStr,
        mut duplicates: Mutable::Mutable<metamodelica::Ref<DuplicateTree::Tree>>,
        mut extDuplicates: &metamodelica::Ref<DuplicateTree::Tree>,
    ) -> Result<metamodelica::Ref<LookupTree::Entry::Entry>> {
        let mut entry: metamodelica::Ref<LookupTree::Entry::Entry>;
        let mut dups: metamodelica::Ref<DuplicateTree::Tree>;
        let mut opt_dup_entry: Option<metamodelica::Ref<DuplicateTree::Entry>>;
        let mut dup_entry: metamodelica::Ref<DuplicateTree::Entry>;
        let mut new_id: i32 = LookupTree::Entry::index(&newEntry);
        let mut old_id: i32 = LookupTree::Entry::index(&oldEntry);
        let mut ty: DuplicateTree::EntryType;
        if LookupTree::Entry::isImport(&oldEntry) {
            entry = newEntry;
            return Ok(entry);
        }
        dups = Mutable::access(duplicates.clone());
        opt_dup_entry = DuplicateTree::getOpt(&dups, name.clone());
        if (opt_dup_entry).is_none() {
            if new_id < old_id {
                entry = newEntry.clone();
                dup_entry = DuplicateTree::newDuplicate(newEntry, oldEntry);
            } else {
                entry = oldEntry.clone();
                dup_entry = DuplicateTree::newDuplicate(oldEntry, newEntry);
            }
            dups = DuplicateTree::add(
                dups,
                &name,
                &dup_entry,
                &*(std::sync::Arc::new(DuplicateTree::addConflictDefault)
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
            Mutable::update(duplicates, dups);
        } else {
            let __pa0 = ::match_deref::match_deref! { match &(opt_dup_entry) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            dup_entry = metamodelica::Own::own(__pa0);
            ty = dup_entry.ty.clone();
            if !(DuplicateTree::idExistsInEntry(&newEntry, &dup_entry)?) {
                if ty == DuplicateTree::EntryType::REDECLARE.clone() {
                    entry = newEntry.clone();
                    assign_field!(
                        dup_entry.children =
                            metamodelica::cons(DuplicateTree::newEntry(newEntry), dup_entry.children.clone())
                    );
                } else {
                    if new_id < old_id {
                        entry = newEntry.clone();
                        dup_entry = metamodelica::Ref::new(DuplicateTree::Entry {
                            entry: newEntry,
                            node: None,
                            children: metamodelica::cons(DuplicateTree::newEntry(oldEntry), dup_entry.children.clone()),
                            ty: dup_entry.ty.clone(),
                        });
                    } else {
                        entry = oldEntry;
                        assign_field!(
                            dup_entry.children =
                                metamodelica::cons(DuplicateTree::newEntry(newEntry), dup_entry.children.clone())
                        );
                    }
                }
                dups = DuplicateTree::update(dups, &name, &dup_entry)?;
                Mutable::update(duplicates, dups);
            } else if !(DuplicateTree::idExistsInEntry(&oldEntry, &dup_entry)?) {
                if ty == DuplicateTree::EntryType::REDECLARE.clone() || new_id < old_id {
                    entry = newEntry;
                    assign_field!(
                        dup_entry.children =
                            metamodelica::cons(DuplicateTree::newEntry(oldEntry), dup_entry.children.clone())
                    );
                } else {
                    entry = newEntry.clone();
                    dup_entry = metamodelica::Ref::new(DuplicateTree::Entry {
                        entry: newEntry,
                        node: None,
                        children: metamodelica::cons(DuplicateTree::newEntry(oldEntry), dup_entry.children.clone()),
                        ty: dup_entry.ty.clone(),
                    });
                }
                dups = DuplicateTree::update(dups, &name, &dup_entry)?;
                Mutable::update(duplicates, dups);
            } else {
                entry = if (new_id < old_id) { newEntry } else { oldEntry };
            }
        }
        Ok(entry)
    }

    fn offsetDuplicates(
        mut name: &ArcStr,
        mut entry: &metamodelica::Ref<DuplicateTree::Entry>,
        mut classOffset: i32,
        mut componentOffset: i32,
    ) -> Result<metamodelica::Ref<DuplicateTree::Entry>> {
        let mut offsetEntry: metamodelica::Ref<DuplicateTree::Entry>;
        let mut parent: metamodelica::Ref<LookupTree::Entry::Entry>;
        let mut children: metamodelica::List<metamodelica::Ref<DuplicateTree::Entry>>;
        parent = offsetDuplicate(&entry.entry, classOffset, componentOffset)?;
        children = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DuplicateTree::Entry>> = metamodelica::nil();
            for mut c in (entry.children.clone()).into_iter().cloned() {
                let __x = offsetDuplicates(name, &(c.clone()), classOffset, componentOffset)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        offsetEntry = metamodelica::Ref::new(DuplicateTree::Entry {
            entry: parent,
            node: None,
            children: children,
            ty: entry.ty.clone(),
        });
        Ok(offsetEntry)
    }

    fn offsetDuplicate(
        mut entry: &metamodelica::Ref<LookupTree::Entry::Entry>,
        mut classOffset: i32,
        mut componentOffset: i32,
    ) -> Result<metamodelica::Ref<LookupTree::Entry::Entry>> {
        let mut offsetEntry: metamodelica::Ref<LookupTree::Entry::Entry>;
        offsetEntry = (match &**entry {
            LookupTree::Entry::CLASS { index: __entry_index } => {
                metamodelica::Ref::new(LookupTree::Entry::Entry::CLASS {
                    index: __entry_index.clone() + classOffset,
                })
            }
            LookupTree::Entry::COMPONENT { index: __entry_index } => {
                metamodelica::Ref::new(LookupTree::Entry::Entry::COMPONENT {
                    index: __entry_index.clone() + componentOffset,
                })
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(offsetEntry)
    }

    fn joinDuplicates(
        mut newEntry: metamodelica::Ref<DuplicateTree::Entry>,
        mut oldEntry: metamodelica::Ref<DuplicateTree::Entry>,
        mut name: &ArcStr,
    ) -> metamodelica::Ref<DuplicateTree::Entry> {
        let mut entry: metamodelica::Ref<DuplicateTree::Entry> = oldEntry;
        assign_field!(entry.children = metamodelica::cons(newEntry, entry.children.clone()));
        entry
    }

    fn enumerateDuplicates(
        mut duplicates: &metamodelica::Ref<DuplicateTree::Tree>,
    ) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
        let mut classes: metamodelica::List<i32>;
        let mut components: metamodelica::List<i32>;
        if DuplicateTree::isEmpty(duplicates) {
            classes = metamodelica::nil();
            components = metamodelica::nil();
        } else {
            (classes, components) = DuplicateTree::fold_2(
                duplicates,
                &move |__a0: ArcStr,
                       __a1: metamodelica::Ref<DuplicateTree::Entry>,
                       __a2: metamodelica::List<i32>,
                       __a3: metamodelica::List<i32>| enumerateDuplicates2(&__a0, &__a1, __a2, __a3),
                metamodelica::nil(),
                metamodelica::nil(),
            )?;
            classes = List::sort(
                classes,
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
            components = List::sort(
                components,
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
        }
        Ok((classes, components))
    }

    fn enumerateDuplicates2(
        mut name: &ArcStr,
        mut entry: &metamodelica::Ref<DuplicateTree::Entry>,
        mut classes: metamodelica::List<i32>,
        mut components: metamodelica::List<i32>,
    ) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
        let mut classes: metamodelica::List<i32> = classes;
        let mut components: metamodelica::List<i32> = components;
        for mut c in &*entry.children.clone() {
            (classes, components) = enumerateDuplicates3(metamodelica::AsArg::as_arg(&c), classes, components)?;
        }
        Ok((classes, components))
    }

    fn enumerateDuplicates3(
        mut entry: &metamodelica::Ref<DuplicateTree::Entry>,
        mut classes: metamodelica::List<i32>,
        mut components: metamodelica::List<i32>,
    ) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
        let mut classes: metamodelica::List<i32> = classes;
        let mut components: metamodelica::List<i32> = components;
        (classes, components) = enumerateDuplicates4(&entry.entry, classes, components)?;
        for mut c in &*entry.children.clone() {
            (classes, components) = enumerateDuplicates3(metamodelica::AsArg::as_arg(&c), classes, components)?;
        }
        Ok((classes, components))
    }

    fn enumerateDuplicates4(
        mut entry: &metamodelica::Ref<LookupTree::Entry::Entry>,
        mut classes: metamodelica::List<i32>,
        mut components: metamodelica::List<i32>,
    ) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
        let mut classes: metamodelica::List<i32> = classes;
        let mut components: metamodelica::List<i32> = components;
        let () = (match &**entry {
            LookupTree::Entry::CLASS { .. } => (),
            LookupTree::Entry::COMPONENT { index: __entry_index } => {
                components = metamodelica::cons(__entry_index.clone(), components);
                ()
            }
            _ => return Err("match: no arm matched"),
        });
        Ok((classes, components))
    }

    fn mapRedeclareChain(
        mut name: &ArcStr,
        mut entry: metamodelica::Ref<DuplicateTree::Entry>,
        mut func: &dyn ::std::ops::Fn(metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>) -> Result<()>,
        mut tree: &metamodelica::Ref<ClassTree>,
    ) -> Result<metamodelica::Ref<DuplicateTree::Entry>> {
        pub type FuncT = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>,
                ) -> Result<()>
                + 'static,
        >;

        let mut entry: metamodelica::Ref<DuplicateTree::Entry> = entry;
        let mut chain: metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>;
        chain = getRedeclareChain(entry.clone(), tree, metamodelica::nil())?;
        if !((chain).is_empty()) {
            func(chain)?;
        }
        Ok(entry)
    }

    fn getRedeclareChain<'__b>(
        mut entry: metamodelica::Ref<DuplicateTree::Entry>,
        mut tree: &'__b metamodelica::Ref<ClassTree>,
        mut chain: metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>,
    ) -> Result<metamodelica::List<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>> {
        '__tco: loop {
            match entry.ty.clone() {
                DuplicateTree::EntryType::REDECLARE => {
                    let mut node_ptr: Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>;
                    let mut node: metamodelica::Ref<InstNode::InstNode>;
                    node_ptr = resolveEntryPtr(&entry.entry, tree)?;
                    if (entry.children).is_empty() {
                        node = Mutable::access(node_ptr.clone());
                        if SCodeUtil::isClassExtends(&(NFInstNode::InstNode::definition(node.clone())?)) {
                            Error::addSourceMessage(
                                &(Error::CLASS_EXTENDS_TARGET_NOT_FOUND.clone()),
                                list![NFInstNode::InstNode::name(&node)?],
                                &(NFInstNode::InstNode::info(&node)),
                            )?;
                        } else {
                            Error::addSourceMessage(
                                &(Error::REDECLARE_NONEXISTING_ELEMENT.clone()),
                                list![NFInstNode::InstNode::name(&node)?],
                                &(NFInstNode::InstNode::info(&node)),
                            )?;
                        }
                        return Err("fail");
                    }
                    {
                        (entry, tree, chain) = (
                            (entry.children).head().cloned()?,
                            tree,
                            metamodelica::cons(node_ptr, chain),
                        );
                        continue '__tco;
                    }
                }
                DuplicateTree::EntryType::ENTRY => {
                    let mut node_ptr: Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>;
                    node_ptr = resolveEntryPtr(&entry.entry, tree)?;
                    return Ok(metamodelica::cons(node_ptr, chain));
                }
                _ => return Ok(chain),
            }
        }
    }

    fn replaceDuplicates2(
        mut name: &ArcStr,
        mut entry: metamodelica::Ref<DuplicateTree::Entry>,
        mut tree: metamodelica::Ref<ClassTree>,
    ) -> Result<(metamodelica::Ref<DuplicateTree::Entry>, metamodelica::Ref<ClassTree>)> {
        let mut entry: metamodelica::Ref<DuplicateTree::Entry> = entry;
        let mut tree: metamodelica::Ref<ClassTree> = tree;
        let mut kept: metamodelica::Ref<InstNode::InstNode>;
        let mut node_ptr: Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>;
        let mut node: metamodelica::Ref<InstNode::InstNode>;
        let mut entries: metamodelica::List<metamodelica::Ref<DuplicateTree::Entry>>;
        let mut broken_entries: metamodelica::List<metamodelica::Ref<DuplicateTree::Entry>>;
        let () = (match entry.ty.clone() {
            DuplicateTree::EntryType::REDECLARE => {
                kept = Mutable::access(resolveEntryPtr(&entry.entry, &tree)?);
                entry = replaceDuplicates3(entry, kept);
                ()
            }
            DuplicateTree::EntryType::DUPLICATE => {
                entries = metamodelica::nil();
                broken_entries = metamodelica::nil();
                kept = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
                for mut e in &*DuplicateTree::entryToList(entry.clone()) {
                    let mut e = e.clone();
                    node_ptr = resolveEntryPtr(&e.entry, &tree)?;
                    node = Mutable::access(node_ptr);
                    assign_field!(e.node = Some(node.clone()), e.children = metamodelica::nil());
                    if !(NFInstNode::InstNode::isEmpty(&node)) {
                        if NFInstNode::InstNode::isEmpty(&kept) {
                            kept = node;
                        }
                        entries = metamodelica::cons(e, entries);
                    } else {
                        broken_entries = metamodelica::cons(e, broken_entries);
                    }
                }
                for mut e in &*entries {
                    node_ptr = resolveEntryPtr(&e.entry, &tree)?;
                    Mutable::update(node_ptr, kept.clone());
                }
                if (entries).is_empty() {
                    assign_field!(entry.node = None, entry.children = metamodelica::nil());
                    return Ok((entry, tree));
                } else {
                    entries = metamodelica::Dangerous::listReverseInPlace(entries);
                    entry = (entries).head().cloned()?;
                    assign_field!(entry.children = listAppend((entries).rest()?, broken_entries));
                }
                ()
            }
            _ => (),
        });
        Ok((entry, tree))
    }

    fn replaceDuplicates3(
        mut entry: metamodelica::Ref<DuplicateTree::Entry>,
        mut node: metamodelica::Ref<InstNode::InstNode>,
    ) -> metamodelica::Ref<DuplicateTree::Entry> {
        let mut entry: metamodelica::Ref<DuplicateTree::Entry> = entry;
        assign_field!(
            entry.node = Some(node.clone()),
            entry.children = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DuplicateTree::Entry>> = metamodelica::nil();
                for mut c in (entry.children.clone()).into_iter().cloned() {
                    let __x = replaceDuplicates3(c.clone(), node.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        );
        entry
    }

    fn linkInnerOuter(
        mut outerNode: metamodelica::Ref<InstNode::InstNode>,
        mut scope: metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<metamodelica::Ref<InstNode::InstNode>> {
        let mut innerOuterNode: metamodelica::Ref<InstNode::InstNode>;
        let mut inner_node: metamodelica::Ref<InstNode::InstNode>;
        inner_node = Lookup::lookupInner(outerNode.clone(), scope)?;
        if metamodelica::valueConstructor((&*&*outerNode))? != metamodelica::valueConstructor((&*&*inner_node))? {
            Error::addMultiSourceMessage(
                &(Error::FOUND_WRONG_INNER_ELEMENT.clone()),
                &(list![
                    NFInstNode::InstNode::typeName(&inner_node)?,
                    NFInstNode::InstNode::name(&outerNode)?,
                    NFInstNode::InstNode::typeName(&outerNode)?
                ]),
                &(list![
                    NFInstNode::InstNode::info(&outerNode),
                    NFInstNode::InstNode::info(&inner_node)
                ]),
            )?;
            return Err("fail");
        }
        innerOuterNode = metamodelica::Ref::new(InstNode::InstNode::INNER_OUTER_NODE {
            innerNode: inner_node,
            outerNode: outerNode,
        });
        Ok(innerOuterNode)
    }

    fn checkOuterClass(mut outerCls: metamodelica::Ref<InstNode::InstNode>) -> Result<()> {
        let mut def: metamodelica::Ref<SCode::ClassDef>;
        if NFInstNode::InstNode::isOnlyOuter(&outerCls)? {
            def = SCodeUtil::getClassDef(&(NFInstNode::InstNode::definition(outerCls.clone())?))?;
            let () = (::match_deref::match_deref! { match &(def) {
                Deref @ SCode::ClassDef::DERIVED { modifications: Deref @ SCode::Mod::NOMOD { .. }, .. } => (),
                Deref @ SCode::ClassDef::DERIVED { modifications: __def_modifications, .. } => {
                    Error::addSourceMessage(&(Error::OUTER_ELEMENT_MOD.clone()), list![SCodeDump::printModStr(__def_modifications.clone(), SCodeDump::defaultOptions.clone())?, NFInstNode::InstNode::name(&outerCls)?], &(NFInstNode::InstNode::info(&outerCls)))?;
                    return Err("fail")
                },
                _ => {
                    Error::addSourceMessage(&(Error::OUTER_LONG_CLASS.clone()), list![NFInstNode::InstNode::name(&outerCls)?], &(NFInstNode::InstNode::info(&outerCls)))?;
                    return Err("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        Ok(())
    }

    fn getBreakModsInExtend(
        mut extendsNode: &metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<metamodelica::List<metamodelica::Ref<SCode::SubMod>>> {
        let mut breaks: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
        let mut r#mod: metamodelica::Ref<SCode::Mod>;
        let mut opt_def: Option<metamodelica::Ref<SCode::Element>>;
        opt_def = NFInstNode::InstNode::extendsDefinition(extendsNode)?;
        breaks = (::match_deref::match_deref! { match &(opt_def) {
            Some(Deref @ SCode::Element::EXTENDS { modifications: __esc_mod @ Deref @ SCode::Mod::MOD { .. }, .. }) => {
                r#mod = (*__esc_mod).clone();
                ({
            let mut __acc: metamodelica::List<metamodelica::Ref<SCode::SubMod>> = metamodelica::nil();
            for mut sm in (var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()).into_iter().cloned() {
                if !(SCodeUtil::isBreakComponentSubMod(&(sm.clone()))) { continue; }
                let __x = sm.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
            },
            _ => metamodelica::nil(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(breaks)
    }

    fn breakComponents(
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut components: metamodelica::Array<Mutable::Mutable<metamodelica::Ref<InstNode::InstNode>>>,
        mut tree: &metamodelica::Ref<LookupTree::Tree>,
        mut duplicates: &metamodelica::Ref<DuplicateTree::Tree>,
    ) -> Result<()> {
        let __ab_components = components.borrow();
        let mut break_mods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
        let mut opt_dentry: Option<metamodelica::Ref<DuplicateTree::Entry>>;
        let mut opt_lentry: Option<metamodelica::Ref<LookupTree::Entry::Entry>>;
        let mut entries: metamodelica::List<metamodelica::Ref<LookupTree::Entry::Entry>>;
        let mut index: i32;
        let mut info: SourceInfo;
        break_mods = getBreakModsInExtend(&node)?;
        if (break_mods).is_empty() {
            return Ok(());
        }
        for mut bm in &*break_mods {
            info = SCodeUtil::getModifierInfo(&bm.r#mod);
            opt_dentry = DuplicateTree::getOpt(duplicates, bm.ident.clone());
            if (opt_dentry).is_some() {
                entries = DuplicateTree::getLookupEntries(&(Util::getOption(opt_dentry)?));
            } else {
                opt_lentry = LookupTree::getOpt(tree, bm.ident.clone());
                entries = if ((opt_lentry).is_some()) {
                    list![Util::getOption(opt_lentry)?]
                } else {
                    metamodelica::nil()
                };
            }
            if (entries).is_empty()
                || List::all(
                    &entries,
                    &move |__a0: metamodelica::Ref<LookupTree::Entry::Entry>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(LookupTree::Entry::isImport(&__a0))
                    },
                )?
            {
                Error::addSourceMessage(
                    &(Error::MISSING_MODIFIED_ELEMENT.clone()),
                    list![bm.ident.clone(), NFInstNode::InstNode::name(&node)?],
                    &info,
                )?;
                return Err("fail");
            }
            for mut e in &*entries {
                index = (match &*e.clone() {
                    LookupTree::Entry::COMPONENT { index: __e_index } => __e_index.clone(),
                    _ => {
                        Error::addSourceMessage(
                            &(Error::NON_BREAKABLE_ELEMENT.clone()),
                            list![bm.ident.clone()],
                            &info,
                        )?;
                        return Err("fail");
                    }
                });
                checkIsBreakable(
                    Mutable::access((*metamodelica::index_checked(&__ab_components, index)?).clone()),
                    node.clone(),
                    info.clone(),
                )?;
                Mutable::update(
                    (*metamodelica::index_checked(&__ab_components, index)?).clone(),
                    crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
                );
            }
        }
        Ok(())
    }

    fn checkIsBreakable(
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut scope: metamodelica::Ref<InstNode::InstNode>,
        mut info: SourceInfo,
    ) -> Result<()> {
        let mut ty_path: metamodelica::Ref<Absyn::Path>;
        let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
        let mut restriction: SCode::Restriction;
        match '__try0: {
            ty_path = unwrap_break_err!(SCodeUtil::getElementTypePath(&(unwrap_break_err!(NFInstNode::InstNode::definition(node.clone()), '__try0))), '__try0);
            (cls_node, _, _) = unwrap_break_err!(Lookup::lookupName(&ty_path, scope.clone(), NFInstContext::NO_CONTEXT.clone(), false), '__try0);
            restriction = unwrap_break_err!(SCodeUtil::getClassRestriction(&(unwrap_break_err!(NFInstNode::InstNode::definition(cls_node.clone()), '__try0))), '__try0);
            Ok::<_, &'static str>((restriction.clone(),))
        } {
            Ok((__try0_o0,)) => {
                restriction = __try0_o0;
            }
            Err(_) => {
                restriction = openmodelica_frontend_types::SCode::Restriction::R_CLASS;
            }
        }
        let () = (match restriction {
            SCode::Restriction::R_MODEL { .. } => (),
            SCode::Restriction::R_BLOCK { .. } => (),
            SCode::Restriction::R_CONNECTOR { .. } => (),
            _ => {
                Error::addMultiSourceMessage(
                    &(Error::NON_BREAKABLE_COMPONENT.clone()),
                    &(list![NFInstNode::InstNode::name(&node)?]),
                    &(list![info, NFInstNode::InstNode::info(&node)]),
                )?;
                return Err("fail");
            }
        });
        Ok(())
    }
}
