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
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::*;
use crate::NFRestriction as Restriction;
use crate::NFType;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::Util;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NFAttributes {
    pub connectorType: i32,
    pub parallelism: Prefixes::Parallelism,
    pub variability: Prefixes::Variability,
    pub direction: Prefixes::Direction,
    pub innerOuter: Prefixes::InnerOuter,
    pub isFinal: bool,
    pub isRedeclare: bool,
    pub isReplaceable: Prefixes::Replaceable,
    pub isResizable: bool,
}

impl metamodelica::gc::MMTrace for NFAttributes {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.connectorType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.parallelism, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.variability, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.direction, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.innerOuter, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isFinal, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isRedeclare, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isReplaceable, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isResizable, __mmv)?;
        Ok(())
    }
}
impl Default for NFAttributes {
    fn default() -> Self {
        Self {
            connectorType: Default::default(),
            parallelism: Default::default(),
            variability: Default::default(),
            direction: Default::default(),
            innerOuter: Default::default(),
            isFinal: Default::default(),
            isRedeclare: Default::default(),
            isReplaceable: Default::default(),
            isResizable: Default::default(),
        }
    }
}

pub type ATTRIBUTES = NFAttributes;

thread_local! { static __DEFAULT_ATTR_TLS: metamodelica::Ref<NFAttributes> = metamodelica::Ref::new(NFAttributes { connectorType: ConnectorType::NON_CONNECTOR.clone(), parallelism: Parallelism::NON_PARALLEL.clone(), variability: Variability::CONTINUOUS.clone(), direction: Direction::NONE.clone(), innerOuter: InnerOuter::NOT_INNER_OUTER.clone(), isFinal: false, isRedeclare: false, isReplaceable: crate::NFPrefixes::Replaceable::NOT_REPLACEABLE, isResizable: false }); }
pub fn DEFAULT_ATTR() -> metamodelica::Ref<NFAttributes> {
    __DEFAULT_ATTR_TLS.with(|__t| __t.clone())
}

thread_local! { static __INPUT_ATTR_TLS: metamodelica::Ref<NFAttributes> = metamodelica::Ref::new(NFAttributes { connectorType: ConnectorType::NON_CONNECTOR.clone(), parallelism: Parallelism::NON_PARALLEL.clone(), variability: Variability::CONTINUOUS.clone(), direction: Direction::INPUT.clone(), innerOuter: InnerOuter::NOT_INNER_OUTER.clone(), isFinal: false, isRedeclare: false, isReplaceable: crate::NFPrefixes::Replaceable::NOT_REPLACEABLE, isResizable: false }); }
pub(crate) fn INPUT_ATTR() -> metamodelica::Ref<NFAttributes> {
    __INPUT_ATTR_TLS.with(|__t| __t.clone())
}

thread_local! { static __OUTPUT_ATTR_TLS: metamodelica::Ref<NFAttributes> = metamodelica::Ref::new(NFAttributes { connectorType: ConnectorType::NON_CONNECTOR.clone(), parallelism: Parallelism::NON_PARALLEL.clone(), variability: Variability::CONTINUOUS.clone(), direction: Direction::OUTPUT.clone(), innerOuter: InnerOuter::NOT_INNER_OUTER.clone(), isFinal: false, isRedeclare: false, isReplaceable: crate::NFPrefixes::Replaceable::NOT_REPLACEABLE, isResizable: false }); }
pub(crate) fn OUTPUT_ATTR() -> metamodelica::Ref<NFAttributes> {
    __OUTPUT_ATTR_TLS.with(|__t| __t.clone())
}

thread_local! { static __CONSTANT_ATTR_TLS: metamodelica::Ref<NFAttributes> = metamodelica::Ref::new(NFAttributes { connectorType: ConnectorType::NON_CONNECTOR.clone(), parallelism: Parallelism::NON_PARALLEL.clone(), variability: Variability::CONSTANT.clone(), direction: Direction::NONE.clone(), innerOuter: InnerOuter::NOT_INNER_OUTER.clone(), isFinal: false, isRedeclare: false, isReplaceable: crate::NFPrefixes::Replaceable::NOT_REPLACEABLE, isResizable: false }); }
pub(crate) fn CONSTANT_ATTR() -> metamodelica::Ref<NFAttributes> {
    __CONSTANT_ATTR_TLS.with(|__t| __t.clone())
}

thread_local! { static __IMPL_DISCRETE_ATTR_TLS: metamodelica::Ref<NFAttributes> = metamodelica::Ref::new(NFAttributes { connectorType: ConnectorType::NON_CONNECTOR.clone(), parallelism: Parallelism::NON_PARALLEL.clone(), variability: Variability::IMPLICITLY_DISCRETE.clone(), direction: Direction::NONE.clone(), innerOuter: InnerOuter::NOT_INNER_OUTER.clone(), isFinal: false, isRedeclare: false, isReplaceable: crate::NFPrefixes::Replaceable::NOT_REPLACEABLE, isResizable: false }); }
pub fn IMPL_DISCRETE_ATTR() -> metamodelica::Ref<NFAttributes> {
    __IMPL_DISCRETE_ATTR_TLS.with(|__t| __t.clone())
}

thread_local! { static __AUGMENTED_ATTR_TLS: metamodelica::Ref<NFAttributes> = metamodelica::Ref::new(NFAttributes { connectorType: ConnectorType::AUGMENTED.clone(), parallelism: Parallelism::NON_PARALLEL.clone(), variability: Variability::CONTINUOUS.clone(), direction: Direction::NONE.clone(), innerOuter: InnerOuter::NOT_INNER_OUTER.clone(), isFinal: false, isRedeclare: false, isReplaceable: crate::NFPrefixes::Replaceable::NOT_REPLACEABLE, isResizable: false }); }
pub(crate) fn AUGMENTED_ATTR() -> metamodelica::Ref<NFAttributes> {
    __AUGMENTED_ATTR_TLS.with(|__t| __t.clone())
}

pub(crate) fn fromSCode(
    mut compAttr: &SCode::Attributes,
    mut compPrefs: &metamodelica::Ref<SCode::Prefixes>,
) -> metamodelica::Ref<NFAttributes> {
    let mut attributes: metamodelica::Ref<NFAttributes>;
    let mut cty: i32;
    let mut par: Prefixes::Parallelism;
    let mut var: Prefixes::Variability;
    let mut dir: Prefixes::Direction;
    let mut io: Prefixes::InnerOuter;
    let mut fin: bool;
    let mut redecl: bool;
    let mut repl: Prefixes::Replaceable;
    attributes = (::match_deref::match_deref! { match &((compAttr, &**compPrefs)) {
        (SCode::Attributes { connectorType: SCode::ConnectorType::POTENTIAL { .. }, parallelism: SCode::Parallelism::NON_PARALLEL { .. }, variability: SCode::Variability::VAR { .. }, direction: Absyn::Direction::BIDIR { .. }, .. }, Deref @ SCode::Prefixes { redeclarePrefix: SCode::Redeclare::NOT_REDECLARE { .. }, finalPrefix: SCode::Final::NOT_FINAL { .. }, innerOuter: Absyn::InnerOuter::NOT_INNER_OUTER { .. }, replaceablePrefix: Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. }, .. }) => DEFAULT_ATTR().clone(),
        _ => {
            cty = Prefixes::ConnectorType::fromSCode(compAttr.connectorType.clone());
            par = Prefixes::parallelismFromSCode(compAttr.parallelism.clone());
            var = Prefixes::variabilityFromSCode(compAttr.variability.clone());
            dir = Prefixes::directionFromSCode(compAttr.direction.clone());
            io = Prefixes::innerOuterFromSCode(compPrefs.innerOuter.clone());
            fin = SCodeUtil::finalBool(compPrefs.finalPrefix.clone());
            redecl = SCodeUtil::redeclareBool(compPrefs.redeclarePrefix.clone());
            repl = crate::NFPrefixes::Replaceable::NOT_REPLACEABLE;
            metamodelica::Ref::new(NFAttributes { connectorType: cty, parallelism: par, variability: var, direction: dir, innerOuter: io, isFinal: fin, isRedeclare: redecl, isReplaceable: repl, isResizable: false })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    attributes
}

pub(crate) fn fromDerivedSCode(mut scodeAttr: &SCode::Attributes) -> metamodelica::Ref<NFAttributes> {
    let mut attributes: metamodelica::Ref<NFAttributes>;
    let mut cty: i32;
    let mut var: Prefixes::Variability;
    let mut dir: Prefixes::Direction;
    attributes = (match scodeAttr.clone() {
        SCode::Attributes {
            connectorType: SCode::ConnectorType::POTENTIAL { .. },
            variability: SCode::Variability::VAR { .. },
            direction: Absyn::Direction::BIDIR { .. },
            ..
        } => DEFAULT_ATTR().clone(),
        _ => {
            cty = Prefixes::ConnectorType::fromSCode(scodeAttr.connectorType.clone());
            var = Prefixes::variabilityFromSCode(scodeAttr.variability.clone());
            dir = Prefixes::directionFromSCode(scodeAttr.direction.clone());
            metamodelica::Ref::new(NFAttributes {
                connectorType: cty,
                parallelism: Parallelism::NON_PARALLEL.clone(),
                variability: var,
                direction: dir,
                innerOuter: InnerOuter::NOT_INNER_OUTER.clone(),
                isFinal: false,
                isRedeclare: false,
                isReplaceable: crate::NFPrefixes::Replaceable::NOT_REPLACEABLE,
                isResizable: false,
            })
        }
    });
    attributes
}

pub(crate) fn mergeComponentAttributes(
    mut outerAttr: &metamodelica::Ref<NFAttributes>,
    mut innerAttr: metamodelica::Ref<NFAttributes>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut parentRestriction: &metamodelica::Ref<Restriction::NFRestriction>,
) -> Result<metamodelica::Ref<NFAttributes>> {
    let mut attr: metamodelica::Ref<NFAttributes>;
    let mut cty: i32;
    let mut par: Prefixes::Parallelism;
    let mut var: Prefixes::Variability;
    let mut dir: Prefixes::Direction;
    let mut fin: bool;
    let mut redecl: bool;
    let mut resize: bool;
    let mut repl: Prefixes::Replaceable;
    if referenceEq(&*(&**outerAttr), &*(DEFAULT_ATTR().clone())) && innerAttr.connectorType.clone() == 0 {
        attr = innerAttr;
    } else if referenceEq(&*(&*innerAttr), &*(DEFAULT_ATTR().clone())) {
        cty = Prefixes::ConnectorType::merge(
            outerAttr.connectorType.clone(),
            innerAttr.connectorType.clone(),
            node,
            false,
        )?;
        attr = metamodelica::Ref::new(NFAttributes {
            connectorType: cty,
            parallelism: outerAttr.parallelism.clone(),
            variability: outerAttr.variability.clone(),
            direction: outerAttr.direction.clone(),
            innerOuter: innerAttr.innerOuter.clone(),
            isFinal: outerAttr.isFinal.clone(),
            isRedeclare: innerAttr.isRedeclare.clone(),
            isReplaceable: innerAttr.isReplaceable.clone(),
            isResizable: innerAttr.isResizable.clone(),
        });
    } else {
        cty = Prefixes::ConnectorType::merge(
            outerAttr.connectorType.clone(),
            innerAttr.connectorType.clone(),
            node,
            false,
        )?;
        par = Prefixes::mergeParallelism(outerAttr.parallelism.clone(), innerAttr.parallelism.clone(), node)?;
        var = Prefixes::variabilityMin(outerAttr.variability.clone(), innerAttr.variability.clone());
        if Restriction::isFunction(parentRestriction) {
            dir = innerAttr.direction.clone();
        } else {
            dir = Prefixes::mergeDirection(outerAttr.direction.clone(), innerAttr.direction.clone(), node, false)?;
        }
        fin = outerAttr.isFinal.clone() || innerAttr.isFinal.clone();
        redecl = innerAttr.isRedeclare.clone();
        repl = innerAttr.isReplaceable.clone();
        resize = innerAttr.isResizable.clone();
        attr = metamodelica::Ref::new(NFAttributes {
            connectorType: cty,
            parallelism: par,
            variability: var,
            direction: dir,
            innerOuter: innerAttr.innerOuter.clone(),
            isFinal: fin,
            isRedeclare: redecl,
            isReplaceable: repl,
            isResizable: resize,
        });
    }
    Ok(attr)
}

pub(crate) fn mergeDerivedAttributes(
    mut outerAttr: metamodelica::Ref<NFAttributes>,
    mut innerAttr: metamodelica::Ref<NFAttributes>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<NFAttributes>> {
    let mut attr: metamodelica::Ref<NFAttributes>;
    let mut cty: i32;
    let mut par: Prefixes::Parallelism;
    let mut var: Prefixes::Variability;
    let mut dir: Prefixes::Direction;
    let mut io: Prefixes::InnerOuter;
    let mut fin: bool;
    let mut redecl: bool;
    let mut resize: bool;
    let mut repl: Prefixes::Replaceable;
    if referenceEq(&*(&*innerAttr), &*(DEFAULT_ATTR().clone())) && outerAttr.connectorType.clone() == 0 {
        attr = outerAttr;
    } else if referenceEq(&*(&*outerAttr), &*(DEFAULT_ATTR().clone())) && innerAttr.connectorType.clone() == 0 {
        attr = innerAttr;
    } else {
        let __arc9 = outerAttr;
        let ATTRIBUTES {
            connectorType: __pa0,
            parallelism: __pa1,
            variability: __pa2,
            direction: __pa3,
            innerOuter: __pa4,
            isFinal: __pa5,
            isRedeclare: __pa6,
            isReplaceable: __pa7,
            isResizable: __pa8,
        } = &*__arc9;
        cty = metamodelica::Own::own(__pa0);
        par = metamodelica::Own::own(__pa1);
        var = metamodelica::Own::own(__pa2);
        dir = metamodelica::Own::own(__pa3);
        io = metamodelica::Own::own(__pa4);
        fin = metamodelica::Own::own(__pa5);
        redecl = metamodelica::Own::own(__pa6);
        repl = metamodelica::Own::own(__pa7);
        resize = metamodelica::Own::own(__pa8);
        cty = Prefixes::ConnectorType::merge(cty, innerAttr.connectorType.clone(), node, true)?;
        var = Prefixes::variabilityMin(var, innerAttr.variability.clone());
        dir = Prefixes::mergeDirection(dir, innerAttr.direction.clone(), node, true)?;
        attr = metamodelica::Ref::new(NFAttributes {
            connectorType: cty,
            parallelism: par,
            variability: var,
            direction: dir,
            innerOuter: innerAttr.innerOuter.clone(),
            isFinal: fin,
            isRedeclare: redecl,
            isReplaceable: repl,
            isResizable: resize,
        });
    }
    Ok(attr)
}

pub(crate) fn mergeRedeclaredComponentAttributes(
    mut origAttr: metamodelica::Ref<NFAttributes>,
    mut redeclAttr: metamodelica::Ref<NFAttributes>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<NFAttributes>> {
    let mut attr: metamodelica::Ref<NFAttributes>;
    let mut cty: i32;
    let mut rcty: i32;
    let mut cty_fs: i32;
    let mut rcty_fs: i32;
    let mut par: Prefixes::Parallelism;
    let mut rpar: Prefixes::Parallelism;
    let mut var: Prefixes::Variability;
    let mut rvar: Prefixes::Variability;
    let mut dir: Prefixes::Direction;
    let mut rdir: Prefixes::Direction;
    let mut io: Prefixes::InnerOuter;
    let mut rio: Prefixes::InnerOuter;
    let mut fin: bool;
    let mut redecl: bool;
    let mut resize: bool;
    let mut repl: Prefixes::Replaceable;
    if referenceEq(&*(&*origAttr), &*(DEFAULT_ATTR().clone())) {
        attr = redeclAttr;
    } else if referenceEq(&*(&*redeclAttr), &*(DEFAULT_ATTR().clone())) {
        attr = origAttr;
    } else {
        let __arc5 = origAttr;
        let ATTRIBUTES {
            connectorType: __pa0,
            parallelism: __pa1,
            variability: __pa2,
            direction: __pa3,
            innerOuter: __pa4,
            isFinal: _,
            isRedeclare: _,
            isReplaceable: _,
            isResizable: _,
        } = &*__arc5;
        cty = metamodelica::Own::own(__pa0);
        par = metamodelica::Own::own(__pa1);
        var = metamodelica::Own::own(__pa2);
        dir = metamodelica::Own::own(__pa3);
        io = metamodelica::Own::own(__pa4);
        let __arc15 = redeclAttr;
        let ATTRIBUTES {
            connectorType: __pa6,
            parallelism: __pa7,
            variability: __pa8,
            direction: __pa9,
            innerOuter: __pa10,
            isFinal: __pa11,
            isRedeclare: __pa12,
            isReplaceable: __pa13,
            isResizable: __pa14,
        } = &*__arc15;
        rcty = metamodelica::Own::own(__pa6);
        rpar = metamodelica::Own::own(__pa7);
        rvar = metamodelica::Own::own(__pa8);
        rdir = metamodelica::Own::own(__pa9);
        rio = metamodelica::Own::own(__pa10);
        fin = metamodelica::Own::own(__pa11);
        redecl = metamodelica::Own::own(__pa12);
        repl = metamodelica::Own::own(__pa13);
        resize = metamodelica::Own::own(__pa14);
        rcty_fs = intBitAnd(rcty, ConnectorType::FLOW_STREAM_MASK.clone());
        cty_fs = intBitAnd(cty, ConnectorType::FLOW_STREAM_MASK.clone());
        if rcty_fs > 0 {
            if cty_fs > 0 && rcty_fs != cty_fs {
                printRedeclarePrefixError(
                    node,
                    Prefixes::ConnectorType::toString(rcty),
                    Prefixes::ConnectorType::toString(cty),
                )?;
            }
        }
        cty = intBitOr(rcty, cty_fs);
        if rpar != Parallelism::NON_PARALLEL.clone() {
            if par != Parallelism::NON_PARALLEL.clone() && par != rpar {
                printRedeclarePrefixError(
                    node,
                    Prefixes::parallelismString(rpar),
                    Prefixes::parallelismString(par),
                )?;
            }
            par = rpar;
        }
        if rvar != Variability::CONTINUOUS.clone() {
            if rvar > var {
                printRedeclarePrefixError(
                    node,
                    Prefixes::variabilityString(rvar)?,
                    Prefixes::variabilityString(var)?,
                )?;
            }
            var = rvar;
        }
        if rdir != Direction::NONE.clone() {
            if dir != Direction::NONE.clone() && rdir != dir {
                printRedeclarePrefixError(node, Prefixes::directionString(rdir), Prefixes::directionString(dir))?;
            }
            dir = rdir;
        }
        if rio != InnerOuter::NOT_INNER_OUTER.clone() {
            if io != InnerOuter::NOT_INNER_OUTER.clone() && rio != io {
                printRedeclarePrefixError(node, Prefixes::innerOuterString(rio), Prefixes::innerOuterString(io))?;
            }
            io = rio;
        }
        attr = metamodelica::Ref::new(NFAttributes {
            connectorType: cty,
            parallelism: par,
            variability: var,
            direction: dir,
            innerOuter: io,
            isFinal: fin,
            isRedeclare: redecl,
            isReplaceable: repl,
            isResizable: resize,
        });
    }
    Ok(attr)
}

pub(crate) fn mergeRedeclaredClassPrefixes(
    mut origPrefs: &metamodelica::Ref<Class::Prefixes::Prefixes>,
    mut redeclPrefs: metamodelica::Ref<Class::Prefixes::Prefixes>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<Class::Prefixes::Prefixes>> {
    let mut prefs: metamodelica::Ref<Class::Prefixes::Prefixes>;
    let mut enc: SCode::Encapsulated;
    let mut par: SCode::Partial;
    let mut fin: SCode::Final;
    let mut io: Absyn::InnerOuter;
    let mut rio: Absyn::InnerOuter;
    let mut repl: metamodelica::Ref<SCode::Replaceable>;
    if referenceEq(&*(&**origPrefs), &*(Class::DEFAULT_PREFIXES.clone())) {
        prefs = redeclPrefs;
    } else {
        let __arc1 = &(*origPrefs);
        let Class::Prefixes::PREFIXES { innerOuter: __pa0, .. } = &**__arc1;
        io = metamodelica::Own::own(__pa0);
        let __arc7 = redeclPrefs;
        let Class::Prefixes::PREFIXES {
            encapsulatedPrefix: __pa2,
            partialPrefix: __pa3,
            finalPrefix: __pa4,
            innerOuter: __pa5,
            replaceablePrefix: __pa6,
        } = &*__arc7;
        enc = metamodelica::Own::own(__pa2);
        par = metamodelica::Own::own(__pa3);
        fin = metamodelica::Own::own(__pa4);
        rio = metamodelica::Own::own(__pa5);
        repl = metamodelica::Own::own(__pa6);
        io = (match (io, rio) {
            (Absyn::InnerOuter::NOT_INNER_OUTER { .. }, _) => rio,
            (_, Absyn::InnerOuter::NOT_INNER_OUTER { .. }) => io,
            (Absyn::InnerOuter::INNER { .. }, Absyn::InnerOuter::INNER { .. }) => io,
            (Absyn::InnerOuter::OUTER { .. }, Absyn::InnerOuter::OUTER { .. }) => io,
            (Absyn::InnerOuter::INNER_OUTER { .. }, Absyn::InnerOuter::INNER_OUTER { .. }) => io,
            _ => {
                printRedeclarePrefixError(
                    node,
                    Prefixes::innerOuterString(Prefixes::innerOuterFromSCode(rio)),
                    Prefixes::innerOuterString(Prefixes::innerOuterFromSCode(io)),
                )?;
                return Err("fail");
            }
        });
        prefs = metamodelica::Ref::new(Class::Prefixes::Prefixes {
            encapsulatedPrefix: enc,
            partialPrefix: par,
            finalPrefix: fin,
            innerOuter: io,
            replaceablePrefix: repl,
        });
    }
    Ok(prefs)
}

pub(crate) fn printRedeclarePrefixError(
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut prefix1: ArcStr,
    mut prefix2: ArcStr,
) -> Result<()> {
    Error::addSourceMessageAndFail(
        &(Error::REDECLARE_MISMATCHED_PREFIX.clone()),
        list![prefix1, InstNode::name(node)?, prefix2],
        &(InstNode::info(node)),
    )?;
    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    Ok(())
}

pub(crate) fn checkDeclaredComponentAttributes(
    mut attr: metamodelica::Ref<NFAttributes>,
    mut parentRestriction: &metamodelica::Ref<Restriction::NFRestriction>,
    mut component: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<NFAttributes>> {
    let mut attr: metamodelica::Ref<NFAttributes> = attr;
    let () = (match &**parentRestriction {
        Restriction::CONNECTOR {
            isExpandable: __parentRestriction_isExpandable,
        } => {
            assertNotInnerOuter(attr.innerOuter.clone(), component, parentRestriction)?;
            if __parentRestriction_isExpandable.clone() {
                assertNotFlowStream(attr.connectorType.clone(), component, parentRestriction)?;
                assign_field!(
                    attr.connectorType =
                        intBitOr(attr.connectorType.clone(), ConnectorType::POTENTIALLY_PRESENT.clone())
                );
            }
            ()
        }
        Restriction::RECORD { .. } => {
            assertNotInputOutput(attr.direction.clone(), component, parentRestriction)?;
            assertNotInnerOuter(attr.innerOuter.clone(), component, parentRestriction)?;
            assertNotFlowStream(attr.connectorType.clone(), component, parentRestriction)?;
            ()
        }
        _ => (),
    });
    Ok(attr)
}

pub(crate) fn invalidComponentPrefixError(
    mut prefix: ArcStr,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut restriction: &metamodelica::Ref<Restriction::NFRestriction>,
) -> Result<()> {
    Error::addSourceMessage(
        &(Error::INVALID_COMPONENT_PREFIX.clone()),
        list![prefix, InstNode::name(node)?, Restriction::toString(restriction)],
        &(InstNode::info(node)),
    )?;
    Ok(())
}

pub(crate) fn assertNotInputOutput(
    mut dir: Prefixes::Direction,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut restriction: &metamodelica::Ref<Restriction::NFRestriction>,
) -> Result<()> {
    if dir != Direction::NONE.clone() {
        invalidComponentPrefixError(Prefixes::directionString(dir), node, restriction)?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn assertNotInnerOuter(
    mut io: Prefixes::InnerOuter,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut restriction: &metamodelica::Ref<Restriction::NFRestriction>,
) -> Result<()> {
    if io != InnerOuter::NOT_INNER_OUTER.clone() {
        invalidComponentPrefixError(Prefixes::innerOuterString(io), node, restriction)?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn assertNotFlowStream(
    mut cty: i32,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut restriction: &metamodelica::Ref<Restriction::NFRestriction>,
) -> Result<()> {
    if Prefixes::ConnectorType::isFlowOrStream(cty) {
        invalidComponentPrefixError(Prefixes::ConnectorType::toString(cty), node, restriction)?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn updateComponentConnectorType(
    mut attributes: metamodelica::Ref<NFAttributes>,
    mut restriction: &metamodelica::Ref<Restriction::NFRestriction>,
    mut context: i32,
    mut component: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<NFAttributes>> {
    let mut attributes: metamodelica::Ref<NFAttributes> = attributes;
    let mut cty: i32 = attributes.connectorType.clone();
    if Prefixes::ConnectorType::isConnectorType(cty) {
        if Restriction::isConnector(restriction) {
            if attributes.variability.clone() < Variability::DISCRETE.clone()
                && !(InstContext::inRelaxed(context))
                && !(Class::isBuiltin(InstNode::getClass(component.clone())?)?)
            {
                Error::addSourceMessage(
                    &(Error::INVALID_CONNECTOR_VARIABILITY.clone()),
                    list![
                        Prefixes::variabilityString(attributes.variability.clone())?,
                        InstNode::name(&component)?
                    ],
                    &(InstNode::info(&component)),
                )?;
                return Err("fail");
            }
            if Restriction::isExpandableConnector(restriction) {
                cty = Prefixes::ConnectorType::setPresent(cty);
            } else {
                cty = intBitAnd(cty, intBitNot(ConnectorType::EXPANDABLE.clone()));
            }
        } else {
            cty = intBitAnd(
                cty,
                intBitNot(intBitOr(
                    ConnectorType::CONNECTOR.clone(),
                    ConnectorType::EXPANDABLE.clone(),
                )),
            );
        }
        if !(Prefixes::ConnectorType::isFlowOrStream(cty)) {
            cty = Prefixes::ConnectorType::setPotential(cty);
        }
        if cty != attributes.connectorType.clone() {
            assign_field!(attributes.connectorType = cty);
        }
    } else if Prefixes::ConnectorType::isFlowOrStream(cty) && !(InstContext::inRedeclared(context)) {
        Error::addStrictMessage(
            Error::CONNECTOR_PREFIX_OUTSIDE_CONNECTOR.clone(),
            list![Prefixes::ConnectorType::toString(cty)],
            &(InstNode::info(&component)),
        )?;
        assign_field!(attributes.connectorType = Prefixes::ConnectorType::unsetFlowStream(cty));
    }
    Ok(attributes)
}

pub(crate) fn updateClassConnectorType(
    mut res: &metamodelica::Ref<Restriction::NFRestriction>,
    mut attrs: metamodelica::Ref<NFAttributes>,
) -> metamodelica::Ref<NFAttributes> {
    let mut attrs: metamodelica::Ref<NFAttributes> = attrs;
    if Restriction::isExpandableConnector(res) {
        assign_field!(attrs.connectorType = Prefixes::ConnectorType::setExpandable(attrs.connectorType.clone()));
    } else if Restriction::isConnector(res) {
        assign_field!(attrs.connectorType = Prefixes::ConnectorType::setConnector(attrs.connectorType.clone()));
    }
    attrs
}

pub(crate) fn updateVariability(
    mut attr: metamodelica::Ref<NFAttributes>,
    mut cls: &metamodelica::Ref<Class::NFClass>,
    mut clsNode: metamodelica::Ref<InstNode::InstNode>,
    mut compNode: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<NFAttributes>> {
    let mut attr: metamodelica::Ref<NFAttributes> = attr;
    let mut var: Prefixes::Variability = attr.variability.clone();
    if referenceEq(&*(&*attr), &*(DEFAULT_ATTR().clone())) && InstNode::isDiscreteClass(clsNode.clone())? {
        attr = IMPL_DISCRETE_ATTR().clone();
    } else if var == Variability::CONTINUOUS.clone() && InstNode::isDiscreteClass(clsNode)? {
        assign_field!(attr.variability = Variability::IMPLICITLY_DISCRETE.clone());
    } else if var < Variability::CONTINUOUS.clone()
        && InstContext::inFunction(context)
        && attr.direction.clone() != Direction::NONE.clone()
        && SCodeUtil::isEmptyMod(
            &((InstNode::getAnnotation(&(literal!("__OpenModelica_functionVariability")), compNode.clone())?).0),
        )
    {
        assign_field!(attr.variability = Variability::CONTINUOUS.clone());
    } else if var == Variability::PARAMETER.clone()
        && !(Flags::isSet(Flags::NF_SCALARIZE.clone())?)
        && Util::getOptionOrDefault(
            SCodeUtil::lookupBooleanAnnotationMod(
                &((InstNode::getAnnotation(&(literal!("__OpenModelica_resizable")), compNode)?).0),
            ),
            false,
        )
    {
        assign_field!(
            attr.variability = Variability::NON_STRUCTURAL_PARAMETER.clone(),
            attr.isResizable = true
        );
    }
    Ok(attr)
}

pub(crate) fn setConnectorType(
    mut cty: i32,
    mut attr: metamodelica::Ref<NFAttributes>,
) -> metamodelica::Ref<NFAttributes> {
    let mut attr: metamodelica::Ref<NFAttributes> = attr;
    assign_field!(attr.connectorType = cty);
    attr
}

pub(crate) fn setVariability(
    mut var: Prefixes::Variability,
    mut attr: metamodelica::Ref<NFAttributes>,
) -> metamodelica::Ref<NFAttributes> {
    let mut attr: metamodelica::Ref<NFAttributes> = attr;
    assign_field!(attr.variability = var);
    attr
}

pub(crate) fn setDirection(
    mut dir: Prefixes::Direction,
    mut attr: metamodelica::Ref<NFAttributes>,
) -> metamodelica::Ref<NFAttributes> {
    let mut attr: metamodelica::Ref<NFAttributes> = attr;
    assign_field!(attr.direction = dir);
    attr
}

pub(crate) fn setInnerOuter(
    mut io: Prefixes::InnerOuter,
    mut attr: metamodelica::Ref<NFAttributes>,
) -> metamodelica::Ref<NFAttributes> {
    let mut attr: metamodelica::Ref<NFAttributes> = attr;
    assign_field!(attr.innerOuter = io);
    attr
}

pub(crate) fn setFinal(mut fin: bool, mut attr: metamodelica::Ref<NFAttributes>) -> metamodelica::Ref<NFAttributes> {
    let mut attr: metamodelica::Ref<NFAttributes> = attr;
    assign_field!(attr.isFinal = fin);
    attr
}

pub(crate) fn setRedeclare(
    mut redecl: bool,
    mut attr: metamodelica::Ref<NFAttributes>,
) -> metamodelica::Ref<NFAttributes> {
    let mut attr: metamodelica::Ref<NFAttributes> = attr;
    assign_field!(attr.isRedeclare = redecl);
    attr
}

pub(crate) fn setReplaceable(
    mut repl: Prefixes::Replaceable,
    mut attr: metamodelica::Ref<NFAttributes>,
) -> metamodelica::Ref<NFAttributes> {
    let mut attr: metamodelica::Ref<NFAttributes> = attr;
    assign_field!(attr.isReplaceable = repl);
    attr
}

pub(crate) fn toDAE(
    mut ina: &metamodelica::Ref<NFAttributes>,
    mut vis: Prefixes::Visibility,
) -> Result<metamodelica::Ref<DAE::Attributes>> {
    let mut outa: metamodelica::Ref<DAE::Attributes>;
    outa = metamodelica::Ref::new(DAE::Attributes {
        connectorType: Prefixes::ConnectorType::toDAE(ina.connectorType.clone()),
        parallelism: parallelismToSCode(ina.parallelism.clone())?,
        variability: variabilityToSCode(ina.variability.clone()),
        direction: directionToAbsyn(ina.direction.clone()),
        innerOuter: innerOuterToAbsyn(ina.innerOuter.clone())?,
        visibility: visibilityToSCode(vis),
    });
    Ok(outa)
}

pub(crate) fn toString(
    mut attr: &metamodelica::Ref<NFAttributes>,
    mut ty: metamodelica::Ref<NFType::NFType>,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*if (attr.isRedeclare.clone()) {
            literal!("redeclare ")
        } else {
            literal!("")
        });
        __mm_s.push_str(&*if (attr.isFinal.clone()) {
            literal!("final ")
        } else {
            literal!("")
        });
        __mm_s.push_str(&*Prefixes::unparseInnerOuter(attr.innerOuter.clone()));
        __mm_s.push_str(&*Prefixes::unparseReplaceable(&attr.isReplaceable));
        __mm_s.push_str(&*Prefixes::unparseParallelism(attr.parallelism.clone()));
        __mm_s.push_str(&*Prefixes::ConnectorType::unparse(attr.connectorType.clone()));
        __mm_s.push_str(&*Prefixes::unparseVariability(attr.variability.clone(), ty)?);
        __mm_s.push_str(&*Prefixes::unparseDirection(attr.direction.clone()));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn toFlatStream(
    mut attr: &metamodelica::Ref<NFAttributes>,
    mut ty: metamodelica::Ref<NFType::NFType>,
    mut s: IOStream::IOStream,
    mut isTopLevel: bool,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    s = IOStream::append(s, Prefixes::unparseVariability(attr.variability.clone(), ty)?)?;
    if isTopLevel {
        s = IOStream::append(s, Prefixes::unparseDirection(attr.direction.clone()))?;
    }
    Ok(s)
}
