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

use crate::NFAttributes;
use crate::NFBackendExtension;
use crate::NFBinding as Binding;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnectionSets::ConnectionSets;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Visibility;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTypeCheck::MatchKind;
use crate::NFTyping as Typing;
use crate::NFVariable as Variable;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;

pub(crate) fn elaborate(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
    mut connections: metamodelica::Ref<Connections::NFConnections>,
) -> Result<(
    metamodelica::Ref<FlatModel::NFFlatModel>,
    metamodelica::Ref<Connections::NFConnections>,
)> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut connections: metamodelica::Ref<Connections::NFConnections> = connections;
    let mut expandable_conns: metamodelica::List<metamodelica::Ref<Connection::NFConnection>>;
    let mut undeclared_conns: metamodelica::List<metamodelica::Ref<Connection::NFConnection>>;
    let mut conns: metamodelica::List<metamodelica::Ref<Connection::NFConnection>>;
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut csets: ConnectionSets::Sets;
    let mut csets_array: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>;
    (expandable_conns, undeclared_conns, conns) = sortConnections(&connections.connections)?;
    if (expandable_conns).is_empty() && (undeclared_conns).is_empty() {
        return Ok((flatModel, connections));
    }
    csets = ConnectionSets::emptySets(((expandable_conns).len() as i32) + ((undeclared_conns).len() as i32));
    csets = addExpandableConnectorsToSets(&expandable_conns, csets)?;
    (undeclared_conns, csets) = List::mapFold(&undeclared_conns, &addExpandableConnectorElementToSets, csets)?;
    (csets_array, _) = ConnectionSets::extractSets(&csets)?;
    vars = flatModel.variables.clone();
    let __range0 = csets_array.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut set in __range0 {
        vars = elaborateExpandableSet(&set, vars)?;
    }
    conns = List::fold(
        &undeclared_conns,
        &fnptr!(
            updateUndeclaredConnection,
            metamodelica::Ref<Connection::NFConnection>,
            metamodelica::List<metamodelica::Ref<Connection::NFConnection>>
        ),
        conns,
    )?;
    conns = List::fold(
        &expandable_conns,
        &move |__a0: metamodelica::Ref<Connection::NFConnection>,
               __a1: metamodelica::List<metamodelica::Ref<Connection::NFConnection>>| {
            updateExpandableConnection(&__a0, __a1)
        },
        conns,
    )?;
    assign_field!(connections.connections = conns);
    vars = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
        for mut v in (vars).into_iter().cloned() {
            let __x = updatePotentiallyPresentVariable(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    assign_field!(flatModel.variables = vars);
    Ok((flatModel, connections))
}

fn sortConnections(
    mut conns: &metamodelica::List<metamodelica::Ref<Connection::NFConnection>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Connection::NFConnection>>,
    metamodelica::List<metamodelica::Ref<Connection::NFConnection>>,
    metamodelica::List<metamodelica::Ref<Connection::NFConnection>>,
)> {
    let mut expandableConnections: metamodelica::List<metamodelica::Ref<Connection::NFConnection>> =
        metamodelica::nil();
    let mut undeclaredConnections: metamodelica::List<metamodelica::Ref<Connection::NFConnection>> =
        metamodelica::nil();
    let mut normalConnections: metamodelica::List<metamodelica::Ref<Connection::NFConnection>> = metamodelica::nil();
    let mut c1: metamodelica::Ref<Connector::NFConnector>;
    let mut c2: metamodelica::Ref<Connector::NFConnector>;
    let mut is_undeclared1: bool;
    let mut is_undeclared2: bool;
    let mut is_expandable1: bool;
    let mut is_expandable2: bool;
    for mut conn in &**conns {
        let __arc2 = conn.clone();
        let Connection::CONNECTION { lhs: __pa0, rhs: __pa1 } = &*__arc2;
        c1 = metamodelica::Own::own(__pa0);
        c2 = metamodelica::Own::own(__pa1);
        is_undeclared1 = Prefixes::ConnectorType::isUndeclared(c1.cty.clone())
            || Prefixes::ConnectorType::isPotentiallyPresent(c1.cty.clone());
        is_undeclared2 = Prefixes::ConnectorType::isUndeclared(c2.cty.clone())
            || Prefixes::ConnectorType::isPotentiallyPresent(c2.cty.clone());
        is_expandable1 = Prefixes::ConnectorType::isExpandable(c1.cty.clone());
        is_expandable2 = Prefixes::ConnectorType::isExpandable(c2.cty.clone());
        if is_expandable1 || is_expandable2 {
            if is_expandable1 && is_expandable2 {
                expandableConnections = metamodelica::cons(conn.clone(), expandableConnections);
            } else {
                Error::addSourceMessageAndFail(
                    &(Error::EXPANDABLE_NON_EXPANDABLE_CONNECTION.clone()),
                    list![
                        Connector::toString(&(if (is_expandable1) { c1.clone() } else { c2.clone() }))?,
                        Connector::toString(&(if (is_expandable1) { c2 } else { c1.clone() }))?
                    ],
                    &(Connector::getInfo(&c1)),
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
        } else if is_undeclared1 || is_undeclared2 {
            if is_undeclared1 && is_undeclared2 {
                Error::addSourceMessageAndFail(
                    &(Error::UNDECLARED_CONNECTION.clone()),
                    list![Connector::toString(&c1)?, Connector::toString(&c2)?],
                    &(Connector::getInfo(&c1)),
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            } else {
                undeclaredConnections = metamodelica::cons(conn.clone(), undeclaredConnections);
            }
        } else {
            normalConnections = metamodelica::cons(conn.clone(), normalConnections);
        }
    }
    normalConnections = metamodelica::Dangerous::listReverseInPlace(normalConnections);
    Ok((expandableConnections, undeclaredConnections, normalConnections))
}

fn addExpandableConnectorsToSets(
    mut conns: &metamodelica::List<metamodelica::Ref<Connection::NFConnection>>,
    mut csets: ConnectionSets::Sets,
) -> Result<ConnectionSets::Sets> {
    let mut csets: ConnectionSets::Sets = csets;
    let mut c1: metamodelica::Ref<Connector::NFConnector>;
    let mut c2: metamodelica::Ref<Connector::NFConnector>;
    for mut conn in &**conns {
        let __arc2 = conn.clone();
        let Connection::CONNECTION { lhs: __pa0, rhs: __pa1 } = &*__arc2;
        c1 = metamodelica::Own::own(__pa0);
        c2 = metamodelica::Own::own(__pa1);
        csets = addConnectionToSets(c1.clone(), c2.clone(), csets)?;
        csets = addNestedExpandableConnectorsToSets(&c1, &c2, csets)?;
    }
    Ok(csets)
}

fn addNestedExpandableConnectorsToSets(
    mut c1: &metamodelica::Ref<Connector::NFConnector>,
    mut c2: &metamodelica::Ref<Connector::NFConnector>,
    mut csets: ConnectionSets::Sets,
) -> Result<ConnectionSets::Sets> {
    let mut csets: ConnectionSets::Sets = csets;
    let mut ecl1: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut ecl2: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut oec: Option<metamodelica::Ref<Connector::NFConnector>>;
    let mut conns: metamodelica::List<metamodelica::Ref<Connection::NFConnection>> = metamodelica::nil();
    ecl1 = getExpandableConnectorsInConnector(c1)?;
    ecl2 = getExpandableConnectorsInConnector(c2)?;
    if (ecl1).is_empty() && (ecl2).is_empty() {
        return Ok(csets);
    }
    for mut ec1 in &*ecl1 {
        (ecl2, oec) = List::deleteMemberOnTrue(ec1.clone(), ecl2, &move |__a0: metamodelica::Ref<
            Connector::NFConnector,
        >,
                                                                         __a1: metamodelica::Ref<
            Connector::NFConnector,
        >| {
            Connector::isNodeNameEqual(&__a0, &__a1)
        })?;
        if (oec).is_some() {
            conns = metamodelica::cons(
                metamodelica::Ref::new(Connection::NFConnection {
                    lhs: ec1.clone(),
                    rhs: oec.ok_or("pattern mismatch")?,
                }),
                conns,
            );
        }
    }
    csets = addExpandableConnectorsToSets(&conns, csets)?;
    Ok(csets)
}

fn getExpandableConnectorsInConnector(
    mut c1: &metamodelica::Ref<Connector::NFConnector>,
) -> Result<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>> {
    let mut ecl: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut nodes: metamodelica::List<Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>>;
    let mut n: metamodelica::Ref<InstNode::InstNode>;
    let mut par_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    ecl = (::match_deref::match_deref! { match c1 {
        Deref @ Connector::CONNECTOR { name: __esc_par_name, ty: Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXPANDABLE_CONNECTOR { expandableConnectors: __esc_nodes, .. }, .. }, .. } => {
            par_name = (*__esc_par_name).clone();
            nodes = (*__esc_nodes).clone();
            ecl = metamodelica::nil();
            for mut n_ref in &*nodes.clone() {
                n = NFInstNode::InstNode::borrow(n_ref.clone())?;
                ty = NFInstNode::InstNode::getType(n.clone())?;
                name = ComponentRef::prefixCref(n.clone(), ty.clone(), metamodelica::nil(), par_name.clone())?;
                ecl = metamodelica::cons(Connector::fromCref(name, ty, ElementSource::createElementSource(NFInstNode::InstNode::info(&n), None, &(openmodelica_frontend_types::DAE::Prefix::NOPRE), (DAE::emptyCref().clone(), DAE::emptyCref().clone())))?, ecl);
            }
            ecl
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ecl)
}

fn addExpandableConnectorElementToSets(
    mut conn: metamodelica::Ref<Connection::NFConnection>,
    mut csets: ConnectionSets::Sets,
) -> Result<(metamodelica::Ref<Connection::NFConnection>, ConnectionSets::Sets)> {
    let mut conn: metamodelica::Ref<Connection::NFConnection> = conn;
    let mut csets: ConnectionSets::Sets = csets;
    let mut c1: metamodelica::Ref<Connector::NFConnector>;
    let mut c2: metamodelica::Ref<Connector::NFConnector>;
    let mut c: metamodelica::Ref<Connector::NFConnector>;
    let mut ec: metamodelica::Ref<Connector::NFConnector>;
    let __arc2 = conn.clone();
    let Connection::CONNECTION { lhs: __pa0, rhs: __pa1 } = &*__arc2;
    c1 = metamodelica::Own::own(__pa0);
    c2 = metamodelica::Own::own(__pa1);
    if Prefixes::ConnectorType::isUndeclared(c1.cty.clone()) {
        c1 = makeVirtualConnector(&c1, &c2)?;
        conn = metamodelica::Ref::new(Connection::NFConnection {
            lhs: c1.clone(),
            rhs: c2,
        });
        c = c1;
    } else if Prefixes::ConnectorType::isUndeclared(c2.cty.clone()) {
        c2 = makeVirtualConnector(&c2, &c1)?;
        conn = metamodelica::Ref::new(Connection::NFConnection {
            lhs: c1,
            rhs: c2.clone(),
        });
        c = c2;
    } else {
        c = if (Prefixes::ConnectorType::isPotentiallyPresent(c1.cty.clone())) {
            c1
        } else {
            c2
        };
    }
    ec = metamodelica::Ref::new(Connector::NFConnector {
        name: ComponentRef::rest(&c.name)?,
        ty: c.ty.clone(),
        face: c.face.clone(),
        cty: ConnectorType::EXPANDABLE.clone(),
        source: c.source.clone(),
    });
    csets = addConnectionToSets(c, ec, csets)?;
    Ok((conn, csets))
}

fn addConnectionToSets(
    mut c1: metamodelica::Ref<Connector::NFConnector>,
    mut c2: metamodelica::Ref<Connector::NFConnector>,
    mut csets: ConnectionSets::Sets,
) -> Result<ConnectionSets::Sets> {
    let mut csets: ConnectionSets::Sets = csets;
    csets = ConnectionSets::merge(Connector::setOutside(c1), Connector::setOutside(c2), csets)?;
    Ok(csets)
}

fn makeVirtualConnector(
    mut virtualConnector: &metamodelica::Ref<Connector::NFConnector>,
    mut normalConnector: &metamodelica::Ref<Connector::NFConnector>,
) -> Result<metamodelica::Ref<Connector::NFConnector>> {
    let mut newConnector: metamodelica::Ref<Connector::NFConnector>;
    let mut virtual_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut normal_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    virtual_cref = virtualConnector.name.clone();
    normal_cref = normalConnector.name.clone();
    ty = normalConnector.ty.clone();
    node = ComponentRef::node(&normal_cref)?;
    node = NFInstNode::InstNode::clone(node)?;
    node = NFInstNode::InstNode::rename(ComponentRef::firstName(&virtual_cref, false)?, node)?;
    node = NFInstNode::InstNode::setParent(ComponentRef::node(&(ComponentRef::rest(&virtual_cref)?))?, node)?;
    node = NFInstNode::InstNode::componentApply(node, &Component::setType, ty.clone())?;
    virtual_cref = ComponentRef::prefixCref(
        node,
        ty.clone(),
        metamodelica::nil(),
        ComponentRef::rest(&virtual_cref)?,
    )?;
    newConnector = metamodelica::Ref::new(Connector::NFConnector {
        name: virtual_cref,
        ty: ty,
        face: virtualConnector.face.clone(),
        cty: virtualConnector.cty.clone(),
        source: virtualConnector.source.clone(),
    });
    Ok(newConnector)
}

fn elaborateExpandableSet(
    mut set: &metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<metamodelica::List<metamodelica::Ref<Variable::NFVariable>>> {
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = vars;
    let mut exp_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Connector::NFConnector>>>;
    let mut exp_conns: metamodelica::List<metamodelica::Ref<Connector::NFConnector>> = metamodelica::nil();
    let mut exp_set_lst: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    exp_set = UnorderedSet::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<Connector::NFConnector>| hashConnector(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Connector::NFConnector>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Connector::NFConnector>, __a1: metamodelica::Ref<Connector::NFConnector>| {
                Connector::isNodeNameEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Connector::NFConnector>,
                        metamodelica::Ref<Connector::NFConnector>,
                    ) -> Result<bool>
                    + 'static,
            >),
        13,
    );
    for mut c in &**set {
        if Prefixes::ConnectorType::isExpandable(c.cty.clone()) {
            exp_conns = metamodelica::cons(c.clone(), exp_conns);
        } else if Prefixes::ConnectorType::isUndeclared(c.cty.clone())
            || Prefixes::ConnectorType::isPotentiallyPresent(c.cty.clone())
        {
            UnorderedSet::add(c.clone(), exp_set.clone())?;
            markComponentPresent(ComponentRef::node(&(Connector::name(metamodelica::AsArg::as_arg(&c))))?)?;
        }
    }
    exp_set_lst = UnorderedSet::toList(exp_set);
    for mut ec in &*exp_conns {
        vars = augmentExpandableConnector(metamodelica::AsArg::as_arg(&ec), &exp_set_lst, vars)?;
    }
    Ok(vars)
}

fn markComponentPresent(mut node: metamodelica::Ref<InstNode::InstNode>) -> Result<()> {
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut cty: i32;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    comp = NFInstNode::InstNode::component(&node)?;
    cty = Component::connectorType(&comp);
    if Prefixes::ConnectorType::isPotentiallyPresent(cty) {
        cty = Prefixes::ConnectorType::setPresent(cty);
        comp = Component::setConnectorType(cty, comp);
        NFInstNode::InstNode::updateComponent(comp.clone(), node)?;
        if Type::isComplex(&(Component::getType(&comp)?)) {
            cls = NFInstNode::InstNode::getClass(Component::classInstance(&comp)?)?;
            ClassTree::applyComponents(&(Class::classTree(cls)?), &markComponentPresent)?;
        }
    }
    Ok(())
}

fn augmentExpandableConnector(
    mut conn: &metamodelica::Ref<Connector::NFConnector>,
    mut expandableSet: &metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<metamodelica::List<metamodelica::Ref<Variable::NFVariable>>> {
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = vars;
    let mut exp_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut elem_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut exp_node: metamodelica::Ref<InstNode::InstNode>;
    let mut comp_node: metamodelica::Ref<InstNode::InstNode>;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    let mut nodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut complex_ty: metamodelica::Ref<ComplexType::NFComplexType>;
    exp_name = Connector::name(conn);
    exp_node = ComponentRef::node(&exp_name)?;
    if NFInstNode::InstNode::isName(&exp_node) {
        Error::addInternalError(
            literal!("Augmenting a virtual element in an expandable connector is not yet supported."),
            Connector::getInfo(conn),
        )?;
        return Err("fail");
    }
    cls_node = NFInstNode::InstNode::classScope(exp_node.clone())?;
    cls_node = NFInstNode::InstNode::clone(cls_node)?;
    cls = NFInstNode::InstNode::getClass(cls_node.clone())?;
    cls_tree = Class::classTree(cls.clone())?;
    for mut c in &**expandableSet {
        elem_name = Connector::name(metamodelica::AsArg::as_arg(&c));
        node = ComponentRef::node(&elem_name)?;
        match '__try0: {
            (comp_node, _) = unwrap_break_err!(ClassTree::lookupElement(unwrap_break_err!(NFInstNode::InstNode::name(&node), '__try0), &cls_tree), '__try0);
            Ok::<_, &'static str>((comp_node.clone(),))
        } {
            Ok((__try0_o0,)) => {
                comp_node = __try0_o0;
            }
            Err(_) => {
                comp_node = crate::NFInstNode::InstNode::interned_EMPTY_NODE();
            }
        }
        if NFInstNode::InstNode::isEmpty(&comp_node) {
            nodes = metamodelica::cons(node.clone(), nodes);
            ty = c.ty.clone();
            elem_name = ComponentRef::prefixCref(node, ty.clone(), metamodelica::nil(), exp_name.clone())?;
            vars = createVirtualVariables(elem_name, ty, ElementSource::getInfo(c.source.clone()), vars)?;
        } else {
            comp_node = NFInstNode::InstNode::resolveInner(comp_node);
            if NFInstNode::InstNode::isComponent(&comp_node)? {
                markComponentPresent(comp_node.clone())?;
            } else {
                Error::addInternalError(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFExpandableConnectors.augmentExpandableConnector"));
                        __mm_s.push_str(&*literal!(" got non-component element"));
                        ArcStr::from(__mm_s)
                    },
                    metamodelica::sourceInfo!("NFFrontEnd/NFExpandableConnectors.mo"),
                )?;
            }
        }
    }
    if !((nodes).is_empty()) {
        cls_tree = ClassTree::addElementsToFlatTree(&nodes, cls_tree)?;
        cls = Class::setClassTree(cls_tree.clone(), cls)?;
    }
    complex_ty = Typing::makeConnectorType(&cls_tree, false)?;
    ty = metamodelica::Ref::new(Type::NFType::COMPLEX {
        cls: NFInstNode::InstNode::identityCell(cls_node.clone()),
        complexTy: complex_ty,
    });
    ty = Type::liftArrayLeftList(ty, &(Type::arrayDims(NFInstNode::InstNode::getType(exp_node.clone())?)));
    cls = Class::setType(ty.clone(), cls)?;
    NFInstNode::InstNode::updateClass(cls, cls_node)?;
    NFInstNode::InstNode::componentApply(exp_node, &Component::setType, ty)?;
    Ok(vars)
}

fn createVirtualVariables(
    mut connectorName: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut connectorType: metamodelica::Ref<Type::NFType>,
    mut info: SourceInfo,
    mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<metamodelica::List<metamodelica::Ref<Variable::NFVariable>>> {
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = vars;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    if Type::isComplex(&connectorType) {
        let __range0 = Type::complexComponents(&connectorType)?
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for mut comp in __range0 {
            ty = NFInstNode::InstNode::getType(comp.clone())?;
            name = ComponentRef::prefixCref(comp, ty.clone(), metamodelica::nil(), connectorName.clone())?;
            vars = createVirtualVariables(name, ty, info.clone(), vars)?;
        }
    } else {
        var = metamodelica::Ref::new(Variable::NFVariable {
            name: connectorName,
            ty: connectorType,
            binding: Binding::EMPTY_BINDING().clone(),
            visibility: Visibility::PUBLIC.clone(),
            attributes: NFAttributes::AUGMENTED_ATTR().clone(),
            typeAttributes: metamodelica::nil(),
            children: metamodelica::nil(),
            comment: metamodelica::Ref::new(SCode::Comment {
                annotation_: None,
                comment: Some(literal!("variable added to expandable connector")),
            }),
            info: info,
            backendinfo: NFBackendExtension::DUMMY_BACKEND_INFO().clone(),
        });
        vars = metamodelica::cons(var, vars);
    }
    Ok(vars)
}

fn updateUndeclaredConnection(
    mut conn: metamodelica::Ref<Connection::NFConnection>,
    mut conns: metamodelica::List<metamodelica::Ref<Connection::NFConnection>>,
) -> metamodelica::List<metamodelica::Ref<Connection::NFConnection>> {
    let mut conns: metamodelica::List<metamodelica::Ref<Connection::NFConnection>> = conns;
    conns = metamodelica::cons(conn, conns);
    conns
}

fn updateExpandableConnection(
    mut conn: &metamodelica::Ref<Connection::NFConnection>,
    mut conns: metamodelica::List<metamodelica::Ref<Connection::NFConnection>>,
) -> Result<metamodelica::List<metamodelica::Ref<Connection::NFConnection>>> {
    let mut conns: metamodelica::List<metamodelica::Ref<Connection::NFConnection>> = conns;
    let mut c1: metamodelica::Ref<Connector::NFConnector>;
    let mut c2: metamodelica::Ref<Connector::NFConnector>;
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut mk: MatchKind;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let __arc2 = &(*conn);
    let Connection::CONNECTION { lhs: __pa0, rhs: __pa1 } = &**__arc2;
    c1 = metamodelica::Own::own(__pa0);
    c2 = metamodelica::Own::own(__pa1);
    (c1, ty1) = updateExpandableConnector(c1)?;
    (c2, ty2) = updateExpandableConnector(c2)?;
    e1 = metamodelica::Ref::new(Expression::NFExpression::CREF {
        ty: ty1.clone(),
        cref: Connector::name(&c1),
    });
    e2 = metamodelica::Ref::new(Expression::NFExpression::CREF {
        ty: ty2.clone(),
        cref: Connector::name(&c2),
    });
    (_, _, _, mk) = TypeCheck::matchExpressions(e1.clone(), ty1, e2.clone(), ty2, TypeCheck::ALLOW_UNKNOWN.clone())?;
    if TypeCheck::isIncompatibleMatch(mk) {
        Error::addSourceMessageAndFail(
            &(Error::CONNECT_TYPE_MISMATCH.clone()),
            list![Expression::toString(e1)?, Expression::toString(e2)?],
            &(Connector::getInfo(&c1)),
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    conns = metamodelica::cons(
        metamodelica::Ref::new(Connection::NFConnection { lhs: c1, rhs: c2 }),
        conns,
    );
    Ok(conns)
}

fn updateExpandableConnector(
    mut conn: metamodelica::Ref<Connector::NFConnector>,
) -> Result<(
    metamodelica::Ref<Connector::NFConnector>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut conn: metamodelica::Ref<Connector::NFConnector> = conn;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let __arc2 = conn.clone();
    let Connector::CONNECTOR {
        name: __pa0, ty: __pa1, ..
    } = &*__arc2;
    name = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    name = ComponentRef::updateNodeType(name)?;
    ty = Type::setArrayElementType(&ty, &(Type::arrayElementType(&(ComponentRef::nodeType(&name)?))));
    conn = metamodelica::Ref::new(Connector::NFConnector {
        name: name,
        ty: ty.clone(),
        face: conn.face.clone(),
        cty: conn.cty.clone(),
        source: conn.source.clone(),
    });
    Ok((conn, ty))
}

fn updatePotentiallyPresentVariable(
    mut var: metamodelica::Ref<Variable::NFVariable>,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    if Prefixes::ConnectorType::isPotentiallyPresent(var.attributes.connectorType.clone()) {
        assign_field!(
            var.attributes =
                Component::getAttributes(&(NFInstNode::InstNode::component(&(ComponentRef::node(&var.name)?))?))
        );
    }
    Ok(var)
}

fn hashConnector(mut conn: &metamodelica::Ref<Connector::NFConnector>) -> Result<i32> {
    let mut res: i32;
    res = stringHashDjb2(&(ComponentRef::firstName(&conn.name, false)?));
    Ok(res)
}
