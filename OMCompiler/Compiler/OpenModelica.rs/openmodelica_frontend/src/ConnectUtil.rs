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

use crate::ConnectionGraph;
use crate::InnerOuter;
use crate::Lookup;
use crate::PrefixUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE::Connect::ConnectorElement;
use openmodelica_frontend_types::DAE::Connect::ConnectorType;
use openmodelica_frontend_types::DAE::Connect::Face;
use openmodelica_frontend_types::DAE::Connect::OuterConnect;
use openmodelica_frontend_types::DAE::Connect::Set;
use openmodelica_frontend_types::DAE::Connect::SetConnection;
use openmodelica_frontend_types::DAE::Connect::SetTrie;
use openmodelica_frontend_types::DAE::Connect::SetTrieNode;
use openmodelica_frontend_types::DAE::Connect::Sets;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
// Import some types from Connect.
// Set graph represented as an adjacency list.
pub type SetGraph = metamodelica::Array<metamodelica::List<i32>>;

pub(crate) fn newSet(mut prefix: DAE::Prefix, mut sets: Sets) -> Sets {
    let mut sets: Sets = sets;
    let mut pstr: ArcStr;
    let mut sc: i32;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let Sets { setCount: __pa0, .. } = sets;
    sc = metamodelica::Own::own(__pa0);
    match '__try1: {
        cr = unwrap_break_err!(PrefixUtil::prefixFirstCref(prefix.clone()), '__try1);
        pstr = unwrap_break_err!(ComponentReferenceBasics::printComponentRefStr(&cr), '__try1);
        Ok::<_, &'static str>((cr.clone(), pstr.clone()))
    } {
        Ok((__try1_o0, __try1_o1)) => {
            cr = __try1_o0;
            pstr = __try1_o1;
        }
        Err(_) => {
            cr = openmodelica_frontend_types::DAE::ComponentRef::interned_WILD();
            pstr = literal!("");
        }
    }
    sets = Sets {
        sets: metamodelica::Ref::new(SetTrieNode::SET_TRIE_NODE {
            name: pstr,
            cref: cr,
            nodes: metamodelica::nil(),
            connectCount: 0,
        }),
        setCount: sc,
        connections: metamodelica::nil(),
        outerConnects: metamodelica::nil(),
    };
    sets
}

pub(crate) fn addSet(mut parentSets: Sets, mut childSets: Sets) -> Result<Sets> {
    let mut sets: Sets;
    sets = 'mc: {
        let __mc_input = (&parentSets, &childSets);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    if !((isEmptySet(&childSets))) { return Err("guard") }
                    Ok(parentSets.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Sets { sets: Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { cref: Deref @ DAE::ComponentRef::WILD { .. }, .. }, .. }, Sets { sets: Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { cref: Deref @ DAE::ComponentRef::WILD { .. }, .. }, .. }) => {
                    Ok(childSets.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Sets { sets: node @ Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. }, .. }, Sets { .. }) => {
                    setTrieGetNode(setTrieNodeName(&childSets.sets), var_field!((**node).nodes, SetTrieNode::SET_TRIE_NODE))?;
                    Ok(parentSets.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Sets { sets: node @ Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. }, setCount: _, connections: c1, outerConnects: o1 }, Sets { sets: _, setCount: sc, connections: c2, outerConnects: o2 }) => {
                    let mut node = (*node).clone();
                    let mut c1 = (*c1).clone();
                    let mut o1 = (*o1).clone();
                    c1 = listAppend(c2.clone(), c1.clone());
                    o1 = listAppend(o2.clone(), o1.clone());
                    assign_variant_field!(node => SetTrieNode::SET_TRIE_NODE; nodes = metamodelica::cons(childSets.sets.clone(), var_field!((*node).nodes, SetTrieNode::SET_TRIE_NODE).clone()));
                    Ok(Sets { sets: node.clone(), setCount: sc.clone(), connections: c1.clone(), outerConnects: o1.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(sets)
}

fn isEmptySet(mut sets: &Sets) -> bool {
    let mut isEmpty: bool;
    isEmpty = (::match_deref::match_deref! { match &(sets) {
        Sets { sets: Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { nodes: Deref @ metamodelica::ListNode::Nil, .. }, connections: Deref @ metamodelica::ListNode::Nil, outerConnects: Deref @ metamodelica::ListNode::Nil, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub(crate) fn addConnection(
    mut sets: Sets,
    mut cref1: metamodelica::Ref<DAE::ComponentRef>,
    mut face1: Face,
    mut cref2: metamodelica::Ref<DAE::ComponentRef>,
    mut face2: Face,
    mut connectorType: &metamodelica::Ref<DAE::ConnectorType>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut e1: ConnectorElement;
    let mut e2: ConnectorElement;
    let mut ty: ConnectorType;
    ty = makeConnectorType(connectorType)?;
    e1 = findElement(cref1, face1, ty.clone(), source.clone(), &sets);
    e2 = findElement(cref2, face2, ty, source, &sets);
    sets = mergeSets(e1, e2, sets)?;
    Ok(sets)
}

fn getConnectCount(mut cref: &metamodelica::Ref<DAE::ComponentRef>, mut trie: &metamodelica::Ref<SetTrieNode>) -> i32 {
    let mut count: i32;
    let mut node: metamodelica::Ref<SetTrieNode>;
    match '__try0: {
        node = unwrap_break_err!(setTrieGet(cref, trie, false), '__try0);
        count = (match &*node {
            DAE::Connect::SetTrieNode::SET_TRIE_NODE {
                connectCount: __node_connectCount,
                ..
            } => __node_connectCount.clone(),
            DAE::Connect::SetTrieNode::SET_TRIE_LEAF {
                connectCount: __node_connectCount,
                ..
            } => __node_connectCount.clone(),
        });
        Ok::<_, &'static str>((count.clone(),))
    } {
        Ok((__try0_o0,)) => {
            count = __try0_o0;
        }
        Err(_) => {
            count = 0;
        }
    }
    count
}

pub(crate) fn addArrayConnection(
    mut sets: Sets,
    mut cref1: &metamodelica::Ref<DAE::ComponentRef>,
    mut face1: Face,
    mut cref2: &metamodelica::Ref<DAE::ComponentRef>,
    mut face2: Face,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut connectorType: &metamodelica::Ref<DAE::ConnectorType>,
) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut crefs1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut crefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
    crefs1 = ComponentReference::expandCref(cref1, false)?;
    crefs2 = ComponentReference::expandCref(cref2, false)?;
    for mut cr1 in &*crefs1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crefs2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cr2 = metamodelica::Own::own(__pa0);
        crefs2 = metamodelica::Own::own(__pa1);
        sets = addConnection(sets, cr1.clone(), face1, cr2, face2, connectorType, source.clone())?;
    }
    Ok(sets)
}

fn makeConnectorType(mut connectorType: &metamodelica::Ref<DAE::ConnectorType>) -> Result<ConnectorType> {
    let mut ty: ConnectorType;
    let mut flowName: Option<metamodelica::Ref<DAE::ComponentRef>>;
    ty = (match &**connectorType {
        DAE::ConnectorType::POTENTIAL { .. } => openmodelica_frontend_types::DAE::Connect::ConnectorType::EQU,
        DAE::ConnectorType::FLOW { .. } => openmodelica_frontend_types::DAE::Connect::ConnectorType::FLOW,
        DAE::ConnectorType::STREAM {
            associatedFlow: __esc_flowName,
        } => {
            flowName = (*__esc_flowName).clone();
            ConnectorType::STREAM {
                associatedFlow: flowName.clone(),
            }
        }
        DAE::ConnectorType::NON_CONNECTOR { .. } => openmodelica_frontend_types::DAE::Connect::ConnectorType::NO_TYPE,
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!("ConnectUtil.makeConnectorType: invalid connector type.")],
            )?;
            return Err("fail");
        }
    });
    Ok(ty)
}

pub(crate) fn addConnectorVariablesFromDAE(
    mut ignore: bool,
    mut classState: &ClassInf::State,
    mut prefix: DAE::Prefix,
    mut vars: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut info: &SourceInfo,
    mut elementSource: metamodelica::Ref<DAE::ElementSource>,
    mut sets: Sets,
) -> Result<Sets> {
    let mut sets: Sets = sets;
    sets = (match classState.clone() {
        ClassInf::State::CONNECTOR {
            path: ref class_path,
            isExpandable: false,
        } if (!(ignore)) => {
            let mut streams: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            let mut flows: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            checkConnectorBalance(vars, class_path.clone(), info)?;
            if !(Flags::isSet(Flags::DISABLE_SINGLE_FLOW_EQ.clone())?) {
                (flows, streams) = getStreamAndFlowVariables(vars);
                sets = List::fold2(
                    &flows,
                    &move |__a0: metamodelica::Ref<DAE::Var>,
                           __a1: metamodelica::Ref<DAE::ElementSource>,
                           __a2: DAE::Prefix,
                           __a3: Sets| addFlowVariableFromDAE(&__a0, __a1, &__a2, __a3),
                    elementSource,
                    prefix.clone(),
                    sets,
                )?;
                sets = addStreamFlowAssociations(sets, prefix, &streams, &flows)?;
            }
            sets
        }
        _ => sets,
    });
    Ok(sets)
}

fn addFlowVariableFromDAE(
    mut variable: &metamodelica::Ref<DAE::Var>,
    mut elementSource: metamodelica::Ref<DAE::ElementSource>,
    mut prefix: &DAE::Prefix,
    mut sets: Sets,
) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    crefs = daeVarToCrefs(variable)?;
    for mut cr in &*crefs {
        sets = addInsideFlowVariable(sets, cr.clone(), elementSource.clone(), prefix)?;
    }
    Ok(sets)
}

pub(crate) fn isExpandable(mut name: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut expandableConnector: bool;
    expandableConnector = (match &**name {
        DAE::ComponentRef::CREF_IDENT {
            identType: __name_identType,
            ..
        } => Types::isExpandableConnector(metamodelica::AsArg::as_arg(&__name_identType)),
        DAE::ComponentRef::CREF_QUAL {
            componentRef: __name_componentRef,
            identType: __name_identType,
            ..
        } => {
            Types::isExpandableConnector(metamodelica::AsArg::as_arg(&__name_identType))
                || isExpandable(metamodelica::AsArg::as_arg(&__name_componentRef))
        }
        _ => false,
    });
    expandableConnector
}

fn daeHasExpandableConnectors(mut DAE: DAE::DAElist) -> Result<bool> {
    let mut hasExpandable: bool;
    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    if System::getHasExpandableConnectors() {
        let DAE::DAE { elementLst: __pa0 } = DAE;
        vars = metamodelica::Own::own(__pa0);
        hasExpandable = List::any(&vars, &move |__a0: metamodelica::Ref<DAE::Element>| {
            isVarExpandable(&__a0)
        })?;
    } else {
        hasExpandable = false;
    }
    Ok(hasExpandable)
}

fn isVarExpandable(mut var: &metamodelica::Ref<DAE::Element>) -> Result<bool> {
    let mut isExpandable: bool;
    isExpandable = (match &**var {
        DAE::Element::VAR {
            componentRef: __var_componentRef,
            ..
        } => self::isExpandable(metamodelica::AsArg::as_arg(&__var_componentRef)),
        _ => false,
    });
    Ok(isExpandable)
}

fn getExpandableVariablesWithNoBinding(
    mut variables: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> {
    let mut potential: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    for mut var in &**variables {
        let () = (::match_deref::match_deref! { match &(var.clone()) {
            Deref @ DAE::Element::VAR { componentRef: __esc_name, binding: None, .. } => {
                name = (*__esc_name).clone();
                if isExpandable(metamodelica::AsArg::as_arg(&name)) {
                    potential = metamodelica::cons(name.clone(), potential);
                }
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    potential
}

fn getStreamAndFlowVariables(
    mut variables: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
) -> (
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
    metamodelica::List<metamodelica::Ref<DAE::Var>>,
) {
    let mut flows: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
    let mut streams: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
    for mut var in &**variables {
        let () = (::match_deref::match_deref! { match &(var.clone()) {
            Deref @ DAE::Var { attributes: Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::FLOW { .. }, .. }, .. } => {
                flows = metamodelica::cons(var.clone(), flows);
                ()
            },
            Deref @ DAE::Var { attributes: Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::STREAM { .. }, .. }, .. } => {
                streams = metamodelica::cons(var.clone(), streams);
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    (flows, streams)
}

fn addStreamFlowAssociations(
    mut sets: Sets,
    mut prefix: DAE::Prefix,
    mut streamVars: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut flowVars: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut flow_var: metamodelica::Ref<DAE::Var>;
    let mut flow_cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut stream_crs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    if (streamVars).is_empty() {
        return Ok(sets);
    }
    let __pa0 = ::match_deref::match_deref! { match &((*flowVars)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    flow_var = metamodelica::Own::own(__pa0);
    let __pa2 = ::match_deref::match_deref! { match &(daeVarToCrefs(&flow_var)?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } => __pa2.clone(),
        _ => return Err("pattern mismatch"),
    } };
    flow_cr = metamodelica::Own::own(__pa2);
    flow_cr = PrefixUtil::prefixCrefNoContext(prefix, flow_cr)?;
    for mut stream_var in &**streamVars {
        stream_crs = daeVarToCrefs(metamodelica::AsArg::as_arg(&stream_var))?;
        for mut stream_cr in &*stream_crs {
            sets = addStreamFlowAssociation(metamodelica::AsArg::as_arg(&stream_cr), flow_cr.clone(), sets)?;
        }
    }
    Ok(sets)
}

fn daeVarToCrefs(
    mut var: &metamodelica::Ref<DAE::Var>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut name: ArcStr;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut crs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let __arc2 = &(*var);
    let DAE::TYPES_VAR {
        name: __pa0, ty: __pa1, ..
    } = &**__arc2;
    name = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    ty = Types::derivedBasicType(&ty);
    crefs = (match &*ty {
        DAE::Type::T_REAL { .. } => list![metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
            ident: name,
            identType: ty,
            subscriptLst: metamodelica::nil()
        })],
        DAE::Type::T_COMPLEX {
            varLst: __ty_varLst, ..
        } => {
            crs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
                for mut v in (__ty_varLst.clone().reverse()).into_iter().cloned() {
                    let __x = daeVarToCrefs(&(v.clone()))?;
                    __acc = __x.append(&__acc);
                }
                __acc
            });
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: name,
                identType: DAE::T_REAL_DEFAULT().clone(),
                subscriptLst: metamodelica::nil(),
            });
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
                for mut c in (crs).into_iter().cloned() {
                    let __x = ComponentReference::joinCrefs(&cr, c.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
        DAE::Type::T_ARRAY { .. } => {
            dims = TypesDump::getDimensions(&ty);
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: name,
                identType: ty,
                subscriptLst: metamodelica::nil(),
            });
            expandArrayCref(&cr, &dims, &(metamodelica::nil()))
        }
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Unknown var "));
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!(" in ConnectUtil.daeVarToCrefs"));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("FrontEnd/ConnectUtil.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(crefs)
}

fn expandArrayCref(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut dims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut accumCrefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    crefs = 'mc: {
        let __mc_input = &**dims;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::cons(cref.clone(), accumCrefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: dim, tail: rest_dims } => {
                    let mut idx: metamodelica::Ref<DAE::Exp>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut dim = (*dim).clone();
                    (idx, dim) = getNextIndex(metamodelica::AsArg::as_arg(&dim))?;
                    cr = ComponentReference::subscriptCref(cref, list![metamodelica::Ref::new(DAE::Subscript::INDEX { exp: idx.clone() })])?;
                    crs = expandArrayCref(&cr, metamodelica::AsArg::as_arg(&rest_dims), accumCrefs);
                    crs = expandArrayCref(cref, &(metamodelica::cons(dim.clone(), rest_dims.clone())), &crs);
                    Ok(crs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(accumCrefs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    crefs
}

fn reverseEnumType(mut dim: metamodelica::Ref<DAE::Dimension>) -> metamodelica::Ref<DAE::Dimension> {
    let mut dim: metamodelica::Ref<DAE::Dimension> = dim;
    let () = (match &*dim {
        DAE::Dimension::DIM_ENUM {
            literals: __dim_literals,
            ..
        } => {
            assign_variant_field!(dim => DAE::Dimension::DIM_ENUM; literals = __dim_literals.clone().reverse());
            ()
        }
        _ => (),
    });
    dim
}

fn getNextIndex(
    mut dim: &metamodelica::Ref<DAE::Dimension>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Dimension>)> {
    let mut nextIndex: metamodelica::Ref<DAE::Exp>;
    let mut restDim: metamodelica::Ref<DAE::Dimension>;
    (nextIndex, restDim) = (::match_deref::match_deref! { match dim {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 } => {
            return Err("fail")
        },
        Deref @ DAE::Dimension::DIM_ENUM { size: 0, .. } => {
            return Err("fail")
        },
        Deref @ DAE::Dimension::DIM_INTEGER { integer: new_idx } => {
            let mut dim_size: i32;
            dim_size = new_idx.clone() - 1;
            (metamodelica::Ref::new(DAE::Exp::ICONST { integer: new_idx.clone() }), metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim_size }))
        },
        Deref @ DAE::Dimension::DIM_ENUM { enumTypeName: p, literals: Deref @ metamodelica::ListNode::Cons { head: l, tail: l_rest }, size: new_idx } => {
            let mut dim_size: i32;
            let mut ep: metamodelica::Ref<Absyn::Path>;
            ep = AbsynUtil::joinPaths(p.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: l.clone() }))?;
            dim_size = new_idx.clone() - 1;
            (metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL { name: ep, index: new_idx.clone() }), metamodelica::Ref::new(DAE::Dimension::DIM_ENUM { enumTypeName: p.clone(), literals: l_rest.clone(), size: dim_size }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((nextIndex, restDim))
}

fn addInsideFlowVariable(
    mut sets: Sets,
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut prefix: &DAE::Prefix,
) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut e: ConnectorElement;
    if '__try0: {
        unwrap_break_err!(setTrieGetElement(&cref, openmodelica_frontend_types::DAE::Connect::Face::INSIDE, &sets.sets), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
        sets.setCount = sets.setCount.clone() + 1;
        e = newElement(cref.clone(), openmodelica_frontend_types::DAE::Connect::Face::INSIDE, openmodelica_frontend_types::DAE::Connect::ConnectorType::FLOW, source.clone(), sets.setCount.clone());
        sets.sets = setTrieAdd(e.clone(), sets.sets.clone())?;
    }
    Ok(sets)
}

fn addStreamFlowAssociation(
    mut streamCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut flowCref: metamodelica::Ref<DAE::ComponentRef>,
    mut sets: Sets,
) -> Result<Sets> {
    let mut sets: Sets = sets;
    sets = updateSetLeaf(sets, streamCref, flowCref, &addStreamFlowAssociation2)?;
    Ok(sets)
}

fn addStreamFlowAssociation2(
    mut flowCref: metamodelica::Ref<DAE::ComponentRef>,
    mut node: metamodelica::Ref<SetTrieNode>,
) -> Result<metamodelica::Ref<SetTrieNode>> {
    let mut node: metamodelica::Ref<SetTrieNode> = node;
    let () = (match &*node {
        DAE::Connect::SetTrieNode::SET_TRIE_LEAF { .. } => {
            assign_variant_field!(node => SetTrieNode::SET_TRIE_LEAF; flowAssociation = Some(flowCref));
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(node)
}

fn getStreamFlowAssociation(
    mut streamCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut sets: &Sets,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut flowCref: metamodelica::Ref<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &(setTrieGet(streamCref, &sets.sets, false)?) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { flowAssociation: Some(__pa0), .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    flowCref = metamodelica::Own::own(__pa0);
    Ok(flowCref)
}

pub(crate) fn addOuterConnection(
    mut scope: DAE::Prefix,
    mut sets: Sets,
    mut cr1: metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: metamodelica::Ref<DAE::ComponentRef>,
    mut io1: Absyn::InnerOuter,
    mut io2: Absyn::InnerOuter,
    mut f1: Face,
    mut f2: Face,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut new_oc: OuterConnect;
    if !(List::any(
        &sets.outerConnects,
        &({
            let __pe_b1 = cr1.clone();
            let __pe_b2 = cr2.clone();
            move |__pe_a0| outerConnectionMatches(&__pe_a0, &__pe_b1, &__pe_b2)
        }),
    )?) {
        new_oc = OuterConnect {
            scope: scope,
            cr1: cr1,
            io1: io1,
            f1: f1,
            cr2: cr2,
            io2: io2,
            f2: f2,
            source: source,
        };
        sets.outerConnects = metamodelica::cons(new_oc, sets.outerConnects.clone());
    }
    Ok(sets)
}

fn outerConnectionMatches(
    mut oc: &OuterConnect,
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut matches: bool;
    matches = (match oc.clone() {
        OuterConnect { .. } => {
            ComponentReferenceBasics::crefEqual(&oc.cr1, cr1)? && ComponentReferenceBasics::crefEqual(&oc.cr2, cr2)?
                || ComponentReferenceBasics::crefEqual(&oc.cr1, cr2)?
                    && ComponentReferenceBasics::crefEqual(&oc.cr2, cr1)?
        }
    });
    Ok(matches)
}

pub(crate) fn addOuterConnectToSets(
    mut cref1: &metamodelica::Ref<DAE::ComponentRef>,
    mut cref2: &metamodelica::Ref<DAE::ComponentRef>,
    mut io1: Absyn::InnerOuter,
    mut io2: Absyn::InnerOuter,
    mut face1: Face,
    mut face2: Face,
    mut sets: Sets,
    mut inInfo: &SourceInfo,
) -> Result<(Sets, bool)> {
    let mut sets: Sets = sets;
    let mut added: bool;
    let mut is_outer1: bool;
    let mut is_outer2: bool;
    is_outer1 = AbsynUtil::isOuter(io1);
    is_outer2 = AbsynUtil::isOuter(io2);
    added = (match (is_outer1, is_outer2) {
        (true, true) => {
            Error::addSourceMessage(
                &(Error::UNSUPPORTED_LANGUAGE_FEATURE.clone()),
                list![
                    literal!("Connections where both connectors are outer references"),
                    literal!("No suggestion")
                ],
                inInfo,
            )?;
            false
        }
        (false, false) => false,
        (true, false) => {
            (sets, added) = addOuterConnectToSets2(cref1, cref2, face1, face2, sets);
            added
        }
        (false, true) => {
            (sets, added) = addOuterConnectToSets2(cref2, cref1, face2, face1, sets);
            added
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((sets, added))
}

fn addOuterConnectToSets2(
    mut outerCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut innerCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut outerFace: Face,
    mut innerFace: Face,
    mut sets: Sets,
) -> (Sets, bool) {
    let mut sets: Sets = sets;
    let mut added: bool;
    let mut node: metamodelica::Ref<SetTrieNode>;
    let mut outer_els: metamodelica::List<ConnectorElement>;
    let mut inner_els: metamodelica::List<ConnectorElement>;
    let mut sc: i32;
    match '__try0: {
        node = unwrap_break_err!(setTrieGet(outerCref, &sets.sets, true), '__try0);
        outer_els = unwrap_break_err!(collectOuterElements(&node, outerFace), '__try0);
        inner_els = ({
            let mut __acc: metamodelica::List<ConnectorElement> = metamodelica::nil();
            for mut oe in (outer_els.clone()).into_iter().cloned() {
                let __x = unwrap_break_err!(findInnerElement(oe.clone(), innerCref, innerFace, &sets), '__try0);
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        sc = sets.setCount.clone();
        sets = unwrap_break_err!(List::threadFold(&outer_els, inner_els.clone(), &mergeSets, sets.clone()), '__try0);
        added = sc != sets.setCount.clone();
        Ok::<_, &'static str>((added.clone(),))
    } {
        Ok((__try0_o0,)) => {
            added = __try0_o0;
        }
        Err(_) => {
            added = false;
        }
    }
    (sets, added)
}

fn collectOuterElements(
    mut node: &metamodelica::Ref<SetTrieNode>,
    mut face: Face,
) -> Result<metamodelica::List<ConnectorElement>> {
    let mut outerElements: metamodelica::List<ConnectorElement>;
    outerElements = (match &**node {
        DAE::Connect::SetTrieNode::SET_TRIE_NODE {
            nodes: __node_nodes, ..
        } => List::mapFlat(
            metamodelica::AsArg::as_arg(&__node_nodes),
            &({
                let __pe_b1 = face;
                let __pe_b2 = None;
                move |__pe_a0| collectOuterElements2(&__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            }),
        )?,
        _ => collectOuterElements2(node, face, None)?,
    });
    Ok(outerElements)
}

fn collectOuterElements2(
    mut node: &metamodelica::Ref<SetTrieNode>,
    mut face: Face,
    mut prefix: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<ConnectorElement>> {
    let mut outerElements: metamodelica::List<ConnectorElement>;
    outerElements = (match &**node {
        DAE::Connect::SetTrieNode::SET_TRIE_NODE {
            cref: cr,
            nodes: __node_nodes,
            ..
        } => {
            let mut cr = (*cr).clone();
            cr = optPrefixCref(prefix, cr.clone())?;
            List::mapFlat(
                metamodelica::AsArg::as_arg(&__node_nodes),
                &({
                    let __pe_b1 = face;
                    let __pe_b2 = Some(cr.clone());
                    move |__pe_a0| collectOuterElements2(&__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                }),
            )?
        }
        DAE::Connect::SetTrieNode::SET_TRIE_LEAF { .. } => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut e: ConnectorElement;
            e = setTrieGetLeafElement(node, face)?;
            cr = getElementName(e.clone());
            e = setElementName(e, optPrefixCref(prefix, cr)?);
            list![e]
        }
    });
    Ok(outerElements)
}

fn findInnerElement(
    mut outerElement: ConnectorElement,
    mut innerCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut innerFace: Face,
    mut sets: &Sets,
) -> Result<ConnectorElement> {
    let mut innerElement: ConnectorElement;
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    let mut ty: ConnectorType;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let ConnectorElement {
        name: __pa0,
        ty: __pa1,
        source: __pa2,
        ..
    } = outerElement;
    name = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    src = metamodelica::Own::own(__pa2);
    name = ComponentReference::joinCrefs(innerCref, name)?;
    innerElement = findElement(name, innerFace, ty, src, sets);
    Ok(innerElement)
}

fn optPrefixCref(
    mut prefix: Option<metamodelica::Ref<DAE::ComponentRef>>,
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut cref: metamodelica::Ref<DAE::ComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(prefix) {
        None => {
            cref
        },
        Some(cr) => {
            ComponentReference::joinCrefs(metamodelica::AsArg::as_arg(&cr), cref)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

fn findElement(
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut face: Face,
    mut ty: ConnectorType,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut sets: &Sets,
) -> ConnectorElement {
    let mut element: ConnectorElement;
    match '__try0: {
        element = unwrap_break_err!(setTrieGetElement(&cref, face, &sets.sets), '__try0);
        Ok::<_, &'static str>((element.clone(),))
    } {
        Ok((__try0_o0,)) => {
            element = __try0_o0;
        }
        Err(_) => {
            element = newElement(cref.clone(), face, ty.clone(), source.clone(), Connect::NEW_SET.clone());
        }
    }
    element
}

fn newElement(
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut face: Face,
    mut ty: ConnectorType,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut set: i32,
) -> ConnectorElement {
    let mut element: ConnectorElement;
    element = ConnectorElement {
        name: cref,
        face: face,
        ty: ty,
        source: source,
        set: set,
    };
    element
}

fn isNewElement(mut element: ConnectorElement) -> bool {
    let mut isNew: bool;
    let mut set: i32;
    let ConnectorElement { set: __pa0, .. } = element;
    set = metamodelica::Own::own(__pa0);
    isNew = set == Connect::NEW_SET.clone();
    isNew
}

fn getElementSetIndex(mut inElement: ConnectorElement) -> i32 {
    let mut outIndex: i32;
    let ConnectorElement { set: __pa0, .. } = inElement;
    outIndex = metamodelica::Own::own(__pa0);
    outIndex
}

fn setElementSetIndex(mut element: ConnectorElement, mut index: i32) -> ConnectorElement {
    let mut element: ConnectorElement = element;
    element.set = index;
    element
}

fn getElementName(mut element: ConnectorElement) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    let ConnectorElement { name: __pa0, .. } = element;
    name = metamodelica::Own::own(__pa0);
    name
}

fn setElementName(mut element: ConnectorElement, mut name: metamodelica::Ref<DAE::ComponentRef>) -> ConnectorElement {
    let mut element: ConnectorElement = element;
    element.name = name;
    element
}

fn getElementSource(mut element: ConnectorElement) -> metamodelica::Ref<DAE::ElementSource> {
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let ConnectorElement { source: __pa0, .. } = element;
    source = metamodelica::Own::own(__pa0);
    source
}

fn setTrieNewLeaf(mut id: ArcStr, mut element: ConnectorElement) -> Result<metamodelica::Ref<SetTrieNode>> {
    let mut leaf: metamodelica::Ref<SetTrieNode>;
    leaf = (match element.clone() {
        ConnectorElement {
            face: DAE::Connect::Face::INSIDE,
            ..
        } => metamodelica::Ref::new(SetTrieNode::SET_TRIE_LEAF {
            name: id,
            insideElement: Some(element),
            outsideElement: None,
            flowAssociation: None,
            connectCount: 0,
        }),
        ConnectorElement {
            face: DAE::Connect::Face::OUTSIDE,
            ..
        } => metamodelica::Ref::new(SetTrieNode::SET_TRIE_LEAF {
            name: id,
            insideElement: None,
            outsideElement: Some(element),
            flowAssociation: None,
            connectCount: 0,
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(leaf)
}

fn setTrieNewNode(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut element: &ConnectorElement,
) -> Result<metamodelica::Ref<SetTrieNode>> {
    let mut node: metamodelica::Ref<SetTrieNode>;
    node = (match &**cref {
        DAE::ComponentRef::CREF_IDENT { .. } => {
            let mut id: ArcStr;
            id = ComponentReferenceBasics::printComponentRefStr(cref)?;
            setTrieNewLeaf(id, setElementName(element.clone(), cref.clone()))?
        }
        DAE::ComponentRef::CREF_QUAL {
            componentRef: __cref_componentRef,
            ..
        } => {
            let mut id: ArcStr;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            cr = ComponentReferenceBasics::crefFirstCref(cref.clone())?;
            id = ComponentReferenceBasics::printComponentRefStr(&cr)?;
            node = setTrieNewNode(metamodelica::AsArg::as_arg(&__cref_componentRef), element)?;
            metamodelica::Ref::new(SetTrieNode::SET_TRIE_NODE {
                name: id,
                cref: cr,
                nodes: list![node],
                connectCount: 0,
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(node)
}

fn setTrieNodeName(mut node: &metamodelica::Ref<SetTrieNode>) -> ArcStr {
    let mut name: ArcStr;
    name = (match &**node {
        DAE::Connect::SetTrieNode::SET_TRIE_NODE { name: __node_name, .. } => __node_name.clone(),
        DAE::Connect::SetTrieNode::SET_TRIE_LEAF { name: __node_name, .. } => __node_name.clone(),
    });
    name
}

fn mergeSets(mut element1: ConnectorElement, mut element2: ConnectorElement, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut new1: bool;
    let mut new2: bool;
    new1 = isNewElement(element1.clone());
    new2 = isNewElement(element2.clone());
    sets = mergeSets2(element1, element2, new1, new2, sets)?;
    Ok(sets)
}

fn mergeSets2(
    mut element1: ConnectorElement,
    mut element2: ConnectorElement,
    mut isNew1: bool,
    mut isNew2: bool,
    mut sets: Sets,
) -> Result<Sets> {
    let mut sets: Sets = sets;
    sets = (match (isNew1, isNew2) {
        (true, true) => addNewSet(element1, element2, sets)?,
        (true, false) => addToSet(element1, element2, sets)?,
        (false, true) => addToSet(element2, element1, sets)?,
        (false, false) => connectSets(element1, element2, sets),
        _ => return Err("match: no arm matched"),
    });
    Ok(sets)
}

fn addNewSet(mut element1: ConnectorElement, mut element2: ConnectorElement, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut node: metamodelica::Ref<SetTrieNode>;
    let mut sc: i32;
    let mut e1: ConnectorElement;
    let mut e2: ConnectorElement;
    sc = sets.setCount.clone() + 1;
    e1 = setElementSetIndex(element1, sc);
    e2 = setElementSetIndex(element2, sc);
    node = sets.sets.clone();
    node = setTrieAdd(e1, node)?;
    sets.sets = setTrieAdd(e2, node)?;
    sets.setCount = sc;
    Ok(sets)
}

fn addToSet(mut element: ConnectorElement, mut set: ConnectorElement, mut sets: Sets) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut index: i32;
    let mut e: ConnectorElement;
    index = getElementSetIndex(set);
    e = setElementSetIndex(element, index);
    sets.sets = setTrieAdd(e, sets.sets.clone())?;
    Ok(sets)
}

fn connectSets(mut element1: ConnectorElement, mut element2: ConnectorElement, mut sets: Sets) -> Sets {
    let mut sets: Sets = sets;
    let mut set1: i32;
    let mut set2: i32;
    set1 = getElementSetIndex(element1);
    set2 = getElementSetIndex(element2);
    if set1 != set2 {
        sets.connections = metamodelica::cons((set1, set2), sets.connections.clone());
    }
    sets
}

fn setTrieGetElement(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut face: Face,
    mut trie: &metamodelica::Ref<SetTrieNode>,
) -> Result<ConnectorElement> {
    let mut element: ConnectorElement;
    let mut node: metamodelica::Ref<SetTrieNode>;
    node = setTrieGet(cref, trie, false)?;
    element = setTrieGetLeafElement(&node, face)?;
    Ok(element)
}

fn setTrieAddLeafElement(
    mut element: ConnectorElement,
    mut node: metamodelica::Ref<SetTrieNode>,
) -> Result<metamodelica::Ref<SetTrieNode>> {
    let mut node: metamodelica::Ref<SetTrieNode> = node;
    let () = (match &*node {
        DAE::Connect::SetTrieNode::SET_TRIE_LEAF { .. } => {
            let () = (match element.face.clone() {
                DAE::Connect::Face::INSIDE => {
                    assign_variant_field!(node => SetTrieNode::SET_TRIE_LEAF; insideElement = Some(element));
                    ()
                }
                DAE::Connect::Face::OUTSIDE => {
                    assign_variant_field!(node => SetTrieNode::SET_TRIE_LEAF; outsideElement = Some(element));
                    ()
                }
                _ => return Err("match: no arm matched"),
            });
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(node)
}

fn setTrieGetLeafElement(mut node: &metamodelica::Ref<SetTrieNode>, mut face: Face) -> Result<ConnectorElement> {
    let mut element: ConnectorElement;
    element = (::match_deref::match_deref! { match &((face, &**node)) {
        (DAE::Connect::Face::INSIDE, Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { insideElement: Some(e), .. }) => {
            e.clone()
        },
        (DAE::Connect::Face::OUTSIDE, Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { outsideElement: Some(e), .. }) => {
            e.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(element)
}

fn setTrieAdd(
    mut element: ConnectorElement,
    mut trie: metamodelica::Ref<SetTrieNode>,
) -> Result<metamodelica::Ref<SetTrieNode>> {
    let mut trie: metamodelica::Ref<SetTrieNode> = trie;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut el_cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut el: ConnectorElement;
    cref = getElementName(element.clone());
    el_cr = ComponentReferenceBasics::crefLastCref(&cref)?;
    el = setElementName(element, el_cr);
    trie = setTrieUpdate(&cref, el, trie, &setTrieAddLeafElement)?;
    Ok(trie)
}

fn updateSetLeaf<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut sets: Sets,
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut arg: Arg,
    mut updateFunc: &dyn ::std::ops::Fn(Arg, metamodelica::Ref<SetTrieNode>) -> Result<metamodelica::Ref<SetTrieNode>>,
) -> Result<Sets> {
    pub type UpdateFunc<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(Arg, metamodelica::Ref<SetTrieNode>) -> Result<metamodelica::Ref<SetTrieNode>> + 'static,
    >;

    let mut sets: Sets = sets;
    sets.sets = setTrieUpdate(cref, arg, sets.sets.clone(), updateFunc)?;
    Ok(sets)
}

fn setTrieUpdate<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut arg: Arg,
    mut trie: metamodelica::Ref<SetTrieNode>,
    mut updateFunc: &dyn ::std::ops::Fn(Arg, metamodelica::Ref<SetTrieNode>) -> Result<metamodelica::Ref<SetTrieNode>>,
) -> Result<metamodelica::Ref<SetTrieNode>> {
    pub type UpdateFunc<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(Arg, metamodelica::Ref<SetTrieNode>) -> Result<metamodelica::Ref<SetTrieNode>> + 'static,
    >;

    let mut trie: metamodelica::Ref<SetTrieNode> = trie;
    let () = (::match_deref::match_deref! { match &((cref.clone(), trie.clone())) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. }) => {
            let mut id: ArcStr;
            id = ComponentReferenceBasics::printComponentRef2Str(var_field!((**cref).ident, DAE::ComponentRef::CREF_QUAL).clone(), var_field!((**cref).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
            assign_variant_field!(trie => SetTrieNode::SET_TRIE_NODE; nodes = setTrieUpdateNode(&id, cref, var_field!((**cref).componentRef, DAE::ComponentRef::CREF_QUAL), arg, updateFunc, var_field!((*trie).nodes, SetTrieNode::SET_TRIE_NODE).clone())?);
            ()
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. }) => {
            let mut id: ArcStr;
            id = ComponentReferenceBasics::printComponentRef2Str(var_field!((**cref).ident, DAE::ComponentRef::CREF_IDENT).clone(), var_field!((**cref).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?;
            assign_variant_field!(trie => SetTrieNode::SET_TRIE_NODE; nodes = setTrieUpdateLeaf(id, arg, var_field!((*trie).nodes, SetTrieNode::SET_TRIE_NODE).clone(), updateFunc)?);
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(trie)
}

fn setTrieUpdateNode<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut id: &ArcStr,
    mut wholeCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut arg: Arg,
    mut updateFunc: &dyn ::std::ops::Fn(Arg, metamodelica::Ref<SetTrieNode>) -> Result<metamodelica::Ref<SetTrieNode>>,
    mut nodes: metamodelica::List<metamodelica::Ref<SetTrieNode>>,
) -> Result<metamodelica::List<metamodelica::Ref<SetTrieNode>>> {
    pub type UpdateFunc<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(Arg, metamodelica::Ref<SetTrieNode>) -> Result<metamodelica::Ref<SetTrieNode>> + 'static,
    >;

    let mut nodes: metamodelica::List<metamodelica::Ref<SetTrieNode>> = nodes;
    let mut node2: metamodelica::Ref<SetTrieNode>;
    let mut n: i32 = 1;
    for mut node in &*nodes.clone() {
        if setTrieIsNode(metamodelica::AsArg::as_arg(&node))
            && metamodelica::stringEq(&(setTrieNodeName(metamodelica::AsArg::as_arg(&node))), &id)
        {
            node2 = setTrieUpdate(cref, arg, node.clone(), updateFunc)?;
            nodes = List::replaceAt(node2, n, nodes)?;
            return Ok(nodes.clone());
        } else {
            n = n + 1;
        }
    }
    nodes = setTrieUpdateNode2(wholeCref, arg, updateFunc, nodes)?;
    Ok(nodes)
}

fn setTrieUpdateNode2<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut arg: Arg,
    mut updateFunc: &dyn ::std::ops::Fn(Arg, metamodelica::Ref<SetTrieNode>) -> Result<metamodelica::Ref<SetTrieNode>>,
    mut nodes: metamodelica::List<metamodelica::Ref<SetTrieNode>>,
) -> Result<metamodelica::List<metamodelica::Ref<SetTrieNode>>> {
    pub type UpdateFunc<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(Arg, metamodelica::Ref<SetTrieNode>) -> Result<metamodelica::Ref<SetTrieNode>> + 'static,
    >;

    let mut nodes: metamodelica::List<metamodelica::Ref<SetTrieNode>> = nodes;
    nodes = (match &**cref {
        DAE::ComponentRef::CREF_IDENT { .. } => {
            let mut id: ArcStr;
            let mut node: metamodelica::Ref<SetTrieNode>;
            id = ComponentReferenceBasics::printComponentRefStr(cref)?;
            node = metamodelica::Ref::new(SetTrieNode::SET_TRIE_LEAF {
                name: id,
                insideElement: None,
                outsideElement: None,
                flowAssociation: None,
                connectCount: 0,
            });
            node = updateFunc(arg, node)?;
            metamodelica::cons(node, nodes)
        }
        DAE::ComponentRef::CREF_QUAL {
            componentRef: __cref_componentRef,
            ..
        } => {
            let mut id: ArcStr;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut child_nodes: metamodelica::List<metamodelica::Ref<SetTrieNode>>;
            cr = ComponentReferenceBasics::crefFirstCref(cref.clone())?;
            id = ComponentReferenceBasics::printComponentRefStr(&cr)?;
            child_nodes = setTrieUpdateNode2(
                metamodelica::AsArg::as_arg(&__cref_componentRef),
                arg,
                updateFunc,
                metamodelica::nil(),
            )?;
            metamodelica::cons(
                metamodelica::Ref::new(SetTrieNode::SET_TRIE_NODE {
                    name: id,
                    cref: cr,
                    nodes: child_nodes,
                    connectCount: 0,
                }),
                nodes,
            )
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(nodes)
}

fn setTrieUpdateLeaf<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut id: ArcStr,
    mut arg: Arg,
    mut nodes: metamodelica::List<metamodelica::Ref<SetTrieNode>>,
    mut updateFunc: &dyn ::std::ops::Fn(Arg, metamodelica::Ref<SetTrieNode>) -> Result<metamodelica::Ref<SetTrieNode>>,
) -> Result<metamodelica::List<metamodelica::Ref<SetTrieNode>>> {
    pub type UpdateFunc<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(Arg, metamodelica::Ref<SetTrieNode>) -> Result<metamodelica::Ref<SetTrieNode>> + 'static,
    >;

    let mut nodes: metamodelica::List<metamodelica::Ref<SetTrieNode>> = nodes;
    let mut n: i32 = 1;
    for mut node in &*nodes.clone() {
        if metamodelica::stringEq(&(setTrieNodeName(metamodelica::AsArg::as_arg(&node))), &id) {
            nodes = List::replaceAt(updateFunc(arg, node.clone())?, n, nodes)?;
            return Ok(nodes.clone());
        }
        n = n + 1;
    }
    nodes = metamodelica::cons(
        updateFunc(
            arg,
            metamodelica::Ref::new(SetTrieNode::SET_TRIE_LEAF {
                name: id,
                insideElement: None,
                outsideElement: None,
                flowAssociation: None,
                connectCount: 0,
            }),
        )?,
        nodes,
    );
    Ok(nodes)
}

pub(crate) fn traverseSets<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut sets: Sets,
    mut arg: Arg,
    mut updateFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SetTrieNode>, Arg) -> Result<(metamodelica::Ref<SetTrieNode>, Arg)>
            + 'static,
    >,
) -> Result<(Sets, Arg)> {
    pub type UpdateFunc<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SetTrieNode>, Arg) -> Result<(metamodelica::Ref<SetTrieNode>, Arg)>
            + 'static,
    >;

    let mut sets: Sets = sets;
    let mut arg: Arg = arg;
    let mut node: metamodelica::Ref<SetTrieNode>;
    (node, arg) = setTrieTraverseLeaves(sets.sets.clone(), updateFunc.clone(), arg)?;
    sets.sets = node;
    Ok((sets, arg))
}

fn setTrieTraverseLeaves<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut node: metamodelica::Ref<SetTrieNode>,
    mut updateFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SetTrieNode>, Arg) -> Result<(metamodelica::Ref<SetTrieNode>, Arg)>
            + 'static,
    >,
    mut arg: Arg,
) -> Result<(metamodelica::Ref<SetTrieNode>, Arg)> {
    pub type UpdateFunc<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SetTrieNode>, Arg) -> Result<(metamodelica::Ref<SetTrieNode>, Arg)>
            + 'static,
    >;

    let mut node: metamodelica::Ref<SetTrieNode> = node;
    let mut arg: Arg = arg;
    let () = (match &*node {
        DAE::Connect::SetTrieNode::SET_TRIE_NODE {
            nodes: __node_nodes, ..
        } => {
            let mut nodes: metamodelica::List<metamodelica::Ref<SetTrieNode>>;
            (nodes, arg) = List::map1Fold(
                metamodelica::AsArg::as_arg(&__node_nodes),
                &setTrieTraverseLeaves,
                updateFunc.clone(),
                arg,
            )?;
            assign_variant_field!(node => SetTrieNode::SET_TRIE_NODE; nodes = nodes);
            ()
        }
        DAE::Connect::SetTrieNode::SET_TRIE_LEAF { .. } => {
            (node, arg) = updateFunc(node, arg)?;
            ()
        }
    });
    Ok((node, arg))
}

fn setTrieGet(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut trie: &metamodelica::Ref<SetTrieNode>,
    mut matchPrefix: bool,
) -> Result<metamodelica::Ref<SetTrieNode>> {
    let mut leaf: metamodelica::Ref<SetTrieNode>;
    let mut nodes: metamodelica::List<metamodelica::Ref<SetTrieNode>>;
    let mut subs_str: ArcStr;
    let mut id_subs: ArcStr;
    let mut id_nosubs: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*trie)) {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { nodes: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    nodes = metamodelica::Own::own(__pa0);
    id_nosubs = ComponentReferenceBasics::crefFirstIdent(cref)?;
    subs_str = List::toStringCustom(
        ComponentReference::crefFirstSubs(cref),
        &move |__a0: metamodelica::Ref<DAE::Subscript>| ExpressionBasics::printSubscriptStr(&__a0),
        literal!(""),
        literal!("["),
        literal!(","),
        literal!("]"),
        false,
        0,
    )?;
    id_subs = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*id_nosubs);
        __mm_s.push_str(&*subs_str);
        ArcStr::from(__mm_s)
    };
    match '__try1: {
        leaf = unwrap_break_err!(setTrieGetNode(id_subs.clone(), &nodes), '__try1);
        Ok::<_, &'static str>((leaf.clone(),))
    } {
        Ok((__try1_o0,)) => {
            leaf = __try1_o0;
        }
        Err(_) => {
            leaf = setTrieGetNode(id_nosubs.clone(), &nodes)?;
        }
    }
    if !(ComponentReference::crefIsIdent(cref)) {
        if '__try2: {
            leaf = unwrap_break_err!(setTrieGet(&(unwrap_break_err!(ComponentReference::crefRest(cref), '__try2)), &leaf, matchPrefix), '__try2);
            Ok::<(), &'static str>(())
        }.is_err() {
            let true = (matchPrefix && !(setTrieIsNode(&leaf))) else { return Err("pattern mismatch") };
        }
    }
    Ok(leaf)
}

fn setTrieGetNode(
    mut id: ArcStr,
    mut nodes: &metamodelica::List<metamodelica::Ref<SetTrieNode>>,
) -> Result<metamodelica::Ref<SetTrieNode>> {
    let mut node: metamodelica::Ref<SetTrieNode>;
    node = List::getMemberOnTrue(id, nodes, &move |__a0: ArcStr,
                                                   __a1: metamodelica::Ref<SetTrieNode>|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(setTrieNodeNamed(&__a0, &__a1))
    })?;
    Ok(node)
}

fn setTrieNodeNamed(mut id: &ArcStr, mut node: &metamodelica::Ref<SetTrieNode>) -> bool {
    let mut isNamed: bool;
    isNamed = (match &**node {
        DAE::Connect::SetTrieNode::SET_TRIE_NODE { name: __node_name, .. } => metamodelica::stringEq(&id, &__node_name),
        DAE::Connect::SetTrieNode::SET_TRIE_LEAF { name: __node_name, .. } => metamodelica::stringEq(&id, &__node_name),
        _ => false,
    });
    isNamed
}

fn setTrieGetLeaf(
    mut id: ArcStr,
    mut nodes: &metamodelica::List<metamodelica::Ref<SetTrieNode>>,
) -> Result<metamodelica::Ref<SetTrieNode>> {
    let mut node: metamodelica::Ref<SetTrieNode>;
    node = List::getMemberOnTrue(id, nodes, &move |__a0: ArcStr,
                                                   __a1: metamodelica::Ref<SetTrieNode>|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(setTrieLeafNamed(&__a0, &__a1))
    })?;
    Ok(node)
}

fn setTrieLeafNamed(mut id: &ArcStr, mut node: &metamodelica::Ref<SetTrieNode>) -> bool {
    let mut isNamed: bool;
    isNamed = (match &**node {
        DAE::Connect::SetTrieNode::SET_TRIE_LEAF { name: __node_name, .. } => metamodelica::stringEq(&id, &__node_name),
        _ => false,
    });
    isNamed
}

fn setTrieIsNode(mut node: &metamodelica::Ref<SetTrieNode>) -> bool {
    let mut isNode: bool;
    isNode = (match &**node {
        DAE::Connect::SetTrieNode::SET_TRIE_NODE { .. } => true,
        _ => false,
    });
    isNode
}

pub(crate) fn equations(
    mut topScope: bool,
    mut sets: Sets,
    mut DAE: DAE::DAElist,
    mut connectionGraph: ConnectionGraph::ConnectionGraph,
    mut modelNameQualified: &ArcStr,
) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist = DAE;
    let mut set_list: metamodelica::List<Set>;
    let mut set_array: metamodelica::Array<Set>;
    let mut dae: DAE::DAElist;
    let mut dae2: DAE::DAElist;
    let mut broken: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
    )>;
    let mut connected: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
    )>;
    {
        let __v = None;
        openmodelica_util::Globals::isInStream.with(|__root| *__root.borrow_mut() = __v)
    };
    if !(topScope) {
        return Ok(DAE);
    }
    set_array = generateSetArray(&sets)?;
    set_list = set_array
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>();
    if daeHasExpandableConnectors(DAE.clone())? {
        (set_list, dae) = removeUnusedExpandableVariablesAndConnections(set_list, DAE)?;
    } else {
        dae = DAE;
    }
    (dae, connected, broken) =
        ConnectionGraph::handleOverconstrainedConnections(connectionGraph, modelNameQualified, dae)?;
    dae2 = equationsDispatch(&(set_list.reverse()), &connected, &broken)?;
    DAE = DAEUtil::joinDaes(&dae, &dae2)?;
    DAE = evaluateConnectionOperators(sets, set_array.clone(), DAE)?;
    DAE = ConnectionGraph::addBrokenEqualityConstraintEquations(DAE, broken)?;
    Ok(DAE)
}

fn getExpandableEquSetsAsCrefs(
    mut sets: &metamodelica::List<Set>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>> {
    let mut crefSets: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> =
        metamodelica::nil();
    let mut cref_set: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    for mut set in &**sets {
        let () = (match set.clone() {
            DAE::Connect::Set::SET {
                ty: DAE::Connect::ConnectorType::EQU,
                ..
            } => {
                cref_set = getAllEquCrefs(&(list![set.clone()]));
                if List::applyAndFold(
                    &cref_set,
                    &fnptr!(boolOr, bool, bool),
                    &move |__a0: metamodelica::Ref<DAE::ComponentRef>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(isExpandable(&__a0))
                    },
                    false,
                )? {
                    crefSets = metamodelica::cons(cref_set, crefSets);
                }
                ()
            }
            _ => (),
        });
    }
    Ok(crefSets)
}

fn removeCrefsFromSets(
    mut sets: metamodelica::List<Set>,
    mut nonUsefulExpandable: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<Set>> {
    let mut sets: metamodelica::List<Set> = sets;
    sets = List::select1(
        sets,
        (std::sync::Arc::new(
            move |__a0: Set, __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>| {
                removeCrefsFromSets2(__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Set, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<bool>
                    + 'static,
            >),
        nonUsefulExpandable,
    )?;
    Ok(sets)
}

fn removeCrefsFromSets2(
    mut set: Set,
    mut nonUsefulExpandable: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<bool> {
    let mut isInSet: bool;
    let mut setCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    setCrefs = getAllEquCrefs(&(list![set]));
    lst = List::intersectionOnTrue(&setCrefs, nonUsefulExpandable, &move |__a0: metamodelica::Ref<
        DAE::ComponentRef,
    >,
                                                                          __a1: metamodelica::Ref<
        DAE::ComponentRef,
    >| {
        ComponentReferenceBasics::crefEqualNoStringCompare(&__a0, &__a1)
    })?;
    isInSet = (lst).is_empty();
    Ok(isInSet)
}

fn mergeEquSetsAsCrefs(
    mut setsAsCrefs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>> {
    let mut setsAsCrefs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> = setsAsCrefs;
    setsAsCrefs = (::match_deref::match_deref! { match &(setsAsCrefs) {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: set, tail: Deref @ metamodelica::ListNode::Nil } => {
            list![set.clone()]
        },
        Deref @ metamodelica::ListNode::Cons { head: set, tail: rest } => {
            let mut sets: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
            let mut set = (*set).clone();
            let mut rest = (*rest).clone();
            (set, rest) = mergeWithRest(set.clone(), rest.clone(), metamodelica::nil())?;
            sets = mergeEquSetsAsCrefs(rest.clone())?;
            metamodelica::cons(set.clone(), sets)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(setsAsCrefs)
}

fn mergeWithRest(
    mut set: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut sets: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
    mut acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((set.clone(), sets)) {
            (_, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((set, acc.reverse()))
            },
            (set1, Deref @ metamodelica::ListNode::Cons { head: set2, tail: rest }) => {
                let mut b: bool;
                let mut rest = (*rest).clone();
                b = ((List::intersectionOnTrue(metamodelica::AsArg::as_arg(&set1), metamodelica::AsArg::as_arg(&set2), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqualNoStringCompare(&__a0, &__a1))?)).is_empty();
                set = if (!(b)) {List::unionOnTrue(metamodelica::AsArg::as_arg(&set1), metamodelica::AsArg::as_arg(&set2), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqualNoStringCompare(&__a0, &__a1))?} else {set1.clone()};
                { (set, sets, acc) = (set, rest.clone(), List::consOnTrue(b, set2.clone(), acc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getOnlyExpandableConnectedCrefs(
    mut sets: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
) -> metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> {
    let mut usefulConnectedExpandable: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    for mut set in &**sets {
        if allCrefsAreExpandable(metamodelica::AsArg::as_arg(&set)) {
            usefulConnectedExpandable = listAppend(set.clone(), usefulConnectedExpandable);
        }
    }
    usefulConnectedExpandable
}

pub(crate) fn allCrefsAreExpandable(mut connects: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> bool {
    let mut allAreExpandable: bool;
    for mut cr in &**connects {
        if !(isExpandable(metamodelica::AsArg::as_arg(&cr))) {
            allAreExpandable = false;
            return allAreExpandable;
        }
    }
    allAreExpandable = true;
    allAreExpandable
}

fn generateSetArray(mut sets: &Sets) -> Result<metamodelica::Array<Set>> {
    let mut setArray: metamodelica::Array<Set>;
    setArray = arrayCreate(
        sets.setCount.clone(),
        Set::SET {
            ty: openmodelica_frontend_types::DAE::Connect::ConnectorType::NO_TYPE,
            elements: metamodelica::nil(),
        },
    );
    setArray = setArrayAddConnections(&sets.connections, sets.setCount.clone(), setArray.clone())?;
    setArray = generateSetArray2(&sets.sets, metamodelica::nil(), setArray.clone())?;
    Ok(setArray)
}

fn setArrayAddConnections(
    mut connections: &metamodelica::List<(i32, i32)>,
    mut setCount: i32,
    mut sets: metamodelica::Array<Set>,
) -> Result<metamodelica::Array<Set>> {
    let mut sets: metamodelica::Array<Set> = sets;
    let mut graph: SetGraph;
    graph = arrayCreate(setCount, metamodelica::nil());
    graph = List::fold(connections, &addConnectionToGraph, graph.clone())?;
    for mut i in 1..=metamodelica::arrayLength(graph.clone()) {
        (sets, graph) = setArrayAddConnection(
            i,
            &({
                let __elt = (*metamodelica::index_checked(&graph.borrow(), i)?).clone();
                __elt
            }),
            sets.clone(),
            graph.clone(),
        )?;
    }
    Ok(sets)
}

fn addConnectionToGraph(mut connection: (i32, i32), mut graph: SetGraph) -> Result<SetGraph> {
    let mut graph: SetGraph = graph;
    let mut set1: i32;
    let mut set2: i32;
    let mut node1: metamodelica::List<i32>;
    let mut node2: metamodelica::List<i32>;
    (set1, set2) = connection;
    node1 = metamodelica::arrayGet(graph.clone(), set1)?;
    graph = metamodelica::arrayUpdate(graph.clone(), set1, metamodelica::cons(set2, node1))?;
    node2 = metamodelica::arrayGet(graph.clone(), set2)?;
    graph = metamodelica::arrayUpdate(graph.clone(), set2, metamodelica::cons(set1, node2))?;
    Ok(graph)
}

fn setArrayAddConnection(
    mut set: i32,
    mut edges: &metamodelica::List<i32>,
    mut sets: metamodelica::Array<Set>,
    mut graph: SetGraph,
) -> Result<(metamodelica::Array<Set>, SetGraph)> {
    let mut sets: metamodelica::Array<Set> = sets;
    let mut graph: SetGraph = graph;
    let mut edge_lst: metamodelica::List<i32>;
    for mut e in &**edges {
        if e.clone() != set {
            sets = setArrayAddConnection2(e.clone(), set, sets.clone())?;
            edge_lst = ({
                let __elt = (*metamodelica::index_checked(&graph.borrow(), e.clone())?).clone();
                __elt
            });
            {
                let __cell0 = metamodelica::nil();
                let __idx0 = e.clone();
                *metamodelica::index_mut_checked(&mut graph.clone().borrow_mut(), __idx0)? = __cell0;
            }
            (sets, graph) = setArrayAddConnection(set, &edge_lst, sets.clone(), graph.clone())?;
        }
    }
    Ok((sets, graph))
}

fn setArrayAddConnection2(
    mut setPointer: i32,
    mut setPointee: i32,
    mut sets: metamodelica::Array<Set>,
) -> Result<metamodelica::Array<Set>> {
    '__tco: loop {
        let mut set: Set;
        set = ({
            let __elt = (*metamodelica::index_checked(&sets.borrow(), setPointee)?).clone();
            __elt
        });
        match set.clone() {
            DAE::Connect::Set::SET { .. } => {
                return Ok(metamodelica::arrayUpdate(
                    sets.clone(),
                    setPointer,
                    Set::SET_POINTER { index: setPointee },
                )?);
            }
            DAE::Connect::Set::SET_POINTER { .. } => {
                (setPointer, setPointee, sets) = (
                    setPointer,
                    var_field!(set.index, Set::SET_POINTER).clone(),
                    sets.clone(),
                );
                continue '__tco;
            }
        }
    }
}

fn generateSetArray2(
    mut sets: &metamodelica::Ref<SetTrieNode>,
    mut prefix: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut setArray: metamodelica::Array<Set>,
) -> Result<metamodelica::Array<Set>> {
    let mut setArray: metamodelica::Array<Set> = setArray;
    setArray = (::match_deref::match_deref! { match sets {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { cref: Deref @ DAE::ComponentRef::WILD { .. }, nodes: __sets_nodes, .. } => {
            List::fold1(metamodelica::AsArg::as_arg(&__sets_nodes), &move |__a0: metamodelica::Ref<SetTrieNode>, __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, __a2: metamodelica::Array<Set>| generateSetArray2(&__a0, __a1, __a2), prefix, setArray.clone())?
        },
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { cref: __sets_cref, nodes: __sets_nodes, .. } => {
            List::fold1(metamodelica::AsArg::as_arg(&__sets_nodes), &move |__a0: metamodelica::Ref<SetTrieNode>, __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, __a2: metamodelica::Array<Set>| generateSetArray2(&__a0, __a1, __a2), metamodelica::cons(__sets_cref.clone(), prefix), setArray.clone())?
        },
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { insideElement: ie, outsideElement: oe, flowAssociation: flow_cr, .. } => {
            let mut prefix_cr: Option<metamodelica::Ref<DAE::ComponentRef>>;
            let mut ie = (*ie).clone();
            let mut oe = (*oe).clone();
            ie = insertFlowAssociationInStreamElement(ie.clone(), flow_cr.clone())?;
            oe = insertFlowAssociationInStreamElement(oe.clone(), flow_cr.clone())?;
            prefix_cr = buildElementPrefix(&prefix)?;
            setArray = setArrayAddElement(ie.clone(), prefix_cr.clone(), setArray.clone())?;
            setArray = setArrayAddElement(oe.clone(), prefix_cr, setArray.clone())?;
            setArray.clone()
        },
        _ => {
            setArray.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(setArray)
}

fn insertFlowAssociationInStreamElement(
    mut element: Option<ConnectorElement>,
    mut flowCref: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<Option<ConnectorElement>> {
    let mut element: Option<ConnectorElement> = element;
    let mut el: ConnectorElement;
    if (element).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(element.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        el = metamodelica::Own::own(__pa0);
        element = (::match_deref::match_deref! { match &(el.clone()) {
            ConnectorElement { ty: DAE::Connect::ConnectorType::STREAM { associatedFlow: None }, .. } => {
                el.ty = ConnectorType::STREAM { associatedFlow: flowCref };
                Some(el)
            },
            _ => element,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(element)
}

fn setArrayAddElement(
    mut element: Option<ConnectorElement>,
    mut prefix: Option<metamodelica::Ref<DAE::ComponentRef>>,
    mut sets: metamodelica::Array<Set>,
) -> Result<metamodelica::Array<Set>> {
    let mut sets: metamodelica::Array<Set> = sets;
    sets = (::match_deref::match_deref! { match &((element, prefix)) {
        (None, _) => {
            sets.clone()
        },
        (Some(el @ ConnectorElement { .. }), None) => {
            setArrayUpdate(sets.clone(), el.set.clone(), metamodelica::AsArg::as_arg(&el))?
        },
        (Some(el @ ConnectorElement { .. }), Some(prefix_cr)) => {
            let mut el = (*el).clone();
            el.name = ComponentReference::joinCrefs(metamodelica::AsArg::as_arg(&prefix_cr), el.name.clone())?;
            setArrayUpdate(sets.clone(), el.set.clone(), metamodelica::AsArg::as_arg(&el))?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(sets)
}

fn buildElementPrefix(
    mut prefix: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<Option<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut cref: Option<metamodelica::Ref<DAE::ComponentRef>>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut id: ArcStr;
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    if (prefix).is_empty() {
        cref = None;
    } else {
        cr = (prefix).head().cloned()?;
        for mut c in &*(prefix).rest()? {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(c.clone()) {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: __pa0, subscriptLst: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            id = metamodelica::Own::own(__pa0);
            subs = metamodelica::Own::own(__pa1);
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                ident: id,
                identType: DAE::T_UNKNOWN_DEFAULT().clone(),
                subscriptLst: subs,
                componentRef: cr,
            });
        }
        cref = Some(cr);
    }
    Ok(cref)
}

fn setArrayUpdate<'__b>(
    mut sets: metamodelica::Array<Set>,
    mut index: i32,
    mut element: &'__b ConnectorElement,
) -> Result<metamodelica::Array<Set>> {
    '__tco: loop {
        let mut set: Set;
        let mut el: metamodelica::List<ConnectorElement>;
        set = ({
            let __elt = (*metamodelica::index_checked(&sets.borrow(), index)?).clone();
            __elt
        });
        match (set.clone(), element.clone()) {
            (DAE::Connect::Set::SET { .. }, ConnectorElement { .. }) => {
                if Config::orderConnections()? && isEquType(&element.ty) {
                    el = List::mergeSorted(
                        list![element.clone()],
                        var_field!(set.elements, Set::SET).clone(),
                        &move |__a0: ConnectorElement, __a1: ConnectorElement| equSetElementLess(&__a0, &__a1),
                    )?;
                } else {
                    el = metamodelica::cons(element.clone(), var_field!(set.elements, Set::SET).clone());
                }
                return Ok(metamodelica::arrayUpdate(
                    sets.clone(),
                    index,
                    Set::SET {
                        ty: element.ty.clone(),
                        elements: el,
                    },
                )?);
            }
            (DAE::Connect::Set::SET_POINTER { .. }, _) => {
                (sets, index, element) = (sets.clone(), var_field!(set.index, Set::SET_POINTER).clone(), element);
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

fn equSetElementLess(mut element1: &ConnectorElement, mut element2: &ConnectorElement) -> Result<bool> {
    let mut isLess: bool;
    isLess = ComponentReferenceBasics::crefSortFunc(&element2.name, &element1.name)?;
    Ok(isLess)
}

fn setArrayGet(mut setArray: metamodelica::Array<Set>, mut index: i32) -> Result<Set> {
    let mut set: Set;
    set = ({
        let __elt = (*metamodelica::index_checked(&setArray.borrow(), index)?).clone();
        __elt
    });
    set = (match set.clone() {
        DAE::Connect::Set::SET { .. } => set,
        DAE::Connect::Set::SET_POINTER { .. } => {
            setArrayGet(setArray.clone(), var_field!(set.index, Set::SET_POINTER).clone())?
        }
    });
    Ok(set)
}

fn equationsDispatch(
    mut sets: &metamodelica::List<Set>,
    mut connected: &metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
    )>,
    mut broken: &metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
    )>,
) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist = DAE::emptyDae().clone();
    let mut eql: metamodelica::List<ConnectorElement> = metamodelica::nil();
    let mut eqll: metamodelica::List<metamodelica::List<ConnectorElement>>;
    let mut flowThreshold: metamodelica::Real = Flags::getConfigReal(Flags::FLOW_THRESHOLD.clone())?;
    for mut set in &**sets {
        DAE = (match set.clone() {
            DAE::Connect::Set::SET_POINTER { .. } => DAE,
            DAE::Connect::Set::SET {
                ty: DAE::Connect::ConnectorType::EQU,
                ..
            } => {
                eqll = ConnectionGraph::removeBrokenConnects(
                    var_field!(set.elements, Set::SET).clone(),
                    connected,
                    broken,
                )?;
                for mut eql in &*eqll {
                    let mut eql = eql.clone();
                    DAE = DAEUtil::joinDaes(&(generateEquEquations(&eql)?), &DAE)?;
                }
                DAE
            }
            DAE::Connect::Set::SET {
                ty: DAE::Connect::ConnectorType::FLOW,
                elements: ref __esc_eql,
            } => {
                eql = __esc_eql.clone();
                DAEUtil::joinDaes(&(generateFlowEquations(metamodelica::AsArg::as_arg(&eql))?), &DAE)?
            }
            DAE::Connect::Set::SET {
                ty: DAE::Connect::ConnectorType::STREAM { .. },
                elements: ref __esc_eql,
            } => {
                eql = __esc_eql.clone();
                DAEUtil::joinDaes(
                    &(generateStreamEquations(metamodelica::AsArg::as_arg(&eql), flowThreshold)?),
                    &DAE,
                )?
            }
            DAE::Connect::Set::SET {
                ty: DAE::Connect::ConnectorType::NO_TYPE,
                ..
            } => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![literal!(
                        "ConnectUtil.equationsDispatch failed on connection set with no type."
                    )],
                )?;
                return Err("fail");
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![literal!(
                        "ConnectUtil.equationsDispatch failed because of unknown reason."
                    )],
                )?;
                return Err("fail");
            }
        });
    }
    Ok(DAE)
}

fn generateEquEquations(mut elements: &metamodelica::List<ConnectorElement>) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist = DAE::emptyDae().clone();
    let mut eql: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut e1: ConnectorElement;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let mut x: metamodelica::Ref<DAE::ComponentRef>;
    let mut y: metamodelica::Ref<DAE::ComponentRef>;
    if (elements).is_empty() {
        return Ok(DAE);
    }
    e1 = (elements).head().cloned()?;
    if Config::orderConnections()? {
        for mut e2 in &*(elements).rest()? {
            src = ElementSource::mergeSources(&e1.source, &e2.source)?;
            src = ElementSource::addElementSourceConnect(&src, (e1.name.clone(), e2.name.clone()));
            eql = metamodelica::cons(
                metamodelica::Ref::new(DAE::Element::EQUEQUATION {
                    cr1: e1.name.clone(),
                    cr2: e2.name.clone(),
                    source: src,
                }),
                eql,
            );
        }
    } else {
        for mut e2 in &*(elements).rest()? {
            (x, y) = Util::swap(
                shouldFlipEquEquation(&e1.name, &e1.source)?,
                e1.name.clone(),
                e2.name.clone(),
            );
            src = ElementSource::mergeSources(&e1.source, &e2.source)?;
            src = ElementSource::addElementSourceConnect(&src, (x.clone(), y.clone()));
            eql = metamodelica::cons(
                metamodelica::Ref::new(DAE::Element::EQUEQUATION {
                    cr1: x,
                    cr2: y,
                    source: src,
                }),
                eql,
            );
            e1 = e2.clone();
        }
    }
    DAE = DAE::DAElist {
        elementLst: eql.reverse(),
    };
    Ok(DAE)
}

fn shouldFlipEquEquation(
    mut lhsCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut lhsSource: &metamodelica::Ref<DAE::ElementSource>,
) -> Result<bool> {
    let mut shouldFlip: bool;
    shouldFlip = (::match_deref::match_deref! { match lhsSource {
        Deref @ DAE::ElementSource { connectEquationOptLst: Deref @ metamodelica::ListNode::Cons { head: (lhs, _), tail: _ }, .. } => {
            !(ComponentReferenceBasics::crefPrefixOf(metamodelica::AsArg::as_arg(&lhs), lhsCref)?)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(shouldFlip)
}

fn generateFlowEquations(mut elements: &metamodelica::List<ConnectorElement>) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist;
    let mut sum: metamodelica::Ref<DAE::Exp>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    sum = makeFlowExp(&((elements).head().cloned()?))?;
    src = getElementSource((elements).head().cloned()?);
    for mut e in &*(elements).rest()? {
        sum = Expression::makeRealAdd(sum, makeFlowExp(metamodelica::AsArg::as_arg(&e))?);
        src = ElementSource::mergeSources(&src, &e.source)?;
    }
    DAE = DAE::DAElist {
        elementLst: list![metamodelica::Ref::new(DAE::Element::EQUATION {
            exp: sum,
            scalar: metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(0.0_f64)
            }),
            source: src
        })],
    };
    Ok(DAE)
}

fn makeFlowExp(mut element: &ConnectorElement) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = Expression::crefExp(element.name.clone())?;
    if isOutsideElement(element) {
        exp = Expression::negateReal(exp);
    }
    Ok(exp)
}

pub(crate) fn increaseConnectRefCount(
    mut lhsCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut rhsCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut sets: Sets,
) -> Result<Sets> {
    let mut sets: Sets = sets;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    if System::getUsesCardinality() {
        crefs = ComponentReference::expandCref(lhsCref, false)?;
        sets.sets = increaseConnectRefCount2(&crefs, sets.sets.clone())?;
        crefs = ComponentReference::expandCref(rhsCref, false)?;
        sets.sets = increaseConnectRefCount2(&crefs, sets.sets.clone())?;
    }
    Ok(sets)
}

pub(crate) fn increaseConnectRefCount2(
    mut crefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut sets: metamodelica::Ref<SetTrieNode>,
) -> Result<metamodelica::Ref<SetTrieNode>> {
    let mut sets: metamodelica::Ref<SetTrieNode> = sets;
    for mut cr in &**crefs {
        sets = setTrieUpdate(
            metamodelica::AsArg::as_arg(&cr),
            1,
            sets,
            &fnptr!(increaseRefCount, i32, metamodelica::Ref<SetTrieNode>),
        )?;
    }
    Ok(sets)
}

fn increaseRefCount(mut amount: i32, mut node: metamodelica::Ref<SetTrieNode>) -> metamodelica::Ref<SetTrieNode> {
    let mut node: metamodelica::Ref<SetTrieNode> = node;
    let () = (match &*node {
        DAE::Connect::SetTrieNode::SET_TRIE_NODE {
            connectCount: __node_connectCount,
            ..
        } => {
            assign_variant_field!(node => SetTrieNode::SET_TRIE_NODE; connectCount = __node_connectCount.clone() + amount);
            ()
        }
        DAE::Connect::SetTrieNode::SET_TRIE_LEAF {
            connectCount: __node_connectCount,
            ..
        } => {
            assign_variant_field!(node => SetTrieNode::SET_TRIE_LEAF; connectCount = __node_connectCount.clone() + amount);
            ()
        }
    });
    node
}

fn generateStreamEquations(
    mut elements: &metamodelica::List<ConnectorElement>,
    mut flowThreshold: metamodelica::Real,
) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    DAE = (::match_deref::match_deref! { match elements {
        Deref @ metamodelica::ListNode::Cons { head: ConnectorElement { face: DAE::Connect::Face::INSIDE, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            DAE::emptyDae().clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: ConnectorElement { face: DAE::Connect::Face::INSIDE, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: ConnectorElement { face: DAE::Connect::Face::INSIDE, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            DAE::emptyDae().clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: ConnectorElement { name: cr1, face: DAE::Connect::Face::OUTSIDE, source: src1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: ConnectorElement { name: cr2, face: DAE::Connect::Face::OUTSIDE, source: src2, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut src: metamodelica::Ref<DAE::ElementSource>;
            let mut dae: DAE::DAElist;
            let mut cref1: metamodelica::Ref<DAE::Exp>;
            let mut cref2: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            cref1 = Expression::crefExp(cr1.clone())?;
            cref2 = Expression::crefExp(cr2.clone())?;
            e1 = makeInStreamCall(cref2.clone())?;
            e2 = makeInStreamCall(cref1.clone())?;
            src = ElementSource::mergeSources(metamodelica::AsArg::as_arg(&src1), metamodelica::AsArg::as_arg(&src2))?;
            dae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::EQUATION { exp: cref1, scalar: e1, source: src.clone() }), metamodelica::Ref::new(DAE::Element::EQUATION { exp: cref2, scalar: e2, source: src })] };
            dae
        },
        Deref @ metamodelica::ListNode::Cons { head: ConnectorElement { name: cr1, source: src1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: ConnectorElement { name: cr2, source: src2, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut src: metamodelica::Ref<DAE::ElementSource>;
            let mut dae: DAE::DAElist;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            src = ElementSource::mergeSources(metamodelica::AsArg::as_arg(&src1), metamodelica::AsArg::as_arg(&src2))?;
            e1 = Expression::crefExp(cr1.clone())?;
            e2 = Expression::crefExp(cr2.clone())?;
            dae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::EQUATION { exp: e1, scalar: e2, source: src })] };
            dae
        },
        _ => {
            let mut dae: DAE::DAElist;
            let mut inside: metamodelica::List<ConnectorElement>;
            let mut outside: metamodelica::List<ConnectorElement>;
            (outside, inside) = List::splitOnTrue(elements, &move |__a0: ConnectorElement| -> metamodelica::Result<_> { ::std::result::Result::Ok(isOutsideElement(&__a0)) })?;
            dae = streamEquationGeneral(outside, inside, flowThreshold)?;
            dae
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(DAE)
}

fn isOutsideElement(mut element: &ConnectorElement) -> bool {
    let mut isOutside: bool;
    isOutside = (match element.clone() {
        ConnectorElement {
            face: DAE::Connect::Face::OUTSIDE,
            ..
        } => true,
        _ => false,
    });
    isOutside
}

fn isZeroFlowMinMax(
    mut streamCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut element: ConnectorElement,
) -> Result<bool> {
    let mut isZero: bool;
    if compareCrefStreamSet(streamCref, &element)? {
        isZero = false;
    } else if isOutsideElement(&element) {
        isZero = isZeroFlow(element, &(literal!("max")))?;
    } else {
        isZero = isZeroFlow(element, &(literal!("min")))?;
    }
    Ok(isZero)
}

fn isZeroFlow(mut element: ConnectorElement, mut attr: &ArcStr) -> Result<bool> {
    let mut isZero: bool;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut attr_oexp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut flow_exp: metamodelica::Ref<DAE::Exp>;
    let mut attr_exp: metamodelica::Ref<DAE::Exp>;
    flow_exp = flowExp(element)?;
    ty = Expression::r#typeof(flow_exp)?;
    attr_oexp = Types::lookupAttributeExp(&(Types::getAttributes(&ty)), attr)?;
    if (attr_oexp).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(attr_oexp) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        attr_exp = metamodelica::Own::own(__pa0);
        isZero = Expression::isZero(&attr_exp)?;
    } else {
        isZero = false;
    }
    Ok(isZero)
}

fn streamEquationGeneral(
    mut outsideElements: metamodelica::List<ConnectorElement>,
    mut insideElements: metamodelica::List<ConnectorElement>,
    mut flowThreshold: metamodelica::Real,
) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist;
    let mut outside: metamodelica::List<ConnectorElement>;
    let mut cref_exp: metamodelica::Ref<DAE::Exp>;
    let mut res: metamodelica::Ref<DAE::Exp>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    let mut eql: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    for mut e in &*outsideElements {
        cref_exp = Expression::crefExp(e.name.clone())?;
        outside = removeStreamSetElement(e.name.clone(), outsideElements.clone())?;
        res = streamSumEquationExp(outside, insideElements.clone(), flowThreshold)?;
        src = ElementSource::addAdditionalComment(&e.source, literal!(" equation generated by stream handling"));
        eql = metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::EQUATION {
                exp: cref_exp,
                scalar: res,
                source: src,
            }),
            eql,
        );
    }
    DAE = DAE::DAElist { elementLst: eql };
    Ok(DAE)
}

fn streamSumEquationExp(
    mut outsideElements: metamodelica::List<ConnectorElement>,
    mut insideElements: metamodelica::List<ConnectorElement>,
    mut flowThreshold: metamodelica::Real,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut sumExp: metamodelica::Ref<DAE::Exp>;
    let mut outside_sum1: metamodelica::Ref<DAE::Exp>;
    let mut outside_sum2: metamodelica::Ref<DAE::Exp>;
    let mut inside_sum1: metamodelica::Ref<DAE::Exp>;
    let mut inside_sum2: metamodelica::Ref<DAE::Exp>;
    if (outsideElements).is_empty() {
        inside_sum1 = sumMap(insideElements.clone(), &sumInside1, flowThreshold)?;
        inside_sum2 = sumMap(insideElements, &sumInside2, flowThreshold)?;
        sumExp = Expression::expDiv(inside_sum1, inside_sum2)?;
    } else if (insideElements).is_empty() {
        outside_sum1 = sumMap(outsideElements.clone(), &sumOutside1, flowThreshold)?;
        outside_sum2 = sumMap(outsideElements, &sumOutside2, flowThreshold)?;
        sumExp = Expression::expDiv(outside_sum1, outside_sum2)?;
    } else {
        outside_sum1 = sumMap(outsideElements.clone(), &sumOutside1, flowThreshold)?;
        outside_sum2 = sumMap(outsideElements, &sumOutside2, flowThreshold)?;
        inside_sum1 = sumMap(insideElements.clone(), &sumInside1, flowThreshold)?;
        inside_sum2 = sumMap(insideElements, &sumInside2, flowThreshold)?;
        sumExp = Expression::expDiv(
            Expression::expAdd(outside_sum1, inside_sum1)?,
            Expression::expAdd(outside_sum2, inside_sum2)?,
        )?;
    }
    Ok(sumExp)
}

fn sumMap(
    mut elements: metamodelica::List<ConnectorElement>,
    mut func: &dyn ::std::ops::Fn(ConnectorElement, metamodelica::Real) -> Result<metamodelica::Ref<DAE::Exp>>,
    mut flowThreshold: metamodelica::Real,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(ConnectorElement, metamodelica::Real) -> Result<metamodelica::Ref<DAE::Exp>> + 'static,
    >;

    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = ({
        let mut __acc: Option<metamodelica::Ref<DAE::Exp>> = None;
        for mut e in (elements.reverse()).into_iter().cloned() {
            let __x = func(e.clone(), flowThreshold)?;
            __acc = Some(match __acc {
                None => __x,
                Some(__cur) => Expression::expAdd(__x, __cur)?,
            });
        }
        __acc.ok_or_else(|| "empty Expression.expAdd reduction")?
    });
    Ok(exp)
}

fn streamFlowExp(mut element: ConnectorElement) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)> {
    let mut streamExp: metamodelica::Ref<DAE::Exp>;
    let mut flowExp: metamodelica::Ref<DAE::Exp>;
    let mut flow_cr: metamodelica::Ref<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &(element.clone()) {
        ConnectorElement { ty: DAE::Connect::ConnectorType::STREAM { associatedFlow: Some(__pa0) }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    flow_cr = metamodelica::Own::own(__pa0);
    streamExp = Expression::crefExp(element.name.clone())?;
    flowExp = Expression::crefExp(flow_cr)?;
    Ok((streamExp, flowExp))
}

fn flowExp(mut element: ConnectorElement) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut flowExp: metamodelica::Ref<DAE::Exp>;
    let mut flow_cr: metamodelica::Ref<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &(element) {
        ConnectorElement { ty: DAE::Connect::ConnectorType::STREAM { associatedFlow: Some(__pa0) }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    flow_cr = metamodelica::Own::own(__pa0);
    flowExp = Expression::crefExp(flow_cr)?;
    Ok(flowExp)
}

fn sumOutside1(
    mut element: ConnectorElement,
    mut flowThreshold: metamodelica::Real,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut stream_exp: metamodelica::Ref<DAE::Exp>;
    let mut flow_exp: metamodelica::Ref<DAE::Exp>;
    let mut flow_threshold: metamodelica::Ref<DAE::Exp>;
    (stream_exp, flow_exp) = streamFlowExp(element)?;
    flow_threshold = metamodelica::Ref::new(DAE::Exp::RCONST { real: flowThreshold });
    exp = Expression::expMul(
        makePositiveMaxCall(flow_exp, flow_threshold)?,
        makeInStreamCall(stream_exp)?,
    )?;
    Ok(exp)
}

fn sumInside1(
    mut element: ConnectorElement,
    mut flowThreshold: metamodelica::Real,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut stream_exp: metamodelica::Ref<DAE::Exp>;
    let mut flow_exp: metamodelica::Ref<DAE::Exp>;
    let mut flow_threshold: metamodelica::Ref<DAE::Exp>;
    let mut flowTy: metamodelica::Ref<DAE::Type>;
    (stream_exp, flow_exp) = streamFlowExp(element)?;
    flowTy = Expression::r#typeof(flow_exp.clone())?;
    flow_exp = metamodelica::Ref::new(DAE::Exp::UNARY {
        operator: DAE::Operator::UMINUS { ty: flowTy },
        exp: flow_exp,
    });
    flow_threshold = metamodelica::Ref::new(DAE::Exp::RCONST { real: flowThreshold });
    exp = Expression::expMul(makePositiveMaxCall(flow_exp, flow_threshold)?, stream_exp)?;
    Ok(exp)
}

fn sumOutside2(
    mut element: ConnectorElement,
    mut flowThreshold: metamodelica::Real,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut flow_exp: metamodelica::Ref<DAE::Exp>;
    flow_exp = flowExp(element)?;
    exp = makePositiveMaxCall(
        flow_exp,
        metamodelica::Ref::new(DAE::Exp::RCONST { real: flowThreshold }),
    )?;
    Ok(exp)
}

fn sumInside2(
    mut element: ConnectorElement,
    mut flowThreshold: metamodelica::Real,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut flow_exp: metamodelica::Ref<DAE::Exp>;
    let mut flowTy: metamodelica::Ref<DAE::Type>;
    flow_exp = flowExp(element)?;
    flowTy = Expression::r#typeof(flow_exp.clone())?;
    flow_exp = metamodelica::Ref::new(DAE::Exp::UNARY {
        operator: DAE::Operator::UMINUS { ty: flowTy },
        exp: flow_exp,
    });
    exp = makePositiveMaxCall(
        flow_exp,
        metamodelica::Ref::new(DAE::Exp::RCONST { real: flowThreshold }),
    )?;
    Ok(exp)
}

pub(crate) fn faceEqual(mut face1: Face, mut face2: Face) -> bool {
    let mut sameFaces: bool =
        metamodelica::valueConstructor((&face1)).unwrap() == metamodelica::valueConstructor((&face2)).unwrap();
    sameFaces
}

fn makeInStreamCall(mut streamExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut inStreamCall: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = Expression::r#typeof(streamExp.clone())?;
    inStreamCall = Expression::makeBuiltinCall(literal!("inStream"), list![streamExp], ty, false);
    Ok(inStreamCall)
}

fn makePositiveMaxCall(
    mut flowExp: metamodelica::Ref<DAE::Exp>,
    mut flowThreshold: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut positiveMaxCall: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut nominal_oexp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut nominal_exp: metamodelica::Ref<DAE::Exp>;
    let mut flow_threshold: metamodelica::Ref<DAE::Exp>;
    ty = Expression::r#typeof(flowExp.clone())?;
    nominal_oexp = Types::lookupAttributeExp(&(Types::getAttributes(&ty)), &(literal!("nominal")))?;
    if (nominal_oexp).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(nominal_oexp) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        nominal_exp = metamodelica::Own::own(__pa0);
        flow_threshold = Expression::expMul(flowThreshold, nominal_exp)?;
    } else {
        flow_threshold = flowThreshold;
    }
    positiveMaxCall = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("$OMC$PositiveMax"),
        }),
        expLst: list![flowExp, flow_threshold],
        attr: metamodelica::Ref::new(DAE::CallAttributes {
            ty: ty,
            tuple_: false,
            builtin: true,
            isImpure: false,
            isFunctionPointerCall: false,
            inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
            tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
            noReturn: DAE::NoReturn::RETURNS.clone(),
        }),
    });
    {
        let __v = Some(true);
        openmodelica_util::Globals::isInStream.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok(positiveMaxCall)
}

fn evaluateConnectionOperators(
    mut sets: Sets,
    mut setArray: metamodelica::Array<Set>,
    mut DAE: DAE::DAElist,
) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist = DAE;
    let mut flow_threshold: metamodelica::Real;
    let mut has_cardinality: bool = System::getUsesCardinality();
    if System::getHasStreamConnectors() || has_cardinality {
        flow_threshold = Flags::getConfigReal(Flags::FLOW_THRESHOLD.clone())?;
        (DAE, _, _) = DAEUtil::traverseDAE(
            DAE,
            openmodelica_frontend_dump::AvlTreePathFunction::Tree::interned_EMPTY(),
            (std::sync::Arc::new({
                let __pe_b2 = setArray.clone();
                let __pe_b3 = has_cardinality;
                let __pe_b4 = flow_threshold;
                move |__pe_a0, __pe_a1| {
                    evaluateConnectionOperators2(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone())
                }
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Sets) -> Result<(metamodelica::Ref<DAE::Exp>, Sets)>
                        + 'static,
                >),
            sets,
        )?;
        DAE = simplifyDAEElements(has_cardinality, DAE)?;
    }
    Ok(DAE)
}

fn evaluateConnectionOperators2(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut sets: Sets,
    mut setArray: metamodelica::Array<Set>,
    mut hasCardinality: bool,
    mut flowThreshold: metamodelica::Real,
) -> Result<(metamodelica::Ref<DAE::Exp>, Sets)> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut sets: Sets = sets;
    let mut changed: bool;
    (exp, changed) = Expression::traverseExpBottomUp(
        exp,
        &({
            let __pe_b1 = sets.clone();
            let __pe_b2 = setArray.clone();
            let __pe_b3 = flowThreshold;
            move |__pe_a0, __pe_a4| {
                evaluateConnectionOperatorsExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4)
            }
        }),
        false,
    )?;
    if changed && hasCardinality {
        (exp, _) = ExpressionSimplify::simplify(exp)?;
    }
    Ok((exp, sets))
}

fn evaluateConnectionOperatorsExp(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut sets: Sets,
    mut setArray: metamodelica::Array<Set>,
    mut flowThreshold: metamodelica::Real,
    mut changed: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut changed: bool = changed;
    (exp, changed) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "inStream" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = evaluateInStream(cr.clone(), sets, setArray.clone(), flowThreshold)?;
            (e, true)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "actualStream" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = evaluateActualStream(cr.clone(), sets, setArray.clone(), flowThreshold)?;
            (e, true)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cardinality" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = evaluateCardinality(metamodelica::AsArg::as_arg(&cr), &sets);
            (e, true)
        },
        _ => {
            (exp, changed)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, changed))
}

fn mkArrayIfNeeded(
    mut ty: &metamodelica::Ref<DAE::Type>,
    mut exp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    exp = Expression::arrayFill(TypesDump::getDimensions(ty), exp)?;
    Ok(exp)
}

fn evaluateInStream(
    mut streamCref: metamodelica::Ref<DAE::ComponentRef>,
    mut sets: Sets,
    mut setArray: metamodelica::Array<Set>,
    mut flowThreshold: metamodelica::Real,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut e: ConnectorElement;
    let mut sl: metamodelica::List<ConnectorElement>;
    let mut set: i32;
    match '__try0: {
        e = findElement(
            streamCref.clone(),
            openmodelica_frontend_types::DAE::Connect::Face::INSIDE,
            ConnectorType::STREAM { associatedFlow: None },
            DAE::emptyElementSource().clone(),
            &sets,
        );
        if isNewElement(e.clone()) {
            sl = list![e.clone()];
        } else {
            let ConnectorElement { set: __pa1, .. } = &e;
            set = metamodelica::Own::own(__pa1);
            let Set::SET {
                ty: ConnectorType::STREAM { .. },
                elements: __pa2,
            } = (unwrap_break_err!(setArrayGet(setArray.clone(), set), '__try0))
            else {
                break '__try0 Err::<_, _>("pattern mismatch");
            };
            sl = metamodelica::Own::own(__pa2);
        }
        exp = unwrap_break_err!(generateInStreamExp(streamCref.clone(), sl.clone(), sets.clone(), setArray.clone(), flowThreshold), '__try0);
        Ok::<_, &'static str>((e.clone(), exp.clone(), sl.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            e = __try0_o0;
            exp = __try0_o1;
            sl = __try0_o2;
        }
        Err(__try0_err) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("- ConnectUtil.evaluateInStream failed for "));
                __mm_s.push_str(&*ComponentReference::crefStr(&streamCref)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            })?;
            return Err(__try0_err);
        }
    }
    Ok(exp)
}

fn generateInStreamExp(
    mut streamCref: metamodelica::Ref<DAE::ComponentRef>,
    mut streams: metamodelica::List<ConnectorElement>,
    mut sets: Sets,
    mut setArray: metamodelica::Array<Set>,
    mut flowThreshold: metamodelica::Real,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut reducedStreams: metamodelica::List<ConnectorElement>;
    reducedStreams = List::filterOnFalse(
        streams,
        &({
            let __pe_b0 = streamCref.clone();
            move |__pe_a1| isZeroFlowMinMax(&__pe_b0, __pe_a1)
        }),
    )?;
    exp = (::match_deref::match_deref! { match &(reducedStreams.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: ConnectorElement { name: c, face: DAE::Connect::Face::INSIDE, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            Expression::crefExp(c.clone())?
        },
        Deref @ metamodelica::ListNode::Cons { head: ConnectorElement { face: DAE::Connect::Face::INSIDE, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: ConnectorElement { face: DAE::Connect::Face::INSIDE, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut c: metamodelica::Ref<DAE::ComponentRef>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(removeStreamSetElement(streamCref, reducedStreams)?) {
                Deref @ metamodelica::ListNode::Cons { head: ConnectorElement { name: __pa0, .. }, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            c = metamodelica::Own::own(__pa0);
            e = Expression::crefExp(c)?;
            e
        },
        Deref @ metamodelica::ListNode::Cons { head: ConnectorElement { face: f1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: ConnectorElement { face: f2, .. }, tail: Deref @ metamodelica::ListNode::Nil } } if (!(faceEqual(f1.clone(), f2.clone()))) => {
            let mut c: metamodelica::Ref<DAE::ComponentRef>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(removeStreamSetElement(streamCref, reducedStreams)?) {
                Deref @ metamodelica::ListNode::Cons { head: ConnectorElement { name: __pa0, .. }, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            c = metamodelica::Own::own(__pa0);
            e = evaluateInStream(c, sets, setArray.clone(), flowThreshold)?;
            e
        },
        _ => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut expr: metamodelica::Ref<DAE::Exp>;
            let mut inside: metamodelica::List<ConnectorElement>;
            let mut outside: metamodelica::List<ConnectorElement>;
            (outside, inside) = List::splitOnTrue(&reducedStreams, &move |__a0: ConnectorElement| -> metamodelica::Result<_> { ::std::result::Result::Ok(isOutsideElement(&__a0)) })?;
            inside = removeStreamSetElement(streamCref, inside)?;
            e = streamSumEquationExp(outside, inside.clone(), flowThreshold)?;
            if !((inside).is_empty()) {
                (expr, _) = streamFlowExp((inside).head().cloned()?)?;
                e = Expression::makePureBuiltinCall(literal!("$OMC$inStreamDiv"), list![e.clone(), expr], Expression::r#typeof(e)?);
            }
            (e, _) = evaluateConnectionOperators2(e, sets, setArray.clone(), false, flowThreshold)?;
            e
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn evaluateActualStream(
    mut streamCref: metamodelica::Ref<DAE::ComponentRef>,
    mut sets: Sets,
    mut setArray: metamodelica::Array<Set>,
    mut flowThreshold: metamodelica::Real,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut flow_cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut flow_exp: metamodelica::Ref<DAE::Exp>;
    let mut stream_exp: metamodelica::Ref<DAE::Exp>;
    let mut instream_exp: metamodelica::Ref<DAE::Exp>;
    let mut rel_exp: metamodelica::Ref<DAE::Exp>;
    let mut ety: metamodelica::Ref<DAE::Type>;
    let mut flow_dir: i32;
    flow_cr = getStreamFlowAssociation(&streamCref, &sets)?;
    ety = ComponentReference::crefLastType(&flow_cr)?;
    flow_dir = evaluateFlowDirection(&ety);
    if flow_dir == 1 {
        rel_exp = evaluateInStream(streamCref, sets, setArray.clone(), flowThreshold)?;
    } else if flow_dir == -1 {
        rel_exp = Expression::crefExp(streamCref)?;
    } else {
        flow_exp = Expression::crefExp(flow_cr)?;
        stream_exp = Expression::crefExp(streamCref.clone())?;
        instream_exp = evaluateInStream(streamCref, sets, setArray.clone(), flowThreshold)?;
        rel_exp = metamodelica::Ref::new(DAE::Exp::IFEXP {
            expCond: metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: flow_exp,
                operator: DAE::Operator::GREATER { ty: ety },
                exp2: metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(0.0_f64),
                }),
                index: -1,
                optionExpisASUB: None,
            }),
            expThen: instream_exp,
            expElse: stream_exp,
        });
    }
    exp = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("smooth"),
        }),
        expLst: list![metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }), rel_exp],
        attr: DAE::callAttrBuiltinReal().clone(),
    });
    Ok(exp)
}

fn evaluateFlowDirection(mut ty: &metamodelica::Ref<DAE::Type>) -> i32 {
    let mut direction: i32 = 0;
    let mut attr: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut min_oval: Option<metamodelica::Ref<Values::Value>>;
    let mut max_oval: Option<metamodelica::Ref<Values::Value>>;
    let mut min_val: metamodelica::Real;
    let mut max_val: metamodelica::Real;
    attr = Types::getAttributes(ty);
    if (attr).is_empty() {
        return direction;
    }
    min_oval = Types::lookupAttributeValue(&attr, &(literal!("min")));
    max_oval = Types::lookupAttributeValue(&attr, &(literal!("max")));
    direction = (::match_deref::match_deref! { match &((min_oval, max_oval)) {
        (None, None) => 0,
        (Some(Deref @ Values::Value::REAL { real: __esc_min_val }), None) => {
            min_val = (*__esc_min_val).clone();
            if (min_val.clone() >= metamodelica::OrderedFloat((0) as f64)) {1} else {0}
        },
        (None, Some(Deref @ Values::Value::REAL { real: __esc_max_val })) => {
            max_val = (*__esc_max_val).clone();
            if (max_val.clone() <= metamodelica::OrderedFloat((0) as f64)) {-1} else {0}
        },
        (Some(Deref @ Values::Value::REAL { real: __esc_min_val }), Some(Deref @ Values::Value::REAL { real: __esc_max_val })) => {
            min_val = (*__esc_min_val).clone();
            max_val = (*__esc_max_val).clone();
            if (min_val.clone() >= metamodelica::OrderedFloat((0) as f64) && max_val.clone() >= min_val.clone()) {1} else if (max_val.clone() <= metamodelica::OrderedFloat((0) as f64) && min_val.clone() <= max_val.clone()) {-1} else {0}
        },
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    direction
}

fn evaluateCardinality(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut sets: &Sets,
) -> metamodelica::Ref<DAE::Exp> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = metamodelica::Ref::new(DAE::Exp::ICONST {
        integer: getConnectCount(cref, &sets.sets),
    });
    exp
}

fn simplifyDAEElements(mut hasCardinality: bool, mut DAE: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut DAE: DAE::DAElist = DAE;
    if hasCardinality {
        DAE = DAE::DAElist {
            elementLst: List::mapFlat(
                &DAE.elementLst,
                &fnptr!(simplifyDAEElement, metamodelica::Ref<DAE::Element>),
            )?,
        };
    }
    Ok(DAE)
}

fn simplifyDAEElement(
    mut element: metamodelica::Ref<DAE::Element>,
) -> metamodelica::List<metamodelica::Ref<DAE::Element>> {
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    elements = 'mc: {
        let __mc_input = &*element;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::IF_EQUATION { condition1: conds, equations2: branches, equations3: else_branch, .. } => {
                    Ok(simplifyDAEIfEquation(metamodelica::AsArg::as_arg(&conds), branches.clone(), else_branch.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: conds, equations2: branches, equations3: else_branch, .. } => {
                    Ok(simplifyDAEIfEquation(metamodelica::AsArg::as_arg(&conds), branches.clone(), else_branch.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ASSERT { condition: Deref @ DAE::Exp::BCONST { bool: true }, .. } => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(list![element.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    elements
}

fn simplifyDAEIfEquation(
    mut conditions: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut branches: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    mut elseBranch: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut cond_value: bool;
    let mut rest_branches: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>> = branches;
    for mut cond in &**conditions {
        let __pa0 = ::match_deref::match_deref! { match &(cond.clone()) {
            Deref @ DAE::Exp::BCONST { bool: __pa0 } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        cond_value = metamodelica::Own::own(__pa0);
        if cond_value == true {
            elements = (rest_branches).head().cloned()?.reverse();
            return Ok(elements);
        }
        rest_branches = (rest_branches).rest()?;
    }
    elements = elseBranch.reverse();
    Ok(elements)
}

fn removeStreamSetElement(
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut elements: metamodelica::List<ConnectorElement>,
) -> Result<metamodelica::List<ConnectorElement>> {
    let mut elements: metamodelica::List<ConnectorElement> = elements;
    (elements, _) = List::deleteMemberOnTrue(
        cref,
        elements,
        &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: ConnectorElement| compareCrefStreamSet(&__a0, &__a1),
    )?;
    Ok(elements)
}

fn compareCrefStreamSet(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut element: &ConnectorElement,
) -> Result<bool> {
    let mut matches: bool;
    matches = ComponentReferenceBasics::crefEqualNoStringCompare(cref, &element.name)?;
    Ok(matches)
}

pub(crate) fn componentFace(
    mut env: FCore::Graph,
    mut componentRef: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<Face> {
    let mut face: Face;
    face = 'mc: {
        let __mc_input = &**componentRef;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
                    Ok(openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, .. } => {
                    ::match_deref::match_deref! { match &(Lookup::lookupVar(FCore::emptyCache(), env.clone(), ComponentReferenceBasics::makeCrefIdent(id.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?) {
                        (_, _, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: _, isExpandable: _ }, .. }, _, _, _, _, _, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { .. } => {
                    Ok(openmodelica_frontend_types::DAE::Connect::Face::INSIDE)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(face)
}

pub(crate) fn componentFaceType(mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>) -> Result<Face> {
    let mut outFace: Face;
    outFace = (::match_deref::match_deref! { match inComponentRef {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE,
        Deref @ DAE::ComponentRef::CREF_QUAL { identType: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: _, isExpandable: _ }, .. }, .. } => openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE,
        Deref @ DAE::ComponentRef::CREF_QUAL { identType: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::CONNECTOR { path: _, isExpandable: _ }, .. }, .. }, .. } => openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE,
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => openmodelica_frontend_types::DAE::Connect::Face::INSIDE,
        _ => return Err("match: no arm matched"),
    } });
    Ok(outFace)
}

pub(crate) fn checkConnectorBalance(
    mut vars: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut info: &SourceInfo,
) -> Result<()> {
    let mut potentials: i32;
    let mut flows: i32;
    let mut streams: i32;
    (potentials, flows, streams) = countConnectorVars(vars)?;
    let true = (checkConnectorBalance2(potentials, flows, streams, path, info)?) else {
        return Err("pattern mismatch");
    };
    Ok(())
}

fn checkConnectorBalance2(
    mut potentialVars: i32,
    mut flowVars: i32,
    mut streamVars: i32,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut info: &SourceInfo,
) -> Result<bool> {
    let mut isBalanced: bool = true;
    let mut flow_str: ArcStr;
    let mut potential_str: ArcStr;
    let mut class_str: ArcStr;
    if Config::languageStandardAtMost(Config::LanguageStandard::_2_x.clone())? {
        return Ok(isBalanced);
    }
    if potentialVars != flowVars {
        flow_str = ArcStr::from(::std::format!("{}", flowVars));
        potential_str = ArcStr::from(::std::format!("{}", potentialVars));
        class_str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
        Error::addSourceMessage(
            &(Error::UNBALANCED_CONNECTOR.clone()),
            list![class_str, potential_str, flow_str],
            info,
        )?;
    }
    if streamVars > 0 && flowVars != 1 {
        flow_str = ArcStr::from(::std::format!("{}", flowVars));
        class_str = AbsynUtil::pathString(path, literal!("."), true, false)?;
        Error::addSourceMessage(
            &(Error::MISMATCHED_FLOW_IN_STREAM_CONNECTOR.clone()),
            list![class_str, flow_str],
            info,
        )?;
        isBalanced = false;
    }
    Ok(isBalanced)
}

fn countConnectorVars(mut vars: &metamodelica::List<metamodelica::Ref<DAE::Var>>) -> Result<(i32, i32, i32)> {
    let mut potentialVars: i32 = 0;
    let mut flowVars: i32 = 0;
    let mut streamVars: i32 = 0;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut ty2: metamodelica::Ref<DAE::Type>;
    let mut attr: metamodelica::Ref<DAE::Attributes>;
    let mut n: i32;
    let mut p: i32;
    let mut f: i32;
    let mut s: i32;
    for mut var in &**vars {
        let __arc2 = var.clone();
        let DAE::TYPES_VAR {
            ty: __pa0,
            attributes: __pa1,
            ..
        } = &*__arc2;
        ty = metamodelica::Own::own(__pa0);
        attr = metamodelica::Own::own(__pa1);
        ty2 = Types::arrayElementType(&ty);
        if Types::isConnector(&ty2) {
            n = ({
                let mut __acc: i32 = 1;
                for mut dim in (Types::getDimensionSizes(&ty)?).into_iter().cloned() {
                    let __x = dim.clone();
                    __acc *= __x;
                }
                __acc
            });
            (p, f, s) = countConnectorVars(&(Types::getConnectorVars(&ty2)?))?;
            if AbsynUtil::isInputOrOutput(DAEUtil::getAttrDirection(&attr)) {
                p = 0;
            }
            potentialVars = potentialVars + p * n;
            flowVars = flowVars + f * n;
            streamVars = streamVars + s * n;
        } else {
            let () = (::match_deref::match_deref! { match &(attr) {
                Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::FLOW { .. }, .. } => {
                    flowVars = flowVars + sizeOfType(&var.ty)?;
                    ()
                },
                Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::STREAM { .. }, .. } => {
                    streamVars = streamVars + sizeOfType(&var.ty)?;
                    ()
                },
                Deref @ DAE::Attributes { direction: Absyn::Direction::BIDIR { .. }, variability: SCode::Variability::VAR { .. }, .. } => {
                    potentialVars = potentialVars + sizeOfType(&var.ty)?;
                    ()
                },
                _ => (),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
    }
    Ok((potentialVars, flowVars, streamVars))
}

fn sizeOfVariableList(mut vars: &metamodelica::List<metamodelica::Ref<DAE::Var>>) -> Result<i32> {
    let mut size: i32 = 0;
    for mut var in &**vars {
        size = size + sizeOfType(&var.ty)?;
    }
    Ok(size)
}

fn sizeOfType<'__b>(mut ty: &'__b metamodelica::Ref<DAE::Type>) -> Result<i32> {
    '__tco: loop {
        ::match_deref::match_deref! { match ty {
            Deref @ DAE::Type::T_INTEGER { .. } => {
                return Ok(1)
            },
            Deref @ DAE::Type::T_REAL { .. } => {
                return Ok(1)
            },
            Deref @ DAE::Type::T_STRING { .. } => {
                return Ok(1)
            },
            Deref @ DAE::Type::T_BOOL { .. } => {
                return Ok(1)
            },
            Deref @ DAE::Type::T_ENUMERATION { .. } => {
                return Ok(1)
            },
            Deref @ DAE::Type::T_ARRAY { .. } => {
                return Ok(({
            let mut __acc: i32 = 1;
            for mut dim in (var_field!((**ty).dims, DAE::Type::T_ARRAY).clone()).into_iter().cloned() {
                let __x = Expression::dimensionSize(&(dim.clone()))?;
                __acc *= __x;
            }
            __acc
        }) * sizeOfType(var_field!((**ty).ty, DAE::Type::T_ARRAY))?)
            },
            Deref @ DAE::Type::T_COMPLEX { varLst: v, equalityConstraint: None, .. } => {
                return Ok(sizeOfVariableList(v)?)
            },
            Deref @ DAE::Type::T_COMPLEX { equalityConstraint: Some((_, n, _)), .. } => {
                return Ok(n.clone())
            },
            Deref @ DAE::Type::T_SUBTYPE_BASIC { equalityConstraint: Some(_), .. } => {
                return Ok(0)
            },
            Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. } => {
                { ty = t; continue '__tco; }
            },
            _ => {
                let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ConnectUtil.sizeOfType failed on ")); __mm_s.push_str(&*TypesDump::printTypeStr(ty.clone())); ArcStr::from(__mm_s) })?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn checkShortConnectorDef(
    mut state: &ClassInf::State,
    mut attributes: &SCode::Attributes,
    mut info: &SourceInfo,
) -> Result<bool> {
    let mut isValid: bool;
    isValid = ({
        let mut pv: i32 = 0;
        let mut fv: i32 = 0;
        let mut sv: i32 = 0;
        (match (state.clone(), attributes.clone()) {
            (
                ClassInf::State::CONNECTOR { .. },
                SCode::Attributes {
                    connectorType: mut ct,
                    direction: Absyn::Direction::BIDIR { .. },
                    ..
                },
            ) => {
                if SCodeUtil::flowBool(ct.clone()) {
                    fv = 1;
                } else if SCodeUtil::streamBool(ct.clone()) {
                    sv = 1;
                } else {
                    pv = 1;
                }
                checkConnectorBalance2(
                    pv,
                    fv,
                    sv,
                    var_field!(state.path, ClassInf::State::CONNECTOR).clone(),
                    info,
                )?
            }
            _ => true,
        })
    });
    Ok(isValid)
}

pub(crate) fn isReferenceInConnects(
    mut connects: &metamodelica::List<ConnectorElement>,
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut isThere: bool = false;
    for mut ce in &**connects {
        if ComponentReferenceBasics::crefPrefixOf(cref, &ce.name)? {
            isThere = true;
            return Ok(isThere);
        }
    }
    Ok(isThere)
}

pub(crate) fn removeReferenceFromConnects(
    mut connects: metamodelica::List<ConnectorElement>,
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(metamodelica::List<ConnectorElement>, bool)> {
    let mut connects: metamodelica::List<ConnectorElement> = connects;
    let mut wasRemoved: bool;
    let mut oe: Option<ConnectorElement>;
    (connects, oe) = List::deleteMemberOnTrue(
        cref,
        connects,
        &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: ConnectorElement| {
            removeReferenceFromConnects2(&__a0, &__a1)
        },
    )?;
    wasRemoved = (oe).is_some();
    Ok((connects, wasRemoved))
}

fn removeReferenceFromConnects2(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut element: &ConnectorElement,
) -> Result<bool> {
    let mut matches: bool;
    matches = ComponentReferenceBasics::crefPrefixOf(cref, &element.name)?;
    Ok(matches)
}

pub(crate) fn printSetsStr(mut sets: &Sets) -> Result<ArcStr> {
    let mut string: ArcStr;
    string = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*ArcStr::from(::std::format!("{}", sets.setCount.clone())));
        __mm_s.push_str(&*literal!(" sets:\n"));
        ArcStr::from(__mm_s)
    };
    string = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*string);
        __mm_s.push_str(&*printSetTrieStr(&sets.sets, literal!("\t"))?);
        ArcStr::from(__mm_s)
    };
    string = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*string);
        __mm_s.push_str(&*literal!("Connected sets:\n"));
        ArcStr::from(__mm_s)
    };
    string = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*string);
        __mm_s.push_str(&*printSetConnections(sets.connections.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    Ok(string)
}

fn printSetTrieStr(mut trie: &metamodelica::Ref<SetTrieNode>, mut accumName: ArcStr) -> Result<ArcStr> {
    let mut string: ArcStr;
    string = (::match_deref::match_deref! { match trie {
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_LEAF { flowAssociation: __trie_flowAssociation, insideElement: __trie_insideElement, name: __trie_name, outsideElement: __trie_outsideElement, .. } => {
            let mut res: ArcStr;
            res = { let mut __mm_s = String::new(); __mm_s.push_str(&*accumName); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*__trie_name); __mm_s.push_str(&*literal!(":")); ArcStr::from(__mm_s) };
            res = { let mut __mm_s = String::new(); __mm_s.push_str(&*res); __mm_s.push_str(&*printLeafElementStr(__trie_insideElement.clone())?); ArcStr::from(__mm_s) };
            res = { let mut __mm_s = String::new(); __mm_s.push_str(&*res); __mm_s.push_str(&*printLeafElementStr(__trie_outsideElement.clone())?); ArcStr::from(__mm_s) };
            res = { let mut __mm_s = String::new(); __mm_s.push_str(&*res); __mm_s.push_str(&*printOptFlowAssociation(__trie_flowAssociation.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
            res
        },
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { name: Deref @ "", nodes: __trie_nodes, .. } => {
            stringAppendList(List::map1(__trie_nodes.clone(), &move |__a0: metamodelica::Ref<SetTrieNode>, __a1: ArcStr| printSetTrieStr(&__a0, __a1), accumName)?)
        },
        Deref @ DAE::Connect::SetTrieNode::SET_TRIE_NODE { name: __trie_name, nodes: __trie_nodes, .. } => {
            let mut name: ArcStr;
            let mut res: ArcStr;
            name = { let mut __mm_s = String::new(); __mm_s.push_str(&*accumName); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*__trie_name); ArcStr::from(__mm_s) };
            res = stringAppendList(List::map1(__trie_nodes.clone(), &move |__a0: metamodelica::Ref<SetTrieNode>, __a1: ArcStr| printSetTrieStr(&__a0, __a1), name)?);
            res
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(string)
}

fn printLeafElementStr(mut element: Option<ConnectorElement>) -> Result<ArcStr> {
    let mut string: ArcStr;
    string = (match element {
        Some(mut e @ ConnectorElement { .. }) => {
            let mut res: ArcStr;
            res = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*printFaceStr(e.face.clone()));
                __mm_s.push_str(&*literal!(" "));
                ArcStr::from(__mm_s)
            };
            res = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*res);
                __mm_s.push_str(&*printConnectorTypeStr(&e.ty)?);
                __mm_s.push_str(&*literal!(" ["));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", e.set.clone())));
                __mm_s.push_str(&*literal!("]"));
                ArcStr::from(__mm_s)
            };
            res
        }
        _ => {
            literal!("")
        }
    });
    Ok(string)
}

pub(crate) fn printElementStr(mut element: &ConnectorElement) -> Result<ArcStr> {
    let mut string: ArcStr;
    string = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&element.name)?);
        __mm_s.push_str(&*literal!(" "));
        ArcStr::from(__mm_s)
    };
    string = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*string);
        __mm_s.push_str(&*printFaceStr(element.face.clone()));
        __mm_s.push_str(&*literal!(" "));
        ArcStr::from(__mm_s)
    };
    string = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*string);
        __mm_s.push_str(&*printConnectorTypeStr(&element.ty)?);
        __mm_s.push_str(&*literal!(" ["));
        __mm_s.push_str(&*ArcStr::from(::std::format!("{}", element.set.clone())));
        __mm_s.push_str(&*literal!("]"));
        ArcStr::from(__mm_s)
    };
    Ok(string)
}

pub(crate) fn printFaceStr(mut face: Face) -> ArcStr {
    let mut string: ArcStr;
    string = (match face {
        DAE::Connect::Face::INSIDE => literal!("inside"),
        DAE::Connect::Face::OUTSIDE => literal!("outside"),
        DAE::Connect::Face::NO_FACE => literal!("unknown"),
    });
    string
}

fn printConnectorTypeStr(mut ty: &ConnectorType) -> Result<ArcStr> {
    let mut string: ArcStr;
    string = (match ty.clone() {
        DAE::Connect::ConnectorType::EQU => literal!("equ"),
        DAE::Connect::ConnectorType::FLOW => literal!("flow"),
        DAE::Connect::ConnectorType::STREAM { .. } => literal!("stream"),
        _ => return Err("match: no arm matched"),
    });
    Ok(string)
}

fn printOptFlowAssociation(mut cref: Option<metamodelica::Ref<DAE::ComponentRef>>) -> Result<ArcStr> {
    let mut string: ArcStr;
    string = (::match_deref::match_deref! { match &(cref) {
        None => {
            literal!("")
        },
        Some(cr) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" associated flow: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?); ArcStr::from(__mm_s) }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(string)
}

fn printSetConnections(mut connections: metamodelica::List<(i32, i32)>) -> Result<ArcStr> {
    let mut string: ArcStr;
    string = stringAppendList(List::map(connections, &fnptr!(printSetConnection, (i32, i32)))?);
    Ok(string)
}

fn printSetConnection(mut connection: (i32, i32)) -> ArcStr {
    let mut string: ArcStr;
    let mut set1: i32;
    let mut set2: i32;
    (set1, set2) = connection;
    string = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\t"));
        __mm_s.push_str(&*ArcStr::from(::std::format!("{}", set1)));
        __mm_s.push_str(&*literal!(" connected to "));
        __mm_s.push_str(&*intString(set2));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    string
}

fn printSetStr(mut set: &Set) -> Result<ArcStr> {
    let mut string: ArcStr;
    string = (match set.clone() {
        DAE::Connect::Set::SET { .. } => stringDelimitList(
            List::map(
                var_field!(set.elements, Set::SET).clone(),
                &move |__a0: ConnectorElement| printElementStr(&__a0),
            )?,
            literal!(", "),
        ),
        DAE::Connect::Set::SET_POINTER { .. } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("pointer to set "));
            __mm_s.push_str(&*intString(var_field!(set.index, Set::SET_POINTER).clone()));
            ArcStr::from(__mm_s)
        }
    });
    Ok(string)
}

fn getAllEquCrefs(mut sets: &metamodelica::List<Set>) -> metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    for mut set in &**sets {
        let () = (match set.clone() {
            DAE::Connect::Set::SET {
                ty: DAE::Connect::ConnectorType::EQU,
                ..
            } => {
                for mut e in &*var_field!(set.elements, Set::SET).clone() {
                    crefs = metamodelica::cons(e.name.clone(), crefs);
                }
                ()
            }
            _ => (),
        });
    }
    crefs
}

fn removeUnusedExpandableVariablesAndConnections(
    mut sets: metamodelica::List<Set>,
    mut DAE: DAE::DAElist,
) -> Result<(metamodelica::List<Set>, DAE::DAElist)> {
    let mut sets: metamodelica::List<Set> = sets;
    let mut DAE: DAE::DAElist = DAE;
    let mut elems: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut expandableVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut unnecessary: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut usedInDAE: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut onlyExpandableConnected: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut equVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut dae: DAE::DAElist;
    let mut setsAsCrefs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let DAE::DAE { elementLst: __pa0 } = &DAE;
    elems = metamodelica::Own::own(__pa0);
    expandableVars = getExpandableVariablesWithNoBinding(&elems);
    dae = DAEUtil::removeVariables(DAE.clone(), &expandableVars)?;
    usedInDAE = getAllExpandableCrefsFromDAE(dae)?;
    setsAsCrefs = getExpandableEquSetsAsCrefs(&sets)?;
    setsAsCrefs = mergeEquSetsAsCrefs(setsAsCrefs)?;
    setsAsCrefs = mergeEquSetsAsCrefs(setsAsCrefs)?;
    onlyExpandableConnected = getOnlyExpandableConnectedCrefs(&setsAsCrefs);
    unnecessary = List::setDifferenceOnTrue(
        onlyExpandableConnected,
        &usedInDAE,
        &fnptr!(
            ComponentReferenceBasics::crefEqualWithoutSubs,
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::Ref<DAE::ComponentRef>
        ),
    )?;
    DAE = DAEUtil::removeVariables(DAE, &unnecessary)?;
    sets = removeCrefsFromSets(sets, unnecessary)?;
    equVars = getAllEquCrefs(&sets);
    expandableVars = List::setDifferenceOnTrue(
        expandableVars,
        &usedInDAE,
        &fnptr!(
            ComponentReferenceBasics::crefEqualWithoutSubs,
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::Ref<DAE::ComponentRef>
        ),
    )?;
    unnecessary = List::setDifferenceOnTrue(
        expandableVars,
        &equVars,
        &fnptr!(
            ComponentReferenceBasics::crefEqualWithoutSubs,
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::Ref<DAE::ComponentRef>
        ),
    )?;
    DAE = DAEUtil::removeVariables(DAE, &unnecessary)?;
    Ok((sets, DAE))
}

fn isEquType(mut ty: &ConnectorType) -> bool {
    let mut isEqu: bool;
    isEqu = (match ty.clone() {
        DAE::Connect::ConnectorType::EQU => true,
        _ => false,
    });
    isEqu
}

pub fn topLevelInput(
    mut componentRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut varDirection: DAE::VarDirection,
    mut connectorType: &metamodelica::Ref<DAE::ConnectorType>,
    mut visibility: DAE::VarVisibility,
) -> Result<bool> {
    let mut isTopLevel: bool;
    let mut newInst: bool = Flags::isSet(Flags::SCODE_INST.clone())?;
    isTopLevel = (::match_deref::match_deref! { match &((varDirection, &**componentRef, visibility, newInst)) {
        (_, _, DAE::VarVisibility::PROTECTED { .. }, _) => false,
        (DAE::VarDirection::INPUT { .. }, _, _, true) => true,
        (DAE::VarDirection::INPUT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }, _, _) => true,
        (DAE::VarDirection::INPUT { .. }, _, _, _) if (faceEqual(componentFaceType(componentRef)?, openmodelica_frontend_types::DAE::Connect::Face::OUTSIDE)) => topLevelConnectorType(connectorType),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isTopLevel)
}

fn topLevelConnectorType(mut inConnectorType: &metamodelica::Ref<DAE::ConnectorType>) -> bool {
    let mut isTopLevel: bool;
    isTopLevel = (match &**inConnectorType {
        DAE::ConnectorType::FLOW { .. } => true,
        DAE::ConnectorType::POTENTIAL { .. } => true,
        _ => false,
    });
    isTopLevel
}

pub(crate) fn getAllExpandableCrefsFromDAE(
    mut inDAE: DAE::DAElist,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut elts: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let DAE::DAE { elementLst: __pa0 } = inDAE;
    elts = metamodelica::Own::own(__pa0);
    let (_, (_, __pa1)) = DAEUtil::traverseDAEElementList(
        elts,
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                collectAllExpandableCrefsInExp,
                metamodelica::Ref<DAE::Exp>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        )> + 'static,
                >),
            metamodelica::nil(),
        ),
    )?;
    outCrefs = metamodelica::Own::own(__pa1);
    Ok(outCrefs)
}

fn collectAllExpandableCrefsInExp(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (outExp, outCrefs) = (match &*exp.clone() {
        DAE::Exp::CREF { componentRef: cr, .. } => (
            exp,
            List::consOnTrue(isExpandable(metamodelica::AsArg::as_arg(&cr)), cr.clone(), acc),
        ),
        _ => (exp, acc),
    });
    (outExp, outCrefs)
}
