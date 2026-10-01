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
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFComponentRef::Origin;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Variability;
use crate::NFRestriction as Restriction;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::MutableWeak;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NFConnector {
    pub name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    pub ty: metamodelica::Ref<Type::NFType>,
    pub face: Face,
    pub cty: i32,
    pub source: metamodelica::Ref<DAE::ElementSource>,
}

impl metamodelica::gc::MMTrace for NFConnector {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ty, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.face, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.cty, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.source, __mmv)?;
        Ok(())
    }
}
impl Default for NFConnector {
    fn default() -> Self {
        Self {
            name: Default::default(),
            ty: Default::default(),
            face: Default::default(),
            cty: Default::default(),
            source: Default::default(),
        }
    }
}

pub type CONNECTOR = NFConnector;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum Face {
    INSIDE = 1,
    OUTSIDE = 2,
}
impl PartialOrd for Face {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Face {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Face {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for Face {
    fn default() -> Self {
        Self::INSIDE
    }
}

pub(crate) fn fromCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<NFConnector>> {
    let mut conn: metamodelica::Ref<NFConnector> =
        fromFacedCref(cref.clone(), ty.clone(), crefFace(cref.clone())?, source.clone())?;
    Ok(conn)
}

pub(crate) fn fromFacedCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut face: Face,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<NFConnector>> {
    let mut conn: metamodelica::Ref<NFConnector>;
    let mut node: metamodelica::Ref<InstNode::InstNode> = ComponentRef::node(&cref)?;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut cty: i32;
    let mut res: metamodelica::Ref<Restriction::NFRestriction>;
    if NFInstNode::InstNode::isComponent(&node)? {
        comp = NFInstNode::InstNode::component(&node)?;
        res = Class::restriction(&(NFInstNode::InstNode::getClass(Component::classInstance(&comp)?)?));
        cty = Component::connectorType(&comp);
    } else {
        cty = intBitOr(ConnectorType::UNDECLARED.clone(), ConnectorType::POTENTIAL.clone());
    }
    conn = metamodelica::Ref::new(NFConnector {
        name: ComponentRef::simplifySubscripts(cref, false)?,
        ty: ty,
        face: face,
        cty: cty,
        source: source,
    });
    Ok(conn)
}

pub(crate) fn fromExp(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
    mut conns: metamodelica::List<metamodelica::Ref<NFConnector>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFConnector>>> {
    let mut conns: metamodelica::List<metamodelica::Ref<NFConnector>> = conns;
    conns = (match &**exp {
        Expression::CREF {
            cref: __exp_cref,
            ty: __exp_ty,
        } => metamodelica::cons(fromCref(__exp_cref.clone(), __exp_ty.clone(), source.clone())?, conns),
        Expression::ARRAY { .. } => {
            for mut i in ({
                let __s =
                    metamodelica::arrayLength(var_field!((**exp).elements, Expression::NFExpression::ARRAY).clone());
                let __e = 1;
                (0i32..)
                    .map(move |__k| __s + __k * (-1))
                    .take_while(move |&__v| __v >= __e)
            }) {
                conns = fromExp(
                    &(metamodelica::Dangerous::arrayGetNoBoundsChecking(
                        var_field!((**exp).elements, Expression::NFExpression::ARRAY).clone(),
                        i,
                    )),
                    source,
                    conns,
                )?;
            }
            conns
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFConnector.fromExp"));
                    __mm_s.push_str(&*literal!(" got unknown expression "));
                    __mm_s.push_str(&*Expression::toString(exp.clone())?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFConnector.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(conns)
}

pub(crate) fn getType(mut conn: &metamodelica::Ref<NFConnector>) -> metamodelica::Ref<Type::NFType> {
    let mut ty: metamodelica::Ref<Type::NFType> = conn.ty.clone();
    ty
}

pub(crate) fn getInfo(mut conn: &metamodelica::Ref<NFConnector>) -> SourceInfo {
    let mut info: SourceInfo = conn.source.info.clone();
    info
}

pub(crate) fn variability(mut conn: &metamodelica::Ref<NFConnector>) -> Result<Variability> {
    let mut var: Variability =
        Component::variability(&(NFInstNode::InstNode::component(&(ComponentRef::node(&conn.name)?))?))?;
    Ok(var)
}

pub(crate) fn isEqual(
    mut conn1: &metamodelica::Ref<NFConnector>,
    mut conn2: &metamodelica::Ref<NFConnector>,
) -> Result<bool> {
    let mut isEqual: bool =
        ComponentRef::isEqual(&conn1.name, &conn2.name)? && conn1.face.clone() == conn2.face.clone();
    Ok(isEqual)
}

pub(crate) fn isEqualNoSubs(
    mut conn1: &metamodelica::Ref<NFConnector>,
    mut conn2: &metamodelica::Ref<NFConnector>,
) -> Result<bool> {
    let mut isEqual: bool =
        ComponentRef::isEqualStrip(&conn1.name, &conn2.name)? && conn1.face.clone() == conn2.face.clone();
    Ok(isEqual)
}

pub(crate) fn isPrefix(
    mut conn1: &metamodelica::Ref<NFConnector>,
    mut conn2: &metamodelica::Ref<NFConnector>,
) -> Result<bool> {
    let mut isPrefix: bool = ComponentRef::isPrefix(&conn1.name, &conn2.name)?;
    Ok(isPrefix)
}

pub(crate) fn isNodeNameEqual(
    mut conn1: &metamodelica::Ref<NFConnector>,
    mut conn2: &metamodelica::Ref<NFConnector>,
) -> Result<bool> {
    let mut isEqual: bool = metamodelica::stringEq(
        &(ComponentRef::nodeName(&conn1.name)?),
        &(ComponentRef::nodeName(&conn2.name)?),
    );
    Ok(isEqual)
}

pub(crate) fn isOutside(mut conn: &metamodelica::Ref<NFConnector>) -> bool {
    let mut isOutside: bool;
    let mut f: Face = conn.face.clone();
    isOutside = f == Face::OUTSIDE.clone();
    isOutside
}

pub(crate) fn isInside(mut conn: &metamodelica::Ref<NFConnector>) -> bool {
    let mut isInside: bool;
    let mut f: Face = conn.face.clone();
    isInside = f == Face::INSIDE.clone();
    isInside
}

pub(crate) fn setOutside(mut conn: metamodelica::Ref<NFConnector>) -> metamodelica::Ref<NFConnector> {
    let mut conn: metamodelica::Ref<NFConnector> = conn;
    if conn.face.clone() != Face::OUTSIDE.clone() {
        assign_field!(conn.face = Face::OUTSIDE.clone());
    }
    conn
}

pub(crate) fn isDeleted(mut conn: &metamodelica::Ref<NFConnector>) -> Result<bool> {
    let mut isDeleted: bool = ComponentRef::isDeleted(&conn.name)?;
    Ok(isDeleted)
}

pub(crate) fn isExpandable(mut conn: &metamodelica::Ref<NFConnector>) -> bool {
    let mut isExpandable: bool = ConnectorType::isExpandable(conn.cty.clone());
    isExpandable
}

pub(crate) fn isArray(mut conn: &metamodelica::Ref<NFConnector>) -> bool {
    let mut isArray: bool = Type::isArray(&conn.ty);
    isArray
}

pub(crate) fn name(mut conn: &metamodelica::Ref<NFConnector>) -> metamodelica::Ref<ComponentRef::NFComponentRef> {
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef> = conn.name.clone();
    name
}

pub(crate) fn toString(mut conn: &metamodelica::Ref<NFConnector>) -> Result<ArcStr> {
    let mut r#str: ArcStr = ComponentRef::toString(&conn.name)?;
    Ok(r#str)
}

pub(crate) fn faceString(mut conn: &metamodelica::Ref<NFConnector>) -> ArcStr {
    let mut r#str: ArcStr = if (conn.face.clone() == Face::INSIDE.clone()) {
        literal!("inside")
    } else {
        literal!("outside")
    };
    r#str
}

pub(crate) fn hash(mut conn: &metamodelica::Ref<NFConnector>) -> Result<i32> {
    let mut hash: i32 = ComponentRef::hash(&conn.name)?;
    Ok(hash)
}

pub(crate) fn hashNoSubs(mut conn: &metamodelica::Ref<NFConnector>) -> Result<i32> {
    let mut hash: i32;
    hash = ComponentRef::hashStrip(&conn.name)?;
    Ok(hash)
}

pub(crate) fn split(
    mut conn: &metamodelica::Ref<NFConnector>,
) -> Result<metamodelica::List<metamodelica::Ref<NFConnector>>> {
    let mut connl: metamodelica::List<metamodelica::Ref<NFConnector>>;
    connl = splitImpl(
        &conn.name,
        &conn.ty,
        conn.face.clone(),
        &conn.source,
        conn.cty.clone(),
        metamodelica::nil(),
        metamodelica::nil(),
    )?;
    connl = metamodelica::Dangerous::listReverseInPlace(connl);
    Ok(connl)
}

pub(crate) fn scalarize(
    mut conn: &metamodelica::Ref<NFConnector>,
) -> Result<metamodelica::List<metamodelica::Ref<NFConnector>>> {
    let mut connl: metamodelica::List<metamodelica::Ref<NFConnector>> = metamodelica::nil();
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut face: Face;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut cty: i32;
    let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let __arc5 = &(*conn);
    let CONNECTOR {
        name: __pa0,
        ty: __pa1,
        face: __pa2,
        cty: __pa3,
        source: __pa4,
    } = &**__arc5;
    name = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    face = metamodelica::Own::own(__pa2);
    cty = metamodelica::Own::own(__pa3);
    source = metamodelica::Own::own(__pa4);
    names = ComponentRef::scalarizeAll(name, false)?;
    ty = Type::arrayElementType(&ty);
    for mut n in &*names {
        connl = metamodelica::cons(
            metamodelica::Ref::new(NFConnector {
                name: n.clone(),
                ty: ty.clone(),
                face: face,
                cty: cty,
                source: source.clone(),
            }),
            connl,
        );
    }
    Ok(connl)
}

pub(crate) fn scalarizePrefix(
    mut conn: metamodelica::Ref<NFConnector>,
) -> Result<metamodelica::List<metamodelica::Ref<NFConnector>>> {
    let mut connl: metamodelica::List<metamodelica::Ref<NFConnector>> = metamodelica::nil();
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut prefix: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut face: Face;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut cty: i32;
    let mut prefixes: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let __arc5 = conn.clone();
    let CONNECTOR {
        name: __pa0,
        ty: __pa1,
        face: __pa2,
        cty: __pa3,
        source: __pa4,
    } = &*__arc5;
    name = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    face = metamodelica::Own::own(__pa2);
    cty = metamodelica::Own::own(__pa3);
    source = metamodelica::Own::own(__pa4);
    prefix = ComponentRef::rest(&name)?;
    if ComponentRef::isEmpty(&prefix) {
        connl = list![conn];
        return Ok(connl);
    }
    prefixes = ComponentRef::scalarizeAll(prefix, false)?;
    ty = ComponentRef::getSubscriptedType(&(ComponentRef::first(name.clone())), false)?;
    for mut p in &*prefixes {
        name = ComponentRef::prepend(p.clone(), name)?;
        connl = metamodelica::cons(
            metamodelica::Ref::new(NFConnector {
                name: name.clone(),
                ty: ty.clone(),
                face: face,
                cty: cty,
                source: source.clone(),
            }),
            connl,
        );
    }
    connl = metamodelica::Dangerous::listReverseInPlace(connl);
    Ok(connl)
}

pub(crate) fn addSubscripts(
    mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut conn: metamodelica::Ref<NFConnector>,
) -> Result<metamodelica::Ref<NFConnector>> {
    let mut conn: metamodelica::Ref<NFConnector> = conn;
    assign_field!(
        conn.name = ComponentRef::mergeSubscripts(subscripts.clone(), conn.name.clone(), true, false, false)?,
        conn.ty = Type::subscript(conn.ty.clone(), &subscripts, true)?
    );
    Ok(conn)
}

fn crefFace(mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<Face> {
    let mut face: Face;
    face = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { restCref: Deref @ ComponentRef::EMPTY, .. } => Face::OUTSIDE.clone(),
        _ => if (NFInstNode::InstNode::isConnector(&(ComponentRef::node(&(ComponentRef::firstNonScope(cref)?))?))?) {Face::OUTSIDE.clone()} else {Face::INSIDE.clone()},
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(face)
}

fn splitImpl<'__b>(
    mut name: &'__b metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ty: &'__b metamodelica::Ref<Type::NFType>,
    mut face: Face,
    mut source: &'__b metamodelica::Ref<DAE::ElementSource>,
    mut cty: i32,
    mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut conns: metamodelica::List<metamodelica::Ref<NFConnector>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFConnector>>> {
    '__tco: loop {
        let mut ct: metamodelica::Ref<ComplexType::NFComplexType>;
        let mut tree: metamodelica::Ref<ClassTree::ClassTree>;
        ::match_deref::match_deref! { match ty {
            Deref @ Type::COMPLEX { complexTy: __esc_ct @ Deref @ ComplexType::CONNECTOR { .. }, .. } => {
                ct = (*__esc_ct).clone();
                conns = splitImpl2(name, face, source, var_field!((*ct).potentials, ComplexType::NFComplexType::CONNECTOR), dims.clone(), conns)?;
                conns = splitImpl2(name, face, source, var_field!((*ct).flows, ComplexType::NFComplexType::CONNECTOR), dims.clone(), conns)?;
                return Ok(splitImpl2(name, face, source, var_field!((*ct).streams, ComplexType::NFComplexType::CONNECTOR), dims, conns)?)
            },
            Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. } => return Ok(metamodelica::cons(metamodelica::Ref::new(NFConnector { name: name.clone(), ty: Type::liftArrayLeftList(ty.clone(), &dims), face: face, cty: cty, source: source.clone() }), conns)),
            Deref @ Type::COMPLEX { .. } => {
                tree = Class::classTree(NFInstNode::InstNode::getClass(Type::complexNode(ty)?)?)?;
                return Ok(splitImpl2(name, face, source, &(({
            let mut __acc: metamodelica::List<Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>> = metamodelica::nil();
            for mut c in (ClassTree::getComponents(&tree)?).borrow().iter() {
                let __x = NFInstNode::InstNode::scopeRef(c.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })), dims, conns)?)
            },
            Deref @ Type::ARRAY { .. } => { (name, ty, face, source, cty, dims, conns) = (name, var_field!((**ty).elementType, Type::NFType::ARRAY), face, source, cty, listAppend(dims, var_field!((**ty).dimensions, Type::NFType::ARRAY).clone()), conns); continue '__tco; },
            _ => return Ok(metamodelica::cons(metamodelica::Ref::new(NFConnector { name: name.clone(), ty: Type::liftArrayLeftList(ty.clone(), &dims), face: face, cty: cty, source: source.clone() }), conns)),
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn splitImpl2(
    mut name: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut face: Face,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
    mut comps: &metamodelica::List<Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>>,
    mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut conns: metamodelica::List<metamodelica::Ref<NFConnector>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFConnector>>> {
    let mut conns: metamodelica::List<metamodelica::Ref<NFConnector>> = conns;
    let mut c: metamodelica::Ref<Component::NFComponent>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut cty: i32;
    let mut comp: metamodelica::Ref<InstNode::InstNode>;
    for mut comp_ref in &**comps {
        comp = NFInstNode::InstNode::borrow(comp_ref.clone())?;
        c = NFInstNode::InstNode::component(&comp)?;
        ty = Component::getType(&c)?;
        cty = Component::connectorType(&c);
        if !(ConnectorType::isPotentiallyPresent(cty)) {
            cref = ComponentRef::append(
                ComponentRef::fromNode(
                    comp,
                    ty.clone(),
                    metamodelica::nil(),
                    ComponentRef::Origin::CREF.clone(),
                )?,
                name,
            )?;
            conns = splitImpl(&cref, &ty, face, source, cty, dims.clone(), conns)?;
        }
    }
    Ok(conns)
}
