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

use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnector as Connector;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::ConnectorType;
use crate::NFType as Type;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NFConnections {
    pub connections: metamodelica::List<metamodelica::Ref<Connection::NFConnection>>,
    pub flows: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    pub broken: BrokenEdges,
}

impl metamodelica::gc::MMTrace for NFConnections {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.connections, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.flows, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.broken, __mmv)?;
        Ok(())
    }
}
impl Default for NFConnections {
    fn default() -> Self {
        Self {
            connections: Default::default(),
            flows: Default::default(),
            broken: Default::default(),
        }
    }
}

pub type CONNECTIONS = NFConnections;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct BrokenEdge {
    pub lhs: metamodelica::Ref<ComponentRef::NFComponentRef>,
    pub rhs: metamodelica::Ref<ComponentRef::NFComponentRef>,
    pub source: metamodelica::Ref<DAE::ElementSource>,
    pub brokenEquations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
}

impl metamodelica::gc::MMTrace for BrokenEdge {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.lhs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.rhs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.source, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.brokenEquations, __mmv)?;
        Ok(())
    }
}
impl Default for BrokenEdge {
    fn default() -> Self {
        Self {
            lhs: Default::default(),
            rhs: Default::default(),
            source: Default::default(),
            brokenEquations: Default::default(),
        }
    }
}

pub type BROKEN_EDGE = BrokenEdge;

pub type BrokenEdges = metamodelica::List<BrokenEdge>;

pub(crate) fn new() -> metamodelica::Ref<NFConnections> {
    let mut conns: metamodelica::Ref<NFConnections> = metamodelica::Ref::new(NFConnections {
        connections: metamodelica::nil(),
        flows: metamodelica::nil(),
        broken: metamodelica::nil(),
    });
    conns
}

pub(crate) fn fromConnectionList(
    mut connl: metamodelica::List<metamodelica::Ref<Connection::NFConnection>>,
) -> metamodelica::Ref<NFConnections> {
    let mut conns: metamodelica::Ref<NFConnections>;
    conns = metamodelica::Ref::new(NFConnections {
        connections: connl,
        flows: metamodelica::nil(),
        broken: metamodelica::nil(),
    });
    conns
}

pub(crate) fn addConnection(
    mut conn: metamodelica::Ref<Connection::NFConnection>,
    mut conns: metamodelica::Ref<NFConnections>,
) -> metamodelica::Ref<NFConnections> {
    let mut conns: metamodelica::Ref<NFConnections> = conns;
    assign_field!(conns.connections = metamodelica::cons(conn, conns.connections.clone()));
    conns
}

pub(crate) fn addFlow(
    mut conn: metamodelica::Ref<Connector::NFConnector>,
    mut conns: metamodelica::Ref<NFConnections>,
) -> metamodelica::Ref<NFConnections> {
    let mut conns: metamodelica::Ref<NFConnections> = conns;
    assign_field!(conns.flows = metamodelica::cons(conn, conns.flows.clone()));
    conns
}

pub(crate) fn addBroken(
    mut broken: BrokenEdges,
    mut conns: metamodelica::Ref<NFConnections>,
) -> metamodelica::Ref<NFConnections> {
    let mut conns: metamodelica::Ref<NFConnections> = conns;
    assign_field!(conns.broken = broken);
    conns
}

pub(crate) fn collectConnections(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
    mut isDeleted: &dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool>,
) -> Result<(
    metamodelica::Ref<FlatModel::NFFlatModel>,
    metamodelica::Ref<NFConnections>,
)> {
    pub type IsDeleted =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>;

    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut conns: metamodelica::Ref<NFConnections> = new();
    let mut lhs: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut rhs: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    for mut eq in &*flatModel.equations.clone() {
        eql = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ Equation::CONNECT { lhs: Deref @ Expression::CREF { ty: __esc_ty1, cref: __esc_lhs }, rhs: Deref @ Expression::CREF { ty: __esc_ty2, cref: __esc_rhs }, source: __esc_source, .. } => {
                ty1 = (*__esc_ty1).clone();
                lhs = (*__esc_lhs).clone();
                ty2 = (*__esc_ty2).clone();
                rhs = (*__esc_rhs).clone();
                source = (*__esc_source).clone();
                lhs = ComponentRef::evaluateSubscripts(lhs.clone())?;
                rhs = ComponentRef::evaluateSubscripts(rhs.clone())?;
                assign_field!(conns.connections = makeConnections(lhs.clone(), ty1.clone(), rhs.clone(), ty2.clone(), source.clone(), isDeleted, conns.connections.clone())?);
                eql
            },
            _ => metamodelica::cons(eq.clone(), eql),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    assign_field!(flatModel.equations = metamodelica::Dangerous::listReverseInPlace(eql));
    Ok((flatModel, conns))
}

pub(crate) fn collectFlows(
    mut flatModel: &metamodelica::Ref<FlatModel::NFFlatModel>,
    mut conns: metamodelica::Ref<NFConnections>,
) -> Result<metamodelica::Ref<NFConnections>> {
    let mut conns: metamodelica::Ref<NFConnections> = conns;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut c: metamodelica::Ref<Connector::NFConnector>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    for mut var in &*flatModel.variables.clone() {
        comp = InstNode::component(&(ComponentRef::node(&var.name)?))?;
        if Component::isFlow(&comp) {
            src = ElementSource::createElementSource(
                Component::info(&comp)?,
                None,
                &(openmodelica_frontend_types::DAE::Prefix::NOPRE),
                (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
            );
            c = Connector::fromFacedCref(
                var.name.clone(),
                var.ty.clone(),
                Connector::Face::INSIDE.clone(),
                src.clone(),
            )?;
            conns = addFlow(c, conns);
            if ConnectorType::isAugmented(var.attributes.connectorType.clone()) {
                c = Connector::fromFacedCref(var.name.clone(), var.ty.clone(), Connector::Face::OUTSIDE.clone(), src)?;
                conns = addFlow(c, conns);
            }
        }
    }
    Ok(conns)
}

pub(crate) fn makeConnections(
    mut lhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut lhsType: metamodelica::Ref<Type::NFType>,
    mut rhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut rhsType: metamodelica::Ref<Type::NFType>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut isDeleted: &dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool>,
    mut connections: metamodelica::List<metamodelica::Ref<Connection::NFConnection>>,
) -> Result<metamodelica::List<metamodelica::Ref<Connection::NFConnection>>> {
    pub type IsDeleted =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>;

    let mut connections: metamodelica::List<metamodelica::Ref<Connection::NFConnection>> = connections;
    let mut cl1: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut cl2: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut c2: metamodelica::Ref<Connector::NFConnector>;
    if isDeleted(lhsCref.clone())? || isDeleted(rhsCref.clone())? {
        return Ok(connections);
    }
    if InstNode::isName(&(ComponentRef::node(&lhsCref)?)) || InstNode::isName(&(ComponentRef::node(&rhsCref)?)) {
        cl1 = list![Connector::fromCref(lhsCref, lhsType, source.clone())?];
        cl2 = list![Connector::fromCref(rhsCref, rhsType, source)?];
    } else {
        cl1 = makeConnectors(lhsCref, &lhsType, source.clone())?;
        cl2 = makeConnectors(rhsCref, &rhsType, source)?;
    }
    for mut c1 in &*cl1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cl2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        c2 = metamodelica::Own::own(__pa0);
        cl2 = metamodelica::Own::own(__pa1);
        if !(isDeleted(c1.name.clone())? || isDeleted(c2.name.clone())?) {
            connections = metamodelica::cons(
                metamodelica::Ref::new(Connection::NFConnection {
                    lhs: c1.clone(),
                    rhs: c2,
                }),
                connections,
            );
        }
    }
    Ok(connections)
}

pub(crate) fn makeConnectors(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>> {
    let mut connectors: metamodelica::List<metamodelica::Ref<Connector::NFConnector>> = metamodelica::nil();
    let mut cref_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    if !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
        connectors = list![Connector::fromCref(
            cref.clone(),
            ComponentRef::getSubscriptedType(&cref, false)?,
            source
        )?];
        return Ok(connectors);
    }
    cref_exp = metamodelica::Ref::new(Expression::NFExpression::CREF {
        ty: ComponentRef::getSubscriptedType(&cref, false)?,
        cref: cref.clone(),
    });
    (cref_exp, expanded) = ExpandExp::expand(cref_exp, false, false)?;
    if expanded {
        connectors = Connector::fromExp(&cref_exp, &source, metamodelica::nil())?;
    } else {
        Error::terminate(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFConnections.makeConnectors"));
                __mm_s.push_str(&*literal!(" failed to expand connector `"));
                __mm_s.push_str(&*ComponentRef::toString(&cref)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            },
            &(ElementSource::getInfo(source)),
        )?;
    }
    Ok(connectors)
}

pub(crate) fn split(mut conns: metamodelica::Ref<NFConnections>) -> Result<metamodelica::Ref<NFConnections>> {
    let mut conns: metamodelica::Ref<NFConnections> = conns;
    assign_field!(
        conns.flows = List::mapFlat(&conns.flows, &move |__a0: metamodelica::Ref<Connector::NFConnector>| {
            Connector::split(&__a0)
        })?,
        conns.connections = List::mapFlat(&conns.connections, &move |__a0: metamodelica::Ref<
            Connection::NFConnection,
        >| Connection::split(&__a0))?
    );
    Ok(conns)
}

pub(crate) fn connectCount(
    mut conn: metamodelica::Ref<Connector::NFConnector>,
    mut connectCounts: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Connector::NFConnector>, i32>>,
) -> Result<i32> {
    let mut count: i32;
    count = UnorderedMap::getOrDefault(conn, connectCounts, 0)?;
    Ok(count)
}

pub(crate) fn scalarize(
    mut conns: metamodelica::Ref<NFConnections>,
    mut keepSingleConnectedArrays: bool,
) -> Result<metamodelica::Ref<NFConnections>> {
    let mut conns: metamodelica::Ref<NFConnections> = conns;
    let mut connect_counts: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Connector::NFConnector>, i32>,
    >;
    let mut flows: metamodelica::List<metamodelica::Ref<Connector::NFConnector>> = metamodelica::nil();
    let mut connections: metamodelica::List<metamodelica::Ref<Connection::NFConnection>> = metamodelica::nil();
    let mut count: i32;
    if keepSingleConnectedArrays {
        connect_counts = analyseArrayConnections(&conns)?;
        for mut f in &*conns.flows.clone() {
            count = connectCount(f.clone(), connect_counts.clone())?;
            if count == 0 {
                flows = metamodelica::cons(f.clone(), flows);
            } else if count > 1 || count == -1 {
                flows = listAppend(Connector::scalarize(metamodelica::AsArg::as_arg(&f))?, flows);
            }
        }
        for mut c in &*conns.connections.clone() {
            if !(ConnectorType::isStream(c.lhs.cty.clone()))
                && connectCount(c.lhs.clone(), connect_counts.clone())? == 1
                && connectCount(c.rhs.clone(), connect_counts.clone())? == 1
            {
                connections = metamodelica::cons(c.clone(), connections);
            } else {
                connections = listAppend(Connection::scalarize(c.clone())?, connections);
            }
        }
        assign_field!(
            conns.flows = metamodelica::Dangerous::listReverseInPlace(flows),
            conns.connections = metamodelica::Dangerous::listReverseInPlace(connections)
        );
    } else {
        assign_field!(
            conns.flows = List::mapFlat(&conns.flows, &move |__a0: metamodelica::Ref<Connector::NFConnector>| {
                Connector::scalarize(&__a0)
            })?,
            conns.connections = List::mapFlat(&conns.connections, &Connection::scalarize)?
        );
    }
    Ok(conns)
}

pub(crate) fn analyseArrayConnections(
    mut conns: &metamodelica::Ref<NFConnections>,
) -> Result<metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Connector::NFConnector>, i32>>> {
    let mut connectCounts: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Connector::NFConnector>, i32>,
    >;
    connectCounts = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<Connector::NFConnector>| Connector::hashNoSubs(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Connector::NFConnector>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Connector::NFConnector>, __a1: metamodelica::Ref<Connector::NFConnector>| {
                Connector::isEqualNoSubs(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Connector::NFConnector>,
                        metamodelica::Ref<Connector::NFConnector>,
                    ) -> Result<bool>
                    + 'static,
            >),
        ((conns.connections).len() as i32),
    );
    for mut conn in &*conns.connections.clone() {
        analyseArrayConnector(conn.lhs.clone(), connectCounts.clone())?;
        analyseArrayConnector(conn.rhs.clone(), connectCounts.clone())?;
    }
    Ok(connectCounts)
}

pub(crate) fn analyseArrayConnector(
    mut conn: metamodelica::Ref<Connector::NFConnector>,
    mut connectCounts: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Connector::NFConnector>, i32>>,
) -> Result<()> {
    fn update(mut count: Option<i32>) -> i32 {
        let mut outCount: i32;
        outCount = (match count {
            Some(mut __esc_outCount) => {
                outCount = __esc_outCount.clone();
                if (outCount >= 0) { outCount + 1 } else { -1 }
            }
            _ => 1,
        });
        outCount
    }

    if Connector::isArray(&conn) {
        UnorderedMap::addUpdate(conn, &fnptr!(update, Option<i32>), connectCounts)?;
    } else if ComponentRef::hasSubscripts(&conn.name)? {
        UnorderedMap::add(conn, -1, connectCounts)?;
    }
    Ok(())
}

pub(crate) fn toString(mut conns: &metamodelica::Ref<NFConnections>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut strl: metamodelica::List<ArcStr> = metamodelica::nil();
    strl = metamodelica::cons(literal!("FLOWS:"), strl);
    for mut f in &*conns.flows.clone() {
        strl = metamodelica::cons(Connector::toString(metamodelica::AsArg::as_arg(&f))?, strl);
    }
    strl = metamodelica::cons(literal!("\nCONNECTIONS:"), strl);
    for mut c in &*conns.connections.clone() {
        strl = metamodelica::cons(Connection::toString(metamodelica::AsArg::as_arg(&c))?, strl);
    }
    strl = metamodelica::Dangerous::listReverseInPlace(strl);
    r#str = stringDelimitList(strl, literal!("\n"));
    Ok(r#str)
}

pub(crate) fn toStringList(
    mut conns: &metamodelica::Ref<NFConnections>,
) -> Result<metamodelica::List<metamodelica::List<ArcStr>>> {
    let mut strl: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
    strl = ({
        let mut __acc: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut c in (conns.connections.clone()).into_iter().cloned() {
            let __x = list![
                Connector::toString(&(c.lhs.clone()))?,
                Connector::toString(&(c.rhs.clone()))?
            ];
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(strl)
}
