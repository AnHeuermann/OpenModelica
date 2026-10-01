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

use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstContext;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFPrefixes::Purity;
use crate::NFSBGraphUtil as SBGraphUtil;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::SBAtomicSet;
use openmodelica_util::SBFunctions;
use openmodelica_util::SBGraph::IncidenceList;
use openmodelica_util::SBGraph::VertexDescriptor;
use openmodelica_util::SBInterval;
use openmodelica_util::SBMultiInterval;
use openmodelica_util::SBPWLinearMap;
use openmodelica_util::SBSet;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub mod SetVertex {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct SetVertex {
        pub name: metamodelica::Ref<Connector::NFConnector>,
        pub vs: metamodelica::Ref<SBSet::SBSet>,
    }

    impl metamodelica::gc::MMTrace for SetVertex {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.vs, __mmv)?;
            Ok(())
        }
    }
    impl Default for SetVertex {
        fn default() -> Self {
            Self {
                name: Default::default(),
                vs: Default::default(),
            }
        }
    }

    pub type SET_VERTEX = SetVertex;

    pub(crate) fn isEqual(
        mut v1: &metamodelica::Ref<SetVertex>,
        mut v2: &metamodelica::Ref<SetVertex>,
    ) -> Result<bool> {
        let mut equal: bool = Connector::isEqual(&v1.name, &v2.name)?;
        Ok(equal)
    }

    pub(crate) fn isNamed(
        mut v: &metamodelica::Ref<SetVertex>,
        mut name: &metamodelica::Ref<Connector::NFConnector>,
    ) -> Result<bool> {
        let mut equal: bool = Connector::isEqual(&v.name, name)?;
        Ok(equal)
    }

    pub(crate) fn toString(mut v: &metamodelica::Ref<SetVertex>) -> Result<ArcStr> {
        let mut r#str: ArcStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Connector::toString(&v.name)?);
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*SBSet::toString(&v.vs)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }
}

pub mod SetEdge {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct SetEdge {
        pub name: ArcStr,
        pub es1: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
        pub es2: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
    }

    impl metamodelica::gc::MMTrace for SetEdge {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.es1, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.es2, __mmv)?;
            Ok(())
        }
    }
    impl Default for SetEdge {
        fn default() -> Self {
            Self {
                name: Default::default(),
                es1: Default::default(),
                es2: Default::default(),
            }
        }
    }

    pub type SET_EDGE = SetEdge;

    pub(crate) fn isEqual(mut e1: &metamodelica::Ref<SetEdge>, mut e2: &metamodelica::Ref<SetEdge>) -> bool {
        let mut equal: bool = metamodelica::stringEq(&e1.name, &e2.name);
        equal
    }

    pub(crate) fn toString(mut e: &metamodelica::Ref<SetEdge>) -> Result<ArcStr> {
        let mut r#str: ArcStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*e.name);
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*literal!("SetVertex 1:\t"));
            __mm_s.push_str(&*SBPWLinearMap::toString(&e.es1)?);
            __mm_s.push_str(&*literal!("\nSetVertex 2:\t"));
            __mm_s.push_str(&*SBPWLinearMap::toString(&e.es2)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }
}

pub type NameVertexTable =
    metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<SBMultiInterval::SBMultiInterval>>>;

pub type SBGraph = metamodelica::Ref<
    IncidenceList::IncidenceList<metamodelica::Ref<SetVertex::SetVertex>, metamodelica::Ref<SetEdge::SetEdge>>,
>;

pub(crate) fn resolve(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut max_dim: i32 = 1;
    let mut v_count: metamodelica::Ref<Vector::Vector<i32>>;
    let mut e_count: metamodelica::Ref<Vector::Vector<i32>>;
    let mut conns: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut graph: SBGraph;
    let mut vss: metamodelica::Ref<SBSet::SBSet>;
    let mut res: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut emap1: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut emap2: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut nmv_table: NameVertexTable;
    for mut var in &*flatModel.variables.clone() {
        max_dim = std::cmp::max(max_dim, Type::dimensionCount(var.ty.clone()));
    }
    v_count = Vector::newFill(max_dim, 1);
    e_count = Vector::newFill(max_dim, 1);
    (flatModel, conns) = collect(flatModel)?;
    graph = IncidenceList::new(
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<SetVertex::SetVertex>, __a1: metamodelica::Ref<SetVertex::SetVertex>| {
                SetVertex::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<SetVertex::SetVertex>,
                        metamodelica::Ref<SetVertex::SetVertex>,
                    ) -> Result<bool>
                    + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<SetEdge::SetEdge>,
                  __a1: metamodelica::Ref<SetEdge::SetEdge>|
                  -> metamodelica::Result<_> { ::std::result::Result::Ok(SetEdge::isEqual(&__a0, &__a1)) },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<SetEdge::SetEdge>,
                        metamodelica::Ref<SetEdge::SetEdge>,
                    ) -> Result<bool>
                    + 'static,
            >),
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<SetVertex::SetVertex>| SetVertex::toString(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SetVertex::SetVertex>) -> Result<ArcStr> + 'static>),
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<SetEdge::SetEdge>| SetEdge::toString(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SetEdge::SetEdge>) -> Result<ArcStr> + 'static>),
    );
    nmv_table = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    createGraph(
        &flatModel.variables,
        &conns,
        graph.clone(),
        v_count.clone(),
        e_count,
        nmv_table.clone(),
    )?;
    if Flags::isSet(Flags::DUMP_SET_BASED_GRAPHS.clone())? {
        metamodelica::print(IncidenceList::toString(graph.clone())?);
    }
    (vss, emap1, emap2) = createMaps(graph.clone())?;
    res = SBFunctions::connectedComponents(vss, &emap1, &emap2)?;
    if Flags::isSet(Flags::DUMP_SET_BASED_GRAPHS.clone())? {
        metamodelica::print(IncidenceList::toString(graph.clone())?);
    }
    conns = generateEquations(&res, &flatModel, graph, v_count, nmv_table)?;
    eql = listAppend(flatModel.equations.clone(), conns);
    assign_field!(flatModel.equations = eql);
    Ok(flatModel)
}

fn collect(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<(
    metamodelica::Ref<FlatModel::NFFlatModel>,
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
)> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut conns: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    (conns, eql) = List::splitOnTrue(
        &flatModel.equations,
        &fnptr!(isConnection, metamodelica::Ref<Equation::NFEquation>),
    )?;
    assign_field!(flatModel.equations = eql);
    Ok((flatModel, conns))
}

fn isConnection(mut eq: metamodelica::Ref<Equation::NFEquation>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(eq) {
            Deref @ Equation::CONNECT { .. } => {
                return true
            },
            Deref @ Equation::FOR { body: Deref @ metamodelica::ListNode::Cons { head: e, tail: _ }, .. } => {
                { eq = e.clone(); continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn createGraph(
    mut variables: &metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut equations: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut graph: SBGraph,
    mut vCount: metamodelica::Ref<Vector::Vector<i32>>,
    mut eCount: metamodelica::Ref<Vector::Vector<i32>>,
    mut nmvTable: NameVertexTable,
) -> Result<()> {
    addFlowsToGraph(variables, graph.clone(), vCount.clone(), nmvTable.clone())?;
    addConnectionsToGraph(equations, graph, vCount, eCount, nmvTable)?;
    Ok(())
}

fn addFlowsToGraph(
    mut variables: &metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut graph: SBGraph,
    mut vCount: metamodelica::Ref<Vector::Vector<i32>>,
    mut nmvTable: NameVertexTable,
) -> Result<()> {
    let mut conn: metamodelica::Ref<Connector::NFConnector>;
    let mut parent_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    for mut var in &**variables {
        if Variable::isFlow(metamodelica::AsArg::as_arg(&var)) {
            parent_cr = ComponentRef::rest(&var.name)?;
            conn = Connector::fromFacedCref(
                parent_cr.clone(),
                ComponentRef::nodeType(&parent_cr)?,
                Connector::Face::INSIDE.clone(),
                ElementSource::createElementSource(
                    var.info.clone(),
                    None,
                    &(openmodelica_frontend_types::DAE::Prefix::NOPRE),
                    (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
                ),
            )?;
            createVertex(conn, graph.clone(), vCount.clone(), nmvTable.clone())?;
        }
    }
    Ok(())
}

fn addConnectionsToGraph(
    mut equations: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut graph: SBGraph,
    mut vCount: metamodelica::Ref<Vector::Vector<i32>>,
    mut eCount: metamodelica::Ref<Vector::Vector<i32>>,
    mut nmvTable: NameVertexTable,
) -> Result<()> {
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    for mut eq in &**equations {
        let () = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ Equation::CONNECT { lhs: __eq_lhs, rhs: __eq_rhs, source: __eq_source, .. } => {
                createConnection(metamodelica::AsArg::as_arg(&__eq_lhs), metamodelica::AsArg::as_arg(&__eq_rhs), __eq_source.clone(), graph.clone(), vCount.clone(), eCount.clone(), nmvTable.clone())?;
                ()
            },
            Deref @ Equation::FOR { range: Some(__esc_range), body: __eq_body, iterator: __eq_iterator, .. } => {
                range = (*__esc_range).clone();
                range = Ceval::evalExp(range.clone(), &(Ceval::EvalTarget::new(Equation::info(metamodelica::AsArg::as_arg(&eq)), NFInstContext::ITERATION_RANGE.clone(), None)))?;
                body = Equation::replaceIteratorList(__eq_body.clone(), metamodelica::AsArg::as_arg(&__eq_iterator), metamodelica::AsArg::as_arg(&range))?;
                addConnectionsToGraph(&body, graph.clone(), vCount.clone(), eCount.clone(), nmvTable.clone())?;
                ()
            },
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFArrayConnections.addConnectionsToGraph")); __mm_s.push_str(&*literal!(" got unknown equation ")); __mm_s.push_str(&*Equation::toString(metamodelica::AsArg::as_arg(&eq), literal!(""))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFArrayConnections.mo")))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(())
}

fn createConnection(
    mut lhs: &metamodelica::Ref<Expression::NFExpression>,
    mut rhs: &metamodelica::Ref<Expression::NFExpression>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut graph: SBGraph,
    mut vCount: metamodelica::Ref<Vector::Vector<i32>>,
    mut eCount: metamodelica::Ref<Vector::Vector<i32>>,
    mut nmvTable: NameVertexTable,
) -> Result<()> {
    let mut lhs_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut rhs_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut lhs_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut rhs_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut mi1: metamodelica::Ref<SBMultiInterval::SBMultiInterval>;
    let mut mi2: metamodelica::Ref<SBMultiInterval::SBMultiInterval>;
    let mut d1: i32;
    let mut d2: i32;
    let mut lhs_conn: metamodelica::Ref<Connector::NFConnector>;
    let mut rhs_conn: metamodelica::Ref<Connector::NFConnector>;
    (lhs_cr, lhs_subs) = separate(Expression::toCref(lhs)?)?;
    (rhs_cr, rhs_subs) = separate(Expression::toCref(rhs)?)?;
    lhs_conn = Connector::fromCref(lhs_cr.clone(), ComponentRef::nodeType(&lhs_cr)?, source.clone())?;
    rhs_conn = Connector::fromCref(rhs_cr.clone(), ComponentRef::nodeType(&rhs_cr)?, source)?;
    (mi1, d1) = getConnectIntervals(lhs_conn, &lhs_subs, graph.clone(), vCount.clone(), nmvTable.clone())?;
    (mi2, d2) = getConnectIntervals(rhs_conn, &rhs_subs, graph.clone(), vCount, nmvTable)?;
    updateGraph(d1, d2, &mi1, &mi2, graph, eCount)?;
    Ok(())
}

fn separate(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<(
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
)> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    cref = ComponentRef::fillSubscripts(cref);
    cref = ComponentRef::replaceWholeSubscripts(cref)?;
    subs = ComponentRef::subscriptsAllFlat(&cref)?;
    cref = ComponentRef::stripSubscriptsAll(&cref);
    Ok((cref, subs))
}

fn getConnectIntervals(
    mut conn: metamodelica::Ref<Connector::NFConnector>,
    mut subs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut graph: SBGraph,
    mut vCount: metamodelica::Ref<Vector::Vector<i32>>,
    mut nmvTable: NameVertexTable,
) -> Result<(metamodelica::Ref<SBMultiInterval::SBMultiInterval>, i32)> {
    let mut outMI: metamodelica::Ref<SBMultiInterval::SBMultiInterval>;
    let mut d: i32;
    (outMI, d) = createVertex(conn, graph, vCount.clone(), nmvTable)?;
    outMI = SBGraphUtil::multiIntervalFromSubscripts(subs, vCount, outMI)?;
    Ok((outMI, d))
}

fn createVertex(
    mut conn: metamodelica::Ref<Connector::NFConnector>,
    mut graph: SBGraph,
    mut vCount: metamodelica::Ref<Vector::Vector<i32>>,
    mut nmvTable: NameVertexTable,
) -> Result<(metamodelica::Ref<SBMultiInterval::SBMultiInterval>, i32)> {
    let mut mi: metamodelica::Ref<SBMultiInterval::SBMultiInterval>;
    let mut d: i32;
    let mut od: Option<i32>;
    let mut v: metamodelica::Ref<SetVertex::SetVertex>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut s: metamodelica::Ref<SBSet::SBSet>;
    let mut name: ArcStr;
    od = IncidenceList::findVertex(
        graph.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = conn.clone();
            move |__pe_a0| SetVertex::isNamed(&__pe_a0, &__pe_b1)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SetVertex::SetVertex>) -> Result<bool> + 'static>),
    )?;
    if (od).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(od) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        d = metamodelica::Own::own(__pa0);
        v = IncidenceList::getVertex(graph, d)?;
        mi = SBAtomicSet::aset(&(UnorderedSet::first(SBSet::asets(&v.vs))?));
        return Ok((mi, d));
    }
    dims = crefDims(Connector::name(&conn))?;
    mi = SBGraphUtil::multiIntervalFromDimensions(&dims, vCount)?;
    s = SBSet::newEmpty();
    s = SBSet::addAtomicSet(SBAtomicSet::new(&mi), s)?;
    v = metamodelica::Ref::new(SetVertex::SetVertex {
        name: conn.clone(),
        vs: s,
    });
    d = IncidenceList::addVertex(graph, v);
    name = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Connector::toString(&conn)?);
        __mm_s.push_str(&*literal!("$"));
        __mm_s.push_str(&*Connector::faceString(&conn));
        ArcStr::from(__mm_s)
    };
    UnorderedMap::addUnique(name, mi.clone(), nmvTable)?;
    Ok((mi, d))
}

fn crefDims(
    mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>> {
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
    let mut c: metamodelica::Ref<ComponentRef::NFComponentRef> = cr;
    while !(ComponentRef::isEmpty(&c)) {
        dims = listAppend(Type::arrayDims(ComponentRef::nodeType(&c)?), dims);
        c = ComponentRef::rest(&c)?;
    }
    Ok(dims)
}

fn updateGraph(
    mut d1: i32,
    mut d2: i32,
    mut mi1: &metamodelica::Ref<SBMultiInterval::SBMultiInterval>,
    mut mi2: &metamodelica::Ref<SBMultiInterval::SBMultiInterval>,
    mut graph: SBGraph,
    mut eCount: metamodelica::Ref<Vector::Vector<i32>>,
) -> Result<()> {
    let mut pw1: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut pw2: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut name: ArcStr;
    let mut se: metamodelica::Ref<SetEdge::SetEdge>;
    (name, pw1, pw2) = SBGraphUtil::linearMapFromIntervals(d1, d2, mi1, mi2, eCount)?;
    se = metamodelica::Ref::new(SetEdge::SetEdge {
        name: name,
        es1: pw1,
        es2: pw2,
    });
    IncidenceList::addEdge(graph, d1, d2, se)?;
    Ok(())
}

fn createMaps(
    mut graph: SBGraph,
) -> Result<(
    metamodelica::Ref<SBSet::SBSet>,
    metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
    metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
)> {
    let mut vss: metamodelica::Ref<SBSet::SBSet>;
    let mut emap1: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut emap2: metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>;
    let mut vs: metamodelica::List<metamodelica::Ref<SetVertex::SetVertex>>;
    let mut es: metamodelica::List<metamodelica::Ref<SetEdge::SetEdge>>;
    let mut e: metamodelica::Ref<SetEdge::SetEdge>;
    vss = SBSet::newEmpty();
    for mut v in &*IncidenceList::vertices(graph.clone()) {
        vss = SBSet::union(&vss, &v.vs)?;
    }
    es = IncidenceList::edges(graph.clone());
    if (es).is_empty() {
        emap1 = SBPWLinearMap::newEmpty();
        emap2 = SBPWLinearMap::newEmpty();
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(IncidenceList::edges(graph)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        es = metamodelica::Own::own(__pa1);
        emap1 = e.es1.clone();
        emap2 = e.es2.clone();
        for mut e in &*es {
            let mut e = e.clone();
            emap1 = SBPWLinearMap::combine(e.es1.clone(), emap1)?;
            emap2 = SBPWLinearMap::combine(e.es2.clone(), emap2)?;
        }
    }
    Ok((vss, emap1, emap2))
}

fn generateEquations(
    mut pw: &metamodelica::Ref<SBPWLinearMap::SBPWLinearMap>,
    mut flatModel: &metamodelica::Ref<FlatModel::NFFlatModel>,
    mut graph: SBGraph,
    mut vCount: metamodelica::Ref<Vector::Vector<i32>>,
    mut nmvTable: NameVertexTable,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut vc_dom: metamodelica::Ref<SBSet::SBSet>;
    let mut vc_im: metamodelica::Ref<SBSet::SBSet>;
    let mut aux_s: metamodelica::Ref<SBSet::SBSet>;
    let mut vc_domi: metamodelica::Ref<SBSet::SBSet>;
    let mut vc_domi_aux: metamodelica::Ref<SBSet::SBSet>;
    let mut iterators: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut pot_vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut flow_vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut vars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut iter_expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    vc_dom = SBPWLinearMap::wholeDom(pw)?;
    vc_im = SBPWLinearMap::image(pw, &vc_dom)?;
    iterators = arrayCreate(Vector::size(vCount), crate::NFInstNode::InstNode::interned_EMPTY_NODE());
    for mut i in 1..=metamodelica::arrayLength(iterators.clone()) {
        {
            let __cell0 =
                NFInstNode::InstNode::newUniqueIterator(Absyn::dummyInfo.clone(), crate::NFType::interned_INTEGER());
            let __idx0 = i;
            *metamodelica::index_mut_checked(&mut iterators.clone().borrow_mut(), __idx0)? = __cell0;
        }
    }
    iter_expl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut i in (iterators.clone()).borrow().iter() {
            let __x = Expression::fromCref(
                ComponentRef::makeIterator(i.clone(), crate::NFType::interned_INTEGER())?,
                false,
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    (pot_vars, flow_vars) = getConnectors(flatModel);
    let __range1 = UnorderedSet::toArray(SBSet::asets(&vc_im))
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut aset in __range1 {
        aux_s = SBSet::newEmpty();
        aux_s = SBSet::addAtomicSet(aset.clone(), aux_s)?;
        vc_domi = SBPWLinearMap::preImage(pw, &aux_s)?;
        vc_domi_aux = SBSet::complement(&vc_domi, &aux_s)?;
        vars = getVars(&pot_vars, &aux_s, graph.clone())?;
        equations = generatePotentialEquations(
            &aset,
            &vc_domi_aux,
            &vars,
            iterators.clone(),
            iter_expl.clone(),
            &pot_vars,
            graph.clone(),
            nmvTable.clone(),
            equations,
        )?;
        equations = generateFlowEquation(
            &aset,
            &vc_domi,
            iterators.clone(),
            &flow_vars,
            graph.clone(),
            nmvTable.clone(),
            equations,
        )?;
    }
    equations = metamodelica::Dangerous::listReverseInPlace(equations);
    Ok(equations)
}

fn intervalToRange(
    mut interval: &metamodelica::Ref<SBInterval::SBInterval>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut lo: i32 = SBInterval::lowerBound(interval);
    let mut hi: i32 = SBInterval::upperBound(interval);
    if lo == hi {
        range = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: lo });
    } else {
        range = Expression::makeIntegerRange(lo, SBInterval::stepValue(interval), hi)?;
    }
    Ok(range)
}

fn generatePotentialEquations(
    mut aset: &metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
    mut dom: &metamodelica::Ref<SBSet::SBSet>,
    mut vars: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut iterators: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
    mut iterExps: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut potVars: &metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut graph: SBGraph,
    mut nmvTable: NameVertexTable,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    let mut sauxi: metamodelica::Ref<SBSet::SBSet>;
    let mut mi: metamodelica::Ref<SBMultiInterval::SBMultiInterval>;
    let mut mi_range: metamodelica::Ref<SBMultiInterval::SBMultiInterval>;
    let mut aux_mi: metamodelica::Ref<SBMultiInterval::SBMultiInterval>;
    let mut inters: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut ranges: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut vars1: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut inds: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let __range0 = UnorderedSet::toArray(SBSet::asets(dom))
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut auxi in __range0 {
        mi = SBAtomicSet::aset(&auxi);
        mi_range = applyOffset(&(mi.clone()), getOffset(&mi, nmvTable.clone())?)?;
        inters = SBMultiInterval::intervals(&mi_range);
        ranges = Array::map(inters.clone(), &move |__a0: metamodelica::Ref<
            SBInterval::SBInterval,
        >| intervalToRange(&__a0))?;
        sauxi = SBSet::newEmpty();
        sauxi = SBSet::addAtomicSet(auxi, sauxi)?;
        vars1 = getVars(potVars, &sauxi, graph.clone())?;
        mi = SBAtomicSet::aset(aset);
        aux_mi = applyOffset(&(mi.clone()), getOffset(&mi, nmvTable.clone())?)?;
        (inds, _) = transMulti(&mi_range, &aux_mi, iterators.clone(), false)?;
        eql = generatePotentialEquations2(&vars1, vars, iterExps.clone(), inds)?;
        equations = generateForLoop(eql, iterators.clone(), ranges.clone(), equations)?;
    }
    Ok(equations)
}

fn generatePotentialEquations2(
    mut vars1: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut vars2: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut inds1: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut inds2: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut l: metamodelica::Ref<Expression::NFExpression>;
    let mut r: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut eq: metamodelica::Ref<Equation::NFEquation>;
    for mut var1 in &**vars1 {
        for mut var2 in &**vars2 {
            if Type::isEqual(
                &(ComponentRef::nodeType(metamodelica::AsArg::as_arg(&var1))?),
                &(ComponentRef::nodeType(metamodelica::AsArg::as_arg(&var2))?),
            )? {
                l = generateConnector(var1.clone(), inds1.clone())?;
                r = generateConnector(var2.clone(), inds2.clone())?;
                ty = Expression::typeOf(l.clone());
                eq = Equation::makeEquality(
                    l,
                    r,
                    ty,
                    DAE::emptyElementSource().clone(),
                    crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
                    Equation::ScalarizeMode::DONT_SCALARIZE.clone(),
                );
                equations = metamodelica::cons(eq, equations);
            }
        }
    }
    equations = metamodelica::Dangerous::listReverseInPlace(equations);
    Ok(equations)
}

fn generateFlowEquation(
    mut aset: &metamodelica::Ref<SBAtomicSet::SBAtomicSet>,
    mut dom: &metamodelica::Ref<SBSet::SBSet>,
    mut iterators: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
    mut flowVars: &metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut graph: SBGraph,
    mut nmvTable: NameVertexTable,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    let mut mi: metamodelica::Ref<SBMultiInterval::SBMultiInterval>;
    let mut mi_range: metamodelica::Ref<SBMultiInterval::SBMultiInterval>;
    let mut mi_range2: metamodelica::Ref<SBMultiInterval::SBMultiInterval>;
    let mut sauxi: metamodelica::Ref<SBSet::SBSet>;
    let mut inters: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut ranges: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut inds: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut is_sum: bool;
    let mut vars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut sum_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut eq: metamodelica::Ref<Equation::NFEquation>;
    mi = SBAtomicSet::aset(aset);
    mi_range = applyOffset(&(mi.clone()), getOffset(&mi, nmvTable.clone())?)?;
    inters = SBMultiInterval::intervals(&mi_range);
    ranges = Array::map(inters.clone(), &move |__a0: metamodelica::Ref<
        SBInterval::SBInterval,
    >| intervalToRange(&__a0))?;
    expl = metamodelica::nil();
    let __range0 = UnorderedSet::toArray(SBSet::asets(dom))
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut auxi in __range0 {
        mi = SBAtomicSet::aset(&auxi);
        mi_range2 = applyOffset(&(mi.clone()), getOffset(&mi, nmvTable.clone())?)?;
        (inds, is_sum) = transMulti(&mi_range, &mi_range2, iterators.clone(), true)?;
        sauxi = SBSet::newEmpty();
        sauxi = SBSet::addAtomicSet(auxi, sauxi)?;
        vars = getVars(flowVars, &sauxi, graph.clone())?;
        for mut var in &*vars {
            e = generateConnector(var.clone(), inds.clone())?;
            if is_sum {
                e = metamodelica::Ref::new(Expression::NFExpression::CALL {
                    call: Call::makeTypedCall(
                        NFBuiltinFuncs::SUM().clone(),
                        list![e.clone()],
                        Expression::variability(e.clone())?,
                        Purity::PURE.clone(),
                        Type::arrayElementType(&(Expression::typeOf(e))),
                    ),
                });
            }
            expl = metamodelica::cons(e, expl);
        }
    }
    if !((expl).is_empty()) {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(expl) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        sum_exp = metamodelica::Own::own(__pa1);
        expl = metamodelica::Own::own(__pa2);
        while !((expl).is_empty()) {
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(expl) {
                Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa3);
            expl = metamodelica::Own::own(__pa4);
            sum_exp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: e.clone(),
                operator: Operator::makeAdd(Expression::typeOf(e)),
                exp2: sum_exp,
            });
        }
        ty = Expression::typeOf(sum_exp.clone());
        eq = Equation::makeEquality(
            sum_exp,
            Expression::makeZero(&ty)?,
            ty,
            DAE::emptyElementSource().clone(),
            crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
            Equation::ScalarizeMode::NO_PREFERENCE.clone(),
        );
        equations = generateForLoop(list![eq], iterators.clone(), ranges.clone(), equations)?;
    }
    Ok(equations)
}

fn generateConnector(
    mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut indices: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    outExp = Expression::fromCref(cr, false)?;
    if Type::isArray(&(Expression::typeOf(outExp.clone()))) {
        subs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
            for mut i in (indices).into_iter().cloned() {
                let __x = Subscript::fromTypedExp(i.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        subs = List::firstN(subs, Type::dimensionCount(Expression::typeOf(outExp.clone())))?;
        outExp = Expression::applySubscripts(&subs, outExp, false)?;
    }
    Ok(outExp)
}

fn generateForLoop(
    mut connects: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut iterators: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
    mut ranges: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let __ab_ranges = ranges.borrow();
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    let mut body: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = connects;
    for mut i in ({
        let __s = metamodelica::arrayLength(iterators.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        if Expression::isInteger(&(*metamodelica::index_checked(&__ab_ranges, i)?)) {
            body = Equation::replaceIteratorList(
                body,
                &({
                    let __elt = (*metamodelica::index_checked(&iterators.borrow(), i)?).clone();
                    __elt
                }),
                &(*metamodelica::index_checked(&__ab_ranges, i)?),
            )?;
        } else {
            body = list![metamodelica::Ref::new(Equation::NFEquation::FOR {
                iterator: ({
                    let __elt = (*metamodelica::index_checked(&iterators.borrow(), i)?).clone();
                    __elt
                }),
                range: Some((*metamodelica::index_checked(&__ab_ranges, i)?).clone()),
                body: body,
                scope: NFInstNode::NO_SCOPE().clone(),
                source: DAE::emptyElementSource().clone()
            })];
        }
    }
    equations = List::append_reverse(&body, equations);
    Ok(equations)
}

fn getConnectors(
    mut flatModel: &metamodelica::Ref<FlatModel::NFFlatModel>,
) -> (
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
) {
    let mut effVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    let mut flowVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    for mut v in &*flatModel.variables.clone() {
        if Variable::isPotential(metamodelica::AsArg::as_arg(&v)) {
            effVars = metamodelica::cons(v.clone(), effVars);
        } else if Variable::isFlow(metamodelica::AsArg::as_arg(&v)) {
            flowVars = metamodelica::cons(v.clone(), flowVars);
        }
    }
    effVars = metamodelica::Dangerous::listReverseInPlace(effVars);
    flowVars = metamodelica::Dangerous::listReverseInPlace(flowVars);
    (effVars, flowVars)
}

fn getOffset(
    mut mi: &metamodelica::Ref<SBMultiInterval::SBMultiInterval>,
    mut nmvTable: NameVertexTable,
) -> Result<metamodelica::Array<i32>> {
    let mut res: metamodelica::Array<i32>;
    let mut i: metamodelica::Ref<SBMultiInterval::SBMultiInterval> =
        <metamodelica::Ref<SBMultiInterval::SBMultiInterval> as ::std::default::Default>::default();
    let mut aux: metamodelica::Ref<SBMultiInterval::SBMultiInterval>;
    res = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    for mut i in &*UnorderedMap::valueList(nmvTable) {
        let mut i = i.clone();
        aux = SBMultiInterval::intersection(mi, &i)?;
        if !(SBMultiInterval::isEmpty(&aux)) {
            res = SBMultiInterval::minElem(&i)?;
        }
    }
    Ok(res)
}

fn applyOffset(
    mut mi: &metamodelica::Ref<SBMultiInterval::SBMultiInterval>,
    mut off: metamodelica::Array<i32>,
) -> Result<metamodelica::Ref<SBMultiInterval::SBMultiInterval>> {
    let mut outMI: metamodelica::Ref<SBMultiInterval::SBMultiInterval>;
    let mut ints: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut res: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut i: metamodelica::Ref<SBInterval::SBInterval>;
    let mut o: i32;
    if SBMultiInterval::ndim(mi) != metamodelica::arrayLength(off.clone()) || off.clone().borrow().is_empty() {
        outMI = SBMultiInterval::newEmpty();
    } else {
        ints = SBMultiInterval::intervals(mi);
        res = metamodelica::arrayCreate(
            metamodelica::arrayLength(ints.clone()),
            ({
                let __elt = (*metamodelica::index_checked(&ints.borrow(), 1)?).clone();
                __elt
            }),
        );
        for mut j in 1..=metamodelica::arrayLength(ints.clone()) {
            i = ({
                let __elt = (*metamodelica::index_checked(&ints.borrow(), j)?).clone();
                __elt
            });
            o = ({
                let __elt = (*metamodelica::index_checked(&off.borrow(), j)?).clone();
                __elt
            });
            {
                let __cell0 = SBInterval::new(
                    SBInterval::lowerBound(&i) - o + 1,
                    SBInterval::stepValue(&i),
                    SBInterval::upperBound(&i) - o + 1,
                );
                let __idx0 = j;
                let _ = unsafe { metamodelica::Dangerous::arrayInitSlotChecked(res.clone().clone(), __idx0, __cell0) }?;
            }
        }
        outMI = SBMultiInterval::fromArray(res.clone())?;
    }
    Ok(outMI)
}

fn getVars(
    mut vars: &metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut sauxi: &metamodelica::Ref<SBSet::SBSet>,
    mut graph: SBGraph,
) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut res: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut vl: metamodelica::List<metamodelica::Ref<SetVertex::SetVertex>>;
    vl = IncidenceList::vertices(graph);
    for mut v in &*vl {
        if !(SBSet::isEmpty(&(SBSet::intersection(&v.vs, sauxi)?))) {
            for mut var in &**vars {
                if ComponentRef::isPrefix(&(Connector::name(&v.name)), &var.name)? {
                    res = metamodelica::cons(var.name.clone(), res);
                }
            }
        }
    }
    res = metamodelica::Dangerous::listReverseInPlace(res);
    Ok(res)
}

fn transMulti(
    mut mi1: &metamodelica::Ref<SBMultiInterval::SBMultiInterval>,
    mut mi2: &metamodelica::Ref<SBMultiInterval::SBMultiInterval>,
    mut iterators: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
    mut forFlow: bool,
) -> Result<(metamodelica::List<metamodelica::Ref<Expression::NFExpression>>, bool)> {
    let __ab_iterators = iterators.borrow();
    let mut outExpl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut flowRange: bool = false;
    let mut ints1: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut ints2: metamodelica::Array<metamodelica::Ref<SBInterval::SBInterval>>;
    let mut i1: metamodelica::Ref<SBInterval::SBInterval>;
    let mut i2: metamodelica::Ref<SBInterval::SBInterval>;
    let mut i1_sz: i32;
    let mut i2_sz: i32;
    let mut m_int: i32;
    let mut x: metamodelica::Ref<Expression::NFExpression>;
    let mut m: metamodelica::Ref<Expression::NFExpression>;
    let mut h: metamodelica::Ref<Expression::NFExpression>;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    if SBMultiInterval::ndim(mi1) != SBMultiInterval::ndim(mi2) {
        return Ok((outExpl, flowRange));
    }
    ints1 = SBMultiInterval::intervals(mi1);
    ints2 = SBMultiInterval::intervals(mi2);
    for mut i in 1..=metamodelica::arrayLength(ints1.clone()) {
        i1 = ({
            let __elt = (*metamodelica::index_checked(&ints1.borrow(), i)?).clone();
            __elt
        });
        i2 = ({
            let __elt = (*metamodelica::index_checked(&ints2.borrow(), i)?).clone();
            __elt
        });
        i1_sz = SBInterval::size(&i1);
        i2_sz = SBInterval::size(&i2);
        x = Expression::fromCref(
            ComponentRef::makeIterator(
                (*metamodelica::index_checked(&__ab_iterators, i)?).clone(),
                crate::NFType::interned_INTEGER(),
            )?,
            false,
        )?;
        if i1_sz == i2_sz {
            m_int = intDiv(SBInterval::stepValue(&i2), SBInterval::stepValue(&i1));
            m = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: m_int });
            h = metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                value: -(m_int * SBInterval::lowerBound(&i1)) + SBInterval::lowerBound(&i2),
            });
            e = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: metamodelica::Ref::new(Expression::NFExpression::BINARY {
                    exp1: m,
                    operator: Operator::makeMul(crate::NFType::interned_INTEGER()),
                    exp2: x,
                }),
                operator: Operator::makeAdd(crate::NFType::interned_INTEGER()),
                exp2: h,
            });
            outExpl = metamodelica::cons(e, outExpl);
        } else if i2_sz == 1 && !(forFlow) {
            outExpl = metamodelica::cons(
                metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                    value: SBInterval::lowerBound(&i2),
                }),
                outExpl,
            );
        } else if i1_sz == 1 && forFlow {
            e = Expression::makeIntegerRange(
                SBInterval::lowerBound(&i2),
                SBInterval::stepValue(&i2),
                SBInterval::upperBound(&i2),
            )?;
            outExpl = metamodelica::cons(e, outExpl);
            flowRange = true;
        } else {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFArrayConnections.transMulti"));
                    __mm_s.push_str(&*literal!(" got invalid intervals."));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFArrayConnections.mo")),
            )?;
        }
    }
    outExpl = metamodelica::Dangerous::listReverseInPlace(outExpl);
    Ok((outExpl, flowRange))
}
