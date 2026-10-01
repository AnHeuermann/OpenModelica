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

use crate::BaseModelica;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFModifier::Modifier;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::SCode::Element;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::MutableWeak;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFClass {
    NOT_INSTANTIATED,
    PARTIAL_CLASS {
        elements: metamodelica::Ref<ClassTree::ClassTree>,
        modifier: metamodelica::Ref<Modifier::Modifier>,
        ccMod: metamodelica::Ref<Modifier::Modifier>,
        prefixes: metamodelica::Ref<Prefixes::Prefixes>,
    },
    PARTIAL_BUILTIN {
        ty: metamodelica::Ref<Type::NFType>,
        elements: metamodelica::Ref<ClassTree::ClassTree>,
        modifier: metamodelica::Ref<Modifier::Modifier>,
        prefixes: metamodelica::Ref<Prefixes::Prefixes>,
        restriction: metamodelica::Ref<Restriction::NFRestriction>,
    },
    EXPANDED_CLASS {
        elements: metamodelica::Ref<ClassTree::ClassTree>,
        modifier: metamodelica::Ref<Modifier::Modifier>,
        ccMod: metamodelica::Ref<Modifier::Modifier>,
        prefixes: metamodelica::Ref<Prefixes::Prefixes>,
        restriction: metamodelica::Ref<Restriction::NFRestriction>,
    },
    EXPANDED_DERIVED {
        baseClass: metamodelica::Ref<InstNode::InstNode>,
        modifier: metamodelica::Ref<Modifier::Modifier>,
        ccMod: metamodelica::Ref<Modifier::Modifier>,
        dims: metamodelica::Array<metamodelica::Ref<Dimension::NFDimension>>,
        prefixes: metamodelica::Ref<Prefixes::Prefixes>,
        attributes: metamodelica::Ref<Attributes::NFAttributes>,
        restriction: metamodelica::Ref<Restriction::NFRestriction>,
    },
    INSTANCED_CLASS {
        ty: metamodelica::Ref<Type::NFType>,
        elements: metamodelica::Ref<ClassTree::ClassTree>,
        sections: metamodelica::Ref<Sections::NFSections>,
        prefixes: metamodelica::Ref<Prefixes::Prefixes>,
        restriction: metamodelica::Ref<Restriction::NFRestriction>,
    },
    INSTANCED_BUILTIN {
        ty: metamodelica::Ref<Type::NFType>,
        elements: metamodelica::Ref<ClassTree::ClassTree>,
        restriction: metamodelica::Ref<Restriction::NFRestriction>,
    },
    TYPED_DERIVED {
        ty: metamodelica::Ref<Type::NFType>,
        baseClass: metamodelica::Ref<InstNode::InstNode>,
        restriction: metamodelica::Ref<Restriction::NFRestriction>,
    },
    DAE_TYPE {
        ty: metamodelica::Ref<DAE::Type>,
    },
}
impl metamodelica::gc::MMTrace for NFClass {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFClass::NOT_INSTANTIATED => Ok(()),
            NFClass::PARTIAL_CLASS {
                elements,
                modifier,
                ccMod,
                prefixes,
            } => {
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modifier, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ccMod, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                Ok(())
            }
            NFClass::PARTIAL_BUILTIN {
                ty,
                elements,
                modifier,
                prefixes,
                restriction,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modifier, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restriction, __mmv)?;
                Ok(())
            }
            NFClass::EXPANDED_CLASS {
                elements,
                modifier,
                ccMod,
                prefixes,
                restriction,
            } => {
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modifier, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ccMod, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restriction, __mmv)?;
                Ok(())
            }
            NFClass::EXPANDED_DERIVED {
                baseClass,
                modifier,
                ccMod,
                dims,
                prefixes,
                attributes,
                restriction,
            } => {
                metamodelica::gc::MMTrace::mm_accept(baseClass, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modifier, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ccMod, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dims, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attributes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restriction, __mmv)?;
                Ok(())
            }
            NFClass::INSTANCED_CLASS {
                ty,
                elements,
                sections,
                prefixes,
                restriction,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(sections, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefixes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restriction, __mmv)?;
                Ok(())
            }
            NFClass::INSTANCED_BUILTIN {
                ty,
                elements,
                restriction,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restriction, __mmv)?;
                Ok(())
            }
            NFClass::TYPED_DERIVED {
                ty,
                baseClass,
                restriction,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseClass, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restriction, __mmv)?;
                Ok(())
            }
            NFClass::DAE_TYPE { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
        }
    }
}
impl NFClass {
    pub fn interned_NOT_INSTANTIATED() -> metamodelica::Ref<NFClass> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFClass> = metamodelica::Ref::new(NFClass::NOT_INSTANTIATED);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_NOT_INSTANTIATED() -> metamodelica::Ref<NFClass> {
    NFClass::interned_NOT_INSTANTIATED()
}
impl Default for NFClass {
    fn default() -> Self {
        Self::NOT_INSTANTIATED
    }
}
pub use self::NFClass::{
    DAE_TYPE, EXPANDED_CLASS, EXPANDED_DERIVED, INSTANCED_BUILTIN, INSTANCED_CLASS, NOT_INSTANTIATED, PARTIAL_BUILTIN,
    PARTIAL_CLASS, TYPED_DERIVED,
};
pub(crate) static DEFAULT_PREFIXES: std::sync::LazyLock<metamodelica::Ref<Prefixes::Prefixes>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Prefixes::Prefixes {
            encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
            partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
            replaceablePrefix: openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE(),
        })
    });

pub mod Prefixes {
    use super::*;
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Prefixes {
        pub encapsulatedPrefix: SCode::Encapsulated,
        pub partialPrefix: SCode::Partial,
        pub finalPrefix: SCode::Final,
        pub innerOuter: Absyn::InnerOuter,
        pub replaceablePrefix: metamodelica::Ref<SCode::Replaceable>,
    }

    impl metamodelica::gc::MMTrace for Prefixes {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.encapsulatedPrefix, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.partialPrefix, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.finalPrefix, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.innerOuter, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.replaceablePrefix, __mmv)?;
            Ok(())
        }
    }
    impl Default for Prefixes {
        fn default() -> Self {
            Self {
                encapsulatedPrefix: Default::default(),
                partialPrefix: Default::default(),
                finalPrefix: Default::default(),
                innerOuter: Default::default(),
                replaceablePrefix: Default::default(),
            }
        }
    }

    pub type PREFIXES = Prefixes;

    pub(crate) fn isEqual(mut prefs1: metamodelica::Ref<Prefixes>, mut prefs2: metamodelica::Ref<Prefixes>) -> bool {
        let mut isEqual: bool = prefs1.clone() == prefs2.clone();
        isEqual
    }

    pub(crate) fn isPartial(mut prefs: &metamodelica::Ref<Prefixes>) -> bool {
        let mut isPartial: bool = SCodeUtil::partialBool(prefs.partialPrefix.clone());
        isPartial
    }

    pub(crate) fn isEncapsulated(mut prefs: &metamodelica::Ref<Prefixes>) -> bool {
        let mut isEncapsulated: bool = SCodeUtil::encapsulatedBool(prefs.encapsulatedPrefix.clone());
        isEncapsulated
    }
}

pub(crate) fn fromSCode(
    mut elements: &metamodelica::List<metamodelica::Ref<Element>>,
    mut isClassExtends: bool,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut prefixes: metamodelica::Ref<Prefixes::Prefixes>,
) -> Result<metamodelica::Ref<NFClass>> {
    let mut cls: metamodelica::Ref<NFClass>;
    let mut tree: metamodelica::Ref<ClassTree::ClassTree>;
    tree = ClassTree::fromSCode(elements, isClassExtends, scope)?;
    cls = metamodelica::Ref::new(NFClass::PARTIAL_CLASS {
        elements: tree,
        modifier: crate::NFModifier::Modifier::interned_NOMOD(),
        ccMod: crate::NFModifier::Modifier::interned_NOMOD(),
        prefixes: prefixes,
    });
    Ok(cls)
}

pub(crate) fn initImports(
    mut cls: metamodelica::Ref<NFClass>,
    mut parent: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<NFClass>> {
    let mut cls: metamodelica::Ref<NFClass> = cls;
    let () = (match &*cls {
        PARTIAL_CLASS {
            elements: __cls_elements,
            ..
        } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; elements = ClassTree::initImports(__cls_elements.clone(), parent)?);
            ()
        }
        _ => (),
    });
    Ok(cls)
}

pub(crate) fn fromEnumeration(
    mut literals: &metamodelica::List<metamodelica::Ref<SCode::Enum>>,
    mut enumType: metamodelica::Ref<Type::NFType>,
    mut prefixes: metamodelica::Ref<Prefixes::Prefixes>,
    mut enumClass: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<NFClass>> {
    let mut cls: metamodelica::Ref<NFClass>;
    let mut tree: metamodelica::Ref<ClassTree::ClassTree>;
    tree = ClassTree::fromEnumeration(literals, enumType.clone(), enumClass)?;
    cls = metamodelica::Ref::new(NFClass::PARTIAL_BUILTIN {
        ty: enumType,
        elements: tree,
        modifier: crate::NFModifier::Modifier::interned_NOMOD(),
        prefixes: prefixes,
        restriction: crate::NFRestriction::interned_ENUMERATION(),
    });
    Ok(cls)
}

pub(crate) fn makeRecordConstructor(
    mut fields: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    mut out: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<NFClass>> {
    let mut cls: metamodelica::Ref<NFClass>;
    let mut tree: metamodelica::Ref<ClassTree::ClassTree>;
    tree = ClassTree::fromRecordConstructor(fields, out)?;
    cls = metamodelica::Ref::new(NFClass::INSTANCED_CLASS {
        ty: crate::NFType::interned_UNKNOWN(),
        elements: tree,
        sections: crate::NFSections::interned_EMPTY(),
        prefixes: DEFAULT_PREFIXES.clone(),
        restriction: crate::NFRestriction::interned_RECORD_CONSTRUCTOR(),
    });
    Ok(cls)
}

pub(crate) fn initExpandedClass(mut cls: metamodelica::Ref<NFClass>) -> Result<metamodelica::Ref<NFClass>> {
    let mut cls: metamodelica::Ref<NFClass> = cls;
    cls = (match &*cls {
        PARTIAL_CLASS {
            ccMod: __cls_ccMod,
            elements: __cls_elements,
            modifier: __cls_modifier,
            prefixes: __cls_prefixes,
        } => metamodelica::Ref::new(NFClass::EXPANDED_CLASS {
            elements: __cls_elements.clone(),
            modifier: __cls_modifier.clone(),
            ccMod: __cls_ccMod.clone(),
            prefixes: __cls_prefixes.clone(),
            restriction: crate::NFRestriction::interned_UNKNOWN(),
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(cls)
}

pub fn getSections(mut cls: metamodelica::Ref<NFClass>) -> Result<metamodelica::Ref<Sections::NFSections>> {
    '__tco: loop {
        match &*cls {
            INSTANCED_CLASS {
                sections: __cls_sections,
                ..
            } => return Ok(__cls_sections.clone()),
            TYPED_DERIVED {
                baseClass: __cls_baseClass,
                ..
            } => {
                cls = NFInstNode::InstNode::getClass(__cls_baseClass.clone())?;
                continue '__tco;
            }
            _ => return Ok(crate::NFSections::interned_EMPTY()),
        }
    }
}

pub(crate) fn setSections(
    mut sections: metamodelica::Ref<Sections::NFSections>,
    mut cls: metamodelica::Ref<NFClass>,
) -> Result<metamodelica::Ref<NFClass>> {
    let mut cls: metamodelica::Ref<NFClass> = cls;
    cls = (match &*cls {
        INSTANCED_CLASS {
            elements: __cls_elements,
            prefixes: __cls_prefixes,
            restriction: __cls_restriction,
            ty: __cls_ty,
            ..
        } => metamodelica::Ref::new(NFClass::INSTANCED_CLASS {
            ty: __cls_ty.clone(),
            elements: __cls_elements.clone(),
            sections: sections,
            prefixes: __cls_prefixes.clone(),
            restriction: __cls_restriction.clone(),
        }),
        TYPED_DERIVED {
            baseClass: __cls_baseClass,
            ..
        } => {
            NFInstNode::InstNode::classApply(__cls_baseClass.clone(), &setSections, sections)?;
            cls
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cls)
}

pub(crate) fn lookupElement(
    mut name: ArcStr,
    mut cls: metamodelica::Ref<NFClass>,
) -> Result<(metamodelica::Ref<InstNode::InstNode>, bool)> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut isImport: bool;
    (node, isImport) = ClassTree::lookupElement(name, &(classTree(cls)?))?;
    Ok((node, isImport))
}

pub(crate) fn tryLookupElement(
    mut name: ArcStr,
    mut cls: metamodelica::Ref<NFClass>,
) -> (Option<metamodelica::Ref<InstNode::InstNode>>, bool) {
    let mut node: Option<metamodelica::Ref<InstNode::InstNode>>;
    let mut isImport: bool;
    let mut n: metamodelica::Ref<InstNode::InstNode>;
    match '__try0: {
        (n, isImport) = unwrap_break_err!(ClassTree::lookupElement(name.clone(), &(unwrap_break_err!(classTree(cls.clone()), '__try0))), '__try0);
        node = Some(n.clone());
        Ok::<_, &'static str>((isImport.clone(), node.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            isImport = __try0_o0;
            node = __try0_o1;
        }
        Err(_) => {
            node = None;
            isImport = false;
        }
    }
    (node, isImport)
}

pub(crate) fn lookupComponentIndex(mut name: ArcStr, mut cls: metamodelica::Ref<NFClass>) -> Result<i32> {
    let mut index: i32;
    index = ClassTree::lookupComponentIndex(name, &(classTree(cls)?))?;
    Ok(index)
}

pub(crate) fn nthComponent(
    mut index: i32,
    mut cls: metamodelica::Ref<NFClass>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut component: metamodelica::Ref<InstNode::InstNode>;
    component = ClassTree::nthComponent(index, &(classTree(cls)?))?;
    Ok(component)
}

pub fn getComponents(
    mut cls: metamodelica::Ref<NFClass>,
) -> Result<metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>> {
    let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>> =
        ClassTree::getComponents(&(classTree(cls.clone())?))?;
    Ok(comps)
}

pub(crate) fn lookupAttributeBinding(
    mut name: ArcStr,
    mut cls: metamodelica::Ref<NFClass>,
) -> metamodelica::Ref<Binding::NFBinding> {
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut attr_node: metamodelica::Ref<InstNode::InstNode>;
    match '__try0: {
        (attr_node, _) = unwrap_break_err!(ClassTree::lookupElement(name.clone(), &(unwrap_break_err!(classTree(cls.clone()), '__try0))), '__try0);
        binding = Component::getBinding(&(unwrap_break_err!(NFInstNode::InstNode::component(&attr_node), '__try0)));
        Ok::<_, &'static str>((binding.clone(),))
    } {
        Ok((__try0_o0,)) => {
            binding = __try0_o0;
        }
        Err(_) => {
            binding = Binding::EMPTY_BINDING().clone();
        }
    }
    binding
}

pub(crate) fn lookupAttributeValue(
    mut name: ArcStr,
    mut cls: metamodelica::Ref<NFClass>,
) -> Option<metamodelica::Ref<Expression::NFExpression>> {
    let mut value: Option<metamodelica::Ref<Expression::NFExpression>> =
        Binding::typedExp(&(lookupAttributeBinding(name.clone(), cls.clone())));
    value
}

pub fn isOnlyBuiltin(mut cls: &metamodelica::Ref<NFClass>) -> bool {
    let mut builtin: bool;
    builtin = (match &**cls {
        PARTIAL_BUILTIN { .. } => true,
        INSTANCED_BUILTIN { .. } => true,
        _ => false,
    });
    builtin
}

pub(crate) fn isBuiltin(mut cls: metamodelica::Ref<NFClass>) -> Result<bool> {
    '__tco: loop {
        match &*cls {
            PARTIAL_BUILTIN { .. } => return Ok(true),
            INSTANCED_BUILTIN { .. } => return Ok(true),
            EXPANDED_DERIVED {
                baseClass: __cls_baseClass,
                ..
            } => {
                cls = NFInstNode::InstNode::getClass(__cls_baseClass.clone())?;
                continue '__tco;
            }
            TYPED_DERIVED {
                baseClass: __cls_baseClass,
                ..
            } => {
                cls = NFInstNode::InstNode::getClass(__cls_baseClass.clone())?;
                continue '__tco;
            }
            _ => return Ok(false),
        }
    }
}

pub fn classTree(mut cls: metamodelica::Ref<NFClass>) -> Result<metamodelica::Ref<ClassTree::ClassTree>> {
    '__tco: loop {
        match &*cls {
            PARTIAL_CLASS {
                elements: __cls_elements,
                ..
            } => return Ok(__cls_elements.clone()),
            PARTIAL_BUILTIN {
                elements: __cls_elements,
                ..
            } => return Ok(__cls_elements.clone()),
            EXPANDED_CLASS {
                elements: __cls_elements,
                ..
            } => return Ok(__cls_elements.clone()),
            EXPANDED_DERIVED {
                baseClass: __cls_baseClass,
                ..
            } => {
                cls = NFInstNode::InstNode::getClass(__cls_baseClass.clone())?;
                continue '__tco;
            }
            INSTANCED_CLASS {
                elements: __cls_elements,
                ..
            } => return Ok(__cls_elements.clone()),
            INSTANCED_BUILTIN {
                elements: __cls_elements,
                ..
            } => return Ok(__cls_elements.clone()),
            TYPED_DERIVED {
                baseClass: __cls_baseClass,
                ..
            } => {
                cls = NFInstNode::InstNode::getClass(__cls_baseClass.clone())?;
                continue '__tco;
            }
            _ => return Ok(crate::NFClassTree::ClassTree::interned_EMPTY_TREE()),
        }
    }
}

pub(crate) fn setClassTree(
    mut tree: metamodelica::Ref<ClassTree::ClassTree>,
    mut cls: metamodelica::Ref<NFClass>,
) -> Result<metamodelica::Ref<NFClass>> {
    let mut cls: metamodelica::Ref<NFClass> = cls;
    let () = (match &*cls {
        PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; elements = tree);
            ()
        }
        EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; elements = tree);
            ()
        }
        PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; elements = tree);
            ()
        }
        INSTANCED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; elements = tree);
            ()
        }
        INSTANCED_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_BUILTIN; elements = tree);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cls)
}

pub(crate) fn classTreeApply(
    mut cls: metamodelica::Ref<NFClass>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<ClassTree::ClassTree>) -> Result<metamodelica::Ref<ClassTree::ClassTree>>,
) -> Result<metamodelica::Ref<NFClass>> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<ClassTree::ClassTree>) -> Result<metamodelica::Ref<ClassTree::ClassTree>>
            + 'static,
    >;

    let mut cls: metamodelica::Ref<NFClass> = cls;
    let () = (match &*cls {
        PARTIAL_CLASS {
            elements: __cls_elements,
            ..
        } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; elements = func(__cls_elements.clone())?);
            ()
        }
        EXPANDED_CLASS {
            elements: __cls_elements,
            ..
        } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; elements = func(__cls_elements.clone())?);
            ()
        }
        PARTIAL_BUILTIN {
            elements: __cls_elements,
            ..
        } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; elements = func(__cls_elements.clone())?);
            ()
        }
        INSTANCED_CLASS {
            elements: __cls_elements,
            ..
        } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; elements = func(__cls_elements.clone())?);
            ()
        }
        INSTANCED_BUILTIN {
            elements: __cls_elements,
            ..
        } => {
            assign_variant_field!(cls => NFClass::INSTANCED_BUILTIN; elements = func(__cls_elements.clone())?);
            ()
        }
        _ => (),
    });
    Ok(cls)
}

pub(crate) fn getModifier(mut cls: &metamodelica::Ref<NFClass>) -> metamodelica::Ref<Modifier::Modifier> {
    let mut modifier: metamodelica::Ref<Modifier::Modifier>;
    modifier = (match &**cls {
        PARTIAL_CLASS {
            modifier: __cls_modifier,
            ..
        } => __cls_modifier.clone(),
        EXPANDED_CLASS {
            modifier: __cls_modifier,
            ..
        } => __cls_modifier.clone(),
        EXPANDED_DERIVED {
            modifier: __cls_modifier,
            ..
        } => __cls_modifier.clone(),
        PARTIAL_BUILTIN {
            modifier: __cls_modifier,
            ..
        } => __cls_modifier.clone(),
        _ => crate::NFModifier::Modifier::interned_NOMOD(),
    });
    modifier
}

pub(crate) fn getCCModifier(mut cls: &metamodelica::Ref<NFClass>) -> metamodelica::Ref<Modifier::Modifier> {
    let mut modifier: metamodelica::Ref<Modifier::Modifier>;
    modifier = (match &**cls {
        PARTIAL_CLASS { ccMod: __cls_ccMod, .. } => __cls_ccMod.clone(),
        EXPANDED_CLASS { ccMod: __cls_ccMod, .. } => __cls_ccMod.clone(),
        EXPANDED_DERIVED { ccMod: __cls_ccMod, .. } => __cls_ccMod.clone(),
        _ => crate::NFModifier::Modifier::interned_NOMOD(),
    });
    modifier
}

pub(crate) fn setModifier(
    mut modifier: metamodelica::Ref<Modifier::Modifier>,
    mut cls: metamodelica::Ref<NFClass>,
) -> Result<metamodelica::Ref<NFClass>> {
    let mut cls: metamodelica::Ref<NFClass> = cls;
    let () = (match &*cls {
        PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; modifier = modifier);
            ()
        }
        EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; modifier = modifier);
            ()
        }
        EXPANDED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_DERIVED; modifier = modifier);
            ()
        }
        PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; modifier = modifier);
            ()
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFClass.setModifier"));
                    __mm_s.push_str(&*literal!(" got non-modifiable class"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFClass.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(cls)
}

pub(crate) fn mergeModifier(
    mut modifier: metamodelica::Ref<Modifier::Modifier>,
    mut cls: metamodelica::Ref<NFClass>,
) -> Result<metamodelica::Ref<NFClass>> {
    let mut cls: metamodelica::Ref<NFClass> = cls;
    let () = (match &*cls {
        PARTIAL_CLASS {
            modifier: __cls_modifier,
            ..
        } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; modifier = Modifier::merge(modifier, __cls_modifier.clone(), &(literal!("")))?);
            ()
        }
        EXPANDED_CLASS {
            modifier: __cls_modifier,
            ..
        } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; modifier = Modifier::merge(modifier, __cls_modifier.clone(), &(literal!("")))?);
            ()
        }
        EXPANDED_DERIVED {
            modifier: __cls_modifier,
            ..
        } => {
            assign_variant_field!(cls => NFClass::EXPANDED_DERIVED; modifier = Modifier::merge(modifier, __cls_modifier.clone(), &(literal!("")))?);
            ()
        }
        PARTIAL_BUILTIN {
            modifier: __cls_modifier,
            ..
        } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; modifier = Modifier::merge(modifier, __cls_modifier.clone(), &(literal!("")))?);
            ()
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFClass.mergeModifier"));
                    __mm_s.push_str(&*literal!(" got non-modifiable class"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFClass.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(cls)
}

pub(crate) fn isIdentical(
    mut cls1: &metamodelica::Ref<NFClass>,
    mut cls2: &metamodelica::Ref<NFClass>,
) -> Result<bool> {
    let mut identical: bool = false;
    if referenceEq(&*(&**cls1), &*(&**cls2)) {
        identical = true;
    } else {
        identical = (::match_deref::match_deref! { match (cls1, cls2) {
            (Deref @ EXPANDED_CLASS { .. }, Deref @ EXPANDED_CLASS { .. }) => Prefixes::isEqual(var_field!((**cls1).prefixes, NFClass::EXPANDED_CLASS).clone(), var_field!((**cls2).prefixes, NFClass::EXPANDED_CLASS).clone()) && ClassTree::isIdentical(var_field!((**cls1).elements, NFClass::EXPANDED_CLASS), var_field!((**cls2).elements, NFClass::EXPANDED_CLASS)),
            (Deref @ INSTANCED_BUILTIN { .. }, Deref @ INSTANCED_BUILTIN { .. }) => {
                if !(Type::isEqual(var_field!((**cls1).ty, NFClass::INSTANCED_BUILTIN), var_field!((**cls2).ty, NFClass::INSTANCED_BUILTIN))?) {
                    return Ok(identical);
                }
                true
            },
            _ => true,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(identical)
}

pub(crate) fn hasDimensions(mut cls: &metamodelica::Ref<NFClass>) -> Result<bool> {
    let mut hasDims: bool;
    hasDims = (match &**cls {
        EXPANDED_DERIVED {
            baseClass: __cls_baseClass,
            ..
        } => {
            metamodelica::arrayLength(var_field!((**cls).dims, NFClass::EXPANDED_DERIVED).clone()) > 0
                || hasDimensions(&(NFInstNode::InstNode::getClass(__cls_baseClass.clone())?))?
        }
        TYPED_DERIVED { ty: __cls_ty, .. } => Type::isArray(metamodelica::AsArg::as_arg(&__cls_ty)),
        _ => false,
    });
    Ok(hasDims)
}

pub(crate) fn getDimensions(
    mut cls: &metamodelica::Ref<NFClass>,
) -> metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    dims = (match &**cls {
        INSTANCED_CLASS { ty: __cls_ty, .. } => Type::arrayDims(__cls_ty.clone()),
        INSTANCED_BUILTIN { ty: __cls_ty, .. } => Type::arrayDims(__cls_ty.clone()),
        TYPED_DERIVED { ty: __cls_ty, .. } => Type::arrayDims(__cls_ty.clone()),
        _ => metamodelica::nil(),
    });
    dims
}

pub(crate) fn dimensionCount(mut cls: &metamodelica::Ref<NFClass>) -> i32 {
    let mut count: i32;
    count = (match &**cls {
        EXPANDED_DERIVED { .. } => {
            metamodelica::arrayLength(var_field!((**cls).dims, NFClass::EXPANDED_DERIVED).clone())
        }
        INSTANCED_CLASS { ty: __cls_ty, .. } => Type::dimensionCount(__cls_ty.clone()),
        INSTANCED_BUILTIN { ty: __cls_ty, .. } => Type::dimensionCount(__cls_ty.clone()),
        TYPED_DERIVED { ty: __cls_ty, .. } => Type::dimensionCount(__cls_ty.clone()),
        _ => 0,
    });
    count
}

pub(crate) fn getAttributes(mut cls: &metamodelica::Ref<NFClass>) -> metamodelica::Ref<Attributes::NFAttributes> {
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    attr = (match &**cls {
        EXPANDED_DERIVED {
            attributes: __cls_attributes,
            ..
        } => __cls_attributes.clone(),
        _ => Attributes::DEFAULT_ATTR().clone(),
    });
    attr
}

pub fn getTypeAttributes(
    mut cls: metamodelica::Ref<NFClass>,
) -> metamodelica::List<metamodelica::Ref<Modifier::Modifier>> {
    let mut attributes: metamodelica::List<metamodelica::Ref<Modifier::Modifier>> = metamodelica::nil();
    let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut r#mod: metamodelica::Ref<Modifier::Modifier>;
    if '__try0: {
        comps =
            unwrap_break_err!(ClassTree::getComponents(&(unwrap_break_err!(classTree(cls.clone()), '__try0))), '__try0);
        let __range1 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut c in __range1 {
            r#mod = Component::getModifier(&(unwrap_break_err!(NFInstNode::InstNode::component(&c), '__try0)));
            if !(Modifier::isEmpty(&r#mod)) {
                attributes = metamodelica::cons(r#mod.clone(), attributes.clone());
            }
        }
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    attributes
}

pub(crate) fn getType(
    mut cls: metamodelica::Ref<NFClass>,
    mut clsNode: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<Type::NFType>> {
    '__tco: loop {
        match &*cls {
            PARTIAL_BUILTIN { ty: __cls_ty, .. } => return Ok(__cls_ty.clone()),
            EXPANDED_DERIVED {
                baseClass: __cls_baseClass,
                ..
            } => {
                (cls, clsNode) = (
                    NFInstNode::InstNode::getClass(__cls_baseClass.clone())?,
                    __cls_baseClass.clone(),
                );
                continue '__tco;
            }
            INSTANCED_CLASS { ty: __cls_ty, .. } => return Ok(__cls_ty.clone()),
            INSTANCED_BUILTIN { ty: __cls_ty, .. } => return Ok(__cls_ty.clone()),
            TYPED_DERIVED { ty: __cls_ty, .. } => return Ok(__cls_ty.clone()),
            _ => return Ok(crate::NFType::interned_UNKNOWN()),
        }
    }
}

pub(crate) fn setType(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut cls: metamodelica::Ref<NFClass>,
) -> Result<metamodelica::Ref<NFClass>> {
    let mut cls: metamodelica::Ref<NFClass> = cls;
    let () = (match &*cls {
        PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; ty = ty);
            ()
        }
        EXPANDED_DERIVED {
            baseClass: __cls_baseClass,
            ..
        } => {
            NFInstNode::InstNode::classApply(__cls_baseClass.clone(), &setType, ty)?;
            ()
        }
        INSTANCED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; ty = ty);
            ()
        }
        INSTANCED_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_BUILTIN; ty = ty);
            ()
        }
        TYPED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::TYPED_DERIVED; ty = ty);
            ()
        }
        _ => (),
    });
    Ok(cls)
}

pub fn restriction(mut cls: &metamodelica::Ref<NFClass>) -> metamodelica::Ref<Restriction::NFRestriction> {
    let mut res: metamodelica::Ref<Restriction::NFRestriction>;
    res = (match &**cls {
        PARTIAL_BUILTIN {
            restriction: __cls_restriction,
            ..
        } => __cls_restriction.clone(),
        EXPANDED_CLASS {
            restriction: __cls_restriction,
            ..
        } => __cls_restriction.clone(),
        EXPANDED_DERIVED {
            restriction: __cls_restriction,
            ..
        } => __cls_restriction.clone(),
        INSTANCED_CLASS {
            restriction: __cls_restriction,
            ..
        } => __cls_restriction.clone(),
        INSTANCED_BUILTIN {
            restriction: __cls_restriction,
            ..
        } => __cls_restriction.clone(),
        TYPED_DERIVED {
            restriction: __cls_restriction,
            ..
        } => __cls_restriction.clone(),
        _ => crate::NFRestriction::interned_UNKNOWN(),
    });
    res
}

pub(crate) fn setRestriction(
    mut res: metamodelica::Ref<Restriction::NFRestriction>,
    mut cls: metamodelica::Ref<NFClass>,
) -> Result<metamodelica::Ref<NFClass>> {
    let mut cls: metamodelica::Ref<NFClass> = cls;
    let () = (match &*cls {
        EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; restriction = res);
            ()
        }
        EXPANDED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_DERIVED; restriction = res);
            ()
        }
        INSTANCED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; restriction = res);
            ()
        }
        INSTANCED_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_BUILTIN; restriction = res);
            ()
        }
        TYPED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::TYPED_DERIVED; restriction = res);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cls)
}

pub(crate) fn isConnectorClass(mut cls: &metamodelica::Ref<NFClass>) -> bool {
    let mut isConnector: bool = Restriction::isConnector(&(restriction(cls)));
    isConnector
}

pub(crate) fn isNonexpandableConnectorClass(mut cls: &metamodelica::Ref<NFClass>) -> bool {
    let mut isConnector: bool = Restriction::isNonexpandableConnector(&(restriction(cls)));
    isConnector
}

pub(crate) fn isExpandableConnectorClass(mut cls: &metamodelica::Ref<NFClass>) -> bool {
    let mut isConnector: bool = Restriction::isExpandableConnector(&(restriction(cls)));
    isConnector
}

pub(crate) fn isExternalObject(mut cls: &metamodelica::Ref<NFClass>) -> bool {
    let mut isExternalObject: bool = Restriction::isExternalObject(&(restriction(cls)));
    isExternalObject
}

pub(crate) fn isFunction(mut cls: &metamodelica::Ref<NFClass>) -> bool {
    let mut isFunction: bool = Restriction::isFunction(&(restriction(cls)));
    isFunction
}

pub fn isEnumeration(mut cls: metamodelica::Ref<NFClass>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(cls) {
            Deref @ PARTIAL_BUILTIN { ty: Deref @ Type::ENUMERATION { .. }, .. } => return Ok(true),
            Deref @ INSTANCED_BUILTIN { ty: Deref @ Type::ENUMERATION { .. }, .. } => return Ok(true),
            Deref @ EXPANDED_DERIVED { baseClass: __cls_baseClass, .. } => { cls = NFInstNode::InstNode::getClass(__cls_baseClass.clone())?; continue '__tco; },
            Deref @ TYPED_DERIVED { baseClass: __cls_baseClass, .. } => { cls = NFInstNode::InstNode::getClass(__cls_baseClass.clone())?; continue '__tco; },
            _ => return Ok(false),
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn isExternalFunction(mut cls: metamodelica::Ref<NFClass>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(cls) {
            Deref @ EXPANDED_DERIVED { baseClass: __cls_baseClass, .. } => {
                { cls = NFInstNode::InstNode::getClass(__cls_baseClass.clone())?; continue '__tco; }
            },
            Deref @ INSTANCED_CLASS { sections: Deref @ Sections::EXTERNAL { language: lang, .. }, .. } => {
                return Ok(!metamodelica::stringEq(&lang, &(literal!("builtin"))))
            },
            Deref @ TYPED_DERIVED { baseClass: __cls_baseClass, .. } => {
                { cls = NFInstNode::InstNode::getClass(__cls_baseClass.clone())?; continue '__tco; }
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn isOverdetermined(mut cls: metamodelica::Ref<NFClass>) -> bool {
    let mut isOverdetermined: bool;
    match '__try0: {
        unwrap_break_err!(lookupElement(literal!("equalityConstraint"), cls.clone()), '__try0);
        System::setHasOverconstrainedConnectors(true);
        isOverdetermined = true;
        Ok::<_, &'static str>((isOverdetermined.clone(),))
    } {
        Ok((__try0_o0,)) => {
            isOverdetermined = __try0_o0;
        }
        Err(_) => {
            isOverdetermined = false;
        }
    }
    isOverdetermined
}

pub(crate) fn getPrefixes(mut cls: metamodelica::Ref<NFClass>) -> Result<metamodelica::Ref<Prefixes::Prefixes>> {
    '__tco: loop {
        match &*cls {
            PARTIAL_CLASS {
                prefixes: __cls_prefixes,
                ..
            } => return Ok(__cls_prefixes.clone()),
            PARTIAL_BUILTIN {
                prefixes: __cls_prefixes,
                ..
            } => return Ok(__cls_prefixes.clone()),
            EXPANDED_CLASS {
                prefixes: __cls_prefixes,
                ..
            } => return Ok(__cls_prefixes.clone()),
            EXPANDED_DERIVED {
                prefixes: __cls_prefixes,
                ..
            } => return Ok(__cls_prefixes.clone()),
            INSTANCED_CLASS {
                prefixes: __cls_prefixes,
                ..
            } => return Ok(__cls_prefixes.clone()),
            TYPED_DERIVED {
                baseClass: __cls_baseClass,
                ..
            } => {
                cls = NFInstNode::InstNode::getClass(__cls_baseClass.clone())?;
                continue '__tco;
            }
            _ => return Ok(DEFAULT_PREFIXES.clone()),
        }
    }
}

pub(crate) fn setPrefixes(
    mut prefs: metamodelica::Ref<Prefixes::Prefixes>,
    mut cls: metamodelica::Ref<NFClass>,
) -> Result<metamodelica::Ref<NFClass>> {
    let mut cls: metamodelica::Ref<NFClass> = cls;
    let () = (match &*cls {
        PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; prefixes = prefs);
            ()
        }
        PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; prefixes = prefs);
            ()
        }
        EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; prefixes = prefs);
            ()
        }
        EXPANDED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_DERIVED; prefixes = prefs);
            ()
        }
        INSTANCED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; prefixes = prefs);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cls)
}

pub(crate) fn isEncapsulated(mut cls: metamodelica::Ref<NFClass>) -> Result<bool> {
    let mut isEncapsulated: bool = Prefixes::isEncapsulated(&(getPrefixes(cls.clone())?));
    Ok(isEncapsulated)
}

pub(crate) fn isPartial(mut cls: metamodelica::Ref<NFClass>) -> Result<bool> {
    let mut isPartial: bool = Prefixes::isPartial(&(getPrefixes(cls.clone())?));
    Ok(isPartial)
}

pub(crate) fn lastBaseClass(
    mut node: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    '__tco: loop {
        let mut cls: metamodelica::Ref<NFClass> = NFInstNode::InstNode::getClass(node.clone())?;
        match &*cls {
            EXPANDED_DERIVED {
                baseClass: __cls_baseClass,
                ..
            } => {
                node = __cls_baseClass.clone();
                continue '__tco;
            }
            TYPED_DERIVED {
                baseClass: __cls_baseClass,
                ..
            } => {
                node = __cls_baseClass.clone();
                continue '__tco;
            }
            _ => return Ok(node),
        }
    }
}

pub(crate) fn getDerivedComments(
    mut cls: metamodelica::Ref<NFClass>,
    mut cmts: metamodelica::List<metamodelica::Ref<SCode::Comment>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Comment>>> {
    let mut cmts: metamodelica::List<metamodelica::Ref<SCode::Comment>> = cmts;
    cmts = (match &*cls {
        EXPANDED_DERIVED {
            baseClass: __cls_baseClass,
            ..
        } => NFInstNode::InstNode::getComments(metamodelica::AsArg::as_arg(&__cls_baseClass), cmts)?,
        TYPED_DERIVED {
            baseClass: __cls_baseClass,
            ..
        } => NFInstNode::InstNode::getComments(metamodelica::AsArg::as_arg(&__cls_baseClass), cmts)?,
        _ => {
            let __range0 = ClassTree::getExtends(&(classTree(cls)?))
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut ext in __range0 {
                cmts = NFInstNode::InstNode::getComments(&ext, cmts)?;
            }
            cmts
        }
    });
    Ok(cmts)
}

pub fn constrainingClassPath(
    mut clsNode: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode> = lastBaseClass(clsNode.clone())?;
    let mut prefs: metamodelica::Ref<Prefixes::Prefixes> =
        getPrefixes(NFInstNode::InstNode::getClass(cls_node.clone())?)?;
    path = (::match_deref::match_deref! { match &(prefs) {
        Deref @ Prefixes::PREFIXES { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { constrainingClass: __esc_path, .. }) }, .. } => {
            path = (*__esc_path).clone();
            path.clone()
        },
        _ => NFInstNode::InstNode::enclosingScopePath(cls_node, false, false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(path)
}

pub fn hasOperator(mut name: ArcStr, mut cls: metamodelica::Ref<NFClass>) -> bool {
    let mut hasOperator: bool;
    let mut op_node: metamodelica::Ref<InstNode::InstNode>;
    if Restriction::isOperatorRecord(&(restriction(&cls))) {
        match '__try0: {
            (op_node, _) = unwrap_break_err!(lookupElement(name.clone(), cls.clone()), '__try0);
            hasOperator =
                SCodeUtil::isOperator(&(unwrap_break_err!(NFInstNode::InstNode::definition(op_node.clone()), '__try0)));
            Ok::<_, &'static str>((hasOperator.clone(),))
        } {
            Ok((__try0_o0,)) => {
                hasOperator = __try0_o0;
            }
            Err(_) => {
                hasOperator = false;
            }
        }
    } else {
        hasOperator = false;
    }
    hasOperator
}

pub(crate) fn makeRecordExp(
    mut clsNode: metamodelica::Ref<InstNode::InstNode>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut typed: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut cls: metamodelica::Ref<NFClass>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut ty_node: metamodelica::Ref<InstNode::InstNode>;
    let mut ty_cell: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut fields: metamodelica::List<metamodelica::Ref<Record::Field::Field>>;
    let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    cls = NFInstNode::InstNode::getClass(clsNode.clone())?;
    let (__pa1, __pa0) = ::match_deref::match_deref! { match &(getType(cls.clone(), clsNode)?) {
        __pa1 @ Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: __pa0, .. }, .. } => (__pa1.clone(), __pa0.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty_cell = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    ty_node = NFInstNode::InstNode::borrow(ty_cell)?;
    comps = ClassTree::getComponents(&(classTree(cls)?))?;
    if typed {
        args = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut c in (comps.clone()).borrow().iter() {
                let __x = Binding::getExp(
                    &(Component::getImplicitBinding(&(NFInstNode::InstNode::component(&(c.clone()))?), scope.clone())?),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        exp = Expression::makeRecord(NFInstNode::InstNode::fullPath(ty_node, false)?, ty, args);
    } else {
        args = metamodelica::nil();
        let __range3 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut c in __range3 {
            fields = Record::collectRecordField(c.clone(), metamodelica::nil())?;
            if !((fields).is_empty()) && Record::Field::isInput(&((fields).head().cloned()?)) {
                args = metamodelica::cons(
                    Binding::getExp(
                        &(Component::getImplicitBinding(&(NFInstNode::InstNode::component(&c)?), scope.clone())?),
                    )?,
                    args,
                );
            }
        }
        args = metamodelica::Dangerous::listReverseInPlace(args);
        exp = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: metamodelica::Ref::new(Call::NFCall::UNTYPED_CALL {
                r#ref: ComponentRef::fromNode(ty_node, ty, metamodelica::nil(), ComponentRef::Origin::CREF.clone())?,
                arguments: args,
                named_args: metamodelica::nil(),
                call_scope: NFInstNode::InstNode::scopeRef(scope),
            }),
        });
    }
    Ok(exp)
}

pub(crate) fn toFlatStream(
    mut cls: &metamodelica::Ref<NFClass>,
    mut clsNode: metamodelica::Ref<InstNode::InstNode>,
    mut format: BaseModelica::OutputFormat,
    mut indent: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut name: ArcStr;
    name = Util::makeQuotedIdentifier(AbsynUtil::pathString(
        NFInstNode::InstNode::scopePath(clsNode, NFInstNode::InstNode::ScopeType::RELATIVE.clone(), false)?,
        literal!("."),
        true,
        false,
    )?)?;
    s = (match &**cls {
        INSTANCED_CLASS {
            elements: __cls_elements,
            restriction: __cls_restriction,
            ..
        } => {
            s = IOStream::append(s, indent.clone())?;
            s = IOStream::append(
                s,
                Restriction::toString(metamodelica::AsArg::as_arg(&__cls_restriction)),
            )?;
            s = IOStream::append(s, literal!(" "))?;
            s = IOStream::append(s, name.clone())?;
            s = IOStream::append(s, literal!("\n"))?;
            let __range0 = ClassTree::getComponents(metamodelica::AsArg::as_arg(&__cls_elements))?
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut comp in __range0 {
                s = IOStream::append(
                    s,
                    NFInstNode::InstNode::toFlatString(comp, format, {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*indent);
                        __mm_s.push_str(&*literal!("  "));
                        ArcStr::from(__mm_s)
                    })?,
                )?;
                s = IOStream::append(s, literal!(";\n"))?;
            }
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("end "))?;
            s = IOStream::append(s, name)?;
            s
        }
        INSTANCED_BUILTIN { .. } => {
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("INSTANCED_BUILTIN("))?;
            s = IOStream::append(s, name)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        TYPED_DERIVED {
            baseClass: __cls_baseClass,
            restriction: __cls_restriction,
            ..
        } => {
            s = IOStream::append(s, indent)?;
            s = IOStream::append(
                s,
                Restriction::toString(metamodelica::AsArg::as_arg(&__cls_restriction)),
            )?;
            s = IOStream::append(s, literal!(" "))?;
            s = IOStream::append(s, name)?;
            s = IOStream::append(s, literal!(" = "))?;
            s = IOStream::append(
                s,
                Util::makeQuotedIdentifier(AbsynUtil::pathString(
                    NFInstNode::InstNode::scopePath(
                        __cls_baseClass.clone(),
                        NFInstNode::InstNode::ScopeType::RELATIVE.clone(),
                        false,
                    )?,
                    literal!("."),
                    true,
                    false,
                )?)?,
            )?;
            s
        }
        _ => IOStream::append(s, {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("UNKNOWN_CLASS("));
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        })?,
    });
    Ok(s)
}

pub fn toFlatString(
    mut cls: &metamodelica::Ref<NFClass>,
    mut clsNode: metamodelica::Ref<InstNode::InstNode>,
    mut format: BaseModelica::OutputFormat,
    mut indent: ArcStr,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: IOStream::IOStream;
    s = IOStream::create(
        literal!("NFClass.toFlatString"),
        openmodelica_util::IOStream::IOStreamType::LIST,
    )?;
    s = toFlatStream(cls, clsNode, format, indent, s)?;
    r#str = IOStream::string(&s)?;
    IOStream::delete(&s)?;
    Ok(r#str)
}
